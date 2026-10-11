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
    #1347), the watermark is never early. Its `first_sample` is the start of the first block
    that begins after the commit's last write, so at most one block after the submit returns
    (while a replacement plan is pending: the block that adopts it). A live value
    of that revision can apply earlier, in any block that ran between the submit's first push
    and its revision store. One transaction's values can spread across those blocks until #1502,
    #1503, #1504, #1312 and #1345 have all landed, which together make it exact (#1314
    Amendment 2).
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
   - #1314's mutation L3 (render's cached `Active` index moved by a lost claim) has no object any
     more: the reader holds no index (see the Attempt record, choice 3).
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

## Attempt record

Commit hashes cited in this record that name pre-rebase commits map to the rebased commits in
#1432's "Commit mapping after the rebase onto 7f8ecb8a1".

### Attempt 1 (implementer, on `codex/d15-stream-b2`, commits d1e0a69dd and 07d5c60ae)

**What landed.**

- `plan.rs`: `PreparedRenderPlan` holds `Arc<RevisionGate>` (built at revision 0) and its `GatePin`;
  `take_revision_gate() -> Option<PlanRevisionGate>` hands the one handle out once.
  `PlanRevisionGate { gate, pin }` has `revision`, `last_published`, `stamp`, `publish`.
  `plan_revision_gate_retained_bytes()` is the `{strong, weak, gate}` allocation (mirror struct, as the
  mailbox's). `PreparedPlanExecutor::begin_live_block(LiveSnapshot)` (default nothing).
  `render_inner` takes `S = gate.snapshot()` after the shape and clock checks, calls
  `begin_live_block`, renders, and returns `S` in `RenderReport::live_snapshot`; it returns a
  crate-private `BlockFailure { error, after_snapshot }`. No render path uses `LiveSnapshot::ALL`.
- `plan_exchange.rs`: `RealtimePlanOwner::render_active` advances the watermark to `S` after `Ok`, and
  counts `live_block_errors` (saturating) when `after_snapshot`. `plan_exchange_at_revision(initial,
  &mut gate, revision, config)` asserts the handle is the plan's by `Arc::ptr_eq`, publishes the
  revision when above 0 and starts the watermark there. `RevisionTarget`, `PlanPublisher::
  {set_revision, revision}`, `PlanReplacementReservation::set_revision` and
  `UnadoptedCandidate::{revision, set_revision}` are gone.
- `spsc.rs`: the revision words, I7, `store_revision`, `active_revision`, the reader's `active` index,
  the `u64` of `MailboxWithdrawal::Withdrawn` and `MailboxPermit::commit`'s revision are gone. I7 stays
  as a tombstone so I8 keeps its name.
- Control plane: `PreparedRuntime::revision_gate` (taken from the prepared plan in `prepare_runtime`);
  `ProviderEpoch::revision_gate`; the initial plan publishes its store revision through
  `plan_exchange_at_revision`; a rebuild publishes `r` on the candidate epoch's gate after the protocol
  commit and before `reservation.commit()`; `publish_committed_revision` publishes on
  `newest_providers_mut()`'s gate. #1314 Amendment 1's debug assertion is gone.
  `graph_session_plus_plan_bytes` adds `plan_revision_gate_retained_bytes()`.
  `SharedPlanState::live_block_errors` is stored by `PlanState::render` on the error path;
  `PlanQueries::live_block_errors()` reads it.
- Outside the authorized list, each for a reason: `scripts/check-web-audioworklet-callgraph.py`
  (the worklet chain failed: `render_inner` has a second `call_indirect`, `begin_live_block`, through
  the same `Box<dyn PreparedPlanExecutor>`, and the `impl` block index in the mangled names moved from
  `Ms2` to `Ms4`; both entries re-pinned with their reason); `docs/C_ABI_V1_QUALIFICATION.md` (one
  sentence named the deleted revision words); `crates/capi/tests/resource_lifecycle.rs` (one exact
  accounting assertion now adds the gate's bytes). `check-realtime-policy.sh` floors did not move
  (96 regions in 27 files); `expected.json` did not move (graphSessionPlusPlanBytes 29842 of 35648).

**Choices the spec left open.**

1. `PlanRevisionGate` lives in `plan.rs`, not `latest_cell.rs` (which this attempt does not edit).
2. A foreign handle given to `plan_exchange_at_revision` panics (documented), as `republish` does.
3. L3 (the reader's cached `Active` index) has no object any more: the reader holds no index. The
   earlier text here said the new loom model's `snapshot == 1` assertion holds its defect class.
   That was wrong: the model picks the gate in its own code, so the assertion can go red only when
   the model itself changes, and no production mutation reaches it. The argument that holds is
   structural: (a) the reader no longer has a cached index to move; (b) `RealtimePlanOwner` changes
   its active plan only when `claim` returns `Some`; (c) the mailbox models keep their "adopted and
   withdrawn both" assertions. The loom model still defends the publication order of D6 (mutation
   L1).
4. The new loom model builds the candidate's gate at 0 and publishes the rebuild's 7 before the commit
   and the live commit's 8 after it, which is D6's order, and checks a withdrawn candidate's gate reads 8.

**Gates (all on the committed tree).**

| Gate | Result |
|---|---|
| `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom` | 13 passed (the new model replaces `..._revision_follows_the_pending_candidate`) |
| `cargo test --locked -p engine --features realtime-audit` | 80 + 4 + 1 + 2 passed |
| `cargo test --locked -p control-plane --features test-support` | passed |
| `cargo test --locked -p capi` | 81 + 3 + 11 passed (gate 3's three tests included) |
| `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` | 0 allocations, deallocations, locks, syscalls; `"total_violations":0` |
| `check-realtime-policy.sh`, `test-realtime-policy.sh` | ok |
| `check-capi-abi.sh`, `check-cross-targets.sh`, `check-workspace-policy.sh` | ok / PASS / ok |
| `cargo fmt --all -- --check`; clippy `-D warnings` over the workspace; `cargo doc -D warnings` | ok |
| Worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh`) | pass after the re-pin above |

**Mutation evidence.** Each row applies one defect to the committed file and reverts it; "red" names
the tests that failed.

| Defect | Red |
|---|---|
| `begin_live_block` receives `LiveSnapshot::ALL` | `each_block_takes_its_snapshot_from_its_own_plans_gate`, `the_direct_render_paths_take_their_own_gates_snapshot`, `a_block_the_executor_fails_after_the_snapshot_is_counted` |
| `begin_live_block` never called | the same three |
| snapshot (and the call) before the shape check | `a_block_that_errors_publishes_nothing`, `each_block_takes_its_snapshot_from_its_own_plans_gate` |
| snapshot after the executor rendered (#1314 M14) | the same three as the `ALL` row; and `a_watermark_advance_names_a_block_that_applied_the_edit` (capi, release) |
| a pre-snapshot failure counted as a live-block error | `a_block_that_errors_publishes_nothing` |
| a post-snapshot failure not counted | `a_block_the_executor_fails_after_the_snapshot_is_counted` |
| the initial revision not published on the initial gate | `each_block_takes_its_snapshot_from_its_own_plans_gate` |
| ownership assert of `plan_exchange_at_revision` removed | `the_initial_revision_is_published_on_the_initial_plans_gate` |
| a second `take_revision_gate` returns a handle | `a_plan_hands_out_its_gate_handle_once` |
| live commit publishes on the current epoch, not the newest | `a_live_edit_on_a_pending_candidate_completes_at_the_swap` |
| rebuild does not publish `r` on the candidate's gate | `a_live_edit_on_a_pending_candidate_completes_at_the_swap`, `a_scheduled_candidate_survives_service_until_render_adopts_it` |
| `live_block_errors` mirror stores 0 | `the_live_block_error_count_reaches_the_any_thread_query` |
| gate bytes not charged to the graph row | `double_live_oracle_drives_exact_and_one_below_c_caps` |
| loom model: render snapshots the running gate after a claim | `spsc_loom_plan_mailbox_snapshot_follows_the_claimed_plan_gate` |
| loom model: render snapshots the candidate's gate after a lost claim | the same model |

**Test value, one line each.** Gate 1: a render path that reads another plan's gate or hands the
executor the wrong `S`. Gate 2 (three rewritten tests): a withdrawn candidate that loses its revision,
a revision on a held candidate reported early, `superseded` counted per advance. Gate 4: a claim race
that snapshots the wrong plan. Gate 5: an errored block that goes uncounted or counts a refused one.
The control-plane mirror test: a count that never leaves the render thread. The handle-once and
foreign-handle tests: a second writer on a gate and a revision published on another plan's gate.

**Open items.** None blocking. #1503's ceiling, #1504's drains and #1312's stamps are untouched.

### Batch follow-ups (stream B batch 2, after the five slices passed)

- **Authorized paths (verdict `1502-attempt1`, n1).** Three paths outside the list were edited, each
  justified, and are now authorized by this note: `scripts/check-web-audioworklet-callgraph.py`
  (two entries: D2 makes `begin_live_block` a trait method on the boxed executor, which adds a second
  `call_indirect` in `render_inner`, and the impl index in a mangled name moved from `Ms2` to `Ms4`;
  each entry gives its reason inline); `docs/C_ABI_V1_QUALIFICATION.md` (one sentence named the
  deleted revision words); `crates/capi/tests/resource_lifecycle.rs` (one accounting identity).
- **The gate's bytes are charged once, in host-core's graph row (m1).** The attempt charged each
  plan's gate (`plan_revision_gate_retained_bytes()`, 16 bytes on wasm32) only in the control
  plane, so the browser's exact retained total, which copies host-core's `graph_session_plus_plan_bytes`,
  left out one real allocation per plan. The charge now sits in `prepare_host_runtime...`'s
  `graph_session_plus_plan_bytes` (and in its admission check), the control plane's separate
  addition is removed, and both hosts count the gate. The same row now charges the browser's
  per-plan `live_values_superseded` count (see #1312 m3). `host-core`'s
  `the_graph_row_counts_the_plans_gate_and_a_count_it_made_and_holds` and `host-web`'s
  `the_exact_retained_total_counts_the_plans_gate_and_its_superseded_count` defend the row on both
  paths; capi's identity in `resource_lifecycle.rs` checks that the carry program is the only term
  capi adds (its second assertion, that the host row carries the gate, was removed by the blocker
  follow-ups: host-core's test is its unique catch). `expected.json` did not move.
- **`plan_exchange_at_revision`'s `# Panics` doc (n2).** It now says that only the foreign-handle
  check is an always-on `assert!`; "a revision not above the gate's published one" is
  `GatePin::publish`'s debug assertion and does not panic in a release build.

**Mutation evidence for the row change.** Each defect applied to `host-core/src/prepare.rs`, then
reverted (green).

| Defect | Red |
|---|---|
| the gate's bytes not added to the graph row | `the_graph_row_counts_the_plans_gate_and_a_count_it_made_and_holds` (host-core, renamed by the blocker follow-ups; `left: 263942, right: 263966` when first written); `the_exact_retained_total_counts_the_plans_gate_and_its_superseded_count` (host-web); `double_live_oracle_drives_exact_and_one_below_c_caps` (capi) |
| the preparation's own count not added | the same host-core and host-web tests (`left: 24, right: 48` on the browser's row, native widths) |
| the count added although the host passed the session's | the host-core test, and capi's second identity (`resource_lifecycle.rs`), which the blocker follow-ups removed so that the host-core test is the one catch (see #1312's blocker follow-ups) |

Test value: a host path (the browser's exact retained total above all) that leaves the gate or the
count out of the row while its ceilings stay green, because the row less the graph's incremental row
is the independent term the plan's own allocations are measured against.

**Gates for the batch follow-ups (all #1432, #1502, #1503, #1504, #1312 items together).**
`cargo test --locked --workspace --no-fail-fast` exit 0 (343 `test result: ok` lines, none failed);
`cargo test --locked --release -p engine --lib latest_cell` 12 passed; `cargo fmt --all -- --check`;
clippy `--workspace --all-targets --all-features -D warnings`; `check-capi-abi.sh` and `--self-test`;
`audit capi` (0 allocations, deallocations, locks, syscalls; `total_violations` 0); the workspace,
realtime (100 regions in 27 files), rack, builtins, host-core, protocol-control and graph policy
scripts; `check-cross-targets.sh` PASS; the worklet chain (`build-web-audioworklet.sh --named-twin`,
`check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py
--artifacts` with `expected.json` unchanged, `check-scalar-oracle-absent.py --wasm`,
`strip-wasm-names.py check`, `test-web-audioworklet.sh` with a private TMPDIR left empty). No exact
resource number was re-pinned: the audit fixtures and the C ABI and browser fixtures did not move.
The V8 spill gate needs Node 22.23.2 and was not run. The engine loom leg did not build at that point; #1503's Root Amendment 2 (`6ae468ceb`) closed it.
- **Executor-failure path counter (batch NIT 3, recorded).** On the #1502 D7 path a value that a setter
  refused (a structural defect only) has already been passed by `CellReader`. The counter that counts
  it, `live_block_errors`, is visible only through `PlanQueries`; neither host exposes it. See #1312's
  blocker follow-ups for the owner.
