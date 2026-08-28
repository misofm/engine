# 249 Sol-max Linux-audit follow-up: four realtime and lifecycle correctness gaps

One-line summary: Convert the corrected Linux-kernel architecture audit in #248 into four bounded,
independently briefed successor issues without reopening the host-core, BTLV, native-effect, source,
protocol-schema, or observation decisions that survived adversarial review.

## PHILOSOPHY

This is a **tracking and briefing issue, not an implementation issue**. Sol max adversarially
rechecked #248 at Engine `8c2f588200e746d7b7119ef7cafd2315f8e7ea84` and Linux
`1b78070aaef63512688aebfbc82365ef9d6660f1`. The affected Engine files are unchanged at current
`origin/main` `66ede390036448e690d8992569d3e8abcb30b153`. Root independently reproduced each accepted
counterexample. The full evidence and citation corrections remain in #248.

The review changed the original conclusions rather than rubber-stamping them:

- **confirmed:** host-core's common preparation seam, conservative BTLV rules, and native-effect
  preparation/registry boundary;
- **narrowed:** prepared-plan exchange, the source-worker/NAPI analogy, interface ownership, future
  C-ABI evolution, and preparation-resource accounting;
- **overturned as scoped:** a protocol schema rewrite, a new generic observation-site architecture,
  and one centralized lifecycle/failpoint enum.

Those overturned scopes must not reappear in the successors below. In particular,
`protocol/schema.rs` already supplies declarative schema metadata, #143 already supplies stable
effect observation sites, #203 owns measured meter-cost work, #124 owns remaining source-worker
qualification, and ownership belongs in a checked interface/owner DAG rather than a global state
enum.

## WHY THIS MUST SPLIT BEFORE IMPLEMENTATION

The four findings have different owners, invariants, and falsifiers. Combining them would cross core
plan exchange, host lifecycle, CAPI telemetry, and effect-control admission, violating the repository
rule that independently useful product outcomes receive independently closable issues. This tracker
authorizes only creation and Sol approval of four stateless successor briefs. It authorizes no Rust,
C header, test, workflow, ABI, graph, DSP, or host behavior change.

## VERIFIED FINDING A — rejected render commits an unpublished plan generation [P0, S]

`RealtimePlanOwner::render_contiguous` calls `enter_block` before validating time, output shape,
clock overflow, or executor success (`crates/miso-engine-core/src/realtime/plan_exchange.rs:423-444`).
`enter_block` replaces the active plan and commits the old plan to retirement (`:382-412`). If render
then rejects, no `RealtimeRenderReport` escapes. CAPI stores `SharedPlanState.active_epoch` only after
success (`crates/miso-engine-capi/src/runtime/plan.rs:184-212`), while control uses that atom to select
providers and active resource reports (`runtime/control.rs:605-657`). A pending replacement crossed
with a bad timestamp can therefore leave the actual plan at generation N while the provider/report
projection remains at N-1.

Smallest successor: core plan exchange plus CAPI generation projection/tests. Freeze the recommended
contract that a rejected call is not a block boundary.

Required gates:

1. Cross one pending replacement with time discontinuity, output-shape refusal, clock overflow, and
   an injected executor error.
2. Every refusal leaves owner plan/epoch, shared epoch, provider, active report, retirement contents,
   and pending candidate in the frozen pre-boundary state.
3. A corrected retry applies the candidate exactly once.
4. Concurrent resource queries never identify an epoch different from the actual owner.
5. Successful publication retains Release/Acquire handoff and render remains allocation-,
   deallocation-, lock-, syscall-, and wait-free.

Non-goals: resource-ledger refactoring, protocol/schema work, DSP changes, multiple-pending redesign,
or importing Linux seqcount over pointer-bearing ownership.

## VERIFIED FINDING B — render ownership lacks teardown authority [P0, S/M]

Host-core documents destruction/reclamation on control after render quiesces
(`crates/miso-engine-host-core/src/lib.rs:47-70`), but `StartedRenderSession` owns the plan with an
ordinary Drop and only a documented `stop(self)` handoff (`render_session.rs:58-61,160-164`).
`RealtimePlanOwner::Drop` and `PlanRetirer::Drop` dispose values in the caller context
(`core/realtime/plan_exchange.rs:478-510`). Destruction can reach `NativeSourceWorker::Drop`, which
performs stop-and-join (`crates/miso-engine-source/src/native_source.rs:792-828`). `!Send`/`!Sync`
protects affinity/exclusivity; it does not prevent destruction on the callback thread. The C header
requires quiescence but does not state the Rust FFI contract's control/off-render destruction domain.

Smallest successor: render-owner teardown/retirement typestate over public core/host-core owners and
source-worker handoff.

Required gates:

1. Armed callback tests drop/early-return every public render-owner shape and observe zero
   allocation/free, lock, syscall, wait, stop, join, or worker teardown.
2. An explicit stop/retire operation returns complete ownership to a `Send` control-side receipt and
   disposes/joins exactly once after quiescence.
