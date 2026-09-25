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

Implementer: Terra, branch `codex/886-direct-scatter-under-observation` (from the verified #885 tip
`a90106e4`). Commits: `e8cab034` (a pre-existing indexing defect, fixed first and separately; see
deviation 1), `7239d052` (this issue's change and gates), and the commit carrying this record.
Only `crates/graph/src/runtime.rs` and this spec changed; `crates/rack` needed nothing.

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

### Tests added (`crates/graph/src/runtime.rs`)

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
  unmetered plan's redirect count (with the fold declined), no fold, the exact
  `[planar, offered, accepted]` counts, the master bits, and every published meter frame (words,
  peak, energy). Redirected lanes per shape: 4, 8, 4 (second cohort; the first declines on the
  in-between clause, metered or not), and 4 (two lanes of each cohort, so the partial bank's
  per-lane scatter is redirected under metering too). The graph crate has no builtin meter, so the
  frame is a test meter's, as in #885.
- `a_redirect_after_a_retired_route_lands_on_its_own_chain` (regression for deviation 1).
- Fixture: #885's `FoldFixture`/`fold_fixture` gain `alias`, `fader`, `meter_at` and
  `scatter_declined`; their defaults reproduce the #885 graph, handles and binding exactly (#885's
  tests pass unchanged). New test types `ScalarTilt`, `folded_bus_plan`, `scatter_shape`.
- Seam `test_only_set_scatter_redirect_declined(bool)`: a thread-local read once per bind in
  `build_sequential`, where the redirects are decided and before any unit is built; render never
  reads it. `#[cfg(test)]` only (deviation 3).

### Mutation controls (each applied to a copy, run, restored)

| Mutation | Result |
|---|---|
| Restore the producer-node observer clause | Gate 1 red; placement probe: last-slot-node meters `[0, 0]`, later-tap and consumer meters still `[4, 0]`. Gate 2 red on the redirect count. |
| Restore the alias (later-tap) clause | Gate 1 red; probe: later-tap meters `[0, 0]`, last-slot-node and consumer meters `[4, 0]`. Gate 2 red on the count. |
| Scatter into the consumer but leave `member.output` on the last slot's own buffer | Gate 2 red at "declining observers: every meter frame" with the master equal (observers isolated). |
| Move a redirected member's observers onto the consumer's op (they run after it) | Gate 2 red at the counts; with the counts check disabled, red at "resident meters: every meter frame" with the master equal. |
| Hand the resident view lane `(lane + 1) % population` | Gate 2 red at the unredirected resident control; with that check disabled, red at the redirected "resident meters" frames. #885's gate 2 also red. |
| Index `units[run]` in `apply_scatter_redirects` again | `a_redirect_after_a_retired_route_lands_on_its_own_chain` panics at bind: "index out of bounds: the len is 16 but the index is 16". |

### Gates

1. Pass (gate 1 test above).
2. Pass (gate 2 test above).
3. `cargo test -p graph -p rack`: graph lib 86 passed, `rt1_direct_bank_alloc` 1, `rt9_resident_bank_input_alloc` 1;
   rack 38 + 10 + 4. `cargo test -p graph --features test-support`: 86, 1, 8.
   `scripts/check-graph-determinism.sh`: `PASS (100/100)`; `scripts/check-graph-policy.sh`: `PASS`;
   `scripts/check-rack-policy.sh`: `PASS`; `scripts/check-realtime-policy.sh`: `ok (50 marked regions in 14 files)`;
   `scripts/check-lane-policy.sh`: `ok` (all exit 0).

Workspace: `cargo fmt --all --check` passes; `cargo clippy --locked --workspace --all-targets
--all-features -- -D warnings` exits 0 with no warnings.

Out-of-crate suites:

| Suite | At `7239d052` | At `e8cab034` (fix only) |
|---|---|---|
| `cargo test -p console-workload` | 4 + 21 + 3 passed | `chain_shape` 21 passed |
| `cargo test -p graph-compiler --no-fail-fast` | lib 72 passed, **1 failed**; other targets 1, 3, 1, 8, 6 passed | lib 73 passed |

The failure is `graph-compiler` `a_meter_on_a_bank_member_declines_that_lanes_scatter_redirect`:
`metered_redirects == quiet_redirects - 1` (left 56, right 55). It pins exactly the rule this
issue removes (a `PostInputBuiltins` meter on ch63 of the bank-free arm declining that lane), so it
was left red for a scope amendment rather than edited. Its other assertions (`metered_pcm ==
quiet_pcm`, windows published with signal) come after the count and were not reached.

No benchmark row is listed; none was run and no saving is claimed.

### Deviations and requests

1. **Pre-existing defect fixed in `e8cab034` (same file, not in the slice).** `apply_scatter_redirects`
   indexed `units` by run, but the route fold retires route runs without emitting units, so any
   chain that redirects after a folded chain panicked at bind or (with more units following)
   repointed another unit's member. This issue lets observed last slots redirect and so would have
   widened that defect's reach to metered plans; it is fixed first, in its own commit, by mapping the
   run through `unit_of_run` (as `arm_resident_inputs` already does), with the regression test
   above. Plans with no retired run are untouched (`unit_of_run` is the identity there), and at the
   fix commit alone graph-compiler (73/73) and `chain_shape` (21/21) stay green. Reachability from a
   compiled session is not established; the graph-level shape needs a fold into a bus followed by a
   bus bank with a dedicated last slot. It can be reassigned to its own issue by moving that commit.
2. **`served_by_the_resident_lane` / `write_resident_lane` not reused** (the coordinator asked for
   reuse): reusing the post-matrix predicate is a no-op (finding above), and the resident-lane write
   is unnecessary on a redirected lane. The render-side resident offer is reused unchanged.
3. **Seam is `#[cfg(test)]`, not `test-support`.** A `test-support` export needs one line in
   `crates/graph/src/lib.rs` (outside the slice); without it a `test-support` build would carry an
   unused function. Needed only if the graph-compiler test below should use it.
4. **Requested amendment (not done):** `crates/graph-compiler/src/lib.rs`,
   `a_meter_on_a_bank_member_declines_that_lanes_scatter_redirect` -- suggested new shape: rename to
   keep-the-redirect, assert `metered_redirects == quiet_redirects`, and compare the metered arm's
   `MeterSnapshot` windows with the same session bound with the redirect declined (needs the
   deviation-3 export), which would put the production `MeterObserver` on the redirected lane. The
   stale prose in `an_observed_alias_on_the_last_slot_declines_that_lanes_scatter_redirect` (its
   name and "the meter would read the previous block's words") no longer describes a rule; the test
   still passes, because its chain split leaves no redirect to decline.

Risks and notes for review:

- A redirect consumer is excluded from the scalar split fader/matrix pairing. A metered plan whose
  chain ends at a dedicated slot feeding a scalar fader could now take the redirect where it
  previously could take the split pair. The split pair is an existing path meant to preserve bits;
  this interaction was not exercised by any test run here.
- The production change moves the web AudioWorklet binary, so
  `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` will need the batch-boundary
  repin; untouched here, as directed.
