# Reduce general graph fan-in in one output traversal

Astra ready-to-number scope, inspected main `d8304664e8015e764285b55837c2970577abbc51` and the exact #349 RT-3 inventory. Queued behind accepted and integrated #419 source because graph arithmetic/tests overlap. Do not edit or broaden active #419. Root must number/synchronize this issue and freeze the integrated base before assignment. No implementation, Git, Cargo or timing was performed in this brief.

## Product outcome and premise

`crates/graph/src/runtime.rs::reduce_plane` still performs sum2 over the complete output and then one full sum_into pass for every additional input. This is the general reduction, independently useful outside RT-2 folded cohorts. For N>=2 contributors, its source-level per-word operand loads are 2N-2 and output stores N-1. The replacement still reads ALL N contributors but writes the output once per word. Do not claim zero contributor loads, literal machine instruction counts or a guaranteed timing win.

Smallest closable slice: change ONLY the >=2 general reduction to vectorize across frames while accumulating all input contributions in the original order in an owned register value, then store once. Retain 0-input zero-fill and 1-input copy/in-place-no-op arms exactly. Arbitrary graph fan-in remains supported within configured resources; no bank-width contributor limit, MAX_TRACKS, chunked subgroup subtotal, per-render allocation or prepare-time per-edge pointer cache.

## Complete safe ownership strategy — no new arena API

A general simultaneous read-many/output-borrow API is unnecessary. Existing `ArenaLease::read` returns a shared slice; load one frame vector into an OWNED `Lane` value and end that slice borrow. Sequentially load each subsequent contributor into an owned value and update the accumulator. Only after all contributor reads for that frame vector have ended, call `ArenaLease::write` and store the accumulator. Repeat for each independent frame vector and then scalar tail.

Conceptually, for one frame-vector offset:

```
acc = L::load(lease.read(plane, first)[offset..])
for input in remaining inputs in their existing order:
    acc = acc.add(L::load(lease.read(plane, input)[offset..]))
acc.store(lease.write(plane, out)[offset..])
```

This is sequential safe borrowing through the already-audited arena. There is never a live shared input slice when the mutable destination slice is formed. The accumulator owns values, not references. Repeated shared source IDs are legal and contribute repeatedly. Muted sources continue to resolve through the existing read API to silence. Do not resolve raw arena addresses, keep a reference across the store, cache muting state, bypass access policy, or add unsafe. No engine source/API or allocation shape change is approved or needed.

The existing private prepared-input invariants still establish valid plane/output access and legal reduction inputs. Preserve the legal one-input self-alias NO-OP explicitly, including the existing muted-self behavior; do not replace it with a read/copy that would change that behavior. General output/input alias combinations rejected by the old prepared contract are not newly admitted or used in tests of the old unsafe simultaneous-borrow methods. Inspect and cite the bind/program output/input invariant at assignment. The sequential strategy itself creates no overlapping references, but that does not authorize a new graph alias policy.

Repeated lease access/muting checks occur per frame vector and contributor rather than once per whole input block. This is the explicit simplicity/performance tradeoff. Record it as a possible residual cost, measure descriptively once, and do not add a pointer-access architecture to chase a timing number. If this strategy cannot satisfy existing invariants or a necessary access API change is discovered, stop before code expansion and amend the numbered issue with a concrete ownership proof. A new arbitrary-fan-in borrowed-view API is architectural scope, not an implicit retry permission. No Class B arithmetic change is authorized.

## Exact source scope

- `crates/graph/src/runtime.rs`: keep the public/private graph interfaces; introduce at most a private generic >=2 reduction helper parameterized by `Lane`, dispatched with existing `FrameLane`. Zero/one arms unchanged. Use existing Lane load/add/store vocabulary, not intrinsics or a new public kernel taxonomy.
- Existing inline graph tests and `tools/console-workload/tests/chain_shape.rs`: focused independent arithmetic/ownership/plumbing coverage only.
- Existing `crates/graph/tests/rt1_direct_bank_alloc.rs`: minimally extend the SAME isolated test's prepared workloads to exercise repeated general reduction with allocator liveness/thread-scoped zero counters. Retain direct/folded proof already present after #419; no second parallel test racing its global mode, new allocator, or ordinary-unit allocator attachment.
- Numbered evidence/spec, one fresh matching arm/usage in existing runner/operator preflight, and the same immutable-source worklet pin/publisher/current-ABI/generated browser evidence files required by #399/#419 when actual Rust changes regenerate the artifact.

