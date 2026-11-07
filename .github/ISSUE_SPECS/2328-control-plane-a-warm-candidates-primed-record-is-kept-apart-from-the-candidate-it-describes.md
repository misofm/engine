# Control plane: a warm candidate's `Primed` record is kept apart from the candidate it describes

Found by the implicit-state-machine audit of 2026-11-06 (tracker #2258, item PR-7, agent-reported;
re-verified here on `d11fed9b4`).

## Mission
Store the `Primed` record of a warm candidate on the candidate's own pending epoch, so it lives and
drops with that epoch and cannot name any other candidate, and make the record a reading of the
candidate's own reservation, so its epoch and schedule cannot differ from what that reservation
publishes. Nothing is cleared by hand, and no `debug_assert` guards the pairing.

## Verified defect (base `d11fed9b4`)
`crates/control-plane/src/control.rs`:
- The record is a loose `SessionState::primed: Option<host_core::PrimedCandidate>` (1619), beside
  `pending_providers`, which holds the candidate it describes. One write sets it, after the
  candidate is pushed and published (`rebuild`, 6940). Six sites clear it by hand:
  - render's adoption (`synchronize_plan_epochs`, 4074-4079);
  - a `Taken` or `Nothing` withdrawal (`withdraw_pending_candidate`, 5852 and 5858);
  - a `Withdrawn` candidate of another epoch than the record's (5841-5846);
  - the deadline step's `Stale` arm (`check_prime_deadline`, 5620-5622);
  - the supersession's overwrite (6940).
- Two `debug_assert`s guard the pairing: "a record names the one pending candidate" (6934-6939),
  and "W's record stays" after a refused transition (5804).
- `crates/host-core/src/warm.rs`: `PrimedCandidate` is a public `Copy` struct with public fields
  (48-56), so any code can build a record of any epoch. `PrimedCandidate::hold_until` (66-82)
  takes the record and the reservation as two arguments, and their epochs agree only by a
  `debug_assert_eq!` (71-75). That point was left here by #2273.
  `DeadlineStep::Stale` (199) exists only for a record that names another epoch than the newest.

**Reachability: no mismatch is reachable today.** The record's invariant is "`primed` is `Some`
exactly while the newest pending epoch is a warm candidate, and it names that epoch with the
schedule its reservation published". Each path that moves a candidate keeps it:
- *Publication.* `pending_providers` is pushed at one site (`rebuild`, 6858). The record is set
  from the same reservation after the push and before anything else runs, with no fallible step
  between.
- *Adoption.* Render's adoption reaches `promote_pending_provider` either from
  `synchronize_plan_epochs` or from a `Taken`/`Nothing` withdrawal. Both clear the record next to
  the promotion.
- *Republish after withdrawal.* A refused supersession (`rebuild_withdrawn`) republishes the
  withdrawn W, whose epoch never left `pending_providers` (only the post-commit supersession pops
  it, 6674). The record still names W, and the `Withdrawn` arm keeps a record of W's epoch.
- *Supersession.* The donor is popped after the commit, and the record is overwritten by B's
  record, or by `None` for a non-warm B, at 6940.
- *Panic-republish.* A panic before the candidate's take (#2044) leaves `self.primed` untouched:
  the successor's record is a local, dropped by the unwind. W is republished with its flags
  restored (`restore_prime_flags`), and its record matches. A panic after the take is outside
  #2044's cover.
- *`Stale` and the other-epoch `Withdrawn` clear.* With the invariant held, the record always
  names the newest epoch, which is the one a withdrawal returns. Both arms are unreachable.

So, as with PR-6, the defect is structural: one forgotten clear or write would silently leave a
record that outlives its candidate, or a warm candidate without one. Without a record the
deadline never fires, and a warm candidate that is never ready is never replaced by the
transition. There is no behavioural test that is red on base. The difference in behaviour is
confined to internal-fault paths. Base clears the record before a `Taken` promotion that can
still refuse `Internal`, and leaves it set when `synchronize_plan_epochs` promotes the candidate
and then returns `Internal` before its clear. With the record on the epoch, it drops exactly at
the promotion in both cases.

## Decision / model
- `EpochPhase::Pending` gains `prime: Option<host_core::PrimedCandidate>`, set to `None` by
  `ProviderEpoch::candidate`. This is the attachment #2287 (PR-6) prepared.
  - Two `Pending`-phase accessors read and write it: `prime()` and `prime_mut()`. A running or
    parked epoch answers `None`.
  - `promote()` drops the record with the phase.
  - A superseded donor drops with its epoch.
  - A withdrawal leaves the epoch pending, so a republished candidate keeps its record.
