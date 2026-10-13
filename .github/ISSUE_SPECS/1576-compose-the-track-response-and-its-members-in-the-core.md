# Compose the track response and its members in the core

Slice S19 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

The track response (the combined EQ and filter curve of one strip, from the values its render plan
holds, with the list of the owners that make it) is composed by one function in `host-core`, and
the render-plan owner can capture it at a block boundary. The browser composes its curve through
this function, with unchanged bytes. #1577 and #1578 give the C ABI the same result; #1579 moves
the browser onto it.

## Context

- **What the core has.** `ResponseSnapshot` (`crates/host-core/src/response.rs:214`) with owners
  (`ResponseSnapshotOwner`, `:189`: `track_id`, `native_id`, `stable_id`, engine `rack`, `slot`,
  `kind`, `bypassed`, `availability`, `left` and `right` sections); the allocation-free collector
  `ResponseSnapshotCollector` (`:227-358`, used only by `crates/host-core/tests/randomized.rs:1173`);
  the composer `query_response_snapshot_into` (`:688-814`), which sums `log10` magnitudes in `f64`
  and applies the -120 dB floor once.
- **Semantics are applied (#779's contract).** The engine copies each owner's retained target
  words at a block boundary: `PreparedRenderPlan::copy_response_snapshot`
  (`crates/engine/src/realtime/plan.rs:845`), the graph's bindings (`crates/graph/src/runtime.rs:2885`,
  built by `response_owner_bindings`, `:6960`: the input-filter stage with rack `0` and every
  effect node with its rack code), the EQ's words (`crates/parametric-eq/src/lib.rs:3906`).
- **What the browser adds.** Host-web composes through `run_live_response_analysis`
  (`hosts/host-web/src/ffi.rs:2010-2110`) after it re-parses its own capture record
  (`parse_live_snapshot`, `:1872-1983`). Its record keeps rack codes `0`, `2` and `4`
  (`live_response_rack_raw`, `:761`; `live_response_rack`, `:771`), so the engine rack is lost
  (comment `:1956-1958`). The owner list (members) is derived only in the SDK
  (`TrackResponseModule.#parseCapture`, `sdk/src/core/live-response.ts:783`).
- **No owner-level capture.** `RealtimePlanOwner` (`crates/engine/src/realtime/plan_exchange.rs:156`,
  methods from `:682`) has no capture method; `RealtimeResponseSnapshot` (`:182`) is declared and
  exported but unused.

## Decisions frozen for this slice

