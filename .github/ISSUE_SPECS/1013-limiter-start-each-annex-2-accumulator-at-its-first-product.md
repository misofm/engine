# Limiter: start each Annex-2 accumulator at its first product

Source: `docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS-2.md`, verified in `VERIFY-LIMITER-2.md`. **The Amendments sections supersede the body wherever they conflict.** Draft names map to issues: limiter-2-1 = #1013; limiter-2-2 and limiter-2-3 (merged) = #1014.


Limiter round 2, slice 1 of 3 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at
`49f696c7`; every `file:line` below was read on that tree). Evidence:
`docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS-2.md`, sections 2 and 5 item 1. The prototype is
`P_SEED` in `docs/handoffs/effects-2026-09-27/limiter-diagnosis-2-prototypes.patch`. It is evidence
only; do not commit it.

This is #992's contract 2 on its own. #992 is held behind its no-regression gate, because as a whole
it measured +8-9 % natively. This contract, measured alone, is faster on every native row. If this
issue lands, #992 should drop its contract 2 and gate 2.

## Product outcome

`annex2_phases` (`crates/true-peak-limiter/src/lib.rs:1145`) starts each of the four phase
accumulators at `+0.0` and adds the first product (`phase0 = phase0.add(row[0].mul(t0))`, the
`tap!(0, ..)` line). This slice starts each accumulator at the first product.

The detector is about half to two thirds of the limiter. Its bound is not its ports but its chains:
each phase is a frozen chain of dependent adds, and the out-of-order window can overlap only about
1.3 channel-frames of them (diagnosis section 2). Dropping the seed shortens every chain from 12
adds to 11 and removes 4 `vaddps` per channel-frame:

* x86 goes from 133 to 129 instructions per channel-frame at `Simd8` and `Simd4`;
* wasm goes from 48 to 44 `f32x4.add`;
* the roster's "true-peak-limiter f32x4 dual" vector count goes from 910 to 878.

Every block pays the detector, so this also lowers the worst case.

Measured on the prototype. Figures are cycles per lane-sample, the change of the minimum over rounds
against the pristine binary, separate binaries, under the timing lock on cpu 31. They are
descriptive only.

| row | native `Simd8` | native `Simd4` |
|---|---:|---:|
| 64-track kernel rig, +3 dBFS noise, linked | 8.69 → 8.28 (-5 %) | 15.55 → 14.74 (-5 %) |
| ditto, `dual_mono` | 10.02 → 9.59 (-4 %) | 18.43 → 17.59 (-5 %) |
| ditto, quiet tone / ramping dispatch | -5 % / -3 % | -5 % / -3 % |
| console minus `eq_comp_simd1`, in place (hot / tone) | -3 % / -2 % | -4 % / -1 % |

Under V8, through the shipped `host_web.wasm` render export (mean of three Node processes), the
limiter alone moved -3.1 % linked and +1.1 % dual, and the console isolate -1.7 %. That is within
V8's ±2-4 % per-process spread. Natively it was faster on every row in each of the seven runs that
included it.

## Proof (the doc comment must carry it)

This is #992's proof 3. Let `S_k` be the accumulator with the `+0.0` seed after tap `k`, and `T_k`
the one without it.

* `S_0 = +0.0 + a_0` and `T_0 = a_0` differ only when `a_0 = -0.0`, where they give `+0.0` against
  `-0.0`. (Round to nearest: `+0.0 + -0.0 = +0.0`. For any other `a_0`, `+0.0 + a_0 = a_0` exactly,
  NaN included.)
* By induction, `S_k` and `T_k` are equal, or they are zeros of opposite sign. `x + a` equals
  `y + a` whenever `a` is nonzero or NaN, and when `a` is a zero both results are zeros.
* The phases feed only `peak.max(phase.abs())` (`detector_peak`, `:1123`), and `abs` erases the
  sign.

So every peak word is bit-identical, except in NaN payloads. A NaN peak cannot reach state or
output:

* A NaN phase makes the peak NaN whatever its payload.
* The peak goes only to the stack scratch and to `select(p > l, l / p, 1)`, where a NaN `p` fails
  the ordered compare and gives exactly `1.0`.
* The base already leaves the payload to the compiler. LLVM commutes these `fadd`s: the base
  listing has `vaddps ymm0,ymm9,ymm0`.

## Invariants

* **Class A.** Every output word, state payload, report and observation is unchanged, at `f32`,
  `Simd4` and `Simd8`, on every target. The phase values themselves may differ in the sign of a
  zero. They are not state.
* Only `annex2_phases` changes. The uniform body, the per-lane body and the collapsed mono body all
  reach it through `detector_peak` / `detector_chunk` (`:2001`), so all three take it.
* Allocation-free, no `unsafe`, no state-layout change.

## Interface contract

