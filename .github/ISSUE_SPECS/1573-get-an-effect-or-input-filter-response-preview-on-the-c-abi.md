# Get an effect or input-filter response preview on the C ABI

Slice S16 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

An iOS or Android app asks for the frequency response of a native effect (or of the input filters)
for parameters it chooses, with one `RESPONSE_GET` command through
`miso_engine_v1_submit_command`, and gets the curves in the reply. The reply comes from the same
core function the browser uses (#1572), at the session's sample rate. This slice freezes the whole
`RESPONSE_GET` wire, for all three targets; the `address` and `strip` targets are refused until
#1575 and #1578 serve them.

## Context

- **No response on the C ABI.** The header (`crates/capi/include/miso_engine_v1.h`) and
  `crates/capi/src/ffi.rs` have no response symbol. The one generic entry is
  `miso_engine_v1_submit_command` (`crates/capi/src/ffi.rs:685`), which runs
  `SessionState::command` (`crates/control-plane/src/control.rs:1082`). An immediate command is
  served by the controller with its provider (`PreparedCommandFrame::Immediate`, `:1093-1095`).
- **Message IDs.** `MessageId` (`crates/protocol/src/wire.rs:98`, last command `DiagnosticsGet =
  0x000b`, `:120`; `from_raw`, `:143-165`). #1551 adds `0x000c` and `0x000d` and retires `0x0004`
  and `0x0005`; its allocated command set is `1..=3 | 6..=13` (`allocated_id`,
  `crates/protocol/src/message_wire.rs:3534-3540`).
- **Pattern to copy.** A command with a request and a typed reply: the `ParameterStateGet` arms of
  the typed frame (`crates/protocol/src/typed_frame.rs`, `CommandPayload` `:42`,
  `SuccessResponsePayload` `:66`, `DecodedCommandPayload` `:118`, decode `:368`), the controller
  (`ControlCommand`, `crates/protocol/src/controller.rs:898`; decode `:1496`; feature gate
  `:2826-2827`; execute from `:2949`; encode `:2243`), the provider trait (`ControlProvider`,
  `:565`) and its test `MockProvider`, and the capability registry (`:3220-3275`). Capability
  flags use bits 0 to 13 (`CapabilityFlags::KNOWN = (1 << 14) - 1`,
  `crates/protocol/src/message_wire.rs:60`; invariants `:3495-3532`).
- **Errors.** A non-OK response carries diagnostics with a `code` string
  (`docs/CONTROL_PROTOCOL_REGISTRY.md:17-19`); statuses are `StatusCode`
  (`crates/protocol/src/wire.rs:179-218`: `INVALID_FIELD` 5, `LIMIT_EXCEEDED` 6, `BACKPRESSURE` 11,
  `VALIDATION_FAILED` 12, `NOT_FOUND` 13, `UNAVAILABLE` 14, `INTERNAL` 18).
- **The provider** is `SessionControlProvider` (`crates/host-core/src/control_provider.rs:53`,
  `impl ControlProvider` `:301`), built in `crates/control-plane/src/compile.rs:854`. The
  controller holds the committed session (`self.controller.session().compiled().normalized_model()`,
  `crates/control-plane/src/control.rs:1362`).
- **The reply must fit.** Every immediate reply is cached for exact replay
  (`ReplayDecision`, `crates/protocol/src/controller.rs:153-167`), and the endpoint's frame limit is the host's
  `maximum_control_frame_bytes` (`crates/capi/src/abi.rs:156`).
- **The core function** is #1572's `prepare_response_query` / `curves()`.

## Decisions frozen for this slice

- **D1. IDs.** `MessageId::ResponseGet = 0x000e` (contract name `RESPONSE_GET`). The allocated
  command set becomes `1..=3 | 6..=14`. `ProviderFeatures` gains `response: bool`; capability flag
  bit 14 means "serves `0x000e`", and `KNOWN` becomes `(1 << 15) - 1`. #1578 adds the event
  `0x8040` and ties bit 14 to both.
- **D2. Request** (`response_request`):
  1 `configuration` `Message` O; 2 `address` `Message` O (#1551 D3's `control_address` with
  `control` and `lane` absent); 3 `strip_id` `Utf8` O; exactly one of 1-3 is present.
  4 `grid` `U8` R (linear 1, logarithmic 2); 5 `points` `U32` R; 6 `minimum_hz` `F32` R;
  7 `maximum_hz` `F32` R; 8 `channels` `U8` R (left 1, right 2, both 3); 9 `sections` `Bool` R;
  10 `override` `Message` O* (`response_override`: 1 `control` `U32` R, 2 `lane` `U8` R
  (left 1, right 2, both 3), 3 `value` `F32` R); 11 `configuration_id` `U64` O; 12 `sample_rate_hz`
  `U32` O; 13 `quantum_frames` `U32` O; 14 `maximum_prepared_bytes` `U64` O; 15
  `maximum_state_bytes` `U64` O; 16 `maximum_scratch_bytes` `U64` O; 17 `maximum_automation_spans`
  `U32` O; 18 `maximum_reply_bytes` `U32` O.
  `response_configuration`: 1 `kind` `U8` R (effect 1, input filters 2); 2 `effect` `Utf8` O,
  3 `quality` `U8` O (`EffectQuality` code), 4 `link_mode` `U8` O (`LinkMode` code), all three
  present exactly for kind 1.
  The codec refuses with `INVALID_FIELD`: not exactly one target; a code outside its table; a
  `strip_id` that is not a valid `StableId`; for `strip_id`, `sections` true or any `override`.
- **D3. Reply** (`response_result`): 1 `request_tag` `U64` O; 2 `sample_rate_hz` `U32` O;
  3 `configuration_id` `U64` O; 4 `floor_db` `F32` O; 5 `bypass` `Bool` O; 6 `section_count` `U32`
  O; 7 `enabled_left` `U32` O; 8 `enabled_right` `U32` O; 9 `retained_bytes` `U64` O;
  10 `frequencies_hz` `PackedF32` O; 11 `left_db` `PackedF32` O; 12 `right_db` `PackedF32` O;
  13 `sections_left_db` `PackedF32` O; 14 `sections_right_db` `PackedF32` O (section-major).
  Either field 1 alone (the `strip` target, #1578), or fields 2-10 with at least one of 11 and 12;
  11 and 12 have the length of 10; 13 and 14 have `section_count` times that length, and each is
  present only with its side's total. The codec refuses every other shape on encode and decode.
  `protocol` gains `response_result_encoded_len(points, channels, section_count, sections) ->
  Option<usize>`, the exact frame length of a curve reply.
- **D4. Overrides follow the decision-17 lane rule.** Kind 1: `control` 0 is the bypass (`both`
  only; value exactly 0 or 1); any other `control` is a descriptor parameter ID. Kind 2: `control`
  3 (`hpf_hz`) and 4 (`lpf_hz`) only (the builtin IDs of `BUILTIN_PARAMETER_DESCRIPTORS`,
  `crates/builtins/src/lib.rs:462`); an absent lane takes the row's default. `both` on a per-lane
  parameter sets both lanes; `left` or `right` on a shared parameter is refused. A (control, lane)
  pair given twice, or `both` with `left` or `right` of the same control, is a duplicate.
  `apply_overrides` (`crates/host-core/src/response.rs:588`) changes to this rule (today it refuses
  `both` on a per-lane parameter), so the browser gets it too.
- **D5. Rate.** On a session endpoint the rate and quantum are the session's; fields 12 and 13
  must be absent (`response.rate.session`). Fields 14-17 are optional; absent means
  `ResponsePreviewLimits::default()` (`response.rs:75-84`).
- **D6. One refusal table, in host-core.** `ResponseQueryError::refusal(&self) -> (ResponseRefusal,
  &'static str)` maps every error to a status and a code:
  `Points` -> `INVALID_FIELD` `response.points`; `Overrides` -> `LIMIT_EXCEEDED`
  `response.override.count`; `SampleRate` and `InvalidShape` -> `INVALID_FIELD` `response.rate`;
  `InvalidGrid` and owner `InvalidFrequencyGrid` -> `INVALID_FIELD` `response.grid`;
  `UnknownEffect` -> `NOT_FOUND` `response.effect.unknown`; `UnsupportedEffect` and owner
  `UnsupportedMode`/`UnsupportedCapability` -> `UNAVAILABLE` `response.effect.unsupported`;
  `InvalidParameter` -> `INVALID_FIELD` `response.override.control`; `ConflictingChannel` ->
  `INVALID_FIELD` `response.override.lane`; `DuplicateParameter` -> `INVALID_FIELD`
  `response.override.duplicate`; owner `Configuration(e)` -> `VALIDATION_FAILED` with `e.code`;
  owner `Capacity`/`ResourceLimit` and a zero `maximum_prepared_bytes` -> `LIMIT_EXCEEDED`
  `response.limit`; owner `Numerical` -> `INTERNAL` `response.numerical`; `OutputShape` ->
  `INTERNAL` `response.internal`. Further codes: `response.reply_too_large` (`LIMIT_EXCEEDED`),
  `response.target.unavailable` (`UNAVAILABLE`).
- **D7. Serving.** `ControlProvider` gains `fn response_get(&mut self, model:
  &session::SessionModel, request: &ResponseGetRequest<'_>) -> Result<ResponseGetReply,
  ResponseProviderError>`; the controller passes the committed model. `SessionControlProvider`
  keeps the session's rate and quantum (given at `try_new`) and, for a `configuration` target:
  builds the #1572 `ResponseQuery`, calls `prepare_response_query`, checks
  `response_result_encoded_len` against the smaller of the endpoint's maximum frame bytes, the
  replay cache's maximum response bytes and field 18 (`response.reply_too_large`), then calls
  `curves()`. An `address` or `strip_id` target is refused `response.target.unavailable`. The
  controller writes a non-OK reply with one error-severity diagnostic carrying the code.
- **D8. A read changes nothing.** `RESPONSE_GET` writes no session, revision, catalog or cell.
  Its reply is cached for exact replay like every reply.

## Deliverables

1. D1-D3 in `crates/protocol` (wire, schema, message-wire codec and `response_result_encoded_len`,
   typed frame, controller dispatch and capability, `MockProvider`, `lib.rs` exports).
2. D4 and D6 in `crates/host-core/src/response.rs`; D7 in `crates/host-core/src/control_provider.rs`
   and the rate and quantum argument at `crates/control-plane/src/compile.rs:854`.
3. `ProviderFeatures::response = true` for the C ABI endpoint.
4. The conformance corpus frames for the request and both reply forms, and the re-pinned
   `COMPLETE_SCHEMA_HASH` (`crates/conformance/src/protocol_corpus.rs:764`).
5. Docs: `docs/CONTROL_PROTOCOL_REGISTRY.md` (the `000e` row, its fields and codes),
   `docs/CONTROL_PROTOCOL_SEMANTICS.md` (rate from the session; replay), the C header comment of
   `miso_engine_v1_submit_command` (names `RESPONSE_GET`; no C symbol or struct change),
   `fuzz/corpus/complete-schema-manifest.md`.

## Authorized paths

- `crates/protocol/src/{wire.rs,schema.rs,message_wire.rs,message_wire/tests.rs,typed_frame.rs,
  controller.rs,controller/tests.rs,lib.rs}`.
- `crates/host-core/src/{response.rs,control_provider.rs,lib.rs}`, `crates/host-core/tests/`.
- `crates/control-plane/src/compile.rs` (the provider arguments only).
- New `crates/capi/tests/response_get.rs`; `crates/capi/include/miso_engine_v1.h` (comment only).
- `crates/conformance/src/protocol_corpus.rs`, `crates/conformance/tests/conformance_corpus.rs`,
  `fuzz/corpus/complete-schema-manifest.md`, `docs/CONTROL_PROTOCOL_{REGISTRY,SEMANTICS,SIZING}.md`.

## Non-goals

- No `address` target (#1575), no `strip` target and no event (#1578).
- No browser change beyond D4's lane rule (#1574 sends the frame from the browser).
- No new C export.

## Objective gates

Commands: `cargo test --locked -p protocol --all-features`, `cargo test --locked -p conformance`,
`cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`bash scripts/check-capi-abi.sh`.

1. **Codec.** Round trip of a configuration request (both kinds, every optional field) and of a
   curve reply with and without sections; refusal on encode and on decode of each D2/D3 shape
   rule (two targets, no target, `strip_id` with `sections` true, a bypass override with `left`,
   a 13 field without 11, a 11 field of the wrong length, a code outside its table).
2. **Same curves as the core.** In `crates/capi/tests/response_get.rs`: compile a 48 kHz session
   with a 1 MiB frame limit; send `RESPONSE_GET` for `miso.parametric-eq` with asymmetric band
   gains, a disabled band and `sections` true, and for input filters with different lane cutoffs.
   Each reply's curves and masks are bit-equal to #1572's `curves()` called directly at 48 kHz and
   the session quantum. A `both` override of a per-lane gain gives the same curves as a `left` and
   a `right` override of the same value.
3. **Refusal table.** One request per D6 code that a configuration target can reach, each
   answered with its status and code; `sample_rate_hz` present gives `response.rate.session`;
   on a second session compiled with a 4096-byte frame limit, a 4096-point request gives
   `response.reply_too_large` before any curve is computed;
   `address` and `strip_id` targets give `response.target.unavailable`. After each refusal,
   `SESSION_SNAPSHOT_GET` and the revision are unchanged.
4. **Capability.** `CAPABILITIES_GET` lists `0x000e` and sets bit 14; a peer frame that lists
   `0x000e` without bit 14 is refused.
5. **Replay.** The same request ID and bytes sent twice return byte-identical replies; the second
   call prepares no owner.

## Test value

- Gate 1 turns red if the codec admits a reply whose arrays do not match its declared shape.
- Gate 2 turns red if the C ABI computes its curves anywhere but in the core, at another rate, or
  with today's lane rule.
- Gate 3 turns red if a refusal is untyped, wrongly classed, or changes state.
- Gate 4 turns red if the new message is not advertised, or advertised without its flag.
- Gate 5 turns red if a replayed read runs again.

## Dependencies

- #1572 (the core function).
- #1551 (the allocated command set and `control_address`; `0x000e` follows `0x000d`).
- `crates/protocol`, `crates/control-plane` and `crates/capi` are stream B's: rebase after stream
  B's open slices that touch the controller and the capability registry.

## Supersedes and shrinks

- Shrinks *Engine-owned analysis: EQ response, spectrum, selective SDK streams, and sample-time
  correlation* (#763): the response preview reaches the C ABI (see the amendment record).
