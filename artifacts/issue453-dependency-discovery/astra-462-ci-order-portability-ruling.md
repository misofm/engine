# Astra #462 CI discovery-order portability ruling

PR464 head `f3e4e2a0ab457266a73180235878f0bd70ae6d32` is NOT mergeable after required CI failure. Preserve `/tmp/engine-464-policy-ci.log` and job101296549269. Prior exact-head PASS was conditional on required qualification; it cannot override the observed test failure. Production checker passed; no production defect or policy change is established.

Confirmed affected assumptions in test-bench-policy.sh: escaper-candidate-grep-error requires effect_interchange.rs immediately after `output:`; unsafe-owner-grep-error requires capi.rs immediately before `; stderr:`; environment-reader-grep-error similarly requires main.rs last. Recursive grep discovery order is filesystem-dependent. All three need the same bounded correction. Sorted-list diagnostics use explicit LC_ALL=C sorting and do not need their deterministic order weakened; single-owner diagnostics are not affected.

## Approved final Sol3 amendment

Amend #462 to include only these benchmark-suite diagnostic portability corrections and corresponding #462/#453 evidence. Existing production checkers, helper, Cargo resolution/flags/delegates, original four execution-error cases, other policy semantics, two actual production mutants and restored-positive behavior remain frozen. This is the final #462 Sol attempt3; it does not reopen stopped #453 or reset counters.

Keep distinct97 unexpected-success versus96 setup/wrong diagnostic. For EACH of the three multi-file grep/error assertions require all of:

1. The exact operation and captured tool status7, with the case's exact stderr sentinel. No generic grep-error or arbitrary nonzero acceptance.
2. The COMPLETE captured stdout payload, not one convenient first/last filename. Capture the real selected grep's stdout/status separately in the disposable shim before injecting the failure. Require real success0 and retain its full output; unselected calls delegate unchanged. Replay that output then the injected stderr/status7.
3. Parse the diagnostic's bounded `output:` field up to its `; stderr:` delimiter and compare it byte-for-byte to the full replayed payload (account only for the existing command-substitution trailing newline convention). Do not sort the whole combined diagnostic or let filename membership anywhere in stderr satisfy the proof.
4. Confirm completeness independently against the existing allowed fixture population: exactly the three escaper candidate paths, six unsafe owner paths and two environment reader paths, respectively, using the current accepted source's sets. Compare sorted lists without uniquing away duplicates; missing, extra and repeated entries must not pass. This set check handles permissible enumeration order while exact diagnostic/payload comparison proves no captured output was dropped. Derive the expectation from the accepted real fixture or its already-frozen owner sets, not from a weakened predicate or a second arbitrarily truncated list.

A tiny local assertion helper in the existing suite is sufficient. No new generic framework, shared helper or scanner formatting redesign. Preserve all existing error-only counterparts. Existing deterministic sort-specific rows remain as they are.

## Targeted reversed-order proof

Exercise each of these same three full-payload assertions twice on otherwise valid existing fixtures: real discovery order and its reversed order. The second shim must reverse the COMPLETE saved real result, not inject a different population; require more than one line and actual order difference. Both should produce the intended status7 refusal and satisfy the same complete-output comparison. This is a permutation of the existing directed producer failure, NOT a third production mutant campaign.

Retain the actual selected producer result/status, replayed order, diagnostic and suite status sufficiently to review the three normal/reversed pairs. Verify each asserted set is complete and unchanged; preserve exact operation/status/sentinel checks and97/96 behavior. This proves the CI failure is corrected even on the author's filesystem, without relying on directory creation order.

Run syntax, actual benchmark policy scanner and its affected suite with retained statuses/logs, plus diff hygiene. Run the existing audit suite once to retain portable-Cargo and both97/restored0 controls if the final combined delivery gates need refresh; do not modify those tests or repeat a relocation campaign absent a change. No Rust source changed, so the already-completed full workspace evidence remains applicable and no new build is needed for this assertion-only correction. Root will rerun required PR CI on the final reviewed head; do not force green or merge the failed run.

Retain the failed CI evidence and candidly supersede the earlier PASS/qualification-readiness record. Root synchronizes this precise amendment before assigning Sol3, checkpoints the coherent correction, then obtains one final consolidated Astra source verdict and exact-head PR review. Another FAIL means hard stop/rescope, not informal fourth correction. #453/#462 stay open until actual corrected delivery; #403/#306/#349 remain open for their other obligations.

Read-only source/CI-log inspection only; no tests/builds/timing or repository/Git/GitHub mutations were performed.
