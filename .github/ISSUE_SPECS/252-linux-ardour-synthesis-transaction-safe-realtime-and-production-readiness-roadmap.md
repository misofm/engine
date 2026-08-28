# 252 Linux + Ardour synthesis: transaction-safe realtime and production-readiness roadmap

One-line summary: Preserve Engine V2's prepared-plan, deterministic graph, AoSoA, bounded-source,
and canonical-session foundations while turning the accepted Linux and Ardour audit findings into
a dependency-ordered program of separately closable correctness and production-readiness issues.

**This is a tracking and decision issue, not implementation authority.** It authorizes no Rust,
C header, test, workflow, ABI, DSP, graph, host, or persistence change. Every implementation row
below must receive its own numbered stateless issue spec, matching open GitHub issue, smallest
closable slice, objective gates, and normal Sol brief -> Terra attempt -> Sol adversarial review.
No cross-cutting "apply the audits" branch is permitted.

## Authority and reproduced baseline

- Linux synthesis authority: #249, with detailed evidence in closed research record #248.
- Ardour synthesis authority: closed research record #250.
- Current Engine authority: synchronized `origin/main`
  `66ede390036448e690d8992569d3e8abcb30b153`.
- The V2 counterexamples cited below were rechecked read-only against that commit. No legacy/V1
  source or archived legacy research was inspected or used as design authority.
- Per the owner instruction that created this synthesis, #249 is administratively superseded by
  this tracker and may close after this record is upstream. Its four findings remain mandatory
  separate successor issues before implementation. #250 was already closed as research; linking it
  here does not claim implementation PASS.

## Synthesis verdict

Do not make V2 resemble either Linux or Ardour wholesale. Preserve the parts already better suited
to a headless, agent-first, cross-target engine:

1. exclusive ownership of one structurally immutable `PreparedRenderPlan`, with preparation and
   structural mutation off render;
2. deterministic typed DAG compilation, stable reductions, exact integer-sample PDC, and fixed
   prepared effect latency;
3. target-pinned scalar/W4/W8 AoSoA execution with no x86 runtime dispatch;
4. bounded generation-tagged source rings, explicit zero-on-underrun, and memory independent of
   stem duration;
5. strict canonical TOML plus the revision-aware binary semantic protocol;
6. the `miso-engine-host-core` preparation seam, conservative BTLV rules, capability discovery,
   native-effect registry boundary, and existing typed observation architecture.

Adopt the narrower outcomes the audits proved are missing:

- a rejected render must not commit a plan boundary;
- teardown authority must be encoded, including exceptional/early-return destruction;
- render work must be bounded over a frozen entry set, not merely backed by bounded storage;
- multiword telemetry must represent one complete generation;
- compatible native-effect DSP state must survive unrelated structural edits;
- effect recovery must have correct units and observable per-instance counters;
- native/device integration and durable session storage need real end-to-end contracts;
- graph lowering needs an independent semantic oracle; and
- per-instance bypass must stop fragmenting homogeneous cohorts without changing latency or bits.

## Decisions frozen by this synthesis

1. **A rejected call is not a block boundary.** Time, shape, overflow, and executor refusal leave the
   active epoch, public projection, retirement state, and candidate eligible for one corrected retry.
2. **Fallible work precedes mutation.** Production render execution after boundary commit must be
   infallible by construction; recoverable DSP faults produce deterministic recovery plus counters.
   Do not attempt to "roll back" already-mutated arbitrary DSP or add a whole-block transactional
   audio copy.
3. **Teardown needs a reserved path.** `!Send`/`!Sync` and documentation do not stop Rust `Drop` from
   running on a callback. Arming a render owner must reserve its off-render escape capacity up front.
4. **Effect-control staging freezes the entry producer cursor.** An arbitrary quota can defer records
   already eligible at the boundary and can consume later arrivals. The consumer instead processes
   exactly the entry snapshot, subject only to a caller-configured prepared capacity.
5. **Telemetry uses a scalar odd/even generation protocol.** This is for non-pointer data only; it
   does not import a Linux seqcount around plan ownership.
