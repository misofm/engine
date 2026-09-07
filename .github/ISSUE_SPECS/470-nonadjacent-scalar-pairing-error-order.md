# Complete nonadjacent serialized scalar pairing with preserved execution-error order

# Astra #443 delivered scheduling preflight

**Decision: adjacency does NOT cover the full applicable serialized scalar population. Number a retained successor before assigning #443.** Read-only delivered source `aba905c0a5ae0bc747a65d1052ba76811fcee3c5` through `engine-456-plan`, and the complete queued #443 spec at `1d3f8d16`. Its appended root ruling is binding. No tests, builds, timing or repository/GitHub mutations were performed.

## Concrete population proof

`crates/graph-compiler/src/schedule.rs:9–65` assigns each node one plus its maximum predecessor level, sorts nodes within levels, and returns ascending levels. `crates/graph/src/program.rs:531–550` enforces that the execution schedule is the concatenation of those levels. This is live single-thread scheduling; it is not the removed worker scheduler.

`crates/builtins-compiler/src/lib.rs:1946–1959` takes the real scalar route when `BankWidth::for_backend(dispatch)` is None, or when no planned banks exist. `into_graph_artifact` then calls `strip_bindings` (`1831–1893`), creating separate actual ConsoleFaderProcessor and ConsoleMatrixProcessor boxes and moving their separate queue consumers. The list's per-track insertion order does not override the graph's level-major execution order.

A concrete source-derived counterexample is two independent, same-depth tracks A and B, both lowered with Backend::Scalar, each with its ordinary unobserved sole-reader PostFader→PostMatrix edge, no sends/delays or other structural incompatibility, and BetweenRenderCalls ownership. At those two levels the real schedule is `F_A, F_B, M_A, M_B` in track/node-ID order. Neither matching pair is adjacent. Both have the same valid scalar dataflow and delivery population targeted by #443; no observer or Concurrent exemption explains their exclusion. Extra zero-work stages between earlier strip stages do not alter this final-level argument. This is a constructive source proof, not an executed fixture claim.

Runtime `units_of` and `build_sequential` (`runtime.rs:1782,1828–1861`) preserve plain scalar units; cohort runs merge bank membership, not these scalar owners. `finish_unit` at2791 returns a plain RuntimeUnit::Op for empty membership. Delivered #430's bank pairing therefore does not collapse the scalar counterexample or repair its scheduling. Current scalar lowering still lacks propagation of the immutable delivery declaration into its two concrete owners; #443 retains that approved task.

The existing proposed pre-build_op adjacent insertion remains appropriate for its bounded slice. It cannot be described as complete serialized scalar delivery. `execute_op` still invokes bound processors at their existing positions and propagates their errors. Eagerly moving M_A to F_A crosses F_B: if F_B fails, old M_A has not consumed its queue or changed state, whereas an eager pair has. Conversely delaying F_A until M_A would change already-completed fader state/queue effects before F_B fails. BetweenRenderCalls freezes producer admission; it does not make failures or state effects commute.

## Required numbered successor body

Suggested title: **Complete nonadjacent serialized scalar pairing with preserved execution-error order**.

Parent #443; retained audit #349 RT-4 population. Depends on delivered #443 adjacent scalar ownership/bridge and its actual-base review. Concurrent scalar ownership remains #444; measurement remains #431. No implementation authority until the finite preparation decision below is resolved and Astra approves an amended executable scope.

Retained product obligation: cover otherwise compatible, unobserved, nonadjacent serialized scalar fader/matrix pairs produced by the existing level-major schedule. Do not close this obligation by renaming all scalar pairing as adjacent, treating BetweenRenderCalls as infallibility, or pointing to bank-tail execution. Preserve original schedule, reductions, exact arithmetic, both queue ownership/drain effects, first error and state at every original execution boundary. Observed, delayed, fan-out, noncompatible and Concurrent paths retain original separate execution.

Smallest first closable slice is a bounded scheduling/error-order design decision on the existing two-track scalar fixture, not an unbriefed general optimizer. Inspect the actual owned scalar stages and intervening operation kinds after #443 delivery. Freeze either (a) a concrete preparation-only eligibility proof and execution mechanism that preserves those boundaries and earns a useful nonadjacent population, with every excluded compatible population explicitly retained; or (b) a precise impossibility/architecture decision explaining which original side effect prevents fusion and what owner decision is needed. A design decision alone does not close the retained product obligation or audit row. Do not authorize speculative public fallibility flags, shared mutable consumers, rollback of arbitrary processors, execution reordering or a new scheduler inside this brief. If a new architecture is required, amend/split before code rather than allowing Luna to choose one mid-attempt.

