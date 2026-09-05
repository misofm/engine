# Integrate settled fader/matrix fusion into serialized live bank paths

Scope refresh on delivered main `4b352b36ba33334ea2e0c6847c0e3ecf6e8ab33a`. #429 is delivered. This numbered amendment remains planning-only until Astra approves the concrete scope and #442 closes.

## Retained full outcome

#442 owns immutable delivery-policy plumbing without arithmetic changes. #430 owns actual serialized live W4/W8 bank pairing and console-free eligible banks. #443 retains live scalar pairing. #444 retains explicit concurrent native admission and safe pairing; existing public raw concurrent producer plans remain separate until that contract is earned. #431 owns fresh measurement. These reciprocal issues retain the complete original live-integration goal; no individual child closes #349 RT-4.

# #430 current-source readiness and bounded refresh

**Readiness: source premise confirmed; not implementation authorization.** Inspected #430, #429's frozen scope and current implementation, and the live builtin/compiler/graph/rack ownership paths at #429 worktree head `d4cfde6b0b8a9eac06291b60573bdc50b1a05fa5`. #429 remains the sole feature until PR #441 completes exact-head review, required CI and delivery. Root must freeze the actual merged base before numbering/synchronizing the split and approving the final ownership seam. No tests, builds, benchmark, legacy inspection or repository/Git/GitHub mutation performed.

## What the live source actually does

- `BuiltinChain`'s #429 improvement is public prepared-scalar processing. Its static FaderStage is not the live FaderRampStage and cannot carry console state for this issue.
- `builtins-compiler::strip_bindings` builds separate FaderProcessor/MatrixProcessor or ConsoleFaderProcessor/ConsoleMatrixProcessor owners. The console fader constructs FaderMuteRampBuiltins; matrix owns MatrixBuiltins. Each owns and drains its own consumer before its process call.
- `planned_strip_banks` plans input/fader/matrix over the same track list, but explicitly does not treat common planning as proof of compatible runtime adjacency. `into_graph_artifact_with_banks` moves each console consumer exactly once using take and emits separate FaderBankProcessor and MatrixBankProcessor boxes. Each has its own live bank state, counters and consumer array. Its input/fader/matrix claim sets are asserted equal.
- `graph::runtime::chains_into` proves the actual lowered lane-wise relationship: equal nonempty membership; exactly one undelayed, non-sidechain input; predecessor equality; sole readership; producer not graph output; no direct or aliased-tap observer. That is the relevant existing boundary proof. A common cohort signature alone is insufficient.
- `RuntimeParts::chain_for` currently takes each processor into a separate opaque BuiltinStage and shares one resident AoSoA scratch. Sharing a chain removes transposes, not the three fader/matrix arithmetic traversals. `GraphPreparedBuiltinBankProcessor` and rack BankStage expose ordinary process/begin_block, not a typed fader/matrix ownership-transfer interface.
- Current fader and matrix bank drains are inside their process methods. The input/effect begin_block drain and mono-collapse witness ordering are a separate earlier step. Moving all drains wholesale into begin_block would be another behavior change and is not justified by #429.
- FaderRampStage has authoritative per-channel/per-lane u32 countdowns and current/target/mute words. Its settled branch uses the current gain and mask; an active branch updates/snap-clears ramp state. Matrix has its own independent countdown/state. Either active stage requires whole-call separate fallback for this slice.

## Split recommendation before coding

The retained #430 scope currently combines two independently useful ownership integrations. Narrow #430 to **live W4/W8 bank fader/matrix pairs in already-admitted adjacent cohort chains**. Create a named, reciprocal successor for **live scalar fader/matrix pairing with preserved post-fader observations** before assigning #430. Scalar bindings are independent per-node boxed owners and do not pass through bank chain_for; adding them is not a tail fix to the bank work. Keep scalar execution as the existing separate fallback in #430, including when backend selection produces no bank.

Do not close broad RT-4/#349 when #430's bank slice ships. Retain scalar integration in the named successor, #431's separately scoped measurement and all other #349 obligations. No host-observation feature is needed for the bank slice: preserve existing send/meter semantics and decline pairing where observable, rather than removing boundaries or adding a new observation API.

