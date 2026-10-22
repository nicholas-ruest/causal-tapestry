use ct_config::Config;
use ct_orchestrator::{investigate, Mode};
use std::time::Instant;

const INCIDENT: &str = include_str!("../../../examples/cache-collapse.jsonl");

fn main() {
    println!("mode,iterations,total_us,mean_us,analogues,allowed,top_e_value");
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
                    INCIDENT,
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
            "{name},25,{elapsed},{},{},{},{:.6}",
            elapsed / 25,
            report.analogues.len(),
            report.decision.allowed,
            report.top_hypotheses[0].e_value
        );
    }
}
