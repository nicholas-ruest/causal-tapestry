# DDD-07: Remediation Planning

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

