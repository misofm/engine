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
- *Carry the link record from the edit to the lane* (#1371).
- *Carry live-controlled effect lanes across a plan swap* (#1280).
- *Carry per-node effect instances across a plan swap* (#1282).

The link-record slice (#1371) brings *Lower the link mode to per-lane state in the linked effects'
banks* (#1368) and *Ramp a lane's detector link between modes* (#1370) with it.

**Decision 17 amendment (2026-10-13).** An insert's link mode is a control row, `LINK_MODE_CONTROL` (`0x8000_0000`) in its insert section, written by `SET_CONTROL` (#1549, #1553; `SetEffectLinkMode` is retired by #1561). For `miso.multiband-compressor` the row is `RebuildOnly(RuledLiveNotBuilt { issue: 1367 })`. Last step of this issue: flip that row's class to `Live` and give the classifier's effect family the link record; no protocol or host code.

## 1367a record (D1 + D2, attempt 1, 2026-10-24)

- **Slice.** 1367a is D1 and D2 only (scope note `1369-chain`): the descriptor's `lane_link = true`,
  per-lane `linked`/`averaged` masks (`shim::LinkLanes`) built at bind from each active member's
  mode (a padded lane takes the first member's), and a `LINK_MIXED` select arm in `link_levels`
  that `render_body` takes when the lanes' modes differ. The uniform arms are unchanged and do not
  read the masks. 1367b (D3-D5 ramp, payload, the row flip from `RuledLiveNotBuilt{1367}`) follows
  #1371. Why now: #1893 made the multiband a console slot, so #1369's per-strip override would
  have split its bank (decision 12).
- **Collapse.** The collapsed body (#1871) reads the near word twice, so a collapsed lane's link
  partner is its own `L == R` pair at its own mode; the mono-collapse probe now also runs a bank
  whose lanes mix the three modes.
- **Lane swap.** The multiband declares no prepared lane defaults (`lane_default_bytes` is the
  default `None`), so `swap_bank_lanes` refuses it and no link word has to travel. The template's
  `exchange_lane_defaults` link word therefore does not apply here; a multiband lane swap is its
  own capability, outside this slice. Once 1367b adds a lane swap or a live link, the per-lane
  masks must move with the lane (re-derived from the lane's mode, padded lanes re-taking the first
  active member's), as #1368b's `set_lane_link` does.
- **Tests (extended, none new).** `padding_tests::every_padded_bank_renders_its_members_per_node_bits`
  gives each member its own link mode (random arm drawn, fixture arm rotated, blocks of 128/37/1,
  every width): red when a mixed bank links a DualMono lane, or when bind ignores the members'
  modes. `shim::the_shims_are_width_independent` adds two mixed banks: red on the same link
  mutant. New lib test `padding_keeps_the_members_uniform_link_arm`: a uniform `Maximum` bank
  padded with a `DualMono` request keeps the `LINK_MAXIMUM` arm; red when bind drops the
  `active_mask` guard. `product.rs`'s key-differs decline now uses a differing bypass.
- **Class A.** Committed corpus digests pass. One untimed `bench console` warmup at base
  `c6e295947` and at this branch: all 52 digest fields over 37 records identical.
- **Web roster.** The rows stay pinned to `process_bank` / `process_bank_mono` and pass at their
  0.10 ceilings: dual 3464 / 0 (was 2527 / 0), collapsed 1862 / 0 (was 1389 / 0), the mixed arm's
  body. No row or ceiling changed; the derivation comment's counts are updated (root authorized).
