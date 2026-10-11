# Give each plan its own revision gate and take each block's live snapshot from it

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-3, D15-17).
Code anchors verified on `codex/d15-stream-b` at `13f335d79`.

## Product outcome

Every prepared plan carries its own revision gate (#1432 D1).

- Render takes the block's live snapshot `S` from the running plan's gate once per block, hands
  it to the plan's executor, and advances the applied-revision watermark to it.
- The control plane publishes every committed revision on the gate of the plan that carries its
  content: the newest provider epoch. That is the routing that already decides which plan's live
  producers a commit writes, so cells and gate follow one routing.
- The mailbox's revision words and their routing go.

This slice changes no rendered bit. It provides the snapshot.
- *Hand each block's live snapshot to every live drain* (#1504) passes `S` to the drains.
- #1312, #1345, #1346 and #1347 then read cells under it, and the watermark becomes exact for each
  lane family as it moves to cells.

## Context

- **Root ruling (2026-10-05): one gate per plan.** The design put the gate in the mailbox cell's
  revision word. The review (`docs/handoffs/decision-15-2026-10-05/revision-bounded-cells/review.md`)
  found two MAJOR defects in that placement.
  - M2: cell writes follow the control plane's provider routing
    (`pending_providers.last_mut()`, else the current provider,
    `crates/control-plane/src/control.rs:1296`), while the mailbox word routes the gate. No check
    ties the two together, so a mismatch would be a silent torn read.
  - M1: a browser build that renders through the exchange while the worklet still admits live
    edits (#1381 before #1382) would never publish the worklet's revisions on the mailbox gate.
  A gate per plan removes the second routing: the gate travels with the plan, its cells and the
  provider epoch that writes them.
- **The landed revision words (#1314).**
  - `crates/engine/src/realtime/spsc.rs`:
    - invariant I7 (`:487`) and `revisions: [AtomicU64; 2]` (`:655`);
    - `MailboxWithdrawal::Withdrawn(T, u64, PlanAdoption)` (`:740`);
    - `MailboxWriter::store_revision` (`:849`);
    - `MailboxPermit::commit(value, revision, adoption)` (`:887`, its revision argument since
      #1311 N4);
    - `MailboxReader::active_revision` (`:943`).
  - `crates/engine/src/realtime/plan_exchange.rs`:
    - `RevisionTarget` (`:86`);
    - `UnadoptedCandidate::set_revision` (`:274`);
    - `plan_exchange_at_revision` (`:394`);
    - `PlanPublisher::set_revision` (`:513`), with its `revision` field;
    - `withdraw` (`:532`) and `republish` (`:568`), which carry the word;
    - `PlanReplacementReservation::set_revision` (`:626`).
  - Render loads `active_revision()` after `enter_block` and advances the watermark after
    `render_inner` returns `Ok`: `render_contiguous` (`:781`, `:799`, `:804`) and `render`
    (`:814`, `:824`, `:826`).
  - Control plane: `publish_committed_revision` (`crates/control-plane/src/control.rs:1578`,
    called as the live commit's last write at `:1560`); the rebuild's `reservation.set_revision`
    (`:1231`) through `ObservedReservation::set_revision` (`crates/control-plane/src/plan.rs:173`);
    `plan_exchange_at_revision` in `compile.rs:761`; the `RevisionTarget` re-export
    (`crates/control-plane/src/lib.rs:37`).
- **The plan.** `PreparedRenderPlan` (`crates/engine/src/realtime/plan.rs:562`) is prepared on
  the control thread (`prepare` `:591`, `prepare_with_executor` `:613`). It renders through
  `render_inner` (`:952`), which both exchange paths and the direct `render` (`:870`) and
  `render_contiguous` (`:888`) use. `RenderReport` (`:547`) is built in one place (`:979`). The
  executor seam is `PreparedPlanExecutor` (`:269`).
- **Resources.** The control plane charges per-plan engine bytes into
  `graph_session_plus_plan_bytes`, as it does the carry program's bytes
  (`crates/control-plane/src/compile.rs:666-669`).

## Decisions frozen for this slice

- **D1. A gate per plan.**
  - `PreparedRenderPlan` holds one shared gate allocation (an `Arc` of #1432's `RevisionGate`),
    made at preparation on the control thread with revision 0.
  - `PreparedRenderPlan::take_revision_gate(&mut self) -> Option<PlanRevisionGate>` hands out the
    plan's one control handle (the shared gate and a `GatePin`) the first time, and `None` after.
  - Only the handle's holder publishes on that gate or stamps writes to that plan's cells. A
    handle moves between threads only by ownership transfer.
  - The engine exports `plan_revision_gate_retained_bytes()`, the allocation's exact layout. The
    control plane adds it to each prepared plan's `graph_session_plus_plan_bytes`, as it adds the
    carry program's bytes.
- **D2. Render takes the block's snapshot.**
  - `render_inner`, after its output-shape and clock checks and before the executor renders,
    takes `S = snapshot()` of the plan's own gate: one `fetch_or`.
  - It calls the new `PreparedPlanExecutor::begin_live_block(&mut self, snapshot: LiveSnapshot)`
    (default: nothing), renders, and returns `S` in the new `RenderReport::live_snapshot`.
  - Every render path reads its own plan's gate: both exchange paths, and the direct `render` and
    `render_contiguous`. No render path uses `LiveSnapshot::ALL`.
  - A block refused by the shape or clock check takes no snapshot.
- **D3. The watermark advances to `S`.**
  - `RealtimePlanOwner::{render, render_contiguous}` advance the watermark with
    `report.live_snapshot` after `render_inner` returns `Ok`, in place of `active_revision()`.
  - The claim's `superseded` and `outcome` payload words and their accounting are unchanged
    (#1314 D5).
  - The initial watermark is the initial plan's gate revision (D5).
- **D4. The mailbox carries no revision.** These go:
  - in `spsc.rs`: I7 and the revision words, `store_revision`, `active_revision`, the revision
    argument of `MailboxPermit::commit` and the revision of `MailboxWithdrawal::Withdrawn`;
  - in `plan_exchange.rs`: `RevisionTarget`, `PlanPublisher::set_revision` and its `revision`
    field, `PlanReplacementReservation::set_revision`, and
    `UnadoptedCandidate::{revision, set_revision}`;
  - in the control plane: `ObservedReservation::set_revision` and the re-export.

  A candidate's revision is its plan's gate, so a withdrawn candidate takes it along by
  construction, and `republish` writes none.
- **D5. The initial revision.** `plan_exchange_at_revision(initial, gate, revision, config)` takes
  the initial plan's handle (`&mut PlanRevisionGate`, checked to be that plan's by pointer
  identity). It publishes `revision` on the handle when the revision is above 0, and starts the
  watermark there. `plan_exchange(initial, config)` stays for hosts that number no revisions.
  Refusing a revision above the ceiling is
  *Bound committed revisions at the plan gate's ceiling* (#1503).
- **D6. The control plane publishes through the provider epoch: one routing.**
  - `ProviderEpoch` (`control.rs:29`) gains its plan's `PlanRevisionGate`, taken from the prepared
    plan before publication: in `compile.rs` for the initial plan, and where a rebuild's candidate
    is prepared.
  - A live or model-only commit publishes its revision on the newest epoch's gate, as the commit's
    last write, in #1314 D2's position (`control.rs:1560`, `:1578`). The newest epoch is the
    last pending provider, else the current one: the routing that chooses the producers the
    commit writes (`:1296`).
  - A rebuild publishes its revision on the candidate epoch's gate before
    `reservation.commit()` (`:1231`).
  - #1314 Amendment 1's debug assertion goes: there is no second routing to compare against.
  - #1312 D7 takes cell-write stamps from the same handle.
- **D7. An executor error after the snapshot is a counted defect path (root ruling 3,
  2026-10-05).**
  - If the executor returns an error after `begin_live_block`, drains may have applied values in
    a block that rendered no output. Such a block is not atomic. No valid plan reaches this
    (`InvalidEnvelope` and `Buffer` are invariant failures), so it is a defect path and has no
    recovery machinery.
  - `RealtimePlanOwner` counts such blocks in a saturating `live_block_errors`, read like
    `carry_mismatch_count`. The control plane's `PlanState::render` mirrors the count into
    `SharedPlanState`, and `PlanQueries::live_block_errors()` reads it.
  - The watermark does not advance on such a block (#1314 D3).
- **D8. What stays conservative until the drains read cells.**
  - Until #1504 hands `S` to every drain, each drain pops its queue at its node as today.
  - Until each lane family reads cells under `S` (#1312 strip lanes, #1345 effect lanes, #1346,
    #1347), the watermark stays conservative by at most one block: never early, possibly one
    block late (#1314 Amendment 2).
- **D9. The acked-batch question: can an ack ever precede a drop? No.**
  - No queue is added.
  - A revision lives on the plan that carries its content, so a withdrawn candidate takes its
    revision along.
  - Publication stays every commit's last write.

## Deliverables

1. D1-D5 and D7 in `crates/engine/src/realtime/` (`plan.rs`, `plan_exchange.rs`, `spsc.rs`,
   `mod.rs`, `watermark.rs` if its doc names the mailbox words, `latest_cell.rs` for the handle
   type if it lives there).
2. D6 and D7's mirror in `crates/control-plane/src/` (`control.rs`, `plan.rs`, `compile.rs`,
   `lib.rs`).
3. The tests of gates 1-5.

## Authorized paths

- `crates/engine/src/realtime/{plan.rs,plan_exchange.rs,spsc.rs,mod.rs,watermark.rs,latest_cell.rs}`.
- `crates/control-plane/src/{control.rs,plan.rs,compile.rs,lib.rs}`.
- `crates/capi/src/runtime/*tests*.rs`, `crates/capi/tests/*.rs`: only where a test names a
  removed API.
- `scripts/check-realtime-policy.sh` (floors only, raised to the measured counts).
- `hosts/host-web/tests/browser-v1/expected.json`: only if the gate's row moves a resource figure
  above its recorded ceiling, with this issue as the reason.

## Non-goals

- Passing `S` to the drains (#1504). Cells (#1312, #1345-#1347).
- The revision ceiling (#1503).
- The C ABI header, the watermark query and its struct: unchanged.

## Hazards

- A render path that takes `S` from any gate but its own plan's reports a candidate's revision
  before render adopts it, or the old plan's after (gate 1).
- Taking the snapshot after the executor renders makes the watermark name a block that applied
  nothing of `S` (gate 3's race test).
- A second control handle on one gate is two writers with two pins: `take_revision_gate` returns
  `Some` once.

## Objective gates

1. **The snapshot is the plan's own (engine unit test, `crates/engine/src/realtime/mod.rs`).**
   - A probe executor records each `S` that `begin_live_block` receives. Revisions are published
     on its gate between blocks.
   - Each block's `S` equals the revision published before that block, and the watermark advances
     to it at that block's first sample.
   - The same probe rendered directly (`PreparedRenderPlan::render`) also receives its own gate's
     `S`.
2. **#1314's engine gates on plan gates** (rewrite `a_revision_completes_at_adoption_not_at_publication`,
   `superseded_and_fallback_revisions_complete_once_per_adoption` and
   `a_revision_follows_a_held_or_published_candidate`, `mod.rs:623`, `:663`, `:740`). Each keeps
   its assertions.
   - A published candidate's revision completes at adoption, not at publication. A withdrawn
     candidate's revision leaves with it, and republished, it completes at adoption.
   - A revision published on a held candidate's gate leaves the watermark unchanged until that
     candidate is republished and adopted.
   - `superseded` and the fallback outcome are counted once per adoption.
   - `RevisionTarget` assertions go with the type.
3. **#1314's C ABI gates stay green unchanged.**
   - `a_live_edit_on_a_pending_candidate_completes_at_the_swap`.
   - `a_live_edit_completes_in_the_next_block`.
   - `a_watermark_advance_names_a_block_that_applied_the_edit` (`crates/capi/tests/plan_swap_race.rs`):
     mutation M14 (the snapshot taken after `render_inner`) still turns it red.
4. **Loom.**
   - Every `spsc_loom_plan_mailbox_*` model stays green with the revision words removed.
   - `spsc_loom_plan_mailbox_revision_follows_the_pending_candidate` (`spsc.rs:1261`) is replaced
     by a model in which control publishes on a candidate's plan gate while render claims and
     snapshots. Render's `S` is the claimed plan's: 1 while the withdrawal wins, the candidate's
     revision while the claim wins.
   - #1314's mutation L3 (render's cached `Active` index moved by a lost claim) stays red.
   - Command: `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo
     test --locked --release -p engine --lib spsc_loom`.
5. **A block that errors after the snapshot (engine unit test).** A probe executor returns an
   error after `begin_live_block`. `live_block_errors` reads 1 and the watermark is unchanged.
   The next rendered block publishes at its own first sample. A block refused by the shape check
   leaves `live_block_errors` at 0.
6. **Realtime.** `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   (0 allocations, deallocations, locks, syscalls, `"total_violations":0`);
   `bash scripts/check-realtime-policy.sh`; `bash scripts/test-realtime-policy.sh`.
7. **Workspace.** `cargo test --locked -p engine --features realtime-audit`;
   `cargo test --locked -p control-plane --features test-support`; `cargo test --locked -p capi`;
   `bash scripts/check-capi-abi.sh`; `bash scripts/check-cross-targets.sh`;
   `bash scripts/check-workspace-policy.sh`; `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.

## Test value

- Gate 1: a render path that reads another plan's gate, or the direct path left without a
  snapshot. No existing test sees `S`, because none exists.
- Gate 2: a withdrawn candidate that loses its revision, a revision on a held candidate reported
  early, or `superseded` counted per advance. These are #1314's defects, re-pinned on plan gates
  because their old form tested the deleted words.
- Gate 4: a claim race that snapshots the wrong plan.
- Gate 5: an errored block that advances the watermark or goes uncounted.

## Dependencies

- *Add the latest-target cell primitive and its loom model* (#1432): the gate.
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314): the code this
  reworks.
- *Adopt a successor plan no earlier than a scheduled sample* (#1311): `MailboxPermit::commit`'s
  signature.
- *Let the control thread withdraw an unadopted candidate plan* (#1343).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).

Dependents: #1503, #1504, #1312, #1310.
