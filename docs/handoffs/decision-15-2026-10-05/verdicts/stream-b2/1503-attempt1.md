PASS

# #1503 attempt 1: verdict (adversarial verifier, 2026-10-11)

Commits 5df1ce8b9 and 9fddf7560 on `codex/d15-stream-b2` (base 70017bf09), judged against the spec as
amended by root in ed62ed794 (Amendment 1). I exported 9fddf7560 to `/tmp/claude-1002/v1503/tree`
(gates) and to `/tmp/claude-1002/v1503/mut` (mutations), touched every file, and built with external
target dirs. I did not build in the worktree or edit it. Small evidence: `/tmp/claude-1002/v1503/ev/`;
private scripts: `/tmp/claude-1002/v1503/scripts/`.

The attempt meets amended D1-D4, the four deliverables and gates 1-7. Every changed path is in the
authorized list (`crates/control-plane/src/compile.rs` is untouched, as Amendment 1 requires). There is
no BLOCKER, no MAJOR and no MINOR. The findings are NITs.

## Requirement check

- **D1.** `prepare_transaction` (`crates/protocol/src/model.rs:937-940`) refuses `current >=
  GATE_REVISION_MAX` with `RevisionExhausted`. The check replaces the `checked_add` in place: after
  ExactRevisionRequired, RevisionConflict and EmptyTransaction, and before the candidate clone, the
  edits and the compile. The store's typed refusal wins over the schema's. Mutations M1 (back to
  `checked_add`) and M2 (no store check, so the compile refuses) both report
  `Validation { .., NumericOutOfSchemaRange at $.revision }` in place of `RevisionExhausted`, and the
  protocol and capi tests go red.
- **D2.** `session::MAX_SESSION_REVISION = engine::realtime::GATE_REVISION_MAX` (`crates/session/src/lib.rs:40`).
  `validate_session` (`validate.rs:41-48`) adds one `NumericOutOfSchemaRange` at `$.revision`.
  - The diagnostic_parity test shows exactly one diagnostic through parse (with its span), typed
    canonical and typed compile, for 2^63 and `u64::MAX`. The ceiling parses, round-trips byte for
    byte and compiles.
  - `SessionStore::new` returns that set.
  - The JSON Schema's `$defs/revision` is the review's pattern, verbatim. I checked it against integer
    comparison for 416,649 values: every digit-length edge, every one-digit change at each position of
    2^63 - 1, ±2000 around the ceiling, the u64 edge, and 500k random values. There was no mismatch.
    Non-canonical spellings (`""`, `00`, `01`, `-1`, `+1`, `" 1"`, `1.0`) are refused. (Python's `$`
    also matches before a trailing `\n`. ECMA-262 and the Rust validator do not, and `$defs/u64` has
    the same form, so this does not count.)
  - The SDK builder refuses `> 9223372036854775807n` with `numeric.out_of_schema_range`. The fold
    refuses `revision >= MAX_SESSION_REVISION` with "session revision exhausted". The schema version
    stays 1. The wire and C ABI fields stay u64.
- **D3.** `PlanExchangeError { Resources(SpscError), RevisionAboveCeiling { revision } }` is
  re-exported at `engine::realtime`. The refusal comes after the own-gate assert and before
  `plan_exchange_inner`. `plan_exchange` keeps `SpscError`. `compile.rs:776-784` still compiles and
  maps both variants. Evidence of defence in depth: with the validator disabled (M3), the capi test
  fails at create with `capi.plan.exchange $`. That is the D3 refusal stopping an above-ceiling
  session at the exchange.
- **D4 / the acked-batch question: can an ack ever precede a drop? No.** In the controller
  (`controller.rs:1766-1776`), `prepare_transaction` runs before the response frame, the replay
  reservation, any live record push and the gate publication. A refusal returns
  `Immediate { outcome: non_ok }`, so it acknowledges nothing and writes nothing. The capi test
  confirms this from outside: the whole `TestTransactionSnapshot` (canonical, queues, live rooms,
  owners) is equal apart from one replay entry, the watermark is unchanged, and the output is
  bit-identical to a twin's.
- **Bypass of `validate_session`.** None found. `CompiledSession` is built only in
  `compile_session` (`session/src/compile.rs:197`). `SessionStore::new` is the only store constructor,
  and it compiles. Its one production call is `control-plane/src/compile.rs:734`, after the parse at
  :720. The host-core loaders call `parse_session_json` and `compile_session` (`prepare.rs:668, :719`).
  Every revision increment goes through `prepare_transaction`. The other `checked_add(1)` sites
  (host-web input-filter owner, effect-compiler effect-control owner, observation job revisions in the
  SDK) are owner revisions, and none of them reaches a plan gate. Production gate publication is
  `control.rs:1601`, which publishes `store.revision()`.
- **Gate 4, measured for real.** `assert_installed()` runs. The warm exchange at the ceiling allocates
  (live probe). The refusals at 2^63 and `u64::MAX` allocate 0, and the gate does not change. Moving
  the check after `plan_exchange_inner` (M5) turns it red with "the refusal allocated", left 5. `>=`
  (M6) turns it red with "the ceiling itself is accepted".
- **Gate 3, C ABI.** The test is in `crates/capi/src/runtime/tests.rs` with its own helpers, which is
  authorized. `compile_children` is the function `miso_engine_v1_compile_session` calls
  (`ffi.rs:382`), so creation is refused as the C ABI refuses it.

