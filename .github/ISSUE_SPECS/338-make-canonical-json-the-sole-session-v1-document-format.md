# Make canonical JSON the sole Session V1 document format

## Objective

Replace TOML with strict canonical JSON as the only durable Session V1 document accepted, emitted,
snapshotted, and published by the engine and SDK. Keep the typed session model, transactional
compilation, binary control protocol, source-content identity, PCM rings, and prepared realtime plan
semantics unchanged.

This is a prelaunch contract reset, not a compatibility layer. At the accepted checkpoint a TOML
session must be refused, no public or live internal API may claim that a session document or
snapshot is TOML, and every supported host must consume the same canonical JSON contract owned by
the Rust engine.

## Architectural context

There are three different boundaries and this issue changes only one:

| Boundary | V1 representation after this issue | Reason |
| --- | --- | --- |
| durable complete session and transactional snapshot | strict canonical JSON | agent authoring, inspection, source control and cross-platform interchange |
| live edits, automation, acknowledgements and snapshots' outer envelope | existing typed BTLV | bounded binary control transport with revisions, request IDs and backpressure |
| source and rendered audio | existing bounded planar PCM rings | realtime-safe bulk sample transport |

The complete JSON document is staged as bounded UTF-8 and parsed, validated, normalized and
compiled off the render thread. It is never incrementally parsed on the render thread and never
travels as a realtime command. A successful compile still produces the immutable control-plane
artifact used to prepare a replacement `PreparedRenderPlan`; publication remains transactional at a
documented block boundary.

gRPC, Protobuf, WebSocket framing, a binary session schema and a serialized prepared plan are not
part of this boundary. A remote service may adapt gRPC or WebSocket to the existing control protocol
outside the engine. If measured startup evidence later justifies a compiled-session cache, it must
be a disposable, version/capability/source-digest-bound derivative of canonical JSON rather than a
second authoring authority.

Session documents remain storage-blind. A source remains exactly its stable ID, canonical-PCM
content identity and declared shape. JSON does not add paths, URLs, FLAC bytes or resolver policy;
those belong in a separate host-specific source binding manifest and resolver/pump successor.

## Current coupling that makes this an engine migration

The SDK cannot implement the real change by translating JSON back to TOML before Wasm boot. Today:

- `crates/session` parses TOML, owns a TOML-specific typed name and diagnostic, writes canonical
  TOML, retains it in `CompiledSession`, and includes its byte size in resource estimates;
- `host-core`, the WebAssembly host and native/C paths pass TOML to that parser;
- the browser resource/status ABI publicly reports `sessionTomlBytes`;
- `protocol` chunks canonical TOML through `SESSION_SNAPSHOT_GET`, and committed mutations update
  that snapshot;
- builtins plan sealing hashes the canonical TOML snapshot;
- the SDK builder exposes `toToml()`, and headless/browser boot objects require it;
- `enginectl session build`, validators, runners, fixtures, browser qualification, benchmarks,
  generated declarations and active documentation all assume TOML.

The render graph, effects, source rings and DSP consume the compiled typed model or prepared plan;
they do not need a JSON parser and must not acquire one.

## Decision: the canonical JSON contract

The normative schema remains Session V1 and keeps the existing snake_case field names, tagged
record shapes, closed tokens, semantic defaults and canonical entity ordering. The JSON data shape
is the normalized object already exposed by the SDK builder, subject to the explicit 64-bit rule
below. This issue changes representation, not mixing behavior.

Canonical output is defined by the engine's schema walk, not by generic map iteration,
`serde_json::to_string`, JavaScript property-order accidents, or RFC 8785/JCS:

1. Bytes are UTF-8 without BOM, use LF only, and end with exactly one LF.
2. The writer uses two-space indentation, no tabs or trailing whitespace, schema-declared object-key
   order, and the existing canonical sorting of entity sets by stable ID and effect parameters by
   `(parameter_id, channel)`. Rack effects and automation segments retain declared order. A
   checked-in full-surface fixture freezes exact whitespace.
3. Input accepts JSON whitespace but refuses comments, trailing commas, multiple top-level values,
   non-JSON numeric tokens, invalid UTF-8, invalid escapes, unpaired UTF-16 surrogates, duplicate
   keys at every depth, missing keys and unknown keys. Duplicate keys never become last-write-wins.
4. Strings escape quote, reverse solidus and control characters deterministically; all other valid
   Unicode scalar values are emitted directly as UTF-8. The exact short escapes and hex case are
   frozen by adversarial fixtures so Rust and TypeScript emit byte-identical text.
5. Booleans and `u8`/`u32` schema integers use JSON booleans/numbers. Integer tokens have no
   fraction or exponent and retain their existing semantic bounds. `bit_depth` remains JSON number
   `16` or `24`, or string `"32f"`.
6. Every typed `u64` leaf -- `revision`, source `frames`, and automation `start_sample` and
   `end_sample` -- is a canonical unsigned decimal string matching `^(0|[1-9][0-9]*)$` and bounded
   by `u64::MAX`. This removes TOML's accidental signed-64 ceiling and preserves the complete engine
   domain through JavaScript without IEEE-754 loss. SDK ergonomic inputs may accept a safe integer
   `number` or a `bigint`, but normalization and durable JSON always use the decimal string.
7. Finite `f32` fields are JSON numbers emitted with the repository's proven shortest spelling that
   round-trips to the identical `f32` bits through both direct-`f32` and `f64`-then-`f32` parsing.
   Integral floats retain a decimal point, exponent notation is not canonical, and negative zero is
   emitted as `-0.0`. NaN and infinities remain invalid. This is why generic JCS and unguarded
   `JSON.stringify()` are not the canonical writer.
8. Noncanonical but semantically valid JSON input is accepted and snapshots to canonical JSON.
   Reparse/rewrite is byte-idempotent. The Rust writer is authoritative; the TypeScript writer must
   match it byte for byte over the full fixture and generated adversarial corpus.

