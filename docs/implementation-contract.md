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

cargo fmt --all -- --check  
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings  
cargo test --locked --workspace  
cargo deny check  
cargo run --locked -p ct-cli -- --incident-id inc-2026-10-21 --input examples/cache-collapse.jsonl --output evidence.rvf  
./scripts/run-benchmark.sh  
node /home/ruv/work/ruvnet-dream-machine-checker/scripts/check-project-contract.mjs /home/ruv/work/causal-tapestry-candidate

## Evidence binding

Every raw receipt records UTC time, command, exit code, tool versions, git source identity, dataset hash, seed, feature flags, and stdout/stderr hash. No fabricated metrics.

