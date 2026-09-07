# Specialize uniform multiband detector access without changing DSP

Status: baseline-first DYN3 child of audit349/518, based on delivered main375a86c280ac5be83004dc78dc25541bfa6371ba after PR535. Root owns all Git/GitHub and checkpoint pushes. Luna high/xhigh implements; Astra medium briefs and reviews this audio-path change, with Astra low permitted for simple delivery checks. Maximum three implementation attempts, one consolidated adversarial verdict each. No timing authority or measured/projected speedup claim.

## Smallest closable outcome and prerequisite

Determine whether native production lowering retains useful scalar gather/packing work in multiband detector_tap when channel offsets are uniform. Only if baseline evidence and a bounded Astra decision support it, replace those uniform accesses with one contiguous row load, retaining original ragged access and every DSP/state contract. Native baseline inspection precedes any source edit. If the compiler already removes the claimed work, record no-change or narrowly rebrief the surviving residual; source temporary arrays do not prove stack traffic.

The previous gate/compressor issues do not deliver this effect. The multiband ring is slot-major B*W with B=Fs/50+1. Offset O=1+round(lookahead*Fs/1000) is clamped to [1,B]; row=wrap(cursor+O,B) using the existing compare/subtract. O=B reads the just-written row; O=1 selects output-delay row. Four calls read low/high for each of two channels, sharing each channel's offset array. Signed detector words are read before magnitude and DualMono/Maximum/Average linking. Preserve independent channel offsets, write-before-read, operation ordering and existing bounds checks. No cross-effect helper.

## Conditional implementation scope

Only crates/multiband-compressor/src/lib.rs, tests/identity.rs, tests/no_alloc_render.rs, and narrowly required tests/support/mod.rs, plus this spec/evidence. Classify exactly L::WIDTH offsets once per actual segment outside the frame loop, independently for each channel; reuse for its two bands. Width1 is naturally uniform. Uniform access derives the original wrapped row then safe L::load from its contiguous existing W words. Ragged access remains the old index traversal and load. Use a transient classification: no persistent cache, state/layout/ring change, resource change, unsafe access, new public API, allocation or per-frame equality scan. Restore/reset must affect the next actual segment. Ramped and settled paths share this seam; bypass remains unchanged.

No changes to split/shim/lane/effect-runtime math, Cargo/dependencies, corpus/pins, SIMD dispatch/target policy, CI/runner, cohort eligibility, staging or mono-collapse. Any necessary expansion must be ruled on before edits, not smuggled into this feature.

## Frozen bounded sequence and gates

1. After numbered/current-base approval, Luna captures one untimed native release production compiler output before editing. Use current pinned toolchain, x86-64-v3 AVX2/FMA, release LTO/codegen settings, isolated target, --locked; retain actual argv/cwd/source/config/toolchain/env/stdout/stderr/numeric status and original IR/assembly sizes/hashes. Select complete actual production scalar/W4/W8 callers as emitted, distinguish W4 emitted-only from native W8 dispatch, follow real instructions/branches/addresses rather than debug-location counts. Reuse issue534 capture methodology, no new framework. Root checkpoints/pushes baseline; Astra decides whether useful access work survives before implementation. No timed workload.
2. Conditional compact source tranche plus independent old-index oracle: W1/W4/W8, distinct lane/frame/band words including signed zeros, O=1/interior/B, wrap boundaries, unequal uniform channels, uniform/ragged and ragged/uniform channels. Test the actual prepared-callsite classification/access route, not only a standalone helper. One actual uniform-to-fallback mutation must fail the SAME positive mechanism assertion while old-word equality still agrees. Preserve exact mutation and failure, restore original source; no wider mutation campaign.
3. Extend existing public identity/state fixture narrowly for uniform and mixed offsets, three link modes, settled/ramped segments and populated restore transitions in both directions. Reuse existing 1536/4096-frame fixtures, render beyond latency where needed, and keep ring/state nonzero so continuation is discriminating. Preserve full reset and bypass regression. Do not create a second fixture corpus or assert undocumented mid-ramp restoration identity; compare appropriate equivalently restored baselines.
4. Reuse installed allocation/free audit and own-thread positive liveness; actual prepared uniform and ragged render must allocate/free zero. Setup/state serialization/destruction outside measurement. Existing PCM/state/latency/nonfinite/crossover/resource gates remain. Run full multiband suite once and changed focused cases debug/release, strict affected Clippy/fmt and proportional existing policy checks. No numerical tolerance/count/pin changes.
5. After source is frozen, capture comparable native lowering plus existing actual linked host-web scalar/simd128 production lowering as applicable. Existing multiband corpus explicitly excludes ring/tap composition: its unchanged digests remain regression checks, never evidence for the new route. Use current native/scalar-Wasm/simd128 corpus runner once with immutable pins. No new corpus/target matrix or false claim that the fallback wrapper is optimized production.

