# Make Session format and compile-order policy reject incomplete inputs

This is the complete amended #417 brief, ready to synchronize to its existing numbered issue before assignment. Parent #401; grandparents #306/#349 TOOL-11. Depends on merged #407 and its final helper API. Root freezes the actual merge base before Luna begins. #401 closes only after #406, #407 and #417 all land; all original obligations remain.

## Smallest closable outcome and paths

The existing Session format/ordering checker rejects every failed or incomplete input operation while preserving its existing policy, first-match order, exact historical exemptions and valid-empty populations. One checker, one small direct fixture suite and one existing CI step are the whole outcome.

Allowed: `scripts/check-session-policy.sh`, new `scripts/test-session-policy.sh`, this numbered spec/evidence, and `.github/workflows/qualification.yml` only to convert the existing Session checker run into a block that runs that same checker then the new suite immediately afterward. No job/router/trigger/expectation changes. `scripts/lib/gate.sh` and `scripts/test-gate-lib.sh` may change only if a concrete necessary capability is absent after #407; the existing forbidden/required/collect/find/sort helpers should suffice. Prefer a small local anchor function to another shared parser. No runtime, manifests, artifact, benchmark, other gate, generic command framework or historical allowlist edit.

Preserve the physical script root derived from `$0`; source its shared library using that root. Do not add a fixture-root argument or change the CLI. Tests execute copies of checker/helper in a disposable repository, including from a foreign working directory.

## Exact policy and input contract

All current violation messages retain the `session policy:` prefix and their existing text. Producer errors add operation-specific context and actual tool status; clean required absence must be distinguishable from inability to search. Capture success/error status explicitly before parsing, filtering or counting. No `! rg`/unchecked conditional/process-substitution pipeline may conflate absence and failure.

Six direct predicates, each independently checked:

1. Engine manifest must not contain the exact reverse dependency row selected by `^session\.workspace = true$`.
2. Session manifest must contain the exact full line `engine.workspace = true`.
3. Session manifest must contain the exact full line `json-syntax = { version = "=0.12.5", default-features = false }`.
4. Its existing beginning-of-line whitespace-aware `toml`/`serde` dependency ban remains unchanged.
5. The existing publication-API pattern over all `crates/session/src` remains unchanged.
6. The existing allocation-vocabulary pattern over `crates/session/src/estimate.rs` remains unchanged.

For rules 2/3, the old `rg -x` full-line behavior must survive helper substitution: add equivalent anchors when using a helper without `-x`. Do not accidentally permit extra leading/trailing material. Freeze all six current regexes and roots from the unchanged checker when recording the assignment; no broadening/grouping that hides a producer is authorized.

`crates/session/src/compile.rs` supplies five separate required scans, in this exact order:

- `let estimate = estimate_session\(session\)`
- `check_caps\(session, estimate, caps\)`
- `validate_session\(session\)`
- `let canonical_json = write_canonical\(session\)`
- `let mut normalized = session\.clone\(\)`

Scan one explicit file, capture the COMPLETE successful `rg -n` output, then select the first row and its leading line field. Require a positive ASCII-decimal line number before arithmetic; interpret it as decimal, never shell expression text or an absent value coerced to zero. Retain strict `estimate < caps < validate < canonical < clone`. Later duplicates remain valid if the first occurrences satisfy the order. Do not impose uniqueness, use early `head`, or treat a failed scan's plausible first row as an anchor.

The exact allowlist `scripts/session-policy-historical-allowlist.txt` is required. Apply the existing two sed expressions removing whitespace-prefixed comments and blank lines, with explicit read/parser status. Empty/comment-only content is valid. No whitespace normalization of surviving entries: equality remains exact. A failed read after plausible output cannot yield a usable partial allowlist. Diagnostics need the operation and sed status, not an echo of partial allowlist contents. Tests prove the shim produced output and the checker rejects its status; preserved partial stdout is not required here.

