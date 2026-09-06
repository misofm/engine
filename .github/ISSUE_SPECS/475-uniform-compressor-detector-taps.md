# 475: Specialize uniform compressor detector taps without changing DSP

Current state: source accepted through bounded child499; integrated delivery qualification active. The decision below supersedes historical scheduling records. Root owns Git/GitHub and exact-path checkpoints; no timing or benchmark capture is authorized.

Active Class-A product child of audit #349 DYN-1. Current implementation base is delivered main571dfc5b after #463 closed. Astra has approved the actual base below; root assigns fresh Luna1 under the Astra review and Sol2/3 fallback workflow. Earlier queued-base records are retained as history, not current scheduling authority.

## Current premise and duplication check

Reviewed immutable delivered main4a814f348136bc5ba1d77bd04388a3c7163a0e10 through engine-471-proof. Its compressor source remains the shape described in /tmp/astra-349-next-lane-dyn-reconciliation.md: kernel.rs fill_taps (817) performs strided copy_lane for every lane, and gather_detector (861) builds a scalar scratch gather before Lane::load. Dual per-frame calls520/521, staged729/730, mono per-frame1216 and mono staged1252 all remain live. Neither helper specializes equal delays. Open GitHub issue roster queried read-only: no dedicated uniform-tap/DYN-1 implementation issue found; #191 is broader product-feature scope. Numbered local specs likewise contain no delivered specialization. Reconfirm roster at the actual issue boundary; do not claim this queue draft has an implementation owner.

DYN-1 is a real source opportunity, not a measured instruction/cycle saving. DYN-2/DYN-3, cross-effect shared helpers, D-in-program-key/cohort repartition and native AArch64 remain excluded. The existing floor ruling treats tap bookkeeping as Class A; changing admission/key/latency or DSP is not authorized.

## Smallest closable implementation — freeze this choice

Implement ONLY compressor-local uniform detector access, in both staged and per-frame ordinary/mono paths. Freeze the staged variant to two contiguous safe slice copies into the EXISTING tap scratch, split at ring wrap. Do not borrow the live ring directly through later writes, remove scratch, change ring allocation/layout, change staging eligibility or reorder DSP.

Choose a transient classification, not a new Channel cache field: a private helper returns Some(D) iff existing delay[0..L::WIDTH] all equal, otherwise None. Compute each channel's classification once on entry to the actual frames_loop/frames_loop_mono segment, outside the frame iteration, and pass it to gather_detector. fill_taps may compute it once per staged segment. No lookahead/delay mutation occurs inside those frame loops; ramps redesign smoothed parameters, while delay changes only at preparation/full reset/restore. This avoids a new persistent field, serialized/cache maintenance, per-frame equality scan and resource-layout change. Preserve independent left/right classification. W1 is uniformly classified by construction; inactive MAX_WIDTH padding does not participate.

Uniform gather: derive row=(write-D) wrapped exactly as today, then return L::load from the one existing contiguous row at row*width. The ring write STILL precedes this read in the per-frame path, including D=0. Ragged gather retains the original lane traversal and scratch load, unchanged arithmetic/order.

Uniform fill: calculate the same initial row and first=min(B-row,len); copy detector[row*width..(row+first)*width] into scratch[..first*width], and detector[..(len-first)*width] into the remaining scratch. Preserve the existing len*width prefix envelope and caller-proved staged bound. Ragged fill retains the original two copy_lane calls per lane. Do not relax D>=len; D=0 and D<len still use the live per-frame path, and D=len remains staged. Existing cursor selection, separate channel cursors, sidechain/linking, nonlinear math and gain recurrence are untouched. No unsafe access, memcpy intrinsics or new public API.

## Exact allowed paths

crates/compressor/src/kernel.rs: the classification/access specialization and focused private unit tests/oracles/witnesses only. crates/compressor/tests/staged_idle.rs: extend existing compact partition/bank fixtures. crates/compressor/tests/mono_collapse.rs and payload.rs: only directed restoration/copy/mono identity transitions if private inline tests cannot exercise the existing public path. crates/compressor/tests/conformance.rs: extend existing installed allocator evidence narrowly if its current case does not distinguish uniform/ragged render. Existing tests/support/mod.rs may gain only shared fixture support needed by these tests. Numbered spec and retained evidence are allowed.

Do NOT edit state.rs serialization/commit arithmetic, Channel layout, lib.rs production factory/dispatch, effect-runtime/rack/lane kernels, Cargo dependency graph, corpus/pins, runtime policies, runner or CI framework. The transient classification must make these unnecessary. If that proves impossible, report the precise seam before changing scope. The existing bench-support dev dependency already supplies the real allocator; no new allocator is allowed.

## Finite discriminating acceptance

