# Prepare a successor across a withdrawn candidate plan

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Host-core can prepare a successor plan B for the plan render runs (P0) when an earlier candidate
A was prepared, committed and then withdrawn unrendered. B carries from P0 exactly what A would
have carried and B leaves unchanged, and B takes over the sources A created together with the
PCM, generation, read position and held seek the host already gave them. The donation is checked
completely before anything is committed, and then cannot fail. This gives supersession (#1310)
the functions it needs; on its own it changes no host behaviour.

## Context

- A successor is prepared against one predecessor: `SuccessorBase { inventory, committed }`
  (`crates/host-core/src/prepare.rs:641`). Its carry program names that predecessor's identity
  (`GraphCarryProgram { predecessor: base.inventory.plan_identity(), .. }`, `prepare.rs:1294-1302`,
  installed at `:1792`). Carried sources and input sections are chosen against the inventory rows
  and the committed model (`carried_inputs`, `:1263-1291`); the input moves are installed with
  `graph::install_builtin_input_carry` (`:1820`).
- The installed hand-over lives in the graph executor: `carry: Option<GraphCarryProgram>` (source
  moves, `(successor index, predecessor index)` pairs) and `carry_inputs: Box<[GraphLaneMove]>`
  (predecessor lane locations), `crates/graph/src/lib.rs:3008-3009`; installers at `:2837` and
  `:2876`.
- `PreparedHost` (`prepare.rs:525-538`) keeps the plan's own `inventory`, not which predecessor
  rows it took. Inventory rows carry the predecessor keys the hand-over reads: a source row's
  `index` (`:542-550`) and an input-section row's `location` (`:566-577`).
- A source the successor carries is prepared vacant; render moves the consumer in with
  `adopt_sources`, which requires a vacant successor entry and an occupied predecessor entry
  (`crates/source/src/lib.rs:1846-1885`; trait `GraphPreparedSourceSetDriver::adopt_sources`,
  `crates/graph/src/lib.rs:2225`). Producers move by ID with `SourceControlSet::adopt_persisting`
  (`crates/host-core/src/source.rs:286`).
- The held seek and the generation live in the consumer (`PcmSourceConsumer::held_seek`,
  `crates/source/src/lib.rs:1018`).

## Decisions frozen for this slice

- **D1. Derive what a successor carried from its installed hand-over.** `PreparedHost` gains
  `carried_base: Option<PlanStateInventory>`. It is computed once, at the end of preparation, from
  what is actually installed in the plan, never recorded family by family:
  - `graph::installed_carry_reads(&mut PreparedRenderPlan) -> Option<GraphCarryReads>` lists every
    predecessor key the installed hand-over reads: source indices from the program, lane locations
    from the input moves. It destructures the executor's carry fields without `..`, so a carry
    section added to the executor later does not compile until this function reports it. That makes
    the installer of each new family its owner here.
  - Host-core restricts the predecessor's inventory to the rows whose key appears in those reads.
    It keeps the predecessor's plan identity. The restriction destructures `PlanStateInventory`
    without `..`, for the same reason.
  - `None` for a plan prepared without a successor base.
- **D2. Prepare across the withdrawn plan with the existing API.** The caller passes
  `SuccessorBase { inventory: A.carried_base, committed: P0's committed model }`. Every row of the
  restricted inventory is a P0 row, so the existing joins run unchanged: an owner carries from P0
  only if A carried it and B leaves it unchanged against P0's model; everything A restarted, added
  or removed is fresh in B. No new preparation entry point.
- **D3. Donation in two steps: check, then apply.**
  - `check_donation(successor, donor) -> Result<DonationPlan, DonationError>` is read-only. Each
    side is a prepared plan, its `SourceControlSet`, its inventory and its committed model. It
    selects every source that is fresh in the successor and created fresh (not carried) in the
    donor, with the same ID, the same declaration (content, channels, bit depth, frames) and the
    same ring configuration. It then checks every condition the apply step needs (D4, D5) and
    returns the pairs. It moves nothing.
  - `apply_donation(successor, donor, DonationPlan) -> usize` performs the swaps and cannot fail.
    It swaps the two consumers inside the plans and the two producers in the control sets. The
    donor gets the successor's fresh ring and drops it with itself. It returns how many moved.
  - Both plans are control-owned between the two calls (the donor withdrawn, the successor
    unpublished), so nothing can invalidate a checked plan. `DonationPlan` borrows neither side
    and is consumed by the apply.
- **D4. The consumer swap.** `GraphPreparedSourceSetDriver` gains `can_swap_sources(&self, other,
  pairs) -> bool` and `swap_sources(&mut self, other, pairs)`, the occupied-to-occupied twin of
  `adopt_sources`. The check verifies every pair (both occupied, equal channel counts and quanta).
  The swap is infallible on checked pairs. `graph` exposes `can_donate_sources` and
  `donate_sources(successor: &mut PreparedRenderPlan, donor: &mut PreparedRenderPlan, pairs)`
  through the executors, as `install_carry_program` reaches them. It runs on the control thread, on
  two plans neither of which render owns.
- **D5. Preconditions, checked in `check_donation`.** The donor never rendered (its
  `rendered_blocks == 0`, `crates/engine/src/realtime/plan.rs:547`, exposed by a read-only
  accessor), and the successor is unpublished. Otherwise `check_donation` refuses with a typed
  error.
- **D6. Acked-batch question: can an ack ever precede a drop? No.** This slice acks nothing. It
  exists so that #1310 never drops PCM, a generation or a held seek the host was acked for: they
  move with the consumer. Because every refusal happens in `check_donation`, #1310 can run it
  before the protocol commit, and the post-commit apply cannot fail.

## Deliverables

1. D1 in `prepare.rs` and `graph`; D3 in host-core; D4 in `graph` and `source`.
2. Doc comments stating D2's base rule and D3's check-then-apply contract.

## Authorized paths

- `crates/host-core/src/prepare.rs` (D1, and the plumbing for D3): stream A's file, sequenced by
  the coordinator.
- `crates/host-core/src/source.rs`, a new `crates/host-core/tests/withdrawn_successor.rs`.
- `crates/graph/src/lib.rs` (`installed_carry_reads`, the D4 trait methods and
  `can_donate_sources`/`donate_sources` only): stream A's crate.
- `crates/source/src/lib.rs` (the `can_swap_sources`/`swap_sources` implementation only).
- `crates/engine/src/realtime/plan.rs` (the `rendered_blocks` accessor only).

## Non-goals

- Withdrawal (#1343) and the control-plane wiring (#1310).
- Any render-thread change.

## Objective gates

1. **Base across a withdrawn plan (new test, `withdrawn_successor.rs`).** Prepare P0 with sources
   `a`, `b` and tracks `t1`-`t3`. Prepare A against P0: change `t1`'s insert quality, remove `b`,
   add source `c`. Prepare B with D2's base: change `t2`'s insert quality. Twin: render-free
   `A.adopt_predecessor_plan(&mut P0')`, then B' prepared against A normally and
   `B'.adopt_predecessor_plan(&mut A)`. B adopting P0 renders bit-identically to B' over 16 blocks,
   with `a` carried, `t1` and `t2` restarted, and `b` absent.
2. **`carried_base` follows the installed hand-over (same file).** For A above, `carried_base`
   holds exactly `a`'s source row and one input-section row per input move A installed, each
   equal to P0's row for that strip, with P0's identity. A plan prepared with no base has `None`.
3. **A re-added source is fresh (same file).** A removes `b`; B adds `b` back unchanged: B
   allocates a fresh ring for `b` (generation 1, frame 0), not P0's.
4. **Donation (same file).** Feed `c` through A's producers for 4 blocks and `seek_at(c, 2, 0,
   S)`; prepare B; `check_donation` returns one pair and `apply_donation` returns 1; B adopting P0
   plays `c` from `S` with the fed PCM, as the twin does.
5. **Refusals move nothing (same file).** A donor that rendered one block is refused by
   `check_donation`, and so is a declaration or ring-config mismatch. Both control sets and both
   plans' source reports are unchanged after each refusal.
6. **Realtime and workspace.** `cargo test --locked -p host-core --features
   control-provider,test-support`; `cargo test --locked -p graph --features test-support`;
   `cargo test --locked -p source`; `bash scripts/check-realtime-policy.sh`;
   `bash scripts/check-graph-policy.sh`; `bash scripts/check-host-core-policy.sh`;
   `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features
   -- -D warnings`.

## Test value

- Gate 1: a base that carries what A restarted, or compares against A's model instead of P0's.
- Gate 2: a `carried_base` that lists rows the hand-over does not read, or misses one it does.
- Gate 3: a carry rule that resurrects P0's ring for a source the host was told is new.
- Gate 4: a donation that drops acked PCM or a held seek.
- Gate 5: a check that passes a donation the apply step cannot perform, or a check with a side
  effect.

## Dependencies

- *Carry fader, mute and pan ramps across a plan swap* (#1277): stream A's open edit of
  `prepare.rs` lands first. Its new carry section is then reported by `installed_carry_reads` (D1).
