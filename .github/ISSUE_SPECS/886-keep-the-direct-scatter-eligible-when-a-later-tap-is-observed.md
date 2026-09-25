# Keep the direct scatter eligible when a later tap is observed

## Product outcome

`scatter_target` lets a full bank scatter straight into its consumers' buffers (delivered by #202/#399), but declines when the producer or any later tap has an observer, so metered sessions fall back to scatter-then-copy. Let observers that only read the final lane words keep the direct scatter. Class A.

## Root evidence

- `crates/graph/src/runtime.rs:4494` `scatter_target` declines when the producer or any later tap is observed (`observed`, `:4764`); the fallback path scatters into `staging_*` and copies per lane (`crates/rack/src/lib.rs` `scatter_tiled` near `:2666-2791`).
- An observer of the post-matrix boundary can read `BankChain::final_output_lane` (`crates/rack/src/lib.rs:2061`) before or after the direct scatter with identical words.

## Smallest closable slice

Authorized paths: `crates/graph/src/runtime.rs`, `crates/rack/src/lib.rs` (accessors only), their tests, and this spec.

Treat an observer whose tap is the chain's final output as not blocking `scatter_target`, and serve it from the resident lane. Observers at intermediate taps keep the current behaviour. No kernel change.

## Non-goals

No change to the fold (separate issue), to partial-bank scatter, or to observation cadence.

## Objective gates

1. New test: a metered full bank binds with direct-scatter targets (assert on the prepared plan).
2. New test: master output and meter snapshots bit-identical to the same plan with direct scatter forcibly disabled.
3. `cargo test -p graph -p rack` green; `scripts/check-graph-determinism.sh`, `scripts/check-graph-policy.sh`, `scripts/check-rack-policy.sh`, `scripts/check-realtime-policy.sh` pass.

## Dependencies

Do after "Keep the route fold eligible when the folded lane is observed"; both touch `observed` call sites and the second should reuse the first's resident-lane observer path.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.

## Attempt 1 evidence

**Scope amendment (Sol).** After the first evidence commit, Sol authorized, in addition to the
original paths: (1) `crates/graph/src/lib.rs`, to export `test_only_set_scatter_redirect_declined`
in the existing `test-support` block, with the seam's cfg widened to
`any(test, feature = "test-support")` (still read once per bind only); (2)
`crates/graph-compiler/src/lib.rs`, test code only, to rewrite
`a_meter_on_a_bank_member_declines_that_lanes_scatter_redirect` against a redirect-declined arm
with the production meter, and to rename and re-describe
`an_observed_alias_on_the_last_slot_declines_that_lanes_scatter_redirect` without weakening it;
(3) a graph test for the scalar split fader/matrix interaction; (4) the `apply_scatter_redirects`
fix kept as its own commit, with the paragraph below; (5) this record.

Implementer: Terra, branch `codex/886-direct-scatter-under-observation` (from the verified #885 tip
`a90106e4`). Commits:

- `e8cab034`: the pre-existing indexing defect, fixed on its own (see "Pre-existing defect fixed").
- `7239d052`: this issue's change and gates 1 and 2.
- `174d609f`: the first version of this record.
- `b0b85957`: the amendment's code: seam export, the two graph-compiler tests, the split-pair test.
- the commit carrying this revision of the record.

Files: `crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs` (one export line),
`crates/graph-compiler/src/lib.rs` (test code only), and this spec. `crates/rack` needed nothing.

Anchor drift: on the base, `scatter_target` is at `runtime.rs:4579` (spec `:4494`) and `observed`
at `:4852` (spec `:4764`); the spec predates #885. The `crates/rack` anchors (`final_output_lane`
`:2061`, `scatter_tiled` `:2666`) match.

### Finding: a post-matrix observer never reached the clause

`scatter_target` has something to redirect only when the lowering refused to run the last slot's
sole reader in place on its buffer; for a reader with one undelayed input and no sidechain (which
`scatter_target` requires) that means one end is dedicated storage (`program::is_dedicated`: the
SIMD-rack effects and `PostInputBuiltins`). Otherwise the reader writes the last slot's own buffer
and the "not already in place" early-out returns before the observer clauses. A `PostMatrix` slot
is not dedicated and feeds only routes, which are not dedicated either, so a post-matrix meter
never declined a redirect, and reusing #885's `served_by_the_resident_lane` (post-matrix only)
verbatim would have changed nothing on any constructible plan. Gate 1 pins that (the #885 shape
binds `[0 redirects, 0 folds]` with the fold declined). The observers the two clauses really
declined sit at the final output of a chain whose consumer cannot run in place on it: in practice a
`PostInputBuiltins` meter, or a meter at an elided rack boundary (`PostSimd1`, `PostDynamic`,
`PostSimd2PreFader`) aliasing the last slot's buffer. All of them read the chain's final output
words.

