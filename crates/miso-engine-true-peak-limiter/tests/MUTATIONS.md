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

## Round 2 — the de-bookkeeped uniform frame loop and the resident detector history

Two class-A kernel changes, neither of which moves a rendered bit.

**R1** takes the bookkeeping out of the uniform-cohort frame. `prefix` and the van Herk `phase`
become block-resident locals, joining the recursive word, the box sum, the detector taps and all
four ramp words that `HotChannel` already held that way; the window minimum is returned as an `L`
instead of round-tripping through a `[f32; 8]` scratch; the three rings become `&mut [f32]` views
cut once per block and addressed with the constant `L::WIDTH` rather than the runtime
`ChannelState::width`; and the frame loop is walked in **wrap-free segments**, so the seven ring
indices are `base + step` inside one instead of a compare and a conditional subtract each. The
per-lane fallback body keeps its arithmetic and its loop structure.

**R2** replaces the detector's `[L; 12]` history with `History<L>`, twelve named fields, because
LLVM idiom-recognises a twelve-word array shift as a block move and the wasm guest was paying a
192-byte `memory.copy` and twelve reloads per frame for it. The tap-major order, the four `+0.0`
accumulators and their twelve separately rounded `add(mul(..))` steps are unchanged.

Every row below is a **bit** test. The claims here are about *when* a state word is written and
*where* it lives, so a gate that compared values with a tolerance would pass every one of them.

| # | mutation | file | test that turned red |
|---|---|---|---|
| round2-1 | the block-end write-back of `prefix` dropped | `src/lib.rs` `limiter_block_uniform` | `a_mixed_lookahead_cohort_falls_back_bit_identically`, `a_restore_that_desyncs_the_phase_falls_back`, `fixed_latency_guarded_ceiling_and_bypass_bits_hold`, `lane_identity_holds_across_widths`, `partition_invariance_holds_over_block_sizes` |
| round2-2 | the block-end write-back of `phase` dropped, so the cohort restarts each block at the phase it entered the previous one with | `src/lib.rs` `limiter_block_uniform` | `a_de_zipper_window_open_across_a_block_boundary_refuses_the_claim`, `a_mixed_lookahead_cohort_falls_back_bit_identically`, `a_negative_zero_input_block_is_not_treated_as_silence`, `a_restore_that_desyncs_the_phase_falls_back`, `a_restore_withdraws_the_silence_claim`, `a_settled_silent_limiter_renders_exactly_the_never_fast_path` |
| round2-3 | `.min(ring - left_end)` dropped from the segment run | `src/lib.rs` `segment` | `the_segment_walk_visits_the_slots_a_frame_at_a_time_walk_visits` — **and nothing else** |
| round2-4 | `.min(ring - left_expiring)` dropped from the segment run | `src/lib.rs` `segment` | `the_segment_walk_visits_the_slots_a_frame_at_a_time_walk_visits` — **and nothing else** |
| round2-5 | `.min(ring - start)` dropped from the segment run | `src/lib.rs` `segment` | six render tests, including `a_settled_silent_limiter_renders_exactly_the_never_fast_path` and `a_restore_that_desyncs_the_phase_falls_back` |
| round2-6 | the main cursor advanced by the run without the segment-end wrap | `src/lib.rs` `limiter_block_uniform` | RED **by hang**: `main - main_cursor` reaches zero, the run reaches zero and the frame loop stops advancing. This is the row `debug_assert!(run >= 1)` exists for |
| round2-7 | the ring cursor advanced by the run without the segment-end wrap | `src/lib.rs` `limiter_block_uniform` | RED by hang, as round2-6 |
| round2-8 | `FrameSlots::advanced` leaves `main_cursor` at the segment's entry value | `src/lib.rs` `FrameSlots::advanced` | `a_de_zipper_window_open_across_a_block_boundary_refuses_the_claim`, `a_mixed_lookahead_cohort_falls_back_bit_identically`, `a_restore_that_desyncs_the_phase_falls_back`, `a_restore_withdraws_the_silence_claim`, `a_settled_silent_limiter_renders_exactly_the_never_fast_path`, `automation_withdraws_the_claim_and_the_resident_tap_keeps_up` |
| round2-9 | `History::shift` written newest-first, so every tap reads a word the shift has already overwritten | `src/lib.rs` `History::shift` | `phase_outputs_match_the_frozen_scalar_order` (E1), `bs1770_annex2_conformance_is_unchanged` (E2) |
| round2-10 | taps 3 and 4 accumulated in the other order | `src/lib.rs` `annex2_phases` | `phase_outputs_match_the_frozen_scalar_order` — the frozen summation order is a bit statement, and swapping two of its twelve steps is visible in the low bit |
| round2-11 | the alignment sample read from tap 0 instead of tap 6 | `src/lib.rs` `detector_peak` | `every_case_has_one_digest_at_every_width` (the E12 `D90_DIGESTS`) |
| round2-12 | the left channel takes the **right** channel's block-end phase: `left.phase.fill(left_phase)` → `fill(right_phase)` | `src/lib.rs` `limiter_block_uniform` | `the_two_channels_of_a_uniform_cohort_keep_their_own_phases` — **and nothing else** |
| round2-13 | the mirror of round2-12: `right.phase.fill(right_phase)` → `fill(left_phase)` | `src/lib.rs` `limiter_block_uniform` | `the_two_channels_of_a_uniform_cohort_keep_their_own_phases` — **and nothing else** |
| round2-14 | the two channels' block-end `prefix` write-backs are swapped | `src/lib.rs` `limiter_block_uniform` | seven tests, including `lane_identity_holds_across_widths`, `partition_invariance_holds_over_block_sizes`, `output_true_peak_never_exceeds_the_ceiling` and `passes_effect_contract_conformance` |

