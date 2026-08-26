# Issue #163 phase 2: the unfused multiply-add contract, measured

The owner's 2026-08-26 GO attached a condition: **the win must be confirmed at console
level, not just kernel level.** This directory is that confirmation, on both arms.

**64-track wasm console: 969.58 -> 172.74 us/block, 5.61x.** The projection the confirmation
rested on was 2.1-3.2x for unfusing alone, landing at ~305-453 us, "with wasm interleaving
composing after". Both parts arrived together: phase 3's interleaving was already in the tree but
recorded as near-null on wasm *until softfma was gone*, and the composed figure is 5.61x.

The browser's share of a core for 64 tracks goes **36.4% -> 6.5%**, and the wasm/native gap
collapses from **10.51x to 1.81x**.

## Attempts

| arm | attempt | files | status | launches |
|---|---|---|---|---|
| native | 1 | `console-benchmark.attempt-1-refused.*` | FAIL `precondition_loadavg_above_ceiling` | 0 |
| native | 2 | `console-benchmark.{raw,accepted,disposition}` | PASS `controlled` | 3 |
| wasm | 1 | `wasm-console-benchmark.attempt-1-refused.*` | FAIL `precondition_loadavg_above_ceiling` | 0 |
| wasm | 2 | `wasm-console-benchmark.{raw,accepted,disposition}` | PASS `controlled` | 3 |

Both arms refused once on the load ceiling and both refusals are kept. A directory that only ever
shows successes cannot distinguish "controlled" from "never tested".

## The wasm arm (the headline)

`wasm32-unknown-unknown` `+simd128` under wasmtime 47.0.3, the artifact the browser ships.
Baseline: `artifacts/issue163-phase2-wasm-baseline/`. Microseconds per 128-frame block, p50,
minimum of two measured rounds.

| row | wasm before | wasm after | speedup | digest |
|---|---|---|---|---|
| **console, `eq+compressor`, real fixture (64)** | 969.58 | **172.74** | **5.61x** | moved |
| console, `eq+compressor`, synthetic (128) | 1941.10 | **345.53** | **5.62x** | moved |
| decomposition: `eq` (64) | 811.62 | **91.23** | **8.90x** | moved |
| decomposition: `compressor` (64) | 461.59 | **127.82** | **3.61x** | moved |
| decomposition: `builtins` (64) | 303.30 | **47.38** | **6.40x** | moved |
| decomposition: `dispatch_only` (64) — **control** | 303.11 | **47.18** | **6.42x** | unmoved |
| **idle (silence input)** (64) — **control** | 328.59 | **73.14** | **4.49x** | unmoved |
| `parametric-eq-nine-track` fixture (9) | 125.06 | **16.08** | **7.78x** | moved |
| nine-track ragged strip (9) | 148.61 | **29.28** | **5.08x** | moved |

**The two controls are the strongest rows in the table.** `dispatch_only` and `idle` render
`output_sha256` `2b015145fd33` and `7b331c02e313` in *both* records -- byte-identical, so the
contract change provably did not alter what they compute -- and they still got **6.42x** and
**4.49x** faster. They are not no-op rows: they run the builtins filter chain configured as an
identity, where the fused and unfused forms agree exactly on every sample. That is the phase's
thesis reduced to a single measurement: same bits, same work, the browser's filter-kernel tax
deleted.

The `eq` row is the largest mover at **8.90x**, which is where the tax lived -- an SVF cascade is
the densest multiply-add path in the engine. The `compressor` row moves least (3.61x) because its
cost is dominated by the detector's polynomial dB conversions, which were already unfused
(`lane_math`, the precedent recorded in the audit).

### All three legs


| row | `native_simd8` before | after | `native_simd4` before | after | wasm/`native_simd8` before | after |
|---|---|---|---|---|---|---|
| **console, `eq+compressor`, real fixture (64)** | 92.27 | 95.46 | 312.82 | 363.28 | 10.51x | 1.81x |
| console, `eq+compressor`, synthetic (128) | 185.05 | 192.03 | 630.72 | 728.71 | 10.49x | 1.80x |
| decomposition: `eq` (64) | 43.31 | 45.00 | 147.40 | 180.51 | 18.74x | 2.03x |
| decomposition: `compressor` (64) | 71.86 | 74.17 | 198.59 | 216.60 | 6.42x | 1.72x |
| decomposition: `builtins` (64) | 23.14 | 23.86 | 31.26 | 33.66 | 13.10x | 1.99x |
| decomposition: `dispatch_only` (64) — **control** | 23.15 | 23.64 | 31.53 | 33.66 | 13.09x | 2.00x |
| **idle (silence input)** (64) — **control** | 38.28 | 38.85 | 36.87 | 38.95 | 8.58x | 1.88x |
| `parametric-eq-nine-track` fixture (9) | 8.38 | 9.20 | 21.14 | 25.93 | 14.91x | 1.75x |
| nine-track ragged strip (9) | 17.17 | 18.43 | 44.35 | 51.35 | 8.65x | 1.59x |

`native_simd4` is the one leg that got *slower* (312.82 -> 363.28, +16.1%). It is not a production
backend -- it exists in this record as the comparison denominator, "the same source at the lane
width `simd128` offers" -- and it is exactly what the contract change predicts on a target that
*has* a hardware fused multiply-add: unfusing there buys nothing and costs the second rounding plus
the longer dependency chain. It is the control in the other direction, and it is the reason the
wasm speedup can be attributed to softfma removal rather than to some unrelated change.

