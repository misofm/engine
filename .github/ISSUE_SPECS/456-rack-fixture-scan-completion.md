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

## Delivered-base confirmation and Luna attempt 1

#455/#471 delivered via PR474; latest main660fce8f is integrated. Root verified exact checker, test, shared helper and entire fixture tree unchanged from approved #456 inputs before integration. The Astra numbered scope approval remains applicable. Assign Luna attempt1 only in `/home/bl/misofm/engine-456-plan`, branch `codex/456-rack-scan-scope`. Only the two named rack checker/test scripts and this issue evidence may change; root owns Git/GitHub and checkpoint pushes. Preserve actual two causal producer controls, exact policy/CLI/output semantics and fake-only lifecycle. Retain actual command stdout/stderr/status, not authored summaries. Existing environment vocabulary applies to test shims too; use compliant registered names or private scratch configuration without introducing a runtime environment surface, and include the real vocabulary gate before final checkpoint. No source/helper/workflow/pin/runner edits or timing. Pause at a coherent focused-green checkpoint before further implementation.
