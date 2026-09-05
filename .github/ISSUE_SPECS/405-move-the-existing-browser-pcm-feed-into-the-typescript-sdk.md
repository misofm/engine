# Move the existing browser PCM feed into the TypeScript SDK

**Issue:** https://github.com/misofm/engine/issues/405
**Scope:** Sol-approved; the matching numbered issue/spec is synchronized before Luna implementation. Astra reviews the implementation PR.
**Engine baseline:** `b89b767d`, with product code and required regressions reviewed PASS by Astra at `bed7634c` for #393 (PR #398).
**Copied current adapter baseline:** `63b4ee6212287000ff85e1cfa969d385f6246d2d`; source ring/feed/prelude remain unchanged by independent OPFS #19.

## Scope decision

Split SDK boot defaults from PCM ingress. This issue moves only the existing engine-specific PCM ingress contract: the MSB1 shared-ring layout/writer, feed attachment lifecycle, and AudioWorklet prelude asset. The adapter remains responsible for verified storage, FLAC decode, pump scheduling, prefill, seek orchestration, and session lifecycle. SDK scratch/default-host construction remains injected exactly as it is today and belongs to a later issue.

This is independently useful: one SDK version becomes the authority for both sides of the engine's existing browser PCM boundary, while any codec-neutral producer can fill its bounded rings. It adds no playback capability and does not change the Rust ABI, host messages, session schema, or all-stems-ready policy.

Because delivery crosses repositories, root should create two linked stateless specs after approval:

1. Engine/SDK authority and packed asset PR.
2. Adapter consumer migration PR, based on an exact locally packed artifact from the engine PR.

The engine issue may close once its package and focused gates pass. The adapter issue cannot claim registry compatibility or close as deployable until the matching SDK version is published and its exact dependency/provenance can truthfully be updated.

## Frozen contract

- Preserve MSB1 magic/version, control/header offsets, field meanings, ring sizing, planar `f32` slots, power-of-two capacity, generation/seek publication, counters, backpressure retention, scalar tail behavior, and quantum-sized frames byte-for-byte.
- Preserve the existing prelude behavior: load before the engine worklet; wrap the registered engine processor; drain without steady-state allocation or waiting; submit through existing `miso_engine_web_v1_source_submit`/seek exports; retain a slot on result 6; drop stale generations; attach rings through the separate `miso-sab-feed-attach` port.
- Preserve `prepareEngineFeed`/`attachEngineFeed` behavior: default capacity 64, exact source ID/channel inputs, injected node factory, bounded ready timeout, attachment probe, idempotent close, writer release, detach, and disconnect.
- The SDK API is codec/storage neutral. It accepts only existing source ID, channel count, quantum, capacity, context/module URL, and node-factory inputs; it returns the existing rings/feed lifecycle and ring writer primitives. It must not accept Blob/File, FLAC, OPFS, hashes, locators, decoded-source metadata beyond ID/channels, or pump policy.
- SDK compilation currently uses ES/WebWorker libraries without the DOM global library. Define the feed's context, worklet, node, node-options, and port requirements as the smallest structural interfaces needed by the implementation. Keep injected browser factories assignable from real `BaseAudioContext`/`AudioWorkletNode` consumers without changing the SDK `tsconfig` global libraries or importing adapter types. Browser globals may be reached behind runtime feature checks/casts in the default branch; do not broaden the entire SDK type environment to DOM.
- Keep adapter public customization working: `feedWorkletModuleUrl`, `createAttachNode`, and `createPump` retain their current meanings. The adapter may keep a thin compatibility facade that delegates to SDK PCM ingress and translates SDK failures into the same existing adapter error codes/messages where tests establish that contract. It must contain no copied ring arithmetic or feed state machine.
- The adapter's pump worker remains adapter-owned but imports the SDK ring writer/size authority. The adapter `./stems` export may re-export the existing `MSB1_CONTROL` and counter type from the SDK so its current public surface does not break.
- Keep verified-all, stored-all, then prefill-before-play ordering unchanged. No progressive/no-storage path.
- Freeze one narrow SDK-native failure type in `pcm-feed.ts`, with a stable operation discriminant limited to `moduleLoad`, `nodeCreate`, `attachPost`, `readyTimeout`, and `closed`. Invalid caller shape continues to use the SDK's existing `MisoUsageError`/range conventions. The adapter facade maps the operation discriminant directly: module load to existing `capability.audio_worklet`; node creation, attach-post failure, and readiness timeout to existing `session.open`; closed readiness to existing `session.closed`. It must not parse messages, import adapter errors into the SDK, mint an ABI reason, or create a general error system.
- Preserve bounded terminal behavior at the moved seam. The baseline defects are confirmed: if initial attach `postMessage` throws, the SDK must release every owned ring's writer state and disconnect the newly owned node before rejecting; if `close()` wins while `ready()` is pending, `ready()` rejects promptly through the frozen `closed` operation rather than waiting for its deadline or resolving falsely ready. Timeout closes before rejecting. These corrections stay inside the existing feed lifecycle and add no cancellation API.

