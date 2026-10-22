//! Anytime-valid evidence accumulation using nonnegative e-processes.
use ct_errors::{Error, Result};

pub fn accumulate(contrasts: &[f64]) -> Result<f64> {
    let mut wealth = 1.0;
    for contrast in contrasts {
        if !contrast.is_finite() || !(-1.0..=1.0).contains(contrast) {
            return Err(Error::InvalidInput(
                "contrast must be finite and within [-1,1]".into(),
            ));
        }
        wealth *= 1.0 + 0.5 * contrast;
    }
    Ok(wealth.max(0.0))
}

pub fn evidence_passes(e_value: f64, threshold: f64) -> bool {
    e_value >= threshold
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn supportive_sequence_increases_wealth() {
        assert!(accumulate(&[0.4, 0.5]).unwrap() > 1.0);
    }
    #[test]
    fn invalid_contrast_fails() {
        assert!(accumulate(&[2.0]).is_err());
    }
}
