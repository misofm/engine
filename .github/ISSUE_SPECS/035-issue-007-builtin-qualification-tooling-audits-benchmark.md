# 035 Issue-007 builtin qualification tooling, audits, and benchmark

## Final status — 2026-08-21

**STOPPED / RESCOPED — NO OVERALL PASS.** Terra attempt 1 and the bounded Sol correction were
consumed at fixture-contract checkpoint `0edc51c6ff60aa8f4a31df73cf73bc2b52e4436e`.
That checkpoint is accepted technical input only; it does not prove independent corpus provenance,
realtime audits, targets or benchmark qualification. No benchmark/workload ran:
`workload_invocations=0`, `timed_benchmark_invocations=0`.

Remaining work moves without hidden state through this exact chain:

1. **Complete independent builtin corpus and corruption proof**;
2. **Builtin direct and graph realtime audit and target qualification**; then
3. **Builtin benchmark preflight and exactly-once qualification**.

Real human listening in Issue 033 remains after the final machine-qualified candidate. Nothing in
this rescope relabels Issue 035 PASS or authorizes its former benchmark command.

## Outcome

Produce the complete independent expected-output corpus, exact direct/graph realtime audits,
cross-target evidence and one validator-valid descriptive benchmark needed to machine-qualify the
corrected builtin candidate, while keeping real human listening as a separate launch gate.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy,
benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated
`PreparedRenderPlan`: graph, schedule and capacities are immutable while DSP state mutates only
under exclusive render ownership. Render performs zero allocation/free, locks, file/network I/O,
logging, syscalls, feature detection, panic/unwind, structural plan mutation or data-dependent
unbounded work. Displaced plans enter a bounded retirement queue and are reclaimed off render; a
full queue defers rather than drops a swap. There is no compiled track or meter ceiling. Audio is
planar `f32`; L/R parameters and state remain independent unless an explicit link mode or smoothed
2x2 matrix says otherwise. Launch session/render rates are exactly 44,100, 48,000, 88,200 and
96,000 Hz, with no implicit SRC. Output is PCM.

Issue 007 stopped after three attempts. Its retained conditioned incremental all-`f32` TPT
recurrence, scalar builtin chain, matrix/pan behavior and seven meter-tap semantics are reusable
technical input, not machine qualification. **Representable TPT cutoff domain and builtin
contract acceptance** accepts the corrected rate-keyed cutoff metadata, sealed-only graph
attachment, exact checked resource accounting and builtin compiler mutation gate. This issue then owns every remaining
machine-qualification deliverable and is the only issue that may eventually authorize the
builtin timed workload. Issue **Issue-007 builtin filter and matrix human listening
qualification** runs only against this issue's sealed accepted candidate and remains required for
launch.

This corrective workflow has at most **two total implementation attempts**: Terra attempt 1,
then one bounded Sol correction/review attempt. A second failure stops and requires a new
rescope/rebrief; do not weaken a threshold, regenerate expected output from the candidate under
test, retry a benchmark, or silently substitute a different candidate.

Current timed benchmark invocation count is **0**. Building the tool, hashing it and validating
synthetic records are permitted only when the preflight reports `workload_launches=0`. No timed
workload may run until every nonbenchmark gate below passes on one clean committed candidate and
root Sol explicitly authorizes the exact command `bash scripts/run-builtins-benchmark.sh`.

## Scope

- Replace fixture declarations with a complete versioned corpus of independently derived response
  values and checked production expected outputs, including exhaustive corruption/coverage tests.
- Qualify direct builtin behavior and the graph-backed prepared-plan render/swap/retirement path
  for exactly one million calls, with complete forbidden-operation detection and exact outputs.
- Run the native/mobile/Wasm and repository nonbenchmark gates against one sealed candidate.
- Implement the exact schema-v2 five-workload benchmark binary, safe exactly-once shell runner,
  aggregate/record validators and exhaustive validator mutations.
- If and only if Sol authorizes after all nonbenchmark gates, execute one external invocation that
  performs exactly two internal rounds and preserve its raw bytes and disposition.

## Required public interfaces/contracts