**Cross-backend identity holds.** Every record carries `digest_identity: all_legs_identical`; the
64-track console renders `30256f812f25` on `native_simd8`, `native_simd4` and `wasm_simd128`
alike. The baseline's `62c6c4d5d0f1` moved, which is the class-B change, and
`docs/rulings/unfused-multiply-add-audit.md` is the evidence the new bits are the intended ones.

## The native arm (must not regress meaningfully)

`artifacts/issue163-phase2/console-benchmark.accepted.jsonl`, paired against phase 3's record.

| workload | phase 3 | phase 2 | raw | drift-adjusted |
|---|---|---|---|---|
| **64-track console** | 91.03 | 95.10 | +4.47% | **-0.33%** |
| 128-track stretch | 184.21 | 190.50 | +3.42% | -1.33% |
| eq only | 41.55 | 43.48 | +4.65% | -0.15% |
| compressor only | 71.35 | 73.09 | +2.44% | -2.26% |
| builtins only | 21.69 | 22.66 | +4.48% | -0.31% |
| **dispatch only (control)** | 21.66 | 22.70 | **+4.81%** | +0.00% |
| **idle (control)** | 35.16 | 36.61 | **+4.13%** | -0.65% |
| nine-track baseline | 8.18 | 8.99 | +9.94% | +4.90% |
| nine-track ragged strip | 16.90 | 18.29 | +8.18% | +3.22% |

The raw +4.47% is **not** the contract change. `dispatch_only` renders `2b015145fd` in both
records -- byte-identical output -- and moved +4.81%; `idle` is likewise bit-identical and moved
+4.13%. That is the host between two sessions. Normalising by the bit-identical control puts the
64-track console at **-0.33%**, inside the round spread.

The fused->unfused latency chain goes **4 -> 6 cycles** per dependent multiply-add. On the 64-track
console phase 3's interleaving hides all of it, which is exactly what phase 3 predicted when it
recorded that the SVF loop is latency-bound rather than width-bound. The two nine-track rows are
the exception (+4.9%, +3.2% adjusted); they are the smallest workloads here, they were the noisiest
rows in phase 3 too, and a 9-track block is short enough for per-block fixed costs to dominate.
Recorded, not explained away.

## Interleave depth: re-tuned, and the answer is "unchanged"

`Lane::SVF_CASCADE_DEPTH` was re-swept post-softfma on the instrument that fixed it in phase 3
(`cargo test --release -p miso-engine-lane --test b2_interleave -- --ignored`), one EQ bank-block
= 4 sections x 2 channels x 128 frames, minimum of three rounds, ns/bank-block:

| width | D=1 | D=2 | D=4 | current | verdict |
|---|---|---|---|---|---|
| scalar | 2880.3 | 2681.0 | **2347.6** | 4 | unchanged, D=4 best (2.084x over serial) |
| `simd4` | 2296.1 | **1647.6** | 2013.5 | 2 | unchanged, D=2 best (2.403x over serial) |
| `simd8` | 2319.9 | **1685.2** | 2072.8 | 2 | unchanged, D=2 best (2.387x over serial) |

Every backend's constant is still the fastest legal depth. At `simd4` -- the width `simd128`
offers, and the one the brief asked about -- D=4 is 22% *slower* than D=2, so the choice is not
marginal.

**Boundary:** this is the native `simd4` arm, not wasm. Phase 3 recorded why no wasm interleave
timing exists: "the instrument that could produce one is the phase-0b kernel-timing harness, and
its cases come from the frozen G5 corpus; adding an interleaved case there is a re-pin, which
phase 3 is forbidden from doing." Phase 2 *is* the re-pin, so that blocker is now lifted and a
successor can add the case. This phase does not: the corpus was re-pinned once here already, and
re-pinning it a second time inside the same phase, for a constant the native evidence says is
already correct, is not a trade worth making.

## Measurement boundary

Fixture `fixtures/session/v1/console-sixty-four-track.toml`, 48 kHz, 128-frame quantum, 1000
observations, one warmup pass and two measured rounds, p50, descriptive only, no threshold. Both
arms ran under `check-bench-preconditions.sh` with `measurement_control: controlled`; both refused
once first.

The native comparison is **cross-session and single-arm**, not the #104 paired alternation. Pairing
is impossible between these two arms -- they differ by a rebuilt numeric contract, not a runtime
switch -- so the control normalisation does the job pairing would, and it is only honest because
the controls' outputs are byte-identical and their movement can therefore only be the host.

The wasm arm is wasmtime, not a browser. It is the determinism-pinned reference; browser numbers
remain the owner's field pass. `comparable_with_console_records: false` is set on every wasm record
and means what it says: the wasm runner's own native legs are its denominators, not the native
console record above.

Every `output_sha256` in this directory differs from every earlier phase's by construction. Phases
1, 3 and 4 were class A and their digest columns compare row against row; phase 2 changes the
numeric contract, so only the timing columns may be compared here.

## Standing note on fixture authority

This record is the **last sealed authority of the current 64-track fixture family**. Issue #175
introduces the intended-placement strip (EQ+comp on simd1, limiter on simd2), and that strip
becomes the standing qualification fixture; these records retire to history at that boundary.

## Links

* Ruling: `docs/rulings/unfused-multiply-add-audit.md`; issue #163 phase 2, owner GO 2026-08-26.
* Audit evidence: `audit/{dense,mutations,exhaustive,conformance}.txt`.
* Baseline: `artifacts/issue163-phase2-wasm-baseline/`; native predecessor
  `artifacts/issue163-phase3/`.
* Runners: `scripts/run-console-benchmark.sh --issue163-phase2`,
  `scripts/run-wasm-console-benchmark.sh --after`, and their preflights.
