# Issue 563 attempt 1 Astra MEDIUM verdict

Verdict: **FAIL**

Reviewed head: `c13d202df07ca875ec1e3bb003bd92dc7f333ba5`

Base: `b20b27d5e3ddc1a1246d003d857ced0bf0cba0c4`

The host-level foreign-thread probe in `crates/host-core/tests/scalar_point_endpoint.rs` captures and prints `foreign_current_thread` but never asserts that all four fields remain zero. The host control therefore does not enforce frozen behavior 4. Add exact equality against `Counters::default()` beside the existing foreign-thread realtime-audit assertions. The separate bench-support unit control correctly asserts this property.

The remainder of the review passed:

- exactly the two permitted source paths changed;
- const TLS contains only a non-dropping `Cell<Counters>`, and updates use bounded wrapping arithmetic without an explicit allocation, lock, recursion, or panic path;
- global counter semantics and original `System` arguments remain intact;
- both preparation contracts remain exact, including the controller's `+2` allocations, `+2` deallocations, `+0` reallocations, and `+9` requested bytes;
- evidence manifests and source hashes authenticate;
- the local issue body matches GitHub; and
- the original failure is preserved as failed run attempt 1 without a rerun.

Residual limitation: the review inspected source and preserved native evidence but did not independently qualify TLS code generation on every target.
