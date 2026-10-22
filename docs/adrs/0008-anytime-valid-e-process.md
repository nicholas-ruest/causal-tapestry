# ADR-0008: Anytime-valid e-process


## Implementation and failure semantics

`ct-evalue` updates nonnegative wealth sequentially from bounded contrasts and rejects values outside the declared interval. The threshold is loaded from validated configuration before the incident begins, so an operator cannot lower it after observing results. Wealth and step count are copied into each promoted hypothesis and evidence record. Removing this component fixes e-value at 1.0 and fails authorization, which the integration test and benchmark expose as a capability loss.
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

