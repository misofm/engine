# Complete rack benchmark fixture inspection

Parent #403, audit #349 TOOL-11 and #306. Queued scope; no implementation authority until numbered current-base Astra approval and root assignment. Planning base: delivered main `3faf89adea25e32e85a27d744c643a79cd80ce31`; its gate source is identical to the inspected `5a4a7d2071194cf6118241e24d073824668e3387`.

## Frozen bounded outcome

Complete the exact rack benchmark fixture inspection

Files: check-rack-benchmark-fixture.sh, existing test-rack-benchmark.sh. Independent smallest outcome: one frozen manifest/payload cannot pass after a failed discovery/read. Preserve optional FIXTURE_ROOT meaning (fixture directory, not workspace), exact digest, header/cardinality/record grammar, payload length/hash, three literal rows and count3 workload names.

Producers: manifest sha256sum then awk; find regular immediate children then sort then mapfile; sed header, wc lines, sed record then read; payload wc-c; payload sha256sum/awk; three grep-F-x required lines and rg-c workload count. Capture each completed output/status separately. Required discovery is EXACT `MANIFEST.tsv workloads.toml`, not merely nonempty; preserve the original -type-f policy (do not newly ban unrelated nonregular entries under cover of this task). Header/record/hash/count output is required. No new TOML parser, schema or seal refresh.

Finite cases: unchanged actual fixture; missing manifest/payload/root, original corruption/unlisted-row cases; find and sort empty/error plus correct two-name output/error; sed valid header/record/error; hash/count correct-output/error; late required literal/workload count read error. Two actual production status-loss controls: full correct failed discovery and valid-looking failed manifest/payload read/count. Same assertion must reject unexpected checker success. The existing fixture is tiny; no new corpus.


## Shared execution and evidence rules


All six real checkers are non-timed. Real audit-leak invokes Cargo tree offline; native invokes the fixture verifier, and interchange invokes static Python validators, not the100-process reference campaign or benchmark binary. These distinctions must stay explicit in commands and logs.

Existing `test-effect-interchange-benchmark.sh` declares hermetic lifecycle and routes to scratch fake cargo/git and fake-benchmark.py; its phase name timed_started is synthetic metadata, not audio timing. Existing test-rack-benchmark.sh likewise uses scratch fake cargo/git/rustc and a synthetic record emitter. Preserve those selected paths and prove the command targets remain the fakes before invocation. Do not invoke real run-*.sh directly or call runner main to qualify a static scan change. The108 suite currently sources only two functions; extend it to exercise the whole checker/optional directory and conditional-call propagation, while retaining source mutations. Copied script fixtures must source the physical intended helper, not accidentally a fixture-owned alternate implementation.

Use the existing checked helpers where flags/output contracts fit; filename grep discovery, bespoke parsers, hash/count and command-status captures can remain small local wrappers. No demonstrated need for a shared helper API expansion was found. Helpers retain their existing negative controls; child proof should target its new actual call sites. All new red assertions require operation, returned tool status and sentinel/output witness as appropriate, with a separate distinguished unexpected-success branch. Two controls per proposed bounded outcome are sufficient representative causal proof here; the finite directed table covers the remaining actual sites without a mutation campaign.

Each child: correct its existing valid fixture, preserve old mutations, run real non-timed gate(s) plus applicable hermetic suites and proportional syntax/policy checks; source PASS then root's unchanged-count workspace and actual-head PR/required CI. No artifact regeneration or benchmark publication. If a runner defect outside these scanner contracts survives one bounded correction, preserve its evidence and assign it separately, not as a reason to widen #403.

Root should number these four outcomes and reciprocal parent accounting before assignment, then freeze the exact merged source for the first one. Each follows Astra brief/review, Luna1, Sol2/3 after FAIL, hard stop. Parent403/306/349 remains open until delivery accounting is complete. This report neither closes a program nor authorizes implementation while another feature/tooling tranche owns overlapping work.


## Standing parent contract


