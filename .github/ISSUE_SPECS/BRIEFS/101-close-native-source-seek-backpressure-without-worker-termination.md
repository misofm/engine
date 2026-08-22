# Sol implementation brief — issue 101 native-source seek backpressure

## Decision

**SOL XHIGH READINESS PASS / READY FOR SOL HIGH PASS 1 / ZERO WORKLOAD, BENCHMARK AND TIMING.** Sol
High implements one focused pass; Sol XHigh adversarially verifies. One bounded HOLD correction is
the only second pass. A second HOLD is terminal STOP/rescope.

Use clean baseline `e1fbbb65` / tree `d35388fc`. Root must synchronize remote Issue 101 to the exact
title `101 Close native-source seek backpressure without worker termination`. The sole direct
dependency is accepted Issue 043, **Exact lock-free native source sanitation telemetry handoff**;
Issue 010 is transitive historical input only. Accepted Issue 101 then gates Issue 073, **Native PCM
reference runner and C ABI qualification**.

## Sole defect and implementation seam

The native controller's bounded worker queue can accept multiple strictly increasing seeks while
the provider's separate one-slot render seek queue is still occupied. Today `apply_command` seeks
the decoder, receives provider `SourceSeekError::Backpressure`, maps it to terminal
`NativeSourceWorkerExit::SeekFailed`, and permanently silences the source.

Modify only private worker state and tests in
`crates/miso-engine-source/src/native_source.rs`. Retain one fixed-size pending seek. During one
worker turn, inspect no more than the existing command-queue capacity and keep the greatest accepted
seek generation. Clear pending decoded PCM/end state as soon as a seek becomes worker-local. Until
the latest pending seek is accepted by the provider, perform no decode or PCM submission for it.
Provider backpressure retains the value and continues; it never terminates the worker. Provider
success updates the worker generation and resumes exact-frame decoding. Stop, wake and sanitation
snapshot remain bounded and ordered; decoder and genuine invariant failures keep their current
terminal types.

The already-enqueued provider seek cannot be withdrawn. It may update consumer bookkeeping for one
boundary, but no intermediate newly decoded audio may be heard. After capacity becomes available,
the greatest worker-local accepted generation is the next delivered seek and the first newly
decoded audible samples begin at its requested frame. Existing generation/frame discard rules
remain the sole render-side enforcement.

## Nonnegotiable boundaries

No public API or enum change; no new diagnostic; no queue resize; no resource/allocation report
change; no `Cargo.lock` change; no edit to `src/lib.rs`; no WAV/parser, worker sleep/wake, topology,
event-schema, host sanitation or performance work. `SeekFailed` stays public, while its
`Backpressure` instance becomes unreachable. The render path is unchanged.

F2–F12 from the remote audit are explicitly deferred. Cross-crate duplication remains coordinated
with Issue 083. Do not opportunistically fix them.

## Focused proof

- Pin the provider slot full, accept at least generations 2/3/4 before a controlled render drain,
  and prove the worker stays live with no terminal/seek-failed event.
- Prove no decode/submit while the latest seek is pending; after bounded drains, generation 4 is the
  next newly decoded audible generation at its exact unequal frame and intermediate PCM is absent.
- Cover one seek, a replacement arriving during backpressure, unchanged controller-queue
  backpressure, wake/snapshot, decoder failure, and stop/join with the provider slot still full.
- Prove stale PCM discard, exact contiguity, sanitation/end state, unchanged queue/resource reports,
  unchanged public surface and zero post-preparation allocation from the correction.

Run focused package tests, warning-denied all-target/all-feature lint/docs, format, applicable
source/realtime policies and mutations, and static API/resource/docs/diff checks. Root alone may
later authorize one clean nonbenchmark workspace seal. Do not run real source workloads,
benchmarks, timers, fuzz targets, playback or listening.

## Paths and checkpoint

Implementation is restricted to `crates/miso-engine-source/src/native_source.rs` and its colocated
tests. The two Issue-101 docs plus minimal README/implementation-plan routing are the only docs.
Anything broader is STOP/rescope. Sol High pauses with a coherent focused-green tranche; Sol XHigh
PASS authorizes only a checkpoint, not execution outside these gates.
