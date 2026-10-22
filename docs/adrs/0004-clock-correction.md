# ADR-0004: Clock correction

Status: Accepted for implementation

## Context

Distributed clocks reorder causes and symptoms.

## Alternatives

Trust timestamps; use global offset.

## Decision

Fit per-service median offset from parent-child constraints and retain uncertainty.

## Tradeoffs

Cannot repair sparse traces.

## Consequences

Stable partial order with explicit uncertainty.

## Validation

Skew fixtures reproduce expected happens-before edges.

