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

## Post-policy executable scope amendment

Root adopts the following exact implementation route and finite gates from Astra. This is queued scope only while PR #450/#442 awaits actual-head review and required CI; no live-pair implementation is assigned. The new planning branch is isolated from the qualifying #442 candidate.

# #430 post-policy executable implementation brief

**Recommendation: amend #430 with the exact route below, then approve its delivered post-#442 base and assign the live bank product.** #442 is still the sole feature in immutable qualification; this is read-only queued design, not authority to edit that candidate. #238 remains independently queued. The smallest outcome is actual serialized live fader/matrix bank pairing, not further policy metadata.

Inspected accepted #442 candidate `dc55baf97074edf98abbfc9477aa6c420f0599af` in `/home/bl/misofm/engine-430-plan`, the full current #430 scope/ruling/disposition, `/tmp/astra-430-current-brief.md` and `/tmp/astra-430-ownership-admission-ruling.md`, and actual graph/builtin/compiler seams. Final merged-base confirmation remains required after #442 delivery. No implementation, source/spec changes, builds, timing or Git/GitHub mutation.

## Minimum product and admission eligibility

Pair actual adjacent PostFader/PostMatrix W4/W8 owners in a proven graph cohort chain when BOTH carry BuiltinControlDelivery::BetweenRenderCalls. This policy is now real in the private PreparedBuiltinsSession, both concrete owner fields and graph's owner-sourced prepared metadata. WebEngine's actual dedicated preparation path supplies it; default/raw Concurrent paths retain their exact existing separate processors and drain timing, even if presently settled. Do not infer policy from platform, width, queue length, targets or transient queue emptiness.

For a minimal auditable first implementation, use policy eligibility alone: console-free Concurrent plans may remain separate. The earlier ruling permits console-free pairing but does not require that extra exception; its omission must be stated explicitly, not confused with native live integration. The required positive is an actual live serialized plan with both real consumer populations.

WebEngine privately retains producers and serializes submit_commands/render_next through exclusive mutable ownership. Producer positions cannot advance between the pair's original drains. Moving the matrix drain earlier within this pair therefore consumes the same FIFO records for the same acknowledged application sample. The policy remains an explicit caller contract for other users of the dedicated entry, not a new enforced cutoff for public raw producers. #444 retains concurrent-native admission and pairing.

## Exact safe owner/factory route

Dependency direction remains builtins-compiler -> graph -> rack, with builtins-compiler also depending on builtins. Graph must not acquire a concrete builtin compiler dependency. Declare a narrow preparation-only factory alias in graph:

```rust
type BuiltinProcessor = Box<dyn GraphPreparedBuiltinBankProcessor>;
type BuiltinPairFactory = fn(BuiltinProcessor, BuiltinProcessor)
    -> Result<BuiltinProcessor, (BuiltinProcessor, BuiltinProcessor)>;
```

Add a default-None factory query on GraphPreparedBuiltinBankProcessor. Only concrete FaderBankProcessor advertises this factory. Use the already-proposed safe Any owner route: explicitly authorize the Any + Send supertrait/static prepared-owner constraint in #430's public Rust decision record, and inspect BOTH actual types before owned downcasts. Existing stored prepared boxes already require static ownership; check all current implementations before coding. This is a preparation metadata/API addition, not permission for an unsafe cast or render reflection. No factory registry, type-name string comparison or pointer-based eligibility.

The factory in builtins-compiler checks exact FaderBankProcessor/MatrixBankProcessor types, both delivery fields, backend, width and active population. On decline, return the exact input boxes untouched and in order. Type checks precede both safe owned downcasts; adjacent control-plane expect after proven type identity is sufficient. Recommended concrete result: FaderMatrixBankProcessor owns the two DOWNCAST Box<FaderBankProcessor> and Box<MatrixBankProcessor> fields. Keep the existing allocations, bank state, consumer arrays and logical counters; do not reconstruct parameters or duplicate consumers. This two-box composition makes failure ownership and preserved state explicit. A small outer composite allocation occurs only at bind and must be charged below.

