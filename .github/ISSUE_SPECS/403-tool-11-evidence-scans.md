# Make qualification and evidence gate traversal complete or fail

Parent #306; depends on foundation #400. Embed parent standing contract. Outcome: six evidence/qualification gates cannot certify an artifact or boundary after a failed search/traversal, while all existing artifact validators and identities stay unchanged.

Exact gate roster:
- scripts/check-bench-policy.sh
- scripts/check-realtime-audit-leak.sh
- scripts/check-native-pcm-runner.sh
- scripts/check-effect-interchange-qualification.sh
- scripts/check-effect-interchange-benchmark-108.sh
- scripts/check-rack-benchmark-fixture.sh

Use common applicable helper(s); bench-policy and realtime-audit-leak contain no current rg sites, so preserve their actual parser/find behavior rather than manufacturing searches. Native runner and interchange qualification have bespoke validation; keep it. Correct unchecked searches, checked-presence operations, filtered results and required non-vacuous traversals only. No changing sealed record schema, fixture identities, benchmarks, validators' numerical tolerances or runtime/ABI behavior.

Affected existing suites: test-bench-policy.sh, test-realtime-audit-leak.sh, test-native-pcm-runner-v1-policy.sh, test-native-pcm-runner-portability-v1-policy.sh, test-effect-interchange-policy.sh, test-effect-interchange-benchmark.sh, test-effect-interchange-benchmark-108-policy.sh, test-rack-benchmark.sh, plus shared helper tests. Inspect each harness's existing stubbing/workload guard before invocation; execute only hermetic mutation/preflight paths, never their named timed runner. New missing-root tests use valid fixtures, assert the intended error class, and include unexpected-success/error-class counter-mutations.

Allowed paths: six gates, listed affected suites, minimal shared helper extension/tests, issue spec/evidence. Freeze per-site inventory and expected empty-set semantics before coding; if one benchmark/runner infrastructure defect requires more than one bounded correction, record/split it rather than extending this migration.

Acceptance: all six real non-timed gates and all applicable existing mutations pass; every migrated gate has real missing-root/read-error proof; no silent producer failures or unexpected empty required scans; full workspace test count unchanged and required CI. No artifact regeneration, timed workloads, new parser framework or package publication. Root checkpoints coherent green tranches; Astra actual PR review. Parent #306 closes only after the full program and all original-scope debt are synchronized, not merely this last code branch's claim.

## Standing contract


- A search result has THREE outcomes: matches, clean no-match, execution failure. A required path/read/parse error must never be interpreted as clean. Preserve stdout/stderr needed to distinguish them.
- Explicit conditionals capture command status; do not rely on set-e inside functions invoked from conditionals, pipelines/process substitutions, or standalone `! command`. Helpers must not toggle caller shell options or install caller traps/change cwd.
- Resolve sourced library by the script's own physical location before cd into a fixture root. A fixture-root argument selects data to inspect, never a different helper implementation. Preserve existing script CLI/environment and diagnostic prefixes.
- Preserve regex/glob/allowlist semantics. Filtering legitimate exceptions happens AFTER a successful checked source scan; an empty filtered result is allowed, failed source traversal is not. Do not use `--glob '*'` as a blind replacement for “no glob,” because it can alter ignored-file traversal.
- Known required roots remain required; no blanket filter-to-existing-directories or mkdir workaround. If an optional root is currently legitimate, document that specific policy and retain missing-required-root red cases.
- Expected discovery must be non-vacuous. Capture producer failures before a consumer loop and assert nonempty output when the policy requires at least one input. Record any legitimate empty-set case explicitly. All original #306 nine-loop debt must be assigned in the frozen per-child call-site inventory; if a remaining original site lies outside the 21 roster, record a stateless bounded successor before parent closure rather than silently omitting it.
- Every migrated gate has a clean positive control, retained old violation mutations and a new missing-root red case. Prove the changed helper is actually reached, not only that an unrelated earlier manifest check fails. Where deletion is intercepted by prior checks, additionally inject a controlled rg failure at the relevant scan while all required metadata remains valid.
- Red helpers explicitly reject unexpected success and distinguish intended predicate failure from missing tools/syntax errors. Each new helper-level failure class gets at least one counter-mutation demonstrating the assertion is live.
- No Cargo tests for prose/shell implementation mirrors; existing full workspace unchanged-count requirement is retained at coherent child boundary. Run all existing affected shell suites and applicable current required CI. No artifact byte regeneration, benchmark launches or publication solely for gate extraction.


## Readiness and assignment

Prerequisite #371 is merged as `2a18b315067898a94fdc02e8f8b80f07b788ff89` and verified CLOSED. Its actual realtime policy has 42 regions in 12 files. This issue is a queued brief, not an implementation claim. Freeze the exact base and per-site inventory at assignment. Current roles: Astra scope/final PR review, Luna one implementation attempt, Sol at most two retries on failure, then preserve evidence and rescope; root owns checkpoints, pushes and GitHub synchronization. Do not edit another owner’s worktree.

## Program closure

Parent #306 and the broad #349 TOOL-11 finding remain OPEN until #400, #401, #402 and #403 are upstream and all original 21-gate, five-extractor and nine-loop obligations are resolved. Each child closes only its named outcome; any discovered original-scope site outside the roster requires a numbered successor before parent closure.
