#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_length_mismatch() {
        let input = vec![1.0; 1024];
        let window = vec![0.5; 512];
        let mut out = vec![0.0; 1024];

        assert_eq!(
            apply_window(&input, &window, &mut out),
            Err(DspError::LenMismatch)
        );
    }
}
