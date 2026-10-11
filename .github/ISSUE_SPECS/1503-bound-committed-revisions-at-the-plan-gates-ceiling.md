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
- `crates/engine/src/realtime/{plan_exchange.rs,mod.rs}`, `crates/engine/Cargo.toml` (dev-dependency `bench-support` only), `crates/engine/tests/plan_exchange_ceiling.rs`, `Cargo.lock`
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
4. **Exchange** (an engine integration test in its own binary, using `bench_support::alloc`).
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

## Dependencies

- *Give each plan its own revision gate and take each block's live snapshot from it*
  (#1502): the gate handle `plan_exchange_at_revision` takes.
- *Add the latest-target cell primitive and its loom model* (#1432): `GATE_REVISION_MAX`.

Dependents: #1312 (stream order).
