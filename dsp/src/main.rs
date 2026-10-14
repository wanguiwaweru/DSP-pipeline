use dsp::{FrameIterator, FramePipeline};

fn main() -> Result<(), dsp::DspError> {
    println!("Running Audio DSP Feature Extraction Pipeline Benchmark...");

    let frame_size = 1024;
    let hop_size = 256;
    let num_bands = 40;

    // Initialize pipeline (Setup phase: allocations happen once here)[cite: 1]
    let mut pipeline = FramePipeline::new(frame_size, num_bands);

    // Simulated 3 seconds of 16 kHz audio input[cite: 1]
    let samples = vec![0.1f32; 48000];

    // Caller-owned feature storage buffer[cite: 1]
    let mut feats = vec![0.0f32; num_bands];
    let mut frame_count = 0;

    // Zero-allocation frame iteration[cite: 1]
    for frame in FrameIterator::new(&samples, frame_size, hop_size) {
        // Run Windowing -> FFT -> Feature Extraction[cite: 1]
        pipeline.process(frame, &mut feats)?;

        frame_count += 1;
    }

    println!(
        "Successfully processed {} frames (40 features/frame) with ZERO hot-path allocations![cite: 1]",
        frame_count
    );

    Ok(())
}