pub mod features;
pub mod framing;
pub mod pipeline;
pub mod spectrum;
pub mod window;

pub use features::FeatureExtractor;
pub use framing::FrameIterator;
pub use pipeline::FramePipeline;
pub use spectrum::SpectrumAnalyzer;
pub use window::{apply_window, hann, DspError};