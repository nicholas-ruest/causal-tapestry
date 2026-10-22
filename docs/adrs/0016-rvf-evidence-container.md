# ADR-0016: RVF evidence container

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

