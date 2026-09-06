# 496: Compute post-ramp channel symmetry with one extraction per lane word

Ready-to-number queued residual RT5 product, distinct from235's delivered stationary cache and238's addressed-record update. Revalidated delivered fd4a937cbb782ebe04be0594f439606389dbe283 via explicit Git object reads; builtins/src/lib.rs blob0baa4bd1dad69faf95a2b5dfdcf56071e7ee3b4a. #238 is delivered and closed; this is a new queued product after active #475, not a revision of #238. No implementation, tests/builds/timing or Git/GitHub mutations performed. Root must integrate and freeze the actual future implementation base before assignment.

## Actual remaining mechanism and smallest slice

After a dual ramp block, InputStage::process calls settle then full refresh; after a collapsed ramp block, process_mono calls settle then mirror_trim_ramp then full refresh. The current full refresh calls compute_lane_channel_symmetry per populated lane. That predicate repeatedly materializes the same two-channel SIMD words through lane_read: current trim, target, step and twelve HPF/LPF coefficient pairs, then tests per-lane bits/countdown. Under an all-symmetric full bank, this is30 source-level Lane::store extractions per active lane, although only30 distinct lane words need materialization. Actual machine spills/cycles are NOT measured; optimization may already hoist some work.

Smallest product: a private post-ramp whole-mask helper computes the SAME per-lane predicate by extracting each compared pair of lane words at most once and comparing all populated lanes before moving to the next pair. Use two temporary eight-word arrays per pair and a u8 candidate mask, not a retained coefficient cache or30 simultaneously retained arrays. Clear bits on bit inequality/countdown inequality; padding/out-of-width remains false. It may stop once no candidate bit remains. It must not approximate floats or depend on ordinary floating equality, which conflates signed zeros.

Call this helper ONLY at the two existing post-ramp refresh sites, in exactly the existing order after settle and after settle+mirror respectively. Leave preparation/reset full refresh,238's single-lane update, and compute_lane_channel_symmetry/debug reader oracle unchanged. This isolates the product to the original ramp-block residual rather than revisiting accepted record handling. The helper's semantic output is a complete refresh, not reuse of potentially stale trim bits. No additional InputStage fields/layout, allocations, unsafe/intrinsics, public API, queue behavior, coefficient updates or audio arithmetic.

This is a useful source-level residual even after238, but source counts alone are not a performance result. Before editing, retain a bounded optimized-body/probe inspection for the existing post-ramp helper under supported native lowering using the established project method. If the compiler already eliminates repeated extraction, record an honest null rather than manufacture a speedup or broaden into a general vectorizer. Do not require new timing machinery to close the source/mechanism outcome.

## Exact paths and finite proof

Only crates/builtins/src/lib.rs (private helper, two call sites, cfg(test) evidence), existing crates/builtins/tests/input_liveness.rs and input_liveness_mono.rs if needed, numbered spec/probe/evidence. Existing crates/host-core/tests/input_liveness_console.rs executes unchanged. No lane kernel, rack, compiler, host API, resource estimate or Cargo change.

1. Compare every produced mask with the unchanged per-lane predicate at W1/W4/W8, full and partial banks. Reuse existing prepared tracks and238 sequences to cover equal/unequal trim/target/step/countdowns, unequal coefficient words, signed-zero bits, re-equalization and padding. The oracle runs OUTSIDE mechanism observation. No new exhaustive Cartesian corpus.
2. Through actual dual and collapsed ramp blocks, prove post-settle/mirror cached masks match that oracle, PCM and exposed state words match the existing never-collapsed/original path, and the first subsequent retarget/disengage remains correct. Existing liveness/mono/host console cases provide most of this; preserve recovery reports, LIVE/AGREEING/DESIGNED terms and C-1/C-2. C-3 remains equivalent under complete mirroring, never a demanded false red.
3. One bounded test-local extraction observation on the symmetric full W8 post-ramp refresh must distinguish at-most30 word extractions from the old240 source calls. Do not include settle/kernel/report/oracle calls in that interval. Freeze the same named excess-extraction assertion, temporarily restore the old full-refresh call at that observed post-ramp seam, require its intended assertion failure and restore passing output. No production counter/public witness or extra mutation campaign. For W1 no cross-lane saving is claimed; early-mismatch old paths may already do less work, so do not advertise a universal reduction.
4. New helper must not run on settled no-record blocks. Preserve preparation/reset refresh and238's one-addressed-lane control evidence unchanged. Current compile-time/public parameter rejection remains unchanged.

