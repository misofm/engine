# Reduce general graph fan-in in one output traversal

## Approved product and measurement ownership

#420 delivers the ordered one-output-traversal general reduction and retains its accepted arithmetic, ownership, realtime, workspace, supported-target and immutable artifact/browser qualification. The sole remaining descriptive measurement obligation transfers intact to #436. It owns the SAME existing `--issue420-rt3` authority and namespace, currently zero runner invocations; no new invocation is granted and #420 relinquishes authority. The bounded readiness monitor ended with load above 0.50 before any runner/workload invocation. No timing, speedup, benchmark completion or complete RT-3 audit closure is claimed. #420 may close only after the synchronized successor exists, actual final-head Astra PR PASS and required qualification CI success. #349 RT-3 remains open until the retained measurement is completed; other audit findings remain open independently.

This explicit Astra-approved amendment supersedes earlier delivery sequencing that held product closure for measurement. Historical evidence and the original single-invocation limits remain intact.

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

## Astra attempt 2 verdict

# Astra #420 attempt 2 review

**FAIL — one bounded final Sol evidence revision remains.** Exact reviewed head `460c485b2ea78161be656c46fabc0c5bed810c80` in `/home/bl/misofm/engine-420`. Full numbered contract, prior verdict, three-path checkpoint diff, current source/fixture and supplied logs inspected. No Cargo, test execution, timing or repository/GitHub mutation. Production reducer is unchanged; no new production correctness defect was established.

The revision substantially completes the assigned evidence: repeated IDs, actual silence/muted contributor, legal muted/unmuted self no-op, independent stereo values, destination poison and unrelated sentinel checks now exercise valid leases. The finite association and16-input subgroup witnesses explicitly distinguish wrong results and compare to old kernels. Both existing RT-2 references remain independent. Debug/release graph52 plus the SAME isolated allocation test1 pass; the direct prepared N4 path retains allocator liveness,16 renders, zero alloc/free counts and no route folds. No new allocator or runtime architecture is needed.

**Remaining blocker: the matching-width hostile matrix is arithmetically degenerate.** `assert_width_matches_old<L>` creates nine contributors by cycling through ten entries containing +infinity, -infinity and a NaN. Every frame omits only one entry. If it omits NaN it necessarily contains both infinities; otherwise it contains NaN. Thus EVERY expected output is NaN, regardless of width/frame shape. This is useful NaN propagation evidence, but the signed-zero/subnormal/finite words never survive to an observable result. The existing finite corpus dispatches only host FrameLane (W8 here), so it does not fill the scalar and W4 proof gap. Merely listing hostile inputs in the evidence does not establish their arithmetic.

Final Sol revision must make the existing old-width comparison discriminating, without broadening it:

- Keep the NaN case, and add a few separate input families whose OLD expected outputs actually retain the claimed category: all-negative-zero reduction, finite nontrivial/cancellation arithmetic, subnormal-preserving or deliberately FP-environment-qualified arithmetic, and infinities without automatically combining opposite signs/NaN on every sample. Use same-width old sum2/sum_into primitives; compare bits, never normalize/tolerate them. Where FTZ legitimately removes subnormals, explicitly record/test that existing convention instead of promising a nonzero subnormal.
- Instantiate those independent families at f32/W4/W8 over the already frozen lengths. Assert the intended finite/signed-zero/nonfinite category on representative old outputs so a future corpus edit cannot silently return this to all-NaN coverage. This is a small parameterized extension, not a new Cartesian corpus or benchmark.
- Re-run the affected graph debug/release tests (including existing isolated integration proof as root chooses for final-source qualification), then focused hygiene. Retain all passing ownership/order tests; no source rewrite or extra framework is justified by this verdict. Record the resulting actual coverage candidly.