Finite decision evidence: use the existing compiler/graph fixture helpers to spell out the two-track schedule and identities; enumerate the actual fader and matrix queue/setter failure points and intervening bound-processor return. Provide a before/after trace for successful settled execution, an intervening error, fader error and later matrix error, including queued prefix and state effects. Existing APIs and source references suffice for the initial decision; no builds, corpus or benchmark are required merely to number the retained work.

Before eventual product implementation, freeze a small actual nonbanked two-track prepared fixture and old separate reference. It must prove scheduled nonadjacency, actual selected mechanism, exact PCM and per-boundary state/queue/error order, plus observed/nonunity-send and Concurrent declines using the existing compact fixture conventions. Reuse #443's accepted arithmetic, resource and live allocation proof mechanisms rather than repeat its entire corpus. One actual selection-to-separate SAME-assertion control is sufficient for this new dispatch; exact commands/allowed files must be frozen against delivered #443 before assignment. No additional measurement invocation is implied.

## Reciprocal accounting and #443 assignment

Before #443 assignment, create/synchronize this successor and add its number to #443, #349 and the RT-4 retention record. #443 may then deliver its explicitly adjacent serialized product with all original finite correctness, host, resource, allocation and qualification gates, while this successor remains open for nonadjacent serialized coverage and #444 for Concurrent coverage. Neither child nor parent wording may claim all scalar integration is finished. Root should preserve the existing public Any/static decision and approved identity substitution; this finding does not reopen those decisions.

This is the smallest honest split: the adjacent capability is independently useful, while nonadjacent error ordering is a separate unresolved architecture boundary. It should not be hidden inside a half-day implementation attempt or waived as a performance-only detail. #460 remains the only active feature; no #443 implementation is authorized by this report.


## Numbered retained obligation

This is #470. #443 retains adjacent serialized scalar integration; #470 retains otherwise compatible nonadjacent serialized scalar scheduling/error-order completion; #444 retains Concurrent admission and scalar/bank rollout. #431 retains measurement. Numbering supplies no implementation or timing authority. The actual delivered #443 base and explicit design decision remain prerequisites.

## Orthogonal buffer-identity retention

New #476 classifies and retains distinct-output serialized scalar pairing. It does not replace or narrow this issue: #470 remains about nonadjacent scheduling and intervening errors. #443 now requires explicit same-output/in-place eligibility in its adjacent slice. Neither retained issue claims a measured improvement or authorizes speculative architecture.

## Current-main scope and reachable population

#443/PR #482 is delivered on current `main`
`30f658ee1c0c7d86002f5f2fea075a5dfa8a7c2c`; #476 is closed as a proved
distinct-output non-applicability decision. The remaining #470 population is real and same-buffer.
The production level-major scalar schedule for two independent equal-depth live tracks is
`F_A -> F_B -> M_A -> M_B`. Both fader-to-matrix edges may satisfy sole-reader, undelayed,
unobserved, in-place and `BetweenRenderCalls` eligibility, but the adjacent #443 selector cannot
pair either one. The existing builtins-compiler scalar helper rewrites its fixture into track-major
singleton levels, so it cannot stand in for this production trigger.

The original error order is also real. Each owner drains its queue FIFO, applies a valid prefix,
consumes the first invalid record, leaves the tail queued, and returns before arithmetic. An invalid
`F_B` must therefore leave fully completed `F_A` effects while `M_A` remains untouched; an invalid
`M_A` must follow both completed faders. Eagerly running A's existing composite at `F_A` moves
`M_A` before `F_B`, while delaying the whole composite to `M_A` omits `F_A` effects on an
intervening error. `BetweenRenderCalls` excludes producer races but does not remove these failures.

## Root architecture ruling and smallest product slice

Root adopts a narrow split-owner mechanism with deferred settled fader arithmetic. The literal
contents of a private, unobserved post-fader buffer may remain temporarily unmaterialized inside
one render call. Every observable boundary remains unchanged, and the correct settled fader output
must be materialized before any execution or observer error returns. This is the explicit decision
that the earlier brief withheld; it does not authorize a scheduler redesign, rollback, speculative
processor execution, queue peek/staging API or generic fallibility metadata.

