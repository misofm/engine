# Spectrum: a one-shot window held for a cancelled arm shields the next arm from a failed block

Found by the implicit-state-machine audit of 2026-11-06 (tracker #2258, item HC-10). Terra attempt
1; five attempts maximum.

## Problem

The one-shot capture's render-local state (`SpectrumOneShotState` in
`crates/host-core/src/spectrum.rs`) is five loose fields: `armed`, `held`, `survived_block_end`,
`latched` and `filled`, beside `first_sample`, `next_sample` and `source_underrun`. Their
combinations encode three phases (idle, filling a window, holding a complete window for its block
end), but nothing ties a phase to the fields that belong to it. The fields are render-local, so
there is no race; the cross-thread one-shot word (`StateWord`, an `AtomicU64` with loom coverage
in `spectrum_handoff_loom`) is not part of this issue.

Verified at `24ea03525`:

- `armed && held` is never reached: `armed` becomes true only on the `ARMED` arm of
  `one_shot_begin`, after `held` was cleared; `held` becomes true only in `one_shot_finish`, which
  clears `armed` in the same step. `filled < SPECTRUM_WINDOW_FRAMES` while armed and `filled ==
  SPECTRUM_WINDOW_FRAMES` while held hold for the same reason. Carry (`pair_carried`, the observer's
  `carry_kind`) moves the whole observer, so it carries the fields unchanged; `settle_unpaired`
  touches only the successor's word.
