# Prove a live EQ band enable or kind change end to end and across every kind pair

Filed by root from #1337's PR-b note (backlog sprint, 2026-10-24). #1337 (PR-a) made a parametric EQ band's `enabled` and `kind` live.

Three deliverables on top of PR-a's live
  EQ band `enabled`/`kind`:
  1. *C ABI end to end (gate 1).* In `crates/capi/src/runtime/live_tests.rs`, on `eq_session`, one
     transaction enables band 2 of the console EQ and switches the eq-insert's band 1 from bell to
     notch. It must commit as live (no plan replacement). After the 64-sample ramp, with both sides
     driven from silence to a common rest, it must render bit-identical to a fresh plan of the
     committed model. Red if a host path still classifies the rows as prepared, or if any target
     refusal remains.
  2. *Contractive-ramp grid (gate 3)*, in `crates/parametric-eq/tests/`. Use every ordered pair of
     the 7 band states (disabled and the six families), so enable<->family pairs are included.
     Grid: frequencies {20, 1000, 18000} Hz, gains {-24, 0, 24} dB, Q {0.1, 0.707, 18}, at each
     launch rate, plus at least one retarget in mid-ramp. Every word set of the `f32` walk must pass
     `validate_svf_within(.., RAMP_PATH_NORM_TOLERANCE)` or be `IDENTITY`. For a full-scale noise
     input, the output must stay finite and below `OUTPUT_MIX_BOUND` times the input peak. The
     argument is the convexity bound in D3.
  3. *Listening note* in the PR, before (rebuild) and after (live), on a music stem: a 1 kHz
     +9 dB bell toggled on and off; the bell switched to a notch; an 80 Hz HPF re-enabled after
     2 s off.

## Paths

`crates/capi/src/runtime/live_tests.rs` (one test), `crates/parametric-eq/tests/**`, this spec.

## Decision record (attempt 1, 2026-10-24)

- Scope note `scope/1859.md` governs. Paths widened by root: `crates/parametric-eq/src/lib.rs` for
  R1 only (10 lines, `test-support`: `test_only_ramp_word_is_admissible` wraps
  `validate_svf_within(.., RAMP_PATH_NORM_TOLERANCE)`; `TEST_ONLY_OUTPUT_MIX_BOUND`), so the grid
  uses the production predicate and does not re-spell it.
- R2: gate 1 at 48 kHz only (one host parity row); gate 3 at all four launch rates.
- R3: band 2's gain (id 20, -6 dB) rides in the gate-1 transaction so the enable is audible.
- Listening (root, feature-sprint mode): a measured before/after note from a scratchpad-only harness
  suffices; the human-listening question is logged for the owner and does not block.

## Evidence (attempt 1)

- Gate 1: `capi` `runtime::live_tests::a_live_eq_band_enable_and_family_change_renders_the_prepared_plan_bits`.
  One transaction (eq4 console `eq`: id 17 = 1, id 20 = -6 dB; last track's `eq-insert`: id 2 = 6,
  notch) commits revision+1, same epoch, nothing pending; with the source `-0.0` until the ramp
  window has passed, every later block is bit-equal to a fresh plan of the independently built
  committed model, and differs from the unedited plan.
- Gate 3: `parametric-eq` `tests/contract.rs`
  `every_live_enable_or_family_ramp_is_contractive_and_bounded` (`test-support` only). 49 ordered
  pairs x 3 frequencies x 3 gains x 3 Q x 4 rates = 5,292 points, each with a pinned lane walk at
  frame 17 and a mid-ramp retarget to the next state of the seven-cycle; 3.0 s in debug.
- Mutants (each restored, `git diff` clean):
  - gate 3: `RAMP_PATH_NORM_TOLERANCE` tightened to `NORM_TOLERANCE` turns it red (44.1 kHz,
    18 kHz, -24 dB, Q 0.1, HPF -> bell): the grid reaches ramp words beyond a settled design's bound.
  - gate 3, review fold: the retarget walk is pinned to the lane at frame 49 too. The verifier's
    mutant (`start_ramp` takes `current` from the old target, not the coefficient) turns that pin
    red (44.1 kHz, 20 Hz, -24 dB, Q 0.1, disabled -> bell -> next). The first walk is checked
    whole on purpose: without a retarget the lane renders all 64 word sets.
  - gate 1: `section_touched` ignoring a general band's kind field (`control.rs`) turns it red
    (in-crate, `bank.rs` `live_enable_and_family_edits_stay_on_their_lane` also does); host-core
    `apply_edit` dropping Boolean/Enumeration edits turns it red (with the HPF-enable C ABI row).
- Listening note (scratchpad harness, not committed; repo `mix` stem from
  `docs/handoffs/control-smoothing-defaults/measure/src/material.rs`, 48 kHz, 4 s). Before = a
  fresh instance at the new state from the block boundary (the rebuild); after = the live
  prepared-target ramp. `r = y - (20 ms raised-cosine crossfade of the two steady outputs)`; step =
  largest |r[n]-r[n-1]| in the 64 samples after the switch (both lanes); hf = energy of r's first
  difference over [-10, +50) ms relative to the reference's own.

  | switch | before step / hf | after step / hf |
  |---|---|---|
  | 1 kHz +9 dB bell on | 0.0064 / -16.3 dB | 0.0048 / -16.8 dB |
  | bell off | 0.0248 / -24.5 dB | 0.0070 / -25.6 dB |
  | bell -> notch | 0.0338 / -10.6 dB | 0.0094 / -11.4 dB |
  | 80 Hz HPF on after 2 s off | 0.0007 / -32.0 dB | 0.0002 / -32.3 dB |

  The live ramp cuts the switch's own sample step 1.3x to 3.6x; the 50 ms broadband residual moves
  0.3 to 1.1 dB, because a 64-sample ramp is still a fast change against a 20 ms crossfade. No
  human listening row was run.
