# Run the `test-support` tests CI never runs, and fix the one that fails

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 5.2. This is the
prerequisite for every cleanup draft beside it: each of them promises that "no test that guards a
live claim is lost". That promise means nothing while some of those tests never run.

## Context

**Which features CI turns on.**

- `test-debug-a` runs `cargo test --workspace` with
  `--features builtins-compiler/test-support,source/test-support,graph/test-support,engine/realtime-audit`.
- `test-debug-b` runs `-p parametric-eq …` with `--features math/lane`.
- `cargo tree --workspace -e features,normal,dev -i <crate>` with those flags shows that
  `test-support` is **never** turned on for `host-web`, `host-core`, `effect-compiler` or
  `parametric-eq`.
- Clippy (`--all-features`) compiles the gated tests, but nothing runs them.

**The audit ran them once, on `a9414c0c`:**
`cargo test -p host-web -p host-core -p effect-compiler -p parametric-eq --features host-web/test-support,host-core/test-support,effect-compiler/test-support,parametric-eq/test-support`.
588 passed and **1 failed**:

```text
tests::acknowledged_pair_render_records_the_same_live_dispatch
panicked at hosts/host-web/src/tests.rs:3220:5
assertion `left == right` failed  left: 2  right: 1      (witness.process_calls)
```

The test was added by #466 ("Fuse eligible live fader and matrix banks with boundary proof",
2026-09-05). It asserts that an acknowledged fader and matrix pair reaches the fused fader-matrix
kernel in one call. Some later change made that two calls, and nothing noticed.

Live-claim tests in the never-run set include:

- `host-web` `tests.rs:2391`, `:2504`, `:2549`, `:3179`, `:3267`, `:3535`
  (`prepared_eq_owner_transaction_is_design_and_allocation_free_after_preparation`), and `:3866`;
- `host-core/tests/observation_demand.rs:526`
  (`dormant_controlled_spectrum_does_no_capture_work_on_render`);
- the `parametric-eq` counters behind `cfg(feature = "test-support")` at `src/lib.rs:141` and
  `:147`.

## Smallest closable slice

