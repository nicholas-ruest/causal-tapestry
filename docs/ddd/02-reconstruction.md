# DDD-02: Reconstruction


## Implementation and failure semantics

The Reconstruction context owns corrected timestamps, parent resolution, and the valid partial order. `ct-clock`, `ct-normalize`, and `ct-timeline` collaborate through typed values rather than shared mutable state. A parent may be resolved only within the admitted corpus; cycles and duplicate identities terminate reconstruction. Its output is immutable and consumed by topology and causal analysis. Median-outlier, stable-sort, and cycle tests pin its invariants.
## Responsibility

Own corrected time and stable partial order.

## Ubiquitous language

happens-before, skew envelope, concurrent event.

## Aggregates, entities, and value objects

Timeline aggregate; CanonicalEvent entity; CorrectedTime value.

## Invariants

DAG acyclic; tie-break stable; uncertainty retained.

## Domain events

ClockModelFitted, TimelineBuilt

## Owned ports and adapters

ClockPort and TimelinePort.

## Context relationships

Consumes Acquisition; feeds Topology and Causal Analysis.

## Validation

Permutation and skew tests.

