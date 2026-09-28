# Run the `test-support` tests CI never runs, and fix the one that fails

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
     `scripts/build-web-audioworklet.sh --module-only`.
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
