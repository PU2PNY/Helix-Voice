#![forbid(unsafe_code)]

use helix_core::PcmFrame;
use helix_dsp::DspChain;
use helix_hvc::{HVC_FRAME_SAMPLES, HVC_SAMPLE_RATE_HZ, HvcDecoder, HvcEncoder};
use helix_xlx::{IntegrationMode, XlxBridgeConfig};
use std::env;
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
    match env::args().nth(1).as_deref() {
        Some("--version") => {
            println!("helix-daemon {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Some("--health") => {
            print_health();
            Ok(())
        }
        Some("--self-test") => self_test(),
        Some(argument) => Err(format!("unsupported argument: {argument}")),
        None => {
            let xlx = XlxBridgeConfig::default();
            println!(
                "Helix Voice Engine research daemon {}",
                env!("CARGO_PKG_VERSION")
            );
            println!("XLX integration mode: {:?}", xlx.mode);
            println!("Network activation: disabled");
            println!("No proprietary codec implementation is included.");
            println!("Use --health or --self-test for local diagnostics.");
            Ok(())
        }
    }
}

fn print_health() {
    let xlx = XlxBridgeConfig::default();
    let xlx_mode = match xlx.mode {
        IntegrationMode::Disabled => "disabled",
        IntegrationMode::ObserveOnly => "observe-only",
        IntegrationMode::ExternalLicensedBackend => "external-licensed-backend",
    };

    println!(
        "{{\"status\":\"ready\",\"version\":\"{}\",\"dsp\":\"ready\",\"hvc\":\"available\",\"xlx\":\"{}\",\"network\":\"disabled\"}}",
        env!("CARGO_PKG_VERSION"),
        xlx_mode
    );
}

fn self_test() -> Result<(), String> {
    let mut input = [0.0_f32; HVC_FRAME_SAMPLES];
    for (index, sample) in input.iter_mut().enumerate() {
        let phase = 2.0 * core::f32::consts::PI * 125.0 * index as f32
            / HVC_SAMPLE_RATE_HZ as f32;
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

    let xlx = XlxBridgeConfig::default();
    if xlx.mode != IntegrationMode::Disabled {
        return Err("XLX must remain disabled by default".to_owned());
    }

    println!("helix-daemon self-test: PASS");
    println!("DSP: PASS");
    println!("HVC encode/decode: PASS");
    println!("XLX default disabled: PASS");
    println!("Network activation: disabled");
    Ok(())
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
        assert_eq!(
            XlxBridgeConfig::default().mode,
            IntegrationMode::Disabled
        );
    }
}
