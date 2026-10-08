# Remove the data-dependent cost of the near-top tail walk

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-08 by the decision-15 root coordinator's ruling D1 (c) on *Cache design bounds across
preparations within a stated preparation budget* (#1457, Amendment 4). Ordered in stream G after
#1457 and before *State a fixed input section's decay, gains and flush stall* (#1465). Code anchors
are those #1457 attempt 3 leaves on `codex/d15-stream-g2`; re-verify every anchor at start.

## Product outcome

A design's tail walk costs about the same per frame whatever its cutoffs. #1457 defines its
frame-equivalent from the slowest frame class it measured, so when that class is no slower than the
others, the frame-equivalent, and with it the stated worst-case preparation cost of the design
bounds, comes down. Every reported bound stays sound.

## Context

- **The slow frame class (#1457 attempt 2 verdict MJ1, attempt 3 calibration).** Long near-top
  walks (`math::tail::fixed_cascade_within`, almost all majorant-pass frames) cost about 12-15 ns
  a frame on the CI-class stand-in (AMD EPYC 7313P, x86-64-v3, release, one pinned core), except in
  a band where the HPF is at about 0.73-0.87 of the maximum cutoff into the LPF at the maximum:
  there a frame costs 21-23.2 ns, at every launch rate and trim. #1457 Amendment 4 defines the
  frame-equivalent as 23.5 ns (that class, rounded up), so the budget of 1,510,000
  frame-equivalents is 35.5 ms, and the measured worst case is about 34.9 ms of design-bound work
  (the band families of `crates/builtins/examples/input_bound_budget.rs`). Without the band the
  slowest class would be about 15 ns a frame.
- **Likely cause, not proven.** On an instrumented copy (#1457 attempt 2 verdict,
  `docs/handoffs/decision-15-2026-10-05/verdicts/stream-g2/1457-attempt2.md`, which states the
  figures of its `probe-subnormal48.log`), the first section's state or its
  error radius is subnormal on about 449,000 of about 450,000 majorant frames in the band, and on
  fewer than 50 outside it (12 kHz and the top cutoff at 48 kHz). Presence alone does not predict
  every point: a 17.28 kHz HPF at 48 kHz was subnormal throughout but ran at 12.9 ns a frame.
  `perf` counters are not available to the workstation's user (`perf_event_paranoid` = 4).
- **Why it matters beyond this box.** If the cost is subnormal operands, it depends on the CPU: a
  core with a larger subnormal penalty than Zen 3 is slower still, and the browser runs the same
  arithmetic (Wasm has IEEE subnormals). The bounds are computed under `lane::CanonicalFpEnv`
  (round to nearest, full subnormals) because the rounding analysis assumes it, so a hardware
  flush-to-zero mode is not a fix; any treatment belongs in the analysis.

## Scope

1. **Diagnose first, by measurement.** Find what makes a band frame cost about 1.7 times a
   typical one. For the band and for designs outside it, at each launch rate: count the subnormal
   operands per frame of each pass (which quantity: a section's state majorant, its error radius,
   or another), and run a flush-to-zero A/B of the same walk (measurement only, never shipped) and
   record the ns per frame both ways. If the cause is not subnormal operands, stop and report the
   measured cause to root with the evidence; do not change the analysis.
2. **If confirmed: a certified flush in the analysis.** Fix a stated tiny bound `tau` and treat
   each quantity that the walk propagates by its kind, so that every propagated quantity stays an
   upper bound of what it bounds:
   - **Non-negative radii and majorants** (the error radius `Majorants::first_error`, the later
     sections' majorants `Majorants::later`, the `Deviation` components): a component below `tau`
     is rounded **up** to `tau`, never down and never to zero. A larger non-negative majorant is
     still a majorant under the non-negative propagation.
   - **The first section's signed state** (`Majorants::first`, propagated by the section's signed
     matrix `A`): rounding up is not defined for it and a larger value bounds nothing. When it is
     below `tau`, it is flushed to **zero**, and an upper bound of its `V`-norm magnitude is added
     to `first_error` in the same step (rounded up), so the radius still covers the exact state.
   Prove it in the derivation: the per-quantity treatment above keeps every step an upper bound,
   and the closing terms (the suffix sums, the remainder, `p_star`, the rest bound) stay sound.
   The proof must state that the walk's rounding model (`STEP_UP`'s relative bound on each
   summand, and `nu = 8 u ||R||` for the first state's step error) holds only when no result
   underflows: a subnormal result has an absolute error and can round to zero. So the proof must
   do one of these, and say which:
   - keep `tau`, and every product and sum that a floored or flushed quantity enters, in the
     normal range of `f64`, and show it;
   - or carry an explicit absolute underflow term (at most one half-ulp of the smallest subnormal,
     `2^-1075`, per rounding) in the bound, and show it is covered.
   State `tau` and why its contribution cannot keep the walk from finishing (a floor at `tau` never
   decays: the remainder test and the falling test must still pass, so `tau`'s share of each, and
   that of the radius added by a flush, must be shown far below its threshold).
3. **Recalibrate and restate.** Rerun #1457's calibration (`input_bound_budget calibrate`, one
   invocation) and restate the frame-equivalent and `INPUT_BOUND_SECTION_CHARGE` from it; rerun
   #1457's gates 2 and 8 with #1457 Amendment 3's commands; restate the worst case (down) with its
   spread and the per-frame cause. The slowest frame class is restated from a robust statistic,
   never from the maximum of single-shot samples (one noisy sample can move that either way: #1457
   attempt 3 verdict n3): each point's ns per frame is the median of repeated runs within the one
   invocation, reported with its spread (minimum to maximum), and the class's value is confirmed
   against gate 2's band families (their design work divided by their charged frame-equivalents).
   Changing the calibration's statistic in `input_bound_budget.rs` is in scope.

## Objective gates

1. **Bounds identical or soundly larger.** Every bound of every family in
   `input_bound_budget.rs` (the band families included) and every bound `tail_contract` checks is
   identical, or larger field by field with the reason stated; `tail_contract`'s gates stay green.
   A smaller bound anywhere fails the gate.
2. **Diagnosis evidence.** The subnormal counts and the flush-to-zero A/B are recorded in the
   attempt record, per rate, for the band and outside it.
3. **Timing.** The calibration's frame classes are recorded before and after on the same box (one
   invocation each); the slowest class's ns per frame and the stated worst case come down, and the
   frame-equivalent is restated from the new slowest class's median with its spread (Scope item
   3), not from a single-shot maximum. #1457's gate 2 runs once (one warmup,
   two measured rounds, no retry, load average recorded before and after).
