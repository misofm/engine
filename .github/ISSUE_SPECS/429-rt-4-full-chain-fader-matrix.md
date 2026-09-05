# Process settled full-chain fader and matrix in one traversal

**Ready for root to number a bounded FIRST product child, with an explicit retained RT-4 integration obligation and separate descriptive-measurement qualification. No implementation before #420 merges and root freezes actual main.** Astra read current source and the exact #349 row; no tests, Cargo, timing, legacy inspection or repository/GitHub mutation. Root has approved splitting independently useful full-chain and bank/compiler outcomes rather than silently closing the broad finding.

## Correct premise and scope ruling

The exact RT-4 row is “Fader and matrix are three block passes for six lane-ops; fuse the settled arms into one”, pointing to the fader/matrix stages. The three passes are LEFT gain/mute, RIGHT gain/mute, and paired matrix. They are not input/fader/matrix. Keep input processing wholly separate: its post-block nonfinite recovery can zero an entire channel and reset filter state before downstream processing. Folding input into this change would introduce a different recovery/ordering problem.

A second inventory correction matters: `BuiltinChain` contains prepared-only `FaderStage<f32>`, not `FaderRampStage`. Its fader cannot ramp. `BuiltinFaderBank` and live scalar faders use `FaderRampStage`; compiler `FaderBankProcessor` and `MatrixBankProcessor` are distinct prepared stages, with separate queues and observable post-fader/post-matrix boundaries. There is no existing full-chain SIMD bank dispatch to optimize by changing BuiltinChain alone.

Smallest child title: **Process settled full-chain fader and matrix in one traversal.** Deliver an actual improvement to public BuiltinChain::process_dual_mono and a generic existing-vocabulary Lane kernel. This child can close independently. It does NOT close RT-4 in #349, nor claim live console bank fusion. Before implementation root must record a named successor obligation for safe bank/compiler integration, including observable-boundary/queue eligibility; do not smuggle that architecture into this child.

## First child's exact implementation boundary

- `crates/lane/src/kernels/builtins.rs`: one ordinary generic block function next to gain_mute_block/matrix2x2_block, combining those exact operations. No new trait, intrinsic, unsafe, scratch or dependency. Follow existing public lane-kernel placement/naming; preserve old primitives as independent reference and section API implementations.
- `crates/builtins/src/lib.rs`: route the settled post-input arm of BuiltinChain::process_dual_mono through that function. InputStage::process still completes first, including reports/recovery. Select fusion only when the matrix's existing relevant countdown is zero at block entry. The prepared FaderStage is always settled. Matrix active/ramping arm remains the existing two-stage sequence for the WHOLE call, even when its ramp ends within the block. Do not add a ramp-tail optimization.
- Existing builtin stage/matrix tests, one focused lane test file (a new small `crates/lane/tests/fader_matrix.rs` is acceptable), and existing `tools/bench/src/builtins.rs` unit-test context only if needed to reuse its installed allocator for a NON-TIMED repeated full-chain proof. Numbered evidence. No compiler/graph/rack change, manifest/dev-dependency change, corpus-pin rewrite, new API for bank pairing, runner work or generated artifacts during this semantic pass.
- Existing section-specific process_input/process_fader_mute/process_matrix APIs, into_sections, setters, reset, readback and first_sample handling stay unchanged.

## Frozen operations, masks and eligibility

For each frame-vector, load both original input planes before writes:

```
l = load(left).mul(gain_left).andnot(mute_left)
r = load(right).mul(gain_right).andnot(mute_right)
yl = select(identity, l, ll.mul(l).add(lr.mul(r)))
yr = select(identity, r, rl.mul(l).add(rr.mul(r)))
store(left, yl); store(right, yr)
```

