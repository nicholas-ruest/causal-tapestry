//! Runtime configuration and resource ceilings.
use ct_errors::{Error, Result};

#[derive(Clone, Debug)]
pub struct Config {
    pub max_events: usize,
    pub max_hypotheses: usize,
    pub min_e_value: f64,
    pub vector_dimension: usize,
    pub policy_text: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_events: 10_000,
            max_hypotheses: 32,
            min_e_value: 1.5,
            vector_dimension: 8,
            policy_text: include_str!("../../../policies/remediation.rego").to_owned(),
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        if self.max_events == 0 || self.max_hypotheses == 0 {
            return Err(Error::InvalidInput(
                "resource ceilings must be positive".into(),
            ));
        }
        if self.vector_dimension != 8 {
            return Err(Error::InvalidInput(
                "the v1 incident embedding has dimension 8".into(),
            ));
        }
        if !self.min_e_value.is_finite() || self.min_e_value < 1.0 {
            return Err(Error::InvalidInput(
                "min_e_value must be finite and at least one".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults_validate() {
        Config::default().validate().unwrap();
    }
}
