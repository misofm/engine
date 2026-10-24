# Guard every #962 fix at scale and linearise scatter_redirects

## Product outcome

#962 made compile and bind linear in track count (65,537 tracks: compile 164 s to 10 s, bind 983 s to 5 s). Its scale gate (`crates/graph-compiler/tests/scale.rs`) runs a session with no effects and one route at `Simd8`, so only the bank-membership scan and the bind scans that session reaches are guarded; #962's `effect_control_resource`, `Backend::Scalar` interval-scan and route-fold metadata fixes are not exercised at scale, and a regression in them would pass. The #962 verification also left `scatter_redirects` (bind) superlinear: 2.3 ms at 8,192 tracks and 47 ms at 65,537.

## Smallest closable slice

1. Add scale sessions that reach every #962 fix: effects with controls on every track, one route per track, and whatever shape reaches the remaining fixes, at `Backend::current()`; bound them so each fits CI's debug job with room.
2. Make `scatter_redirects` linear (or state why its growth is bounded) with identical results.
3. Time sources, per-track observers and meters, and sidechains at scale (#962 did not), and fix any quadratic found.

## Objective gates

- Every compiled and bound result byte-identical before and after (the #962 fingerprint and randomized probes).
- A timing table at 8,192 and 65,537 tracks for each shape.

## Amendment (root, 2026-09-29): scope extension from #1045

Exercise every per-track session feature at 65,537 tracks. That covers effects and effect
controls, one route per track, sources and source-set bind, per-track observers and meters, and
sidechains, plus console controls, sends and submixes, and automation on every track. Record
Sol's measurement from #1045 attempt 2: banks plus one route per track bind super-linearly, at 23 s
in release at 65,537 tracks (the curve is in #1045's attempt-3 evidence). Related: #1078 (host-core
preparation per PR) and #1079 (parse, protocol store, capi prepare and host-web boot, nightly).

## Decision record (attempt 1, 2026-10-24)

Scope is slice 1 of the scope note; R1-R3 are root's decisions.

- **D1 (R1).** The amendment's wider sweep (sources and source-set bind, per-track observers and
  meters, sidechains, sends and submixes, automation on every track) moves to one successor issue
  that root files: one nightly per-track session at 65,537 tracks, measured once, and one issue per
  quadratic found. #967 closes on slice 1.
- **D2 (R3).** #962 fixes 6 and 7 (`scalar_split_interval_is_clear`, `crossing_sources`,
  `ops_naming_buffers`, `plain_run_of_op`) guard code that no longer exists: #1767 removed the fused
  fader+matrix split pass and owner ruling R8 removed the whole-plan scalar backend. Nothing is left
  to guard.
- **D3.** `scatter_target`'s in-between clause no longer walks `[chain's first op, consumer)`.
  `scatter_redirects` builds, once per call, `ops_naming_each_buffer`: for each buffer the ascending
  ops that name it (exactly `op_names_buffer`'s relation, now defined through one shared
  `buffers_named_by`). Per lane it takes the `partition_point` slice `[first_op, consumer)` of the
  consumer's buffer's list and declines if any op there is not in the chain's sorted run. Same set
  of ops, same answer. The one `op_dataflow` result in `build_sequential` now serves both the
  redirect pass and the unit loop.
- **D4 (R2).** The nightly `compiles_and_binds_65_537_tracks_with_builtins` keeps its name and
  60 s bound and now runs one route per track plus the standing console's `eq` as a console slot
  with a live control lane (`attach_effect_live_controls`) on every track. No new per-PR test and
  no new timing test. It ran in 16.8 s locally (release, overflow checks on), so one test suffices.

## Evidence (attempt 1)

Descriptive, PR evidence only: release (no overflow checks), AVX2 host, one invocation per binary
under the exclusive machine lock, a warmup pass at 8,192 tracks, then two measured rounds. Before
is main 5346bbd70, after is dc0e29e13. Seconds, round 1 / round 2.

Bind of a hand-built plan, input -> post-input builtin bank -> one route per track -> output (the
#1045 shape that measured 23 s):

| tracks | before bind | after bind |
|---|---|---|
| 8,192 | 0.340 / 0.340 | 0.101 / 0.099 |
| 16,384 | 1.185 / 1.178 | 0.219 / 0.211 |
| 32,768 | 4.341 / 4.305 | 0.487 / 0.476 |
| 65,537 | 16.673 / 16.436 | 1.092 / 1.087 |

Before grows about 3.7x per doubling and after grows about 2.2x.

Compiled sessions through `compile_with_builtins` and `into_bound`, compile then bind (compile is
session compile, effects, live controls, builtins and graph compile):

| shape, tracks | before compile / bind | after compile / bind |
|---|---|---|
| plain, 8,192 | 0.364 / 0.277, 0.369 / 0.281 | 0.363 / 0.286, 0.362 / 0.303 |
| plain, 65,537 | 4.730 / 3.633, 4.380 / 3.560 | 4.829 / 3.391, 4.394 / 3.443 |
| routes+console, 8,192 | 0.734 / 0.470, 0.729 / 0.468 | 0.707 / 0.468, 0.719 / 0.507 |
| routes+console, 65,537 | 9.657 / 5.831, 9.091 / 5.488 | 9.600 / 5.964, 8.930 / 5.454 |

Finding: the compiled sessions never reached the walk. Both builds report the same numbers, and the
redirect count is 0 on every row. So the reshaped nightly guards `effect_control_resource` and the
route-fold metadata at scale, but it does not guard `scatter_redirects`. Only the hand-built shape
reaches that code.

Counters are the same before and after on every row: `bank_scatter_redirects` 0 for all shapes,
and `bank_route_folds` 0 (plain) and one per track (routes+console). The hand-built shape has 0
redirects. The `graph_fixture` fingerprint is byte-identical before and after.
`console-workload --test chain_shape` passes (22 tests).

Mutant: the in-between check counts the run's own ops (`.any(|_| true)`). It turns
`cohort_chain_merging_preserves_dataflow_on_random_graphs` red at `program/tests.rs:1704`.

- **D5 (root ruling, after the finding).** The nightly `compiles_and_binds_65_537_tracks_with_builtins`
  now also binds and renders the hand-built shape (`hand_built_bank_plan(true)`: builtin banks and
  one route per track) under a 15 s bound of its own, and asserts its exact bank-member and route
  counts. The test is the same one that `nightly.yml` already runs by name, so no workflow changes.
  The compiled routes+console phase stays as the guard for `effect_control_resource` and the
  route-fold metadata. The per-PR hand-built bank test shares the builder, without routes. Checked
  once each, release with overflow checks: on pre-fix `runtime.rs` (5346bbd70) the new phase fails
  at 16.66 s, and on the branch the whole test passes in 20.1 s.
- **Deviation from the one-invocation timing rule.** The timing above took two invocations. The
  first (compiled shapes) showed no difference before and after, so a frozen hand-built workload
  (the #1045 shape) was added and timed in a second invocation, once. Neither was retried.
- **D6 (root review).** The hand-built phase's bound is 5 s, not 15 s. That is about 4.5x the
  fixed bind (1.1 s) and about 3.3x below the old quadratic (16.7 s), so the guard still fails the
  quadratic on a runner faster than this box.
