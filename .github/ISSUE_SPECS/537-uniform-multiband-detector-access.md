# Specialize uniform multiband detector access without changing DSP

Status: OPEN #537. Astra medium attempt2 source PASS and all local artifact/ABI/browser gates passed. Exact-head/current-base review and required CI/merge remain pending. Attempt1 FAIL remains counted. No measured or projected speedup claim.

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

One native release capture passed at69acb3cb with unchanged multiband source SHA1058d6043d38c80f5ee3f61e794b5e35a8a4b35bf447661ed077c46739879779, pinned Rust1.97.1/LLVM22.1.6, fatLTO/codegen1/debug1 and configured AVX2/FMA. Full metadata/output identities and six historically complete scalar/W4/W8 compiler extract ranges are recorded under artifacts/issue537-baseline; issue #625 removed the selected LLVM payloads from current main while the selected assembly remains. Root independently verified all selected identities against exact original line ranges, sizes and hashes.

The report identifies surviving per-lane wrapped addressing/checks and scalar-load/pin assembly in native W8 (selected ASM6246–6566 and6673–6752), with analogous W4 emitted-only evidence. Scalar W1 naturally has no cross-lane packing opportunity. Source arrays/debug locations alone are not credited; no machine-address or runtime speedup claim is made. No source rewrite has occurred. Bounded Astra review of surviving work is required before implementation.

## Bounded baseline decision and attempt 1 authority — Astra medium

**PASS — bounded baseline decision: authorize Luna xhigh attempt 1’s first compact tranche.**

Verified clean, pushed `ce3fb612` against live main `375a86c`; #537 is OPEN with exact matching title/body. Source SHA matches `1058d604…39879779`. Recorded original LLVM/ASM sizes and hashes match; capture status is 0 under the pinned release configuration. Post-main `34083008320` is **SUCCESS**.

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

## Attempt 1 failed mechanism gate — implementation paused

The single frozen mutation changed actual uniform access `if uniform` to `if false`, but the fully-qualified fixture still passed1/0,status0. This is a FAILED negative-control gate, not evidence of correct mechanism coverage. Root identified that the test-only counter increments from classification before the branch, so it records intent rather than executed access. The passing mutant source SHA13ca418cc782a7ad47e154969f55e4d154022dbac1c99e87fc56be40a7bab6df and exact patch/command/output/status are retained. No assertion was weakened.

Luna prepared a second observability patch but did not execute it; its explicitly unrun patch is retained separately and is not accepted implementation or mutation evidence. Root stopped further source/public-fixture work. Original source was restored byte-for-byte to SHA dbdc6847ab29ebeafeb84e04752559b0b95936c30d7170764464db43d88c1932, and the one actual restored focused invocation passed1/0,status0. A prior restored invocation used a nonexistent worktree path and started no command/capture. No second mutation ran.

The useful source checkpoint remains buildable, but this attempt lacks the required discriminating actual-route control and remaining public transition/allocation/candidate qualification. One consolidated Astra medium attempt1 verdict is now required before a revision. Do not relabel a further mutation as the original frozen run or reset the attempt counter.

## Consolidated attempt 1 verdict and bounded attempt 2 — Astra medium

**FAIL — consolidated attempt-1 verdict. Attempt 2 is authorized within #537.**

Verified clean, pushed `c73e4b56`, live base `375a86c`, and OPEN #537 with exact title/body identity. Restored source matches `dbdc6847…88c1932`.

The frozen negative control failed: forcing actual uniform access to fallback still produced **1 passed, status 0**. [The counter records classification before executing the branch](/home/bl/misofm/engine-multiband-detector-access/crates/multiband-compressor/src/lib.rs:900), so it cannot prove uniform access occurred. Plausible arithmetic does not satisfy this gate.

Additional bounded corrections:

- Global counters can receive calls from concurrently running `split` tests; isolate observation to the witness’s test thread.
- Corrected words distinguish rows/lanes, but the single-ring oracle and count-only render witness do not establish distinct band/channel dataflow. Add distinguishable band/channel words and check actual accessed words against the old-index oracle.

The production diff retains safe loads, original ragged indexing, width-bounded transient classification and unchanged DSP/state layout. This does not establish acceptance.

**Attempt 2 — Luna xhigh:**

1. Correct executed-branch observability first, retaining W1/W4/W8 and mixed-channel coverage. Freeze the corrected candidate; run one fallback-only control that fails the **same mechanism assertion** while old-word equality passes. Restore byte-exactly, verify focused green, then pause for root checkpoint/push.
2. Complete frozen populated PCM/state/report transitions, reset/bypass, allocation/free and own-thread liveness gates; then candidate native/Wasm lowering and final checks.