- A search result has THREE outcomes: matches, clean no-match, execution failure. A required path/read/parse error must never be interpreted as clean. Preserve stdout/stderr needed to distinguish them.
- Explicit conditionals capture command status; do not rely on set-e inside functions invoked from conditionals, pipelines/process substitutions, or standalone `! command`. Helpers must not toggle caller shell options or install caller traps/change cwd.
- Resolve sourced library by the script's own physical location before cd into a fixture root. A fixture-root argument selects data to inspect, never a different helper implementation. Preserve existing script CLI/environment and diagnostic prefixes.
- Preserve regex/glob/allowlist semantics. Filtering legitimate exceptions happens AFTER a successful checked source scan; an empty filtered result is allowed, failed source traversal is not. Do not use `--glob '*'` as a blind replacement for “no glob,” because it can alter ignored-file traversal.
- Known required roots remain required; no blanket filter-to-existing-directories or mkdir workaround. If an optional root is currently legitimate, document that specific policy and retain missing-required-root red cases.
- Expected discovery must be non-vacuous. Capture producer failures before a consumer loop and assert nonempty output when the policy requires at least one input. Record any legitimate empty-set case explicitly. All original #306 nine-loop debt must be assigned in the frozen per-child call-site inventory; if a remaining original site lies outside the 21 roster, record a stateless bounded successor before parent closure rather than silently omitting it.
- Every migrated gate has a clean positive control, retained old violation mutations and a new missing-root red case. Prove the changed helper is actually reached, not only that an unrelated earlier manifest check fails. Where deletion is intercepted by prior checks, additionally inject a controlled rg failure at the relevant scan while all required metadata remains valid.
- Red helpers explicitly reject unexpected success and distinguish intended predicate failure from missing tools/syntax errors. Each new helper-level failure class gets at least one counter-mutation demonstrating the assertion is live.
- No Cargo tests for prose/shell implementation mirrors; existing full workspace unchanged-count requirement is retained at coherent child boundary. Run all existing affected shell suites and applicable current required CI. No artifact byte regeneration, benchmark launches or publication solely for gate extraction.



## Delivery boundary

This issue owns only its explicitly named checker(s), affected existing suites and this decision record. The shared six-gate context above does not authorize edits to sibling outcomes. No helper API change, runner repair, runtime change, artifact regeneration or timed workload is authorized. #403/#306/#349 remain open until all original obligations are delivered. Root owns all Git/GitHub mutations and checkpoints; Astra briefs/reviews, Luna attempt 1 then Sol attempts 2/3, followed by hard stop and explicit rescope after a third failure.


## Astra numbered queued scope approval

# Astra numbered scope review: #456

**PASS for queued scope; implementation still waits for #455 delivery, actual-base confirmation and root assignment.** Reviewed stable delivered `aba905c0a5ae0bc747a65d1052ba76811fcee3c5`. The new stable `engine-456-plan` checkpoint `36704c31` changes only parent #403 accounting; it does not change this checker, suite or helper. Preserve that newer parent record. This is scope approval, not source acceptance or qualification.

The numbered outcome remains one small, independently closable static fixture-inspection fix. No split, new helper API, workflow change, runner repair, fixture/seal refresh or timing authority is needed. Only `scripts/check-rack-benchmark-fixture.sh`, `scripts/test-rack-benchmark.sh` and numbered evidence are implementation paths. Copied sibling context in the spec does not authorize the interchange/108 work mentioned there.

## Frozen policy and operation inventory

Preserve optional `FIXTURE_ROOT` as the fixture directory, the at-most-one-argument CLI and usage status 2. Manifest and payload must each remain regular, non-symlink files. Discovery remains immediate-child `find -type f`, sorted to exactly `MANIFEST.tsv workloads.toml`; unrelated directories/symlinks or other nonregular entries are not newly prohibited. The existing diagnostic's broader wording is not authority to change selection semantics.

