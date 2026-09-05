# Astra #462 Cargo portability ruling — prior PASS withdrawn

Reviewed `7a4ad0e7211df3ba820ce3dfcff7a7b8784eee5b`. Root's finding is confirmed: five generated Cargo shims in `scripts/test-realtime-audit-leak.sh` delegate to the literal `/home/bl/.cargo/bin/cargo`. On a normal CI host without that path, the earlier package cannot resolve and the suite fails before the intended later-package fault. My previous source PASS correctly assessed the four added assertions but missed this inherited portability defect; it is withdrawn for qualification readiness. Do not run parent full qualification or ship this checkpoint as accepted.

## Exact amendment, approved before correction

Amend #462 to include the minimum portable resolution of those five delegates in the SAME existing suite, plus scoped evidence/parent record. Both production scanners, shared helpers, benchmark suite and all original fault selectors, payloads, statuses,97/96 assertions, two actual mutants and restored-positive semantics remain frozen.

Resolve the actual executable Cargo from the incoming PATH once, before any test shim is prepended. Use an executable-path lookup (for example Bash type -P, with checked nonempty/executable result); if incoming PATH yields a relative path, make it absolute before run_gate changes directory. Preserve the Cargo executable/symlink spelling; resolving the rustup proxy to a differently named executable is unnecessary and risks dispatch semantics. Do not accept a shell function string as an executable pathname. Report an explicit setup failure if executable resolution fails.

Capture that absolute path and embed it safely into each of the five generated Bash delegates. Bash printf %q for the resolved pathname, or an equivalently sound quoted assignment followed by exec "$saved_cargo" "$@", is sufficient. Preserve literal runtime "$@" and every argument; do not use JSON stringification or unsafe interpolated shell text. Resolve BEFORE shim injection so delegates cannot recursively rediscover themselves. Do not add a generic command resolver/helper or touch other unrelated tool paths.

The five existing case sites are status-loss-cargo, status-loss-cargo-empty, status-loss-cargo-matching, cargo-empty-success, and status-loss-grep. They all need the same correction; leaving four inherited hardcoded sites would still prevent the new child from working in CI.

## Required targeted relocation proof

In disposable scratch outside the repository, place a forwarding executable named cargo in a directory whose name contains a space. Resolve the real Cargo before arranging the test PATH; the forwarding script logs its current fixture directory plus arguments to a fixed scratch log, then execs that saved real Cargo with unchanged arguments. Prepend this directory to PATH and run the actual affected suite. No build or benchmark is involved: its Cargo operations remain locked offline tree metadata queries.

Require suite exit0, the existing two actual97 mutant outcomes and restored0. From the forwarding log verify that each of the five named fixtures invoked this relocated executable for its EARLIER `fixture` package with the frozen complete Cargo-tree flags; the precise later-package refusal diagnostics must still pass. These per-fixture delegate hits distinguish real use of the resolved path from accidentally succeeding through the author's still-existing absolute Cargo installation. The spaced pathname proves quoting. Retain command, scratch wrapper text, trace, statuses and relevant assertions in evidence; do not replace the test's later producers with new fake graphs or add another production mutant campaign. An explicit search must confirm no `/home/bl/.cargo/bin/cargo` remains in the suite.

Re-run affected syntax and suite in ordinary incoming PATH too, plus diff hygiene. The benchmark suite and two real scanner results remain applicable if their source/input has not changed; repeat them only if an actual delta invalidates them. No redundant full workspace until revised source acceptance. No new permanent framework or test corpus is required for this one relocation proof.

## Attempt and parent accounting

This defect must be fixed within the explicitly AMENDED #462 portability/completion contract, not deferred beyond the PR and not repaired informally on stopped #453. It is in the same allowed suite and directly prevents delivery of the four assigned cases. #453's three-attempt history and accepted production semantics remain preserved; full parent qualification/actual-PR/CI stays binding.

Because this discovery overturns Luna1 qualification readiness, root should record the correction to the verdict and assign the bounded coherent revision to Sol as #462 attempt2 under the user's model workflow. Do not silently give Luna another pass or reset counters. The amendment adds exactly executable discovery, five safe delegates and the relocation evidence above; no fourth #453 attempt or wider tooling work is authorized.

Read-only source inspection only; no tests, builds, source/spec/Git/GitHub mutations or timing were performed. Root owns amendment synchronization, implementation assignment and checkpoints.
