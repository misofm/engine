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
