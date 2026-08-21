# 009 Native deterministic multicore scheduler

## Outcome

Ship a first deployable Linux/cloud native renderer that executes the accepted graph dependency
waves on prestarted dedicated workers while preserving the accepted single-thread PCM, state,
PDC, observer and stable-reduction semantics exactly. Unsupported, disabled or insufficiently
parallel configurations use the same prepared jobs sequentially. Browser rendering remains one
render thread.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy,
benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated
`PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only
through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O,
logging, syscalls, feature detection, thread spawn/join, panic/unwind, structural plan mutation or
data-dependent unbounded engine work; displaced plans and scheduler workers are stopped/joined
only when the plan is reclaimed off render. There is no compiled track limit. Audio is planar
`f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or
smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100,
48,000, 88,200 and 96,000 Hz. Source/engine mismatches have no implicit SRC. Output is PCM.

Issue **Deterministic graph compiler, sends, submixes, sidechains, and PDC** supplies the accepted
stable sequential schedule, immutable `DependencyLevel` IR, edge order, balanced reduction, exact
PDC, buffer/resource report and scalar executor behavior. Issue **Production SIMD builtin bank
graph retention and reachability qualification** supplies retained native effect/builtin banks
whose members already share a dependency wave. A bank is one indivisible ready job; scheduling is
an execution grouping and never a graph/PDC/reduction rewrite.

This issue has exactly two total attempts: one Terra implementation/review attempt and, if needed,
one bounded Sol correction/review. A second failure stops and requires a stateless rescope. The
authoritative Sol brief is
`.github/ISSUE_SPECS/BRIEFS/009-native-deterministic-multicore-scheduler.md`.

## Scope

- Derive immutable native render waves and stable job units from the accepted dependency levels.
  Ordinary nodes are one unit; every prepared native effect or builtin bank is one indivisible
  unit keyed by its smallest stable member ID. Partition units deterministically into a
  caller-supplied render-lane count without padding or a compiled track ceiling.
- Add a native scheduler boundary with one coordinator lane plus prestarted dedicated auxiliary
  workers, one fixed-capacity command SPSC and one completion SPSC per worker, and move-only job
  ownership. Startup/handshake/arming and stop/join occur off render. Workers and the coordinator
  use only bounded queue operations and user-space polling while armed; no OS mutex, condition
  variable, blocking channel, park/unpark, work stealing or heap job queue is render-reachable.
- Refactor the private graph executor only as needed so every dispatched job owns disjoint mutable
  processor/state/output/scratch. All prior-wave inputs are committed before dispatch. Each
  destination applies PDC and reduces its already ordered contributions in the accepted stable
  edge/node order; completion order never selects arithmetic order. Observer order remains stable.
- Preserve `RenderMode::SingleThread`. `RenderMode::DependencyWaves` plus a valid supported native
  host configuration selects parallel waves off render. Disabled/unsupported native targets,
  one requested lane, worker startup failure returned transactionally, and graphs without useful
  wave width select the same job representation's sequential driver with a typed frozen fallback
  reason. A worker failure after dispatch is returned in stable partition order after every issued
  ownership token is recovered; partially advanced state is never rerun as a disguised fallback.
- Extend the accepted core SPSC only additively, if required, with a safe move-only constructor
  over its existing implementation and safety invariant. Do not add another unsafe queue or alter
  publication/retirement semantics.
- Add representative determinism/fallback/protocol tests, one fixed threaded realtime audit, and
  one descriptive benchmark runner. Timing is not an optimization gate.

## Required public interfaces/contracts

`RenderWaveV1` contains an immutable level ID and stable, nonoverlapping prepared job partitions.
`NativeSchedulerConfigV1` uses an explicit nonzero render-lane count, where lane zero is the render
coordinator and `lanes - 1` is the exact auxiliary-worker count; it never calls
`available_parallelism` or detects CPU features in render. `NativeSchedulerV1::render_wave`
dispatches each auxiliary partition exactly once, executes the coordinator partition, recovers
every issued job, and reports completions in partition-ID order. The concrete Rust job carrier may
remain engine-internal, but its move-only ownership and no-alias contract are tested.

Sequential binding remains the compatibility API. An additive native binding/preparation API
consumes the graph, runtime bindings and scheduler configuration transactionally and returns all
owned inputs on startup/configuration failure. Prepared metadata exposes selected lane count,
worker count, wave/job/partition counts, exact scheduler-owned retained bytes and a stable
`SchedulerSelectionV1`/`FallbackReasonV1`. Bounded queue/poll/error counters are readable only
after render is disarmed. These Rust types are not a C ABI commitment.

## Deliverables

- `miso-engine-native-scheduler` / `miso_engine_native_scheduler`, depending only on
  `miso-engine-core`, with prestarted worker lifecycle, fixed SPSC protocol and sequential driver;
- additive move-only SPSC construction in the existing core ownership boundary if the scheduler
  needs it, with drop-order and full/empty ownership tests;
- graph bind/executor integration and exact checked scheduler/job/scratch resource accounting;
- one deterministic representative graph builder/report, generated schedule perturbations,
  fallback and failure-path tests;
- one 10,000-render native threaded allocation/lock/syscall/lifecycle audit;
- a scheduler dependency/realtime policy plus mutation tests; and
- a zero-launch-preflighted, exactly-once descriptive benchmark producing two measured rounds.

## Explicit non-goals

