# Remove third-party effect packages and persisted effect-state migration

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

**Owner ruling (2026-09-28):** Approved.

**Blocked on two owner rulings.** Scoping study:
`docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 7, R6. The rulings to record:

- **R6a:** "Third-party effects are out of scope until a new issue reopens them." Open #27 and #28
  close as descoped.
- **R6b:** "There is no persisted DSP state or state migration without a product need."

R6a alone removes the package, CID and C-header surface. R6a and R6b together remove the whole
`effect-package` crate.

## Context

**What exists.**

- `crates/effect-package`: about 13,400 lines (about 5,350 production, 2,960 in-source tests and
  5,090 in `tests/`). It contains `cid.rs`, `package.rs`, `state.rs`, `wire.rs`, `ffi.rs` (707 lines,
  `c-abi` feature) and `diagnostic.rs`. It also ships `include/miso_engine_effect_descriptor_v1.h`
  (392 lines), a 917-line C smoke test in `tests/c/`, a `MUTATIONS.md`, and the `cdylib` crate
  type.
- `crates/effect-compiler`:
  - `src/migration.rs` (1,197 lines);
  - the state span of `src/prepare.rs` (imports `:11-21`, about `:95-844`: `state_replay`,
    `WireBoundNativeEffectFactory`, `bind_native_effect_factory_state`, restore admission,
    `prepare_unpublished_effect_bank_state`, scalar and bank snapshot and restore);
  - the tests that use `effect_package`: `bank_state.rs` (1,038), `scalar_state.rs` (913),
    `migration.rs` (1,073), `migration_terminal.rs` (2,381), `symmetry_restore.rs` (464) and
    `observation_identity.rs` (131).
- **Scripts** (about 3,400 lines):
  - the interchange qualification (`check-effect-interchange-qualification.sh`,
    `check-effect-interchange-targets.sh`, `effect-interchange-v1-reference.py`,
    `run-effect-interchange-reference-processes.sh`, `test-effect-interchange-policy.sh`,
    `test-effect-interchange-reference-runner.sh`,
    `test-effect-interchange-target-export-parser.sh`), 1,107 lines. `05-…` deletes the benchmark
    half.
  - `check-effect-package-v1.sh`, `effect-package-v1-reference.py`, `check-effect-descriptor-v1.sh`,
    `effect-descriptor-v1-reference.py`, `test-effect-descriptor-capi.sh`,
    `check-effect-state-migration-v1.sh` and `effect-state-v1-reference.py`.
- **Fixtures:** `fixtures/effect-package/v1`, `fixtures/effect-descriptor/v1`,
  `fixtures/effect-state/v1` and `fixtures/effect-interchange/v1` (25 files, about 48 KB).
- **Fuzz targets:** `fuzz/fuzz_targets/effect_package.rs` and `effect_state.rs`.
- **Docs:** `docs/EFFECT_PACKAGE_V1.md`, `EFFECT_DESCRIPTOR_WIRE_V1.md`,
  `EFFECT_INTERCHANGE_QUALIFICATION_V1.md`, `EFFECT_STATE_V1.md` and `EFFECT_STATE_MIGRATION_V1.md`.

**Browser dependency: none that executes.**

- `effect-package` is in the module's closure only through `effect-compiler`'s state and
  migration paths. Only tests and the #081 benchmark call those paths.
- host-core uses none of them. It uses `prepare_native_session_effects`,
  `launch_native_effect_registry`, `EffectCompileCaps`, control and observation attach,
  `EffectRack` and the EQ response query.
- **Compile proof** (audit, section 7). A scratch copy without the crate and without the
  effect-compiler state span passed `cargo check --workspace --all-targets --all-features` and
  the `wasm32` check.
- **Module comparison.** The module built from that copy has the same 2,749 functions with the
  same body sizes, the same code bytes (2,969,017) and the same data size. Only symbol order and
  one moved 2-byte string differ, so the pin moves but no rendered code does.

**Sessions.**

- `docs/session-v1.schema.json` accepts `{"kind":"cid"}` effect identities. `session` parses and
  validates them (`parse.rs:1009-1039`, `model.rs:282`, `validate.rs:317`).
- `effect-compiler` refuses them at `prepare.rs:897-902` with
  `effect.third_party.unavailable_at_launch`.
- graph-compiler has a never-bank arm (`banks.rs:37-44`), pinned by
  `third_party_dynamic_effects_are_never_bank_candidates`.
- Removing the `cid` identity is optional: it moves the refusal from compile time to parse time,
  a Session V1 grammar change you would own.

**CI.**

- lint: 20 s + 5 s.
- cross-target: 16 s + 5 s + a share of 37 s.
- `release-shape` pins `effect-package` in the cdylib set.
- test-debug-a: about 105 tests.
- nightly: 6 minutes of fuzzing, plus the `package_allocation` release-budget test.
- `fuzz.yml`'s `paths:` includes the crate.

**What stays:** native effects, the dynamic rack, `EffectBankPreparation`, the static descriptors
in `effect-contract`, `tools/parameter-metadata`, and control and observation.

## Smallest closable slice

1. **R6a.**
   - Delete `cid.rs`, `package.rs`, `ffi.rs`, the C header and C smoke test, the `c-abi` feature
     and the `cdylib` crate type.
   - Delete the package and descriptor scripts, their fixtures and the `effect_package` fuzz
     target.
   - Close #27 and #28 as descoped.
2. **R6b.**
   - Delete `state.rs`, `wire.rs`, and hence the whole crate.
   - Delete `effect-compiler`'s `migration.rs`, the `prepare.rs` state span and the test files
     above.
   - Delete the state scripts, the interchange qualification, the `effect_state` fuzz target, and
     the state and interchange fixtures.
   - `observation_identity.rs` (131 lines) has three tests. Port the two with live claims onto
     `effect-contract` descriptors:
     - test 2 (`:85`): the four dynamics effects each declare one "Gain Reduction" tap in dB,
       Resident;
     - test 3 (`:119`): `state_layout_version == 1`.

     Test 1 (`:46`) accounts for wire bytes, so it goes with the wire.
3. **CI and policies.**
   - `check-release-shape.py` drops `effect-package`.
   - Remove the interchange rows from `check-cross-targets.sh`.
   - Remove the two effect-package steps from the `cross-target` job, and the lint interchange
     step.
   - Remove the state-migration mutations from `test-effect-runtime-policy.sh`.
   - Remove the crate from `fuzz/Cargo.toml`, `fuzz.yml`'s `paths:` and header comment, and the
     nightly fuzz list and release-budgets line.
   - Remove its entries from `check-realtime-policy.sh`'s unsafe exclusions.
4. **Optional: remove the session `cid` identity.** It is a grammar change, so it needs its own
   explicit approval.
5. **Later:** remove the per-effect `snapshot_state_payload`/`restore_state_payload` hooks and the
   bank variants in `effect-contract` and the 8 effects. Each is left with only test callers. Put
   this in a separate issue with its own digest evidence.
   - `ChannelSymmetryWitness::RESTORED` (`effect-contract/src/symmetry.rs:172`) is set only in
     the deleted state span (`prepare.rs:292`, `:832`), but `rack-compiler/src/lib.rs:925` still
     reads it.
   - After R6b that pool-class row can never be reached. Say so in this issue, and remove it in
     the follow-up with its own digest evidence.
6. **Docs.** Update the five `EFFECT_*_V1.md` docs, mentions in `EFFECT_CONTRACT_V1.md`,
   `SESSION_SCHEMA_V1.md` and `IMPLEMENTATION_PLAN.md`, and AGENTS.md's "Third-party effects are
   designed now…" paragraph and scope line.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p effect-compiler`
     passes.
   - `cargo check --locked --manifest-path fuzz/Cargo.toml --bins` passes.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change. `bash scripts/run-wasm-gates.sh` passes.
