use ct_config::Config;
use ct_orchestrator::{investigate, Mode};
use std::time::Instant;

const INCIDENT: &str = include_str!("../../../examples/cache-collapse.jsonl");

fn expanded_incident() -> String {
    let originals: Vec<serde_json::Value> = INCIDENT
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).expect("valid benchmark fixture"))
        .collect();
    let mut lines = Vec::with_capacity(originals.len() * 50);
    for repetition in 0..50_u64 {
        for original in &originals {
            let mut event = original.clone();
            let id = event["id"].as_str().expect("fixture id").to_owned();
            event["id"] = format!("{id}-{repetition}").into();
            if let Some(parent) = event["parent_id"].as_str() {
                event["parent_id"] = format!("{parent}-{repetition}").into();
            }
            let timestamp = event["timestamp_ms"].as_u64().expect("fixture timestamp");
            event["timestamp_ms"] = (timestamp + repetition * 10_000).into();
            lines.push(serde_json::to_string(&event).expect("serialize benchmark event"));
        }
    }
    lines.join("\n")
}

fn main() {
    let incident = expanded_incident();
    println!("mode,events,iterations,total_us,mean_us,analogues,allowed,top_e_value");
    for (name, mode) in [
        ("combined", Mode::Combined),
        ("no_ruvector", Mode::NoRuVector),
        ("no_regorus", Mode::NoRegorus),
        ("no_eprocess", Mode::NoEProcess),
        ("no_rvf", Mode::NoRvf),
    ] {
        let dir = tempfile::tempdir().expect("benchmark tempdir");
        let start = Instant::now();
        let mut last = None;
        for iteration in 0..25 {
            let path = dir.path().join(format!("{name}-{iteration}.rvf"));
            last = Some(
                investigate(
                    &format!("bench-{iteration}"),
                    &incident,
                    &path,
                    &Config::default(),
                    mode,
                )
                .expect("benchmark investigation"),
            );
        }
        let elapsed = start.elapsed().as_micros();
        let report = last.expect("at least one iteration");
        println!(
            "{name},600,25,{elapsed},{},{},{},{:.6}",
            elapsed / 25,
            report.analogues.len(),
            report.decision.allowed,
            report.top_hypotheses[0].e_value
        );
    }
}
