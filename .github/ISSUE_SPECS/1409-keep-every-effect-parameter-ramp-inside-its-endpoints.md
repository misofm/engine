# Keep every effect parameter ramp inside its endpoints

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-05 as the successor of *Keep every trim, fader and matrix ramp inside its endpoints*
(#1408), whose D7 left the effect ramps out. Code anchors verified on `codex/d15-stream-g` at
`b800633e2`; the D6 and D8 test anchors (compressor, limiter, soft clip) on `1a2a6bc4b`.

## Product outcome

A live effect parameter move never renders a value outside the values it moves between. Today
every effect ramps a parameter (or a coefficient designed from one) with the D11 law over 64
samples, and its accumulated rounding can carry the word up to about 30 ulps past its target
before the final snap: a ratio ramp from `1.0000039` to its domain floor `1.0` reaches
`0.99999994` at frame 34, below the ratio domain. After this slice every effect ramp word stays in
`[min(start, target), max(start, target)]` on every frame, at every lane width, with no new state,
through the clamp #1408 adds for the builtins.

## Context

- **The law.** One division at the event, `step = (target - current) / n`
  (`crates/effect-runtime/src/ramp.rs:79-89`); then `current = current + step` per sample and an
  exact assignment of the target on the last (`:140-155`); `docs/EFFECT_CONTRACT_V1.md:133-147`
  states it. `current + step` accumulates rounding, as for the builtins (#1408 Context).
- **The probe (2026-10-05, `/tmp/claude-1002/d15-g-effect-ramps/probe.py`, output `out.txt`).** The
  scalar law at `n = 64`, for edge targets `T` and starts `T ± d` ulps, `d` in `1..=4096`: 421 to 480
  of the 4,096 starts on each side leave `[min, max]` before the snap, by up to 30 ulps of `T`.
  Domain-edge examples (first frame out, word): ratio `1.0000039 -> 1.0` (frame 34, `0.99999994`),
  mix `0.99999428 -> 1.0` (frame 49, `1.0000001`), delay feedback `0.949998 -> 0.95` (frame 34,
  `0.95000005`), ratio `19.999937 -> 20.0` (`20.000002`), threshold `-79.99975 -> -80.0`
  (`-80.0000076`), gate range `95.99975 -> 96.0` (`96.0000076`). Every one is a reachable pair of
  in-domain values. No proof of "no overshoot" exists to be had, so this slice takes the clamp (D0).
- **The windows.** Every effect retarget passes its crate's 64-sample constant, and each restore
  validator bounds `remaining` by the same constant: compressor `SMOOTHING_SAMPLES`
  (`crates/compressor/src/design.rs:47`; `kernel.rs:152`, `:160`; the hold ramp `:166-171` has a
  zero step), gate `RAMP_SAMPLES` (`crates/gate-expander/src/lib.rs:55`, `:587`), multiband
  `SMOOTHING_SAMPLES` (`crates/multiband-compressor/src/lib.rs:105`, `:1291`), delay
  `RAMP_SAMPLES` (`crates/delay/src/lib.rs:64`, `:1470`, `:1475`), soft clip `RAMP_SAMPLES`
  (`crates/soft-clip/src/lib.rs:53`, `:644-647`), transient shaper `RAMP_SAMPLES`
  (`crates/transient-shaper/src/lib.rs:67`, `:795-798`), true-peak limiter `RAMP_UPDATES`
  (`crates/true-peak-limiter/src/lib.rs:89`, `:3865-3868`). So the largest window an effect ramp
  receives is 64. `LinearRamp::set_target` and `ParameterSmoother` themselves accept any `u32`.
- **Render sites of the law (each `current + step` iterated):**
  1. `lane::kernels::ramp_block` (`crates/lane/src/kernels.rs:862`, update `:873`, frozen order
     `:850-860`; `RampSegment` `:839-848`). No production caller: its users are
     `crates/effect-runtime/tests/ramp.rs:23`, `crates/lane/tests/support/mod.rs:528` and the lane
     corpus (`tools/wasm-gate-corpus/src/lib.rs:1380`, pinned in `LANE_DIGESTS`, `:1761`).
  2. `LinearRamp::next_value` (`crates/effect-runtime/src/ramp.rs:140-155`, update `:150`) and
     `LinearRamp::advance_block` (`:175-196`, whose first-frame word `current + step` is `:176-180`).
     Users: the compressor's sidechain prefix `Channel::advance_ramps`
     (`crates/compressor/src/kernel.rs:198-230`, all seven parameters and the attack and release
     coefficient ramps); the transient shaper's `advance` (`crates/transient-shaper/src/lib.rs:398-410`:
     attack amount, sustain amount, mix); the delay's `chunk_of` (`crates/delay/src/lib.rs:982-1005`)
     and cross ramp (`:880`), which feed site 7.
  3. `effect_contract::ParameterSmoother`'s `Linear` arm (`crates/effect-contract/src/lib.rs:1608`):
     the contract's statement of the law, no render user, proven bit-identical to site 2 by
     `crates/effect-runtime/tests/contract_ramp_identity.rs`.
  4. Compressor `RampVec::advance_where` (`crates/compressor/src/kernel.rs:962-971`, update `:967`),
     the main detector's bank path: threshold, ratio, knee, attack, release, makeup, mix and the two
     rate coefficients.
  5. Gate `channel_step`'s `RAMPING` prologue (`crates/gate-expander/src/kernel.rs:173-195`, update
     `:187-191`): threshold, ratio, range, hysteresis.
  6. Multiband `run_segment` (`crates/multiband-compressor/src/lib.rs:889-912`, update `:906-911`),
     fed by `Side::segment` (`:1069-1083`, `Segment` `:643-646`, which carries no target) and
     written back by `store_segment` (`:1091-1101`): per band threshold, ratio, attack, release,
     makeup.
  7. Delay `delay_chunk` (`crates/delay/src/lib.rs:1027`, starts `:1068-1072`, updates
     `:1125-1131`), fed `(start, step)` pairs by `LaneChunk` (`:753`) and `CrossChunk` (`:775`) in
     chunks bounded by `ramp_bound` (`:953-959`): per lane the damping coefficient `g`, feedback and
     mix, and the shared cross-feedback position. It adds the steps on every frame, ramping or not.
  8. Soft clip `soft_clip_block` (`crates/soft-clip/src/kernel.rs:166`, update `:186-188`), with
     block-constant `SoftClipCoef` (`:59-70`, steps only) built per segment by `process`
     (`crates/soft-clip/src/lib.rs:489-538`, `step_vector` `:540`, write-back `synchronize_currents`
     `:550`): drive and output linear gains, mix. It adds the steps on every frame, ramping or not.
  9. True-peak limiter `RampLanes::advance` (`crates/true-peak-limiter/src/lib.rs:1169-1176`, update
     `:1173`): the limit coefficient (from the ceiling) and the release coefficient.
