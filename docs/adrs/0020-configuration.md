# ADR-0020: Configuration


## Implementation and failure semantics

`ct-config` owns numeric thresholds and limits, provides conservative defaults, and validates every field before orchestration. Invalid environment count, evidence threshold, hypothesis limit, or blast-radius bound fails before telemetry admission. Configuration is passed explicitly instead of read globally inside domain crates, which makes replay and tests honest. The default-validation test plus the end-to-end fixture establish the accepted production-like profile.
Status: Accepted for implementation

## Context

Implicit defaults make evidence ambiguous.

## Alternatives

Environment only; config file only.

## Decision

Validated config with safe defaults, environment overrides, and emitted effective config hash.

## Tradeoffs

More setup.

## Consequences

Receipts explain behavior.

## Validation

Invalid and precedence tests.

