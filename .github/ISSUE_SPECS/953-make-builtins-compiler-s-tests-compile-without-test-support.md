# Make builtins-compiler's tests compile without test-support

## Product outcome

`cargo test -p builtins-compiler` without `--features test-support` fails to compile, already on `main` (`14f2917b`), because tests reach test-only APIs that the feature gates. CI only ever runs the crate with the feature, so nothing notices. A crate whose test target does not build in its default configuration is a trap for every implementer who runs the obvious command.

## Smallest closable slice

Either gate the affected tests behind `#[cfg(feature = "test-support")]` (or the crate's `[[test]] required-features`), or enable the feature for the crate's own tests through a `[dev-dependencies]` self-reference, whichever the crate's existing pattern prefers. Then make CI build the crate's tests in the default configuration once (`cargo test -p builtins-compiler --no-run`).

## Objective gates

- `cargo test -p builtins-compiler` and `cargo test -p builtins-compiler --features test-support` both build and pass.
- The CI job that builds workspace tests covers the default configuration of this crate.

Found by the #944 implementer and confirmed pre-existing by its verification.
