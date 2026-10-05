# Supersede an unadopted candidate plan by compare-and-swap

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A host submits a structural transaction while an earlier one's candidate plan has not been
adopted by render yet, including on a paused host, and it commits. The control plane withdraws
the unadopted candidate, prepares one successor that carries both transactions, and publishes it.
Repeated structural edits on a paused host all commit; none returns `BACKPRESSURE` because a
candidate is pending. The watermark reports the displaced revisions as `superseded` when the
successor is adopted.

## Context

- **The refusal.** `command` refuses every structural transaction while a candidate is pending:
  `if !self.pending_providers.is_empty() { return Err(CommandError::Backpressure) }`
  (`crates/capi/src/runtime/control.rs:959-960`). A paused host never adopts, so every later
  structural edit is refused for as long as it stays paused.
- **Capacities and admission.** *Size the C ABI's plan capacities and resource admission for a
  superseding candidate* (#1398) deletes the #1042 lag check (`:897`), makes the full publication
  or retirement (`:961-974`) and full report table or pending list (`:992-996`) refusals
  impossible, and sums every held plan and model in admission. This slice relies on its
  capacities: report rows 4, pending providers 2, retirement capacity 3, retired providers 2.
- **The exchange can take a candidate back** after *Let the control thread withdraw an unadopted
  candidate plan* (#1343): `PlanPublisher::withdraw() -> Withdrawal::{Withdrawn, Taken, Nothing}`,
  and after #1311 also `Returned`. `Taken` means render adopted it.
- **Base and producers.** A candidate is prepared against the newest epoch's inventory and the
  current committed model (`control.rs:907-914`, `SuccessorBase`,
  `crates/host-core/src/prepare.rs:641`). The carry program names its predecessor's plan identity
  (`prepare.rs:1296-1297`), so a candidate prepared against an unadopted plan cannot carry from the
  plan render actually runs. Persisting source producers move to the candidate after the commit
  (`adopt_persisting`, `control.rs:1019`; `crates/host-core/src/source.rs:286`).
- **Host-fed state lives in a candidate.** From the commit on, submissions and seeks address the
  newest committed session (`crates/capi/include/miso_engine_v1.h`, "Sources across a structural
  transaction"). A source the candidate added already holds PCM, a generation and possibly a held
  `seek_at` in the candidate's ring (`PcmSourceConsumer::held_seek`,
  `crates/source/src/lib.rs:1018`).
- **The predecessor's model is dropped at commit.** `SessionStore::commit_prepared` overwrites the
  compiled session (`crates/protocol/src/model.rs:966`).
- **The watermark** (#1314 D1, D5) carries a `superseded` count beside each candidate's revision in
  its mailbox cell. Render adds it once, at that candidate's first advance. #1314 always writes 0;
  this slice is the producer.

## Decisions frozen for this slice

- **D1. Withdraw first, then prepare.** When a transaction needs a rebuild and a candidate A is
  the newest epoch, the control plane withdraws A before preparing. Outcomes:
  - **Withdrawn**, or **Returned** (#1311): A is back unrendered and control-owned, and render
    still runs P0. The new candidate B is prepared against P0, with A as a donor (D2).
  - **Taken:** render adopted A. A is the base, exactly as today's path with A newest.
  - **Nothing:** no candidate is pending; today's path.

  This replaces the plan's "CAS, then re-target if render took it": B's preparation must know its
  render predecessor (the carry program names it), and A's host-fed rings can move only while
  the control thread owns A.
- **D2. Base across a withdrawn candidate.** B is prepared with `SuccessorBase { inventory:
  A.carried_base, committed: P0's kept model (D3) }` (*Prepare a successor across a withdrawn
  candidate plan*, #1344 D1-D2). So an owner carries from P0 only if A carried it and B leaves it
  unchanged; anything A restarted, added or removed is fresh in B. The sources A created that B
  keeps are donated (#1344 D3), and persisting producers move from A's set into B's
  (`adopt_persisting(&mut A.sources)`; A holds P0's moved producers).
- **D3. Keep the predecessor's model.** The protocol's structural commit returns the compiled
  session it displaces. The control plane keeps it in the epoch being succeeded for as long as a
  successor of that epoch is pending, and drops it when that epoch retires. In the withdrawn case
  P0 already holds its model, so the model B's commit displaces (A's) is dropped. A live commit
  drops its displaced model as today. The kept model is a held compiled model, so both admissions
  count it (#1398 D4's "every held model" rule): three models while a successor is
  pending and a live edit is admitted.
- **D4. Every check runs while A is withdrawn, before the commit.** In order: preparation of B; the
  resource admission; the publication reservation; `check_donation(B, A)` (#1344 D3); the
  protocol's commit predicate. Nothing moves before the last one passes. On any refusal A is
  republished unchanged, with its own retirement credit, revision and `superseded` words, and the
  refusal is returned. Republishing cannot fail: the control thread is the only publisher and the
  mailbox has an `Empty` cell.
- **D5. Then, infallibly, in this order:**
  1. protocol commit;
  2. `apply_donation` with D4's checked plan (it cannot fail, #1344 D3) and the producer moves;
  3. B's `superseded` word (D6) and revision (#1314 D2);
  4. publish B;
  5. return to P0's epoch every producer A still holds that P0 holds vacant:
     `P0.sources.adopt_persisting(&mut A.sources)` (`crates/host-core/src/source.rs:286`). A's
     commit moved P0's persisting producers into A's set (`crates/capi/src/runtime/control.rs:1019`);
     step 2 moved into B the ones B keeps, so what remains for P0 are the producers of sources P0
     still renders and B removed. Infallible and allocation-free; keyed by source ID;
  6. drop A's plan, provider epoch and credit on the control thread; remove A's report row.

  B is published only after it owns the donated rings, because render may adopt it at once. Step 5
  runs before step 6 because P0's consumers keep rendering until B is adopted: a producer dropped
  with A would leave a source P0 still plays with no producer, and *Remove a strip in two phases:
  ramp out, then a scheduled swap* (#1325) D4 routes phase-1 submits for a removed source to it.
- **D6. This slice owns `SUPERSEDED`.** Let `r_P0` be the revision last written to P0's cell (the
  control plane keeps it per provider epoch) and `r_B` B's own revision. Every revision after `r_P0`
  and before `r_B` was committed onto A or onto a candidate A had superseded, and its content is in
  B. The control plane stores `r_B - 1 - r_P0` as B's `superseded` word before publishing B. When
  render adopts B, the watermark advances with `SUPERSEDED` set, `superseded` grows by that count,
  and `exact` takes the rest (#1314 D5).
- **D7. Live edits, submits and seeks** keep addressing the newest epoch, which is B after D5.
  A live edit made while A was pending is in the committed model B is prepared from.
- **D8. The acked-batch question: can an ack ever precede a drop? No.** A's revision was acked
  and stays committed: its edits are in the model B is compiled from, and its host-fed sources move
  into B (D2). Every way the donation could fail is checked before the commit (D4), so the
  post-commit apply cannot lose a ring, and a producer of a source P0 still renders returns to P0's
epoch before A drops (D5 step 5). If B is refused, A is republished untouched (D4). B is acked
  only after every fallible step. A's revisions complete when B is adopted, reported as
  `superseded` (D6).

## Deliverables

1. The supersession path in the control plane's `command`, with D3's model retention and D6's
   count.
2. The protocol change of D3.
3. Header text (`miso_engine_v1.h`, after "Sources across a structural transaction"): a
   transaction submitted while an earlier replacement is unadopted commits and supersedes it; a
   donated source keeps playing; the displaced revisions complete as `superseded`.
4. `docs/CONTROL_PROTOCOL_SEMANTICS.md` and `docs/C_ABI_V1_QUALIFICATION.md`: the same, briefly.

## Authorized paths

- `crates/control-plane/src/` (the moved `control.rs` and `compile.rs`).
- `crates/protocol/src/model.rs`, `crates/protocol/src/controller.rs` (D3 only).
- `crates/capi/include/miso_engine_v1.h` (prose only), capi tests.
- `docs/CONTROL_PROTOCOL_SEMANTICS.md`, `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- The exchange mailbox (#1343), the host-core base and donation functions (#1344), the watermark
  (#1314), and the capacities and admission terms (#1398).
- Scheduled or exact-sample adoption (#1311). Superseding a running catch-up (#1357).
- The browser (it adopts the control plane in #1332).

## Objective gates

1. **Supersession equals one combined transaction (new capi test).** Compile, render one block.
   Without rendering, submit T1 (add a source `s` and insert an effect on track `t1`), T2 (change
   `t2`'s insert quality), T3 (remove `t3`'s insert). Each returns `RESULT_OK`. Feed `s` and every
   persisting source; render 8 blocks. The PCM is bit-identical to a twin that submits T1∪T2∪T3
   as one transaction at the same point. The watermark after the adoption block reports T3's
   revision with `EXACT | SUPERSEDED`, `superseded_count` 2 and `exact_count` 1 more than before.
   *Red if B carries from A's identity while P0 runs, if the base model is the current one instead
   of P0's, if any of T1-T3 refuses, or if the superseded count is off.*
2. **A donated source keeps its PCM and its held seek (new capi test).** T1 adds `s`; submit 4
   blocks of PCM for `s` and `seek_at(s, 2, 0, A)` with `A` four blocks ahead; T2 changes another
   track's insert; render 8 blocks. `s` is heard from `A` exactly as in a twin where T1 was adopted
   before T2. *Red if B allocates a fresh ring for `s`, dropping acked PCM and the held seek.*
3. **A refused successor restores the candidate (new capi test).** T1 pending; T2 is refused by
   preparation (a cap one byte below its need). The response is the compile rejection, the
   revision is T1's, and after one render the output equals the T1-only twin. Repeat with each
   test fault phase of `TestStructuralFaultPhase` armed during T2, including one between
   `check_donation` and the commit: owners and credits balance as in
   `every_structural_phase_and_ordered_dual_fault_preserves_owners_and_credits`
   (`crates/capi/src/runtime/tests.rs:2094`), which this extends to the withdrawn path.
4. **Three-plan ceiling (new capi test).** A `maximum_graph_session_plus_plan_bytes` that fits two
   plans and their models but not three: T1 pending, T2 returns `RESULT_COMPILE_REJECTED` with
   `graph.resource.limit`, revision unchanged, and after one render the output equals the T1-only
   twin (A was republished). After that render (A adopted, a two-plan peak) T2 again returns
   `RESULT_OK`.
5. **Concurrent render (extend `crates/capi/tests/plan_swap_race.rs`).** A render thread and a
   control thread submitting 64 structural edits back to back: every submit returns `RESULT_OK`,
   and after quiescence the output equals the twin of the final model. The run reaches both
   `Withdrawn` and `Taken` (a `test-support` counter per outcome; both nonzero over the run). The
   harness's tolerance of a transient structural `RESULT_BACKPRESSURE` (`plan_swap_race.rs:391`)
   is removed: any structural `RESULT_BACKPRESSURE` now fails it. The source-submit tolerance at
   `:294` is a full ring and stays.
6. **A removed source's producer returns to the running epoch (new capi test).** P0 has sources
   `s` and `u`. Without rendering, T1 changes `u`'s track's insert (A persists `s`, so `s`'s
   producer moves into A's set), then T2 removes `s`'s track. Through the `test-support` owner
   counters and a `test-support` accessor on the epoch's source set: after T2 the running epoch's
   set holds `s`'s producer (not vacant), B's set has no `s`, and no producer was dropped on the
   control thread. After B's adoption `s`'s producer retires with P0's epoch as today.
7. **Superseded tests.** Delete the assertions that a second structural edit is
   `RESULT_BACKPRESSURE` while one is pending (find them under `crates/capi/`); gate 1 replaces them.
8. **Realtime.** `cargo build --locked --release -p audit -p capi && target/release/audit capi`:
   allocations, deallocations, locks, syscalls and `total_violations` 0.
9. **Workspace.** `cargo test --locked -p capi`; `cargo test --locked -p control-plane --features
   test-support`; `cargo test --locked -p protocol --features test-support`;
   `bash scripts/check-capi-abi.sh`; `bash scripts/check-protocol-control-policy.sh`;
   `bash scripts/check-realtime-policy.sh`; `bash scripts/check-workspace-policy.sh`;
   `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features
   -- -D warnings`.

## Test value

- Gate 1: a successor that carries from the wrong predecessor or compares against the wrong
  committed model, or a wrong `superseded` count. No test supersedes today, because supersession
  does not exist.
- Gate 2: a supersession that drops a withdrawn candidate's host-fed ring, which is an acked
  submission lost.
- Gate 3: a refusal path that drops or leaks the withdrawn candidate, or a donation step placed
  after the commit (the fault between check and commit must leave both plans whole).
- Gate 4: a three-plan peak admitted beyond a cap, or a refusal that loses the withdrawn candidate.
- Gate 5: a race between withdrawal and render's claim; judged by reaching both outcomes.
- Gate 6: a supersession that drops A with the producers B did not take (step 5 missing or
  after step 6) leaves the running epoch's entry for `s` vacant, so a source P0 still renders has
  no producer. Gates 1-5 do not see it: none removes a source that A persisted.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Let the control thread withdraw an unadopted candidate plan* (#1343).
- *Prepare a successor across a withdrawn candidate plan* (#1344).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Size the C ABI's plan capacities and resource admission for a superseding candidate* (#1398).