### Design

Both observer clauses are removed from `scatter_target` (and the `spec`/`observers` parameters
from it and `scatter_redirects`). The reasoning they encoded -- "the last slot's own buffer goes
unwritten, so its observers read the previous block" -- was never true of the observers: every
observer of the producer (direct, or through a `program::Tap`) is attached to the last slot's own
`RuntimeOp` and dispatched from it, and `apply_scatter_redirects` repoints that op's `output` at
the consumer's buffer. Both dispatchers run it straight after the chain's unit (`lib.rs` render
loop: `execute(unit)` then `observe_unit(unit)` / `observe_active_unit(unit)`), i.e. after the
scatter wrote the consumer's buffer and before the consumer's own unit rewrites it in place. That
unit is strictly later: the whole chain renders as one unit at its first op's position, and the
consumer is a plain op after it (it reads the last slot). At that point:

- the resident offer (`BankChain::final_output_lane`, unchanged; the dispatcher offers it to every
  final-slot observer of a full-population lane shape, redirected or not) is the words the scatter
  transposed;
- an observer that declines it reads `member.output`, which is the consumer's buffer holding those
  same words; `scatter_redirects`' in-between clause already keeps that physical slot the
  scatter's alone from the chain's first op to the consumer's.

So no dispatcher, kernel, observation cadence or meter arithmetic changes, and no copy is added:
#885's `write_resident_lane` is not needed (the redirected scatter already wrote the words where the
observer reads them; calling it would add a copy). What is reused from #885 is the render-side
path as it stands -- `observe_unit`/`observe_active_entry` offering `final_output_lane` -- and the
seam pattern. The `observed(.., served)` predicate is not used by `scatter_target` any more: every
observer it inspected reads the final output, so any `served` set that is sound excuses all of
them and the call would be an always-false guard. `chains_into` (observers at a chain's
*intermediate* boundaries) is untouched and still splits the chain there.

