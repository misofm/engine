# Test that a refused batch does not undo a committed browser VCA batch

Stream B of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2: a refused
submission changes nothing, and an ack never comes before a drop). This is a test-only follow-up to
*Hold route-lane values in latest-target cells* (#1347). It comes from verdict mutation M13 of
attempt 1 and note n4 of attempt 2 (`/home/bl/misofm/submix-verdicts/1347-attempt1.md:116-118`,
`:211`; `1347-attempt2.md:145-148`). Code anchors verified on `codex/d15-stream-b3` at
`3bcd99ebe`. The anchors move after #1345 attempt 2: re-verify every anchor before you start.

Root decisions 2026-10-12 after an adversarial review: the solo and route sibling tests are in
scope (gate 2).

## Product outcome

A browser app rides a VCA (for example "band −6 dB") or mutes it, and the engine admits the ride.
If a later submission is refused, the next ride still starts from the committed value. Today no
test fails when the browser forgets to commit its VCA state after an admitted batch. The defect
would make a later refused batch roll the VCA mirror back to before the admitted ride. The next
ride would then stage no fader or mute record, and the members would stay at the old ride for
good. This issue adds the test that catches that defect, and the same-shape tests for solo and
route state if gate 2 shows that they are missing too.

## Context

- **Where the browser commits.** `admit_commands` (`hosts/host-web/src/lib.rs:4649-4691`) runs the
  staged admission. On `Ok` it calls `ready.solo.commit()`, `ready.routes.commit()` and
  `ready.vcas.commit()` (`:4670-4675`). On every refusal it calls the three `rollback`s
  (`:4676-4681`). Every refusal kind reaches that one rollback site (`1347-attempt2.md:23-26`). An
  unknown-VCA refusal does too: it is raised in `admit_commands_staged` (`:5165-5183`), which
  `submit_commands_inner` reaches through `admit_commands` (`:3417`).
- **What commit does.** `LiveVcaState` (`crates/host-core/src/vca.rs`) has one transaction:
  - The first mutation of a submission copies the mirrors aside (`shadow`, `:417-428`) and sets
    `open`.
  - `rollback` restores the copies, but only while `open` (`:362-372`).
  - `commit` clears `open` (`:357-360`).
  If the success path drops `commit`, `open` stays set. The next submission takes no new shadow.
  If that submission is refused, its rollback restores the state from before the admitted batch,
  although the admitted batch's fader and mute cells already reached the plan.
- **Solo and route state have the same shape.** Their `commit`/`rollback` use the same open-flag
  transaction (`solo.rs:357-372`, `live_route_state.rs:201-210`), so the same defect class applies
  to them. Whether a dropped `ready.solo.commit()` or `ready.routes.commit()` is caught today is
  not known; gate 2 measures it.
- **Why no test catches it.**
  - `assert_vca_refusal` (`hosts/host-web/src/tests.rs:14170-14190`) checks that a refusal leaves
    `vca_snapshot` unchanged and closes the transaction. Every test that calls it refuses on a
    host whose VCA state never had an admitted batch before the refusal.
    `vca_records_are_addressed_and_shape_checked` (`:14939`) runs its refusals (`:14944-15125`)
    before its admitted domain-edge rides (`:15127-15145`).
  - The randomized `a_live_vca_ride_lands_on_a_fresh_plans_bits` (`:14242`) never submits a
    refused batch.
  - The verifier measured M13 (delete `ready.vcas.commit();`) green on every browser test at
    #1347's parent and at its head.
- **The primitive is covered.** host-core's `rollback_restores_every_mirror_and_commit_keeps_them`
  (`crates/host-core/tests/vca_live.rs:803`) shows that `commit` keeps the mirrors and that a second
  transaction shadows again. The gap is only in the browser caller.
- **The C ABI has no such gap (verified).**
  - No `LiveVcaState` and no VCA staging exist in `crates/capi` or `crates/control-plane`
    (`grep -rn LiveVcaState crates/capi crates/control-plane` finds nothing).
  - `classify_live_delta` sends every delta to a rebuild while either model declares a VCA
    (`crates/host-core/src/live_delta.rs:230-231`, `LiveRebuild::Vca`). The committed model is
    swapped as a whole.
  - *Deliver value-only VCA edits to the running C ABI plan* (#1247, stream F) keeps it so. Its D6
    says "The C ABI keeps no `LiveVcaState`" and diffs committed models, so there is no mirror to
    forget to commit.
- **Helpers that exist** (`hosts/host-web/src/tests.rs`):
  - `vca_ride_model` (`:14006`; rows are `VcaRow = (id, [dB; 2], [mute; 2], members)`, `:13984`);
  - `vca_ride_host` (`:14036`);
  - `stage_vca` (`:14044`; kinds `COMMAND_VCA_FADER_DB = 16` and `COMMAND_VCA_MUTE = 17`,
    `lib.rs:932`, `:940`);
  - `vca_lockstep` (`:14092`; renders the same blocks on every host and compares host 0 bit for
    bit with another);
  - `assert_vca_refusal` (`:14170`);
  - `follow_render` with `SOLO_FOLLOW_TRACKS` (`:12390`, `:12487`).
- **#1562 replaces this code later.** *Admit browser live edits in the Worker through the committed
  model* (#1562, stream H; formerly #1382 D5) deletes host-web's VCA composition pass. On `codex/d15-stream-b3`, #1382's D5
  (`1382-…md:121-125`) only says that composition tests "move … or are deleted", so it does not
  yet name this test. The render half of this test (gate 1, step 5) is the part that carries over to
  the shared path.

## Decisions frozen

- **D1. Test only.** No product code changes. The defect is not present today; the tests guard
  the existing `commit` calls.
- **D2. Two checks in one test.**
  - The mirror check (`assert_vca_refusal`'s snapshot) fails at the point of the defect.
  - The render check (`vca_lockstep` against a reference host that never saw the refused batch)
    shows what the app hears, and it does not depend on the browser's mirror. #1562 can carry it
    over.
- **D3. The refused batch also rides the VCA.** Record 0 is a valid ride of the same VCA to a third
  value, and record 1 names an unknown VCA. The refusal therefore rolls back a real VCA change.
  The snapshot check then proves that rollback lands on the committed ride, not on the value from
  before it.
- **D4. Both VCA kinds.** The test loops over a dB ride (kind 16) and a mute (kind 17). Each kind
  has its own mirror (`vca_db`, `vca_mute`) and its own consequence (fader records, or the strip
  mute owner's VCA term).
- **D5. Smoothing 0 everywhere.** No ramp length is in question here.

## Deliverables

One to three new `#[test]`s in `hosts/host-web/src/tests.rs`, after
`a_member_mute_after_a_vca_mute_in_one_batch_composes_with_it` (`:15538`):

- `a_refused_batch_does_not_undo_a_committed_vca_batch` (always).
- A solo test and a route test of the same shape, only for each commit that gate 2 finds green when
  deleted. Name each in the same form (`a_refused_batch_does_not_undo_a_committed_solo_batch`,
  `a_refused_batch_does_not_undo_a_committed_route_batch`).

Each has a test-value doc comment in the file's usual form.

## Authorized paths

- `hosts/host-web/src/tests.rs` (the new tests only; no helper changes).
- This issue's own spec file.

## Non-goals

- The C ABI (no mirror; see Context).
- Any change to `admit_commands`, `LiveVcaState` or `assert_vca_refusal`.
- More than three new tests in total.

## Objective gates

1. **The VCA test.** For each `(kind, ride, back)` in
   `[(COMMAND_VCA_FADER_DB, -6.0, 0.0), (COMMAND_VCA_MUTE, 1.0, 0.0)]`:
   1. `model = vca_ride_model(&[], &[("band", [0.0; 2], [false; 2], &["bass", "drums"])])`.
      Boot two hosts from it with `vca_ride_host(&model, 16)`: `live` and `reference`. Render
      block 0 on both (`vca_lockstep` with `compare: Some(1)`).
   2. On both: `stage_vca(host, 0, kind, 0, 2, ride, 0)`. `submit_commands(1) == RESULT_OK`.
      Render 2 blocks in lockstep (still bit-identical).
   3. On `live` only: `stage_vca(live, 0, kind, 0, 2, <a third value: 3.0 for dB, 0.0 for mute>, 0)`
      and `stage_vca(live, 1, kind, 5, 2, ride, 0)`. Then
      `assert_vca_refusal(live, 2, RESULT_INVALID_ARGUMENT, COMMAND_REASON_UNKNOWN_VCA, 1, ..)`.
      That helper requires `vca_snapshot` unchanged and the transaction closed.
   4. On both: `stage_vca(host, 0, kind, 0, 2, back, 0)`. `submit_commands(1) == RESULT_OK`.
   5. Render 8 blocks in lockstep with `compare: Some(1)`. Both hosts must be bit-identical in
      every block, and at least one compared block must be audible (`vca_lockstep` returns `true`).
   - Mutation M13 (PR evidence): delete `ready.vcas.commit();` (`lib.rs:4673`). The test turns red
     at step 3: the snapshot moves back to the state before step 2.
   - With step 3's snapshot check removed (local edit, not committed), M13 still turns the test red
     at step 5. `live`'s mirror says the VCA is already at `back`, so the step 4 ride stages
     nothing, and `live` stays at `ride` while `reference` returns. This shows that the render half
     has its own catch for #1562 to carry.
   - On the unmutated tree the test is green. Record both runs in the attempt record.
2. **Sibling commits.** Run the browser tests once with `ready.solo.commit();` deleted and once with
   `ready.routes.commit();` deleted, and record red or green with the failing test's name. For each
   that stays green, add one test of gate 1's shape in the same file: an admitted solo or user mute
   (for solo), or an admitted change to a following route's source mute (for routes), then a refused
   batch that also changes that state, then a ride back. Check `assert_*`'s snapshot and render in
   lockstep against a reference host. At most three new tests in total in this issue. Each new test
   has its own test-value sentence and is shown red under its own deleted commit (PR evidence).
3. **Commands.**
   - `cargo test --locked -p host-web --lib a_refused_batch_does_not_undo_a_committed_vca_batch`
   - `cargo test --locked -p host-web --all-features`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked -p host-web --all-targets --all-features -- -D warnings`
   - `bash scripts/check-workspace-policy.sh`

## Test value

A browser admission that does not commit its VCA state after an admitted batch (dropped, moved
behind a conditional, or committed on the wrong owner) turns the VCA test red. A later refused batch
then rolls the VCA back past a ride that the plan already rendered, and the next ride is silently
lost. No existing test catches this (M13 is green on every browser test today).

Solo and route tests (if added): red if admission does not commit that state, so a later refused
batch rolls back past a rendered change.

## Dependencies

- None open for the code. The browser VCA admission landed with #1245. It runs on any tree that has
  #1347.
- It must land before H #1562, which replaces the code under test. Root amends #1562 (formerly #1382 D5) to add: "The
  render half of `a_refused_batch_does_not_undo_a_committed_vca_batch` (and its solo and route
  siblings, if any) moves to the shared path; it is not deleted."
- `vca_ride_host`'s second argument is the boot word that the sibling issue *Make the browser
  live-command boot word an on/off flag* turns into a 0/1 flag. If that issue lands first, pass `1`
  where this spec passes `16`. Either order works.

## Placement (root edits `STREAMS.md`)

- Stream B, after #1345. It is test-only and can be folded into the current `codex/d15-stream-b3`
  batch next to #1345, or the next B batch.
- `hosts/host-web/src/tests.rs` row ("about 30 slices ... edit it"): in any order with the rest;
  the later slice rebases. It must land before H #1332 and #1562, like the other B slices in the
  `hosts/host-web/src/lib.rs` row.
- Owner of the code: browser VCA admission is #1245's (decision 13,
  `docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`; umbrella #1239). In decision 15, host-web
  is stream H's, the C ABI VCA path is stream F's (#1247), and stream B holds the verifier finding.
  B is the natural home for a test-only issue that lands before H rewires the file.

## Attempt record

- Attempt 1: not started.

**Decision 17 amendment (2026-10-13).** The shared-path half (a refused batch of a VCA offset, a solo and an out-of-domain pan changes nothing, and the next 8 blocks equal a run without it) is #1559 gate 3. The browser half stays this issue's until #1565 deletes host-web's VCA, solo and route mirrors and their commit; this issue then closes as superseded by #1565.
