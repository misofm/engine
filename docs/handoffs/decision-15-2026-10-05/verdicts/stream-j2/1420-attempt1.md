PASS

# #1420 attempt 1 verdict (decision-15 stream J batch 2, under Amendment 1)

Commit `f48e87b1c` on `codex/d15-stream-j2`. I reviewed `git diff f48e87b1c^ f48e87b1c` against the
spec (D1-D5, Hazards, Amendment 1, gates, Evidence), AGENTS.md (test value, ceremony boundary) and the
owner principle. I built and tested an export (`git archive`) in `/tmp/claude-1002/v1420`. I did not
touch the worktree. The diff changes only the `tests` module of
`crates/builtins-compiler/src/lib.rs` and the spec, so it stays inside the authorized paths. There is no
queue, so the acked-batch question does not apply.

## Spec conformance

- **D1.** M1 is red. The M3, M5 and M7 arms are gone: no per-variant `selected_index` match is left, and
  `output_track` has a `NonadjacentOutputConflict` case only (`lib.rs:6178-6185`).
- **D2.** Met. The #916 test names `t00`/`t01` by literal ID for the `Output` edge source of
  `track_graph_variant(2, _)` (`lib.rs:8042-8068`). The harness builds its graph with the same function
  (`lib.rs:6948`).
- **D3 (Amendment 1).** Met, through the observation path. The witness exists in both feature sets
  (`graph/src/runtime.rs:262-285`). The harness takes `selected_index` from the witness
  (`lib.rs:7150-7181`). It re-derives nothing, so no production-rule mutant is needed.
- **D4.** Met. The `NonadjacentTrackA` output arm is deleted and the reason is recorded. M5-reverse
  stays green in both configurations (see below), so no outcome reads that arm. Moving the `Output` to
  t00 broke no render comparison, and no test pins a value. In
  `..._invalid_fader_matrix_boundaries_preserve_retry_order`, the final PCM now observes t00 while the
  capture observes t01, so that test observes slightly more than before.
- **D5.** Met. Both `builtins-compiler` clippy runs are clean. Their only warnings are the existing
  `clippy.toml:80-81` "does not refer to a reachable function" config warnings, which this commit does
  not touch.
- **Evidence.** Present: the gate-1 table, where D3's check lives (the harness), observed versus
  re-derived, and the `PAIR_WITNESS_LOCK` note. The record's candid note on test value is accurate (see
  MINOR-1).

## BLOCKER

None.

## MAJOR

None.

## MINOR

**MINOR-1 (needs a root ruling). The witness-keyed nonadjacent block in the harness has no unique
catch. The selected track is still unobservable.** (`lib.rs:7006-7010`, `7150-7227`)

- **The slot assertions cannot go red from production.** They read only the schedule the harness built
  itself (`nonadjacent_schedule` is cloned from `graph.sequential_schedule` right after the harness
  assigns it).
  - With `n == 2` and a stage-major schedule, both orientation branches hold.
  - Mutant SWAP keys the slot assertions on the other track (`selected_index = n - 1 - selected_index`
    after the mapping). The full test-support package stays **green**.
  - So the "which track is A" choice is still a harness choice that changes nothing, which is the
    problem D1 names. It is now correct by construction instead of stale, but no test can see it.
- **D3's test-value sentence cannot come true on this path.** The sentence is "red if the harness's
  'A' track and production's selected pair disagree". Under the observation path the two agree by
  definition. This follows from the spec (Amendment 1 prefers observation, and the Problem section
  already admits the n == 2 symmetry). It is not an implementer error.
- **The selection itself is defended elsewhere.** Mutant PREV makes production select the last
  candidate, `(0..run_units.len()).rev()` at `runtime.rs` `'split`. It turns 4 existing literal-ID tests
  red (`_schedule_selects_the_split_owner`, `_overlapping_..._select_one...`,
  `_failed_render_materializes...`, `_ramp_retarget...`). The harness block fires on none of them.
- **Recommendation for root:**
  - Option 1, preferred under D1 and AGENTS.md's ceremony boundary: delete the whole witness-keyed
    block (the schedule clone, the witness read, the presence check, the mapping and the slot
    assertions). Record that `selected_index` is gone with its arms because no harness assertion can
    observe it at n == 2.
  - Option 2: keep the block as an explicit fixture self-check. State in its comment that it cannot
    tell the orientation. Also correct the carried-over messages: "production scalar schedule places
    F_B ..." (`lib.rs:7198`, `7224` and siblings) describes the harness's hand-built schedule, not
    production's.

**MINOR-2. The new presence check `selected.is_some() == between_render_calls` (`lib.rs:7158-7164`)
has no unique catch.** I tested it with and without the check (BLESS = check removed):

| Mutant | With the check | Without the check (BLESS) |
| --- | --- | --- |
| PSTALE: drop the witness reset in `build_sequential` (`runtime.rs:5722-5723`) | red at the check | still red: `..._failed_render_materializes_post_fader_before_error` (`lib.rs:8406`, `separate.test_only_post_fader_node.is_none()`). `graph`'s own `a_metered_redirect_consumer_stays_out_of_the_split_pair_and_keeps_the_bits` (`runtime.rs:13171`) is also red. |
| PNONE: `scalar_split_pair_factory` returns `None` | red | still red: 9 lib tests + 2 `allocation_tracker` tests |
| PSEP2: the concurrent (separate) twin gets a split factory, both guards removed (`lib.rs:4415-4418`, `4795-4796`) | red | still red: 5 lib tests + `actual_scalar_split_table_and_failed_render_fit_the_resource_gate` |
| PSEP: factory gate only | green | green. This mutant is equivalent: `make_scalar_split_pair` rechecks delivery at `lib.rs:4795`. |
| WITNONE: the harness's witness read returns `None` | red in 7 tests | n/a: this is a defect in the check's own read, not a plausible product defect |