1. `annex2_phases` seeds each accumulator with its first product (`row[p].mul(history.t0)`), then
   takes the eleven remaining `add(mul(..))` steps in today's order.
2. The old form stays under `#[cfg(test)]` as `annex2_phases_seeded`: E1's oracle, and the oracle of
   the new E1b.

## Smallest closable slice

Authorized paths:

* `crates/true-peak-limiter/src/lib.rs` (`annex2_phases`, its doc comment, the tests module);
* `crates/true-peak-limiter/tests/seedless.rs` (new);
* `crates/true-peak-limiter/tests/MUTATIONS.md`;
* this spec.

Steps:

1. **On the base:** write gate 4's scenario and pin its digest.
2. Contracts 1 and 2.
3. The gates, then the evidence.

## Non-goals

* The rest of #992: `l / max(p, l)`, and the link and bypass arms. Measured here, `l / max(p, l)` in
  steady frames was a net loss (diagnosis section 5).
* Any reordering of the summation. That is class B, and diagnosis section 8 item 1 recommends against
  it.
* Any change to the detector's loop shape. Seven shapes were measured, and all lost natively
  (diagnosis section 5).

## Objective gates

1. **E1 stays and E1b is new**, in dev and in release.
   * E1 (`phase_outputs_match_the_frozen_scalar_order`, `:4044`) now tests `annex2_phases_seeded`.
   * E1b compares `detector_peak` with the new seed against the seeded oracle's peak, at `f32`,
     `Simd4` and `Simd8`. Inputs:
     * E1's noise;
     * impulses of `+0.0`, `-0.0`, `+1.0` and `-1.0` at each of the twelve taps;
     * histories of mixed signed zeros;
     * random subnormals;
     * one NaN tap with each of two payloads.
   * Non-NaN peaks are compared by bits, NaN peaks as "both NaN".
   * E2 (`bs1770_annex2_conformance_is_unchanged`, `:4083`) stays green, unchanged.
2. **Existing identity gates stay green**, in dev and in release:
   * `randomized_scenarios_render_exactly_the_unmodified_kernel` (`:7365`), whose reference kernel
     calls `detector_chunk` and therefore also takes the change. So this gate proves the dual/linked
     agreement, not the seed. E1b and gate 4 prove the seed.
   * `a_uniform_cohort_renders_exactly_the_per_lane_path` (`:4988`);
   * `tests/determinism.rs` (D90) and `wasm-gates` G5 in release;
   * `tests/{gain_law,mono_collapse,linked,observation}.rs`;
   * every console workload's 64-block digest (`console-workload` tests).
3. **Wasm identity.** The V8 digests of the shipped artifact equal the base's on the diagnosis
   sessions: `lim`, `limdm`, `bi`, `console` and `nolim`, 300 blocks of seeded noise
   (`limiter-diag-2-harness/scripts/web.mjs digest`).
4. **Scenario pinned on base** (`tests/seedless.rs`). Bank API, 96 blocks at W8, W4 and a scalar
   instance, `Maximum` and `DualMono`, with blocks of:
   * +3 dBFS noise;
   * all-signed-zero input (mixed `+0.0` and `-0.0` per lane);
   * subnormals;
   * one NaN, which exercises the §4.4 reset.

   One SHA-256 per width of every output word and every track's payload per block, recorded on
   `49f696c7`.
5. **Mutations**, each recorded red in `MUTATIONS.md`:
   * M1: drop the first product instead of the seed. E1b goes red.
   * M2: seed phase 3 with phase 0's first product. E1b and D90 go red.
   * M3: start from the second tap's product and add the first at the end (a reorder). E1b goes
     red.
   * Record also, green, the equivalent mutation "seed with `-0.0`". It shows why the proof holds.
6. **Realtime and wasm:**
   * `tests/allocation.rs`;
   * `check-realtime-policy.sh` and `check-lane-policy.sh`;
   * `check-web-audioworklet.sh`: the "true-peak-limiter f32x4 dual" row matches exactly one
     function, its vector count falls, the rule-3 kernel count does not drop, and the render
     callgraph is unchanged.
7. **No-regression timing gate** (descriptive otherwise; freeze the workload before timing).
   * **Native.** The out-of-tree kernel rig of the diagnosis: 64 tracks, `HotLinked`,
     `HotDualMono`, `QuietLinked` and `RampLinked`, at `Simd8` and `Simd4`. Base and change as
     separate binaries, in forward-then-reverse order over three passes, pinned, under the lock.
     Every row's minimum over rounds must be ≤ base.
   * **V8.** Through the shipped `host_web.wasm` render export: three separate Node processes of 10
     rounds each. The mean of the three `lim - bi`, `limdm - bi` and `console - nolim` isolates must
     each be ≤ base + 2 %.
   * **Codegen record.** Quote the detector loop's instruction count (x86, both widths; V8) and
     the roster vector count.

## What the implementer will hit

