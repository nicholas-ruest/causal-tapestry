# ADR-0022: Benchmark design

Status: Accepted for implementation

## Context

Performance and capability claims need reproducible comparison.

## Alternatives

Microbench only; wall-clock anecdote.

## Decision

Freeze 20 scenarios, seed 20261021, runner metadata, accuracy and latency metrics.

## Tradeoffs

Synthetic fixtures limit generality.

## Consequences

Claims are source-bound and repeatable.

## Validation

Raw CSV/JSON plus command receipts.

