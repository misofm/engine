# Red mutations for `miso-engine-true-peak-limiter` (issue #90, wave 2)

Every gate in this crate landed with the mutation below applied once and observed red. Each row
states the edit, the test that failed, and — where it matters — why that test and not the obvious
one. Restore the file after each run; the mutations are not committed.

Run one with, e.g.:

```
cargo test -p miso-engine-true-peak-limiter --lib phase_outputs
```

| # | mutation | file | test that turned red |
|---|---|---|---|
| 1 | `fir.iter().zip(history.iter())` → `fir.iter().rev().zip(history.iter().rev())` (FIR summed in decreasing tap order) | `src/lib.rs` `annex2_phases` | `phase_outputs_match_the_frozen_scalar_order` |
| 2 | tap 1 phase 0 of `ANNEX2_FIR` moved by one decimal digit (`0.010_986_328` → `0.010_986_329`) | `src/lib.rs` | `bs1770_annex2_conformance_is_unchanged` |
| 3 | main ring `N + 6` → `N + 7` | `src/lib.rs` `Shape::new` | `fixed_latency_guarded_ceiling_and_bypass_bits_hold` |
| 4 | `MINIMUM_RAMP_WINDOW` 32 → 1 | `src/lib.rs` | `production_tracks_the_f64_oracle` (E5) |
| 5 | `smoothed = box_sum / window` → `smoothed = quantised` (box smoother removed, bare minimum applied) | `src/lib.rs` `channel_frame` | `the_gain_ramp_falls_gradually_and_arrives_at_the_requirement` |
| 6 | `flush(target.max(released))` → `target.max(released)` | `src/lib.rs` `channel_frame` | `silence_restores_exact_identity_including_signed_zero` |
| 7 | van Herk suffix pass guarded by `complete && width < 2`, so it never runs at W4/W8 | `src/lib.rs` `sliding_minimum` | `lane_identity_holds_across_widths` |
| 8 | `left.prefix.fill(1.0)` / `right.prefix.fill(1.0)` at block entry (min-filter prefix not carried across blocks) | `src/lib.rs` `limiter_block` | `partition_invariance_holds_over_block_sizes` |
| 9 | `return;` inserted after `limiter_block` so the §4.4 boundary check never runs | `src/lib.rs` `LimiterCore::process_block` | `a_nonfinite_block_is_zeroed_reset_and_counted` |
| 10 | the `recomputed == box_sum` check in restore disabled | `src/lib.rs` `read_lane` | `state_v2_round_trips_and_rejects_corruption` |
| 11 | `peak = history[6].abs()` → `peak = L::zero()` (sample term dropped from `P`) | `src/lib.rs` `detector_peak` | `fixed_latency_guarded_ceiling_and_bypass_bits_hold` |
| 12 | `box_sum.div(window)` → `box_sum.mul(1.0.div(window))` (reciprocal instead of divide) | `src/lib.rs` `channel_frame` | `silence_restores_exact_identity_including_signed_zero` |
| 13 | `current = select(remaining > 0, current + step, target)` → `current = current + step` (D11 snap removed) | `src/lib.rs` `RampLanes::advance` | `the_lane_ramp_reproduces_the_scalar_ramp_bit_for_bit` |
| 14 | `write_u32(bytes, MAIN_CURSOR, cursors.main.swap_bytes())` (payload endianness) | `src/lib.rs` `snapshot_lane` | `state_v2_round_trips_and_rejects_corruption` |
| 15 | `let leak: Vec<f32> = vec![0.0; 4];` inside the van Herk suffix pass | `src/lib.rs` `sliding_minimum` | `the_render_path_allocates_nothing` |
| 16 | `crates/miso-engine-true-peak-limiter/src/lib.rs` re-added to the `check-math-policy.sh` allowlist while `limit_coefficient` calls `10.0_f32.powf(..)` | `src/lib.rs` | `scripts/check-math-policy.sh` (allowlist entry with zero call sites fails) |

## Mutations that survived their first target, and what was done about it

Recorded rather than quietly re-aimed, because each one says something about what a gate does and
does not prove.

