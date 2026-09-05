# Astra #427 Luna attempt 1 review — FAIL

Exact source checkpoint: `cf5aa81d9df34d38f5f44ac93e14b531e9e7166f`, `/home/bl/misofm/engine-404-plan`.

FAIL. Four finite groups below prevent source acceptance. They complete the original frozen three-family/status/causal-proof contract; no new framework, opcode policy, target matrix or workspace-policy work is needed. Luna attempt1 is consumed; Sol attempt2 may correct these coherently, with only one further retry thereafter.

Read the full #427 contract, frozen-base approval, checker, complete new suite, CI placement and retained root suite result. Root's suite exit0 is real, but the suite does not establish the claimed finite contract. Real Cargo qualification remains pending after source PASS; the corrected PATH/toolchain record is candid and is not a waiver.

## 1. Real archive identity is wrong and the fake producer repeats the mistake

The checker at line27 searches `lib${family//_/-}-*.rlib`. For target_smoke this becomes `libtarget-smoke-*.rlib`, while Cargo's library is `target_smoke` (`crates/target-smoke/Cargo.toml:10`). The retained actual-toolchain evidence in `artifacts/issue435-qualification/sol-435-nonlto-inspection.log:10–12` names `libtarget_smoke-f8895a03d6c8d18b.rlib`. The real gate therefore cannot satisfy its third family even after a successful build.

The fake Cargo wrapper uses the same underscore-to-hyphen replacement and manufactures the wrong name, making the synthetic positive green. Use exact library family names engine/source/target_smoke in both real discovery and the faithful fixture. Keep Cargo package argument target-smoke unchanged. No new build is needed to establish this defect; real-toolchain qualification follows correction/source PASS.

## 2. Finish checked population reconciliation and duplicate detection

The duplicate check at line38 compares unchecked wc command substitutions, one containing `sort -u | wc -l`. A failed sort/wc can be interpreted as equal counts or a duplicate predicate instead of an execution error. The realpath command substitution appended into an array at line31 also loses the producer status. Check each actual producer separately, or remove those incidental producers with a simple bounded equivalent; do not rely on errexit inside these conditionals/substitutions.

Current reconciliation only checks that each listed `.o` exists after extraction. It never requires the archive listing's object set to be nonempty or proves reverse equality with the discovered objects. A listing that omits a real extracted object still permits that object to be accepted, and an extracted extra `.o` is accepted without membership reconciliation. Derive the complete expected object-member set from the checked listing; require it nonempty per family, reject duplicate OBJECT member names, and compare exact expected/extracted/discovered identities before decoding. Do not broaden the policy into rejecting arbitrary duplicate non-object metadata names. Retain separately checked object find/sort and every decoder invocation over the reconciled population.

## 3. Preserve execution status and diagnostics, and complete required searches

Many `if ! producer; then` arms lose the original status and print only a generic message. cfg, ar listing/extraction and decoder stderr are redirected to scratch files but never emitted; cleanup unconditionally deletes that scratch. Retaining only the build target does not preserve the failed decoder/listing diagnostic. The frozen contract requires operation/status/identity and original error evidence. Capture original exit statuses explicitly, print the relevant stderr (and appropriate partial output) before exit, and do not certify or discard a failure merely because a plausible list/decoded body exists.

cfg predicates and the source ObservationSlot fallback still use `rg -q`, despite the approved no-quiet/pipeline-ambiguity requirement. Use completed nonquiet searches with discarded/captured stdout and explicit 0/1/>1 classification. Keep pointer capability presence and atomics-feature absence distinct, source fallback optional only after complete clean object no-match, and the existing required binary-symbol pattern/options. No new decoded-text grammar: successful empty decoded output remains allowed. Opcode regex remains `atomic\.`.

The current loop does continue observation searches after an earlier match and already distinguishes their returned error statuses; preserve that accepted behavior. Corrected failure records must remain observable after cleanup.

## 4. Finish the original finite hermetic table and execute the SAME assertions against verified mutants

`run_case` currently treats ANY nonzero checker exit as a passing negative. It neither checks the selected operation/status/sentinel nor distinguishes setup failure. This already conceals a concrete wrong-site case: `opcode-scan-error` matches any argument containing `atomic`, so it fails the earlier cfg pointer-atomic search rather than the decoded opcode scan. Target the intended operation/arguments and, for late cases, let all earlier calls execute correctly.

The nominal `no-observation` positive still emits default `observe clean` objects; it never enters source fallback. Add a genuine all-object-no-match plus source-match positive, and prove a successful object match does not consult a failing/missing source fallback. The stale fixture is created under `CARGO_TARGET_DIR/stale`, inside the fresh child, rather than outside it in the caller's existing parent cache. Put the pre-existing stale archive/sentinel in that caller-owned location and verify it survives; do not claim a nested disposable child file proves preservation of the parent cache.

The existing suite also omits the frozen paired archive/object sort/discovery failures, partial member listing/extraction, missing extracted member and reverse-reconciliation controls, actual duplicate object member case, empty/error decoder variant, and a late observation error after an earlier match. `duplicate-members` has unused producer code but is never invoked and does not form duplicate object members. Complete these named original cases with representative parameterized wrappers, not a second corpus. For cfg/list/discovery/decoder failures pair empty/error with otherwise-valid complete-looking output/error where applicable; ar metadata and valid clean disassembly must not themselves trigger unrelated rejection. Allow successful valid empty decoding. Saved real tools must preserve their own failure status when not selected—the current find wrapper overwrites delegated find status with zero.

Both counter blocks directly run a mutant and EXPECT checker success, then print that the “focused assertion rejected” it. They never execute `run_case` or its original failure assertion against the mutant, never require a distinguished unexpected-success assertion status, and do not verify that each source edit matched uniquely. That is not the required same-assertion proof.

Make the ordinary targeted assertion reusable with an explicit checker path. Require the unmodified checker to fail for the injected operation/status/sentinel, then execute THAT SAME assertion against each uniquely verified actual production mutant: one swallowing the decoder error with valid clean partial output, one swallowing a complete-looking population producer error. The mutant must make the checker succeed, causing the same targeted assertion to exit at its named unexpected-success branch (for example97). A setup, syntax, missing-diagnostic or tool error must have a different outcome and cannot count. Preserve exact edit/diff and executed results; do not replace these two bounded controls with a general mutation framework.

## Accepted scope and next step

The fresh gate-owned child target, explicit scalar flags and CARGO_PROFILE_RELEASE_LTO=false are the correct approved inspection strategy. Parent target/cache ownership is not intentionally deleted. The CI change is confined to the existing scalar atomics step's suite call and corrected non-LTO comment; the separate eighteen-package build and remaining job/router gates stay intact. Naming success as scalar NON-LTO engine/source/target_smoke is correct once the actual populations are proven. The checker/suite executable modes are appropriate.

After one coherent Sol correction, run syntax/diff, the hermetic suite and relevant policies; return the exact source/evidence checkpoint for one adversarial verdict. Only after source PASS run the actual available toolchain gate once for the required real non-LTO proof. No shipped AudioWorklet rebuild, benchmark or #404 workspace-policy edits belong here. #404 retains all workspace remainder obligations; #306/#349 remain open through their own accounting.

This review made no source edits, builds/real-gate executions, timing or Git/GitHub mutations. Only this `/tmp` verdict was written. Findings above derive from actual source and retained records; no green suite label or authority substitutes for the missing controls.
