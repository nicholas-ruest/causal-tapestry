# ADR-0005: Partial-order timeline


## Implementation and failure semantics

`ct-timeline` orders by corrected time and then identifier, while parent-child edges express happens-before constraints independently of wall-clock ties. This prevents a skewed timestamp from inventing causality. A cycle is rejected instead of broken heuristically, because any silent repair would contaminate causal claims. The orchestrator consumes the returned immutable sequence, and `ct-topology` consumes only validated edges. Tests exercise stable ties and cyclic failure.
Status: Accepted for implementation

## Context

A total sort invents order among concurrent events.

## Alternatives

Total timestamp sort; vector clocks required at source.

## Decision

Build a DAG from parent, deployment, and corrected-time constraints; stable topological tie-break.

## Tradeoffs

More complex than sorting.

## Consequences

Concurrency remains visible.

## Validation

Permutation tests produce identical timeline hashes.

