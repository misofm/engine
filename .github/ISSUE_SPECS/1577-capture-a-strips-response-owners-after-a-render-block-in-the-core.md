# Capture a strip's response owners after a render block in the core

Slice S20a of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13), first half of the planner's S20: the core capture. #1578 serves it on the protocol and
the C ABI. Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

The control plane can ask the render thread for one strip's response owners, and get them back
from the first successful block of the newest plan, with no allocation, lock or syscall on the
render thread. The control thread then composes the track response with #1576's function. This is
the one render-half step that both hosts use: the C ABI from #1578, the browser from #1579.

## Context

- **The plan belongs to the render thread.** `PlanState` (`crates/control-plane/src/plan.rs:62`)
  owns the `RealtimePlanOwner`; its `render` (`:253-275`) renders through `render_contiguous` and
  publishes the active epoch. The control thread never borrows the plan. The C ABI's render entry
  calls it (`crates/capi/src/ffi.rs:928`); after *Swap and retire browser plans through the
  Worker's service loop* (#1381) the browser worklet renders through it too.
- **A capture is valid only at a boundary.** `PreparedRenderPlan::copy_response_snapshot`
  (`crates/engine/src/realtime/plan.rs:845`) refuses after a failed render; #1576 D3 adds the
  owner-level `RealtimePlanOwner::copy_response_snapshot`.
- **An allocation-free sink exists.** `ResponseSnapshotCollector` (`crates/host-core/src/response.rs:227-358`)
  preallocates strings and fixed section arrays in `new` and copies without allocating;
  `finish` (allocating) runs on the control side.
- **Move-only queues exist.** `engine::realtime::bounded_spsc_move`
  (`crates/engine/src/realtime/spsc.rs:250`): a full push returns the value to its producer; a pop
  transfers it once.
- **Construction.** `compile_children` (`crates/control-plane/src/compile.rs:751`) makes the plan
  exchange (`:827`), the provider (`:854`) and `PlanState::new` (`:916`). A replacement plan's
  provider epoch is pushed at `crates/control-plane/src/control.rs:1314`. `SessionState::service`
  (`:1059-1065`) is the deferred-work step both hosts call.
- **IDs.** A `StableId` is at most 127 bytes (`crates/session/src/id.rs:10-14`); a response owner
  has at most `RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS` (6) sections per side
  (`crates/effect-contract/src/lib.rs:33`).

## Decisions frozen for this slice

- **D1. The job.** A new module `crates/host-core/src/response_capture.rs` holds
  `ResponseCaptureJob { tag: u64, plan_epoch: u64, strip_id: String, collector:
  ResponseSnapshotCollector, outcome: CaptureOutcome }` (fields private), with
  `CaptureOutcome::{Pending, Captured(RealtimeResponseSnapshot), Stale,
  Failed(ResponseSnapshotError)}`. The control side allocates every job. `strip_id` has capacity
  127; the collector is sized for `1 + the session's console slot count + the strip's insert
  count` owners, 6 sections and 127-byte identities, from the committed model.
- **D2. The channel.** `response_capture_channel() -> Result<(ResponseCaptureClient,
  ResponseCaptureServer), ResponseCaptureSetupError>` builds two `bounded_spsc_move` queues of
  capacity 1 of `Box<ResponseCaptureJob>`: requests (control to render) and replies (render to
  control). It is built once per session, at boot, on the control thread.
- **D3. The render step.** `ResponseCaptureServer::after_block(&mut self, owner: &mut
  RealtimePlanOwner)`, called by `PlanState::render` after `render_contiguous` returns `Ok` and
  never after an error. It holds at most one job:
  1. with no job held, it tries one pop (one atomic load when the queue is empty);
  2. if the active epoch is lower than the job's `plan_epoch`, it keeps the job;
  3. if the active epoch is higher, the outcome is `Stale`;
  4. if they are equal and the outcome is `Pending`, it runs `owner.copy_response_snapshot(strip,
     collector)` once and records `Captured` or `Failed`;
  5. a job whose outcome is not `Pending` is pushed to the replies; a full push keeps it for the
     next block (the job is never dropped and never captured twice on the render thread).
  The step sits in a `REALTIME_POLICY_BEGIN`/`END` region. It allocates, frees, locks and logs
  nothing; its cost is bounded by the collector's capacity, fixed when the control side armed it.