Preserve coefficient-before-sample multiplication order exactly, not merely mathematical commutativity. Both matrix arms are evaluated then selected as in the old primitive. No FMA, coefficient precombination, alternate summation, gain-one shortcut, zero multiplication for mute, extra sanitization/flush or new identity predicate. Muting clears to exact +0.0; matrix identity returns the already-faded words, preserving signed zero. Carry through existing nonfinite arithmetic semantics when testing raw kernel input. Cross-target NaN payload identity is NOT claimed: the existing corpus explicitly excludes it, while full-chain nonfinite inputs are sanitized at the input boundary. Compare same-width old/new primitive outputs under the same FP environment without normalization or tolerance.

The generic kernel's shape is AoSoA `frames * L::WIDTH`: width is tracks, not frame SIMD. There is no invented partial-vector tail in this API. Test scalar tracks and complete W4/W8 frames, including partial populations represented by existing identity/unmuted padding and selected/holey coefficient/mute/identity masks. Do not claim support for malformed short slices or new active-mask behavior. The first product dispatch remains scalar; generic kernel W4/W8 tests earn arithmetic reuse, not bank integration.

Input may be ramping while downstream matrix is settled: because input is completed unchanged first, this is still eligible. A nonzero matrix countdown is ineligible even if current coefficients happen to equal identity or the ramp completes this call. Reset/retarget semantics and all ramp words/countdowns must match the old sequence. No automation drain or queue changes.

## Small, discriminating acceptance set

1. **Old primitive oracle:** for each f32/Simd4/Simd8 width run gain_mute_block(left), gain_mute_block(right), matrix2x2_block on independent copied buffers, then compare every output bit to the new kernel. Frames1,3,8,9,128 suffice for independent frame-loop sizes; widths do not imply temporal tail treatment. Cover asymmetric gains/mutes, nontrivial crossfeed, identity/mixed identity masks, signed zeros, finite cancellation, subnormal and nonfinite inputs. Include poison outside processed slices and preserve unrelated buffers. Pin representative wrong alternatives (mute via multiplication; matrix after overwriting left; zero-seeded/recombined arithmetic) to inputs that actually distinguish them before DUT assertions. Do not build a large Cartesian corpus.
2. **Actual public dispatch:** independently prepared chains receive identical buffers and events. DUT calls process_dual_mono; reference calls the three existing section APIs in order. Compare L/R bits, full BuiltinProcessReport, input state/recovery counts and existing matrix state/readback after each block and reset. Use enabled/asymmetric and disabled input sections, sanitized NaN/infinity and existing injected bad-state recovery case. Input recovery remains before cross-channel mixing.
3. **Eligibility and smoothing:** settled matrix must use the new helper; initial/retargeted matrix ramps use unchanged fallback, including zero-ramp immediate target, block-crossing snap, mid-ramp retarget and next block becoming eligible. Preserve partition-at-event behavior from existing stage/matrix tests. There is no fader ramp in this child; explicitly retain and rerun existing live fader-ramp tests without claiming those stages were fused.
4. **Mechanism:** add a narrowly scoped test-only dispatch witness reachable through the actual public method, preferably the existing private test context with per-instance/serialized test state. It must distinguish fused vs old separate calls and show the eligible call actually selected fusion; restoring old dispatch must make the same assertion fail. Avoid global mutable counters racing tests, permanent telemetry or source-string-only labels. Kernel source establishes one paired frame traversal/one store per plane after the arithmetic. Output equivalence alone is insufficient.
5. **Realtime:** reuse the existing tools/bench builtins allocator and `audit::in_render_scope` for a non-timed unit test if no existing test covers the new public seam. Prepare/allocate outside the measured scope, prove installed/live allocator (positive allocation AND free), then repeat eligible and fallback full-chain renders with zero audited operations. Use the existing allocator process setup rather than attach a new allocator to builtin unit binaries. Do not run any benchmark to earn this gate. Retain realtime/lane/builtin/unfused policy, focused debug/release tests, fmt/diff/clippy and existing deterministic pins.

This is one coherent half-day source/test pass. If a necessary test seam or allocation harness requires an unrelated ownership/dependency change, stop before expansion and freeze the specific minimum amendment. No generic instrumentation framework or extra feature.

## Separate qualification and measurement

