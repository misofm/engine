# Specialize stationary limiter dispatch without changing DSP

Status: OPEN #539. Number/title/body synchronization precedes the baseline-only review. #537 is CLOSED through PR538/main32a4c2058f2c01dea78af32affab660dbc17cae0. Boundary inventory259 local numbered specs/353 remote issues found no missing local-to-remote identities and verified534/536/537 CLOSED. No implementation or lowering capture is authorized before numbered/current-base approval. Root owns Git/GitHub; Luna high/xhigh implements, Astra medium verifies audio correctness. Maximum three implementation attempts. No timing authority or speedup claim.

## Smallest closable outcome

Establish whether actual native limiter production lowering retains per-frame stationary conditionals in the dual/mono, uniform/per-lane bodies. Only if native baseline evidence demonstrates a useful residual and Astra accepts the narrowed premise, specialize the existing common bodies at one block-entry decision while retaining the ramping path and every DSP/state contract. Compiler-eliminated or unprovable residual is a no-change result, not permission to rewrite for a descriptive number. FX2 detector-chunk access is a separate successor; it does not belong here.

## Conditional scope and invariants

Only crates/true-peak-limiter/src/lib.rs (including existing private tests), narrowly needed tests/mono_collapse.rs, and tests/allocation.rs limited to existing-allocator liveness/focused path coverage, plus this issue/evidence. Existing stationary admission requires remaining=0 and bit-equal current/target; dual admission checks all four ramp/channel sets, mono checks the left pair under unchanged collapse eligibility. Keep one block-entry decision, no mid-block transition, persistent cache, per-lane dispatch, duplicated divergent DSP body or new public API. Preserve existing uniform/per-lane selection and all ramp snapshots/retargeting.

Keep detector_peak and annex2_phases history shift, center tap, four phases and twelve separately rounded multiply/add steps in original tap order and ordered maxima. Gain law, quantization, lookahead/window shape, fixed latency Fs/100+6, tail, bypass signed-zero bits, independent L/R history/phase/prefix state and the existing bank-shared main/ring cursor pair, active scratch prefixes and state codecs are unchanged. Preserve uniform segmentation, restore-desynchronized fallback, mono right-plane non-access and disengage copy, silent-rest admission/automation invalidation, nonfinite recovery and rejection ordering. No detector-chunk/ring access rewrite, resources, dependencies, backend policy, corpus/reference/pin change or generic harness. Artifact delivery pin changes require a separate bounded decision if the ordinary builder demonstrates an expected mismatch.

## Frozen finite sequence

1. Confirm actual numbered GitHub title/body/current default base and issue boundary. Capture one untimed native release production baseline with pinned toolchain, x86-64-v3 AVX2/FMA, existing release LTO/codegen settings, --locked and isolated target. Reuse #537 capture methodology; retain actual argv/cwd/source/config/environment/stdout/stderr/status and complete mapped scalar/native W8 production IR/assembly with original hashes/ranges. Inspect all four stationary bodies separately and distinguish the already-existing silent-rest path. Do not infer machine branches from source if-statements or absence of an inline helper symbol. Root checkpoints/pushes, then Astra rules on surviving mechanism before edits.
2. Conditional compact implementation and bounded existing private witness. Compare populated PCM/full state against the old unspecialized behavior for ramping through a within-block ramp endpoint, then the next stationary block; cover both uniform/per-lane and dual/mono supported paths. Prove actual admitted branch use with an independent discriminating mechanism control, not counters recording intended classification. Freeze one fallback-only control, retain its expected failure on the same mechanism assertion while identity remains equal, restore byte-exactly and pass. No mutation campaign or new fixture framework. Root checkpoints each coherent green tranche before another.
3. Retain existing stationary-hoist/open-window/ramp-bit-identity, phase-order, uniform/mixed-lookahead/independent-LR/restore fallback, mono transition/desymmetrization and active-prefix regressions. Existing conformance, determinism/corpus pins, gain-law/f64, reset/rejection/nonfinite/silent-rest and allocation/free checks remain. Ensure actual supported W8 execution is identified rather than treating a skip as evidence. Extend existing own-thread allocation liveness only if necessary; no new allocator.
4. Frozen-source full affected suite and changed focused debug/release checks, strict affected Clippy/fmt and proportional existing policies. Comparable candidate native lowering must establish the intended access/dispatch mechanism while preserving math; actual supported production host-web scalar/simd128 lowering and existing immutable corpus remain portability/regression gates. No fresh numerical corpus, benchmark runner, exhaustive target matrix or listening campaign.

