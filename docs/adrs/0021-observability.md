# ADR-0021: Observability

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

