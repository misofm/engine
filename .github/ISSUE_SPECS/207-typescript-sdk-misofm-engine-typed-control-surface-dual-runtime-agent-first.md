# 207 TypeScript SDK: @misofm/engine — typed control surface, dual runtime, agent-first

**Authority: GitHub issue #207.** Its design body, adversarial-verification ledger comment
`5434208066`, and implementation-starting-point comment `5434209861` are the stateless brief. This
file is the local Sol decision record and evidence log; it never replaces those three authorities.

## Engagement boundary

This branch owns only the first three separately mergeable slices:

1. Phase 0: prerequisites P1 (ABI-layout emission), corrected P2 (observation transaction-kind
   metadata), and P4 (automatic artifact provenance).
2. Phase 1: deterministic SDK codegen plus the zero-runtime-dependency core package under `sdk/`.
3. Phase 2: the headless Bun/Node engine over the shipped zero-import Wasm module.

P3, P5, and Phases 3–6 remain follow-ups. Owner questions Q1–Q4 do not block these slices. No
frozen file under `hosts/miso-engine-host-web/web/` may change. No render-path ABI or DSP behavior
may change.

## Sol-approved decisions and corrections

- P2 uses a separate observation-transaction-kind table for wire kinds 7/8. It does not append
  them to `miso.command.v1`'s applied kinds 1–6 and therefore preserves issue #140's invariant that
  every declared command kind is applied.
- The generated ABI document describes the actual `repr(C)` prepare, status, resource, command
  report, and meter-header layouts; the fixed 48-byte little-endian command record; and frozen
  result, state, backend, and buffer constants. Rust tests compare the emitter's table to
  `core::mem::offset_of!`/`size_of!` on the real public structs.
- Provenance is generated only from a clean Git tree and pins the source revision plus SHA-256 and
  byte length for every shipped web/metadata asset. It is never hand-refreshed.
- Phase 1 synthesizes one `main` output and one unity `post_matrix` route per otherwise unrouted
  track, making the flagship one-track builder valid and audible. Explicit outputs/routes suppress
  this convenience.
- The issue-sketch `{ left, right }` track-source convenience is constrained by the actual Session
  V1 model: it has one `source_id`. The SDK therefore accepts the ergonomic form only when both
  lanes name the same declared source and both channel indexes are within that source's declared
  channel count; otherwise it raises a typed local error before TOML emission.
- Tuple effect indices use `Exclude<Partial<T>["length"], T["length"]>`; an out-of-range literal is
  a compile-time error.
- The builder only exposes Session V1 rack-effect automation. Live commands cover the
  `liveUpdatable` half of the builtin strip (fader, mute, matrix/pan), not polarity/trim/filters.
- Implementation finding: the issue sketch's `linkMode: "linked"` is not a Session V1 token and
  cannot round-trip through the real validator. The smallest truthful surface uses the engine's
  closed `dual_mono`, `maximum`, and `average` tokens. Launch native effects remain `quality:
  "normal"`; the wider schema quality tokens are not promised by the generated native catalog.
- `validateSession()` creates a fresh Wasm instance for every call, reads diagnostics as a
  NUL-terminated prefix of the capacity-only buffer, and disposes the instance.
- `render(frames)` accepts any non-negative integer frame count, renders whole engine quanta, and
  returns only the requested prefix while retaining the unused tail for the next call. Source
  submission and command application remain quantum-paced.
- Source/session sample-rate mismatch is rejected before preparation with a typed error. A sticky
  compile/render failure reads diagnostics and requires dispose-and-recreate; the SDK never
  pretends recovery in place.
- `wav16` output is deferred with browser/SAB/package work; Phase 2 writes `f32le-planar` and
  `wav32f` only, avoiding an unspecified quantization/dither policy.

## Phase gates

### Phase 0

- E0a: the ABI-layout schema gate passes, its `--self-test` proves every rule red, and a copied
  document with one flipped offset is rejected.
