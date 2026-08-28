# 250 Ardour architecture audit: realtime execution, continuity, hosts, and recovery lessons

One-line summary: Adversarial source comparison of Ardour 9.8-era code and Engine V2's common
mixing/mastering scope, preserving V2's stronger prepared-plan, deterministic-DSP and semantic
contracts while recording six bounded gaps in live-state continuity, host integration, fault
observability, persistence, cohort efficiency and independent graph verification.

**This is a completed research and decision record, not an implementation issue.** It authorizes no
production-code change and no cross-cutting "make V2 like Ardour" branch. Each residual must receive
its own smallest-closable Sol-approved stateless brief before Terra implementation and Sol
adversarial review. Existing #124, #130 and #203 own their current source-worker, native-scheduler
and meter-cost scopes; do not duplicate them from this record.

## Authority, pins and method

The read-only comparison used these exact baselines:

- Engine V2: [`8c2f588200e746d7b7119ef7cafd2315f8e7ea84`](https://github.com/misofm/engine-v2/tree/8c2f588200e746d7b7119ef7cafd2315f8e7ea84).
  The audited checkout's tree was byte-identical to that then-current `origin/main` merge commit.
- Ardour: [`34f8b1a710ca065be23425ad6292ee92b24d7167`](https://github.com/Ardour/ardour/tree/34f8b1a710ca065be23425ad6292ee92b24d7167),
  locally described as `9.8-2-g34f8b1a710`.
- Recording baseline: current Engine `origin/main` is `66ede390036448e690d8992569d3e8abcb30b153`;
  its only changes from the audited Engine pin are the accepted #241 stem-identity files and
  workspace registration, not the runtime files cited below.

Scope was restricted to the overlap: audio graph preparation/execution, DSP layout, source
streaming, plugin-delay compensation, live structural change, host/device boundaries, effect fault
handling and session-state durability. Ardour's timeline editor, GUI, recording workflow, MIDI,
control surfaces, plugin breadth and delivery features are not product requirements merely because
they exist there.

No cross-project benchmark or listening test ran. Different languages, plugin sets, routing shapes,
build flags and workloads make raw timing non-comparable. Performance statements below are therefore
architectural/capacity claims or direct code facts, never a claim that either whole product is
faster. Focused Engine library tests passed at the audit pin:

```text
cargo test -p miso-engine-core -p miso-engine-graph-compiler \
  -p miso-engine-source -p miso-engine-host-core --lib

154 passed; 0 failed
```

The method was adversarial in both directions: identify a difference, find the constraints that
make each design rational, inspect the concrete implementation and tests, then adopt an outcome only
where it fits V2's mission and objective realtime rules.

## Executive verdict

Do **not** re-architect V2 around Ardour. V2's core shape is the better fit for an agent-first,
headless, cross-target engine:

1. one exclusively owned, structurally immutable prepared plan with bounded block-boundary exchange;
2. exact off-render resource admission and deterministic graph/PDC compilation;
3. homogeneous across-track AoSoA effect cohorts with compile-time target selection;
4. generation-tagged bounded source rings with explicit underrun semantics; and
5. strict versioned TOML plus a revision-aware binary semantic protocol.

Ardour is materially ahead in four production outcomes that V2 either lacks or has not yet proven:

1. compatible DSP objects survive graph reordering instead of being reconstructed;
2. native device/backend rate, block, port and latency state is integrated end to end;
3. session saves have autosave, backup, temporary-file replacement and recovery/version handling; and
4. route work and disk I/O can use configurable prestarted worker sets.

The correct adoption is those outcomes under V2's stricter ownership and realtime contracts, not
Ardour's callback try-lock, OS-semaphore, shared-ownership, dynamically resized queue or fallback
render-thread deletion mechanisms.

## Preservation decisions — where V2 is stronger for this mission

### P1 — Preserve prepared ownership and bounded plan exchange

`PreparedRenderPlan` owns the mutable executor and arena, is `Send` but deliberately not `Sync`, and
is prepared off render ([`plan.rs:168-182,420-481`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-core/src/realtime/plan.rs#L168-L182)).
The exchange reserves retirement before replacement, defers a swap when retirement is full, moves a
handover without allocating, and reclaims through the control-side owner
([`plan_exchange.rs:340-412,496-510`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-core/src/realtime/plan_exchange.rs#L340-L412)).
`StartedRenderSession` additionally pins the started plan to the attested render thread and refuses
`Send`/`Sync` ([`render_session.rs:1-30,43-90`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-host-core/src/render_session.rs#L1-L30)).

Ardour instead takes a process mutex with `TryLock` in the callback and emits an xrun/silence when it
cannot acquire it ([`audioengine.cc:227-268`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/audioengine.cc#L227-L268)).
Its graph workers coordinate through OS semaphores, the callback waits for completion, and graph
preparation can resize an MPMC queue whose `reserve` deletes and allocates storage
([`graph.cc:214-228,288-345,432-453`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/graph.cc#L214-L228),
[`mpmc_queue.h:41-76`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/pbd/pbd/mpmc_queue.h#L41-L76)).
Its `rt_safe_delete` falls back to immediate deletion when Butler delegation fails
([`rt_safe_delete.h:27-36`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/ardour/rt_safe_delete.h#L27-L36)).
Those are understandable compromises in a large plugin DAW, but they are weaker than V2's approved
callback contract and are not patterns to import.

Qualification: closed audit #248 later found that rejected renders and terminal destruction still
need explicit generation/teardown corrections. Preserve the successful reservation/move/retirement
shape subject to [#248 N1/N2](https://github.com/misofm/engine-v2/issues/248); this audit neither
overturns those findings nor silently folds them into F1 below.

### P2 — Preserve V2's deterministic graph, PDC and requested-delay separation

V2 computes exact per-edge compensation in one checked longest-path pass, sorts route/delay reports
by stable ID, propagates tails with checked arithmetic, and deliberately excludes requested
multi-mic track delay from PDC
([`pdc.rs:1-23,38-163`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-graph-compiler/src/pdc.rs#L1-L23)).
Typed send taps and sidechains therefore participate in the same deterministic DAG semantics.

Ardour's generality is real, but its route/send latency propagation may restart up to four times
after send changes ([`session.cc:7326-7372`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/session.cc#L7326-L7372)),
and its own plugin code notes that runtime latency changes are not click-free
([`plugin_insert.cc:895-914`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/plugin_insert.cc#L895-L914)).
That is rational for opaque plugins whose latency can change while running; it is not an argument to
weaken V2's fixed prepared-latency contract.

### P3 — Preserve target-pinned AoSoA SIMD; use route-level parallelism only when measured

V2 selects scalar/Wasm SIMD4/AArch64 SIMD4/native SIMD8 at compile/prepare time and attests the
pinned x86-64-v3 artifact once, with no render-time CPU dispatch
([`backend.rs:1-28,82-128`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-lane/src/backend.rs#L1-L28)).
Its production graph fuses homogeneous track-local effects into across-track bank chains, then
currently executes those units sequentially
([`graph/lib.rs:1385-1424`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-graph/src/lib.rs#L1385-L1424)).

Ardour chooses generic buffer kernels at startup from AVX-512/FMA/AVX/SSE/NEON/default function
pointers ([`globals.cc:200-307`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/globals.cc#L200-L307))
and runs ready route nodes on a prestarted worker graph
([`graph.cc:103-140,288-345`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/graph.cc#L103-L140)).
That is a sensible design for heterogeneous opaque plugin chains and gives Ardour multicore graph
capacity V2's current production executor does not have. It does not establish a speed ranking.
Open [#130](https://github.com/misofm/engine-v2/issues/130) already owns V2's dependency-counter DAG;
keep it dormant until a named native/cloud budget fails, then preserve preallocation, disjoint writes,
stable node-ID reduction and the single-thread correctness fallback rather than porting Ardour's
semaphore/shared-pointer scheduler.

### P4 — Preserve V2's source-ring semantics; qualify the single worker before pooling

V2 starts one shared worker for the prepared source set and services every bounded source job in
order ([`native_source.rs:1101-1150,1698-1745`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-source/src/native_source.rs#L1101-L1150)).
The surrounding source contract remains stronger for V2's scope: bounded SPSC PCM, generation-tagged
seeks, positive-zero underrun with counters, one source consumption followed by fanout, and memory
independent of stem duration.

The single loop creates a plausible head-of-line risk when one reader/decode call is slow. Ardour has
a configurable prestarted `IOTaskList` and distributes queued I/O functions across its workers
([`io_tasklist.cc:42-101,103-128,162-185`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/io_tasklist.cc#L42-L101)).
This proves a mature alternative exists, not that V2 needs it. Open
[#124](https://github.com/misofm/engine-v2/issues/124) owns the decision: first freeze a many-source
workload with a deliberately slow/heterogeneous reader and assert deadline, underrun, seek-storm,
idle-CPU and memory budgets. Add a caller-configurable bounded pool only if that gate fails, with
preallocated assignments and exactly one producer owner per source.

### P5 — Preserve strict canonical sessions and revision-aware transactions

V2 validates before producing one canonical TOML form
([`canonical.rs:1-26`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-session/src/canonical.rs#L1-L26))
and exposes canonical snapshots through the same revision-aware protocol that applies structural
transactions. That is a better semantic authority for agents than copying Ardour's monolithic XML
object serialization. F4 below concerns durable storage around canonical bytes, not changing their
schema ownership or relaxing unknown-field rejection.

## Findings and bounded successor briefs

Priority is architectural risk, not permission to start every item. Each finding is independently
closable; do not combine them into one implementation branch.

### F1 — Preserve compatible DSP state across structural plan replacement [P0, M]

**Evidence.** A CAPI structural command prepares a complete fresh runtime and plan, with source state
explicitly reset at the replacement boundary
([`control.rs:660-765`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-capi/src/runtime/control.rs#L660-L765),
[`runtime/tests.rs:569-584`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-capi/src/runtime/tests.rs#L569-L584)).
Core can move a generic `ExecutorHandover`, but production `GraphExecutor` implements neither
`take_handover` nor `accept_handover`; only the sample clock is adopted. Consequently an unrelated
structural edit reconstructs unchanged effect envelopes, filter histories, lookahead/delay rings and
tails. This is deterministic but musically discontinuous.

Ardour rebuilds a `GraphChain` from the existing `Route`/`GraphNode` shared objects
([`session.cc:2563-2587,2650-2683`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/session.cc#L2563-L2587),
[`graph.cc:604-660`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/graph.cc#L604-L660)).
The useful pattern is identity-preserving graph metadata replacement, not shared ownership itself.

**Smallest closable slice.** Add an off-render stable-instance compatibility map for unchanged native
effects, then transfer ownership of compatible state at the block boundary through a bounded
move-only graph-executor handover. Compatibility must include stable effect instance ID, exact bytes/
contract major, state-layout version, sample rate, quantum, quality, ports/link mode and any
algorithm-specific state shape. Reset only new, removed, reordered-incompatibly or changed instances.
Keep the currently explicit source-reset policy out of this first slice.

**Acceptance.** An unrelated route/track edit during sustained EQ, compressor, limiter and delay-tail
fixtures matches a no-edit reference from the replacement sample onward within each effect's frozen
identity tolerance; unchanged fixed-latency impulses do not move or click. Every incompatible field
mutation resets only that instance. Boundary work performs zero allocation/free/drop/lock/syscall/
wait and never copies an unbounded delay/reverb buffer. Failure before publication leaves the old
state owner intact; successful publication moves each state exactly once and retirement drops each
old/new owner exactly once off render. Preserve #248 N1/N2 as separate prerequisites where needed.

### F2 — Resolve the native/mobile host mission contradiction, then close the real device boundary [P0, S then L]

**Evidence.** The repository mission requires native/cloud embedding, iOS, Android and browser Wasm,
but the native and mobile crates are shells that explicitly defer platform audio callbacks
([`host-native/main.rs:1-29`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/hosts/miso-engine-host-native/src/main.rs#L1-L29),
[`host-mobile/lib.rs:1-26`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/hosts/miso-engine-host-mobile/src/lib.rs#L1-L26)).
Closed [#23](https://github.com/misofm/engine-v2/issues/23) originally specified native iOS/Android
adapters, then was closed with the decision that mobile support is browser-based and native embedding
is out of current product scope. Both cannot remain authoritative.

Ardour integrates backend capture/playback latency ranges, external port latency and resampler
latency ([`port.cc:506-527,588-624,627-690`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/port.cc#L506-L527))
and has backend latency callbacks. V2's internal PDC cannot report or compensate a boundary it does
not model.

**Smallest first slice.** Make one explicit product decision: either amend the mission/scope to say
native mobile embedding is not a launch target, or rebrief #23 as bounded platform issues. Do not
write adapters while the authority conflicts.

If native/mobile remains required, split implementation by host. Each adapter must negotiate the
actual device rate/block size, bridge variable/max host blocks to the fixed engine quantum without
render allocation, expose device + bridge + engine latency, reject unsupported actual rates or
reprepare explicitly, handle route/device reconfiguration, set platform thread priority/workgroups,
and publish bounded xrun/deadline telemetry. iOS and Android device evidence are separate product
slices; neither should hold the other's usable adapter open.

### F3 — Correct and surface effect-recovery counters [P1, S/M]

**Evidence.** `ProcessReport` says every recovery counter counts blocks, never samples
([`effect-contract/lib.rs:986-1007`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-effect-contract/src/lib.rs#L986-L1007)).
Production scalar graph execution drops the returned report
([`graph/runtime.rs:1074-1103,1133-1166`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-graph/src/runtime.rs#L1074-L1103)),
and the rack deliberately drops `BankProcessReport`
([`rack/lib.rs:520-553`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-rack/src/lib.rs#L520-L553)).
Soft clip increments by `frames` on one failed block
([`soft-clip/lib.rs:960-977,1012-1045`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-soft-clip/src/lib.rs#L960-L977)),
and gate/expander does the same per failed lane while its test freezes `FRAMES`
([`gate-expander/lib.rs:750-782,1441-1448`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-gate-expander/src/lib.rs#L750-L782)).
Recovery exists, but production cannot observe it and two implementations violate the unit contract.

**Smallest closable slice.** First correct all launch effects to increment block counters by exactly
one per rejected channel/lane block and update fixtures/mutations. Then accumulate saturating,
preallocated per-instance counters through scalar and bank execution and expose a bounded snapshot
only after render is disarmed or through the existing typed counter transport. Do not log from render,
reuse sample-observation lanes, or fold this into #203's meter benchmark.

**Acceptance.** Scalar and W4/W8 injected failures produce identical per-instance block counts;
partitioned calls count calls rather than frames; unaffected lanes stay zero; overflow saturates;
no report is discarded in production; disabled telemetry changes neither topology, PCM nor realtime
trace; typed snapshots identify stable instance/channel and define reset/read semantics.

### F4 — Add durable canonical-session persistence before incompatible schema evolution [P1, M]

**Evidence.** V2 owns strict canonical bytes but no engine-owned atomic save, backup, pending recovery
or migration tool. Ardour autosaves dirty non-recording sessions, backs up the previous state, writes
to a temporary file and renames it into place
([`session_state.cc:678-683,786-930`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/session_state.cc#L678-L683)).
On load it detects pending crash state and preserves a pre-upgrade backup for older session versions
([`session_state.cc:1200-1235,1259-1299`](https://github.com/Ardour/ardour/blob/34f8b1a710ca065be23425ad6292ee92b24d7167/libs/ardour/session_state.cc#L1200-L1235)).

**Smallest closable slice.** First decide and document whether durable files belong to an Engine
sidecar/SDK or a host application; keep filesystem I/O out of the engine runtime ABI. Before schema
V2 or the first user-facing project writer, add a utility around canonical TOML with same-directory
temporary write, flush/error handling, atomic replacement where the platform supports it, prior-good
backup, stale-temp/pending recovery policy and an offline V1 -> V2 migration command that never
silently rewrites the only copy. Do not copy Ardour's object-graph XML serialization.

**Acceptance.** Fault injection at every open/write/flush/rename/backup boundary leaves either the
old complete canonical snapshot or the new complete snapshot, never a torn accepted file; recovery
selection is deterministic; unsupported future schema is refused; migration preserves the original
and produces canonical idempotent output. Filesystem durability claims are stated per platform rather
than inferred from rename alone.

### F5 — Remove prepare-time bypass from cohort identity without losing warm state or exact latency [P1, M/L]

**Evidence.** The source already records the defect and target design: `bypass` is per-instance
configuration but remains in `EffectProgramKey`, so one bypassed lane splits a homogeneous cohort
and forces structural rebuilding
([`effect-contract/lib.rs:911-960`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-effect-contract/src/lib.rs#L911-L960)).
The documented correction is a warm wet path plus per-lane bitwise select against an exactly
latency-matched dry path; arithmetic mixing is not bit-equivalent for signed zero. Ardour supplies no
better engine-specific answer here.

**Smallest slices.** Do not remove the key field first. Add the shared latency-matched per-lane bypass
shunt/mask contract, migrate bank kernels in independently closable effect-family slices with exact
scalar/bank/state tests, then remove `bypass` from cohort identity only when every bank-eligible
launch effect supports it. Dynamic-rack/live bypass semantics and PDC must remain unchanged.

**Acceptance.** Mixed bypass states remain one cohort; wet state advances while bypassed; re-enable
is continuous; bypassed PCM matches exact delayed dry bits including signed zero; latency/PDC and
state payloads are unchanged; scalar/W4/W8/Wasm tolerances remain frozen; no structural compile is
needed for a bypass toggle.

### F6 — Restore an independent test-only scalar graph oracle [P1, M]

**Evidence.** The fifty-seed graph corpus still binds and renders complex DAGs, but its own comment
states that removal of the native executor also removed the cross-executor oracle
([`graph/lib.rs:3428-3467`](https://github.com/misofm/engine-v2/blob/8c2f588200e746d7b7119ef7cafd2315f8e7ea84/crates/miso-engine-graph/src/lib.rs#L3428-L3467)).
This matters because production lowering now combines level-major scheduling, buffer coloring,
aliasing, bank-chain merging, gather/scatter redirection, PDC and stable reductions. Many component
fixtures exist, but "renders nonsilent" is not an output-correctness oracle for their composition.

**Smallest closable slice.** Build a test-only uncolored scalar interpreter directly from normalized
graph semantics: one dedicated planar buffer per semantic node/edge as needed, simple per-sample
reference delays, stable edge-ID reductions, and scalar prepared effects or frozen transparent mock
processors. It must not call production coloring, aliasing, cohort-run, gather/scatter or route-fold
helpers. Compare production PCM and timing across the existing seeded corpus; keep it out of release
artifacts and benchmark authority.

**Acceptance.** Mutations to schedule order, buffer reuse, alias lifetime, bank membership,
gather/scatter target, per-edge PDC and reduction order each fail against the independent oracle;
the oracle itself has analytic tiny-graph fixtures; deterministic seeds and corpus shape are frozen;
native/Wasm release packages do not include it.

## Existing issue ownership — do not duplicate

| Concern surfaced by this comparison | Existing authority / action |
| --- | --- |
| Native source worker count, idle policy and duplication | [#124](https://github.com/misofm/engine-v2/issues/124). Add the slow-reader/many-source qualification there if absent; create no second pool issue. |
| Native multicore dependency execution | [#130](https://github.com/misofm/engine-v2/issues/130). Start only after a named budget fails; preserve deterministic reductions and browser single-thread fallback. |
| Meter/observation cost | [#203](https://github.com/misofm/engine-v2/issues/203). F3 is typed effect-fault accounting, not meter samples and not another #203 timing run. |
| Native iOS/Android embedding | Closed [#23](https://github.com/misofm/engine-v2/issues/23). F2 first resolves its closure against the current mission; do not silently reopen either policy. |
| Successful plan exchange plus its rejected-render/teardown qualifications | [#248](https://github.com/misofm/engine-v2/issues/248) N1/N2. F1 adds compatible DSP-state movement and does not absorb those correctness scopes. |
| Signal-path optimization and console shape | [#202](https://github.com/misofm/engine-v2/issues/202), [#210](https://github.com/misofm/engine-v2/issues/210). This audit changes no chain order or console feature scope. |

## Deliberate non-adoptions

1. Do not import Ardour's callback process mutex, OS-semaphore wait, dynamically resized MPMC graph
   queue, shared-pointer graph ownership or fallback callback-thread deletion.
2. Do not replace V2's strict typed DAG/PDC with iterative runtime route latency propagation merely
   to resemble a plugin DAW. Third-party dynamic-latency policy remains future explicit scope.
3. Do not add runtime CPU dispatch on x86. Keep the pinned x86-64-v3 artifact and boot attestation.
4. Do not replace across-track homogeneous AoSoA banking with only per-buffer generic SIMD. The two
   optimization axes can coexist if measurement later justifies native graph workers.
5. Do not introduce unbounded work stealing for render or decode. Any future pools have explicit
   capacity, ownership, priority, admission and forward-progress rules.
6. Do not make the engine own a timeline editor, DAW GUI, delivery codecs, unrestricted plugin host,
   MIDI subsystem or Ardour-compatible session format.
7. Do not claim Ardour is categorically safer/faster because it is mature, or V2 categorically
   better because it is newer. Every adoption above names the narrower contract and gate.

## Recommended issue order

1. F1 DSP-state continuity, coordinated with #248 N1/N2 but kept as a distinct product outcome.
2. F3 counter units and visibility; it turns silent recovery into evidence before more optimization.
3. F2 mission decision, then one native host at a time only if native embedding remains required.
4. Finish #124's frozen qualification; add a bounded source pool only on a failed gate.
5. F5 per-lane bypass foundation/effect slices, then final cohort-key removal.
6. F6 independent graph oracle before the next large lowering/scheduler change.
7. F4 persistence ownership and recovery before schema V2 or a user-facing project writer.
8. Leave #130 dormant until a named native/cloud deadline budget is missed.

## Completion contract for this audit issue

This issue is complete as a research record when:

- both repository commits and the no-cross-project-benchmark limitation remain explicit;
- P1-P5 are recorded as preservation constraints with the stated qualifications;
- F1-F6 each name evidence, a smallest closable slice, objective gates and non-goals;
- existing issue ownership is reconciled so the audit creates no duplicate implementation scope;
- this matching `.github/ISSUE_SPECS/250-...md` record is committed and pushed; and
- no Engine production code, fixture, benchmark, workflow or acceptance pin changes while recording
  the audit.

Implementation PASS is explicitly not claimed. Every successor follows the normal Sol brief -> Terra
attempt -> Sol adversarial review workflow and the three-attempt stop rule.
