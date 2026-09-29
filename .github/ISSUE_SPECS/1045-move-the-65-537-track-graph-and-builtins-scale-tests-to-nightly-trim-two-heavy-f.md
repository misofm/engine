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
| 1045-4 | a ceiling in bind (`into_bound`, more than 65,537 external nodes) | GREEN: nothing per PR binds through `into_bound` at this size. (Corrected in attempt 2: a ceiling in graph bind or render is now caught per PR by a hand-built plan; this wrapper's is not.) | RED |
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

## Attempt 2 evidence

Terra, 2026-09-29, on `codex/1045-scale-tests-nightly` after merging `main` at `a8955ad4`
(batch 3). Same host and hygiene as attempt 1; the load was 45-60 throughout.

### The merge

`main` merged cleanly except for `nightly.yml`'s `math-sweeps` job, as Sol predicted. I kept
main's job, which has no wasm guest step, and added #1045's session sweep step after F1 with
the 25-minute timeout. `release-budgets`, `failure-notice` and the routing scripts merged
without conflicts. I left the job's name, `math exhaustive sweeps`, as main has it (Sol's
finding 4, cosmetic).

### What changed

- **`a_hand_built_65_537_input_plan_binds_and_renders_every_track`**, a new per-PR test in
  `graph-compiler/tests/scale.rs`, as Sol asked:
  - It builds a plan through the public `PreparedGraphPlan::new`. The plan has 65,537
    `TrackStage::Input` nodes, each routed through its own unity route to one output.
  - It binds the plan with `PreparedGraphPlan::bind` and renders one 16-frame block.
  - Each input's processor writes 1.0 on the left and 2.0 on the right. The test asserts that
    every output word is exactly 65,537 and 131,074, so the output counts the tracks that
    reached it. Every partial sum is an integer below 2^24, so the order of summation cannot
    change the result.
  - A ceiling in bind or render therefore refuses, and a narrowed index that drops tracks
    changes the sum.
- **`:91`** now asserts that the refusal is exactly one diagnostic,
  `[("graph.resource.limit", "$.graph_compile_caps")]`.
- **`MUTATIONS.md`**:
  - rows 1045-8 to 1045-11 are new;
  - 1045-4 is corrected: a ceiling in the builtins wrapper's `into_bound` stays nightly-only,
    and a ceiling in graph bind is now caught per PR;
  - 1045-1 and 1045-3 were re-run against the exact assertion.
- **This spec:** the attempt-1 evidence's 1045-4 row is corrected the same way.

### Plants (the brief's proof)

I applied each plant to a scratch copy of this tree and ran
`cargo test -p graph-compiler --test scale`. That is the per-PR binary, built in debug with
test-debug-a's features. I restored the tree between plants, and the unmutated tree was run
before P5, before P6 and at the end. All three unmutated runs were GREEN. The plant code for
P3a, P5, P5b and P6 is Sol's.

| # | planted defect | per PR |
|---|---|---|
| P5 (1045-9) | graph bind refuses more than 65,536 bindings | **RED**, new test: `65,537-input bind: graph.plan.binding`. GREEN once reverted |
| P6 (1045-10) | render refuses more than 65,535 runtime units | **RED**, new test: `the 65,537-input plan renders: InvalidEnvelope`. GREEN once reverted |
| P3a (1045-8) | `graph.resource.limit` at `$.tracks` above 65,535 tracks, i.e. the node cap's code at another path | **RED**, `:91`: it expects the one pair and gets `[..., ("graph.resource.limit", "$.tracks")]` |
| 1045-11 | render walks its active units through a `u16` index | **RED**, new test: the output is `[NaN, NaN]`, never written |
| 1045-1 | 964-11's `graph.track.limit` | RED, `:91`: it gets `[("graph.track.limit", "$.tracks")]` |
| 1045-3 | a truncating `u16` track index in graph compile | RED, `:91`: the compile is accepted under the one-below cap |
| P5b (1045-4) | the builtins wrapper `into_bound` refuses more than 65,536 bindings | GREEN, as expected: it is in the residual set below |

### Gates re-run on the merge

- **Build checks.** `cargo fmt --all --check`, `cargo check --workspace --all-targets
  --all-features` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  all pass with no warnings.
- **The three crates** (graph-compiler, builtins-compiler and session, with test-debug-a's
  features), run under the timing lock at load 50-57:

  | profile | wall | CPU | tests |
  |---|---:|---:|---|
  | dev | 120.3 s | 252.9 s | 263 passed, 7 ignored |
  | release | 15.0 s | 17.7 s | 263 passed, 7 ignored |

  - Against attempt 1 there is one more passing test (the new one) and one more ignored test.
    The extra ignored test is #1049's `the_full_meter_pass_...`, which arrived with `main`.
  - Wall times are higher than attempt 1's (72.0 s dev) because the host load was 50-57, not 20.
- **The new test's time.** Debug, alone: 5.6 s and 5.3 s (5.0-5.3 s CPU, 240 MB) at load 46-53.
  In release it is under a second.
  - It adds no wall time to the `scale` binary, because it runs in parallel with `:91`. At the
    same load, the whole binary took 24.75 s and `:91` alone took 24.94 s.
  - Sol measured 2.3 s for his version. Mine carries a route per track, so its plan has twice
    the nodes, but the cost is CPU in parallel with `:91`, not wall time.
- **Routing.** `check-ci-path-routing.py` and `test-ci-path-routing.py` (`python3 -B`) pass.
  actionlint 1.7.7 is clean on `nightly.yml` and `qualification.yml`.
- **Policy scripts.** 52 pass, each `scripts/check-*` run with `python3 -B` for the Python ones,
  plus `test-workspace-policy.sh`:
  - the scripts that need an argument ran with `--self-test`: abi-layout, parameter-metadata,
    listening-033, listening-111, the AudioWorklet call graph and #1059's new
    `check-scalar-oracle-absent.py`;
  - the V8 spill check ran with `--check-toolchain`;
  - `check-workspace-policy.sh`, which includes #1052's source-scrape lint, passes;
  - `check-sdk-types.sh` was not run, because `sdk/node_modules` is not installed here;
  - `check-web-boot-budget.mjs` was not run, because it needs a built module.
