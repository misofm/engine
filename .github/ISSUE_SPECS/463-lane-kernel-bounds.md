# Prove block-slice bounds once in three lane kernels

Owns audit #349 LANE-2 in full. QUEUED behind active #459/#462 and already queued #460; this scope does not authorize overlapping implementation or timing. Planning base `29a8c88b82de8660a5d573e75b7e67d977496576`; root must freeze the later actual implementation base and obtain numbered Astra approval before Luna attempt1. User model workflow takes precedence: Astra briefs/reviews, Luna first attempt, Sol fallback after FAIL.

The following three adopted documents form one stateless contract. The FINAL gate amendment takes precedence over the earlier optional recommendation for scalar-Wasm disassembly only: actual matrix identity execution on scalar-Wasm and simd128 is required, with the minimal existing-corpus addition specified below. All original three-kernel arithmetic and correctness obligations remain.

# LANE-2: prove block-slice bounds once in three lane kernels

Ready-to-number bounded draft, not implementation authorization. Freeze the numbered issue on delivered main60519995 (or root-reviewed later main). The root checkout currently points to older e7e1a37f; this review used Git60519995 for current graph call sites and confirmed the three lane bodies and named lane/corpus/probe tools are unchanged between those revisions. Queue behind active #459/#430, #453 tooling and already-queued #460; do not overlap their files or displace their ordering. Root must synchronize number/title/body and actual implementation base before Luna1. Astra scopes/reviews, Luna1 implements, Sol2/3 follows FAIL, then hard stop/rebrief.

## Smallest outcome and accounting

Close the entire original LANE-2 finding for `sum2_block`, `sum_into_block`, and `mix2x2_block`: either demonstrate that current supported release lowering already eliminates the alleged repeated bounds checks, or implement safe one-time prefix validation plus exact-width traversal that removes avoidable repeated slice-shape checks while preserving arithmetic. An honest null is a measured code-generation conclusion about these exact bodies/targets, not a source inspection assumption or universal architecture claim. Do not silently close the three-kernel finding after inspecting only one.

No reduction algorithm, reassociation, FMA policy, graph scheduling, route fusion, rack integration, Lane implementation, public signature or error-type change. `reduce_many` and `ordered_accumulate_block` are excluded. No native AArch64 revival, LANE-1/DYN-1 integration or legacy source. Parent #349 stays open for all other findings. Do not inherit historical Apple instruction counts or invent a projected native/Wasm speedup.

## Frozen existing semantics