No lane production change is needed: `lane::Lane` and frame vector types are already dependencies. Do not reuse #419's bounded 1..8-contributor cohort kernel by subtotaling arbitrary inputs; that changes D9 association. No engine change, new dependency, generic reducer framework, render scratch, fold eligibility, routing/meter/automation change or deferred AArch64 qualification.

## Frozen arithmetic and independent tests

For N>=2 each output sample computes `(((x0+x1)+x2)+...)` using existing Lane::add semantics, independent vectorization across frames and scalar tails. No zero seed, tree reduction, subgroup sums, fusion, sanitization, flush/canonicalization, route transform change or reordering. The first two operands must retain their order as well as the later additions. No tolerance relaxation.

Freeze a TEST-ONLY reference implementing the old algorithm explicitly with existing `sum2_block` followed by `sum_into_block`, plus original 0/1 behavior. Do not let the #419 folded-oracle fixture call the newly changed reduce_plane and thereby compare two changed implementations as if independent. Redirect its reference through the frozen old primitive sequence or keep the existing explicit independent oracle if #419 already supplies one. This oracle refactor is required evidence within graph tests, not permission to change #419 production code.

Required representative gates:

1. Fan-ins 0,1,2,3,8,9,64 and at least one larger-than-64 count; repeated source IDs; silence ID and muted inputs; unmuted/muted legal one-input self-alias; independently asymmetric L/R planes. Poison destination and preserve unrelated buffers. No invalid-lease UB reproduction.
2. Frame lengths 1, below vector width, exact width, width+1, several vectors plus tail and128; instantiate scalar, Simd4 and Simd8 against the same old-width oracle. Existing hostile corpus includes signed zeros/subnormals/infinities/NaN payloads, with bitwise comparisons under the repository's actual FP environment and matching execution-arm behavior. Do not invent cross-platform NaN guarantees beyond existing primitives or weaken an observed identity mismatch.
3. Strong finite ordering witness `[16777216,1,-16777216]` gives0 with the old left association and1 when the small term is accumulated after cancellation. Add many-input sequences whose 8-contributor subgroup subtotal changes the result; explicitly compute both wrong and old outcomes in the test and require them to differ before asserting the implementation matches the old one. The oracle is the old primitive sequence, not the new helper against itself.
4. A private graph/prepared-program fixture establishes the actual nonfolded multi-input path, stable input order and no output alias under normal preparation. Keep prior graph redirect/PDC/observation/route-fold decline tests and #419 first/continuation cohort witnesses. General reduction must not invalidate the independent folded oracle.
5. Repeated real prepared graph renders through the general fan-in path perform zero audited allocations/frees after preparation; positive allocator liveness is required. Source/mechanism evidence shows output write only after the inner contributor loop, once per vector/sample. A callback/access counter in a small test seam is optional if it improves discrimination, but no new permanent telemetry or instruction-count byte gate is required.

The complete source slice should remain half-day bounded using the existing API and fixture corpus. Do not attach extra graph features or a new large qualification corpus to it.

## Actual existing workload and one descriptive measurement

The standing `SixtyFourTrackPlumbingOnly` console workload is the appropriate named row, not an assumption that every console row reaches reduce_plane. `tools/console-workload/src/lib.rs` explicitly binds NO builtin bank in this arm; the existing chain-shape test proves zero bank slots/round-trips, so routes cannot fold into a bank. Confirm after #419 integration that its prepared master reduction still has the intended64 ordered inputs. Preserve its output digest and frozen workload bytes. Strengthen only the existing test assertion if needed; no new timing workload or schema is required.

