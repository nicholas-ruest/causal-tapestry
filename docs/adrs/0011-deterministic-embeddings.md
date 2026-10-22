# ADR-0011: Deterministic embeddings


## Implementation and failure semantics

`ct-embedding` maps a hypothesis and service topology to a fixed eight-dimensional vector with normalized, inspectable features. It does not claim semantic equivalence or use hidden model state. Dimension is encoded by the public type contract and checked again at the RuVector boundary. Identical inputs replay byte-for-byte, while changed evidence or topology changes the vector. Unit tests fix dimension and value range; the README states the research-grade limitation.
Status: Accepted for implementation

## Context

Remote models weaken replay.

## Alternatives

Hosted embeddings; learned local model.

## Decision

Hash stable normalized structural features into a fixed vector space.

## Tradeoffs

Lower semantic richness.

## Consequences

No network or model drift.

## Validation

Golden vector hash tests.

