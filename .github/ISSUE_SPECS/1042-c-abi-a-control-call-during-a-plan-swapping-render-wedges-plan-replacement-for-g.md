# C ABI: a control call during a plan-swapping render wedges plan replacement for good

## Problem (pre-existing, found by the #1020 verification)

`RealtimePlanOwner::enter_block` swaps plans and commits the retired plan to the retirement queue at the *start* of a render call (`crates/engine/src/realtime/plan_exchange.rs:361-424`), but capi publishes `active_epoch` only after the render call returns (`crates/capi/src/runtime/plan.rs:215-216`). A control call in that window reaches `synchronize_plan_epochs` (`crates/capi/src/runtime/control.rs:619-661`), sees `active_epoch == providers.epoch`, does not promote, reclaims the retired plan, finds no retired provider for its epoch and returns `Internal`; it also skips removing that epoch's report row because it compares against the stale atomic. On the next call the stale provider is pushed into `retired_providers` and never leaves, the report table stays full, and every later structural command returns `Backpressure`.

Reproduced on the unmodified base by a deterministic split-render test (`Err(Internal)`, then `retired [0]` forever, then permanent `Backpressure`) and by a two-thread test racing edits against a structural swap (14 of 20 runs). A second thread is not required: `enter_block` swaps before `render_contiguous` checks the block's time and shape, so a *rejected* render call (for example a wrong `absolute_sample`) swaps plans and never publishes the atomic, and the next control call on the same thread fails the same way (Sol, attempt 1 verdict). Every control entry point synchronizes (submit, seek, command, event), so a mobile app that feeds PCM from a decode thread while the audio callback renders can hit it on the first structural edit during playback. Evidence: `docs/handoffs/live-control-2026-09-28/VERIFY.md`, finding F1. This is the mobile playback surface (`docs/rulings/engine-footprint-2026-09-28.md`), so it is a product bug, not a cleanup item.

## Smallest closable slice

