# Skip empty browser meter scans with prepared publication readiness

## Problem

With a meter lease active, the AudioWorklet calls `meter_poll` after every rendered quantum. For a
multi-block meter period, `AudioWorkletEngineHost::poll_meters` currently walks the prepared track
set and attempts queue pops on every incomplete quantum even though no complete master interval can
exist. Publication is already decimated; the empty scans are avoidable control work. Observation of
audio samples must remain continuous, and loss/recovery polls at completed boundaries must retain
the #516 correctness contract.

This is the second efficiency successor to #516 and does not depend on selective metrics.

## Smallest useful slice

Use the host’s existing fixed-capacity master interval ring as the readiness authority. At the start
of `poll_meters`, after lease/state/attachment checks, return zero in O(1) when `master_count == 0`.
Do not inspect track consumers, pending candidates or effect-observation readers on that path. The
render path already increments master readiness only when it closes the configured period and keeps
partial master peaks across empty polls.

When a master interval is ready, retain the existing capacity-derived drain budget and exact
track/master validation. A ready master does not assert that every track survived: asymmetric loss,
stale generation data and saturated queues still enter the bounded recovery path and publish only a
coherent interval. Empty return continues to preserve every public frame/header byte.

This slice intentionally keeps the existing worklet call per leased block. It removes the expensive
empty track/effect scan without adding an ABI readiness export, timer, callback, shared atomic,
runtime subscription or application cadence setting. If measurements show the remaining O(1) FFI
call matters, propose it separately with browser evidence.

No new retained allocation should be needed. If implementation requires state, it must be fixed at
preparation, charged before the exact budget gate, and justified against the existing `master_count`
authority.

## Acceptance evidence

- For periods of 2, 8 and 32 quanta, polling after every quantum performs zero track queue pop
  attempts and zero effect-reader scans before a master interval is complete, then publishes the
  same exact frame at the boundary. Use existing consumer empty/pop counters plus a focused
  test-only effect-scan counter; do not infer work avoidance only from return value.
- Peak payload, generation, validity/loss count, sequence and half-open span are bit-identical to a
  boundary-only polling oracle for active audio, silence and a partial trailing period.
- An early master impulse survives intervening O(1) empty polls. Empty polls preserve the last
  complete publication and unfinished master state.
- Delayed polling across multiple ready windows still delivers one bounded window per poll.
  Queue saturation, asymmetric track/master loss, producer reset, and full stale queues after lease
  reacquisition recover within prepared budgets and expose loss; readiness never hides recovery.
- Meter-disabled preparation binds no observers. Lease-off performs no scan. Meter-enabled and
  disabled PCM remain bit-identical for arbitrary track counts including SIMD tails.
- Existing callback allocation/free, syscall, trap-owner and Wasm static gates pass. Focused
  host-web tests cover all four launch rates; affected native host-core/builtins tests remain green.
- Exact resource reports are unchanged, or any fixed delta is independently derived and passes the
  exact retained and maximum-allocation gates.

Freeze a long-window active-audio fixture before measurement. Report poll calls, track-pop attempts,
effect-reader scans and callback duration for the current and changed paths in one invocation with
one warmup and two measured rounds. Observation cadence and emitted values must match.

## Out of scope

No metric selection, new subscription protocol, SDK/app cadence API, queue replacement, aggregated
windows, network transport, UI work, benchmark framework, or DSP kernel optimization.