### Frozen parser and lexical details

`parse_session_json` remains the public control-plane `&str` parser. Byte-oriented boundaries
validate bounded UTF-8 before calling it; UTF-8 decoding is not a private JSON parser. Rename the
existing boundary diagnostics to `web.document.utf8` and `capi.document.utf8`, both at `$`;
`session.utf8` remains the native runner's format-neutral preflight code. A UTF-8 BOM reaches the
JSON parser and refuses as one `json.syntax` diagnostic at `$`, spanning bytes `0..3`.

A duplicate object member is a strict-JSON syntax refusal: return exactly one `json.syntax`
diagnostic at the decoded path of the duplicated member, with the span on the second key. This
includes escape-equivalent names such as `"id"` and `"\u0069d"`. Duplicate values are never
visited, validated, or retained.

The parser has an explicit maximum nesting depth of 128, counting the root object as depth one and
incrementing for every nested object or array. Opening depth 129 refuses as one bounded
`json.syntax` diagnostic before building that subtree. The parser dependency's undocumented
default recursion limit is not the contract.

Canonical strings emit `\"`, `\\`, `\b`, `\t`, `\n`, `\f`, and `\r`. Other Unicode C0/C1 control
characters emit uppercase four-digit `\uXXXX`. Solidus is not escaped. Every other valid Unicode
scalar, including U+2028, U+2029 and non-BMP scalars, is emitted directly as UTF-8; canonical output
never emits surrogate pairs or `\UXXXXXXXX`. Objects use `": "`, commas after every member except
the last, two-space indentation at each depth, and no trailing commas; the root closes with exactly
one LF.

For unsigned JSON-number fields, a leading minus sign, including `-0`, is invalid. Fraction and
exponent forms are invalid for integer fields. Finite `f32` fields may accept semantically valid
noncanonical integer or exponent spellings and canonicalize them under rule 7.

Parser dependency selection is part of implementation, not an excuse to weaken this contract. Use
an exact-pinned, license-compatible parser only after a focused proof that duplicate members,
numeric lexemes, bounded allocation and useful syntax locations can meet the gates. Continue the
explicit typed walk rather than deriving a permissive model deserializer. Do not commit a new
general-purpose JSON parser unless the candidate dependency cannot meet a documented gate and Sol
approves that scope expansion.

## Public naming and compatibility reset

Rename format-bound implementation and public surfaces in the same final checkpoint:

- `SessionToml` -> `SessionModel`;
- `parse_session_toml` -> `parse_session_json`;
- `canonical_session_toml` -> `canonical_session_json`;
- `CompiledSession::canonical_toml()` and storage -> `canonical_json()`;
- syntax diagnostic `toml.syntax`/`TomlSyntax` -> `json.syntax`/`JsonSyntax`;
- protocol `canonical_toml_chunk` -> `canonical_json_chunk` while retaining snapshot opcode
  `0x0002`, field number `3`, chunk offsets, length and EOF framing;
- browser status/resource `sessionTomlBytes` -> `sessionDocumentBytes`, because that field measures
  staged input bytes rather than a canonical-format allocation;
- SDK `toToml()` -> `toJson()` for canonical text. Keep `toJSON()` as the normalized object hook and
  document the case-sensitive distinction;
- examples, arguments and locals named `toml` -> `document`, `json` or `model` according to what
  they actually hold.

The source-level C ABI reset also renames
`miso_engine_v1_compile_limits.maximum_toml_bytes` to `maximum_document_bytes` and the compile
function's `toml`/`toml_bytes` parameters to `document`/`document_bytes`. Preserve the struct
offset, width, exported symbol and calling convention. Rename `capi.toml.limit` to
`capi.document.maximum_bytes`, `capi.toml.utf8` to `capi.document.utf8`, and `web.toml.utf8` to
`web.document.utf8`. Regenerated Rust/C/TypeScript declarations and layout fixtures must agree;
prelaunch source compatibility with the retired field names is intentionally not retained.

Do not retain deprecated aliases, accept both grammars, sniff the first byte, add a format flag, or
bump the live product to V2. This repository is prelaunch and its current contract remains V1.
Historical issue specs, archived evidence and research records that truthfully describe TOML at an
older checkpoint are not rewritten. The active architecture guide, normative session/schema and
control-protocol documentation, live code, generated artifacts and current examples are updated.

The numeric BTLV message/opcode/field identities do not change merely because field 3 now carries
JSON. Old prelaunch clients are intentionally incompatible and must be rebuilt from the new
generated declarations and assets; there is no wire negotiation for the retired TOML meaning.

## Smallest closable product slice

One closable slice must make the engine, native/C and browser hosts, protocol snapshots, SDK,
`enginectl`, representative tools and shipped fixtures agree on JSON. Merging an SDK-only
translation, an engine that emits JSON but still accepts TOML, or a mixed fixture tree would create
two authorities and is not useful product state.

Large historical-corpus rewriting, extended target matrices, deep fuzz duration, benchmark
framework changes and source-binding playback are explicitly separated. Representative launch
fixtures and every shipped/runtime fixture are part of this slice; archived evidence prose is not.

## Implementation plan

### 1. Freeze the representation and migration inventory

- Replace the existing normative session schema document in place or rename it without losing Git
  history. Include one minimal and one full canonical JSON example and the exact canonicalization
  rules above.
- Add a checked JSON Schema 2020-12 artifact with `additionalProperties: false` at every object.
  Treat it as editor/tooling assistance: the Rust parser remains authoritative for duplicate keys,
  raw numeric spelling, exact `f32`, stable IDs, cross-references and ordering.
- Amend `AGENTS.md` from strict versioned TOML to strict versioned canonical JSON while preserving
  transactional compilation and snapshot requirements.