Focused commands: `cargo test --locked -p builtins --lib`, `cargo test --locked -p builtins --test input_liveness --test input_liveness_mono`, and `cargo test --locked -p host-core --test input_liveness_console`, each debug/release with actual named populations. Fmt, strict affected Clippy and existing builtins/realtime/lane/workspace policy checks follow the current approved workflow. Retain the narrow native code-generation result and no blanket store/register/cycle claim. Existing immutable supported-target/artifact/resource/actual-PR/CI delivery remains proportional; layout must be unchanged.

## Accounting and workflow

This completes repeated cross-lane extraction for the two ramp-block mask refreshes only. It does not remove all ramp bookkeeping, settle/countdown/report lane reads, or provide descriptive sustained-record timing. #238 remains its own record-handling product. If a benchmark is later desired, freeze a separate genuine ramp workload and one invocation under standing rules; none is authorized here. Any proposal changing compared words, equality, mirroring, ramp arithmetic or collapse eligibility stops for explicit ruling.

Root must number/synchronize a stateless issue and perform actual-base review after the current runtime delivery boundary. Astra scope/review, Luna1 then Sol2/3 after failure, hardstop after3. No implementation follows merely from this draft; queued runtime priorities remain root-controlled.


## Delivered-source and duplicate-scope reconciliation

The premise still holds at fd4a937c. The post-dual-ramp call is immediately after settle (lib.rs1273); the mono call follows settle and mirror_trim_ramp (1356). Full refresh at1042 still invokes the per-lane predicate; compute_lane_channel_symmetry at1652 reads three trim/ramp pairs and twelve section coefficient pairs, plus integer countdowns. lane_read at819 stores the complete Lane into an eight-word array. For a symmetric populated W8 bank this is 15 pairs ×2 words ×8 lanes=240 source calls versus30 distinct word extractions. This count excludes settle and DSP work; it is not a machine-code spill or timing measurement. The full per-lane predicate's early rejection can cost less on some asymmetric inputs, so no universal performance claim follows.

Read-only open GitHub title query for ramp/symmetry/RT5/InputStage found no dedicated open successor; current local numbered scope search finds #238's explicit residual retention, not a separate assigned implementation. #235/#238 already own stationary caching and addressed retarget respectively. No duplicate numbered product was found by those targeted checks; root must still run its normal issue/body synchronization boundary before numbering. #475 compressor and #495 graph bind maintenance do not share the allowed production path.

Freeze two named private tests before assignment: `post_ramp_symmetry_mask_matches_lane_oracle` for the compact mask/actual dual+mono behavior cases, and `post_ramp_symmetry_extracts_each_word_once` for the isolated symmetric W8 mechanism assertion. Existing cfg(test) support may add one thread-local extraction observation at lane_read, armed only around the real post-ramp refresh call; default disabled, no production counter or public interface. Clear/read observation outside that interval and run the unchanged predicate oracle after disabling it. Mutation restores the original full-refresh call at the selected actual seam and must fail this same assertion (240 vs30) while correct masks remain equal. A counted helper invocation alone is insufficient: count real lane_read extraction operations. Preserve #238's independent predicate-call witness.

Finite executable gates: each new private test in debug/release with actual full module name and `--exact` (confirm one matching test); `cargo test --locked -p builtins --test input_liveness`; corresponding `--test input_liveness_mono`; `cargo test --locked -p host-core --test input_liveness_console`, all debug/release. Run builtins library tests in both profiles to retain #238 controls, affected strict builtins Clippy all-targets/all-features, fmt/diff and existing realtime/lane/workspace policy scripts. Existing named liveness cases include `the_settled_arm_leaves_the_ramp_words_untouched`, `the_trim_ramp_is_bit_identical_to_the_parameter_smoother`, `the_trim_ramp_is_partition_invariant`, `trim_and_polarity_do_not_overwrite_each_other`, and `a_banked_lane_ramps_exactly_as_the_same_track_alone`. Do not create new target matrices, test-support crates, permanent Cargo probes or allocator instrumentation: this helper's stack-only diff and retained render allocation gates suffice. Retain each actual command/status/count separately.

