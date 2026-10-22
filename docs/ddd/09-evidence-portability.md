# DDD-09: Evidence Portability


## Implementation and failure semantics

The evidence aggregate contains ordered stage records, their hash-chain root, upstream versions, decision data, and report identifiers. `ct-evidence` owns canonical hashing; `ct-rvf` owns append-only storage and reopen/query verification. The aggregate is complete only after readback reproduces expected records. No domain context writes a shadow database. Chain and real-RVF round-trip tests validate integrity, while the no-RVF ablation leaves analysis intentionally non-durable.
## Responsibility

Own append-only records and RVF bundle.

## Ubiquitous language

segment, manifest, supersession, seal.

## Aggregates, entities, and value objects

EvidenceLedger aggregate; EvidenceRecord entity; BundleHash value.

## Invariants

Content addressed; finalize once; verification before replay.

## Domain events

EvidenceAppended, BundleSealed, ReplayVerified

## Owned ports and adapters

EvidencePort; RvfBundlePort owned.

## Context relationships

Consumes all context events; supplies Replay.

## Validation

Real RVF roundtrip and tamper tests.

