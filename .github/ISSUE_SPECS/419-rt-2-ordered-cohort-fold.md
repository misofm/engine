# Accumulate folded cohorts into the master in one ordered pass

Astra ready-to-number final brief, revalidated 2026-09-05 against merged main `1fa4424d732b0d9150dda5512da80cb95d76a33e` (#399). Root must number/synchronize before implementation and freeze its actual integrated source. Exact #349 finding: “Route fold accumulates into the master once per lane — 8 read-modify-write passes per cohort per plane.” Class A/high. No code, Cargo, benchmark or Git operation was performed for this brief.

## Premise and corrected traffic claim

Still present on the merged #399 main: ArenaMembers::fold_plane runs mix2x2_block for one staged lane then writes/accumulates the master, and rack calls it per lane. Existing route_fold eliminates separate route/reduction nodes but does NOT eliminate these per-lane master passes. This is not a null result.

Smallest closable outcome: retain current all-lanes-or-none graph fold eligibility and change the eligible folded epilogue to one ordered master traversal per cohort per plane. Keep the route transform per lane exactly unchanged. The first contributing cohort starts from its first lane and never reads stale master; later cohorts load the previous master once per word, add their lanes sequentially, then store once. Correct the audit prose: zero master loads applies only to the opening cohort, not every cohort. Sixty-four contributors in eight cohorts produce eight master stores and seven master loads per output word/plane, not one plan-wide store or zero loads everywhere. Do not quote theoretical saved cycles or promise a timing win.

#399 is merged and this technical dependency is satisfied. The dedicated allocation fixture is `crates/graph/tests/rt1_direct_bank_alloc.rs`, introduced by #399, not #415. #415 is qualification-only and owns no allocation fixture. Root schedules implementation only after its pending measurement quiet window; do not compete with that measurement. #306 tooling and deferred automation are not technical prerequisites. Use a fresh worktree, not the active #415 checkout.

## Exact arithmetic contract

For each sample/channel, route each lane using the existing mix2x2_block::<FrameLane> and frozen coefficients. If this cohort contains the unique FoldLane.store contributor, initialize m by COPYING that first routed value, then evaluate `m = m + t1`, `m = m + t2`, etc., in existing lane/route order. If this is a later cohort, initialize m from the existing master, then evaluate `m = m + t0`, `m = m + t1`, etc. Do NOT sum a later cohort from its own zero/first lane then add the subtotal to master: that changes D9 association. Do not seed the first contributor with 0.0, reassociate/tree-reduce, combine route matrices, fuse multiply/add, change flushes or canonicalize NaNs. First-contributor negative-zero copy is an absolute gate.

Keep the complete existing bind-time witness: input_producers equals concatenated folded route IDs in exact render order; single master, full association, no observer/sidechain/delay/alias/intervening buffer hazard or illegal target alias. Preserve each existing decline and the candid shadowed-clause notes; do not invent coverage claims for structurally unreachable cases. No new partial-fold eligibility, multiple-master support, master-op cross-cohort fusion, RT-3 reduce_plane rewrite or RT-11 tile form.

## Minimal source boundary

- crates/rack/src/lib.rs: one whole-folded-cohort callback seam with default behavior equivalent to existing per-lane fold_plane calls, and dispatch after needed lane staging exists. Retain fold_plane for compatibility and mixed/default fallback.
- crates/graph/src/runtime.rs: ArenaMembers override transforms staged lanes in place then accumulates the master once in preserved order. Current route_fold/apply_route_fold witnesses should remain logically unchanged; only minimal callback hookup is allowed.
- crates/lane/src/kernels.rs (public as lane::kernels through the existing pub mod declaration): one lane-generic ordered accumulation block primitive beside sum2_block/sum_into_block, with existing frame-vector/tail conventions and focused tests. It supports initial-store and continuation-load modes with exact scalar/W4/W8 operation order. No new module/API taxonomy or general graph reducer.
- Existing inline rack/graph tests, crates/lane/tests/g2_kernel_identity.rs and tools/console-workload/tests/chain_shape.rs; crates/graph/tests/rt1_direct_bank_alloc.rs may be minimally extended for folded cohorts while retaining its direct-scatter proof. No allocator implementation or ordinary-unit-binary allocator attachment.
- Numbered issue/evidence, one matching runner/preflight arm, and existing artifact/publisher/current-identity/generated-browser files only if actual changed source requires regeneration, following #399's immutable source-candidate convention.

No new engine unsafe borrowing API is needed: staging is rack-owned and master is arena-owned. Borrow distinct staging windows with safe slice splitting and fixed stack arrays bounded by bank width 4/8. Never use heap Vec construction on render. Preserve scratch stride versus used frame count and two-channel disjointness. New safe callback/kernel inputs must reject inconsistent lengths/widths before writes or encode the invariant privately; avoid public forged capacity metadata, unchecked slicing and new marked panic surfaces.

## Full/partial/default semantics

The optimized callback sees only active folded lanes, with their ORIGINAL physical lane IDs in stable order; compacted positions cannot replace IDs. Production graph arms every rendered lane or none, but BankChain's public seam can represent mixed masks. Keep mixed folded/unfolded behavior on the old path unless exact callback/scatter ordering is proven and explicitly covered; that generalization is not required. A partial physical bank whose active lanes are all folded may use the new callback with its actual active count, including one lane and holes in a test provider. Stage only active lanes, preserve inactive output/staging and aux semantics. Default providers must receive exactly the same fold_plane calls, once per complete lane block in original ID order. No fold callback for an empty active set. Preserve all counters and RT-1 direct behavior for unfolded banks.

## Discriminating gates

1. Baseline/candidate full workspace comparison, focused lane/rack/graph suites in debug and release, existing console-workload chain/identity fixtures, realtime gate/mutations, lane/rack/graph/workspace policies, fmt/diff and applicable clippy. Required supported wasm/AudioWorklet static/resource/browser qualification remains before merge; native AArch64 stays deferred. Never run concurrent Cargo in a shared target.
2. Ordered kernel oracle uses existing per-route mix2x2 then reduce_plane/sum2_block/sum_into_block, not the new kernel against itself. Compare to_bits at scalar/W4/W8 over sub-width/ragged/128-frame lengths, one/many contributing lanes, multiple consecutive cohorts, first-store and continuation modes, asymmetric L/R routes, signed zeros, subnormals, infinities and NaN payloads where the existing primitives define bits. Include cancellation witnesses that distinguish `((master+t0)+t1)` from `master+(t0+t1)` and reversed order. Never loosen an identity mismatch to tolerance; if existing cross-width behavior limits hostile cases, document that exact pre-existing behavior and compare matching execution arms.
3. Rack callbacks: direct/default whole-lane equivalence for full W4/W8 and partial/holey masks; mixed masks retain old callback/scatter ordering; empty/inactive lanes untouched; wrong shape refused nonpartially; retained fold/aux/mono behavior; repeated blocks and counters. A test sentinel/counter must prove the eligible override is actually selected and that no per-lane master update remains; timing alone cannot establish this.
4. Real graph fixtures compare opening and later folded cohorts with the existing independent D9 oracle; retain route_fold decline/interleaving/PDC/observation tests and named shadowed-case limitations. Check standing plumbing/gain-pan master digests and bank_route_folds counts unchanged. Add a nonzero prior-master continuation witness; an all-zero master cannot discriminate an incorrect subtotal implementation.
5. Repeated folded render zero allocations/frees via the post-#399 isolated allocator test binary, with liveness and thread-scoped audit counters. Keep direct and folded measurements in the existing ONE integration test function or otherwise serialize within that executable; do not add parallel tests that race its process-global Count/restore mode. All preparation/assertion allocations outside scope; no global test-mode contamination. Safe ownership proof and producer checks reviewed before any timing.
6. Code/source evidence shows one master traversal per plane per cohort and only one read of prior master on continuation, with unchanged per-word ordered additions. A bounded release excerpt or test callback/operation witness suffices; no second codegen/benchmark framework. Floor arithmetic remains unchanged because this removes traffic, not the lane-op algorithm.

## One frozen descriptive benchmark

Root registers one fresh `--issue419-rt2` arm in scripts/run-console-benchmark.sh and scripts/operator/preflight-console-benchmark.sh, including matching usage and fresh artifacts/issue419-rt2 namespace. Preserve #399 and all consumed namespaces. Freeze committed source/workload/fixtures/floor/validators after Astra semantic PASS; no timing during iteration. Root runs the existing non-timed operator preflight first and persists output externally to the namespace. It must verify arguments, schema, writes/failure propagation and overwrite refusal with zero workload launches.

Then exactly one invocation of the existing console runner: one warmup/two measured rounds/full frozen 46-record corpus, when competing builds/qualification load have settled. Inspect sixty_four_track_plumbing_only and sixty_four_track_gain_pan_only, and report eligibility/counters/output digests. Compare with #415 only if it has successfully delivered applicable same-workload evidence, candid about differing hosts/control; #399 itself has no timing baseline because its invocation was refused before workload; no before-run timing or retry. Validate each record and aggregate with unchanged current validators, preserve raw/stderr/disposition/hashes and actual runner binary/profile/candidate provenance. Missing cycle fields remain null. Post-workload tooling failure preserves raw output and becomes a tooling successor; no rerun or tuning. Benchmarks are descriptive, identity/safety/mechanism are acceptance.

## Delivery

Astra briefs; Luna one coherent implementation attempt; Sol only after Astra FAIL, maximum two retries then hard stop/rescope. Root owns fresh worktree, exact-path checkpoints/pushes/spec/issue synchronization and PR. Root checkpoints before layering tranches; no overlapping #399 edits. After source PASS complete immutable qualification/evidence, request Astra review of actual pushed PR, wait required CI and only then merge/close/synchronize RT-2. No plan-wide master fusion or expanded routing scope is silently included.


## Final interface and discriminating-test clarifications

The actual insertion points are public `lane::kernels::{sum2_block,sum_into_block}` (vectorization across independent frames, scalar tails), `BankMembers::fold_plane`'s default no-op, both `BankChain::scatter` and `scatter_tiled`, and the private graph `ArenaMembers::fold_plane`. The new cohort callback's DEFAULT must delegate to the existing fold_plane method in original lane order, preserving providers that override only that method. Graph overrides the cohort callback; keep the old method for compatibility/mixed fallback. Keep route_fold/apply_route_fold unchanged. A minimal shared callback/input type may live in rack; no new engine export or unsafe is needed.

The new kernel needs two explicit modes: initial-store, copying the first contributor, and continuation, loading prior master BEFORE every cohort addition. It must validate all input slice lengths and bounded contributor count (1 through 8) before ANY output write, or take a privately enforced equivalent shape. Do not rely on debug assertions for a new safe externally constructible shape. Graph validates all lane metadata and L/R/master capacities before transforming any lane, so malformed input cannot partly route a cohort then fall back and route it twice. Use safe fixed storage/slice splitting; no unbounded cohort construction, Vec or unsafe. Existing prepared scratch/lease allocation invariants stay intact.

Strong finite continuation witness: prior master = 16777216.0, next cohort lanes = [1.0, -16777216.0]. Existing D9 left association produces 0.0; a cohort subtotal then master addition produces 1.0, as does reversing these lanes. Run this through scalar/W4/W8 block widths and graph continuation with coefficients/source chosen so the independently executed existing route transform yields those values. Separately use initial contributors [16777216.0, 1.0, -16777216.0], first routed negative zero with poisoned old master, one-contributor initial/continuation cases, and opposite/asymmetric L/R values. A negative-zero kernel fixture must assert the oracle actually contains negative zero; route coefficients may otherwise erase the intended sign before accumulation.

The traffic proof is source/mechanism evidence: transform every contributor once, then hold each output vector/sample accumulator across the ordered contributor loop and store once. Tests must distinguish the graph override from merely invoking the default per-lane implementation. Preserve full W4/W8, active partial/holey physical IDs, mixed-fold legacy ordering and empty/inactive sentinels. The production fold remains all rendered lanes or none; the new callback must not create partial graph eligibility.

This remains one bounded Class A product outcome. No second benchmark framework, extended target matrix, plan-wide accumulation fusion, storage-removal capability contract or unrelated gate repair belongs here. The existing per-source immutable artifact and publisher qualification is required because Rust source changes, using the same established flow as #399. Current benchmark control ceiling remains 0.50 with 60-second binary cooldown; never alter these or authorize an uncontrolled override. Exactly one invocation is consumed even on prelaunch refusal; preserve its disposition and require explicit later qualification ruling rather than an automatic retry.

## Numbered assignment

Issue #419 implements finding RT-2 in #349. Astra supplied the frozen brief above. The issue and local spec are synchronized before implementation. Base is `1fa4424d732b0d9150dda5512da80cb95d76a33e`; #399 is merged. The #415 measurement window has completed successfully, so its quiet-window dependency is satisfied. Luna owns attempt 1; root owns Git checkpoints and delivery. Root handles benchmark registration and all timing only after source review PASS.

## Luna attempt 1 evidence

Implemented the bounded ordered cohort path in `crates/lane/src/kernels.rs`, `crates/rack/src/lib.rs`, and `crates/graph/src/runtime.rs`. The callback validates IDs, widths, strides, capacities, and store mode before routing; the graph override routes each physical lane once, holds each frame-vector accumulator across the ordered cohort, and stores once, while the default callback preserves original per-lane `fold_plane` behavior. Added scalar/W4/W8 D9, continuation, negative-zero, and pre-write shape rejection witnesses in `crates/lane/tests/g2_kernel_identity.rs`.

Focused debug and release lane/rack/graph suites, graph allocation fixture, and console workload chain-shape fixtures passed. Lane, rack, graph, and realtime policy checks passed; focused clippy passed with only the repository's existing invalid-path configuration warnings. No benchmark or runner invocation was made. Root owns the exact-path checkpoint and subsequent Astra review.

## Astra attempt 1 verdict — FAIL; Sol attempt 2

# Astra #419 RT-2 attempt 1 review

**FAIL at exact pushed `ea240a377bfa63420e953f1f7b2fb06fa53ee3de`. Luna's one attempt is consumed; assign the bounded correction/evidence pass to Sol.** No timing, runner, artifact or full-workspace promotion is authorized from this source checkpoint.

## Concrete contract regression

Both `BankChain::scatter` and `scatter_tiled` now gather folded lane IDs while immediately scattering every unfolded lane, then invoke all folds afterward. For a mixed mask folding lanes0/2 with lane1 unfolded, the old observable callback sequence is `fold_plane(0), plane_mut(1), fold_plane(2)`; the new default path is `plane_mut(1), fold_plane(0), fold_plane(2)`. This violates the explicitly frozen mixed-mask callback/scatter order, even though a provider that only sums its passed slices can still produce equal PCM. A stateful safe provider may observe or use the intervening scatter. Current old rack tests check results, not this ordering.

Restore the old per-lane path when any active lane is unfolded, in both full/tiled and partial paths. Use the cohort callback only when every active lane is folded. Full unfolded RT-1 direct scatter stays intact. No new graph partial-fold eligibility or ordering generalization is approved.

## Evidence absent from the coherent pass

The actual cumulative diff adds no rack tests, graph tests, console chain-shape tests or folded allocation fixture. Only three lane tests were added. They establish useful finite cancellation/continuation/negative-zero examples, but all use lengths equal to nine times the vector width, so the new scalar-tail/sub-width arms are not exercised. They compare simple expected values rather than the required old primitive-sequence oracle. The rejection test covers only a two-input length mismatch at scalar width.

Consequently the existing green graph/rack counts do not prove default-versus-override equivalence, full/partial/holey physical IDs, mixed ordering, cohort shape rejection before route mutation, real graph opening versus continuation, actual optimized graph callback selection, or repeated folded render zero allocations. The existing isolated `rt1_direct_bank_alloc.rs` graph has no prepared routes and proves the direct path only; rerunning it cannot qualify the new folded callback.

## Useful source to preserve

The new lane kernel has the correct high-level initial-copy/continuation-load ordered accumulation and checks contributor count/lengths before writing. The graph override transforms routes separately, retains physical IDs and first-store semantics, checks metadata/capacities before its route loop, and keeps fixed stack arrays. No new unsafe or arena ownership API is introduced. The private FoldCohort construction constrains representable requests; source review found no new aliasing defect. These findings do not replace the missing integration/rejection evidence. The default callback's silent invalid-shape return must not be described as a proven complete fallback; the numbered contract requires either impossible invalid shapes or explicit nonpartial rejection before any PCM mutation.

## Bounded Sol attempt 2

Keep the same approved source paths/API boundary and implement the mixed-mask correction first. Complete the frozen representative evidence in the existing files, not a new corpus/framework:

- Rack: a trace or equivalent observer differentiates the exact old mixed sequence above for tiled and partial paths; full/holey all-active-folded masks select the cohort override, pass original physical IDs, preserve inactive outputs/staging and default providers' fold_plane order. Empty active/fold sets produce no callback. Compare default old per-lane behavior and optimized provider PCM/counters on repeated blocks. Test representable malformed shape rejection before writes; if construction makes a shape impossible, explicitly test/document that boundary instead of inventing unsafe metadata.
- Lane: use existing sum2/sum_into primitives as an independent same-width D9 oracle, covering Scalar/Simd4/Simd8, 1/subwidth/exact/ragged/128 lengths, counts1..8 representative endpoints and invalid0/>8/late-short shapes with poisoned output. Preserve asymmetric/signed-zero/hostile cases under existing primitive semantics. Include the nonzero prior-master continuation witness and prove the wrong subtotal/reversed result differs. No tolerance relaxation or new FP rule.
- Graph: a real folded opening and later cohort must execute the new override, match the independent existing route-transform-plus-D9 oracle on both planes and preserve counters/declines. Exercise the explicit continuation witness through routing, not only directly in the lane kernel. Prove malformed lane/store/length metadata is either unrepresentable or rejected before any route or master mutation; no partially routed then retried fallback. Retain old fold_plane as the compatibility path.
- Extend the existing isolated graph allocation test's ONE serialized function with an actually folded prepared graph and repeated renders, preserving its direct proof, positive allocator liveness and thread-local zero allocation/free accounting. No process-global test-mode races, allocator framework or ordinary-unit allocator attachment. Show fold/callback mechanism evidence, not just a successful render of a nonfolding graph.

Use source/mechanism evidence to show one master load on continuation and one store per output vector/sample, retaining route transforms and addition order. Keep bind eligibility witnesses unchanged. Do not touch queued #420 general reduction or benchmark rows/validators/floors. Preparation/shape checks remain bounded by W<=8 and allocate no render memory.

After one coherent pass, focused lane/rack/graph debug/release, isolated direct+fold allocation proof, existing console chain/identity and applicable realtime/lane/rack/graph policies, fmt/diff/clippy. Root checkpoints/pushes before further edits and Astra gives one attempt-2 verdict. At most one subsequent Sol attempt remains if that fails; attempt3 failure is a hard stop/rescope. Timing/target/artifact/full-workspace qualification and actual PR review follow only semantic PASS.

This verdict used read-only source/spec/diff inspection and prior completed evidence; no Cargo, benchmark, repository or GitHub mutation.

## Sol attempt 2 evidence — pending Astra verdict

Corrected both rack scatter paths so the cohort callback runs only when every active lane is folded. Any mixed mask retains the established immediate per-lane fold/scatter callback order; full unfolded direct scatter remains unchanged. Rack witnesses distinguish that order on tiled and partial paths and prove full and holey all-active-folded masks select one cohort callback with original physical IDs over repeated blocks while inactive planes remain untouched.

The ordered lane kernel is now checked against the independent existing `sum_into_block` D9 sequence at scalar/W4/W8 for one, sub-width, exact-width, ragged and 128-word blocks, representative contributor counts through eight, opening and continuation modes, hostile values, signed zero and the finite continuation discriminator. Zero, over-eight and late-short contributor shapes preserve poisoned output. Graph cohort tests now use existing route and reduction kernels as the oracle across an opening and later cohort, include the live-master cancellation witness on both planes, and show duplicate IDs, a later store and an unknown lane leave routed staging and master unchanged.

The existing isolated allocator test's single serialized test function retains allocator liveness and the direct graph proof, then prepares a real four-route folded graph outside the measured scope and renders it repeatedly with zero allocations/frees and unchanged PCM/counters. Focused lane/rack/graph debug and release suites, release console chain-shape, realtime mutation tests, lane/rack/graph/workspace policies, focused clippy, formatting and diff hygiene pass. No benchmark runner, timing, artifact, target qualification, full workspace, Git or GitHub operation was performed by Sol.

## Astra attempt 2 verdict — FAIL; final Sol attempt 3

# Astra #419 RT-2 attempt 2 review

**FAIL at exact pushed `f716b501eec2382ebbfb77def2053b22f99856d7`.** Preserve this coherent checkpoint. Sol has one final coherent revision, attempt 3; no timing, runner, artifact or full-workspace promotion is authorized yet.

## Accepted corrections

Both rack scatter paths now use the cohort seam only when every active lane is folded. Mixed masks immediately interleave fold_plane and plane_mut in original physical order. The changed trace test discriminates the original regression in both tiled and partial paths. The ordered kernel remains a single accumulator/store per vector/sample, with continuation loading the master before ordered additions. The new same-width old-sum_into oracle covers scalar/W4/W8, sub-width/exact/ragged/128 lengths, representative counts through eight, hostile values, initial negative zero, nonzero-master cancellation and poisoned-output shape rejection. Graph's direct cohort tests exercise opening/continuation using existing route/reduction primitives; its independent finite routing witness rejects subtotal association. The original binding eligibility logic remains intact.

The isolated allocation fixture retains one test function, mode restoration, positive thread-scoped allocator liveness and the direct graph proof, and now prepares explicit routes outside the measured scope. Focused debug/release and policy logs are green. Those are useful results, but the following frozen obligations remain unresolved.

## 1. Public shape boundary is unchecked

`FoldCohort::new` is public and accepts arbitrary IDs, stride, frame count and slice capacities. Thus malformed shapes are representable; the earlier private-construction rationale is inapplicable. Its public `planes_mut` computes `index * stride` and `start + frames` without checked arithmetic before returning an optional slice. A safe caller can construct overflow metadata: debug arithmetic can panic; release arithmetic can wrap into an unrelated in-bounds window or a later slicing panic. This is a contract/realtime robustness defect, not a claim of memory unsafety. No malformed Rust reproduction was executed.

The graph consumer checks capacity arithmetic before its normal routing loop, so this finding does not claim that the prepared graph currently routes forged IDs. Nevertheless the newly public API itself must meet the frozen inconsistent-shape contract. Also graph performs max/duplicate traversal before rejecting count >8; enforce the bounded count before traversing externally representable metadata.

Use the smallest checked shape boundary: preferably a fallible constructor that validates bounded nonempty count, unique IDs, stride/frame relation, checked offset/end arithmetic and both complete capacities, retaining private fields. Alternatively keep public construction only with a complete checked accessor and explicit consumer validation contract; no unchecked wrap/panic or partially valid request may be presented as a valid view. Do not introduce unsafe, allocation or a generic borrowed-arena API. Define the zero-frame case explicitly. Production chain construction failures must be impossible by demonstrated staging invariants or handled before callbacks/writes, without expect/unwrap in marked render code.

## 2. Required default and shape fixtures are still absent

The only rack provider exercising folded cohorts, PlanesWithFold, now overrides fold_cohort. Consequently the added full/holey test does not execute the trait DEFAULT at all. It checks cohort IDs and fold traces but does not compare default versus override PCM/counters. There is no direct default malformed-shape fixture. Its full case is W4 and holey case W8; full W8, a one-active-lane cohort and empty-active callback behavior remain undiscriminated. Only left inactive planes are asserted, and staging sentinels are not checked.

Add a minimal wrapper/provider overriding only fold_plane, paired with the existing override provider. Exercise full W4/W8, one representative holey/single partial case, mixed ordering and empty/inactive behavior; compare both planes, exact callback order/count and repeated-block PCM/counters. Assert inactive staging/output and unused used-frame/stride tail sentinels where the chosen shape exposes them. No new fixture corpus is necessary.

The graph malformed fixture covers duplicate IDs, a nonfirst store and an unknown lane only. It does not cover representable short L/R capacity, stride < frames, frame count beyond lease, count >8/empty, or overflowing offsets. Cover these at the checked constructor/accessor when invalid shapes become unrepresentable; otherwise cover the actual consumer rejection before any staging route or master mutation. Keep late bad-metadata cases that would expose routing one valid lane before rejecting a later lane. Test the new boundary in debug and release with untouched poison on failure. The default callback must also be exercised at its actual validation/delegation boundary.

## 3. Prepared-graph allocation proof lacks a fold mechanism assertion

The isolated folded fixture asserts only PCM and `qualification_counters() == [16,16]`. Those values come from IdentityBank's process counter and are identical for its direct and folded configurations. They do not establish that bind admitted route folding or that the new override was selected; a nonfolding equivalent graph can pass them and the zero-allocation check.

Before the measured scope, assert existing `bank_route_folds()` is exactly four for this four-route fixture and zero for its direct counterpart. Retain these assertions after rendering if useful. Combine this actual prepared eligibility assertion with a bounded callback/operation witness which fails if the eligible graph falls back to per-lane fold_plane. The existing rack cohort trace plus an actual graph override-specific test/counter or narrowly scoped test-only rejection of the compatibility path is sufficient; do not add production telemetry or a new framework. A direct unit invocation of ArenaMembers::fold_cohort proves its arithmetic, but alone cannot prove prepared execution selects it.

The existing graph first-contributor signed-zero test still invokes only fold_plane. Extend it to the actual cohort override with poisoned old master and verify the routed oracle really retains negative zero. Preserve compatibility coverage. Existing console decline/digest/fold-count tests should remain green; correct their now-stale comment that the epilogue is per lane while retaining the route-count metric (one folded route per track, cohort accumulation traversal).

## Final Sol attempt 3 scope

This is one bounded API-and-directed-evidence completion in the already allowed rack/lane/graph test paths, with the existing console comment if needed. Preserve corrected mixed ordering, ordered arithmetic and binding witnesses. Do not touch RT-3, floors, benchmark workloads/validators, target/artifact tooling or unrelated source. Finish the validation boundary and these missing default/shape/mechanism tests in one coherent pass; run the frozen focused lane/rack/graph debug/release suites, isolated allocation proof, existing console chain identity/shape and applicable policies/fmt/clippy. Root checkpoints and pushes, then Astra supplies one final verdict. Attempt-3 FAIL is a hard stop/rescope, not permission for a fourth repair.

Review used source/spec and completed-log inspection only; no Cargo, malformed-code execution, timing, repository or GitHub mutation. No performance claim or broad qualification acceptance is made.

## Sol attempt 3 evidence — pending final Astra verdict

The public cohort boundary is now fallible and validates nonempty bounded lane count before traversal, unique physical IDs, nonzero frames, stride, checked offsets/end positions and both slice capacities. Private fields remain inaccessible and `planes_mut` also uses checked arithmetic. Production dispatch handles the construction result without panic. Directed rack tests cover empty, over-eight, duplicate, zero-frame, stride-short, left/right-short and overflowing-offset requests with poisoned storage unchanged, plus a valid strided request whose unused tails remain untouched.

A provider overriding only `fold_plane` is paired with the cohort override for repeated full W4, full W8, holey W8 and single-active W8 blocks. Both planes, physical callback order/count, PCM, inactive outputs and callback selection are compared; an empty active chain is explicitly unrepresentable. Graph tests retain compatibility `fold_plane`, run signed zero through the actual cohort override with a poisoned master, reject later-store/unknown-lane/frame-beyond-lease metadata before route/master mutation, and retain the opening/continuation independent oracle. The serialized allocation test asserts existing prepared-plan `bank_route_folds()` is zero for the direct fixture and four for the routed folded fixture before measuring repeated zero-allocation/free renders. The standing console comment now describes per-track retired routes and cohort accumulation accurately.

Focused debug and release lane/rack/graph suites, the isolated allocation test, release console chain shape, realtime mutations, lane/rack/graph/workspace policies, formatting, diff hygiene and focused clippy are the final local gates. No timing, runner, artifact, full-workspace, Git or GitHub operation belongs to this attempt. Attempt 3 is the hard stop and requires Astra's final verdict.

## Final attempt verdict — FAIL; hard stop and numbered completion

# Astra #419 final attempt 3 review

**FAIL at exact pushed `aafa59a17c4292f8f1837eac557e06e47305f1a4`. This is the hard stop: preserve the checkpoint and rescope/rebrief before further implementation. No fourth repair, timing, artifact or workspace promotion is authorized from this verdict.**

## Concrete remaining contract defect

The fallible public constructor and its two production consumers disagree on which slice capacities are valid.

`FoldCohort::new` validates `max(lane_id * stride + frames)`. It therefore successfully constructs a cohort with IDs `[0]`, stride 4, frames 2, and left/right slices of length 2. Its checked `planes_mut(0)` correctly returns those complete two-frame planes. However both the default `BankMembers::fold_cohort` and graph `ArenaMembers::fold_cohort` retain the older requirement `(max_lane + 1) * stride`, which is 4 for this same request. They silently return without any fold callback or accumulation.

The same discrepancy occurs with IDs `[0,1]`, stride 4, frames 2 and six-element planes. The override test provider consumes such a constructor-approved cohort via planes_mut; the trait default silently drops it. Thus the promised default equivalence is not established for the public accepted domain. This is a deterministic API/semantic defect, not arithmetic reassociation or an unsafe-memory claim. The production chain currently allocates fully padded staging, so this finding does not claim that ordinary prepared renders currently lose samples.

The new valid-stride fixture has eight-element planes and consequently covers padding preservation but not the exact accepted-capacity boundary. It misses this disagreement. There must be one explicit shape contract shared by construction, default delegation and graph validation: either require complete final stride padding at construction or accept only the actually accessed frame endpoints everywhere. A success value cannot mean “valid” for the accessor and “silently discarded” for default delegation. Choosing and proving that boundary belongs to the post-stop amended scope, not an unreviewed patch under attempt 3.

## Improvements accepted and preserved

The constructor rejects empty/over-eight/duplicate IDs, zero frames, stride-short requests, left/right short slices and overflowing offsets before returning a value. Its count bound precedes traversal, private fields prevent forging and planes_mut now uses checked arithmetic. Those address the earlier overflow/panic finding. Prepared dispatch handles constructor results without marked expect/unwrap. No new unsafe or unbounded fan-in change appears.

The paired default-only and override providers now cover repeated full W4/W8, holey W8 and single-active W8 calls, both planes and physical IDs. Empty active chains are explicitly rejected at construction. Mixed callback order remains corrected. New strided sentinels cover unused tails. Graph shape tests cover late-store/unknown metadata and frames beyond the lease without route/master mutation. The new actual cohort signed-zero test poisons the old master and confirms the routed input and output preserve negative zero; compatibility coverage remains.

The allocation integration remains one serialized function with positive thread-scoped liveness and mode restoration. It now asserts the direct plan has zero folded routes and the routed plan has four before measured renders, retaining four afterward, with zero allocations/frees. The independent D9 lane and routed opening/continuation oracles remain intact. Root corrected the handoff: its test-only compatibility-path rejection phrase was an inference, not evidence. No separate rejecting mechanism is present in the committed source/spec or named logs. Preserve the prepared fold count, but complete the explicitly assigned override-specific rejection witness in the numbered successor; do not inherit the unsupported claim.

Completed logs show focused lane/rack/graph debug and release green (graph includes its isolated allocation test), console chain release 21 passing, realtime 42 regions/12 files and mutations, lane/rack/graph/workspace policies, and clippy completion with the recorded warnings. These checks did not include the accepted-domain counterexample above. No blanket qualification PASS follows from green existing cases.

## Hard-stop disposition

Preserve all three attempt checkpoints and candid evidence. Root should record the remaining accepted-shape disagreement, then amend/rebrief one bounded completion outcome before restarting the workflow. Keep ordered accumulation, corrected mixed dispatch and accepted representative tests; do not expand into RT-3, change D9 or introduce new tooling. The next bounded scope must freeze one capacity formula and test its exact boundary through constructor/accessor/default/graph consumers, plus record the outstanding mechanism evidence accurately. Broader qualification and the single frozen benchmark remain after semantic acceptance. This report is not permission for a fourth #419 correction under the current attempt budget.

Review used source/spec and completed-log inspection only. The arithmetic counterexample was established directly from the checked formulas; no malformed Rust reproduction, Cargo, timing, repository/Git or GitHub mutation was executed.

Ready-to-number bounded successor brief: `/tmp/astra-419-cohort-completion-brief.md`. Parent #419 keeps every original RT-2 product and qualification obligation until the completion and final delivery are accepted.

## Rescoped dependency #422

The three source attempts are exhausted and preserved. #422 now owns the remaining validated public-cohort capacity agreement and actual graph dispatch mechanism proof under Astra's separately frozen completion brief. No fourth correction is authorized under this issue's exhausted attempt sequence. This parent remains OPEN and retains every RT-2 arithmetic, binding, realtime, identity, allocation, workspace, supported-target, immutable-artifact/browser, one-invocation descriptive measurement and final PR/CI obligation. One integrated final PR may close #422 and #419 only after both complete contracts pass; there is no standalone runtime merge bypass.

## Source completion accepted through #422

Astra PASS at `1958120d829d8a2f6144ed9020ee8ebbc6f07d49` resolves the preserved source blockers via separately briefed #422. Its full verdict is recorded there. Root now registers only the existing runner/preflight namespace `--issue419-rt2` and freezes this checkpoint for retained qualification. No workload, validator, readiness limit or invocation count changes. Workspace, supported targets, immutable artifact/browser, one controlled descriptive invocation and actual PR/CI remain pending. Neither issue is closed.

## Immutable artifact qualification checkpoint

Frozen source candidate: `0a0e39e42e4ae2585d5f5ee507a4cb9aaf7b741a`. Independent REPIN and normal verification builds produced SHA-256 `518b5aa864c0a825cd324112b24270a7e0714fc63db6bd1029779f21066ea9de`; normal output is `/tmp/engine-419-worklet-verify.5OwTT4`. Static/object checks, expected-resource comparator with 26 rejecting mutations, hermetic worklet mutations and recorded Chromium/Firefox/WebKit qualification passed. Current pin, publisher equality consumer and generated browser records follow this source; historical artifact identities remain intact. Logs `/tmp/engine-419-{worklet-repin,worklet-verify,check-web,expected-resources,hermetic-worklet-mutations,browser-all-record}.log`. Individual matrix verification legs are underway.

Supported scalar 18-package closure and SIMD smoke/protocol checks passed, with explicit target cfg and scalar/SIMD protocol export/execution proofs. Sol retained a separately checked non-LTO scalar inspection of all three objects in engine/source/target_smoke archive families, every decoder successful and every atomic scan clean; this is that named scope only, not a fat-LTO inspection claim (#404 remains). Logs `/tmp/sol-419-supported-wasm-summary.log`, `/tmp/sol-419-nonlto-inspection.log`; procedure `/tmp/sol-419-nonlto-inspect.sh`.

The non-timed existing runner preflight passed with zero workload launches (`/tmp/engine-419-benchmark-preflight.log`). No descriptive benchmark invocation has occurred. Initial fullworkspace use of a shared target failed before tests on stale lane metadata; its log is retained (`/tmp/engine-419-candidate-workspace.log`). A fresh isolated target fullworkspace is running; no source repair was needed.

## Completed workspace and qualification review

Fresh isolated `cargo test --locked --workspace` terminated exit 0: 274 successful result blocks, 1,566 passed, zero failed, 24 ignored. Accepted baseline (`/tmp/engine-417-candidate-workspace.log`, terminal exit 0) has 274 blocks, 1,559 passed, zero failed, 24 ignored. The seven added tests are exactly graph lib 46→49, rack lib 24→27 and lane G2 5→6. Candidate log `/tmp/engine-419-candidate-workspace-fresh.log` reaches its final wasm_gates doctest. Initial shared-target pretest failure is preserved separately; it is not counted as passing evidence. All three individual browser check-matrix legs also terminated exit 0. Publisher digest equality is verified.

Astra reviewed the artifact/target evidence and found the original Sol archive enumeration recorded mapfile status rather than find/sort status. The original log and script are preserved. Root independently captured and checked find, sort, ar listing and ar member reads against the same existing target, proving exactly one archive per family and byte/hash identity of the complete three-object population with the already decoded/scanned objects. `/tmp/root-419-confirm-object-population.py` and `.log` record this supplement; no rebuild or source change occurred. Astra accepted the corrected named non-LTO scope in `/tmp/astra-419-qualification-evidence-review.md`.

The exact runner profile (opt-level 3, LTO false, codegen-units 16) build terminated exit 0 in `/tmp/engine-419-runner-profile-build.log`. The benchmark candidate will be the later committed evidence head; runtime/workload/validator source remains identical to immutable artifact/preflight candidate `0a0e39e42e4ae2585d5f5ee507a4cb9aaf7b741a`. Timing remains uninvoked pending quiet readiness.

## Single controlled measurement completed

The sole `--issue419-rt2` invocation terminated exit 0 at candidate `9cd6ba25c7a3b7f80788cd04789a10d36ee10e92`: controlled CPU63, one warmup, two measured rounds, 46 accepted records. Aggregate and all 46 individual record validators pass. Raw/accepted SHA-256 `f2ed6356ebda8e936c41a2af74a6e6e2de2cd6109094889557f9c598b34b8299` (79,187 bytes); binary SHA-256 `e0d9e2752e50df486c4622e1b7d80de46ac59d5cd698e4da81cdfe45d462227a`. Readiness load0.32, runner load0.27, binaryage236s, sibling0.00%, unchanged limits and no override. Evidence is in `artifacts/issue419-rt2/`; matching prior-capture comparison and final PR review remain. No further timing is authorized.

## Final capture comparison and integration

Root and Luna compared all 46 unique keys with delivered #415: every emitted output digest, transpose counter, render-error and forbidden-operation field agrees in presence and value, along with named stable fixture/layout/target fields. The 42 emitted error/forbidden rows are zero; four hoist rows omit them. `artifacts/issue419-rt2/compare-rt1.py` retains exact checks. No causal speedup or absent cycle/fold telemetry is claimed. Main #424 was integrated afterward without runtime/workload/validator changes; artifact and measured source identities remain their original frozen commits. All local delivery gates are complete; actual PR Astra review and required CI remain.
