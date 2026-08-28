# 248 Linux-kernel architecture audit: boundary, lifecycle, UAPI, and execution-domain lessons

One-line summary: Evidence-backed Linux-source comparison, corrected by a Sol-max adversarial pass,
that preserves confirmed Engine V2 seams, overturns three over-broad follow-ups, and records four
bounded correctness/realtime findings with objective successor gates.

**Authority: GitHub issue #248 plus its Sol-max adversarial addendum.** The original issue body and
the second-pass record below together are the complete stateless research and decision record. The
addendum is controlling wherever it narrows or overturns the original body. Read `AGENTS.md`, this
file, and `gh issue view 248 --repo misofm/engine-v2 --comments` before using it.

This record authorizes no production-code change and no cross-cutting implementation branch. A
successor must receive its own Sol-approved stateless brief before Terra implementation and Sol
adversarial review. Preserve the confirmed host-core, BTLV, and native-effect seams. Do not duplicate
the existing #124, #130, #163, #203, or #239–#246 scopes. The render-plan exchange is preserved on
successful boundaries but now has two explicit lifecycle/correctness qualifications below.

## Sol-max adversarial pass — 2026-08-28

### Scope and evidence hygiene

Sol max independently reviewed the original P1–P5 and F1–F6 claims read-only. Root separately
reproduced every accepted counterexample before recording it. Exact baselines:

- Engine `origin/main`: `8c2f588200e746d7b7119ef7cafd2315f8e7ea84`;
- Linux: `1b78070aaef63512688aebfbc82365ef9d6660f1`;
- no Engine production code, test, workflow, or public interface was changed by either pass.

The original issue overstated its citation hygiene. Several Engine links named the current-main
commit but retained line ranges from the older audit branch. The underlying P1/P4 claims survive,
but those links did not prove what their captions said. Correct current-main anchors are:

- P1 executor/plan/render: `core/realtime/plan.rs:168-182,419-481,711-775`;
- P1 exchange roles/swap/render: `core/realtime/plan_exchange.rs:25-147,332-412,446-462`;
- P4 factory/prepared processors/registry: `effect-contract/src/lib.rs:1408-1463,1483-1594,1817-1861`;
- P5 worker creation/read sizing/idle: `source/src/native_source.rs:1132-1150,1438-1467,1533-1744`;
- F2 exact-size checks: `capi/src/ffi.rs:51-55,208-223,255-264,441-450,751-760,873-884`.

This correction is material to the evidence record even where it does not change the verdict.

### Adversarial verdict on the original claims

| Claim | Verdict | Controlling disposition |
|---|---|---|
| P1 prepared-plan exchange | **NARROWED** | Successful block-boundary reservation, deferral, move, and retirement mechanics are sound. Rejected renders can nevertheless commit a swap without publishing the new generation, and terminal destruction is not execution-context safe. See N1/N2. |
| P2 host-core seam | **CONFIRMED** | Host-core remains the single compile/prepare pipeline. Do not infer that adapters may depend *only* on it: CAPI and web have intentional direct contract dependencies. |
| P3 BTLV wire rules | **CONFIRMED** | Versioning, bounded framing, unknown-field rules, PCM exclusion, request/revision semantics, and capability discovery remain good preservation constraints. |
| P4 native-effect seam | **CONFIRMED** | Off-render factory preparation and prepared scalar/bank ownership remain narrow and correct; dynamic dispatch by itself is not a defect. |
| P5 native source worker | **NARROWED** | Shared worker, bounded per-job drains, fixed scratch, and park/unpark are sound. They are not equivalent to NAPI's single global poll budget. #124 deliberately remains open for its existing qualification/tooling work. |
| F1 stability/ownership inventory | **NARROWED** | Create one checked owner/interface/layer DAG, but encode legitimate cross-layer edges rather than enforcing a strict linear dependency chain. Absorb F6's ownership-map portion here. |
| F2 C ABI extension policy | **NARROWED** | Exact-size V1 is already the implemented policy. The missing decision is how a future minor extends it, plus real old-header/new-library and new-header/old-library gates. Linux's prefix-growth rule is one option, not an automatic transplant. Preserve capability discovery. |
| F3 managed preparation resources | **NARROWED** | Rust ownership already supplies most cleanup. Use an off-render preparation-accounting transaction and explicit retirement receipts; render-owned objects carry immutable facts only. Do not attach Drop-coupled ledger permits to render-owned objects. Split adapter child domains if they expand the slice. |
| F4 protocol schema/source-of-truth | **OVERTURNED as written** | `protocol/schema.rs` is already a declarative registry delivered by #102. Line/function counts do not prove an architecture defect. `MockProvider` is production CAPI state, so it cannot simply be test-gated. Residual work, if justified, is two small scopes: isolate conformance/corpus builders; separately rename/split the bounded production provider from fixture constructors. No schema rewrite. |
| F5 stable observation sites | **OVERTURNED as a new issue** | #143 already shipped stable effect tap IDs, declared menus, structural zero, and an O(1) unarmed gate; track/send tap tokens also exist. #203 owns measured meter-cost work. `ProcessReport` counters are a different semantic class and need a consumer-specific brief, not reuse of sample-observation lanes. |
| F6 centralized lifecycle/failpoints | **OVERTURNED as one issue** | Ownership belongs in F1. A workspace-wide lifecycle enum duplicates subsystem state and is not a smallest slice. Add local failpoints only to the lifecycle successor that needs them. |

