#![forbid(unsafe_code)]

use helix_core::PcmFrame;
use helix_dsp::Decimator2;
use helix_hvc::{
    HVC_BITRATE_BPS, HVC_FRAME_SAMPLES, HVC_PACKET_BYTES, HVC_SAMPLE_RATE_HZ, HvcDecoder,
    HvcEncoder,
};
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;
use std::time::Instant;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("helix-lab: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("roundtrip") => {
            let input = args
                .get(2)
                .ok_or_else(|| "missing input WAV path".to_owned())?;
            let output = args
                .get(3)
                .ok_or_else(|| "missing output WAV path".to_owned())?;
            roundtrip_file(Path::new(input), Path::new(output))
        }
        Some("benchmark") => benchmark(),
        Some("self-test") => self_test(),
        _ => {
            println!("Helix Voice laboratory");
            println!("Usage:");
            println!("  helix-lab roundtrip <input.wav> <output.wav>");
            println!("  helix-lab benchmark");
            println!("  helix-lab self-test");
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
struct WavPcm16Mono {
    sample_rate_hz: u32,
    samples: Vec<i16>,
}

#[derive(Debug, Clone, Copy)]
struct AudioMetrics {
    rms: f32,
    peak: f32,
    clipping_samples: usize,
}

fn roundtrip_file(input: &Path, output: &Path) -> Result<(), String> {
    let wav = read_wav(input)?;
    validate_lab_wav(&wav)?;

    let input_pcm = prepare_hvc_pcm(&wav)?;
    let input_metrics = measure(&input_pcm);

    let started = Instant::now();
    let decoded = hvc_roundtrip(&input_pcm)?;
    let elapsed = started.elapsed();
    let output_metrics = measure(&decoded);
    let spectral_distance = band_log_spectral_distance_db(&input_pcm, &decoded);

    write_wav(
        output,
        &WavPcm16Mono {
            sample_rate_hz: HVC_SAMPLE_RATE_HZ,
            samples: decoded.into_iter().map(f32_to_i16).collect(),
        },
    )?;

    let frame_count = input_pcm.len().div_ceil(HVC_FRAME_SAMPLES);
    let encoded_bytes = frame_count * HVC_PACKET_BYTES;
    let audio_seconds = input_pcm.len() as f64 / f64::from(HVC_SAMPLE_RATE_HZ);
    let realtime_factor = if elapsed.as_secs_f64() > 0.0 {
        audio_seconds / elapsed.as_secs_f64()
    } else {
        f64::INFINITY
    };

    println!("HVC v0 WAV round-trip complete");
    println!("input:  {}", input.display());
    println!("output: {}", output.display());
    println!("source_sample_rate_hz: {}", wav.sample_rate_hz);
    println!("hvc_sample_rate_hz: {}", HVC_SAMPLE_RATE_HZ);
    println!("samples: {}", input_pcm.len());
    println!("frames: {frame_count}");
    println!("encoded_bytes: {encoded_bytes}");
    println!("nominal_payload_bitrate_bps: {HVC_BITRATE_BPS}");
    println!(
        "input_rms={:.6} input_peak={:.6} input_clipped={}",
        input_metrics.rms, input_metrics.peak, input_metrics.clipping_samples
    );
    println!(
        "output_rms={:.6} output_peak={:.6} output_clipped={}",
        output_metrics.rms, output_metrics.peak, output_metrics.clipping_samples
    );
    if let Some(distance_db) = spectral_distance {
        println!("band_log_spectral_distance_db={distance_db:.3}");
    }
    println!("processing_ms={:.3}", elapsed.as_secs_f64() * 1_000.0);
    println!("realtime_factor={realtime_factor:.2}x");

    Ok(())
}

fn hvc_roundtrip(input: &[f32]) -> Result<Vec<f32>, String> {
    if input.is_empty() {
        return Err("input contains no samples".to_owned());
    }

    let mut encoder = HvcEncoder;
    let mut decoder = HvcDecoder::default();
    let mut output = Vec::with_capacity(input.len());

    for (frame_index, chunk) in input.chunks(HVC_FRAME_SAMPLES).enumerate() {
        let mut padded = [0.0_f32; HVC_FRAME_SAMPLES];
        padded[..chunk.len()].copy_from_slice(chunk);

        let timestamp = (frame_index * HVC_FRAME_SAMPLES) as u64;
        let frame = PcmFrame::from_slice(HVC_SAMPLE_RATE_HZ, timestamp, &padded)
            .map_err(|error| format!("PCM frame creation failed: {error:?}"))?;
        let packet = encoder
            .encode(&frame)
            .map_err(|error| format!("HVC encode failed: {error:?}"))?;
        let decoded = decoder
            .decode(&packet, timestamp)
            .map_err(|error| format!("HVC decode failed: {error:?}"))?;

        output.extend_from_slice(&decoded.samples()[..chunk.len()]);
    }

    Ok(output)
}

fn benchmark() -> Result<(), String> {
    const FRAMES: usize = 5_000;

    let mut encoder = HvcEncoder;
    let mut decoder = HvcDecoder::default();
    let started = Instant::now();
    let mut checksum = 0.0_f32;

    for frame_index in 0..FRAMES {
        let frequency = 85.0 + (frame_index % 160) as f32;
        let samples = synthetic_voiced_frame(frequency, 0.12);
        let timestamp = (frame_index * HVC_FRAME_SAMPLES) as u64;
        let frame = PcmFrame::from_slice(HVC_SAMPLE_RATE_HZ, timestamp, &samples)
            .map_err(|error| format!("benchmark frame failed: {error:?}"))?;
        let packet = encoder
            .encode(&frame)
            .map_err(|error| format!("benchmark encode failed: {error:?}"))?;
        let decoded = decoder
            .decode(&packet, timestamp)
            .map_err(|error| format!("benchmark decode failed: {error:?}"))?;
        checksum += decoded.samples()[0];
    }

    let elapsed = started.elapsed();
    let audio_seconds = FRAMES as f64 * HVC_FRAME_SAMPLES as f64 / f64::from(HVC_SAMPLE_RATE_HZ);
    let realtime_factor = audio_seconds / elapsed.as_secs_f64().max(f64::EPSILON);

    println!("HVC v0 synthetic benchmark");
    println!("frames: {FRAMES}");
    println!("audio_seconds: {audio_seconds:.1}");
    println!("processing_ms: {:.3}", elapsed.as_secs_f64() * 1_000.0);
    println!("realtime_factor: {realtime_factor:.2}x");
    println!("checksum: {checksum:.6}");
    println!("Note: this is host-specific SW evidence, not target-hardware evidence.");

    Ok(())
}

fn self_test() -> Result<(), String> {
    let samples: Vec<f32> = (0..(HVC_FRAME_SAMPLES * 50))
        .map(|index| {
            let time = index as f32 / HVC_SAMPLE_RATE_HZ as f32;
            0.14 * (2.0 * core::f32::consts::PI * 125.0 * time).sin()
        })
        .collect();

    let output = hvc_roundtrip(&samples)?;
    if output.len() != samples.len() {
        return Err("self-test changed sample count".to_owned());
    }
    if output
        .iter()
        .any(|sample| !sample.is_finite() || sample.abs() > 1.0)
    {
        return Err("self-test produced invalid PCM".to_owned());
    }

    println!("helix-lab self-test: PASS");
    Ok(())
}

fn validate_lab_wav(wav: &WavPcm16Mono) -> Result<(), String> {
    if wav.sample_rate_hz != HVC_SAMPLE_RATE_HZ && wav.sample_rate_hz != 16_000 {
        return Err(format!(
            "HVC v0 lab accepts 8000 or 16000 Hz mono PCM; input is {} Hz",
            wav.sample_rate_hz
        ));
    }
    if wav.samples.is_empty() {
        return Err("WAV data chunk is empty".to_owned());
    }
    Ok(())
}

fn prepare_hvc_pcm(wav: &WavPcm16Mono) -> Result<Vec<f32>, String> {
    validate_lab_wav(wav)?;

    let input: Vec<f32> = wav
        .samples
        .iter()
        .map(|sample| i16_to_f32(*sample))
        .collect();

    if wav.sample_rate_hz == HVC_SAMPLE_RATE_HZ {
        return Ok(input);
    }

    let mut decimator = Decimator2::default();
    let required = decimator.required_output_len(input.len());
    let mut output = vec![0.0_f32; required];
    let produced = decimator
        .process(&input, &mut output)
        .map_err(|error| format!("16 kHz to 8 kHz resampling failed: {error:?}"))?;
    output.truncate(produced);
    Ok(output)
}

fn measure(samples: &[f32]) -> AudioMetrics {
    if samples.is_empty() {
        return AudioMetrics {
            rms: 0.0,
            peak: 0.0,
            clipping_samples: 0,
        };
    }

    let mut sum_sq = 0.0_f32;
    let mut peak = 0.0_f32;
    let mut clipping_samples = 0_usize;

    for sample in samples {
        sum_sq += sample * sample;
        peak = peak.max(sample.abs());
        if sample.abs() >= 1.0 {
            clipping_samples += 1;
        }
    }

    AudioMetrics {
        rms: (sum_sq / samples.len() as f32).sqrt(),
        peak,
        clipping_samples,
    }
}

fn band_log_spectral_distance_db(reference: &[f32], test: &[f32]) -> Option<f32> {
    const BANDS: usize = 8;
    const SILENCE_RMS: f32 = 0.003;
    const POWER_FLOOR: f32 = 1.0e-12;

    let frame_count = reference.len().min(test.len()) / HVC_FRAME_SAMPLES;
    let mut total = 0.0_f32;
    let mut active_frames = 0_usize;

    for frame_index in 0..frame_count {
        let start = frame_index * HVC_FRAME_SAMPLES;
        let end = start + HVC_FRAME_SAMPLES;
        let reference_frame = &reference[start..end];
        let test_frame = &test[start..end];

        if measure(reference_frame).rms < SILENCE_RMS {
            continue;
        }

        let reference_bands = band_log_energies(reference_frame, POWER_FLOOR);
        let test_bands = band_log_energies(test_frame, POWER_FLOOR);

        let squared_error = reference_bands
            .iter()
            .zip(test_bands)
            .map(|(left, right)| {
                let difference = *left - right;
                difference * difference
            })
            .sum::<f32>()
            / BANDS as f32;

        total += squared_error.sqrt();
        active_frames += 1;
    }

    if active_frames == 0 {
        None
    } else {
        Some(total / active_frames as f32)
    }
}

fn band_log_energies(frame: &[f32], floor: f32) -> [f32; 8] {
    const BANDS: usize = 8;
    let mut bands = [0.0_f32; BANDS];
    let frame_len = frame.len() as f32;

    for bin in 1..(HVC_FRAME_SAMPLES / 2) {
        let mut real = 0.0_f32;
        let mut imag = 0.0_f32;

        for (index, sample) in frame.iter().enumerate() {
            let window_phase =
                2.0 * core::f32::consts::PI * index as f32 / (HVC_FRAME_SAMPLES - 1) as f32;
            let window = 0.54 - 0.46 * window_phase.cos();
            let angle =
                2.0 * core::f32::consts::PI * bin as f32 * index as f32 / frame_len;
            let weighted = *sample * window;
            real += weighted * angle.cos();
            imag -= weighted * angle.sin();
        }

        let power = real * real + imag * imag;
        let band = (bin * BANDS / (HVC_FRAME_SAMPLES / 2)).min(BANDS - 1);
        bands[band] += power;
    }

    for energy in &mut bands {
        *energy = 10.0 * energy.max(floor).log10();
    }

    bands
}

fn synthetic_voiced_frame(frequency_hz: f32, amplitude: f32) -> [f32; HVC_FRAME_SAMPLES] {
    let mut samples = [0.0_f32; HVC_FRAME_SAMPLES];
    for (index, sample) in samples.iter_mut().enumerate() {
        let phase =
            2.0 * core::f32::consts::PI * frequency_hz * index as f32 / HVC_SAMPLE_RATE_HZ as f32;
        *sample = amplitude * phase.sin();
    }
    samples
}

fn i16_to_f32(sample: i16) -> f32 {
    f32::from(sample) / 32_768.0
}

fn f32_to_i16(sample: f32) -> i16 {
    let clamped = sample.clamp(-1.0, 1.0);
    if clamped <= -1.0 {
        i16::MIN
    } else {
        (clamped * f32::from(i16::MAX)).round() as i16
    }
}

fn read_wav(path: &Path) -> Result<WavPcm16Mono, String> {
    let bytes =
        fs::read(path).map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    parse_wav(&bytes)
}

fn parse_wav(bytes: &[u8]) -> Result<WavPcm16Mono, String> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err("unsupported or invalid RIFF/WAVE file".to_owned());
    }

    let mut offset = 12_usize;
    let mut sample_rate_hz = None;
    let mut data_range = None;

    while offset + 8 <= bytes.len() {
        let id = &bytes[offset..offset + 4];
        let size = read_u32_le(bytes, offset + 4)
            .ok_or_else(|| "truncated WAV chunk size".to_owned())? as usize;
        let start = offset + 8;
        let end = start
            .checked_add(size)
            .ok_or_else(|| "WAV chunk size overflow".to_owned())?;
        if end > bytes.len() {
            return Err("truncated WAV chunk".to_owned());
        }

        if id == b"fmt " {
            if size < 16 {
                return Err("WAV fmt chunk is too short".to_owned());
            }
            let audio_format = read_u16_le(bytes, start).ok_or_else(|| "bad fmt".to_owned())?;
            let channels =
                read_u16_le(bytes, start + 2).ok_or_else(|| "bad channel count".to_owned())?;
            let rate = read_u32_le(bytes, start + 4).ok_or_else(|| "bad sample rate".to_owned())?;
            let bits = read_u16_le(bytes, start + 14).ok_or_else(|| "bad bit depth".to_owned())?;

            if audio_format != 1 {
                return Err("only integer PCM WAV is supported".to_owned());
            }
            if channels != 1 {
                return Err("only mono WAV is supported".to_owned());
            }
            if bits != 16 {
                return Err("only 16-bit PCM WAV is supported".to_owned());
            }
            sample_rate_hz = Some(rate);
        } else if id == b"data" {
            data_range = Some((start, end));
        }

        offset = end + (size & 1);
    }

    let sample_rate_hz = sample_rate_hz.ok_or_else(|| "WAV fmt chunk not found".to_owned())?;
    let (start, end) = data_range.ok_or_else(|| "WAV data chunk not found".to_owned())?;
    if (end - start) % 2 != 0 {
        return Err("16-bit WAV data has odd byte count".to_owned());
    }

    let mut samples = Vec::with_capacity((end - start) / 2);
    for chunk in bytes[start..end].chunks_exact(2) {
        samples.push(i16::from_le_bytes([chunk[0], chunk[1]]));
    }

    Ok(WavPcm16Mono {
        sample_rate_hz,
        samples,
    })
}

