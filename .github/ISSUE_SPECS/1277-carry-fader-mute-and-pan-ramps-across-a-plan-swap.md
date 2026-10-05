# Carry fader, mute and pan ramps across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8).
Slice 8 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

A strip keeps its exact fader, mute and pan or matrix state through a plan swap, including a ramp
that is in flight at the swap block. This holds in move mode and in copy mode (*Carry plan state by
copy as well as by move*, #1322). A producer who pulls a fader while a track is added hears the ramp
go on, not jump. A live record admitted before the swap is applied, not lost. A structural
transaction that also changes a carried strip's fader, mute or pan sounds exactly like "live edit,
then structural edit" (D15-7, carry then retarget).

## Context

- The seam-side builtins render in banks:
  - `FaderBankProcessor` (`crates/builtins-compiler/src/lib.rs:715`, impl `:725`);
  - `MatrixBankProcessor` (`:778`, impl `:787`);
  - the fused `FaderMatrixBankProcessor` (`:831`, impl `:1172`), used in plans prepared between
    render calls (the browser).
- Their kernels and state are `FaderStage` (`crates/builtins/src/lib.rs:2583`), `FaderRampStage`
  (`:2653`, with its `remaining` count per lane) and `MatrixStage` (`:2884`, with its settled-flag
  sync).
- On `main` these banks drain bounded live queues inside `process` (#1253). *Hold live values in
  latest-target cells on both hosts* (#1312), a dependency, replaces those queues with
  latest-target cells: render applies every dirty cell at block entry, and the stage exposes
  `apply_pending(&mut self)`, the same read and canonical order (fader before mute, left before
  right, #1312 D4 and D6). Its cell writers cannot fail (#1312 D5). This slice is written against
  the cells.
- Effective fader values include VCA offsets (`SessionModel::effective_strip_faders`,
  `crates/session/src/vca.rs:100`).
- The input-section carry exists (#1276, on `main`): the rack slot accessor, `as_any_mut`, the
  chain agreement rule, the inventory rows (`InputSectionInventoryRow`,
  `crates/host-core/src/prepare.rs:569`) and the join against `SuccessorBase::committed`
  (`:632-647`). This slice follows the same pattern.
- The classifier `classify_live_delta` (`crates/host-core/src/live_delta.rs:211`) emits the records
  that bring a running plan from one model's live values to another's (`LiveStripRecords`, `:63`).
  It refuses any delta whose track set differs, so it cannot be called on a structural delta.

## Decisions frozen for this slice

- **D1. Keys.** The owner keys are `(strip ID, PostFader)` and `(strip ID, PostMatrix)`, or the
  fused stage's key. Padding lanes never carry.
- **D2. Comparison base (D15-7, P1.4).** The join compares the successor's model with the
  **predecessor's base**. The base is the model the predecessor plan was prepared from, plus every
  live record pushed to that plan. This slice rewrites `SuccessorBase::committed`'s doc to say so.
  - Today the C ABI passes its committed model, and the two are equal there: a structural
    transaction is refused while a candidate is pending, and `commit_live` pushes to the newest
    plan.
  - *Supersede an unadopted candidate plan by compare-and-swap* (#1310) must pass the displaced
    plan's own base, and that issue owns the gate for it.
- **D3. Live and prepared values.** A value of this family is *live* when the classifier would emit
  a record for its change on a plan that attaches this owner's control kind. Every other difference
  is *prepared*. Today a VCA in either model is prepared (`LiveRebuild::Vca`, until #1247).
- **D4. Rule.** A stage carries when both plans attach the same control kind and no prepared value
  of it differs between the base and the successor's model. A difference in live values does not
  stop the carry (D5). A stage that does not carry starts at rest, and its strip ID goes into the
  successor's **restart set** (D6).
- **D5. Carry, then retarget (D15-7).**
  - The per-strip record derivation inside `classify_live_delta` is extracted into one per-owner
    function. `classify_live_delta` calls it; nothing is duplicated.
  - For every carried strip whose live values differ, the successor entry points that return
    `HostLiveControlHandles` write that strip's records into the successor's fader and matrix cells
    (#1312's D5 writers) before they return: the fader and mute cells, then the matrix cell.
  - At the swap block the carry copies the predecessor's lane state, then the successor applies its
    own dirty cells at block entry. So the record applies on the swap block's first sample, exactly
    where a live edit written to the predecessor just before the swap would apply.
  - A cell write cannot fail. There is no room check, and preparation gains no failure and never
    returns BACKPRESSURE for a retarget (D15-2). No ack precedes a drop: every fallible step of
    preparation runs before the writes, and the transaction commits after them.
  - Copy mode writes nothing at preparation. The records are kept on the prepared successor. *Pre-roll
    a successor whose latency grows* (#1287) writes them into the cells at publication with the held
    live edits, so they apply at `S` (D15-17).
- **D6. Restart set.** `PreparedHost` gains `restarted_strips()`: the strip IDs present in both
  plans for which at least one owner failed D4 for a prepared difference, sorted. Added strips are
  not in it. *Duck-swap a strip whose state cannot continue across a plan swap* (#1324) consumes it,
  and the later carry slices add their families' restarts to it.
- **D7. Lane state.** Each of the fader/mute stage, the matrix stage and the fused stage gets a
  fixed-size, plain-data lane state: current gains, ramp targets and positions (`remaining`), and
  the matrix ramp state. Each bank gets `export_lane` and `import_lane`. Import re-syncs derived
  state: the matrix stage's settled flags and the ramp's `remaining`.
- **D8. Move mode.** Call #1312's `apply_pending` on each predecessor stage once, so every value
  written to its cells is applied, then export the lane and import it into the successor lane.
- **D9. Copy mode (#1322 D2, D7).** The same `apply_pending`-then-export, into the successor's
  preallocated lane. `apply_pending` applies the cells at the boundary the predecessor's next block
  would apply them, so none of the predecessor's bits move. The section adds its bytes to `carry_program_copy_bytes`.
- **D10. Fused and split forms.** A lane may move between the split and the fused form only if a
  host prepares both kinds for one session. No host does. So a cross-form pair does not carry: it
  starts at rest, and the join records it in the restart set.

## Deliverables

1. D7 in `crates/builtins`. D8-D9 in `crates/builtins-compiler` and `crates/graph` (the program
   section, both modes). The inventory rows, the join (D2-D4), D5 and D6 in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`
- `crates/host-core/src/live_delta.rs`: only the extraction of D5's per-strip function. This is
  stream B's file, and root orders the merge with *Hold live values in latest-target cells on both
  hosts* (#1312).

## Non-goals

- No solo or VCA host state, and no VCA retarget. VCA changes stay prepared until #1247.
- No effects and no delay lines.
- No transition for a restarted strip (#1324).
- No change to the C ABI or the browser call sites. The successor entry points write the records
  into the cells themselves.

## Objective gates

1. **Gap-free acceptance, both widths.** At `Backend::Simd8` and `Backend::Simd4`, session A has
   nine tracks with non-unity faders and non-centre pans (per track, per channel), plus #1276's
   input settings. Session B adds a muted track whose ID sorts first. Every block of the swapped run
   equals a fresh B fed from frame 0.
2. **Ramp in flight.** Prepare with live controls. Write a fader record with a multi-block ramp to
   A's cells between blocks 5 and 6, commit it in the base, and swap at block 6, before A renders
   again (so the value is still dirty in A's cell and only `apply_pending` applies it). Write a pan
   record the same way. Every block equals the reference A plus the muted track, fed the same
   records at the same blocks. A second case writes the records to the pending successor's cells:
   also bit-identical.
3. **Fused form.** Gate 2 again, with both runs prepared between render calls
   (`FaderMatrixBankProcessor`), the browser's form.
4. **Carry, then retarget.** B also changes strip X's fader and pan (live values). In run 1, X's
   stage carries and D5's records retarget it. In run 2, the same records are written to A just
   before the swap, and B is prepared from a base that already holds them. Both runs are
   bit-identical in every block. X is not in the restart set.
5. **Prepared change restarts.** B adds a VCA with a +3 dB offset over X. X's stage starts at rest
   at the new effective value, and `restarted_strips()` is exactly `[X]`. #1247 rewrites this gate
   when VCA changes become live.
6. **Copy mode.** Gate 2's swap is replaced by a copy after block 6 followed at once by the
   adoption (#1322 gate 2's shape). Every block equals the move run. A keeps rendering after the
   copy and equals its uncopied twin.
7. **Realtime.** The swap block and the copy call each make zero allocations and frees.
8. Commands:
   - `cargo test --locked -p builtins -p builtins-compiler -p graph -p host-core --features builtins-compiler/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-audit.sh target/release/audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - `cargo test --locked -p console-workload`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh`, `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 2: a ramp imported without its `remaining` count, or a value lost because the carry
  exported before `apply_pending`, turns it red.
- Gate 3: a carry that handles only the split banks leaves the browser's fused form at rest. It
  turns red.
- Gate 4: a join that refuses to carry on a live difference starts X at rest. A carry that skips the
  retarget keeps X's old gain. A retarget applied before the carry import (the import overwrites it)
  keeps the old gain too. Each turns it red.
- Gate 5: a rule that treats a VCA change as live carries X at the old gain. A restart set that
  lists added or unchanged strips fails the exact list. Either turns it red.
- Gate 6: a copy that skips `apply_pending`, or imports into the wrong lane, turns it red. An
  `apply_pending` that moves the predecessor's bits fails the twin.

## Dependencies

- *Hold live values in latest-target cells on both hosts* (#1312): the cells D5 writes and the
  `apply_pending` D8 and D9 call.
- *Carry plan state by copy as well as by move* (#1322).
- #1253 and #1276 are on `main`.
