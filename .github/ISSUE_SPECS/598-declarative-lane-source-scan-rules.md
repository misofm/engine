# Declare the four lane source-scan rules as data

Status: proposed TOOL11 child of audit #349 and lane-B handoff #560, based on delivered main `be17e3293fa7fabb425d1b7eb6edd20bd13c6867` after #596 / PR #597 and successful post-main qualification `34175470011`. This advances an original partial finding, not an original open finding. Sol HIGH coordinates and owns checkpoints, GitHub synchronization and delivery. Luna HIGH or XHIGH implements. Astra LOW performs every scope, source and exact-head/current-base verification. No audio, browser, artifact or timed benchmark qualification is indicated.

## Smallest closable outcome

Move exactly the four existing Rust source-scan rule definitions in `scripts/check-lane-policy.sh`—fusion/SIMD vocabulary, relaxed SIMD, raw architecture intrinsics and runtime SIMD detection—into one declarative TOML file. Load and validate those records through one narrow standard-library-only loader, then execute them in their existing order through the delivered checked scan and exclusion primitives from `scripts/lib/gate.sh`.

Preserve every current regex, `*.rs` glob, root (`crates`, `hosts`, `tools`, `sidecars`), exemption regex, diagnostic and fail-closed status rule byte-for-byte in behavior. An absent exclusion remains a real empty value. This child implements only the declarative-rule step two explicitly excluded from the delivered #306/#403 chain; it does not require other gates or approved bespoke wrappers to use this representation and does not reopen their checked-producer, extractor, status or mutation work.

## Exact ownership

Allowed implementation paths are:

- `scripts/check-lane-policy.sh`
- `scripts/test-lane-policy.sh`
- new `scripts/policies/lane-source.toml`
- new `scripts/lib/gate-rules.py`
- this numbered spec and bounded issue evidence

The loader may use Python's standard-library `tomllib`; add no dependency or manifest. Its output protocol must be explicit, validated, lossless for the frozen strings, and consumed without `eval`. Freeze it to these four source-scan records and their exact fields; it is not a general executable policy language.

Exclude `scripts/lib/gate.sh`, every other policy/check/test script, workflow files, manifests/lockfiles, marker-window logic, dependency extraction, pins and workspace membership controls, product/runtime/host/DSP/session/control/SDK/browser code, generated artifacts and pins. Active lane-A #594 owns prepared builtin endpoint lifecycle source/tests and is disjoint. Lane B retains AudioWorklet artifact qualification/pinning ownership if #594 changes the shipped six-file artifact.

## Objective gates

1. The TOML declares exactly four unique, ordered rules and owns each existing ID, scan description, forbidden regex, `*.rs` glob, four roots, optional exclusion description/regex and failure diagnostic. The shell contains no second copy of those values and delegates execution to the existing checked scan/filter primitives.
2. Existing lane policy and its complete mutation suite pass unchanged in accepted behavior. Each rule independently rejects a forbidden occurrence; every current exemption is accepted, while adjacent non-exempt paths are rejected. Rule order and emitted policy diagnostics remain exact.
3. Missing or malformed policy, unknown fields, duplicate IDs, missing required fields, an empty rule population, wrong rule count/order, empty roots, missing scan roots, invalid regex, and invalid loader output fail closed with the lane policy diagnostic.
4. Scan, exclusion-filter and loader producer failures cannot pass, including nonempty partial output. A swallowed-status counter-mutant reaches and fails the intended assertion. Fixture resource copying includes the policy and loader without weakening any existing counter-mutant.
5. `bash scripts/check-lane-policy.sh`, `bash scripts/test-lane-policy.sh`, Python syntax checks, shell syntax checks, formatting/diff, workspace policy and its mutation suite pass. No marker, dependency, pin, membership or product gate changes receive credit. No audio, browser, artifact or timed benchmark run receives credit.

## Stop and split triggers

