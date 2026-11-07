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
- No change to the one-shot word's protocol, its orderings or the number of atomic operations
  render performs; `render` stays allocation-, lock- and syscall-free.
- Edits only `crates/host-core/src/spectrum.rs` and this spec (not `spectrum_service.rs`).

## Non-goals

`SpectrumAnalysis`'s loose fields (the #2255 follow-up), the continuous state, and the cross-thread
word.

## Gates

- A regression test red on `24ea03525`, green after.
- A one-time no-bit-moved comparison against the base, recorded below, not committed.
- `spectrum_handoff_loom` (the qualification host-core loom leg), since its render thread reads the
  one-shot state.
- `cargo test --locked -p host-core --features test-support`; `cargo clippy --locked -p host-core
  --all-targets -- -D warnings`; `cargo fmt --check`; `cargo doc`; `scripts/check-workspace-policy.sh`;
  the realtime policy tool.

## Evidence

Pending.