6. **Native and mobile embedding remain required.** The current mission explicitly names native,
   iOS, and Android. Closed #23's browser-only closure is stale policy and must be superseded by new
   platform briefs rather than silently treated as authority.
7. **Durable files belong above the realtime/runtime ABI.** Canonical bytes remain owned by the
   session/control model; filesystem policy belongs in a sidecar/SDK/tool or host layer.
8. **Worker pools remain evidence-triggered.** Ardour proves mature alternatives exist, not that V2
   should add workers before a named V2 workload fails a budget.

## Ordered implementation program

The order below is a dependency order. Keep at most one launch-critical implementation issue active.
Qualification/research may proceed independently only when it edits disjoint files and cannot make
the active issue's gates ambiguous.

### 0. Governance and briefing only

Record in #144 that copied/legacy-derived material is quarantined from current architectural
authority under the current mission; any retained outcome needs independent V2 evidence. Then create
the separate successor specs described below. Closing #249 under the owner's superseding direction
does not waive this requirement and does not authorize implementation from this tracker.

### 1. Transactional rejected-render boundary [P0, S]

**Ownership:** `miso-engine-core` plan/render exchange and the `miso-engine-capi` epoch projection.

Add a pre-mutation validation/refusal phase for time continuity, I/O shape, clock overflow, and every
production executor condition that can still return `RenderError`. Once activation starts, the
production executor must either render successfully or use its declared deterministic recovery path;
it must not surface a late error after arbitrary state mutation. Commit candidate ownership,
handover, retirement, and the externally queryable epoch through one documented linearization
protocol. A short in-progress scalar generation may make concurrent queries retry/return none; they
must never return a provider or resource report for a different committed epoch.

Do not merely move `enter_block` after render, which would render the old plan, and do not "fix" the
projection by publishing a failed epoch.

**Exit gates:** Cross a pending replacement with time discontinuity, input/output refusal, overflow,
and an injected pre-mutation executor refusal. Each rejection leaves ownership, epoch projection,
retirement, handover, clock, and pending candidate unchanged; a corrected retry applies exactly once.
Deterministic interleavings prove resource queries return one matching epoch or a documented bounded
retry/none result. Successful publication retains Release/Acquire ordering and the callback remains
allocation-, deallocation-, drop-, lock-, syscall-, and wait-free.

### 2. Render-owner teardown receipt and exceptional handoff [P0, S/M]

**Ownership:** public owners in `miso-engine-core` and `miso-engine-host-core`, source-worker ownership,
and the C API destruction contract/smokes.

Replace ordinary callback-context destruction with an armed typestate that owns its payload through
a no-drop representation. `stop`/`retire` returns a complete `Send` control-side receipt after
quiescence. Early return, unwind, or accidental destruction of an armed owner moves the entire active,
pending, and retired bundle into a pre-reserved bounded handoff; arming fails off render if that
capacity cannot be reserved. The callback must never free a box, destroy an Arc's last owner, stop or
join a source worker, or depend on a fallible full-queue fallback.

**Exit gates:** Exercise normal stop and every public early-drop path under the render audit. Observe
zero alloc/free/drop/lock/syscall/wait/stop/join on render, then reclaim and join every object exactly
once on a named control thread. Preserve PCM, swaps, resource reports, and exclusivity compile-fails.
The public Rust docs, C header, and C smoke fixture must state the same quiescence and execution-domain
contract. No RCU, custom refcount, worker-pool redesign, graph change, or general ABI redesign.

### 3. Entry-snapshot effect-control staging [P0 realtime hardening, S]

**Ownership:** the bounded SPSC snapshot primitive in `miso-engine-core`, scalar staging in
`miso-engine-effect-contract`, and bank staging in `miso-engine-rack`/`miso-engine-graph`.

Capture the producer cursor once at stage entry and consume until the consumer reaches that frozen
cursor. Do not reload the live producer cursor inside the invocation. Preserve FIFO and canonical
last-wins insertion for every entry-eligible record; records published later remain for the next
boundary. The bound is the caller-configured queue capacity, never a compiled `MAX_TRACKS`.

