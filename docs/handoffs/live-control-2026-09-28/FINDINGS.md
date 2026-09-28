# Live mix control on mobile: can the C ABI reuse the browser's live-control lane? (#1020)

Date: 2026-09-28. Research only, for open issue #1020. Base: local branch `research-1020` at
`d70956bf` (`codex/batch-slim-1`: the dead-code audit, its verification and the owner's rulings in
`docs/rulings/engine-footprint-2026-09-28.md`). Every fact below was read from source at that base
or reproduced by a build or a test. The prototype in section 4 was built in a scratch copy outside
the repository; no product code, test, script or workflow in this repository changed. Nothing was
pushed, and no GitHub issue was edited. The scratch copy and its `target/` were deleted afterwards.
(The research agent could not write this file itself; root saved its text verbatim.)

## 1. Answers

| question | answer |
|---|---|
| 1. Can the C ABI reuse the browser's live-control lane? | **Yes, and the lane is already shared code.** The queues, the render-side drains and the effect lane's staging live in `host-core`, `builtins-compiler` and `effect-contract`. `host-web` adds only a front end that decodes its 48-byte records. capi does not have the lane today because it prepares with `HostConsoleRequest::default()`, which asks for no console. Reuse needs no new C entry point, no wrapper around `admit_commands` and no new protocol frame. capi attaches the console when it prepares a plan. When a `SessionTransactionApply` changes only live values, capi lowers the change to lane records instead of replacing the plan. A scratch prototype of about 290 production lines passed 3 behavioural tests on the nine-track fixture: a live mute reached PCM at the next block, the source rings were kept, and the mute-and-unmute round trip was bit-identical to a plan prepared without the console. |
| 2. Which unwired lines and which parts of #140 become unnecessary? | **All of them.** That is 8,244 lines of host-core endpoints and 4,930 lines of protocol delivery files. With these gone, 330 lines of plumbing in the protocol controller, queue and exports also go. In scratch, deleting all of it together with the prototype compiled `--workspace --all-targets --all-features`, and every protocol and host-core test passed. #140's remaining outcomes (the D1-D5 decisions and children A, B and C) deliver *transient, sample-timed* `AutomationEnqueue` records. A fan changing a mix needs a *persistent* value change, and the protocol already has one: `SetTrackFader`, `SetTrackMatrixOrPan` and `UpsertEffectParam`. The net footprint is **about -12,500 lines** (measured -12,811 in scratch, before the slice's remaining gates). |
| 3. If not, what is the smallest change? | Not needed. For the record, the only route without the lane would keep source rings across a structural replacement. That still restarts every filter, dynamics and delay state and prepares a whole plan on every fader tick (section 3, option D). |
| 4. Can an ack ever precede a drop? | **Not under the recommended design**, provided it keeps five conditions (section 5). The model the ack commits is the durable owner of the value. The lane only carries the value to the running plan. If a plan is replaced before its queue drains, the replacement is compiled from that same model. |

**Recommended ruling (one line):** *Mobile live control ships as value-only `SessionTransactionApply`
lowered onto the existing console lanes (draft L1 in section 7); delete the unwired #140 delivery stack
now (draft 02 including its step 4) and close #140 as superseded.*

## 2. Traced facts

### 2.1 Browser: command to DSP

1. **Preparation attaches the console.** The browser is prepared with
   `control_queue_depth = Some(n)`, from `console_request` (`hosts/host-web/src/lib.rs:8284-8307`).
   `compile_ready` (`:7715-7805`) calls `prepare_host_runtime_with_selected_meters_between_render_calls`,
   the observation-demand variants, or `prepare_host_runtime_with_console_and_spectrum`. The first two
   declare `BuiltinControlDelivery::BetweenRenderCalls`. The spectrum entry uses the default,
   `Concurrent` (`crates/host-core/src/prepare.rs:544-559` passes `false`).
2. **host-core returns the producers.** `HostConsoleHandles` (`prepare.rs:354-379`) holds one
   `TrackControlProducer` per track, with matrix, fader and input queues
   (`crates/builtins-compiler/src/lib.rs:240-259`). It also holds one `EffectControlProducer` per
   effect instance (`crates/effect-compiler/src/prepare.rs:1204`), each a checked handle over a
   `bounded_spsc`.
