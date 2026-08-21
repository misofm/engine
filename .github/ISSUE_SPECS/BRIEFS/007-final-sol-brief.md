# Sol implementation brief — issue 007 final machine-evidence rescope

## Decision and attempt budget

**READY FOR FINAL ATTEMPT 3.** This brief supersedes all earlier issue-007 briefs except as
historical evidence. The conditioned incremental all-`f32` TPT recurrence and the retained
checkpoint `0627618` are not reopened. Never inspect V1/legacy. The exactly-once benchmark
invocation count is **0**; briefing and preflight must not launch a timed workload.

Attempt 3 is limited to completing the machine-verifiable issue-007 contract: truthful resources
and ownership, complete fixtures, graph-backed render/swap evidence, and the fixed descriptive
benchmark. A failure of any gate ends issue 007 after this attempt. Do not lower a tolerance,
replace expected outputs with declarations, accept a self-consistent estimate as actual
containment, or run/tune/retry the timed benchmark.

Real listening execution moves to the stateless issue **Issue-007 builtin filter and matrix human
listening qualification**. This is an ordering correction, not a sound-quality waiver: issue 007
may claim only *machine-qualified*, the listening issue is a hard dependency of end-to-end release
qualification, and no audible-quality or launch-readiness statement is permitted before it passes.

## Retained DSP and functional contract

Retain without semantic change:

- three graph sections in exact order: polarity/trim/HPF/LPF, then racks, then fader/mute, then the
  explicit smoothed 2x2 matrix;
- independent L/R parameters and filter state, zero builtin latency, enabled-filter infinite
  tail, constant-power pan and cosine balance adapters, and exact N-update matrix smoothing;
- the recurrence brief's prepared `c1/a2/a3/k`, `ic1/ic2`, non-fused all-`f32` operation order,
  pairwise recovery, sanitization, reset and response thresholds at 44.1/48/88.2/96 kHz;
- all seven transparent track-boundary meters, exact interval/cumulative semantics and bounded
  SPSC loss accounting;
- the accepted session schema, graph topology/PDC/reduction rules, issue-008 SIMD ownership and
  issue-032 four-rate launch tier.

Checkpoint `0627618` is required: production preparation evaluates the cast-coefficient
state-space cutoff response and rejects outside `-3.0102999566 +/- 0.005 dB`; the complete snapped
analytic probes, DC/Nyquist limits, dense monotonicity, impulse tail/partition evidence and full
three-column sustained fit remain enabled.

Complete parameter metadata must describe, rather than imply, its domain. Replace ambiguous
`per_lane: bool` and an infinite HPF/LPF maximum with stable scope/domain values that encode:

- `PerLane` for polarity, trim, HPF, LPF, fader and mute; `MatrixShared` for four coefficients;
- Boolean, decibel-amplitude, hertz and linear mappings explicitly;
- `0 Hz` disabled, otherwise `10 Hz <= f < current_sample_rate/2` for filters;
- prepared-only versus block-target update rate, reset behavior and matrix linear-N smoothing.

No public meter/processor adapter may discard an error. Either make graph observation return and
propagate a bounded `RenderError`, or expose an infallible internal observation method whose shape
and end time are proven by the already validated render envelope. A bare `let _ = observe(...)`
is not accepted.

## Sealed prepared-builtin artifact

`PreparedBuiltinsSession` is opaque outside `miso-engine-builtins-compiler`: its session seal,
processors, observers, consumers, tails and resource report are private. Read-only inspection and
one consuming graph-lowering method are allowed; callers cannot construct or mutate the artifact.
The consuming parts type cannot be converted back into a prepared-builtin artifact.

The immutable seal contains:

1. SHA-256 of canonical session TOML, sample rate and quantum;
2. sorted exact track IDs;
3. exactly three `(track ID, stage)` processor identities per track;
4. exact `(track ID, BuiltinTail)` values, recomputable from enabled session filters;
5. sorted exact `(meter handle, track ID, tap, reset generation, period, hold, decay bits,
   logical queue capacity)` requests;
