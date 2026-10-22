//! Canonical domain contracts shared across bounded contexts.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub id: String,
    pub service: String,
    pub partition: String,
    pub timestamp_ms: i64,
    pub corrected_ms: i64,
    pub kind: String,
    pub severity: f64,
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Corpus {
    pub incident_id: String,
    pub events: Vec<Event>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Timeline {
    pub ordered: Vec<Event>,
    pub clock_offsets_ms: BTreeMap<String, i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ServiceEdge {
    pub caller: String,
    pub callee: String,
    pub weight: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Hypothesis {
    pub id: String,
    pub cause_service: String,
    pub effect_service: String,
    pub score: f64,
    pub e_value: f64,
    pub stable_environments: usize,
    pub rationale: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Analogue {
    pub incident_id: String,
    pub distance: f32,
    pub outcome: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Proposal {
    pub action: String,
    pub target_service: String,
    pub estimated_blast_radius: u32,
    pub reversible: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub reasons: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EvidenceRecord {
    pub sequence: u64,
    pub kind: String,
    pub payload: String,
    pub previous_hash: String,
    pub hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InvestigationReport {
    pub incident_id: String,
    pub event_count: usize,
    pub top_hypotheses: Vec<Hypothesis>,
    pub analogues: Vec<Analogue>,
    pub proposal: Proposal,
    pub decision: PolicyDecision,
    pub evidence_root: String,
    pub rvf_path: String,
}