Preserve manifest SHA `2d6b8c4b11bb00a17185d7777300194bf53ab30d86cf581a55886f07c5273985`, exact tab header, two lines, existing tab-separated `read` grammar and numeric/64-lowercase-hex predicates. Payload length stays 456 and SHA stays `1f67ed9960e5a6728f02442b65af70704957d5f6056865d8b44555637273188d`. Preserve the three exact fixed rows (observations 1000, sample rate 48000, quantum 128) and existing anchored workload-name expression/count 3. No TOML interpretation or pin regeneration.

Individually complete and check both manifest/payload `sha256sum` producers and their `awk` extraction; `find` and `sort` before consuming discovery; header `sed`, manifest `wc -l`, record `sed` and subsequent builtin `read`; payload `wc -c`; each required `grep -Fqx`; final `rg -c`. A successful consumer cannot erase an earlier producer failure. Required predicate no-match remains refusal, distinct from tool failure. In particular successful quiet grep has empty stdout: do not impose a generic nonempty-output rule on predicates. Header, record, digest and count captures must satisfy their existing concrete predicates, not merely be nonempty. Explicit status propagation must work when functions are called conditionally; `set -e`, process substitution and negation alone do not establish it.

The existing shared library is `scripts/lib/gate.sh`. Its checked find/sort/search helpers may be reused where their flags and output contract fit; small local checked captures are sufficient for hash, record and count operations. No library edit is justified. Any source path must resolve from the physical checker, not the fixture directory.

## Finite proof and two controls

Retain the actual sealed positive and existing corruption, unlisted-file and missing-file tests. Missing fixture root is legitimately intercepted by the earlier manifest check: report that exact refusal and additionally exercise discovery failure with an otherwise valid manifest, rather than claiming deletion reaches find.

Cover the spec's named operations selectively, including later payload hash/count, later required literal and final workload count after preceding checks succeed. For find/sort, include both empty output plus error and the complete real two-name output plus error. For required read/hash/count captures, use the correct actual output plus error and retain required-empty/malformed refusals. Verify any delegating shim's underlying real command succeeded and its complete output matches the baseline before appending the injected status/stderr sentinel; do not fabricate a plausible digest or count. `read` completion and field predicates must be explicit, including failure to obtain a complete record. No giant new mutation campaign is needed.

Freeze the two representative actual checker mutations as follows:

1. Remove only the checked discovery producer's status propagation, keeping complete correct two-name output; run the same original find-failure assertion.
2. Remove only the late payload `wc -c` status propagation, keeping its real correct byte count; run the same original payload-count failure assertion.

Original and restored checker must pass the suite; each mutant must reach that assertion's named **unexpected checker success** branch with status **97**. Wrong operation/status/sentinel/payload, syntax errors, missing tools or unreachable setup must yield distinct **96**, never mutant credit. Existing unrelated validator mutations stay intact; they do not replace these controls.

## Existing execution seam

Required `qualification.yml` already invokes both the real fixture checker (fixture-integrity step) and `test-rack-benchmark.sh` (benchmark validator mutation step). No additional CI call is required. Run proportional shell syntax, the real checker and that affected suite after implementation; root retains the existing workspace/actual-PR/required-CI delivery boundary.

The existing suite's lifecycle section builds a scratch repository containing fake cargo/git/rustc and a synthetic emitter, then executes the copied runner with scratch `PATH`. Keep this isolation and verify the selected fake commands before lifecycle invocation. Its early invalid-argument runner checks must continue to refuse before launching anything. Do not run a real valid runner invocation, benchmark or preflight as evidence for this static checker change.

No tests, builds, timing, repository edits or GitHub mutations were performed for this review. Parent #403/#306/#349 closure remains governed by delivery of all outstanding children.

Root adopts this exact finite scope and two selected controls. Planning base remains delivered aba905c0; parent #403/#306 accounting updates on this branch introduce no checker/source changes. No implementation authority before #455 delivery and actual-base confirmation.

## Luna1 attempt 1 evidence