In `synchronize_plan_epochs`: a reclaimed epoch equal to `providers.epoch` proves that the one pending candidate is now active, so promote it there; then keep only the report rows that the lagging atomic, the current epoch or a pending epoch can still read. (The verifier's scratch fix is +25/-14 lines.) Commit the deterministic split-render test and the two-thread swap test with it.

## Objective gates

1. The deterministic split-render test is red on the base and green with the fix.
2. The two-thread swap race passes 20 of 20 runs (and a larger count in release) with the fix; it fails on the base.
3. No ack precedes a drop: the failing call acks nothing before or after the fix; every existing capi test, the `resource_lifecycle` oracles and `audit capi` (zero allocations, locks and syscalls on render) pass.
4. The browser is unaffected: host-web does not use plan replacement; the shipped AudioWorklet artifact bytes are unchanged.
5. Lands before the live-control slice (L1), which the #1020 findings describe.

## Attempt 1 evidence

Terra, 2026-09-28. Branch `codex/1042-capi-plan-swap-wedge`, based on `codex/batch-slim-1` at `ed0556a9`. Rust 1.97.1, x86-64-v3 Linux host. Scratch and `target/` were outside the repository and were deleted afterwards.

### Change

The fix is `SessionState::synchronize_plan_epochs` in `crates/capi/src/runtime/control.rs`: +56/-31 lines, including doc comments and a small `promote_pending_provider` helper.

- **Promotion by the atomic.** The atomic promotes only when it is ahead of the providers (`>`, not `!=`). After a promotion inside the window, the atomic lags the providers until the render call returns.
- **Promotion by reclaim.** If a reclaimed plan's epoch equals `providers.epoch`, the one pending candidate is now active, so it is promoted there. The old provider then leaves `retired_providers` on the same reclaim.
- **Report rows.** Every synchronization keeps exactly three kinds of row: the atomic's (read under the report lock), the current provider's and any pending candidate's. Before, a row was removed only on reclaim, and only if it differed from the stale atomic.
- **Room check first.** Promotion checks that `retired_providers` has room before it moves anything. The base removed the pending provider first and dropped the previous one on that path. The path is unreachable, but it no longer loses state.

Nothing else changed: no render path, exported symbol, header, protocol or host-web code.

### Tests (both committed with the fix)

**Deterministic split-render test.** `runtime::tests::control_calls_inside_a_plan_swapping_render_call_keep_replacement_live` runs four rounds. Each round runs `RealtimePlanOwner::render_contiguous`, which swaps plans and retires the old one, and publishes `active_epoch` only afterwards. Between the two halves it makes these control calls:
- an immediate command;
- a lossy dequeue;
- a structural command, which must get transient `Backpressure` with the revision unchanged;
- both resource queries.

After publication, the same structural request must be admitted.

| tree | outcome |
|---|---|
| base | red. Observed `("Internal", "ok", "Backpressure", 43, "Backpressure", 1, [0], [], [0, 1])`; expected `("ok", "ok", "Backpressure", 43, "ok", 1, [], [2], [1, 2])`. That is `Err(Internal)`, then `retired [0]` for good, a full report table and permanent `Backpressure`. |
| fix | green |

**Two-thread swap race.** `resource_lifecycle::control_calls_racing_plan_swapping_renders_never_wedge_replacement` uses the exported C ABI only.
- The render thread renders back to back and counts its own allocations.
- The control thread commits 24 structural swaps (`SetSessionId`).
- After each swap, the control thread spins on reliable dequeue, lossy dequeue and `miso_engine_v1_plan_resources` until two more blocks have rendered.
- Any code other than OK fails the test, and so does `BACKPRESSURE` more than 256 blocks after an admission.
- The test also asserts that some control call overlapped a render call, and that render made 0 allocations and 0 frees.

| runs | base | fix |
|---|---|---|
| dev, 20 runs | 0 of 20 pass; all 20 fail on `RESULT_INTERNAL` (255) from a dequeue inside the window, 16 of them on the first swap | 20 of 20 pass |
| release, 20 runs | 0 of 20 pass; the same failure | 20 of 20 pass (part of the 1,000) |
| release, 1,000 runs | not run | 1,000 of 1,000 pass |
| whole `resource_lifecycle` binary under the parallel harness, dev, 20 runs | not run | 20 of 20 pass |

**How often the race hits the window.** A temporary `eprintln!` in the reclaim-promotion branch was used for this measurement and was not committed. That branch ran on 23 of the 23 raced swaps in each of 20 dev runs, and on 21-23 of 23 in each of 20 release runs. The 24th swap is not raced, because it ends the loop.

**Mutations.** Each mutation was run against both tests.

| mutation | split-render test | race test |
|---|---|---|
| `>` back to `!=` | red | red (255) |
| drop the lagging-atomic row | red | red (`plan_resources` 255) |
| remove the reclaim promotion | red | red (255) |

### Gates

1. **Split-render test.** Red on the base, green with the fix (above).
2. **Swap race.** Fails 20 of 20 on the base. With the fix it passes 20 of 20 in dev and 1,000 of 1,000 in release.
3. **No ack precedes a drop.** The window's structural command returns `Backpressure` and leaves the revision unchanged. The same request id is then admitted, which shows there was no replay entry.
   - `audit capi` (release) wrote this record: `calls 100000`, `render_errors 0`, `allocations 0`, `deallocations 0`, `locks 0`, `syscalls 0`, `panic_unwinds 0`, `total_violations 0`, `pcm_digest ff6cdcb96cdcdad5`.
   - `cargo test -p capi` passes in dev and in release: the lib has 33 tests and `resource_lifecycle` has 5, and these include the lifecycle oracles.
   - `scripts/check-capi-abi.sh` passes against the fixed `libcapi.so` and `libcapi.a` (shared and static linkage).
4. **The browser is unaffected.**
   - `cargo tree -p host-web -i capi` finds no path from host-web to capi.
   - `scripts/build-web-audioworklet.sh --module-only` builds byte-identical modules with and without the fix: `3f744b03e22ed0ecb45ab8358eea73fb3bf4a834a17e1ec4ac348aaca64b25da`.
   - **Batch-boundary note.** That module does not match the checked-in pin `476e58ad…`, and the batch base `ed0556a9` does not match it either. `main` (`a9414c0c`), rebuilt in a temporary worktree, does match the pin. Earlier batch commits (#1023, #1041) moved it. The batch repins at its boundary; this issue does not.
5. **Ordering.** This lands on the batch branch before any L1 work.

### Other checks

- **Target builds.**
  - x86-64: `cargo build --release -p capi` builds and links the rlib, staticlib and cdylib.
  - `aarch64-apple-ios` and `aarch64-linux-android`: `cargo check --release -p capi` and `cargo clippy -p capi --all-targets -- -D warnings` pass.
  - A release build with a no-op linker compiled the whole dependency chain for both AArch64 targets and produced a real `libcapi.a`. Linking the cdylib needs Xcode or the NDK, and neither is installed here.
- **Lint and docs.** `cargo clippy --workspace --all-targets -- -D warnings` passes, with and without `--all-features`. `cargo fmt --all -- --check` and `RUSTDOCFLAGS='-D warnings' cargo doc -p capi --no-deps` pass.
- **Policy scripts.** All of these pass:
  - `check-workspace-policy`, `test-workspace-policy`, `check-session-policy`, `check-env-vocabulary`, `check-bench-policy`, `check-host-core-policy`, `check-protocol-control-policy`;
  - `check-realtime-policy`, `test-realtime-policy`, `check-realtime-audit-leak`, `check-artifact-evidence-leak`, `check-lane-policy`, `check-unfused-seal`;
  - `check-rack-policy`, `check-builtins-policy`, `check-graph-policy`, `check-effect-runtime-policy`, `check-conformance-boundaries`, `check-native-pcm-runner`;
  - `check-test-support-ci.py`, `check-step-vocabulary.py`, `check-ci-path-routing.py`.

### Finding for review (pre-existing, not fixed here)

`active_resources` in `crates/capi/src/runtime/plan.rs` backs the any-thread query `miso_engine_v1_plan_resources`. It loads `active_epoch` *before* it takes the report lock. This can fail in three steps:
1. The query loads the old epoch.
2. Render publishes the new epoch, and a control call removes the old row.
3. The query takes the lock and finds no row, so its `expect` panics.

Release builds use `panic = "abort"`, so a mobile app that polls resources while it edits during playback would abort. The window is a few instructions wide, and the base has the same exposure, because its reclaim also removed the old row once the atomic had moved on.

The fix is to load the atomic while holding the lock. This attempt already does that on the control side, so the reader side needs only a two-line reorder. It is outside this slice and needs Sol's ruling: amend this issue, or open a bounded successor.

## Sol attempt 1 verdict: FAIL

Sol, 2026-09-28, reviewing `95ddc0eb`. Scratch work (an isolated `git archive` copy of the commit, a scratch `target/` and a Python interleaving model) stayed outside the repository and was deleted afterwards.

### What holds

- **Replicated.**
  - The split-render test is red on the base, with the same tuple as reported, and green with the fix.
  - The race test fails 5 of 5 runs on the base (255 from a dequeue after 1 swap).
  - With the fix, it passes 20 of 20 in dev and 300 of 300 in release.
  - Pinned to one core (`taskset`), it passes 200 of 200 in release and 40 of 40 in dev.
- **The promotion rule is correct.** An exhaustive model at the code's step granularity was used:
  - Synchronization is modeled in three steps: load and promote; one step per `try_reclaim`; retain under the lock.
  - Render is modeled as `enter_block`, then publish or reject.
  - The model uses capi's capacities: rows 2, pending 1, retired 1, publication 1, retirement 1.
  - Scripts ran up to 14 control operations, 10 render calls (4 of them rejected) and 3 reads: 39,252 states.
  - The fix never returns `Internal` and never wedges after quiescing, and every acked epoch is activated.
  - The model catches each of the three reported mutations.
- **Cases.**
  - *Before the swap:* the pending candidate gives `Backpressure`.
  - *During the swap:* the reclaim promotes.
  - *After the swap:* the atomic promotes.
  - *Back to back:* no candidate can commit while the atomic lags, because both rows are in use. So the lag is at most one epoch.
  - *Superseded candidate:* unreachable. `command` refuses while a candidate is pending, and the publication capacity is 1.
  - *Empty retirement queue:* only the atomic path runs.
  - *Full retirement queue:* unreachable for reserved candidates. The credit is taken at reservation, so the command gets `Backpressure` and acks nothing. `DeferredRetirementFull` applies only to the legacy `publish`.
  - *No ack precedes a drop.* An ack comes only after the publication slot, the retirement credit and the room checks are all secured.
- **F1 needs no second thread.** `enter_block` swaps before `render_contiguous` checks time and shape. So a *rejected* render call, such as one with a wrong `absolute_sample`, swaps plans and never publishes the atomic.
  - I tested this through the C ABI on one thread only.
  - Base: the next dequeue returns `INTERNAL`, then every structural command returns `BACKPRESSURE`.
  - Fix: the next control calls succeed, and the same request is admitted after the next good render, in 4 of 4 rounds.
- **Mutations.** I ran seven mutations (`!=`, `>=`, no lagging row, no reclaim promotion, no current row, no pending row, no `retain`). Each one turns both committed tests red: the split-render test fails and the race test passes 0 of 5. `>=` also hangs an existing lib test.
- **Realtime.**
  - The diff has 0 lines in `crates/engine`, `plan.rs` or `ffi.rs`, and render takes no lock.
  - Release `audit capi` reproduces the implementer's record: 100,000 calls, 0 allocations, deallocations, locks, syscalls and violations, `pcm_digest ff6cdcb96cdcdad5`.
  - The audit driver never swaps plans. Swap-path evidence is the race test's render-thread counter (0 allocations, 0 frees).
- **Other checks.** The capi dev and release suites pass (33 + 5). `clippy -p capi --all-targets -D warnings` and `fmt` pass. host-web has no path to capi.

### Why it fails

1. **The `plan_resources` reader race belongs in this slice, and attempt 2 must fix it.**
   - *Test.* A third thread polled `miso_engine_v1_plan_resources` during the committed race, extended to 200 swaps, in release.
   - *With the fix:* 19 of 100 runs panicked at `plan.rs:43`. The `MutexGuard` temporary is still live when that panic fires, so it poisons `reports`:
     - every later query panics at `plan.rs:40`;
     - every synchronization returns `Internal`, which is the permanent wedge this issue exists to close;
     - under `panic = "abort"` the process aborts.
   - *On the base:* 0 of 100 runs hit it, but only because the control thread wedges at the first swap in 100 of 100 runs. The fix is what exposes the race.
   - *With the two-line reorder* (take the lock, then load the atomic): 300 of 300 runs pass, and the model finds no violation.
   - *Why it belongs here:* without the reorder, the new `retain` comment ("the rows a reader can still ask for") is false for the any-thread reader.
2. **The peak check misclassifies a structural command inside the window.**
   - *Mechanism.* `command` checks the candidate against the lagging atomic's row, which is the plan being reclaimed, not the plan that is rendering.
   - *Test.* I set `maximum_effect_state_bytes = 8,424 + 7,488`, so both pairs of plans that really coexist fit. The test removes eq0's EQ, swaps, and puts the EQ back inside the window.
   - *Result.* The window call returns `RESULT_COMPILE_REJECTED` (`effect.resource.limit`). After publication, the same request is admitted.
   - *Why the committed test misses it.* It checks for transient `Backpressure` in the window only under loose limits.

### Attempt 2 must

1. Reorder `active_resources` in `crates/capi/src/runtime/plan.rs`. Add a reader-thread race that fails on `95ddc0eb` at a stated rate and passes 1,000 or more release runs with the fix.
2. In the structural arm, return `Backpressure` before `prepare_runtime` whenever the atomic still lags `providers.epoch` after synchronization. The table is full in that state anyway, and this also skips a wasted compile. Commit the tight-limit window test, which fails on `95ddc0eb`.
3. Recommended, not blocking: commit the single-thread rejected-render C-ABI test, and correct the Problem text, since F1 does not need a second thread.

Note: `promote_pending_provider` could fail inside the reclaim loop after the plan was already popped. That is unreachable at capi's capacities, and the model never hits it, so leave it.

## Attempt 2 evidence

Terra, 2026-09-28. First the batch head (`b8bea8e1`) was merged into the branch (`70502112`). The attempt-1 fix (`95ddc0eb`) is unchanged; this attempt adds to it. Rust 1.97.1, x86-64-v3 Linux host. Scratch and `target/` were outside the repository and were deleted afterwards.

### Changes

The code changes are in two files: `crates/capi/src/runtime/plan.rs` (+10/-3) and `crates/capi/src/runtime/control.rs` (+21/-4).

1. **Reader reorder (verdict item 1).** `active_resources` backs `miso_engine_v1_plan_resources`. It now takes the report lock before it loads `active_epoch`.
   - The control thread removes a row only under that lock, and only once the atomic has moved past it.
   - The atomic never moves back, so an epoch read under the lock always has its row.
2. **Window peak check (verdict item 2).** While `active_epoch < providers.epoch`, the structural arm returns `Backpressure` right after the `plan_alive` check. That is before `prepare_runtime`, so the call compiles nothing and acks nothing.
3. **The same misclassification before the swap (an extension; please rule on it).** The peak check reads the row from the new `replacement_base_report`, which replaces `active_resource_report`. That row belongs to the newest plan: the pending candidate while one waits, else the current provider.
   - **The problem.** The old code paired a candidate with the atomic's row. While a candidate is pending, that row belongs to a plan the new candidate can never coexist with.
   - **Reproduced.** The problem predates this issue, on `ed0556a9` and `95ddc0eb` alike. Under Sol's tight limit, the same valid edit sent while the eight-EQ plan was pending returned `CompileRejected(effect.resource.limit)`.
   - **Why this form of the fix.** An alternative was to move the pending check ahead of the compile. I did not, because that would change the existing pinned owner counters (the "publication-full canceled candidate" path in `exported_c_replay_revision_event_and_publication_pressure_statuses_are_exact`).
   - **What is unchanged.** With nothing pending and the atomic caught up, the newest row is the rendering plan's row, as before. Admission itself is unchanged.
4. **Problem text.** The Problem section now notes that a rejected render call opens the window on one thread.

### Tests

**Tight-limit window test.** `runtime::tests::a_valid_edit_inside_the_swap_window_is_backpressured_not_compile_rejected` sets `maximum_effect_state_bytes` to the nine-EQ bytes plus the eight-EQ bytes, both measured. It removes eq0's EQ, then puts it back twice:
- once while the eight-EQ plan is pending;
- once inside the split-render window, where the call must also compile no candidate (the owner counter must not move).

After publication, the same request must be admitted at revision 44.

| tree | result |
|---|---|
| `95ddc0eb` | red: the window call returns `CompileRejected(effect.resource.limit)` |
| `ed0556a9` | red: the window call returns `Internal` |
| attempt 2 | green |

**Single-thread rejected-render test.** `runtime::tests::a_rejected_render_call_that_swaps_plans_leaves_the_c_session_live` runs 4 rounds through the C wrappers. Each round:
1. renders at a wrong sample, which returns `RESULT_RENDER_REJECTED` and swaps plans;
2. makes a lossy dequeue;
3. sends a structural command, which must get `BACKPRESSURE` with the canary untouched and the revision unchanged;
4. renders the good block;
5. sends the same request again, which must be admitted.

| tree | result |
|---|---|
| `ed0556a9` | red: `(0, 8, 255, 6, true, 43, 0, 1, 0, 6)`. The dequeue returns `INTERNAL`, and the retry stays at `BACKPRESSURE`. |
| `95ddc0eb` | green |
| attempt 2 | green |

**Reader race.** `resource_lifecycle::resource_queries_racing_plan_swaps_always_find_the_published_row` runs 200 swaps with two reader threads polling `miso_engine_v1_plan_resources`. It shares one helper, `race_plan_swaps`, with the attempt-1 race, which becomes `race_plan_swaps(24, 0)` and is otherwise unchanged.

| tree | runs passed | how it fails |
|---|---|---|
| `95ddc0eb`, release | 0 of 100 | Every run panics at `plan.rs:43` ("active plan epoch retains its resource report"). The lock is then poisoned, and 88 runs also panic at `plan.rs:40`. |
| `95ddc0eb`, dev | 0 of 20 | same |
| `ed0556a9`, release and dev | 0 of 20 each | The wedge comes first: `INTERNAL` from a dequeue. |
| attempt 2 with only the reorder reverted, release | 3 of 20 (17 fail) | panic at `plan.rs:43` |
| attempt 2, dev | 20 of 20 | |
| attempt 2, release | 1,000 of 1,000 (479 s) | |

**Swap race (attempt 1).**

| tree | runs passed |
|---|---|
| attempt 2, dev | 20 of 20 |
| attempt 2, release | 1,000 of 1,000 (54 s) |
| `ed0556a9`, dev and release | 0 of 20 each |

**Whole `resource_lifecycle` binary, parallel harness, dev.** 20 of 20 runs pass.

**Split-render test (attempt 1).** It stays green. It now checks the any-thread query in the window against the lagging epoch's row in the snapshot, because the control-side `active_resource_report` it used to call is gone.

**Mutations of the attempt-2 changes.** Each one turns the named test red.

| mutation | red test |
|---|---|
| remove the lag check | tight-limit test (the window call compiles a candidate) |
| base row ignores the pending candidate | tight-limit test (pending arm: `CompileRejected`) |
| base row is the atomic's | tight-limit test |
| revert only the reader reorder | reader race (17 of 20 runs fail) |

### Gates

1. **Tests.** Every new test is red on the tree it targets and green with the fix (tables above).
2. **Races.** The swap race passes 1,000 of 1,000 release runs, and the reader race passes 1,000 of 1,000.
3. **No ack precedes a drop.**
   - The window refusal happens before the compile, the replay entry and the ack.
   - The pending path is unchanged except for which row its peak check reads.
   - `audit capi` (release) gives the same record as attempt 1: `calls 100000`, `render_errors 0`, `allocations 0`, `deallocations 0`, `locks 0`, `syscalls 0`, `panic_unwinds 0`, `total_violations 0`, `pcm_digest ff6cdcb96cdcdad5`.
   - Render takes no new lock. The reorder touches only the any-thread reader.
   - `cargo test -p capi` passes in dev and in release (lib 35, `resource_lifecycle` 6).
   - `scripts/check-capi-abi.sh` passes for shared and static linkage.
4. **The browser is unaffected.**
   - `cargo tree -p host-web -i capi` finds no path from host-web to capi.
   - The `--module-only` AudioWorklet module is byte-identical with and without this attempt: `3f744b03…`.
   - The pin mismatch (`476e58ad…`) is the batch's own, as recorded in attempt 1.
5. **Other checks.**
   - `cargo clippy --workspace --all-targets -- -D warnings` passes, with and without `--all-features`.
   - `fmt` passes, and so does `cargo doc -p capi` with `-D warnings`.
   - For `aarch64-apple-ios` and `aarch64-linux-android`: `cargo check --release`, `cargo clippy -p capi --all-targets -D warnings`, and a release codegen to `libcapi.a` with a no-op linker all pass. Linking the cdylib needs Xcode or the NDK.
   - All 22 policy scripts listed in attempt 1 pass.
