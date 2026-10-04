# Qualify live C ABI edits against a concurrently rendering plan

Core slice 6 of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053). It is the qualification of #1257: the realtime and
acked-batch contracts with a render thread running concurrently, at the exact resource caps, and
under reliable-event backpressure. Anchors verified on `main` at `54b0a1bf8`; re-verify them after
#1257 lands.

## Product outcome

Evidence that a phone app can move faders, mutes and pans while the audio thread renders and while
other edits rebuild the plan, and that:

- the audio thread never allocates, frees, locks or makes a syscall;
- no call returns `INTERNAL`;
- no acked edit is lost;
- the live admission holds the caller's caps to the byte.

No product code changes unless a gate finds a defect. A fix is in scope only if it stays inside
capi's live arm (#1257's `commit_live`). Otherwise stop, record the evidence, and file a bounded
issue.

## Context (verified at `54b0a1bf8`)

All paths are in `crates/capi/` unless named.

- **The race harness.** `tests/resource_lifecycle.rs` already races control calls against
  plan-swapping renders: `race_plan_swaps` (`:1541`) and
  `control_calls_racing_plan_swapping_renders_never_wedge_replacement` (`:1770`). It counts
  allocations and frees with its own global allocator, which the realtime-policy scan exempts
  (`scripts/check-realtime-policy.sh:29`).
- **The exact-cap oracle.** `double_live_oracle_drives_exact_and_one_below_c_caps` (`:1263`)
  derives exact caps from two live reports and drives the C entry point at the cap and one byte
  below it.
- **The render audit.** `tools/audit/src/capi.rs` renders 100,000 calls of the nine-track EQ
  fixture, all inside one `audit::in_render_scope` (`:164-185`). CI validates its one JSON record
  (`.github/workflows/qualification.yml`, the "Validate Issue-544 runtime audit records" step, and
  `scripts/run-aarch64-tests.sh:166-185`): the key set, `calls`, and every forbidden counter are
  exact; `pcm_digest` is checked only for its form. So the record's shape must not change, and the
  digest may.