- **Nightly tests.** I did not re-run `:160`, builtins `scale` or the 65,537-track allocation
  row: attempt 2 does not change them. Sol ran all three on the merged tree, and `:160` took
  19.5 s against its 60 s bound.
- **Mutation passes and the #966/#970 reverts** were not re-run. `bank_levels.rs`,
  `collapse_arming.rs` and the product code are unchanged, and the new test only adds catches.

### Test value

- **The new test** catches a ceiling, or a narrowed index, in graph bind or render above
  65,535 track inputs (P5, P6, 1045-11). Nothing else per PR binds at this size.
- **`:91`'s exact refusal** catches a ceiling that reuses the node cap's code at another path
  (P3a).

### Remaining risk (replaces attempt 1's list)

These are caught nightly only, at most a day late, under amendment 5. Each needs the
unconstrained compile, which takes 28-40 s in debug, plus 13-17 s for bind:

- a ceiling that copies the node cap's code **and** path (P3b);
- a ceiling in the builtins wrapper `into_bound` (P5b, 1045-4);
- a ceiling in bank attachment;
- #962's quadratic compile and bind (1045-5, caught by the 60 s bound);
- an accounting count that is exact only below 65,536 tracks (A2, caught in the nightly
  65,537-track allocation row).

Two gaps sit outside this issue:

