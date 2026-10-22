# ADR-0012: Recall fusion


## Implementation and failure semantics

Recall augments a causal hypothesis but never substitutes for causal support. `ct-recall` queries the vector adapter only after the e-process has produced a ranked candidate, and it returns analogue identifiers and similarity separately from the hypothesis score. A high-similarity incident cannot authorize remediation when current evidence is weak. The orchestrator preserves both fields in the report and RVF record, allowing reviewers to distinguish present evidence from historical context.
Status: Accepted for implementation

## Context

Analogue similarity must not override causal evidence.

## Alternatives

Replace causal score; ignore analogues.

## Decision

Attach analogues as supporting evidence with capped weight and provenance.

## Tradeoffs

Can underuse strong history.

## Consequences

Causal claim remains primary.

## Validation

Ablation verifies bounded rank movement.