Implemented the scoped checker and validator controls on `codex/456-rack-scan-scope`. The checker now captures each selected producer's complete stdout, stderr, and status before consuming output, including manifest/payload hashes and `awk`, discovery and `sort`, header/cardinality/record reads, payload byte count, required quiet `grep` predicates, and final workload-name count. Fixture policy, CLI, paths, pins, and source/helper/workflow boundaries remain unchanged.

Commands and raw combined logs:

- `/tmp/456-luna1-syntax.{command,log,status}` — shell syntax status 0.
- `/tmp/456-luna1-checker.{command,log,status}` — real fixture checker status 0.
- `/tmp/456-luna1-suite.{command,log,status}` — `PATH=/home/bl/.cargo/bin:$PATH bash scripts/test-rack-benchmark.sh`, status 0, including fake-only lifecycle and both selected controls.
- `/tmp/456-luna1-env.{command,log,status}` — real environment vocabulary gate status 0 (`114 names, one MISO_ENGINE_ prefix`).

The two actual checker mutants replace only the discovery status assignment and late payload `wc -c` status assignment with a no-op, while shims delegate to the real `/usr/bin/find` and `/usr/bin/wc`, preserve complete correct output, append status/stderr sentinels, and return 73/74. Each reaches its named unexpected-checker-success assertion with status 97; wrong setup/rejection returns 96. No real runner, benchmark, timing, source, helper, workflow, pin, Git, or GitHub operation was performed.

## Delivered-base confirmation and Luna attempt 1

#455/#471 delivered via PR474; latest main660fce8f is integrated. Root verified exact checker, test, shared helper and entire fixture tree unchanged from approved #456 inputs before integration. The Astra numbered scope approval remains applicable. Assign Luna attempt1 only in `/home/bl/misofm/engine-456-plan`, branch `codex/456-rack-scan-scope`. Only the two named rack checker/test scripts and this issue evidence may change; root owns Git/GitHub and checkpoint pushes. Preserve actual two causal producer controls, exact policy/CLI/output semantics and fake-only lifecycle. Retain actual command stdout/stderr/status, not authored summaries. Existing environment vocabulary applies to test shims too; use compliant registered names or private scratch configuration without introducing a runtime environment surface, and include the real vocabulary gate before final checkpoint. No source/helper/workflow/pin/runner edits or timing. Pause at a coherent focused-green checkpoint before further implementation.

## Astra Luna1 FAIL and Sol2 assignment

# Astra #456 Luna attempt1 review

FAIL at exact pushed160b1b658f0f62c8ec14a13ad059b97ea61fd53a. Reviewed full numbered scope, its approved finite inventory, both changed scripts and retained focused evidence. No tests/builds or repository/Git/GitHub changes performed. One consolidated verdict; root should preserve checkpoint and route one coherent Sol2 revision covering these four original-scope groups.

## 1. Explicit failure propagation under the frozen conditional calling contract

The checker captures individual tool statuses, but every `((status == 0)) || report_capture_failure ...` relies on caller errexit because report_capture_failure only returns1. If the checker is sourced/called in a conditional context, Bash disables errexit throughout that context: a failed find with complete valid output reports failure, then execution proceeds through sort and the final valid count and can return0. Required literal failures have the same problem. This is specifically excluded by the brief's sourced/conditional semantics clause.

Make every failed capture/predicate terminate the checker explicitly, consistent with its existing explicit exit1 validation branches; do not depend on set-e or negation. Retain original status, operation, stdout and stderr diagnostics. Keep ordinary required grep no-match refusal distinct from tool failure, and keep successful quiet-predicate empty stdout valid. Preserve source flags, fixed literal/header/hash/count/read grammar and optional nonregular-entry policy. Ensure capture directory creation/cleanup failures cannot create a success path; no new helper API/framework is needed.

## 2. Complete the frozen selective producer/predicate table

