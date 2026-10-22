# DDD-12: Operator Interface


## Implementation and failure semantics

The operator interface owns argument parsing, safe file reads, configuration selection, and JSON rendering through `ct-cli`. It delegates all investigation semantics to the orchestrator and has no direct dependency on RuVector, Regorus, or RVF. Invalid input paths and domain errors produce a nonzero exit without claiming evidence. The quickstart uses the checked-in fixture and locked dependencies; structured output exposes hypotheses, proposal, decision, root, and bundle path.
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

