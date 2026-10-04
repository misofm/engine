# #1270 attempt 1 verdict: PASS

PASS with minors: no BLOCKER and no MAJOR findings.

- **Commit reviewed:** `22c9bd5a1` (parent `41517fc35`), exported with `git archive` to
  `/tmp/claude-1002/v1270/attempt1/`. Logs are in `vlogs/` there.
- **Paths changed:** `crates/engine/src/realtime/{mod,plan,plan_exchange}.rs` and the slice spec.
  All are authorized, and `lib.rs` is unchanged.
- **Commit hygiene:** `git diff --check` is clean. The message ends with the Co-Authored-By line.
  The commit adds no `unsafe`.

## Spec conformance

- **D1:** `as_any_mut` and `adopt_predecessor` exist with the frozen defaults.
  `CarryOutcome` (`Copy`, three variants) is exported from `engine::realtime`.
- **D2:** `enter_block` runs `carry_from` only on the `Applied` path. It runs after
  `adopt_absolute_sample` and before `placeholder.commit`. The `None` path and both
  `DeferredRetirementFull` returns report `NotRequested`. `Drop` never calls the hook.
- **D3:** `adopt_predecessor_plan` is public. It adopts the clock, then runs the hook inside
  `in_render_scope`. The scope nests, because it tracks depth (`audit.rs`).
- **D4:** `RealtimeRenderReport.carry` is added, and `carried_count` and `carry_mismatch_count`
  are saturating `const fn`s like `deferred_count`.
- **D5:** all new engine code sits inside existing `REALTIME_POLICY` regions. The default hook is
  a constant return.
- **Width coverage:** not applicable. The slice touches no bank code.

## Gates re-run in the export

All gates are green.

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | ok |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | ok |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | ok |
| `cargo test --locked -p engine` | 36 lib + 4 + 1 ok |
| `cargo test --locked -p engine --features realtime-audit` | 38 + 4 + 1 ok |
| loom `spsc_loom` (release, `--cfg loom`) | ok |
| `cargo build --locked --release -p audit -p capi` | ok |
| `trace-realtime-audit.sh target/release/audit 1000000` | ok (see below) |
| `trace-builtins-graph-audit.sh` | PASS (see below) |
| `./target/release/audit capi` | 0 allocations, 0 syscalls, 0 violations |
| `check-workspace-policy.sh` / `test-workspace-policy.sh` | ok / ok |
| `check-realtime-policy.sh` / `test-realtime-policy.sh` | ok, 54 regions in 15 files / ok |
| `check-capi-abi.sh` | ok |
| `check-cross-targets.sh` | PASS (see below) |
| Worklet chain (see below) | all ok |
| `check-web-audioworklet-v8-spill.py` (self-test and the named twin) | ok |
| Dependents' tests: `cargo test --locked -p graph -p host-core -p capi -p host-web -p audit` | 571 passed, 0 failed |

- **`trace-realtime-audit.sh`:** I ran it under `timeout 600s` because the box was loaded. Result:
  1,000,000 blocks, `swaps_accepted` 2, `swaps_deferred` 1, allocations 0, syscalls 0. Its plans
  have no executor, so `carry_from` returns early there.
- **`trace-builtins-graph-audit.sh`:** `swaps_applied == 1` with graph executors, so the default
  hook is dispatched, with zero allocations and locks.
- **`check-cross-targets.sh`:** PASS. The ten #1018 `memset_pattern16` expected-failure rows are
  byte-identical to a run on a parent export (`41517fc35`, also PASS). Nothing is new.
- **Worklet chain:** `build-web-audioworklet.sh --named-twin`,
  `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts` and `test-web-audioworklet.sh` all pass.

### ARTIFACT CHANGED

The claim holds.

- **Candidate:** the shipped module is
  `a28285ebc38f4084174918cc735605a8d28aceaeb8c6620ff4393a4ae910d144`. It reproduces exactly in
  both `--named-twin` and `--module-only` builds.
- **Parent:** the shipped module is
  `30d075d3ce6382f21235675996184c675753acf6d451e11d7d676a3d50aaeff4`. That equals the
  `audioworklet-sha256` commit status CI recorded on `24029badb`, and `41517fc35` adds only
  `.github` specs over that commit.
- **Pin:** not re-pinned (`6c952a2c...`, untouched). That is correct per `docs/RELEASE.md`
  "Between releases".

### Allocation-counter deferral

The claim holds. Gate 4 in the slice spec says explicitly that the allocation-counter gate for a
real carry is slice 3's (host-core, `bench_support`) and that this slice adds no `unsafe`
allocator. The two existing swap audits above stay at zero.

## Test value (one sentence each; mutations run by me in the export, then reverted and verified pristine by `cmp` against the commit)

