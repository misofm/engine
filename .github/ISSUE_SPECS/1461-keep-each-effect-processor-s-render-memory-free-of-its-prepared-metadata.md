# Keep each effect processor's render memory free of its prepared metadata

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Filed 2026-10-06 by root order (root ruling on #1377's open item, after #1460). Code anchors
verified on `codex/d15-stream-g` at `38ef4fe7a`. Ordered after #1377.

## Product outcome

No native effect's prepared processor carries its `PreparedEffectMetadata` in the memory that
render owns. Today every native effect's prepared processor keeps a whole copy of that record
(for example `PreparedGate::metadata`, `crates/gate-expander/src/lib.rs:346`, set at `:415-428`)
and returns it from `metadata()` (`:889`) so that preparation can check it. Since #1377 the copy
also carries `tail_every_peak` and `rest`, which render never reads. After this slice each
processor keeps only the values its render path reads, and preparation checks the metadata from the
control side.

## Context

- **Root ruling (2026-10-06).** The decision-15 root coordinator, under the owner's no-shortcuts
  delegation (`no-shortcuts-correctness-first`), ruled that the copy breaks #1329's ruling R5
  ("render-owned memory carries no control-only data"). It predates #1377, so neither #1377 nor
  #1460 is widened; this successor owns it.
- **#1460** moved `PreparedEffectMetadata` out of the graph's render node table into the prepared
  plan's control-side table (`PreparedGraphPlan::effects`, keyed by node). It left the processors
  out of scope.
- **The check.** `crates/effect-compiler/src/prepare.rs:561` calls `processor.metadata()` after
  `factory.prepare(request)` and compares every field with `expected_prepared_metadata`
  (`:562-575`, including `tail`, `tail_every_peak` and `rest` since #1377). The mismatch check
  turns a processor that reports a value its descriptor did not state into a typed diagnostic. A
  second read is at `:1059` (`self.inner.metadata()`).
- **Sites.** `fn metadata(&self) -> PreparedEffectMetadata` in compressor (`src/lib.rs:973`),
  delay (`:1285`), gate-expander (`:889`), multiband-compressor (`:1708`), parametric-eq
  (`:3742`), soft-clip (`:1149`), transient-shaper (`:972`) and true-peak-limiter (`:4422`); bank
  forms return `PreparedBankMetadata` (compressor `:1129`, gate-expander `:980`, multiband
  `:1769`, parametric-eq `:3820`, soft-clip `:1203`, transient-shaper `:1015`, limiter `:4487`).
  Test effects implement the trait in `crates/builtins-compiler/src/lib.rs:6114`,
  `crates/graph/src/lib.rs:3639` and `crates/graph-compiler/src/lib.rs:8918`.
- **Render reads some fields.** For example the gate reads `metadata.sample_rate` (`:489`, `:778`)
  and `metadata.automation_capacity` (`:572`) inside its processor. Those values stay in the
  processor as plain fields; only the record goes.

## Decisions frozen for this slice

- **D1. The prepare result carries the metadata.** The prepare path returns the processor together
  with its `PreparedEffectMetadata` (and the bank form with its `PreparedBankMetadata`), built on
  the control thread; the processor stores none of it. The effect-compiler check compares that
  returned metadata with `expected_prepared_metadata`, field by field, as today. The trait method
  `metadata()` on the processor is removed (or, if a reader needs it at run time, replaced by a
  control-side lookup in the prepared plan's table); no render-path code calls it.
- **D2. Render-read values stay inline.** Each processor keeps exactly the scalars its render path
  reads (sample rate, automation capacity, quantum, bypass and link words already held, and any
  other field a render function reads), as plain fields. No new indirection or lookup on the render
  path.
- **D3. One shape for every effect.** All eight native effects and their bank forms follow D1 and
  D2; the test effects follow the trait change.
- **D4. Class A.** No rendered bit moves.

## Deliverables

1. The trait change in `crates/effect-contract` and its doc.
2. The eight effects and their bank forms changed per D1-D3.
3. The effect-compiler check reading the returned metadata.
4. The test effects updated.
5. The PR evidence: per-effect processor sizes before and after, and the differential (gate 1).

## Amendment 1 (root rulings, 2026-10-06, after #1377's verdict)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), after the #1377 verifier found this spec's scope too small
(`/home/bl/misofm/submix-verdicts/1377-attempt1.md`, Items for ROOT 4):

- **A1. Every reader moves.** Removing the processor trait's `metadata()` also reaches the bank
  `metadata()` readers in `crates/rack/src/lib.rs` (`:781`, `:812`, `:820`, `:1005`, `:1023`,
  `:1059` at `38ef4fe7a`) and `crates/graph/src/runtime.rs:4888`, the test implementations in
  `crates/compressor/tests/native_points.rs`, `crates/effect-compiler/tests/native_session.rs`,
  `crates/rack/tests/live_control_bank.rs` and `tools/console-workload/tests/paired_spans.rs`, and
  about 45 effect-crate test files (`tests/` directories and `padding_tests.rs`) that call
  `.metadata()` on a prepared processor. Each reads the control-side metadata (the prepare result
  or the prepared plan's table). **No non-render accessor is kept for tests.**
- **A2. Authorized paths widen** to those files (re-grep `metadata()` at start and list every file
  in the attempt record), `crates/rack/src/lib.rs` and `crates/graph/src/runtime.rs` (the readers
  only).
- **A3. Split if large.** If the work exceeds half a working day, split before implementation:
  slice 1 the trait, the prepare result and every production reader; slice 2 the test migrations.
  Both keep this issue's gates.
- **A4. Order.** #1461 is a dependency of #1372-#1376 and sits in the `effect-compiler/src/prepare.rs`
  and `effect-contract/src/lib.rs` hot-file rows after #1377 (STREAMS updated).

## Authorized paths (named exceptions)

- `crates/effect-contract/src/lib.rs` (the prepare result and the processor trait)
- `crates/effect-compiler/src/prepare.rs` (the mismatch check and the second read)
- the eight effect crates' `src/lib.rs` (and `src/kernel.rs` only where a render function reads a
  metadata field): `compressor`, `delay`, `gate-expander`, `multiband-compressor`,
  `parametric-eq`, `soft-clip`, `transient-shaper`, `true-peak-limiter`
- the test-effect trait impls in `crates/builtins-compiler/src/lib.rs`, `crates/graph/src/lib.rs`
  and `crates/graph-compiler/src/lib.rs` (trait shape only)
- `crates/conformance` (the harness's read of the prepared metadata, if it calls `metadata()`)
- `docs/EFFECT_CONTRACT_V1.md` (where it describes `metadata()`)
- `docs/handoffs/decision-15-2026-10-05/STREAMS.md` (this slice's row and hot-file note)
- this spec

## Non-goals

- Any change to metadata values, effect behaviour, latency, tail or rest bounds.
- The graph render node (#1460, done).
- Persisted state (R6b).

## Hazards

- The effect crates' `src/lib.rs` are hot files of the #1409 family (#1409, #1411, #1458) and of
  #1372-#1376 (per-effect tail and rest bounds), which edit the same metadata construction. This
  slice lands after #1377 and before #1372-#1376, which rebase onto it; it rebases onto any
  #1409-family slice already landed. See the STREAMS hot-file note.
- `crates/effect-compiler/src/prepare.rs` follows the order in STREAMS (#1377 first by root
  ruling; B #1315/#1345 and G #1339/#1340 rebase).
- A processor that reads a metadata field only in a rare render branch (a reset, a failed block)
  must keep that field: read every render function, not only `process`.

## Objective gates

1. **No rendered bit moves.** A base-versus-head differential through every native effect at
   scalar, `Simd4` and `Simd8`, with automation, resets and restores, at 44.1 and 96 kHz: every
   output and state word identical (PR evidence). `conformance_fixtures --check`, the G5 wasm
   digests, `audit capi`'s `pcm_digest` and the browser expected digests are unchanged.
2. **Each processor shrinks.** `size_of` of each native effect's prepared processor and bank
   processor, before and after, recorded per effect; each falls by at least the removed record
   less any render-read fields kept inline.
3. **The check still bites.** A test in effect-compiler (or the conformance harness) is red when a
   prepare result reports metadata that differs from `expected_prepared_metadata` in any compared
   field (a forged-metadata mutant), and green otherwise.
4. **No allocation or new lookup on render.** `audit capi`: 0 allocations, 0 syscalls; the render
   functions' reads of the kept fields are direct field reads.
5. **Workspace gates.** fmt; workspace clippy with and without `--all-features`;
   `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; the `test-debug-a` and
   `test-debug-b` commands from `.github/workflows/qualification.yml`;
   `bash scripts/check-effect-contract.sh`; `bash scripts/check-workspace-policy.sh`;
   `bash scripts/check-realtime-policy.sh`; `bash scripts/run-wasm-gates.sh`;
   `bash scripts/check-cross-targets.sh`; `bash scripts/check-capi-abi.sh`; the worklet chain.

## Test value

Gate 3's test turns red if the prepare-time check stops comparing a field or compares the
processor's own report with itself, which no other test catches once the processor no longer holds
the record. Gates 1 and 2 are PR evidence, not committed digests.

## Dependencies

#1377 (and #1460 through it).

## Attempt record

None yet.
