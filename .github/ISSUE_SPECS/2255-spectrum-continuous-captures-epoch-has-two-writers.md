# Spectrum: continuous capture's epoch has two writers, so a restart racing a render failure can lose an epoch or report Failed for a fresh stream

Found by the implicit-state-machine audit of 2026-11-06. Terra attempt 1; five attempts maximum.

## Problem

Continuous spectrum capture's shared state (`SpectrumContinuousShared` in
`crates/host-core/src/spectrum.rs`: `active`, `phase`, `epoch`, `failures`, `failure_epoch`,
`drops`, `drop_epoch`, beside a separate `mode` word) is an implicit machine written by two threads.
Control's `restart_continuous`/`commit_continuous` and render's `continuous_fail` each load `epoch`
and store `epoch + 1`, so a restart racing a render failure can give two stream generations one
epoch, and render's `failures` increment can land after control's reset so the fresh stream reads
`Failed` for its own epoch. `mode = CONTINUOUS` with `active = 0` (render's response to an epoch or
failure-count overflow) is a state no transition names. `SpectrumService`
(`crates/host-core/src/spectrum_service.rs`) mirrors the stream in loose bools and options
(`stream_active`, `stream_cadence`, `stream_smoothing`, `stream_window_ready`, `caller_armed`,
`arm_dropped`) that can disagree.

## Contract

- One packed continuous state word (mode/run state, epoch, begun flag) replaces `mode`, `active`,
  `phase`, `epoch`, `failures` and `failure_epoch`. It is written only by compare-and-swap or one
  read-modify-write, with one writer per transition: control owns every transition into and out of
  the off state; render owns the in-epoch transitions (begun, failure, exhaustion), each a single
  compare-and-swap from the word it latched, which fails, and is moot, once control ended that
  epoch. Render never loops, waits or allocates.
- Drops stay a separate word whose only writer is render; they belong to the epoch whose begun bit
  render set after resetting them.
- The overflow state gets a name and a defined exit.
- `SpectrumService`'s stream and arm state becomes explicit enums.
- Reader-visible semantics (`NotActive`, `Warming`, `Pending`, `Gap`, `Failed`, drop counts) stay
  as documented unless the old behaviour was the bug; any change is recorded below.
- Edits only `crates/host-core/src/spectrum.rs`, `crates/host-core/src/spectrum_service.rs` and
  this spec.

## Gates

- A loom model of restart racing a render failure, in `spectrum_handoff_loom` (so CI's existing
  host-core loom leg runs it): red against the old multi-word design (or a recorded reason loom
  cannot express it), green after.
- `cargo test --locked -p host-core`; the loom leg; `cargo clippy --locked -p host-core
  --all-targets -- -D warnings`; `cargo fmt --check`; `scripts/check-workspace-policy.sh`;
  `tools/realtime-policy`.

## Evidence

(appended as the attempt proceeds)