3. **Shipped artifact.** Build base and change on one machine, as audit section 11 describes. Show that the function set, body
   sizes, code bytes and data size are identical. That is what the audit's scratch build found:
   only symbol order and string placement move. Then re-pin with that reason.
4. **CI routing.**
   - `check-release-shape.py` and its `--self-test` pass, as do `check-cross-targets.sh`,
     `check-effect-runtime-policy.sh`, `test-effect-runtime-policy.sh`,
     `check-effect-runtime-fixtures.sh`, `check-ci-path-routing.py` and
     `test-ci-path-routing.py`.
   - `fuzz.yml`'s `paths:` equals the fuzz crate's new closure.
   - The `verdict` table is unchanged.
5. **No live claim lost.**
   - Session refusal of a `cid` effect keeps its test, unless step 4 is approved.
   - The never-bank rule keeps `third_party_dynamic_effects_are_never_bank_candidates` while the
     `cid` identity exists.
   - `observation_identity.rs`'s claim is ported.
   - Every other removed test is listed with the removed feature it covered.

## Dependencies

The owner rulings, then `00-…` and `05-…`.

## Standing rules for the implementer

- Commit on `codex/<issue>-remove-effect-packages`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F10. The recommendation stands.

1. **Mobile playback does not need persisted state or migration.** Sessions carry no effect state
   (`docs/session-v1.schema.json` has no state member), `prepare_native_session_effects`
   (`effect-compiler/src/prepare.rs:873-1272`) never touches `effect_package` or restore, and
   neither host-core, host-web nor capi calls the state or migration paths (the one call in
   `graph-compiler/src/lib.rs:14269` is inside `mod tests`). Third-party Wasm on iOS would also
   need an interpreter, since iOS forbids JIT; that strengthens R6a rather than weakening it.
