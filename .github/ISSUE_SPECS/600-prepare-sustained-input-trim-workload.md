# Prepare sustained input-trim workload for RT5 measurement

GitHub: https://github.com/misofm/engine/issues/600

Bounded final RT5 child of audit lane A #559 on delivered main
`51ba7023cdf3c16f93f7d153dc78c1880bfc6576`. #238/PR #486 already replaced per-record whole-bank
channel-symmetry refresh with addressed-lane refresh. #496/PR #508 already replaced the two
post-ramp whole-bank predicates with one shared whole-mask extraction. Their correctness and
mechanism proofs are delivered. This issue owns only the retained descriptive measurement and
closure accounting; it does not authorize another DSP optimization.

Luna HIGH or XHIGH implements the benchmark subject, validator, runner and untimed evidence. Astra
LOW performs every scope, source, harness, capture and exact-head verification. The Sol HIGH
coordinator owns this brief, exact-path checkpoints, the single timing authorization, GitHub
synchronization and delivery. Lane B #598 remains the only other active issue and owns disjoint lane
policy paths.

## Smallest closable outcome

Add a narrow native benchmark subject for one genuine W8 prepared-plan input-trim automation ride.
Immediately before every corresponding render call it must successfully enqueue one
`TrackInputRecord::TrimDb` with `BuiltinLaneSelector::Both` for each of eight populated tracks.
These queue records carry no sample timestamp, and `BuiltinBankProcessor::begin_block` does not use
`first_sample`; the eight records are drained at that render call's block boundary while the harness
records the absolute render sample. Targets alternate deterministically between -6 dB and -12 dB,
with `smoothing_samples = 256`, at 48 kHz and q128. Every render therefore drains eight accepted
records while the trim remains nonstationary.

Time only the prepared plan's render call. Record publication, deterministic PCM refill, output
hashing and evidence collection happen outside the measured interval. There is no baseline arm and
no before/after claim. One runner process performs one interleaved warmup phase of 512 blocks per
independently prepared owner, 1,024 total warmup render calls, followed by measured rounds 1 and 2 of
exactly 4,096 blocks per owner. Each owner maintains its own absolute-sample and target-alternation
continuity, both begin measurement in equivalent state, and warmup output is excluded from measured
hashes and traffic counts. The run reports the current delivered-plan render cost under this exact
workload.

## Frozen interpretation and accounting

The audit phrase “spills up to 240 SIMD registers” was a source-level characterization, not a
machine-code or cycle measurement. #496's 240-versus-30 evidence counts lane-word extractions at the
old and delivered post-ramp seams; it does not prove executed spills, isolated helper cycles or a
universal saving. #238's historical 8.46 versus 5.60 microseconds/block synthetic observation and
64-track extrapolation remain provenance only and cannot be promoted as current results.

The new records must state both measured-round values and describe them only as current native
x86-64-v3 render cost in nanoseconds per block for the frozen eight-record-per-block W8 trim ride.
They must not claim isolated channel-symmetry cost, a historical speedup, a universal extraction
saving, a percentage improvement, or a 64-track estimate. Effect-floor accounting is inapplicable to
this whole-plan bookkeeping workload: do not invent `isolated_cycles_per_lane_sample`, use a nominal
CPU frequency, or borrow an effect floor. Name the costs still combined in the measurement: queue
draining, trim ramp arithmetic, settle/report reads, graph execution and remaining required refresh
work.

Successful accepted capture plus upstream evidence closes RT5's retained measurement/accounting
obligation. It does not establish a release budget or authorize tuning from the descriptive number.

## Exact ownership

Allowed implementation paths:

- `tools/bench/src/input_symmetry.rs` (new);
- `tools/bench/src/main.rs` (subject registration only);
- `scripts/preflight-input-symmetry-benchmark.sh` (new);
- `scripts/run-input-symmetry-benchmark.sh` (new);
- `scripts/test-input-symmetry-benchmark.sh` (new);
- `scripts/input-symmetry-benchmark-validator.py` (new);
- this numbered spec;
- focused review records under `docs/audits/`;
- `artifacts/issue600-input-symmetry/` for separately named protected preflight, untimed and capture
  records;
- concise #559/#560 handoff status under root ownership.

