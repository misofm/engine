# C ABI: a control call during a plan-swapping render wedges plan replacement for good

## Problem (pre-existing, found by the #1020 verification)

`RealtimePlanOwner::enter_block` swaps plans and commits the retired plan to the retirement queue at the *start* of a render call (`crates/engine/src/realtime/plan_exchange.rs:361-424`), but capi publishes `active_epoch` only after the render call returns (`crates/capi/src/runtime/plan.rs:215-216`). A control call in that window reaches `synchronize_plan_epochs` (`crates/capi/src/runtime/control.rs:619-661`), sees `active_epoch == providers.epoch`, does not promote, reclaims the retired plan, finds no retired provider for its epoch and returns `Internal`; it also skips removing that epoch's report row because it compares against the stale atomic. On the next call the stale provider is pushed into `retired_providers` and never leaves, the report table stays full, and every later structural command returns `Backpressure`.

Reproduced on the unmodified base by a deterministic split-render test (`Err(Internal)`, then `retired [0]` forever, then permanent `Backpressure`) and by a two-thread test racing edits against a structural swap (14 of 20 runs). Every control entry point synchronizes (submit, seek, command, event), so a mobile app that feeds PCM from a decode thread while the audio callback renders can hit it on the first structural edit during playback. Evidence: `docs/handoffs/live-control-2026-09-28/VERIFY.md`, finding F1. This is the mobile playback surface (`docs/rulings/engine-footprint-2026-09-28.md`), so it is a product bug, not a cleanup item.

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
