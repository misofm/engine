# Carry an owner or source a superseding candidate keeps unchanged from P0 from P0's state

Successor of #1344 (prepare a successor across a withdrawn candidate plan) and #1310 (supersede an
unadopted candidate plan by compare and swap), both batch 15a. Found in the batch 15a review; root
ruling 2026-10-16. Code anchors verified on the batch 15a fold branch `codex/controls-b15a-fold`
at `804fe6939` (committed tree; the branch has uncommitted edits in `control.rs` while this is
written, so the implementer re-reads the line numbers). Issue number: assigned at filing (written
here as TBD42). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review.

## Product outcome

An edit followed quickly by its undo is not audible. When the running plan P0 has a committed
candidate A that render has not adopted, and a later transaction commits B that puts an owner or a
source X back to its P0 form, B carries X from P0, as a direct P0 -> B swap would. Today X starts
fresh in B whenever A restarted or removed it.

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

- **Lifts the batch 15a guard.** Batch 15a fold 2 (root ruling 2026-10-16) refuses with
  `BACKPRESSURE` a supersession B that declares a source X as P0 does while A redeclared X and A's
  X producer has acked anything (`host_core::drops_acked_reverted_source`, called in
  `SessionState::rebuild` before the admission). TBD42 removes that guard and its tests
  (`supersede_tests::a_supersession_that_would_drop_acked_pcm_on_a_redeclared_source_is_
  backpressure`, `control_parity::a_supersession_that_would_drop_acked_pcm_is_backpressure_alike_
  on_both_hosts`), replacing them with gate 2, once D1's mechanism lets B read A's acked chunks.

## Dependencies

- #1344 and #1310 (batch 15a): the withdrawn-candidate base, the donation and the supersession.
- Root ruling 2026-10-16: this is the first K slice of batch 15b, ahead of the rest of the source
  chain.

## Size

Half a day.

## Decision record: TBD42

To be filled by the implementer. The base decision goes first.