Composition with #885 on a metered bank: a lane is folded, redirected, or neither -- never both
(`build_sequential` still filters folded runs out of the redirects). A post-matrix meter keeps the
fold (#885) and has no redirect to keep; a meter at a dedicated last slot or its later tap still
declines the fold (unchanged: #885 excuses post-matrix only) and now keeps the direct scatter.
Gate 1's route-consumer arms pin exactly that hand-over.

Composition with the scalar split fader/matrix pair (amendment item 3): `build_sequential` keeps a
redirect consumer out of the split pair, so a metered plan now changes shape -- **yes, explicitly:
a fully metered plan whose fader consumes a redirect used to take the split pair (the meter declined
the redirect and freed the fader) and now takes the redirect**. That is the shape the same plan
takes unmetered, before and after this issue; before #886 the metered and unmetered plans differed
in shape, and now they do not. The bits are the same either way
(`a_metered_redirect_consumer_stays_out_of_the_split_pair_and_keeps_the_bits`).

### Pre-existing defect fixed (`e8cab034`)

- **Symptom.** `apply_scatter_redirects` indexed `units` by run index, but the route fold retires
  route runs without emitting units for them, so every run after a retired run sits at a lower unit
  index. A chain that redirects its scatter after a folded chain either panicked at bind ("index out
  of bounds: the len is 16 but the index is 16" on the regression shape) or, with more units after
  it, repointed a member of whatever unit sat at that index (wrong audio, and the redirect counter
  still reporting the lane). Fix: map the run through `unit_of_run`, as `arm_resident_inputs`
  already does; plans with no retired run are untouched, since `unit_of_run` is the identity there.
- **Minimal graph shape.** Four tracks `Input -> PostMatrix (one W4 builtin bank) -> Route -> bus`,
  with the routes summed at a bus node, then `bus Input -> PostInputBuiltins (a one-lane builtin
  bank) -> PostFader (bound) -> Route -> Output`. The tracks' routes fold into the bus input (four
  retired runs), and the bus bank, rendered after them, redirects its dedicated last slot's scatter
  into the fader. Three dangling pad strips (`Input -> PostFader`, bound, unread) are needed only so
  the colouring does not put the session output on a track's slot, which would decline the fold.
- **Can a compiled session reach it?** Not demonstrated, and I believe not on any checked-in
  fixture: there the only redirecting chains are the post-input builtin banks of the bank-free
  (per-node effects) arm, and every track's post-input stage sits at the first bank level, before
  every route, so no redirecting run follows a retired one; graph-compiler and console-workload
  passed with the defect in place. It needs a bank chain rendered *after* a folded track's route
  whose last slot is dedicated storage feeding a plain op. Session V1 submixes are plain summing
  nodes with no strip, so that chain would have to be another track whose strip is deeper than the
  folded track's route level (the scheduler is level-major) and whose chain ends at a SIMD-rack
  effect or post-input bank feeding an unbanked op, while every master contributor still folds. I
  did not construct such a session and cannot rule it out.
- **Regression test.** `runtime::tests::a_redirect_after_a_retired_route_lands_on_its_own_chain`
  (folded arm `[1 redirect, 4 folds]`, declined-fold oracle `[1, 0]`, master bits equal over 6
  blocks); restoring `units[run]` panics it at bind.

### Tests (`crates/graph/src/runtime.rs` unless stated)

- `an_observer_of_the_scattered_lane_keeps_the_direct_scatter_armed` (gate 1). Shape:
  `Input -> PostInputBuiltins (builtin bank) -> PostSimd1 (elided) -> PostFader (bound, non-identity)
  -> Route -> Output`, W4 x 4 and W8 x 8. Asserts `[bank_scatter_redirects, bank_route_folds]` on
  the bound plan: unmetered `[tracks, 0]`; metered at the last slot's node and its later tap, each
  alone, through the controlled catalog, with resident-declining observers, and at the consumer's
  own node: all `[tracks, 0]`; the seam `[0, 0]`; route consumer unmetered `[0, tracks]` (the fold
  takes every lane), metered `[tracks, 0]` (the fold declines, the direct scatter takes every lane);
  #885's post-matrix shape with the fold declined `[0, 0]`.
- `a_redirected_metered_plan_is_the_unredirected_plans_master_and_meters_bit_for_bit` (gate 2).
  Two meters per track (last slot's node + later tap), 6 blocks of seeded noise, a non-identity
  bank, per-track route 2x2s; consumers fader and route; shapes W4 x 4 (13 frames), W8 x 8 (13),
  two W4 cohorts (16) and W4 x 6 with a partial second cohort (5). Oracle: the same plan with every
  redirect declined through the seam and every observer declining the resident view (the words the
  unredirected scatter wrote into the last slot's own buffer), checked equal first to the same
  unredirected plan with resident meters. Five candidate arms (resident meters, declining observers,
  resident offer withdrawn, and both dispatch modes under controlled activation) each assert the
  unmetered plan's redirect count (with the fold declined), no fold, no split pair, the exact
  `[planar, offered, accepted]` counts, the master bits, and every published meter frame (words,
  peak, energy). Redirected lanes per shape: 4, 8, 4 (second cohort; the first declines on the
  in-between clause, metered or not), and 4 (two lanes of each cohort, so the partial bank's
  per-lane scatter is redirected under metering too). The comparison is the helper
  `assert_redirected_meters_are_the_declined_plans`, shared with the next test.
- `a_metered_redirect_consumer_stays_out_of_the_split_pair_and_keeps_the_bits` (amendment item 3).
  Gate 2's shape plus a bound scalar `PostMatrix` after each fader and a fader owner that offers the
  scalar split pair (`SplitFader`, whose `DeferredSplit` owner defers the fader's arithmetic to the
  matrix slot). W4 x 4 and W8 x 8, 13 frames. **Result, on the bound plan
  (`[redirects, folds]`, selected split fader):** unmetered `([tracks, 0], None)`; metered at the
  last slot and its later tap `([tracks, 0], None)` -- the redirect, the unmetered shape; the
  declined oracle `([0, 0], Some(track00/PostFader))` -- the split pair. Then the gate-2
  comparison against that oracle, which renders through the split pair: all five metered,
  redirected arms render the master and every meter frame bit for bit as it (pass).
- `a_redirect_after_a_retired_route_lands_on_its_own_chain` (regression for the defect above).
- graph-compiler `a_meter_on_a_bank_member_keeps_that_lanes_scatter_redirect` (renamed from
  `a_meter_on_a_bank_member_declines_that_lanes_scatter_redirect`): the bank-free arm of the
  64-track intended strip with a production `MeterObserver` at ch63's `PostInputBuiltins`, 12
  blocks. **Final assertions:** `quiet_redirects > 0`; metered `metered_redirects ==
  quiet_redirects` (was `quiet_redirects - 1`); meter input counts `[0, 12, 12]` (the resident view
  on every block); `metered_pcm` bit-equal to `quiet_pcm`; windows non-empty and carrying signal
  (both kept); declined oracle (every redirect declined through the seam, resident offer withdrawn so
  the meter reads the last slot's own buffer): 0 redirects, counts `[12, 0, 0]`, master bit-equal to
  the metered arm, and `frames == declined_frames` (whole `MeterSnapshot` vectors); the same oracle
  with the resident view: master equal and windows equal to the planar oracle's; offer withdrawn on
  the redirected plan: `quiet_redirects` redirects, counts `[12, 0, 0]` (the planar read of the
  redirected member output, i.e. the EQ's buffer), master equal, and windows equal to the
  oracle's. This is the one place the production meter runs on a redirected lane.
- graph-compiler `a_pre_fader_meter_splits_its_cohorts_chain_and_reads_the_limiter` (renamed from
  `an_observed_alias_on_the_last_slot_declines_that_lanes_scatter_redirect`). **Final
  assertions, unchanged in substance:** slots `STRIP_SLOTS_PER_COHORT * cohorts`; chains `cohorts +
  1` (ch00's cohort splits at the observed alias, through `chains_into`); transposes
  `BLOCKS * chains`; redirects `0` (message now: the split chain feeds a banked fader and every
  other chain ends in a buffer its consumer reads in place); master bit-equal to the per-node-effects
  arm; windows non-empty, equal to that arm's, and carrying signal. The prose now says that since
  #212 the alias sits inside the chain, `chains_into` is the clause that answers, and since #886
  `scatter_target` has no observer clause at all. The reference to it in
  `a_send_from_the_last_slots_alias_declines_that_lanes_scatter_redirect` is renamed with it.
- Fixture: #885's `FoldFixture`/`fold_fixture` gain `alias`, `fader`, `meter_at`,
  `scatter_declined` and `split_pair`; their defaults reproduce the #885 graph, handles and binding
  exactly (#885's tests pass unchanged). New test types `ScalarTilt`, `SplitFader`, `MatrixTilt`,
  `DeferredSplit`, `folded_bus_plan`, `scatter_shape`, `selected_split_fader`.
- Seam `test_only_set_scatter_redirect_declined(bool)`: a thread-local under
  `cfg(any(test, feature = "test-support"))`, exported `#[doc(hidden)]` from `crates/graph/src/lib.rs`
  beside `test_only_set_route_fold_declined`; read once per bind in `build_sequential`, where the
  redirects are decided and before the split-pair passes and any unit is built; render never reads
  it.

### Mutation controls (each applied to a copy, run, restored)

| Mutation | Result |
|---|---|
| Restore the producer-node observer clause | Gate 1 red; placement probe: last-slot-node meters `[0, 0]`, later-tap and consumer meters still `[4, 0]`. Gate 2 red on the redirect count. graph-compiler `a_meter_on_a_bank_member_keeps_that_lanes_scatter_redirect` red: "ch63's lane keeps its redirect" (left 55, right 56). |
| Restore the alias (later-tap) clause | Gate 1 red; probe: later-tap meters `[0, 0]`, last-slot-node and consumer meters `[4, 0]`. Gate 2 red on the count. |
| Scatter into the consumer but leave `member.output` on the last slot's own buffer | Gate 2 red at "declining observers: every meter frame" with the master equal (observers isolated). graph-compiler test red at "a planar read of a redirected lane is the unredirected plan's window", after the master and resident-window checks passed. |
| Move a redirected member's observers onto the consumer's op (they run after it) | Gate 2 red at the counts; with the counts check disabled, red at "resident meters: every meter frame" with the master equal. |
| Hand the resident view lane `(lane + 1) % population` | Gate 2 red at the unredirected resident control; with that check disabled, red at the redirected "resident meters" frames. #885's gate 2 also red. graph-compiler test red at "every meter window of the redirected plan is the unredirected plan's". |
| Drop the redirect-consumer exclusion from the split pass | Split-pair test red: unmetered arm `([4, 0], Some(track00/PostFader))`, expected `([4, 0], None)`. |
| Make `DeferredSplit` drop the deferred fader (the oracle's split path) | Split-pair test red at "resident meters: the redirected master is the unredirected master's bits" -- the oracle really renders through the split pair. |
| Index `units[run]` in `apply_scatter_redirects` again | `a_redirect_after_a_retired_route_lands_on_its_own_chain` panics at bind: "index out of bounds: the len is 16 but the index is 16". |

### Gates (at `b0b85957`)

1. Pass (gate 1 test above).
2. Pass (gate 2 test above, and the split-pair test on the same comparison).
3. `cargo test -p graph`: lib 87 passed, `rt1_direct_bank_alloc` 1, `rt9_resident_bank_input_alloc` 1.
   `cargo test -p graph --features test-support`: lib 87, rt1 1, rt9 8. `cargo test -p rack`:
   38 + 10 + 4. `scripts/check-graph-determinism.sh`: `PASS (100/100)`;
   `scripts/check-graph-policy.sh`: `PASS`; `scripts/check-rack-policy.sh`: `PASS`;
   `scripts/check-realtime-policy.sh`: `ok (50 marked regions in 14 files)` (all exit 0;
   `scripts/check-lane-policy.sh` `ok` at `7239d052`, and the amendment touches no lane code).

| Command | Result |
|---|---|
| `cargo test -p graph-compiler --no-fail-fast` | lib 73 passed; other targets 1, 3, 1, 8, 6 passed |
| `cargo test -p console-workload --no-fail-fast` | 4 + 21 + 3 passed |
| `cargo fmt --all --check` | pass |
| `cargo clippy --locked -p graph -p graph-compiler --all-targets --all-features -- -D warnings` | exit 0 (only the pre-existing `clippy.toml` "does not refer to a reachable function" notices for `math::fast_db`, printed on any `-p` run) |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0, no warnings |

At the fix commit `e8cab034` alone: graph-compiler lib 73/73, console-workload `chain_shape` 21/21.

No benchmark row is listed; none was run and no saving is claimed.

### Deviations and notes for review

1. **`served_by_the_resident_lane` / `write_resident_lane` not reused** (the coordinator asked for
   reuse): reusing the post-matrix predicate is a no-op (finding above), and the resident-lane write
   is unnecessary on a redirected lane. The render-side resident offer is reused unchanged.
2. **The indexing fix is outside the original slice** (same file); it is its own commit, `e8cab034`,
   for the coordinator to track separately.
3. **Stale mutation-log name (not edited; path not authorized):** `crates/graph/tests/MUTATIONS.md`
   row 218-3 still names `an_observed_alias_on_the_last_slot_declines_that_lanes_scatter_redirect`
   as a catcher; the test is now `a_pre_fader_meter_splits_its_cohorts_chain_and_reads_the_limiter`.
   The `route_fold` ledger in `runtime.rs` records both names. Historical files under `artifacts/`
   keep the old names, as recorded output.
4. The production change moves the web AudioWorklet binary, so
   `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` will need the batch-boundary
   repin; untouched here, as directed.

## Sol attempt 1 verdict: PASS

Adversarial review (Fable 5.1, high effort) against `e8cab034`, `7239d052`, `174d609f`,
`b0b85957` and `966d0a7b` on base `a90106e4` (the verified #885 tip). No blocking findings.
Independently verified: every observer that can attach to a chain's last slot (direct, alias tap
for the three elidable stages, spectrum capture, controlled entries) reads either
`final_output_lane` or the op's `output`, which `apply_scatter_redirects` repoints, and observers
bind only to track stages so submix nodes and routes are never observed; the render loop runs
`execute(unit)` then the unit's observation with nothing between, a redirect consumer is never a
bank member, never in the same unit, and always later, and folded, PDC-staged, sidechained,
split-pair and fan-in consumers are all excluded before the question arises; the split-pair
exclusion is pre-existing and taking the redirect selects the unpaired fader-then-matrix path the
production split owner is itself held bit-equal to; the `apply_scatter_redirects` fix is the
identity mapping whenever no run is retired, so it cannot change a plan that previously bound, and
no Session V1 fixture reaching the defect could be constructed either way (routes go only to
submix or output inputs, and submixes have no strip). Five mutations re-applied and reverted, two
of the reviewer's own (observe a unit only after the next unit executes; drop the folded-run
filter), each red on the named tests.

Recorded, no code change: the split-pair interaction test drives settled coefficients only, and
no ramping test is owed by this issue because the redirect selects the reference path. Mutation
log row 218-3 is renamed in this commit to the test's new name. The pre-existing bind-time defect
in `apply_scatter_redirects` gets its own tracking issue from the coordinator so its fix commit
is not only a paragraph here.

Reviewer-run gates, all green: `cargo fmt --all --check`; `cargo test -p graph` (87/1/1 and, with
`test-support`, 87/1/8); `-p rack`; `-p graph-compiler`; `-p console-workload`;
`check-realtime-policy.sh`; `check-graph-policy.sh`; `check-rack-policy.sh`;
`check-lane-policy.sh`; `check-graph-determinism.sh` (100/100). Workspace clippy not re-run; the
evidence's run at `b0b85957` covers every code line.
