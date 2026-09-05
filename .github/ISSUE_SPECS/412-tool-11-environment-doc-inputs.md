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

## Frozen-base approval and attempt 1 assignment

Astra reviewed exact head `b4a3a5d6866a9600910b593c03dd431a025aa7f7` on delivered main `e7e1a37f36fe8a22c237d0bfcd3737373c6d4deb` and recorded PASS. The three target gates, shared helper, existing suites and workflow are unchanged by #411/#438; the approved selective fault controls, NUL preservation, grammar and single research-suite CI invocation remain binding. Root authorizes Luna attempt 1 within this frozen scope. Review: `/tmp/astra-412-frozen-base-review.md`.

## Luna attempt 1 checkpoint

Luna implemented the three scoped gates, two existing suites, new research suite and one adjacent CI invocation. The shared helper now terminates rg options before patterns, permitting the research bibliography pattern beginning with a hyphen; conditional helper scope was used. Luna reports three production gates, three affected suites, Bash syntax and diff checks passing in `/tmp/luna-412-*`. Root additionally ran the required shared helper suite successfully in `/tmp/engine-412-root-helper-suite.log`. This is a compiling/green checkpoint for adversarial review, not an acceptance claim: Astra must verify complete selective producer/consumer coverage and actual same-assertion counter controls against the frozen brief. No workspace, benchmark, artifact or browser run is claimed for this tooling checkpoint.

## Astra attempt 1 verdict and Sol attempt 2

# Astra #412 Luna attempt 1 review

**FAIL at `6e6e6eaa4f2fcae11b06506b1c004b272f66cb2f`.** One bounded Sol revision is required against the already frozen producer/grammar/fixture contract. Reviewed complete nine-path delta, three checkers, actual new tests and approved brief. Ran only three tiny disposable environment probes; no Cargo/builds/timing or repository/GitHub mutations.

Useful progress: source file enumeration stays in a NUL-containing temporary file until conversion, per-file grep now distinguishes match/no-match/error, migration uses the shared required/forbidden helpers, research whole-note extraction is no longer hidden behind process substitution, and the one allowed research CI invocation is present. These do not complete acceptance.

## 1. Environment source still falsely passes errors and changes grammar

Independently reproduced against a tiny fixture under `/tmp/astra412-probe-1vszjtkd`:

- Existing required tools/scripts directories with no names and vocabulary `table` exit0: `env vocabulary: ok (0 names, one MISO_ENGINE_ prefix)`. Frozen used/documented populations are required nonempty.
- A vocabulary row missing its closing backtick is accepted with one name. New documented grep dropped the closing backtick from the original exact row regex. Restore the exact original grammar.
- A selective Git shim emits `fatal: simulated repository error` and exits128; the checker falls back to find and exits0. Status128 alone cannot identify genuine non-Git input, and stderr was discarded. Preserve deliberate non-Git fixture support without treating every fatal repository/process error as that case.

Additionally, `stray_names="$(sort ... | grep ... || true)"` still swallows both producer/consumer failures outright. Other grouped pipelines accept aggregate status1 even when an earlier transform failed (e.g. first exclusion grep error followed by clean no-match second filter). Used/vocabulary grouped transforms and final display count likewise do not implement the explicitly frozen per-producer contract. Check each retained operation before consumers, use real successful no-match only where permitted, require tools/scripts roots/nonempty used and documented sets, preserve diagnostics, and do not depend on global pipefail/errexit. Reuse captured populations; no repeated discovery/framework.

## 2. Research still conflates producer errors with allowed emptiness

The Primary-key and whole-note chains inspect only a combined pipeline status and treat status1 as clean no-match. A sort/tr execution failure returning1 is not a valid empty key population. Whole-note such failure can bypass bibliography checks after valid Primary counts. Counts also rely on implicit command-substitution errexit. Separately capture/check extraction, conversion, uniqueness/count and lookup stages as frozen; shell builtins may replace unnecessary external transforms. Preserve the current exact bracket grammar, literal bibliography lookup, sixteen headings and nonempty rule rather than broadening research policy. All late console/template/listening input errors remain in the directed table.

## 3. The purported counter-controls do not exercise the claimed mechanism

Migration mutant is copied to `$temp/check-effect-state-migration-mutant.sh`; physical sourcing resolves `$temp/lib/gate.sh`, which does not exist in the fixture (the helper is `$temp/scripts/lib/gate.sh`). It fails before the intended serialization call. Its assertion accepts any nonzero exit and discards output. It neither proves a replacement occurred (grep only checks the existing line label) nor reruns the SAME intended fault assertion expecting unexpected success from the swallowed-error mutant.

