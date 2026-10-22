# Design-to-implementation traceability

| Requirement | Design | Planned implementation | Evidence |
|---|---|---|---|
| deterministic ingest | ADR-0002, DDD-01 | ct-ingest, ct-otel | invalid and golden fixtures |
| reproducible timeline | ADR-0004, DDD-02 | ct-clock, ct-timeline | skew and partial-order tests |
| causal ranking | ADR-0007, DDD-04 | ct-hypothesis, ct-causal, ct-evalue | 20 frozen scenarios |
| analogue recall | ADR-0010, DDD-06 | ct-embedding, ct-ruvector, ct-recall | upstream search receipt |
| policy gate | ADR-0013, DDD-08 | ct-remediation, ct-regorus, ct-policy | real Rego fixtures |
| portable evidence | ADR-0016, DDD-09 | ct-evidence, ct-rvf | byte-stable roundtrip |
| orchestration | ADR-0019, DDD-10 | ct-orchestrator, ct-cli | E2E + cancellation |
| evaluation | ADR-0022, DDD-11 | benchmark subcommand | baselines and ablations |

