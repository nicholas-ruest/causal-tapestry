# ADR-0015: Policy input minimization

Status: Accepted for implementation

## Context

Sending raw telemetry to policy leaks data and couples schemas.

## Alternatives

Full corpus input; string template.

## Decision

Pass a versioned decision view with service, action, confidence and risk.

## Tradeoffs

Some policy rules require adapters.

## Consequences

Stable least-data boundary.

## Validation

Schema and unknown-field tests.

