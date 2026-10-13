# Start, select, read and stop the spectrum stream through one core state machine

Slice S29 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

The continuous spectrum stream has one state machine, in host-core: start, select with smoothing,
read with its seven statuses, reset, stop, the stream metadata and the smoothing history. host-web
keeps only its metadata record and its exports, and calls the machine. The browser's bytes do not
change, except for one ruled refusal (D4). The C ABI slices call the same machine.

## Context

- **The core kernels.** `crates/host-core/src/spectrum.rs`: `start_continuous` (`:601`; a single
  capture refuses `Busy` unless idle, `:606-609`), `start_continuous_with_hop` (`:618`),
  `restart_continuous` (`:665`), `stop_continuous` (`:717`), `cadence` (`:742`), `stream_epoch`
  (`:757`), `try_read_continuous` (`:765`, the read state machine `:776-838`); the collection forms
  (`:1041`, `:1059`, `:1078`, `:1085`, `:1094`, `:1101`, `:1117`); a collection start replaces any
  arm of the selected entry (`begin_continuous(cadence, true)`, `:1054`, `:1073`). `SpectrumHop`
  (`:1151`), `SpectrumCadence` (`:1174`), `SpectrumContinuousWindow` (`:1265`),
  `SpectrumContinuousReadError` (`:1307`), `SpectrumContinuousCaptureError` (`:1330`),
  `SpectrumSmoothingConfig` (`:2353`), `SpectrumAnalysisHistory` (`:2393`; `reset` `:2421`,
  `analysis_epoch` `:2433`), `SpectrumAnalysisMetadata` (`:2470`), `analyze_continuous` (`:2545`).
- **host-web holds the machine.**
  - State: `SpectrumStaging`'s stream fields (`hosts/host-web/src/ffi.rs:123-146`:
    `stream_metadata`, `stream_active`, `stream_cadence`, `stream_smoothing`, `stream_history`,
    `stream_window`) and `SpectrumStreamWindowFacts` (`:148`).
  - Metadata bookkeeping: `stream_metadata_profile` (`:1104`), `stream_metadata_error` (`:1127`),
    the import check `imported_stream_configuration` (`:1152-1216`).
  - Commit logic: `select_spectrum_with_smoothing` (`:2842-2965`: preflight of the analysis epoch,
    host select, restart on a smoothing change, history reset, metadata refresh), the exports
    `spectrum_stream_select` (`:2983`), `spectrum_stream_start` (`:3055-3133`),
    `spectrum_stream_read` (`:3136-3286`: status mapping, history reset on gap and failure),
    `spectrum_stream_analysis` (`:3289`), `spectrum_stream_analysis_configure` (`:3307`),
    `spectrum_stream_reset` (`:3333`), `spectrum_stream_stop` (`:3349-3379`), and the refusal of
    `spectrum_cancel` while a stream runs (`:3412-3415`).
  - Host methods: `start_spectrum_stream` (`hosts/host-web/src/lib.rs:2563`, the cadence error
    mapping `:2583-2592`), `restart_spectrum_stream` (`:2597`), `read_spectrum_stream` (`:2620`),
    `stop_spectrum_stream` (`:2636`), `spectrum_stream_cadence` (`:2652`),
    `spectrum_stream_epoch` (`:2661`); the boot hop `spectrum_hop` (`:2143`), set by
    `miso_engine_web_v1_boot_with_spectrum_hop` (`ffi.rs:3857`, `boot_staged_with_spectrum_hop`
    `:3836`).
  - Records: the status codes (`lib.rs:255-267`) and `WebSpectrumStreamMetadata` (`lib.rs:362`).