## SDK asset decision

Stage the prelude source JavaScript as a normal SDK-owned package asset beside the existing browser assets and expose its package-relative URL through `BUNDLED_ENGINE_ASSETS`. Preserve its provenance header from adapter `63b4ee6` and the earlier source attribution already present in the file.

Attribute the moved TypeScript ring/feed sources to adapter `63b4ee6` and preserve the adapter's recorded earlier provenance. Update the engine repository `NOTICE`, then have the existing stage script copy that notice into `sdk/dist/NOTICE`; package smoke must assert the installed SDK carries it. Attribution applies to source and packed output, not only the prelude header.

Do not add this hand-maintained JS file to the Rust-generated ABI artifact authority or regenerate Rust manifests. `sdk/codegen/stage-package.mjs` may copy it from an SDK source-asset directory after staging the unchanged engine artifact closure. Package smoke must prove the new file is present and addressable. The engine Wasm, generated host JS/declaration, ABI layout, parameter metadata, and their hashes remain unchanged.

## Exact allowed paths

### Engine / SDK authority PR

- `.github/ISSUE_SPECS/405-move-the-existing-browser-pcm-feed-into-the-typescript-sdk.md`
- `sdk/src/browser/pcm-ring.ts` (new; name may be `ring.ts` if root prefers)
- `sdk/src/browser/pcm-feed.ts` (new; name may be `feed.ts`)
- `sdk/src/browser/index.ts`
- `sdk/src/assets.ts`
- `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js` (new source asset; exact directory may follow existing SDK convention)
- `sdk/codegen/stage-package.mjs`
- `sdk/test/browser-evals.mjs` or new focused `sdk/test/browser-pcm-evals.mjs` (matched by the existing headless eval glob)
- `sdk/test/console-types.ts` or a new focused `sdk/test/browser-pcm-types.ts`
- `sdk/test/package-tarball-smoke.mjs`
- `scripts/sdk-package.sh` only if its current staged-source allowlist requires the new normal SDK asset
- `sdk/README.md` and repository `NOTICE` only for the new public export/provenance obligation; `sdk/codegen/stage-package.mjs` owns staging `NOTICE` under `dist/`

No `crates/`, `hosts/host-web/src`, generated host/worklet files, Wasm, ABI/parameter JSON, or generated provenance TypeScript may change.

## Objective gates

### SDK authority

1. A layout fixture creates representative mono/stereo rings and proves every existing byte length, offset/control word, source-ID encoding limit, slot view, generation/seek field, counter, and invalid-layout refusal equals adapter baseline. Use frozen expected values or a copied test fixture, not runtime comparison against retained duplicate production code.
2. Writer tests preserve reserve/commit/backpressure/zero-fill/seek/release behavior, including odd track/source counts and a final partial quantum. No whole-source allocation is introduced.
3. The existing feed-prelude oracle is moved to the SDK and proves submit, seek retry on result 6, stale-generation rejection, attach/detach, and no first-use/steady-drain typed-array or subview allocation. Run the existing realtime source-policy gate if it accepts this asset; otherwise add only a focused discriminator for the same rules.
4. Feed lifecycle tests prove injected node precedence, exact attach message/ring identity, empty-source readiness, bounded timeout closes/releases/detaches once, successful readiness, and repeated close safety. They reproduce the confirmed attach-post cleanup and close-during-ready defects: attach-post failure releases/disconnects before the typed SDK rejection; close makes every pending readiness call reject once with the `closed` operation. No node or ring remains active after failed attachment.
5. `BUNDLED_ENGINE_ASSETS` resolves the staged prelude from a freshly packed SDK. The packed consumer imports PCM ingress from `@misofm/engine/browser`, attaches with defaults and with the injected node/module seam, and contains no adapter dependency. The tarball contains `dist/NOTICE` with the moved-source attribution.
6. The package manifest continues to describe the unchanged generated Engine artifact closure truthfully; the normal prelude asset is separately staged and checked. Wasm, generated host/worklet JS, host declaration, ABI JSON, and parameter JSON are byte-identical to baseline.
7. Run `scripts/check-sdk-types.sh`, `scripts/check-sdk-generated.sh`, `scripts/check-sdk-headless.sh`, `scripts/sdk-package.sh check`, and the focused browser/package/tarball evals. No full Rust workspace rebuild is required beyond what these repository gates invoke.

## Explicit exclusions

- Default scratch worker, default host import/construction, default `AudioContext`, or a new top-level browser engine object. Those form the next independent boot-default issue.
- Request counters/brokers or any `run(requestId)` API. Issue #393 makes the host the sole allocator.
- Console receipts, coalescing, meters/telemetry behavior, readback, revisions, automation, agent APIs, structural replacement, progressive playback, alternate storage, or nonisolated transport.
- Rust, C ABI, worklet-host wire messages, source introspection, session parsing changes, and new ABI error vocabulary.
- Adapter error types inside the SDK. SDK failures use the SDK's existing usage/engine error conventions; the adapter facade performs only the narrow translation needed to preserve its established public errors. No general error subsystem is added.
- npm publication. These PRs produce local packed integration evidence and an explicit release-order blocker.