Mandatory correctness/realtime/target/identity and proportional full-workspace delivery remain with the source child. Root performs those after semantic PASS with an immutable source candidate. Supported target/artifact/static/browser requirements remain applicable if current Rust reaches the artifact; use the existing proven candidate convention. No AArch64 scope revival or source-level claim of machine instruction count/speedup.

Descriptive timing needs a separate stateless qualification child before scheduling: the existing `tools/bench/src/builtins.rs` FullChainFilters and IdentityChain workloads actually call this public seam; console46 bank rows do NOT. Existing run-builtins-benchmark.sh/preflight-builtins-benchmark.sh are sealed Issue072 tooling with historical issue35/68 dependencies and hardcoded output/seal authority. They are not a fresh reusable namespace. Do not invoke, overwrite, relabel or quietly generalize them in the feature child.

That qualification brief must freeze exact existing workload/fixtures/validator applicability, a fresh output authority and profile/binary, zero-launch preflight and bounded runner changes BEFORE timing. At most one controlled invocation with one warmup and two measured rounds, no tuning/retry; preserve refusal/raw failure and all historical records. If adapting sealed tooling is not a bounded coherent correction, keep measurement queued rather than inflate the source slice. No newly invented performance acceptance budget and no claim that untimed tests are measurement. This draft authorizes no timing or runner implementation.

## Retained RT-4 bank/compiler successor

Root must keep an explicit named successor and RT-4 OPEN after the first child. Its product is actual safe joint execution of the live settled fader+matrix pair. It must first establish adjacent compatible cohort/member/order ownership and absence of an intervening observable post-fader tap/send/meter, including record drain/application boundaries. The original separate processing remains fallback for any missing proof, incompatible/holey layouts, automation/ramping lane, intermediate observation or unsupported processor shape. Fusing cannot skip either queue consumer or move a recorded sample's application time. Do not reuse the prepared-only FaderStage as if it carried live fader state.

That successor needs a fresh Astra source-level ownership brief after the first child integrates; this paragraph preserves the obligation but does NOT authorize a speculative compiler pairing implementation. It will cover live scalar and W4/W8 pair selection, prepared dispatch discrimination, fallback, zero allocation, state/report/readback and exact per-lane arithmetic. If pairing spans independently useful compiler and host-observation changes, split before coding. The generic kernel child does not earn those claims.

## Queue and roles

Freeze post-#420 merged source before assigning Luna. One feature WIP remains #420 until delivery. Root numbers/synchronizes product, retained integration and qualification obligations before implementation; Astra approves the resulting stateless scopes. Luna attempt1, Sol after FAIL for at most two revisions, hard stop/rescope after attempt3. Root owns checkpoints/pushes and actual-head PR/required-CI merge. First product can close independently; broad RT-4/#349 remains open until the explicitly retained runtime integration and required accounting are complete.

## Numbered RT-4 accounting

This is #429. #429 owns the independently closable public full-chain product, #430 retains live bank/compiler integration, and #431 retains separately scoped descriptive measurement. Completing #429 alone does not close RT-4 in #349. All three are synchronized before implementation; #429 remains queued until #420 merges and Astra approves the numbered source scope on the frozen base. #430/#431 require their own final stateless scope approval before any dependent code or timing.

## Astra numbered scope approval

# Astra #429 numbered scope / #430–#431 retention review

**PASS for planning checkpoint `69d5cdadfadbaf398c8c4f36ad3baf7b8f092386`.** #429/#430/#431 are remotely OPEN with titles matching the local specs. #429 differs from the approved Astra draft only by its numbered title and explicit reciprocal accounting. No implementation is authorized until #420 merges, root freezes actual main and the scoped source seam is reconfirmed at assignment.

#429 is the independently closable public full-chain product: input completes unchanged; static prepared FaderStage plus settled matrix uses one paired frame traversal; matrix ramp calls retain whole-call fallback. Exact masks, multiplication operand order, identity select, NaN/FP limits, section APIs and reset/report/state behavior stay frozen. Generic W4/W8 kernel proof does not claim a live full-chain bank API. The focused source/test/allocator mechanism contract and mandatory product qualification remain with #429.

