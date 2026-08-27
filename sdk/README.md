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
parameters because that CLI currently stops before effect preparation. The runtime
`validateSession()` path is authoritative for a complete session: it creates a fresh instance of
the shipped zero-import Wasm module, prepares and compiles the document, reads the NUL-terminated
`code\tpath` diagnostic prefix, and disposes the instance.

This distinction is tested explicitly. Compressor `ratio = 20.000002` is the next finite `f32`
above the catalog maximum. It passes all four CLI stages but is refused by the full Wasm compile
path as `effect.parameter.domain` at the effect leaf. SDK callers should treat local builder
validation as immediate feedback and full engine validation as final authority.

Generated TypeScript is derived from the packaged JSON assets. `npm run check:generated` verifies
that neither the generated catalog/ABI/provenance modules nor their source assets have drifted.
