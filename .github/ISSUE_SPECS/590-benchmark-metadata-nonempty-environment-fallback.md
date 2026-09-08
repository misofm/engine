# Share nonempty environment fallback in benchmark metadata

Status: proposed numbered TOOL9 child of audit #349 and lane-B handoff #560, based on delivered main `08dabaf7a8b55a5e89a6b7e2a41bf89d445267f6` after #588 / PR #589 and successful post-main qualification `34170256498`. This advances an original partial finding, not an original open finding. Sol HIGH coordinates and owns checkpoints, GitHub synchronization and delivery. Luna HIGH implements. Astra LOW performs every scope, source and exact-head/current-base verification. No audio, browser or artifact qualification is indicated.

## Smallest closable outcome

Add one explicitly named nonempty-or-unknown lookup method to the existing immutable `bench_support::metadata::Metadata` snapshot. Migrate only the session and protocol benchmark helpers that currently implement that exact policy. Preserve their behavior: retain every nonempty Unicode value byte for byte, while absent, non-Unicode and empty values become exactly `"unknown"`.

Keep `Metadata::var` and every other consumer unchanged. Whitespace-only values and strings such as `"default"` and `"not measured"` are nonempty and must remain untouched. Protocol's later numeric parsing and both record schemas remain local and unchanged. This removes one concrete metadata-projection duplicate; TOOL9 remains partial afterward.

## Exact ownership

Allowed implementation paths are:

- `tools/bench-support/src/metadata.rs`
- `tools/bench/src/session.rs`
- `tools/bench/src/protocol.rs`
- this numbered spec and bounded issue evidence

Exclude every other metadata consumer and policy, JSON serialization, schemas, numeric parsing/defaults, timing/workloads/corpora, manifests/lockfiles, policies/workflows, runtime/host/DSP/session-compiler/control code, SDK/browser code, generated artifacts and pins. Active lane-A #587 owns its documented `host-core` preparation/endpoint/test and bounded evidence paths; there is no overlap.

## Objective gates

1. Through injected private metadata snapshots, prove missing, non-Unicode and empty values map to exactly `"unknown"` without mutating the process environment or depending on the global `OnceLock` order.
2. Prove whitespace-only, Unicode, `"default"`, `"not measured"` and other nonempty values remain byte-identical, with no trimming or sentinel filtering.
3. Both session and protocol production helpers delegate to the shared method and their private duplicate lookup bodies disappear. `Metadata::var` and all other consumers remain unchanged.
4. Existing session and protocol record oracles pass unchanged, including protocol numeric-default behavior.
5. Focused bench-support/session/protocol tests pass in debug and release; complete affected bench debug and proportional release tests pass. Strict affected Clippy/rustdoc, formatting/diff, workspace policy, bench policy and its mutation suite pass. Preserve the unrelated release allocator disposition. No timed benchmark, browser or artifact run receives credit.

## Stop and split triggers

Stop before migrating another consumer, normalizing distinct policies, changing a schema or numeric parser/default, introducing public environment injection, changing `Metadata::var`, or touching a manifest/lock, policy/workflow, workload/timing, runtime/audio/browser/artifact path. Preserve the checkpoint and brief a separate successor if the exact duplicate cannot be removed within these three source files.

One Luna HIGH attempt receives one Astra LOW adversarial verdict and pauses at the first coherent focused-green tranche for root checkpointing. After three failed attempts, preserve evidence and rebrief without weakening gates. Historical #543/#555 → #558/#552 and #542 → #567 delivery order and verdict provenance remain unchanged.

## Preliminary residual audit

Astra LOW reviewed current main `08dabaf7a8b55a5e89a6b7e2a41bf89d445267f6` and found the exact policy duplicated in `tools/bench/src/session.rs` and `tools/bench/src/protocol.rs`. Both use the frozen `bench_support::metadata::Metadata` snapshot, retain nonempty Unicode strings unchanged, and map absence, invalid Unicode and empty strings to `"unknown"`. Session uses the result for runtime/browser metadata; protocol uses it for target/Wasm metadata and subsequent local numeric parsing.

Rack's ASCII rule, builtins' non-control-Unicode rule, graph's sentinel tracking and interchange's missing-value policy are intentional differences and are excluded. TOOL11 and IO5 still require broader contracts. This child is disjoint from #587 and the live PR/worktree roster. Astra LOW is sufficient for all review. Activation requires exact local/GitHub numbered identity, a pushed clean brief, current-base and ownership checks, and Astra LOW scope PASS.

## Numbered current-base scope review

Astra LOW returned **PASS** for exact brief and upstream `103775b5877edde27774a7fb9e6251fdc8c0dade`, with current main and merge-base `08dabaf7a8b55a5e89a6b7e2a41bf89d445267f6`. The clean tree's sole delta is this spec, diff checks pass, GitHub #590 has exact open identity, and base qualification `34170256498` succeeded. #560 activation, #559/#587 ownership and the live PR/worktree boundaries are consistent and disjoint.

The policy bodies are identical. Three-file ownership and the injected-snapshot gates adequately preserve all value distinctions, `Metadata::var`, other consumers, record oracles and protocol numeric defaults. No correction is required; Astra LOW is sufficient. Luna HIGH attempt 1 may begin from this exact brief/base and must pause at its first focused-green checkpoint. No audio, browser, artifact or timed benchmark work is authorized.

## Attempt 1 implementation checkpoint

Luna HIGH delivered source checkpoint `fb94415538c87f58907596ab312c5aab3e0c59ae`, changing exactly the three authorized tooling files. `Metadata::nonempty_or_unknown` implements the existing policy, and the session and protocol production helpers now delegate to it. Private injected-snapshot tests cover missing, non-Unicode, empty, whitespace-only, Unicode, `"default"`, `"not measured"` and ordinary values without process-environment mutation or `OnceLock` ordering.

Focused bench-support/session/protocol debug tests pass 1, 3 and 5 tests; proportional release tests pass the same counts; complete bench debug passes 43 tests. Strict affected Clippy/rustdoc, formatting/diff, workspace policy, bench policy and mutation gates pass. A transient lockfile reorder was restored; only the three authorized paths remain in the source diff. Record oracles and protocol numeric defaults remain unchanged. No timed benchmark, browser or artifact work ran. Astra LOW attempt-1 adversarial source review remains pending.

## Attempt 1 source verdict

Astra LOW returned **PASS** at exact pushed review head and upstream `d57b1e64385353bd8794725608b3357f2655f22b`, source `fb94415538c87f58907596ab312c5aab3e0c59ae`, and current main/merge-base `08dabaf7a8b55a5e89a6b7e2a41bf89d445267f6`. Private-snapshot tests discriminate every required value class without environment mutation or `OnceLock` dependence. Both production helpers delegate correctly; `Metadata::var`, other consumers, record oracles and numeric defaults remain unchanged.

Independent full debug bench-support/bench tests pass 44 and 43 tests, release metadata/session/protocol tests pass 2, 3 and 5 tests, and strict Clippy/rustdoc, formatting/diff, workspace policy, bench policy and mutation gates pass. The pushed tree is clean, GitHub identity and allowed-path census pass, and there is no #587 overlap or excluded drift. The unrelated allocator disposition remains untouched. No correction or current-base integration is needed. Astra LOW is sufficient; final exact-head delivery review remains required before PR authorization.
