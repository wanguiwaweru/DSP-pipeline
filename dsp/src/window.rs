use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DspError {
    LenMismatch,
    Fft,
}

/// Pre-computes periodic Hann window coefficients during setup.
pub fn hann(n: usize) -> Vec<f32> {
    (0..n)
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / n as f32).cos()))
        .collect()
}

/// Applies pre-computed window coefficients to an input frame.
/// Hot Path: Zero dynamic heap allocations.
pub fn apply_window(input: &[f32], window: &[f32], out: &mut [f32]) -> Result<(), DspError> {
    if input.len() != window.len() || out.len() != input.len() {
        return Err(DspError::LenMismatch);
    }

    for ((o, x), w) in out.iter_mut().zip(input).zip(window) {
        *o = x * w;
    }

    Ok(())
}