At the fader's original op, run its original reduction and queue drain. A ramping fader executes
its complete original arithmetic immediately. A settled fader records a bounded pending state and
defers only its stateless settled arithmetic. At the matrix's original op, retain the existing
self-copy reduction, drain matrix commands, then use the existing settled fused kernel when both
stages are settled; otherwise materialize the fader and run the original matrix arithmetic. On a
matrix drain failure, materialize the pending fader before returning the original error, preserving
the matrix prefix and queue tail. Before propagating any intervening execution or observer error,
materialize every earlier pending fader in original order without draining its matrix. Pending
state must be cleared on every success and failure path and may never cross a render return.

The first product slice selects deterministic nonoverlapping scalar pair intervals and permits at
most one deferred interval at a time. It must earn the ordinary two-track A pair across `F_B`;
overlapping candidates and every unproved interval remain separate. Preparation must prove the
exact concrete live owners and serialized delivery, same-track PostFader/PostMatrix identity, one
undelayed in-place matrix input, no bank/retired/redirect ownership, no direct or aliased observer,
send, sidechain or output crossing, and no intervening read/write/observer of the deferred physical
buffer. #444 retains Concurrent delivery and pairing; #431 retains its one descriptive capture.

Authorize one narrow prepared split-pair owner interface in graph, implemented only by the exact
builtins-compiler owners. The owner exposes original-position fader begin, matrix finish and
infallible pending-fader completion; the preparation-only factory performs safe checked ownership
transfer. Preserve the delivered adjacent factory and behavior. Allocate all owner/index metadata
at preparation, charge the actual retained layouts and largest allocation before admission, and
release the original owners once off render. Render adds no PCM scratch, allocation/free, lock,
syscall, queue refill or general processor call during failure completion.

## Exact path ownership

Luna attempt 1 may edit only:

- `crates/graph/src/lib.rs`
- `crates/graph/src/runtime.rs`
- `crates/builtins-compiler/src/lib.rs`
- `crates/builtins/src/lib.rs`
- `crates/builtins-compiler/tests/allocation_tracker.rs`
- `crates/graph-compiler/src/lib.rs`
- `crates/graph-compiler/src/compile.rs` solely to fold the derived runtime-metadata reservation
  into the existing pre-cap admission path
- this issue spec and the lane-A tracker/evidence paths

Lane A claims those paths for #470 until a coherent checkpoint is reviewed and delivered. No
scheduler/lowering, SPSC, protocol, host, DSP-kernel, Cargo, workflow or artifact-pin edit is
authorized. Shared paths must remain yielded by lane B. Lane B alone orchestrates any required
AudioWorklet artifact qualification and pin changes after a frozen source checkpoint.

## Objective gates for attempt 1

1. Use an actual production-compiled two-track Scalar graph and assert
   `F_A,F_B,M_A,M_B`, absence of banks, same-buffer identities and actual selection. Compare with
   independent separate owners; do not reuse the track-major #443 helper unchanged.
2. Prove exact PCM, retained state, queue prefix/tail, first error and retry behavior for success
   and invalid `F_A`, `F_B`, `M_A` and `M_B` records.
3. Inject an intervening observer error after completed `F_B`; prove pending `F_A` materializes
   before return and `M_A` does not drain.
4. Prove settled/settled uses the fused arithmetic with a SAME-assertion selection control.
   Ramping fader executes at `F_A`; matrix ramp/retarget falls back at `M_A` with unchanged bits
   and state. A failed render followed by retry detects pending leakage or double processing.
5. Preserve direct/aliased observation data, nonunity send, sidechain, Concurrent, physical-buffer
   conflict and overlapping-interval declines.
6. Reuse the existing allocator/resource machinery to prove zero render allocations and frees on
   success and failure, positive audit liveness, exact retained bytes/largest allocation/cap and
   overflow refusal, and single off-render owner release.
7. Run proportional affected debug/release suites, strict lint/format/realtime policy, supported
   scalar and SIMD target qualification, then freeze the source for Astra LOW adversarial review,
   following the user's 2026-09-07 verification-routing direction. Coordinate any browser artifact
   qualification with lane B. No timing is authorized.

