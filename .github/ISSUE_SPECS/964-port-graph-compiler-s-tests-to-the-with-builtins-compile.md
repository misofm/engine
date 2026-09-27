# Port graph-compiler's tests to the with-builtins compile

Split from #959 (option (b), owner ruling 2026-09-27). Evidence: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` section 3.2 and `VERIFY.md`.

## Smallest closable slice

While `GraphCompiler::compile` still exists, port every graph-compiler test that uses it to `compile_with_builtins`: the 38 functions in `crates/graph-compiler/src/lib.rs` (the B, P and G groups), `tests/track_delay.rs` and `tests/scale.rs`. Rules for a faithful port:
- **Never weaken a test.** Re-pin a SHA, tail or chain count only with a one-line reason; a test whose subject was the builtins-less path itself is deleted and listed with its reason.
- `tests/scale.rs` keeps 65,537 tracks and moves to `Backend::current()` (needs the linear-compile fix).
- The Output-tail test gives its tracks input sections with no filters, so the builtins tail stays `FiniteZero` (`crates/builtins/src/lib.rs:3360-3371`) and is not re-pinned to `Infinite`.
- Hand-built test plans in `crates/graph` (all its tests, rt1, rt9, rt10), `crates/source` (`lib.rs:3334`, `native_source.rs:3261`, `:3657`) and `crates/builtins-compiler` (`lib.rs:5138`, `:6384`) are **out of scope**: they construct graphs directly rather than compiling, and `graph` cannot dev-depend on graph-compiler without a crate cycle.

## Objective gates

- After the port, no test in `crates/graph-compiler` calls the builtins-less entry.
- The ported suite passes in dev and release; the list of re-pins and deletions is in the evidence.
- Every console workload digest unchanged; fmt, clippy `-D warnings`, doc `-D warnings`, graph policy and determinism scripts.

## Dependencies

The linear-compile fix; #957.

