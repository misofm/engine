# Complete benchmark ownership and production dependency scans

Parent #403, audit #349 TOOL-11 and #306. Queued scope; no implementation authority until numbered current-base Astra approval and root assignment. Planning base: delivered main `3faf89adea25e32e85a27d744c643a79cd80ce31`; its gate source is identical to the inspected `5a4a7d2071194cf6118241e24d073824668e3387`.

## Frozen bounded outcome

Complete benchmark-harness ownership and production dependency scans

Files: check-bench-policy.sh, check-realtime-audit-leak.sh; existing test-bench-policy.sh and test-realtime-audit-leak.sh. This owns EXACTLY the three original #306 loops assigned to #403: bench's one manifest loop and audit-leak's structural and resolution loops. Their shared population/production-harness boundary makes this one useful outcome.

Bench's original scans are grep, not rg: sole_owner, shared escaper presence, escaper candidate discovery plus per-candidate awk, forbidden private SHA patterns, each timed subject's required timer and forbidden clock/digest, exact unsafe-owner set, exact metadata-reader set; then find/sort/awk manifests and final count formatting. Preserve ERE/include semantics, the exact escaper indentation/40-line/comment/backslash algorithm, exact owner sets and exemptions. Grep0 means match,1 clean absence, other statuses failure; no quiet positive scan that hides completion failure. Capture every producer before sort/filter/comparison; check awk even when its valid result is empty/delegate. A failed delegate parser currently can be interpreted as no offender. Exact-owner scans must contain their owner; forbidden scans may be empty; dependency violation output may be empty. Preserve legal duplicate-free set semantics without adding a new Rust parser.

Manifest discovery retains `find crates hosts sidecars -mindepth2 -maxdepth2 -name Cargo.toml` and sort. All three roots must exist; aggregate required nonempty, individually empty roots allowed. Each parser uses its existing section grammar. bench permits crate dev dependencies but bans host/sidecar occurrences as before. Audit-leak permits dev sections plus the existing engine/conformance feature declarations; preserve exact exceptions. Its second loop extracts each package name (required nonempty) and executes EXACT `cargo tree --locked --offline -p NAME -e features,no-dev --target all`. Capture Cargo stdout/stderr/status separately before complete grep. Cargo failure with a perfectly clean-looking graph must fail; grep1 is clean,0 violation,other failure. Cargo tree is non-timed metadata resolution, not a build or benchmark. Do not hide its stderr or accept an empty completed graph as proving a named package resolved; require nonempty Cargo graph output.

Existing bench fixture copies tools but creates only empty crates/hosts and omits sidecars: correct the fixture with all roots and one actual valid package manifest. Do not relax production roots. Audit suite currently copies the workspace and runs real offline Cargo tree; retain that real non-timed positive/old mutations. For directed failures, add a tiny valid manifest fixture and a selective Cargo shim returning faithful named-package output, with earlier operations real/delegated. No replacement resolver.

Finite cases: clean existing tree; valid delegate and empty violations; each missing root, empty aggregate, named late manifest read; grep empty/error and real matching output/error, sort complete-list/error, awk empty/error and delegate/error, clean Cargo output/error, and grep error after successful Cargo. Selective targeting must reach later subjects/packages. Two actual uniquely verified production status-loss controls: complete failed manifest discovery and clean-looking failed Cargo graph. Run the SAME original error assertion on each mutant, require distinguished unexpected-success outcome; unrelated setup/error diagnostic failures must not satisfy it.


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

## Astra numbered scope approval

# Astra #453 numbered scope review — PASS, queued after #404

Exact clean planning checkpoint `a8f4880d5e658392e533393b6eead8c40a4a3807`, `/home/bl/misofm/engine-403-plan`, based on delivered main `3faf89adea25e32e85a27d744c643a79cd80ce31`.

PASS for numbered scope/readiness. No further amendment is required before the post-#404 delivered-base check and root assignment. This approves the bounded brief, not implementation or current production correctness. All scripts are unchanged from the previously inspected `5a4a7d2071194cf6118241e24d073824668e3387`; the checkpoint adds only #403/#453–456 planning records.

Verified all four numbered outcome bodies retain their respective approved scope paragraphs without omission. #453 owns bench-policy and realtime-audit-leak plus their two existing suites; #454 owns both native runner modes; #455 owns qualification and108 authority plus their dependent hermetic suites; #456 owns the rack fixture seal. The parent lists all matching filenames/numbers/titles and retains all six original gates. #453 alone owns the original bench manifest loop and both audit-leak manifest loops, exactly the three #306 loops assigned to #403. Neither the additional sibling populations nor #404's workspace/Wasm remainder disappears into this count. Root's reported exact remote number/title/body verification is recorded as root evidence, not an independent remote verification by this review.

Rechecked #453 against actual source and suites. The known unchecked grep/sort/awk ownership/parser sites, timed-subject presence/forbidden scans, exact owner/environment sets, three find-backed loops, package-name extraction and Cargo-tree-to-grep pipeline are all covered. Expected empty results are appropriately distinguished: forbidden/manifest violation output may be empty; exact owner sets, manifest aggregate, each package name and successful resolved Cargo output may not. Individually empty required roots stay legal, but all crates/hosts/sidecars roots must exist. The frozen ERE/include/40-line delegate grammar, dev/forwarding exemptions, exact Cargo flags and existing CLI/prefixes remain binding.

