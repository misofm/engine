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

## Astra attempt 1 verdict and Sol attempt 2

# Astra #453 Luna attempt 1 — FAIL

Exact checkpoint `820b410ca71320735e523f44f9a4bab7f2aa1dd2`, `/home/bl/misofm/engine-403-plan`, against the complete numbered two-checker/three-original-loop contract and delivered-base approval.

FAIL. The pass improves manifest/Cargo producer handling, but several frozen scanner sites still lack complete status semantics, and the new suites do not provide the directed or actual-counter-mutation proof claimed by the scope. The following four finite groups define one coherent Sol attempt2. No framework/helper/workflow, runtime, runner or sibling-scope expansion is authorized.

## 1. Complete the remaining benchmark grep/parser/count sites

The shared escaper's required definition still uses `grep -qE ... 2>/dev/null`; each required timed-subject marker still uses `grep -q`. The frozen scope explicitly requires complete nonquiet positive searches. Preserve the original ERE versus ordinary-grep distinction and includes, remove quiet behavior, and separately diagnose required absence1 versus execution failure with original status/stderr and useful partial output. Do not replace grep grammar with rg output.

The escaper candidate awk is executed as a bare command before `local awk_status=$?`. Under the script's normal set-e, a failing parser exits before that diagnostic branch; stderr has already been redirected into scratch. Capture it through an explicit conditional, as the manifest parsers already do. Preserve the exact original indentation/40-line/comment/backslash/delegate algorithm; a valid empty or delegate result must remain legal only after successful parsing.

Complete status/diagnostic capture for the expected unsafe/environment owner-list sort operations as well as their discovered-list sorts. Several current sort failures report only “sort failed,” losing returned status and operand/output context. The final `printf` still nests unchecked `wc -l | tr -d ' '`; a failed count formatter can be hidden by the successful outer printf. Check the named count/format stages explicitly or use an equivalent checked local representation, preserving the original count and owner sets. Do not treat diagnostic-only diff failure as policy acceptance: it already ends in fail and needs no new framework.

## 2. Preserve complete audit resolution and execution evidence

The audit checker correctly captures one shared manifest population for its structural and resolution loops; this accounts for the two original loops without requiring duplicate enumeration. Together with bench's loop these are exactly the three original #306 debts. Required roots/nonempty manifests, unchanged section exceptions, required package name, and exact `cargo tree --locked --offline -p NAME -e features,no-dev --target all` plus nonempty graph checks are useful accepted changes.

The downstream Cargo-graph grep remains quiet and does not preserve its stderr or partial output in the named execution diagnostic. Run the complete original pattern over the captured graph, distinguish match0/clean1/error, and retain useful stdout/stderr/status. The checker must not erase a matching/error or clean-output/error producer history merely because a downstream result looks decisive. Likewise retain useful captured Cargo stdout when reporting a Cargo failure, not only its stderr; do not mix stderr into graph input. Apply the same operation/status/partial-output evidence rule to the two checkers' remaining find/sort/awk failures.

Keep current ERE/include/dependency/dev/engine/conformance exemptions, depth/root membership, package-name grammar and original diagnostic prefixes. Do not add target resolution, reservation, licensing or new semantic policies.

## 3. Build the frozen selective table on valid reachable fixtures

The new bench tests have only a fabricated partial find failure and indiscriminate sort/awk failures. They discard all diagnostics. There are no directed ownership grep, escaper presence/candidate/delegate parser, private SHA, late timed-subject timer/forbidden, unsafe/environment sets, late manifest reader or count formatter cases. An all-awk shim can fail at the early delegate parser, so it does not prove the named late manifest parser path.

The audit suite's final `cargo-graph-grep-error` case is concretely unreachable: earlier it deletes every manifest in `$copy`, then expect_failure_with_path invokes run_gate on that same `$copy`. It never uses the separately created status-loss-grep repository. The case therefore passes on empty manifest discovery before Cargo or the selected grep executes. Fix the target fixture and demand the intended graph-scan operation/status/sentinel.

