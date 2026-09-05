# Prepare paused PCM seeks before audible resume

## Outcome

Provide the narrow SDK-owned PCM consumer preparation needed for a browser host to seek while its AudioContext is suspended, refill current-generation PCM, and resume with correct target PCM on the first quantum. The SDK owns ring layout, consumer indices, epoch acknowledgement and worklet processing. The web adapter owns session readiness; applications must not manipulate ring internals.

## Concrete defect and baseline

App issue misofm/app#101 integrates SDK445 source 11c52271e1f686d15eeee6fce90107fcdec50b5e and adapter34 source 6d448ea8a2507fcabe062e0e52acb833bdd78588. During paused resume the producer publishes a new generation while old ring slots remain full. App occupancy incorrectly accepts those slots as fresh prefill. After unmuting, the worklet discards them before replacement PCM arrives. Actual eight-stem Ghost evidence records 512 stale slots discarded and 16 underruns between pause and resume. Independent Astra review confirms this is a correctness gap; initial attachment-ready is not post-seek readiness.

Preserve the original dirty engine checkout. Work from the exact reviewed SDK445 checkpoint in an isolated branch. Do not include unrelated later engine work.

## Smallest slice and first decision gate

Before freezing a new public API, use existing actual-Wasm PCM parity fixtures and the existing browser PCM evaluator to prove the proposed bounded, consumer-owned suspended preparation mechanism. Fill both the old internal engine queue and all old shared-ring slots, publish a new seek, prepare on the owning worklet control port while the context stays suspended, refill, then require exact target PCM on the first resumed quantum without an underrun. Confirm the control-port handler actually executes while suspended.

A source-seek admission ACK is insufficient: Rust PcmSourceProducer.try_seek queues work consumed by render and may return backpressure. The proof must discriminate old internal queue state and engine-applied seek semantics. Stop and report if the message-only mechanism fails; do not silently render/discard a quantum, move the target frame, hide underruns with mute, or weaken the first-quantum assertion.

If that gate passes, implement one bounded awaited public feed preparation operation using the existing control port and shared seek/stale handling. It releases stale slots only through their owning consumer, retains current-generation work, and reports typed failure/backpressure with explicit close, timeout and supersession behavior. No allocation, lock, I/O, logging or unbounded work is added to process/render. A Rust change, if the decisive proof requires one, needs an amended decision record and review before implementation.

## Expected boundary

SDK browser feed, SDK-owned PCM worklet prelude, public types/exports if needed, existing SDK PCM tests/browser evaluator and this spec. Adapter consumption and app archive adoption are bounded successors under app#101. No new storage backend, progressive playback, codec, generic transport framework, benchmark framework, or unrelated architecture work.

## Acceptance

- Existing real-Wasm first-quantum discriminator is RED on the old behavior and GREEN on the final behavior with old queues full.
- Suspended-context control delivery and preserved target frame are demonstrated in the existing browser gate.
- Current-generation PCM proof is distinct from ring occupancy and initial attachment; stale or superseded preparation cannot grant readiness.
- Close and timeout reject outstanding preparation; typed queue refusal is preserved, never acknowledged before a drop.
- Proportional existing SDK type/headless/package/browser checks pass; no unnecessary full engine rebuild if Rust/engine assets are unchanged. Modified SDK-owned feed bytes receive accurate package provenance.
- Dedicated Astra medium review records PASS before dependent adapter implementation. Root checkpoints exact paths, pushes and synchronizes this GitHub issue; completion requires upstream evidence.

## Execution

User-selected Astra medium implements and a separate Astra medium agent reviews. Root approves this bounded contract; mechanism remains conditional on the first decisive proof. Detailed read-only diagnosis: /private/tmp/dx101-paused-resume-readiness-brief.md. Actual failure evidence: /private/tmp/dx101-ghost-32db3f2.json. No implementation has started.

## First decision gate: message-only preparation blocked

Astra medium ran a bounded actual-Wasm probe using the existing SDK sessionDocument fixture and WasmBoundary, with the reviewed SDK445 artifact from the writer checkout. The bytes compare exactly with the app's installed reviewed module; SHA256 `22e4c25cba7f97b66db720ad8ac8cf653de0afcabe84101693f4fa166b90d4e6`. No Rust rebuild or production modification.

