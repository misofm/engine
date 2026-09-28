# Move the 65,537-track scale tests to nightly; trim the two heaviest fixed loops

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §4.1 and §5 item 4). Base `a9414c0c`. No ruling
needed: the claim stays, only its cadence changes.

## Problem

These are the largest single costs in `test-debug-a`, the job that runs every workspace crate except
the DSP ones. Local time is `exec_time` from a run under the timing lock; CI time is from PR
#1016's log.

| test | local | CI binary | claim |
|---|---:|---:|---|
| `crates/graph-compiler/tests/scale.rs:160` `compiles_and_binds_65_537_tracks_with_builtins` | 60.8 s | 67.9 s for both scale tests | no compiled track ceiling (AGENTS.md: no `MAX_TRACKS`) |
| `crates/graph-compiler/tests/scale.rs:91` `compiles_65_537_tracks_or_rejects_only_a_configured_resource` | 55.3 s | (above) | same, plus one-below cap rejection |
| `crates/builtins-compiler/tests/allocation_tracker.rs:1174` `phase_two_allocator_layouts_match_the_checked_resource_report` | 37.4 s | 40.3 s | phase-two allocations equal the checked report; loops tracks `[1, 4, 65_537]` × meters `[0, 1, 7]` (`:1186`) |
| `crates/session/src/value.rs:86` `ten_million_deterministic_f32_patterns_round_trip` | 30.6 s | about 34 s | canonical f32 spelling round-trips |
| `crates/builtins-compiler/tests/scale.rs:34` `prepares_65_537_tracks_or_rejects_only_the_configured_resource` | 6.5 s | 6.5 s | no track ceiling in prepare |

Discrimination evidence:

- **Mutation.** Every graph-compiler mutant the rest of its suite let through (80, from
  `cargo mutants -p graph-compiler`) was re-run against the two scale tests. **They caught none**:
  79 survived and 1 timed out, as it had without them.
- **The count does not matter here.** The 2026-09-04 ledger (`03-compilers-hosts-tools.md:127`) found
  that layout-class equality does not depend on the track count. `[1, 4]` tracks exercise every
  linear container.
- **Duplication.** `:91` compiles the same 65,537-track session as `:160` (`../data/candidates-dsp-graph.md`
  item 47).
- **Nothing else runs the exhaustive case.** `value.rs:169` `exhaustive_f32_round_trip` is `#[ignore]`
  and no workflow runs it.

## Outcome

- **The 65,537-track claim moves to nightly.** `nightly.yml` runs `scale.rs:160` and builtins-compiler
  `scale.rs:34`, as `--ignored`, in a release job. Per PR, `scale.rs:91` keeps only its
  configured-resource rejection, at a small track count.
- **`allocation_tracker`** loops `[1, 4]` tracks per PR. The 65,537 row runs nightly with the scale
  tests.
- **`value.rs:86`** runs 1,000,000 patterns per PR. Nightly runs `value.rs:169`, the exhaustive round
  trip, in release.

## Scope

Authorized paths:
- `crates/graph-compiler/tests/scale.rs`;
- `crates/builtins-compiler/tests/scale.rs`, `crates/builtins-compiler/tests/allocation_tracker.rs`;
- `crates/session/src/value.rs`;
- `.github/workflows/nightly.yml`;
- this issue's spec.

## Gates

1. **Mutation equivalence, graph-compiler.** `../tools/run-mutants.sh graph-compiler` (all 412
   mutants), before and after. The per-PR caught set is unchanged. The audit's baseline without the
   scale tests is 265 caught, 77 missed, 3 timeouts, 67 unviable. Adding the scale tests changed
   nothing.
2. **Mutation equivalence, builtins-compiler.** `../tools/run-mutants.sh builtins-compiler --shard 0/4
   --sharding round-robin --features test-support`, before and after. The per-PR caught set is
   unchanged, or every lost mutant is caught by the nightly 65,537 row. List those.
3. **The claim still fails nightly.** In a scratch branch, a `u16` track index in graph compile
   (truncate `TrackId` to 16 bits) makes the nightly scale job red.
4. **Historical bugs.** `../tools/revert.py` for #966 and #970. The same tests as today go red, all
   in `bank_levels.rs` and `collapse_arming.rs`.
5. **Cost.** `test-debug-a` on a full-route PR is at least 100 s shorter. Record it from the job
   timings.

## Saving and risk

- **Saving:** about 130 s off `test-debug-a` (408 s → about 280 s) per PR.
- **Risk:** a defect that shows only above 65,536 tracks is caught by the nightly run, up to a day
  later, instead of per PR.