- The check does have one design reason: it stops the `if let` from silently skipping the slot block.
  That reason disappears with MINOR-1, because the slot block itself catches nothing.
- Earlier verdicts graded rows with no unique catch as NIT or MINOR (#1234 NIT 2, #1335 MINOR 2), so
  this is MINOR.
- Remove the check together with the block if root takes MINOR-1 option 1. Under option 2, say in its
  comment that it only guards the fixture against a silent skip.

## NIT

- **NIT-1. The harness comment overstates the lock** (`lib.rs:7150-7154`). It says the read is
  race-free "under the caller's `PAIR_WITNESS_LOCK`".
  - The read is correct because the witness is thread-local and `build_sequential` resets it on every
    build (`runtime.rs:262-285`, `5722-5723`), not because of the lock.
  - The test-support callers in `crates/builtins-compiler/tests/allocation_tracker.rs` (via
    `test_only_prepared_scalar_split_pair_graph*` and `test_only_observed_scalar_*split_pair_binding`)
    hold `SESSION`, not `PAIR_WITNESS_LOCK`, and they run this read too.
  - The spec's Hazards wording has the same imprecision. If the block stays, word it as "thread-local
    and reset per build; the lock is not what makes it race-free".

## Test value (one sentence per new or rewritten assertion)

- **D2 `Output`-source assertion** (`lib.rs:8056-8068`): it is red if the harness feeds
  `NonadjacentOutputConflict`'s `Output` from t00 instead of t01 (M1). That defect makes the #916 test
  pass without testing the output-track case it names. It is unique: M1 + D2 removed leaves the #916
  test green, and M1 alone fails only that test, at `lib.rs:8060`.
  - `sources.len() == 1` (`lib.rs:8053`) is the guard for indexing `sources[0]`. It makes no separate
    claim.
- **Presence check** (`lib.rs:7158-7164`): no unique catch (MINOR-2).
- **Mapping panic** (`lib.rs:7166-7181`): this is a guard on `find`, not a claim. Production cannot
  reach it, because the split pass records only a `TrackStage::PostFader` whose matrix is on the same
  track (`runtime.rs:5640-5657`, `5675`).
- **Witness-keyed slot assertions** (`lib.rs:7182-7227`): no unique catch. They read the harness's own
  schedule, and SWAP is green (MINOR-1).
- **Adjacent-branch assertion with literal `n - 1`** (`lib.rs:7011-7029`): this rewrite keeps the old
  behaviour, because the old `selected_index` was `n - 1` for every non-nonadjacent variant. The
  existing claim is unchanged, so it is not a new claim.

## Mutation runs

All runs are on the export, with `cargo test --locked -p builtins-compiler --features test-support`
(`--no-fail-fast` for the production mutants) unless the table says otherwise. Each run started from
pristine sources, and I restored and diff-checked them afterwards.

| Mutant | Result |
| --- | --- |
| M1: `NonadjacentOutputConflict` output `n - 1` -> `0` | red: 53 passed, 1 failed. Only `actual_scalar_nonadjacent_output_track_takes_the_split_pair_now_the_output_is_dedicated` fails, at D2 (`lib.rs:8060`, "NonadjacentOutputConflict feeds the Output from t01's post-matrix") |
| M1 + D2 loop removed (that test only) | green, so D2 is the unique catch |
| M3, M7 | arms deleted (checked in the diff) |
| M5 | arm deleted |
| M5-reverse: `NonadjacentTrackA` output back to `n - 1` | green in test-support (full package) and with no features (`--lib`, 47 passed), so the deletion changes no outcome |
| SWAP: slot assertions keyed on the other track | green, so the orientation is unobservable |
| WITNONE, PSTALE, PNONE, PSEP, PSEP2, each with and without the check | see MINOR-2 |
| PREV: production selects the last candidate | red in 4 literal-ID tests; the harness block stays silent |
| PSTALE on `cargo test -p graph --features test-support` | red: `a_metered_redirect_consumer_stays_out_of_the_split_pair_and_keeps_the_bits` |

## Gates run (export of `f48e87b1c`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1420/target`)

All pass.

- Gate 2: `cargo test --locked -p builtins-compiler --features test-support`
  - lib: 54 passed.
  - Integration: `allocation_tracker` 11 passed (1 ignored), plus 3, 5, 1 and 2 passed.
- Gate 2: `cargo test --locked -p builtins-compiler`
  - lib: 47 passed.
  - Integration: 3, 4, 1 and 2 passed.
- Gate 3: `cargo clippy --locked -p builtins-compiler --all-targets -- -D warnings` and the same with
  `--features test-support`. Both are clean, apart from the existing `clippy.toml` config warnings.
- Gate 4:
  - `cargo fmt --all -- --check`
  - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
  - `bash scripts/check-builtins-policy.sh` (ok)
  - `bash scripts/check-workspace-policy.sh` (ok)
