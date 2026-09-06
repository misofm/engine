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

### Narrow worker packaging amendment

Root approves adding a pinned build-only esbuild development dependency in sdk/package.json and sdk/package-lock.json and using sdk/codegen/stage-package.mjs to bundle emitted dist/browser/scratch-worker.js in place as an import-complete browser ES module. Existing TypeScript alone cannot preserve the worker dependency graph when a public URL is relocated by an app bundler. This preserves the public entry name, has no runtime package dependency, does not touch the six generated artifact bytes, and must make both default and forwarding custom Worker factories work in the packed browser. Preserve provenance/NOTICE.

### Attempt 1 implementation evidence (Astra medium, 2026-09-05)

Package-owned defaults are implemented on the existing `BrowserEngine`; independent injected
context/scratch/host functions and asset overrides remain available. The default context type is
resolved structurally from the consuming environment, and injected factories retain their actual
return type. `scratchBootInWorker`'s executable body is unchanged. A one-shot Worker helper owns
handshake/request deadlines, abort/listener/timer cleanup and termination. The narrow host helper
imports the selected shipped module and applies the existing `toWebBootOptions` adapter.

The approved build-only esbuild 0.28.1 dependency bundles the emitted Worker to an import-complete
43,914-byte entry. Vite's literal default Worker build and its raw copied exported Worker URL both
load successfully. No runtime dependency, generated host declaration, generated artifact byte,
PCM implementation or policy algorithm changed. NOTICE retains the moved adapter attribution.
Source/package milestones were checkpointed at `fca2f3a3`, `cba19b96`, and `b6747844`.

The first actual browser probe found that a context constructed after user activation may already
be running. The shipped host correctly refuses that state before worklet creation. The existing
adapter factory's suspension step was therefore preserved in the SDK default-host helper; focused
regressions prove suspension before construction and accepted-context closure if suspension
rejects. This is a construction precondition, with no host or wire relaxation.

Validation:

- `bash scripts/check-sdk-types.sh`: PASS under unchanged ES2022/WebWorker libs.
- `node --test sdk/test/browser-defaults-evals.mjs`: PASS, 17 focused tests, including fail-close
  handshake/request faults, late events, abort inside construction, exact default host forwarding,
  injected precedence, host failure cleanup and the native suspension precondition.
- Existing `sdk/test/browser-evals.mjs` against `/private/tmp/dx-393-current-artifacts`: PASS, 16
  tests. `bash scripts/check-sdk-headless.sh /private/tmp/dx-393-current-artifacts`: PASS, 152
  passed / one existing skip before the two additional suspension regressions above.
- `bash scripts/check-sdk-generated.sh`: PASS after final implementation; no Wasm rebuild/repin.
- `MISO_ENGINE_SDK_BROWSER_TOOLS=/private/tmp/miso-dx-app/node_modules bash
  scripts/sdk-package.sh check /private/tmp/dx-393-current-artifacts`: PASS, including the generated
  gate, CLI tests, packed strict DOM consumer, structural thin context negative checks, standalone
  Worker AST check, tarball smoke and actual Vite 8.2.2 / Playwright Chromium boot.
- The retained optional browser mode uses an existing Vite/Playwright `node_modules` selected by
  `MISO_ENGINE_SDK_BROWSER_TOOLS`; it adds no browser framework dependency. It serves the production
  bundle on loopback, clicks both boot paths, resumes/connects/status-checks/suspends/closes each
  engine and asserts no failed requests or HTTP errors. Both report 48000 Hz, 128 frames, status
  result 0 and context state `closed`; the forwarding factory receives the exact exported URL and
  `{ type: "module" }`. All observed requests returned HTTP 200. This probe required normal
  loopback/browser permissions beyond the sandbox's network restriction.
- Direct byte comparison: all six staged generated artifacts equal the approved input directory;
  manifest lengths/digests match those exact bytes; the staged PCM feed equals its unchanged
  source. The generated manifest retains exactly six artifact entries.