`miso_engine_builtins_fixture --write SCRATCH_DIRECTORY` writes a complete candidate corpus only
to the explicit scratch root; `--check FIXTURE_DIRECTORY` is read-only and validates the complete
manifest/coverage/expected-output contract. The direct and graph audit binaries emit canonical
checksummed records and accept only their frozen one-million-call configuration. The benchmark
binary emits only the 20 schema-v2 JSONL records defined below and is never a general CLI. The
no-argument `scripts/run-builtins-benchmark.sh` is the sole workload entrypoint and owns raw,
validation and accepted-artifact lifecycle. All failures are typed/nonzero and preserve evidence;
no interface silently regenerates goldens, retries work or discards an observation/render error.

## Retained DSP/functional oracle contract

Production DSP semantics are out of scope for modification. HPF/LPF are second-order Butterworth
(`Q=1/sqrt(2)`) TPT sections designed in `f64`, then use stored `f32 c1/a2/a3/k`, two independent
`f32` integrator states per enabled filter/lane and the retained non-fused incremental operation
order. Preparation rejects a cast-state cutoff response outside
`-3.0102999566 +/- 0.005 dB`. Reset/recovery sets both state words to positive zero. Enabled
cutoff is `10 Hz <= f <= maximum_hz(sample_rate)` under issue 036's exact rate-keyed table; exact zero disables. Builtin latency is zero and an
enabled filter has infinite tail.

The independent `miso-engine-dsp-reference` oracle cannot depend on production builtins. At all
four launch rates, analytic cast-state and one-second impulse-DFT magnitude agree with the
independent `f64` RBJ response within `0.005 dB` and `0.05 dB` respectively wherever reference
magnitude is at least `-120 dB`. Coherent amplitude-0.5 sines settle for `sample_rate/2` frames
and measure `sample_rate/4` frames with a full `f64` DC/sine/cosine least-squares fit. Where
reference gain is at least `-90 dB`, fundamental error is at most `0.05 dB` and non-fundamental
residual is at most `-100 dB` relative to input RMS; below `-90 dB`, total production gain is at
most `-88 dB`. DC/Nyquist limits, dense monotonicity, finite state, one-second final-4096-frame
tail energy, exact block partitions, sanitization, pairwise recovery/reset and L/R isolation are
all checked.

## Complete versioned expected-output fixtures

`fixtures/builtins/v1/MANIFEST.tsv` is sorted and contains exact relative paths, exact byte lengths
and lowercase 64-hex SHA-256 values for every file. Every case resolves to checked-in expected
bytes; prose saying a case exists is not evidence. The complete formats are:

- `cases.toml`: canonical sorted case IDs with fully expanded rate, quantum, section, cutoff,
  probe, gain, matrix, ramp, block split/retarget, meter, graph/PDC and resource tuples;
- `benchmark/<kind>-<rate>.toml`: ten fully expanded workload/rate input bundles, including exact
  parameters/targets/session identities and referenced PCM paths/hashes;
- `pcm/<case>.f32le`: headerless little-endian `f32`, planar L then R, for bit-exact identity,
  gain, polarity, fader, mute, matrix/pan/balance, ramp/retarget, signed zero, partition,
  sanitization/recovery/reset, L/R isolation, graph taps and output;
- `reference/filter-response.csv`: sorted case/rate/section/cutoff/probe/quantum rows with the
  independent `f64` RBJ magnitude, cast-state analytic magnitude, impulse-DFT magnitude,
  sustained fundamental/residual/total metrics, tail energy and recovery count. Decimal `f64`
  fields use exactly 17 significant digits;
- `meters/<case>.jsonl`: one canonical object per expected snapshot, with all `f32`/`f64` results
  represented by lowercase fixed-width IEEE bit hex plus identities, sample times and counters;
- `diagnostics.jsonl`: sorted invalid parameter/domain, block/time, meter, resource and all eight
  seal-category cases with exact code/path; and
- `resources.jsonl`: exact retained-payload total, maximum request and allocation layout/count
  breakdown for the pinned native fixture ABI over tracks `{1,4,65537}` by meter-set counts
  `{0,1,7}` at logical capacity four.

