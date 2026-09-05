# RT-2 ordered folded-cohort capture

One controlled invocation completed one warmup and two measured rounds, producing 46 records accepted by the unchanged aggregate and individual validators. Raw and accepted files are byte-identical: 79,187 bytes, SHA-256 `f2ed6356ebda8e936c41a2af74a6e6e2de2cd6109094889557f9c598b34b8299`. The disposition preserves original stderr identity and exact launch counts.

Measured candidate: `9cd6ba25c7a3b7f80788cd04789a10d36ee10e92`. Runtime, workload, fixtures, validators and floor source are unchanged from immutable artifact/preflight candidate `0a0e39e42e4ae2585d5f5ee507a4cb9aaf7b741a`; intervening commits record artifact identities and qualification evidence. The exact runner-profile binary SHA-256 is `e0d9e2752e50df486c4622e1b7d80de46ac59d5cd698e4da81cdfe45d462227a`, matching the untimed prepared binary. Profile: opt-level 3, LTO false, codegen-units 16. The preflight default-profile binary is separate evidence, not this binary.

Readiness recorded load 0.32, binary age 236 seconds, CPU 63 affinity and sibling 31 at 0.00% busy. The runner independently recorded load 0.27 and the same affinity/sibling controls. Frozen limits remained load 0.50, cooldown 60 seconds and sibling busy 5%; no override or retry occurred.

| Workload | Round 1 p50 µs/block | Round 2 p50 µs/block |
|---|---:|---:|
| 64-track plumbing only | 6.700 | 6.910 |
| 64-track gain/pan only | 8.704 | 8.753 |
| 64-track console | 90.715 | 90.555 |
| 64-track idle | 25.979 | 25.849 |

These are descriptive observations, not a causal speedup claim. Plumbing-only has no bank/fold epilogue and serves as a control. Neither plumbing-only nor gain/pan-only emits fold/eligibility counters; the source and prepared dispatch/allocation fixtures establish RT-2 admission and mechanism. Missing cycle fields remain unavailable.

## Supported scalar inspection correction

`original-scalar-inspection.sh` and its original log are preserved as historical execution evidence. Astra found its archive enumeration reported mapfile status rather than the find/sort producers. `confirm-scalar-population.py` and `scalar-population.log` independently check find, sort, archive listing and member reads against the existing target. They prove exactly one archive per named family, the complete three-object manifest and byte/hash identity with every object previously decoded and scanned successfully. No rebuild was needed. This evidence covers scalar non-LTO engine/source/target_smoke objects only; it does not claim fat-LTO inspection or repair issue #404.

## Comparison with prior capture

All 46 unique `(record, workload_kind, round)` keys match #415. Every emitted output digest, transpose counter, render-error and forbidden-operation field has identical presence and value, along with the named stable fixture/layout/target fields. Error fields are emitted on 42 rows and are zero; four hoist rows omit them. `compare-rt1.py` reproduces this comparison and rejects missing/duplicate keys or mismatches. No absent field is interpreted as zero. Plumbing-only prior p50 was 6.710/6.790 µs; gain/pan-only was 8.332/8.503 µs. These separate captures do not isolate a causal RT-2 timing effect.
