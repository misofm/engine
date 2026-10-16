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
  (`crates/control-plane/src/control.rs:1095-1096`). A paused host never adopts, so every later
  structural edit is refused for as long as it stays paused.
- **Capacities and admission.** *Size the C ABI's plan capacities and resource admission for a
  superseding candidate* (#1398) deletes the #1042 lag check (`:1033`), makes the full publication
  or retirement (`:1097-1110`) and full report table or pending list (`:1128-1132`) refusals
  impossible, and sums every held plan and model in admission. This slice relies on its
  capacities: report rows 4, pending providers 2, retirement capacity 3, retired providers 2.
- **The exchange can take a candidate back** after *Let the control thread withdraw an unadopted
  candidate plan* (#1343): `PlanPublisher::withdraw() -> Withdrawal::{Withdrawn, Taken, Nothing}`.
  `Taken` means render adopted it. *Adopt a successor plan no earlier than a scheduled sample*
  (#1311) keeps these three outcomes: render adopts in the same step as it claims, so no
  candidate is ever in flight or handed back.
- **Base and producers.** A candidate is prepared against the newest epoch's inventory and the
  current committed model (`control.rs:1043-1050`, `SuccessorBase`,
  `crates/host-core/src/prepare.rs:641`). The carry program names its predecessor's plan identity
  (`prepare.rs:1296-1297`), so a candidate prepared against an unadopted plan cannot carry from the
  plan render actually runs. Persisting source producers move to the candidate after the commit
  (`adopt_persisting`, `control.rs:1155`; `crates/host-core/src/source.rs:286`).
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
  - **Withdrawn:** A is back unrendered and control-owned, and render still runs P0. The new
    candidate B is prepared against P0, with A as a donor (D2).
  - **Taken:** render adopted A. A is the base, exactly as today's path with A newest.
  - **Nothing:** no candidate is pending; today's path.

  A is published `Next`, `NoEarlierThan(S)` or `Primed` (#1311); the outcomes and every step below
  are the same for each. A pending `Primed` warm successor (D15-8 (round-5 amendment)) is an
  ordinary pending candidate here. B is published with the adoption kind of its own
  classification, never with A's.

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
  republished unchanged, with its own retirement credit and its revision, `superseded` and
  `outcome` words (#1314 D1), and the refusal is returned. Republishing cannot fail: the control
  thread is the only publisher and the mailbox has an `Empty` cell.
- **D5. Then, infallibly, in this order:**
  1. protocol commit;
  2. `apply_donation` with D4's checked plan (it cannot fail, #1344 D3) and the producer moves;
  3. B's `superseded` word (D6) and revision (#1314 D2);
  4. publish B;
  5. return to P0's epoch every producer A still holds that P0 holds vacant:
     `P0.sources.adopt_persisting(&mut A.sources)` (`crates/host-core/src/source.rs:286`). A's
     commit moved P0's persisting producers into A's set (`crates/control-plane/src/control.rs:1155`);
     step 2 moved into B the ones B keeps, so what remains for P0 are the producers of sources P0
     still renders and B removed. Infallible and allocation-free; keyed by source ID. This covers
     a superseded `Primed` warm candidate too: its preparation moved P0's persisting producers into
     its set like any successor's;
  6. drop A's plan, provider epoch and credit on the control thread; remove A's report row. For a
     warm A this also drops the control plane's `PrimedCandidate` record of A; a B published
     `Primed` sets its own (*Classify a latency-growth edit and publish its warm successor from
     the control plane*, #1403 D4, which adds the record and this drop).

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
- Scheduled and primed adoption (#1311). The warm successor's own preparation and fallback
  (#1354, #1358).
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
   `cargo run --locked --release -q -p realtime-policy`; `bash scripts/check-workspace-policy.sh`;
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

## Amendment 1 (root, 2026-10-05)

Root ruled one revision gate per plan (*Give each plan its own revision gate and take each block's
live snapshot from it*, #1502), which lands before this issue. The mailbox no longer carries
a revision word. The decisions above keep their meaning, read through the following map.

- **A candidate's revision** is the revision on its plan's gate. The control plane publishes it
  through the gate handle in that plan's provider epoch (#1502 D6).
  - D4's "its revision ... words (#1314 D1)" on a republished A: A's plan carries its gate, so
    republication keeps A's revision by construction.
  - The `superseded` and `outcome` words stay payload words, written before publication.
- **D5 step 3** publishes B's revision on B's gate handle before step 4 publishes B, and stores
  B's `superseded` word.
- **D6's `r_P0`** is the revision last published on P0's gate. The control plane keeps it per
  provider epoch, as D6 says.
- **Live edits while A or B is pending** go to the newest epoch's producers, stamped and published
  through that epoch's gate handle (#1312 D7). So cells and gate follow one routing.
- When A is dropped (D5 step 6), its gate handle drops with its provider epoch. Writes to A that
  render never adopts are outside `live_values_superseded`'s unit (#1312 D2). The watermark
  reports their revisions as `SUPERSEDED` (D6).

## Dependencies

- *Give each plan its own revision gate and take each block's live snapshot from it*
  (#1502): the plan gate (Amendment 1).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Let the control thread withdraw an unadopted candidate plan* (#1343).
- *Prepare a successor across a withdrawn candidate plan* (#1344).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Size the C ABI's plan capacities and resource admission for a superseding candidate* (#1398).

## Amendment (controls coordinator, batch 15 check, 2026-10-15)

Checked against `e2f6cfb24`.

**Batch order (root, 2026-10-15).** Batch 15a is the controls lane: #1344, #1398, #1310, in
that order. #1324 left batch 15a (root ruling, 2026-10-15); it lands in a later batch after #1288,
#1391, #1363 and #1325, in that order. The rest of batch 15 follows. #1319 and #1323 need this
slice (root rulings B15-R4 and B15-R5 of 2026-10-15).

**Moved anchors.** Each fact in Context still holds; only the lines moved.
- The one-candidate refusal is `crates/control-plane/src/control.rs:1742-1744`. The #1042 lag
  check is `:1579-1586`. The reservation refusals are `:1751-1765` (four arms since #1343 and
  #1311). The full report table or pending list is `:1781-1788`.
- The base: `prepare_runtime` with its `SuccessorBase` literal is `control.rs:1662-1683`.
  `SuccessorBase` is `crates/host-core/src/prepare.rs:1070-1097`. The carry program names its
  predecessor at `prepare.rs:2131-2134` (and `:3067-3073`).
- The persisting producers move at `control.rs:1846-1849` (`adopt_persisting`, still
  `crates/host-core/src/source.rs:286`).
- `SessionStore::commit_prepared` is `crates/protocol/src/model.rs:1384`. The controller's
  structural commit is `commit_prepared_structural`, `crates/protocol/src/controller.rs:2260`.
- The exchange: `PlanPublisher::withdraw` `crates/engine/src/realtime/plan_exchange.rs:552`,
  `republish` `:586`, `PlanReplacementReservation::set_superseded` `:642`, `UnadoptedCandidate`
  `:251`.
- Gate 3's fault test is `crates/capi/src/runtime/tests.rs:2364`. `TestStructuralFaultPhase` is
  `control.rs:161-175`.
- Gate 5: the structural tolerance in `plan_swap_race.rs` is now `:420` (a bounded wedge window,
  `WEDGE_BLOCKS`); the source-submit tolerance is `:323`.
- Gate 7's assertions are in `crates/capi/tests/resource_lifecycle.rs` (`paused_bursts`,
  `:3311-3365`, and the test at `:3078`).

**Code notes for the implementer.**
- D3: since #1613 a value-only transaction can commit in place (`commit_prepared`'s
  `TransactionForm::InPlace`, `model.rs:1397-1400`), with no displaced compilation. A rebuild
  never does: the structural arm reads `prospective_session().compiled()` (`control.rs:1662-1667`).
  So D3's displaced model always exists on the rebuild path. A live commit while A waits swaps
  owners in A's committed model, never in P0's kept one.
- D4 and D5: `check_donation` reads A's plan through `UnadoptedCandidate::plan()`. After the
  commit A is never republished, so step 2 takes A's plan with `into_plan()`. That returns A's
  credit early, which only frees a credit. No `plan_exchange.rs` edit is needed.
- D4: `republish` panics if a newer epoch was committed after the withdrawal. B's reservation
  takes no epoch until its commit (step 4), so a refusal before the commit can republish A.
- D6: `r_P0` is the revision last published on P0's gate (Amendment 1). P0's `ProviderEpoch`
  (`control.rs:37`) holds that gate's handle, and `PlanRevisionGate::last_published`
  (`crates/engine/src/realtime/plan.rs:663`) reads it. No new field is needed.

**Decisions.** D1, D4-D7 hold against the code and decision 17. Two do not hold as written.
- **ROOT RULING (2026-10-15): B's base keeps P0's monitor overlay with P0's model, and the edit
  ramps committed after `r_P0`, merged with B's own (B wins on a shared row).** This is the
  proposal below; D2 and D3 read with it. The finding follows. The rest of B's base (D2, D3).
  D2 writes `SuccessorBase { inventory,
  committed }`. `SuccessorBase` now also has `monitor` (#1559), `edit_ramps` (#1394) and
  `observation` (#1584). `monitor.committed` is "the overlay the predecessor's cells hold"; today
  the control plane passes its one `self.monitor` (`control.rs:1677-1681`). While A waits, a solo
  edit goes to A's cells (Amendment 1), not P0's. So for B the overlay P0's cells hold is not
  `self.monitor`. `edit_ramps` is B's own transaction's. A row that only A (or a live edit while
  A waited) changed then ramps with the session's `control_smoothing` in B, and gate 1's twin
  (one combined transaction) ramps it with A's length. Proposed: D3 also keeps P0's monitor
  overlay beside P0's model, and B's `monitor.committed` is that kept overlay. The control plane
  keeps the edit ramps of every edit committed after `r_P0` and drops them with the epoch; B's
  `edit_ramps` are those merged with B's own, B's ramp winning on a row both set. `observation`
  is the host's bindings as now.
- **ROOT RULING (2026-10-15): B arms its taps from the host's bindings (#1584 D6a). A record kind
  the bindings cannot express moves to B before publication, with room checked in D4. A kind that
  fits neither stops the implementer, who reports to root.** D8 reads with this. Where the
  proposal below says "before step 4", the ruling's "before publication" governs. The finding
  follows. Records queued on A (D8). Each epoch's effect producers own the
  instance's observation queue (`EffectControlProducer`, `crates/effect-compiler/src/prepare.rs:619`).
  A record admitted between A's commit and its adoption waits in A's queue: see
  `hosts/host-web/src/tests.rs:7020-7032` ("the record is queued on the successor"). D5 step 6
  drops A with that queue, so an acked record can be lost. D8 says nothing about it. Proposed: D8
  adds that B's preparation arms each tap from the host's bindings as they stand after every
  admitted record (#1584 D6a), so B holds what a record queued on A asked for, and the record
  drops with A. The implementer shows this for each record kind the queue holds. A kind that the
  bindings do not express moves to B's matching instance queue before step 4, with room checked
  in D4. If neither holds for a kind, stop and report to root.

**The browser (non-goal).** "The browser (it adopts the control plane in #1332)" is stale. Since
#1381a the browser runs `control_plane::SessionState` in its one instance
(`hosts/host-web/src/lib.rs:6-16`). Supersession reaches it through the same code, which decision
17 C1 asks for. No browser export changes. Browser tests that expect a second structural edit to
be refused while a candidate waits are superseded with gate 7. Add `hosts/host-web/src/tests.rs`
to Authorized paths for that only, and run `cargo test --locked -p host-web --features
host-web/test-support` in gate 9.

**Dependencies on GitHub.**
- Closed: #1502, #1309, #1343, #1314, #1311, #1312.
- Open, batch 15a before this slice: #1344, #1398.
- #1403 (open, not in batch 15) adds `PrimedCandidate`; D5 step 6's drop of it is #1403's. #1325
  (open, not in batch 15) is cited only for D5's reason.
- #1482 (open) also edits `plan_exchange.rs`'s area; this slice does not edit that file.

**Gates and commands.** Every named command, package, feature and script exists
(`scripts/check-protocol-control-policy.sh`, `scripts/check-capi-abi.sh`, protocol and
control-plane `test-support`).

**Realtime-path edits.** None planned under `crates/engine/src/realtime`. The exchange's
`withdraw`, `republish` and `set_superseded` exist.

## Decision record: #1310 (batch 15a)

**Shipped.**
- D1-D6 in `crates/control-plane/src/control.rs`: the structural arm withdraws the newest pending
  candidate (`withdraw_pending_candidate`), then `rebuild` prepares, admits, reserves, checks the
  donation and the observation forwards, checks the protocol predicate and commits; a refusal
  leaves the candidate in `withdrawn` and `command` republishes it. After the commit, in D5's
  order: `apply_donation` and `adopt_persisting` from A, B's `superseded` word
  (`r_B - 1 - last_published(P0's gate)`) and revision, publication, `adopt_persisting` of A's
  remaining producers back into P0's epoch, then A's plan and epoch drop. The one-candidate
  `Backpressure` check is deleted. `Taken`/`Nothing` take the old path with A as base.
- D2/D3 (root ruling B15-Q3): `ProviderEpoch::carried_base` (from `PreparedHost::carried_base`, charged
  as a capi epoch row with `carried_base_retained_bytes`) and `ProviderEpoch::kept: KeptBase
  { model, monitor, edit_ramps }`, set on the epoch a rebuild succeeds, extended by every later
  live or superseding commit (`merged_ramps`, the later edit winning), dropped when that epoch is
  promoted out. B's base across a withdrawn A is `{ A.carried_base, kept.model, merged ramps,
  MonitorBase { kept.monitor, next, ramp } }`. `held_models()` charges every kept model in both
  admissions (`compiled_model_admission_over`); the two-model helpers are deleted.
- D3 protocol: `SessionStore::commit_prepared_displacing` and
  `CommittedCommandFrame::take_displaced` (`crates/protocol`).
- B15-Q4 (root ruling): an `ObservationSlot::queued` flag marks a tap whose record waits on an
  unadopted plan; `observation_forwards` re-queues the tap's bound state on B's matching instance
  before publication, room checked before the commit. The observation queue holds only `Observe`
  records, which the bindings express; but a tap B carries from P0 takes P0's state at the swap
  and never sees a binding, so the record moves (route 2).
- Header, `CONTROL_PROTOCOL_SEMANTICS.md`, `C_ABI_V1_QUALIFICATION.md`: deliverables 3 and 4.

**Departures and findings.**
- `crates/engine/src/realtime/plan_exchange.rs` (one attempt, accepted): the reservation owns B's
  plan, so D4's check and D5's donation need `PlanReplacementReservation::{plan, plan_mut}`
  (control side, outside every realtime-policy region). The amendment's "no edit needed" missed it.
- Found by gate 5: with A taken but not promoted, B committed on it leaves two adopted candidates
  pending; when the atomic names B, `promote_pending_provider` now retires every older pending
  epoch with the running one. Before the fix a race reached `capi.source.epoch` and
  `source.ring.vacated`.
- Gate 1: "insert quality" does not prepare (#1344); T2 replaces the insert's effect. Gate 1 adds a
  fourth track whose fader T1 moves with its own ramp length, so a base of the current model or a
  lost ramp merge is red.
- Gate 3: the cap is `maximum_effect_state_bytes` one byte below B's own; `ProtocolCommit` is in
  the fault list (returns `INTERNAL`).
- Gate 5: a new back-to-back race (`structural_edits_back_to_back_supersede_or_swap_and_never_
  backpressure`) with an unlocked control thread; the old races keep their pacing and now fail on
  any structural `BACKPRESSURE`. Typical run: 32 withdrawn, 31 taken of 64.
- Gate 7: `paused_bursts` now expects the second paused edit to commit; the swap-window, epoch,
  lifecycle, double-live oracle and pressure tests were updated (`resource_lifecycle.rs`,
  `runtime/tests.rs`). The live peak with a pending candidate gains the kept model.
- Browser gap (B15-Q4), closed in the follow-up below.

**Gates** (all pass): `supersede_tests::{a_superseding_replacement_renders_as_one_combined_
transaction, a_donated_source_keeps_its_pcm_and_its_held_seek, a_refused_replacement_restores_the_
withdrawn_candidate, a_replacement_over_the_three_plan_ceiling_is_refused_and_the_candidate_kept,
a_removed_sources_producer_returns_to_the_running_epoch}` (gates 1-4, 6; red checked for: step 5
removed, donation skipped, superseded word 0, base model current, base inventory A's, ramps not
merged); `plan_swap_race` (gate 5, four tests); `resource_lifecycle` (gate 7);
`model::tests::a_compiled_commit_returns_the_compilation_it_displaced` (D3);
`host-web structural_apply::a_structural_edit_behind_an_unadopted_candidate_supersedes_it`.

**Follow-up (coordinator, same batch).**
1. *Browser copy of forwarded records (B15-Q4).* The core keeps the records the last rebuild queued
   again (`SessionState::observation_forwarded() -> &[ObservationForward]`, set by the
   superseding commit, empty otherwise). The browser's `run_command` copies each onto the new
   plan's `PlanShape` through `note_observe`, the same note `record_observe` makes for an
   admitted record (armed bit, arm sample at the command's `next_absolute_sample`, commanded
   bit), so the hand-over keeps it; the browser decides nothing itself. Test
   `a_subscription_queued_on_a_superseded_candidate_publishes_on_its_replacement` (host-web):
   red without the browser copy (mask 0) and red without the core's re-queue (no window). No
   parity row: the C ABI admits no observation record, so it cannot reach the case.
2. *B wins on every row it writes (B15-Q3).* `session::EditRamps` now also records rows written with
   no length and targets rewritten whole (`clear_target`); `EditRamps::overlay` lays a later
   transaction over an earlier one with the later winning on every row it wrote; `get` and
   `entries` are unchanged. `merged_ramps` uses it, and the live path's merge runs for any edit
   that writes a live row (`is_untouched`). Tests: `edit_ramp::tests::an_overlay_lets_the_later_
   transaction_win_on_every_row_it_wrote` and capi `a_superseding_edit_without_a_ramp_length_
   takes_the_default_on_its_row` (red with the old merge). One transaction may not give a row two
   lengths (`control.ramp.conflict`), so that test's reference is the transaction holding T1's
   structural edit and T2.
3. *Kept monitor overlay.* `KeptBase::monitor_ramp` keeps the ramp of the latest overlay change
   committed since the epoch's last revision; a replacement that writes no monitor row renders
   the change between the kept overlay and its own with it. Test `a_solo_committed_while_a_
   candidate_waits_holds_on_its_replacement` (capi): equal to one transaction of T1, the solo and
   T2; red with the current overlay as the base and red without the kept ramp.

**Review fold (batch 15a).**
- *B1 (host difference).* A paused browser page refused the fifth structural edit with
  `BACKPRESSURE` (probe: eight add-track transactions without render gave `[OK,OK,OK,OK,6]`)
  where the C ABI commits: a superseded candidate's `PendingShape` stayed until a later adoption.
  The core now says which plans it holds (`SessionState::holds_plan_epoch`), and the browser drops
  the tables and spectrum captures of every unadopted entry whose epoch the core no longer holds,
  after each publishing command and in `reclaim_shapes` (`drop_superseded_shapes`); it decides
  nothing itself. The `PENDING_SHAPES` comment is corrected. Tests: host-web
  `structural_apply::paused_structural_edits_all_commit_and_the_newest_renders_as_a_fresh_boot`
  (eight paused edits all OK, the adopted plan holds all eight tracks, superseded count +7, equal
  to a fresh boot of the committed model from the adoption block) and parity row
  `control_parity::paused_structural_edits_commit_alike_on_both_hosts` (48 and 96 kHz); both red
  with the drop disabled (`[0,0,0,0,6,6,6,6]`; parity: `control.plan.backpressure`).
- *M1 (with #1398 D3).* Render claims A and is preempted before P0's retirement reaches control;
  C finds A `Taken` and publishes B on it; render adopts B; D finds B `Taken` with `[A, B]`
  pending and hit the D3 accounting fault. `withdraw_pending_candidate` now promotes every pending
  epoch up to the newest on `Taken`, or on an empty mailbox while one is pending (render adopts in
  publication order), so a rebuild meets at most the one candidate it may supersede; the dead
  older-epoch loop in the non-superseding branch is a `debug_assert`. New control-plane seam
  `test_hold_reclamation` (test-support) keeps displaced plans in the retirement queue. Test capi
  `supersede_tests::edits_after_render_took_two_candidates_unretired_each_commit`: two `Taken`,
  blocks bit-equal to a paced run, every owner balanced; red without the promotion (the D3
  `debug_assert`).
- *Item 2 (coordinator default, root approved).* A three-plan refusal is `BACKPRESSURE` when the
  successor fits beside the newest held plan alone and the committed model (what a retry meets
  after render adopts the candidate), and `COMPILE_REJECTED` only when it cannot fit even then.
  The second check charges the candidate's rows as prepared, so a ring it takes by donation is
  counted twice there: it errs toward the permanent refusal by at most those rings' bytes, never
  toward an admission. Tests: gate 4 now expects `Backpressure` at a cap between the two- and
  three-plan peaks and adds a cap between P0+A and A+B that gives `graph.resource.limit` before
  and after A is adopted; `runtime::tests::a_valid_edit_inside_the_swap_window_is_admitted_
  against_the_running_plan` expects `Backpressure` for the pending case (each red against the
  other behavior). Header, `C_ABI_V1_QUALIFICATION.md` and `CONTROL_PROTOCOL_SEMANTICS.md` say so.
- *m3.* Test capi `a_live_ramp_committed_while_a_candidate_waits_holds_on_its_replacement`: T1
  pending, a live `t4` fader edit with a one-sample ramp on A's lanes, T2 supersedes; equal to one
  transaction of all three; red with the live path's kept-ramps merge deleted.
- *m4.* Gate 3 adds a case whose withdrawn candidate is B (superseded word 1): a fault before the
  protocol commit republishes B, whose adoption completes T1 as superseded and T2 as exact; a
  retried T3 completes two as superseded. Red with `PlanPublisher::republish` writing a zero word
  (checked in a scratch copy outside the worktree; no edit under `crates/engine/src/realtime/`).
- *m5.* The `newest_providers` doc moved off `kept_holder`; `held_models` refuses a kept model with
  no row (`debug_assert!` and `INTERNAL`) instead of skipping it; the `synchronize_plan_epochs` and
  `promote_pending_provider` comments name every pending candidate.
- *Header gap.* The header's "neither refeeds nor reseeks" holds for every source the new plan
  keeps from the waiting replacement or from the running plan, except one case: a source or owner
  that A restarted or removed and B reverts to its P0 form starts fresh in B. That gap closes with
  the successor issue "Carry an owner or source a superseding candidate keeps unchanged from P0
  from P0's state".
- *Edits outside the original authorized paths, and who accepted them.*
  `crates/session/src/edit_ramp.rs` (coordinator, under B15-Q3); `hosts/host-web/src/lib.rs`
  (coordinator, under B15-Q4); `crates/engine/src/realtime/plan_exchange.rs`, the reservation
  accessors (one attempt, accepted by root on 2026-10-16); `crates/capi/src/runtime/mod.rs`
  (mechanical).

**Review fold 2 (batch 15a).**
- *B-1 (C ABI wedge).* Two `Taken` outcomes while render lagged its retirements parked two epochs
  (`retired [0,1] current 2 pending [3]`); render then adopted C and published epoch 3, and every
  later call promoted from the atomic before it drained, found `2 + 1 > 2` and answered
  `INTERNAL` forever. Fix, both halves: `RETIRED_PROVIDERS = RETIREMENT_CAPACITY` (3; each parked
  epoch's plan holds one retirement credit), and `synchronize_plan_epochs` reclaims every
  reclaimable plan before it promotes from the atomic. Test capi
  `supersede_tests::two_taken_candidates_and_a_published_adoption_never_wedge_the_session`
  (hold-reclamation and unpublished-render seams; asserts the reported state, then four blocks of
  service and a fourth edit commit, owners balanced). Red: with both halves reverted the first call
  after the hold clears fails (`capi.source.epoch`, the submit's service step answering
  `INTERNAL`); with only the capacity reverted, the test's step that reclaims nothing cannot park
  a third epoch (`Internal`).
- *M-1 (a displaced plan uncharged after a `Taken` promotion).* `ProviderEpoch::parked:
  Option<LiveEpochResources>` captures an epoch's report row, capi resources and carried bytes
  when it is parked (the report table drops the row; no later commit changes it), and
  `held_epochs` now returns every parked epoch first, then the running plan, then the candidates,
  so both admissions charge a plan until it is reclaimed. Ordering keeps M2: an epoch carries only
  from the one before it in that list or from a plan already reclaimed, so "every held epoch but
  the oldest leaves its carried bytes" counts each ring once. The `held_epochs` and admission docs
  say so. Test capi `a_displaced_plan_not_yet_reclaimed_is_charged_after_a_taken_promotion`:
  gate 4's cap (269,263, three-plan peak 311,372) gives `BACKPRESSURE` with P0 parked, one service
  clears it and the same T2 commits; a source-total cap at the counted-once peak (24,430 = 11,394
  + 22,478 - 10,640 + 22,478 - 21,280) admits and one byte below refuses. Red: parked epochs left
  out gives `Ok` at the graph cap and `Ok` at 24,429; parked epochs listed after the running plan
  (double count) refuses at 24,430.
- *m-1.* The header (supersession paragraph and the sizing rule), `CONTROL_PROTOCOL_SEMANTICS.md`
  and `C_ABI_V1_QUALIFICATION.md` no longer promise that a retry after a render commits:
  `BACKPRESSURE` means the edit fit against the newest held plan alone, so a retry normally
  commits, within a small band (about +64 graph and +158 capi bytes between B prepared across A
  and B' on adopted A; a donated ring counted twice in the second check). The sizing rule notes
  that a plan render has just displaced counts until reclaimed.
- *m-5 (browser).* Test host-web `structural_apply::a_superseded_candidates_spectrum_handover_
  drops_with_the_superseding_command` (single capture, stream running): A's entry and hand-over
  drop with T2's command, B's waits, the service after adoption promotes it and the stream
  publishes windows; red with the command's `drop_superseded_shapes` disabled. Test
  `structural_apply_realtime::a_superseded_candidate_with_a_spectrum_hand_over_costs_no_render_
  call_an_allocation`: render-locked allocator calls 0 over every block, no attachment drop in
  render; red (7 calls) when the adoption block drops the superseded entry's captures.
- *Found with m-5, fixed (outside the listed items).* `promote_spectrum` evaluated
  `entry.spectrum.take()` for every entry before checking `adoption`, so any service step or
  command between a rebuild's commit and its adoption block dropped the waiting candidate's
  spectrum hand-over. It now takes only an adopted entry's. The m-5 test asserts the hand-over
  survives a service step before adoption (red without the fix). A stream test with a single
  carried capture stayed green on the bug (the service's carried reader keeps serving), so the
  user-visible effect is unproven for that case and the test was not kept; a non-carried capture,
  a one-shot or a collection may differ (not investigated).
- *Re-pin.* Reference session `capi_retained_bytes` 165,922 -> 168,810 (x86-64, +2,888: one more
  1,648-byte provider-epoch slot and the 248-byte parked row in each of five slots), within the
  175,296 ceiling, which stays (`resource_lifecycle.rs`, `C_ABI_V1_QUALIFICATION.md`).
- *Acked PCM on a reverted source (root ruling of 2026-10-16, the acked-batch rule).* The header
  gap was worse than "starts fresh": when A redeclared a source X, the host fed A's restarted X
  ring (acked), and B declares X as P0 does, X is not donated (its declaration differs between A
  and B), B prepares it fresh, and A's acked chunks dropped with A. Guard:
  `host_core::drops_acked_reverted_source(B's model, A's donation side, P0's kept model)` (true
  when such an X's producer on A is no longer unfed: a chunk, a seek or an end of region), called
  in `rebuild` after preparation and before the admission; it answers `BACKPRESSURE`, so the
  caller republishes A unchanged, no event is queued and the revision stays. After render adopts
  A, the retry commits on A. It is small (one read-only function, one call; no state), so it was
  coded. Tests: capi `supersede_tests::a_supersession_that_would_drop_acked_pcm_on_a_redeclared_
  source_is_backpressure` (refusal, A still the candidate, no event, revision 44; every block
  bit-identical to the sequential run; the adoption block differs from a run whose host never fed
  A's ring, so the acked chunk is heard) and parity row `control_parity::a_supersession_that_would_
  drop_acked_pcm_is_backpressure_alike_on_both_hosts` (48 and 96 kHz: both hosts `6`, same
  revision, no browser event, retry commits alike, same bits after). Red without the guard: capi
  `Ok(80)`; parity `(0, 0)` against `(6, 6)`. The header, `CONTROL_PROTOCOL_SEMANTICS.md` and
  `C_ABI_V1_QUALIFICATION.md` now say such a supersession is refused with `BACKPRESSURE` until A
  adopts; TBD42 lifts the guard (a line added to its spec).
- *Edits outside the original authorized paths.* `hosts/host-web/src/lib.rs` (`promote_spectrum`
  only), `hosts/host-web/tests/structural_apply_realtime.rs` (the m-5 cost test),
  `hosts/host-web/tests/control_parity.rs` (the guard's parity row),
  `.github/ISSUE_SPECS/TBD42-*.md` (one line). No edit under
  `crates/engine/src/realtime/`.
