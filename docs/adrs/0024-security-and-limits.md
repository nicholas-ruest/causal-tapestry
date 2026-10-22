# ADR-0024: Security and limits


## Implementation and failure semantics

Admission limits, numeric validation, opaque identities, minimized policy input, safe output paths, and absence of an execution port form the primary safety envelope. Cargo-deny rejects unknown sources and unapproved licenses; the one ignored advisory is documented as an unmaintained transitive bincode release, not a vulnerability. Regorus defaults to deny on evaluation failure. Malformed telemetry is proven not to create an RVF artifact or partial evidence chain.
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

