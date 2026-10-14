use crate::features::FeatureExtractor;
use crate::spectrum::SpectrumAnalyzer;
use crate::window::{DspError, apply_window, hann};


pub struct FramePipeline {
    window: Vec<f32>,   // Hann window coefficients[cite: 1]
    windowed: Vec<f32>, // Reused buffer for windowed audio frame[cite: 1]
    analyzer: SpectrumAnalyzer,
    extractor: FeatureExtractor,
}

impl FramePipeline {
    /// SETUP · Runs once. All memory allocation occurs here[cite: 1].
    pub fn new(n: usize, bands: usize) -> Self {
        let analyzer = SpectrumAnalyzer::new(n);
        let extractor = FeatureExtractor::new(analyzer.num_bins(), bands);

        Self {
            window: hann(n),
            windowed: vec![0.0; n],
            analyzer,
            extractor,
        }
    }

    /// PER FRAME · Zero hot-path dynamic allocations[cite: 1].
    pub fn process(&mut self, frame: &[f32], out: &mut [f32]) -> Result<(), DspError> {
        if frame.len() != self.window.len() {
            return Err(DspError::LenMismatch);
        }

        // 1. Apply Window into reused scratch buffer[cite: 1]
        apply_window(frame, &self.window, &mut self.windowed)?;

        // 2. Compute RealFFT with reused scratch space[cite: 1]
        let bins = self.analyzer.compute(&mut self.windowed)?;

        // 3. Extract power features into caller-owned output buffer[cite: 1]
        self.extractor.extract(bins, out)?;

        Ok(())
    }
}

/// Compatibility wrapper that returns one power value for each FFT bin.
pub struct AudioPipeline {
    pipeline: FramePipeline,
    power: Vec<f32>,
}

impl AudioPipeline {
    pub fn new(frame_size: usize) -> Self {
        let num_bins = frame_size / 2 + 1;
        Self {
            pipeline: FramePipeline::new(frame_size, num_bins),
            power: vec![0.0; num_bins],
        }
    }

    pub fn process_frame(&mut self, frame: &[f32]) -> &[f32] {
        self.pipeline
            .process(frame, &mut self.power)
            .expect("failed to process audio frame");
        &self.power
    }
}
