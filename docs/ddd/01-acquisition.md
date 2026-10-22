# DDD-01: Acquisition


## Implementation and failure semantics

`IncidentCorpus` is created atomically from the entire bounded input; there is no partially admitted aggregate. `ct-ingest` owns JSON-line parsing and size checks, while `ct-otel` owns vendor-shape decoding behind the acquisition boundary. On acceptance it emits canonical raw events plus a corpus hash; on rejection it returns a typed reason and downstream ports are never called. The malformed end-to-end test confirms the evidence path remains absent.
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

