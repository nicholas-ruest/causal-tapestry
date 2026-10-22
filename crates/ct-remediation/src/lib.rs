//! Reversible remediation proposal generation; never executes changes.
use ct_types::{Hypothesis, Proposal};

pub fn propose(hypothesis: &Hypothesis) -> Proposal {
    Proposal {
        action: "isolate_and_shift_traffic".into(),
        target_service: hypothesis.cause_service.clone(),
        estimated_blast_radius: if hypothesis.stable_environments >= 2 {
            1
        } else {
            3
        },
        reversible: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generated_action_is_reversible() {
        let h = Hypothesis {
            id: "h".into(),
            cause_service: "db".into(),
            effect_service: "api".into(),
            score: 1.0,
            e_value: 2.0,
            stable_environments: 2,
            rationale: String::new(),
        };
        assert!(propose(&h).reversible);
    }
}
