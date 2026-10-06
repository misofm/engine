PASS

# #1377 attempt 1 -- adversarial verdict (Sol verifier, 2026-10-06)

Reviewed: the net #1377 change `git diff 320baffce 38ef4fe7a` on `codex/d15-stream-g`
(`a5b4d0727` code, `38ef4fe7a` record), plus the #1377 parts of `7b02c3959` (Amendment 1, the
`prepare.rs` hot-file row) and `fcb7151b7` (Amendment 2). I also read `03db4574f` (#1461 spec) for
consistency. I checked the change against D1-D5, Amendments 1 and 2, gates 1-3, AGENTS.md,
decision 15 D15-4(b), #1329's spec (R5, G1, the `tail_every_peak` name) and the owner's
no-shortcuts rule.

All work was done on exports: `tree` and `tree2` = `38ef4fe7a`, `base` = `320baffce`. I did not
edit, build, check out or commit in the worktree. (Its uncommitted `builtins/src/tail.rs`,
`math/src/tail.rs` and `tests/zz_probe.rs` belong to the parallel #1433 implementer.) Every
mutation was applied to the `tree` export and then restored; `git status` of that export is
clean. Small evidence is in `/tmp/claude-1002/v1377/evidence/`.

**Verdict.** The implementation does what D1-D5 and both amendments say:

- One function per effect, `tail_and_rest(rate, quality) -> EffectTailBound`.
- `QualityDescriptor::tail` is deleted.
- `expected_prepared_metadata` is the only place that copies the three values.
- The program key and the `effect-compiler` mismatch check carry and compare `tail_every_peak` and
  `rest`.
- The values are unchanged.
- The render node does not move.
- No audio bit moves.

Every gate I ran again is green.

There is no BLOCKER and no MAJOR finding. There is one MINOR finding: the gate-1 conformance check
has no unique catch, and the record claims one for it. The spec requires the check, so this is
not an attempt defect. Root must decide on gate 1 (Items for ROOT, item 1). There are three NITs,
all about documentation and evidence.

## BLOCKER

None.

## MAJOR

None.

## MINOR

**m1. The gate-1 conformance check (`metadata.tail_bound`) and its fault row catch nothing that
another test does not already catch. The record says they do.**

- *Where.* The check is at `crates/conformance/src/effect.rs:1115-1125`. The mock fault
  `UndeclaredRestBound` is at `:159-161` and `:411-413`. The row is at
  `crates/conformance/tests/effect_contract.rs:215`. The claims are in the spec: the gate-1 test
  value at `:130-131` and the record at `:252-258` ("the new check's own catch is a wrong
  `expected_prepared_metadata`, seen on real effects").
- *Why the spec's gate-1 claim is false.* The spec says: "an effect whose prepare writes a tail or
  rest of its own ... is red; nothing compares metadata to a computed bound today". D3 put both
  new fields into `EffectProgramKey`, so the existing `metadata.exact` (`effect.rs:1112`) now
  catches such an effect. M6 shows this. With the check disabled, the `UndeclaredRestBound` mock is
  still refused, as `["metadata.changed", "metadata.exact", "reset.semantics"]`. Only the label
  `metadata.tail_bound` is lost. In production, the `effect-compiler` mismatch check also refuses
  such an effect (gate 2).
- *Why the record's replacement claim is not a unique catch.* The record names "a wrong
  `expected_prepared_metadata`". I applied the two forms the check can see:
  - **M7** (`tail: TailSamples::Infinite`). Red in `tail_bound_tests`, and in three
    `graph-compiler` fixture tests: `launch_gate_expander_fixture_…`, `launch_soft_clip_fixture_…`
    and `launch_transient_shaper_fixture_…`. It is also red in the new check (the mock and the gate,
    soft clip and transient shaper conformance runs).
  - **M3** (`tail_every_peak: bound.tail`). Red in `tail_bound_tests` and in the new check.
  - **M1, M2, M4 and M5.** Real effects state constant `Infinite`/`Unstated`, so the check cannot
    see these four mutations at all.

  So the check catches nothing that `tail_bound_tests` or `metadata.exact` does not already catch.
  I confirmed the narrow claim that M7 alone with the check removed leaves
  `gate-expander --test conformance` green. That claim is true, but the defect is caught
  elsewhere.
- *Scope.* The row is the file's only change. It is outside that file's field-only authorization,
  and the record says so (`:220-224`).
- *Why MINOR and not MAJOR.* Gate 1 is a binding spec gate. The implementer built it as written
  and disclosed the overlap. The precedent is #1234's NIT 2: a spec-required row with no unique
  catch is not an attempt defect. Only the record's sentence claims more than the evidence shows.
- *Fix.* Root decides on gate 1 (Items for ROOT, item 1). The record's sentence becomes: "no unique
  catch: `metadata.exact` catches an effect that writes its own value, and `tail_bound_tests` (with
  three `graph-compiler` fixture tests for M7) catches a wrong `expected_prepared_metadata`".

## NIT

- **n1. The contract doc's opening field list is stale.** `docs/EFFECT_CONTRACT_V1.md:16-19` says
  that prepared metadata fixes "latency, tail, state-section sizes, …" and that "The semantic
  `EffectProgramKey` contains these fields directly". It does not name `tail_every_peak` or `rest`.
  The new paragraph at `:95-107` is correct; only this earlier list was not updated.
- **n2. The `tail_every_peak` bullet names only builtin homes.** `docs/EFFECT_CONTRACT_V1.md:77-80`
  says "beside `tail` wherever `tail` is stated", then names only `builtins::InputSectionBound` and
  `PreparedBuiltinsSession::input_bounds`. Since #1377 it is also in `EffectTailBound`,
  `PreparedEffectMetadata` and `EffectProgramKey`.
- **n3. The record's size table leaves out two types that grew.** I measured on x86-64, base
  against head:
  - `EffectProgramKey`: 112 -> 152.
  - `PreparedBankMetadata`: 120 -> 160. Every bank processor's `metadata()` returns this type by
    value, and the rack's response-snapshot path reads it (`crates/rack/src/lib.rs:812`, `:820`).
  - `EffectDescriptor`: 112 -> 120.
  - `QualityDescriptor`: 64 -> 48.

  None of these is render-owned storage. The record's own rows are exact (see "Sizes" below).

## Review answers (the caller's questions)

- **Is every effect's tail unchanged?** Yes. I compared base and head independently with a probe
  example, `effect-compiler/examples/v1377_tail_probe.rs`, built in each export and then removed:
  - At base, the probe printed `QualityDescriptor::tail` for every quality row of the eight
    production descriptors.
  - At head, it printed `(descriptor.tail_and_rest)(row.sample_rate, row.quality).tail`.
  - All 32 rows (8 effects x 44.1/48/88.2/96 kHz x `Normal`) are byte-identical: EQ, compressor,
    limiter, multiband and delay are `Infinite`; gate and transient shaper are `Finite(0)`; soft
    clip is `Finite(29)`.
  - At head, all 32 rows state `tail_every_peak: Infinite` and `rest: Unstated` (D4).
  - The test doubles keep their old tails, by diff: the mock `Finite(3)`, `native_session`
    `Finite(7)`, `registry` and `response_analysis` `Finite(0)`.
  - No quality function varied its tail by rate, so the rate-independent bodies are exact.
- **Is anything lost with `QualityDescriptor::tail`?** No. At base, the only readers were
  `expected_prepared_metadata` and the four frozen-descriptor tests the spec names. No validator,
  wire, tool or SDK read it (I searched `crates`, `hosts`, `tools` and `sdk`).
- **The conformance harness addition.** See m1. The extra `effect.metadata()` call does not change
  the mock's other faults: `metadata_calls` counts `process` calls (`effect.rs:434`), not
  `metadata` calls.
- **Mutations.** Each was re-applied by me, red as named, and green on restore:

  | Mutation | Red | Unique? |
  |---|---|---|
  | M1 expected `rest: Unstated` | `tail_bound_tests` (Unstated vs Bounded) | yes |
  | M2 function at fixed 48 kHz | `tail_bound_tests` (Finite(48) vs Finite(44)) | yes |
  | M3 `tail_every_peak: bound.tail` | `tail_bound_tests`; mock, gate, soft clip and TS conformance | no (also the new check) |
  | M4 key `tail_every_peak: self.tail` | `tail_bound_tests` | yes |
  | M5 key `rest: Unstated` | `tail_bound_tests` | yes |
  | M6 harness check disabled | `every_faulty_mock_is_detected` only (the message in the record) | label only (m1) |
  | M7 expected `tail: Infinite` | `tail_bound_tests`; 3 graph-compiler fixture tests; the new check (4 runs) | no |
  | G2a drop the `tail_every_peak` comparison | gate-2 test ("a drifted `tail_every_peak` prepared") | yes |
  | G2b drop the `rest` comparison | gate-2 test ("a drifted `rest` prepared") | yes |

  For the uniqueness column, I applied M1+M2+M4+M5 together and ran test-debug-a and test-debug-b
  in full with `--no-fail-fast`. The only red test was `tail_bound_tests`. I ran M3 and M7 alone
  through both suites, M6 through test-debug-b, and G2a+G2b through test-debug-a. With both
  comparisons removed, the gate-2 test is the only red test. No other test reaches
  `effect.metadata.mismatch`: the only other `metadata_mismatch` code is graph-compiler's
  sidechain check (`compile.rs:191`).
- **No audio bit moves.** The change edits no render code. These pins are green at head:
  - `conformance_fixtures --check`;
  - `graph_fixture --check`;
  - `check-graph-determinism.sh` (100/100);
  - `audit capi`, with `pcm_digest` `cb10fbface44a3a4`, the value the #1460 verdict recorded at
    its base and head;
  - `check-browser-expected-resources.py --artifacts`, with its direct-oracle digest parity;
  - `run-wasm-gates.sh` G5 wasm digests;
  - both debug suites.

  The bank cohort key gained two fields, but both are functions of (descriptor, rate, quality),
  which the key already holds. So no grouping can change.
- **Sizes.** I measured on x86-64 with a temporary `size_of` test in a scratch copy of
  `crates/graph/src/lib.rs`, base against head, each in its own target directory:

  | Type | Base | Head |
  |---|---|---|
  | `NodeKind`, `RuntimeOp`, `RuntimeUnit` | 32, 112, 248 | 32, 112, 248 |
  | `EffectNode`, `LiveControlEffect` | 24, 128 | 24, 128 |
  | `GraphPreparedEffect` (control side) | 200 | 240 |
  | `GraphPreparedEffectBank` | 120 | 120 |
  | `PreparedEffectMetadata` | 104 | 144 |

  The record's table is exact. The render node table does not move, so Amendment 2, ruling 2 holds.
  The 40 B growth of each processor's own `PreparedEffectMetadata` copy is owned by #1461 and is
  not counted here.
- **Realtime safety.** `tail_and_rest` has two production call sites:
  - `expected_prepared_metadata` (`effect-contract/src/lib.rs:2677`), after
    `validate_prepare_request` has accepted the (rate, quality) row;
  - the conformance harness.

  Every caller of `expected_prepared_metadata` is on the preparation path, in response
  configuration or in tests (I searched every crate). Render never calls it. `audit capi` reports
  0 allocations, 0 deallocations, 0 locks, 0 syscalls and 0 violations, and
  `check-realtime-policy.sh` passes.
- **Naming and versions.** `RestBound`, `EffectTailBound`, `tail_and_rest`, `tail_every_peak`,
  `UndeclaredRestBound`, `metadata.tail_bound`, `tail_bound_tests` and `metadata_mismatch_tests`
  are all unversioned internal names. No wire, ABI or sealed spelling changes.
  `check-workspace-policy.sh` passes.
- **Placeholders and the no-shortcuts rule.** `RestBound::Unstated` and `tail_every_peak:
  Infinite` are documented as transitional values. They are not described as defaults:
  - The `RestBound` doc (`lib.rs:186-199`) says that `Unstated` exists only while #1372-#1376 land,
    that #1378 removes it together with `Infinite` in both tail fields, and that the end state has
    neither (D2, Amendment 1 ruling 1).
  - The `EffectTailBound::tail_every_peak` doc says it is `Infinite` because no `R(P*)` is derived.
  - Each effect's `tail_and_rest` doc names its own slice: #1372 EQ, #1373 multiband, #1374 delay,
    #1375 compressor and limiter, #1376 gate, transient shaper and soft clip.
  - `docs/EFFECT_CONTRACT_V1.md:104-107` says the same.
  - The order is genuine: #1372-#1376 depend on #1379 and on #1375's helper.

  I found no other interim shortcut in the slice.
- **Authorized paths.** Every changed file is in the list. `crates/graph/src/lib.rs` changes in
  test literals only (Amendment 2, ruling 2). The four `quality.tail` test reads moved onto
  `tail_and_rest(...).tail` with the same values. The only exception is m1's row.
- **The re-apply.** The `+`/`-` lines of `d0af9d53d` and `a5b4d0727` outside `.github` are
  identical. The record's "no code changed" is true.
- **Consistency of #1461 (`03db4574f`) with #1377.** They are consistent. #1461 is ordered after
  #1377. It names the copy that #1377's "Open" item describes. Its 15 `metadata()` anchors match
  `38ef4fe7a` (I checked each one). #1461 has scoping gaps of its own (Items for ROOT, item 4).
  None of them is a #1377 defect.

## Test-value sentences

- `effect_contract::tail_bound_tests::prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate`:
  an `expected_prepared_metadata` that writes the wrong `rest`, or calls `tail_and_rest` at the
  wrong rate (M1, M2), or a `program_key` that crosses or drops `tail_every_peak` or `rest` (M4,
  M5), turns it red, and no other test in either debug suite catches these. Every shipped
  descriptor states constant `Infinite`/`Unstated`, so only this test's per-rate distinct
  descriptor makes them visible.
- `effect_compiler::prepare::metadata_mismatch_tests::a_processor_whose_rest_or_tail_every_peak_drifts_is_refused`:
  a mismatch check that stops comparing `rest` (G2b) or `tail_every_peak` (G2a) lets a processor
  whose metadata differs from its descriptor's function prepare. This test is the only red test in
  test-debug-a with both comparisons removed.
- Conformance `metadata.tail_bound` and its `UndeclaredRestBound` row: no unique catch (m1).
  Disabling the check only drops the label, because `metadata.exact` still refuses the fault, and
  the wrong-expected-metadata defects it sees (M3, M7) are red in `tail_bound_tests` too.
- The four rewritten frozen-descriptor assertions (gate, transient shaper, soft clip contract
  tests; limiter `:5130-5134`) keep their existing value. They pin each effect's declared tail, now read
  through `tail_and_rest`. They are re-expressions, not new tests.

## Gates run (all at `38ef4fe7a`, all PASS)

- `cargo fmt --all -- --check`.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, and the same with
  `--all-features`.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
- test-debug-b, the spec's gate-3 command: 888 passed, 0 failed, 25 ignored.
- test-debug-a, the `qualification.yml` command with the `builtins-compiler --no-run` step: 1445
  passed, 0 failed, 10 ignored.
- `cargo run -p conformance --example conformance_fixtures -- --check`.
- `graph_fixture --check`, and `check-graph-determinism.sh` (100/100).
- Release build of `audit`, `bench`, `capi` and `session-validator`. `audit capi`: 0 allocations,
  deallocations, locks and syscalls; `total_violations` 0; `pcm_digest` `cb10fbface44a3a4`.
- `check-capi-abi.sh` and `--self-test`; `check-scalar-oracle-absent.py --native`.
- `check-effect-contract.sh target/release/bench`: 8 production factories, 0 failed gates.
- `check-builtins-fixtures.sh` and `check-console-fixtures.sh`.
- The lint-job policy scripts: workspace, session, test-support-ci, script-reachability,
  host-core, protocol-control, realtime, realtime-audit-leak, artifact-evidence-leak, lane,
  unfused-seal, rack, builtins, graph, effect-runtime policy and fixtures, conformance-boundaries,
  dsp-research.
- The worklet chain, with fresh output directories:
  - `build-web-audioworklet.sh --named-twin`;
  - `strip-wasm-names.py --self-test` and `check`;
  - `check-web-audioworklet.sh --without-metadata-regeneration`;
  - `check-browser-expected-resources.py --artifacts`;
  - `check-scalar-oracle-absent.py --wasm`;
  - `test-web-audioworklet.sh`.
- `run-wasm-gates.sh --without-v8-spill --without-native`.
- `check-cross-targets.sh`: PASS, with only the known #1018 iOS memset rows.
- Not run: AArch64 tests (CI only), and the `test-release` job's release unit tests.

## Items for ROOT

1. **Decide gate 1 (m1).** Gate 1's test-value sentence stopped being true when D3 put
   `tail_every_peak` and `rest` into the program key. There are two options:
   - Remove the harness check, the `UndeclaredRestBound` fault and its row. `metadata.exact` and
     `tail_bound_tests` already cover them, and AGENTS says that a change that supersedes a test
     deletes it.
   - Keep gate 1 as a contract statement, and amend its test value to say honestly that it has no
     unique catch today.

   I recommend removal. The per-effect slices will not give the check a unique catch either:
   `tail_bound_tests` already covers every generic copy defect with distinct per-rate values.
2. **Cost of `tail_and_rest` before #1372-#1376.** `expected_prepared_metadata` calls it for
   every prepared instance and again for every bank member at bind (each `bind_homogeneous_bank`
   recomputes the metadata of each candidate), and also in `EqResponseConfiguration::prepare`.
   Today the bodies are constants. The per-effect slices will put certified derivations behind it,
   and #1329's builtin derivation needed #1457's cache. D5 makes the value a function of
   (type, rate, quality) only. A ruling now (for example: compute once per type and rate when the
   registry is built, beside #1330's once-per-type validation) keeps each per-effect slice from
   building its own cache.
3. **No check holds the three values consistent.** Nothing validates that
   `tail_every_peak >= tail` (`T_rest = max(T_decay, R(P*))`), or that `Unstated` goes with
   `Infinite` and `Bounded` with a finite `tail_every_peak`. A per-effect slice could state an
   inconsistent set of values and no test would see it. A descriptor-validation rule over the
   launch rates (once per type, #1330) would catch this. It belongs to #1372-#1376 or #1330, not to
   this slice.
4. **#1461's scope (not a #1377 defect).** D1 removes the processor trait's `metadata()`, but the
   Authorized paths do not cover these readers and implementations:
   - Bank `metadata()` readers in `crates/rack/src/lib.rs` (`:781`, `:812`, `:820`, `:1005`,
     `:1023`, `:1059`) and in `crates/graph/src/runtime.rs:4888`.
   - Implementations in `crates/compressor/tests/native_points.rs`,
     `crates/effect-compiler/tests/native_session.rs`, `crates/rack/tests/live_control_bank.rs`
     and `tools/console-workload/tests/paired_spans.rs`.
   - About 45 effect-crate test files that call `.metadata()` on a prepared processor (`tests/`
     directories and `padding_tests.rs`).

   There are also two order gaps in STREAMS:
   - The hot-file note orders #1461 before #1372-#1376, but those rows' Dependencies columns do not
     list #1461.
   - The `effect-compiler/src/prepare.rs` and `effect-contract/src/lib.rs` hot-file rows do not
     include #1461.

   #1461 should be rebriefed, or should keep a non-render accessor, before it starts.
5. **#1378's path list is from before #1377.** `hosts/host-web/src/tests.rs`,
   `crates/effect-contract/tests/registry.rs` and the new test doubles now name `RestBound` and
   `TailSamples`. Its re-grep clause covers them; refresh its anchors when it starts.
6. **GitHub sync.** GitHub sync of #1377 still waits for the owner's permission (Amendment 1,
   ruling 4). Close the issue only after that permission and after the evidence commit is
   upstream.