Research has the same misplaced-mutant/helper failure. Its shim exempts `-o` key extraction and injects on earlier filters.md searches, not the named whole-note producer. The sed replacement touches both sort pipelines rather than uniquely targeting the whole-note handling. The earlier removed listening literal remains missing in the actual `$temp/dsp-research`; `$temp/mutant-research` is unused. Thus independent early failures certify the printed counter result. No intended late consumer counter exists.

The new env “counter-control” is a Git listing fault injection only: it makes no actual gate mutation and accepts arbitrary failure without the stage diagnostic. Required environment discovery and late-consumer mutation groups are absent.

Replace these with the exact frozen actual-counter groups using valid otherwise-passing fixtures and physical script/helper layout. Assert the replacement count/changed callsite, run the same selective negative assertion against normal and mutated code, and require the mutant to be rejected by THAT assertion for unexpected checker success. Restore originals and prove positive green. No permanent mutants, generic runner, broad-any-failure or expected-panic substitute.

## 4. Directed fixtures omit most of the frozen callsite contract

Only Git listing and migration final-ban injections were added. Missing directed mechanisms include Git classification/find/path transformations/source versus vocabulary reads/both comm consumers; migration required doc/API and other two bans; research section/Primary/whole-note extraction and late bibliography/template/listening reads with otherwise-valid partial output. Cover the existing finite table from the final brief, grouping identical repeated helper predicates by semantic class but not substituting an early failure for distinct late consumers. Each needs error-only and otherwise-valid-output-then-error, exact status/diagnostic assertion and successful surrounding metadata. chmod-only unreadability remains unsuitable.

The research suite copies the real corpus instead of making the approved small synthetic structural fixture. Its “empty-section” mutation deletes the Fixtures heading itself, so only missing-heading is tested. Its “duplicate-primary” appends a key outside Primary, so it cannot establish fewer than two distinct Primary keys; the unchanged section still controls that count. Build the compact synthetic two-key fixture and directly establish missing note/support, genuinely empty existing section, insufficient/duplicate Primary, unresolved outside-Primary key and final literal cases. Preserve all existing env and #406 effect-runtime semantic controls while adding the scoped ones. No research content additions.

Shared helper `--` change: it is generally sensible option termination and the root helper suite passed, but no #412 helper call requires a leading-dash pattern (bibliography already uses its own `rg -F --` helper). The frozen conditional helper allowance does not justify unrelated API hardening. Prefer restore the unchanged helper for this slice; alternatively demonstrate a concrete necessary callsite before retaining it and test that exact behavior. No new helper framework/modes needed.

## Bounded Sol revision

Complete these four groups in the already allowed scripts/suites/spec and one existing CI call. Preserve useful checked migration/source scanning and all established grammar/empty-set rules. Record an explicit final callsite-to-selective-test mapping plus actual counter assertion evidence. Run three real checkers, three affected suites, shared helper suite only if changed, shell syntax/diff. No broad workspace/PR qualification before focused source acceptance; no runtime/artifact/benchmark scope.

Luna attempt1 is consumed; Sol has attempts2 and, only if needed, final3. A third FAIL requires a preserved checkpoint and explicit rebrief, not another repair. This verdict adds no obligations beyond the frozen #412 contract.

Root assigns Sol attempt 2 within these four original-contract groups. The earlier helper-necessity claim was incorrect: bibliography uses its own `rg -F --` call; restore the shared helper unless a concrete necessary callsite is demonstrated. No source repair has been made by root.

## Sol attempt 2 checkpoint

Sol restored the shared required/forbidden helper API and kept the leading-hyphen bibliography lookup local to the research checker. The environment checker now preserves NUL-delimited Git/find discovery until the intentional conversion, distinguishes only Git's actual non-repository diagnostic from other classification failures, checks the required roots and nonempty used/documented populations, restores the closing-backtick vocabulary grammar, and checks each retained filtering, scan, sort, comparison and count result before consumption. Producer diagnostics include otherwise-valid partial rows for Git listing, source scanning and vocabulary scanning.

The research checker now independently checks heading queries, section and Primary awk extraction, Primary and whole-note bracket extraction, delimiter conversion, unique sorting and literal bibliography/console/template/listening queries. Its two key populations remain distinct: two unique Primary keys are counted in shell, while every whole-note key is resolved with the original exact bibliography literal. Partial extraction output is retained on producer errors. Migration keeps the original required tokens/APIs and three bans; its suite selectively reaches documentation, API, runtime-owned, descriptor and final serialization searches in both error-only and partial-output modes.

