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

## Decision record

Implemented on `codex/controls-batch-6k` (batch 5, #1600, #1572 and #1591 present). Adaptations to
the frozen decisions; anchors moved (#1599 compact catalog, #1561 `SET_CONTROL`-only).

- **D1, IDs.** As frozen: `MessageId::ResponseGet = 0x000e`, allocated commands `1..=3 | 6..=14`,
  `ProviderFeatures::response` (in `ALL`, so the C ABI endpoint serves it), bit 14, `KNOWN` and
  `B4_BASE` are `(1 << 15) - 1`. The capability invariants tie bit 14 to `0x000e` in both
  directions. The capability set holds 12 command IDs.
- **D2/D3, types.** The request and reply are typed in `crates/protocol/src/message_wire/response.rs`
  (a child of `message_wire`, because that file is 4.9k lines): `ResponseGetRequest` with
  `ResponseTarget::{Configuration, Address, Strip}`, `ResponseConfiguration::{Effect,
  InputFilters}` (quality and link mode are `session::EffectQuality` and `session::LinkMode`, codes
  1 to 3), `ResponseGrid`, `ResponseOverride`; the reply is `ResponseResult::{Tag, Curves}`
  (`ResponseCurves`). The spec's `ResponseGetRequest<'_>` is owned (no lifetime), like
  `ControlListRequest`, because the decoded command is owned. The reply type is `ResponseResult`,
  not `ResponseGetReply`. A curve reply with `request_tag` is refused, as D3 says ("field 1 alone").
  Reply values are all finite, at least 2 points. The sections arrays are also refused when
  `section_count` is 0.
- **Bypass rule in the codec.** Gate 1 lists "a bypass override with `left`" among the codec's
  refusals, so the codec refuses, for an effect configuration, a control-0 override that is not lane
  `both` or whose value is not exactly 0 or 1 (`INVALID_FIELD`, no diagnostic code). The provider
  repeats the lane check (`response.override.lane`) and the value check
  (`response.override.control`) for a typed caller that skips the wire. Consequence: the EQ has no
  shared parameter, so `response.override.lane` cannot be reached by a wire request on a
  configuration target (gate 3 asks only for codes a configuration target can reach); it is pinned
  by the refusal table test in `crates/host-core/tests/response_query.rs`. `response.rate`,
  `response.numerical` and `response.internal` are likewise unreachable on a session endpoint (its
  rate is a launch rate; no EQ query fails numerically) and are pinned by the same table test.
- **D4, lane rule.** `apply_overrides` follows the rule; the old "sentinel" duplicate test is gone
  (pairwise overlap decides). The rows of the input filters (controls 3 and 4, found by name in
  `BUILTIN_PARAMETER_DESCRIPTORS`) fold into the four cutoffs in the new
  `host_core::input_filter_target`. The browser gets the effect rule through the same function
  (`cargo test -p host-web --all-features` passes unchanged). `crates/host-core/tests/randomized.rs`'s
  independent override law and its defect generator follow the new rule.
- **D5, rate.** The provider reads the sample rate and quantum from the committed model the
  controller passes (`ControlProvider::response_get(&SessionModel, &ResponseGetRequest)`), not from
  a copy given at `try_new`, so a committed `SetSampleRateHz` can never leave a stale rate. The
  `try_new` argument that `compile.rs` passes is therefore the reply bound
  (`maximum_reply_bytes` = the host's control frame limit, which is both the endpoint's frame limit
  and the replay cache's response bound there), not a rate and a quantum.
- **D6, one table.** `ResponseQueryError::refusal` and `ResponseRefusal` are in
  `host_core::response`; the provider maps the class to `StatusCode`. Zero `maximum_prepared_bytes`
  is refused by the provider before preparation (`response.limit`); a count over 256 overrides is
  refused for both kinds.
- **D7, serving.** The controller also checks the reply it is given against the response bound
  (`fit_response`: `LIMIT_EXCEEDED`, `response.reply_too_large`), so a provider that did not size
  first still cannot cache an oversized reply. The provider sizes from `response_result_encoded_len`
  (the exact frame length, header included) after the owner is prepared and before `curves()`.
  `MockProvider` serves the configuration target with a deterministic shape and counts its calls.
- **Test hooks.** `host_core::take_response_counts` (test-support) counts owner preparations and
  evaluations on the thread; gate 3's size rule and gate 5's replay use it.
- **Files outside the authorized list.** `tools/audit/src/protocol.rs` (one `RESPONSE_GET` command
  in the allocation audit corpus), `tools/realtime-policy/src/policy.rs` and
  `docs/REALTIME_DEPENDENCY_POLICY.md` (the new test file calls the C entries, so it joins the
  unsafe allowlist beside `plan_swap_race.rs`), `crates/conformance/src/main.rs` and
  `crates/conformance/tests/mutation_million.rs` (the frame count, 47 to 50),
  `scripts/check-protocol-wasm-parity.sh` (the pin and the count in its self-test),
  `crates/capi/src/runtime/tests.rs` (two pinned capability frames gain `0e00` and `ff7f`),
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md` and `docs/C_ABI_V1_QUALIFICATION.md`.
- **Re-pin.** `COMPLETE_SCHEMA_HASH` `7de408612b837afd` to `cdfdb0540f093424`: the corpus gains
  `command.response_get`, `response.response_get` and `response.response_get_tag` (50 frames), and
  the capabilities frame lists `0x000e` with bit 14.
- **Pre-existing, not touched.** `scripts/check-env-vocabulary.sh` fails on
  `MISO_BLESS_FIXTURES` in `crates/protocol/tests/set_control_frames.rs` (#1563).

**Batch 6 review fold, 2026-10-14.**

- **Two capability frames, not three.** The record above counted three pinned capability frames in
  `crates/capi/src/runtime/tests.rs`; two changed (each gains `0e00` and `ff7f`).
- **The provider asks the core first.** `serve_response` used to fold the override rows (the bypass,
  the input-filter cutoffs) and check the byte budget before the core saw the request, so a request
  with a bad point count and a bad bypass value answered the bypass. The core's rate, point-count,
  override-count and axis checks are now one function (`check_response_request`) that
  `prepare_response_query` and the provider both call, and the provider calls it first, ahead of its
  own rules (`response.rate.session`, the zero byte budget, the bypass rows). Request order of
  refusal: the core's, then the provider's. A bypass value other than exactly 0 or 1 is
  `response.override.bypass` (`INVALID_FIELD`), no longer the shared `response.override.control`;
  the wire codec still refuses it earlier without a code, so only a typed caller reaches it. Pinned
  by `response_get_asks_the_core_first_and_names_a_bad_bypass`, and listed in the registry.
- **Tests that tell a lane swap and a hard-coded rate.** `a_one_lane_section_and_another_session_rate_reach_the_reply`
  enables band 3 on the left lane only (the left mask has it, the right mask and curve do not move),
  and answers sessions at 44.1 and 96 kHz with the core's curves at that rate and an axis ending at
  that rate's Nyquist, refusing one hertz past it.
- **Sizing.** `docs/CONTROL_PROTOCOL_SIZING.md` records the reply worst case (245,976 bytes: 4,096
  points, both sides, the EQ's 6 sections) and the request bound (16,672 bytes plus the effect ID's
  field; 82,216 bytes at the default string limit).
- **Axis before preparation.** See the #1572 record: the axis rules run before any owner is prepared
  and the request codec refuses non-finite endpoints and override values.
