PASS

# "#1448 guard" attempt 1 verdict (stream J3, commit 58cdcaf5f, parent a37144f24)

Spec: the "#1448 guard" section of
`.github/ISSUE_SPECS/1443-check-realtime-regions-with-a-rust-syntax-tree-tool-put-every-non-test-pop-in-a.md`
(root ruling option (C)). I exported `58cdcaf5f` to `/tmp/claude-1002/v1448/tree` and `a37144f24`
to `/tmp/claude-1002/v1448/parent`, and built with `CARGO_TARGET_DIR=/tmp/claude-1002/v1448/target`.
I did not touch the worktree.

## Findings

No BLOCKER, MAJOR or MINOR.

- **NIT-1.** The attempt record's gate 3 gives the refusal line as `lib.rs:3497`. My hand-written
  mode-1 mutant (the same shape: `available = METER_QUEUE_DEPTH` folded by
  `min(available_at_entry() + usize::from(pending.is_some()))`, `passes = available`, no per-queue
  count) is refused at `lib.rs:3491`. The line depends on how the mutant is formatted. The class is
  the same (drain, "bound it with available_at_entry"). Nothing to change.

## What I checked

- **G-D1, no line moves.** The diff of `hosts/host-web/src/lib.rs` is two lines replaced: the
  blank at `:3438` (before `poll_meters`'s doc comment) becomes `// REALTIME_POLICY_BEGIN`, and the
  blank at `:3789` (after its closing brace) becomes `// REALTIME_POLICY_END`. There are no other
  lines in the diff.
- **G-D3, no other change.** The commit changes only `lib.rs`, the region floor in
  `tools/realtime-policy/src/policy.rs` and the spec's attempt record.
- **Gate 1, the tool accepts `poll_meters`.** `cargo run --locked --release -q -p realtime-policy`
  on the guard tree gives exactly one finding, `crates/effect-contract/src/live.rs:366` (expected
  before #1345). Nothing is reported for `hosts/host-web/src/lib.rs`. The tool used is the one on
  the tree, so B1a-D8b and root Amendment 1's macro/attribute and terminal-call rules apply.
- **Gate 2, the parent shape is refused.** I took `98d4a1534^:hosts/host-web/src/lib.rs` and
  replaced its blanks at `:3422` and `:3759` with the markers (the same blank lines that
  6d28a80ec has). The tool refuses
  `hosts/host-web/src/lib.rs:3466: && let Ok(snapshot) = meter.consumer.try_pop()` with the drain
  class. This agrees with the spec's `:3466`.
- **Gate 3, the minimum-bound mutant is refused.** See NIT-1. It is refused with the drain class.
- **Floors equal the measured values.** At 95/26 the tool gives no floor finding. With the floors
  raised to 96/27 (a scratch edit, then reverted), it reports "expected at least 27 marked realtime
  files" and "expected at least 96 marked realtime regions". So the tree has exactly 95 regions
  in 26 files. The file floor correctly stays 26, because `lib.rs` already had a region. The base
  fixture's `BASE_REGIONS = 94` is correct: it is the count of the fixture tree, not the
  workspace's.
- **Gate 4, the worklet module is UNCHANGED.** I built `scripts/build-web-audioworklet.sh
  --module-only` on the parent export `a37144f24`: shipped
  `9784d3509dba6bfd8188b00fc48f9ae4f20020ef9583cbc409cea1d079878d9c` and named twin
  `75fdb18eac4d...`. I also did the full `--named-twin` build at `e6dd440e0`. Its `hosts/`, `crates/` and
  `sdk/src/` are byte-identical to `58cdcaf5f` (`git diff --stat` is empty). It gives the same two
  digests.
- **Gate 5.** These all exit 0: `cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh`
  (ok), `cargo clippy --locked -q -p host-web --all-targets -- -D warnings` and
  `cargo test --locked -p realtime-policy` (186 passed).
- **Does a test pin the 95 floor? No, and none should.** Every case uses `common::policy()`, which
  sets the floors to the fixture's 94/26. `Policy::workspace()`'s 95 is read only by the binary.
  The floor is enforced by `audit-native`, which runs `./target/release/realtime_policy` on the
  real tree. A test that asserts the constant would pin a number and defend no behaviour. AGENTS.md's
  test-value rule and ceremony boundary refuse that kind of test. Lowering the floor is a policy edit
  that review sees, as #1438's Hazards require.

## Test value

The commit adds no test. The two markers are the committed guard. Gates 2 and 3 show that a change
that restores the capacity bound or the minimum form of #1448's poll is refused (reproduced above).
No host-web test can show that on today's thread layout.

## Gates run

Gates 1-5 as above. I also re-measured the floors and rebuilt the parent module. Scratch trees and
targets are deleted.