- Produce a checked inventory classifying every TOML-named hit as live contract, current fixture,
  generic Cargo configuration, or immutable historical evidence. Only the first two categories are
  migration targets; `Cargo.toml` and unrelated TOML are obviously not.
- Freeze semantic before/after pairs for the minimal, full-surface, 256-track, browser command,
  observation and enginectl-stems sessions before deleting the old parser.

### 2. Migrate the Rust session authority

- Rename the typed model and format-bound APIs listed above without changing field IDs, semantic
  validators, cross-reference ownership or compile ordering.
- Replace the TOML frontend with a strict bounded JSON frontend that retains structured diagnostic
  paths and useful source locations. Syntax failure returns one bounded `json.syntax`; typed/schema
  failures retain their existing codes and JSONPath-like paths.
- Implement `canonical_session_json` as another consumer of the existing `VisitModel` canonical
  walk. Keep exactly one field/token registry shared by canonical JSON and BTLV emission.
- Store canonical JSON in `CompiledSession`, update canonical byte estimates and cap preflight, and
  retain the rule that cap checks precede large canonical allocation, cloning, sorting and index
  construction.
- Remove the session crate's TOML dependency and pin/audit any JSON dependency. Update `Cargo.lock`
  only through the normal resolver; do not hand-edit it.
- Rename format-specific tests, fuzz targets and policy checks, preserving the existing invalid
  semantic matrix and adding JSON grammar/duplicate/numeric/Unicode cases.

### 3. Carry JSON through hosts and the control protocol

- Make `host-core`, WebAssembly boot, C/native compilation and the session validator accept the
  same bounded JSON bytes and invoke the same engine parser. No host gets a private parser.
- Rename UTF-8/document diagnostics and `sessionDocumentBytes`; regenerate ABI layout, TypeScript
  declarations and packaged pins through existing generators. Preserve offsets and widths unless
  a measured requirement proves otherwise.
- Keep `SESSION_SNAPSHOT_GET` request and response framing, numeric opcode/tag identities,
  revision behavior and chunk boundaries. Change only field/API semantics to canonical JSON and
  prove a multi-chunk snapshot reparses after both initial boot and a committed transaction.
- Update the plan seal to hash canonical JSON. Prove semantic render output is unchanged and that a
  plan sealed for one canonical snapshot cannot be paired with a different session. Do not invent a
  format-independent plan identity in this issue unless an existing public invariant requires it.
- Recalculate the browser/Wasm parse transient projection from measured JSON-parser behavior.
  Browser and headless Wasm retain the exact 1 MiB staged-session-document ceiling and refuse before
  UTF-8 decoding or parser allocation. The C ABI retains its caller-supplied
  `maximum_document_bytes`, and the native PCM runner retains its separate 4 MiB session-file cap.
  The enginectl authoring request retains its separate 4 MiB input-envelope bound; the session it
  produces must still pass the embedded engine's 1 MiB staged-document limit. Do not describe these
  distinct boundaries as one engine-global cap, and do not mechanically retain multiplier `80`
  without evidence.

### 4. Migrate the SDK and `enginectl`

- Make `SessionDocument` accept canonical JSON bytes/text or a builder with `toJson()`. Raw strings
  and bytes still go directly to the engine so the engine remains the acceptance authority.
- Preserve `SessionBuilder.toJSON()` as a frozen normalized data model; implement `toJson()` with a
  dedicated canonical writer that preserves `-0.0`, exact `f32` round trips, schema order and the
  decimal-string sample fields. Do not delegate canonical bytes to `JSON.stringify()`.
- Update headless, scratch/browser and reload paths together. Both browser boots must submit the
  exact same bytes and derive shape from the engine as today.
- Make `enginectl session build --request` and `--stems` publish canonical `.session.json` content;
  stdout mode emits only JSON session bytes and file mode still emits its separate compact JSON
  receipt after atomic publication. The receipt's `output.sha256` remains the digest of the emitted
  session document; each `stems[].content` remains the digest of canonical decoded PCM. Because
  stdout can now contain either a session document or receipt depending on output mode, retain and
  clearly document the established stream framing.
- Preserve stem discovery, PCM hashing, receipt path provenance, no-clobber/overwrite, publication
  ordering, failure JSON, exit classes and offline/noninteractive behavior.

All durable `u64` leaves in `SessionModel` and `toJSON()` are decimal strings: `revision`, source
`frames`, `start_sample`, and `end_sample`. Author-facing TypeScript builder inputs accept either a
nonnegative safe integer `number` or a `bigint` through `u64::MAX`; normalization always produces a
string. Because an enginectl JSON request cannot carry a JavaScript `bigint`, its corresponding
fields accept either a nonnegative safe integer JSON number or a canonical unsigned-decimal string
through `u64::MAX`, then pass the exact integer to the builder. Unsafe JSON numbers, signs,
whitespace, leading zeroes and overflow refuse before builder construction.

### 5. Convert live consumers and fixtures without forging history

- Before removing the old parser, use it once as a migration oracle to parse every live session
  fixture and write JSON through the new Rust writer. Compare normalized typed models and semantic
  BTLV walks, then remove the converter from the shipped product.
- Rename live `.toml` session fixtures to `.json` across browser host tests/qualification, C API,
  protocol, graph/effect/builtins compilers, source/native runner, tools, fuzz seeds, SDK tests and
  examples. Update static servers/MIME handling where necessary.
- Re-pin byte counts and SHA-256 values only where they identify live generated artifacts or
  fixtures. Never mass-edit accepted issue specs, archived benchmark records or derivation prose.
- Update `session-validator`, native PCM runner examples, SDK README, control protocol docs and
  active architecture documentation. The native runner's source-root resolver continues matching
  canonical PCM identities and never interprets a JSON string as a path.
- Add a policy check that rejects live session `.toml`, TOML parser dependencies and retired public
  names outside an explicit historical allowlist, while allowing Cargo's own `.toml` files.

### 6. Package and release closure