## Research and DSP authority

Dispatch-only: preserve the current source equations (lib.rs:11–25), not the superseded initial step/hold law. With N=Fs/100, T=N+6, R=N+1, L=round(lookahead_ms*Fs/1000), Wb=clamp(L+1,32,R), P=max(abs(h[6]),abs(four Annex2 phases)), r=(P>limit ? limit/P : 1), m the existing sliding minimum, m_q=floor(m*16384)/16384, s the Wb-term box average, d=max(1-s, existing lane multiply-add(c,(1-s)-previous_d,previous_d)), g=1-d, y=x[n-T]*g. Keep all rounding/order/quantization and native/base-Wasm arithmetic policies. Quantized sums remain exact within launch R<=961; no reassociation or reciprocal substitution.

Current lib.rs:769–783 derives limit=10^((ceiling_db-1)/20) and c=1-exp(-1/(0.001*release_ms*Fs)) in f64 through math, rounded once to f32 at the existing parameter-update seam. Ceiling -24..0 dBTP-est (default-1) and release10..2000ms (default100) use existing64-update linear-domain ramps with exact endpoints/retargeting. Preparation-only lookahead0..10ms (default5) and its rounding/clamp remain unchanged. Launch rates exactly44.1/48/88.2/96kHz, latencyT and infinite tail. Preserve lane::flush on recursive d, existing effect-runtime block nonfinite boundary recovery and validation, not the superseded original per-value recovery.

Exact inherited research/decision authority: .github/ISSUE_SPECS/BRIEFS/016-true-peak-limiter.md, especially the issue90 wave2 amendment (gain-law/coefficient/sanitation sections around292–403); .github/ISSUE_SPECS/090-audit-miso-engine-true-peak-limiter.md; current lib.rs and docs/EFFECT_CONTRACT_V1.md. The original016 sections are historical where that amendment/current source supersede them. ITU-BS1770-5 supplies only the detector coefficient table/estimator; REISS-COMP supports detector/gain separation and one-pole timing; SMITH-SASP supplies signal-processing context as recorded in the research corpus. Existing fixtures include phase-order, independent f64 gain-law/ceiling, exact width/partition/state, and zero-allocation/free gates. Historical cost measurements and audition/listening records in016/049 remain historical: no timed invocation or new human listening occurs under this dispatch slice, and no new sound-quality claim is made.

## Delivery

After consolidated Astra source PASS, qualify the actual native ABI and ordinary published artifact with existing static/resources/hermetic/three-browser obligations. Get exact-head/current-base Astra review and actual required qualification SUCCESS, assert live main immediately before exact-head merge, verify parents, then synchronize GitHub closure and audit349/518. No task is reported delivered before those remote steps. All failed attempts/captures remain candid; after three failed attempts stop and rescope, never perform a fourth disguised retry.

## Numbered/current-base baseline approval — Astra medium

**PASS — baseline-only scope review for #539.** One untimed native baseline capture is authorized. No implementation, timing, or projected gain is authorized.

Verified read-only:

- Clean, pushed HEAD `918efad74e90b7b2d2786359886765e1ed9a9d74`, directly parented by live default-branch main `32a4c2058f2c01dea78af32affab660dbc17cae0`. Production source/configuration is unchanged from that base. The local `main` ref is stale; use the verified remote SHA.
- [GitHub #539](https://github.com/misofm/engine/issues/539) is OPEN, titled **“Specialize stationary limiter dispatch without changing DSP”**, with a byte-exact match to the numbered local body.
- [#537](https://github.com/misofm/engine/issues/537) is CLOSED; [PR #538](https://github.com/misofm/engine/pull/538) is MERGED with the stated merge SHA and expected parents. Run `34088692973` and its `qualification` verdict succeeded. Post-main run `34089134766` remained in progress at the final check.
- Current inventory is **260 numbered local specs / 354 remote issues**, with no missing local-number identities; #534/#536/#537 are CLOSED. The recorded 259/353 inventory predates #539. This verifies number coverage, not universal historical title/body equality.
- Read applicable root AGENTS, the numbered brief, retained unnumbered prebrief, current limiter seams, inherited #016 wave-2 amendment/#090 pointer, contract, and #537 capture methodology. User routing overrides the guide’s older model assignments: Luna high/xhigh implementation; Astra medium audio verification.

The FX1 scope is appropriately small and conditional. Source conditionals occur at `lib.rs:1701`, `1890`, `3086`, and `3197`: dual per-lane/uniform and mono per-lane/uniform. Stationary admission requires zero remaining updates and bit-equal current/target; dual checks four ramp sets, mono the left pair under existing collapse eligibility. Existing silent-rest admission is a separate mechanism. **None of these source facts proves a surviving machine branch.**

The inherited DSP contract is sufficient for a dispatch-only change: preserve the current windowed, quantized gain law; coefficient design and exact ramp endpoints; separately rounded Annex-2 tap arithmetic; ordered maxima; latency, signed-zero bypass, state restoration, mono disengagement, and nonfinite recovery. The superseded step/hold law supplies no authority. FX2 detector access remains a separate successor.

Record these narrow clarifications with the baseline instructions:

1. **Caller coverage is not an eight-cell requirement.** Capture scalar production `process` and supported W8 `process_bank` / `process_bank_mono`, including any outlined callees. Map all four bodies through W8. Scalar is dual-only and its valid one-lane state is uniform; do not manufacture scalar mono/per-lane probes or credit unreachable code.
2. **Correct the cursor wording:** L/R history, prefix and phase state are independent; the current engine shares one main/ring cursor pair across the bank. Preserve that arrangement.
3. **Preserve effective compiler provenance.** Reuse the existing capture approach with Rust `1.97.1`, `--locked --release`, isolated target, fat LTO/codegen-units=1 and existing `+avx2,+fma` configuration. Record effective environment overrides and compiler invocation; do not add `target-cpu`. Retain complete mapped IR/ASM and original identities/ranges. Distinguish crate production lowering from a final linked native artifact.
4. The brief conditionally permits allocator-liveness work but omits `tests/allocation.rs` from its path list. Resolve that narrowly **only if implementation proceeds**; the existing allocator is already thread-local. No allocator rewrite is warranted.

The finite conditional gates are feasible using existing private and integration machinery: populated PCM/full-state comparison with the old unspecialized behavior, a ramp ending within a block followed by a stationary block, all four supported routes, and one discriminating fallback-only negative control. Classification counters alone are insufficient. Preserve existing arithmetic, state, mono, allocation and corpus gates; identify actual W8 execution. Later native/Wasm and ordinary artifact qualification remain conditional delivery obligations.

After the **single capture**, root checkpoints and pushes the evidence; Astra then decides whether a useful, reachable residual survives. Partial survival narrows the implementation. Compiler-eliminated or unprovable residual yields **no change**, not another optimization premise. Maximum **three implementation attempts** remains binding.

No edits, builds, tests, benchmarks, captures, Git/GitHub writes, agents, or report-file writes were performed.

Root adopts all four baseline clarifications above. Capture actual scalar process and W8 process_bank/process_bank_mono including outlined callees; map all four bodies through supported W8, never invent scalar mono/per-lane probes. Capture is crate production lowering, not a final linked native artifact. If implementation proceeds, explicitly resolve any narrowly needed tests/allocation.rs liveness extension before editing; no allocator rewrite. Post-main34089134766 has now completedSUCCESS.

## Single unchanged native baseline checkpoint — Luna xhigh

One cargo rustc --locked --release -p true-peak-limiter --lib -- --emit=asm,llvm-ir capture returned0 atc97c1944. Source SHA d945d98c8799457f7567485b63daf87aa78e9eb52ff7174f7678fbbc77984689 remains unchanged. Metadata retains actual cargo argv, toolchain/config/source and selected environment; it is not a captured verbose rustc invocation. Ten complete scalar/W8 caller/core IR/ASM excerpts and full-output identities are retained. Root verified every selected byte range/size/hash.

Worker preparation was interrupted after the successful capture/extraction without a final branch-mapping report. No second capture or source edit ran. No stationary residual is claimed by this checkpoint; Astra medium must independently map the retained compiler dataflow and decide whether a useful reachable residual survives, including effective provenance limits. The finite baseline gate and no-change option remain unchanged.

## Baseline residual decision and attempt 1 authority — Astra medium

**PASS — baseline sufficient; a reachable FX1 residual survives in all four W8 bodies. Luna high/xhigh attempt 1 is authorized within the bounded scope below.** This establishes retained dispatch work, not a measured or projected speedup.

Verified clean, pushed HEAD `a57c8a7f7f89ef51c9d188aa6e4f1f711360cd3d` against live main `32a4c2058f2c01dea78af32affab660dbc17cae0`. [#539](https://github.com/misofm/engine/issues/539) remains OPEN with exact title/body synchronization. Source and captured configuration hashes match; no production changes separate this checkpoint from the base. Post-main qualification `34089134766` completed SUCCESS.

I independently verified both full original compiler-output hashes and all ten selected intervals, sizes and hashes. The successful capture is at `c97c1944`, status 0. The interrupted worker report supplies no mapping evidence; the findings below come from the retained compiler output.

**Provenance is sufficient for this bounded decision.** Captured Cargo argv, Rust `1.97.1`/LLVM `22.1.6`, configuration and source identities are consistent. The retained Cargo fingerprint records `["-C","target-feature=+avx2,+fma"]`; actual function attributes include AVX2/FMA, and the module identifies the pinned compiler and optimized compilation. Its profile fingerprint matches the retained #537 baseline/candidate captures under the unchanged release configuration.

The environment record is selective and there is no verbose rustc invocation. Fat LTO/codegen-units=1 are recorded configuration, not proof that these crate-emitted excerpts are final linked, post-LTO native code. That limitation does not invalidate this previously authorized crate-production baseline. No extracted object is credited and no replacement capture is needed.

The production caller chain is explicit: scalar `process` calls scalar `LimiterCore::process_block`; W8 `process_bank` calls W8 `LimiterCore::process_block`; W8 `process_bank_mono` contains the mono bodies.

All line numbers below are **selected-file lines** in [the retained package](/home/bl/misofm/engine-limiter-stationary-dispatch/artifacts/issue539-baseline/selected/manifest.json).

| Supported W8 body | Stationary test and taken target | Frame-loop backedge |
|---|---|---|
| Dual per-lane, source 1701 | `process-core-w8.s:1888`: `cmpb $0, 256(%rsp)`; `jne .LBB32_98` | Line 3224 returns to `.LBB32_96`, before the test |
| Dual uniform, source 1890 | `process-core-w8.s:5125`: `cmpb $0, 2304(%rsp)`; `jne .LBB32_315` | Line 5720 returns to `.LBB32_313`, before the test |
| Mono per-lane, source 3086 | `process-bank-mono-w8.s:1429`: `cmpb $0, 48(%rsp)`; `jne .LBB60_88` | Line 2077 returns to `.LBB60_86`, before the test |
| Mono uniform, source 3197 | `process-bank-mono-w8.s:3037`: `cmpb $0, 8(%rsp)`; `jne .LBB60_178` | Line 3335 returns to `.LBB60_176`, before the test |

These are stationary flags, not attribution based solely on debug locations:

- Their definitions test each ramp’s `remaining` word at offset 12 against zero and compare current/target as integer words at offsets 0/4. Dual combines four ramp sets; mono combines two.
- Successful admission writes the true flag before entering the frame walk. IR carries those invariant predicates to the corresponding branches at W8-core lines **1158/3310** and W8-mono lines **786/1996**.
- In each W8 case, the stationary jump bypasses decrement/max/compare/add/select ramp updates and rejoins the shared DSP frame body. That body’s backedge revisits the same flag test.
- Uniform admission checks lane shapes and phases, on both channels for dual and the live channel for mono. Valid mixed lookahead or restored phase differences reach per-lane fallback while coefficients can remain stationary.
- Positive signal with stationary coefficients reaches these loops without taking silent-rest bypass. Silent-rest has separate input/state admission and cursor/phase advancement.

Consequently, compiler unswitching has **not removed these four per-frame stationary tests** in this capture. Existing stationary handling already avoids ramp arithmetic; #539 must claim only removal of the repeated dispatch test, not rediscover that earlier saving.

Scalar dual/uniform also retains the test: `process-core-scalar.s:5183` compares `992(%rsp)`; `je .LBB34_287` enters ramp handling, and line 5783 returns to `.LBB34_283`. The emitted scalar per-lane body is not credited as supported one-lane execution. There is no scalar mono claim.

**Bounded attempt-1 authority**

After root records this decision, Luna may specialize the four existing common bodies using the existing block-entry stationary decision. Preserve uniform/per-lane selection, ramping behavior for the entire block, and all frozen DSP/state contracts. No mid-block transition, cached eligibility, per-lane dispatch, hand-maintained divergent DSP copies, FX2 access rewrite, or new public surface.

Required mechanism/state gates:

1. **Old-behavior oracle:** retain the original runtime-predicate behavior in a bounded private test path. Compare populated PCM and complete state for stationary blocks, a ramp ending inside a block, and the following stationary block across all four W8 routes and scalar dual/uniform. Preserve ramp `current`, `target`, `step` and `remaining` exactly. An always-advance oracle is not automatically equivalent to the original stationary path.
2. **Actual-path control:** observe execution inside the selected specialization, on the witness’s test thread—not classification before dispatch. Freeze one fallback-only control that routes through the old unspecialized behavior. Identity assertions must pass before the same positive specialization assertion fails. Retain that failure, restore byte-exactly, verify focused green, then pause for root checkpoint.
3. **Mono comparison:** compare candidate mono state against old mono behavior, including right-plane non-access. Retain dual equivalence and full-state comparison at the existing disengagement/desymmetrization boundary.
4. **Candidate lowering:** under comparable settings, demonstrate block-entry specialization and removal of these repeated stationary tests from the admitted frame loops. Preserve the ramping path and arithmetic order. Test instrumentation alone cannot establish the machine-code mechanism.
5. Retain the numbered brief’s finite existing correctness, realtime, native/Wasm and delivery gates. Explicitly identify supported W8 execution; skips are not evidence.

Resolve the allocation-path ambiguity narrowly: `crates/true-peak-limiter/tests/allocation.rs` may receive only existing-allocator positive liveness controls and focused coverage needed for the changed paths. Reuse its thread-local allocator; no new allocator or framework.

No implementation attempt has started; this authorizes **attempt 1 of at most three**. No timing, projected gain, FX2 work, finding-status change, delivery acceptance or issue closure follows from this decision.

Review was read-only: no edits, builds, tests, benchmarks, captures, Git/GitHub writes, agents or report-file writes.

## Attempt 1 first coherent source/witness checkpoint — Luna high

Only lib.rs changed: common-source stationary/ramping specialization retains original runtime-predicate behavior for the private oracle. Focused tests::stationary_dispatch_matches_runtime_oracle_and_observes_selected_body passed1/0/29filtered at sourceSHA5b25b0337de2106c58d48edd71dac0332eedce61322fa4ee0988e729defce28d, blob5b21199dc5df1a477407e7daa1613646702c5de0. Witness covers allfourW8routes and scalardualuniform, populatedPCM/fullstate/ramp and monorightplane. Initial compile failure101 and corrected green captures are preserved.

Raw helper/target paths inherited luna2 in their names; this remains attempt1, not an attempt reset. Root checkpoints before the single frozen fallback control; mutation, broader gates, candidate lowering and consolidated Astra review remain pending. No source acceptance or speedup claim.

## Attempt 1 discriminating fallback control

One frozen mutation changes exactly four stationary callsite const arguments to DISPATCH_RUNTIME, leaving oracle/observation/assertions/arithmetic untouched. MutantSHA768f1aba29990fdfa92d477710b1746911d5be237c20dd273a6eeb84513a1465; exactpatchSHAf9e2f32a573d8c2e705aeb16222d0c972c7684820434b1ff9ea0c78fd2c719b1. Focused mutant returned101 at W8 dual per-lane selected specialization block1, observedRuntime versusexpectedStationary, after left/rightPCM, complete-state and silent-state identities passed. Later routecases are not credited as executed by that stopped mutant.

Source was restored byte-exactly to5b25b0337de2106c58d48edd71dac0332eedce61322fa4ee0988e729defce28d. The first restored invocation returned101 while Cargo reused the prior mutant binary after timestamp-preserving restoration (no compile in stderr); it is retained candidly. Refreshing only source mtime forced a recompile, and the corrective restored selector passed1/0/29filtered,status0. No second mutant ran and no assertion changed. Pristine/mutant source copies remain in /tmp/issue539-luna2; Git preserves pristine386e4c36 plus exactpatch. Remaining finite gates/candidate lowering/consolidated review are pending. This remains attempt1.

## Attempt 1 final source / allocator checkpoint — Luna high

Existing thread-local allocator now proves own-thread allocate/free liveness and measures prepared mono bank rendering in addition to existing paths. Allocation and mono-collapse binaries each passed2tests,status0, with actual nativeW8 execution. Existing mono_collapse.rs remained unchanged. Strict affected alltarget/allfeatureClippy and cargo fmt passed; original Clippy101 is retained. The only lib.rs follow-up changes are cfg(test) on observationenum and a narrow too_many_arguments allowance on the existing-shape private oracle, with no DSP change.

Frozen source: lib.rs SHA08ea2bfaa5eb83e996e61bced7c9de5e64f5a4a23b932716ce92dbbd7be09d90/blobdda39328b3bdf2ca3352f4ba57b4e7e22318cb02; tests/allocation.rs SHA7825d30723192724a0cab7165953eeb00860e5dad66dc6b7d74333d1ca0ea920/blobf61cefe50807e4994388f50c9c92c357389d0d44. Updated helper captures both paths; prior helper retained separately. Rawlabels attempt1-luna3 denote tranche3 within attempt1, not a reset. Fullsuite/release/policy/corpus/candidate lowering/consolidated review remain pending.

## Attempt 1 frozen-source final gates and candidate lowering

Full affected suite42passed/0failed/1descriptiveignored. Privateoldbehavior/path release selector passed1; releaseallocation/monobinaries eachpassed2. Existing native/scalarWasm/simd128 corpus passed139cases/349comparisons/0mismatch each with unchangedpins. Realtime/lane/workspace/environmentvocabulary policies passed. Root initially invoked nonexistent check-env-policy.sh; actualbashchild returned127, retained; corrected existing check-env-vocabulary.sh returned0. No policy/source change fixed that command-name error. No ignoredbenchmark was run.

Comparable native crate release and actuallinked scalar/simd128host-web builds passed. Thirty-two complete native root/callee IR/ASM intervals and eight complete supportedWasm wrapper/core bodies are retained with originalsizes/hashes and exactcommand/status. W8mono nowoutlines into process_bank_inner; rootincludedthatcallee and allreachabledefinedlimiterhelpers ratherthancreditingthewrapperalone. All selectedintervals were independently verified. The candidateclassification/loopmechanism and code-size tradeoff still require consolidatedAstra medium attempt1 review; no speedup or sourceacceptance yet. Source remains frozen615787e9.

User nowrequires completedmergedlocalworktreesremoved afterrequiredsync with clean/pushedevidencepreserved; apply this to539atdelivery. Independent documentation/cleanupissue540 records nine oldworktreesremoved/~31.8GiB reclaimed. Active539worktree and allrawcandidateevidence remain.
