# Implementation contract

## Stage gates

1. This design commit is immutable input to implementation.
2. An upstream API spike must compile and execute RuVector, RVF, and Regorus before their adapters count.
3. Each crate needs nontrivial source, public typed behavior, a documented caller, and a passing test.
4. The frozen end-to-end workload must cross every bounded context.
5. Format, strict Clippy, locked tests, dependency/license audit, failure tests, benchmark, baselines, and ablations produce raw logs.
6. The pinned checker must exit 0 on an indexed git checkout.
7. Semantic review inspects call paths, upstream symbols, receipts, and README claims before publication.

## Acceptance commands

cargo fmt --check  
cargo clippy --workspace --all-targets --all-features -- -D warnings  
cargo test --workspace --locked  
cargo deny check  
cargo run -p ct-cli -- investigate examples/incident.json --policy policies/remediation.rego --out evidence  
cargo run -p ct-cli -- replay evidence/investigation.rvf  
cargo run -p ct-cli -- benchmark --seed 20261021

## Evidence binding

Every raw receipt records UTC time, command, exit code, tool versions, git source identity, dataset hash, seed, feature flags, and stdout/stderr hash. No fabricated metrics.

