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
