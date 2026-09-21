#![forbid(unsafe_code)]

/// Maximum number of mono samples carried by one internal frame.
///
/// 960 samples covers 20 ms at 48 kHz.
pub const MAX_FRAME_SAMPLES: usize = 960;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameError {
    Empty,
    TooLarge,
    InvalidSampleRate,
}

/// Fixed-capacity PCM frame for the real-time path.
///
/// The frame stores normalized mono f32 PCM without heap allocation.
#[derive(Clone)]
pub struct PcmFrame {
    samples: [f32; MAX_FRAME_SAMPLES],
    len: usize,
    sample_rate_hz: u32,
    timestamp_samples: u64,
}

impl PcmFrame {
    pub fn from_slice(
        sample_rate_hz: u32,
        timestamp_samples: u64,
        input: &[f32],
    ) -> Result<Self, FrameError> {
        if input.is_empty() {
            return Err(FrameError::Empty);
        }
        if input.len() > MAX_FRAME_SAMPLES {
            return Err(FrameError::TooLarge);
        }
        if !(8_000..=48_000).contains(&sample_rate_hz) {
            return Err(FrameError::InvalidSampleRate);
        }

        let mut samples = [0.0; MAX_FRAME_SAMPLES];
        samples[..input.len()].copy_from_slice(input);

        Ok(Self {
            samples,
            len: input.len(),
            sample_rate_hz,
            timestamp_samples,
        })
    }

    #[must_use]
    pub const fn sample_rate_hz(&self) -> u32 {
        self.sample_rate_hz
    }

    #[must_use]
    pub const fn timestamp_samples(&self) -> u64 {
        self.timestamp_samples
    }

    #[must_use]
    pub fn samples(&self) -> &[f32] {
        &self.samples[..self.len]
    }

    pub fn samples_mut(&mut self) -> &mut [f32] {
        &mut self.samples[..self.len]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecKind {
    Open,
    ExternalLicensedBackend,
    Unsupported,
}

pub trait CodecAdapter {
    fn name(&self) -> &'static str;
    fn kind(&self) -> CodecKind;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_rejects_invalid_shapes() {
        assert!(matches!(
            PcmFrame::from_slice(8_000, 0, &[]),
            Err(FrameError::Empty)
        ));

        let too_large = [0.0_f32; MAX_FRAME_SAMPLES + 1];
        assert!(matches!(
            PcmFrame::from_slice(8_000, 0, &too_large),
            Err(FrameError::TooLarge)
        ));

        let one_sample = [0.0_f32; 1];
        assert!(matches!(
            PcmFrame::from_slice(7_999, 0, &one_sample),
            Err(FrameError::InvalidSampleRate)
        ));
        assert!(matches!(
            PcmFrame::from_slice(48_001, 0, &one_sample),
            Err(FrameError::InvalidSampleRate)
        ));
    }

    #[test]
    fn frame_is_fixed_capacity_and_preserves_samples() {
        let src = [0.25_f32, -0.25, 0.5];
        let frame = PcmFrame::from_slice(8_000, 123, &src).expect("valid frame");
        assert_eq!(frame.samples(), src);
        assert_eq!(frame.sample_rate_hz(), 8_000);
        assert_eq!(frame.timestamp_samples(), 123);
    }
}
