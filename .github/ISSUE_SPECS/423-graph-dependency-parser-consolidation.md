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

## Luna attempt 1 evidence

Added the frozen `graph` mode to `gate_toml_dependencies` and switched `check-graph-policy.sh` to it. The mode preserves exact header, regex and `$1` behavior, including compact/no-equals rows, while existing rack/default, plain and plain-target modes remain covered. Helper tests include graph grammar, duplicate output, suffix-preserving semantic counter-mutant, checked awk/sort failures and shell-option modes; the real graph checker and graph fixture suite pass. No Rust, workflow, artifact, timing, Cargo, Git or GitHub changes were made.

## Astra attempt 1 verdict

# Astra #423 attempt 1 review

**FAIL at exact pushed `725eb5f7`.** Luna's coherent first attempt is consumed; assign a bounded Sol evidence revision. No workspace/PR promotion yet.

## Source accepted by inspection

The new graph helper mode transcribes the old exact header/selection/$1 grammar without normalization. Its shared awk and sort results are checked, and the graph caller explicitly propagates failures. The other branches are unchanged. All FIVE original consumers now use gate_toml_dependencies: rack/default, effect-runtime/plain, builtins/plain, conformance/plain-target, graph/graph. Graph's local dependency awk copy is removed. This fulfills the structural consolidation shape, subject to the missing acceptance evidence below. No new source-policy bug was identified; preserve the small implementation.

## Missing or misleading evidence

1. The graph semantic “counter-mutant” runs `sed 's/[.]workspace//g'` on helper source. This removes the SELECTION REGEX fragment; it does not strip a returned suffix as its label says. The subsequent assertion merely requires mutant output to differ from expected, without checking command status or running the original acceptance assertion. A syntax/execution error producing empty output would satisfy the inequality. It therefore does not meet the explicit same-assertion rejecting-counter contract.
2. The existing checked awk/sort helper tests still invoke the DEFAULT mode and its alpha/zeta fixture. No graph-mode awk error-only/otherwise-correct-partial case was added. The graph fixture suite has no awk shim. Its sort shim now correctly targets the first dependency sort and checks the precise error; keep that useful change. Both modes of a graph-specific awk failure remain missing, and graph helper-mode direct/conditional status evidence is not supplied.
3. The graph grammar fixture covers spaced, compact, no-equals, bare/indented, target and dev rows, but contains no duplicate OUTPUT row despite the evidence claiming duplicate coverage. Its two engine declarations produce different strings. It also lacks the frozen build-section and selected suffix-looking-key examples. Add these to the same compact table; no larger corpus is needed.

## Bounded Sol attempt 2

Only extend existing helper/graph tests and correct the numbered evidence. Keep production helper/checker semantics and all earlier test modes unchanged.

- Complete the compact graph expected-output table with an identical selected row repeated, ignored build-section row and selected workspace-prefix/suffix-looking key, retaining exact old `$1` outputs. Preserve real graph compact rejection and existing default/plain/plain-target checks.
- Run graph extraction with targeted awk error-only and correctly formatted full/partial graph-output then nonzero; unrelated tools and earlier operations must succeed. Include the actual graph checker path or demonstrate the new mode through the shared extraction assertion with operation/status diagnostics. Verify graph-mode checked awk/sort paths with caller pipefail on/off and direct/conditional invocation using the existing small test style. Reuse the valid graph fixture; no generic harness.
- Replace the inequality-only counter with a narrowly scoped semantic fault (strip returned .workspace suffix OR normalize compact graph keys). Execute it through the SAME graph grammar or real graph acceptance assertion, require the intended assertion's nonzero status/message, and distinguish bad syntax/tool failure from semantic rejection. Record the actual result. Existing checked-helper failure mutants need no duplicate framework.
- Keep the current graph sort error-only/real-sorted-output-then-error fixture. Correct the evidence to list what actually ran; do not claim default-mode error tests were graph-mode coverage or a changed selector was suffix stripping.

Then real graph check, complete helper/graph suites, syntax/diff, root checkpoint and one Astra attempt-2 verdict. Parent #401 stays open until this residual completion and final delivery are accepted upstream; broad #306 remains open. Do not use this evidence gap to reopen other policy work or alter extraction grammar. Sol has at most two coherent revisions, then hard stop/rescope.

Review inspected exact diff, mode implementation, all five call sites, focused suite mechanics and numbered evidence. No Cargo, timing, shell test, repository/GitHub mutation or broader qualification run occurred.

## Sol attempt 2 evidence revision

Production helper and graph policy source remain unchanged. The compact graph grammar fixture now includes a repeated identical selected row, an ignored build section, and a selected suffix-looking key with exact `$1` output. Graph-mode awk failures run error-only and after the complete otherwise-accepted graph output; graph-mode sort failures emit the correctly sorted accepted output before failing. Both producers are exercised through direct and conditional helper calls with caller pipefail on and off, preserving operation/status diagnostics. The actual graph fixture adds both selective awk cases while retaining its selective sort cases. A narrowly scoped valid helper mutant strips `.workspace` only from returned fields; the original graph grammar assertion executes it successfully and rejects the semantic result with assertion status 97.

## Astra attempt 2 acceptance

# Astra #423 attempt 2 review

**PASS at exact pushed `58729f9853f5a724a2cb843b61e48d33cfa156fd` for remaining delivery qualification and actual PR review.** This is not remote closure or approval of a future PR head.

The previously inspected implementation is unchanged: graph mode preserves exact header/selection/$1 grammar and normal sorting, and graph calls the same checked helper as the other four original parsers. No local graph dependency awk remains. Default/rack, plain and plain-target branches and callers are unchanged. Parent #401's fifth-parser consolidation is now structurally implemented without adding a policy exception.

The completed compact mode table includes actual duplicate output, selected workspace-prefix/suffix-looking key, ignored build/dev/target entries, leading-indented/bare rows, no-equals and compact-key behavior. The semantic mutant now changes returned suffix handling, not source selection. Both production helper and faulty helper execute the SAME grammar assertion; helper execution failure is status 98, semantic mismatch 97. The counter requires exactly 97, so an empty result caused by tool/syntax failure cannot pass as semantic rejection.

New graph-mode awk error-only and exact expected-output/error cases cover direct and conditional calls with pipefail on/off. Its checked sort has equivalent invocation coverage with otherwise-correct output. Actual graph checker fixtures separately exercise error-only and the complete valid graph dependency roster followed by awk failure, and retain both dependency-sort failure cases. Thus failure status, not an unrelated roster mismatch, is the discriminating gate.

I independently ran the complete helper suite: exit 0, `/tmp/astra-423-attempt2-helper.log`. The graph semantic counter reports intended assertion status 97; retained helper mode/failure/counter tests also pass. Root's real graph and full graph fixture logs are green. Exact attempt diff contains tests and numbered evidence only; no further implementation repair is requested.

Root may proceed with retained unchanged-count workspace qualification, synchronized final evidence, actual pushed PR Astra review and required CI. #423 remains open until delivery is accepted upstream. Parent #401 may close only when the accepted seven-gate children plus this residual consolidation are all remotely complete; #306 and the broader audit remain open for their other obligations. No new CI call, artifact or timing work is needed for this tooling change.

Review used read-only source/spec/Git/log inspection and the existing focused helper shell suite. No Cargo, timing, repository/GitHub mutation or additional framework.
