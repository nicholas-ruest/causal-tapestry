# ADR-0023: Ablation contract

Status: Accepted for implementation

## Context

Ingredient value cannot be inferred from presence.

## Alternatives

No ablations; arbitrary flags.

## Decision

Predeclare combined, RuVector-only, Regorus-only, causal-only and removal runs.

## Tradeoffs

Extra runtime.

## Consequences

Composition claims are falsifiable.

## Validation

All rows share dataset hash and seed.

