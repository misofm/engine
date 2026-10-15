# Subscribe to an effect's observation tap with a transaction edit

Slice S25 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1,
C9 and C13). It delivers the observation half of the #1057 design note's P5
(`docs/handoffs/one-edit-api/1057-design-note.md:1088`), for which no issue was filed. Code anchors
verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal
tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

A C ABI app arms and disarms an effect's observation tap, such as a compressor's gain reduction,
with one `Observe` edit in `SESSION_TRANSACTION_APPLY`, addressed by the strip ID and the console
slot or insert ID. The core keeps the subscriptions in a monitor overlay: never saved, no revision,
reply path `monitor`. Several subscribers share one arm when they ask for the same window. A
subscription survives every plan rebuild while its effect exists. A mobile app gets the
subscriptions the browser has today. The browser moves onto the same edit in #1589.

## Context

- **The browser's subscriptions.** Kinds 7 and 8 (`hosts/host-web/src/lib.rs:861`, `:863`) carry a
  strip index, a rack, an effect index and the tap ID in `parameter_id`; `into_observe_record`
  (`:4649`) checks the tap (unknown `COMMAND_REASON_UNKNOWN_TAP`, computed cost
  `COMMAND_REASON_UNSUPPORTED_KIND`, no lane `COMMAND_REASON_OBSERVATION_UNBOUND`); the admission
  (`:5446-5500`) refuses a second record for one tap in one submission; `ReadyOwnership::push`
  (`:1936`, the `Observe` arm `:1956-1985`) pushes the record and writes the arm mask and arm
  sample. #1565 leaves these two kinds as the only browser command kinds.
- **The lane.** `EffectControlRecord::Observe { tap_index, armed, window_blocks }`
  (`crates/effect-contract/src/live.rs:123`) rides the effect's observation queue, one slot per
  declared tap; `EffectControlProducer::try_push_observe` (`crates/effect-compiler/src/prepare.rs:860`),
  `observe_available_capacity` (`:879`). Render applies it at block entry (`apply_observe`,
  `live.rs:1104-1112`; `ObservationLane::arm`, `:1258`), which restarts the window at the block's
  first sample. A re-arm wins with its new window.
- **No C ABI path.** `compile.rs:616-619` (`crates/control-plane/src/compile.rs`) asks for no
  observation capacity (`observation_taps` `0`), and `:646-651` drops `effect_observations`. Host-core
  refuses capacity without effect lanes (`crates/host-core/src/prepare.rs:1690-1700`); the C ABI's
  plans have effect lanes.
