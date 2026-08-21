# 039 Native graph scheduler qualification and benchmark

## Outcome

Qualify the existing Linux/cloud native graph scheduler against the frozen deterministic,
move-ownership, realtime and portability contracts, then run its descriptive sequential/two-lane/
four-lane benchmark exactly once. This issue proves the scheduler checkpoint; it does not redesign
the scheduler, graph, DSP or benchmark framework.

## Context

Engine V2 is greenfield and must never inspect, copy, benchmark against or inherit V1/legacy.
The render thread exclusively owns a preallocated `PreparedRenderPlan`; render performs zero
allocation/free, locks, file/network I/O, logging, syscalls, feature detection, thread lifecycle,
panic/unwind, structural mutation or data-dependent unbounded engine work. There is no compiled
track ceiling. Audio is planar `f32`; dual-mono state remains independent unless an explicit
contract links it. Launch rates are exactly 44,100, 48,000, 88,200 and 96,000 Hz, with no implicit
SRC. Output is PCM.

Issue **Native deterministic multicore scheduler** stopped after its two-attempt budget without an
overall PASS. Upstream checkpoint `3236b9c` is accepted here only as coherent technical input: it
contains a real Linux x86-64 parallel graph executor with immutable waves, move-owned disjoint
partition jobs, indivisible retained banks, stable coordinator staging/observation, transactional
native binding, typed sequential fallbacks, fixed SPSC worker transport, audit/benchmark tools and
zero-launch runner checks. Its focused functional, policy, cross-target, Wasm-object and 10,000-
callback audit evidence remains valid period evidence. It did not complete the frozen qualification
matrix or authorize timing, and this rescope does not retroactively declare Issue 9 PASS.

This issue has exactly **two total attempts**: one Terra qualification attempt and, if needed, one
bounded Sol correction/review. A second failure stops. The authoritative tracked brief is
`.github/ISSUE_SPECS/BRIEFS/039-native-graph-scheduler-qualification-and-benchmark.md`. Timed
benchmark invocation count starts at **0**.

## Scope

- Build one reusable 48-kHz/128-frame representative production graph through accepted public
  compiler/preparation APIs. It is asymmetric dual mono and contains a real retained builtin bank,
  scalar tails, stateful processing, an explicit sidechain, a send/submix reduction, nonzero exact
  PDC and stable observers. Parameterize only launch rate and scheduler lane count for the frozen
  differential matrix.
- Prove the representative graph byte-identical across consecutive and continuation blocks in
  sequential, two-lane and four-lane execution at every launch rate. Freeze exactly 32 completion-
  order perturbations from seed `0x000000000009d37a` without changing arithmetic or observer order.
- Prove immutable wave/unit/partition preparation over exactly 100 fresh preparations and generated
  track counts `1,3,4,5,12,17`, including indivisible banks, narrow waves, scalar tails, no padding,
  no track ceiling and exact checked-cap rejection.
- Add bounded test-only protocol injection for startup handshake failure, queue full, stale
  generation and duplicate completion. Prove stable error selection, full move-owned parcel
  recovery and exactly-once drop/execution; never add a product retry or unsafe alias.
- Trace every participating Linux audit thread and prove the callback/worker interval contains no
  unexpected syscall. Retain the existing per-thread forbidden-operation snapshots, fixed storage,
  plan swap and off-render retirement evidence.
- Compile the native path for macOS, run the complete warning-denied rustdoc and repository gates,
  and seal the existing benchmark workload/runner/validators on one clean committed candidate.
- Only after all nonbenchmark gates pass and root Sol explicitly authorizes it, invoke the existing
  scheduler benchmark runner exactly once for one untimed warmup and two measured rounds.

## Required public interfaces/contracts

No new production API is required. Test-only completion/startup injection must be inaccessible from
normal production builds and must preserve the real SPSC ownership protocol. The representative
fixture must use the production compiler, retained builtin-bank artifact and native graph binding;
a mock-only graph, direct scheduler parcel loop or synthetic byte fold is not evidence.

