# Capture, select and analyze one spectrum window through one core service

Slice S28 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

Decision 17 moves the spectrum feature into the core in seven slices: this slice (the one-shot
capture), #1592 (the stream), #1593 (spectrum taps on every plan), #1594 (`SPECTRUM_CONFIGURE`),
#1595 (the C ABI reads), #1596 (the browser and the SDK send the same frames) and #1597 (the
browser's own spectrum code goes).

## Product outcome

One host-core module owns the one-shot spectrum: the request rules (target kind, target ID,
channels, byte budget), the prepared single target or collection, arm, read, cancel and select, the
snapshot token, and the analysis of one window into dB bins. host-web keeps only its records and
exports and calls this module. The browser's bytes do not change, except for one ruled refusal
(D7). The C ABI slices call the same module.

## Context

- **The core already owns the kernels.** `crates/host-core/src/spectrum.rs`: `SpectrumTarget`
  (`:168`), `SpectrumTargetRef` (`:222`), `SpectrumCaptureRequest` (`:233`),
  `SpectrumCaptureCollectionRequest` (`:253`), `SpectrumWindow` (`:428`), `SpectrumCapture`
  (`:486`; `arm` `:527`, `cancel` `:538`, `try_read` `:568`), `SpectrumCaptureCollection` (`:858`;
  `arm_selected` `:931`, `cancel` `:939`, `select` `:979`, `selection_would_change` `:1023`,
  `selection_epoch` `:926`, `try_read` `:1107`), the read and arm errors (`:1129`, `:1140`), the
  render observer (`:1520`, its realtime region `:1593-1649`) and the analyzer (`SpectrumAnalyzer`
  `:2490`, `analyze` `:2515`). The two preparation forms are `prepare_capture` (`:1410`) and
  `prepare_capture_collection` (`:1479`).
- **host-web holds the rest of the one-shot logic.**
  - Request rules: `spectrum_channels` (`hosts/host-web/src/ffi.rs:926`), `spectrum_target_ref`
    (`:937`), `spectrum_target` (`:950`) and the semantic half of `staged_spectrum_request`
    (`:964-1069`: ID length 1 to 127, single budget 1 byte to 1 MiB, collection budget non-zero,
    empty collection with budget 0). Constants: target kinds (`hosts/host-web/src/lib.rs:228-233`),
    channel masks (`:235-239`), `SPECTRUM_MAXIMUM_ID_BYTES` (`:241`, 127),
    `SPECTRUM_CAPTURE_BYTES` (`:243`, 1 MiB).
  - The preparation enum `SpectrumPreparationRequest` (`lib.rs:1432`) and the wrapper
    `PreparedSpectrumCapture` (`lib.rs:1439-1540`).
  - Host methods with state checks, error mapping and the snapshot token: `arm_spectrum`
    (`lib.rs:2518`), `read_spectrum` (`:2535`, token `spectrum_token` `:2141`), `cancel_spectrum`
    (`:2669`), `spectrum_target` (`:2685`), `spectrum_channels` (`:2696`), `select_spectrum`
    (`:2712`; a single preparation answers `RESULT_UNSUPPORTED`, `:2726`),
    `spectrum_selection_would_change` (`:2738`), `spectrum_selection_epoch` (`:2769`).
  - The read export `miso_engine_web_v1_spectrum_read` (`ffi.rs:3010`) checks the requested
    channels against the selected mask (`:3028-3031`), pops (`:3032-3034`), checks the mask again
    after the pop (`:3040-3042`) and encodes the window (`write_spectrum_window`, `:1238`).
  - The analysis driver `run_spectrum_analysis` (`ffi.rs:1326-1524`) decodes the window record,
    checks it, runs the analyzer and encodes `WebSpectrumResult` (`lib.rs:332`); failures encode
    through `spectrum_failure` (`ffi.rs:1085`).
