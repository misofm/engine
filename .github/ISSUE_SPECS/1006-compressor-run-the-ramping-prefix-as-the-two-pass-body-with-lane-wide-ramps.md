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

## Attempt 1 evidence

Implementer attempt 1 (Terra), 2026-09-27, branch `codex/1006-compressor-ramping-prefix` (the #995
branch with the batch head merged in; the batch head `ce2f1534`, with #995 and its verdict, is
merged as `b0d8569f`). Code commit `3a293404`; `ef585bf5` is the separate doc-only correction of
#995's `#[cold]` comment (Sol's #995 finding). The bit-identity oracle is the batch head's kernel
(`081fdc6c` and `ce2f1534` carry the same compressor, host and effect runtime), on every input.
Host: AMD EPYC 7313P (Zen 3), rustc 1.97.1, Node 22.23.2 (V8 12.4, `--no-liftoff`). Every timed
run held `timing.lock`, was pinned with `taskset -c 31`, and names its load; the longest hold was
1m30s. `scripts/run-console-benchmark.sh` was not run.

### The change (class A)

`crates/compressor/src/kernel.rs`; the scenario in `crates/compressor/tests/`; `MUTATIONS.md`.

* **Contracts 1 and 2.** `RampVec` (`gather`, `scatter`, `advance_where`: `next_value` as selects,
  with the `remaining == 0` hold) and `ChannelRamps` (`gather` of every ramp, rate ramp and word
  once per block; `advance_curve`; `advance_output`, the rate ramps gated by their parameter
  ramp's was-moving mask; `curve`; `coef`; `wet`). `design_curve` is `GainComputerCoef::new` per
  lane with `knee_coefficients`' branch as `W > 0 && 1/(2W) < +inf`. The redesign keeps
  `design_lane`'s `changed` rule: it writes the four curve words only on lanes where a curve
  parameter moved.
* **Contracts 3 and 4.** `ramping_main` (dual) and `ramping_main_mono` (collapsed) replace
  `frames_loop::<L, true>` and `frames_loop_mono::<L, true>` for `Detector::Main`: the settled
  body's `SETTLED_CHUNK` two passes, pass 1 advancing threshold, ratio and knee and computing the
  targets, pass 2 advancing attack, release, makeup and mix and running the recurrence and the
  output. The DualMono arm is #984's; the wet arm (#982) is taken only when the block is
  unbypassed, no mix ramp is open on either channel and every lane's mix is `1`. Four
  `(DUAL_MONO, WET)` instantiations each, `#[inline(always)]` into the roster functions.
* **Contract 5.** `curve_target` takes `&GainComputerCoef` (every caller passes `&coef.curve`);
  `Coef::new` builds the masks for `Coef::load` and for the prefix, so there is one mask rule.
* **Contract 6.** `Silent` and `Sidechain` keep `frames_loop::<L, true>` (and the collapsed body's
  non-`Main` detectors `frames_loop_mono::<L, true>`) unchanged; there is no C1 and no C1 routing.
  `advance_ramps` is `#[inline(never)]`, where it already was in both the wasm and the native
  builds, so that it cannot land in the collapsed roster function.
* **One measured deviation: the scalar instance's prefix is out of line** (`ramping_main_scalar`,
  taken when `L::WIDTH == 1`). With the prefix inlined, `process_block::<f32>` grew too large to
  inline into the scalar instance's `process`, and natively its settled loops came out with more
  instructions and stack traffic: interleaved A/B, 64 unbanked settled instances (native `Simd4`
  dispatch), six alternations at load 20-24: base 126.54-127.30 us, inlined 127.77-128.18
  in five alternations and 139.54 in one disturbed one (+1.0 %), out of line 126.30-126.83
  (flat). Out of line, every settled
  loop of the scalar instance is the batch head's in instruction and stack-access counts but one
  (one instruction longer). Banks keep the prefix inline, so the roster carries it.

### Timing (us per 64-track block; isolate = row minus builtins-only in the same round)

**V8, the shipped `host_web.wasm` render export**, through the live console's command queue
(`channel = 2` Points each block), the diagnosis harness `web_auto.mjs` (paths fixed, subjects
`console_comp` and `mono_attack` added), `ROUNDS=6 BLOCKS=500`, arms interleaved. Δ is the
isolate's change from `settled` in the same module.

