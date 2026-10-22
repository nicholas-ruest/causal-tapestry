//! Invariant causal ranking across operational environments.
use ct_errors::Result;
use ct_evalue::accumulate;
use ct_types::{Hypothesis, Timeline};
use std::collections::BTreeMap;

pub fn rank(mut candidates: Vec<Hypothesis>, timeline: &Timeline) -> Result<Vec<Hypothesis>> {
    let environments: Vec<String> = timeline
        .ordered
        .iter()
        .map(|e| e.partition.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    for candidate in &mut candidates {
        let mut contrasts = Vec::new();
        let mut stable = 0;
        for environment in &environments {
            let mut cause = Vec::new();
            let mut other = Vec::new();
            for event in timeline
                .ordered
                .iter()
                .filter(|event| &event.partition == environment)
            {
                if event.service == candidate.cause_service {
                    cause.push(event.severity);
                } else {
                    other.push(event.severity);
                }
            }
            if !cause.is_empty() && !other.is_empty() {
                let mean = |values: &[f64]| values.iter().sum::<f64>() / values.len() as f64;
                let contrast = (mean(&cause) - mean(&other)).clamp(-1.0, 1.0);
                if contrast > 0.0 {
                    stable += 1;
                }
                contrasts.push(contrast);
            }
        }
        candidate.e_value = accumulate(&contrasts)?;
        candidate.stable_environments = stable;
        candidate.score *= candidate.e_value * (1.0 + stable as f64);
        candidate.rationale.push_str(&format!(
            "; e-value {:.3} in {stable} stable environments",
            candidate.e_value
        ));
    }
    candidates.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.id.cmp(&b.id)));
    Ok(candidates)
}

pub fn environment_counts(timeline: &Timeline) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for event in &timeline.ordered {
        *counts.entry(event.partition.clone()).or_default() += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn environment_counts_are_explicit() {
        let e = ct_types::Event {
            id: "1".into(),
            service: "s".into(),
            partition: "p".into(),
            timestamp_ms: 0,
            corrected_ms: 0,
            kind: "k".into(),
            severity: 1.0,
            attributes: Default::default(),
        };
        assert_eq!(
            environment_counts(&Timeline {
                ordered: vec![e],
                clock_offsets_ms: Default::default()
            })["p"],
            1
        );
    }
}
