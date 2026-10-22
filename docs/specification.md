# Specification

## Original problem and users

SRE incident commanders, service owners, and regulated-platform auditors need one reproducible investigation that connects raw traces to root-cause claims and policy-safe remediation. Existing tracing tools show correlations; they do not combine invariant causal evidence, incident analogue recall, policy evaluation, and portable signed evidence.

## System capability

Given a frozen trace corpus and declared environment partitions, Causal Tapestry deterministically reconstructs an incident, ranks causal hypotheses using an anytime-valid e-process, recalls structurally similar incidents, evaluates remediation policy, and emits a replayable RVF receipt.

## End-to-end workload

1. Ingest 600 OpenTelemetry-like span, log, metric, and deployment events across six services and three environment partitions.
2. Validate identifiers, timestamps, parent edges, paths, and bounded sizes.
3. Normalize clocks and reconstruct a stable partial-order timeline.
4. Generate and prune candidate causal edges.
5. Accumulate per-environment evidence and rank hypotheses with e-values.
6. Query RuVector with deterministic incident embeddings for analogous incidents.
7. Form remediation proposals and evaluate Rego policy in Regorus.
8. Seal inputs, ranked claims, policy decisions, and hashes as an RVF bundle.
9. Replay the bundle and reproduce rankings and decisions byte-for-byte.

## Success criteria

- True injected root cause is top-1 in at least 18 of 20 frozen scenarios.
- Replay reproduces claim order, policy result, and manifest hash exactly.
- RuVector recall returns the matching incident family in top-3 for at least 90% of scenarios.
- Unsafe remediation is denied in 100% of policy fixtures.
- Removing each major ingredient degrades its predeclared metric.
- The combined path processes the 600-event corpus in under 2 seconds on the recorded CI runner.
- All 22 crates participate in tests or executable call paths.

## Constraints

No live production mutations; proposals only. No LLM is required for causal ranking. Missing environment partitions fail closed for causal claims. Missing RuVector degrades recall but is explicit. Missing Regorus blocks proposal approval. RVF failure blocks completion.