- E0b: the emitting tool's Rust golden tests pass against the real struct offsets/sizes; a
  deliberate field-offset mutation fails.
- Corrected-P2 and provenance schema gates pass their self-tests and named red mutations.
- `scripts/sweep.sh`, `cargo fmt --check`, and workspace clippy with `-D warnings` are green before
  Phase 1 begins.

### Phase 1

- E1: generated catalog parity with the engine JSON, including the shipped metadata copy; a value
  mutation is rejected.
- E2: the command-reason vocabulary gate has seven synchronized spellings and its generated-table
  mutation is rejected.
- E3/E4: at least 40 builder documents cover every effect and parameter boundary/choice, routing,
  sends, all automation shapes, and 1/2/64 tracks; every validator stage passes and canonical
  stdout is byte-identical.
- E5: every parameter rejects values immediately outside its domain locally. E5a drives
  schema/model/builtin values through the real four-stage validator CLI; E5b drives effect values
  through the full fresh-instance Wasm compile pipeline. Both require typed leaf authority, and
  the pinned ratio witness proves the approved CLI-pass/Wasm-fail asymmetry. Type tests reject an
  out-of-range tuple effect index.
- `scripts/sweep.sh`, `cargo fmt --check`, and workspace clippy with `-D warnings` are green before
  Phase 2 begins.

### Phase 2

- E6: two in-process and one fresh-process render have identical planar-stream SHA-256 digests
  under Node and Bun.
- E7: SDK and native C-ABI/WAV runner outputs are byte-identical at 48 and 96 kHz. This crosses the
  capi+real-WAV and web-host front ends; `direct-oracle.mjs` already supplies lower-level prior art.
- E8: the first affected fader sample, `ack.appliedAtSample`, and `k * quantum` are identical.
- E9: backpressure and unknown track/parameter/tap/observation-unbound refusals are exact typed
  resolved acknowledgements; no ack precedes a later drop and a valid command still succeeds.
- E10a: a one-byte Wasm mutation fails typed hash attestation before instantiation and leaves no
  partial engine.
- The eval runner executes under both Node >=20 and Bun >=1.1. Every eval has a recorded deliberate
  red mutation. The final `scripts/sweep.sh`, `cargo fmt --check`, and workspace clippy with
  `-D warnings` are green.

## Delivery and evidence rules

Each phase receives its own coherent local checkpoint commit and adversarial Sol verdict before the
next phase starts. All heavy Cargo/Wasm work runs only through the engagement's shared CPU
serializer. Shell edits receive immediate `bash -n`. The branch is neither pushed nor merged; the
final handoff names exact commits, gate counts/digests, unresolved findings, and verifier probes.

## Evidence log

### Phase 0: local PASS via bounded successor #209

- `6c8a967` emits and independently validates `miso.web.abi-layout.v1`; the metadata tool has two
  real-structure layout tests, including a scoped duplicate-field red mutation.
- `9888bfe` preserves applied `miso.command.v1` kinds 1–6 and separately emits
  `miso.observe.v1` transaction kinds 7/8 with wrong/truncated/relabelled/protocol/applied red
  mutations.
- `7bd2930`, `56ea90f`, and `387b789` add clean-tree provenance generation, independent sibling
  byte/hash verification, a real wrapper smoke, and complete scoped E0b coverage.
- The third #207 Phase 0 attempt stopped candidly at `91/92` sweep rows: the correct ABI-layout
  artifact was placed out of lexical order in the checker's expected list. No other row failed.
  Stateless successor #209 owns that sole gate repair under commits `0fedc9e` and `c77a0d0`.
- Independent Sol review: initial HOLDs were resolved; #209 terminal verdict PASS. The regression
  probe compares the canonical list with an independent `LC_ALL=C sort` oracle and rejects the
  exact former ordering.