Graph's RuntimeParts::chain_for (runtime.rs:1605) is the insertion point, before stage_for (:1682) turns a builtin owner into BuiltinStage. Walk the existing run in order with a two-entry lookahead. Attempt only adjacent builtin entries with PostFader then PostMatrix roles, equal ORDERED track membership (compare track identity, since GraphNodeId stage differs), equal width/backend/quantum and contiguous populated lanes. These are entries already admitted to the SAME run by existing cohort/chains_into proof. Do not manufacture adjacency from matching track names across runs. On factory success emit one BuiltinStage; on decline put the returned owners through the unchanged separate stage path. Other effect/input/unknown slots retain their current path.

Retain chains_into (:2813) unchanged: single undelayed input, no sidechain, exact predecessor, sole readership, no intermediate graph output, direct observer or aliased tap observer. A post-fader send adds readership; post-fader meters split the chain. No common-cohort or zero-latency shortcut can override those barriers. Existing chain scratch selection/drop remains at bind; do not add a scratch copy or runtime ownership transition.

## Exact live arithmetic bridge and fallback

The existing #429 `lane::kernels::builtins::fader_matrix_block` already supplies the arithmetic, but live BuiltinFaderBank and BuiltinMatrixBank currently expose only separate process methods and privately hold different stage enums. Add ONE narrow builtins-level method, for example BuiltinFaderBank::try_process_settled_with_matrix(&mut self, &mut BuiltinMatrixBank, left, right, frames) -> bool. False must perform no DSP/state mutation. It first checks compatible shape/enum widths, then all populated L/R fader integer countdowns and matrix countdowns are zero. A generic private helper can match Simd4/Simd4 and Simd8/Simd8 and call the existing fader_matrix_block with fader ramp.current/mute words and matrix.coef. Keep raw state private; do not add public setters, borrowed internals or another kernel implementation. Preserve the existing processing slice contract and safe mismatch refusal; no unsafe or runtime CPU dispatch.

The composite process drains fader records in original lane/FIFO order, then matrix records in original lane/FIFO order, using the SAME setter logic as the separate owners. Refactor small drain helpers locally so the separate Concurrent process still does its own drain immediately before its own arithmetic. Do not move these queues to begin_block, change input/effect drains or the collapse decision. Only after both drains call the narrow settled bridge. True runs one combined traversal. False runs original fader bank processing then original matrix bank processing for the ENTIRE call. A ramp that ends mid-call cannot switch to the combined kernel within that call; the next call may fuse. Never run the owner process wrappers after already draining, which would create another poll.

Preserve failure ordering as well as valid host commands: fader drain failure returns before arithmetic; if matrix drain fails after the fader drain succeeded, retain the original fader arithmetic/logical-counter completion before returning that error, with no matrix arithmetic. This prevents early matrix draining from silently erasing the fader side effects that the old ordered pair already performed on an invalid raw record. No new rejection rule is required. Normal validated WebEngine submissions remain the primary live proof.

Use exact existing gain then bitwise mute, coefficient-before-sample matrix products and unfused additions, identity selection of the faded samples, and final stores. No coefficient precombination, FMA, gain-one shortcut, alternate sanitization or exceptional-value policy. Existing input recovery remains upstream. The pair is seam-side, has the same SEAM_SIDE_WITNESS, and never supplies a one-plane process path; preserve duplication before the fader and all collapse agreement logic.

Increment each moved owner's logical process_calls/frames_processed once per completed logical stage, including successful fused calls. Return their componentwise saturating sum from the composite qualification_counters, matching the existing rack sum over two slots (rack/lib.rs:2443). Do not report one logical stage merely because one kernel traversal ran. Use test-only/private/disposable witnesses to distinguish fused versus fallback dispatch, rather than changing those logical counters or adding public host reports.

## Resource charge: explicit amendment, not deferred discovery