Pause each coherent compiling/focused-green tranche for root exact-path commit and push before more edits. Benchmarks are not needed to prove this access-only contract; any timing remains separately authorized weekly work. Baseline/candidate lowering is untimed mechanism evidence. Generic runner repair, extended targets and listening scheduling do not expand this slice. Three failed attempts requires honest stop/rescope, no fourth retry.

## Inherited DSP and research contract

This changes data access only. Keep current two-stage TPT LR4 split: first stage v1/lp1, all-pass ap=x-2*k*v1 with k=sqrt(2), second low=LP2(lp1), high=ap-low. Each band uses the existing Giannoulis-Massberg-Reiss fixed 6 dB knee gain curve, branching smoother and exact update order. Launch sample rates, prepared crossover/lookahead, existing 64-update parameter ramps, bounded numeric domains, coefficient conditioning, safe NaN/denormal/reset semantics, latency Fs/50 and infinite tail remain authoritative in current lib.rs, shim.rs, lane/effect-runtime and docs/EFFECT_CONTRACT_V1.md. No algebraic reassociation, coefficient redesign, sound-quality claim or algorithmic class-B ruling is authorized.

Standing equations/fixtures/objective/listening evidence and limitations are in issue018, issue051, issue094 and docs/rulings/multiband-ramping-split-boundary.md. Primary research authority remains REISS-COMP (Giannoulis, Massberg and Reiss, 2012), ORFANIDIS-ISP, SMITH-SASP and VST3-LATENCY as recorded by issue018; accepted builtin TPT/LR4 mapping evidence remains unchanged. No new listening session is claimed and no timing estimate is acceptance evidence.

## Delivery

The preceding boundary inventory found258 local numbered specs/352 remote issues, no missing identities, and verified534/536 CLOSED before this issue was created. Match this issue's actual GitHub number/title/body and clean base before any baseline capture. After consolidated Astra PASS and unchanged-source local delivery qualification (native/ABI/current artifact/browser obligations proportionate to changed binary), root obtains exact-head/current-base review and actual required qualification SUCCESS. Assert live main immediately before exact-head merge and verify actual parents. Synchronize closure only once accepted evidence is upstream. Post-main CI and audit349/518 continuation remain part of delivery. PR535 post-main34083008320 is currently running and is not claimed successful.

## Numbered/current-base approval — Astra medium

**PASS — baseline-only authorization for #537.**

- Verified clean, pushed `da550e463ea256a0e1c229eaba03398779419361`, directly based on live main `375a86c280ac5be83004dc78dc25541bfa6371ba`. Only the issue brief and retained evidence differ.
- GitHub #537 is **OPEN**, with matching number, title and byte-exact body. #534/#536 are **CLOSED**, delivered through merged PR535. Required run `34082558237` succeeded; post-main `34083008320` remains **in progress**.
- Current detector/callsite/ring/restore code supports the narrow boundary: independent channel offsets, four signed reads after ring writes, existing wrap semantics, and a shared ramped/settled segment seam. Transient classification avoids restore/reset cache invalidation.
- Proposed finite gates are adequate: independent old-index oracle, actual-callsite mechanism assertion and fallback mutation, populated bidirectional restore transitions, PCM/state identity, and allocation/free checks. The current allocator fixture lacks a positive liveness test; supply the brief’s required own-thread control within the allowed test file. Corpus digests remain arithmetic regression evidence only.

Next: Luna high/xhigh captures the unchanged native production baseline; root checkpoints/pushes it; Astra low/medium reviews surviving access work before any rewrite. If useful work does not survive lowering, record no-change or narrowly rebrief. Keep the three-attempt limit.

