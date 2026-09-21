#![forbid(unsafe_code)]

use helix_core::PcmFrame;

#[derive(Debug, Clone, Copy)]
pub struct Level {
    pub rms: f32,
    pub peak: f32,
}

#[must_use]
pub fn measure(samples: &[f32]) -> Level {
    if samples.is_empty() {
        return Level {
            rms: 0.0,
            peak: 0.0,
        };
    }

    let mut sum_sq = 0.0_f32;
    let mut peak = 0.0_f32;

    for &sample in samples {
        sum_sq += sample * sample;
        peak = peak.max(sample.abs());
    }

    Level {
        rms: (sum_sq / samples.len() as f32).sqrt(),
        peak,
    }
}

#[derive(Debug, Clone)]
pub struct AdaptiveGain {
    target_rms: f32,
    min_gain: f32,
    max_gain: f32,
    silence_rms: f32,
    gain: f32,
    attack: f32,
    release: f32,
}

impl Default for AdaptiveGain {
    fn default() -> Self {
        Self {
            target_rms: 0.125,
            min_gain: 0.25,
            max_gain: 4.0,
            silence_rms: 0.003,
            gain: 1.0,
            attack: 0.35,
            release: 0.08,
        }
    }
}

impl AdaptiveGain {
    pub fn process(&mut self, samples: &mut [f32]) {
        let level = measure(samples);

        if level.rms < self.silence_rms {
            return;
        }

        let desired = (self.target_rms / level.rms).clamp(self.min_gain, self.max_gain);
        let smoothing = if desired < self.gain {
            self.attack
        } else {
            self.release
        };

        self.gain += (desired - self.gain) * smoothing;

        for sample in samples {
            *sample *= self.gain;
        }
    }