- Fresh serialized SDK build: seven exact files; Wasm `2,494,615` bytes,
  SHA-256 `99c08301577dc27799bee3c13fe74dfee87db36b0b54864d97c92935666368d6`;
  provenance `1,264` bytes,
  SHA-256 `37a01e6a54e9d0bd806682e80bc9c4fccacd1b99c717a4ce2a99e5d9009d63ef`;
  independent provenance recomputation PASS.
- Fresh full gates: `scripts/sweep.sh` `92/92` PASS in `121s`; `cargo fmt --all -- --check`
  PASS; `cargo clippy --locked --workspace --all-targets -- -D warnings` PASS. Every heavy command
  ran through the engagement CPU serializer.
- Delivery state: local Phase 0 is separately mergeable and green. No commit was pushed by owner
  instruction, so GitHub #209 remains open and is not claimed remotely synchronized or complete.

### Phase 1 tranche A: generated data scaffold (`fe22270`)

- `sdk/` now has the zero-runtime-dependency `@misofm/engine` package scaffold, strict NodeNext
  TypeScript configuration, and the seven exact Phase 0 artifacts copied from the accepted sealed
  build. No session builder, command encoder, or runtime host is included in this tranche.
- `sdk/codegen/generate.mjs` deterministically transcribes parameter metadata, ABI layout, and
  provenance into immutable `as const` generated modules. `--check` refuses any manual drift.
- E1 compares the generated catalog literal with the shipped metadata using deep equality; its
  self-test changes one generated reason value and proves both parity and codegen checks refuse it.
- TypeScript `6.0.3` is a pinned development-only dependency. The gate runs the real compiler with
  `--noEmit`; its self-test proves a wrong-type assignment red. The compiler caught and drove a
  generator correction that keeps `as const` on the JSON literal's closing line.
- `check-command-reason-vocabulary.py` now reads the generated catalog as the seventh spelling and
  its self-test rejects a catalog-only reason rename. Phase 0's separate observation-transaction
  kinds remain separate from applied `miso.command.v1` kinds.
- Focused green evidence: `bash -n scripts/check-sdk-generated.sh`; real `tsc --noEmit` and its red
  self-test; generated codegen check; E1 self-test; metadata schema check against the SDK asset;
  and command-reason vocabulary self-test (19 red mutations). Full sweep, Cargo, and Wasm gates
  were intentionally not run in this tranche.

### Phase 1 tranche B: core catalog and command primitives (`a6db2dd`)

- `describe()` returns one frozen `miso.sdk.describe.v1` document with the generated catalog,
  revision, ABI, asset hashes, Wasm byte count, launch rates, and render quantum.
- The command encoder takes all offsets, record size, maximum count, and wire kinds from generated
  ABI data. Reserved bytes stay zero; invalid integer fields, NaN/infinity, and values that overflow
  finite `f32` are rejected locally with a typed path.
- Focused green evidence: generated-data gate self-test; real TypeScript compile; core runtime
  self-test under Node. Red command mutations for NaN and finite-f64-to-infinite-f32 both fail.
- The builder and its validator corpus remain outside this checkpoint by design; this tranche is a
  recoverable core primitive boundary, not a Phase 1 PASS claim.

### Phase 1 tranche C: typed Session V1 builder (`c314175`)

- The persistent builder emits the exact 14-key Session V1 shape, resolves native effect metadata
  into typed parameter rows, creates stable rack-local slot IDs, supports explicit graph and
  rack-effect automation declarations, and supplies the corrected default `main` output plus one
  unity `post_matrix` route per track when the graph is otherwise undeclared.
- Canonical values use engine channel order and Rust-compatible finite-`f32` spellings, including
  the two documented double-rounding patterns. JSON-safe automation sample strings serialize back
  to bare TOML integers and are bounded to TOML `i64`.
