# Ramp the detector link of the de-esser and the dynamic EQ between modes

Filed by root as #1370's R5 successor (backlog sprint, 2026-10-24).

## Context

#1370 adds `retarget_link` and the shared detector-link glide for the compressor, the gate-expander, the transient shaper and the true-peak limiter. The de-esser and the dynamic EQ are also `lane_link: true` with `LinkModeSet::ALL`, so a live link change (#1371) needs the same glide in them.

## Smallest closable slice

1. `retarget_link` and the glide (w_link, w_avg per lane and channel, the D2 blend, exact arrival, mul plus add) in both effects, through the shared helpers #1370a added in `effect-contract`.
2. The D6 payload words and the restore rule as #1370a ruled them.
3. Per effect: the f64-oracle glide test, the exact-arrival and resting-lane law test, the bank-mate test and the refusal test, in the form #1370a used.

## Order

After #1370 (both slices). Before #1371 flips the link row to live for these two effects.

## Paths

`crates/de-esser/`, `crates/dynamic-eq/` (`src/` and `tests/`), `crates/dsp-reference/` if the oracle needs a new law, this spec.

## Decision record (attempt 1, 2026-10-25)

- **Shape (root, binding): 1370a/1370b's.** `retarget_link` on the scalar instance (track 0
  only) and the bank (a padded lane refuses), per-channel `w_link`/`w_avg` as
  `effect_runtime::dynamic_band::WordRamp`s beside the live words (`Chan::links`), set through
  `LinearRamp::set_target` and `effect_contract::{link_glide_weights, link_glide_mode,
  LINK_GLIDE_MAX_SAMPLES}`. Refusals: `2^24 + 1` samples, a padded lane, a lane past the width,
  scalar track 1, an unsupported mode, `lane_link` false (both effects support every mode and
  link per lane, so the last two have no input that reaches them).
- **D3 as built.** The shared `detector_level_db` (effect-runtime, not in this issue's paths) is
  untouched. Each kernel's `detector_level::<GLIDE>` computes the blend (`max`, `avg` in the shared
  order, then `combined = max + w_avg (avg - max)`, `d = own + w_link (combined - own)`, separate
  `mul` and `add`) and feeds a gliding lane's `d` into `detector_level_db` as a dual-mono word
  (its masks cleared on that lane), so the dB conversion is the shared one; a resting or arriving
  lane passes today's operands and masks, so its bits are the non-glide body's. The de-esser
  advances the weights in `back`; the dynamic EQ advances them once a frame in `open` and all
  four bands read them. Only the ramping prefix of a block with a glide in flight takes the
  `GLIDE` body (two more `segment` instantiations per shape); every other block is main's code.
  The split arm (#1984, an open owner question) is not touched.
- **Bank clock.** `retarget` (a parameter span) now raises `ramp_frames_left` with `max`, never
  lowers it (the 1370a finding); class A: before this issue the clock never held more than 64.
- **Rest.** `channel_rest_lanes` counts the weights' `remaining`, so a glide in flight is never
  at rest, never takes the silent path and is never skipped (`advance_clocks`); it rests again on
  arrival. The disengage copy (`desymmetrize_channels`) copies the whole `Chan`, weights
  included; `channels_bit_equal` and `designed_channel_symmetry` compare them.
- **Mode as a default** (1370a's rule). A full reset and preparation seat the weights at the
  lane's mode; a discontinuity reset snaps a glide to its target; a lane swap
  (`exchange_lane_defaults`) rests them at the incoming mode; the D7 recovery
  (`reset_lane_full` of one channel) leaves a glide alone (`set_lane_words` keeps the weights).
- **D6 as built.** Each channel section gains the two weight quadruples at its end
  (`STATE_LINK_WORD = CHAN_WORDS + PARAMETER_COUNT`): de-esser 65 -> 73 words, dynamic EQ
  350 -> 358. Restore refuses (`effect.state.link`) a weight off `[0, 1]` on its path, `-0.0`, a
  moving weight with a zero step, a settled weight off its target, targets that are no supported
  mode's, or channels at two modes; a payload of the lane's own mode commits the glide (and
  raises the clock with `max`), one of another mode leaves the lane's own mode at rest.
  `STATE_LAYOUT_VERSION` stays 1.
- **Oracle.** `dsp-reference`'s `ReferenceDeEsser` and `ReferenceDynamicEqBand` split `process`
  into halves (`tap`/`taps`, `finish`), the same operations, so a test drives two channels with
  `reference_link_blend` between them. `bind_bank` gains the gate's `NATIVE_ONLY` switch so the
  unit tests bind `Simd4` and `Simd8` on one host.
- **Class A.** No retarget, no moved bit: every existing de-esser and dynamic-eq test, the
  graph compiler's `bank_levels` and effect-compiler pass unchanged.

## Test value (attempt 1, both crates, each red first on a mutant)

- `glide_tests::a_glide_tracks_the_f64_blend_into_the_target_law` (scalar, Simd4, Simd8; 64 and
  4,800 samples, each crossed by a range span on every lane that moves no model word): a
  parameter retarget that cuts the glide (`= RAMP_SAMPLES`, red on the 4,800 case), swapped
  weights, or (dynamic EQ) weights `open` never advances leave the `f64` model or the exact
  arrival. Measured worst deviation `8.9e-7` (de-esser), `9.3e-7` (dynamic EQ), bound `1e-5`.
- `kernel::glide_law_tests::a_glide_lands_on_its_target_law_and_a_resting_lane_never_leaves_its_own`:
  a blend on resting or arriving lanes turns a hostile partner word into NaN.
- `glide_tests::a_glide_moves_no_bank_mates_bit_and_allocates_nothing` (gates 2 and 6): a
  retarget that reaches lane 1, or a heap call in the retarget, moves a bank-mate's bit or the
  counter (the target mode is neither lane 0's nor lane 1's).
- `glide_tests::a_lane_snapshotted_mid_glide_continues_bit_for_bit_and_forged_weights_are_refused`
  (gate 5): a restore that drops the weights, adopts them under another mode, admits a settled
  weight off its target or channels at two modes is red.
- `glide_tests::an_out_of_range_glide_is_refused_and_changes_nothing`: a dropped `2^24` bound, a
  padded lane accepted or scalar track 1 accepted is red.
- `glide_tests::a_silent_bank_never_skips_a_glide`: a rest mask that ignores the weights is red.
- `glide_tests::a_collapsed_glide_matches_the_dual_glide_and_survives_the_disengage`: a disengage
  copy that drops the weights, or a collapsed block that skips the glide, parts from the dual
  bank once the channels differ.
