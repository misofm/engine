# 017 Dynamic EQ

## Outcome

Implement a bankable dynamic EQ that combines disclosed parametric filters with per-band dynamics.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement per-band filter descriptors, detector source/link, threshold/ratio/range, attack/release, direction, sidechain and quality/latency declarations.

## Required public interfaces/contracts

`DynamicEq` implements `NativeEffect`; every band has stable numeric ID and independent state; declared ports/latency include sidechain and detector topology.

## Deliverables

Effect/note, metadata, allocation-free band storage strategy, test fixtures, kernels, benchmark and listening evidence.

## Explicit non-goals

Unbounded runtime band creation, third-party modules, implicit sidechain link, or linear-phase mode.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- Launch parametric EQ recurrence derivation and runtime proof
- Launch feed-forward peak compressor
- Deterministic graph compiler, sends, submixes, sidechains, and PDC

## Hazards/decisions

Use the accepted conditioned filter decision from issue 045 and dynamics references from issue 013,
restated in the effect note; parameters must define the order of filter/gain updates.

## Acceptance gates with objective measurements

Static mode matches issue 045's 0.05 dB/0.1% response gates; dynamic gain matches the independent
design within 0.1 dB and timing within the greater of one sample or 2%; external sidechain
routing/PDC and bypass latency are sample-exact; the maximum prepared band count has 0 process
allocations.

## Target matrix

Scalar/4-lane/AVX2 native core; browser/mobile through common core.

## Required evidence

Per-band response/envelope fixtures, capacity test, scalar/SIMD data, benchmark and listening record.

## S1 decision and evidence record (2026-10-24, attempt 1)

Slice S1 per the root scope note: the `dynamic-eq` crate and its band oracle
(`dsp_reference::ReferenceDynamicEqBand`), **not** registered in `effect-compiler` (S2).
Algorithm note: `dsp-research/dynamic-eq.md`. Built on the shared `effect_runtime::dynamic_band`
step and its oracle from #15 S1 (unchanged here).

Decisions (root unless marked):
- Four bell bands, cascaded; parameters at `b*16 + id` (12 per band, all live, 64-sample ramps,
  automatable); bell only (dynamic shelves are S4, owner-gated); no console eligibility; latency 0;
  one quality; no lookahead. The spec's "issue 045 static gate" is #87's (word equality + oracle).