1. Private helper oracle compares uniform and ragged access against a retained direct transcription of OLD lane-index access, at W1/W4/W8, using distinct frame/lane payload bits. Include start-row before/at/after wrap, len0/1 and representative complete/tail segment lengths, exact scratch prefix with outside sentinels. Fill comparisons must actually execute the uniform two-copy route; ragged must execute old gather. Use private test-only witness/counting or an equally direct callsite witness, not production telemetry. One temporary actual uniform-dispatch-to-ragged mutation must fail the SAME positive mechanism assertion while the old bit oracle still agrees. No wider mutation campaign.
2. Extend existing staged_idle tests, which already span partitions1,7,63,64,65,127,128,129,512 and lookahead-derived D960,720,120,64,24,0 at48k. Preserve the 512-frame public per-frame reference, but recognize it also gains the uniform optimization: independent OLD tap oracle from gate1 is therefore mandatory. Demonstrate D=0, D=len, D<len, wrapping, uniform/ragged W4/W8 (plus scalar), and left uniform/right ragged or unequal uniform D. Preserve exact PCM and serialized final state, including its existing left-only rejection/cursor-divergence test. Use finite signed-zero/finite/asymmetric source data; no all-NaN equivalence claim.
3. Existing mono-collapse/copy and payload restore tests remain. Add a compact direct transition through nonuniform→uniform and uniform→nonuniform restored active delays, defaults/full reset, and whole-channel copy/mono reopen. Since classification is transient, assert the NEXT actual render uses the new classification and preserves old PCM/state; do not add a serialized field. Do not assert mid-ramp restore bit identity against pre-restore history: state.rs explicitly documents its existing re-derived-step behavior. Compare identical restored baselines.
4. Reuse installed conformance/bench-support allocation hooks: positive allocate AND free liveness outside the audit; repeated actual prepared uniform staged, uniform per-frame D0, and ragged fallback rendering has zero allocations/frees. Prepare buffers/state/snapshots outside counting. Existing default allocator behavior must not be changed, and no global counter contamination may be credited as isolated evidence.
5. Existing compressor corpus pins and native W1/W4/W8 identity remain immutable. Supported scalar-Wasm and simd128 replay the existing compressor corpus through current wasm-gates; no new corpus or AArch64 qualification. Existing conformance, state, sidechain/linking, reset, nonfinite and mono suites remain release blockers. No numerical tolerance, equation, latency, ring depth, admission or floor change.

## Executable local gates and object evidence

Focused source qualification uses the actual named existing targets (nonempty): cargo test --locked -p compressor --lib; cargo test --locked -p compressor --test staged_idle; --test mono_collapse; --test payload; --test conformance; --test cross_target. Repeat affected tests with --release, then cargo test --locked -p compressor, strict clippy for compressor all-targets, fmt, realtime/lane/workspace policy. Record actual test names/counts; do not credit a zero-match filter. Immutable workspace and supported-target qualification follow source PASS under the normal delivery workflow.

Capture untimed baseline and candidate native release object/IR with the same supported x86-64-v3 configuration, rustc version and profile. Existing cargo rustc --locked --release -p compressor --lib -- --emit=asm,llvm-ir can expose production-instantiated functions; use a dedicated target. If LTO makes an object uninspectable, use explicitly supplemental CARGO_PROFILE_RELEASE_LTO=false for BOTH baseline and candidate and record the distinction from shipped artifact. Identify actual compressor process_bank/per-frame/staged monomorphizations and their call sites, not unrelated lane probes. Record named bodies/source mapping and whether the uniform arm loads one row / copies contiguous runs, while ragged work stays conditional. Inspect supported Wasm scalar/SIMD production objects with the existing checked disassembler procedure; count only successfully decoded objects, no bad-magic falsePASS. No report framework or new permanent probe binary.

If baseline lowering already eliminates a claimed gather/copy cost for a specific instantiation, record that exact honest null before editing; source and correctness mechanism evidence remain distinct from a speed claim. If symbol inlining prevents a trustworthy hot-body claim, retain the observed IR/caller evidence and state the limit rather than fabricate instruction counts. Object evidence is not a latency/cycle benchmark.

No timing invocation is authorized here, including the existing lane_sample_timing example. This small product issue should not acquire runner repair or a new measurement corpus. Any later descriptive capture requires its own explicitly frozen existing workload/validator/authority; DYN-1 may not borrow console or historical hoist measurements as proof of this access change. #349 retains DYN-2/DYN-3 and all unrelated work; no support revival or projected speedup.

## Numbered queue and delivered base

This is GitHub #475, with the exact title above. Current base is delivered main660fce8f2c4f76d38c82590f4c0411c117ba857d; compressor/Cargo/config inputs match the reviewed4a814f34 source. The fresh open issue roster contains broader #191 but no duplicate dedicated uniform-detector-tap issue. This is queued scope only: Astra must approve the numbered spec and actual base before Luna implementation. #443 remains the active runtime feature; no build, object capture, workload invocation or performance measurement is authorized by this checkpoint.

## Inherited DSP and evidence contract

