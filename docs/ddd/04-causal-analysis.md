# DDD-04: Causal Analysis

## Responsibility

Own hypothesis lifecycle and promotion.

## Ubiquitous language

environment, invariant residual, candidate cause.

## Aggregates, entities, and value objects

InvestigationHypotheses aggregate; Hypothesis entity; CausalScore value.

## Invariants

Temporal precedence; environment count minimum; threshold predeclared.

## Domain events

HypothesisGenerated, EvidenceObserved, CausePromoted

## Owned ports and adapters

HypothesisPort and CausalRankPort.

## Context relationships

Consumes Topology and Reconstruction; calls E-Process.

## Validation

Frozen causal scenarios and null simulations.