Stop before adding another rule kind or policy consumer, migrating another gate, introducing arbitrary expressions or `eval`, adding dependencies, changing a ban/exemption/diagnostic/order, editing `gate.sh`, replacing an approved bespoke wrapper, or touching marker/dependency/pin/membership/product/audio/browser/artifact paths. Preserve the checkpoint and brief a separate successor if these four rules cannot be moved within the frozen four implementation/test paths.

One Luna HIGH or XHIGH attempt receives one Astra LOW adversarial verdict and pauses at the first coherent focused-green tranche for root checkpointing. After three failed attempts, preserve evidence and rebrief without weakening gates. Historical #306/#403 child attribution, #543/#555 → #558/#552 and #542 → #567 delivery order, and all earlier verdict provenance remain unchanged.

## Preliminary residual audit

Astra LOW reviewed delivered main `be17e3293fa7fabb425d1b7eb6edd20bd13c6867`. `scripts/check-lane-policy.sh` currently embeds four repeated scan/filter/reject rule shapes at the fusion, relaxed-SIMD, architecture and detection sites. Durable #306/#403 closure records state that checked producers, extraction, status propagation and mutation coverage are delivered while declarative policy TOML/runner step two remains excluded. The former temporary reconciliation path is unavailable, but the committed records preserve that disposition.

This issue advances only that excluded step. TOOL11 remains partial after this child until Astra LOW performs a post-delivery residual audit; this brief does not predeclare closure. Activation requires exact local/GitHub numbered identity, a pushed clean brief, current-base and ownership checks, and Astra LOW scope PASS before implementation.

## Numbered current-base scope review

Astra LOW returned **PASS** for exact clean brief and upstream `57a76b6c9a8af09ae4279c8a91a2109f272609d9`, with current remote main and merge-base `be17e3293fa7fabb425d1b7eb6edd20bd13c6867`. The sole delta is this spec, diff checks pass, GitHub #598 has exact open identity/body, and base qualification `34175470011` succeeded. The four-rule extraction, standard-library TOML loader, lossless no-`eval` output, reuse of checked primitives and adversarial loader/status/counter-mutant gates are sufficiently bounded and disjoint from #594.

No blocking correction is required. The review identified and this continuation corrects one editorial path-count statement: exact ownership freezes four implementation/test paths plus this spec/evidence. Astra LOW is sufficient and no artifact, audio, browser or timed benchmark qualification applies. Luna HIGH or XHIGH attempt 1 may begin after Astra LOW binds this correction-and-verdict-only continuation to its exact pushed head.

## Attempt 1 implementation checkpoint

Luna HIGH delivered source checkpoint `83e00bf49bb5cda045bb8fc7b5691b2d33d7cca1`, changing exactly the four authorized implementation/test paths. The four ordered lane source rules now live in `scripts/policies/lane-source.toml`. A standard-library `tomllib` loader validates version, schema, exact IDs/order/count, required/optional fields, roots and regexes, then emits each string as base64 in an explicit tabular protocol. The shell decodes without `eval` and runs every rule through the existing checked scan and optional exclusion-filter primitives.

The mutation harness now copies the loader and policy into counter-mutants and covers missing/malformed policy, unknown and missing fields, empty population, duplicate/wrong order, empty roots, invalid regex and invalid loader output in addition to its existing independent rule, exemption, missing-root, scan/filter partial-output and swallowed-status controls. Root reran Python and shell syntax, lane policy and its complete mutation suite, workspace policy and its complete mutation suite, and diff checks successfully. Python bytecode cache output was removed; only the four authorized paths are in the source checkpoint. No audio, browser, artifact or timed benchmark work ran. Astra LOW attempt-1 review must verify that loader failure/partial-output and counter-mutant coverage fully discriminate the new path.

## Attempt 1 source verdict