This access-only optimization retains the current feed-forward peak detector, explicit DualMono/Maximum/Average linking, dB soft-knee gain computer and branching one-pole. Mathematically the smoother is `G <- G + c * (C-G)`; attack is selected strictly when target reduction `C < G`, equality uses release. Current `design.rs::rate_coefficient` computes `c = 1-exp(-1/(0.001*time_ms*Fs))` in f64 and rounds once to f32 at coefficient design/update. The current kernel's bounded level/gain math and exact unfused operation order are unchanged. Do not copy historical FMA or per-sample-libm wording from the original #013 brief over current source and the unfused ruling.

The affected operation is only the existing integer detector address `(write + B - D) mod B`, expressed using the existing wrap rules; writes precede reads, including `D=0`. Fixed latency remains `N=Fs/50` samples, ring length `B=N+1`, preparation/state-only lookahead0..20ms and declared infinite tail. Existing threshold/ratio/knee/attack/release/makeup/mix units, domains and 64-update ramps remain as specified by #013 and its amendments. Current recursive-state flush and end-of-block per-channel rejection/reset rules in `docs/EFFECT_CONTRACT_V1.md` and the kernel remain unchanged; the specialization must preserve nonfinite, subnormal, signed-zero and recovery behavior bit-for-bit.

Research basis is retained in `dsp-research/dynamics.md` and primary-source entries `[REISS-COMP]` (Giannoulis, Massberg and Reiss, JAES2012) and `[SMITH-SASP]` in `dsp-research/BIBLIOGRAPHY.md`; fixed reported latency follows its `[VST3-LATENCY]` entry. The exact original signal/state laws and subsequent amendments are in `.github/ISSUE_SPECS/BRIEFS/013-compressor.md`; current source and standing rulings resolve superseded implementation names and arithmetic. Existing compressor corpus, independent reference and #046 qualification/listening-handoff requirements remain the evidence baseline. No completed human listening or new benchmark result is claimed here. This issue proves bit-preserving access specialization with the finite gates above; any later timing has separate authority.

## Astra numbered scope approval

# Astra #475 numbered scope — PASS, queued only

Reviewed0e37c99f72ce80c4307e8d2683ee7b07ba1948bc in engine-475-plan and complete475-uniform-compressor-detector-taps.md against the original /tmp/astra-dyn1-current-scope-brief.md. Only the numbered spec differs from660fce8. Read-only Git comparison confirms compressor/Cargo/configuration inputs unchanged between that inspected base and deliveredfa3485c6. No implementation, tests, builds, object captures, timing or Git/GitHub mutations performed.

The original bounded uniform-tap design is preserved: transient per-segment delay classification, independent planes, safe contiguous per-frame row loads and staged two-copy wrap handling into existing scratch, old ragged fallback, unchanged channel/state/ring layout and staging eligibility. Finite old-tap oracle, W1/W4/W8/wrap/delay boundaries, next-render restore/copy/mono transitions, genuine mechanism control, zero allocations/frees and existing immutable corpus/target qualification remain. No D-in-key/cohort redesign, DYN2/3 work, new runner/framework or AArch64 revival is introduced.

The added inherited DSP paragraph is consistent with current source: design::rate_coefficient's f64 design/update exponential, existing unfused branching one-pole and strict attack predicate, fixed Fs/50 latency, ring N+1, Infinite tail and existing64-update ramps. The prose appropriately defers exact operation ordering, nonfinite/reset/flush behavior to current kernel and EFFECT_CONTRACT, rather than reviving historical FMA/per-sample-libm claims. Existing REISS-COMP, SMITH-SASP and VST3-LATENCY bibliography entries and retained compressor design/qualification/listening handoff establish the standing references. No new research campaign or completed human listening/timing result is claimed.

Approve numbered queue scope. Actual implementation-base verification and root assignment remain required when its queue position is reached; active443/runtime and473/tooling are unchanged. This review grants no implementation or benchmark authority.

## Delivered-base readiness after #463

Root integrated current deliveredmain571dfc5be5fb040a537c11d683734b8015f37c32 without conflicts. #463 is remotely CLOSED after PR493; #492 is independent host-core delivery maintenance. Boundary reconciliation reports329 remote issues,234 local numbered specs, no missing local identities. Compressor, Cargo/configuration remain unchanged from the approved queued base; lane kernels/tests and the G5 matrix corpus now include delivered #463. Frozen narrow compressor scope is unchanged, but prior no-delta statements do not cover these new lane/corpus inputs. Pending Astra actual-base approval before Luna1 baseline or implementation.

## Current actual-base approval and Luna attempt 1

# Astra #475 actual implementation-base review — PASS

Reviewed exact clean head2daa3510e683aefd7d90e9065d9c4cb9639f9e92 in engine-475-plan against delivered main571dfc5be5fb040a537c11d683734b8015f37c32. Only the numbered475 spec differs from that main; crates/hosts/tools/Cargo/config/scripts source is identical. Read the complete updated scope and previous conditional readiness. Root's retained boundary audit reports329 remote/234 local numbered identities, missing0, with #463 delivery/closure separately confirmed by root. No tests, builds, object captures, timing or repository/Git/GitHub mutations performed.

