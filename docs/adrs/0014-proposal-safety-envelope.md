# ADR-0014: Proposal safety envelope

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

