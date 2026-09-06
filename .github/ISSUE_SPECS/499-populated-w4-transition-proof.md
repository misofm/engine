# 499: Prove compressor W4 state transitions with populated delayed history

Delivered via PR502; GitHub issue CLOSED. Historical attempt records follow.

This is a bounded proof-completion child of stopped #475, based on preserved source019ac62a / reviewedce745d8a and parent hardstop8c52c832. Parent475 keeps its full original product acceptance and delivery obligations. Its three attempts are exhausted; this child must be numbered/synchronized and scope-approved before a fresh Luna1 pass, then Sol2/3 only after a consolidated FAIL. No further parent repair or qualification is authorized before child PASS. #498 is independent maintenance.

## Sole remaining problem and exact scope

The accepted cfg(test) W4 owner fixture calls real production restore/reset/mono/copy/process/snapshot methods, but currently renders at most256 samples from cleared rings. At48k the main latency is960 and selected detector delays are960/840/720/600. Its PCM and detector-read comparisons therefore see startup silence. The missing claim is meaningful next-render behavior after state transitions, not construction, factory admission, access arithmetic, allocation safety or object identity.

Change ONLY the existing cfg(test) fixture in crates/compressor/src/kernel.rs, plus numbered spec/evidence. Reuse bank4, payloads, restore_payloads, render4, initial_values/population and the existing old_detector_word helper as appropriate. No production change, lib.rs/interface/backend override, new feature/harness/test crate, allocator, corpus/pin, object rebuild, G5 rerun or additional mutation is requested. Preserve all accepted earlier475 source/tests/evidence and marker correction. Do not broaden into a new DSP or target matrix.

## Freeze the finite phases

Use the existing128-frame W4 render helper with a deterministic finite, mirrored, frame/lane-varying input sequence. Make the block/frame origin explicit so consecutive blocks write distinguishable histories, rather than accidentally repeating one block. Keep signed-zero representatives but do not make the comparison all zero. If current default threshold makes the selected detector histories non-discriminating, choose existing validated finite threshold/ratio inputs in this fixture only; no parameter law changes. Use identical inputs and sample times for candidate/reference owners.

1. Both restore directions: prime the source owner through exactly8 consecutive128-frame blocks (1024 frames), exceeding the961-row ring and960 latency, before snapshot. Cover ragged source→uniform-default destination and uniform source→ragged-default destination using existing populations. Validate the source's actual main/detector history has populated frame/lane-varying words; check with original tap indexing that the distinct uniform/ragged delay choices select different live detector words for the affected lanes at the next cursor. Do not count a stored delay value alone as proof of different accessed data. Restore the saved payload into the destination using the real trait API. Obtain the uninterrupted source's next block as expected; BEFORE candidate PCM equality assert expected output has nonzero words on both planes and distinguishable frame/lane values. Render the destination's FIRST post-restore block at the same sample time and compare both complete PCM bit vectors and every lane's complete serialized state. No post-transition warmup may replace the first-render assertion.

2. Mono copy/reopen: prime the existing asymmetric owner (uniform left/ragged right) and the uniform dual oracle through the same8-block sequence. Install asymmetric state through actual per-track restore, preserving the existing test-construction explanation. Validate populated/distinct detector choices before the transition. Render the next real mono block versus the uniform dual reference with mirrored input; compare left PCM and assert meaningful expected output. Invoke desymmetrize_channels, then compare all copied state as already required. On the FIRST subsequent dual render, compute and assert nonzero/distinguishable expected PCM before candidate equality, and compare both planes plus all serialized lane state. This covers a populated copy boundary rather than cleared-ring equality. Do not change public native-W4 rejection; this remains the approved private W4 state-owner execution seam.

3. Full reset: preserve the legitimate first128-frame silence and complete state comparison against a fresh bank with the same ragged preparation defaults after restoring uniform state and calling FullToDefaults. Assert first-block zero explicitly; it is expected, not evidence of populated output. Continue both owners for exactly8 more128-frame blocks with the same deterministic varying inputs (total1152 frames). Compare PCM/state through that fixed continuation and on the final full post-latency block assert expected output contains nonzero/distinguishable words before candidate equality. Confirm reset-default delay choices access the actual populated history. This adds no reset semantic requirement: it exercises the existing one through the point where delayed output can discriminate it.

