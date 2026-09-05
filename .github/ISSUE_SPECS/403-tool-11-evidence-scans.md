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

## Astra bounded delivery refinement

# #403 current delivery brief — split before implementation

Read-only inspection on the stable source of delivered main `5a4a7d2071194cf6118241e24d073824668e3387` in `/home/bl/misofm/engine-402-closure`. Read full #403, #306's original loop accounting, all six checkers and the listed suites' invocation/stubbing seams. No tests, builds, timed workloads, source/spec changes or Git/GitHub mutations.

Recommend four bounded outcomes, numbered by root before coding. Keep #403 as reconciliation parent until all four deliver. These six scripts combine independent harness dependency scans, a native runner seal, interchange authority and a rack fixture seal; one implementation attempt across all four is not a credible half-day slice. Serialize the children and their helper use; #404 remains independent and untouched. Do not import unrelated runner repair, validators, benchmark measurement or the remaining21-gate program.

## A. Complete benchmark-harness ownership and production dependency scans

Files: check-bench-policy.sh, check-realtime-audit-leak.sh; existing test-bench-policy.sh and test-realtime-audit-leak.sh. This owns EXACTLY the three original #306 loops assigned to #403: bench's one manifest loop and audit-leak's structural and resolution loops. Their shared population/production-harness boundary makes this one useful outcome.

Bench's original scans are grep, not rg: sole_owner, shared escaper presence, escaper candidate discovery plus per-candidate awk, forbidden private SHA patterns, each timed subject's required timer and forbidden clock/digest, exact unsafe-owner set, exact metadata-reader set; then find/sort/awk manifests and final count formatting. Preserve ERE/include semantics, the exact escaper indentation/40-line/comment/backslash algorithm, exact owner sets and exemptions. Grep0 means match,1 clean absence, other statuses failure; no quiet positive scan that hides completion failure. Capture every producer before sort/filter/comparison; check awk even when its valid result is empty/delegate. A failed delegate parser currently can be interpreted as no offender. Exact-owner scans must contain their owner; forbidden scans may be empty; dependency violation output may be empty. Preserve legal duplicate-free set semantics without adding a new Rust parser.

Manifest discovery retains `find crates hosts sidecars -mindepth2 -maxdepth2 -name Cargo.toml` and sort. All three roots must exist; aggregate required nonempty, individually empty roots allowed. Each parser uses its existing section grammar. bench permits crate dev dependencies but bans host/sidecar occurrences as before. Audit-leak permits dev sections plus the existing engine/conformance feature declarations; preserve exact exceptions. Its second loop extracts each package name (required nonempty) and executes EXACT `cargo tree --locked --offline -p NAME -e features,no-dev --target all`. Capture Cargo stdout/stderr/status separately before complete grep. Cargo failure with a perfectly clean-looking graph must fail; grep1 is clean,0 violation,other failure. Cargo tree is non-timed metadata resolution, not a build or benchmark. Do not hide its stderr or accept an empty completed graph as proving a named package resolved; require nonempty Cargo graph output.

Existing bench fixture copies tools but creates only empty crates/hosts and omits sidecars: correct the fixture with all roots and one actual valid package manifest. Do not relax production roots. Audit suite currently copies the workspace and runs real offline Cargo tree; retain that real non-timed positive/old mutations. For directed failures, add a tiny valid manifest fixture and a selective Cargo shim returning faithful named-package output, with earlier operations real/delegated. No replacement resolver.

Finite cases: clean existing tree; valid delegate and empty violations; each missing root, empty aggregate, named late manifest read; grep empty/error and real matching output/error, sort complete-list/error, awk empty/error and delegate/error, clean Cargo output/error, and grep error after successful Cargo. Selective targeting must reach later subjects/packages. Two actual uniquely verified production status-loss controls: complete failed manifest discovery and clean-looking failed Cargo graph. Run the SAME original error assertion on each mutant, require distinguished unexpected-success outcome; unrelated setup/error diagnostic failures must not satisfy it.

## B. Complete both native PCM runner static seals

Files: check-native-pcm-runner.sh and its existing v1/portability suites. Preserve `[root] [v1|portability|all]`, exact prefixes and both mode outcomes. No runner execution or publication adapter redesign.

V1 producers: independent `generate.py --check` (verification only), find RIFF files -> wc exact4, required dependency/ABI matches, forbidden dependency matches, source bypass scan -> exact ABI exclusion, and four-root reachability scan -> own-package and doc-comment exclusions. Retain rf64 requirement and exact fixture identities. Source scans must finish before exception filtering; final empty allowed. Check find and wc individually even if failed find emits all four valid paths. Required roots are crates/hosts/tools/sidecars for reachability; an empty individual root is legal. Existing fixture must include them all. No new nonempty reachability requirement: this is a forbidden population.

