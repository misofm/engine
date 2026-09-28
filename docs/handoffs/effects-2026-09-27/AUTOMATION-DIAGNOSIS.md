# Automation cost diagnosis: EQ, compressor and true-peak limiter (2026-09-27)

Base: `codex/batch-plumbing-floor-2` at `49f696c7`; the compressor rewrite (#981-#985), #976,
#980 and #990 are in. Branch `automation-diagnosis`. This is diagnosis only. Every prototype below
was measured and then reverted, and the tree carries none of them. The prototypes, both harnesses
and the scratch tooling are one patch, `automation-diagnosis-prototypes.patch` (applies to
`49f696c7`). Raw timing output is in `automation-diagnosis-raw-timings.txt`. The draft issues are
`issues/automation-1-*.md` to `issues/automation-5-*.md`.

## Verdict

* **On a mono stem, one parameter edit costs as much as a running ramp, and keeps costing it.**
  Mono stems are the product's common case.
  * Any live write to an EQ, compressor or limiter parameter retires that track's mono collapse,
    for the life of the plan. That includes a restatement of the value already held, and the
    web host's both-channel command.
  * It also retires the collapse of every other track in the track's bank.
  * The cause is that the host lowers a per-lane parameter's "both channels" to a Left record
    and a Right record. Each one clears the witness's `LIVE` term at the drain, and `LIVE` never
    comes back.
  * On the 64-track mono console, once every bank has been touched the block costs
    **151 → 243 us under V8 (+61 %)**, 69 → 108 us natively at `Simd8`, and 119 → 188 us at
    `Simd4`. Nothing needs to be moving.
  * A drain that sees both halves of a write, with bit-equal final values, can keep the term
    (**P1**, class A). That restores 151 us, and with P1 8-of-64 automation costs 177 us instead
    of 285 us.
* **The EQ drops a whole bank to its unelided per-section path when one lane ramps.**
  * The fixture's ramping block runs 12 single-chain passes (six sections, two channels) where a
    settled block runs one interleaved pass, identity sections included.
  * The bank costs **7.8-7.9x settled** at every width. Measured on one bank, one Point per block:
    +8.5 us natively at `Simd8`, +8.3 us at `Simd4`, +9.3 us under V8.
  * Eight automated tracks, one per bank, add **+65 us under V8 to a 22 us isolate**, which is
    three times the EQ's whole settled cost.
  * Running only live or ramping sections under the existing elision gate (**E1**, class A) cuts
    that to +11 us. Vector lane writes on the target path (**E2**) cut it to +9 us, and all 64
    tracks from +146 to +24 us.
* **The compressor's ramping prefix is the old one-pass loop with scalar per-lane ramps.**
  * One automated lane makes its bank **2.3-2.7x settled**. All eight lanes make it 4.1x natively,
    because the ramp scan, advance and curve redesign are scalar per lane per frame.
  * Running the prefix as the settled body's two passes, with lane-wide ramps each advanced in the
    pass that reads it (**C2**, class A), cuts it as follows:
    * 8 of 64 tracks: +45 → +14 us under V8, and +40 → +8 us natively;
    * all 64 tracks: +148 → +28 us under V8, and +80 → +10 us natively.
* **Limiter automation is cheap. Nothing is proposed.**
  * One ramping lane costs +0.3 us natively and +1.0 us under V8 (1.05-1.14x the bank).
  * All 64 tracks cost +5 us natively and +7 us under V8. Most of that is one `f64` coefficient
    design per event (about 35-40 ns each).
  * The one trap is a *left-only* ceiling or release write. It permanently unlinks #990's pair,
    at +1.5 us natively and +2.2 us under V8 per bank. That is #990's L2, which is already
    recorded.
* **A "linear segment" is not a separate path.**
  * No shipped route delivers `AutomationSpanKind::Linear` spans to these effects. All three
    declare `AutomationRate::Block`, and their `apply_automation` counts any non-Point span as
    invalid.
  * A host plays a segment as one Point per block along the line, and the effect renders it
    exactly as a knob drag: a 64-sample ramp, then a 64-sample hold, in every block.
  * The segment arm measures the same as the Point arm, within noise, everywhere.
* **The realistic mix moves as follows.** It is the full stereo strip, with 8 of 64 tracks each
  riding one control: 3 EQ gains, 3 compressor thresholds and 2 limiter ceilings.
  * V8: 245 → 286 us today, and 251 us with E1, E2 and C2.
  * Native `Simd8`: 107 → 149 us today, and 116 us with the three.
  * On the mono strip with P1 as well, the mix costs 169 us under V8, against 285 us today.
* **Owner rulings needed:**
  * P1's witness semantics (drain pairing, or a host `Both` record as #210 did for trim);
  * one invariant-based refinement of C2 (optional);
  * the law-changing ideas in section 8, which are flagged and not proposed.

## Method

* **Host and measurement.**
  * Host: AMD EPYC 7313P (Zen 3), Linux 6.8. Toolchain: rustc 1.97.1, release profile (fat LTO,
    one codegen unit), native `+avx2,+fma`.
  * Every timed run held the shared lock (`flock -w 7200 .../timing.lock`, each hold under
    2 minutes) and was pinned with `taskset -c 31`.
  * Load averages are printed in each raw record. They were 4-10 on most runs and 19-20 on the
    last native P1 check, whose absolute numbers are therefore off, though its paired comparison
    stands. `perf` is unavailable.
  * The in-process clock read 3.697-3.701 GHz, from a dependent `vaddss` chain at 3 cycles per add.
* **Native harness** (`tools/console-workload/tests/automation_diag.rs`, in the patch).
  * `SessionRuntime` for the decomposition rows, with the live-console control channel attached
    (`control: true`). The rows are `eq_only`, `compressor_only`, a new `limiter_only` (builtins
    plus the `simd2` limiter), `builtins_only`, the full console and the mono console.
  * Every automated control is first set on every track to the same base value, off the clock.
    Then 64 untimed blocks run.
  * Each block, the arm pushes its Points through `SessionRuntime::push_parameter` (the
    production queue, drained at the top of the next block; the EQ goes through its
    prepared-target owner), off the clock, and times `render`.
  * Six interleaved rounds of 800 blocks, with a builtins-only arm in every round. The isolate is
    row minus builtins in the same round; the table gives the median over rounds of the per-round
    p50.
  * Prototypes are switched per runtime by per-crate atomics read once per block.
  * **Native `Simd4`** means four-lane EQ and compressor banks bound through a diagnosis-only
    switch (`lane::diag::NATIVE_W4`). The shipped x86 build refuses them and renders those
    effects per node. Every native `Simd4` figure is therefore the browser's bank shape on x86,
    as in the earlier diagnoses.
* **V8 harness** (`automation-diag-tools/web_auto.mjs`).
  * It loads the **shipped `host_web.wasm`**, built with the delivery recipe (`+simd128`,
    `strip=debuginfo`, path remaps), into Node 22.23.2 (V8 12.4, `--no-liftoff`).
  * It boots session documents derived from the 64-track fixture: builtins, EQ, compressor,
    limiter, full console and mono console. The console queue is attached (64 records, no
    meters).
  * It stages one command batch per block through the real `miso_engine_web_v1_command_submit`,
    or, for the EQ, through `prepared-control.js`, `eq_target_prepare` and
    `prepared_command_submit`, exactly as the SDK does.
  * Only `miso_engine_web_v1_render` is timed. Five or six interleaved rounds of 500-600 blocks,
    with modules interleaved round by round.
  * Every prototype was a **clean build** (switches compiled in as constants), checked against
    `KERNEL_ROSTER` (rule 1-3 script) and against the base artifact's output digests.
* **Arms.** Each automated arm pushes both channels (the web host's `channel = 2`), except
  `one_left_only`.
  * `settled`: every control at its base, nothing pushed.
  * `one_point`: track 0 alternates base ± step every block, so a 64-sample window is always open
    in one lane of one bank.
  * `one_segment`: track 0 follows a triangle sweep, one new value per block.
  * `one_left_only`: track 0, left channel only.
  * `eight_of_64`: tracks 0, 8, …, 56. That is one lane in each of the 8 `Simd8` banks, or 8 of
    the 16 `Simd4` banks.
  * `all_64`.
  * The mono runs add `untouched`: no write ever, including in the pre-roll.
* **Controls.** EQ band-1 gain (3 ± 0.25 dB) and frequency; compressor threshold (−24 ± 0.5 dB)
  and attack; limiter ceiling (−3 ± 0.25 dB) and release.
* **Identity.** Every prototype was checked for identical output in three ways:
  * 64-track console digests (every subject and arm, both widths natively, and the V8 artifacts);
  * a randomized bank-level EQ differential (see E1);
  * the compressor's existing reference-kernel differential (see C2).

## 1. How a parameter change reaches each effect today

* **Admission (web).**
  * `COMMAND_EFFECT_PARAM` with `channel = 2` on a `PerLane` parameter lowers to **two**
    `EffectControlRecord::Parameter` records, one Left and one Right (`hosts/host-web/src/lib.rs`,
    `into_effect_records`). All three effects' parameters are `PerLane`.
  * An EQ write goes through the prepared-target owner instead: a design off render, then one
    `PreparedTarget` record per channel.
* **Drain.**
  * `EffectControlLane::stage` (`crates/effect-contract/src/live.rs:260`) pops each record, folds
    it into the lane's channel-symmetry witness (`:284`), and stages a `Point` span at the block's
    `first_sample`.
  * A one-channel record is `SymmetryEvent::Desymmetrize` (`symmetry.rs:319`), which clears
    `LIVE`. Nothing ever sets it again.
* **EQ** (`crates/parametric-eq/src/lib.rs`).
  * The rack applies each target (`crates/rack/src/lib.rs:1270`), and `apply_target_lane`
    (`:2148`) calls `start_ramp` (`:1247`). `start_ramp` sets that lane's target and step words
    and `remaining = 64`, through `lane_set`: a store, a scalar write and a reload per word
    (`:932`).
  * `render` (`:2191`) takes `stationary` from `no_ramp_in_flight` on both channels (`:2207`).
    With any ramp in flight, `process_channels` (`:1520`) runs `Channel::process_block` (`:1364`)
    on each channel: six `process_section` passes (`:1376`), every section, identity included.
    Dedicated cuts always run the masked kernel.
  * Each ramping section splits the block at every ramp end, and snaps ended lanes one lane at a
    time (`snap`, `:1329`, twelve `lane_set` per lane).
* **Compressor** (`crates/compressor/src/kernel.rs`).
  * `apply_automation` (`lib.rs:347`) calls `Channel::set_parameter_target` (`:170`): one
    division per event, plus an `f64` `rate_coefficient` for attack and release.
  * `process_block` (`:476`) runs `frames_loop::<L, true>` (`:681`) for
    `min(max_remaining, frames)` frames, which is 64 frames in a Point-per-block ride.
  * Every frame, for both channels, `advance_ramps` (`:217`):
    * scans 7 ramps × `W` lanes;
    * advances each ramping lane in scalar;
    * redesigns the static curve per lane (`design_lane`, `design.rs:156`: `GainComputerCoef::new`,
      with two divisions);
    * reloads `Coef`.
  * The frame law is the general one-pass law. The prefix gets neither #981-#985's two passes nor
    the wet or DualMono arms. The settled body (`settled_main`, `:561`) runs the rest of the block.
* **Limiter** (`crates/true-peak-limiter/src/lib.rs`).
  * `apply_automation` (`:2769`) designs `limit_coefficient` or `release_coefficient` in `f64`
    (`:962`, `:969`) per event, then `LinearRamp::set_target`.
  * With any ramp open, the whole block takes `DISPATCH_RAMPING`: `RampLanes::advance` (`:931`),
    lane-wide, every frame, on 2 ramps per channel.
  * A both-channel write keeps #990's linked pair. A left-only write unlinks it for good.
* **What a lone moving lane does to its bank.**
  * EQ: the whole bank takes the 12-pass path, and settled lanes step by `+0.0`.
  * Compressor: the whole bank takes the ramping prefix for the longest ramp in either channel,
    and every lane pays the scan.
  * Limiter: the whole bank takes the ramping dispatch, and settled lanes' advances are identities.
  * Mono collapse: the whole bank goes dual, because one lane without `LIVE` declines the cohort.

## 2. Cost table

Isolates are in µs per 64-track block. The isolate is the effect row minus `builtins_only` in the
same round, and Δ is the change from `settled`. Cycles per lane-sample are
`µs × 3.7 GHz / 16384`.

### EQ (band-1 gain; frequency within 1 µs of it)

| arm | native `Simd8` | native `Simd4` | V8 `simd128` | path |
|---|---:|---:|---:|---|
| settled | 9.9 (2.2 c/ls) | 19.4 (4.4) | 21.9 (4.9) | stationary: one interleaved pass, identity sections elided |
| one Point / block | +8.5 | +8.3 | +9.3 | that bank: 12 single-chain passes, 2 targets, per-lane snaps |
| one segment | +8.7 | +8.6 | +10.0 | the same |
| one lane, left only | +8.2 | +8.7 | +9.9 | the same |
| 8 of 64 | +67.2 | +66.0 | +64.7 | 8 banks on the 12-pass path |
| all 64 | +89.2 | +148.5 | +145.6 | every bank, plus 128 targets |
| **one moving bank ÷ settled bank** | **7.9x** | **7.8x** | **7.8x** | |

### Compressor (threshold; attack is similar, cheaper per frame, with an `f64` design per event)

| arm | native `Simd8` | native `Simd4` | V8 `simd128` | path |
|---|---:|---:|---:|---|
| settled | 24.8 (5.6 c/ls) | 47.3 (10.7) | 61.9 (14.0) | two-pass settled body |
| one Point / block | +5.0 | +4.0 | +6.6 | that bank: 64-frame scalar-ramp prefix, general law |
| one segment | +5.9 | +4.1 | +6.2 | the same |
| one lane, left only | +4.6 | +3.0 | +5.4 | the same prefix; one channel redesigns |
| 8 of 64 | +40.1 | +30.7 | +46.2 | 8 banks |
| all 64 | +77.1 | +93.9 | +143.8 | every bank, 8 (4) ramping lanes each |
| **one moving bank ÷ settled bank** | **2.6x** | **2.3x** | **2.7x** | all lanes moving: 4.1x (Simd8) |

### Limiter (ceiling; release within 0.5 µs)

| arm | native `Simd8` | native `Simd4` | V8 `simd128` | path |
|---|---:|---:|---:|---|
| settled | 46.2 (10.4 c/ls) | 78.3 (17.7) | 110.1 (24.9) | stationary dispatch, linked pair |
| one Point / block | +0.3 | −0.1 | +1.0 | ramping dispatch on that bank |
| one segment | −0.2 | −0.2 | +0.3 | the same |
| one lane, left only | **+1.5** | **+1.5** | **+2.2** | the pair unlinks (#990 L2) and stays unlinked |
| 8 of 64 | +1.1 | +0.9 | +2.8 | |
| all 64 | +5.1 | +3.7 | +7.3 | 128 events, each about 36 ns (native) or 35-40 ns (V8) of `f64` design |

### Whole console, realistic mix (8 of 64 tracks, one control each), row µs per block

| row | native `Simd8` | native `Simd4` | V8 |
|---|---:|---:|---:|
| stereo console, settled | 106.7 | 185.7 | 245.4 |
| stereo console, 8 of 64 automated | 148.7 (+39 %) | 222.7 (+20 %) | 285.8 (+16 %) |
| … with E1 + E2 + C2 | 116.3 | 192.4 | 251.5 |
| stereo console, all 64 automated | 226.6 | 386.8 | 471.4 |
| … with E1 + E2 + C2 | 127.9 | 217.8 | 286.4 |
| **mono console, untouched (collapsed)** | **69.2** | **119.2** | **151.2** |
| mono console, every track written once, then settled | 107.6 (+55 %) | 188.2 (+58 %) | 243.2 (+61 %) |
| mono console, 8 of 64 automated | 147.8 | 224.7 | 284.8 |
| … with P1 | 91.9 | 139.7 | 177.3 |
| … with P1 + E1 + E2 + C2 | 80.5 | 128.9 | 168.5 |
| mono console, all 64 automated, P1 + E1 + E2 + C2 | 105.8 (base 228.9) | 170.3 (base 387.7) | 232.0 (base 464.9) |

With P1 the collapse counters confirm the mechanism. The collapse holds on every block of every
cohort through both-channel automation (42,112 of 42,112 block-cohorts at `Simd8`). A left-only
write still retires exactly one cohort (36,928). Without P1, no cohort collapses after the pre-roll
writes.

## 3. Where the time goes

### EQ: the ramping block is the unelided six-section schedule, one chain at a time

* **Twelve serial passes instead of one.** A settled bank of the fixture (one live bell) runs one
  interleaved depth-1 pass over both channels, about 20 cycles per frame for the two chains
  together. The ramping bank runs six sections per channel as separate single-chain passes. Four
  of those sections are identity (two dedicated cuts in their masked kernel, two disabled bands),
  and each still pays the full SVF step. That is 12 × 128 frames × about 20 cycles, roughly
  8 µs, which matches the measured +8.3-8.5 µs.
* **Why the bank drops.** `stationary` is whole-bank and whole-block, and the non-stationary
  arm has no elision (`process_channels`, `:1520-1528`). Elision was only ever proven for fixed
  coefficients.
* **Target application: about 0.2 µs per target, natively and under V8.** `start_ramp`,
  `settle` and `snap` write lanes with `lane_set`: a 32-byte store, a 4-byte write, then a
  32-byte reload that cannot be forwarded. That is 18 of them per retarget, and 12 more per lane
  at the ramp's end. This is what separates `all_64` from `eight_of_64`: 112 more targets cost
  +22 µs natively.

### Compressor: scalar ramps inside the generic loop

* **Scalar per-lane control work per frame.** `advance_ramps` runs per channel per frame. It
  scans 7 × `W` `is_ramping` flags, calls `next_value` per moving lane, redesigns the curve per
  moving lane (two divisions and a branch), and stores words that `Coef::load` reloads
  (8 loads, 3 compares).
  * Lane-wide ramps kept in registers, with nothing else changed (**C1**), recover about 60 % of
    the one-lane cost natively (+5.8 → +2.1 µs) and remove the per-lane scaling (all 64: +79.6 →
    +21.6 µs).
* **The one-pass law.** The prefix frames lose the settled body's two-pass scheduling and its wet
  and DualMono arms. Restoring them (**C2**) takes the one-lane cost to +1.2 µs and all 64 to
  +9.9 µs natively.
* **What is left after C2 (about 1.2 µs per ramping bank at `Simd8`):**
  * the exact ramp advance as selects (6-7 ops where the law needs 1 add);
  * the full curve redesign per frame (2 divisions);
  * the per-frame coefficient masks;
  * the per-block gather and scatter of the scalar `LinearRamp`s. Gathering only the moving ramps
    moved nothing.
  * Per-word design (**C2b**, section 5) removes the divisions for a threshold-only ride:
    `Simd4` 8 of 64 goes +8.3 → +6.1 µs.

### Limiter: ramp dispatch and per-event design

* The ramping dispatch costs about 0.07-0.14 µs per ramping bank-block (`eight_of_64` minus its
  16 events).
* The `f64` `db_to_gain` or `exp` design costs about 36 ns natively and 35-40 ns under V8 per
  event.
* A left-only write loses the linked pair's 10-12 %.
* The whole-block dispatch runs `advance()` on all 128 frames although the ramp ends at 64.
  Splitting at the ramp end would save well under 0.1 µs per bank. It is not proposed.

### Mono collapse: one-channel records

* `LIVE` is a monotone latch. The web host's both-channel command arrives as two one-channel
  records, so the first knob touch on a mono track sends its whole bank dual for the rest of the
  session. The measured cost of a dual bank is 55-61 % of the collapsed row.
* The builtins input stage already met this for trim: #210 phase 3 made `channel = 2` one
  `Both` record "or it would retire the track's mono collapse". Effect parameters never got the
  same treatment, and the effects refuse `Both` spans for `PerLane` parameters.

## 4. The class-A floor of the moving case

The constants are those of `docs/rulings/effect-floor-accounting.md`: lane-ops at 3.7 ops per
cycle and 8 lanes.

* **EQ.**
  * A ramping live section adds its 6 coefficient adds on each ramping frame (64 of 128), so the
    fixture's floor goes from 27 to 30 lane-ops, **+11 %** for the moving bank. Target
    application sits outside the frame loop.
  * Measured after E1 + E2: the moving bank costs **2.3x** settled natively (+1.6 µs on 1.2 µs)
    and 2.7x under V8.
  * The named residual is that the per-section path runs the bell's left and right chains as two
    single-chain passes. The settled path interleaves them. Latency-bound, that is about 2x for
    the pass.
  * That residual is E3, not prototyped.
* **Compressor.**
  * A threshold ramp needs one add per ramping frame (the snap is an assignment). Redesigning the
    curve needs nothing beyond passing the word through. The floor is therefore **< +2 %** over
    settled.
  * Measured after C2: **+39 %** for the moving bank natively (+1.2 µs on 3.1 µs) and +66 % under
    V8. The named gap is the exact-advance selects, the full redesign with its two divisions,
    the coefficient masks and the gather/scatter.
* **Limiter.** One add per ramp per ramping frame is < 1 % of about 130 lane-ops. Measured: +5 %
  for the moving bank natively and +14 % under V8. The gap is the dispatch over the whole block
  and the per-event `f64` design.
* **Mono collapse.** The floor of a both-channel write is zero. The collapse's own class-A
  premise holds, because the two channels receive the same operation on the same state. Measured
  today: 55-61 % of the row.

## 5. Ranked changes (all class A)

The A/B runs were in process for native (one binary, per-runtime switches) and on clean builds
for V8. Figures are Δ against settled, in µs per 64-track block.

| # | change | native `Simd8` | native `Simd4` | V8 | draft |
|---|---|---:|---:|---:|---|
| 1 | **P1** Keep the mono collapse through paired both-channel writes | mono console, all written: 107.6 → 69.3 row; 8 of 64: 147.8 → 91.9 | 188.2 → 119.5; 224.7 → 139.7 | **243.2 → 151.5; 284.8 → 177.3** | `automation-1` |
| 2 | **E1** EQ ramping block runs only live or ramping sections under the elision gate | one +9.0 → +1.8; 8/64 +67.7 → +8.9; all +88.7 → +29.8 | +8.1 → +1.5; +65.0 → +8.7; +148.1 → +33.7 | **+10.5 → +2.9; +64.9 → +11.2; +146.0 → +38.7** | `automation-2` |
| 3 | **C2** Compressor ramping prefix as the two-pass body with lane-wide ramps (dual and collapsed) | one +5.8 → +1.2; 8/64 +40.3 → +8.4; all +79.6 → +9.9 | +3.8 → +1.8; +30.4 → +8.8; +94.4 → +18.5 | **+6.3 → +2.6; +45.0 → +13.6; +147.7 → +28.3** | `automation-3` |
| 4 | **E2** EQ target path writes lanes with a vector select, and snaps by mask | on E1: all +29.8 → +18.0 | +33.7 → +20.2 | on E1: all +38.7 → +23.9; 8/64 +11.2 → +9.1 | `automation-4` |
| 5 | **C2b** (after C2, optional) per-word curve design | 8/64 +9.7 → +8.9 | 8/64 +8.3 → +6.1; all +17.7 → +13.4 | not built | in `automation-3` |
| — | Console benchmark row `console_mixing_automation` | measurement | | | `automation-5` |

### P1: keep the mono collapse through paired both-channel writes

* **What.** In `EffectControlLane::stage`, a one-channel `Parameter` or `PreparedTarget` record is
  not folded into the witness when it is drained. At the end of the drain, `LIVE` is cleared
  unless every one-channel write pairs:
  * **spans**: the staged, last-wins `(parameter, Left)` span has a `(parameter, Right)` twin with
    the same value bits, and the other way round;
  * **targets**: the last target per `(slot, Left)` has a last `(slot, Right)` twin with equal
    words, and the other way round.

  `Both` records, `Bypass` and `Observe` are folded as today. The prototype is 50 lines and
  allocation-free (iterators, bounded by the queue capacity).
* **Why it is class A.**
  * The collapse renders the dual bits whenever the two channels' upstream state is bit-equal.
    That is its standing premise, and mono-collapse M1-M3 gate it.
  * A paired drain gives both channels of a lane the same operations on the same prior state in
    the same block: `apply_automation` handles `pending[0][p]` and `pending[1][p]` identically,
    and the EQ applies the same designed words.
  * Span acceptance is symmetric by construction: same value, same parameter, and canonical order
    and capacity guaranteed by staging.
  * EQ target acceptance depends on each channel's prepared `(enabled, kind)`. Paired words carry
    those fields. The owner's publication validates every target against its candidate
    configuration (`effect-compiler/src/control.rs`, `validate_targets`), whose enabled and kind
    entries are the prepared values, which are not automatable. So equal words mean equal
    prepared `(enabled, kind)`, and a pair is permitted on both channels or on neither.
* **Measured.**
  * Identical output on every console arm at both widths and in V8, mono and stereo.
  * The collapse counters show the collapse held through automation, and a left-only write
    retired exactly one cohort.
  * On the stereo console, P1 costs nothing measurable: V8 8 of 64 went 281.9 → 282.6 µs.
* **Wasm risk.** It is not a kernel, so the roster is unaffected. It is in the render closure
  (the drain), so the callgraph gate must show no new trap owner. Artifact +0.6 KB.
* **Gate.** See `automation-1`: a witness-soundness differential (collapsed against forced-dual,
  with state after every block), pairing-failure cases, mutations and the V8 no-regression gate.

### E1: EQ ramping-block elision

* **What.** In `process_channels` and `process_channels_mono`, a non-stationary block computes a
  section list in place of calling `Channel::process_block`:
  * a section is dead when it is the exact identity on every lane of both channels
    (`identity[s]`, which is fresh for any section with no lane in flight) and no lane of it is
    ramping on either channel;
  * dead sections are dropped under the stationary gate's three legs: (a) no `-0.0`, infinity,
    NaN or word above `ELISION_MAGNITUDE_CEILING` in either input plane; (b) dead sections'
    integrators at `+0.0`; (c) no `-0.0` integrator in live sections.
  * The kept sections run `process_section` exactly as today, left then right per section.
* **Why it is class A.**
  * The stationary proof is per dead section. It needs only that no `-0.0` reaches the section
    and that the section's state is `+0.0`.
  * The one new obligation is that a ramping live section emits no `-0.0` from an input that
    carries none. This holds whenever its `m0` word is exactly `1.0` with a `+0.0` step on every
    lane of both channels. Then `m0 * v0 = v0`, and the proof's `m0 = 1.0` case applies whatever
    the other words do.
  * That covers bell, low-shelf, notch and high-pass automation, and every HPF-cut ramp.
  * A ramping section of any other shape (a high shelf's gain, a low pass, an LPF-cut toggle)
    keeps every dead section after it, so no new argument is needed. The LPF is last anyway.
* **Measured identity.**
  * A bank-level randomized differential, E1 against base:
    * random configurations of every kind, with bands off on all lanes so that E1 engages;
    * cut toggles, and one- and two-channel retargets through the production target
      preparation;
    * hostile input: `-0.0`, subnormals, raw bit patterns, ±inf, NaN, 1e31;
    * dual and collapsed.

    It compared every output word and every lane's payload after every block, with no NaN
    relaxation. 300 scenarios × 96 blocks per width passed in release, and 40 × 96 in dev. E1
    engaged on about 4,000 ramping blocks per width.
  * Two mutations go red: dropping leg (a), and eliding a ramping identity section.
  * A third mutation, dropping the unsafe-ramp rule, **does not go red** in that differential,
    because the counterexample needs a constructed subnormal underflow. The issue therefore
    gates the rule structurally.
* **Wasm risk (rule 1).** The first clean build *failed* `KERNEL_ROSTER`: the EQ arithmetic was
  outlined into `render`, and then into `process_bank_inner`. `#[inline(always)]` on both
  restored "parametric-eq f32x4 dual: exactly one function, vector 312". The kernel count
  stayed at 14 and the artifact grew by 5 KB.

### C2: compressor two-pass ramping prefix with lane-wide ramps

* **What.** For `Detector::Main`, the ramping prefix runs `settled_frames`' two passes over
  `SETTLED_CHUNK` chunks, with the same `DUAL_MONO` and `WET` arms.
  * Each channel's ramps are gathered once per block into lane vectors, as
    `current/target/step/remaining`.
  * Pass 1 advances threshold, ratio and knee, and redesigns the curve lane-wide: the same `f32`
    division, with the knee's finiteness branch as a select.
  * Pass 2 advances attack and release (the rate ramps gated by their parameter ramp's
    was-moving mask), and makeup and mix.
  * The ramps are scattered back once. `Silent` and `Sidechain` keep `frames_loop::<L, true>`.
  * The advance is `LinearRamp::next_value` as selects: `remaining == 0` leaves every word alone
    (not only at rest), `== 1` assigns the target and clears the step, and otherwise it adds.
  * The wet arm is taken only when mix is 1 on every lane of both channels and no mix ramp is
    open.
* **Why it is class A.**
  * Every word is produced by the same operation on the same operands as the scalar path.
  * Each ramp advances exactly once per frame, in the pass that reads it. A target depends only
    on the frame's input and the curve words.
  * The reordering across independent frames is #983's.
* **Measured identity.** The crate's reference-kernel randomized differential
  (`settled_body_tests`, the pre-#981 kernel as oracle) was run with the prototype on:
  * 320 seeds × 128 blocks at `f32`, `Simd4` and `Simd8`, in release;
  * random retargets on every parameter and lane, resets, bypass toggles, sidechain and silent
    blocks, and hostile input;
  * every output word and state word compared by bits.

  C1, C2, and C2 with C2b all passed. The only relaxation is the wet arm's documented
  signalling-NaN quieting in blocks that `finish_channel` rejects.
* **Wasm risk.**
  * The roster held: "compressor f32x4 dual", vector 516 → 1,084, 14 kernels.
  * The prototype carried both C1 and C2, and the artifact grew by 56 KB. The issue keeps C2 only
    and must check `check-web-boot-budget.mjs`.
  * Moving the dual prefix off `advance_ramps` inlined that function into
    `process_block_mono::<f32x4>`, whose scalar count went 0 → 9 (budget 26). The collapsed body
    must get the same rewrite: `frames_loop_mono::<L, true>` is the mono stems' prefix.
* **C2b (optional).**
  * Each curve word depends on one parameter: threshold on threshold, `1/R − 1` on ratio, the
    knee pair on knee. So a lane whose threshold alone moves needs only the threshold word
    rewritten.
  * This is exact given the invariant `words[0..4] == design(ramps[0..3].current)` on every lane.
    Preparation, both resets, restore (`redesign`) and every ramp frame maintain it.
  * It passed the same differential. It is class A **by an invariant**, not by construction, so
    it is flagged for the owner (section 8).

### E2: EQ vector lane writes on the target path

* **What.**
  * `start_ramp` and `settle` write the target, step and coefficient words with
    `select(lane_mask(track), splat(word), vector)`. `lane_mask` loads a static one-hot row, so
    there is no store-to-load round trip.
  * `process_section` snaps every lane that ended in a segment by one masked select per word, in
    place of per-lane `snap`.
  * `lane_get` stays: a narrow load from a wide store forwards.
* **Why it is class A.** `select` is bitwise, so the stored words are exactly the scalar
  writes' words, `+0.0` included.
* **Measured identity.** The same bank differential (modes 2 and 3) and every console digest.
* **Wasm risk.** It adds no scalar `f32` arithmetic to `process_bank`, and `start_ramp` stays
  `#[inline(never)]`.

## 6. Should the console benchmark gain an automated row? Yes, one

`console_automation` exists. It is one compressor threshold on one track, on the compressor-only
row, native only. It sees none of the three findings that matter most:

* the EQ's whole-bank path;
* the collapse loss;
* a spread of automated tracks across banks.

The proposed definition is `automation-5`, row `console_mixing_automation`:

* **Session.** `sixty_four_track_console_mono`, the mono fixture as written: the product's common
  case, and the only one where the collapse is at stake. The plan is `control: true`.
* **Automated controls.** 8 of 64 tracks, one control each:
  * `ch00`, `ch24`, `ch48`: EQ band-1 gain, 3 ± 0.25 dB;
  * `ch08`, `ch32`, `ch56`: compressor threshold, −24 ± 0.5 dB;
  * `ch16`, `ch40`: limiter ceiling, −3 ± 0.25 dB.

  Each is pushed Left and Right with the same value every block (the web host's `channel = 2`).
  The value alternates base + step and base − step, so every block opens a 64-sample window.
* **Pre-roll.** 64 blocks after the settling write.
* **Arms, alternated per observation:**
  * `quiet`: no write, ever;
  * `restated`: the eight controls restated at base every block;
  * `automated`.
* **Record.**
  * p50, p95 and p99 per arm;
  * the paired deltas `automated − restated` and `restated − quiet`;
  * `bank_collapse_counters` per arm;
  * the automated track and effect ids.
* **Asserted in-run.** `quiet == restated` digests, and `automated != restated`.
* **Descriptive only.** Native `Simd8` and the wasm console arm.
* **What today shows.** `restated − quiet` shows the collapse loss, which is non-zero today and
  about 0 after P1. `automated − restated` shows the ramping costs.

## 7. Not worth doing, or not prototyped

* **Limiter.**
  * Splitting the ramping dispatch at the ramp end: under 0.1 µs per bank.
  * Caching or precomputing the `f64` event design: 35-40 ns per event, and moving it off render
    means a prepared-target contract for the limiter, which is architectural.
  * Re-linking #990's pair after a left-only write: #990 L2, a liveness question that needs a
    full gain-state comparison.
* **E3: interleave the ramped left and right passes in the EQ.** It needs a ramped variant of
  `svf_cascade_interleaved` split at the union of both channels' ramp ends. The residual it
  targets is measured: E1 + E2 one moving `Simd4` bank costs +2.3 µs under V8 against a
  1.37 µs settled bank. It was not prototyped, so no saving is quoted. It is the next EQ step
  after E1 and E2.
* **Lean gather in C2** (gather only the moving ramps): no measurable change.
* **The per-target control work beyond E2**: queue pop, decode, `refresh_identity`'s six scans.
  About 95 ns per target natively after E2. `start_ramp`'s `refresh_identity` cannot change the
  flag, because the coefficients do not move there. Dropping it is exact, but it breaks a
  maintenance rule the code keeps on purpose.
* **Segments as their own path.** None exists (section 1), and the measured cost equals the Point
  arm.

## 8. For the owner

1. **P1's form.** The prototype pairs at the drain, which works with today's hosts. The
   alternative follows the #210 trim precedent: the web host lowers `channel = 2` on a `PerLane`
   effect parameter to one `Both` record, and the drain expands it to Left and Right spans (and
   the EQ owner emits a `Both` target). That is cleaner, but it changes admission and the owner
   path. Either is class A. Rule which one.
2. **C2b** is exact by a state invariant rather than by construction. Is an invariant with a
   standing debug assertion and a randomized oracle enough?
3. **Law changes, flagged and not proposed. Each changes the sound and needs DSP evidence:**
   * a smoothing window equal to the quantum, so that a played segment is a continuous ramp
     rather than a ramp and a hold per block;
   * sample-rate `Linear` spans for these parameters;
   * interpolating the EQ's design parameters (frequency, Q) instead of its words;
   * shorter or longer windows;
   * a per-block (not per-sample) coefficient update for the compressor curve.

   No projection is quoted for any of them.
4. **Browser evidence** is Node 22.23.2 (V8 12.4, `--no-liftoff`), not Chrome.
5. **Native `Simd4` figures** use a diagnosis-only four-lane bind. The shipped native `Simd4` leg
   renders the EQ and the compressor per node.
6. **#986 is superseded.** `automation-3` is its measured and prototyped slice.

## Reproduction

The patch `automation-diagnosis-prototypes.patch` applies to `49f696c7`. It contains:

* the switches:
  * `lane::diag::NATIVE_W4`;
  * `parametric_eq::diag::MODE` (bit 0 E1, bit 1 E2);
  * `compressor::diag::MODE` (bit 0 C2, bit 1 C1, bit 2 lean gather, bit 3 C2b);
  * `effect_contract::live_diag::PAIRED` (P1);
* the prototypes;
* `tools/console-workload/tests/automation_diag.rs` (`rows`, `identity`);
* `automation_diag_identity.rs` (the EQ bank differential);
* a `SixtyFourTrackLimiterOnly` workload and `SessionRuntime::control_channel`;
* the compressor's `diag_ramping_prefix_randomized`;
* `docs/handoffs/effects-2026-09-27/automation-diag-tools/`: `web_auto.mjs`, the build and run
  scripts, and the document generator.

```text
CARGO_INCREMENTAL=0 cargo test --release -p console-workload --test automation_diag --no-run
flock -w 7200 LOCK taskset -c 31 target/release/deps/automation_diag-<hash> --ignored --nocapture \
    --test-threads 1 rows     # AUTO_DIAG_WIDTHS=8,4 AUTO_DIAG_SUBJECTS=eq_gain,... AUTO_DIAG_MODES=0,1,...
cargo test --release -p console-workload --test automation_diag_identity -- --ignored --nocapture
cargo test --release -p compressor --lib diag_ramping_prefix_randomized -- --ignored --nocapture
build_web.sh NAME; ROUNDS=6 BLOCKS=600 run_web.sh SUBJECTS ARMS t-web-A/.../host_web.wasm t-web-B/...
DIGEST=150 node --no-liftoff web_auto.mjs SUBJECTS all A.wasm B.wasm   # output identity
```

`AUTO_DIAG_MODES` values are hexadecimal:

* bits 0-7: EQ;
* bits 8-15: compressor;
* bits 16-23: limiter (unused);
* bit 24: P1.

For example, `0x1000103` is P1 + E1 + E2 + C2. A V8 prototype build compiles its switch as a
constant: `mode()` returns the value, and P1's `paired` is `true`. The EQ one also carries
`#[inline(always)]` on `render` and `process_bank_inner`.
