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

### Attempt 1 (2026-10-06, implementer; base `7e8379523`)

Not split (A3): one coordinated tranche, the effect crates migrated by five parallel workers on
the frozen contract below, so slices 1 and 2 land together.

**Contract (D1).** `effect-contract` gains `PreparedEffect { processor, metadata }` and
`PreparedEffectBank { processor, metadata }`. `NativeEffectFactory::prepare` returns the first and
`bind_homogeneous_bank` returns `Option` of the second. `fn metadata()` is removed from
`PreparedNativeEffect` and `PreparedNativeEffectBank`; nothing replaces it, so no processor can
report metadata and no non-render accessor exists for tests (A1). `valid_runtime_span` now takes
the `&EffectDescriptor` it alone read, not the metadata. `docs/EFFECT_CONTRACT_V1.md` states the
prepare result.

**The check (deliverable 3).** `effect-compiler`'s mismatch check destructures the prepare result
and compares its `metadata` with `expected_prepared_metadata`, field by field as before. The
second read (`Drifting::metadata`, the test double) is gone with the method.

**Effects (D2, D3).** Each processor keeps, as plain fields, the scalars a processor method reads
(any trait method, rare branches included), and no record:

| Effect | Removed | Kept (reader) |
|---|---|---|
| compressor `Instance<L>` (scalar and banks) | `PreparedEffectMetadata` | `sample_rate` (reset, render, render_mono, restore, automation, point), `bypass` and `link_mode` (render, render_mono), `automation_capacity` (automation), `state_sizes` (snapshot, restore) |
| compressor `PreparedCompressor` / `PreparedCompressorBank<L>` | bank `PreparedBankMetadata` | `sidechain_connected` (process: detector source); bank `width`, `quantum` (process_bank guard) |
| gate-expander `PreparedGate<L, C>` | both records | `sample_rate` (rederive, restore parse), `quantum` (a bank `debug_assert`), `automation_capacity`; bypass and link were already in `coef`, width in `bank_width` |
| delay `PreparedDelay` | `PreparedEffectMetadata` | `bypass`, `sample_rate` (automation, restore), `automation_capacity`, `state_sizes` |
| multiband `PreparedMultibandCompressor` / `...Bank<L, W>` | both records (bank held two) | `automation_capacity`, `state_sizes`; bank `width`; `Instance` already held rate, bypass, link |
| parametric-eq `PreparedParametricEq<L, W>` | both records | `sample_rate` (render, render_mono, restore, response snapshot), `quantum`, `bypass`, `width` (process_bank_inner; the scalar instantiation stores `Four`, unread) |
| soft-clip `SoftClip<L>` / `PreparedSoftClipBank<L>` | both records | `bypass`, `automation_capacity`; bank `width`, `quantum` |
| transient-shaper `Shaper<L, W>` / `PreparedTransientShaperBank<L, W>` | both records | `bypass`, `link_mode`, `automation_capacity`, `state_sizes`; bank `width`, `quantum` |
| true-peak-limiter `LimiterCore<L>` / `PreparedTruePeakLimiterBank<L>` | both records | `sample_rate`, `automation_capacity`, `quantum` (a bank `debug_assert`); bypass reuses `coefficients.bypass`; bank `width` |

Every kept value is a direct field read; no indirection, lookup, `Box` or reference was added on
render.

**Readers (A1, A2).** Re-grep at start: 67 files call `.metadata()` or implement it. Production
readers moved: `rack::EffectBankStage::new` and `LiveControlEffectBankStage::new` take the
`PreparedEffectBank` and read its metadata at bind (`EffectBankStage` keeps one `bypassed: bool`
for the response snapshot); `graph::GraphPreparedEffectBank` gains a control-side `metadata`
field beside its processor (as `GraphPreparedEffect` has), which `graph-compiler::banks` fills and
checks at bind, `estimate.rs` reads, and `runtime::stage_for` hands to the stage; `conformance`'s
harness reads `metadata.exact` from the prepare result. Every other `prepare`/bind caller
(effect-crate tests, `effect-compiler`, `builtins-compiler`, `graph`, `graph-compiler`,
`host-core`, `tools/{audit,bench,console-workload}`) takes `.processor` or `.metadata` from the
result. 101 files changed in all (`git diff --name-only`).

**Tests superseded and deleted (same commit).**
- `rack/tests/live_control_bank.rs` `a_staging_window_larger_than_the_capacity_is_refused_at_bind`
  and its `ShrinkingCapacityBank`: it forged two `metadata()` reads that disagree; with one
  metadata value read once at bind the window is sized from the value it is checked against, so
  the defect is unreachable by type. The `check_automation_window` call stays as the bind-time
  statement of #1012's precondition.
