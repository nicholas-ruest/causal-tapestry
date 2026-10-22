# ADR-0025: Publication gate

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