## Findings

- **NIT n1, the `invalid_matrix.rs` row was deleted, not changed** (`crates/session/tests/invalid_matrix.rs:58-59`).
  I accept this. The test is named `u64_model_fields_accept_the_full_unsigned_domain`, and a refusal
  row does not belong in it. A ceiling-acceptance row would duplicate diagnostic_parity, which pins
  both the typed-canonical refusal and the ceiling round trip. Root may want to note that the spec's
  "row change" became a deletion.
- **NIT n2, `docs/C_ABI_V1_QUALIFICATION.md:145`.** The added sentence leaves an unwrapped 140-column
  line in a paragraph that is otherwise wrapped (the file already has long lines elsewhere).
- **NIT n3, `docs/CONTROL_PROTOCOL_SEMANTICS.md:7`.** "The committed `SessionModel` owns its revision
  and lies in `0..=…`": the grammatical subject of "lies" is the model, not the revision.
- **NIT n4, `sdk/test/builder-evals.mjs`.** The engine half of the new test is guarded by
  `if (asset !== undefined)`. The guard is false only under `MISO_ENGINE_SDK_SKIP_ASSET=1`, and CI
  loads the asset (the test ran and passed here). Nearby tests use `asset` without a guard.
- **NIT n5, `json_grammar.rs:140`.** The lexical vehicle moved to `frames`, so `"0"` became `"1"`
  (frames 0 is semantically invalid). The lexical `"0"` stays covered by `canonical-minimal.json`'s
  revision `"0"`, so nothing is lost.

## Test value (one sentence each)

- `protocol` `the_last_commit_reaches_the_ceiling_and_the_next_is_exhausted_before_any_work`: red when
  the store's bound is still `u64::MAX` or relies on the schema (M1, M2: `Validation` in place of
  `RevisionExhausted`), or when the validator refuses the ceiling itself (M4).
- `protocol` `a_session_above_the_ceiling_is_refused_by_the_one_constructor`: red when the store
  constructor accepts an above-ceiling model (M3).
- `session` `revision_above_the_plan_gate_ceiling_is_refused_by_every_entry_point`: red when any of
  parse, canonical or compile skips the bound, or puts it at the wrong edge (M3, M4).
- `session` `session_schema_revision_domain_is_the_validators_ceiling`: red when the normative schema
  disagrees with the validator, at the ceiling or by one (M8: schema back to `$defs/u64`; M9: pattern
  off by one).
- `engine` `an_exchange_above_the_gate_ceiling_is_refused_before_it_allocates`: red when the ceiling
  check sits after `plan_exchange_inner` (M5, the refusal allocated 5 times) or refuses the ceiling
  (M6).
- `capi` `the_c_abi_bounds_revisions_at_the_plan_gate_ceiling`: red when a host path commits past the
  store's bound (M1, M2: `ValidationFailed`), when creation skips the domain (M3), or when a refused
  edit leaves a record, a model change or a sample behind (snapshot, watermark and twin equality).
- `session-validator` `mapped_source_shape_and_map_errors_are_transactional` (changed): red when the
  fold's own bound is removed (M7: "transformed session is invalid" in place of "session revision
  exhausted").
- SDK builder-evals `the revision ceiling is the engine's…` and enginectl-cli (changed): red when the
  builder's bound drifts to `u64::MAX` (SDK mutation: both `not ok`).
- Changed vehicles (`json_grammar.rs` two tests, `json_contract_artifacts.rs` u64 test,
  `invalid_matrix.rs`): repointed to `frames`, so full-u64 lexical and schema coverage stays.

## Gates run (on the export of 9fddf7560)

- Gate 5 cargo: `cargo test --locked` for `-p protocol --features test-support`,
  `-p engine --features realtime-audit`, `-p control-plane --features test-support`, `-p capi`,
  `-p session` and `-p session-validator`. All exit 0.
- `cargo fmt --all -- --check` and `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: exit 0.
- Policy scripts, all exit 0: `check-protocol-control-policy.sh`, `check-bench-policy.sh`,
  `check-realtime-audit-leak.sh`, `check-workspace-policy.sh`, `check-realtime-policy.sh`.
- `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
  `./target/release/audit capi`: allocations 0, deallocations 0, syscalls 0, total_violations 0.
  `check-capi-abi.sh` and `check-capi-abi.sh --self-test`: exit 0. `check-cross-targets.sh`: PASS.
- The qualification `sdk` step: `npm ci` in the export's `sdk/`, then the artifact from
  `build-web-audioworklet.sh --named-twin`, then `check-sdk-generated`, `check-sdk-deletions`,
  `check-sdk-types`, `check-sdk-headless` (363 pass) and `sdk-package.sh check` (enginectl 18 pass).
  All exit 0. I deleted the stray `sdk/dist`.
- The worklet chain: `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm` and
  `test-web-audioworklet.sh` with a private TMPDIR. All exit 0, and the TMPDIR was empty afterwards.
- Mutations M1-M9 plus the SDK mutation: each one red as named above, then reverted. Logs:
  `ev/mutations.txt` and `ev/mut-*.log`.
- Not run: the AArch64 parity gates and the three-browser matrix (CI only).