The recommended composite retains both existing owner boxes and allocates one new outer box containing two typed box pointers. Existing strip_processor_bytes currently charges each concrete owner plus its consumer array. Those charges must remain, because neither allocation disappears. Add a conservative allowance for the actual size_of<FaderMatrixBankProcessor> for each potentially pairable fader bank in the pre-bind resource estimate; charging all fader banks is an acceptable explicitly documented conservative bound if selection is not yet available there. Cover the outer allocation in both retained/peak totals and the maximum-single-allocation calculation before bind, not only total bytes. During bind the old owners remain alive, so existing owner charges plus the outer charge cover their coexistence. Existing scratch overestimation stays conservative; do not attempt an unrelated exact scratch-accounting optimization.

Authorize necessary corresponding capi resource_lifecycle mirrors/expectation changes and existing builtins-compiler/host resource tests in #430. This is control-plane resource bookkeeping, not a capi runtime/ABI change. Preserve exact cap acceptance/one-below refusal with an independently expressed owner layout; no guessed magic composite size or new accounting framework. Do not add HostConsoleHandles observation fields or retire aggregate construction again.

## Finite product proof

Use existing builtins live-ramp/mono and compiler/graph fixtures, extending only where the new mechanism needs a discriminator:

1. Actual prepared serialized live W4/W8 pair versus the original forced-separate owners with identical nontrivial fader/mute/crossfeed state, partial trailing population and real FIFO commands. Compare PCM bits, fader remembered gain/mute, matrix current/target/ramp state and logical counters after each call. The reference calls the old separate bank arithmetic, never the new bridge. Use private tests for otherwise-private state; no public diagnostic API.
2. A live render-path witness proves eligible plans execute the combined arithmetic. Replacing the actual graph pairing decision with separate dispatch must fail the SAME mechanism assertion while unchanged PCM remains expected. Directly calling a new helper is not proof the compiler selected it. Keep a Concurrent live plan as a structural and state/PCM fallback control, not an unearned native cutoff experiment.
3. A compact event sequence covers both queues at one application sample, multiple records/same queue, immediate changes, positive ramps, mid-ramp retarget, ramp ending inside the call, next-call settled fusion, gain changes while muted/unmute and reset. Verify actual host acknowledgement/application-sample behavior through the existing serialized WebEngine path. Delay/drop either drain or test settlement before drain must contradict focused state/PCM/dispatch assertions. Retain FIFO and existing rejection/backpressure tests.
4. Reuse `a_meter_leased_at_post_fader_splits_the_chain_and_still_meters`: nonempty published windows and PCM must match separate reference, the observed cohort remains separate and another compatible unobserved cohort fuses. Include nonunity post-fader send plus crossfeeding matrix and the existing aliased-observer barrier; post-matrix observation may remain eligible. Factory decline controls cover wrong concrete type, order/member/width/policy mismatch and untouched returned owners. Preserve existing hole/reorder structural refusal and legal trailing lanes.
5. Preserve input nonfinite recovery and mono seam/disengage tests, including asymmetric right output. Reuse the installed allocator audit with positive allocation AND free liveness off render and repeated eligible/ramp/observation-fallback renders with zero of both on render. No independent benchmark fixture corpus is required.

Focused debug/release tests, realtime/lane/unfused/resource/workspace gates precede immutable full workspace, supported native/browser-Wasm SIMD and relevant shipped artifact/static/browser qualification. W8 software fixtures are not an AArch64 or arbitrary-native-host qualification claim. No timing or speedup estimate is authorized; #431 must later measure a workload that actually selects these live pairs.

## Required numbered amendment and closure

Root should adopt the policy-only eligibility choice, exact graph/factory/two-box ownership route, narrow existing-kernel bridge, resource charge and finite proof above into #430 before assignment. Allowed implementation paths are builtins/lib.rs and existing focused tests; builtins-compiler/lib.rs; graph/lib.rs/runtime.rs; directly affected existing graph-compiler/host-web integration and graph allocation/resource tests; capi resource_lifecycle mirrors/expectations; numbered evidence. No production rack or host admission redesign, new dependency, benchmark tooling or broad corpus. Explicitly record the prepared trait's Any/static addition and unchanged HostConsoleHandles API.