6. exact observer `(node, handle)` and consumer `(handle, track ID, tap)` identities; and
7. the checked resource report described below.

Graph compilation recomputes this expected seal from the same effect-prepared session and compares
all values before consuming anything. Unknown tracks, duplicate handles/requests, missing/extra
processors, a changed tail value, changed observer node, changed consumer metadata, session/rate/
quantum mismatch, or resource-report mismatch returns one sorted typed diagnostic and both complete
prepared inputs. Concrete processor/observer boxes can originate only from the private compiler
constructor. The generic graph attachment seam must not let an ordinary caller masquerade an
arbitrary processor vector as the sealed builtin artifact.

Tests prove compile-time opacity plus transactional rejection for every independently corruptible
seal field through a dedicated test-only corruption constructor owned by the compiler crate. Do
not expose a production tamper method merely to make the test possible.

## Exact builtin resource metric and cap preflight

Resource fields are renamed/documented as **engine-owned retained payload bytes**. They are not
RSS and do not claim allocator-private metadata. Exactness means the sum of the stable Rust layout
sizes and requested capacities that this artifact retains, computed with checked arithmetic before
the first issue-007 payload allocation. `maximum_single_allocation_bytes` is the largest requested
engine payload among those same owned allocations. Allocator headers, page rounding and unrelated
session/effect artifacts are explicitly excluded rather than guessed.

The prepared artifact retains a compact session seal, not a deep `CompiledSession` clone. Count
exactly:

- each concrete input, fader and matrix processor box payload;
- processor/observer/consumer/tail/seal vector capacities at their actual element layouts;
- every retained stable-ID/string byte payload (prefer exact `Box<str>` where practical);
- each meter observer box, producer and consumer endpoint payload;
- each SPSC ring logical header and its exact `capacity + 1` slot payload, using a checked resource
  helper in `miso-engine-core` shared by preflight and queue construction; and
- all alignment padding included by the engine-owned `Layout` formulas.

Do not count the transient `BuiltinChain` after it has been split, and do not double-count a box
payload through both its concrete value and trait-object pointer. Every `usize/u64/isize`, add,
multiply, capacity and three-times-track conversion uses checked arithmetic; no cast, saturation
or `unwrap_or(u64::MAX)` substitutes for `builtin.resource.arithmetic_overflow`.

Preparation is two phase. Phase 1 validates domains, exact counts/layouts and all caps without
allocating processor, meter queue or artifact payloads. Any failure returns sorted diagnostics and
allocates none of those payloads. Phase 2 performs only allocations already covered by the report;
fallible queue construction remains transactional. A test-only tracking allocator records each
phase-2 requested layout and proves total owned payload and largest request match the report. It
also proves a cap one byte below each independent total/largest boundary rejects in phase 1.

Retain the 65,537-track success/configured-resource case with zero meters and a configured subset.
No compiled track or meter ceiling is introduced.

## Complete versioned expected-output fixtures

`fixtures/builtins/v1/MANIFEST.tsv` remains sorted exact-length lowercase SHA-256, but every matrix
entry must resolve to checked-in expected data. A declaration saying a case exists is not an
expected-output fixture. Use these version-1 formats:

- `cases.toml`: canonical sorted case IDs and the fully expanded rate/quantum/section/cutoff/probe,
  gain/matrix/ramp, meter, graph and resource tuple for each case;
- `pcm/<case>.f32le`: headerless little-endian `f32` lane samples in declared planar L-then-R
  order for bit-exact identity/gain/mute/matrix/ramp/reset/partition and graph-tap cases;
- `reference/filter-response.csv`: sorted rows containing case/rate/section/cutoff/probe/quantum,
  independent f64 RBJ magnitude, cast-state analytic magnitude, impulse-DFT magnitude, sustained
  fundamental/residual/total metrics, tail energy and recovery count. Decimal f64 values use 17
  significant digits;
- `meters/<case>.jsonl`: one canonical object per expected snapshot; every f32/f64 numeric result
  is stored by lowercase fixed-width IEEE bit hex alongside exact identities, times and counters;