Response coverage is the Cartesian product of rates `{44100,48000,88200,96000}` and quanta
`{1,127,128,255,1024}`. For each HPF and LPF separately, cutoffs are
`{10,20,100,1000,min(20000,0.1*rate),0.45*rate}` Hz after exact-bit deduplication. Each section's
probes are `{0.25*f,f,4*f,0.2*rate,0.45*rate}`, clipped strictly below Nyquist, snapped to the
nearest 4-Hz coherent bin and deduplicated; analytic rows additionally include exact cutoff and
`0.49*rate`. Every cascade is fixed—not derived from the single-section cutoff—to a 100-Hz HPF
followed by a 1-kHz LPF in production order, and uses the sorted deduplicated union of those two
sections' probe sets.

Functional coverage includes asymmetric lanes, polarity/trim/fader/mute, signed-zero identity,
all 16 matrix coefficient corners, constant-power pan and cosine-balance endpoints and centers,
N-update ramps for `N={0,1,2,127,128,u32::MAX}`, every declared block split and mid-ramp retarget,
builtin input/fader/filter/matrix sanitization, pairwise recovery and reset PCM, seven graph taps
with distinct values, exact output and PDC coexistence. Meter coverage includes partial/multiple
windows, wrap, full/drop, interleaved drain, discontinuity, both reset modes, time/counter overflow,
hold/decay and sanitization with exact interval/cumulative/loss semantics.

The generator writes only to a caller-supplied scratch directory. `--check` is read-only and
verifies manifest grammar/bytes, fully expanded Cartesian coverage and every reference. Golden
production PCM is never generated into the expected directory during its comparison. For each of
the six format classes TOML (including cases and benchmark bundles), `f32le`, CSV, meter JSONL,
diagnostics JSONL and resources JSONL,
tests independently delete, byte-alter, add and create a manifest-valid coverage hole; all 24
mutations fail. Manifest deletion, byte mutation, duplicate/unsorted entries, wrong length/hash,
unsafe path and unlisted payload also fail.

## Direct and graph-backed realtime audits

The direct scalar audit makes exactly 1,000,000 process calls at 48 kHz/128 frames through the
production builtin API. Its fixed early schedule starts a 257-update matrix ramp, retargets it at
the next block boundary while updates remain, injects nonfinite input/target cases, exercises
pairwise filter-state recovery and both reset paths, then continues deterministic steady state.
Two meter sets exercise successful drain and queue-full/drop across all seven taps. Exact PCM,
matrix state, recovery/sanitization/reset and meter counter results are compared with checked
fixtures, not only with call counts.

The separate graph audit uses production compilation and render ownership APIs. It compiles the
canonical accepted session/effect artifact and sealed builtins with seven meters, binds only the
genuine external source/output plus fixture processors required by the declared rack/PDC graph,
and renders through `PreparedRenderPlan`/`RealtimePlanOwner`; it never calls `BuiltinChain`
directly. Deterministic nonidentity rack stages make input, post-input, post-SIMD1, post-dynamic,
post-SIMD2/pre-fader, post-fader and post-matrix values pairwise distinguishable. A fixed positive-
latency side route proves exact integer PDC coexistence. Every drained snapshot and output PCM is
checked against fixtures.

Exactly 1,000,000 graph renders plus render-side swap decisions occur between audit markers. Plan
A renders block 1. Plan B applies at the next block boundary and fills a capacity-one retirement
queue with A. A pending Plan C is then deferred while the queue is full, so B renders blocks
2–1,000,000. The record therefore has exactly one applied swap, at least one explicit
retirement-full deferral, render counts by `(plan_id, epoch)` of A=1 and B=999,999, stable backing
addresses, and no render-thread destruction. After the closing marker, the retirement owner
reclaims/destroys A and the control owner disposes of never-applied C; destruction markers prove
both occur off render and after their applicable render markers.

Both audits count allocation, deallocation, lock, log, file I/O, network I/O, syscall, feature
detection and panic/unwind separately, plus their exact total. Every count is zero. Marker-delimited
native `strace` finds no syscall. There is one deliberate terminating detector probe for each of
the nine categories, and the probe driver fails if a detector returns normally. Publication,
meter draining and retirement happen outside the marked scope.