## Smallest closable bank product

Prepare an actual joint owner for one adjacent compatible live fader/matrix bank pair after the existing graph relation proves the pairing safe. At render, that owner drains both existing queues according to the frozen admission/application contract, checks live settlement after those drains, then uses #429's kernel for a fully settled pair or the original fader-then-matrix sequence for the whole call. It must hold real FaderRampStage/MatrixStage state, not reconstructed parameters or a prepared-only fader.

Eligible pairing requires the actual same backend/width, same populated lane count, same track identity and order, matching contiguous active lanes and direct fader-to-matrix dataflow. No holes, reordered members, intervening stage, delayed edge, sidechain, extra reader, graph output, direct observer or aliased observed tap may be crossed. Preserve ordinary trailing identity/unmuted padding. Any missing proof, unrecognized processor shape or unsupported pairing returns the original owners and separate processing. Do not infer eligibility solely from track IDs, expected stage names or all-zero latency.

Keep input processing and its recovery before this pair; preserve the existing mono seam that duplicates before the asymmetric fader/cross-channel matrix. A paired owner remains seam-side and may never consume a stale second plane from one-plane collapse. Preserve the logical fader and matrix accounting/readbacks rather than silently counting two logical stages as one. Quantified resource estimates must cover the actual replacement owner and preparation peak; use existing resource/lifecycle assertions, not a new accounting framework.

## Ownership seam that must be frozen before assignment

The current erased processor/slot API cannot retrieve both concrete owners safely just because two nodes have matching names. The narrow candidate is a **preparation-only compiler-owned pairing factory**, invoked at the graph's proven adjacent bank seam before stage erasure into rack slots. It consumes the two existing owners exactly once and returns either one combined bank processor or the same two unmodified owners. The compiler retains responsibility for concrete fader/matrix type/state ownership; graph retains responsibility for dataflow/observation eligibility. The resulting combined slot can live in the existing BankChain without changing the generic rack render loop.

Before implementation, record the exact Rust type/function ownership route on the merged base, including failure return ownership and unchanged fallback consumers. This is a source-level design checkpoint, not permission to invent a general graph optimizer, reflection registry or new dependency layer. Do not use unsafe downcasts, shared mutable aliases, Arc/Mutex, duplicated Consumer handles or render-time construction. If a bounded safe preparation interface cannot express this with the existing dependency direction, stop and split that ownership prerequisite explicitly; do not widen the feature quietly. Authorized likely source boundary is builtins, builtins-compiler and the graph preparation/bind seam, with existing graph-compiler integration and allocation/resource tests. No rack-wide execution rewrite or host subsystem work is justified.

## Queue, arithmetic and fallback contract

Each fader and matrix queue retains exactly one consumer. Preserve per-queue FIFO, admitted block/application sample, every record's application and existing rejection/backpressure behavior. Fader records still update remembered fader gain, mute endpoint/mask and smoothing; matrix records still retarget actual matrix state. Determine eligibility only after both applicable drains, so an admitted ramp cannot be rendered as settled. Do not add a queue peek followed by later consumption or skip a consumer on an eligible block.

Freeze the meaning of moving the matrix drain before the fader's sample loop: it may affect only that same block's matrix state and may not change which concurrently admitted records qualify for the block. The concrete brief must identify the existing admission/render-boundary mechanism that earns this equivalence. If independent queues can receive new block-eligible records during render, an unrestricted earlier while-pop is not proved equivalent; preserve a bounded admission cutoff or decline that fusion rather than inventing new timestamps or weakening the ack contract. Ask explicitly: can an ack now precede a dropped or delayed record?

Once drained, fusion requires every populated fader L/R and matrix countdown zero at entry to the arithmetic call. Any ramp means separate processing for the whole call, even if it ends inside the call or its present values look settled. Next call may fuse. Preserve retarget, exact endpoint snap, reset, remembered muted gain, current/target readback, reports and counters. No mixed-lane fusion or ramp-tail optimization in this slice.