3. Active, pending, and retired plans plus source workers obey the same rule.
4. Compile-fail exclusivity, successful swaps, PCM, and resource reports remain unchanged.
5. The C header and C smoke tests state and preserve the destruction execution domain.

Non-goals: RCU, custom reference counting, a worker-pool redesign, graph/DSP changes, or a general
C-ABI redesign.

## VERIFIED FINDING C — CAPI render telemetry can mix two blocks [P1, S]

The writer stores `render_sample`, then `render_peak_bits`, then increments `render_sequence`
(`crates/miso-engine-capi/src/runtime/plan.rs:226-235`). The reader acquires sequence once, records it
as consumed, then independently loads sample and peak without a closing generation read
(`runtime/control.rs:419-428`). It can observe sequence N, sample N+1, and peak N. Lossy/conflating
semantics permit skipping complete generations, not synthesizing a tuple from two generations.

Smallest successor: CAPI telemetry only, reusing the existing odd/even conflating-slot pattern in
`crates/miso-engine-core/src/realtime/observe.rs:12-18,121-189` or an equivalent non-pointer protocol.

Required gates:

1. Deterministically interleave every writer store and reader load; accept only a complete single
   generation.
2. An odd/in-flight or changed generation retries or returns none under a documented bounded rule.
3. Skipped complete generations are counted/documented.
4. The writer stays wait-, lock-, allocation-, deallocation-, and syscall-free.
5. Mutations removing the opening or closing generation check fail.

Non-goals: reliable telemetry, pointer publication, plan-generation changes, meter DSP changes, or a
new generic observation architecture.

## VERIFIED FINDING D — concurrent drain-to-empty is not capacity-bounded [P1, S]

Each effect-control queue has fixed storage, and `EffectControlLane::stage` drains until `try_pop`
reports empty (`crates/miso-engine-effect-contract/src/live.rs:167-194`). The comment claims queue
capacity bounds the loop (`:27-33`), but SPSC `try_pop` reloads the concurrently advancing producer
cursor whenever its cache appears drained (`crates/miso-engine-core/src/realtime/spsc.rs:421-451`). A
producer can refill slots as the consumer releases them, so one callback can chase the producer
indefinitely. Capacity bounds occupancy, not successful pops over a concurrent interval.

Smallest hard-fix successor: freeze the records eligible at entry or impose a fixed per-call quota in
the shared scalar/bank staging contract.

Required gates:

1. Adversarial concurrent refill proves successful pops and sorted-insertion comparisons never exceed
   the frozen invocation bound.
2. Every record eligible at the boundary retains FIFO/canonical last-wins behavior; later arrivals
   remain for the next boundary with no silent loss.
3. Scalar and bank paths apply the same rule.
4. Render remains allocation-, deallocation-, lock-, syscall-, and wait-free.
5. A mutation restoring drain-to-empty fails.

A separately briefed P2 qualification may then sum/report all per-lane quotas and admit them against
an explicit host-owned next-boundary CPU-work policy. Burst-storage capacity and service budget must
remain distinct. Do not introduce a compiled `MAX_TRACKS` or defer records that were eligible under
the frozen boundary rule.

## PRESERVE THESE ADDITIONAL PATTERNS

- Keep CAPI `miso_engine_v2_query_capabilities` and BTLV `CAPABILITIES_GET`; capability discovery is
  already the correct Linux-like pattern, not a missing feature.
- Treat `PreparedGraphPlan` as repairable pre-bind state and `PreparedRenderPlan` as the opaque sealed
  executable. Do not open a broad graph-typestate refactor without a caller-visible stale-program
  counterexample.
- Future C-ABI work must document whether exact-size V1 stays exact or a later minor introduces a
  prefix-growth rule, then test real old-header/new-library and new-header/old-library pairs. Linux's
  growable syscall struct is an option, not a mandate.
- Preparation accounting must remain off-render and hand immutable facts/explicit retirement receipts
  into render ownership; never attach Drop-coupled ledger permits to render-owned objects.

## GOVERNANCE DEPENDENCY

Current `AGENTS.md:7` forbids legacy/V1 inspection, copying, benchmarking, or inherited architecture.
Provenance metadata says `docs/research/legacy-v2old` contains copied legacy research, and current
`host-core/src/render_session.rs:11-13` attributes the started-session shape to legacy engine-v2-old.
No archived legacy content was inspected during this audit. Existing #144 must record an owner ruling:
quarantine that archive from current authority and independently re-derive retained decisions, or
explicitly amend the mission. Do not silently delete or reinterpret the history.

## TRACKER CLOSE GATES

Close this tracker only when:

1. Findings A-D each have a separate local numbered issue spec and matching open GitHub issue with
   Sol-approved scope, gates, non-goals, and exact ownership;
2. the A-D issue links and intended order are recorded here (A, B, C, D unless Sol reorders them with
   evidence);
3. #144 records the governance ruling above; and
4. GitHub/local issue state is synchronized upstream.

Closing this tracker means the briefs exist. It does not mean any product fix has passed review.

## AUTHORITY

- Parent research record: #248 and its Sol-max addendum.
- Current issue: #249 is the authority for successor creation only.
- No implementation attempt starts from this tracker.
