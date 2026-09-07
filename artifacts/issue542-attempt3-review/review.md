# Issue 542 attempt 3 adversarial review

Verdict: **FAIL** — one retained-status-coverage requirement is not met.
Reviewer: Astra LOW. Historical Sol and Astra verdict attribution is unchanged.
Reviewed candidate: `57fb9ca0f9ffe9d1ade8c42d2a557d4d6358386e`.
Current origin/main: `b95c9b7b028ed07cfea2f7669467689c05320c37`.
Comparison for bounded correction: previously reviewed `fc6798e426ffc34407e0280add664a38c763bdcb`.
Date: 2026-09-07 UTC.

## Blocking finding

`scripts/test-conformance-boundaries.sh:117` changes the preexisting generic sort-error expectation from the workspace `gate_sort_lines` diagnostic to an earlier engine TOML dependency extraction failure. Line 118 targets only the conformance TOML dependency sort. Consequently neither injected sort failure reaches `gate_sort_lines`, which independently handles workspace manifest and library-name sorting. This loses existing status/fail-open coverage explicitly frozen by attempt 3; changing the expected diagnostic made the row green without preserving its original consumer coverage.

Concrete adversarial proof: in a temporary copy only, changed the `gate_sort_lines` sort failure branch from `else rc=$?` to `else rc=0`, leaving its output and every other helper unchanged. Ran the complete candidate fixture suite with CHECKER pointing to that scratch checker/helper. It exited **0**, printed `conformance boundary fixtures: ok`, and still reported its module counter-mutant rejected. The candidate suite therefore accepts that fail-open workspace-sort consumer. Exact command, mutation, UTC, exit and compressed output are in `sort-status-countermutant.json` and `sort-status-countermutant.log.gz`. No tracked product/helper source was edited.

The correction needs a status injection that reaches workspace manifest/name sorting after the earlier TOML extraction succeeds, while retaining the separate TOML sort controls. This is a failed third verdict; root must follow the issue's rescope rule before further implementation, not silently layer a fourth attempt.

## Other observations

The ordinary hermetic suite and production boundary checker both independently exit 0. Shell syntax and complete base-to-head/working diff whitespace checks exit 0. Commands and outputs are retained in `commands.json` and `check-*.log.gz`.

The clean fixture contains the exact three cfg(test)/mod tests parents and children and the approved conformance dependency union. Missing-child population is checked by the fixture assertion; missing/changed guards and outside-path use are checked by the production checker. Each row starts from a fresh base copy. Existing find/awk/paste/rg and module counter-mutant controls remain executable. The old rg-manifest row now duplicates the module probe because manifest discovery moved to the TOML parser; it must not be described as continuing to exercise an rg manifest scan. No separate blocker is assigned to that obsolete scan migration.

Only the authorized test fixture script, #542 spec, and new attempt-3 evidence differ from fc6798e. Product source, production checker, corpus, dependency/lockfile, Wasm and prior qualification/review artifacts remain byte-identical. The 11 new metadata/raw pairs are readable; all 83 recorded source hashes match the candidate. Their precommit HEAD is correctly accompanied by the modified fixture status and exact candidate fixture hash. The live test census is distinct from the one-test synthetic children. Evidence authenticity does not repair the coverage gap above.

The initial review worktree was clean; only `artifacts/issue542-attempt3-review/` is added and left uncommitted. No benchmarks, pushes or GitHub mutations were performed.