The controlling length is out.len for sum2, acc.len for sum_into, and left.len for mix2x2. Keep debug equality assertions for every other slice: unequal lengths, including longer input, remain rejected in debug. Release accepts a longer other slice and processes exactly the controlling-length prefix, leaving suffixes untouched (including right's suffix for mix2x2). A shorter other slice must reject rather than truncate silently via zip; validate all required prefixes before entering the write loop. This deliberate early rejection is the bounds-proof implementation choice; no compatibility claim is made for the old partially modified buffers after panic. Zero-length controlling slices preserve those same debug/release rules. No unsafe indexing or alias assumption.

After preserving debug checks, take checked prefixes once and split vector prefix/scalar remainder. Traverse exact-width chunks of equal proven span; zip is acceptable only AFTER the checks, never as the validator. Use unchanged Lane load/store operations and scalar remainder; no padding that changes processing of real samples.

Preserve exact operand order: sum2 is a.add(b), accumulation is old_acc.add(x). Matrix snapshots BOTH old planes before either store; left is `lr.fma(old_r, ll.mul(old_l))`, right is `rr.fma(old_r, rl.mul(old_l))`. Current Lane::fma means separately rounded multiplication then addition on both scalar/wide implementations. Do not rewrite its operand order as a visually equivalent equation, use hardware fused arithmetic, horizontally reduce, sanitize new values, or add denormal processing. Maintain the existing canonical FP environment policy.

## Pre-edit code-generation checkpoint: smallest honest-null test

Before kernel edits, compile ONE tiny retained non-inlined wrapper source instantiating all three functions with runtime-provided slice pointers/lengths and coefficients, using the established #388 probe style and existing compiler/disassembler tools. Opaque arguments must prevent constant lengths, identity coefficients, or dead outputs from erasing the question. No new Cargo package or generic report framework; a disposable example/probe source retained as evidence is sufficient. Freeze exact rustc/LLVM, source SHA, target flags, profile/LTO/codegen choices, command statuses and output hashes. Existing `run-native-vectorization-report.sh` is general supporting evidence, not proof of these three symbols by itself.

Inspect actual x86-v3 Simd8 bodies and relevant scalar/tail paths; inspect supported Wasm simd128 Simd4 and scalar instantiations with the existing wasm tooling. Require named nonempty decoded symbols and successful compiler/decoder statuses. LLVM bitcode is not decoded Wasm; use an explicitly identified inspectable non-LTO probe where necessary and do not mislabel it the shipped worklet. Inspect entry length tests separately from loop backedges, inner length-dependent branches and panic paths. A panic symbol anywhere in a module is neither proof of an inner-loop check nor a failure of the desired optimization. Retain the actual control-flow interpretation per kernel/target.

If all named supported instances already have only necessary entry/tail checks with no avoidable inner-loop bounds checks, checkpoint the concrete null and request Astra source/evidence acceptance before deciding on any source rewrite. Do not manufacture a cosmetic change or broaden the search to find a win. If only some instances are null, preserve those results and keep the full three-kernel contract; implement the same bounded safe traversal where warranted and compare identical probes at frozen before/after sources. Any residual Lane load/store length proof must be reported honestly, not counted as removed because indexing syntax changed.

## Finite correctness and reachability proof

Extend existing `crates/lane/tests/g2_kernel_identity.rs` (and existing support module only if needed) rather than making a new corpus. Test W1/W4/W8, lengths0,1,W-1 where distinct,W,W+1,2W+1, with full vector and scalar tails. Compare PCM words against a retained old indexed body or explicit original scalar operand-order oracle independent of rewritten traversal. Cover finite asymmetric/cancellation-sensitive values, signed zeros, normal/subnormal values under canonical FP settings, infinities and a separately identified NaN case; require nonvacuous expected categories before comparisons. Do not let every hostile input collapse to NaN and pretend finite/order/zero behavior was tested.

For each operand position, test a short slice in release and debug; test longer input debug rejection and release prefix behavior with suffix sentinels. Equal-length legal cases must still succeed. For two-input sum, either input can be short; for matrix preserve the right excess suffix. A catch_unwind fixture is allowed outside realtime; no new production Result or recoverable render error.

Run existing G2 and P1 debug/release and the existing graph route/fallback identity cases. At60519995, NodeKind::Route calls mix2x2_block; folded cohorts also call it, and ArenaMembers::fold_plane calls mix2x2_block plus sum_into_block through real rack fallback. Ordinary graph reduction now uses reduce_many, and optimized master accumulation uses ordered_accumulate_block. Sum2 remains public/evidence-oracle usage (`graph::reduce_left_to_right`), not today's general render reducer. Preserve these distinctions in results and any narrowly corrected stale caller prose.

Supported cross-target identity uses existing `scripts/run-wasm-gates.sh`/wasm-gate-corpus and native lane G2. Note the shared Wasm lane corpus directly enumerates Sum2/SumInto but does NOT directly enumerate Mix2x2; do not claim otherwise. Use the existing supported Wasm console/graph route identity execution to cover the real matrix caller and its frozen digest, alongside the explicit matrix probe/disassembly and native direct oracle. No new full corpus or repinning of existing expected outputs is authorized. If the existing real route identity path cannot demonstrate execution, report that exact evidence seam before expanding the corpus.

Suggested focused commands: `cargo test --locked -p lane --test g2_kernel_identity --test p1_partition` and matching --release; focused existing graph/route tests with named filters selected and recorded from the frozen base; existing lane/realtime policies, fmt and diff hygiene. Coordinate unique targets and serialize mutable instrumentation. No zero-result test invocation counts as proof. These stateless kernels introduce no allocations; existing actual render allocation/free gates remain mandatory delivery evidence, with no new allocator harness.

## Allowed paths, qualification and closure

Product source only `crates/lane/src/kernels.rs`; existing lane G2/support tests; existing graph tests/caller prose only where required to keep the actual reachability claim accurate; numbered spec and narrowly scoped probe/evidence. Existing tools are invoked, not redesigned. No generic gate/helper/workflow repair belongs here. Root must freeze the exact selected existing graph/Wasm commands with the implementation base before assignment; if tool defects arise, preserve failures and scope tooling separately.

Separate descriptive measurement/artifact promotion from this small product outcome before implementation when required: a subsequent measurement may use the existing frozen console runner (one warmup/two rounds, exactly one invocation after preflight, no tuning/retry), but its route-heavy numbers cannot be attributed to sum2 or these kernels alone. No benchmark framework or timing authority is created by this brief. Any mandatory supported-target/correctness proof stays binding; broader artifact/browser packaging follows root's existing delivery discipline and actual-byte changes, never a fabricated new execution claim. A null-only evidence delivery requires no changed-artifact qualification.

Acceptance requires all three bodies accounted for, finite semantic gates, truthful supported before/after or null codegen evidence, exact-head Astra review, required CI and remote issue synchronization. Number a narrowly bounded qualification successor before implementation if delivery work exceeds this product slice; keep original LANE-2 obligations explicitly retained until satisfied, not silently waived. No builds, tests, timing, implementation or repository/Git/GitHub mutations were performed while drafting this brief.


# LANE-2 exact gate resolution at delivered29a8c88b

Read-only supplement to `/tmp/astra-349-lane2-numbering-brief.md`. No tests/builds/timing run. These are source-verified existing test names and execution paths, not fresh passing evidence. Queue remains behind460 and active459/462; no implementation authorization.

## Existing native commands and exact scope

Run from the frozen repository with an isolated root-assigned CARGO_TARGET_DIR. For each command below also run the same command with --release before `--`; --exact prevents an accidental neighboring test match. Require the named test to execute once, not merely exit0 with zero tests.

```
cargo test --locked -p graph --lib runtime::tests::route_applies_folded_gain_with_frozen_op_order -- --exact
cargo test --locked -p graph --lib tests::executor_applies_exact_pdc_then_fixed_pairwise_reduction -- --exact
cargo test --locked -p graph --lib runtime::tests::a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit -- --exact
cargo test --locked -p graph --lib runtime::tests::the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign -- --exact
cargo test --locked -p lane --test g2_kernel_identity --test p1_partition
```

`route_applies_folded_gain_with_frozen_op_order` (runtime.rs4171) directly invokes mix2x2_block<FrameLane> with gain-folded asymmetric coefficients,4096 seeded samples and lengths1/3/7/63/65/129/511, comparing bits to the independent unfused-f64 oracle. It is a direct kernel test, NOT a bound Route-op execution.

`executor_applies_exact_pdc_then_fixed_pairwise_reduction` (graph/lib.rs3495) builds actual PreparedRoute entries with identity transforms and bound source/output processors, invokes the prepared render plan, and verifies delayed/reduced PCM. Routes are not externally bound and the fixture contains no bank cohorts; they execute NodeKind::Route at runtime.rs1189, which calls mix2x2_block. This supplies actual nonfolded route reachability, with identity coefficients; the preceding direct test supplies the asymmetric arithmetic discriminator.

`a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit` (runtime.rs3500) compares staged cohort epilogues with independent old route/reduction plumbing at multiple lengths. Its oracle invokes mix2x2_block followed by old_reduce_plane (sum_into_block for later contributors). Its DUT uses fold_cohort and ordered_accumulate_block. It must NOT be described as an actual scalar per-lane fold_plane fallback test.

`the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign` (runtime.rs3749) directly invokes REAL ArenaMembers::fold_plane, exercising mix2x2_block and the initial-store branch with negative-zero PCM. It does not exercise the subsequent `sum_into_block` branch because its sole FoldLane.store is true.

### Precise remaining native fallback seam before numbering

No existing graph test found here invokes real ArenaMembers::fold_plane with store=false. Existing rack fold tests use their own FoldMembers implementation; they prove dispatch but cannot be credited as invoking the graph's actual kernel callback. The earlier numbering draft's blanket “existing real graph fallback case” therefore needs this explicit narrow amendment: extend the existing first-contributor test in runtime.rs with a second real fold_plane contribution with store=false, preserving the first-store negative-zero assertion, and compare both planes' words with an independent original ordered-add expectation. A finite multiple-frame/tail length can reuse the same fixture. No production callback or new test framework is needed. This is within the previously allowed existing graph-test scope but must be written into the numbered acceptance before Luna starts. The existing test command remains exact and nonempty.

## Existing supported Wasm identity commands

The non-timed G5 command is:

```
bash scripts/run-wasm-gates.sh target/ci/lane2-wasm-gates
```

It builds/runs existing wasm-gate-guest/corpus with native, scalar-Wasm and simd128 legs. Its direct lane cases enumerate Sum2 and SumInto, not Mix2x2. Keep all current pins unchanged. The comment about historical software FMA does not override today's Lane::fma two-rounding implementation.

An existing actual shipped simd128 graph/route identity gate DOES exist, without invoking wasm-console timing:

```
cargo test --locked -p host-web --lib tests::native_identity_session_digest_pins_the_wasm_parity -- --exact
cargo test --locked -p host-web --lib tests::native_command_timeline_digest_pins_the_wasm_parity -- --exact
bash scripts/build-web-audioworklet.sh "$lane2_artifact_directory"
node hosts/host-web/tests/browser-v1/direct-oracle.mjs "$lane2_artifact_directory" hosts/host-web/tests/browser-v1/expected.json
```

Root supplies a new empty absolute artifact directory and retains immutable source/build provenance; no existing artifact overwrite or repin. The native tests also run --release. Direct-oracle's exact CLI is ARTIFACT_DIRECTORY EXPECTED_JSON; it executes the actual `miso-engine-v1-audio-worklet.simd128.wasm`, compares PCM word digests to existing native pins, and rejects mismatches before printing. It additionally executes command/observation timelines. This is non-timed Wasm execution, not the browser matrix or a latency measurement.

The fixture has an explicit track PostMatrix→Route→Output with identity RouteTransform. Current graph routing always calls the matrix kernel for NodeKind::Route; if bank route folding is selected, ArenaMembers' folded cohort also invokes the same mix2x2_block before ordered accumulation. Thus route folding does not erase this matrix seam. The command timeline changes the builtin matrix; do not confuse that builtin's arithmetic with the separately prepared route matrix. Existing native direct asymmetric test plus unchanged route digest together provide the representative arithmetic/live-target evidence; the browser identity route itself is identity-coefficient coverage, not an asymmetric-route matrix corpus.

### Wasm limitation and exact scope decision needed

The shipped direct oracle executes simd128 only. G5 has no graph dependency and no direct Mix2x2 case, so neither existing command provides scalar-Wasm matrix execution. The numbering draft requires scalar-Wasm matrix code-generation inspection, which is still supplied by the three-kernel probe. If root intends to require executed direct matrix identity at BOTH scalar-Wasm and simd128, that is not currently supplied by these existing gates: freeze a bounded additional Mix2x2 case in the existing wasm-gate-corpus before numbering (one kernel variant and its four existing signal entries, with baseline scalar-derived new pins and all OLD pins unchanged). That would add the existing corpus lib/pin files to scope; it must not be improvised as an implementation-time requirement. Alternatively explicitly freeze representative executed simd128 matrix identity plus scalar-Wasm decoded-code evidence and G5's direct sum coverage as the intended existing-tools target gate. The latter is the smallest representative choice consistent with the previous draft's existing-route path, and is my recommendation. No universal direct-three-kernel scalar-Wasm execution claim then follows.

Do not invoke `wasm-console` or operator timing scripts to obtain identity: its CLI executes timing workloads and has no identity-only switch. Likewise do not run `wasm_gates --native-timing/--wasm-timing`. The G5 and direct-oracle commands above are the existing non-timed paths.

Before final numbering, adopt the one existing fold_plane-test extension and make the scalar-Wasm matrix evidence choice explicit. No new framework or production integration is needed, and no test/build/benchmark ran during this source-only gate resolution.


# LANE-2 gate amendment — approved root choice frozen

Supplement binding at numbering to `/tmp/astra-349-lane2-numbering-brief.md` and `/tmp/astra-349-lane2-exact-gates.md`. Root explicitly chooses executed matrix identity on BOTH scalar-Wasm and simd128, and the real fallback accumulation extension. Queue remains behind460 and active work. No implementation, tests, builds or timing performed here.

## Minimal existing corpus change before kernel edits

Allow ONLY `tools/wasm-gate-corpus/src/lib.rs` and its existing `src/lane_digests.in` in addition to the prior product/test/evidence paths. Add a single `Kernel::Mix2x2` variant/name/import/run arm and append it after SumInto in KERNELS (12→13). Reuse the four existing SIGNALS in their current order: Noise, Impulse, Dc, Subnormal. This creates exactly four new lane cases; do not introduce a signal family, runner, package, expected-output format or new corpus.

Freeze coefficients `[0.9, -0.1, 0.2, 0.8]`, matching the existing native lane support choice. The existing corpus result adapter holds a single output block: do not silently hash only the left matrix output and let the right store disappear. Smallest fixed-layout representation: split that prefilled block at its half-length into mutable left/right planes, run mix2x2_block<L> on those equal halves, and leave both output halves in the existing block for ordinary deinterleave/digest. Existing FRAMES is even and both halves contain whole groups at W1/W4/W8; assert/document that representation invariant. First half of each returned lane transcript is the left result, second half the right result. Noise gives distinct temporal input halves; all four existing stimuli retain their normal fill/state rules. This uses the current single-array digest route and covers both stores without a new two-plane digest framework. Do not combine outputs by arithmetic or discard one plane. Direct native hostile/short/tail tests remain the stronger per-sample independent oracle; corpus fixtures are cross-target execution identity.

## New pins must precede the optimization

Create a coherent baseline-evidence checkpoint containing the added corpus case and its NEW pins while ALL THREE production kernel bodies still match the frozen unmodified implementation byte-for-byte. Derive the four new pins only from the current baseline scalar kernel through the existing `wasm_gates --print-pins` mechanism. Retain its exact command/status/output and the kernel-unchanged Git comparison. Do not derive pins from rewritten kernels, a Wasm result, a mismatch repair or timing output.

The array currently lays out all kernel×signal cases before three elementwise cases. Adding the four cases therefore inserts four entries before the old elementwise tail; later case indices shift. Preserve every OLD named case's exact32 digest bytes and ordering relative to other old cases. This is not permission to regenerate/replace old pins. Verify the complete old named-case→digest mapping is unchanged and exactly four named entries are new; ignore index relocation when comparing identity. Other families' own pin files are untouched. Existing computed CASE_COUNT/LANE_CASE_COUNT offsets adapt from KERNELS length; no arbitrary report/schema limit change.

Run the ordinary existing non-timed G5 gate on this baseline corpus addition before optimizing: `bash scripts/run-wasm-gates.sh target/ci/lane2-baseline-wasm-gates`. Its native scalar/Simd4/Simd8 and supported scalar-Wasm/simd128 executions must all accept the four new pins plus every old pin. Retain the case names/results and require nonempty full successful execution. Neither --native-timing nor --wasm-timing is authorized.

After any justified three-kernel rewrite, rerun the SAME corpus, coefficients, signal count and pins via `bash scripts/run-wasm-gates.sh target/ci/lane2-candidate-wasm-gates`. No pin edits after baseline freeze. This supplies actual scalar-Wasm and simd128 mix2x2 execution, removing the earlier evidence limitation. It does not replace named before/after code-generation inspection, direct native oracle tests, or the actual route reachability fixture. Root coordinates isolated targets; gate script fixed target locations must not overlap another agent's Wasm work.

## Real fallback accumulation completion

Allow the previously identified narrow extension in `crates/graph/src/runtime.rs` test `runtime::tests::the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign`. Preserve its first-store negative-zero assertions; add a second actual ArenaMembers::fold_plane invocation with FoldLane.store=false and independent per-plane original ordered-add expected bits. Reuse its lease/fixture, with a finite vector-plus-tail length if extending frames. This must execute the graph callback's real sum_into_block branch, not merely the old_reduce_plane oracle or rack's custom fake FoldMembers. No production callback/folding change.

The exact native command remains `cargo test --locked -p graph --lib runtime::tests::the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign -- --exact` plus --release. Retain the other exact commands in the prior supplement (independent asymmetric route oracle, actual bound Route-op PDC/reduction fixture, folded cohort comparison, G2/P1). Require the named test executes once.

## Scope and closure

This is the root-selected minimal execution-evidence completion for the same three-kernel product, not a broader matrix, benchmark framework or new DSP outcome. Baseline corpus checkpoint is prerequisite to kernel source edits and to any later before/after evidence claim. If pre-edit decoded bodies already eliminate the alleged checks, preserve the honest-null decision; do not manufacture a source optimization. Added corpus baseline still closes the execution-evidence gap without claiming a performance gain.

All previous arithmetic, debug equality/release prefix/short rejection, zeroalloc, real reachability and qualification restrictions remain. No existing pin can be changed to obtain green output. Final numbered issue must adopt this amendment and exact base before Luna1. Root remains responsible for scope synchronization, checkpoints, later qualification, actual PR Astra review and required CI; #349 unrelated findings remain open.




## Numbered scope PASS; queued

# Astra #463 numbered scope — PASS, queued only

Exact planning checkpoint `b7ef921e8d568bdaacf11341249ff32862eff974`, `/home/bl/misofm/engine-lane2-plan`, base29a8c88b82de8660a5d573e75b7e67d977496576. PASS for numbered scope and synchronization, not implementation authorization.

Live issue463 is OPEN with exact title “Prove block-slice bounds once in three lane kernels”; body exactly matches `.github/ISSUE_SPECS/463-lane-kernel-bounds.md`. Only that spec differs from the planning base. All three adopted /tmp documents are included verbatim. The introductory precedence clause explicitly selects the FINAL amendment over the earlier scalar-Wasm disassembly-only option.

Scope preserves all three kernels, original two-rounding operand order and matrix snapshots, debug equality/release checked-prefix/short rejection, exact native graph filters and the pre-edit honest-null code-generation checkpoint. Final mandatory gates correctly include the real ArenaMembers::fold_plane store=false extension and one Mix2x2 variant with four existing signal cases. Both output halves feed the unchanged corpus digest mechanism. New scalar-baseline pins precede ANY kernel rewrite; all existing named pin bytes remain immutable despite index shifts. Existing G5 must execute both scalar-Wasm and simd128 matrix cases. No new corpus/runner/framework/timing authority or production graph change is added.

Queue remains behind active459 and queued460; current462 delivery is also recorded. Historical draft references to older heads/issue states are contextual and superseded by the explicit planning-base/queue paragraph. Root must perform a separate actual implementation-base review, synchronize that freeze and assign Luna1 only when ordering permits. No feature implementation, qualification or timing follows merely from this numbered approval.

Read-only Git/GitHub/spec inspection; no tests, builds, timing or repository/Git/GitHub mutations.

## Delivered-base readiness refresh

# #463 current-base readiness — scope remains valid, queued

Read numbered463 at engine-lane2-plan79b88a4f and current delivered024ad674 source in engine-483-floor-parity. No code, tests/builds, timing or Git/GitHub mutations. #238 remains runtime priority; this is not assignment authorization.

Relevant lane kernels, G5 runner/corpus/pins and gate tools have no source delta across the compared bases. Current kernels remain sum2_block473, sum_into_block498, mix2x2_block612. Graph runtime changed for delivered scalar pairing, but the named route/fold tests and actual matrix/sum callback paths remain; no graph production adjustment is required for463. Public debug equal-length assertions, release controlling-prefix behavior/short rejection, two-rounding operand order and matrix old-plane snapshots remain the contract.

Refresh stale scheduling/base references only: old459/460/462 work is delivered; queue now follows active238 and a root-frozen later main. Historical60519995/29a8 line references are evidence context. Current exact graph filters still exist:
- runtime::tests::route_applies_folded_gain_with_frozen_op_order (runtime4633)
- tests::executor_applies_exact_pdc_then_fixed_pairwise_reduction (graph lib3598)
- runtime::tests::a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit (runtime3962)
- runtime::tests::the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign (runtime4211)

Use `cargo test --locked -p graph --lib FILTER -- --exact` and matching --release for these four filters, requiring one actual test each. Existing lane command remains `cargo test --locked -p lane --test g2_kernel_identity --test p1_partition`, debug/release. The negative-zero fixture currently tests real fold_plane store=true and a separate fold_cohort case; neither supplies real fold_plane store=false. The frozen SECOND contribution extension remains necessary and must preserve both existing assertions, not replace them. Its old-order expected words stay independent of the candidate sum traversal.

The final root-selected corpus amendment remains binding over earlier optional scalar-Wasm wording: KERNELS currently12, FRAMES1024, four SIGNALS, no direct Mix2x2 case. Add exactly one variant and four named cases, using the prescribed half-array L/R representation and coefficients. Generate ONLY four new scalar-baseline pins with `cargo run --locked --release -q -p wasm-gates -- --print-pins`; preserve full output as evidence and transplant no other pin. All old named-case digest bytes/order must remain identical despite shifted indices. Both planes must remain in the ordinary digest transcript. No record schema, other family pins or signal population change.

Mandatory order: freeze actual main; retain pre-edit three-kernel body identity; perform the opaque-wrapper native/scalar-Wasm/simd128 code-generation inspection; checkpoint the corpus addition/new scalar pins while all three kernel bodies remain unchanged; execute `bash scripts/run-wasm-gates.sh target/ci/lane2-baseline-wasm-gates`; only then consider a justified kernel rewrite. The final candidate repeats the SAME cases/pins using `target/ci/lane2-candidate-wasm-gates`. The gate internally fixes guest targets at target/ci/wasm-gates-* despite its output-directory argument, so root must serialize those directories with other Wasm work. No timing switches, console runner or workload measurement.

No prebuilt opaque-wrapper evidence currently resolves the claimed checks. The numbered requirement to freeze a disposable retained probe source/exact rustc/LLVM/flags/status/body hashes before edits remains; no null conclusion may be inferred from these source reads or a generic vectorization report. If all supported decoded bodies already remove the alleged checks, keep the honest-null path and do not manufacture an optimization. The baseline corpus still provides the explicitly approved execution evidence.

Existing native host filters remain `tests::native_identity_session_digest_pins_the_wasm_parity` and `tests::native_command_timeline_digest_pins_the_wasm_parity` with `cargo test --locked -p host-web --lib FILTER -- --exact` and release. Existing normal worklet builder and direct-oracle.mjs ARTIFACT_DIRECTORY EXPECTED_JSON remain the untimed shipped simd128 route gate, with root-owned empty artifact directory/current accepted pin. Direct G5 now must cover matrix execution on BOTH scalar-Wasm and simd128; no claim that simd128 direct oracle alone covers scalar-Wasm.

Allowed product/test/probe/corpus paths and original finite W1/W4/W8 bounds/hostile-category/suffix/zero-allocation gates need no expansion. A later actual-main integration and source-equivalence check still precede Luna1. Preserve broader artifact/measurement separation, no native AArch64 revival, no changes to reduce_many/ordered_accumulate_block, and no timing authority.

Root selects463 as the next runtime feature after238 delivery. Actual mainf357330ce0f391e785429e82807d7d79ba9a077c is integrated atdda36da0fbb772671bf3467db0f03f6039edbcfa; crates/tools/hosts/Cargo/.cargo/scripts are byte-identical to that main. Historical behind459/460/462/238 scheduling is superseded by this record. Pending fresh actual-base Astra approval before Luna1. Existing pre-edit probe and four new baseline scalar pins must precede any kernel rewrite; timing remains unauthorized. Independent488 test-only work touches graph/program.rs, so463 must not edit that file; its allowed real fallback proof is graph/runtime.rs.

## Actual implementation base approval and Luna attempt 1

# Astra #463 actual implementation-base review — PASS

Exact pushed head3c6c765c3948a9b9eaa3d22fcf8fa208402fd844, engine-lane2-plan. Actual delivered mainf357330ce0f391e785429e82807d7d79ba9a077c integrated atdda36da0fbb772671bf3467db0f03f6039edbcfa. Independently checked the only tree delta from that main is463spec. Relevant lane/G5 inputs retain the prior reviewed baseline; the appended current-base readiness and final corpus/executed-Wasm priority preserve the approved contract. No tests/builds/timing or repository/Git/GitHub mutations performed.

Approve root assignment of fresh Luna1 for the ordered initial pre-edit work. Freeze and retain three-kernel body identities and exact opaque probe source/toolchain/flags/decoded symbols on native x86-v3, scalar Wasm and simd128. Determine whether the alleged repeated checks survive actual lowering; preserve an honest null rather than forcing an optimization.

Before any kernel rewrite: add only the single prescribed Mix2x2 corpus variant and its four existing-signal cases, derive ONLY four new baseline scalar pins while all three original kernel bodies remain byte-unchanged, retain all preexisting named pins, and checkpoint that corpus/pin milestone. Execute the existing baseline Wasm gate on both scalar and simd128. A rewrite may follow only this accepted/recoverable baseline sequence and a supported mechanism finding. No new timing or benchmark authority exists.

All existing arithmetic/order, debug equality and release prefix/short rejection, W1/W4/W8 hostile-category and allocation obligations remain. The real fold_plane store=false second-contribution extension stays in graph/runtime.rs, preserving existing store=true/cohort assertions and an independent ordered expected value. Do not edit graph/program.rs:488 owns that independent test-only file. No reduce_many/ordered_accumulate_block rewrite or AArch64 revival.

Root must serialize the G5 fixed internal guest target directories with ongoing Wasm qualification (the output-directory argument does not isolate those targets). Root controls worker assignment, checkpoint/push and eventual combined qualification. The current updated sequence after delivered238 supersedes historical queue references. Scope is ready without further amendment; source acceptance, exact-head PR review and requiredCI remain later gates.

Root adopts PASS and assigns fresh Luna1, beginning only the mandatory retained opaque probe and unchanged-kernel corpus/pin baseline sequence. Report/checkpoint before kernel edits. No timing is authorized; use the isolated worktree target directories and do not overlap488 graph/program.rs.

## Luna attempt 1 recoverable pre-edit checkpoint

The three kernels are unchanged. The G5 corpus adds the prescribed matrix case/four scalar-derived pins; root verifies every old named pin is unchanged. Raw baseline G5 output reports native/scalar-Wasm/simd128 cases139/comparisons349 with no mismatches. A retained opaque example and large native/Wasm disassemblies exist. This checkpoint preserves work before any kernel rewrite; it is not baseline acceptance.

Root notes the reported /tmp/463-luna1-* files contain build/decode/output logs but no retained command/status/toolchain/hash record files matching the claimed provenance. The current probe hardcodes Simd8 and one common length for all spans; the frozen supported Simd4/scalar interpretation and exact loop-check claims require independent assessment. Initial placeholder pins were replaced by actual scalar output before this checkpoint. Astra must assess baseline sufficiency and allowed permanent probe placement before any rewrite. No timing or runtime kernel change is claimed.

## Astra Luna attempt 1 baseline verdict and Sol attempt 2

# Astra #463 Luna attempt 1 baseline gate — FAIL; no kernel rewrite authorized

Exact head9eb2e3eb64c38287ae91e3a159d0e8fd975cefd4, engine-lane2-plan. Read frozen numbered gate/precedence, cumulative source and available463-luna1 outputs. This is the one consolidated initial-pass verdict; root may assign Sol2 after recording FAIL, not further Luna repairs. No tests/builds/timing or repository/Git/GitHub mutations performed.

Accepted progress: all three production kernel bodies remain untouched. The corpus adds exactly the prescribed Mix2x2 variant after SumInto, splits the existing array into left/right halves, applies[0.9,-0.1,0.2,0.8], and retains both outputs in the ordinary digest. The four new signal pins are inserted before the elementwise tail; other pin families are untouched. Root's old-named-map verification reports51 old entries unchanged and4 new. Existing G5 output reports139 cases/349 comparisons and no mismatches across native/scalar-Wasm/simd128. This is useful baseline execution output, not sufficient frozen provenance or supported codegen proof.

Three finite original gate groups require completion:

1. Correct probe scope and arguments. The new crates/lane/examples/issue463_probe.rs is a permanent Cargo example with crate-level unsafe allowance and executable allocation/argument handling. Frozen permission was a disposable retained inspection probe, not a new shipping Cargo target. Remove it from permanent lane target discovery and retain exact probe source as evidence outside that target tree. Use independent runtime lengths for every slice (out/a/b; acc/x; left/right) plus opaque coefficients/output. The current single shared len proves equal spans by construction and removes the unequal-length question. Preserve valid-span/nonoverlap safety if using raw ABI wrappers; no production unsafe or new public kernel API. Instantiate native Simd8 and relevant scalar/tail, actual Wasm simd128 Simd4 and scalar implementations explicitly. Hardcoding Simd8 for every target cannot stand in for the required Simd4/f32 instantiations, even if Simd8 itself lowers to pairs of v128 instructions.

2. Supply named-body interpretation and authentic build/decode provenance. Current huge decoded modules contain real named bodies; they are not empty evidence. For example native issue463_mix2x2 has repeated compares/branches at13f20–13f2d before the vector loads and a backedge13f6c, so one cannot declare a universal null merely from presence of vector arithmetic. However that one body and current same-length hardcoded-width probe do not establish all three kernels/required targets. Retain exact probe/source hash, rustc/LLVM versions, target flags, profile/LTO/codegen settings, compiler AND decoder commands/numeric exits, object/module/body hashes and a concise per-body account separating entry rejection, loop backedges, inner bounds/panic branches and scalar tails. Do not infer target applicability from a module-wide panic search. Existing logs lack the claimed command/status/toolchain/hash record files; recapture under the corrected probe with honest new attribution rather than inventing old records. Non-LTO inspection is not the shipped artifact. Only accepted corrected baseline evidence can justify either the bounded rewrite or an honest null.

3. Finish baseline pin/execution provenance BEFORE any rewrite. Preserve original print-pins outputs and G5 logs, but capture the actual current baseline scalar --print-pins command/status/output and kernel-unchanged comparison, exact51-old/4-new named-map proof, and the ordinary baseline G5 command/status/nonempty result records. Current source pins must remain identical during this completion; a mismatch requires diagnosis, not silent replacement. Reconfirm both output planes, all four existing signals and every old named pin through the same gate. This is the already-required baseline run, not a new workload or corpus. Preserve placeholder/initial attempts candidly. Checkpoint corrected probe/corpus/evidence with kernels still untouched and obtain the pre-edit gate ruling before implementation proceeds.

No expansion to a generic report framework, timing, AArch64, more kernels or additional corpus. Full product arithmetic/bounds/negative-zero store=false/identity/allocation and candidate qualification remain retained obligations, not failures manufactured for work deliberately held until baseline acceptance. Their absence at this initial gate does not authorize dropping them. Root controls isolated targets and fixed G5 guest-directory serialization. Existing source may be preserved as a useful failed checkpoint; no optimization or speedup claim is accepted yet.

Root adopts this consolidated FAIL and assigns Sol2 to repair the original baseline proof first. Preserve the four accepted-shape corpus additions and old pins; production kernels remain unchanged until root/Astra accept an actual target-correct baseline. Any later warranted kernel implementation remains within the original scope and this attempt; no timing or broader optimizer work is authorized.

## Sol attempt 2 corrected pre-edit baseline, pending Astra acceptance

At `60a3bb858d80ee186f0f38bef3beb11988a38f2a`, Sol removed the probe from Cargo example
discovery and retained it under `artifacts/issue463-lane-bounds-baseline/`. Its opaque ABI supplies
independent lengths for out/a/b, acc/x and left/right, runtime matrix coefficients, and observable
checksums covering all written outputs. The named instantiations are native Simd8 plus `f32`, and
Wasm Simd4 plus `f32`. All three production kernels remain byte-unchanged.

Rust 1.97.1/LLVM 22.1.6 compiled non-LTO native x86-64-v3, scalar-Wasm and simd128 objects; all
corrected compiler and decoder statuses are zero, with the earlier missing-dependency-path failures
preserved candidly. Every required named body is nonempty. Direct body inspection finds repeated
operand-span comparisons and reachable slice-failure edges inside the vector loops of sum2,
sum_into and mix2x2 on each applicable target, with separate loop backedges and scalar tails. This
is a supported mechanism finding that warrants the frozen checked-prefix rewrite if Astra accepts
the corrected baseline; it is not a timing or speed claim. Exact commands, flags, hashes, offsets
and interpretations are retained in the artifact README and full decoded outputs.

Fresh scalar `--print-pins` output exited zero and is byte-identical to the checked-in pin file.
The named-map comparison proves 51 old entries unchanged, zero changed and exactly four new
Mix2x2 signal entries. Fresh ordinary G5 exited zero: native, scalar-Wasm and simd128 each executed
139 cases/349 comparisons with empty mismatch lists, and both detector-residency legs passed.
The corpus shape and current pins were not changed during this repair. This is the first coherent
Sol2 baseline tranche; no kernel rewrite begins before root checkpoints it and Astra explicitly
accepts the pre-edit baseline.

## Root exact-command recapture

Root independently rebuilt and decoded the unchanged probe/kernels at9c2741ab. Sol’s original compiler command record used placeholders and is retained as a template, not represented as exact invocation provenance. `root-recapture/` supplies actual expanded argv/cwd/environment overrides, tool versions, numeric exits, source/rlib/object/decoded/body hashes and raw outputs. All12 commands returned0. All three object hashes match Sol’s recorded objects. All18 decoded instruction bodies match; five native extracted records additionally include the next section label, documented in comparison.json without modifying either raw capture. The compiler/decoder recapture is the authoritative exact-command baseline; no kernel rewrite or timing occurred.

Pending mandatory Astra pre-edit acceptance of Sol attempt2 baseline.

## Sol attempt 2 baseline acceptance

# Astra #463 Sol attempt 2 PRE-EDIT baseline — PASS

Exact pushed head c7c8e4f09bb1f5c43c02b135f092b04599688f15, engine-lane2-plan. This accepts the corrected baseline gate only and permits continuation of the SAME Sol2 implementation attempt; it is not a final product verdict. No tests/builds/timing or repository/Git/GitHub mutations performed.

The failed Luna probe is removed from permanent Cargo target discovery. Retained evidence probe now supplies independent lengths for every operand, opaque matrix coefficients and observable output; actual native Simd8/scalar and Wasm Simd4/scalar instantiations are separately named. Raw-pointer contracts state valid independent spans and mutable nonoverlap; this compile-only disposable inspection does not add production unsafe/API. The checksum observer is separate from the kernel loops and must remain so in candidate interpretation.

All production kernels remain unchanged, SHA25620dcb8d8abbc834f51e5357ff8fb8ee4c1e32e373bc5d7d8b32e6d358a9e469b. Corrected decoded bodies support a non-null opportunity: native Simd8 sum_into repeatedly checks both spans at40..66 before load/add/store6c..76 and returns on its backedge8a; sum2/matrix similarly retain per-operand span branches in their vector loops. Actual Wasm Simd4 bodies include the expected v128/f32x4 arithmetic with span checks inside the encompassing loop; scalar-Wasm uses scalar lowering with corresponding bounds exits. Explicit f32 bodies/tails are present, with compiler autovectorization where applicable. Do not equate every branch or module panic import with avoidable work: candidate comparison must still separate entry validation, genuine loop termination, vector checks and scalar remainder/observer work. This baseline supports the frozen checked-prefix rewrite, not a universal claim that every scalar/native branch will disappear.

Root recapture is the authoritative invocation evidence: expanded argv/cwd/environment overrides, exact rustc/LLVM/decoder versions, matching dependencies and explicit opt-level3/codegen-units1/non-LTO/target flags are retained, with12 actual numeric0 statuses. Original Sol placeholder compiler commands remain labelled templates. The three recaptured object identities match prior objects, and18 body comparisons preserve the documented distinction between instruction identity and five native records including a next-section label. No LTO bitcode is falsely described as decoded Wasm and this probe is not the shipped worklet.

Independently verified all109 unique manifest payloads by hash/size and exact tracked coverage110 including manifest. Independently compared named pins:51 old entries all unchanged, precisely4 new Mix2x2 noise/impulse/dc/subnormal entries. Corrected corpus remains the prescribed two-plane half-array case. Retained scalar print-pins command/status0 and ordinary G5 command/status0 accompany the raw outputs; G5 records native/backend2 and Wasm/backend0/1 each139cases/349comparisons with no mismatches. Root/corrected recaptures are new evidence, not invented provenance for Luna's missing records. Current pins are frozen before rewrite.

Root may now authorize only the already-frozen three-kernel checked-prefix/chunk implementation within Sol2, preserving debug equality, release controlling prefix/short rejection, exact arithmetic/order and matrix snapshots. All final public bounds/hostile-category/identity/allocation, real fold_plane store=false and SAME-corpus candidate/codegen gates remain mandatory. No pin edits, extra kernels, timing, AArch64, generic framework or fourth-attempt accounting follows from this baseline PASS. Final Sol2 gets one consolidated product verdict after its coherent completion.

Root adopts baseline PASS and authorizes continuation of the SAME Sol2 attempt, with checkpoint at the first coherent focused-green implementation tranche. Final product verdict remains pending.
