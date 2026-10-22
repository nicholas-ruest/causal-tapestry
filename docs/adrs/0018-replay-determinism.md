# ADR-0018: Replay determinism


## Implementation and failure semantics

All core transformations use sorted collections, stable tie-breakers, fixed embeddings, and immutable configuration. The system does not read wall-clock time, randomness, or network state during an investigation. Incident IDs and source bytes are explicit inputs; benchmark repetition namespaces IDs deterministically. This makes a report reproducible from its fixture and configuration, while upstream dependency versions are locked. Hash and benchmark evidence bind the exact corpus and lockfile.
Status: Accepted for implementation

## Context

Auditors must reproduce results.

## Alternatives

Best-effort rerun; store only final report.

## Decision

Bind config, seed, corpus, policy, versions and outputs in the bundle.

## Tradeoffs

Larger artifacts.

## Consequences

Exact replay comparison is possible.

## Validation

Two runs match hashes and ordered claims.