4. **#1457's gate 8.** Every 64-track console document is exact at every launch rate under the
   restated charge.
5. **The flush is sound per quantity.** Math-level tests show:
   - a non-negative radius or majorant below `tau` propagates as `tau`: red when its flush rounds
     down or to zero;
   - the first section's signed state below `tau` is flushed to zero and its magnitude enters
     `first_error`: red when the state is flushed without that addition, or rounded up to `tau`
     instead;
   - the derivation's normal-range condition (or its underflow term) holds on the band designs
     of gate 1.
   Each red result is a recorded mutation run.

## Non-goals

- Changing `INPUT_BOUND_BUDGET_FRAMES` (a root ruling). Tightening any bound (#1433, #1468). Any
  render-side change. Shipping a hardware flush-to-zero mode in the bound computation.

## Authorized paths

`crates/math/src/tail.rs` (the walk), `crates/builtins/src/tail.rs` (the frame-equivalent and the
section charge, their docs), `crates/builtins/examples/input_bound_budget.rs`,
`crates/builtins/tests/tail_contract.rs`, `docs/derivations/1329-input-section-tail-and-rest.md`
(the walk's derivation), this spec and `STREAMS.md`. Anything else: stop and report.

## Dependencies

- #1457 (the budget, the frame-equivalent and the committed gate-2 workload).
- Hot files: `crates/math/src/tail.rs`, `crates/builtins/src/tail.rs` and
  `crates/builtins/tests/tail_contract.rs` after #1457 and before #1465 (`STREAMS.md`).

## Attempt record

### Attempt 1 (2026-10-08, implementer; branch `codex/d15-stream-g3`)

Box: the workstation that stood in for the CI-class runner in #1457 (AMD EPYC 7313P, x86-64-v3),
release, `taskset -c 7`. Other agents build on this box; the load averages are recorded and no
timing was retried.

**Result.** Scope items 1 and 2 are done; the cause is subnormal operands, and the walk now has a
certified flush. Scope item 3 was first **blocked on a root ruling** (resolved: see "Root's ruling"
and "Completion" at the end of this attempt): the slowest frame class is no longer a
near-top walk, so the calibration's frame classes (near-top walks only, as the spec says) give a
frame-equivalent (14.0 ns) that gate 2's typical and cheap two-section families exceed (up to
15.66 ns per frame-equivalent consumed). The committed constants stay at 23.5 ns and 290 (still an
over-charge, so the stated 35.5 ms budget remains a true bound); see "Open for root".

#### Scope 1: diagnosis (gate 2 evidence)

An instrumented copy (never committed) counted, per majorant frame, the subnormal value of each
quantity after the step, and timed each design 7 times interleaved, normal and with MXCSR FTZ+DAZ
set around `fixed_cascade_within` only (median, min-max, ns per walked frame). +24 dB, LPF at the
maximum, the HPF at the stated fraction of the maximum. The bounds were identical both ways.

| rate | HPF | frames | subnormal frames: first state / error radius | normal ns | FTZ ns |
|---|---|---|---|---|---|
| 44.1k | 0.50 | 456,194 | 455,388 / 41 | 12.92 (12.85-13.17) | 12.23 |
| 44.1k | 0.74 | 453,378 | 452,135 / 452,178 | 16.39 (15.79-16.59) | 12.21 |
| 44.1k | 0.778 (band) | 453,122 | 75 / 451,726 (state exactly 0 on 451,599) | 22.59 (22.03-23.02) | 12.18 |
| 44.1k | 0.87 (band) | 452,867 | 130 / 450,496 (state 0 on 450,281) | 22.73 (21.97-23.95) | 12.20 |
| 44.1k | 0.90 | 452,868 | 449,677 / 449,787 | 13.00 (12.92-13.07) | 12.26 |
| 44.1k | top - 1 | 528,640 | 0 / 0 | 12.20 (12.15-12.31) | 12.31 |
| 48k | 0.50 (12 kHz) | 453,634 | 42 / 41 (state 0 after) | 12.18 (12.13-12.25) | 12.18 |
| 48k | 0.778 (band) | 450,818 | 75 / 449,422 (state 0 on 449,295) | 22.56 (22.08-23.82) | 12.24 |
| 48k | 0.778, 0 dB | 398,082 | 75 / 396,686 | 22.80 (21.94-23.43) | 12.24 |
| 48k | 0.80 | 450,818 | 449,215 / 449,272 | 16.37 (15.87-17.98) | 12.24 |
| 48k | 17.28 kHz | 451,074 | 449,913 / 449,954 | 16.47 (15.74-18.30) | 12.28 |
| 48k | top - 1 | 525,520 | 0 / 0 | 12.21 (12.18-12.27) | 12.21 |
| 88.2k | 0.778 (band) | 453,122 | 75 / 451,726 | 22.62 (22.02-25.79) | 12.24 |
| 88.2k | 0.87 (band) | 452,867 | 130 / 450,496 | 22.83 (22.05-23.62) | 12.16 |
| 88.2k | 0.74 | 453,378 | 452,135 / 452,178 | 15.95 (15.84-17.37) | 12.21 |
| 88.2k | top - 1 | 528,640 | 0 / 0 | 12.17 (12.12-12.38) | 12.20 |
| 96k | 0.778 (band) | 450,818 | 75 / 449,422 | 22.79 (22.08-23.90) | 12.22 |
| 96k | 0.87 (band) | 450,563 | 130 / 448,192 | 22.22 (21.88-23.01) | 12.23 |
| 96k | 0.74 | 450,818 | 449,575 / 449,618 | 16.91 (16.29-18.02) | 12.16 |
| 96k | 0.50 | 453,634 | 42 / 41 | 12.22 (12.17-12.26) | 12.15 |

Later sections' majorants and the deviation components were never subnormal in these designs (0
frames). Measured cause: **subnormal operands**. Under FTZ every design costs 12.15-12.31 ns a frame.
The band (about 1.85 times) is where the first state reaches exactly zero and the error radius is
stuck at a subnormal fixed point (`fl(fl(q e)(1 + 4u)) = e` in the subnormal grain) for the rest of
the walk; where both the state and the radius stay subnormal it is about 1.35 times; a subnormal
state alone, about 1.06 times. (The attempt-2 verdict's 12.9 ns at 17.28 kHz is not reproduced:
16.47 ns here.)