Keep the existing independently decoded/native/G5/access-oracle proofs as separate evidence; no new instruction-count or timing claim follows. The old tap-index helper is only an assertion oracle outside processing, never a replacement renderer. Payload comparisons must remain complete, not counts, selected headers, or a new serializer. No broad property framework or Cartesian population is needed: the two restore directions, one mono/copy sequence and one reset sequence above are the whole child.

## Objective gates and delivery boundary

The corrected existing `kernel::tests::w4_state_owner_transitions_drive_the_next_real_render` must execute exactly once in debug and release with `cargo test --locked -p compressor --lib kernel::tests::w4_state_owner_transitions_drive_the_next_real_render -- --exact` and release equivalent. Retain actual expected-population assertions in source, meaningful outputs/state proof, exact command/cwd/source/log/numeric status. Run compressor library tests debug/release to preserve the accepted old-access/branch witness, affected strict compressor all-targets/all-features Clippy, fmt/diff and existing realtime/lane/workspace/environment policies. No unnecessary integration/allocation/G5/object reruns solely for this cfg(test) change; retain their accepted identities and evidence. Root checkpoints coherent tranches promptly and owns all Git/GitHub writes.

After one consolidated child source PASS, parent475 may freeze integrated current main and complete its retained immutable workspace/supported targets/current artifact delivery qualification, exact-head actual PR review and required CI. Child PASS alone neither closes the full product nor claims measurement. If the bounded child also exhausts three attempts, stop/rescope again under the standing rule; no disguised fourth parent attempt. No implementation or benchmark is authorized by this unnumbered draft.

Prepared read-only from accepted source and final review; no tests/builds/timing or repository/Git/GitHub mutations performed.

## Numbered bounded successor

GitHub499 title/number match this stateless child. Branch codex/475-populated-w4-proof is isolated from preserved parent hardstop8c52c832; accepted implementation source remains019ac62a. Parent475 remains open and stopped pending this child. #498 is independent graph maintenance. Before fresh Luna1, Astra must confirm this numbered scope/base; no correction or delivery qualification is authorized by numbering alone.

## Astra numbered approval and assignment

# Astra #499 numbered scope/base review — PASS

Exact clean head9c10b5689d615abc1de4b4ad7cde29a3ffbb5af2 in engine-475-populated-proof, isolated from parent hardstop8c52c832. Only numbered499 spec and reciprocal475 retention documentation changed. Crates/hosts/tools/Cargo/config are byte-identical to preserved019ac62a. The complete approved populated-history brief body is retained verbatim under its numbered title; root reports synchronized remote499 identity/body.

Approve fresh Luna1 for this proof-only child. The two restore directions and mono/copy sequence must use populated varying histories before the first transition render, with nonzero/distinguishable reference PCM and distinct actual delayed detector history before candidate equality. Full reset retains correct initial silence/state and then a fixed continuation beyond latency. Only the existing kernel.rs cfg(test) W4 fixture and evidence may change; private actual state-owner construction remains the approved seam, not native factory admission.

No production/API/backend, allocator, corpus/G5/object, new harness, mutation campaign or timing change is authorized. Existing finite focused/library/Clippy/policy gates remain unchanged. Parent475 remains stopped and open with full original product/delivery obligations; this is a separately numbered fresh attempt, not a fourth parent correction. After child source PASS root may integrate actual current main, verify source identity and complete parent qualification followed by actual-head PR review and required CI. Numbering does not itself claim implementation or delivery.

Read-only source/Git comparison; no tests/builds/timing or source/spec/Git/GitHub mutations performed.

Root assigns fresh Luna1 to this bounded proof child. Root owns exact-path checkpoints and GitHub. Parent475 remains stopped until child PASS; no delivery qualification or timing is authorized during implementation.

## Luna attempt1 final candidate

Source90bea997 completes the frozen8-block priming, same-lane populated alternate-tap checks, meaningful first post-transition PCM/full payload comparisons, and fixed reset continuation. Final focused debug/release1 each and library debug/release2 each pass. Root independently captures each strict Clippy/fmt/realtime/lane/workspace/environment/diff gate at this committed source with numeric0. The artifact package preserves all earlier logs and candidly corrects Luna's Git-blob-as-SHA256 label and inherited-PATH metadata. No new production, numerical, object/G5, mutation, timing or delivery claim. Consolidated Astra child review remains pending.

## Astra child source PASS

# Astra #499 Luna attempt 1 — PASS

