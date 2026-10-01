# Make builtins-compiler's tests compile without test-support

## Product outcome

`cargo test -p builtins-compiler` without `--features test-support` fails to compile, already on `main` (`14f2917b`), because tests reach test-only APIs that the feature gates. CI only ever runs the crate with the feature, so nothing notices. A crate whose test target does not build in its default configuration is a trap for every implementer who runs the obvious command.

## Smallest closable slice

Either gate the affected tests behind `#[cfg(feature = "test-support")]` (or the crate's `[[test]] required-features`), or enable the feature for the crate's own tests through a `[dev-dependencies]` self-reference, whichever the crate's existing pattern prefers. Then make CI build the crate's tests in the default configuration once (`cargo test -p builtins-compiler --no-run`).

## Objective gates

- `cargo test -p builtins-compiler` and `cargo test -p builtins-compiler --features test-support` both build and pass.
- The CI job that builds workspace tests covers the default configuration of this crate.

Found by the #944 implementer and confirmed pre-existing by its verification.

## Sol brief approval and bounded execution — 2026-10-01

Root approves a qualification slice immediately after crate housekeeping #1134, within the user's request to keep the crate's tests necessary and usable. Worker B independently reproduced the existing default no-run failure before any #1134 edit: twenty E0425 errors, with feature-only first/last fader/matrix trace helpers referenced by two ordinary unit tests. The trace types and producers are already available to unit tests through cfg(test).

Smallest slice: make only those four pure unit-test trace-reading helpers available wherever their existing unit-test callers compile, preserving all tests/assertions and production feature/API behavior. Add one default `cargo test --locked -p builtins-compiler --no-run` step before workspace feature unification in the existing workspace debug job, so this exact feature gap fails required qualification. No new test, job, harness or algorithm; no relaxation of the test-support whole-package gate.

Owned paths: crates/builtins-compiler/src/lib.rs, .github/workflows/qualification.yml and this spec. Two coherent implementation attempts maximum, each receiving one root adversarial verdict. Stop at compiling/focused-green checkpoint for exact-path root commit. Objective evidence: ordinary and test-support package tests both build/pass; strict package Clippy/fmt; existing workflow routing/test-support coverage policy and mutation gates; full normalized workflow diff shows only the default compile step. No supported-target repetition is needed for cfg(test)-only helpers or workflow. Push evidence, synchronize/close GitHub after PASS, merge through required qualification, and retire closed local spec at the next boundary.

### Attempt-1 scope completion

Removing the four guards exposed a pre-existing hard unreachable_pub error: TEST_ONLY_PAIR_GRAPH_TRACKS is public inside the private test module and reexported only with test-support. Root approves moving this same constant/value to crate root under cfg(any(test, feature = "test-support")), removing its inner declaration/reexport item, and retaining the current test-support public path and all ordinary tests. This is necessary for the same default-build outcome within the already-owned source file; no test is suppressed and no production API/value changes. Existing nonfatal default-only dead-code warnings may be reported without broad warning cleanup.

## Attempt 1 evidence — Worker B, 2026-10-01

Checkpoint `c8a57144`, integrated/pushed as `5bfa1345`. The four pure trace readers now share their existing callers' availability. The fixture track-count constant moves to crate root under cfg(any(test, feature = "test-support")), with the same Backend::current().width() + 1 value and public feature path; ordinary callers continue through super::*. No test body, assertion, production feature or DSP code changed. Rust changes: six lines added/eight removed; workflow: two lines added.

The existing ordinary overlapping-candidate test defends exclusive owner selection and independent fader/matrix state; the invalid-boundary/retry test defends valid-prefix application, queue-tail retention and retry order. Their first/last state assertions remain intact. No tests were added, gated away or deleted. The new default no-run CI step catches feature-availability failures that workspace feature unification masks, before the existing whole-workspace command. Original baseline: twenty E0425s; removing the four guards exposed the additional hard unreachable_pub error, resolved through the approved scope completion without a lint waiver.

Final gates (Cargo build/test commands use CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-b):

- `cargo test --locked -p builtins-compiler --no-run`: PASS.
- `cargo test --locked -p builtins-compiler`: PASS, 44 unit and 10 integration tests; one existing nightly scale ignore, zero doctests.
- `cargo test --locked -p builtins-compiler --features test-support`: PASS, 51 unit and 21 integration tests; two existing nightly scale ignores, zero doctests.
- `cargo clippy --locked -p builtins-compiler --all-targets --features test-support -- -D warnings`: exit 0. Existing clippy.toml warnings name unreachable FAST-DB paths; no code warning under this feature configuration.
- `cargo fmt --all --check` and `git diff --check`: PASS.
- `python3 -B scripts/check-ci-path-routing.py`, `scripts/test-ci-path-routing.py`, `scripts/check-test-support-ci.py` and `scripts/test-test-support-ci.py`: all PASS (same Python invocation for each).
- One-time YAML normalization against the pre-change workflow: removing only the new default compile step makes the complete parsed workflow equal; its position precedes workspace feature unification. Jobs, routes, pinned actions, timeouts and existing feature gates are preserved.

Default tests retain two nonfatal pre-existing dead-code warnings: initial_matrix_state and BoundaryVariant's Nonadjacent/NonadjacentOutputConflict variants. Supported-target repetition, release/codegen work and timing were unnecessary for this test-availability/CI slice. All local gates are complete; root adversarial verdict and required remote qualification remain root-owned.

## Root adversarial verdict: PASS — attempt 1

Root reviewed every source and workflow hunk against the approved scope, all unchanged callers and the ordinary/feature gate evidence. The four readers belong to the existing unit-test module; the root constant keeps its value, visibility under test-support and target-width behavior. Every existing test/assertion survives. The default compile step precedes workspace feature unification and catches the reproduced missing-helper/unreachable-public configuration failures that the existing unified feature build masks. No new or rewritten test needs a distinct-purpose verdict. No production arithmetic, ABI or render behavior changed. PASS covers the local fix and evidence; merge remains subject to unchanged required qualification.