No implementation acceptance or speedup claim. No edits, builds, benchmarks, Git/GitHub writes, or agents were performed.

## Unchanged native baseline checkpoint — Luna xhigh

One native release capture passed at69acb3cb with unchanged multiband source SHA1058d6043d38c80f5ee3f61e794b5e35a8a4b35bf447661ed077c46739879779, pinned Rust1.97.1/LLVM22.1.6, fatLTO/codegen1/debug1 and configured AVX2/FMA. Full metadata/output identities and six complete scalar/W4/W8 compiler extracts are retained under artifacts/issue537-baseline. Root independently verified all selected files against exact original line ranges, sizes and hashes.

The report identifies surviving per-lane wrapped addressing/checks and scalar-load/pin assembly in native W8 (selected ASM6246–6566 and6673–6752), with analogous W4 emitted-only evidence. Scalar W1 naturally has no cross-lane packing opportunity. Source arrays/debug locations alone are not credited; no machine-address or runtime speedup claim is made. No source rewrite has occurred. Bounded Astra review of surviving work is required before implementation.

## Bounded baseline decision and attempt 1 authority — Astra medium

**PASS — bounded baseline decision: authorize Luna xhigh attempt 1’s first compact tranche.**

Verified clean, pushed `ce3fb612` against live main `375a86c`; #537 is OPEN with exact matching title/body. Source SHA matches `1058d604…39879779`. Retained original LLVM/ASM sizes and hashes match; capture status is 0 under the pinned release configuration. Post-main `34083008320` is **SUCCESS**.

Native W8 instructions confirm surviving per-lane offset loads, wrap/index calculations, bounds checks, and `vmovd`/`vpinsrd` packing with `vinserti128`. No uniform-offset arm bypasses this work. W4 corroborates the mechanism **as emitted-only evidence**; W1 offers no cross-lane packing opportunity.

**Narrowing:** low/high bands already reuse computed indices through compiler CSE. Do not credit eliminating duplicate wrap/address calculations across those bands, or infer temporary-array stack traffic.

Authorize only transient per-channel classification of exactly `L::WIDTH` offsets once per actual segment, safe contiguous row loads for uniform offsets, and original ragged fallback. Preserve arithmetic, state, resources, bounds safety and bypass.

First tranche: compact source change plus independent old-index oracle and actual-callsite witness, including the frozen fallback mutation. Pause immediately when compiling/focused-green for root’s exact-path checkpoint/push. Public transitions, allocation/liveness and candidate lowerings follow under the frozen spec; maximum three attempts.

This is no implementation acceptance or projected speedup claim.

## Attempt 1 compact source checkpoint — Luna xhigh

The sole changed source lib.rs classifies channel offsets once per segment, uses a safe contiguous row load for uniform offsets and preserves the old ragged gather. Private test-only route counters and an independent old-index word oracle cover W1/W4/W8, offsets1/interior/B, wrap, signed words, unequal uniform channels and both mixed directions, including an actual render callsite witness. Corrected fully-qualified focused selector executed1 test:1pass/0fail,status0. Source SHA3309e4ead47d586413d4d57b85ce1295fb11fc13c74881b96a8f0a7a3d7d5587.

The first status0 invocation selected0 tests/17filtered and is retained as compile/filter evidence only; it is not credited as a passing fixture. Both captures are in artifacts/issue537-luna-attempt1. This is a recoverable first checkpoint, not source acceptance. The one forced-fallback mutation, public populated transitions/allocation evidence, candidate lowering and final gates remain pending.

## Attempt 1 oracle discrimination correction

Root found the original eight-word periodic fixture made all W8 rows identical and W4 rows repeat, weakening wrapped-row discrimination. Luna xhigh replaced it with distinct row/lane bit patterns, explicit +0/-0 words and pairwise row-distinctness assertions. The actual prepared-callsite witness now covers both mixed channel directions. Same fully-qualified focused test passed1/0,status0 at sourcedbdc6847ab29ebeafeb84e04752559b0b95936c30d7170764464db43d88c1932. Prior capture is retained honestly, not credited for discriminating W8 wrap. No mutation or public/candidate qualification has run yet.