#### Scope 2: the certified flush (`crates/math/src/tail.rs`)

`TAU = 2^-600`. Non-negative quantities (`Majorants::first_error`, `Majorants::later`, each frame's
`out.input[1..]`, the walked `Deviation` components and its output value) are stored as
`max(fl(step), TAU)`. The first state's words below `TAU` in magnitude are set to zero and `TAU` per
flushed word is added to `first_error`, rounded up (`Majorants::flush_first`, after the state's
step). `Deviation::floor` is separate from `Deviation::step`, so `linear_map` stays linear. The
remainder's `v_norm` of the first state is replaced by `|s1| + |s2|` when both words are below
`2^-500` (both squares can underflow; found while proving the closing terms). The proof is the
second of the spec's two: an explicit absolute underflow term (`2^-1075` per rounding), covered by
the floor when the bounded value is at most `tau` and by the step's relative margin (at least
`2^-54 tau = 2^-654`) when it is above; the derivation states it, the closing terms and why `tau`
cannot keep the walk from finishing (`docs/derivations/1329-input-section-tail-and-rest.md`,
"Numerical limits").

**Gate 1 (bounds).** One-time comparison against the pre-change base (PR evidence, not a committed
test): every family of `input_bound_budget.rs` at the four rates plus every design of the
calibration grid (399,668 lines of `InputSectionBound`s and frames walked), and 3,984
`math::tail::fixed_cascade_within` results (`CascadeBound` with `flush_floor`, and frames) over a
166-cutoff grid of one-section and two-section cascades at 0 and +24 dB: **identical, bit for bit**
(sha256 `08c95dc3...` and `e29ae333...` both ways). Frames walked are unchanged, so every charge is
unchanged. `tail_contract` (release): 13 passed.

