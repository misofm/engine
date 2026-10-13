# Read spectrum windows and streams on the C ABI

Slice S31 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

Split from the planner's S31: this slice serves the reads on the C ABI; #1596 sends the same
frames from the browser and the SDK; #1597 deletes the browser's own spectrum exports.

## Product outcome

A mobile app reads the same spectrum as the browser: one 2048-frame window analyzed into 1025 dB
bins, or the smoothed continuous stream with its statuses and metadata. It sends
`SPECTRUM_CAPTURE` (arm, read, cancel, select) and `SPECTRUM_STREAM` (start, select, read,
reset, stop) through `miso_engine_v1_submit_command`, and the reply carries the analyzed bins. The
analysis runs on the control thread, never in render. A read that cannot be delivered consumes
nothing.

## Context

- **The C ABI entry.** `miso_engine_v1_submit_command` (`crates/capi/src/ffi.rs:685`) runs
  `SessionState::command` (`crates/control-plane/src/control.rs:1082`) and maps
  `CommandError::BufferTooSmall { required }` to `RESULT_BUFFER_TOO_SMALL` with
  `required_bytes` (`ffi.rs:724-732`). The session's frame limit is `maximum_control_frame_bytes`
  (`crates/control-plane/src/compile.rs:787`); the C ABI tests set 4,096 bytes
  (`crates/capi/src/runtime/tests.rs:41`).
- **Sizes.** A ready stereo one-shot reply carries 3 × 1025 `f32` (about 12.3 KB); with both raw
  planes another 2 × 2048 `f32` (16 KB). The codec measures a payload before encoding
  (`measure_payload`, `crates/protocol/src/typed_frame.rs:582`; the frame limit check `:591`).
- **The status registry.** `StatusCode` (`crates/protocol/src/wire.rs:179`): `INVALID_FIELD` 5,
  `LIMIT_EXCEEDED` 6, `BACKPRESSURE` 11, `VALIDATION_FAILED` 12, `NOT_FOUND` 13, `UNAVAILABLE` 14.
- **The browser's reads today.** One-shot: `miso_engine_web_v1_spectrum_read`
  (`hosts/host-web/src/ffi.rs:3010`), then the analysis instance's `spectrum_analysis`
  (`:2773`). Stream: `spectrum_stream_read` (`:3136`), then `spectrum_stream_analysis` (`:3289`).
  Records: `WebSpectrumResult` (`hosts/host-web/src/lib.rs:332`), `WebSpectrumStreamMetadata`
  (`:362`).
- **#1591, #1592, #1593, #1594**: the service, the analysis, the monitor, `adopted_epoch`, the
  `PreparedCommandFrame::Session` dispatch.

## Decisions frozen for this slice

- **D1. IDs.** `MessageId::SpectrumCapture = 0x0011` and `MessageId::SpectrumStream = 0x0012`,
  command and response. Contract identity stays V1.
- **D2. `SPECTRUM_CAPTURE`.**
  - Request (`spectrum_capture_request`): 1 `op` `U8` (arm 1, read 2, cancel 3, select 4);
    2 `channels` `U8` (read and select only; left 1, right 2, both 3); 3 `target_kind` `U8` and
    4 `target_id` `Utf8` (select only; #1594 D2's codes); 5 `include_window` `Bool` optional (read
    only; absent is false). Any other presence is `INVALID_FIELD`.
  - Reply (`spectrum_capture_reply`): 1 `adopted_epoch` `U64`; 2 `selection_epoch` `U64`;
    3 `status` `U8` (done 1: arm, cancel or select succeeded; pending 2: no window yet; ready 3).
    A ready reply adds: 4 `target_kind` `U8`, 5 `target_id` `Utf8`, 6 `channels` `U8`,
    7 `sample_rate_hz` `U32`, 8 `window_frames` `U32` (2048), 9 `bin_count` `U32` (1025),
    10 `floor_db` `F32` (-120), 11 `captured_sample` `U64`, 12 `end_sample` `U64`,
    13 `snapshot_token` `U64`, 14 `source_underrun` `Bool`, 15 `frequencies_hz` `PackedF32`
    (1025), 16 `left_db` and 17 `right_db` `PackedF32` (each present exactly when `channels` holds
    that plane), 18 `left_window` and 19 `right_window` `PackedF32` (2048; present exactly with
    `include_window` and the plane).