- **D4. The control side.** `ResponseCaptureClient` (held by `SessionControlProvider`, given at
  `try_new`) has:
  - `set_newest_plan_epoch(&mut self, epoch: u64)`: set at boot and whenever a replacement
    plan's epoch is pushed (`control.rs:1314`). The value never decreases (a debug assertion).
    No `control-plane` path withdraws a published candidate today (`PlanPublisher::withdraw`,
    `crates/engine/src/realtime/plan_exchange.rs:545`, has no caller there); a later slice that
    adds one must also re-arm a held capture;
  - `arm(&mut self, model: &SessionModel, strip_id: &str, grid: ResponsePreviewGrid, channels:
    ResponseChannels) -> Result<u64, ResponseCaptureRefusal>`: checks, in this order and before
    any write, an unknown strip (`UnknownStrip`), a capture already pending (`Pending`), the
    points and the grid at the session rate (`Grid`, the #1572 rules); then allocates the job with
    the next tag (from 1, per session) and the newest epoch, and pushes it. A full push is
    `Pending` and changes nothing;
  - `poll(&mut self, model: &SessionModel) -> Option<ResponseReady>`: pops one reply. `Captured`:
    `collector.finish` and #1576's `compose_track_response` with the armed grid and channels.
    `Stale`: if the strip is still in `model`, a new job (same tag, newest epoch, a new bound) is
    pushed and `poll` returns `None`; else the result is `StripRemoved`. `Failed(e)`: the mapped
    failure. `poll` returns each request's `ResponseReady` once;
  - `release(&mut self)`: ends the pending state. Until it is called, `arm` answers `Pending`, so
    one session has at most one capture in flight. In this slice `test_take_track_response` calls
    it; #1578 calls it when the host dequeues the `RESPONSE_READY` event.
  - `ResponseReady { tag: u64, strip_id: Box<str>, result: Result<TrackResponse,
    ResponseCaptureFailure> }`; `ResponseCaptureFailure::{StripRemoved, Unsupported, Capacity,
    Internal}` with codes `response.strip.removed`, `response.effect.unsupported`,
    `response.limit` and `response.capture` (`MissingTrack` maps to `StripRemoved`; an
    `UnsupportedProvider` composition to `Unsupported`).
- **D5. Wiring in control-plane.** `compile_children` builds the channel and passes the client
  to the provider and the server to `PlanState::new`. `SessionState::service` polls the client
  after `synchronize_plan_epochs` and keeps the result in a new `response_ready:
  Option<ResponseReady>` field; #1578 turns it into an event. The protocol controller gains one
  accessor that lends the provider mutably and the committed session immutably at the same time
  (`provider_and_session_mut`), because `poll` needs both. Under `test-support`, `SessionState`
  exposes `test_arm_track_response(strip_id, grid, channels)` and `test_take_track_response()`.

## Deliverables

1. D1-D4 in `crates/host-core/src/response_capture.rs`, the provider's client field and the methods that
   forward to it (`set_newest_plan_epoch`, `arm`, `poll`, `release`) in
   `crates/host-core/src/control_provider.rs`, exports in `crates/host-core/src/lib.rs`.
2. D3's call in `crates/control-plane/src/plan.rs`; D5 in `crates/control-plane/src/{compile.rs,
   control.rs}`; the accessor in `crates/protocol/src/controller.rs`.
3. The tests below.

## Authorized paths

- `crates/host-core/src/{response_capture.rs,response.rs,control_provider.rs,lib.rs}`.
- `crates/control-plane/src/{plan.rs,compile.rs,control.rs}`, `crates/control-plane/Cargo.toml`
  (dev-dependency `bench-support`), new `crates/control-plane/tests/response_capture.rs`.
- `crates/protocol/src/controller.rs` (the accessor only).

## Non-goals

- No protocol message, no event, no C header change (#1578).
- No browser change (#1579). The capture semantics stay applied (#779): no committed-model
  composition.

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`,
`cargo test --locked -p capi`, `cargo build --locked --release -p realtime-policy &&
./target/release/realtime_policy`.

All in `crates/control-plane/tests/response_capture.rs` (its own `ControlAdapter`,
`compile_children`, `PlanState::render` and `SessionState::service`), on a 48 kHz session with two
tracks, input filters, an EQ insert and an EQ console slot:

1. **One capture.** Arm track 1: before any render, `service` yields nothing. Render one block and
   call `service`: the result has tag 1 and equals `compose_track_response` of a capture that the
   test takes from the same `PlanState` (`test_owner_mut`, `crates/control-plane/src/plan.rs:221`)
   after the same block.
2. **Applied values.** Commit a live EQ band-gain edit, arm, render one block: the curve shows the
   new gain and `captured_sample` is the block's end.
