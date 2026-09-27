# Delete the builtins-less compile entry

Scoping study: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` (option (a), owner-directed removal, 2026-09-27).

## Amendments (adversarial verification, 2026-09-27): rescoped

This issue is now slice (iii) only: **delete the builtins-less compile entry and its `Option`
arms**, after #962 (linear compile), #963 (tools and audits) and #964 (graph-compiler tests)
have removed every caller. Steps 2 and 3 of the slice below moved to #964 and #963.
- **Gate 1 is replaced** (the old pattern matched six unrelated functions):
  `rg -nP 'GraphCompiler::compile(?!_with_builtins)|GraphCompileRequest|PreparedGraphArtifact\b' crates tools hosts`
  finds nothing, and `rg 'PreparedGraphPlan::new' tools` finds nothing.
- Hand-built plans in `crates/graph`, `crates/source` and `crates/builtins-compiler` tests are
  graph constructions, not compiles, and are out of scope (crate cycle).
- **Flagged for the owner, out of scope:** `PreparedGraphPlan::new` plus `bind` stays public (the
  compilers need it), and `Backend::Scalar` still produces bankless with-builtins plans (host-core
  tests, the CI scalar wasm compile, scalar-only split-pair code).

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

## Smallest closable slice (option (b), owner ruling 2026-09-27)

1. **Delete the builtins-less compile.** Remove `GraphCompiler::compile` (the entry that takes no
   prepared builtins) and every `Option` arm that exists only for it in
   `crates/graph-compiler/src/compile.rs` (the study cites `:449`, `:654`, `:679`); builtins become
   a required input of the one remaining compile entry. No `#[cfg(test)]` or feature-gated
   builtins-less entry may remain anywhere.
2. **Port every test that compiled without builtins** to `compile_with_builtins`, in
   `crates/graph-compiler` (the study's 47 functions: the B, P and G groups, `tests/track_delay.rs`,
   `tests/scale.rs`) and anywhere else the grep in gate 1 finds. Re-pin SHAs, tails and chain counts
   that move, each with a one-line reason; never delete a test to make the port easier. A test
   whose subject was the builtins-less path itself is deleted and listed.
3. **Port the tools** (`graph_fixture`, the #650 audit, the #006 compile benchmark) to
   `compile_with_builtins` at `Backend::current()`, and regenerate `fixtures/graph/v1/*` and its
   manifest in the same change, coordinated with #947.
4. **Docs.** Update every doc and ruling that describes a builtins-less compile as available.

## Objective gates

1. `rg -n 'GraphCompiler::compile\(|fn compile\(' crates tools hosts` finds only
   `compile_with_builtins` (or its renamed successor) and no builtins-less entry.
2. Every ported test passes; the list of deleted tests is in the evidence with a reason each.
3. Every console workload digest and unit census unchanged; the regenerated fixtures pass #947's gate.
4. fmt, clippy `-D warnings`, doc `-D warnings`, `cargo test --workspace` (or the CI debug and release
   test jobs' commands), the graph policy, determinism, realtime and wasm-gate scripts.

## Dependencies

#958; #947.

## Standing rules for the implementer

- No product behaviour changes; every console digest unchanged is a hard stop.
- Commit on `codex/<issue>-<slug>`. Do not run timed benchmarks.