Capture each of these four find invocations separately, retaining its arguments:

```
find fixtures/session -type f -name '*.toml'
find fixtures/native-pcm-runner -type f -name '*.toml'
find hosts/host-web/qualification hosts/host-web/tests/browser-v1 -type f -name '*.toml'
find sdk fuzz -type f \( -name '*.session.toml' -o -path '*/session_*/*.toml' \)
```

All six roots are required. Each successful invocation may return zero paths. All four must finish successfully before any path is considered for exemption or violation. Append only nonempty captures, then perform one checked sort; preserve duplicates as the original sort does. Empty aggregate is valid and must not manufacture a blank violation. Remove suppressed find stderr so errors remain observable. Keep exact path comparisons for TOML findings.

The final retired-name search keeps its exact existing regex, root `.`, and exclusions for target, .git, sdk/node_modules, the checker itself and the SDK-deletion checker. Capture the complete search and reject execution errors before any allowlist filtering. Successful empty output is valid. Parse path/line/rest with shell builtins, remove a leading `./` from candidate paths exactly as before, and apply the same exact historical entries. Do not expand exclusions or normalize allowlist entries. Shell-only path parsing/is_historical needs no external helper.

## Minimal direct fixture and objective cases

Use one small base fixture copied per mutation: checker/helper; normally comment-only allowlist; minimal engine/Session manifests with the exact required rows; clean estimate source; a synthetic compile source with the five ordered anchors; and all six discovery roots. It need not compile. Its default has zero TOML matches, zero retired matches and an empty historical list, and MUST pass. Add one valid allowlisted-path positive. Do not copy the entire repository or run Cargo.

A bounded table/loop of the following operation cases is the acceptance contract. Each error shim delegates unrelated calls to the real tool, so earlier checks stay valid. Assert intended operation/class plus nonzero result and explicitly reject unexpected success. Do not use a generic first failing rg shim as evidence for a later operation.

- All six predicates: one true policy violation each; distinct search-error injection selected by the actual pattern/input for each, with useful partial output before failure. Test required exact-line semantics against a line containing the required text plus extra material. Remove engine manifest, Session manifest, estimate source and compile source in otherwise-valid cases to prove required inputs.
- All five anchors: remove each in a loop; inject each search's error after a plausible valid `line:text` row; reject malformed/zero line text. One adjacent-order swap is sufficient for the common strict-order comparison. Duplicate all five anchors later in the file as one positive preserving their first occurrences. Add a duplicate of the last anchor before the first anchor and require the existing ordering failure; this discriminates first-match selection from last-match selection without inventing duplicate rejection.
- Allowlist: missing-file red, comment-only positive, checked sed failure with no output and after plausible allowlisted rows. Use controlled read failure rather than privileged chmod-only tests.
- Four find calls: remove each of the six roots in a loop, preserving other metadata. Inject error-only and useful partial output then error at EACH distinct invocation. Earlier find calls must delegate successfully. Include a non-allowlisted TOML finding from each of the three pattern shapes (plain .toml, .session.toml, nested session-directory .toml); the two ordinary roots share the same pattern and need no redundant corpus. Include exact allowlisted TOML positive.
- Sort: a two-path ordering fixture checks the first reported violation follows existing sorted order; a shim emitting sorted valid-looking paths then failing must report sort error before the loop. Empty combined discovery remains positive.
- Retired search: clean empty, actual forbidden spelling, exact allowlisted-path positive, error-only, and allowed-looking or violating partial stdout then error. Confirm original self/SDK-deletion exclusions using the disposable fixture. Keep scope unchanged.

The new suite itself is in the real checker’s scan domain: construct forbidden retired fixture words from separated shell fragments. Do not put whole forbidden tokens in committed comments, labels, heredocs or mutation names. Do not alter the historical allowlist/exclusions to hide the suite. Temporary tool shims and captured outputs containing these words must remain outside the scanned fixture except the deliberately targeted case.

