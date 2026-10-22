# DDD-11: Evaluation

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

