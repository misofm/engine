# Compressor round 2 — the staged idle body, measured

One class-A change to `crates/miso-engine-compressor/src/kernel.rs` and to nothing else, measured
as a **paired pair**: the same tooling, the same fixtures, the same runner and the same host, with
the base commit's kernel in one arm and the staged kernel in the other.

| question | answer |
|---|---|
| Did any rendered bit move? | **No.** All 11 native rows and all 33 wasm (row, leg) digests are byte-identical between the arms, and every wasm record still reports `digest_identity: all_legs_identical`. |
| What does the strip cost now? | **−8.7% native** on the intended console strip (124.17 → 113.32 µs), **−15.6%** on the compressor-only row. |
| And on wasm? | **Also faster**, unlike round 1's null: −11.1% on the compressor-only row under `simd128`, −5.2% on the console strip. |
| Which lane width gains most? | `Simd4`: −30.2% on the compressor-only row of the wasm runner's native leg. |

**The authority does not move.** `artifacts/issue175/` remains the standing qualification record for
*what* the strip renders. This directory measures *what it costs*, and the digest equality is the
round's null detector: a class-A change that moved a digest would not be a faster compressor, it
would be a different one.

## Arms

| arm | commit | directory |
|---|---|---|
| baseline | `65f960d` — `eae51a0` plus only the runner-arm commit | `artifacts/round2-comp-baseline/` |
| candidate | `round2-comp2` | `artifacts/round2-comp/` |

Both arms `measurement_control: controlled`, `render_total_forbidden_operations: 0`, 48 kHz,
128-frame quantum, 1 000 observations, p50 µs/block, minimum of the two measured rounds.

## Attempts

| arm | leg | attempts | refusals kept |
|---|---|---|---|
| candidate | native | 5 | 4 × `precondition_loadavg_above_ceiling` |
| candidate | wasm | 2 | 1 × same |
| baseline | native | 1 | none |
| baseline | wasm | 2 | 1 × same |

Every refusal launched nothing (`raw_sha256: null`) and is kept as
`*.attempt-N-refused.{disposition.json,stderr.log}`. The machine was shared with other agents for
the whole session, and the ceiling is a flat 0.50 one-minute load average.

One mechanical note for the next round: **a kept refusal is itself a dirty tree**, and the runner
refuses a dirty tree, so a naive retry wedges on attempt 2 with `requires a clean committed
candidate`. The residue has to be committed between attempts, not merely renamed.

## What changed

`idle_frames_staged` visits an idle segment three times instead of once — steps 1 to 5, then the
ballistic recurrence alone, then steps 7 and 8 — with the detector taps of the whole segment
pre-gathered into a frame-major scratch, two contiguous strided runs per lane around the ring wrap.
Per lane and per sample the operation order is unchanged; only the interleaving across frames
moves, which is the argument class of #163 phase 3's SVF interleave. `frames_loop` stays the
general body and the fallback for any segment whose lanes tap a row the segment writes first
(`D < len`), and remains the only body that may ramp.

## Kernel-level A/B — descriptive, `examples/lane_sample_timing`

| | base | staged | Δ |
|---|---|---|---|
| bank `W8` | 2.612 / 2.612 / 2.609 | **1.927 / 1.925 / 1.930** | **−26.2%** |
| scalar | 10.996 / 10.976 / 10.957 | **7.150 / 7.060 / 7.064** | **−35.7%** |

Strip-mining the passes to 16, 32 or 64 frames was measured and is **slower** at every size
(bank 1.933 / 1.936 / 1.938 against 1.927 unstripped); at 128 frames the scratch already sits in L1.

## The measured table

### native `Simd8` (console runner)

| row | baseline | candidate | Δ | Δ% |
|---|---|---|---|---|
| **compressor only** | 72.448 | **61.146** | −11.302 | **−15.60%** |
| **eq+compressor — intended** | 82.015 | **70.654** | −11.361 | **−13.85%** |
| **eq+compressor — legacy dynamic rack** | 85.993 | **74.842** | −11.151 | **−12.97%** |
| **console — the intended strip** | 124.166 | **113.316** | −10.850 | **−8.74%** |
| console, 128 tracks | 247.289 | 226.049 | −21.240 | −8.59% |
| nine-track ragged strip | 24.947 | 22.603 | −2.344 | −9.40% |
| idle (silence) — *control* | 39.014 | 39.304 | +0.290 | +0.74% |
| eq only, 64 — *control* | 37.902 | 37.762 | −0.140 | −0.37% |
| eq only, 9 — *control* | 6.101 | 6.132 | +0.031 | +0.51% |
| builtins only — *control* | 22.683 | 22.773 | +0.090 | +0.40% |
| identity — *control* | 21.932 | 22.022 | +0.090 | +0.41% |

**Every row that carries a live compressor moved; every control is inside ±0.75%.** The idle row is
the sharpest control of the set: it *does* carry a compressor, but renders silence, so #182's
silence fixed point skips the kernel entirely and it correctly does not move.

Cycle columns for the compressor-only row: **24.12 → 20.32 cycles per lane-sample**, and
percent-of-floor **22.8% → 27.0%** against the same 5.507-cycle derived floor.

### the wasm runner, all three legs

| row | leg | baseline | candidate | Δ% |
|---|---|---|---|---|
| compressor only | `wasm_simd128` | 128.464 | **114.238** | **−11.07%** |
| compressor only | `native_simd8` | 74.191 | 62.860 | −15.27% |
| compressor only | `native_simd4` | 219.116 | **153.011** | **−30.17%** |
| eq+compressor — intended | `wasm_simd128` | 150.025 | 136.029 | −9.33% |
| console strip, 64 | `wasm_simd128` | 275.774 | 261.518 | −5.17% |
| console strip, 128 | `wasm_simd128` | 550.314 | 525.440 | −4.52% |
| nine-track strip | `wasm_simd128` | 46.999 | 44.955 | −4.35% |
| eq only, 64 — *control* | `wasm_simd128` | 78.539 | 79.181 | +0.82% |
| builtins — *control* | `wasm_simd128` | 47.119 | 47.320 | +0.43% |
| identity — *control* | `wasm_simd128` | 47.280 | 47.160 | −0.25% |
| idle (silence) — *control* | `wasm_simd128` | 77.537 | 77.898 | +0.47% |

Round 1's wasm result was a ruled null. This one is not: the per-access bounds check the pre-gather
removes is a real branch on wasm, and `Simd4` — the width wasm actually runs — gains most of all.

## Files

* `console-benchmark.{raw,accepted}.jsonl`, `.disposition.json`, `.stderr.log`,
  `.core-clock.csv` — the native arm.
* `wasm-console-benchmark.*` — the wasm arm.
* `*.attempt-N-refused.*` — refusals that launched nothing, kept.
* `artifacts/round2-comp-baseline/` — the same set for the baseline arm.
