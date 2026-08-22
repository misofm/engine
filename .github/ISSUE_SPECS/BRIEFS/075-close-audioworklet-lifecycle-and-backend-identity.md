# Sol implementation brief — issue 075 close AudioWorklet lifecycle and backend identity

## Decision

**READY FOR TERRA ATTEMPT 1.** Consume stopped Issue-024 checkpoint `ba7ffc6` as technical input,
not a PASS dependency. Use one Terra attempt plus one bounded Sol correction. Close only the exact
JS/worklet identity/lifetime defects and then run at most one preauthorized representative browser
correctness invocation. Benchmark/workload/timed counts remain zero.

## Literal correction

- Compare requested backend with both post-compile Rust resource/status numeric backends
  (`scalar=0`, `simd128=1`) before ready. Swapped/mislabeled modules reject and clean up.
- Compare worklet-global `sampleRate` exactly before prepare and retain the exposed-nonzero quantum
  comparison. Mismatch is `REPREPARE_REQUIRED` with no prepared/rendered product.
- Funnel every constructor return/throw through one init-failure finalizer. Dispose a nonzero handle
  once, mark disposed, post one address-free error and make `process()` return `false`. On main-realm
  creation rejection, settle once, remove handlers, close the port where supported and disconnect.
- Parse ready/status/resources/ACK/error recursively: exact keys, types/ranges and backend/rate/
  quantum/memory identities. Preserve returned source views and unique underlying buffers. Keep the
  existing request/transfer behavior; disclose that an actual user-agent crash cannot return already
  transferred storage.
- Make the process policy transitive over `silence` and require SIMD `f32x4.mul/add/sub`. Do not edit
  the frozen Rust safe host, ABI or resource layouts.

Hermetic mutations must independently catch a swapped artifact, each global rate/quantum mismatch,
every constructor stage after handle creation, malformed nested ready/status/resource values,
malformed returned planes, helper allocation/post/BigInt/memory growth and missing/extra SIMD ops.
All failure rows prove exact one-time disposal and no live processor.

## One browser authorization

Once nonexecuting gates are green on a clean committed candidate, hand Sol the sealed browser command,
browser identity and input/expected hashes. Sol may authorize it exactly once, without retry. That
single invocation contains forced scalar and supported simd128, two fresh contexts each, at 48 kHz
and explicit actual-browser quantum. It proves backend identity, source/seek/ownership, consecutive
PCM against the independent direct fixture, status/resources, unchanged memory and disposal. It is
a correctness invocation, never a workload, timer or benchmark.

Issue 074 alone owns demo/deployment, browser/device breadth, long runs, GC, bundle and performance.
