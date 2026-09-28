# EQ: run only live or ramping sections on a ramping block, under the elision gate

Source: `docs/handoffs/effects-2026-09-27/AUTOMATION-DIAGNOSIS.md`, verified in `VERIFY-AUTOMATION.md`. **The Amendments section at the end supersedes the body wherever they conflict.** Draft names map to issues: automation-5 = #1003, automation-1 = #1004, automation-2 = #1005, automation-3 = #1006, automation-4 = #1007.

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

## Attempt 1 evidence

Terra, 2026-09-28, branch `codex/1005-eq-ramping-elision`. Code: `b787233b` (the list) and
`39908196` (leg (c) held to kernel-written states), on `a1fcab3d`; then `c75032f3` merges the batch
head `27cf2413` (#999, #1013). #999 made `process_channels` return the folded §4.4 verdict; the
ramping arm returns `None`, so `render` scans after a ramping block as before. Every gate and timing
below is on the merged tree against `27cf2413` unless it says otherwise. Host: AMD EPYC 7313P,
rustc 1.97.1, Node 22.23.2 (V8 12.4). Every timed command held the shared lock, was pinned with
`taskset -c 31`, and printed its load average.

### What changed

* **Contracts 1, 2, 4.** `ramping_sections` / `ramping_sections_mono` return the list, and the
  `!stationary` arms of `process_channels` / `process_channels_mono` run `process_section` for each
  listed section, left then right. `Channel::process_block` stays (corpus, tests, oracle). The
  proof on `cascade_sections` gains "# A ramping block": the safe-ramp case (`m0 = 1.0` with a
  `+0.0` step on every lane of both channels), the keep-everything-after-an-unsafe-ramp rule, and
  finiteness, which needs no gain bound for interpolated words: an inert identity section is exact
  on every finite non-`-0.0` input, and a non-finite word at a dead section leaves that lane
  non-finite in both arms, so the §4.4 check zeroes the plane and clears the channel in both.
* **Contract 3** needed no edit: `#[inline(always)]` has been on `render` and `process_bank_inner`
  since #978. The roster is unchanged (gate 6).
* **Leg (c) is tightened in the ramping lists only** (`section_state_is_flush_shaped`: every
  integrator of a kept section is `+0.0` or finite with `|x| >= FLUSH_EPS`). The stationary leg
  (finite, no `-0.0`) admits a restored subnormal, and the high-shelf case of the `-0.0` induction
  argues from kernel-written states. A live high shelf at `m0 = m2 = 0.5` (gain `-6.0206` dB as an
  `f32`) holding restored `ic1 = ic2 = -2^-149`, fed `-2^-149`, emits `-0.0` on its first frame, and
  an elided identity after it passes `-0.0` where the executed one writes `+0.0`. The batch-head
  ramping path never elides, so without the tightening this change was not class A there. It is
  stricter than the brief's "same three legs", never weaker; the refused block flushes the words and
  the next one engages.
* **Deviation: the differential is in-crate** (`mod ramping_elision` in `lib.rs`), not
  `tests/ramping_elision.rs`: an x86 build's factory refuses four-lane banks, the base arm has to
  be the batch-head code, and the list is private. The arms are `prepare_width` (the body of
  `bind_homogeneous_bank`) driven through `process_bank`, `process_bank_mono`,
  `apply_prepared_target_lane`, `reset`, `restore_track_state_payload` and
  `snapshot_track_state_payload`; the scalar arm calls `render`/`render_mono`, which `process`
  wraps. The oracle is `Channel::process_block`, selected by the unit-test-only `RAMPING_LIST`
  switch, whose default is the batch-head path: the unit tests written before #1005 call
  `process_channels(.., false)` as their full per-section oracle for the stationary elision, and a
  list that elided too would have turned those gates into "elided == elided". No feature build
  carries the switch; integration tests, the bench and the module take the list.
* The test-support counter `test_only_ramping_elided_blocks` counts ramping blocks whose list was
  shorter than six.

### Gate 1: the bank differential (release)

Every output word, every report, every lane payload and every internal word (coefficients, steps,
targets, integrators, `remaining`, identity flags, semantic targets, the fixed-point witness)
compared by bits after every block, strictly, with no NaN allowance. Scenarios draw rates, per-lane
kinds (lane-mixed included), bands off on every lane, cut toggles, 1-3 retargets per block through
`prepare_targets` (one- and two-row, so `Both` targets), bank-wide rides, refused targets (a family
change), resets mid-ramp (both kinds, ~2 %), restores (integrators `-0.0`, `1e31`, `-3e38`, NaN, inf,
`1e-40`, near `FLUSH_EPS`, in dead and live sections; forged ramps from and to the identity; cut
enables flipped with a matching ramp), boundary values (gain +-24 dB, Q 0.1/18, 10/20/20 000 Hz),
block lengths 1-128, and hostile input (`-0.0`, raw bits, +-inf, NaN, `1e31`, `-3e38`, words at the
ceiling). Identical before and after the merge:

| width, body | scenarios x blocks | ramping blocks | elided a section | targets (`Both`) | restores accepted: integrator / identity ramp / cut toggle |
|---|---|---:|---:|---|---|
| f32 dual | 300 x 96 | 11,450 | 6,425 | 66,391 (5,552) | 541 / 418 / 257 |
| f32 collapsed | 300 x 96 | 11,396 | 6,632 | 47,877 (all) | 500 / 436 / 288 |
| Simd4 dual | 300 x 96 | 11,422 | 5,860 | 99,616 (10,594) | 550 / 414 / 268 |
| Simd4 collapsed | 300 x 96 | 11,779 | 6,097 | 73,148 (all) | 545 / 443 / 308 |
| Simd8 dual | 300 x 96 | 11,316 | 5,584 | 141,978 (18,376) | 558 / 437 / 297 |
| Simd8 collapsed | 300 x 96 | 11,527 | 5,749 | 107,343 (all) | 501 / 398 / 304 |

Dev runs 40 x 96 per width and body (677-898 elided blocks each). All pass.

### Gate 2: the list, structurally (f32, Simd4, Simd8; dual and collapsed lists)

`the_unsafe_ramp_rule_keeps_every_dead_section_after_it`: a ramping high shelf in band 2 keeps
3, 4 and the LPF (`[1,2,3,4,5]`); a ramping bell drops them (`[1,2]`); an LPF toggle keeps nothing
extra (`[1,5]`); an HPF toggle drops the dead bands (`[0,1]`); an identity -> high-shelf ramp keeps
them (M7's shape); a 0 dB high shelf (`m0 = 1.0` exactly) ridden to +6 dB keeps them; channel-mixed
and lane-mixed bell/shelf sections keep them, also with only the bell lane ramping; `-0.0` input, a
`-0.0` dead state and a `-0.0` live state refuse (six); all-live gives six.
`a_ramping_identity_section_is_never_dead`: a one-channel HPF ramp from the identity is listed (dual
either side, collapsed). `a_restored_subnormal_live_state_refuses_the_list`: the shape above renders
the batch-head bits through the list, dual and collapsed, and the list is six; with the state
flushed, it engages (`[0,1]`).

### Gate 3: console scenarios pinned on `a1fcab3d`

`tools/console-workload/tests/eq_ramping_scenario.rs`, 64-block pre-roll then 128 blocks, band-1
gain 3 +- 0.25 dB, at `Simd8` and `Simd4` dispatch (the widths render the same bits):

* `sixty_four_track_eq_only`, `Left` then `Right` owner edits: settled `81363c22…`, eight_of_64
  `5470166c…`, all_64 `b51448fd…`;
* `sixty_four_track_console_mono`, one `Both` owner edit per block (asserted: every cohort collapsed
  on every block): settled `f973869e…`, eight_of_64 `bf0e96d1…`, all_64 `6e063a3e…`.

Each ride is asserted to differ from settled. Green on the change and after the merge, dev and
release.

### Gate 4: existing gates (merged tree)

`cargo test -p parametric-eq` dev and release, each with and without `test-support`: 12 binaries
ok. `cargo test -p console-workload` release and dev: all ok, `chain_shape` and every digest
unchanged. `cargo clippy --workspace --all-targets -- -D warnings`, and `-p parametric-eq -p
console-workload --all-features`; `cargo fmt --check`; `RUSTDOCFLAGS=-D warnings cargo doc -p
parametric-eq --features test-support`: clean. `check-realtime-policy.sh`, `check-lane-policy.sh`,
`check-env-vocabulary.sh`, `check-parametric-eq-render-contract.sh`, `check-workspace-policy.sh`:
ok. The crate has no `tests/allocation.rs`; the conformance harness's `process.allocation` gate is
the EQ's, and it passes.

### Gate 5: mutations (re-run on the merged tree)

M1 (both lists, dual, collapsed), M2, M4, M5 (both, dual, collapsed), M6: red in the differential
and gate 2. M3 and M7: red in gate 2 only (the differential stays green in dev and release, as the
diagnosis found for M3). M8 (leg (c) back to the stationary form; both, dual, collapsed): red in
`a_restored_subnormal_live_state_refuses_the_list` only, on a real bit (`-0.0` against `+0.0`).
Recorded in `tests/MUTATIONS.md`.

### Gate 6: realtime and wasm (merged tree)

The module built with the delivery recipe (`55772fcb…`, 3,490,066 bytes, +7,164 over the base's
3,482,902; the same bytes as the timed module):

* `KERNEL_ROSTER`: `parametric-eq f32x4 dual` vector 672 / scalar 0 and `collapsed` 336 / 0, each
  one function; kernels 15 and `f32x4` arithmetic 14,112, both as base. Render callgraph as base.
  `check-web-audioworklet.sh` on the assembled seven-file directory: passes.
* `run-wasm-gates.sh`: ok; V8 spill gate ok (dual tail 109 instructions, mono pair 78, mono tail 53,
  none carried).
* **Every EQ loop, masked included** (the gate's own loop analysis over both `process_bank`
  functions, base against change). The one difference is the **dual masked depth-2 pair: 11
  carried slots at base, 12 with the change**. Everything else matches: the mono masked pair 0 and
  0, both masked tails 0 and 0, the ramped kernels 1 and 1, the select-free pair 10 and 10. The
  count moves with unrelated edits: 12 at `a1fcab3d` (before #999), 12 with the list functions
  outlined, 12 with the whole ramping arm outlined out of `process_bank`. The all-six-live stereo
  settled row (a scratch document with every band and both cuts on, so every pass is a masked
  pair) was compared change minus base in seven paired launches: -8.9, +3.1, +3.1, +3.4, -4.9,
  -4.0, -2.9 µs (median -2.9) on a 143-157 µs row, and in one four-module launch the base (11 slots)
  was the slowest of `a1fcab3d`, base, change and outlined (12 each). No regression is resolvable
  above the ±4 µs launch-to-launch spread. The mono all-six-live row matched within 1 µs throughout.
* V8 output identity (`web_auto.mjs`, `DIGEST=150`, `eq_gain`, `mono_eq` and the two all-six-live
  subjects, all seven arms, base against change): all identical, and every moving arm differs from
  settled. The console mixing-automation preflight: all seven digests equal base's.

### Gate 7 and the timing table

µs per 64-track block; Δ against settled in the same run.

**V8** (`web_auto.mjs`, extracted from the diagnosis patch with `verify-automation-harness.patch`,
paths fixed, plus two all-six-live subjects; 6 rounds x 500 blocks; load 4.6 -> 3.6):

| arm | `eq_gain` base | change | `mono_eq` base | change | all-six stereo base | change | all-six mono base | change |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| settled isolate | 22.37 | 22.29 | 99.56 | 99.38 | 94.20 | 96.66 | 133.88 | 133.64 |
| one lane | +9.52 | **+3.50** | +4.14 | **+1.59** | +4.68 | +4.45 | +1.95 | +2.18 |
| 8 of 64 | +64.90 | **+11.46** | +30.64 | **+3.98** | +28.51 | +27.15 | +13.64 | +13.75 |
| all 64 | +145.68 | **+38.08** | +73.44 | **+20.25** | +73.78 | +70.77 | +39.51 | +39.79 |

Settled holds within 2 % on both gate subjects (-0.4 %, -0.2 %). The all-six stereo settled
isolate is +2.6 % in this launch; see gate 6 for the seven launches, whose median has the change
faster. All-six rows elide nothing, so they price the list's overhead: none resolvable. Engagement:
8 of 64 saves 53.4 µs (stereo) and 26.7 µs (collapsed) against thresholds of 26.3 and 13.7. The
first merged launch (load 9.9 -> 14.7, rising) agrees on every gate figure. Before the merge, three
launches on `a1fcab3d` agree as well: 8 of 64 +10.8 to +11.3 (stereo) and +4.1 to +4.4 (collapsed).

**Browser `console_mixing_automation` arm** (`web-mixing-automation-benchmark.mjs run`, one warmup and
two measured launches per module, alternated; load 6.1 -> 7.7), p50 µs, rounds 1/2:

| module | quiet | restated | automated | paired ramp Δ |
|---|---|---|---|---|
| base | 155.32 / 154.35 | 158.52 / 157.48 | 172.30 / 171.06 | 13.50 / 13.41 |
| change | 154.06 / 153.50 | 158.12 / 156.73 | 161.58 / 160.55 | **3.18 / 3.64** |

On the pre-merge build one launch read quiet +2.2 µs over base with identical EQ loops. It did not
reproduce in the two launches since, and `web_auto` instances of one module differ by up to 4 µs, so
it is instance placement.

**Native** (scratch harness after the diagnosis's `rows`: one `Both` owner edit per riding track,
isolate = row - builtins-only in the same round; base, change, change, base, each 6 rounds x 800
blocks; load 4.5 -> 3.8). `Simd4` binds four-lane EQ and compressor banks through a scratch-only
switch in both builds, as the diagnosis did; the shipped x86 build renders them per node. Rows are
the mean of the two invocations, Δ likewise:

| arm | stereo `Simd8` base / change | stereo `Simd4` | collapsed `Simd8` | collapsed `Simd4` |
|---|---|---|---|---|
| settled row | 35.27 / 35.47 | 60.01 / 60.30 | 69.30 / 68.41 | 93.20 / 93.00 |
| one lane | +8.43 / **+0.97** | +8.43 / **+1.62** | +4.21 / **+0.70** | +4.06 / **+1.12** |
| 8 of 64 | +66.18 / **+7.84** | +66.00 / **+9.62** | +31.66 / **+3.17** | +31.02 / **+2.72** |
| all 64 | +87.72 / **+28.88** | +148.20 / **+34.78** | +46.90 / **+18.42** | +74.04 / **+16.98** |

The stereo settled row reads +0.2 and +0.3 µs (0.6 %, 0.5 %). Before the merge it read +0.5 µs at
both widths, and that was code placement: the stationary depth-one tail loop was base's instructions
with renamed registers, and with `-C llvm-args=-align-loops=64` in both builds (diagnostic only) the
settled rows went 35.81 -> 35.58 at `Simd8` and 62.75 -> 62.34 at `Simd4`. An earlier merged
invocation set was discarded: the load rose from 7.4 to 13.3 during it, and the change's second
`Simd4` pass read 17-41 µs above its first on every arm, settled included.

### For the verifier

* **A pre-existing class-A gap on the stationary path, not fixed here.** `cascade_sections` and
  `cascade_sections_mono` admit the restored-subnormal shape above. A scratch probe on the
  `a1fcab3d` code (one live high shelf at `-6.0206` dB, `ic1 = ic2 = -2^-149`, input `-2^-149`,
  every other section dead) rendered `0x80000000` through the elided cascade and `0x00000000`
  through the full one. It needs a restore payload; the kernel never writes a subnormal. The same
  tightening of leg (c), or a restore that refuses non-zero integrators below `FLUSH_EPS`, would
  close it. It wants its own issue, since #998 is editing that gate.
* **Rebase with #998:** it caches leg (b) per channel and refreshes the cache after an executed
  identity section. The ramping lists read leg (b) directly, and a ramping block can execute
  identity sections, so the merge must keep the direct read here or refresh the cache after a
  ramping block.
* M3, M7 and M8 are red only in the structural gates, by construction.
