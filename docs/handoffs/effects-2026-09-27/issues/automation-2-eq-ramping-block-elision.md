# EQ: run only live or ramping sections on a ramping block, under the elision gate

Automation follow-up E1 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at `49f696c7`;
every `file:line` is `crates/parametric-eq/src/lib.rs` on that tree unless named). Evidence:
`docs/handoffs/effects-2026-09-27/AUTOMATION-DIAGNOSIS.md`, sections 2, 3 ("EQ") and 5 (E1).
The prototype is bit 0 of `parametric_eq::diag::MODE` in
`docs/handoffs/effects-2026-09-27/automation-diagnosis-prototypes.patch`. It is evidence only; do
not commit it.

## Product outcome

A prepared target starts a 64-sample word ramp on one lane of one section (`start_ramp`, `:1247`).
From then until the ramp ends, `render` (`:2191`) sees `stationary == false` for the whole bank
(`:2207`), and `process_channels` (`:1520`) runs `Channel::process_block` (`:1364`) on each
channel.

That path is six `process_section` passes per channel (`:1376`). Every section runs, whether
identity or not: two dedicated cuts in their masked kernel, and disabled bands through
`svf_block`. Each pass is a single-chain recurrence. The fixture's settled bank runs one
interleaved pass over its one live bell. Its ramping bank runs twelve.

So one knob ride, or one automation lane, makes its bank 7.8-7.9x as expensive at every width.
Eight automated tracks spread over eight banks add three times the EQ's whole settled cost.
Measured, in µs per 64-track block, EQ isolate, Δ against settled:

| arm | native `Simd8` base / E1 | native `Simd4` base / E1 | V8 base / E1 |
|---|---:|---:|---:|
| one lane, a Point per block | +9.0 / **+1.8** | +8.1 / **+1.5** | +10.5 / **+2.9** |
| 8 of 64 tracks | +67.7 / **+8.9** | +65.0 / **+8.7** | +64.9 / **+11.2** |
| all 64 | +88.7 / **+29.8** | +148.1 / **+33.7** | +146.0 / **+38.7** |
| settled | 9.6 / 9.7 | 18.9 / 18.4 | 21.5 / 21.5 |

These figures are descriptive only. The native `Simd4` column is a four-lane bank bound through a
diagnosis switch.

## Why this is exact

* The stationary elision (`cascade_sections`, `:1772`, and its proof) drops a section that is
  the exact identity on every lane of both channels. Its obligation is per dead section: no
  `-0.0` reaches it, its integrators are `+0.0`, and the block's input is finite and inside the
  ceiling. Legs (a), (b) and (c) discharge it.