| session, control | arm | base | #1006 |
|---|---|---:|---:|
| stereo console, threshold (load 10.9-11.1) | settled isolate | 193.15 | 192.30 |
| | one lane, a Point per block: Δ | +5.45 | **+1.78** |
| | 8 of 64: Δ | +43.40 | **+13.54** |
| | all 64: Δ | +142.79 | **+26.61** |
| mono console, threshold, P1 in both builds, banks collapsed (load 9.8-10.2) | untouched / settled isolate | 98.32 / 98.64 | 97.46 / 97.79 |
| | one lane: Δ | +4.03 | **+1.27** |
| | 8 of 64: Δ | +32.96 | **+7.63** |
| | all 64: Δ | +96.11 | **+18.39** |
| mono console, attack, P1, collapsed | settled isolate | 98.00 | 97.23 |
| | one lane / 8 of 64 / all 64: Δ | +4.53 / +30.84 / +78.95 | **+1.33 / +7.74 / +19.79** |
| mono console as shipped (no P1: the settle writes send every bank dual) | settled isolate | 194.96 | 196.90 |
| | one lane / 8 of 64 / all 64: Δ | +6.42 / +43.74 / +143.71 | **-0.90 / +9.82 / +24.95** |
| compressor only, threshold | settled isolate | 61.46 | 61.42 |
| | one lane / 8 of 64 / all 64: Δ | +6.03 / +44.40 / +145.35 | **+2.09 / +13.04 / +27.53** |
| compressor only, attack | settled isolate | 61.79 | 60.83 |
| | one lane / 8 of 64 / all 64: Δ | +5.82 / +39.15 / +109.13 | **+2.64 / +13.68 / +29.34** |

P1 (#1004) has not landed. The collapsed rows use the verification's spans-only P1 prototype,
compiled on, in **both** scratch builds; nothing of it is committed. Without P1 the product's mono
console runs dual banks once any compressor knob is touched, which is the "as shipped" row.

**No regression on the other compressor shapes under V8.** Settled isolates above are within
-1.3 to +1.0 %. The #995 sessions through the same artifact (9 x 1,000 blocks, two reps, load
5.1-6.5), rows base / #1006: 64 sidechained unbanked 188.75, 189.24 / 187.61, 190.41; 64 settled
banked 109.08, 110.35 / 109.36, 110.68; collapsed mono 63.61, 64.12 / 63.49, 63.63; 3 unbanked
main-detector 11.05, 11.11 / 11.12, 11.23; 3 sidechained 11.42, 11.34 / 11.36, 11.41. The V8
listings of the settled loops are unchanged (below).

**Native**, the diagnosis's console harness (`automation_diag`, production control queue; six
interleaved rounds of 600 blocks; two reps; load 4.7-5.4). Isolates, rep 1 / rep 2:

| shape | arm | base | #1006 |
|---|---|---:|---:|
| `Simd8`, compressor only, threshold | settled | 24.14 / 24.98 | 24.51 / 23.89 |
| | one lane / 8 of 64 / all 64 | 29.82 / 65.05 / 103.57 | **26.31 / 34.08 / 36.10** |
| `Simd8`, stereo console, threshold | settled | 81.34 / 81.80 | 81.58 / 82.01 |
| | one lane / 8 of 64 / all 64 | 86.79 / 121.31 / 160.44 | **83.32 / 90.24 / 92.95** |
| `Simd8`, mono console, P1, collapsed | settled | 43.55 / 43.66 | 43.85 / 43.56 |
| | one lane / 8 of 64 / all 64 | 47.04 / 69.79 / 90.09 | **44.33 / 50.32 / 53.25** |
| `Simd4` banks (the diagnosis's four-lane bind), threshold | settled | 46.71 / 47.41 | 47.45 / 47.33 |
| | one lane / 8 of 64 / all 64 | 50.87 / 77.07 / 140.98 | **48.29 / 55.53 / 64.92** |
| `Simd4` dispatch as shipped (every compressor an unbanked `f32` instance) | settled | 84.22 / 84.81 | 84.90 / 85.56 |
| | one lane / 8 of 64 / all 64 | 88.12 / 114.29 / 323.55 | **86.36 / 94.02 / 159.48** |

The native settled rows, and the other shapes, re-measured at lower load: the settled consoles,
five alternations at load 3.0-3.7 (rows): stereo 106.01 / 106.46 median, mono as shipped
107.70 / 107.72, mono untouched (collapsed) 69.00 / 69.48; the #995 sessions (two reps, load
3.7-4.9), `Simd8` rows base / #1006: 64 sidechained 137.46, 137.75 / 136.76, 137.09; 64 settled
banked 48.00, 49.13 / 48.84, 48.84; collapsed mono 30.47, 30.76 / 30.50, 30.48; 3 unbanked
7.58, 7.66 / 7.61, 7.62; and at `Simd4` dispatch 64 unbanked main-detector 124.58, 124.57 /
124.52, 124.65 and 64 sidechained 154.42, 156.16 / 154.40, 153.96. The largest settled
difference anywhere is the collapsed untouched console's +0.7 % in one harness, flat in the
other.

