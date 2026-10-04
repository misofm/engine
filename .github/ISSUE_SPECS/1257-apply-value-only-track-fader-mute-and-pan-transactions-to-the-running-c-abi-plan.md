# Apply value-only track fader, mute and pan transactions to the running C ABI plan

Core slice 5 of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053), its decisions D2, D6-D10. Anchors verified on
`main` at `54b0a1bf8`; re-verify them after #1253, #1255 and #1256 land.

## Product outcome

A `SESSION_TRANSACTION_APPLY` whose committed delta is only track fader, mute and pan or matrix
values (#1053 D1) changes the running C ABI plan:

- no new plan, no source-ring reset, no silent block, no seek and no resubmission;
- the change is heard within one quantum plus `latency_samples` (#1053 D2), as a step until #1054
  gives the ramps (#1053 D3);
- an acked change is never lost.

Every other transaction rebuilds the plan exactly as today.

## Context (verified at `54b0a1bf8`)

- **The structural arm today.** `SessionState::command` (`crates/capi/src/runtime/control.rs:694-860`)
  runs these steps in order:
  1. `plan_alive` (`:727`);
  2. the epoch-lag check (`:734`);
  3. the response-size check (`:737-742`);
  4. `prepare_runtime` (`:748-754`);
  5. `validate_replacement_peak` with `compiled_model_admission` (`:773-784`);
  6. the pending check (`:792`);
  7. the plan reservation (`:795-807`);
  8. the report row (`:820-829`);
  9. the protocol commit (`:840-842`);
  10. the catalog replacement (`:843-845`);
  11. the publication (`:846-848`);
  12. the response (`:855-857`).
- **The protocol token.**
  - `PreparedStructuralCommand` (`crates/protocol/src/controller.rs:1117-1153`).
  - `commit_prepared_structural` (`:1944-2004`) fails only with `WrongController` or
    `StaleGeneration`: a changed generation, revision or automation-queue occupancy. The token
    reserved its reliable event when it was prepared (`:1639`).
  - Reliable-event backpressure (`:1734-1746`) and replay (`:1676-1689`) are decided before a
    token exists. They return an immediate outcome, so neither ever reaches the structural arm.
    Reliable-event backpressure is a protocol response with status `Backpressure` under C result
    `RESULT_OK` (the pinned `EVENT_FULL` vector, `crates/capi/src/runtime/tests.rs:1903-1908`), not
    `RESULT_BACKPRESSURE`.
- **The classifier (#1255).** `host_core::classify_live_delta(current, next, LiveRamps::for_session(next))`,
  behind host-core's `control-provider` feature, which capi enables.
- **The producers (#1256).** `ProviderEpoch::strips`, with `controls: Box<[TrackControlProducer]>`
  and `track_count`.
- **The newest plan.** `replacement_base_report` (`control.rs:611-623`) uses the same "pending,
  else current" rule as D7.
- **The catalog.** The provider catalog lists effect parameters only
  (`crates/host-core/src/control_provider.rs:420-520`). A fader or pan delta leaves it as it is.
- **The test helpers** (`crates/capi/src/runtime/tests.rs`): `command_bytes_at_revision` (`:53`),
  `generated_parity_session` (`:110`), `submit_c` (`:144`), `boxed_c_children` (`:180`) and
  `command_c` (`:198`). They are private to that module today.
  - `generated_parity_session` declares a 192-frame source, and `render_parity_shape` (`:394-470`)
    submits it once with `end_of_region` and seeks at block 3. Do not copy that feeding pattern:
    most of its blocks are an effect tail or silence, which would let a mute gate pass trivially.
- **The reliable lane has two slots** (`crates/capi/src/runtime/compile.rs:98`). A test that makes
  more than two edits must dequeue the reliable events between them, or the third edit returns
  `BACKPRESSURE` for the reliable-event queue.

## Decisions

- **D1. The control flow.** After the existing `plan_alive` and response-size checks, call
  `classify_live_delta` with:
  - the committed model, `self.controller.session().compiled().normalized_model()`;
  - the prospective model, `prepared.get().prospective_session().compiled().normalized_model()`;
  - `LiveRamps::for_session` of the prospective model.

  `Err(_)` takes the existing structural path, unchanged. The epoch-lag check moves below the
  classification and applies to that path only. `Ok(delta)` calls a new `commit_live`.
- **D2. `commit_live`.** The steps run in #1053 D6's order:
  1. **Admission (D5).** A refusal is `CompileRejected`.
  2. **Resolve.** For each strip in the delta, find its producer in the newest epoch's
     `strips.controls[..track_count]` by `strip_id` equality. A miss is `Internal`.
  3. **Room.** Every queue the delta touches needs room: `fader.available_capacity()` at least the
     strip's fader-record count, and `producer.available_capacity()` at least its matrix-record
     count. Too little room is `Backpressure`.
  4. **Check.** `self.controller.check_prepared_structural(prepared.get())` (D3). A failure is
     `Internal`.
  5. **Push** every record. Treat a failed `try_push` as unreachable (`expect` with a message):
     the room was checked, and the control thread is the only producer.
  6. **Commit** through `ObservedPreparedToken::commit`. Treat a failure as unreachable
     (`expect`), after step 4.
  7. **Respond.** Write the committed response, as the structural arm does.

  Steps 1-4 change nothing on failure. Do not call `replace_session_catalog`, reserve a plan or add
  a report row.
- **D3. The protocol check.** Add `pub fn check_prepared_structural(&self, prepared:
  &PreparedStructuralCommand) -> Result<(), PreparedCommandCommitError>`. It is exactly the
  predicate `commit_prepared_structural` uses today, and that function now calls it first, so there
  is one definition.
- **D4. The epoch keeps its resources.** `ProviderEpoch` gains `capi: CapiResources`, set from
  `PreparedRuntime::capi` at compile and at every replacement.
- **D5. Admission.** Add `validate_live_peak` beside `validate_replacement_peak` in `compile.rs`.
  It implements #1053 D8, term for term:
  - **graph:** the `graph_session_plus_plan_bytes` of the current epoch's report row and of a
    pending epoch's row, plus `compiled_model_admission(current, prospective).retained_bytes`;
  - **capi:** the newest epoch's report-row `capi_retained_bytes`, plus the current epoch's
    `CapiResources::epoch_retained` when a candidate is pending, plus the newest epoch's
    `CapiResources::prepared_protocol_retained`. That last term includes a catalog
    (`catalog_retained_bytes`, `compile.rs:190-193`) the live arm never builds: a conservative
    overcount. Keep it and say so in the function's documentation;
  - **largest allocation:** the maximum of both plans' `largest_named_allocation_bytes`, the newest
    epoch's `CapiResources::largest` and `compiled_model_admission(..).largest_allocation_bytes`.
- **D6. Documentation.**
  - The header comment (`crates/capi/include/miso_engine_v1.h`, after the thread-ownership
    block) gains a paragraph. It covers:
    - which transactions are value-only;
    - the D2 timing of #1053;
    - `BACKPRESSURE` on a full lane, with the model unchanged;
    - draining the reliable lane after each edit;
    - how a host still detects a rebuild.
  - `docs/C_ABI_V1_QUALIFICATION.md` gains the same contract as a section.
  - `docs/CONTROL_PROTOCOL_SEMANTICS.md`'s delivery-status paragraph (`:15`) says #1053 delivers
    it, and that every other transaction still replaces the plan.
- **D7. Test-only fault phase.** Optionally, add `TestStructuralFaultPhase::BeforeLivePush`, after
  step 4, for gate 3.
- **D8. A distinct diagnostic.** A full live lane returns a new `CommandError::LiveBackpressure`.
  The boundary (`crates/capi/src/ffi.rs:625-631`, beside `Backpressure`) maps it to
  `RESULT_BACKPRESSURE` with the last error `control.live.backpressure`, so a host (and #1258) can
  tell it from `control.plan.backpressure`. The result code is unchanged.

## Authorized paths

- `crates/capi/src/runtime/control.rs`, `compile.rs`, `mod.rs` and `tests.rs` (helpers may become
  `pub(super)`).
- `crates/capi/src/runtime/live_tests.rs` (new; register it in `mod.rs` under `#[cfg(test)]`).
- `crates/capi/tests/resource_lifecycle.rs`, only if `ProviderEpoch`'s layout row needs it.
- `crates/protocol/src/controller.rs`: `check_prepared_structural` only.
- `crates/capi/include/miso_engine_v1.h`: the comment only.
- `crates/capi/src/ffi.rs`: the one `LiveBackpressure` match arm only (D8).
- `docs/C_ABI_V1_QUALIFICATION.md`, `docs/CONTROL_PROTOCOL_SEMANTICS.md`.
- This spec.

## Non-goals

- No ramp length other than `LiveRamps::for_session` (#1054).
- No submix strips, routes or VCAs (#1225, #1226 and #1247), and no effects or input section
  (#1261-#1266).
- No concurrency race, live-cap oracle or audit change: those are #1258.
- No new symbol, opcode, field, result code or event.

## Hazards

- **The epoch-lag check must not run for the live arm.** During the lag the newest provider is the
  plan that is rendering (#1053 D7). Refusing there would make a live edit fail for no reason.
- **Resolve by ID, never by index into the model.**
- **Never emit a redundant record.** The classifier ensures it; do not add records in capi.
- **A record pushed before a refused commit** would play an unacked value. That is why D2 runs
  every fallible step before the first push.

## Objective gates

Run every command from the repository root.

1. **Live PCM.** New module `crates/capi/src/runtime/live_tests.rs`. Run it for 1 and 10 tracks,
   at 44.1, 48, 88.2 and 96 kHz, on `generated_parity_session` with its source made long enough
   for the whole run. Make each edit through `miso_engine_v1_submit_command`.
   - **The source.** Submit one quantum per block for the whole run: a continuous signal with no
     zero sample, distinct on each lane. Never seek and never set `end_of_region`.
   - **The window.** Let E be the block after the edit and
     K = ceil((`latency_samples` + the largest `smoothing_samples` of the edit) / quantum) + 1.
     Keep each matrix edit's `smoothing_samples` at most one quantum.
   - **Non-vacuous.** In every compared window, the reference output must be non-zero; assert it.
   - **(a) Mute.** A `SetTrackFader` that mutes every track returns `OK`, and the revision rises by
     one. `providers.epoch` and `pending_providers` are unchanged, so no plan was prepared. There
     is no seek and no resubmission. From block E + K on, the output is exactly `+0.0`.
   - **(b) Unmute.** From block E + K on, the output is bit-identical to a lanes-free
     `host_core::prepare_host_runtime` plan of the original compiled session, fed the same source
     from sample 0.
   - **(c) Fader and pan.** `-6` dB left and `+3` dB right on one track, and a
     `SetTrackMatrixOrPan` with an asymmetric `Matrix` on another. From block E + K on, the output
     is bit-identical to a plan compiled from the committed snapshot (`SessionSnapshotGet`) and fed
     the same source from sample 0.

   *Test value: it turns red if a value-only edit still rebuilds, if a record lands on the wrong
   track or lane, or if the live value differs from what preparation bakes.*
2. **Persistence.** After a live mute:
   - `SessionSnapshotGet` returns the mute.
   - A structural `SetSourceContent`, on a source no track reads or with a changed content string,
     then replaces the plan. After a seek and a resubmission, the new plan's output is
     bit-identical to a fresh plan of the final snapshot (muted).
   - A control run without the live mute renders non-zero output.

   *Test value: it turns red if a live edit is missing from the committed model, so that a rebuild
   would revert an acked edit.*
3. **No ack before a drop.** Dequeue the reliable events after every edit.
   - **(a) A full lane.** 16 single-record edits on one track, with no render between them, return
     `OK`. The 17th returns `BACKPRESSURE`, with last error `control.live.backpressure` (D8). The
     model's canonical bytes, the revision, the replay length, the reliable-event occupancy and
     that track's fader-queue room are all unchanged. After one render, a retry with a new request
     ID returns `OK`.
   - **(b) All or nothing.** A transaction that edits two tracks, where only one track's queue is
     full, returns `BACKPRESSURE`. The other track's queue room is unchanged.
   - **(c) Replay.** An exact replay of a live edit returns the cached response, and no queue room
     changes.
   - **(d) A pending candidate.** A live edit while a structural candidate is pending returns `OK`.
     The candidate's queue room falls and the current plan's does not. After the swap, the
     candidate renders the value.

   *Test value: it turns red if a record is pushed before every queue's room is checked, if a
   refused edit commits, or if a pending candidate misses an acked edit.*
4. **The boundary of liveness.** Each of these produces a new epoch, which is a rebuild:
   - a fader edit in a session with a VCA (G3);
   - a mute of a track that a `follows_mute` send follows (G2);
   - a submix fader edit (G1);
   - a `SetSourceContent`.

   A fader at 30 dB returns `COMPILE_REJECTED` with today's builtins diagnostic, and pushes nothing.

   *Test value: it turns red if a guard is skipped, which would render something other than the
   committed model, or if a domain failure reaches a queue.*
5. **The admission formula.** A unit test of `validate_live_peak` in `compile.rs`. Build reports
   whose live peak is exactly each cap, with and without a pending epoch: the function accepts at
   the cap and refuses one byte below it, once per term. (#1258 repeats this end to end through
   the C entry point.)
   *Test value: it turns red if a term of #1053 D8 is missing or counted against the wrong plan.*
6. **Nothing else changes.**
   - `cargo test --locked -p capi`: every existing test passes.
   - `cargo test --locked -p protocol --features test-support`
   - The workspace test command:
     `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi` reports zero allocations, frees, locks and syscalls.
   - `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`
   - `python3 -B scripts/check-scalar-oracle-absent.py --native target/release/libcapi.so`
   - The `resource_lifecycle` oracles change only by `ProviderEpoch`'s new field.
7. **Workspace and policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `for x in host-core realtime workspace protocol-control; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - 4-lane (NEON): `bash scripts/run-aarch64-tests.sh debug` is CI-only here (the `aarch64-debug`
     job); record it as not run locally.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.
- The documentation diff.

## Dependencies

- *Bound the builtin fader and matrix drains to the records present at block entry* (#1253)
- *Classify a committed session delta as a live track fader, mute and pan update or a rebuild*
  (#1255)
- *Prepare C ABI plans with live track fader and matrix lanes* (#1256)

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- No ack precedes a drop. Every fallible step runs before the first push.
- "Bit-identical" gates are hard stops.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Attempt record

### Attempt 1 (implementer, on `f02d09b24`)

**What changed.**
- `protocol::ProtocolController::check_prepared_structural` (D3): the commit predicate, factored
  out; `commit_prepared_structural` calls it first, so there is one definition.
- capi `control.rs`: `SessionState::commit_live` (D1, D2) is called from one branch point in the
  structural arm, after `plan_alive` and the response-size check. It classifies, and a rebuild gets
  the token back untouched (`LiveCommit::Rebuild`); the epoch-lag check now sits below that branch
  and guards the rebuild path only. The live steps run in #1053 D6's order: admission
  (`live_admission` -> `validate_live_peak`, `COMPILE_REJECTED`), resolution by strip ID in the
  newest epoch (`INTERNAL` on a miss), room on every touched queue (`LiveBackpressure`), the
  protocol predicate (`INTERNAL`), then the pushes (`unreachable!` on a full queue), the commit
  (`expect`) and the response. Nothing is mutated before the last check; no plan is reserved, no
  report row added and the catalog is not replaced. `ProviderEpoch` gains `capi: CapiResources`
  (D4), set at compile and at every replacement; #1256's `#[allow(dead_code)]` on `StripLanes` and
  `ProviderEpoch::strips` are gone and the ownership wording is corrected. Test-only:
  `TestStructuralFaultPhase::BeforeLivePush` (D7) and a `live_rooms` field on
  `TestTransactionSnapshot`.
- capi `compile.rs`: `LiveEpochResources` and `validate_live_peak` (D5), term for term from #1053
  D8, with the catalog overcount documented.
- capi `ffi.rs`: the `LiveBackpressure` arm (D8: `RESULT_BACKPRESSURE`,
  `control.live.backpressure`). **Outside the authorized paths, minimum change:** a
  `#[cfg(test)] fn test_last_error` beside the other `test_*` hooks. Gate 3 must read the last
  error, `miso_engine_v1_last_error` needs `unsafe`, and the realtime policy confines `unsafe` to
  `ffi.rs`, so the only safe reader is a hook there.
- Docs (D6): a "Live edits" paragraph in `miso_engine_v1.h` after the thread-ownership block; a
  "Value-only track edits on the running plan (#1257)" section in `C_ABI_V1_QUALIFICATION.md`; the
  delivery-status paragraph of `CONTROL_PROTOCOL_SEMANTICS.md` now says #1053 delivers it and that
  every other transaction still replaces the plan.
- `tests.rs`: the shared helpers became `pub(super)`.

**New tests and their test value** (each answer was proven by the mutation run listed under it:
the defect was introduced, the test went red, and the defect was reverted).
- `live_fader_mute_and_pan_edits_change_the_running_plan_bit_exactly` (gate 1; 1 and 10 tracks
  at 44.1/48/88.2/96 kHz; continuous, never-zero source with distinct lanes; no seek): red if a
  value-only edit still rebuilds, a record lands on the wrong track or lane, or the live value
  differs from what preparation bakes. M1, the live arm always rebuilds: red (`(43, 1, 0, 1)`,
  a candidate was prepared). M3, records pushed to the next track: red (10 tracks, edited block
  25).
- `a_live_track_edit_beside_a_submix_strip_reaches_its_track` (gate 1 on a session with a submix
  strip, added at the #1256 verifier's request): red if strip resolution confuses the
  tracks-then-submixes table. M3b, index + 1 (eq9 resolves to `bus`): red. The bus is stateless
  (filters off, console EQ bypassed): with a stateful bus, the live run's bus filter memory
  holds pre-edit input forever, so no window makes it bit-equal to a plan started with the edit.
  This was diagnosed during the attempt (the live pan was applied, the record consumed, and the
  difference persisted 30 blocks only through the bus EQ) and is a property of the comparison,
  not a defect.
- `a_live_mute_survives_a_later_rebuild` (gate 2): red if a live edit is acked without reaching
  the committed model. M9b, acknowledge without committing the token: red ("SessionSnapshotGet
  returns the live mute"). The control run without the mute renders non-zero and matches its own
  fresh plan, which also proves the seek/resubmission alignment.
- `a_full_live_lane_refuses_before_anything_changes` (gate 3a): red if the room check is missing
  or off by one, or the diagnostic is the plan's. M5, no room check: red. M6, `<=` for `<`: red
  on edit 15. M7, `control.plan.backpressure`: red.
- `a_live_transaction_with_one_full_lane_pushes_to_no_lane` (gate 3b; the full track is second in
  delta order): red if a strip is pushed as soon as its own room checks. M4, push-as-you-check:
  red.
- `a_replayed_live_edit_pushes_nothing` (gate 3c): red if a replayed live edit reaches the live arm
  and pushes again. No capi-side mutation reaches it: the protocol answers a replay before a token
  exists, by two independent checks (stale exact revision, then the replay preflight). It is kept
  as the spec's invariant guard; its mutation was not run. (Corrected after the verdict, NIT 2:
  a capi-side mutation does reach it -- a live response that diverges from the committed frame;
  see the follow-ups below.)
- `a_live_edit_while_a_candidate_is_pending_reaches_the_candidate` (gate 3d): red if a pending
  candidate misses an acked edit. M2, push to the current provider: red (candidate room
  `(16, 16)`).
- `deltas_outside_the_live_set_rebuild_and_a_domain_failure_pushes_nothing` (gate 4; VCA, followed
  mute, submix fader, source content; 30 dB fader is `COMPILE_REJECTED` with exactly the
  diagnostic `compile_children` gives that session, no push): red if capi feeds the classifier the
  wrong models. M9, `classify(next, next)`: red (followed mute did not rebuild).
- `a_fault_before_the_live_push_leaves_every_queue_and_the_model_alone` (D7): red if any push moves
  above the last fallible check. M8, the pushes moved above the fault point: red (room `(15, 16)`).
- `the_live_admission_accepts_each_cap_and_refuses_one_byte_below` (gate 5, in `compile.rs`; every
  term its own bit, each cap at the peak and one byte below, with and without a pending epoch;
  for the largest allocation each term in turn is the strict maximum, and the current epoch's capi
  `largest` is a decoy while a candidate waits). M10, pending graph term dropped: red. M11, capi
  row of the current plan instead of the newest: red. M12, current epoch's capi `largest`: red.
  M13, compiled-model largest dropped: red. M14, current `epoch_retained` dropped: red.

**Gates** (on the implementation tree; logs in the implementer's scratch directory).
- `cargo test --locked -p capi`: 46 + 11 passed (re-run after the submix test).
- `cargo test --locked -p protocol --features test-support`: 146 passed.
- The workspace test command: 1,324 passed, 0 failed.
- `cargo build --locked --release -p audit -p bench -p capi -p session-validator`: pass.
  `./target/release/audit capi`: 100,000 calls, allocations 0, deallocations 0, locks 0,
  syscalls 0, total_violations 0.
- `check-capi-abi.sh` and `--self-test`: pass. `check-scalar-oracle-absent.py --native
  target/release/libcapi.so`: pass.
- `resource_lifecycle`: 11 passed; no oracle needed an edit for `ProviderEpoch`'s new field.
- `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: pass.
- host-core, realtime, workspace and protocol-control policy check + self-test: pass.
  `check-cross-targets.sh`: pass. `check-ci-path-routing.py`: pass.
- Worklet chain: not run. `protocol` and capi are not in the browser module (`host-web` does not
  enable `host-core/control-provider`, and `cargo tree -p host-web --target wasm32-unknown-unknown`
  lists no `protocol`).
- 4-lane NEON (`run-aarch64-tests.sh debug`): CI only, not run locally.

**Notes for the verifier.**
- The epoch-lag check now follows the response-size check, so a structural edit that is both too
  large for the caller's buffer and inside the lag now reports `BUFFER_TOO_SMALL` before
  `BACKPRESSURE`. D1 orders it so.
- Resolution scans the track producers from where the previous strip was found, wrapping, so it
  is linear when the delta and the table share canonical order and still correct when they do not.
- No block-atomicity is claimed anywhere; the docs state #1053 D2's one-quantum skew.
- The skipped epoch-lag check during a lag, and every producer/render race, are #1258's.
  (Corrected after the verdict, MINOR 2: the skipped lag check is this slice's own first hazard,
  and #1258's race gate retries every `control.plan.backpressure`, so it would not catch a lag
  check moved above the live arm. A deterministic in-window test now covers it; see the
  follow-ups below. Producer/render races remain #1258's.)

### Follow-ups applied (after the attempt 1 PASS, verdict MINOR 1-4 and NIT 2-4, NIT 6)

All tests are in `crates/capi/src/runtime/live_tests.rs`; each mutation was applied, run with
`cargo test --locked -p capi --lib`, and reverted from a saved copy.

- MINOR 1 (room-check terms). `a_full_live_lane_refuses_before_anything_changes` now also fills
  eq3's matrix lane with 16 distinct pan edits (room `(16, 0)`); the 17th returns `BACKPRESSURE` /
  `control.live.backpressure` with the refusal state and every current room unchanged and no plan
  prepared. It then leaves eq4's fader lane room 1 and sends one edit whose left and right dB
  differ (two `FaderDb` records): `BACKPRESSURE`, refusal state and rooms unchanged, room still 1.
  Test value: red if the room check drops its matrix term or counts edits instead of fader records.
  Mutations: matrix term `|| false`: red (the push hits `unreachable!`, live_tests.rs:618);
  fader term `count().min(1)`: red (live_tests.rs:647). Both were green on every capi test before.
- MINOR 2 (epoch lag). New `a_live_edit_inside_a_plan_swapping_render_call_commits_without_a_candidate`,
  modelled on `control_calls_inside_a_plan_swapping_render_call_keep_replacement_live`: a
  structural edit prepares a candidate, `owner.render_contiguous` swaps it in while
  `active_epoch` still names the retired plan, and a value-only `SetTrackFader` inside that window
  returns `Ok`, commits one revision, prepares no candidate (no pending epoch) and takes exactly one
  record from the promoted provider's eq0 fader lane, with the atomic still lagging. Test value:
  red if the epoch-lag check runs before the live arm. Mutation: the lag check copied above
  `commit_live`: red (`Err("Backpressure")`, revision unchanged), unique in the capi suite. The
  attempt-record sentence above is corrected.
- MINOR 3: `docs/C_ABI_V1_QUALIFICATION.md`'s #1257 section states the +96 B
  `capi_retained_bytes` per session (`ProviderEpoch`'s 32-byte `CapiResources`, inline current
  epoch plus two reserved slots; 258,135 -> 258,231 on the nine-track reference, the verifier's
  measurement) and that exact-cap callers must raise `maximum_capi_retained_bytes`.
- MINOR 4 / NIT 6: gate 1(a)'s mute window now asserts that the unmuted reference carries signal
  in every compared block. Test value: red if the mute window's input goes silent, which would make
  the exact-`+0.0` check vacuous. Mutation: `source_sample` returns 0.0 for frames below 6 * 128
  (silencing the warm-up and the mute window): red at the new assertion ("1 tracks at 44100 Hz:
  block 5: the reference carries signal in the mute window"); the same mutation on the
  pre-change test is green.
- NIT 2 (gate 3c's test value). `a_replayed_live_edit_pushes_nothing` is red if the live arm's
  response diverges from the committed frame the replay cache serves, the plausible defect behind
  owner question Q3 (a live/rebuild flag in the response). Mutation: flip the last response byte
  after `committed.write_into` in `commit_live`: red at live_tests.rs:715 (`replay == first`),
  and no other capi test. This replaces the "its mutation was not run" note above.
- NIT 3 (events). New `a_live_edit_emits_the_commit_events_of_a_rebuild`: a live edit emits
  exactly one `SESSION_COMMITTED`; with one automation batch queued (handle 5, accepted), the next
  live edit emits `SESSION_COMMITTED` then `AUTOMATION_CANCELED`. Test value: red if a live edit
  commits through a path that loses or skips the protocol commit's events. Mutations: (i) the
  live arm dequeues one reliable event after its commit: red here (and in #1258's
  `a_live_edit_without_reliable_event_room_...`, incidentally); (ii) the protocol commit skips
  `cancel_queued_automation_reserved`: red here and in
  `all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes`, no protocol test red. Not a
  unique catch for the protocol-level defect; it pins #1053 D10 for the live arm, as the verdict
  asks.
- NIT 4: the header's live-edit paragraph says "takes the replacement path exactly as before"
  (comment only).
- Gates on the follow-up tree (`978463341`, x86-64-v3, one run for the whole #1253-#1257
  follow-up set): `cargo fmt --all -- --check`; `cargo test --locked` for `host-core
  --all-features`, `builtins`, `builtins-compiler`, `capi` and `host-web`; workspace clippy
  `--all-targets --all-features -D warnings`; `cargo doc` with `-D warnings`; release build and
  `./target/release/audit capi` (100,000 calls, allocations 0, syscalls 0, total_violations 0);
  `check-capi-abi.sh`; realtime, workspace and host-core policy (check, plus the realtime and
  workspace self-tests); `check-cross-targets.sh` (iOS `memset_pattern16` expected failures
  unchanged): all pass. Worklet chain not run: no line compiled into the browser module changed
  (host-core, host-web, builtins and builtins-compiler changed only in docs, comments and
  tests).

### Verdict

**Verdict.** Sol attempt 1: PASS (verdict file `docs/handoffs/live-updates-1053/1257-attempt1.md`). MINOR 1 (room-check terms), MINOR 2 (the live arm skips the epoch-lag check), MINOR 3 (+96 B doc) and MINOR 4 (non-zero reference in the mute window), with NIT 2-4, applied in `978463341` (gates `0539791a1`).
