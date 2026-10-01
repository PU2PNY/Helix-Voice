#![forbid(unsafe_code)]

use helix_core::{MAX_FRAME_SAMPLES, PcmFrame};
use helix_dsp::DspChain;
use helix_hvc::{HVC_FRAME_SAMPLES, HVC_SAMPLE_RATE_HZ, HvcDecoder, HvcEncoder};
use helix_xlx::{
    IntegrationMode, PCM_BRIDGE_FLAG_APPLY_DSP, PCM_BRIDGE_FLAG_OK, PCM_BRIDGE_FLAG_RESET_STREAM,
    PCM_BRIDGE_HEADER_LEN, PCM_BRIDGE_MAX_PACKET_LEN, PCM_BRIDGE_MAX_SAMPLES, PcmBridgeFrame,
    XLXD_MAX_STREAMS, XlxBridgeConfig,
};
use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::os::unix::net::{UnixDatagram, UnixListener, UnixStream};
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("helix-daemon: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("--version") => {
            ensure_no_extra_args(args)?;
            println!("helix-daemon {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Some("--health") => {
            ensure_no_extra_args(args)?;
            print_health();
            Ok(())
        }
        Some("--self-test") => {
            ensure_no_extra_args(args)?;
            self_test()
        }
        Some("--pcm-bridge") => {
            let socket_path = args
                .next()
                .ok_or_else(|| "--pcm-bridge requires a Unix socket path".to_owned())?;
            ensure_no_extra_args(args)?;
            serve_pcm_bridge(&socket_path)
        }
        Some("--pcm-observe") => {
            let socket_path = args
                .next()
                .ok_or_else(|| "--pcm-observe requires a Unix datagram socket path".to_owned())?;
            ensure_no_extra_args(args)?;
            serve_pcm_observe(&socket_path)
        }
        Some(argument) => Err(format!("unsupported argument: {argument}")),
        None => {
            let xlx = XlxBridgeConfig::default();
            println!(
                "Helix Voice Engine research daemon {}",
                env!("CARGO_PKG_VERSION")
            );
            println!("XLX integration mode: {:?}", xlx.mode);
            println!("Network activation: disabled");
            println!("Local PCM bridge: available but not started");
            println!("Non-blocking PCM observer: available but not started");
            println!("No proprietary codec implementation is included.");
            println!(
                "Use --health, --self-test, --pcm-bridge <unix-socket> or --pcm-observe <unix-dgram>."
            );
            Ok(())
        }
    }
}

fn ensure_no_extra_args(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    if let Some(extra) = args.next() {
        return Err(format!("unexpected argument: {extra}"));
    }
    Ok(())
}

fn print_health() {
    let xlx = XlxBridgeConfig::default();
    let xlx_mode = match xlx.mode {
        IntegrationMode::Disabled => "disabled",
        IntegrationMode::ObserveOnly => "observe-only",
        IntegrationMode::ExternalLicensedBackend => "external-licensed-backend",
    };

    println!(
        "{{\"status\":\"ready\",\"version\":\"{}\",\"dsp\":\"ready\",\"hvc\":\"available\",\"xlx\":\"{}\",\"network\":\"disabled\",\"pcm_bridge\":\"available-local-only\"}}",
        env!("CARGO_PKG_VERSION"),
        xlx_mode
    );
}

fn self_test() -> Result<(), String> {
    let mut input = [0.0_f32; HVC_FRAME_SAMPLES];
    for (index, sample) in input.iter_mut().enumerate() {
        let phase = 2.0 * core::f32::consts::PI * 125.0 * index as f32 / HVC_SAMPLE_RATE_HZ as f32;
        *sample = 0.1 * phase.sin();
    }

    let mut frame = PcmFrame::from_slice(HVC_SAMPLE_RATE_HZ, 0, &input)
        .map_err(|error| format!("PCM creation failed: {error:?}"))?;
    let mut dsp = DspChain::default();
    let level = dsp.process(&mut frame);

    if !level.rms.is_finite() || level.rms <= 0.0 || level.peak > 1.0 {
        return Err("DSP self-test produced invalid level".to_owned());
    }

    let mut encoder = HvcEncoder;
    let packet = encoder
        .encode(&frame)
        .map_err(|error| format!("HVC encode failed: {error:?}"))?;

    let mut decoder = HvcDecoder::default();
    let decoded = decoder
        .decode(&packet, HVC_FRAME_SAMPLES as u64)
        .map_err(|error| format!("HVC decode failed: {error:?}"))?;

    if decoded
        .samples()
        .iter()
        .any(|sample| !sample.is_finite() || sample.abs() > 1.0)
    {
        return Err("HVC self-test produced invalid PCM".to_owned());
    }

    let mut bridge_state = BridgeState::default();
    let bridge_input = [1000_i16; HVC_FRAME_SAMPLES];
    let mut bridge_frame = PcmBridgeFrame::new(
        PCM_BRIDGE_FLAG_APPLY_DSP | PCM_BRIDGE_FLAG_RESET_STREAM,
        1,
        HVC_SAMPLE_RATE_HZ,
        0,
        &bridge_input,
    )
    .map_err(|error| format!("PCM bridge frame failed: {error:?}"))?;
    bridge_state.process(&mut bridge_frame)?;
    if bridge_frame.flags() & PCM_BRIDGE_FLAG_OK == 0
        || bridge_frame
            .samples()
            .iter()
            .any(|sample| *sample == i16::MIN && bridge_input[0] != i16::MIN)
    {
        return Err("PCM bridge self-test produced invalid response".to_owned());
    }

    let xlx = XlxBridgeConfig::default();
    if xlx.mode != IntegrationMode::Disabled {
        return Err("XLX must remain disabled by default".to_owned());
    }

    println!("helix-daemon self-test: PASS");
    println!("DSP: PASS");
    println!("HVC encode/decode: PASS");
    println!("Local PCM bridge processing: PASS");
    println!("XLX default disabled: PASS");
    println!("Network activation: disabled");
    Ok(())
}