- The exact tuple-index constraint is compiled both green with `@ts-expect-error` and red as an
  unsuppressed out-of-range call. Builder self-tests also reject metadata-domain overflow,
  finite-f64-to-infinite-f32 values, invalid dual-source mappings, out-of-bounds channels,
  overlapping automation, and sample times above TOML `i64`.
- Independent Sol probe: the real serialized `miso-engine-session-validator` accepted the flagship
  1,993-byte builder document at all four stages and its `--canonical` stdout was byte-identical.
  A separate 127-frame-quantum mutation also passed all four stages, causing removal of an invented
  power-of-two SDK restriction. The >=40-document E3/E4 corpus remains the next tranche.

### Phase 1 E5 oracle amendment (coordinator-approved)

- The verified E5 text requires each forced outside-effect-domain TOML value to fail the real
  session validator at stage 2 or 3 with the corresponding effect leaf. The current validator does
  not perform that operation: stages 2/3 own schema/unit-local validation and resource/canonical
  compilation, while stage 4 prepares builtins only. Native descriptor validation lives separately
  in `prepare_native_session_effects` and is not called by the validator.
- Concrete serialized probe: changing compressor `ratio` from `2.0` to the next `f32` above its
  metadata maximum (`20.000002`, maximum `20.0`) was accepted with PASS at all four validator
  stages. This agrees with `docs/SESSION_SCHEMA_V1.md`, which assigns native descriptor/effect
  validity to issue 011 rather than the base session compiler.
- Coordinator comment `5435519665` approves a domain-owner split. E5a keeps the four-stage CLI as
  oracle for schema/model/builtins domains. E5b uses a fresh zero-import Wasm instance through the
  full prepare/compile/diagnostic-buffer path for effect-parameter domains. The primary local gate
  still covers every catalog domain in both groups.
- The ratio probe is now a required asymmetric red witness: `20.000002` must PASS the four-stage CLI
  and fail the Wasm compile oracle with `effect.parameter.domain` at the effect leaf. The SDK does
  not extend the CLI; remote successor #211 owns its future fifth effect-preparation stage.

### Phase 1 tranche D: E3-E5 corpus (pending checkpoint)

- The E3/E4 green corpus contains 247 canonical Session V1 documents: all boundary/default/choice
  rows for all 66 native effect parameters and all ten builtin parameters, all seven send taps,
  all three automation shapes, and 1/2/64-track sessions. The real validator CLI accepts every
  document at all four stages and returns byte-identical canonical stdout.
- E5 locally rejects 20 builtin and 132 effect just-outside-domain cases with the complete generated
  descriptor attached to `MisoSessionError`. The engine oracles reject every forced TOML mutation
  at an exact typed leaf: builtin cases through the CLI, and effect cases through a new prepared
  Wasm instance per document. Of the 132 effect cases, 127 reach `effect.parameter.domain`; five
  meet the stricter Session V1 numeric envelope first and return its exact numeric leaf.
- The compressor `ratio = 20.000002` witness passes the four-stage CLI canonically and fails the
  full Wasm compile with
  `effect.parameter.domain\t$.tracks[id=track].effects[id=effect]`. E3 schema, E4 canonical-byte,
  and E5 local-domain deliberate mutations are each proven red under `--self-test`.
- Focused serialized gate result: `documents=247`, `effect-parameters=66`,
  `builtin-negative-cases=20`, `effect-negative-cases=132`, `effect-prepare-cases=127`,
  `schema-envelope-cases=5`, `ratio-asymmetry=PASS`. Phase-wide sweep/fmt/clippy remain pending.

### Phase 1 integration sweep repair (pending checkpoint)

- The first serialized 94-row phase sweep reported `90/94` in `114s`. All four failures had two
  integration causes: the corpus gate's private validator-path environment variable violated the
  repository-wide `MISO_ENGINE_` vocabulary, and the Issue-081 source-artifact policy treated the
  deliberately packaged SDK Wasm as an accidental build output. The failures are recorded as red;
  none was suppressed.