## Review and delivery

Root approves and numbers both linked specs before their respective implementations. Implement the engine SDK authority first and checkpoint it. Astra reviews that PR before the adapter migration starts. The adapter implementation then consumes the reviewed exact tarball; it must not overlap active issue #19. Each repository follows its own checkpoint and three-attempt rules. Success is two reviewable PRs with truthful local packed integration evidence, not a registry release claim.

## Root implementation decisions

This numbered issue owns SDK authority only; adapter production changes are forbidden here and will have their own linked spec after SDK review. `prepareEngineFeed` defaults its optional module URL to the packaged PCM prelude; the explicit URL still wins. Use the concrete names `pcm-ring.ts`, `pcm-feed.ts`, and `miso-engine-v1-pcm-feed-worklet.js` above. The narrow feed-operation failure type frozen above is allowed; broader errors remain excluded.

Reuse the exact pinned CI artifact closure already qualified for #393 at `/private/tmp/dx-393-current-artifacts`; all six generated artifact bytes must remain unchanged by this issue. Darwin's independently reproduced baseline digest mismatch is documented in #393/#333/#345 and must not cause a repin.

Implementation starts only after the current OPFS tranche is checkpointed. Implement all minimum focused regressions for this move in the same coherent tranche before the first focused-green handoff; root commits exact paths before any further implementation. A pending gate is explicitly pending, never PASS by inference. Existing PR #398 may carry this bounded SDK issue alongside the separately completed #393; Astra reviews the new issue diff and final combined PR. No npm publication is authorized.

## Luna attempt 1 — source checkpoint, review pending

Luna moved the current ring/feed/prelude into the approved SDK files, exported the codec-neutral primitives and asset URL, staged the separate prelude and NOTICE, and reported passing TypeScript/build/package checks against `/private/tmp/dx-393-current-artifacts`. The six generated engine artifact files remain unchanged. This is a recoverable source checkpoint only: required focused ring/feed, allocation, DOM-consumer and packed-consumer regressions remain pending, as does Astra review. A direct browser-eval invocation omitted its artifact environment; use the existing `scripts/check-sdk-headless.sh ARTIFACT_DIRECTORY` wrapper for proper qualification. No PASS or completed SDK ownership claim is made yet.

## Luna attempt 1 — focused test checkpoint

Added the focused PCM eval file and packed-consumer asset/export/NOTICE assertions. Luna reports `scripts/check-sdk-headless.sh /private/tmp/dx-393-current-artifacts`: 137 pass, one platform skip, zero failures; package check passes with the separate prelude and NOTICE and unchanged six generated artifacts. Dedicated Astra review remains pending and will assess whether every frozen lifecycle, layout, allocation and real DOM-consumer discriminator is complete. No review PASS is inferred from the suite count.

## Astra attempt 1 verdict — FAIL (2026-09-05)

Dedicated Astra reviewed pushed `a86f3cf4`. Required corrections: close must reject every pending ready caller even when its injected wait is blocked; restore the original monotonic timeout clock; preserve ID/channels source inputs and real DOM context/factory assignability; complete the frozen ring, lifecycle, runtime prelude allocation and packed consumer regressions; preserve earlier source provenance in NOTICE. No prelude algorithm change is warranted: Astra independently verified copied bytes, bounded submit/seek/backpressure behavior and zero instrumented render allocations. Six generated artifacts remain identical. Advertised headless (137 pass, 1 skip) and package gates passed but do not satisfy these missing contracts.

This is the first failed implementation attempt. The full review is attached to PR #398. A separately briefed attempt 2 must keep this issue's product scope unchanged. PR #398 also has independent required CI failures (environment vocabulary and a pre-existing real-clock telemetry assertion), which are not authorized implementation paths in this issue. No merge or release readiness is claimed.

## Attempt 2 authorization (2026-09-05)

Sol supplied the bounded revision brief and root approved it after the adapter qualification attempt stopped and its evidence was checkpointed. Luna implements one coherent second attempt; dedicated Astra reviews the exact pushed result. The original contract and gates remain frozen.

## Required product corrections

