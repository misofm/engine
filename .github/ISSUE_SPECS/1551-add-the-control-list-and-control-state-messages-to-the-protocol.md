# Add the control-list and control-state messages to the protocol

Slice S3 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C4),
first half: the wire. Split from the planner's S3; #1552 serves the messages from the core. Code
anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer,
normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

The control protocol has one query for every control of a session: `CONTROL_LIST_GET` returns
pages of owner, section and control rows with their descriptors and values, and
`CONTROL_STATE_GET` returns the values alone. The old effect-only parameter messages are refused
and their codes are never used again.

## Context

- **Message IDs.** `MessageId` (`crates/protocol/src/wire.rs:97-133`): commands `0x0001`-`0x000b`;
  `ParameterMetadataGet = 0x0004` (`:106`), `ParameterStateGet = 0x0005` (`:108`). `from_raw`
  (`:143-165`) maps an unlisted ID to `DecodeError::UnsupportedMessage` (`:163`), status
  `UNSUPPORTED_MESSAGE = 3` (`:186-187`, mapping `:507`). `0x000c` and `0x000d` are free. Retired
  codes are "refused and never reallocated" (`docs/CONTROL_BTLV_V1.md:61`); no message ID is
  retired yet.
- **Hard-coded ranges.** `allocated_id` (`crates/protocol/src/message_wire.rs:3534-3540`,
  `matches!(id, 1..=11)`); `capability_registry`'s command loop (`crates/protocol/src/controller.rs:3223-3239`,
  `4 | 5 => features.parameters`); capability flag bit 7, set at `:3261-3263`, with decode
  invariants at `message_wire.rs:3516` (bit 7 iff both `0x0004` and `0x0005`) and `:3525`;
  `maximum_parameter_page_items` (`message_wire.rs:93`, `:131`; schema field 21,
  `crates/protocol/src/schema.rs:696`; value 256, `controller.rs:3321`; bound at
  `message_wire.rs:3505`).
- **Wire types.** `schema::Wire` (`schema.rs:115-131`): `U8`, `U16`, `U32`, `U64`, `I64`, `F32`,
  `F64`, `Bool`, `Utf8`, `Bytes`, `Message`, `PackedU16`, `PackedU32`, `PackedU64`, `PackedF32`.
  A repeated message is `FieldSpec::msg(id, mandatory, repeated = true, &SPEC)` (example
  `metadata_page::DESCRIPTOR`, `:389`); fixed records are count + stride + `Bytes` (example
  `state_page`, `:405-415`).
- **The old messages.** Schemas `metadata_request` (`schema.rs:375-382`), `metadata_page`
  (`:385-393`), `descriptor` (`:279-304`), `enum_choice` (`:268-276`), `state_request`
  (`:396-402`), `state_page` (`:405-415`). Types in `message_wire.rs` (`ParameterDescriptor` `:355`,
  `ParameterMetadataRequest` `:379`, `ParameterMetadataPage` `:386`, `ParameterStateRecord` `:394`,
  `ParameterStateRequest` `:402`, `ParameterStatePage` `:408`) and their codec (`:1099-1240`,
  `:2369-2389`, `:2744-3121`). Typed frames (`crates/protocol/src/typed_frame.rs:46-47`, `:70-71`,
  `:122-123`, `:145-146`, `:192-193`, `:210-211`, `:427-431`, `:488-492`, `:657-661`,
  `:699-703`). The provider trait (`controller.rs:564-605`: `parameter_metadata`,
  `parameter_state`, `parameter_descriptor`), `ControlCommand` (`:895-900`), decode
  (`:1493-1497`, `:2136-2140`), the feature gate (`:2826-2827`), execute (`:2949-2960`) and encode
  (`:2240-2244`). `ProviderFeatures::parameters` (`:110-118`).