- `diagnostics.jsonl`: sorted invalid domain/block/meter/resource/seal cases with exact code/path;
  and
- `resources.jsonl`: exact retained-payload totals, largest payload and allocation-count breakdown
  for the pinned native fixture ABI, including 1/4/65,537 tracks and 0/1/7 meter sets.

The manifest includes all of those files. The generator writes only to a caller-supplied scratch
directory. `--check` never rewrites the repository and verifies manifest bytes, complete Cartesian
coverage and every referenced file. Independent response values come from
`miso-engine-dsp-reference`, which cannot depend on production builtins. Golden production PCM is
compared against checked-in bits; it is never regenerated into the expected directory during a
test and then compared with itself.

Required coverage is the recurrence brief's complete four-rate/five-quantum single-section and
cascade grids; gain/polarity/fader/mute/asymmetric/signed-zero cases; all 16 matrix corners,
pan/balance endpoints/centers, N `0/1/2/127/128/u32::MAX` bounded-prefix ramps, every block split
and mid-ramp retarget; sanitization/recovery/reset/LR isolation; all seven graph taps with
distinguishable stage values and PDC coexistence; meter partial/multiple/wrap/full/drain/drop/
discontinuity/reset/overflow cases; exact diagnostic paths; and resource boundaries. Mutation
tests delete, alter, add and coverage-hole one artifact of every format and must fail.

Run 10,000 deterministic compiler mutations spanning valid/invalid parameters, meter requests,
matrix targets, block shapes/times and every cap. The test records success or exact typed failure,
never partial success, panic, timeout or a payload allocation beyond the accepted report.

## Graph-backed realtime and swap audit

Retain a direct scalar-kernel audit for ramp/retarget, sanitization, recovery/reset and meter queue
success/full paths. Add a separate graph-backed audit using production APIs:

1. compile an accepted session/effect artifact and sealed builtins with seven meters;
2. bind only the genuine external source and output nodes;
3. render through `PreparedRenderPlan`/`RealtimePlanOwner`, not a direct `BuiltinChain`;
4. use distinguishable deterministic values at all seven boundaries and drain/verify every meter
   off render;
5. prepare replacement plans off render and prove one block-boundary applied swap plus a pending
   replacement deferred when the bounded retirement queue is full; and
6. reclaim and destroy every displaced plan only after the render marker on the retirement owner.

The allocation/audit-hook and `strace` marker covers exactly 1,000,000 graph renders plus the
render-side applied/deferred swap entries. Publication, consumer draining and retirement happen
outside render audit scope. The trace reports zero allocation, deallocation, lock, log, file I/O,
network I/O, syscall, feature detection, panic/unwind or total forbidden operations. It records
render counts by plan/epoch, exactly seven observer windows per drained test block, queue-success
and queue-full counts, one applied swap, at least one retirement-full deferral, stable backing
addresses and off-render reclamation. Existing deliberate detector probes remain required.

An end-to-end graph test compares rendered tap snapshots and output PCM with the checked fixtures;
preparation-only node counts do not satisfy this gate.

## Fixed exactly-once benchmark workload

After every machine-verifiable nonbenchmark gate in this brief passes, root Sol may invoke
`scripts/run-builtins-benchmark.sh` exactly once. The runner refuses arguments and existing raw or
accepted artifacts. Failure preserves raw bytes plus validator reason/hash and is not retried.

The binary executes exactly two internal rounds and the Cartesian product of rates
`{48000,96000}` with these five workload kinds at quantum 128:

1. `full_chain_filters`: one track, asymmetric dual-mono 100/200-Hz HPF and 1/2-kHz LPF, fixed
   trim/fader and non-diagonal matrix, continuous state;
2. `identity_chain`: one exact identity track;
3. `matrix_ramp`: one track alternating two declared matrices, one complete 128-update ramp per
   operation;
4. `meter_success_full`: all seven taps, with one set drained for a successful snapshot every
   operation and one prefilled set exercising queue-full/drop; and