- **D1. Result types** (`crates/host-core/src/response.rs`, exported):
  - `pub enum ResponseMemberSection { Builtins, Console(Box<str>), Inserts(Box<str>) }` (the slot
    ID or the insert's effect ID; wire codes 4, 5 and 2, the decision-17 section codes);
  - `pub enum ResponseMemberKind { ParametricEq = 1, InputFilters = 2, Unavailable = 3 }`;
  - `pub struct ResponseMember { section, native_id: Box<str>, kind, bypassed: bool,
    section_count: u8, enabled_left: u32, enabled_right: u32 }` (`section_count` is the owner's
    left section count; bit `i` is set when section `i` of that side is enabled, `i < 32`);
  - `pub struct TrackResponse { strip_id: Box<str>, captured_sample: u64, sample_rate_hz: u32,
    floor_db: f32, frequencies_hz: Vec<f32>, left_db: Option<Vec<f32>>, right_db:
    Option<Vec<f32>>, excluded_count: u32, members: Vec<ResponseMember> }`, members in declared
    signal order, `floor_db` `-120.0`.
- **D2. The composer.** `pub fn compose_track_response(strip_id: &str, snapshot:
  &ResponseSnapshot, grid: ResponsePreviewGrid, channels: ResponseChannels) ->
  Result<TrackResponse, ResponseSnapshotQueryError>` (`ResponseChannels` from #1572). It refuses
  points outside `2..=RESPONSE_MAXIMUM_POINTS` (`InvalidGrid`), an owner whose `track_id` is not
  `strip_id` and an engine rack other than `0`, `1`, `2`, `3` (`MalformedSnapshot`), then calls
  `query_response_snapshot_into` unchanged. Rack `0` is `Builtins`; `2` is
  `Inserts(stable_id)`; `1` and `3` are `Console(stable_id)`. A `DeclaredUnavailable` owner is
  `Unavailable`; else owner kind `1` is `ParametricEq` and `2` is `InputFilters`.
- **D3. The owner-level capture.** `RealtimePlanOwner::copy_response_snapshot(&mut self, track_id:
  &str, sink: &mut dyn ResponseSnapshotSink) -> Result<RealtimeResponseSnapshot,
  ResponseSnapshotError>` delegates to the active plan and fills `RealtimeResponseSnapshot {
  active_epoch, captured_sample, owners }`. It sits in the existing `REALTIME_POLICY_BEGIN` region
  of `impl RealtimePlanOwner` (`plan_exchange.rs:681`) and allocates nothing.
- **D4. The browser composes through D2.** `live_response_rack` returns the engine rack (record
  `4` maps to `1`; `0` and `2` are unchanged); `run_live_response_analysis` calls
  `compose_track_response` and writes the same header and the same three vectors as today. The
  capture record and the SDK do not change.

## Deliverables

1. D1 and D2 in `crates/host-core/src/response.rs` and `lib.rs`.
2. D3 in `crates/engine/src/realtime/plan_exchange.rs`, with its test in the realtime test module.
3. D4 in `hosts/host-web/src/ffi.rs`.

## Authorized paths

- `crates/host-core/src/{response.rs,lib.rs}`, new `crates/host-core/tests/track_response.rs`.
- `crates/engine/src/realtime/{plan_exchange.rs,mod.rs}`.
- `hosts/host-web/src/ffi.rs` (`live_response_rack` and `run_live_response_analysis` only).

## Non-goals

- No mailbox, no render-thread step and no wire (#1577, #1578).
- No change to the browser's capture record, exports or SDK (#1579, #1580).

## Objective gates

Commands: `cargo test --locked -p host-core`, `cargo test --locked -p engine`,
`cargo test --locked -p host-web`, `cargo build --locked --release -p realtime-policy &&
./target/release/realtime_policy`,
`bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`.

1. **Members and curves.** In `crates/host-core/tests/track_response.rs`, prepare a host
   (`prepare_host_runtime`) for a 48 kHz track with asymmetric HPF and LPF, a parametric EQ
   pre-insert console slot, a parametric EQ insert with one disabled band and bypass on, a
   compressor insert and a parametric EQ post-insert console slot. Capture with
   `PreparedHost::copy_response_snapshot` and `ResponseSnapshotCollector`. `compose_track_response`
   returns five members in signal order with sections `Builtins`, `Console(pre)`,
   `Inserts(eq)`, `Inserts(compressor)`, `Console(post)`, kinds `InputFilters`, `ParametricEq`,
   `ParametricEq`, `Unavailable`, `ParametricEq`, the bypass flag, the mask without the disabled
   band, `excluded_count` 1, and totals bit-equal to `query_response_snapshot_into` on the same
   snapshot. With `channels` `Left`, `right_db` is `None`.
2. **Refusals.** 4097 points (`InvalidGrid`); a snapshot owner of another strip and an owner with
   rack `5` (`MalformedSnapshot`).
3. **Owner capture.** In the realtime tests: before the first render and after a successful
   render, the owner's capture copies the same owners as the plan's own
   `copy_response_snapshot`, and `active_epoch` names the active plan; after a published successor
   is adopted it names the successor; after a failed render it refuses with `Owner`.
4. **Browser parity.** `live_response_ffi_tests` (`hosts/host-web/src/ffi.rs:4882`) and
   `sdk/test/live-response-evals.mjs` pass unchanged.

## Test value

- Gate 1 turns red if a member gets the wrong section, kind or mask, or the order is not the
  signal order, so a mobile app and the browser would label the curve differently.
- Gate 2 turns red if the composer accepts a snapshot it cannot interpret.
- Gate 3 turns red if a capture reads another plan than the active one.
- Gate 4 turns red if routing the browser through the composer changes its bytes.

## Dependencies

- #1572 (`ResponseChannels`, `RESPONSE_MAXIMUM_POINTS`).
- `crates/engine/src/realtime/plan_exchange.rs` follows the edit order of
  `docs/handoffs/decision-15-2026-10-05/STREAMS.md` row 84 (B, C, H #1381, B #1349): rebase after
  those. `hosts/host-web/src/ffi.rs` is stream H's.

## Supersedes and shrinks

- None.
