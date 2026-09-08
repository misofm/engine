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
