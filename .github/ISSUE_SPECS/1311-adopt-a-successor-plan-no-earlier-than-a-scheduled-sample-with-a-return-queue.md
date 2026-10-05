# Adopt a successor plan no earlier than a scheduled sample, with a return queue

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8, D15-9).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The plan exchange can hold a published candidate until a chosen render sample. A candidate
published "no earlier than S" is adopted at the first block that starts at or after S; one
published "exactly at S" is adopted at the block that starts at S, and if render first sees it
later, render hands it back to the control thread through a capacity-1 return queue instead of
adopting it late. Until it is due, the candidate stays withdrawable, so a later structural edit can
still supersede it. Duck-swap and two-phase removal (stream D) schedule swaps with this, and the
warm successor (stream C) uses exact adoption and the return queue.

## Context

- Today the render thread adopts a reserved candidate at the first block after publication:
  `enter_block` pops it into `pending` and swaps unless retirement is full
  (`crates/engine/src/realtime/plan_exchange.rs:375-437`; applied at `:436`). There is no notion
  of a due sample, and no path from render back to control other than retirement
  (`PlanRetirer::try_reclaim`, `:512`).
- The incoming plan adopts the outgoing plan's clock at the swap (`:415-418`), so "the block that
  starts at S" means `absolute_sample == S` on the host's render timeline, in both
  `render_contiguous` (`:448`) and `render` (`:473`).
- `RealtimePlanOwner` already keeps saturating counters for deferred and carried swaps
  (`deferred_count` `:361`, `carried_count` `:366`).
- The mailbox of #1343 replaces the publication queue with two cells and
  one atomic state word: the control thread fills an empty cell and marks it full; the render
  thread claims a full cell by compare-and-swap; the control thread withdraws a full cell by
  compare-and-swap. The loser of a race never touches the cell.

## Decisions frozen for this slice

- **D1. Adoption mode on the reservation.** `PlanAdoption::{Next, NoEarlierThan(u64),
  ExactlyAt(u64)}` is fixed when the control thread reserves a replacement and stored in the cell
  beside the plan. `Next` is today's behaviour; every existing caller passes it.
- **D2. Render reads the due sample before it claims.** The cell's due sample lives in an
  `AtomicU64` written before the cell is marked full. Render loads the state word, then the due
  sample, and claims (compare-and-swap on the whole word, generation included) only when the
  block's start is at or past the due sample. A word that changed between the two loads makes the
  claim fail; render then does nothing this block and looks again next block. Render never spins
  and never claims a candidate that is not due, so a scheduled candidate stays withdrawable until
  its block.
- **D3. Exactly at S, else return.** For `ExactlyAt(S)`, a claim at a block starting at `S`
  adopts. A claim at a later block does not adopt: render pushes the candidate, with its retirement
  credit, into the return queue (an SPSC of capacity 1, preallocated at exchange creation, render
  to control). `RealtimePlanOwner::returned_count` counts these, saturating.
- **D4. The return queue never drops.** At most one candidate is in flight, and the control thread
  drains the return queue before every reservation, so render's push finds room. If it does not
  (a broken invariant), render keeps the candidate in a one-entry render-local slot and retries the
  push at the next block; it never adopts it and never frees it.
- **D5. Off-grid schedules.** `ExactlyAt(S)` with `S` not a multiple of the quantum relative to the
  plan's clock origin is refused at reservation with a typed error. `NoEarlierThan(S)` accepts any
  `S` and adopts at the first block whose start is at or past it.
- **D6. Control-side API.** `PlanPublisher::reserve_replacement(plan, adoption)`, and
  `PlanReturns::try_reclaim() -> Option<(PlanEpoch, PreparedRenderPlan)>` beside
  `PlanRetirer`. The control plane threads `PlanAdoption::Next` through its publication and drains
  the return queue in `synchronize_plan_epochs`. A `Next` candidate is never returned, so a
  returned candidate there is an internal fault: it is dropped on the control thread and the call
  reports `CommandError::Internal`. The callers that schedule own their returned candidates.
