# Refuse commands that would be acknowledged with no effect

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-13 E4).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

No host acknowledges a command whose commanded state render will not reach. Two such commands
exist on `main`, both verified below: `AUTOMATION_ENQUEUE` on the C ABI, and a browser live
command that lifts the prepared bypass of a delay or a multiband compressor. Each is now refused
with a type, before anything changes, and the C ABI no longer advertises the command it refuses.

The rule this slice applies: a command is "acknowledged with no effect" when, after the ack, the
state render reaches differs from the state the command set. A command that sets a value already
in effect is not one: its commanded state holds.

## Context

**Verified acked-with-no-effect commands (both refused here):**

1. **`AUTOMATION_ENQUEUE` (C ABI).** The controller validates the batch and enqueues it
   (`crates/protocol/src/controller.rs:2961-2999`, `try_enqueue_automation`); nothing outside the
   protocol's own tests ever dequeues it (`ProtocolQueues::try_dequeue_automation`,
   `crates/protocol/src/queue.rs:801`, has no production caller), so it reaches no PCM. The
   automation queue holds one batch (`automation_batch_slots: one`,
   `crates/control-plane/src/compile.rs:142-167`), so after one accepted batch every later one is
   `BACKPRESSURE` until a transport locate cancels it. capi enables every provider family
   (`ProviderFeatures::ALL`, `compile.rs:798`), so the command is advertised. Documented as
   undelivered in `docs/CONTROL_PROTOCOL_SEMANTICS.md:15` and in decision 14
   (`docs/rulings/live-update-versus-rebuild-2026-10-04.md:203-206`).
2. **Browser `COMMAND_EFFECT_BYPASS` lifting a prepared bypass.** The delay and the multiband
   compressor keep a session bypass as a prepared bypass (`NEVER_BANKED_EFFECTS`,
   `PREPARED_BYPASS_EFFECTS`, `lowers_session_bypass`, `crates/effect-compiler/src/prepare.rs:236-266`).
   Their lane is seeded bypassed, and a live `COMMAND_EFFECT_BYPASS` with value `0` is admitted
   and renders nothing different (documented at `hosts/host-web/src/lib.rs:836-843`; admitted at
   `:4489-4500` and `:5030-5050`).

**Examined and not refused:**

