# 041 Issue-010 source streaming qualification tooling and adversarial evidence

## Outcome

Qualify the accepted source-streaming product with the missing adversarial corpus, seek-race,
native-worker-delay and duration-independent allocation evidence without blocking runner, host or
adapter implementation and without running a benchmark.

## Context

Engine V2 is greenfield and must not inspect or inherit V1. Render owns a preallocated plan and
performs zero allocation/free, locks, file/network I/O, logging, syscalls, thread lifecycle,
structural mutation or data-dependent unbounded work. Plan retirement stops and destroys workers
off render. Source PCM uses bounded planar rings; unavailable in-region samples are positive zero,
EOF is not an underrun, generations switch only at block boundaries, and source duration cannot
determine retained engine memory.

Issue **JIT PCM streaming and host-supplied source rings** remains FAIL. Issue
**Issue-010 launch-critical source ownership and accounting closure** owns and must first accept the
production lifetime/API/accounting contract. This issue adds no product feature. It owns only the
larger qualification evidence omitted from Issue 010: exact invalid-diagnostic corpus coverage,
frozen randomized seek races, a real delayed native worker in the 100,000-render audit, and actual
one-minute/sparse-multi-hour allocation-layout and RSS records.

This issue has exactly **two total attempts**: one Terra qualification/review attempt and, if
needed, one bounded Sol harness correction/review. A second failure stops. Any production semantic
defect fails and returns to a new product issue; it cannot be repaired here. Timing and benchmark
invocations are forbidden and remain zero.

## Scope

- Extend the existing single `miso-engine-source-fixture` generator/checker and sorted manifest;
  do not create a second fixture or conformance framework.
- Add nonzero-start/short-region valid cases and an exact diagnostic matrix spanning representative
  container/header/chunk/size/format/GUID/alignment/divisibility/parser-cap failures. Expected PCM
  bits and expected diagnostics remain independent from production parser/conversion code.
- Add exactly 256 frozen-seed randomized ring/seek schedules with strictly increasing generations,
  delayed old chunks, full/empty/wrap/EOF transitions and exact stale/underrun counters.
- Correct the existing source audit to delay the real production native decoder worker through a
  deterministic test-only/off-render synchronization seam, then render exactly 100,000
  48-kHz/128-frame blocks without any callback wait.
- Prepare an actual one-minute WAVE and an actual sparse multi-hour WAVE with identical format,
  channel, quantum and ring settings. Compare captured engine allocation-layout multisets and exact
  source reports, and record non-null descriptive Linux RSS separately.

## Required public interfaces/contracts

Qualification uses the production parser, native worker, ring, graph source set and prepared plan.
Any delay/injection hook is test-support-only, bounded, deterministic, off-render controlled and
absent from ordinary production builds. It cannot introduce a second worker implementation or a
render synchronization primitive.

The randomized generator is a frozen local algorithm with seed `0x00000000010a5ee1`, exactly 256
schedules and a recorded transcript hash. Its independent model decides only expected generation,
absolute frame, positive-zero gaps, EOF and counters; it does not call production ring logic.

The allocation multiset records semantic engine allocation category, requested size, alignment and
count without allocator headers. File length, sparse extents, OS page cache, thread stack and RSS
are separate observations. Both durations must yield byte-identical engine multisets and equal
exact resource reports. Linux RSS is numeric and descriptive; it is never substituted for exact
accounting or used as a performance threshold.

## Deliverables

- expanded sorted checksummed source corpus and exact invalid-diagnostic matrix;
- frozen 256-schedule seek-race transcript and independent expected-state model;
- corrected real-native-worker 100,000-render functional audit;
- actual one-minute/sparse-multi-hour allocation-layout and descriptive RSS evidence;
- focused tool/policy/workspace evidence; and
- a final evidence record with benchmark invocation count zero.

## Explicit non-goals

