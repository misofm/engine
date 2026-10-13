# Set any control with one transaction edit

Slice S4a of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C5).
Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

A C ABI host sets any control of a session with one transaction edit, `SET_CONTROL`, addressed by
the control's stable address. The core's one validator refuses a bad write before anything
changes, with a typed code. Whether the edit is live or a rebuild is still the existing
classifier's answer, so a `SET_CONTROL` fader edit is exactly as live as `SetTrackFader`.

## Context

- **Edits.** `SessionEdit` (`crates/protocol/src/model.rs:189`) and `SessionEditOpcode`
  (`:23-119`, `from_raw` `:131-181`) hold 47 opcodes; `0x08xx` is free, and
  `docs/CONTROL_PROTOCOL_REGISTRY.md:104` says appended codes take the next unallocated ID.
  `SessionEdit::opcode()` (`:497`), `ramp_samples()` (`:555`, exhaustive) and `record_edit_ramps`
  (`:615`, exhaustive; a new edit must be classified, comment `:704-707`).
- **Apply.** `apply_session_edit` (`model.rs:792`) writes one edit into the candidate model.
  `SessionStore::apply_transaction` (`:1191`) calls `prepare_transaction` (`:1198`): per edit
  `apply_session_edit` and `record_edit_ramps` (`:1227-1233`), then `compile_session`, then the
  ramp bound (`:1242-1253`, a ramp above the session rate is `RampOutOfRange`). A failure is
  `SessionStoreError::Edit { operation_index, error }` (`:1119`); its codes are
  `session.edit.*` with path `edits[i]` (`crates/protocol/src/controller.rs:3514-3538`), status
  `VALIDATION_FAILED` (`:3501`).
- **Wire.** `tx_edit_payload` (`crates/protocol/src/session_wire.rs:338`) and `parse_edit`
  (`:1225`); an unknown opcode is `DecodeError::InvalidTlv` (`:1230`). An optional ramp is
  `FieldSpec::opt(3, Wire::U32)` (example `set_track_fader::RAMP_SAMPLES`,
  `crates/protocol/src/schema.rs:1402`); `ramp_samples_field` (`:1607`) and `payload_spec`
  (`:1662`) are exhaustive per opcode.
- **Corpus pins.** `complete_all_opcode_fixture` (`crates/conformance/src/protocol_corpus.rs:16`,
  one edit per opcode, ends `:298`); `COMPLETE_SCHEMA_HASH` (`:763-764`); the 47-opcode and
  46-edit assertions (`crates/protocol/src/controller/tests.rs:327`); the fixture payload check
  (`schema.rs:1771`).
- **The control path on the C ABI.** `commit_live` (`crates/control-plane/src/control.rs:1358`)
  classifies with `classify_live_delta` (`crates/host-core/src/live_delta.rs:215`), which diffs the
  two models and never reads opcodes; a refusal is a rebuild. #1313 (the edit path in the reply) is
  open, so tests read live or rebuild from the plan epoch (`Rig::summary`,
  `crates/capi/src/runtime/live_tests.rs`; `assert_rebuilds`, `:1567`).
