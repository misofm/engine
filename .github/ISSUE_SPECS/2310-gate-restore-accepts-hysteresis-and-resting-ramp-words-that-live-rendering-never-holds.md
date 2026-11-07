# Gate: restore accepts hysteresis and resting-ramp words that live rendering never holds

Tracker: #2258, item FX-7 (gate/expander). Found by the implicit-state-machine audit of
2026-11-06.

## Problem
The gate's hysteresis is two SIMD lane words per channel, `open` (`+0.0` or `1.0`) and `hold` (the
countdown), in `HysteresisState` (`crates/effect-runtime/src/envelope.rs`) and stepped by
`channel_step` (`crates/gate-expander/src/kernel.rs`). The encoding is sound and stays lane words.
Its restore validator, `PreparedGate::parse_lane` (`crates/gate-expander/src/lib.rs`), is looser
than the states live rendering can reach:

- It checks `open` and `hold` separately, so it accepts `open == +0.0 && hold != 0`: a closed lane
  with a countdown. Live rendering never holds one. Closing needs an expired countdown, and only an
  opening reloads it.
- `integral_within` accepts `hold == -0.0`. No writer produces `-0.0` there.
- It does not require a settled parameter ramp (threshold, ratio, range, hysteresis) or a settled
  link weight to sit at its target. The coefficient ramps in the same function already require it.
  Every live writer snaps `current` to `target` when a ramp settles.