Reuse `bench-support` timing, statistics, metadata, digest and allocation facilities; current
prepared graph/builtins binding patterns; `prepare_session_builtins_with_console`; actual
`TrackControlProducer.input`; and `TrackInputRecord::TrimDb`. Reuse the existing eight-track
`fixtures/session/v1/parametric-eq-bank-console.json` without changing it. The tools/bench package
already has the required dependencies; no manifest or lockfile change is authorized.

Do not edit `crates/builtins`, any other runtime crate, fixtures, existing benchmark subjects,
existing benchmark runners/validators, the consumed #431 capture, effect-floor tables, policies,
workflows, browser/SDK/ABI code, artifact pins or qualification matrices. Do not create a generic
benchmark framework. If the frozen outcome requires a runtime, manifest, fixture or existing-runner
change, stop and rebrief.

## Subject and untimed correctness contract

Prepare two independent runtime owners from the existing eight-track session. Prove the selected
native dispatch is W8 and all eight tracks are populated in the same eligible bank. Use the fixture's
existing source mapping and deterministic nonzero PCM. Preallocate every record, input/output buffer,
hash buffer and evidence counter needed before any render audit or timed interval.

For each block, outside timing, publish exactly eight records immediately before its render call;
every push must return success. Record the render call's absolute sample without treating it as queue
admission metadata. Alternate the same per-block target for all tracks between -6 dB and -12 dB and
keep `smoothing_samples = 256`. Refill deterministic PCM outside timing. Time the render call only,
then hash the completed output outside timing. Maintain checked counts for attempted and accepted
records, rendered blocks, render errors and output words.

Before timing is ever authorized, an untimed proof must establish:

1. the prepared plan contains the intended full W8 bank and eight addressable input producers;
2. all eight records per block are accepted and drained on the intended sample boundary;
3. a representative sequence remains nonstationary and produces deterministic nonzero PCM and
   trim/ramp state consistent across both independently prepared owners;
4. the two owners produce the same frozen traffic/output digest;
5. a no-record control differs at the same frozen traffic/output assertion;
6. every render succeeds, no accepted work remains, and render allocation/free counters stay zero;
7. the unchanged #238/#496 focused mechanism and PCM/state tests still pass in debug and release.

Add no production counter or public witness. Test-only observation must be private to the new
subject/tests or use existing public ownership/report surfaces without changing runtime code.

Run and restore one untimed direct mutation: suppress actual input-record pushes while retaining the
reported schedule. The same frozen accepted-traffic/output assertion used by the positive proof must
fail for the intended reason. The mutation must never enter the timed runner, and the source must be
restored before review.

## Harness and validator contract

The validator accepts exactly two measured records, rounds 1 and 2, and rejects missing, duplicate,
extra or reordered rounds. Each record must carry the frozen sample rate, quantum, lane width, track
count, records per block, target pair, smoothing length, warmup blocks, measured blocks, actual
attempted/accepted record counts, zero render errors, output digest, finite positive elapsed
nanoseconds and finite positive nanoseconds per block. It must require internally consistent checked
counts and the two independently prepared round identities.

Records also include source commit/tree, binary and fixture hashes, exact argv and cwd, effective
Rust/compiler/target/build flags, and CPU/OS metadata with explicit honest missing values. Preserve
raw stdout, stderr, validator stderr, accepted records and runner disposition with separate hashes,
byte counts and child/validator/runner statuses. A zero launcher exit alone is never PASS. Mutation
records cannot appear in accepted timing output.

The runner refuses overwrite of every protected output, verifies the preflight seal and all frozen
source/binary/fixture/validator identities before launch, propagates child failure, preserves raw
output on validator failure, publishes accepted output transactionally, and records shell exit
semantics. A post-workload tooling failure preserves raw evidence and becomes a separate repair or
promotion issue; it never authorizes rerunning this timed workload.

The preflight and harness self-test must exercise invalid arguments, valid and invalid schemas,
two-round completeness/order, fixture/source/binary identity mismatch, child failure propagation,
raw-output preservation, transactional accepted output and overwrite refusal using untimed stubs.
They must prove zero real benchmark-process, workload and timing invocations. Freeze the validator,
workload and runner hashes in the final preflight seal before the capture. Untimed source/harness
review writes its own protected filenames and does not create or overwrite that final seal or the
capture namespace.

