# ADR-0010: RuVector analogue memory

Status: Accepted for implementation

## Context

Similar historical incidents add context but must not become instructions.

## Alternatives

Lexical search; hosted vector API.

## Decision

Use pinned RuVector in one ACL with deterministic embeddings and verified top-k.

## Tradeoffs

Upstream API/build risk.

## Consequences

Fast reproducible analogue recall.

## Validation

Real upstream insert/search test and failure degradation.

