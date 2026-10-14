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

## Amendment (controls coordinator, batch 8, 2026-10-14)

Checked against `main` at `32cee58d0`.

- **Dependencies met.** #1591 merged in batch 6 (`44ff8004c`, review fold `9625fe263`). #1165
  merged in batch 4 (`e2371f932`): `SpectrumCaptureCollection::select` now moves the old entry's
  checked cadence to the new entry (`crates/host-core/src/spectrum.rs:997-1012`). The Context
  bullet on #1165 is history. Gate 2's "holds only with #1165 on `main`" is now unconditional.
- **`spectrum.rs` anchors** (the file has not changed since batch 6). The single forms keep their
  lines (`:601`, `:618`, `:665`, `:717`, `:742`, `:757`, `:765`). The rest moved by two lines: the
  collection forms `:1043`, `:1061`, `:1080`, `:1087`, `:1096`, `:1103`, `:1119`;
  `begin_continuous(cadence, true)` `:1056`, `:1075`; `SpectrumHop` `:1153`, `SpectrumCadence`
  `:1176`, `SpectrumContinuousWindow` `:1267`, `SpectrumContinuousReadError` `:1309`,
  `SpectrumContinuousCaptureError` `:1332`, `SpectrumSmoothingConfig` `:2355`,
  `SpectrumAnalysisHistory` `:2395` (`reset` `:2423`, `analysis_epoch` `:2435`),
  `SpectrumAnalysisMetadata` `:2472`, `analyze_continuous` `:2547`.
- **`hosts/host-web/src/ffi.rs` anchors** (lines moved because the flip deleted code above them):
  `SpectrumStaging` `:136-165`, which already holds #1591's `analysis: Option<SpectrumAnalysis>`
  (`:150`); its stream fields `:157-163`; `SpectrumStreamWindowFacts` `:167`;
  `stream_metadata_profile` `:1105`, `stream_metadata_error` `:1128`,
  `imported_stream_configuration` `:1153`; `select_spectrum_with_smoothing` `:2826`; the exports
  `spectrum_stream_select` `:2967`, `_start` `:3026`, `_read` `:3107`, `_analysis` `:3262`,
  `_analysis_configure` `:3280`, `_reset` `:3306`, `_stop` `:3322`; the stream refusal of
  `spectrum_cancel` `:3385`; `boot_staged_with_spectrum_hop` `:3753` and
  `miso_engine_web_v1_boot_with_spectrum_hop` `:3774`.
- **`hosts/host-web/src/lib.rs` anchors.** The status codes `:251-263`; `WebSpectrumStreamMetadata`
  `:358`; #1591's result mapping `spectrum_result` `:409`; the host's `spectrum_hop` `:1692`;
  `start_spectrum_stream` `:2381` (its cadence mapping `:2405-2414`), `restart_spectrum_stream`
  `:2419`, `read_spectrum_stream` `:2438`, `stop_spectrum_stream` `:2454`,
  `spectrum_stream_cadence` `:2470`, `spectrum_stream_epoch` `:2480`. The worklet's stream
  operations are `miso-engine-v1-audio-worklet.js:1345-1411` (select `:1270-1330`); the SDK's
  analysis instance is `sdk/src/core/spectrum.ts:699-754`, called from
  `sdk/src/browser/response-worker.ts:109`.
- **#1591's hand-off ends here.** `SpectrumService::captures_mut` and `end_caller_arm`
  (`crates/host-core/src/spectrum_service.rs:502-509`) exist for this slice. Today host-web calls
  `end_caller_arm` after a successful start (`lib.rs:2402-2404`): that is the "the arm was
  discarded" behaviour that D4 changes. When `start_stream` owns the start, delete
  `end_caller_arm` and the stream use of `captures_mut`.
- **Decisions and gates hold.** The slice touches no browser rebuild path, deleted command kind,
  host mirror or carry-chain code. The flip did not change spectrum behaviour.
- **Lane.** D7 is host-web code, which only lane W edits (STREAMS.md, controls program, "Lanes").
  #1565B also edits `lib.rs` and `ffi.rs` in batch 8; the later of the two rebases. The old note
  "rebase after stream H" reads: stream H's open host-web work is the Worker deployment (batch
  table, row "later"); whichever of it and this slice lands second rebases.

## Decision record: S29 (batch 8)