Use the existing #429 kernel and its exact gain/mute/matrix operand order: load both original planes, gain then bitwise mute, coefficient-before-sample multiplication, existing unfused additions, identity selection of faded words, then stores. No FMA/reassociation, coefficient precombination, gain-one shortcut, alternate mute, sanitization or numerical tolerance. Input recovery stays separate; preserve same-width exceptional-value/FP-environment limits from #429 rather than claiming cross-target NaN payload identity.

## Minimum discriminating proof and qualification

Use existing tests rather than create another large fixture corpus:

1. Compile a small actual live-console graph with multiple W4/W8 members, asymmetric fader gains/mutes and nontrivial matrix crossfeed. Compare fused and deliberately forced-separate prepared paths bit-for-bit after every block, including actual state/readback/report/counter outcomes. The reference must retain the old processors and arithmetic, not call the new combined helper.
2. An actual prepared/render dispatch witness must show eligible pairs run the combined arithmetic, and each ineligible case stays separate. One output-only oracle is insufficient: restoring old dispatch must fail the same mechanism assertion. Exercise real backend selection where supported and explicit W4/W8 arithmetic/ownership fixtures without representing a software width as native machine qualification.
3. Drive both queues with distinct, output-discriminating commands at the same admitted block, multiple same-queue records, zero-duration retarget, nonzero ramp, mid-ramp retarget, ramp ending inside a block, next-call settlement, mute/unmute while gain changes, and reset. Delaying either drain, dropping a record, inspecting settlement before drain, or skipping fallback must fail the same focused assertions. Include nonzero first_sample and admission/application sample checks through the real console path.
4. Preserve the existing `a_meter_leased_at_post_fader_splits_the_chain_and_still_meters` proof: compare nonempty published windows and PCM against separate reference, and prove that the observed cohort declines fusion while another unobserved cohort still fuses. Add/reuse a nonunity post-fader send with a crossfeeding matrix so pre/post substitution changes a bit; post-matrix observation remains eligible. Cover aliased observers as accepted by chains_into, reorder/mismatch/holey rejection at the narrow preparation seam, and legal trailing population.
5. Retain input nonfinite recovery and mono-collapse/seam regression tests. Prove the pair never observes pre-recovery samples or stale/unduplicated right planes. Do not fuse input or change its queue/symmetry policy.
6. Reuse installed allocator/audit infrastructure with positive allocation AND free liveness outside render; repeatedly render eligible and ramp/observation fallback plans with zero audited allocation/frees. Retain realtime/lane/unfused/workspace policy and exact retained-resource/cap admission checks. No new unsafe/locks/I/O/logging or unbounded admission loop may enter render.

After focused debug/release source PASS, root runs proportional full workspace, supported native/browser-Wasm SIMD and relevant shipped-artifact/static/browser qualification on the immutable merged candidate, preserving existing gates. No AArch64 revival, benchmark run or speedup claim is authorized here. #431 must separately identify workloads that actually exercise the newly paired live path; full-chain-only #429 timings cannot measure this product.

## Delivery and current decision

This refresh identifies a concrete bank product, the existing eligibility proof and two still-required ownership/admission design facts. It is **not yet a ready-to-assign implementation brief**: root must finish #429, freeze merged source, explicitly number the scalar successor, and amend/synchronize #430 with the concrete safe factory/consumer-transfer route and the established admission cutoff proof. Astra then gives one numbered source-scope approval. Do not delegate speculative implementation to answer those design questions.

Once approved: Luna attempt 1, root exact-path coherent checkpoint, Astra adversarial verdict, Sol at most attempts 2 and 3 only after FAIL, then hard stop/rescope. Root owns final actual-head PR review, required CI, upstream evidence and issue synchronization. The current feature WIP limit and any measurement quiet window remain binding.

# #430 ownership and admission ruling on the current #441 candidate

**Both source questions are resolved sufficiently to choose scope.** A safe preparation-only pairing route is available without reversing dependencies or changing the rack renderer. A universal queue cutoff is not present: moving the matrix drain is proved equivalent for WebEngine's serialized command/render ownership, but not for the public concurrent raw-producer path. Do not authorize unrestricted native/live pairing on the strength of browser acknowledgements.