- **Low, pre-existing (Sol's finding 3):** no test parses a document above 65,535 tracks
  (P1). `descriptive_scale.rs` is ignored and nothing schedules it. It needs its own issue.
- **#1002's memory claim** is still unguarded (amendment 4).

### Lines

Attempt 2 against the merge commit: `graph-compiler/tests/scale.rs` +210/−12,
`MUTATIONS.md` +16/−5, and this spec. The whole branch against `a8955ad4`: code, workflow and
scripts +301/−54; with `MUTATIONS.md` and this spec, +632/−54 before this section.

## Sol verdict, attempt 2

Sol, 2026-09-29, head `5794f81b`, checked in a scratch worktree. Host: `x86_64` (`x86-64-v3`),
rustc 1.97.1, `CARGO_INCREMENTAL=0`, `nice`, load 18-40. `main` (`a8955ad4`) has the same tree as
the batch head I merged in attempt 1.

**FAIL.** Attempt 1's blocking finding is closed: bind and render caps are now red per PR. But a
cap in bank attachment is still green per PR, and a cheap per-PR test catches it. That is the FAIL
condition.

That case is my miss. Attempt 1's verdict listed bank attachment as needing the real compile, and
attempt 2 followed that list. It does not need the real compile: `PreparedGraphPlan::with_builtin_banks`
is public, like `PreparedGraphPlan::new`.

### Findings, by severity

1. **High, blocking: a track cap in bank attachment is green per PR, and a 7 s test catches it.**
   - **The plant, PBANK.** `with_builtin_banks` refuses when the banks hold more than 65,535
     members in all.
     - Every per-PR test stays GREEN, including `scale.rs`'s two per-PR tests.
     - Nightly's `:160` goes RED: `validated fixed builtin member shape: InvalidMembers`.
   - **The probe that catches it.** I built it by hand, like attempt 2's test.
     - The plan has 65,537 `Input` nodes and one `PostInputBuiltins` node per track.
     - Each `PostInputBuiltins` node sums straight into the output.
     - Banks hold `Backend::current()`'s lane count, with an identity processor, and attach
       through `with_builtin_banks`.
     - It binds, renders one block, and asserts that the output equals the track count.
   - **Result.** The probe is GREEN unmutated and RED on PBANK (`attach: InvalidMembers`). Alone in
     debug it takes 7.2 s: 1.0 s to build, 1.3 s to attach, 4.6 s to bind and 0.3 s to render. It
     runs beside `:91`, which takes 16 s.
   - **What else it covers per PR.** It also runs the runtime's bank gather and scatter across
     65,537 lanes. No per-PR test reaches that today.
   - **Keep the probe free of per-track routes.** Banks plus one route per track bind
     super-linearly:
     - debug: 0.2 s, 1.1 s and 3.3 s at 1,024, 2,048 and 4,096 tracks;
     - release: 0.8 s, 3.3 s, 6.7 s and 23 s at 8,192, 16,384, 32,768 and 65,537 tracks;
     - declining the route fold changes little: 0.7 s, 2.6 s and 10 s in debug at 2,048, 4,096
       and 8,192 tracks.

     Without banks (attempt 2's test), or without routes (the probe), bind stays linear. This is
     #967's territory (one route per track); see finding 3.
   - **Attempt 3 must:**
     - add the probe per PR, in `graph-compiler/tests/scale.rs`, at `Backend::current()`'s bank
       width;
     - record PBANK red on it and green reverted;
     - move bank attachment out of the residual list.
2. **The residual list, case by case.** Each needs the unconstrained 65,537-track compile. That
   compile takes 28 s in debug at `Backend::Scalar` and 40 s at `Backend::current()`, against the
   constrained compile's 12 s that it would replace.
   - **P3b, a cap copying the node cap's code *and* path: acceptable nightly.**
     - Only a successful compile tells it apart from the configured refusal. That costs at least
       16 s more on the per-PR job.
     - It is the least plausible form. A named cap is caught by the identifier lint, a distinct
       code by `:91`, and another path by `:91`'s exact pair.
     - The bank plan the compiler forms before `with_builtin_banks` falls under this case too.
   - **P5b, a cap in `into_bound`: acceptable nightly.**
     - `PreparedBuiltinsGraphArtifact` is sealed, so only a compile makes one.
     - The wrapper's bind prevalidation is set-based and has no count in it.
   - **#962's quadratic compile and bind (1045-5): acceptable nightly, under amendment 5.**
     - A debug per-PR wall-clock bound is not reliable, for #962's own reason.
     - The nightly 60 s bound catches it: RED at 231 s in attempt 1's check.
   - **A2, the accounting count through a `u16`: acceptable nightly.**
     - It needs a 65,537-track preparation with the allocation tracker.
     - It is caught in the nightly row.
   - **With finding 1 fixed**, the engine rule is protected per PR at every layer a cap can hide
     without a full compile:
     - session compile (`scale_transaction.rs`);
     - builtin preparation and the graph compile's front end (`:91`);
     - bank attachment (the probe);
     - bind and render (attempt 2's test).

     Parse (P1) is still unguarded anywhere; that gap predates this issue.
3. **Low, for #967, not #1045.** The super-linear bind in finding 1 is new evidence for #967,
   which is open ("one route per track"):
   - 65,537 banked tracks with a route each take 23 s to bind in release;
   - the same probe without routes takes 4.6 s to bind, in debug.

   Record it there.

### Confirmed

- **Attempt 1's finding is closed.** I re-planted each defect on the head and ran
  `cargo test -p graph-compiler --test scale` with test-debug-a's features.

  | plant | result per PR | failure |
  |---|---|---|
  | P5 | RED | `65,537-input bind: graph.plan.binding` |
  | P6 | RED | `the 65,537-input plan renders: InvalidEnvelope` |
  | P3a | RED, `:91` | extra `("graph.resource.limit", "$.tracks")` |

  - The unmutated head is GREEN.
  - The test runs in `test-debug-a`, a required job (via `qualification`'s verdict). The router
    sends every edit to `graph`, `graph-compiler`, `builtins-compiler` or `session` down the `full`
    route that job needs.
- **The new test's cost.** Alone in debug it finishes in 3.4 s. The whole `scale` binary takes
  16.9 s wall, the same as `:91` alone. In release it is under a second.
- **`nightly.yml`.**
  - `math-sweeps` keeps `main`'s job, adds the session sweep after F1, and has a 25-minute
    timeout.
  - `failure-notice` needs `math-sweeps` and `release-budgets`, and reports both.
  - The routing checker and its self-test pass under `python3 -B`, and actionlint 1.7.7 is clean.
- **Nightly commands on the head.** All three `release-budgets` 65,537-track commands pass. `:160`
  takes 19.6 s against its 60 s bound.
- **Gates on the head.**
  - fmt, `cargo check --workspace --all-targets --all-features` and clippy `-D warnings` pass.
  - The three crates pass in dev and in release: 257 passed, 7 ignored.
- **Policy scripts, 73 run, all pass.** That is every `scripts/check-*` and `scripts/test-*`,
  Python ones under `python3 -B`, plus `check-stem-store-v1.mjs`.
  - It includes `check-workspace-policy.sh`, which carries #1052's source-scrape lint, and
    `check-graph-determinism.sh` (100 of 100).
  - Scripts that need an argument ran with `--self-test`, or with their CI arguments.
  - Seven were not run because they need external toolchains or built artifacts: capi-abi,
    cross-targets, protocol-wasm-parity, sdk-headless, sdk-types, wasm-realtime-atomics and
    web-audioworklet.

### Test value

- **Attempt 2's hand-built test** catches a cap or a narrowed index in graph bind or render
  above 65,535 track inputs: P5, P6 and 1045-11.
- **`:91`'s exact pair** catches a cap that reuses the node cap's code at another path: P3a.
- **The probe attempt 3 adds** would catch a cap in bank attachment, or in the runtime's bank
  tables, above 65,535 lanes.

## Attempt 3 evidence

Terra, 2026-09-29, on `codex/1045-scale-tests-nightly` at `da4c48e6` (no new merge; `main` is
still `a8955ad4`). Same host and hygiene as before, load 22-37.

### What changed

- **`a_hand_built_65_537_track_plan_attaches_builtin_banks_binds_and_renders`**, a new per-PR test
  in `graph-compiler/tests/scale.rs`, Sol's probe (`verify-1045/probe_banks_1045.rs`) made a test:
  - every track's input feeds its `PostInputBuiltins` stage, and every stage sums straight into
    the output, with no per-track route;
  - the stages attach as builtin banks at `Backend::current()`'s width
    (`BankWidth::for_backend`, so Simd8 on x86-64-v3 and Simd4 on AArch64; no width returns
    early), through the public `PreparedGraphPlan::with_builtin_banks`;
  - it asserts 65,537 bank members, then binds, renders one block and asserts the exact track
    count, like attempt 2's test.
- **The two hand-built tests share their construction.** The helpers `track_stages`, `edge`,
  `hand_built_plan` and `bind_and_render_every_track` replace attempt 2's inline code, so the
  two tests differ only in their graph shape. They cannot share one built plan: a plan with both
  banks and a route per track binds super-linearly (below), and the routed test's claim is the
  route path. They run in parallel with each other and with `:91` in the same binary.
- **`MUTATIONS.md`:** rows 1045-12 and 1045-13; bank attachment leaves the residual list.

### Plants

Each was applied to a scratch copy of this tree and run with `cargo test -p graph-compiler --test
scale` (debug, test-debug-a's features). The tree was restored between plants. The unmutated
tree was GREEN at the start, between PBANK and PBANK16, and at the end.

| # | planted defect | per PR |
|---|---|---|
| PBANK (1045-12) | `with_builtin_banks` refuses more than 65,535 members in all | **RED**, the banked test: `65,537-track bank attachment: InvalidMembers`. **GREEN reverted** |
| PBANK16 (1045-13) | `with_builtin_banks` keeps only the banks a `u16` bank index reaches | GREEN. Not a ceiling here: 65,537 tracks are 8,193 banks. Discarded |
| P5 | graph bind refuses more than 65,536 bindings | RED, both hand-built tests (`graph.plan.binding`) |
| P6 | render refuses more than 65,535 units | RED, both (`InvalidEnvelope`) |
| 1045-11 | render walks its units through a `u16` index | RED, both (`[NaN, NaN]`) |
| P3a | `graph.resource.limit` at `$.tracks` | RED, `:91` (the exact pair) |

P5, P6 and 1045-11 were re-run because attempt 2's test was refactored onto the shared helpers.

### Per-PR cost

Debug, the per-PR `scale` binary, under the timing lock at load 25-37:

| run | wall | CPU |
|---|---:|---:|
| the banked test, alone | 6.6 s; 8.0 s | 6.3 s; 7.8 s (280 MB) |
| attempt 2's routed test, alone | 3.4 s | 3.3 s |
| `:91`, alone | 17.1 s | 16.3 s |
| the binary, default threads | 18.2 s | 28.3 s |
| the binary, `--test-threads 4` (a 4-vCPU runner) | 16.3 s | 24.8 s |

So the new test adds about 6-8 s of CPU and no wall time: the binary still finishes when `:91`
does. The whole three-crate suite: dev 52.3 s wall (190 s CPU), release 6.9 s, 264 passed and 7
ignored (one more passing test than attempt 2). In release the `scale` binary takes 3.3 s.

### For #967 (a pointer, not a fix)

Sol's attempt-2 verdict measured that a plan with builtin banks **and** one route per track binds
super-linearly: 0.2 / 1.1 / 3.3 s in debug at 1,024 / 2,048 / 4,096 tracks, and 0.8 / 3.3 / 6.7 /
23 s in release at 8,192 / 16,384 / 32,768 / 65,537 tracks. Declining the route fold changes
little. Without banks (attempt 2's test) or without routes (this test) bind stays linear. That is
#967's shape ("one route per track"); record it there. #1045's tests keep the two apart.

### Gates re-run

- `cargo fmt --all --check`, `cargo check --workspace --all-targets --all-features` and `cargo
  clippy --workspace --all-targets --all-features -- -D warnings`: pass, no warnings.
- The three crates in dev and release: pass (above).
- Policy scripts: 54 pass. That is every `scripts/check-*` (Python under `python3 -B`; the ones
  that need an argument with `--self-test`, the V8 spill check with `--check-toolchain`), plus
  `test-workspace-policy.sh`, `test-ci-path-routing.py` and `test-graph-policy.sh`. It includes
  `check-workspace-policy.sh` (#1052's source-scrape lint) and `check-graph-determinism.sh`.
  `check-sdk-types.sh` (no `sdk/node_modules`) and `check-web-boot-budget.mjs` (needs a built
  module) were not run.
- Not re-run: the nightly commands, the mutation passes and the #966/#970 reverts. Product code,
  `nightly.yml` and the nightly tests are unchanged since attempt 2, which Sol checked.

### Test value

- **The banked test:** a ceiling in bank attachment above 65,535 bankable stages, which nothing
  else per PR attaches (PBANK), and the runtime's bank gather and scatter across 65,537 lanes.
- **Attempt 2's routed test:** a ceiling or narrowed index in bind or render with 65,537 routes.

### Remaining risk (replaces attempt 2's list)

Nightly only, at most a day late (amendment 5): a ceiling copying the node cap's code and path
(P3b), a ceiling in `into_bound` (P5b), #962's quadratic compile and bind (the 60 s bound), and
an accounting count exact only below 65,536 tracks (A2). Outside this issue: parse above 65,535
tracks (P1, no test anywhere), #1002's memory claim, and #967's banked-and-routed bind above.

### Lines

Attempt 3: `scale.rs` +212/−76, `MUTATIONS.md` +9/−4, this spec. The branch against `main`
(`a8955ad4`), code, workflow and scripts: +436/−53.

## Sol verdict, attempt 3

Sol, 2026-09-29, head `e593a441`. I merged it into `codex/batch-slim-4` (`e53dc445`) in a scratch
worktree and checked it there. Host: `x86_64` (`x86-64-v3`), rustc 1.97.1, `CARGO_INCREMENTAL=0`,
`nice`, load 10-24.

**FAIL.**

- **Closed.** Attempt 2's blocker is fixed: a cap in bank attachment is red per PR.
- **Still open.** The sweep found one more layer #1045 moved to nightly that a cheap test can
  hold per PR: the compiler's builtin bank plan.
- **My miss.** My attempt-2 verdict wrongly filed this layer under P3b ("the bank plan the compiler
  forms"). It is reachable through a public entry, like `with_builtin_banks`.
- **Complete list.** To end the rounds of discovery, the list below covers every layer the brief
  named, with measurements.

### The merge

- **No conflicts.** Batch 4 does not touch `nightly.yml`. Its changes to
  `check-ci-path-routing.py`, `test-ci-path-routing.py` and `qualification.yml` merge cleanly around
  #1045's three `NIGHTLY_BUDGET_COMMANDS` rows.
- **Root applies nothing by hand.**
- **The merged workflow.**
  - `failure-notice` still needs `math-sweeps` and `release-budgets`, and reports both.
  - `math-sweeps` keeps `main`'s job, with the session sweep after F1 and a 25-minute timeout.
- **No semantic conflict.**
  - Batch 4 rewrote about 300 lines of `graph/src/runtime.rs`, so I re-ran all three
    `release-budgets` 65,537-track commands on the merge. They pass, and `:160` takes 23.9 s
    against its 60 s bound.
  - The per-PR `scale` binary passes: 3 passed, 1 ignored.

### Findings, by severity

1. **High, blocking: a cap in the compiler's builtin bank plan is green per PR, and 1 s inside
   `:91` catches it.**
   - **What the layer is.** `compile_graph` calls `PreparedBuiltinsSession::graph_builtin_bank_resource`.
     That runs `planned_strip_banks`, then `rack_compiler::plan_bank_groups` (the workspace's one
     cohort planner, which effect banks use too), then `builtin_bank_resource`.
   - **Why nothing per PR reaches it.** It runs after the node cap refuses `:91`, and the
     hand-built tests skip it.
   - **Before this change**, `:160` held it per PR at `Backend::current()`.

   | plant | per PR (session, graph, graph-compiler, builtins-compiler, rack-compiler) | nightly `:160` | the proposed check in `:91` |
   |---|---|---|---|
   | PPLAN: `plan_bank_groups` refuses more than 65,535 candidates | GREEN | RED: `DuplicateId` panic in `planned_builtin_bank_members` | RED: the same panic |
   | PRES: `builtin_bank_resource` refuses more than 65,536 lanes | GREEN | RED: `graph.resource.arithmetic_overflow` at `$.graph.builtin_banks` | RED: `the builtin bank plan at 65,537 tracks` |

   - **The proposed check.** It is in `scratchpad/verify-1045/a3/scale-proposal.diff`: 23 lines in
     `:91`, placed before the constrained compile consumes `builtins`.
     - It calls `graph_builtin_bank_resource(Backend::current(), levels, SessionPoolClasses::from_session(&session))`.
       The levels hold one dependency level per bankable stage, built with `track_stages`.
     - It asserts `bank_count == 3 * ceil(65,537 / lanes)`.
   - **Its cost.** The planning takes 0.9 s. The prepared builtins are the ones `:91` already
     builds. `:91` took 17.5 s with the check and 16.2 s without it, at load 10-17.
   - **Attempt 4 must:**
     - add the check;
     - record PPLAN and PRES red on it and green once reverted;
     - take the compiler's bank plan off the residual list.
2. **The sweep, layer by layer.** "Per PR" means a test in `test-debug-a`. That job is required
   through `qualification`'s verdict, and the router sends every edit to these crates down its
   `full` route.

   | layer | per PR today | if not, is there a cheap test? |
   |---|---|---|
   | session parse (P1) | no | no: a 65,537-track document is 93.7 MB, and parsing it takes 19.3 s in debug. It predates #1045 and needs its own issue |
   | session compile | yes: `scale_transaction.rs`, `:91` | - |
   | builtin preparation | yes: `:91` (P4, 1045-6, 1045-7) | - |
   | graph compile, front end | yes: `:91` (1045-1, -2, -3, P3a) | - |
   | graph compile, back end (topo, timings, buffers, estimate) and P3b | no | no public entry. It needs the unconstrained compile, +16-28 s. Acceptable nightly |
   | **builtin bank plan** | **no** | **yes, 1 s: finding 1** |
   | builtin bank lowering (`into_graph_artifact_with_banks`) and `into_bound` (P5b) | no | moderate. My probe lowers a hand-built 65,537-track strip plan in 13.4 s of debug CPU, and 28.5 s with bind and render. Acceptable nightly. It is width-bounded per bank, and the bank and bind layers either side of it are held per PR |
   | bank attachment | yes: attempt 3's test (PBANK re-planted: RED, `InvalidMembers`) | - |
   | graph bind and render | yes: attempt 2's test (P5, P6) | - |
   | #962 quadratics | no | no reliable per-PR clock. Nightly's bound, RED at 231 s |
   | A2 (the accounting count through a `u16`) | no | needs the 65,537-track allocation tracker. Nightly row |
   | host-core preparation, before the graph compile | no, and never was | **yes, 4.5 s** (below). Predates #1045 and is outside its paths: finding 3 |
   | capi prepare, host-web boot | no, and never was | no: both parse the JSON document, 19.3 s at this size. Predates #1045 |

3. **Medium, not blocking: host-core preparation has never been held above 65,535 tracks.**
   - **The gap.** `prepare_host_runtime` runs per-track stages before the graph compile: the
     count check, the track-to-source mapping, effects and builtins. A cap there is green per PR,
     before and after #1045.
   - **The probe.** It compiles 65,537 tracks and then prepares them with
     `maximum_builtin_retained_bytes = 1`. It refuses with exactly
     `builtin.resource.limit $.builtin_compile_caps` in 4.5 s of debug time. Any host cap placed
     earlier would change that refusal.
   - **Recommendation.** Open a successor issue for it. It is not #1045's regression, and
     `host-core/tests` is outside #1045's authorized paths.
4. **Low.** The hand-built plans have 131,075 and 196,612 nodes, and the real graph has 458,761. A
   ceiling keyed on node count between those sizes is nightly-only. That is implausible, so I note
   it without a request.

### Confirmed

- **PBANK closes.** I re-planted it on the merge and ran `cargo test -p graph-compiler --test
  scale` with test-debug-a's features. It goes RED
  (`65,537-track bank attachment: InvalidMembers`), and GREEN on the unmutated merge.
- **Per-PR cost, alone in debug at load 11:**

  | test | wall |
  |---|---:|
  | the banked test | 6.1 s |
  | the routed test | 3.5 s |
  | `:91` | 16.2 s |
  | the whole binary | 16.0 s |

  So the two hand-built tests add no wall time.
- **Gates on the merge.**
  - fmt, `cargo check --workspace --all-targets --all-features` and clippy `-D warnings` pass.
  - session, graph, graph-compiler and builtins-compiler pass in dev (97 s wall) and in release:
    372 passed, 7 ignored.
  - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass under `python3 -B`, and
    actionlint 1.7.7 is clean.
- **Policy scripts, 76 runs, all pass.** That is every `scripts/check-*` and `scripts/test-*`,
  Python ones under `python3 -B`, and `check-stem-store-v1.mjs`.
  - It includes `check-workspace-policy.sh` with #1052's source-scrape lint, and
    `check-graph-determinism.sh` (100 of 100).
  - `test-realtime-audit-probes.sh` ran in all three CI modes.
  - Six were not run because they need external toolchains or built artifacts: capi-abi,
    cross-targets, protocol-wasm-parity, sdk-headless, sdk-types and web-audioworklet.

### Test value

- **Attempt 3's banked test** catches a cap or a narrowed index in bank attachment, or in the
  runtime's bank gather and scatter, above 65,535 lanes (PBANK).
- **The check finding 1 asks for** would catch a cap in the compiler's builtin bank plan or its
  resource accounting above 65,535 tracks (PPLAN, PRES). Nothing else per PR plans banks at this
  size.

## Attempt 4 evidence

Terra, 2026-09-29, on `codex/1045-scale-tests-nightly` at `18e133dc`. There was no new merge;
Sol's attempt-3 check merged batch 4 cleanly. Same host and hygiene as before; the load was
19-50.

### What changed

- **`:91` now plans the compiler's builtin banks.** This is Sol's check from
  `verify-1045/a3/scale-proposal.diff`, as written, with only an assertion message added.
  - It runs before the constrained compile consumes `:91`'s prepared builtins.
  - It calls `graph_builtin_bank_resource(Backend::current(), levels, SessionPoolClasses::from_session(&session))`,
    with one dependency level for each of the three bankable stages. That call goes through
    `planned_strip_banks`, then `plan_bank_groups`, then `builtin_bank_resource`.
  - It asserts `bank_count == 3 × ceil(65,537 / lanes)`.
  - `:91`'s doc comment now says so.
- **`MUTATIONS.md`:** rows 1045-14 to 1045-16 are new. The builtin bank plan leaves the
  residual list.

### Plants

Each plant went into a scratch copy of this tree. I ran `cargo test -p graph-compiler --test scale`
(debug, with test-debug-a's features) and restored the tree between plants. The unmutated tree was
GREEN before and after every plant. PPLAN and PRES are Sol's plants, re-typed into my script.

| # | planted defect | per PR |
|---|---|---|
| PPLAN (1045-14) | `plan_bank_groups` refuses more than 65,535 candidates | **RED**, `:91`: `one node per track per stage, so ids are unique: DuplicateId`. **GREEN once reverted** |
| PRES (1045-15) | `builtin_bank_resource` refuses more than 65,536 lanes | **RED**, `:91`: `the builtin bank plan at 65,537 tracks`. **GREEN once reverted** |
| 1045-16 | `builtin_bank_resource` counts banks through a `u16` | GREEN. This is not a ceiling at this size: 3 × 8,193 banks fit in 16 bits. Discarded |

### Per-PR cost

All times are debug, under the timing lock, at load 36-50.

| run | wall | CPU |
|---|---:|---:|
| `:91` with the check, alone (twice) | 17.2 s; 17.4 s | 16.5 s; 16.7 s |
| `:91` without the check (attempt 3, load 32) | 17.1 s | 16.3 s |
| the banked test, alone | 6.4 s | 6.2 s |
| the routed test, alone | 3.5 s | 3.3 s |
| the binary, default threads | 17.6 s | 26.5 s |
| the binary, `--test-threads 4` | 17.3 s | 25.8 s |

- **The check's cost.** It adds about 1 s to `:91`. Sol measured 16.2 s → 17.5 s at load 10-17;
  my two runs are too noisy to split out a smaller number.
- **The three crates' suites** (graph-compiler, builtins-compiler, session, with test-debug-a's
  features):

  | profile | wall | CPU | tests |
  |---|---:|---:|---|
  | dev | 54.4 s | 207 s | 264 passed, 7 ignored |
  | release | 9.9 s | 14.9 s | 264 passed, 7 ignored |

  In dev, the `scale` binary took 17.0 s; in release, 3.9 s.

### The final layer map

This is Sol's attempt-3 sweep: every layer the brief named, with this attempt's result filled in.
"Per PR" means a test in `test-debug-a`, which is required through `qualification`'s verdict and
which the router runs for every edit to these crates.

| layer | held per PR | by | otherwise |
|---|---|---|---|
| session parse (P1) | no | - | no cheap test: a 65,537-track document is 93.7 MB and takes 19.3 s to parse in debug. It predates #1045; successor B below |
| session compile | yes | `scale_transaction.rs`, `:91` (P2) | - |
| builtin preparation | yes | `:91` (P4, 1045-6, 1045-7) | - |
| graph compile, front end | yes | `:91` (1045-1, -2, -3, P3a) | - |
| graph compile, back end (topo, timings, buffers, estimate) and P3b | no | - | no public entry; it needs the unconstrained compile (+16-28 s). Nightly: `:160` |
| builtin bank plan | **yes (attempt 4)** | `:91`'s check (PPLAN, PRES) | - |
| builtin bank lowering (`into_graph_artifact_with_banks`) and `into_bound` (P5b) | no | - | moderate: 13.4 s of debug CPU to lower a hand-built plan, 28.5 s with bind and render. It is width-bounded per bank, and the layers on either side are held. Nightly: `:160` |
| bank attachment | yes (attempt 3) | the banked hand-built test (PBANK) | - |
| graph bind and render | yes (attempt 2) | both hand-built tests (P5, P6, 1045-11) | - |
| #962 quadratics | no | - | no reliable per-PR clock. Nightly: `:160`'s 60 s bound (RED at 231 s and 451 s) |
| A2, the accounting count through a `u16` | no | - | needs the 65,537-track allocation tracker. Nightly: the 65,537 row |
| host-core preparation, before the graph compile | no, and never was | - | yes, 4.5 s, but outside #1045's paths: successor A below |
| capi prepare, host-web boot | no, and never was | - | no cheap test: both parse the JSON document (19.3 s). Successor B below |

### Proposed successors (root files these; not #1045's regressions)

- **A. Hold host-core preparation above 65,535 tracks per PR.** `prepare_host_runtime` runs
  per-track stages before the graph compile: the count check, track-to-source mapping, effects
  and builtins. A ceiling there is green per PR, before and after #1045.
  - Sol's probe, `verify-1045/a3/probe_host_1045.rs`, compiles 65,537 tracks and then prepares
    them with `maximum_builtin_retained_bytes = 1`. It refuses with exactly
    `builtin.resource.limit $.builtin_compile_caps` in 4.5 s of debug, so any host cap placed
    earlier would change that refusal.
  - Paths: `crates/host-core/tests/`.
- **B. Parse, capi prepare and host-web boot above 65,535 tracks.** No test parses a document
  this large, anywhere.
  - A 65,537-track document is 93.7 MB, and `parse_session_json` takes 19.3 s of it in debug
    (Sol's `probe_parse_1045.rs`). capi prepare and host-web boot pay that parse too.
  - `session/tests/descriptive_scale.rs` parses 65,536 tracks, but it is `#[ignore]`d and nothing
    schedules it.
  - The likely shape is a nightly job, or one release-mode parse shared by the three entries.

### Gates re-run

- **Build checks:** `cargo fmt --all --check`, `cargo check --workspace --all-targets
  --all-features` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` all
  pass with no warnings.
- **The three crates in dev and release:** pass (above).
- **Policy scripts:** 54 pass.
  - That is every `scripts/check-*`, with the Python ones under `python3 -B`. The ones that
    need an argument ran with `--self-test`; the V8 spill check ran with `--check-toolchain`.
  - It also includes `test-workspace-policy.sh`, `test-ci-path-routing.py` and
    `test-graph-policy.sh`, plus `check-workspace-policy.sh` (which carries #1052's
    source-scrape lint) and `check-graph-determinism.sh`.
  - Two were not run: `check-sdk-types.sh` (no `sdk/node_modules` here) and
    `check-web-boot-budget.mjs` (it needs a built module).
- **Not re-run:** the nightly commands, the mutation passes and the #966/#970 reverts. Product
  code, `nightly.yml` and the nightly tests are unchanged since attempt 2, and Sol re-ran the
  nightly commands on the batch-4 merge.

### Test value

`:91`'s bank-plan check catches a ceiling in the compiler's builtin bank plan or its resource
accounting above 65,535 tracks (PPLAN, PRES). Nothing else plans banks at this size per PR.

### Remaining risk (replaces attempt 3's list)

These are caught nightly only, at most a day late (amendment 5):

- the graph compile's back end and P3b;
- builtin bank lowering and `into_bound` (P5b);
- #962's quadratic compile and bind (the 60 s bound);
- A2.

These are outside #1045:

- successors A and B;
- #1002's memory claim;
- #967's banked-and-routed bind.

### Lines

Attempt 4: `scale.rs` +38/−2, `MUTATIONS.md` +11/−7, and this spec.

## Sol verdict, attempt 4

Sol, 2026-09-29, head `cbc13b7b`. I merged it into `codex/batch-slim-4` (`b6fd4274`, which is
`3a2782f1` plus #1075) in a scratch worktree and checked it there. Host: `x86_64` (`x86-64-v3`),
rustc 1.97.1, `CARGO_INCREMENTAL=0`, `nice`, load 15-25.

**FAIL.** PPLAN and PRES are closed. But the map calls one more layer "nightly, no cheap test",
and that is wrong: the builtins' bank lowering (`into_graph_artifact_with_banks`). This is my
attempt-3 misjudgement ("moderate"), and the map copied it.

- **What I measured.** A per-PR test that lowers 65,537 tracks costs about 13 s of debug CPU. It
  adds about half a second to the `scale` binary's wall time.
- **What it is.** Lowering is a public pipeline stage. `:91` and `:160` held it per PR before this
  change.
- **The rule this breaks.** It is the brief's FAIL condition.

To make attempt 5 mechanical:

- **Complete map.** The map below covers every stage and every per-track feature. I checked each
  against the code.
- **Rule for "cheap".** A test is cheap when it adds about a second or less to the `scale`
  binary's wall time, running beside `:91`, and costs at most about 15 s of debug CPU.
- **My commitment.** If attempt 5 adds the test below, the plants go red, and the gates pass, I
  will PASS it. I will raise no layer outside this map.

### The merge

- **No conflicts.** Batch 4 still leaves `nightly.yml` alone. The routing scripts merge cleanly.
- **Root applies one text fix.**
  - #1046 (on the batch) deletes `crates/session/tests/descriptive_scale.rs`.
  - Successor B's line in the attempt-4 evidence ("`descriptive_scale.rs` parses 65,536 tracks,
    but it is `#[ignore]`d") is therefore stale on the merge.
  - It should say that nothing parses a document of this size at all.
- **`nightly.yml`.** `failure-notice` needs and reports `math-sweeps` and `release-budgets`.
  `math-sweeps` has the session sweep after F1 and a 25-minute timeout.
- **No semantic conflict.** #1046 also trimmed `graph/src/lib.rs` and `builtins-compiler/src/lib.rs`.
  On the merge, the three `release-budgets` 65,537-track commands pass, and `:160` takes 17.7 s
  against its 60 s bound.

### Findings, by severity

1. **High, blocking: a cap in builtin bank lowering is green per PR, and a parallel test catches it
   at no wall cost.**
   - **Where the stage sits.** `compile_with_builtins` hands the compiled graph to the public
     `PreparedBuiltinsSession::into_graph_artifact_with_banks`. That call builds every input, fader
     and matrix bank, then calls `with_builtin_banks`.
   - **Why nothing per PR reaches it.** The constrained compile refuses before it, and the
     hand-built tests skip it.

   | plant | per PR on the attempt-4 tree (session, graph, graph-compiler, builtins-compiler, rack-compiler) | nightly `:160` | the proposed test |
   |---|---|---|---|
   | LOWER_PANIC: lowering asserts at most 65,535 tracks | GREEN | RED: `builtin bank lowering track limit` | RED, the same panic |
   | LOWER_SILENT: above 65,535 tracks, lowering silently banks nothing | GREEN | RED: bank member count 0 | RED: the bank count assertion |

   - **The proposed test.** It is in `scratchpad/verify-1045/a4/lowering-probe.diff`, and it is
     a separate `#[test]` in `scale.rs`, so it runs beside `:91`.
     - It prepares the scale session's builtins.
     - It builds a hand-built strip plan with `track_stages`, `edge` and `hand_built_plan`: input,
       post-input, fader, matrix, output.
     - It lowers the plan with `into_graph_artifact_with_banks(plan, (), Backend::current(), levels, classes)`.
     - It asserts `prepared_builtin_bank_count() == 3 * ceil(65,537 / lanes)` and
       `graph().builtin_bank_members().count() == 3 * 65,537`.
   - **Its cost.**

     | run | wall | CPU | peak memory |
     |---|---:|---:|---:|
     | the test alone | 13.8 s | 13.0 s | 0.8 GB |
     | the `scale` binary with it (`--test-threads 4`) | 17.8 s | - | 1.6 GB |
     | the `scale` binary without it | 17.2 s | - | - |

     It is GREEN unmutated. It also catches PPLAN and PRES.
   - **Attempt 5 must:**
     - add the test;
     - record LOWER_PANIC and LOWER_SILENT red and green once reverted;
     - move lowering to "held" in the map.
2. **Medium: the attempt-4 map is not complete. With finding 1 applied, this is the map.**
   - **Terms.** "Per PR" means `test-debug-a`. That job is required through `qualification`'s
     verdict, and the router's `full` route covers every crate below.
   - **Cheap.** As defined above.

   | layer | status | by, or why |
   |---|---|---|
   | session parse (P1) | pre-existing gap | successor B |
   | session validate, estimate, canonical write, compile | per PR | `scale_transaction.rs`; `:91` (P2) |
   | builtin preparation | per PR | `:91` (P4, 1045-6, 1045-7) |
   | graph compile, front end (lowering to nodes and edges, node cap, cycle check) | per PR | `:91` (1045-1/-2/-3, P3a) |
   | graph compile, back end: `topo`, PDC `timings`, `buffer_assignments`, `ports_for`, rack cohorts, `resource_estimate`, `effect_control_resource`, `effect_bank_resource`, graph's count-arithmetic helpers (`GraphRuntimeMetadataResourceEstimate`, `GraphBankSlotResourceEstimate`), the capped-estimate checks, `PreparedGraphPlan::new` on the real plan; P3b | nightly only (`:160`) | no public entry except the count helpers. Those take counts that only this stage computes, so feeding them made-up counts tests arithmetic, not the pipeline. Running the stage costs the unconstrained compile, +11-23 s wall |
   | builtin bank plan (`graph_builtin_bank_resource` to `plan_bank_groups` to `builtin_bank_resource`) | per PR | `:91`'s check (PPLAN, PRES; re-planted, both RED) |
   | **builtin bank lowering (`into_graph_artifact_with_banks`)** | **per PR once finding 1 lands** | the proposed test (LOWER_PANIC, LOWER_SILENT) |
   | bank attachment (`with_builtin_banks`) and the runtime's bank gather and scatter | per PR | the banked hand-built test (PBANK) |
   | `into_bound` prevalidation (P5b) | nightly only | it needs a lowered artifact and a real bind: +6 s wall beyond the lowering test. Its checks are set comparisons with no count in them |
   | graph bind (`PreparedGraphPlan::bind`: lowering to the program, `preflight_sequential`, route fold, scatter redirects) and render | per PR | both hand-built tests (P5, P6, 1045-11) |
   | real builtin bank kernels rendering | nightly only | each is bounded by the bank width. The runtime that iterates them is held per PR |
   | #962's quadratic compile and bind | nightly only | there is no reliable per-PR clock. `:160`'s 60 s bound was RED at 231 s and 451 s |
   | A2 (accounting through a `u16`) | nightly only | an under-count, not a cap. It needs the 65,537-track allocation tracker |
   | per-track session features: effects and effect controls, one route per track, sources and source-set bind, per-track observers and meters (observation-activation bind), sidechains | pre-existing gap | never exercised at this size, even by the old `:160`. Successor: **#967** (open) already names effects with controls, one route per track, sources, observers and meters, and sidechains at scale |
   | console controls, sends and submixes, automation on every track | pre-existing gap | not named anywhere. Add them to #967's shapes |
   | host-core preparation | pre-existing gap | successor A |
   | protocol session store edits, capi prepare, host-web boot (and the native and mobile shells over them) | pre-existing gap | successor B |

3. **Successors A and B are honestly pre-existing, not blocking.**
   - **Checked on `origin/main` (`a8955ad4`).** No test in `host-core`, `capi`, `hosts`,
     `protocol` or `source` builds more than 65,535 tracks. The `65_536` hits in host-web are byte
     limits.
   - **What main had.** Its only large-document parse was `descriptive_scale.rs`, which was
     ignored and never scheduled.
   - **What `:160` reached.** Even before #1045, `:160` never reached these entries. It started
     from a model, not from a document or a host.
   - **Scopes for root to file:**
     - **A.** Hold `host-core` preparation above 65,535 tracks per PR. Prepare a compiled
       65,537-track session through `prepare_host_runtime` with a configured later refusal
       (`maximum_builtin_retained_bytes = 1`), and assert exactly `builtin.resource.limit
       $.builtin_compile_caps`. That takes 4.5 s in debug (`verify-1045/a3/probe_host_1045.rs`);
       the paths are `crates/host-core/tests/`.
     - **B.** Hold document parse, the protocol session store, capi prepare and host-web boot
       above 65,535 tracks. A 65,537-track document is 93.7 MB and takes 19.3 s to parse in debug,
       so parse once in a release-mode nightly gate and drive each entry from it.
4. **Low, for an owner ruling, not a test gap: a V1 wire field is `u16`.**
   - **The field.** The protocol capability `maximum_telemetry_handles` is a `u16`
     (`schema.rs` field 23).
   - **The effect.** A V1 provider cannot advertise more than 65,535 meter handles, so one
     telemetry configuration cannot meter more than 65,535 tracks.
   - **Why it is only a ruling.** It is a sealed wire contract, and telemetry is noncritical.
     Whether that is acceptable under the no-track-cap rule is the owner's call.

### Confirmed

- **PPLAN and PRES are closed.** I re-planted both on the merge and ran the per-PR suites of
  session, graph, graph-compiler, builtins-compiler and rack-compiler.
  - PPLAN: RED at `:91` (`DuplicateId`).
  - PRES: RED at `:91` (`the builtin bank plan at 65,537 tracks`).
  - Both are GREEN on the unmutated merge. The tests run in the required `test-debug-a`.
- **Gates on the merge.**
  - fmt, `cargo check --workspace --all-targets --all-features` and clippy `-D warnings` pass.
  - session, graph, graph-compiler, builtins-compiler and rack-compiler pass in dev (106 s wall;
    the `scale` binary 17.2 s) and in release: 379 passed, 6 ignored.
  - The three nightly commands pass.
  - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass under `python3 -B`, and
    actionlint 1.7.7 is clean.
- **Policy scripts, 75 runs, all pass.** That is every `scripts/check-*` and `scripts/test-*`,
  Python ones under `python3 -B`, `test-realtime-audit-probes.sh` in its three CI modes, and
  `check-stem-store-v1.mjs`.
  - It includes `check-workspace-policy.sh` with #1052's lint, and `check-graph-determinism.sh`.
  - Six were not run because they need external toolchains or built artifacts: capi-abi,
    cross-targets, protocol-wasm-parity, sdk-headless, sdk-types and web-audioworklet.

### Test value

- **`:91`'s bank-plan check** catches a cap in the compiler's builtin bank plan or its resource
  accounting above 65,535 tracks (PPLAN, PRES).
- **The lowering test finding 1 asks for** would catch a cap, or a silent fallback that skips
  banking, in builtin bank lowering above 65,535 tracks (LOWER_PANIC, LOWER_SILENT). Nothing else
  per PR lowers banks at this size.