Reproducible browser assertions are in `sdk/test/package-tarball-smoke.mjs`, not solely in temporary
logs. Local logs: `/private/tmp/dx428-final-package-browser.log`,
`/private/tmp/dx428-final-generated.log`, `/private/tmp/dx428-focused.log`, and
`/private/tmp/dx428-headless.log`. Dedicated independent review and root's final upstream/issue
synchronization remain pending. These gates prove a packed SDK capability, not npm publication or
completion of the downstream adapter/app integration.

### Attempt 2 bounded revision evidence (Astra medium, 2026-09-05)

Attempt 1 received independent **FAIL at `56bbd518`**, preserved in
`/private/tmp/dx-428-astra-medium-review.md`: its Worker transport erased typed engine refusal
fields, and its resource fixture did not detect removal of settlement `clearTimeout`.

Checkpoint `2da95d23` addresses those two findings in the existing scratch client, entry and
focused test file only. The reply carries an explicit engine/usage discriminator; an actual
`MisoEngineError` carries phase, code, result and diagnostics, while `MisoUsageError` retains its
existing usage identity. The client constructs the corresponding SDK error class and preserves
the original formatted message verbatim without parsing it or decorating it twice. The generic
name/message fallback remains available for other Worker failures. No public engine authority,
context type, policy, generated artifact or PCM behavior changed.

The focused test runs the actual scratch primitive against the approved real Wasm and document
`{}`, then routes that same refusal through the actual Worker entry, structured-cloned messages
and actual one-shot client. It verifies `instanceof MisoEngineError`, name/message, phase/code/
result/diagnostics and diagnostic getters against the direct refusal; usage reconstruction is
also exercised through the entry/client. Compact local timer and AbortSignal listener accounting
requires zero active resources after success/abort/error. Captured removed Worker, abort and timer
callbacks are actually invoked after settlement; they remain inert with one termination/request
and one observed settlement.

Validation after the correction:

- `bash scripts/check-sdk-types.sh`: PASS.
- `MISO_ENGINE_SDK_ARTIFACTS_HEX=2f707269766174652f746d702f64782d3339332d63757272656e742d617274696661637473
  node --test sdk/test/browser-defaults-evals.mjs`: PASS, **19/19**. The artifact input is now
  required for the real-Wasm refusal regression, following the existing SDK fixture convention.
- Exact requested red mutation: remove only `clearTimeout(timer)` from `finish` in an isolated
  source copy. The focused run exits **1**, failing `no scratch deadline survives settlement`
  with actual 1 / expected 0. Source copy:
  `/private/tmp/dx428-attempt2-timer-9uffw4ye`; log:
  `/private/tmp/dx428-attempt2-timer-mutant.log`. The unmodified baseline exits 0.
- `bash scripts/check-sdk-headless.sh /private/tmp/dx-393-current-artifacts`: PASS,
  **156 passed / one existing skip** (157 total).
- `MISO_ENGINE_SDK_BROWSER_TOOLS=/private/tmp/miso-dx-app/node_modules bash
  scripts/sdk-package.sh check /private/tmp/dx-393-current-artifacts`: PASS, including its
  generated-surface gate, strict packed consumer types, standalone Worker/package checks and
  retained real Vite/Chromium default and forwarding-factory boot. Both paths report 48000 Hz,
  128 frames, status result 0 and closed contexts; all observed network responses are 200.
- Direct comparison again confirms all six generated artifacts equal the approved input bytes,
  their six-entry manifest lengths/digests match, and the staged PCM feed is unchanged.

Logs: `/private/tmp/dx428-attempt2-focused.log`,
`/private/tmp/dx428-attempt2-headless.log`, and
`/private/tmp/dx428-attempt2-package-browser.log`. Independent attempt-two review and final remote
issue synchronization remain pending; the first FAIL is retained rather than rewritten.

## Dedicated Astra medium attempt2 PASS

Independent review at9ab79d13 verifies typed scratch refusal classes/fields and meaningful timer cleanup regressions, plus headless and actual packed default/forwarding browser startup. Six generated artifacts and PCM bytes are unchanged. Report attached to PR432. Both first-review findings are resolved; historical FAIL retained. This completes package-owned startup, not downstream app integration or registry publication.
