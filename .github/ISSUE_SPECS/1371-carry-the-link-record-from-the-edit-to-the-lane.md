# Carry the link record from the edit to the lane

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-2, D15-7, D15-9, D15-13 E4).
Slice L3b of *Let a strip override a console slot's link mode* (#1236).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A host changes a strip's console link override, a console slot's default link mode, or an insert's
link mode on the compressor, gate-expander, transient shaper or true-peak limiter, and the change
is a live update: no plan rebuild, the detector glides over the edit's ramp (or the session
default), and the edit is heard from the next block. Any number of link edits between two render
calls is admitted; the last one wins. A swap during the glide carries it. After this slice, decision 15's
D15-9 duck-swap covers a link-mode change only for effects whose link stays prepared.

## Context

- Today records reach an effect through its `EffectControlLane`: `EffectControlRecord`
  (`crates/effect-contract/src/live.rs:54` onward) has `Parameter`, `PreparedTarget`, `Observe`
  and `Bypass`; `stage` (`:341`) applies a `Bypass` to the lane's flag at the block boundary
  (`:398-401`). Banked lanes stage in `LiveControlEffectBankStage::drain` and apply in
  `process_inner` (`crates/rack/src/lib.rs:1257`, `:1294`); per-node lanes in the graph runtime's
  `NodeKind::LiveControlEffect` arms (`crates/graph/src/runtime.rs:1527`, `:3510`).
