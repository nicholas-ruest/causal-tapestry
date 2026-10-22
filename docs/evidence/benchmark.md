# Benchmark and ablation evidence

## Frozen workload

The benchmark deterministically expands the 12-event `cache-collapse.jsonl` fixture
into 50 isolated trace groups: 600 events per investigation. Event identifiers,
parents, and timestamps are namespaced by repetition, preserving each causal chain
without cross-run edges. Fixture SHA-256 is
`4a13cb2aea6a0b285f8f67587b45f2483edc4ca3efd14e1f378015da94a249aa`.

## Method

`./scripts/run-benchmark.sh` executes a release build and 25 complete investigations
for each mode. Each iteration parses and validates telemetry, corrects clock skew,
reconstructs the partial order and topology, ranks invariant hypotheses, updates the
e-process, optionally queries RuVector, evaluates the Regorus policy, and optionally
writes and reads back RVF evidence. The seed is implicit because all inputs and
algorithms are deterministic. Times are wall-clock microseconds on the same RuOS
machine and should be treated as comparative, not universal performance claims.

## Raw result

| Mode | Events | Iterations | Total µs | Mean µs | Analogues | Allowed | Top e-value |
|---|---:|---:|---:|---:|---:|---|---:|
| combined | 600 | 25 | 109691 | 4387 | 2 | true | 2.067250 |
| no_ruvector | 600 | 25 | 107180 | 4287 | 0 | true | 2.067250 |
| no_regorus | 600 | 25 | 100220 | 4008 | 2 | true | 2.067250 |
| no_eprocess | 600 | 25 | 109735 | 4389 | 2 | false | 1.000000 |
| no_rvf | 600 | 25 | 64535 | 2581 | 2 | true | 2.067250 |

## Interpretation

RuVector removal eliminates all retrieved analogues. Regorus removal substitutes an
explicitly labelled unsafe-allow baseline, so the safe fixture still says `allowed`
but loses enforceable governance. E-process removal collapses evidence to 1.0 and
denies authorization. RVF removal preserves analysis but leaves no durable evidence
bundle or reopen/query readback. These are capability ablations, not merely latency
comparisons. The raw governed transcript is
`/home/ruv/.ruos/activity/2026-10-22/run-cd-home-ruv-work-causal-tapestry-candidate-s-020923.log`;
the RuOS activity index retains the exact command, exit code 0, and stdout.
