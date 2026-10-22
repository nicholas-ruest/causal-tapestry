# ADR-0025: Publication gate


## Implementation and failure semantics

Publication requires format, strict lint, locked tests, dependency policy, real upstream tests, benchmark and ablations, checker exit zero, independent semantic review, PR CI, governed merge, and repository/Gist readback. The structural checker is explicitly insufficient on its own. Candidate commits remain private until those gates pass, and merge uses an expected head SHA. Any missing receipt produces a non-success status instead of a marketing claim.
Status: Accepted for implementation

## Context

Structural counts can pass with decorative content.

## Alternatives

Publish on checker pass; publish on tests alone.

## Decision

Require checker 0, semantic call-path review, raw upstream receipts, CI green, PR merge and readback.

## Tradeoffs

Publication may be blocked.

## Consequences

BUILT status remains evidence-based.

## Validation

Exact commit is revalidated and read back.