- The validator binary is now passed as an ordinary positional argument, avoiding a new process
  environment contract. The artifact policy exempts only
  `sdk/assets/miso-engine-v2-audio-worklet.simd128.wasm`; a deliberate sibling Wasm mutation proves
  the policy remains red for generated artifacts elsewhere under `sdk/`.
- Focused green evidence: shell syntax for every edited script; qualification policy plus its red
  mutation suite; environment vocabulary plus its mutation suite; generated SDK self-test; and a
  fresh serialized E3-E5 corpus with the same `247/20/132/127/5` counts. A fresh full sweep remains
  required after this checkpoint.

### Phase 1 adversarial attempt 1: HOLD and bounded revision (pending checkpoint)

- Independent Sol review of `98e3c8e` returned HOLD. It reproduced six gaps: signed negative
  exponent expansion moved the decimal point; synthesized output `main` could collide with a track;
  source length/content, pan, and sidechain references could escape local validation; command
  value arrays could be short or overwrite reserved bytes when long; `fromJson` lost negative zero;
  and sweep did not invoke focused core/builder tests or verify the real packaged sibling bytes.
  The later `589fc44` integration repair was correctly excluded from that verdict.
- Red-before-fix probes reproduced the command defect (one value was accepted) and canonical-float
  defect (`-1e-7` emitted as `-0.000001`). The committed eval expansion also covers five-value
  reserved-byte overwrite, signed-zero rebuild, default-ID collision, zero-length and empty-content
  sources, pan outside `[-1,1]`, and a sidechain referencing a missing graph entity.
- The bounded correction counts exponent positions on the unsigned coefficient, clones normalized
  JSON structurally without decimal text conversion, requires exactly four command values, applies
  the relevant Session V1 source/pan/u32 constraints, checks the synthesized ID, and validates
  routed sidechain graph references. The corpus's independent float-token helper now asserts every
  forced neighbor decimal reconstructs the intended exact `f32` bits.
- `check-sdk-generated.sh --self-test` now runs core and builder runtime/self-test gates plus the
  actual `sdk/assets` provenance sibling-byte check and its changed-artifact mutation. Focused gate
  PASS; serialized E3-E5 PASS with `documents=248`, `effect-parameters=66`,
  `builtin-negative-cases=20`, `effect-negative-cases=132`, `effect-prepare-cases=127`,
  `schema-envelope-cases=5`, and `ratio-asymmetry=PASS`. Full gates and independent re-review remain
  pending; Phase 2 has not started.

### Phase 1 adversarial attempt 2: HOLD and terminal bounded revision (pending checkpoint)

- Independent Sol re-review of `a2dbcde` confirmed the attempt-1 findings closed, but returned a
  second HOLD on six newly isolated contract gaps. The builder erased its generic shape instead of
  returning `SessionPlan<S>` with typed tracks and prepare limits; JSON text round-trips still lost
  signed zero; `quantum_frames` lacked its `u32` upper bound; canonical strings did not match Rust
  control-character escaping or reject unpaired UTF-16 surrogates; generated effect input types
  incorrectly allowed per-lane objects for shared parameters; and the README described the future
  Phase 2 validator API in the present tense.
- Red-before-fix evidence reproduced the type erasure and shared-parameter defect with real
  `tsc --noEmit` failures (including unused expected-error directives), and the runtime builder
  self-test failed because the returned plan had no typed track summary. This is the third and
  terminal Phase 1 implementation attempt under the three-attempt rule.
- The builder now returns `SessionPlan<Tracks>`, preserving exact track keys and effect tuples,
  exposes a frozen typed track summary, and derives prepare-limit override fields from the generated
  ABI layout. Effect parameter inputs derive scalar-versus-per-lane shape from each generated
  parameter's `channelPolicyName`, so delay `cross feedback` rejects a lane object while per-lane
  compressor parameters retain lane typing.