Luna must pause at each coherent compiling/focused-green tranche for root's exact-path status,
commit and upstream audit. This is attempt 1. If the implementation needs a second scheduler,
arbitrary processor rollback, queue API changes or PCM scratch, stop and request a bounded
rebrief rather than widening the slice.

## Luna attempt 1 mechanism checkpoint

Pushed head `73591f76d353d5ceccd4155158f0ad77afb2d3d8` implements the first coherent
split-owner mechanism tranche in the four authorized source files
`graph/src/{lib,runtime}.rs`, `builtins/src/lib.rs` and `builtins-compiler/src/lib.rs`.
Graph now carries a preparation-only split-pair factory/owner, stores the owner outside the two
original runtime ops, invokes fader begin and matrix finish at their original positions, and
materializes pending faders before returning an execution or observer error. The concrete builtin
owner defers only settled fader arithmetic, preserves ramping execution at the fader boundary, and
uses the existing fused kernel only at the matrix boundary. Selection proves same-buffer dataflow,
private physical-buffer interval, serialized exact owners and the existing observation/send/
sidechain declines, then admits at most one deterministic nonadjacent interval.

The tranche includes focused unit coverage for split completion, an explicit two-track level-major
schedule, settled fused selection and matrix-error completion against separate owners. Luna and
root independently ran `cargo test --locked -p graph --lib` (58 passed) and
`cargo test --locked -p builtins-compiler --lib` (40 passed); `cargo fmt --all -- --check` and
`git diff --check` also passed. Root confirmed only the four authorized paths changed and pushed
the exact checkpoint.

This is not source PASS or attempt-1 review. The required actual graph-compiler-produced fixture,
full invalid `F_A/F_B/M_A/M_B` and retry table, intervening observer error, ramp/retarget and
selection mutation proofs, complete decline/conflict/overlap matrix, resource charging,
allocation/free/off-render ownership evidence, release gates and target/artifact qualification
remain. No benchmark or artifact qualification ran, and no performance claim is made.

## Luna attempt 1 boundary and resource checkpoint

Pushed head `ee158ab9` adds a second coherent tranche in three already authorized paths:
`builtins-compiler/src/lib.rs`, `builtins-compiler/tests/allocation_tracker.rs` and
`graph-compiler/src/lib.rs`. The new discriminating reference graphs explicitly disable the split
factory; preparation assertions require one selected split owner for the candidate and zero for the
separate reference. The invalid `F_A`, `F_B`, `M_A` and `M_B` table compares first error, queue
prefix/tail, retained state and retry behavior. An observer on intervening `M_A` proves the pending
`F_B` fader is completed before error return, its later matrix command remains queued, and retry
matches separate-owner PCM/state before settled fusion resumes.

The retained resource estimate now reserves the larger of the adjacent and split outer-owner
layouts for every possible scalar pair. Independent allocation tracking proves a selected split
render allocates and frees nothing, then releases its original owners and outer exactly once off
render. The checked allowance covers two possible split outers while the adjacent accounting
fixture retains its smaller selected outer; the measured layout relationship is asserted without
hard-coding platform byte sizes.

Luna and root reproduced `cargo test --locked -p builtins-compiler --lib` (42 passed),
`cargo test --locked -p builtins-compiler --features test-support --test allocation_tracker`
(8 passed) and `cargo test --locked -p graph-compiler --lib` (65 passed). Graph's existing 58-test
suite, format and diff checks also pass. Root rejected and corrected one self-comparison in the
initial test draft before this checkpoint: the separate references had accidentally enabled the
same split factory. The pushed tests now fail that regression through explicit preparation
witnesses.

This remains attempt 1 without source PASS. The production graph-compiler fixture proves the real
level-major `F_A,F_B,M_A,M_B` ordering under a scalar registry, but that artifact's current buffer
coloring does not expose an in-place fader/matrix output pair and therefore cannot select the split
owner. The existing builtins prepared harness still supplies the live same-buffer selection proof.
Before review, root must resolve this applicability contradiction against the issue's claimed
production same-buffer population rather than relabel the harness as production. Ramping and
retarget boundary assertions, the complete physical-conflict/overlap decline table, failure-path
allocation evidence, release/static/target gates and lane-B artifact qualification also remain.

## Astra MEDIUM attempt 1 review — FAIL