Production-native optimized-body inspection is bounded pre-edit evidence and can legitimately yield a lowering null. It must not become a blocker requiring a new probe framework; use the existing compiled production caller/IR and report any inlining limit. The source-level simplification and exact extraction witness remain the closable product, with no promise of fewer executed machine instructions. After source acceptance, existing immutable workspace/supported-target/current-artifact delivery rules apply separately; no descriptive benchmark authority is granted.

## Numbered queue checkpoint

GitHub #496 matches this title and spec. Branch codex/rt5-ramp-symmetry starts at delivered main fd4a937cbb782ebe04be0594f439606389dbe283. Root boundary inspection found no missing remote identities among 235 previously numbered local specs. This is queued after #475, not active implementation. Before assigning fresh Luna1, root must integrate then-current delivered main, verify scope/source equivalence and obtain Astra actual numbered-base approval. #238 stays delivered; this owns only the retained post-ramp extraction product. No timing or benchmark invocation is authorized.

## Astra numbered scope review

# Astra #496 numbered scope review — PASS, queued only

Reviewed clean head659bd19a83d6fd1236d8b5f35b8d7532899e296d in engine-rt5-ramp-plan, based on delivered fd4a937cbb782ebe04be0594f439606389dbe283. Only496-post-ramp-symmetry-extraction.md differs from that base. The entire approved refreshed brief body is preserved verbatim beneath the numbered title, followed by the explicit queue checkpoint. Root reports creation/synchronization and owns the final remote number/title/body verification before assignment.

The scope remains one private post-ramp whole-mask extraction helper and exactly two existing dual/mono call sites, with unchanged per-lane oracle, #238 addressed update, preparation/reset behavior, DSP arithmetic, layout and public API. Compact mask/PCM/state and isolated actual extraction-count proof, the same old-full-refresh control, existing liveness/gates and honest native lowering-null limitation are retained. No extra proof, matrix, framework, allocator or timing authority is introduced.

Approve numbered queue scope only. #475 remains the active runtime issue. Before future Luna1, root must integrate actual then-delivered main, verify source/scope equivalence, confirm remote identity and obtain actual implementation-base approval. #235/#238 remain delivered; #496 does not claim all RT5 bookkeeping or descriptive measurement is complete.

Read-only source/Git comparison; no tests/builds/timing or source/spec/Git/GitHub mutations performed.

## Current implementation-base readiness

Root integrated delivered mainad00d16b8ef8e3aa5ba4c406d00db4c62ff311b5 after PR502 closed475/499 and carried their closure record. Builtins/Cargo/config are byte-identical to scopedfd4a937c; no implementation has started. This becomes the next sole runtime feature after475 delivery, independently of500 graph-compiler maintenance. Await Astra actual numbered-base review before fresh Luna1 and bounded pre-edit native inspection. No timing or broader optimization is authorized.

## Astra implementation-base PASS and activation

# Astra #496 actual implementation-base review — PASS

Reviewed clean exact head 5be6737abdb017854e0c16a8655cb7757bf21d6a in engine-rt5-ramp-plan, integrated delivered main ad00d16b8ef8e3aa5ba4c406d00db4c62ff311b5 after PR502/#475/#499 delivery. Only numbered scope and closure documents differ from main. All crates/hosts/tools/scripts/Cargo/config match delivered main; builtins/Cargo/config also remain byte-identical to the originally scoped fd4a937c. The entire previously approved numbered #496 text is retained, followed by its review and current-base record. No source implementation has started.

Approve fresh Luna1 as the next sole runtime feature. #500 graph-compiler maintenance is independent. Begin with the frozen bounded pre-edit native optimized-body inspection; report inlining limits or an honest lowering null without a new probe framework or timing requirement. Then implement only the private stack-only whole-mask helper and two ordered post-ramp call sites. Preserve the unchanged per-lane predicate, preparation/reset, #238 addressed updates, bitwise comparisons/countdowns/padding, mirroring, layout, public API and DSP arithmetic.

The frozen finite proof remains unchanged: W1/W4/W8 compact oracle cases; actual dual/mono ramp PCM/state and subsequent retarget behavior; no helper invocation on settled blocks; isolated full-W8 at-most30 extraction observation versus old240 and the same intended assertion under the one actual old-full-refresh mutation. Keep oracle/report/settle work outside that observation. Existing liveness/console tests, proportional strict Clippy/fmt/policies and later immutable qualification remain required. No universal instruction/cycle reduction, C-3 false failure, additional matrix, allocator instrumentation or benchmark authority is introduced.