The restore is the in-memory plan-swap and lane-carry payload (`snapshot_state_payload` /
`restore_state_payload`, #1278 D3), not persisted state (R6b).

## Contract
- Record, per combination, why live rendering cannot reach it, and what the accepted state renders
  next to the reachable state closest to it.
- `parse_lane` refuses each one through the existing typed refusal path (`StatePayloadError`):
  `effect.state.hold` for the two countdown cases, `effect.state.parameter` for a parameter ramp
  and `effect.state.link` for a link weight. A refused restore changes nothing.
- Every state the effect's own snapshot can carry still restores.

## Non-goals
- The kernel and the lane-word encoding: no render code changes, so no rendered bit moves.
- The same "settled at its target" looseness in other effects' validators, if they have it. It is
  listed as a follow-up and not fixed here.
- Moving the at-target rule into `effect_runtime::state_payload::ramp_path_inside`. That is shared
  by every effect, so it is a cross-cutting change.

## Gates
- Regression tests that are red on main and green after.
- Mutation of the fix: each new clause reverted turns its test red.
- `cargo test --locked -p gate-expander` (its `conformance` and `randomized` suites round-trip
  the effect's own payloads), plus `effect-compiler`, `host-core` and `graph-compiler`, which
  carry gate payloads across plan swaps; clippy `-D warnings`; `cargo fmt --check`; `cargo doc`
  with `-D warnings`; `scripts/check-workspace-policy.sh`; the realtime policy tool (`parse_lane`
  runs in the plan-swap block).

## Evidence (attempt 1)

Base: `main` @ `d441cd408`. Line numbers below are base's.

**Reachability.** These are the writers of the hysteresis words. `open` is written by
`channel_step` (`kernel.rs` ~452), `seed_lane` (`lib.rs` ~683, preparation, full reset and the D7
recovery), `reset_lane_discontinuity` (~796), `commit_lane` (~1481, the restore) and
`desymmetrize` (~1831, a whole-state copy). `hold` is written by the same writers, plus
`retime_hold` (~977).
- `channel_step`: a closed lane that does not open keeps its `hold` (`select(holding, ..,
  hold)` with `holding` false). An opening reloads it to `H`. An open lane closes only when it is
  below the re-arm level and its countdown is `<= 0`, and it keeps that word. The word is never
  negative: `holding` needs `hold > 0` and decrements by one. So a lane closes with `+0.0`, and
  only an opening changes the word after that.
- `seed_lane` and `reset_lane_discontinuity` write `open = 1.0, hold = H`.
- `retime_hold` writes `min(r, H1)` on a closed lane, which is `+0.0` when `r = +0.0`. On an open
  lane it writes `max(0, r + H1 - H0)`, an exact integer sum of non-negative words.
- `desymmetrize` copies the left channel's reachable state.

So `open == +0.0 && hold != +0.0` is unreachable, and so is `hold == -0.0` in either phase.

The resting ramps: every writer of a parameter ramp or a link weight lands a settling ramp on its
target bit for bit. `LinearRamp::set_target` (`effect-runtime/src/ramp.rs` ~90) snaps
`current = target` when it does not arm a ramp. `advance_ramp` (`kernel.rs` ~353) assigns
`target` on the final frame. `seed_lane`, `reset_lane_discontinuity` (~800) and `rest_link`
(~703) write `current = target`. `retarget_link` (~723) goes through `set_target`. So a settled
ramp whose `current` is not its `target` is unreachable. The coefficient ramps' validator already
required this (#1336 D6, ~1405), but the parameter ramps (~1368) and the link weights (~1435) did
not.

**What the accepted states render.** This was checked with a scratch probe through the public
`restore_state_payload` / `process` / `retarget_link` API (deleted, not committed). Each restore
was compared with the reachable state closest to it, on the same material, at 48 kHz, with
10 ms hold and ratio 2:

| accepted on base | against | PCM |
|---|---|---|
| closed, `hold = 480` | closed, `hold = +0.0` | identical (0 samples differ), through quiet and a reopening burst. The snapshot echoes `0x43f00000` until the next opening reloads the word. |
| settled threshold `current -30`, `target -20` | settled `-30 / -30` | identical (0 of 8000 differ). |
| settled link weight `current 0.25` under `Maximum` (`target 1`) | its own snapshot (`1 / 1`) | identical at rest, then **4729 right-channel samples differ** in a 4096-sample glide to `DualMono`. |

The countdown and the parameter `target` are dead words in those phases. No output reads them, so
the accepted states are not canonical, but they are inert. The snapshot carries them forward, and
a future kernel that read them would turn them into audio. The resting link weight is not inert.
The prepared law renders while the weight rests. The next glide then starts from the stale word,
so the lane jumps off the law in force. No reachable state renders that sequence.

**Fix.** This is a validation-only change in `parse_lane`, inside its existing
`REALTIME_POLICY_BEGIN` region: three comparisons and no allocation. Each refusal goes through the
function's existing `state_error` codes, and no new code is added:
- `effect.state.hold`: `is_negative_zero(hold) || (open is +0.0 && hold is not +0.0)`. This is
  checked beside the existing `[0, H]` integral bound.
- `effect.state.parameter`: a settled parameter ramp has `current == target` by bits. This is the
  coefficient ramps' clause.
- `effect.state.link`: a settled link weight has `current == target` by bits.

**No bit moved.** No render code changed: the diff touches only `parse_lane` and its comments.
`kernel.rs`, `envelope.rs`, `run_block`, `apply_automation` and every reset are untouched. So no
no-bit-moved comparison is needed. Every state the effect's own snapshot carries still restores:
- the gate's swap, carry, randomized and conformance suites are green;
- the consumers that carry gate payloads across plan swaps (`effect-compiler`, `host-core`,
  `graph-compiler`) are green.

**Regression tests** (`crates/gate-expander/tests/state.rs`), red on base:
```
thread 'a_settled_ramp_off_its_target_is_refused' panicked at crates/gate-expander/tests/state.rs:922:18:
threshold: ()
thread 'a_hold_countdown_live_rendering_never_holds_is_refused' panicked at crates/gate-expander/tests/state.rs:850:18:
closed, hold 1: ()
test result: FAILED. 7 passed; 2 failed
```
- `a_hold_countdown_live_rendering_never_holds_is_refused`. Defect: a restore that checks `open`
  and `hold` one at a time accepts a closed lane that carries a countdown, or a `-0.0` countdown.
- `a_settled_ramp_off_its_target_is_refused`. Defect: a restore that holds a settled parameter
  ramp or link weight only to its domain accepts one that is off its target.
No existing test covered these. The nearest ones are `a_moving_ramp_word_past_its_domain_is_refused`
(domain only) and `a_coefficient_ramp_in_flight_survives_a_carry` (coefficient ramps only). No test
became useless.

**Mutations** (each clause reverted alone on the fixed tree; `cargo test -p gate-expander --test
state`):

| mutation | result |
|---|---|
| M1: drop `closed && hold != +0.0` | red: `a_hold_countdown…` at `closed, hold 1` |
| M2: drop `is_negative_zero(hold)` | red: `a_hold_countdown…` at `open, hold -0.0` |
| M3: drop the parameter ramp at-target clause | red: `a_settled_ramp…` at `threshold` |
| M4: drop the link weight at-target clause | red: `a_settled_ramp…` at `link weight` |

**Follow-ups (not in scope).** Other effects' restore validators may accept a settled parameter
ramp or a resting link weight off its target. A quick grep finds an at-target clause in
de-esser, dynamic EQ, multiband, limiter and transient shaper, but none in the compressor's
`state.rs`/`lib.rs`. Each needs the same reachability check. Moving the at-target rule into
`effect_runtime::state_payload::ramp_path_inside` would cover every effect at once, but that is
a cross-cutting change.
