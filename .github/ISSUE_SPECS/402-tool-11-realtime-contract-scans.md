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
