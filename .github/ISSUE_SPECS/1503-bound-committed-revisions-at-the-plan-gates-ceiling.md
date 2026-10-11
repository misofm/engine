# Bound committed revisions at the plan gate's ceiling

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-17).
Code anchors verified on `codex/d15-stream-b` at `13f335d79`.

## Product outcome

No committed session revision ever exceeds `GATE_REVISION_MAX = 2^63 - 1`, the largest revision a
plan's revision gate can hold (#1432 D1: bit 63 is render's announcement).

- Creating or restoring a session above the ceiling is refused.
- A transaction that would pass it is refused with `RevisionExhausted` before anything changes.
- A plan exchange cannot start above it.

The boundary is stated and tested instead of being left to an overflow that would corrupt the
gate's announcement bit in a release build.

## Context

- **Root ruling 2 (2026-10-05), amended 2026-10-11.** The ceiling is part of the session schema's domain: `revision` lies in `0..=2^63 - 1`. `session::validate_session` (validate.rs:30) is the one validator, and `parse_session_json` (parse.rs:623), `compile_session` (compile.rs:129) and `canonical_session_json` (canonical.rs:12) all call it. So creation, restore, every host loader and every transaction candidate refuse an above-ceiling revision through one site. The store keeps its own typed transaction bound (D1). `plan_exchange_at_revision` refuses an initial revision above the ceiling (D3).
  - The review recommended the store over a control-plane pre-check, which a future commit path
    could bypass.
- `SessionModel` has public fields, so the bound is semantic (validator), not lexical (parser). parse.rs:740 keeps the u64 type domain.
- These state the revision's domain and change with it: `docs/session-v1.schema.json:11`, `docs/SESSION_SCHEMA_V1.md:27-29`, `docs/CONTROL_PROTOCOL_SEMANTICS.md:7`, and the SDK builder `sdk/src/core/session.ts:1029`. `enginectl session build` checks with the embedded engine, so a stale SDK bound fails `sdk/test/enginectl-cli.mjs:185-192`.
- **The store.**
  - `SessionStore::new` compiles the initial model (`crates/protocol/src/model.rs:879-884`). It
    is the one constructor, used for boot and restore (`crates/control-plane/src/compile.rs:720`).
  - `apply_transaction` computes `next_revision = current.checked_add(1)` and refuses with
    `SessionStoreError::RevisionExhausted` (`:932-935`, the variant at `:846`).
  - Its test pins the `u64::MAX` boundary (`MAX_SESSION_REVISION`, `:1172`, `:1395-1425`).