#430 properly retains actual live scalar/bank/compiler pairing, both queue consumers and application time, post-fader/post-matrix observability, eligibility/fallback and ownership proof. It explicitly requires a fresh complete stateless amendment before implementation. It is not an implied compiler rewrite or authority to substitute the static fader for live ramp state.

#431 properly isolates descriptive capture: the existing full-chain workloads exercise this seam, console46 alone does not; sealed Issue072 authority/history must remain untouched. A fresh frozen workload/profile/output/preflight/validator contract and at most one controlled invocation remain prerequisites. No runner code or timing is authorized from this retention record, and no correctness/realtime/target obligation is transferred out of #429.

Completing #429 alone does not close RT-4/#349. Both retained outcomes are numbered before implementation; original bank integration and evidence obligations are visible and not silently discarded. No material correction required. The earlier draft wording “ready for root to number” is historical context superseded by the explicit numbered accounting and need not block the queue.

Read-only local diff/spec and remote identity inspection. No tests, Cargo, timing, repository/GitHub mutation or implementation performed.

## Post-RT-3 implementation base

PR #437 merged as `c7469e28d52a716339b6f3119a57b69afb3411f1`, delivering #420 after exact-head Astra PASS and required qualification SUCCESS. Root freezes that exact main as the first full-chain product base. The descriptive RT-3 measurement is separately retained in #436 and does not own new source implementation. This feature remains unassigned until Astra reconfirms the actual builtin/lane/allocator seam against this merged base. #430/#431 retain their separate obligations; no compiler pairing or benchmark work is authorized here.

## Astra frozen-base approval and Luna assignment

# Astra #429 frozen-base confirmation

**PASS for planning head `8a6a9baf8be3f147a01fdd3a93b94893976f7a2d`, based on merged main `c7469e28d52a716339b6f3119a57b69afb3411f1`.** Root may assign the single Luna product attempt. This is scope/base approval, not source or delivery acceptance.

The post-RT-3 delta contains only the three approved planning specs. BuiltinChain, lane builtin kernels and tools/bench builtins allocator seam are unchanged from the inspected pre-RT-3 source. Actual full-chain processing still completes input then runs static left/right fader gains and matrix separately. Prepared FaderStage is static; MatrixStage owns the relevant countdown. Existing gain_mute_block and matrix2x2_block provide the exact independent primitive oracle and arithmetic vocabulary. The approved one-pass settled post-input scope therefore remains executable without compiler/graph ownership changes.

One concrete allocator-use clarification before coding: tools/bench already links the audited allocator. Prove installation and positive allocation/free liveness OUTSIDE render using its existing counters and an actual held/dropped allocation; `assert_installed` alone only asserts allocation movement, so include the free. Then use the existing thread-scoped realtime audit around repeated real full-chain renders for the zero-operation proof. Do not assert process-global zero deltas amid parallel tests, and do not change the process-global Abort/Count mode merely to perform the positive probe. No new allocator, dependency, benchmark invocation or mode-sensitive parallel test is required. This stays within the approved bench unit-test seam.

Retain exact arithmetic operand/mask/identity order, input recovery and reports, matrix ramp whole-call fallback, actual dispatch witness and category-preserving old-kernel identity tests. W4/W8 kernel tests do not imply a full-chain bank API. #430 retains live compiler pairing/queue/observation proof; #431 retains separately scoped measurement. #436's unused RT-3 measurement authority is independent and unchanged. #429 alone must not close RT-4/#349.

Root owns coherent checkpoint/push, Astra source verdict, Sol only following FAIL with at most two revisions, then hard stop/rescope. Mandatory source/target/workspace/current-artifact delivery and actual-head PR/required-CI gates remain. Respect any later #436 quiet window; #429 is the sole active feature and isolated #411 tooling may proceed without overlapping edits.

Read-only source/commit/spec inspection; no tests, Cargo, timing, repository/GitHub mutation or implementation performed. No further scope amendment is needed beyond recording the concrete allocator clarification.

