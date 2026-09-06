# Make realtime numeric and contract gates reject scan failures

Parent #306; depends on foundation #400 and CLOSED/upstream #371. The complete standing contract is embedded below. Outcome: six numeric/realtime/contract gates preserve their existing rules and fail when their actual scan cannot run.

Exact gate roster:
- scripts/check-realtime-policy.sh
- scripts/check-lane-policy.sh
- scripts/check-effect-state-migration-v1.sh
- scripts/check-env-vocabulary.sh
- scripts/check-dsp-research.sh
- scripts/check-unfused-seal.sh

For realtime retain #371's actual merged discovery and 12-file/42-region floors (or independently landed higher counts). Propagate discovery rg errors before entering the loop, and source-search errors before unsafe allowlist filtering. Preserve execute_op, EffectControlLane::stage, host-root, new-file, removal-floor and unsafe-owner mutations. This closes the error-handling debt #371 explicitly deferred, not a request to broaden its runtime marker roster or malformed-marker grammar.

Lane and unfused gates preserve all exact absence/presence/count/target rules and exemptions. The environment gate is a bidirectional table/set comparator with no current rg sites; source common applicable helpers but do not invent an rg migration. Preserve fail behavior and verify its actual input-read failure path. Effect-state migration and research documentation gates preserve their existing domains/fixtures, no new claims or prose hashes.

Allowed support: minimal shared-library additions with tests; affected existing suites test-realtime-policy.sh, test-lane-policy.sh, test-effect-runtime-policy.sh (covers state migration), test-env-vocabulary.sh; small hermetic suites for dsp-research/unfused only if no existing exact checker coverage is found. No generic mutation framework. Freeze call-site/expected-empty inventory on merged main before coding; missing required root tests must reach the intended operation, not only arbitrary earlier refusals.

Acceptance: real six gates pass; all old mutation cases remain green; missing-root/read error and discovery failure rejected through the appropriate checked path in each gate; helper negative self-checks discriminate unexpected acceptance and execution errors; no floor lowering/allowlist broadening. Full workspace unchanged-count comparison and required CI. No Rust/DSP/codegen changes, benchmark or artifact repin. Root checkpoints, Astra final PR review, Luna one attempt then at most two Sol revisions. Parent stays OPEN.

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

## Astra bounded-child readiness ruling

The six-gate implementation is split before coding into three stateless children, retaining every original gate and producer obligation. #410 owns realtime/lane traversal and follows merged #417 and then #423, preserving the completed #406/#407 helper work and serializing Session changes before the next helper tranche. #411 owns the unfused seal and follows merged #410. #412 owns environment vocabulary, effect-state migration and DSP research input scans and follows merged #411 (and #406). This serialized order prevents overlapping shared-helper and effect-runtime-suite changes.

Astra revalidated all three briefs against main a9e801fea91dc49a4d2acc9bea939d3fdc38dec9. Existing required CI covers #410/#411; #412 may add only its focused research suite beside the existing research checker. Each child retains unchanged-count workspace qualification, existing policy semantics and directed execution-failure tests. This parent remains OPEN until all three children are merged, reviewed PASS and remotely synchronized. Publishing queued briefs authorizes no implementation ahead of dependencies.

## Updated queue after compiler gate delivery

#407 is merged and CLOSED via PR #421. Astra revalidated #410 against main `a0e4d123a038160b4f5934dac14aacc72c9fbbf2`; its complete current brief preserves exact producer policies, directed failure cases and existing CI wiring. #410 remains queued until active #417 and the remaining graph extractor completion #423 merge. #411 and #412 keep their original serial dependencies and complete assigned scope. This scheduling update changes no closure obligation or implementation gate.

## Delivered-child reconciliation for parent closure review

Current review base is delivered main `5a4a7d2071194cf6118241e24d073824668e3387`. On 2026-09-05 root verified all five child/successor issues below are remotely CLOSED. This is a bounded parent completion review, not authorization for another implementation tranche or a claim that parent #306 is complete.