Preserve the zero-selected capture, periodic-oracle correction, passing mutant and restored pass. The prepared second patch remains **unexecuted**, not evidence. No baseline redo, new framework/corpus, gate weakening or performance claim. Maximum **three total attempts**.

## Attempt 2 first coherent observability checkpoint — Luna xhigh

Test-only observation now uses fixed-capacity thread-local storage, records values inside the actual selected access arm, and compares actual returned words against independently indexed copies of four distinguishable channel/band rings. Existing W1/W4/W8 private wrap/offset oracle and actual mixed-direction callsite cases remain. Corrected fully-qualified focused invocation passed1/0,status0 at sourceSHA ad34dd9adfc1753fb4db3a6bbc96d94a9c3f5c3a836bfa5f8f3853fb4d345331. The prior status101 fixture compile diagnostic (missing const-generic annotation) is retained; no production algorithm change was made to repair it. Fresh frozen attempt2 mutation and remaining public/candidate gates are pending.

## Attempt 2 discriminating mechanism control

Assertions were reordered without changing their contents so the old-index/actual accessed-word checks precede the same positive executed-route assertion. Frozen source SHAaf2776551abe0abc08a4bd80fbddfb6e461cfc9d64f1567dbbb6efa54e1121fa. The one fresh attempt2 mutation changes only `if uniform` to `if false`; it returned101 after old-word and first actual-callsite word equality completed, failing `prepared uniform channels: executed uniform calls` at0 versus12. The source was restored byte-for-byte and the same fully-qualified focused test passed1/0,status0. Full mutation/restoration captures and exact patch are retained. No second attempt2 mutation ran. Public/candidate gates and consolidated attempt2 review remain pending.

## Attempt 2 public identity checkpoint

Existing identity runners now accept directed offset profiles while retaining the original varied-values fixture. Uniform, unequal-channel uniform (0/20ms), and ragged-lane profiles compare exact scalar/W4/W8 PCM, full snapshots and reports across all three links for1536frames, including first-block parameter ramps and settled post-latency output. Assertions require populated output/ring payloads. Focused debug identity profile test passed1/0,status0 at identity.rs SHA4c59f1743fec93a6c7477b240957e1daae5d9fa0ae930e0f0c7bc95db309010f. No production source changed. Dedicated mixed uniform/ragged channel profiles, populated restore transitions, reset/bypass and later gates are still pending.

## Attempt 2 populated public transition checkpoint

Dedicated uniform-left/ragged-right and ragged-left/uniform-right public profiles now join the exact scalar/W4/W8 × three-link identity comparisons. Existing restoration machinery exercises populated1536-frame donors and warmed receivers in both profile directions, followed by4096frames through cursor wrap and full-reset comparison with a fresh destination baseline. Ring/output population checks use numeric nonzero values; unchanged bypass regression remains part of the final suite.

Expanded-profile and transition selectors each passed1/0 in debug and release (four successful invocations), at identity.rs SHAa45d7921282dde5f9a562e7d982b9b3a88db7e9bbffbac1d9ff81fe9864fba24. The first compile diagnostic from a missing restore import is preserved as status101; no production or expectation change repaired it. Allocation/liveness, final suite/format/Clippy, candidate lowering and consolidated review remain pending.

## Attempt 2 final implementation / allocation checkpoint

The existing thread-local TrackingAllocator now has distinct own-thread positive allocate/free controls outside realtime windows, and prepared W4/W8 uniform/ragged/mixed bank render requires zero events. Original scalar/bank reset/restore checks remain. Focused allocation binary passed3/0/0filtered. Strict affected all-target/all-feature Clippy passed after two retained status101 lint diagnostics (needless borrow/type complexity); fixes were test-only annotations/borrows and formatting, not DSP or gate changes. cargo fmt was applied.

Frozen source hashes: lib.rs efd7523a19b5355b53cdd8620a0abc75161a7d39440d54c286c288e5512e09f2; identity.rs e27bb19cb841f7101bf7a8c31d23f500021bf93dd6f4b564828bdeeaa7dee526; no_alloc_render.rs 04033e9535455d151de254f69101a3062b45e9e5cee87479b5e7763bf4e6be4e. Full suite, final release checks, existing corpus, candidate native/production Wasm lowering and consolidated review remain pending; no acceptance or speedup claim.

## Attempt 2 frozen-source final gates and lowering package

