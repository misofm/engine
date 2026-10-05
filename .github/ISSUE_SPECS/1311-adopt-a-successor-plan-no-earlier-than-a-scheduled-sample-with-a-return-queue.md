# Adopt a successor plan no earlier than a scheduled sample, with a return queue

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8, D15-9).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The plan exchange can hold a published candidate until a chosen render sample. A candidate
published "no earlier than S" is adopted at the first block that starts at or after S. One
published "exactly at S" is adopted at the block that starts at S. If render first sees it later,
render hands it back to the control thread instead of adopting it late: the candidate stays in its
mailbox cell, marked `Returned` with a reason, until its owner takes it. The title's "return queue"
is this in-place return and nothing more: the cell holds at most one returned candidate, no
separate queue exists, and nothing drains returns in the background (D6). Until it is due, a candidate stays withdrawable, so a later structural edit
can still supersede it. Duck-swap and two-phase removal (stream D) schedule swaps with this, and
the warm successor (stream C) uses exact adoption and the return path.

## Context

- Today the render thread adopts a reserved candidate at the first block after publication
  (`enter_block`, `crates/engine/src/realtime/plan_exchange.rs:375-437`; applied at `:436`). There
  is no notion of a due sample, and no path from render back to control other than retirement
  (`PlanRetirer::try_reclaim`, `:512`).
- The incoming plan adopts the outgoing plan's clock at the swap (`:415-418`), so "the block that
  starts at S" means `absolute_sample == S` on the host's render timeline, in both
  `render_contiguous` (`:448`) and `render` (`:473`).
- `RealtimePlanOwner` keeps a saturating `carried_count` (`:366`).
- *Let the control thread withdraw an unadopted candidate plan* (#1343) replaces publication with a
  two-cell mailbox in `crates/engine/src/realtime/spsc.rs`: one `AtomicU64` state word holds a
  generation and a 2-bit state per cell (`Empty`, `Full`, `Active`). Render claims a `Full` cell by
  one compare-and-swap that makes it `Active` and the old `Active` cell `Empty`. Control withdraws a
  `Full` cell by compare-and-swap: `Withdrawal::{Withdrawn, Taken, Nothing}`. It also removes
  `deferred_count` and `DeferredRetirementFull`.

## Decisions frozen for this slice

- **D1. Adoption mode on the reservation.** `PlanAdoption::{Next, NoEarlierThan(u64),
  ExactlyAt(u64)}` is fixed when the control thread reserves a replacement and stored in the cell
  beside the plan, with the due sample. Both are written before the cell is marked `Full`. `Next`
  is today's behaviour; every existing caller passes it.
  - The mode and the due sample are two atomics of the cell (`AtomicU8` and `AtomicU64`), outside
    the `UnsafeCell` that holds the plan payload. Control stores them (`Relaxed`) before the
    state word's `Release` that marks the cell `Full`. Render loads them before it claims (D2),
    while control may withdraw and republish into the same cell, so they are never fields of the
    payload: a plain read of the payload before the claim would race with that republish.
- **D2. Render reads the due sample before it claims.** Render loads the state word (`Acquire`),
  then the `Full` cell's mode and due sample from their atomics (D1; `Relaxed` suffices, because
  the claim below re-checks the generation). It touches the payload only after the claim
  succeeds. It claims (#1343's one compare-and-swap, generation
  included) only when the block's start is at or past the due sample. If control withdrew and
  republished between the two loads, the generation changed and the claim fails. Render then does
  nothing this block and looks again next block. Render never spins and never claims a candidate
  that is not due, so a scheduled candidate stays withdrawable until its block.