- **Corpus and docs.** `crates/conformance/src/protocol_corpus.rs` (`:775`, `:808-809`,
  `:836-844`, `:938-947`; `COMPLETE_SCHEMA_HASH` `:764`; `retired_code_rows()` `:393`),
  `crates/conformance/tests/conformance_corpus.rs:62-99`, `fuzz/corpus/complete-schema-manifest.md`,
  `docs/CONTROL_PROTOCOL_REGISTRY.md:13`, `:28-29`, `:37`, `:39`,
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3`, `docs/CONTROL_PROTOCOL_SEMANTICS.md:29`,
  `docs/CONTROL_PROTOCOL_SIZING.md:9`.
- **The address and rows.** #1546 D4 (`ControlAddress`, section codes own 6, builtins 4, console 5,
  inserts 2), #1546 D2 and D6 (`Family`, `Taper`, `ControlKind`, `ControlClass`, `RebuildReason`,
  `ControlScope`, `LaneShape`), #1549 D6 (display strings).

## Decisions frozen for this slice

- **D1. IDs.** `MessageId::ControlListGet = 0x000c` and `MessageId::ControlStateGet = 0x000d`.
  `ParameterMetadataGet` and `ParameterStateGet` are removed. `0x0004` and `0x0005` decode to
  `UnsupportedMessage` and are listed in a `RETIRED_MESSAGE_IDS` constant in `wire.rs`, which
  `from_raw` and `allocated_id` consult; a test pins that no variant uses a retired code. The
  allocated command set becomes `1..=3 | 6..=13`. Capability flag bit 7 means "serves `0x000c` and
  `0x000d`"; `ProviderFeatures::parameters` is renamed `controls`. `maximum_parameter_page_items`
  keeps its wire field (21) and bound (256) and is renamed `maximum_control_page_items` in Rust.
  Contract identity stays V1; the Rust names are unversioned.
- **D2. Request** (`control_list_request`, also used by `CONTROL_STATE_GET`): 1 `structure_epoch`
  `U64` required (`0` for the first page), 2 `after_handle` `U32` required, 3 `limit` `U16` required
  (1 to 256), 4 `owner_kind` `U8` optional, 5 `owner_id` `Utf8` optional (present only with
  `owner_kind`, and absent for the session owner).
- **D3. Address** (`control_address`): 1 `owner_kind` `U8` (1 session, 2 strip, 3 route, 4 VCA),
  2 `owner_id` `Utf8` optional (absent exactly for the session), 3 `section` `U8` (own 6,
  builtins 4, console 5, inserts 2), 4 `slot_id` `Utf8` optional (present exactly for console and
  inserts), 5 `control` `U32` optional (absent exactly in a header row), 6 `lane` `U8` optional
  (`effect_contract::ParameterChannel` codes: left 1, right 2, both 3; absent exactly in a header
  row). The codec refuses any other combination with `INVALID_FIELD`.
- **D4. List page** (`control_list_page`): 1 `revision` `U64`, 2 `structure_epoch` `U64`,
  3 `last_handle` `U32`, 4 `eof` `Bool`, 5 `entry` `Message` repeated (`control_entry`).
- **D5. Entry** (`control_entry`): 1 `handle` `U32`, 2 `row_kind` `U8` (1 owner header, 2 section
  header, 3 control), 3 `address` `Message`. A header row adds 4 `header_kind` `U8` (owner: track 1,
  submix 2, route 3, VCA 4, session 5; section: the section code), 5 `name` `Utf8`, 6 `effect` `Utf8`
  optional (a console or insert section's effect identity), and for a route owner 7 `source` `Utf8`,
  8 `destination` `Utf8`, 9 `tap` `U8` (the `SendTap` code). A control row adds 10 `name` `Utf8`,
  11 `display_name` `Utf8`, 12 `display_unit` `Utf8`, 13 `unit` `U8` (`effect_contract::ParameterUnit`
  codes), 14 `kind` `U8` (continuous 1, toggle 2, enumeration 3), 15 `minimum` `F32`, 16 `maximum`
  `F32`, 17 `disabled_value` `F32` optional, 18 `default` `F32`, 19 `taper` `U8` (linear 1,
  logarithmic 2, exponential 3, decibel 4, stepped 5), 20 `step` `F32`, 21 `precision` `U8`,
  22 `ladder` `PackedF32`, 23 `choice` `Message` repeated (the existing `enum_choice` spec:
  `value F32`, `label Utf8`), 24 `ramp_row` `U8` optional (absent: fixed; codes are `LiveRampRow`'s
  declaration order from 1: fader 1 ... send matrix 15, pinned by a test), 25
  `default_ramp_samples` `U32`, 26 `class` `U8` (live 1, rebuild-only 2, derived 3), 27 `reason`
  `U8` optional (new memory 1, new graph 2, processing order 3, latency 4, glitch 5, optimisation 6,
  correctness 7, ruled live not built 8; present exactly when `class` is 2), 28 `reason_issue`
  `U32` optional (present exactly with reason 8), 29 `reopen` `Utf8` optional (present exactly when
  `class` is 2), 30 `scope` `U8` (model 1, monitor 2), 31 `automatable` `Bool`, 32 `family` `U8`
  (strip level 1, strip matrix 2, strip input 3, route 4, effect 5, setting 6), 33 `active` `Bool`,
  34 `value` `F32`. Every code table is one Rust enum with explicit discriminants in `protocol`,
  and a value outside it is `INVALID_FIELD`.
- **D6. State page** (`control_state_page`): 1 `revision` `U64`, 2 `structure_epoch` `U64`,
  3 `last_handle` `U32`, 4 `eof` `Bool`, 5 `count` `U16`, 6 `record_bytes` `U16` (= 8), 7 `records`
  `Bytes` (per control row: `handle` `u32` LE, `value` `f32` LE; header rows are skipped; handles
  strictly ascending).
- **D7. Provider trait.** `ControlProvider::parameter_metadata`, `parameter_state` and
  `parameter_descriptor` are replaced by `control_list(&ControlListRequest) ->
  Result<ControlListPage, ParameterProviderError>`, `control_state(&ControlListRequest) ->
  Result<ControlStatePage, ParameterProviderError>` and `control_row(handle) ->
  Result<&ControlEntry, ParameterProviderError>` (the transient read `AUTOMATION_ENQUEUE`'s domain
  check uses; it keeps its present rules: an automatable continuous, toggle or enumeration row).
  `ParameterProviderError` gains `Stale`, which the controller answers with
  `REVISION_CONFLICT` and the diagnostic code `control.list.stale`. The test `MockProvider`
  (`controller.rs:625-745`) serves a small fixed list.
- **D8. Docs.** The registry, conformance, semantics and sizing docs and the fuzz manifest describe
  the two new messages and list `0x0004` and `0x0005` as retired.

## Deliverables

1. D1-D7 in `crates/protocol` (wire, schema, codec, typed frame, controller, `lib.rs` re-exports).
2. The conformance corpus frames, the retired-code rows and the re-pinned `COMPLETE_SCHEMA_HASH`.
3. D8.
4. The tests below. Tests of the deleted messages are deleted or rewritten for the new ones.

## Authorized paths

- `crates/protocol/src/{wire.rs,message_wire.rs,message_wire/tests.rs,schema.rs,typed_frame.rs,
  controller.rs,controller/tests.rs,lib.rs}`.
- `crates/conformance/src/protocol_corpus.rs`, `crates/conformance/tests/conformance_corpus.rs`,
  `fuzz/corpus/complete-schema-manifest.md`.
- `crates/host-core/src/control_provider.rs` and `crates/control-plane/src/compile.rs`: only what
  the trait change needs to compile, with every new method answering `NotFound` until #1552. The
  capability advertised by the C ABI keeps bit 7 clear until #1552.
- `crates/capi/src/runtime/{tests.rs,live_tests.rs}`, `tools/audit/src/protocol.rs`: delete or
  rewrite the uses of the removed messages.
- `docs/CONTROL_PROTOCOL_{REGISTRY,CONFORMANCE,SEMANTICS,SIZING}.md`, `docs/CONTROL_BTLV_V1.md`.

## Non-goals

- No catalog built from `controls`, no `structure_epoch` bookkeeping, no C ABI service (#1552).
- No `SET_CONTROL` (#1553). No change to `AUTOMATION_ENQUEUE`'s record layout (#1567).

## Objective gates

Commands: `cargo test --locked -p protocol --all-features`, `cargo test --locked -p conformance`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo test --locked -p host-core --all-features`.