Exact clean head05c73e7ef244805816cd270d711b3d46455d2912, source90bea9972cc452e140de580e9a78b08a8eb19f4f, engine-475-populated-proof. One consolidated source verdict against the full numbered proof child and retained parent475 contract. No tests/builds/timing or source/Git/GitHub mutations performed.

The sole remaining populated-history defect is closed. The existing W4 fixture now primes8×128 actual frames, with explicit absolute frame origins and varying lane/frame values, before both restore directions and asymmetric mono/copy transitions. It verifies actual main/detector population and, for each affected same-lane alternative delay, finite nonzero unequal accessed history using the independent old tap indexing. This is no longer a test of stored delay metadata or cleared history alone.

The uninterrupted reference's first post-restore output is checked for finite nonzero and numerically distinct words on each plane before complete candidate equality, including every serialized lane payload. Mono processing retains the mirrored-input precondition; populated left output is compared, copied state is checked after desymmetrize, then the FIRST dual post-reopen output/state is compared against a meaningful expected block. Full reset explicitly retains first-block positive-zero PCM and exact fresh-owner state, then exactly8 further blocks reach meaningful output beyond latency, with complete comparisons throughout and populated reset-default histories. No warmup after restore/reopen substitutes for the first transition render.

All changes are inside the existing cfg(test) fixture. Production specialization, public native-width rejection, private W4 owner construction, original access oracle/actual branch mutation, W1/W8 transition evidence, isolated allocation proof and accepted native/Wasm/G5 evidence remain untouched. No additional mutation, object/G5 execution, API/backend change or numerical expectation was introduced. The parent hardstop was respected through a separately numbered child, not a fourth parent pass.

Verified all19 manifest payload hashes/sizes and exact20-file tracked coverage including manifest. Final focused test executes1 in each profile; library executes2 in each profile, all numeric0. Final log identity6d6c8b338b7bb03e060c5b3a46c882fab0689350 is a Git blob, not SHA256; root's separately attributed record correctly identifies source90bea997 and actual SHA2560e4d84c4bb2b13133f3dd67fbb1d26045777a81126805baeddab1d072c86b145. Original mislabeled logs and earlier PATH/combined-exit provenance remain candid. Root's independent seven committed-source strict-Clippy/fmt/realtime/lane/workspace/environment/diff invocations each have actual0 results. No source changed after the qualified fixture checkpoint.

Parent475 may now integrate actual current delivered main, verify accepted compressor/source identity and freeze the combined source for its retained immutable workspace, supported-target and ordinary artifact delivery qualification. Do not promote stale standalone artifact identities; an actual combined mismatch needs the bounded current-pin/current-consumer ruling with unchanged PCM/resource expectations. Retain all original parent evidence and limits. Child source PASS is not completed parent delivery: actual-head PR review and required CI SUCCESS remain merge gates. No timing authority or measured gain is granted.

Root integrated delivered main95abdd015e28823905800d051d03837255d91612 and verified the entire compressor crate is byte-identical to reviewed05c73e7e. Parent delivery qualification now proceeds on this combined frozen source. Child remains open until its evidence/product delivery is merged and remotely synchronized.

Delivery continues with parent475 in separate codex/475-compressor-tap-delivery checkout from1c71000d; the immutable source qualification checkout remains unchanged. Parent spec records the actual integrated artifact mismatch and bounded ruling; no child/production proof requirement changes.

## Integrated delivery qualification complete

Frozen1c71000d passes immutable full workspace (transcript277 blocks/1639 passed/0 failed/24 ignored, including isolated child result), supported scalar/simd128 checks, native release C API and shared/static ABI. Separate delivery candidatede410134 passes verified artifact/static/resources26/hermetic/current3browser+self-tests/matrix; modulefa78dc8d3f0d391b94419f5252504eee2aafbbf13853884157e24935ed092cc3 is independently hashed. Complete command/source/output/numeric status and source equivalence are retained in artifacts/issue475-integrated-delivery. No production/numerical change after source acceptance. Actual-head PR review and required qualification remain before merged closure of475 and499.

## Delivered and remotely closed

PR502 merged at2026-09-06T05:39:25Z asad00d16b8ef8e3aa5ba4c406d00db4c62ff311b5. Astra exact-head PASS and required qualification34014259495 SUCCESS both apply to13fd9d1b6e3df7cc9005d0eef1b9e0723a3b0a1e before merge. GitHub475 and499 are verified CLOSED. This delivers the uniform compressor detector-access DYN1 product and its populated W4 proof; no measured speedup or other-effect closure is claimed. Post-main qualification is monitored separately.
