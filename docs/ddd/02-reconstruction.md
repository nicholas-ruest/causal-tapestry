# DDD-02: Reconstruction

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