* **`MINIMUM_RAMP_WINDOW` 32 → 1** does **not** turn the ceiling gate (E4) red. It moves the worst
  true-peak margin over the whole E4 matrix from **−0.961 dB to −0.398 dB** — it eats 0.56 dB of
  the 1 dB internal guard without breaching the user ceiling on these corpora. The floor is
  therefore gated by E5 (the `f64` oracle carries the same `W_MIN` and disagrees immediately) and
  by E6 (the ramp shape), and the 0.56 dB is reported to #49 as the measured cost of removing it.
  The plan's suggested E4 mutation assumed the floor was load-bearing for the ceiling itself; on
  the corpora that exist it is load-bearing for the *headroom*.
* **`flush(d)` removed** does not change any *output* sample: the release decays past
  `FLUSH_EPS` into the far-subnormal range, and `1 - 1e-30` still rounds to exactly `1.0`, so the
  identity holds anyway. What it changes is the recursive word itself, which is where FTZ makes
  two targets disagree. E7 was strengthened to assert the reduction word in the state payload is
  exactly `+0.0` bits; the mutation is red against that.
* **Reciprocal instead of divide** survives at most window lengths because `Wb * (1 / Wb)` happens
  to round back to exactly `1.0`. It does not at `Wb = 97`, which is a 2 ms lookahead at 48 kHz, so
  E7 sweeps lookaheads `{0, 2, 5, 10} ms` and the mutation is red at the 2 ms one.
* **Min-filter prefix always `1.0`** (rather than not carried across blocks) is *partition
  invariant* — it is consistently wrong — so it is not a P1 mutation at all. The P1 mutation is
  clearing the prefix at block entry, which is what row 8 does.
* **D11 snap removed** survives the cross-target corpus, because no corpus case retargets a ramp:
  every case runs with `remaining == 0` throughout, so `RampLanes::advance` is a no-op there. The
  lane ramp got its own dedicated gate (E13, row 13) instead of widening the corpus.

## Issue #143 — the resident gain-reduction tap

| # | mutation | file | test | result |
|---|---|---|---|---|
| 143-E6-c | `observe_resident` "freshens" the recursive reduction word by one release step (`* 0.9`) in the read | `true-peak-limiter/src/lib.rs` | `observation::the_limiter_reads_the_reduction_word_the_envelope_persists` | RED — the tap and the state envelope disagree: `1044868013` vs `1046320150`. The envelope is a second, already-gated route to the same kernel word, so agreeing with it is agreeing with the kernel |

## Issue #182 S1 — the uniform-cohort vectorisation

`sliding_minimum` and the box-expiry gather of `channel_frame` were the kernel's only scalar
sections, because `LaneShape` is a per-lane preparation parameter. `lanes_uniform` is a whole-bank
gate — one branch per block, per the single-branch discipline — under which both collapse to
lane-wide row operations (`sliding_minimum_uniform`, and one `L::load` of the expiring box term).
Bit identity is structural: `Lane::min` is defined as `select(self < b, self, b)` (decision D8) and
`scalar_min` is `if a < b { a } else { b }`, so one lane of `a.min(b)` *is* `scalar_min(a, b)`.

The E12 corpus exercises both paths at every width without any change: cases 2 and 3 give every
lane the same lookahead and therefore take the vector path at W4 and W8, while cases 0, 1 and 4
are mixed and take the fallback. `D90_DIGESTS` did not move.

