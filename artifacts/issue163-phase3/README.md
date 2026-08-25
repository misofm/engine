# Issue #163 phase 3 — interleave independent recurrences — measurement record

Subject: the phase-4 tree (`d0b5cfb`) with the parametric EQ rendering both channels' four-section
cascades in one interleaved frame loop. Class A: no rendered bit moves.

## Runner invocations against this phase

| attempt | files | status | workloads launched | rounds |
|---|---|---|---|---|
| 1 | `console-benchmark.attempt-1-refused.*` | FAIL `precondition_loadavg_above_ceiling` | 0 | 0 |
| 2 | `console-benchmark.{raw,accepted,disposition}` | PASS `complete`, `measurement_control: controlled` | 3 | 2 |

Attempt 1 refused before launching anything — the one-minute load average was still decaying from
the candidate's own build. It timed nothing (`raw_sha256: null`), so freeing the canonical names for
attempt 2 discarded no measurement. It is kept so the invocation count is on the record.

Attempt 2: `candidate_commit 7a856ec`, cpu affinity 15, SMT sibling idle, cooldown honoured, two
measured rounds after one warmup, `raw` and `accepted` byte-identical. Round-to-round spread is at
most 1.02% on any row.

## Result, against the phase-4 record (`artifacts/issue163-phase4`)

`Simd8`, 48 kHz, 128-frame quantum, 1 000 observations, p50 µs/block, minimum of two rounds.

| row | tracks | phase 4 | phase 3 | ratio | digest |
|---|---|---|---|---|---|
| console, `eq+compressor`, real fixture | 64 | 108.17 | **91.03** | **1.188x** | unchanged |
| console, `eq+compressor`, synthetic | 128 | 220.53 | **184.21** | 1.197x | unchanged |
| decomposition: `eq` | 64 | 59.41 | 41.55 | 1.430x | unchanged |
| decomposition: `compressor` | 64 | 70.84 | 71.35 | 0.993x | unchanged |
| decomposition: `builtins` | 64 | 21.85 | 21.69 | 1.007x | unchanged |
| decomposition: `identity` (control) | 64 | 21.84 | 21.66 | 1.008x | unchanged |
| idle (silence input) | 64 | 35.67 | 35.16 | 1.015x | unchanged |
| `parametric-eq-nine-track` fixture | 9 | 13.36 | 8.18 | 1.634x | unchanged |

**64-track console: 108.17 → 91.03 µs/block, −15.8%.** 128-track: 220.53 → 184.21, −16.5%.

## Reading it

* **The EQ increment over the dispatch-only control** is the quantity phase 3 acts on:
  59.41 − 21.84 = **37.57 µs** becomes 41.55 − 21.66 = **19.89 µs**, a **1.889x** reduction of
  −17.68 µs.
* **The decomposition is additive.** The console block fell 17.14 µs and the EQ row fell 17.86 µs;
  they agree to 0.7 µs, which is inside the 1% round spread.
* **The controls held.** `identity` (a strip with no effect at all) and `builtins` moved +0.8% and
  +0.7%, which is this host's drift floor, and `compressor` — untouched by phase 3 — moved −0.7%.
  Nothing outside the EQ moved.
* **The isolated kernel gained 2.45x; the EQ row gained 1.889x.** The difference is the part of the
  EQ increment that is not the recurrence: the planar/AoSoA transpose round trip, the phase-4
  all-`+0.0` scan, the §4.4 boundary check, and the per-lane automation offsets. Phase 3 touches
  none of them.
* **The plan projected ~1.48x on the EQ share, about −10% of the block.** The delivered 1.889x on
  the EQ increment and −15.8% of the block are better, because the plan's mechanism was one axis
  (independent banks) and the shipped kernel takes two (channels *and* cascade depth).
* **Idle is unchanged**, 35.67 → 35.16 (−1.4%, at the drift floor). The phase-4 earned-fixed-point
  path still engages on every timed idle block: it is evaluated before the interleave and its
  claim is still per bank, still observed on a block that was actually run, and still compared over
  the same integrator words.

## Class A

Every one of the nine workloads above reports the **same `output_sha256` as phase 4**, which is the
phase-1 digest. Alongside that:

* gate G2 pins `svf_cascade_interleaved` against a chain of `svf_block` calls — the audio and every
  integrator word — at all three widths and all three legal depths, with three recorded red
  mutations (`crates/miso-engine-lane/tests/MUTATIONS.md`);
* `interleave_identity` in the EQ drives **both arms of the same entry point** over the frozen E9
  corpus signals at all three widths, cold and with a subnormal-adjacent seeded state. It exists
  because the E9 corpus itself renders one channel at a time and therefore never reaches the new
  path — the pinned digests alone would not have covered it;
* wasm gates: 133 cases, 331 comparisons, 0 mismatches, all legs;
* graph fresh-process determinism 100/100; the full release workspace suite and all 88 `sweep.sh`
  rows green (the one standing red, host-core `observation_cost_classes`, is #159 and reproduces
  3/3 on the clean parent).

The schedule is untouched. No bank ops were fused, no unit was reordered, and no bank window
changed, so the #169/#170 window-hold invariant is preserved by construction rather than by
argument.

## What is not measured

* **wasm timing.** The shipped `simd128` artifact does carry the interleaved body, unrolled at
  `S = 2, D = 2` — its single parametric-EQ kernel goes from 32 to 74 `f32x4.{mul,add,sub,div}`
  operations with scalar arithmetic still at 0, and the loop body holds four unrolled section-steps
  (4 `f32x4.sub`, 12 `f32x4.mul`). But no *timing* number exists for it. The instrument that could
  produce one is the phase-0b kernel-timing harness, and its cases come from the frozen G5 corpus;
  adding an interleaved case there is a re-pin, which phase 3 is forbidden from doing. On wasm a
  fused multiply-add is ~54 instructions (softfma), so that kernel is throughput-bound where the
  native one is latency-bound, and the native ratios above should **not** be read across to it in
  either direction.
* **`Simd4` and `Scalar` on the console.** The console bench arm runs `Simd8` only. The isolated
  sweep measures all three (`Simd4` 2.653x, `Scalar` 2.092x), but a kernel ratio is not a block
  ratio, and no `Simd4` console number exists to compose them into.
* **Cross-bank fusion.** Measured as a bounded null at the production native backend — 1.10x above
  what shipped — and recorded in `docs/rulings/cross-bank-interleave.md`. It is measured, not
  assumed, but it is measured in isolation: no console number exists for it either.
* **Other bank kernels.** Phase 3 changed the EQ only. The compressor's banks carry recurrences of
  their own and its row is 71.35 µs, now the largest single term in the block; whether the same two
  axes exist there is untested.
