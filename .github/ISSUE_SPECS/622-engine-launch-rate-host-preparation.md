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
2. Prove the invariant from production code: all `CompiledSession` constructors remain behind session validation. With otherwise-valid sessions and generous compile caps, unsupported launch rates must still fail with the exact field-local session diagnostic `sample_rate.unsupported_at_launch\t$.sample_rate_hz` before host-shape validation; this issue makes no diagnostic-precedence claim over earlier estimation or cap failures.
3. Exercise every canonical `engine::LAUNCH_SAMPLE_RATES` entry through `AnyLaunchRate`. Exercise adjacent unsupported rates through the public compile/prepare boundary. Preserve exact-rate and wrong-rate/quantum `Exact` cases and valid/invalid source-ring cases under both policies.
4. Require a live Rust production/test-consumer scan to find no remaining `LAUNCH_SAMPLE_RATES_HZ` or `host.sample_rate.unsupported` spelling, excluding historical specs and evidence that must remain intact. Require the canonical engine constant to have a single literal definition and both affected test consumers to use its typed entries.
5. Run focused and complete debug/release `host-core` tests, the affected `parameter-metadata` ABI-layout test, strict affected Clippy, workspace formatting/diff checks, workspace policy, and relevant host/realtime policies. No benchmark or timing invocation is authorized.
6. Root checkpoints each coherent green tranche before more implementation. Astra LOW adversarially reviews the exact pushed source/evidence head and again after any current-main integration.
7. Root makes and records an AudioWorklet artifact-applicability decision after source PASS. `host-web` directly depends on `host-core`, so artifact neutrality must not be presumed from the source shape. Any authorized identity probe and any drift qualification/pinning successor remain separate from implementation and lane-B/root-owned.
8. Open one PR only after exact-head/current-main Astra LOW PASS. Require the repository `qualification` check, verify live main immediately before guarded exact-head merge, verify merge parents and post-main qualification, synchronize #560/#622, and remove the clean delivered worktree.

One Luna implementation pass is initially authorized after scope PASS. A substantive finding receives at most the remaining attempts under the repository's three-attempt rule. Gates may not be weakened and no fourth retry is allowed.

## Astra LOW initial scope review — FAIL

Astra LOW returned **FAIL** at exact clean pushed head
`4a10788e1b75f3e9960bce55235d54aaff27e487`, live main
`cf9e079cd5ef80d1c7284e9edd0ffcc90b0db335`, and tracker
`1351a37c8b495d8fb54d7be57cbc604943b6c882`. The bounded implementation and path ownership were
sound, but the brief incorrectly presumed artifact N/A despite `host-web`'s direct `host-core`
dependency, applied the removal scan to historical records, and did not isolate the rate diagnostic
from earlier estimation/cap failures. The three objective gates above now require an explicit
post-source artifact decision, a live Rust consumer scan that preserves history, and otherwise-valid
sessions with generous caps. No implementation or artifact action was authorized by this verdict.

## Astra LOW corrected scope review — PASS

Astra LOW returned **PASS** at exact clean pushed feature head
`f007e4623e1f0e6f4eee0977ee063fcee7c78b39`, live main/merge-base
`cf9e079cd5ef80d1c7284e9edd0ffcc90b0db335`, and exact clean pushed tracker
`fc5c161c4bf5a5b3b3a9b7bb0aa102c82a810510`. The three substantive corrections pass, the stale
#559 active-slot sentence is corrected, local/GitHub #559/#560/#622 bodies match, the feature tree
is clean, and #621/#622 ownership is disjoint. Luna HIGH/XHIGH may begin the bounded first
implementation pass in the four named source/test paths and issue evidence. Artifact applicability
remains a separate root-owned decision after source PASS.

## Luna HIGH attempt 1 checkpoint

Luna HIGH changed exactly the four authorized source/test paths, and root checkpointed the coherent
tranche as `fece7a2ccf5796da71a29168cd865b921bb0679f`. `host-core` now publicly re-exports
`engine::LAUNCH_SAMPLE_RATES`; the host-owned raw array, unreachable `AnyLaunchRate` membership
test, and dead `host.sample_rate.unsupported` outcome are removed. The two repository test consumers
use the typed canonical entries. Focused coverage now sends all four canonical rates through
`prepare_host_session`, keeps otherwise-valid adjacent-rate session diagnostics, and asserts ring
rejection under both AnyLaunchRate and Exact policies while retaining the existing Exact rate and
quantum mismatch cases.

