# ADR-0023: Ablation contract


## Implementation and failure semantics

Ablations are explicit `Mode` variants in the same orchestrator, so they retain all unrelated code paths and change one major ingredient at a time. No RuVector removes analogues; no Regorus exposes a labelled unsafe baseline; no e-process fixes evidence at 1.0 and denies; no RVF omits the durable artifact. Integration assertions and benchmark columns test these observable losses. A latency change alone is never accepted as proof of ingredient value.
Status: Accepted for implementation

## Context

Ingredient value cannot be inferred from presence.

## Alternatives

No ablations; arbitrary flags.

## Decision

Predeclare combined, RuVector-only, Regorus-only, causal-only and removal runs.

## Tradeoffs

Extra runtime.

## Consequences

Composition claims are falsifiable.

## Validation

All rows share dataset hash and seed.

