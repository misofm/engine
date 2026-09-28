# Make the graph resource fixtures cover both shipped bank widths

## Product outcome

#963 ported `graph_fixture` to `compile_with_builtins` at `Backend::current()` and added a regenerate-and-compare test for `fixtures/graph/v1/*`. The resource fixture now records bank-width-dependent figures (builtin-bank charges), so it is correct only on a host whose `Backend::current()` is `Simd8`: on AArch64 (iOS, Android) or any `Simd4` or scalar build the compare test fails and tells the developer to regenerate. `tools/audit/src/prepared_effect_allocations.rs:540` also hard-codes a divide by 8. Found by the #963 verification (low finding 2).

## Smallest closable slice

1. Generate the width-dependent fixtures for both shipped bank widths explicitly, `Simd4` (the browser) and `Simd8` (native x86), whatever the host, and compare each against its own file. No fixture may depend on the build host's width.
2. Replace the hard-coded `/8` with the bank width of the backend being audited.
3. Fix the comment at `tools/graph_fixture` (`graph_fixture.rs:154-156`) that names the wrong estimate for the canonical text.

## Objective gates

- The compare test passes when run with `Backend::current()` forced to either width (or on a `Simd4` target), and a mutation in the builtin-bank charge turns it red at both widths.
- `scripts/check-graph-determinism.sh` passes; fmt, clippy `-D warnings`.