The new `program.rs` test is valid private test-only scope:64 plain tracks produce a64-input master, stable route buffer order and no output alias through real lowering. It is a representative synthetic plain graph, not a direct invocation of the console workload. The frozen brief allows an existing graph fixture, so no public introspection or circular console dependency is required. Complete its evidence linkage explicitly: the named console model loads `console-sixty-four-track-intended.json` (64 tracks/64 routes/one main-out), applies PlumbingOnly while retaining routing, checks track count64, and uses the unbanked GraphCompiler path. Existing chain_shape checks zero banks/transposes/folds. Record/check the actual route destination and stable ch00-main…ch63-main order alongside this representative lowering result; do not call the synthetic test alone an inspection of the benchmark's prepared plan. This can be source/fixture evidence, not a new framework or runtime edit.

Policy scope must remain honest: the current real policy runs and legacy suites pass, but historical lane fixtures still emit missing-sidecars diagnostics, whose producer-status repair is owned by #410. They do not demonstrate that defect is fixed here. No weakening or unrelated gate repair belongs in #420.

This is the coherent attempt2 verdict. Root may assign Sol the FINAL attempt3 above and checkpoint it before review. Source/evidence PASS must precede fullworkspace, targets/artifacts, runner work and the one descriptive invocation. Attempt3 failure is a hard stop/rescope, not permission for another unnumbered repair.

## Final revision category clarification

# #420 final-attempt case clarification

This is a concrete implementation aid for the already issued attempt2 FAIL, not another verdict, new gate or authorization to change production. Keep the existing old-width primitive oracle, f32/Simd4/Simd8 instantiations and lengths `1, max(W-1,1), W, W+1, 3W+1,128`. A handful of separately named nine-contributor families is sufficient; do not build a Cartesian corpus.

**Use the existing canonical FP guard around BOTH old and DUT execution in this test:** `lane::fpenv::CanonicalFpEnv::enter()`, retaining its guard until comparison completes. This is the actual render policy, not a newly assumed flush convention. Current fpenv.rs explicitly pins round-to-nearest-even with FTZ/DAZ disabled; Wasm has full subnormal arithmetic. D7's separate recursive-state flush does not belong in sum2/sum_into or general reduction. Do not mutate raw control words, add unsafe, accept zero because the ambient host happened to enable FTZ, or compare outputs produced under different environments. The existing guard restores the caller's thread environment on scope exit.

For each output frame use one of these rows as its ordered contributor values; pad to nine contributors as stated:

| Family | Ordered values for one frame | Required OLD-output assertion before DUT bit equality |
|---|---|---|
| Finite arithmetic/order | Alternate frames between `[2.0,-0.5,0.25,+0.0…]` and `[16777216.0,1.0,-16777216.0,+0.0…]` | First pattern exactly1.75, second exactly+0.0; all outputs finite. Keep existing explicit wrong-association/subtotal tests as the independent wrong-result proof. |
| Negative zero | All nine inputs `-0.0` | Every output exactly `0x80000000`, not numeric equality with zero. |
| Small normal | `[f32::MIN_POSITIVE,f32::MIN_POSITIVE,+0.0…]` | Every output exactly `0x01000000`, nonzero finite normal; this also detects inappropriate blanket state-style flushing. |
| Subnormal | `[f32::from_bits(1),f32::from_bits(1),+0.0…]` | Every output exactly bits2, nonzero subnormal under the canonical environment. No tolerance or category-only comparison. |
| Infinity without invalid cancellation | Alternate frames `[+infinity,1.0,+0.0…]` and `[-infinity,-1.0,+0.0…]` | Exact signed infinity according to the frame; no NaN, and no opposite-signed infinity in the same sum. |
| NaN separately | `[f32::from_bits(0x7fc04201),+0.0…]` | Old result is NaN, then DUT bits equal that SAME-width old result. Do not assert one universal NaN payload across architectures or unrelated execution arms. The existing richer NaN rotation may also be retained but is not needed for the other categories. |

The frame1 shape naturally exercises only one member of an alternating family; larger existing shapes cover both. If convenient swap which finite pattern begins a run so the ordering pattern is also directly tested at frame1; the existing dedicated frame1 ordering test already proves that host path, so do not expand this into another matrix.