1. **Round trip and strict refusal.** Codec tests encode and decode a request, a list page with
   one owner header, one section header and control rows of each `class`, `kind` and optional
   field, and a state page. Each D3 and D5 presence rule, each code table, a limit of 0 or 257,
   descending state handles and a wrong `record_bytes` are refused on encode and on decode alike.
2. **Retired codes.** A frame with message ID `0x0004` or `0x0005` is answered
   `UNSUPPORTED_MESSAGE`; the capability registry lists `0x000c` and `0x000d` and not the retired
   two; the conformance corpus carries both retired rows.
3. **Dispatch.** The controller's full-frame tests dispatch both new commands through typed frames
   to the `MockProvider`, and a stale `structure_epoch` returns `REVISION_CONFLICT` with
   `control.list.stale`.

## Test value

- Gate 1 turns red if the codec accepts a row whose fields contradict its kind (a header with a
  value, a rebuild-only row with no reason), or the two directions disagree.
- Gate 2 turns red if a retired code is reused or still served.
- Gate 3 turns red if a new message is not reachable through the frame path or the stale refusal is
  untyped.

## Dependencies

- #1546 and #1549 (the vocabulary and display strings the codes describe).

## Supersedes and shrinks

- None. #1552 serves the messages.

## Decision record (batch 2 review fold, 2026-10-13)

- **D-fold-1. One address codec.** The address message has one encoder and one decoder, shared by
  the control list and `SET_CONTROL` (#1553 D13), and one set of conversions to and from
  `controls::ControlAddress`; `host-core`'s list builder uses the same conversion with a fallible
  ID copy. `EncodeError::InvalidField` (#1552) is the encoder's status for an address or a request
  that breaks the rules.

