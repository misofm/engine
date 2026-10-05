# Let the control thread withdraw an unadopted candidate plan

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The plan exchange's publication side becomes a two-cell mailbox. The control thread can take
back a published candidate that render has not claimed yet, whole and unrendered, and can publish
it again. Render claims a candidate with one compare-and-swap and never waits on the control
thread or retries; the control thread never waits on render. This is the primitive that the
watermark (#1314), supersession (#1310) and scheduled adoption (#1311) build on.

## Context

- Publication is a bounded SPSC queue (`PlanPublisher`, its `queue` field,
  `crates/engine/src/realtime/plan_exchange.rs:67-73`). Render pops the candidate into its own
  `pending` at block entry (`enter_block`, `:375-381`), so once pushed, a candidate can never come
  back to the control thread.
- Reserved publication: `reserve_replacement` (`:265`) takes a publication slot and a retirement
  credit; `PlanReplacementReservation::commit` (`:304`) is infallible. The unreserved `publish`
  (`:235`) and its `legacy_outstanding` counter remain for tests only
  (`crates/engine/src/realtime/mod.rs:310-316`, `:437-471`, `:585`;
  `crates/source/src/lib.rs:3751`); with them comes the render-side deferral
  `SwapOutcome::DeferredRetirementFull` (`plan_exchange.rs:49-56`, `:384-409`).
- The only production caller is capi's structural path (`crates/capi/src/runtime/control.rs:962`;
  in the control plane after #1309).
- `scripts/check-realtime-policy.sh:29` admits `unsafe` in the realtime root only in
  `crates/engine/src/realtime/spsc.rs` and `disjoint.rs`. `spsc.rs` already owns the
  `UnsafeCell` shim that swaps in loom's types under `--cfg loom` (`:14-45`; loom's atomics import
  is `AtomicUsize` only, `:40`) and the SPSC's loom model (`:461-462`).
- The loom step runs `cargo test -p engine --lib spsc_loom` (`.github/workflows/qualification.yml:678`).

## Decisions frozen for this slice

- **D1. Where the unsafe lives.** The mailbox is a generic type in `spsc.rs`, the file the realtime
  policy already approves for `unsafe`: `plan_mailbox::<T>() -> (MailboxWriter<T>,
  MailboxReader<T>)`. Its two cells are `UnsafeCell<Option<T>>` through the existing loom shim,
  which gains `AtomicU64`. It exposes only safe methods. `plan_exchange.rs` uses it and stays free
  of `unsafe`; the policy script does not change.
- **D2. Two cells, one state word.** One `AtomicU64` holds a generation counter and, per cell, a
  2-bit state: `Empty`, `Full` or `Active`. Exactly one cell is `Active`: it stands for the plan
  render runs, whose payload render has already moved out. The other cell is `Empty` or `Full`.
  Cell 0 starts `Active` for the initial plan. Every successful transition increments the
  generation. `PlanExchangeConfig::publication_capacity` goes: the mailbox holds one published
  candidate by construction. `plan_exchange_resource_report` charges the two cells and the word in
  place of the publication ring. The control plane's resource rows follow from it.
- **D3. Control publishes into the `Empty` cell.** It writes the candidate (plan, epoch,
  retirement credit) into the cell whose state is `Empty`. Then it compare-and-swaps the word to
  mark that cell `Full` (`Release`). Render changes the word only when a cell is `Full`, and
  control publishes only when none is, so this compare-and-swap cannot fail. A failure would be a
  broken invariant: it is returned as a typed internal error and never retried.
- **D4. Render claims in one compare-and-swap, with no separate release.** At block entry render
  loads the word (`Acquire`). If a cell is `Full`, one compare-and-swap of the whole word marks
  that cell `Active` and the previously `Active` cell `Empty`. That single transition is the
  adoption decision. Render then moves the candidate out of its cell (a bounded move) and adopts it
  as today (clock, carry, retirement). The compare-and-swap fails only when control withdrew the
  candidate after render's load. Render then does nothing this block and looks again next block.
  Render never retries within a block and never spins. It has no second compare-and-swap that
  could fail. Control never touches an `Active` cell's payload.
