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
- **D8a. #1594 D10 moves here whole (root decision R3 of #1594, 2026-10-24).** This slice adds
  capability bit 16, `CapabilityFlags::KNOWN` with it, `ProviderFeatures::spectrum` and the two
  decode invariants of #1594 D10 (bit 16 is set exactly when `0x0010`, `0x0011` and `0x0012` are
  listed; the three are listed together or not at all). #1594 left `0x0010` allocated, served by
  both hosts' sessions and listed by no registry; the `SessionCommand` variant of
  `PreparedCommandFrame::Session` is where this slice's reads go.
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
- #1609 (make the spectrum one-shot handoff safe across threads), root 2026-10-14.

## Supersedes and shrinks

- None.

## Decision and evidence record: slice 1595a (attempt 1)

Scope (root, 2026-10-24; scope note `1595.md`, R1-R7 accepted): 1595a is `SPECTRUM_CAPTURE`
(`0x0011`), served and unadvertised as `0x0010` is. 1595b (`SPECTRUM_STREAM` `0x0012`, bit 16,
`KNOWN`, `ProviderFeatures::spectrum`, the decode invariants, R2's switch) is a later slice. The
root decisions amend the frozen ones above:

- **R1.** No new C symbol or struct: the read rides `miso_engine_v1_submit_command` through
  `PreparedCommandFrame::Session`; the header changes prose only (`check-capi-abi.sh` green).
- **R3 (amends D5).** The `BufferTooSmall` branch is dropped: the controller already refuses a new
  request whose output is below the frame limit before the session runs. The one preflight is
  `ProtocolCodec::spectrum_capture_ready_frame_len`, which measures a ready reply of the read's
  shape with the encoder's own `CountSink`; a read above the frame limit answers `LIMIT_EXCEEDED`
  `spectrum.frame` before it pops.
- **R4.** Gate 5 and `hosts/host-web/tests/spectrum_parity.rs` are dropped.
- **R5.** Tests in `crates/control-plane/tests/spectrum_monitor.rs`, the codec test in
  `message_wire/tests.rs`, one capi row at a 65,536-byte frame limit.
- **R6.** `SPECTRUM_CONFIGURE` refuses (`LIMIT_EXCEEDED` `spectrum.frame`) a tap whose one-plane
  read without the window the frame limit cannot carry.
- **R7.** The monitor makes one boxed `SpectrumAnalysisResult` with its service; the capi charge
  is `SPECTRUM_MONITOR_RETAINED_BYTES` (`SpectrumService::RETAINED_BYTES` plus that box).

Decisions made while building (implementer, for review):

- The reply type borrows: `SpectrumCaptureReply<'a>` holds `Cow` planes, so the session encodes
  straight from the monitor's result and the popped window (no plane copy); a decoded reply owns
  them. `SessionCommandReply` gains the lifetime and loses `Copy`; a capture reply is encoded in
  `complete_session_command` itself (`complete_session_encoded`), not held as a `Body`.
- The codec pins `window_frames` 2,048 and `bin_count` 1,025 (`SPECTRUM_CAPTURE_*` constants),
  and checks `end_sample = captured_sample + 2048`, a non-zero token and a finite floor.
- A ready reply's `target_kind`/`target_id` are the service's selected target (the window's).
- D6 has no row for `WindowInvalid` or `MetadataInvalid`: both answer `UNAVAILABLE`, as
  `CaptureInvalid` does. `Busy` answers `BACKPRESSURE` as D6 says; the controller's non-OK builder
  then attaches its replay-cache backpressure record (a pre-existing shape of every
  `BACKPRESSURE` reply), with the code `spectrum.busy`.
- `include_window` absent and present-false decode alike (D2: absent is false); the encoder
  writes it only when true.
- `crates/protocol/src/typed_frame.rs` is edited (the payload enums and their dispatch), as
  #1594 did; the spec authorizes `crates/protocol/src/**`, the scope note's list omitted it.
- Gate 7 is at the session (`spectrum_monitor.rs`), not `controller/tests.rs`: its second half
  ("the next new read is pending") needs the session; the controller's own replay of a session
  command is #1594's `a_session_command_completes_once_and_replays_its_exact_bytes`.
- Gate 2 taps `eq0` post-input: the fixture's `eq0` post-pan carries about 2e-17 (its window is at
  the floor in every bin), which would make a bin comparison vacuous.

Corpus re-pin (approved paths: `conformance` `main.rs`, `mutation_million.rs`, the parity script,
`docs/CONTROL_PROTOCOL_CONFORMANCE.md`): the corpus gains `command.spectrum_capture` (a read of
both planes with the window) and `response.spectrum_captured` (a ready one-plane read); 52
frames. `COMPLETE_SCHEMA_HASH` moves from `a3d6a9dd72195701` to `c7e010362827dab3`. The FNV hash
of the corpus with those two frames left out, in order, is `a3d6a9dd72195701`, the old pin: the
50 earlier frames are byte-equal.

Frame sizes (measured by the gate-1 test against the encoder, identity `eq0`): one plane 8,496
bytes, both 12,608, one plane with the window 16,696, both with the window 29,008.

Gates (attempt 1), test value:

- Gate 1 `message_wire::tests::spectrum_capture_round_trips_and_refuses_malformed_frames_both_ways`:
  red if the codec admits a contradictory request or a reply of the wrong shape (planes against
  `channels`, a 1,024-bin plane, a lone window plane, a pending reply with window fields), or the
  frame-limit measure drifts from the encoder.
- Gate 2 `a_capture_read_returns_the_reference_analysis`: red if the read path analyzes, labels
  or counts a window differently from `analyze_into` on the reference window.
- Gate 3 and R6 `a_read_the_frame_cannot_carry_consumes_nothing`: red if a read pops a window it
  cannot deliver, or a configure accepts a tap no read can carry.
- Gate 6 `each_capture_refusal_answers_its_status_and_changes_nothing`: red if a refusal writes
  state or maps outside D6 (unprepared, not armed, select single, busy twice, unknown entry,
  channels, dropped).
- Gate 7 `a_replayed_read_returns_its_bytes_and_pops_nothing`: red if a replayed read pops again.
- Gate 8, a capture phase in
  `alternating_configures_against_a_rendering_thread_allocate_nothing_on_render` (1,000 captures,
  20 of them ready): red if a read allocates on the render thread or answers `INTERNAL`.
- R7 row `a_plan_with_spectrum_charges_the_service_once` (amended): red if the result box is not
  charged.
- capi `runtime::spectrum_tests::spectrum_commands_cross_the_c_abi_to_the_session` replaces the
  #1594 row (whose 4,096-byte frame limit R6 now refuses): red if a C ABI client cannot reach a
  ready read.

Acked-batch question for a read: can a reply acknowledge a window it did not return? No. Every
refusal (the D5 measure, `Channels`, `NotArmed`, `Exhausted`) runs before the pop, the output is
reserved for the frame limit before the session runs, and the reply is recorded for replay in the
call that writes it. The one consume-then-fail path is an analyzer refusal of a popped window
(`WindowInvalid`: the analyzer's numerical refusal, which huge finite samples can cause); it
answers `UNAVAILABLE` `spectrum.window.invalid`, has consumed the window and ended the arm, and
the semantics doc states the exception.

Mutants (each red on its gate, restored by `git checkout` + `touch`): no read preflight (gate 3),
no configure R6 check (gate 3), `Busy` to `VALIDATION_FAILED` (gate 6), left bins from the right
plane (gate 2), no replay record for a capture reply (gate 7), the R6 charge without the result box
(R7 row), a plane length not checked (gate 1), a read accepting target fields (gate 1). One
mutant was equivalent (re-planning a cached request: the preflight refuses it anyway) and is not
counted.

Fold after the PASS verdict (root, 2026-10-24):

- The read preflight's `include_window` is tested: a `Left` read with its window (16,696 bytes)
  at the 12,288-byte limit is refused and the window stays (the `*include_window -> false`
  mutant, which survived, is now red).
- R6's lower bound is tested: a configure of the tap is accepted at 12,288 bytes (the
  `Left -> Both` mutant is now red).
- The monitor's one-shot slot (`OneShot`: the result and the popped snapshot) is boxed once with
  the service, so a ready read no longer boxes its snapshot; `SPECTRUM_MONITOR_RETAINED_BYTES`
  charges the whole slot and the R7 row adds `size_of::<SpectrumSnapshot>()`.
- Gate 7 renders a second window before the replay; the next new read is ready with token 2 at
  frame 2,048.
- Follow-ups recorded by root: a spectrum-arm backpressure queue kind (NIT 5); R6 runs only in
  `SPECTRUM_CONFIGURE`, the browser's boot path is unadvertised (NIT 7, for 1595b).

## Decision and evidence record: slice 1595b (attempt 1)

Scope (root, 2026-10-24; scope note `1595.md`, R1-R7 accepted): 1595b is `SPECTRUM_STREAM`
(`0x0012`), capability bit 16 with `CapabilityFlags::KNOWN`, `ProviderFeatures::spectrum`, the
two decode invariants (bit 16 is set exactly when `0x0010`, `0x0011` and `0x0012` are listed; the
three are listed together or not at all) and R2's per-host switch. It follows 1595a's shapes: the
`Cow` reply encoded in `complete_session_command` (now one `complete_session_borrowed` for both
reads), the codec's frame-limit measure (`ProtocolCodec::spectrum_stream_ready_frame_len`), the
monitor's one preallocated read slot (a ready stream read analyzes into the same result) and the
`WindowInvalid` exception.

- **R2.** `ControlAdapter::SPECTRUM` (default false; capi true) builds
  `ProviderFeatures { spectrum: A::SPECTRUM, ..ALL }`. Every session serves the three commands;
  only the C ABI advertises them. `ProviderFeatures::ALL` includes `spectrum`.
- **NIT 7 of the 1595a verifier (R6 on the boot path), addressed.** An adapter with `SPECTRUM`
  runs R6 on a boot configuration in `compile_children` (`spectrum.frame\t$\n`), so advertising
  and R6 move together; #1596 turns it on for the browser by setting the const. The C ABI passes
  no boot configuration today.

Decisions made while building (implementer, for review):

- **R6 grows to the stream read.** A stream reply carries five more scalar fields, so its
  one-plane ready frame (8,576 bytes, identity up to 8 bytes) is larger than the one-shot's
  (8,496). R6 (configure and advertised boot) now refuses a tap unless both one-plane reads fit,
  so every accepted tap has a readable one-plane capture and, on a one-plane entry, a readable
  stream. A two-plane entry still streams only under a frame limit of 12,688 bytes; below it, each
  stream read is refused `LIMIT_EXCEEDED` `spectrum.frame` before it reads (the entry fixes the
  channels). The alternative, refusing a stereo tap below 12,688, would reverse 1595a's accepted
  12,288-byte stereo case; it is not taken.
- `smoothing_ms` is required on `start` and optional on `select` (absent keeps it). `select`
  runs `SpectrumService::select_target`: the plain selection without a stream (a smoothing then
  is `spectrum.stream.inactive`), the restarting selection with one, as the browser's select does.
- The reply's selection is `Option<SpectrumStreamTarget>`: kind and channels `0` and no identity
  without a selection, and a ready reply needs one. The reply's target is the service's
  committed selection.
- Only a read answers `ready`. After a ready read, a `reset` discards the window at hand, so its
  reply (and that of any non-read op whose metadata still says ready) answers `pending`.
- Every reply's `analysis_epoch` and `history_start_sample` are the analysis's own (a reset
  changes only the analysis); a ready read's metadata is the one `analyze_stream_into` wrote.
- `SessionCommand` loses `Eq` (the stream request holds an `f64`). `read_f64` joins the BTLV
  readers.
- Paths outside the scope note, needed by R2's advertisement and recorded for root:
  `crates/capi/src/runtime/tests.rs` (two pinned C ABI capability vectors re-pinned: field 25
  length `0x1e` with two padding bytes, payload `0x190`, flags `0x17fdf`) and
  `hosts/host-web/tests/protocol_cross_host.rs` (the C ABI's capabilities are the browser's plus
  `0x0010`-`0x0013` and bit 16; #1596 removes the spectrum part of that exception).
- #1944 (the `BACKPRESSURE` record's queue kind for spectrum refusals) stays open; a stream
  `Busy` answers as a capture `Busy` does.

Corpus re-pin (the four approved paths): the corpus gains `command.spectrum_stream` (a select
with a smoothing) and `response.spectrum_streamed` (a ready one-plane read); 54 frames.
`COMPLETE_SCHEMA_HASH` moves from `c7e010362827dab3` to `c146e8fad0b26299`. The FNV hash of the
corpus with those two frames left out, in order, is `c7e010362827dab3`, the old pin: the 52
earlier frames are byte-equal. The static capabilities frame lists no spectrum command and leaves
bit 16 clear, which the invariant allows.

Frame sizes (measured against the encoder): a ready stream reply is 8,576 bytes for one plane and
12,688 for both (identity up to 8 bytes; 8,696 and 12,808 at 127 bytes).

Gates (attempt 1), test value:

- Gate 1 `message_wire::tests::spectrum_stream_round_trips_and_refuses_malformed_frames_both_ways`:
  red if the codec admits a contradictory stream request or a reply of the wrong shape (planes
  against `channels`, a 1,024-bin plane, a smoothing of 10,001 or NaN, a status outside the
  table, a selection without its identity, a ready window count that is not `sequence + 1`), or
  the stream measure drifts from the encoder.
- Bit 16 invariants, assertions added to
  `message_wire::tests::capability_id_views_reject_order_allocation_and_session_family_identically`:
  red if a record that lists only some of the three, or lists them without bit 16, or sets bit 16
  without them, decodes.
- R2 at the controller, assertions added to
  `controller::tests::the_capability_registry_lists_the_control_messages_and_not_the_retired_ones`:
  red if `ProviderFeatures::spectrum` does not switch the three IDs and bit 16 together.
- Gate 4 `a_stream_read_follows_the_core_machine`: red if a reply's status, sequence, window
  count, drops, samples, hop or epochs differ from the core machine (the reference session's
  `read_stream`, fed the same PCM), or the first four ready windows' bins differ word for word
  from `analyze_stream_into` there; it also walks select (epochs advance), gap (two drops), reset
  (`pending`, analysis epoch one more), the one-shot cancel refusal (`spectrum.stream.active`)
  and stop.
- Gate 3 and R6 `a_stream_read_the_frame_cannot_carry_consumes_nothing`: red if a size-refused
  stream read consumes its `gap` or drops (the core's next read is that gap, two drops), or R6
  ignores the stream's larger one-plane size (a configure and an advertised boot at 8,496 bytes
  are refused, at 8,576 accepted).
- Gate 9, the capi row `spectrum_commands_cross_the_c_abi_to_the_session` (amended): red if a C
  ABI client cannot discover the three commands and bit 16, or a stream read cannot reach ready.

Acked-batch question for a stream: can a reply acknowledge a window it did not deliver, or lose a
drop count? No. The size refusal runs before `read_stream`, so it pops nothing and consumes no
`gap` or `failed` status; a `gap` reply carries `dropped_captures`; the output is reserved for
the frame limit before the session runs and the reply is recorded for replay in the call that
writes it. The consume-then-fail paths are the analyzer's `WindowInvalid` of a popped window (as
for a capture) and the core's own `Exhausted` refusal of `read_stream` (an analysis epoch or
window count at `u64::MAX`), which the core records as `failed`.

