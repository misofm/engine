# Replace the running browser session in the Rust host

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-11, D15-17).
Slice B2 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

Rewritten for decision 15. The old body replaced the whole document with
`replace_session(document)` on the audio thread. Its live values carried only through B3/B3b's
three-way merge. Decision 15 supersedes that design:
- D15-11: one edit API, the transaction;
- D15-10: preparation in the Worker;
- D15-17: asynchronous completion.

*Keep browser live strip state across a session replacement* (#1291) and *Keep browser live
effect edits across a session replacement* (#1292) close as not planned. The committed model
replaces their merge.

## Product outcome

The browser engine's Rust host applies a structural transaction while it plays. The Worker-side
control half validates the transaction, classifies it as `rebuild`, and prepares the successor
from the committed model. It commits and returns `{revision, path: "rebuild"}` at once, without
waiting for render. The render half (the worklet) adopts the successor at a block boundary, with
the C ABI's guarantees:
- unchanged sources keep their rings;
- unchanged nodes keep their state;
- the clock continues;
- a refused transaction leaves the running engine and the model untouched.

The watermark reports when the revision is in effect. On a page that is not isolated, the same
call runs the blocking rebuild in the worklet's message handler, never inside `process()`. It is
reported and counted. This slice is the Rust host, tested natively. The Wasm export, worklet wiring
and SDK are the rewritten slices #1293-#1297.

## Context

- `AudioWorkletEngineHost` (`hosts/host-web/src/lib.rs:1990`) boots once
  (`boot_with_spectrum_config`, `:2043`). Boot runs:
  - the document size and parse-projection budgets;
  - `compile_host_model`;
  - the rate and quantum shape check;
  - the VCA bound;
  - then `compile_ready` (`:6617`). It prepares through one of three branches: a single
    spectrum capture, a spectrum collection, or no spectrum through
    `prepare_host_runtime_with_selected_meters_between_render_calls` (`:6644-6681`).

  The host owns one `ReadyOwnership` (`:1509`) and a `host_generation` (`:2004`). Prepared
  companions are bound to that generation and address strips by index.
- `render_next` (`:3212`) renders `ready.host.plan` directly, with `RenderTime { absolute_sample:
  status.next_absolute_sample }` (`:3234-3239`). It has no plan exchange.
- **The C ABI's shape, which this slice reuses.**
  - The control thread moves the persisting source producers into the candidate at commit
    (`crates/control-plane/src/control.rs:1146-1155`). In the browser the producers have one
    owner, the Worker: *Move browser source submission and seeks into the Worker* (#1387) puts the
    `SourceControlSet` there, and #1381 D1-D2 puts it inside the Worker's `SessionState`, which
    routes every source call.
  - It publishes through `PlanPublisher::reserve_replacement`
    (`crates/engine/src/realtime/plan_exchange.rs:265`).
  - Render adopts in `RealtimePlanOwner::enter_block` (`:375`). That step continues the clock,
    runs `carry_from` (`crates/engine/src/realtime/plan.rs:917`) and sends the old plan to the
    retirement queue. The control side reclaims it with `PlanRetirer::try_reclaim`
    (`crates/engine/src/realtime/plan_exchange.rs:512`).
- **Successor preparation.**
  - `SuccessorBase { inventory, committed }` (`crates/host-core/src/prepare.rs:641`).
  - `prepare_host_runtime_with_live_controls_successor` (`:926`).
  - `SourceControlSet::adopt_persisting` (`crates/host-core/src/source.rs:286`).
  - No successor entry exists yet for the two spectrum branches, and the no-spectrum branch
    (`prepare_host_runtime_with_selected_meters_between_render_calls`, `prepare.rs:1051`) has no
    concurrent form. *Prepare every browser preparation branch concurrently, as a successor too,
    in host-core* (#1401) adds all of them.
- **Cost.** A 64-track boot is 23.9 ms (V8 p50) against a 2.667 ms quantum (round-1 C4;
  *Measure a session rebuild on the browser's audio thread*, #1289, closed). In Worker mode this
  cost leaves the audio thread. In `single` mode it is the blocking rebuild that D15-10 accepts,
  reported and counted.

## Decisions frozen for this slice

- **D1. Entry point.** The control half's `apply(transaction) -> Result<{revision, path},
  refusal>` is `control_plane::SessionState`'s transaction apply (`crates/control-plane`,
  #1309). It runs in the Worker on the `SessionState` that #1381 constructs. It prepares through
  the browser's `RuntimePreparer` (*Prepare through an adapter-supplied preparer in the
  control-plane crate*, #1400).
  - A `rebuild` delta prepares through #1401's successor entry for the boot's branch: the
    single-capture or collection spectrum successor, or the concurrent selected-meter successor
    (`prepare_host_runtime_with_selected_meters_successor`).
  - These entries are concurrent; none is a `_between_render_calls` entry. The
    producers live in the Worker, and render runs at the same time.
  - The current spectrum configuration is kept. A transaction that removes the strip a spectrum
    capture observes is refused as `web.apply.spectrum_target`.
  - A change of rate or quantum is refused with `RESULT_REPREPARE_REQUIRED`.
  - Any refusal returns before commit and changes nothing.
- **D2. Base.** The successor is prepared from the committed model, which includes every live edit
  already committed (D15-7, P1.4). `SuccessorBase.committed` is the committed model of the
  predecessor plan plus every record pushed to it. The old D4 rule ("an owner that received a
  live record is not carried") is superseded by D15-7.
- **D3. Budget.** The projection charges the running plan, every unadopted candidate and the new
  plan together. Each carried ring counts once. The parse transient is included. The total is
  checked against `maximum_memory_bytes`.
- **D4. Swap.**
  - At commit, the control half:
    1. moves the persisting producers into the candidate (`adopt_persisting`);
    2. publishes the candidate through the reserved replacement;
    3. advances `host_generation`, so a companion bound to the old strip indices is refused.
  - The render half adopts the candidate at block entry through `RealtimePlanOwner`, and the
    Worker's service loop reclaims the retired plan. Both are wired by *Swap and retire browser
    plans through the Worker's service loop* (#1381). The carry is stream A's;
    this slice adds nothing to it.
  - A second structural transaction while a candidate is unadopted supersedes it by
    compare-and-swap (#1310). The displaced revision completes as `superseded`.
- **D5. Completion.** `apply` never waits for render.
  - The revision is pending until #1314's watermark covers it. The browser reads it through
    *Publish the applied-revision watermark in the browser status* (#1349). The watermark's
    first-sample-in-effect is the block at which `enter_block` returned `Applied` for that
    candidate.
  - Live edits submitted while a candidate is pending are written to the newest candidate's lanes
    (D15-17).
- **D6. Single mode.** With no Worker, the same `apply` runs in the worklet's control handler, in
  its message handler, never inside `process()` and outside the render-locked window. The same
  render half adopts at the next block.
  - Path stays `rebuild`.
  - Each such apply increments the host status's saturating `blockingRebuilds` counter, and its
    allocations and frees count in `singleModeControlAllocations` (both #1332 D1). Decision 15
    accepts this, counted and reported, for a non-isolated page only (D15-10). The render-locked
    allocation count stays exactly 0.
  - The service step runs from the same handler on #1381 D6's service message, never inside
    `process()`, so the retired plan is freed there.
- **D7. Observation.** Meter and effect observation state of unchanged owners carry across the
  swap as *Carry meter and effect observation state across a plan swap* (#1327) specifies, and
  spectrum capture state as *Carry spectrum capture state across a plan swap* (#1395) specifies
  (D15-14). Their readers live in the plan's attachment, and #1381 D3's hook pairs them in the
  swap block. This slice calls no adoption or pairing function of its own. The meter lease
  carries. Gate 7 holds the browser-visible result.

## Deliverables

1. D1-D6 in `hosts/host-web/src/lib.rs`: the control half's `apply` with the browser's
   successor preparation, publication, and the `blockingRebuilds` increment.
2. Native tests in `hosts/host-web/src/tests.rs`, and the integration test binary
   `hosts/host-web/tests/structural_apply_realtime.rs` (gate 8).

## Authorized paths

- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/structural_apply_realtime.rs` (new), `hosts/host-web/Cargo.toml`
  (dev-dependency `bench-support` only, if no earlier slice added it)

## Non-goals

- No Wasm export, no worklet or SDK change. The rewritten #1293-#1297 cover those.
- No document convenience. Diffing a document into one transaction, `apply_document` and the
  `replace` export are *Diff a replacement document against the committed model and export
  replace from the browser engine module* (#1386), which calls this slice's `apply`.
- No SDK surface. `replaceSession` and `engine.apply` are *Apply session transactions from the
  browser SDK* (#1296), over the exports of *Export transaction apply and anchored seek from the
  browser engine module* (#1293) and #1386.
- No transitions of its own. Fade-in, duck-swap and two-phase removal come from #1288, #1324 and
  #1325.
- No warm-successor deadline check: *Check the warm-successor deadline in the browser Worker's
  service loop and report its outcome* (#1361).

## Hazards

- **The browser folds meters as one group.** It publishes a meter frame only when every strip's
  snapshot has the same window sequence and bounds (`hosts/host-web/src/lib.rs:3485-3500`). An
  added strip's meter stays on the carried meters' grid only through #1327 D6. Gate 7 checks it
  with an added track; a red gate here for that reason is a #1327 defect, not a reason to reset
  the meter stream.

## Objective gates

1. **Gap-free acceptance.** Native host-web test. The control half and the render half run on two
   threads, with live controls on.
   - Boot A: nine tracks with enabled high-pass and low-pass filters and non-centre pans.
   - Render 6 blocks, feeding sources through `submit_source`.
   - Apply a transaction that adds a muted track whose ID sorts first.
   - Render 6 more blocks.
   - The response is `{revision: 1, path: "rebuild"}` and returns before the render thread
     adopts.
   - From the adoption block on, every block equals a fresh boot of the committed model fed the
     same PCM from frame 0.
   - The watermark reaches revision 1 with outcome `exact` and the adoption block's first sample.
   - A response capture after the swap is stamped with the continued clock.
2. **Live edits are part of the base.** Apply a live fader edit to track 3, then a structural
   transaction. After adoption, track 3 renders at the edited fader value. Its ramp state carries
   (#1277). The output equals a fresh boot of the committed model.
3. **Refusals change nothing.** These are each refused with their typed result:
   - a malformed transaction;
   - a rate change;
   - a transaction over the memory budget;
   - the removal of a spectrum-observed strip.

   Revision and model are unchanged, no candidate is published, and the next blocks are
   bit-identical to a run without the call.
4. **Supersession.** Apply two structural transactions before the render thread enters a block.
   Only the second is adopted. Revision 1 completes as `superseded`, revision 2 as `exact`. No
   BACKPRESSURE is returned.
5. **Each preparation branch.** Gate 1 passes with a single spectrum capture and with a spectrum
   collection configured.
6. **Companions.** A prepared companion bound before the apply is refused after it.
7. **Meters and spectrum continue.** In gate 1's script, with one meter per strip and a single
   continuous spectrum capture on A's track 3, poll the browser host's meter delivery and
   spectrum read after every block. Across the structural apply and its adoption block, `meter_generation` does not advance, the meter header
   reports no reset, every carried strip's meter continues its window sequence with no counted
   loss, the added track's meter joins the published frame from the adoption block's window on,
   and the spectrum read returns no `Failed` epoch and no sequence gap.
8. **Realtime.** Integration test binary `hosts/host-web/tests/structural_apply_realtime.rs`.
   Decision 15 rules that host-web's native allocation-count gates live in an integration binary,
   never in the lib's unit tests, whose test build already registers a `#[global_allocator]`
   (`hosts/host-web/src/ffi.rs:4540`, in `live_response_ffi_tests`). It links
   `bench_support::alloc`, calls `assert_installed()` first, and runs the scripts of gates 1, 2
   and 4 through the test-support halves (#1381). The render thread's thread-scoped counters read
   `allocations == 0 && frees == 0` around every render call, after warm-up. Every retired plan
   is dropped on the control thread.
9. **Single mode.** Gate 1 driven on one thread (apply between render calls, as the message
   handler runs it) gives the same blocks and the same response. `blockingRebuilds` is exactly 1.
   The browser legs of single mode (`singleModeControlAllocations` growing across the apply, the
   render-locked count staying 0) are *Qualify a structural browser edit in real browsers*
   (#1295)'s, not this slice's.
10. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support` (runs gate 8's binary)
   - `bash scripts/check-web-audioworklet.sh`
   - `bash scripts/test-web-audioworklet.sh`
   - the browser legs: `npm run qualify -- --artifacts ... --sdk-root ... --browser <b> --check-matrix --self-test-mutations`
     in `hosts/host-web/qualification`, for chromium, firefox and webkit
   - `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`,
     `bash scripts/check-workspace-policy.sh`
   - The shipped artifact changes: report its digest (`artifact-identity`).

## Test value

- Gate 1: turns red if the apply reboots internally (fresh rings or state), if the successor's
  clock restarts at 0, or if `apply` blocks until adoption.
- Gate 2: turns red if the successor is prepared from the boot document instead of the committed
  model, which would lose live edits. That is the defect B3/B3b's merge existed to patch.
- Gate 3: turns red if a refusal runs after the producers moved or after publication.
- Gate 4: turns red if a pending candidate still causes BACKPRESSURE, or if a displaced revision
  never completes.
- Gate 6: turns red if a companion addressed by an old strip index reaches the new plan.
- Gate 7: turns red if a structural apply restarts the browser's meter or spectrum stream: a
  reader left on the retired plan's producer, delivery state not carried with the companions, or
  a carry that misses an unchanged strip after the added track shifts every index. No host-core
  test sees the browser's delivery state.
- Gate 8: turns red if the render half drops a retired plan, or allocates on adoption.
- Gate 9: turns red if single mode takes a different code path that diverges in bits, or if its
  rebuild is not counted.
- Superseded tests: none. The old body's tests were never written.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
- *Report each transaction's edit path in its response* (#1313).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348).
- *Publish the applied-revision watermark in the browser status* (#1349).
- *Give every browser plan live strip fader and mute lanes* (#1326): the transitions on the
  browser need a live strip mute.
- *Carry fader, mute and pan ramps across a plan swap* (#1277), for gates 1 and 2.
- *Carry meter and effect observation state across a plan swap* (#1327), for D7.
- *Carry spectrum capture state across a plan swap* (#1395), for D7's spectrum carry.
- *Move browser source submission and seeks into the Worker* (#1387): the producers' owner.
- *Prepare every browser preparation branch concurrently, as a successor too, in host-core* (#1401).
- *Prepare through an adapter-supplied preparer in the control-plane crate* (#1400).

#1382 removed from the dependencies by decision 17 (root, 2026-10-13): #1562 (formerly #1382, amended) depends on this issue's rebuild arm.

## Amendment (decision 17, root 2026-10-13)

**Single instance first.** Read "Worker" as "the control half": the one instance's message
handler in `single` mode (first), and the Worker after #1332. `blockingRebuilds` is defined here
(D6), not in #1332.

- D1's `web.apply.spectrum_target` refusal and gate 3's fourth case are deleted (root Q4): #1593
  keeps the session's spectrum taps on every plan it prepares.
- The structural apply is `SessionState::command` through #1562a's entry. Gate 2's live fader edit
  is a `SET_CONTROL`.
- Split, all in this issue:
  - **(a)** the rebuild apply: gates 1, 2, 3 (cases 1-3), 5, 6 and 8. Dependencies: as filed,
    without #1332, #1387, #1310, #1327 and #1395, with #1381a for #1381 and with #1562a.
  - **(d+e)** gate 7 and single-mode gate 9. Dependencies: (a), #1381b, #1327, #1395.
  - **(b)** gate 4 (supersession). Dependencies: (a), #1310 (after stream B's #1344 and #1398).
  - (c), the spectrum refusal, is dropped (root Q4).
- #1332 and #1387 are removed from the dependencies.

## Amendment (controls coordinator, batch 10, 2026-10-14)

Checked against `12d4a4f9d`.

- **(a) is reachable now.** `SessionState::command` runs the rebuild arm through
  `WebRuntimePreparer` with a `SuccessorBase` (`hosts/host-web/src/lib.rs:4549-4660`, every boot
  branch has its successor entry), and `refuse_rebuild` stops it unless `allow_rebuild` is set
  (#1562a D9). (a)'s native gates drive the arm through the test-support switch
  (`structural_command_for_test`, `:1870-1890`); deleting the refusal stays #1562b's.
  `blockingRebuilds` (D6) does not exist yet; (a) adds it.
- **D4 step 3 and gate 6 are stale.** `host_generation` and the prepared companions bound to it are
  gone (#1565 deleted the prepared commands). Options: (1) delete D4 step 3 and gate 6; (2) restate
  gate 6 on what is still bound to boot-time indices: after the adoption block, the observation
  catalog exports (`miso_engine_web_v1_observation_*`, until #1590 deletes them) and the source
  table (`miso_engine_web_v1_source_count`, `_source_id`) describe the successor. Recommendation:
  (2), with its observation part void once #1590 lands.
- **D7.** Meter consumers and observation handles are in the plan's attachment and pair in the swap
  block (#1381b). Spectrum captures are in `SpectrumService` on the control side: (d) keeps the
  successor's prepared captures beside the candidate and calls #1395's `SpectrumService` pairing
  when the candidate is promoted (#1395's batch 10 amendment). Gate 7's spectrum part is (d)'s.
- **Effects.** Gates 1 and 2 as written have no effect, so (a) does not wait for the carry chain. A
  session with a console slot or an insert is bit-identical after the swap only with #1280a (and
  #1281, #1282 for inserts that change representation or render per node).
- **Dependencies.** (a): #1381a, #1562a (both landed). (d+e): (a), #1381b, #1327a, #1327b, #1395a,
  #1395b.
- **Anchors moved** in `hosts/host-web/src/lib.rs`: `AudioWorkletEngineHost` `:1598`,
  `boot_with_spectrum_config` `:1655`, `render_next` `:2904`, `compile_ready` `:4982`. #1349 is
  closed; the watermark is in the browser status (D5).

## Ruling (root, 2026-10-14): gate 6

Gate 6 is restated on what is still bound to boot-time indices: after adoption, the observation
catalog exports (until #1590) and the source table describe the successor. D4 step 3's prepared
commands no longer exist (#1565A) and leave the gate.

## Decision record: (a) the rebuild apply (batch 10)

**Shipped.** The browser host applies a structural transaction while it plays, through the existing
`SessionState::command` rebuild arm and `WebRuntimePreparer`'s successor entries. It is reachable
now only through the test-support switch `structural_command_for_test` (the refusal is #1562b's).

- **Tables travel with the plan.** `PlanShape` (the strip order, the dense effect table, the
  observation arm tables and the per-meter scratch) was ReadyOwnership's boot-time state. It is now
  built by one function (`build_plan_shape`) in the preparer, for the boot plan and for every
  successor. A command that publishes a rebuild keeps the successor's tables in a bounded
  `pending_shapes` list (room checked before the command, so an ack never meets a full list). The
  block that adopts the plan swaps them in place (`adopt_shape`, a pointer exchange, no indexing,
  no allocation); the service step drops the retired tables. The tables live in the host, not in the
  plan's attachment, because reaching an attachment is a `dyn Any` call and the render export pins
  zero indirect sites in `render_next`.
- **Gate 6 (restated).** After the adoption block the observation catalog and strip order describe
  the successor; the source table is the committed model's from the commit. An observation record
  (kinds 7 and 8) is `BACKPRESSURE` from the commit until the service step after the adoption block
  (its tables and the control plane's producers are one plan's again then); nothing is queued.
- **D3 (budget).** `ControlLimits.maximum_replacement_peak_bytes` (new; the C ABI sets `u64::MAX`)
  bounds the sum of both plans' graph, both models and both plans' source bytes (a carried ring
  once). The browser sets it after boot to its one budget less its exact bridge rows
  (`SessionState::set_replacement_peak_limit`). A successor's tables are charged to its plan row.
- **Rate or quantum change.** The plan exchange answered it as retryable `BACKPRESSURE`. The core
  now refuses it before preparing (`control.apply.reprepare_required`, `REPREPARE_REQUIRED`); the
  browser maps it to `RESULT_REPREPARE_REQUIRED`. This also fixes the C ABI's answer.
- **D6.** `AudioWorkletEngineHost::blocking_rebuilds()` (saturating `u32`), counted when a command
  publishes a replacement (`SessionState::newest_plan_epoch` advanced). No export and no status word
  carry it yet: the worklet status field is #1293-#1297's, so no ABI mirror changed.
- **Meters across a swap (until #1290d).** A swap restarts the meter delivery state (a new delivery
  epoch, `meter_generation` + 1). The published frame keeps its boot size (the worklet binds fixed
  views over it), so a swap that changes the strip count publishes nothing instead of a misshaped
  frame. Observation arm bits are not carried (#1327).

**Departures.**
- Gates 1, 2, 3, 5, 6 and 8 run on one thread (apply between render calls). No test-support
  control/render halves exist for two real threads; that form arrives with the Worker.
- Gate 8 uses the module's own `RenderLockedAllocator` and the export counters, as the sibling
  binaries do (the workspace has no `bench_support::alloc`; the spec's `assert_installed` form does
  not apply).
- D4 step 3 and the old gate 6 (prepared companions) are gone with #1565A.
- Gate 7 (meters, spectrum), gate 9 (single-mode browser legs) and gate 4 (supersession) are not
  in (a). Effects: the gate sessions have no console slot and no insert; an effect carries state
  across a swap only with #1280a.

**Gates -> tests** (`cargo test --locked -p host-web --features host-web/test-support`):
- gate 1: `tests::structural_apply::a_rebuild_swaps_without_a_gap`
- gate 2: `a_live_edit_is_part_of_the_base_the_successor_is_prepared_from` (the ramp is in flight
  at the adoption block)
- gate 3: `a_malformed_transaction_is_refused_and_changes_nothing`,
  `a_rate_or_quantum_change_is_refused_as_a_required_reprepare`,
  `a_transaction_over_the_memory_budget_is_refused_and_changes_nothing`
- gate 5: `a_rebuild_swaps_with_a_spectrum_capture_or_a_collection_configured`
- gate 6: `the_catalogs_describe_the_successor_from_the_adoption_block_on`
- meter interim: `meter_delivery_restarts_on_the_successor_and_never_publishes_a_misshaped_frame`
- gate 8: `tests/structural_apply_realtime.rs` (two tests); render-locked allocations 0 across every
  render call, the adoption block included; the retired plan and tables drop in the service step.

## Fold (batch 10 review)

Four findings of the batch 10 review, folded in `hosts/host-web` (plus two accessors of
`crates/control-plane`).

- **Observation records across a swap (MINOR-2).** Gate 6's `BACKPRESSURE` window is gone: the
  browser admits an observation record (kinds 7 and 8) between a rebuild's commit and its adoption
  block, as the control plane admits a live value, so no behaviour differs from the C ABI but the
  sequenced rebuild refusal. `ReadyOwnership::replacement_pending` and the `submit_commands` check
  are deleted. `admit_on_newest_plan` exchanges the waiting replacement's tables with the host's
  (a pointer swap, back before it returns; nothing allocates), and the control plane's
  `effect_producers` / `push_effect_observe` now address the newest plan's producers (the
  successor's while one waits, else the current plan's; `current_plan_epoch` had no other reader
  and is deleted). The record is queued on the successor's observation queue, drains at its
  adoption block, and arms the successor's tap (the arm bits and arm samples live in its own
  tables, so the plan that renders is untouched). `adopt_shape` marks the in-flight counts for the
  clearing pass, since a block that did not adopt cleared only the rendering plan's tables. Arm
  state is still not carried across a swap (#1327): a record admitted to the retiring plan before
  the commit, or to a candidate a later rebuild supersedes, ends with that plan.
- **Meter windows across a strip-count change (MINOR-3).** While the published frame cannot
  describe the plan (the strip count differs), `poll_meters` drains the master intervals it cannot
  publish and adds each to `meter_loss_count`; `adopt_shape` keeps the count across the restart and
  adds the windows the restart discards (the master intervals waiting and a half-assembled track
  group). The only counter that reports meter loss is the header's (`reserved[1]`), which a frame
  publishes, so the count reaches the page with the first frame after #1290d grows the frame; no
  status word was added.
- **Rebuild without tables after a commit (MINOR-6).** The preparer leaves the plan's tables in the
  record under a lock whose failure refuses the preparation (`web.apply.shape_unavailable`, before
  the commit); `run_command` no longer has a refusal after a commit (a committed rebuild always has
  its tables; the take is an `expect` of that invariant).
- **NIT-3.** `blocking_rebuilds()` stays; its doc names #1293-#1297 (the status field, #1332 D1)
  and #1295 (the first reader).

Gates -> tests: `structural_apply::the_catalogs_describe_the_successor_from_the_adoption_block_on`
(an observation submitted between the commit and the adoption block is admitted, queued on the
successor's full one-slot queue, leaves the rendering plan unarmed and arms the successor's
compressor at the adoption block); `meter_windows_a_strip_count_change_leaves_unpublished_are_counted`;
`a_rebuild_whose_tables_cannot_reach_the_host_is_refused_before_the_commit` (nothing committed).
