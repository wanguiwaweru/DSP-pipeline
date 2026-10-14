use crate::window::DspError;

/// Zero-allocation frame iterator over borrowed sample slices.
pub struct FrameIterator<'a> {
    samples: &'a [f32],
    frame_size: usize,
    hop_size: usize,
    current_pos: usize,
}

impl<'a> FrameIterator<'a> {
    pub fn new(samples: &'a [f32], frame_size: usize, hop_size: usize) -> Self {
        assert!(frame_size > 0, "frame_size must be non-zero");
        assert!(hop_size > 0, "hop_size must be non-zero");

        Self {
            samples,
            frame_size,
            hop_size,
            current_pos: 0,
        }
    }
}

impl<'a> Iterator for FrameIterator<'a> {
    type Item = &'a [f32];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos <= self.samples.len()
            && self.frame_size <= self.samples.len() - self.current_pos
        {
            let frame = &self.samples[self.current_pos..self.current_pos + self.frame_size];
            self.current_pos = self.current_pos.saturating_add(self.hop_size);
            Some(frame)
        } else {
            None
        }
    }
}

/// Iterate over fixed-size, overlapping frames, discarding any incomplete tail.
#[inline]
pub fn frames(samples: &[f32], frame_size: usize, hop_size: usize) -> FrameIterator<'_> {
    FrameIterator::new(samples, frame_size, hop_size)
}

#[inline]
pub fn validate_frame_length(frame_len: usize, expected_len: usize) -> Result<(), DspError> {
    if frame_len != expected_len {
        Err(DspError::LenMismatch)
    } else {
        Ok(())
    }
}