The existing bench fixture still omits sidecars and actual package manifests; the numbered scope explicitly requires repairing that fixture rather than filtering production roots. The audit suite still copies a resolvable workspace and invokes real locked/offline Cargo tree, which is metadata resolution rather than timing. The brief retains its old direct-enable/dev-edge controls and confines new selective faults to valid fixtures with earlier operations delegated. Both scripts have no original rg sites; there is no permission to replace their grammar with rg helper output.

The actual shared helper supplies checked find/sort/count where applicable; it has no need for an API extension here. Preserve local grep/parser/Cargo captures when their output or flag contract differs. Source the intended physical helper before fixture-root changes. No helper source/test expansion, runner repair, Rust/runtime edits or sibling-suite work is authorized by the copied shared context; each numbered delivery boundary expressly limits it.

Finite directed cases retain empty/error and meaningful matching/delegate/Cargo-output error forms, later-file/package targeting, missing-root/read and empty-required-population distinctions. The two mandatory actual production mutants are complete failed manifest discovery and clean-looking failed Cargo graph. Both must execute the SAME original targeted assertion and reach distinguished unexpected success; setup/syntax/missing-diagnostic failures are not proof. Existing helper controls need not be duplicated. No new corpus or mutation framework is requested.

The four scopes retain the parent's original standing contract and unchanged-count workspace/actual-PR/CI delivery, while the later sibling briefs remain queued for their own current-base reviews. No benchmark executable is authorized. Start #453 only after #404 delivery and a proportional source/helper comparison against that actual merged base; unchanged source would not justify reopening this design. #403/#306/#349 remain OPEN until all retained outcomes are delivered and reconciled.

Read-only source/Git comparisons and scope inspection only. No implementation, tests, builds, timing, repository/spec or Git/GitHub mutation; only this /tmp verdict was written.

## Delivered #404 base integration

Root integrated delivered main `60519995c37f95e3f91abb45f45790ecad1ed244` after PR #458 merged and #404 was verified CLOSED. The two assigned checkers, their two existing suites, shared helper directory and helper suite have no source delta from approved planning `9119c6c65ea198e2eb7a6c6a07903a92b6ca1f19`. The branch differs from main only in the five #403/#453-456 planning specs. Root requests the proportional actual-base Astra approval before Luna assignment; no implementation has started.

## Astra delivered-base approval and Luna attempt 1

# Astra #453 delivered-base review — PASS

Exact clean checkpoint `14422a20d4b95310d2ba5751304a37f3a1b3b726`, `/home/bl/misofm/engine-403-plan`, includes delivered main `60519995c37f95e3f91abb45f45790ecad1ed244` as an ancestor.

PASS. Root may assign Luna attempt1 for the bounded #453 tooling outcome. No further design amendment or pre-assignment test run is required.

Read-only comparisons confirm the two assigned checkers, their two existing suites, shared helper directory and `scripts/test-gate-lib.sh` are unchanged from both approved scope checkpoint `a8f4880d5e658392e533393b6eead8c40a4a3807` and recorded planning `9119c6c65ea198e2eb7a6c6a07903a92b6ca1f19`. The entire branch difference from delivered main remains only five #403/#453–456 planning specs. The #453 delivered-base paragraph correctly records that boundary; root reports #404 merged/CLOSED with required CI success and #453 remote body synchronized.

The numbered PASS in `/tmp/astra-453-numbered-scope-review.md` therefore remains applicable unchanged: exactly bench-policy/realtime-audit-leak and their two suites; exactly three assigned original manifest loops; original grep/parser/exception semantics; required roots/nonempty aggregates versus legal empty violation results; checked Cargo-tree production; finite directed cases and two actual SAME-assertion status-loss controls. No helper expansion, runner work, benchmark, Rust/runtime change or sibling scope is authorized.

#453 may proceed as independent tooling alongside #459 runtime under root coordination. #454–456 remain queued, and #403/#306/#349 remain open until their complete retained outcomes are delivered. Source acceptance, unchanged-count workspace and actual-head PR/required CI remain future implementation/delivery gates.

No tests, builds, timing, repository/spec changes or Git/GitHub mutations were performed. Only this /tmp review was written.

Root assigns Luna attempt 1 on this approved delivered base. The complete two-gate/three-loop outcome and finite directed/control table remain mandatory. Use the named existing fixtures and real offline Cargo tree metadata resolution; no benchmark workload is authorized. Stop at a coherent focused-green checkpoint for root commit/push before layering.

## Luna attempt 1 implementation checkpoint

Luna completed its first coherent pass in the four authorized scanner/suite files. The reported changes add checked producer/status handling, required root and manifest/package populations, corrected valid fixture roots/manifests and directed status-loss tests. Root verified the exact four-path source delta and diff hygiene before checkpointing; this is not source acceptance or proof that the complete frozen directed table is satisfied.

Luna reports terminal exit 0 for: `bash -n scripts/check-bench-policy.sh scripts/check-realtime-audit-leak.sh scripts/test-bench-policy.sh scripts/test-realtime-audit-leak.sh`; each of the two real scanner scripts and two existing suites via `PATH=/home/bl/.cargo/bin:$PATH bash scripts/<name>.sh`; and a final syntax/suite/diff-hygiene rerun. No dedicated log files were retained by Luna: these are reported session executions, not retained-log artifacts. Root does not fabricate missing logs or count them as independent evidence. Astra will inspect source/suite coverage against the full issue and issue one consolidated verdict before further implementation or qualification. No benchmark, Rust build, full-workspace, artifact or Git/GitHub mutation was performed by Luna.