Generate buffers from these rows, execute the unchanged old sum2 followed by seven sum_into steps, assert the category/exact value specified, then compare every DUT bit. Runtime-loaded values and existing lane kernels avoid treating a hand-computed scalar expression as the only oracle. Both vector body and scalar tail remain exercised at each width. Nine contributors already meet this targeted repair; existing finite65/129 tests and16-input subgroup witness remain intact.

The only other completion from the attempt2 report is a concise honest workload-link record: name the console fixture's64 tracks and64 ordered post-matrix routes to main-out, the PlumbingOnly transformation's preserved routing, the track-count assertion/unbanked compiler arm and the representative private64-input lowering test. Checked read-only source/fixture evidence is sufficient; no public runtime inspection API or additional workload is required. Do not describe the synthetic lowering test as if it directly instantiated SessionRuntime.

Retain accepted ownership/sentinel/allocation/source tests. Run the proportional affected debug/release proof after the single coherent revision; root owns checkpoint and final Astra review. No source, timing, artifact or runner work is authorized by this clarification. Read-only current FP policy and old kernel inspection only; no tests/Cargo/timing or repository/GitHub mutation performed.

## Sol final implementation evidence

The width-specific old-primitive comparison now runs six independent nine-contributor families
under `CanonicalFpEnv`: finite/order, all-negative-zero, small-normal, subnormal, signed infinity
without invalid cancellation, and NaN. It asserts each old result's required exact bits (or NaN
category before same-width bit comparison) before comparing every DUT output bit, at the frozen
frame lengths for scalar, Simd4 and Simd8. Production code is unchanged.

The named `SixtyFourTrackPlumbingOnly` model loads
`console-sixty-four-track-intended.json`, whose 64 tracks have 64 ordered post-matrix routes
`ch00-main` through `ch63-main`, each targeting `main-out`. `apply_strip(PlumbingOnly)` clears the
effect racks but retains that routing; `SessionRuntime::build_full` asserts the model has 64 tracks
and selects the unbanked `GraphCompiler::compile` arm. The existing `chain_shape` assertions pin
zero banks, transposes and folds. Separately, the private `program.rs` lowering test represents the
same plain graph shape and proves an ordered, non-aliasing 64-input master reduction; it does not
instantiate `SessionRuntime` or inspect the benchmark's prepared plan.

Final-source focused qualification passes: graph debug and release each run 52 unit tests plus the
one isolated allocation integration test; release console `chain_shape` runs 21 tests. Graph,
realtime (42 marked regions in 12 files), lane, workspace and realtime-audit-leak policies and their
mutation/control suites pass. The lane mutation suite still prints the known missing-`sidecars`
diagnostics owned by #410 before reporting success; this revision does not claim to repair them.
Formatting, diff hygiene and graph all-target clippy pass; clippy retains the existing unreachable
`math::fast_db` allow-list warnings. Logs are `/tmp/sol-420-final-graph-debug.log`,
`/tmp/sol-420-final-graph-release.log`, `/tmp/sol-420-final-console-release.log`,
`/tmp/sol-420-final-policies.log`, and `/tmp/sol-420-final-hygiene.log`.

## Final source acceptance and qualification freeze

# Astra #420 FINAL attempt 3 source review

**PASS at exact pushed head `a7549425c2f1be34c259c6253eee79b5084f5565`.** Root may proceed with the remaining frozen delivery qualification. This is semantic/focused-evidence acceptance, not a benchmark launch or merge approval. Reviewed complete frozen contract, prior verdicts/final clarification, current source and final two-file delta plus named logs. No Cargo, tests, timing or repository/GitHub mutation performed.

The final revision resolves the sole remaining arithmetic discriminator gap. Each f32/Simd4/Simd8 comparison now runs six separate nine-contributor families across the frozen sub-width/exact/ragged/multiple-vector-tail/128 lengths. The old primitive outputs are checked before DUT equality: finite1.75/order0, negative-zero bits, small-normal0x01000000, subnormal bits2, signed infinity without invalid cancellation, and separate NaN category. Both computations share the existing CanonicalFpEnv guard, which restores the caller's environment and provides the actual non-flushing render contract. NaN output bits are compared only against the same-width old result, without invented universal payload pins. The families can no longer all disappear into NaN.

