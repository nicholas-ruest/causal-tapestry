# ADR-0009: Hypothesis pruning

Status: Accepted for implementation

## Context

Candidate explosion harms latency and interpretability.

## Alternatives

All pairs; one heuristic cause.

## Decision

Require topology reachability, temporal precedence, and symptom coverage.

## Tradeoffs

True cause can be pruned if telemetry is incomplete.

## Consequences

Auditable bounded search.

## Validation

Ablation reports recall/latency tradeoff.