**Shipped.** D1-D7 as one state machine in `crates/host-core/src/spectrum_service.rs`.
`SpectrumService` (now `new(captures, rate, quantum, hop)`, one boxed `SpectrumContinuousWindow`)
owns `start_stream`, `select_stream`, `read_stream`, `stop_stream`, the metadata, the cadence, the
last window and the `StreamActive` refusal of `cancel`. `SpectrumAnalysis` owns the history (boxed on
the first analysis or import), the stream cadence and smoothing, `reset_stream`, `configure_imported`
and `analyze_stream_into`. host-web keeps its byte buffers, the metadata record, one
`SpectrumAnalysis` and thin exports (`ffi.rs`: `stream_metadata_of`, `write_stream_metadata`,
`imported_stream_metadata`, `adopt_imported_stream`). #1591's `captures()`, `captures_mut()` and
`end_caller_arm` are deleted, as are the host's `spectrum_hop` field, `restart_spectrum_stream` and
`spectrum_selection_would_change`. D4 holds: a stream start refuses `Busy`
while the caller's arm stands (`RESULT_BACKPRESSURE` in the browser).

**Departures from the text, and why.**
- `select_stream`'s `smoothing` is `Option<SpectrumSmoothingConfig>`. The plain `spectrum_select`
  export, while a stream runs, restarts the stream exactly as the stream select does (the browser's
  bytes), so it calls `select_stream(.., None, ..)` (`None` keeps the smoothing). To make that the
  only road, the plain `SpectrumService::select` now refuses `StreamActive` while a stream runs.
- `stop_stream(&mut self)` has no analysis argument (as the text); the analysis side has
  `SpectrumAnalysis::end_stream`, which the host-web stop export calls.
- `analyze_stream_into` takes the window with its own capture facts. The analysis keeps only whether
  a window is at hand (`has_stream_window`): a reset or an unavailable read clears it, as the old
  `stream_window = None`.
- The history is boxed lazily (first `analyze_stream_into` or `configure_imported`), not at
  `SpectrumAnalysis::new`. With an eager history a stream started after a one-shot analysis would
  report `analysis_epoch` 1 where the browser reports 0.
- The audio thread keeps no `SpectrumAnalysis`, so the headless instance, which starts a stream
  before its analysis exists, is served by `miso_engine_web_v1_spectrum_stream_analysis`: when the
  analysis does not run the stream and the metadata record is a ready window, it adopts the stream
  from the record with the import's checks (`adopt_imported_stream`). Without a ready record the
  answer is `RESULT_WRONG_STATE`. One edge differs from the old code: an analysis export after a
  `pending` read answered `RESULT_INVALID_ARGUMENT`, and now answers `RESULT_WRONG_STATE`.
- The metadata record's analysis fields (`analysis_epoch`, `history_start_sample`) are refreshed
  from the analysis on every service call that takes one, and are 0 without one.
- The host-side stream window (about 16 KiB) is charged in `spectrum_staging_retained_bytes` and
  `spectrum_staging_largest_allocation_bytes` for a configured spectrum.
- Gate 1 uses the fixture's two entries (`eq0` post-input Left, `main-out` Stereo), not the text's
  post-pan and post-input pair.

**Gates and tests.**
1. `stream_sequence_one_owner` (crates/host-core/tests/spectrum_service.rs): select, start,
   warming, pending, ready 0/1/2 at 2,048 spacing, restart on a changed entry and on changed
   smoothing (history reset, fresh capture epoch), gap with two drops, `StreamActive`, stop, inactive.
2. `an_explicit_hop_survives_start_and_select`: hop 1,024 spacing before and after `select_stream`.
3. `a_stream_start_never_discards_a_callers_arm` (core) and
   `stream_start_refuses_a_callers_arm_on_a_collection` (host-web, the one new browser-parity test).
4. `imported_metadata_is_checked_and_analyzes_like_the_owner`: 14 malformed fields, each
   `MetadataInvalid` with no adopted state; an import analyzes bit-equal to the owner.
5. `stream_calls_allocate_and_free_nothing`: 0 allocations, reallocations and frees for start,
   select, read, stop and cancel on `bench_support::alloc`'s thread-scoped counters.
6. Browser parity: host-web `--lib` and integration tests pass; `check-sdk-headless.sh` (463 SDK
   tests, including the spectrum evals) passes against a rebuilt artifact;
   `check-web-audioworklet.sh` (callgraph, boot budget, ABI layout) passes. The in-browser
   qualification rows were not run here.
- One host-web test is deleted: `failed_stream_metadata_starts_new_epoch_drop_count_at_zero`
  tested a moved helper; the rule is `SpectrumStreamMetadata::record_outcome`, tested in
  `spectrum_service::tests::a_failed_window_starts_a_fresh_epoch_with_no_drops`. Host-web tests that
  read the removed staging stream fields now read the analysis through test helpers.
- `realtime-policy`: ok, 130 marked regions in 28 files; the two spectrum-service regions now also
  hold the stream calls.
