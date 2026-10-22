# ADR-0019: Saga orchestration

Status: Accepted for implementation

## Context

Many failure boundaries need one explicit lifecycle.

## Alternatives

Ad hoc CLI sequence; distributed workflow engine.

## Decision

One cancellable investigation saga with typed states and compensation.

## Tradeoffs

Single-process throughput ceiling.

## Consequences

Easy deterministic testing.

## Validation

State-transition and cancellation tests.

