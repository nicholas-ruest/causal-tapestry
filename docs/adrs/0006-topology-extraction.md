# ADR-0006: Topology extraction


## Implementation and failure semantics

`ct-topology` aggregates validated causal edges into typed service-to-service weights without retaining vendor payloads. Self-edges are ignored for cross-service blast-radius calculations but remain represented by service-local event counts upstream. Aggregation occurs after timeline validation, so no dangling or cyclic relation reaches this context. Callers use the map for hypotheses and remediation scope. The edge aggregation test fixes direction, weight, and deterministic ordering.
Status: Accepted for implementation

## Context

Causal candidates need service structure.

## Alternatives

Fully connected graph; operator-authored graph only.

## Decision

Derive call/deployment topology and accept signed overrides.

## Tradeoffs

Missing edges can reduce recall.

## Consequences

Candidate search is bounded and explainable.

## Validation

Graph fixtures verify edges and override provenance.