### Codegen

* **V8 listings** (`--print-wasm-code`, natural loops, one iteration laid out). The dual kernel
  `process_block::<f32x4>`: all eight settled loops have the head's instruction and frame-access
  counts or one less; the one-pass prefix (476 instructions, 121 frame accesses per frame, plus
  two out-of-line `advance_ramps` calls) is replaced by eight two-pass loops, pass 1 366-378
  instructions and 33-34 frame accesses, pass 2 444-481 and 61-62, no calls. The collapsed kernel:
  all eight settled loops identical; the prefix is replaced by pass 1 203-217 / 25-35 and pass 2
  256-281 / 62-63. `process_block::<f32>`'s twelve settled loops are identical. **Carried stack
  slots: the new pass 2 loops carry the output ramps' state through the frame** (dual 7-9 slots,
  collapsed 16-17), because every ramp field of attack, release, makeup and mix is loop-carried
  and does not fit the 16 registers. Recorded, not chased: the brief's "let them spill", and the
  measured Δ already falls 4-6x. The retained sidechain prefix loops are unchanged.
* **Native `Simd8`** (`objdump`): the dual and collapsed settled loops have the head's
  instruction and stack-access counts; `process_block::<f32x8>` is now out of line from
  `Instance::render` (it grew), measured flat.
