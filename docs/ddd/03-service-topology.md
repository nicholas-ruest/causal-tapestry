# DDD-03: Service Topology


## Implementation and failure semantics

The topology aggregate groups directed service edges and event participation after reconstruction has established valid causality. `ct-topology` owns edge aggregation and deterministic ordering; no other context may infer blast radius from raw strings. Hypothesis generation reads service nodes, while remediation planning reads bounded neighbourhood size. The edge test fixes direction and multiplicity, and the safe fixture demonstrates topology-derived proposal scope.
## Responsibility

Own service/deployment graph.

## Ubiquitous language

call edge, deployment edge, signed override.

## Aggregates, entities, and value objects

ServiceGraph aggregate; ServiceNode and CallEdge entities.

## Invariants

No dangling nodes; overrides preserve provenance.

## Domain events

TopologyDerived, OverrideApplied

## Owned ports and adapters

TopologyPort.

## Context relationships

Consumes Reconstruction; supplies candidate reachability.

## Validation

Graph and override fixtures.