The dependency change is precisely accounted for. From old approved660fce8 to delivered571dfc5b, compressor and Cargo/config remain unchanged; lane's three block kernels/tests and G5 corpus/lib plus pins contain delivered463. The compressor detector helpers still perform their original strided per-lane fill_taps and scratch gather_detector work, with ordinary dual and mono call sites intact. They do not call the three changed lane block kernels. The delivered lane arithmetic/bounds and four new matrix corpus entries are now the baseline; do not reuse the earlier statement that lane/corpus inputs are unchanged, remove those entries, or regenerate existing compressor pins.

Approve fresh Luna1 assignment by root for the existing bounded product: classify active WIDTH delays once per actual frame-loop segment, independently per channel; uniform row load after existing writes; staged two safe contiguous copies into existing scratch; retain ragged traversal, staging eligibility, cursors, ring/layout, state serialization, latency and all DSP behavior. No D-in-key/cohort change, persistent cache, public API, unsafe, new corpus, framework or timing authority. Exact allowed compressor paths and finite frozen proof gates remain sufficient without amendment.

First checkpoint sequence remains mandatory: capture current-base actual native production-instantiation baseline object/IR before access rewrites, retaining commands/status/toolchain/profile and any honest lowering null. Use dedicated targets independent of #492 qualification. Then make the compact private kernel specialization and old-access oracle/witness checkpoint, compile and run its meaningful uniform/ragged W1/W4/W8 and wrap coverage, and pause for root's recoverable checkpoint before adding the remaining public transition/allocation evidence. Do not treat that first tranche as product acceptance. Full dual/mono PCM/state, D0/Dlen/Dltlen, restore/copy/reset next-render classification, genuine same-assertion mechanism control, zero allocations/frees, existing corpus and supported object evidence remain mandatory for the single consolidated attempt verdict.

#463 no longer occupies runtime WIP; #492 is independent host maintenance. No additional scope ruling is needed before this assignment. Root retains all checkpoint/synchronization ownership, and later source acceptance plus proportional immutable delivery/actual-head PR/required CI remain separate gates.

Root adopts PASS and assigns fresh Luna1. Preserve actual native production baseline before edits, then pause at the first compact compiling/focused-green specialization/old-access-oracle tranche for exact-path root checkpoint and push. Continue remaining proof only after checkpoint. No timing; root owns Git/GitHub and one consolidated adversarial verdict per attempt.

## Luna attempt 1 consolidated submission

# Luna attempt 1 submitted evidence

Product source243ca03e; test checkpoints55e1c1e8 and89ae24ef. Native baseline preceded edits atcf5c6148. Full compiler outputs remain in original task targets; baseline/candidate process-callers.ll and source-mapping.txt are root’s later deterministic extracts, not new compiler runs. Root selected complete LLVM define blocks containing compressor and process_block/fill_taps/process_bank_inner; frames_loop and idle_frames_staged survive as inline debug mappings rather than separate define bodies. Original IR/ASM hashes are retained. This corrects any interpretation that debug metadata itself proves a separately emitted function. No isolated instruction-count or timing claim.

Luna stopped with dedicated public next-render transition/state/PCM, real allocation/free liveness/repeated zero-count proof, and successfully decoded production Wasm object evidence unfulfilled. The claimed mutation failed a positive classification assertion; its authentic log is retained but no standalone actual mutation diff was saved. Final adversarial assessment must determine whether it proves actual optimized dispatch.

Wasm logs later completed: root found actual STATUS=0 and all three backend records each139cases/349comparisons/zero mismatches in both retained logs; both JSONL outputs are retained. Root process inspection found no matching live475/run-wasm-gates process. Luna’s earlier unresolved-status report is superseded by these terminal records; no status was invented and no additional G5 run was started. The two calls contended on fixed script build directories, with source89ae24ef unchanged throughout.

Existing68 release tests/strictClippy/initial access fixture are useful but do not substitute for unfulfilled frozen gates. This is one incomplete Luna attempt, pending one consolidated Astra verdict; no completed product or measured speed gain is claimed.

## Current assignment: Sol attempt 2 — Astra decision

# Astra #475 Luna attempt 1 — FAIL

Exact reviewed head4262fe40df09c3a7e59333f45faa9ddef6517051, production243ca03e, final tests89ae24ef, engine-475-plan. One consolidated verdict against the complete frozen475 contract. No further Luna repair is authorized; root may assign Sol2 to the finite original groups below. No tests/builds/timing or source/Git/GitHub mutations performed.

Production is accepted on inspection: transient classification reads only L::WIDTH, independently per channel and once before each ordinary/mono frame segment; unchanged writes precede uniform detector-row loads, including D0. Uniform staged access uses two safe contiguous copies into the existing bounded scratch; ragged traversal remains intact. Delay values cannot change within those loops. No channel/state layout, serialized field, staging admission, cursor, DSP arithmetic, public API, unsafe or dependency change was introduced. The enum instead of Option is an equivalent private representation. The remaining work is original proof completion, not product redesign.

