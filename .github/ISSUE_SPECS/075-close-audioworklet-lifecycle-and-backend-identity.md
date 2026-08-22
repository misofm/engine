# 075 Close AudioWorklet lifecycle and backend identity

## Outcome

Close the exact launch blockers in stopped Issue 024's checkpoint `ba7ffc6`, then execute the one
representative Chromium correctness gate needed to accept the immutable scalar/simd128 AudioWorklet
host. Preserve the frozen 14-export Wasm ABI, strict host-fed session, source/seek behavior and
artifacts; do not reopen DSP, graph, source, session or host architecture.

## Status, technical input and attempt budget

**FINAL SOL FAIL / STOPPED / RESCOPED; NO OVERALL PASS.** The bounded product and fixture checkpoint
at `a0c46af` remains technical input only. Terra plus the bounded Sol correction are consumed, as is
the sole browser authorization. No retry is permitted in this issue. Issue 076 owns only the
WebDriver response-compatibility repair, clean reseal and one fresh no-retry execution of the same
representative browser proof.

## Narrow product correction

1. After compiling and reacquiring `WebResourceReportV1`/`WebStatusV1`, map requested `scalar` to
   numeric backend 0 and `simd128` to 1. Both Rust records must equal that value before ready. Any
   mismatch disposes the live handle, terminates the processor and rejects creation; a swapped or
   mislabeled module can never claim the requested backend.
2. Before Wasm preparation, compare the worklet-global `sampleRate` with `sampleRateHz` exactly.
   Compare each exposed nonzero worklet quantum as already frozen. Mismatch returns
   `REPREPARE_REQUIRED`, prepares/renders nothing and terminates construction.
3. Make constructor failure transactional at every stage: validation; module instantiation; ABI;
   config handle; config write; prepare; TOML copy/compile; post-compile pointer/status/resource
   acquisition; backend identity; and ready publication. If a handle exists, dispose it exactly once.
   Mark the processor disposed so later `process()` returns `false`. The main wrapper settles the
   creation Promise exactly once, clears handlers, closes the port where supported and disconnects
   the node on every rejection. Successful explicit disposal remains idempotent.
4. Validate complete nested `miso.ready.v1`, `miso.status.v1`, resource and ACK/error values: exact
   keys, frozen scalar types/ranges, backend/rate/quantum identity, `memoryBytes` identity and returned
   source-plane types/offsets/lengths. Unknown or mismatched values reject address-free. Preserve the
   one-pending request, strict monotonic request IDs, unique transfer-list and returned ownership on
   all ordinary ACK/error paths. An unrecoverable user-agent/processor crash is reported as such and
   cannot promise recovery of buffers already transferred out of the main realm.
5. Extend the static realtime check through every helper callable from `process()`—currently only
   positive-zero silence—so moving an allocation, post, BigInt, feature check or memory growth into a
   helper fails. Require exact Wasm SIMD `f32x4.mul`, `f32x4.add` and `f32x4.sub`; scalar remains free
   of vector opcodes and both remain free of atomics, relaxed SIMD, imports and shared memory.

These are the only product edits. The 14 export names/signatures, config/status/resource layouts,
source staging, internal safe-host timeline, render `(handle, actual_frames)`, sticky mismatch,
resource formulas and Rust safe host remain unchanged. A required Rust/core/source/graph/session/DSP
change is a STOP.

## Representative browser gate

After focused Rust/JS/static/object gates pass, add one deterministic local fixture and one browser
runner. On a unique scratch artifact set and one clean committed candidate, the sole authorized
Chromium/Chrome invocation must:

- create a 48-kHz `OfflineAudioContext` with explicit actual-browser `quantumFrames`, checking every
  nonzero exposed main/worklet quantum;
- run forced scalar and supported simd128, with two fresh contexts per backend inside the same one
  browser invocation;
- prove ready backend equals both Rust backend records, source submit/returned ownership, seek,
  consecutive PCM continuation, positive-zero pre-ready/failure behavior, status/resource schema,
  internal sample progression and idempotent disposal;
- compare consecutive PCM plus `WebStatusV1`/`WebResourceReportV1` with the independent direct V2
  fixture using the frozen backend tolerance; and
- prove `memoryBytes` and `memory.buffer` identity remain unchanged after ready.

The command, browser identity, fixture and expected hashes must be sealed before authorization.
Failure is final; do not tune, rerun, directly invoke a browser binary or substitute a different
fixture/browser.

## Allowed files and non-goals

Allowed implementation is only `hosts/miso-engine-host-web/web/**`, the existing exact web build/
check/hermetic scripts, a bounded browser fixture/runner under `hosts/miso-engine-host-web/tests/**`
or `scripts/`, and Issue-075 evidence. `Cargo.toml`, `Cargo.lock`, Rust production/ABI files, accepted
corpora, CI and unrelated policies are frozen.

No broad browser/version/device matrix, demo/deployment breadth, million-quantum or ten-minute run,
GC/performance/bundle measurement, SAB/Atomics/threads, plan swap, decoder/SRC, network PCM,
third-party Wasm, benchmark, timing or listening. Those separable qualification rows remain Issue
074.

## Dependencies by exact issue title

- Bootstrap Rust workspace and target matrix
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Stable C ABI and host-fed planar PCM render
- Exact lock-free native source sanitation telemetry handoff
- Production SIMD builtin bank graph retention and reachability qualification
- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification

## Nonexecuting gates and evidence

Before browser authorization: shell syntax; focused hermetic JS tests with one mutation for every
correction stage; exact five-artifact/export/import/object/static checks in unique scratch; focused
host-web tests/check and warning-denied Clippy; format/diff; applicable workspace/realtime policies;
clean candidate/source/lock/tool/fixture seals; and static proof no browser/benchmark/timer has run.

Final evidence records strict Terra/Sol verdicts, exact commands/hashes, each constructor cleanup and
schema mutation, backend/rate rows, artifact/opcode records, the sole browser result and counters.
Workload/benchmark/timed counts must remain zero.

## Final Sol evidence and disposition

The clean `a0c46af` candidate passed the frozen nonexecuting product, artifact, independent-oracle,
fixture and lifecycle gates. Its no-browser seal is 2,744 bytes with SHA-256
`5f30451e45ba2d81e8ec665a726f0ca423268100ee3f812aab9ba16b7addf0c7`.

The sole authorized browser runner invocation then stopped during WebDriver navigation, before the
fixture import or any product/audio observation. The server returned the standards-conforming HTTP
200 response `{ "value": null }`; the generic Python request helper rejected every non-DELETE null
value and raised `RuntimeError` at `request()`/navigation. The W3C WebDriver Navigate To algorithm
explicitly returns [success with data null](https://www.w3.org/TR/webdriver2/#navigate-to), so this is
a runner compatibility defect, not an AudioWorklet or PCM result. The seal remains preserved;
browser evidence and checksum files were not created, the tree is clean and no browser/driver
process remains.

Final counters: `browser_correctness_invocations=1`, `workload_invocations=0`,
`benchmark_invocations=0`, `timed_invocations=0`. There is no product/audio PASS or FAIL result.
Issue 076 is the stateless successor; checkpoint `a0c46af`, its fixture and this failed invocation are
technical input rather than a PASS dependency.