Luna reported focused host-core prepare debug/release 13 passed and 1 ignored per profile; complete
host-core debug/release suites and doctests passed; parameter-metadata ABI layout debug passed 7/7;
strict affected Clippy, formatting, diff, workspace, host-core and realtime policies passed. The live
Rust scan found neither removed spelling and `Cargo.lock` is unchanged. A release-profile
parameter-metadata ABI-layout command was attempted twice and failed during Cargo duplicate-output/
crate resolution with E0463 for `effect_compiler`/`host_core`; it did not reach the test. This
failure is preserved as reported and is not waived or classified as baseline. Astra LOW must
independently review the exact pushed source, rerun only the proportional gates needed for its
verdict, and decide whether that release failure is attributable to the tranche.

Final SHA-256 values are `0bf7982674c44d07a62d54626f88b6e776ea1a3c8cfa91a5e5a5cb5f7c6bc341`
for `crates/host-core/src/lib.rs`, `6a2befa5c5e6d7551943b080ce3667b20ff6e19d676c44f74d6f5c596852fc7e`
for `crates/host-core/src/prepare.rs`, `171b9ff9a7bb889ddad47ad9dad09c54a4ce35b78d9003ddba86aff5475d828b`
for `crates/host-core/tests/prepare.rs`, and `a4edd2cf03e0e894b9da06ee6722e846fed12b22ace13eed8b8616991a85b917`
for `tools/parameter-metadata/tests/abi_layout.rs`. This is implementation evidence, not source or
artifact acceptance.

## Astra LOW attempt 1 source review — PASS

Astra LOW returned **PASS** at exact clean pushed evidence head
`c2a40c49422d5122b7d467df028a3ab62f9e54e9`, source
`fece7a2ccf5796da71a29168cd865b921bb0679f`, live main/merge-base
`cf9e079cd5ef80d1c7284e9edd0ffcc90b0db335`, and synchronized tracker
`78bdc087b85503348e8cb952f7371028e69ec4f3`. The reviewer confirmed the private-field and sole-
constructor `CompiledSession` invariant, canonical typed re-export, removed live Rust spellings,
unchanged reachable Exact/ring diagnostics and order, discriminating tests, four-path scope, lock,
issue synchronization, and #621 disjointness.

Independent host-core debug and release suites each passed 82 tests with 2 ignored, including
doctests; focused prepare debug passed 13 with 1 ignored; metadata ABI layout debug passed 7/7;
strict affected Clippy, formatting, diff, host-core, workspace and realtime policies passed. The
release metadata command failed with status 101 in separate fresh targets on both this head and exact
main, with the same duplicate-output collisions and E0463 `effect_compiler` resolution failure at
`graph-compiler/src/lib.rs:12`. This is independently reproduced pre-existing infrastructure failure,
not a successful gate and not a #622 correction. Source is accepted. Root may make the separately
owned artifact-applicability decision; no artifact, PR or merge action is authorized by this verdict.

## Ordinary AudioWorklet artifact probe — DRIFT

Root ran exactly one ordinary no-bypass `scripts/build-web-audioworklet.sh` invocation at clean,
pushed source-accepted head `ca5a8b49`, whose accepted production source is `fece7a2c`, against live
main/merge-base `cf9e079c`. Compilation succeeded, then the unchanged builder exited 1 before
copying output because candidate simd128 Wasm
`ac71c64033b0cfc637cf14edcacaa6ed1b3bbf7093a5caa641ef84adea7e88e3` differs from delivered pin
`f80b6392b1ea7141aaac639d08094f88982febb883c4d08c3f1114418093e664`. The scratch output census is
zero and the builder's temporary module was removed by its exit trap.

Complete checksum-verified probe evidence is under `artifacts/issue622-artifact-probe/`. No retry,
overlay build, qualification gate, browser/SDK run, repin, consumer update, benchmark, timing
workload, source edit, workflow edit, or main mutation occurred. Retained qualification does not
apply. #622 source is passive while one separately numbered lane-B qualification/pinning successor
reproduces the candidate from this frozen source with only scratch pin/lineage overlays, runs the
existing static/resource/hermetic/SDK/three-browser gates, obtains Astra LOW review, and conditionally
promotes only the repository pin and generated qualification lineage before post-pin proof.
