# ADR-0022: Benchmark design


## Implementation and failure semantics

The benchmark expands a frozen 12-event trace into 50 independent trace groups, yielding 600 events without cross-group causal edges. It runs 25 complete investigations per mode on one machine and reports total and mean wall time plus capability outputs. No warmup is hidden, and compilation output is outside the printed CSV rows. Results are comparative rather than hardware claims. Fixture and lockfile hashes permit reproduction from the published source.
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