Use the two existing suites with their original clean/violation controls. Keep the real offline Cargo baseline and dev/direct-enable cases. For new directed audit faults use the already-required tiny valid manifest fixture and selective named-package Cargo output, not repeated `cp -R "$root"` (which also includes .git/build products and is unnecessary). Bench's corrected roots plus real package manifest are accepted; add a second valid package/subject when needed to reach later calls.

Freeze a small operation table covering the named sites above and all original manifest/discovery/parser/Cargo chains. Each introduced failure-prone stage must have empty/error and otherwise-valid complete/matching/delegate output/error where the successful output class is nonempty. For clean forbidden/dependency results, successful empty output is the honest valid payload. Capture a real tool's complete result or a faithful named-package resolver fixture; unselected calls delegate to real tools. Verify earlier operations succeed, selected operation executes, and its intended returned status and sentinel/partial payload appear. Preserve valid empty forbidden/delegate/violation results; required roots/aggregate/package/Cargo graph remain nonempty. Add each missing-root and required-empty distinction with precise diagnostics rather than arbitrary failure.

## 4. Execute the TWO actual production controls through the SAME assertions

Neither suite mutates the production checker. The two cases labeled “controls” are only failing producer shims. They do not demonstrate that an assertion rejects unsafe acceptance after the production status check is removed. Every new expect_failure_with_path also accepts any nonzero exit and discards the distinguishing output, so setup/syntax/early failures currently count as success.

Implement exactly the two frozen disposable checker mutations: (a) suppress the checked complete manifest-discovery failure, (b) suppress clean-looking failed Cargo graph rejection. Verify each exact selected production mutation is applied once. Run the SAME original targeted error assertion against the mutant on the otherwise-valid fixture; require a distinguished unexpected-success result (e.g.97), separate from wrong-diagnostic/setup failure (e.g.96). A partial manifest filename that names a real valid package is useful, but the full fixture must pass if that selected status alone is lost. The Cargo payload must faithfully name the package selected by the real extractor, not a generic “fixture” unrelated to the requested package. Preserve the original production file and rerun the restored positive. No further mutation campaign is requested.

## Evidence and delivery

Luna's prior syntax/real scanners/suites are candidly reported session executions without dedicated logs. Do not invent logs or infer directed coverage from their exit0. Source inspection is sufficient to establish the above blockers; I did not rerun the defective heavyweight-copy suite merely to reproduce its green label.

After the coherent correction retain exact commands/statuses for both real scanners (Cargo tree is offline metadata only), both affected suites, syntax and diff hygiene, including original/mutant/restored results. No benchmark binary/runner invocation, Cargo build/test, full workspace, artifact or timing is authorized during source correction. Root owns later unchanged-count workspace and actual PR/required CI after source PASS.

The narrow shared population/captured Cargo approach and unchanged parser bodies should be preserved. #453 is now one FAIL; Sol2/3 are available only under the standing workflow, then hard stop/rescope. Parent #403/#306/#349 remain open. Review performed only source/Git/spec inspection and wrote this /tmp report; no tests/builds/timing or repository/GitHub mutations were performed.

Root records one consolidated FAIL and authorizes Sol attempt 2 for the four finite original-contract correction groups above. Preserve accepted manifest/Cargo handling and exact parser/policy semantics; complete all named directed stages and both actual SAME-assertion production controls. No helper/framework/runner/runtime expansion or weaker gate is authorized. Root commits/pushes the coherent correction before further work; one final Sol attempt remains only after a failed verdict, then hard stop/rescope.

## Sol attempt 2 implementation checkpoint

Sol completed the four authorized correction groups in one pass. The benchmark checker now uses
nonquiet captured positive greps, conditionally captures the unchanged delegate parser, checks both
expected and discovered owner sorts, and checks count production and formatting independently. Both
checkers retain status, stderr, and useful partial stdout/input for the original find, grep, sort,
awk, Cargo, and count operations. The audit graph scan keeps Cargo stdout separate from stderr and
distinguishes match, clean absence, and execution failure without changing its pattern, exemptions,
or Cargo-tree flags.