- **The browser splits capture and analysis over two instances.** The worklet's port handler
  (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1267`) calls the arm, read, cancel and
  select exports, which are render-locked (`tools/realtime-policy/render-thread-exports.json`), and
  copies the window record out. A separate SDK Worker instance analyzes it
  (`sdk/src/browser/response-worker.ts:103-105` → `SpectrumModule.analyzeSnapshot`,
  `sdk/src/core/spectrum.ts:607`, which calls `spectrum_capture_set_bytes` and
  `spectrum_analysis`). The headless SDK reads and analyzes in one instance
  (`sdk/src/core/boundary.ts:925-941`).
- **Single versus collection.** A collection's `select` arms the new entry by itself and cancels
  every other entry (`spectrum.rs:979-1021`, `cancel_except` `:947`). A select therefore discards a
  one-shot window that the caller armed with `arm` and has not read. A single preparation has no
  select. The stream half of the difference (`start_continuous`, `:601` versus `:1041`) is #1592's.

## Decisions frozen for this slice

- **D1. One module.** New file `crates/host-core/src/spectrum_service.rs`, re-exported from
  `crates/host-core/src/lib.rs`. It calls `spectrum.rs` and does not edit it (stream A owns that
  file: STREAMS.md row 89).
- **D2. Request rules.** Moved from host-web, with the browser's codes:
  - Constants `SPECTRUM_TARGET_TRACK_POST_INPUT = 1`, `SPECTRUM_TARGET_TRACK_POST_PAN = 2`,
    `SPECTRUM_TARGET_OUTPUT = 3`, `SPECTRUM_CHANNEL_LEFT = 1`, `SPECTRUM_CHANNEL_RIGHT = 2`,
    `SPECTRUM_CHANNEL_BOTH = 3`, `SPECTRUM_MAXIMUM_ID_BYTES = 127`,
    `SPECTRUM_MAXIMUM_CAPTURE_BYTES = 1 << 20`. host-web re-exports them under its present names,
    so its public constants do not change.
  - `spectrum_target_ref(kind: u32, id: &str) -> Result<SpectrumTargetRef<'_>, SpectrumRequestError>`,
    `spectrum_target_kind(target: &SpectrumTarget) -> u32` and
    `spectrum_channels_from_raw(raw: u32) -> Result<SpectrumChannels, SpectrumRequestError>`. None
    of them allocates.
  - `pub enum SpectrumPreparation { Single(SpectrumCaptureRequest), Collection(SpectrumCaptureCollectionRequest) }`
    (host-web's `SpectrumPreparationRequest`, renamed and moved). Constructors
    `SpectrumPreparation::single(kind, id, channels, maximum_capture_bytes)` and
    `SpectrumPreparation::collection(entries: &[(u32, &str, u32)], maximum_capture_bytes) ->
    Result<Option<Self>, SpectrumRequestError>` apply today's rules: ID 1 to 127 bytes; a valid
    kind and mask; a single budget from 1 to `SPECTRUM_MAXIMUM_CAPTURE_BYTES`; a collection budget
    that is non-zero (an aggregate, with no 1 MiB cap); an empty collection only with budget 0,
    which returns `None`. Duplicate entries stay a preparation refusal (`host.spectrum.duplicate`).
  - `SpectrumRequestError { TargetKind, TargetId, Channels, Budget }`, with
    `code() -> &'static str`: `spectrum.target.kind`, `spectrum.target.id`, `spectrum.channels`,
    `spectrum.budget`.
  - The record checks (`struct_size`, `abi_version`, reserved words, UTF-8 of the staged bytes,
    single and collection staged together) stay in host-web: they are transport.
- **D3. One service.** `pub struct SpectrumService` owns the prepared captures (host-web's
  `PreparedSpectrumCapture`, moved as `PreparedSpectrumCaptures`), the snapshot token, the session
  rate and a `caller_armed: bool`. Its one-shot methods (#1592 adds the stream to the same type):
  - `new(captures: PreparedSpectrumCaptures, sample_rate_hz: u32, quantum_frames: u32) -> Self`.
  - `arm(&mut self) -> Result<(), SpectrumServiceError>`: the single capture's `arm` or the
    collection's `arm_selected`. A success sets `caller_armed`.
  - `read(&mut self, requested: SpectrumChannels) -> Result<Option<SpectrumSnapshot>, SpectrumServiceError>`.
    Before the pop it checks that an entry is selected (`NotArmed`), that `requested` is a subset
    of the selected mask (`Channels`) and that the token can advance (`Exhausted`). Then it pops:
    pending is `Ok(None)`; a window advances the token, clears `caller_armed` and returns the
    snapshot with `window.channels = requested`; an invalid capture clears `caller_armed` and
    returns `CaptureInvalid`. A refusal before the pop leaves the window queued.
  - `cancel(&mut self)` clears `caller_armed`.
  - `select(&mut self, target: SpectrumTargetRef<'_>, channels: SpectrumChannels) -> Result<bool, SpectrumServiceError>`
    (`true` when the selection changed), `selection_would_change`, `selection_epoch`,
    `target() -> Option<&SpectrumTarget>`, `target_kind() -> u32` (0 without a selection),
    `channels() -> Option<SpectrumChannels>`.
  - Every method above is allocation-free: the browser calls them inside render-locked exports on
    the audio thread.
- **D4. The snapshot.** `pub struct SpectrumSnapshot { pub window: SpectrumWindow, pub target_kind: u32,
  pub sample_rate_hz: u32, pub token: u64 }`, with `validate(&self) -> Result<(), SpectrumServiceError>`:
  a valid kind, a non-empty mask, a representable end sample, a non-zero token and a launch rate
  (`WindowInvalid` otherwise). host-web decodes its window record into this type and calls
  `validate`; the semantic checks of `run_spectrum_analysis` (`ffi.rs:1348-1363`: kind, mask,
  frames, token, end sample) move here, and the record checks of the same block stay.
- **D5. The analysis half.** `pub struct SpectrumAnalysis` holds one boxed `SpectrumAnalyzer`,
  made once by `SpectrumAnalysis::new()`. `analyze_into(&self, snapshot: &SpectrumSnapshot, out: &mut SpectrumAnalysisResult)
  -> Result<(), SpectrumServiceError>` validates, runs `SpectrumAnalyzer::analyze` and fills `out`.
  `pub struct SpectrumAnalysisResult` holds `target_kind`, `channels`, `sample_rate_hz`,
  `captured_sample`, `end_sample`, `token`, `source_underrun`, `frequencies_hz: [f32; 1025]`,
  `left_dbfs: [f32; 1025]` and `right_dbfs: [f32; 1025]` (a plane outside `channels` holds the
  floor and is not read). The caller owns `out`, so a repeated analysis allocates nothing.
- **D6. Errors.** `pub enum SpectrumServiceError`, with `code()` and the host-web mapping that
  keeps today's result words:

  | Variant | `code()` | host-web result |
  |---|---|---|
  | `Unprepared` | `spectrum.unprepared` | `RESULT_UNSUPPORTED` |
  | `SelectSingle` | `spectrum.select.single` | `RESULT_UNSUPPORTED` |
  | `Busy` | `spectrum.busy` | `RESULT_BACKPRESSURE` |
  | `NotArmed` | `spectrum.not_armed` | `RESULT_WRONG_STATE` |
  | `CaptureInvalid` | `spectrum.capture.invalid` | `RESULT_RENDER_REJECTED` |
  | `Exhausted` | `spectrum.exhausted` | `RESULT_REFUSED_BUDGET` |
  | `UnknownEntry` | `spectrum.entry.unknown` | `RESULT_INVALID_ARGUMENT` |
  | `Channels` | `spectrum.channels` | `RESULT_INVALID_ARGUMENT` |
  | `WindowInvalid` | `spectrum.window.invalid` | `RESULT_INVALID_ARGUMENT` |

  host-web keeps one mapping function; its lifecycle check (`STATE_READY`) stays in host-web.
- **D7. Ruling: single versus collection, the select half.** An arm that the caller made with
  `arm` belongs to the caller until a read returns its window or `CaptureInvalid`, or until
  `cancel`. While `caller_armed` is set, `select` refuses with `Busy` and changes nothing. An arm
  that `select` made itself (the collection's implicit arm) is not the caller's, and a later
  `select` replaces it as today. Reason: the acked-batch rule; a caller's acknowledged arm must
  not lose its window without a refusal. The same rule covers the single form, which has no select
  (`SelectSingle`, unchanged). #1592 D4 applies the rule to the stream start. The only browser
  sequence whose result changes is `arm` followed by `select` on a collection (was: replaced; now:
  `RESULT_BACKPRESSURE`). No SDK path sends it.
- **D8. host-web calls the service.** The ready state holds `Option<SpectrumService>` in place of
  `spectrum_capture` and `spectrum_token`; `None` answers `RESULT_UNSUPPORTED`. The host methods
  and the arm, read, cancel and select exports become a call plus the D6 mapping.
  `write_spectrum_window` and the result encoder stay (transport). `run_spectrum_analysis`
  becomes: decode the record, `SpectrumSnapshot::validate`, `SpectrumAnalysis::analyze_into` into
  a result held by the staging, encode. The staging's `analyzer` field becomes a
  `SpectrumAnalysis`. The post-pop mask check (`ffi.rs:3040-3042`) goes: the pre-pop check covers
  it.

## Deliverables

1. D1-D7 in `crates/host-core/src/spectrum_service.rs` and the re-exports in
   `crates/host-core/src/lib.rs`.
2. D8 in `hosts/host-web/src/{lib.rs,ffi.rs}`.
3. The new test binary `crates/host-core/tests/spectrum_service.rs` (gates 1-4). host-web's
   `spectrum_ffi_tests` (`ffi.rs:5350`) keep their assertions; a test that only checked logic now
   in the core moves to the new binary.

## Authorized paths

- `crates/host-core/src/spectrum_service.rs` (new), `crates/host-core/src/lib.rs` (module and
  re-exports), `crates/host-core/tests/spectrum_service.rs` (new).
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/ffi.rs` (spectrum code only).
- `docs/REALTIME_DEPENDENCY_POLICY.md` (the spectrum sentences at `:331-337`, only if the reached
  functions change their names).

## Non-goals

- No stream change (#1592). No new wire message (#1594, #1595). No edit of `spectrum.rs`.
- No change to a `Web*` record, an export name or an ABI layout row.
- No change to the SDK.

## Objective gates

Commands: `cargo test --locked -p host-core --features host-core/test-support`,
`cargo test --locked -p host-web --features host-web/test-support`,
`bash scripts/check-host-core-policy.sh`, `bash scripts/test-host-core-policy.sh`,
`cargo run --locked --release -q -p realtime-policy`, `bash scripts/check-web-audioworklet.sh`,
`bash scripts/test-web-audioworklet.sh`, `bash scripts/check-sdk-headless.sh`,
`python3 -B scripts/check-abi-layout-v1.py`, `bash scripts/check-cross-targets.sh`,
`bash scripts/check-workspace-policy.sh`, and `npm run qualify -- --artifacts ... --sdk-root ...
--browser <b> --check-matrix --self-test-mutations` in `hosts/host-web/qualification` for
chromium, firefox and webkit.

1. **Request rules.** A table test over D2: each kind (0, 1, 3, 4), mask (0, 1, 3, 4), ID length
   (0, 1, 127, 128), single budget (0, 1, 1 MiB, 1 MiB + 1), collection budget (0 with entries,
   0 without, 2 MiB) gives the D2 result and code.
2. **One-shot sequences.** On sessions prepared with `prepare_host_runtime_with_spectrum` and
   `prepare_host_runtime_with_spectrum_collection` (`crates/host-core/src/prepare.rs:1331`,
   `:1360`), fed a 997 Hz sine: read before arm is `NotArmed`; read while capturing is `Ok(None)`;
   read with `Left` on an entry prepared `Right` is `Channels`, and the next read with `Right`
   returns that same window; tokens are 1, 2, 3 over three arm and read rounds; `cancel` then
   read is `NotArmed`; select of an unknown entry is `UnknownEntry`; select on the single form is
   `SelectSingle`.
3. **The D7 ruling.** On the collection: `select(a)`, then `select(b)` replaces `a`'s implicit arm
   and `b`'s window arrives. Then `arm` fails `Busy` (already armed by select); after the read,
   `arm` succeeds, and `select(a)` is `Busy` with `selection_epoch` unchanged; the next read
   returns `b`'s window, and only then does `select(a)` succeed.
4. **Analysis.** `analyze_into` of a snapshot fills the same words as `SpectrumAnalyzer::analyze`
   with the same window and rate; a snapshot with token 0, kind 4 or rate 32 kHz is
   `WindowInvalid` and `out` is unchanged.
5. **Realtime.** In the test binary, after warm-up, `arm`, `read`, `cancel` and `select` read
   `allocations == 0 && frees == 0` on `bench_support::alloc`'s thread-scoped counters; in the
   browser qualification `miso_engine_web_v1_render_allocation_count` stays 0.
6. **Browser parity.** host-web's spectrum tests, the SDK's `spectrum-evals.mjs` and
   `spectrum-browser-evals.mjs` (through `check-sdk-headless.sh`) and the browser qualification's
   spectrum rows pass with no edit, except one new host-web test for D7's changed sequence. The
   one-time comparison of the analyzed bytes against the base commit is PR evidence.

## Test value

- Gate 1 turns red if a rule moved to the core accepts or refuses differently from the browser.
- Gate 2 turns red if a read consumes a window it refuses, if the token skips or repeats, or if a
  single preparation accepts a select.
- Gate 3 turns red if a select discards a caller's armed window without a refusal, or if the
  select-made arm is no longer replaceable.
- Gate 4 turns red if the analysis half changes a bin or writes output for a refused snapshot.
- Gate 5 turns red if the moved code allocates on the browser's audio thread.
- Gate 6 turns red if the move changes a byte that the browser or the SDK reads.

## Dependencies

- None among the decision-17 slices. `hosts/host-web/src/lib.rs` is stream H's file (STREAMS.md
  row 101): this slice rebases after stream H's open edits to it.

## Supersedes and shrinks

- None.