## Target and repository qualification

One candidate and fixture manifest must pass native debug and pinned scalar release focused tests;
the independent response/fixture suite; end-to-end graph tap/PDC tests; both million-call audits,
trace and all probes; native scalar release execution; `aarch64-linux-android` and
`aarch64-apple-ios` pure-Rust release checks; and `wasm32-unknown-unknown` scalar-semantic builds
with both `-simd128` and `+simd128`. Then run locked workspace tests, warning-denied all-target
Clippy and rustdoc, formatting and every workspace/realtime/research/graph/builtin policy and
mutation suite. Cross-compilation is not a device-listening claim.

## Exact schema-v2 benchmark workload

The benchmark binary executes exactly two internal rounds and the Cartesian product of rates
`{48000,96000}` with these five workload kinds at quantum 128, producing exactly 20 JSONL records:

1. `full_chain_filters`: one track, asymmetric dual-mono 100/200-Hz HPF and 1/2-kHz LPF, fixed
   trim/fader, a non-diagonal matrix and continuous state;
2. `identity_chain`: one exact identity track;
3. `matrix_ramp`: one track alternating two fixture-declared matrices, with one complete
   128-update ramp per operation;
4. `meter_success_full`: one graph track with two meter sets across all seven distinct taps; one
   logical-capacity-one set is drained every operation and one is prefilled to exercise exact
   queue-full/drop behavior; and
5. `prepare_256_tracks`: off-render preparation of one deterministic precompiled 256-track
   session with all seven taps on exactly eight named tracks and logical queue capacity four.

Each render workload performs 64 untimed warm-up batches and 512 measured batches of eight
operations; it records 4,096 observations after dividing each measured batch's integer
nanoseconds by eight. Each operation advances its sample time by exactly 128 frames. Preparation
uses 16 untimed warm-ups and 128 measured single preparations; its timestamp ends before
off-timer destruction. Each round begins from the same declared state. Timing is descriptive,
has no speed threshold, and uses integer nanoseconds per operation with nearest-rank
`min/p50/p95/p99/p99_9/max`.

Every record has exactly these schema-v2 identities/shapes: `schema_version=2`, `issue=35`,
`workload_kind`, `workload_id`, `sample_rate_hz`, `quantum_frames=128`, `round`, `render_scope`,
`warmup_batches`, `measured_batches`, `operations_per_batch`, `total_operations`,
`frames_per_operation`, `tracks`, `meter_observers`, `meter_queue_capacity`,
`retained_payload_bytes`, `percentile_method="nearest_rank"`, `units="ns_per_operation"`, the six
integer percentile fields and `descriptive_only=true`. Stable workload IDs are exactly
`issue035.<kind>.<rate>hz.q128` and are validator-bound to kind/rate. Render records use
`frames_per_operation=128`; preparation uses JSON null. The first three workloads have zero meter
observers and null queue capacity; `meter_success_full` has 14/capacity 1; preparation has 256
tracks, 56 observers/capacity 4.

Every record also contains `candidate_commit`, `binary_sha256`,
`fixture_manifest_id="fixtures/builtins/v1/MANIFEST.tsv"`, `fixture_manifest_sha256`, a
workload/rate-specific `input_fixture_id` and `input_fixture_sha256`, and deterministic
`output_sha256`. Each input ID is exactly
`fixtures/builtins/v1/benchmark/<kind>-<rate>.toml`; that manifest-listed file is fully expanded
and self-contained except for referenced PCM whose path and SHA-256 it records. Its manifest hash
is `input_fixture_sha256`; the manifest itself is never a generic input substitute. Render output
hashes cover the canonical concatenation of all measured planar PCM and, for the meter workload,
all emitted snapshots/counters. Preparation output hashes the canonical address-free seal/resource
projection (track/tail/meter identities, counts, layout breakdown, total and largest request).
Candidate, binary and manifest identities are identical across all records; input and output
identities are stable across the two rounds of each workload/rate.