The existing suites now use selective shims over valid fixtures. Unselected operations delegate to
the real tools. The audit directed cases use a frozen tiny offline-resolvable `fixture` package;
only its baseline uses the full real repository. Directed cases cover required roots and empty
aggregate, manifest find/sort, both manifest parsers, unnamed packages, clean and matching failed
Cargo payloads, and matching-output graph-grep failure with precise sentinels. Benchmark cases cover
ownership, escaper presence/candidate/delegate parsing, private SHA, timed marker/forbidden scans,
unsafe/environment scans and both expected/discovered sorts, the late dependency parser, and count
plus formatter failures while retaining the existing clean and violation controls.

The audit suite applies exactly two disposable production mutations once each: suppressed checked
manifest-discovery status and suppressed checked Cargo status. The same targeted assertions return
97 only for the mutant's unexpected success and 96 for setup/wrong-diagnostic failure. The retained
suite output records both mutants at status 97 and the restored tiny fixture at status 0.

Focused evidence is retained in `/tmp/453-*.log` with matching `/tmp/453-*.status` files. The exact
commands use `PATH=/home/bl/.cargo/bin:$PATH`: four-file `bash -n`; both real checkers; both existing
suites; and `git diff --check`. All six statuses are 0. The real audit checker invokes only offline
Cargo metadata resolution. No benchmark binary, runner, build, full-workspace gate, artifact,
timing, Git, GitHub, workflow, Rust/runtime, or sibling-scope action was performed. Root owns the
exact-path checkpoint before adversarial source review.

## Astra attempt 2 verdict and final Sol assignment

# Astra #453 Sol attempt 2 — FAIL

Exact checkpoint `5ab1a298d04d2d0b01f268a0c789ac17ab0cde9e`, `/home/bl/misofm/engine-403-plan`.

FAIL for the remaining frozen directed-evidence contract. Preserve the corrected production status flow and the accepted two actual counter-controls. The final Sol attempt is bounded to the finite omissions below; no new framework/helper/workflow/runner or mutant campaign is needed. A third FAIL requires hard stop/rescope.

## Accepted source and causal evidence

The required escaper/timer searches and final Cargo graph search are now nonquiet. The delegate awk is correctly inside an explicit conditional, preserving the original 40-line/indentation/comment/backslash algorithm. Original grep ERE/include behavior and manifest/dev/engine/conformance exceptions remain. Expected/discovered set sorts and count/format operations now capture nonzero status before success; Cargo output/status and downstream grep are separated. Required roots and nonempty aggregate/name/graph checks remain. The single shared audit population legitimately serves both original audit loops, plus the bench loop: all three original debts remain accounted for.

The tiny audit fixture replaces repeated full-root copies for directed cases, and the formerly unreachable post-Cargo grep case now targets its actual valid fixture. Two actual disposable production mutations suppress manifest-discovery status and Cargo status respectively; each mutation site is checked unique and the SAME original targeted assertion returns97 only on unexpected success, versus96 on wrong diagnostic. `/tmp/453-test-audit.log` records both97 outcomes and a restored positive0. These are genuine accepted controls, not merely producer shims. Preserve them without adding more mutants.

All six supplied `/tmp/453-*.status` files are0 and correspond to syntax, real scanners, suites and diff checks. Those executions do not fill the missing test rows below.

## 1. Finish the finite operation/mode/late-consumer table

The new benchmark table still covers chiefly one output/error shape and only some occurrences. It must not infer later/other named operations from the shared wrapper. Complete the original scope's paired empty/error and otherwise-valid matching/complete/delegate/error modes at these already-named operations:

- Benchmark find and manifest sort; each sole-owner discovery class (allocator implementation, allocator registration, percentile, digest sink), their source-result sorts; required shared definition, escaper candidate discovery/sort and per-candidate delegate parser; forbidden private SHA discovery/sort; timed-subject required marker and forbidden clock/digest at a LATER subject after earlier ones succeed; exact unsafe/environment source scans and expected/discovered sorts; a LATER manifest dependency reader after a previous valid manifest; final count and formatter.
- Audit find/sort, structural parser, package-name parser, Cargo tree and downstream graph grep. Add a second tiny valid package and select its later structural/name/Cargo/grep call, so prior success cannot hide that consumer's failure. Preserve exact Cargo argument/order contract and faithful requested-package output.

Current omissions are concrete: the benchmark sort tests address only initial manifest sort and ordinal8–11, not the intervening sole-owner/candidate/forbidden list sorts; timed-marker and forbidden selectors stop at rack.rs; only one manifest is in the directed fixture. Audit's directed fixture likewise has only one package. Neither table supplies the frozen error-only/valid-output pair at most nonempty-producing operations.

For an operation whose legitimate successful result is empty (forbidden/private dependency scans, for example), empty/error is the valid failed result; do not fabricate a nonempty clean row or duplicate identical empty cases. Retain actual missing-root/aggregate/name cases, but require their intended diagnostic rather than the current broad any-failure checks. Add successful-empty Cargo graph refusal: it is explicitly a required population, not equivalent to failed Cargo output.

## 2. Replace invalid “valid-output” payloads with faithful results

Several current rows prove only violation/error precedence, not the assigned clean-looking status-loss scenario:

- The escaper parser emits `own`, which is intrinsically a private-implementation violation. The frozen case is successful `delegate` (or legitimate empty parse) followed by failure. Use a real accepted delegate file and its actual parser output, and select a later candidate where applicable.
- `manifest-awk-partial` and `structural-partial-sentinel` are nonempty violation strings. Successful clean dependency parsing emits no violation; test empty output/error for that acceptance class. Keep the existing nonempty rows only if labeled as error precedence.
- The unsafe owner shim emits just alloc.rs, while the exact accepted set contains six files; the environment shim emits one of two owners. Losing status still yields a policy mismatch. Emit the full legitimate sets through the real selected grep before failing; retain empty/error as the second required mode.
- `grep-partial-sentinel` is not a matching shared escaper definition or timed marker. Use real successful matched output in those legs. The audit manifest-sort shim emits a fake row rather than the real valid sorted manifest list; use the real complete list before failure.

A small local selective wrapper can capture the saved real tool's stdout/stderr/status, require its legitimate success (0, or1 only for a clean no-match), replay actual output, then emit a distinct injected error/status. Keep original data and sentinel distinct. Nonnamed commands must use saved real tools. No generic fixture framework is necessary.

The new benchmark expect_failure_with_path still permits optional diagnostic checks and uses the same failure status for unexpected success and wrong diagnostic. For new directed rows require operation, tool status and sentinel/output witness, and distinguish unexpected-success97 from setup/wrong-diagnostic96 consistently. Reuse the already-correct audit assertion pattern; retain old legacy predicates without falsely upgrading their evidence. This does not require another production mutant campaign.

## Narrow diagnostic completion

Production sort diagnostics retain input and stderr but omit captured partial sort stdout; expected-set/count diagnostics also omit their captured output. The issue expressly requires useful stdout/stderr when a producer fails. Add those already-captured outputs to their operation/status diagnostic (no parser or success-path change), so the final faithful-output/error rows can assert the actual partial output rather than only the input. This is the remaining original error-evidence requirement, not new semantic policy.

## Final pass and delivery

Retain exact two original/mutant/restored controls. Complete these rows in the two existing suites and the small diagnostic additions, execute both real non-timed scanners and both affected suites, syntax and diff hygiene, and retain their actual commands/statuses/output. Real Cargo tree remains locked/offline metadata resolution; never run a benchmark/build to validate these scans. No full workspace/artifact/CI delivery until source acceptance.

Review inspected the complete issue/prior verdict, exact production/suite delta, current operation lists and supplied logs/status files. No tests/builds/timing or repository/GitHub mutations were performed. Only this /tmp verdict was written. Parent #403/#306/#349 remains open, and Sol attempt3 is the final pass under the standing hard stop.

