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
