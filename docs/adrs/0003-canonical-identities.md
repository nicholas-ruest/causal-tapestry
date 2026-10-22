# ADR-0003: Canonical identities


## Implementation and failure semantics

Canonical identifiers are opaque strings preserved from admission through topology, hypotheses, evidence records, and the final report. The normalizer never derives identity from display names or array position, because either would make replay order-dependent. Parent references are resolved only after the full corpus is admitted. Duplicate IDs and missing references are errors with source context. Timeline and topology tests prove stable ordering when timestamps tie.
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