Root authorizes Sol attempt 3 for these finite table/payload/late-consumer and diagnostic-output gaps, preserving accepted production flow and both actual counter-controls. This is the final coherent pass in the series; a FAIL requires preserving evidence, hard stop and explicit bounded rescope. No fourth retry, new mutant campaign, helper/framework/runner expansion or weaker acceptance is authorized. Root owns exact-path checkpoints and remote synchronization.

## Sol attempt 3 implementation checkpoint

The final Sol pass preserves the accepted production flow and the two actual status-loss mutants.
The only production adjustment adds already-captured partial stdout to sort, expected-set, count, and
formatter failure diagnostics. No success-path, parser, pattern, owner set, exemption, Cargo flag,
or policy semantics changed.

The benchmark suite now uses two valid manifests so the dependency parser failure is selected at a
later manifest. Its selective rows cover empty/error and faithful real-output/error modes for
manifest find/sort; all four sole-owner discoveries and their individual sorts; shared-definition
and candidate discovery/sort; the accepted later delegate parser; private-SHA discovery/sort; a
later timed subject's real marker and clean forbidden scan; exact six-file unsafe and two-file
environment scans plus both expected/discovered sorts; and real count/formatter output. Directed
assertions require the named operation/status and injected sentinel, returning 97 for unexpected
success and 96 for a wrong diagnostic or missing witness.

The audit fixture now contains ordered `fixture` and `later-fixture` packages. Selective structural,
name, Cargo, and graph-grep failures target `later-fixture` after the earlier package succeeds.
Find/sort rows cover both complete real lists and empty output on error; parsers cover legitimate
empty structural output and both valid/empty package-name output on error. Cargo rows retain the
exact requested package name for clean and matching output/error, reject successful empty output,
and graph-grep rows use the real grep over matching or clean Cargo payloads. Missing roots, empty
aggregate, and unnamed later package assertions require their precise diagnostics. The uniquely
verified manifest and Cargo production mutants still produce distinguished status 97, while the
restored two-package fixture produces status 0.

Final focused evidence is retained in `/tmp/453-*.log` with corresponding status files for the
four-file syntax check, both real scanners, both suites, and diff hygiene. The audit scanner uses
only locked offline Cargo-tree metadata resolution. No benchmark binary, runner, build,
full-workspace gate, timing, artifact, Git, GitHub, workflow, runtime, or sibling-scope action was
performed. This is the final implementation attempt; a failed adversarial verdict requires hard
stop and bounded rescope.


## Final Sol attempt 3: hard stop

# Astra #453 final Sol attempt 3 — FAIL / hard stop

Exact clean reviewed head `ad489984716d2ce2a049bd42b5b764e3de15896a`, `/home/bl/misofm/engine-403-plan`. ONE consolidated final verdict. Preserve this checkpoint and all evidence; no fourth repair or full qualification is authorized. The remaining defect is a finite test-table omission, not a production-policy failure. Rebrief/number a narrowly bounded completion before further implementation.

## Accepted and preserved

The two production checkers retain the previously accepted status/semantic behavior. Final source changes add captured stdout to the named sort/count diagnostics without changing parser grammar, grep ERE/includes, the delegate40-line/indentation/comment/backslash logic, owners/exemptions, Cargo flags or success paths. All three original loops are accounted for: bench manifest loop and both audit loops over the one checked shared population. No helper, workflow, runtime, runner or unrelated gate changes.

The final suite now supplies the missing sole-owner/candidate/private-SHA sort sites; complete actual unsafe/environment owner output; second manifests/packages; later subject/dependency/structural/name/Cargo/grep targeting; faithful delegate output; required roots/empty aggregate/name diagnostics; and successful-empty Cargo refusal. Exact two actual production mutants remain uniquely located, run through the targeted error assertion, and distinguish97 unexpected success from96 diagnostic/setup failure, with restored fixture0. Do not expand that already-accepted representative mutation campaign.