**Gate 5 (tests, `math::tail::tests`) and mutation runs.** Each mutation was applied, the math lib
tests run, and the file restored; all four new tests are green on the final code.

| mutation | red test(s) |
|---|---|
| M1 radius flushed to zero when at most `tau` | `a_non_negative_majorant_below_tau_propagates_as_tau`, `the_band_designs_walk_with_every_propagated_quantity_at_least_tau` |
| M1b radius floor removed (`max(0)`, subnormal kept) | same two |
| M2 later majorant flushed to zero | `a_non_negative_majorant_below_tau_propagates_as_tau` |
| M3 output majorant floor removed | `a_non_negative...`, `the_band_designs...` |
| M4 deviation floor flushes to zero | `a_non_negative...` |
| M5 state flushed without adding `tau` to the radius | `the_first_state_below_tau_is_flushed_to_zero_and_its_magnitude_enters_the_radius` |
| M6 state rounded up to `tau` instead of flushed | `the_first_state...` |
| M7 words below `4 tau` flushed with only `tau` added | `the_first_state...` |
| M8 flush before the state's step instead of after | `the_band_designs...` |
| M9 state flush removed | `the_first_state...`, `the_band_designs...` |
| M10 remainder takes `v_norm` of a tiny state | `a_tiny_first_state_still_enters_the_remainder` |

