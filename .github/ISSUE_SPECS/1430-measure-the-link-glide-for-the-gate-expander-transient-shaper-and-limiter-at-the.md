# Measure the link glide for the gate-expander, transient shaper and limiter at the ramp defaults

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-13 E5).
Successor of #1055 (root, 2026-10-05, from #1055). Code anchors verified on `codex/d15-stream-e` at `7d67810a6`.

## Product outcome

`docs/handoffs/control-smoothing-defaults/FINDINGS.md` says, for the gate-expander, the transient
shaper and the true-peak limiter, whether a detector link change over `faderMs` (20 ms by default)
clicks no more than the mute baseline, or whether the link needs another key. #1055 answered this
for the compressor only (`FINDINGS.md` 9.5); 9.7 lists the other three as not verified.

## Context

- **#1055's method** (`FINDINGS.md` 9.5, `data/link_glide.csv`). The compressor is prepared once
  with `dual_mono` and once with `maximum`; the switch is emulated as the bypass crossfade's output
  crossfade (9.4), `dual_mono` as dry and `maximum` as wet. Materials `mix`, `mix-wide` and
  `bass-kick`; ramps 0, 2, 5, 10, 20 and 50 ms (`LINK_RAMPS_MS`,
  `docs/handoffs/control-smoothing-defaults/measure/src/live_rows.rs:49`); the four launch rates;
  `link_glide`, `live_rows.rs:669-715`. 9.5 argues that the emulation bounds the real glide because
  the compressor's curve and its attack/release smoother act after the blend.
- **Why the bound does not carry** (9.5). Below its threshold the gate-expander's gain is the
  detector raised to the power `ratio - 1` (up to 19), clamped by its range, against the
  compressor's `1 - 1/ratio`; a link change can also move a lane across the gate's open/close
  threshold; its attack is 1 ms by default and 0.1 ms at least. The transient shaper's and the
  limiter's gain paths follow their own detector laws.
- **The link code on this tree.**
  - Gate-expander: the link, `crates/gate-expander/src/kernel.rs:202-212`; the detector level in
    dB, `:213-215`; the open, re-arm and hold logic at the threshold, `:217-238`; the curve
    `(ratio - 1)(level - threshold)` clamped to `[-range, 0]`, `:240-245`; the attack/release
    one-pole, `:246-247`. Attack parameter: 0.1 to 50 ms, default 1 ms
    (`crates/gate-expander/src/lib.rs:174-186`). Link modes: all three (`lib.rs:296`).
  - Transient shaper: `link`, `crates/transient-shaper/src/lib.rs:286-301`, feeds both followers
    (fast 0.5 ms / 20 ms, slow 10 ms / 100 ms, module doc `:11-16`); `frame`, `:318-349`, takes
    their ratio as the contrast and shapes the gain; `step`, `:354-368`, links (`:365`) then runs
    `frame` per channel (`:366-367`). Link modes: all three (`lib.rs:182`).
  - True-peak limiter: the per-bank `link` mask from `LimiterCoef::link_max`
    (`crates/true-peak-limiter/src/lib.rs:2741`), applied to the detector peaks at `:2784-2786`
    (the per-lane body; the other bodies at `:3073-3074`, `:3258-3259`, `:3285-3286`); the gain
    law, module doc `:11-24`. Link modes: `dual_mono` and `maximum` only (`LinkModeSet::new(3)`,
    `:296-299`).
- **The harness's reference.** `link_glide` measures against the undelayed source and asserts
  that the compressor adds no latency (`live_rows.rs:677,681`). The bypass rows measure against
  the latency-matched dry signal instead (`shunt_dry`, `live_rows.rs:552-575`;
  `measure/src/effects.rs:324`). The limiter has a lookahead, so the compressor's reference does
  not carry to it.
- **The key.** #1054 D3 and #1370 give the link glide `fader_ms`. #1370's gates check the blend
  against an `f64` reference, bank-mates' bits and the limiter's ceiling; none measures a click.

## Decisions frozen for this slice

