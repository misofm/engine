# Use the engine launch-rate authority during host preparation

GitHub: https://github.com/misofm/engine/issues/622

Parent: #560 (lane B, IO21). Coordination: #559. Baseline: delivered main `cf9e079cd5ef80d1c7284e9edd0ffcc90b0db335`.

`compile_session` validates the session rate through `engine::is_launch_sample_rate` before it can construct a `CompiledSession`. `HostPrepareCaps::validate_shape` nevertheless checks `CompiledSession::sample_rate()` against a second raw `u32` array for `AnyLaunchRate` and can return `host.sample_rate.unsupported`. No valid caller can reach that rejection. This issue removes the unreachable boundary-law repetition and makes the engine's typed launch-rate set the one discoverable authority used by host tests and metadata qualification.

The global eight-partial barrier is clear. #539/#619 delivered through PR #620, required PR qualification and post-main qualification succeeded, and their clean worktrees are removed. Lane A issue #621 owns only true-peak limiter source/tests/evidence. This issue owns disjoint host preparation and metadata-test paths and is the second active issue slot. Sol HIGH coordinates documentation, checkpoints, GitHub synchronization, and artifact applicability. Luna HIGH or XHIGH implements. Per the user's current routing, Astra LOW performs every scope, implementation, exact-head, integration, artifact-applicability, and delivery verification that would otherwise use Sol HIGH/XHIGH.

## Exact ownership and outcome

Production ownership is limited to:

- `crates/host-core/src/prepare.rs`
- `crates/host-core/src/lib.rs`

Focused verification may change:

- `crates/host-core/tests/prepare.rs`
- `tools/parameter-metadata/tests/abi_layout.rs`
- this numbered spec and bounded evidence

Remove `host-core`'s independently spelled `LAUNCH_SAMPLE_RATES_HZ` value and publicly re-export the existing `engine::LAUNCH_SAMPLE_RATES` typed constant so downstream repository checks use the canonical values without another array or conversion authority. This is a prelaunch Rust API cleanup; no wire, schema, C ABI, browser ABI, exported symbol, diagnostic payload, session JSON, or artifact spelling changes. Repository callers of the removed raw-array spelling must migrate in the same checkpoint.

For `HostShapePolicy::AnyLaunchRate`, remove only the unreachable launch-rate membership test and `host.sample_rate.unsupported` outcome. A `CompiledSession` has already crossed the session validator, so this policy accepts its validated rate and continues to enforce the source-ring shape. Preserve `HostShapePolicy::Exact` sample-rate/quantum equality and its existing `host.session.shape` diagnostic. Preserve `host.source.ring_frames`, validation order for reachable failures, compile caps, resource projections, allocations, preparation behavior, and every source/effect/graph path.

Do not change `engine` or `session`, add a dependency, add a conversion helper or replacement raw-rate array, broaden host policy, alter accepted session JSON, touch audio/DSP/realtime code, run a benchmark, regenerate browser artifacts, or change pins/workflows. If implementation requires any such change, stop for rescope.

## Objective gates

1. Before implementation, push and synchronize this brief and the #559/#560 coordination records, then obtain Astra LOW exact-clean scope PASS.
2. Prove the invariant from production code: all `CompiledSession` constructors remain behind session validation, and unsupported launch rates still fail with the exact field-local session diagnostic `sample_rate.unsupported_at_launch\t$.sample_rate_hz` before host-shape validation.
3. Exercise every canonical `engine::LAUNCH_SAMPLE_RATES` entry through `AnyLaunchRate`. Exercise adjacent unsupported rates through the public compile/prepare boundary. Preserve exact-rate and wrong-rate/quantum `Exact` cases and valid/invalid source-ring cases under both policies.
4. Require a repository scan to find no remaining `LAUNCH_SAMPLE_RATES_HZ` or `host.sample_rate.unsupported` spelling. Require the canonical engine constant to have a single literal definition and both affected test consumers to use its typed entries.
5. Run focused and complete debug/release `host-core` tests, the affected `parameter-metadata` ABI-layout test, strict affected Clippy, workspace formatting/diff checks, workspace policy, and relevant host/realtime policies. No benchmark or timing invocation is authorized.
6. Root checkpoints each coherent green tranche before more implementation. Astra LOW adversarially reviews the exact pushed source/evidence head and again after any current-main integration.
7. Root records AudioWorklet artifact applicability after source PASS. Because this is host preparation and test-only metadata use, expected disposition is N/A with a source-dependency proof; any actual six-file source dependency or byte drift requires a separately numbered lane-B qualification issue before pinning.
8. Open one PR only after exact-head/current-main Astra LOW PASS. Require the repository `qualification` check, verify live main immediately before guarded exact-head merge, verify merge parents and post-main qualification, synchronize #560/#622, and remove the clean delivered worktree.

One Luna implementation pass is initially authorized after scope PASS. A substantive finding receives at most the remaining attempts under the repository's three-attempt rule. Gates may not be weakened and no fourth retry is allowed.