After #442 is delivered, freeze/recheck that merged source and obtain Astra numbered approval, then Luna attempt1/Sol only after FAIL up to three total. Source PASS still precedes immutable qualification and actual-head PR/required CI. #430 closes only actual serialized live bank pairing; #443 scalar, #444 concurrent-native admission/pairing and #431 measurement remain retained, so broad RT-4/#349 stays open. This brief resolves implementation choices without authorizing changes to the current qualifying feature.

## Root execution decisions

Adopt policy-only pairing eligibility: BOTH actual owners must declare BetweenRenderCalls; console-free Concurrent plans remain separate. Adopt the safe preparation-only Any + Send/static trait addition, typed two-box composite and declined-owner return route exactly as described, after checking all current trait implementations. Adopt the one narrow builtins bridge to the existing settled kernel; no second arithmetic implementation or exposed state. Charge the actual composite size in retained/peak/maximum-single-allocation accounting, with the explicitly allowed conservative per-fader allowance and necessary CAPI test mirrors. Preserve logical stage counters, failure ordering, all observation barriers and current HostConsoleHandles construction.

The allowed paths and five finite product gates above are binding. This amendment supersedes earlier unresolved factory/bridge/accounting choices without weakening their correctness or retained outcome requirements. #443 owns scalar pairing, #444 concurrent-native admission/pairing, #431 measurement, and #238 remains queued independently. After #442 delivery root will integrate the actual default branch, synchronize this numbered scope and obtain Astra scope/base approval before Luna attempt 1. No benchmark or performance projection is authorized by this amendment.

## Delivered policy prerequisite and current base

PR #450 delivered #442 as main `452a327881bfd883c6c569b6606009a40b981e22` after exact-head Astra PASS and required qualification SUCCESS. #442 is verified CLOSED. Root integrated that actual delivered base here, preserving the new executable amendment in the document-only merge conflict. #430 is ready for Astra numbered-scope/actual-base approval; implementation remains unassigned until that approval. No runtime source differs from delivered main at this planning boundary.

## Numbered current-base approval and Luna attempt 1

# Astra #430 numbered live-bank scope/base review — PASS

Reviewed planning checkpoint `d8a570f8e2b01ab8ce112c5fb26426a353dfae3d` in `/home/bl/misofm/engine-430-live` against delivered #442 main `452a327881bfd883c6c569b6606009a40b981e22` and `/tmp/astra-430-post-policy-implementation-brief.md`.

PASS: the bounded actual serialized live W4/W8 bank product is ready for Luna attempt1. No further design amendment is required before assignment. HEAD is exact and clean; delivered main is an ancestor; the entire difference from that base is the #430 spec amendment. Relative to the previously inspected immutable #442 candidate, the relevant crate/host code is unchanged: the only listed differences under crates/hosts/tools and Cargo inputs are the delivered host-web qualification matrix, results and artifact pin.

Read the full numbered scope, historical ownership/admission ruling, adopted executable brief and final root decisions. The adopted route is faithful. The final decisions explicitly settle the earlier console-free allowance in favor of BOTH actual owners declaring BetweenRenderCalls, with all Concurrent plans remaining separate. The final delivered-base paragraph supersedes the preserved historical prerequisite-pending statements. Scalar #443, concurrent-native admission/pairing #444, measurement #431 and broad RT-4/#349 remain retained; this issue does not close those outcomes. Root reports #442 CLOSED and the matching GitHub #430 body synchronized; this read-only source readiness review does not claim an independent remote-state audit.

The exact source seams remain suitable:

- `crates/graph/src/lib.rs:546` owns the existing Send processor trait. Repository-wide Rust implementation discovery finds exactly the three concrete compiler owners (BuiltinBankProcessor, FaderBankProcessor, MatrixBankProcessor) and two owned test processors (CountingIdentityBuiltin, IdentityBank). Their actual types have no borrowed lifetime fields preventing the explicitly authorized Any/static addition. The default-None preparation factory, safe checks of BOTH types before owned downcasts, original-box return on decline and typed two-box composite preserve the current dependency direction. No render reflection or public HostConsoleHandles storage change is authorized.
- `crates/graph/src/runtime.rs:1605` still constructs the chain before `stage_for` erases each processor. Ordered adjacent builtin entries in the existing run provide the narrow pairing insertion point. `chains_into` at line2813 still enforces sole undelayed non-sidechain dataflow, predecessor identity, sole readership, no intermediate graph output and no direct or aliased observer. That proof remains unchanged; matching tracks alone cannot replace it.
- The real fader and matrix owners at `crates/builtins-compiler/src/lib.rs:539` and line615 still independently own their banks, consumer arrays, counters and delivery policies. Their existing process drains and setter/error order remain the reference. The composite's post-drain settlement test, whole-call fallback and matrix-error preservation of completed fader arithmetic/counters are explicitly frozen.
- The current builtins state still carries authoritative integer fader/matrix countdowns and the existing settled kernel is available. The single narrow bridge may inspect this private state internally and call that kernel; it must return false without mutation and must not introduce alternate arithmetic. Logical stage counter sums and the seam-side/two-plane contract are retained.
- `strip_processor_bytes` at compiler line683 and both estimation/materialization paths at lines1615–1670 remain the existing accounting seam. The amendment expressly retains both old owner/consumer charges and adds actual composite allocation size to retained/peak and maximum-single-allocation bounds, with the allowed conservative per-fader allowance and independent CAPI resource mirrors. No accounting premise relies on disappearing owner allocations.
- `crates/host-core/src/prepare.rs:494` remains the explicit caller-contract entry and `hosts/host-web/src/lib.rs:2630` the actual private WebEngine opt-in. Existing wrappers stay Concurrent. The previously earned serialized ownership/admission argument therefore applies on this delivered source; no universal native queue cutoff is claimed.

The five finite product gates retain actual live compiler-selected dispatch and its SAME-assertion separate-dispatch control, forced-separate state/PCM/logical-counter reference, real two-queue FIFO/application-sample and ramp/mute/reset behavior, nonempty post-fader meter/send/alias barriers with another eligible cohort, and existing recovery/mono/allocation-free render controls. They do not expand into another corpus or timing framework. Focused source acceptance must precede immutable workspace, supported target/artifact qualification and actual-head PR/required CI delivery.

This approves implementation scope and base, not an unimplemented fusion result. Use the existing Luna1 then Sol2/3 only after FAIL workflow, with one consolidated adversarial verdict per coherent attempt and the hard stop after three failures. #430 may now be the sole runtime feature while #427's independent tooling qualification proceeds; #238 remains queued.

No repository/spec edits, Git/GitHub mutations, builds, tests, benchmark or legacy inspection were performed. Only this /tmp review was written.

Root assigns Luna attempt 1 after this approval checkpoint is pushed and synchronized. #430 is the sole active runtime feature; #427 runs independent immutable tooling qualification. Exact scope, public-API decisions, accounting and five finite gates remain binding. Root owns the coherent checkpoint, push and GitHub evidence; Astra issues one consolidated verdict before broader qualification or any Sol retry.

## Luna attempt 1 compiling checkpoint

Luna returned a five-source-path implementation of the owner factory/composite, narrow settled bridge and graph bind-time selection. Its reported checks were cargo check and graph/builtins-compiler test compilation with --no-run; these are not executed semantic tests and do not establish the five finite product gates. Root independently ran `cargo check --locked -p builtins-compiler -p graph` on this exact source with exit0, retained in `/tmp/engine-430-attempt1-check.log`. No source acceptance, real live-dispatch proof, capi resource qualification, full workspace or timing is claimed. This compiling checkpoint is preserved for one consolidated Astra review of the whole attempt before further implementation.

## Astra attempt 1 verdict and bounded Sol attempt 2

