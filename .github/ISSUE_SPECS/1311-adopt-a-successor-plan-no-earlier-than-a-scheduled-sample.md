# Adopt a successor plan no earlier than a scheduled sample

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8, D15-9; D15-8 (round-5 amendment)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The plan exchange can hold a published candidate until a chosen render sample. A candidate
published "no earlier than S" is adopted at the first block that starts at or after S. A candidate
published `Primed { not_before, lead_blocks }` is adopted at the first block that starts at or
after `not_before` and at which the running plan reports it ready for a prime of `lead_blocks`
blocks. Render checks the due sample and the readiness before it claims, and it adopts in the same
step as the claim, so a candidate is always either unclaimed and withdrawable, or adopted. Nothing
is ever handed back from render. Duck-swap and two-phase removal (stream D) schedule swaps with
this, and the warm successor (stream C) publishes `Primed`.

## Context

- Today the render thread adopts a reserved candidate at the first block after publication
  (`enter_block`, `crates/engine/src/realtime/plan_exchange.rs:375-437`; applied at `:436`). There
  is no notion of a due sample.
- The incoming plan adopts the outgoing plan's clock at the swap (`:414-418`), so "the block that
  starts at S" means `absolute_sample == S` on the host's render timeline, in both
  `render_contiguous` (`:448`) and `render` (`:473`).
- `RealtimePlanOwner` keeps a saturating `carried_count` (`:366`).
- The executor seam is `PreparedPlanExecutor` (`crates/engine/src/realtime/plan.rs:269`). Its
  defaulted hooks (for example `adopt_predecessor`, reached through `carry_from`, `:917`) are how
  the engine asks a graph plan something without depending on graph types.
