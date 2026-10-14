use crate::spectrum::SpectrumProcessor;
use crate::window::{apply_window, hann_window};

pub struct AudioPipeline {
    window: Vec<f32>,
    windowed: Vec<f32>,
    spectrum: SpectrumProcessor,
    power: Vec<f32>,
}

impl AudioPipeline {
    pub fn new(frame_size: usize) -> Self {
        let window = hann_window(frame_size);

        Self {
            window,
            windowed: vec![0.0; frame_size],
            spectrum: SpectrumProcessor::new(frame_size),
            power: vec![0.0; frame_size / 2 + 1],
        }
    }

    pub fn process_frame(&mut self, frame: &[f32]) -> &[f32] {
        assert_eq!(frame.len(), self.window.len());

        apply_window(frame, &self.window, &mut self.windowed);

        let bins = self.spectrum.process(&self.windowed);

        crate::features::power_spectrum(bins, &mut self.power);

        &self.power
    }
}