Astra reviewed exact pushed head `a4f9be3f` read-only and rejected source PASS while retaining the
approved split-owner architecture. The graph-compiler fixture was misconfigured rather than proof
of an applicability blocker: `compile_console_model_with_builtins` uses host dispatch and prepares
no live controls, while `scalar_console_registry` changes only effect-bank eligibility. Attempt 2
must use existing graph-compiler APIs with explicit `Backend::Scalar`,
`prepare_session_builtins_between_render_calls` and actual track-control requests, then assert no
builtin banks, the real `F_A,F_B,M_A,M_B` schedule, physical fader/matrix buffer identity, bind-time
split selection and nonzero output. The existing program permits the sole undelayed matrix reader
to consume a nondedicated fader buffer in place; the corrected fixture must turn that source
conclusion into executed evidence without changing coloring.

The new failure tests compare independent owners after root's pre-review correction, but they do
not observe the failed call's private post-fader buffer. Settled completion changes neither retained
state nor queue counters, and retry input overwrites that buffer. Attempt 2 must inspect failed-call
buffer bits through existing narrow graph/runtime test seams against asymmetric nonunity separate
execution, including matrix state, and show that removing completion makes the same assertion
fail.

The larger split outer is charged, but runtime also retains the boxed split-owner table, an
`Option<SplitPairSlot>` in every runtime op and the new runtime owner-table field. The current
accounting test observes an adjacent fixture and cannot prove those layout/allocation deltas fit a
named cap. Attempt 2 must name and bound every new retained allocation/layout through existing
checked-add and allocator machinery on an actually selected split binding. It must also validate
the settled fader envelope after draining and before setting pending state; a direct invalid
envelope must leave the later matrix queue untouched with no pending completion.

Finally, attempt 2 retains split-specific ramp/retarget boundary proof, physical-conflict and
overlap declines, failure-path zero allocation/free, selection mutation and proportional
release/static/supported-target gates. The production minimum is track A across `F_B`; the existing
synthetic B-across-`M_A` harness does not substitute for it. No artifact qualification, benchmark
or performance claim is authorized. This is failed attempt 1 of the three-attempt maximum; preserve
its commits and evidence without weakening any gate.

## Luna attempt 2 production applicability checkpoint

Pushed source `b8775fe5` corrects the attempt-1 fixture entirely within the authorized
`graph-compiler/src/lib.rs` test path. It compiles the actual two-track console session with
explicit `Backend::Scalar`, prepares concrete live controls through
`prepare_session_builtins_between_render_calls`, and asserts zero builtin banks. The semantic
schedule is the required `F_A,F_B,M_A,M_B`; the independently lowered `ExecutionProgram` proves
`F_A` and `M_A` share the same physical `BufferRef`, the matrix reads that buffer and is marked
in-place. Binding the actual production artifact selects exactly one split factory/member, and
twelve rendered quanta contain nonzero output.

The first attempt-2 probe mistakenly inspected semantic `graph.buffer_assignments`, which
intentionally gives the stages distinct assignments and is not passed to the runtime binder. Luna
reverted that probe without checkpointing it. Astra traced the actual binder through independent
`program::lower`, froze the correct executable-program oracle, and rejected any inapplicability or
distinct-output conclusion. Root inspected the final diff and reproduced the exact production
fixture test; format and diff checks pass. The applicability correction is a green checkpoint, not
attempt-2 PASS. Failed-call PCM observation, full runtime metadata charging, direct settled-envelope
validation, ramp/retarget/decline/failure-allocation and release/target gates remain.

## Luna attempt 2 failure-completion checkpoint

Pushed source `78b1ca82` closes the failed-call observability and direct-envelope findings in the
three authorized paths `builtins-compiler/src/lib.rs` and `graph/src/{lib,runtime}.rs`. The graph
test seam records the bind-time selected split fader node and physical buffer, copies that private
buffer into fixed-capacity word storage only after an error, and can disable pending completion for
one mutation control. The concrete fixture selects track `t00` and proves the production-minimum
`F_A,F_B,M_A,M_B` interval, while its separate-owner arm binds no split owner.

With asymmetric nonunity fader and matrix controls, both an intervening `F_B` observer error and an
`M_A` matrix-record error now compare failed-call post-fader words, owner-tagged fader/matrix state,
drained prefixes and untouched tails against separate execution. Disabling completion makes the
same failed-call buffer equality fail. The fixed state trace and PCM capture report overflow rather
than silently truncating. A direct malformed settled fader envelope drains the fader prefix, leaves
the matrix queue untouched, arms no pending work and changes no buffer on a later completion call.

