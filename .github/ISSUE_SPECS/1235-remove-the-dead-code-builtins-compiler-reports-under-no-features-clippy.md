# Remove the dead code builtins-compiler reports under no-features clippy

## Mission

`builtins-compiler`'s lib test target does not pass clippy without features: two test helpers are
compiled under `cfg(test)` but used only by tests compiled under
`cfg(all(test, feature = "test-support"))`. Gate the helpers exactly as their users are gated, so the
crate's tests lint clean with and without `test-support`.

## The defect (found by the #1220 attempt 1 verdict, MINOR-3)

Every anchor below is verified on the batch K3 follow-up tree (branch `codex/batch-submix-k3`); the
failure is identical at `0268a1c74`, so it predates batch K3.

```
cargo clippy --locked -p graph -p builtins-compiler --all-targets -- -D warnings
```

fails with two `dead_code` errors in `builtins-compiler`'s lib test target:

- `fn initial_matrix_state` (`crates/builtins-compiler/src/lib.rs:911`, `#[cfg(test)]`) is never
  used. Its callers (`:7924`, `:8061`, `:8158`) sit in tests gated
  `#[cfg(all(test, feature = "test-support"))]`. Its writer `record_initial_matrix_state` (`:888`)
  and the thread-local `SCALAR_INITIAL_MATRIX_TRACE` (`:811-813`) share its `cfg(test)` gate.
- `BoundaryVariant::{Nonadjacent, NonadjacentOutputConflict}` (`:5876-5878`) are never
  constructed: their constructors (`:7518`, `:7545`, `:7704`, `:7738`, `:7765`, `:7826`, `:7850`)
  are in `test-support`-gated tests too.

CI does not see it: `qualification.yml`'s lint job runs `cargo clippy --locked --workspace
--all-targets --all-features -- -D warnings`, which enables `test-support`.

## Invariants

- No production code, no test, and no test's behaviour changes: only `cfg` attributes move (or an
  item becomes `cfg`-gated with its users). Every test that runs today under `--all-features` still
  runs, unchanged.
- Nothing is silenced: no `#[allow(dead_code)]` and no `#[expect(dead_code)]`.

## Deliverables

1. Gate `initial_matrix_state`, `record_initial_matrix_state`, `SCALAR_INITIAL_MATRIX_TRACE` and
   any type or constant used only by them (`TestOnlyInitialMatrixTrace`,
   `EMPTY_INITIAL_MATRIX_TRACE`, if so) and their call site at `:2201` with
   `cfg(all(test, feature = "test-support"))`, matching their users.
2. Gate the two `BoundaryVariant` variants, or the match arms that name them, the same way, so the
   enum's variant set equals what the compiled tests construct. Prefer the smallest edit that keeps
   the exhaustive matches compiling in both configurations.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (`cfg` attributes on the items above and their uses only).
- This spec's own record sections.

## Non-goals

- The two `clippy.toml` warnings (`math::fast_db::fast_level_db` and `fast_gain_from_db` "does not
  refer to a reachable function") that the same command prints for crates that do not depend on
  `math`'s fast-dB tier: they are warnings, not errors, and are not this issue.
- No other crate, and no change to CI wiring.

## Objective gates

1. `cargo clippy --locked -p graph -p builtins-compiler --all-targets -- -D warnings` exits 0.
2. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` exits 0.
3. `cargo test --locked -p builtins-compiler --features test-support` passes with the same test
   count as the base, and `cargo test --locked -p builtins-compiler` (no features) passes.
4. `cargo fmt --all -- --check` and `bash scripts/check-workspace-policy.sh` pass.

## Evidence

- Each gate's command, exit status and test counts, base and head.

## Dependencies

- None.
