# Validate environment and documentation input scans completely

Queued child of #402; grandparents #306/#349 TOOL-11. This is issue #412; the matching numbered local spec and GitHub body are synchronized before implementation. Depends on merged #400, #406 and #411 (which follows #410): #406 owns the effect-runtime mutation suite today, so use its merged final shape. Queue after other active shared-helper migrations; never overlap edits to that library or suite. No runtime feature prerequisite.

## Closable outcome and allowed paths

The environment vocabulary, migration contract and DSP research structural gates distinguish valid empty results from failed/incomplete input reads. Allowed: `scripts/check-env-vocabulary.sh`, `scripts/check-effect-state-migration-v1.sh`, `scripts/check-dsp-research.sh`; existing `scripts/test-env-vocabulary.sh` and `scripts/test-effect-runtime-policy.sh`; new small `scripts/test-dsp-research.sh` if no exact existing suite appears; minimal shared helper/tests; numbered spec/evidence. `.github/workflows/qualification.yml` is permitted ONLY to add `bash scripts/test-dsp-research.sh` beside the existing real DSP research gate in its existing job. Preserve job/router/verdict/triggers and every existing command. Do not make another workflow/job or duplicate other suites. Env and effect-runtime suites are already required-CI paths; helper tests are already reached through workspace mutations.

## Environment contract

This checker uses grep/find/git/xargs/comm, not rg. Do not invent rg searches. Preserve vocabulary regular-file/non-symlink rule, required tools/scripts inputs, source exclusions for vocabulary and issue specs, prefixes, trailing-underscore fragment exclusion and used/documented bidirectional equality. Required used/documented sets must be populated by successful reads; empty forbidden/difference sets pass only after complete successful producers. Preserve genuine non-Git fixture support and Git tracked-plus-untracked-not-ignored discovery. A failed Git invocation must not masquerade as a deliberate non-Git fixture fallback.

Capture all discovery, pathname-filter, source grep, sort, vocabulary grep and comm statuses explicitly. Valid partial output followed by error is failure. xargs' aggregate status is not grep's 0/1/2 contract: handle clean no-prefix files versus read/execution errors correctly, without dropping files or accepting partial reads. Existing pathname processing converts NUL to lines and back; preserve its current supported shape and do not claim new arbitrary-newline-filename support. Avoid embedding new literal environment names in production/test sources that would change the vocabulary corpus.

## Migration contract

Preserve actual regular-file checks for docs/EFFECT_STATE_MIGRATION_V1.md and crates/effect-compiler/src/migration.rs, all seven required documentation tokens, five required API names across effect-package/compiler sources, and three exact bans. No-match in required presence queries fails; successful no-match in bans passes. Capture source-read/search failures distinctly. Do not add documentation content or change semantic constraints. Existing test-effect-runtime-policy.sh already copies an applicable fixture, runs this narrow checker positively and tests a render-migration violation. Extend its merged #406 form with focused narrow-checker producer failures; never overwrite #406 coverage or add another repository fixture framework.

## Research contract

Preserve the exact ten required notes, sixteen headings with current nonempty-section rule, six support artifacts, bibliography format, console names and synthetic-listening literals. There are two key populations: >=2 distinct keys inside Primary and official sources, plus every key in the WHOLE note needing bibliography resolution. Check both extraction pipelines and the whole-note process-substitution producer; never validate only the counted section. Capture awk/rg/sort/read failures before counts/loops, including partial valid keys. No new prose hashes, research claims, citation rules or expanded corpus.

## Directed verification

Existing env/effect-runtime suites pass with all original violations. Add valid non-Git/Git fixture controls, clean no-prefix source files, used/documented agreement, deliberate mismatch, optional-empty forbidden/filter results, required-file/root absence with intact surrounding metadata, and failure after valid partial producer output. Test vocabulary read and source grep failure separately; do not let an unrelated early missing file certify later scan checks. Migration red cases must reach all newly checked mechanisms with valid docs/APIs. The small research suite builds synthetic structural fixture text from the frozen headings and two dummy bibliography entries: it makes no scientific claim. Cover missing note, empty section, unresolved whole-note key outside the primary section, and source/key producer read/error/partial-output failures. Run both real research checker and suite through the existing required job using the minimal wiring above.

Every red assertion explicitly rejects success and matches the intended diagnostic; counter-mutations show new failure mechanisms are enforced. chmod-only unreadability is insufficient under privileged execution. Gates: all three real checkers, existing two/new research suites, helper tests if modified, bash syntax/diff, retained full-workspace unchanged-count comparison and required CI. No timing, browser build, artifact rewrite, publication or research-corpus addition.

## Shared delivery contract