#[derive(Default)]
struct BridgeState {
    chains: HashMap<u32, DspChain>,
    order: VecDeque<u32>,
}

impl BridgeState {
    fn process(&mut self, frame: &mut PcmBridgeFrame) -> Result<(), String> {
        let apply_dsp = frame.flags() & PCM_BRIDGE_FLAG_APPLY_DSP != 0;
        let reset = frame.flags() & PCM_BRIDGE_FLAG_RESET_STREAM != 0;
        let stream_id = frame.stream_id();

        if apply_dsp {
            if reset {
                self.chains.remove(&stream_id);
                self.order.retain(|id| *id != stream_id);
            }

            if !self.chains.contains_key(&stream_id) {
                while self.chains.len() >= XLXD_MAX_STREAMS as usize {
                    let Some(stale) = self.order.pop_front() else {
                        return Err("PCM bridge stream-state accounting failed".to_owned());
                    };
                    self.chains.remove(&stale);
                }
                self.chains.insert(stream_id, DspChain::xlx_crossmode());
                self.order.push_back(stream_id);
            }

            let mut normalized = [0.0_f32; MAX_FRAME_SAMPLES];
            for (dst, src) in normalized.iter_mut().zip(frame.samples().iter()) {
                *dst = f32::from(*src) / 32_768.0;
            }

            let mut pcm = PcmFrame::from_slice(
                frame.sample_rate_hz(),
                frame.timestamp_samples(),
                &normalized[..frame.samples().len()],
            )
            .map_err(|error| format!("PCM bridge rejected frame: {error:?}"))?;

            self.chains
                .get_mut(&stream_id)
                .ok_or_else(|| "PCM bridge stream state missing".to_owned())?
                .process(&mut pcm);

            for (dst, src) in frame.samples_mut().iter_mut().zip(pcm.samples().iter()) {
                *dst = float_to_i16(*src);
            }
        }

        frame
            .set_flags(frame.flags() | PCM_BRIDGE_FLAG_OK)
            .map_err(|error| format!("PCM bridge response flag failed: {error:?}"))?;
        Ok(())
    }
}

fn float_to_i16(sample: f32) -> i16 {
    if sample >= 1.0 {
        i16::MAX
    } else if sample <= -1.0 {
        i16::MIN
    } else {
        (sample * 32_768.0).round() as i16
    }
}

fn serve_pcm_bridge(socket_path: &str) -> Result<(), String> {
    let path = Path::new(socket_path);
    if let Ok(metadata) = fs::symlink_metadata(path) {
        if !metadata.file_type().is_socket() {
            return Err(format!(
                "refusing to replace non-socket path: {}",
                path.display()
            ));
        }
        fs::remove_file(path)
            .map_err(|error| format!("remove stale socket {}: {error}", path.display()))?;
    }

    let parent = path
        .parent()
        .ok_or_else(|| "PCM bridge socket requires a parent directory".to_owned())?;
    if !parent.exists() {
        return Err(format!(
            "PCM bridge parent directory does not exist: {}",
            parent.display()
        ));
    }

    let listener = UnixListener::bind(path)
        .map_err(|error| format!("bind PCM bridge {}: {error}", path.display()))?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o660))
        .map_err(|error| format!("chmod PCM bridge {}: {error}", path.display()))?;

    println!("Helix PCM bridge listening on {}", path.display());
    println!("External legacy codec backend remains outside Helix.");

    let mut state = BridgeState::default();
    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                if let Err(error) = handle_client(&mut stream, &mut state) {
                    eprintln!("helix-daemon: PCM bridge client closed: {error}");
                }
            }
            Err(error) => eprintln!("helix-daemon: PCM bridge accept failed: {error}"),
        }
    }
    Ok(())
}