- `SessionState::primed` is deleted, and with it the six clears and two `debug_assert`s.
  `SessionState::newest_prime()` reads `pending_providers.last()`'s record. The deadline step, the
  transition fallback, the republish's flag restore and the `test_primed` seam all read it.
- **`hold_until` is bound to its candidate** (the #2273 hand-off). `PrimedCandidate`'s fields
  become private, with `const` getters.
  - The one constructor is `PrimedCandidate::of(&reservation)`. It reads the reservation's own
    epoch and its `PlanAdoption::Primed { not_before, lead_blocks }`.
  - `publish_primed` returns nothing.
  - The hold is `host_core::hold_primed_until(&mut reservation, not_before)`. It raises the
    `not_before` of the reservation's own `Primed` adoption and never lowers it.
  - `rebuild` reads the record once the schedule is final, just before the push, so the record is
    exactly what the commit publishes. No record and reservation can be passed apart, and the
    `debug_assert_eq!` is deleted.
- `host_core::warm::DeadlineStep::Stale` and `check_prime_deadline`'s `newest_epoch` parameter are
  deleted: the record the caller passes is the newest pending epoch's own.
- The model leaves one check in place, `fall_back_pending_warm`'s arm for a withdrawn candidate
  whose epoch is not the newest pending epoch. That arm is about the publisher and the pending
  list agreeing, not about the record, and it stays as defence.
- Control thread only. Render reads the reservation's published adoption through the mailbox and
  the rings' prime flags, and neither changes. No render thread reads the record, so there is no
  loom model.

## Non-goals
- `pending_providers` as an `Option` (the #2287 follow-up), HC-5 (`ControlSource.provider`), and
  #2313's edits to `rebuild`'s give-back step and `crates/host-core/src/source.rs`.
- No change to `restore_prime_flags`'s rule (#2273) or to the deadline arithmetic.
- No render-path change.

## Gates
- No mismatch is reachable, so no test can be red on base. Recorded instead: the reachability
  proof above, plus a mutation run that shows which existing tests defend the new placement and
  the reservation-derived record.
- `cargo test --locked` for control-plane, host-core, capi, host-web and bench, with the CI
  test-support features. Also clippy `-D warnings`, `cargo fmt --check`, rustdoc `-D warnings`
  and `bash scripts/check-workspace-policy.sh`.
- The realtime policy tool is run as a check. No marked region changes.

## Evidence (attempt 1)

**Verified on `d11fed9b4`.** Every write and clear of `SessionState::primed` is listed under
Verified defect, and the reachability proof covers the four cases the brief named: a republish
after withdrawal, a superseded candidate, a `Stale` deadline and a panic-republish. None of them
gives a reachable mismatch, so no regression test is red on base. As the brief directs, the record
moved onto the candidate anyway. `hold_until` is bound by construction: the record is a reading of
the reservation, not a separate value. No render thread reads the record, so no loom model.

**Model as built.**
- `crates/host-core/src/warm.rs`:
  - `PrimedCandidate` has private fields and `epoch()`, `not_before()`, `lead_blocks()`. It is
    made only by `PrimedCandidate::of(&reservation)`.
  - `publish_primed` returns `()`.
  - `hold_primed_until(&mut reservation, not_before)` replaces `PrimedCandidate::hold_until` and
    its `debug_assert_eq!`. It is exported beside `publish_primed`.
  - `DeadlineStep::Stale` and `check_prime_deadline`'s `newest_epoch` are gone.
- `crates/control-plane/src/control.rs`:
  - `EpochPhase::Pending { prime, .. }`, with `ProviderEpoch::prime()`/`prime_mut()`.
  - `SessionState::newest_prime()`.
  - The `primed` field, its six clears and its two `debug_assert`s are deleted. Its initializer in
    `crates/control-plane/src/compile.rs` goes too.
  - `rebuild` publishes and holds on the reservation, then stores `PrimedCandidate::of` on the
    candidate's epoch just before the push.
  - Tests read the record through the getters. Three struct literals became tuples; nothing else
    in them changed.

**Red on base.** None is reachable (see the reachability proof). No new test is added: the
existing `control::tests::warm_successor` gates already defend each new transition, as the
mutations below show.

**Mutations** (each applied alone and reverted; `cargo test -p control-plane --features
test-support,host-core/test-support --lib -- warm_successor`, 19 tests):
- M1, the rebuild never stores the record on its candidate: 10 red, including
  `a_growth_is_published_primed_and_recorded`,
  `a_warm_candidate_not_ready_by_its_deadline_takes_the_transition` and
  `the_service_step_runs_the_deadline_step_once`.
- M2, the hold does not raise `not_before`: 4 red, including
  `a_growth_that_removes_a_strip_is_primed_at_the_ducks_sample`.
- M3, the hold lowers `not_before` (`min`): 6 red.
- M4, the republish never restores a primed candidate's flags:
  `a_republished_warm_candidate_keeps_its_own_prime_flags` red.
- M5, a `Withdrawn` withdrawal drops the candidate's record (the old clear, misapplied): 5 red,
  including `a_refused_fallback_at_the_deadline_keeps_the_donor_pending` and
  `the_record_names_the_one_pending_warm_candidate`.
- M6, `newest_prime` reads `pending_providers.first()`: survives. It is an equivalent mutant,
  because at most one candidate is ever pending (#2287's proof). The `Option` follow-up below would
  remove the choice.
- M7, the record is read right after `publish_primed`, before the duck's hold (the
  record/reservation split this change removes): 2 red,
  `a_growth_that_removes_a_strip_is_primed_at_the_ducks_sample` and
  `a_warm_candidate_not_ready_by_its_deadline_takes_the_transition`.

**No bit moved.** No render-path code changed. Render reads the reservation's published adoption
and the rings' prime flags. The adoption is now written by `hold_primed_until` with the same
`max`, and the flags are untouched. The realtime policy tool reports `ok (295 marked regions in 46
files)`.

**Gates (head).**
- `cargo test --locked -p control-plane -p host-core -p capi -p host-web -p bench --all-targets`,
  with the test-debug-a feature list: 101 binaries, 1164 passed, 0 failed.
- `cargo clippy --locked -p host-core -p control-plane --all-targets --features
  host-core/test-support,host-core/control-provider,control-plane/test-support -- -D warnings`,
  `-p host-core --all-targets` with no features, and `-p control-plane --lib`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p host-core -p control-plane`: clean.
  Also clean with `--document-private-items`, for control-plane with `test-support` and for
  host-core with `control-provider,test-support`.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `realtime_policy`: ok.

**Coordination.** Branch `codex/issue-2313-host-core-producer-state` (#2313) edits `rebuild`'s
give-back (`adopt_persisting` becomes `reclaim_from_withdrawn`) a few lines below the deleted
`self.primed = primed`. `git merge-tree` of the two heads merges cleanly.

**Follow-ups (not in this issue).**
- `pending_providers` as an `Option` (from #2287). It would make `newest_prime` the only choice
  (M6) and remove the dead two-pending arms of `restore_prime_flags` and `kept_holder`.

## Verdict (attempt 1)
**PASS** (Sol's adversarial review). The reviewer confirmed:
- the change is equivalent on every reachable path;
- both internal-fault paths now hold the record exactly as long as its candidate;
- nothing outside control-plane used the changed host-core API;
- the branch merges cleanly with `main` and with #2313.

Three minors, folded on the branch after merging `origin/main`:
1. Three doc lines past 100 columns are rewrapped: the `EpochPhase` doc, `promote()`'s doc and
   `publish_primed`'s doc.
2. `hold_primed_until`'s rule now has a host-core test, so it is no longer defended only by
   control-plane's `warm_successor` (M2 survived in host-core and capi). The test is
   `a_primed_hold_raises_not_before_and_never_lowers_it` in
   `crates/host-core/tests/warm_successor.rs`. It checks that:
   - a hold on a `Next` reservation changes nothing, and `PrimedCandidate::of` reads `None`;
   - a `Primed` reservation's `not_before` rises to the latest hold and is never lowered;
   - the lead is kept;
   - `of` reads back the reservation's epoch and schedule.

   Each mutation turns it red: M2 (no raise), M3 (`min`), and a hold that replaces `not_before`
   outright.
3. `fall_back_pending_warm`'s defensive arm, for a withdrawn candidate that is not the newest
   pending epoch, no longer repeats silently. In release it used to answer `NotPending` and run
   again on every control call. It now republishes the candidate, drops the newest pending
   epoch's record so the next step does not withdraw again, and returns `Internal` once. The
   service step reports that as `ServiceFailure::Deadline`; the C ABI reports it as
   `INTERNAL`/`control.internal`. The arm is unreachable and keeps its `debug_assert!(false)`, so
   no test drives it.

Gates re-run on the folded head are recorded in the issue's evidence comment.
