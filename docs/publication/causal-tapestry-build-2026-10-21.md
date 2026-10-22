# Dream Machine Build — 2026-10-21 — Causal Tapestry

Repository: https://github.com/nicholas-ruest/causal-tapestry
Final main commit: eb01096ae941ce49c494dfcb927ff3101853fc0c
Research record: https://gist.github.com/nicholas-ruest/4514ae68f52118374705e9919aa93b76

## Delivered capability

Causal Tapestry is a 22-crate Rust workspace that ingests clock-skewed distributed telemetry, reconstructs a validated partial order and service topology, ranks causes for stability across environments, accumulates anytime-valid evidence, recalls analogous incidents through RuVector, proposes a reversible traffic isolation action, evaluates it with Microsoft Regorus, and seals/reopens/queries the investigation through RVF. It never executes production remediation.

## Architecture and documentation

- 22 implemented crates; all 22 appear on the ct-cli executable dependency graph.
- 25 substantive ADRs plus index.
- 12 substantive DDD records plus index.
- Specification, architecture, research, integration matrix, implementation contract, traceability, core-memory guidance map, validation evidence, and benchmark evidence.
- Two project-specific animated SVGs and licensed local technology marks.

## Exact ingredients

- RuVector core 2.4.0, source 46d816ddc4c2a8a06cd3613534a79b1d557d858b.
- RVF runtime 0.3.2, same source commit.
- Microsoft Regorus 0.13.0, source 9d19b6a578d22d6a223c48ba37e632d7d74ee0db.
- Invariant causal prediction plus an anytime-valid e-process.
- Ruflo 3.47.1 for governed coordination; MetaHarness 0.4.4 score, Darwin 0.10.3 suite verification, and Flywheel genesis-ledger readback as external build receipts.

## Reproducible validation

- cargo fmt --all -- --check: exit 0.
- cargo clippy --locked --workspace --all-targets --all-features -- -D warnings: exit 0.
- cargo test --locked --workspace: 24 tests, 0 failed.
- cargo deny check: advisories, bans, licenses, and sources OK.
- Pinned checker 80f006f6960bc4e491e97f51e890ff708e81aa88: exit 0 with 25 ADRs, 12 DDDs, 22 crates, and 2 animated SVGs.
- PR #1 CI: success; final main CI: success (GitHub Actions run 39711458483).

## Benchmark and ablations

600 events, 25 complete investigations per mode on one RuOS machine:

| Mode | Mean microseconds | Analogues | Allowed | Top e-value | Lost capability |
|---|---:|---:|---|---:|---|
| combined | 4387 | 2 | yes | 2.067250 | none |
| no RuVector | 4287 | 0 | yes | 2.067250 | historical recall |
| no Regorus | 4008 | 2 | yes | 2.067250 | enforceable policy; labelled unsafe baseline |
| no e-process | 4389 | 2 | no | 1.000000 | anytime-valid evidence/authorization |
| no RVF | 2581 | 2 | yes | 2.067250 | durable bundle and readback |

Times are comparative, not universal performance claims. Fixture SHA-256: 4a13cb2aea6a0b285f8f67587b45f2483edc4ca3efd14e1f378015da94a249aa.

## Limitations

This is research-grade decision support, not an autonomous remediator. The deterministic eight-dimensional embedding is inspectable but not semantic. Sparse environment variation can produce weak evidence. Regorus is a policy boundary, not IAM. RVF evidence is local and not replicated by this project.