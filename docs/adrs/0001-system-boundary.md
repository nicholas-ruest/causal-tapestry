# ADR-0001: System boundary

Status: Accepted for implementation

## Context

Treat the product as an offline incident investigation system, not a monitoring daemon.

## Alternatives

A live mutation controller; a trace viewer; a benchmark-only tool.

## Decision

Use a replay-first investigation aggregate with proposals but no production mutation.

## Tradeoffs

Operators need a separate execution system.

## Consequences

Safer scope and deterministic evidence.

## Validation

E2E fixture ends in a verified bundle and never invokes a deployment API.

