# Compressor: run the ramping prefix as the two-pass body with lane-wide ramps

Source: `docs/handoffs/effects-2026-09-27/AUTOMATION-DIAGNOSIS.md`, verified in `VERIFY-AUTOMATION.md`. **The Amendments section at the end supersedes the body wherever they conflict.** Draft names map to issues: automation-5 = #1003, automation-1 = #1004, automation-2 = #1005, automation-3 = #1006, automation-4 = #1007.

**Scope recorded (root, 2026-09-27):** the collapsed (mono) prefix is in scope and is the half that matters (VERIFY-AUTOMATION F4: the prototype's dual-only rewrite gave mono stems nothing). C2b (per-word curve design) is exact by an invariant that holds today but is worth about 1-2 us; it stays out of this slice pending an owner ruling.

Automation follow-up C2 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at `49f696c7`;
every `file:line` is `crates/compressor/src/kernel.rs` on that tree unless named). Evidence:
`docs/handoffs/effects-2026-09-27/AUTOMATION-DIAGNOSIS.md`, sections 2, 3 ("Compressor") and 5
(C2). The prototype is bits 0 and 3 of `compressor::diag::MODE` in
`docs/handoffs/effects-2026-09-27/automation-diagnosis-prototypes.patch` (`ramping_main`,
`ChannelRamps`, `RampVec`, `design_curve`). It is evidence only; do not commit it.

**This supersedes #986** ("make the ramping prefix cost scale with the ramping lanes"). #986 left
its scope to be chosen after a measurement. The measurement and the prototype are here, and they
choose a superset of #986's options (a) and (c). #986's gates are carried over.

## Product outcome

A Point on a compressor parameter opens a 64-sample `Linear 64` ramp. That happens every block
while a knob is dragged or a lane of automation plays. While it is open, `process_block` (`:476`)
runs `frames_loop::<L, true>` (`:681`) for the longest ramp in either channel.

Every frame of that prefix, for both channels:

* `advance_ramps` (`:217`) scans 7 ramps × `W` lanes;
* each moving lane advances in scalar (`LinearRamp::next_value`);
* the static curve is redesigned per moving lane (`design_lane`, `design.rs:156`: two divisions
  and a branch);
* `Coef::load` reloads the words.

The frame law is the general one-pass law. The prefix gets none of #981-#985's two passes, wet arm
or DualMono arm. The collapsed body's prefix (`frames_loop_mono::<L, true>`, `:853`) is the same
loop on one plane.

Measured, in µs per 64-track block, compressor isolate, threshold ridden, Δ against settled:

| arm | native `Simd8` base / C2 | native `Simd4` base / C2 | V8 base / C2 |
|---|---:|---:|---:|
| one lane, a Point per block | +5.8 / **+1.2** | +3.8 / **+1.8** | +6.3 / **+2.6** |
| 8 of 64 tracks | +40.3 / **+8.4** | +30.4 / **+8.8** | +45.0 / **+13.6** |
| all 64 | +79.6 / **+9.9** | +94.4 / **+18.5** | +147.7 / **+28.3** |
| attack, all 64 | +56.9 / **+12.8** | | +112.3 / **+28.8** |
| settled | 25.2 / 26.1 | 48.0 / 47.3 | 62.2 / 62.7 |

A moving bank costs 2.3-2.7x a settled one today, and 4.1x with all eight lanes moving. Lane-wide
ramps alone (C1, one pass) recover about 60 % of that. The two passes recover the rest down to
+1.2 µs per moving `Simd8` bank. These figures are descriptive only.

## Why this is exact

* **The advance.** `RampVec::advance_where(gate)` is `LinearRamp::next_value` as selects, per lane:
  * `remaining == 0`: every word is left as it is, not only at rest. A restored payload may carry
    `remaining = 0` with `current != target`, and `next_value` returns `current` there forever.
  * `remaining == 1`: the target is assigned and the step is set to `+0.0`.
  * otherwise: the step is added once and `remaining` is decremented.

  `remaining` is an exact small integer in `f32` (at most 64, `state.rs` validates this). The rate
  ramps advance only on lanes whose attack or release parameter ramp was in flight, exactly as
  `advance_ramps` gates them.
* **The design.** `design_curve` is `GainComputerCoef::new` on every lane:
  * `1/R - 1` as `1.0 / ratio - 1.0`;
  * the knee pair as `(0.5 * W, 1 / (2 * W))` when `W > 0` and the reciprocal is below `+inf`,
    else `(+0.0, +0.0)`, which is `knee_coefficients` (`effect_runtime::dynamics`) with its
    branch as a select.

  It is written into a lane's words only where a curve parameter of that lane moved this frame,
  which is `design_lane`'s `changed` rule.
