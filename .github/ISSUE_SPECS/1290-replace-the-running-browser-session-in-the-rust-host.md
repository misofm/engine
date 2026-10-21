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
- gate 2: `a_live_edit_is_part_of_the_base_the_successor_is_prepared_from` (deleted by TA4) (the ramp is in flight
  at the adoption block)
- gate 3: `a_malformed_transaction_is_refused_and_changes_nothing`,
  `a_rate_or_quantum_change_is_refused_as_a_required_reprepare`,
  `a_transaction_over_the_memory_budget_is_refused_and_changes_nothing`
- gate 5: `a_rebuild_swaps_with_a_spectrum_capture_or_a_collection_configured` (deleted by the
  host-web read cut; held by `crates/host-core/tests/browser_successor.rs`
  `every_branch_swaps_in_without_a_gap`)
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

## Amendment (controls coordinator, batch 12, 2026-10-15): (d+e)

Checked against `ea336c039`.

- **Where things are.** #1381b pairs the meter consumers and the observation readers in the swap
  block (`RenderCompanions::adopt_predecessor`, `hosts/host-web/src/lib.rs:5190`). `adopt_shape`
  (`:1562`) moves the meter pending windows and the tap state. A swap that carries a meter keeps
  `meter_generation`. What is not done yet:
  - The published frame does not grow or shrink with the plan. `meter_frame` is sized at boot
    (`:1450`, `:6054`), and `poll_meters` publishes nothing when the strip count differs (`:3411`).
    The worklet cuts fixed views and builds a fixed `meterMessage` at boot from the header's counts
    (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:728-747`).
  - The successor's spectrum capture is not paired (#1395 is open).
  - #1381's gate 3, the frame with a changed strip count (`track_count` 3, the shifted strip's word,
    `master_track_plus_one` 3), was left to (d) by #1381b.
- **D7.** The batch 10 amendment still holds: meters and taps pair in the swap block, and spectrum
  pairs at promotion through #1395's `SpectrumService` method.
- **Meter frame growth, options.** `process()` must not cut a view or allocate (#1562a record).
  - (1) Size the frame and the worklet's views for a boot maximum of strips. The `3(T + S) + 3`
    layout is packed by count, so the view offsets still move with `T`. This needs a new layout at
    capacity offsets, which changes the ABI rows and `sdk/src/core/boundary.ts`, and every post
    clones the capacity.
  - (2) The frame goes with the plan's tables. It is allocated in `build_plan_shape` beside the
    per-meter scratch and exchanged in `adopt_shape` (a pointer exchange). The message handler that
    commits the rebuild also cuts the successor's views and builds its `meterMessage` in advance.
    `process()` changes to them by reference at the first poll after adoption.
    `rebindMemoryViews` also cuts the pending views again. The layout and the ABI rows do not
    change, except for an export that gives the pending frame pointer, if one is necessary.
  - **Recommendation: (2).** It is the #1290a `PlanShape` pattern, and it keeps exact sizes.
- **A handle shift must not lose a carried window.** The #1381b fold records this: a snapshot
  published before the swap keeps the old meter handle. When the handles shift (an added strip
  sorts first), the poll's handle check drops the snapshot and counts it as loss. Gate 7 polls after
  every block, so gate 7 does not see this. (d) matches a carried pending snapshot by its carried
  slot, and adds a gate: gate 7's script with a poll every second block shows no counted loss.
- **Split, so lane W does not wait for lane K:**
  - **(d1) meters.** Option (2), #1381's gate 3 with a changed strip count, the handle-shift gate,
    and gate 7's meter part. This has no lane K dependency.
  - **(d2) spectrum.** Gate 7's spectrum part: at promotion, take the successor's capture out of
    the adopted plan's attachment and call #1395's method with the pairing list kept beside the
    `PendingShape`. Dependencies: #1395a and #1395b.
- **(e) needs no code.** Gate 9's native part is already met by (a):
  - (a)'s gates run on one thread, so the apply runs between render calls, as the message handler
    runs it.
  - `blocking_rebuilds() == 1` is asserted at `hosts/host-web/src/tests.rs:6150` and
    `hosts/host-web/tests/structural_apply_realtime.rs:393`.
  - The browser legs are #1295's.
  - Record (e) as met when (d2) closes.
- **Restarted and removed effect owners.** A subscription on an owner that restarts or is removed
  is #1584 D6a and D6b, not (d). (d)'s gate 7 has no effect.
- **No edit under `crates/engine/src/realtime`.**
- **Order in lane W:** (d1), #1584 D6b, (d2), #1562b-W. The four slices edit `hosts/host-web/src/lib.rs`
  and `src/tests.rs` in that order. (d1) and #1562b-W also edit the worklet JS and the ABI layout
  mirrors, in that order.

## Ruling (root, 2026-10-15): meter frame growth, (d1)

The meter frame travels with the `PlanShape`; its views are cut in the message handler that
commits (control side, counted on a non-isolated page), and `process()` switches to them by
reference at the swap block. `process()` never cuts a view or allocates. Gate: a strip-count change
publishes a correctly shaped frame on the first block after adoption, with zero render
allocations. (d1) lands in batch 12a; (d2) in batch 12b after #1395.

## Decision record: (d1) meters (batch 12, lane W)

**Shipped.** A strip-count change no longer leaves the browser publishing nothing. The meter frame and
its header travel with the `PlanShape` (root's ruling); the page cuts its views over the successor's
frame in the message handler that commits, and `process()` switches to them by reference.

- **Frame and header in the shape.** `PlanShape` owns `meter_frame` (`3(T + S) + 3` words, built by
  `build_plan_shape`), `meter_header` (counts, master index and frame address from the plan; the poll's
  stream fields at run time) and `meter_carried_from`. `ReadyOwnership` lost its own frame and header.
  `adopt_shape` copies the stream fields (`windows`, `first/end_sample`, `sequence`, `master_gr_present`,
  `reserved[0..1]`) into the adopted header: the same stream continues in a frame of the new size. A
  pointer exchange and scalar writes; nothing allocates in the swap block.
- **`WebMeterHeader` 72 -> 80 bytes, nothing moved.** `reserved_pad` (68) became `retired`; appended
  `frame_pointer` (72) and `successor_pointer` (76). A commit writes the newest candidate's header
  address into the rendering plan's `successor_pointer` (`run_command`); the adopting block marks the
  header it leaves `retired = 1`, names the adopted header in its `successor_pointer`, and points the
  adopted header at any candidate still waiting. ABI mirrors regenerated (`abi_layout.rs`,
  `sdk/assets`, `sdk/src/generated/abi.ts`, `check-abi-layout-v1.py`, its self-test fixture, the
  direct oracle, `docs/BUILTINS_AND_METERING_V1.md`).
- **Worklet.** The views are per meter set (`meterSet`, `pendingMeterSet`), cut by `cutMeterSet` in
  `syncMeterViews`, which every message handler runs last (and `rebindMemoryViews` re-cuts both sets
  on a memory growth). `postMeterFrame` reads the active header's `retired` word; if set it switches
  `meterSet = pendingMeterSet` (a reference assignment) before the poll; a retired header with no set
  cut for the header it names is sticky `RESULT_INTERNAL` (cannot happen in single mode). A lease
  released across a swap is caught by `syncMeterViews`, which compares the host's current header
  address with the page's. The pinned post spelling in `check-web-audioworklet.sh` and its mutation in
  `test-web-audioworklet.sh` follow (`this.port.postMessage(set.message)`: the set's message is
  allocated with its views, in the handler).
- **Handle shift.** A meter's handle is its position, so a strip added or removed in front shifts the
  survivors' handles. The hook records both handles per carried meter (`CarriedReader`), the shape keeps
  the predecessor's in `meter_carried_from[successor]`, and the poll accepts a snapshot named by either.
  A queue is one strip's, so the second handle names the same strip (by carry identity, not position).
  Not kept: a window two swaps old still names a handle the table no longer holds and is dropped with a
  count; it needs a second swap between two polls.
- **Counted loss kept.** A window the predecessor closed before an added strip existed cannot be
  described by the successor's frame (that strip has no data for it): it is dropped with a count by the
  poll that finds it, as before. A removed strip's window is published without it (gate below).
- **Charging.** The frame was an uncharged allocation at boot. It is now a row of the shape
  (`ShapeRows::frame_bytes`, `carried_bytes`): the boot adds it to the bridge rows, a replacement to its
  plan row. `observation_staging_resource_is_exact_for_the_taps_off_and_on_modes` gained the frame
  term. Boot high-water growth 20,447,232 bytes (before: 20,447,232); `check-browser-expected-resources`
  green with no re-pin.

**Departures.** None from root's ruling. Mechanical edits outside lane W's own files, listed for the
coordinator: `tools/realtime-policy/render-thread-exports.json` and `src/policy.rs` (the handler now
calls `meter_header_ptr` after construction: one table row and one named unwrapped export),
`scripts/check-web-audioworklet.sh` and `scripts/test-web-audioworklet.sh` (the pinned post), the ABI
mirrors named above.

**Gates -> tests.**
- Root's gate (a strip-count change publishes a correctly shaped frame on the first block after
  adoption, zero render allocations): `tests/structural_apply_realtime.rs`
  `a_strip_count_change_publishes_the_successors_frame_on_the_first_block_after_adoption` (render and
  `meter_poll` under the render-locked counters, adoption block included; every frame after it equals a
  fresh boot of the committed model for the same window); `structural_apply::
  a_strip_count_change_publishes_the_successors_frame_from_the_adoption_block` (the same in the lib, with
  the retired/successor words).
- #1381 gate 3 (changed strip count, `track_count` 10, shifted words): the two tests above.
- Handle shift: `a_window_closed_before_a_handle_shift_is_published_by_its_carried_strip` (remove the
  first strip, poll every second block so the window is polled after the swap: published, sequence
  continues, no counted loss; red when the carried-from acceptance is disabled, checked).
  `an_aligned_poll_every_second_block_loses_nothing_across_an_added_strip` is gate 7's cadence variant.
- Counted loss: `windows_an_added_strip_cannot_describe_are_counted_and_the_next_frame_is_the_successors`
  (replaces `meter_windows_a_strip_count_change_leaves_unpublished_are_counted`);
  `meter_delivery_carries_through_a_swap_that_keeps_the_strips` (first half of the old
  `..._never_publishes_a_misshaped_frame`).
- Worklet: three blocks in `scripts/test-web-audioworklet.mjs` (views cut in the handler and switched by
  reference at the adopting block, sequence continues; lease released across a swap; retired header with
  no pending set is sticky).

**Limits found.** (1) The shipped module cannot run a rebuild until #1562b, so the real-wasm pointer
chain (`retired`, `successor_pointer`) is covered by the hermetic worklet fake and, on native, by
equality with `address_word` (which is `0` for a 64-bit address); the chromium leg proves the boot
`frame_pointer` path with real addresses. (2) The SDK's `meterProjection` still checks a frame against
its own session map, so after a structural apply it refuses the successor's frame until the SDK map
follows the apply (#1296's); the engine side is complete.

## Fold (batch 12a review)

- **Pointer checks at full width.** The native check of `successor_pointer` compared two
  `address_word` results, and `address_word` is `0` for any address above 4 GiB, so on a 64-bit
  host both sides were `0` and the check was always true (the "Limits found" note above recorded
  the weakness and left it). `PlanShape` now keeps the full-width address beside the header word
  (`name_successor` sets both where the commit and `adopt_shape` set the word; test builds only),
  and the gate `a_strip_count_change_publishes_the_successors_frame_from_the_adoption_block`
  compares that address with the candidate's and the adopted plan's header addresses
  (`AudioWorkletEngineHost::meter_successor_address`). Quick mutation, reverted: with the
  address not recorded the gate fails (`assertion left != right`, tests.rs:6777).
- **Doc fixes.** `miso_engine_web_v1_meter_header_ptr` no longer says "stable" (the address changes
  at each swap, each plan carrying its own header); the browser oracle's header size comment says
  80 bytes.

## Decision record: (d2) spectrum (batch 12, lane W)

**Shipped.** A browser rebuild keeps a running spectrum stream, an armed one-shot and a collection's
selection. At a plan's promotion the host calls `SpectrumService::adopt_successor(captures, pairing,
carried)` with the plan's own pairing list and the engine's real carry outcome.

- **Where the successor's captures wait (departure from the text).** The spec said to take them out of
  the adopted plan's attachment. The attachment is the render half's once the plan is published, and
  two adoptions before one service step would leave the first plan's captures unreachable. The
  preparer now leaves a plan's captures and `inventory.spectrum_pairing()` in the `BootRecord`, beside
  the plan's tables (`SpectrumHandover`); boot moves them into the service, and a command that
  publishes a replacement keeps them in the plan's `PendingShape`. `RenderCompanions` lost its
  `spectrum_capture` field (and `has_spectrum_capture`, `take_spectrum_capture`). The pairing is
  charged to the plan's rows with its tables.
- **Carry outcome.** The adoption block reads `RealtimePlanOwner::carried_count()` and compares it with
  the count at the previous adoption (`carried_swaps_seen`): a swap carried exactly when it advanced.
  One `u64` load and compare in the swap block; `PendingShape::adoption` records `Some(carried)`.
  The block that adopted the plan marks it; a candidate a later one superseded keeps `None` and its
  captures drop with the entry.
- **Promotion.** `ReadyOwnership::promote_spectrum`, run first in `reclaim_shapes` (the service step
  and the start of every command), installs each adopted plan's captures in commit order and drops
  the displaced captures there, on the control side. Spectrum exports run in render-locked windows,
  so promotion is deliberately not called from them.
- **Lane K.** #1618 adds a fourth argument, `analysis: Option<&mut SpectrumAnalysis>`, to
  `adopt_successor`. The browser holds no analysis state (the worklet passes `None`; the caller of
  `start_spectrum_stream` owns it), so the call in `promote_spectrum` passes `None` at the merge.

**Gates -> tests** (`hosts/host-web/src/tests.rs`, `structural_apply`):
- gate 7, stream: `a_running_spectrum_stream_continues_across_a_rebuild` (an added track sorts first
  and shifts every index; the stream is read after every block and equals a fresh boot of the
  committed model window for window; one capture epoch, no `Failed`/`Gap`; service step right after
  the adoption block, and 18 blocks later)
- gate 7, collection: `a_collection_selection_and_its_stream_continue_across_a_rebuild` (second entry
  selected and streamed) (deleted by the host-web read cut; held by
  `crates/host-core/tests/successor_swap.rs`
  `spectrum_carry::a_reordered_collection_pairs_by_key_and_its_reader_keeps_receiving`)
- armed one-shot: `an_armed_one_shot_completes_across_a_rebuild` (window spans the swap, equal to the
  committed model's) (deleted by the host-web read cut; held by
  `crates/host-core/tests/successor_swap.rs` `spectrum_carry::an_armed_one_shot_completes_across_the_swap`)
- `web_runtime_preparer_attaches_the_companions_on_every_branch` now reads the capture kind from the
  record.
- Red check: forcing `swap_carried = false` turns the stream, collection and one-shot tests red.

**Limits found.**
1. With the carried arm, promotion is not observable: the service keeps the same reader whether it
   holds the predecessor's capture or the paired successor's shell. Only a swap that did not carry
   distinguishes them, and the browser cannot make one yet: the only natural refusal is the
   identity mismatch of a successor of a candidate that never rendered (supersession, #1310), and
   a second command while a candidate waits is `Backpressure` today. No test covers the not-carried
   arm on the browser; `a_refused_hand_over_installs_the_fresh_capture` (host-core) covers
   `adopt_successor`'s side. Revisit when #1310 lands: add the two-rebuilds-before-one-block case.
2. A removed spectrum-observed strip: not tested here (#1593 keeps the taps).
3. Gate 8 (render-locked allocations) is unchanged; the adoption block adds one load and compare
   (`realtime-policy` ok).

## Fold (batch 12b review)

- **m1/m7.** The batch 12 note above says the browser passes `None` to `adopt_successor` in
  `promote_spectrum`. That left the analysis of an instance that owns both halves (the ffi staging's
  `SpectrumAnalysis`) active after a promotion that does not keep the stream: `spectrum_stream_reset`
  answered ok, and `spectrum_stream_analysis` adopted the stale metadata record again and re-reported
  the last window. `service`, `control_command`, `run_command`, `reclaim_shapes` and
  `promote_spectrum` now take `Option<&mut SpectrumAnalysis>`. The two exports lend the staging's
  analysis through `with_promotion_analysis` (`try_borrow_mut`; `None` when the staging is borrowed,
  so the audio thread's own instance, which keeps none, is unchanged). When the promotion ended the
  analysis, the export also writes the stream record from the service and drops the staged window, as
  `spectrum_stream_stop` does; without that the record still says `Ready`.
- **Gate:** `ffi::spectrum_ffi_tests::a_promotion_that_drops_the_stream_ends_the_analysis`, run for
  both exports. The swap carries, and its pairing is emptied so it does not name the capture (the
  host cannot make a refused hand-over yet; limit 1 above stands for that arm). After the promotion
  the analysis is not `stream_active`, `spectrum_stream_reset` answers a wrong state, and
  `spectrum_stream_analysis` does not answer ok. Red with the exports passing `None`.
- **m6.** `the_headers_structure_epoch_is_the_control_lists` now reads the epoch from a real
  `CONTROL_LIST_GET` page through the control entry, after a rebuild and after a live commit that
  moves the epoch (a VCA added). It is red when `stamp_structure_epoch` runs for rebuilds only (the
  live stamp removed): the header keeps 2 while the page reads 3.
