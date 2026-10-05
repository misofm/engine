# Prepare a successor across a withdrawn candidate plan

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Host-core can prepare a successor plan B for the plan render runs (P0) when an earlier candidate
A was prepared, committed and then withdrawn unrendered. B carries from P0 exactly what A would
have carried and B leaves unchanged, and B takes over the sources A created together with the
PCM, generation, read position and held seek the host already gave them. This gives
supersession (#1310) the two functions it needs; on its own it changes no host behaviour.

## Context

- A successor is prepared against one predecessor: `SuccessorBase { inventory, committed }`
  (`crates/host-core/src/prepare.rs:641`). Its carry program names that predecessor's identity
  (`GraphCarryProgram { predecessor: base.inventory.plan_identity(), .. }`, `prepare.rs:1294-1302`,
  installed at `:1792`). Carried sources and input sections are chosen against the inventory rows
  and the committed model (`carried_inputs`, `:1263-1291`).
- `PreparedHost` (`prepare.rs:525-538`) keeps the plan's own `inventory`, not which predecessor
  rows it took.
- A source the successor carries is prepared vacant; render moves the consumer in with
  `adopt_sources`, which requires a vacant successor entry and an occupied predecessor entry
  (`crates/source/src/lib.rs:1846-1885`; trait `GraphPreparedSourceSetDriver::adopt_sources`,
  `crates/graph/src/lib.rs:2225`). Producers move by ID with `SourceControlSet::adopt_persisting`
  (`crates/host-core/src/source.rs:286`).
- The held seek and the generation live in the consumer (`PcmSourceConsumer::held_seek`,
  `crates/source/src/lib.rs:1018`).

## Decisions frozen for this slice

- **D1. Record what a successor carried.** `PreparedHost` gains `carried_base:
  Option<PlanStateInventory>`: the predecessor's inventory restricted to the rows this plan's
  carry program takes (sources and input sections today; each carry family stream A adds records
  its rows here too), with the predecessor's plan identity. `None` for a plan prepared without a
  successor base.
- **D2. Prepare across the withdrawn plan with the existing API.** The caller passes
  `SuccessorBase { inventory: A.carried_base, committed: P0's committed model }`. Every row of the
  restricted inventory is a P0 row, so the existing joins run unchanged: an owner carries from P0
  only if A carried it and B leaves it unchanged against P0's model; everything A restarted, added
  or removed is fresh in B. No new preparation entry point.
- **D3. Donate an unrendered candidate's sources.** A new host-core function
  `donate_unrendered_sources(successor, donor) -> usize`, where each side is a prepared plan, its
  `SourceControlSet`, its inventory and its committed model. For every source that is fresh in
  the successor and created fresh (not carried) in the donor, with the same ID, the same
  declaration (content, channels, bit depth, frames) and the same ring configuration, it swaps
  the two consumers inside the plans and the two producers in the control sets. The donor gets
  the successor's fresh ring and drops it with itself. Returns how many moved.
- **D4. The consumer swap.** `GraphPreparedSourceSetDriver` gains `swap_sources(&mut self, other,
  pairs: &[(u32, u32)]) -> bool`, the occupied-to-occupied twin of `adopt_sources`: every pair is
  checked (both occupied, equal channel counts and quanta) before any swap, so a refusal moves
  nothing. `graph` exposes `donate_sources(successor: &mut PreparedRenderPlan, donor: &mut
  PreparedRenderPlan, pairs) -> bool` through the executors, as `install_carry_program` reaches
  them. It runs on the control thread, on two plans neither of which render owns.
- **D5. Preconditions.** The donor must never have rendered (its `rendered_blocks == 0`,
  `crates/engine/src/realtime/plan.rs:547`, exposed by a read-only accessor) and the
  successor must be unpublished; `donate_unrendered_sources` refuses otherwise with a typed error
  and moves nothing.
- **D6. Acked-batch question: can an ack ever precede a drop? No.** This slice acks nothing. It
  exists so that #1310 never drops PCM, a generation or a held seek the host was acked for: they
  move with the consumer.

## Deliverables

1. D1 in `prepare.rs`; D3 in host-core; D4 in `graph` and `source`.
2. Doc comments stating D2's base rule and D5's preconditions.

## Authorized paths

- `crates/host-core/src/prepare.rs` (D1, and the plumbing for D3): stream A's file, sequenced by
  the coordinator.
- `crates/host-core/src/source.rs`, a new `crates/host-core/tests/withdrawn_successor.rs`.
- `crates/graph/src/lib.rs` (the D4 trait method and `donate_sources` only): stream A's crate.
- `crates/source/src/lib.rs` (the `swap_sources` implementation only).
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
2. **A re-added source is fresh (same file).** A removes `b`; B adds `b` back unchanged: B
   allocates a fresh ring for `b` (generation 1, frame 0), not P0's.
3. **Donation (same file).** Feed `c` through A's producers for 4 blocks and `seek_at(c, 2, 0,
   S)`; prepare B; `donate_unrendered_sources` returns 1; B adopting P0 plays `c` from `S` with the
   fed PCM, as the twin does.
4. **Preconditions (same file).** A donor that rendered one block is refused, and so is a
   declaration or ring-config mismatch; nothing moves (both control sets and both plans' source
   reports unchanged).
5. **Realtime and workspace.** `cargo test --locked -p host-core --features
   control-provider,test-support`; `cargo test --locked -p graph --features test-support`;
   `cargo test --locked -p source`; `bash scripts/check-realtime-policy.sh`;
   `bash scripts/check-graph-policy.sh`; `bash scripts/check-host-core-policy.sh`;
   `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features
   -- -D warnings`.

## Test value

- Gate 1: a base that carries what A restarted, or compares against A's model instead of P0's.
- Gate 2: a carry rule that resurrects P0's ring for a source the host was told is new.
- Gate 3: a donation that drops acked PCM or a held seek.
- Gate 4: a donation that moves a ring render owns, or half a batch.

## Dependencies

- *Carry fader, mute and pan ramps across a plan swap* (#1277): stream A's open edit of
  `prepare.rs` lands first; later carry families record their rows in `carried_base` as D1 says.
