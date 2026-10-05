# Make the nonadjacent split-pair harness's track choices observable to its tests

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Found by
the verdict of *Remove the dead code builtins-compiler reports under no-features clippy* (#1235),
"Observations outside this issue"
(`docs/handoffs/decision-15-2026-10-05/verdicts/stream-j/1235-attempt1.md`). The gap predates
#1235. Test code only.

## Problem (verified on `main` at `0a1176b3b`)

The scalar split-pair tests in `crates/builtins-compiler/src/lib.rs` build their graph with a
harness that picks tracks by `BoundaryVariant` (`:6068-6080`):

- **`output_track`** (`:6172-6177`): the track whose `PostMatrix` feeds the `Output`.
  `NonadjacentTrackA` and `NonadjacentOutputConflict` (test-support only) give `n - 1`; every other
  variant gives `0`.
- **`selected_index`** (`:6998-7003`): the track whose pair the scalar-schedule slot assertions
  (`:7004-7068`) treat as "A". `NonadjacentTrackA` and `NonadjacentOutputConflict` give `0`; every
  other variant gives `n - 1`.

Four mutants of these arms leave the whole suite green, on `main` as on #1235's base (the verdict
calls them M1/B1, M3/B3, M5/B5 and M7/B7):

| Mutant | Arm | Features |
| --- | --- | --- |
| M1 | `output_track`: `NonadjacentOutputConflict => n - 1` becomes `0` | test-support |
| M3 | `selected_index`: `NonadjacentOutputConflict => 0` becomes `n - 1` | test-support |
| M5 | `output_track`: `NonadjacentTrackA => n - 1` becomes `0` | none |
| M7 | `selected_index`: `NonadjacentTrackA => 0` becomes `n - 1` | none |

- **M1.** The #916 test
  `actual_scalar_nonadjacent_output_track_takes_the_split_pair_now_the_output_is_dedicated`
  (`:8003`) says its second fixture "makes t01 the output track" (doc comment, `:7996-8000`). It
  never checks that. It passes unchanged when t00 is the output track, so it does not test its own
  premise.
- **M3, M7.** With `n == 2` and a stage-major schedule
  (`[I0, I1, B0, B1, F0, F1, M0, M1, Out]`), the slot assertions hold for both orientations by
  construction. So `selected_index` cannot change any outcome.
- **M3, a stale arm.** The #916 test asserts that production selects t01's pair for
  `NonadjacentOutputConflict` (`graph::test_only_selected_split_fader()`, `:8063-8073`), while the
  harness's arm names track 0 as the selected track. `NonadjacentTrackA`'s arm (0) agrees with what
  `actual_scalar_overlapping_nonadjacent_candidates_select_one_and_keep_the_other_separate` expects
  production to select (t00, `:7904-7912`). The default arm (`n - 1`) agrees with the #916
  test's first fixture (`Nonadjacent`, t01) only: production's choice for `Nonadjacent` depends on
  the meter, not on the variant (Amendment 1).
- **M5.** No test reads which track feeds the `Output` for `NonadjacentTrackA`. Since #916 the
  `Output` is dedicated storage, so the output track may no longer affect any outcome.

## Decisions

- **D1. Every arm is observable or gone.** After this issue, each of M1, M3, M5 and M7 turns at
  least one test red, or its arm no longer exists. An arm that no outcome can observe is deleted
  (the variant then takes the default arm), and its deletion is recorded with the reason. A harness
  choice that changes nothing is configuration that misleads the reader.
- **D2. The #916 test checks its premise (M1).** In that test, assert on the graph
  `track_graph_variant(2, variant)` builds that the `Output` edge's source is t00's `PostMatrix`
  for the first fixture (`Nonadjacent`) and t01's for the second (`NonadjacentOutputConflict`).
  Name the tracks by literal ID, not through `output_track`, so the assertion does not restate the
  harness.
