# Configure meters by stable target and receive windowed output peaks on the C ABI

Slice S24a of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1,
C9 and C13). Split from the planner's S24: this slice is the wire, the batch rule and the output
meter; #1587 adds the strip meters and gain reduction on the C ABI, and #1588 serves the same
batches in the browser. Code anchors verified on `main` `df585e503`. Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or mutation
runs.

## Product outcome

A C ABI app configures its meters with `TELEMETRY_CONFIGURE` as a table of stable targets: the
output, or a strip by its ID, each with a metric. The 1-based position of an entry in the table
is its meter handle. Each `METER_BATCH` then reports one closed meter window: its exact sample
span, its sequence number, and the output's own left and right peaks. A handle never moves when
a track is added or removed. Today every handle reports the same block peak labelled as the left
channel.

## Context

- **The wire today.** `TelemetryConfiguration` (`crates/protocol/src/message_wire.rs:648`) has
  `meter_handles: Vec<u32>` and `meter_period_blocks`; its schema
  (`crates/protocol/src/schema.rs:521-540`) has field 1 `meter_handles` `PackedU32` and field 2
  `meter_period_blocks` `U32`; `validate_telemetry_configuration` (`message_wire.rs:2088`) wants at
  most 256 nonzero, strictly increasing handles and a nonzero period when any handle is set.
  `MeterRecord` (`:570`) is `{handle, component, flags, value}`; `MeterBatch` (`:583`, schema
  `schema.rs:509-519`) is `{observed_sample, count, record_bytes, records}`. The registry rows are
  `docs/CONTROL_PROTOCOL_REGISTRY.md:13` (components and flags), `:33` (`0009`), `:37` (the meter
  record) and `:43` (`8020`). `docs/CONTROL_PROTOCOL_SEMANTICS.md:29` says meter handles are
  revision-scoped.
