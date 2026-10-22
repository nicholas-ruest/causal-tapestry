# DDD-09: Evidence Portability

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