Read-only source investigation on the #429/#441 candidate at `d4cfde6b0b8a9eac06291b60573bdc50b1a05fa5`; final merged-base confirmation remains necessary. No source edits, tests/builds, benchmark or Git/GitHub operations performed.

## 1. Concrete safe preparation-only ownership route

Current direction is `builtins-compiler -> graph -> rack`, while builtins-compiler also depends on builtins. Graph does not depend on builtins or builtins-compiler (`crates/graph/Cargo.toml`; `crates/builtins-compiler/Cargo.toml`). A graph-owned concrete FaderBankProcessor enum would reverse that layering. The existing graph owns boxed `GraphPreparedBuiltinBankProcessor`s (`crates/graph/src/lib.rs:519`) and converts them into opaque BuiltinStage slots in `RuntimeParts::stage_for`, called by `chain_for` (`crates/graph/src/runtime.rs:1605`). Pair before that erasure.

Recommended narrow interface, declared in graph and implemented by builtins-compiler:

```rust
// Preparation-only types; names are unversioned.
type BuiltinProcessor = Box<dyn GraphPreparedBuiltinBankProcessor>;
type BuiltinPairFactory = fn(
    BuiltinProcessor,
    BuiltinProcessor,
) -> Result<BuiltinProcessor, (BuiltinProcessor, BuiltinProcessor)>;
```

Add a default-none factory query to the processor trait, and make its type-erased owner safely inspectable as `Any + Send` on the control plane. With the workspace's pinned Rust 1.97.1 (`Cargo.toml:53`), the implementation can use safe trait-object upcasting to `dyn Any`/`Box<dyn Any + Send>` and checked `is`/`downcast`; no unsafe downcast, pointer identity, reflection registry or new dependency is necessary. If explicit forwarding methods are preferred for clarity, they must remain preparation-only and expose the same safe standard-library type test, not render-time reflection.

Only FaderBankProcessor advertises `pair_fader_matrix_banks`; defaults for input, matrix, unknown/test processors and effect slots stay None. The factory lives in builtins-compiler beside the concrete processors (`crates/builtins-compiler/src/lib.rs:538,610`). Before consuming into Any it checks BOTH exact concrete types by shared reference plus compatible bank backend/width/population and delivery eligibility. On every declined check it returns the original two boxes, in the original order, untouched. After those type checks the owned downcasts cannot fail without a programming invariant violation; a control-plane expect is acceptable, with the verified type checks adjacent. The resulting FaderMatrixBankProcessor owns the moved concrete FaderBankProcessor and MatrixBankProcessor fields (including both consumer arrays and counters). No copied consumers, reconstructed state, new shared owner or render-time allocation is introduced.

Graph's `chain_for` should inspect only adjacent Membership::Builtin entries already admitted to the SAME cohort run. Before extracting either owner it confirms actual PostFader/PostMatrix roles, equal ordered track members, width/backend and contiguous active population. It relies on the existing lowered `chains_into` proof, not common planning: single undelayed non-sidechain input, exact predecessor, sole readership, no graph-output/intermediate direct or aliased observer (`crates/graph/src/runtime.rs:2813-2860`). Invoke the first owner's factory before converting these two owners into generic rack slots. Success emits one combined BuiltinStage; failure feeds the returned owners through the existing two-stage path. Other bank metadata, scratch ownership and graph op accounting stay with graph. Redundant scratch is reclaimed at bind exactly where chain_for already drops merged-slot scratch, never at render.

This is an implementable local ownership handshake, not a proposed general optimizer. No rack::BankStage redesign is required: the composite implements the same graph processor trait and is adapted once by existing BuiltinStage. Its logical two-stage counters/resource accounting and seam-side witness must remain explicit. Input stays outside the pair.

## 2. The actual cutoff evidence and its limit

The browser host really provides a between-render admission boundary:

- `hosts/host-web/src/lib.rs:546` keeps ReadyOwnership and its `controls` private. It does not hand those producer endpoints back to callers.
- `submit_commands(&mut self)` documents `port.onmessage` admission between process quanta on the same thread (`:1270-1286`). It reads `status.next_absolute_sample` before admission and reports that exact `applied_at_sample` only after successful admission (`:1297-1315`).
- `admit_commands_staged` validates/lowers, counts each destination's room, then pushes the complete accepted submission. ReadyOwnership::push dispatches into the actual fader/matrix producers (`:677-692`); successful enqueue updates in_flight (`:2170-2182`).
- `render_next(&mut self)` exclusively borrows the same WebEngine and renders at that next sample, advancing the clock and clearing in_flight only after render (`:1204-1237`). A safe caller cannot concurrently invoke submit_commands and render_next on the same instance. The Worklet sequencing reinforces this existing ownership rule; no new Web Audio assumption is needed.

Thus for this path the queues' producer positions cannot advance between fader drain and matrix drain during a render call. Moving the matrix drain earlier within the adjacent pair consumes exactly the same FIFO records. Both stages still apply every acknowledged record to the identical first sample. Capacity bounds the number drained because there is no refill during the call. Determine settlement after both drains; a new nonzero ramp forces separate whole-call arithmetic. Moving the matrix drain does not justify moving the input/effect begin_block drains or collapse decision.

The general embedded API does NOT supply that proof:

- HostConsoleHandles publicly exposes the TrackControlProducer vector (`crates/host-core/src/prepare.rs:314-320`), and TrackControlProducer publicly exposes all three Producer endpoints (`crates/builtins-compiler/src/lib.rs:215-232`). The plan and handles can be owned on different threads.
- TrackControlRecord and TrackFaderRecord carry target/smoothing data, not an application sample, batch epoch or cutoff (`crates/builtins-compiler/src/lib.rs:86-130`).
- Consumer::try_pop calls is_drained, which reloads the producer cursor whenever the cached cursor is exhausted (`crates/engine/src/realtime/spsc.rs:426-450`). A while-pop loop is not a snapshot of the queue at block entry and can observe newly arriving records. Consumer::is_empty is just a contemporaneous acquire observation (`:419-421`), not a no-future-writes guarantee.
- Existing bank fader and matrix drain at the start of their respective process methods (`crates/builtins-compiler/src/lib.rs:547-641`); the scalar processors do the same (`:3071-3119`). The existing bank command test explicitly admits commands between blocks (`crates/graph-compiler/src/lib.rs:5887`), so that test is not a concurrent cutoff proof.

A producer may enqueue a matrix record after an early paired drain but before the original separate matrix drain. Untagged queues cannot tell the pair that this record belonged to the already acknowledged/current block. A second late poll cannot preserve the original boundary while retaining one traversal: once fader samples have been rendered to reach the old drain point, the traversal has already happened. Empty checks, queue-length snapshots, moving all drains into begin_block, a new arbitrary per-block pop cap, or deferring the record silently do not solve equivalence. Do not introduce those changes under an arithmetic optimization.

## 3. Recommended concrete scope decision

**First live bank integration must be limited to an explicitly declared between-render delivery contract, with default concurrent/raw endpoint plans retaining the existing separate processors.** The currently proven shipped caller is WebEngine. W4 is its Wasm path; W8 can be exercised by the same serialized WebEngine native test path, not mislabeled as unrestricted native-host fusion. Keep scalar pairing in its separately named successor. Broad RT-4 remains OPEN for unearned native admission/integration obligations.

The delivery eligibility cannot be guessed from nonempty controls, backend, platform, `queue.is_empty`, current settlement or observed test usage. Carry an immutable preparation policy from the host path through the prepared artifact to the concrete bank owners, for example `BuiltinControlDelivery::{Concurrent, BetweenRenderCalls}`, defaulting every existing general preparation API to Concurrent. The BetweenRenderCalls contract explicitly requires the host to retain producer ownership and admit only between exclusive render calls; no runtime guessing or second drainer. Use a dedicated preparation entry used by WebEngine's private ReadyOwnership construction (`hosts/host-web/src/lib.rs:2624-2630,2790`), not a silent behavior change to `prepare_host_runtime_with_console` that still exports raw producers. The public low-level declaration, if exposed, is an explicit caller contract; do not claim the compiler can establish it for arbitrary hosts. The only production opt-in in this slice should be the source-proven WebEngine owner. Unknown/unproven callers retain Concurrent.

