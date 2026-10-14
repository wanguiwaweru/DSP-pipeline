pub fn hann_window(size: usize) -> Vec<f32> {
    if size == 0 {
        return Vec::new();
    }

    let denominator = if size > 1 { size - 1 } else { 1 };
    (0..size)
        .map(|n| {
            let phase = std::f32::consts::PI * 2.0 * (n as f32) / (denominator as f32);
            0.5 * (1.0 - phase.cos())
        })
        .collect()
}

pub fn apply_window(samples: &[f32], window: &[f32], output: &mut [f32]) {
    assert_eq!(samples.len(), window.len());
    assert_eq!(samples.len(), output.len());

    for ((sample, weight), out) in samples.iter().zip(window.iter()).zip(output.iter_mut()) {
        *out = sample * weight;
    }
}

pub fn apply_hann_window(samples: &[f32]) -> Vec<f32> {
    let window = hann_window(samples.len());
    let mut output = vec![0.0; samples.len()];
    apply_window(samples, &window, &mut output);
    output
}