3. **Newest plan.** Arm, then commit a structural edit that adds an insert to track 1, then render:
   the result includes the new insert. Arm on track 2, then commit an edit that removes track 2,
   then render twice with `service` between: the result is `StripRemoved`. The tag never changes
   for one request.
4. **Refusals change nothing.** A second `arm` while one is pending is `Pending`; an unknown strip
   is `UnknownStrip`; 4097 points is `Grid`. After each, the next armed capture still gets tag 2.
5. **Realtime.** The file links `bench_support::alloc` and calls `assert_installed()` first. The
   render thread's thread-scoped counters read `allocations == 0 && frees == 0` around each of
   200 render calls while the control thread arms a capture every 4 blocks and polls between them
   (statics warmed first). The realtime policy binary passes.

## Test value

- Gate 1 turns red if the render step copies other owners than the plan holds, or the control side
  composes them differently.
- Gate 2 turns red if the capture reads the committed model instead of the plan's applied targets.
- Gate 3 turns red if a capture is answered from a plan older than the newest structure, or never
  answered after its strip is removed.
- Gate 4 turns red if a refused request leaves a job, a tag or a pending flag behind.
- Gate 5 turns red if the render step allocates, frees or drops a job.

## Dependencies

- #1576 (the composer and the owner-level capture), #1572 (`ResponseChannels`, the grid rules).
- *Swap and retire browser plans through the Worker's service loop* (#1381) is not needed for the
  C ABI; it puts the browser on the same `PlanState::render` (used by #1579).
- `crates/control-plane`, `crates/protocol` and the render half are stream B's: rebase after
  stream B's open slices that touch `plan.rs`, `compile.rs` and `control.rs`.

## Supersedes and shrinks

- None.

## Decision and evidence record (attempt 1, 2026-10-24, against `main` 506196ce5)

- **Anchors moved.** `PlanState::render` `crates/control-plane/src/plan.rs:355`; owner capture
  `RealtimePlanOwner::copy_response_snapshot` `crates/engine/src/realtime/plan_exchange.rs:1031`;
  `compile_children` `compile.rs:1589`; the epoch push `control.rs:3952`; `service` `control.rs:2501`.
- **Root decisions applied (scope note R1-R6).** `after_block` runs at the end of `render`'s `Ok`
  path, after the epoch store. The client has three states (`Idle`, `InFlight`, `Delivered`);
  `release` is a no-op outside `Delivered`. One joint accessor,
  `ProtocolController::provider_and_session_mut`. Three test functions instead of five.
- **D4 correction.** The spec says no control-plane path withdraws a published candidate. That is
  wrong today: #1310 supersession withdraws one (`control.rs` `withdraw_pending_candidate`). The
  epoch rule holds anyway: a refused supersession republishes the candidate with its own epoch
  (`PlanPublisher::republish`), and a committed one publishes a newer epoch through the push that
  sets the newest epoch. A job armed at the withdrawn epoch then turns `Stale` and is armed again.
- **Realtime.** `after_block` and the collector's `ResponseSnapshotSink` impl are in
  `REALTIME_POLICY` regions; so is the client's one reply pop (the tool's rule for a `try_pop`
  outside the control-side allowlist). A held slot is filled through `Option::replace` and the
  `None` it displaces is forgotten, so no drop of a job is reachable on render.