Luna and root reproduced the focused failure test and the complete
`cargo test --locked -p builtins-compiler --features test-support --lib` suite (44 passed); root also
ran `cargo test --locked -p graph --lib` (58 passed), format and diff checks. Compiler
`-Zprint-type-sizes` evidence shows that the test-only owner tag occupies existing padding: the
fader processor remains 232 bytes and the matrix processor 184 bytes both with and without
`test-support`.

This remains a green attempt-2 checkpoint rather than source PASS. Complete runtime metadata
charging on an actually selected binding, split-specific ramp/retarget and decline/overlap gates,
failure-path allocation/free evidence and the proportional release/static/supported-target gates
remain. Artifact qualification and pinning remain lane B's responsibility after source freeze.

## Attempt 2 exact-path amendment for runtime metadata admission

The resource checkpoint found that the existing cap fold is implemented in
`crates/graph-compiler/src/compile.rs`, while the original exact-path list named only that crate's
`lib.rs`. Authorize `compile.rs` solely for the smallest required change: derive the new runtime
metadata reservation, fold it transactionally into the published/capped graph estimate before the
existing graph/plan/largest-allocation checks, and return the existing resource diagnostic and all
ownership on overflow or limit failure. No scheduler, lowering, canonical identity, cap schema or
unrelated compiler change is authorized. Luna reverted its exploratory edit before this amendment;
the preserved uncommitted tranche touched only the previously authorized graph and builtins paths.

## Luna attempt 2 runtime-resource checkpoint

Pushed source `9728ec21` completes the retained-metadata correction in the six authorized graph,
graph-compiler and builtins paths. Every compiled graph now folds a derived runtime reservation
before the existing caps. Old-layout mirrors establish the current deltas without byte literals:
the split slot adds 16 bytes to `RuntimeOp`, 8 bytes to its `RuntimeUnit` container, and the table
field adds 16 bytes to the boxed `GraphExecutor`. The report charges the larger op/unit delta once
per bounded emitted op, uses the full containing allocations for the largest-allocation cap, and
keeps the semantic canonical estimate unchanged. Eligible serialized scalar builtins separately
reserve the one-entry boxed split-owner table; no runtime field or op bytes are double charged.

The graph gates prove derived layout identities, zero/one table estimates, checked multiplication
overflow and transactional estimate rollback. Graph-compiler proves the runtime term is published
once, exact caps admit it, and each one-below graph/plan/largest cap rejects with ownership
returned. The allocation tracker observes the actual selected one-entry table allocation, an
ineligible reference with no table allocation, zero allocations/frees on settled success and an
injected observer failure, a completion-disable mutation, and one off-render release of the
original owners and split outer.

Luna and root reproduced `cargo test --locked -p graph --lib` (60 passed),
`cargo test --locked -p graph-compiler --lib` (65 passed),
`cargo test --locked -p builtins-compiler --features test-support --lib` (44 passed), and
`cargo test --locked -p builtins-compiler --features test-support --test allocation_tracker`
(9 passed). `cargo check` with `test-support`, format and diff checks are warning-free and green.
This remains an attempt-2 checkpoint rather than source PASS. Split-specific ramp/retarget,
physical-conflict/overlap and selection-mutation gates plus proportional release/static/supported-
target qualification remain. Lane B still owns artifact qualification and pinning after source
freeze.

## Luna attempt 2 final behavior checkpoint

Pushed source `d58a59d4` completes the remaining directed behavior proofs in the authorized
`crates/builtins-compiler/src/lib.rs` test module without changing production code. An actual
selected track-A nonadjacent split is compared call by call with independently prepared separate
owners while the fader ramps at `F_A`, the matrix ramps and is retargeted at `M_A`, and a valid
matrix-ramp prefix followed by an invalid record fails then retries. Exact final PCM, captured
post-matrix words, fader/matrix state, queue-drain counts and fused/fallback dispatch match. The
failure capture proves the pending fader is completed before return and the retry consumes the
retained matrix tail exactly once.

Two directed selection cases complete the bounded eligibility evidence. A two-candidate production
fixture selects the deterministic first interval, leaves the later candidate on its original
separate owners and matches both tracks' captured PCM/state to a fully separate reference. A
session-output physical-buffer conflict declines before owner transfer while an otherwise identical
non-output control selects; the declined graph matches separate-owner output and state. These cases
join the already green direct/aliased observer, nonunity-send, connected-sidechain and Concurrent
declines rather than duplicate them.

