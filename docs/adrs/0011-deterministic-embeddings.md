# ADR-0011: Deterministic embeddings

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