* **The passes.** A frame's target depends only on its input and the curve words, and the curve
  words depend only on the threshold, ratio and knee ramps. So pass 1 advances those ramps and
  computes the chunk's targets. Pass 2 advances attack, release, makeup and mix and runs the
  recurrence and the output. Each ramp advances once per frame, in the pass that reads it. The
  order across independent frames is #983's.
* **The arms.** DualMono detection is #984's. The wet arm (#982) is taken only when `mix == 1` on
  every lane of both channels, the block is unbypassed, and no mix ramp is open on either channel.
  Makeup may move: the arm's argument holds per frame for any makeup.

## Invariants

* **Class A.** Every output word, recursive word, coefficient word, ramp field (`current`,
  `target`, `step`, `remaining`, parameter and rate ramps), report and payload is unchanged, at
  `f32`, `Simd4` and `Simd8`, dual and collapsed.
* The only relaxation is the existing one: the wet arm's signalling-NaN quieting, in blocks that
  `finish_channel` rejects.
* `Detector::Silent` and `Sidechain` keep `frames_loop::<L, true>` unchanged. The sidechain body is
  #995's.
* Allocation-free. Two `ChannelRamps` on the stack (at most 9 × 4 vectors plus 8 words per
  channel) and the settled body's existing 2 KiB target scratch. No `unsafe`, and no state-layout
  change.

## Interface contract

1. `struct RampVec<L>` with `gather`, `scatter` and `advance_where`, and `struct ChannelRamps<L>`
   with `gather` (moving parameters, the three curve currents when any curve parameter moves, and
   the words), `scatter`, `advance_curve`, `advance_output`, `curve` and `coef`.
2. `fn design_curve<L>(threshold, ratio, knee) -> GainComputerCoef<L>`, as above.
3. `fn ramping_main<L>(..)` for `Detector::Main`, dual: the settled body's arm selection and
   `ramping_frames::<L, DUAL_MONO, WET>`.
4. `fn ramping_main_mono<L>(..)`: the same on the collapsed plane, replacing
   `frames_loop_mono::<L, true>` for `Detector::Main`.
5. `curve_target` takes the curve coefficients rather than a whole `Coef`, or pass 1 builds a
   `Coef` whose other fields it never reads. The prototype did the latter. Prefer the former.
6. `frames_loop::<L, true>` remains for `Silent` and `Sidechain`. C1's one-pass loop is not kept:
   the prototype carried both and the artifact grew by 56 KB.

## Smallest closable slice

Authorized paths:

* `crates/compressor/src/kernel.rs`;
* `crates/compressor/tests/` (the scenario);
* `crates/compressor/tests/MUTATIONS.md`;
* `crates/compressor/tests/bench_ramp.rs` (#986's step 1 arms: `threshold_one_lane` and
  `threshold_all_lanes`, frozen before timing);
* this spec, and #986's.

Steps:

1. **On the base:** write gate 3's scenario and pin its digests.
2. Contracts 1-6.
3. The gates, and the evidence.

## Optional within the slice: C2b, per-word curve design

Each curve word depends on one parameter: the threshold word on threshold, `1/R - 1` on ratio,
the knee pair on knee. Rewrite only the words whose own parameter moved on that lane. For a
threshold ride this removes both divisions per frame. Measured, 8 of 64 tracks: `Simd4`
+8.3 → +6.1 µs, and `Simd8` +9.7 → +8.9 µs.

It is exact under the invariant *`words[0..4]` is `design(ramps[0..3].current)` on every lane*.
Preparation, both resets, `restore` (via `redesign`) and every ramp frame maintain it. It is
class A by an invariant, not by construction, so it needs the owner ruling in diagnosis section 8,
item 2. Without that ruling, leave it out. With it, add a debug assertion of the invariant after
every block, and a mutation that breaks one maintenance site.

## Non-goals

* Changing the `Linear 64` law, the window, or the per-frame update rate of the curve. Those are
  class B and need a ruling.
