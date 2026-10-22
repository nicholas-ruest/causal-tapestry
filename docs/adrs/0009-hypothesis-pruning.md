# ADR-0009: Hypothesis pruning


## Implementation and failure semantics

Pruning is deterministic: candidates are sorted by evidence, then stable identity, and truncated only after every declared environment contributes. This keeps runtime bounded without allowing arrival order to choose the survivor. The configured maximum is validated as nonzero and recorded in the report. Downstream recall embeds only retained hypotheses, while the evidence bundle includes their scores and threshold. Ranking tests cover ties and empty histories.
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