All command-producing operations must distinguish successful output, successful allowed emptiness and execution failure before any filter/count/consumer can certify a result. Never depend on caller pipefail/errexit, conditional-function behavior, standalone ! assertions or lost process-substitution status. Preserve stderr/status evidence, exact regex/glob/allowlist semantics, physical-script helper sourcing before fixture-root cd, caller options/cwd and existing CLI. Do not filter required roots down to those present or mkdir around a missing input. Minimal common helpers only; no command-runner/parser framework.

Astra scopes and reviews; Luna gets one coherent implementation attempt, Sol at most two retries only after FAIL, then hard stop/rescope. Root owns isolated worktrees, exact-path checkpoints/pushes, status/issue synchronization and merge; pause at coherent green tranche before layering more edits. No active shared-helper edits overlap #406 or another tooling child. Full qualification is immutable and shared-target Cargo is serialized. Astra reviews the actual pushed PR head and required CI must succeed before merge.

Each child closes its named concrete outcome only after upstream evidence and PASS. #402 remains OPEN until all three children account for all six original gates, with no silent producer or policy-coverage omission. #306 and #349 TOOL-11 remain OPEN through their other children. The serialized children are #410 (realtime/lane), #411 (unfused), and #412 (environment/migration/research); #410 follows merged #406. No implementation is authorized merely by publishing this queued brief.

## Final stateless clarification

# #412 final brief refresh — environment and documentation inputs

**Scope confirmed, with the concrete clarifications below to append to the existing numbered spec before assignment.** Preserve the existing #412 body: its three gates form a bounded tooling outcome and its policy/closure clauses remain binding. Do not manufacture another child or generic framework. Reviewed current main c746 source, shared helper and existing suites/CI; no tests/builds/timing or mutations. Implementation must wait for PR #440's accepted #411/#438 delivery to merge and root to freeze the actual base. #427 follows #412; #402/#306/#349 remain open for their remaining obligations.

## Exact paths and reuse

Production: scripts/check-env-vocabulary.sh, scripts/check-effect-state-migration-v1.sh, scripts/check-dsp-research.sh. Tests: existing scripts/test-env-vocabulary.sh and scripts/test-effect-runtime-policy.sh, plus one small scripts/test-dsp-research.sh. The shared library is **scripts/lib/gate.sh**, not gate-lib.sh; use existing checked required/forbidden searches and line transforms where their output semantics match. scripts/test-gate-lib.sh changes only if an actual helper change is necessary. No parser mode additions or generic command wrapper are indicated. Source the helper from the physical script directory before fixture-root cd.

CI permits exactly one new invocation, `bash scripts/test-dsp-research.sh`, adjacent to check-dsp-research in the existing evidence job. Env real/suite already run in lint; narrow migration is exercised by test-effect-runtime-policy, already in lint alongside the broader effect gate. Preserve all #406 mutations and unrelated workflow commands/router/verdict/triggers. No new runtime, vocabulary entry, research note, dependency, artifact or benchmark.

## Frozen producer/consumer contract

Every retained external producer/transform must be captured and checked before downstream use, even if it emitted the exact otherwise-valid output first and then failed. Read/search errors retain stage-specific diagnostics. Shared helper returns must be explicitly propagated, including calls from conditional functions; don't rely on errexit. A replacement shell builtin can remove a consumer invocation and its associated fault case; record the final callsite mapping instead of testing a command that no longer runs.

### Environment

- Vocabulary remains a regular non-symlink file; tools and scripts remain required input roots. Preserve tracked plus untracked-not-ignored Git discovery and genuine non-Git fixture fallback, exact target/.git exclusions in fallback, vocabulary and issue-spec exclusion from rule1, full other-source scanning and underscore-suffixed family-fragment exclusion from used names. Do not change grep into rg.
- Git classification failure must not silently choose find. Freeze a deliberate non-Git classification separately from an execution/repository error; fixtures must demonstrate both a genuine non-Git directory and an actual initialized Git fixture, plus selective Git-probe failure and Git listing failure. Do not use a PATH shim which merely makes every Git call fail and call that a non-Git positive.
- Checked stages: classification; selected git ls-files or find; NUL/newline conversion and path normalization/exclusion; source grep; prefix extraction/filter and uniqueness; tools/scripts grep and fragment filter; vocabulary row grep and delimiter removal/uniqueness; BOTH comm differences; final display count if external. Reuse captured populations rather than rediscovering.
- **Do not route NUL-delimited output through command substitution or gate_find_collect**, which captures text in a Bash string. Preserve NUL until the existing intentional supported pathname conversion, using bounded local shell/file handling suitable for this tooling. This does not add newline-filename support. Preserve statuses before converting/filtering; temporary output is not a new framework.
- grep no-prefix/no-match status1 is allowed for individual source files and rule1 forbidden set; execution/read error is not. xargs status123 does not distinguish clean grep1 from grep2. Use a strategy that distinguishes them explicitly; do not simply accept xargs123. Prefix/difference/filter sets may be empty; used/documented final sets must be nonempty and exactly equal. Source set filtered empty is not itself a forbidden-prefix violation; required vocabulary/use checks still apply.
- Tests must preserve existing semantic mutations, add clean no-prefix files, Git and non-Git positives, and distinguish source reading from vocabulary reading. Required roots/file absence and symlink refusal need otherwise-valid surroundings. Build environment-name fixture strings in pieces as the existing suite does, avoiding new vocabulary entries accidentally introduced by tests.

