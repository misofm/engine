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
- The entry exposes typed Effect programs for opening headless and browser engines, scoped variants
  that always release them, and semantic-console submission. Expected Promise rejections enter a
  named `EngineEffectError` channel carrying the failed operation and original cause.
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
- Scoped programs are the preferred Effect API. Unscoped open programs remain available because
  application-owned runtimes sometimes need an engine that outlives one scope, but their returned
  engine retains the same explicit `dispose()`/`close()` contract as the Promise surface.

## Evidence

Pending implementation attempt 1.