Test value: the first test catches a floor that rounds down or to zero on any non-negative quantity;
the second a flush that loses the flushed magnitude, rounds the signed state up, or flushes words it
should keep; the third a band design leaving a quantity below `tau` at the end of a frame (the
flush's placement); the fourth the remainder's underflow.

#### Scope 3: recalibration (gate 3), measured, not restated

`input_bound_budget.rs calibrate` now takes each frame-class point's median of five measured sweeps
after one warmup sweep, with its spread; gate 2 prints each preparation's charge and its design work
per frame-equivalent consumed (the charge when every design is exact, else the budget).

- **Before** (pre-change walk, same tool; load 3.82 -> 3.69): slowest class 21.95 ns
  (21.82-22.54), 88.2 kHz, 0 dB, HPF at 0.87; fastest medians 11.96-12.00 ns; fractions above 1.2
  times the fastest: 0.66-0.98 at every rate. Largest fixed cost per section 310.0 frame-equivalents
  of 21.95 ns (6.80 us).
- **After** (load 0.90 -> 1.05): slowest class **13.80 ns (13.72-13.91)**, 88.2 kHz, +24 dB, the top
  pair; fastest medians 13.05-13.10 ns; no point above 1.2 times the fastest. Largest fixed cost per
  section 13.64 us for a two-section cascade at 96 kHz (6.82 us a section, 487.1 frame-equivalents
  of 14.0 ns). Per-design term -1.66 to -2.44 us at every rate.
- The flush costs about 1.1 ns a frame on designs that never went subnormal (fastest medians 12.0 ->
  13.1 ns); compare-select instead of `f64::max` and flooring off the recurrence chain did not move
  it measurably (noise about 0.5 ns on this box).

**Gate 2 with the near-top restatement (14.0 ns, charge 490; not committed)**, one invocation, one
warmup and two measured rounds, load 0.72 -> 0.84. Design work per family over rates and rounds,
and ns per frame-equivalent consumed:

| family | design work ms | ns per consumed FE |
|---|---|---|
| 64 band designs | 19.70-20.66 | 13.04-13.68 |
| 4,096 band designs | 19.83-20.13 | 13.13-13.33 |
| 64-design near-top | 20.64-21.12 | 13.67-13.98 |
| 65,537 near-top | 20.61-21.40 | 13.65-14.17 |
| 64 typical | 10.73-21.57 | 14.61-14.87 |
| 256 typical | 22.23-22.68 | 14.72-15.02 |
| cheap two-section 1 kHz / 1.28 kHz +24 dB | 20.26-**23.64** (88.2 kHz) | 13.42-**15.66** |
| cheap two-section 1.28 / 5.12 kHz | 18.73-21.42 | 12.40-14.19 |
| other cheap families | 14.25-19.58 | 9.44-12.97 |

The band families' design work fell from 33.48-34.90 ms (#1457 attempt 3) to 19.70-20.66 ms, and
they confirm the 13.80 ns class. But the typical and cheap two-section families run 14.6-15.7 ns per
frame-equivalent, above 14.0: under that restatement the budget's 21.14 ms would be exceeded
(23.64 ms). A single-design FTZ A/B shows this is not subnormal: the typical design (20 Hz HPF into
20 kHz LPF) costs 15.1-16.9 ns per walked frame and 14.5-15.6 ns per charged frame-equivalent, the
same with FTZ (within 0.3 ns); the calibration's short-walk fit slopes are 15.1-16.4 ns per frame.
Medium and short walks cost more per frame than long near-top walks, and the spec's frame classes do
not measure them.

**Committed state:** `INPUT_BOUND_SECTION_CHARGE` 290 and the 23.5 ns frame-equivalent unchanged
(every measured frame, 13-17 ns, stays covered; the largest section fixed cost, 6.82 us in one
round, is 0.07 % above 290 x 23.5 ns = 6.815 us, within run noise; #1457 attempt 3 measured 6.78 us
for the same class, and the walk change does not touch the fixed cost). Gate 8 (Amendment 3's command): exact at every rate, unchanged charges (96 kHz:
charged 1,328,256, margin 181,744). At charge 490 the 96 kHz margin would be 130,544.

#### Open for root

1. **Ruling needed (gate 3).** Restate the frame-equivalent from which class? Options: (a) add the
   medium-walk classes (typical and cheap two-section designs) to the calibration's frame classes and
   take the slowest median, about 15-16 ns, with the section charge restated in those units (6.82 us
   is 430 at 16 ns); (b) keep the near-top classes only and confirm against every gate-2 family, not
   only the band. Either needs one more calibration and gate-2 invocation on the final constants.
2. Specs outside this slice cite 23.5 ns / 35.5 ms / 34.90 ms: #1468 (`:152`), #1470 (`:28-31`),
   #1471 (`:21`); they need the restated figures once root rules.
3. The flush adds about 1.1 ns a frame to every walk (12.0 -> 13.1 ns); a cheaper formulation is a
   possible follow-up, not pursued here.

#### Other gates run

`cargo fmt --all -- --check`; `cargo clippy --locked -p math -p builtins --all-targets --features
builtins/test-support -D warnings` and `-p math --features lane`: clean. `cargo test -p math`: all
pass. `tail_contract` in release (the CI command): 13 passed. Gate 8: passed. `check-workspace-
policy.sh`: ok; `check-realtime-policy.sh`: ok; `check-cross-targets.sh`: exit 0 (only the expected
#1018 failures reported).

#### Root's ruling (2026-10-08)

- The frame-equivalent comes from the slowest class over all frame classes: near-top, band,
  typical and cheap two-section. The typical and cheap two-section designs join the calibration's
  frame classes, each point the median of repeated runs, with min-max.
- It is confirmed against every gate-2 family, not only the band.
- `INPUT_BOUND_SECTION_CHARGE` is restated from it.
- One calibration and one gate-2 invocation (#1457 Amendment 3's commands; one warmup, two measured
  rounds, no retry, load before and after) on the final constants, both recorded.
- Gate 8 must still show every 64-track document exact at every rate.
- The flush's +1.1 ns a frame is recorded as a cost; it gets no follow-up issue.
- Authorized in addition: the stale figures in the local specs of #1468, #1470 and #1471.
- Timing waits until the 1-minute load is below about 2 (a verifier builds on this box).

#### Completion

**Calibration change.** `calibrate` now runs the design-class fits first, then the frame classes:
near-top (51 HPFs, 0 and +24 dB), typical (HPF 20, 40, 80 Hz into LPF 16, 18, 20 kHz, 0 and
+24 dB) and gate 2's cheap two-section designs (1/1.28, 1.28/5.12, 2.56/5.12 and 5.12/10.24 kHz at 0,
+12 and +24 dB). A point's value is its ns per walked frame **net of the fixed cost** (the rate's
mean measured intercept of one cascade of two sections), so short walks compare frame for frame with
long ones; the median of five measured sweeps after one warmup sweep, with min-max.

**The calibration (one invocation, `taskset -c 7`, load 1.52 -> 2.18).** Slowest median per class
and rate, ns per frame net of the fixed cost (min-max):

| rate | near-top 0 / +24 dB | typical 0 / +24 dB | cheap two-section 0 / +12 / +24 dB |
|---|---|---|---|
| 44.1k | 13.73 / 13.68 | **16.54 (16.39-17.10)** / 16.52 (16.37-18.25) | 13.66 / 14.35 / 16.05 (15.81-16.36) |
| 48k | 13.69 / 13.69 | 16.26 / 16.36 | 13.23 / 13.22 / 15.21 |
| 88.2k | 13.65 / 13.61 | 15.24 / 16.24 (15.83-17.14) | 14.64 / 14.27 / 15.76 |
| 96k | 13.68 / 13.67 | 14.97 / 15.05 | 12.81 / 14.52 / 14.61 |