3. **The front end is host-web's own.** The wasm export `miso_engine_web_v1_command_submit(handle: u32, count: u32)`
   (`hosts/host-web/src/ffi.rs:5473`) takes 32-bit wasm addresses and handles, as every export in
   that file does (for example `:2725`). It calls `admit_commands` (`lib.rs:6246`). That function
   decodes fixed 48-byte `miso.command.v1` records and lowers them to `TrackFaderRecord`,
   `TrackControlRecord` or `EffectControlRecord`. It composes solo into mute, runs the EQ owner
   transaction with prepared targets and checks room on every destination queue before pushing
   anything (`:6246-7034`). The whole front end is about 1,500 lines (`:5533-7034`), plus
   `control_targets.rs` (855).
4. **Room accounting assumes one thread.** `ready.in_flight` is exact only because "the browser's
   control plane and render plane are the same thread" (`lib.rs:2114-2120`). It resets after each
   successful render (`:4895-4902`). The admission sample is `next_absolute_sample`, known exactly.
5. **Render drains at the stage.** Fader, mute and matrix records are drained by
   `ConsoleFaderProcessor` and `ConsoleMatrixProcessor` or their bank drains
   (`builtins-compiler/src/lib.rs:958`, `:994`, `:4257`, `:4320`). Effect records are staged by
   `EffectControlLane::stage` (`crates/effect-contract/src/live.rs:291`), called from
   `graph/src/runtime.rs:3377` and `rack/src/lib.rs:1242`. It emits `Point` spans at the block's
   `first_sample` and folds the channel-symmetry witness (#1004). A record is dropped only when the
   staging window holds more distinct targets than it can take. Preparation makes that unreachable by
   capping the queue at the effect's `automation_capacity` (`live.rs` doc on `stage`).

### 2.2 C ABI today: command to DSP

1. **Preparation attaches no console.** `prepare_runtime` calls `prepare_host_runtime(compiled, &caps)`
   (`crates/capi/src/runtime/compile.rs:410`). That is `prepare_host_runtime_with_console(.., &HostConsoleRequest::default())`
   with `control_queue_depth: None` (`prepare.rs:508-516`, `:325-336`). The plan has no fader, matrix
   or effect queues.
2. **Every command is a protocol frame.** `miso_engine_v1_submit_command` reaches `SessionState::command`
   (`control.rs:663`), which calls `ProtocolController::prepare_command_frame` (`:675-686`).
3. **Every successful transaction is structural.** `plan_structural_command`
   (`crates/protocol/src/controller.rs:1979-2135`) returns `Structural` for any valid exact-revision
   `SessionTransactionApply`. capi then calls `prepare_runtime` (`control.rs:711`), which builds a new
   plan, new source rings and a new catalog. It reserves a plan replacement and commits the protocol
   transaction. The new plan's rings start empty. That is the frozen policy
   `ResetAtReplacementBoundary` (`control.rs:46-53`), and the existing test asserts an all-zero block
   after the swap (`runtime/tests.rs:581`, assertion at `:721`). A fader, mute or pan change
   (`SetTrackFader` 0x020f, `SetTrackMatrixOrPan` 0x0210) or an effect parameter change
   (`UpsertEffectParam` 0x020d) therefore costs a full prepare, silence until the host re-seeks and
   resubmits PCM, and a restart of all DSP state.
4. **`AutomationEnqueue` (0006) is admitted and never applied.** It is queued, and its only consumer is
   cancellation on the next revision change (`controller.rs:2296`;
   `docs/CONTROL_PROTOCOL_SEMANTICS.md:15`). The provider catalog lists effect parameters only
   (`crates/host-core/src/control_provider.rs:420-510`). **Fader, mute and pan have no protocol
   handle**, so even a finished #140 could not address a fader through `AutomationEnqueue` without
   extending the catalog.

### 2.3 What differs, and what does not matter

| | browser | C ABI | consequence for reuse |
|---|---|---|---|
| console at preparation | yes | no (`compile.rs:410`) | one call-site change |
| command format | 48-byte records at u32 wasm addresses | protocol BTLV frames | the wasm FFI cannot be reused, and need not be: lower protocol edits instead |
| threads | control and render on one thread | control thread and render thread | must use `Concurrent` delivery; room checks stay exact because a single producer's `available_capacity` only grows (`crates/engine/src/realtime/spsc.rs:326-343`) |
| where the value lives | transient host state; the SDK holds the document | `SessionModel` plus revision (canonical snapshot) | reuse must write the model, or a later structural swap would silently revert an acked edit |
| application time | first sample of the next block, all queues at once | next drain of each stage | a transaction touching two stages can land one block apart (section 6) |
| plan replacement | none after boot | epoch swaps (`control.rs:619-661`) | records must go to the newest epoch's producers |

The lane code is the same crates in both hosts. host-web and capi share host-core (`capi/Cargo.toml`
enables `control-provider`). The 32-bit pointers and the crate boundary matter only to option B,
which would move host-web's front end.

### 2.4 What the unwired stack is

- **`host-core/src/builtin_batch_endpoint.rs` (2,529) and its tests (1,727).** This is a ticketed
  wrapper around the *same* console queues. Control publishes a fixed batch through protocol's
  delivery core; the render owner claims the batch, copies it into the prepared builtin producers and
  completes the ticket after the block (`:1-6`). It owns its own prepared host and render session. It
  has no path to capi's plan exchange and epoch swap, and it covers fader, mute and matrix only.
- **`host-core/src/scalar_point_endpoint.rs` (1,060) and its tests (2,928).** This delivers
  `AutomationEnqueue` Points to **one** native compressor's makeup parameter only (`:1-7`,
  `MAKEUP_INDEX = 5`). It is the #524 and #528 demonstration of #140 D4.