- **D3. `selected_index` is the track production selects (M3, M7; Amendment 1).** Both
  per-variant `selected_index` arms go. If production exposes its selection, or the thread-local
  witness `graph::test_only_selected_split_fader()` records it, the harness reads `selected_index`
  from that observation instead of re-deriving it. Only if no such observation exists does the
  harness re-derive it from production's meter rule (`Nonadjacent` selects t00 unless t00's
  post-fader is metered, `post_fader_observed && meter_track == 0`, and then t01;
  `NonadjacentOutputConflict` selects t01, `n - 1`), and then a mutant of that production rule
  must turn the harness's test red (record it in gate 1's table). Where the harness prepares a
  paired graph and the witness is `Some`, assert that it is `selected_index`'s `PostFader`. The
  witness is reliable inside the harness (Hazards), so the per-test fallback does not apply.
- **D4. The `NonadjacentTrackA` output track (M5).** Find a test whose outcome depends on it. If
  there is one, make it assert the output track by literal ID as D2 does. If there is none, delete
  the arm under D1.
- **D5. `cfg` arms.** #1235 made these matches compile under both feature sets. Keep that: after
  D3 and D4, `cargo clippy -p builtins-compiler --all-targets -- -D warnings` (no features) and the
  test-support build stay free of dead-code warnings.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs`: the test module only (the harness functions
  `track_graph_variant_with_route_transform` and `prepared_pair_graph_variant*`,
  `BoundaryVariant`, and the tests that use a nonadjacent variant).
- This spec

## Non-goals

- Production code, and any test outside the nonadjacent split-pair group.
- Other harness variants.

## Hazards

- `crates/builtins-compiler/src/lib.rs` is a hot file (`STREAMS.md`). This issue edits its test
  module only; the slice that lands second rebases.
- `PAIR_WITNESS_LOCK` serialises the witness tests. The harness cannot take it itself: its callers
  already hold it, and the lock is not re-entrant. The witness is thread-local and
  `graph::build_sequential` resets it on every build (`crates/graph/src/runtime.rs:5723`), so the
  harness's read is race-free under the caller's lock. Record this in the Evidence.
- A deleted arm can change a fixture's track choice, so a render comparison can move. Each test
  compares a paired graph with its separate-owner twin built from the same variant, so both move
  together; a test that compares against a pinned value is a stop-and-report.

## Objective gates

1. **Every mutant is red, or its arm is gone (PR evidence).** For each of M1, M3, M5 and M7 that
   still has an arm, apply it alone and run
   `cargo test --locked -p builtins-compiler --features test-support --lib` (and, for M5 and M7,
   `cargo test --locked -p builtins-compiler --lib`): at least one test fails. Record the failing
   test names, or the deletion and its reason.
2. **The suite passes in both configurations.**
   `cargo test --locked -p builtins-compiler --features test-support` and
   `cargo test --locked -p builtins-compiler`.
3. **No dead code in either configuration.**
   `cargo clippy --locked -p builtins-compiler --all-targets -- -D warnings` and
   `cargo clippy --locked -p builtins-compiler --all-targets --features test-support --
   -D warnings`.
4. `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   `bash scripts/check-builtins-policy.sh`; `bash scripts/check-workspace-policy.sh`.

*Test value.*
- D2's assertion is red if the harness wires the `Output` from the wrong track for
  `NonadjacentOutputConflict` (M1), which makes the #916 test pass without exercising the case it
  names; no other test reads that edge.
- D3's assertion is red if the harness's "A" track and production's selected pair disagree (M3,
  M7), which today no assertion can see because both orientations satisfy the slot checks.

## Evidence

- Gate 1's table: per mutant, the failing tests or the arm's deletion and reason.
- Where D3's assertion lives (harness or tests), and why; whether `selected_index` is read from an
  observation or re-derived (D3), and if re-derived, the production-rule mutant and its red test.
- The `PAIR_WITNESS_LOCK` note (Hazards).

## Amendment 1 (root, 2026-10-05)

Attempt 1 stopped before committing: D3 as written (fix the `NonadjacentOutputConflict` arm and
keep the default `n - 1` for `Nonadjacent`) makes two tests red,
`actual_scalar_nonadjacent_schedule_selects_the_split_owner` and
`actual_scalar_nonadjacent_intervening_observer_error_completes_and_retries`, because production
selects t00 for an unmetered `Nonadjacent` graph. Root ruled option A: `selected_index` comes from
production (observed where possible, else the production meter rule with a mutant that turns the
harness red), and both per-variant arms go. D2, D4 and D5 are unchanged. The stopped run consumed
no verdict; the next implementation is attempt 1.

