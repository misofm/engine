# Remove the dead code builtins-compiler reports under no-features clippy

Stream J of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0).
Code anchors verified on `main` at `6fb211594`.

## Mission

`builtins-compiler`'s lib test target does not pass clippy without features: two test helpers are
compiled under `cfg(test)` but used only by tests compiled under
`cfg(all(test, feature = "test-support"))`. Gate the helpers exactly as their users are gated, so the
crate's tests lint clean with and without `test-support`.

## The defect (found by the #1220 attempt 1 verdict, MINOR-3)

The failure reproduces on `main` at `6fb211594` (it was first seen at `0268a1c74`, before batch
K3): the command exits 101 with the two errors below.

```
cargo clippy --locked -p graph -p builtins-compiler --all-targets -- -D warnings
```

fails with two `dead_code` errors in `builtins-compiler`'s lib test target:

- `fn initial_matrix_state` (`crates/builtins-compiler/src/lib.rs:1004`, `#[cfg(test)]`) is never
  used. Its callers (`:8243`, `:8380`, `:8477`) sit in tests gated
  `#[cfg(all(test, feature = "test-support"))]`. Its writer `record_initial_matrix_state` (`:981`)
  and the thread-local `SCALAR_INITIAL_MATRIX_TRACE` (`:904-906`) share its `cfg(test)` gate.
- `BoundaryVariant::{Nonadjacent, NonadjacentOutputConflict}` (`:6075`, `:6077`) are never
  constructed: their constructors (`:7837`, `:7864`, `:8023`, `:8057`, `:8084`, `:8145`, `:8169`)
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
   any type or constant used only by them (`TestOnlyInitialMatrixTrace`
   at `:869-871`, `EMPTY_INITIAL_MATRIX_TRACE` at `:889-890`, if so) and their call site at `:2315-2317` with
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

## Attempt record

### Attempt 1 (base `9edd1a6b8` on `codex/d15-stream-j`)

Change, `crates/builtins-compiler/src/lib.rs` only:

- Deliverable 1: `TestOnlyInitialMatrixTrace`, `EMPTY_INITIAL_MATRIX_TRACE`, the
  `SCALAR_INITIAL_MATRIX_TRACE` thread-local, `record_initial_matrix_state`, `initial_matrix_state`
  and the `record_initial_matrix_state` call site in `strip_bindings` move from `cfg(test)` to
  `cfg(all(test, feature = "test-support"))`, the gate of their three callers.
- Deliverable 2: `BoundaryVariant::{Nonadjacent, NonadjacentOutputConflict}` are gated
  `cfg(all(test, feature = "test-support"))`, the gate of every test that constructs them
  (`NonadjacentTrackA` stays ungated: ungated tests construct it). Four `matches!` uses named the
  gated variants inside or-patterns, where a `cfg` attribute cannot go; each became the equivalent
  `match` whose gated variants sit in their own `cfg`-attributed arm. Every arm returns what the
  `matches!` form returned, so with the feature on the four expressions are the same functions of
  `variant`, and with it off the arms for absent variants are absent.
- Nothing is silenced: no `allow`/`expect(dead_code)` added. No production code moved (the one
  production-file site is a `cfg(test)` statement whose gate narrowed).
- No new or rewritten tests, so no test-value answer or mutation run is owed; the defect's own
  reproducer is gate 1, red on base and green on head.

Gates (host heavily loaded; times not recorded):

| Gate | Base `9edd1a6b8` | Head |
| --- | --- | --- |
| 1. `cargo clippy --locked -p graph -p builtins-compiler --all-targets -- -D warnings` | exit 101: `function initial_matrix_state is never used` (`lib.rs:1004`), `variants Nonadjacent and NonadjacentOutputConflict are never constructed` (`lib.rs:6075`) | exit 0 (only the two out-of-scope `clippy.toml` fast-dB warnings) |
| also `cargo clippy --locked -p builtins-compiler --all-targets --features test-support -- -D warnings` | -- | exit 0 |
| 2. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | -- | exit 0 |
| 3a. `cargo test --locked -p builtins-compiler --features test-support` | exit 0; 76 passed, 0 failed, 2 ignored (lib 54) | exit 0; 76 passed, 0 failed, 2 ignored; sorted test-name list identical to base |
| 3b. `cargo test --locked -p builtins-compiler` | exit 0; 57 passed, 0 failed, 1 ignored | exit 0; 57 passed, 0 failed, 1 ignored; sorted test-name list identical to base |
| 4. `cargo fmt --all -- --check`; `bash scripts/check-workspace-policy.sh` | -- | both exit 0 |
