# DDD-04: Causal Analysis


## Implementation and failure semantics

The context receives a reconstructed timeline partitioned by environment and emits ranked `Hypothesis` entities with per-environment support. It never consumes recalled analogues while estimating current causality, preventing historical similarity from leaking into the causal score. `ct-hypothesis` generates eligible causes and `ct-causal` ranks invariant support. The e-process is a downstream collaborator. Frozen scenarios and ablations distinguish causal evidence from recall.
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

