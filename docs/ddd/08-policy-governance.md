# DDD-08: Policy Governance

## Responsibility

Own Regorus-backed decisions.

## Ubiquitous language

decision view, fail closed, policy diagnostic.

## Aggregates, entities, and value objects

PolicyDecision aggregate; DecisionInput entity; PolicyHash value.

## Invariants

Malformed/unavailable policy denies; policy hash bound.

## Domain events

PolicyLoaded, ProposalAllowed, ProposalDenied

## Owned ports and adapters

PolicyGatePort; RegorusAdapter owned.

## Context relationships

Consumes proposals; feeds Evidence.

## Validation

Real allow/deny/malformed fixtures.