- **Resources.** The channel's two queues are charged in `capi_resources`'s fixed aggregate rows
  (`host_core::response_capture_retained_bytes`); capi's completeness oracle saw the 544 bytes
  uncharged before. Attempt 2 (root ruling, the #1325a rule: a control-side heap owner that
  outlives a call is charged at the model's bounds): one armed job and one delivered result are
  charged in the same rows (`host_core::response_capture_ceiling_bytes`, through
  `control_plane::response_capture_bytes`): the strip with the most owners (input filters, every
  console slot, the most inserts any strip has) with three 127-byte identities and two
  six-section arrays per owner plus the strip ID, and a 4,096-point result (frequencies, left,
  right) with its members. The bound is the committed model's, not `maximum_effects`: the browser
  runs with `maximum_effects = u64::MAX`, so a cap bound cannot be represented there. It is
  52,076 bytes for the nine-track fixture; capi's reference ceiling moves 212,288 -> 275,584
  (measured 250,475, plus 10 %). capi's completeness oracle adds the row as a bound;
  `a_capture_at_the_bound_stays_within_its_charge` holds a capture at the bound within it.
  Attempt 3 (root ruling): the ceiling is taken at the session's owner high-water, the most
  owners one strip of any committed model of this session has had (`capture_owners_high_water`,
  the `observation_slots_high_water` precedent: set at boot, raised after each rebuild's commit,
  passed to every successor's preparation), so a job or result sized under a model a rebuild
  displaced (deferred adoption, a full retirement queue, a render finishing an older block) stays
  charged. `ResponseCaptureCeiling` is split into `job_bytes` and `result_bytes`; the result
  charges two strip IDs (`ResponseReady::strip_id` and `TrackResponse::strip_id`), and
  `compose_track_response` reserves its member table exactly. The nine-track fixture's row is
  52,203 bytes; capi's reference row measures 248,532 against its 275,584 ceiling after the merge
  of `main` at attempt 3, so the ceiling stays.
- **Browser call-graph gate.** `scripts/check-web-audioworklet.sh` fails on two pins, outside this
  issue's paths: `ResponseCaptureServer::after_block` is a new trap owner with 2 sites (the
  inlined SPSC `slots[...]` index of its pop and push, the class `enter_block` is admitted for),
  and `RealtimePlanOwner::copy_response_snapshot` is a new `call_indirect` site (1: the
  executor's `copy_response_snapshot` through `Box<dyn PreparedPlanExecutor>`). No allocation or
  drop glue is reachable. Root authorized both pins (2026-10-24, extra paths
  `scripts/check-web-audioworklet.sh` and `scripts/check-web-audioworklet-callgraph.py`):
  `--trap-owner 21ResponseCaptureServer11after_block=2` (the SPSC `slots[...]` index of
  `after_block`'s one pop and one push, `panic_bounds_check`, the same class as `enter_block`),
  and an `INDIRECT_SITES` row of 1 for `RealtimePlanOwner::copy_response_snapshot` (the capture
  copies the plan's snapshot through the executor trait object, once per armed job; no
  allocation, no drop). The two checks are not removed instead: they are the engine SPSC's own
  `slots[local]` index (`crates/engine/src/realtime/spsc.rs`), which every SPSC user inlines, so
  removing them is an engine change outside this issue.

- **Reachability (attempt 2 correction).** The keep branch (active epoch below the job's) is
  reachable: through the C ABI's cross-thread order, and on one thread through
  `SessionState::test_set_next_adoption(PlanAdoption::NoEarlierThan(..))`, which
  `capture_follows_newest_plan` now drives. Only the full-reply-push branch is unreachable: the
  channel holds one job, and the client pushes a request only when it holds no reply, so the
  reply queue is empty whenever render pushes.

### Test value (`crates/control-plane/tests/response_capture.rs`)

- `capture_follows_newest_plan` (gates 1, 3, 4): red if the render step captures from a plan older
  than the job's epoch (the keep branch: a rebuild adopted 4 blocks later, with no allocation on
  any block, the swap block included), if it captures before the block,
  if a replacement's epoch is not recorded, if a reply is dropped or delivered twice, or if a
  refused request leaves a tag or a pending state behind, or checks a pending capture before an
  unknown strip.
- `capture_reads_applied_values` (gate 2): red if the step reads the values held before the
  block's live snapshot instead of the applied targets after it.
- `render_step_allocates_nothing` (gate 5, the root comment): red if the render step, the
  collector's copy or the job's move allocates or frees on render, or a job is lost; a structural
  push between an arm and its render drives the stale path under the count.
- `a_capture_at_the_bound_stays_within_its_charge`: red if the job's charge leaves out a part of
  the job (the collector's table or its three identities per owner), or the result's leaves out a
  part of the result that persists across calls.
- `a_capture_held_across_a_shrinking_rebuild_stays_within_its_charge`: red if the charge follows
  the newest committed model alone, not the most owners any committed model has had.

Red first, each mutant restored by `git checkout`: step before the block (follows and applied red);
no newest-epoch record at the push (follows red); a refusal that takes a tag (follows red);
`Delivered` read as `Idle` (follows red); render drops a finished job (all three red); the sink
allocates its track ID (allocates red only). Attempt 2: the keep branch deleted (follows red:
delivered at sample 128, before the adoption at 640); `Pending` checked before `UnknownStrip`
(follows red); a stale job armed again at its old epoch (follows and allocates red); the curves
left out of the charge (bound red: delivered 49,455 bytes against a 4,199-byte charge).
Attempt 3: the high-water ignored (shrinking red: a held job of 5,072 bytes against a 2,177-byte
job charge); the member table, the member identities and the collector's 3 x 127 identities left
out (bound red: a 3,142-byte job against 1,999; shrinking red: 5,072 against 3,167).