1. In `sdk/src/browser/pcm-feed.ts`, make the feed source input exactly the existing public input shape: source ID and channel count. `attachEngineFeed` derives `frameCapacity` from `quantumFrames` and ring capacity from `capacityChunks ?? 64`; callers must not provide `Msb1RingLayout` or its `frameCapacity`/`capacity` fields.
2. Keep the SDK free of DOM globals while preserving real browser assignability. Parameterize the feed options/function over the caller's structural context (or use an equivalently narrow context-preserving signature) so a `BaseAudioContext` and an existing factory typed `(BaseAudioContext, string, AudioWorkletNodeOptions) => AudioWorkletNode` pass strict consumer typechecking. `FeedNodeOptions` may remain the minimal number-of-inputs/outputs shape because that value is assignable to `AudioWorkletNodeOptions`. Do not add `DOM` to `sdk/tsconfig.json`, import adapter types, require consumer casts, widen the SDK with unrelated browser interfaces, or add a second factory API.
3. Make `close()` synchronously signal every `ready()` call already pending. Two callers blocked in injected waits must promptly reject once with `PcmFeedError.operation === "closed"`; they cannot wait for their own timer/wait promise to settle. A small feed-private terminal promise/notification raced with each wait is sufficient. Do not expose cancellation, a signal, a pending-call registry, or another public state.
4. Preserve timeout ownership: the caller that observes its deadline closes the feed and rejects with `readyTimeout`; other pending callers awakened by that close reject with `closed`. Release every ring, attempt one detach, and disconnect once. Repeated close remains safe. Restore the adapter baseline's monotonic default `performance.now()`; retain injected `now`/`wait` only as the existing test/composition seam.
5. Preserve the working attach-post cleanup. If initial attach publication throws after the port has observed and engaged the rings, all writer states are zero before disconnect and the caller receives `attachPost`, even when detach and disconnect also throw. Cleanup exceptions must not replace the typed primary failure.
6. Expand repository `NOTICE` to retain all relevant lineage already recorded by adapter `63b4ee6212287000ff85e1cfa969d385f6246d2d`: the MSB1 writer came from the authorized `misofm/engine` baseline `bd7f330a9773ce43bb077f0e6d5c8fc30fe9e27c`; the MSB1 allocation/reader contract and AudioWorklet feed prelude/attach mechanism came from authorized `misofm/app` `7485693e9bbcf2f65a91a4e5950e22d678d99062`; this SDK move came through adapter `63b4ee6`. Keep the prelude header intact and prove staged `dist/NOTICE` contains the exact source identities. Attribution must cover the TypeScript ring/feed sources as well as the JS asset.

No rewrite of `pcm-ring.ts` or the prelude is expected. If focused evidence exposes a real mismatch with the copied `63b4ee6` contract, stop for a scope amendment rather than hardening or redesigning it opportunistically.

## Required focused regressions

Use the existing `node:test`/SDK eval setup. Port the current adapter behavior into `sdk/test/browser-pcm-evals.mjs` where it is part of the moved contract. In particular, use `/private/tmp/dx-405-astra-prelude.mjs` and adapter `tests/feed-prelude.test.ts` at `63b4ee6` as concrete verified fixtures; make paths repository-relative and consolidate helpers instead of creating another runner or framework.

### Ring and writer boundary

- Freeze literal mono and stereo examples: total byte length, 128-byte control area, ID offset/capacity, 256-byte slot-header offset, calculated PCM offset, and every initialized control word. Decode and compare the source ID bytes, including a multibyte ID; accept exactly 128 UTF-8 bytes and reject 129, proving the boundary is bytes rather than JavaScript characters.
- After `engage`, `reserve` and `commit`, inspect the actual control words, slot `Int32`/`BigInt64` headers and planar PCM offsets. Assert sequence, signed generation tag, full generation, start frame, frame count, end-of-region flag, write index and wrote counter. A final partial quantum must leave its unwritten lane samples at positive zero.
- Fill a capacity-two ring, prove the next reserve returns `null`, increments overflow exactly once, and leaves queued slot/header/PCM data and write index intact. Advance the public shared read index as the consumer would, reuse the wrapped slot, and prove reserve zero-fills both mono/stereo planes before reuse.
- Assert seek publication (`generation`, frame, generation tag and epoch), release, occupancy across wrap, and the public constructor's existing malformed-buffer refusals (non-shared input, wrong magic/version, invalid power-of-two/zero shape). Do not invent new validation semantics or copy the adapter reader into SDK production.

### Feed lifecycle

- Prove default and explicit module URLs and typed `moduleLoad`; prove typed `nodeCreate` and that an injected factory wins even when a usable default `AudioWorkletNode` is present.
- Capture the exact `{ op: "attach", rings }` message. The array must contain the same ring objects returned by the feed, in source order; use three mono/stereo/mono sources to cover an odd count. Assert derived quantum/capacity control words and default capacity 64.
- Prove empty-source readiness and ordinary successful readiness by publishing `ATTACHED` on all captured rings. Neither case waits past its required state.
- Reproduce Astra's blocked-wait case exactly: start two `ready()` calls whose injected waits never resolve, observe both entered, call `close()`, and bound the observation with the test harness. Both reject `closed`; neither resolves later. Writer release, one detach and one disconnect are asserted. Repeated close changes none of those counts.
- For timeout, use the injected deterministic clock. The timing-out caller rejects `readyTimeout`, a second independently pending caller rejects `closed`, and release/detach/disconnect each occur once.
- For attach-post failure, have the captured attach handler set every ring's writer state nonzero and then throw. Assert every state is zero at disconnect time and after rejection. Make detach and disconnect throw in this fixture and still require the original `attachPost` error. This makes cleanup observable rather than relying on rings' initial zero state.

