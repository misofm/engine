# Preview a response in the browser with the same protocol frame as the C ABI

Slice S17 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

The SDK's response preview (`createResponsePreview`, browser and headless) sends the same
`RESPONSE_GET` frame that a C ABI host sends, and decodes the same reply bytes. The preview
instance runs the core's code for it, and the browser's own request and result records, their
eleven exports and the SDK's layout code are deleted. The public SDK preview API does not change.

## Context

- **The browser path today.** The SDK fills `WebResponseRequest` (`hosts/host-web/src/lib.rs:517`)
  and `WebResponseParameter` (`:551`) rows in Wasm memory (`ResponsePreviewModule.#query`,
  `sdk/src/core/response.ts:337-418`), calls `miso_engine_web_v1_response_query`
  (`hosts/host-web/src/ffi.rs:2467`) and reads `WebResponseResult` (`lib.rs:562`) back
  (`response.ts:419-472`). The eleven exports are `ffi.rs:2405-2508`. After #1572,
  `run_response_query` (`ffi.rs:2184`) only decodes, calls the core and lays out.
- **The preview instance has no session.** The browser runs it in the SDK's response Worker
  (`sdk/src/browser/response-worker.ts:69-72`, a standalone `WebAssembly.Instance` with no
  imports); headless runs it in its own instance (`sdk/src/headless/response.ts:30`). The public
  entry is `createResponsePreview` (`sdk/src/browser/index.ts:19`, `sdk/src/browser/response.ts:199`;
  `sdk/src/headless/response.ts:45`). With no session there is no `SessionState`, so #1562's
  generic command entry cannot serve it.
- **SDK logic beyond transport.** `responseParameterRows` (`response.ts:267-284`) splits a
  per-lane `both` into `left` and `right` rows, because the core refused `both` on a per-lane
  parameter before #1573 D4. `normalizedConfiguration` (`:239-265`) fills defaults for the
  `requestedConfiguration` echo. Names map to IDs through the generated catalog (`parameterRows`).
- **Mirrors of the old exports.** `tools/parameter-metadata/src/abi_layout.rs` (export names
  `:195-205`; `response_request_fields` `:695`, `response_parameter_fields` `:803`,
  `response_result_fields` `:820`; structure rows `"responseRequest"` `:1890` and its two
  neighbours; constant tables `"responseTargets"` `:2091`, `"responseFields"` `:2116`),
  `scripts/check-abi-layout-v1.py:177-187`, `scripts/check-web-audioworklet.sh:263-273`,
  `sdk/src/generated/abi.ts` and `sdk/assets/miso-engine-v1-abi-layout.json` (generated).
- **The staging.** `ResponseStaging` (`ffi.rs:109`) holds the preview fields and the track-response
  fields together; `live_response_staging_retained_bytes` (`ffi.rs:381-389`) charges both.
- **The wire and the codec.** #1573 freezes `RESPONSE_GET`; #1563's
  `sdk/src/core/control-frame.ts` is the SDK's frame codec; `ProtocolCodec::decode_typed_command`
  (`crates/protocol/src/typed_frame.rs:368`), `encode_success_response_frame_into` (`:278`) and
  `encode_non_ok_response_frame_into` (`:317`) are the Rust side.

## Decisions frozen for this slice

- **D1. The session-less core entry.** `host_core::serve_response_frame(request: &[u8], reply:
  &mut Vec<u8>) -> Result<(), ResponseFrameError>` (feature `control-provider`). It decodes one
  command frame with the frame limit `1 << 20`; a frame whose header does not decode is
  `ResponseFrameError::Malformed` and writes no reply. Any other message is answered with a
  non-OK `UNSUPPORTED_MESSAGE` frame. For `RESPONSE_GET` it requires a `configuration` target
  (else `UNAVAILABLE` `response.target.unavailable`) and both `sample_rate_hz` and
  `quantum_frames` (else `INVALID_FIELD` `response.rate`), then runs the same private function as
  #1573's provider (one function for both: build the query, prepare, check the reply length,
  compute, encode). The reply echoes the request ID with revision 0.
- **D2. One export family, justified.** The frozen per-feature decision gives the session-less
  preview one bytes-in/bytes-out export family, in `hosts/host-web/src/ffi.rs`, on no render path
  and never called by a worklet:
  `miso_engine_web_v1_response_frame_ptr(bytes: u32) -> u32` (staging for a request of `bytes`
  bytes, `1..=1 << 20`; `0` otherwise), `miso_engine_web_v1_response_frame_submit(bytes: u32) ->
  u32` (`RESULT_OK` when a reply frame exists, OK or non-OK; `RESULT_INVALID_ARGUMENT` for
  `Malformed`), `miso_engine_web_v1_response_frame_reply_ptr() -> u32`,
  `miso_engine_web_v1_response_frame_reply_bytes() -> u32` and
  `miso_engine_web_v1_response_frame_close() -> u32` (frees both buffers).