Browser parallel rendering; Apple audio-workgroup membership; Android/mobile realtime-thread
registration; affinity, priority, power, NUMA or idle wake/parking policy; work stealing; dynamic
load balancing; a cost model; changing graph topology, dependency levels, PDC, reductions,
liveness-report identity, bank/cohort membership, DSP algorithms or session/wire tokens; automatic
mid-block re-execution after worker state advances; a second fixture/benchmark framework; long-run
P99.99/deadline qualification; performance thresholds, tuning or retries.

Platform workgroup/affinity/idle policy, extended stress/fault research and systematic scheduler
optimization require separate stateless issues. Long-duration release timing belongs to
**End-to-end release performance and listening qualification**, not this first vertical.

## Dependencies by exact issue title

- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Production SIMD builtin bank graph retention and reachability qualification

## Sol implementation brief (2026-08-21)

**READY FOR TERRA ATTEMPT 1.** The tracked authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/009-native-deterministic-multicore-scheduler.md`. It freezes the
accepted graph/bank inputs, two-attempt budget, production boundary, representative suite/audit
and sole descriptive benchmark authorization.

## Hazards/decisions

The accepted graph executor is currently one monolithic `Send` owner and its liveness coloring was
proved for the semantic sequential schedule. Do not share it through a mutex or raw pointer. Move
owned wave partitions through the accepted SPSC boundary, keep concurrent mutable state/output
disjoint, and charge any execution-only storage needed for parallel liveness to the plan. The
canonical graph schedule, buffer-assignment report and hash do not change merely because a host
selects a different lane count.

A worker owns a job only between command publication and its matching completion. Queue-full or
duplicate/missing completion is an invariant error, never permission to lose a job or run it
twice. Idle dedicated workers may user-space poll only while an explicitly enabled plan is armed;
more efficient parking/wakeup and platform workgroups are follow-ups because waking from the
callback would violate the no-syscall contract. Scheduler destruction is therefore part of
off-render plan retirement.

## Acceptance gates with objective measurements

1. The same accepted graph produces the same immutable wave/unit order and partition report over
   100 fresh preparations. Generated track counts `1,3,4,5,12,17` prove stable partitions, no
   padding/ceiling, indivisible effect/builtin banks and exact checked resource/cap rejection.
2. At all four launch rates with quantum 128, sequential, two-lane and four-lane plans render the
   representative asymmetric dual-mono graph byte-identically across consecutive stateful blocks.
   It includes a real retained builtin bank, scalar tails, a sidechain, a send/submix reduction,
   exact PDC and observers. Exactly 32 completion perturbations from a frozen seed produce the
   same PCM, continuation PCM, counters and observer transcript as sequential execution.
3. `SingleThread`, one-lane, unsupported-target, insufficient-wave-width and injected
   prepare/handshake failure take the declared sequential or transactional fallback path. Queue
   full, stale generation, duplicate completion and stable worker-error selection recover every
   move-only job exactly once; no advanced job is executed twice.
4. One release audit renders exactly 10,000 48-kHz/128-frame callbacks through a four-lane
   production graph. Coordinator and auxiliary-worker audit snapshots each report zero
   allocation/free, lock, feature detection, log, file/network I/O, unexpected syscall,
   panic/unwind or structural mutation. Fixed addresses and exact command/completion counts hold;
   a block-boundary replacement is complete, and the retired scheduler is stopped/joined and
   destroyed only on the retirement thread after audit disarms.
5. Focused and locked workspace tests/checks, warning-denied all-target Clippy/rustdoc, format,
   workspace/realtime/graph/rack/builtin/scheduler policies and their relevant mutation suites
   pass. Linux x86-64 runs the threaded gates; Linux AArch64 and macOS compile the native path;
   iOS/Android compile and select only their declared sequential fallback. Wasm scalar and
   `simd128` builds retain the single-thread graph and object/dependency inspection finds no
   scheduler thread or atomic opcode in the browser artifact.
6. Before timing, the no-argument scheduler runner's schema, real graph reachability, output
   persistence, shell status propagation, overwrite refusal and no-retry behavior pass with
   `workload_launches=0`. Candidate, runner, validator and workload identities are sealed on one
   clean committed candidate.
7. Only after gates 1-6 pass and root Sol authorizes it, invoke the scheduler benchmark exactly
   once. It performs one untimed warmup and exactly two measured rounds for the same frozen
   48-kHz/128-frame sufficiently parallel graph in sequential, two-lane and four-lane modes,
   emitting six strict JSONL records with identical output hashes, zero render/forbidden-operation
   errors and honest environment metadata. Results and ratios are descriptive only: there is no
   speedup or callback-percentile acceptance threshold, tuning or retry.

## Target matrix

The first execution-qualified parallel vertical is Linux/cloud native x86-64. Linux AArch64 and
macOS build the native scheduler; enabling/qualifying it there is a follow-up unless the same
attempt supplies representative runtime evidence without expanding scope. iOS and Android retain
deterministic sequential fallback until their host realtime-thread integration issues. Browser
Wasm is explicitly sequential with no scheduler workers or shared-memory claim.

## Required evidence

Candidate/source hashes; selected/fallback metadata and exact resource report; wave/job/partition
transcript and frozen seed; sequential/two/four-lane PCM, continuation, PDC, counter and observer
hashes; protocol ownership/failure matrix; per-thread audit and syscall trace; plan-retirement
thread evidence; target/dependency/object and policy reports; benchmark preflight
`workload_launches=0`; and, only after authorization, raw/accepted benchmark hashes plus exact
invocation/warmup/round counts and rough descriptive ratios. Record Terra and final Sol PASS/FAIL
verdicts. Never run a second benchmark to improve or repair a result.
