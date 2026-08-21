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

Issues **JIT PCM streaming and host-supplied source rings** and **Issue-010 launch-critical source
ownership and accounting closure** remain FAIL. Issue **Exact lock-free native source sanitation
telemetry handoff** must first accept the preserved lifetime/API/accounting checkpoint plus its
remaining telemetry correction. This issue adds no product feature. It owns only the
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

- Exact lock-free native source sanitation telemetry handoff
- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime

## Sol implementation brief (2026-08-21)

**READY FOR TERRA ATTEMPT 1 only after the replacement product-closure dependency passes.** The tracked brief is
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

## Terra attempt 1 — fixture/diagnostic checkpoint (2026-08-21)

**Gate 1 PASS; qualification remains incomplete.** The sole existing
`miso-engine-source-fixture` framework and sorted `fixtures/sources/v1` manifest now contain the
accepted RIFF/RF64 classic/extensible six-encoding corpus plus
`riff-pcm16-stereo-nonzero-short-region-v1`: a three-frame PCM16 stereo source decoded only from
absolute frame 1 for one frame. Its independent planar-bit oracle verifies that short terminal
region, exact EOF, signed-zero preservation where applicable, and sanitation counts.

The frozen exact diagnostic matrix is production-oracle independent and names these mutations:
`container-rifx`, `riff-root-size`, `truncated-container`, `rf64-ds64-size`,
`rf64-ds64-table`, `rf64-data-placeholder`, `duplicate-fmt`, `duplicate-data`,
`unsupported-compression-tag`, `unsupported-extensible-guid`,
`extensible-valid-container-bits`, `byte-rate`, `block-align`, `data-frame-divisibility`,
`chunk-count-cap`, and `skipped-metadata-cap`. It requires the exact stable
`SourceDiagnosticCode` for each rather than merely accepting an error; malformed containers map to
`source.container.invalid`, unsupported format forms to `source.format.unsupported`, and fixed
parser caps to `source.resource.limit`.

The manifest SHA-256 is
`cc3eb2cb547b32dee751aa0a7246ddc3926d692cda45c3418ac1b7be479e0b79`; the independent checker
source SHA-256 is `b2bba2a13233218ebcf148821899f11ef7f172b5619883b9d13204d23b70b675`.
Focused PASS: `cargo fmt --check -p miso-engine-source-fixture`; `cargo test -p
miso-engine-source-fixture --locked`; direct checker invocation; and warning-denied
`cargo clippy -p miso-engine-source-fixture --all-targets --locked -- -D warnings`.

Not yet run: the frozen 256-schedule model, real-worker 100,000-render audit, sparse-duration
allocation/RSS evidence, or broader workspace/policy gates. No production source semantics changed
and no benchmark/timing workload was invoked; `timed_benchmark_invocations=0`.

## Terra attempt 1 — frozen seek/ring checkpoint (2026-08-21)

**Gate 2 PASS; qualification remains incomplete.** The same source fixture checker now generates
exactly 256 sealed one-channel/4-frame ring schedules from seed `0x00000000010a5ee1`, cycling
capacities `1, 2, 3, 8` quanta. The canonical action-byte transcript SHA-256 is
`ec3b7fef8e86937d4431466d2cea8a68ec56feb2897bcdc655fa10d5bf30a41c` and is checked before the
candidate ring is evaluated.

The test-only independent bounded model owns no production endpoint and models only that frozen
action language. It predicts accepted/full/stale submissions; strictly increasing generation-2 and
generation-3 seek boundaries; queued old-block discard; empty/missing positive-zero output;
wrap/recycle; full; short and zero-frame EOF; copied frames; cumulative stale discards; and exact
underrun frame/event totals. Every production `PcmSourceRing` schedule is then compared action by
action against those predicted output bits and reports. The model caught and corrected a fixture
sequencing mistake before publication; no production source behavior was changed.

Focused PASS: `cargo fmt --check -p miso-engine-source-fixture`; `cargo test -p
miso-engine-source-fixture --locked`; direct checker invocation; and warning-denied `cargo clippy
-p miso-engine-source-fixture --all-targets --locked -- -D warnings`. The current checker source
SHA-256 is `fb68fbd5bc86ea3e200db330dbe2f3cc7f0297f061a604dc2648e6603fd507f1`.

Not yet run: real-worker 100,000-render audit, sparse-duration allocation/RSS evidence, or broad
workspace/policy gates. No benchmark/timing workload was invoked; `timed_benchmark_invocations=0`.

## Terra attempt 1 — real native-worker audit checkpoint (2026-08-21)

**Gate 3 PASS; qualification remains incomplete.** The existing
`miso-engine-source-audit` now uses the production native resolver, RIFF/WAVE parser, decoder
worker, bounded ring, sealed graph source set and `PreparedRenderPlan`. A `test-support`-only
native worker gate uses three existing move-owned capacity-one SPSC exchanges: the controller waits
off render for a post-seek submitted prefill to be held, renders the declared unavailable quantum,
then queues the strictly newer resume seek and releases off render. The gate is absent without the
source crate's `test-support` feature and introduces no `Arc`, lock, render synchronization, second
worker, or ordinary source-path behavior.

The one final functional invocation rendered exactly 100,000 48-kHz/128-frame plan blocks. It
observed one positive-zero missing block (`underrun_frames=128`, `underrun_events=1`), resumed
finite PCM at declared source frame `384`, retained the output address, reported zero render
allocation/deallocation/lock/log/file/network/syscall violations, and dropped the plan before
waiting for the native terminal event, so stop/join remained off render. The emitted record was:
`{"blocks":100000,"quantum_frames":128,"underrun_frames":128,"underrun_events":1,"resumed_source_frame":384,"native_worker_hold_release":true,"total_violations":0}`.

For candor, an earlier **pre-render harness topology** used an initially full two-block ring and
asked the worker gate to hold only after a post-seek submission; the post-seek submission could not
obtain a recycled block, so its off-render hold wait deadlocked before the render loop. That process
was terminated. The harness alone was corrected to a four-block EOF prefill in a five-block ring,
leaving one preallocated block for the held post-seek prefill, and to queue the newer resume seek
before release. No production source behavior changed; exactly one final 100,000-block audit was
then run successfully. This was functional evidence, not a benchmark or timing run.

Focused PASS: source/source-audit format check; locked source and audit check/tests (29 source unit
tests plus one compile-fail doctest); warning-denied Clippy with the test-support feature;
realtime-policy mutation tests; and `git diff --check`. The baseline realtime policy check is
currently externally blocked by pre-existing unapproved `unsafe` in
`tools/miso-engine-graph-audit/src/parametric_eq_main.rs` (outside Issue 041); no source policy
exception was added. No Gate 4 RSS/allocation-layout work was started and
`timed_benchmark_invocations=0`.
