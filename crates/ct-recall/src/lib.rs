//! Analogue recall and prior-outcome interpretation.
use ct_errors::Result;
use ct_ruvector::AnalogueIndex;
use ct_types::Analogue;

pub fn recall(index: &AnalogueIndex, embedding: &[f32], limit: usize) -> Result<Vec<Analogue>> {
    index.search(embedding, limit)
}

pub fn prior_outcome_signal(analogues: &[Analogue]) -> f64 {
    if analogues.is_empty() {
        return 0.0;
    }
    analogues
        .iter()
        .map(|a| 1.0 / (1.0 + f64::from(a.distance)))
        .sum::<f64>()
        / analogues.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_history_has_zero_signal() {
        assert_eq!(prior_outcome_signal(&[]), 0.0);
    }
}