- **Two instances in the browser.** The worklet runs start, select, read and stop and copies the
  raw window and the metadata out (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1347-1414`).
  The SDK's analysis Worker imports the metadata and analyzes with its own history
  (`sdk/src/core/spectrum.ts:699-754`). The headless SDK does both in one instance.
- **#1165.** `SpectrumCaptureCollection::select` restarts a running stream on the new entry with
  the default cadence (`spectrum.rs:1001`), so an explicit hop is lost. *Preserve configured
  spectrum cadence when switching collection targets* (#1165) owns the fix.
- **#1591** adds `SpectrumService`, `SpectrumAnalysis`, the snapshot, the error enum and the D7
  ruling's select half.

## Decisions frozen for this slice

- **D1. Status and metadata.** In `crates/host-core/src/spectrum_service.rs`:
  - `#[repr(u32)] pub enum SpectrumStreamStatus { Inactive = 0, Warming = 1, Pending = 2, Gap = 3,
    Failed = 4, Stopped = 5, Ready = 6 }` (the browser's codes).
  - `pub struct SpectrumStreamMetadata` with the fields of `WebSpectrumStreamMetadata` except
    `struct_size`, `abi_version`, `result`, `reserved0` and `reserved1`: `status`, `target_kind`,
    `channels` (raw mask, 0 without a selection), `sample_rate_hz`, `quantum_frames`,
    `hop_frames`, `source_underrun: bool`, `capture_epoch`, `sequence`, `dropped_captures`,
    `windows`, `captured_sample`, `end_sample`, `analysis_epoch`, `history_start_sample`,
    `smoothing_ms: f64`. host-web's record adds `result`, the mapped result word of the last call.
- **D2. The capture side, on `SpectrumService`.** `SpectrumService::new` gains
  `hop: Option<SpectrumHop>`. The service holds the stream state (active, cadence, smoothing,
  window facts), the metadata and one boxed `SpectrumContinuousWindow` made at construction.
  - `start_stream(&mut self, smoothing: SpectrumSmoothingConfig, analysis: Option<&mut SpectrumAnalysis>)
    -> Result<(), SpectrumServiceError>`: with `hop` it calls the `_with_hop` start, else the
    default start, with the service's rate and quantum.
  - `select` (#1591) and `select_stream(&mut self, target, channels, smoothing, analysis)`: the
    commit logic of `select_spectrum_with_smoothing`, unchanged in effect. `select_stream` needs an
    active stream (`StreamInactive`).
  - `read_stream(&mut self, analysis: Option<&mut SpectrumAnalysis>) -> Result<(), SpectrumServiceError>`:
    one `try_read_continuous` and the bookkeeping of `spectrum_stream_read`. A ready window lands in
    the boxed window; `stream_metadata()` and `stream_window() -> Option<&SpectrumContinuousWindow>`
    read the result. A gap or failure resets the given analysis.
  - `stop_stream(&mut self)`, `stream_metadata(&self)`, `stream_cadence`, `stream_epoch`.
  - `cancel` refuses `StreamActive` while a stream runs.
  - The `analysis` argument is `Some` where one owner holds both halves (the headless instance,
    and the C ABI from #1593). Then every preflight that today reads the history (an analysis epoch
    at `u64::MAX`) runs before the first write, and the history reset happens at the same commit.
    The browser's worklet instance passes `None`.
  - Every capture-side method is allocation-free (the worklet calls them render-locked).
- **D3. The analysis side, on `SpectrumAnalysis`.** It gains the history (boxed once), cadence,
  smoothing and window facts:
  - `reset_stream(&mut self) -> Result<(), SpectrumServiceError>`;
  - `configure_imported(&mut self, metadata: &SpectrumStreamMetadata) -> Result<(), SpectrumServiceError>`,
    the semantic checks of `imported_stream_configuration` (status ready, valid kind and mask,
    non-zero capture epoch, `windows == sequence + 1`, `end_sample == captured_sample + 2048`,
    a valid cadence and smoothing); the record checks stay in host-web;
  - `analyze_stream_into(&mut self, window: &SpectrumContinuousWindow, out: &mut SpectrumAnalysisResult,
    metadata: &mut SpectrumStreamMetadata) -> Result<(), SpectrumServiceError>`, which runs
    `analyze_continuous` and writes `analysis_epoch` and `history_start_sample` into `metadata`.
- **D4. Ruling: single versus collection, the start half.** A stream start refuses `Busy` while
  #1591's `caller_armed` is set (a caller's arm or its unread window), on both forms. On the
  collection, an arm that `select` made itself is replaced, as `begin_continuous(cadence, true)`
  does today. On the single form an idle capture starts, as today. The one browser sequence that
  changes is `arm`, then `streamStart`, on a collection (was: the arm was discarded; now:
  `RESULT_BACKPRESSURE`). No SDK path sends it. The rule is #1591 D7's: no acknowledged arm loses
  its window without a refusal.
- **D5. Errors added to `SpectrumServiceError`** (host-web keeps today's words):

  | Variant | `code()` | host-web result |
  |---|---|---|
  | `StreamActive` | `spectrum.stream.active` | `RESULT_WRONG_STATE` |
  | `StreamInactive` | `spectrum.stream.inactive` | `RESULT_WRONG_STATE` |
  | `CadenceRate` | `spectrum.cadence.rate` | `RESULT_UNSUPPORTED` (start); `RESULT_WRONG_STATE` (restart) |
  | `CadenceShape` | `spectrum.cadence.shape` | `RESULT_INVALID_ARGUMENT` (start); `RESULT_WRONG_STATE` (restart) |
  | `Smoothing` | `spectrum.smoothing` | `RESULT_INVALID_ARGUMENT` |
  | `MetadataInvalid` | `spectrum.metadata.invalid` | `RESULT_INVALID_ARGUMENT` |

  Hop and epoch overflow are `Exhausted` (`RESULT_REFUSED_BUDGET`). A read is never an error for
  its status: host-web maps `Inactive` to `RESULT_WRONG_STATE`, `Warming` and `Pending` to
  `RESULT_BACKPRESSURE`, `Gap` and `Ready` to `RESULT_OK`, `Failed` to `RESULT_RENDER_REJECTED`,
  as `ffi.rs:3161-3195` does.
- **D6. The acked-batch rule.** Every check runs before the first write, as today. `read_stream`
  changes the read counters (gap, recovery, failure) only by returning a status. A caller that
  cannot deliver the status must not call it; #1595 D5 preflights the reply size for this reason.
- **D7. host-web calls the machine.** `SpectrumStaging` keeps its byte buffers, the metadata
  record and one `SpectrumAnalysis`; its stream fields and `SpectrumStreamWindowFacts` go. The
  stream exports and host methods become a call, the D5 mapping and the record encode or decode.

## Deliverables

1. D1-D6 in `crates/host-core/src/spectrum_service.rs` (and re-exports).
2. D7 in `hosts/host-web/src/{lib.rs,ffi.rs}`.
3. Stream tests in `crates/host-core/tests/spectrum_service.rs`.

## Authorized paths

- `crates/host-core/src/spectrum_service.rs`, `crates/host-core/src/lib.rs` (re-exports),
  `crates/host-core/tests/spectrum_service.rs`.
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/ffi.rs` (spectrum code only).