The one-source 48 kHz/128-frame fixture uses a 512-frame internal PCM ring. Four old-generation quanta are accepted; the fifth returns typed backpressure (6), establishing a full internal queue. All 64 shared-ring slots are also filled with old-generation work, then the producer publishes generation 2 at target frame 10000. Actual engine seek admission returns OK, but direct submission of the new target quantum still returns backpressure (6). The first actual render returns zeros; a fresh same-document engine with the same seek and target PCM returns the exact nonzero ramp (left starts 0.00390625, 0.0078125, 0.01171875). Exact first-quantum equality is RED.

Probe `/private/tmp/dx457-first-quantum-probe.mjs`, output `/private/tmp/dx457-first-quantum-probe.log`; command `node /private/tmp/dx457-first-quantum-probe.mjs` exits 1 at the first-target assertion. This narrower ABI probe does not claim worklet control-port delivery or a complete browser gate. It establishes a prerequisite failure even if stale shared slots were ideally released: the full internal queue cannot accept target PCM until render consumes its pending seek. Consequently no public preparation API or worklet implementation was frozen, no output was silently rendered/discarded, and no counters or target frame were changed. Execution stops at the brief's decision gate. A reviewed amendment is required before any Rust/internal-consumer change or alternative mechanism.


## Approved Rust consumer preparation amendment

Root approves the following bounded amendment after independent Astra medium review. The failed SDK-only prerequisite is preserved above; first-quantum, realtime and ownership gates remain unchanged. Implementation may now start the Rust tranche only, checkpointing its green decisive proof before SDK control-port work.

# SDK457 amendment proposal: prepare the existing web source seek on its owner

The retained actual-Wasm RED probe proves the SDK-only mechanism cannot satisfy the frozen first-quantum contract. Amend457 to include the minimal internal consumer preparation below; keep the SDK control-port/readiness work and all original gates. No production implementation has begun.

## Smallest first implementation tranche

Strengthen the existing web host's `seek_source` operation so its successful ACK means the admitted seek has also been applied to its exclusively owned source consumer and stale internal transfer blocks have been recycled. Keep the existing web ABI `miso_engine_web_v1_source_seek(handle, id_bytes, generation, frame)` and result codes. No new control-schema record, native transport, source command, or public C API operation is needed. This changes web-host acknowledgement timing, not the target frame or PCM format. Native/C API producers retain their existing queued-seek semantics.

`AudioWorkletEngineHost` owns `PreparedHost.plan` and invokes it through `&mut self`, on the same worklet owner that executes render. Its source-seek message handler and process callback cannot run simultaneously. Forward a narrow internal `prepare_source_seek(source_index, expected_generation, expected_frame)` operation through the plan's existing executor and graph source-set driver. No alias, shared consumer handle, lock, downcast, or arbitrary control-thread access. The method requires exclusive plan ownership between render blocks; it does not arm native concurrent control access.

In `PcmSourceConsumer`, reuse the non-consuming prefix of `begin_block` (source/lib.rs1106): finish/recycle any retained played block, flush deferred recycling, apply pending admitted seek command(s), and acquire/recycle stale queued blocks. Stop before the branches that advance next_frame, cumulative_read_frames, underrun counters or played_frames. Retain any matching current-generation block. Bound command work by its prepared queue capacity and data work by transfer_block_count. For an exact requested generation/frame, process older already-admitted seeks through that request; do not acknowledge an older/superseded generation as the requested one. Confirm the resulting active generation and next_frame before returning success. No render_next, DSP processing, output writes, absolute-clock advance or target offset.

The recycle queue and preallocated transfer blocks already provide ownership return. Do not copy PCM, allocate replacement buffers, overwrite a deferred Box, drop/free a transfer block on the worklet, or introduce an unbounded drain. Explicitly test full data/recycle/current/deferred states against the existing allocation/free observer. Normal begin_block behavior remains unchanged apart from sharing the extracted prefix where appropriate.

