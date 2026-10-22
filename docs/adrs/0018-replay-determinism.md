# ADR-0018: Replay determinism

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