- **Restore validators.** `ramp_path_inside` and `ramp_path_within`
  (`crates/effect-runtime/src/state_payload.rs:284-318`) walk a carried ramp's remaining path with
  the unclamped update (`:310-317`); every effect's payload reader calls one (compressor
  `crates/compressor/src/state.rs:130`, `:150`; gate `lib.rs:788`; multiband `lib.rs:1397`; delay
  `lib.rs:1626`; transient shaper `lib.rs:683`; limiter `lib.rs:4084`) with a 64-ulp slack that
  exists for this overshoot. Delay's `a_carried_ramp_is_refused_unless_its_whole_path_is_valid`
  (`crates/delay/src/lib.rs:2746`) refuses two forged finite steps whose path leaves the domain
  (`"path past the domain"`, `:2773-2778`), and the limiter's restore corruptions
  (`crates/true-peak-limiter/src/lib.rs:7613`) refuse `"moving limit walked past its bounds by its
  step"` (`:7623-7625`, current and target at the ceiling, step `-1e30`, 8 remaining), and the
  compressor's payload tests refuse a mix step that walks out of `[0, 1]`
  (`a_step_that_leaves_the_domain_before_the_snap_is_refused`,
  `crates/compressor/tests/payload.rs:391-418`) and `"a moving path below zero"` on the release
  coefficient (`:449-450`, in `a_coefficient_below_zero_or_above_its_design_is_refused`): all rest
  on the walk alone.
- **Tests that assert the old law's overshoot.** Soft clip's
  `an_overshooting_ramp_restores_and_continues` (`crates/soft-clip/tests/state_roundtrip.rs:155`)
  asserts that its snapshot's `current` has crossed the edge (`:177-181`); its four callers
  (`:247`, `:255`, `:267`, `:276`) and `a_drive_overshoot_retargeted_inward_restores_and_continues`
  (`:294`, which starts from an overshot drive) rest on that premise. Soft clip's randomized
  `a_restored_near_edge_ramp_continues_bit_for_bit` (`crates/soft-clip/tests/randomized.rs:199`)
  asserts that its generator crosses every edge (`:200`, `:209-221`, fed by `near_edge_case`'s
  crossing report, `:193`). After D1 no engine ramp crosses its target, so all of these turn red.
- **#1301's probe.** Stream J's #1301 (*Make the shared edge-ramp restore probe cheap enough for
  every pull request*) rewrites `edge_ramp_restore_violations` (`crates/conformance/src/randomized.rs`)
  and records its catches as mutation counts measured on the unclamped law (its gate 1: delay M18
  24 refusals, gate strict current 32, limiter 4-ulp budget 4). Each catch is an effect's own
  snapshot past its target being refused, so after D1 the counts may fall to zero.
- **Oracles of the law in tests:** `crates/effect-runtime/tests/{ramp,partition,lane_identity,stationary_hoist,contract_ramp_identity}.rs`,
  `crates/soft-clip/tests/ramp_law.rs`, `crates/transient-shaper/tests/partition.rs`, the
  compressor kernel unit tests (`crates/compressor/src/kernel.rs:1530-1640`), and the delay unit test
  that iterates `segment.step` by hand (`crates/delay/src/lib.rs:2021-2046`).
- **Pinned artifacts that may render an effect ramp:** `D1_DIGESTS`
  (`crates/effect-runtime/src/corpus.rs:252`), `C1_DIGESTS` (`crates/compressor/src/corpus.rs:236`),
  `GATE_DIGESTS` (`crates/gate-expander/src/corpus.rs:212`, `gate_digests.in`), multiband `DIGESTS`
  (`crates/multiband-compressor/src/corpus.rs:252`, `corpus_digests.in`), delay `G5_DIGESTS`
  (`crates/delay/src/corpus.rs:231`), `SOFT_CLIP_DIGESTS` (`crates/soft-clip/src/corpus.rs:197`,
  `corpus_digests.in`), transient `CROSS_TARGET_DIGESTS` (`crates/transient-shaper/src/corpus.rs:78`),
  limiter `D90_DIGESTS` (`crates/true-peak-limiter/src/corpus.rs:66`) and `LANE_DIGESTS`; all are
  read by `tools/wasm-gates/tests/g5_native_corpus.rs`, the single cross-target corpus owner, which
  prints the scalar-oracle digests on a mismatch.
- **Not this law.** The parametric EQ's SVF coefficient word ramps (`svf_block_ramped*`,
  `crates/lane/src/kernels.rs:664-731`; `crates/parametric-eq/src/lib.rs:1417-1440`, window 64) are
  a filter retarget law, like #1408's input filter sites 5-6 and #1407. The delay's tap crossfade
  (`crates/delay/src/lib.rs:988-1001`, `:1143-1153`) steps `alpha` by `1/128` from an exact `j/128`,
  so every word is exact and it provably cannot overshoot. `SmoothingRule::OnePole99` has no
  native effect user. The indexed route ramp is a function of the frame index (#1408 Context).

## Decisions frozen for this slice

- **D0. Root decision (2026-10-05).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), approved this issue: every effect
  parameter ramp stays inside its endpoints, either proven free of overshoot by a reachable-input
  scan or clamped with #1408's branch-free `ramp_toward`. The scan (Context, "The probe") finds
  reachable overshoots at the 64-sample window on every domain edge it tried, so this slice clamps.
  Rationale: a ramp that leaves `[start, target]` renders a value the user never set, here past a
  parameter's own domain (a ratio below 1, a mix above 1, a feedback above 0.95), and one law for
  builtins and effects keeps one proof. Stream G files and owns it.
