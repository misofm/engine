# Move the 65,537-track graph and builtins scale tests to nightly; trim two heavy fixed loops

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md`, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`. Related cleanup issues: #1017-#1042.

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

## Attempt 1 evidence

Terra, 2026-09-29, branch `codex/1045-scale-tests-nightly` from `a509b681` (main plus #1031,
#1030, #1033 and #1061). Host: `x86_64` (`x86-64-v3`), rustc 1.97.1, `CARGO_INCREMENTAL=0`,
`nice`, on a shared 32-core host at load 20-100 (other agents' mutation passes). Timed runs held
the shared timing lock; the load is recorded with each number, and before and after ran back to
back.

### The base, recounted

- `crates/graph-compiler/tests/scale.rs:91` and `:160`, `crates/builtins-compiler/tests/scale.rs:34`,
  `crates/session/src/value.rs:86` and `:169`, `crates/session/tests/scale_transaction.rs:22`,
  `compile.rs:389` (the node cap refuses before banks or bind) and `MUTATIONS.md:257` (964-11) are
  where the spec says.
- The phase-two allocation test moved to `allocation_tracker.rs:1225`, its loop to `:1237`.
- `tools/bench/src/graph.rs` is gone (#1026). `tools/audit/src/fixture_builtins.rs:1369` still
  prepares 65,537 tracks, in the release `audit-native` job, and only prepares (amendment 2).
- `scripts/check-workspace-policy.sh:275` still bans `MAX_TRACKS`, `MAX_TRACK_COUNT`,
  `DEFAULT_MAX_TRACKS` and `TRACK_LIMIT`.
- graph-compiler has 408 mutants here, not 412.
- The four binaries, debug with test-debug-a's features, inside one whole-suite run (load 20-26):
  graph-compiler `scale` 68.2 s, `allocation_tracker` 38.5 s, session unit tests 36.4 s,
  builtins-compiler `scale` 6.7 s: 149.8 s. Alone, `:91` took 85 s and `:160` 84 s of CPU (load 50).

### What changed

- **`graph-compiler/tests/scale.rs`.** `:91` keeps only its constrained compile, per PR. Its node
  cap is now `NODES - 1`, one below the graph the session lowers to (`7 x 65,537 + 2`), not `1`.
  The refusal is then also a count of that graph: a truncated track index fits under the cap and
  is accepted, so it goes red (1045-3; its control at `maximum_nodes = 1` stays green). It also
  asserts the refused compile's 3 x 65,537 builtin processors, which it already prepares. `:160` is
  `#[ignore]`d, carries `:91`'s old report assertions (nodes, edges, routes, effects), and asserts
  that the whole path from session compile to the rendered block takes under 60 s.
- **`builtins-compiler/tests/scale.rs`.** `#[ignore]`d, nightly.
- **`builtins-compiler/tests/allocation_tracker.rs`.** The loop body is a helper over track
  counts. `phase_two_allocator_layouts_match_the_checked_resource_report` keeps its name and runs
  `[1, 4]` per PR; `..._at_65_537_tracks` is `#[ignore]`d and runs `[65_537]` nightly, with its
  first-touch warm-up at that size so the warm-up comparison still holds.
- **`session/src/value.rs`.** `one_million_deterministic_f32_patterns_round_trip` (renamed) runs
  1,000,000 patterns: 996,076 finite, 0 fallbacks, maximum length 48 (recomputed by a standalone
  copy of the loop, which reproduces the old 9,960,907 / 1 / 48 at ten million). The ten-million
  sample's only fallback, `0x15ae_43fd`, is already a directed case. The exhaustive test's ignore
  reason now says where it runs.
- **`nightly.yml`.** `release-budgets` runs the three 65,537-track gates in its exact-selector
  budget step, each with `--config profile.release.overflow-checks=true`; timeout 15 -> 30 min.
  `math-sweeps` runs the exhaustive f32 sweep in release; timeout 15 -> 25 min.
- **`scripts/check-ci-path-routing.py`, `scripts/test-ci-path-routing.py`.** The checker pins
  `release-budgets`' step to its exact command list, so the three commands join the list; the
  self-test's hard-coded 3 becomes the list's length. See the deviations.
- **`graph-compiler/tests/MUTATIONS.md`.** Rows 1045-1 to 1045-7.

`cargo test -- --list` over the three crates: nothing lost. One test added (the nightly 65,537
row), one renamed (ten million -> one million), three more `#[ignore]`d (`:160`, builtins `scale`,
the 65,537 row).

### The no-ceiling claim, per PR and nightly (the brief's proof)

Planted in a scratch copy of the change, run, restored. "Per PR" is the debug test with test-debug-a's
features; "nightly" is the `release-budgets` command.

| # | planted defect | per PR | nightly |
|---|---|---|---|
| 1045-1 | 964-11's ceiling, `graph.track.limit` above 65,536 tracks, in `compile_graph` | RED, `:91` (the refusal's diagnostics) | RED |
| 1045-2 | a `u16` track counter in `compile_graph`'s node loop | RED, `:91`: `attempt to add with overflow` | RED, the same panic. Release without overflow checks: GREEN (it wraps) |
| 1045-3 | a truncating `u16` track index: a track that does not fit gets no nodes | RED, `:91` (accepted under the one-below cap). The draft's `maximum_nodes = 1`: GREEN | RED, wrong node count |
| 1045-4 | a ceiling in bind (`into_bound`, more than 65,537 external nodes) | GREEN: nothing per PR binds at this size | RED |
| 1045-5 | #962 fix 1 reverted (linear `contains` per bank member) | GREEN | RED by the bound: 451 s against 60 s (unmutated 28.6 s, same load) |
| 1045-6 | a `u16` track counter in builtin preparation's preflight | RED, `:91`: overflow panic | RED, builtins `scale` |
| 1045-7 | a builtin ceiling above 65,536 tracks, disguised as `builtin.resource.limit` | RED, `:91` | RED, builtins `scale` |

So per PR a compiled ceiling or a narrowed track index is red wherever the session compile
(`scale_transaction.rs`), builtin preparation or the graph compile's front end would impose it.
A ceiling in bank attachment, bind or render (1045-4) and a quadratic compile or bind (1045-5) are
red nightly only, at most a day late: amendment 5's accepted risk, now with its bound.

The trimmed loops, per PR:

| # | planted defect | result |
|---|---|---|
| A1 | the report charges 8 retained copies of each track ID, not 9 | RED, `[1, 4]`: `tracks=1, meters=0` |
| A2 | the report counts the strip vector through a `u16` (exact below 65,536 tracks) | GREEN per PR; RED nightly in the 65,537 row. The one catch the trim moves |
| V1 | no `.0` suffix | RED, the 1M loop (and the directed and corpus tests) |
| V2 | every value takes the f64 fallback | RED, the 1M loop's fallback count |
| V3 | no value takes the fallback | GREEN in the 1M loop; RED in the directed test at `0x15ae_43fd`, the value the 10M loop caught it with |
| V4 | subnormals spelled `0.0` | RED, the 1M loop (3,930 fallbacks), and the directed test |
| V8 | one binade (`2^73`) spelled one ULP off; the fallback rescues the round trip | RED in the 1M loop **only**: the directed and corpus values never reach that binade |

### Gates

1. **Mutation equivalence, graph-compiler.** `run-mutants.sh graph-compiler 5 10`, every mutant, on
   the base and on the change: **identical.** 408 mutants each; 261 caught, 77 missed, 67 unviable,
   3 timeouts (`schedule.rs:132`, `:187`, `:200`, `+= -> *=`, on both). Mutant by mutant: none
   lost, none gained, no outcome changed. The pass took 85 min on the base and 50 min on the change.
2. **Mutation equivalence, builtins-compiler, shard 0/4.** `run-mutants.sh builtins-compiler 5 10 --features test-support
   --shard 0/4 --sharding round-robin`, on the base and on the change: **identical.** 207 mutants
   each; 111 caught, 50 missed, 45 unviable, 1 timeout; none lost, none gained. Both runs skip one
   pre-existing baseline failure (deviation 5), so the nightly 65,537 row has no lost mutant to
   answer for.
3. **The claim still fails, per PR and nightly.** Rows 1045-1 (964-11's ceiling, per PR) and
   1045-2/1045-3 (a `u16` track index: an overflow panic with overflow checks, a wrong count when
   it truncates) above. Amendment 3's gate: 1045-5 is red by the bound, not the timeout.
4. **Historical bugs.** `revert.py` on a scratch copy of the change, test-debug-a's command:
   - #966: exactly the 9 `bank_levels.rs` tests (1,252 passed, 9 failed, 10 ignored);
   - #970: the 7 `collapse_arming.rs` reproducers and the `bank_levels.rs` probe (8 failed).
   - On a copy of the base, graph-compiler, builtins-compiler and session (the only binaries this
     change touches) fail the same `bank_levels.rs` tests and nothing else: the base's scale tests,
     65,537 row and ten-million loop stay green under both bugs. test-debug-b's crates depend on
     neither `graph` nor `graph-compiler`.
5. **Cost.** Not measurable from CI job timings before the batch push (this attempt pushes
   nothing). Locally, the four binaries go from 149.8 s to about 20 s in the whole-suite debug run
   (graph-compiler `scale` 16.6 s, session unit tests 3.0 s, the other two under 0.5 s): **about
   130 s saved**. PR #1016's CI times for the same binaries (67.9 + 40.3 + about 34 + 6.5 s) less
   the kept remnant predict the same order. Record the real `test-debug-a` delta at the batch push.

### Other gates

- `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets
  --all-features -- -D warnings`, `cargo fmt --all --check`: pass.
- The three crates' whole suites (graph-compiler, builtins-compiler, session; test-debug-a's
  features), back to back under the lock:

  | profile | base | change |
  |---|---:|---:|
  | dev, wall (CPU) | 205.9 s (474 s), load 26 | 72.0 s (252 s), load 20 |
  | release, wall (CPU) | 25.9 s (44.5 s) | 6.6 s (12.5 s) |

  264 passed / 3 ignored before; 262 passed / 6 ignored after.
- Nightly, measured here (release, overflow checks): `:160` 21.6 s (load 35) and 28.6 s (load 60+)
  against its 60 s bound; builtins `scale` 0.7 s; the 65,537 row 3.2 s. The exhaustive f32 sweep
  passes: 343 s wall, 1,325 s CPU on four cores (with overflow checks; the nightly step runs
  without). The release builds took 56 s (graph-compiler), 76 s (builtins-compiler) and 58 s
  (session) on 32 loaded cores; on a 4-vCPU runner expect several minutes each, hence the timeouts.
  **Nightly time added: about 7-10 min in `release-budgets` and 7-9 min in `math-sweeps`, two
  jobs that run in parallel**, estimated from these numbers.
- `graph-compiler`'s release test build no longer needs `panic="unwind"`: with `effect_package`
  gone (#1037) it builds under the shipped `panic = "abort"`, base and change alike.
- `check-ci-path-routing.py` and `test-ci-path-routing.py` (`python3 -B`): pass. actionlint 1.7.7:
  clean, before and after.
- Every `scripts/check-*` with no arguments: pass, except the ones that require an argument
  (`--self-test` run instead where they have one: abi-layout, parameter-metadata, listening-033,
  listening-111, AudioWorklet call graph; the V8 spill check's `--check-toolchain`: all pass;
  `check-web-boot-budget.mjs` needs a built module and was not run) and `check-sdk-types.sh`
  (`sdk/node_modules` is not installed here). `check-workspace-policy.sh`
  (#1052's source-scrape lint) and `test-workspace-policy.sh`: pass.

### Deviations

1. **Scripts outside the authorized paths.** Amendment 3 puts `:160` in `release-budgets`, whose
   budget step `check-ci-path-routing.py` pins to an exact command list, and whose self-test
   assumes that step holds the file's first `--ignored --exact`. A separate step placed before it
   failed the self-test ("workflow mutation was accepted"); one placed after it fails the checker.
   So the three commands join the pinned list (which also means no PR can drop the only guard of
   #962 without a red), and the self-test's count follows the list. 9 lines.
2. **The exhaustive sweep runs in `math-sweeps`, not `release-budgets`,** with the job's own
   `-- --ignored <filter>` form: it is an exhaustive sweep, it takes about six minutes, and an
   `--ignored --exact` line ahead of `release-budgets` would take the self-test's anchor.
3. **The per-PR cap is one below the graph, not `1`,** and `:91` counts the builtin processors: a
   strengthening, shown by 1045-3's control.
4. **The bound covers the whole path** (session compile, builtins, graph compile, bind, one block),
   not only compile and bind: a quadratic in session compile or preparation is caught too.
5. **builtins-compiler's mutation passes skip one test on both trees.**
   `actual_runtime_bank_slot_owners_fit_retained_largest_and_conversion_reservation` fails the
   unmutated baseline under `run-mutants.sh`'s `opt-level = 1` on the base as well (a controlled
   `realloc` is not observed: `LARGEST_REALLOC_BYTES` 0 against 64), so cargo-mutants refuses to
   start. It passes at opt-level 0, as CI runs it. Not this issue's; worth its own look.

### Test value (AGENTS.md)

- `:91` per PR: a track ceiling or narrowed track index in builtin preparation or the graph
  compile's front end at 65,537 tracks (1045-1, -2, -3, -6, -7); nothing else per PR compiles a
  graph above 65,536 tracks.
- `:160` nightly: a ceiling or narrowed index in bank attachment, bind or render, or a quadratic
  compile or bind (1045-4, -5).
- The 65,537 row nightly: an accounting count that is exact only below 65,536 tracks (A2).
- The 1M loop: a spelling defect in a binade no directed or corpus value reaches (V8).

### Remaining risk

- 1045-4, 1045-5 and A2 are caught a day late, not per PR.
- #1002's memory claim stays unguarded (amendment 4); its nightly RSS budget is its own issue.

## Sol verdict, attempt 1

Sol, 2026-09-29. I merged `6f244ce1` into `codex/batch-slim-3` (`3ba8982a`) in a scratch worktree and
checked it there. Host: `x86_64` (`x86-64-v3`), rustc 1.97.1, `CARGO_INCREMENTAL=0`, `nice`, on a
shared host at load 20-55.

**FAIL.** The nightly wiring, the trimmed loops and every gate hold. But a track ceiling in bind or
render is green on every PR, and a 2.3 s per-PR test catches both. That is the brief's FAIL
condition. The fix is small and stays inside the authorized paths.

### The merge

- **One textual conflict:** `nightly.yml`, `math-sweeps`. #1039 removed the Wasm target and the
  wasm-console guest step. #1045 added its session step just above that guest step.
- **Resolution for root:**
  - Keep the batch's job: the name `math exhaustive sweeps`, the one-line toolchain install, and no
    guest step.
  - Add #1045's `Session canonical f32 spelling, exhaustive round trip` step after F1.
  - Take #1045's 25-minute timeout.
- **Everything else merges cleanly.** That covers `release-budgets`, which sits beside #1049's
  `full-size-tests`. It also covers `failure-notice`: #1049's needs list and table already carry
  `math-sweeps` and `release-budgets`. The routing scripts merge cleanly too.
- **No semantic conflict.** On the merge, the three crates pass in dev and in release. The three
  `release-budgets` commands pass as well: `:160` took 19.5 s against its 60 s bound.

### Findings, by severity

1. **High, blocking: a track ceiling in bind or render is caught only nightly, though a cheap
   per-PR test catches it.**
   - **Method.** I planted each ceiling on the merge. Then I ran the per-PR suites of `session`,
     `graph`, `graph-compiler` and `builtins-compiler` with test-debug-a's features, a synthetic
     bind test (below), and the `release-budgets` command.

   | # | planted ceiling | per PR | synthetic bind | nightly |
   |---|---|---|---|---|
   | P1 | `parse_session_json` refuses more than 65,535 tracks | GREEN | GREEN | nothing parses such a document |
   | P2 | `compile_session` refuses more than 65,535 tracks | RED: `scale_transaction.rs`, `:91` | - | - |
   | P3a | graph compile adds `graph.resource.limit` at `$.tracks` above 65,535 tracks | GREEN | GREEN | not run |
   | P3b | graph compile refuses early, with the node cap's own code and path | GREEN | GREEN | RED |
   | P4 | builtin preparation adds `builtin.resource.limit` above 65,535 tracks | RED: `:91` | - | - |
   | P5 | `PreparedGraphPlan` bind refuses more than 65,536 bindings | GREEN | **RED** | RED |
   | P5b | `into_bound` refuses more than 65,536 bindings | GREEN | GREEN | RED |
   | P6 | graph render refuses more than 65,535 runtime units | GREEN | **RED** | RED |

   - **Before this change**, `:160` ran per PR and caught P3a, P3b, P5, P5b and P6.
   - **A discarded plant.** P6b, a render ceiling at 7 x 65,535 units, never fires at 65,537
     tracks, because banking leaves fewer units than that. It is not a ceiling, so it is not in the
     table.
   - **The synthetic test.** It builds a `PreparedGraphPlan` through the public
     `PreparedGraphPlan::new`: 65,537 `TrackStage::Input` nodes, each routed to one output. It
     binds them with identity bindings and renders one block. In debug it takes 0.5 s to build and
     2.3 s in all. In `scale.rs` it runs in parallel with `:91` (18.5 s), so the binary's wall time
     does not grow. It is green on the clean merge.
   - **Attempt 2 must:**
     - add that test to `crates/graph-compiler/tests/scale.rs`, per PR;
     - make `:91` assert the refusal exactly: one diagnostic,
       `("graph.resource.limit", "$.graph_compile_caps")`. This costs nothing and turns P3a red
       (verified; the extra `("graph.resource.limit", "$.tracks")` shows);
     - record both as rows, and correct the evidence's 1045-4 row, "no per-PR test binds at this
       size: this is the one-day gap the issue accepts".
2. **Accepted residual, not blocking: what still needs a real 65,537-track compile.**
   - **What.** P3b, a ceiling that copies the node cap's code and path. P5b, a ceiling in the
     `into_bound` wrapper. A ceiling in bank attachment. #962's quadratics.
   - **Why it stays nightly.** Each one needs the unconstrained compile. Timed phase by phase in
     debug, that compile takes 28-40 s and bind another 13-17 s. That is not cheap next to the
     saving.
   - **Condition.** Nightly with the bound is acceptable under amendment 5, provided the spec's
     remaining-risk list names exactly this set.
3. **Low, pre-existing: no test parses a document above 65,535 tracks (P1).** Nothing does, per PR
   or nightly. `descriptive_scale.rs` parses 65,536 tracks, but it is ignored and nothing schedules
   it. This is not #1045's regression; it needs its own issue.
4. **Low, cosmetic.** The `math-sweeps` job and its failure-notice row are both named
   `math exhaustive sweeps`, but the job now runs the session sweep too.

### Confirmed

- **Nightly wiring.**
  - `release-budgets` runs the three moved tests in release with
    `--config profile.release.overflow-checks=true`.
  - The 60 s bound discriminates. With #962 fix 1 re-injected (a linear `required_bindings.contains`
    per bank member), the test goes RED by the bound at 231 s. Clean, it takes 19.5 s on the same
    host.
  - `failure-notice` needs both jobs and prints both.
  - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass under `python3 -B`, and
    actionlint 1.7.7 is clean.
- **The trimmed loops keep their catches per PR.**
  - A1 (8 retained copies, not 9): RED in `[1, 4]`, at `tracks=1`.
  - A2 (the strip vector counted through a `u16`): GREEN per PR, and RED in the nightly
    65,537-track row.
  - V8 (the 2^73 binade one ULP off): RED in the one-million loop, with 3,921 fallbacks against 0.
- **Gates on the merge.**
  - `cargo check --workspace --all-targets --all-features`, clippy `-D warnings` and fmt pass.
  - The three crates pass in dev (79 s wall; graph-compiler `scale` 18.5 s) and in release.
  - I ran 55 scripts under `python3 -B`, and all pass. They are the lint job's policy scripts and
    their mutation companions, #1043's nightly self-test suites, and `check-workspace-policy.sh`,
    which includes #1052's source-scrape lint.
- **Method note.** One merge run first reused a mutated artifact from the scratch target directory
  it shared with the mutation worktree. I re-ran it in a clean target directory, and the result
  above is from that run.
- **Not re-run.** I did not re-run the cargo-mutants passes or the #966/#970 reverts. This change
  does not touch the tests they turn red.

### Test value

- **`:91` with its one-below cap:** a narrowed track index that drops a track's nodes in graph
  compile, which nothing else per PR catches.
- **`:160` under its bound:** a quadratic compile or bind, and a ceiling in `into_bound` or bank
  attachment, a day late.
- **The nightly 65,537-track allocation row:** an accounting count that is exact only below 65,536
  tracks (A2).
- **The one-million loop:** a spelling defect in a binade that no directed or corpus value reaches
  (V8).
- **The recommended synthetic test:** a ceiling in bind or render above 65,535 track inputs, which
  nothing else per PR binds.