    #[must_use]
    pub const fn current_gain(&self) -> f32 {
        self.gain
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SoftLimiter {
    drive: f32,
}

impl Default for SoftLimiter {
    fn default() -> Self {
        Self { drive: 1.5 }
    }
}

impl SoftLimiter {
    pub fn process(&self, samples: &mut [f32]) {
        let normalizer = self.drive.tanh();

        for sample in samples {
            *sample = (*sample * self.drive).tanh() / normalizer;
            *sample = sample.clamp(-1.0, 1.0);
        }
    }
}

pub const DECIMATOR2_INPUT_RATE_HZ: u32 = 16_000;
pub const DECIMATOR2_OUTPUT_RATE_HZ: u32 = 8_000;
pub const DECIMATOR2_TAPS: usize = 129;
pub const DECIMATOR2_GROUP_DELAY_INPUT_SAMPLES: usize = (DECIMATOR2_TAPS - 1) / 2;
pub const DECIMATOR2_GROUP_DELAY_OUTPUT_SAMPLES: usize = DECIMATOR2_GROUP_DELAY_INPUT_SAMPLES / 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResampleError {
    OutputTooSmall,
}

/// Stateful 2:1 speech-band decimator.
///
/// The filter is independently generated from a windowed-sinc low-pass design:
/// 3.7 kHz cutoff at 16 kHz input, 129 taps, Blackman window. Convolution is
/// only evaluated for output samples, so the steady-state cost is one
/// 129-tap dot product per 8 kHz output sample.
#[derive(Debug, Clone)]
pub struct Decimator2 {
    taps: [f32; DECIMATOR2_TAPS],
    delay: [f32; DECIMATOR2_TAPS],
    write_pos: usize,
    emit_next: bool,
}

impl Default for Decimator2 {
    fn default() -> Self {
        Self {
            taps: design_decimator2_taps(),
            delay: [0.0; DECIMATOR2_TAPS],
            write_pos: 0,
            emit_next: true,
        }
    }
}

impl Decimator2 {
    #[must_use]
    pub fn required_output_len(&self, input_len: usize) -> usize {
        let phase_extra = if self.emit_next { 1 } else { 0 };
        (input_len + phase_extra) / 2
    }

    pub fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<usize, ResampleError> {
        let required = self.required_output_len(input.len());
        if output.len() < required {
            return Err(ResampleError::OutputTooSmall);
        }

        let mut produced = 0_usize;
        for &sample in input {
            self.delay[self.write_pos] = sample;
            self.write_pos = (self.write_pos + 1) % DECIMATOR2_TAPS;

            if self.emit_next {
                let mut filtered = 0.0_f32;
                for (tap_index, coefficient) in self.taps.iter().enumerate() {
                    let delay_index =
                        (self.write_pos + DECIMATOR2_TAPS - 1 - tap_index) % DECIMATOR2_TAPS;
                    filtered += coefficient * self.delay[delay_index];
                }
                output[produced] = filtered;
                produced += 1;
            }

            self.emit_next = !self.emit_next;
        }

        Ok(produced)
    }

    pub fn reset(&mut self) {
        self.delay.fill(0.0);
        self.write_pos = 0;
        self.emit_next = true;
    }
}

fn design_decimator2_taps() -> [f32; DECIMATOR2_TAPS] {
    const CUTOFF_HZ: f32 = 3_700.0;
    let normalized_cutoff = CUTOFF_HZ / DECIMATOR2_INPUT_RATE_HZ as f32;
    let midpoint = (DECIMATOR2_TAPS - 1) as f32 / 2.0;
    let denominator = (DECIMATOR2_TAPS - 1) as f32;
    let mut taps = [0.0_f32; DECIMATOR2_TAPS];
    let mut sum = 0.0_f32;

    for (index, tap) in taps.iter_mut().enumerate() {
        let offset = index as f32 - midpoint;
        let ideal = if offset.abs() < f32::EPSILON {
            2.0 * normalized_cutoff
        } else {
            (2.0 * core::f32::consts::PI * normalized_cutoff * offset).sin()
                / (core::f32::consts::PI * offset)
        };
        let phase = 2.0 * core::f32::consts::PI * index as f32 / denominator;
        let window = 0.42 - 0.5 * phase.cos() + 0.08 * (2.0 * phase).cos();
        *tap = ideal * window;
        sum += *tap;
    }

    for tap in &mut taps {
        *tap /= sum;
    }

    taps
}

#[derive(Default)]
pub struct DspChain {
    agc: AdaptiveGain,
    limiter: SoftLimiter,
}

impl DspChain {
    pub fn process(&mut self, frame: &mut PcmFrame) -> Level {
        self.agc.process(frame.samples_mut());
        self.limiter.process(frame.samples_mut());
        measure(frame.samples())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limiter_never_exceeds_full_scale() {
        let mut samples = [3.0_f32, -3.0, 0.5];
        SoftLimiter::default().process(&mut samples);
        assert!(samples.iter().all(|sample| sample.abs() <= 1.0));
    }

    #[test]
    fn agc_does_not_raise_digital_silence() {
        let mut samples = [0.0_f32; 160];
        let mut agc = AdaptiveGain::default();
        agc.process(&mut samples);
        assert_eq!(agc.current_gain(), 1.0);
        assert!(samples.iter().all(|sample| *sample == 0.0));
    }

    #[test]
    fn decimator2_preserves_speech_band() {
        const INPUT_SAMPLES: usize = 4_096;
        let mut input = [0.0_f32; INPUT_SAMPLES];
        for (index, sample) in input.iter_mut().enumerate() {
            let phase = 2.0 * core::f32::consts::PI * 1_000.0 * index as f32
                / DECIMATOR2_INPUT_RATE_HZ as f32;
            *sample = 0.5 * phase.sin();
        }

        let mut output = [0.0_f32; INPUT_SAMPLES / 2];
        let mut decimator = Decimator2::default();
        let produced = decimator.process(&input, &mut output).expect("decimate");

        let input_level = measure(&input[256..]);
        let output_level = measure(&output[128..produced]);
        let ratio = output_level.rms / input_level.rms;
        assert!((0.95..=1.05).contains(&ratio), "ratio={ratio}");
    }

    #[test]
    fn decimator2_suppresses_out_of_band_alias() {
        const INPUT_SAMPLES: usize = 4_096;
        let mut input = [0.0_f32; INPUT_SAMPLES];
        for (index, sample) in input.iter_mut().enumerate() {
            let phase = 2.0 * core::f32::consts::PI * 6_000.0 * index as f32
                / DECIMATOR2_INPUT_RATE_HZ as f32;
            *sample = 0.5 * phase.sin();
        }

        let mut output = [0.0_f32; INPUT_SAMPLES / 2];
        let mut decimator = Decimator2::default();
        let produced = decimator.process(&input, &mut output).expect("decimate");
        let output_level = measure(&output[128..produced]);
        assert!(output_level.rms < 0.005, "rms={}", output_level.rms);
    }

    #[test]
    fn decimator2_is_streaming_and_bounded() {
        let input_a = [0.1_f32; 161];
        let input_b = [0.1_f32; 159];
        let mut decimator = Decimator2::default();

        let mut output_a = [0.0_f32; 81];
        let required_a = decimator.required_output_len(input_a.len());
        assert_eq!(required_a, 81);
        let produced_a = decimator.process(&input_a, &mut output_a).expect("a");
        assert_eq!(produced_a, required_a);

        let mut output_b = [0.0_f32; 80];
        let required_b = decimator.required_output_len(input_b.len());
        assert_eq!(required_b, 79);
        let produced_b = decimator.process(&input_b, &mut output_b).expect("b");
        assert_eq!(produced_b, required_b);
        assert!(
            output_a[..produced_a]
                .iter()
                .all(|sample| sample.is_finite())
        );
        assert!(
            output_b[..produced_b]
                .iter()
                .all(|sample| sample.is_finite())
        );
    }

    #[test]
    fn chain_processes_fixed_frame() {
        let input = [0.02_f32; 160];
        let mut frame = PcmFrame::from_slice(8_000, 0, &input).expect("frame");
        let mut chain = DspChain::default();
        let level = chain.process(&mut frame);
        assert!(level.rms > 0.0);
        assert!(level.peak <= 1.0);
    }
}