* The settled body, and the sidechain body (#995).
* Deferring the per-event `f64` `rate_coefficient` of an attack or release Point.

## Objective gates

1. **Old body is the oracle.** The crate's `settled_body_tests` harness already compares
   production against the pre-#981 reference kernel. Extend its randomized differential so that:
   * retargets on every parameter hit both channels and one channel;
   * mid-ramp retargets occur;
   * the `remaining = 0, current != target` restored state appears;
   * mix ramps occur with all-wet tables.

   Run 320 seeds × 128 blocks in release and 10 in dev, at `f32`, `Simd4` and `Simd8`, dual and
   collapsed. Compare every output word, recursive word, coefficient word and ramp field by bits.
   Allow the NaN relaxation only where `finish_channel` rejects the block. (The prototype passed
   this at 320 × 128 × 3 widths for C1, C2 and C2 with C2b.)
2. **Engagement witness.** A `#[cfg(test)]` counter of blocks whose prefix ran the two-pass body.
   It engages on `Main` with any open ramp, dual and collapsed. It never engages on `Silent` or
   `Sidechain`. The wet arm engages only when `mix == 1` everywhere and no mix ramp is open.
3. **Scenario pinned on base.** A bank-API scenario, 128 blocks at W8 and W4, dual and collapsed:
   * a threshold ride on one lane;
   * an all-lanes attack ride;
   * a makeup and mix ride on an all-wet table;
   * a `DiscontinuityKeepParameters` reset mid-ramp;
   * hostile input.

   Pin one SHA-256 of all output words and per-block payloads on `49f696c7`.
4. **Existing gates unchanged:**
   * `ramps`, `partition`, `native_points`, `payload`, `cross_target` (`dual_mono_ramping`),
     `mono_collapse`;
   * the kernel's `coefficient_ramp_tests`;
   * `tools/console-workload`'s `automation.rs` and every console digest.
5. **Mutations**, each recorded red:
   * advance at rest (drop the `remaining == 0` hold);
   * gate the rate ramps by their own `remaining` rather than the parameter ramp's;
   * advance attack in pass 1;
   * write the redesigned curve on every lane rather than the moved lanes;
   * take the wet arm while a mix ramp is open;
   * scatter the words before the ramps' last frame.
6. **Realtime and wasm.**
   * `tests/allocation.rs`, `check-realtime-policy.sh` and `check-lane-policy.sh` pass.
   * `check-web-audioworklet.sh`: "compressor f32x4 dual" and "collapsed" each match exactly one
     function, and each has scalar arithmetic 0. The prototype, which changed only the dual body,
     left `advance_ramps` inlined into the collapsed kernel at scalar 9: contract 4 removes it.
   * `scripts/check-web-boot-budget.mjs` passes. Record the artifact size.
7. **No regression through the shipped artifact.** Build base and change with the delivery recipe
   and run the diagnosis's V8 harness (`web_auto.mjs comp_threshold,comp_attack all`, then
   `mono_mix` with P1 if it has landed) once, paired, 6 rounds.
   * The `settled` isolate must not be slower than base by more than 2 %.
   * Record the automated arms. This is descriptive.

## What the implementer will hit

* **Register pressure.** All seven ramps moving is 36 vectors per channel. The prototype let them
  spill, and it was still 5-8x cheaper than the scalar loop. Do not specialise per parameter set
  before measuring.
* **Gathering only moving ramps** measured no faster. Keep whichever is simpler, but the curve
  currents are needed whenever any curve parameter moves.
* **The partition invariance of ramps** (`partition` test) holds only if `scatter` writes
  `remaining` back exactly. It is an exact integer in `f32`.

## Dependencies

None. It is independent of the EQ items. Land it after P1 (`automation-1`) if the mono-console
gate is to show the collapsed body's gain.

## Amendments (Sol verification, 2026-09-27)

Evidence: `docs/handoffs/effects-2026-09-27/VERIFY-AUTOMATION.md` (F4, F6, F7, F8 and section 4)
and `verify-automation-raw-timings.txt`.

### What reproduces

On the **dual** body:

* **V8 threshold Δ** (one lane / 8 of 64 / all 64): +6.05 / +44.6 / +142.8 µs → **+2.81 / +14.4 /
  +29.9 µs**. Attack, all 64: +106.9 → +31.0 µs.
* **Native `Simd8`:** +5.11 / +39.3 / +81.5 µs → +1.00 / +10.3 / +12.2 µs.
* **Differential:** `diag_ramping_prefix_randomized` passed in release, 320 seeds × 128 blocks ×
  {`f32`, `Simd4`, `Simd8`}, for C2, C1 and C2 + C2b.
* **Wasm:** the dual kernel's vector count goes 516 → 1,084, and the collapsed kernel's scalar count
  0 → 9, as the draft says.

### A1. The collapsed body is the primary deliverable, and it must be measured

The prototype rewrote the dual prefix only. A collapsed bank, which is what a mono stem is once
`automation-1` lands, still runs `frames_loop_mono::<L, true>`.

