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

## Attempt 1 evidence

Terra, 2026-09-28. Code commit `03a5afe3` on `codex/1013-limiter-seedless-accumulators`; its parent
is `52ad1460`, the batch head, whose limiter crate (`99fed5cb`) is the one on `220c5db5`. Nothing
pushed. **Gate 7 passes on clean builds**, natively on every row and in V8 on every isolate.

### What changed

* `annex2_phases` starts `phase0..3` at `row[p].mul(history.t0)` and then takes `tap!(1..=11)` as
  before, written exactly as the verification's `variants.py` `seed` arm. Its doc comment carries
  the proof (tap 0, the induction, the one reader, the inert NaN peak). The doc comments of
  `detector_peak` and `History` that said "twelve steps from `+0.0`" were corrected.
* `annex2_phases_seeded` (`#[cfg(test)]`) is the old body verbatim. E1 now tests it.
* E1b (`seedless_peaks_match_the_seeded_order_at_every_width`), `tests/seedless.rs` (gate 4) and a
  `MUTATIONS.md` section. No other file, no state-layout change, no `unsafe`.
* **Beyond the brief, for the verifier to judge.**
  * E1b also checks every **phase**, not only the peak: the same word, zeros of opposite sign, or
    both NaN. That is the proof's induction run as a test.
  * E1b adds four designed histories, one per phase, whose twelve products are all `-0.0`, so the
    seed is visible at every width. A random signed-zero mix gets there once in 4096 windows per
    phase.
  * Gate 4 adds a collapsed-body arm (`process_bank_mono`, Maximum) beside the two dual arms, runs
    eight scalar instances (tracks 0-7) rather than one, and also folds every report and the
    resident observation.

### Gates 1-6

| gate | result |
|---|---|
| 1. E1, E1b, E2 | Green in dev and in release. E1b covers 41 streams, 13,942 samples, at `f32`, `Simd4` and `Simd8`: E1's noise, `±0.0`/`±1.0` impulses at all twelve taps on either zero, the four all-`-0.0`-product histories, 4096 mixed signed zeros, 4096 subnormals, NaN taps `0x7FC01234` and `0xFFA05A5A`, `±inf` taps, and `+inf`/`-inf` pairs at every spacing in either order. Non-vacuity, asserted: 9 phases flip a zero's sign at each width (at least 4 are required), 86 NaN peaks at each width, and 244 windows whose products are all non-NaN yet whose seeded chain ends NaN (the NaN is made at an add). |
| 2. identity | Green in dev and in release: the limiter crate (52 passed, 1 ignored, including `randomized_scenarios_render_exactly_the_unmodified_kernel`, `a_uniform_cohort_renders_exactly_the_per_lane_path`, D90, `gain_law`, `mono_collapse`, `linked`, `observation`); `host-core` (187 passed, including `limiter_linked_session` at `Simd8`, `Simd4` and `Scalar`); `console-workload` (57 passed); `run-wasm-gates.sh` G5 (142 cases, 0 mismatches on every leg). Also run: the #990 differential over 150 seeds of 96 blocks gives the same combined digest on both binaries (W8 `c8a711d827569736`, W4 `ff492b1bf5fc6edd`, W1 `36f79d5a6ada2623`), and the rig's in-place console digests (`Simd8`/`Simd4`, hot and tone, console and `eq_comp_simd1`) are identical. |
| 3. wasm identity | V8 digests of the two shipped modules, 300 blocks, identical: `lim` `dea70183`, `limdm` `3ff478a2`, `bi` `b42bcd42`, `console` `a2594b72`, `nolim` `3c42efe0`. |
| 4. gate-4 pin | Recorded on `52ad1460` (limiter crate tree `99fed5cb`, `src/lib.rs` SHA-256 `5ca9ba88…`) in dev and in release, with identical digests: W8 `4b57d4da…`, W4 `c5782640…`, scalar `ec135dac…`. Green on `03a5afe3` in dev and in release. Every arm asserts it limited (`deepest > 0.1`), put `-0.0` and subnormal words out, and was zeroed by the §4.4 reset in exactly blocks 60-63 (the NaN block and the emptied line), or never for a scalar track without the NaN. The reset is read from the audio because the limiter's §4.4 counter is not in its `ProcessReport`. |
| 5. mutations | M1, M2 and M3 red on E1b (and M1/M2 on D90, E2, E5, `linked.rs`, gate 4 and the session test at every width; M3 on `linked.rs` and gate 4). The `-0.0` seed is green everywhere. Reverting the slice (`+0.0` seed) is red on E1b alone, on "0 phases flipped a zero's sign", so E1b sees the seed. With E1b's phase leg made non-failing, its peak leg alone is still red under M1, M2 and M3. Details in `tests/MUTATIONS.md`. |
| 6. realtime, wasm | `tests/allocation.rs` green in dev and in release. `check-realtime-policy.sh` (57 regions), `check-lane-policy.sh`, `check-env-vocabulary.sh` and the analyser's `--self-test` ok. `check-web-audioworklet.sh` ok on the commit. The batch pin `8934cdd9…` matches neither base (`a383a188…`) nor change, so the full gate was run in the scratch worktree with the change module's digest substituted into the pin file, which was then restored. "true-peak-limiter f32x4 dual" matches one function and falls **910 → 878**; the collapsed kernel falls 440 → 424; rule 3 counts 15 kernels on both (minimum 11); the render callgraph line is identical (closure 8, traps 5, same owner). `run-wasm-gates.sh` ok, including detector residency and the V8 spill gate. |

