# Rack: a live bank stage's quiet-walk cache is cleared by hand at five sites, and drain-before-render is a convention

Item GR-5 of the implicit-state-machine tracker #2258 (audit of 2026-11-06).

## Mission
Make the per-block contract between a bank stage's drain and its process body explicit in
`crates/rack/src/lib.rs`, where it can be made explicit without rippling through every
`BankStage` implementation:
- the live stage's quiet-walk cache (`quiet_at`, issue #1763) is owned beside the lanes it
  describes, so no mutator can keep a stale mark;
- the stage's settle record (`settled`, `settle_edges`, `settle_primed`, issue #2135) cannot hold
  bits before a drain has written any;
- the chain's step 0 (every slot's drain) yields a proof that the block's stage bodies, rendered or
  skipped, require.

No rendered bit moves.

## Verified defect (base `d11fed9b4`)
`crates/rack/src/lib.rs`, base lines:
- **Drain then process.** `BankStage::begin_block` (511) is documented as the drain that runs
  before any lane of the block. `LiveControlEffectBankStage::drain` (2735) writes `offsets`,
  `packed`, `staged_spans` and `walk` (fields 1646-1652, written by `read_cells`, 2796-2838), and
  `process_inner` (2988) reads them (`let packed = self.staged_spans`, 2991; `self.walk.targets`,
  `needs_dry`, `ramping`). Nothing but `BankChain::run_with_input` (4353) sequences the two calls:
  the drain loop (4383-4390), then the skip (4481-4489) or the two process loops (4501-4555).
- **The settle record.** `settled: u8`, `settle_edges: u8` and `settle_primed: bool` (1689-1694).
  `settle_primed` says whether `settled` means anything; before the first drain
  `settled_bypassed_lanes` (2646) ignores `settled` and derives the bits from the lanes. A lane swap
  (2465-2466, 2518-2524) trades raw bits whatever either side's `settle_primed` says.
- **The quiet mark.** `quiet_at: Option<u64>` (1654) is written by `read_cells` (2837) and read by
  `drain` (2739) to skip the cell walk under an unchanged snapshot publication. Five mutators clear
  it by hand: `carry_lane_from` (1952), `carry_lane_from_instance` (2076), `install_carry_slots`
  (2176), and `swap_lanes` on itself (2428) and on its cross-stage peer (2494).

## Reachability on base

**Drain then process: no path processes without a fresh drain.**
- The only render caller of `BankStage::process`, `process_mono` and `process_from_left` is
  `BankChain::run_with_input`. The graph reaches a chain only through `run` and
  `run_with_resident_input` (`graph/src/runtime.rs` 5522, 5530); `slot_stage_mut` (4935) is "never
  called by `run`" and serves bind and carry only.
- Every path of `run_with_input` drains every slot with an active lane first (4383-4390), under
  the same `has_active_lanes` guard the process loops use.
- *Silent skip* (4481-4489): after the drains; it calls `advance_clocks`, never `process`.
- *Collapsed block*: after the drains; `process_mono` on the prefix, the seam after it.
- *Carry*: `carry_observes` (a queue drain) and the carries never call `process`. The successor's
  first block is an ordinary `run`, so it drains first.
- *First block after a swap*: `PreparedPlanExecutor::adopt_predecessor`
  (`engine/src/realtime/plan.rs` 336-352) runs "before it renders that block", and that block
  drains first.
- *Errors*: a drain that fails returns before any process. A process that fails leaves later slots
  drained and unprocessed; the next block drains them again, which rewrites `offsets`, `packed`
  and `walk` (a quiet skip keeps only an empty partition).

**The settle record: no stale bit is read on base.**
- `settle_edges` is read only by `run_with_input`, right after each slot's drain (4387).
- `settled_bypassed_lanes` (read by the lane packer at the top of a block, `graph/src/runtime.rs`
  11737) answers from the lanes until the first drain.
- A drained stage that takes a lane from an undrained one by a cross-stage swap takes that stage's
  never-written bit. Reachable through the public surface: a lane bypassed at bind, moved into a
  drained stage, is reported unsettled until the next drain, and that drain reports a settle edge
  the lane never made. Not reachable through the graph: a lane swap needs the slot activity of
  both lanes to agree (`BankChain::lane_swap_check`), and the graph gives a control lane only to a
  bank member (`graph/src/runtime.rs` 9177-9187), so an undrained stage's lanes have no control
  lane and their bits are `0` either way.

