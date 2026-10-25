# Ramp a lane's detector link between modes

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-7, D15-13 E4).
Slice L3a of *Let a strip override a console slot's link mode* (#1236).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Each of the four linked effects (compressor, gate-expander, transient shaper, true-peak limiter)
can move one lane's detector link from one mode to another while it renders, over a given ramp,
with no click, without moving a bank-mate's bit, and, for the limiter, without any sample over the
ceiling. The engine entry point exists and is tested; *Carry the link record from the edit to the
lane* (#1371) connects it to edits.

## Context

- After #1368, each lane holds its own link mode, and each bank picks its arm per block from the
  lanes' modes.
- The link laws, in each crate's frozen operation order: `own = |x|`; `maximum = max(|l|, |r|)`;
  `average = 0.5|l| + 0.5|r|` (compressor `link_frame`, `crates/compressor/src/kernel.rs:316-339`;
  gate `kernel.rs:205-207`; transient shaper `link`, `crates/transient-shaper/src/lib.rs:286-300`;
  limiter, `maximum` only, through the per-lane `link` mask, `crates/true-peak-limiter/src/lib.rs:2741`).
- The detector feeds a smoothed gain path in each effect, so the output gain is continuous whatever
  the detector does. D15-1 still requires the switch itself to ramp.
- The limiter's ceiling proof needs `g[n] <= r_own[n - N]` on each channel (module doc,
  `crates/true-peak-limiter/src/lib.rs:11-24`). Its linked-pair fast path depends on the record
  `gain_linked` (`:3452`, decided per block at `:3588-3594`, re-established by `gain_state_agrees`).
- Ramps: `effect_runtime::ramp::LinearRamp` (`crates/effect-runtime/src/ramp.rs`) is the shared
  linear ramp with an exact final assignment; state payload helpers `ramp_path_within`,
  `read_f32`/`write_f32` (`crates/effect-runtime/src/state_payload.rs`).

## Decisions frozen for this slice

- **D1. Entry point.** `PreparedNativeEffect` and `PreparedNativeEffectBank` gain
  `retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool`, render-safe (no
  allocation, no lock, bounded). It returns `false`, changing nothing, when the descriptor's
  `lane_link` is false, the mode is not in `supported_link_modes`, or `track` is out of range. The
  default implementation returns `false`. It applies from the next frame the effect renders.
- **D2. Weights.** Each lane holds, per channel, two `LinearRamp`s: `w_link` (0 = own, 1 = linked
  combination) and `w_avg` (0 = maximum, 1 = average). At rest they are exactly 0.0 or 1.0 and
  equal the lane's mode. `retarget_link` sets both targets over `samples` (0 = a step). A retarget
  during a ramp starts from the current words.
- **D3. Blend.** On a frame where either weight of a lane is ramping, that lane's detector is
  `combined = max + w_avg * (avg - max)`, `d = own + w_link * (combined - own)`, as separate `mul`
  and `add` (no fused multiply-add), with `own`, `max` and `avg` computed in the crate's existing
  order. A bank with any ramping lane runs the blended arm for that segment; a lane at rest inside
  it takes the `select` form from its derived masks, so its bits are today's. When a ramp arrives,
  the lane's mode becomes the target and its masks are rederived; the bank's per-block arm choice
  (#1368 D4) reads the new modes.
- **D4. Limiter safety.** The limiter blends only `w_link` (it has no `average`). The blended peak
  is at least the own peak (`max >= own` and `0 <= w_link <= 1`), so each channel's required gain
  stays at or below its own, and the ceiling proof holds in every window that mixes modes.
- **D5. Limiter agreement** (amended by root ruling, 2026-10-25). On a switch into `maximum`,
  `gain_linked` is false, and every block with a glide in flight clears it. A per-bank countdown
  that starts at the switch covers the glide plus `N + R` frames (raised with `max`, never
  lowered): the frames in which both gain rings are rewritten under the new law. When it ends,
  the gain rings are compared once; if they agree, the `W` reduction words `d` are compared once
  per block (only while the bank stays all-`Maximum` with agreeing designed words and no glide)
  until they agree, and then `gain_linked` is re-established. The fast path never runs on
  disagreeing words.
- **D6. Payload (D15-7).** Each lane's state payload gains its mode and the two ramps (4 words
  each), so a swap carries a link in flight. Restore validates the ramps with `ramp_path_within`
  and the mode against the descriptor. `STATE_LAYOUT_VERSION` stays 1 (nothing persists state; the
  word-count header refuses a stale length). These codecs are stream A's code in the same crates:
  coordinate with the #1279/#1280 owner.