# Astra #430 Luna attempt1 source review — FAIL

Exact clean checkpoint: `742722a14076201c3d4d1dc6d2310ffe95a7f2a7`, `/home/bl/misofm/engine-430-live`, compared with approved planning base `d8a570f8e2b01ab8ce112c5fb26426a353dfae3d`.

FAIL. The checkpoint compiles but does not implement reachable live pairing and has no executed semantic proof of the frozen product. The following four finite groups constitute one consolidated correction for Sol attempt2; preserve the accepted narrow approach and do not add a framework or broaden the issue.

## 1. Make actual graph pairing reachable and preserve decline ownership/scratch

In `crates/graph/src/runtime.rs`, the new condition requires the first left member to be PostFader and the first right member to be PostMatrix, then requires `left.members == right.members`. Those members are GraphNodeIds whose TrackStage variant includes the stage. The first elements cannot simultaneously have different stages and be equal. Every legitimate live pair therefore remains separate. Compare each ordered member's track identity while validating the respective stage for every lane, with equal population, width/backend/quantum and existing contiguous/run eligibility. Keep chains_into unchanged; do not substitute common track membership for its observation/dataflow proof.

Correct the adjacent decline paths at the same time: both `Err((left, right))` and `None` currently push stages and `continue` before the common scratch/active initialization. Once the impossible predicate is repaired, a run starting at such a declined pair can reach `scratch.expect` with None. Preserve the first slot's scratch/active exactly as ordinary separate stage construction does, including when the run starts at PostFader after an upstream boundary. Return/retain original processors in order and do not lose another slot or scratch ownership on any decline. Exercise that first-pair decline, not only an input-prefixed run that masks the omission.

The required actual compiled/rendered selection witness and same-assertion old/separate-dispatch control must prove this route is selected; direct helper tests or equal PCM from two separate plans cannot do so.

## 2. Preserve composite seam and shared drain semantics

`FaderMatrixBankProcessor` overrides seam_side but omits lane_symmetry, inheriting GraphPreparedBuiltinBankProcessor's DECLINED witness. Both original owners return SEAM_SIDE_WITNESS, and the frozen contract explicitly requires that same neutral seam-side witness. Restore it without enabling one-plane processing. Retain upstream collapse/duplication and prove asymmetric right-plane behavior under the existing mono/seam fixtures.

The new composite duplicates both original drain loops while the approved route calls for small local shared drain helpers using the same setter logic. Extract those helpers and use them from the separate owners and composite, retaining every original Concurrent drain at its original process point. The paired path must drain each queue exactly once, fader then matrix, before settlement; whole-call fallback must invoke bank arithmetic directly, not redrain through wrappers. Preserve the current attempted error ordering: a fader drain error precedes arithmetic, while a matrix drain error after successful fader drain completes fader arithmetic/counters before returning without matrix arithmetic. Verify these behaviors along with logical two-stage counter sums and state, rather than treating the new code's presence as proof.

The narrow builtins bridge's use of the existing kernel and integer countdown checks is the accepted approach. Preserve false-without-mutation, current gain/mute/matrix operand order, whole-call ramp fallback and next-call settled dispatch. No new arithmetic or sanitization is needed.

## 3. Complete the already-authorized resource proof and mirrors

Adding actual size_of<FaderMatrixBankProcessor> to the conservative per-fader processor charge is consistent with the approved two-box design. The old owners and consumer allocations remain charged. Existing resource folding propagates this aggregate into retained totals and the maximum-allocation bound, conservatively. Do not replace this with guessed savings or remove old-owner charges.

However `crates/capi/tests/resource_lifecycle.rs` and its independent primitive owner rows still describe only the old fader/matrix owners, and neither the CAPI mirror nor relevant executed resource/cap tests was supplied. Add the actual two-box outer-owner mirror/allowance to those existing independent expectations, preserving the explicit conservative charge for all fader banks (including Concurrent plans). Prove retained/peak/maximum-single allocation coverage and exact-cap acceptance/one-below refusal through the existing resource fixtures. Do not make the oracle merely call the production estimator or pin a magic composite size.

