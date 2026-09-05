# Deferred RT-1 descriptive measurement

One controlled invocation completed one warmup and two measured rounds, producing 46 accepted records. The unchanged production per-record and aggregate validators pass. Candidate `a74477c68eaf8650fdc86ec6d0f3ac04a18cb880` contains the RT-1 implementation merged as `1fa4424d732b0d9150dda5512da80cb95d76a33e`, plus only this measurement's registration and evidence.

## Recorded timings

| Workload | Round 1 p50, µs/block | Round 2 p50, µs/block |
|---|---:|---:|
| 64-track idle | 24.717 | 25.078 |
| 64-track console | 90.024 | 90.325 |
| 9-track ragged strip | 20.961 | 21.041 |
| Mono collapse eligible | 58.257 | 58.407 |
| Mono collapse forced off | 91.697 | 92.778 |

These are descriptive observations from one capture, not evidence of an RT-1 speedup. Mono eligible/forced-off are existing paired workload arms, not before/after RT-1 implementations. No cycle fields were emitted; cycle data remains unavailable and none is inferred from wall time, CPU metadata or floor denominators.

## Identity and interpretation

The matching key is `(record, workload_kind, round)`. All 46 keys are unique and match both `artifacts/issue368-floor-recount` and `artifacts/issue388-lane4-evidence`. Each authority passes all 160 explicit output-digest, transpose-counter and render-error/forbidden-operation field comparisons; the other stable fixture/layout/configuration fields also match. The placement row retains eight split and eight merged transposes per block. Both mono arms preserve output identity and eight transposes per block.

The direct full-unfolded-bank traffic reduction is established by #399's source and fixtures; retained staging capacity and folded/partial fallbacks are unchanged. This benchmark contains folded workloads and does not isolate a causal timing benefit from direct scatter. #368 supplies controlled same-workload output/structure evidence but has a different binary, candidate and load history. #388 is explicitly uncontrolled. Neither is used here as a causal timing baseline. No floor, workload, fixture or production validator changed.

## Control and provenance

The host was an AMD EPYC 9355, performance governor, native Simd8, 48 kHz and 128-frame quantum, Rust 1.97.1 / LLVM 22.1.6. Preflight passed with zero workload launches. Readiness recorded load 0.20, binary age 113 seconds, affinity CPU 63 and sibling 31 busy 0.00%; the runner independently recorded load 0.23. The frozen limits remained load 0.50, cooldown 60 seconds and sibling busy 5%. No uncontrolled override was used.

The runner used opt-level 3, LTO=false and codegen-units=16. Its binary SHA-256 was `748fe3460c6c52d457261f2cd41d4319fd6c888c0f525031cead8354fdb76eaf`, matching the untimed prepared binary. The different default-release preflight binary is recorded separately in `preflight.json`. Current runner, validator, fixture and floor identities agree with the frozen preflight.

Raw and accepted data are byte-identical: 79,132 bytes, SHA-256 `ef4913f8267a1d5987a913f2bc7d4ceeb82f310d17df3518b947fd4bda1e559f`. Original stderr is 88 bytes and matches its disposition digest. The PASS disposition records exactly one invocation, three workload launches, one warmup and two completed measured rounds. #399's refused capture remains untouched. No measurement was repeated.
