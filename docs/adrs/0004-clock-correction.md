# ADR-0004: Clock correction


## Implementation and failure semantics

`ct-clock` computes a deterministic median offset per source from declared observations and applies it before ordering. Median correction was chosen because one extreme clock sample must not move an entire service partition. Sources without calibration retain zero offset and are made visible rather than guessed. Corrected timestamps remain attached to original event identity so evidence can reproduce the transformation. The outlier test and replay fixture validate stable correction.
Status: Accepted for implementation

## Context

Distributed clocks reorder causes and symptoms.

## Alternatives

Trust timestamps; use global offset.

## Decision

Fit per-service median offset from parent-child constraints and retain uncertainty.

## Tradeoffs

Cannot repair sparse traces.

## Consequences

Stable partial order with explicit uncertainty.

## Validation

Skew fixtures reproduce expected happens-before edges.

