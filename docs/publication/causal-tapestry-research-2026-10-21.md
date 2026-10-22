# Dream Machine Research — 2026-10-21 — Causal Tapestry

## Problem and users

SREs and incident commanders often receive clock-skewed traces, logs, and metrics from multiple services. Existing trace viewers help navigate an incident, but they do not combine reproducible partial-order reconstruction, cross-environment causal stability, anytime-valid evidence, historical analogue recall, policy-governed remediation proposals, and portable append-only readback.

Causal Tapestry targets responders who need to explain why a cause was promoted and preserve the complete decision path for review.

## Candidate comparison

| Candidate | System capability | Why selected/rejected |
|---|---|---|
| Causal Tapestry | Reconstruct distributed incidents and produce governed, replayable causal evidence | Selected: naturally requires acquisition, reconstruction, causal analysis, memory, policy, evidence, evaluation, and operator subsystems |
| Capacity forecaster | Predict resource saturation | Rejected: too close to a scoring/forecasting utility without a necessary multi-system transaction |
| Policy explainer | Explain Rego decisions | Rejected: wrapper-scale and dominated by one enterprise dependency |
| Trace normalizer | Convert vendor telemetry | Rejected: adapter-scale and lacks causal/evidence/governance depth |

The selection is materially distinct from recent Dream Machine systems: it is incident reconstruction and causal root-cause evidence, not provenance supply-chain control, memory continuity, context migration, counterfactual plan shielding, or canary-release governance.

## Executed integration matrix

| Ingredient | Pin | License | Executed boundary | Test | Ablation |
|---|---|---|---|---|---|
| RuVector core | 2.4.0 from ruvnet/RuVector 46d816ddc4c2a8a06cd3613534a79b1d557d858b | MIT | Rust VectorDB insert and nearest-neighbour search | nearest_analogue_executes_ruvector | NoRuVector yields zero analogues |
| RVF runtime | 0.3.2 from the same source commit | MIT OR Apache-2.0 | RvfStore create, ingest, close, reopen, query | real_rvf_round_trip | NoRvf leaves no durable bundle |
| Microsoft Regorus | 0.13.0 from microsoft/regorus 9d19b6a578d22d6a223c48ba37e632d7d74ee0db | MIT AND Apache-2.0 AND BSD-3-Clause | Engine policy load, minimized input, rule evaluation | bounded allow / wide-blast deny | NoRegorus uses labelled unsafe baseline |
| Invariant causal prediction | Peters, Bühlmann & Meinshausen (2016) | research mechanism | Candidate support must remain stable across declared environments | multi-environment fixture | pooled-only ranking excluded |
| Anytime-valid e-process | Ramdas et al.; Vovk & Wang | research mechanism | Nonnegative wealth updated under continuous inspection | supportive and invalid-contrast tests | NoEProcess fixes 1.0 and denies |

## Primary sources

- RuVector: https://github.com/ruvnet/RuVector/tree/46d816ddc4c2a8a06cd3613534a79b1d557d858b
- Regorus: https://github.com/microsoft/regorus/tree/9d19b6a578d22d6a223c48ba37e632d7d74ee0db
- Peters, Bühlmann & Meinshausen, Causal inference by using invariant prediction: https://doi.org/10.1111/rssb.12167
- Ramdas et al., Testing exchangeability: https://doi.org/10.1111/rssb.12315
- Vovk & Wang, E-values: calibration, combination and applications: https://doi.org/10.1214/20-STS790

## Success criteria

A valid system must reconstruct a deterministic timeline from 600 clock-skewed events; rank supported hypotheses; execute real RuVector, Regorus, and RVF boundaries; degrade observably under four ingredient ablations; fail before persistence on malformed telemetry; pass strict locked Rust gates; and preserve a read-back-verifiable evidence bundle.

This research record does not claim implementation success; the separate build record binds results to the final commit.