Slowest class: **16.54 ns** (spread 16.39-17.10), typical, an 80 Hz HPF into an 18 kHz LPF, 0 dB,
44.1 kHz. **Frame-equivalent: 17.0 ns** (rounded up; superseded by root's 2026-10-08 rulings
below: 17.5 ns, then 18.5 ns). Largest fixed cost per section: **7.03 us**
(96 kHz, round 2; the other rounds and rates 5.53-6.61 us), 413.5 frame-equivalents of 17.0 ns:
**`INPUT_BOUND_SECTION_CHARGE` = 420** (superseded below: 410, then 380). The per-design term is negative at
every rate (-1.55 to -3.46 us). The budget, 1,510,000 frame-equivalents, is **25.67 ms** (superseded
below: 26.43 ms, then 27.94 ms).

**Gate 2 on the final constants (one invocation, load 1.96 -> 2.13).** Design work over rates and
measured rounds, and ns per frame-equivalent consumed:

| family | design work ms | ns per consumed FE |
|---|---|---|
| top pair | 7.19-7.43 | 13.58-14.10 |
| LPF at the maximum | 3.71-3.76 | 8.27-8.38 |
| 64-design near-top | 20.74-21.41 | 13.74-14.18 |
| 65,537 near-top | 20.47-21.23 | 13.55-14.06 |
| 64 band designs | 19.94-20.21 | 13.21-13.39 |
| 4,096 band designs | 19.92-20.07 | 13.19-13.29 |
| 64 typical | 10.73-20.88, one sample 28.44 | 14.46-14.98, one sample 21.35 |
| 256 typical | 21.91-22.72 | 14.51-15.05 |
| 4,096 cheap one-section | 15.22-16.71 | 10.08-11.06 |
| 4,096 cheap two-section 1 / 1.28 kHz, +24 dB | 21.86-**25.44** | 14.48-**16.85** |
| 4,096 cheap two-section 1.28 / 5.12 kHz | 20.47-22.74 | 13.56-15.06 |
| 4,096 cheap two-section 2.56 / 5.12 kHz | 18.75-22.01 | 12.42-14.57 |
| 4,096 cheap two-section 5.12 / 10.24 kHz | 17.13-22.02 | 11.34-14.58 |
| 4,096 cheap two-cascade | 15.30-16.40 | 10.13-10.86 |

Every family confirms the 17.0 ns frame-equivalent (at most 16.85 ns per consumed frame-equivalent),
except one sample: the 64 typical designs at 88.2 kHz, round 2, no-cache preparation, 28.44 ms. In
the same round the first preparation of the same designs (the same walks plus the cache insertion)
took 19.43 ms, and the family's other seven measured samples took 10.73-20.88 ms
(14.46-14.98 ns per frame-equivalent), so this is interference on the box (the load rose to 2.13 and
18 runnable tasks during the run), not a frame class. Not retried, as the ruling requires.

**The stated worst case:** **25.44 ms** of design-bound work (4,096 cheap two-section designs, 1 kHz
into 1.28 kHz, +24 dB, 88.2 kHz, round 1), inside the 25.67 ms budget (26.43 ms, then 27.94 ms, after the follow-ups below), down from #1457's 34.90 ms;
spread as in the table. The per-frame cause: a frame costs 13.2-13.7 ns in long near-top and band
walks and 15-16.5 ns in typical and short walks (net of the fixed cost); no class depends on
subnormal operands any more. The slowest whole preparation is 55.86 ms (65,537 distinct near-top
designs at 96 kHz), of which 21.23 ms or less is design work; the rest is the keying and lookup of
65,537 strips, as in #1457.