## Effect evidence (AGENTS.md list)

- **Equations:** D3. **Update rule:** linear in the weights over `samples`, exact assignment on
  arrival. **Stability:** the detector is a convex combination of nonnegative magnitudes, so it
  stays in `[own, max(own, max)]`; no recursive coefficient changes. **Latency, tail:** unchanged.
  **Smoothing:** the ramp; the effect's own ballistics follow. **NaN/denormal:** the weights are in
  `[0, 1]` and finite; a NaN detector input is handled as today (D8 clamps, D7 boundary check).
- **Citations:** Giannoulis, Massberg and Reiss, JAES 60(6), 2012 (detector and link);
  ITU-R BS.1770-5 Annex 2 (true peak).
- **Listening:** a blinded A/B of a `dual_mono` to `maximum` switch over the default ramp against
  a step, on a wide stereo bus, recorded in the PR.

## Deliverables

1. D1 in `crates/effect-contract/src/lib.rs`.
2. D2-D6 in the four effect crates.
3. Tests per gate, and an `f64` reference of D3's blend in `crates/dsp-reference`.

## Authorized paths

- `crates/effect-contract/src/lib.rs` (the two trait methods)
- `crates/compressor/`, `crates/gate-expander/`, `crates/transient-shaper/`,
  `crates/true-peak-limiter/` (`src/` and `tests/`)
- `crates/dsp-reference/src/`

## Non-goals