- **D1. The law.** Every site of the list becomes, for each ramped word,
  `ramp_toward(current, step, target)` in place of `current + step`, keeping its own snap (the
  countdown, segment split or select on the last frame) unchanged. One body: #1408's
  `ramp_toward` (in `crates/lane/src/kernels/builtins.rs`), re-exported as
  `lane::kernels::ramp_toward` and called at `L = f32` by the scalar sites (2, 3, 7, and
  `advance_block`'s first-frame word). No second copy.
- **D2. Why it is exact, for any finite step.** `ramp_toward` returns a value inside
  `[min(current, target), max(current, target)]` (for a non-NaN `next`), and that interval lies
  inside the previous frame's, so by induction every word lies between the word at the event (or
  at the restore) and the target, whatever the finite step. With an engine step, #1408 D2 adds that
  the clamp acts only on a word that would pass the target.
- **D3. Operand order.** As #1408 D3: `high.min(low.max(next))`, so an in-range word keeps today's
  bits, signed zeros included, and a NaN step propagates.
- **D4. Targets reach the kernels, no new state.** Sites 6-8 get the target words beside their
  steps in their transient per-segment arguments: `Segment` gains `target: [L; RAMP_COUNT]`
  (gathered from `ramps[track][index].target`), `SoftClipCoef` gains `drive_target`,
  `output_target`, `mix_target`, and `LaneChunk`/`CrossChunk` carry `(start, step, target)`. A lane
  that is not ramping keeps a zero step, for which `ramp_toward` is the identity (`low <= current <=
  high`). No state record, payload word or sealed size changes.