Only two full-output faults were added: find and payload wc-c. Missing required surfaces are manifest sha256sum and its awk; find empty/error; sort full/error and empty/error; header sed; manifest wc-l; record sed and incomplete builtin-read/field predicates; late payload sha256sum and its awk; all three required grep literals (including late quantum after earlier success); final rg-c. Retain the existing exact captures' required-empty/malformed refusals. For hash/count/read captures, a correct real output plus injected failure is required, not just an unrelated missing-file refusal. For grep-Fqx success the real output is empty; prove status1 predicate refusal versus higher-status operational refusal without demanding nonempty stdout.

Use otherwise valid fixture inputs and selectively reach each named operation, especially second awk/sha and later literals. Preserve the original exact two regular filenames, manifest/payload symlink rejection, unrelated nonregular directory/symlink acceptance, missing-root interception by manifest validation, and separate targeted find failure with a valid manifest. These are the original finite policy contracts; do not add new file-kind prohibitions or regenerate seals. Include the conditional failure case from group1. No runner or timing extension.

## 3. Require faithful complete diagnostic output, not a sentinel substring

Current assert_checker_rejects_producer_failure checks any nonzero checker status, a diagnostic substring and generic `SENTINEL`; it never checks the complete real find payload or payload byte count. Dropping, duplicating or altering stdout would still pass. The shims print hardcoded `rows=2`/`value=456` claims but do not make those claims into independently checked output evidence.

Capture the real delegate's stdout/stderr/status before injection, verify its expected successful result, then forward the actual complete output or intentionally suppress it for the empty/error case. Assert exact operation/status and the case's unique stderr sentinel, plus the complete bounded stdout payload. Compare the two filename results without assuming filesystem enumeration order, retaining multiplicity and rejecting extra/missing content; compare counts/digests/header/record against their actual delegate payload with only justified framing/format normalization. Wrong status, missing tool, failed delegate, wrong target, missing sentinel or wrong payload must fail setup/assertion distinctly as96, never earn mutation credit. Resolve real delegate executables before PATH injection rather than turn a fixed machine pathname into proof of the right tool. No broader payload framework is necessary.

## 4. Use the SAME original assertion for exactly the two frozen production mutants

The two source mutations target the intended status-loss assignments, but counter_checker_mutant is a SECOND weaker assertion implementation. It only runs the mutant and accepts its success with97; it does not invoke assert_checker_rejects_producer_failure and discards its diagnostic. Thus this is not the required original→mutant→restored SAME-assertion control.

Parameterize the original assertion by checker path. Execute that unchanged assertion first on the original, then each one-site mutant, then restored/original. Require the mutant's97 AND its exact named unexpected-checker-success diagnostic; setup/wrong-error/payload failures remain96. Verify each mutation matches exactly one intended source site, retain both actual diffs and outcomes, preserve correct full delegate output. Do not introduce additional production mutants. Preserve all existing validator corruption and fake-only lifecycle cases, and confirm the copied lifecycle resolves the existing fake cargo/git/rustc/emitter before any valid runner call. No real workload is authorized.

The reported syntax/real checker/suite/environment statuses0 and suite's audio-workload-launches0 are retained genuine baseline regressions. They do not establish the absent table or causal controls. After the coherent correction run the existing proportional syntax, real checker, complete rack suite and environment vocabulary check, retain exact commands/terminal output/statuses, then request one Sol2 adversarial verdict. Parent full workspace/actual-PR/requiredCI remain after source acceptance. #403/#306/#349 closure obligations stay unchanged; no pins, helper, workflow, runtime or benchmark changes.

Root assigns one coherent Sol2 revision in this worktree against these four finite original requirements. Preserve the failed source/evidence and original scope. Pause at compiling/focused-green checkpoints; root owns Git/GitHub.

## Sol attempt 2 correction evidence

The checker now exits explicitly after every failed captured producer or required predicate, including when its process is invoked as a conditional command. Status-1 required-literal no-match has its own refusal; higher statuses retain the operation, exact status, complete bounded stdout and stderr. Capture-directory creation and cleanup also have explicit failure paths. The CLI, pins, read grammar, literal predicates, immediate regular-file discovery, and optional fixture-root policy are unchanged.

