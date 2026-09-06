# Sol attempt 2 review — issue #496

Verdict: PASS for the sole bounded attempt-1 correction group. The production helper and call sites were unchanged from accepted attempt 1. Root checkpointed and pushed the test-only correction as `5a4f4147af8d1c0263128cec0474881b783a6d77`.

## Correction reviewed

Only `crates/builtins/src/lib.rs` changed. The compact cfg(test) fixture now starts every case from an equal stage, independently toggles one active lane, checks the exact cleared bit and the unchanged per-lane predicate's complete mask, toggles the same field back, and checks that the bit returns. Cases cover trim, target, step, integer countdown, and c1/a2/a3/m0/m1/m2 in each of the two sections. The table executes W1, W4 partial/full, and W8 partial/full populations. Widths with another member assert that an unaffected bit stays set. The fixture separately exercises +0/-0 and restoration to +0, and every mask assertion rejects padding bits.

This closes Astra's sole attempt-1 finding: later predicate comparisons are no longer hidden by candidates cleared in earlier cases. No production code, layout, API, DSP arithmetic, mutation campaign, PCM harness, or benchmark changed.

## Identity and records

Final committed HEAD: `5a4f4147af8d1c0263128cec0474881b783a6d77`.

Final `crates/builtins/src/lib.rs` Git blob: `52ffe66d04369d89a47ce240a9a620c1a5c7d5c3`; SHA-256: `c21b54c7147b32601daf82d54d698beca68425019753bf134330ec37e5feb4a9`.

Every actual subprocess wrote combined stdout/stderr directly to `/tmp/496-sol2-<gate>.stdout-stderr`. Its separate `/tmp/496-sol2-<gate>.metadata` records argv, cwd, effective PATH, HEAD, file blob, file SHA-256, exit, and log SHA-256. `/tmp/496-sol2-gates.summary` and `/tmp/496-sol2-exact-gates.summary` contain the aggregate statuses.

The first focused compile exposed missing generic type inference and is preserved, unmodified, in `/tmp/496-sol2-focused-debug.stdout-stderr` with exit 101 and matching metadata. The bounded correction was then made. Pre-checkpoint exact debug/release records are preserved as `focused-debug-final` and `focused-release-final`; each ran one test and passed. All final records below use committed HEAD `5a4f4147`.

## Final committed-source gates

- Each of the four private post-ramp tests ran by full module name with `--exact`, independently in debug and release: eight commands, each exit 0 and exactly one passing test. Record prefixes: `exact-post_ramp_symmetry_mask_matches_lane_oracle-*`, `exact-post_ramp_symmetry_handles_differing_words_countdowns_and_padding-*`, `exact-post_ramp_symmetry_extracts_each_word_once-*`, and `exact-post_ramp_symmetry_helper_is_off_for_settled_blocks-*`.
- `cargo test --locked -p builtins --lib`: exit 0, 7 passed. Release counterpart: exit 0, 7 passed.
- `cargo test --locked -p builtins --test input_liveness --test input_liveness_mono`: exit 0, populations 13 passed and 8 passed. Release counterpart: same populations, exit 0.
- `cargo test --locked -p host-core --test input_liveness_console`: exit 0, 10 passed. Release counterpart: exit 0, 10 passed.
- `cargo clippy --locked -p builtins --all-targets --all-features -- -D warnings`: exit 0.
- `cargo fmt --all -- --check`: exit 0.
- `bash scripts/check-realtime-policy.sh`: exit 0, 42 marked regions in 12 files.
- `bash scripts/check-lane-policy.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: exit 0.

The worktree is clean and matches its pushed branch. Attempt-1's accepted actual 240-versus-30 old-refresh mutation and existing nonzero PCM/state/retarget/disengage evidence remain the applicable evidence; they were not repeated or replaced.
