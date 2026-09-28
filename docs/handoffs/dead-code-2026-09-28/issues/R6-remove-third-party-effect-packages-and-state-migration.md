# Remove third-party effect packages and persisted effect-state migration

**Blocked on two owner rulings.** Scoping study:
`docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 7, R6. The rulings to record:

- **R6a:** "Third-party effects are out of scope until a new issue reopens them." Open #27 and #28
  close as descoped.
- **R6b:** "There is no persisted DSP state or state migration without a product need."

R6a alone removes the package, CID and C-header surface. R6a and R6b together remove the whole
`effect-package` crate.

## Context

**What exists.**

- `crates/effect-package`: 13,389 lines (5,354 production, 2,961 in-source tests, 5,088 in
  `tests/`). It contains `cid.rs`, `package.rs`, `state.rs`, `wire.rs`, `ffi.rs` (707 lines,
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
   - `observation_identity.rs` (131 lines) may guard a live observation-identity claim: port what
     it needs onto `effect-contract` descriptors instead of deleting it.
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
   bank variants in `effect-contract` and the 8 effects, and the dual-mono witness's `RESTORED`
   term. Each is left with only test callers. Put this in a separate issue with its own digest
   evidence.
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
3. **Shipped artifact.** Build base and change on one machine. Show that the function set, body
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
