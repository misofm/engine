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

Pending implementation.
