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

Root actual-Wasm resource check of native581842 fails the old resource pin: bridgeMetadata1113143→1151831 and bridgeRetained1133652→1172340 (/tmp/issue793-native581842-resources.log). Inspection identifies an original-contract violation: fixed256 prepared-target staging and a collection aggregate budget capped by the unrelated one-window1MiB staging cap. Do not repin or accept this shape. SDK layering pauses while the bridge is corrected to caller-configured bounded collection staging/admission, allocated off render before cached views, with aggregate capture resources separate from one-window staging. This is correction within the first coherent endpoint attempt, not a new feature or waived gate; prior buildable checkpoint remains preserved.

The native staging correction is focused-green: caller-count entry storage and packed ID storage replace fixed256 allocation; aggregate capture admission is separate from the one-window buffer. Compatibility metadata0 denotes caller-sized capacity, with actual capacity returned after staging. Focused FFI8/8 includes257 entries and2MiB aggregate budget; host check, metadata/ABI validator/self-test, generated SDK surface, formatting and diff checks pass (/tmp/issue793-native-dynamic-*). Fresh actual-Wasm resource evidence remains required before SDK layering resumes.

Root fresh Wasm0b6d3 artifact SHA256056fee8760ae955e813060c424238945a18bd7cd7fe630c905b4923c0aeeb711 is preserved at /tmp/issue793-native0b6d3-artifact. Actual resource accounting and native witness now agree, with26 mutations rejected (/tmp/issue793-native0b6d3-resources-corrected.log). Unconfigured bridge overhead is only the additive32-byte WebSpectrumCollectionRequest: metadata1113143→1113175, retained1133652→1133684. Caller-sized empty entry/ID tables retain no payload. The earlier38688-byte fixed-table increase is removed. SDK wiring may resume on the corrected native candidate; final hostJS/browser/package qualification remains pending.

SDK/headless vertical checkpoint compiles and keeps the existing singular spectrum suite7/7 green. The real collection probe found stale target metadata after selection (/tmp/issue793-sdk-collection-headless.log). Native FFI now refreshes selected target/channels and gates reset on an actual selection-epoch change; focused switch/idempotency regression and formatting pass (/tmp/issue793-native-switch-*). Fresh Wasm revalidation is pending. This checkpoint is explicitly incomplete: SDK currently refuses valid combined target+smoothing changes, which violates the frozen sole-owner update contract and must be corrected by a complete native transaction, not accepted as a limitation. Clippy also identifies too-many-arguments in project_buffers introduced by this branch staging-resource plumbing; fix it locally without a broad refactor. Preserve the compiling checkpoint; no endpoint PASS is claimed.

Native complete stream-selection checkpoint: target, exact channel mask and smoothing are admitted together through the additive V1 stream-select operation. Same-entry smoothing changes restart the existing capture at a fresh epoch and discard queued old windows without a stop/start failure gap. Target-only switches also reset existing history; exact effective no-ops preserve it. No analyzer/history is allocated on the Worklet host merely to start or switch a stream. Host-core library30/30, host-web FFI8/8, ABI7/7, validators/self-test, all-feature Clippy, formatting and diff checks pass (/tmp/issue793-native-history-*). The earlier ab2484 real-Wasm basic collection switch also passed (/tmp/issue793-sdk-collection-ab2484.log). Complete SDK/browser transaction wiring and whole-endpoint verification remain pending; no endpoint acceptance is claimed.

SDK complete-update checkpoint: the shared owner now sends target/channel/smoothing changes through one native transaction and uses its copied warming metadata. Existing tests cover shared refusal before mutation, no stop/start during combined updates, preserved owned old arrays and exact no-op behavior. Spectrum suite8/8 with the real artifact enabled, TypeScript and ABI codegen pass (/tmp/issue793-sdk-focused-artifact.*). Root actual-Wasm combined target/channel/smoothing smoke passes (/tmp/issue793-sdk-combined-update-root.log). Native743081 artifact SHA256c1191d67052806984441eca262d6678583f36f88eaec9f7495a3580d4d81c7b4 is preserved at /tmp/issue793-native743081-artifact and its actual resource/native-witness/26-mutation check passes. Browser collection forwarding and full acceptance remain pending.

Browser pipeline checkpoint: collection preparation and atomic stream selection now cross the existing host/Worklet/SDK interfaces. The existing browser fixture captures distinct A/B/A results with both/left/both masks in a running AudioContext; Chromium151.0.7922.34 qualification and self-test mutations pass (/tmp/issue793-browser-collection-chromium.log). TypeScript, syntax and diff checks pass. Root full headless gate also passes228/228 (/tmp/issue793-sdk80fe-headless.log). This is a useful compiling checkpoint, not endpoint acceptance: root inspection finds no explicit selected-target identity fence in the analysis Worker history. Rust currently resets on config/epoch/sequence changes, and per-entry capture counters are not global selection identity. Astra medium must verify and correct same-mask/smoothing target-history behavior, along with ordinary endpoint verification, without scope expansion. Final all-browser qualification and package/static gates remain required.

Astra medium verification correction tranche: first managed-stream admission now initializes the existing Worker and reusable buffer before native start. Collection selection uses the existing deadline; ambiguous timeout retires only the spectrum lifetime and holds cleanup until the late transaction settles/stops before replacement. Focused regression proves preflight, timeout, stale-handle refusal, blocked early replacement, one late stop and fresh lifetime with the same Worker;8/8 browser-focused tests and types pass (/tmp/issue793-review-browser-focused.log, /tmp/issue793-review-types.log). Astra disproved the ordinary dropped-B history concern: a new entry starts at sequence0 and its one-slot queue preserves that first record, forcing the existing history reset. No speculative target fence was added. Whole-endpoint review and final qualification continue within attempt1; no PASS yet.
