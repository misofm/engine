# Hand the outgoing plan to its successor at the swap block

Slice 1 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

The engine core gives a successor plan one chance, in the block that makes it active, to take state
from the plan it displaces. It runs on the render thread, inside the render scope, after the clock
adoption and before the successor renders, and only in the block that applies the swap. A
synchronous host (the browser) makes the same hand-over without the exchange, and its successor
continues the predecessor's clock. After this slice no production executor uses the hook; slices 2
and 7-14 fill it.

## Context

- `RealtimePlanOwner::enter_block` (`crates/engine/src/realtime/plan_exchange.rs:358-404`) pops a
  candidate, takes a retirement slot, replaces the active plan (`core::mem::replace`, `:392`), adopts
  the clock (`:395`) and pushes the old plan to retirement (`:396`). Between `:392` and `:396` it
  owns both plans. `render_contiguous` (`:415`) and `render` (`:439`) call it inside
  `super::audit::in_render_scope`; the realtime-policy region spans `:341-456`.
- A deferred swap (retirement full) returns before `:392` (`:372-386`).
- `PreparedRenderPlan` (`crates/engine/src/realtime/plan.rs:508`) owns an optional
  `Box<dyn PreparedPlanExecutor>` (trait at `:269`; production implementations are policy-limited to
  `graph`, `:265-268`). The trait already carries hidden hooks with defaults.
- `adopt_absolute_sample` is `pub(crate)` (`plan.rs:580`). `copy_response_snapshot` stamps its
  capture with the plan's own `next_absolute_sample` (`:643`), so a plan that never adopts the clock
  stamps from 0. The browser renders through `plan.render` with the host's clock
  (`hosts/host-web/src/lib.rs:3229-3234`).
- `engine` cannot dev-depend on `bench-support` (which depends on `engine`), and a counting
  `#[global_allocator]` needs `unsafe`, which `scripts/check-realtime-policy.sh:28-32` refuses
  outside a fixed file list.

## Decisions frozen for this slice

- **D1. The hook.** Add to `PreparedPlanExecutor`:
  - `fn as_any_mut(&mut self) -> Option<&mut dyn core::any::Any> { None }`;
  - `fn adopt_predecessor(&mut self, predecessor: &mut dyn PreparedPlanExecutor) -> CarryOutcome
    { CarryOutcome::NotRequested }`.
  - `CarryOutcome` is a new `Copy` enum in `engine::realtime`: `NotRequested`, `Carried`,
    `PredecessorMismatch`.
- **D2. When it runs.** In `enter_block`, after `adopt_absolute_sample` (`:395`) and before the
  retirement push (`:396`), on the `Applied` path only. Never on `None`, on
  `DeferredRetirementFull`, or for a candidate dropped with the owner.
- **D3. Synchronous form.** `PreparedRenderPlan::adopt_predecessor_plan(&mut self, predecessor: &mut
  PreparedRenderPlan) -> CarryOutcome` is public. It adopts the predecessor's
  `next_absolute_sample`, then calls the executor hook when both plans have executors. When called
  outside a render scope it arms one (`in_render_scope`), so the audit sees the hand-over in every
  host.
- **D4. Report.** `RealtimeRenderReport` gains `carry: CarryOutcome` (`NotRequested` unless the swap
  was applied). `RealtimePlanOwner` keeps two saturating counters, carried swaps and mismatched
  swaps, read like `deferred_count` (`:355`).
- **D5. Realtime.** The hook is render-thread code: no allocation, free, lock, syscall, log or
  unbounded loop, inside the existing `REALTIME_POLICY` region.

## Deliverables

1. D1-D4 in `crates/engine/src/realtime/plan.rs`, `plan_exchange.rs`, `mod.rs`.
2. A test-only executor in the engine's unit tests: a sine oscillator whose phase is its only
   state; its `adopt_predecessor` downcasts the predecessor and copies the phase.
3. Rustdoc on the hook: where it runs, that it is render-thread code, that the predecessor is
   retired after it returns.

## Authorized paths

- `crates/engine/src/realtime/plan.rs`, `plan_exchange.rs`, `mod.rs`
- `crates/engine/src/lib.rs` (re-exports only)

## Non-goals

- No graph, source, host-core, C ABI or browser change.
- No change to publication, retirement credits or deferral; no new `SwapOutcome` variant.

## Objective gates

1. **Gap-free acceptance.** Render the oscillator executor through `plan_exchange`: plan A for 5
   blocks, publish B (same envelope, phase zero, carry on), render 5 more. The 10 blocks are
   bit-identical to one unswapped oscillator rendered for 10 blocks. With carry off in B, block 6
   differs.
2. **Only on the applied block.** With the retirement queue full the swap defers: no hook call,
   `carry == NotRequested`. After a reclaim the swap applies on a later block and the hook runs once,
   then; gate 1's equality holds across the deferred blocks.
3. **Never on drop.** Dropping the owner with a reserved, unapplied candidate never calls the hook.
4. **Realtime.** The existing swap audits, which now pass through the hook on every applied swap
   (it returns `NotRequested` there), stay at zero violations and zero syscalls:
   `trace-realtime-audit.sh` and `trace-builtins-graph-audit.sh`. The allocation-counter gate for a
   carry that moves real state is slice 3's, in host-core, with `bench_support`; this slice adds no
   `unsafe` test allocator.
5. **Synchronous form.** `adopt_predecessor_plan` on two directly owned plans gives gate 1's
   equality, and a response capture after it is stamped with the continued clock.
6. Commands:
   - `cargo test --locked -p engine` and `cargo test --locked -p engine --features realtime-audit`
   - `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom`
   - `cargo build --locked --release -p audit && timeout 120s bash scripts/trace-realtime-audit.sh target/release/audit 1000000 && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - the umbrella's inherited gates.

## Test value

- Gate 1: a hook that runs after the successor's first render, or not at all, turns it red; no
  existing test checks state across a swap.
- Gate 2: a hook that runs on a deferred block (handing over state the predecessor then keeps
  changing) turns it red.
- Gate 3: a hook that runs from `Drop` (off the render thread, on a plan never made active) turns it
  red.
- Gate 5: a synchronous form that leaves the successor's clock at 0 stamps captures wrongly and turns
  it red.

## Dependencies

None.
