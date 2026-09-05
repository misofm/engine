# Expose bounded read-only PCM observation for existing source spectrum

## Required existing behavior and boundary

The app already displays input-source spectrum using its2048-point FFT and a bounded pull from source rings. SDK#405 moved PCM layout authority out of adapter/app. App#101 must retain spectrum without copying private ring offsets or using Msb1RingReader, which changes audio consumer state. The SDK owns this codec-neutral read-only observation; app retains FFT/display and adapter will later map source IDs to observers. No post-effect or arbitrary graph taps.

## Smallest API

Export PcmSourceChunk {generation:bigint,startFrame:bigint,frames:number,endOfRegion:boolean,planes:readonly Float32Array[]} and Msb1RingObserver from @misofm/engine/browser. Constructor accepts an existing SharedArrayBuffer ring and uses the same SDK validation authority. Expose channels, frameCapacity, pull(consume:(chunk:PcmSourceChunk)=>void, maximumChunks?:number):number, counters():Msb1RingCounters, close():void. Add only already-present missing wire counters needed by existing app totals (underruns,drainBlocks,depth) to the shared counter snapshot authority; no new wire words.

## Frozen bounded read contract

- Independent observation cursor, no Atomics stores/adds or mutation of any playback shared word/PCM. Never advances read/write indices, attaches/detaches the ring, holds slots, or adds another producer. Observer runs on app/control thread only.
- Own one reusable planar scratch chunk and metadata object. Callback data is borrowed until return; frames bounds valid samples. After construction, pull allocates no PCM buffers/views and does bounded work. Default maximum min(capacity,32); explicit maximum integer1..32. Slow observers catch up; they never stall audio.
- Begin at oldest currently live chunk. Handle ring-index wrap, missed data, seek epoch/generation changes, stale/invalid/torn slots, mono/stereo and partial tail. On close release local ownership idempotently; subsequent pull returns0.
- Copy into scratch only after checking slot sequence/generation and current live interval. Recheck sequence/generation/live interval after copying and drop observations whose slot became reusable/overwritten. A sequence-only test is insufficient because writer may overwrite PCM before publishing new sequence. A deterministic during-copy mutation in the existing fixture must reject mixed-generation/reused data without changing shared state. This is best-effort visualization observation, not guaranteed lossless delivery.
- Snapshot counters from existing words through one SDK authority; do not claim multiword atomicity. No source document parser, session changes, ring/prelude ABI change, decoder/storage input, telemetry protocol or new framework.

## Allowed paths and execution

sdk/src/browser/pcm-ring.ts or one adjacent pcm-observer.ts using shared validation helpers; sdk/src/browser/index.ts; sdk/test/browser-pcm-evals.mjs; existing strict packed type/runtime smoke; sdk/README.md; this spec. No Rust, generated six artifacts, PCM worklet prelude, SDK boot/control or adapter/app edits. Begin isolated codex/dx-pcm-observation after SDK428 review PASS and its evidence checkpoint. Source baseline9ab79d13 contains reviewed405 and completed428 revision awaiting review. Astra medium implements; separate Astra medium reviewer checks. Root commits focused-green tranche before final evidence.

## Evidence

Existing focused SDK fixture proves representative shapes/tail, budget cap, independent observers, index wrap/overrun, seek/stale generation, deliberate slot reuse during copy, byte-identical shared buffer before/after read-only operations, reusable local planes and closed no-op. Use fixed expectations rather than a second layout implementation. Existing type/headless/generated/package gates and strict public consumer pass. Six generated engine artifacts and PCM prelude remain byte-identical. No extra browser matrix, benchmark or listening gate for this visualization-only control API.

Matching issue misofm/engine#434.

## Attempt 1 focused checkpoint

Astra implementation adds `Msb1RingObserver` beside the existing writer, sharing its private validator and existing browser export. Observer owns reusable scratch/metadata, bounds candidate attempts, catches up across index wrap and seek epochs (including equal low-word generation tags), and validates the live interval again after copying. The existing counter snapshot type now includes the three existing wire words. Close drops ring/scratch references and retains a final counter snapshot; reentrant pull is rejected to protect borrowed scratch.

Focused evidence: `node --test sdk/test/browser-pcm-evals.mjs` PASS (8 tests, `/private/tmp/dx434-focused.log`); `sdk/node_modules/.bin/tsc --project sdk/tsconfig.json` PASS (`/private/tmp/dx434-typecheck.log`). Fixtures include mono/stereo partial tails, independent cursors, 32-attempt default, invalid-slot budget, wrap/overrun, stale/torn slots, full seek generation, constructor/view allocation traps, reusable identities and byte-for-byte shared-state preservation. Deterministic copy hooks advance READ_INDEX and reserve/zero a reused slot while its old sequence remains published, or seek during copy; both deliver zero callbacks. Existing packed smoke now exercises the public observer type and runtime. Full headless/generated/package evidence follows root's exact-path checkpoint; not yet claimed.

## Attempt 1 final validation at acf30599

- `bash scripts/check-sdk-headless.sh /private/tmp/dx-393-current-artifacts`: PASS, 160 passed / 1 existing skip (`/private/tmp/dx434-headless.log`).
- `npm run check:generated` and `npm run check:assets` from `sdk/`: PASS (`/private/tmp/dx434-generated.log`, `/private/tmp/dx434-assets.log`). The package gate also runs the complete `scripts/check-sdk-generated.sh`, including generated banners and the host declaration mirror; PASS.
- `npm run check:package -- /private/tmp/dx-393-current-artifacts` from `sdk/`: PASS (`/private/tmp/dx434-package.log`), including 11 CLI tests, strict extracted-tarball consumer types and public observer runtime assertions. No extra browser matrix was run for this control-thread observation API.
- Direct byte comparison: all six staged generated engine artifacts equal the approved input directory; the PCM prelude equals both baseline `e3a52dce` and its staged copy (`/private/tmp/dx434-bytes.log`). No Wasm rebuild or repin. Production/source paths remained clean throughout full validation.

Implementation and required evidence are ready for the separate independent Astra review; this record does not claim review PASS or issue closure.