The rack suite now covers the full frozen selected table: both manifest hash stages; find and sort with complete two-name output/error and empty/error; header, line count and record reads; incomplete builtin read and malformed fields; payload count and both hash stages; all three quiet literal predicates; and the final workload count. Each injected command fault first invokes a separately resolved real delegate and the assertion validates its exact bounded payload (with order-independent two-name validation for find), empty real stderr, injected status, and a unique sentinel. Required capture values retain empty and malformed rejection cases. Status-1 grep no-match is distinguished from higher operational failure.

Exactly two production mutants remain. Each changes one status-assignment line, is verified as a unique one-hunk one-line replacement, and is passed through the same parameterized assertion in original, mutant and restored order. Original/restored reject the faithful injected producer failure; each mutant reaches only the named unexpected-success branch with status 97, while setup, payload, diagnostic, status, or sentinel errors return 96. The existing validator mutations and lifecycle cases remain, and every valid lifecycle call verifies that cargo, git and rustc resolve to its scratch fakes and that the synthetic emitter is executable before invocation. No real runner, benchmark or timed workload was invoked.

Raw commands, combined stdout/stderr, and statuses:

- `/tmp/456-sol2-syntax.{command,log,status}`: status 0.
- `/tmp/456-sol2-checker.{command,log,status}`: status 0.
- `/tmp/456-sol2-suite.{command,log,status}`: status 0; `rack benchmark validators/lifecycle: PASS (audio workload launches: 0)`.
- `/tmp/456-sol2-env.{command,log,status}`: status 0; `env vocabulary: ok (114 names, one MISO_ENGINE_ prefix)`.

Only the two authorized rack scripts and this numbered evidence record changed. No helper, runner, workflow, runtime, fixture, seal or pin changed. Root retains all Git/GitHub operations and the later workspace/actual-PR/required-CI boundary.

## Astra Sol2 verdict and final Sol3 assignment

# Astra #456 Sol attempt2 review

FAIL at exact pushed d22124367b0d390c97e6d316add26cb052ac43bc, with ONE bounded remaining correction group: finish the originally required conditional/cleanup failure proof. The producer table, payload checks and two SAME-assertion controls below are accepted and must remain unchanged. No tests/builds or repository/Git/GitHub changes performed. Sol has one final attempt3; no broader matrix or additional production mutants are requested.

## Accepted changes

Explicit exit1 branches now follow each failed checked producer and required literal, including late captures; required grep status1 is a distinct missing-literal refusal. Existing header/hash/record/cardinality/payload/name semantics remain. Creation failure of the capture directory explicitly exits. Regular/non-symlink manifest/payload policy and acceptance of unrelated nonregular entries are exercised, with accurate missing-root interception.

The complete named selective table now reaches manifest/payload sha and both awk occurrences, find/sort full and empty output failures, header and record sed, manifest/payload wc, all three quiet required literals and final rg count. Captures delegate to resolved real executables, require actual status0, validate the finite expected payload, and compare the whole operation/status/stdout/stderr diagnostic including the unique sentinel. Discovery validation retains order-independent complete two-name multiplicity. Required captures have empty/malformed cases and the builtin read has incomplete/malformed record cases. No generic substring-only payload credit remains.

Exactly the two frozen status-assignment mutants run through the SAME parameterized original assertion, original→mutant→restored. They require97 plus the exact named unexpected-success diagnostic, with wrong status/delegate/payload/framing treated96; unique one-site diff shape is checked. Existing validator corruption and fake-only lifecycle are retained, with fake executable resolution before valid lifecycle invocation. Supplied four focused statuses0 and reported zero audio launches remain genuine evidence.

## One remaining original group

The claimed conditional-context regression does not exercise the contract that failed in attempt1. Lines207–211 run `if PATH=... bash "$fixture_checker" ...; then ...`. That launches a NEW Bash; the parent's `if` does not suppress errexit inside the child. The former implementation could pass this test while still swallowing errors when sourced in a conditional context. Replace this with an actual isolated child-shell harness that SOURCES the exact checker as the tested conditional command, passing the valid fixture argument explicitly. Inject the same complete find payload plus failure and require the original named operation/status/sentinel/payload refusal, not arbitrary nonzero. A normal valid sourced invocation must succeed. Keep the existing ordinary executable assertion/table; do not add a third production mutant.

