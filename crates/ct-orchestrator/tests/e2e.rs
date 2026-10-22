use ct_config::Config;
use ct_orchestrator::{investigate, Mode};

const INCIDENT: &str = include_str!("../../../examples/cache-collapse.jsonl");

#[test]
fn combined_workload_executes_all_boundaries() {
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("evidence.rvf");
    let report = investigate(
        "inc-2026-10-21",
        INCIDENT,
        &output,
        &Config::default(),
        Mode::Combined,
    )
    .unwrap();
    assert_eq!(report.event_count, 12);
    assert!(!report.top_hypotheses.is_empty());
    assert_eq!(report.analogues.len(), 2);
    assert!(output.exists());
    assert_eq!(report.evidence_root.len(), 64);
}

#[test]
fn ablations_change_observable_capability() {
    let dir = tempfile::tempdir().unwrap();
    let config = Config::default();
    let combined = investigate(
        "combined",
        INCIDENT,
        &dir.path().join("combined.rvf"),
        &config,
        Mode::Combined,
    )
    .unwrap();
    let no_vector = investigate(
        "no-vector",
        INCIDENT,
        &dir.path().join("novector.rvf"),
        &config,
        Mode::NoRuVector,
    )
    .unwrap();
    let no_policy = investigate(
        "no-policy",
        INCIDENT,
        &dir.path().join("nopolicy.rvf"),
        &config,
        Mode::NoRegorus,
    )
    .unwrap();
    let no_eprocess = investigate(
        "no-eprocess",
        INCIDENT,
        &dir.path().join("noeprocess.rvf"),
        &config,
        Mode::NoEProcess,
    )
    .unwrap();
    let no_rvf_path = dir.path().join("norvf.rvf");
    let no_rvf = investigate("no-rvf", INCIDENT, &no_rvf_path, &config, Mode::NoRvf).unwrap();
    assert!(!combined.analogues.is_empty());
    assert!(no_vector.analogues.is_empty());
    assert_eq!(
        no_policy.decision.reasons[0],
        "Regorus ablated: unsafe allow baseline"
    );
    assert!(!no_eprocess.decision.allowed);
    assert_eq!(no_eprocess.top_hypotheses[0].e_value, 1.0);
    assert_eq!(no_rvf.event_count, combined.event_count);
    assert!(!no_rvf_path.exists());
}

#[test]
fn malformed_telemetry_fails_before_evidence_write() {
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("bad.rvf");
    assert!(investigate(
        "bad",
        r#"{"id":"a"}"#,
        &output,
        &Config::default(),
        Mode::Combined
    )
    .is_err());
    assert!(!output.exists());
}