Changing production parser/ring/worker/source-set semantics; repairing a product defect; adding a
format or codec; performance tuning; timed benchmarking; device/browser runtime qualification;
general fuzzing; exhaustive WAVE interoperability; a second audit/fixture framework; SRC; or
repeating the already accepted full target matrix without a touched production boundary.

## Dependencies by exact issue title

- Issue-010 launch-critical source ownership and accounting closure
- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime

## Sol implementation brief (2026-08-21)

**READY FOR TERRA ATTEMPT 1 only after the product-closure dependency passes.** The tracked brief is
`.github/ISSUE_SPECS/BRIEFS/041-issue-010-source-streaming-qualification-tooling-and-adversarial-evidence.md`.
It freezes the seed/count, real-worker audit, allocation/RSS comparison, two-attempt maximum and
zero-timing rule.

## Hazards/decisions

A same-thread omitted host submission is not worker-delay evidence. Two calls to the same report
function with different labels are not allocation-layout evidence. `null` is not a recorded RSS
measurement. `is_err()` is not an exact diagnostic oracle. Do not let instrumentation allocate in
the armed render interval, retain duration-sized PCM, race on wall-clock sleeps, or adjust a seed,
count, fixture or expected result after observing a failure.

## Acceptance gates with objective measurements

1. The one sorted manifest covers existing RIFF/RF64, classic/extensible and six-encoding cases plus
   a valid nonzero-start short region. Independent expected bits and sanitation counts match. Each
   frozen invalid case asserts its exact `SourceDiagnosticCode`; representative root/header size,
   truncation, malformed `ds64`, duplicate `fmt `/`data`, unsupported tag/GUID/valid bits,
   byte-rate/block-align/data-divisibility and parser-cap classes are present and never panic or
   retain declared data length.
2. Exactly 256 schedules from seed `0x00000000010a5ee1` cover ring capacities 1, 2, 3 and 8
   quanta, wrap/full/empty/EOF, strictly increasing seeks and delayed old chunks. No old-generation
   sample renders after a request boundary; positive-zero output, stale counts, underrun frames/
   events, cursors and EOF match the independent model. Freeze the schedule transcript hash.
3. The production native worker is deterministically held outside render to make one in-region
   block unavailable, then released/seeked so PCM resumes at the declared next frame. Exactly
   100,000 48-kHz/128-frame prepared-plan renders report exact missing frames/maximal events,
   positive-zero silence, fixed addresses and zero allocation/free, lock, log, file/network I/O,
   syscall or structural mutation. Worker stop/join occurs only after disarm/reclamation.
4. Actual one-minute and sparse-multi-hour sources prepared with identical channel/ring settings
   have byte-identical `(category,size,alignment,count)` engine allocation multisets and equal exact
   resource reports. Decoder scratch and retained PCM are fixed; duration appears only in metadata/
   region values. Numeric Linux RSS observations are recorded separately with environment metadata
   and no threshold.
5. Fixture corruption checks, focused source/tool tests, format, locked workspace test/check,
   warning-denied Clippy/rustdoc and relevant workspace/realtime/source policies pass. Production
   target/object gates are rerun only if a supposedly test-only change alters ordinary reachability.
6. No benchmark workload, timing value, benchmark runner or retry is introduced or invoked;
   `timed_benchmark_invocations=0`.

## Target matrix

Linux x86-64 is mandatory for real native-worker, sparse-file and RSS evidence. Fixture and
randomized ring tests remain target-neutral. This issue makes no device, mobile or browser runtime
claim.

## Required evidence

Product-closure candidate hash; manifest and independent-oracle hashes; exact invalid diagnostic
table; seed/count/schedule transcript hash; real-worker lifecycle and 100,000-render audit record;
both sparse-source identities, allocation multisets, exact reports and numeric RSS/environment;
workspace/policy results; explicit `timed_benchmark_invocations=0`; and Terra plus final Sol
PASS/FAIL verdicts.