All remaining local gates passed on source13d0580b: full suite42passed/0failed/2descriptiveignored; private oracle release1pass; full identity release6pass; allocation release3pass; format and four existing policy checks. The existing native/scalar-Wasm/simd128 corpus run returned139cases/349comparisons/0mismatches per leg, with immutable pins. It is arithmetic regression evidence only, not multiband ring/tap-route evidence. No ignored benchmark was run.

A direct attempt to execute non-executable check-realtime-policy.sh failed before spawning a child (implementation agent reported PermissionError13); its metadata/empty streams and absent child status are preserved. The canonical bash invocation passed separately. No numeric child exit status is invented for a process that never started.

Root captured comparable native release compiler outputs and both linked host-web scalar/simd128 release modules successfully. Complete selected native scalar/W4/W8 assembly bodies and actual Wasm scalar func761, SIMD-module scalar func811 and supported SIMD4 bank func813 remain with original-output identities and command/status records. The selected native LLVM payloads were historical and issue #625 removed them from current main; root verified every selected byte range/hash against its original decoder/compiler output and retains the original LLVM ranges in metadata. Emitted W8 Wasm/mono wrappers are not credited as supported dispatch; inspection modules are not published artifact identities.

Native W8 candidate selected ASM6317–6366 shows the uniform condition, safe row-length guards and contiguous vector loads bypassing the ragged arm at LBB40_176; original low/high index-CSE narrowing remains. These are inspection pointers, not consolidated acceptance or a net speedup claim. Candidate conditional access and preserved semantics await one consolidated Astra medium attempt2 review. Full original output identities remain at manifest paths under /tmp; selected LLVM payloads are removed from current main by issue #625, while surviving assembly remains; no source changed after13d0580b.

## Consolidated attempt 2 verdict — Astra medium

**PASS — consolidated adversarial attempt-2 verification for #537 (Astra medium).**

Reviewed clean, pushed `5c0c387501c05b9da382f1b60ec06a3113bf40b9` against live main `375a86c280ac5be83004dc78dc25541bfa6371ba`. GitHub #537 is OPEN with matching number, title and byte-exact body. Source remains frozen at `13d0580b`; all three supplied final hashes match. Attempt 1 remains FAIL and counted; this is Luna xhigh attempt 2, with no attempt relabeling.

No blocking finding remains within the frozen implementation scope:

- [Production access](/home/bl/misofm/engine-multiband-detector-access/crates/multiband-compressor/src/lib.rs:939) retains the old ragged indexing and safe lane load. [Classification](/home/bl/misofm/engine-multiband-detector-access/crates/multiband-compressor/src/lib.rs:1058) is transient, per channel, outside the frame loop, over the W offsets matching L::WIDTH; its result serves both bands. Ring writes precede detector reads. Arithmetic/link order, state/layout/resources, restore/reset logic, public API and dependencies are unchanged. Test instrumentation is cfg(test)-only.
- [Executed-arm observation](/home/bl/misofm/engine-multiband-detector-access/crates/multiband-compressor/src/lib.rs:916) now records returned words inside each arm using fixed-capacity thread-local storage. The independent old-index oracle distinguishes rows/lanes, including signed zeros; the actual render witness checks four separately seeded channel/band rings and both mixed directions. [Word assertions precede mechanism assertions](/home/bl/misofm/engine-multiband-detector-access/crates/multiband-compressor/src/lib.rs:2255). The sole attempt-2 fallback-only mutation reconstructs to captured SHA `c970cf23…f680861`: status101, expected executed-uniform count12 versus actual0, after word checks. Restored SHA `af277655…e1121fa` passed; later lib changes are formatting, not altered mechanism or assertions. Attempt-1 failure evidence remains retained.
- [Public identities](/home/bl/misofm/engine-multiband-detector-access/crates/multiband-compressor/tests/identity.rs:379) cover all three links and scalar/W4/W8, uniform/unequal/mixed profiles, ramps and settled rendering, exact PCM/full state/reports and populated rings. [Bidirectional populated restoration](/home/bl/misofm/engine-multiband-detector-access/crates/multiband-compressor/tests/identity.rs:668) continues through4096 frames; scalar full-reset comparison and existing bank reset/bypass regressions are retained. [Allocation liveness](/home/bl/misofm/engine-multiband-detector-access/crates/multiband-compressor/tests/no_alloc_render.rs:142) proves own-thread allocate/free detection; prepared scalar and uniform/ragged bank paths require zero events.
- Native W8 classification (selected ASM5912 onward) produces per-channel flags before the frame loop. ASM6317–6366 selects guarded contiguous loads from detector ring bases; LBB40_176 retains per-lane fallback. Supported host-web SIMD4 func813 independently compares offsets1264–1276 and2560–2572 before its loop (selected lines2228–2274); flags5/9 select detector arms. Wrapped row indexing feeds guarded v128 loads from low/high bases1280/1288 and2576/2584 (2701–2714,3011–3024); fallback retains scalar/lane insertion (2724–2779,3034 onward). These are detector loads, not coefficient loads. Scalar-module func761 and SIMD-module scalar func811 retain bounded single-lane detector reads using offsets372/708. W8 Wasm is not credited as supported dispatch.