The directed callsite mapping is recorded in the affected suites: environment covers real non-Git and initialized-Git positives, Git classification/listing, per-file source and vocabulary reads, the final `comm -13` consumer, required root/file/symlink and semantic equality mutations; migration covers one representative required-documentation query, required-API query and each distinct ban; research covers heading, section awk, Primary awk/key extraction, whole-note extraction, bibliography, console, template and final listening queries. The synthetic research fixture directly exercises missing note/support, an existing empty section, insufficient and duplicate Primary keys, an unresolved key outside Primary and a missing final literal.

Actual same-assertion disposable counter controls target environment Git listing and final unused-name comparison, migration's final serialization ban, research's whole-note producer and final listening consumer. Each proves a unique source replacement, runs the production negative assertion first, reruns that same assertion against the mutant and rejects the mutant because the intended diagnostic disappears, then restores the physical positive fixture. No mutant is retained.

Focused evidence at this checkpoint: all three real checkers, all three affected mutation suites and the shared helper suite pass together in `/tmp/sol-412-focused.log` (`focused_rc=0`). Individual research, runtime and environment runs are retained in `/tmp/sol-412-research.log`, `/tmp/sol-412-runtime.log` and `/tmp/sol-412-env.log`. No Cargo, workspace, benchmark, artifact, browser, Git or GitHub mutation was run by Sol.

## Sol attempt 3 final checkpoint

The final bounded revision narrows non-Git fallback to Git's exact ordinary no-repository result when neither `GIT_DIR` nor `GIT_WORK_TREE` is configured. The environment suite now proves ordinary non-Git and initialized-Git positives, a selective fatal probe, and a real initialized fixture with an invalid configured `GIT_DIR`; the configured-repository failure is retained and rejected.

The environment operation table selects find, path NUL conversion, normalization, both exclusions, source scans, stray sorting/prefix filtering, used-name scan/fragment filtering/sort, vocabulary scan/delimiter conversion/sort, both `comm` consumers, count and count formatting. Each selected operation runs once error-only and once after delegating the complete real fixture operation; the checker retains the delegated output before reporting the exact stage/status. Research similarly runs error-only and delegated-real-output cases for its two awk producers and both Primary/whole-note `tr` and `sort` stages, in addition to its rg/helper table. Migration required queries delegate their real matching rows before failure, while negative bans delegate their correct empty result before failure.

All five disposable counter groups now distinguish unexpected checker success with status 86. The production assertion returns zero only for the intended diagnostic and payload; wrong diagnostics, missing payloads, syntax/setup failures and unrelated policy failures return a different status. Each mutant replacement count remains exactly one, and the counter accepts only status 86 plus the named `unexpectedly succeeded` result: environment Git listing and final `comm -13`, migration serialization, research whole-note extraction and final listening lookup. Positive physical fixtures pass after the controls. The env checker and research suite executable modes are restored to 100755.

Final focused evidence is split only to keep each command below the execution watchdog: the three real gates and env/migration suites are green in `/tmp/sol-412-final-focused.log`; research and the shared helper suite are green with `focused_tail_rc=0` in `/tmp/sol-412-final-focused-tail.log`. Individual complete suite logs are `/tmp/sol-412-final-env.log`, `/tmp/sol-412-final-runtime.log` and `/tmp/sol-412-final-research.log`. No Cargo, workspace, benchmark, artifact, browser, Git or GitHub mutation was run by Sol.

## Astra attempt 2 verdict and final Sol attempt 3

# Astra #412 Sol attempt 2 review

**FAIL at `8e5f597557be44f717fee26d02b393dd71c4a6cb`.** The implementation improves source completion, but the frozen error-classification and directed/counter-proof contract is still incomplete. One final coherent Sol attempt 3 remains; another FAIL must hard-stop/rescope.

Reviewed the three complete checkers, affected suites, cumulative helper/workflow delta and full frozen brief/prior verdict. Shared helper is restored to the prerequisite API, and the workflow retains exactly the one authorized research-suite invocation. Environment paths remain in a NUL-containing file until intentional conversion; required roots/nonempty populations and closing-backtick grammar are restored. Research extraction/conversion/sort stages are now separately checked, and its finite synthetic fixture correctly tests an existing empty section, distinct Primary keys and outside-Primary bibliography resolution. Migration retains the exact tokens/APIs/bans and helper semantics. Keep these useful changes.