Render records contain integer-zero `render_errors`, `render_allocations`,
`render_deallocations`, `render_locks`, `render_logs`, `render_file_io`, `render_network_io`,
`render_syscalls`, `render_feature_detection`, `render_panic_unwind` and
`render_total_forbidden_operations`, with total equal to the sum of the nine operation categories.
Preparation labels every render field with the exact string `not_applicable` and makes no render
claim.

Metadata fields are `cpu_model`, `cpu_architecture`, `logical_core_count`,
`physical_core_count`, `os`, `kernel`, `governor_or_power_mode`, `rust_version`, `llvm_version`,
`target_triple`, `target_features`, `profile`, `opt_level`, `lto`, `codegen_units` and
`background_load_note`. Discovered values use their correct JSON string/integer type;
undiscoverable values are JSON null. `missing_metadata` is the sorted unique exact list of null
metadata field names. Sentinels such as `unknown`, `default` or empty strings do not count as
discovered metadata.

The record validator enforces the exact required key set/types, workload-ID binding, per-workload
shape, rates/rounds/counts, observation counts, ordered percentiles, identities, null/missing
metadata equivalence and audit totals. The aggregate validator requires exactly one record for
each of 5 kinds x 2 rates x 2 rounds, stable candidate/binary/manifest and stable per-pair
input/output hashes. Mutation tests cover every required field and type, each workload/rate/round/
cardinality/duplicate omission, every workload-specific shape, unordered percentiles/missing
metadata, dishonest metadata, nonzero/mistotaled audit counts, fixture/input/output mismatch and
cross-round output drift. Synthetic validator tests never launch the workload.

## Safe exactly-once runner and benchmark ownership

`scripts/run-builtins-benchmark.sh` uses `set -euo pipefail`, accepts no arguments, resolves the
repository from its own quoted `BASH_SOURCE`, checks dependencies before workload launch, uses no
`eval`, sourced input, unsafe word splitting or user-derived command text, and passes precollected
metadata through fixed environment variables. It refuses before launch if any raw, accepted or
disposition artifact already exists: `target/issue35/builtins-benchmark.raw.jsonl`,
`target/issue35/builtins-benchmark.jsonl`,
`target/issue35/builtins-benchmark.validator.stderr` or
`target/issue35/builtins-benchmark.disposition.json`.

The runner contains exactly one workload launch. It writes stdout directly to a newly created raw
JSONL path and never edits or deletes those bytes. Workload failure or validation failure
preserves the raw artifact and writes a separate checksummed disposition containing exit status,
validator stderr/reason, validator/tool hash and raw SHA-256; it never promotes failure. Success
also preserves raw bytes and creates an atomic byte-identical accepted copy plus PASS disposition.
It refuses overwrite and has no retry/resume/tuning option. Signal/interruption leaves whatever
raw bytes were produced and cannot authorize another invocation.

Only this issue's root Sol may authorize exactly one external invocation, and only after all
nonbenchmark gates pass on the final committed tree. The one invocation internally runs two
rounds. No other issue, agent or helper may invoke the binary directly or call the runner. A
workload or validation failure consumes the authorization and fails this attempt; it is not
retried.

## Deliverables

- complete checked fixture corpus, independent oracle outputs and corruption/coverage suite;
- exact direct and graph million-call audit binaries, trace gates and nine detector probes;
- target/repository qualification scripts and checksummed nonbenchmark evidence;
- exact schema-v2 benchmark binary, record/aggregate validators, exhaustive mutations, safe
  runner and zero-launch preflight; and
- after sole authorization only, immutable raw/accepted/disposition artifacts and a machine-
  qualification decision record.

## Explicit non-goals

Changing DSP coefficients/recurrence/thresholds, adding SIMD kernels, certifying loudness/true
peak, changing session/graph/PDC semantics, setting a performance threshold, retrying/tuning a
benchmark, executing real listening, creating synthetic listeners, or making a launch claim.

## Dependencies by exact issue title

- Representable TPT cutoff domain and builtin contract acceptance
- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral

## Sol implementation brief (2026-08-21)