- *Let the control thread withdraw an unadopted candidate plan* (#1343) replaces publication with a
  two-cell mailbox in `crates/engine/src/realtime/spsc.rs`: one `AtomicU64` state word holds a
  generation and a 2-bit state per cell (`Empty`, `Full`, `Active`). Render claims a `Full` cell by
  one compare-and-swap that makes it `Active` and the old `Active` cell `Empty`. Control withdraws a
  `Full` cell by compare-and-swap: `Withdrawal::{Withdrawn, Taken, Nothing}`. It also removes
  `deferred_count` and `DeferredRetirementFull`.
- D15-8 (round-5 amendment) replaces the off-thread catch-up with prime adoption. It deletes the
  exact-sample adoption ("adopt exactly at S, else return") and the return path this issue once
  carried; the warm successor is published `Primed` instead.

## Decisions frozen for this slice

- **D1. Adoption kind on the reservation.** `PlanAdoption::{Next, NoEarlierThan(u64), Primed {
  not_before: u64, lead_blocks: u32 }}` is fixed when the control thread reserves a replacement and
  stored in the cell beside the plan. `Next` is today's behaviour; every existing caller passes it.
  - The kind, the due sample (`NoEarlierThan`'s S or `Primed`'s `not_before`) and `lead_blocks` are
    three atomics of the cell (`AtomicU8`, `AtomicU64`, `AtomicU32`), outside the `UnsafeCell` that
    holds the plan payload. Control stores them (`Relaxed`) before the state word's `Release` that
    marks the cell `Full`. Render loads them before it claims (D2), while control may withdraw and
    republish into the same cell, so they are never fields of the payload: a plain read of the
    payload before the claim would race with that republish.
  - `UnadoptedCandidate` keeps its `PlanAdoption`. A withdrawn candidate that is republished (#1310
    D4's refusal path) is published with the same kind and due sample.
- **D2. Render decides before it claims.** Render loads the state word (`Acquire`), then the `Full`
  cell's kind, due sample and `lead_blocks` from their atomics (D1; `Relaxed` suffices, because the
  claim below re-checks the generation). It touches the payload only after the claim succeeds.
  - It claims (#1343's one compare-and-swap, generation included) only when the block's start is at
    or past the due sample and, for `Primed`, the readiness hook (D3) returns `true`.
  - If control withdrew and republished between the loads and the claim, the generation changed and
    the claim fails. Render then does nothing this block and looks again next block. Render never
    spins and never claims a candidate that is not due or not ready, so such a candidate stays
    withdrawable.
- **D3. The readiness hook.** `PreparedPlanExecutor` gains `fn prime_ready(&self, block_start:
  u64, lead_blocks: u32) -> bool`, defaulting to `false`; `PreparedRenderPlan` forwards it.
  - Render calls it on the **active** plan, never on the candidate, and only for a `Primed`
    candidate that is due. It runs inside the render scope and must obey render's rules: no
    allocation, free, lock, syscall or unbounded loop.
  - The graph plan's implementation is *Adopt a warm successor with a raw-frame prime at the first
    ready block* (#1355), over *Let a source consumer check and replay its next blocks for a prime*
    (#1320). It reads per-ring `prime_required` atomics that control stores before the `Release`
    that publishes the candidate. A stale read is harmless: a read that belongs to another
    publication goes with a state word of another generation, so the claim fails (D2).
  - The default `false` means a plan that cannot check readiness never adopts a `Primed`
    candidate. Only #1355's slices publish `Primed`, and they publish it only over graph plans.
- **D4. Claim and adoption are one step.** A claimed candidate is adopted in the same `enter_block`
  call, before the block renders: the clock adoption and `carry_from` (`:414-421`), and for
  `Primed` the prime that #1355 runs in that block. There is no in-flight, returned or partial
  state. `Withdrawal::Taken` keeps meaning adopted, and only that.
- **D5. Off-grid schedules.** `NoEarlierThan(S)` and `Primed { not_before: S, .. }` accept any `S`
  and adopt at the first qualifying block whose start is at or past it.
- **D6. Control-side API.** `PlanPublisher::reserve_replacement(plan, adoption)`.
  `PlanPublisher::withdraw()` keeps #1343's outcomes `Withdrawn`, `Taken` and `Nothing`.
  - The control plane threads `PlanAdoption::Next` through its publication.
  - A `Primed` or `NoEarlierThan` candidate is an ordinary `Full` cell for every other rule:
    `PlanPublisher::set_revision` routes to it (#1314 D2), and supersession withdraws it (#1310).
  - The deadline after which a `Primed` candidate that never became ready falls back is control's
    business (*Fall back to the transition when a warm successor is not ready by its deadline*,
    #1358): it withdraws the candidate, and `Taken` there means render adopted it exactly.
- **D7. Acked-batch question: can an ack ever precede a drop? No.** Scheduling changes when a
  candidate is adopted, never whether its content survives. An unclaimed candidate stays in its
  cell, withdrawable (D2). A claimed one is adopted in the same step (D4). Nothing render or the
  service step does discards a candidate.

## Deliverables

1. D1-D5 in `spsc.rs` (the cell atomics), `plan_exchange.rs` and `plan.rs` (the D3 hook and its
   forwarder), with doc comments that state the claim rule.
2. D6 in the control plane.
3. A loom model of the due and readiness check, the claim and withdrawal.

## Authorized paths

- `crates/engine/src/realtime/spsc.rs` (the mailbox state machine and its loom tests),
  `crates/engine/src/realtime/plan_exchange.rs`, `crates/engine/src/realtime/plan.rs` (the D3
  hook only), `crates/engine/src/realtime/mod.rs` (exports and its exchange tests).
- `crates/control-plane/src/` (D6 call sites only).

## Non-goals

- Choosing S for any edit (duck-swap #1324, two-phase removal #1325, warm successor #1287).
- The graph plan's readiness check and the prime (#1320, #1355).
- Supersession (#1310).

## Hazards

- The due-sample load, the readiness check and the claim are separate steps. D2's generation in
  the word is what makes a stale due sample or a stale readiness answer harmless. A design that
  checks after claiming would let render hold a candidate it cannot adopt and make it
  unwithdrawable.
- `render` (`:473`) takes an explicit time: compare against `time.absolute_sample`, not the plan's
  own clock.

## Objective gates

1. **Engine unit tests (new, `crates/engine/src/realtime/mod.rs`).** A test executor whose
   `prime_ready` returns a test-controlled flag and records its arguments:
   - `NoEarlierThan(S)`: blocks before S render the predecessor (`SwapOutcome::None`, its plan ID);
     the first block starting at or after S applies the swap.
   - `Primed { not_before: S, lead_blocks: k }` with the flag `true`: not adopted before S; adopted
     at the first block starting at or after S, and `prime_ready` saw that block's start and `k`.
   - The same with the flag `false` until block `S + 3q`: blocks S to `S + 2q` render the
     predecessor and the candidate stays `Full`; it is adopted at `S + 3q`, the first block after
     the flag turns `true`.
   - A `Primed` candidate withdrawn while unready yields `Withdrawn` with its epoch, plan ID and
     `PlanAdoption` intact; republished, it is adopted with the same `not_before`.
   - `prime_ready` is never called for a `Next` or `NoEarlierThan` candidate, nor for a `Primed`
     one before its `not_before`.
   - A `NoEarlierThan` candidate withdrawn before S is never adopted.
   - A plan whose executor keeps the default `prime_ready` never adopts a `Primed` candidate.
   - Retirement credits balance after each case (reserve, withdraw, drop or republish, adopt).
2. **Loom (new `spsc_loom_plan_mailbox_*` tests in `spsc.rs`).** Render checks the due sample and a
   readiness flag that control stores before its `Release`, and claims, while control withdraws and
   republishes with another kind and another readiness flag: exactly one side wins each race, a
   candidate is adopted at most once, an undue or unready candidate is never claimed, and a
   readiness flag read from one publication never lets render claim another. Command:
   `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test
   --locked --release -p engine --lib spsc_loom` (the CI step, `qualification.yml:678`).
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
- Gate 1, `Primed` unready: a claim that ignores readiness adopts a warm successor whose prime
  frames are missing, so its carried nodes diverge from the predecessor at S.
- Gate 1, readiness arguments: a hook called on the candidate instead of the active plan, or with
  the wrong block start, checks the wrong consumers or the wrong window.
- Gate 1, withdrawn and republished: a republish that drops the kind adopts a warm successor
  without its prime or before its `not_before`.
- Gate 1, default hook: a default of `true` lets a plan that cannot check readiness adopt
  unprimed.
- Gate 2: an ordering bug between the readiness read, the claim and withdraw that only some
  interleavings expose.

## Dependencies

- *Let the control thread withdraw an unadopted candidate plan* (#1343). This slice follows it in
  `spsc.rs` and `plan_exchange.rs`, and both precede stream C's edits there.
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309), for D6.
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314): the cell's
  revision word and the routing rule D6 relies on.