- Rebuild the Wasm engine and generated ABI/catalog declarations through existing reproducible
  scripts; stage the exact accepted artifacts into the npm package and regenerate its artifact
  manifest.
- Verify an extracted npm tarball with repository discovery and Cargo unavailable: build a session
  through both request and FLAC-stems modes, compare the authored canonical JSON bytes exactly,
  boot it headlessly, submit PCM, render, and reject mutated engine/decoder artifacts. The Web and
  headless SDK intentionally expose no session-snapshot operation. Sol explicitly ruled that this
  package boundary proves exact canonical author output plus engine boot; canonical snapshot
  reconstruction remains separately proved at the Rust/C control-protocol boundary. This ruling
  does not invent a snapshot or broaden the frozen Web ABI.
- After the extracted package candidate passes, regenerate the four previously supplied external
  dogfood outputs (`ghost`, `play-me`, `war`, and `wide-open`) as `.session.json` when their owner
  supplies those source roots. They are not repository inputs and must not be fabricated. Their
  absence from this workspace does not block implementation review, but issue closure requires
  either the real regeneration evidence or an explicit owner decision moving that deployment
  evidence to a successor.

## Required surface inventory

The implementation owner must confirm this inventory against current `main` before editing:

- session authority: `crates/session/{Cargo.toml,src/{lib,model,parse,canonical,value,diagnostic,
  visit,validate,estimate,compile}.rs}`, its tests and fuzz targets;
- engine adapters: `crates/host-core/src/prepare.rs`, `hosts/host-web/src/{lib,ffi}.rs`, C API
  runtime/header/tests, browser worklet host/loader and boot-budget gates;
- transactional control: `crates/protocol/src/{model,controller,message_wire,schema}.rs`, protocol
  conformance fixtures and active registry/semantics documentation;
- format-derived preparation: builtins session sealing and any audit/benchmark code reading
  `CompiledSession::canonical_toml()`;
- SDK: `sdk/src/core/session.ts`, headless/browser engine entry points, generated ABI declarations,
  `sdk/src/enginectl.ts`, `sdk/src/cli/session-request.ts`, README and all relevant tests;
- tools: session validator, native PCM runner, graph fixtures, audit/bench/console workload inputs,
  build/package scripts and browser static-serving harnesses;
- live data: session fixtures under `fixtures/session/v1`, host-web browser/qualification sessions,
  native-runner sessions, SDK fixtures, generated ABI layout and package asset manifests; and
- active policy/docs: `AGENTS.md`, `docs/SESSION_SCHEMA_V1.md`, current control/runner/SDK docs and a
  new live-name allowlist check. Historical issue specs, derivations and archived evidence are
  excluded unless a live gate directly consumes them.
- `crates/capi/src/abi.rs`, `crates/capi/include/miso_engine_v1.h` and C consumers;
- `sdk/src/enginectl.ts`, `sdk/src/cli/{session-request,stems}.ts`,
  `sdk/test/{enginectl-cli,package-tarball-smoke}.mjs`;
- `.claude/skills/author-session/SKILL.md`, session fuzz workflow seed paths, console/builtins
  benchmark fixture-path validators, and active operator scripts.

The implementation inventory is derived from exact baseline `51468d5d`, not from a filename suffix
alone. The corrected inventory contains 25 session-document TOMLs: 14 under
`fixtures/session/v1`, five native-runner sessions, and six host-web browser/qualification
sessions. The ten `fixtures/builtins/v1/benchmark/*.toml` inputs are builtin workload configuration,
not Session V1 documents, and remain generic tooling TOML. Additional live format-bound names occur throughout the C
ABI, SDK/enginectl, protocol, tools, workflows, policy scripts and the `author-session` skill.
The checked inventory produced in the first checkpoint is authoritative and must distinguish these
from Cargo/configuration TOML and immutable historical evidence.

Historical benchmark records may retain old `.toml` identities only through the explicit
historical allowlist; new workload inputs must use JSON.

## Objective gates

1. Minimal and full-surface canonical JSON parse, validate, compile, snapshot and reparse
   byte-identically in Rust; the SDK writer emits those exact bytes.
2. Equivalent documents with shuffled object keys, insignificant whitespace, escaped versus direct
   Unicode and shuffled order-insensitive entity arrays normalize to the same canonical bytes.
3. Duplicate keys at root and every nested record, unknown/missing fields, wrong types, comments,
   trailing commas/data, BOM, invalid UTF-8/escapes/surrogates and invalid numeric forms refuse with
   bounded exact code/path behavior and no compiled artifact.
4. Directed and generated finite-`f32` values, including both known double-rounding cases,
   subnormals, minimum/maximum finite values and `-0.0`, round-trip bit-exactly through Rust and
   TypeScript. NaN/infinities refuse. Changing the sign of zero turns the identity/equality gate red.
5. Minimum/maximum admitted integers and decimal-string 64-bit fields cross Rust/TypeScript without
   precision loss. Cover `0`, `i64::MAX`, `i64::MAX + 1`, and `u64::MAX`; overflow, negative, plus
   sign, whitespace, leading zero and JSON-number substitutes refuse typed.
6. A TOML session, format sniffing, a deprecated alias or an SDK JSON-to-TOML translation fails the
   sole-format policy gate. No live product surface claims V2.
7. Initial and post-transaction `SESSION_SNAPSHOT_GET` responses reconstruct exact canonical JSON
   across one and many chunks, including a byte page that splits a multibyte UTF-8 scalar. Revision,
   conflict and atomic-failure behavior and numeric BTLV tags remain unchanged; an acknowledgement
   can never precede a dropped or uncommitted mutation.
8. Failed parse, validation, resource preflight, downstream preparation or replacement leaves the
   prior compiled model, plan, revision and JSON snapshot intact.
9. Converted representative sessions yield the same normalized semantic walk, prepared graph,
   source/track map, resource intent and bit-exact rendered PCM as their pre-migration baselines.
   Only format-derived byte counts, digests, seals and diagnostics may move.
