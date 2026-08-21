# Sol implementation brief — issue 041 source streaming qualification tooling and adversarial evidence

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1 only after Issue-043 has a recorded PASS.** Qualify that exact product
candidate; do not repair production semantics here. This issue permits one Terra
qualification/review attempt and at most one bounded Sol correction to test-only tooling. A second
failure stops. A production defect immediately FAILs and requires a new stateless product issue.

Do not time, benchmark, tune or retry a workload. `timed_benchmark_invocations=0` remains invariant.

## Reuse one framework

Extend only `miso-engine-source-fixture`, `fixtures/sources/v1` and the existing source audit. A
small test-support-only native-worker gate or allocation recorder may be added inside the existing
source/tool boundary if it compiles away from ordinary builds. Do not add a second parser oracle,
ring implementation, worker implementation, general fuzzer, benchmark harness or qualification
framework.

Expected PCM bits, exact diagnostic codes and seek-model outcomes must be constructed independently
from production parsing, conversion and ring code. Freeze inputs before evaluating the candidate.

## Frozen corpus and diagnostic matrix

Retain the existing sorted SHA-256 manifest and six-encoding RIFF/RF64 cases. Add at least one valid
nonzero-start region whose length ends within a quantum and verify exact planar bits, signed zero,
sanitation count and EOF without underrun.

Add one named mutation for each required class: RIFF root/header size, truncation, malformed RF64
`ds64`/table/placeholder, duplicate `fmt `, duplicate `data`, unsupported compression tag,
unsupported extensible GUID, mismatched valid/container bits, byte rate, block align, data-frame
divisibility, chunk-count cap and skipped-metadata cap. Store each expected stable
`SourceDiagnosticCode`; `is_err()` alone fails. Mutations must not allocate from declared payload
length or panic.

## Frozen seek-race matrix

Use seed `0x00000000010a5ee1` and exactly 256 generated schedules. Cycle exact ring capacities 1, 2,
3 and 8 quanta; include full/empty/wrap, final/zero-frame EOF, strictly increasing generation
requests, queued and delayed old chunks, missing new chunks and recovery only at declared absolute
frames. Freeze the generated action transcript hash before interpreting production output.

An independent bounded model yields exact output bits, active generation, cursors, stale discards,
underrun frames, maximal underrun events and EOF. After a request boundary no older-generation bit
may render. All source-layer silence is positive zero. Do not use wall-clock scheduling or random
thread interleavings as the oracle.

## Corrected real-worker audit

Use the production native resolver, decoder worker, ring, graph source set and
`PreparedRenderPlan`, not a same-thread host producer. At 48 kHz and quantum 128, prefill declared
PCM, deterministically hold the worker outside render so one in-region quantum is unavailable, then
release/seek it so the next PCM begins at the declared frame. Synchronization and waits occur only
before/after individual callback scopes and cannot depend on sleeps.

Render exactly 100,000 blocks. Assert exact positive-zero missing samples, one maximal event per
missing run, resume frame/PCM bits, fixed output/source addresses and zero allocation, deallocation,
lock, log, file/network I/O, syscall and structural mutation in render. Disarm before telemetry,
worker stop/join and destruction. Freeze the lifecycle/counter/PCM transcript hash.

## Duration-independent allocation and RSS proof

Generate an actual one-minute WAVE and actual sparse multi-hour WAVE on Linux with identical
container/encoding/channel/rate, region-independent ring configuration and bounded parser caps. Do
not materialize duration-sized PCM in memory. Prepare each from a clean equivalent state.

Capture the engine-owned allocation multiset as exact `(semantic_category, requested_size,
alignment, count)` entries using fixed test instrumentation. Compare it byte-for-byte and compare
all exact source/session/graph resource-report fields. File size may change parsed metadata and
region counters only. Record numeric Linux RSS around each preparation plus OS/toolchain/process
metadata separately; impose no RSS or timing threshold and never use RSS to forgive a multiset or
report mismatch.

## Ordered gates

1. Sorted manifest/checksums, valid subregion oracle and exact diagnostic mutation table.
2. Exactly 256 frozen-seed seek schedules and transcript/model equality.
3. Exactly 100,000 real-worker prepared-plan renders with exact counters and zero forbidden work.
4. Actual one-minute/sparse-multi-hour allocation multiset/report equality and numeric RSS record.
5. Focused source/fixture/audit tests, format, locked workspace check/test, warning-denied Clippy/
   rustdoc and relevant policy checks. Recheck target reachability only if ordinary production code
   changed unexpectedly.
6. Explicit absence of benchmark/timing artifacts and invocation count zero.

## Stop conditions and dependency rule

FAIL for production changes disguised as instrumentation, a synthetic worker/ring, sleep-based
races, mutable seed/count/corpus, production-derived oracle, `is_err()` diagnostics, host omission
claimed as worker delay, report values claimed as captured layouts, `null` RSS, duration PCM
retention, an armed render violation, any timing/benchmark call or a third attempt.

This issue is nonblocking for native runner, mobile, browser, effects and other source consumers.
Only final release qualification depends on its PASS.
