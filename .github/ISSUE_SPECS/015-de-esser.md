# 015 De-esser

## Outcome

Implement a transparent dual-mono de-esser with disclosed detection and attenuation behavior.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement wideband and split-band attenuation, selectable band detection, frequency/bandwidth, threshold, ratio/range, attack/release, listen output, detector link mode, external sidechain, optional lookahead and declared quality modes.

## Required public interfaces/contracts

`DeEsser` implements `NativeEffect`; detector/filter topology and listen port are exposed; latency/tail and all frequency/dynamics units are numeric metadata.

## Deliverables

Algorithm note, effect code, fixtures with sibilance/noise controls, scalar/4-lane/8-lane kernels, tests, benchmark and listening evidence.

## Explicit non-goals

Machine-learning speech classification, undocumented dynamic EQ replacement, implicit stereo detection, or realtime FFT allocation.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- Launch parametric EQ recurrence derivation and runtime proof
- Launch feed-forward peak compressor

## Hazards/decisions

Document the chosen filter/detector equations and cite primary sources in the DSP note; tests must distinguish desired-band attenuation from broadband gain change.

## Acceptance gates with objective measurements

In split mode, tones outside the affected band change by no more than 0.1 dB where the documented crossover response is nominal; in-band burst gain reduction matches the independent design within 0.1 dB and timing within the greater of one sample or 2%; wideband, split, listen, sidechain and link outputs are deterministic; one million samples produce no NaN/Inf; 0 render alloc.

## Target matrix

Native scalar/AVX2 and 4-lane portable/Wasm core.

## Required evidence

Band-response plots, burst traces, scalar/SIMD comparisons, cycles/frame, and blinded vocal listening record.

## S1 decision and evidence record (2026-10-24, attempt 1)

Slice S1 per the root scope note: the `de-esser` crate and its oracle, **not** registered in
`effect-compiler` (S2). Algorithm note: `dsp-research/de-esser.md`.

Decisions (root unless marked):
- Design as the scope note: one SVF per channel (words from `parametric_eq::design_svf_words_f64`,
  rounded once), band or high detector tap, hard-knee downward law clamped by range, all link
  modes, split `x - (1-g) f`, wideband `g x`, listen `f`; ten live parameters with 64-sample
  ramps; latency 0; one quality; one Gain Reduction tap; an optional sidechain-in port. `listen`
  is a Boolean parameter, no new port role. No lookahead. Defaults 6.5 kHz, Q 2, -20 dB, 4:1,
  20 dB, 1 ms, 50 ms.
