# ADR-0019: Saga orchestration


## Implementation and failure semantics

The orchestrator executes admission, reconstruction, analysis, recall, proposal, policy, evidence-chain, and RVF stages in a fixed saga. Each stage returns a typed value or typed error; later stages cannot observe partial unvalidated state. Durable output is last, and failure before that point leaves no evidence file. RVF readback completes the saga. Integration tests cover the full path, each major ingredient ablation, and malformed input before persistence.
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

