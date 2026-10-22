# Validation evidence

Candidate: `build/2026-10-21-causal-tapestry`; source state at this checkpoint:
`6af7c39207f21ab5d20472f17093e16d5201c800`. Lockfile SHA-256:
`787fe117ad29dc32958b9a0969b7eafb17d824b438aebc2e9054e3d20713686d`.
The authoritative publication commit and CI readback are appended after governed merge.

## Reproducible gates

| Gate | Exact command | Result |
|---|---|---|
| format | `cargo fmt --all -- --check` | exit 0 |
| strict lint | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| locked tests | `cargo test --locked --workspace` | exit 0; 24 unit/integration tests, 0 failures |
| policy/audit | `cargo deny check` | exit 0; advisories, bans, licenses, sources OK; duplicate versions warned |
| benchmark | `./scripts/run-benchmark.sh` | exit 0; five 600-event modes, 25 iterations each |
| MetaHarness | `ruflo metaharness score --format json` | exit 0; hard constraints 6/6, scaffold-ready |
| Darwin | `npx --yes @metaharness/darwin@0.10.3 bench verify /tmp/causal-tapestry-suite.json` | exit 0; hash `eca83410409b…` OK |
| Flywheel | `ruflo metaharness flywheel status --format json` | exit 0; valid genesis ledger, no promotion |

The test suite executes the real `ruvector_core::VectorDB`, `rvf_runtime::RvfStore`,
and `regorus::Engine` boundaries. The combined integration test traverses all bounded
contexts; ablations assert loss of analogue recall, policy enforcement, anytime-valid
evidence, and RVF durability. Malformed telemetry fails before evidence creation.

## Dependency-policy rationale

Cargo-deny explicitly permits MIT-0 because Regorus depends on `borrow-or-share` under
that license. Workspace-only path dependencies are allowed without synthetic version
constraints. `RUSTSEC-2025-0141` is documented and ignored because it marks transitive
`bincode 1.x` from pinned RuVector as unmaintained, not vulnerable; no compatible
RuVector release removes it. All other advisories remain denied.

## Governed receipts

- format: `/home/ruv/.ruos/activity/2026-10-22/run-cd-home-ruv-work-causal-tapestry-candidate-g-020905.log`
- lint: `/home/ruv/.ruos/activity/2026-10-22/run-cd-home-ruv-work-causal-tapestry-candidate-c-020910.log`
- locked tests: `/home/ruv/.ruos/activity/2026-10-22/run-cd-home-ruv-work-causal-tapestry-candidate-c-020915.log`
- dependency policy: `/home/ruv/.ruos/activity/2026-10-22/run-cd-home-ruv-work-causal-tapestry-candidate-c-020839.log`
- benchmark: `/home/ruv/.ruos/activity/2026-10-22/run-cd-home-ruv-work-causal-tapestry-candidate-s-020923.log`
- MetaHarness: `/home/ruv/.ruos/activity/2026-10-22/run-cd-home-ruv-work-causal-tapestry-candidate-r-021152.log`
- Darwin suite verify: `/home/ruv/.ruos/activity/2026-10-22/run-cd-home-ruv-work-causal-tapestry-candidate-n-021123.log`
- Flywheel status: `/home/ruv/.ruos/activity/2026-10-22/run-cd-home-ruv-work-causal-tapestry-candidate-r-021159.log`

## Structural and semantic review

The checker at `nicholas-ruest/ruvnet-dream-machine`
`80f006f6960bc4e491e97f51e890ff708e81aa88` passed its three regression tests.
Its first candidate run correctly failed all undersized ADR/DDD bodies. After those
records gained decision-specific implementation and failure semantics, the exact
indexed checkout at `1617e1d97f2099500761173a02b93923818140e3` exited 0 with
`STRUCTURAL_PASS_REQUIRES_EVIDENCE_REVIEW`: 25 ADRs, 12 DDDs, 22 implemented crates,
and two animated SVGs. Receipt:
`/home/ruv/.ruos/activity/2026-10-22/run-cd-home-ruv-work-causal-tapestry-candidate-g-021959.log`.

Independent review then confirmed 22 Cargo workspace members and all 22 on the
`ct-cli` executable dependency path. Source-bound inspection found real
`ruvector_core::VectorDB` construction/search, `rvf_runtime::RvfStore`
create/ingest/reopen/query, and `regorus::Engine` policy load/input/rule evaluation.
The 600-event benchmark and locked tests exercise those same paths. The checker is
therefore corroborating evidence only; GitHub PR CI and final public readback remain
separate publication gates.
