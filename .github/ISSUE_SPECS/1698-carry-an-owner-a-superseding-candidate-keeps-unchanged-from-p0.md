# Carry an owner a superseding candidate keeps unchanged from P0 from P0's state

Successor of #1344 (prepare a successor across a withdrawn candidate plan) and #1310 (supersede an
unadopted candidate plan by compare and swap), both batch 15a. Found in the batch 15a review; root
ruling 2026-10-16. Code anchors verified on the batch 15a fold branch `codex/controls-b15a-fold`
at `804fe6939` (committed tree; the branch has uncommitted edits in `control.rs` while this is
written, so the implementer re-reads the line numbers). Issue number: assigned at filing (written
here as #1698). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review.

## Product outcome

An edit followed quickly by its undo is not audible. When the running plan P0 has a committed
candidate A that render has not adopted, and a later transaction commits B that puts an owner X (a
strip, an effect, a meter, an observation tap or another owner of plan state) back to its P0 form,
B carries X from P0, as a direct P0 -> B swap would. Before this issue X started fresh in B
whenever A restarted or removed it. Sources stay under the host contract (amended by the
coordinator ruling of 2026-10-16, confirmed by root; see the decision record): a source A added,
redeclared or removed starts fresh in B unless B keeps it as A declares it.

## Context

- **The defect.** #1344 D2 prepares B with `SuccessorBase { inventory: A.carried_base, committed:
  P0's committed model }`. `carried_base` is P0's inventory restricted to the rows A's installed
  hand-over reads (`PlanStateInventory::restricted_to`, `crates/host-core/src/prepare.rs:1086`;
  `PreparedHost::carried_base`, `:689`). So an owner carries from P0 only if A carried it.
  Anything A restarted, added or removed is fresh in B (the doc at `prepare.rs:1188`-`:1196`
  states this as the rule). For an X that A restarted or removed and B reverts to its P0 form,
  that is wrong: P0 still holds X's state, because A was never adopted, and the combined twin
  (P0 -> B with one transaction) carries it.
- **Where B's base is built.** `SessionState`'s structural commit, `let base = if superseding`
  (`crates/control-plane/src/control.rs:2044`), reads A's `carried_base` at `:2053` and P0's kept
  model.
- **Where producers move.** After the commit, the superseding branch applies the checked donation
  (`:2325`-`:2347`, #1344 D3: rings A created fresh that B keeps) and then
  `candidate_provider.sources.adopt_persisting(&mut donor.sources)` (`:2349`) from A's set. After
  the swap is published, `holder.sources.adopt_persisting(&mut donor.sources)` (`:2461`) returns
  to P0's epoch every producer A holds that P0 holds vacant, before A drops.
  `SourceControlSet::adopt_persisting` (`crates/host-core/src/source.rs:291`) fills each vacant
  entry from the predecessor's entry of the same ID, keyed by ID, allocation-free.
- **Host submits between A's commit and B's swap.** A restarted X with a fresh ring. The host's
  chunks for X after A's commit went to A's X ring, not P0's. If B carries X's ring from P0, A's
  ring and its acked chunks drop with A, and P0's ring underruns for those frames. That breaks
  the acked-batch rule (AGENTS.md: can an ack ever precede a drop?).
- **Tests to extend.** `crates/host-core/tests/withdrawn_successor.rs` (P0, A and B models at
  `:147`, `:164`, `:178`; `twin_run`, `:306`; `backends`, `:184`) and
  `crates/capi/src/runtime/supersede_tests.rs`
  (`a_superseding_replacement_renders_as_one_combined_transaction`, `:157`;
  `a_removed_sources_producer_returns_to_the_running_epoch`, `:554`).

## Decisions frozen for this slice

- **D1. No ack before a drop (root, 2026-10-16).** Every chunk a host submit acks for X between
  A's commit and B's swap is either read by B or refused before its ack. No acked chunk drops
  with A. The implementer proposes the mechanism (B reads A's X ring for those frames, or the
  submit to a ring that a later supersession can drop is refused with typed backpressure before
  its ack, or another shape) and the coordinator rules; the rule itself is not open.

## Decisions to take (the implementer proposes, the coordinator rules)

- **Amend #1344 D1 and D2.** The base for an owner or source that B keeps unchanged from P0 is
  P0's inventory row, not A's restricted base. Decide whether `PreparedHost::carried_base` and
  `PlanStateInventory::restricted_to` stay (for example as the base of what A changed and B keeps)
  or are replaced by P0's inventory with a rule for the rows A changed. #1344's doc at
  `prepare.rs:1188`-`:1196` is amended with the decision.
- **A P0 producer pass.** Before the donation pass (`control.rs:2325`-`:2347`), add one
  `adopt_persisting`-shaped pass from P0's source set, so a reverted X takes P0's producer and
  never A's fresh one. State its order against the donation and against the return pass
  (`:2461`), and prove it allocation-free.
- **Where submits for X go between A's commit and B's swap**, under D1.

## Deliverables

1. The base and the producer pass in `crates/host-core/src/{prepare.rs,source.rs}` and
   `crates/control-plane/src/control.rs`.
2. D1's mechanism, wherever the ruling places it.
3. The gates below; #1344's and #1310's spec text amended by appending, not rewritten.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/src/source.rs`,
  `crates/host-core/tests/withdrawn_successor.rs`
- `crates/control-plane/src/control.rs` and its tests
- `crates/capi/src/runtime/supersede_tests.rs`
- The source ring's submit path (`crates/source/src/lib.rs`) only if the D1 ruling puts the
  refusal there
- This spec file's decision record; appended amendments in #1344's and #1310's specs

## Non-goals

- No change to the donation of rings A created fresh that B also creates fresh (#1344 D3).
- No change to what a direct P0 -> successor swap carries.
- No new C export or protocol message.

## Objective gates

1. **Revert carries from P0.** A removes X, B reverts X; and A restarts X, B reverts X; for X a
   strip owner and for X a source. X carries from P0: B's rendered blocks are bit-identical to the
   combined twin's (P0 -> B with one transaction, fed the same chunks), at both lane widths
   (`Backend::VECTOR`), in `crates/host-core/tests/withdrawn_successor.rs` and in the capi
   supersede tests. Each case is red on the base before the fix (recorded in the PR).
2. **No acked chunk is lost (D1).** A host submits chunks for X between A's commit and B's swap.
   Every submit either returns a typed refusal or is acked, and every acked chunk's frames are
   read by B: B's output equals the twin fed exactly the acked chunks, and the count of acked
   chunks no plan reads is 0.
3. **The swap block allocates nothing.** `allocations == 0` across B's adoption block, measured
   with `bench_support::alloc`'s thread-scoped counters after warm-up.
4. **Commands:** `cargo test --locked -p host-core --all-features`,
   `cargo test --locked -p control-plane --features control-plane/test-support`,
   `cargo test --locked -p capi`, `cargo run --locked --release -q -p realtime-policy`;
   clippy `-D warnings` and fmt on the touched crates; `bash scripts/check-workspace-policy.sh`.

## Test value

- Gate 1 turns red if a reverted owner or source starts fresh in B, or takes A's fresh producer.
- Gate 2 turns red if a chunk acked into A's ring drops with A.
- Gate 3 turns red if the producer pass or the D1 mechanism allocates at the swap.

- **Lifts the batch 15a guard, reverted case only.** Batch 15a (fold 2, widened in fold 3 by root
  ruling M-A of 2026-10-16) refuses with `BACKPRESSURE` a supersession B that leaves behind a ring
  A created (A added or redeclared the source) whose producer has acked anything
  (`host_core::drops_acked_fresh_source`, called in `SessionState::rebuild` right after
  `check_donation`). #1698 lifts only case (a), where B declares X as P0 does, once D1's mechanism
  lets B read A's acked chunks: that case leaves the guard and its rows in
  `supersede_tests::a_supersession_that_would_drop_acked_pcm_on_a_fresh_source_is_backpressure`
  and `control_parity::a_supersession_that_would_drop_acked_pcm_is_backpressure_alike_on_both_hosts`
  are replaced by gate 2. Cases (b) B redeclares X a third way, (c) B removes X, and (d) A added X
  and B removes or redeclares it stay refused until an issue gives B a way to read those chunks or
  the owner rules that a removed source's chunks may drop.

## Dependencies

- #1344 and #1310 (batch 15a): the withdrawn-candidate base, the donation and the supersession.
- Root ruling 2026-10-16: this is the first K slice of batch 15b, ahead of the rest of the source
  chain.

## Size

Half a day.

## Decision record: #1698 (batch 15b)

**Base decision (amends #1344 D1 and D2).** Across a withdrawn candidate A, B's base inventory is
P0's own inventory, the inventory of the epoch A succeeds (`kept_holder().inventory`), through
`PlanStateInventory::across_withdrawn(&A.inventory, A.plan)` (`crates/host-core/src/prepare.rs`).
Every owner row is P0's, so an owner B keeps unchanged from P0 carries from P0 as a direct P0 -> B
swap carries it, also when A restarted or removed it. `committed` stays P0's kept model. The only
filter is on sources: a P0 source row is kept only when A carries that source from P0 (A's plan
holds it vacant, `can_prepare_source_seek` false on A's index). The filter reads A's ring, not A's
model, so a chain stays right: a source a candidate restarted is fresh in it, whatever its
declaration. `PreparedHost::carried_base`, `PlanStateInventory::restricted_to`, the report row
`carried_base_retained_bytes`, `ProviderEpoch::carried_base`, the capi epoch row term, the
test-support views `test_only_inventory_rows` / `test_only_carry_read_keys`, and
`graph::installed_carry_reads` with `GraphCarryReads` (no production caller left) are removed.
The new base is built once per supersession on the control thread, before the commit; it is not
kept.

**Why sources do not carry from P0.** The host contract, `docs/CONTROL_PROTOCOL_SEMANTICS.md:23`:
"a source whose declaration the transaction changed restarts the same way, in a new ring at
generation 1, frame 0, with the PCM accepted for it before the commit discarded with its plan, so
the host restarts its feed." Submits and seeks address the newest committed session
(`SessionState::submit_unserviced`, `newest_providers_mut`). After A's commit the host feeds A's
X; B declares X differently from A, so a host that follows the contract restarts X at generation
1, frame 0. P0's X producer requires P0's generation and next frame (`validate_submission_metadata`
in `crates/source/src/lib.rs`), so carrying P0's X would refuse that feed (`StaleGeneration` or
`NonContiguous`). The host could follow only through a new signal, which the spec's non-goals
forbid. B reading A's acked chunks (D1 option i) is also wrong: they are A's declaration's
content. So D1 needs no new mechanism: `host_core::drops_acked_fresh_source` stays unchanged for
all four cases (a)-(d), and the spec's line "Lifts the batch 15a guard, reverted case only" does
not hold. The guard's capi and parity rows stay. The producer passes are unchanged: every source B
prepares vacant is one A carried from P0, whose producer A's commit moved into A's set, so no P0
producer pass is needed (the order stays: donation, `adopt_persisting` from A, the return pass
after publication).

**Gates (the new list; gate 2 is dropped by the ruling).**
1. Owners carry from P0, bit-identical to the combined twin P0 -> B:
   - `crates/host-core/tests/withdrawn_successor.rs`
     `an_owner_a_restarts_or_removes_and_b_reverts_carries_from_p0` (A replaces `t1`'s insert, or
     removes track `t3`; B reverts it and replaces `t2`'s; Simd8 and Simd4; `restarted_strips()`
     is `[t2]`; 16 blocks equal, zero allocator calls per block).
   - capi `supersede_tests::an_owner_the_candidate_restarts_and_the_replacement_reverts_carries_from_p0`
     and `..._removes_and_the_replacement_restores_carries_from_p0` (9 blocks equal the combined
     transaction; build target's lane width only, as every capi test).
   - Red on the base before the fix (recorded here): host-core restart case `restarted_strips`
     `["t1", "t2"]` / `["t2", "t3"]`, and bits differ at block 0 in both cases at both widths;
     capi both cases differ at block 1.
2. Sources stay as before: `a_source_b_reverts_is_fresh_and_a_source_a_carries_still_carries` (A
   redeclares or removes `b`, B reverts it: B's `b` owns a fresh ring and takes generation 1 at
   frame 0; `a`, carried by A, is vacant in B and its producer moves from A). Red with the source
   filter removed (P0's full inventory): the `b` freshness assert.
3. Tap across a restart: host-web `a_subscription_on_a_candidate_that_restarted_the_tap_publishes_
   on_a_replacement_that_reverts_it` (A changes `comp`'s link mode, a subscription is acked
   against A, B puts `comp` back; B's tables hold the subscription and windows publish). Red with
   `observation_forwards` emptied (B now carries P0's unarmed tap). No spectrum test: a spectrum
   capture carries by target and channels only, so A can restart one only by removing its target,
   and the browser refuses that preparation (`RESULT_COMPILE_REJECTED`, probe not kept), so the
   pairing B gets is P0's in both the old and the new base.
4. Gate 3 (allocations) runs unchanged: every rendered block in the host-core tests above asserts
   zero allocator calls.
5. Deleted, superseded: `the_carried_base_holds_exactly_the_rows_the_hand_over_reads` and its
   rich-session helpers. Gate 1's and gate 3's `installed_carry_reads` asserts became
   `can_prepare_source_seek` asserts on B's index.

**Measured.** Reference session `capi_retained_bytes` 168,810 -> 167,514 (x86-64, -1,296 = one
216-byte `Option<PlanStateInventory>` in each of six provider epochs); the 175,296 ceiling stays.
No ABI change: the row was a control-plane epoch term, not a header field.
Amendment (batch 15b fold): after #1285's `input_floors` and `raised_arrivals` merged the row
measures 167,754 (+240); the ceiling stays.

**Edits outside the listed paths (authorized by the ruling or mechanical).**
`crates/graph/src/lib.rs` (deletion), `crates/host-core/src/{lib.rs,render_session.rs}`,
`crates/control-plane/src/compile.rs`, `crates/capi/tests/resource_lifecycle.rs` (destructure,
term, re-pin note), `hosts/host-web/src/tests.rs` (the tap test),
`crates/capi/include/miso_engine_v1.h` (comment only: the "one case is not covered yet" sentence
now states the owner carry and the source rule), `docs/CONTROL_PROTOCOL_SEMANTICS.md`,
`docs/C_ABI_V1_QUALIFICATION.md`. No edit under `crates/engine/src/realtime/`.
