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

## Astra-approved two-child execution split

# Astra #401 readiness ruling — 2026-09-05

**Not ready to assign as the current seven-gate implementation pass.** #400 focused acceptance passed at 58e15750, but must merge and freeze the actual shared API first. This read-only review does not delay #400. Luna's inventory is useful, with the corrections below; its phrase “smallest implementation slice” is not a scope approval.

## Required inventory corrections

- The dependency parsers are not interchangeable yet. Graph's current parser returns workspace-suffixed keys only; effect-runtime/builtins strip the key from the full line before `=`, accepting `name="value"` without whitespace; #400's rack parser uses `$1` and therefore has different formatting semantics. Conformance additionally reads `[target.*.dependencies]`; tools/bench actually uses that form. Preserve each gate's current accepted declaration forms and target-section boundary. A narrowly parameterized common extractor may support these modes; no global “scan all dependencies” replacement, missing target entries, or accidental rejection of compact `name=value` declarations. Freeze sorted real-manifest outputs and directed whitespace/target/dev-section examples before implementing the mode extension. The original five extractor copies are rack, graph, effect-runtime, conformance and builtins; “two extractors” in effect-runtime means two calls to one function.
- Effect-runtime inventory omits its `helper_definitions` producer and exact-count loop. This contains `rg ... crates/*/src || true`, an exclusion filter, then wc; partial/failed scanning could satisfy a pinned zero or lower count. Include it with all helper-manifest rows unchanged. The print-manifest environment mode is existing fixture support, not permission to bypass earlier policy failures.
- Host-core inventory's “three host scans” is incomplete: there is a fourth forbidden C-export scan. Include exact-count grep queries and both rg|wc producers, preserving the optional/non-default protocol checks. Producer errors cannot become expected counts, including partial output.
- Protocol optional message_wire/session_wire files remain optional; present unreadable paths must fail. Empty MockProvider public fields are legitimate (MockProvider is test-support-only). ControlProvider itself is the required named protected surface: absence of the extracted trait must not silently produce a clean scan. Do not require MockProvider production availability or mark every optional message file required.
- Session has NO fixture-root argument today: it resolves its own script directory. Its fixture discovery is a grouped process substitution containing four find invocations over six root paths, additional to the historical nine *syntactically direct* find loops. Inventory it explicitly rather than assuming the historical nine exhaust all producers. Existing roots are fixtures/session, fixtures/native-pcm-runner, hosts/host-web/qualification, hosts/host-web/tests/browser-v1, sdk and fuzz. All are required repository populations for this gate; finding no TOML is the intended clean result. Successful empty filtered historical/retired matches are also valid. Every find, sort, allowlist-sed and rg status must survive. Five ordering anchors are individually required numeric first-match positions; never let empty values coerce to zero and accidentally satisfy arithmetic ordering. Keep all current allowlists/self exclusions and compile order unchanged. Test via a disposable minimal repository tree with script/helper copied intact, or explicitly brief a backward-compatible fixture-root argument before implementation; do not silently invent one.
- Conformance individual manifests need not each carry a `[lib]` table (bin-only packages are real); the aggregate set of production library names must be nonempty after successful complete discovery. Mandatory named production crate manifests/source roots must not silently disappear under `[[ -f ]] || continue`. Local same-named `mod conformance` is a valid exemption after a successful checked probe, not an excuse to accept unreadable source. All four root directories remain required but empty individual roots are valid. Preserve target dependency sections and host/sidecar no-match success.

## Numbered implementation slices

The seven-gate umbrella remains OPEN until all four children close:

- #406: protocol-control, effect-runtime, host-core and builtins. Merged as PR #414 / 882277b65ff64780f57c4df33dee127abc6a33e2; verified CLOSED.
- #407: graph and conformance. Merged as PR #421 / `a0e4d123a038160b4f5934dac14aacc72c9fbbf2`; verified CLOSED. Owns graph's two and conformance's one original find loops and their exact parser/discovery obligations. Two focused suites are wired beside their current checkers in required CI.
- #417: Session format/compile-order policy, after merged #407. Owns the four grouped find producers over six required populations, first-match ordering, allowlists and its focused suite/CI wiring.
- #423: the remaining graph dependency-parser consolidation, after merged #417 and before #410. Preserves graph’s exact workspace-prefix/full-first-field grammar while moving the fifth original parser body into the existing shared helper.

Astra invoked #407's explicit pre-code split because Session's independent fixture scope made the three-gate pass too broad for half a working day. No gate, extractor or producer debt is removed from this parent. Shared-helper and workflow edits remain serialized.

## Common acceptance for every child

