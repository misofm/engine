# Compute the response preview with one core function

Slice S15 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

The response preview (the frequency response of one native effect or of the input filters, for
given parameters) is computed by one function in `host-core`. The browser calls it today; the
C ABI (#1573) and the browser's protocol path (#1574) call the same function later. The browser's
bytes do not change, except that a request for more than 4096 points is now refused, on every
host.

## Context

- **The core already owns the numbers.** `crates/host-core/src/response.rs` holds the target
  (`ResponsePreviewTarget`, `:24`), the overrides (`ResponseParameterOverride`, `:53`), the limits
  (`ResponsePreviewLimits`, `:64`), the grid (`ResponsePreviewGrid`, `:88`), the request (`:136`),
  the errors (`ResponsePreviewError`, `:166`), the preparation (`prepare_response_preview`, `:502`,
  with `apply_overrides`, `:588`) and the axis (`generate_response_grid`, `:639`).
  `PreparedResponsePreview::query_into` (`:452`) fills caller buffers. Exports at
  `crates/host-core/src/lib.rs:153-159`.
- **Host-web adds the rest.** `run_response_query` (`hosts/host-web/src/ffi.rs:2184-2403`) checks
  the request, turns the override rows into `ResponseParameterOverride`s, builds the target,
  prepares, checks the result budget (`response_buffer_budget`, `:2112-2171`), allocates the
  output vectors, queries, folds the enabled sections into two `u32` masks (`:2365-2372`), takes
  `bypass` from the configuration view (`:2386`) and lays out `WebResponseResult` (`:2373-2402`).
  Helpers: `response_error_code` (`:812`), `response_grid` (`:826`), `response_quality` (`:843`),
  `response_link_mode` (`:847`), `response_channel` (`:851`), `append_f32_values` (`:855`).
  Records: `WebResponseRequest` (`hosts/host-web/src/lib.rs:517`), `WebResponseParameter`
  (`:551`), `WebResponseResult` (`:562`); constants `:180-208`.
- **No points cap.** The only bound is `maximum_result_bytes` up to 16 MiB
  (`RESPONSE_MAXIMUM_RESULT_BYTES`, `hosts/host-web/src/lib.rs:208`). The track response caps at
  4096 (`LIVE_RESPONSE_MAXIMUM_POINTS`, `:218`). The protocol's default frame cap is 1 MiB
  (`docs/CONTROL_BTLV_V1.md:43`).
- **The launch rates** are `engine::LAUNCH_SAMPLE_RATES` (`crates/engine/src/lib.rs:48`). Today a
  non-launch rate is refused only indirectly (the registry's tail bound, `response.rs:547-555`).
- **Only two owners have a response:** the parametric EQ (`crates/parametric-eq/src/lib.rs:3512`)
  and the builtin input filters.

## Decisions frozen for this slice

- **D1. The core API** (in `crates/host-core/src/response.rs`, exported from `lib.rs`):
  - `pub const RESPONSE_MAXIMUM_POINTS: usize = 4096;`
    `pub const RESPONSE_MAXIMUM_OVERRIDES: usize = 256;`
  - `pub enum ResponseChannels { Left = 1, Right = 2, Both = 3 }` with `from_raw(u8|u32)`.
  - `pub struct ResponseQuery<'a> { configuration_id: u64, sample_rate_hz: u32, quantum_frames:
    u32, target: ResponsePreviewTarget<'a>, grid: ResponsePreviewGrid, channels:
    ResponseChannels, sections: bool, limits: ResponsePreviewLimits }` (all fields `pub`).
  - `pub fn prepare_response_query(query: &ResponseQuery<'_>) -> Result<PreparedResponseQuery,
    ResponseQueryError>`.
  - `PreparedResponseQuery` keeps the prepared owner, the grid, the channels and `sections`, and
    has `section_count(&self) -> usize`, `points(&self) -> usize` and
    `curves(&self) -> Result<ResponseCurves, ResponseQueryError>`.
  - `pub struct ResponseCurves { configuration_id: u64, sample_rate_hz: u32, floor_db: f32,
    bypass: bool, section_count: u32, enabled_left: u32, enabled_right: u32, retained_bytes: u64,
    frequencies_hz: Vec<f32>, left_db: Option<Vec<f32>>, right_db: Option<Vec<f32>>,
    sections_left_db: Option<Vec<f32>>, sections_right_db: Option<Vec<f32>> }`. A side's total is
    `Some` exactly when `channels` includes it; a side's section curves (section-major) are `Some`
    exactly when `sections` is true and `channels` includes that side. The masks keep today's fold:
    bit `min(index, 31)` for each enabled section.
  - `pub enum ResponseQueryError { Preview(ResponsePreviewError), Points, Overrides, SampleRate }`.
- **D2. Order of checks.** `prepare_response_query` refuses, before it prepares an owner or
  allocates a curve: a sample rate that is not in `LAUNCH_SAMPLE_RATES` (`SampleRate`); points
  outside `2..=RESPONSE_MAXIMUM_POINTS` (`Points`); more than `RESPONSE_MAXIMUM_OVERRIDES` overrides
  (`Overrides`). Then it calls `prepare_response_preview` unchanged. `curves()` allocates the
  vectors and calls `query_into`; an error leaves nothing behind.
- **D3. Host-web keeps only transport.** `run_response_query` keeps the `repr(C)` checks
  (`struct_size`, `abi_version`, `reserved`, the channel and field bits, the effect-ID and
  parameter-count staging bounds), decodes the raw codes into the D1 types, calls
  `prepare_response_query`, then `response_buffer_budget` with `section_count()` (the budget is a
  property of the `WebResponseResult` layout and stays until #1574 deletes the layout), then
  `curves()`, then lays out the same bytes as today. `Points`, `Overrides` and `SampleRate` map to
  `RESULT_INVALID_ARGUMENT`; `Preview(e)` maps through `response_error_code` as today.
- **D4. No other behaviour change.** Every request that the browser accepts today with 4096 points
  or fewer gives the same result bytes. The override lane rule is unchanged in this slice (#1573
  changes it).

## Deliverables

1. D1 and D2 in `crates/host-core/src/response.rs` and the exports in `crates/host-core/src/lib.rs`.
2. D3 in `hosts/host-web/src/ffi.rs`.
3. The tests below.

## Authorized paths

- `crates/host-core/src/{response.rs,lib.rs}`, new `crates/host-core/tests/response_query.rs`.
- `hosts/host-web/src/ffi.rs` (the response-query functions only, `:812-879` and `:2112-2403`).

## Non-goals

- No wire change, no new export, no SDK change.
- No change to the track response (`run_live_response_*`) or to `query_response_snapshot_into`.
- The `WebResponse*` records and the result budget stay (#1574 deletes them).

## Objective gates

Commands: `cargo test --locked -p host-core`, `cargo test --locked -p host-web`,
`bash scripts/check-web-audioworklet.sh`,
`bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`.

1. **Assembly.** In `crates/host-core/tests/response_query.rs`, for a parametric EQ with
   different left and right band gains and one disabled band, and for input filters with
   different left and right cutoffs, at each launch rate: with `channels` `Left`, `Right` and
   `Both` and `sections` false and true, `curves()` has exactly the D1 `Some`/`None` pattern, the
   vector lengths `points` and `section_count * points`, the masks that match the disabled band,
   and the totals bit-equal to `PreparedResponsePreview::query_into` on the same request.
2. **Bounds.** 4096 points pass and 4097 are refused `Points`; 256 overrides pass and 257 are
   refused `Overrides`; 192000 Hz and 22050 Hz are refused `SampleRate`. In each refused case no
   owner is prepared: the thread-scoped `bench_support::alloc` counters show zero allocations
   during the refused call (warm statics first).
3. **Browser parity.** The existing host-web response tests (`response_budget_tests`,
   `hosts/host-web/src/ffi.rs:4852`) and the SDK's `sdk/test/response-evals.mjs` pass unchanged.
   One new host-web test: a 4097-point request returns `RESULT_INVALID_ARGUMENT` and a header-only
   result.

## Test value

- Gate 1 turns red if the core drops a side, swaps left and right, or folds the enabled sections
  into the wrong bits, so the C ABI and the browser would show different curves later.
- Gate 2 turns red if a bound is checked after the owner is prepared, or not at all.
- Gate 3 turns red if moving the logic changes the browser's result bytes.

## Dependencies

- None among the decision-17 slices. `hosts/host-web/src/ffi.rs` is stream H's file
  (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`, the stream H "Owns" line): this slice
  rebases after stream H's open slices that touch `ffi.rs`.

## Supersedes and shrinks

- None.
