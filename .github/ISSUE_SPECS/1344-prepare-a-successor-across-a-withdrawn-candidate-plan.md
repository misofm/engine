# Prepare a successor across a withdrawn candidate plan

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Host-core can prepare a successor plan B for the plan render runs (P0) when an earlier candidate A
was prepared, committed and then taken back before render adopted it. B carries from P0 exactly
what A would have carried and B leaves unchanged, and B takes over the sources A created together
with the PCM, generation, read position and held seek the host already gave them. The donation is
checked completely before anything is committed (against A itself, or against a record of A's
sources when render holds A), and then cannot fail. This gives supersession (#1310) the functions
it needs; on its own it changes no host behaviour.

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
  - `check_donation(successor, donor) -> Result<DonationPlan, DonationError>` is read-only. The
    successor side is a prepared plan, its `SourceControlSet`, its inventory and its committed
    model. The donor side is one of:
    - `Donor::Held`: a donor plan the control thread holds (withdrawn, returned or taken back from
      a catch-up), with its `SourceControlSet`;
    - `Donor::Record`: a `DonorRecord` plus the donor's `SourceControlSet`, for a donor the control
      thread does not hold at the commit (render is copying into it, *Supersede a running catch-up
      by a structural edit*, #1357 D1). `DonorRecord::of(&PreparedHost)` is captured once, at the
      donor's preparation, before its first publication: for each source the donor created fresh
      (not carried), its ID, declaration and ring configuration. It owns plain values and borrows
      nothing.
  - It selects every source that is fresh in the successor and created fresh in the donor, with the
    same ID, the same declaration (content, channels, bit depth, frames) and the same ring
    configuration. It then checks every condition the apply step needs (D4, D5) and returns the
    pairs. It moves nothing.
  - The apply has two infallible halves, each consuming its part of the `DonationPlan`:
    - `donate_producers` swaps the two producers in the control sets;
    - `donate_consumers` swaps the two consumers inside the plans (D4).
    `apply_donation(successor, donor, DonationPlan) -> usize` runs both and returns how many moved.
    The donor ends with the successor's fresh rings and drops them with itself.
  - The halves run apart only in #1357's in-flight case: `donate_producers` right after the commit,
    `donate_consumers` when the service step has the donor back. In between, every submission and
    seek for a donated source addresses the successor's control set, which already holds the
    donor's producer, so it lands in the ring the successor will own. Nothing is ever written into
    a ring that goes to the donor after the commit.
  - Between check and apply neither side can change what was checked: the successor is
    unpublished and control-owned, and the donor is either control-owned or, for a record, a plan
    whose fresh entries nothing touches before its adoption (D5). `DonationPlan` borrows neither
    side.
- **D4. The consumer swap.** `GraphPreparedSourceSetDriver` gains `can_swap_sources(&self, other,
  pairs) -> bool` and `swap_sources(&mut self, other, pairs)`, the occupied-to-occupied twin of
  `adopt_sources`. The check verifies every pair (both occupied, equal channel counts and quanta).
  The swap is infallible on checked pairs and moves each entry's whole contents. A fresh entry
  never holds a peek: a warm successor keeps it closed until adoption (*Catch up a returned
  successor and adopt it exactly at a scheduled sample*, #1355 D2). `graph` exposes `can_donate_sources` and `donate_sources(successor: &mut
  PreparedRenderPlan, donor: &mut PreparedRenderPlan, pairs)` through the executors, as
  `install_carry_program` reaches them. It runs on the control thread, on two plans neither of
  which render owns.
- **D5. Preconditions, checked in `check_donation`.** The successor is unpublished, and every
  donated ring is unconsumed since its generation start: its consumer has never begun a block.
  - `PcmSourceConsumer::has_begun_block(&self) -> bool` is a new read-only accessor over a
    render-owned `bool` that `begin_block_with` (`crates/source/src/lib.rs:1119`, called by
    `begin_block` at `:1103` and `begin_block_at` at `:1115`) sets. Before that first block the
    consumer has popped no PCM and no command, so every acked chunk, the generation and any seek
    are still in the ring.
  - How many blocks the donor rendered is not a condition: a warm successor's catch-up and its
    render-thread pre-roll render blocks, but an added entry stays closed until adoption: it reads
    `+0.0` and its consumer's `begin_block_at` is never called (#1355 D2). A donor that played one
    of its fresh rings is refused with `DonationError::Consumed`, a typed error.
  - With `Donor::Record` the consumer is out of reach (render may be copying into the donor). The
    condition then holds by construction: a fresh entry is never begun before its plan is adopted,
    because render's copy writes only the entries its program names (*Snapshot a running plan into
    a returned successor at a block*, #1354), and neither the off-thread catch-up nor the
    render-thread pre-roll (*Fall back from a missed catch-up deadline: bounded render-thread
    pre-roll, then the transition*, #1358 D4) opens an added entry before adoption (#1355 D2).
    `donate_consumers` debug-asserts `!has_begun_block()` for each pair when it runs.
  - An adopted plan is never a donor: it is the base (#1310's *taken* case).
- **D6. Acked-batch question: can an ack ever precede a drop? No.** This slice acks nothing. It
  exists so that #1310 and the catch-up never drop PCM, a generation or a held seek the host was
  acked for: they stay in a ring no consumer has begun, and they move with the consumer. Every
  refusal happens in `check_donation`, so a caller can run it before the protocol commit, and the
  post-commit halves cannot fail. In the split case, submissions after the commit reach the ring
  the successor keeps (D3).

## Deliverables

1. D1 in `prepare.rs` and `graph`; D3 (with `DonorRecord` and the two halves) in host-core; D4
   and D5's accessor in `graph` and `source`.
2. Doc comments stating D2's base rule and D3's check-then-apply contract.

## Authorized paths

- `crates/host-core/src/prepare.rs` (D1, and the plumbing for D3): stream A's file, sequenced by
  the coordinator.
- `crates/host-core/src/source.rs`, a new `crates/host-core/tests/withdrawn_successor.rs`.
- `crates/graph/src/lib.rs` (`installed_carry_reads`, the D4 trait methods and
  `can_donate_sources`/`donate_sources` only): stream A's crate.
- `crates/source/src/lib.rs` (the `can_swap_sources`/`swap_sources` implementation and the
  `has_begun_block` accessor only).

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
3. **A source absent from the running plan is never carried from the withdrawn one (same
   file).** P0 lacks `d`. A adds `d` with 4 seconds of frames; B adds `d` with 5 seconds, so no
   donation applies: B allocates a fresh ring for `d` (generation 1, frame 0), not A's. (A source
   present in P0 that A removes and B restores is *Remove a strip in two phases: ramp out, then a
   scheduled swap* (#1325) D6's restore, which keeps the running plan's ring; this gate does not
   cover it.)
4. **Donation (same file).** Feed `c` through A's producers for 4 blocks and `seek_at(c, 2, 0,
   S)`; prepare B; `check_donation` returns one pair and `apply_donation` returns 1; B adopting P0
   plays `c` from `S` with the fed PCM, as the twin does.
5. **Refusals move nothing (same file).** A donor whose fresh `c` consumer began one block (a
   control-thread `render_contiguous` of A) is refused with `DonationError::Consumed`, and so is
   a declaration or ring-config mismatch. Both control sets and both plans' source reports are
   unchanged after each refusal.
6. **Record and split apply (same file).** Capture `DonorRecord::of(A)` before A is published.
   `check_donation` with `Donor::Record` returns the same pairs as with `Donor::Held`. Run
   `donate_producers` alone, then submit two more chunks of `c` and a `seek_at(c, 3, ..)` through
   B's control set, then `donate_consumers`: B adopting P0 plays every chunk fed through either
   set, honours the generation-3 seek, and A's dropped ring held nothing submitted after the
   producer half.
7. **Realtime and workspace.** `cargo test --locked -p host-core --features
   control-provider,test-support`; `cargo test --locked -p graph --features test-support`;
   `cargo test --locked -p source`; `bash scripts/check-realtime-policy.sh`;
   `bash scripts/check-graph-policy.sh`; `bash scripts/check-host-core-policy.sh`;
   `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features
   -- -D warnings`.

## Test value

- Gate 1: a base that carries what A restarted, or compares against A's model instead of P0's.
- Gate 2: a `carried_base` that lists rows the hand-over does not read, or misses one it does.
- Gate 3: a carry rule that takes a ring from the withdrawn plan's inventory (instead of by a
  checked donation) for a source the running plan does not have.
- Gate 4: a donation that drops acked PCM or a held seek.
- Gate 5: a check that passes a consumed ring (whose dropped frames the host was acked for), a
  donation the apply step cannot perform, or a check with a side effect.
- Gate 6: a record that selects different pairs from the held donor, or a split apply that leaves
  the successor's set pointing at its own fresh producer, so chunks acked after the commit go to
  the ring the donor drops.

## Dependencies

- *Carry fader, mute and pan ramps across a plan swap* (#1277): stream A's open edit of
  `prepare.rs` lands first. Its new carry section is then reported by `installed_carry_reads` (D1).