Root records this allocator clarification and assigns Luna attempt 1 on the approved merged RT-3 base. The bounded #436 readiness window ended before invocation when load exceeded 0.50; its measurement remains unconsumed. #429 is the sole active feature implementation and must preserve the separate #430/#431/#436 obligations.

## Luna attempt 1 checkpoint

Root committed and pushed Luna's four-path tranche at `40de16521bae514150fe6729e052866c5ab1899e`: lane kernel and focused lane test, public builtin dispatch/tests, and existing bench unit-test seam. Luna reports focused debug/release, full lane/builtins release, bench unit tests, formatting, lane/realtime/unfused policies and focused clippy passing. Retained logs are `/tmp/luna-429-focused-final.log`, `/tmp/luna-429-release-tests.log`, `/tmp/luna-429-bench-tests.log` and `/tmp/luna-429-clippy.log`. No benchmark was invoked. These are implementation-supplied results; Astra is reviewing the complete frozen arithmetic, public dispatch/state/recovery and repeated-render allocation obligations before source acceptance. No compiler/bank integration, runtime performance claim or artifact qualification is earned by this checkpoint.

## Astra attempt 1 verdict and Sol assignment

# Astra #429 Luna attempt 1 review

**FAIL at `40de16521bae514150fe6729e052866c5ab1899e`.** One bounded Sol revision is required to complete the frozen identity/eligibility evidence. Reviewed actual four-file source diff, full numbered contract/base clarification and named logs. No Cargo/tests/timing or repository/GitHub mutation performed.

Production follows the intended minimal design: input completes before fusion, including recovery; the prepared fader remains static; all relevant matrix countdowns must be zero; otherwise the entire call retains the original fader then matrix processing. The new kernel loads both inputs, applies sample*gain then exact mute clearing, evaluates original coefficient*sample products/addition and identity selection, and stores once per plane. No new unsafe, dependency, scratch, compiler/bank pairing or benchmark changes are present. No concrete production arithmetic defect was established.

## Accepted evidence and precise limits

The lane test compares against the correct independent gain/gain/matrix primitive sequence at f32/W4/W8, but only at9 frames, all matrix identity masks false and a single finite/subnormal input pattern. The two new public tests establish one settled branch entry and a ramp that completes inside an8-frame fallback call. They do not establish the rest of the frozen public behavior contract.

**Do not incorrectly report that real render zero-audit proof is absent.** The existing `all_render_workloads_arm_only_product_render_without_timing` test exercises actual FullChainFilters/IdentityChain/MatrixRamp render through `run_render_operation`, with warmups, two independently prepared rounds, scoped audit snapshots and no timer. It passed in the reported31-test bench suite. The added outside-render held/dropped allocation probe provides allocation/free liveness without changing global allocator mode. These are useful actual product/allocation evidence; retain them, explicitly link eligible/fallback workloads in the evidence record, and do not add a duplicate allocator framework/test solely because the new test only contains the positive probe.

## Bounded remaining requirements for Sol attempt2

1. **Complete the small lane matrix in the existing file.** Use frozen frames1,3,8,9,128 at all three widths; add true/mixed identity masks, asymmetric gain/mute cases, existing identity/unmuted padding conventions and processed-subslice boundary sentinels. Include separate finite, signed-zero, subnormal and nonfinite representatives rather than a corpus whose observable outputs all become NaN. Compare same-width old primitive bits under the existing canonical FP environment; do not invent cross-target NaN payload pins or temporal partial-vector tails for this AoSoA kernel.
2. **Make the named wrong-equation witnesses live.** Add representative inputs for mute-via-zero-multiplication (negative input -> wrong signed zero), overwriting left before computing right, and the frozen zero-seed/coefficient-recombination arithmetic alternative. Compute wrong and old results, first require they differ, then require DUT matches old bits. No exhaustive FMA survey or second corpus is needed.
3. **Actual full public-chain equivalence is missing.** Add one compact existing builtin test helper pairing independent prepared DUT and reference chains. DUT calls process_dual_mono; reference uses the three unchanged section APIs in order. Compare both output planes, complete report, input retained/recovery state and matrix readback after each representative call/reset. Cover enabled/asymmetric and disabled input sections, sanitized NaN/infinity, and the existing injected bad-state recovery seam before downstream cross-channel mixing. Existing bank/per-track section tests do not exercise the new combined public dispatch and cannot substitute.
4. **Finish the frozen eligibility sequence.** In the same compact public fixture exercise immediate zero-ramp target, active ramp crossing a block, mid-ramp retarget, whole-call fallback when the ramp ends inside the call, NEXT call becoming fused, and reset. Include input ramping while downstream matrix is settled. Keep the per-instance dispatch witness—no global race—and demonstrate it discriminates the actual selected path, not just output equality. Restore the old dispatch in a disposable focused control or otherwise provide the specified actual assertion evidence; no new permanent telemetry or generic mutation framework. Preserve partition-at-event and existing live fader-ramp tests; this child does not fuse live faders.
5. Correct the new kernel comment saying fader products have “coefficient before sample operand order”: the code and old gain primitive correctly use sample.mul(gain); coefficient-first refers to MATRIX products. Fix prose only, not correct arithmetic.

