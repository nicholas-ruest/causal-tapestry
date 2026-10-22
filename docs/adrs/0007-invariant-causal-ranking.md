# ADR-0007: Invariant causal ranking


## Implementation and failure semantics

The implementation partitions observations by declared environment and compares each candidate's mean severity contribution rather than pooling all records. Candidates without support across the minimum environment count cannot lead the ranking. Temporal precedence from the reconstructed timeline is a hard prerequisite, not a score feature. `ct-causal` emits typed ranked hypotheses for the e-process. Tests verify environment counts, invalid contrasts, and the frozen injected cache-root scenario.
Status: Accepted for implementation

## Context

Pooled correlation confounds environment-specific changes.

## Alternatives

Correlation; PC algorithm; LLM judge.

## Decision

Test residual stability across declared environments and rank only supported candidates.

## Tradeoffs

Narrower claims than general discovery.

## Consequences

Claims are tied to interventions.

## Validation

Injected-root scenarios meet top-1 threshold.