* **The codegen is unstable** (diagnosis section 4). If gate 7 fails although the loop is 4
  instructions shorter, report it. Do not add compensating changes to the kernel.
* **Keep the reference kernel honest.** `tests::reference_block` (`:6534`) reaches `detector_chunk`,
  so it changes too. That is why E1b and gate 4, not the randomized oracle, are this slice's
  discriminating gates.

## Amendments (Sol verification, 2026-09-27)

Evidence: `docs/handoffs/effects-2026-09-27/VERIFY-LIMITER-2.md` (F1, F9, F10) and
`verify-limiter-2-raw-timings.txt`. **These amendments supersede the body wherever they conflict.**
The slice stands as drafted; the amendments tighten its evidence and its gate.

### A1. Gate 7 builds the change arm from the slice's own commit, with nothing else in the tree

The diagnosis timed every prototype in a tree that also carried its whole scaffold (`LIMDIAG2`
switches, 20 prototypes as dead code). That scaffold alone moves this kernel. The same seed edit,
built inside a scaffolded tree, measured +5 % (`Simd8` `HotLinked`) and +24 % (`Simd8`
`RampLinked`) against the clean base; built clean from the pristine tree it measured -4 % and -3 %.

* The change arm is the slice's commit. The base arm is its parent. Neither carries a diagnostic
  switch, a prototype or any other dead code.
* Record both binaries' SHA-256 in the evidence.

### A2. Expected values, restated from clean builds

Minimum over 4 passes of 4 rounds, separate binaries, under the lock at load 10-12, cycles per
lane-sample, against the pristine binary:

| row | `Simd8` | `Simd4` |
|---|---:|---:|
| `HotDualMono` | 9.89 → 9.65 (-2 %) | 18.21 → 17.63 (-3 %) |
| `HotLinked` | 8.65 → 8.30 (-4 %) | 15.38 → 14.79 (-4 %) |
| `QuietLinked` | 8.63 → 8.18 (-5 %) | 15.37 → 14.86 (-3 %) |
| `RampLinked` | 8.98 → 8.71 (-3 %); another run +0.4 % | 16.04 → 15.59 (-3 %) |

Under V8 (mean of three Node processes, load 8-12) the isolates moved -0.8 % (`lim - bi`), -0.2 %
(`limdm - bi`) and -1.2 % (`console - nolim`). **The slice is neutral in the browser** and is
justified by the native rows and the every-block applicability, not by a browser saving. The
diagnosis's `Simd4` -5 % does not reproduce clean; -3 % does. Its ramping -3 % reproduced in three
clean runs of four (see A3). Two later quiet V8 runs agree: -0.3 to -1.9 %.

### A3. Gate 7's statistic and tolerance, exactly

* **Native statistic.** For each row, the minimum over every round of at least three passes; each
  pass runs each arm as its own process, forward then reverse. Print the load average with each
  pass. A run whose load average exceeds 15 is not evidence and is repeated whole, once, before any
  row is judged.
* **Native criterion.** `HotLinked`, `HotDualMono` and `QuietLinked` must each be below base.
  `RampLinked` must be at most base + 2 %.
  * The seed's `RampLinked` effect measured -3, -3, -2 and +0.4 % (`Simd8`) in four clean runs.
  * The pristine binary's own `RampLinked` minimum varied by 1.8 % between runs on this host.
  * So the draft's strict "≤ base" on that row is a coin flip, not a gate. The stationary rows moved
    -2 to -5 % in every run, and the strict criterion stays on them.
* **V8.** The mean over three separate Node processes of each process's median isolate (the
  diagnosis's `web.mjs time`), each isolate ≤ base + 2 %.

### A4. E1b also covers a NaN made inside a chain

Add to E1b's inputs:

* `+inf` and `-inf` taps;
* a history whose products reach `+inf` and `-inf` within one phase, so the chain creates a NaN
  mid-sum rather than receiving one.

Compare as the body says: non-NaN peaks by bits, NaN peaks as "both NaN".

**Verified:** E1, the frozen-order bit test, passes unchanged against the seedless
`annex2_phases`. Its noise never produces a signed-zero first product. Keep E1 on
`annex2_phases_seeded` as contract 2 says, so that it remains a statement of the frozen order. E1b's
signed-zero rows are the only rows that see the sign difference. The whole crate suite (50 tests)
passes in release with the seedless form.

### A5. Corrections

* `tests::reference_block` is at `:6499`; `:6534` is `reference_block_uniform`. Both reach
  `detector_chunk`, so the randomized oracle still cannot see this change.
* Gate 4's digest may be recorded on the slice's parent (`220c5db5` today). The limiter crate is
  identical there and on `49f696c7`. Record `git rev-parse HEAD:crates/true-peak-limiter` beside the
  digest.
* The roster count (910 → 878) and the loop count (133 → 129 at `Simd8`) reproduce exactly.