## Workflow and objective gates

1. Astra LOW reviews this pushed stateless brief and current-base applicability before implementation.
2. Luna HIGH/XHIGH implements one coherent attempt. Root checkpoints each green exact-path tranche
   before more implementation. Astra LOW reviews the source and untimed harness evidence. At most
   three implementation attempts are allowed.
3. Before timing, pass the new subject unit tests and validator/runner lifecycle self-test,
   strict Clippy and rustdoc for tools/bench, formatting/diff checks, workspace/realtime/builtins/
   graph/lane policy gates, and the unchanged focused #238/#496 tests in debug and release. Record
   exact commands/statuses and restored mutation evidence.
4. Only after Astra LOW source/harness PASS may root authorize exactly these commands in order:
   `bash scripts/test-input-symmetry-benchmark.sh`,
   `bash scripts/preflight-input-symmetry-benchmark.sh`, then exactly once
   `bash scripts/run-input-symmetry-benchmark.sh`. The final command launches one benchmark process,
   with one internal warmup phase and two measured rounds. The first command revalidates the untimed
   harness without touching the final preflight/capture names; the second creates the sole final
   candidate seal; the third consumes it once. Do not tune or retry.
5. Astra LOW validates raw/accepted bytes, record schema, traffic/output discriminators, timing
   finiteness, metadata, statuses, hashes, invocation counts and the honest accounting statement. A
   valid descriptive result passes regardless of its magnitude because this issue sets no budget.
6. Merge current main, recheck exact-path disjointness with #598, obtain Astra LOW exact-head PASS,
   push once, run required PR qualification, merge, require successful post-main qualification,
   synchronize and close the numbered issue, mark RT5 delivered in #559/#560, verify remote states,
   and remove every clean delivered worktree.

A failed attempt may not weaken gates. After three failed implementation attempts, preserve evidence
and create a newly bounded successor. A benchmark-runner defect gets one bounded correction; if it
remains, preserve evidence and move repair/promotion to a tooling issue. No original open finding is
authorized until this partial barrier is delivered.

## Initial Astra LOW applicability review

Read-only review of current main found no remaining RT5 runtime repair: addressed setter refresh and
both shared post-ramp refreshes are delivered, while preparation/reset correctly retain full refresh.
Existing console traffic benchmarks drive EQ/compressor controls rather than the input trim queue,
and #431's timing authority is consumed. The retained measurement remains applicable through the
real input-record path. Astra LOW recommends this benchmark-only successor and rejects a
source-based inapplicability closure; a documentation-only disposition would require an explicit
owner ruling withdrawing #559's measurement request.

## Astra LOW scope review 1

Exact pushed brief `f9daa49eb9a1f57b78c67effcf27057c4fb8bdc7` received **FAIL** before any
implementation attempt. The overall benchmark-only shape is accepted, but the brief used the wrong
selector name and implied timestamped queue admission, left 512 warmup blocks ambiguous across two
owners, and reserved the artifact namespace only after capture despite required pre-capture
evidence. This correction uses `BuiltinLaneSelector::Both`, freezes immediate pre-render publication
and boundary drain semantics, requires 512 warmup blocks per owner with explicit continuity and
exclusion, and separates protected untimed/final-preflight/capture filenames. Formal rereview is
required before Luna implementation.

## Astra LOW scope review 2

Astra LOW returned **PASS** on exact pushed head
`b5fbabaa541240d65b714d358d9a16209be7b832`. The corrected brief now matches the actual selector
and queue-boundary contract, freezes 512 warmup blocks per owner with explicit continuity and
exclusion, and separates untimed, final-preflight and capture namespaces. The live GitHub issue and
local spec match; current main remains the inspected base; #598 ownership is disjoint. Luna HIGH or
XHIGH attempt 1 may implement the benchmark and untimed harness. Timing remains unauthorized until
the later source/harness review passes.

## Attempt 1 review

