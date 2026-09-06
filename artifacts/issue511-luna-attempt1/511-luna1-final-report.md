# #511 Luna attempt 1 immutable final validation

Validation ran from immutable source HEAD `f8215094d5f6e4cf74466a3111aaeccf29858c10` in `/home/bl/misofm/engine-slot-reservation`.

Source identity captured before runs is in `/tmp/511-luna1-final-identity.txt`. The six changed source/test blobs were hashed there; the worktree was clean at the captured HEAD.

## Results

| Gate | Result | Count/status | Raw records |
| --- | --- | --- | --- |
| `cargo test --locked -p graph-compiler --lib` | FAIL | 60 passed, 4 failed, exit 101 | `/tmp/511-luna1-final-graph-debug.stdout`, `/tmp/511-luna1-final-graph-debug.stderr`, `.meta` |
| `cargo test --locked --release -p graph-compiler --lib` | FAIL | 60 passed, 4 failed, exit 101 | `/tmp/511-luna1-final-graph-release.stdout`, `/tmp/511-luna1-final-graph-release.stderr`, `.meta` |
| allocator tracker debug with `test-support,graph/test-support` | PASS | 7 passed, 0 failed, exit 0 | `/tmp/511-luna1-final-physical-debug.stdout`, `/tmp/511-luna1-final-physical-debug.stderr`, `.meta` |
| allocator tracker release with `test-support,graph/test-support` | PASS | 7 passed, 0 failed, exit 0 | `/tmp/511-luna1-final-physical-release.stdout`, `/tmp/511-luna1-final-physical-release.stderr`, `.meta` |
| `cargo clippy --locked -p graph --all-targets --all-features -- -D warnings` | PASS | exit 0 | `/tmp/511-luna1-final-clippy-graph.stdout`, `/tmp/511-luna1-final-clippy-graph.stderr`, `.meta` |
| `cargo clippy --locked -p graph-compiler --all-targets --all-features -- -D warnings` | FAIL | 6 errors, exit 101 | `/tmp/511-luna1-final-clippy-graph-compiler.stdout`, `/tmp/511-luna1-final-clippy-graph-compiler.stderr`, `.meta` |

The four graph-suite failures are existing numeric expectation mirrors whose values are now lower than the admitted estimate by the new slot reservation delta. They are `launch_multiband_compressor_fixture_closes_bank_graph_and_transactional_caps`, `launch_transient_shaper_fixture_closes_banks_tails_pdc_and_transactional_caps`, `launch_soft_clip_fixture_closes_banks_tails_pdc_support_and_transactional_caps`, and `launch_true_peak_limiter_fixture_retains_banks_tails_latency_and_transactional_caps`. No automatic repin was performed.

Clippy’s six errors are `inconsistent_digit_grouping` on the new test plan IDs `511_0` through `511_5`; the compiler suggests `5_110` through `5_115`. This is a source correction required by the strict gate, so validation stopped as instructed.

## Bounded correction authorized by root

After all live validation commands were terminal, root authorized the mechanical correction of those six test-only literals. The corrected source is currently dirty relative to checkpoint `f8215094`; its strict graph-compiler Clippy rerun passed with exit 0. Raw records are `/tmp/511-luna1-final-clippy-graph-compiler-correction.stdout`, `/tmp/511-luna1-final-clippy-graph-compiler-correction.stderr`, and `.meta`; the `.meta` records the post-correction source hashes and immutable base HEAD.

## Prior focused evidence

The four frozen focused tests passed before this immutable run in debug/release, with nonzero filtered counts. Raw records are `/tmp/511-luna1-graph-test5.log`, `/tmp/511-luna1-graph-release.log`, `/tmp/511-luna1-physical-test4.log`, and `/tmp/511-luna1-physical-release.log`, with corresponding `.meta` files.

The physical test proves the same production `bank_chain` conversion with prepared inputs outside attribution, actual requested stage and runtime slot capacities, retained ownership through snapshot, and separate off-render release. It also exercises the existing actual bound graph and successful fused builtin path. It does not expose a typed membership-count/slot-count witness for that opaque bound graph, so it must not be credited as a complete direct proof of every actual-bound multiple-slot/unpaired inequality. The helper-only component proof is intentionally reported as limited evidence.

`cargo fmt --all` and `git diff --check` passed on the candidate source before checkpoint; their records are `/tmp/511-luna1-fmt.meta` and `/tmp/511-luna1-diff-check.meta`. Final graph/realtime/workspace policy runs and builtins-compiler Clippy were not run after the strict graph-compiler Clippy failure, per the stop-on-source-correction instruction.