**The flush's cost.** About +1.1 ns a frame on designs that never went subnormal (long walks: 12.0
-> 13.1 ns). Recorded as a cost; no follow-up issue (root's ruling).

**Gate 8 (Amendment 3's command):** passed, every 64-track console document exact at every rate.
Charges at charge 420: 44.1 kHz 717,952 (margin 792,048), 48 kHz 766,336 (743,664), 88.2 kHz
1,265,280 (244,720), 96 kHz 1,361,536 (margin **148,464**); the mono document at 96 kHz 704,576.

**Gates rerun on the final tree:** `cargo fmt --all -- --check`; clippy (`-p math -p builtins -p
builtins-compiler --all-targets` with test-support, `-D warnings`): clean. `cargo test -p math`, the
`builtins` and `builtins-compiler` lib tests (test-support), and `tail_contract` in release (13
passed): all pass. `check-workspace-policy.sh`, `check-realtime-policy.sh`: ok.
`check-cross-targets.sh`: exit 0.

**Specs corrected** (root-authorized): #1468 (`:152`, the restated charge), #1470 (`:28-31`, the
budget, the worst case and the spreads), #1471 (`:21`, the budget).

#### Follow-ups after attempt 1's verdict (PASS; MINOR-1..3, NIT-1..4)

**Root's ruling (2026-10-08): a stated worst case is a bound.** (Its statistic, 17.5 ns, 410 and
26.43 ms are superseded by root's re-ruling below: 18.5 ns, 380 and 27.94 ms.) So the frame-equivalent comes from
the slowest class's measured **maximum**, not its median. The statistic: the slowest frame class
(by median) is typical, an 80 Hz HPF into an 18 kHz LPF, 0 dB, 44.1 kHz, median 16.54 ns, spread
16.39-17.10 ns; its maximum, 17.10 ns, rounded up to the next half nanosecond, gives **17.5 ns**.
The reason: a median lets half of the slowest class's samples cost more than the unit the budget
charges, so a worst case stated from it is not a bound. This is a constant change only: no new
calibration and no new gate-2 run.

- **`INPUT_BOUND_SECTION_CHARGE` = 410.** The largest recorded fixed cost per section, 7.03 us, is
  401.7 frame-equivalents of 17.5 ns. Rounded up to the next ten (the rule that gave 420 from
  413.5), 410 frame-equivalents are 7.18 us, 2.1 % above it.
- **The budget** (unchanged, 1,510,000 frame-equivalents) is now 1,510,000 x 17.5 ns = **26.43 ms**
  (26.425 ms) of design-bound work.
- **The margin (MINOR-3).** The measured worst family, 25.44 ms (4,096 cheap two-section designs,
  1 kHz into 1.28 kHz, +24 dB, 88.2 kHz), is 96.3 % of 26.43 ms: a margin of 0.99 ms (3.7 %), still
  inside that family's own 4.6 % run-to-run spread (24.28-25.44 ms) at that rate; per consumed
  frame-equivalent, 16.85 against 17.5 ns (3.7 %).
- **Not covered by this statistic (flagged for root).** One other point of the recorded
  calibration has a larger maximum than 17.5 ns: typical, +24 dB, 44.1 kHz, median 16.52 ns, spread
  16.37-**18.25** ns (and typical, +24 dB, 88.2 kHz reaches 17.14 ns). The ruling takes the slowest
  class's maximum, which is 17.10 ns; I applied it as ruled and did not widen it.
- **The calibration binary (NIT-4).** `g3/cal-final.log` printed the then-committed constants
  ("frame-equivalent 23.5 ns (committed)"), but its measurement does not use them: `one_design`
  runs with a budget of `u64::MAX`, so the figures above are valid for 17.5 ns and 410. The
  example now carries 17.5 ns and its `calibrate` prints the slowest class's maximum as well as its
  median.
- **Gate 8** (Amendment 3's command): passed, every 64-track console document exact at every rate
  at charge 410. Charged and margin: 44.1 kHz 715,392 (794,608), 48 kHz 763,776 (746,224),
  88.2 kHz 1,262,720 (247,280), 96 kHz 1,358,976 (**151,024**); the mono document at 96 kHz 703,296
  (806,704). Frames walked are unchanged (96 kHz 1,254,016).
- **The cache cap's arithmetic** (`INPUT_BOUND_CACHE_ENTRIES`' doc) still used Amendment 4's 290:
  a design charges at least 257 + 410 = 667 frame-equivalents, so at most 2,263 designs a
  preparation; the cap of 8,192 still holds them.
- **MINOR-1.** The flush test now also flushes `first = [0.9 tau, 0.9 tau]` (a `V`-norm of about
  1.66 `tau`) and asserts the radius grows by at least that norm. Mutation `flushed = TAU` (one
  `tau` for two words) in `flush_first`: red (`the_first_state_below_tau_...`, the new assertion);
  reverted: green.
- **NIT-1..3.** Budget figures are written to two decimals where a margin matters (25.67 and
  26.43 ms, not 25.7); `TAU` is a private `const`; the derivation's termination paragraph covers
  the suffix-sum search for `t0` (at most `2^26 tau = 2^-574` per suffix) and states that the
  horizon, not the margins, guarantees termination.
- **Specs corrected** (root-authorized): #1468 `:152`, #1470 `:28-31` and `:48` (the section
  charge, which still said 290), #1471 `:21`.

