# ADR-0001: System boundary


## Implementation and failure semantics

The `ct-orchestrator` application service owns the complete investigation transaction, while `ct-cli` is only an input/output adapter. Its output is a typed report plus an RVF path; there is deliberately no port capable of mutating production. Cancellation or adapter failure returns a typed error before a remediation can be represented as executed. Reviewers validate the boundary by searching dependency edges and confirming that only evidence and proposal types cross the application surface.
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