- **D5. Control withdraws.** `PlanPublisher::withdraw() -> Withdrawal`:
  - `Withdrawal::Withdrawn(UnadoptedCandidate)`: the cell was `Full`. One compare-and-swap marks it
    `Empty` and control moves the candidate out (plan, epoch, retirement credit).
  - `Withdrawal::Taken`: render's claim won. The candidate is adopted, and that cannot change.
  - `Withdrawal::Nothing`: no candidate was published since the last claim or withdrawal.

  If control's compare-and-swap fails, render changed the word. Render does that at most once per
  published candidate, so control reloads once and reports from that load. `UnadoptedCandidate`
  can be published again with `PlanPublisher::republish`, infallibly, keeping its own credit and
  epoch. It can also be dropped, which returns its credit.
- **D6. Every candidate carries a credit.** The unreserved `publish` becomes reserve-then-commit
  and fails `Full` without a credit. `legacy_outstanding`, `SwapOutcome::DeferredRetirementFull`
  and `deferred_count` are removed: a claimed candidate always finds retirement room for the plan
  it displaces. Their tests move to the reserved form or are deleted (gate 4).
- **D8. The deferral audits (Amendment 1).** The `realtime`, `graph` and `builtins_graph` audits
  and their trace scripts prove the render-side deferral path that D6 removes. Each audit replaces
  its deferral round with two rounds, with exact asserted counts:
  (i) **refused reservation**: while the retirement queue is full, every reservation is refused
  `Full` on the control side; render keeps rendering the active plan with zero allocations,
  deallocations, locks and syscalls. `swaps_deferred` becomes `reservations_refused`, and
  `prior_plan_renders_on_deferred` becomes `prior_plan_renders_while_refused` (the builtins-graph
  audit keeps its long range under the new names, each count exact);
  (ii) **withdraw and republish**: a published candidate is withdrawn before any render
  (`Withdrawn`), published again, and adopted at the next block with its epoch intact
  (`withdrawals` and `republished_adoptions` exact).
  Each trace script checks the new counts exactly in place of the deferred counts.
- **D7. Acked-batch question: can an ack ever precede a drop? No.** Nothing in the exchange drops
  a candidate. A claimed candidate is adopted. A withdrawn one is returned whole to the control
  thread. A dropped `UnadoptedCandidate` is dropped only by the control thread's own choice.

## Deliverables

1. D1-D4 in `spsc.rs`, with doc comments that state the state machine and its invariants; D2-D6 in
   `plan_exchange.rs`.
2. A loom model of the mailbox.

## Authorized paths

- `crates/engine/src/realtime/spsc.rs` (the mailbox type, the loom shim's `AtomicU64`, its loom
  tests), `crates/engine/src/realtime/plan_exchange.rs`, `crates/engine/src/realtime/mod.rs`
  (exports and exchange tests).
- `crates/source/src/lib.rs` (its one test that publishes, `:3751`, only).
- The control plane's publication call (`crates/control-plane/src/control.rs`), only as far as the
  API change requires. Withdrawal is used by #1310.
- Amendment 1, mechanical edits only (the removed `publication_capacity` field and the move to
  reserve-then-commit): `crates/control-plane/src/compile.rs`,
  `crates/host-core/tests/successor_swap.rs`, `crates/host-core/tests/support/successor.rs`,
  `crates/graph/tests/rt11_swap_carry_alloc.rs`, and `crates/source/src/lib.rs`'s
  `exchange_config` test helper.
- Amendment 1, the deferral audits (D8): `tools/audit/src/realtime.rs`, `tools/audit/src/graph.rs`,
  `tools/audit/src/builtins_graph.rs`, `scripts/trace-realtime-audit.sh`,
  `scripts/trace-graph-audit.sh`, `scripts/trace-builtins-graph-audit.sh`.

## Non-goals

