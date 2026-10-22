# ADR-0002: Corpus contract

Status: Accepted for implementation

## Context

Unbounded telemetry can exhaust memory and hide malformed causality.

## Alternatives

Accept arbitrary JSON; require a database ingest.

## Decision

Use a versioned bounded JSON corpus with explicit limits and hashes.

## Tradeoffs

Requires export before analysis.

## Consequences

Inputs are portable and reject early.

## Validation

Boundary tests cover size, path, timestamp and duplicate IDs.