Independently ran two tiny disposable probes; results are `/tmp/astra-412-attempt2-probes.log`. No repository/Git/GitHub mutation, broad build or timing performed. The supplied combined focused run reports all real gates/suites green; the probes below show why that is not yet sufficient.

## 1. Git fatal classification still has a concrete false pass

The fallback condition accepts any status128 output containing `not a git repository`. In an actual initialized Git fixture, set GIT_DIR to a nonexistent repository: real git reports a fatal invalid-repository error with that substring, and the checker selects find and exits0 (`env vocabulary: ok (1 names, one MISO_ENGINE_ prefix)`). This is an execution/configured-repository failure, not the deliberately non-Git fixture case.

Make the nongit classification deliberately narrower, preserving genuine nongit positives while rejecting an invalid explicit repository/worktree configuration and other repository faults. Do not classify by status or a broad substring alone. Add that real initialized-repository failure and the original fatal-probe case alongside the nongit/Git positives, with diagnostic preservation. This corrects the original frozen classification requirement, not a new Git feature.

## 2. Actual counter wrappers still accept unrelated failure

All counter sites treat ANY nonzero return from assert_fault/assert_rg_fault/assert_migration_fault as a successful control. Those assertions return the same generic failure for checker success, missing diagnostics, missing partial output, syntax/setup failure or another policy rejection. The outer wrapper never requires the named unexpected-checker-success assertion and its distinguished status. The evidence's claim that losing the expected diagnostic suffices explicitly does not meet the frozen contract.

Independent reproduction replaced both research disposable mutants immediately before execution with a script containing only `exit 23`. The suite still exits0 and prints `dsp research mutations: ok`. Thus the physically corrected helper layout and source-replacement checks do not prevent the very unrelated-failure certification identified in attempt 1.

Use the same focused assertion for original and mutant, but distinguish checker unexpected success from every other assertion failure. Require only the exact intended assertion/status in the counter wrapper. Verify every actual edit affects the intended unique production callsite (including env edits after mutation), record executed original/mutant outcomes, and restore/prove positive fixtures. No generic runner is needed. Keep all five frozen groups: env discovery, env latest consumer, migration final ban, research whole-note producer and research late consumer.

## 3. The finite directed table and otherwise-valid payloads remain incomplete

The new tests do not implement the complete frozen per-operation table:

- Environment omits selective find, NUL conversion, normalization, both path exclusions, stray sort/prefix filter, tools/scripts used-name read, fragment filter, used sort, vocabulary delimiter conversion/sort, first comm difference and final count/format transforms. Current source/vocabulary/probe injections have only one partial shape, not their error-only/full-valid-output pair. The source shim's `source partial` is not the selected grep's valid output, and the vocabulary shim invents an undocumented PARTIAL row rather than the fixture's exact valid result. Missing scripts/empty used/documented semantic controls and fatal128 discrimination are not present in the affected suite.
- Research omits both Primary and whole-note tr/sort faults. Its two awk probes always emit `awk partial`; there is no error-only pair, and the Primary payload cannot satisfy the required two-key section if status is ignored. Primary/whole rg partials emit only `[DUMMY-A]` instead of the real otherwise-valid full extraction. The Primary result cannot pass the distinct-key rule; it is therefore not the promised full-looking producer proof. Select the actual operation/phase, delegate all other calls, and use its real valid output before injected failure. Preserve the distinction between Primary population and whole-note literal resolution.
- Migration now selects each required/banned semantic class in both modes, but its required-document/API payload is the same invented `intended partial row`, not that query's actual successful output. Replace it with delegated successful output plus an injected diagnostic/error. For negative predicates where correct output is empty, retain the clean-empty/error control and separately ensure plausible violation output cannot outrank the execution error.

Complete the already frozen table in these same suites. A command genuinely removed from production needs no resurrected test; commands still present do. Do not replace the missing cases with broad early faults, file permission changes or another corpus/framework. Each selective assertion must name the intended stage and injected error outcome, not merely any nonzero status.

## 4. Executable entry points regressed

The checkpoint changes `scripts/check-env-vocabulary.sh` and `scripts/test-dsp-research.sh` from mode100755 to100644. Bash-prefixed CI hides this, but the existing env checker can no longer be invoked directly as before. Restore the executable modes; no scope instruction authorized retiring that entry behavior.

## Final bounded revision

Complete these four existing-contract groups in one coherent pass, preserving the accepted source fixes, exact grammar, helper restoration, #406 controls and the single CI addition. Run the three real gates and affected suites plus syntax/diff; record the actual per-callsite mapping and distinguished counter outcomes. Root checkpoints before the final Astra verdict. Broader workspace/PR qualification waits for focused PASS. No runtime/artifact/benchmark change, speculative parser hardening, fourth attempt or gate weakening is authorized.

