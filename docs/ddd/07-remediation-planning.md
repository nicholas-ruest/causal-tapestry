# DDD-07: Remediation Planning


## Implementation and failure semantics

The `RemediationProposal` aggregate is a reversible suggestion derived from the leading supported hypothesis and validated topology. Its invariants require a target, bounded blast radius, rollback description, and the absence of execution authority. `ct-remediation` is the sole owner. Policy Governance consumes the proposal, but only an external system could later act. Tests assert reversibility, and the README states clearly that no production mutation occurs.
## Responsibility

Own non-executable proposals.

## Ubiquitous language

proposal, precondition, rollback, blast radius.

## Aggregates, entities, and value objects

RemediationPlan aggregate; Proposal entity; RollbackStep value.

## Invariants

Every proposal has rollback and bounded scope; no execute method.

## Domain events

ProposalDrafted, ProposalWithdrawn

## Owned ports and adapters

PlannerPort.

## Context relationships

Consumes promoted causes; feeds Governance.

## Validation

Invariant and no-side-effect tests.