Capability discovery is an additional **preservation pattern**, not new work: CAPI exposes
`miso_engine_v2_query_capabilities` (`capi/src/ffi.rs:186-223`), and BTLV exposes
`CAPABILITIES_GET` with typed schema fields (`CONTROL_BTLV_V1.md:27-43`,
`protocol/src/schema.rs:676-725`). This is the useful analogue to generic-netlink family/policy
discovery in Linux `include/uapi/linux/genetlink.h:40-68`.

## New findings

### N1 — A rejected render can commit a plan transition without publishing its generation [P0, S]

`RealtimePlanOwner::render_contiguous` calls `enter_block` before validating time, output shape,
clock overflow, or executor success (`plan_exchange.rs:423-444`). `enter_block` replaces the active
plan and commits the old plan to retirement (`:382-412`). On a later render error, no
`RealtimeRenderReport` escapes. CAPI publishes `SharedPlanState.active_epoch` only after success
(`capi/runtime/plan.rs:184-212`), while control uses that atom to select providers and resource
reports (`capi/runtime/control.rs:605-657`). A queued replacement crossed with a discontinuous time
therefore leaves the actual owner at generation N while provider/report state remains at N-1.

This is a concrete correctness counterexample, not an analogy. The smallest successor is core plan
exchange plus the CAPI generation projection/tests. Freeze the recommended rule that a rejected call
is not a block boundary, then prove:

1. queued replacement crossed with time discontinuity, output-shape refusal, clock overflow, and an
   injected executor error leaves owner plan/epoch, shared epoch, provider, active report, retirement
   contents, and pending candidate in the documented pre-boundary state;
2. a corrected retry applies the candidate exactly once;
3. concurrent resource queries never identify a generation different from the actual owner; and
4. successful publication retains its Release/Acquire handoff and render remains allocation-,
   deallocation-, lock-, syscall-, and wait-free.

Linux seqcount documentation reinforces the need for a coherent generation, but it also forbids
pointer-bearing snapshots and warns about realtime-reader livelock (`locking/seqlock.rst:10-36`). Do
not port seqlock over plan ownership; validate/admit before swap or expose one explicit generation
commit result.

### N2 — Render ownership does not carry safe teardown authority [P0, S/M]

Host-core says destruction/reclamation runs on control after render quiesces
(`host-core/src/lib.rs:47-70`), yet `StartedRenderSession` owns a plan with an ordinary Drop and only a
documented `stop(self)` handoff (`render_session.rs:58-61,160-164`). `RealtimePlanOwner::Drop` and
`PlanRetirer::Drop` dispose pending/active/retired values in their caller's context
(`plan_exchange.rs:478-510`). That can reach `NativeSourceWorker::Drop`, which calls a blocking
stop-and-join (`source/src/native_source.rs:792-828`). `!Send`/`!Sync` protects affinity and
exclusivity; it does not prevent destruction on the callback thread. The public C header requires
quiescence but, unlike the Rust FFI docs, does not say destroy is control/off-render
(`capi/include/miso_engine_v2.h:20-32`; `capi/src/ffi.rs:948-977`).

Linux makes execution-context effects explicit: reclaim-capable allocation may sleep while atomic
contexts require a different contract (`core-api/memory-allocation.rst:39-50`), and the last
`kref_put` runs its release callback in the caller (`core-api/kref.rst:65-73`). The lesson is context
and lifetime typing, not importing kernel refcounts.

Smallest successor: render-owner teardown/retirement typestate over public core/host-core owners and
source-worker handoff. Gates:

1. armed callback tests drop/early-return every public render-owner shape and observe zero
   allocation/free, lock, syscall, wait, stop, join, or worker teardown;
2. an explicit stop/retire operation returns complete ownership to a `Send` control-side receipt,
   disposed and joined exactly once after quiescence;
3. active, pending, and retired plans plus source workers obey the same rule;
4. compile-fail exclusivity, normal swaps, PCM, and resource reports remain unchanged; and
5. C header and C smoke tests state and enforce the destruction execution domain.

### N3 — CAPI's render telemetry snapshot can mix two blocks [P1, S]

The writer stores `render_sample`, then `render_peak_bits`, then increments `render_sequence`
(`capi/runtime/plan.rs:226-235`). The reader acquires the sequence once, marks it consumed, and then
loads sample and peak without a closing generation read (`capi/runtime/control.rs:419-428`). A reader
can observe sequence N, then sample N+1 after the next writer begins, then peak N before that writer
stores it. Lossy/coalescing semantics permit skipping complete generations, not fabricating a tuple
from two generations.