**Exit gates:** Adversarial refill at every pop proves successful pops and insertion comparisons do
not exceed the frozen invocation bound. Empty, partially full, full, wraparound, and producer-wrap
cases retain exact FIFO/last-wins behavior with no loss. Scalar and W4/W8 bank paths share the rule.
A mutation restoring drain-to-live-empty fails. Realtime audit stays clean.

A separate P2 qualification may later sum all prepared per-lane entry bounds and admit them against a
host-owned next-boundary work budget. Storage capacity and service budget remain distinct.

### 4. Coherent CAPI render-telemetry cell [P1, S]

**Ownership:** `miso-engine-capi` only, reusing the proven non-pointer pattern in
`miso_engine_core::realtime::observe` rather than creating another observation architecture.

Publish sample, peak, and sequence between odd/even generation stores. The reader accepts data only
between two equal even reads, acknowledges only that stable generation, and retries a fixed number of
times or returns none. Count/document skipped complete generations; loss is allowed, torn tuples are
not.

**Exit gates:** Deterministically interleave every writer store and reader load and accept only one
complete block. Removing either generation check is a red mutation. The writer remains wait-, lock-,
allocation-, deallocation-, and syscall-free. Do not change meter DSP, reliability semantics, or plan
generation.

### 5. Compatible native-effect state continuity [P0 product continuity, split M slices]

**Dependencies:** items 1 and 2 must be closed first.

Compile an off-render transfer manifest keyed by stable `EffectNodeId` plus exact compatibility facts:
native effect identity/build contract, contract major, state-layout version, sample rate, quantum,
quality, ports, link mode, latency/tail/state shape, and any algorithm-specific storage shape. Admit
the number of boundary moves against an explicit caller-owned swap-work budget. At the boundary move
state-owning handles exactly once; never serialize/copy an unbounded delay or lookahead ring. A
cohort/lane reordering must move the matching per-instance state, not the old lane number.

Also freeze what crosses for an unchanged instance: smoothed parameters, envelopes/filter histories,
latency/tail rings, live bypass/symmetry state, acknowledged control records, recovery counters, and
observation subscription/window state each need an explicit preserve/reset rule. No acknowledged
record may disappear between provider epochs. Source generation/ring state keeps the currently
explicit reset policy and is outside this slice.

Split delivery into independently useful vertical slices rather than one all-effect rewrite:

1. generic manifest/handover plus one end-to-end parametric-EQ scalar and regrouped-bank continuity
   fixture;
2. compressor and true-peak-limiter envelope/lookahead continuity; and
3. dual delay's large-ring/tail continuity, followed by bounded family issues for remaining launch
   effects where their state shape requires distinct work.

**Exit gates per slice:** During sustained processing, an unrelated route/track edit matches a
no-edit reference from the replacement sample within the effect's frozen tolerance; fixed-latency
impulses do not move or click. Every incompatible key field resets only that instance. Pre-publication
failure keeps the old owner intact; success moves once and off-render retirement drops once. Scalar,
W4, W8, and Wasm-supported paths retain their existing tolerances and realtime trace gates.

### 6. Correct and expose effect-recovery counters [P1, two S slices]

First correct the unit bug across the launch roster: only `nonfinite_left_blocks` and
`nonfinite_right_blocks` count rejected channel/lane process calls and therefore increment by one,
not by frames. `invalid_spans` retains span semantics, and the explicitly named conformance-only
sanitized-sample fields retain sample semantics; do not mechanically convert every field to blocks.

Then saturating-accumulate every scalar and bank report into preallocated counters keyed by stable
effect instance and channel/lane, and expose typed read/reset semantics either after render disarm or
through the existing bounded counter transport. Do not log on render, reuse sample-observation lanes,
or fold this into #203's meter-cost work.

**Exit gates:** Injected scalar/W4/W8 failures give equal per-instance block counts; partitioning
counts calls rather than frames; unaffected lanes remain zero; overflow saturates; production no
longer discards reports. Enabling/disabling external delivery changes neither topology nor PCM, and
the callback remains clean.

### 7. Real host/device boundary, one platform at a time [P0 product, separate L slices]

First synchronize policy: the current mission supersedes #23's browser-only closure. Do not reopen
its combined iOS+Android implementation shape. Brief a working `host-native` platform adapter first,
then iOS and Android as separate product issues; select and cite each platform API in that brief. A
shared bridge abstraction may be extracted with the first working host, not built speculatively.

