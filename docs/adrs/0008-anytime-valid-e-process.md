# ADR-0008: Anytime-valid e-process

Status: Accepted for implementation

## Context

Investigations inspect evidence sequentially, invalidating fixed-horizon p-values.

## Alternatives

Fixed p-values; Bayes factors with subjective priors.

## Decision

Multiply bounded per-partition e-values in log space with a predeclared threshold.

## Tradeoffs

Conservative and sensitive to clipping.

## Consequences

Streaming evidence remains valid under optional stopping.

## Validation

Null simulations control false promotions.

