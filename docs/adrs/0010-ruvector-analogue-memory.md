# ADR-0010: RuVector analogue memory


## Implementation and failure semantics

`ct-ruvector` is an anti-corruption layer over `ruvector_core::VectorDB`; it owns insertion, dimension checks, nearest-neighbour search, and conversion back to domain analogues. `ct-recall` is the only domain caller. The default end-to-end path inserts two historical investigations and performs a real search. The `NoRuVector` mode removes the adapter and returns zero analogues, making the boundary observable in tests and measured ablation output.
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

