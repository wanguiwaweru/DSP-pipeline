use num_complex::Complex32;

pub fn power_spectrum(
    bins: &[Complex32],
    output: &mut [f32],
) {
    assert_eq!(bins.len(), output.len());

    for (bin, power) in bins.iter().zip(output.iter_mut()) {
        *power = bin.norm_sqr();
    }
}

pub fn band_energy(
    power: &[f32],
    start_bin: usize,
    end_bin: usize,
) -> f32 {
    power[start_bin..end_bin].iter().sum()
}