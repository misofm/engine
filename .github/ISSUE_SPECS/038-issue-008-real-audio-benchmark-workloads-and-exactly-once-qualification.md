# 038 Issue-008 real audio benchmark workloads and exactly-once qualification

## Outcome

Replace the placeholder byte-fold benchmark with the three frozen real audio workloads, a strict
record/aggregate schema and a safe exactly-once runner; after production SIMD reachability passes,
authorize at most one invocation containing one warmup and exactly two measured rounds.

## Context

Issue **AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels** stopped after two attempts with its
architecture correction preserved but overall acceptance failed. Its benchmark tool merely times
the bytes of workload labels, and its preflight falsely reports readiness without validating the
required audio work, schema or artifact lifecycle. Issue **Production SIMD builtin bank graph
retention and reachability qualification** separately owns all product integration, deterministic
correctness, instruction and realtime audit work. This issue begins only after that candidate
passes and changes no engine DSP/runtime semantics.

This workflow has exactly **two total attempts**: one Terra implementation/review attempt and, if
needed, one bounded Sol correction/review. A second failure stops. Current timed benchmark
invocation count is **0**. Building/hashing the binary and validating synthetic records are allowed
only under preflight with `workload_launches=0`. No workload may run until every nonbenchmark gate
passes on one clean committed candidate and root Sol explicitly authorizes
`bash scripts/run-rack-benchmark.sh`.

This issue does not block **Native deterministic multicore scheduler** or **JIT PCM streaming and
host-supplied source rings** unless a later stateless amendment proves their own product contract
requires these descriptive timings. It gates release performance qualification, not unrelated
feature delivery.

## Scope

- Implement the exact real scalar, host-selected bank and mixed production-graph workloads below.
- Emit six strict JSONL records: three workloads times measured rounds 1 and 2.
- Add single-record and aggregate validators with exhaustive bounded mutation tests.
- Harden zero-launch preflight and the raw/accepted/disposition artifact lifecycle.
- Only after root Sol authorization, run one external invocation with one untimed warmup and two
  measured rounds. Preserve the first raw bytes and never tune or retry.

## Required public interfaces/contracts

The benchmark binary accepts no arguments and is not a general harness. The no-argument
`scripts/run-rack-benchmark.sh` is the sole workload entrypoint. The runner owns one warmup launch
and exactly two measured rounds, raw stdout persistence, strict validation, byte-identical
accepted promotion and a checksummed disposition. The benchmark observes production APIs from the
Issue-037 candidate; it does not duplicate TPT equations or use mock label work.

## Frozen workloads

All workloads use 48,000 Hz, quantum 128, deterministic asymmetric dual-mono input and continuous
state, with exactly 1,000 measured observations per workload/round:

1. `scalar_eight_tracks`: eight separately prepared scalar post-input builtin TPT tracks.
2. `host_selected_eight_track_bank`: the same eight tracks in one production host-selected x86
   eight-lane bank; record whether dispatch is `X86Avx2` or `X86Avx2Fma`. Preflight must refuse an
   unsupported host before consuming the sole authorization.
3. `mixed_twelve_track_graph`: the exact Issue-037 production 12-track graph with a full bank,
   missing/identity position, stable scalar tail and incompatible scalar fallback.

Input preparation/fill and result hashing occur outside each observation timer. Each duration is
integer nanoseconds divided by exactly 128 frames and reported as `ns_per_frame`. Sorting and
nearest-rank percentile calculation occur after measurement. Timing is descriptive: no speedup or
absolute threshold is an acceptance gate.

## Exact record contract

Every record contains exactly: schema/identity (`schema_version=2`, `issue=38`, workload kind/ID,
round); shape (`sample_rate_hz=48000`, `quantum_frames=128`, tracks, bank backend/width/count,
scalar-tail/fallback count, identity-lane count); measurement (`observations=1000`,
`percentile_method="nearest_rank"`, `units="ns_per_frame"`, ordered integer
`min/p50/p95/p99/p99_9/max_ns_per_frame`, `descriptive_only=true`); candidate/binary/fixture/input/
output SHA-256 identities; zero render errors and zero allocation/deallocation/lock/feature-
detection/log/file/network/syscall/panic-unwind counters with an exact summed total; and CPU model,
architecture, logical/physical cores, OS, kernel, governor/power mode, Rust, LLVM, target triple,
target features, profile, opt level, LTO, codegen units and background-load note.

Undiscoverable metadata is JSON null. `missing_metadata` is the sorted unique exact list of null
metadata field names; empty strings or `unknown/default` sentinels are invalid discovered values.
Workload IDs are `issue038.<kind>.48000hz.q128`. Candidate/binary/fixture identity is constant
across all six records; input/output identity is stable across the two rounds for each workload.

