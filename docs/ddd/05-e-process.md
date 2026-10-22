# DDD-05: E-Process

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

