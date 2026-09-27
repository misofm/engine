# Make the with-builtins compile linear in track count

## Product outcome

`PreparedGraphPlan::with_builtin_banks` (`crates/graph/src/lib.rs:1339-1342`) checks `self.required_bindings.contains(member)`, a linear scan of a `Vec` of about 4 x tracks entries, for every bank member (3 x tracks), so every host's compile is quadratic in track count. Measured by the builtins-less removal verification (release, `Simd8`): 8,192 tracks 3.94 s, 16,384 tracks 9.94 s, 32,768 tracks 37.4 s, 65,537 tracks **158.6 s**; with a set lookup, 16,384 tracks 2.26 s and 65,537 tracks 11.2 s. It stayed hidden because the only scale gate (`crates/graph-compiler/tests/scale.rs`) compiles without builtins at `Backend::Scalar`, where no bank attaches. AGENTS.md: track counts are constrained only by configured resources. Evidence: `docs/handoffs/builtins-less-removal-2026-09-27/VERIFY.md` section 5.

## Smallest closable slice

1. Replace the linear membership test with an ordered-set or sorted-slice lookup built once per call (compile time, off the render thread), with identical results and identical iteration order of everything it produces.
2. Search `crates/graph` and `crates/graph-compiler` for other `Vec::contains` or nested scans over per-track lists on the compile path, and fix any that is quadratic in track count, listing each.
3. Add a with-builtins scale gate: the 65,537-track session of `tests/scale.rs` compiled through `compile_with_builtins` at `Backend::current()`, asserting it completes and binds (a wall-clock bound only if CI can hold one reliably; otherwise record the time).

## Objective gates

- Every compiled artifact byte-identical before and after on the graph and builtins fixtures and every console workload (digests and unit census unchanged).
- The new scale gate passes in release and debug within CI's budget; record both times.
- fmt, clippy `-D warnings`, doc `-D warnings`, `cargo test -p graph -p graph-compiler -p builtins-compiler --features test-support`, graph policy and determinism scripts.

## Dependencies

Lands before the graph-compiler test port (the option (b) split of #959).