Register one fresh issue-owned runner/preflight namespace; preserve consumed #399/#415 namespaces. Freeze candidate, existing46-record workload/fixtures/floor/validators and actual profile/binary. Root completes non-timed committed-head preflight with zero launches, builds the exact runner profile before readiness, lets other work settle, and permits exactly one controlled invocation: one warmup and two measured rounds. Unchanged load ceiling0.50, cooldown60seconds, affinity/sibling checks; no uncontrolled override. Inspect the two plumbing p50 rows and output/structural identities explicitly. Other folded rows are contextual and cannot independently prove the general reduction mechanism. Compare historical timing only with stated comparability limits; no causal speedup from uncontrolled/different-profile evidence and no fabricated cycles.

Run unchanged complete record and aggregate validators, preserve raw/stderr/disposition/identities. Prelaunch refusal consumes the invocation and is preserved; no automatic retry or successor chain. A post-workload tooling failure preserves raw evidence and moves repair to an explicitly scoped tooling issue. Descriptive timing is not permission for performance retries or arithmetic changes.

## Delivery gates and workflow

Freeze main baseline/candidate workspace counts, focused graph/lane-applicable and console chain/identity suites in debug/release, realtime42/12 policy/mutations (or the accepted current marker count), lane/graph/workspace/audit policies, fmt/diff/clippy and supported Wasm/artifact/static/resource/browser gates. Keep native AArch64 deferred. Use isolated targets and no concurrent Cargo in one target. Actual artifact consumers follow the existing immutable-source-candidate convention; no unrelated publication.

Root synchronizes numbered issue/spec before implementation and owns checkpoint/pushes. Astra scopes/reviews; Luna attempt1, Sol only after FAIL, maxthree attempts then hard stop/rescope. No overlap with active #419; wait for its accepted integrated source and preserve its independent oracle before changing general reduction. After semantic PASS, finish frozen qualification, actual PR Astra review and required CI before merge/closure. Broader #349 remains open for its other findings.

## Numbered queue

Issue #420 owns RT-3 in #349. Astra supplied this approved scope; root synchronized it before implementation. It remains queued until accepted and integrated #419 source. The actual implementation base and unchanged workload/invariant checks will be frozen at assignment. No implementation is authorized to overlap active #419.

## Frozen implementation base and Astra clarification

PR #425 merged and #419/#422 closed. Root freezes actual merged main `36bf58730b9724e665ccd37049debb83f588baf4` for Luna attempt 1 in `codex/420-single-pass-reduction`. Both folded/default capacity and RT-2 ordered cohort source are accepted and remain unchanged.

# Astra #420 base-readiness confirmation

**Scope remains READY, conditional on #425 merging and root freezing the actual merged base before Luna assignment.** This confirms the existing numbered brief; it authorizes no overlapping implementation or new scope.

Current accepted #425 source still has the exact RT-3 premise: reduce_plane's >=2 arm performs one sum2 full-block pass followed by one sum_into full-block pass per remaining input. Existing FrameLane, lane::Lane and ArenaLease read/write suffice for the approved owned-value per-frame-vector accumulation. Keep 0/1 arms exactly, including legal muted-self no-op. Root should retain the standing source-level traffic claim and candid repeated-access cost, not infer guaranteed machine loads or a speedup.

Luna's inventory is useful and the minimal implementation shape is correct. Apply these precise clarifications when freezing assignment:

1. Ownership relies on EXISTING prepared lease/program invariants, not a newly general validated-input arena API. ArenaLease::read/write contain existing audited unsafe internally and write-set checks partly rely on established construction/debug assertions. The new graph helper introduces no unsafe and must retain valid plane/IDs/access sets; it does not earn arbitrary malformed-lease safety. Cite `program.rs` lower's `single`/`in_place` branch (around lines690–707), existing fan_in_keeps_its_reduction test, and the existing arena I1–I4 contract. Owned Lane values end shared borrows before each write. No read-many/pointer-cache/API change.
2. Keep reduce_case as a DUT helper for existing reduction tests. Add a separate test-only old primitive oracle; do not redirect every reduce_case test to the reference and thereby stop exercising production reduction. At this accepted source TWO folded/dispatch oracle sites call reduce_plane: the actual graph-wrapper oracle around lines3103/3104 and the routed folded-epilogue oracle around3267/3268. Redirect BOTH to the frozen old sum2/sum_into sequence when production changes. Parameterize the old-width reference as needed for scalar/W4/W8 comparison; no reference built from the new reducer.
3. The runtime source boundary is graph-only, but allowed test paths explicitly include chain_shape and the existing isolated allocation fixture. Luna's “edit only runtime.rs” describes production scope, not a reason to omit required evidence. The existing direct prepared allocation arm has four bank member outputs feeding the output directly, no prepared routes and bank_route_folds==0; it already reaches genuine general fan-in. Reuse and make that mechanism explicit, minimally extending its prepared fixture/assertions if needed. Repeated actual nonfolded general reduction zero allocations/frees is REQUIRED, not optional “if it can.” Retain folded and direct RT-1/RT-2 proofs and the single serialized allocator test function.
4. The existing plumbing workload's zero bank/transpose/fold assertions remain valid after RT-2. Confirm its actual64 ordered master inputs via existing private prepared/program access or an existing graph fixture; zero banks alone does not prove fan-in count/order. Do not introduce a new public introspection API just for this assertion. Frozen workload bytes and timing schema remain unchanged.
5. Retain all numbered discriminators: >64 inputs, repeated IDs, silence/muting, legal single self-alias, two asymmetric planes, poisons/sentinels, sub-width/exact/ragged/128 frames, matching-width hostile values, wrong-association and eight-input-subtotal counterexamples. This is the existing mandatory representative set, not discretionary test expansion. No new policy suite beyond those actually applicable; original old-PDC/route/observation/decline coverage remains.

The accepted #419/#422 complete-stride/fold callback implementation stays untouched in production. Any new arena or architectural access requirement requires a concrete amendment before expansion; no Class B reassociation or <=8 contributor shortcut is permitted. Existing arithmetic/fault cases must compare bits under established primitive semantics, never loosen to tolerance.

Proceed after base freeze with one coherent Luna source/test pass, root checkpoint, Astra adversarial verdict, Sol retries only after FAIL. Source PASS precedes retained fullworkspace, supported-target/immutable artifact/static/resource/browser qualification and the ONE controlled descriptive invocation through a fresh #420 namespace. Existing0.50/60second controls, zero-launch preflight, no automatic retry and final actual-head PR/required-CI review remain unchanged. Broader #349 remains open.

Read-only review of numbered spec, Luna inventory and current accepted graph/arena/program/chain-shape source. No tests, Cargo, timing, repository/GitHub mutation or implementation occurred.

## Luna attempt 1 source evidence

The general reduction's `N >= 2` path now accumulates owned `Lane` values across frame vectors and writes each output vector once, using sequential checked `ArenaLease::read`/`write` borrows. Zero-input and one-input behavior remain unchanged. A test-only old primitive oracle using `sum2_block` followed by `sum_into_block` was added and both folded oracle sites were redirected to it. Fan-in coverage includes 8, 9, 64, 65 and 129 contributors while preserving existing hostile/order/self-alias cases. Graph debug/release (49/49), release console chain fixtures (21/21), graph/realtime policies, and formatting passed. Logs are recorded under `/tmp/luna-420-*`; no runner, artifact, timing, full-workspace or Git operation was performed.

## Astra attempt 1 verdict

# Astra #420 attempt 1 review

**FAIL — bounded Sol revision required.** Reviewed exact pushed head `45a0619455a1fa69574bdcfea280a925ee7c98e3` in `/home/bl/misofm/engine-420`, full numbered spec, cumulative implementation diff and focused logs. No Cargo, timing or repository/GitHub mutation performed. This is the first coherent implementation verdict, not authorization for qualification or timing.

The production strategy follows the approved design: zero/one arms remain identical; the private generic >=2 helper owns each loaded Lane value, accumulates contributors in order and ends shared borrows before one output store per vector/sample. There is no contributor cap, subtotal, new unsafe, allocation, raw pointer or arena API. Prepared alias policy remains the existing `program.rs` single/in_place lowering contract. Both RT-2 oracle sites correctly use the frozen old primitive sequence. Existing finite coverage now includes 65 and129 inputs. No concrete production defect was established by this read-only review.

