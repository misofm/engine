# Make the multiband compressor's link mode live

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-7, D15-13 E4).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A host changes a multiband compressor insert's detector link mode while it plays, and the change
is a live update: no rebuild, both bands' detectors glide from one link law to the other over the
edit's ramp or the session default, and multiband inserts that differ only by link mode share one bank. Decision 14 rule
3 makes a value live unless a reason keeps it prepared; the multiband's link has none (below).

## Context

- The link law is a compile-time `MODE` in `link_levels` (`crates/multiband-compressor/src/shim.rs:47-73`),
  applied to each band's level per frame (`run_segment`, `crates/multiband-compressor/src/lib.rs:876`,
  the two `link_levels::<L, LINK>` calls). Its doc says the mode is fixed at preparation.
- The mode is chosen **once per block** at run time: `render` matches `(instance.link,
  instance.bypass)` (`lib.rs:1022-1052`). So the const-generic arm is a per-block choice that a
  per-lane mode can still make ("every lane one mode": today's arm; mixed: a select arm).
- The link mode is in the bank key (`EffectProgramKey.link_mode`,
  `crates/effect-contract/src/lib.rs:1179`), so mixed-mode multiband inserts never share a bank.
- *Lower the link mode to per-lane state in the linked effects' banks* (#1368) adds the
  descriptor flag `lane_link` and the key lowering; *Ramp a lane's detector link between modes*
  (#1370) defines `retarget_link` and the blend; *Carry the link record from the edit to the lane*
  (#1371) delivers the lane's link cell and the classifier rows for `lane_link` effects. The multiband
  was left out of them because it is not console-eligible.
- **No reason to keep it prepared.** No optimisation needs a constant mode: the arm is already
  chosen per block (above). No glitch is forced: the link feeds each band's level into the smoothed
  gain path (`band_amplitude`, branching smoother), which keeps the output gain continuous, and the
  switch ramps (D15-1). No correctness rule depends on it (the multiband has no ceiling proof).
  Today a change is a rebuild with a D15-9 transition (*Duck-swap a strip whose state cannot
  continue across a plan swap*, #1324).

## Decisions frozen for this slice

- **D1. Flag.** The multiband's descriptor sets `lane_link = true` (#1368 D1). Its key lowers the
  link as #1368 D2 does; its bank factory reads each member's mode into per-lane `linked` and
  `averaged` masks.
- **D2. Arms.** `render` picks today's const arm when every active lane shares one mode, and a
  select arm otherwise; the select arm computes each lane's levels in `link_levels`' frozen order
  for that lane's mode. No lane's bits depend on its bank-mates.
- **D3. Ramp.** `retarget_link` (#1370 D1) on the multiband sets per-lane, per-channel `w_link`
  and `w_avg` ramps (#1370 D2); while either ramps, each band's level is #1370 D3's blend of `own`,
  `max` and `avg`. The ramps advance inside the segment loop like the ten parameter ramps, so the
  segment split (`plan_segment`, `lib.rs:1140`) and `flat_path_is_identity` (`:1178`) cover them,
  and a cut by a neighbour cannot move a lane's bits (the rule #1069 establishes).
- **D4. Payload.** Each channel's payload gains the lane's mode and the two weight ramps, after the
  existing words; restore validates them as #1370 D6 does. `STATE_LAYOUT_VERSION` stays 1.
- **D5. Classifier.** No new code: #1371's rule covers every `lane_link` effect, so a multiband
  insert's link change becomes one link-cell write, with the edit's ramp or the session default
  (#1371 D3). Add the multiband rows to its tests.
- **D6. Bypass.** The prepared-bypass arm is untouched here; *Give the multiband compressor a live
  bypass shunt* (#1340) owns the bypass.

## Effect evidence (AGENTS.md list)

- Equations: #1370 D3 per band. Update rule: linear weights, exact arrival. Stability: a convex
  combination of nonnegative magnitudes; no recursive coefficient changes. Latency 0 and tail
  unchanged. NaN and denormal behaviour unchanged (D8 clamps, the once-per-block D7 check).
- Citations: Giannoulis, Massberg and Reiss, JAES 60(6), 2012.
- Listening: a blinded A/B of a `dual_mono` to `maximum` switch against a step on a wide mix,
  recorded in the PR.

## Deliverables

1. D1-D4 in `crates/multiband-compressor/src/lib.rs` and `shim.rs`.
2. D5's test rows in `crates/host-core/tests/live_delta.rs` (stream B's file: coordinate).
3. Tests per gate.

## Authorized paths

- `crates/multiband-compressor/src/`, `crates/multiband-compressor/tests/`
- `crates/dsp-reference/src/` (the multiband's blend reference, if separate from #1370's)
- `crates/host-core/tests/live_delta.rs` (multiband rows only)

## Non-goals

- Console eligibility, padding, the live crossover (*Make the multiband compressor's crossover
  live*, #1338), the bypass (#1340).

## Objective gates

1. **Mixed bank.** At `Simd4` and `Simd8`, a full bank of multiband inserts with mixed modes binds
   (it declined before), and each lane is bit-identical to its scalar instance at its own mode,
   whole and in chunked blocks of 37 frames.
2. **Uniform banks keep their arm and bits.** Every checked-in digest
   (`crates/multiband-compressor/src/corpus_digests.in` included) is unchanged.
3. **Switch.** Each direction over 64 samples, mid-block, on channels 20 dB apart: both bands'
   level words match the `f64` blend reference within the crate's SIMD/scalar tolerance, and from
   the arrival frame equal the target mode's `link_levels` bit for bit; bank-mates keep their bits.
4. **Payload.** A lane snapshotted mid-switch and restored renders bit-identically to the
   uninterrupted one.
5. **Live end to end.** A committed insert link change on the C ABI classifies to one link-cell
   write and no rebuild.
6. **Realtime.** A switching block makes zero allocations and frees (`tests/no_alloc_render.rs`).
7. Commands:
   - `cargo test --locked --all-targets -p multiband-compressor -p effect-runtime -p dsp-reference --features math/lane,lane/test-support`
   - `cargo test --locked -p host-core -p graph-compiler --features host-core/test-support,graph/test-support`
   - `bash scripts/check-cross-targets.sh`; `scripts/run-aarch64-tests.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
     `cargo fmt --all -- --check`; `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: an arm chosen from one lane's mode renders a neighbour at the wrong law; it turns red.
  Nothing banks mixed multiband modes today.
- Gate 3: weight ramps refreshed at a segment boundary instead of per frame move bits when a
  neighbour cuts the segment; it turns red.
- Gate 5: a multiband left without `lane_link` stays structural; it turns red.

## Dependencies

- *Multiband compressor: a ramp's cut moves a lane's bits in a bank* (#1069).
- *Carry the link record from the edit to the lane* (#1371), which brings #1368 and #1370.
- *Carry live-controlled effect lanes across a plan swap* (#1280) and *Carry per-node effect
  instances across a plan swap* (#1282).
