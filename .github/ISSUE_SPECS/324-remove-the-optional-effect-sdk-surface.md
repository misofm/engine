# Remove the optional Effect SDK surface

## Objective

Remove Effect from the production TypeScript SDK before its first publication. The SDK's primary
contract is direct, typed, realtime control of the embedded Wasm engine; it must not carry a second
orchestration programming model that does not materially improve that path.

This issue deliberately reverses the optional integration added by #323 without reverting the
general lifecycle corrections discovered while qualifying it.

## Product contract

- `@misofm/engine` has four code entries: root, headless, browser, and assets. There is no
  `@misofm/engine/effect` entry.
- The package has no dependency, peer dependency, development dependency, source import,
  declaration, test, or documentation requirement on the Effect library.
- The established Promise APIs and fully typed semantic console remain the sole public control
  model across headless and browser surfaces.
- `BrowserEngine.close()` remains idempotent, disposes the worklet host before its `AudioContext`,
  and closes the context even when host disposal rejects.
- Disposed headless handles continue to report their disposed state without dereferencing a cleared
  Wasm handle.

## Scope

- Delete the optional Effect source entry and Effect-only type/runtime tests.
- Remove its package export and all dependency metadata and lockfile records.
- Remove Effect-specific package smoke setup, declaration imports, README guidance, and package-gate
  invocation.
- Retain and requalify the Promise lifecycle behavior introduced alongside #323.

No engine ABI, Wasm artifact, semantic-console command, browser transport, session schema, render
behavior, or package version change is in scope.

## Objective gates

1. The clean tarball exposes exactly the four code entries plus `package.json`, has no ordinary or
   peer dependencies, and contains no Effect entry or declaration.
2. A strict fresh TypeScript consumer imports every supported entry without installing Effect.
3. Repository search finds no Effect-library integration residue under `sdk/` or the SDK package
   gate; domain uses of the word “effect” for audio processors remain untouched.
4. Direct browser close ordering, idempotency, failure cleanup, and disposed headless-state tests
   remain green.
5. SDK type, generated-surface, deletion, headless, package, browser, main CI, and release gates stay
   green.

## Decision record

- Effect improved optional consumer interoperability but not the SDK's realtime Wasm control path.
  Maintaining another public programming model, an Effect v3 peer range, and a future v4 migration
  is unjustified without a concrete consumer.
- The lifecycle defects and invariants found during #323 are independent product improvements. They
  stay in the ordinary Promise API and retain their adversarial tests.
- A future application may add its own Effect adapter over the Promise API. Reintroducing an
  official adapter requires a concrete product consumer and a new issue.

## Evidence

Implementation attempt 1:

- Commit `34be6130f3f32d2cc984c841ad967801fc5525c0` removes the `./effect` export, Effect
  source and tests, README guidance,
  development/peer dependency metadata and transitive lockfile graph, and the Effect-only package
  gate invocation.
- The packed-artifact and hermetic package evals now pin exactly four code entries, no runtime or
  peer dependencies, no Effect development dependency, and no emitted `dist/effect.js` or
  `dist/effect.d.ts`.
- `BrowserEngine.close()` and disposed-handle state were not reverted. Their direct Promise tests
  remain in the standard Wasm-backed suite.

Local evidence on 2026-09-02:

- `npm ci --ignore-scripts`: PASS with three development packages; `npm ls effect --all` is empty.
- `check-sdk-types.sh`: PASS.
- `check-sdk-deletions.py`: PASS over 40 SDK source files.
- `check-sdk-generated.sh`: PASS.
- `check-sdk-headless.sh` against the exact pinned browser artifact: PASS, 111 tests / 27 suites.
  This includes browser close order/idempotency/failure cleanup, disposed headless state, all eleven
  console commands, torn-ack refusal, and async writer serialization.
- `sdk-package.sh check`: PASS, 59 files / 937.4 kB packed / 3.5 MB unpacked.
- `npm publish --dry-run --ignore-scripts`: PASS using a task-owned npm cache.
- A source/package search finds no Effect-library import, package specifier, dependency, entry,
  integration test, or package-gate invocation. Audio-processor uses of “effect” are unchanged.
- The no-argument headless gate first stopped before tests on the already-recorded macOS/Linux
  AudioWorklet digest difference. The successful run used the exact artifact from upstream browser
  qualification, without changing its pin.

Upstream evidence:

- Browser qualification [33621484909](https://github.com/misofm/engine/actions/runs/33621484909):
  PASS, including Chromium, Firefox, WebKit, and the shipped artifact.
- Release build [33621484916](https://github.com/misofm/engine/actions/runs/33621484916): PASS.
- Main CI [33621484972](https://github.com/misofm/engine/actions/runs/33621484972): the SDK
  typecheck, browser artifact/package, cross-target digest, and x86 jobs PASS. The aggregate run
  failed only in the pre-existing #159 wall-clock observation-cost test on unchanged Rust after
  its neighboring tests passed. Its raw timing evidence is attached to #159; the workload was not
  retried, and the repository's descriptive-benchmark rule keeps it from blocking unrelated SDK
  publication.

Adversarial verdict: **PASS**. No supported import or realtime-control capability was removed.
The package has one Promise-based ownership/control model, no Effect maintenance obligation, and
the acked-batch question remains answered by the direct console tests: no acknowledgement can be
returned before transport settlement or after a dropped command.
