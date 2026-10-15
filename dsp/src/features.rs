use crate::window::DspError;
use num_complex::Complex;

/// Computes feature band edges and groups complex power spectrum into band energies
pub struct FeatureExtractor {
    edges: Vec<usize>,
}

impl FeatureExtractor {
    pub fn new(num_bins: usize, num_bands: usize) -> Self {
        assert!(num_bands > 0, "num_bands must be non-zero");

        let edges = (0..=num_bands).map(|b| b * num_bins / num_bands).collect();

        Self { edges }
    }

    /// Sums squared magnitude (Re² + Im²) across frequency bands into caller-owned buffer
    #[inline]
    pub fn extract(&self, bins: &[Complex<f32>], out: &mut [f32]) -> Result<(), DspError> {
        if out.len() != self.edges.len() - 1 || bins.len() != *self.edges.last().unwrap() {
            return Err(DspError::LenMismatch);
        }

        for (o, e) in out.iter_mut().zip(self.edges.windows(2)) {
            *o = bins[e[0]..e[1]]
                .iter()
                .map(|c| c.norm_sqr()) // Power calculation: Re² + Im²
                .sum();
        }

        Ok(())
    }

    pub fn edges(&self) -> &[usize] {
        &self.edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extraction_includes_remainder_bins() {
        let extractor = FeatureExtractor::new(5, 2);
        let bins = [
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];
        let mut out = [0.0; 2];

        extractor.extract(&bins, &mut out).unwrap();

        assert_eq!(extractor.edges(), &[0, 2, 5]);
        assert_eq!(out, [1.0, 29.0]);
    }
}