- **`protocol/src/delivery.rs` (2,135), `controller_delivery.rs` (2,102) and `tests/delivery_ownership.rs` (693).**
  These are the #460 ticket, reservation and cancellation service and its controller facade (#530).
  Their only callers outside `protocol` are the two endpoints.
- Together they implement #140's D2-D4 and #444's recommended "closed batch claim" for
  **transient, timestamped** automation. Two things were never built: integration with capi's plan
  replacement (#140 child C, #444 item 3), and builtin handles in the catalog.

## 3. Options

Line counts cover production and tests. The scratch figures in option A are measured; the rest are
estimates.

| option | what it is | lines | ack and drop | timing | main risk |
|---|---|---:|---|---|---|
| **A. Value-only transactions over the lanes (recommended)** | capi attaches the console. When the committed-model delta is only live values, capi lowers it to lane records, pushes after a room check, then commits the prepared protocol transaction. Nothing else changes. | about +300 production and +450 tests for L1 (the prototype: +293 and +378); **net about -12,500 with draft 02 and step 4** | safe (section 5) | each stage applies at its next drain: at most 1 block of skew inside one transaction | per-stage rather than per-batch application; the four fader and matrix drains must be bounded first (section 6) |
| B. Port the browser's record ABI to capi | Add `miso_engine_v1_submit_console`, move `admit_commands` and its helpers (about 1,500 lines plus 855) into host-core, and replace the single-thread `in_flight` accounting. | +600 if moved, +2,400 if duplicated; the unwired stack can still be deleted | **unsafe as it stands**: the overlay is not in the model, so the next structural swap silently reverts acked edits | next block, all queues at once, only if render claims the batch | a second command surface on the C ABI, contrary to the R3 ruling ("the C ABI's command path is the protocol"); still needs a model write-back |
| C. Finish #140 | Wire the endpoints into capi, add builtin handles to the catalog, and build child C (controller, plan and lifecycle integration) and the #444 lifecycle item | keeps 13,504 lines and adds children; each prior child here ran 1,000-3,000 lines | ack means admission; cancellation is reported | sample-accurate Points | wrong semantics for a mix change: the transient overlay (D5) reverts at every structural swap, and the work is the largest by far |
| D. Status quo, or keep rings across swaps | Each edit remains a structural replacement | 0, or a few hundred lines to keep rings | safe | block after the swap | a full prepare per fader tick; every EQ, compressor and delay state restarts; with the status quo, also silence until the host resubmits |

## 4. Prototype (scratch, not committed)

This is option A for fader, mute and matrix/pan only, built on a copy of `d70956bf`.

- **host-core `live_delta.rs` (223 lines).** `live_builtin_delta(current, next, fader_smoothing, max_smoothing)`
  clones `next` and copies `current`'s revision and each track's `fader` and `matrix_or_pan` into the
  clone. If the clone is not equal to `current`, it returns `None` (structural). Otherwise it emits
  records. It sends `FaderDb` or `Mute` with `Both` when both lanes move to the same value, and
  per-lane records otherwise. A matrix record goes through `pan_matrix` or `Matrix2x2::checked`. A
  value outside the live domain (`[-144, 24]` dB, or a bad coefficient) returns `None`, so the
  structural path reports the same preparation diagnostic it reports today. `fits()` compares
  per-queue counts against `available_capacity`. `push()` pushes the records.
- **capi (+67/-7).** `prepare_runtime` calls `prepare_host_runtime_with_console` (depth 16, Concurrent).
  `ProviderEpoch` gains `console: Vec<TrackControlProducer>`. In `command()`'s `Structural` arm, after
  the existing output-size checks, capi calls `live_builtin_delta`. When the result is `Some`, it
  picks the newest epoch (pending, else current), returns `Backpressure` if the delta does not fit,
  pushes, commits the protocol token and returns the committed response. No plan is prepared.
- **Results** (x86-64-v3, debug, nine-track `parametric-eq-nine-track.json`, quantum 128). Three
  behavioural tests, plus one descriptive probe:
  - **Bit-identical before any edit.** Block 0 from the console-attached capi plan equals, bit for
    bit, a console-free `host_core::prepare_host_runtime` plan fed the same PCM.
  - **One transaction mutes the mix at the next block.** A single `SetTrackFader` transaction muting
    all 9 tracks produced revision +1, no pending provider, a model that carries the mute, and an
    exactly zero block 1.
  - **Unmute plays on from the same ring.** After the unmute transaction, block 2 is bit-identical to
    the reference's block 2, read from the same source ring with no re-seek.
  - **A replay pushes nothing.** An exact replay returned the cached response; the revision was
    unchanged and block 3 still matched.
  - **Backpressure comes before the commit.** After 16 un-rendered transactions filled the depth-16
    queues, the 17th returned `Backpressure` with the revision unchanged; one render freed the room.
  - **A later structural swap keeps the value.** A live mute followed by a `SetSessionId` swap
    renders zero on the new plan. The control run without the mute renders non-zero.
  - **Existing capi tests.** All 32 existing capi library tests pass. Two `resource_lifecycle`
    oracles fail only because `ProviderEpoch` grew by one `Vec` (24 bytes per retained epoch); a
    production slice updates the mirror and charges the console rows.
  - **Descriptive cost** of the console on the fixture: graph plus plan grows 237,481 -> 257,065 bytes,
    builtin retained 17,451 -> 36,873, and effect-control payload is 15,361 (the EQ owners).
  - **Protocol backpressure seen on the way.** Each live commit emits one reliable `SESSION_COMMITTED`.
    With the fixture's reliable-event capacity of 2, a host that never dequeues gets typed
    `Backpressure(ReliableEvent)` on the third edit. That is the existing contract, not a loss.
- **Footprint check.** On the same scratch tree, deleting draft 02's four files and step 4's three
  protocol files needed the following plumbing: `controller.rs` -302/+37 (the `DeliveryContext` and
  `ScalarStateSlot` threading and six `delivery_*` methods), `queue.rs` -16/+1 and `lib.rs` -12.
  After that:
  - `cargo check --workspace --all-targets --all-features` and the wasm `simd128` check of `host-web`
    and `host-core` pass.
  - The protocol, host-core (`control-provider`) and capi tests pass, apart from the 2 oracles above.
  - Measured total: **+709 / -13,520 lines, net -12,811.**
- **Not verified:**
  - AArch64 (no aarch64 target is installed here);
  - a threaded submit-while-rendering test;
  - allocation counting on render with edits in flight;
  - clippy on the prototype (it flags `Result<_, ()>` in `push`).

## 5. The acked-batch check for option A

An ack is the protocol's `TransactionApplied` response. capi writes it only after the protocol
commit, and commits only after every record has been pushed.

1. **Room is checked before the commit, and a push cannot fail after the check.** Session calls are
   serialized (header thread contract), so each queue has exactly one producer. `available_capacity`
   can only grow until that producer pushes (`spsc.rs:326-343`). A full queue returns
   `Backpressure`: nothing is committed, no replay entry is written and no event is sent.
2. **Render never drops an admitted record.** The builtin drains apply every record they pop. The
   effect lane drops only when its staging window overflows, and preparation rules that out by capping
   the queue depth at the effect's automation capacity. A record whose value would fail inside render
   (`set_fader_db` rejects values outside `[-144, 24]`) is refused at lowering instead, by the same
   check. Otherwise the render-time failure would pop, and so lose, the record.
3. **A plan replacement cannot lose a value.** A queued record may die with the old plan. The
   replacement is compiled from the committed model, which already holds the value, so the new plan
   starts at it; only the ramp is lost, as with every structural swap today. The scratch test above
   shows this.
4. **A pending candidate is handled.** The records go to the newest epoch's producers. A candidate
   plan was compiled before the live edit, so it needs the records and drains them on its first block.
5. **A replay does not push twice.** An identical replay returns the cached response before the
   `Structural` arm runs.

Conditions the implementation must keep, each a gate in L1:

- lower only values the render-side setter accepts;
- push, then commit (fault injection must show that a failure between the two leaves no ack);
- bound the drains (section 6);
- route to the newest epoch;
- never let an `EffectControlLane` queue exceed its staging window (already enforced at preparation).

Option B fails check 3 unless it adds the same model write-back. Option C keeps its reported-cancel
semantics, but its D5 overlay reverts *applied* transient values at every structural swap.

## 6. Risks and open decisions for option A

- **Application is per stage, not per batch.** `Concurrent` producers let one transaction's fader
  record and matrix record land one block apart. That is the gap #444 described ("records may
  arrive between the current fader and matrix process calls"). A fan's fader move does not need a
  closed-batch contract. If a product later needs block-atomic multi-track scenes, a render-side
  claim or epoch tag is a bounded successor, far smaller than the endpoints (#444's own
  recommendation).
- **The drains must be bounded first (realtime rule).** Four fader and matrix drains use
  `while let Ok(record) = try_pop()` (`builtins-compiler/src/lib.rs:958`, `:994`, `:4257`, `:4320`).
  With a concurrent producer, a drain can keep running as the queue refills, which is data-dependent
  and unbounded. The input drain (`:450`, `:4163`) and the effect lane already stop at
  `available_at_entry()`. Bounding the other four the same way changes nothing for the browser,
  because nothing pushes during its render.
- **Per-lane effect pairs (a successor issue).** A `Left`/`Right` pair pushed concurrently can split
  across two drains. That clears `LIVE` and retires mono collapse, which costs performance but not
  correctness (#1004).
- **Fader smoothing has no session field.** `DualMonoFader` carries none; the matrix has
  `smoothing_samples`. The prototype used 0, the browser SDK's default (`sdk/src/core/console.ts:126`).
  This needs an owner decision (D3 in L1).
- **Each edit compiles the session.** `prepare_transaction` compiles the whole session on every edit,
  as it does today, on the control thread. The prepare step is skipped. The cost at 30-60 Hz fader
  drags has not been measured; the app should coalesce to the latest value.
- **Resources.**
  - The console costs about 35 KB on the 9-track fixture.
  - A live commit can grow the canonical JSON, which is one of capi's retained rows, so the live path
    must run `compiled_model_admission` before it commits.
  - Native loses the browser's fused fader-and-matrix pair: `BetweenRenderCalls` only, and capi must
    not declare it. The performance effect has not been measured.
- **Only value edits become live.** Structural edits still reset rings (`ResetAtReplacementBoundary`),
  and `AutomationEnqueue` stays admitted but not applied, as documented.

## 7. Draft issue L1 (the smallest closable slice)

### Title

Deliver value-only fader, mute and pan transactions to the running C ABI plan through the live console lanes

### Context

- A fan's fader, mute or pan change through `miso_engine_v1_submit_command` is a `SessionTransactionApply`.
  Today it always replaces the plan, which resets source rings and restarts all DSP state
  (`capi/src/runtime/control.rs:46-53`, `:711`).
- The engine already has live lanes for exactly these values, used by the browser
  (`HostConsoleRequest::control_queue_depth`, `TrackControlProducer`).
- capi does not request them (`compile.rs:410`).
- Findings: `docs/handoffs/live-control-2026-09-28/FINDINGS.md`.

### Scope

1. **Bound the drains.** Bound the four fader and matrix drains by `available_at_entry()`
   (`builtins-compiler/src/lib.rs:958`, `:994`, `:4257`, `:4320`). This is its own commit.
2. **Attach the console in capi.** Prepare with `Concurrent` delivery and keep `track_controls` in
   `ProviderEpoch`. Effect producers stay idle in this slice.
3. **Add the live delta.** Add `live_builtin_delta` behind host-core's `control-provider` feature,
   so the browser build does not compile it.
4. **Commit live deltas without a new plan.** In `command()`'s `Structural` arm: when the delta is
   `Some`, run `compiled_model_admission`, check room, push to the newest epoch, then commit the
   prepared token. Otherwise run the existing path unchanged.
5. **Charge the resources.** Charge the console producer table and queue rows in capi's resource
   report, and update the `resource_lifecycle` oracles.
6. **Document the behaviour.** Update the C header comment, `docs/C_ABI_V1_QUALIFICATION.md` and
   `docs/CONTROL_PROTOCOL_SEMANTICS.md`. Value-only transactions apply at each stage's next block
   and are never lost. Structural transactions replace the plan.

### Decisions to freeze before coding

- **D1. Classification.** Classify by committed-model delta. A delta is live when `fader` and
  `matrix_or_pan` are the only fields that differ and every value passes the render-side setter's
  domain. Anything else is structural. Edit opcodes are not inspected.
- **D2. Timing contract.** A live edit is admitted all or nothing and applied at the next drain of
  each destination stage (at most one block of skew). It is never lost. No block-atomic claim
  (#444) is made.
- **D3. Fader and mute ramp.** Options: 0 (step at the block boundary; the browser SDK default,
  bit-exact to a re-prepared plan), or a fixed ramp such as one quantum. The owner rules.
- **D4. Queue depth.** Use a fixed per-track depth (the prototype used 16). A full queue is typed
  `Backpressure` before commit. No compile-limit ABI field is added.

### Objective gates

1. **PCM through the C ABI.** Through the exported entry points, a `SetTrackFader` or
   `SetTrackMatrixOrPan` transaction changes the next block of the *same* plan: no pending
   provider, and no re-seek or resubmission. A mute-and-unmute round trip is bit-identical to a
   console-free reference plan fed from the same ring. Check at 1 and 10 tracks and at the four
   launch rates.
2. **Persistence.** The snapshot and the revision carry the value, and `SESSION_COMMITTED` is
   emitted. A later structural replacement starts at the committed value, checked against a control
   run without the edit.
3. **No ack before a drop.**
   - A full queue gives `Backpressure` with the model, revision and replay unchanged.
   - Fault injection between push and commit leaves no ack and no committed revision.
   - A live edit while a candidate is pending lands in the candidate.
4. **Realtime.** A two-thread barrier test (the shape of
   `barrier_schedule_separates_one_source_producer_from_exclusive_render`) submits live edits while
   render runs. It shows zero allocations and frees on render (the `resource_lifecycle` allocator)
   and a final state equal to the last committed model.
5. **Unchanged behaviour.**
   - Every existing structural capi test passes unchanged.
   - Console digests are byte-identical: `gain_pan_profile digests` and the host-web console tests.
   - The shipped artifact's hash moves only because of the bounded drains; record the move.
6. **Builds.**
   - Native `--all-targets --all-features`, `clippy -D warnings`, and the wasm `simd128` check pass.
   - AArch64 passes through #1017's CI once that exists; until then, a compile check on the targets.

### Out of scope

These are successors, each a separate issue:

- **L2:** effect parameters and bypass (`UpsertEffectParam`, `SetEffectBypass`) through
  `EffectControlProducer`, with provider readback.
- **L3:** EQ prepared targets (`EqTargetPreparer`), and input trim and polarity.
- Solo stays an app-side composition of mutes in one transaction.
- The block-atomic batch claim (#444) is added only if a product asks for it.
- Sample-accurate transient automation.

### Dependencies

None that block it. Draft 02 and its step 4 are independent and can land first.

## 8. Consequences if the owner accepts the ruling

- **Draft 02 (held) can land as drafted, with its step 4.** Section 4 shows the deletion compiles and
  the tests pass. Its #140 comment should point to L1.
- **#140 can close as superseded.** Its browser outcomes (live effect spans, ramped fader and mute,
  admission) were delivered long ago: the lanes L1 reuses. Its remaining outcome, delivering protocol
  `AutomationEnqueue` to PCM, has no product consumer. #444 is already closed.
- **Optional follow-up, not part of this ruling:** capi could refuse `AutomationEnqueue` as
  `UNAVAILABLE`, so an app cannot mistake admission for delivery. It is documented as retained but
  not applied today (`CONTROL_PROTOCOL_SEMANTICS.md:15`).
