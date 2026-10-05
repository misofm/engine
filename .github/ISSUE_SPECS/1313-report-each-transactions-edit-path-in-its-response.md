# Report each transaction's edit path in its response

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-3, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The success response to every `SESSION_TRANSACTION_APPLY` names how the engine applies it: `live`
(records on the running plan's live lanes, no plan prepared), `model_only` (committed, nothing for
render to do) or `rebuild` (a successor plan was prepared and published). The host learns the path
synchronously, from the same response that carries the new revision. Exact replay of the request
returns the same bytes, path included.

This reverses #1053 Q3 ("not now"). D15-17 refines D15-3: the path is exactly one of these three
values, all known at submit. Fallbacks (pre-roll, transition) and supersession are known only after
submit, so they are reported by the watermark's outcome flags in *Publish an applied-revision
watermark and complete edits asynchronously* (#1314). There is no `rebuild_with_transition` path.

## Context

- The response payload is `TransactionApplied { applied_operations: u32 }`
  (`crates/protocol/src/message_wire.rs:163`), schema `transaction_applied` with one field
  (`crates/protocol/src/schema.rs:349-355`), encoded at `message_wire.rs:1019-1048`.
- The response frame is encoded during preparation, before the host classifies the edit:
  `prepare_structural_plan` (`crates/protocol/src/controller.rs:1598-1662`) encodes it and stores it
  both in the token's `response` and in `prospective_replay` (`:1629-1638`).
  `commit_prepared_structural` (`:1974`) installs that replay and returns the frame (`:2010-2018`).
  The protocol-only path `process_structural_plan_into` (`:1826`) encodes its own frame; no product
  host uses it (only `crates/protocol/src/controller/tests.rs` and `tools/audit/src/protocol.rs`).
- The C ABI decides the path after preparation. `SessionState::command`
  (`crates/capi/src/runtime/control.rs:843`) calls `commit_live` (`:888-891`), which classifies
  with `host_core::classify_live_delta` (`:1071`; `crates/host-core/src/live_delta.rs:211`).
  `LiveCommit::Done` is the live arm (`control.rs:411-416`); `LiveCommit::Rebuild` continues to the
  successor preparation and commits at `:1008`. Both commits go through
  `ObservedPreparedToken::commit` (`:353`).
- A delta that changes only the session ID, a profile ID or the stored automation is "live with no
  records" (`live_delta.rs:181-185`); so is a transaction that rewrites identical values
  (`:109-110`). Both are `model_only` here.
- The header says "The ABI gives no signal that tells a live edit from a replacement, and a host
  needs none" (`crates/capi/include/miso_engine_v1.h:60-62`); the qualification record says the
  same under "No rebuild signal" (`docs/C_ABI_V1_QUALIFICATION.md:302-308`).
- The registry documents the response as `1:applied-operations U32 R`
  (`docs/CONTROL_PROTOCOL_REGISTRY.md:27`).
- The one protocol wire corpus pin is `COMPLETE_SCHEMA_HASH`
  (`crates/conformance/src/protocol_corpus.rs:737`); its success corpus builds a
  `TransactionApplied` at `:904-909`. `tools/audit/src/protocol.rs:233-243` and `:381-401` build it
  too.

## Decisions frozen for this slice

- **D1. Wire.** `TransactionApplied` gains a required field `2:edit-path U8 R`. Values: `live = 1`,
  `model_only = 2`, `rebuild = 3`. Zero and every other value are refused on decode as an
  unallocated value, like every protocol enum. Rust: `pub enum EditPath { Live = 1, ModelOnly = 2,
  Rebuild = 3 }` in `crates/protocol`, exported beside `TransactionApplied`, field `edit_path`.
- **D2. Classification (C ABI).** `rebuild` when `classify_live_delta` returns `Err(_)`. Otherwise
  `model_only` when the delta has no strip and no effect entry, and `live` when it has at least one.
  Add `LiveDelta::is_empty(&self) -> bool` (`strips` and `effects` both empty) in
  `live_delta.rs` and use it; no second rule. A live delta committed while a rebuild candidate is
  pending (its records go to the newest candidate, #1053 D7) is still `live`: no plan is prepared.
- **D3. Path known before commit, written at commit.** `commit_prepared_structural` takes the path:
  `commit_prepared_structural(prepared, path: EditPath)`. Commit re-encodes the response into the
  token's own frame buffer with the path. The field is fixed width, so the length equals the
  prepared `response_len()` and the encode cannot fail (assert it). Commit then overwrites the
  prospective replay entry's bytes with the same frame through a new
  `ReplayCache::overwrite_completed_response(request_id, frame)`, which requires the entry to have
  been completed by this token and the length to be equal, and cannot fail otherwise. Prepare still
  measures and reserves everything; commit stays infallible after `check_prepared_structural`.
  The prepared frame carries `model_only` until commit; it never leaves the token before commit.
- **D4. Protocol-only path.** `process_command_frame_into` with no host classification reports
  `model_only`: that controller owns no render plan, so the commit changes only the model.
- **D5. Events unchanged.** `SESSION_COMMITTED` is unchanged. The path is never put on the reliable
  event lane.
- **D6. Acked-batch question.** The path adds no queue and no drop point. A refused live edit
  (`control.live.backpressure`) and a refused rebuild return no response payload, as today. Can an
  ack ever precede a drop? No: the path is written inside the same infallible commit.
- **D7. Prelaunch amendment.** This is an in-place amendment of the V1 response schema before launch
  freezes it (D15-3). No C ABI symbol, struct or feature bit changes: the field travels inside the
  response bytes, which every host decodes with this protocol crate's decoder.

## Deliverables

1. D1 in the protocol schema, typed frame, codec and decoder; `EditPath` exported.
2. D3 in the controller and the replay cache; D4.
3. D2 in the C ABI command path, with `LiveDelta::is_empty`.
4. Every `TransactionApplied` constructor updated (protocol tests, conformance corpus, audit tool);
   `COMPLETE_SCHEMA_HASH` re-pinned with the reason "TransactionApplied gains field 2" in the PR.
5. Docs: header `miso_engine_v1.h:35-62` (the live-edit paragraph) states the path field and drops
   "The ABI gives no signal ..."; `C_ABI_V1_QUALIFICATION.md` "No rebuild signal" bullet becomes
   "Edit path in the response"; registry row `0003` lists `2:edit-path U8 R` and the enum values.

## Authorized paths

- `crates/protocol/src/message_wire.rs`, `schema.rs`, `typed_frame.rs`, `controller.rs` (it holds
  `ReplayCache`, `:209`), `queue.rs`, `model.rs`, `lib.rs`, `controller/tests.rs`, `message_wire/tests.rs`
- `crates/capi/src/runtime/control.rs` (or its successor file in the crate that #1309 creates),
  `crates/capi/src/runtime/live_tests.rs`, `crates/capi/src/runtime/tests.rs`,
  `crates/capi/include/miso_engine_v1.h` (comment text only)
- `crates/host-core/src/live_delta.rs` (`LiveDelta::is_empty` only)
- `crates/conformance/src/protocol_corpus.rs` (constructor and hash re-pin only)
- `tools/audit/src/protocol.rs` (constructors only)
- `docs/C_ABI_V1_QUALIFICATION.md`, `docs/CONTROL_PROTOCOL_REGISTRY.md` (the rows named above)

## Non-goals

- The application sample, outcome flags and counters (#1314). The browser's `engine.apply`
  response (stream H, on the crate from #1309). Any change to classification itself.

## Hazards

- `prepare_structural_plan`'s frame and replay entry are built before classification; a commit
  that forgets the replay overwrite returns `model_only` on replay of a `live` edit. Gate 2 exists
  for this.

## Objective gates

1. **Path per arm (capi).** In `runtime/live_tests.rs`, one test drives three transactions on one
   session and decodes each response with the protocol decoder: a track fader change → `live`; a
   session ID change only → `model_only`; adding a track → `rebuild`. A fourth case: a fader change
   in the same transaction as a change of a track insert's `quality` → `rebuild`
   (`LiveRebuild::Structure`), so a transaction that also has live records is still reported as
   the rebuild it takes.
2. **Replay carries the path.** Resend the exact request bytes of the `live` case; the response is
   byte-identical to the first, and its decoded path is `live`.
3. **Decoder strictness.** In `message_wire/tests.rs`: a `TransactionApplied` with path 0, 4 or a
   missing field 2 is refused; 1, 2 and 3 round-trip.
4. **Protocol-only path.** In `controller/tests.rs`, `process_command_frame_into` of a transaction
   returns `model_only`.
5. Commands:
   - `cargo test --locked -p protocol --features protocol/test-support`
   - `cargo test --locked -p capi`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p conformance` and `bash scripts/check-protocol-wasm-parity.sh`
   - `bash scripts/check-protocol-control-policy.sh` and `bash scripts/test-protocol-control-policy.sh`
   - `cargo build --locked --release -p audit -p capi && bash scripts/run-protocol-allocation-audit.sh target/release/audit && ./target/release/audit capi`
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
   - `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: red if the path is derived from anything but the classifier's result (for example
  "records pushed" without the VCA refusal, or `live` for an empty delta); no existing test reads
  the response payload's path.
- Gate 2: red if commit forgets to overwrite the replay entry, so a retry reports the prepared
  placeholder; the existing replay tests compare bytes of responses that had no path.
- Gate 3: red if the decoder accepts an unallocated path value or treats field 2 as optional.
- Gate 4: red if the protocol-only path reports `rebuild` or `live` for a controller with no plan.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309): it moves
  `control.rs`; land this on top of it so the edit lands once, in its final file.