- **D7. Acked-batch question: can an ack ever precede a drop? No.** Scheduling changes when a
  candidate is adopted, never whether its content survives: an unclaimed candidate stays in its
  cell (withdrawable, D2), a claimed one is adopted or returned whole (D3), and a returned one is
  held until the control thread takes it (D4). Nothing render does discards a candidate.

## Deliverables

1. D1-D5 in `plan_exchange.rs`, with doc comments that state the claim and return rules.
2. D6 in the control plane.
3. A loom model of claim, withdraw and return.

## Authorized paths

- `crates/engine/src/realtime/plan_exchange.rs`, `crates/engine/src/realtime/mod.rs` (exports and
  its exchange tests).
- `crates/control-plane/src/` (D6 call sites only).
- `.github/workflows/qualification.yml`: the loom step's test filter (gate 2) only; outside
  stream B's file set, sequenced by the coordinator.

## Non-goals

- Choosing S for any edit (duck-swap #1324, two-phase removal #1325, warm successor #1287).
- The copy-at-B mode of D15-8 step 2 (render copies its state into a candidate and returns it):
  *Pre-roll a successor whose latency grows* (#1287) adds it on this return queue, after *Carry
  plan state by copy as well as by move* (#1322).
- Supersession (#1310).

## Hazards

- The due-sample load and the state-word load are two loads: D2's generation in the word is what
  makes a stale due sample harmless. A design that reads the due sample after claiming would let
  render hold an undue candidate and make it unwithdrawable.
- `render` (`:473`) takes an explicit time; compare against `time.absolute_sample`, not the plan's
  own clock.

## Objective gates

1. **Engine unit tests (new, `crates/engine/src/realtime/mod.rs`):**
   - `NoEarlierThan(S)`: blocks before S render the predecessor (`SwapOutcome::None`, its plan ID);
     the first block starting at or after S applies the swap.
   - `ExactlyAt(S)`: published on time, it applies at S; published after render passed S, it is
     returned (`returned_count` 1, `try_reclaim` yields it, the predecessor keeps rendering).
   - A `NoEarlierThan` candidate withdrawn before S is never adopted.
   - `ExactlyAt` off the grid is refused at reservation.
   - Retirement credits balance after each case (reserve, return, reclaim).
2. **Loom (new `spsc_loom_plan_mailbox_*` tests, or a filter change to the loom step).** Render
   claims while control withdraws and republishes: exactly one side wins each race, a due
   candidate is adopted at most once, an undue one is never claimed, and a returned one reaches
   control. Command: `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom
   --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom` (the CI step,
   `qualification.yml:678`).
3. **Render stays allocation-free and syscall-free.** `cargo build --locked --release -p audit
   -p capi && target/release/audit capi`: allocations, deallocations, locks, syscalls and
   `total_violations` 0. `bash scripts/check-realtime-policy.sh` and
   `bash scripts/test-realtime-policy.sh`.
4. **No behaviour change for `Next`.** `cargo test --locked -p capi`,
   `cargo test --locked -p host-core --features control-provider,test-support --test
   successor_swap`, `cargo test --locked -p graph --features test-support --test
   rt11_swap_carry_alloc`, `cargo test --locked -p engine --features realtime-audit`.
5. **Workspace.** `cargo fmt --all -- --check`; `cargo clippy --locked --workspace
   --all-targets --all-features -- -D warnings`; `bash scripts/check-cross-targets.sh`.

## Test value

- Gate 1, `NoEarlierThan`: a swap applied at the first block regardless of S.
- Gate 1, `ExactlyAt` late: a candidate adopted late (breaking the warm successor's exact catch-up)
  or dropped instead of returned.
- Gate 1, withdrawn before S: render claiming an undue candidate, which would make a scheduled
  candidate impossible to supersede.
- Gate 2: an ordering bug between claim and withdraw that only some interleavings expose.

## Dependencies

- *Let the control thread withdraw an unadopted candidate plan* (#1343). This slice follows it in
  `plan_exchange.rs`, and both precede stream C's edits there.
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309), for D6.
