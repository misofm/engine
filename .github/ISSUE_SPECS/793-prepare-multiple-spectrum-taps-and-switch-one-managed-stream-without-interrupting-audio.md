# Prepare multiple spectrum taps and switch one managed stream without interrupting audio

Parent: #763. Builds on accepted #789/#791 source
`1749b05fe7c06548e18b92bda30b0cad51d4828e`. Astra xhigh scopes; Luna implements;
fresh Astra medium verifies concrete bugs, at most five attempts. Root creates
and synchronizes this numbered issue before implementation, after PR #792's
required checks/merged delivery and the preceding issue-boundary audit. Accepted
#789/#791 behavior and evidence are the baseline, not reopened review work.

## Smallest closable outcome

Prepare several explicitly selected graph taps when opening one engine, then
use `subscribeSpectrum` and the existing handle's `update` to select one of them
without stopping playback, rebuilding the plan, seeking sources or recreating
the AudioContext. This enables the app's single expanded EQ panel to follow
arbitrary focused-track changes. The app uses `trackPostMatrix`; preserve all
three existing tap kinds.

This child supports multiple **prepared** entries and one **active** spectrum
job with identical-consumer sharing. Simultaneous jobs and atomic multi-target
batches remain required #763 successors. Keep the current FFT, smoothing,
subscription owner, timer, Worker and pooled transport; no general framework,
SAB implementation, benchmark or new test harness.

## Requirements

1. **Bounded preparation.** Add an optional collection input with semantic shape
   `{ entries: [{ target, channels }], maximumCaptureBytes }`, mutually exclusive
   with the existing singular `spectrum` option. An entry is an exact supported
   tap kind + stable track/output ID + `left`/`right`/`both` mask. Reject duplicate
   exact entries. Different masks may be separately prepared; runtime requests
   require an exact prepared mask, preserving the accepted managed contract.
   The old singular form remains a one-entry compatibility path.
2. Resolve and admit the **entire** collection against the prepared graph before
   publishing the host. Use checked aggregate capture-byte and existing
   graph/host/largest-allocation limits; reject unknown entries and insufficient
   budgets atomically. No implicit all-track preparation or compiled track limit.
   Expose the accepted entries and effective profile/resource facts through the
   host/SDK. This small preparation report is not a new discovery framework.
3. Reuse the existing `SpectrumCaptureRequest`, `prepare_capture`, observer and
   one-slot queue per entry, with unique observer identities and one prepared
   collection. Target lookup stays off render. Charge actual observer, queue,
   control, ID and collection bytes, including both currently reserved PCM
   arrays for a single-lane entry. Report capture separately from analysis and
   transport. Empty preparation retains no capture observers; inactive entries
   do no PCM copying, FFT or traffic. Memory cannot grow with duration or focus
   changes. Do not claim that inactive observer checks cost zero CPU.
4. **Atomic selection.** Add one native host operation to replace the selected
   capture under exclusive ownership between render calls; the browser executes
   it in one Worklet message handler outside `process()`. Validate the target,
   exact mask, configuration, budgets and epoch arithmetic before retiring the
   working selection. A refusal preserves the old native capture and SDK
   configuration/references/result. A successful commit disarms and clears the
   old partial/queued capture, arms the selected entry and acknowledges its
   effective configuration, unambiguous selection identity and earliest eligible
   sample boundary. First captured sample remains pending until actually known.
   Two separately fallible SDK stop/start requests do not satisfy this contract.
5. Reuse the shared SDK owner's serialization and in-flight gate. Wait for the
   bounded previous read/analysis to settle under the existing deadline before
   replacement; commit handle state after native success. Old replies/results
   cannot be attributed to a new target. A changed selection begins warming
   with fresh capture/smoothing history; A -> B -> A cannot recover A's stale
   partial window. Returned owned arrays keep their original identity/span.
   Identical effective updates preserve history and #791's monotonic loss
   baseline. Incompatible updates while other handles share the active job
   refuse before disturbing them; existing cadence/callback/close behavior stays.
6. Reuse one spectrum analyzer/history, Worker and transfer credit per engine,
   initialized once off render before first subscription acceptance. Size
   staging/transport for the largest admitted entry so switching from one lane
   to both cannot allocate a replacement pipeline. Native capture allocations
   happen at preparation. Keep current result-copy ownership and separately
   bounded returned arrays; no Worker or FFT instance per prepared entry/handle.
   N, Hann, normalization, floor, H and smoothing domain stay as accepted in #789.
   Generated records/exports are additive V1; frozen layouts remain compatible.