The existing no-argument runner remains the sole timing entrypoint. It owns exactly one untimed
warmup process and measured rounds `1` and `2`, emits exactly six strict JSONL records (three modes
times two rounds), preserves raw bytes, refuses overwrite/retry and records an honest disposition.
There is no performance threshold or speedup requirement.

## Deliverables

- one shared q128 representative graph fixture and exact differential/continuation/observer report;
- the exactly-32 seeded completion-perturbation suite;
- exact-100 preparation and generated-track partition/resource transcript;
- startup/completion protocol fault-injection and move-ownership matrix;
- per-thread Linux syscall trace joined to the existing audit evidence;
- macOS, full rustdoc, workspace/target/policy and clean-candidate seal reports; and
- one authorized exactly-once descriptive benchmark artifact with rough ratios.

## Explicit non-goals

Scheduler/graph/PDC/reduction redesign; new DSP; a second fixture or benchmark framework; another
worker transport; shared mutable executor state; mutex/raw-pointer/unsafe aliasing; work stealing;
affinity, priority, workgroups, parking or wake policy; mobile/browser parallel rendering; device
runtime qualification; changing session/protocol/C ABI; tuning, retries, a performance threshold or
long-duration release qualification. A production-semantic defect discovered by qualification
fails this issue and gets its own stateless correction; do not hide it in evidence code.

## Dependencies by exact issue title

- Native deterministic multicore scheduler
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Production SIMD builtin bank graph retention and reachability qualification
- Real-time memory, buffers, queues, and plan lifetime

The stopped Issue-009 dependency means only checkpoint `3236b9c` and its explicitly preserved
evidence described above; it does not imply an Issue-009 PASS.

## Sol implementation brief (2026-08-21)

**READY FOR TERRA ATTEMPT 1 after Issue 39 exists remotely and matches this body.** The tracked
brief freezes the fixture, seeds/counts, protocol injections, trace boundary, target gates,
two-attempt budget and sole exactly-once timing authorization.

## Hazards/decisions

Qualification must observe the real scheduler rather than create a parallel test implementation.
Completion perturbation may delay when a returned parcel is accepted by the test coordinator, but
cannot select reduction/observer order or execute a parcel twice. Fault injection must leave every
move-owned parcel recoverable and must not become a production control surface. A syscall trace
must distinguish startup/handshake and retirement from the armed render interval and include the
coordinator plus all auxiliary workers.

The benchmark is descriptive. Faster or slower results both pass if correctness, audit and schema
gates pass. A workload or runner failure consumes the one timing authorization; preserve raw and
disposition bytes and never rerun to repair or improve a number.

## Acceptance gates with objective measurements

1. One q128 production fixture proves byte-identical PCM for sequential, two-lane and four-lane
   execution at 44,100/48,000/88,200/96,000 Hz over consecutive stateful blocks plus a fresh
   continuation block. Output/PDC/counter hashes and the complete stable observer transcript match.
2. Exactly 32 completion perturbations from seed `0x000000000009d37a` produce the same PCM,
   continuation, counters, observer transcript and recovered-partition report as sequential.
3. Exactly 100 fresh preparations produce one immutable transcript hash. Counts
   `1,3,4,5,12,17` prove deterministic near-even partitions, narrow-wave fallback, indivisible
   retained banks, stable scalar tails, no padding/ceiling, exact resource accounting, overflow and
   configured-cap rejection.
4. Injected handshake failure returns every bind input transactionally. Queue-full, stale-
   generation and duplicate-completion cases recover every parcel exactly once, select errors in
   stable partition order and never rerun an advanced job. Endpoint/drop-order tests report one and
   only one drop for each move-owned token.
5. The 10,000-callback four-lane q128 audit retains zero coordinator and worker forbidden-operation
   counters, fixed addresses, exact command/completion counts, one block-boundary swap and
   retirement-thread destruction. A per-thread Linux syscall trace proves zero unexpected syscalls
   by coordinator or workers while their render scopes are armed.
