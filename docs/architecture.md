# Architecture

## Bounded contexts

Acquisition owns external trace decoding. Reconstruction owns clocks, timelines, and topology. Causal Analysis owns hypotheses and anytime-valid evidence. Analogue Memory owns RuVector. Remediation Governance owns proposals and Regorus. Evidence Portability owns RVF sealing and replay. Application owns orchestration and operator interfaces.

## Dependency rule

Domain crates do not import upstream adapters. The orchestrator depends on typed ports; adapter crates implement those ports. Only ct-ruvector imports RuVector, only ct-regorus imports Regorus, and only ct-rvf imports RVF packages.

## Workspace map

| # | Crate | Responsibility | Typed public API | Real caller | Design refs | Tests and acceptance |
|---:|---|---|---|---|---|---|
| 1 | `ct-types` | Shared value objects | `TraceId, IncidentId, ServiceId, EvidenceHash` | all crates | ADR-0001; DDD-01 | unit + caller-path + acceptance |
| 2 | `ct-errors` | Typed error taxonomy | `TapestryError, ErrorCode` | all crates | ADR-0002; DDD-02 | unit + caller-path + acceptance |
| 3 | `ct-config` | Validated runtime configuration | `Config::load, Limits` | ct-runtime, ct-cli | ADR-0003; DDD-03 | unit + caller-path + acceptance |
| 4 | `ct-ingest` | Corpus boundary and validation | `Ingestor::ingest` | ct-orchestrator | ADR-0004; DDD-04 | unit + caller-path + acceptance |
| 5 | `ct-otel` | OpenTelemetry-like envelope adapter | `OtelDecoder::decode` | ct-ingest | ADR-0005; DDD-05 | unit + caller-path + acceptance |
| 6 | `ct-normalize` | Canonical event normalization | `Normalizer::normalize` | ct-orchestrator | ADR-0006; DDD-06 | unit + caller-path + acceptance |
| 7 | `ct-clock` | Clock-skew correction | `ClockModel::fit, correct` | ct-normalize | ADR-0007; DDD-07 | unit + caller-path + acceptance |
| 8 | `ct-timeline` | Stable partial-order reconstruction | `TimelineBuilder::build` | ct-orchestrator | ADR-0008; DDD-08 | unit + caller-path + acceptance |
| 9 | `ct-topology` | Service graph construction | `Topology::from_timeline` | ct-hypothesis | ADR-0009; DDD-09 | unit + caller-path + acceptance |
| 10 | `ct-hypothesis` | Candidate cause generation | `HypothesisGenerator::generate` | ct-causal | ADR-0010; DDD-10 | unit + caller-path + acceptance |
| 11 | `ct-causal` | Invariant causal scoring | `CausalRanker::rank` | ct-orchestrator | ADR-0011; DDD-11 | unit + caller-path + acceptance |
| 12 | `ct-evalue` | Anytime-valid e-process | `EProcess::observe` | ct-causal | ADR-0012; DDD-12 | unit + caller-path + acceptance |
| 13 | `ct-embedding` | Deterministic incident features | `Embedder::embed` | ct-ruvector | ADR-0013; DDD-01 | unit + caller-path + acceptance |
| 14 | `ct-ruvector` | RuVector analogue adapter | `AnalogueIndex::insert/search` | ct-recall | ADR-0014; DDD-02 | unit + caller-path + acceptance |
| 15 | `ct-recall` | Analogue evidence fusion | `RecallService::recall` | ct-orchestrator | ADR-0015; DDD-03 | unit + caller-path + acceptance |
| 16 | `ct-remediation` | Proposal construction | `Planner::propose` | ct-orchestrator | ADR-0016; DDD-04 | unit + caller-path + acceptance |
| 17 | `ct-regorus` | Microsoft Regorus adapter | `PolicyGate::evaluate` | ct-policy | ADR-0017; DDD-05 | unit + caller-path + acceptance |
| 18 | `ct-policy` | Policy decision aggregation | `DecisionService::decide` | ct-orchestrator | ADR-0018; DDD-06 | unit + caller-path + acceptance |
| 19 | `ct-rvf` | RVF evidence sealing and replay | `EvidenceBundle::seal/replay` | ct-evidence | ADR-0019; DDD-07 | unit + caller-path + acceptance |
| 20 | `ct-evidence` | Append-only evidence ledger | `EvidenceLedger::append/finalize` | ct-orchestrator | ADR-0020; DDD-08 | unit + caller-path + acceptance |
| 21 | `ct-orchestrator` | Investigation saga | `Investigator::run` | ct-cli, integration tests | ADR-0021; DDD-09 | unit + caller-path + acceptance |
| 22 | `ct-cli` | Operator executable | `investigate, replay, benchmark` | operator | ADR-0022; DDD-10 | unit + caller-path + acceptance |

## Call path

ct-cli calls ct-orchestrator. The orchestrator calls ingest, normalize, timeline, topology, hypothesis, causal/evalue, recall/ruvector, remediation/policy/regorus, and evidence/rvf. Shared types, errors, and config are consumed across boundaries. Every adapter is exercised by the frozen end-to-end fixture.

## Failure boundaries

- Invalid corpus: reject before any evidence append.
- Clock model underdetermined: quarantine affected partition.
- Causal partition missing: no promoted root-cause claim.
- RuVector unavailable: explicit degraded recall, never synthetic results.
- Regorus unavailable or malformed policy: deny approval.
- RVF write or verification failure: investigation remains incomplete.
- Cancellation and timeouts propagate from orchestrator to all adapters.

