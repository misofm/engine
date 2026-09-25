# Keep the route fold eligible when the folded lane is observed

## Product outcome

The fused route fold (delivered by #218/#419) removes a whole `mix2x2` pass and a whole master-reduction pass per lane, but `foldable_lane` declines whenever the chain's last slot or the route is observed. With a meter on every track the fold never fires. Let a post-matrix observer read the lane's resident output and keep the fold armed. Class A.

## Root evidence

- `crates/graph/src/runtime.rs:4826` `foldable_lane` returns `None` when `observed(...)` (`:4764`) is true for the chain's last slot or the route; the resident-observe branch in `observe_unit` (`:1915`, conditions near `:1930-1943`) and `observe_active_entry` (near `:2117-2133`) require `fold.is_empty() && chain.fold_lanes().is_empty()`.
- The fold consumes the scatter staging tile in `fold_plane`/`fold_cohort` (`:1218-1305`); the resident AoSoA scratch is untouched by it, and `BankChain::final_output_lane` (`crates/rack/src/lib.rs:2061`) already exposes the lane's final words.
- Bit-identity gate already exists: `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit` (`runtime.rs:7040`).

## Smallest closable slice

Authorized paths: `crates/graph/src/runtime.rs`, `crates/graph/src/observation_activation.rs` (only if the activation entry needs a resident-lane variant), `crates/rack/src/lib.rs` (only `final_output_lane` or a sibling accessor), their tests, and this spec.

Allow `foldable_lane` to return a fold when the only observers on the last slot / route read the post-matrix boundary, and make those observers read `final_output_lane` (the words the fold will mix) instead of the member buffer. Observers at other taps keep the current behaviour. Do not change the fold kernel.

## Non-goals

No change to scatter redirect eligibility (separate issue), to meter arithmetic, or to observation cadence.

## Objective gates

1. New test: a plan with a `PostMatrix` meter on every track of a full bank binds with the fold armed (assert on the prepared plan's fold lanes).
2. New test: for random input, the master output and every published meter snapshot of that plan are bit-identical to the same plan rendered with meters bound and the fold forcibly disabled (the current path).
3. `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit` and the rest of `cargo test -p graph` pass; `scripts/check-graph-determinism.sh`, `scripts/check-graph-policy.sh`, `scripts/check-realtime-policy.sh` pass.
4. Descriptive metered console row before/after if the row exists.

## Dependencies

Measurement only: "Add a metered live-console row to the console benchmark". Independent of the web binding issue.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.

## Attempt 1 evidence

Implementer: Terra, branch `codex/885-route-fold-under-observation`. Implementation and tests in
`79ef3fd9`; this record in the commit that adds it. Only `crates/graph/src/runtime.rs` and this spec
changed: `crates/rack` (`final_output_lane` already sufficed) and `observation_activation.rs` (the
catalog's coordinates are the same folded or not) needed nothing. The spec's `file:line` anchors
matched this base exactly; no drift.

### Design

`foldable_lane` now asks `observed(.., served)`: the chain's last slot declines the fold only for an
observer bound -- directly or through a `program::Tap` alias -- to a node other than
`TrackStage::PostMatrix`; the route still declines for any observer (unchanged, and not
constructible anyway: bind admits observers on track stages and the output only). At render,
`observe_unit` and `observe_active_entry` drop their `fold.is_empty()` clauses, so a folded lane's
observers are offered `BankChain::final_output_lane` -- the resident AoSoA words, which the epilogue
never writes (it mixes a transposed copy in the staging tile). An observer that declines the
resident view (or the test-only switch that withdraws the offer) gets the lane's member buffer
written from those same words first, once per member per block (`write_resident_lane`: that lane's
share of the skipped scatter, a pure strided word copy), so its planar block is bit for bit what the
unfolded scatter left there. The fold kernel, meter arithmetic, observation cadence and scatter
redirect eligibility are untouched.

### Tests added or changed (`crates/graph/src/runtime.rs`)

- `a_post_matrix_meter_on_every_track_of_a_full_bank_keeps_the_fold_armed` (gate 1): W4 x 4 and
  W8 x 8 full banks with a `PostMatrix` meter on every track bind with `bank_route_folds() ==
  tracks`, through plain bind, the controlled-activation catalog, and with resident-declining
  observers. Controls: an unmetered `PostFader` strip folds, the same strip metered at `PostFader`
  declines (other taps keep today's behaviour), and the test-only switch declines.
- `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit` (gate 2): seeded noise
  per track and block, a non-identity bank, a distinct route 2x2 per track, 6 blocks; shapes W4 x 4
  (13 frames), W8 x 8 (13), two full W4 cohorts (16) and W4 x 6 with a partial second cohort (5).
  Oracle: the same plan bound with the fold declined and every observer declining the resident view,
  i.e. the words the unfolded scatter really wrote (checked equal to the unfolded plan with
  resident-reading meters first). Five folded arms -- resident meters, declining observers, resident
  offer withdrawn, and both dispatch modes under controlled activation -- each assert all routes
  folded, the exact `[planar, offered, accepted]` counts, the master output bits, and every
  published meter frame (the exact words plus peak and energy) equal to the oracle's. The graph crate
  has no builtin meter, so the frame is a test meter's; the builtin `MeterObserver` consumes the same
  words through unchanged arithmetic.
- Test-only seam `test_only_decline_route_fold` (`#[cfg(test)]`, read once in `preflight_sequential`
  at bind; render gains no branch).
- Updated: `resident_meter_entry_has_one_final_output_dispatch_and_admission_control` (the
  dispatcher shape gate now requires the folded-lane write and flag, with three new mutation
  controls) and `resident_meter_dispatch_preserves_binding_order_lazy_fallback_and_accepted_errors`
  (new `observe` signature only).

Measured red mutations: drop `write_resident_lane` from `observe` or from `observe_one` -> the
declining arms publish stale words; copy the wrong lane -> same; restore the `fold_lanes().is_empty()`
eligibility clause in either dispatcher -> the render fails loudly (`InvalidEnvelope`); excuse no
observer -> gate 1's metered arms stop folding; excuse every observer -> the `PostFader`-metered arm
folds.

### Gates

1. Pass (test above).
2. Pass (test above).
3. `cargo test -p graph`: lib 83 passed, `rt1_direct_bank_alloc` 1, `rt9_resident_bank_input_alloc` 1.
   `cargo test -p graph --features test-support`: lib 83, rt1 1, rt9 8. `cargo test -p rack`: all
   pass. `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit` passes.
   `scripts/check-graph-determinism.sh`: `PASS (100/100)`; `scripts/check-graph-policy.sh`: `PASS`;
   `scripts/check-realtime-policy.sh`: `ok (50 marked regions in 14 files)`;
   `scripts/check-lane-policy.sh`: `ok`.
4. Not applicable: the metered live-console row does not exist on this base (its own issue adds
   it). No benchmark was run and no saving is claimed.

Workspace: `cargo fmt --all --check` passes; `cargo clippy --locked --workspace --all-targets
--all-features -- -D warnings` exits 0 with no warnings. (Without `--all-features`,
`cargo clippy -p graph -p rack --all-targets -- -D warnings` fails on dead code in
`crates/graph/tests/rt9_resident_bank_input_alloc.rs`; that is pre-existing on the base and
unrelated.)

### Out-of-scope reds (blocking; need an amended scope)

Three tests outside the authorized paths assert the clause this issue removes, and fail at that
premise assertion before any bit comparison:

- `crates/graph-compiler/src/lib.rs` `a_meter_on_the_matrix_declines_the_route_fold_and_still_meters`
  (`assert_eq!(folds, 0, ..)`). Its remaining assertions -- master bits equal to the per-node-effects
  arm, the real post-matrix meter's windows equal to that arm's, signal present -- are this issue's
  claim with the builtin meter; the natural amendment flips the count to 64 and keeps the rest.
- `crates/graph-compiler/src/lib.rs` `the_intended_strip_folds_every_route_into_its_cohorts_epilogue`
  (`assert_eq!(unfolded_folds, 0, ..)`): its unfolded oracle was a post-matrix meter.
- `tools/console-workload/tests/chain_shape.rs` `the_folded_master_is_the_reductions_own_bits`
  (`assert_eq!(metered_runtime.bank_route_folds(), 0, ..)`): same oracle.

The last two need an unfolded oracle that no longer comes from a meter; exporting the bind switch as a
`test-support` seam from `crates/graph/src/lib.rs` would supply one. Related passing evidence:
console-workload `console_facilities_do_not_change_the_chain_shape_or_the_bits` (the 64-track console
with a post-matrix meter per track, now folded, renders the unmetered baseline's digest);
`cargo test -p builtins-compiler --features test-support` and `cargo test -p host-core --all-features`
all pass; graph-compiler 71 pass / 2 fail and console-workload `chain_shape` 20 pass / 1 fail, exactly
the three above.

### Notes for review

- The planar fallback in `observe`/`observe_one` now re-slices the member buffer per declining
  observer instead of caching the slices, because the lease must stay writable until the first
  acquisition; that is two bounds checks per declining observer and no added pass or copy on an
  unfolded lane.
- `write_resident_lane`'s `None` arm (render error) is reachable only for a chain without the
  full-population lane shape, which no rendering chain has; the eligibility mutation above shows it
  fails loudly rather than exposing stale words.
- No allocation-audit test covers the folded, observed path specifically; the new render code is in
  marked realtime regions, allocates nothing by construction, and passes the realtime policy scan.