* A ramping block adds one obligation. A dead section downstream of a **ramping** live section
  receives that section's output, and the proof's "a live section emits no `-0.0`" was argued for
  designed words, not interpolated ones.
  * **Safe ramping section.** The section's `m0` word is exactly `1.0` with a `+0.0` step on
    every lane of both channels. Then `m0 * v0 = v0` on every frame of the ramp, and
    `y = m2*v2 + (m1*v1 + v0)` is `-0.0` only if `v0` is. The proof's `m0 = 1.0` case applies,
    whatever `m1`, `m2` and the other words do. This covers bell, low-shelf, notch and high-pass
    automation, and every HPF-cut ramp (the identity and the HPF both have `m0 = 1`).
  * **Any other ramping section** (a high shelf's gain, a general low pass, an LPF-cut toggle)
    keeps every dead section after it in the list. The existing proof then covers everything that
    is dropped. The LPF is last anyway.
* **Freshness.** A section is dead only if no lane of it is in flight on either channel. For such
  a section `identity[s]` is fresh: every ramp ends in `snap` followed by `refresh_identity`, and
  `settle` and `start_ramp` refresh too. No ramp can start mid-block.
* The kept sections run `process_section` exactly as today, in cascade order, left then right per
  section. The channels are independent, so interleaving them by section is exact.

## Invariants

* **Class A.** Every output word, integrator, coefficient word, step, target, `remaining`,
  identity flag, report and payload is unchanged, at `f32`, `Simd4` and `Simd8`, dual and
  collapsed.
* The gate is the stationary gate with the same three legs, nothing weaker. It refuses into
  today's path.
* No change to the stationary path, the kernels, `start_ramp`, `snap` or the state layout.
* Allocation-free. The list is a `[usize; 6]` on the stack, as in `cascade_sections`.

## Interface contract

1. `fn ramping_sections<L, W>(left, right, left_io, right_io, frames) -> ([usize; 6], usize)`
   with the rule above, and `ramping_sections_mono` with the right-channel terms dropped, as
   `cascade_sections_mono` does.
2. In `process_channels` and `process_channels_mono`, the `!stationary` arm runs `process_section`
   for each listed section (both channels, left first) in place of `process_block`.
   `Channel::process_block` stays for the corpus (`corpus.rs`) and the tests.
3. `#[inline(always)]` on `PreparedParametricEq::render` and on `process_bank_inner`. Without it,
   the prototype's clean wasm build outlined the EQ's arithmetic out of
   `PreparedNativeEffectBank::process_bank`, first into `render` and then into
   `process_bank_inner`, and failed `KERNEL_ROSTER` rule 1. With both, the roster is exactly one
   function, vector count 312, and the kernel count stays at 14.
4. The proof doc on `cascade_sections` gains the ramping paragraph above.

## Smallest closable slice

Authorized paths:

* `crates/parametric-eq/src/lib.rs`;
* `crates/parametric-eq/tests/` (a new `ramping_elision.rs`);
* `crates/parametric-eq/tests/MUTATIONS.md`;
* `tools/console-workload/tests/` (the scenario);
* this spec.

Steps:

1. **On the base:** write gate 3's scenario and pin its digests.
2. Contracts 1-4.
3. The gates, and the evidence.

## Non-goals

* Interleaving the kept sections' left and right chains in one ramped pass (E3). That is the
  next step, measured as the residual here (a moving `Simd4` bank at +2.3 µs under V8 after E1
  and E2), and it is not prototyped.
* E2's lane writes (`automation-4`).
* #977's select-free passes, or any change to the dry masks of kept dedicated cuts.

## Objective gates

1. **Identity, bank differential** (`automation_diag_identity.rs` in the patch is the model).
   Bind two banks per scenario at `Simd4` and `Simd8` (and scalar instances); one runs base,
   one runs the change. Scenarios:
   * random per-lane configurations of every kind;
   * bands off on every lane in some scenarios, so that the list engages;
   * cut enable toggles;
   * one- and two-channel retargets through the production target preparation
     (`prepare_targets`, `apply_prepared_target_lane`), at 55 % of blocks;
   * block lengths 1-128;
   * hostile input: `-0.0`, subnormals, raw bit patterns, ±inf, NaN, 1e31;
   * restores of hostile integrators: `-0.0` in a live section, a large finite `ic2`, NaN, in a
     dead section and in a live one;
   * dual, and collapsed with symmetric events.

   After every block, compare every output word and every lane's payload by bits. A NaN output
   word may compare as "both NaN" only where the §4.4 check rejects the block in both arms.
   Minimum 300 scenarios × 96 blocks per width in release and 40 in dev. Record the number of
   ramping blocks that dropped at least one section (a `test-support` counter). It must be in the
   thousands per width.
2. **The unsafe-ramp rule, structurally.** The random differential cannot see it: the diagnosis's
   mutation M3 stayed green there. So assert the list directly, at every width:
   * a ramping high shelf in band 2, with bands 3 and 4 dead: the list keeps 3 and 4;
   * the same with a ramping bell: the list drops them;
   * an LPF-cut toggle: the list keeps nothing extra, because nothing follows it;
   * an HPF-cut toggle: the list drops dead bands after it.
3. **Scenario pinned on base.** `sixty_four_track_eq_only` with the diagnosis's `eight_of_64` and
   `all_64` gain rides through `push_parameter`, 128 blocks, W8 and W4. Pin one SHA-256 of all
   output words on `49f696c7`.
4. **Existing gates unchanged:**
   * the crate's suite in dev and release (`interleave_identity`, the elision tests,
     `mono_collapse`, E9 `determinism`);
   * `tools/console-workload` digests and `chain_shape`.
5. **Mutations**, each recorded red:
   * M1: drop leg (a). The differential goes red (the diagnosis saw it at seed 2).
   * M2: treat a ramping identity section as dead. The differential's state goes red.
   * M3: drop the unsafe-ramp rule. Gate 2 goes red.
   * M4: read `identity[s]` without the "not ramping" term, on the mono list only.
   * M5: skip leg (b) for dead sections.
6. **Realtime and wasm.**
   * `tests/allocation.rs`, `check-realtime-policy.sh` and `check-lane-policy.sh` pass.
   * `check-web-audioworklet.sh`: "parametric-eq f32x4 dual" and "collapsed" each match exactly
     one arithmetic-carrying function, rule 3's count does not drop, and no scalar `f32`
     arithmetic appears in either.
7. **No regression through the shipped artifact.** Build base and change with the delivery recipe
   and run the diagnosis's V8 harness (`web_auto.mjs eq_gain all`) once, paired, 6 rounds.
   * The `settled` isolate must not be slower than base by more than 2 %.
   * `eight_of_64` must show the saving: descriptive, but a gain of under half the diagnosis's
     means the list is not engaging, and must be explained before a verdict.

## What the implementer will hit

* `process_block` is `#[inline(always)]` for the roster's sake. Adding a second caller of
  `process_section` pushed LLVM to outline one level up, which is why contract 3 exists. Check
  the roster on a clean `host-web` build, not a flag build.
* Build the list from `remaining` at block start. A lane that snaps mid-block is still "ramping"
  for this block, which is the conservative direction.
