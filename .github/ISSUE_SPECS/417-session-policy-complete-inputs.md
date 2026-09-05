# Make Session format and compile-order policy reject incomplete inputs

Parent #401; grandparents #306/#349 TOOL-11. Depends on merged #407, which serializes shared-helper and workflow edits. This is the pre-implementation Session split Astra required under #407's half-day rule; no original obligation is removed. #401 closes only after #406, amended #407 and this child all land.

## Closable outcome and allowed paths

The existing Session format and compile-order policy rejects incomplete inputs without changing its format, order or allowlists. Only scripts/check-session-policy.sh, new scripts/test-session-policy.sh, minimal scripts/lib/gate.sh and scripts/test-gate-lib.sh changes if still necessary, this numbered spec/evidence, and .github/workflows/qualification.yml solely to add its new test immediately after the existing Session checker. Preserve job/router/triggers/expectations and all other steps. No runtime, manifest, artifact, benchmark or generic harness.

No CLI expansion: copy the checker/helper into a disposable fixture repository and preserve physical-script-root behavior.

## Frozen semantics

Preserve the physical script-root behavior and required allowlist/session manifest/source. Check every direct negative and positive rg operation separately (engine reverse edge, session engine/json-syntax presence, parser baggage, publication APIs, allocation vocabulary). Each of the five compile-order anchors must have at least one match from a successful complete scan; use its first numeric line as before, then enforce the existing strict order. Multiple matches retain the existing first-match behavior. Capture the successful scan before selecting its first result; missing/error values must not coerce to zero.

The grouped producer at lines 38–43 contains four independent `find` invocations over six required repository populations: `fixtures/session`, `fixtures/native-pcm-runner`, `hosts/host-web/qualification`, `hosts/host-web/tests/browser-v1`, `sdk`, and `fuzz`. Capture every producer and sort status before looping. Zero TOML matches after a complete traversal is valid because the policy forbids non-allowlisted live Session TOML. Preserve historical allowlist parsing and exact patterns. The retired-spelling rg producer at line 51 may validly filter to empty; its search, glob exclusions, self exclusions, and allowlist handling must complete successfully first.

## Directed acceptance

The focused hermetic suite needs a clean positive control, existing violations, all six required roots, each of five ordering anchors, duplicate-anchor first-match positives, expected-empty TOML and retired-spelling results, allowlist read failures, and find/sort/read/rg failures AFTER plausible valid output. Each intended operation must be reached with other metadata intact; missing/error line values never become zero. The four find producers and all filters/consumers must have checked completion.

The new suite itself is scanned by the retired-spelling rule. Construct forbidden fixture words from separated shell fragments so the committed suite does not self-match. Do not broaden exemptions, glob exclusions or historical allowlists. Keep the current scan domain.

Every assertion must explicitly reject unexpected success and match the intended error class. Counter-mutations must run the actual acceptance assertions against a faulty implementation and show rejection; constructing a bad control alone is insufficient. Preserve shared helper/parser defaults, caller shell state, diagnostics, exact patterns and allowed emptiness.

## Gates and delivery

Real Session checker, complete focused suite, helper tests if changed, syntax/diff and required CI; retain the full workspace unchanged-count comparison at the coherent delivery boundary. No Cargo build or timing is needed for the implementation pass. Astra scopes and adversarially reviews; Luna attempt 1, Sol only after FAIL, three total maximum then hard stop/rescope. Root owns isolated checkout, exact-path checkpoints/pushes, local/remote synchronization and actual PR merge after Astra PASS and required CI. Publishing a queued brief does not authorize implementation before dependencies. Parent #401 and the broader #306 program remain OPEN until their full assigned obligations are upstream.

Assigned number: #417. Parent closure is #406 + #407 + #417; this issue remains queued until #407 merges.