Root retains remote identity/body synchronization, checkpoint ownership and attempt escalation. Source acceptance and later current-artifact/actual-PR/required-CI delivery remain separate gates. Read-only source/Git comparison only; no builds/tests/timing or source/spec/Git/GitHub changes performed.

Root activates fresh Luna attempt 1 on the approved production base. The separately discovered compressor allocation-proof issue #503 now has source and exact-PR Astra PASS plus authentic local focused/full/DSP integration PASS; its PR504 awaits required CI. Its changes are confined to compressor tests/dev-dependency, so this builtins runtime slice is independent. Root retains Git/checkpoint ownership and no timing is authorized.

## Luna attempt 1 candidate and retained evidence

Source deb73c0c adds the whole-mask helper and two real post-ramp call-site replacements. Root checkpoints preserve implementation and test milestones. Raw records in artifacts/issue496-luna-attempt1 include the actual old-refresh mutation failing the extraction assertion at240 versus30, restored passing tests, initial failures and existing-consumer/profile/policy commands. The report filename typo and early dirty-HEAD provenance limitations are candidly documented. Consolidated Astra review must verify all frozen PCM/state and adversarial gates; no delivery or timing claim is made.

## Astra attempt 1 FAIL and bounded Sol attempt 2

# Astra #496 Luna attempt1 consolidated review — FAIL, one bounded proof group

Reviewed clean exact head 7a91fecaf1c8693a1add6dcf8724edb4a7af9884 in engine-rt5-ramp-plan, final source deb73c0c. Production implementation is accepted pending completion of the frozen finite mask proof below; this is one consolidated attempt1 verdict, not permission for more Luna revisions.

## Accepted source and evidence

The private helper faithfully changes traversal of the unchanged predicate: bitwise trim/target/step, integer countdown, and all six coefficient words in both sections. It uses a bounded u8 candidate mask, two extracted arrays per pair, handles member/width padding and early exhaustion, and writes only symmetry. Exactly the two post-ramp calls replace full refresh, after settle and after settle+mirror respectively. Preparation/reset, addressed-lane refresh, predicate oracle, DSP arithmetic, layout and API remain unchanged. No production defect is identified by this read-only review.

The actual dual-seam temporary old full refresh fails the same extraction assertion at240 versus30, after the semantic mask comparison; restored exact debug/release tests pass. Observation brackets the real refresh call only and excludes settle/report/oracle work. Mono extraction and settled dual non-invocation are also demonstrated. Do not add another mutation campaign or claim universal machine instruction savings.

The existing integration evidence DOES contain real nonzero PCM and exposed-state proof, so it must not be dismissed merely because those files did not change. input_liveness_mono::a_symmetric_ride_through_a_collapse_renders_never_collapsed_bits runs ten real blocks with an in-flight trim ride, compares against the never-collapsed arm, desymmetrizes, compares both later planes, and compares retained integrator/ramp words. The existing drain/disengage fixture applies later asymmetric trim/polarity commands and checks the decline and subsequent actual PCM/state. Those execute W4/W8 and cover the real changed mono/dual seams. Their never-collapsed arm is not an old-implementation oracle; the separate unchanged per-lane predicate remains essential for mask correctness. Passing unchanged tests are credited for what they actually exercise, not as blanket coverage of every new helper field case.

Retained library debug/release7 each, exact helper tests1 each, input_liveness13 each, mono8 each and host console10 each execute nonempty populations. Strict affected Clippy/fmt/diff and corrected explicit-bash policies have successful final records; initial PATH/permission/test failures remain candidly preserved. The retained pre-edit extraction contains15 complete actual predicate/caller IR bodies and input provenance, not a standalone module or a new target matrix. Independently verified all113 manifest payload hashes/sizes and exact tracked114-file coverage.

## Sole bounded correction group: non-masked complete predicate cases