* The mono list reads the left channel only. That is sound only on a collapse-eligible bank, as
  `cascade_sections_mono` argues.

## Dependencies

None. Land it before `automation-4`, which touches the same `start_ramp` and `process_section`
code.

## Amendments (Sol verification, 2026-09-27)

Evidence: `docs/handoffs/effects-2026-09-27/VERIFY-AUTOMATION.md` (sections 2 and 5, F6 and F7)
and `verify-automation-raw-timings.txt`.

The slice stands. E1 was reproduced:

* **V8 EQ Δ** (one lane / 8 of 64 / all 64): +9.25 / +64.0 / +145.4 µs → **+3.04 / +11.4 /
  +38.4 µs**.
* **Native `Simd8`:** +8.62 / +67.3 / +89.3 µs → +1.39 / +8.9 / +30.6 µs.
* **Collapsed body** (V8, mono console, EQ ridden on 8 and on 64 tracks; the SDK sends a `Both`
  target, so the collapse holds at base): +31.6 → **+4.2 µs**, and +73.4 → +20.2 µs.
* **Differential:** the bank differential passed at 300 × 96 per width in release. E1 elided
  sections on 4,040 ramping blocks at W4 and 3,859 at W8.
* **Contract 3 confirmed.** A clean `host_web.wasm` with E1 but without the two
  `#[inline(always)]` fails `KERNEL_ROSTER` rule 1 ("0 arithmetic-carrying kernels match
  `parametric_eq.*4wide6f32x4.*12process_bank`"). With them, the roster is unchanged: dual 312,
  collapsed 156, 14 kernels. The artifact grows by 8,125 bytes.

### A1. Differential additions (gate 1)

* **`Both` targets, the product path.** The web SDK sends a symmetric both-channel edit as **one
  `Both` target**. Prepare some retargets with both rows of a band changed in one
  `prepare_targets` call, which emits a `Both` target when the sections agree, and apply it with
  `apply_prepared_target_lane`. The diagnosis's differential only ever applied `Left` or `Right`
  targets.
* **Resets mid-ramp:** `DiscontinuityKeepParameters` and `FullToDefaults`, in about 2 % of blocks.
* **Boundary targets:**
  * high shelf at +24 and −24 dB, which is the unsafe-ramp path;
  * Q at both domain ends;
  * frequency at 20 Hz and at the rate's upper bound;
  * a band enabled into a high shelf and disabled out of one, which ramps from the identity to a
    shelf;
  * a lane-mixed section, with a bell on one lane and a high shelf on another, both ramping.
* **Comparison.** The bank output is read after the §4.4 check, so compare every output word
  **strictly by bits**. The "both NaN" allowance applies only to a plane read before the check, if
  the implementer adds one, and only in blocks the check rejects in both arms. Payloads are
  compared strictly after every block.

### A2. Gate 2 additions

* An identity → high-shelf enable ramp in band 2, with bands 3 and 4 dead. The list **keeps** 3
  and 4, because `coef.m0` starts at `1.0` but `step.m0 != +0.0`.
* A lane-mixed section, a bell on one lane and a high shelf on another, keeps its downstream dead
  sections.

### A3. Gate 3 additions

Pin `sixty_four_track_console_mono` as well, with the EQ ride pushed as one
`ParameterChannel::Both` owner edit per block (`push_parameter(ch, p, Both, v)`, the SDK's
shape). Without P1 that keeps every cohort collapsed, so the collapsed list
(`ramping_sections_mono`) is exercised at console level. Pin both on the current tip, which is
code-identical to `49f696c7`.

### A4. Gate 5 additions

* **M6:** the dual list's `dead` without the "not ramping" term. The draft's M4 covers the mono
  list only.
* **M7:** `unit_m0` checks `coef.m0` only, not `step.m0`. It must go red on A2's enable ramp.

### A5. Gate 7, made self-contained, with a mono row

1. Extract `automation-diag-tools/` from the diagnosis patch.
2. In `web_auto.mjs`, fix its three hard-coded paths: the `prepared-control.js` import, the ABI
   JSON and `ROOT`.
3. Add `SUBJECTS.mono_eq = {...SUBJECTS.eq_gain, doc: "mono", effectIndex: 0}`, or apply
   `verify-automation-harness.patch`.
4. Run `gen_docs.py`.
5. Build base and change with the flags of `scripts/build-web-audioworklet.sh` into
   `…/t-web-NAME/wasm32-unknown-unknown/release/host_web.wasm`.
6. Under the lock, pinned, run `web_auto.mjs` with `ROUNDS=6 BLOCKS=500` on subjects
   `eq_gain,mono_eq` and arms `settled,one_point,eight_of_64,all_64`.

Gates:

* **No regression.** The `settled` isolate must not be more than 2 % slower on either subject.
* **Engagement.** `eight_of_64` must show at least half of the saving above, on both subjects.
* **Identity.** `DIGEST=150` on both subjects and all arms must print "all identical", and every
  arm quoted as identity evidence must differ from `settled` (F6).