7. A selection update while paused may succeed but remains warming until real
   rendering resumes. It never resumes/renders audio to obtain a spectrum.
   Existing response selection and actual captured-state timing remain intact.
   The app's immediate paused edits can use the delivered requested-configuration
   preview; no response DSP or command-scheduling change belongs in this child.

## Three checkpoints and proportional evidence

1. **Prepared collection and native switch.** Extend the existing spectrum
   fixture at 48 kHz with two differently identified/tuned tracks and request
   order different from compiled order. Cover A -> B -> A during partial capture,
   unknown-entry and aggregate-budget refusal, one unsupported mask, and fresh
   history/span/selection association. The same fixture proves one active
   capture, bit-identical audio, zero render allocations/frees, empty/idle costs
   and fixed memory through repeated switches. Reuse existing rate/Q192,
   numerical and failure tests; do not expand their matrices. Root checkpoints
   the focused-green native tranche before bridge work.
2. **Host/SDK selection over the existing pipeline.** Extend current managed
   spectrum tests for successful update, refused shared/invalid update preserving
   the old stream, a delayed old reply across switching, and paused switch/resume.
   Verify the same Worker/buffer is reused across different prepared masks and
   existing close/reload/loss tests remain green. Do not add a second lifecycle
   harness. Root checkpoints the focused-green bridge/SDK tranche.
3. **Combined acceptance.** Extend the existing known-signal SDK/browser probe to
   switch the selected track and compare its tone/gain/meter evidence at the
   correct target/sample span while audio continues. Run the existing browser
   qualification and proportional SDK/generated ABI/package/artifact/resource/
   realtime gates on the stopped candidate. Carry #789/#791's unchanged DSP,
   transport-failure and loss evidence; no new listening, timing campaign,
   numerical corpus or paused-preview matrix. One fresh adversarial verdict
   follows the coherent attempt; root delivers and synchronizes on PASS.

## Downstream and parent boundary

App/adapter changes are a separate delivery slice. The adapter must forward the
prepared collection and managed SDK analysis methods without another analyzer
or subscription owner; publish matching SDK/adapter packages and prove packed
Worker/Worklet consumption. The last verified app main `00f77713` pins SDK 0.2.4
and adapter 0.5.0 (`0564511d`, codec 0.1.1). Fetch their latest main again before
scoping/implementation, work outside the dirty primary checkout, and integrate
latest app main again before landing with relevant gates rerun. Preserve BLAKE3
identities/cache migration, sparse decoding/concurrency/progress and existing UI.

#763 remains open for simultaneous multiple targets/configurations and their
sharing; atomic multi-target batch subscribe/update; full effect/graph discovery
and response fields/bands; complete clock/plan/state/content/publication identity
and bounded joins; SAB; native/remote packed-vector parity/compatibility; remaining
integrated examples/resource evidence and the frozen descriptive comparison.
This child neither deletes those requirements nor reopens accepted #789/#791.

Milestone 1 native checkpoint: Luna max adds a bounded prepared collection by reusing existing captures/observers, transactional collection preparation, and exact-entry native selection. Existing spectrum integration suite passes 9/9, including a two-track selection/PCM/allocation fixture; host-core check, all-feature Clippy and formatting pass (/tmp/issue793-milestone1-*). This is a native checkpoint, not endpoint acceptance: complete host configuration/selection identity admission and bridge/SDK operation remain in milestone 2.

Latest downstream baseline refresh: app main 53d9b069c4c5d9fc09856c2fdf3030fb20079fb3 (PR209), SDK0.2.4/adapter0.5.1; adapter main cced684beb84f2152cd237beeecd40e4d9a68ab6. Preserve 512KiB warm verification reads separately from128KiB ingest caps, integrity/cancellation/progress semantics and existing latest app behavior. Refresh again before final app implementation and landing. Primary app checkout remains untouched.

Milestone 2 host bridge checkpoint: Luna max adds bounded collection staging, native exact-target/configuration transaction with selection-epoch preflight, collection one-shot/stream lifecycle, fixed staging/resource accounting and additive V1 ABI exports/layouts. The FFI regression proves a rejected selection leaves the active capture intact. Host-core spectrum tests9/9, host-web91 passed/2 ignored, metadata10 tests, Clippy, formatting and ABI/codegen checks pass (/tmp/issue793-milestone2-*). SDK/Worklet message wiring, actual artifact qualification and single whole-endpoint Astra medium verdict remain required.

App integration requirements are now recorded in misofm/app#210, based on fresh main53d9b069 with SDK0.2.4/adapter0.5.1. Its isolated spec worktree preserves the dirty primary. Implementation waits for this issue acceptance and compatible published SDK/adapter; user-specified Luna xhigh implementation then Astra medium verification remain binding.