| Original gate obligation | Delivered issue(s) | Merged PR and commit |
| --- | --- | --- |
| Realtime and lane complete traversal | #410 | #433, `1af76181490a623675960c244a6c677c06aae745` |
| Unfused seal complete searches and recount | #411 and proof successor #438 | #440, `e7e1a37f36fe8a22c237d0bfcd3737373c6d4deb` |
| Environment vocabulary, effect-state migration, DSP research reads/discovery | #412 and proof successor #448 | #449, `39da065507beb822ef70a1552ff5dcc363938dd4` |

Astra must verify the complete six-gate parent obligations against current source, the child decision records, retained actual failure controls and delivered qualification evidence. Closed child state alone is insufficient. Preserve the candid three-attempt histories and bounded successor proofs. Check that subsequent main changes have not invalidated those claims. Any actual omission remains open and receives a bounded correction scope; do not weaken the parent's original contract.

No runtime, checker, fixture, helper or workflow change is proposed in this reconciliation. Existing delivered workspace and CI evidence can substantiate unchanged code; no benchmark, artifact/browser rebuild or redundant broad test campaign is authorized merely for this parent record. Actual PR Astra approval and required CI still apply to a delivery commit. Parent #306, #403, #404 and broad #349 remain OPEN regardless of this parent's outcome.

## Astra parent completion review

# Astra #402 six-gate parent closure review — PASS

Exact reviewed checkpoint `317af392a393c365891476b6f5d40441cbe99de9`, `/home/bl/misofm/engine-402-closure`, on delivered main `5a4a7d2071194cf6118241e24d073824668e3387`. HEAD is exact and clean. Its entire delta from delivered main is the 14-line parent reconciliation record.

PASS for a docs-only #402 closure-evidence PR. No unassigned original six-gate omission was found. This acceptance rests on unchanged delivered source, discriminating child controls and completed qualification, not child closed states alone. The eventual closure PR still requires actual-head review and required CI; this review makes no repository or remote state change.

| Parent gate | Source and proof traceability |
| --- | --- |
| check-realtime-policy.sh | #410 preserves the original unsafe-owner regex/filter order and 12-file/42-region floors. Checked filename discovery/sort, independent BEGIN/END reads, body extraction/persistence and final predicate distinguish execution failure from permitted empty results. Existing execute_op, EffectControlLane::stage, host/tools/new-file/removal-floor/unsafe-owner mutations remain in the suite. Accepted selective partial/error-only cases and actual same-assertion controls cover unsafe source, failed discovery, per-file read and final predicate. |
| check-lane-policy.sh | #410 owns both original lane find-backed loops. All required roots, nonempty lane-source/workspace-name aggregate, package/lock grammar, exact pins, marker window and dependency/exemption rules remain. Individual no-name manifests and empty dependency lists stay legal. Actual controls cover failed find, nonempty failed version, allowed failed dependency output and failed non-wide membership, using the original targeted assertion's unexpected-pass diagnostic. |
| check-unfused-seal.sh | #411 production status/caller/grammar fixes plus #438 complete the retained contract: actual registry7+1=8, marker placement/window, required roots, legitimate empty results, complete source discovery and count/retired searches. The 62-case suite remains. #438's uniquely identified actual count_calls status mutation accepts only the injected later registered occurrence status9 while preserving real mul_add( stdout and ordinary wc-derived1; original exits9, mutant checker0, SAME assertion97. No registry answer fallback is credited. Discovery and retired controls remain. |
| check-env-vocabulary.sh | #412 implements actual grep/Git/find/tr/sed/sort/comm/count reads, not an invented rg migration. It preserves vocabulary set/grammar rules, required tools/scripts roots and nonempty used/documented sets. Git failure is distinguished from narrowly recognized genuine nonrepository fallback. #448 completes missing-scripts with otherwise-valid metadata and original Git-listing assertions in both empty/error and real complete NUL-output/error modes. Actual listing-status mutation reaches SAME unexpected-success86; late-comparison control remains. |
| check-effect-state-migration-v1.sh | #412 completes checked required documentation/API and forbidden runtime/descriptor/serialization scans through the already-qualified shared helper. Required files/roots and existing migration/runtime mutations remain. Late serialization error is targeted after valid earlier inputs; the unique actual failure-propagation mutation must reach the same assertion's named unexpected-success outcome, not arbitrary nonzero. |
| check-dsp-research.sh | #412 preserves the existing ten-note/headings/support-file/source-key/console/listening domain. Heading/content, primary-section extraction, key extraction/conversion/sort, bibliography and final consumers all have checked execution paths. The finite synthetic fixture has valid source identity/sections; missing-note/support cases and selective producer/late-consumer cases discriminate the intended operation. Actual whole-note producer and final listening consumer mutations run the same focused assertions and require named unexpected-success86. No new research claim or prose hash gate is introduced. |