Mutants (each red on its gate, restored by `git checkout` + `touch`): no stream read preflight
(gate 3), R6 without the stream measure (R6 row), no boot R6 (R6 row), stream left bins from the
right plane (gate 4), a reset reply with a stale analysis epoch (gate 4), a ready status kept on a
reset reply (gate 4), select through `select_stream` (gate 4), the reply's smoothing unchecked
(gate 1), the `0x0011` together rule dropped (invariants), bit 16 never set (controller row),
capi `SPECTRUM` false (capi row). One survived and was folded: the ready `windows = sequence + 1`
rule unchecked; gate 1 now refuses it.

Attempt 2 (root rulings on the verifier's FAIL, 2026-10-24):

- **R6 (MINOR, done).** R6 refuses a tap unless its one-plane one-shot read (without the window)
  and its stream read of the tap's own channels fit: 8,576 bytes for a one-plane tap, 12,688 for
  a stereo tap (identity up to 8 bytes), at configure and at an advertising host's boot. No
  accepted tap is refused on every read. The 1595a decision above ("the alternative ... is not
  taken") is reversed. Gate 3 (one-shot) now runs at 16,384 bytes and keeps a stereo refusal
  through the window (29,008 bytes); gate 3 (stream) keeps its size-refused `gap` on a session
  that does not advertise (its boot runs no R6) and checks R6 at each tap's exact size and one
  byte below, by configure and by an advertising boot (test adapter `Advertised`).
- **iOS memset (MAJOR), blocked on one reader.** With the ruled host-core change (dead floor fills
  to zero; `analyze_into` and `analyze_stream_into` write straight into `out`; the unrequested
  plane is unspecified) the shipped capi iOS library has 0 `memset_pattern16` calls charged to
  host-core (the ratchet asks to delete the row). The C ABI encoder and the browser read only
  requested planes, but `crates/host-core/tests/spectrum_service.rs:527-532` reads the unrequested
  plane and asserts it is the floor (the old contract), so the ruling's stop rule applies: the
  change is not committed, and the `host-core` row stays at 10.
- **iOS memset, done (root authorized, 2026-10-24).** The plane contract of
  `SpectrumAnalysisResult` changes: a plane outside `channels` is not read and its contents are
  unspecified (it held the floor). The floor fills of `SpectrumAnalyzer::analyze` and
  `analyze_continuous` were dead stores and are zero fills; `analyze_into` and
  `analyze_stream_into` write straight into `out` (no staging arrays or copies; a refusal still
  leaves `out` unchanged, since the analyzer writes only after its last `Err`). Requested planes'
  bits do not move (gates 2 and 4 compare them word for word). The assertion that an unrequested
  plane holds the floor (`crates/host-core/tests/spectrum_service.rs`) is deleted, because the
  ruling removes that claim; the test's other assertions stay. The host-core iOS
  `memset_pattern16` count is 0 (4 on main before this slice, 10 in attempt 1), so its row is
  deleted; the `builtins` and `lane` rows stay for #1957.