- **D3. `SPECTRUM_STREAM`.**
  - Request (`spectrum_stream_request`): 1 `op` `U8` (start 1, select 2, read 3, reset 4, stop 5);
    2 `smoothing_ms` `F64` (start and select only; finite, 0 to 10,000); 3 `target_kind`,
    4 `target_id`, 5 `channels` (select only).
  - Reply (`spectrum_stream_reply`): 1 `adopted_epoch` `U64`; 2 `selection_epoch` `U64`; then the
    fields of #1592's `SpectrumStreamMetadata`: 3 `status` `U8` (#1592 D1 codes), 4 `target_kind`
    `U8` (0 without a selection), 5 `target_id` `Utf8` optional, 6 `channels` `U8`,
    7 `sample_rate_hz` `U32`, 8 `quantum_frames` `U32`, 9 `hop_frames` `U32`, 10 `capture_epoch`,
    11 `sequence`, 12 `dropped_captures`, 13 `windows`, 14 `captured_sample`, 15 `end_sample`,
    16 `analysis_epoch`, 17 `history_start_sample` (all `U64`), 18 `smoothing_ms` `F64`,
    19 `source_underrun` `Bool`; a `ready` reply adds 20 `frequencies_hz`, 21 `left_db`,
    22 `right_db` (`PackedF32`, as D2).
  - A read's status is never a refusal: `inactive`, `warming`, `pending`, `gap`, `failed`,
    `stopped` and `ready` are `OK` replies.