Record the actual focused results and the accepted existing zero-audit seam in the numbered spec. Retain source approach and useful current tests; complete these bounded groups in one coherent Sol pass within already allowed lane/builtin/bench test paths. Run affected debug/release and relevant policy/hygiene checks; no fullworkspace, targets/artifacts, runner or timing before focused acceptance. #430/#431 retain their separate integration/measurement obligations. Luna attempt1 is consumed; Sol gets attempt2 and only if needed final attempt3, then hard stop/rescope.

Root assigns Sol attempt 2 to the bounded arithmetic/public-chain/eligibility proof completion above. The actual existing non-timed render audit is accepted and must be reused; no duplicate allocation harness is required. Production arithmetic stays unchanged except the stated comment correction unless a concrete correctness defect is demonstrated. One coherent checkpoint and one Astra verdict precede broader qualification.

## Sol attempt 2 implementation evidence

Sol completed the bounded evidence revision without changing production arithmetic. The kernel
comment now states the actual frozen order: sample times gain for the fader and coefficient times
sample for matrix products.

The lane oracle now runs frames 1, 3, 8, 9 and 128 at f32, W4 and W8, with false, true and mixed
matrix identity masks, asymmetric gains and mutes, padding conventions, and sentinels around every
processed subslice. Finite, signed-zero, subnormal and nonfinite families remain separate. Three
live witnesses first prove the old primitive differs from mute-by-zero-multiplication,
left-overwrite-before-right, and zero-seeded matrix recombination, then prove the fused DUT equals
the old primitive bits.

The public fixture pairs an independent DUT and three-section reference chain and compares both
planes, the complete report, retained input state and matrix readback after every call and reset.
It covers enabled asymmetric filters/faders/matrix, disabled input sections, NaN/infinity input
sanitization, injected nonfinite input state recovery before crossmix, zero-window retarget,
cross-block ramp, mid-ramp retarget, a ramp ending inside a whole fallback call, fusion on the next
call, input trim ramping under a settled matrix, and reset. Its per-instance dispatch counter
asserts each selected path. A disposable old-three-section-dispatch control is a passing
`should_panic` test because the same fused-path assertion fails under that dispatch.

Focused results, all PASS:

- `cargo test -p lane -p builtins` (includes retained fader-ramp, matrix and partition gates);
  log `/tmp/sol-429-focused-debug.log`.
- `cargo test --release -p lane -p builtins`; log
  `/tmp/sol-429-focused-release.log`.
- `cargo test -p bench --bin bench builtins::tests::`; 10 passed, including
  `all_render_workloads_arm_only_product_render_without_timing` over real
  FullChainFilters/IdentityChain/MatrixRamp renders and
  `audited_allocator_is_live_outside_the_render_scope` with held allocation and drop;
  log `/tmp/sol-429-bench-debug.log`.
- Lane, builtins, realtime and unfused policy checks PASS; logs
  `/tmp/sol-429-{lane,builtins,realtime,unfused}-policy.log`.
