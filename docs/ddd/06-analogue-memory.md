# DDD-06: Analogue Memory


## Implementation and failure semantics

Analogue Memory owns the translation from deterministic incident embeddings to RuVector records and back to `Analogue` values. `ct-embedding` defines the inspectable vector; `ct-ruvector` exclusively owns upstream API calls; `ct-recall` applies domain ordering. The context cannot change causal rank or policy input. Real VectorDB insertion/search tests and the zero-analogue ablation prove both execution and removable responsibility.
## Responsibility

Own deterministic incident recall.

## Ubiquitous language

incident fingerprint, top-k, analogue evidence.

## Aggregates, entities, and value objects

AnalogueCatalog aggregate; IncidentVector entity; Similarity value.

## Invariants

Vector dimension fixed; provenance mandatory; failures explicit.

## Domain events

AnalogueIndexed, AnaloguesRecalled, RecallDegraded

## Owned ports and adapters

EmbeddingPort; RuVectorIndexPort.

## Context relationships

Supplies bounded evidence to Causal Analysis/Application.

## Validation

Real upstream and ablation tests.

