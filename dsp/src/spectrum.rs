use crate::window::DspError;
use num_complex::Complex;
use realfft::{RealFftPlanner, RealToComplex};
use std::sync::Arc;

/// Real-to-Complex Forward FFT wrapper with zero hot-path allocations.
pub struct SpectrumAnalyzer {
    plan: Arc<dyn RealToComplex<f32>>,
    bins: Vec<Complex<f32>>,
    scratch: Vec<Complex<f32>>,
}

impl SpectrumAnalyzer {
    pub fn new(n: usize) -> Self {
        let plan = RealFftPlanner::<f32>::new().plan_fft_forward(n);
        let bins = plan.make_output_vec();
        let scratch = plan.make_scratch_vec();

        Self { plan, bins, scratch }
    }

    /// Computes FFT from a windowed frame directly into complex output bins
    #[inline]
    pub fn compute(&mut self, windowed_frame: &mut [f32]) -> Result<&[Complex<f32>], DspError> {
        self.plan
            .process_with_scratch(windowed_frame, &mut self.bins, &mut self.scratch)
            .map_err(|_| DspError::Fft)?;

        Ok(&self.bins)
    }

    pub fn num_bins(&self) -> usize {
        self.bins.len()
    }
}
