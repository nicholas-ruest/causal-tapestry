# ADR-0006: Topology extraction

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