- *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345) replaces the
  effect lane's queue with latest-target cells on #1312's primitive (#1345 D1: one bypass cell,
  parameter cells, target cells), drained in a canonical order (bypass, parameters, targets, then
  the observation FIFO; #1345 D3). A write never fails, and superseded values are counted by
  `live_values_superseded`. Decision 15, D15-2, forbids typed backpressure for a live value.
- *Ramp a lane's detector link between modes* (#1370) gives the effects
  `retarget_link(track, mode, samples) -> bool`, render-safe.
- The C ABI classifier compares an effect's identity, quality, link mode and sidechain
  structurally (`crates/host-core/src/live_delta.rs:150-156`), so any link change is
  `LiveRebuild::Structure`. Bypass records are emitted per instance ahead of parameter records
  (`:376-395`).
- Ramp lengths: *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan
  changes* (#1054) owns the session default table (its D3 names one key per live row). Its D3
  table maps the detector link glide to `fader_ms`, read as `LiveRamps::link_samples` (its D4).
  *Carry an optional per-edit ramp length on live session edits* (#1394) owns the optional
  per-edit ramp field on a transaction edit (decision 15, D15-1; root ruling R9: absent means the
  session default, an explicit 0 is legal and is a step), `EditRamps` with its `Link` row (#1394
  D3: `SetTrackConsole` on the strip, `SetEffectLinkMode` on the effect, `SetConsole` on the
  console) and `LiveRamps::resolve` (#1394 D5).
- The browser reaches the same classifier when its control plane moves to the shared crate (*Run
  the browser control plane in a Worker and keep the AudioWorklet render-only*, #1332, on *Extract
  the C ABI control plane into a portable crate both hosts call*, #1309). The worklet message
  protocol stays internal (D15-11); no browser command kind is added.

## Decisions frozen for this slice

- **D1. A link cell, not a record.** Each lane of an effect whose descriptor has `lane_link`
  (#1368 D1) gains one latest-target cell on #1312's primitive (#1312 D1: a triple buffer whose
  slots carry their own sequence), holding one word. Bits 0-23
  hold the ramp length in samples; bits 24-31 hold the mode's wire code (`dual_mono` 1, `maximum`
  2, `average` 3, as `enum_link`, `crates/protocol/src/session_wire.rs:237-241`). A ramp fits: the
  session ramp is bounded at 1000 ms (#1054 D1) and the per-edit ramp at the session rate (#1394
  D4), 96 000 samples at 96 kHz, below 2^24. The writer asserts the bound in debug builds; the classifier never produces a larger value.
  No `EffectControlRecord::Link` variant is added.
- **D2. Drain order.** #1345's canonical drain becomes: bypass, link, parameters, targets, then
  the observation FIFO. The stage that owns the processor reads a dirty link cell at the block
  boundary, before processing, and calls `retarget_link(track, mode, samples)`. A `false` return
  increments the lane's existing refusal counter; the classifier makes it unreachable (D4).
  #1312's read applies unchanged (#1312 D1-D2): render reads the newest completed word in one
  pass, never tears and never skips, and when it reads sequence `s` after `p` it adds `s - p - 1`
  superseded link words to `live_values_superseded`.
- **D3. Ramp length.** The word's ramp is `LiveRamps::resolve(link row, edit_ramp)` (#1394 D5),
  where `edit_ramp` is the `EditRamps` entry for the write's `Link` row (#1394 D3, D6): the
  effect's entry for an insert link change, the strip's for a console-entry override, the
  console's for a slot-default change. That is the edit's own ramp when the transaction carries
  one (root ruling R9; an explicit 0 is a step), else `LiveRamps::link_samples`, which #1054 D3
  maps to the session's `fader_ms` key ("detector link glide"). Absent ramps from the browser and the SDK are resolved by *Resolve an absent live
  ramp to the session default on the browser and in the SDK* (#1364); this slice reads the
  resolved field.
- **D4. Classifier.** For an effect with `lane_link`, a resolved link mode that differs between
  the committed and the next model is not structural: the classifier emits one link write for that
  lane. The resolved mode is the entry's override or, for `slot`, the slot's declared mode
  (#1369 D1), so a changed slot default writes one cell per strip whose entry says `slot`; a
  changed insert link writes one for that insert. A mode the effect does not support stays a
  preparation refusal (`effect.link_mode.unsupported`), never a write. For any other effect a link
  change stays structural (D15-9 duck-swap).
- **D5. Admission and the acked-batch question: can an ack ever precede a drop? No.** The link
  cell has no room to check. `commit_live` keeps #1345 D5's order: every fallible check of the
  transaction (domains, preparation refusals, revision checks), then all cell writes, then the
  protocol commit. A link write cannot fail, so nothing acked is dropped; a replaced word is in
  the committed model and its replacement is counted.
- **D6. Carry (D15-7).** A swap carries the lane's link state through the payload (#1370 D6).
  *Carry console effect lanes across a plan swap* (#1279) D1 requires link mode to be equal for a
  carry; for `lane_link` effects this slice amends it to "carry, then retarget". The successor is
  prepared from the committed model. When the carried lane's link target differs from the
  committed resolved mode (an unread write, or a link change in the rebuilding transaction), the
  control plane writes the successor's link cell before it publishes the successor, with the
  committed mode and the ramp of the last word it wrote for that lane (the control plane keeps a
  mirror of each link cell's last word; it is the cell's only writer). The successor's first block
  then glides the carried lane to the committed mode.

## Deliverables

1. D1 and D2 in `crates/effect-contract/src/live.rs` (the cell and its drain),
   `crates/rack/src/lib.rs` and `crates/graph/src/runtime.rs` (the `retarget_link` call).
2. D3-D5 in `crates/host-core/src/live_delta.rs` and the C ABI admission (or their successors in
   the control-plane crate, #1309).
3. D6 in the inventory join (`crates/host-core/src/prepare.rs`) and #1279's spec text if still
   open.
4. Docs: `crates/capi/include/miso_engine_v1.h` "Live edits" and
   `docs/C_ABI_V1_QUALIFICATION.md:393` (link mode leaves the rebuild list for these effects).

## Authorized paths

- `crates/effect-contract/src/live.rs` (the link cell and the drain; streams A, B and E also edit
  this file: rebase on their merges, #1345 first)
- `crates/rack/src/lib.rs`, `crates/graph/src/runtime.rs` (the cell's application only; stream
  A's files: coordinate)
- `crates/host-core/src/live_delta.rs`, `crates/host-core/tests/live_delta.rs`,
  `crates/host-core/src/prepare.rs` (the carry rule only), `crates/control-plane/src/` (if #1309
  has moved the classifier), `crates/capi/src/runtime/control.rs` (the link write in
  `commit_live`), `crates/capi/src/runtime/live_tests.rs`, `crates/capi/include/miso_engine_v1.h`
  (docs only); stream B owns these: coordinate the merge
- `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- A browser command kind; the browser gets this through the shared control plane (#1332).
- The multiband compressor's link (#1367).

## Objective gates

1. **Classifier.** For each of the four effects: an entry override change, a slot-default change
   (three strips on `slot`, one overriding) and an insert link change each classify live, with
   exactly the link writes D4 names, each with D3's ramp (an edit's explicit ramp, an explicit 0,
   and an absent ramp resolved to `LiveRamps::link_samples`), and no write for the
   overriding strip on a slot-default change. The same changes on a `lane_link = false` effect
   stay `LiveRebuild::Structure`.
2. **Live on the C ABI.** A committed override change on a console compressor: no plan
   replacement; the render equals a reference plan whose lane's `retarget_link` is called with the
   same mode and ramp at the same block.
3. **Many link edits, none refused (new capi test).** A paused host (no render between submits)
   sends 40 transactions, each changing one console compressor's link override (cycling
   `maximum`, `average`, `dual_mono`); every one returns `RESULT_OK`. The next render equals a twin
   that made only the last edit, and `LIVE_VALUES_SUPERSEDED` grows by exactly 39. A transaction
   carrying a link change and a parameter value outside its domain is refused whole with the
   domain error; the committed model, the link cell (its sequence) and the running plan are
   unchanged.
4. **Swap mid-glide.** A rebuild that adds a track while a lane's link ramp is in flight: the
   swapped run and a reference that never swapped are bit-identical for the carried lane. A second
   case writes a link edit and swaps before any render: the successor's first block starts the
   glide to the committed mode.
5. **Realtime.** The block that applies a link cell makes zero allocations and frees
   (`bench_support::alloc` thread counters, statics warmed).
6. Commands:
   - `cargo test --locked -p effect-contract -p rack -p graph -p host-core -p capi --features graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit && ./target/release/audit capi`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
     `cargo fmt --all -- --check`; `bash scripts/check-workspace-policy.sh`;
     `bash scripts/check-realtime-policy.sh`

## Test value

- Gate 1: a classifier that resolves `slot` against the old slot default, emits for an
  overriding strip, or drops the edit's own ramp for the session default applies the wrong mode or
  glide on one strip; it turns red.
- Gate 2: a cell drained but never handed to `retarget_link` is acked and never heard; it turns
  red.
- Gate 3: a link value left in a queue (refused for room, or rebuilt), a miscounted supersession,
  or a write made before a later check fails turns it red. #1345's gate 1 covers parameters, not
  the link cell.
- Gate 4: a carry that refuses unequal link modes resets the lane at the swap, and a join that
  forgets an unread link write leaves the successor on the old mode; either turns it red.
- Superseded in this PR: any `live_delta.rs` test that asserts a link-mode change is structural for
  these four effects.

## Dependencies

- *Ramp a lane's detector link between modes* (#1370).
- *Declare a strip's console link mode in the session, the wire and the SDK* (#1369).
- *Hold live values in latest-target cells on both hosts* (#1312): the cell primitive and the
  counter.
- *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345): the drain
  order this slice extends.
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054): `link_samples` (`fader_ms`).
- *Carry an optional per-edit ramp length on live session edits* (#1394): the per-edit ramp
  field, `EditRamps` and `LiveRamps::resolve`.
- *Resolve an absent live ramp to the session default on the browser and in the SDK* (#1364).
- *Carry console effect lanes across a plan swap* (#1279).
- *Carry live-controlled effect lanes across a plan swap* (#1280).