## 4. Supply and execute the frozen finite product gates

The six-path delta adds no semantic test: the only existing test edits are required Any forwarding implementations. Cargo check and tests --no-run cannot establish any of the five approved product gates. Implement the already-numbered finite cases using the existing fixtures and execute focused debug/release tests before the next coherent handoff:

- Actual serialized W4/W8 live owner pairs, legal partial population, and forced-separate reference: PCM bits plus fader/mute and matrix current/target/ramp state and logical counters after each call. The reference retains original separate arithmetic. Include true live dispatch and the actual pairing-to-separate production mutation failing the SAME mechanism assertion.
- Both actual queues at one acknowledged application sample, same-queue FIFO, immediate commands, ramps, mid-ramp retarget, ramp ending inside the call and next-call fusion, gain changes while muted/unmute, reset and existing rejection/backpressure behavior. Use focused state/PCM/dispatch assertions that discriminate delayed/dropped drains or pre-drain settlement. Include the actual serialized WebEngine acknowledgement/application-sample route; preserve Concurrent structural/state/PCM fallback.
- Nonempty post-fader meter windows and PCM against the separate reference, with another compatible unobserved cohort still fusing; nonunity post-fader send plus crossfeed, aliased observer and allowed post-matrix observation. Cover factory wrong-type/shape/policy and graph order/member/hole declines with untouched owners, including the scratch case above.
- Existing input nonfinite recovery and mono seam/disengagement, and the installed allocator audit's positive allocation AND free liveness off render plus repeated eligible/ramp/observation-fallback renders with zero allocations/frees on render.
- The resource tests in group3 and proportional formatting/realtime/lane/unfused/workspace policy gates. Retain actual commands, statuses and source identity. Do not substitute compilation for execution, software W8 for native target qualification, or an unexecuted mutant patch for causal evidence.

These are the original five finite gates consolidated around the concrete corrections, not a new corpus. Full immutable workspace/supported-target/artifact qualification follows source PASS under root ownership; no benchmark is authorized.

## Accepted boundaries and evidence

The preparation-only graph-owned factory alias, default-None hook, safe concrete type checks/downcasts and two retained typed boxes respect the approved dependency direction. Both actual delivery fields guard the factory; no runtime policy lookup, host admission change, queue format change, unsafe cast, rack renderer rewrite or HostConsoleHandles storage addition appears. Existing chains_into remains unchanged. The existing settled kernel is reused and no coefficient precombination/FMA or alternate identity/mute arithmetic was introduced. Preserve these decisions rather than redesigning them during correction.

Read the full #430 spec including binding scope/base approval and root's candid attempt record, the exact cumulative six-path delta, affected source and resource mirrors, and `/tmp/engine-430-attempt1-check.log`. Root's exact `cargo check --locked -p builtins-compiler -p graph` exit0 is accepted as compilation evidence only. No semantic execution or product acceptance is credited. No source edits, Git/GitHub mutations, builds, tests or timing were performed in this review; only this /tmp verdict was written.

Attempt1 is now a single recorded FAIL. Sol has attempts2 and3 available only under the existing workflow; a third FAIL requires hard stop/rescope. #443, #444, #431 and broad RT-4/#349 remain retained, and #427's independent delivery is unaffected.

Root authorizes Sol attempt 2 for these four finite correction groups. The compiling Luna checkpoint is retained candidly; no semantic acceptance is implied. Preserve the accepted route and complete the original five product gates with executed proof in one coherent pass. No new framework, arithmetic, host policy, timing or retained-scope reduction is authorized. Root owns the next checkpoint before further work.

## Sol attempt 2 implementation and focused evidence

Sol corrected the four finite Astra groups on clean pushed base `f0dac1071c3eb1e6daab14275dde080fbf1cb8c8`. Graph pairing now compares equal-length ordered track identities while validating PostFader/PostMatrix on every lane; the impossible full-node equality is gone. The unchanged `chains_into` proof still decides adjacency. Both factory-decline arms initialize the chain from the first fader slot's scratch and active mask before retaining the original processors in order.