## Counter-checks and delivery

Reuse the SAME operation assertions to counter-test disposable faulty checker/helper variants: ignored anchor failure, ignored find failure, ignored sort failure, ignored allowlist-read failure and ignored retired-search failure. Each must be rejected at its intended assertion. The anchor mutant must still reject ordinary missing anchors while accepting only execution errors, proving the partial-error case actually matters. Record the real assertion/status; building a mutant or printing that it was rejected is not evidence. Keep this bounded inside the new suite or record an independently executed disposable acceptance run, not a new harness framework.

Once one coherent implementation pass is ready: real checker, full focused suite, shared helper tests only if changed, Bash syntax/diff. Root checkpoints/pushes before more edits; Astra gives one adversarial verdict. Full workspace unchanged-count comparison remains a delivery-boundary gate, followed by actual PR review and required CI. Luna attempt 1; Sol only after Astra FAIL, at most two revisions; after three failed attempts hard stop/rescope. No timing or benchmark tooling belongs here.

This brief resolves Sol's readiness questions without changing the original Session policy. #417 remains queued until #407 merges; root must synchronize this amended existing issue/spec and freeze the actual base before assignment.

## Frozen implementation assignment

#407 is merged and verified CLOSED at `a0e4d123a038160b4f5934dac14aacc72c9fbbf2`; this is the fresh implementation base for #417. Astra's final brief above is synchronized before Luna attempt 1. The six direct predicates, all five anchor patterns, allowlist parsing, four find argument sets and final retired-name search are frozen to `scripts/check-session-policy.sh` at that base. Root owns exact-path checkpoints, upstream synchronization, workspace comparison, final PR and merge. No current benchmark quiet window is active.

## Luna attempt 1 evidence

The frozen checker and focused disposable fixture suite are implemented. `bash scripts/check-session-policy.sh` passes against the repository, and `bash scripts/test-session-policy.sh` passes direct predicate violations, required-input removals, five anchor removals, strict order and duplicate anchors, six-root discovery, allowlisted TOML, retired spelling, and selective nonzero producer shims. The checker retains the physical `$0` root, exact search/find patterns and exclusions, explicit producer statuses, complete first-match anchor output, positive ASCII-decimal validation, one checked sort, and clean-empty handling. The existing workflow step runs the checker followed immediately by the focused suite.

## Astra attempt 1 verdict — FAIL; Sol attempt 2

# Astra #417 attempt 1 review

**FAIL at exact pushed `18ba7c3d09e3494fa9f21a41619971cbd347c3cf`.** Luna's first attempt is consumed. Assign one bounded Sol evidence revision; no workspace/PR promotion yet.

## Source worth preserving

The checker now explicitly checks complete producer results before interpretation. Both required manifest rows are anchored, retaining old full-line behavior and original policy messages. Anchor scans capture complete numbered output, select the first occurrence and reject absent/nonpositive/nondecimal line fields before strict ordering. The allowlist sed status is explicitly checked, including partial-output failure; clean comment-only input remains valid. All four find producers complete before the checked sort and path loop, with empty captures omitted. Retired-name search checks its status before historical exclusions. Existing roots, patterns, allowlist equality, physical $0 root and CLI are preserved. The CI change is exactly checker then focused suite in the existing step. No concrete source-policy regression was identified in this read-only pass; do not rewrite the checker merely to make tests easier.

## The suite does not yet prove its frozen claims

`check()` redirects BOTH output streams to /dev/null, and `red()` accepts any nonzero result. Every producer test therefore accepts unrelated syntax, earlier policy or missing-tool failure. This is explicitly excluded by the complete brief.