- **The live arm (#1257).** `commit_live` runs admission (`validate_live_peak`), resolve, room,
  `check_prepared_structural`, push, commit, respond. The live admission is #1053 D8.
- **The reliable lane** has two slots (`src/runtime/compile.rs:98`). Reliable-event backpressure is
  decided before a token exists (`crates/protocol/src/controller.rs:1734-1746`). It is a protocol
  response with status `Backpressure` under C result `RESULT_OK` (the pinned `EVENT_FULL` vector,
  `src/runtime/tests.rs:1903-1908`), not `RESULT_BACKPRESSURE`.
- **Two kinds of `RESULT_BACKPRESSURE`.** After #1257's D8, a full live lane sets the last error
  `control.live.backpressure`; a pending candidate or the epoch lag sets
  `control.plan.backpressure` (`src/ffi.rs:625-631`).

## Decisions

- **D1. The race session.** Use tracks with an empty console, no inserts, no input filter and zero
  `delay_samples`, fed a constant, distinct, non-zero value on each lane. Nothing then keeps state
  that depends on history, so the final comparison holds after any sequence of swaps. Keep every
  pan or matrix edit's `smoothing_samples` at most one quantum, or add the largest one to the
  settle window.
- **D2. Rebuild triggers.** Interleave structural edits that render identically, such as a changed
  content string on an unread source. Do not use `SetSessionId`: it becomes model-only in #1260.
- **D3. Audit edits.** `audit capi` submits live edits between render calls, never inside the
  audited render scope, and keeps its record shape. The one scope around all calls becomes one
  scope per render call, with each submission and each event dequeue outside it. After each edit
  the audit dequeues the reliable events; otherwise every third edit is refused with a protocol
  backpressure response and never reaches a drain.

## Authorized paths

- `crates/capi/tests/resource_lifecycle.rs`.
- `tools/audit/src/capi.rs`.
- `crates/capi/src/runtime/live_tests.rs`, for gate 4 only.
- `crates/capi/src/runtime/control.rs` and `compile.rs`, only to fix a defect that a gate here
  finds inside the live arm.
- This spec.

## Non-goals

- No new feature. No change to the classifier, host-core or the protocol.
- No benchmark.

## Objective gates

Run every command from the repository root.

1. **The race.** Add a two-thread test in `tests/resource_lifecycle.rs`, beside `race_plan_swaps`:
   - The render thread renders continuously through `miso_engine_v1_render_f32_planar`.
   - The control thread submits live fader, mute and pan edits, several per render period, and a
     structural edit every eighth edit (D2). It feeds the sources, seeks again after every swap,
     and drains the events.
   - Run it 20 times. Assert:
     - every render call shows `allocations == 0` and `frees == 0` in the file's counters, after
       warm-up;
     - no call returns `INTERNAL`;
     - every `RESULT_BACKPRESSURE` carries `control.live.backpressure` or
       `control.plan.backpressure`, and the test retries it with a new request ID, at most a
       bounded number of times per edit (as `WEDGE_BLOCKS` bounds it,
       `resource_lifecycle.rs:1546`);
     - after the last edit, render `latency_samples` plus one quantum; the final block is then
       bit-identical to that of a fresh plan compiled from the final committed snapshot and fed the
       same constant source.

   *Test value: it turns red if a drain allocates or frees on the render thread, if a live edit
   that races a swap reaches the retiring plan or is lost (the final block differs), or if epoch
   synchronization returns `INTERNAL` under live traffic.*
2. **The audit.** In `tools/audit/src/capi.rs`, every 64th call submits a live edit before the
   render call: a mute toggle on one track and a pan move on another, alternating. The record's
   shape stays as it is, and every forbidden counter stays 0:
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `./target/release/audit capi`
   - `cargo test --locked --release -p audit -p bench -p console-workload`

   *Test value: it turns red if draining a non-empty fader or matrix lane allocates, locks or makes
   a syscall on the render thread.*
3. **Exact live caps.** Add a test in `tests/resource_lifecycle.rs`, beside the double-live oracle.
   Use a fader edit whose canonical JSON grows the compiled model (for example `-6.0` to
   `-6.0123`):
   - With `maximum_graph_session_plus_plan_bytes` equal to the live peak of #1053 D8, the edit
     returns `OK`.
   - One byte below, it returns `COMPILE_REJECTED` with `graph.resource.limit`, and the revision
     and the queue room are unchanged.
   - Do the same for `maximum_capi_retained_bytes`, and once with a structural candidate pending.

   *Test value: it turns red if the live arm admits an edit whose peak is over the caller's cap, or
   refuses one within it.*
4. **Reliable-event backpressure.** Make two live edits without dequeuing. The third returns
   `RESULT_OK` with a protocol response of status `Backpressure` for the reliable-event queue; the
   revision is unchanged and no strip queue's room changes.
   *Test value: it turns red if a live push comes before the reliable-event capacity check.*
5. **Workspace and policy.**
   - `cargo test --locked -p capi`
   - `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `for x in realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - 4-lane (NEON): `bash scripts/run-aarch64-tests.sh release` (it runs `audit capi` on arm64) and
     `bash scripts/run-aarch64-tests.sh debug` are CI-only here (`aarch64-release`,
     `aarch64-debug`); record them as not run locally.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.
- The race's run count and its retry and `BACKPRESSURE` counts.

## Dependencies

- *Apply value-only track fader, mute and pan transactions to the running C ABI plan* (#1257)

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- Every wait on another thread has a deadline and fails with a message; never hang (#1251).
- "Bit-identical" gates are hard stops.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Attempt record

### Attempt 1 (implementer, on `ef5ac335c`)

**What changed.** Tests and the audit only. No product code changed: no gate found a defect in
#1257's live arm. Every change is inside the authorized paths.

- `crates/capi/tests/resource_lifecycle.rs`: the gate 1 race, the gate 3 cap test, their C ABI
  helpers (`submit_header`, `last_error_c`, `drain_events_c`, `snapshot_c`, `ConstantFeed`,
  `fed_render_c`, `transaction`), and #1256 MINOR-1 (below). The capi epoch terms the double-live
  oracle computed inline are now `HostHalf::capi_epoch_terms`, shared by both cap tests.
- `tools/audit/src/capi.rs` (gate 2, D3): every 64th call first commits a live edit through
  `miso_engine_v1_submit_command`, alternating a mute toggle on `eq0` and a pan move on `eq1`, and
  dequeues the reliable lane until it is empty. The one `in_render_scope` around all 100,000 calls
  is now one scope per render call, and the edits and dequeues run outside it. The record's shape
  is unchanged. Only `pcm_digest` moved (`ff6cdcb96cdcdad5` -> `18e56b897a3abf17`), which CI checks
  only for its form. The audit asserts that all 1,563 edits commit. With 16-record lanes, the 17th
  mute toggle would be refused if a render did not drain the lane, so the drains really run inside
  the audited scope.
- `crates/capi/src/runtime/live_tests.rs` (gate 4): one test.
- No new audit mode, command or test-file pattern, so the CI router lists and the workflows are
  unchanged (lesson c). Nothing compiled into the browser module changed, so the worklet chain was
  not run.

**Lesson (d), and why the race does not call `render_while_producing`.** `bench_support` installs
its own `#[global_allocator]`. `resource_lifecycle.rs` has its own counting allocator, and the
gate's counts come from it, so the two cannot link into one test binary. capi also does not
depend on `bench-support`, and its `Cargo.toml` is outside the authorized paths. So the race keeps
this file's own pattern from `race_plan_swaps` and applies the lesson by hand:
- the render thread never asserts. It records a refused block and stops;
- every wait on the control thread has a 10 s deadline (`PROGRESS_DEADLINE`) and fails with a
  message;
- `StopOnDrop` stops the render thread however the control thread leaves the scope, so a failure
  on either side ends the run instead of hanging the join.

**New tests, each with its test value and the mutation runs that prove it.** For each run the
defect was introduced, the test went red, and the defect was reverted.

- `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free` (gate 1, 20
  runs). The race session follows D1: the nine-track fixture with an empty console, no inserts,
  HPF/LPF off, zero delay, and a constant source of `[0.375, -0.21875]`. Each run makes 156 edits:
  live fader, pan (smoothing 16-128) and mute edits cycling over the nine tracks (fader and pan
  values distinct; each mute edit flips its track's left mute, 36 of 43 changing a value; see
  Follow-ups applied), and a `SetSourceContent` structural edit every eighth (D2). The control thread waits for
  render progress every third edit, so several edits land in each render period. It feeds the
  source, seeks under a new generation after every swap (it detects a swap by
  `source.generation.stale`), and drains both event lanes after every call. Halfway through each
  run, the render thread parks, as a paused audio session does. Meanwhile the control thread fills
  `eq0`'s fader lane until the arm refuses with `control.live.backpressure`, commits a structural
  edit, and submits a second one that is refused with `control.plan.backpressure`. Both are retried
  under new request IDs once rendering resumes. Without the pause the render thread drains lanes
  faster than a debug control thread fills them, and no retry ever ran: the first draft counted 0
  of each.

  Every `RESULT_BACKPRESSURE` must carry one of the two diagnostics. A retry is bounded by 256
  rendered blocks after the edit's first refusal (as `WEDGE_BLOCKS` bounds `race_plan_swaps`). Any
  other result, `INTERNAL` included, fails the run. The render thread's allocations and frees,
  counted after `begin()` warms the statics, must be exactly 0 over all its render calls. After the
  last edit, the plan renders `latency_samples` plus one quantum (every pan ramp is at most one
  quantum), and its next block must be bit-identical to the matching block of a fresh C ABI plan
  compiled from the final `SessionSnapshotGet` snapshot and fed the same constant source. That block
  must also be non-zero on both lanes.

  The last structural edit is edit 151, so the run ends with four live edits that no later rebuild
  re-prepares: a pan (152) and three mutes (153-155), of which 153 and 155 push a record. In the first draft the run ended with a structural edit, and mutation M1-a survived:
  the final rebuild prepares from the committed model and so healed every lost live edit.

  Counts over 20 runs, on each of three base runs: 3,080 live and 420 structural commits; 20 live
  and 20-21 plan `RESULT_BACKPRESSURE`; 40-41 retries (40 of them the forced paused-phase
  resubmissions, which follow-ups removed from the counter); 0 protocol event backpressure; 380-381 seeks
  after swaps; 3,160-3,161 control calls that overlapped a render call; about 10,400 race blocks.
  The 20-run test takes about 7 s in debug.

  *Test value:* red if a fader or matrix drain allocates or frees on the render thread, if a live
  edit that races a swap reaches the retiring plan or is lost, if epoch synchronization returns
  `INTERNAL` under live traffic, or if a `RESULT_BACKPRESSURE` carries another diagnostic or never
  clears. Mutation runs:
  - **M1-b**, a `Box` allocated per record in `drain_matrix_controls`: red, with
    `Snapshot { allocations: 47, .. }` on the render thread. No other capi test catches it: the
    full `cargo test -p capi` without this test was green with M1-b.
  - **M1-a**, the live arm pushing to `self.providers` instead of the newest epoch: red in run 0,
    where the final block was `[0.08013036, 0.31109753]` against `[0.014650766, 0.3247056]`. The
    existing `a_live_edit_while_a_candidate_is_pending_reaches_the_candidate` also catches it.
  - **M1-c**, `synchronize_plan_epochs` keeping only the atomic's and the pending rows (the current
    provider's row dropped): red with
    `edit 24: live edit: result 255 ... last error control.internal`. Three existing swap-window
    tests in `runtime/tests.rs` also catch it.

- `live_edits_are_admitted_at_their_exact_graph_and_capi_peaks_and_refused_one_byte_below`
  (gate 3). The edit moves `eq0`'s left fader from `-6.0` to `-6.0123`, which grows the canonical
  JSON (the test asserts the model grows). Each peak is derived from observations, never from the
  arm:
  - the C report of the current plan and, with a candidate pending, of the candidate (read after
    the render that swaps it in);
  - each compiled model's own estimate;
  - the capi epoch terms from their owning crates' reports.

  The three cases: graph, 292,807; capi, 397,315; graph with a `SetSourceContent` candidate
  pending, 546,741. At each peak the edit commits at revision + 1. One byte below, it returns
  `RESULT_COMPILE_REJECTED` with `graph.resource.limit\t$\n` or `capi.resource.limit\t$\n`. Its
  `SessionSnapshotGet` snapshot and the header revision are unchanged, and the plan renders 4 blocks
  bit-identical to a plan that never saw the edit. So no record reached a lane. The C ABI exposes no
  room counter; it is a test-only hook inside the crate. As a positive control, the edit admitted at
  the peak changes the rendered block. In the pending case, the test asserts that the structural
  edit's own peak is below the live peak, so the structural edit is admitted one byte below.

  The capi row with a pending candidate is not driven through the C ABI. For a candidate that keeps
  the catalog, the live pending peak (candidate row + current `epoch_retained` + the
  prepared-protocol term) equals the structural edit's own capi peak. One byte below would
  therefore refuse the structural edit first. Its arithmetic is covered by #1257's
  `the_live_admission_accepts_each_cap_and_refuses_one_byte_below`.

  *Test value:* red if the live arm admits an edit whose peak is over the caller's cap, or refuses
  one within it, on either row, or if a cap refusal leaves a record or a revision behind. Mutation
  runs:
  - **M3-e**, `live_admission` passing the current model as the prospective one: red (admitted one
    byte below).
  - **M3-f**, `live_admission` ignoring the pending candidate: red (pending case admitted one byte
    below).
  - **M3-d**, `>=` for `>` on the capi row: red (refused at the peak).

  M3-e and M3-f are input errors that #1257's unit test cannot see. With both applied, the full
  `cargo test -p capi` without this test was green.

  During the attempt an apparent over-cap admission was traced to the test, not the arm: a
  request ID lower than the snapshot's (902 after 1001) is `ReplayExpired` under `RESULT_OK`. The
  test now uses increasing request IDs.

- `a_live_edit_without_reliable_event_room_is_protocol_backpressure_and_pushes_nothing` (gate 4,
  `live_tests.rs`). It makes two live edits without dequeuing. A third edit, a fader and a pan on
  two other tracks, returns `RESULT_OK` with protocol status `Backpressure`. The revision is
  unchanged, no plan is prepared, and every strip queue's room (`live_rooms`, all epochs) is
  unchanged. After a dequeue, the same edits under a new request ID commit live, taking one record
  of room each.

  *Test value:* red if a live push comes before the reliable-event capacity check, or that check
  stops refusing a live transaction. Mutation run **M4**, the capacity check in
  `ProtocolController` disabled (`if false && ...`): red, with `RESULT_INTERNAL` from the commit
  predicate instead of the protocol refusal. The pinned `EVENT_FULL` vector
  (`exported_c_replay_revision_event_and_publication_pressure_statuses_are_exact`) also catches M4
  for a rebuild edit. This test is the live-arm case, and the only one that checks that no lane
  moved.

- Gate 2, audit mutations. Each was run through the release `audit capi`:
  - **G2-M1**, a `Box` per record in `drain_matrix_controls`: the audit aborts at the allocation
    (exit 134). Control: the same mutation against the pre-#1258 audit (`HEAD`'s `capi.rs`) stays
    green with total_violations 0, because that audit never submits an edit.
  - **G2-M2**, the same in `drain_fader_controls`: aborts (exit 134).

**Folded in: #1256 MINOR-1.** No exact resource oracle ran a session with submix strips or a route
into a submix. `capi_retained_bytes_charge_every_byte_the_compile_retains` now also observes
`two_track_routed_submix_session`: two tracks and three submix strips, with the first track routed
through `bus0` to the main output. Its line reads: capi 149,142 observed; store 10,711 of 11,225;
plan 89,452 of 174,270. Mutation runs:
- **O3**, `prepared_capi_resources` counting `normalized_model().tracks` instead of `.strips()`:
  red on the routed-submix case only (observed 149,142 against charged 148,722).
- **M3**, `HostLiveLanes { routes: true, ..FADER_AND_MATRIX }` in the C ABI lane selection: red
  on the routed-submix case only (150,085 against 149,142).

The three earlier sessions stay green under both mutations, which is the gap the verifier found.

**Gates, all run from the attempt's tree.**
- `cargo test --locked -p capi`: lib 47 passed, `resource_lifecycle` 13 passed.
- `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`: ok.
- `cargo fmt --all -- --check`: ok.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: ok. The first
  run flagged three `manual_is_multiple_of` and one unsafe block without a safety comment in the
  new code; all four are fixed.
- `for x in realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`:
  ok.
- `cargo build --locked --release -p audit -p bench -p capi -p session-validator`: ok.
- `./target/release/audit capi`: one record with every forbidden counter 0. The CI step "Validate
  Issue-544 runtime audit records", extracted from `qualification.yml` and run on that record,
  passes.
- `cargo test --locked --release -p audit -p bench -p console-workload`: ok.
- `bash scripts/check-cross-targets.sh`: PASS, with the iOS `memset_pattern16` rows reported as the
  expected failures (#1018). No product code moved (lesson b).
- AArch64 (`bash scripts/run-aarch64-tests.sh release`, which runs `audit capi` on arm64, and
  `debug`) is CI-only and was not run locally.

**Housekeeping.** The disk was full when the attempt started: 0 MB free, and the tool's own
temporary output failed with ENOSPC. 12.9 GB of older duplicate test executables in
`target/debug/deps` were deleted (the newest build of each test binary was kept), all in this
worktree's own target directory and while no cargo process was using it. Cargo relinks a deleted
binary on demand.

### Follow-ups applied (after the attempt 1 PASS; batch follow-ups worker)

- **MINOR 1.** A race mute edit is now `let mute = (step / 3 + track as u64).is_multiple_of(2);`,
  so 36 of 43 mute edits per run flip their track's left mute (it was 3 of 43). Edits 153 and 155
  among the final four live edits push records, the right lanes are never muted, and 4 of 9 tracks
  end left-muted, so the final mix stays non-zero (asserted). The `RaceEdits::edit` and `RACE_EDITS`
  docs and this record now say so. Mutations, each reverted afterwards:
  - mute-drop (the live arm skips every `TrackFaderRecord::Mute` push in `control.rs`): race
    **red** in run 0 on 3 of 3 invocations (final block `[-0.16999753, -0.25600186]` against
    `[-0.1212493, 0.039765462]`); it was green before this fix;
  - M1-a (the live arm pushes to `self.providers`): race red in run 0 on 3 of 3 invocations.
- **NIT 1.** The timing sentence now reads "the 20-run test takes about 7 s".
- **NIT 2.** `paused_bursts` no longer adds the two forced resubmissions to `retries`; the counter
  now counts only resubmissions `RaceControl::commit` found refused again: 0-6 per 20 runs over
  five invocations.
- Gates: `cargo test --locked -p capi` pass (50 lib, 13 `resource_lifecycle`); the three race tests
  passed in 5 more invocations (7.8-9.2 s each); `cargo fmt --all -- --check`, `cargo clippy --locked
  -p capi --all-targets --all-features -- -D warnings`, `scripts/check-capi-abi.sh`,
  `scripts/check-realtime-policy.sh` and `scripts/check-workspace-policy.sh` pass.

### Verdict

**Verdict.** Sol attempt 1: PASS (verdict file `docs/handoffs/live-updates-1053/1258-attempt1.md`). MINOR 1 (race mute edits now flip values) and NIT 1-2 applied in `fe3bb37e8`.
