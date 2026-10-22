# DDD-01: Acquisition

## Responsibility

Own corpus admission and vendor decoding.

## Ubiquitous language

TraceId, raw event, corpus boundary.

## Aggregates, entities, and value objects

IncidentCorpus aggregate; RawEvent entity; CorpusHash value.

## Invariants

Reject before downstream calls; IDs unique; sizes bounded.

## Domain events

Accepted, CorpusRejected

## Owned ports and adapters

IngestPort inbound; OtelDecodePort owned.

## Context relationships

Feeds Reconstruction through CanonicalEvent.

## Validation

Golden, invalid, and limit fixtures.