- **Configuration is never refused.** `ControlProvider::telemetry_configure`
  (`crates/protocol/src/controller.rs:600`) returns the configuration; the controller stores what it
  echoes (`:3059`). *Report each configured counter's own value in the C ABI counter snapshot*
  (#1351 D2) changes it to return `Result<TelemetryConfiguration, ParameterProviderError>`.
- **Staging and batching.** `stage_meter_batch_event` (`controller.rs:2434`) stages one
  `TelemetryRecord` (`crates/protocol/src/queue.rs:495`) per record into the lossy queue, where a
  record of the same key replaces an older one (`stage_telemetry`, `queue.rs:954`).
  `prepare_next_lossy_event` (`controller.rs:2731`) groups consecutive records of one revision and
  one `observed_sample` into a batch.
- **The defect.** `collect_render_activity` (`crates/control-plane/src/control.rs:661`, called by
  `service` `:1059`) stages one `{handle, Left, flags 1, peak}` per configured handle once per
  service step that sees a new `render_sequence` (not once per block), from the one combined peak that
  `observe_rendered_block` (#1570 D3) publishes through
  `render_peak_bits` (`crates/control-plane/src/plan.rs:14`), gated by `render_peak_observed`
  (`:29`, refreshed by `refresh_render_peak_gate`, `control.rs:653`). `meter_period_blocks` is
  only checked for being nonzero.
- **The core pieces exist.** #1581's output meter (`OutputPeakReader`, `OutputPeakGate`, kept by
  the C ABI's `SessionState`) and #1582's `MeterDelivery`. The C ABI calls `service` at least once
  per render-buffer period (`crates/capi/include/miso_engine_v1.h:117`), and each meter queue holds
  8 windows.

## Decisions frozen for this slice

- **D1. The meter table (the frozen handle rule).** `TelemetryConfiguration::meter_handles` is
  replaced by `meters: Vec<MeterEntry>`, `MeterEntry { target: MeterTarget, metric: MeterMetric }`,
  `MeterTarget::{Output, Strip(Box<str>)}`, `MeterMetric::{Peak = 1, GainReduction = 2}`. The
  handle of an entry is its 1-based index in the table, in the order the client sent. A strip
  entry names the strip by its stable ID, so its handle does not move when the structure changes.
  An entry whose strip is not in the adopted plan stays in the table; its records carry no valid
  bit (D5) until a commit brings that ID back.
  - *Why a table and not handles valid for one `structure_epoch`:* an epoch handle dies at every
    structural commit, so the app must reconfigure after each one, and a batch staged before the
    commit and dequeued after it names a handle of a dead epoch. A table entry is a stable ID that
    the app chose; nothing can relabel it. It is the same shape as an automation batch's address
    header table (ruling C10).
- **D2. Wire.**
  - Schema `telemetry_configuration`: field 1 is retired and never reused (a frame that carries it
    is refused as an unlisted field); new field 7 `meter` `Message` repeated (0 to 256, schema
    `meter_entry`: 1 `target` `U8` (output 1, strip 2), 2 `strip_id` `Utf8` optional (present
    exactly when `target` is strip; at most the session grammar's 127 bytes), 3 `metric` `U8`
    (peak 1, gain reduction 2)); new field 8 `meter_window_blocks` `U32` optional, refused in a command
    and present in a success echo exactly when the provider has a meter delivery (D4).
  - The codec refuses with `INVALID_FIELD`: an unknown code, a duplicate entry (same target, strip
    and metric), `{output, gain reduction}`, more than 256 entries, a nonempty table with
    `meter_period_blocks == 0`.
  - Schema `meter_batch`: new field 5 `first_sample` `U64` and field 6 `window_sequence` `U64`,
    both required. `observed_sample` (field 1) is the window's exclusive end. The codec refuses
    `first_sample >= observed_sample` and `window_sequence == 0` on encode and decode.
  - `MeterBatch` gains `first_sample: SampleTime` and `window_sequence: u64`.
    `TelemetryRecord` gains the same two fields. `stage_meter_batch_event(revision, window:
    MeterWindowSpan { first_sample, end_sample, window_sequence }, records)` takes them, and
    `prepare_next_lossy_event` groups records by revision, end sample and window sequence.
    A record's handle must be at most the table length (else `EncodeError::LimitExceeded`, as
    today for an unconfigured handle).
- **D3. Refusals.** `TELEMETRY_CONFIGURE` is all or nothing (#1351 D2); a refusal keeps the stored
  configuration. In order:
  - the controller refuses a strip entry whose ID is not a strip of the committed model:
    `NOT_FOUND`, diagnostic `meter.strip.unknown`, path `meters[i]`;
  - the provider refuses through #1351's `Result`. New `ParameterProviderError::InvalidField`
    (answered `INVALID_FIELD`) and `Unavailable` (exists, answered `UNAVAILABLE`):
    `meter.period` (`INVALID_FIELD`) when `meter_period_blocks` is not a positive multiple of the
    plan's meter window; `meter.gain_reduction.unavailable` (`UNAVAILABLE`) for a gain-reduction
    entry, until #1587; `meter.strip.unavailable` (`UNAVAILABLE`) for a strip peak entry, until
    #1587; `meter.unavailable` (`UNAVAILABLE`) for any entry on a provider with no meter delivery
    (the browser, until #1588). Each refusal names the first refused entry's path.
- **D4. The echo.** The success echo carries the accepted table, the period and, when the provider
  has a meter delivery, `meter_window_blocks`: the window of the plan the configuration applies to
  (#1581 D2). A configure with an empty table always succeeds, so counters and diagnostics are
  configured on every host, and a client learns the window from its echo. `ControlProvider` gains
  `fn meter_window_blocks(&self) -> Option<NonZeroU32>` (`None`: no meter delivery). The test
  `MockProvider` answers `Some(1)`.
- **D5. Batches from the core fold.**
  - The C ABI's `SessionState` owns one `MeterDelivery` (#1582) built at boot from the output
    meter with no strip consumers. A nonempty table sets its level on at the configure; an empty
    one sets it off. The output gate follows the level, so the output is measured only while some
    client has a table.
  - `MeterDelivery::poll` publishes an output-only frame when it holds no strip consumer. Today's
    fold returns at once in that case (`hosts/host-web/src/lib.rs:3599-3602`); the core fold
    groups the output window alone instead. The only browser change is a session with no strip at
    all: it now publishes its output pair (silence) where it published nothing.
  - `collect_render_activity` polls it. A `MeterBatchFold` (new, `crates/host-core/src/meters.rs`,
    shared with #1587 and #1588) folds `k = meter_period_blocks / meter_window_blocks` consecutive
    frames: the peaks are the maxima, the span is the first frame's start to the last frame's
    end, and a gap between two frames (a frame whose start is not the previous end, or a new
    generation) discards the partial fold and adds its windows to the lost count.
  - For each completed fold it stages one batch, `window_sequence` counting folds from 1 after each
    accepted configure. Records follow the table order. An output peak entry gives `{h, Left}` and
    `{h, Right}`; `flags` has `VALID`, and `CLIPPED` when the value is above `1.0`; `HELD` is never
    set.
  - Lost windows (the delivery's loss count and the fold's discards) add to a new counter,
    `CounterId::MeterWindowsLost`, at the next free ID when this merges (tests use the name). The
    provider serves it from construction, at 0 (#1351 D1).
- **D6. The legacy peak goes.** `render_peak_bits`, `render_peak_observed`,
  `refresh_render_peak_gate`, the peak argument of `publish_render_observation` and the scan in
  `observe_rendered_block` are deleted. The render sequence and sample stay (counters use them).
- **D7. Docs.** `docs/CONTROL_PROTOCOL_REGISTRY.md` (`0009` row, `8020` row, meter enums),
  `docs/CONTROL_PROTOCOL_SEMANTICS.md:29` ("meter handles are configuration-scoped: the 1-based
  index of a `TELEMETRY_CONFIGURE` entry"), `docs/C_ABI_V1_QUALIFICATION.md` and the header prose
  (`crates/capi/include/miso_engine_v1.h`) describe D1-D5. No C symbol or struct changes.
- **D8. The acked-batch question.** A configure is checked whole before it is stored. Batches are
  lossy telemetry: every window that never reaches a batch is counted (D5), and a coalesced
  record is counted by `TELEMETRY_COALESCED` as today.

## Deliverables

1. D1-D2 and D4's trait method in `crates/protocol`; D3 in the controller and #1351's provider
   `Result`; the conformance corpus and `COMPLETE_SCHEMA_HASH`.
2. `MeterBatchFold` in `crates/host-core/src/meters.rs`; the provider's `meter_window_blocks` and
   the served counter in `crates/host-core/src/control_provider.rs`.
3. D5 and D6 in `crates/control-plane/src/{control.rs,plan.rs}`, `crates/capi/src/ffi.rs` (the call
   only).
4. D7.
5. `hosts/host-web/tests/protocol_cross_host.rs` (#1570's parity script): step 5 configures
   counters and diagnostics with an empty meter table; #1588 puts a meter entry back.

## Authorized paths

- `crates/protocol/src/{message_wire.rs,message_wire/tests.rs,schema.rs,typed_frame.rs,controller.rs,controller/tests.rs,queue.rs,lib.rs}`.
- `crates/conformance/src/protocol_corpus.rs`, `crates/conformance/tests/conformance_corpus.rs`,
  `fuzz/corpus/complete-schema-manifest.md`.
- `crates/host-core/src/{meters.rs,control_provider.rs}`, `crates/host-core/tests/meters.rs`.
- `crates/control-plane/src/{control.rs,plan.rs,compile.rs}`.
- `crates/capi/src/ffi.rs`, `crates/capi/src/runtime/tests.rs`, `crates/capi/include/miso_engine_v1.h`
  (prose), `tools/audit/src/{capi.rs,protocol.rs}`.
- `hosts/host-web/tests/protocol_cross_host.rs` (step 5 only).
- `docs/CONTROL_PROTOCOL_{REGISTRY,SEMANTICS}.md`, `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- No strip meter or gain reduction on the C ABI (#1587); no browser batch (#1588).
- No aggregate component and no held peak (the registry keeps their codes).
- No change to counter snapshots beyond the new counter.

## Objective gates

Commands: `cargo test --locked -p protocol --all-features`, `cargo test --locked -p conformance`,
`cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo test --locked -p host-web --test protocol_cross_host`,
`cargo build --locked --release -p audit -p capi && target/release/audit capi`.

1. **Wire.** Round trips of a table with each target and metric and of a batch with its span;
   every D2 refusal on encode and on decode alike; field 1 refused; field 8 refused in a command
   and accepted in an echo.
2. **Refusals change nothing.** Through the C ABI: an unknown strip (`NOT_FOUND`,
   `meter.strip.unknown`, `meters[1]`), a period of 5 on a 12-block window (`meter.period`), a
   gain-reduction entry and a strip peak entry (`UNAVAILABLE`). After each, an empty-table configure
   echoes the previous configuration, and the event lane holds no new batch.
3. **Windowed output peaks.** Configure `[{output, peak}]` with period `2 × window`. Render a
   script whose left plane peaks at `0.5` and right at `0.25` in the first window and `0.75` /
   `0.125` in the second, calling `service` after each block. The first batch has `first_sample`
   `0`, `observed_sample` `2 × window × quantum`, `window_sequence` `1`, `{1, Left, VALID, 0.75}`
   and `{1, Right, VALID, 0.25}`. A sample of `1.5` sets `CLIPPED`.
4. **Handles survive structure.** Configure `[{output, peak}]`, then commit `UpsertTrack`; the
   next batch still uses handle 1 with the same span rule and the next sequence.
5. **Loss is counted.** Render 9 windows without a `service` call: the delivery loses at least
   one window, `MeterWindowsLost` rises by exactly the number of windows with no batch, and the
   next batch's `window_sequence` is the previous one plus one.
6. **Cross-host script.** `protocol_cross_host` stays byte-identical on both legs with step 5 as
   changed.
7. **Realtime.** The capi race test (`crates/capi/tests/resource_lifecycle.rs:3364`) runs with a
   meter table configured: `allocations == 0 && frees == 0` around every render call after
   warm-up.

## Test value

- Gate 1 turns red if the table, the span or the retired field disagree between encode, decode and
  the registry.
- Gate 2 turns red if a bad configure is half stored or a refusal is untyped.
- Gate 3 turns red on today's defect (one combined block peak per handle, labelled left) or if a
  fold of two windows loses a peak.
- Gate 4 turns red if a handle is positional over the structure.
- Gate 5 turns red if a lost window is not counted or the sequence hides it.
- Gate 6 turns red if the browser and the C ABI stop agreeing on the other protocol bytes.
- Gate 7 turns red if the output meter or the gate allocates on render.

## Dependencies

- #1581, #1582, #1570.
- *Report each configured counter's own value in the C ABI counter snapshot* (#1351): the
  `Result` of `telemetry_configure` and the served-counter set.
- `crates/capi` and `crates/control-plane` are stream B's files (`STREAMS.md` row 92); this slice
  rebases after the open B slices there.

## Supersedes and shrinks

- Supersedes, with #1581 and #1587, *Report each configured meter handle's own meter in the C ABI
  meter batch* (#1352); see the amendment line.