The parent-wide helper rules remain supported by the delivered helper and its existing negative/counter tests: complete checked producer results before consumers, three search outcomes, filtering only after successful scans, physical script-owned helper resolution and no helper changes to caller options/traps/cwd. Environment remains an explicit local non-rg pipeline, as its approved child inventory requires; sourcing an unused helper is not an additional closure condition. Gate predicates and permitted empty cases retain their original distinctions. Required-root deletion cases are supplemented by selective otherwise-valid producer/read failures rather than unrelated setup refusals.

Verified with read-only Git comparisons that realtime/lane checkers and suites have no delta from their #410 merge, unfused has no delta from its #411/#438 merge, and the three #412 checkers, affected suites and shared helper directory have no delta from their #412/#448 merge. Thus later feature and tooling delivery has not silently altered these accepted scan implementations. Current source inspection agrees with the retained control claims; no new runtime marker grammar, relaxed root set, widened allowlist or lowered floor appears in this reconciliation.

Reviewed #410's complete source/actual-PR records and the retained #438/#448 source/control records, original failed histories and relevant logs. Independently verified size/SHA-256 for all14 #438 manifest evidence entries and all36 #448 entries: no mismatches. The retained #438 status proof contains actual producer output and counts; the suites still execute actual checker mutations through the same targeted assertions. Historical failed attempts remain failures; each bounded successor resolves its named remaining proof rather than relabeling a stopped series.

Delivered qualification trace:

- #410 / PR433: baseline4557865e and candidatee63e142c locked workspace including doctests each terminated0 with274 result blocks /1566 passed /0 failed /24 ignored; actual-PR Astra PASS at973d3670. Both affected suites were independently executed at source review and real checkers passed.
- #411+#438 / PR440: immutable67ac8993 workspace terminated0 with274/1569/0/24 matching its independently delivered RT-3 baseline. Actual source review executed the62-case suite and reconstructed the registered status-loss proof; actual-PR acceptance retained.
- #412+#448 / PR449: immutable26913f2b complete integrated run includes all three real checkers, three suites and helper suite exit0. Workspace including doctests275/1575/0/24 matches delivered435. Earlier watchdog-split logs remain historical and are not substituted for the completed integrated qualification.

Read-only GitHub verification now confirms PR433 MERGED at1af76181490a623675960c244a6c677c06aae745, PR440 MERGED ate7e1a37f36fe8a22c237d0bfcd3737373c6d4deb and PR449 MERGED at39da065507beb822ef70a1552ff5dcc363938dd4; each actual PR reports completed required qualification SUCCESS. Issues410/411/438/412/448 are all CLOSED. The parent's table matches those identities.

The differing workspace counts across child deliveries reflect intervening independently delivered runtime features, not shell-induced test-population changes; each child used its own unchanged-count baseline. No new workspace, artifact/browser regeneration or timing is required merely to repeat this docs-only reconciliation. Actual closure-PR CI remains separate.

#402 owns these six gates and the explicitly assigned lane loops. No additional original site outside that roster was discovered in this bounded review. The remaining program-level21-gate/five-extractor/nine-loop accounting stays with #306 and its still-open #403/#404 and other retained obligations; #402 closure must not claim completion of them or broad #349. No new mandatory gate, successor or framework is requested.

No implementation/spec edits, Git/GitHub mutations, builds, tests, benchmark or artifact publication were performed. Only this /tmp review was written.