- The revision word per cell (#1314). Supersession itself (#1310). Scheduled and primed adoption
  (#1311).

## Hazards

- The move out of a cell happens after the claim. It is safe only because control never writes an
  `Active` cell and publishes only into the `Empty` one (D3).
- `Withdrawal::Taken` is final, but render may still be moving the plan out. The control plane
  promotes providers from its epoch synchronisation, as today.

## Objective gates

1. **Unit tests (new, `crates/engine/src/realtime/mod.rs`).**
   - Publish, then withdraw before any render: `Withdrawn`, its epoch and plan ID intact. Render
     after that applies nothing. Republish it: the next block applies it with that epoch.
   - Render one block after publication, then withdraw: `Taken`, and the plan is active.
   - Withdraw with nothing published: `Nothing`.
   - Publish, claim, publish a second candidate: it lands in the cell the first claim left `Empty`.
   - Publish without a credit fails `Full`.
   - Credits balance after each case.
2. **Loom (new `spsc_loom_plan_mailbox_*` tests in `spsc.rs`, so the CI filter runs them).** One
   render thread claiming and one control thread publishing, withdrawing and republishing: exactly
   one side wins each race; a candidate is adopted at most once and never both adopted and
   withdrawn; render never reads a cell control is writing; render makes at most one
   compare-and-swap per block; credits balance. Command:
   `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test
   --locked --release -p engine --lib spsc_loom`.
3. **Realtime.** `cargo build --locked --release -p audit -p capi && target/release/audit capi`
   reports allocations, deallocations, locks, syscalls and `total_violations` 0.
   `bash scripts/trace-realtime-audit.sh`, `bash scripts/trace-graph-audit.sh` and
   `bash scripts/trace-builtins-graph-audit.sh` pass with D8's counts (Amendment 1).
   `bash scripts/check-realtime-policy.sh` passes with the script unchanged (no `unsafe` outside
   `spsc.rs`); `bash scripts/test-realtime-policy.sh`.
4. **Superseded tests.** The `DeferredRetirementFull` cases (`mod.rs:309-315`, `:465-470`) are
   deleted with the variant. Gate 1's "publish without a credit fails `Full`" covers the reason
   they existed.
5. **Workspace.** `cargo test --locked -p engine --features realtime-audit`; `cargo test --locked
   -p capi`; `cargo test --locked -p host-core --features control-provider,test-support --test
   successor_swap`; `cargo test --locked -p graph --features test-support --test
   rt11_swap_carry_alloc`; `cargo test --locked -p source`; `bash scripts/check-cross-targets.sh`;
   `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features
   -- -D warnings`.

## Test value

- Gate 1: a withdrawal that loses or corrupts the candidate, a republish that changes its epoch,
  or a publication into the `Active` cell.
- Gate 2: a claim/withdraw race in which both sides take the candidate, render reads a cell being
  written, or render loops on a failed compare-and-swap. Only loom's interleavings reach it.
- D8's audit rounds: a control-side refusal that disturbs the running plan (an allocation, lock or
  syscall on render, or a render that stops rendering the active plan), and a withdraw-republish
  cycle that loses the candidate or its epoch in the release build the audits measure.
- Gate 4's replacement: a credit-less candidate published, which could defer a swap forever.

## Amendment 1 (root, 2026-10-05)

D2 and D6 remove `publication_capacity`, the unreserved `publish`,
`SwapOutcome::DeferredRetirementFull` and `deferred_count`. That breaks files the original
authorized paths did not list: mechanical call sites in `crates/control-plane/src/compile.rs`,
two host-core test files, one graph test and `crates/source/src/lib.rs`'s `exchange_config`; and
three audit binaries with their trace scripts, which exist to prove the deferral path. Root ruled:
authorize the mechanical files for mechanical edits only, and the three audits and trace scripts
with the frozen proof of D8; add the three trace runs to gate 3. One issue, no sibling split: the
tree compiles only with both, so a same-commit sibling would be ceremony. This takes the slice
past AGENTS.md's half-day size (about half a day plus two to three hours for the audits); root
accepted the overrun for that reason. The first run stopped before any change and is not an
attempt.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309), for the call
  site's path.