10. Web scratch and AudioWorklet boots, headless Wasm, native/C ABI and session validator all accept
    the same JSON fixture and report the same shape. No parser, writer, allocation, I/O, logging,
    lock or syscall enters render.
11. `sessionDocumentBytes`, parse projections and compiled canonical-byte accounting are exact at
    zero/minimal/maximum sizes. The engine's 1 MiB document ceiling still refuses before parsing;
    the enginectl request envelope retains its separate 4 MiB bound. Representative peak allocation
    stays within the newly evidenced projection.
12. `enginectl` request and stems modes publish engine-accepted JSON atomically; raw stdout remains
    one session document with empty successful stderr, while file mode emits one receipt after
    publication. PCM identities and path receipts retain their established meanings.
13. The npm tarball contains the newly built JSON-capable engine, declarations and correct artifact
    pins and works without Cargo, repository lookup, subprocess media tools or network access.
14. Native workspace tests, warning-denied Clippy/rustdoc, strict TypeScript, SDK/package smoke,
    browser/Wasm correctness, protocol/C ABI conformance, policy checks and launch target builds are
    green at one exact checkpoint.

## Required adversarial mutations

- Accept the second occurrence of a duplicate key, including escape-equivalent `"id"` and
  `"\u0069d"`.
- Serialize object fields by map insertion order, reorder one rack/automation sequence, or stop
  sorting one entity set.
- Use `JSON.stringify()` so `-0.0` becomes `0`.
- Parse every numeric token through `f64` so a known double-rounding vector moves.
- Emit any typed `u64` as a JavaScript number.
- Leave TOML acceptance or one `toToml`/`canonical_toml_chunk` live alias reachable.
- Change snapshot field number 3 or acknowledge a transaction before its JSON snapshot is retained.
- Feed different bytes to browser scratch and AudioWorklet boots.
- Keep the old parse multiplier without measuring, or move its cap check after parser allocation.
- Hash FLAC/container bytes when regenerating a stems session.

Each mutation must make its named gate red, and production must be restored before the checkpoint.

## Proportional qualification

This is not an SDK-only change. Run focused gates after each local checkpoint, then one coherent
cross-workspace qualification at the batch boundary:

- session parser/canonical/diagnostic/resource tests and bounded parser/compiler fuzz smoke;
- protocol snapshot/transaction and C ABI conformance;
- host-web native tests, Wasm build, direct browser oracle and AudioWorklet correctness;
- SDK strict types, builder/headless/browser/enginectl tests, generated/deletion gates and extracted
  package/tarball smoke;
- graph/effect/builtins/source representative compile and bit-exact render parity fixtures;
- native host, `aarch64-apple-ios`, `aarch64-linux-android`, `wasm32-unknown-unknown` scalar and
  `+simd128` launch build gates;
- workspace format, warning-denied Clippy/rustdoc, policy checks and `cargo test --workspace
  --all-targets` once after focused failures are exhausted.

Freeze the existing representative 256-track session workload in JSON before timing. After all
functional gates pass, run exactly one benchmark invocation with one warmup and two measured rounds
for parse-plus-canonical and compile. Record JSON fixture bytes/hash/counts, peak allocation,
toolchain/target and the pre-migration descriptive baseline; do not tune or retry. The result is
descriptive unless the 1 MiB engine boot budget or an existing named launch ceiling is missed. Deep
fuzzing beyond the bounded parser/compiler smoke remains nightly qualification rather than a reason
to keep the product slice open.

The single descriptive benchmark invocation occurs only after functional acceptance. It blocks
closure only if the 1 MiB boot budget or another already-named launch ceiling is missed. A
preflight, runner, persistence or post-workload defect preserves raw output and moves to a bounded
tooling successor; it does not keep this otherwise functional representation migration open.

No DSP benchmark or listening pass is warranted because session representation must not change
audio arithmetic. Bit-exact representative render parity is the discriminating gate.

## Non-goals and successors

- embedding filesystem paths, URLs or FLAC bytes in the portable session;
- implementing the receipt/source-manifest resolver, decoder and PCM pump;
- gRPC, WebSocket or Protobuf at the embedded engine boundary;
- replacing BTLV live control or PCM rings with JSON;
- incremental/streaming JSON parsing on render;
- a binary session authoring format or serialized `PreparedRenderPlan`;
- TOML compatibility, automatic migration in the shipped runtime or a V2 namespace;
- changing default gains/routing, source identity, effect parameters or DSP;
- rewriting historical issue/evidence records; or
- inventing new benchmark thresholds, extended target/device matrices or long fuzz campaigns.

Create a bounded successor for a typed source-binding manifest plus native/browser resolver/decoder/
pump convenience. Create another only if measured JSON startup cost warrants a disposable compiled
cache. Neither successor blocks this representation migration.

## Risks and stop conditions

- **Two authorities:** HOLD if any supported path parses JSON in the SDK and TOML in the engine, or
  if native and browser hosts use different parsers.
- **Silent data loss:** HOLD if duplicate members overwrite, a 64-bit field passes through a JS
  number, `-0.0` changes sign, or a numeric spelling changes an `f32` bit.
- **Protocol drift:** HOLD if snapshot numeric tags/framing change without an explicit new wire issue
  or if transaction success can precede retention of the canonical JSON snapshot.
- **Realtime contamination:** HOLD if JSON work reaches render or replacement reclamation.
- **Unbounded parser work:** HOLD if the document cap/preflight no longer precedes parser allocation
  or adversarial nesting/diagnostics exceed established bounds.
- **Scope explosion:** if exact spans require a new general parser, or a second large fixture or
  benchmark framework becomes necessary, stop after one bounded proof and create a focused
  successor rather than hiding it inside attempt 2 or 3.