2. **The optional grammar step (4) must edit the SDK too.** `sdk/src/internal/session-json.ts:91`
   knows the `cid` identity in its canonical writer. No fixture or SDK test uses `cid`.
3. **Gate 3 is plausible but was not re-proved here.** The module-identity claim (same functions,
   code bytes and data size) is consistent with the call graph above; the implementer must still
   rebuild base and change on one machine.
4. **The C ABI stays live for mobile (R2 amended).** R6a removes only `effect-package`'s own C
   header and `c-abi` feature; it must not touch `crates/capi`.

## Attempt 1 evidence

Terra, 2026-09-28. The branch is `codex/1037-remove-effect-packages`:

- `c59f9345` holds the change.
- `14674297` merges batch head `9644196d` (#1052, #1042, #1024, #1025).
- `0cf45472` drops #1052's source-scrape allow-list rows for the deleted files.
- `75c4c080` merges batch head `c867ec2a` (#1026, #1056). Its message lists the conflict
  resolutions, each the union of both removals. #1026 deleted `tools/bench/src/builtins.rs` and
  this change deleted `effect_interchange.rs`, which were the last two delegating escaper wrappers.
  So `test-bench-policy.sh`'s escaper-candidate and delegate-parser cases now seed a wrapper into
  `conformance.rs` first.
- `7e252bc4` drops the five env names the merged tree no longer reads.

The comparisons below use base `c867ec2a` against the merged head. Two earlier passes, with bases
`b8bea8e1` and `9644196d`, gave the same result.

### What was delivered

- **R6a and R6b (steps 1-3).** Deleted:
  - `crates/effect-package`, with its C header, C smoke test, `c-abi` feature and `cdylib`;
  - effect-compiler's `migration.rs`;
  - the `prepare.rs` persisted-state span: `state_replay`, `WireBoundNativeEffectFactory`,
    `bind_native_effect_factory_state`, restore admission, the unpublished bank state and the
    scalar and bank snapshot and restore;
  - the test files `bank_state.rs`, `scalar_state.rs`, `migration.rs`, `migration_terminal.rs` and
    `symmetry_restore.rs`;
  - the 14 package, descriptor, state and interchange-qualification scripts;
  - the four fixture directories, the two fuzz targets and the five `EFFECT_*_V1.md` docs.

  `observation_identity.rs` keeps tests 2 and 3, now reading only the `effect-contract`
  descriptors. Test 2 loses one line, the 48-byte record arithmetic, which was wire accounting.
  `lane` becomes an effect-compiler dev-dependency, because only `tests/symmetry_designed_words.rs`
  uses it now.
- **#1028 absorbed.** R6 was ruled before #1028 merged, and #1028 says it is absorbed in that case.
  This deletes the `effect-interchange` bench subject, its seven scripts, its env-vocabulary names
  and the unread `artifacts/issue455-interchange-completion/`. After the #1026 merge, the
  benchmark binary-digest, phase-marker and fake-bench names lost their last readers too. #1028's amendment is applied:
  `test-bench-policy.sh`'s delegate cases are re-pointed to `tools/bench/src/builtins.rs`, the one
  remaining delegating wrapper.
- **CI and policies.**
  - `qualification.yml`: dropped the lint interchange step and the two cross-target
    package/descriptor steps. The cross-target job no longer installs wabt, since nothing in
    `check-cross-targets.sh` uses `wasm-objdump` now.
  - Nightly: dropped the two fuzz targets and the `package_allocation` budget.
  - `fuzz.yml`'s `paths:` now equals the fuzz crate's `cargo metadata` workspace closure:
    `engine`, `protocol` and `session`. `effect-contract`, `lane` and `math` left it along with
    `effect-package`.
  - Updated gates: `check-release-shape.py` (four cdylib/staticlib packages),
    `check-cross-targets.sh`, `check-realtime-policy.sh` (three unsafe exclusions),
    `check-effect-runtime-policy.sh` (the package-reference and migration scans are gone),
    `check-conformance-boundaries.sh` and `check-ci-path-routing.py` (three nightly budgets).
    Their mutation suites were updated to match.
  - `check-artifact-evidence-leak.sh` now counts `effect-compiler` as shipped, so the N1 split in
    `check-cross-targets.sh` is still gated. effect-compiler is in every host's closure.
  - The `verdict` table is unchanged.
- **Docs (step 6).** Updated AGENTS.md's third-party paragraph and scope line, `EFFECT_CONTRACT_V1.md`,
  `SESSION_SCHEMA_V1.md`, `IMPLEMENTATION_PLAN.md`, `REALTIME_DEPENDENCY_POLICY.md`,
  `TARGET_MATRIX.md`, `STEM_IDENTITY_V1.md`, `STEM_STORE_V1.md`, `docs/README.md` and
  `ENGINE_ENV_VOCABULARY.md`. Documented env names went from 117 to 90, and
  `test-env-vocabulary.sh`'s pinned count follows.
- **Not done: step 4, the `cid` grammar.** It needs its own approval, and R6a/R6b do not record
  one. The grammar, the SDK's canonical writer (`sdk/src/internal/session-json.ts:91`), the
  compile-time refusal `effect.third_party.unavailable_at_launch` and
  `third_party_dynamic_effects_are_never_bank_candidates` all stay, and they stay consistent with
  each other. `crates/capi` is untouched.
- **Step 5 is the follow-up.** `ChannelSymmetryWitness::RESTORED` is now never cleared: only the
  deleted span cleared it. So the `MonoSymmetricAtPrepare` test row that reads it
  (`rack-compiler/src/lib.rs:916`, inside `mod tests`) describes a witness no production path can
  produce any more. The per-effect
  `snapshot_state_payload`/`restore_state_payload` hooks and their bank variants now have only test
  and evidence-tool callers (`conformance`, `tools/bench`). The follow-up removes them with its own
  digest evidence.
- **Lines, against `c867ec2a`, before this evidence:** +199 / -38,004. The deletions break down
  as:

  | area | lines removed |
  | --- | ---: |
  | `crates/effect-package` | 14,856 |
  | effect-compiler | 7,896 |
  | the #455 artifact | 7,188 |
  | scripts | 5,379 |
  | fixtures, fuzz, bench, CI and manifests | 2,107 |
  | docs and AGENTS.md | 567 |

### R6b: nothing live used the deleted state paths

- **Compile proof.** With the span and the crate deleted, these all pass:
  - the native workspace check with `--all-targets --all-features`;
  - the wasm `simd128` check of `host-web` and `effect-compiler`;
  - `aarch64-apple-ios` and `aarch64-linux-android`: `--workspace --lib --all-features` check and
    clippy, and `--all-targets --all-features` for capi, host-core, host-mobile, effect-compiler,
    effect-contract, graph-compiler, engine and session.

  So no Rust caller exists in capi, host-core, host-web or host-mobile.
- **The SDK.** It reaches Rust only through host-web's wasm exports, and it has no state
  snapshot or restore surface: a grep of `sdk/src` and `hosts/host-web/web` finds none. The
  shipped module's functions are unchanged; see the artifact section below.
- **Tests of live claims.** The deleted tests are listed below. Every one exercised the persisted
  envelope, migration, the wire or the removed benchmark.
- **In-memory state kept.** None of it is touched:
  - `EffectBankPreparation`, which graph-compiler bank binding and host-core `control_provider`
    use;
  - the effect-contract payload hooks;
  - the banks' `copy_state_from` and `desymmetrize_channels`;
  - everything plan replacement uses.

### Gates

| gate | result |
| --- | --- |
| `cargo check --locked --workspace --all-targets --all-features` | pass |
| clippy `-D warnings`, default and `--all-features` | pass |
| `cargo fmt --all --check` | pass |
| `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` | pass |
| wasm `simd128` check of `-p host-web -p effect-compiler` | pass |
| `cargo check --locked --manifest-path fuzz/Cargo.toml --bins` | pass (six binaries) |
| iOS and Android `--lib` check and clippy, plus the `--all-targets` product set | pass |
| `cargo test --locked --workspace` | pass: 2,320 passed, 0 failed, 38 ignored |
| effect-compiler, graph-compiler, host-core and host-web with test-support | pass: 541 passed |
| release `-p bench -p console-workload -p audit` | pass: 154 passed |
| console `gain_pan_profile digests` | 17 rows, byte-identical to base |
| `bash scripts/run-wasm-gates.sh` (native, wasm scalar, wasm simd128, V8 EQ loops) | pass |
| `check-cross-targets.sh` | pass |
| `check-release-shape.py` and `--self-test` | pass |
| `check-ci-path-routing.py` and `test-ci-path-routing.py` | pass |
| `check-effect-runtime-policy.sh`, `test-effect-runtime-policy.sh`, `check-effect-runtime-fixtures.sh`, `test-effect-runtime-fixtures.sh` | pass |
| realtime, conformance-boundary, artifact-evidence-leak, bench and env-vocabulary checks and their tests | pass |
| `check-workspace-policy.sh` and `test-workspace-policy.sh`, after the #1052 merge | pass |
| `check-step-vocabulary.py` | pass at `c867ec2a`; it failed at `9644196d` on #1052's own spec, which the batch then fixed |
| the remaining `check-*.sh` and argument-free `check-*.py` | pass |
| `check-sdk-generated.sh`, `check-sdk-types.sh`, `check-sdk-deletions.py` and `--self-test`, `check-sdk-headless.sh` | pass |
| `check-capi-abi.sh` | pass (shared and static linkage); `crates/capi` and its header are unchanged |
| `check-graph-determinism.sh`, `check-protocol-wasm-parity.sh`, `check-realtime-audit-leak.sh`, `check-wasm-realtime-atomics.sh`, `check-effect-contract.sh` | pass |

`sdk-package.sh check` was not run. It needs the full pinned artifact closure, and mid-batch
`build-web-audioworklet.sh` refuses to produce that without a re-pin, which this brief rules out.
`sdk/` is unchanged.

### Gate 3: the shipped artifact

Both modules were built on one machine with `build-web-audioworklet.sh --module-only`:

- base `c867ec2a`: `f7bd75ca…`, the same module as at `9644196d`;
- change: `6867027b…`.

Neither is re-pinned; the batch boundary does that.

- **Sizes.** Both modules have the same section sizes: code 2,965,873 and data 101,679.
- **Functions.** Both have 2,733 functions, with body bytes 2,961,091 in each and the same
  body-size multiset.
- **Instructions.** Every function is instruction-for-instruction identical (1,315,283
  instructions) once these are normalised:
  - call targets, because function order moved;
  - symbol crate hashes;
  - `Ms<n>_` impl disambiguators, because effect-compiler's metadata and impl order moved.
- **Data.** 196 bytes differ, and the data size is the same:
  - two panic `Location` line numbers in `effect-compiler/src/prepare.rs`, 1042 to 280 and 1289
    to 527, which are the 762 deleted lines above them;
  - a reordering of the transient-shaper and multiband-compressor descriptor strings in rodata,
    with the 16 pointer words that address them moving by 2 bytes.

No rendered code moved, which agrees with the unchanged console digests and wasm gates.

### Tests: `cargo test --workspace --all-targets --all-features -- --list`

Base 2,456 and change 2,347: 109 removed and none added. The same 109 went against `b8bea8e1`
(2,556 to 2,447) and against `9644196d` (2,560 to 2,451). Each removed test is listed with the feature it covered:

- **effect-package, 73 tests. All removed under R6a (wire, package and CID, C header) or R6b
  (state):**
  - lib unit tests: 35;
  - `descriptor_v1_qualification`: 9;
  - `effect_interchange_abi`: 2;
  - `effect_interchange_mutation`: 1;
  - `package_v1_qualification`: 3;
  - `package_vectors`: 2;
  - `state_vectors`: 15;
  - `package_allocation`: 6.
- **effect-compiler, 36 tests:**
  - the persisted-envelope snapshot and restore (R6b): `bank_state` 6, `scalar_state` 7 and
    `symmetry_restore` 3. The per-effect hook round trips they drove through the envelope are
    still tested in each effect crate: `soft-clip/tests/state_roundtrip.rs`,
    `gate-expander/tests/state.rs` and the parametric-eq `contract.rs` tests;
  - state migration (R6b): `migration` 6 and `migration_terminal` 11;
  - `observation_identity::every_declared_tap_costs_exactly_its_record_and_its_two_strings`, which
    was descriptor-wire byte accounting (R6a).
- **bench, 2 tests, from the #1028 subject:**
  - `exact_four_rate_migration_envelope_without_timing` is lost with R6b, as #1028's gate 5
    allows when R6 is ruled first;
  - `digest_hex_matches_known_abc_digest` is held by `bench-support/src/digest.rs`'s own `abc`
    known-answer test.

### Notes for root

- #27 and #28 close as descoped, under R6a. Per this brief, I did not edit GitHub.
- These open specs name files this change deletes:
  - #1036 (`effect-package/src/wire.rs`, `check-effect-descriptor-v1.sh`);
  - #1046 and #1047 (effect-package and migration test rows, already marked as #1037's);
  - #1062 (the package and descriptor scalar legs, which its amendment 2 anticipates).