- **D3. Exactly at S, else returned in place.** The cell state gains a fourth value, `Returned`,
  which the 2-bit field already has room for. The state word also gains a `ReturnReason` field for
  the one cell that can be `Returned`.
  - For `ExactlyAt(S)`, a claim at a block starting at `S` adopts.
  - At a later block, render does not adopt. Its one compare-and-swap marks the cell `Returned`
    with `ReturnReason::Late` instead of `Active`. The plan, its retirement credit, its revision
    word (#1314) and its epoch stay in the cell. Render never touches a `Returned` cell again.
  - The compare-and-swap fails only if control withdrew the candidate first.
  - `RealtimePlanOwner::returned_count` counts returns, saturating.
  - The reason exists so that every way of returning a candidate is told apart from a late one.
    This issue defines the whole `ReturnReason` enum, so later slices add no reason bits:
    - `Late`: an `ExactlyAt` candidate render saw after its sample (this issue).
    - `Copying`: render claimed a copy-and-return candidate and is copying into it now (written by
      *Snapshot a running plan into a returned successor at a block*, #1354 D1).
    - `Copied`: that copy finished (#1354 D1).
    - `CopyRefused`: that copy was refused, the candidate untouched (#1354 D1), so a refused copy
      can never be taken for a copied one.
    - `PreRollBound`: render's bounded pre-roll could not finish (*Fall back from a missed catch-up
      deadline: bounded render-thread pre-roll, then the transition*, #1358 D4).
  - The reason field is 3 bits, enough for the five values.
  - Render changes a `Returned` word in exactly one case: it stores `Copied` or `CopyRefused` over
    its own `Copying`. Control never changes a `Copying` cell (D6).
- **D4. The return path never drops, and needs no push.** A returned candidate stays in its cell,
  so nothing render does on return can fail or need room. While a cell is `Returned`, the other
  cell is `Active` and nothing can be published, so a second return is impossible. The return path
  costs no bytes beyond #1343's two cells: the reason is three bits of the state word. The returned
  plan's memory is the candidate's, already counted as the pending plan.
- **D5. Off-grid schedules.** `ExactlyAt(S)` with `S` not a multiple of the quantum relative to the
  plan's clock origin is refused at reservation with a typed error. `NoEarlierThan(S)` accepts any
  `S` and adopts at the first block whose start is at or past it.
- **D6. Control-side API.** `PlanPublisher::reserve_replacement(plan, adoption)`.
  `PlanPublisher::withdraw()` gains two outcomes:
  - `Withdrawal::Returned { candidate: UnadoptedCandidate, reason: ReturnReason }`, for a
    `Returned` cell whose reason is `Late`, `Copied`, `CopyRefused` or `PreRollBound`. It is one
    compare-and-swap that marks the cell `Empty` and cannot fail, because render does not change
    the word while a cell is `Returned` with one of these reasons (D3).
  - `Withdrawal::InFlight`, for a `Returned` cell whose reason is `Copying`. The word is unchanged
    and the candidate stays in its cell. The caller never waits on it: it acts on its next call
    (#1354 D1; the structural path's handling is *Supersede a running catch-up by a structural
    edit*, #1357 D1). This issue produces no `Copying` cell, so here `InFlight` appears only in a
    unit test that sets the word directly.
  - `Withdrawal::Taken` keeps meaning adopted, and only that. A returned candidate is never
    reported as `Taken`.
  - Nothing drains returns in the background. `synchronize_plan_epochs` and the service step of
    #1348 never take a `Returned` candidate. Only the code that published the candidate takes it,
    with `withdraw`, as the catch-up does (#1355, #1360). The structural path treats `Returned` like
    `Withdrawn`: the plan render runs is still the predecessor (#1310 D1).
  - The control plane threads `PlanAdoption::Next` through its publication. A `Next` candidate is
    never returned, by D3's construction.
  - **The revision of a returned candidate.** #1314 D2 sends every revision committed while a
    candidate is pending to that candidate. This issue extends `PlanPublisher::set_revision` to a
    `Returned` cell: it stores into that cell's revision word (an atomic, so a store during
    `Copying` is safe) and reports `RevisionTarget::Pending`. Render never claims a `Returned`
    cell, so the revision waits for the candidate's republication and adoption; it is never
    written into the `Active` cell while a candidate is returned.
- **D7. Acked-batch question: can an ack ever precede a drop? No.** Scheduling changes when a
  candidate is adopted, never whether its content survives. An unclaimed candidate stays in its
  cell, withdrawable (D2). A claimed one is adopted. A late one stays in its cell, `Returned`, until
  the control thread takes it whole (D3, D6). Nothing render or the service step does discards a
  candidate.

## Deliverables

1. D1-D5 in `spsc.rs` (the state and reason bits) and `plan_exchange.rs`, with doc comments that
   state the claim and return rules.
2. D6 in the control plane.
3. A loom model of claim, withdraw and return.

## Authorized paths

- `crates/engine/src/realtime/spsc.rs` (the mailbox state machine and its loom tests),
  `crates/engine/src/realtime/plan_exchange.rs`, `crates/engine/src/realtime/mod.rs` (exports and
  its exchange tests).
- `crates/control-plane/src/` (D6 call sites only).

## Non-goals

- Choosing S for any edit (duck-swap #1324, two-phase removal #1325, warm successor #1287).
- The copy-and-return mode of D15-8 step 2 and its return reasons: *Snapshot a running plan into a
  returned successor at a block* (#1354), after *Carry plan state by copy as well as by move*
  (#1322).
- Supersession (#1310).

## Hazards

- The due-sample load and the state-word load are two loads. D2's generation in the word is what
  makes a stale due sample harmless. A design that reads the due sample after claiming would let
  render hold an undue candidate and make it unwithdrawable.
- `render` (`:473`) takes an explicit time: compare against `time.absolute_sample`, not the plan's
  own clock.

## Objective gates

1. **Engine unit tests (new, `crates/engine/src/realtime/mod.rs`):**
   - `NoEarlierThan(S)`: blocks before S render the predecessor (`SwapOutcome::None`, its plan ID);
     the first block starting at or after S applies the swap.
   - `ExactlyAt(S)` published on time applies at S.
   - `ExactlyAt(S)` published after render passed S is returned: `returned_count` 1, the
     predecessor keeps rendering, `withdraw` yields `Returned { reason: Late }` with the epoch and
     plan ID intact, and before that `withdraw` nothing new can be published.
   - While the candidate is `Returned`, a model-only revision committed through
     `PlanPublisher::set_revision` lands in the returned cell (`RevisionTarget::Pending`): the
     watermark (#1314) does not advance while the predecessor keeps rendering.
   - After the returned candidate is taken, a `Next` republish of it is adopted at the next block,
     and that block's watermark advance covers the revision committed while it was returned.
   - A `NoEarlierThan` candidate withdrawn before S is never adopted.
   - `ExactlyAt` off the grid is refused at reservation.
   - Retirement credits balance after each case (reserve, return, take, drop or republish).
2. **Loom (new `spsc_loom_plan_mailbox_*` tests in `spsc.rs`).** Render claims or returns while
   control withdraws and republishes: exactly one side wins each race, a due candidate is adopted
   at most once, an undue one is never claimed, and a returned one is taken by control exactly once
   and never reported `Taken`. Command: `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom
   --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom` (the CI step,
   `qualification.yml:678`).
3. **Render stays allocation-free and syscall-free.** `cargo build --locked --release -p audit
   -p capi && target/release/audit capi`: allocations, deallocations, locks, syscalls and
   `total_violations` 0. `bash scripts/check-realtime-policy.sh` and
   `bash scripts/test-realtime-policy.sh`.
4. **No behaviour change for `Next`.** `cargo test --locked -p capi`,
   `cargo test --locked -p control-plane --features test-support`,
   `cargo test --locked -p host-core --features control-provider,test-support --test
   successor_swap`, `cargo test --locked -p graph --features test-support --test
   rt11_swap_carry_alloc`, `cargo test --locked -p engine --features realtime-audit`.
5. **Workspace.** `cargo fmt --all -- --check`; `cargo clippy --locked --workspace
   --all-targets --all-features -- -D warnings`; `bash scripts/check-cross-targets.sh`.

## Test value

- Gate 1, `NoEarlierThan`: a swap applied at the first block regardless of S.
- Gate 1, `ExactlyAt` late: a candidate adopted late (breaking the warm successor's exact catch-up),
  dropped instead of returned, or reported as `Taken` (which would make supersession build against
  a plan render does not run).
- Gate 1, revision while returned: a `set_revision` that falls back to the `Active` cell for a
  `Returned` candidate reports the revision in effect while the predecessor, which lacks it, still
  renders.
- Gate 1, withdrawn before S: render claiming an undue candidate, which would make a scheduled
  candidate impossible to supersede.
- Gate 2: an ordering bug between claim, return and withdraw that only some interleavings expose.

## Dependencies

- *Let the control thread withdraw an unadopted candidate plan* (#1343). This slice follows it in
  `spsc.rs` and `plan_exchange.rs`, and both precede stream C's edits there.
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309), for D6.
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314): the cell's
  revision word and the routing rule D6 extends to `Returned`.