- Session JSON uses a narrowly tagged representation for negative zero during `JSON.stringify`;
  `SessionPlan.fromJson` decodes it and preserves the value. Both construction and `fromJson`
  enforce the full positive `u32` quantum range. Canonical quoting escapes all Rust control
  characters through U+009F and rejects unpaired surrogates while retaining valid pairs.
- The README now distinguishes the current direct-Wasm E5b corpus from the Phase 2 public
  `validateSession()` API. Focused green evidence: real TypeScript compilation, typecheck red
  mutation, core and builder self-tests, JavaScript syntax, and diff hygiene. The serialized corpus
  passes with `documents=249`, `effect-parameters=66`, `builtin-negative-cases=20`,
  `effect-negative-cases=132`, `effect-prepare-cases=127`, `schema-envelope-cases=5`, and
  `ratio-asymmetry=PASS`. Final full gates and the terminal independent verdict remain pending;
  Phase 2 has not started.

### Phase 1 terminal verdict: PASS (`38cdd37`)

- Fresh owner gates after the terminal revision: `scripts/sweep.sh` passed `94/94` rows in `132s`;
  `cargo fmt --all -- --check` passed; and
  `cargo clippy --locked --workspace --all-targets -- -D warnings` passed. Every command ran through
  the engagement CPU serializer.
- Independent Sol terminal review returned PASS. It verified exact builder generic preservation and
  effect tuples, all 25 generated-ABI prepare-limit fields and their `u32`/`u64` TypeScript shapes,
  real stringify/parse negative-zero bit preservation, both quantum entry points at the `u32`
  boundary, every U+007F through U+009F escape, paired and unpaired surrogate behavior, and
  channel-policy-dependent parameter typing.
- The review also reproduced the attempt-1 correction probes, confirmed E1/E2 parity, inspected the
  249-document E3/E4 corpus and E5's `20/132/127/5` oracle split, and verified the exact compressor
  ratio asymmetry required by coordinator comment `5435519665`. The copied host artifacts are
  byte-identical, packaged siblings pass provenance, frozen web sources are unchanged, and no
  Phase 2 files exist at this checkpoint.
- Independent lightweight gates included the generated SDK self-test, environment-vocabulary red
  mutations, effect-interchange policy red mutations, and public API verifier probes. Phase 1 is
  separately mergeable; Phase 2 may begin only after this evidence checkpoint.

### Phase 2 tranche A: headless runtime foundation (pending checkpoint)

- Added the generated-ABI-driven zero-import Wasm boundary, fresh-instance NUL-terminated session
  diagnostics, exact pre-compilation SHA-256 attestation, RIFF/RF64 PCM and floating-point decoding,
  native block-planar and WAV32f output encoders, source-ID byte-order pacing, partial-quantum output
  retention, status/resources, console command reports, observation commands, and meter polling.
- Unanticipated ABI fact: a raw `{ toml }` carries no out-of-band rate/quantum and the compiled host
  exposes no source-declaration query. The smallest-footprint host reads only the two required root
  integer scalars before giving the complete document to the real compiler; this is not a TOML
  schema parser. Raw-TOML source inputs currently denote a complete region starting at zero because
  nonzero region discovery is impossible through the frozen ABI; typed `SessionPlan` inputs retain
  exact declared regions. This limitation is flagged rather than hidden or addressed by duplicating
  TOML parsing.
- The first live probe caught a JavaScript `WebAssembly.instantiate(bytes)` overload-shape mistake
  before any eval claim. After correction, the real 48 kHz native-runner fixture rendered `1,024`
  frames / `8,192` bytes with SHA-256
  `cef2b4282bb8478687b4dec5f764a9f04bc64fc7a35d3a8edd5b398a80494771`, exactly matching the
  independently pinned native manifest. Real `tsc --noEmit` and diff hygiene pass. Phase 2 eval
  gates, deliberate red mutations, Bun coverage, 96 kHz cross-oracle, and full gates remain pending.

