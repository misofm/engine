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