Because introducing that host-to-prepared-plan declaration crosses the previously scoped compiler boundary, number one bounded prerequisite before #430 implementation: **“Carry explicit between-render builtin control delivery into prepared bank ownership.”** It owns only the immutable policy/default, the dedicated preparation plumbing and WebEngine's proved opt-in. It must not fuse anything or change a queue, record, command ack, callback or DSP operation. Prove default/raw calls remain Concurrent, WebEngine's private path declares BetweenRenderCalls, no declaration is derived from queue contents, and the exact application-sample/all-or-nothing/backpressure browser tests remain unchanged and green. Record the caller responsibility plainly; do not market this declaration as enforced scheduling for arbitrary external hosts. This avoids expanding Luna's fusion attempt into discovery of host semantics.

After that prerequisite and #429 merge, #430 can implement the concrete factory/composite above with an immutable delivery guard. Fusion is forbidden for live Concurrent banks even if presently settled. The default path preserves original drains and records exactly. Console-free banks have no producer concurrency and may pair under ordinary structural eligibility, but they do not substitute for the required live serialized-path proof.

If the owner instead requires fusion for arbitrary concurrent HostConsoleHandles, that is a DIFFERENT prerequisite: establish an explicit sample/epoch admission contract with a bounded render cutoff and atomic batch/backpressure semantics before optimization. Existing untimestamped records do not earn it. That work would touch admission/record/resource contracts and deserves its own complete issue; neither #430 nor a host-policy flag may claim it delivered. Do not alter acknowledgements or silently drop/delay records to avoid that issue.

## 4. Gates added by this ruling, without another framework

The ownership prerequisite needs a small typed-policy/caller-path discrimination test and retained existing browser command-ack tests. #430 then needs actual serialized WebEngine/compiled bank tests proving both queues' same-block effects, post-drain eligibility, typed default fallback, exact state/PCM, and the existing post-fader meter/send boundaries; one test must leave another unobserved compatible cohort eligible. Verify the safe factory returns exact original owners on type/shape/policy decline and consumes each queue once on success. Use per-owner test witnesses or existing counters, not pointer labels alone.

Retain #429's exact kernel arithmetic and #430's earlier focused debug/release/realtime/resource/workspace/target/artifact requirements. No timing, broad graph rewrite, shared-consumer design, unsafe code or speculative fourth implementation round is authorized. Root should turn this ruling into numbered reciprocal scopes only after #441 delivery, then obtain the normal Astra scope approval before Luna attempt 1.

## Delivered-base confirmation and mandatory native retention

Root reports #441 delivered and #429 CLOSED at main `4b352b36ba33334ea2e0c6847c0e3ecf6e8ab33a`. Read-only Git comparison confirms no difference from the inspected candidate in builtins-compiler, graph, rack, host-core, host-web or the engine SPSC implementation. This ruling therefore applies to that delivered base. The unrelated dirty `scripts/check-dsp-research.sh` on root main was neither inspected nor changed.

Before narrowing #430, root must also number a reciprocal retained successor titled **“Establish explicit application-sample admission for concurrent live builtin controls and enable safe bank pairing.”** Its initial status is queued/unbriefed; the title retains both the missing admission prerequisite and the original native pairing outcome, without authorizing that broader implementation. Its eventual source brief must decide whether admission and pairing require two independently closable children. #430's serialized live-bank slice, the named scalar successor and this named concurrent-native successor collectively retain the original live integration goal. None alone closes RT-4/#349. The policy-plumbing prerequisite described above is separate from concurrent admission: declaring that WebEngine already serializes is not a solution for public raw concurrent producers.

## Numbered disposition

The numbered issues above satisfy the ruling's required scope split. Historical statements that numbering or merged-base confirmation is pending are superseded by this disposition: #441 delivered main `4b352b36ba33334ea2e0c6847c0e3ecf6e8ab33a`, and #442/#443/#444 are remotely open with matching local specs. No implementation starts until numbered Astra approval and applicable prerequisites. Root will freeze the post-#442 source before Luna assignment. Exact prior arithmetic, queue ownership, fallback, observation, accounting and qualification gates remain binding.