- Two fields go stale: `survived_block_end` after `held` clears, and `armed` while a continuous
  stream runs (`block_end`'s continuous branch clears only `held`). Neither is observable: the
  first is read only together with `held`, and the second is cleared by the first one-shot
  `one_shot_begin` after the stream, because the word cannot be the latched `(g, CAPTURING)`
  again (a stream starts only from `IDLE`, an arm moves `g` to `g + 1`, and only render writes
  `CAPTURING`).
- **One stale combination is reachable and observable.** `held` (with `survived_block_end`) stays
  set after control cancelled the held window, until the next block end or begin notices that the
  word moved. `one_shot_invalidate` returns early on `held && survived_block_end` without checking
  the word. So: a window of generation `g` is held behind a stale record (a cancelled window that
  render pushed after its cancel drained the queue; the race the existing stale-record tests
  model); control cancels and arms `g + 1`; the next block fails (`invalidate_after_failure`), is
  malformed (unequal planes) or the render fails (`invalidate`). An observer that never held a
  window moves `(g + 1, ARMED)` to `INVALID` and the reader reads `Invalid` once. This observer
  leaves the word `ARMED`, and the reader reads `Pending` and later gets a window. The dead
  window's protection leaks onto an arm of a later generation.

## Contract

- One render-local enum replaces the five fields:
  `OneShotWindow { Idle, Filling(OneShotFill), Held(OneShotHeld) }`. `OneShotFill` carries
  `latched`, `filled`, `first_sample`, `next_sample` and `source_underrun`; `OneShotHeld` carries
  `latched`, `first_sample`, `source_underrun` and `survived_block_end`. A combination outside
  these three cannot be written.
- A held window survives a later failed or malformed block only while the word is still the one it
  latched. Once control moved the word, the window is dead, and the block ends the word's current
  arm exactly as it would on an observer that never held a window.
- The continuous branch of `block_end` ends any one-shot window (`Idle`), filling or held.
- No change to the one-shot word's protocol, its writers or its orderings; `render` stays
  allocation-, lock- and syscall-free.
- Edits only `crates/host-core/src/spectrum.rs` and this spec (not `spectrum_service.rs`).

## Non-goals

`SpectrumAnalysis`'s loose fields (the #2255 follow-up), the continuous state, and the cross-thread
word.

## Gates

- A regression test red on `24ea03525`, green after.
- A one-time no-bit-moved comparison against the base, recorded below, not committed.
- `spectrum_handoff_loom` (the qualification host-core loom leg), since its render thread reads the
  one-shot state, and a loom model of a failed block racing the cancel and re-arm of a surviving
  held window, which reaches the guard's load of the word.
- `cargo test --locked -p host-core --features test-support`; `cargo clippy --locked -p host-core
  --all-targets -- -D warnings`; `cargo fmt --check`; `cargo doc`; `scripts/check-workspace-policy.sh`;
  the realtime policy tool.

## Evidence

### Attempt 1 (Terra, branch `codex/issue-2290-spectrum-one-shot-window`)

Base `24ea03525`. Commit `d0f351b72` (the enum, the fix and the test), then this evidence commit.

**Model.** `SpectrumOneShotState { state, window: OneShotWindow }`, with
`OneShotWindow { Idle, Filling(OneShotFill), Held(OneShotHeld) }`.
- `one_shot_begin` returns `(destination, count)` from the `Filling` it starts or continues. So
  callers no longer read `filled` off the state.
- `one_shot_finish` turns `Filling` into `Held` when the window is full.
- `one_shot_block_end` matches `Held`.
- `one_shot_invalidate` loads the word first. It keeps the window only for
  `Held { survived_block_end: true }` whose `latched` still equals the word; anything else goes to
  `Idle`, and an `ARMED`/`CAPTURING` word moves to `INVALID` by the same one compare-and-swap as
  before.
- The word protocol is untouched: same writers, same compare-and-swaps from the same latched
  values, same orderings. The one difference in atomic operations is in `one_shot_invalidate`'s
  surviving-window path, which now does one `Acquire` load of the word before it returns.
- The test-only accessor `SpectrumOneShotState::window_start` replaces the three tests' direct
  reads of `latched`, `first_sample` and `held`. In the two render-thread tests (the 1000-seed
  cancel race and `spectrum_handoff_loom`'s `handoff`), "a new window began" is now
  `window_start()` returning a latch that differs from the last one. That is the same event as
  before, because both tests read it right after a capture that began a window, while it is
  `Filling` or `Held`.

**Red on base.** Test: `spectrum::tests::a_dead_held_window_does_not_shield_a_new_arm_from_a_failed_block`.
It holds a generation-2 window behind a stale record (a surviving held window), then cancels and
arms generation 3. It then delivers a failed block, a malformed block and a render-failed block,
each to its own pair. Each read is compared with an observer that never held a window (the
oracle, which reads `Invalid`). The committed test, run against base `spectrum.rs` (mutant M0
below):

```
assertion `left == right` failed: each block after the re-arm reads as it does on a fresh observer
  left: [("failed", Err(Pending)), ("malformed", Err(Pending)), ("render-failed", Err(Pending))]
 right: [("failed", Err(Invalid)), ("malformed", Err(Invalid)), ("render-failed", Err(Invalid))]
```

At head it passes. **Test value.** The test turns red if the survival guard ignores whether the
word still holds the held window's latch. In that case a failed, malformed or render-failed block
after a cancel and re-arm leaves the new arm `ARMED`, so the reader sees `Pending` and later a
window, where it should see `Invalid`. No existing test re-arms between a surviving held window
and a failed block.

**Mutations** (each against head, `cargo test -p host-core --features test-support --lib spectrum`):

| Mutant | Change | Result |
|---|---|---|
| M0 | base `spectrum.rs` with the new test | red: the new test |
| M1 | drop `word == held.latched` from the survival guard | red: the new test |
| M2 | drop `held.survived_block_end` from the guard | red: `a_window_completed_by_a_failed_block_is_never_returned` |
| M3 | drop the survival guard | red: `a_held_window_survives_a_later_failed_or_malformed_block` |
| M4 | continuous `block_end` clears only `Held` (base behaviour) | green, as expected: unobservable (see Problem) |
| M5 | the `ARMED` arm does not reset the window before its compare-and-swap | green, as expected: a failed swap means control moved the word, which the next block end or begin already notices |

**No bit moved.** This was a one-time scratch run, not committed, and it was reverted. A scratch
hook in `SpectrumCaptureObserver` folded a digest (FNV-1a) after every `capture`,
`capture_resident`, `invalidate`, `invalidate_after_failure` and `block_end`. The digest covered:
- the one-shot projection (phase, latch, filled, first and next sample, underrun, survived);
- both shared words;
- the window planes' bits while the window is held.

On drop the hook appended the digest, keyed by the test thread's name, to a file. Over the whole
`cargo test --release -p host-core --features test-support` suite:
- Base was run twice. 72 named test threads (4,204 observers) gave the same digests on both runs.
  The one unstable key was the unnamed render threads.
- At head, 70 of those 72 are identical.
- The two that differ are expected: the new test, which is red at base, and the scratch walk below.

Seeded walk: 4,000 seeds × 600 steps, each step one or two control operations and then one graph
block.
- Control operations: arm, cancel, read (each read's window bits and its full `analyze` output are
  folded in), a stale record pushed as render would, start or stop continuous, and the replace
  switch.
- Graph blocks: random length 64-3,000 frames, a contiguous or jumping start sample, NaN or
  infinite samples, underrun and generation-changed flags, unequal planes, and the block's end
  (`block_end`, `invalidate_after_failure` or `invalidate`).
- In total it read 66,886 windows and spent 74,160 steps holding a surviving window.

Results:
- The 2,052 seeds that never reach either changed case are identical at base and head, in their
  external digest and in their render-local digest.
- The 1,632 seeds where a continuous `block_end` meets a `Filling` window have identical external
  digests. Their render-local digest is identical up to that event, and then differs only by the
  intended `Idle`.
- The 316 seeds that reach a dead surviving held window at an invalidating block have identical
  digests up to that point. 8 of them then diverge externally, which is the fixed behaviour. The
  other 308 met the dead window while the word was `IDLE`, so no word moved.

**Gates (head).**
- `cargo test --locked -p host-core` and `cargo test --locked -p host-core --features
  test-support`: all pass.
- The loom leg, as in `qualification.yml` (`--cfg loom`, `+avx2,+fma`,
  `target/ci/loom-host-core`):
  `cargo test --locked --release -p host-core --lib spectrum_handoff_loom`. It runs 3 tests,
  `spectrum_handoff_loom`, `spectrum_mode_switch_loom` and `spectrum_continuous_restart_loom`;
  all pass.
- `cargo clippy --locked -p host-core --all-targets -- -D warnings`, with and without
  `test-support`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p host-core --features
  control-provider`: clean. Without that feature it fails on an existing link in `prepare.rs` to
  the feature-gated `crate::warm`. CI documents the whole workspace, where the feature is on.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --release -p realtime-policy`: `realtime policy: ok (292 marked regions in 46 files)`.

**Verdict: PASS** (Sol's adversarial review of attempt 1). The review independently confirmed:
- the defect is reachable within the contract;
- the guard's `Acquire` load is correct and has no ABA risk;
- its own differential walk diverges exactly at the trigger, on 87 of 4,000 seeds;
- the merged tree is green.

Two items were folded on the same branch (`3cf27c2a6`):
- **Loom model.** `spectrum_dead_held_window_loom` is a new test in `spectrum_handoff_loom`, so
  CI's existing host-core loom leg runs it.
  - Setup: a generation-1 window is held behind a stale record and has survived its block end.
  - Race: render runs `invalidate_after_failure` while control runs `cancel`, `arm` (generation
    2), a release store of an `armed` flag, and `try_read`. Before failing, render loads the flag.
  - The read is always `Pending` or `Invalid`, never a window.
  - After join, the word is at generation 2. It is `IDLE` when the read returned `Invalid` (the
    read parks it); otherwise it is `ARMED` or `INVALID`.
  - When render saw the flag, its failure ran after the arm. The arm must then be ended: either
    the read returned `Invalid` or the word is `INVALID`.
  - Mutation: dropping `word == held.latched` from the guard (M1) turns it red with "a failure
    after the re-arm left generation 2 armed".
  - Cost: the model runs in 0.13 s. The whole leg, run exactly as `qualification.yml` runs it,
    takes 35.5 s for 4 tests, so it stays in budget.
- **`one_shot_finish`'s let-else.** The impossible non-`Filling` path now has
  `debug_assert!(false, ...)`. The realtime policy trusts `debug_assert!`.

Gates at `3cf27c2a6`:
- the loom leg, as CI runs it: 4 pass;
- `cargo test --locked -p host-core`, with and without `test-support`: all pass;
- clippy `-D warnings`, with and without `test-support`: clean;
- `cargo fmt --check`: clean;
- `cargo doc` (with `control-provider`): clean;
- workspace policy: ok;
- realtime policy: `ok (292 marked regions in 46 files)`.

**Open.** None in scope. `SpectrumAnalysis`'s loose fields remain the #2255 follow-up.
