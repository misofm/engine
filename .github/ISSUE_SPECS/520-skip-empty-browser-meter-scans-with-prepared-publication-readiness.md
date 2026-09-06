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

## Implementation decision and evidence

- `ReadyOwnership::master_count` remains the sole readiness authority. After lease, ready-state and
  attached-meter checks, `poll_meters` now returns immediately when that count is zero. The return
  precedes drain-budget construction, every track consumer access, pending-candidate inspection and
  effect-observation scanning. It adds no retained state or resource-report delta.
- A test-only work probe sums the existing per-consumer successful/empty pop counters and counts
  effect-observation entries at the actual scan site. For periods of 2, 8 and 32 quanta, every
  incomplete-quantum poll leaves both counts unchanged. A prepared effect-observation fixture also
  proves the early path reaches zero effect scans rather than relying on an empty effect table.
- The same test renders identical active PCM into an every-quantum polling host and a boundary-only
  oracle. At each completed interval their frame and header are exactly equal; an early full-scale
  impulse survives, and a partial trailing interval preserves the previous publication byte for
  byte.
- Existing delayed-window, saturation, asymmetric missing-master, producer-reset, stale full-queue,
  lease reacquisition, source-seek, four-launch-rate and nine-track-tail tests remain the lifecycle
  evidence. A missing master now consumes the older track-only candidate on the first ready poll
  and publishes the matching queued interval on the next bounded poll, with loss visible; an empty
  master ring no longer drives speculative track draining.
- Focused evidence on 2026-09-06: the new readiness test passes and all 12 meter-filtered host-web
  tests pass. Final full host-web, realtime source-policy and shipped Wasm qualifications remain for
  the frozen checkpoint.


## Timing disposition and independent review

Dedicated Astra medium source review passed, conditional on final shipped artifact qualification. The bounded timing helper was separately reviewed after correcting its JSON serializer. Release preflight passed. The sole timing invocation at `11cb3c2e` then failed during warmup: its final source chunk omitted the end-of-region marker and was correctly rejected. No measured rounds or timing figures exist. Raw evidence is committed in `docs/evidence/metering-519-520/timing-failed.log` and `timing-failed.jsonl`. No retry was performed and no measured speedup is claimed.

Per the repository's bounded benchmark-failure rule, the remaining timing requirement is explicitly transferred to [#522](https://github.com/misofm/engine/issues/522), whose stateless brief was written and reviewed by dedicated Astra medium. That tooling successor does not relax numerical, realtime, resource or work-avoidance gates for this feature.


## Final shipped-artifact qualification

Canonical Linux/amd64 Rust 1.97.1 build of candidate `a1aeefb830b2ac192274153539847765385ed7f1` (same executable source as `11cb3c2e`, reviewed pin adopted at `924e524e`) produces SHA-256 `671d615de9a74232c2af623592fc85eca68b8cd571fb5b7542ba648e621af4e3`. The ordinary fingerprint-enforcing builder passed. The unchanged static/object/callback/resource gates and hermetic worklet/mutation suite passed. Browser qualification passed Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5, including mutation proofs; the committed results/matrix identifies this exact candidate and artifact.

Focused source suites passed: 73 host-web tests (two explicitly ignored release timing tests), selective builtins tests covering all 15 nonempty subsets and zero forbidden peak-only operations, compiler 38 tests, host-core unit 8 tests and preparation 13 tests (one intended ignored). Formatting, generated matrix consistency and realtime policy passed. The earlier baseline-only native tangent ULP failure remains documented; DSP arithmetic is unchanged. No callback/resource gate was weakened.

Timing remains unavailable: the single invocation failed during fixture warmup, and repair/measurement is explicitly assigned to #522. The supported performance result is avoided operations and empty scans, not a measured `µs/block` or end-to-end browser speedup. SDK/app rendering and presentation tuning remain downstream work.
