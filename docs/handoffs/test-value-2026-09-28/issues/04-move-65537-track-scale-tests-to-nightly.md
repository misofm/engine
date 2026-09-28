# Move the 65,537-track graph and builtins scale tests to nightly; trim two heavy fixed loops

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §4.1 and §5 item 4). Base `a9414c0c`. Paths
starting `../` are relative to the audit's handoff folder. No ruling needed: the claim stays, and
only its cadence and size change.

## Problem

These are the largest single costs in `test-debug-a`, the job that runs every workspace crate except
the DSP ones. Local time is `exec_time` from a run under the timing lock; CI time is from PR
#1016's log.

| test | local | CI binary | claim |
|---|---:|---:|---|
| `crates/graph-compiler/tests/scale.rs:160` `compiles_and_binds_65_537_tracks_with_builtins` | 60.8 s | 67.9 s for both scale tests | no compiled track ceiling (AGENTS.md: no `MAX_TRACKS`) |
| `crates/graph-compiler/tests/scale.rs:91` `compiles_65_537_tracks_or_rejects_only_a_configured_resource` | 55.3 s | (above) | same, plus: only a configured cap refuses |
| `crates/builtins-compiler/tests/allocation_tracker.rs:1174` `phase_two_allocator_layouts_match_the_checked_resource_report` | 37.4 s | 40.3 s | phase-two allocations equal the checked report; loops tracks `[1, 4, 65_537]` × meters `[0, 1, 7]` (`:1186`) |
| `crates/session/src/value.rs:86` `ten_million_deterministic_f32_patterns_round_trip` | 30.6 s | about 34 s | canonical f32 spelling round-trips |
| `crates/builtins-compiler/tests/scale.rs:34` `prepares_65_537_tracks_or_rejects_only_the_configured_resource` | 6.5 s | 6.5 s | no track ceiling in prepare |

Discrimination evidence:

- **Mutation.** Every graph-compiler mutant the rest of its suite let through (80, from
  `cargo mutants -p graph-compiler`) was re-run against the two scale tests. **They caught none**:
  79 survived and 1 timed out, as it had without them.
- **One recorded catch, still covered.** `crates/graph-compiler/tests/MUTATIONS.md:257` (964-11)
  records `scale.rs:91` red on a hand-made "compiled track ceiling above 65,536" mutation. That is
  exactly the claim, and it stays guarded per PR by the tests listed under the outcome.
- **The count does not matter here.** The 2026-09-04 ledger (`03-compilers-hosts-tools.md:127`) found
  that layout-class equality does not depend on the track count. `[1, 4]` tracks exercise every
  linear container.
- **Duplication.** `:91`'s unconstrained compile repeats the session `:160` compiles
  (`../data/candidates-dsp-graph.md` item 47).
- **Nothing else runs the exhaustive case.** `value.rs:169` `exhaustive_f32_round_trip` is `#[ignore]`
  and no workflow runs it.

## Outcome

**Per PR, the no-ceiling claim keeps these guards:**
- `crates/session/tests/scale_transaction.rs:22` still compiles 65,537 tracks in debug, per PR, with
  overflow checks;
- `scale.rs:91` keeps its constrained compile, so only a configured cap may refuse, at 65,537
  tracks with tight caps. Its cost is dominated by the unconstrained compile, which goes;
- `tools/audit/src/fixture_builtins.rs:1370` renders 65,537-track resource rows;
- `scripts/check-workspace-policy.sh:274-276` bans `MAX_TRACKS`-style identifiers.

**Moves to nightly:** `scale.rs:160`, builtins-compiler `scale.rs:34`, and the 65,537 row of
`allocation_tracker`.
- They run in a nightly job with `--ignored`, in a profile with **overflow checks on**
  (`CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true`, or the debug profile). A `u16` wrap then panics
  instead of wrapping silently.
- `allocation_tracker` loops `[1, 4]` tracks per PR.

**Trimmed:** `value.rs:86` runs 1,000,000 patterns per PR. Nightly runs `value.rs:169`, the
exhaustive round trip, in release.

## Scope