Also green: `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets -- -D warnings`.

### Gate 7: no-regression timing (A1, A3)

**Builds.** Each arm was built from a scratch worktree detached at its commit, whose only other
content was the untracked out-of-tree rig (`limiter-diag-2-harness` from the diagnosis patch, with
the workspace `Cargo.lock` copied in). Every native build started from an empty target directory.
The wasm arms are `scripts/build-web-audioworklet.sh --module-only`, each in a fresh target.
`CARGO_INCREMENTAL=0` throughout.

| arm | commit | native rig `ld2` SHA-256 | `host_web.wasm` SHA-256 |
|---|---|---|---|
| base | `52ad1460` | `6ea786ef36862895a511b33d2911cdf3adcc911f694623ae8c4e851eaac77bc1` (rebuilt: byte-identical) | `a383a188dee709468b4a8cd75ca765511aa873fd365e28dd4df9476849d5bdc8` |
| change | `03a5afe3` | `6cb58acbd009b19fa1d7d6ade065dda56117b16f00888c6fe4958f985f1c7cd0` | `4fdcc82ad6b7c9387983e33fe2dc3df648e5fb083a15c4bd9549a54af5479d7d` (`run-wasm-gates.sh`'s own build: the same) |

**Native.** The kernel rig: 64 tracks, `kbench`, 4 rounds of 800 blocks per shape. Each run is four
passes, each arm its own process, forward then reverse, under the lock, pinned with `taskset -c 31`.
The statistic is the minimum over all 16 rounds, in cycles per lane-sample. Two runs, both under
load 15:

| row | run 1 (load 4.2-4.3): base → change | run 2 (load 3.6-3.8): base → change |
|---|---:|---:|
| `Simd8` `HotDualMono` | 9.977 → 9.701 (**-2.8 %**) | 9.968 → 9.692 (-2.8 %) |
| `Simd8` `HotLinked` | 8.670 → 8.292 (**-4.4 %**) | 8.694 → 8.248 (-5.1 %) |
| `Simd8` `QuietLinked` | 8.668 → 8.271 (**-4.6 %**) | 8.631 → 8.268 (-4.2 %) |
| `Simd8` `RampLinked` | 8.978 → 8.792 (**-2.1 %**) | 8.947 → 8.700 (-2.8 %) |
| `Simd4` `HotDualMono` | 18.134 → 17.594 (**-3.0 %**) | 18.260 → 17.576 (-3.7 %) |
| `Simd4` `HotLinked` | 15.285 → 14.808 (**-3.1 %**) | 15.334 → 14.808 (-3.4 %) |
| `Simd4` `QuietLinked` | 15.297 → 14.841 (**-3.0 %**) | 15.436 → 14.840 (-3.9 %) |
| `Simd4` `RampLinked` | 16.066 → 15.726 (**-2.1 %**) | 16.180 → 15.692 (-3.0 %) |

The stationary rows are below base in both runs. `RampLinked` is also below base, which is stricter
than A3's +2 % allowance. The medians move the same way (-1 to -5 %). The values match A2.

**V8.** The shipped modules through `miso_engine_web_v1_render`. Each run is three separate Node
22.23.2 processes of 10 rounds of 800 blocks, both arms in each, pinned to cpu 31, under the lock.
The statistic is the mean over the processes of each process's median isolate, in cycles per
lane-sample, with the per-process values in brackets:

| isolate | run 1 (load 3.4-3.6): base → change | run 2 (load 2.5-2.7, arms reversed) |
|---|---:|---:|
| `lim - bi` | 22.14 [22.15 22.13 22.13] → 21.95 [21.84 21.89 22.13] (**-0.8 %**) | 22.11 → 21.96 (-0.7 %) |
| `limdm - bi` | 24.33 [24.37 24.23 24.38] → 24.15 [24.24 24.12 24.08] (**-0.7 %**) | 24.29 → 24.14 (-0.6 %) |
| `console - nolim` | 22.60 [22.67 22.28 22.84] → 21.88 [21.88 21.95 21.82] (**-3.2 %**) | 22.37 → 22.18 (-0.8 %) |

Each isolate is below base, inside the +2 % bound. As A2 says, the slice is neutral in the browser.

**Codegen record.**

| | base | change |
|---|---:|---:|
| x86 detector loop, `Simd8` (8 instantiations in `process_block`) | 133 instructions (4 at 133, 2 at 135, 2 at 136) | **129** (7 at 129, 1 at 126) |
| x86 detector loop, `Simd4` | 133 (6 at 133, 1 at 134, 1 at 136) | 125-128 |
| x86 `vaddps` per channel-frame | 48, plus one `vxorps` zero | 44, no `vxorps` |
| V8 TurboFan detector loop (`LimiterCore<f32x4>::process_block`, two shapes) | 200 / 198 instructions, 81 vector memory moves, 48 `vaddps` | **196 / 194**, 80, 44 |
| roster "true-peak-limiter f32x4 dual" | 910 | **878** |

### Not done, and why

* `scripts/run-console-benchmark.sh` was not run, as instructed.
* No isolated detector benchmark (`detbench`). Gate 7 does not ask for one.

### For the verifier

* Reproduce gate 7 by applying the diagnosis patch's `limiter-diag-2-harness/` only, into a
  worktree of each commit, then `cargo build --release --features console --bin ld2`,
  `LD2_SHAPES=HotLinked,HotDualMono,QuietLinked,RampLinked ld2 kbench NAME 4 800` per pass, and
  `web.mjs time` from `verify-limiter-2-harness.patch`. One native run takes 16 s under the lock
  and one V8 run 50 s.
* Gate 4's pin can be re-checked on the base with
  `git checkout 52ad1460 -- crates/true-peak-limiter/src` on top of `03a5afe3`, which leaves the
  test file in place.
* E1b's first failure under M1-M3 is its phase leg; the peak leg alone was checked separately
  (gate 5 row).
* The code commit was made in the scratch worktree first and timed from there. The branch was
  fast-forwarded to it only after gate 7 passed, so the timed change arm is `03a5afe3` itself.
