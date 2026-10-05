# Keep every effect parameter ramp inside its endpoints

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-05 as the successor of *Keep every trim, fader and matrix ramp inside its endpoints*
(#1408), whose D7 left the effect ramps out. Code anchors verified on `codex/d15-stream-g` at
`b800633e2`.

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
  (`"path past the domain"`, `:2773-2778`).
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
  two `"path past the domain"` rows now describe an admitted payload whose render stays inside its
  endpoints; they are deleted (superseded), and any other effect test row whose refusal rested on a
  finite step walking past its target is listed in the PR and deleted.
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
2. `ramp_path_inside` per D6, and the superseded refusal rows deleted.
3. Gates 1-3 as tests; every oracle in Context updated to the clamped law.
4. Re-pins per D7, with the evidence in this spec.
5. `docs/EFFECT_CONTRACT_V1.md:133-147` states the clamped law.

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
- One new `tests/ramp_endpoint.rs` in each of `compressor`, `gate-expander`,
  `multiband-compressor`, `delay`, `soft-clip`, `transient-shaper`, `true-peak-limiter`;
  `crates/soft-clip/tests/ramp_law.rs`, `crates/transient-shaper/tests/partition.rs`
- Pin data only: the digest tables of D7 and their `.in` files,
  `tools/wasm-gate-corpus/src/lane_digests.in`
- `docs/EFFECT_CONTRACT_V1.md` (the smoothing paragraph), this spec

`crates/lane`, `crates/effect-runtime` and the effects' parameter code are stream G's column; the
`ParameterSmoother` arm (stream J's `crates/effect-contract`), the payload refusal rows (stream A's
payload code) and the corpus pins are named exceptions recorded in
`docs/handoffs/decision-15-2026-10-05/STREAMS.md`.

## Non-goals

- The builtins (#1408), the SVF coefficient word ramps of the EQ and the input filter (#1407), the
  delay tap crossfade, `OnePole99`, the indexed route ramp; any window, countdown, snap or design
  function; the 64-ulp restore slacks (with D2 they are unused headroom; removing them is payload
  work for stream A, flagged to root).
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
- #1301's recorded mutation counts (`.github/ISSUE_SPECS/1301-*.md` gate 1: gate strict current 32
  refusals, limiter 4-ulp budget 4) were measured on the old law; after this slice engine snapshots
  stay inside their endpoints, so those mutations may refuse nothing. Root amends #1301; this slice
  does not edit it.
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
   bit-identical to the clamped law; every other existing test passes unchanged except D6's deleted
   rows and those D7's evidence names.
4. **Re-pins per D7 only.**
5. Commands:
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

## Attempt record

None yet.