- **The core.** `controls::set` and `ControlError` (#1546 D7), effect rows (#1549 D5), the address
  message (#1551 D3).
- **Dependency.** `protocol` depends on `engine` and `session` only. `controls` depends on
  `session`, `builtins`, `effect-contract` and `effect-compiler`, and never on `protocol`, so
  `protocol` can depend on `controls`.

## Decisions frozen for this slice

- **D1. The edit.** `SessionEdit::SetControl { address: controls::ControlAddress, value: f32,
  ramp_samples: Option<u32> }`, opcode `SessionEditOpcode::SetControl = 0x0800`. No sample time.
- **D2. Wire.** Schema `set_control`: 1 `address` `Message` (#1551's `control_address`, with
  `control` and `lane` required), 2 `value` `F32`, 3 `ramp_samples` `U32` optional. Arms in
  `ramp_samples_field` and `payload_spec`. A non-finite value is decoded and then refused by the
  validator (D3), not by the codec, so the refusal names the edit.
- **D3. Apply.** The `apply_session_edit` arm calls
  `controls::set(session, address, value, ramp_samples, session.sample_rate_hz)`. A
  `ControlError` becomes `SessionEditError::Control(ControlError)`, and its diagnostic code is the
  control code itself (`control.address.unknown`, `control.read_only`, `control.inactive`,
  `control.lane`, `control.domain`, `control.ramp.fixed`, `control.monitor`) with path `edits[i]`,
  status `VALIDATION_FAILED`. A refused edit refuses the whole transaction; the model, the revision
  and every cell stay as they were (the store's existing all-or-nothing rule).
- **D4. Ramps.** `ramp_samples()` returns the edit's field. `record_edit_ramps` maps the address to
  the existing rows: strip `fader_db` → (`Strip`, `Fader`); `mute` → (`Strip`, `Mute`); `pan`,
  `matrix_*` and own `matrix_mode` → (`Strip`, `Matrix`); `trim_db`, `polarity_invert` →
  (`Strip`, `Input`); route `gain_db` → (`Route`, `RouteGain`); route `mute` and `follows_mute` →
  (`Route`, `RouteMute`); route `matrix_*` → (`Route`, `RouteMatrix`); VCA `offset_db` → (`Vca`,
  `VcaFader`); VCA `mute` → (`Vca`, `VcaMute`). Every other row has a fixed ramp, so `check` has
  already refused an explicit length (`control.ramp.fixed`). The existing bound
  (`ramp_samples` at most the session rate) applies unchanged.
- **D5. Classification unchanged.** The old classifier stays. A `SET_CONTROL` on a value it
  already makes live (track fader, mute, pan, matrix, effect parameter, effect bypass) is live; on
  any other value it is a rebuild, as the equivalent per-kind op is. Old ops keep working (#1561
  retires them).
- **D6. Registry.** `docs/CONTROL_PROTOCOL_REGISTRY.md` lists `0800` (48 allocated opcodes), its
  payload and its ramp row; `COMPLETE_SCHEMA_HASH` is re-pinned with a history line; the corpus
  fixture gains one `SetControl` edit.
- **D7. Acked-batch question.** Every check runs in `apply_session_edit`, before compile and before
  any cell write; a refused transaction acks nothing.

## Deliverables

1. D1-D4 in `crates/protocol` (model, wire, schema, controller code mapping) and its `controls`
   dependency.
2. D6: corpus fixture, hash, opcode-count assertions, registry doc.
3. The tests below.

## Authorized paths

- `crates/protocol/{Cargo.toml,src/model.rs,src/session_wire.rs,src/session_wire/tests.rs,
  src/schema.rs,src/controller.rs,src/controller/tests.rs}`, `crates/protocol/tests/edit_ramps.rs`.
- `crates/conformance/src/protocol_corpus.rs`, `crates/conformance/tests/**` (pins only).
- `crates/capi/src/runtime/live_tests.rs` (new tests), `tools/audit/src/capi.rs` (one `SET_CONTROL`
  edit in `next_edit`'s cycle).
- `docs/CONTROL_PROTOCOL_REGISTRY.md`, `Cargo.lock`.

## Non-goals

- No classifier change and no new live lane (#1554).
- No solo writer: a `SET_CONTROL` on `solo` is `control.monitor` until #1559.
- No browser or SDK encoder (#1563). No retirement of old ops (#1561).

## Objective gates

Commands: `cargo test --locked -p protocol --all-features`, `cargo test --locked -p conformance`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo build --locked --release -p audit -p capi` then `./target/release/audit capi`.

1. **Live like the old op.** On a capi rig, `SET_CONTROL` on a track's `fader_db` (`Left`, then
   `Both`), `mute`, `pan`, a matrix row after a `matrix_mode` switch in the same transaction, an
   effect parameter and an effect bypass each commits with the plan epoch unchanged, and the next 8
   blocks are bit-identical to the same edit sent as the per-kind op on a second rig.
2. **Rebuild like the old op.** `SET_CONTROL` on `trim_db`, `delay_samples` and a send's `gain_db`
   commits as a rebuild (`assert_rebuilds`), exactly as the per-kind op does.
3. **Ramps.** On `fader_db`, `ramp_samples` absent, `0` and `480` give the session default, a step
   and 480 samples (rendered ramp length); an explicit ramp on an effect parameter is refused
   `control.ramp.fixed`; a ramp above the session rate is `session.edit.ramp_out_of_range`.
4. **Atomic refusal.** A transaction of one valid fader `SET_CONTROL` and one out-of-domain pan is
   refused `control.domain` at `edits[1]`; the model, the revision and the next 8 rendered blocks
   are unchanged. Each other `ControlError` code is produced once through the frame path.
5. **Wire.** Round trip of `SetControl` with and without a ramp; the opcode pins and the corpus hash
   pass.

## Test value

- Gate 1 turns red if `SET_CONTROL` writes a different field, lane or value than the per-kind op.
- Gate 2 turns red if a rebuild-only value is acked as live.
- Gate 3 turns red if an absent ramp is read as 0, or a fixed-ramp row silently ignores a length.
- Gate 4 turns red if one bad edit lets a good edit of the same transaction commit.
- Gate 5 turns red if the codec and the registry disagree on the new opcode.

## Dependencies

- #1546, #1549, #1551 (the address message).

## Supersedes and shrinks

- None.

## Decision record (implementation, 2026-10-13)

- **D8. `controls::get` and `controls::set` resolve one row (carry-forward from the batch-1
  review).** `set` is now the hot control-thread path (a transaction of N `SET_CONTROL` edits), and
  both functions used to build every row of the owner and run the allocating `effective(model)` per
  call, so N edits cost O(N x model). `registry::resolve` now finds the one row by owner, section
  and control ID (`strip::resolve`, `route::resolve`, `vca::resolve`, `session_rows::resolve`,
  `effect_rows::resolve`): the descriptor comes from the section table, from the builtin table with
  the owner's shape for `active`, or from the effect contract's descriptor, and the committed value
  from that row's accessor alone. Only a derived row in `get` computes `effective`; `set` never does
  (a derived row is refused by `check` first, and solo needs no overlay). Results are identical:
  the existing `controls` tests are unchanged and green, and `get_returns_the_listed_value_of_every_row`
  pins `get` to `list` for every row of the fixture under a soloed overlay.
- **D9. The edit's address type.** `protocol::control_core` re-exports `controls::{ControlAddress,
  ControlError, Lane, Owner, Section}` so `conformance` (whose dependency list the
  `check-conformance-boundaries.sh` gate pins) can build the corpus edit without a new dependency.
- **D10. The ramp row comes from the descriptor.** `record_edit_ramps` asks
  `controls::row_descriptor` for the row's `Motion` and maps its `LiveRampRow` to the existing
  `EditRampRow`, so D4's table lives in the control core and cannot drift; a row of an effect section
  has a fixed ramp. The mapping function has no wildcard, so a new live ramp row must be classified.
- **D11. Wire.** The address is the control list's `ControlAddress` message; the decoder requires
  `control` and `lane` and refuses an owner ID or slot ID that disagrees with its kind or section
  (`INVALID_FIELD`) and an unallocated owner, section or lane code (`INVALID_TLV`). The value is read
  without the finiteness check the other float fields have, so a NaN reaches the validator
  (`control.domain` at its own edit).
- **Mirrors re-pinned.** `COMPLETE_SCHEMA_HASH` is `0x9f13a06993efa99e`; the same value is in
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md`, `fuzz/corpus/complete-schema-manifest.md` and the fault
  injection search strings of `scripts/check-protocol-wasm-parity.sh` (which still named the value
  from before #1551).
- **D12. Ramp conflict (batch 2 review fold).** Several `SET_CONTROL` rows share one `(target,
  row)` ramp key, so "last edit decides" would let the later edit silently set the earlier one's
  length. One transaction that resolves two different lengths for one key through `SET_CONTROL`
  (an explicit length against another, or against none) is refused at the later edit with the new
  code `control.ramp.conflict` (`ControlError::RampConflict`, path `edits[i]`), before any write;
  equal lengths agree. A per-kind edit is not a control write and keeps its last-edit-decides rule.