1. Complete the actual access oracle and causal dispatch proof. Current private test establishes useful W1/W4/W8 row/lane values, padding exclusion and one wrapping uniform fill, but the extra len0/1/wrap cases run after delays become ragged for wide lanes. They do not cover the frozen uniform no-wrap/wrap boundary population; the uniform gather is passed a hardcoded class and only one write position. Run both real classifications through the frozen compact start-row before/at/after wrap and len0/1/complete/tail cases, compare all actual output words against independent old lane-index access, and preserve outside-prefix sentinels. Cover uniform and ragged as applicable (W1 cannot be ragged through real classification). Crucially, assert actual optimized gather/fill route execution with private test-only witnesses at the branches, not only delay_class's return. The retained mutation fails `Ragged != Uniform(2)` before access, and would not reject a production branch bypass that leaves classification correct. Retain ONE actual uniform-dispatch-to-ragged mutation diff/command/status, showing the same positive access mechanism assertion fails while old bit equality still holds; restore and pass the unchanged test. Do not reconstruct the missing Luna diff or add a larger mutation campaign.

2. Finish existing public PCM/state and next-render transition proof. No staged_idle/mono_collapse/payload tests changed. Existing suites provide valuable partition/link/sidechain/nonfinite/regression evidence but do not establish all named new paths. Extend their existing helpers narrowly for W1/W4/W8 uniform/ragged, D0/live, D=len/staged, D<len/live, wrap and independent channel classification (left uniform/right ragged or unequal uniform D), with finite/asymmetric/signed-zero data and complete PCM plus serialized final-state equality. Retain the existing left-only rejection/cursor-divergence case. Complete nonuniform→uniform and uniform→nonuniform through supported restore, defaults/full reset, and channel-copy/mono-reopen transitions; assert the NEXT actual render classification and PCM/state, not just direct assignment followed by delay_class. Compare identical restored baselines, respecting existing re-derived ramp-step behavior; no new restore API or serialization change. The existing 512-frame reference is also optimized, so it supplements, not replaces, group1's old-access oracle.

3. Complete real realtime allocation proof in the allowed existing conformance/bench-support seam. Establish positive allocate AND free liveness outside the measured region, then repeated actual prepared uniform staged, uniform per-frame D0 and ragged fallback renders with zero allocations/frees. Prepare buffers and snapshots outside counting; preserve allocator installation and avoid contamination. The current source/test change supplies no dedicated case proving these three actual paths. No new allocator or global framework is requested.

4. Complete the frozen object/final-gate evidence. Native pre-edit IR/ASM and candidate actual process_block/fill_taps/Instance-render definitions are useful retained evidence. Root's later extracts are accurately attributed; inline frames_loop/idle metadata is not a separately emitted body or standalone instruction count. Retain that limitation and identify the actual uniform row/copy arm plus conditional ragged path in the production caller/IR. Supply successful decoded scalar-Wasm and simd128 production-object evidence using the existing checked procedure, recording actual compiler/decoder statuses, object identities/population and source mapping; do not count bitcode bad-magic or G5 execution as object inspection. Finish affected debug/release tests, strict Clippy, fmt/diff and realtime/lane/workspace policy on the completed candidate with actual commands/source/log/numeric exits and nonempty named filters. Existing retained Clippy points to55e1 and fmt log tocf5; they must not be labeled final89ae proof. No full delivery qualification or timing is authorized by this FAIL.

Evidence accepted within its actual scope: all25 manifested payload hashes/sizes and exact26-file tracked coverage verified. Both retained G5 logs now truly end STATUS=0; each JSONL records native/scalar-Wasm/simd128139 cases/349 comparisons with no mismatches. Earlier unresolved-status prose is superseded, not a current failure. Corpus/pins are unchanged. Existing full debug/release compressor runs and initial access test remain useful accepted regression evidence. Do not rerun identical G5 merely to erase historical concurrent launches if source remains equivalent; after real relevant source changes, use the frozen existing gate as appropriate. No measured speedup, new corpus/framework, D-in-key, DYN2/3 or AArch64 work is requested.

These four groups are the original contract's finite omissions. Preserve accepted production and evidence while completing them in one coherent Sol2 pass, with root checkpoints; then one consolidated adversarial verdict. Product/source PASS remains prerequisite to immutable delivery qualification and actual-head PR/required CI.

## Sol attempt 2 submission

Exact reviewed source is `60004323b0b8fb71b80864927c2c9e186dc6be49`; production is unchanged from
the accepted attempt-1 implementation. The four finite proof corrections are complete in
`artifacts/issue475-sol-attempt2/`:

- the real classifier drives a W1/W4/W8 old-access oracle over uniform and applicable ragged
  no-wrap/wrap boundaries, empty/singleton/complete/tail prefixes and outside-prefix sentinels;
  private branch-local witnesses prove actual uniform/ragged fill and gather dispatch, and one
  retained live uniform-to-ragged dispatch mutant preserves old-access bits before the mechanism
  witness rejects it;