6. Focused/locked tests plus full locked workspace check/test, format, warning-denied Clippy and
   rustdoc, workspace/realtime/graph/rack/builtin/scheduler policies and relevant mutation suites
   pass. Linux x86-64 runs the threaded evidence; macOS compiles the native path. Existing Linux
   AArch64, iOS/Android fallback and Wasm scalar/simd128/object evidence is revalidated only if the
   qualification change touches its production dependency boundary.
7. On one clean committed candidate, zero-launch preflight proves production fixture reachability,
   strict single/aggregate schemas, exactly six measured-record cardinality, output persistence,
   shell failure propagation, interruption disposition, overwrite/argument refusal and no retry.
   Candidate, binary, source/fixture, runner and validator hashes are sealed; it reports
   `workload_launches=0`, `warmup_rounds=1`, `measured_rounds=2`.
8. Only after gates 1–7 pass and root Sol records authorization, invoke exactly
   `bash scripts/run-scheduler-benchmark.sh` once. It performs one untimed warmup and exactly two
   measured rounds, yields six accepted records with identical mode output hashes and zero render/
   forbidden-operation errors, preserves byte-identical raw/accepted data plus disposition, and
   records rough sequential/two-lane/four-lane ratios without tuning or retry.

## Target matrix

Linux/cloud x86-64 is execution-qualified. macOS must compile the native scheduler path. Linux
AArch64 and iOS/Android retain their declared sequential selection until separate host integration;
browser Wasm remains single-threaded. This issue makes no new runtime claim for those targets.

## Required evidence

Issue-009 checkpoint and source hashes; fixture identity; all-rate/lane PCM, continuation, PDC,
counter and observer hashes; exact perturbation seed/count/transcript; exact preparation count/set/
transcript and resource failures; protocol ownership/failure matrix; per-thread audit snapshots and
syscall trace; retirement-thread record; macOS/rustdoc/workspace/policy reports; clean zero-launch
seal; root authorization; raw/accepted/disposition hashes and sizes; exact invocation/warmup/round/
record counts; environment metadata; rough descriptive ratios; and Terra plus final Sol PASS/FAIL
verdicts. Timed invocation count before authorization is exactly zero.

## Terra attempt 1 — nonbenchmark gate record (2026-08-21)

The shared q128 fixture completed the frozen all-rate/lane differential, exactly-32 completion
perturbation, exactly-100 preparation/resource, and test-only ownership-protocol matrices. The
four-lane 48-kHz/q128 audit rendered exactly 10,000 callbacks with one block-boundary replacement,
20,000 fixed-storage observer records, fixed output storage, coordinator forbidden total `0`, and
worker forbidden totals `[0,0,0]`; its all-thread trace retained the raw files, validator and
SHA-256 manifest under `target/issue039/scheduler-audit-strace/`. The validator attributed the
armed interval to the coordinator and all three active replacement workers and found zero
syscalls for each.

The remaining nonbenchmark gates passed: `cargo fmt --all -- --check`; locked workspace
all-target/all-feature check and test; warning-denied workspace Clippy and rustdoc; workspace,
realtime, graph, rack, builtins and scheduler policy checks plus their mutation suites; and
`cargo check --offline --locked -p miso-engine-graph --target x86_64-apple-darwin`. The macOS
standard-library target was installed for that compile only; this is a compile claim, not macOS
runtime qualification.

**Gate 7 did not seal.** `bash scripts/preflight-scheduler-benchmark.sh` stopped at its initial
clean-candidate guard with the exact output `Issue-009 preflight requires a clean candidate`.
It therefore did not run its zero-launch benchmark validation/build steps and emitted no candidate
or binary seal. No scheduler benchmark binary, runner, timed workload, warmup or measured round
was invoked; the timed invocation count remains `0`. Per the frozen stop rule, Terra attempt 1
stops here for Sol review rather than treating an unsealed candidate as ready for timing.