The new differing-words test does not fulfill the frozen field/width/partial/re-equalization table. It cumulatively poisons trim lane0, target lane1, step lane2, only section0.c1 lane3, then countdown lane4. Earlier cleared candidates never test later comparisons. W1 therefore tests only signed-zero trim; W4 tests no countdown; W8 uses five members and ends with all candidates cleared. None of a2/a3/m0/m1/m2 or the second section has an independent unequal witness, and no helper re-equalization restores a bit. Full/partial W4 coverage is absent in that adversarial helper table. An implementation omitting most coefficient comparisons could pass these tests.

Sol2 should correct the existing compact cfg(test) fixture only: start each predicate-field case from a valid equal stage, perturb one active lane's compared field, assert the independently expected bit clears as well as full equality with the unchanged predicate, restore that field and assert the bit returns. Cover trim/target/step/countdown and each of six words in each of the two sections; keep an unaffected member when width permits so later comparisons are not hidden by an empty candidate mask. Exercise W1/W4/W8 and representative full/partial W4/W8 populations; retain bitwise signed-zero and explicit padding-false checks. A small direct field-case helper is sufficient, not a Cartesian corpus or new framework. No extra production field/mutation, new public API, allocator or timing requirement.

Retain the actual seam extraction control and already executed PCM/state/disengage fixtures. Re-run the corrected exact test and affected existing finite debug/release/library/liveness gates with final source identity and actual statuses, plus proportional Clippy/fmt/policies. There is no separately demanded new PCM harness: if those existing fixtures are kept unchanged, their named source/behavior attribution above supplies the requested compatibility evidence. Parent immutable qualification/current-artifact/actual-PR/CI remains after source acceptance.

No tests/builds/timing or source/Git/GitHub mutations performed during review.

Root integrated delivered main71059eab and closure records before Sol2. Builtins remains byte-identical to reviewed7a91feca. Source/actual-seam mutation and existing PCM/state fixtures remain accepted; only the finite independent predicate-field table is assigned to Sol2. User workflow escalation applies; no further Luna revision is authorized.

## Sol attempt 2 correction and evidence

Source5a4f4147 changes only the independent field-case fixture. It clears/restores each compared field from equality against expected bits and the unchanged per-lane oracle across required widths/partial populations. All exact final-source private tests and finite debug/release library/liveness/console gates pass, with strict Clippy/fmt/diff/policies. Authentic records and initial compile failure are retained in artifacts/issue496-sol-attempt2. Accepted production and original actual-seam mutation remain unchanged. Consolidated Astra review pending.

## Astra Sol attempt 2 PASS and immutable delivery qualification

# Astra #496 Sol attempt2 consolidated review — PASS

Reviewed clean exact head ddcdfeacb5c06777139cc337679907a99b8db2f7 in engine-rt5-ramp-plan, source 5a4f4147af8d1c0263128cec0474881b783a6d77. Only the compact cfg(test) field fixture changes after attempt1's accepted production; no source delta follows the final source checkpoint.

The sole bounded proof gap is fixed. Each trim/target/step/countdown and each of six coefficient fields in both sections now starts from a fresh valid equal stage. The independently specified active mask is checked, one lane's field is toggled, its exact bit must clear while any unaffected member remains set, then the same field is restored and the exact bit must return. Every stage is additionally compared with the unchanged per-lane predicate. No later field is hidden by a cumulative earlier mismatch. The finite population covers scalarW1, W4 full/three-member partial, W8 full/five-member partial. Separate +0/-0 trim mismatch and restoration are bit-sensitive; padding stays false. This satisfies the original field/width/partial/re-equalization scope without an expanded corpus.

Retain acceptance of the private stack-only helper and its two ordered post-ramp calls, unchanged preparation/reset/addressed updates, predicate, DSP arithmetic, layout and API. The actual old-full-refresh mutation and original/restored 240-versus30 same assertion remain unchanged in attempt1 evidence. The honest15-body native baseline observation is not a new instruction-count or speedup claim. Existing nonzero mono/dual ramp, never-collapsed PCM/state and subsequent retarget/disengage proof remains credited as detailed in attempt1 review; no redundant PCM harness is needed.

Final authentic metadata identifies committed source, source blob52ffe66d04369d89a47ce240a9a620c1a5c7d5c3 and SHA256 c21b54c7147b32601daf82d54d698beca68425019753bf134330ec37e5feb4a9. Four exact private tests pass one each debug/release; library7 each, liveness13+mono8 each and host-console10 each pass. Strict affected all-targets/all-features Clippy, fmt/diff and realtime/lane/workspace policy final statuses are0. The initial compile failure remains separate. Independently verified50 payload hashes/sizes and exact51-file tracking including manifest; the accidentally included root checkpoint comment is retained as a comment, not execution evidence.