- `cargo clippy -p lane -p builtins --tests -- -D warnings` exits zero; only the existing
  unreachable clippy-disallowed-method configuration notices appear; log
  `/tmp/sol-429-clippy.log`.
- `cargo fmt --all -- --check` PASS; log `/tmp/sol-429-fmt.log`.

No full workspace, target matrix, artifact qualification, runner, allocator framework, global
allocator mode change or timed benchmark was run. #430/#431/#436 remain separate.

## Astra attempt 2 verdict and final Sol assignment

# Astra #429 Sol attempt 2 review

**FAIL at `2ea360c9120db101c38ffaecd6d1be6e532de9e4`.** One final bounded Sol evidence revision is needed. Production arithmetic remains accepted in approach; no new production defect was identified. Read complete frozen spec, cumulative source and supplied focused logs; no tests/builds/timing or repository/GitHub mutations performed.

## Accepted completion

The new paired public-chain fixture invokes actual process_dual_mono against an independently prepared three-section reference, comparing both PCM planes, complete report, input state and matrix readback. Enabled asymmetric input, disabled filters, NaN/infinity sanitization and injected nonfinite filter state are exercised. The eligibility sequence covers immediate target, a ramp crossing calls, mid-ramp retarget, whole-call fallback when completion is inside that call, following-call fusion, input trim ramp with settled matrix and reset. These are meaningful additions and need no second fixture framework.

Lane tests now exercise f32/W4/W8 at all frozen frame lengths, false/true/mixed identity masks and independent finite, signed-zero, subnormal and nonfinite families, with prefix/suffix sentinels. This is no longer a vacuous all-NaN corpus. All three wrong alternatives first assert a different old result and then compare DUT against the unchanged primitives. The comment now correctly distinguishes sample*gain from coefficient*sample. Retained stage/ramp/partition tests were included in debug/release focused runs. The 10-test bench subset includes actual repeated product render zero-audit and positive allocator/free liveness; that accepted evidence remains sufficient.

## Three concrete remaining gaps

1. **The claimed actual dispatch mutation is not an actual mutation.** `old_dispatch_fails_the_actual_fused_path_assertion` calls process_reference directly and asserts its never-incremented counter is one under should_panic. It does not call a changed public process_dual_mono, does not run the existing assert_pair assertion against restored dispatch, and would still pass if the public method incorrectly counted fusion but used old separate processing. The spec's “disposable old-three-section-dispatch control” description overstates this permanent expected-panic test. Perform one disposable mutation replacing the eligible public branch with the old dispatch, including its natural absent fused counter update, and run the existing positive public fixture unchanged. Require failure at its `selected path witness` assertion, restore the source, rerun that focused fixture green, and retain commands/exact mutation/terminal statuses and assertion text. No generic mutation harness, new telemetry or committed mutant. Remove or accurately describe the redundant expected-panic example; it cannot earn this gate.

2. **Claimed partial-population padding is not present.** In compare_case, every lane has real generated audio and non-neutral fader parameters; even the all-identity matrix case continues to fade/mute those lanes. No case represents unused lanes with zero input, gain one, unmuted mask and identity matrix coefficients/mask. Add one small W4/W8 representative with actual active/holey lane population and neutral padding following the frozen convention; assert padded lanes remain the old expected words and active lanes match the old oracle. Existing sentinel/length/category matrix stays; no Cartesian expansion, malformed-slice cases or invented temporal tails.

3. **The requested canonical FP environment is not installed by the new lane tests.** Neither test enters the existing CanonicalFpEnv, while the prior verdict explicitly required the same canonical environment for the raw subnormal/nonfinite comparisons. Add the existing scoped guard to those entry points (no dependency or new environment API), retain separate categories and same-width old/new bits, and record only the actual platform's canonical denormal behavior. Do not add cross-target NaN payload assertions or require unflushed subnormal outputs on a flushing platform.