Production remains the previously reviewed bounded owned-Lane accumulation: original zero/one arms, original contributor order, no initial zero seed/subtotals, arbitrary configured fan-in and one write per vector/sample after all contributor reads. No new unsafe, access API, allocation, alias policy or arithmetic change has been introduced. The repeated lease/muting checks remain the candid simplicity cost; no machine-instruction or timing claim is earned by source inspection.

All previously accepted focused obligations remain present: finite65/129-input coverage; explicit association and eight-input-subtotal wrong-result witnesses; repeated/silence/muted IDs, both legal single self-alias states, asymmetric planes, poison and unrelated sentinels; both RT-2 folded oracle sites redirected to the independent old algorithm; real64-input non-aliasing/stable-order private lowering fixture. The final record accurately links the unchanged console fixture/model transformation and unbanked path without pretending that the representative lowering test directly instantiated SessionRuntime.

Final supplied logs establish graph52 unit tests plus the SAME isolated allocation integration test1 in both debug/release, and release console chain_shape21. The allocation fixture retains positive allocator/free liveness, repeated actual nonfolded N4 prepared renders and zero audited alloc/free counts alongside the existing folded proof. Current realtime42/12, relevant policies, fmt/diff and clippy pass. Known historical lane fixture missing-sidecars diagnostics remain explicitly owned by #410; no repair is claimed here. Existing clippy allow-list warnings are recorded rather than suppressed.

No remaining frozen source acceptance blocker was found. Preserve source while root completes baseline/candidate fullworkspace, supported targets and immutable artifact/current consumers/browser/static/resource qualification, then matching runner/preflight registration and the ONE controlled descriptive invocation under the frozen contract. Zero-launch preflight and quiet readiness precede timing; refusal/failure consumes its authority and is preserved. Exact final PR Astra review and required CI remain mandatory before merge/closure. #349's other findings remain open.

Root normally integrated merged main `4557865ee1fa8f8381ed75e7eace91d15b649d27` and registers only the matching `--issue420-rt3` runner/preflight arm and usage. Workload, validators, floors, controls and invocation budget remain unchanged. This checkpoint freezes the source for retained workspace, supported-target, immutable artifact/browser and one controlled descriptive capture. None of those pending gates is presumed complete by source PASS.

## Completed immutable-source delivery qualification

Frozen source/artifact/preflight candidate `51e2aed211b30523076e0e8dd07973b13b57dc11`. Fresh isolated `cargo test --locked --workspace` (including doctests) terminated exit 0 with 274 result blocks, 1,569 passed, zero failed, 24 ignored. The exact integrated main `4557865ee1fa8f8381ed75e7eace91d15b649d27` baseline terminated exit 0 with 274 blocks, 1,566 passed, zero failed, 24 ignored; three added graph tests account for the delta. Logs `/tmp/engine-420-candidate-workspace.log` and `/tmp/engine-main-455-workspace-baseline.log` reach the final doctests.

Independent REPIN and normal verification builds reproduce SHA-256 `24f81af304e541ba0e734de5c7a3dc5221e71fa4de73f2545edea3c2960761fe` in `/tmp/engine-420-worklet-verify.y85Za8`. Static/object checks, resource comparison with 26 rejecting mutations, hermetic worklet mutations, recorded Chromium/Firefox/WebKit and all three individual check-matrix legs terminated exit 0. Current pin, publisher equality, browser lineage and ABI prose use the exact source/digest; historical identities remain. Logs `/tmp/engine-420-{worklet-repin,worklet-verify,check-web,expected-resources,hermetic-worklet-mutations,browser-all-record,browser-chromium-check,browser-firefox-check,browser-webkit-check}.log`.