Luna ran the complete test-support builtins-compiler library suite (47 passed), the allocation
tracker (9 passed), a warning-free test-support check, formatting and diff check. Root independently
audited the one-file exact-path diff and reproduced the complete 47-test library suite. This is a
green source checkpoint, not attempt-2 PASS. Root must integrate current delivered main, run the
proportional debug/release/static/supported-target gates, coordinate artifact qualification with
lane B after source acceptance, and obtain the required Astra LOW exact-head adversarial verdict.

## Attempt 2 integrated source qualification

Root merged delivered main `b95c9b7b` without conflicts at pushed checkpoint `b441d820`; no
intervening main commit touched a #470 graph or builtins source path. On that combined source the
complete affected debug suites passed: graph 60, graph-compiler 65, builtins 9,
builtins-compiler 47 and allocation tracker 9. The same five suites passed in release.

The first strict affected Clippy run then rejected three finite source-shape details introduced by
the mechanism: the fixed preparation constructor's eighth ownership argument, test-only capture
field assignment after `Default`, and an index-only range loop in the selector. No semantic test
failed. Luna made only those three corrections in `crates/graph/src/runtime.rs`; root audited,
tested and pushed them as `9d2e443c`. The final exact-head graph 60, graph-compiler 65,
builtins-compiler 47 and allocation-tracker 9 release suites pass.

Strict affected all-target Clippy with test support, formatting, diff check, realtime policy
(42 regions in 12 files), builtins policy, lane policy, unfused seal (eight registered calls), and
workspace policy all pass. The existing supported cross-target matrix passes for native
x86-64-v3 plus Wasm scalar and simd128; native AArch64 remains unsupported under #378. No timing
or performance claim is made. Source and proportional local qualification are frozen at
`9d2e443c` for Astra LOW adversarial review. Lane B alone retains normal AudioWorklet artifact
qualification and any necessary pin work after source acceptance.

## Astra LOW attempt 2 source verdict — PASS

Astra LOW adversarially reviewed exact clean pushed head
`01a26d4afb6d751693fc3702f2a71522f76b140e` against integrated main `b95c9b7b`, the complete
amended scope and the preserved attempt-1 failure. The reviewer found no blocking correctness,
ownership, realtime, indexing or resource-accounting defect. It independently reproduced six
nonadjacent behavior tests, all nine allocation/resource tests, the production compiler selection
fixture and the runtime metadata overflow/transactionality test. The conservative mixed-runtime
resource reservation is accepted as a bound rather than represented as an exact population count.

Source PASS authorizes delivery qualification only. Lane B must qualify the frozen AudioWorklet
candidate and owns any necessary pin/current-consumer changes. Actual PR-head review, required CI,
merge, GitHub synchronization and clean delivered-worktree removal remain. #444 retains Concurrent
RT4, and neither this source verdict nor the earlier descriptive capture establishes a measured
speedup or closes broad RT4.

## Post-source-PASS delivery qualification finding

The CI-shaped workspace debug command passed every preceding package and #470's 47 library plus
nine allocation tests, then failed one CAPI resource-lifecycle equality. The actual report carries
`graph_session_plus_plan_bytes = graph_incremental_plan_bytes = 228_476` and
`graph_metadata_bytes = 52_575`; the frozen single-plan fixture still expects 227_148 and 51_247.
The exact +1_328 delta is the 16-byte runtime table field plus 82 emitted nodes times the accepted
conservative 16-byte per-op/unit reservation. No PCM, ABI, cap, ownership or render assertion
failed.

This is a stale delivery-side numeric consumer of the accepted resource model, not a source-PASS
reversal or authority for lane A to edit `crates/capi/tests/resource_lifecycle.rs`. Preserve the
failed command and actual/expected values. Lane B owns qualification/pinning and must reconcile
the CAPI frozen report together with the ordinary AudioWorklet resource/current-consumer check,
with an independently justified exact value and one-below behavior before #470 opens a PR. No
blanket repin, timing run or unrelated fixture change is authorized.