Use the existing odd/even conflating-slot pattern (`core/realtime/observe.rs:12-18,121-189`) or an
equivalent non-pointer snapshot. Deterministic interleaving tests must cut between every store/load
and accept only complete generations; in-flight/changed generations retry or return none; skipped
complete generations are counted/documented; and the render writer remains wait-, lock-, allocation-,
and syscall-free. A mutation removing either generation check must fail.

### N4 — A bounded queue does not bound a concurrent drain-to-empty invocation [P1, S]

Each effect queue is sized independently at `min(depth, automation_capacity)`
(`effect-compiler/src/prepare.rs:1232-1241,1248-1296`). `EffectControlLane::stage` uses drain-to-empty
and performs sorted insertion (`effect-contract/src/live.rs:167-194,224-260`); scalar and bank lanes
do this at block entry (`graph/src/runtime.rs:1113`; `rack/src/lib.rs:882`). The source comment claims
the loop is bounded by queue capacity (`live.rs:27-33`), but SPSC `try_pop` reloads the live producer
cursor whenever its cache appears drained (`core/realtime/spsc.rs:421-451`). A concurrent producer
can refill each slot after the consumer releases it, so one invocation can keep chasing the producer.
Storage capacity bounds occupancy, not the number of successful pops during a concurrent interval.
That is a concrete violation of the no data-dependent unbounded-call rule.

Smallest hard-fix successor: freeze the records eligible at entry or impose a fixed per-call quota.
Adversarial producer-refill stress must prove successful pops/comparisons never exceed it; every
record eligible at the boundary retains FIFO/canonical last-wins semantics; later arrivals remain for
the next boundary; scalar and bank paths agree; and render stays allocation/free/lock/syscall/wait
free. A mutation restoring drain-to-empty must fail.

There is then a separate P2 qualification question: the aggregate of all per-lane quotas is still a
next-boundary CPU-work budget, distinct from burst-storage capacity. Report/sum it at preparation and
reject above explicit host policy, or brief that as its own bounded successor. Linux NAPI's explicit
poll `budget` (`networking/napi.rst:61-87`) is the relevant distinction. Do not introduce a compiled
`MAX_TRACKS`, silently lose records, or move records that were already eligible at the boundary.

### Design note — keep the graph draft/sealed distinction explicit, but do not open an issue yet

`PreparedGraphPlan` exposes repairable structural fields and re-lowers at bind
(`graph/src/lib.rs:335-358,827-867`). Adversarial review found this intentional: consuming bind
validates and derives a fresh execution program before producing opaque realtime state
(`:630-718,945-1041`), and the builtins artifact exposes only shared access. Draft-to-executable
typestate therefore already exists semantically. The construction-time `program()` cache can become
stale after direct mutation, but no accepted inconsistent-bind or caller-harm counterexample was
found. Preserve the pattern; promote a sealing refactor only with a concrete stale-evidence failure.

### Governance conflict discovered without inspecting legacy content

Current `AGENTS.md:7` forbids inspecting, copying, benchmarking against, or inheriting architecture
from legacy/V1. Provenance metadata alone states that `docs/research/legacy-v2old` contains copied
legacy research (`PROVENANCE.md:1-10`), and current `host-core/src/render_session.rs:11-13` explicitly
attributes the `StartedRenderSession` shape to legacy engine-v2-old. GitHub #144 remains the existing
owner. This pass did **not** inspect the archived files or the issue body.

Do not silently delete or reinterpret that history. #144 needs an owner ruling under the current
mission: quarantine the archive from current research authority and independently re-derive any
retained decisions from primary sources/objective gates, or explicitly amend the mission. Until that
ruling, no successor may cite the archive as authority.

## Revised implementation order

1. N1 rejected-boundary generation split — concrete P0 correctness counterexample.
2. N2 render-owner teardown authority — P0 realtime/lifecycle hole, before native/mobile adapters.
3. F2 future ABI extension ruling and real cross-version harness — no layout/symbol change.
4. N3 coherent render telemetry snapshot.
5. F1 owner/interface/layer DAG, absorbing F6 ownership; no global lifecycle enum.
6. F3 off-render accounting transaction only after N2 fixes teardown semantics.
7. N4 bounded live-control boundary drain; split aggregate service admission/qualification if needed.
8. Optional F4 residuals as separate protocol-only scopes; no schema rewrite.
9. Existing owners only: #124 for source qualification/tooling and #203 for meter-cost work. No F5
   successor and no combined F6 successor.

The #144 governance ruling is an authority prerequisite, not an implementation tranche. Every new
product successor above still requires a separate stateless issue brief and the normal three-attempt
workflow. This research addendum itself authorizes no code change.