Supported scalar 18-package closure, explicit scalar/SIMD cfg, SIMD smoke/protocol checks and both protocol export/execution legs passed. The named scalar non-LTO engine/source/target_smoke supplement checks complete find/sort/archive/member populations, each decoder and each atomic scan, with three objects and no opcode matches. A separate same-member/hash supplement completes checked observation scans (statuses 0/0/1, two matches, source fallback unconsulted) and pointer-atomic presence/atomics-feature absence cfg predicates. Astra accepted that exact scope in `/tmp/astra-420-supported-wasm-review.md`; no claim about fat-LTO inspection or unexecuted fallback error handling is made. Procedures/logs `/tmp/sol-420-nonlto-inspect.py`, `/tmp/sol-420-nonlto-inspection.log`, `/tmp/sol-420-wasm-supplement.py`, `/tmp/sol-420-wasm-supplement.log`, `/tmp/sol-420-supported-wasm-summary.log`. #427 retains the actual gate repair.

Existing operator preflight terminated exit 0 with zero workload launches (`/tmp/engine-420-benchmark-preflight.log`). Its invalid-round rejection produces the expected abort diagnostic before a workload; no timed capture occurred. The exact runner-profile build (opt-level 3, LTO false, codegen-units 16) terminated exit 0 (`/tmp/engine-420-runner-profile-build.log`). All local build/browser processes are terminal. Quiet readiness, the sole controlled invocation and actual PR/required-CI delivery remain pending. Later evidence/current-consumer commits do not change the frozen runtime/workload/validator source.

## Retained qualification and deferred readiness

Original qualification logs/procedures and accepted versus historical source reviews are now retained under `artifacts/issue420-qualification/`, with byte/hash manifest and exact main455 workspace baseline. A bounded readiness monitor at candidate `66827b87b7210931c0a76a6dfcbcd00419ca3683` completed twelve probes with load above 0.50 and terminated exit 1. No runner or workload was invoked; the sole measurement authority remains unconsumed. The quiet window ended and independent #411 implementation resumed. No source, workload, validator, profile, controls or gates changed.

## Integration of delivered policy traversal repair

