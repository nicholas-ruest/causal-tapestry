# DDD-12: Operator Interface

## Responsibility

Own CLI input/output and replay commands.

## Ubiquitous language

investigate, replay, benchmark.

## Aggregates, entities, and value objects

OperatorSession aggregate; Command entity; ExitCode value.

## Invariants

Machine-readable output stable; errors nonzero; safe paths only.

## Domain events

CommandAccepted, ReportRendered

## Owned ports and adapters

CLI command ports.

## Context relationships

Customer of Saga and Evaluation.

## Validation

CLI integration and snapshot tests.

