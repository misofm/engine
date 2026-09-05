# Make browser SDK boot defaults package-owned

**Status:** Approved bounded SDK brief under the user-confirmed end-to-end plan. Astra medium implements and a separate Astra medium agent reviews. Adapter #22 and SDK #405 are now independently approved; root will synchronize this issue before implementation.

## Baseline and dependency truth

Start from reviewed integrated SDK checkpoint `a0493021` on `codex/dx-browser-control`, containing approved #393 host ownership, #409 fixture correction, and renewed approved #405 PCM ingress. Reuse existing asset/NOTICE staging and preserve all PCM bytes; this issue changes only default boot ownership. Registry publication and progressive playback are not prerequisites.

Current SDK `createEngine` already owns source-format preflight, scratch-derived shape, construct/verify/close/retry, pre-worklet quantum refusal, worklet boot, console binding, and host-then-context close. Its three injected functions are required. The SDK compiles with `ES2022` + `WebWorker`, while a normal packed browser consumer compiles with DOM types. The solution must preserve both environments.

## Smallest closable product slice

Make `createEngine({ document })` boot the existing `BrowserEngine` through packaged defaults. Make `createContext`, `scratchBoot`, and `createHost` optional while preserving their current signatures and precedence when supplied. Move the adapter's existing engine-specific scratch Worker client/entry and shipped-host import factory into the SDK, and add a guarded default `AudioContext` constructor. This changes construction ownership only; it creates no second engine/session abstraction and no delivery behavior.

The future adapter consumer remains a separate issue. This issue only leaves public, narrow SDK helpers it can call later.

## Frozen SDK contract

- Existing `simd128ModuleUrl` and `workletModuleUrl` overrides keep their behavior. Add only `hostModuleUrl`, `scratchWorkerModuleUrl`, and the structural Worker factory/deadline/signal inputs needed by the defaults. Each supplied injection or URL wins independently; an all-injected call touches no global constructor, Worker, or dynamic import.
- The default context factory performs a guarded runtime lookup of `globalThis.AudioContext` and constructs it with the existing exact `{ sampleRate, renderSizeHint }` request. Absence is a typed boot failure. Do not add `DOM` to SDK `tsconfig`.
- Parameterize `CreateEngineOptions` and `BrowserEngine` over the context returned by `createContext`. In a packed DOM consumer, the default call must infer the structural instance type of that consumer's `globalThis.AudioContext`, permitting `resume()`, `suspend()`, and `engine.host.node.connect(engine.context.destination)` without casts. An injected factory retains its exact return type and must not acquire capabilities it did not return. Use a structural conditional/overload; do not cast the result to a richer public type.
- Context ownership stays exact: every wrong-rate candidate closes before retry; quantum or later boot failure closes the accepted context; `BrowserEngine.close()` remains idempotent and disposes host before context, including when host disposal rejects. There is no caller-supplied context ownership mode.
- Preserve the current `sources` refusal and ordering. SDK-authored `32f` declarations refuse before scratch/context. Raw documents remain opaque bytes/string/builder output; add no parser or introspection.
- Keep `scratchBootInWorker` unchanged. Export one narrow `scratchBootWithWorker` one-shot helper; any reusable client underneath is private. It launches the packaged module Worker, waits for the existing ready handshake, sends one correlated document/options/Wasm-URL request, returns `SessionShape`, and terminates exactly once after success or failure.
- Preserve the adapter-derived bounded handshake/request deadline and optional abort. Timeout, abort, Worker `error`/`messageerror`, synchronous post failure, or a rejected Worker result must settle once, remove timers/listeners, terminate before rejection, and make late events inert. Abort racing with ready/result cannot return a live client. Repeated cleanup is safe.
- The packaged Worker entry fetches the requested Wasm URL, requires an OK response, reads its bytes, calls `scratchBootInWorker`, and posts the existing correlated serializable success/error shape. It owns no caller resource and exposes no session inspection API.
- Export one narrow default-host helper. It dynamically imports the selected packaged host module once, converts current SDK worklet boot options with `toWebBootOptions`, and calls `createMisoAudioWorkletHost` with the exact context, document, Wasm URL, and worklet URL. It loads no feed prelude and changes no host/wire request.
- Add a narrow browser-boot error discriminant only if exact operation identity cannot otherwise be tested or translated. Limit it to default context unavailable, scratch Worker start/load/deadline, and host import/creation. Existing engine refusals, `MisoUsageError`, and caller abort reasons pass through unchanged.
- Emit the Worker entry as ordinary TypeScript package output and expose its package-relative URL through `BUNDLED_ENGINE_ASSETS`. It is outside the six Rust-generated artifacts and their generated manifest/provenance closure. Copy repository `NOTICE` to `dist/NOTICE`, add the moved-code attribution from adapter baseline `63b4ee6212287000ff85e1cfa969d385f6246d2d`, and keep the generated artifact bytes and manifest byte-identical.

