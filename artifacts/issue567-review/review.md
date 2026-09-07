# Issue 567 attempt 1 adversarial review

Verdict: **PASS for source and independently verified objective gates**. Correct the evidence attribution described below before delivery; the two mismatched direct captures receive no exact-command provenance credit.
Reviewer: Astra LOW. Historical #542 verdicts remain unchanged.
Reviewed candidate: `6d72e9fcb6f16708b548621ab0fec4116cacae5c`.
Inherited #542 head: `546e003d8bddd42c291dd6617f324ef42782c95b`.
Current origin/main: `b95c9b7b028ed07cfea2f7669467689c05320c37`.
Date: 2026-09-07 UTC.

The source delta consists only of additions to the authorized fixture script. The workspace sort shim recognizes manifest-path input containing both engine and protocol Cargo.toml paths, which earlier TOML dependency-name lists cannot match. It does not count invocations. Empty and partial-output failures require the exact workspace manifest `gate_sort_lines` diagnostic. Existing engine/conformance TOML sort controls remain separately live and require their own diagnostic. All previous fixture guards, dependency union, find/awk/paste/rg controls, exact-child controls and the module counter-mutant remain unchanged.

The new in-suite scratch mutation really changes only the copied `gate_sort_lines` error branch to return status zero; the expected downstream wrong-diagnostic check prevents a no-op mutation or unrelated early failure from receiving credit. The complete ordinary fixture suite independently exits 0 and rejects both the workspace-sort and module counter-mutants. The production checker and shell syntax independently exit 0. Base-to-head and working diff checks exit 0.

Independent direct proof: this review made a separate temporary helper copy, asserted exactly one replacement within `gate_sort_lines`, changed `else rc=$?` to `else rc=0`, and ran the candidate suite against that scratch checker/helper. The suite exits 1 at `wrong conformance diagnostic for workspace library manifest discovery sort errored (sort status 8): conformance boundary failure: no workspace library names found`. The TOML controls succeed before it. Exact executed argv, UTC, mutation and numeric exit are in `direct-countermutant.json`; combined output is `direct-countermutant.log.gz`. This closes the concrete fail-open mutant that escaped #542 attempt 3 without relying on the disputed captures below.

## Evidence attribution correction

`artifacts/issue567-attempt1/sort-status-countermutant-direct.meta` and `sort-status-countermutant-verified.meta` are not exact-command provenance for their paired raw outputs. Their recorded command prints `mutant_suite_exit` before catting suite output and ends by printing `causal_diagnostic=...`; their raw outputs put the exit after suite text and omit the final causal line. These pairs must not be credited as exact captures of the recorded command. Preserve them and mark this limitation in the issue/manifest, using this review's independently captured direct proof and the causal in-suite control for acceptance. This requires evidence attribution correction, not a new product implementation attempt.

The two already-denied ineffective quoted-shell probes remain non-credit evidence as previously documented. All 11 verified metadata/raw pairs decompress, and their 55 source-hash entries match the candidate, but hash agreement does not establish command identity for the disputed pair. Larger protocol/dependency/census captures remain supporting evidence; this bounded issue adds no product behavior requiring their repetition in review.

Only the successor spec, fixture script and issue567 evidence differ from the inherited head. Product, production checker, shared helper, lockfile, prior qualification and all #542 evidence/review bytes are unchanged. The worktree initially was clean; this uncommitted report directory is the only review addition. No product edits, benchmarks, GitHub mutations or pushes were performed. Root still owns evidence-attribution correction, exact-head delivery review and remote qualification/synchronization.