**Gates.** `cargo test --locked -p math`: all pass. `tail_contract` in release with
`builtins/test-support`: 13 passed. Gate 8: passed (above). `cargo fmt --all -- --check`: clean.
clippy (`-p math -p builtins -p builtins-compiler --all-targets --features builtins/test-support`
and `-p math --features lane --all-targets`, `-D warnings`): clean (only the pre-existing
`clippy.toml` `fast_db` path notes). `check-workspace-policy.sh`: ok. `check-cross-targets.sh`:
exit 0, PASS (only the expected #1018 rows).

#### Follow-up: root's re-ruling (2026-10-08) -- the maximum over all points

**Root's ruling (2026-10-08; supersedes the 17.5 ns ruling).** A stated worst case is a bound, and
a bound does not pick which outlier to believe. So the frame-equivalent comes from the **maximum
over all recorded calibration points**, not the slowest class's maximum. The statistic: the largest
recorded sample is **18.25 ns** (typical, +24 dB, 44.1 kHz, the class's slowest point, an 80 Hz HPF
into an 18 kHz LPF, median 16.52 ns, spread 16.37-18.25 ns; `g3/cal-final.log`); rounded up to the next half
nanosecond, the frame-equivalent is **18.5 ns**. The reason: any recorded sample above the unit
the budget charges is a frame the stated worst case does not bound. Constant change only: no new
calibration and no gate-2 run.

- **`INPUT_BOUND_SECTION_CHARGE` = 380.** The largest recorded fixed cost per section, 7.03 us
  (96 kHz, round 2), is 7,030 / 18.5 = 380.0 frame-equivalents. Rounded up to a ten by the same
  rule as before (413.5 -> 420, 401.7 -> 410), it stays **380**, because 380.0 is already a ten.
  It is not below the measured cost: the same log prints that point's intercept for the cascade's
  two sections as 14.05 us, so the unrounded per-section cost lies in 7.025-7.0275 us
  (379.7-379.9 frame-equivalents), and 380 frame-equivalents are 7.03 us.
- **The budget** (unchanged, 1,510,000 frame-equivalents) is 1,510,000 x 18.5 ns = **27.935 ms**,
  written 27.94 ms.
- **The margin.** The measured worst family, 25.44 ms (4,096 cheap two-section designs, 1 kHz
  into 1.28 kHz, +24 dB, 88.2 kHz, round 1), is 25.44 ms of 27.94 ms (91.1 %): a margin of
  2.50 ms (8.9 %), outside that family's 4.6 % run-to-run spread at that rate (24.28-25.44 ms);
  per consumed frame-equivalent, 16.85 against 18.5 ns.
- **What "recorded" covers.** The calibration log prints, per class and rate, only the slowest
  point (by median) with its min-max, so 18.25 ns is the maximum over the printed points. The
  example's `calibrate` now tracks and prints the largest sample over **every** point of every
  class ("largest sample over every point"), and states the section charge against it, so the next
  calibration reports this statistic directly.
- **The cache cap's arithmetic** (`INPUT_BOUND_CACHE_ENTRIES`' doc): a design charges at least
  257 + 380 = 637 frame-equivalents, so at most 1,510,000 / 637 = 2,370 designs a preparation; the
  cap of 8,192 still holds them.
- **Gate 8** (Amendment 3's command): passed, every 64-track console document exact at every rate
  at charge 380 (frames walked unchanged). Charged and margin, for the stereo documents
  (`console-sixty-four-track`, `-app`, `-intended`, `-sends`, identical): 44.1 kHz 707,712
  (802,288), 48 kHz 756,096 (753,904), 88.2 kHz 1,255,040 (254,960), 96 kHz 1,351,296
  (**158,704**); the mono document: 44.1 kHz 365,120 (1,144,880), 48 kHz 390,464 (1,119,536),
  88.2 kHz 649,792 (860,208), 96 kHz 699,456 (810,544).
- **Statements updated:** `crates/builtins/src/tail.rs` (the charge's and the budget's docs, and
  the cache cap's per-design figure), `crates/builtins/examples/input_bound_budget.rs`
  (`FRAME_EQUIVALENT_NS` = 18.5 and the `calibrate` statistic), and the local specs of #1468
  (`:152`), #1470 (`:28-31`, `:48`) and #1471 (`:21`) (root-authorized).
  `docs/derivations/1329-input-section-tail-and-rest.md` states none of these figures (no
  frame-equivalent, charge or budget), so it is unchanged.

**Gates.** `tail_contract` in release with `builtins/test-support`: 13 passed. Gate 8: passed
(above). `cargo test --locked -p math`: all pass. `builtins` and `builtins-compiler` lib tests
(test-support): 15 and 57 passed. `cargo fmt --all -- --check`: clean. clippy (`-p math -p
builtins -p builtins-compiler --all-targets --features builtins/test-support` and `-p math
--features lane --all-targets`, `-D warnings`): clean (only the pre-existing `clippy.toml`
`fast_db` path notes). `check-workspace-policy.sh`: ok. `check-cross-targets.sh`: exit 0, PASS
(only the expected #1018 rows).

**Note (2026-10-08, from #1465).** #1465 restated this issue's figures after it: under root's
second frame-equivalent ruling of 2026-10-08 (each design's cost the median of five measured
samples) and its ruling (c) of the same day (a calibration sample is a batch of 48 different
designs, each batch's median per design against its charge per design), the frame-equivalent is
17.0 ns and `INPUT_BOUND_SECTION_CHARGE` 550, so the budget is 25.67 ms and the cache cap's
arithmetic 257 + 550 = 807 a design, at most 1,871 designs a preparation. The 18.5 ns, 380 and
27.94 ms above, and #1465's earlier 21.5 ns, 470 and 32.47 ms, are superseded; #1465's attempt record has the
calibration and gate 2.
