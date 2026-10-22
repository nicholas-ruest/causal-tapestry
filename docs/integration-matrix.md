# Integration matrix

| Ingredient | Pin / license | Executed boundary | Crate | Test | Ablation |
|---|---|---|---|---|---|
| RuVector core | 46d816d; MIT | in-process index insert and top-k query | ct-ruvector | real_upstream_ruvector | replace with lexical scan |
| RVF | 46d816d; MIT OR Apache-2.0 | write, verify, read evidence bundle | ct-rvf | real_upstream_rvf_roundtrip | JSON-only receipt |
| Microsoft Regorus | 9d19b6a; MIT AND Apache-2.0 AND BSD-3-Clause | parse/evaluate Rego policy | ct-regorus | real_upstream_regorus | static deny-only gate |
| Invariant e-process | cited papers; implemented locally | per-environment causal evidence accumulation | ct-causal + ct-evalue | frozen causal scenarios | pooled correlation |
| Ruflo 3.47.1 | installed runtime | coordination and provenance receipts | external ledger | transcript readback | disclose absent ledger |
| MetaHarness/Darwin/Flywheel | local source checkout | evaluation receipt only; no runtime authority | evidence scripts | source-bound run | disclose if unavailable |