- Scope change (root, mid-attempt): the detector and gain law are one shared module,
  `effect_runtime::dynamic_band` (`band_step`, `detector_level_db`, `dynamic_band_gain`,
  `applied_gain`; downward/upward and a knee for #17), with the `f64` oracle
  `dsp_reference::dynamic_band`; the de-esser applies only its output law
  (`dsp_reference::de_esser` is its oracle). The band step's oracle test lives once in
  `effect-runtime` (`tests/dynamic_band.rs`), which needed `dsp-reference` as a dev-dependency of
  `crates/effect-runtime/Cargo.toml`.
- Implementer: the split, listen and detector arms all use the **selected** tap `f` (band or
  high), so high-detector split mode is a dynamic high-pass cut; recorded for review.
- Implementer: the smoother is the rate form `G' = snap(G + c (t - G))` (the gate's, with the
  #1336 coefficient ramps), attack while `|t| > |G|`; the snap is the compressor's plateau edge
  `5.38e-7` dB (#1784 B1), without which a 500 ms release rests after about 25 s.
- Tail (root ruling, accepted): `NodeTailBound::UNBOUNDED`. The split and listen arms carry the
  SVF ring-out, so a follower bound alone under-declares. Reopening condition: when #1372 lands the
  certified bound for a live, Q-variable SVF, this effect states its bound from it, and #1378
  (retire the Infinite tail) counts this effect. `tests/rest.rs` replaces the tail-contract gate.
- Implementer: the "nominal region" of the 0.1 dB gate is computed, not the note's "1 octave at
  Q >= 1": 3.72 / 2.75 / 1.83 / 1.08 / 0.58 octaves from the centre at Q 0.5 / 1 / 2 / 4 / 8, for
  any range up to 40 dB (`dsp-research/de-esser.md`).
- #95 ratchet (root ruling): effect-runtime has no shared span pass (`effect_contract::valid_runtime_span`
  is the contract's whole-block law, which a drop-and-count effect cannot call), and each effect's
  pass commits into its own ramp layout, so a shared one would be the #95 sweep across the effects.
  The de-esser keeps the real names `apply_automation` and `state_error`, and
  `scripts/check-effect-runtime-policy.sh` raises those two rows by one each (to 5 and 7), with the reason.
- Selected tap (root, accepted): split mode with the high-pass detector is a dynamic high cut;
  stated on the detector parameter's doc.
- Gain snap in the shared module (root, accepted): class A. Only `effect_runtime::dynamic_band` has
  it; the compressor, gate and multiband code is unchanged against main 82024d367 (`git diff` empty),
  so no shipped bit moves.
- Production size: about 1,650 lines (`lib.rs` 1,378, `kernel.rs` 277, `dynamic_band.rs` 138),
  over the 2026-08-23 comment's 600. The bulk is the contract surfaces the shipped effects carry
  (descriptor table, state payload and its validation, bank, collapse, rest, clocks, lane-defaults
  exchange).

Evidence (48 kHz, this worktree):
- Band step vs oracle (8 kHz burst, -6 dBFS, 6.5 kHz Q 2, 1e6 samples): worst change error
  `2.2e-5` dB (downward, hard knee) and `2.9e-5` dB (upward, 6 dB knee); attack 263/263 and 266/266
  samples, release 5528/5528; deepest change -7.89 dB; all samples finite.
- Output law vs oracle: worst `2.9e-7` over wideband, split (band and high) and listen.
- Split isolation (defaults, 7 kHz burst over 1 kHz): 1 kHz moves `-0.020` dB, 7 kHz `-7.0` dB.
- Rest: Q 8, 50 ms release, noise burst: all lanes at rest 38,784 frames after the input stops.
- Mutants (each restored): swapped attack/release (dynamic_band red); split sign `(1-g)` (output
  law red); band widened 4x in the effect and the oracle together (isolation red, output law
  green); lane 1+ reading lane 1's automation offsets (bank parity red); collapsed body with a
  zero partner (collapse red); `advance_clocks` advancing one channel (rest red); full reset a
  no-op (conformance red). Survived: SVF joint flush disarmed (rest stays green: this fixture's
  SVF reaches `+0.0` by the per-word flush alone, and the gain release settles last). That
  mutant is owned by #1372's tail and rest-bound gate, part of the tail's reopening condition.

## S1 attempt 2 (2026-10-24): verifier folds

- Link modes: `effect-runtime/tests/dynamic_band.rs` gains two rows that feed the burst to the
  partner channel only, Maximum and Average, against the oracle's level of `max(|a|, |b|)` and of
  the mean, within 0.1 dB.
- Sidechain wiring: `de-esser/tests/output_law.rs` gains a row with a steady -10 dBFS 1 kHz main
  tone and the burst on the connected sidechain only; the wideband output is `g x` with the
  reference's change for the sidechain input (within `1e-5`), and bit-exact before the burst.
- `band_step` takes a block-constant `armable` and runs `svf_step_when`; the de-esser decides it
  per segment with `silence_armable_holding` and advances an unarmable segment's silence clocks
  with `silence_skip_block`, as the multiband does (#1328 A9).
- `tests/rest.rs` doc corrected: this fixture rests through the gain release.
- Evidence: the new link rows are red on Maximum or Average made a no-op in
  `detector_level_db`; the sidechain row is red on the connected detector filtering the main input
  (each restored). One-time A/B (PR evidence, not a committed test): a bank with mono collapse,
  two link modes and listen on alternate lanes, plus a scalar instance, 400 blocks with silences of
  10,240 frames (armed and unarmable segments both occur): 3,686,400 output bytes identical before
  (b75d18b6e) and after the `armable` change.

## S2 attempt 1 (2026-10-24): registry and consumer wiring (insert only)

- `miso.de-esser` is the ninth launch factory (`effect-compiler`, registry order last). Root
  decision: insert only at launch; it is not on decision 12's console list until the owner rules
  (S3). Wired: `check-effect-runtime-policy.sh` boundary, `SIDECHAIN_EFFECTS`, the SDK assets and
  codegen, `EFFECT_LINK_MODES` (all three modes), `SESSION_SCHEMA_V1.md` (ports, eligibility),
  the `qualification.yml` DSP job `-p` and workspace `--exclude` lists, and every test row that
  enumerates the registry (counts 8 -> 9, ports 18 -> 21, taps 4 -> 5, quality rows 32 -> 36).
- Defect found by `bypass_shunt_identity` (per-node leg, Extreme level): a prepared-bypass lane
  whose wet state (SVF integrators) alone failed D7 zeroed its dry output, where the graph's
  shunt passes it. `finish_block` now zeroes a bypassed lane only when its output fails; the state
  is still recovered and counted. Red before the change, green after.
- `parameter_lattice`'s geometric-step check now skips intrinsic points. The binding lattice law
  (`crates/effect-contract/src/step.rs:7-12`, #239 ruling 5461507633 section B) allows the
  endpoint and default detents one irregular adjacency; the de-esser's Q (0.5-8) ends
  7.99821205 -> 8, gap 0.0018, below the first generated gap. Every geometric row of the nine
  effects ends on a clamp; only this one exposed the old rule.
- `observation_identity`: the de-esser was born with its tap, so its contract is 1.0 (no bump).
- `console-workload` `paired_spans`: the de-esser is collapse-capable, so it joins the
  collapse-capable set and carries an A3 row (every link mode, 10 live PerLane parameters, 97
  twin pairs), as the gate and multiband do.
- No new test: the de-esser rides the existing per-effect rows (`parameter-metadata` round trip
  boots every registry effect as an insert and renders it on the web host; the SDK
  `console-evals` boot each catalog effect at every link mode and hear its bypass lifted).
- Browser budgets unmoved (`check-browser-expected-resources.py`, `check-web-audioworklet.sh`
  boot high-water gate both green; no re-pin).
- SVF chord fix (#1372a verifier finding): `retarget` skipped a ramp whose design bits did not
  move, so a mid-ramp one-ulp frequency nudge that keeps `a2`'s bits left `a2` on its old schedule
  while `c1` and `a3` restarted; off-chord triples are not contractive (spectral radius up to 1.36
  at the extreme). When any SVF design word moves, the three words now restart together from
  their current values, so every loaded triple is a convex combination of the current point and
  the new design. `tests/svf_chord.rs` is red before the change (frame 33: fractions 1/64, 2/64,
  1/64) and green after. Class A: the de-esser is not registered on main, so no shipped bit moves.
