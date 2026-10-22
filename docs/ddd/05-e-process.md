# DDD-05: E-Process


## Implementation and failure semantics

The `EvidenceProcess` aggregate contains wealth, observation count, and the predeclared threshold. Each bounded contrast is a domain event that either advances nonnegative wealth or rejects invalid input; callers cannot overwrite wealth. Promotion occurs only when the current value clears the configured threshold. `ct-evalue` owns this lifecycle. Unit tests cover supportive accumulation and invalid contrasts, while no-e-process integration fixes value at 1.0 and denies.
## Responsibility

Own anytime-valid evidence accumulation.

## Ubiquitous language

test martingale, evidence wealth, clipping.

## Aggregates, entities, and value objects

EvidenceProcess aggregate; Observation entity; LogEValue value.

## Invariants

Values finite/nonnegative; log accumulation; no post-hoc threshold.

## Domain events

EValueUpdated, ThresholdCrossed

## Owned ports and adapters

EValuePort.

## Context relationships

Called only by Causal Analysis.

## Validation

Property tests and optional-stopping simulation.