The four find shims emit `fixtures/session/partial.toml` then exit 9. That path is not allowlisted. Even a checker that ignores the find failure will reject the emitted path as live TOML, so these tests cannot distinguish error propagation from ordinary violation detection. The anchor shims similarly emit `1:partial` for every target: for caps/validate/canonical/clone this breaks ordinary ordering even if the execution failure is swallowed. A plausible row must carry that anchor's actual otherwise-valid first line.

No counter-mutant is implemented or separately recorded: there are no faulty checker/helper variants, intended assertion results or five required counter-check statuses. The green suite log is consequently not evidence for those gates.

## Bounded Sol attempt 2: complete the existing suite

Keep this to the existing four paths, preferably suite plus candid spec evidence only. Use one small diagnostic-preserving runner/assertion pair with explicit unexpected-success rejection and expected operation/class checks. Selective shims must delegate unrelated calls to real tools; preserve a sentinel/status showing that the intended producer was reached. Keep shim output with retired words outside the scanned fixture except deliberate policy cases. No generic harness or shared-helper change is needed.

| Required surface | Exact remaining completion |
|---|---|
| Six direct predicates | Add a distinct actual json-syntax pin violation (currently absent), plus selective partial-output/error injection for ALL six predicates (currently none). Preserve valid preceding metadata. Both required rows need exact-line positives/extra-material negatives; test json pin separately from a missing engine row. Capture the original violation message or operation/error class. |
| Five anchors | Keep removal loop and later-duplicate positive. Use the correct actual first `line:text` per anchor for partial-error tests, not constant line 1. Add malformed and zero line fields; add the specified early duplicate of clone before estimate and assert ordering failure. Keep an order inversion with the intended ordering diagnostic. |
| Allowlist | Add missing-file failure and error-only sed failure; retain comment-only positive and useful-partial sed failure. Assert operation plus sed status, and separately prove the shim emitted its partial row without requiring the checker to echo that row. |
| Four find producers | Keep six-root removal loop with traversal-specific diagnostics. For EACH invocation test error-only and useful allowed-looking partial paths then failure, using a real exact allowlist entry so ignored status would otherwise pass. Earlier invocations must succeed. Add .session.toml and nested session-directory .toml violations alongside the existing plain .toml case. |
| Sort | Add two paths whose reported first violation proves sorting; add sorted otherwise-allowlisted partial paths then sort error and require the sort diagnostic before path processing. Preserve the empty discovery positive. |
| Retired-name search | Keep real forbidden/clean-empty tests; add exact historically allowlisted retired-path positive, error-only and useful partial-output failure. Verify the existing checker/SDK-deletion exclusions without broadening them. Select this final query only, allowing all earlier producers to succeed. |

Retain required manifest/source removals, foreign-working-directory execution and existing policy violations, but stop discarding their diagnostics. Do not add Cargo fixtures or normalize allowlist strings. Construct banned fixture spellings from separated fragments as the current suite already does.

Execute the five frozen disposable counter-mutants THROUGH THE SAME assertions: ignored anchor failure, ignored find failure, ignored sort failure, ignored allowlist-read failure, ignored retired-search failure. Anchor mutation must preserve missing-anchor rejection and differ only on execution failure with usable output. Find/sort/retired counter inputs must otherwise pass their actual policy, so a lost error cannot still be caught as a TOML/retired violation. Require each faulty variant to fail the test assertion intended to catch its newly accepted error, record the real status and assertion name, and reject accidental syntax/other-check failure. No label-only or constructed-but-unexecuted controls.

Run the real checker, complete focused suite and bash syntax/diff; shared helper is unchanged, so no extra helper work is requested. Root checkpoints/pushes and Astra gives one attempt-2 verdict. At most one subsequent Sol attempt remains if necessary; full workspace comparison and actual PR/required CI follow focused acceptance. Do not weaken or defer these explicitly assigned selective/error/counter gates to claim the present green suite satisfies them.

This verdict used source, complete brief, workflow and completed-log inspection. No tests, Cargo, timing, repository/Git or GitHub mutation were performed.
