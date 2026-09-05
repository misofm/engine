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