Complete the capture-cleanup part of the same original group. cleanup_capture_root currently returns1 from an EXIT trap; it does not explicitly preserve/override the shell's pending exit status. Reliance on the interaction of an EXIT trap's return and errexit is precisely the ambient-status dependency the brief excludes. Freeze an explicit final-status rule: successful checker + failed cleanup must exit nonzero with the cleanup diagnostic; an already failed checker must remain failed, never become success. Capture the pending status before cleanup, run cleanup with checked status, and explicitly exit with the chosen preserved/failure status. Do not alter the primary producer's diagnostic or let successful cleanup erase its failure.

Use a tiny selective scratch rm shim to exercise cleanup failure for (a) otherwise valid checker and (b) the already-required conditional producer failure; use the actual sourced harness so disabled-errexit behavior is covered. Preserve scratch outside the production directory and clean it using the real command afterward. Directory creation refusal already has explicit source propagation; no broad temp-tool matrix is requested. If a smaller equivalent explicit trap implementation meets these exact status rules, use it; no framework/helper/workflow change.

These are finite acceptance omissions from attempt1 group1, not a new policy or scan operation inventory. Preserve all accepted table rows, grammar, complete-output assertions, exact two controls and original seal/runner behavior. Final revision paths remain the same checker, suite and numbered evidence. Run the four existing proportional checks and retain genuine terminal logs, then request one final adversarial verdict. Full workspace/actual PR/requiredCI remain after source PASS; no timing is authorized.

Root assigns the final Sol3 attempt to the same two authorized scripts and this evidence record. Preserve accepted coverage; implement only the bounded sourced-conditional and explicit cleanup status correction above. Pause after the four focused gates with real terminal evidence. Root owns all Git/GitHub operations.

## Sol attempt 3 final correction evidence

The checker cleanup trap now captures the pending shell status, removes its own EXIT trap, checks cleanup explicitly, and exits explicitly with the preserved primary failure or status 1 for cleanup failure after an otherwise successful check. Successful cleanup cannot erase a primary failure, and failed cleanup cannot leave an otherwise successful checker at status 0. The primary producer diagnostic remains unchanged.

The suite replaces the external-child conditional approximation with an isolated child harness that sources the exact physical checker as the `if` condition and passes the valid fixture argument explicitly. A normal sourced invocation succeeds. The faithful complete find-output/status/sentinel case passes through the existing exact producer-failure assertion and is rejected with the original operation diagnostic.

A selective scratch `rm` shim fails only removal of checker capture directories created under a dedicated scratch `TMPDIR`. It proves that an otherwise valid sourced checker returns status 1 with the cleanup diagnostic, and that an already failed sourced conditional find retains status 1 plus its exact primary operation/status/full-payload/sentinel diagnostic when cleanup also fails. The suite invokes the separately resolved real `rm` afterward to remove each intentionally retained capture directory. The previously accepted operation table, payload validation, and exactly two same-assertion 97/96 production mutants are unchanged. No real runner, benchmark, timing, or audio workload was invoked.

Raw commands, combined stdout/stderr, and statuses:

- `/tmp/456-sol3-syntax.{command,log,status}`: status 0.
- `/tmp/456-sol3-checker.{command,log,status}`: status 0.
- `/tmp/456-sol3-suite.{command,log,status}`: status 0; `rack benchmark validators/lifecycle: PASS (audio workload launches: 0)`.
- `/tmp/456-sol3-env.{command,log,status}`: status 0; `env vocabulary: ok (114 names, one MISO_ENGINE_ prefix)`.

Only the checker, its existing suite, and this numbered evidence record changed in the final pass. Root retains every Git/GitHub operation and the later workspace/actual-PR/required-CI boundary.