## Allowed paths

- `.github/ISSUE_SPECS/428-make-browser-sdk-boot-defaults-package-owned.md`
- `sdk/src/browser/engine.ts`
- `sdk/src/browser/scratch.ts` (new)
- `sdk/src/browser/scratch-worker.ts` (new; one equivalent emitted entry is allowed)
- `sdk/src/browser/default-host.ts` (new; a bounded section of `engine.ts` is allowed)
- `sdk/src/browser/index.ts`
- `sdk/src/assets.ts`
- `sdk/codegen/stage-package.mjs`
- `sdk/test/browser-evals.mjs`
- one focused SDK scratch/default-host eval file
- one focused structural-context type probe
- `sdk/test/package-tarball-smoke.mjs`
- `sdk/README.md`
- `NOTICE`

No adapter path, Rust/Cargo path, generated host/worklet JS or declaration, Wasm, ABI/parameter JSON, generated provenance, policy algorithm, session/console implementation, runtime dependency, codec, storage, PCM behavior, backend-selection, transport, or release workflow may change. If making the public Worker URL import-complete after packaging demonstrably requires a small build-only bundling step, propose the exact build paths/dependency before implementation; do not hide a broken worker graph behind the default-factory test.

## Proportional objective gates

1. Existing injected browser evals remain green. Add default-path and per-injection precedence coverage proving one existing `BrowserEngine`, exact URLs/options/identity, source refusal before work, construct/verify/close/retry, quantum refusal before host creation, shared scratch/worklet policy words, and idempotent host-then-context close.
2. Focused scratch tests cover success plus representative fail-close races: never-ready timeout, abort during request, Worker error/messageerror, synchronous post failure, late reply, and repeated cleanup. Assert one settlement, one termination, and no surviving timer/listener; do not build a combinatorial fixture matrix.
3. Default-host coverage proves one import, exact `toWebBootOptions` result and URL forwarding, returned host identity, and accepted-context cleanup on import/factory failure. No feed prelude or wire change appears.
4. `scripts/check-sdk-types.sh` passes under the unchanged ES/WebWorker libs. A packed strict DOM consumer calls `createEngine({ document })`, resumes/suspends its inferred context, and connects the host node to `destination` without casts. Two distinct injected structural factories retain only their actual capabilities.
5. `scripts/sdk-package.sh check` and packed tarball smoke pass. The extraction contains the Worker graph, `NOTICE`, and resolvable package-relative URLs; the generated six-artifact manifest/closure and pinned bytes remain unchanged. Also run `scripts/check-sdk-generated.sh`, `scripts/check-sdk-headless.sh`, and the focused browser evals. No full Rust rebuild, browser matrix, benchmark, or listening gate is required.

## Release and review truth

This SDK issue is independently closable when Astra's bounded implementation, proportional local gates, and dedicated Astra adversarial review pass, its evidence commit is upstream, and the matching GitHub issue is synchronized and closed. A local packed tarball proves the capability; it does not claim npm publication or adapter compatibility. Adapter migration, exact published dependency integration, no-double-boot orchestration, storage/readiness/playback, and progressive delivery remain future consumer/release work.

## App delivery requirement

The existing app intentionally vendors exact reviewed package archives with hash/source provenance. Use this workflow for downstream integration; npm publication is not a prerequisite. This issue must prove packaged Worker loading through default and ordinary forwarding custom factory paths, including dependency graph resolution in a real packed browser consumer, so it does not repeat the adapter worker URL defect. App UI/offline/spectrum compatibility remains in the downstream adapter/app preservation work.

## Execution record

Matching issue misofm/engine#428. Isolated branch codex/dx-sdk-boot starts from approved PCM boundary a0493021. Astra medium implements; separate Astra medium review follows. This SDK ownership slice is independent of the adapter OPFS correction, with no shared checkout, files or build outputs. Root checkpoints coherent exact-path tranches before further implementation.