## Deliverables

- real production benchmark runtime and frozen workload inputs;
- schema-v2 single/aggregate validators and mutation fixtures;
- zero-launch preflight and negative runner/artifact lifecycle tests;
- safe exactly-once runner with raw, accepted, stderr and disposition artifacts; and
- if authorized, six accepted descriptive records plus hashes and a Sol verdict.

## Explicit non-goals

Engine/runtime/DSP/graph changes, a second benchmark framework, optimization, threshold tuning,
retries, extra rounds/workloads/rates, device/browser benchmarking, human listening, changing an
observation after seeing timings, or blocking Issues 009/010 on descriptive performance.

## Dependencies by exact issue title

- Production SIMD builtin bank graph retention and reachability qualification

## Sol implementation brief (2026-08-21)

**READY FOR TERRA ATTEMPT 1 ONLY AFTER ISSUE 037 PASSES.** The tracked authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/038-issue-008-real-audio-benchmark-workloads-and-exactly-once-qualification.md`.
It freezes the workloads, schema, artifact lifecycle, two-attempt budget and sole authorization.

## Hazards/decisions

Names are not workloads: preflight must prove the binary reaches production scalar/bank/graph
APIs. Do not accept compiler auto-vectorization or mock banks as the production-bank workload.
The warmup is not a measured round. A workload/validator/runner failure consumes the authorization;
preserve raw output and do not rerun. Post-workload promotion failure may be repaired only through
a new tooling issue using the preserved bytes, never by remeasurement.

## Acceptance gates with objective measurements

Before timing, fixture/input completeness, real API reachability, schema validators, aggregate
cardinality, every required field/type mutation, kind/round duplicate/omission, percentile order,
hash drift, dishonest metadata, nonzero/mistotaled audit counts, output persistence, shell exit
propagation, interruption disposition, overwrite refusal, argument rejection and no-retry source
policy all pass with `workload_launches=0`. Full locked workspace check/test, warning-denied Clippy
and rustdoc, format and applicable workspace/realtime/builtin/graph/rack policies pass on the same
committed Issue-037 candidate. Candidate, binary, runner, validator and fixture hashes are sealed.

Only then may root Sol authorize exactly one `bash scripts/run-rack-benchmark.sh`. It performs one
untimed warmup and exactly two measured rounds, emits exactly six validator-valid records, reports
zero render errors/forbidden operations, preserves raw bytes and creates a byte-identical accepted
copy plus PASS disposition. There is no timing threshold and no retry.

## Target matrix

One pinned native x86-64 AVX2-capable qualification host; the record distinguishes AVX2 without
FMA from AVX2+FMA. ARM/Wasm instruction/product evidence belongs to Issue 037, not this timing run.

## Required evidence

Issue-037 candidate identity; workload reachability proof; synthetic validator/mutation and runner
negative logs; preflight record with `workload_launches=0`; sealed hashes; explicit root Sol
authorization; raw/accepted/disposition size and SHA-256; six-record aggregate summary; exact
invocation/warmup/round counts; environment metadata; and Terra plus final Sol PASS/FAIL verdicts.

## Terra attempt 1 implementation record (2026-08-21)

Candidate started from Issue-037 PASS commit `4680eeb`; the shared mainline advanced to
`372d984` while this tooling-only attempt was prepared. `miso-engine-rack-bench` now prepares:

- eight independent real post-input builtin TPT chains;
- one prepared host-selected production eight-lane `BuiltinInputBankV1`; and
- a sealed 12-track production `GraphCompiler::compile_with_builtins` plan with a retained full
  builtin bank and scalar graph paths.

The no-argument runner owns one warmup and two measured process launches, retains untouched raw
stdout, validates six exact schema-v2 records, copies an exactly byte-identical accepted artifact,
and writes a checksummed PASS/FAIL disposition. Single and aggregate jq validators reject unknown
or missing keys, type/shape/hash/audit/metadata drift, duplicates and round/workload cardinality
errors. The preflight passed fixture hashing, all bounded per-key deletion/type mutations, runner
argument/source checks, release build and `workload_launches=0`. No benchmark binary or timing
runner invocation was performed by Terra; the timed invocation count remains **0** pending root
Sol authorization and review.

## Sol attempt 1 adversarial verdict (2026-08-21)

**FAIL; bounded correction required.** Review found that the runner supplied a 40-character Git
object ID where the record schema required SHA-256, so the first warmup could never have produced
an accepted record. The nominal mixed workload did not construct the frozen missing/identity,
scalar-tail and incompatible-fallback shape, performed input work inside the graph render call and
hashed only the last output block. The scalar workload timed the full builtin chain instead of the
specified post-input TPT operation, direct scalar/bank calls lacked an armed allocation audit, and
input identity was neither workload-specific nor a hash of the full measurement input. Validators
also accepted matched nonzero forbidden-operation counters, while preflight did not exercise the
required partial-output, interruption, pipe-failure and exactly-once artifact lifecycle. No timed
workload was launched while discovering these defects; invocation count remained **0**.

## Sol bounded correction / attempt 2 nonbenchmark verdict (2026-08-21)

**NONBENCHMARK PASS; one timed invocation remains pending.** The correction now uses eight real
scalar post-input TPT processors, one host-selected production eight-lane bank, and a sealed
12-track production graph whose preparation asserts one eight-track cohort, two compatible scalar
tails, two incompatible scalar fallbacks and two missing/identity slot positions. Per-track left
and right parameters are asymmetric and nonidentity. All 1,000 deterministic source observations
are prepared before graph timing; every observation times only the production operation, divides
integer nanoseconds by 128, and hashes the complete semantic input and output streams outside the
timer. Scalar and bank render regions arm the realtime audit allocator. The record validators now
require every forbidden counter and the exact sum to be zero, exact six-record/two-round
cardinality, stable per-workload identities and honest metadata/null accounting.

The runner now derives the 64-character candidate SHA-256, rejects arguments, unsupported hosts,
dirty trees and all pre-existing artifact paths before launch, uses append-only raw persistence,
propagates pipeline failures and records checksummed PASS/FAIL disposition state. Hermetic
zero-audio lifecycle tests cover warmup failure, partial round-one and round-two output, invalid
aggregate output, metadata-pipeline failure, interruption, overwrite refusal, successful
byte-identical promotion and rerun refusal. Fixture content and manifest mutations are checked
independently.

The final shared pre-commit checkpoint passed:

- `cargo test --workspace --locked`, including all doctests;
- warning-denied workspace Clippy and rustdoc, full locked workspace check and format;
- rack-benchmark unit, schema/mutation, fixture and runner lifecycle tests;
- workspace/realtime/builtin/graph/rack policies and their applicable mutations; and
- fresh-process graph determinism at 100/100.

The rack lifecycle suite reports `audio workload launches: 0`; no `artifacts/issue038` timing
artifact exists and the timed invocation count is still **0**. Root owns the coherent checkpoint
commit and upstream synchronization. After that exact commit passes
`bash scripts/preflight-rack-benchmark.sh` on a clean AVX2 host with `workload_launches=0`, this Sol
review authorizes root to invoke **exactly once**:

```sh
bash scripts/run-rack-benchmark.sh
```

That single invocation owns one warmup and two measured rounds. Any workload or runner failure
consumes the authorization; preserve raw/disposition bytes and do not retry. Final Issue-038 PASS
still requires the six accepted records, byte-identical raw/accepted hashes, PASS disposition and
remote evidence synchronization.

## Final exactly-once benchmark result (2026-08-21)

**PASS.** Root ran `bash scripts/preflight-rack-benchmark.sh` on clean commit `ccd3967`; it passed
with `workload_launches=0`, one warmup, two measured rounds and six required records. Root then
invoked `bash scripts/run-rack-benchmark.sh` exactly once. The runner exited zero; no retry or
tuning occurred. Final timed invocation count is **1**.

The accepted artifact contains exactly six records and is 10,978 bytes. Raw and accepted JSONL are
byte-identical with SHA-256
`7abce9d0532c5fbc9326a4f03b76be6e50ccd2319406ee9245165ad756f37e07`; the PASS disposition is
715 bytes with SHA-256 `1ac6dd71f26f9573076143bd0e610be0465008e4faea3504c4868f6efaef3c3f`.
The frozen aggregate validator passes the accepted bytes. Every record reports zero render errors,
allocations, deallocations and forbidden operations, stable workload-specific input/output hashes,
and complete host/build metadata.

Nearest-rank descriptive results in `ns_per_frame` were:

| Workload | Backend | Round 1 p50 / p95 / p99 | Round 2 p50 / p95 / p99 |
| --- | --- | --- | --- |
| eight independent scalar TPT tracks | Scalar | 100 / 103 / 200 | 100 / 101 / 201 |
| one eight-track bank | X86Avx2Fma | 138 / 138 / 276 | 138 / 138 / 147 |
| sealed mixed twelve-track graph | X86Avx2Fma | 781 / 810 / 822 | 805 / 817 / 820 |

These are rough host-specific observations on an AMD Ryzen 7 9700X under the `powersave` governor
with uncontrolled background load. They have no pass/fail threshold and establish no speedup
claim. In this harness the bank p50 is slower than eight independent scalar chains; optimization is
therefore deferred to the weekly performance workflow rather than reopening Issue 038.
