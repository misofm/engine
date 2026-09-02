# Publishable TypeScript SDK with embedded Engine V1 artifacts

## Objective

Turn `sdk/` from a private source-vendoring tree into a publishable `@misofm/engine` package whose
tarball contains executable ESM, declarations, and the exact gated Engine V1 Wasm/browser artifact
set. A Node or Bun headless consumer must boot the bundled engine without cloning this repository,
building Rust, or supplying Wasm bytes.

This is the smallest closable production-distribution slice of issue #207. Fully typed live-control
coverage and any Effect integration are successor issues because each is independently useful and
neither may keep the distributable package open.

## Product contract

- `npm pack` produces a public, non-private `@misofm/engine` package with root, `headless`,
  `browser`, and `assets` export maps, each naming emitted JavaScript and declarations.
- The package contains the six-file Engine V1 browser artifact set produced by
  `scripts/build-web-audioworklet.sh`, including the simd128 Wasm module. No checked-in Wasm build
  output becomes source authority.
- A generated package manifest records SHA-256 and byte length for every embedded artifact.
- `createOfflineEngine(document, options?)` and `validate(document, options?)` use a verified,
  once-compiled bundled asset when no explicit `MisoEngineAsset` is supplied. Explicit asset
  injection remains supported for callers that share one compilation across sessions.
- Browser consumers receive stable URLs for every bundled artifact; higher-level browser lifecycle
  changes are outside this issue.
- The build is deterministic with respect to its inputs, refuses stale generated source assets,
  and does not publish tests, `node_modules`, Rust build output, or repository-only scripts.

## Scope

- `sdk/package.json`, build/typecheck configuration, package documentation and license payload.
- An `assets` SDK entry and the headless bundled-asset loader.
- A package-asset staging generator and one root build/check script.
- Focused package, type, and live-Wasm tarball smoke tests plus proportional CI wiring.

No control-surface redesign, browser pump, Effect runtime dependency, registry publication, release
signing, or application migration is in scope.

## Objective gates

1. `npm run build` emits JavaScript and `.d.ts` for all four public entry points and stages the
   exact six-file artifact set plus a manifest under `dist/assets/`.
2. `npm pack` contains only the declared package surface; its Wasm digest and byte count match the
   generated manifest and the artifact built in the same invocation.
3. From an unpacked tarball in a fresh temporary directory, Node imports all four public specifiers,
   loads the bundled asset with no caller-supplied bytes, boots a Session V1 fixture, renders one
   quantum, and disposes successfully.
4. Existing SDK generated-surface, behavioral, and strict type gates remain green.
5. A red mutation that removes or changes the embedded Wasm is rejected before instantiation.

## Decision record

- The Wasm remains a build output, not a tracked source file. It is embedded in the distributable
  tarball by the release build, preserving the engine build as authority.
- The package ships compiled ESM and declarations. Requiring every production consumer to compile
  vendored TypeScript is no longer the distribution contract.
- The tarball contains the full browser artifact closure, not only Wasm, because the packaged
  browser entry must never require artifacts from a different engine release.
- Registry publication itself remains an owner/release credential action. This issue proves that
  the produced tarball is publishable and self-contained.

## Evidence

Implementation attempt 1:

- `@misofm/engine@0.1.0` now emits ESM and declarations for `.`, `headless`, `browser`, and
  `assets`; it is public-package shaped and limits the tarball to the prepared `dist/` tree.
- `scripts/sdk-package.sh` reuses one Engine artifact directory, stages the six provenance-named
  artifacts, writes their SHA-256/byte manifest, packs, extracts, and runs the consumer smoke.
- The headless default verifies and compiles the packaged Wasm once per SDK lifetime; two default
  engines were proven to share the same `MisoEngineAsset`. Explicit asset injection is unchanged.
- The packed browser engine now defaults both artifact URLs to its own package and exposes the
  shipped host type instead of `unknown`.

Local gates on 2026-09-02:

- `check-sdk-headless.sh`: PASS, 101 tests / 25 suites against a live locally built Wasm.
- `check-sdk-types.sh`: PASS, including the shipped-host mirror.
- `check-sdk-deletions.py`: PASS over 35 source files.
- `sdk-package.sh check`: PASS; 55-file tarball, 932.4 kB packed / 3.5 MB unpacked, all four ESM
  entries import, a fresh strict TypeScript consumer resolves every declaration dependency, the
  embedded engine boots and renders, and a one-byte Wasm mutation refuses as `sdk.asset.digest`
  before compilation.
- Default bundled-asset caching: PASS; two sessions share one verified compilation.

Adversarial review:

- PASS on packaging closure, runtime/declaration resolution, asset integrity, failure retry, and
  explicit-asset compatibility.
- The repository's pinned AudioWorklet digest does not reproduce in this managed macOS sandbox;
  commit `f0509c3f` records the same known sandbox limitation. The pin was neither changed nor
  bypassed in product code. The upstream browser-Wasm job reused the exact artifact after its
  existing pin, ABI, and native-parity gates and passed.
- Final PASS: implementation commit `e9237134` passed CI run `33596094706`, browser qualification
  run `33596094657`, and release-build run `33596094709`. The distributable package is therefore
  self-contained and publishable with respect to this issue's artifact, packaging, and boot gates.