- public scalar/native-bank fixtures compare complete PCM bits and serialized state for D=0,
  D=len, D<len, wrap, both-ragged and independently classified channels, plus bidirectional
  restore, full-default reset and mono-reopen channel-copy transitions on the next actual render;
- the existing audited allocator proves positive allocation and free liveness outside an isolated
  interval, then zero allocation/free/reallocation over 32 repeated actual renders of uniform
  staged, uniform D=0 per-frame and ragged bank paths; the initially contaminated parallel run is
  retained rather than hidden;
- locked debug and release compressor suites, strict Clippy, fmt/diff and realtime/lane/workspace
  policy pass. Fresh linked scalar-Wasm and simd128 production modules compile and decode
  successfully, with identities/populations/source mapping and the uniform two-copy plus
  conditional ragged shape recorded. Direct `--emit=obj` produced LLVM bitcode and its bad-magic
  decode is retained with no inspection credit.

Public test width claims are bounded to W1 scalar and native W8 on this host. The private oracle
instantiates W1/W4/W8; decoded Wasm production modules carry named W4/W8 instantiations and the
unchanged accepted G5 records cover scalar-Wasm/simd128 execution. No timing, new corpus/framework,
immutable delivery qualification, full-workspace qualification or PR/CI claim is made. Pending one
consolidated Astra adversarial verdict.

Root authorizes the named attempt within this frozen scope. Pause at each compiling, focused-test-green tranche for root checkpoint before layering more implementation.

## Sol attempt 2 consolidated FAIL and final attempt assignment

# Astra #475 Sol attempt 2 — FAIL

Exact head da23c05be4a8e8965e71b360fb15c6eb321b076f, source60004323b0b8fb71b80864927c2c9e186dc6be49, engine-475-plan. One consolidated verdict against frozen475 and four Luna1 findings. No tests/builds/timing or repository/Git/GitHub mutations performed. Production specialization remains accepted; two finite completion groups below remain for final Sol3.

Accepted corrections: the old-access oracle now uses actual classification for uniform/ragged W1/W4/W8, compact write positions and lengths0/1/4/10, full bit comparisons and outside-prefix sentinels. Branch-local fill/gather counters follow the real access arms. The actual retained fill-uniform-to-ragged mutation changes the production branch, passes preceding old-word comparisons, then fails the SAME FILL_UNIFORM_CALLS positive assertion with101; restored source passes. This replaces Luna's inadequate classification-only mutation. No second mutation is required.

Public native-bank tests now compare full PCM and serialized state for independent channel delay populations/partitions, both restore directions, full reset, and mono reopen/copy followed by the first dual render. Existing scalar delay/partition/link/recovery tests remain useful. The actual allocation test proves positive allocation and deallocation liveness outside32 repeated staged W1, D0 W1 and ragged native-bank renders, with zero alloc/free/realloc. Its exact-filter child avoids the demonstrated process-global counter contamination; failed original runs are correctly retained. Accepted scope does not require a new allocator or wider allocation matrix.

Successful linked production Wasm decoding is now present separately from retained bad-magic LLVM-bitcode failures. Root's complete scalar/simd128 fill_taps extracts and original disassembly identities are attributed as later extraction, not new compilation. Both W4/W8 instances and uniform memory-copy/conditional ragged structure are represented. Native caller/inline limitations remain candid. Earlier accepted unchanged G5 scalar/native/simd128139/349 records remain applicable and need no duplicate execution merely for test-only edits. All16 payload hashes/sizes verified; final package's narrow source/width claims are honest. Final debug/release/strict Clippy and named policy evidence close their actual recorded scope, not unexecuted public widths.

1. Complete the frozen actual W4 transition/state path. Both new public bank transition functions obtain native_bank_width; on this x86-64-v3 build they execute only W8. Generic W4 tap-array oracle plus decoded W4 functions plus unchanged G5 corpus does not execute the new restored-delay/full-reset/mono-reopen transition behavior at W4. The README correctly admits that limit, but cannot substitute for the frozen gate. Do not change Backend::current or public factory admission: lib.rs explicitly declines unavailable bank widths. Smallest existing private seam is a cfg(test) fixture under the allowed kernel.rs test module constructing the existing crate-private PreparedCompressorBank<Simd4>/Instance using the same validated defaults/metadata and invoking its actual PreparedNativeEffectBank restore/reset/process/desymmetrize trait methods. This exercises the real W4 state owner and public processing implementation without claiming native factory admission; Rust child-module privacy permits ancestor-private access. If necessary root should explicitly record this test-only construction clarification before assignment; no lib.rs production edit, backend override, public test API, new Wasm harness or corpus is warranted. Reuse the finite restored uniform↔ragged, full-reset and copy/reopen next-render PCM/full-state comparisons, with unchanged old-access oracle. Preserve already accepted native W8 and scalar evidence; do not demand an impossible ragged W1 population or expand to every width×partition Cartesian combination. Actual supported-Wasm execution of those existing cases is an alternative only if available through existing machinery, not a reason to build a new runner.