For each changed gate: real-tree positive check, all existing relevant violations, explicit required-root/required-surface deletion, clean optional-empty positive, injected producer error with otherwise-valid metadata, and failure AFTER valid partial output. Check producer status before filtering, counting or looping. Test direct/no-match/positive queries separately; filters may validly leave nothing. Error assertions require the intended class, explicit rejection of unexpected success and one counter-mutation per new helper failure mechanism. Preserve physical-script library sourcing, CLI defaults, diagnostics, caller shell state, exact roots/globs/allowlists and no runtime/source changes.

Final gates are affected shell suites, bash syntax, real policy scripts, existing workspace unchanged-count comparison and required CI. No benchmark, artifact regeneration or publication. Root checkpoints one coherent pass; Luna first attempt, Sol only following Astra FAIL (three total maximum), Astra actual PR review before merge. #401 closes only after all four children and all seven gate/extractor obligations are upstream/closed; broad #306/TOOL-11 remain open for the rest of their program.

No implementation, Cargo, benchmark, policy mutation run or GitHub change was performed in this readiness review.


Root synchronized the final #406 evidence and the amended #407 / new #417 stateless briefs before next implementation. #407's base is 882277b6; Session keeps the exact first-match and optional-empty semantics in #417. This parent stays OPEN until all remaining assigned gates are upstream and CLOSED.

## Current delivered boundary coverage

Six of the seven assigned gates are upstream through closed #406 and #407. The remaining Session gate is active in #417, whose first source checkpoint awaits Astra review. #401 stays OPEN until Session and the residual parser consolidation #423 pass their complete contracts, actual PR reviews and required CI, and are merged/synchronized. This status does not close broader #306 or remove any original extractor or discovery obligation.

## Residual extractor closure ruling

# Astra #401 remaining extractor obligation

**Keep #401 OPEN. Graph's local parser is a remaining consolidation obligation, not an approved exception.**

The parent explicitly requires all five original production-dependency awk copies to consolidate into the shared helper. Current rack uses gate_toml_dependencies default; effect-runtime and builtins use plain; conformance uses plain-target. Graph alone retains `graph_dependencies_raw=$(awk ...)` followed by gate_sort_lines. Its statuses are checked, so #407 delivered the reviewed traversal/error-handling behavior, but the broader DRY obligation is not thereby fulfilled.

The approved child briefing deliberately preserved graph's distinct regex and `$1` output; it did not exempt that parser from consolidation. My actual #407 review did not identify this residual parent accounting gap. Preserve the accepted child history and accurately record this remaining parent obligation rather than inventing retrospective approval or closing #401 on #417 alone.

Root should number/synchronize the bounded successor in `/tmp/astra-401-graph-extractor-successor.md` before #417 closure. #417's own Session capability may still merge/close after its complete review/CI gates; this is parent closure debt, not a new Session code dependency. Serialize the small successor after #417 and preferably before #410; update #410 scheduling only once root chooses/numbers that order. #306 and broad TOOL-11 remain open through all their other children.

Read-only inspection of parent/child specs, earlier approval and current five gate call sites; no implementation, tests, Cargo, Git or GitHub operations.

Numbered successor #423 preserves this obligation without reopening accepted #407 scan correctness or expanding #417. The serialized order is #417, #423, #410, #411, #412.

## Final child accounting for #423 delivery

All seven gate implementations and five original dependency extractors are now covered without policy exceptions. Foundation #400 supplied the checked helper. Closed #406 (PR #414, merge `882277b65ff64780f57c4df33dee127abc6a33e2`) delivered protocol-control, effect-runtime, host-core and builtins; closed #407 (PR #421, merge `a0e4d123a038160b4f5934dac14aacc72c9fbbf2`) delivered graph/conformance scan correctness; closed #417 (PR #424, merge `1cfd49d2929b2a75f6054badebfa9979c069ae71`) delivered Session policy completeness. Astra accepted #423 source at `58729f9853f5a724a2cb843b61e48d33cfa156fd`, preserving graph grammar and completing the remaining extractor.

The five shared-helper modes are rack/default, effect-runtime/plain, builtins/plain, conformance/plain-target and graph/graph; no local production dependency parser remains at these sites. Sol independently checked this original-obligation accounting in `/tmp/sol-401-final-accounting.md`. #423 owns the remaining unchanged-count workspace comparison and actual PR Astra/required-CI delivery. This parent may close jointly with #423 only when those pass and the accepted evidence is upstream; #306 and broader TOOL-11 remain open for their separate obligations. Final delivery evidence is recorded in #423 and the remote merge/closure comments.