- **False completion:** HOLD if live fixtures/package assets remain TOML, generated ABI still says
  `sessionTomlBytes`, or dogfood JSON cannot boot from the extracted npm package.

## Delivery and review workflow

1. Start from synchronized `main` on one `codex/batch-*` branch; keep the user's existing worktree
   and unrelated changes untouched.
2. Sol-high approves this scope and exact representation before implementation.
3. Implement one coherent attempt with the requested implementation agent. Keep at most one
   uncommitted tranche: representation/model, engine/host/protocol, SDK/tools/fixtures, then package
   closure, with focused green gates and exact-path root commits between tranches.
4. Do not push intermediate checkpoints or open/update a PR while the CI-conscious batch is
   accumulating. Temporary local dual-format code is permitted only as a fixture conversion aid and
   must be gone before the first review checkpoint.
5. Run the proportional functional gates once at the coherent boundary, then the single descriptive
   benchmark invocation. Preserve raw output if post-workload tooling fails.
6. Obtain fresh Sol-high adversarial review against every gate. Permit at most two bounded Sol
   corrections; after attempt 3 HOLD, preserve evidence, split/rescope and stop.
7. Push the batch branch at most once, open one PR, merge only with required checks green, then push
   `main` once. Synchronize this issue's evidence after the upstream commit and close only after the
   remote state is verified.

## Planning evidence

Sol-high architecture review on 2026-09-02 inspected synchronized baseline `51468d5d` and traced
the complete document lifecycle through the Rust session authority, host-core, C/native and
WebAssembly entry points, protocol snapshots and mutations, builtins sealing, SDK
builder/headless/browser paths, enginectl, validators/runners, package artifacts, fixtures,
resource accounting and active policy.
It found that an SDK-only JSON facade would leave TOML as the engine and snapshot authority and
therefore rejected that shape.

The review approved engine-native canonical JSON as the sole prelaunch Session V1 durable format,
with the existing typed model and realtime BTLV/PCM boundaries preserved. It also ruled that the
canonical writer must remain session-specific because generic JCS/`JSON.stringify()` cannot carry
the repository's exact `f32 -0.0` and cross-language 64-bit requirements without semantic loss.
No implementation, dependency change, fixture rewrite or performance claim is made by this
planning checkpoint.

## Attempt 1 implementation evidence — tranche 1

The session-authority checkpoint uses exact-pinned `jstrict 0.14.0` without default features and
retains a contract-owned preflight for decoded duplicate members and the explicit depth-128 limit.
The dependency preserves raw numeric source spans through its byte-based `CodeMap`; the typed walk
therefore applies the existing exact-f32 reader to the original lexeme rather than an intermediate
`f64`. Direct runtime serde, serde_json and TOML dependencies are absent. `serde_json 1.0.151` is a
dev-only mutation/artifact-test dependency.

Focused tests cover root and nested duplicate families (including escape-equivalent keys, exact
second-key byte spans, and malformed/huge values that are never visited), opening-depth-129
refusal, raw `-0`, exponent and huge-exponent typed diagnostics, all u64 leaves, strict JSON
syntax/Unicode cases, multibyte/newline byte spans, exact canonical fixtures, preserved semantic
diagnostics and resource-preflight ordering. The checked Draft 2020-12 schema test recursively
requires `additionalProperties: false` on every object schema and checks the four-way migration
inventory.

Allocation-call observations for one, 256 and 4,096 tracks were respectively: raw parser
`84/13873/221245`; preflight plus owned model beyond that raw parse `305/53600/856160`; complete
parse `389/67473/1077405`; canonical writer `19/26/30`; compiler `45/2636/41394`; estimator
`0/0/0`. The smallest documented conservative integer-slope parse envelope covering those points
with at least 32 fixed calls of headroom is `263 * tracks + 192`. This is allocation projection
evidence, not the issue's one-shot timing benchmark; that benchmark was not run.

At this checkpoint `cargo test --locked -p session --no-fail-fast`, native and
`wasm32-unknown-unknown` session checks, formatting, and warning-denied session Clippy are green.
The two explicitly ignored timing/exhaustive qualification tests were not authorized or run. This
records tranche-1 evidence only and makes no claim that the later host/protocol/SDK/package
migration or full issue acceptance is complete.

## Attempt 1 implementation evidence — tranche 2

The host/protocol checkpoint routes native host preparation, the C ABI and browser boot directly
to the single Rust JSON authority. The source-level C limit is now `maximum_document_bytes` and
compile parameters are `document`/`document_bytes`; the exported symbols, layout offsets, widths
and calling convention are unchanged. Browser bindings expose `sessionDocumentBytes` (Rust
`session_document_bytes`) and invalid UTF-8 is `web.document.utf8`. The generated ABI layout and
declaration mirrors agree with those source names.

The browser cap is checked before UTF-8 decoding or parser construction. Allocation observations
for representative documents were: 447 input bytes / 6,588 peak bytes (14.738x), 3,438 / 40,551
(11.795x), 177,885 / 2,172,554 (12.213x), 532,317 / 5,720,586 (10.747x), and a one-MiB whitespace-
padded maximum document / 5,720,586 (5.456x). The smallest conservative integer multiplier with
more than ten percent headroom over the observed maximum is therefore 17. The browser tests keep
raw parse, retained projection, exact retained, compilation and largest-allocation refusals
distinct; this allocation projection was not the issue's one-shot timing benchmark.

Protocol opcode `0x0002`, field 3 and its framing remain unchanged while the source/API name is
`canonical_json_chunk`. A focused controller test reconstructs and reparses the initial snapshot,
commits a Unicode-bearing transaction, then reconstructs the retained post-transaction snapshot
through one-byte pages that split UTF-8 scalars. The acknowledged revision is observed only after
the canonical JSON is retained. Existing transactional fault and saturation tests remain green and
continue to prove that failed preparation/publication preserves the prior model, plan, revision and
snapshot. Builtin plan seals now hash `CompiledSession::canonical_json()` bytes.