2. Fix the concrete child-marker policy defect before delivery. conformance.rs22 introduces `MISO_475_ALLOCATION_AUDIT_CHILD`. check-env-vocabulary.sh scans tracked source for MISO_ names and rejects every name outside MISO_ENGINE_; this test file is in its population. The passing realtime/lane/workspace logs do not execute this distinct policy. Mechanically rename this one test-only marker consistently to `MISO_ENGINE_COMPRESSOR_ALLOCATION_AUDIT_CHILD` (or an equally precise existing-prefix test name), preserving exact child selection and counter isolation. It is crate-only, so the current vocabulary registry's tools/scripts keyset does not require adding an unused documentation row. Read-only no-ignore search found the offending spelling only in conformance.rs, not the Sol2 retained package. Run the actual vocabulary gate after correction and retain its numeric status; no exemption, gate change or historical log rewrite. This is necessary test integration, not an extra DSP matrix.

After these finite corrections, retain meaningful exact new W4 and allocator tests in debug/release plus the frozen affected full suites, strict Clippy/fmt/diff and policies on final source. Preserve accepted production, native/object/G5 evidence and authentic failed history. No more informal Sol2 revisions; root may checkpoint this FAIL and authorize one final Sol3 pass. Another FAIL hardstops/rescopes. Immutable delivery/artifact qualification and actual PR/CI still follow source acceptance; no timing authority is granted.

Root adopts the two finite corrections and authorizes Sol attempt 3, the final attempt. The W4 construction clarification is explicit: only cfg(test) kernel.rs child-module code may construct the existing private PreparedCompressorBank<Simd4>/Instance using validated existing defaults and metadata, then invoke the real PreparedNativeEffectBank restore/reset/process/desymmetrize operations. This is a test fixture for the existing state owner, not a claim that the native public factory accepts W4. No production lib.rs change, backend override, public API, harness, corpus, additional mutation or timing is authorized. Preserve accepted native W8/scalar and generic old-access evidence. Rename the test marker consistently to MISO_ENGINE_COMPRESSOR_ALLOCATION_AUDIT_CHILD and run the actual environment vocabulary gate. Complete one coherent pass and one consolidated Astra verdict; another FAIL requires hardstop and explicit rescope.

## Sol attempt 3 final submission

Final source checkpoint `019ac62a` completes the two authorized corrections without changing
production behavior. A `cfg(test)` child fixture constructs the existing W4 state owner through
validated production metadata/default/ring derivation, invokes the real bank trait's restore,
reset, dual/mono process, desymmetrize and snapshot methods, and compares next-render PCM plus all
four complete lane payloads for both restored delay directions, full-default reset and mono reopen.
It also proves the public native factory continues to decline unavailable W4. The allocator child
marker is now `MISO_ENGINE_COMPRESSOR_ALLOCATION_AUDIT_CHILD`; the unchanged isolated allocator
test passes in debug/release and the actual environment vocabulary gate reports PASS.

The final affected debug and release lib/staged-idle/mono-collapse/payload/conformance/cross-target
suites pass with nonempty counts. Full compressor debug, strict all-target Clippy, fmt/diff and
realtime/lane/workspace/environment policies pass. Failed fixture compilation and the first
executed mirrored-plane oracle failure remain verbatim in `artifacts/issue475-sol-attempt3/`
beside the corrected evidence. Accepted Sol2 production/object/G5/mutation/allocation evidence is
unchanged and was not rerun for these test-only edits. No timing or delivery qualification is
claimed. Pending the single consolidated final Astra verdict; any FAIL hardstops this issue shape.

## Final attempt verdict — hardstop

# Astra #475 FINAL Sol attempt 3 — FAIL; hardstop/rescope

Exact headce745d8a5b25c77964641f9483d33794aa90d7ea, source019ac62a, engine-475-plan. One final consolidated verdict against the two remaining original groups and adopted private-W4 construction clarification. No tests/builds/timing or repository/Git/GitHub mutations performed. This is attempt3: preserve this checkpoint/evidence and rescope a bounded proof successor before further correction; no informal fourth475 pass or delivery qualification.

Accepted: production remains unchanged from accepted specialization; all new kernel code is cfg(test). W4 fixture construction uses the existing PreparedCompressorBank<Simd4>/Instance with production metadata/default/ring validators and invokes the actual restore/reset/process/mono/desymmetrize/snapshot trait methods. It does not change native factory admission and explicitly records that its unavailable-width check concerns this x86-64-v3 host. Both restore directions, full reset and mono reopen are represented. The child-marker rename is correct; actual environment policy now passes114 names. Isolated allocation audit remains valid and passing. Earlier native/object/G5/access-mechanism evidence is preserved without invented reexecution.

One finite original proof defect remains: the W4 transition comparisons never reach populated delayed audio or detector taps. request() fixes48k and render4 processes128 frames. The compressor latency is960 samples (ring961). Uniform lookahead0 yields D960; ragged lookaheads0/2.5/5/7.5ms yield D960/840/720/600. Restored source is primed only once (128 frames), then compared on its second128-frame render. Reset renders once from cleared rings. Mono reopen renders one mono block then one dual block from initially clear state. All are at most256 frames after clear, below even the smallest detector delay600 and far below audio latency960.