Root authorizes final Sol attempt 3 for these four frozen-contract groups. The executable-mode regression must be restored. A further FAIL requires a preserved hard-stop checkpoint and explicit rescope; no fourth repair or weaker assertion is authorized.

## Final Astra verdict: three-attempt hard stop

# Astra #412 final Sol attempt 3 review

**FAIL at `dbb47aac0f3d3fee8e21e05c673db0a7ca49847e`.** The remaining gap is confined to the environment suite's explicitly frozen Git-listing/required-root acceptance evidence. Preserve this useful checkpoint and hard-stop this three-attempt series; no fourth repair under #412.

The major prior blockers are resolved. Git fallback now requires the exact genuine non-repository diagnostic/status and absence of explicit GIT_DIR/GIT_WORK_TREE, and the real invalid configured repository is tested. Producer status checks in the environment/research sources remain complete; no concrete new false-pass was found there. Exact vocabulary closing-backtick/nonempty populations and research key/section rules are preserved. New transform probes delegate the real selected operation before injecting status7, with separate error-only legs; migration's required searches now receive real successful output. Research has the compact synthetic fixture and correctly distinguishes Primary from whole-note populations. Canonical source policies and conditional helper restoration remain accepted.

All five mutant wrappers now require the distinguished assertion result86 plus unexpected-checker-success text. A checker merely exiting23 cannot satisfy that route: it reaches diagnostic assertion failure1 rather than86. Physical helper placement and research late-fixture setup are corrected. The restored executable modes are755. Retained final env/runtime/research logs each report success; the combined watchdog interruption is not treated as a completed whole command. These improvements must be retained.

## Finite remaining evidence gap

In scripts/test-env-vocabulary.sh, the only Git-listing shim always executes real `git ls-files ...` and then exits7. It has no error-only mode. The normal checker is exercised with this shim only through the older broad `if PATH=... check ... >/dev/null 2>&1` assertion. The later `assert_fault` stage-specific assertion is invoked for the MUTANT but is never invoked for the original listing call. Thus this group has neither the frozen two payload modes nor the same precise original/mutant assertion pair. A generic normal failure and a specifically checked mutant outcome do not establish the required same-assertion causal control.

The prior attempt2 verdict also explicitly named a missing-scripts-root fixture. Final source still adds only missing-tools; there is no missing-scripts case with otherwise-valid vocabulary/tools population. This is a small omission, but the frozen final pass specifically committed to completing that finite table, and the suite must enforce it durably rather than inheriting a prose claim.

No additional production mechanism or unrelated tests are requested. The exact remaining completion is:

- Give the selective Git-listing shim error-only and real-full-output-then-error modes. Run the existing assert_fault on the ORIGINAL checker in both modes, requiring its Git-listing diagnostic and full listing evidence where applicable; retain the same assertion with its result86 unexpected-success route for the existing actual listing-status mutant. Keep the valid Git fixture and successful original checker restoration.
- Add the expressly requested missing-scripts fixture while tools/vocabulary and remaining metadata remain intact, and require the intended missing-root diagnostic. Keep missing-tools and all existing semantic controls.

## Hard-stop disposition

One bounded successor for environment acceptance-table completion is sufficient if root chooses to continue. It should inherit this accepted three-gate implementation and all current tests, touch only the environment suite and reciprocal evidence/spec, and run the affected suite plus actual original/mutant outcomes. #412 and its parent #402 stay OPEN until that completion and required delivery qualification; do not close the broad tooling finding or silently waive the two missing obligations. Root must number/synchronize and Astra approve that new bounded brief before edits. No fourth implementation pass in this series.

Read-only inspection of complete frozen contract, actual source/suites and retained logs; no builds/tests/timing or repository/GitHub mutations were performed. No requirement beyond the named final attempt2 table and original same-assertion rule was added.

Root records the hard stop and preserves the accepted source/test improvements at this checkpoint. No further implementation under this series is authorized. A separately numbered, synchronized and Astra-approved environment acceptance-table successor is required before the two remaining suite changes. #412/#402 remain OPEN; full delivery qualification remains outstanding.

## Explicit bounded successor

#448, “Complete environment discovery and required-root acceptance controls”, owns only the two remaining environment acceptance-table obligations under a new Astra-approved brief. The old #412 series remains stopped. Accepted production and other suites are inherited unchanged; #412/#402 remain OPEN until successor completion and required qualification/remote delivery.
