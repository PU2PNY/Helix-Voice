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
        return Level { rms: 0.0, peak: 0.0 };
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
    fn chain_processes_fixed_frame() {
        let input = [0.02_f32; 160];
        let mut frame = PcmFrame::from_slice(8_000, 0, &input).expect("frame");
        let mut chain = DspChain::default();
        let level = chain.process(&mut frame);
        assert!(level.rms > 0.0);
        assert!(level.peak <= 1.0);
    }
}