At this checkpoint the combined focused test command
`cargo test --locked -p protocol -p host-core -p capi -p host-web -p builtins-compiler
--no-fail-fast` is green. `scripts/check-capi-abi.sh`, the generated ABI-layout checks, the browser
expected-resource checker and its 26 red-mutation self-test, workspace policy, the scalar plus
simd128 protocol Wasm parity check, and a `wasm32-unknown-unknown` check of all five changed packages
are green. The browser simd128 fixture reports 1,919 document bytes, 24,744 bridge-retained bytes
and the unchanged target-specific graph/effect/source rows; native and target-independent witness
rows agree. This records tranche-2 evidence only; SDK/tool/package migration and final package
artifact rebuilding remain explicitly deferred.

The hermetic AudioWorklet JavaScript tests pass. The package-seal wrapper was also exercised after
the source rebuild and stopped only at the intentionally deferred checked-in Wasm artifact digest:
expected `6ddf154d02fcb4dfaa1a397280a28ab9f38b0cd6dff466a316f120266ce2223f`, observed
`6dcd9ced2daeb886843a764bcc6abc0b4f1b2c7a50af1ed91151a5ab366461e5`. Per the tranche boundary,
the packaged artifact and its digest are not changed here and must be rebuilt/resealed in tranche 4.

## Attempt 1 implementation evidence — tranche 3

The SDK builder now has a dedicated `toJson()` canonical writer whose field order, final newline,
escaping and finite-f32 spelling are byte-checked against the Rust authority. `toJSON()` remains the
normalized object surface and represents every durable u64 leaf as a decimal string. Builder inputs
accept safe nonnegative JavaScript numbers or bigint values through `u64::MAX`; enginectl request
JSON accepts the same safe-number domain plus canonical decimal strings through `u64::MAX`, and
passes the resulting session document directly to the engine authority without format sniffing or
translation. Headless and browser source paths use the same document boundary.

All 25 live Session V1 TOML documents identified by the corrected inventory were replaced or
deleted: 14 session fixtures, five native-runner sessions and six host-web test/qualification
sessions. The ten builtins benchmark `.toml` files are workload configuration records, not Session
V1 documents; their session-template references and digests now identify canonical JSON. Native
runner generation, console intended/mono derivation and their checkers transform JSON structures
and delegate canonical byte production to `session-validator`. Rust consumers, fuzz seeds,
workflows, active operator scripts and documentation were migrated. The author-session skill is a
canonical-JSON-only workflow. `scripts/check-session-policy.sh` gates the sole runtime format and
uses an explicit exact-file allowlist for historical evidence that must retain its original text.

Focused evidence is green for native-pcm-runner (19 tests), session-validator (eight validator plus
one skill test), source (55 tests), parameter-metadata (seven ABI-layout plus three round-trip
tests), console/native fixture regeneration and mutation checks, SDK strict types/generated/assets,
the SDK builder/headless/browser source tests, and a source-assembled enginectl package (21 tests,
including maximum-u64 requests). Workspace all-target checking, changed-package warning-denied
Clippy, formatting, session policy, deletion-policy mutations, generated checks, fixture checks and
C ABI checks are green. A source-built Wasm SDK run passed 130 of 131 tests; the remaining harness
case is environmental under the root test user: it expects a non-executable file to produce the
tool's exit 2, while the shell refuses execution first with exit 126. No functional session-format
assertion failed.

Checked-in package Wasm rebuilding, package tarball closure and the intentionally stale packaged
Wasm digest remain tranche 4 work. No package artifact was rebuilt and the one-shot descriptive
benchmark was not run in this tranche.

## Attempt 1 implementation evidence — tranche 4

The reproducible Web build now seals the JSON-capable AudioWorklet module at 2,648,237 bytes with
SHA-256 `6dcd9ced2daeb886843a764bcc6abc0b4f1b2c7a50af1ed91151a5ab366461e5`.
The six-file Engine artifact closure and four-file FLAC decoder closure regenerate cleanly, and the
SDK ABI layout, parameter metadata, generated modules, generated surface and package asset manifest
all agree with their source authorities. The browser boot-budget gate was corrected from the
retired TOML parser's 80x projection to the measured 17x JSON projection recorded in tranche 2.

`scripts/sdk-package.sh check` produced and extracted a 69-file npm tarball (1.1 MB packed, 3.9 MB
unpacked) with no runtime dependencies or test/repository files. Its independent smoke gate imports
every public entry point and resolves every declaration from the extraction, then runs request and
FLAC-stems session builds with an empty `PATH`, an isolated nonexistent `HOME` and the package as
its working directory. Both outputs are exact canonical JSON snapshots; both boot through the
embedded headless Wasm, accept submitted PCM and render non-silence. A one-byte engine-Wasm mutation
is rejected by the package manifest before compilation, and a one-byte decoder-Wasm mutation exits
as `internal.packaged_decoder` without publishing a session. Thus neither Cargo, repository
discovery nor an external media subprocess is available to make the extraction pass.

The source-built SDK gate is green at 131/131 tests. Its formerly failing root-only permission case
now copies the shell gate and expected `sdk` sibling into the mode-0755 scratch tree before dropping
to uid/gid 65534, so it reaches the deliberately unsearchable artifact directory and preserves the
exact exit-2 assertion instead of failing to traverse `/root` with exit 126. The complete shipped-
artifact static/object gate, JSON 17x boot high-water/refusal gate, formatting, generated checks and
sole-session-format policy are green. The independent raw-Wasm render oracle, browser fixture/runner
static check, built-artifact expected-resource agreement, its 26-mutation self-test and the hermetic
AudioWorklet suite are also green after replacing the last TOML-shaped fixture substrings in that
checker with structural JSON assertions.