Astra LOW returned **FAIL** at exact head/upstream `117ff49036f8b0fc94789acae0927792588cede3`, source `83e00bf49bb5cda045bb8fc7b5691b2d33d7cca1`, and current main/merge-base `be17e3293fa7fabb425d1b7eb6edd20bd13c6867`. Empty base64 exclusion fields are not lossless through Bash's whitespace-IFS tab parsing: the relaxed rule shifts its failure diagnostic into the exclusion field. The invalid-output fixture is Bash executed through Python and proves only syntax failure. No causal status-zero malformed-output, nonzero partial/complete-output or swallowed-loader-status counter-mutant exists. Empty-root and invalid-regex edits create malformed TOML instead of reaching their intended validations. Exemption/adjacent-path and competing-rule order/diagnostic cases are incomplete.

Independent lane/workspace policies and mutation suites, Python AST and Bash syntax, and diff checks pass, but do not discriminate those defects. TOML values match the original rules, checked primitives are reused, no production `eval` or dependency/excluded-path change exists, GitHub identity is exact and #594 is disjoint. Attempt 2 is authorized only for these corrections within the existing four paths. Astra LOW remains sufficient; no artifact, audio, browser or timed qualification applies.

## Attempt 2 implementation checkpoint

Luna HIGH delivered correction checkpoint `a7a7391c8f8ee44038b3207cd245b9d62b5cd8f8` within three authorized paths; the TOML policy remains unchanged. The loader and shell now use a non-whitespace record delimiter so the relaxed rule's empty optional exclusion fields and failure diagnostic retain their exact positions. Loader controls are valid Python programs that causally exercise status-zero malformed output and nonzero complete/partial output. A swallowed-loader-status source mutant must reach and fail the intended malformed-output assertion. Empty-root and invalid-regex mutations remain syntactically valid TOML and assert their specific schema diagnostics. Compact fixtures cover every exemption, adjacent non-exempt paths, competing violations, exact first-rule order and diagnostics.

After rejecting an initial handoff whose fixtures were still Bash text invoked by Python, root reran the corrected final tree. Bash and Python AST syntax, lane policy and its complete mutation suite, workspace policy and its complete mutation suite, and diff checks pass; the lane suite prints its success marker. No generated bytecode remains, and the worktree is clean at the pushed correction checkpoint. No audio, browser, artifact or timed benchmark work ran. Astra LOW attempt-2 source review remains required.

## Attempt 2 source verdict

Astra LOW returned **FAIL** at exact clean head/upstream `eaa215cb4d4a052a602cc647563af24ae5ab81d2`, correction `a7a7391c8f8ee44038b3207cd245b9d62b5cd8f8`, and merge-base `be17e3293fa7fabb425d1b7eb6edd20bd13c6867`. Attempt 2 fixes the empty-column transport defect, causal valid-Python output/status controls, swallowed-loader-status mutant and syntactically valid schema controls. It still accepts a status-zero relaxed record whose final diagnostic field is absent because the shell does not enforce exact cardinality or require a nonempty failure diagnostic. Gate 2 also lacks positive audit-fusion and lane-lib detection exemptions, adjacent negatives for several exemptions, and exact relaxed-before-architecture plus architecture-before-detection order/diagnostic cases.

Independent lane/workspace checkers and mutation suites, Bash/Python syntax and diff checks pass, but do not cover those two gaps. GitHub #598 remains exact and open; source scope is preserved. Main advanced disjointly through #594 and #600 to `ccafe150bb8b129d85d30601cda1f6f68176127c`, whose required qualification `34184236199` succeeded. Preserve this FAIL, integrate current main, and use the third and final attempt only for exact consumer-record validation and the missing compact behavior fixtures. Astra LOW remains sufficient; no artifact, audio, browser or timed qualification applies.

## Attempt 3 current-base authorization

Root integrated current main `ccafe150bb8b129d85d30601cda1f6f68176127c` without conflict at pushed head `9b606cc2a7c60f11f7bbde17598afc380ec3daa6`; that main is the exact merge-base. The lane policy and complete lane mutation suite pass after integration. Lane-A #603 owns disjoint capture-script/evidence paths. Luna HIGH or XHIGH attempt 3 is authorized only to enforce exact loader-record shape and required decoded fields, and to add the missing compact exemption, adjacent-path and ordered-diagnostic fixtures. This is the final attempt.