| # | mutation | file | test that turned red |
|---|---|---|---|
| 182-1 | shape leg dropped: `lanes_uniform` returns only the phase test | `src/lib.rs` `lanes_uniform` | `a_mixed_lookahead_cohort_falls_back_bit_identically` (and `lane_identity_holds_across_widths`) |
| 182-2 | phase leg dropped: `lanes_uniform` returns only the shape test | `src/lib.rs` `lanes_uniform` | `a_restore_that_desyncs_the_phase_falls_back` — **and nothing else**, which is what makes the leg's own justification testable rather than asserted |
| 182-3 | `suffix.min(L::load(..))` → `suffix.max(..)` in the vector suffix pass | `src/lib.rs` `sliding_minimum_uniform` | `a_mixed_lookahead_cohort_falls_back_bit_identically`, `a_restore_that_desyncs_the_phase_falls_back`, `lane_identity_holds_across_widths` |
| 182-4 | uniform box gather reads the cohort's `end_offset` instead of `box_offset` | `src/lib.rs` `channel_frame_uniform` | as 182-3 |
| 182-5 | the vector suffix pass guarded by `complete && width < 2`, so it never runs at W4/W8 (row 7's analogue in the new path) | `src/lib.rs` `sliding_minimum_uniform` | `a_uniform_cohort_renders_exactly_the_per_lane_path` — **and nothing else**: every cohort the other cross-width tests build falls back |

### Why the cross-path comparison is where it is

A scalar instance is `L = f32`, `W = 1`, so it is uniform by construction and takes the *new* path.
That is why `a_uniform_cohort_renders_exactly_the_per_lane_path` cannot be red for a mutation that
applies at every width — both of its arms move together — and why the honest cross-path gate is
`a_mixed_lookahead_cohort_falls_back_bit_identically`, whose bank lanes run the per-lane body while
their scalar twins run the uniform one. Rows 182-3 and 182-4 are recorded against that test rather
than against the one whose name sounds like it should own them.

### A mutation that does not turn anything red, and why it is not a gap

**Swapping the argument order of all three `L::min` sites** in `sliding_minimum_uniform`
(`prefix.min(newest)` → `newest.min(prefix)`, and so on) leaves
every test in the crate green, including the E12 pins. This is recorded rather than papered over,
because the reason is a statement about the kernel's value domain and not about the gates.

D8 makes `min` asymmetric only where the two operands are *indistinguishable as numbers but
distinguishable as bits*, or unordered: `min(+0.0, -0.0)`, `min(-0.0, +0.0)`, and either order with
a NaN. Neither is reachable in the required-gain ring. `r = select(P > limit, limit / P, 1)` with
`limit >= 10^(-25/20) > 0` yields a strictly positive quotient or exactly `1.0`; `P` is a maximum of
absolute values, so `P > limit` implies `P > 0` and the quotient is never a zero of either sign and
never a NaN. `prefix` rests at `1.0` and only ever takes minima of ring values, so it inherits the
same domain. `P = +inf` — the one way to reach `r = +0.0` — is a §4.4 boundary failure, which zeroes
the block and resets the instance before the ring can be read again.

So the argument order is unobservable here *for the data this kernel can hold*, and a test that
made it observable would have to reach past the public API to plant a `-0.0` in the ring. It is kept
in the frozen order anyway, and the order is the reason the D8 citation in
`sliding_minimum_uniform`'s prose is a citation and not a hand-wave: the equality it claims is an
equality of definitions, which holds on the whole `f32` domain rather than only on the reachable
part of it.

## Issue #182 S2 — the earned silence fixed point

The compressor's `silent_fixed_point` design (#163 phase 4 item 1) at a kernel whose rest state is
not all zeros. `LimiterCore::process_block` admits a block when the claim stands, no ramp window is
open, the bypass flag has not moved and both input planes are exactly `+0.0`; it earns the claim
back from a block that *ran* and left both channels at `clear_runtime`'s documented rest state with
an all-`+0.0` output. A skipped block advances the two cursors and each lane's van Herk phase and
touches nothing else, which makes it bit-identical to the block that ran rather than merely
equivalent to it.

Every gate here has two arms over the same signal, the same parameters and the same block
boundaries; the control arm withdraws the claim before every block, so the only difference between
the arms is the fast path. Both the rendered samples **and** the whole instance state — every ring,
the recursive word, the cursors, the phases and all four ramps — are compared, block by block.

| # | mutation | file | test that turned red |
|---|---|---|---|
| 182-6 | input `block_is_positive_zero` legs dropped from the admission test | `src/lib.rs` `LimiterCore::process_block` | `a_settled_silent_limiter_renders_exactly_the_never_fast_path`, `a_negative_zero_input_block_is_not_treated_as_silence`, `a_stale_detector_history_refuses_the_claim` |
| 182-7 | `block_is_positive_zero(&self.reduction)` dropped from `is_at_silent_rest` | `src/lib.rs` `ChannelState::is_at_silent_rest` | `a_limiter_still_releasing_through_the_silence_is_never_frozen` |
| 182-8 | `block_is_positive_zero(&self.main_ring)` dropped | `src/lib.rs` `ChannelState::is_at_silent_rest` | `a_settled_silent_limiter_renders_exactly_the_never_fast_path`, `a_negative_zero_input_block_is_not_treated_as_silence` |
| 182-9 | `block_is_positive_zero(&self.history)` dropped | `src/lib.rs` `ChannelState::is_at_silent_rest` | `a_stale_detector_history_refuses_the_claim` — **and nothing else** |
| 182-10 | output `block_is_positive_zero` legs dropped from the claim | `src/lib.rs` `LimiterCore::process_block` | `a_settled_silent_limiter_renders_exactly_the_never_fast_path` |
| 182-11 | `self.cursors.advance(frames, &self.shape)` deleted from the fast path | `src/lib.rs` `LimiterCore::process_block` | `a_settled_silent_limiter_...`, `a_negative_zero_input_...`, `a_stale_detector_history_...` (via the state comparison, not the sample comparison) |
| 182-12 | both `advance_rest_phase(frames)` calls deleted from the fast path | `src/lib.rs` `LimiterCore::process_block` | as 182-11 |
| 182-13 | the four `ramps_are_stationary` legs dropped from the admission test | `src/lib.rs` `LimiterCore::process_block` | `a_de_zipper_window_open_across_a_block_boundary_refuses_the_claim` — **and nothing else** |
| 182-14 | `self.silent_fixed_point = false` deleted from `restore_track` | `src/lib.rs` `LimiterCore::restore_track` | `a_restore_withdraws_the_silence_claim` — **and nothing else** |
| 182-15 | `if !block.automation.is_empty()` withdrawal deleted from `process` | `src/lib.rs` `PreparedTruePeakLimiter::process` | `automation_withdraws_the_claim_and_the_resident_tap_keeps_up` |
| 182-16 | the same withdrawal deleted from `process_bank` | `src/lib.rs` `PreparedTruePeakLimiterBank::process_bank` | `automation_withdraws_the_claim_on_the_bank_path_too` |

### Two legs that a 128-frame quantum cannot reach

`RAMP_UPDATES` is 64 and `HISTORY_WORDS` is 12, so at the quantum every other test in this crate
uses, a de-zipper window is fully consumed inside the block that opened it and twelve stale detector
taps are flushed by the first block that renders after them. Rows 182-9 and 182-13 are therefore red
only at a **short** quantum, and their tests run eight-frame blocks to reach them. The render quantum
is caller-supplied, so neither implication is a property of the effect; both are properties of one
configuration, and a gate that only held at that configuration would be a gate that held by luck.

### Four legs that turn nothing red, and the argument that they are still right

`all_exactly_one` on `required_ring`, on `box_ring` and on `prefix`, and the `box_sum == Wb` test,
can all be deleted with every test in this crate still green. Recorded rather than deleted, because
the reason is a dynamics argument and dynamics arguments are exactly what a bit-identity claim
should not rest on.

The reduction word is always the last thing to settle, by three orders of magnitude. `d = +0.0`
already implies `1 - S/Wb <= 0`, hence `S >= Wb`; every box term is at most `1`, so `S <= Wb`; so
`S == Wb` exactly and every one of the `Wb` terms in the sum is exactly `1.0` — which is every term
the ring will be read for while the claim holds. The ring legs are therefore *implied* for the
purpose of rendering the right samples. Separately, reaching `d = +0.0` at all needs the release to
decay to `FLUSH_EPS = 1e-20`, which is around 218 000 samples at a 100 ms release and four million
at 2 000 ms, against a ring of `R = 481` slots: by the time the recursive word snaps, every slot has
been overwritten with `1.0` hundreds of times.

So the four legs are kept for what the *dynamics* cannot promise: that the skipped block leaves the
arena bit-identical, including the slots outside the current window that a rendered block would have
rewritten and a skipped one does not touch. They cost one linear scan of a settling block, on a path
that is about to replace a whole block of rendering, and they make `is_at_silent_rest` a literal
transcription of `clear_runtime` rather than a subset of it that has to be re-derived every time the
release law changes.

### One leg that is inert today

`self.silent_bypass == self.metadata.bypass` turns nothing red, and cannot: `bypass` is fixed at
preparation and there is no path that changes it, so `silent_bypass` is always equal to it. It is
kept for parity with `miso-engine-compressor` and `miso-engine-parametric-eq`, which carry the same
leg for the same reason, and because it is the leg that becomes load-bearing the day bypass becomes
a per-block flag — at which point every standing claim would otherwise silently survive a change of
kernel arm. One bool compare per block is not where this crate's ceremony budget goes.

`LimiterCore::reset` withdraws the claim and that turns nothing red either, for a different and
weaker reason: both resets land on `clear_runtime`, which *is* the rest state the claim describes,
so a claim that survived a reset would happen to be true. It is withdrawn because the claim is a
statement about a block that was rendered and observed, and it must not come to depend on
`clear_runtime` and `is_at_silent_rest` continuing to coincide.
