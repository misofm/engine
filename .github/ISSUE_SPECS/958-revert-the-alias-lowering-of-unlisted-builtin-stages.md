# Revert the alias lowering of unlisted builtin stages

Scoping study: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` (option (a), owner-directed removal, 2026-09-27).

## Amendments (adversarial verification, 2026-09-27; override conflicting text)

1. **Authorized paths:** `crates/graph-compiler/src/compile.rs`, `crates/graph/src/program.rs`,
   `crates/graph/src/program/tests.rs`, `crates/graph/src/lib.rs` (the `lower` call only), the
   #925 tests in `crates/graph` and `crates/graph-compiler`, `crates/graph/tests/MUTATIONS.md`,
   and this spec.
2. Simulating the revert turns red exactly: the two #925 tests, the B test and 2 of the 4 P tests.
   It moves no fixture byte. Evidence: `docs/handoffs/builtins-less-removal-2026-09-27/VERIFY.md`.
3. **Order:** land after #964 (the graph-compiler test port), so the same tests are not churned
   twice.

## Rulings (coordinator, 2026-09-27)

The owner directed the complete removal of the builtins-less path ("I don't think we should be
benchmarking something that never gets used in the real world"). Option (b) is taken, by the owner's
second ruling ("We should remove everything related to a builtins-less compile because a
builtins-less compile is never needed in production"): the builtins-less compile is removed
entirely, including any test-only entry point, builtins become mandatory in the graph compiler,
and every test that compiled without builtins is ported to `compile_with_builtins`. The optimisation batch lands unchanged and this work deletes #937's
code afterwards. The wasm console arm is re-indexed. `sixty_four_track_gain_pan_only` is the
pure-audio-path target; its fused twin follows. #938 is re-based onto the gain/pan session. The
phase-profile harness is retargeted to `gain_pan_only`. Ported tools compile at
`Backend::current()`.

## Smallest closable slice

- **S1. Revert #925's alias arm.**
  - `compile.rs:803-835` lists the three builtin stages unconditionally.
  - `program::lower` loses `bindable` (`program.rs:544-581`, `:649-664`) and `is_builtin_stage`.
  - Delete the #925 tests (study §3.2 and §3.1), and drop the `builtins_bound` argument at its 28
    call sites.
  - The graph-compiler B test and the P tests' builtins-less arms go back to the identity-bound
    shape or are deleted.
  - Gate: every with-builtins program is unchanged. The predicate already reduces to
    `is_alias_candidate` there.

## Dependencies

#957.

## Standing rules for the implementer

- Class A for every with-builtins plan: every program, digest and unit census unchanged is a hard stop.
- Commit on `codex/<issue>-<slug>`. Do not run timed benchmarks.