- `crates/protocol` already depends on `engine` (`crates/protocol/Cargo.toml:17`), so it can name
  `engine::realtime::GATE_REVISION_MAX` (#1432 D3).
- The session parser reads `revision` as a full `u64` (`crates/session/src/parse.rs:740`), so a
  document can carry a revision above the ceiling.
- `plan_exchange_at_revision` takes the initial plan's gate handle after
  *Give each plan its own revision gate and take each block's live snapshot from it*
  (#1502 D5).

## Decisions frozen for this slice

- **D1. The transaction bound (amended 2026-10-11).** `prepare_transaction` (model.rs:917, called by `apply_transaction` and controller.rs:1768) refuses with `RevisionExhausted` when `current >= GATE_REVISION_MAX`. The check replaces the `checked_add` at :932-935 in place: after ExactRevisionRequired, RevisionConflict and EmptyTransaction, and before the candidate clone, any edit and the compile. So the store's typed refusal always wins over the schema's, and nothing is written (D15-2 condition 3).
- **D2. The revision domain (amended 2026-10-11).** The session schema's `revision` domain is `0..=MAX_SESSION_REVISION`, where `session::MAX_SESSION_REVISION` is a `pub const` defined as `engine::realtime::GATE_REVISION_MAX`. `validate_session` refuses a larger revision with one `NumericOutOfSchemaRange` at `$.revision`. `SessionStore::new` needs no check of its own: its `compile_session` returns that `DiagnosticSet`. A model at exactly the ceiling is accepted, and its first transaction is refused by D1. The JSON Schema gives `revision` its own `$defs/revision`, with this pattern (the review checked it against 200k random values and the edges):
  `^(?:0|[1-9][0-9]{0,17}|[1-8][0-9]{18}|9[0-1][0-9]{17}|92[0-1][0-9]{16}|922[0-2][0-9]{15}|9223[0-2][0-9]{14}|92233[0-6][0-9]{13}|922337[0-1][0-9]{12}|92233720[0-2][0-9]{10}|922337203[0-5][0-9]{9}|9223372036[0-7][0-9]{8}|92233720368[0-4][0-9]{7}|922337203685[0-3][0-9]{6}|9223372036854[0-6][0-9]{5}|92233720368547[0-6][0-9]{4}|922337203685477[0-4][0-9]{3}|9223372036854775[0-7][0-9]{2}|922337203685477580[0-6]|9223372036854775807)$`
  - The SDK builder refuses a revision above `9223372036854775807n` with `numeric.out_of_schema_range`.
  - The session-validator fold refuses with "session revision exhausted" when `revision >= session::MAX_SESSION_REVISION`.
  - The schema version stays 1 (prelaunch narrowing). The wire and C ABI fields stay u64.
- **D3. The exchange (amended 2026-10-11).** `plan_exchange_at_revision` returns `Result<_, PlanExchangeError>`, with `PlanExchangeError { Resources(SpscError), RevisionAboveCeiling { revision: u64 } }`. It refuses after the own-gate assert and before `plan_exchange_inner`, so it allocates and publishes nothing. `plan_exchange` keeps `SpscError`. The control plane's existing `.map_err(|_| failure(ResourceFault::PlanExchange))` (compile.rs:784) maps both variants unchanged. D2 makes this unreachable from the control plane.
- **D4. The acked-batch question: can an ack ever precede a drop? No.** Every refusal comes before
  the first write of its transaction or session, and nothing is acknowledged.

## Deliverables

1. D1 in `crates/protocol/src/model.rs`, with its test moved to the ceiling.
2. D2:
   - `MAX_SESSION_REVISION` and the validator check in `crates/session/src/`.
   - The schema, prose and protocol-semantics docs.
   - The existing full-u64 revision tests repointed to `frames`, or reduced to non-revision leaves.
   - The SDK builder bound and its two tests.
   - The fold bound and its test.
3. D3 in `crates/engine/src/realtime/plan_exchange.rs`, re-exported from `realtime/mod.rs`, and its integration test.
4. One C ABI test, and the sentence in `docs/C_ABI_V1_QUALIFICATION.md`.

## Authorized paths

- `crates/protocol/src/model.rs`
- `crates/session/src/{lib.rs,validate.rs}`
- `crates/session/tests/{diagnostic_parity.rs,json_grammar.rs,invalid_matrix.rs,json_contract_artifacts.rs}`
- `docs/session-v1.schema.json`, `docs/SESSION_SCHEMA_V1.md`, `docs/CONTROL_PROTOCOL_SEMANTICS.md`
- `crates/engine/src/realtime/{plan_exchange.rs,mod.rs}`
- `crates/engine/Cargo.toml` (removal of the `bench-support` dev-dependency only), `Cargo.lock` (the `engine` package's `bench-support` line only)
- `tools/bench-support/tests/plan_exchange_ceiling.rs` (moved from `crates/engine/tests/plan_exchange_ceiling.rs`, which is deleted)
- `tools/session-validator/src/lib.rs` (fold bound only), `tools/session-validator/tests/fold_mono.rs`
- `sdk/src/core/session.ts` (revision bound only), `sdk/test/builder-evals.mjs`, `sdk/test/enginectl-cli.mjs`
- `crates/capi/src/runtime/tests.rs` or `crates/capi/tests/`, `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- The gate itself (#1432) and its place in each plan (#1502).

## Objective gates

1. **Store bound** (protocol unit test, replacing model.rs:1395-1425 and the `MAX_SESSION_REVISION = u64::MAX` constant at :1172).
   - A store at `GATE_REVISION_MAX - 1` commits one transaction, to `GATE_REVISION_MAX`.
   - The next non-empty exact transaction returns exactly `Err(RevisionExhausted)`, not `Validation`, and the canonical snapshot is unchanged.
   - `Any` and empty transactions at the ceiling keep their earlier refusals.
   - Test value: a store still bounded at u64::MAX, or a D1 that relies on the schema and so reports `Validation` instead of REVISION_EXHAUSTED.
2. **Schema domain** (session, a new `diagnostic_parity.rs` case plus the `invalid_matrix.rs` row change).
   - `revision = MAX_SESSION_REVISION + 1`, and also `u64::MAX`, is refused with exactly one `NumericOutOfSchemaRange` at `$.revision` through `parse_session_json`, `compile_session` and `canonical_session_json`.
   - `MAX_SESSION_REVISION` parses and round-trips.
   - Protocol: `SessionStore::new` with `GATE_REVISION_MAX + 1` returns that set.
   - Test value: a session entry point, or a store constructor, that skips the validator, so an above-ceiling session is accepted and its first publication corrupts its gate's announcement bit.
3. **C ABI (new capi test).**
   - A session created from a document whose revision is `GATE_REVISION_MAX + 1` is refused, as
     the C ABI refuses any document the compiler refuses.
   - A session at `GATE_REVISION_MAX - 1` takes one live fader edit: the watermark then reads
     `GATE_REVISION_MAX`.
   - The next edit is refused with the revision-exhausted status. No record is pushed, the output
     equals a twin without that edit, and the watermark is unchanged.
4. **Exchange** (a bench-support integration test in its own binary, `tools/bench-support/tests/plan_exchange_ceiling.rs`, using `bench_support::alloc`'s thread-scoped counters; it runs in the required `test-debug-a` job; Amendment 2).
   - First, a warm call: an exchange at `GATE_REVISION_MAX` succeeds, and the thread counters show allocations > 0. This proves the probe is live.
   - Then the call at `GATE_REVISION_MAX + 1` returns `RevisionAboveCeiling { revision }`, with `allocations == 0` across the call (the dropped plan's deallocations are expected), and the gate's `revision()` unchanged.
   - Test value: a ceiling check placed after `plan_exchange_inner`, which allocates the exchange before refusing.
5. **Workspace.** `cargo test --locked -p protocol --features test-support`;
   `cargo test --locked -p engine --features realtime-audit`;
   `cargo test --locked -p control-plane --features test-support`; `cargo test --locked -p capi`;
   `bash scripts/check-protocol-control-policy.sh`; `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; plus:
   - `cargo test --locked -p session`
   - `cargo test --locked -p session-validator`
   - `cargo test --locked -p bench-support --test plan_exchange_ceiling` (Amendment 2)
   - the qualification `test-release` loom leg, `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom` (13 tests; Amendment 2)
   - `bash scripts/check-bench-policy.sh`, `bash scripts/check-realtime-audit-leak.sh`
   - the qualification `sdk` job's "Qualify the SDK package against the shared artifact" step, against an artifact built from this tree by `scripts/build-web-audioworklet.sh`
6. **Schema artifact** (json_contract_artifacts.rs).
   - The u64-pattern test moves its vehicle from `revision` to source `frames`.
   - A revision test, computed from `MAX_SESSION_REVISION`: the JSON Schema accepts `"9223372036854775807"` and refuses `"9223372036854775808"` and `"18446744073709551615"`.
   - Test value: a normative schema that disagrees with the engine's validator.
7. **SDK.**
   - builder-evals: revision `9223372036854775807n` builds, and `9223372036854775808n` is refused with `numeric.out_of_schema_range`.
   - enginectl-cli:185-192: the maximum revision becomes `"9223372036854775807"`, and `"9223372036854775808"` joins the malformed list at :367 with exit 3.
   - Test value: an SDK that builds documents the embedded engine refuses.

## Test value

- Gate 1: a store still bounded at `u64::MAX`, or a D1 that relies on the schema and so reports
  `Validation` instead of REVISION_EXHAUSTED.
- Gate 2: a session entry point, or a store constructor, that skips the validator, so an
  above-ceiling session is accepted and its first publication corrupts its gate's announcement bit.
- Gate 3: a host path that bypasses the store's bound, or a refused edit that still pushes a
  record.
- Gate 4: a ceiling check placed after `plan_exchange_inner`, which allocates the exchange before
  refusing.
- Gate 6: a normative schema that disagrees with the engine's validator.
- Gate 7: an SDK that builds documents the embedded engine refuses.

## Amendment 1 (root, 2026-10-11): the ceiling is the session schema's domain

The first implementation run stopped before any edit (it is not an attempt): D2 as written needed `protocol` to build a
`session::DiagnosticSet`, whose constructors are crate-private, and `crates/session/` was not
authorized. Root ruled (under the owner's delegation of 2026-10-11, after a fresh adversarial
review, ACCEPT WITH CHANGES; the review's replacement text is adopted verbatim): the ceiling is part
of the session schema's domain, checked once in `session::validate_session` with one
`NumericOutOfSchemaRange` at `$.revision`; D1 replaces `prepare_transaction`'s `checked_add` in
place so `RevisionExhausted` wins; D3 returns `PlanExchangeError`; gate 4 is measured for real with
`bench_support::alloc` in an engine integration test. `crates/control-plane/src/compile.rs` leaves
the authorized paths (its existing mapping covers both variants). This amends root ruling 2 of
2026-10-05 only in where creation and restore are bounded.

Scope (AGENTS.md split rule): the slice now reaches `session`, the normative schema docs, the SDK
builder and `session-validator`. Root keeps it one issue: it is one product outcome (the revision
domain), and a split would leave the SDK building documents the engine refuses, which turns the
required `sdk` check red.

## Root Amendment 2 (2026-10-11): gate 4 moves to bench-support; the loom leg joins gate 5

`5df1ce8b9` made `bench-support` a dev-dependency of `engine`. bench-support depends on `engine`. Under `--cfg loom`, cargo then builds `engine` as a plain library for bench-support, and that build cannot see `loom`, which is a dev-dependency. The required loom leg fails with "loom unresolved". Gate 4's test moves unchanged to bench-support's own tests, and `engine` drops the dev-dependency. That removes the cycle. A `cfg(loom)` dependency is refused: it would put loom in engine's non-dev graph, against `docs/REALTIME_DEPENDENCY_POLICY.md`, and it would keep the cycle.

Authorized paths: replace the line `crates/engine/src/realtime/{plan_exchange.rs,mod.rs}`, `crates/engine/Cargo.toml` (dev-dependency `bench-support` only), `crates/engine/tests/plan_exchange_ceiling.rs`, `Cargo.lock` with:
- `crates/engine/src/realtime/{plan_exchange.rs,mod.rs}`
- `crates/engine/Cargo.toml` (removal of the `bench-support` dev-dependency only), `Cargo.lock` (the `engine` package's `bench-support` line only)
- `tools/bench-support/tests/plan_exchange_ceiling.rs` (moved from `crates/engine/tests/plan_exchange_ceiling.rs`, which is deleted)

Gate 4, first line: "**Exchange** (a bench-support integration test in its own binary, `tools/bench-support/tests/plan_exchange_ceiling.rs`, using `bench_support::alloc`'s thread-scoped counters; it runs in the required `test-debug-a` job)." The rest of the gate is unchanged.

Gate 5: add `cargo test --locked -p bench-support --test plan_exchange_ceiling` and the qualification `test-release` loom leg, `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom` (13 tests). The loom leg is part of #1503's gate 5: the slice changes engine's dependency graph, and the leg is the gate that catches a dev-dependency cycle on a crate built under `cfg(loom)`.

## Dependencies

- *Give each plan its own revision gate and take each block's live snapshot from it*
  (#1502): the gate handle `plan_exchange_at_revision` takes.
- *Add the latest-target cell primitive and its loom model* (#1432): `GATE_REVISION_MAX`.

Dependents: #1312 (stream order).

## Attempt record

Commit hashes cited in this record that name pre-rebase commits map to the rebased commits in
#1432's "Commit mapping after the rebase onto 7f8ecb8a1".

### Attempt 1 (implementer, on `codex/d15-stream-b2`)

**What landed (Amendment 1 as written; no spec change needed).**

- `session`: `pub const MAX_SESSION_REVISION = engine::realtime::GATE_REVISION_MAX`; `validate_session` refuses a
  larger revision with one `NumericOutOfSchemaRange` at `$.revision` (the first check after `schema_version`).
- `protocol`: `prepare_transaction` replaces the `checked_add` in place with `current >= GATE_REVISION_MAX ->
  RevisionExhausted` (D1); `SessionStore::new` has no check of its own (its `compile_session` returns the set).
- `engine`: `PlanExchangeError { Resources(SpscError), RevisionAboveCeiling { revision } }`, returned by
  `plan_exchange_at_revision` after the own-gate assert and before `plan_exchange_inner`; re-exported at
  `engine::realtime`. `bench-support` is a dev-dependency of `engine` (`Cargo.lock` gains one line); the gate-4 test is
  its own binary, `tests/plan_exchange_ceiling.rs`. `control-plane/compile.rs` is untouched (its `.map_err(|_| ..)`
  covers both variants).
- `tools/session-validator`: the fold refuses `revision >= MAX_SESSION_REVISION` with "session revision exhausted".
- Schema/docs: `$defs/revision` (the review's pattern) in `session-v1.schema.json`; `SESSION_SCHEMA_V1.md`;
  `CONTROL_PROTOCOL_SEMANTICS.md`; one paragraph in `C_ABI_V1_QUALIFICATION.md`. Schema version stays 1.
- SDK: `sdk/src/core/session.ts` refuses `revision > 9223372036854775807n` with `numeric.out_of_schema_range`
  (`CODE.outOfRange`); `enginectl` needed no source change (its `MisoUsageError` path already exits 3
  `request.shape` with the code).
- Tests: see the gate table. Repointed full-u64 revision vehicles to source `frames`
  (`json_grammar.rs` two tests, `json_contract_artifacts.rs` u64 test; the `invalid_matrix.rs` row was deleted, because
  `revision` is no longer a full-domain leaf, and the ceiling is pinned in `diagnostic_parity.rs`). Gate 1's old
  `u64::MAX` test was replaced in place.

**Gates.**

| Gate | Test | Result |
|---|---|---|
| 1 | `protocol` `the_last_commit_reaches_the_ceiling_and_the_next_is_exhausted_before_any_work` | green |
| 2 | `session` `revision_above_the_plan_gate_ceiling_is_refused_by_every_entry_point`; `protocol` `a_session_above_the_ceiling_is_refused_by_the_one_constructor` | green |
| 3 | `capi` `the_c_abi_bounds_revisions_at_the_plan_gate_ceiling` (above-ceiling document refused with `numeric.out_of_schema_range\t$.revision`; edit at `ceiling - 1` commits, watermark = ceiling; next edit `RevisionExhausted`; whole `TestTransactionSnapshot` equal but one replay entry; watermark unchanged; bit-identical to a twin that never saw the edit) | green |
| 4 | `engine` `an_exchange_above_the_gate_ceiling_is_refused_before_it_allocates` (warm call allocates; refusals at `GATE+1` and `u64::MAX` allocate 0, gate unchanged) | green |
| 5 | `cargo test --locked` for `protocol` (test-support), `engine` (realtime-audit), `control-plane` (test-support), `capi`, `session`, `session-validator`; `check-protocol-control-policy`, `check-bench-policy`, `check-realtime-audit-leak`, `check-workspace-policy`, `check-realtime-policy`; `cargo fmt --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | all green |
| 5 (SDK) | the `sdk` job's "Qualify the SDK package against the shared artifact" step (`check-sdk-generated`, `check-sdk-deletions`, `check-sdk-types`, `check-sdk-headless` (363 tests), `sdk-package.sh check` (enginectl 18 tests)) against an artifact from `build-web-audioworklet.sh --named-twin` of this tree | green |
| 6 | `json_contract_artifacts.rs` `session_schema_revision_domain_is_the_validators_ceiling` (also checks the validator accepts the ceiling) and the moved u64 test | green |
| 7 | builder-evals `the revision ceiling is the engine's...` (builder refuses, the embedded engine accepts the ceiling document and refuses ceiling+1); `enginectl-cli` maximum `9223372036854775807`, `...808` in the malformed list (exit 3) | green |

Also run: `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py
--artifacts`, `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh` (private TMPDIR left empty), all green
(the browser-compiled session validator changed); `cargo build --release` + `./target/release/audit capi` (0
allocations, 0 deallocations, 0 syscalls, `total_violations: 0`), `check-capi-abi.sh` and `--self-test`,
`check-cross-targets.sh` (engine code changed) all green. AArch64 not run (CI only).

**Mutation evidence (each applied, red, then reverted, green).**

| Mutation | Red |
|---|---|
| gate 4: ceiling check moved after `plan_exchange_inner` | `plan_exchange_ceiling`: "the refusal allocated" |
| gate 4: `>` -> `>=` | `plan_exchange_ceiling`: "the ceiling itself is accepted" |
| `validate_session` revision check disabled | `diagnostic_parity` (revision test), `protocol` constructor test, `capi` ceiling test |
| D1 back to `checked_add(1)` | `protocol` last-commit test (`Validation` instead of `RevisionExhausted`), `capi` ceiling test (`ValidationFailed` instead of `RevisionExhausted`) |
| fold bound disabled | `fold_mono` `mapped_source_shape_and_map_errors_are_transactional` ("transformed session is invalid" instead of "session revision exhausted") |
| schema `revision` back to `$defs/u64` | `json_contract_artifacts` revision test ("schema admitted revision 9223372036854775808") |
| SDK `REVISION_MAX` set to `u64::MAX` | builder-evals ceiling test and `enginectl-cli` "usage and request refusals" |

**Notes.** `sdk/dist` was deleted after the SDK gates; `sdk/node_modules` (ignored) was installed with `npm ci` and is
not committed.
`crates/capi/src/runtime/live_tests.rs` is not authorized, so the capi test lives in `tests.rs` with its own helpers.

### Batch follow-ups (stream B batch 2, after the five slices passed)

- **The `invalid_matrix.rs` row was deleted, not changed (verdict `1503-attempt1`, n1).** Deliverable
  2 named a row change. The row sat in `u64_model_fields_accept_the_full_unsigned_domain`, whose
  name says the u64 model fields accept the whole domain; `revision` no longer does, so a refusal
  row does not belong in it, and a ceiling-acceptance row would duplicate `diagnostic_parity.rs`,
  which pins the typed-canonical refusal and the ceiling's round trip.
- **`json_grammar.rs` lexical vehicle (n5).** The lexical test's vehicle moved from `revision` to
  `frames`, so its literal `"0"` became `"1"` (frames `0` is semantically invalid). The lexical
  `"0"` stays covered by `canonical-minimal.json`'s revision `"0"`.
- **Docs (n2, n3).** `docs/C_ABI_V1_QUALIFICATION.md` rewrapped the 140-column line the added
  sentence left; `docs/CONTROL_PROTOCOL_SEMANTICS.md:7` now says "owns its revision, and the
  revision lies in `0..=...`" (the subject of "lies in" was the model).
- **builder-evals guard kept (n4).** The new test's `if (asset !== undefined)` guard stays. It is the
  guard the neighbouring route test (`builder-evals.mjs`, the accepted/rejected routes) uses for the
  same reason: `MISO_ENGINE_SDK_SKIP_ASSET=1` runs the builder checks without the engine asset, and
  without the guard this test's engine half would fail there. The verdict's "neighbours use asset
  without a guard" holds only for the tests that do not support that mode.
- **Closed by Amendment 2 (was OPEN), found by the batch gates: the CI loom leg did not build since this slice.**
  `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked
  --release -p engine --lib spsc_loom` fails to compile `engine` (12 errors, `loom` unresolved in
  `latest_cell.rs` and `spsc.rs`) at `5df1ce8b9` and every commit after it; at `70017bf09` (the
  commit before) it passes with 13 tests. Cause: this slice's dev-dependency `bench-support`
  (`crates/engine/Cargo.toml`) depends on `engine`, so under `--cfg loom` cargo builds `engine` as a
  normal library, where `loom` (a dev-dependency) is not linked. With the dev-dependency and
  `tests/plan_exchange_ceiling.rs` removed temporarily, the leg passes (13 tests, also with
  `-C debug-assertions=on`) on the branch tip, so nothing else in the batch breaks it. Not fixed
  here: both fixes change this slice's authorized paths. Either move gate 4's test to a crate that
  already depends on both `engine` and `bench-support` (and drop the dev-dependency), or declare
  `loom` under `[target.'cfg(loom)'.dependencies]` in `crates/engine/Cargo.toml` (the standard
  pattern, subject to the workspace policy on engine's dependencies). The coordinator decides.
