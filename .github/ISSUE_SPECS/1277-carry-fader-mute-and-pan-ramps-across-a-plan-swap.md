# Carry fader, mute and pan ramps across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8).
Slice 8 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

A strip keeps its exact fader, mute and pan or matrix state through a plan swap, including a ramp
that is in flight at the swap block. The carry moves the state at the swap block (move mode), and
the predecessor never renders again. A fader move made while a track is added keeps
ramping; it does not jump. A live record admitted before the swap is applied, not lost. A structural
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
    returns BACKPRESSURE for a retarget (D15-2), except the stamp's refusal (#1432 D1). A fresh
    candidate gate cannot return it. If it is returned, it fails the rebuild with the internal
    refusal, and the candidate is dropped (#1312 D7). No ack precedes a drop: every fallible step of
    preparation runs before the writes, and the transaction commits after them.
  - The retargets are written exactly once, at preparation, before the transaction commits. Nothing
    writes them again: not at publication, not at adoption. A live edit committed after the
    successor's commit writes the newest candidate's cells later (#1053 D7), so a retarget, which
    comes from the older prepared model, can never overwrite it. This holds for every successor,
    including a warm successor that adopts at a later block `S` (#1287): its cells stay dirty until
    its first block and apply there.
  - Amendment 1: each retarget write is stamped from the successor plan's own gate handle with the
    structural transaction's revision. That revision is published on the successor's gate before
    the successor is published (#1312 D6, D7;
    *Give each plan its own revision gate and take each block's live snapshot from it*,
    #1502). The successor applies its dirty cells under its adoption block's snapshot, which
    covers that revision (#1432 D1).
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
- **D9. Fused and split forms.** A lane may move between the split and the fused form only if a
  host prepares both kinds for one session. No host does. So a cross-form pair does not carry: it
  starts at rest, and the join records it in the restart set.

## Deliverables

1. D7 in `crates/builtins`. D8 in `crates/builtins-compiler` and `crates/graph` (the program
   section). The inventory rows, the join (D2-D4), D5 and D6 in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`
- `crates/host-core/src/live_delta.rs`: only the extraction of D5's per-strip function. This is
  stream B's file, and root orders the merge with *Hold live values in latest-target cells on both
  hosts* (#1312).
- Added by Amendment 3:
  - `crates/engine/src/realtime/plan.rs`: D11's method and the handle docs only (B's file).
  - `crates/control-plane/src/control.rs`: the structural arm's `SuccessorBase` literal
    (`revision`), the one read of `r` with its range check before `prepare_runtime`, and the
    existing stamp's use of that value. Nothing else. `compile.rs` is not touched.
  - `crates/capi/tests/resource_lifecycle.rs`: the one `SuccessorBase` literal, `revision: 1`.
  - `crates/capi/src/runtime/live_tests.rs`: gate 8 only.
  - Host-core test support: it must publish `r` on the successor's handle before
    `reserve_replacement`. Otherwise the stamped retarget is never covered and is never heard.

## Non-goals

- No solo or VCA host state, and no VCA retarget. VCA changes stay prepared until #1247.
- No effects and no delay lines.
- No transition for a restarted strip (#1324).
- No change to the C ABI surface or header, or to the browser call sites. The successor entry
  points write the records into the cells themselves. The control plane changes only where it reads
  `r` and builds `SuccessorBase` (D10).

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
6. **Realtime.** The swap block makes zero allocations and frees.
7. Commands:
   - `cargo test --locked -p builtins -p builtins-compiler -p graph -p host-core --features builtins-compiler/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-audit.sh target/release/audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - `cargo test --locked -p console-workload`
   - `cargo test --locked -p capi` (added by Amendment 3)
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh`, `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
8. **A refused candidate is never reused (Amendment 2; strengthened by Amendment 3).** In
   `crates/capi/src/runtime/live_tests.rs`, beside
   `a_fault_before_the_live_write_leaves_every_cell_and_the_output_alone`, with the `Rig` and twin
   pattern, a debug build and `test_set_structural_faults(.., [Some(BeforeProtocolCommit), None])`:
   a rebuild that adds a track and changes strip X's fader to g1 (a carry retarget) is refused at
   `BeforeProtocolCommit`. Then the same structural change is committed *without* X's fader change.
   The twin takes only that change, and every block is bit-identical. No debug assertion fires. A
   refused attempt uses no revision, so the next one stamps the same `r`; if the accepted rebuild
   were the same edit, a kept candidate, cell or stamp would be inaudible and the gate could not go
   red.

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
- Gate 8: a refused candidate (plan, epoch, handle or cells) kept and reused, which would carry an
  open stamp and a stale retarget into a later transaction.

## Amendment 1 (root, 2026-10-05): revision-bounded cells

Root's binding requirement (2026-10-05) makes every live value of one revision take effect in one
block. #1432 Amendment 1 and #1312 Amendment 2 put the cells under each block's snapshot, with one
revision gate per plan (#1502).

- D5 gains the stamp rule.
- D8's `apply_pending` reads the quiescent predecessor with `LiveSnapshot::ALL` (#1312 D6). Every
  predecessor write is below the successor's revision, so this equals reading under the adoption
  block's snapshot.

Nothing else changes.

## Amendment 2 (root, 2026-10-11): a refused candidate is never reused

Root ruling on #1312 D7 (#1312 Amendment 3, triggered by #1432 follow-ups verdict m3): D5's carry
retargets are stamped with the structural revision `r` inside preparation, before fallible steps.
That is safe only because a refused candidate (plan, provider epoch, gate handle and cells) is
dropped by the same call and never kept, cached or reused. New gate 8 defends it. Nothing else
changes.

## Dependencies

- *Hold live values in latest-target cells on both hosts* (#1312): the cells D5 writes and the
  `apply_pending` D8 calls.
- #1253 and #1276 are on `main`.

## Amendment 3 (root, 2026-10-11): where preparation takes the retarget stamp

Trigger: the stream A coordinator reported a spec gap. In the authorized paths, preparation has
neither `r` nor the handle: `SuccessorBase` has no revision, and the control plane takes the handle
after preparation (`compile.rs:673-676`). Gate 8's fault is in the control plane, and its fault
tests are in capi. Root ruled option (b): an engine method that stamps on the plan's own pin.

- **D5, Amendment 1's stamp sentence becomes:** Each retarget write is stamped with the structural
  revision `r` on the successor plan's own gate pin, through
  `PreparedRenderPlan::stamp_held_gate`, while the plan still holds its handle. The open stamp
  moves with the pin into the handle that the control plane takes after preparation returns (#1502
  D1, unchanged). The control plane stamps `r` again on that handle as its last fallible step
  (`GatePin` allows a stamp at the open revision). It publishes `r` before `reservation.commit()`.
  A preparation that writes no retarget does not stamp.
- **D10. The revision.** `SuccessorBase` gains `revision: u64`. This is `r`, the revision the
  successor commits as. Precondition: `1 <= revision <= GATE_REVISION_MAX`.
  - The control plane reads `r` once from the prepared token, before `prepare_runtime`, and uses
    that one value for the base and for its stamp.
  - If `r` is out of range, the control plane returns `CommandError::Internal` before preparation,
    with nothing built. #1503 D1 makes this unreachable.
  - Host-core checks the same precondition at entry, before it allocates. It refuses with the
    whole-session line `host.successor.revision` (`PrepareRejection::Session`).
  - A `None` or `Err` from `stamp_held_gate` returns the same line. This cannot happen after the
    entry check, because a pin that was not handed out has published nothing.
  - `revision` has no neutral value. So #1323 D3's constructor is
    `SuccessorBase::new(inventory, committed, revision)`, and #1323's sentence "the first slice that
    adds a field" no longer holds. Root amends #1323. #1310 D2's base and #1344's literals carry `r`
    too.
- **D11. Engine method.**
  `PreparedRenderPlan::stamp_held_gate(&mut self, revision: u64) -> Option<Result<LiveStamp<'_>, StampRefused>>`.
  It returns `None` after `take_revision_gate`; otherwise it is `GatePin::stamp` on the plan's own
  pin. The `PlanRevisionGate` doc changes from "only the holder stamps" to "the plan's owner before
  hand-out, then the holder". No other engine change.
- **Gate 8** goes in `crates/capi/src/runtime/live_tests.rs`, beside
  `a_fault_before_the_live_write_leaves_every_cell_and_the_output_alone`. It uses the `Rig` and twin
  pattern, a debug build and `test_set_structural_faults(.., [Some(BeforeProtocolCommit), None])`.
  - Change to the gate: after the refused rebuild (add a track, and strip X's fader to g1), commit
    the same structural change *without* X's fader change. The twin takes only that change, and
    every block must be bit-identical.
  - Reason: a refused attempt uses no revision, so the next one stamps the same `r`. If the
    accepted rebuild is the same edit, a kept candidate, cell or stamp is inaudible and the gate
    cannot go red. The existing capi test uses this same reasoning.
  - Gate 8 in "Objective gates" is updated to this shape, and `cargo test --locked -p capi` is added
    to gate 7.
- The "Authorized paths" section gains the Amendment 3 bullets, and the last "Non-goals" bullet is
  replaced, as in the sections above.

## Attempt record

### Attempt 1

Stopped before any edit, on a spec gap: preparation had neither `r` nor the gate handle in the
authorized paths (see Amendment 3's trigger). No code was written. Root ruled option (b) and issued
Amendment 3.