- Pole words `c1, a2, a3` and the mix word `m1` from `parametric_eq::design_svf(Bell)` at the
  static gain; dynamics move only `m1[n] = m1 + k A^2 (g - 1) = k (A^2 g - 1)`, `y = x + m1[n] v1`
  (f64-introduction section 4). Detector: see attempt 2 (an SVF at the band's own `Q`), on the
  sidechain when external and connected; an unconnected or absent sidechain falls back (absent on a
  connected instance reads silence, as the compressor's `Detector::Silent`).
- Tail: `NodeTailBound::UNBOUNDED`, the de-esser's ruling. Reopening condition: when #1372 lands
  the certified bound for a live, Q-variable SVF, this effect states its bound from it, and #1378
  counts this effect. `tests/rest.rs` stands in for the tail gate.
- #95 ratchet: `apply_automation` and `state_error` keep their real names;
  `scripts/check-effect-runtime-policy.sh` raises both rows by one (to 6 and 8) with the reason.
- Implementer: the knee width ramps as one word and the curve's `(W/2, 1/(2W))` are redesigned per
  frame only while ramping (the compressor's rule), so the curve stays continuous; the direction
  switch flips at its ramp's midpoint.
- Implementer: a disabled band ramps to the EQ's identity section; once settled, its frozen
  integrators are cleared at the next block boundary (unobservable, the EQ's #1337 D4 effect), so a
  live-disabled lane reaches rest. A segment skips a band that is settled identity and at rest on
  every lane (an exact no-op, so bank and scalar may decide differently and agree).
- One observation tap per band, kind `GainReductionDb` (the only V1 kind); its peak-magnitude fold
  drops an upward band's sign. S2 depends on #1918 (a signed gain-change kind and fold).
- Link modes: no new row; the dynamic EQ calls the shared `detector_level_db` per band, whose
  Maximum and Average rows are in `effect-runtime/tests/dynamic_band.rs`.

Evidence (48 kHz, this worktree):
- Static mode (range 0), 16 bell rows in four 4-band cascades vs the cookbook product: worst
  `1.5e-5` dB (gate 0.05 dB).
- Dynamic vs the oracle (1 kHz burst at -6 dBFS, Q 1.5): change worst `2.8e-4` dB down (+3 dB
  static, 6 dB knee) and `1.6e-4` dB up (gate 0.1 dB); output worst `1.2e-6`; attack 472/472 and
  251/251 samples, release 6645/6645 and 6673/6673; the upward row holds under its +6 dB range
  (the curve asks +18 dB); 1e6 samples finite.
- External detector: bit-exact before the sidechain burst, worst `2.4e-7` against the reference.
- Rest: four bands at Q 16, +12 dB, 200 ms release, band 3 disabled mid-burst: every lane at rest
  138,752 frames after the input stops.
- Mutants (each restored, all red): full reset a no-op (conformance); poles designed at 0 dB
  (static); attack and release swapped (dynamic); the detector SVF fed the main word (external);
  a band skipped when any one lane is idle (bank parity); the collapsed body with a zero partner
  (collapse); identity bands never cleared (rest).
- `paired_spans` (console-workload) is registry-driven, so it is S2's obligation.

## S1 attempt 2 (2026-10-24): verifier folds (root rulings)

- Detector bandwidth (root revises its decision): one detector SVF per band at quality `Q`
  (`design_svf_words_f64`'s 0 dB high-pass row, rounded once, static), for the internal detector
  and as the sidechain detector's shape. Reason: the cookbook `Q` fixes the bell's half-gain band,
  the -3 dB band of a band-pass at `Q`; `k v1` has quality `Q A`, so its level moved by about
  `10 log10(A)` dB with the static gain. Cost: +1 SVF per band per channel. The bell's `k` word is
  no longer read and left the ramps (17 words per band; payload 7 signal words per band). The
  oracle's detector is the RBJ band-pass at `Q`.
- Shared helpers: `WordRamp`, `advance_word_ramps` and `blend` are in
  `effect_runtime::dynamic_band`; the de-esser and the dynamic EQ use them. No #95 row moves (none
  counts these names).
- Signed observation: S2 depends on #1918 (MAJOR 1, not in S1).
- `live_bands` doc: "an exact no-op for finite inputs".
- Evidence: A/B (PR evidence, not committed tests): the de-esser's output bytes (a bank with
  automation, three link modes, listen, a silence run and a mono-collapse span, plus a scalar;
  3,686,400 bytes) identical before (425918785) and after; the dynamic EQ with every static gain
  0 dB (bank, collapse, scalar; 2,764,800 bytes) identical before and after, so at 0 dB the new
  detector is the old one bit for bit. Oracle: change worst `2.8e-4` / `1.6e-4` dB, output
  `1.2e-6`, attack 470/470 and 251/251, release 6643/6643 and 6673/6673; static `1.5e-5` dB;
  external `2.4e-7`; rest 145,280 frames. Mutant: the detector designed at `Q A` again: red
  (`dynamic_mode_matches_the_reference_band`, the +3 dB row).

## S2 decision and evidence record (2026-10-24, attempt 1)

Slice S2 per the root scope note: the dynamic EQ is the tenth launch effect, **insert only**
(not in `CONSOLE_ELIGIBLE_EFFECTS`; root decision). Wiring follows the de-esser's S2 (#1922).

Decisions (root unless marked):
- Observation: the four per-band taps are `GainChangeDb` with the `PeakSigned` fold, unit dB,
  domain `-24 .. 24` (#1918's admission rule), so an upward band reads as a boost. The tap still
  reads the smoother's own signed change word; no state word moved.
- Tail stays `NodeTailBound::UNBOUNDED`: a disabled band ramps to the identity, so #1372a's
  bounded-tail rule does not apply.
- #1928: a retarget restarted only the ramps whose design bits moved, so a one-ulp frequency nudge
  mid-ramp left the bell's and the detector's `a2` on their old schedule (probe: frame 33 at
  fraction 1/32 against 1/64 for the restarted words). Fixed: when any of a band's nine design
  words moves, all nine restart together from their current values, so every SVF triple the
  kernel loads is on the chord to the new design. Class A (the effect was not registered).
- Implementer: a prepared-bypass lane whose wet state alone fails D7 now keeps its dry output, as
  the de-esser's S2 (`graph-compiler`'s `bypass_shunt_identity` was red on block 13, extreme
  input). Class A for the same reason.
- Implementer: the host-web parity session carries the dynamic EQ on `t3` (position 4). The SDK
  evals that boot that fixture set `observationTaps: 4`: the dynamic EQ is the first effect with
  more than one tap, and a cap below an effect's tap count refuses the boot
  (`effect.observation.taps`). That refusal is the existing contract and is unchanged here.

Evidence (this worktree):
- `crates/dynamic-eq/tests/svf_chord.rs` red on the old retarget, green after.
- Browser budget (fresh named twin): `bridgeMetadataBytes` 1,519,458 of 1,628,864; no re-pin.
