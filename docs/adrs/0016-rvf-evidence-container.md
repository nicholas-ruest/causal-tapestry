# ADR-0016: RVF evidence container


## Implementation and failure semantics

`ct-rvf` converts typed evidence records into `rvf_runtime::RvfStore` segments, closes the store, reopens it from disk, and queries records before returning success. A successful write without readback is therefore not accepted as durable evidence. The adapter owns filesystem interaction and path errors; the orchestrator only supplies a validated output path. The real round-trip unit test and combined integration test execute the upstream runtime rather than a mock.
Status: Accepted for implementation

## Context

A directory of JSON files is easy to alter and hard to replay.

## Alternatives

Tarball; database snapshot; JSON only.

## Decision

Use pinned RVF writer/reader for manifest, segments and integrity verification.

## Tradeoffs

Upstream API must be proven.

## Consequences

Portable tamper-evident investigation.

## Validation

Real RVF roundtrip plus bit-flip rejection.