### Migration

Retain exact two regular-file preconditions, seven documentation tokens, five API names searched across BOTH package/compiler src roots, and three bans (runtime-owned roots; redundant identity/validation in migration.rs; serialization there). Required match uses complete checked search, not quiet early-success behavior. Ban no-match is valid only after all requested roots were inspected. A missing runtime root must fail even when a different existing root has a match/no-match. No new semantic regex or documentation obligation.

Extend the existing broad fixture with selective narrow-checker invocations for documentation required search, API required search and EACH distinct ban. Both no-output errors and otherwise-valid partial-output failures must reach the intended call; an early documentation failure cannot certify the final serialization ban. Existing #406 dependency faults remain intact. Use existing required/forbidden helpers without changing their established grammar.

### Research

Preserve ten note names, sixteen exact heading strings, the current awk first-section nonempty rule, six nonempty support files, five console/DAW literal names, every template heading and all three synthetic-listening literals. Do not reinterpret headings as Markdown parsing or strengthen science content rules.

Capture independently: heading search; section-content awk; Primary-section extraction awk; its bracket-key extraction; distinct sorting/counting; WHOLE-note key extraction/delimiter conversion/unique sort; each bibliography literal lookup; console-name queries; template queries; each final listening query. The bracket grammar remains `\[[A-Z0-9][A-Z0-9-]+\]` (at least two characters inside brackets), and bibliography membership remains the exact literal `- \`[KEY]\``. Do not silently anchor it or widen the key grammar. Two distinct keys are required specifically in Primary; references outside Primary still require bibliography resolution. Empty Primary key output fails count; duplicate same key does not count twice. No lost process-substitution producer status.

A synthetic structural fixture can generate the frozen headings/notes and two dummy bibliography entries. Include a key outside Primary unresolved in the bibliography, not just a Primary missing-key case. Cover missing note/support artifact, empty section, duplicate/insufficient Primary keys and final literal absence. These are structural tests with no listening/scientific claim.

## Directed fault and actual counter controls

Keep a compact callsite table in the evidence/tests. For each distinct retained external operation above, use a selector that lets preceding operations and fixture construction succeed, then emits (a) error-only and (b) the real otherwise-valid output followed by error. Capture status and stage diagnostic; fail explicitly on unexpected checker success. Generic missing-input failures or broad `any policy failure` do not earn selective coverage. Identical repeated queries through one checked helper need one representative fault per semantic class, while genuinely different late consumers still need coverage. No all-command global fault shim.

Minimum actual counter-control groups, using existing fixture/same intended assertion:

1. Environment discovery status loss after valid listing.
2. Environment late source/vocabulary or final difference consumer status loss (select the latest retained consumer; early listing must remain successful).
3. Migration final serialization-ban error treated as no-match.
4. Research whole-note key producer error swallowed after valid keys, plus a late bibliography/listening consumer error treated as successful presence. These may share one compact control helper but are separate actual callsites.

For each, mutate the actual relevant gate/helper error handling in a disposable fixture copy, rerun the SAME targeted negative assertion, and show that assertion rejects the mutant for unexpected success. Then the normal checker must pass the positive fixture and reject the injected error. A manufactured standalone should-panic test or unconditional forced failure is not a gate mutation. Avoid regex edits that match zero occurrences; prove intended replacement occurred. No committed mutants, generic mutation runner or added matrix.

## Delivery

One Luna attempt, then at most two Sol revisions after adversarial FAIL; three total is a hard stop/rescope. Exact-path checkpoint after coherent pass, root owns push/issue synchronization. Three real gates, affected suites, syntax/diff and helper suite only if modified precede source review. Existing proportional full-workspace unchanged-count/required CI and actual-head Astra PR review remain final delivery gates. No builds or measurement are part of this readiness approval, and this file does not authorize implementation before the post-440 base freeze.

## Frozen post-prerequisite base

PR #440 delivered #411/#438 as `e7e1a37f36fe8a22c237d0bfcd3737373c6d4deb` after actual-head Astra PASS and required qualification SUCCESS. Both issues are remotely verified CLOSED. Root freezes that exact main for #412 and requests Astra confirmation before Luna assignment. The shared helper API is unchanged by #411/#438. #427 follows this tooling slice; #429 remains the sole independent active runtime feature.
