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
