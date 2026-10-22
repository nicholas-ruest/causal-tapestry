# ADR-0017: Append-only evidence

Status: Accepted for implementation

## Context

Later stages must not rewrite earlier claims.

## Alternatives

Mutable report; event database.

## Decision

Append content-addressed evidence records and finalize once.

## Tradeoffs

Corrections become superseding records.

## Consequences

Audit trail is complete.

## Validation

Mutation and duplicate-finalize tests.

