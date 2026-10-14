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