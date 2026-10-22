# ADR-0003: Canonical identities

Status: Accepted for implementation

## Context

Vendor identifiers differ and collisions corrupt graphs.

## Alternatives

Keep raw strings; random UUID rewrite.

## Decision

Normalize typed IDs while preserving raw provenance.

## Tradeoffs

Mapping table adds storage.

## Consequences

Stable joins and readable receipts.

## Validation

Collision fixtures and roundtrip tests.

