use realfft::{RealFftPlanner, RealToComplex};
use std::sync::Arc;

pub struct SpectrumProcessor {
    fft: Arc<dyn RealToComplex<f32>>,
    output: Vec<num_complex::Complex32>,
    scratch: Vec<num_complex::Complex32>,
}

impl SpectrumProcessor {
    pub fn new(frame_size: usize) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(frame_size);

        let output = fft.make_output_vec();
        let scratch = fft.make_scratch_vec();

        Self {
            fft,
            output,
            scratch,
        }
    }

    pub fn process(&mut self, input: &[f32]) -> &[num_complex::Complex32] {
        assert_eq!(input.len(), self.fft.len());

        let mut input_buffer = input.to_vec();
        self.fft
            .process_with_scratch(&mut input_buffer, &mut self.output, &mut self.scratch)
            .expect("FFT processing failed");

        &self.output
    }
}