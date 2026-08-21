# Sol implementation brief — issue 009 native deterministic multicore scheduler

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1** from clean `main` at `4680eeb`. Implement one Linux/cloud native
vertical. This workflow permits one Terra implementation/review and at most one bounded Sol
correction/review; a second failure stops. Do not inspect V1/legacy. Do not run a benchmark until
all nonbenchmark gates pass on one committed candidate and root Sol authorizes the exact command.

## Accepted inputs — retain, do not redesign

- Issue 003 owns `PreparedRenderPlan` exclusive `Send`/`!Sync` render ownership, the bounded SPSC
  acquire/release protocol, block-boundary publication and off-render retirement. Its public SPSC
  currently requires `Copy`, although the same implementation already carries move-only plans
  internally. Issue 009 may expose one additive safe move-only constructor in `realtime/spsc.rs`;
  it may not add another unsafe boundary or change publication/retirement behavior.
- Issue 006 owns `GraphSpec`, `sequential_schedule`, `DependencyLevel`, stable node/edge order,
  pairwise reductions, exact PDC, observers, buffer/resource reports and scalar output. The
  canonical graph bytes/hash remain independent of host lane count and completion order.
- Issue 037 owns real retained effect/builtin banks, AoSoA scratch, independent lane state and
  production reachability. All members of a retained bank share a dependency level. A bank is one
  indivisible scheduler job and its gather/process/scatter semantics do not change.
- `RenderMode::{SingleThread, DependencyWaves}` already exists in strict session TOML and protocol
  wire. Do not change either token or schema. Capability/worker resources are explicit native-host
  preparation inputs, never discovered inside render. Browser remains sequential.

The current private `GraphExecutor` is one monolithic `Send` owner with shared node/buffer vectors.
Do not put it behind a mutex and do not share it through raw pointers. Refactor ownership before
threading it.

## Production boundary

Add `miso-engine-native-scheduler` / `miso_engine_native_scheduler`, depending only on core. It
owns generic move-only job transport, prestarted worker lifecycle and deterministic wave dispatch;
it knows nothing about sessions, graphs, effects, builtins or hosts. `miso-engine-graph` may depend
on it only for supported native targets and remains the sole production `PreparedPlanExecutor`.
Graph compiler continues to supply accepted dependency levels; native graph binding lowers them
off render into scheduler jobs. Update manifests, docs and exact dependency/policy guards. Add no
external dependency and no unsafe outside the existing SPSC file.

Modify only core's additive SPSC surface, the new scheduler crate, graph/compiler and sealed
builtins binding seams needed for transactional native preparation, relevant manifests/policies,
one scheduler audit tool and one benchmark tool/runner. Do not change DSP, graph topology/hash,
PDC equations, reduction order, rack cohorts, session/protocol wire, C ABI, streaming or hosts.

## Frozen wave and ownership model

1. Convert each accepted `DependencyLevel` into `RenderWaveV1`. Stable job units are ordered by
   `GraphNodeId`; a retained effect/builtin bank replaces all its members with one unit keyed by
   the smallest member ID. Reject missing/duplicate/cross-level membership transactionally.
2. `NativeSchedulerConfigV1.render_lanes` is explicit and nonzero. Lane zero is the callback
   coordinator; auxiliary worker count is exactly `render_lanes - 1`. Partition each wave into at
   most `min(render_lanes, unit_count)` contiguous, near-even stable unit ranges. Never pad jobs,
   invent tracks, call `available_parallelism`, use a cost model or rebalance at render time.
3. A graph job owns its mutable processor/effect/delay/bank state, output and scratch. PDC state
   belongs to the destination job. Before dispatch, the coordinator copies committed prior-wave
   outputs into the next jobs' preallocated edge inputs in accepted edge order. The job applies
   delay and pairwise reduction in that same order and writes only its owned output.
4. Do not rely on sequential liveness aliases for concurrent writes. Retain the accepted canonical
   buffer-assignment report/hash, but allocate and charge exact execution-only disjoint output and
   contribution storage required by the native job layout. Cap/overflow failure returns every
   graph/binding/config input and no publishable partial plan.
5. Workers do not invoke graph observers. After all wave parcels return, the coordinator observes
   completed outputs in accepted node then handle order. Final output copy likewise occurs only
   after its producing wave completes. Completion order never affects arithmetic or telemetry
   order.

Each auxiliary worker has one capacity-one command SPSC and one capacity-one completion SPSC. A
command moves one prepared parcel plus wave/partition/generation/time metadata to its sole worker;
completion returns the same parcel, a bounded result and that worker's audit counters. At the
start/end of every wave the coordinator owns every parcel. Queue full, stale generation,
duplicate/missing completion or wrong partition is a typed invariant error; recover all issued
parcels before returning and never execute an advanced parcel twice. Ordinary processor errors are
reported only after all issued parcels return and selected by stable partition/unit order. A panic
is forbidden, not a supported recovery path.

