# Prepare a successor across a withdrawn candidate plan

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Host-core can prepare a successor plan B for the plan render runs (P0) when an earlier candidate A
was prepared, committed and then taken back before render adopted it. B carries from P0 exactly
what A would have carried and B leaves unchanged, and B takes over the sources A created together
with the PCM, generation, read position and held seek the host already gave them. The donation is
checked completely against A before anything is committed, and then cannot fail. This gives supersession (#1310) the functions
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
- *Adopt a successor plan no earlier than a scheduled sample* (#1311): render adopts a candidate in
  the same step as it claims it, so a withdrawn candidate was never rendered and is wholly
  control-owned. Under D15-8 (round-5 amendment) this holds for a warm successor too: render
  checks its readiness on the running plan's consumers and primes it only at adoption.

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
    model. The donor side is a withdrawn plan the control thread holds, with its
    `SourceControlSet`.
  - It selects every source that is fresh in the successor and created fresh in the donor, with the
    same ID, the same declaration (content, channels, bit depth, frames) and the same ring
    configuration. It then checks every condition the apply step needs (D4, D5) and returns the
    pairs. It moves nothing.
  - The apply has two infallible halves, each consuming its part of the `DonationPlan`:
    - `donate_producers` swaps the two producers in the control sets;
    - `donate_consumers` swaps the two consumers inside the plans (D4).
    `apply_donation(successor, donor, DonationPlan) -> usize` runs both, in that order, and
    returns how many moved. The donor ends with the successor's fresh rings and drops them with
    itself.
  - Between check and apply neither side can change what was checked: the successor is
    unpublished and control-owned, and so is the withdrawn donor. `DonationPlan` borrows neither
    side.
- **D4. The consumer swap.** `GraphPreparedSourceSetDriver` gains `can_swap_sources(&self, other,
  pairs) -> bool` and `swap_sources(&mut self, other, pairs)`, the occupied-to-occupied twin of
  `adopt_sources`. The check verifies every pair (both occupied, equal channel counts and quanta).
  The swap is infallible on checked pairs and moves each entry's whole contents. `graph` exposes
  `can_donate_sources` and `donate_sources(successor: &mut PreparedRenderPlan, donor: &mut
  PreparedRenderPlan, pairs)` through the executors, as
  `install_carry_program` reaches them. It runs on the control thread, on two plans neither of
  which render owns.
- **D5. Preconditions, checked in `check_donation`.** The successor is unpublished, and every
  donated ring is unconsumed since its generation start: its consumer has never begun a block.
  - `PcmSourceConsumer::has_begun_block(&self) -> bool` is a new read-only accessor over a
    render-owned `bool` that `begin_block_with` (`crates/source/src/lib.rs:1119`, called by
    `begin_block` at `:1103` and `begin_block_at` at `:1115`) sets. Before that first block the
    consumer has popped no PCM and no command, so every acked chunk, the generation and any seek
    are still in the ring.
  - A withdrawn donor was never rendered by render (#1311), so in production the condition always
    holds. It is still checked, because a plan rendered on the control thread (a synchronous host,
    or a test) can be a donor. A donor that played one of its fresh rings is refused with
    `DonationError::Consumed`, a typed error.
  - An adopted plan is never a donor: it is the base (#1310's *taken* case).
- **D6. Acked-batch question: can an ack ever precede a drop? No.** This slice acks nothing. It
  exists so that #1310 and the warm successor's transition fallback (#1358, #1397) never drop PCM,
  a generation or a held seek the host was acked for: they stay in a ring no consumer has begun,
  and they move with the consumer. Every refusal happens in `check_donation`, so a caller can run
  it before the protocol commit, and the post-commit halves cannot fail.

## Deliverables

1. D1 in `prepare.rs` and `graph`; D3 (with its two halves) in host-core; D4
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
6. **Realtime and workspace.** `cargo test --locked -p host-core --features
   control-provider,test-support`; `cargo test --locked -p graph --features test-support`;
   `cargo test --locked -p source`; `cargo run --locked --release -q -p realtime-policy`;
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

## Dependencies

- *Carry fader, mute and pan ramps across a plan swap* (#1277): stream A's open edit of
  `prepare.rs` lands first. Its new carry section is then reported by `installed_carry_reads` (D1).

## Amendment (controls coordinator, batch 15 check, 2026-10-15)

Checked against `e2f6cfb24`.

**Batch order (root, 2026-10-15).** Batch 15a is the controls lane: #1344, #1398, #1310, in
that order. #1324 left batch 15a (root ruling, 2026-10-15); it lands in a later batch after #1288,
#1391, #1363 and #1325, in that order. The rest of batch 15 follows. This slice is first.

**Moved anchors.** Each fact in Context still holds, except as noted.
- `SuccessorBase` is `crates/host-core/src/prepare.rs:1070-1097`. It now has six fields:
  `inventory`, `committed`, `revision` (#1277 D10), `edit_ramps` (#1394), `monitor` (#1559) and
  `observation` (#1584). D2's literal `SuccessorBase { inventory, committed }` must set all six.
  Tests here use the neutral values: `EditRamps::new()`, `MonitorBase::NONE` (`prepare.rs:1125`),
  `NO_OBSERVATION` (`:1100`) and a revision of at least 1 (`host.successor.revision`). The real
  values for a superseding successor are #1310's.
- The carry program is built at `prepare.rs:2131-2134` and installed at `:2744`. A second
  install, with no sources, is at `:3067-3073` (a plan whose only carry is a line or a send).
- `carried_inputs` is no longer a function. It is a local at `prepare.rs:2016`; the input moves
  are installed at `:2772`. `carried_sources` is the local at `:1953`.
- `PreparedHost` is `prepare.rs:659-675`. `SourceInventoryRow` is `:695-703` (`index` at `:702`).
  `InputSectionInventoryRow` is `:871` (its `location` at `:876`). `PlanStateInventory` is
  `:711-751`.
- The executor's carry fields are `crates/graph/src/lib.rs:3211-3244`. The layout witness
  `GraphExecutorWithoutSplitPairTable` repeats them at `:4445-4456`. The installers are
  `install_builtin_input_carry` `:3390` and `install_carry_program` `:3662`.
- `GraphPreparedSourceSetDriver` is `graph/src/lib.rs:2565`; its `adopt_sources` is `:2629`.
- In `crates/source/src/lib.rs`: `adopt_sources` is `:1902-1938`, `held_seek` `:1018`,
  `begin_block` `:1103`, `begin_block_at` `:1115`, `begin_block_with` `:1119`. `begin_block_with`
  now takes a private `SeekClock` (`:1030-1038`).
- `PreparedRenderPlan::adopt_predecessor_plan` (gate 1's twin) is
  `crates/engine/src/realtime/plan.rs:1201`.
- `SourceControlSet::adopt_persisting` is still `crates/host-core/src/source.rs:286`.

**D1: the carry families at HEAD.** The executor now installs nine carry sections, not two:
`carry` (sources), `carry_inputs`, `carry_strips` (#1277), `carry_effects` with `carry_scratch`
(#1279-#1282), `observer_carry` (meters and observation taps, #1327), `carry_delays` (#1283),
`carry_strip_delays` and `carry_routes` (#1284). Host-core also carries spectrum captures
(#1395): the pairing is `PlanStateInventory::spectrum_pairing`, built in host-core, not in the
executor. D1 holds as a design. Its scope follows from it:
- `installed_carry_reads` reports a predecessor key for every executor section above.
- The restriction keeps the inventory rows each section reads: `sources`, `input_sections`,
  `strip_stages`, `effect_lanes`, `meters`, `observations`, `delay_lines`, `strip_delays` and
  `live_routes`. It keeps the `spectrum` rows that A's own `spectrum_pairing` names.
- The scalar fields keep P0's values (`plan_identity`, `sample_rate_hz`, `quantum_frames`,
  `spectrum_collection`). `spectrum_pairing` and `restarted` describe A against P0, not P0, so
  the restricted base holds them empty.
- Gate 2 covers each family above, one row each, not only sources and input moves. Its test
  value covers the same defect for every family.
- The restart set still works across a withdrawn plan. A strip A restarted has no row in the
  base, but the joins come from the committed models. `resolve_strip_carry` (`prepare.rs:4799`)
  and `resolve_effect_carry` (`:5197`) mark a strip restarted when its predecessor row is
  missing. So B lists it in `restarted_strips()` and #1324 ducks it.
- Size: nine families make this slice larger than its brief assumed. The rows are mechanical;
  the attempt budget stands.

**D2-D6 hold** against the code. Decision 17 does not touch them: the base and donation are
host-core internals, with no host surface.
- D3's donor is an `UnadoptedCandidate` (#1343, `crates/engine/src/realtime/plan_exchange.rs:251`).
  It gives `plan()` (shared) and `into_plan()`. `check_donation` needs only `plan()`.
  `apply_donation` needs `&mut` on the donor; #1310 calls it after the commit, when A is never
  republished, so `into_plan()` serves. No `plan_exchange.rs` edit is needed.
- D5's flag is one render-side store in `begin_block_with`. The non-goal "Any render-thread
  change" means no change beyond that store. `realtime-policy` covers it.

**Dependencies on GitHub.**
- #1277 is closed, and so are #1279-#1284, #1311 and #1343.
- #1325 (open) is named in gate 3 for scope only. It is not a dependency.
- Downstream: #1310 (batch 15a), #1358 and #1397 (open, not in batch 15).

**Authorized paths.** Unchanged. The spectrum rows are read in `prepare.rs`, so
`crates/host-core/src/spectrum.rs` is not needed.

**Gates and commands.** Every named package, feature and script exists: host-core features
`control-provider` and `test-support`, graph `test-support`, `scripts/check-graph-policy.sh`,
`scripts/check-host-core-policy.sh`, `tools/realtime-policy`. `withdrawn_successor.rs` is new here.

**Realtime-path edits.** None under `crates/engine/src/realtime`. D5 adds one `bool` store to the
source consumer's block entry (`crates/source/src/lib.rs`, render-side). D4's swap runs on the
control thread.

## Decision record: #1344 (batch 15a)

**Shipped.**
- D1: `graph::installed_carry_reads(&PreparedRenderPlan) -> Option<GraphCarryReads>` names every
  executor field without `..` and reports a predecessor key for all nine carry sections (sources,
  input lanes, `(owner, lane)` strip stages, effect sites, `(kind, location)` observers for meters
  and spectrum, observation tap sites, delay lines, strip delay lines, live routes; joins and the
  scratch read none). `PreparedHost::carried_base` is the base inventory restricted by
  `PlanStateInventory::restricted_to` (destructures without `..`), keeping P0's identity, rate,
  quantum and capture kind, with `spectrum_pairing` and `restarted` empty. New report row
  `carried_base_retained_bytes`; new accessor `PlanStateInventory::spectrum_count`.
- D2: doc on `SuccessorBase`; no new entry point.
- D3: `check_donation(DonationSide, DonationSide) -> Result<DonationPlan, DonationError>`
  (read-only), `donate_producers`, `donate_consumers`, `apply_donation(DonationSideMut,
  DonationSideMut, DonationPlan) -> usize` in `host-core/src/source.rs`. `DonationPlan { producers,
  consumers }` holds the two halves; it borrows neither side.
- D4: `GraphPreparedSourceSetDriver::{can_swap_sources, swap_sources}` (source crate implements
  them, whole-entry swap, all or nothing); `graph::{can_donate_sources, donate_sources}`.
- D5: `PcmSourceConsumer::has_begun_block`, one `bool` store in `begin_block_with`.

**Departures.**
- The amendment's "check_donation needs only `plan()`" and "no edit under
  `crates/engine/src/realtime`" cannot both hold: no shared executor downcast existed. Added
  `PreparedPlanExecutor::as_any(&self)` (default `None`) and `PreparedRenderPlan::executor_any(&self)`
  in `crates/engine/src/realtime/plan.rs` (control-side, no render code; one attempt, accepted).
- The driver trait also gained `as_any(&self)` and `source_has_begun_block`, needed to downcast the
  donor through a shared reference and to type `DonationError::Consumed`.
- `installed_carry_reads` takes `&PreparedRenderPlan`, not `&mut`.
- Spectrum rows are kept by the observer section's installed `Spectrum` moves (D1's rule), with a
  debug assertion that they equal the predecessor entries of A's `spectrum_pairing` (the
  amendment's wording).
- A successor with no section installed gets `Some` (P0's identity, no rows), not `None`: its
  replacement still needs a base, and nothing carries.
- Gate 5's "declaration or ring-config mismatch is refused": D3 selects only sources with equal
  declaration and ring, so a mismatched `c` is not selected; the check returns an empty plan and
  nothing moves. A typed refusal would block every other donation and refuse a legitimate
  transaction that redeclares a source; the redeclared source's old PCM is not owed, exactly as in
  the existing carry rule (#1272 gate 3). `DonationError` is `Consumed` and `Unswappable`.
- Gate 1's "insert quality" edit does not prepare (no launch effect declares a second quality:
  `effect.quality.unsupported`); the tests replace the insert's effect (compressor to
  gate-expander) under the same insert ID. Gate 1/3/4/5 sessions have no console slot, because on
  an AVX2 target a console slot at `Backend::Simd4` fails `console.slot.unbanked` even for the
  unmodified fixture (see successor work).

**Gates** (`crates/host-core/tests/withdrawn_successor.rs`, all pass):
1. `a_successor_prepared_across_a_withdrawn_plan_renders_as_its_twin` (Simd8 and Simd4): 16
   blocks bit-identical to the twin, `a` carried (B's reads name P0's index), `b` absent,
   `restarted_strips() == [t1, t2]`; a fresh B differs.
2. `the_carried_base_holds_exactly_the_rows_the_hand_over_reads`: all ten inventory families
   non-empty in A's reads, base count equals read count per family, P0 identity, source rows equal
   P0's, effect lanes and observations strictly fewer than P0's; a first plan has `None`.
3. `a_source_absent_from_the_running_plan_is_never_carried_from_the_withdrawn_one`.
4. `a_donated_ring_plays_its_acked_pcm_and_held_seek` (Simd8 and Simd4): one pair, apply returns 1,
   bit-identical to the twin; without the donation it differs; every rendered block makes zero
   allocator calls (`bench_support::alloc` thread counters).
5. `a_refused_or_mismatched_donation_moves_nothing`.
6. `cargo test --locked -p host-core --all-features` (41 targets ok), `-p graph --features
   test-support`, `-p source`, `-p host-web --all-features`: pass. `-p capi`: passes with the
   mechanical one-line fix below applied (the coordinator later committed it; see below). `audit capi`: `pcm_digest
   bf7dcedd1d92621f`, 0 violations. `realtime-policy`: ok, 150 regions (none added).
   `check-graph-policy.sh`, `check-host-core-policy.sh`, `check-workspace-policy.sh`, fmt, clippy
   on `engine`, `graph`, `source`, `host-core` (`--all-targets --all-features -D warnings`): pass.

**Edits outside the original authorized paths, and who accepted them.**
- `crates/engine/src/realtime/plan.rs`: the read-only `PreparedPlanExecutor::as_any` and
  `PreparedRenderPlan::executor_any` (control-side, no render code); one attempt, accepted by root
  on 2026-10-16.
- `crates/host-core/src/lib.rs` (re-exports) and `crates/host-core/src/render_session.rs` (the
  `carried_base` field): mechanical.
- `crates/capi/tests/resource_lifecycle.rs`: the coordinator added the `carried_base,`
  destructure and `assert!(carried_base.is_none())` (mechanical). This corrects the attempt's
  record, which said the file was not edited: the attempt did not edit it, the coordinator did.

**Review fold (batch 15a).**
- M3, gate 2: the session now has track `t` and source `a`, which sort first, and A removes both,
  so A's keys are not P0's; A also redeclares `c` and replaces `t0000`'s insert. Under live input
  lanes an input-section or strip-stage value edit carries and retargets, so those families (and
  meters, delay lines, strip delays, live routes) restart by removal only; the plan's one spectrum
  capture carries. The test compares sets, not counts, through two test-support views
  (`test_only_inventory_rows`, `test_only_carry_read_keys` in `prepare.rs`): base keys equal read
  keys, and base owners equal the owners both plans hold less those A restarted. Red evidence:
  `restricted_to` returning the full inventory fails "base keys equal read keys";
  `installed_carry_reads` reporting successor keys fails for each section on its own (sources,
  delay lines and strip delays on the owner set; input lanes, strip lanes, meters and live
  routes on the key set; effect sites and observation taps on "base rows are P0 rows"; spectrum
  on prepare's own pairing assertion).
- M3, gate 1: gate 1's plans are now prepared with live controls, so inserts bank with live
  lanes and can carry. `t3` carries a transient shaper (audible state, its own bank), and every
  input section carries its high-pass state. Red evidence: dropping the effect-lane or
  input-section rows from the restriction fails gate 1. The reviewer's "an unrestricted base is
  red" assertion is not added. That assertion and root's item 7 cannot both hold: the restricted
  base and P0's full inventory differ only in rows A restarted or removed, and an owner from
  those rows carries from P0 only when B reverts it to its P0 form. So the assertion would lock in
  the behaviour that root ruled a defect (an owner or source that A restarts and B reverts starts
  fresh in B). Item 7 is not small, and it is not coded here, for three reasons. (1) The fix
  removes the restriction: the control plane passes P0's inventory as the base, which reverses D1
  and D2. (2) It needs a P0 producer pass before the donor pass: for a source that A redeclared and
  B reverts, the donor's producer feeds A's ring and not the P0 ring that B reads. (3) It must
  decide where host submits made between A's commit and B's swap go. Successor placeholder (the
  coordinator writes it): *B carries an owner or source it keeps unchanged from P0 from P0's state*.
- m1: `donate_producers` now moves the donor's producer into the successor and leaves the donor
  vacant, so a later submit or seek to the donor is refused (`Vacated`) and never acked. The
  successor's own fresh producer is dropped. `check_donation` refuses a successor producer that
  has already been fed with the new `DonationError::SuccessorFed`, so that drop never loses acked
  PCM. `apply_donation` first re-reads both halves' preconditions (producer pairs resolve,
  `can_donate_sources`) and moves nothing if either fails. It then runs the consumer swap (all or
  nothing) and, only if that moved, the producers, so a donated ring and its producer are never
  separated. Test: `a_donation_never_separates_an_acked_ring_from_its_producer`, and gate 4 now
  asserts the donor's `Vacated`. Red evidence: restoring the producer swap fails gate 4; removing
  the fed check fails the new test; the old producers-first order fails the new test.
- m2: `crates/source` tests `a_source_swap_refuses_every_unswappable_pair_and_moves_nothing`
  (every `swappable` refusal: either index out of range, either entry vacant, channel counts, quanta,
  either index repeated, a foreign driver; nothing moves) and
  `a_source_swap_carries_a_prepared_seek_into_the_next_block`. Each `swappable` condition removed
  on its own and the `pending_generation_change` OR removed are each red. The OR stays set-wide.
  A comment at the OR records why this is safe. It can only over-report one block's
  generation-change validity and never hides a real change. It cannot be reached in production,
  because a donor never had `prepare_source_seek`. A per-entry flag would add reads on render.

**Open, outside this slice's paths (resolved).** `crates/capi/tests/resource_lifecycle.rs:810`
destructures `PreparedHost` exhaustively and needed `carried_base,` plus
`assert!(carried_base.is_none())`. The attempt did not edit it (batch 15a rule for `crates/capi`).
The coordinator added it, as recorded above.

**Review fold 2 (batch 15a).**
- *m-2.* A release-build probe fed B's producer between `check_donation` and `apply_donation` and
  lost the acked chunk: `producers_resolve` did not re-read `unfed`. It now requires the
  successor's producer to be unfed, so the apply refuses and moves nothing. `donate_producers` and
  `donate_consumers` are `pub(crate)` (no caller outside `host-core`) and no longer re-exported.
  Test `a_successor_fed_between_the_check_and_the_apply_moves_nothing`: apply returns 0, both
  sides keep their rings, B's producer continues after its acked chunk and A's is not vacated.
  Red with the `unfed` re-read removed: the `debug_assert!` in `donate_producers` (debug) and
  "a fed successor refuses the apply" (release).
- *m-3.* `a_seek_or_an_empty_end_of_region_on_the_successor_refuses_the_donation`: a seek alone
  (generation 2) and a seek to the region's end closed by a zero-frame end-of-region chunk each
  give `SuccessorFed`. Red with `unfed` reading only the written frames ("seek only"). A
  zero-frame end-of-region chunk at generation 1 is not reachable: the validator refuses a
  zero-frame source and the ring refuses the chunk as non-contiguous, so the chunk follows a seek.
  The removed pre-check is caught by the m-2 test above.
- *m-4.* The doc of `a_donation_never_separates_an_acked_ring_from_its_producer` no longer claims
  the old producer swap; it names gate 4's `Vacated` assert
  (`a_donated_ring_plays_its_acked_pcm_and_held_seek`) as the catch.
- *Reverted-source guard (root ruling of 2026-10-16).* `host_core::drops_acked_reverted_source`
  (`source.rs`, beside `check_donation`, reusing `unfed`) reports a source the donor redeclared
  from the running plan, fed since, that the successor declares as the running plan does: the one
  case the donation cannot move and the acked chunks would drop. #1310's rebuild refuses it as
  `BACKPRESSURE` (see #1310's fold 2); #1698 lifts it.
- *Fold 3 (M-A, root ruling of 2026-10-16).* The guard is widened and renamed
  `host_core::drops_acked_fresh_source(donor side, &DonationPlan)`: any fed ring the donor created
  (`can_prepare_source_seek` on the donor, as in `check_donation`) that the donation does not move
  is refused, so B reverting, redeclaring a third way or removing the source, and A adding it, are
  all covered; a source the donor carried from P0 is not. Unit test
  `withdrawn_successor::the_guard_refuses_exactly_a_fed_fresh_ring_the_donation_leaves_behind`;
  see #1310's fold 3.