Portability producers: required fixed boundaries/contract literals; forbidden impossible claims/hard-link/replacement/cleanup patterns; Python Unix-import predicate; counted identity checks>=2 and O_NOFOLLOW exactly4; late ownership checks. Preserve all current numeric/regex/Python behavior. Python status/read failure is fatal, not a new portability ruling. Required source/contract deletion is separate from selective later scan errors with intact inputs.

Finite controls: retain all fixture corruption, ABI bypass and portability mutations; add otherwise-valid missing required root/file, partial/error discovery/count, each positive/forbidden/filter class, late contract/source read. Two actual status-loss mutants: allowed complete source scan before exclusions, and late forbidden publication scan. Same targeted unexpected-success assertions, not generic nonzero. Use actual fixture generator --check; no Rust build or native runner invocation.

## C. Complete interchange qualification and successor authority scans

Files: check-effect-interchange-qualification.sh, check-effect-interchange-benchmark-108.sh; existing test-effect-interchange-policy.sh, test-effect-interchange-benchmark-108-policy.sh and affected test-effect-interchange-benchmark.sh. Keep these together because qualification sources108's functions and chooses the081/108 authority branch. Do not repair or edit runner/preflight/validator semantics.

Qualification must check each sha256sum/awk, sort-c, sha256sum-check, wc/tr result before consuming it. Preserve pinned accepted manifest hash, exactly27 rows, all record hashes and old refreshed-baseline mutation. Required path checks remain; none may be created in production. Required rg text/fixed/multiline modes preserve exact grammar without quiet completion masking. The optional issue108 branch predicate is genuinely tri-state:0 enters108;1 enters the existing081 validation; error must fail, not select081. Preserve both Python authority validators and all supported source/target/API identities. Forbidden dependency/reference-process/runtime/serialization scans allow successful empty but never missing-root/read failure. Export discovery/count remains exactly2; capture both scan and count completion.

Two find populations are VIOLATION sets: direct regular files beside ACCEPTED.sha256, and whole-tree generated artifacts under the existing target prune and exact suffix set. Empty is the correct clean result; do NOT require nonempty or fabricate a clean-looking prohibited artifact as a successful-output control. Remove proof dependence on quiet grep/early find quit; finish checked traversal before testing nonempty. Preserve the original regular-file/depth/prune scope, not a new artifact registry.

108 validates benchmark source and cross-file output authority in Python, then namespace via forbidden rg, and tests the optional target/issue108 directory for entries. Sourcing this checker exposes functions used in a conditional subshell: explicitly propagate each failure instead of relying on sourced set-e. Do not introduce helper-induced options/traps/cwd changes. The optional artifact directory may legitimately be ABSENT (the existing clean state) or present EMPTY; both pass. A present directory with any entry is a violation. Failure inspecting an existing directory must fail; do not equate arbitrary find status with absence. Distinguish absence through an explicit path decision first, then checked find for an existing directory. Do not create the directory as a workaround.

Finite cases: current081/108 authority semantics retained; valid current tree, optional-dir absent/empty, corpus no-extra/generated no-match; required missing source/root/manifest; hash/count producer error after correct output; branch search error, late forbidden/API/export scan error; both find failures with empty output (and diagnostic precedence for violation output/error); sourced function error with all earlier metadata valid. Two actual status-loss controls: the108 forbidden namespace scan error through the sourced function and a late qualification required/forbidden scan error. Require the same precise unexpected-success assertion. Violation-producing find errors need exact operation/status proof, not a forced mutant success that is impossible under the preserved violation predicate.

## D. Complete the exact rack benchmark fixture inspection

Files: check-rack-benchmark-fixture.sh, existing test-rack-benchmark.sh. Independent smallest outcome: one frozen manifest/payload cannot pass after a failed discovery/read. Preserve optional FIXTURE_ROOT meaning (fixture directory, not workspace), exact digest, header/cardinality/record grammar, payload length/hash, three literal rows and count3 workload names.

Producers: manifest sha256sum then awk; find regular immediate children then sort then mapfile; sed header, wc lines, sed record then read; payload wc-c; payload sha256sum/awk; three grep-F-x required lines and rg-c workload count. Capture each completed output/status separately. Required discovery is EXACT `MANIFEST.tsv workloads.toml`, not merely nonempty; preserve the original -type-f policy (do not newly ban unrelated nonregular entries under cover of this task). Header/record/hash/count output is required. No new TOML parser, schema or seal refresh.

Finite cases: unchanged actual fixture; missing manifest/payload/root, original corruption/unlisted-row cases; find and sort empty/error plus correct two-name output/error; sed valid header/record/error; hash/count correct-output/error; late required literal/workload count read error. Two actual production status-loss controls: full correct failed discovery and valid-looking failed manifest/payload read/count. Same assertion must reject unexpected checker success. The existing fixture is tiny; no new corpus.

## Shared execution and evidence rules

All six real checkers are non-timed. Real audit-leak invokes Cargo tree offline; native invokes the fixture verifier, and interchange invokes static Python validators, not the100-process reference campaign or benchmark binary. These distinctions must stay explicit in commands and logs.