- **`successor_continues_the_predecessor_state_gap_free`.** It turns red when:
  - `enter_block` never runs the hook (M1);
  - the hook runs after the successor's first render (M5, which I wrote: stash the predecessor,
    render, then carry and retire; block 5 restarts at 0.5 and the 10-block equality fails);
  - the report drops the carry outcome (M8).

  No earlier test checks executor state across a swap. Its carry-off half proves the oscillator
  really depends on the hand-over.
- **`hand_over_runs_only_on_the_applied_block`.** It turns red when the hook also runs on a
  deferred block (M2), or when it takes state when the candidate is first popped instead of at
  the applied block (M2b).
- **`dropping_the_owner_never_hands_over_to_an_unapplied_candidate`.** It turns red when
  `Drop for RealtimePlanOwner` hands over to a queued candidate (M3).
- **`synchronous_hand_over_continues_state_and_clock`.** It turns red when
  `adopt_predecessor_plan` skips the clock adoption (M4: the capture is stamped 0), or skips the
  executor hook (M7).
- **`a_mismatched_predecessor_is_reported_not_carried`.** It turns red when a mismatch is counted
  as carried (M6) or the outcome is not plumbed (M1, M8).

Two mutations survive: M3b (NIT-3) and M9 (MINOR-1).

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

1. **No test pins D3's render-scope arming.**
   - **Where:** `crates/engine/src/realtime/plan.rs:893-896`.
   - **Evidence:** M9 replaces `super::audit::in_render_scope(|| self.carry_from(predecessor))`
     with a bare `self.carry_from(predecessor)`. It stays green with and without
     `--features realtime-audit`. So the browser hand-over (#1290) could silently leave the audit.
   - **Fix:** under `#[cfg(feature = "realtime-audit")]`, have the test `Oscillator` record
     `crate::realtime::audit::is_render_scope_active()` inside `adopt_predecessor`. Assert it is
     true after `adopt_predecessor_plan` (`mod.rs:948`) and after an exchange swap.
2. **The attempt record's base digest is wrong.**
   - **Where:** slice spec, line 162.
   - **Evidence:** the record says the shipped module at `41517fc35` was `5d21f73e...2675`. The
     reproducible value is `30d075d3...aeff4`, which matches CI's record for `24029badb`. The
     conclusion (ARTIFACT CHANGED, no re-pin) is unaffected.
   - **Fix:** correct the number.
3. **The synchronous form has no envelope guard.**
   - **Where:** `plan.rs:893`.
   - **Evidence:** the exchange path gets umbrella P1.2 ("same envelope") by construction,
     because `publish` and `reserve_replacement` refuse a different envelope. But
     `adopt_predecessor_plan` adopts the clock and hands state across any rate, quantum or channel
     count. #1290 is the first caller.
   - **Fix:** return `CarryOutcome::PredecessorMismatch` without touching the clock or calling
     the hook when `self.envelope() != predecessor.envelope()`. Or state the precondition in the
     rustdoc and make #1290's brief own the check.

### NIT

1. **Stale rustdoc.** `plan.rs:614`, the `adopt_absolute_sample` rustdoc, says "Only
   `RealtimePlanOwner` calls this". `adopt_predecessor_plan` now calls it too.
2. **"Not expressible" is wrong.** The record (spec line 146) says the "hook after the
   successor's first render" defect cannot be written as a mutation. It can (M5 above), and gate 1
   catches it. Replace the stand-in note with that mutation.
3. **Drop's `pending` branch is uncovered.** The gate 3 test (`mod.rs:934`) drops only a
   candidate still in the publication queue. M3b (hand over to the `pending` candidate in `Drop`)
   stays green. Only a legacy `publish` candidate can be deferred there; a reserved one cannot.
   So the spec's literal gate holds, but a second drop after a deferred `publish` would close the
   branch.
4. **A missing predecessor executor is never counted.** `carry_from` (`plan.rs:899`) reports
   `NotRequested` when the predecessor has no executor. A successor that wanted state is then
   never counted as a mismatch (P11's counter). This is unreachable in production, where every
   plan comes from `graph` with an executor. Keep it in mind for #1271 and #1272.
5. **`RealtimePlanOwner::render`'s carry plumbing is untested.** Only `render_contiguous` is
   exercised with an executor (`plan_exchange.rs:473`). Production hosts use `render_contiguous`
   (`capi/src/runtime/plan.rs:208`, `host-core/src/render_session.rs:114`).
6. **Swap doc omits the hand-over.** `docs/REALTIME_MEMORY.md:96-101` describes the swap sequence
   without the hand-over step. That file is outside this slice's authorized paths, so a later
   slice should add it.
7. **"Two" memset failures is the wrong count.** The record says "the two #1018 expected
   failures". The script reports ten expected-failure crate rows, each at its ceiling and
   identical on the parent. This is wording only.
