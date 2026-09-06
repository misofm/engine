# #443 bounded release-test qualification ruling

APPROVE the two-stage execution decision below; no source/Cargo/profile/benchmark change. This is a qualification procedure ruling, not a new feature attempt or waiver of release correctness. Read the recorded baseline command/error and current Cargo/dependency/policy sources; no builds/tests executed by this reviewer.

The exact delivered baseline reproduces E0463 for effect_compiler. Graph-compiler normally depends on effect-compiler, which depends on effect-package; effect-package emits both rlib and cdylib. Release is fat LTO, codegen-units1, panic=abort, debug1. docs/REALTIME_DEPENDENCY_POLICY.md:209–231 already documents test unwind versus ordinary abort unit duplication and unhashed shared/static outputs, and endorses a test-only unwind override for the workspace test command. scripts/run-release-workspace-tests.sh implements it. The error is consistent with that known collision, but source inspection alone does not prove that every per-package failure has exactly that cause. Preserve observed errors instead of declaring a speculative diagnosis certain.

The policy's later blanket claim that per-package commands cannot encounter the collision is contradicted by the retained exact baseline failure under the present dependency graph. Record this narrowly in #443; it does not authorize changing shipped panic behavior or expanding to an all-features workspace command. The current helper forces --workspace --all-targets, so do not invoke that entire helper merely to qualify one blocked crate.

## Fixed bounded decision sequence

1. On the frozen baseline, one clean isolated target, try only:
   PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=<owned-baseline-lib-target> cargo test --locked --release -p graph-compiler --lib scalar_dispatch_compiles_without_banks_on_any_host
   Retain exact command, status and output; verify the named test actually executes. --lib reduces requested target population while preserving all release settings. It may still fail because the colliding crate is a dependency; do not promise it fixes the issue. If it succeeds, use the same --lib shape for the candidate's complete required graph-compiler unit tests (including the new scalar fixture), retaining ordinary release settings.

2. If that single baseline --lib attempt still hits the collision, use the already established test-only panic-unification mechanism, narrowly scoped:
   PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=<owned-unwind-test-target> CARGO_PROFILE_RELEASE_PANIC=unwind cargo test --locked --release -p graph-compiler --lib
   Execute the relevant baseline and candidate in separately owned targets or otherwise strictly isolated serialized build outputs. No --workspace/--all-features expansion is needed. Preserve fat LTO, codegen-units1, target CPU/features, opt level, toolchain, dependency lock and all other settings. Record exact immutable hashes and actual nonempty test names/counts. --no-run is insufficient for the required PCM/state/resource correctness gate.

This is one bounded target-selection experiment followed, if necessary, by the existing documented panic workaround. Do not repeat identical failing commands, tune codegen, switch dependencies, add a profile or repair production source to make tests link. If the second shape fails, stop with the retained error and scope a separate tooling successor; do not broaden #443 or call the release proof complete.

## Claim separation and remaining delivery

A successful unwind-profile test proves optimized correctness in the explicit test configuration. It is NOT the shipped abort artifact or evidence of its exact byte identity/performance. Test harnesses already unwind by Cargo design; the override unifies ordinary dependency units with that harness. Keep this exception local to the command, never export it into subsequent shells or reuse its target directory for shipped artifacts.

Retain separate normal panic=abort shipped/native/Wasm artifact construction and existing supported ABI/static/realtime/resource/browser gates under the accepted immutable delivery workflow. Those mandatory shipped-artifact checks preserve the target/abort contract. Other already successful per-package release gates need not be rerun with this override absent evidence of the same issue. No benchmark invocation, AArch64 revival, pin refresh or expanded DSP matrix follows.

A new issue is not needed merely to execute this established test-only workaround. Persistent runner/dependency-shape repair or moving panic settings between profiles is separate tooling/architecture scope and requires its own issue/ruling. The Sol3 source acceptance remains governed by its full original product proof; qualification records must distinguish this inherited build failure and the exact successful replacement gate, if obtained.
