# ADR-0017: Append-only evidence


## Implementation and failure semantics

`ct-evidence` builds a hash chain over canonical serialized records: each digest binds the previous root and current payload. Records are produced in fixed stage order and are never updated in place. The final root is included in both the report and RVF contents so later readback can detect omission or reordering. Chain verification runs before RVF success is reported. Tests alter ordering and confirm that only the original sequence reproduces the stored root.
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