Normal merge `c2bcd21907bfce50bef2b9742e67cd6c148955a0` integrates main `1af76181490a623675960c244a6c677c06aae745` (#410). Its delta is confined to the realtime/lane checkers and their mutation suites plus issue records; runtime, workspace test population, workload, validators and artifact source are unchanged. Both actual policies and both mutation suites terminated exit 0 against the integrated RT-3 tree, including realtime 42 marked regions in 12 files. The exact log is retained as `artifacts/issue420-qualification/engine-420-post410-policies.log`. This completes the known missing-sidecars fixture repair through its own accepted delivery; the earlier immutable-source logs remain historical evidence. The sole controlled descriptive invocation and actual PR/required-CI delivery remain pending.

## Astra delivery split ruling

# Astra #420 product / descriptive-qualification split ruling

**APPROVED, conditional on root numbering and synchronizing the single successor and reciprocal amendment BEFORE product PR closure.** This applies to current product head `e55e86bba6b375b0fdfa7ee0cab872b12f17381f`; it is not final actual-PR acceptance or a timing authorization. The source's three-attempt process is complete and PASS; this split does not authorize a fourth implementation repair.

The remaining descriptive measurement is held by external host load, not a correctness defect, measured regression or named release budget. The standing smallest-product/qualification separation rule therefore supports shipping the already useful runtime correction while retaining a concrete measurement obligation. No remaining decisive source acceptance gap was identified by this scope review. Existing actual-head PR and required CI gates remain mandatory.

Two corrections to Sol's proposal: workspace counts are NOT unchanged—the qualified candidate is274 blocks/1569 passed/0 failed/24 ignored versus baseline274/1566/0/24, with three added graph tests. The runner identity is NOT missing: retained manifest/readiness evidence names SHA-256 `53dce85d8ff683693598da8dce79195ecfad1ad76300b68bb348196c00f81bab`, and I independently hashed the existing `target/release/bench` to that exact value. No rebuild or repeated preflight is currently justified. Recheck bytes before eventual execution because files can subsequently change.

## Exact reciprocal #420 amendment

Root can use this text, substituting the successor's actual number:

> #420 delivers the ordered one-output-traversal general reduction and retains its accepted arithmetic, ownership, realtime, workspace, supported-target and immutable artifact/browser qualification. The sole remaining DESCRIPTIVE measurement obligation transfers intact to #N. It owns the SAME existing `--issue420-rt3` authority and namespace, currently zero runner invocations; no new invocation is granted and #420 relinquishes authority. The bounded readiness monitor ended with load above0.50 before any runner/workload invocation. No timing, speedup, benchmark completion or complete RT-3 audit closure is claimed. #420 may close only after the synchronized successor exists, actual final-head Astra PR PASS and required qualification CI success. #349 RT-3 remains open until the retained measurement is completed; other audit findings remain open independently.

Retain all historical records and failed-readiness observations; do not replace them with a fictitious benchmark refusal. Monitoring failed readiness is distinct from invoking the runner and has not consumed the authority.

## Ready-to-number successor body

Title: **Complete the retained RT-3 descriptive measurement authority**

This is the ONE qualification successor of #420/#349 RT-3. It transfers rather than renews the existing authority. No source arithmetic, validator, fixture, controls or runner namespace change is authorized. Queue until #420 product delivery is merged, a dedicated frozen checkout is verified and a coordinated quiet window is available. Root owns synchronization/checkpoints and execution; Astra reviews the numbered contract and final evidence/PR. No implementation is authorized merely by queueing this issue.

### Frozen subject and existing preparation

- Accepted immutable runtime/workload/validator source: `51e2aed211b30523076e0e8dd07973b13b57dc11`; final source verdict: `a7549425c2f1be34c259c6253eee79b5084f5565`.
- Worklet qualification source51e2aed2, reproduced artifact SHA-256 `24f81af304e541ba0e734de5c7a3dc5221e71fa4de73f2545edea3c2960761fe`. This is source provenance, not the timing binary hash.
- Existing runner profile: opt-level3, LTO=false, codegen-units16; exact runner binary SHA-256 `53dce85d8ff683693598da8dce79195ecfad1ad76300b68bb348196c00f81bab`.
- Retained evidence and hashes: `artifacts/issue420-qualification/manifest.json`, original preflight/build logs, readiness-deferred.log, source/workspace/target/artifact/browser records and later post-#410 policy integration record. Workspace candidate274/1569/0/24 versus baseline274/1566/0/24 is correctness evidence, not a timing result.
- Existing preflight completed with zero workload launches. A bounded12-probe readiness observation at66827b87b7210931c0a76a6dfcbcd00419ca3683 ended exit1 because load remained above0.50; runner_invocations=0, workload_process_launches=0. No output/refusal directory is consumed by that monitor.

### Authority and identity gates

The ONLY permitted runner command remains `bash scripts/run-console-benchmark.sh --issue420-rt3`, with corresponding existing operator-preflight arm and `issue420-rt3` namespace. Do not register a successor-number arm, reset counters, remove an output/refusal directory, reuse historical namespaces or create another authority.

Before another readiness window, record actual product merge SHA and exact execution checkout head. Verify with actual Git/file/byte comparisons that runtime, workload/fixtures, record/aggregate validators, floors, runner/profile/controls are equivalent to the frozen subject (allow the already reviewed #410 policy-only and evidence integration, not unreviewed feature changes). Compare retained manifest entries to real files and the existing executable to the exact binary hash above. Do not substitute prose claims or current main after another feature has landed. Use an isolated retained source-equivalent checkout as necessary; preserve history.

If bytes/provenance drift, STOP for a bounded Astra ruling before substitution; no automatic rebuild, preflight loop, alternate profile or authority refresh. Existing successful preflight/build remain applicable when their subject is unchanged. A necessary later zero-launch preflight may be explicitly justified by changed preparation metadata, but it cannot create a second invocation. Do not rebuild merely because the qualification issue number changed.

### Single controlled capture and acceptance

Keep the existing full46-record workload and validators, one warmup/two measured rounds. The named general-reduction subject is SixtyFourTrackPlumbingOnly; inspect BOTH round records explicitly. All other rows remain the unchanged full workload, not a new benchmark or claimed proof that every row exercises general reduction.

Builds/tests/browser work must be terminal and local work quiet before readiness. Retain exact load ceiling0.50,60-second cooldown, fixed affinity/sibling checks and all existing admissibility limits. No uncontrolled override. Coordinate a bounded readiness window; unsuccessful monitoring before invoking the runner leaves the SAME authority unused, not a reason for an automatic retry loop or successor chain.

Invoke the runner at most ONCE. Any runner invocation—including prelaunch refusal before workload, overwrite rejection or later tooling failure—consumes this authority. Preserve raw stdout, stderr, readiness/preflight, disposition, binary/source identities and all terminal statuses. Never delete/refill the namespace or rerun to improve a number.

On success, run unchanged full per-record and aggregate validators over all46 records, verify1 warmup/2 measured rounds and exact invocation/process counts, and verify emitted output/structural identities against the retained previous console evidence. Preserve absent cycle fields as unavailable. Report the two plumbing p50 observations, source-level reduced output traffic and repeated-access cost separately; no causal speedup from nonidentical hosts/profiles or fabricated machine counts.

A refusal/failure is an honest terminal consumed-authority record, not successful measurement or permission for another automatically numbered attempt. Preserve it and request explicit disposition/rebrief only if needed; this issue/RT-3 cannot be represented as measured/fully closed by refusal alone. A successful complete capture plus exact-head Astra evidence PR review and required CI permits successor closure and RT-3 tracker update. #349 remains open for other findings.

Allowed changes: this numbered spec/evidence, reciprocal #420/tracker accounting, and capture evidence in the existing issue420 namespace. No runner/helper/workload/validator production edit, new framework, Rust/artifact rebuild or publication belongs in this transferred qualification slice. If a real tooling defect surfaces, preserve the consumed outcome and seek a separately justified bounded ruling; no implicit repair-and-rerun.

## Adoption sequence / review limits

Number and synchronize the successor and reciprocal transfer first. Final product PR Astra review must still check actual cumulative source/qualification/current artifact consistency and required CI; this ruling does not replace it. Product closure can report the runtime capability delivered with measurement explicitly pending; do not mark #349 RT-3 complete early. Freeze and verify the successor execution subject before scheduling its single capture. This cleanly separates descriptive scheduling from mandatory safety/correctness without weakening either.

Read-only proposal/spec/manifest/readiness inspection and existing binary hashing only. No tests, build, preflight, readiness, timing, source or GitHub mutation performed.

Root's additional `/tmp/engine-420-split-source-identity.json` was inspected: checked Git diff exits0, enumerates all60 changed paths and records no unexpected runtime/workload/validator/profile changes; all changes are accepted policy integration, current consumers or evidence. The existing binary hash matches, reserved measurement artifacts are absent and worktree is clean. Retain this concrete transfer-time identity record with the successor. It supports the current unconsumed-equivalent-subject ruling; repeat actual byte/source checks before eventual execution rather than relying forever on this snapshot.


## RT-3 measurement acceptance under #436

The transferred authority was consumed successfully at execution commit `1fc6ed1ed3fe57ccac0b47899a1cbe36c9050571`: one controlled invocation, one warmup, two measured rounds, all46 records accepted. Astra independently recorded capture PASS in `artifacts/issue420-rt3/astra-436-measurement-review.md`. All46 keys and the enumerated stable output/workload/counter identities match #419. Exactly42 records report zero render errors and forbidden operations; the four hoist records omit those fields. Plumbing-only p50 is6.670/6.710us per block; no cycle evidence or causal speedup is claimed. The source reduces repeated output traffic but retains checked-view access overhead. Prior unused-authority and deferred-readiness statements above are historical; no further invocation is authorized. Actual PR review and required qualification remain the final delivery gates.
