# DDD-10: Investigation Saga

## Responsibility

Own cross-context lifecycle and cancellation.

## Ubiquitous language

stage, compensation, deadline, degraded mode.

## Aggregates, entities, and value objects

Investigation aggregate; Stage entity; Deadline value.

## Invariants

Monotonic state; one terminal state; cancellations propagate.

## Domain events

InvestigationStarted, StageCompleted, InvestigationFailed

## Owned ports and adapters

Investigator inbound port.

## Context relationships

Coordinates every context; CLI is its customer.

## Validation

E2E, cancellation and failure matrix.

