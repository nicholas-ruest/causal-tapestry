# DDD-03: Service Topology

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

