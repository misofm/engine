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
