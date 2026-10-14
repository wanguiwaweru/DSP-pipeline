use dsp::framing::frames;

#[test]
fn produces_expected_overlapping_frames() {
    let samples: Vec<f32> = (0..2048).map(|x| x as f32).collect();
    let result: Vec<&[f32]> = frames(&samples, 1024, 256).collect();

    assert_eq!(result.len(), 5);
    assert_eq!(result[0][0], 0.0);
    assert_eq!(result[1][0], 256.0);
    assert_eq!(result[4][0], 1024.0);
    assert_eq!(result[0].len(), 1024);
}

#[test]
fn discards_incomplete_trailing_frame() {
    let samples = vec![0.0; 1100];
    let result: Vec<&[f32]> = frames(&samples, 1024, 256).collect();

    assert_eq!(result.len(), 1);
}

#[test]
fn produces_one_frame_when_input_matches_frame_size() {
    let samples: Vec<f32> = (0..4).map(|x| x as f32).collect();
    let result: Vec<&[f32]> = frames(&samples, 4, 2).collect();

    assert_eq!(result, vec![samples.as_slice()]);
}

#[test]
fn produces_no_frames_when_input_is_shorter_than_frame_size() {
    let samples = vec![0.0; 3];

    assert!(frames(&samples, 4, 2).next().is_none());
}

#[test]
fn supports_hops_larger_than_frame_size() {
    let samples: Vec<f32> = (0..12).map(|x| x as f32).collect();
    let result: Vec<&[f32]> = frames(&samples, 3, 5).collect();

    assert_eq!(result, vec![&samples[0..3], &samples[5..8]]);
}

#[test]
#[should_panic(expected = "frame_size must be non-zero")]
fn rejects_zero_frame_size() {
    let _ = frames(&[], 0, 1);
}

#[test]
#[should_panic(expected = "hop_size must be non-zero")]
fn rejects_zero_hop_size() {
    let _ = frames(&[], 1, 0);
}