Implementation checkpoint `c735e5d5172b446fdc6c515784a24f70eeeed982` received Astra LOW
**FAIL**; two attempts remain and timing is not authorized. Safe validator/lifecycle, strict Clippy,
rustdoc, formatting/diff, and unchanged #238/#496 gates pass, but the real preflight build flags are
invalid; untimed proof calls the timing helper and the timer includes buffer construction; exact
drain/pending/ramp-state and independent digest witnesses are missing; the required restored source
mutation was replaced by a permanent suppression mode; validator type/duplicate-key/metadata/seal
checks are incomplete; runner persistence/status/cwd behavior is inaccurate; and lifecycle tests
miss source/binary and post-workload publication failures. Full verdict:
`docs/audits/600-attempt1-review.md`. Attempt 2 stays inside the six implementation paths and focused
records. No capture, final preflight, runtime change or path expansion is authorized.

## Attempt 2 review and runner circuit breaker

Correction checkpoint `000de31e30bacaabb68576be958a26b50fcaad21` received Astra LOW **FAIL**;
one implementation attempt remains and timing is not authorized. Untimed/timed separation, checked
arithmetic, capacity-one drain evidence, build flags/profile isolation, cwd, explicit raw failure
preservation, source/binary cases and five-launch accounting improved. Runtime ramp/nonstationarity
proof remains disconnected and self-derived; durable restored mutation evidence is absent; actual
two-round timing markers disagree with the one-invocation claim; strict seal validation remains
post-launch; backend and seal numeric types remain permissive; and failure dispositions still invent
validator/scratch/timing facts. Full verdict: `docs/audits/600-attempt2-review.md`.

The benchmark-runner defect survived its one bounded correction. Under the frozen circuit breaker,
remaining runner repair and capture promotion must move to a separate tooling successor. Before
attempt 3, rebrief #600 to a smallest closable subject/untimed-runtime proof and remove the
known-defective new runner paths from its delivered delta. Do not use the remaining attempt as a
second runner correction.

## Authoritative attempt 3 rebrief

This section supersedes the original measurement/runner outcome above for attempt 3 and delivery.
The earlier scope and two failed reviews remain immutable history. The runner defect survived its one
bounded correction, so #600 no longer owns timing, capture, preflight, validation or promotion.

The smallest closable capability is one reproducible, entirely untimed `bench input-symmetry`
qualification subject proving that the real prepared W8 plan processes the intended sustained input
trim queue traffic. #600 closes only this workload capability. RT5 remains partial for a later
numbered tooling successor that will own strict prelaunch validation, runner durability, record
validation, exactly one authorized capture, honest accounting and RT5 closure. Create that issue only
after #600 closes and releases this lane slot.

### Exact attempt 3 paths

Retain and modify only:

- `tools/bench/src/input_symmetry.rs`;
- the existing subject registration in `tools/bench/src/main.rs`;
- this renamed numbered spec;
- `docs/audits/600-attempt3-review.md` and final review records;
- `artifacts/issue600-input-symmetry/attempt3/` for untimed qualification and mutation evidence;
- concise #559/#560 handoff status under root ownership.

Delete the four new `scripts/*input-symmetry*` files from #600's delivered delta. Their failed
versions remain preserved in Git history and the attempt reviews. Do not replace them, edit an
existing runner, or add another harness. No runtime, dependency, manifest, lockfile, fixture,
existing benchmark, policy, workflow, browser/SDK/ABI, pin or matrix change is authorized.

### Untimed qualification contract

Remove `bench_support::timing`, every timer/helper, elapsed or nanosecond field, timing marker and
performance metadata from the subject. The command must never observe a clock or emit a performance
claim. Its concise deterministic qualification result identifies issue, workload, source fixture,
backend, counts and frozen digests only.

Retain 48 kHz, q128, native W8, eight populated/addressable tracks, immediate publication of eight
`TrimDb { lanes: BuiltinLaneSelector::Both, ... }` records before every render, alternating -6 dB
and -12 dB targets, and 256-sample smoothing. Use checked arithmetic throughout. Each of two
independently prepared owners executes 512 untimed preparation blocks, excluded from phase counts and
digests, then two 4,096-block qualification phases. This is 8,704 render calls per owner and 17,408
total. Preserve each owner's absolute-sample and alternation continuity. Call them qualification
phases, never measured rounds.