fn write_wav(path: &Path, wav: &WavPcm16Mono) -> Result<(), String> {
    let bytes = encode_wav(wav)?;
    fs::write(path, bytes).map_err(|error| format!("cannot write {}: {error}", path.display()))
}

fn encode_wav(wav: &WavPcm16Mono) -> Result<Vec<u8>, String> {
    let data_bytes = wav
        .samples
        .len()
        .checked_mul(2)
        .ok_or_else(|| "WAV size overflow".to_owned())?;
    let data_bytes_u32 =
        u32::try_from(data_bytes).map_err(|_| "WAV data exceeds RIFF limit".to_owned())?;
    let riff_size = 36_u32
        .checked_add(data_bytes_u32)
        .ok_or_else(|| "WAV RIFF size overflow".to_owned())?;

    let mut bytes = Vec::with_capacity(44 + data_bytes);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&riff_size.to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&wav.sample_rate_hz.to_le_bytes());
    let byte_rate = wav
        .sample_rate_hz
        .checked_mul(2)
        .ok_or_else(|| "WAV byte rate overflow".to_owned())?;
    bytes.extend_from_slice(&byte_rate.to_le_bytes());
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_slice(&16_u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_bytes_u32.to_le_bytes());
    for sample in &wav.samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    Ok(bytes)
}

