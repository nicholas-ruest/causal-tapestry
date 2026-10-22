# ADR-0002: Corpus contract


## Implementation and failure semantics

`ct-ingest` accepts newline-delimited JSON under explicit byte, event-count, identifier, and numeric-range limits before any upstream engine is called. A malformed line aborts the whole corpus rather than creating a partial investigation with ambiguous provenance. The fixture hash binds benchmarks to exact bytes. Unit tests cover empty input, malformed structures, and duplicate identities; the end-to-end failure test confirms that no RVF file exists after admission failure.
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