## Dependencies

- None. *Remove the dead code builtins-compiler reports under no-features clippy* (#1235) is on
  `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.

## Attempt 1 record (implementer, 2026-10-05)

**D3: observed, not re-derived.** Production's selection is observable: the thread-local witness
`graph::test_only_selected_split_fader()` records the fader of the split pair `build_sequential`
admits, and builtins-compiler's dev-dependency on `graph` enables it in both feature sets. So under
Amendment 1 the harness reads it and re-derives nothing; no production-rule mutant applies. The
nonadjacent slot assertions in `prepared_pair_graph_variant_observed` move after `into_bound`: the
harness keeps a copy of the schedule it built, reads the witness, maps its node to the harness
track whose `PostFader` it is (a panic if it is no harness track's post-fader), and runs the slot
assertions on that track. D3's "the witness is `selected_index`'s `PostFader`" becomes exactly that
mapping, since `selected_index` now comes from the witness. The harness also asserts that a
nonadjacent scalar preparation has a witness exactly when it is the paired one
(`between_render_calls`): the separate-owner twin selects no split pair, so its witness is `None`
and it has no selected track to check. The adjacent (track-major) branch keeps its one adjacency
assertion on the last track, now named `n - 1` literally instead of through `selected_index`.

**`PAIR_WITNESS_LOCK` note (Hazards).** The witness is thread-local and `graph::build_sequential`
resets it on every build (`crates/graph/src/runtime.rs:5723`), so the read right after the
harness's own `into_bound` describes that binding only. The harness does not take
`PAIR_WITNESS_LOCK`: its callers hold it and it is not re-entrant. Recorded in the harness comment.

**D2.** The #916 test asserts, by literal ID, that the `Output` edge's source in
`track_graph_variant(2, _)` is t00's `PostMatrix` for `Nonadjacent` and t01's for
`NonadjacentOutputConflict`.

**D4.** No test outcome depends on `NonadjacentTrackA`'s output track (M5 green on `main`; since
#916 the `Output` is dedicated storage), so its arm is deleted and the variant takes track 0. No
render comparison moved: every affected test compares a paired graph with its twin from the same
variant. The comment in the TrackA post-matrix capture test that relied on the old output track is
reworded. **D5.** `output_track` is two `cfg`'d `let`s (the conflict variant exists only under
test-support), and the nonadjacent predicate is one `match` computed once; both clippy runs are
clean.

**Gate 1.**

| Mutant | Result |
| --- | --- |
| M1 (`NonadjacentOutputConflict` output `n - 1` -> `0`) | red: `actual_scalar_nonadjacent_output_track_takes_the_split_pair_now_the_output_is_dedicated` (test-support lib run: 53 passed, 1 failed) |
| M3 (`selected_index` arm for `NonadjacentOutputConflict`) | arm deleted: `selected_index` is read from production's witness (D3, Amendment 1) |
| M5 (`NonadjacentTrackA` output `n - 1`) | arm deleted: no outcome reads it (D4) |
| M7 (`selected_index` arm for `NonadjacentTrackA`) | arm deleted: as M3 |

Candid note on test value: with `n == 2` and a stage-major schedule the slot assertions hold for
either orientation, so they still cannot tell t00's pair from t01's. Which pair production selects
stays defended by the tests that assert the witness by literal ID
(`actual_scalar_nonadjacent_schedule_selects_the_split_owner`,
`actual_scalar_overlapping_nonadjacent_candidates_select_one_and_keep_the_other_separate`, the #916
test). The harness change removes the misleading per-variant choice rather than adding a catch.

**Gates 2-4 (all pass).** `cargo test --locked -p builtins-compiler --features test-support` and
without features; both `cargo clippy --locked -p builtins-compiler --all-targets` runs with
`-D warnings`; `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets
--all-features -- -D warnings`; `scripts/check-builtins-policy.sh`;
`scripts/check-workspace-policy.sh`; `scripts/check-realtime-policy.sh`.