fn serve_pcm_observe(socket_path: &str) -> Result<(), String> {
    let path = Path::new(socket_path);
    if let Ok(metadata) = fs::symlink_metadata(path) {
        if !metadata.file_type().is_socket() {
            return Err(format!(
                "refusing to replace non-socket path: {}",
                path.display()
            ));
        }
        fs::remove_file(path)
            .map_err(|error| format!("remove stale socket {}: {error}", path.display()))?;
    }

    let parent = path
        .parent()
        .ok_or_else(|| "PCM observe socket requires a parent directory".to_owned())?;
    if !parent.exists() {
        return Err(format!(
            "PCM observe parent directory does not exist: {}",
            parent.display()
        ));
    }

    let socket = UnixDatagram::bind(path)
        .map_err(|error| format!("bind PCM observe {}: {error}", path.display()))?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o660))
        .map_err(|error| format!("chmod PCM observe {}: {error}", path.display()))?;

    println!("Helix PCM observer listening on {}", path.display());
    println!("Observer is one-way and never participates in legacy audio delivery.");

    let mut state = BridgeState::default();
    loop {
        let mut wire = [0_u8; PCM_BRIDGE_MAX_PACKET_LEN];
        let received = socket
            .recv(&mut wire)
            .map_err(|error| format!("receive PCM observe datagram: {error}"))?;
        if received < PCM_BRIDGE_HEADER_LEN {
            continue;
        }

        let Ok(mut frame) = PcmBridgeFrame::decode(&wire[..received]) else {
            continue;
        };
        let _ = state.process(&mut frame);
    }
}

fn handle_client(stream: &mut UnixStream, state: &mut BridgeState) -> Result<(), String> {
    loop {
        let mut wire = [0_u8; PCM_BRIDGE_MAX_PACKET_LEN];
        match stream.read_exact(&mut wire[..PCM_BRIDGE_HEADER_LEN]) {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::UnexpectedEof => return Ok(()),
            Err(error) => return Err(format!("read PCM header: {error}")),
        }

        let sample_count = u16::from_le_bytes([wire[6], wire[7]]) as usize;
        if sample_count == 0 || sample_count > PCM_BRIDGE_MAX_SAMPLES {
            return Err("invalid PCM sample count".to_owned());
        }
        let packet_len = PCM_BRIDGE_HEADER_LEN + sample_count * core::mem::size_of::<i16>();
        stream
            .read_exact(&mut wire[PCM_BRIDGE_HEADER_LEN..packet_len])
            .map_err(|error| format!("read PCM payload: {error}"))?;

        let mut frame = PcmBridgeFrame::decode(&wire[..packet_len])
            .map_err(|error| format!("decode PCM bridge frame: {error:?}"))?;
        state.process(&mut frame)?;

        let response_len = frame
            .encode(&mut wire)
            .map_err(|error| format!("encode PCM bridge response: {error:?}"))?;
        stream
            .write_all(&wire[..response_len])
            .map_err(|error| format!("write PCM response: {error}"))?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_self_test_passes_without_network() {
        self_test().expect("daemon self-test");
    }

    #[test]
    fn default_xlx_mode_is_disabled() {
        assert_eq!(XlxBridgeConfig::default().mode, IntegrationMode::Disabled);
    }

    #[test]
    fn observe_only_bridge_is_bit_exact() {
        let input = [-30_000_i16, -1000, 0, 1000, 30_000];
        let mut frame = PcmBridgeFrame::new(0, 42, 8_000, 0, &input).expect("frame");
        BridgeState::default().process(&mut frame).expect("process");

        assert_eq!(frame.samples(), input);
        assert_ne!(frame.flags() & PCM_BRIDGE_FLAG_OK, 0);
        assert_eq!(frame.flags() & PCM_BRIDGE_FLAG_APPLY_DSP, 0);
    }

    #[test]
    fn stale_stream_state_is_evicted_without_unbounded_growth() {
        let input = [1000_i16; 160];
        let mut state = BridgeState::default();

        for stream_id in 1..=(XLXD_MAX_STREAMS as u32 + 20) {
            let mut frame = PcmBridgeFrame::new(
                PCM_BRIDGE_FLAG_APPLY_DSP | PCM_BRIDGE_FLAG_RESET_STREAM,
                stream_id,
                8_000,
                0,
                &input,
            )
            .expect("frame");
            state.process(&mut frame).expect("process");
        }

        assert_eq!(state.chains.len(), XLXD_MAX_STREAMS as usize);
        assert_eq!(state.order.len(), XLXD_MAX_STREAMS as usize);
    }

    #[test]
    fn processing_bridge_is_bounded() {
        let input = [10_000_i16; 160];
        let mut frame = PcmBridgeFrame::new(
            PCM_BRIDGE_FLAG_APPLY_DSP | PCM_BRIDGE_FLAG_RESET_STREAM,
            1,
            8_000,
            0,
            &input,
        )
        .expect("frame");
        BridgeState::default().process(&mut frame).expect("process");

        assert_eq!(frame.samples().len(), input.len());
        assert_ne!(frame.flags() & PCM_BRIDGE_FLAG_OK, 0);
        assert!(frame.samples().iter().all(|sample| *sample != i16::MIN));
    }
}
