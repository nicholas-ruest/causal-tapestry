# ADR-0024: Security and limits

Status: Accepted for implementation

## Context

Hostile trace paths, huge values, NaNs and policy text can break the tool.

## Alternatives

Trust fixtures; sanitize late.

## Decision

Validate before allocation, canonicalize paths, reject nonfinite values, set timeouts.

## Tradeoffs

Rejects some unusual telemetry.

## Consequences

Safe deterministic boundaries.

## Validation

Fuzz/property tests and dependency audit.

