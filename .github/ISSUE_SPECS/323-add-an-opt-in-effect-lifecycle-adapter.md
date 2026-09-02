# Add an opt-in Effect lifecycle adapter

## Objective

Make engine acquisition, asynchronous console submission, and guaranteed release composable in
Effect without imposing Effect on the SDK's ordinary Promise consumers or its real-time boundary.
Close the browser facade's lifecycle gap with one idempotent cleanup operation that both programming
models share.

This is issue #207's smallest closable Effect slice. It layers orchestration over the already typed
semantic console from #322; it does not replace that console or change its transaction semantics.

## Product contract

- `@misofm/engine/effect` is an explicit optional entry point backed by the stable Effect v3 line.
  Effect is an optional peer dependency and an exact development dependency; importing the root,
  headless, browser, or asset entries does not resolve or load it.
- The entry exposes typed scoped acquisition for headless and browser engines plus semantic-console
  submission. Expected Promise rejections enter a named `EngineEffectError` channel carrying the
  failed operation and original cause.
- An engine command refusal remains a successful `CommandReport`, not an Effect failure. The engine
  is the authority on admission, and callers must be able to inspect generated result/reason names,
  rejected index, admitted count, and application sample.
- `BrowserEngine.close()` disposes the worklet host before closing its `AudioContext`, invokes each
  operation at most once, and still closes the context when host disposal fails.
- The existing Promise APIs remain unchanged. No Effect runtime or scheduler is used by render,
  PCM submission, Wasm calls, wire encoding, or synchronous edit construction.

## Scope

- One optional package entry, its typed failure model, lifecycle helpers, tests, and documentation.
- One explicit browser-facade close operation used by both Promise and Effect consumers.
- Package metadata and clean-tarball qualification for a consumer with and without the optional
  peer installed.

No Effect v4 release-candidate dependency, core API rewrite, render-path abstraction, retry policy,
telemetry service, source pump, registry publication, or engine/host byte change is in scope.

## Objective gates

1. A fresh extraction imports every existing entry without Effect installed; its optional Effect
   entry imports and typechecks when the declared peer is installed.
2. A scoped live-Wasm headless engine is disposed after success, typed failure, and interruption.
3. A scoped browser engine and direct `BrowserEngine.close()` dispose the host before the context,
   are idempotent, and close the context even when host disposal rejects.
4. Headless/browser acquisition rejection and asynchronous submit rejection produce the named
   operation in `EngineEffectError` with the original cause retained.
5. A typed engine command refusal stays in the Effect success channel with zero admitted records;
   no acknowledgement is synthesized before the underlying transport settles.
6. The package has no ordinary runtime dependency on Effect, and no Effect import is reachable from
   the render/PCM/direct-boundary source graph.
7. Existing SDK behavior, type, generated-surface, package, browser, deletion, and upstream release
   gates stay green.

## Decision record

- The user-provided v4 onboarding describes the right capabilities—typed failures, structured
  concurrency, and resource safety—but npm currently tags stable as `3.22.1` and v4 as
  `4.0.0-rc.112`. A production SDK must not make a release candidate its runtime foundation, so
  this entry targets `effect@^3.22.1`. Reconsider v4 after it reaches the stable tag in a separate
  compatibility issue.
- Effect belongs at the ownership and asynchronous-I/O boundary. The engine's synchronous render
  and command encoding are already explicit, bounded operations; wrapping them in a scheduler would
  obscure rather than standardize the real-time contract.
- Effect acquisition is scoped-only. A non-cancellable Promise may finish opening an engine after
  its fiber is interrupted, losing the only reference before a finalizer is registered. The
  existing Promise constructors are the explicit caller-owned API for engines that outlive a scope.

## Evidence

Implementation attempt 1:

- Added the isolated `@misofm/engine/effect` entry over `effect@3.22.1`, declared as an optional
  peer and exact dev dependency. The existing four code entries import from a clean extraction
  before the peer is mounted; the Effect entry then imports and resolves declarations with it.
- Added typed open/scoped programs for headless and browser engines plus semantic submission.
  Protocol refusals remain successful `CommandReport` values; Promise rejections preserve their
  original cause in an operation-tagged `EngineEffectError`.
- Added one idempotent `BrowserEngine.close()` that disposes host then context and uses `finally` so
  a failed host disposal cannot leak the context.
- Adversarial lifecycle tests exposed a pre-existing contradiction: `state()` included `disposed`
  in its return type but queried a status pointer through the cleared handle after disposal. The
  boundary now answers owned disposal state before its live-handle guard.

Local gates on 2026-09-02:

- `check-sdk-headless.sh`: PASS, 111 tests / 27 suites against live Wasm.
- `effect-integration.mjs`: PASS, 5 tests covering scoped success/failure/interruption, headless and
  browser acquisition failures, browser release order, protocol refusal, and no result before a
  rejecting transport settles.
- `check-sdk-types.sh`: PASS, including exact Effect success/error/Scope requirement pins.
- `check-sdk-deletions.py`: PASS over 43 SDK source files.
- `sdk-package.sh check`: PASS with 61-file clean tarball, optional-peer isolation, declaration
  resolution, embedded-Wasm boot/digest mutation, and the Effect integration suite.
- `npm publish --dry-run --ignore-scripts`: PASS; 939.4 kB packed / 3.5 MB unpacked.
- The checked-in AudioWorklet digest is known not to reproduce on this macOS toolchain environment
  (recorded during #319); the local package gates therefore used the same source-built artifact
  without changing its pin. Upstream Linux artifact and release jobs remain authoritative for the
  pinned digest.

Adversarial review of attempt 1:

- HOLD: `Effect.tryPromise` is interruptible while the host Promise is not cancellable. An unscoped
  open could finish after interruption and leak its engine, and an interrupted command fiber could
  detach while its mutation remained in flight. Packaging, typing, and lifecycle finalizers passed.

Implementation revision attempt 2:

- Removed unscoped Effect acquisition; the established Promise constructors remain the
  caller-owned lifecycle. Scoped acquisition cannot lose ownership between open and finalizer
  registration.
- Made semantic submission uninterruptible after dispatch. A new deferred-ack probe interrupts its
  fiber, proves interruption does not complete before the transport acknowledgement, then proves
  the pending interrupt is delivered after settlement.

Adversarial review of attempt 2:

- PASS locally. Strict types and all 5 lifecycle tests pass after the revision. The package gate
  passes against the exact pinned artifact uploaded by browser qualification run
  [33602511772](https://github.com/misofm/engine/actions/runs/33602511772): 61-file tarball,
  939.6 kB packed / 3.5 MB unpacked.
- The cancellation probe now establishes the transaction rule directly: interruption requested
  after dispatch remains pending until transport acknowledgement, then is delivered. An Effect
  fiber can neither invent an ack nor leave a command outstanding while its scope releases.
- Final closure requires the revision commit's upstream main, browser, and release workflows.

Final upstream evidence on 2026-09-02:

- Revision commit `f54d845fd10aa88d7c739b89e2038e8138ee2f7e` is on `main`.
- Main CI [33604440678](https://github.com/misofm/engine/actions/runs/33604440678): PASS.
- Browser qualification [33604440726](https://github.com/misofm/engine/actions/runs/33604440726): PASS.
- Release build [33604440568](https://github.com/misofm/engine/actions/runs/33604440568): PASS.
- Final adversarial verdict: **PASS**. The opt-in adapter has typed failure and scoped ownership,
  does not impose Effect on ordinary SDK consumers, and cannot acknowledge before or detach from
  an outstanding transport mutation.