- **D1. Method.** #1055's link-glide method: the same harness, rates, ramp lengths (0, 2, 5, 10,
  20 and 50 ms), measures, and output-crossfade emulation between the `dual_mono` and `maximum`
  renders, both directions (`link`, `unlink`). The materials are D2's per effect. The reference
  for every effect is the source delayed by the effect's latency (`shunt_dry`, as the bypass rows
  use), not the undelayed source; the zero-latency assertion stays only for the compressor.
- **D2. Settings and materials.** Each effect at #1055's representative setting (`FINDINGS.md`
  9.4), calibrated under `maximum`, on the materials where the link change moves its gain:
  - gate: closing on the kick (threshold -6 dBFS, ratio 20, range 80 dB, no hysteresis, attack
    1 ms, hold 20 ms, release 50 ms) on `bass-kick`, where the bass lane opens on the kick only
    when linked (`dual_mono` keeps it closed);
  - transient shaper: attack +50 % on `bass-kick` (linked, the kick's transients shape the bass
    lane) and `mix-wide`;
  - limiter: ceiling for 6.0 dB peak reduction under `maximum`, on `bass-kick` and `mix-wide`.
  Every row reports `step_db` (the energy of `maximum - dual_mono` around the switch, relative to
  the programme). A row whose `step_db` is at or below -40 dB (the floor above which 9.4's
  scaling holds) is reported as vacuous and supports no answer; an effect with no non-vacuous row
  gets no answer in D4, and the subsection says so.
- **D3. The bound.** For each effect, state whether the emulation's click bounds the real blend
  (#1370 D3) as 9.5 argues for the compressor, and where it does not (the gate's threshold
  crossing, its steep curve, short attacks), say so and by how much it can be exceeded, or that it
  is not assessed.
- **D4. Answer.** Per effect, from its non-vacuous rows only, each answer stated with their
  `step_db`: `faderMs` (20 ms) is right, or the link needs another key, judged against the mute
  baseline (`data/mute_click.csv`, the 10 ms mute) as 9.5 judged the compressor.
  A key change is root's decision; this slice gives the evidence.

## Deliverables

1. A `FINDINGS.md` subsection under 9.5 with the three effects' results, D3 and D4; 9.7's
   not-verified item updated.
2. A new CSV in `data/` (link_glide.csv's columns plus `effect`); `link_glide.csv` and every other
   committed CSV stay byte for byte.
3. The harness change in `measure/` (the three effects in the link-glide run) and its
   `measure/README.md` lines.

## Authorized paths

- `docs/handoffs/control-smoothing-defaults/` (`FINDINGS.md`, `data/`, `measure/`; not
  `listening/`)

## Non-goals

- Any product code, key or default change (#1054, #1370).
- The real #1370 blend: it is emulated here.
- Listening; a new contrast in `listening/PREREGISTRATION.md`.
- The `average` mode and attack times below the defaults, except as D3's stated bound.

## Objective gates

1. The harness tests pass from `measure/`:
   `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=<scratch>/target cargo test --release --offline`.
2. The new CSV and every committed CSV reproduce byte for byte over two four-rate runs into two
   scratch directories, then `cmp`: `control_smoothing_measure live` for the section-9 CSVs and
   the new one, `control_smoothing_measure measure` for the seven section 5-6 CSVs.
3. `bash scripts/check-workspace-policy.sh`, `bash scripts/check-env-vocabulary.sh` and
   `bash scripts/check-dsp-research.sh` pass.
4. The new subsection gives each effect's 0 and 20 ms results at 48 kHz, the range over the four
   rates, D3's bound, and D4's answer, each traceable to a CSV row.

## Test value

- A harness test that turns red if the limiter's link rows measure against the undelayed source
  instead of the latency-matched one (the lookahead then misaligns the reference with both
  planes, which align with each other by construction). Today's `link_glide` passes the undelayed
  source and is right only because the compressor adds no latency; no test checks the reference.

## Dependencies

- *Research: default ramp lengths for live mute, fader and pan changes (cited, measured, listened)*
  (#1055)

*Ramp a lane's detector link between modes* (#1370) and *Carry the link record from the edit to the
lane* (#1371) use its answer.
