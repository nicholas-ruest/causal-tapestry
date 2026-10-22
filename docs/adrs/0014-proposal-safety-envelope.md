# ADR-0014: Proposal safety envelope


## Implementation and failure semantics

`ct-remediation` creates only a reversible traffic-isolation proposal with target, blast radius, and rollback description. It has no executor trait, credentials, or network dependency. Blast radius is derived from validated topology rather than caller text, and an empty target cannot be constructed. Regorus receives these typed fields alongside evidence strength. Unit and integration tests ensure the safe fixture remains bounded and that the report describes a proposal, never an applied change.
Status: Accepted for implementation

## Context

The project must not execute remediation.

## Alternatives

Shell execution; API mutation; text advice only.

## Decision

Emit typed proposals with scope, preconditions, rollback and risk; never execute.

## Tradeoffs

Operator must use another system.

## Consequences

Evidence is useful without production authority.

## Validation

Static scan and tests prove no mutating adapters.

