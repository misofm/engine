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

- **Root ruling 2 (2026-10-05).** The ceiling is enforced in the protocol session store, through
  its existing `checked_add` and `RevisionExhausted`. Creating or restoring a session above it is
  refused, and `plan_exchange_at_revision` refuses an initial revision above it.
  - The review recommended the store over a control-plane pre-check, which a future commit path
    could bypass.
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

- **D1. The transaction bound.** `apply_transaction` refuses with `RevisionExhausted` when
  `current + 1` would exceed `GATE_REVISION_MAX`. The refusal comes before any edit is applied,
  as today, so nothing is written (D15-2 condition 3).
- **D2. Creation and restore.** `SessionStore::new` refuses an initial model whose revision is
  above `GATE_REVISION_MAX`. It returns a `DiagnosticSet` with one `NumericOutOfSchemaRange`
  diagnostic at `/revision`, the code the session schema uses for a number outside its domain. A
  model at exactly `GATE_REVISION_MAX` is accepted, and its first transaction is refused by D1.
- **D3. The exchange.** `plan_exchange_at_revision` refuses an initial revision above
  `GATE_REVISION_MAX` with a typed error, before it allocates or publishes anything. The control
  plane maps that error as it maps the exchange's other preparation failures. D2 makes it
  unreachable from the control plane.
- **D4. The acked-batch question: can an ack ever precede a drop? No.** Every refusal comes before
  the first write of its transaction or session, and nothing is acknowledged.

## Deliverables

1. D1-D2 in `crates/protocol/src/model.rs`, with its test moved to the new boundary.
2. D3 in `crates/engine/src/realtime/plan_exchange.rs` and its mapping in
   `crates/control-plane/src/compile.rs`.
3. One C ABI test, and one sentence in `docs/C_ABI_V1_QUALIFICATION.md`: "a session revision is
   at most 2^63 - 1".

## Authorized paths

- `crates/protocol/src/model.rs` (the store bound, the constructor check and their tests).
- `crates/engine/src/realtime/plan_exchange.rs` (D3 only), `crates/engine/src/realtime/mod.rs`
  (its test).
- `crates/control-plane/src/compile.rs` (D3's mapping only).
- `crates/capi/src/runtime/tests.rs` or `crates/capi/tests/` (gate 3), `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- The gate itself (#1432) and its place in each plan (#1502).

## Objective gates

1. **Store bound (protocol unit test, replacing the `u64::MAX` case at `model.rs:1395-1425`).**
   - A store at `GATE_REVISION_MAX - 1` commits one transaction (revision `GATE_REVISION_MAX`).
   - The next transaction is refused with `RevisionExhausted`, and the canonical snapshot is
     unchanged.
2. **Creation (same file).**
   - `SessionStore::new` with a model at `GATE_REVISION_MAX + 1` is refused with
     `NumericOutOfSchemaRange` at `/revision`.
   - A model at `u64::MAX` is refused the same way.
   - A model at `GATE_REVISION_MAX` is accepted.
3. **C ABI (new capi test).**
   - A session created from a document whose revision is `GATE_REVISION_MAX + 1` is refused, as
     the C ABI refuses any document the compiler refuses.
   - A session at `GATE_REVISION_MAX - 1` takes one live fader edit: the watermark then reads
     `GATE_REVISION_MAX`.
   - The next edit is refused with the revision-exhausted status. No record is pushed, the output
     equals a twin without that edit, and the watermark is unchanged.
4. **Exchange (engine unit test).** `plan_exchange_at_revision` with `GATE_REVISION_MAX + 1` is
   refused, allocates no exchange and leaves the plan's gate at its prior revision.
5. **Workspace.** `cargo test --locked -p protocol --features test-support`;
   `cargo test --locked -p engine --features realtime-audit`;
   `cargo test --locked -p control-plane --features test-support`; `cargo test --locked -p capi`;
   `bash scripts/check-protocol-control-policy.sh`; `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.

## Test value

- Gate 1: a store still bounded at `u64::MAX`, which lets a revision with bit 63 reach a gate and
  read as an announcement.
- Gate 2: a restored session above the ceiling that is accepted, so its first publication corrupts
  its gate.
- Gate 3: a host path that bypasses the store's bound, or a refused edit that still pushes a
  record.
- Gate 4: an exchange started above the ceiling.

## Dependencies

- *Give each plan its own revision gate and take each block's live snapshot from it*
  (#1502): the gate handle `plan_exchange_at_revision` takes.
- *Add the latest-target cell primitive and its loom model* (#1432): `GATE_REVISION_MAX`.

Dependents: #1312 (stream order).
