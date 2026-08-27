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
- E5: every parameter rejects values immediately outside its domain locally and through the real
  validator with matching leaf authority. Type tests reject an out-of-range tuple effect index.
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

### Phase 1 tranche A: generated data scaffold (pending checkpoint)

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

### Phase 1 tranche B: core catalog and command primitives (pending checkpoint)

- `describe()` returns one frozen `miso.sdk.describe.v1` document with the generated catalog,
  revision, ABI, asset hashes, Wasm byte count, launch rates, and render quantum.
- The command encoder takes all offsets, record size, maximum count, and wire kinds from generated
  ABI data. Reserved bytes stay zero; invalid integer fields, NaN/infinity, and values that overflow
  finite `f32` are rejected locally with a typed path.
- Focused green evidence: generated-data gate self-test; real TypeScript compile; core runtime
  self-test under Node. Red command mutations for NaN and finite-f64-to-infinite-f32 both fail.
- The builder and its validator corpus remain outside this checkpoint by design; this tranche is a
  recoverable core primitive boundary, not a Phase 1 PASS claim.