All six retained `/tmp/453-{syntax,check-bench,check-audit,test-bench,test-audit,diff-check}.status` files are0. The audit log records both actual97 mutants and restored0. These are useful executed gates, but passing them does not supply rows absent from the suite.

## Sole remaining finite blocker: four absent error-only rows

The binding attempt2 revision explicitly froze paired empty/error and otherwise-valid output/error modes at the nonempty-producing operations, including the delegate parser, final count/formatter and Cargo tree. Current final source has only the nonempty leg at these four locations:

1. `test-bench-policy.sh:321`, `delegate-parser-output-error`: selected accepted later `effect_interchange.rs` awk emits actual `delegate` then exits6. There is no corresponding same selected delegate parser producing no stdout then exit6. The empty structural dependency parser is a different operation and does not cover this site.
2. `test-bench-policy.sh:376`, `count-error`: wc emits actual6 then exits9. There is no same wc failure with empty stdout.
3. `test-bench-policy.sh:382`, `count-formatter-error`: tr emits actual6 then exits10. There is no same formatter failure with empty stdout, after the successful count still supplied input6.
4. `test-realtime-audit-leak.sh:165`, `status-loss-cargo`: the later package emits clean named graph then exits8; the matching graph/error row also emits output. `cargo-empty-success` exits0 and tests a DIFFERENT required-population branch. There is no later Cargo process returning failure8 with no stdout after the earlier package resolves successfully.

These are exactly the missing modes from the prior finite table, not newly requested predicates, extra packages or a broader matrix. No additional mutation or framework is needed. Production behavior appears correct for them; this verdict does not assert a discovered failure in those implementations.

## Smallest bounded successor recommendation

One evidence-only child: complete the four remaining execution-error rows for the accepted benchmark/dependency scanners. Allowed changes: the two existing suites and child/parent evidence only. Freeze current production checker bytes and all accepted tests/controls. Add each empty/error leg beside its existing real-output/error leg, at the SAME selected operation. Require original production refusal with the exact operation, tool status, `<empty>` stdout and distinct injected stderr sentinel. For the tr row also retain input6; for the later Cargo row retain the two valid packages and real earlier package success. Reuse existing97/96 assertions; do not add a third production mutant or change either existing mutant.

Run syntax, both affected suites and both existing non-timed real checkers plus diff hygiene, retaining logs/statuses. Existing two mutant97/restored0 controls must remain green. Parent#453 retains the full original outcome and subsequent workspace/actual-PR/CI delivery; #403/#306/#349 remain open. Number and synchronize this child before assigning a fresh bounded Luna pass. The four-mode completion must not be made informally on this stopped series.

Review used read-only source/Git/log inspection; no tests, builds, Cargo resolution, timing or repository/GitHub mutation were performed. No other finite blocker is being reserved for an unbounded follow-up review.


## Numbered completion child #462

#462 owns exactly the four missing empty-output/execution-error test rows after the hard stop above. Its scope freezes accepted production and both genuine mutation controls. #453 retains the complete original scanner outcome and all workspace/PR/CI delivery obligations; neither issue is complete until that proof and remote delivery are accepted. Other #403 children remain queued.


## Completed source and workspace qualification

Astra accepted amended #462 Sol attempt2 at8d4520bd, completing the four missing rows and portable Cargo delegation while preserving #453's scanner semantics and existing controls. The immutable full-workspace candidate2c8e0c48646192ae1484e56356ff2a26279a403e completed `cargo test --locked --workspace` with exit0:275result blocks,1576passed,0failed,24ignored including doctests, identical to the unchanged Rust baseline. No runtime/fixture/configuration inputs changed against delivered29a8c88b. Evidence, terminal status, source identity, relocation proof and prior verdicts are retained under `artifacts/issue453-dependency-discovery/` with byte/hash manifest. Actual PR exact-head Astra review and required CI remain the final gates before #453/#462 closure; #403/#306/#349 remain open for other scopes.