* **Wasm roster** (`check-web-audioworklet-callgraph.py`): "compressor f32x4 dual" vector 427 ->
  949, scalar 0; "collapsed" 262 -> 524, **scalar 0** (the prototype's scalar 9 is gone); 15
  kernels. Artifact 3,400,324 -> 3,456,671 bytes (+56,347); `check-web-boot-budget.mjs` passes.

### Exactness

* **In-tree**, dev and release: the grid and the three randomized differentials now assert, per
  block, that the dual prefix and the collapsed prefix ran exactly on a `Main` block with an open
  window, and that the prefix took the wet arm exactly when predicted from the pre-block state
  (unbypassed, no mix ramp open, every lane's mix `1`); #982's relaxation is granted only where a
  witness shows an arm ran, and only in a rejected channel. Additions per A2: retargets on one
  channel or on both with one value; payload restores through `validate_channel` and
  `commit_channel` at `remaining = 0` with `current != target` (or mid-window); knees at the
  smallest subnormal and either side of `MIN_SOFT_KNEE_DB`. Release coverage per width: 11,500
  dual and 10,200-10,500 collapsed prefixes, 2,750-4,850 on the wet arm.
* **Scenarios pinned before the change** on the unmodified batch head, dev and release, unchanged
  after: `scenario_1006_ramping_prefix_is_pinned` (`162979dd...`; kernel, `f32`, `Simd4`, `Simd8`,
  DualMono and Maximum, dual and collapsed, a threshold ride on one lane and a left-only write,
  an all-lanes attack ride, a makeup and mix ride on the all-wet table, two discontinuity resets
  mid-window, a restore at `remaining = 0`, windows crossing block boundaries, hostile input;
  kernel words, recursive words, masks, finished words and every lane's payload words) and
  `tests/ramping_prefix_scenario.rs` (`61ecffd0...`; the same schedule through the bank contract at
  this build's width, dual and `process_bank_mono`, with reports and every track's payload; 0
  invalid spans, some rejected blocks). The bank API binds only this build's width natively, so
  the four-lane bank is covered by the kernel scenario. They were pinned on `081fdc6c`, not
  `49f696c7`: the main detector's kernel is the same on both (#994 was already in; #995 changed
  only the sidechain arms).
* **Independent**, scratch only: the #995 harness with the batch head's kernel as `Base`, built
  from its own sources beside #1006's, now with the payload restores and a prediction of the
  prefix's wet arm. Every output word, every coefficient word, all ramp fields and the recursive
  words, then masks and words after `finish_channel`; NaN payloads relaxed only where the prefix
  (or the settled body) is predicted to take the wet arm, in rejected channels. **0 failures**:
  native release 3,000 seeds x 160 blocks at `f32`, `Simd4` and `Simd8` (about 88,000 dual and
  40,000 collapsed prefixes per width, 56,000 of them still open at the block's end, 10,300-10,700
  collapse transitions, 13,400 restores); native dev 96 seeds; the public-factory differential
  1,200 seeds; V8 TurboFan 1,500 seeds at `f32` and `Simd4` plus 600 factory seeds; V8 default
  tiering 64. Strict mode (no relaxation) fails, so the relaxation is real and the harness sees
  it; the mutants M1 (`Simd4`, `Simd8`), M3, M5 and M6 fail it.
* **Digests.** All 30 standing console rows and 16 session rows, identical base / #1006. The
  native automated arms (threshold and attack, compressor only, stereo and mono consoles, seven
  arms each, at `Simd8`, the four-lane bind and `Simd4` dispatch, with and without P1): 83 rows
  identical. V8 `DIGEST=150` on the shipped artifact, compressor-only, stereo and mono consoles,
  with and without P1: all identical, and every automated arm moves bits except the one-track
  attack arms (F6). The committed #1003 browser preflight (`web-mixing-automation-benchmark.mjs
  preflight`): all seven arm digests identical, `automated_compressor_only` included.

### Gates

| gate | result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo clippy --locked --workspace --all-targets -- -D warnings`, and with `--all-features` | PASS |
| `cargo test --locked -p compressor`, dev and `--release` | 101 passed, 0 failed, each |
| `cargo test --locked -p console-workload` / `-p effect-runtime` | 44 / 90 passed |
| `bash scripts/run-wasm-gates.sh` | PASS: native, wasm scalar and `simd128` legs, 0 mismatches; the #1000 V8 spill gate ok |
| `bash scripts/check-env-vocabulary.sh` | PASS (134 names) |
| `check-realtime-policy.sh`, `check-lane-policy.sh`, `check-unfused-seal.sh` | PASS |
| `check-web-audioworklet-callgraph.py`: render rule 3, roster, `meter_poll`, `command_submit` | PASS (roster above; render closure 8, traps 5, unchanged) |
| `check-web-boot-budget.mjs` | PASS |
| allocation (`conformance`'s `process.allocation`) | PASS (in the crate run) |

### Mutations

`crates/compressor/tests/MUTATIONS.md`, section "Issue #1006". Red: M1 (the `remaining == 0` hold
dropped) 4, M3 (output ramps advanced in pass 1) 16, M5 (wet arm with a mix ramp open) 9, M6 (words
scattered from before the prefix) 16, M7 (the collapsed prefix skips the redesign) 11, M8 (the
collapsed prefix's wet arm with a mix ramp open) 6, M9 (the `abs` arm for every link mode) 11, M10
(threshold never advanced) 17, M11 (#986: the right channel's ramping set from the left) 7, M12
(#986: makeup and mix words not written) 12. Green, recorded with reasons: M2 (rate ramps gated by
their own `remaining`: equivalent, a rate ramp's `remaining` is always `0` or its parameter
ramp's) and M4 (redesign on every lane: equivalent by C2b's invariant, which this slice does not
rely on).

### For the verifier

* **The spec's M2 and M4 cannot go red** (above). M4 green is C2b's invariant showing through; C2b
  itself stays out.
* **The scalar-prefix outlining** is the one structural choice the brief did not name; its A/B is
  above. Without it the native unbanked settled path is +1.0 %.
* **Pass 2 spills under V8** (7-9 carried slots dual, 16-17 collapsed). The next saving is there
  (for example, specialising pass 2 by which output ramps are open), not attempted: the brief
  says measure before specialising, and the remaining Δ is 1-2 us per moving bank.
* `tests/bench_ramp.rs` is untouched. It is MQ-2's frozen, script-driven harness with a recorded
  identity, so adding #986's arms would change a frozen measurement; the arms were measured here
  with the diagnosis harness instead.
* Scratch harness sources, session documents and raw logs: `scratchpad/impl-1006/{tools,logs}`.
