# DDD-10: Investigation Saga


## Implementation and failure semantics

`Investigation` is the application aggregate coordinating context outputs without absorbing their rules. `ct-orchestrator` advances the saga only after each typed result validates, records major-stage evidence, and writes durability last. Any failure prevents later calls and returns one structured error. It exposes combined and controlled ablation modes through the same API. Three integration tests cover successful composition, capability loss, and fail-before-write behavior.
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