Worker spawn, ready handshake and arming finish before publication. Armed workers user-space poll
their fixed queues; the coordinator does useful lane-zero work before polling completions. The
number of commands/completions is prepared and bounded. No mutex, condition variable, blocking
channel, park/unpark, sleep/yield, heap job queue, work stealing, time/feature query or syscall is
render-reachable. Stop and join occur only when retirement reclaims/destroys the plan off render.
Platform workgroups, affinity/priority and a more efficient idle/wakeup policy are follow-ups.

## Selection and compatibility

Keep existing sequential `bind` behavior. Add a transactional native bind/preparation path. A
`SingleThread` session always selects the sequential driver. `DependencyWaves` selects parallel
execution only for an explicitly supported/armed native configuration with at least two lanes and
one wave with at least two units. One lane, disabled/unsupported platform, insufficient width or a
declared host policy restriction selects the same prepared jobs' sequential driver and records one
stable `FallbackReasonV1`; no duplicate DSP state exists. Startup/handshake failure returns owned
inputs rather than silently publishing a fallback plan.

The prepared plan exposes address-free selection, lane/worker/wave/unit/partition counts, exact
retained bytes and bounded command/completion/empty/error/over-budget counters after audit disarms.
There is no wall-clock read in render. A host/benchmark measures callback deadlines externally.
Do not attempt single-thread re-execution after a dispatched job has advanced state.

## Representative nonbenchmark proof

Use one deterministic 12-track, 48-kHz/128-frame production graph builder derived through current
public preparation APIs: asymmetric dual mono, one real retained builtin bank, scalar tails, a
stateful processor, sidechain, explicit send/submix reduction, nonzero PDC and observers. Reuse it
for differential tests and the threaded audit; do not create a second fixture framework.

- Freeze 100 fresh-preparation wave/partition transcripts; counts `1,3,4,5,12,17` cover narrow
  waves, full banks, tails, no padding/ceiling and checked caps.
- At all four launch rates/quantum 128, compare sequential, two-lane and four-lane consecutive
  block PCM plus continuation PCM, counters and observer transcript byte-for-byte. Exactly 32
  test-only completion perturbations from seed `0x000000000009d37a` must not change results.
- Exercise `SingleThread`, one-lane, insufficient-width and unsupported-target fallback;
  transactional spawn/handshake failure; queue full/stale/duplicate/error ordering; and move-only
  item drop exactly once in every endpoint/drop order.
- Run one four-lane release audit for exactly 10,000 callbacks. Arm coordinator and worker audit
  scopes, collect per-thread snapshots through completions, and prove zero allocation/free, lock,
  feature detection, log, file/network I/O, syscall and panic/unwind. Assert fixed storage,
  command/completion counts and output hash. Include one plan swap; worker stop/join and all plan
  destruction occur on the retirement thread after disarm. Trace every participating thread, not
  only the coordinator.
- Add scheduler dependency/realtime policy and mutations covering reverse dependencies, unsafe,
  thread creation/join or blocking/wakeup APIs in render, allocation, feature/time query, compiled
  track ceiling and browser scheduler reachability.

Run focused tests first, then format, locked workspace check/test, warning-denied all-target
Clippy/rustdoc, and workspace/realtime/graph/rack/builtin/scheduler policies. Run Linux x86-64
threaded evidence; compile Linux AArch64/macOS native paths and iOS/Android declared fallback;
compile Wasm scalar and simd128 and prove the linked browser graph contains no scheduler thread or
atomic opcode. Do not expand into device/browser runtime qualification.

## Exactly-once descriptive benchmark

Prepare one real 48-kHz/128-frame sufficiently parallel graph outside timing and render it in
sequential, two-lane and four-lane modes with identical deterministic input/state. Input fill and
output hashing stay outside observations. The no-argument runner owns one untimed warmup total and
measured rounds 1 and 2, emitting exactly six strict records (three modes times two rounds) with
integer nearest-rank percentiles, output identity, selection/shape, zero render/audit errors and
honest CPU/OS/governor/Rust/LLVM/build/background-load metadata. There is no speedup threshold.

Preflight must validate real production reachability, schema/aggregate mutations, output
persistence, status propagation, overwrite refusal and no-retry source with
`workload_launches=0`. Seal candidate/binary/runner/validator/workload hashes. Only root Sol may
then authorize exactly `bash scripts/run-scheduler-benchmark.sh`. Preserve first raw bytes; failure
consumes authorization. Never tune, retry or turn a runner/promotion defect into another timing
attempt.

## Ordered stop conditions

FAIL immediately for a graph/PDC/reduction/hash or bank semantic change; shared mutable state;
worker observers; uncharged parallel storage; unsafe outside core SPSC; allocation/free, OS lock,
blocking/wakeup/thread lifecycle, time/feature query or syscall in render; lost/duplicated jobs;
mid-block re-execution; browser worker/atomic reachability; a compiled track ceiling; benchmark
before authorization; extra warmups/rounds/invocations; timing threshold/tuning; or work beyond the
two-attempt budget. Preserve evidence and create a stateless follow-up for platform integration,
idle policy, extended fault/stress research or optimization rather than widening Issue 009.
