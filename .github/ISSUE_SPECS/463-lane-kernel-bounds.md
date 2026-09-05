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