Existing `test-effect-interchange-benchmark.sh` declares hermetic lifecycle and routes to scratch fake cargo/git and fake-benchmark.py; its phase name timed_started is synthetic metadata, not audio timing. Existing test-rack-benchmark.sh likewise uses scratch fake cargo/git/rustc and a synthetic record emitter. Preserve those selected paths and prove the command targets remain the fakes before invocation. Do not invoke real run-*.sh directly or call runner main to qualify a static scan change. The108 suite currently sources only two functions; extend it to exercise the whole checker/optional directory and conditional-call propagation, while retaining source mutations. Copied script fixtures must source the physical intended helper, not accidentally a fixture-owned alternate implementation.

Use the existing checked helpers where flags/output contracts fit; filename grep discovery, bespoke parsers, hash/count and command-status captures can remain small local wrappers. No demonstrated need for a shared helper API expansion was found. Helpers retain their existing negative controls; child proof should target its new actual call sites. All new red assertions require operation, returned tool status and sentinel/output witness as appropriate, with a separate distinguished unexpected-success branch. Two controls per proposed bounded outcome are sufficient representative causal proof here; the finite directed table covers the remaining actual sites without a mutation campaign.

Each child: correct its existing valid fixture, preserve old mutations, run real non-timed gate(s) plus applicable hermetic suites and proportional syntax/policy checks; source PASS then root's unchanged-count workspace and actual-head PR/required CI. No artifact regeneration or benchmark publication. If a runner defect outside these scanner contracts survives one bounded correction, preserve its evidence and assign it separately, not as a reason to widen #403.

Root should number these four outcomes and reciprocal parent accounting before assignment, then freeze the exact merged source for the first one. Each follows Astra brief/review, Luna1, Sol2/3 after FAIL, hard stop. Parent403/306/349 remains open until delivery accounting is complete. This report neither closes a program nor authorizes implementation while another feature/tooling tranche owns overlapping work.

## Numbered outcomes and current scheduling

- #453: Complete benchmark ownership and production dependency scans. Local spec `.github/ISSUE_SPECS/453-benchmark-dependency-scan-completion.md`.
- #454: Complete native PCM runner static seal scans. Local spec `.github/ISSUE_SPECS/454-native-runner-seal-scan-completion.md`.
- #455: Complete interchange qualification and authority scans. Local spec `.github/ISSUE_SPECS/455-interchange-authority-scan-completion.md`.
- #456: Complete rack benchmark fixture inspection. Local spec `.github/ISSUE_SPECS/456-rack-fixture-scan-completion.md`.

Root created and verified all four matching remote issue numbers/titles/bodies in this planning checkpoint. All are OPEN and queued; none has implementation authority. Planning base is delivered main `3faf89adea25e32e85a27d744c643a79cd80ce31`; its gate source matches the Astra-inspected base. Serialize these children after the active #404 tooling tranche, starting with #453 after numbered/current-base review. #453 owns exactly the three original #306 loops assigned to #403. The four children cover every original six-gate obligation without adding runner repair or timing. #403/#306/#349 remain OPEN through complete delivery reconciliation.


## Delivery reconciliation after PR #467

- #453 and its bounded completion #462 delivered the benchmark/dependency pair through PR #464 (`b6836835e1e2d309deee83f1bbe2ae9b5f2206fc`); both remote issues are closed.
- #454 delivered the native PCM checker through PR #467 (`aba905c0a5ae0bc747a65d1052ba76811fcee3c5`), after exact-head Astra PASS and required CI success; the remote issue is closed.
- #455 owns the remaining two interchange checkers and is active in Luna attempt 1 on the delivered #467 base. Its numbered scope includes the existing-suite CI wiring; no completion is claimed.
- #456 owns the remaining rack fixture checker and remains queued for numbered scope approval/current-base assignment after #455 delivery.

Three of the six parent checker paths are delivered; all six original obligations and the assigned #306 loop inventory remain required for final parent closure. This parent and #306/#349 remain open. Historical queued-state statements above describe their original checkpoints and are superseded by this delivery record.

## Interchange delivered; rack-scan boundary active

PR474 delivered #455/#471 at660fce8f2c4f76d38c82590f4c0411c117ba857d after exact-head Astra PASS and required qualification SUCCESS. Both issues are remotely closed. #403 has five of its six checker scopes delivered; only #456 rack fixture scans remain. This parent stays open pending the remaining delivery and its retained inventory/loop/accounting closure requirements.

## Rack checker delivered; final parent reconciliation pending

PR477 delivered #456 at `fa3485c6bb1a69e6dd01df734a1ad9c945964715`, after exact-head Astra PASS and required qualification SUCCESS; #456 is closed and unclaimed. All six #403 checker scopes are now delivered. Final retained inventory/loop/extractor reconciliation is being reviewed before claiming parent completion; TOOL-11's separately excluded framework work remains under #349. No original requirement is removed by this status record.
