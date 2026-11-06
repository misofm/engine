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

### Attempt 1 (Terra, branch `codex/issue-2255-spectrum-continuous-word`)

Commits: `86b2bf398` (the word, the loom model, the exhaustion test), `9a6345e45` (service enums),
and this evidence commit.

**State model.** `SpectrumContinuousShared.word` (loom's atomic under `--cfg loom`): bits 0-1 run
state (`OFF` = one-shot mode, `RUNNING`, `EXHAUSTED`), bit 2 begun, bits 3-63 epoch
(`CONTINUOUS_EPOCH_MAX = 2^61 - 1`). Writers, every write one read-modify-write:

- control: start `(e, OFF) -> (e + 1, RUNNING)` by compare-and-swap (control alone writes an off
  word); stop/cancel `any -> (e, OFF)` by one `fetch_and`, which returns the exact word; restart =
  that stop then that start; when render spent the last epoch between restart's load and its stop,
  `(MAX, OFF) -> (MAX, EXHAUSTED)` by swap.
- render: begun, failure (`e -> e + 1`, unbegun) and exhaustion (`(MAX, RUNNING) -> EXHAUSTED`),
  each one compare-and-swap from the word it latched at the block's start; a failed swap means
  control ended that epoch, so the failure is moot. No loop, wait or allocation.
- `drops` stays its own word, render its only writer: reset before the begun swap, added to by load
  and store. The reader trusts it only while the word is begun and unchanged around the load.
- The reader keeps `seen_epoch` instead of the `failures` counter: only render moves a running
  stream's epoch, so a word epoch past `seen_epoch` is the unreported failure.
- `mode`, `active`, `phase`, `epoch`, `failures`, `failure_epoch`, `drop_epoch` are gone; the
  observer no longer holds a separate mode `Arc`, so the per-capture resource charge drops one
  `SpectrumStateAllocation`.

**Stuck state.** `mode = CONTINUOUS, active = 0` is now the named `EXHAUSTED`: it captures
nothing, reads `NotActive` (as before), refuses a restart with `EpochOverflow`, refuses an arm
with `Busy`, and leaves through stop or cancel to an armable one-shot capture.

**Service.** `SpectrumService`'s six loose fields become `CallerArm { Free, Held, Dropped }` and
`ServiceStream { Stopped, Running { cadence, smoothing, window_ready } }`. Two enums, not one: a
`Dropped` arm survives a stream start today (the next one-shot `read` still answers `Dropped`
once), and one enum would lose it. `SpectrumAnalysis` keeps its own fields (out of this scope).

**Reader-visible changes** (all at a boundary or a race the old code got wrong):

1. A restart racing a render failure: before, both could take `e + 1` and the fresh stream could
   read `Failed` for its own epoch; now the failure lands first (the restart takes `e + 2`) or is
   moot. This is the bug.
2. Records carry the epoch render latched at the block's start, not the epoch loaded at publish:
   a window of history captured before a restart can no longer be tagged with the restart's epoch.
3. Render skips a block whose begun swap fails (control ended the epoch); before, it captured it
   under the old epoch and the reader discarded the record.
4. `restart_continuous` on an exhausted stream returns `EpochOverflow` (before: `Busy`); epoch
   overflow is at `2^61 - 1` (before `u64::MAX`); a restart that loses the race to the last epoch
   leaves the stream `EXHAUSTED` rather than running, since restoring it would lose a render
   failure the stop made moot.
5. A `Failed` read whose drop count is unknown (the word moved around the load) sets the seen
   count to 0, so drops already counted in the new epoch are reported by a later `Gap`; before they
   were absorbed. A `Gap`'s epoch is the current epoch (before: the epoch stored at the drop, equal
   except under a race).

**Red/green.** Loom leg (CI's command, `.github/workflows/qualification.yml` "spectrum handoff loom
leg"): `RUSTFLAGS='--cfg loom --check-cfg=cfg(loom) -C target-feature=+avx2,+fma' cargo test
--locked --release -p host-core --lib spectrum_handoff_loom`; the new model sits in that module so
the existing leg runs it. Red on the old design, with its shared words switched to loom atomics
for the run only (otherwise loom cannot interleave them; patch not committed):

```
test spectrum::tests::spectrum_handoff_loom::spectrum_continuous_restart_loom ... FAILED
assertion `left != right` failed: render's failure and control's restart claimed one epoch
  left: 2
 right: 2
```

and, with that first assertion disabled, the second:
`the fresh stream 2 read a failure of epoch 2`. Green after: `3 passed` (the new model takes about
30 s of the leg's 34 s).

**Test value.**

- `spectrum_continuous_restart_loom`: turns red when a render failure and a restart can both take
  the next epoch, or a failure of the ended epoch reaches the fresh stream as `Failed` (shown red on
  the old design); no existing test races a restart against a failure.
- `a_failure_at_the_last_epoch_exhausts_the_stream_and_a_stop_leaves_it`: turns red when a failure
  at the last epoch wraps the packed epoch (mutation `epoch <= MAX` in `continuous_fail`: red,
  word `1` instead of the exhausted word) or the exhausted state has no exit; no existing test
  reaches epoch overflow.

Existing tests adapted, not weakened: the drain tests' "make it run" setup writes the running word;
`inactive_observer_does_no_validation_or_capture_work` uses the exhausted state (its old setup was
the unnamed stuck state); the resident-lane test latches the word as the observer entry does.

**Gates.** `cargo test --locked -p host-core`: 290 passed, 0 failed. Dependents
`cargo test --locked -p control-plane -p capi -p host-web`: 454 passed, 0 failed;
`cargo check --locked --workspace --all-targets` clean (one pre-existing host-web warning). Loom
leg: 3 passed. `cargo clippy --locked -p host-core --all-targets -- -D warnings`: clean.
`cargo fmt --check -p host-core`: clean. `scripts/check-workspace-policy.sh`: ok.
`realtime_policy`: ok (291 marked regions in 46 files).
