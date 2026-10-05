# Carry the link record from the edit to the lane

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-7, D15-9, D15-13 E4).
Slice L3b of *Let a strip override a console slot's link mode* (#1236).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A host changes a strip's console link override, a console slot's default link mode, or an insert's
link mode on the compressor, gate-expander, transient shaper or true-peak limiter, and the change
is a live update: no plan rebuild, the detector glides over the session ramp, and the edit is
heard from the next block. A swap during the glide carries it. After this slice, decision 15's
D15-9 duck-swap covers a link-mode change only for effects whose link stays prepared.

## Context

- Records reach an effect through its `EffectControlLane`: `EffectControlRecord`
  (`crates/effect-contract/src/live.rs:54` onward) has `Parameter`, `PreparedTarget`, `Observe`
  and `Bypass`; `stage` (`:341`) applies a `Bypass` to the lane's flag at the block boundary
  (`:398-401`). Banked lanes stage in `LiveControlEffectBankStage::drain` and apply in
  `process_inner` (`crates/rack/src/lib.rs:1257`, `:1294`); per-node lanes in the graph runtime's
  `NodeKind::LiveControlEffect` arms (`crates/graph/src/runtime.rs:1527`, `:3510`).
- *Ramp a lane's detector link between modes* (#1370) gives the effects
  `retarget_link(track, mode, samples) -> bool`, render-safe.
- The C ABI classifier compares an effect's identity, quality, link mode and sidechain
  structurally (`crates/host-core/src/live_delta.rs:150-156`), so any link change is
  `LiveRebuild::Structure`. Bypass records are emitted per instance ahead of parameter records
  (`:376-395`).
- The session's ramp default is *Session `controlSmoothing`: configurable ramp lengths for live
  mute, fader and pan changes* (#1054, D15-1).
- The browser reaches the same classifier when its control plane moves to the shared crate (*Run
  the browser control plane in a Worker and keep the AudioWorklet render-only*, #1332, on *Extract
  the C ABI control plane into a portable crate both hosts call*, #1309). The worklet message
  protocol stays internal (D15-11); no browser command kind is added.

## Decisions frozen for this slice

- **D1. Record.** `EffectControlRecord::Link { mode: LinkMode, samples: u32 }`. `stage` stores
  it as the lane's pending link (the last one in a drain wins, as for bypass); the stage that owns
  the processor calls `retarget_link(track, mode, samples)` at the block boundary, before
  processing. A `false` return increments the lane's existing refusal counter; the classifier
  makes it unreachable (D3).
- **D2. Ramp length.** `samples` is the session's `controlSmoothing` default for effect link
  changes, from #1054. An explicit 0 is a step (D15-1).
- **D3. Classifier.** For an effect whose descriptor has `lane_link` (#1368 D1), a resolved link
  mode that differs between the committed and the next model is not structural: the classifier
  emits one `Link` record for that instance, after any `Bypass` record and before its parameter
  records. The resolved mode is the entry's override or, for `slot`, the slot's declared mode
  (#1369 D1), so a changed slot default emits one record per strip whose entry says `slot`; a
  changed insert link emits one for that insert. A mode the effect does not support stays a
  preparation refusal (`effect.link_mode.unsupported`), never a record. For any other effect a link
  change stays structural (D15-9 duck-swap).
- **D4. Admission and the acked-batch question.** A `Link` record takes a slot in the lane's queue
  like any record. The transaction validates every record and checks queue room (with #1280 D3's
  rule while a successor is pending) before the first push, and refuses the whole transaction with
  typed backpressure otherwise. So an ack never precedes a drop.
- **D5. Carry (D15-7).** A swap carries the lane's link state through the payload (#1370 D6), and
  a pending `Link` record through the inherited queue (#1280 D2). *Carry console effect lanes
  across a plan swap* (#1279) D1 requires link mode to be equal for a carry; for `lane_link`
  effects this slice amends that rule to "carry, then retarget": the successor is prepared from the
  committed model, and the classifier's `Link` record moves the carried lane to it.

## Deliverables

1. D1 in `crates/effect-contract/src/live.rs`, `crates/rack/src/lib.rs` and
   `crates/graph/src/runtime.rs`.
2. D3 and D4 in `crates/host-core/src/live_delta.rs` (or its successor in the control-plane crate,
   #1309), with D2's ramp source.
3. The carry rule amendment in the inventory join (`crates/host-core/src/prepare.rs`) and #1279's
   spec text if still open.
4. Docs: `crates/capi/include/miso_engine_v1.h` "Live edits" and
   `docs/C_ABI_V1_QUALIFICATION.md:393` (link mode leaves the rebuild list for these effects).

## Authorized paths

- `crates/effect-contract/src/live.rs` (the record and `stage`; streams A, B and E also edit this
  file: rebase on their merges)
- `crates/rack/src/lib.rs`, `crates/graph/src/runtime.rs` (the record's application only; stream
  A's files: coordinate)
- `crates/host-core/src/live_delta.rs`, `crates/host-core/tests/live_delta.rs`,
  `crates/host-core/src/prepare.rs` (the carry rule only), `crates/capi/src/runtime/live_tests.rs`,
  `crates/capi/include/miso_engine_v1.h` (docs only); stream B owns these: coordinate the merge
- `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- A browser command kind; the browser gets this through the shared control plane (#1332).
- The multiband compressor's link (#1367).

## Objective gates

1. **Classifier.** For each of the four effects: an entry override change, a slot-default change
   (three strips on `slot`, one overriding) and an insert link change each classify live, with
   exactly the `Link` records D3 names and no record for the overriding strip on a slot-default
   change. The same changes on a `lane_link = false` effect stay `LiveRebuild::Structure`.
2. **Live on the C ABI.** A committed override change on a console compressor: no plan
   replacement; the render equals a reference plan fed the same `Link` record at the same block.
3. **Backpressure, not a drop.** With the lane's queue one short of full, a transaction carrying a
   `Link` and a parameter record for that lane is refused whole, the committed model and the
   running plan unchanged; after one block it commits.
4. **Swap mid-glide.** A rebuild that adds a track while a lane's link ramp is in flight: the
   swapped run and a reference that never swapped are bit-identical for the carried lane.
5. **Realtime.** The block that applies a `Link` record makes zero allocations and frees.
6. Commands:
   - `cargo test --locked -p effect-contract -p rack -p graph -p host-core -p capi --features graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
     `cargo fmt --all -- --check`; `bash scripts/check-workspace-policy.sh`;
     `bash scripts/check-realtime-policy.sh`

## Test value

- Gate 1: a classifier that resolves `slot` against the old slot default, or emits for an
  overriding strip, applies the wrong mode on one strip; it turns red.
- Gate 2: a record staged but never handed to `retarget_link` is acked and never heard; it turns
  red.
- Gate 3: admission that pushes the `Link` before checking room for the rest drops an acked
  record; it turns red.
- Gate 4: a carry that refuses unequal link modes resets the lane at the swap; it turns red.
- Superseded in this PR: any `live_delta.rs` test that asserts a link-mode change is structural for
  these four effects.

## Dependencies

- *Ramp a lane's detector link between modes* (#1370).
- *Declare a strip's console link mode in the session, the wire and the SDK* (#1369).
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes* (#1054).
- *Carry console effect lanes across a plan swap* (#1279) and *Carry live-controlled effect lanes
  across a plan swap* (#1280).