**The quiet mark: no mutator forgets it on base.**
- All five clears are present.
- Three are load-bearing, and the existing `a_quiet_live_slot_renders_what_one_read_every_block_does`
  (`rack/tests/lane_swap.rs`) is red when any one is dropped: `carry_lane_from` (1952), and both
  `swap_lanes` clears (2428, 2494).
- Two are defensive: `carry_lane_from_instance` (2076) and `install_carry_slots` (2176). They run
  only on a successor that has never drained (`install_effect_carry_slots` at install,
  `graph/src/lib.rs` 5318; `carry_effect_lanes` at the swap block, 7723, before the successor
  renders), so its mark is `None` already. No rack test turns red when either is dropped.
- The process body's own lane mutation (`advance_bypass_mix`, 3188) runs only when `walk.ramping`,
  which a quiet mark excludes.

So the defect is structural (severity M by the tracker's legend: a forgotten clear in a future
mutator would render a stale walk, wrong bits, until the next publication). The one red-on-base
test is the settle record's swap rule, through the public surface.

## Decision / model
- **`ControlLanes`** (`crates/rack/src/control_lanes.rs`, a private module): the stage's lanes and
  the quiet mark, which nothing else in the crate can touch.
  - Reads go through `Deref<Target = [Option<EffectControlLane>]>`.
  - Every mutable access is `lanes_mut()`, which forgets the mark.
  - Only `walk(snapshot, f)` sets it: it hands the walk the lanes and records the walk's own
    verdict.
  - `quiet_under(snapshot)` is the drain's skip test, `LiveSnapshot::ALL` excluded as before.
  - The five hand clears are gone: carries, slot installs, both swap arms and the crossfade advance
    reach the lanes through `lanes_mut()`.
  - Tracker pattern 1's check at the point of use: a drain that skips `debug_assert!`s
    `quiet_walk_holds`. It rescans the lanes: an empty partition, no staged target, no crossfade
    running, the kept `LaneWalk`, and the kept settled bits.
- **`settle: Option<SettleRecord { settled, edges }>`** replaces the three fields. `None` before the
  first drain.
  - `settled_bypassed_lanes` answers from the lanes while it is `None`; `settle_edges` answers `0`.
  - The drain may skip its walk only on `Some`: the skip arm matches the record.
  - A swap between two drained stages trades both bits, as before.
  - A drained stage that takes a lane from an undrained one records the moved lane's own settled
    state (what the undrained stage reported for it) and no edge. This is the one behaviour change.
- **`Drained`** (zero-sized, private, `#[must_use]`): returned only by `BankChain::drain_slots`,
  which is step 0 extracted from `run_with_input`. `render_slots` (the two process loops and the
  seam, extracted) and `skip_block` take it by value, so neither runs in a block whose drains did
  not, and a block runs at most one of them. No run-time cost.
- **The trait is unchanged.** A `Drained` in `BankStage::process`'s signature would touch every
  implementation: 41 impls (25 of `BankStage`, 16 of `GraphPreparedBuiltinBankProcessor`) in
  `rack`, `graph`, `builtins-compiler`, `graph-compiler` and their tests. Every overriding
  `begin_block` outside `rack` would have to mint the token, so it would prove nothing. A direct
  driver of a stage keeps the obligation by `begin_block`'s contract, and the `Drained` doc says so.

## Non-goals
- `builtins-compiler`'s own `quiet_at` caches (`quiet_walk_skips`), which are a separate owner.
- `crates/effect-contract`, including `EffectControlLane`'s bypass phase (GR-3).
- A token in the `BankStage` trait's signatures (above).
- `BankChain::lanes_at_rest` read after `run` by the graph's packer (`graph/src/runtime.rs` 5579).
  It reads the block's drain output after the block. That is conservative, because a staged span
  declines rest, and the packer treats its facts as a block old by design.

## Gates
- A regression test red on base: the settle record's swap rule.
- A recorded proof for each point that is not reachable.
- The mutations of the new model, each red in a named test.
- A one-time no-bit-moved comparison against base (PR evidence, not a committed digest).
- `cargo test --locked` for `rack` and its dependents `graph`, `builtins-compiler` and `host-core`.
- clippy `-D warnings`, `cargo fmt --check`, rustdoc `-D warnings`.
- `scripts/check-workspace-policy.sh` and the realtime policy tool.

## Evidence

(Filled in below as attempt 1 records it.)

## Attempt record
- **Attempt 1**: in progress.
