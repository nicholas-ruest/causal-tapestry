# ADR-0013: Regorus policy gate

Status: Accepted for implementation

## Context

Remediation proposals require enterprise policy semantics.

## Alternatives

Hard-coded Rust; OPA sidecar; Cedar.

## Decision

Evaluate pinned Rego through Microsoft Regorus behind a fail-closed adapter.

## Tradeoffs

Policy compilation adds latency.

## Consequences

Portable policy-as-code and precise diagnostics.

## Validation

Real allow/deny/malformed policy tests.