Authorized paths:
- `crates/graph-compiler/tests/scale.rs`;
- `crates/builtins-compiler/tests/scale.rs`, `crates/builtins-compiler/tests/allocation_tracker.rs`;
- `crates/session/src/value.rs`;
- `.github/workflows/nightly.yml`;
- this issue's spec.

## Gates

1. **Mutation equivalence, graph-compiler.** `../tools/run-mutants.sh graph-compiler 5 10`, all 412
   mutants, before and after. The per-PR caught set is unchanged. The audit's baseline without the
   scale tests: 265 caught, 77 missed, 3 timeouts, 67 unviable. Adding the scale tests changed
   nothing.
2. **Mutation equivalence, builtins-compiler.** `../tools/run-mutants.sh builtins-compiler 5 10
   --features test-support --shard 0/4 --sharding round-robin`, before and after. The per-PR
   caught set is unchanged, or every lost mutant is caught by the nightly 65,537 row. List those.
3. **The claim still fails, per PR and nightly.** In a scratch branch:
   - 964-11's hand-made track ceiling makes the per-PR `scale.rs:91` constrained compile red;
   - a `u16` track index in graph compile (truncate `TrackId` to 16 bits) makes the nightly scale
     job red **by an overflow panic or a wrong count**, not silently green.
4. **Historical bugs.** `../tools/revert.py` for #966 and #970. The same tests as today go red, all
   in `bank_levels.rs` and `collapse_arming.rs`.
5. **Cost.** `test-debug-a` on a full-route PR is at least 90 s shorter. Record it from the job
   timings.

## Saving and risk

- **Saving:** about 120 s off `test-debug-a` (408 s → about 290 s) per PR. The constrained compile
  that stays in `scale.rs:91` still walks 458,761 nodes, so the scale binary does not drop to zero.
- **Risk:** a defect that shows only above 65,536 tracks in *graph binding with builtins*, and not
  in session compile, is caught nightly instead of per PR.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **The evidence does not see the claim these tests exist for (finding F1).**
   - `cargo mutants` never plants an O(n) → O(n²) change, so "0 of 80 surviving mutants" says nothing
     about #962, a compile and bind that is linear in track count.
   - I re-injected #962 fix 1 (`Vec::contains` per bank member in `with_builtin_banks`) and ran the
     debug binaries under the timing lock:

     | test | base | #962 re-injected |
     |---|---:|---:|
     | this draft's per-PR remnant (the constrained compile of `scale.rs:91`) | 16.9 s; re-run 19.3 s | 17.7 s; re-run 16.6 s |
     | `scale.rs:160` | 73.2 s | **673.9 s** |

   - `compile.rs:389` refuses on `maximum_nodes` before any bank attaches or any bind runs. So the
     remnant cannot see #962.
2. **Two of the listed per-PR guards are not guards.**
   - `tools/audit/src/fixture_builtins.rs:1370` only prepares builtins: no graph compile, no bind, no
     render.
   - `bench graph_validate_65537_tracks` is `#[ignore]`d (`tools/bench/src/graph.rs:691`), and #1026
     deletes it.

   After this draft, **nothing per PR** guards #962's fixes 1 and 3-8.
3. **The nightly job needs an explicit budget, not only overflow checks.**
   - Put `scale.rs:160` in `nightly.yml`'s `release-budgets` job, in release, with a wall-clock
     bound of 60 s. Linear takes about 17 s and quadratic about 20 minutes (#962 spec).
   - Graph-compiler release tests need `--config 'profile.release.panic="unwind"'` (#962 spec,
     "Deviations").
   - New gate: re-inject #962 fix 1 in a scratch branch; the nightly job goes red by its bound, not by
     its timeout.
4. **#1002's memory claim was never guarded here.** The scale session has no effects, so it never
   takes the #971/#1002 path (#1002 spec). Record it as a gap. Its guard would be a nightly RSS
   budget on the 65,537-track limiter session; that belongs to its own issue, not this one.
5. **Risk statement.** Per-PR detection of a quadratic regression is lost; it becomes up to one day
   late. That is an owner ruling. The verification recommends accepting it with the bound above;
   otherwise keep `scale.rs:160` per PR and move only the rest.
6. **The saving, measured.** The four binaries take a median 114 s over 8 full-route runs, and 149 s
   on PR #1016's run. Minus the kept constrained compile that is about 95-125 s.