1. **Find why the failing test fails.** Bisect from `6589c518` (#466) to `main` for the commit
   where `acknowledged_pair_render_records_the_same_live_dispatch` first fails.
   - If the second fused call is a regression (the pair should still be one fused dispatch), fix
     the code.
   - If the dispatch shape changed on purpose (for example, a later fusion splits the pair by
     design), update the assertion. State in the evidence which issue changed it and why the new
     count is the intended claim.
   - Either way, no other test changes.
2. **Add the missing features to CI.**
   - Add `host-web/test-support,host-core/test-support,effect-compiler/test-support` to
     `test-debug-a`'s `--features` list.
   - Add `parametric-eq/test-support` to `test-debug-b`'s.
   - Update the local-reproduction comment above each step.
3. **Record the test lists.** Record `cargo test … -- --list` for both jobs before and after. The
   "after" list is the baseline every later cleanup draft compares against.

## Objective gates

1. The `test-debug-a` and `test-debug-b` commands, with the new features, pass locally with 0
   failures. The evidence lists the tests newly enabled: the `-- --list` difference with and
   without the features. For reference, the audit's single run of the four crates with the
   features ran 589 tests in all: 588 passed and 1 failed.
2. **Native and wasm build:**
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` passes.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web` passes.
3. **Console digests unchanged.**
   - `cargo test --locked --release -p console-workload --test gain_pan_profile -- --ignored --exact digests --nocapture`
     is byte-identical on base and change.
   - If step 1 changes product code, `bash scripts/run-wasm-gates.sh` passes.
4. **Shipped artifact.** Unchanged if step 1 only edits a test.
   - If step 1 fixes product code, build base and change on the same machine with
     `scripts/build-web-audioworklet.sh --module-only EMPTY_DIR`.
   - Explain every function whose disassembly differs, and re-pin with that reason.
5. **CI routing.** `python3 -B scripts/check-ci-path-routing.py` and
   `python3 -B scripts/test-ci-path-routing.py` pass. The job names and the `verdict` expectation
   table are unchanged, because only `--features` lists change.
6. **No live claim lost.** Nothing is deleted. The only test edit allowed is the one step 1
   justifies.

## Dependencies

None. It blocks `01-…` to `08-…` and every `R…` draft.

## Standing rules for the implementer

- If the failing test exposes a real regression in the fused fader-matrix path, stop and report
  before fixing: it is a product bug and may deserve its own issue.
- Commit on `codex/<issue>-run-test-support-tests`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

Reproduced on `a9414c0c`; see `../VERIFY-DEAD-CODE.md`, finding F1.

1. **The failure is already bisected. It is an intended dispatch change that the never-run test
   missed, not a product regression.**
   - First failing commit: `608f0379` (#916, 2026-09-25, "write the master straight into the host
     planes"), merged to `main` by `97435208` (#922). Its parent line (`86be6792`) passes.
   - #916's own scope amendment `9c762d7c` records the reason: the Output became dedicated
     storage, so "track 0 pairs" and the builtins-compiler harness counts were updated. The host-web
     test is behind `host-web/test-support`, so nobody saw it.
   - The witness on `main` reads `process_calls 2, fused_calls 2, fallback_calls 0,
     process_members 9, fader_records_drained 1, matrix_records_drained 1`. With only the three
     count assertions changed (2, 9, 2), every other assertion in the test passes, including the
     drained-record counts and the output values (checked in a scratch copy).
   - So step 1 is a test edit. Replace "bisect" with: update the counts, and re-word the
     `process_members` message, which still says "the selected scalar track-8 tail". The witness
     now aggregates the fused bank call and the tail call. If the claim "the acknowledged records
     reached track 8" must stay discriminating, add a per-call member witness rather than
     asserting the aggregate alone. Cite `608f0379` and `9c762d7c` in the evidence.
   - Gate 4 then applies in its "only edits a test" form: the shipped artifact cannot move.
2. **What is actually never run is smaller than "588 tests".** The four crates' feature-on run is
   589 tests, but CI already runs all except:
   - four whole tests: host-web `acknowledged_pair_render_records_the_same_live_dispatch`,
     `prepared_eq_owner_transaction_is_design_and_allocation_free_after_preparation`,
     `prepared_mixed_eq_builtin_fader_commits_and_refuses_atomically`, and host-core
     `dormant_controlled_spectrum_does_no_capture_work_on_render`;
   - gated assertion blocks inside tests that do run: host-web `tests.rs:9318-9324`,
     `:9467-9486` (spectrum operation counts); host-core `tests/observation_demand.rs:1511`,
     `:1970`, `:2040`; parametric-eq `tests/bank.rs:913-923`, `:1344-1354` (the pass counts
     resolve to `None` without the feature, so their assertions are skipped).

   Gate 1's `-- --list` diff will therefore show 4 added names, not hundreds. The evidence must
   also show the gated assertion blocks executing (for example a red run with one count altered).
   `tests.rs:2391`, `:2504`, `:2549` and `:3267` are gated helpers, not tests.
3. **Add a recurrence guard.** The gap existed because nothing checks that a `test-support`
   feature is turned on by some CI test job. Add one small check, for example in
   `scripts/check-ci-path-routing.py` or its own script with a mutation test: every workspace
   package that declares `test-support` must be enabled, directly or by forwarding, in the
   `--features` of a `cargo test` step that tests that package. It must go red if a feature is
   removed from `test-debug-a` or `test-debug-b`.
4. **The operator-script half of the "broken" finding is not fixed by this draft.** It is spread
   over 04b (deletes `preflight-rack-benchmark.sh`), 04c (deletes `run-wasm-kernel-timing.sh` and
   the seal script, repairs `prepare-builtins-listening.sh`) and R9 (deletes the wasm console
   pair). If R9 is ruled "keep", or waits, nothing repairs the wasm console scripts. See the new
   draft `00b-repair-the-operator-script-roots.md`.
5. **Mobile scope.** No change to this draft. Instrumented builds are native x86 only.

## Attempt 1 evidence

Implementer: Terra (Claude Opus 5.5), branch `codex/1021-run-test-support-tests` from
`codex/batch-slim-1` at `d70956bf`. Commits: `cb6cb530` (the test), `df847b72` (CI features and
the guard). Nothing is pushed.

### Step 1: the failing test (test-only)

- **Reproduced on the base.** `tests.rs:3220`, `process_calls` left 2, right 1.
- **Cause, as Amendment 1 says.** #916 (`608f0379`, scope amendment `9c762d7c`) made the session
  Output dedicated storage, so track 0 pairs. Tracks 0-7 now take one fused eight-lane bank call
  before the track-8 tail.
- **Witness on the base:** `process_calls 2`, `fused_calls 2`, `fallback_calls 0`,
  `process_members 9` (8 + 1), records drained 1 and 1. This is the intended dispatch, not a
  regression.
- **Edit.** The counts become 2, 9, 2 (`fused + fallback`) and 2 (`fused`).
  - The `process_members` message now reads "eight bank members plus the one scalar track-8 tail
    member".
  - The claim "the acknowledged records reached the selected track-8 tail" moves to the per-call
    witness that already exists: `first_left_bits`/`first_right_bits`, which the last call records.
    Only track 8 carries the acknowledged -6 dB and `[0.5, 0, 0, 1]`.
  - The `fused_calls` message now says both calls fuse.
  - Nothing else in the test and no other test changes.
- **The claim still discriminates.** In a red run with both records aimed at track 0 instead of
  track 8, the counts still pass, but the reworded assertion fails with `0.5`: that is track 8's
  identity output.
- **A per-call member field was tried and dropped.** I added `first_output_members` to the
  test-support witness in `builtins-compiler/src/lib.rs`, compiled only under
  `cfg(any(test, feature = "test-support"))`. It moved the shipped artifact anyway:
  `4288feac…` against the pin `476e58ad…`.
  - The cause is line numbers. Inserting lines in that file shifts the panic-location line numbers
    compiled into the release wasm.
  - A single added comment line inside the test-support-only struct also moved it (`eb545691…`).
  - Amendment 1 says the artifact cannot move, so the test uses the existing output witness
    instead.

### Step 2: the CI features

The guard (below) found two packages the audit missed: `protocol` and `builtins`.

| job | `--features` added | what now runs |
|---|---|---|
| `test-debug-a` | `host-web/test-support`, `host-core/test-support`, `effect-compiler/test-support`, `protocol/test-support` | the 4 tests from Amendment 2, protocol's `controller_facade_preparation_has_one_queue_allocation_authority`, and the gated blocks in host-web and host-core |
| `test-debug-b` | `parametric-eq/test-support`, `builtins/test-support` | the gated pass-count blocks in `parametric-eq` `tests/bank.rs` and the peak-merge block in `builtins` `tests/meter.rs` |

- **Forwarding makes some entries redundant.** `host-web/test-support` also forwards to
  `host-core`, `effect-compiler`, `builtins-compiler` and `parametric-eq`. The explicit entries
  follow the brief.
- **The comments above both steps are rewritten.** The old test-debug-a comment said every named
  feature is unified in anyway. That was true only for `builtins-compiler` and `graph`: nothing
  enables `source/test-support` under that step without the flag.
- **Workflow shape.** Job names, the router, leaf `route` gating and the verdict table are
  unchanged. The lint job gains one step (the guard). No job is added.

### Step 3: test lists (`cargo test <job command> -- --list`, names with their binary)

- **test-debug-a:** 1529 names before, 1534 after. The five added names:
  - `host_web` `tests::acknowledged_pair_render_records_the_same_live_dispatch`;
  - `host_web` `tests::prepared_eq_owner_transaction_is_design_and_allocation_free_after_preparation`;
  - `host_web` `tests::prepared_mixed_eq_builtin_fader_commits_and_refuses_atomically`;
  - `observation_demand` `dormant_controlled_spectrum_does_no_capture_work_on_render`;
  - `delivery_ownership` `controller_facade_preparation_has_one_queue_allocation_authority`.
- **test-debug-b:** 834 names before and after. Its gain is gated assertions only.
- **The "after" lists are the baseline** for later cleanup drafts. The same lists come out of the
  final tree.

**Gated blocks proven to execute.** In each red run I altered one assertion or counter in a
scratch edit, ran with and without the feature, then restored the file.

| block | mutation | feature on | feature off |
|---|---|---|---|
| host-web spectrum counts (`protected_eq_boot_is_dormant…`, `protected_eq_controls_use_native_receipts…`) | `assert_eq!` to `assert_ne!` | 2 FAILED | 2 ok |
| host-core `observation_demand.rs` (`continuous_spectrum_replaces…`, `protected_track_spectrum_preserves…`) | `assert_eq!` to `assert_ne!` | 2 FAILED | 2 ok |
| parametric-eq `bank.rs` pass counts | counts forced to 0 | `odd_live_counts_render_the_base_bits`, `admitted_blocks_render_the_base_bits_without_selects` FAILED | 11 ok |
| builtins `meter.rs` peak merges | count + 1 | `a_block_peak_merge_publishes_the_scalar_meters_snapshots` FAILED | 13 ok |

The first parametric-eq mutation (count + 1) stayed green. Those assertions use `after - before`
deltas and `> 0`, so they had to be forced to 0 instead.

### Recurrence guard

`scripts/check-test-support-ci.py` is static: it needs no toolchain and reads only the manifests
and `qualification.yml`.

- **The rule.** Every workspace package that declares `test-support` needs an unconditional,
  whole-package `cargo test` step that enables `<pkg>/test-support`, in `--features` or by
  `[features]` forwarding.
  - "Whole-package" means: it selects the package, has no `--no-run`, no narrower target than
    `--all-targets`/`--tests`, and no harness filter.
  - A feature turned on by a dependency declaration does not count, because a local
    `cargo test -p <pkg>` would not reproduce it.
- **It fails on the base:** `builtins, effect-compiler, host-core, host-web, parametric-eq,
  protocol`. It passes now, for all 10 packages.
- **`scripts/test-test-support-ci.py`** holds 20 hermetic mutation cases: 17 must fail and 3 must
  pass. Each red case asserts the exact set of packages reported as uncovered. It goes red when:
  - any non-redundant feature, or all of them, is removed from test-debug-a or test-debug-b;
  - host-web is excluded;
  - the step gets a step-level `if:`, `--no-run`, `--lib`, a harness filter or a positional
    filter;
  - the feature appears only in a shell comment;
  - a new package declares `test-support`;
  - a feature name is misspelt;
  - host-web's manifest stops forwarding `host-core/test-support`.

  It stays green when a redundant entry is removed and forwarding still covers it. The unmutated
  baseline must pass and report every declaring package.
- **The guard runs in the lint job.** `check-ci-path-routing.py` pins that step, and a new
  mutation case in `test-ci-path-routing.py` covers the pin.

### Objective gates

1. **Tests.** Both job commands pass with the new features, from a workspace with cleaned
   workspace members:
   - test-debug-a: 1526 passed, 0 failed, 10 ignored. Before: 1521 passed.
   - test-debug-b: 807 passed, 0 failed, 28 ignored.
   - The audit's four-crate command: 589 passed, 0 failed, 8 ignored. The audit got 588 passed and
     1 failed.
2. **Builds.** Both pass:
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, and the same with
     `--all-features`;
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`.
3. **Console digests.** `gain_pan_profile` `digests` output is byte-identical on base and change,
   apart from the `finished in` timing line. `d70956bf..HEAD` changes only one Rust file,
   `hosts/host-web/src/tests.rs`.
4. **Shipped artifact.** `scripts/build-web-audioworklet.sh` in pinned mode passes: the build equals
   the pin `476e58ad…`. There is no repin.
5. **Routing.** `check-ci-path-routing.py` and `test-ci-path-routing.py` pass. The workflow parses
   as YAML.
6. **Nothing deleted.** The only test edit is the one step 1 justifies.

**Also passing:** `cargo fmt --check`, and every hermetic lint-job policy pair: workspace, session,
env-vocabulary, bench, host-core, protocol-control, realtime, realtime-audit-leak,
artifact-evidence-leak, lane, rack, builtins, graph, effect-runtime, conformance-boundaries,
parametric-eq render contract, and the release-shape self-test.

### CI wall time

Measured locally on 32 cores, one run each. `cargo clean --workspace` ran first, keeping
dependencies as rust-cache does.

| step | before | after | change |
|---|---|---|---|
| test-debug-a | 247.3 s | 249.6 s | +2.3 s, about 1% (compile 10.1 s to 11.7 s; the new tests' binaries +0.05 s) |
| test-debug-b | 227.4 s | 227.7 s | +0.3 s (noise) |
| lint job | | | about +2 s: the guard 0.1 s, its mutations 1.8 s, plus two path-routing mutation cases |

On the 4-vCPU runner, expect roughly +10 s on test-debug-a from compiling the extra test-support
code; the added tests themselves take under 0.1 s. No external crate's features change, so the
rust-cache dependency caches stay valid.

### Not done

- No push, PR or GitHub edit.
- The operator-script half (Amendment 4) belongs to 00b.

## Sol attempt 1 verdict: PASS

Reviewer: Sol (Claude Opus 5.5), on `86b96383` in the `engine-1021` worktree, with
`CARGO_INCREMENTAL=0`. Nothing is pushed.

**1. The counts are #916's intended dispatch.** I instrumented the fader-matrix witness in scratch
`git archive` extracts, with every file touched so that no stale artifact from a shared target
could be reused, and ran the test at each commit.

| commit | pair calls per render (members) | result |
|---|---|---|
| `86be6792` (parent) | one call, the one-lane track-8 tail | pass |
| `608f0379` (#916) | an eight-lane `FaderMatrixBankProcessor` call over tracks 0-7, then the one-lane tail | `tests.rs:3220` fails, 2 against 1 |
| `9c762d7c` | the same as `608f0379` | fails |

- The bank call is fused, drains nothing, and outputs 0.5. The tail is fused, drains one fader
  record and one matrix record, and outputs 0.1252968.
- That gives 2 calls, 9 members, 2 fused, 0 fallback, and 1 and 1 records drained: exactly the
  edited assertions.
- The first failing commit is `608f0379`. #916 made the Output dedicated storage, so the cohort
  that contains track 0 now fuses.

**2. The output witness discriminates.** Each of these mutations fails at the reworded assertion
(`tests.rs:3236`) with `0.5`:

- both records staged to track 0;
- both records staged to track 7, inside the bank;
- in the product, host-web admission changed to `(track + 1) % track_count`;
- in the product, host-web admission changed to `track - 1`.

A record fanned out to two tracks would also raise `*_records_drained`, which the test pins at 1.

**3. Guard coverage.** It is complete for every package that declares `test-support`, and it fails
closed on parser drift.

- It goes red, as tested, for: a new declaring package, `--list`, `--ignored`, `--no-run`, name
  filters, and removing a feature.
- A covering step in a job outside the verdict is refused by `check-ci-path-routing.py`. A job
  with `if: false` fails the verdict table.
- Every other `cfg(feature)` that gates tests is on today:
  - `lane` and `realtime-audit` are named on the command line;
  - `c-abi` comes from effect-package's own dev-dependency;
  - `control-provider` comes from capi, unified under `--workspace`: 20 + 10 tests run in
    test-debug-a.

  Non-blocking follow-ups:

  - (a) The guard is bound to the name `test-support`. A renamed test feature that is also dropped
    from CI passes.
  - (b) Failure masking passes both this guard and the routing checker: a step or job with
    `continue-on-error: true`, `|| true` or `; exit 0` after `cargo test`, or a shell `if false`.
    This is a gap for every job, not one #1021 introduced.

**4. Workflow.** The diff adds one lint step, the features and comments, and nothing else. The
router, the leaf `route` gating, the job names and the verdict table are unchanged.
`check-ci-path-routing.py` and `test-ci-path-routing.py` pass. Both new scripts pass (20 cases,
1.8 s). The new paths route to `full`.

**5. Runs.**

- test-debug-a: 1526 passed, 0 failed, 10 ignored, 1534 listed. All five new tests ran.
- test-debug-b: 807 passed, 0 failed, 28 ignored.
- The builtins peak-merge block is red with a count + 1 when its feature is on, and green when it
  is off.
- `cargo fmt --check` and the host-web `--all-features` clippy are clean.
- `build-web-audioworklet.sh` in pinned mode produces `476e58ad…`, equal to the pin.
- Not re-measured: CI wall time.
