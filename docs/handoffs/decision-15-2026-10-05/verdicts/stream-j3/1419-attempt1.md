PASS

# #1419 *Use the shared StopOnDrop in the C ABI plan-swap race test*: verdict, attempt 1

Commit `95cb7bad5` on `codex/d15-stream-j3` (parent `7290d6b92`), reviewed as
`git diff 7290d6b92 95cb7bad5` against the spec, AGENTS.md, decision 15 and the owner principle. I
exported `08b5f478b` (which contains `95cb7bad5`; `plan_swap_race.rs` and `resource_lifecycle.rs` are
identical in both) to `/tmp/claude-1002/v1419/tree` and built with
`CARGO_TARGET_DIR=/tmp/claude-1002/v1419/target`. I did not write to the worktree.

No BLOCKER. No MAJOR. No MINOR. No NIT.

## Scope

- D1: the private `StopOnDrop` and its doc comment are deleted from `crates/capi/tests/plan_swap_race.rs`;
  the import is `bench_support::producer::{StopOnDrop, render_while_producing}` (`:16`). The use
  (`:504`, `let stop_readers = StopOnDrop(&stop);`) and the `drop(stop_readers)` (`:534`) are unchanged.
  The reason moved to a comment at the use site (`:502-503`). The shared guard
  (`tools/bench-support/src/producer.rs:34`) has the same body (`store(true, Release)`) and a public
  field, so the tuple constructor compiles unchanged. The binding is named (not `let _ = ...`), as the
  shared guard's doc requires.
- D2: `crates/capi/tests/resource_lifecycle.rs:2069` now ends "`plan_swap_race.rs` uses the shared
  guard."; the rest of the comment and the private copy (justified by #1251 D4) are unchanged.
- Only the three authorized paths changed. No other private copy was touched
  (`observation_transport.rs:110`, `spsc.rs:1493` are the justified ones).
- The attempt record's re-measured parent lines are correct: guard `:256-263`, use `:512`, drop `:541`,
  `resource_lifecycle.rs` comment `:2065-2069`.

## Gate 2 reproduced (planted failure)

- Planted `assert!(block < 3, "planted failure at block {block}");` before `rendered.store(block + 1, ..)`
  in the render closure of `resource_queries_racing_plan_swaps_always_find_the_published_row`.
  `cargo test --locked -p capi --test plan_swap_race resource_queries_racing`: exit 101, panic
  "planted failure at block 3", wall 0.17 s (test binary already built). No hang.
- Control: the same planted failure with the guard binding replaced by `let stop_readers = ();`:
  `timeout 40` expired (exit 124). So the shared guard is what releases the reader threads. Both
  mutations reverted; the file was compared byte for byte with the commit after the restore. No hung
  test process remained.

*Test value.* No new test (spec). Gate 2 shows the shared guard keeps the fail-instead-of-hang behaviour
#1251 gave the private copy; the control run shows the guard, not something else, ends the test.

## Gates run (at the commit's tree)

| Gate | Result |
|---|---|
| `cargo test --locked -p capi --test plan_swap_race` | exit 0, 3 passed |
| Gate 2 planted failure / control without guard | exit 101 in 0.17 s / exit 124 at 40 s |
| `cargo test --locked -p capi --test resource_lifecycle` | exit 0, 11 passed |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --locked -p capi --all-targets --all-features -- -D warnings` | exit 0 |
| `bash scripts/check-workspace-policy.sh` | exit 0 ("workspace policy: ok") |