Consequently the complete PCM assertions compare delayed startup silence, and the saved state comparisons chiefly prove copied/written rings, parameters and cursor. They do not discriminate using the wrong delay population for actual detector reads on the first post-transition render: all selected histories are still zero. No per-call classification/branch assertion in this new test supplies the missing discrimination. The independent W4 tap oracle proves access arithmetic in isolation, not this required transition/state-owner linkage. This is the remaining original next-render PCM/state gate, not an additional target matrix or requested performance test.

Smallest successor scope: only complete this existing cfg(test) W4 fixture with a populated, frame/lane-varying detector/main history before the restore and mono-copy transitions, through existing real rendering or supported payload restore. Keep the first post-transition comparison and explicitly assert its old/reference expected PCM contains nonzero distinguishable words before candidate equality; ensure delay alternatives sample different live history. Retain complete serialized-state comparison. Full reset must still prove its legitimate first-block silence/state, then continue the same fixed finite sequence past latency to compare the populated output/default-delay behavior; do not mislabel first-after-reset silence as populated output. Freeze these few phases before coding, preserve existing W1/W8 and old-access/mechanism proofs, and add no public API/backend override, new runner/corpus or broader matrix. Parent475 keeps the full original acceptance and later qualification obligations; numbered proof child must precede implementation.

Evidence otherwise checks out: all12 manifest payload hashes/sizes and exact13-file tracked coverage verified. Actual focused/affected debug-release, full debug, strict Clippy, fmt/diff and realtime/lane/workspace/environment statuses are0; both wrong-cwd/corrected package verifications and earlier compile/oracle failures are authentic retained history. Green executions do not remove the startup-silence limitation above. No additional production defect is identified, and no other new correction group is imposed.

Root adopts the hardstop. No further implementation or delivery qualification under parent475 is authorized until a numbered, Astra-briefed proof successor owns the finite populated-history correction. The parent retains the complete product contract and all eventual delivery gates; no acceptance requirement is weakened.

The one remaining populated-history proof obligation is now numbered child #499. Parent implementation remains frozen; child499 has its own bounded brief and fresh attempt budget, without weakening the parent acceptance or delivery contract.

## Child499 acceptance and integrated delivery freeze

Astra child499 Luna1 PASS at05c73e7e completes the sole populated W4 proof gap; the full review is retained in499 spec. Parent hardstop history remains preserved, and no fourth parent implementation occurred. Root integrated delivered main95abdd015e28823905800d051d03837255d91612 plus the498 closure record, and git diff over the entire compressor crate against reviewed05c73e7e is empty. Retained immutable workspace/supported-target and ordinary current-artifact qualification are now authorized at the next pushed frozen checkpoint. No timing, numerical expectation or corpus change.

## Integrated current-artifact amendment

# Astra #475/#499 integrated artifact mismatch — approve bounded delivery amendment

Reviewed immutable combined source1c71000d19abd609159e1006fc6291c57ff4670d in engine-475-populated-proof. Entire compressor remains byte-identical to accepted499 head05c73e7e, including the retained475 product and completed child proof. Current delivered maintenance is integrated without further compressor change.

Authentic /tmp/475-delivery-builder.command.json/.log/.status names this source and dedicated target/output, records successful release compilation, then builder exit1: expected8a42eb47c36ff8053b66054d72a06dee48b3c3577943f21119e0709fe64d0d0c, observedfa78dc8d3f0d391b94419f5252504eee2aafbbf13853884157e24935ed092cc3. The observed identity is the builder result, not a new PCM/resource result. Retain original invocation and failure bytes/status.

Root may synchronize the bounded delivery amendment and checkpoint only the actual observed pin plus evidence, AFTER the immutable workspace/target/ABI sequence terminates or in a separate delivery checkout with verified identical production/build source. Do not mutate the running candidate. Ordinary verified rebuild on that same combined source must establish the actual published artifact identity; follow with existing static/object/ABI, expected resources and26 red controls, separate hermetic suite, current Chromium/Firefox/WebKit qualification with self-test mutations and matrix check. Update generated candidate/hash records only to the actual qualified source/output.

No PCM/resource numerical expectations, existing corpus pins, DSP/source behavior, lint/CI/policy configuration or target matrix may change under this ruling. An additional discrepancy requires its own concrete assessment. No timing, new framework, stale standalone artifact promotion or automatic merge approval. Finish the retained parent immutable workspace/supported-target/ABI evidence independently, package candid provenance and final statuses, then obtain exact-head PR review and required CI SUCCESS before parent/child delivery closure.

Read-only source/log inspection; no tests/builds/timing or source/spec/Git/GitHub mutations performed.

Root uses a separate delivery checkout from frozen1c71000d; the source qualification worktree remains untouched while workspace/target/ABI gates finish.