- **D3. Deleted.** The eleven `miso_engine_web_v1_response_*` exports; `run_response_query`,
  `response_buffer_budget`, `response_failure`, `response_header_bytes`, `append_f32_values`,
  `response_error_code`, `response_grid`, `response_quality`, `response_link_mode` and
  `response_channel`; `WebResponseRequest`, `WebResponseParameter`, `WebResponseResult`,
  `RESPONSE_TARGET_*`, `RESPONSE_FIELD_*`, `RESPONSE_MAXIMUM_EFFECT_ID_BYTES`,
  `RESPONSE_MAXIMUM_PARAMETER_OVERRIDES`, `RESPONSE_MAXIMUM_RESULT_BYTES` and the three
  `RESPONSE_*_BYTES` sizes; the preview fields of `ResponseStaging` and their terms in
  `live_response_staging_retained_bytes`; every mirror row listed in Context. The grid and channel
  constants that the track response still reads stay until #1580.
- **D4. The SDK.** A new `sdk/src/core/response-frame.ts` encodes a `RESPONSE_GET` request and
  decodes the `response_result` reply and a non-OK reply's diagnostics, with #1563's TLV
  primitives. `ResponsePreviewModule` (`response.ts:287`) calls D2's exports with it:
  - a parameter name maps to its ID through the generated catalog as today; `both` is sent as
    `both` (`responseParameterRows`' split is deleted); bypass is control 0; input-filter cutoffs
    are controls 3 and 4; quality and link mode keep their codes;
  - `maximumPreparedBytes`, `maximumTotalStateBytes`, `maximumScratchBytes`,
    `maximumAutomationSpansPerBlock` and `maximumResultBytes` become fields 14-18;
  - `ResponsePreviewResult` keeps every field; `resultBytes` is the reply frame length;
    `requestedConfiguration` keeps the `normalizedConfiguration` echo (presentation only);
  - a non-OK reply throws `MisoEngineError` whose `code` is the diagnostic code and whose `result`
    is `invalidArgument` (`INVALID_FIELD`, `VALIDATION_FAILED`, `NOT_FOUND`), `unsupported`
    (`UNAVAILABLE`), `refusedBudget` (`LIMIT_EXCEEDED`) or `internal` (`INTERNAL`).
- **D5. One wire.** The preview's reply payload equals the C ABI's for the same request.
- **D6. One generated bound.** The ABI layout generator emits a new constant
  `maximumResponsePoints` (`host_core::RESPONSE_MAXIMUM_POINTS`, 4096) into
  `sdk/src/generated/abi.ts`; #1579 reads the SDK's track-response bounds from it. The SDK does not
  check points itself: the engine refuses with `response.points`.

## Deliverables

1. D1 in `crates/host-core/src/response.rs` (sharing #1573's serving function) and its export.
2. D2 and D3 in `hosts/host-web` and the mirrors.
3. D4 in `sdk/src/core/` and the SDK evals.

## Authorized paths

- `crates/host-core/src/{response.rs,control_provider.rs,lib.rs}`, `crates/host-core/tests/`.
- `hosts/host-web/src/{ffi.rs,lib.rs}`, `hosts/host-web/Cargo.toml` (the `control-provider`
  feature of `host-core`, if #1562 has not enabled it), `hosts/host-web/src/tests.rs`.
- `tools/parameter-metadata/src/abi_layout.rs`, `scripts/check-abi-layout-v1.py` and its
  self-test fixture, `scripts/check-web-audioworklet.sh`, `sdk/src/generated/abi.ts` and
  `sdk/assets/miso-engine-v1-abi-layout.json` (regenerated, never hand-edited).
- `sdk/src/core/{response.ts,response-frame.ts}`, `sdk/test/response-evals.mjs`,
  `hosts/host-web/qualification/sdk-response-entry.ts` (the preview rows only).

## Non-goals

- No change to the track response or its exports (#1579, #1580).
- No preview through a booted engine's command entry (the preview needs no session).
- No change to the SDK's public preview types.

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`, `cargo test --locked -p host-web`,
`bash scripts/check-web-audioworklet.sh`, `python3 -B scripts/check-abi-layout-v1.py`,
`bash scripts/check-sdk-types.sh`,
`bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
`bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
`python3 -B scripts/check-sdk-deletions.py`,
`bash scripts/sdk-package.sh check target/ci/qualification-artifacts`, and the browser
qualification (`npm run qualify -- --artifacts ... --sdk-root ... --browser <b> --check-matrix
--self-test-mutations` in `hosts/host-web/qualification`) for chromium, firefox and webkit.

1. **One wire.** A host-core test sends one configuration request (EQ, sections, both lanes) to
   `serve_response_frame` at 48 kHz and the same request without fields 12-13 to a 48 kHz C ABI
   session (#1573's test session); the two reply payloads (after the 48-byte header) are
   byte-identical. This is a wire format, so a byte comparison is the right gate.
2. **Session-less refusals.** `serve_response_frame` answers an `address` target with
   `response.target.unavailable`, a missing rate with `response.rate`, a `CAPABILITIES_GET` frame
   with `UNSUPPORTED_MESSAGE`, and returns `Malformed` for a 10-byte input.
3. **SDK parity.** `sdk/test/response-evals.mjs` passes on the headless and browser legs with the
   eval inputs unchanged. One new eval: a per-lane gain given as `both` gives curves bit-equal to
   the same value given as `left` and `right`. One new eval: 4097 points throws `MisoEngineError`
   with `code` `response.points` and `result` `invalidArgument`.
4. **The old surface is gone.** The ABI layout check and `check-web-audioworklet.sh` list the five
   D2 exports and none of the eleven old ones; `check-sdk-deletions.py` refuses the deleted SDK
   names.

## Test value

- Gate 1 turns red if the browser and the C ABI encode a different reply for one request.
- Gate 2 turns red if the session-less entry serves a target it cannot resolve or invents a rate.
- Gate 3 turns red if the SDK sends a different request than before, or loses the error code.
- Gate 4 turns red if an old export survives in one list while another drops it.

## Dependencies

- #1572 and #1573 (the core function and the wire).
- #1563 (the SDK frame codec, `sdk/src/core/control-frame.ts`).
- #1562 (host-web links `control-plane`, so `host-core`'s `control-provider` code is in the
  artifact).
- Stream H owns `hosts/host-web` and `sdk/`; `tools/parameter-metadata/src/abi_layout.rs` follows
  `docs/handoffs/decision-15-2026-10-05/STREAMS.md` row 105: this slice rebases after those.
- #1610 (session-less response core function), root 2026-10-14.

## Supersedes and shrinks

- None beyond #1573's amendment of #763.

## Amendment (controls coordinator, batch 9, 2026-10-14)

Checked against `main` at `7feb27800`.

- **#1610 landed (batch 8) with a different shape from D1's text.** The core function is
  `pub(crate) fn serve_response_at(sample_rate_hz: u32, quantum_frames: u32, maximum_reply_bytes:
  usize, request: &ResponseGetRequest, session: bool) -> Result<ResponseResult,
  ResponseProviderError>` in `crates/host-core/src/control_provider.rs:672` (typed request in, typed
  result out). It runs the preflight (target, then the core's checks) and, when `session` is true,
  the `response.rate.session` rule. D1's `serve_response_frame` therefore lives in
  `control_provider.rs`, not in `response.rs`: it decodes the frame, checks the target and then that
  `sample_rate_hz` and `quantum_frames` are present (D1's order), calls
  `serve_response_at(rate, quantum, bound, &request, false)`, and encodes the reply.
- **D1 does not name the reply bound.** Proposal: a host-core constant equal to today's browser bound,
  16 MiB (`RESPONSE_MAXIMUM_RESULT_BYTES = 16 << 20`, which D3 deletes from host-web), narrowed by the
  request's `maximum_reply_bytes` (field 18), as `serve_checked` already does. Then no request that
  the browser serves today is refused for its size.
- **Anchors moved.** `hosts/host-web/src/lib.rs`: `RESPONSE_*` constants `:178-204`,
  `WebResponseRequest` `:535`, `WebResponseParameter` `:569`, `WebResponseResult` `:580`, sizes
  `:607-611`. `hosts/host-web/src/ffi.rs`: `ResponseStaging` `:123`,
  `live_response_staging_retained_bytes` `:400`, the helpers `:838-910`, `response_buffer_budget`
  `:2078`, `response_failure` `:2139`, `run_response_query` `:2150`, the eleven exports `:2339-2432`.
  D3 also deletes `mod response_budget_tests` (`ffi.rs:4531`), which tests only the deleted budget.
  `abi_layout.rs`: export names `:170-180`, field lists `:722`, `:830`, `:847`, structure rows
  `:1653-1669`. `typed_frame.rs`: `encode_success_response_frame_into` `:284`,
  `encode_non_ok_response_frame_into` `:323`, `decode_typed_command` `:374`. The SDK anchors held.
- **Dependencies.** #1572, #1573, #1563 and #1610 are closed. #1562 is open, but the part this slice
  needs is on `main`: host-web links `control-plane`, which builds `host-core` with
  `control-provider`. Host-web's `Cargo.toml` names that feature itself rather than rely on feature
  unification (an authorized path).
- **The browser's rebuild refusal does not apply.** The preview has no session and sends no
  transaction.
- **Edit order.** In `host-core/src/control_provider.rs`: after #1341a1 (batch 9) and #1567; before
  #1615. In `hosts/host-web/src/lib.rs`: after #1602 and #1611 (each changes one line there).
