# Consolidate the remaining graph dependency parser without changing its grammar

Ready to number as one bounded successor of parent #401; grandparents #306/#349 TOOL-11. Prerequisites: merged #407 shared helper and merged #417 to serialize helper work. Prefer assignment before #410; root must freeze that actual order/base and update reciprocal scheduling before coding. Parent #401 retains every original obligation and closes only when #406/#407/#417 and this residual extractor completion are upstream/closed. Session #417 owns no changes from this successor.

## Smallest closable outcome

Move graph's last local production-dependency awk body into an explicit graph mode of existing gate_toml_dependencies, call it from check-graph-policy.sh, and prove old grammar/output/error handling is unchanged. This finishes the fifth-of-five consolidation requirement. No parser framework, new dependency rule or renewed graph/conformance discovery audit.

Allowed: scripts/lib/gate.sh, scripts/check-graph-policy.sh, scripts/test-gate-lib.sh, scripts/test-graph-policy.sh, numbered evidence and root-owned parent accounting. No other gate, Rust, manifest, workflow or artifact changes. Existing CI already runs both relevant suites (helper through workspace tests); add no calls/jobs.

## Frozen exact mode

Graph recognizes exact `[dependencies]` only, exits on any subsequent line beginning `[`, selects `^[a-zA-Z0-9_-]+[.]workspace` WITHOUT an equals requirement, and prints FULL `$1` WITHOUT stripping `.workspace`. Preserve case/whitespace/header/regex behavior verbatim. Sort normally, retaining duplicates. No leading-whitespace normalization, full TOML parsing, Boolean validation or target/dev/build inclusion.

The real sorted graph output remains exactly effect-contract.workspace, engine.workspace, lane.workspace, rack.workspace. `engine.workspace = true` yields engine.workspace. `engine.workspace=true` yields engine.workspace=true and still fails the graph exact-output policy. A selected workspace key without `=` still yields that key; do not tighten it during consolidation. Bare `engine = ...` and target/dev/build entries remain outside this graph mode. Duplicate selected keys remain duplicated and therefore fail the exact real roster when injected.

Preserve existing rack/default `$1` semantics, plain compact full-key extraction and plain-target inclusion without modifying any current caller. Unknown-mode handling is not part of this task. Extraction awk and subsequent sort statuses must both be checked before result interpretation, including useful complete-looking output then failure. Retain graph failure prefix/context; avoid reducing failures to a misleading boundary mismatch.

## Focused evidence

Add a compact direct helper-mode table containing spaced and compact declarations, a no-equals selected row, leading-indented/bare keys, exact versus target/dev/build headers, selected suffix-looking keys and duplicates. Expected outputs derive from the preserved old regex/$1 grammar, not normalized TOML assumptions. Retain all existing default/plain/plain-target mode tests.

Run existing real graph gate and fixture suite. Its compact-declaration rejection must remain live. Add/reuse a mode-specific awk error-only and correct partial-output/error case, plus sort error-only and correct sorted-output/error, targeting the actual helper calls while earlier fixture operations succeed. Assertions preserve diagnostics and reject unexpected success at the intended stage. A mode semantic counter-mutant that strips `.workspace` or normalizes compact keys must fail the real helper/graph assertion; checked-helper failure mutants already maintained need not be duplicated unless this change adds a new failure mechanism. Existing tests must still prove conditional/direct calls do not rely on caller pipefail/errexit.

At completion inspect all five original gate parser sites: rack/default, effect-runtime/plain, builtins/plain, conformance/plain-target and graph/graph must use the same helper; graph has no remaining local production-dependency awk body. Bespoke graph policy/discovery remains untouched. This is structural proof of the actual parent consolidation claim, not merely fewer duplicate lines.

Focused gates: existing helper suite, graph suite, real graph check, syntax/diff and direct backward-mode fixtures. Required qualification covers the other real shared-helper callers; parent delivery retains its established unchanged-count workspace comparison, root-controlled after focused PASS. No timing, artifact regeneration or broad new framework.

Astra approves the numbered synchronized scope before assignment. Luna one coherent attempt, Sol at most two revisions only after Astra FAIL, root checkpoints/synchronizes, actual PR Astra review plus required CI before merge. Third failure hard-stops. #401/#306 cannot claim consolidation complete until this successor's accepted evidence is upstream and remotely synchronized.

## Numbered successor

This is #423, created before #417 closure after the explicit parent #401 consolidation audit. Implementation stays queued until #417 merges; it then precedes #410 to serialize shared-helper changes. Root freezes the actual merged base and obtains Astra numbered approval before assigning Luna.

## Astra numbered scope approval

# Astra numbered #423 scope approval

**PASS for numbered planning checkpoint `9f7b288ec6c64b3d5a4ecf1fd19a50fab83212aa`. Implementation remains queued until #417 merges and root freezes that actual source base.**

The actual #423 spec matches the approved bounded successor: one explicit graph mode in the existing shared extractor, exact historical regex/$1/suffix/header/compact behavior, checked extraction and sort, direct backward-mode evidence and one live semantic counter-mutant. Its path scope excludes unrelated policy, workflow, runtime and artifact changes. Existing CI already reaches the suites. No extra framework or parser policy is introduced.

Parent #401 expressly retains the previously missed fifth-extractor consolidation obligation through #423; it does not invent a retrospective exception or reopen accepted #407 traversal correctness. #417 may close its own Session outcome after its required gates while the parent remains open. Root records reciprocal scheduling #417 → #423 → #410 → #411 → #412, preventing shared-helper overlap.

Root may assign Luna only after #417 merges and records the exact base. This approves the numbered scope and objective gates, not source completion, qualification or issue closure. Three-attempt rules and actual-head Astra/required-CI review remain intact.

Read-only local numbered spec and parent amendment review. Remote synchronization and reciprocal queued-branch checkpoint are root-recorded; no tests, Cargo, Git/GitHub operation, source edit or timing performed.

## Implementation assignment

PR #424 merged and #417 closed. Root freezes merged main `1cfd49d2929b2a75f6054badebfa9979c069ae71` as the implementation base. Luna owns attempt 1 in the isolated `codex/423-graph-parser` worktree. The approved order remains #417 → #423 → #410 → #411 → #412.