- **D4. Execution.** Through #1594 D9's `PreparedCommandFrame::Session`, on the monitor's service
  and analysis (#1593). A one-shot read is `SpectrumService::read`, then
  `SpectrumAnalysis::analyze_into` into one `SpectrumAnalysisResult` the monitor allocated when the
  configuration was set. A stream read is `read_stream(Some(&mut analysis))`, then
  `analyze_stream_into`. The FFT runs on the thread that calls `submit_command`.
- **D5. Size preflight: a read never consumes what it cannot deliver.** Before `read` or
  `read_stream`, the session measures the reply that a ready result of the request's shape would
  have (the requested `channels` and `include_window`; for a stream, the selected mask), with the
  codec's measure. If it exceeds the frame limit the read answers `LIMIT_EXCEEDED`
  (`spectrum.frame`); if it exceeds the caller's output capacity it answers
  `CommandError::BufferTooSmall { required }`. Neither pops a window, consumes a gap or failure
  status, or advances a token.
- **D6. Refusals.** Every #1591 and #1592 error answers with one status and its `code()`:

  | Error | Status |
  |---|---|
  | `Unprepared`, `UnknownEntry` | `NOT_FOUND` |
  | `Busy` | `BACKPRESSURE` |
  | `Dropped`, `CaptureInvalid` | `UNAVAILABLE` |
  | `Exhausted` | `LIMIT_EXCEEDED` |
  | `SelectSingle`, `NotArmed`, `Channels`, `StreamActive`, `StreamInactive`, `CadenceRate`, `CadenceShape` | `VALIDATION_FAILED` |
  | `SpectrumRequestError`, `Smoothing` | `INVALID_FIELD` |

  A refusal changes no service state.
- **D7. Replay.** Replies are recorded like every response: an exact replay of a read returns the
  same bytes and pops nothing. A host that reads often gives each read a new request ID.
- **D8. Capability.** With `ProviderFeatures::spectrum` set (the C ABI session), the registry
  lists `0x0010`, `0x0011` and `0x0012` and sets capability flag bit 16 (#1594 D10).
- **D9. Docs.** As #1594 D11, for the two messages: the semantics doc states D5 and that the
  analysis runs on the calling control thread.

## Deliverables

1. D1-D3 and D8 in `crates/protocol`; corpus frames and the schema hash.
2. D4-D7 in `crates/control-plane/src/{control.rs,spectrum.rs}`.
3. D9.
4. The C ABI tests and the cross-host parity test below.

## Authorized paths

- `crates/protocol/src/**`, `crates/conformance/src/protocol_corpus.rs`,
  `crates/conformance/tests/conformance_corpus.rs`, `fuzz/corpus/complete-schema-manifest.md`.
- `crates/control-plane/src/**`, `crates/host-core/src/spectrum_service.rs` (only if a method
  needs a read-only accessor for D5).
- `crates/capi/src/runtime/spectrum_tests.rs`, `crates/capi/include/miso_engine_v1.h` (prose),
  `tools/audit/src/protocol.rs`.
- `hosts/host-web/tests/spectrum_parity.rs` (new; gate 5).
- `docs/CONTROL_PROTOCOL_{REGISTRY,SEMANTICS,SIZING}.md`, `docs/CONTROL_BTLV_V1.md`,
  `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- No spectrum event: reads are pulls (frozen C13 decision).
- No browser or SDK change (#1596). No change to the analysis maths.

## Objective gates

Commands: `cargo test --locked -p protocol --all-features`, `cargo test --locked -p conformance`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo test --locked -p host-web --features host-web/test-support`,
`cargo build --locked --release -p audit -p capi && target/release/audit capi`,
`bash scripts/check-capi-abi.sh`, `cargo run --locked --release -q -p realtime-policy`.

Fixture as #1593's, with `maximum_control_frame_bytes` 65,536 unless a gate says otherwise.

1. **Codec.** Both requests and replies round-trip. Each presence rule of D2 and D3, a status
   outside its table, a 1024-entry `left_db` and a smoothing of 10,001 are refused on encode and
   on decode.
2. **One-shot.** Configure single `(t1 post-pan, Stereo)` (#1594). `arm` is done; `read` is pending
   until 2048 frames have rendered; then `read` is ready. Its bins equal word for word
   `SpectrumAnalysis::analyze_into` of the reference window (a session prepared with
   `prepare_host_runtime_with_spectrum`, fed the same PCM from frame 0); with `include_window`
   the planes equal that window. The second round's `snapshot_token` is 2.
3. **Nothing is lost.** With a 4,096-byte output buffer, a ready `read` answers
   `RESULT_BUFFER_TOO_SMALL` with `required_bytes`; a second `read` with a large enough buffer
   returns the same window (same token and `captured_sample`). On a session whose frame limit is
   12,288 bytes, a stereo `read` answers `LIMIT_EXCEEDED` (`spectrum.frame`), and a `Left` read then
   returns the same window. A `gap` stream read refused for size answers `gap` on the next read.
4. **Stream.** Collection `(t1 post-pan, Stereo)`, `(t2 post-input, Left)`: `select` `t1`, `start`
   at 100 ms, reads go `warming` to `ready` with the hop of #1592 gate 1; `select` `t2` at 50 ms
   advances `selection_epoch` and `analysis_epoch`; three skipped hops give `gap` with
   `dropped_captures` 2, then `ready`; `reset` advances `analysis_epoch`; `stop` gives `stopped`;
   a cancel of the one-shot during the stream is `VALIDATION_FAILED` `spectrum.stream.active`.
5. **Cross-host parity.** In one process, the browser host (host-web native, through its present
   arm, read and analysis exports) and the C ABI session, on the same document and PCM: the
   one-shot bins and the bins of the first four ready stream windows are equal word for word.
6. **Refusals.** Each D6 row once (for example read before arm, select of an unknown entry, arm
   twice, read `Left` on a `Right` entry, start while a caller's arm is pending): each answers its
   status and code, and the next valid call behaves as if the refused one never ran.
7. **Replay.** An exact replay of a ready read returns the same bytes and the next new read is
   pending.
8. **Realtime.** The control thread runs 1,000 reads (both messages) while the render thread
   renders: the render thread's `bench_support::alloc` counters read `allocations == 0 &&
   frees == 0` around every render call, and no call answers `INTERNAL`.
9. **Capability.** `CAPABILITIES_GET` on the C ABI session lists `0x0010`, `0x0011` and `0x0012`
   and sets bit 16.

## Test value

- Gate 1 turns red if the codec accepts a contradictory request or a reply of the wrong shape.
- Gate 2 turns red if the C ABI analyzes differently from the core or labels the window wrongly.
- Gate 3 turns red if a read pops a window or consumes a status it then fails to deliver.
- Gate 4 turns red if a stream status, epoch or reset differs from the core machine.
- Gate 5 turns red if the two hosts give different bins for the same audio.
- Gate 6 turns red if a refusal writes state or uses a status outside the table.
- Gate 7 turns red if a replayed read pops again.
- Gate 8 turns red if a read path allocates on render or races the plan.
- Gate 9 turns red if a client cannot discover the spectrum commands and the hop.

## Dependencies

- #1594 (the dispatch variant and the configuration), #1593, #1592, #1591.
- `crates/capi` is stream B's: rebase after stream B's open edits.

## Supersedes and shrinks

- None.