The separate and composite owners now call one local helper per queue. The composite drains fader then matrix exactly once, preserves the established matrix-error ordering, decides settlement only after both drains, and calls bank arithmetic directly for whole-call fallback. It restores `SEAM_SIDE_WITNESS`, does not add one-plane processing, and retains two logical stage counter contributions. The settled builtins bridge and its arithmetic are unchanged from attempt 1.

The independent C-API primitive oracle now restates the outer owner as two typed box pointers and charges one actual mirror per potentially pairable fader bank while retaining both original processor and consumer-array rows. The resulting 32 bytes per nine-track prepared candidate are graph-plan payload; builtin preparation ownership remains independently 9,963 bytes.

Two focused product witnesses were added. The compiler test renders actual serialized W4 and W8 tail cohorts through preparation, graph binding and `PreparedRenderPlan::render`, compares exact PCM with Concurrent forced-separate plans, and observes both factory selection and composite execution. The exact temporary mutation `&& left.members == right.members` restored Luna's unreachable predicate; the same mechanism assertion failed `left: 0, right: 1` with exit 101. The mutation was removed and the final source passed; the failure log is `/tmp/sol430-pair-mutation.log`. The WebEngine test submits real fader and matrix records in one batch and proves both affect the first sample of the single acknowledged application block.

Focused executed evidence, all with `PATH=/home/bl/.cargo/bin:$PATH` and `CARGO_TARGET_DIR=/tmp/sol430-target`:

- `cargo test --locked -p builtins-compiler --lib`: exit 0, 18 tests, including actual selection, W4/W8 tail bit identity, delivery metadata and resource boundaries.
- `cargo test --locked -p builtins --test stage --test fader_ramp --test matrix --test input_liveness_mono`: exit 0, 35 tests covering full W4/W8 arithmetic, tails, immediate/ramped/mid-ramp/reset/mute/unmute state and PCM, false bridge fallback, recovery and seam behavior.
- `cargo test --locked -p builtins-compiler --features test-support --test allocation_tracker`: exit 0, 2 tests; the installed tracker observed the preparation allocations/frees and matched retained layouts/caps.
- `cargo test --locked -p host-web --lib` focused command acknowledgement, fader, mute, backpressure, paired fader+matrix and meter cases: each executed test exit 0. The new paired case passed in debug and release.
- `cargo test --locked -p host-core --test input_liveness_console --test symmetry_witness`: exit 0, 21 tests; asymmetric seam output, live drain/disengagement and recovery remained green.
- `cargo test --locked -p console-workload --test chain_shape`: exit 0, 21 tests; observed/aliased/collapsed cohorts and compatible unobserved cohorts remained green.
- `cargo test --locked -p graph --lib` and `cargo test --locked -p graph --test rt1_direct_bank_alloc`: exit 0, 52 and 1 tests; structural declines, ownership reuse, observation/dataflow barriers, exact PCM and zero render allocation remained green.
- `cargo test --locked -p capi --test resource_lifecycle`: debug and release exit 0, 4 tests each; retained/peak/largest coverage, exact-cap/one-below refusal, and positive allocation plus deallocation disposal balance passed.
- Release actual-selection test: exit 0. `cargo fmt --all -- --check`, `bash scripts/check-realtime-policy.sh`, `bash scripts/check-builtins-policy.sh`, and `bash scripts/check-workspace-policy.sh`: exit 0. The realtime policy counted 42 marked regions in 12 files.

The first policy invocation without `bash` returned 126 because the script is not executable, and a second builtins-policy invocation without the Cargo PATH returned 127; both invocation errors were corrected as shown above and do not represent a source failure. No full workspace, supported-target, artifact, benchmark, Git or GitHub action was performed. Root still owns the exact-path checkpoint and immutable qualification after Astra source review.
