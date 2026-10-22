# ADR-0013: Regorus policy gate


## Implementation and failure semantics

`ct-regorus` owns the Microsoft Regorus engine lifecycle: load the pinned Rego policy, set a minimized JSON input, and evaluate the single allow rule. Parse, load, or evaluation errors fail closed through `ct-errors`. `ct-policy` translates the domain decision but cannot bypass the engine in combined mode. Tests exercise a bounded reversible allow and a wide-blast-radius deny. The explicit ablation labels its unsafe allow baseline so it cannot be confused with governance.
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

