# ADR-0015: Policy input minimization


## Implementation and failure semantics

The policy adapter receives a purpose-built serializable input containing evidence threshold status, reversibility, and blast radius; raw telemetry and arbitrary operator strings are excluded. This reduces both policy ambiguity and accidental sensitive-data disclosure. Field names are stable contract terms owned by `ct-policy`, and missing values are construction errors rather than Rego defaults. Allow/deny fixtures validate the schema and the README documents that IAM authorization remains external.
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