V8, mono console, threshold rides, with P1 in both builds so that the banks stay collapsed:

| arm | P1 | P1 + C2 |
|---|---:|---:|
| 8 of 64 | +33.7 µs | +32.0 µs |
| all 64 | +97.1 µs | +95.3 µs |

Both are noise. The C2 gain the draft tables quote therefore applies to stereo, or uncollapsed,
banks only.

* Contract 4 (`ramping_main_mono`) is the half that serves the owner's common case. It is
  unprototyped, so its cost and saving are unknown. Quote none until it is measured.
* The engagement witness (gate 2) counts **dual and collapsed** prefix engagements separately.
  Both must be non-zero on `Main` with an open ramp.
* The existing `randomized` differential already runs a `Mono` arm (`process_block_mono`). It must
  be shown to reach the new collapsed prefix, through its own counter.
* **Dependency.** Land after `automation-1`. Until then, any compressor write retires the mono
  console's collapse, and the V8 mono gate below cannot see the collapsed body.

### A2. Differential additions (gate 1)

* Retargets that hit **both channels with the same value in the same block**, which is the web
  host's `channel = 2` traffic, as well as one-channel ones.
* The restored `remaining = 0, current != target` state, reached through
  `restore_state_payload` rather than by poking fields.
* **Boundary values:**
  * knee at `+0.0`, at the smallest subnormal and just below `MIN_SOFT_KNEE_DB` (which takes
    `design_curve`'s `inv < +inf` select), and at 24 dB;
  * ratio, threshold, attack and release at both domain ends (attack and release drive the `f64`
    rate coefficient);
  * makeup at both ends;
  * mix at 0 and at 1.
* **The NaN relaxation (F8).**
  * The prototype's `wet_arm_relaxed` grants the payload relaxation to every ramping, `Main`,
    unbypassed block.
  * Grant it only when a `#[cfg(test)]` counter shows that the prefix, or the settled body, took
    the wet arm in that block, and only where `finish_channel` rejects the block.
  * Words after `finish_channel`, and every state word, compare strictly by bits.

### A3. Gate 5 additions

* The collapsed prefix skips the curve redesign on a moved lane.
* The collapsed prefix takes the wet arm while a mix ramp is open.
* If C2b is ruled in: `discontinuity_reset` designs with `changed = 0`. It goes red only under
  C2b, because the reference kernel re-derives all four curve words on the next advance.

### A4. C2b's invariant (section 4 of the verification)

The invariant is `words[0..4] == GainComputerCoef::new(ramps[0..3].current)`. It holds on every
write site on today's tree:

* `seed_from_defaults`, `full_reset`, `discontinuity_reset`, `redesign` on restore, and
  `advance_ramps` each rewrite all four words through `design_lane`;
* `copy_state_from` copies words and ramps together;
* `set_target` writes `current` only when it is already bit-equal.

Each curve word depends on one parameter. It is exact, but by maintenance, not by construction.
Measured natively: 8 of 64 goes +10.3 → +8.2 µs, and all 64 goes +12.2 → +10.9 µs. Take it only
on the owner's ruling, with the debug assertion after every block and A3's mutation.

### A5. Gate 7, made self-contained, with mono rows

1. Extract `automation-diag-tools/` from the diagnosis patch.
2. Fix `web_auto.mjs`'s three hard-coded paths.
3. Add `mono_comp = {...comp_threshold, doc: "mono", effectIndex: 1}`, or apply
   `verify-automation-harness.patch`.
4. Run `gen_docs.py`.
5. Build base and change with the flags of `scripts/build-web-audioworklet.sh`, with P1 in both.
6. Under the lock, pinned, run `ROUNDS=6 BLOCKS=500`:
   * `comp_threshold,comp_attack` on arms `settled,one_point,eight_of_64,all_64`;
   * `mono_comp` on arms `settled,eight_of_64,all_64`.

Gates:

* **No regression.** The `settled` isolate must not be more than 2 % slower, dual or mono.
* **Descriptive.** Record the mono saving. It is the product number this slice exists for.
* **Identity.** `DIGEST=150` must print "all identical". The one-track `comp_attack` arms equal
  `settled` on this fixture (F6), so the identity evidence must use an arm that moves bits.

### A6. Wasm

* "compressor f32x4 collapsed" must return to scalar 0.
* The prototype's clean build grew by 53,316 bytes. That includes C1, which `mode & 3` routes onto
  `Silent` and `Sidechain` blocks. The implementation must carry neither C1 nor that routing.
* Record the size under `check-web-boot-budget.mjs`.