- The same bypass lift through `SESSION_TRANSACTION_APPLY` (C ABI) is already a rebuild:
  `classify_live_delta` returns `LiveRebuild::PreparedBypass`
  (`crates/host-core/src/live_delta.rs:380-385`). Its D15-9 transition is *Duck-swap a strip whose
  state cannot continue across a plan swap* (#1324).
- Model-only transaction edits (session ID, profile IDs, the stored automation table;
  `docs/CONTROL_PROTOCOL_SEMANTICS.md:15`) change the committed document, which is their
  commanded state. *Report each transaction's edit path in its response* (#1313) reports them as
  `model_only`; rendering stored automation is #1058.
- `TRANSPORT_SET` sets endpoint-local transport state and position, as documented
  (`docs/CONTROL_PROVIDER_BOUNDARY.md:3-5`, `crates/host-core/src/control_provider.rs:432-438`);
  `TRANSPORT_GET` and the `TRANSPORT_STATE` event reach that state.
- Every other C ABI command family and every other browser command kind has a render or
  readback effect.

**Wire rule that shapes D2.** A capabilities record must advertise `0x8002`
(`AUTOMATION_CANCELED`) exactly when it advertises `0x8001`, and flag bit 5 exactly when it
advertises command `0x0006` (`crates/protocol/src/message_wire.rs:3496`, `:3512`, `:3522`).

## Decisions frozen for this slice

- **D1. An `automation` provider feature.** `ProviderFeatures` gains `automation: bool`; `ALL`
  sets it, `NONE` clears it. `execute` returns `StatusCode::Unavailable` for
  `ControlCommand::AutomationEnqueue` when it is false, in the same feature match as the other
  families (`execute`, `controller.rs:2820-2833`), before the revision checks.
- **D2. Advertise only what is served.** With `automation` false, `capability_registry`
  (`controller.rs:3219`) omits command `0x0006` and flag bit 5. Event `0x8002` stays advertised,
  because the wire rule ties it to the session family; with no batch admitted, nothing raises it.
- **D3. The control plane serves no automation.** Its controller config uses
  `ProviderFeatures { automation: false, ..ProviderFeatures::ALL }` (today `compile.rs:798`).
  It stays false until the slice that answer A9 of *Research: render stored session automation in
  the engine, identically on every platform* (#1058) names wires a render-side drain.
- **D4. Browser bypass lift is refused.** Preparation records on each effect producer whether its
  bypass is prepared (`!lowers_session_bypass(id)` and session `bypass` true). The browser's
  admission refuses a `COMMAND_EFFECT_BYPASS` with value `0` to such an instance with
  `COMMAND_REASON_UNSUPPORTED_KIND` at that record's index, and admits nothing from the batch.
  Value `1` to it is admitted (its commanded state holds). The `COMMAND_EFFECT_BYPASS` and
  `COMMAND_REASON_UNSUPPORTED_KIND` doc comments say so.
- **D5. Docs.** `docs/CONTROL_PROTOCOL_SEMANTICS.md:15` ("Delivery status") states the refusal
  and its owner (#1058); decision 14's IO-5 follow-up is met.
- **D6. The acked-batch question.** Both refusals are typed and happen before anything is
  admitted, so no ack is followed by silence: that is this slice's purpose.

## Deliverables

1. D1-D3 in `protocol` and the control plane; D4 in effect-compiler and host-web; D5.

## Authorized paths

- `crates/protocol/src/controller.rs`, `crates/protocol/src/lib.rs` (exports).
- `crates/control-plane/src/compile.rs` (D3).
- `crates/capi/src/runtime/tests.rs` (gates 1, 4).
- `crates/effect-compiler/src/prepare.rs` (the producer's prepared-bypass bit only).
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs` (D4; stream H's files, sequenced
  by the coordinator).
- `docs/CONTROL_PROTOCOL_SEMANTICS.md`, `docs/CONTROL_PROTOCOL_REGISTRY.md`.

## Non-goals

- Wiring automation to render (#1058). Refusing automation on non-`Block` parameters (#1335).
- A live shunt for the delay or the multiband compressor (#1339, #1340); a transition for their
  rebuild (#1324).
- Changing what `TRANSPORT_SET` means.

## Objective gates

1. **`AUTOMATION_ENQUEUE` is refused through the C ABI (new capi test).** Submit one valid batch
   for a `Block`-rate parameter at the exact revision: the response is a non-OK frame with status
   `UNAVAILABLE` (14); the automation queue's occupancy is 0; the revision and the reliable event
   lane are unchanged. `CAPABILITIES_GET` lists no `0x0006`, flag bit 5 clear, `0x8002` still
   listed, and the record passes the wire validator.
2. **Protocol keeps serving automation when enabled.** The protocol's own automation tests pass
   unchanged with `ProviderFeatures::ALL`; a new protocol unit test checks `UNAVAILABLE` with
   `automation: false` and that the capabilities record omits `0x0006` and bit 5.
3. **Browser refuses the lift (new host-web test).** A session with a bypassed delay insert and a
   bypassed multiband insert: a batch of a fader edit followed by `COMMAND_EFFECT_BYPASS` value
   `0` to the delay returns `RESULT_UNSUPPORTED` with `COMMAND_REASON_UNSUPPORTED_KIND` at
   index 1 (the mapping at `hosts/host-web/src/lib.rs:4668-4677`), and nothing is admitted (the
   fader edit is not heard);
   the same for the multiband. Value `1` to either is admitted. A bypassed compressor's lift is
   still admitted and heard.
4. **Superseded oracle bytes.** `all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes`
   (`crates/capi/src/runtime/tests.rs:1857`) and `capi_controller_dispatches_every_advertised_command_family`
   (`:2250`) enqueue automation as an accepted command. Rewrite each to assert gate 1's refusal;
   re-pin the capabilities and response frames they pin (wire bytes), naming this issue as the
   reason. The six-family test keeps `0x8002` only if another path still raises it on the C ABI;
   otherwise it becomes a five-family test and says why.
5. **Workspace.** `cargo test --locked -p protocol --features test-support`;
   `cargo test --locked -p capi`; `cargo test --locked -p host-web --features test-support`;
   `cargo test --locked -p effect-compiler --features test-support`;
   `bash scripts/check-protocol-control-policy.sh`; `python3 -B
   scripts/check-command-reason-vocabulary.py`; `bash scripts/check-web-audioworklet.sh`;
   `bash scripts/check-workspace-policy.sh`; `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.

## Test value

- Gate 1: the C ABI acking automation it never renders (decision 14's acked-batch "yes"), or
  advertising a command it refuses.
- Gate 2: the feature switch leaking into the protocol's own semantics.
- Gate 3: a browser bypass lift on a prepared bypass acked and never heard; the compressor case
  is red if the check refuses lifts it should admit.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309), for D3's path.

**Decision 17 amendment (2026-10-13).** Which values stored automation may target is decided by each control row's computed `automatable` (#1566); `BUILTIN_AUTOMATION_TARGETS` and `effect_automation_diagnostics` are deleted. `AUTOMATION_ENQUEUE` addresses by a table of control addresses (#1567), and this issue's refusal of the command (D1-D3) stays until automation renders.

## Amendment (controls coordinator, batch 8, 2026-10-14)

Checked against `main` at `32cee58d0`.

- **Anchors (D1-D3).** `ProviderFeatures` is `crates/protocol/src/controller.rs:111-136`; it has
  `controls` and `response` now, and still no `automation`. `execute`'s feature match is
  `:3139-3157`; the `AutomationEnqueue` arm `:3302-3347` (`validate_automation_domains` `:3849`);
  `capability_registry` `:3704`. `ProtocolQueues::try_dequeue_automation`
  (`crates/protocol/src/queue.rs:801`) still has no production caller. `automation_batch_slots:
  one` is `crates/control-plane/src/compile.rs:199` and `provider_features: ProviderFeatures::ALL`
  is `:974`. The wire rule is `crates/protocol/src/message_wire.rs:4340`, `:4356`, `:4367`. The
  "Delivery status" paragraph is `docs/CONTROL_PROTOCOL_SEMANTICS.md:19`; decision 14's IO-5 is
  `:206`. Gate 4's tests are `crates/capi/src/runtime/tests.rs:2099` and `:2492`.
- **D1-D3 hold, and now cover the browser.** Since the flip (#1381a, #1562a, #1570b) the browser
  builds its session through the same `control_plane::compile_children`, so the one
  `ProviderFeatures::ALL` at `compile.rs:974` serves both hosts. The browser now acks
  `AUTOMATION_ENQUEUE` and renders nothing, the same defect as the C ABI. D3's one change refuses it
  on both hosts. `hosts/host-web/tests/protocol_cross_host.rs` step 11 (`:704-718`) already sends it
  to both hosts and compares the bytes; it stays green with both refusing `UNAVAILABLE` and is the
  browser leg of gate 1. D5's sentence names both hosts.
- **D4 no longer holds and leaves this slice.** #1565A (`5576cdc56`) deleted the browser's
  `COMMAND_EFFECT_BYPASS` with every value kind. A bypass lift is now a `SET_CONTROL` on the
  effect's bypass row. For the delay and the multiband compressor that row is rebuild-only
  (`crates/controls/src/effect_rows.rs:199-201`, through `lowers_session_bypass`,
  `crates/effect-compiler/src/prepare.rs:314`), so the core classifies the lift
  `LiveRebuild::RebuildOnly` with the row's reason (`crates/host-core/src/live_delta.rs:220-224`;
  `LiveRebuild::PreparedBypass` is gone). The C ABI rebuilds. The browser refuses the frame
  `web.apply.rebuild_pending` before commit until #1562b, and rebuilds after it. No host acks a
  lift that render does not reach. These hold the claim:
  `crates/host-core/tests/live_delta.rs::a_prepared_bypass_change_needs_a_rebuild` (#1266 gate 1),
  `hosts/host-web/tests/control_admission.rs::a_rebuild_frame_is_refused_before_commit_and_changes_nothing`,
  and the delay and multiband bypass parity rows that #1562b owes (#1562, batch 7 amendment).
- **Gates and paths that change.** Gate 3 is deleted. The authorized paths lose
  `crates/effect-compiler/src/prepare.rs` and `hosts/host-web/src/{lib.rs,tests.rs}`. Gate 5 loses
  `cargo test -p effect-compiler`, `check-command-reason-vocabulary.py` and
  `check-web-audioworklet.sh`; it keeps `cargo test --locked -p host-web --features test-support`
  for `protocol_cross_host.rs`.
- **What remains.** D1-D3 and D5 in `protocol`, `control-plane`, the `capi` tests and the two docs;
  gates 1, 2, 4 and 5. #1567 depends on it ("Still not served", its gate 4).
