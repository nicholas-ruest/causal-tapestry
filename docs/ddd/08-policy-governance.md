# DDD-08: Policy Governance


## Implementation and failure semantics

Policy Governance combines evidence status and proposal safety into a `PolicyDecision` with explicit reasons. `ct-policy` owns the domain schema and invokes `ct-regorus`, whose anti-corruption layer owns Rego parsing and evaluation. Errors and undefined results become denial. The context neither edits proposals nor executes them. Allow/deny tests cover evidence and blast radius, while the no-Regorus baseline is visibly marked unsafe in results and evidence.
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