- The control record, the classifier, any host path (#1371).
- The multiband compressor (#1367).

## Objective gates

1. **Blend matches the reference.** Per effect, at `Scalar`, `Simd4` and `Simd8`: a switch in each
   supported direction over 64 samples, started mid-block, on a two-channel signal whose channels
   differ by 20 dB. The detector words match the `f64` reference of D3 within the crate's existing
   SIMD/scalar tolerance, and from the arrival frame on, each detector word equals the target
   mode's law (the `select` form) bit for bit.
2. **Bank-mates keep their bits.** A full bank where one lane switches and the others rest or ramp
   parameters: every other lane is bit-identical to its scalar instance.
3. **Limiter ceiling.** Every switch direction, at every ramp length in {0, 1, 64, 4800}, on a
   full-scale fixture with intersample peaks: no output sample's true peak exceeds the ceiling.
4. **Limiter agreement returns.** After a switch into `maximum` on a uniform bank, the linked-pair
   path (test-support counter) does not run before the glide plus `N + R` frames, runs again once
   the words agree (within a stated bound on stationary `+6 dBFS` noise), and output is
   bit-identical to the per-lane body over the same blocks.
5. **Payload.** A lane snapshotted mid-switch and restored renders bit-identically to the
   uninterrupted one; a forged weight outside `[0, 1]` is refused.
6. **Realtime.** `retarget_link` and a switching block make zero allocations and frees
   (`bench_support::alloc` thread counters, statics warmed).
7. Commands:
   - `cargo test --locked --all-targets -p effect-contract -p compressor -p gate-expander -p transient-shaper -p true-peak-limiter -p dsp-reference --features math/lane,lane/test-support`
   - the touched effects' `KERNEL_ROSTER` rows; the browser legs as in #1368
   - `bash scripts/check-cross-targets.sh`; `scripts/run-aarch64-tests.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
     `cargo fmt --all -- --check`; `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: a blend in the wrong order, or a lane that keeps the old mode after arrival, turns it red.
- Gate 2: a bank-wide ramping arm that blends resting lanes too moves their bits; it turns red.
- Gate 3: a blend that can fall below a channel's own peak overshoots the quieter channel; it
  turns red. No existing test switches a limiter's link.
- Gate 4: an agreement record never re-established loses the fast path for good (counter stays 0);
  one re-established too early renders the mirror on disagreeing words (bits differ).

## Dependencies

- *Lower the link mode to per-lane state in the linked effects' banks* (#1368).

## Decision record (root split and 1370a, 2026-10-24)

- **Split.** 1370a: the trait entry point (D1), the shared glide (D2-D3), the payload (D6) and
  gates 1, 2, 5, 6 in the compressor and the gate-expander, and the `f64` reference
  (`dsp_reference::{ReferenceLinkGlide, reference_link_blend}`, and the reference gate's
  `reference_gate_expander_process_with_link_glide`). 1370b, after #1894 PR 2 merges: the
  transient shaper and the true-peak limiter (D4, D5, gates 3-4). De-esser and dynamic-eq stay out
  (R5, successor issue). No control-row change: the row stays `RebuildOnly` until #1371 (R1).
- **Class A.** Every block with no glide in flight renders main's bits: the glide rides the
  ramping prefix, and only a prefix with a glide in flight takes the blend (`Glide` law, `GLIDE`
  body). The existing bank, padding, swap and digest tests pass unchanged. A glide is the
  feature, not a class-B move.
- **D1 additions.** `samples > LINK_GLIDE_MAX_SAMPLES` (`2^24`, the exact `f32` countdown) also
  refuses; a bank's padded lane refuses. `link_glide_weights` and `link_glide_mode` sit beside
  the trait methods in `effect-contract` as the shared statement of D2's weights.
- **D3 as built.** `retarget_link` sets the lane's mode (and its masks) to the target at once;
  the weights carry the transition. A lane still in flight after a frame's advance takes the
  blend; every other lane, the arriving one included, takes the `select` form. A blend at rest
  weights equals the `select` form on finite words, but not on non-finite ones (`0 * inf`), so the
  `select` is load-bearing.
- **Mode as a default.** The retargeted mode is the lane's mode: a full reset keeps it and seats
  the weights at rest, a discontinuity reset snaps a glide to it, a lane swap moves it (and rests
  the weights until the swap's payload restore brings a glide), and the D7 recovery leaves a glide
  alone.
- **D6 as built.** Each channel section gains the two weights (8 words; compressor 37 -> 45,
  gate 30 -> 38). The mode is the weights' targets (`link_glide_mode`), so it takes no word.
  Restore refuses (`effect.state.link`) a weight outside `[0, 1]`, `-0.0`, a moving weight with a
  zero step, targets that are no supported mode's, or channels that disagree. A payload whose mode
  is the lane's own commits the glide; one of another mode (today's plan-swap carry into a
  successor prepared with a new mode) leaves the lane's own mode at rest, as before the glide.

- **Gate ramp clock (attempt 2).** A parameter retarget (`apply_automation`) and a rate
  retarget (`retime_rate`) each raise the gate's bank clock to at least 64 samples (`max`), never
  lower it: before, `= RAMP_SAMPLES` cut a longer glide (its weights froze mid-flight, at 4,608
  samples left in the verifier's probe). The oracle's 4,800-sample case sends both a threshold
  span (re-sent at its value) and an attack span on every lane in one block, and is red with
  either site alone reverted to `= RAMP_SAMPLES`: `apply_automation` red, `retime_rate` red. Bits
  are unchanged for every block without a glide: before #1370 the clock never held more than 64.
- **Collapsed body.** It has no glide test of its own: on a collapsed lane `L = R`, so the three
  link laws, and every blend of them, give the same level.
- **Known limit for #1371.** A mid-glide plan-swap carry into a successor prepared at another
  mode seats that mode at rest (D6 above): a step, correct while a link change is rebuild-only.
  #1371 must either prepare the successor at the lane's carried mode and retarget after
  adoption, or keep the carried words and retarget them to the lane's mode.

## Test value (1370a)

- `glide_tests::a_glide_tracks_the_f64_blend_into_the_target_law` (both effects, f32/Simd4/Simd8):
  a blend with its weights swapped, a glide the ramping prefix does not cover, or a retarget that
  leaves the old masks renders away from the `f64` model. Its longer glide crossed by a parameter
  ramp on every lane (gate 4,800 samples with a threshold and an attack span, compressor 300 with
  a makeup ramp) turns red on a parameter or rate retarget that cuts the glide (the gate's clock,
  red at each site), and its compressor sidechain leg on a glide that skips the sidechain body; each
  case also asserts the glide arrives exactly.
- `glide_tests::an_out_of_range_glide_is_refused_and_changes_nothing` (both): a dropped `2^24`
  bound, or a padded lane or scalar track 1 accepted, returns `true` or moves a payload word.
- `dsp-reference` `link_glide::tests::rest_weights_give_the_three_link_laws_and_the_glide_arrives_exactly`:
  a reference whose rest weights do not give the three laws, or whose glide arrives a frame late,
  would mislead both effects' oracle tests in the same way as a matching production defect.
- `kernel::glide_law_tests::a_glide_lands_on_its_target_law_and_a_resting_lane_never_leaves_its_own`
  (both): a blend applied on the arrival frame or to resting lanes turns a partner's `inf` into
  NaN where the `select` form would not.
- `glide_tests::a_glide_moves_no_bank_mates_bit_and_allocates_nothing` (both, gates 2 and 6): a
  glide body that picks the wrong output arm or skips a bank-mate's parameter ramps, a retarget
  that reaches other lanes, or a heap call in the retarget moves a bank-mate's bit or the counter.
- `glide_tests::a_lane_snapshotted_mid_glide_continues_bit_for_bit_and_forged_weights_are_refused`
  (both, gate 5): a restore that drops the weights, adopts them under another mode, keeps a prior
  glide, or admits a weight outside `[0, 1]` turns it red.

## Decision record (1370b, 2026-10-25)

- **Transient shaper (D2-D3, D6).** Each channel's lane holds `w_link` and `w_avg` as
  `LinearRamp`s beside its parameter ramps (`Ramps::links`); the glide rides the ramp prefix
  (`Ramps::prefix` counts the weights). A block with a glide in flight renders that prefix through
  one glide body (`Shaper::run_glide`, outside the four per-arm `run`s): the weights are gathered
  as vectors, each lane's detector is the `LINK_MIXED` select form, and a lane still in flight
  takes the blend. The rest of the block, and every block with no glide, renders through the arm
  as before. Payload 14 -> 22 words per channel (`LINK_WORD = 14`); the 1370a restore rule and
  refusals (`effect.state.link`; a settled weight must also hold its target). No bank clock: the
  prefix is read from the ramps, so no parameter span can cut a glide.
- **True-peak limiter (D2-D4, D6).** One weight per channel (`ChannelState::link`, inline, no
  heap); the payload gains its quadruple after the history (`LANE_HEADER_WORDS` 27 -> 31, lane
  words `3N + 39`). The glide's frames of a block render through the per-lane body with a `GLIDE`
  const (`limiter_block_glide`, ramping dispatch, which is the identity on a resting ramp) and the
  rest of the block through today's dispatch. `glide_peak`: `P = own + w_link * (max - own)`, the
  select form on resting and arriving lanes. D4 holds by construction (`max - own >= 0`, so the
  blend is at least `own`). The collapsed body has the same `GLIDE` const (`max(p, p)`, the dual
  body's bits on `L == R`). A glide in flight is never quiet and never at rest.
- **D5 as built, and a finding against its "once".** A switch (a retarget that changes the mode
  or leaves a weight in flight, or a restore that brings a glide) clears `gain_linked` and raises
  `relink_frames` with `max` to the glide plus `N + R`, not `R`: a box term at frame `n` reads
  `r[n-N ..]`, so the box ring is rewritten under the new law only `N + R - 1` frames after the
  glide. Every rendered, skipped (`advance_clocks`) or silent frame counts it down; at zero on a
  dual path, `gain_state_agrees` re-derives the record once. Every block with a glide in flight
  also clears the record: a whole-bank D7 reset mid-glide restates it from cleared words while
  the glide continues (gate 4 crosses that reset).
  **Finding:** the reduction word `d` is recursive, and on stationary hot material it does not
  re-agree within any ring count: measured on `+6 dBFS` noise on both channels at W8, one lane's
  `d` stayed apart from block 130 to past block 169 (1,920 frames after the countdown), while the
  rings, prefix and box sums agreed. With the "once" rule such a bank keeps the per-lane body
  until the next reset, restore or disengage (a cost, never a wrong bit). Gate 4 therefore uses a
  burst after the glide whose attack makes `d` agree before the check.
  **Root ruling (2026-10-25), built:** "once" is replaced by a watch. At the countdown's end the
  rings (`gain_rings_agree`: both gain rings, prefix, box sum, phase) are compared once; if they
  agree and `d` does not, `relink_watch` is set, and each dual block that keeps the rings equal
  (every lane `Maximum`, designed words agreeing, no glide; steps 1-5 never read `d`) compares
  the `W` reduction words, and on their first agreement re-derives the record with the full
  `gain_state_agrees`. Any other block, a collapsed block, a reset, a restore, a D7 recovery or a
  new switch ends the watch. Class A: `gain_linked` only selects between bodies that give the
  same bits (the `LinkedPair` reference arm proves it per block).
- **Not built here:** no bank clock exists in either effect that a parameter or rate retarget
  writes; the 4,800-sample oracle cases still cross a parameter span (shaper: attack amount;
  limiter: ceiling and release) on every lane.

## Test value (1370b)

- `glide_tests::a_glide_tracks_the_f64_blend_into_the_target_law` (shaper; scalar, Simd4, Simd8):
  swapped weights, a prefix that ignores the weights, or a retarget that keeps the old masks
  renders away from the `f64` shaper fed the reference blend. Limiter twin
  (`tests::glide::...`): a skipped glide prefix or kept old mask leaves the `f64` limiter.
- `tests::glide::no_switch_puts_a_sample_over_the_ceiling` (limiter, gate 3): a blend that can
  fall below a channel's own peak (red: blending toward the partner's peak) puts a sample over
  the ceiling.
- `tests::glide::the_linked_pair_returns_once_the_words_agree` (gate 4, attempt 2 folds the
  burst test into it and deletes that test): on `+6 dBFS` stationary noise with no burst, a
  countdown written without `max` (bank-mates switch during lane 0's long glide), one that covers
  `R` and not `N + R`, a missing ring check, a glide block that does not clear the record after a
  whole-bank D7 reset, or a single check with no watch (the "once" code: "the linked pair never
  returned") is red. Measured return after the countdown: 47 / 47 / 1,247 frames at W1 / W4 / W8.
  Its "within one block" clause is gone: under the watch the return is bounded (4,800 frames),
  not tied to the countdown's block.
- `tests::glide::a_collapsed_block_advances_a_glide_...` (limiter) also holds that a glide in
  flight is never quiet, dual and collapsed: red on `glide == glide` in either quiet test.
- `glide_tests::a_silent_bank_never_skips_a_glide` (shaper): red when `channel_rest_lanes` ignores
  the weights; the shaper's collapsed test asserts `channels_agree` after the disengage (red when
  `desymmetrize` drops the weights).
- `a_glide_moves_no_bank_mates_bit_and_allocates_nothing` (both, gates 2 and 6): a retarget that
  reaches other lanes or a heap call moves a bank-mate's bit or the counter.
- `a_lane_snapshotted_mid_glide_continues_bit_for_bit_and_forged_weights_are_refused` (both,
  gate 5): a restore that drops the weights or adopts them under another mode is red; forged
  words are refused.
- `an_out_of_range_glide_is_refused...` (both; the limiter also refuses `Average`): a padded lane
  or a dropped `2^24` bound accepted is red.
- `a_glide_lands_on_its_target_law_and_a_resting_lane_never_leaves_its_own` (both): a blend on
  resting or arriving lanes turns a hostile partner word into a NaN.
- `a_collapsed_block_advances_a_glide_as_the_dual_block_on_equal_channels` (both): a collapsed
  block that skips the glide parts from the dual block on `L == R`.
