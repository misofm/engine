# Red mutations for the issue-#93 delay gates

Every test in `crates/miso-engine-delay` (the `tests` module of `src/lib.rs` and
`tests/determinism.rs`) was seen **red** under the mutation named here before it was committed
green, per issue #125's rule and master plan #83 §1.6. Each row is one edit to production code, the
test that caught it, and what the failure looked like. Reproduce by applying the edit and running
`cargo test -p miso-engine-delay --all-targets`.

Mutations are listed in the order the gates appear in the issue-#93 plan's eval table.

| # | eval | mutation (production edit) | test that goes red | observed failure |
|---|---|---|---|---|
