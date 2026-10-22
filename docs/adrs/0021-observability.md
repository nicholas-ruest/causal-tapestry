# ADR-0021: Observability


## Implementation and failure semantics

Observability is represented as stage-specific typed results, counts, evidence values, decisions, and governed executor transcripts rather than unconstrained event logging. The CLI emits one structured JSON report suitable for external tracing and avoids printing raw telemetry. Failures retain stage and cause through `ct-errors`. RuOS receipts capture commands and exit codes for validation. This design favors auditable boundaries while leaving metrics export to a deployment adapter.
Status: Accepted for implementation

## Context

The investigator itself needs diagnosable behavior without leaking corpus content.

## Alternatives

Verbose payload logs; no tracing.

## Decision

Structured spans with IDs, counts, durations and error codes only.

## Tradeoffs

Less debugging context.

## Consequences

Privacy-preserving operational evidence.

## Validation

Log snapshot excludes raw payloads.

