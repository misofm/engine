SOUND

# Adversarial review: revision-bounded live cells (decision 15 stream B)

Reviewer: adversarial design review, read-only on `wt-d15-b` at `65a33e9a4`.
Inputs: `revision-bounded-cells-design.md`, `revision-bounded-cells-prototype.rs`, AGENTS.md, the
D15 ruling (D15-2, D15-17), specs #1432 #1312 #1314 #1343 #1311 #1277 #1280 #1345 #1346 #1347
(and #1053 #1310 #1322 #1332 #1355 #1371 #1381 #1390 where the design touches them), and the code
in `spsc.rs`, `plan_exchange.rs`, `plan.rs`, `control.rs`.

## Verdict

The primitive and its proof meet root's binding requirement. I found no interleaving that gives a
torn value, a value newer than `S`, a skipped newest-`<= S` value, a double apply, a wrong
superseded count, a failing or allocating write, an ack before a drop, or an inexact
`first_sample`, inside the design's own scope. The memory orderings are necessary and sufficient,
and the "every gate write is a read-modify-write" argument is a correct C++20 argument, not a loom
artefact.

The findings are about integration and the plan around the primitive. Two are MAJOR:

- **M1.** The drop-in browser text breaks the browser once #1381 lands before #1382: every
  browser live edit would wait forever.
- **M2.** The cells written and the gate stamped are chosen by two different routings. The
  proposed debug assertion compares the wrong pair. Today the routings agree, but if they ever
  differ, the failure is a silent torn read. This is the strongest reason to put the gate in the
  plan (root decision 1).

## What I verified (evidence)

All work used an export of `HEAD`. The export and targets are deleted. Small files are kept in
`/tmp/claude-1002/r1432/keep/`.

- **Prototype models.** L1, L2 and L3 pass under loom 0.7.2 (release). The three unit tests pass
  without loom.
- **Design mutation table, 8 rows re-run.** Each re-run row gives exactly the design's result:
  `reader_load_not_rmw`, `protect_only_a` and `writer_forgets_pin` turn L2 red only;
  `dirty_before_snapshot` turns L1 and L3 red; `reader_never_defers` and
  `reader_ignores_snapshot` turn all three red; `gate_relaxed` turns L1 and L3 red; `tag_first`
  stays all green.
- **The loom claim (3.1 point 3) is true.** The probe fails with `first 0x0 last
  0x8000000000000000`: the first `fetch_or` read 0, and the final `fetch_or` read the first one's
  write, after both plain stores. C++20 forbids this. `s2` happens-before `rmw2` (join), so `rmw2`'s
  immediate predecessor in modification order is `s2`, or `rmw1` placed after `s2`, which must then
  have read 2.
  - Cause in loom 0.7.2 (`src/rt/atomic.rs`): modification order is a partial order (a
    `VersionVec` per store). `rmw` picks a maximal store, and `apply_load_coherence` then joins
    coherence onto that store *after* it was chosen. So a plain store that is unordered with an
    RMW's write can end up before it, although the RMW read an older value. RMW atomicity is
    lost.
  - On a location that has only RMW writes, each RMW reads the unique maximal store and adds one
    above it, so the order stays a chain and loom is exact. The gate and the dirty word are
    RMW-only, so the design's models are exact there.
  - **Soundness does not rest on loom.** Section 3.4 is a C++ proof. C++20 `[intro.races]`
    defines a release sequence as the head plus the later RMWs. C++20 removed the same-thread plain
    stores, so Rule G is exactly what keeps the sequences unbroken. Rust atomics follow the C++20
    model.
- **Three extra loom models of my own** (`keep/review-models.rs`). All three pass. I checked that
  each one can fail:
  - **R1, a claim races an unpinned stamp.** P runs on gate 0. C carries a retarget at 2, is
    rearmed, and is published by a CAS. Control then commits 3, 4 and 5 to C with the pin of C's
    gate (`None` at first). Render claims, snapshots and drains. Red under `protect_only_a`,
    `reader_load_not_rmw` and `reader_never_defers`.
  - **R2, an in-place rewrite races reads.** Two writes per revision. Red under
    `no_rewrite_in_place`, `protect_only_a` and `reader_load_not_rmw`. The design's L1-L3 stay
    green under `no_rewrite_in_place`; only the sequential unit test catches it.
  - **R3, partial transactions over three cells in one dirty word.** Every block must equal the
    committed model at its `S`. Red under `reader_never_defers` and under a new mutation, "a
    nonzero take replaces the deferred mask". L1 is also red under that mutation, so R3 adds no
    unique catch and is not recommended.
- **Spec reading.** I checked every claim in sections 4, 5 and 8 against the specs listed above.

## Answers to the five attack questions

1. **Correctness under interleavings.** I tried all of the following:
   - commits between and during blocks;
   - a paused host, and lapping;
   - a pending candidate (stamp pin `None` while render claims; R1);
   - withdraw and republish (#1343): the rearm happens-after render's last `fetch_or`, because the
     claim's release is read by `try_reserve`'s acquire;
   - scheduled adoption (#1311): it only delays the claim;
   - supersession (#1310): B is prepared against P0; every write to P0 happened before A's
     publication, and A's revisions complete `SUPERSEDED`;
   - the carry (#1277 `apply_pending(ALL)`, #1280 `peek_unread`): quiescence (Q) and ordering (R)
     hold, because control writes only the newest plan (`control.rs:1296`). Copy-mode carry, the
     one case where a predecessor keeps rendering and Q would fail, is retired (D15-8 round 5;
     #1322 closed).

   None breaks C1-C6. Each row of the orderings table (3.5) is needed and enough. The dirty
   pre-check `Relaxed` load is sound: the mark is sequenced before the publishing swap, that swap
   synchronizes with the snapshot `fetch_or`, and the snapshot is sequenced before the load. So
   write-read coherence forces the load to see the mark or a later value. A Lemma 1 wording defect
   is m1.
2. **Slot protection lemma.** It holds.
   - Every stamp revision `r` is greater than every published revision, and so greater than any
     pin. So the in-place slot is never `A` or `B`, and `{A, B}` leaves one free slot.
   - **Paused host.** The `ANNOUNCED` bit stays set from render's last `fetch_or`. The first swap
     after it sets `pin := S_last`. Later swaps keep that pin. Control rotates the slot that is not
     `A` and not `B`. This protects more than needed but costs nothing, and memory stays fixed
     (prototype test: 9,999 commits).
   - A pin left from an announcement whose section ended long ago only over-protects.
   - The one unguarded precondition is m2: a stamp whose revision is at or below a published
     revision.
3. **Multi-cell atomicity.** It holds.
   - Per cell, render applies the newest write with revision `<= S`. So a block brings every cell
     to the committed model at `S`, and partial transactions are included (R3).
   - **A cell whose only fresh value is above `S` is never lost.** The `swap(0, Acquire)`
     synchronizes with every mark before it in modification order. So when the take returns a
     cell's bit, the read sees that write's revision or a later one, and sets `newer`. Each slot's
     revision only grows and read-read coherence holds, so the cell stays deferred until a
     snapshot covers it.
   - **A bit taken in an earlier block is not lost either.** That block deferred the cell (the
     value was above its `S`) or applied the cell's newest value at or below its `S`.
4. **Cost.** The claims are correct.
   - **Render.** One `fetch_or` per block per plan replaces an `Acquire` load. An idle stage costs
     one `Relaxed` load (today: one `Acquire` load per queue). A pending cell costs three tag loads,
     one sequence load and `N` word loads. Banked SIMD kernels are unchanged, because drains only
     retarget ramp state.
   - **Gate home.** A gate per mailbox cell and a gate per plan cost the same on render and on
     control. The choice is about correctness structure (M2, M1), not cost.
   - **Wait-freedom (n3).** On AArch64 without LSE (ARMv8.0), `fetch_or` and `swap` compile to
     LL/SC loops. That is the same property the existing claim CAS and the dirty swap already have.
5. **Scope and split.** The order #1432 → X1 → X2 → #1312 is sound.
   - #1432 fits half a day.
   - X1 fits half a day as designed. With a gate per plan it is larger; split it then (decision 1).
   - X2 fits half a day, but its path list is incomplete (m7).
   - #1312 is at risk with gate 9 (m6).
   - The drop-in texts agree with D15-2 and D15-17 except for M1 and m8, and the list of
     consequential amendments is incomplete (m4).

---

## MAJOR

### M1. After #1381 and before #1382, every browser live edit waits forever

- **Reference.** Design 4.1 and the #1312 D7 drop-in: "the browser ... stamps each admitted batch
  with a host-local, strictly increasing batch number, `LiveStamp::unpinned`, and renders with
  `LiveSnapshot::ALL` ... until #1382 moves edits behind #1381's exchange".
- **The conflict.** #1381 (stream H, before #1382) moves the worklet onto `RealtimePlanOwner`: "The
  worklet renders through `RealtimePlanOwner`". Its Worker owns `SessionState` and the
  `PlanPublisher`, with "the committed model at revision 0". Its non-goals keep live edits on the
  worklet: "No live edit in the Worker (#1382)". After X1, `RealtimePlanOwner::render` always takes
  `S` from the mailbox gate.
- **Interleaving.**
  1. #1381 is merged. The gate of cell 0 holds 0, published by the Worker.
  2. The worklet admits a fader batch and writes the fader cell stamped `unpinned(1)`.
  3. Worklet render: `snapshot()` returns 0. The cell's revision is 1, which is above 0, so the read
     is `Unchanged` and the cell is deferred.
  4. Nothing ever publishes 1 on the gate: the Worker owns the publisher and sees no edit. So the
     edit never applies.
- **Effect.** `engine.apply` acknowledged a value that render never applies. That is an ack before
  a drop, in effect. #1381's own "same bits" gate turns red on any browser leg that makes a live
  edit.
- **Fix (root picks one).**
  - (a) Merge #1382 with #1381, or before it. Then no build renders through the exchange while
    edits stay on the worklet.
  - (b) A gate per plan (decision 1). The rule becomes "the thread that writes a plan's cells
    publishes on that plan's gate". In the #1381 window the worklet is the only writer of the
    active plan's gate (the Worker only builds new plans), so the worklet publishes its batch
    number and render takes the same path as the C ABI. Then `LiveSnapshot::ALL` leaves every
    production render path, and the browser needs no interim mode.
  - Do not add a "render with `ALL`" switch to `RealtimePlanOwner`. That is an interim mode
    (AGENTS.md; memory "no shortcuts").

### M2. Cell writes and the stamped gate use two routings, and no check ties them together

- **Reference.** Design 3.2 and the #1312 D7 drop-in: `PlanPublisher::live_stamp(revision)` routes
  by the mailbox word (`Full` cell, else `Active`), and "control asserts that the stamp's `GateId`
  and the publication's agree".
- **The two routings.** The cells actually written are chosen by the control plane's provider
  routing: `pending_providers.last_mut()`, else `providers` (`control.rs:1296`). The stamp's pin and
  the publication are both chosen by the mailbox word. So the proposed assertion compares two
  values from the same routing. It cannot detect a mismatch between the plan whose cells were
  written and the gate whose pin protected them.
- **Today the two routings agree.** I checked each case:
  - a rebuild is refused while a candidate is pending (`control.rs:1154`);
  - #1310 does withdraw, prepare and publish inside one call;
  - a candidate render has claimed is `Active` in the same cell.
- **Effect of a mismatch.** The writer's `B` protects the wrong plan's snapshot. Control can then
  overwrite the slot render is reading, with no detection: a torn value or a wrong value. That is
  the worst outcome the binding requirement forbids. `LiveStamp::unpinned` is a free constructor
  with the same failure mode if it is ever used on a plan render has adopted.
- **Fix.**
  - A gate per plan (decision 1) removes the second routing: the gate lives with the plan's cells
    and the provider epoch's writers.
  - If the gate stays in the mailbox cell, then:
    - record the mailbox cell in the provider epoch at publish and at republish;
    - debug-assert that the stamp's `GateId` equals the cell of the epoch whose cells are written;
    - make `LiveStamp::unpinned` constructible only from `PlanReplacementReservation` and
      `UnadoptedCandidate`, which control holds exclusively;
    - give the single-thread browser its own named constructor.

## MINOR

- **m1. Lemma 1 is false as written.** Its hypothesis "x does not happen-before the write" admits
  sections opened by an `f` that reads `w_{j+1}` or later. Then the write happens-before `x`, and
  `S_f` is not in `{rev(w_j), pin_j}`. The theorems survive, because such a write has revision
  `<= S_f` and is `N` or older. The dirty-word synchronization (a write that happens-before `x`
  through `swap(0)`) also leads only to a contradiction in the proof's second case.
  - Fix: restrict the lemma to writes whose revision is `> S_f`, which are the only ones the
    proof needs. Say that the dirty word's edges cannot place `f` after `w_{j+1}`.
- **m2. Nothing guards against a stamp revision at or below a revision already published on the
  gate.**
  - The in-place rule would then rewrite a published slot that render may be reading: a torn
    read.
  - The only check is "stamp revisions per cell are non-decreasing". It allows equality, so it
    does not catch this.
  - Fix: `GatePin` keeps the last published revision, and `stamp(r)` debug-asserts that `r` is
    above it. Unpublished plans are exempt: #1277 retargets equal the publication revision. Add the
    same check in `publish`.
  - In `commit_live`, debug-assert that the prospective revision used for the stamp equals
    `session().revision()` at `publish_committed_revision`.
- **m3. "Every live drain runs every block" must become a stated, tested invariant.**
  - With queues, a skipped drain made the watermark late. Under `S`, a skipped drain makes it
    **early**: the watermark covers `S` while a cell holding a value `<= S` was never scanned.
  - #1053 D11 records this as a forward hazard for silence skip. The memory "skip work on silence"
    makes it likely. Elided sections (#1268) and inactive routes (#1217 `RouteActivity`) are
    present-day candidates to check.
  - Fix: X2's gate also asserts that every live owner's drain is invoked once per block, the
    blocks of a silent or elided owner included. Alternatively, adopt the block-start pass of 4.4.
- **m4. The list of consequential amendments (8.4) is incomplete.** Not listed:
  - #1390 D6: "a later write before the same drain supersedes the earlier one";
  - #1371 D1: "#1312 D1: a triple buffer whose ...";
  - #1280's Context bullet (line 34), which describes the triple buffer. Only D2 is listed.
- **m5. T5 overstates its reach.** "Values on a predecessor are resolved by the carry" is true only
  for lanes the carry touches.
  - Values on lanes a swap restarts (#1277 D6), and values in a candidate that is withdrawn and
    dropped, are neither applied by render nor counted.
  - They are not lost. The successor is prepared from a model that holds them, or their revision
    completes `SUPERSEDED`.
  - Fix: write the unit as "writes to a plan that render adopts" (decision 4).
- **m6. #1312 is at risk of the half-day rule.**
  - #1312 has already been split once for size. The design adds D12's drain move (three
    processors plus the test-only scalar ones), gate 8 and gate 9.
  - Gate 9 is the hard one: a single-threaded replay converse under a real race. #1314's
    attempt-1 author reported that a first draft of exactly this comparison went red on
    unmodified code because of harness subtleties.
  - Fix: keep gate 8 in #1312. The loom models (L1-L3, and an L4 built like R1) already prove the
    race property. Make gate 9 a stateless successor, or bound it to one attempt.
- **m7. X2's path list misses an out-of-path trait implementation.**
  - `crates/graph-compiler/tests/scale.rs` implements `GraphPreparedBuiltinBankProcessor`
    (`IdentityBank`), so `begin_block`'s new signature breaks it.
  - X2 lists only `crates/graph`, `crates/rack` and `crates/builtins-compiler`. This repeats the
    cause of #1312's Amendment 1 stop.
  - The construction count is right: 27 `GraphBindingBlock {` sites in `builtins-compiler`, 6 in
    `graph`. The `begin_block` implementations are also in `graph/tests/rt{9,10,11}` and
    `rack/tests/mono_reengage.rs`, which the listed crates cover.
- **m8. The #1312 D9 header text has a bound the host cannot observe.** It replaces D15-2's bound
  ("the first block whose render call begins after the submit returns") with "the first block whose
  live snapshot is taken after the submit returns", which a C host cannot see.
  - D15-2's bound is still true: a render call that begins after the submit returns takes its
    snapshot after the publication.
  - Fix: keep D15-2's sentence for hosts, and add "every live value of one transaction takes
    effect in the same block". Use the snapshot wording only in engine docs.

## NIT

- **n1.** Write "bits 0-62 hold the revision". "Bits 0..63" is open to misreading.
- **n2.** The #1432 hazard text cites "loom L2 red" for a gate written with a plain store. The
  design's own footnote says part of that red comes from loom's imprecision. Cite the
  `protect_only_a` and `writer_forgets_pin` rows instead.
- **n3.** "Wait-free, retry-free" is true at the algorithm level. On ARMv8.0 without LSE the RMWs
  are LL/SC loops (see answer 4). Do not cite `ldsetal` as the cost model; state it in
  `docs/REALTIME_DEPENDENCY_POLICY.md` terms.
- **n4.** Decision 3 says a block-start pass would make an errored block exact "by construction".
  It would not: a block that errors after its drains has still advanced some ramp state and
  rendered no output. Both placements leave an invariant error as a defect path.
- **n5.** Every reader of a gate word must mask `ANNOUNCED`: `withdraw`'s load of a `Full` cell's
  gate, and tests that read the gate. Today the bit cannot be set there, but the mask costs
  nothing.
- **n6.** Base L4 on R1. Include at least two commits to the claimed candidate after its
  publication, so that the candidate gate's `B` protection is exercised. The L4 described in 6.3
  makes one commit, which never needs `B`.

---

## Root decisions (section 10): recommendations

1. **Gate home: put the gate in the plan.** Reasons:
   - one routing: the gate travels with the plan's cells and with the provider epoch that writes
     them (M2);
   - it resolves M1 structurally: the writer of a plan's cells publishes on that plan's gate, on
     either thread;
   - no production render path needs `LiveSnapshot::ALL`: direct render, the browser and the audit
     tools all read their plan's gate, which matches the memory "one implementation shape";
   - no rearm and no pin reset across epochs;
   - it deletes `RevisionTarget`, `store_revision`'s routing, the withdrawn revision load,
     `UnadoptedCandidate::set_revision`'s special case and the "racy converse" of #1314
     Amendment 1.

   The cost is reworking #1314's code, which is on the unmerged batch branch, not on `main`. The
   gate also needs a shared allocation at preparation, with its resource row. Split X1 into two
   slices: (a) the plan gate, the snapshot and the watermark read from the plan's gate, and the
   mailbox revision words removed; (b) the control-plane stamp and publication through the
   provider epoch. If root keeps the gate in the mailbox cell, then M1 needs merge order (a) and
   M2 needs all of its guards.
2. **The revision ceiling: in the protocol session store.**
   - `protocol` already depends on `engine`, so it can use `engine::GATE_REVISION_MAX` directly.
   - The store's existing `checked_add` already refuses with `RevisionExhausted`
     (`model.rs:932-935`). Bound it by `GATE_REVISION_MAX` instead of `u64::MAX`.
   - Refuse to create or restore a session above the ceiling.
   - `plan_exchange_at_revision` refuses an initial revision above it.
   - A control-plane pre-check would be one more place that a future commit path can bypass.
3. **Executor errors after the drains: accept as a defect path.**
   - Keep the drains at stage entry. A block-start pass does not make an errored block exact
     (n4).
   - Make sure the existing render diagnostics count such a block.
   - Separately, adopt m3's invariant and its gate. That hazard is the real one.
4. **The counter gap: confirm the exclusion, and widen the statement.**
   - The unit is "writes to a plan that render adopts".
   - Writes to a candidate that is withdrawn and dropped complete through the revision-level
     `SUPERSEDED` outcome (#1310 D6, D15-17).
   - Values on lanes a swap restarts are in the successor's prepared model.
   - Write this into #1312 D2 so that "exact" has a stated unit (m5).
5. **#1345 D4: agree; it must change.**
   - The staging window holds every live parameter cell, sized at preparation, and no deferral
     path remains.
   - A window deferral would split a revision. It would also break T5's counting: a cell deferred
     past the block that covered it is resolved under a later `S`, so a covered value can be
     counted as superseded although it should have been applied.

**Questions to add to section 10:**

- **6.** The browser order for #1381 and #1382 (M1). This question goes away with decision 1 as
  recommended.
- **7.** The "every live drain runs every block" invariant, with its gate (m3).
- **8.** Structural guards on stamps: scope `unpinned`, and assert that a stamp's revision is above
  the last published revision (m2, M2). Needed only if the gate stays in the mailbox cell; the
  revision check is useful in both designs.