### Prelude behavior and realtime discriminator

- Execute the moved asset in the existing VM-style AudioWorklet sandbox, register the wrapped engine and attach processors, and run three mono/stereo/mono rings. Submit real nonzero PCM with a partial tail and assert exact planar staging plus positive-zero tail values and submit metadata.
- Prove result-6 submit retains the slot without a refusal count, a later success drains it, result-6 seek retries, the successful newer seek drops the queued stale generation, underrun accounting advances, and attach/detach toggles all rings. Assert the relevant submitted/refused/stale/seek/occupancy counters, not only that methods were called.
- Instrument all typed-array constructors plus `subarray` and `slice` only while the first successful drain and later steady/partial drains run. Construction and attachment happen before arming. Require zero tracked events. Run one in-memory source mutation inserting `new Float32Array(4)` into `drainSharedRing` through the same helper and require the tracker to turn red. The existing narrow source checks may stay as complementary evidence; they are not the allocation proof.
- Do not edit the copied prelude to make the test pass, build a browser timing harness, or introduce a generic allocation-testing framework.

### Packed public consumer

- Extend the existing fresh tarball TypeScript consumer in `sdk/test/package-tarball-smoke.mjs` to import `prepareEngineFeed`, `attachEngineFeed` and the needed PCM writer primitive from `@misofm/engine/browser`. Under its existing strict `lib.dom` program, call the API with `{ sourceId, channels }`, a declared `BaseAudioContext`, and a factory explicitly typed `(BaseAudioContext, string, AudioWorkletNodeOptions) => AudioWorkletNode`. This exact fixture must catch both attempt-1 declaration failures without casts or SDK DOM-library changes.
- In the packed runtime portion, use small structural fakes to load the packaged default prelude URL, then attach once with the default module URL and injected node seam. Assert the explicit module URL still wins, ring identity/default capacity, successful readiness and idempotent close. Do not instantiate a real audio device or duplicate all source-level lifecycle cases here.
- Assert the packaged prelude is separately present/addressable and `dist/NOTICE` contains adapter, earlier Engine and earlier app provenance. Keep the manifest's six generated Engine artifacts exact and compare all six bytes with `/private/tmp/dx-393-current-artifacts`.

These tests qualify externally supported behavior: public ring layout/writer semantics, feed terminal lifecycle and browser assignability, the shipped prelude's synchronous drain, and fresh-package consumption. Do not add exhaustive corrupt-buffer fuzzing, internal helper snapshots, duplicate source and tarball suites, a second adapter reader, full browser/device matrices, new performance machinery, or byte pins for prose/tests. Existing current-adapter cases may be moved or adapted only where they directly prove this boundary.

## Exact paths for attempt 2

- `.github/ISSUE_SPECS/405-move-the-existing-browser-pcm-feed-into-the-typescript-sdk.md` — attempt-2 evidence only
- `sdk/src/browser/pcm-feed.ts`
- `sdk/test/browser-pcm-evals.mjs`
- `sdk/test/package-tarball-smoke.mjs`
- `NOTICE`

Use `sdk/test/browser-pcm-types.ts` only if a source-level non-DOM type assertion cannot live cleanly in the existing type corpus; the strict real-DOM declaration proof still belongs in the fresh packed consumer. `sdk/src/browser/pcm-ring.ts`, the prelude asset, barrels, assets table and staging script should remain unchanged from attempt 1 unless a direct gate failure proves a correction is necessary. No adapter, Rust, host/worklet, generated artifact, ABI/parameter JSON, SDK `tsconfig`, package dependency, workflow or CI-tooling path is allowed.

## Gates and evidence

Run once after the coherent revision is complete:

1. Focused PCM eval through the repository's existing headless wrapper with `/private/tmp/dx-393-current-artifacts`; record focused subtest names/counts, including the blocked-ready and allocation-mutation discriminators.
2. `bash scripts/check-sdk-types.sh` and show the fresh packed strict DOM consumer has no diagnostics. A local red mutation restoring `sources: Msb1RingLayout[]` or fixing the factory context to `FeedContext` must make that consumer fail.
3. `bash scripts/check-sdk-generated.sh`.
4. `bash scripts/check-sdk-headless.sh /private/tmp/dx-393-current-artifacts`.
5. From `sdk`, `npm run check:package -- /private/tmp/dx-393-current-artifacts` (or the exact repository wrapper it invokes), including the fresh tarball runtime/type consumer.
6. Byte-compare all six generated artifacts against `/private/tmp/dx-393-current-artifacts`; record the prelude and NOTICE separately and do not repin the documented Darwin mismatch.
7. `git diff --check` and an exact-path diff audit.

