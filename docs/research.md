# Research

## Selected composition

### RuVector

Primary source: https://github.com/ruvnet/RuVector/tree/46d816ddc4c2a8a06cd3613534a79b1d557d858b  
Pin: 46d816ddc4c2a8a06cd3613534a79b1d557d858b. License: MIT. Package: ruvector-core 2.4.0. Executed role: HNSW-backed analogue storage and search through Dataset/Vector-style APIs verified during implementation. Interface uncertainty is a blocking spike, not permission to mock.

### RVF

Primary source: https://github.com/ruvnet/RuVector/tree/46d816ddc4c2a8a06cd3613534a79b1d557d858b/crates/rvf  
Same source pin; nested RVF workspace license MIT OR Apache-2.0. Packages include rvf-types, rvf-wire, and rvf-runtime. Executed role: create, verify, and replay an evidence bundle. Exact writer/reader API is verified by a source-pinned smoke test before adapter implementation.

### Microsoft Regorus

Primary source: https://github.com/microsoft/regorus/tree/9d19b6a578d22d6a223c48ba37e632d7d74ee0db  
Pin: 9d19b6a578d22d6a223c48ba37e632d7d74ee0db. Version: 0.13.0. License: MIT AND Apache-2.0 AND BSD-3-Clause. Executed role: parse policy, load structured input, evaluate the allow decision, and return typed diagnostics.

## Frontier mechanism

Causal claims use invariant prediction across declared environment partitions and an anytime-valid e-process. The architecture therefore stores per-partition residual evidence and never promotes a claim from a pooled correlation alone.

Primary sources:
- Peters, Buhlmann, Meinshausen, Causal inference by using invariant prediction: https://doi.org/10.1111/rssb.12167
- Ramdas et al., Testing exchangeability: forks, mergers, and e-processes: https://doi.org/10.1111/rssb.12315
- Vovk and Wang, E-values: calibration, combination and applications: https://doi.org/10.1214/20-STS790

Implementation is deliberately narrow: deterministic mean-shift residuals, bounded likelihood ratios, product e-process in log space, and a threshold declared before data. It does not claim general causal discovery.

## Rejected alternatives

PC/FCI alone lacks streaming anytime-valid evidence. A language model judge is nondeterministic and uncalibrated. Plain correlation cannot distinguish deployment-environment interventions. A proprietary vector service weakens reproducibility. OPA sidecars add deployment coupling where Regorus provides the policy language in-process.