### Phase 2 tranche B: E6-E10a Node/Bun gate (pending checkpoint)

- `check-sdk-headless.sh` builds the real native C-ABI/WAV runner once, emits the TypeScript SDK,
  and runs the same eval file under Node 22.23.2 and Bun 1.4.0. The new sweep row invokes its
  `--self-test`; shell syntax was checked immediately after each edit.
- Red-before-green harness evidence was preserved: the first run looked for an underscored instead
  of Cargo's documented hyphenated runner binary; the next used an empty graph as a Wasm-valid
  validation fixture; E8 initially measured a deliberately cancelling pan fixture; and E6's first
  1,024-frame fixture ended before the compressor/EQ/limiter chain's fixed latency. Each failure was
  corrected at the fixture or harness boundary without weakening the runtime contract.
- E6 renders a non-silent 4,096-frame session containing compressor, parametric EQ, true-peak
  limiter, two generated stereo WAV sources, and Session V1 automation twice in-process and once in
  a fresh process. Node and Bun agree on SHA-256
  `67490eb8c623c7e6797ee88f46b777d4fe6c7da2b2873ab553871fd5019d0f43`.
- E7 compares complete bytes from SDK `renderToFile(..., {format:"f32le-planar"})` with the real
  native runner: 48 kHz is
  `cef2b4282bb8478687b4dec5f764a9f04bc64fc7a35d3a8edd5b398a80494771`; 96 kHz is
  `dcb0de625cb09c064ea424dff6b1eca01896ba1e7ee602c72dc7454ad9b74f16`. Both also equal the
  independent repository manifest pins under both JavaScript runtimes.
- E8 proves first changed sample = `ack.appliedAtSample` = `2 * 128 = 256`. E9 returns, in order,
  `backpressure`, `unknownTrack`, `unknownParameter`, `unknownTap`, `observationUnbound`, and a final
  successful `none`, with the overfilled transaction admitting zero records. E10a flips one Wasm
  byte and proves the typed error names asset/expected/actual while an instrumented Wasm compile
  counter remains zero.
- Self-test mutates one E6 digest, one E7 output byte, the E8 acknowledgement, one E9 reason, and the
  E10a ordering count; all five mutations turn their eval predicate red under both runtimes. Extra
  focused probes cover fresh-instance diagnostics, PCM16/24 plus 32f/RF64 WAV decode, exact
  partial-quantum retention, early rate mismatch, and compile-time exact headless track/effect/
  parameter access. Generated/core/builder self-tests remain green. Full sweep/fmt/clippy and
  independent Phase 2 review remain pending.

### Phase 2 tranche C: public-boundary hardening (pending checkpoint)

- High-level fader, mute, pan, and bypass calls now reject malformed/out-of-domain values locally;
  typed acknowledgements retain the exact raw numeric report; status, resource, command, and meter
  structures verify every generated reserved word. `validateSession()` sizes TOML staging to the
  supplied document rather than imposing an accidental 1 MiB ceiling.
- WAV support is narrowed to the specified PCM16, PCM24, and IEEE-f32 formats and now checks RIFF/
  RF64 declared sizes, RF64 `ds64` totals, duplicate format/data chunks, extensible fields, and pad
  bytes. The stricter gate correctly turned red when its synthetic odd-byte PCM24 fixture lacked
  the mandatory pad; fixing the fixture left the production parser strict and both native E7
  digests unchanged.
- The package declares its core and `./headless` subpaths with `sideEffects:false`. The README now
  states the non-multiple render policy, early rate refusal, sticky failure recovery, fresh-instance
  validation, supported output formats, and raw-TOML region limitation in present tense.
- Real TypeScript/public-type checks and the complete Node/Bun E6-E10a gate remain green with the
  same digests and five red mutations. Duration-bounded path-backed WAV input and file output are
  the next tranche; full gates and independent review remain pending.
