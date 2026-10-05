# Let the control thread withdraw an unadopted candidate plan

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The plan exchange's publication side becomes a two-cell mailbox. The control thread can take
back a published candidate that render has not claimed yet, whole and unrendered, and can publish
it again. Render claims a candidate with one compare-and-swap and never waits on the control
thread; the control thread never waits on render. This is the primitive that supersession
(#1310) and scheduled adoption (#1311) build on.

## Context

- Publication is a bounded SPSC queue (`PlanPublisher::queue`,
  `crates/engine/src/realtime/plan_exchange.rs:64-73`). Render pops the candidate into its own
  `pending` at block entry (`enter_block`, `:375-381`), so once pushed, a candidate can never come
  back to the control thread.
- Reserved publication: `reserve_replacement` (`:265`) takes a publication slot and a retirement
  credit; `PlanReplacementReservation::commit` (`:304`) is infallible. The unreserved `publish`
  (`:235`) and its `legacy_outstanding` counter remain for tests only
  (`crates/engine/src/realtime/mod.rs:309-315`, `:437-470`, `:585`;
  `crates/source/src/lib.rs:3751`); with them comes the render-side deferral
  `SwapOutcome::DeferredRetirementFull` (`:49-56`, `:384-409`).
- The only production caller is capi's structural path (`crates/capi/src/runtime/control.rs:962`;
  in the control plane after #1309).
- The loom step runs `cargo test -p engine --lib spsc_loom` (`qualification.yml:678`); the SPSC's
  own loom model is `crates/engine/src/realtime/spsc.rs:462`.

## Decisions frozen for this slice

- **D1. Two cells, one state word.** The mailbox holds two cells, each storing one candidate
  (plan, epoch, retirement credit) in place, and one `AtomicU64` state word: a generation counter
  and, per cell, a 2-bit state `Empty`, `Full` or `Claimed`. At most one cell is `Full` and at
  most one is `Claimed`. Every transition is one compare-and-swap of the whole word, and every
  successful one increments the generation. `PlanExchangeConfig::publication_capacity` goes: the
  mailbox holds one published candidate by construction. `plan_exchange_resource_report` charges
  the two cells and the word in place of the publication ring, and the control plane's resource
  rows follow from it.
- **D2. Control publishes into an `Empty` cell.** It writes the candidate into a cell whose state is
  `Empty` (the render thread never touches such a cell), then compare-and-swaps that cell to
  `Full`. With at most one `Claimed` cell, control always finds an `Empty` one, so publication
  never waits for render.
- **D3. Render claims.** At block entry render loads the word; if a cell is `Full` it
  compare-and-swaps it to `Claimed`, moves the candidate out (a bounded move), and swaps the cell
  to `Empty`. A failed compare-and-swap means control withdrew or replaced it: render does nothing
  this block. Render never spins. Adoption then proceeds as today (clock, carry, retirement).
- **D4. Control withdraws.** `PlanPublisher::withdraw() -> Withdrawal` compare-and-swaps the
  `Full` cell to `Empty` and moves the candidate out: `Withdrawal::Withdrawn(UnadoptedCandidate)`
  (the plan, its epoch and its retirement credit). If no cell is `Full`, the result is
  `Withdrawal::Taken` (render claimed it, or nothing was published). `UnadoptedCandidate` can be
  published again with `PlanPublisher::republish`, infallibly and with its own credit and epoch,
  or dropped, which returns its credit.
- **D5. Every candidate carries a credit.** The unreserved `publish` becomes reserve-then-commit
  and fails `Full` without a credit. `legacy_outstanding`, `SwapOutcome::DeferredRetirementFull`
  and `deferred_count` are removed; their tests move to the reserved form or are deleted
  (gate 4).
- **D6. Acked-batch question: can an ack ever precede a drop? No.** Nothing in the exchange drops
  a candidate: a claimed candidate is adopted, a withdrawn one is returned whole to the control
  thread, and a dropped `UnadoptedCandidate` is dropped only by the control thread's own choice.

## Deliverables

1. D1-D5 in `plan_exchange.rs`, with doc comments stating the state machine and its invariants.
2. A loom model of the mailbox.

## Authorized paths

- `crates/engine/src/realtime/plan_exchange.rs`, `crates/engine/src/realtime/mod.rs` (exports and
  exchange tests).
- `crates/source/src/lib.rs` (its one test that publishes, `:3751`, only).
- The control plane's publication call (`crates/control-plane/src/control.rs`), only as far as the
  API change requires; withdrawal is used by #1310.

## Non-goals

- Supersession itself (#1310); scheduled or exact adoption and the return queue (#1311).

## Hazards

- The move out of a `Claimed` cell happens after the claim; control must never write a `Claimed`
  cell (D2 picks the other cell).
- `Withdrawal::Taken` does not mean "adopted yet": render may be inside the claim. The control
  plane learns adoption from its epoch synchronisation, as today.

## Objective gates

1. **Unit tests (new, `crates/engine/src/realtime/mod.rs`).** Publish then withdraw before any
   render: `Withdrawn`, its epoch and plan ID intact; render after that applies nothing. Republish
   it: the next block applies it with that epoch. Render one block after publication, then
   withdraw: `Taken`, and the plan is active. Credits balance after each case.
2. **Loom (new `spsc_loom_plan_mailbox_*` tests, so the CI filter runs them).** One render thread
   claiming and one control thread publishing, withdrawing and republishing: exactly one side wins
   each race, a candidate is adopted at most once and never both adopted and withdrawn, render
   never reads a cell control is writing, and credits balance. Command:
   `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test
   --locked --release -p engine --lib spsc_loom`.
3. **Realtime.** `cargo build --locked --release -p audit -p capi && target/release/audit capi`
   reports allocations, deallocations, locks, syscalls and `total_violations` 0;
   `bash scripts/check-realtime-policy.sh`; `bash scripts/test-realtime-policy.sh`.
4. **Superseded tests.** The `DeferredRetirementFull` cases (`mod.rs:309-315`, `:465-470`) are
   deleted with the variant: a candidate without a credit can no longer be published, which the new
   unit test "publish without a credit fails `Full`" covers.
5. **Workspace.** `cargo test --locked -p engine --features realtime-audit`; `cargo test --locked
   -p capi`; `cargo test --locked -p host-core --features control-provider,test-support --test
   successor_swap`; `cargo test --locked -p graph --features test-support --test
   rt11_swap_carry_alloc`; `cargo test --locked -p source`; `bash scripts/check-cross-targets.sh`;
   `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features
   -- -D warnings`.

## Test value

- Gate 1: a withdrawal that loses or corrupts the candidate, or a republish that changes its epoch.
- Gate 2: a claim/withdraw race in which both sides take the candidate or render reads a cell
  being written; only loom's interleavings reach it.
- Gate 4's replacement: a credit-less candidate published, which could defer a swap forever.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309), for the call
  site's path.
