# Sol implementation brief — issue 101 native-source seek backpressure

## Decision

**STOPPED / SECOND SOL XHIGH HOLD / NO OVERALL PASS.** Sol High completed one focused pass and the
sole bounded correction; Sol XHigh reviewed both. No further Issue-101 implementation or seal retry
is authorized.

Use clean baseline `e1fbbb65` / tree `d35388fc`. Root must synchronize remote Issue 101 to the exact
title `101 Close native-source seek backpressure without worker termination`. The sole direct
dependency is accepted Issue 043, **Exact lock-free native source sanitation telemetry handoff**;
Issue 010 is transitive historical input only. Issue 101 did not pass. Issue 112, **Close
native-source seek submission qualification and seal backpressure fix**, owns the remaining
test-only synchronization and fresh seal before Issue 073, **Native PCM reference runner and C ABI
qualification**.

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

## Terminal evidence and handoff

Pass 1 received strict Sol XHigh focused PASS for the private pending-seek production correction.
The sole broad nonbenchmark seal on clean `dfdefff` / tree `49f03a` then stopped after format and
locked check passed: the single-seek continuation test read zero for its second block. Preserved
counts are 543 passed, 1 failed and 8 ignored overall, with source 33 passed and 1 failed; later
broad gates were skipped and no retry ran.

The cause was test-only: `SnapshotSanitation` is emitted before later decode/submission and cannot
prove PCM readiness. The sole bounded correction at `02bec81` made the audit gate repeatable and
replaced that test's readiness proxy with two explicit submit acknowledgements. Source all-feature
tests then passed 34/34 with focused format/check/Clippy/rustdoc gates. The resulting one-file SHA is
`d8fd1762702a5b75a2943b8b99a45724e67afb7a82e5f63bff4f8bcb3f8aa98a`; frozen `lib.rs` and
`Cargo.lock` retained the spec's hashes.

Final Sol XHigh review found one remaining snapshot-as-readiness call immediately before the
coalescing test's required frame-24 read. It can return before that PCM is submitted, so one green
focused run is not deterministic evidence. This second HOLD exhausts Issue 101. The implementation
is technical input only; there is no product defect finding and no overall PASS.

Issue 112 must preserve production semantics and close only that test-support handshake before one
fresh clean broad nonbenchmark seal. All real source workload, benchmark, timing, fuzz, playback and
listening counters remain zero. The route is `043 -> 101 (stopped) -> 112 -> 073`.