Update issue #405 with Luna attempt-2 evidence but make no PASS claim. Root commits the exact coherent paths before any new implementation tranche. Dedicated Astra then reviews the pushed exact commit and reruns the reproduced blocked-ready, DOM consumer, attach cleanup, runtime allocation/red-mutation and packed-consumer gates. No adapter migration begins before that verdict.

## Luna attempt 2 — source and focused regression checkpoint (2026-09-05)

Luna corrected the public source shape to `{ sourceId, channels }`, made the feed factory context generic so a strict real-DOM `BaseAudioContext`/`AudioWorkletNode` factory assigns without casts, restored the monotonic `performance.now()` default, and added a private terminal notification so every already-blocked `ready()` rejects promptly with `closed`. Timeout ownership remains with the caller that observes its deadline; attach-post cleanup releases all ring writer states before best-effort detach/disconnect, preserving the typed primary `attachPost` failure. Repository `NOTICE` now carries the adapter, authorized Engine, and authorized app source identities and the staged package smoke checks all three exact commits.

The four focused `browser-pcm-evals.mjs` subtests now cover literal mono/stereo layout bytes and UTF-8 limits, writer headers/counters/backpressure/wrap zero-fill/seek/release/malformed buffers, default and explicit prelude URLs, typed factory precedence, odd-source attach identity, empty/success readiness, two blocked callers, timeout ownership, attach-post cleanup under throwing cleanup calls, and the copied prelude's odd mono/stereo drain behavior. The prelude scenario instruments all typed-array constructors plus `subarray`/`slice` only after setup and records zero events across first/steady/partial drains; an in-memory `new Float32Array(4)` mutation turns the tracker red.

Evidence from this checkpoint: `bash scripts/check-sdk-types.sh` PASS; `bash scripts/check-sdk-generated.sh` PASS; `node --test sdk/test/browser-pcm-evals.mjs` PASS, 4/4; `bash scripts/check-sdk-headless.sh /private/tmp/dx-393-current-artifacts` PASS, 137 pass / 1 skip / 0 fail; `npm run check:package -- /private/tmp/dx-393-current-artifacts` PASS, including the fresh strict DOM declaration consumer and packed runtime consumer. All six generated Engine artifacts compare byte-identically with `/private/tmp/dx-393-current-artifacts`; the prelude remains `d81c2159b6ca088df97e76f09c4803540e6451920cc3af1b091336fa1bdba58d`. This is an implementation checkpoint for dedicated Astra review, not a PASS claim.

## Review stop questions

- Can `close()` settle every already-blocked `ready()` without waiting for the injected wait, and does timeout preserve its own `readyTimeout` result?
- Can an ordinary `{ sourceId, channels }` plus real DOM factory typecheck against packed declarations without casts?
- Does failure cleanup demonstrably release engaged rings before disconnect, even when cleanup calls throw?
- Does runtime evidence prove first and steady drain allocation behavior and turn red for an inserted typed-array allocation?
- Does the installed package actually use PCM ingress and retain every relevant provenance source while its six generated artifacts remain unchanged?
- Did any work expand into adapter behavior, progressive playback, boot defaults, storage/codec/pump policy, host messages, generated artifacts, or new test infrastructure?

## Dedicated Astra attempt 2 verdict — FAIL (2026-09-05)

Product corrections and packed DOM/runtime consumption pass independent review at `c3c2c972`. Three concrete escaping mutations prevent PASS: later populated-drain allocation, full-ring PCM corruption, and release after disconnect. The frozen related ring/lifecycle assertions remain incomplete. Full review and reproductions are attached to PR #398. Root approves the following Sol-authored final test-only brief. Issue #409 CI correction was independently reviewed and integrated separately; it changes no PCM source.

## Attempt 3 authorization — final test-only correction

## Authority and boundary

Attempt 2 at clean pushed `c3c2c972` failed dedicated Astra review only because three required regressions remain ineffective or incomplete. The product correction, strict real-DOM consumer, normal package gate, headless gate, copied prelude, cleanup implementation, provenance, and six generated artifact bytes independently pass. Preserve them unchanged.

This is the third and final attempt under the issue workflow. Luna may edit only:

- `sdk/test/browser-pcm-evals.mjs` — focused test corrections below.
- `.github/ISSUE_SPECS/405-move-the-existing-browser-pcm-feed-into-the-typescript-sdk.md` — truthful attempt-3 evidence after gates.

No production, package, adapter, Rust, generated artifact, ABI, workflow, dependency, runner, or CI-#409 edit is authorized. Add no framework, broad matrix, new feature, or fourth attempt.

## Exact remaining blockers