### Why a crossed phase needed its own test and a crossed prefix did not

Rows 1 and 2 gate a **dropped** write-back. Rows 12 and 13 are the **crossed** one, and they were an
open gap until the adversarial verifier's M-A found it: writing the right channel's phase into the
left one survived every other test in this crate while moving rendered bits.

The asymmetry with row 14 is the whole explanation. `prefix` is a running minimum of the signal, so
the two channels hold different values in it on any block carrying material — crossing them is
visible immediately, at any configuration, and seven existing tests see it. `phase` is a *counter*:
it advances one step per frame and wraps at `Wb`, so two channels prepared with the same lookahead
hold the same phase in every block, always, and crossing them is the identity. Every test in the
crate before this one prepares both channels alike. Lookahead is the one parameter that is per
channel *and* sets `Wb` rather than a coefficient, so a left/right lookahead split is the only way
to reach the state where the crossing is observable at all — and it is reachable from the public
parameter surface, which is what makes the gap a defect rather than a curiosity.

Once the split exists, neither of the crate's two standing comparison shapes reaches it either, and
this is why the new test compares two *banks* rather than a bank against scalar twins:

* **cross-width** compares a bank against scalar instances, and a scalar instance is `W = 1` and
  therefore uniform by construction — it runs the same crossed write-back. Both widths corrupt
  identically and agree.
* **partition invariance** compares one long block against several short ones. The uncorrupted
  right phase is `frames mod Wb_right` at any shared block boundary whatever the partition was, and
  the corrupted left phase inherits it and re-syncs there. Both partitions corrupt identically and
  agree.

The oracle therefore has to be a rendering of the same asymmetric configuration that does not run
the uniform write-back at all. The per-lane fallback body is exactly that: it writes each lane's
phase from that lane's own `sliding_minimum`, per channel, and shares no code with the crossed
line. The test's oracle arm is a W8 bank whose lane 7 carries a third, different *left* lookahead,
which makes `lanes_uniform(left)` false and sends the whole bank down the fallback; lanes 0 through
6 are prepared identically in the two arms and must agree to the bit.

### Why rows 3 and 4 have exactly one gate, and why that is the point