Recorded final checks: full suite42 passed/0 failed/2 descriptive ignored; release private oracle1, identity6 and allocator3 passed; strict Clippy, format and four policy checks passed. Final capture source hashes match. All99 attempt-2 and40 lowering capture files match raw copies; native/Wasm selected ranges, sizes and hashes match originals, including linked module identities. The immutable corpus passed139 cases/349 comparisons per native/scalar/simd leg; it proves arithmetic regression only, not ring composition. No checks were rerun.

This PASS accepts the bounded source/evidence slice, not completed delivery. Published-artifact/browser qualification, exact PR head/current-base/required-CI review, merge and remote closure remain subsequent gates. Inspection modules are not published artifact identities. No speedup claim, benchmark claim or credit for already-CSE’d low/high index calculations is made. No repository edits, builds, tests, mutations, Git/GitHub writes or agents were performed; only this requested report was written outside the worktree.


## Bounded published artifact pin exception — Astra low

**APPROVE — bounded artifact-pin scope expansion for #537.**

Root may checkpoint this decision and authorize Luna xhigh to change only `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` from:

`e70a4311da3f053ea77fc0ec7ac00348d534dfb2919e05591fefed515988802d`

to:

`1bc18ab8cfb3e2a3e5a0ebeda185a64551e8398870abb2f3581077da0dfd3a3f`

Reviewed applicable AGENTS, numbered #537, consolidated attempt-2 PASS, actual detector source, delivery captures/capture script, ordinary builder, and the branch’s #534 ruling. HEAD is clean `e8c9c5f6`; production/build paths are unchanged since frozen `13d0580b`. Native shared/static ABI capture reports status 0. Ordinary artifact compilation completed, then exited 1 at the exact pin mismatch above.

The builder hashes its temporary module before comparison, exits before copying output on mismatch, and deletes its temporary directory through its exit trap. The output directory is empty. **The new hash is a recorded builder observation, not an independently rehashed retained artifact.** This matches the bounded #534 precedent.

Conditions:

1. Preserve original command metadata, source identities, stdout, stderr and status 1 as failure provenance; record this explicit exception to #537’s original pin prohibition.
2. Keep Luna’s edit limited to the exact pin replacement. No numerical, corpus, resource expectations, policy, CI, build-script or production-source changes.
3. Run the ordinary verified builder with no repin bypass on unchanged accepted source/build inputs. It must reproduce this hash and successfully emit the actual artifact; retain its identity and successful capture.
4. Complete existing static/object checks, resource checks with all 26 red controls, hermetic tests, actual Chromium/Firefox/WebKit qualification with mutations, and matrix verification. Qualification records must identify the actual candidate/output; do not relax matrix requirements.
5. Any further hash discrepancy or qualification failure requires concrete assessment, not another automatic repin or weakened gate.

Attempt-2 source PASS remains intact; attempt-1 FAIL remains counted. This ruling establishes neither published-artifact success nor speedup, completed delivery, PR/merge approval, or issue closure. Exact-head/current-base and required CI obligations remain.

Review was read-only: no edits, builds/tests, agents, Git/GitHub mutations, or report-file writes.

## Approved pin checkpoint — Luna xhigh

Luna applied only the approved exact hash replacement, preserving the newline. Original failure and bounded Astra low ruling remain retained. DSP/build source remains frozen. The ordinary verified builder and remaining delivery checks are pending; this checkpoint claims no artifact qualification.

## Local delivery qualification checkpoint

Ordinary verified builder reproduced pin1bc18ab8cfb3e2a3e5a0ebeda185a64551e8398870abb2f3581077da0dfd3a3f and emitted the actual2,728,956-byte module at candidate62366ba20cab5742aa6e0636f428ec18365104bf. Root independently hashed the retained output and recorded all six file identities. Native shared/static ABI, static/object checks, resource expectations with26 red controls, hermetic AudioWorklet tests, actual Chromium151.0.7922.34/Firefox153.0/WebKit26.5 with mutations, and matrix verification all returned0. Browser records identify that actual candidate/output. No accepted DSP/build-source change or corpus/resource/gate relaxation. PR exact-head/base/required CI and remote delivery remain pending.
