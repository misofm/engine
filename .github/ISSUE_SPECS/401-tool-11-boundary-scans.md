# Make compiler and runtime boundary gates reject failed scans

Parent #306; depends on foundation #400. The complete standing contract is embedded below. Smallest outcome: seven boundary gates cannot pass from failed scans while dependency ownership/allowlists remain identical.

Exact gate roster (no other gate edits):
- scripts/check-graph-policy.sh
- scripts/check-protocol-control-policy.sh
- scripts/check-session-policy.sh
- scripts/check-effect-runtime-policy.sh
- scripts/check-host-core-policy.sh
- scripts/check-conformance-boundaries.sh
- scripts/check-builtins-policy.sh

Use the shared library; consolidate the remaining production-dependency awk copies in graph, effect-runtime, conformance and builtins with rack's helper, proving existing sorted outputs and supported declaration forms unchanged. Keep bespoke policy logic: session ordering rules, graph executor discovery, conformance manifest-derived lists, effect-runtime filtered helper-leak scans, host-core optional protocol edge and builtins allocation-tracker allowlist. Check upstream scan status before applying exclusions or reading process-substitution results; assert non-vacuity for required graph/conformance discovery. Do not add new policy concepts or regex bypasses.

Allowed support paths: scripts/lib/gate.sh only for minimal necessary checked-result helper(s), its focused tests; the corresponding existing suites below; new small per-gate fixture suites ONLY for a listed gate lacking direct coverage; per-issue spec/evidence. Before implementation freeze a concise call-site inventory for these seven scripts mapping direct bans, positive queries, filtered scans, producers and expected empty sets to tests. This is the acceptance checklist, not a workspace re-audit. If that inventory requires a new generic harness or a half-day-plus slice, split this child before coding.

Existing suites to preserve: test-protocol-control-policy.sh, test-effect-runtime-policy.sh, test-host-core-policy.sh, test-builtins-policy.sh; test-builtins-benchmark.sh has graph/builtins checker coverage. Session and conformance have no directly named current mutation suite; create minimal isolated policy fixtures, not build or runtime frameworks. Graph missing-root/discovery cases belong in a dedicated small gate suite if the benchmark harness cannot exercise the failure without workload execution.

Each of the seven gates: clean control, all previous violation cases, a missing required scan root, a producer/read error where relevant, and a counter-mutation showing unexpected acceptance is caught. Retain exact host-core optional-edge mutations introduced by #369. Run real gates, all affected suites, shared helper tests, full workspace unchanged-count comparison and required CI. No benchmark invocation, artifact rebuilding, source changes or publication. Root checkpoints each compiling/focused-green tranche; Astra reviews final PR. Parent remains OPEN.

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