Approve source acceptance. Root may integrate actual delivered main, verify accepted builtins/fixture identity and freeze the immutable workspace/supported-target/nativeABI qualification and ordinary artifact build. No stale artifact digest may be promoted: a real current-source mismatch requires the bounded current-pin/current-consumer ruling with unchanged numerical expectations. Exact-head actual PR review and required CI remain delivery gates. No timing, additional mutation campaign or universal runtime performance claim is authorized.

Read-only source/evidence review; no tests/builds/timing or source/spec/Git/GitHub mutations performed.

Root adopts PASS and freezes this delivered-main source for immutable workspace, scalar/SIMD Wasm and native C ABI checks, alongside the ordinary worklet builder. No further source edits occur during these checks. #505 post-main qualification is also verified SUCCESS and its closure note is carried here. Any actual artifact mismatch receives a separate bounded ruling before pin update/consumer checks. No timing authorized.

## Immutable delivery checks and actual artifact ruling

Frozen94e1f3c3 completes workspace tests, scalar/SIMD Wasm target checks, native C API release build and ABI checks with individual exit0 and final clean-source assertion. Transcript result counts are recorded separately without a unique-test claim. The ordinary worklet builder compiled successfully and exited1 on actual expectedbed7d77cbd55e91f38679d4ed1ea6d99684ad12d8a1a0f785049d788630f17ee versus observed25c1b72a65ebfd081c74d431614cfba42492e95e490cc4d7c203ee14fe8737e9. All immutable steps finished before this pin mutation.

# Astra #496 artifact integration ruling — approved, bounded

Reviewed clean frozen94e1f3c3e4edd7d6ffd9221e394d877f80c7e162 in engine-rt5-ramp-plan. Builtins source is byte-identical to accepted Sol2 source5a4f4147. The actual ordinary builder metadata identifies this exact source, output/tmp/engine-496-qualified and target/tmp/engine-496-artifact-target. Its raw log records successful release compilation followed by numeric builder exit1: expected bed7d77cbd55e91f38679d4ed1ea6d99684ad12d8a1a0f785049d788630f17ee, observed25c1b72a65ebfd081c74d431614cfba42492e95e490cc4d7c203ee14fe8737e9.

Approve only the exact observed pin checkpoint and the existing seven-step qualification: ordinary verified rebuild, static/object/ABI checks, resource gate including26 rejection controls, hermetic worklet, npm installation, current three-browser qualification plus existing self-tests, and generated matrix check. Verify the published module bytes/SHA against that exact pin and all current records. Candidate/hash identity records may change; canonical/PCM/resource numerical expectations, corpus pins, schemas, scripts/CI/lints and runtime source remain frozen. Any numerical or additional source discrepancy requires its own concrete ruling; do not repin expected numerical output to obtain success.

The immutable workspace/supported-target/nativeABI sequence is still running on this source. Do not edit its tracked worktree until terminal, including pin/spec edits. Root may wait, or use a separate delivery checkout with explicitly verified source identity and isolated build target while preserving the immutable run. This ruling does not claim those pending gates passed or independently hash an artifact the mismatch builder has not yet published.

Preserve the authentic mismatch command/log/status and source lineage. Following all mandatory terminal gates, final actual-PR exact-head Astra review and required CI success remain necessary. No timing, benchmark authority, new qualification matrix or unconditional production change is authorized. Read-only review; no builds/tests/source/spec/Git/GitHub mutations performed.

## Current artifact consumers complete

Pinned candidatee9a6519e passes all seven existing artifact/consumer steps with individual exit0. Published module independently hashes25c1b72a65ebfd081c74d431614cfba42492e95e490cc4d7c203ee14fe8737e9. Generated browser records differ only in candidate/hash identity; canonical/PCM/resource expectations remain unchanged. artifacts/issue496-integrated-delivery retains immutable and consumer execution, actual mismatch and exact module identity. Source-level full-W8 refresh observation is240 to30 extractions; no timed improvement is claimed. Actual PR review and required CI remain mandatory.