Each platform prepares from the **actual** device rate and maximum callback size, rejects unsupported
rates outside 44.1/48/88.2/96 kHz or explicitly reprepares, and uses a preallocated bridge between
variable host blocks and the fixed engine quantum. A callback above the admitted maximum fails to a
bounded zero/xrun result rather than resizing. Reconfiguration quiesces and prepares off render.
Report device I/O latency, bridge fill latency, and engine latency as distinct terms; external device
latency is not silently folded into internal graph PDC. Publish bounded coherent xrun/deadline
telemetry and apply platform priority/workgroup guidance.

**Exit gates per platform:** Actual-rate/block negotiation, variable-block partition invariance,
maximum-block refusal, route/device change, latency impulse, deadline/xrun injection, callback audit,
and a real simulator/device or backend run. No implicit SRC, callback allocation/locks/syscalls, stale
plan render, or one platform holding another's usable adapter open.

### 8. Independent scalar semantic graph oracle [P1 verification, M]

Build a test-only uncolored scalar interpreter from normalized graph semantics. Give semantic
nodes/edges dedicated planar storage, simple reference delays, stable edge-ID reductions, and
discriminating deterministic mock processors for effects/sidechains. It must not call production
coloring, aliasing, cohort construction/run, gather/scatter, route-fold, PDC-buffer, or scheduler
helpers. Analytic tiny graphs first validate the oracle itself; then compare it with production over
the frozen fifty-seed corpus.

**Exit gates:** Independent mutations to schedule order, buffer reuse, alias lifetime, bank
membership, gather/scatter target, route coefficient fold, per-edge PDC, and reduction order all fail.
The oracle and corpus never enter release artifacts or benchmark authority. Close this before the
final cohort-key change below or any renewed #130 scheduler implementation.

### 9. Remove prepare-time bypass from cohort identity [P1 efficiency/correctness, family slices]

Generalize the existing latency-matched `BypassShunt`; do not create a competing dry-path mechanism.
For each bank-eligible effect family, keep the wet state advancing, feed the exact latency-matched dry
line when required, and restore selected lanes with bitwise/copy selection that preserves signed zero.
Charge every dry buffer and latency line to preparation resources. Freeze whether a "toggle" means a
live control record or a structural session mutation before claiming it needs no recompile.

Only after every bank-eligible launch effect advertises and proves the shared per-lane contract may a
final issue remove `bypass` from `EffectProgramKey`. Until then the key remains the correctness guard.

**Exit gates per family:** Mixed initial/live bypass states share one cohort; wet state stays warm;
re-enable is continuous; delayed dry PCM is bit-exact including signed zero; latency/PDC/state payloads
do not change; scalar/W4/W8/Wasm gates remain frozen; sessions with no eligible bypass/control retain
their documented zero-storage/trace path. The final key-removal mutation must fail if any registered
bank effect lacks the capability.

### 10. Durable canonical-session persistence [P1 release gate, split decision/implementation]

Place filesystem policy in a native sidecar/SDK/tool or host crate, never in the render ABI. The
session crate continues to validate and emit canonical bytes and may expose pure migration functions.
Implement same-directory temporary writes, file flush/error handling, prior-good backup, atomic
replacement where the OS guarantees it, stale-temp/pending recovery, and explicit per-platform
durability claims (including directory durability where relevant). Browser/mobile storage receives a
separate host policy rather than pretending POSIX rename semantics are universal.

Do not invent a V1 -> V2 transformer before schema V2 exists. Make persistence/recovery ready before
the first user-facing writer; add the offline migration command in the same issue that defines the
actual incompatible schema, preserving the original and producing canonical idempotent output.

**Exit gates:** Fault injection at every open/write/flush/backup/replace boundary leaves either the
old complete canonical snapshot or the new one, never a torn accepted file. Recovery choice is
deterministic, unsupported future schema is refused, overwrite/symlink/race ownership follows a
documented platform policy, and the eventual migration preserves its source.

## Existing issue ownership and conditional work