These are the final finite revision groups, not an invitation to expand runtime/fixture scope. Keep production equations and accepted public sequence/audit evidence unchanged. Run affected focused lane/builtin debug/release checks and the actual disposable dispatch control plus restoration, with policy/hygiene proportional to changes. Record accurate evidence in #429. No broader workspace/target/artifact or benchmark work before source acceptance. #430/#431 remain separate and RT-4 remains open.

Sol attempt 2 is consumed. Attempt 3 is the final coherent revision; a FAIL then requires preserved evidence and explicit rescope, never a fourth repair.

Root assigns FINAL Sol attempt 3 to these three finite proof corrections only. Accepted arithmetic, public-chain equivalence and allocation evidence remain intact. Any temporary public-dispatch mutation must be restored before checkpoint and final focused green run; no mutant is committed. A third FAIL triggers the hard stop and explicit rescope, not another repair under this series.

## Sol attempt 3 final evidence

The permanent evidence delta closes the two lane-test gaps. Each raw lane-test entry point now
holds `CanonicalFpEnv::enter()` across both the old primitive oracle and fused DUT. On this x86
host, all same-width finite, signed-zero, subnormal and nonfinite comparisons pass under that
canonical environment; this records no cross-target NaN payload or unflushed-subnormal claim.
`holey_populations_keep_neutral_wide_padding` adds one W4 and one W8 active/holey shape. Active
lanes carry nontrivial input/gain/mute/matrix words and match the old oracle bitwise. Every unused
lane carries zero input, gain one, unmuted masks and identity coefficients/mask, and is asserted
both against the old oracle and exact positive-zero padding output. The prior frame/category matrix
is unchanged. The redundant permanent expected-panic reference-only test was removed.

The actual public-dispatch mutation control used the existing positive
`full_public_chain_matches_the_three_section_reference` fixture unchanged. In the eligible branch
of `BuiltinChain::process_dual_mono`, the temporary patch replaced the test counter increment plus
`fader_matrix_block::<f32>(...)` call exactly with:

```rust
self.fader_mute.stage.process(left, right, frames);
self.matrix.stage.process(left, right, frames);
```

Thus the mutant was the real old public section dispatch and naturally omitted the fused counter
update. The authoritative command was
`cargo test -p builtins --lib tests::full_public_chain_matches_the_three_section_reference -- --exact`
with the issue target directory and toolchain PATH. It exited 101 at
`assertion left == right failed: selected path witness`, with `left: 0`, `right: 1`. The complete
failure transcript is `/tmp/sol-429-final-dispatch-mutant.log`; a second unpiped invocation
confirmed terminal status 101. The inverse source patch restored the exact fused branch. Running
the same command and unchanged fixture then exited 0 with one passed test; transcript
`/tmp/sol-429-final-dispatch-restored.log`. The final diff contains no mutant.

Final focused results:

- `cargo test -p lane -p builtins`: PASS, including the accepted public state/recovery/ramp
  fixtures and retained fader-ramp/partition gates; `/tmp/sol-429-final-focused-debug.log`.
- `cargo test --release -p lane -p builtins`: PASS;
  `/tmp/sol-429-final-focused-release.log`.
- After the last lint-only loop correction, the three-test lane file passes in debug and release;
  `/tmp/sol-429-final-lane-{debug,release}.log`.
- Lane, builtins, realtime and unfused policy checks: PASS;
  `/tmp/sol-429-final-{lane,builtins,realtime,unfused}-policy.log`.
- `cargo clippy -p lane -p builtins --tests -- -D warnings`: exit 0, with only the pre-existing
  unreachable disallowed-method configuration notices; `/tmp/sol-429-final-clippy.log`.
- `cargo fmt --all -- --check`: PASS; `/tmp/sol-429-final-fmt.log`.

Attempt 2's accepted ten-test non-timed bench proof remains unchanged: real
FullChainFilters/IdentityChain/MatrixRamp repeated render scopes report zero forbidden operations,
and the outside-render held/dropped allocation proves allocator allocation/free liveness. No
benchmark, timing, full workspace, target matrix or artifact qualification was run. Production
arithmetic is unchanged, and #430/#431 remain separate while RT-4/#349 stays open.
