# 100 Audit: miso-engine-native-scheduler

One-line summary: Audit the native deterministic scheduler: prestarted workers, disjoint outputs, stable reduction order and the deadline-miss gate.

**Authority: GitHub issue #100 and its plan comment.** This file is a stateless pointer, not a
second copy of the brief. The issue body carries the findings with `path:line` evidence; the plan
comment on the issue carries the numbered steps, evals, acceptance checklist and hazards; and the
master plan (the first comment on issue #83) decides everything cross-cutting -- the numeric
contract (D1-D12), the `Lane` trait and its per-operation semantics, the block-kernel contract, the
`miso-engine-math` and `miso-engine-effect-runtime` boundaries, the fixture re-pin policy of §8, the
workstream waves of §9 and the evals of §10. Where this file and those comments disagree, they win
and this file is corrected in the same checkpoint.

Read, in order: `AGENTS.md`; issue #125 (standing instructions for the audit workstream); issue #83
body, master-plan comment and execution-plan comment; then `gh issue view 100` and its plan
comment.

Do not re-decide anything the master plan decides, do not loosen a gate, and do not pin a fixture
from production output: fixtures are regenerated only from an independent `f64` oracle or from the
scalar `Lane` instantiation, with the old-to-new deviation and the audit finding cited in the
commit message.

## Decision record (wave 4, #100 F1-F8, F10, F11)

* **Ownership**. Threads belong to a control-plane `NativeWorkerPoolV1` that is *plan independent*
  and outlives every plan. Its coordinator endpoints travel with the active plan as a
  `WorkerLeaseV1`, handed from the retiring executor to its replacement inside
  `RealtimePlanOwner::enter_block` through two new `PreparedPlanExecutor` methods (`take_handover`
  / `accept_handover`, moves only, a refusal is given back to the retiring plan). A structural
  change no longer spawns and joins N-1 threads. `NativeSchedulerV1` owns no thread.
* **Idle policy: spin then park.** Workers spin for a calibrated budget while a block is open and
  park between blocks. The coordinator issues **at most one** `Thread::unpark` per rendered block
  from `wake_root`; workers wake their binary-tree children. This is the single documented
  render-thread syscall (`docs/REALTIME_DEPENDENCY_POLICY.md`, "Issue 100 worker idle policy and
  the single wake"), and `AGENTS.md`'s render prohibition sentence is amended to name it. Issue
  009's "parking is a follow-up" hazard note is superseded.
* **Command issue order is descending worker id**, and it is load-bearing for the wake tree: a
  worker wakes its children as soon as it takes its own command, so ascending order could wake a
  child before that child's command existed.
* **Bounded recovery.** `recover_issued` spins a calibrated budget and then declares the worker
  dead *for that block*, leaves its parcel trapped, mutes every edge sourced from it, and finishes
  the block on the coordinator. Two amendments to the plan's §3, both strictly safer: the budget is
  one full render quantum (floor `MINIMUM_RECOVERY_NS`, 2 ms) rather than half a quantum, because
  parking adds an operating-system wake latency the quantum does not bound; and "dead" is cleared
  when the worker finally returns its parcel, because a false deadline miss must cost one degraded
  block rather than the life of the lease. `NativeSchedulerConfigV1::with_recovery_deadline_ns`
  lets a qualification harness that measures determinism rather than deadlines opt out.
* **Data movement: destination pulls (F1/F8).** One plan-owned `DisjointArena`
  (`crates/miso-engine-core/src/realtime/disjoint.rs`, the only `unsafe` in this job) holds every
  buffer, every op output has a globally unique buffer for the life of the plan, and a consumer
  reads its producers in place on its own worker. The coordinator copies nothing between waves;
  only a delayed edge copies, into a staging buffer the *consuming* parcel owns and stages itself
  through the same `RuntimeOp::staged` list the sequential executor uses. `stage_wave`,
  `observe_wave` and `StageOperation` are deleted. Both executors now drive the same lease API, so
  "where the audio lives" has one implementation as well as "what happens to it".
* **Arena invariants** I1-I4 are proved once at bind by `ArenaLeaseSetBuilder::finish` and do not
  depend on any worker being on time; see the module documentation and the policy doc.
* **Observers run inside the owning parcel**, on the worker that rendered the node. An observer is
  still invoked exactly once per block per node with exactly the audio the sequential executor
  sees; cross-node arrival order is unspecified, and the q128 transcript is sorted before it is
  compared or hashed.
* **Partitioning by prepared cost (F2)**, not unit count: `partition_weighted_units_v1` is greedy
  longest-processing-time-first over `max(1, width * slots) + incoming_main_edges`, laid out
  contiguously so the canonical cover still holds, with the session output's unit pinned to
  partition 0 (the coordinator's own lane, so the host copy-out can never read a trapped parcel).
* **Target gate removed (F6).** `UnsupportedTarget` fires only on `wasm32`; the parallel path
  compiles and is unit-tested on every native target.
* **Fault injection is dev-only (F7).** The scheduler feature is renamed `fault-injection` and may
  be requested only from a `dev-dependencies` table; `scripts/check-scheduler-policy.sh` enforces
  it, and `scripts/test-scheduler-policy.sh` proves the enforcement red.
* **Deleted**: the completion-acceptance-order API and its 32-perturbation fixture matrix (bits
  never depended on acceptance order, so the matrix was vacuous), `partition_stable_units_v1`,
  `stop_workers`, the per-plan thread spawn and its generation preflight, and the wasm
  `render_wave` body.
* **Not in this job**: `spsc.rs` cache-line padding (#84 F6), thread priority/affinity and platform
  workgroups (#106), the dependency-counter DAG scheduler (successor issue opened with the measured
  serial fraction).
