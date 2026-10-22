# ADR-0012: Recall fusion

Status: Accepted for implementation

## Context

Analogue similarity must not override causal evidence.

## Alternatives

Replace causal score; ignore analogues.

## Decision

Attach analogues as supporting evidence with capped weight and provenance.

## Tradeoffs

Can underuse strong history.

## Consequences

Causal claim remains primary.

## Validation

Ablation verifies bounded rank movement.

