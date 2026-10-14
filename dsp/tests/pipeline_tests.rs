use dsp::pipeline::AudioPipeline;

#[test]
fn silence_produces_zero_power() {
    let frame_size = 1024;
    let mut pipeline = AudioPipeline::new(frame_size);
    let frame = vec![0.0_f32; frame_size];

    let power = pipeline.process_frame(&frame);

    assert_eq!(power.len(), frame_size / 2 + 1);

    for &value in power {
        assert!(
            value.abs() < 1e-6,
            "expected zero power, got {value}"
        );
    }
}

#[test]
fn sine_wave_produces_finite_nonnegative_power() {
    let frame_size = 1024;
    let sample_rate = 16_000.0_f32;
    let frequency = 1000.0_f32;

    let frame: Vec<f32> = (0..frame_size)
        .map(|i| {
            let phase =
                2.0 * std::f32::consts::PI * frequency * i as f32
                    / sample_rate;
            phase.sin()
        })
        .collect();

    let mut pipeline = AudioPipeline::new(frame_size);
    let power = pipeline.process_frame(&frame);

    assert_eq!(power.len(), frame_size / 2 + 1);

    assert!(
        power.iter().all(|p| p.is_finite() && *p >= 0.0),
        "power values must be finite and nonnegative"
    );

    assert!(
        power.iter().any(|&p| p > 1e-6),
        "a nonzero input should produce nonzero power"
    );
}

#[test]
fn can_process_multiple_frames_sequentially() {
    let frame_size = 1024;
    let mut pipeline = AudioPipeline::new(frame_size);

    let silence = vec![0.0_f32; frame_size];
    let first = pipeline.process_frame(&silence);
    assert!(first.iter().all(|&p| p.abs() < 1e-6));

    let tone: Vec<f32> = (0..frame_size)
        .map(|i| {
            (2.0 * std::f32::consts::PI * i as f32 / 16.0).sin()
        })
        .collect();

    let second = pipeline.process_frame(&tone);

    assert!(
        second.iter().any(|&p| p > 1e-6),
        "pipeline should process new input after previous input"
    );
}