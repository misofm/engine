# 2173 Nightly: the pack sort's pattern counters overflow at 65,536 strips

## Problem

Since 2026-10-27, the nightly job "release-mode wall-clock budgets" fails. The failing command is:

```
cargo test --locked --release --config profile.release.overflow-checks=true -p graph-compiler --test scale -- --ignored --exact compiles_and_binds_65_537_tracks_with_builtins
```

The test panics with `attempt to add with overflow` at `crates/graph/src/runtime.rs:11926:13`
(the line number at the failing commits). The last green nightly ran on `0f6a1734e` (2026-10-26).
The first red one ran on `3374b21a0`, the first nightly after the #2135 merge.

Failing runs: 41037177443 (`7ef7567bd`), 40853281713 (`723595ed5`) and 40665535237 (`cf22f2a19`).
Each job log shows the same panic at the same line.

## Root cause

The #2135 pattern sort (`pack_sort` in `crates/graph/src/runtime.rs`) is a counting sort. It
keeps one counter for each pattern in `LaneMap::counts`. That field was `Box<[u16]>`. Three values
used `u16`:

- the count of the strips with one pattern (`*count += 1`, now line 11939);
- the prefix sum of the counts (`let mut next = 0_u16`, now line 11943);
- the rank of each strip in its pattern (`*rank += 1`, now line 11958).

A 65,537-track session makes one pack group with more than 65,535 movable strips. All of them have
pattern 0 at bind, so the count of pattern 0 passes `u16::MAX`. With overflow checks on, the test
panics. In a production release build, overflow checks are off. The counter then wraps silently,
and the packer writes wrong targets. That breaks the rule of arbitrary track counts (no
`MAX_TRACKS`).

## Fix

- `LaneMap::counts` is now `Box<[u32]>` (line 10192). The prefix sum `next` is `u32` (line 11943).
  The rank is read with `*rank as usize`.
- `u32` is the type of the sibling `target` and `movable` arrays and of every position in `at` and
  `home`. A count or a rank is at most the group's span. `LaneMap::from_groups` now checks at bind
  that each group's span fits a `u32` (line 10498), so no counter, prefix sum or rank can overflow.
  `usize` would give no more range on wasm32, and on 64-bit it would only double the table.
- The table has `PACK_PATTERNS` = 512 entries. It grows from 1,024 to 2,048 bytes for each plan
  that has a pack group. It is allocated at bind, as before. Render does not allocate.

## Audit

These integers in the swap-to-pack and lane-map code (#1981, #2135, #2142) were checked. None other
can overflow for a strip or lane count:

- `LaneMap::idle` (`u16`): it counts blocks, not strips, and uses `saturating_add`.
- `LaneMap::dwell` (`u16`): at most `PACK_DWELL_BLOCKS` = 16, counted down only when not zero.
- `LaneMap::pattern` (`u16`): at most 9 bits (`PACK_AUDIBLE | bits`), or `PACK_FIXED`.
- `running_bits` (`u16`): at most `PACK_RANKED_SLOTS` = 8 bits.
- `LaneMap::producer_lane` (`u8`): a lane inside one bank (at most 8). `u8::try_from` guards it.
- `swap_scratch` (`u8`): a byte buffer, not a counter.
- `PackGroup::{first, last, width, chains, rows, base, units_base, ranked_base, ranked}` (`u32`):
  each is set with `u32::try_from(..).expect(..)` at bind.
- `PackGroup::displaced` and `PackGroup::dwelling` (`u32`): each is at most the span.
- `LaneMap::{group_of, unit_chain, unit_row, chain_units, at, home, target, movable}` (`u32`): unit
  and position indices, each set with `u32::try_from` at bind.
- `LaneMap::cursor` and `LaneMap::budget` (`u32`): a group index and a bank width.
- `PackBudget` (`usize`): `4 * W`, with `W` at most 8.
- `lane_moves` and `pack_attempts` (`u64`, test only): `saturating_add`.
- `current_location`'s `u16::try_from(lane)`: a lane inside one bank; it falls back to the bind
  location.

## Evidence

- Reproducer: the existing scale test above. It is red on base (the three runs above). It is green
  on head.
- Fast reproducer: `runtime::tests::the_pattern_sort_counts_more_strips_than_a_u16_holds` in
  `crates/graph`. It sorts one group of 65,540 strips with `FakePackHost` and checks every target.
  It runs in about 10 ms, in the per-PR graph tests. The nightly scale test gates nothing, so it
  hid the bug for three days. On the revert of `25319c852`, the test is red in a debug build
  (`attempt to add with overflow` at the count) and in a release build
  (`the first position with a wrong target`, `left: Some(1)`, `right: None`). It is green on head
  in both builds.
- No rendered bit moves for a group of 65,535 strips or fewer. Below the old limit, every `u16`
  value of the old code equals the `u32` value of the new code, so the targets and every move are
  the same.
- The other five 65,537-track commands of the same nightly step pass on head: builtins-compiler
  `scale` (28 s), builtins-compiler `allocation_tracker` (30 s), session `scale_transaction`
  (23 s), protocol `scale` (40 s) and host-core `scale` (100 s), build time included. The
  graph-compiler scale test takes 91 s with its build, 19.2 s for the test.
- One-time comparison (PR evidence, not a committed test): `cargo test --release -p graph
  --features test-support --lib` gives the same 133 passing tests at base `6c1c943da` and at head.
- Checks on head: `cargo fmt --all --check`; clippy on `graph` and `graph-compiler` with
  `--all-targets --all-features -- -D warnings`; `cargo test --release -p graph -p graph-compiler`;
  `scripts/check-workspace-policy.sh`; `realtime_policy`. All pass.
