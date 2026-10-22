//! Investigation saga: the executable system-level vertical slice.
use ct_causal::rank;
use ct_config::Config;
use ct_embedding::embed;
use ct_errors::{Error, Result};
use ct_evidence::Ledger;
use ct_hypothesis::generate;
use ct_ingest::ingest_json_lines;
use ct_normalize::normalize;
use ct_policy::authorize;
use ct_recall::recall;
use ct_remediation::propose;
use ct_ruvector::AnalogueIndex;
use ct_timeline::causal_order;
use ct_topology::extract;
use ct_types::InvestigationReport;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Combined,
    NoRuVector,
    NoRegorus,
    NoEProcess,
    NoRvf,
}

pub fn investigate(
    incident_id: &str,
    input: &str,
    output: &Path,
    config: &Config,
    mode: Mode,
) -> Result<InvestigationReport> {
    config.validate()?;
    let mut ledger = Ledger::default();
    let corpus = ingest_json_lines(incident_id, input, config.max_events)?;
    ledger.append("ingest", &format!("{} events", corpus.events.len()))?;
    let timeline = normalize(&corpus)?;
    let order = causal_order(&timeline)?;
    ledger.append(
        "timeline",
        &serde_json::to_string(&order).map_err(|e| Error::Evidence(e.to_string()))?,
    )?;
    let edges = extract(&timeline);
    let candidates = generate(&timeline, &edges, config.max_hypotheses);
    let mut hypotheses = rank(candidates, &timeline)?;
    if mode == Mode::NoEProcess {
        for hypothesis in &mut hypotheses {
            hypothesis.e_value = 1.0;
        }
    }
    let top = hypotheses
        .first()
        .cloned()
        .ok_or_else(|| Error::InvalidInput("no hypotheses generated".into()))?;
    ledger.append(
        "causal_ranking",
        &serde_json::to_string(&hypotheses).map_err(|e| Error::Evidence(e.to_string()))?,
    )?;
    let vector = embed(&timeline, &hypotheses);
    let analogues = if mode == Mode::NoRuVector {
        Vec::new()
    } else {
        let mut index = AnalogueIndex::new()?;
        index.insert(
            "prior-cache-saturation",
            vec![0.78, 0.55, 0.20, 0.18, 0.16, 0.32, 0.12, 0.70],
            "traffic shifted; cache rebuilt",
        )?;
        index.insert(
            "prior-db-lock",
            vec![0.64, 0.42, 0.10, 0.12, 0.22, 0.45, 0.20, 0.62],
            "write pool isolated; lock owner terminated",
        )?;
        recall(&index, &vector, 2)?
    };
    ledger.append(
        "analogue_recall",
        &serde_json::to_string(&analogues).map_err(|e| Error::Evidence(e.to_string()))?,
    )?;
    let proposal = propose(&top);
    let decision = if mode == Mode::NoRegorus {
        ct_types::PolicyDecision {
            allowed: true,
            reasons: vec!["Regorus ablated: unsafe allow baseline".into()],
        }
    } else {
        authorize(
            &config.policy_text,
            &proposal,
            top.e_value,
            config.min_e_value,
        )?
    };
    ledger.append(
        "policy_decision",
        &serde_json::to_string(&decision).map_err(|e| Error::Evidence(e.to_string()))?,
    )?;
    let evidence_root = if mode == Mode::NoRvf {
        ledger
            .records()
            .last()
            .map(|record| record.hash.clone())
            .unwrap_or_default()
    } else {
        ledger.seal_rvf(output)?
    };
    Ok(InvestigationReport {
        incident_id: incident_id.into(),
        event_count: timeline.ordered.len(),
        top_hypotheses: hypotheses.into_iter().take(5).collect(),
        analogues,
        proposal,
        decision,
        evidence_root,
        rvf_path: output.display().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_input_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        assert!(investigate(
            "x",
            "",
            dir.path().join("x.rvf").as_path(),
            &Config::default(),
            Mode::Combined
        )
        .is_err());
    }
}