**READY FOR TERRA ATTEMPT 1 ONLY AFTER ISSUE 036 PASSES.** The tracked authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/035-issue-007-builtin-qualification-tooling-audits-benchmark.md`. It
freezes the two-attempt budget, complete expected-output/corruption grid, exact direct/graph
million-call audits, target sequence and schema-v2 binary/runner/validator. This issue alone may
eventually receive one root-Sol authorization for `bash scripts/run-builtins-benchmark.sh`, which
runs two internal rounds; current invocation count is 0.

## Hazards/decisions

Checked-in expected output must be independent of the comparison run; a generator and production
implementation agreeing with themselves is not conformance. Batch timings must be normalized per
operation. Meter taps must observe distinct production graph boundaries, not fourteen copies of
one post-chain buffer. The graph audit must prove render values and lifecycle, not infer them from
prepared node counts. Metadata gaps are honest nullable data, never favorable defaults. Because
the benchmark authorization is nonrenewable, preflight must exercise every shell and validator
path without starting the workload.

If a response, fixture or audit exposes a production defect, fail this issue and open a stateless
corrective implementation issue; do not modify the retained DSP under this tooling issue. Human
listening remains pending in issue 033 and cannot be replaced by machine evidence.

## Acceptance gates with objective measurements

All complete fixture grids and all 24 per-format mutations pass; independent response and golden
PCM/meter/diagnostic/resource comparisons pass at their frozen tolerances. Direct and graph audits
each complete exactly 1,000,000 calls/renders with exact fixture values, lifecycle/count records,
stable addresses, zero counts in all nine forbidden categories and zero total; marker-delimited
trace and all nine deliberate probes pass. The full target/repository matrix passes on one clean
candidate. The preflight seals candidate/binary/fixture/audit/target identities and reports
`workload_launches=0`.

Sol then performs a final adversarial nonbenchmark review. Only if no gate remains may root Sol
authorize the single command. Acceptance requires exactly 20 validator-valid schema-v2 records,
immutable matching raw/accepted bytes, a PASS disposition, two rounds for all ten workload/rate
pairs and no retry. The final state is **machine-qualified, human-listening pending**. It does not
satisfy issue 026 or permit an audible-quality/launch claim until issue 033 passes.

## Target matrix

Pinned native scalar debug/release execution; compile checks for `aarch64-linux-android` and
`aarch64-apple-ios`; `wasm32-unknown-unknown` with `-simd128` and `+simd128`. The sole descriptive
timed benchmark runs only on the one pinned native host recorded in its schema-v2 metadata.

## Required evidence

Manifest and fixture hashes; independent oracle provenance; Cartesian coverage report; all
corruption results; exact audit records/traces/probes; target/workspace logs; zero-launch preflight
with `timed_benchmark_invocations=0`; final Sol authorization; raw/accepted/disposition hashes and
validator/tool hashes if invoked; and `human_listening_status=pending` naming issue 033. If the
benchmark is not legitimately authorized or fails, record FAIL and do not retry.

## Fixture-contract checkpoint (2026-08-21)

Terra's first bounded tranche exposed a real test-contract defect: the initial manifest-valid
meter-JSONL coverage-hole mutation was accepted. Sol's bounded correction reduced the patch to the
read-only V1 fixture-contract boundary and made all 24 format-class delete/alter/add/coverage-hole
mutations reject. The checker now parses a strictly sorted safe manifest, verifies exact payload
bytes, rejects unlisted files, classifies the frozen required path set, and keeps `--write` limited
to its explicit scratch boundary while `--check` is read-only.

Focused evidence PASS: all three `miso-engine-builtins-fixture` tests, package formatting,
warning-denied all-target Clippy, and `git diff --check`. This is a recoverable tooling checkpoint,
not Issue-035 completion: independent expected-output provenance, the full checked corpus, direct
and graph audits, target/workspace qualification, schema-v2 benchmark readiness, final Sol review,
and any authorized benchmark remain unrun/incomplete. No benchmark or workload was invoked;
`timed_benchmark_invocations=0` and `workload_invocations=0`.

The broad remainder is intentionally not continued in this exhausted issue. Issue 056 owns only
the complete independent corpus; Issue 057 owns direct/graph audits and targets; Issue 058 owns
benchmark schema/preflight and any sole exactly-once run. Each successor has a fresh two-attempt
budget and a half-day-closable boundary.