Astra LOW independently reproduced the CAPI failure and traced the complete consumer correction.
The single-plan frozen report must add 1,328 to each of its three graph totals. The independent
primitive replacement oracle must add the runtime field and conservative 82-entry allowance, so
its double-live graph peak rises by 2,656 from 504,132 to 506,788; the old comment that uniform plan
shifts cancel is false for a peak that sums both plans. Largest-allocation values do not move
because the 82 × 256-byte containing-op allocation is below the existing maxima. The current
browser resource pin is `hosts/host-web/tests/browser-v1/expected.json`; its Wasm fixture must be
derived and qualified independently rather than copying the native delta. Debug/release focused
CAPI and full resource-lifecycle gates plus ordinary browser expected-resource checks remain lane
B work.

## Lane B native resource-consumer checkpoint

Root integrated delivered main `3ac24f7f` without conflict and pushed merge checkpoint `ab31615e`.
Luna HIGH then changed only `crates/capi/tests/resource_lifecycle.rs`: the three single-plan graph
totals add the derived 1,328 bytes, while two explicit primitive rows charge the 16-byte runtime
table field and 82 × 16-byte op/unit reservation per live plan. The double-live graph peak is
therefore 506,788; the largest-allocation values remain unchanged and the external exact/one-below
cap checks continue to use the independently summed primitive oracle.

Both debug and release `capi --test resource_lifecycle` suites passed four tests. The ordinary CAPI
ABI linkage check and its mutation self-test passed, as did formatting and diff checks. No graph,
builtins, browser, artifact, SDK, ABI declaration, Cargo or workflow path changed. Root must
checkpoint this coherent tranche before Luna begins the independently derived Wasm/browser
resource and artifact qualification. This is delivery qualification under accepted attempt 2, not
a new product attempt or a performance claim.

## Exact-source pre-pin artifact probe

On clean pushed checkpoint `6d3ef49c713f059e5c60b4000889eecaa9cff3db`, Luna HIGH ran the
repin-probe form of `scripts/build-web-audioworklet.sh` exactly once. It returned zero and printed
the lowercase Wasm digest `63dd5f8b0febf193847b697fa8e4d92e791b7e4775f3b4b6b61252783f153e9f`;
the required output directory remained empty and the repository remained clean. The current
delivered pin is still `6452f0db237da1d57b3594e7d95dd53a089a604d5b0791ea8b3533c5930c5a1c`.
Raw stdout, stderr, status, command and source/pin identities are retained in
`/tmp/issue470-prepin-probe` pending durable evidence assembly.

This observed digest is a candidate, not authority to repin. Root created a detached exact-source
scratch worktree and authorized only a provisional scratch pin overlay so the ordinary builder can
retain the six-file candidate. Luna must derive the Wasm resource report from that actual artifact,
classify every difference from the committed browser expectation, and complete the finite native,
artifact, PCM, SDK and browser gates before Astra LOW pre-pin review. Repository pin, browser
expectation, checked result and deployment matrix remain unchanged until the corresponding evidence
is reviewed.

## Wasm resource-consumer checkpoint

From the detached exact-source scratch worktree, the ordinary no-bypass builder ran once and
retained the exact six-file candidate in `/tmp/issue470-qualified-artifact`. Its Wasm is 2,747,774
bytes and reproduces candidate digest `63dd5f8b0febf193847b697fa8e4d92e791b7e4775f3b4b6b61252783f153e9f`.
The shipped artifact static/object/ABI checks passed. Independent direct-Wasm and native fixture
oracles returned zero. Identity, command-timeline and observation-timeline PCM digests remain
bit-identical to their committed Wasm and native expectations.

The stale browser-resource gate reported exactly three Wasm target-sensitive changes and passed
all 26 red self-test mutations: `graphSessionPlusPlanBytes` and `graphIncrementalPlanBytes` each
move from 29,498 to 29,578, and `graphMetadataBytes` moves from 3,659 to 3,739. The independent
native witness reported 45,050, 45,050 and 6,159 for those rows; these values were evidence for the
target-sensitive classification and were not copied into the Wasm fixture.

Luna HIGH changed only those three values in
`hosts/host-web/tests/browser-v1/expected.json`. The artifact-backed expected-resource gate, its 26
mutations, focused host-web resource/layout checks, all three native parity digest tests, formatting,
diff and exact semantic-scope checks pass. Repository artifact pin, checked browser results and
deployment matrix remain unchanged. Root must checkpoint this tranche before the remaining SDK and
three-browser pre-pin qualification; candidate artifact and raw evidence remain under `/tmp` until
durable evidence assembly.