## Non-goals

- No edit of `spectrum.rs` (the #1165 fix is its own issue). No wire message. No record, export or
  ABI layout change. No SDK change.

## Objective gates

Commands: as #1591's gate list.

1. **Stream sequences.** A collection `(t1 post-pan, Stereo)`, `(t2 post-input, Left)` at 48 kHz,
   quantum 128, fed a sine on each track, owner with both halves: `select(t1)`, `start_stream(100 ms)`
   gives `Warming`, then `Pending`, then `Ready` with `sequence` 0, 1, 2 and the default hop
   spacing; `select_stream(t2, 100 ms)` advances `selection_epoch` and `analysis_epoch`, resets
   `history_start_sample`, and the next ready window is `t2`'s; `select_stream(t2, 50 ms)` restarts
   the capture epoch; three skipped hops give `Gap` with `dropped_captures` 2 and a reset history,
   then `Ready`; `cancel` during the stream is `StreamActive`; `stop_stream` gives `Stopped`.
2. **Explicit hop.** With `hop = 1024` the ready windows start 1024 samples apart; after
   `select_stream(t2, ..)` they still do (holds only with #1165 on `main`).
3. **The D4 ruling.** Collection: `select(t1)` then `start_stream` succeeds; `cancel`, `arm`, then
   `start_stream` is `Busy` and the armed window still reads. Single: `arm` then `start_stream` is
   `Busy` (unchanged).
4. **Import.** A table over D3's checks: each malformed field gives `MetadataInvalid` and leaves
   the history unchanged; a valid import followed by `analyze_stream_into` equals the one-owner
   result for the same window.
5. **Realtime.** After warm-up the capture-side methods read `allocations == 0 && frees == 0` on
   `bench_support::alloc`'s thread-scoped counters.
6. **Browser parity.** host-web's spectrum tests, the SDK's spectrum evals and the browser
   qualification's continuous-spectrum and hop rows pass with no edit, except one new host-web
   test for D4's changed sequence.

## Test value

- Gate 1 turns red if a status, an epoch or a history reset moves to a different call than today.
- Gate 2 turns red if the service starts or switches a stream with the default cadence when the
  preparation named a hop.
- Gate 3 turns red if a stream start discards a caller's arm, or no longer replaces a select-made
  arm.
- Gate 4 turns red if the analysis instance accepts metadata the one-owner path would never write.
- Gate 5 turns red if the moved stream code allocates on the browser's audio thread.
- Gate 6 turns red if the move changes a byte that the browser or the SDK reads.

## Dependencies

- #1591.
- *Preserve configured spectrum cadence when switching collection targets* (#1165) lands first. It
  is a half-day fix in `spectrum.rs` with its own gates; this slice moves code and does not mix a
  behaviour fix into that move. Root adds #1165 to STREAMS.md row 89.
- `hosts/host-web/src/lib.rs` is stream H's file (STREAMS.md row 101): rebase after stream H.

## Supersedes and shrinks

- None.