- `conformance`'s `FaultKind::ChangingMetadata`, `FaultKind::ChangingTail` and the
  `metadata.changed` check (and the reset probe's metadata term): a metadata value held by the
  caller cannot change after preparation.

**Gate 1 (no rendered bit moves).** A scratch differential (`/home/bl/misofm/diff1461`, not
committed, deleted after) built twice, against the base worktree and the head, through the
launch registry: every native effect, every launch quality row at 44.1 and 96 kHz, every supported
link mode, sidechain connected and unconnected, prepared bypass on and off; scalar, and every bank
width this x86-64-v3 build binds (Simd8 for all banking effects; Simd4 also binds for the
multiband compressor and the limiter), plus `process_bank_mono` where a bank supports collapse;
24 blocks of 64 frames with silent and loud blocks, Point and Linear automation (74,832 spans, of
which 25,112 are refused by the effects' own rules, so both paths run), resets of both kinds and a
mid-run restore; every output word, every report counter and every lane's state payload after
every block hashed. 208 configurations: base and head identical. `conformance_fixtures --check`
passes; `audit capi`'s `pcm_digest` is `cb10fbface44a3a4`, the value #1460 recorded on main;
`bash scripts/run-wasm-gates.sh` (native + wasm simd128 + V8 EQ loops; the G5 digests) ok; the worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh` with a fresh TMPDIR left empty) passes, the browser expected digests agreeing with the built module.

**Gate 2 (each processor shrinks).** `size_of_val` of each boxed processor, x86-64, base -> head
(`PreparedEffectMetadata` is 144 bytes, `PreparedBankMetadata` 160):

| Effect | scalar | Simd4 bank | Simd8 bank |
|---|---|---|---|
| compressor | 3424 -> 3304 | (not bound here) | 3680 -> 3392 |
| delay | 424 -> 304 | (never banks) | (never banks) |
| gate-expander | 1000 -> 864 | (not bound here) | 2400 -> 2272 |
| multiband-compressor | 856 -> 728 | 3056 -> 2784 | 5792 -> 5504 |
| parametric-eq | 2288 -> 1992 | (not bound here) | 15840 -> 15552 |
| soft-clip | 376 -> 240 | (not bound here) | 768 -> 480 |
| transient-shaper | 336 -> 216 | (not bound here) | 1696 -> 1440 |
| true-peak-limiter | 832 -> 696 | 1584 -> 1312 | 2368 -> 2112 |

Each falls by the removed record(s) less the kept scalars and padding (scalar falls 120-136 of
144; the EQ's one struct held both records and falls 296 of 304).

**Gate 3 (the check bites).** `effect-compiler`'s
`a_prepare_result_whose_metadata_differs_in_any_compared_field_is_refused` replaces #1377's
two-field test (superseded, rewritten in place): a factory wrapping the production EQ edits its
prepare result's metadata, one forgery for each of the 16 compared fields (descriptor id, contract
major and state layout version included), and each must yield `effect.metadata.mismatch` on all
nine entries; the unforged factory prepares. Mutations, each run and reverted: dropping each one of
the 16 comparisons in turn -> red, naming that field; comparing the returned metadata with itself
-> red; revert -> green. Which plausible defect turns it red that no other test catches: a mismatch
check that stops comparing a field, or compares the prepare result with itself. In `conformance`,
`every_faulty_mock_is_detected` now pins `metadata.exact` for `LatencyChangingBypass` and
`BadResources`, whose lies moved into the prepare result: deleting the `metadata.exact` push ->
red (`LatencyChangingBypass escaped detection`); revert -> green.

**Gate 4.** `audit capi`: 0 allocations, 0 deallocations, 0 syscalls. Kept fields are direct reads.

**Re-pin (one row, its own reason).** `capi/tests/resource_lifecycle.rs`
`effect_bank_metadata_bytes`: the bank's `PreparedBankMetadata` moved from the processor box,
which the row never charged, into `GraphPreparedEffectBank`, which it charges per bank. Eight
lanes measured 1,125 (two banks x 160 above the 805 the old 896 ceiling bounded), ceiling 1,280.
The four-lane ceiling (1,600) is derived, not measured: #1304's measured 921 plus three banks x
160 = 1,401, plus 10 %, rounded to 64. CI's `aarch64-debug` prints the measured row; **open item:**
confirm it there.

**Gate 5.** fmt clean; `cargo clippy --locked --workspace --all-targets` with and without
`--all-features`, `-D warnings`: clean; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
--no-deps`: clean; test-debug-a: 1,453 passed, 0 failed, 10 ignored, doctests 22 passed; test-debug-b: 888 passed, 0 failed, 25 ignored, doctests 3
passed; release `audit`/`bench`/`console-workload` tests 113 passed, 0 failed; `check-effect-contract.sh` ok
(8 production factories, 0 failed gates); `check-workspace-policy.sh` ok; `check-realtime-policy.sh`
ok (89 regions); `check-capi-abi.sh` ok; `check-cross-targets.sh` PASS (`ios-asm-memset-pattern16` expected failures #1018 unchanged).

### Batch follow-ups (stream G2 part 1, after the attempt-1 PASS)

From `/home/bl/misofm/submix-verdicts/1461-attempt1.md`.

- **m1, the EQ.** `PreparedParametricEq<L, W>` no longer stores `quantum` or `width`. The bank
  body takes its width from the const `W` (`BankWidth::for_lanes(W)`; the `else` arm, reached only
  by a `W` no bank has, refuses the block with an empty report of the block's own width, as the
  guard refuses a block of another width), and its quantum from the rest planes
  (`PreparedParametricEq::quantum`, `rest[0].len() / W`, the length `prepare_width` gives them).
  The tautological `self.width.lanes() != W` test goes with the field. `prepare_width` loses its
  `width` argument, so its callers (the factory, the bank binder and the unit tests) pass none.
  A const `BankWidth` evaluated at monomorphization was tried first and refused: the unit
  tests' width-generic arms instantiate the bank trait methods for `W = 1` in a branch they never
  take, so a post-monomorphization panic does not compile.
- **m1, the gate-expander.** `PreparedGate::quantum` and the bank `debug_assert!(block.frames <=
  self.quantum)` are removed: the gate has no buffer sized by a quantum, and
  `EffectBankProcessBlock::new` already refuses a block longer than its caller's quantum. (The
  limiter's `quantum` is another slice's crate and stays.)
- **NIT.** `effect-compiler/src/prepare.rs`: the `FORGERIES` doc comment now sits on
  `const FORGERIES`, not on `type Forge`. `conformance/src/effect.rs:155` and
  `docs/EFFECT_CONTRACT_V1.md:314` (the faulty-mock list) are rewrapped to 100 columns.
- **Tests.** No test added or rewritten: the unit tests changed only by dropping the removed
  argument (and building a block's width from `BankWidth::for_lanes(W)` where they read the
  field). Probe, not a gate: replacing `quantum()` with `u32::MAX` leaves every parametric-eq
  test green, as the stored field was before; the guard's quantum term has no test of its own
  (open item below).
- **No rendered bit moves.** All pass on the follow-up tree (x86_64): fmt; workspace clippy `-D
  warnings` with and without `--all-features`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked
  --workspace --no-deps --exclude gate-expander` (gate-expander's own doc failure is part 2's);
  `check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-effect-runtime-policy.sh`;
  test-debug-a 1,462 passed, 0 failed; test-debug-b 890 passed, 0 failed; `conformance_fixtures
  --check`; release `lane`/`math`/`wasm-gates` (G5 `g5_native_digests_match_pins`) 123 passed;
  `run-wasm-gates.sh` (native 144 cases, wasm simd128 144 cases, 0 mismatches; V8 spill ok);
  `check-cross-targets.sh` PASS (known-defect rows unchanged: builtins 5, host-core 4, soft-clip 1);
  the worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`,
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`,
  `test-web-audioworklet.sh` with a private TMPDIR left empty, the V8 spill gate on the named twin):
  all pass. AArch64: CI only.

**Open item.** No test defends the EQ bank guard's `frames <= quantum` term (it was untested
before this follow-up too; a longer block is already refused by `EffectBankProcessBlock::new`
when its caller states the prepared quantum).

### Batch follow-ups, part 2 (stream G2, 2026-10-07; root rulings)

- **The EQ bank guard's `frames <= quantum` term is reachable, so it gets a test** (part 1's open
  item). `EffectBankProcessBlock`'s fields are all public and it is not `#[non_exhaustive]`, so a
  caller can build a block by struct literal without `EffectBankProcessBlock::new`, and `new` itself
  checks only the quantum its caller states, not the bank's prepared one. The guard is therefore
  the bank's only check against the rest planes it sized at preparation, and stays a release check.
  New test `padded_banks::a_bank_block_longer_than_the_prepared_quantum_is_refused`
  (`crates/parametric-eq/src/lib.rs`): at every bank width, a `QUANTUM + 1`-frame block built by
  literal, through `process_bank` and `process_bank_mono`, gives an empty report and leaves both
  planes bit-identical.
  - Test value: dropping the guard's quantum term lets an over-length block render past the
    quantum the rest planes were sized for; no other test builds an over-length bank block.
  - Mutation: the term removed: red ("4 lanes, mono false: a refused block's planes changed"; the
    rest of the parametric-eq lib suite stays green, 50 passed); reverted: green (51 passed,
    `cargo test -p parametric-eq --features test-support --lib`).
- **Root-ratified named exceptions** (verdict m2). Root ratified attempt 1's edits outside the
  authorized paths, each a minimal edit the frozen design (D1, the prepare result carrying the
  metadata) forced:
  - `crates/capi/tests/resource_lifecycle.rs` (stream F's file): the `effect_bank_metadata_bytes`
    ceilings (eight lanes 1,280 from a measured 1,125; four lanes 1,600 derived from #1304's 921
    plus three banks x 160 = 1,401, plus 10 %, rounded to 64). Stream F's later slices that touch
    the file rebase onto this one.
  - `crates/host-core/src/control_preparation.rs` (its test module): the `prepare` caller migration.
  - `tools/audit` (`src/compressor.rs`, `src/delay.rs`, `src/gate_expander.rs`,
    `src/parametric_eq.rs`) and `tools/bench` (`src/console.rs`, `src/effect_contract.rs`): the
    `prepare`/bind caller migrations.
  - The four-lane ceiling 1,600 is still derived, not measured: the pull request's CI
    `aarch64-debug` run prints the measured row, and that run confirms or corrects it (open until
    then; AArch64 runs only in CI).