1. **Allocation coverage over the complete populated-drain scenario.** Keep tracking armed only around `process()` execution, but assert the tracker remains empty after all first, later successful, result-6 retry, partial, seek, stale-drop, and underrun processing. Preserve the unconditional inserted-allocation red discriminator. Add direct assertions for captured submit generation/start/channels/frames/end and successful retry, captured seek generation/frame and retry, and that the stale queued slot is never submitted. Astra's exact escaping mutant inserts `if (control[CONTROL_WROTE] > 1) new Float32Array(4)` before `const staging = this.sourcePcm`; the focused suite must fail it.

2. **Full-ring byte retention and already-frozen ring boundaries.** Immediately before a capacity-two full `reserve()`, snapshot queued slot headers, queued planar PCM, and write index. After `reserve()` returns `null`, assert all snapshots are unchanged and overflow advances exactly once. Astra's exact escaping mutant zeroes the first queued PCM plane in the full-ring null branch; the suite must fail it. In the same fixture complete only the original small boundary assertions: literal layout constants and every initialized control word including the last four; decoded bytes of a 128-byte multibyte ID; signed generation tag distinct from full generation; malformed writer-buffer magic plus zero/non-power-of-two shape; and explicit index wrap/occupancy. Use discriminating generation values, not 1/7/8. Do not add fuzzing or a reader implementation.

3. **Observable cleanup ordering and deterministic lifecycle ownership.** In attach-post failure, capture all three engaged writer states inside `disconnect()` and require `[0,0,0]`; assert exactly one detach and one disconnect while both throw, and preserve `PcmFeedError("attachPost")`. Astra's exact escaping mutant moves ring release after disconnect; the suite must fail it. Add the missing typed `nodeCreate` failure. Replace the weak timeout case with an engaged ring, one explicitly controlled deadline-owner wait, and a second never-resolving wait; advance and resolve only the owner, then assert ordered results `readyTimeout`, `closed`, one release/detach/disconnect, and idempotent close. Bound both this observation and the existing two-blocked-caller close case with the test harness so a regression fails promptly.

Use `/private/tmp/dx-405-astra-mutations-attempt2.mjs` and `/private/tmp/dx-405-astra-review-attempt2.md` as exact reviewer reproductions; do not copy them into the repository.

## Handoff and gates

Implement one coherent test tranche. Run the focused PCM eval, then `bash scripts/check-sdk-headless.sh /private/tmp/dx-393-current-artifacts`, and the existing packed/package gate from `sdk` against that same artifact directory. Also run `git diff --check` and exact-path audit. Record subtest names/counts and prove each of the three reviewer mutants turns red. Do not alter production to satisfy a test.

Root must checkpoint the exact allowed paths before any further Luna work and push the review commit. Dedicated Astra then reviews that exact commit for the three blockers and scope conservation. On PASS, root synchronizes issue/PR evidence; on FAIL, stop and rescope under the three-attempt rule. Adapter migration remains paused until PASS.

## Luna attempt 3 — final focused-test correction (2026-09-05)

Luna made the final authorized test-only correction in `sdk/test/browser-pcm-evals.mjs`. The ring fixture now freezes the literal layout/control words through the final control words, decodes the exact 128-byte multibyte ID boundary, distinguishes signed generation tags from full generations, proves malformed magic and zero/non-power-of-two shapes, snapshots queued headers/planar PCM/write index across full-ring backpressure, and exercises index wrap/occupancy. The lifecycle fixture observes all engaged writer states inside disconnect, counts one detach/disconnect under throwing cleanup, covers typed `nodeCreate`, and bounds both blocked-ready and deterministic timeout ownership with ordered `readyTimeout`/`closed` results. The prelude fixture keeps allocation tracking armed across first, later successful, backpressure retry, partial, seek retry, stale-drop, and underrun processing, and captures submit/seek metadata while proving the stale slot is never submitted.

Final evidence:

- `node --test sdk/test/browser-pcm-evals.mjs`: 4 pass / 0 fail; log `/private/tmp/dx-405-luna-attempt3-focused.log`.
- Exact Astra reproductions from `/private/tmp/dx-405-astra-mutations-attempt2.mjs`: all three exit 1 as intended — steady populated-drain allocation, full-ring PCM corruption, and release-after-disconnect; summary `/private/tmp/dx-405-luna-attempt3-mutants.log`, isolated logs under `/private/tmp/dx-405-mutations-g5osC2/`.
- `bash scripts/check-sdk-headless.sh /private/tmp/dx-393-current-artifacts`: 137 pass / 1 platform skip / 0 fail; log `/private/tmp/dx-405-luna-attempt3-headless.log`.
- From `sdk`, `npm run check:package -- /private/tmp/dx-393-current-artifacts`: PASS, including the fresh packed consumer and package artifact checks; log `/private/tmp/dx-405-luna-attempt3-package.log`.
- `git diff --check`: PASS. Exact working-tree diff contains only `sdk/test/browser-pcm-evals.mjs` and this issue evidence file; log `/private/tmp/dx-405-luna-attempt3-diff.log`.

