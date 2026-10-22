# ADR-0005: Partial-order timeline

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

