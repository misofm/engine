# `@misofm/engine`

The Engine V2 TypeScript SDK is an agent-facing, zero-runtime-dependency control surface. The core
package publishes the engine catalog and ABI facts, constructs immutable Session V1 documents, and
encodes the fixed-width control records described by the shipped metadata.

## Determinism and validation

Session builders validate values against the generated catalog before emitting anything. A
successful `build()` returns frozen JSON-safe data plus canonical TOML; the TOML byte stream uses
engine ordering and Rust-compatible finite-`f32` formatting. Rebuilding from the JSON form must
produce identical bytes.

The four-stage `miso-engine-session-validator` CLI is the schema, typed-model, resource/canonical,
and builtin-preparation oracle. Its success is necessary but is not sufficient for native effect
parameters because that CLI currently stops before effect preparation. Phase 1's E5b corpus drives
the shipped zero-import Wasm module directly to cover that gap. The headless entry exposes the same
fresh-instance prepare/compile/NUL-diagnostic mechanism as `validateSession()`, the
authoritative complete-session check for SDK callers.

This distinction is tested explicitly. Compressor `ratio = 20.000002` is the next finite `f32`
above the catalog maximum. It passes all four CLI stages but is refused by the full Wasm compile
path as `effect.parameter.domain` at the effect leaf. SDK callers should treat local builder
validation as immediate feedback and full engine validation as final authority.

`SessionPlan.json` keeps ordinary values as ordinary JSON data. Its non-enumerable JSON serializer
uses the tagged object `{ "$miso.sdk.f32": "-0" }` only for negative zero, so
`SessionPlan.fromJson(JSON.parse(JSON.stringify(plan.json)))` preserves canonical `f32` bits.

Generated TypeScript is derived from the packaged JSON assets. `npm run check:generated` verifies
that neither the generated catalog/ABI/provenance modules nor their source assets have drifted.

## Headless Node and Bun

The `@misofm/engine/headless` entry drives the shipped zero-import Wasm module directly under Node
20+ and Bun 1.1+. `createOfflineEngine()` verifies SHA-256 before WebAssembly compilation, runs the
real prepare/compile pipeline, accepts planar `Float32Array` sources or RIFF/RF64 PCM16/PCM24/32f
WAV data, and exposes the typed console, meters, streaming render, and file output APIs.

`render(frames)` accepts any non-negative safe integer. The host renders whole engine quanta,
returns exactly the requested prefix, and retains the unused tail for the next call. Source/session
sample-rate mismatch is rejected before source submission; no implicit SRC exists. A compile or
render failure is sticky engine state: diagnostics are read as a NUL-terminated capacity prefix,
and recovery is dispose-and-recreate rather than an in-place retry. `validateSession()` always uses
a fresh Wasm instance for the same reason.

`renderToFile()` supports exact per-quantum `f32le-planar` records and interleaved stereo `wav32f`.
`wav16` remains deferred because v1 has no quantization/dither policy. Raw `{ toml }` validation is
fully engine-authoritative. For rendering, the frozen ABI cannot query a compiled source's nonzero
region, so raw-TOML source inputs denote a complete zero-origin region; use a typed `SessionPlan`
when a declared source region is not zero-origin.