Web seek validation and producer admission remain in SourceControlSet.seek, in their current order. Only after admission succeeds does the owning web host prepare the corresponding canonical source consumer. Source index comes from the host's compiled canonical source order, already used for shape reporting. Prevalidate the internal source capability/index before admission so a missing dispatch cannot yield an admitted producer mutation followed by an unrelated capability refusal. Unexpected generation disagreement must fail honestly; never claim prepared readiness after a partial/mismatched operation.

## Exact expected paths

- `crates/source/src/lib.rs`: consumer preparation and SourceGraphSourceSetDriver forwarding; existing source tests including no allocation/free.
- `crates/engine/src/realtime/plan.rs`: narrow executor trait method and exclusive PreparedRenderPlan forwarding. Default unsupported result for plans without streamed sources.
- `crates/graph/src/lib.rs`: source-set driver/GraphExecutor forwarding. No graph topology or schedule changes.
- `hosts/host-web/src/lib.rs`: existing seek_source invokes prepared consumer operation after admission; existing host tests in `hosts/host-web/src/tests.rs`.
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` / corresponding existing documentation only if its queued-ACK description needs correction; no signature change. Existing ABI/generated asset machinery regenerates affected provenance normally after the required Rust Wasm rebuild.
- Existing SDK457 paths: `sdk/src/browser/pcm-feed.ts`, `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`, `sdk/test/browser-pcm-evals.mjs`, public export/type file only if needed, numbered spec. Any exact generated paths follow existing generator output, not manual pin edits.

No host-core/source producer rewrite is anticipated. If forwarding proves to require another owner or a public ABI addition, report the exact dependency before expanding this list.

## Decisive proof and checkpoint order

First tranche is only the Rust consumer/owner forwarding and the existing actual-Wasm RED discriminator translated into the existing SDK PCM evaluator. Fill the internal queue to actual backpressure and all64 shared slots; admit the paused seek, prepare it without rendering, accept target-generation PCM, and require the first render's two planes to equal the fresh same-document/target oracle exactly. Assert status nextAbsoluteSample/renderedQuanta and source cumulative-read/underrun state did not advance during preparation. Include zero/current/stale block and bounded queued-seek coverage with allocation/free proof. Run proportional Rust tests and required updated Wasm artifact checks; pause exact-path coherent checkpoint as soon as this first-target proof is GREEN.

Then the SDK control operation can safely apply the same existing source_seek on its owning worklet port, release old SAB slots, acknowledge exact epoch/generation, and permit fresh refill. Preserve close/timeout/supersession/backpressure as in457. Prove actual suspended-context port delivery and exact first target PCM in the existing browser gate. Only after reviewed SDK PASS does the separate adapter successor await this stronger preparation plus fresh prefill before paused seek readiness; app consumes that public promise.

This proposal requires dedicated review and a synchronized457 amendment before implementation. It preserves the stop condition: if first-target proof still fails, report the new concrete cause, never add an output-discard/mute/silence workaround.

## Rust preparation tranche evidence

Implemented after reviewed amendment20ceb425. Existing web seek now prevalidates source/consumer dispatch, admits the producer seek, and applies it through the exclusive plan/executor/graph driver. Consumer preparation reuses bounded seek observation and stale acquisition/recycling, without consuming target PCM or advancing read/underrun/time state. Native producer command admission is unchanged. A second web seek can now succeed before render because the previous acknowledged command was consumed; the existing web unit assertion was translated accordingly, while native source queue backpressure tests remain intact.

Root authorized the exact existing `hosts/host-web/tests/boot_transient_budget.rs` path for allocation/free proof using its allocator, with two operation counters added to that observer. Full old internal queues, an intervening render/refill, and repeated seeks report zero allocations, zero frees and unchanged clock/quanta. Source tests also cover retained stale/current blocks, mismatched generation refusal and exact next target output. No new allocator framework.

Rust library tests for engine/graph/host-web/source PASS:194 passed, one existing ignored; focused allocation test PASS; existing SDK PCM evaluator9/9 PASS. Formatting and diff check PASS. Logs `/private/tmp/dx457-rust-{focused,tests}.log`, `/private/tmp/dx457-allocation.log`, `/private/tmp/dx457-pcm-focused.log`.

The same added actual-Wasm assertion in `sdk/test/browser-pcm-evals.mjs` is RED against reviewed SDK445 (`/private/tmp/dx457-wasm-first-red.log`: target admission backpressure, first planes zero) and GREEN against this tranche (`/private/tmp/dx457-wasm-first-green.log`: exact first left/right target planes). Both internal and shared old queues are full before seek; the test explicitly distinguishes direct Rust admission proof from the pending SDK shared-ring/control-port mechanism. No hidden render or target-frame offset.

For this decisive proof only, root authorized the existing build script's exact Rust flags with persistent target `/private/tmp/dx457-wasm-target`: `CARGO_TARGET_DIR=/private/tmp/dx457-wasm-target RUSTFLAGS="-C target-feature=+simd128 -C strip=debuginfo --remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=/cargo --remap-path-prefix=/private/tmp/miso-dx-sdk-resume=/repo" cargo build --locked --release --target wasm32-unknown-unknown -p host-web`. Build PASS, log `/private/tmp/dx457-wasm-build.log`. Provisional artifact `/private/tmp/dx457-probe-artifacts/miso-engine-v1-audio-worklet.simd128.wasm`, SHA256 `fa0039d8119ce34efd2c1a5b6540252b4a27a36bb1fe1535a5efbd838506ed4c`. Existing release pin/generated package assets were not manually changed. Final canonical artifact build/promotion, SDK control-port implementation, real suspended-browser proof and package qualification remain pending. Pause this coherent Rust tranche for root's exact-path checkpoint before further implementation.

## SDK consumer control-port tranche after cad8b6db

Public `EngineFeed.prepareSeek({ timeoutMs? })` is separate from initial attachment-ready and from producer prefill. It requires an attached feed and suspended context, permits one outstanding request, and sends captured full generation/frame/epoch identities over the existing attach port. Worklet preparation uses the strengthened web seek, releases only provably older shared slots and retains current/future PCM. The ACK must match both the request and live identity, with the context still suspended. Concurrent calls reject `prepareBusy`; supersession, wrong state, actual engine refusal (including numeric result6), close and timeout remain distinct typed failures. Timeout closes the feed. No new ring layout or rendering operation.

Independent review caught two races during this implementation pass: a superseded request must not discard future-generation slots, and control preparation must apply its captured tuple rather than mix a captured epoch with freshly read generation/frame words. Both are corrected and discriminated in the existing PCM evaluator. Steady render retains lazy generation/frame reads only on a changed epoch; typed-array allocation mutation remains RED in the existing gate. No new framework.

Publication precondition is explicit in public documentation: await the producer seek ACK, then serialize the synchronous prepareSeek snapshot/post against any other producer seek commands. Shared generation/frame words precede the epoch publication, so an already in-progress producer publication is not a valid input snapshot. The existing adapter Worker ACK/lifecycle queue supplies this ordering. Later seeks may supersede a pending request and are handled by the captured-identity checks above; this does not introduce another transport or queue.

Existing PCM evaluator12/12 PASS, including actual-Wasm full internal/SAB queues through the real prelude/control handler, exact first target planes, unchanged preparation sample/drain/underrun state, fresh-slot retention, both supersession races, bounded concurrent admission, typed refusal, close and timeout. SDK types, syntax and diff check PASS. Logs `/private/tmp/dx457-port-{pcm,types,build,stage}.log`.

The existing `sdk/test/package-tarball-smoke.mjs` browser gate now includes one paused-seek case, retaining both original boot/factory cases. Its test-only first-quantum recorder observes actual engine output; it is not a production processor or SDK dependency. Using a provisional package with the honestly identified local Wasm above, Vite/Chromium PASS: preparation completes while context.state remains suspended, AudioContext time and engine sample clock unchanged, occupancy0 after64 stale slots; the first resumed two planes exactly equal a fresh same-document/target Wasm oracle. After capture: underruns0, refused0, torn0, errors0, seeksApplied1, submittedGenerationTag2. Evidence `/private/tmp/dx457-port-browser.json` and `/private/tmp/dx457-port-browser-corrected.log`; the tested packed feed bytes compare exactly with current source.

The first browser invocation hit sandbox localhost EPERM before launch; authorized escalation followed. The first actual browser invocation exposed missing required frames/sampleRateHz in the new test's public host submission, corrected in that same existing test before the passing run; no production workaround. An initial npm pack cache permission failure used a task-owned cache on retry. Canonical Linux artifact promotion, final package qualification and final dedicated457 verdict remain pending; this provisional browser result is not a release-package PASS. Pause these five exact paths for root checkpoint before pin promotion or further implementation.

## Canonical Linux artifact identity and source review

Dedicated Astra medium review passes the Rust tranche at cad8b6db and the SDK control-port tranche at 30aa7009. Reports /private/tmp/dx-457-astra-rust-tranche-review.md and /private/tmp/dx-457-astra-port-tranche-review.md include independent first-target and supersession checks. This is source acceptance, not final package or adapter acceptance.

Existing qualification run 33959847637 on exact cad8b6db8370fa67da7e8549bfb4fec4e738921f used Ubuntu 24.04.4 (image 20260831.293.1), Rust 1.97.1 x86_64-unknown-linux-gnu. Its artifact job 101289741547 observed WASM SHA256 271a2bf3c8cf52f5156dadec091efd399324cd3f0c51aa7b4a2a08e632a648ce and correctly refused the old 22e4c25 pin, uploading nothing. All independent native/debug/release/audit/cross-target/wasmtime/lint/docs jobs passed; SDK/browser/artifact consumers were skipped after that expected failure. The overall run is FAIL, not qualification PASS. Raw evidence /private/tmp/dx457-linux-33959847637-artifact.log and identity record /private/tmp/dx457-linux-33959847637-identity.md.

Update only the authoritative source pin to that actually observed Linux digest. The Rust source is unchanged by 30aa7009; no provisional Darwin artifact is relabeled, no generated payload or old package provenance is patched. The normal existing qualification workflow must now re-earn the pin and upload its six-file closure before canonical SDK packaging. Final package/browser qualification and reviewed archive remain pending.

## Canonical qualification and bounded deletion-scan correction

The existing Ubuntu artifact job101291838905 in run33960625717 on facce76e4ed4218e5581e7ecbf736879b7470d18 passed and uploaded artifact9967841154. Its six-file closure is retained at `/private/tmp/dx457-canonical-artifacts`, with provenance `/private/tmp/dx457-canonical-artifacts-provenance.json`. The 2637943-byte Wasm has the pinned SHA256271a2bf3c8cf52f5156dadec091efd399324cd3f0c51aa7b4a2a08e632a648ce. Local existing headless check with that explicit directory passed167 tests with one existing skip; package check with the same directory and existing Chromium tools passed, including actual suspended-context preparation and exact first resumed target planes. Logs `/private/tmp/dx457-final-headless.log` and `/private/tmp/dx457-final-package-browser.log`.

CI separately exposed deletion-scan spelling collisions. The internal preparation reply kind is now `confirmed` in the feed, prelude and existing PCM tests; public prepareSeek behavior, message operation and every assertion remain unchanged. The same scan then exposed the pre-existing scratch test's local `phase` variable over handshake/request, incorrectly matching its error-phase pattern. Rename only that loop variable to `scratchStage` in the existing `sdk/test/browser-defaults-evals.mjs`; retain all cases and names. This exact additional test path is a proportional CI correction, not a boot contract change. The checker is unchanged. Existing deletion scan now PASS, PCM plus defaults31/31 PASS, TypeScript and diff check PASS: `/private/tmp/dx457-final-deletions-green.log`, `/private/tmp/dx457-final-port-defaults.log`, `/private/tmp/dx457-final-types.log`.

These local package/browser results precede the internal reply spelling correction; the final corrected package must be checked before release. The same CI run's browser jobs refused the old recorded Wasm lineage before executing browser assertions. That historical matrix cannot be relabeled: its refresh requires the existing actual Linux qualification recording workflow. Final CI, corrected package qualification and reviewed archive remain pending. Pause this coherent five-path correction for the root checkpoint.