Dropping either clamp lets a segment run past the frame at which that channel's window end or box
slot wraps, so the walk addresses `end + step` with `end + step >= R` — a slot of the *next* lap
of the ring, or past the view entirely. Nothing in the render suites catches it, and the reason is
arithmetic rather than luck: at the shapes those suites build, the write cursor's own wrap (or the
end of the block) always arrives first, so the dropped term is never the minimum. The window end
is `Wb` slots ahead of the write cursor and the box slot `R - Wb` behind it, so which of the seven
wraps first is a function of the cohort's lookahead and of where in the ring the block starts —
and a fixed corpus visits a fixed handful of those.

`the_segment_walk_visits_the_slots_a_frame_at_a_time_walk_visits` sweeps them instead: every
launch rate, the boundary lookaheads (zero, the `MINIMUM_RAMP_WINDOW` clamp, `N - 1` and `N`,
where `Wb == R` collapses the window end onto the write cursor and the box offset to zero), and
cursor positions at both ends of both rings. It compares the segment walk's slot sequence against
a frame-at-a-time oracle written with `%` rather than with `wrapped`, so it is an independent
formulation and not the same conditional subtraction compared with itself, and it walks both
channels together because their window shapes differ and their wrap points interleave. That a
render test cannot reach these two rows is the argument for the test existing, not against it.

### A note on the two hangs

Rows 6 and 7 are recorded as RED by hang rather than by assertion because that is what they do: an
unwrapped cursor drives `ring - ring_cursor` or `main - main_cursor` to zero, the run to zero, and
the `while frame < span` loop never advances. In a debug build `debug_assert!(run >= 1)` fires
first and says so, which is why that assertion is in the source; in a release build the mutation
would spin. Recorded as-is rather than softened, because "the gate is a hang" is a weaker gate
than "the gate is an assertion" and the reader should know which one this is.

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

## Mono-collapse M2 — the collapsed kernel and the disengage copy

Driver: one mutation at a time, `cargo test -p miso-engine-true-peak-limiter --test mono_collapse`,
tree restored between rows.

| # | mutation | file | test | result |
|---|---|---|---|---|
| M2-L1 | `ChannelState::copy_state_from` drops `history` | `true-peak-limiter/src/lib.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED |
| M2-L2 | `ChannelState::copy_state_from` drops `main_ring` | `true-peak-limiter/src/lib.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED |
| M2-L3 | `ChannelState::copy_state_from` drops `required_ring` | `true-peak-limiter/src/lib.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED |
| M2-L4 | `ChannelState::copy_state_from` drops `box_ring` | `true-peak-limiter/src/lib.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED |
| M2-L5 | `ChannelState::copy_state_from` drops `reduction` | `true-peak-limiter/src/lib.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED |
| M2-L6 | `ChannelState::copy_state_from` drops `box_sum` | `true-peak-limiter/src/lib.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED |
| M2-L7 | `ChannelState::copy_state_from` drops `limit` | `true-peak-limiter/src/lib.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED |
| M2-L8 | `ChannelState::copy_state_from` drops `release` | `true-peak-limiter/src/lib.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED |

Three properties of the test corpus are what make those eight rows red, and each was arrived at by
watching a row stay green first:

* the ceiling is retargeted **below the signal**, so `required_ring` is not uniformly `1.0` and the
  twelve oversampling history taps actually decide an output sample. Against a corpus under the
  ceiling, `history` is green;
* the content carries a per-block **envelope**. A steady loud signal drives the release recursion to
  a fixed point within a few blocks, and a frozen recursive word that has converged to the same
  value as a live one is a state difference no output can show. Against steady content, `reduction`
  is green;
* the release retarget is **away from the descriptor default**. Against the default value the span
  is a no-op and `release` is green.

`prefix`, `phase`, `lookahead_ms` and `lane` are on the copy list and are not individually red, and
the two pairs are not the same kind of gap. `lookahead_ms` and `lane` are the prepared window shape
and no rendered block writes them, so nothing *can* make them diverge on a bound bank. `prefix` and
`phase` are genuine running state -- `UniformHot::new` loads them and the block write-back stores
them -- and a collapsed block advances only the left channel's, so a divergence ought to be
constructible and this corpus does not construct one. An earlier draft of this row claimed they were
re-derived at the next van Herk block boundary; that is not true of either word and the claim is
withdrawn rather than repaired. The gap is recorded, not explained.