fn read_u16_le(bytes: &[u8], offset: usize) -> Option<u16> {
    let slice = bytes.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u32_le(bytes: &[u8], offset: usize) -> Option<u32> {
    let slice = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spectral_distance_is_zero_for_identical_signal() {
        let signal: Vec<f32> = (0..(HVC_FRAME_SAMPLES * 4))
            .map(|index| {
                let time = index as f32 / HVC_SAMPLE_RATE_HZ as f32;
                0.2 * (2.0 * core::f32::consts::PI * 440.0 * time).sin()
            })
            .collect();

        let distance =
            band_log_spectral_distance_db(&signal, &signal).expect("active speech-like signal");
        assert!(distance < 1.0e-4, "distance={distance}");
    }

    #[test]
    fn wav_round_trip_preserves_pcm16() {
        let original = WavPcm16Mono {
            sample_rate_hz: 8_000,
            samples: vec![0, 1, -1, i16::MAX, i16::MIN, 1234, -2345],
        };
        let bytes = encode_wav(&original).expect("encode wav");
        let parsed = parse_wav(&bytes).expect("parse wav");
        assert_eq!(parsed.sample_rate_hz, original.sample_rate_hz);
        assert_eq!(parsed.samples, original.samples);
    }

    #[test]
    fn hvc_lab_preserves_sample_count_and_bounds() {
        let input: Vec<f32> = (0..1_337)
            .map(|index| {
                let time = index as f32 / HVC_SAMPLE_RATE_HZ as f32;
                0.1 * (2.0 * core::f32::consts::PI * 110.0 * time).sin()
            })
            .collect();

        let output = hvc_roundtrip(&input).expect("hvc roundtrip");
        assert_eq!(output.len(), input.len());
        assert!(output.iter().all(|sample| sample.is_finite()));
        assert!(output.iter().all(|sample| sample.abs() <= 1.0));
    }

    #[test]
    fn accepts_16khz_and_resamples_for_hvc_v0() {
        let wav = WavPcm16Mono {
            sample_rate_hz: 16_000,
            samples: vec![0; 320],
        };
        let prepared = prepare_hvc_pcm(&wav).expect("resample");
        assert_eq!(prepared.len(), 160);
        assert!(prepared.iter().all(|sample| sample.is_finite()));
    }

    #[test]
    fn rejects_unsupported_sample_rate_for_hvc_v0() {
        let wav = WavPcm16Mono {
            sample_rate_hz: 44_100,
            samples: vec![0; 160],
        };
        assert!(validate_lab_wav(&wav).is_err());
    }
}