5. `prepare_256_tracks`: off-render preparation of one deterministic precompiled 256-track
   session with all seven taps on exactly eight named tracks and logical queue capacity four.

Each render workload uses 64 warm-up batches and 512 measured batches of eight operations. The
preparation workload uses 16 warm-ups and 128 measured single preparations; its timestamp ends
before off-timer destruction. Samples, targets and preparation inputs are fixed by fixture IDs.
Timing is descriptive and uses nearest-rank percentiles over batch nanoseconds divided by
operations, with `min/p50/p95/p99/p99_9/max` ordered. There is no speed threshold.

Every one of the exactly 20 schema-v2 JSONL records contains:

- `schema_version`, issue, workload kind, stable workload ID, rate, quantum and round;
- warm-up/measured batch and operations-per-batch counts;
- frames, tracks, meters, queue capacity and retained payload bytes where applicable;
- percentile method and integer `min/p50/p95/p99/p99_9/max` nanoseconds;
- fixture-manifest SHA-256, input-fixture ID/SHA-256 and deterministic output SHA-256;
- render allocations, deallocations and each forbidden-operation count (all zero; the preparation
  workload explicitly labels these as non-render/not-applicable rather than inventing a render
  claim);
- CPU model/architecture/core counts, OS/kernel, governor or power mode, Rust/LLVM, target triple,
  target features, profile, opt level, LTO, codegen units and background-load note; and
- a sorted `missing_metadata` list for anything the runner cannot discover.

The shell runner gathers metadata before launching the binary and passes it through fixed
environment variables. The validator requires exactly two rounds for every workload/rate pair,
the frozen observation counts, ordered percentiles, stable input/output identities across rounds,
zero render errors/forbidden counts and honest missing metadata. Validator mutation tests cover
every required field, every workload/rate/cardinality error and output/fixture mismatch. Preflight
may build and hash the binary and validate synthetic records but must report `workload_launches=0`.

## Listening hand-off and launch semantics

Keep both existing preregistrations honest and unchanged except for filling the accepted candidate
artifact identities after the sole benchmark. Issue 007 passes attempt 3 only as machine-qualified
when its machine gates and benchmark pass. Its report must say: `human_listening_status=pending`,
name the follow-up issue, and make no audible-quality or release claim.

The follow-up runs the preregistered real 20-trial filter ABX and 20-trial randomized matrix-ramp
procedure against the exact candidate commit, fixture hashes and render hashes. Synthetic rows,
agent-generated listener identities or a preregistration-only status fail it. End-to-end release
issue 026 depends on that result. An incomplete/adverse record blocks launch; an adverse result
creates a new DSP correction issue and invalidates qualification of any changed candidate.

## Ordered machine acceptance and benchmark authorization

Before benchmark authorization, all of the following pass in this order:

1. focused builtin/compiler/graph tests, complete independent response and fixture checks, 10,000
   mutations, opacity/corruption/resource-boundary tests and 65,537-track scale;
2. direct and graph-backed million-render/swap audits, syscall trace and all detector probes;
3. native pinned scalar release execution, Android/iOS pure-Rust release checks, Wasm `-simd128`
   and `+simd128` scalar-semantic builds;
4. locked workspace tests, warning-denied all-target Clippy and rustdoc, formatting and every
   workspace/realtime/research/graph/builtin policy plus mutation suite;
5. fixture, audit, target and zero-launch benchmark preflight artifacts sealed to the same clean
   candidate; and
6. Sol adversarial review finds no unresolved machine gate.

Only then may root Sol authorize the one runner invocation. Listening is deliberately after the
sealed machine candidate and benchmark and remains a separate hard release gate.

## Pass/fail

Attempt 3 passes only if all machine gates and the sole validator-valid 20-record benchmark pass.
The result is **machine-qualified, human-listening pending**. It is not launch qualification and
does not satisfy issue 026 without the follow-up.

If any machine gate fails, or the sole benchmark fails validation, stop issue 007 after attempt 3.
Preserve evidence and create a new stateless corrective issue; do not retry the benchmark or amend
this brief to make observed output pass.