No production, package, adapter, generated artifact, ABI, workflow, dependency, runner, or CI-#409 paths changed. No Rust rebuild or Darwin baseline repin was performed. This is the third and final attempt; root must checkpoint and push these two exact paths for dedicated Astra review. No PASS or issue closure is claimed by Luna.

## Dedicated Astra attempt 3 verdict — FAIL; implementation STOPPED (2026-09-05)

Astra reviewed `2473cfdf`. All three prior escaping mutants now fail at their intended assertions, and focused tests and packed consumer pass. Two required discriminators still escape: a retained-slot retry may skip the actual submission, and the writer may accept malformed non-power-of-two capacity. These are test-evidence gaps; the reviewed production corrections remain unchanged. The final review is attached to PR #398. The three-attempt budget is consumed: no fourth implementation/test revision is authorized here. Claims requiring these regressions remain unqualified.

For delivery isolation, approved host request-ID and CI-fixture work is now on PR #413. This stopped PCM branch merges that reviewed base without changing its PCM implementation and is preserved as a separate dependent PR. This administrative isolation is not a new implementation attempt. No merge, publication or PCM qualification PASS is claimed.

## Renewed execution — user-confirmed end-to-end plan

After the historical attempt stop, the user explicitly confirmed the SDK/adapter ownership plan, requested execution of the rest with Astra medium, and set completion as the real misofm/app flow without shortcuts. This authorizes renewed completion of this existing boundary, preserving prior evidence and all functional/realtime requirements. Astra medium will correct the outstanding concrete retry-submission and malformed-writer-capacity test discriminators, run the existing focused/package gates, and submit a complete result to a separate Astra medium reviewer. No product feature, backend, protocol, or test framework is added. This is renewed user-directed work, not a claim that the earlier attempt passed.

## Astra renewed completion checkpoint (2026-09-05)

The renewed test-only correction closes the two concrete evidence gaps without changing production: the prelude fixture now requires four captured calls after refusal and five after retry, then checks the new call's exact generation, start frame, channels, frame count, end flag and PCM. The writer fixture supplies a capacity-four buffer whose capacity word is changed to three and requires constructor rejection. This distinguishes the writer boundary from the existing creator validation.

Audit of the current ring, feed lifecycle, copied prelude, public exports and packed consumers found no additional product correction needed for the frozen SDK boundary. The existing evidence still covers literal layout/generation fields, full-ring retention, mono/stereo partial PCM, seek retry and stale-drop behavior, terminal readiness and cleanup ordering, allocation tracking over populated drains, real DOM assignability and installed-package consumption. App/adapter integration remains downstream and is not claimed by this checkpoint.

Validation of the coherent tranche:

- Focused PCM eval: all four named cases pass — `MSB1 layout and writer preserve frozen bytes, headers, counters and reuse`; `feed lifecycle preserves URL, typed seams, identity and terminal cleanup`; `attach-post cleanup releases engaged rings before throwing cleanup failures`; `moved prelude drains odd mono/stereo rings and allocation mutation turns red`. Log: `/private/tmp/dx-405-renewed-focused.log`.
- All five exact reviewer mutants fail meaningfully. The three earlier reproductions from `/private/tmp/dx-405-astra-mutations-attempt2.mjs` fail at the complete allocation tracker, queued PCM snapshot and disconnect-time writer states respectively (logs `/private/tmp/dx-405-mutations-0X7KfD/`, summary `/private/tmp/dx-405-renewed-mutants.log`). The unchanged reviewer retry-drop worklet fails the new call-count assertion with `4 !== 5`; the unchanged non-power-of-two writer mutant fails with a missing constructor exception. Updated test copies reuse the reviewer sources under `/private/tmp/dx-405-final-boundaries-1DjM6m/renewed-{retry,writer}.mjs`; logs `/private/tmp/dx-405-renewed-{retry,writer}.log`.
- Type and generated gates pass: `/private/tmp/dx-405-renewed-types.log`, `/private/tmp/dx-405-renewed-generated.log`.
- Existing headless wrapper against `/private/tmp/dx-393-current-artifacts`: 137 pass, 1 platform skip, 0 fail; `/private/tmp/dx-405-renewed-headless.log`.
- Existing package gate against the same artifact directory passes, including fresh packed strict DOM and runtime consumers; `/private/tmp/dx-405-renewed-package.log`.
- All six staged generated artifacts are byte-identical to that reviewed baseline. Prelude hash remains `d81c2159b6ca088df97e76f09c4803540e6451920cc3af1b091336fa1bdba58d`; staged NOTICE is byte-identical and its three provenance identities pass the packed gate. `/private/tmp/dx-405-renewed-artifacts.log` records the comparison. No Rust rebuild or Darwin repin.
- `git diff --check` passes; only this spec and `sdk/test/browser-pcm-evals.mjs` changed.

Implementation is paused at this green exact-path checkpoint for root commit/push and separate Astra review. Historical FAIL verdicts stand; this record makes no independent-review PASS, remote closure, publication or app end-to-end claim.
