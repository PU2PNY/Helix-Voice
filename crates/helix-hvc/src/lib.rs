#![forbid(unsafe_code)]

use helix_core::{CodecAdapter, CodecKind, FrameError, PcmFrame};

pub const HVC_SAMPLE_RATE_HZ: u32 = 8_000;
pub const HVC_FRAME_SAMPLES: usize = 160;
pub const HVC_FRAME_DURATION_MS: u32 = 20;
pub const HVC_PACKET_BYTES: usize = 12;
pub const HVC_BITRATE_BPS: u32 = 4_800;

const HVC_MAGIC: u8 = 0x48;
const HVC_VERSION: u8 = 0;
const LPC_ORDER: usize = 8;
const MIN_PITCH_LAG: usize = 20;
const MAX_PITCH_LAG: usize = 114;
const REFLECTION_LIMIT: f32 = 0.95;
const VOICED_CORRELATION_THRESHOLD: f32 = 0.38;
const VOICED_RMS_THRESHOLD: f32 = 0.003;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HvcError {
    WrongSampleRate,
    WrongFrameLength,
    InvalidPacketMagic,
    UnsupportedVersion,
    ChecksumMismatch,
    InvalidPacketFields,
    Frame(FrameError),
}

impl From<FrameError> for HvcError {
    fn from(value: FrameError) -> Self {
        Self::Frame(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HvcPacket {
    bytes: [u8; HVC_PACKET_BYTES],
}

impl HvcPacket {
    pub fn from_bytes(bytes: [u8; HVC_PACKET_BYTES]) -> Result<Self, HvcError> {
        validate_packet(&bytes)?;
        Ok(Self { bytes })
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; HVC_PACKET_BYTES] {
        &self.bytes
    }

    #[must_use]
    pub const fn into_bytes(self) -> [u8; HVC_PACKET_BYTES] {
        self.bytes
    }

    #[must_use]
    pub const fn is_voiced(&self) -> bool {
        self.bytes[2] & 0x80 != 0
    }

    #[must_use]
    pub const fn energy_index(&self) -> u8 {
        self.bytes[2] & 0x7f
    }

    #[must_use]
    pub const fn pitch_lag(&self) -> u8 {
        self.bytes[3]
    }

    #[must_use]
    pub const fn pitch_confidence_index(&self) -> u8 {
        self.bytes[4] & 0x3f
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct HvcCodec;

impl CodecAdapter for HvcCodec {
    fn name(&self) -> &'static str {
        "hvc-v0"
    }

    fn kind(&self) -> CodecKind {
        CodecKind::Open
    }
}

#[derive(Debug, Default)]
pub struct HvcEncoder;

impl HvcEncoder {
    pub fn encode(&mut self, frame: &PcmFrame) -> Result<HvcPacket, HvcError> {
        if frame.sample_rate_hz() != HVC_SAMPLE_RATE_HZ {
            return Err(HvcError::WrongSampleRate);
        }

        let samples = frame.samples();
        if samples.len() != HVC_FRAME_SAMPLES {
            return Err(HvcError::WrongFrameLength);
        }

        let level = rms(samples);
        let energy_index = quantize_energy(level);
        let reflection = analyze_reflection(samples);
        let (pitch_lag, pitch_correlation) = estimate_pitch(samples);

        let voiced = level >= VOICED_RMS_THRESHOLD
            && pitch_correlation >= VOICED_CORRELATION_THRESHOLD
            && (MIN_PITCH_LAG..=MAX_PITCH_LAG).contains(&pitch_lag);

        let mut bytes = [0_u8; HVC_PACKET_BYTES];
        bytes[0] = HVC_MAGIC;
        bytes[1] = HVC_VERSION;
        bytes[2] = energy_index | if voiced { 0x80 } else { 0 };
        bytes[3] = if voiced { pitch_lag as u8 } else { 0 };
        bytes[4] = if voiced {
            quantize_unit_6(pitch_correlation)
        } else {
            0
        };
        pack_reflection(&reflection, &mut bytes[5..11]);
        bytes[11] = crc8(&bytes[..11]);

        HvcPacket::from_bytes(bytes)
    }
}

#[derive(Debug)]
pub struct HvcDecoder {
    synthesis_history: [f32; LPC_ORDER],
    pitch_countdown: usize,
    rng_state: u32,
}

impl Default for HvcDecoder {
    fn default() -> Self {
        Self {
            synthesis_history: [0.0; LPC_ORDER],
            pitch_countdown: 0,
            rng_state: 0x48_56_43_30,
        }
    }
}

impl HvcDecoder {
    pub fn decode(
        &mut self,
        packet: &HvcPacket,
        timestamp_samples: u64,
    ) -> Result<PcmFrame, HvcError> {
        let bytes = packet.as_bytes();
        validate_packet(bytes)?;

        let target_rms = dequantize_energy(packet.energy_index());
        let mut output = [0.0_f32; HVC_FRAME_SAMPLES];

        if packet.energy_index() == 0 {
            for sample in &mut self.synthesis_history {
                *sample *= 0.5;
            }
            return Ok(PcmFrame::from_slice(
                HVC_SAMPLE_RATE_HZ,
                timestamp_samples,
                &output,
            )?);
        }

        let reflection = unpack_reflection(&bytes[5..11]);
        let lpc = reflection_to_lpc(&reflection);
        let voiced = packet.is_voiced();
        let pitch_lag = usize::from(packet.pitch_lag()).max(MIN_PITCH_LAG);
        let pitch_confidence = f32::from(packet.pitch_confidence_index()) / 63.0;

        for sample in &mut output {
            let noise = self.next_noise();
            let excitation = if voiced {
                let pulse = if self.pitch_countdown == 0 {
                    self.pitch_countdown = pitch_lag.saturating_sub(1);
                    1.0
                } else {
                    self.pitch_countdown -= 1;
                    0.0
                };

                let periodic = pulse - (1.0 / pitch_lag as f32);
                periodic * (0.55 + 0.45 * pitch_confidence)
                    + noise * (1.0 - pitch_confidence) * 0.35
            } else {
                noise
            };

            let mut synthesized = excitation;
            let mut order = 1_usize;
            while order <= LPC_ORDER {
                synthesized -= lpc[order] * self.synthesis_history[order - 1];
                order += 1;
            }

            let mut history_index = LPC_ORDER - 1;
            while history_index > 0 {
                self.synthesis_history[history_index] =
                    self.synthesis_history[history_index - 1];
                history_index -= 1;
            }
            self.synthesis_history[0] = synthesized.clamp(-4.0, 4.0);
            *sample = self.synthesis_history[0];
        }

        normalize_rms(&mut output, target_rms);

        Ok(PcmFrame::from_slice(
            HVC_SAMPLE_RATE_HZ,
            timestamp_samples,
            &output,
        )?)
    }

    fn next_noise(&mut self) -> f32 {
        let mut x = self.rng_state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng_state = x;
        (x as f32 / u32::MAX as f32) * 2.0 - 1.0
    }
}

fn validate_packet(bytes: &[u8; HVC_PACKET_BYTES]) -> Result<(), HvcError> {
    if bytes[0] != HVC_MAGIC {
        return Err(HvcError::InvalidPacketMagic);
    }
    if bytes[1] != HVC_VERSION {
        return Err(HvcError::UnsupportedVersion);
    }
    if bytes[4] & 0xc0 != 0 {
        return Err(HvcError::InvalidPacketFields);
    }
    if crc8(&bytes[..11]) != bytes[11] {
        return Err(HvcError::ChecksumMismatch);
    }

    let voiced = bytes[2] & 0x80 != 0;
    let pitch_lag = usize::from(bytes[3]);
    if voiced {
        if !(MIN_PITCH_LAG..=MAX_PITCH_LAG).contains(&pitch_lag) {
            return Err(HvcError::InvalidPacketFields);
        }
    } else if pitch_lag != 0 || bytes[4] != 0 {
        return Err(HvcError::InvalidPacketFields);
    }

    Ok(())
}

fn rms(samples: &[f32]) -> f32 {
    let sum = samples.iter().map(|sample| sample * sample).sum::<f32>();
    (sum / samples.len() as f32).sqrt()
}

fn quantize_energy(value: f32) -> u8 {
    if value < 0.001 {
        return 0;
    }

    let db = 20.0 * value.max(1.0e-6).log10();
    let normalized = ((db + 50.0) / 47.0).clamp(0.0, 1.0);
    (1.0 + normalized * 126.0).round() as u8
}

fn dequantize_energy(index: u8) -> f32 {
    if index == 0 {
        return 0.0;
    }

    let normalized = f32::from(index - 1) / 126.0;
    let db = -50.0 + normalized * 47.0;
    10.0_f32.powf(db / 20.0)
}

fn quantize_unit_6(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 63.0).round() as u8
}

fn estimate_pitch(samples: &[f32]) -> (usize, f32) {
    let mean = samples.iter().sum::<f32>() / samples.len() as f32;
    let mut best_lag = 0_usize;
    let mut best_correlation = 0.0_f32;

    let mut lag = MIN_PITCH_LAG;
    while lag <= MAX_PITCH_LAG {
        let mut cross = 0.0_f32;
        let mut left_energy = 0.0_f32;
        let mut right_energy = 0.0_f32;

        let mut index = lag;
        while index < samples.len() {
            let left = samples[index] - mean;
            let right = samples[index - lag] - mean;
            cross += left * right;
            left_energy += left * left;
            right_energy += right * right;
            index += 1;
        }

        let denominator = (left_energy * right_energy).sqrt();
        let correlation = if denominator > 1.0e-9 {
            (cross / denominator).clamp(-1.0, 1.0)
        } else {
            0.0
        };

        if correlation > best_correlation {
            best_correlation = correlation;
            best_lag = lag;
        }

        lag += 1;
    }

    (best_lag, best_correlation.max(0.0))
}

fn analyze_reflection(samples: &[f32]) -> [f32; LPC_ORDER] {
    let mean = samples.iter().sum::<f32>() / samples.len() as f32;
    let mut windowed = [0.0_f32; HVC_FRAME_SAMPLES];

    for (index, (destination, source)) in windowed.iter_mut().zip(samples).enumerate() {
        let phase = 2.0 * core::f32::consts::PI * index as f32
            / (HVC_FRAME_SAMPLES.saturating_sub(1)) as f32;
        let window = 0.54 - 0.46 * phase.cos();
        *destination = (*source - mean) * window;
    }

    let mut autocorrelation = [0.0_f32; LPC_ORDER + 1];
    for (lag, value) in autocorrelation.iter_mut().enumerate() {
        *value = windowed
            .iter()
            .zip(windowed.iter().skip(lag))
            .map(|(left, right)| left * right)
            .sum();
    }

    levinson_reflection(&autocorrelation)
}

fn levinson_reflection(autocorrelation: &[f32; LPC_ORDER + 1]) -> [f32; LPC_ORDER] {
    let mut reflection = [0.0_f32; LPC_ORDER];
    if autocorrelation[0] <= 1.0e-9 {
        return reflection;
    }

    let mut coefficients = [0.0_f32; LPC_ORDER + 1];
    coefficients[0] = 1.0;
    let mut error = autocorrelation[0];

    let mut order = 1_usize;
    while order <= LPC_ORDER {
        let mut accumulator = autocorrelation[order];
        let mut index = 1_usize;
        while index < order {
            accumulator += coefficients[index] * autocorrelation[order - index];
            index += 1;
        }

        let coefficient = (-accumulator / error).clamp(-REFLECTION_LIMIT, REFLECTION_LIMIT);
        reflection[order - 1] = coefficient;

        let previous = coefficients;
        let mut update = 1_usize;
        while update < order {
            coefficients[update] =
                previous[update] + coefficient * previous[order - update];
            update += 1;
        }
        coefficients[order] = coefficient;

        error *= (1.0 - coefficient * coefficient).max(1.0e-4);
        order += 1;
    }

    reflection
}

fn reflection_to_lpc(reflection: &[f32; LPC_ORDER]) -> [f32; LPC_ORDER + 1] {
    let mut coefficients = [0.0_f32; LPC_ORDER + 1];
    coefficients[0] = 1.0;

    let mut order = 1_usize;
    while order <= LPC_ORDER {
        let coefficient = reflection[order - 1].clamp(-REFLECTION_LIMIT, REFLECTION_LIMIT);
        let previous = coefficients;

        let mut index = 1_usize;
        while index < order {
            coefficients[index] =
                previous[index] + coefficient * previous[order - index];
            index += 1;
        }
        coefficients[order] = coefficient;
        order += 1;
    }

    coefficients
}

fn pack_reflection(reflection: &[f32; LPC_ORDER], destination: &mut [u8]) {
    debug_assert_eq!(destination.len(), 6);
    let mut packed = 0_u64;

    for (index, coefficient) in reflection.iter().enumerate() {
        let normalized = ((*coefficient + REFLECTION_LIMIT) / (2.0 * REFLECTION_LIMIT))
            .clamp(0.0, 1.0);
        let quantized = (normalized * 63.0).round() as u64;
        packed |= quantized << (index * 6);
    }

    for (index, byte) in destination.iter_mut().enumerate() {
        *byte = ((packed >> (index * 8)) & 0xff) as u8;
    }
}

fn unpack_reflection(source: &[u8]) -> [f32; LPC_ORDER] {
    debug_assert_eq!(source.len(), 6);
    let mut packed = 0_u64;

    for (index, byte) in source.iter().enumerate() {
        packed |= u64::from(*byte) << (index * 8);
    }

    let mut reflection = [0.0_f32; LPC_ORDER];
    for (index, coefficient) in reflection.iter_mut().enumerate() {
        let quantized = ((packed >> (index * 6)) & 0x3f) as u8;
        let normalized = f32::from(quantized) / 63.0;
        *coefficient = normalized * (2.0 * REFLECTION_LIMIT) - REFLECTION_LIMIT;
    }

    reflection
}

fn normalize_rms(samples: &mut [f32], target_rms: f32) {
    let current = rms(samples);
    if current <= 1.0e-9 || target_rms <= 0.0 {
        return;
    }

    let gain = (target_rms / current).clamp(0.05, 20.0);
    for sample in samples {
        *sample = (*sample * gain).clamp(-1.0, 1.0);
    }
}

fn crc8(data: &[u8]) -> u8 {
    let mut crc = 0_u8;

    for byte in data {
        crc ^= *byte;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 {
                crc.wrapping_shl(1) ^ 0x07
            } else {
                crc.wrapping_shl(1)
            };
        }
    }

    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine_frame(frequency_hz: f32, amplitude: f32) -> PcmFrame {
        let mut samples = [0.0_f32; HVC_FRAME_SAMPLES];
        for (index, sample) in samples.iter_mut().enumerate() {
            let phase = 2.0 * core::f32::consts::PI * frequency_hz * index as f32
                / HVC_SAMPLE_RATE_HZ as f32;
            *sample = amplitude * phase.sin();
        }
        PcmFrame::from_slice(HVC_SAMPLE_RATE_HZ, 0, &samples).expect("synthetic frame")
    }

    #[test]
    fn v0_operating_point_is_4800_bit_per_second() {
        assert_eq!(
            HVC_PACKET_BYTES as u32 * 8 * (1_000 / HVC_FRAME_DURATION_MS),
            HVC_BITRATE_BPS
        );
    }

    #[test]
    fn silence_round_trip_stays_silent() {
        let input = [0.0_f32; HVC_FRAME_SAMPLES];
        let frame =
            PcmFrame::from_slice(HVC_SAMPLE_RATE_HZ, 0, &input).expect("valid silence frame");
        let mut encoder = HvcEncoder;
        let packet = encoder.encode(&frame).expect("encode silence");
        assert!(!packet.is_voiced());
        assert_eq!(packet.energy_index(), 0);

        let mut decoder = HvcDecoder::default();
        let decoded = decoder.decode(&packet, 0).expect("decode silence");
        assert!(decoded.samples().iter().all(|sample| sample.abs() < 1.0e-9));
    }

    #[test]
    fn detects_approximately_100_hz_pitch() {
        let frame = sine_frame(100.0, 0.2);
        let mut encoder = HvcEncoder;
        let packet = encoder.encode(&frame).expect("encode tone");
        assert!(packet.is_voiced());
        assert!((75..=85).contains(&packet.pitch_lag()));
    }

    #[test]
    fn voiced_frame_encodes_and_decodes_to_finite_pcm() {
        let frame = sine_frame(125.0, 0.15);
        let mut encoder = HvcEncoder;
        let packet = encoder.encode(&frame).expect("encode tone");

        let mut decoder = HvcDecoder::default();
        let decoded = decoder.decode(&packet, 160).expect("decode tone");

        assert_eq!(decoded.samples().len(), HVC_FRAME_SAMPLES);
        assert!(decoded.samples().iter().all(|sample| sample.is_finite()));
        assert!(decoded.samples().iter().all(|sample| sample.abs() <= 1.0));
        assert!(rms(decoded.samples()) > 0.001);
    }

    #[test]
    fn corrupted_packet_is_rejected() {
        let frame = sine_frame(100.0, 0.2);
        let mut encoder = HvcEncoder;
        let packet = encoder.encode(&frame).expect("encode tone");
        let mut bytes = packet.into_bytes();
        bytes[6] ^= 0x20;

        assert_eq!(
            HvcPacket::from_bytes(bytes),
            Err(HvcError::ChecksumMismatch)
        );
    }

    #[test]
    fn wrong_frame_shape_is_rejected() {
        let input = [0.1_f32; 80];
        let frame =
            PcmFrame::from_slice(HVC_SAMPLE_RATE_HZ, 0, &input).expect("valid core frame");
        let mut encoder = HvcEncoder;
        assert_eq!(encoder.encode(&frame), Err(HvcError::WrongFrameLength));
    }

    #[test]
    fn hvc_reports_open_codec_kind() {
        let codec = HvcCodec;
        assert_eq!(codec.name(), "hvc-v0");
        assert_eq!(codec.kind(), CodecKind::Open);
    }
}