- **D5. No settled frame pays the clamp.** Sites 7 and 8 add a step on every frame today. Each
  decides once per chunk or segment whether any of its words has a non-zero step and runs the
  clamp only then (as the gate and multiband `RAMPING` parameters already do); the settled path
  keeps today's arithmetic and bits. Sites 4-6 and 9 already run their ramp code only while a
  ramp is in flight (or the #144 stationary hoist skips it).
- **D6. The restore walk is the same law.** `ramp_path_inside`'s walk (`:310-317`) iterates
  `ramp_toward` as `next_value` now does, so its doc ("iterated exactly as `next_value` iterates
  them") stays true. By D2 the walk then accepts exactly when `current` and `target` are inside, so
  the loop is deleted and the function keeps its `remaining`, finite-step, endpoint and settled-step
  clauses; its doc cites D2. The 64-ulp slacks at the callers are unchanged (non-goal). The delay's
  two `"path past the domain"` rows, the limiter's `"moving limit walked past its bounds by its
  step"` row, the compressor's `a_step_that_leaves_the_domain_before_the_snap_is_refused` (its
  accepting half, a path inside `[0, 1]` restoring, goes with it) and its `"a moving path below
  zero"` row now describe an admitted payload whose render stays inside its endpoints; they are
  deleted (superseded), with their doc lines, and any other effect test row whose refusal rested on
  a finite step walking past its target is listed in the PR and deleted.
- **D8. Root decision (2026-10-05): this slice owns #1301's consequences.** Stream J's #1301 lands
  before this slice, so this slice edits on top of it. It (a) deletes the `ramp_path_inside` walk
  and the superseded refusal rows with their tests (D6); (b) deletes the soft clip tests that
  assert the old law's overshoot (Context): `an_overshooting_ramp_restores_and_continues`, its four
  callers and `a_drive_overshoot_retargeted_inward_restores_and_continues`, and in
  `a_restored_near_edge_ramp_continues_bit_for_bit` the `crossed` counter and its every-edge
  assertion (the bit-for-bit continuation stays); (c) re-measures #1301's gate-1 mutations on the
  clamped law, per pull request and full walk exactly as #1301 runs them, and records each count
  here, naming any catch that falls to zero; (d) adds an amendment note to the #1301 spec carrying
  those counts, beside the note root filed on 2026-10-05 (if #1301's spec has already left
  `.github/ISSUE_SPECS/`, the note goes in a comment on GitHub #1301 instead). It does not edit the
  probe, its six caller tests or `crates/delay/tests/MUTATIONS.md`; a lost catch is reported to
  root, and the restore-slack removal that gives the probe a catch again is #1411 (*Remove the
  64-ulp restore slack once every effect ramp is clamped*), which ships in the same pull request
  (D9).
- **D9. Root confirmation (2026-10-05).** The decision-15 root coordinator, under the same
  delegation, confirmed two points. (a) The four extra tests assert superseded behaviour: the
  limiter's `"moving limit walked past its bounds by its step"` row
  (`crates/true-peak-limiter/src/lib.rs:~7623`), the compressor's two payload tests and rows
  (`crates/compressor/tests/payload.rs:396`, `a_step_that_leaves_the_domain_before_the_snap_is_refused`,
  and `:450`, `"a moving path below zero"`), and soft clip's overshoot tests
  (`crates/soft-clip/tests/state_roundtrip.rs` and `crates/soft-clip/tests/randomized.rs`, D8(b)).
  This slice rewrites or deletes them in the same pull request, as `AGENTS.md` requires (a change
  that supersedes a test deletes it in the same PR). They stay in D6 and D8(b) and in the
  authorized paths. (b) Main must never carry a window in which #1301's six probe tests catch
  nothing, so this slice and #1411 ship in one Stream G pull request, this slice's commits first;
  neither merges alone. This slice's record of the lost catches (D8(c), gate 5) is PR evidence
  only; #1411's gate 3 is the gate that makes each of the six probes red again on its mutant.
- **D7. Bits move only where the old word passed its target**, from that frame until the snap.
  The PR carries one-time before/after evidence over every test and pinned artifact that renders an
  effect ramp: each moved case, its first moved frame, its largest change, and confirmation that on
  every moved frame the old word was strictly beyond its target. Re-pin only the tables that move,
  one at a time, from the scalar-oracle digests `g5_native_corpus` prints, naming each case and the
  reason ("D11 endpoint clamp: the ramp word holds its target instead of passing it") in the commit
  message. `E9_DIGESTS`, `M3_DIGESTS` and `BUILTINS_DIGESTS` must not move here. Any other moved
  pin, or any moved frame where the old word was in range, stops the slice. Accepted by D0.

## DSP evidence (AGENTS.md)

- **Equations:** `w[k+1] = snap_k ? T : clamp_strict(w[k] + s, min(w[k], T), max(w[k], T))`,
  `s = (T - w0) / 64` once per event; `snap_k` as each site decides it today.
- **Coefficient and update rules:** D1-D5. Designed words (compressor rate coefficients, delay
  damping `g`, limiter coefficients) are clamped between their designed endpoints; no design
  function changes.
- **Numerical limits:** without the clamp, up to about 30 ulps of the target past it at `n = 64`
  (probe); with it, inside `[min(w0, T), max(w0, T)]` exactly, by D2 (IEEE 754-2019 §4.3; N. J.
  Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., §2.1-2.2). The snap frame is
  unchanged, so a ramp's length is unchanged.
- **Latency and tail:** unchanged for every effect.
- **Units and smoothing:** each parameter's own unit, 64-sample linear smoothing as today.
- **Denormal/NaN:** finite targets and steps on the render path; the clamp creates no subnormal and
  hides no NaN (D3).
- **Fixtures and objective tests:** gates 1-4. **Benchmarks:** descriptive only; four lane
  operations per ramped word per frame while a ramp is in flight, none on settled frames (D5), so no
  row of `docs/rulings/effect-floor-accounting.md` moves (every standing fixture is settled).
  **Listening:** none run; the change is at most about 30 ulps near the end of a ramp.

## Deliverables

1. The re-export and the nine sites (D1, D4, D5); each site's frozen-order doc states the clamp.
2. `ramp_path_inside`'s walk deleted per D6; the delay, limiter and compressor refusal rows and the
   soft clip overshoot tests deleted per D6 and D8(b).
3. Gates 1-3 as tests; every oracle in Context updated to the clamped law.
4. Re-pins per D7, with the evidence in this spec.
5. `docs/EFFECT_CONTRACT_V1.md:133-147` states the clamped law.
6. #1301's mutation counts re-measured and recorded here, and the #1301 amendment note (D8(c-d)).

## Authorized paths

- `crates/lane/src/kernels.rs` (`ramp_block`, its doc, the `ramp_toward` re-export)
- `crates/effect-runtime/src/ramp.rs`, `crates/effect-runtime/src/state_payload.rs`
  (`ramp_path_inside` and its doc), `crates/effect-runtime/tests/ramp_endpoint.rs` (new), the
  effect-runtime tests named in Context
- `crates/effect-contract/src/lib.rs` (the `ParameterSmoother` `Linear` arm and its doc only)
- The render sites and their argument structs: `crates/compressor/src/kernel.rs`,
  `crates/gate-expander/src/kernel.rs`, `crates/multiband-compressor/src/lib.rs`,
  `crates/delay/src/lib.rs`, `crates/soft-clip/src/kernel.rs`, `crates/soft-clip/src/lib.rs`,
  `crates/true-peak-limiter/src/lib.rs`; in those files also the unit tests named in Context and
  D6's refusal rows
- `crates/compressor/tests/payload.rs`: D6's two deletions only
- `crates/soft-clip/tests/state_roundtrip.rs` and `crates/soft-clip/tests/randomized.rs`: D8(b)'s
  deletions only
- `.github/ISSUE_SPECS/1301-make-the-shared-edge-ramp-restore-probe-cheap-enough-for-every-pull-request.md`:
  D8(d)'s amendment note only (or a comment on GitHub #1301)
- One new `tests/ramp_endpoint.rs` in each of `compressor`, `gate-expander`,
  `multiband-compressor`, `delay`, `soft-clip`, `transient-shaper`, `true-peak-limiter`;
  `crates/soft-clip/tests/ramp_law.rs`, `crates/transient-shaper/tests/partition.rs`
- Pin data only: the digest tables of D7 and their `.in` files,
  `tools/wasm-gate-corpus/src/lane_digests.in`
- `docs/EFFECT_CONTRACT_V1.md` (the smoothing paragraph), this spec
- Root-authorized in attempt 2 (the #1411 verifier's MINOR 1), doc comments only: the
  `the_effects_own_edge_ramp_snapshots_restore` docs in
  `crates/{delay,compressor,gate-expander,multiband-compressor,true-peak-limiter,transient-shaper}/tests/randomized.rs`,
  and the `EffectDifferential::edge_ramp_restore_violations` doc in
  `crates/conformance/src/randomized.rs`

`crates/lane`, `crates/effect-runtime` and the effects' parameter code are stream G's column; the
`ParameterSmoother` arm (stream J's `crates/effect-contract`), the payload refusal rows and soft
clip overshoot tests (stream A's payload code), the #1301 note and the corpus pins are named
exceptions recorded in
`docs/handoffs/decision-15-2026-10-05/STREAMS.md`.

## Non-goals

- The builtins (#1408), the SVF coefficient word ramps of the EQ and the input filter (#1407), the
  delay tap crossfade, `OnePole99`, the indexed route ramp; any window, countdown, snap or design
  function; the 64-ulp restore slacks (with D2 they are unused headroom; #1411 removes them in the
  same Stream G pull request, D9).
- Rejected alternatives:
  - Prove no overshoot per parameter: the probe finds reachable overshoots on every domain edge.
  - Clamp to the parameter's domain instead of `[start, target]`: still renders values the user
    never set inside the domain, needs the domain in every kernel, and is a second law.
  - Keep `ramp_path_inside` walking the unclamped update: a clamped ramp held at its target with
    samples remaining walks past it, so the validator could refuse the engine's own snapshot.
  - Clamp only at the domain edges' parameters: not one shape for every word.

## Hazards

- The V8 spill gate and the worklet's kernel roster see the target words (D4) and the clamp in the
  effect kernels, which compile into the browser module; the compressor's `advance_ramps` stays
  `#[inline(never)]` (`kernel.rs:192-197`). If a gate turns red, report it with the build evidence;
  do not weaken a gate.
- D5's split adds a kernel shape in sites 7-8; it must not change the settled path's bits or cost.
- #1301's recorded mutation counts (`.github/ISSUE_SPECS/1301-*.md` gate 1: delay M18 24 refusals,
  gate strict current 32, limiter 4-ulp budget 4) were measured on the old law; after this slice
  engine snapshots stay inside their endpoints, so those mutations may refuse nothing. This slice
  re-measures and records them (D8); it does not weaken or rewrite the probe to restore a catch.
- Hot files: `effect_runtime` ramps and the effect ramp bodies are shared with stream A's carry
  slices (#1279, #1280, #1282) and stream G's #1336, #1338 and #1370, which add `LinearRamp`
  users; `crates/effect-contract/src/lib.rs` is J #1330's and A #1362's. This slice lands first
  (STREAMS.md merge order); any of those that landed earlier gets its new ramp bodies clamped here.
- Size: if the slice cannot close in half a day, the coordinator splits it before implementation
  into the shared law (sites 1-3, D6) and the effect bodies (sites 4-9).

## Objective gates

1. **Law** (`crates/effect-runtime/tests/ramp_endpoint.rs`): 100,000 seeded `(start, target, n)`,
   half with `target` from the probe's edge values and `start` within `1..=4096` ulps of it, half
   with magnitudes in `[2^-20, 2^7]`, both signs, `n` in `1..=4096`: `LinearRamp::next_value`,
   `ParameterSmoother` (`Linear`) and `ramp_block` (every width the build has, as `g4_flush.rs`
   does, partitioned at random through `advance_block`) produce identical words, every word lies in
   `[min(start, target), max(start, target)]`, and the words are bit-identical to the unclamped law
   up to its first frame outside that interval. A finite step of either sign and any magnitude on a
   restored ramp stays inside (D2); `ramp_path_inside` agrees with walking it.
2. **Effect reproducers** (each crate's `tests/ramp_endpoint.rs`): for every ramped word of sites
   4-9 (and the transient shaper through site 2), one in-domain move whose old-law word passes its
   target, found by the probe's scan over the parameter's domain edges (designed words: over
   parameter pairs whose designed words are 33-4096 ulps apart), recorded here. Render the window in
   one-frame blocks and, after each, read the ramp's `current` from the effect's state snapshot:
   it lies in `[min(start, target), max(start, target)]`. Render the same window as one block: the
   output and final snapshot are bit-identical to the one-frame run. The compressor runs with and
   without a connected sidechain (sites 2 and 4). Red on revert of each site. If a word's search
   finds no overshooting move, record the bounds here; gate 1 covers its law.
3. **Twin and oracles:** `contract_ramp_identity.rs` passes unchanged; every oracle in Context is
   bit-identical to the clamped law; every other existing test passes unchanged except D6's and
   D8(b)'s deletions and those D7's evidence names.
4. **Re-pins per D7 only.**
5. **#1301 re-measured (PR evidence, not committed).** On the merged tree with #1301, apply each
   of #1301's gate-1 mutations (delay M18, gate strict current, limiter 4-ulp budget), run that
   crate's `the_effects_own_edge_ramp_snapshots_restore` per pull request (no variable set) and
   full (`MISO_ENGINE_RANDOMIZED_SCALE=1`), and record each "its own snapshot is refused" count,
   whether the two lists are byte-identical, and green after revert, in this spec and the #1301
   note (D8). This record is PR evidence only, not a merge gate: a count that falls to zero is
   expected, and #1411's gate 3, in the same pull request, restores a catch for each probe (D9).
6. Commands:
   - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p effect-contract -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   - `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml`
   - `bash scripts/run-wasm-gates.sh` (native and simd128 legs, V8 spill gate)
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - the worklet chain, as in `qualification.yml`:
     `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`,
     `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`,
     `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-cross-targets.sh`, `bash scripts/check-lane-policy.sh`,
     `bash scripts/check-effect-runtime-policy.sh`, `bash scripts/test-effect-runtime-policy.sh .`,
     `bash scripts/check-effect-contract.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a `LinearRamp`, `ParameterSmoother` or `ramp_block` left on the old law, or the three
  disagreeing, is red on the seeded edge moves; no test bounds an effect ramp word today.
- Gate 2: any one of sites 4-9 left on the old law (or given the wrong target lane by D4) passes its
  target on its edge move; the partition half catches a site clamped in one block shape only.

## Dependencies

- *Keep every trim, fader and matrix ramp inside its endpoints* (#1408), for `ramp_toward`, its
  law test and the `ReferenceLinearRamp` twin, and for moving the trim oracle off
  `ParameterSmoother` (its D4) before this slice changes it
- *Make the shared edge-ramp restore probe cheap enough for every pull request* (#1301, stream J),
  which lands first: this slice re-measures its mutation counts on the probe as #1301 leaves it and
  amends its spec (D8)
- *Remove the 64-ulp restore slack once every effect ramp is clamped* (#1411, stream G), which
  ships in the same pull request, after this slice's commits (D9)

## Attempt record

### Attempt 1 (implementer, 2026-10-05)

**Change (D1-D6).** One body: `lane::kernels::ramp_toward` (#1408's, re-exported from
`kernels::builtins`) replaces `current + step` at every site; each site keeps its own snap.

- Site 1, `lane::kernels::ramp_block`: `g = ramp_toward(g, step, target)` (frozen order step 5).
- Site 2, `LinearRamp::next_value` and `advance_block`'s first-frame word
  (`crates/effect-runtime/src/ramp.rs`); module doc states the clamp.
- Site 3, `ParameterSmoother`'s `Linear` arm (`crates/effect-contract/src/lib.rs`), at `L = f32`
  through `lane::kernels::ramp_toward` (lane is already a dependency); its D11 doc states the clamp.
- Site 4, compressor `RampVec::advance_where`; site 5, the gate's `RAMPING` prologue; site 9, the
  limiter's `RampLanes::advance`. These already run only while a ramp is in flight.
- Site 6, multiband `run_segment`: `Segment` gains `target: [L; RAMP_COUNT]`, gathered in
  `Side::segment` (D4). The flat-path identity docs now name `ramp_toward`.
- Site 7, delay: `LaneChunk`/`CrossChunk` carry `(start, step, target)`; `delay_chunk` is
  `delay_chunk::<RAMPING>`, and `process_chunk` picks `true` once per chunk when any of the seven
  steps is non-zero (`LaneChunk::ramping`, D5). The settled shape keeps the plain additions.
- Site 8, soft clip: `SoftClipCoef` gains `drive_target`, `output_target`, `mix_target`
  (`process` fills them with the new `target_vector`). `soft_clip_block` decides once per block,
  before its frame loop, with `mask_any` over the three step vectors, whether to take the clamped
  update or today's additions (D5). The choice is a loop-invariant branch, not two copies of the
  body: a first version with two const-generic copies raised soft clip's iOS
  `memset_pattern16` count from 22 to 37 in `check-cross-targets.sh` (#1018; the stored splat
  constants double with the body). The ceiling was not touched; with the branch the count is 22
  again and the check passes. The delay's two-copy `delay_chunk::<RAMPING>` moved no count.
- D6: `ramp_path_inside` keeps its `remaining`, finite-step, endpoint and settled-step clauses and
  loses the walk; its doc gives D2. `ramp_path_within` and the 64-ulp slacks are unchanged (#1411).
- `docs/EFFECT_CONTRACT_V1.md` states the clamped law.

Edits outside the listed paths, each forced by D4 or by accuracy: the two other `SoftClipCoef`
literals, `crates/soft-clip/src/corpus.rs` (case 4's ramps run for the whole case, so each targets
the value 1024 steps from its start, twice the case length, and the clamp never acts:
`SOFT_CLIP_DIGESTS` does not move) and `crates/soft-clip/tests/polyphase_identity.rs` (zero steps,
targets the case's values); in `crates/lane/src/kernels.rs` the `IndexedRamp` doc's one-line
statement of D11; in `state_payload.rs` two words of `ramp_path_within`'s doc that named the
deleted walk.

**Deleted (D6, D8(b), D9(a)).**
- Delay `a_carried_ramp_is_refused_unless_its_whole_path_is_valid`: the two `"path past the
  domain"` rows and their doc clause (the test keeps its five other rows; M19 still names it).
- Limiter restore corruptions: the `"moving limit walked past its bounds by its step"` row.
- Compressor `a_step_that_leaves_the_domain_before_the_snap_is_refused` (whole test) and the
  `"a moving path below zero"` row of `a_coefficient_below_zero_or_above_its_design_is_refused`.
- Soft clip `state_roundtrip.rs`: `an_overshooting_ramp_restores_and_continues`, its four callers,
  `a_drive_overshoot_retargeted_inward_restores_and_continues` and the `decibels_near` helper.
- Soft clip `randomized.rs`: the `crossed` counter, its every-edge assertion and `converted`;
  `near_edge_case` returns `()`; the bit-for-bit continuation is unchanged. The module doc's
  overshoot paragraph is rewritten.
- No other effect test row rested on a finite step walking past its target.

**Oracle updated (gate 3).** `crates/effect-runtime/tests/partition.rs`'s makeup ramp and the
delay unit test that iterates `segment.step` by hand now use `ramp_toward`.
`contract_ramp_identity.rs` passes unchanged. The other oracles in Context pass unchanged (their
ramps do not overshoot, or they drive `LinearRamp` itself).

**Gate 1** (`crates/effect-runtime/tests/ramp_endpoint.rs`, three tests).
`every_statement_of_the_law_stays_inside_its_endpoints_and_agrees`: seeded `(start, target, n)`,
100,000 in `--release` (4,000 in debug, a prefix of the same sequence, as #1408's law test),
half with an edge target from the probe (1, 0.95, -0.95, 0.995, 20, -80, 96, -3, 0.5) and a start
1..=4096 ulps from it on either side, half with both ends of magnitude `[2^-20, 2^7]` and either
sign; `n` is 64 for half and uniform in `1..=4096` for the rest. `LinearRamp::next_value`,
`ParameterSmoother` (`Linear`) and `ramp_block` at every width (`lane::each_lane!`, random
partitions of 1..=97 frames through `advance_block`) agree word for word, every word is inside
`[min, max]`, the words equal the unclamped law's up to its first out-of-interval frame, the last
is the target, and both halves reach the clamp. `the_ratio_floor_example_holds_its_target` pins the
probe's ratio example. `a_restored_ramp_with_any_finite_step_stays_inside_and_the_validator_agrees`:
200,000 restored ramps (20,000 debug) with a step drawn independently of the endpoints (any finite
bit pattern, scaled engine steps, `0.0`, subnormals) and `remaining` in `0..=65`: every walked word
stays inside, and `ramp_path_inside` equals a walking reference on four bound pairs each. Timing:
3.5 s debug, 6.0 s release.

**Gate 2** (each crate's `tests/ramp_endpoint.rs`). One harness (copied per crate): prepare with
the parameter at `start`, send `Point`s to `target` at sample 0 on every channel, render 72 frames
in one-frame blocks; after each, every ramp word of the payload must lie between its value at rest
and its snapshot target; the moved word's own unclamped walk (rest word, snapshot step) must leave
that interval (so the move reaches the clamp); and a second instance rendering the window as one
block must give the same ramp words, output and snapshot. The compressor runs unconnected (site 4)
and connected (site 2). A second test per banking effect (all but the delay) runs the move on the
last lane of a native-width bank whose other lanes rest at the defaults, and requires the moving
lane's output and ramp words to be bit-identical to a scalar instance after every frame (D4's
target lane). Moves found by the scan (start bits, target, first out-of-interval step of the
unclamped walk):

| Effect | Word | Start | Target | First out |
| --- | --- | --- | --- | --- |
| compressor | threshold | `0xc29fffdf` | -80 | 34 |
| compressor | ratio | `0x3f800021` | 1 | 34 |
| compressor | knee | `0x41bfffdf` | 24 | 34 |
| compressor | attack | `0x3dccccee` | 0.1 | 34 |
| compressor | release | `0x459c3fdf` | 5000 | 34 |
| compressor | makeup | `0x41bfffdf` | 24 | 34 |
| compressor | mix | `0x3f7fffa0` | 1 | 49 |
| compressor | attack coefficient | attack `0x3dccccf4` | 0.1 | 34 |
| compressor | release coefficient | release `0x40a00027` | 5 | 34 |
| gate | threshold, ratio, range, hysteresis | `0xc29fffdf`, `0x3f800021`, `0x42bfffdf`, `0x41bfffdf` | -80, 1, 96, 24 | 34 |
| multiband | low and high threshold, ratio, attack, release, makeup | `0xc29fffdf`, `0x3f800021`, `0x3dccccee`, `0x459c3fdf`, `0x41bfffdf` | -80, 1, 0.1, 5000, 24 | 34 |
| delay | feedback | `0x3f733312` | 0.95 | 34 |
| delay | damping coefficient `g` | damping `0x3e800021` | 0.25 | 34 |
| delay | mix, cross feedback | `0x3f7fffa0` | 1 | 49 |
| soft clip | drive gain | drive `0x420ffffc` | 36 dB | 34 |
| soft clip | output gain | output `0x41bffff7` | 24 dB | 34 |
| soft clip | mix | `0x3f7fffa0` | 1 | 49 |
| transient | attack, sustain, mix | `0x3f7fffa0`, `0xbf7fffa0`, `0x3f7fffa0` | 1, -1, 1 | 49 |
| limiter | limit coefficient | ceiling `0xc1bffff6` | -24 dB | 34 |
| limiter | release coefficient | release `0x41200027` | 10 | 34 |

Scan bounds where an edge had no move: delay damping `g` has none within 4096 word ulps of either
edge (toward 0 the designed word jumps past 4096 ulps at once; toward 0.995 the unclamped walks stay
inside), so its move is an interior pair from a scan over 17 targets (26 hits); the limiter's 0 dB
ceiling edge designs the same limit word for every start the scan reached (distance 0), so its move
is at -24 dB. Every other parameter had moves at both edges; the table uses one per word, avoiding
subnormal starts.

Deviation, for the verifier (corrected in attempt 2): for the multiband compressor's high ratio
and high attack moves only, the partition half compares the ramp words only, not the output and
whole snapshot. That effect refreshes its ratio, attack and release coefficients once per segment
by its frozen design (`tests/identity.rs`, `partition_control_trajectory_preserves_ramp_positions`),
and on those two moves the refresh makes the one-frame and one-block outputs differ (left output)
whatever the ramp law. Every other move, low ratio, low attack, low release and high release
included, compares output and whole snapshot.

**Mutation evidence** (each applied alone, the named tests run, reverted; all green after revert):

| Mutant | Red test |
| --- | --- |
| site 1 `ramp_block` back to `g.add(step)` | gate 1 (`ramp_block at f32 differs from LinearRamp`) |
| site 2 `next_value` back to `current += step` | gate 1 (three tests); compressor connected (threshold, frame 33, `0xc2a00001`); transient (attack, frame 48); delay (feedback, frame 33) |
| site 2 `advance_block` first word back to `current + step` | gate 1; delay (one-block output) |
| site 3 smoother back to `current + step` | gate 1 (`ParameterSmoother differs`) |
| `ramp_path_inside` without its target clause | gate 1 restore test |
| site 4 `advance_where` back to `add` | compressor unconnected (threshold, frame 33) |
| site 4 gather: target from lane `W - 1 - lane` | compressor bank test |
| site 5 gate prologue back to `add` | gate (threshold, frame 33) |
| site 6 `run_segment` back to `add` | multiband (low threshold, frame 33) |
| site 6 `Side::segment` target from lane `W - 1 - track` | multiband bank test |
| site 7 `ramp_word` at `RAMPING` back to `value + step` | delay (one-block output) |
| site 7 D5 choice inverted | delay (one-block output) |
| site 8 drive back to `add` | soft clip (drive, frame 33, `0x427c620b`) |
| site 8 D5 choice inverted (`ramping` negated) | soft clip (drive, frame 33) |
| site 8 `target_vector` from lane `W - 1 - lane` | soft clip bank test |
| site 9 limiter back to `add` | limiter (limit coefficient, frame 33, `0x3d6655c2`) |

The scalar instance cannot see a wrong target lane (`W = 1`), which is why the bank half exists:
the multiband wrong-lane mutant was green before it was added.

**D7, bits moved.** No pinned table moved: `g5_native_corpus` (`D1_DIGESTS`, `C1_DIGESTS`,
`GATE_DIGESTS`, multiband `DIGESTS`, `G5_DIGESTS`, `SOFT_CLIP_DIGESTS`, transient
`CROSS_TARGET_DIGESTS`, `D90_DIGESTS`, `LANE_DIGESTS`, and `E9_DIGESTS`, `M3_DIGESTS`,
`BUILTINS_DIGESTS`) passes unchanged, so there is no re-pin. Every existing bit-exact test passes
unchanged apart from the deletions above. The only moved words are the ones the new tests force:
each is a frame on which the unclamped word was strictly past its target (gate 1 asserts bit
identity before the first such frame; gate 2 asserts the unclamped walk leaves the interval).

**Gate 5, #1301 re-measured (PR evidence).** Delay M18, gate strict current and limiter 4-ulp
budget, each applied as #1301 states them, per pull request and full: 0 and 0 refusals each, lists
identical (empty), green after revert; recorded in the #1301 amendment note. All three catches fall
to zero, as expected; #1411's gate 3 restores a catch per probe in the same pull request.

**Gates.**
- `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p effect-contract -p delay
  -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper
  -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features
  math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`: pass (soft clip
  rerun in full after the branch change: pass).
- `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`: pass, no pin
  moved.
- `test-debug-a` workspace command: pass.
- `bash scripts/run-wasm-gates.sh` (native, simd128, V8 spill gate): pass.
- `conformance_fixtures -- --check`: pass.
- Worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
  `test-web-audioworklet.sh`): pass.
- `check-cross-targets.sh`: pass (memset ceilings unchanged; counts unchanged, see site 8).
- `check-lane-policy.sh`, `check-effect-runtime-policy.sh`, `test-effect-runtime-policy.sh .`,
  `check-effect-contract.sh`, `check-realtime-policy.sh`, `check-workspace-policy.sh`: pass.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`:
  pass.

**Open items.** The multiband partition deviation above. The 64-ulp restore slacks are now unused
headroom and #1301's three catches are zero; both are #1411's, which ships in the same pull
request. The #1411 spec already covers soft clip's `ramp_current_valid`/`ulp_at` (its site 8 and
D3), so it was not edited.

### Attempt 2 (implementer, 2026-10-06)

Tests and docs only. No production code changed: the `src` edits below are doc comments (and one
doc moved above its `#[derive]`); `git diff -U0 -- crates/*/src` shows no non-comment line.

**MAJOR 1, the delay's gate-2 test now reaches site 7 for every word.** At the 250 ms default
delay time the taps were silent for the whole 72-frame window, so feedback, damping `g` and cross
feedback never reached the output or the ring. `values_with` now also sets the delay time
(parameter 0) to its 1 ms minimum (`DELAY_TIME_MS`, 48 samples at 48 kHz), so the taps carry
signal from frame 48, inside the 64-sample ramp. The test is green unmutated. Per-word site-7
mutants, each applied alone to `delay_chunk` (`x = ramp_word::<RAMPING>(x, w)` back to
`x = x + w.1`), `cargo test -p delay --test ramp_endpoint`, then reverted:

| Mutant (site 7 word) | Result |
| --- | --- |
| left damping `g` (`gain_left`) | red: damping coefficient, the final snapshot depends on the partition |
| left feedback (`feedback_left`) | red: feedback, the final snapshot depends on the partition |
| right damping `g` (`gain_right`) | red: damping coefficient, the final snapshot depends on the partition |
| right feedback (`feedback_right`) | red: feedback, the final snapshot depends on the partition |
| cross position (`position`) | red: cross feedback, the final snapshot depends on the partition |
| all reverted | green |

Each is red at the partition half's final-snapshot comparison, which covers the ring; with the
250 ms default all five were green in every delay suite (verdict MAJOR 1).

**MINOR 1, multiband `whole`.** `whole: true` is restored on low ratio, low attack, low release and
high release; the four pass the full output-and-snapshot comparison. High ratio and high attack
keep `whole: false`; set to `true` each is red on `left output`. The deviation sentence above and
the harness `Move::whole` doc (all seven copies) and the multiband module doc now name those two
moves only.

**#1411 MINOR 1 (root-authorized path addition, recorded under Authorized paths).** The seven
probe docs no longer say that the effect's iterated `current + step` rounds past the edge or that
the fix for a red probe is to admit it. Each caller doc (`crates/{delay,compressor,gate-expander,
multiband-compressor,true-peak-limiter,transient-shaper}/tests/randomized.rs`) now says: since
#1409 every ramp word stays between its start and its target, so the strict restore (#1411) admits
each snapshot; the probe is red when a render site leaves a ramp word outside its endpoints at
render (a missing or reverted #1409 clamp) or when the restore refuses a valid in-range snapshot;
the fix is that clamp or that validation, never a restore slack. The conformance probe doc
(`EffectDifferential::edge_ramp_restore_violations`) says the same, and states that the probe uses
the unclamped walk only to pick the moves and the `k - 2 ..= k` positions.

**NITs.**
- Rewrapped to 100 columns: the `IndexedRamp` doc (`crates/lane/src/kernels.rs`), the multiband
  `store_segment` and flat-path (`run_segment`) docs, and the delay unit test doc
  (`a_carried_ramp_is_refused_unless_its_whole_path_is_valid`); the two early line ends (compressor
  `advance_where`, multiband `flat_path_is_identity`) are joined.
- The multiband `Segment` doc now sits above its `#[derive(Clone, Copy)]`.
- The harness `request` doc (seven copies) now says the 48 kHz quality row (`qualities[1]`, the
  rate the moves were scanned at) and the dual-mono link mode, and the code asserts
  `quality.sample_rate == 48_000`.
- **Soft clip settled cost (D5 hazard), one descriptive measurement, not tuned.** A scratch,
  uncommitted release bench: a native-width bank (AVX2, 8 lanes) of default-valued soft clips,
  settled, 40,000 blocks of 128 frames, one warmup and two measured rounds, one invocation per tree.
  Parent of attempt 1 (`c58c4e55f`, plain additions): 80.88 / 81.38 / 80.87 ns per bank frame.
  This attempt's tree (the `if ramping` branch): 78.27 / 78.22 / 78.19 ns. No cost is visible at
  this resolution; the difference is within run-to-run and tree-to-tree noise (the two trees also
  differ by #1411 and #1328's follow-up), so it is no speedup claim.

**Gates.**
- The spec's debug command (gate 6, first line): pass (169 result lines, 0 failed), including the
  six probes `the_effects_own_edge_ramp_snapshots_restore`.
- `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`: pass
  (`g5_native_digests_match_pins` ok, no pin moved).
- Release run of the touched targets: `--test ramp_endpoint` for the seven effects and
  `effect-runtime` (gate 1 at 100,000, 6.0 s): pass; the six probes in `--release`: pass.
- `conformance_fixtures -- --check`: pass.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`:
  pass.
- No browser-compiled code changed (doc comments only in `src`), so the worklet chain was not
  rerun.

**Open items.**
- Gate 1's 100,000-ramp size runs in no CI job: `effect-runtime` is in no release job, so per-PR
  CI runs only the 4,000-ramp debug prefix (which checks its own reach). The full size passed by
  hand (`cargo test --release -p effect-runtime --test ramp_endpoint`, 6.0 s, verdict). Adding a
  release run is a workflow change outside this issue's paths; root decides.
- The verdict's MINOR 2 (soft clip line clause) is #1411's and its verifier confirmed it covered.
- Verdict NIT 5's shared harness (one helper instead of seven copies) and NIT 7 (a wasm effect-ramp
  corpus case) stay optional follow-ups.
