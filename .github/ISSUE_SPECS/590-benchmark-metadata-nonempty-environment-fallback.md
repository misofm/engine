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
