
/// Iterate over fixed-size, overlapping frames in an audio sample buffer.
///
/// `frame_size` is the number of samples per frame.
/// `hop_size` is the number of samples to advance between frames.
///
/// Incomplete trailing frames are discarded.
pub fn frames(
    samples: &[f32],
    frame_size: usize,
    hop_size: usize,
) -> impl Iterator<Item = &[f32]> {
    assert!(frame_size > 0, "frame_size must be non-zero");
    assert!(hop_size > 0, "hop_size must be non-zero");

    samples
        .windows(frame_size)
        .step_by(hop_size)
}