Acceptance is incomplete despite green existing suites:

1. **Matching-width arithmetic proof is absent.** The new helper is only dispatched/tested through host FrameLane; the old oracle hardcodes FrameLane and is only used by the folded fixtures. The finite reduction corpus compares the DUT to scalar `reduce`, not the assigned old-width primitive oracle. Add direct scalar/Simd4/Simd8 comparisons against separately instantiated old sum2_block/sum_into_block behavior, including sub-width/exact/width+1/multiple-vector-plus-tail/128 shapes and signed zero, subnormal, infinity and NaN payload representatives under the existing FP environment. Do not normalize bits or weaken tolerances. Existing signed-zero assertions and finite random cases do not meet this requirement.
2. **Required ownership/access representatives are missing.** Existing reduce_case poisons its output, but uses unique nonzero IDs on one plane. Existing self-alias test is unmuted. Cover repeated source IDs, silence ID, actual muted inputs, both legal single-input self-alias cases, asymmetric L/R, and preservation of unrelated poisoned/sentinel buffers using valid prepared leases. Keep zero/one behavior exercised through the DUT. Do not execute illegal multi-input destination aliases against old simultaneous-borrow code.
3. **The specified association counterexamples are missing.** Preserve the existing four-input epsilon witness, and add the frozen `[16777216,1,-16777216]` witness plus a many-input sequence discriminating an eight-contributor subtotal. Explicitly compute the wrong and old results, require that they differ, then compare DUT bits to the old primitive sequence. Finite random coverage alone does not establish these discriminators.
4. **The actual64-input prepared path remains unproven.** Existing plumbing assertions only establish zero banks/transposes/folds. Add the scoped private prepared/program proof of64 ordered master inputs and non-aliasing, tied to the unchanged plumbing construction; preserve existing graph/PDC/observation/fold tests. No public introspection API or workload/schema change is authorized.

The existing isolated allocation fixture is useful evidence, not absent evidence: its nonfolded direct arm has four member outputs feeding master, `bank_route_folds()==0`, positive allocator liveness,16 repeated renders, asymmetric PCM and zero audited allocation/free counts; the supplied debug log passes. Reuse this SAME serialized test and make its actual general-reduction mechanism explicit in assertions/documentation, extending minimally only if required. Retain the folded proof and mode restoration. Complete the focused debug/release evidence after the coherent revision; do not add another allocator framework or parallel mode-sensitive test. The current allocation log predates the final import cleanup (it reports a now-removed unused import); record final-source applicability candidly rather than claiming every log was captured from the checkpoint verbatim.

Sol gets one coherent revision within the existing allowed runtime/test paths and spec. Correct the current evidence claim that preserved hostile cases satisfy the assigned new matrix; name actual coverage and final logs. Source approach, old oracle redirects, >64 corpus and current policy results can be retained. No runner, fullworkspace, target/artifact or benchmark work until the completed source/test pass receives Astra PASS. No scope expansion or arithmetic relaxation is needed.

## Sol attempt 2 evidence revision

The accepted production reducer is unchanged. Graph tests now instantiate the DUT and frozen old `sum2_block`/`sum_into_block` oracle at scalar, Simd4 and Simd8 widths across sub-width, exact-width, ragged, multi-vector-tail and 128-frame shapes with signed zero, subnormal, infinity and NaN payload inputs. Valid leases cover repeated source IDs, silence, an actually muted contributor, unmuted and muted legal one-input self-alias, asymmetric stereo, poisoned destinations and unrelated sentinel preservation. The finite ordering and eight-input-subtotal counterexamples explicitly prove their wrong results differ before comparing DUT bits with the old primitive oracle. A private lowering fixture proves 64 plain plumbing tracks produce one ordered, non-aliasing 64-input master reduction. The existing single serialized allocation test remains the actual nonfolded N=4 prepared path: `bank_route_folds()==0`, asymmetric output, positive allocator liveness, 16 renders and zero allocations/frees; no second allocator fixture was added.