- Amend #124 with the slow/heterogeneous-reader, many-source deadline/underrun/seek-storm/idle-CPU/
  memory workload if it is still absent. Add a prestarted bounded decode pool only when that frozen
  workload fails, with exactly one producer owner per source.
- Keep #130 dormant until a named native/cloud deadline budget fails. If triggered, preserve
  preallocation, disjoint outputs, stable node-ID reduction, set-form determinism, and the
  single-thread/browser fallback. Never port Ardour's semaphore/shared-pointer scheduler.
- #203 alone owns meter/observation cost. Item 6 is typed effect-fault accounting, not meter DSP.
- Future C-ABI extension policy must choose and test exact-size versus prefix-growth behavior with
  real old-header/new-library pairs; neither audit mandates a Linux-shaped ABI.
- Capability discovery, `protocol/schema.rs`, stable observation sites, preparation accounting,
  and the source-worker baseline are preserved, not reopened by this tracker.

## Deliberate non-adoptions

1. No callback process mutex, OS-semaphore wait, work stealing, dynamically resized render queue,
   shared-pointer graph ownership, or callback-thread deletion fallback.
2. No seqcount around pointer ownership, global lifecycle/failpoint enum, generic observation rewrite,
   protocol schema rewrite, or graph-wide typestate refactor without a new counterexample.
3. No iterative runtime latency propagation, implicit SRC, runtime x86 CPU dispatch, replacement of
   AoSoA with only within-buffer SIMD, or weakening of fixed prepared latency/PDC.
4. No timeline editor, DAW GUI, unrestricted plugin host, delivery codec, Ardour session format, or
   cross-project performance claim.

## Adversarial verification of this plan

The first draft did **not** pass. Review against the current code and repository rules found and
corrected these defects:

1. Moving/rolling back a plan after a late executor error cannot restore arbitrary mutated DSP. The
   plan now requires pre-mutation refusal and infallible post-commit production execution.
2. A mandatory `stop(self)` still permits callback-context `Drop`. The plan now requires a
   pre-reserved exceptional handoff and exact-once control-side receipt.
3. A fixed quota alone can consume post-entry records or defer entry-eligible records. The plan now
   freezes the producer cursor at invocation entry.
4. Implementing DSP continuity directly on the current exchange would compound the rejected-render
   and teardown defects. Items 1 and 2 are hard prerequisites, and transfer work has explicit
   caller-configured admission rather than a compiled track cap.
5. Moving only DSP bytes can lose acknowledged control records, live bypass state, counters, or
   observation windows. The continuity brief must state a preserve/reset rule for each class and
   must move handles rather than copy unbounded rings.
6. "Every recovery counter counts blocks" overstates the actual structure. The plan narrows the fix
   to the `nonfinite_*_blocks` fields and requires explicit units for every exposed field.
7. The host decision was left ambiguous even though current `AGENTS.md` names native/iOS/Android.
   The corrected plan treats #23 as stale and splits platforms.
8. The bypass draft duplicated machinery already present in `BypassShunt` and removed the cohort key
   too early. The corrected order generalizes the existing shunt, migrates families, then removes the
   key under a complete registry gate.
9. An immediate V1 -> V2 migration command is impossible before V2 exists, and rename alone is not a
   cross-platform durability proof. The corrected plan separates persistence foundation from the
   eventual concrete migration and states durability by platform.
10. One combined implementation issue would violate the smallest-slice and WIP rules. This record is
    a tracker only; every row and named sub-slice remains separate implementation authority.

**Verdict: PASS as a dependency-ordered program after those corrections.** This verdict means the
plan is coherent and falsifiable. It is not implementation PASS for any row.

## Tracker close gates

Close this tracker only when:

1. items 1-10 and every named vertical sub-slice that proceeds have matching local/GitHub issues with
   scope, owners, dependencies, objective gates, and non-goals;
2. #144 records the quarantine/independent-rederivation governance ruling;
3. #23's stale closure is explicitly superseded without reviving its combined two-platform scope;
4. #124/#130/#203 ownership remains non-duplicated and conditional gates are recorded;
5. each implemented capability has a pushed evidence commit, Sol PASS, synchronized GitHub evidence,
   and closed remote issue; and
6. all preserved decisions and deliberate non-adoptions above remain green.
