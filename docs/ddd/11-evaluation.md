# DDD-11: Evaluation


## Implementation and failure semantics

Evaluation owns the frozen fixture, expansion rule, iteration count, measurement schema, and interpretation of ablations. It does not own production behavior or tune thresholds after results. The benchmark executable calls the public orchestrator for every mode, so it measures the same upstream and evidence boundaries as the CLI. Corpus and lockfile hashes bind reproducibility. Capability fields accompany latency to prevent faster-but-empty baselines from appearing superior.
## Responsibility

Own datasets, baselines and ablation comparison.

## Ubiquitous language

dataset hash, seed, baseline, ablation.

## Aggregates, entities, and value objects

EvaluationRun aggregate; Scenario entity; Metric value.

## Invariants

Same inputs across variants; raw results immutable; no fake metrics.

## Domain events

BenchmarkStarted, RunRecorded, ComparisonPublished

## Owned ports and adapters

BenchmarkPort.

## Context relationships

Calls Investigation Saga with feature matrix.

## Validation

Twenty frozen scenarios and receipt checks.