The production qualification must prove all publishes succeed, each boundary render succeeds, no
accepted record remains pending after the final tested boundary, output is nonzero, and the two
owners produce the exact reviewed phase digests. Capacity/refill drain probes must run on separate
representative witness owners so they do not add records to the frozen ride. The witness queue
capacity and inference must be stated accurately; the attempt-2 draft used capacity 16, not one.

Replace the disconnected arithmetic-only oracle. On a separate representative prepared runtime
owner, compare per-block PCM and exposed state/report observations for an alternating trim sequence
against an independently calculated trim-ramp expectation derived from the delivered update rule and
the deterministic source words. The expected calculation must not call the runtime setter/kernel
under test. Cover successive retargets before the prior 256-sample ramp settles, at least three
boundaries, both channels, nonzero samples, target alternation and continuing nonstationarity. A
no-record owner must differ on the same connected assertion. If current public surfaces cannot
connect the independent expectation to actual runtime PCM/state without a runtime edit, stop; that is
a scope blocker.

Freeze complete qualification-phase digest constants only after the connected oracle passes. Record
their provenance: exact source commit/tree, Rust toolchain, target/backend, debug/release profile,
fixture hash, deterministic PCM generation and qualifying command. A digest is a regression
discriminator, not independent DSP evidence, and cannot be obtained merely by accepting the current
implementation's output. Debug and release must match their separately recorded reviewed constants
or a documented exact shared constant.

Render remains allocation/free, lock, syscall, I/O and logging free. The untimed qualification must
prove zero current-thread allocations, reallocations and deallocations and zero realtime audit
violations across representative traffic renders, with all publication and evidence collection
outside render.

### Required direct mutation and durable evidence

After positive constants and assertions are frozen, temporarily suppress the actual production
input `try_push` operation while preserving the reported attempted and accepted schedule/count
values. Run the unchanged frozen connected PCM/output-digest assertion and require its direct
nonzero mismatch; a counter mismatch may be additional evidence but cannot satisfy this gate.
Restore the source immediately. Preserve under the attempt-3 artifact directory: the exact mutation
diff, command, exit status, direct PCM/digest diagnostic, pre/post source hashes, clean restoration
diff, restored debug/release results, qualification output and a checksum manifest. No permanent
suppression mode, alternate test branch or caught synthetic assertion is accepted.

### Attempt 3 gates and closure

Before source PASS, run the new subject tests and untimed qualification command in debug and release;
W8 population/addressability, checked counts, separate drain/no-pending witnesses, connected
per-block ramp oracle, independent owners, reviewed constants, nonzero PCM and no-record
discriminator; zero render allocation/free/realtime violations; the restored direct mutation; the
unchanged #238/#496 symmetry and input-liveness tests in debug and release; strict bench Clippy and
rustdoc; formatting/diff; and workspace/realtime/builtins/graph/lane policies.

Astra LOW reviews the exact pushed attempt-3 source/evidence. A FAIL consumes the last attempt and
requires a new bounded successor, with no retry. On PASS, integrate current main, recheck #598
disjointness, obtain Astra LOW exact-head PASS, run required PR qualification, merge, require
successful post-main qualification, synchronize and close #600, and remove its clean delivered
worktree. Do not mark RT5 delivered or start an original open finding. Only after #600 cleanup may
root create the promised tooling/capture successor in the released lane slot.

## Astra LOW circuit-breaker rebrief recommendation

Astra LOW approved this split in read-only review of exact pre-amendment head
`87b9521e36f3467c151b2152116ca847f189c7a5`. The review requires the retitled subject-only
capability, deletion of the four known-defective new scripts, zero timing surface, a connected
independent runtime ramp oracle, separate drain witnesses, reviewed digest provenance, durable real
mutation evidence, and explicit retention of RT5 measurement. This amended brief must itself receive
formal Astra LOW PASS before Luna begins the sole remaining implementation attempt.

The first formal review of amended head `7af0d2f2e7bace1e4c7d701a168f3592e49f2d25` accepted every
other rebrief term but rejected an `accepted-traffic or PCM/digest` mutation escape. The corrected
gate above preserves both attempted and accepted reported counts while suppressing actual pushes and
requires the unchanged connected PCM/output digest itself to fail. This scope correction consumes no
implementation attempt and requires exact-head rereview.