The coherent pre-benchmark qualification is green: `cargo test --locked --workspace --all-targets
--no-fail-fast`, warning-denied workspace Clippy with all features, warning-denied workspace rustdoc,
and formatting all pass. The native host runs with the pinned AVX2 backend; host-mobile builds in
release mode for `aarch64-apple-ios` and `aarch64-linux-android`; and the complete CI package set
builds in release mode for both scalar (`-simd128`) and SIMD (`+simd128`)
`wasm32-unknown-unknown`. Workspace, session-format, SDK deletion plus its 37 mutations, generated
SDK and C ABI policy gates pass.

The four external dogfood source roots (`ghost`, `play-me`, `war` and `wide-open`) were not supplied
in this workspace, so no outputs were fabricated. Closure still requires an explicit owner decision
to defer that deployment evidence if the real roots remain unavailable. The one-shot descriptive
benchmark was not run in this tranche.

## Attempt 1 one-shot descriptive benchmark

After every runnable functional gate above was green and checkpoint `8c107f2d` was clean,
`scripts/run-session-benchmark.sh` was invoked exactly once at `2026-09-03T01:30:42Z`. The wrapper
completed successfully without retry or tuning: one fixed warmup phase and two measured rounds for
each of parse-plus-canonical and compile. The generated 256-track canonical JSON fixture was
573,833 bytes with SHA-256
`1a6357221dd631f5df594b82a1ce3138a9484794271e0423e0bcc85812e1d7af`; it contained one source,
256 tracks, one output, 256 routes, 256 automation programs, 256 effects, 256 effect parameters and
256 automation segments.

On the AMD EPYC 9355 x86_64 Linux host with rustc 1.97.1 / LLVM 22.1.6, release opt-level 3, LTO
off and 16 codegen units, parse-plus-canonical recorded p50 5,529,402 ns and 5,529,948 ns (p95
5,565,499 ns and 5,558,493 ns). Compile recorded p50 714,381 ns and 704,479 ns (p95 721,494 ns and
713,153 ns). The only missing environmental metadata was power source; governor was `performance`.
These are descriptive observations with no decision threshold and miss no named launch ceiling.

The timing runner records fixture bytes/hash/counts and toolchain/target but does not sample peak
heap bytes. The separately frozen allocation evidence therefore remains the applicable observation:
at 256 tracks, raw JSON parse made 13,873 allocation calls, preflight plus owned-model construction
made 53,600, total parse made 67,473, canonicalization made 26, compilation made 2,636 and resource
estimation made zero. Browser peak-byte/high-water evidence and the 17x bound are recorded in tranche
2. No retained pre-migration timing record exists in the repository or GitHub issue history, so no
before/after speed claim is made and a prohibited second invocation was not manufactured.

## Attempt 2 correction evidence — code checkpoint before live browsers

The SDK canonical writer now chooses an explicit key-order table for every Session V1 record
family and chooses every tagged variant by its `kind`; object construction order cannot affect
output. It also checks that a supplied normalized record has exactly the keys declared for that
shape. A red-mutation test recursively reconstructs every object in reverse insertion order and
still obtains byte-identical Rust canonical JSON. The tagged corpus reaches native and CID effect
identities, none and routed sidechains, track and submix-output route sources, submix-input and
output-input destinations, and pan and matrix track variants.

`fixtures/session/v1/canonical-writer-corpus.json` is a bounded checked artifact generated by the
Rust authority. It carries exact minimal and full-tagged-surface documents, 29 finite-f32 cases
(13 directed and 16 deterministic generated patterns), positive and negative zero, minimum and
maximum subnormals, minimum normal, maximum positive and negative finite values, both exhaustive-
sweep double-rounding exceptions, and ASCII/control/C1/direct-Unicode escaping. The SDK test feeds
that corpus through its actual canonical writer and compares every byte or scalar spelling with
the Rust-produced expectation; it does not implement another parser.

The schema's u64 definition now admits canonical decimal strings only through
`18446744073709551615`. Its Rust artifact test compiles the complete checked Draft 2020-12 schema,
validates the canonical-minimal artifact, then mutates its revision leaf to prove valid
zero/short/19-digit/20-digit/maximum cases plus leading-zero, signed, maximum-plus-one, larger
20-digit and 21-digit refusals. The private shipped implementation class is now
`MisoEngineAudioWorkletProcessor`; the registered V1 processor token is unchanged. Workspace
policy and a red mutation reject a numbered implementation class, and the extracted-package smoke
asserts the unversioned class in the actually staged script.

Architecture inspection confirmed that Web/headless intentionally exposes no session-snapshot
operation. The package requirement above now records Sol's explicit boundary ruling: the package
proves exact canonical author bytes and real engine boot/render, while canonical snapshot
reconstruction remains proved at the existing Rust/C control-protocol boundary. No fake snapshot
or Web ABI expansion was added.

At this pre-browser checkpoint, `cargo test --locked -p session --no-fail-fast` is green (including
the 10-million-pattern f32 test; the separately ignored exhaustive one-shot was not run), the
source-built SDK gate is green at 133/133 tests, and the focused reverse-order/corpus run is green
at 2/2. `scripts/sdk-package.sh check` is green after rebuilding and resealing the six Engine and
four decoder artifacts: all 21 enginectl tests pass and the self-contained 69-file extracted
tarball boots/renders both package workflows and rejects mutations. The Web static/object and 17x
boot-budget gate, hermetic AudioWorklet suite, SDK type/generated checks, session policy, workspace
policy and its mutation suite, formatting, and warning-denied session Clippy are green. The Wasm
binary is source-unchanged and retains SHA-256
`6dcd9ced2daeb886843a764bcc6abc0b4f1b2c7a50af1ed91151a5ab366461e5`.

Per checkpoint discipline, no live Chromium, Firefox or WebKit process was launched here. Root
must first commit this coherent correction tranche; live three-browser qualification will be run
and recorded against that exact candidate commit and artifact hash in a separate evidence
checkpoint. The descriptive benchmark was not run again.
