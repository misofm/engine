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

Implementation and adversarial evidence will be recorded here before closure.
