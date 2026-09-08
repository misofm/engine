# Reuse shared nonempty metadata lookup in effect-contract benchmark

Status: proposed final TOOL9 child of audit #349 and lane-B handoff #560, based on delivered main `2119544764dd351b130b6fe262162bc5b9cb2d24` after #593 / PR #595 and successful post-main qualification `34173840466`. This advances an original partial finding, not an original open finding. Sol HIGH coordinates and owns checkpoints, GitHub synchronization and delivery. Luna HIGH or XHIGH implements. Astra LOW performs every scope, source and exact-head/current-base verification. No DSP, audio, browser, artifact or timed benchmark qualification is indicated.

## Smallest closable outcome

Replace only effect-contract benchmark metadata's duplicate nonempty environment lookup with the delivered `bench_support::metadata::Metadata::nonempty_or_unknown` authority from #590. Keep effect-contract's existing quote substitution local and byte-exact: after lookup, every `"` remains replaced by `'` and no other character is changed.

This is the final concrete TOOL9 duplicate found by Astra LOW after #593. Missing, non-Unicode and empty values must remain `"unknown"`; whitespace, Unicode and every nonempty sentinel remain unchanged before quote substitution. The six production metadata call sites and their order, the record schema and format string, process-level immutable snapshot, acquisition timing, workload and output count remain exact.

## Exact ownership

Allowed implementation paths are:

- `tools/bench/src/effect_contract.rs`
- this numbered spec and bounded issue evidence

Exclude `tools/bench-support`, every other benchmark consumer, metadata acquisition or helper changes, JSON escaping corrections, schemas and record templates, formatters/statistics, manifests/lockfiles, policies/workflows, runtime/host/DSP/session/control/SDK/browser code, generated artifacts and pins. Active lane-A #594 owns its prepared builtin endpoint lifecycle paths and is disjoint from this one-file tooling slice. Lane B retains AudioWorklet artifact qualification/pinning ownership if #594 changes the shipped six-file artifact.

## Objective gates

1. Effect-contract production metadata obtains its value through `Metadata::gather().nonempty_or_unknown(name)` and then applies the existing exact double-quote-to-single-quote substitution. The duplicate `.var(...).ok().filter(...).unwrap_or_else(...)` lookup body disappears.
2. Existing shared lookup tests continue to prove missing, non-Unicode and empty values map to `"unknown"`, while whitespace, Unicode, ordinary values and nonempty sentinels are preserved. A deterministic effect-contract test proves its local projection replaces every double quote and changes no other bytes, including whitespace and Unicode.
3. The six production metadata names and order, complete benchmark record format, sentinel spelling, immutable snapshot use and all non-metadata behavior remain unchanged. Direct source inspection must show the shared lookup and local substitution compose in production.
4. Focused bench-support metadata and effect-contract tests pass in debug and release; the complete affected bench debug suite passes. Strict affected Clippy/rustdoc, formatting/diff, workspace policy, bench policy and its mutation suite pass. Preserve the unrelated release allocator disposition. No timed benchmark, audio, browser or artifact run receives credit.

## Stop and split triggers

Stop before changing the shared helper, migrating another consumer, correcting escaping, changing any schema/record template, adding environment mutation or generic metadata machinery, touching a manifest/lock or policy/workflow, or changing workload/timing/runtime/audio/browser/artifact paths. Preserve the checkpoint and brief a separate successor if the exact lookup migration cannot remain within the one production/test file.

One Luna HIGH or XHIGH attempt receives one Astra LOW adversarial verdict and pauses at the first coherent focused-green tranche for root checkpointing. After three failed attempts, preserve evidence and rebrief without weakening gates. Historical #543/#555 → #558/#552 and #542 → #567 delivery order and all earlier verdict provenance remain unchanged.

## Preliminary residual audit

Astra LOW reviewed delivered main `2119544764dd351b130b6fe262162bc5b9cb2d24`. At `tools/bench/src/effect_contract.rs`, the local metadata function repeats #590's exact nonempty lookup and then performs a distinct `.replace('"', "'")`. The lookup is the bounded duplicate; the replacement is a schema-specific consumer policy and stays local. Other remaining metadata structures and record assemblers encode differing fields, missing-value rules, character policies or acquisition choices and do not justify a shared schema or framework.

Activation requires exact local/GitHub numbered identity, a pushed clean brief, current-base and ownership checks, and Astra LOW scope PASS. Implementation is not authorized until that verdict is recorded and pushed. After delivery, Astra LOW must perform a final TOOL9 closure audit before #560 may mark the original partial finding delivered.

## Numbered current-base scope review

Astra LOW returned **PASS** for exact clean brief and upstream `813771980691f9cc5f5227057eeb57d73f0e5251`, with current main and merge-base `2119544764dd351b130b6fe262162bc5b9cb2d24`. The sole delta is this spec, diff checks pass, GitHub #596 has exact open identity/body, and base qualification `34173840466` succeeded. The one-file effect-contract tooling ownership is disjoint from #559/#594.

The brief preserves the shared lookup law, intentionally local quote substitution, six production call sites and complete record format. No correction is required; Astra LOW is sufficient, and no audio, browser, artifact or timed benchmark qualification is indicated. Luna HIGH or XHIGH attempt 1 may begin after this verdict-only continuation is pushed and synchronized, and must pause at the first focused-green checkpoint.

## Attempt 1 implementation checkpoint

Luna HIGH delivered source checkpoint `79977f9594ab3a47440c8a01a3da00060596a3bc`, changing only `tools/bench/src/effect_contract.rs`. Production metadata now composes `Metadata::gather().nonempty_or_unknown(name)` with the unchanged local double-quote-to-single-quote projection. A pure deterministic projection test covers whitespace, Unicode and multiple quotes. The six production callers, their order, record schema and format remain unchanged.

Focused bench-support metadata debug/release tests pass 2 tests each, the effect-contract debug/release test passes, and the complete bench debug suite passes 44 tests. Strict affected Clippy/rustdoc, formatting/diff, workspace policy, bench policy and its mutation suite pass. Cargo's generated lockfile ordering churn was restored, leaving only the authorized one-file source delta. No timed benchmark, audio, browser or artifact work ran. Astra LOW attempt-1 source review remains pending.

## Attempt 1 source verdict

Astra LOW returned **PASS** at exact pushed head and upstream `9ae363aff2e22bab6cb5820a3d6a8b76ea32474a`, source `79977f9594ab3a47440c8a01a3da00060596a3bc`, and current main/merge-base `2119544764dd351b130b6fe262162bc5b9cb2d24`. Production composes the shared nonempty lookup with the tested local quote substitution. All six callers, their order, the record template and non-metadata behavior remain unchanged. Only the authorized source file and this spec differ from main; the tree is clean, GitHub identity matches, and #594 remains disjoint.

Independent metadata debug/release tests pass 2 tests each, effect-contract debug/release passes 1 test each, and complete bench debug passes 44 tests. Strict Clippy/rustdoc, formatting/diff, workspace policy, bench policy and its mutation suite pass. No correction is needed. Astra LOW is sufficient and no artifact, audio, browser or timed qualification applies. Final exact-head delivery review and post-delivery TOOL9 closure audit remain required.
