# Serve a response without a session

Successor of #1573, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on `5415352a9` (PR #1608).
Issue number: assigned at filing (written here as #1610). Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

`host-core` has one response-serving function that needs no session: it takes the endpoint's
sample rate, quantum and reply bound and a decoded `RESPONSE_GET` request. The session provider
calls it through a thin wrapper, and #1574's session-less preview calls it directly, so the
browser preview and the C ABI run the same code. A C ABI client sees the same replies, refusal
codes and bytes as today.

## Context

- **The serving function today.** `serve_response(maximum_reply_bytes, model: &SessionModel,
  request)` (`crates/host-core/src/control_provider.rs:606-776`) reads only two fields of the
  model: `model.sample_rate_hz` (the core check at `:637` and the query at `:729`) and
  `model.quantum_frames` (`:730`). Its one caller is `SessionControlProvider::response_get`
  (`:829-835`), which the controller calls with the committed model
  (`crates/protocol/src/controller.rs:3197-3207`).
- **The refusal order.** The function refuses a non-configuration target first
  (`response.target.unavailable`, `:615-620`), then runs the core's checks in the core's order
  (rate, points, override count, axis: `check_response_request`, `:637-638`), then the session
  rule (`response.rate.session` when the request names a rate or a quantum, `:640-642`), then the
  limits, the bypass rows, preparation, the reply bound (`response.reply_too_large`, `:738-755`)
  and the curves. `response_get_asks_the_core_first_and_names_a_bad_bypass`
  (`control_provider.rs:2143`) pins that the axis precedes the session rule.
- **The C ABI tests.** `crates/capi/tests/response_get.rs` sends every reachable refusal
  (`every_reachable_refusal_is_typed_and_changes_nothing`, `:597`), the curves of the core
  (`a_configuration_request_answers_the_curves_of_the_core`, `:376`;
  `a_one_lane_section_and_another_session_rate_reach_the_reply`, `:502`) and the reply bound
  (`an_oversized_reply_is_refused_before_any_curve_is_computed`, `:804`).
- **The consumer.** #1574 D1 needs "the same private function as #1573's provider" for its
  session-less `serve_response_frame`, with the rate and quantum from the request. Today that
  function needs `&SessionModel`, which a preview instance does not have.

## Decisions frozen for this slice

- **D1. Three functions.** In `crates/host-core/src/control_provider.rs`:
  - `response_preflight(sample_rate_hz: u32, request) -> Result<ResponsePreviewGrid,
    ResponseProviderError>`: the target refusal and `check_response_request`, in today's order;
    it returns the grid.
  - `serve_checked(sample_rate_hz, quantum_frames, maximum_reply_bytes, request, grid)`
    (private): everything after the session rule, unchanged in order and code.
  - `pub(crate) fn serve_response_at(sample_rate_hz: u32, quantum_frames: u32,
    maximum_reply_bytes: usize, request: &ResponseGetRequest) -> Result<ResponseResult,
    ResponseProviderError>`: `response_preflight` and then `serve_checked`. This is the core
    function #1574 calls.
- **D2. The session wrapper.** `serve_response` keeps its name and signature: it calls
  `response_preflight(model.sample_rate_hz, request)`, then the session rule
  (`response.rate.session`), then `serve_checked(model.sample_rate_hz, model.quantum_frames, ...)`.
  So the session path runs the same checks in the same order as today, with no check twice.
- **D3. The core reads no rate from the request.** `serve_response_at` and `serve_checked` read
  neither `request.sample_rate_hz` nor `request.quantum_frames`: each caller resolves them. The
  session wrapper refuses them; #1574's caller requires them.
- **D4. Same bytes.** No refusal code, status, order, counter (`RESPONSE_PREPARATIONS`,
  `RESPONSE_EVALUATIONS`) or reply byte changes on the C ABI.
- **D5. No new surface.** No export, wire field, feature or public item. `serve_response_at` is
  `pub(crate)` and lives under the `control-provider` feature, as its module does.

## Deliverables

1. D1-D3 in `crates/host-core/src/control_provider.rs`.
2. The host-core tests below.
3. The byte comparison in this spec's decision record.

## Authorized paths

- `crates/host-core/src/control_provider.rs`.
- This spec's decision record.

## Non-goals

- No session-less frame entry, export or SDK change (#1574).
- No change to the response core in `crates/host-core/src/response.rs`, to the wire or to the
  controller.

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`, `cargo test --locked -p capi --test
response_get`, `cargo test --locked -p protocol`.

1. **Same C ABI bytes (PR evidence).** Before the change (on `5415352a9` or the branch's parent)
   and after it, a scratch test sends every request of `crates/capi/tests/response_get.rs` (each
   refusal of `:597`, the curve requests of `:376` and `:502`, the oversized reply of `:804`)
   through the C ABI and concatenates the response frames. The SHA-256 digests are equal. The
   harness is not committed (AGENTS.md: a one-time "no bit moved" comparison is PR evidence).
   Record the digests and the base commit in the decision record.
2. **The order is kept.** `response_get_asks_the_core_first_and_names_a_bad_bypass` and every
   test of `response_get.rs` stay green unchanged.
3. **The core serves without a session.** A new host-core test calls `serve_response_at` at
   44,100 Hz and 128 frames with an EQ configuration request whose `sample_rate_hz` field names
   48,000, and asserts the curves are bit-equal to the core's `prepare_response_query` at 44,100 Hz
   and 128 frames, and that `serve_response` on a 44,100 Hz session gives the same curves for the
   request without the field.
4. **The core refuses as the session does.** The same test sends a non-configuration target, a
   point count of 1 and a NaN axis to `serve_response_at` and gets
   `response.target.unavailable`, `response.points` and `response.grid`, the session's codes.

## Test value

- Gate 1 turns red if the split moves a refusal or a reply byte on the C ABI.
- Gate 3 turns red if the core reads the request's rate field, or a rate from anywhere other
  than its argument.
- Gate 4 turns red if the core skips the target or core checks that only the session wrapper
  ran before.

## Dependencies

- #1573 (merged).
- Dependency of #1574: #1574 calls `serve_response_at`.

## Size

Half a day, one sonnet-high implementer.

## Amendment (controls coordinator, batch 8, 2026-10-14)

Checked against `main` at `32cee58d0`.

- **Anchors.** Batch 7 changed `control_provider.rs` only in its catalog-staging code
  (`ModelView`, `verify_stages`). `serve_response` did not change, but its lines moved:
  `serve_response` is `crates/host-core/src/control_provider.rs:619-784`; the target refusal
  `:624-629`; `check_response_request` `:646-647`; the session rule `:649-651`; the query's rate
  and quantum `:738-739`; the reply bound `:748-765`; `SessionControlProvider::response_get`
  `:838-844`; `response_get_asks_the_core_first_and_names_a_bad_bypass` `:2207`. The controller
  calls it at `crates/protocol/src/controller.rs:3291-3300`. The `crates/capi/tests/response_get.rs`
  anchors did not move.
- **Decisions.** D1-D5 hold. The model still gives only `sample_rate_hz` and `quantum_frames`.
- **Scope note.** Since the flip, the browser's session runs the same control plane and provider
  (`control_plane::compile_children`, `ProviderFeatures::ALL` at
  `crates/control-plane/src/compile.rs:974`). The split therefore also serves the browser's session
  `RESPONSE_GET`. D4's "same bytes" holds there too; gate 1's harness stays on the C ABI.
- **Gate 1 base.** "The branch's parent" is now `32cee58d0` or later.

## Decision record: serve a response without a session (batch 8)

Shipped D1-D5 in `crates/host-core/src/control_provider.rs`: `response_preflight` (private; the
target refusal and `check_response_request`, returns the grid), `serve_checked` (private; the
limits, bypass rows, preparation, reply bound and curves, unchanged in order and code),
`serve_response_at` (`pub(crate)`) and `serve_response` as the session wrapper (preflight, then
`response.rate.session`, then `serve_checked` with the model's rate and quantum). The core reads
neither `request.sample_rate_hz` nor `request.quantum_frames`.

- **Departure.** Until #1574 calls it, `serve_response_at` has no non-test caller. It carries
  `#[cfg_attr(not(test), expect(dead_code, ...))]`, so #1574's first use makes the build fail until
  the attribute is removed. `serve_checked` re-extracts the configuration with a `let ... else`
  that cannot be reached after the preflight (the target refusal is repeated there only to avoid a
  panic path); it runs no check the preflight ran.
- **Gate 1 (C ABI bytes).** A scratch copy of `crates/capi/tests/response_get.rs` (not committed,
  deleted) wrote every response frame of its five tests (length-prefixed, one file per test,
  concatenated in file-name order) and hashed them with SHA-256. Base `d402a00d3` (code identical to
  `32cee58d0`): `3d4d51229c10c806fccbbd61b2c246231f18a2d81a545ff1f65b0e6771eac4f4`, twice (the
  harness is deterministic). After the change:
  `3d4d51229c10c806fccbbd61b2c246231f18a2d81a545ff1f65b0e6771eac4f4`. Equal.
- **Gate 2.** `response_get_asks_the_core_first_and_names_a_bad_bypass` and all five tests of
  `crates/capi/tests/response_get.rs` pass unchanged.
- **Gates 3 and 4.** `control_provider::tests::the_core_serves_without_a_session_and_refuses_as_the_session_does`:
  curves at 44,100 Hz and 128 frames bit-equal to `prepare_response_query`, with a request naming
  48,000 Hz and 64 frames; `serve_response` on a 44,100 Hz session gives the same curves; the
  target, point-count and NaN-axis refusals are `response.target.unavailable`, `response.points`
  and `response.grid` from both functions.
- Commands: `cargo test --locked -p host-core --all-features`, `cargo test --locked -p capi --test
  response_get`, `cargo test --locked -p protocol` pass; clippy `-D warnings` on host-core and capi
  (all targets, all features) is clean.