- **A rebuild drops subscriptions.** Each preparation builds every tap unarmed, and
  `ObservationLane::disarm_all` documents that a plan replacement drops them (#143 D7);
  `crates/host-core/tests/effect_observation.rs:511` pins it at host-core level.
- **The monitor class.** #1559: `SET_CONTROL` monitor edits are recorded on the prepared token,
  a monitor-only transaction keeps the revision and replies `EditPath::Monitor`, the overlay is
  written after every fallible step, and the rebuild arm prepares the successor with the overlay.
- **The SDK shares arms.** `ObservationSubscriptionOwner` (`sdk/src/core/observation-subscriptions.ts:471`)
  counts references per tap and window (`refs`, `:206`, `:589-635`) and refuses a second window on
  a held tap ("observation window conflicts with a managed subscriber", `subscribe` `:547`).
- **The frozen op.** Opcode `0x0801`, addressed by the decision-17 address's owner (strip) and
  section (console slot or insert), no control field, plus `tap_id`, `armed`, `window_blocks`.

## Decisions frozen for this slice

- **D1. The edit.** `SessionEdit::Observe { strip_id, section: ObserveSection, tap_id: u32,
  armed: bool, window_blocks: u32 }`, `ObserveSection::{Console(slot_id), Inserts(effect_id)}`,
  opcode `SessionEditOpcode::Observe = 0x0801`. Schema `observe`: 1 `address` `Message` (#1551's
  `control_address` with `owner_kind` strip, `section` console or inserts, `slot_id` present,
  `control` and `lane` absent), 2 `tap_id` `U32`, 3 `armed` `Bool`, 4 `window_blocks` `U32`
  (`0`: the plan's meter window, #1581 D2). Any other address shape is `INVALID_FIELD`.
  `ramp_samples()` is `None`. It is never saved and never in a snapshot.
- **D2. Target check in the core.** `controls::observe_target(model: &SessionModel, edit) ->
  Result<ObserveTarget { address: LiveEffectAddress, tap_index: u32 }, ObserveError>`, run by the
  transaction apply against the prospective model at the edit's position (so an `Observe` after an
  `UpsertTrack` in the same transaction can address the new effect). Codes, each
  `VALIDATION_FAILED` with path `edits[i]`: `observation.address.unknown` (no such strip, slot or
  insert), `observation.tap.unknown` (no declared tap with that ID, `0` included),
  `observation.tap.unsupported` (a tap whose cost is not `Resident`), `observation.window`
  (`window_blocks` above 4096). The edit writes nothing to the model; the token records
  `MonitorEdit::Observe { key: TapKey, target: ObserveTarget, armed, window_blocks }` in transaction
  order (#1559 D3's list). `TapKey { strip: StableId, section: SectionKey, tap_id: u32 }`.
- **D3. The binding table shares arms.** The bindings are endpoint-local monitor state, held where
  the transaction is checked: `ProtocolController` owns `ObservationBindings` beside its
  `telemetry_configuration` (`crates/protocol/src/controller.rs:1271`), a sorted vector of `TapKey`
  to `TapBinding { window_blocks: NonZeroU32 (resolved), refs: u32 }`. It is never saved and never
  in a snapshot. The transaction apply applies the `Observe` edits in order to a scratch copy:
  - arm, no binding: a binding with `refs` 1;
  - arm, binding with the same resolved window: `refs + 1`;
  - arm, binding with another window: refused `observation.window_conflict`;
  - disarm, no binding or another window: refused `observation.not_subscribed`;
  - disarm: `refs - 1`, and at `0` the binding goes.
  Each refusal is `VALIDATION_FAILED` with path `edits[i]`, the existing transaction refusal form.
  The token carries the scratch copy and the net records: at most one per tap (no binding to
  binding: arm; binding to none: disarm; a window changed by a disarm and an arm in one
  transaction: one arm with the new window, a re-arm). `commit_prepared_structural` installs the
  copy; a token that is dropped changes nothing.
- **D4. Room, then commit (the acked-batch question).** Before it commits the token, the control
  plane checks, for each effect, that the newest plan's
  `EffectControlProducer::observe_available_capacity()` holds that effect's net records. If not, it
  drops the token and returns `CommandError::Backpressure` with the diagnostic
  `observation.backpressure`, exactly as a full plan exchange returns `control.plan.backpressure`
  today (retryable after a render drains the queue; nothing is recorded). After every fallible step
  (#1559 D8) it commits the token and pushes the net records; a push cannot fail after the room
  check, because the control thread is the only producer. A monitor-only transaction replies
  `EditPath::Monitor` with the revision unchanged (#1559 D4); a mixed one follows #1559 D5. Every
  C ABI plan has an observation queue for every declared tap (D7), so a missing queue is
  `CommandError::Internal`.
- **D5. Pending plans.** Records go to the newest plan's lanes, as every live record does
  (D15-2); while a replacement plan is pending, the subscription takes effect when it is adopted.
- **D6. Every plan the control plane prepares carries the overlay** (P5). Each preparation entry
  that the control plane calls takes `armed_taps: &[ArmedTap]`, `ArmedTap { strip_id: Box<str>,
  address: LiveEffectAddress, tap_index: u32, window_blocks: NonZeroU32 }`, resolved from the
  overlay against the next model, and `attach_effect_observation` builds those taps armed: each
  opens its first window at its first rendered block. A tap whose lane carries (#1327 D1, D2) takes
  the predecessor's state instead, so it continues with no restart. In the same transaction apply,
  the controller drops each binding whose strip, slot or insert the prospective model no longer
  has (it is part of the token's scratch copy). The control plane reads the committed table
  (`ProtocolController::observation_bindings()`) to build `armed_taps`.
- **D7. Capacity on every C ABI plan.** The control plane's preparation asks for every declared
  tap: `observation_taps = host_core::launch_maximum_observation_taps()` (the largest tap count of
  any launch effect, computed from the registry). `PreparedRuntime` keeps
  `effect_observations: Box<[EffectObservationHandle]>`. The browser keeps its boot word until
  #1590.
- **D7a. The browser waits for #1589.** Until #1589, kinds 7 and 8 push `Observe` records from the
  browser's own admission, and two producers must never write one observation queue. So
  `ProviderFeatures` gains `observation: bool` (the existing per-family switch,
  `crates/protocol/src/controller.rs:110`). When it is false, the transaction apply refuses an
  `Observe` edit with `VALIDATION_FAILED`, code `observation.unavailable`, path `edits[i]`. The C ABI
  sets it; the browser leaves it clear until #1589. Capability bit 15 stays clear until #1586. This
  order is forced by the browser's old path; it is not a placeholder.
- **D8. Docs.** `docs/CONTROL_PROTOCOL_REGISTRY.md` lists opcode `0801` and its payload;
  `docs/CONTROL_PROTOCOL_SEMANTICS.md` states the monitor class of `Observe` and D3's sharing
  rule; the header prose and `docs/C_ABI_V1_QUALIFICATION.md` describe the subscription. No C
  symbol changes.

## Deliverables

1. D1, D3 and D7a in `crates/protocol` (edit, opcode, schema, codec, the binding table on the
   controller and the token, `ProviderFeatures::observation`, the corpus).
2. D2 in `crates/controls`.
3. D4 and D5 in `crates/control-plane/src/control.rs`.
4. D6 in `crates/host-core/src/prepare.rs` and `crates/effect-compiler/src/prepare.rs`
   (`attach_effect_observation`), with the control plane's preparation calls.
5. D7 in `crates/control-plane/src/compile.rs` and host-core; D8.

## Authorized paths

- `crates/protocol/src/{model.rs,session_wire.rs,schema.rs,controller.rs,controller/tests.rs,lib.rs}`,
  `crates/conformance/src/protocol_corpus.rs`, `fuzz/corpus/complete-schema-manifest.md`.
- `crates/controls/**`.
- `crates/control-plane/src/{control.rs,compile.rs}`.
- `crates/host-core/src/{prepare.rs,lib.rs}`, `crates/host-core/tests/effect_observation.rs`.
- `crates/effect-compiler/src/prepare.rs` (`attach_effect_observation` only).
- `crates/capi/src/runtime/{tests.rs,live_tests.rs}`, `crates/capi/tests/resource_lifecycle.rs`,
  `crates/capi/include/miso_engine_v1.h` (prose), `tools/audit/src/protocol.rs`.
- `docs/CONTROL_PROTOCOL_{REGISTRY,SEMANTICS}.md`, `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- No arm tag and no render-written arm state (#1585); no read command (#1586).
- No browser change: kinds 7 and 8 keep working until #1590.
- No observation entry in the control list: a subscription is not a value (ruling C9).

## Objective gates

Commands: `cargo test --locked -p protocol --all-features`, `cargo test --locked -p controls`,
`cargo test --locked -p control-plane --features test-support`,
`cargo test --locked -p host-core --all-features`, `cargo test --locked -p capi`,
`cargo test --locked -p conformance`,
`cargo build --locked --release -p audit -p capi && target/release/audit capi`.

1. **Arm on the C ABI.** Fixture: tracks `t1`, `t2`; a compressor insert `c1` on `t1`. One
   transaction with `Observe` arm of `c1`'s gain-reduction tap, window 4: reply `monitor`, revision
   unchanged. After 12 rendered blocks the tap's reader (from `PreparedRuntime`) holds windows equal
   to the scalar reference of `effect_observation.rs` (`scalar_reference_windows`, `:564`) from the
   draining block on.
2. **Sharing.** Two arms with window 4 (in two transactions) queue one record; one disarm leaves
   the tap armed; the second disarm queues the disarm. An arm with window 8 while another
   subscriber holds window 4 is refused `observation.window_conflict` and changes nothing. A
   disarm and an arm with window 8 in one transaction queue one record and re-arm with 8.
3. **Refusals change nothing.** Unknown strip, unknown insert, tap `0`, an unknown tap ID, window
   4097, a disarm with no binding: each is refused with its code at its `edits[i]`, and model,
   revision, binding table and queues are unchanged. A `controls` unit test refuses a test descriptor's
   computed tap with `observation.tap.unsupported`.
4. **Backpressure.** Arm, then disarm before any render: the disarm returns
   `RESULT_BACKPRESSURE` with the diagnostic `observation.backpressure`, and the binding table,
   the revision and the replay cache are unchanged; after one render the same request is accepted.
5. **Rebuilds keep subscriptions.** With the tap armed: commit `UpsertTrack` of a new track (a
   rebuild in which `c1` carries): its windows continue with no gap and no restart. Commit a
   prepared-only change of `c1` (its lane does not carry): the successor's tap is armed and its
   first window opens at the adoption block. Commit the removal of `c1`: the binding goes, and a
   later disarm is `observation.not_subscribed`.
6. **Replay and events.** An exact replay of an `Observe` transaction returns the same bytes and
   queues nothing. A monitor-only transaction emits no `SESSION_COMMITTED`.
7. **Codec.** `Observe` round-trips; an address with a `control` or `lane` field, a route owner or
   a builtins section is refused on decode.
8. **Realtime.** The capi race test (`crates/capi/tests/resource_lifecycle.rs:3364`) also races
   arm and disarm transactions over 20 runs: `allocations == 0 && frees == 0` around every render
   call after warm-up, zero `INTERNAL` results.

## Test value

- Gate 1 turns red if the C ABI cannot arm a tap or arms it at another window or block.
- Gate 2 turns red if two subscribers do not share one arm, or one subscriber's disarm silences
  the other.
- Gate 3 turns red if a malformed subscription is accepted or half applied.
- Gate 4 turns red if an ack precedes a dropped record.
- Gate 5 turns red if a rebuild drops a subscription, restarts a carried tap, or keeps a binding
  for a removed effect.
- Gate 6 turns red if a replayed subscription re-arms or a monitor edit reports a session change.
- Gate 7 turns red if the codec accepts an address the op cannot use.
- Gate 8 turns red if the observation path allocates on render.

## Dependencies

- #1559 (the monitor class, `EditPath::Monitor`, the prepared token's monitor edits), #1553 (the
  transaction op set), #1551 (`control_address`), #1581 (the meter window that `0` means).
- *Carry meter and effect observation state across a plan swap* (#1327) and *Carry live-controlled
  effect lanes across a plan swap* (#1280): a carried tap keeps its state (gate 5).
- *Report each transaction's edit path in its response* (#1313), through #1559.
- `crates/control-plane` and `crates/capi` are stream B's files; this slice rebases after them.

## Supersedes and shrinks

- None. The SDK's own sharing goes in #1590.

## Amendment (controls coordinator, batch 12, 2026-10-15)

Checked against `ea336c039`.

### The defect D6 closes first (root, batch 12)

- **What is dropped today.** A subscribe that the host acknowledged is lost at a swap when its effect
  owner does not carry. The carry drain applies pending `Observe` records only for effect moves: its
  second loop iterates `moves` (`crates/graph/src/runtime.rs:5443-5470`). A record in the queue of a
  restarted owner (a prepared change, such as link mode) or a removed owner is never applied and
  never counted, and the successor's tap is prepared unarmed (#1327b record, "Departures"; #1327
  batch 11 fold). A carried owner keeps its subscription (#1280b drain, then #1327b's
  `Runtime::carry_taps`, `:5494`).
- **Which host can reach it.** Only the browser acknowledges a subscribe today (kinds 7 and 8,
  through `SessionState::push_effect_observe`, `crates/control-plane/src/control.rs:2244`, on the
  newest plan's producer). The C ABI has no subscribe path until D1-D5 and D7. The browser rebuild
  path is test-support only until #1562b-W, so D6 must land before #1562b-W.
- **A candidate that a later rebuild supersedes** cannot hold a subscribe now: a rebuild while a
  candidate waits returns `BACKPRESSURE` (`control.rs:1292`). The slice that makes supersession
  reachable (#1290 (b) or #1310) owns that gate.

### Frozen decisions that do not hold as written

- **D3, where the table lives.** #1559 shipped the monitor overlay as `controls::MonitorOverlay`
  (`crates/controls/src/effective.rs:28`), held at `SessionState.monitor` (`control.rs:403`), and not
  on `ProtocolController`. The monitor edits are `controls::MonitorWrite` (`effective.rs:103`), not
  `MonitorEdit`. `host_core::SuccessorBase` already passes the overlay to every successor preparation
  as `MonitorBase { committed, next, ramp }` (`crates/host-core/src/prepare.rs:1004`, `:1034`).
  - Option 1, as filed: `ObservationBindings` on `ProtocolController`, and a new `armed_taps`
    argument through `RuntimePreparer::prepare` and every host-core successor entry. That gives two
    owners of monitor state and two paths into the preparation.
  - Option 2: the bindings are a field of `controls::MonitorOverlay`. `MonitorOverlay::after` applies
    the observe writes in transaction order and drops each binding whose target the next model does
    not have, and `MonitorChange` returns those ended keys. Host-core arms the taps of
    `MonitorBase.next` when it prepares a successor. No preparation signature changes.
  - **Recommendation: option 2.** D3's sharing rule (refs, `observation.window_conflict`,
    `observation.not_subscribed`) does not change. Only its owner changes.
- **D6, `armed_taps`.** With option 2, `ArmedTap` and the `armed_taps` argument are not needed:
  host-core resolves `MonitorBase.next`'s bindings against the successor's model.
- **D6, "opens its first window at its first rendered block".** At preparation the adoption sample
  is not known. `ObservationLane::arm` takes the window's `first_sample`
  (`crates/effect-contract/src/live.rs:1630`), and `accumulate` keeps it (`:1754`). So the slice
  adds a prepared arm to `ObservationLane`: the tap is armed, and `accumulate` takes the block's
  `first_sample` as the window start on its first block. `swap_state` moves this state with the tap
  table, so a carried tap replaces the prepared arm completely. Do not queue a record into the
  successor's own queue instead: a carried tap would then be re-armed by its first block, and its
  window would restart.
- **D6, the removed owner.** As filed, the binding goes silently. Root needs a typed end. Options:
  - (A) A reliable event, `OBSERVATION_ENDED { event_sequence, revision, strip_id, section, tap_id }`,
    one for each ended binding, queued with the transaction's `SESSION_COMMITTED`. The room for
    all of these events is checked before commit (`required_events`,
    `crates/protocol/src/controller.rs:1897-1901`). If there is not sufficient room, the transaction
    is refused with `BACKPRESSURE` and nothing changes. The end takes effect at the revision's
    watermark. All listeners see it on both hosts (the browser drains the lane since #1570b).
  - (B) A list of ended keys in the transaction reply. Only the sender sees it, and other
    subscribers stay silent.
  - (C) Refuse a transaction that removes a subscribed target (`observation.subscribed`) until every
    subscriber disarms. This is honest, but a mixer cannot remove an insert while a meter shows it,
    and shared refs make this worse.
  - **Recommendation: (A).** A binding also ends when the next model keeps the owner but its tap ID
    does not resolve to a `Resident` tap (an insert ID reused for another effect type).
- **Records left in a predecessor queue that did not move.** With D6, these records do not need a
  counter. The overlay is the record of what was acknowledged, and the successor is prepared from
  it. The queue is not that record. This closes the #1327 batch 11 fold item.
- **D7a.** Holds. Until #1589, the browser's kinds 7 and 8 write the overlay's bindings through
  `push_effect_observe` (absolute set and clear, refs 1). The `Observe` edit is refused on the
  browser (`ProviderFeatures::observation` false), so one host never has two writers of one
  observation queue.
- **D7, the capacity request moved.** Since #1400 the C ABI asks for its live request in
  `CapiPreparer` (`crates/capi/src/runtime/mod.rs:97`, `HostLiveControlRequest::default()`,
  `observation_taps` 0). `prepare_runtime` (`crates/control-plane/src/compile.rs:689`) keeps the
  strip, effect and route producers and drops the other handles (`:713-719`). D7's edits go there.
  Add `crates/capi/src/runtime/mod.rs` to the authorized paths.

### The batch 12 slice: #1584 D6a (lane K), then D6b (lane W)

- **D6a scope.** Option 2's bindings on `controls::MonitorOverlay`, with the ended keys from `after`.
  `push_effect_observe` writes the bindings after a successful push. Host-core arms the bindings of
  `MonitorBase.next` in each successor preparation that attaches observation, with
  `attach_effect_observation` (`crates/effect-compiler/src/prepare.rs:1776`) and the prepared arm on
  `ObservationLane`. `OBSERVATION_ENDED` (option A) is added to the protocol wire, schema, corpus,
  and `docs/CONTROL_PROTOCOL_{REGISTRY,SEMANTICS}.md`, with its room check. D6a does not include
  D1-D5, D7 or D8's C ABI parts. The C ABI keeps `observation_taps` 0, so its overlay has no
  bindings yet.
- **D6a gates** (control-plane test-support, with a test preparer that asks for observation
  capacity; and host-core `successor_swap.rs`):
  1. Restarted owner, subscribe already applied: compressor insert `c1` on `t1`, gain-reduction tap
     armed with window 4, 6 blocks rendered. Commit a prepared-only change of `c1` (link mode). After
     adoption, the successor's tap is armed with window 4, its first window opens at the adoption
     block's first sample, and every window equals a fresh plan of the committed model with the tap
     armed at that block.
  2. Restarted owner, subscribe still pending (root's case): the same, with the arm admitted after
     the last block before the swap, so that it is still in the predecessor's queue at the swap
     block. The result is the same as gate 1.
  3. Pending disarm on a restarted owner: the tap was armed before, and the disarm is still in the
     queue at the swap. The successor's tap is unarmed and publishes nothing.
  4. Removed owner: commit the removal of `c1`, then a variant that removes `t1`, then a variant
     that reuses `c1`'s ID for an effect without that tap. The binding is gone. The reliable lane
     holds `SESSION_COMMITTED` and one `OBSERVATION_ENDED` with the committing revision and the
     stable key. The successor has no such tap.
  5. Room before commit: the reliable lane has room for `SESSION_COMMITTED` but not for the ended
     event. The removal is refused with `BACKPRESSURE`, and the model, revision, overlay and plan do
     not change. After a drain, the same request commits.
  6. Carried owner: #1327b's `observation_carry` tests stay green. A carried tap that the overlay
     arms continues its window sequence with no restart: the prepared arm never wins over the
     swapped state.
  7. Realtime: zero allocations and frees on the swap block and the successor's first block. The
     `ObservationLane` change stays inside its `REALTIME_POLICY` region, and `realtime-policy`
     passes.
  - `crates/host-core/tests/effect_observation.rs:534` (`a_plan_replacement_drops_every_subscription`,
    E13) is superseded. Rewrite it: a replacement prepared with the overlay keeps the subscription,
    and a replacement prepared with `MonitorBase::NONE` starts unarmed.
- **D6a authorized paths:** `crates/controls/src/effective.rs`,
  `crates/control-plane/src/control.rs`, `crates/host-core/src/prepare.rs`,
  `crates/effect-compiler/src/prepare.rs` (`attach_effect_observation` only),
  `crates/effect-contract/src/live.rs` (`ObservationLane` only),
  `crates/protocol/src/{message_wire.rs,schema.rs,controller.rs,lib.rs}`,
  `crates/conformance/src/protocol_corpus.rs`, `fuzz/corpus/complete-schema-manifest.md`, the
  host-core and control-plane tests above, and `docs/CONTROL_PROTOCOL_{REGISTRY,SEMANTICS}.md`.
- **D6b (lane W, after D6a).** `build_plan_shape` (`hosts/host-web/src/lib.rs:5437`) sets
  `observation_armed` for the bindings of `MonitorBase.next`, with arm sample 0 and the commanded bit
  clear: a fresh tap has no stale window. Gates through `structural_command_for_test`:
  - After a restart of a subscribed compressor, the meter frame's gain-reduction word and the
    observation read return windows from the adoption block on.
  - After an insert removal, there is no slot for the removed insert, and the browser's event drain
    returns `OBSERVATION_ENDED`.
  - `tests/structural_apply_realtime.rs` stays at zero render-locked allocations.
- **The SDK's reaction** (`sdk/src/core/observation-subscriptions.ts` ends a managed subscription
  on `OBSERVATION_ENDED`) goes to #1562b-K.
- **No edit under `crates/engine/src/realtime`.**
- **Remainder of #1584.** D1-D5, D7 and D8 (the C ABI `Observe` edit) are a later slice. It writes
  the same bindings through `MonitorWrite`, and gate 5's removal case then also checks
  `observation.not_subscribed` after the end.

### Anchors moved

- `hosts/host-web/src/lib.rs`: kinds 7 and 8 are at `:754` and `:756`, `into_observe_record` at
  `:3944`, and the admission push is `push_observe` at `:1759`.
- `crates/effect-contract/src/live.rs`: `Observe` `:126`, `apply_observe` `:1471`,
  `ObservationLane` `:1528`, `arm` `:1630`, `disarm_all` `:1670`.
- `crates/effect-compiler/src/prepare.rs`: `try_push_observe` `:782`, `observe_available_capacity`
  `:801`.
- `crates/protocol/src/controller.rs`: `ProviderFeatures` `:109`, `telemetry_configuration`
  `:663`, `commit_prepared_structural` `:2150`.
- `crates/host-core/tests/effect_observation.rs`: the replacement test is at `:534`, and
  `scalar_reference_windows` at `:587`.
- The capi race loop is at `crates/capi/tests/resource_lifecycle.rs:3350`.
- `sdk/src/core/observation-subscriptions.ts`: `ObservationSubscriptionOwner` `:477`, `refs`
  `:212`, the conflict `:570`.
- `SessionState::command` is at `control.rs:1208`, and `miso_engine_v1_submit_command` at
  `crates/capi/src/ffi.rs:680`.
- `host_core::launch_maximum_observation_taps` does not exist yet (D7 adds it).

## Ruling (root, 2026-10-15): batch 12

1. **D3, where the bindings live.** Neither as filed nor inside `MonitorOverlay`: solo and
   subscriptions are different things. The bindings get their own type (for example
   `ObservationBindings`), owned by `SessionState` next to the monitor overlay, and they travel to
   every successor preparation in `SuccessorBase` the same way the monitor does (a field, not a new
   preparer argument). Its `after()` drops the bindings whose target is gone and returns their keys.
   `ProtocolController` does not own them. D6's `ArmedTap`/`armed_taps` are replaced by this.
2. **D6, a removed owner.** A reliable `OBSERVATION_ENDED` event (revision and stable key),
   room-checked with `SESSION_COMMITTED` before the commit; no room is `BACKPRESSURE` and nothing
   changes. The same on both hosts, with a parity row.
3. **Order.** D6a (core, lane K) lands in batch 12a; D6b (browser arm bits, lane W) in batch 12b.

## Decision record: #1584 D6a (batch 8, lane K, batch 12)

What shipped, on `codex/controls-batch-12k`:

- **The bindings type** (`crates/controls/src/observation.rs`): `ObservationBindings` (a sorted table of
  `TapKey { strip, section: TapSection::{Console, Inserts}, tap_id }` to the window as asked for),
  apart from `MonitorOverlay`. `after(next_model)` drops the bindings whose target the model lacks and
  returns their keys (`ObservationChange { bindings, ended }`); a target is the strip, the console slot
  or insert, and a resident tap with that ID (`effect_rows::resident_observation_tap`). The window is
  stored as asked (`0` = the plan's meter window, which each plan resolves for itself); refs and the
  conflict codes (D3) belong to the slice that writes the table from the core's `Observe` edit.
- **Owner and carry**: `SessionState.observation` beside `monitor`; `SuccessorBase.observation:
  &ObservationBindings` (field, no new preparer argument; `host_core::NO_OBSERVATION` for a host
  without subscriptions). The rebuild arm computes `after()` from the prospective model, passes the
  result as the base's field and writes it with the commit, after the last fallible step.
- **Prepared arm**: `ObservationLane::arm_prepared(tap, window)` (`start_pending`); `accumulate` takes
  the first accumulated block's `first_sample` as the window start and clears the flag; `arm` and
  `disarm_all` clear it; `swap_state` moves it with the tap, so a carried tap's state replaces it. No
  record is queued. `attach_effect_observation` takes `&[ObservationArm]` (strip, console flag, ID, tap
  ID, window) and arms by tap ID; host-core builds the arms from the base's bindings.
- **`OBSERVATION_ENDED` (`8011`)**: reliable event, `observation_ended` schema (sequence 1, strip 2,
  section 3 = rack code inserts `2` or console `5`, effect/slot ID 4, tap ID 5; revision in the event
  header), advertised with `8001` (`ProviderFeatures::observation_events`, in `ALL`). The key is text, so
  the queue slot holds a cell index and the controller keeps `slots + 1` prepared cells (an event popped
  for egress keeps its cell until delivered). The host attaches the ends to the prepared token with
  `ProtocolController::attach_observation_ended`, which reserves the room with the token's own
  `SESSION_COMMITTED` and refuses with `NoRoom`; the control plane returns `BACKPRESSURE` with the token
  dropped, before anything is prepared or published. The commit emits the ends after `SESSION_COMMITTED`
  at the committing revision, in key order, sequences following the commit's. One control plane serves
  both hosts, so the event is the same on both; the wire parity row is `event.observation_ended` in the
  conformance corpus (and the capabilities row in its capabilities frame).
- **`push_effect_observe`** records the pushed state per observation slot (`observation_pending`, sized
  when a plan is published) and the next `service()` folds it into the table. Departure from "writes the
  bindings after a successful push": the browser admits a subscription inside `process()`
  (`a_bus_observation_admits_and_renders_without_allocating`, #1213 gate 7), and a binding owns its key's
  text, so the write cannot allocate there. The push is still the acknowledgement; the fold runs before
  any rebuild reads the table (`command()` services first) and before the successor's slots replace the
  pending ones. `observation_bindings()` takes `&mut self` for the same reason.
- **Resource row**: `capi_resources` charges the cell store (`Option<ObservationEnded>` x slots + 1).

Gates (D6a's seven) and tests:

1. Restarted owner, subscribe applied: `observation_carry::a_restarted_effects_subscription_survives_the_rebuild_at_eight_lanes`
   (pending=false), and E13 rewritten as `a_plan_replacement_keeps_the_subscriptions_the_host_armed`
   (`effect_observation.rs`; the replacement changes the slot's link mode so nothing carries). The
   reference is a record applied at the adoption block in the successor's own queue: same state and input,
   windows equal field for field, first window at `SWAP_SAMPLE`.
2. Pending subscribe: the same test with pending=true (the record waits in the predecessor's queue).
3. Pending disarm: `a_pending_disarm_leaves_the_restarted_tap_unarmed_at_eight_lanes`.
4. Removed owner: `a_removed_effect_ends_its_binding_with_an_event` (insert removed, strip removed, ID
   reused for `miso.delay`: one `OBSERVATION_ENDED`, committing revision, stable key, sequence after the
   commit's, successor has no tap); `controls/tests/observation.rs` (`after()` incl. console slot and an
   unknown tap); host-core `a_stale_binding_arms_nothing`.
5. No room: `no_room_for_the_end_refuses_the_removal_and_changes_nothing` (model, revision, table, plan
   epoch, lane unchanged; the same request commits after a drain); protocol
   `the_ends_need_room_with_the_commit_and_a_dropped_token_releases_it` and
   `attached_observation_ends_follow_their_commit_in_order`.
6. Carried owner: `a_carried_tap_is_not_restarted_by_the_prepared_arm` (NodeToNode, BankToBank,
   NodeToBank; streams equal the empty-table run, sequences 1, 2, 3); the #1327b tests are green.
7. Realtime: the swap-block allocator and render-audit counters are `((0,0,0),(0,0))` in gates 1, 2 and
   6; `realtime-policy` passes (141 regions); the browser admission allocation gate stays at zero.

Re-pins: `COMPLETE_SCHEMA_HASH` `0x4b21_d9e6_f520_afd9` to `0x33e4_b1ce_bcbf_d902` (reason: #1584 D6a adds
the event `8011`, so the corpus gains `event.observation_ended`, 46 frames, and the capabilities frame
lists `8011`), with the history line, the parity script's self-test strings, the manifest and the
conformance record; two C ABI capabilities vectors in `capi/src/runtime/tests.rs` (the capabilities
response lists `8011`; same length). No rendered bit moved: the audit's `pcm_digest` is
`bf7dcedd1d92621f`.

Departures and limits:

- The reliable lane of both hosts has two slots (`protocol_queue_config`), so a transaction can end at
  most one binding (the commit takes the other). A transaction that ends more can never fit and is
  `BACKPRESSURE` forever; the app disarms first. Successor: size the lane for the largest ended set, or
  carry the ends in one event.
- The C ABI keeps `observation_taps` 0, so its table is empty; the Observe edit (D1-D5, D7, D8) writes the
  same table through the core.
- The capi race test and `tools/audit` do not cover the new event (not in D6a's paths).

## Fold (root ruling A, batch 12)

Root ruling A replaces D6a's one event per ended binding with one `OBSERVATION_ENDED` per commit. Commit
"Fold #1584 D6a: one OBSERVATION_ENDED per commit carries every ended key (root ruling A)".

- **Wire**: `OBSERVATION_ENDED` (`8011`) is the sequence (field 1) and one repeated `key` message (field 2,
  mandatory, at least one); a key holds strip ID 1, section 2 (inserts `2`, console `5`), effect or slot ID 3 and
  tap ID 4. Keys are strictly ascending (strip, console before inserts, effect ID, tap ID), so the encoder and the
  decoder both refuse an empty list, an out-of-order list and a repeated key. Revision stays in the event header.
  Typed `ObservationEnded { event_sequence, keys: Vec<ObservationTapKey> }`; schema modules `observation_tap_key`
  and `observation_ended`; codecs, typed frame, registry and semantics docs follow.
- **Room**: `ProtocolController::attach_observation_ended` reserves ONE reliable slot beside the token's
  `SESSION_COMMITTED` whatever the number of keys, and returns the event frame's length. `BACKPRESSURE` now means
  only that the lane holds unread events. The commit moves the prepared event into its cell: it allocates
  nothing (the keys are built and sized when the transaction prepares, off the commit).
- **Size at preparation**: the event is encoded for size by `attach` against the endpoint's codec, whose frame limit
  is the control frame. A frame over that limit (or an unencodable list) is `AttachObservationEndedError::Unencodable`,
  and the control plane refuses the transaction as `CompileRejected(resource limit)` and not as backpressure: a retry
  could never succeed. `capi_resources` charges the heap a cell can own, one key per observation slot of the plan
  (`observation_ended_heap_bytes`, from the producers' `observe_capacity`, their strip and effect ID text and the key
  struct), for every cell (slots + 1) and for the token.
- **Gates**: `a_strip_removed_with_many_subscribed_inserts_gives_one_event_with_every_key` (control plane: N = 1, 2,
  7 and 40 subscribed inserts of a removed strip commit, one event, N keys in key order; the lane has two slots);
  `no_room_for_the_end_refuses_the_removal_and_changes_nothing` (gate 5, now stated as unread events only);
  protocol `attached_observation_ends_follow_their_commit_in_one_event`,
  `the_end_needs_room_with_the_commit_and_a_dropped_token_releases_it` and
  `observation_ended_round_trips_every_key_and_refuses_anything_else`; the corpus row `event.observation_ended` now
  carries two keys (console, then insert) and is the cross-host wire parity row (one control plane serves both
  hosts; the native and Wasm corpus runs pin the same bytes).
- **Re-pin**: `COMPLETE_SCHEMA_HASH` `0x33e4_b1ce_bcbf_d902` to `0xcc9a_7bcc_502c_4936` (history line "#1584 D6a
  fold: one OBSERVATION_ENDED per commit"), the parity script's self-test strings, the fuzz manifest and the
  conformance record. The capabilities frame is unchanged, so the C ABI vectors stay. No rendered bit moved
  (audit `pcm_digest` `bf7dcedd1d92621f`, checked below).
- **Limit that remains**: one event must fit one control frame. A commit that ends so many keys that their event
  exceeds `maximum_control_frame_bytes` (hundreds of keys at the browser's 16 KiB) is refused as a resource limit;
  the app disarms some first. Successor candidate: cap the table at admission by the keys one frame holds.
- **Charge made exact (capi)**: at D6a (97233c465) `cargo test -p capi` failed `capi_retained_bytes_charge_every_byte_the_compile_retains`
  (parametric-eq-nine-track: observed 165710, charged 165670) and `tiny_control_frame_still_accounts_three_provider_counters_exactly`,
  both 40 bytes over the charge: the session's `observation_offsets` table (`(producers + 1) x u32`, here 10 x 4) and its
  `observation_pending` slots were allocated and never charged. `capi_resources` now charges both
  (`ObservationResources { producers, slots, ended_heap_bytes }`), beside the fold's ended-event heap; the whole
  `cargo test -p capi` passes (132 passed). Audit capi `pcm_digest` `bf7dcedd1d92621f`, allocations 0.

## Fold (batch 12a review)

- **Ended-event charge is a frame bound.** The `OBSERVATION_ENDED` cells were charged as the
  active plan's producers' key heap times the cell count. After a removal the successor (fewer
  producers) was charged while its cells could still hold the predecessor's keys. The charge is
  now `ended_event_ceiling(maximum_control_frame_bytes)`: an event must fit one control frame, so
  its text is at most the frame and its key count at most the frame over the smallest encoded key
  (measured by encoding one key and two, so it follows the wire), times `size_of::<ObservationTapKey>()`.
  The token row uses the same ceiling. `capi_resources` also charges the `ObservationBindings`
  table (capacity at most twice its length, each binding with the text of two IDs at 127 bytes)
  and the transient copy `after()` makes (each binding is kept or ended, so at most `slots`
  together, plus the ended protocol keys), per plan, so both the active plan and a candidate
  carry them. `ObservationResources` lost `ended_heap_bytes`.
- **Measured (capi, x86-64, control frame 4,096):** `capi_retained_bytes` of the reference
  session 165,758 -> 186,614; the budget ceiling is 186,614 plus 10 %, rounded to 64: 205,312.
  `capi_retained_bytes_charge_every_byte_the_compile_retains` and the tiny-frame test stay exact:
  they add `control_plane::observation_reserve_bytes` (test-support) to the observed side beside
  the send producers' bound, since nothing holds those bytes until a tap is armed. The two peak
  tests add the token's ceiling to the prepared-protocol term.
- **No allocation after the commit.** The pending-subscription slots are built before the
  protocol commit (`observation_slots` reserves with `try_reserve_exact` from the candidate's
  producers; a failure is a `CompileRejected(Allocation)` refusal with nothing changed) and the
  commit only assigns them. `ObservationBindings::after` returns `Result` and reserves both
  lists exactly before it copies, as does the ended-key list; the text clones of a key remain
  ordinary `String` allocations (before the commit, in a refusable step, but not fallible).
- **Over-frame refusal gate.** `an_end_event_over_one_control_frame_refuses_the_removal_and_changes_nothing`
  (control-plane, frame 1,024 bytes, 60 subscribed inserts on the removed strip): the removal is
  `CompileRejected(Resource(Limit))` and the revision, canonical session, bindings, plan epoch and
  event lane are as they were; a strip with two subscribed inserts commits under the same limit.
  Red when the refusal is turned into backpressure (checked, reverted).
- **Dead API.** `ObservationLane::is_prepared_arm_pending` had no caller (its doc said gates use
  it) and is deleted.
- **Internal invariant spelled accurately.** A reliable `OBSERVATION_ENDED` slot whose cell is
  empty is now `EventEgressError::ObservationEndedCellMissing` (was `DiagnosticStorageFull`);
  behaviour (an internal error) is unchanged.

## Decision record: #1584 D6b (batch 12b, lane W)

What shipped, in `hosts/host-web/src/lib.rs`:

- `build_plan_shape` takes the bindings (`&[controls::ObservationBinding]`, from the `SuccessorBase` the
  preparer received; empty at boot) and sets `observation_armed` for each binding whose strip, section
  (console or inserts), effect ID and tap ID match an observed instance of the plan. The bit is the tap's
  index in the descriptor's observations, as `push_observe` sets it. The arm sample stays 0 and the
  commanded bit stays clear. The match is the one host-core uses for the lane's prepared arm, so the
  table and the lane cannot disagree; a binding the plan lacks sets nothing. A carried reader still
  takes the predecessor's bits (`carry_tap_state`), so the prepared arm never wins over carried state.
  Nothing new runs in `process()`: the bits are written on the control side while the shape is built.
- Doc comment of `onControlEvent` names `OBSERVATION_ENDED` in
  `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` and its copy
  `sdk/src/browser/shipped-host.d.ts` (equal). The worklet drain (`drainControlEvents`) copies any
  frame it dequeues and decodes none, so the event passes through with no JS change (the reliable lane
  holds `SESSION_COMMITTED` and `OBSERVATION_ENDED`, within the drain's 16-call bound). The SDK reaction
  stays with #1562b-K.

Gates (`structural_apply` in `hosts/host-web/src/tests.rs`, through `structural_command_for_test`):

1. `a_restarted_effects_subscription_keeps_publishing_windows_on_the_browser`: the subscribed compressor's
   link mode changes (rebuild, no carry); the successor has `observation_armed_taps() == 1`, its first
   window opens at the adoption block's first sample with sequence 1, the windows continue (sequence 3
   after four more blocks), and the meter frame's gain-reduction word is positive. Red with the
   bindings passed as `&[]` (armed taps 0 after the swap).
2. `a_removed_inserts_subscription_ends_with_an_event_and_has_no_slot`: after the insert removal the
   browser's event drain returns `SESSION_COMMITTED` then one `OBSERVATION_ENDED` with the committing
   revision and the key (`track`, inserts, `comp`, tap 1); the successor has no armed tap and
   `read_observation_addresses` refuses the removed insert's address.
3. `tests/structural_apply_realtime.rs`: 3 passed, zero render-locked allocations.

Runs: `cargo test -p host-web --features host-web/test-support` (all), `cargo test -p control-plane
--features test-support`, clippy `-D warnings` on host-web, fmt, the wasm build, `check-web-audioworklet.sh`,
`test-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, `check-sdk-generated.sh`,
`check-sdk-types.sh`, `check-sdk-headless.sh`, the chromium qualification (151.0.7922.34, all
render-allocation rows 0), `realtime-policy` (141 regions) and `check-bench-policy.sh` all pass. The
shipped module sha256 is `179a933a...c3c1ae`; no export, layout or JS file changed.
