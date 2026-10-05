# Decision 15, stream J: verdicts

This directory holds byte-for-byte copies of every stream-J verifier verdict. The work is on
branch `codex/d15-stream-j`, base `0e3e21b68`. Each issue's spec in `.github/ISSUE_SPECS/` holds
its Attempt record.

| Issue | Title | Commits on this branch | Attempts | Verdict |
| --- | --- | --- | --- | --- |
| #1248 | Name the failed predicate and bound the waits of the browser continuous-spectrum gate by a deadline | `3854c03bb`; follow-up `f023cee3e` | 1 | PASS ([1248-attempt1.md](1248-attempt1.md)) |
| #1251 | Make concurrent tests fail instead of hanging when a thread panics | `b3095ca2a`; follow-up `941e8368b` | 1 | PASS ([1251-attempt1.md](1251-attempt1.md)) |
| #1232 | Make the C ABI checker's header mutation legs reach the compiler | `3ade8e969`; follow-up `a96382e76` | 1 | PASS ([1232-attempt1.md](1232-attempt1.md)) |
| #1330 | Validate each effect descriptor once per type, not once per prepared instance | `a9f09e29a`, `9edd1a6b8`; follow-up `c12fe0651` | 1 | PASS ([1330-attempt1.md](1330-attempt1.md)) |
| #1235 | Remove the dead code builtins-compiler reports under no-features clippy | `3bf212cad` | 1 | PASS ([1235-attempt1.md](1235-attempt1.md)) |
| #1234 | Anchor the worklet callgraph checker's C allocator names | `2e5bcc31f`, `492d0153d`; follow-up `c9d3e17bc` | 1 | PASS ([1234-attempt1.md](1234-attempt1.md)) |
| #1301 | Make the shared edge-ramp restore probe cheap enough for every pull request | `ab9620a07`; follow-up `2c26f4077` | 1 | PASS ([1301-attempt1.md](1301-attempt1.md)) |
| #1302 | Make the realtime-policy drain rule structural instead of one regex line | `e3375bc1d`, `16a5b920b`, `4f15b5c7e`; follow-up `bc515d61c` | 3 | FAIL ([1](1302-attempt1.md)), FAIL ([2](1302-attempt2.md)), PASS ([3](1302-attempt3.md)) |
| #1304 | Tighten the four-lane reference graph ceilings from measured AArch64 rows | `5304737c1`; follow-up `5ca360a72` | 1 | PASS ([1304-attempt1.md](1304-attempt1.md)) |
| #1237 | Bound route gain and matrix values | `d1a9daf30`, `a8332e93b`, `784b5d61c`, `e9dc1a4b2`, `842139f4e`; follow-up `83afd35d5` | 2 | FAIL ([1](1237-attempt1.md)), PASS ([2](1237-attempt2.md)) |

Skipped: #1384 (needs stream A #1285) and #1303 (needs stream A #1277 and #1312). Both are ready
to resume when those land.

## Open for root/S0

These items come from the verdicts and the specs' Attempt records. Each is outside the issue's
authorized paths or needs the PR's CI run.

- **#1237 J1-2.** Replace the two literal copies of the route domain (`session/src/validate.rs`,
  `graph-compiler/src/ids.rs`) with one shared bounds constant, for example in
  `crates/session/src/model.rs`. Gate 2's test guards the drift until then.
- **#1237 J1-3.** The public `RouteControlRecord::new` (re-exported by host-core) bypasses
  `route_values` and D3. File it, or fold it into #1225.
- **#1234 MINOR-1.** The whole-name anchoring admits prefixed C or third-party allocator spellings
  that the substring form refused. Today's module has none. The verdict proposes a follow-up or a
  spec amendment (refuse C names as a substring of unmangled names only).
- **#1234 NIT 3 (spec text).** The spec's non-goal names `check-web-audioworklet.sh` as the script
  that runs the self-test; it is `scripts/test-web-audioworklet.sh`. Spec scope text, coordinator
  owned.
- **#1251 NIT-4.** `crates/capi/tests/plan_swap_race.rs` keeps a private `StopOnDrop`; it could use
  the now-public `bench_support::producer::StopOnDrop`. File as a follow-up.
- **#1235 surviving mutants (pre-existing).** The #916 test
  `actual_scalar_nonadjacent_output_track_takes_the_split_pair_now_the_output_is_dedicated` does
  not verify its own premise (t01 as the output track), so the `NonadjacentOutputConflict` mutants
  M1/B1, M3/B3 and the `NonadjacentTrackA` mutants M5/B5, M7/B7 survive. Fix in a follow-up issue.
- **C ABI race flake.** `crates/capi/tests/resource_lifecycle.rs`
  `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free` failed twice
  under heavy load with the raced plan's final block `[0, 0]` on both lanes (recorded in #1330's
  and #1304's Attempt records). Not reproduced in 60 loaded runs. It may be a lost-edit or
  lost-source defect, not harness timing. The #1304 verdict recommends a stateless issue with a
  capture loop that keeps the failing log and the run index. Compare with #1404 on `main`.
- **#1301 CI leg (gate 2).** On the PR's qualification run, read `test-debug-b`: the delay's
  `tests/randomized.rs` `finished in` must be 15 s or less (before: 80.18 s). In `test-debug-b`
  and `aarch64-debug`, no "running for over 60 seconds" line may name
  `the_effects_own_edge_ramp_snapshots_restore`. Record both CI numbers in the spec's Evidence
  section (`aarch64-debug` before: 113.30 s).
- **#1304 CI legs (gates 1 and 2).** Put the six-row table in the PR body. On the PR's AArch64
  debug job, read that the budget test passes and its printed rows sit under the new ceilings;
  the full AArch64 debug and release legs were not run locally.
- **#1248 note for #1106.** The 10 s spectrum deadline is spelled twice
  (`runContinuousSpectrumQualification` and the hop probe's `10_000`). #1106 can hoist one shared
  constant.
