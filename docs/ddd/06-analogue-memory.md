# DDD-06: Analogue Memory

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

