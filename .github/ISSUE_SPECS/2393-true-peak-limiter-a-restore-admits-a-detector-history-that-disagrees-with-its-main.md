# True-peak limiter: a restore admits a detector history that disagrees with its main ring

Successor of #2378 row C2(b), which came from #2324's follow-up. It is filed when #2378's bundle reached its bound.

## Problem
`read_lane` in `crates/true-peak-limiter/src/lib.rs` (~5111) admits every word of a lane's twelve detector `history` taps and every word of its `main_ring`. #2324 chose this so that a live lane's own snapshot stays restorable while it holds a non-finite delay word. The note on `read_lane` says the kernel writes a non-finite history word only beside the same sample in the main ring. More generally, every history tap is a copy of one main-ring word: the history holds `x[n-11 ..= n]`, and the main ring holds the same samples, delayed. A payload whose `history[k]` disagrees with that main-ring word is not one the kernel ever produced. Today it restores. The detector then answers from a history that is not the line's for up to twelve frames. This is bounded, and it is the FX-7 shape: a loose restore.

## Contract
- Establish the exact index relation between `history[k]` and the main-ring word for the same sample. It is a function of `main_cursor`, the ring length and `k`, per lane.
- Prove the relation holds after every writer, not only after the frame loop:
  - both resets (`ResetKind::FullToDefaults`, `DiscontinuityKeepParameters`) and `reset_lane`;
  - the D7 recovery (`reset_failed_lanes` and `clear_runtime`, ~909 and ~4563);
  - the collapse disengage copy (`desymmetrize`, ~4718);
  - a restore, the plan-swap carry and both halves of a lane swap;
  - lookahead automation, which moves a lane's window and end offset but not the ring;
  - start-up, before the ring has been filled once.

  If any writer breaks the relation, either repair that writer (a red-on-base test first) or stop and record why the tie cannot hold.
- Once the relation is proven, `read_lane` refuses with `effect.state.signal` a payload whose `history[k]` is not bit-for-bit the matching main-ring word. A non-finite pair that agrees still restores: #2324's own-snapshot rule is kept.

## Non-goals
- Any change to what the kernel renders.
- The other effects' ring validators.
- The type-level lane-swap pairing (#2392).

## Gates
- Red on base: a forged payload whose `history[k]` differs from its main-ring word restores on base and is refused here.
- `swap_tests::a_lane_holding_a_non_finite_delay_word_restores_and_swaps_exactly` stays green.
- The relation is defended against every writer listed above. Use a randomized own-snapshot round trip that snapshots after every step of a harness driving all of them (#2264's limiter harness is the model) and restores into a twin, all green. Every engine-produced payload must still restore.
- No bit moved: a one-time comparison against base, recorded as PR evidence.
- `cargo test --locked -p true-peak-limiter -p effect-compiler`, clippy `-D warnings`, `cargo fmt --check`, `cargo doc -D warnings` and the realtime policy tool. The restore runs on the render thread during a lane swap, so the new check must be allocation-free and bounded: twelve words per channel.

## Evidence / attempt record

### Attempt 1 (Terra), base `3b3301d02`

**The relation.** History tap `k` is `x[n - k]` (`History::shift`, newest at tap 0). Each frame
reads the main ring at the bank's one cursor, stores the same loaded input word there, and then
advances the cursor. So after frame `n` the main ring holds `x[n - k]` at slot
`(main_cursor + B - 1 - k) mod B`, that is at age `B - 1 - k` from the cursor. `B = Fs/100 + 6`
(447 or more) is greater than 12. The relation is per lane and per channel, and it is the same in
the payload's physical ring order, because `snapshot_lane` writes `cursors.main` and the ring
slot by slot.

**Every writer, re-verified at `3b3301d02`** (lines are `crates/true-peak-limiter/src/lib.rs` at
head):
- The frame loops are the dual per-lane, uniform, linked-uniform, stationary two-pass, glide and
  collapsed bodies. Every one runs `detector_chunk` (3060) over the block's input before any
  output is written. It stores to the main ring the same `L::load` of that input word
  (`channel_frame` 2190, `output_step` 2742, and the uniform and linked frames). It advances one
  cursor pair per frame. **Holds.**
- Both resets (4297, which calls `clear_runtime` 909 and sets the cursors to zero), `reset_lane`
  (4345) and the D7 recovery (`reset_failed_lanes` 4563, whole or per lane through
  `clear_lane_runtime` 951) all write `+0.0` over the lane's whole history and main ring. The
  shared cursor either moves to zero or stays. An all-zero pair is tied at any cursor. **Holds.**
- The collapse disengage (`desymmetrize` 4718, through `copy_state_from` 787) copies the history
  and the main ring together under the bank's one cursor. A collapsed bank's right channel is
  frozen while the cursor moves. A snapshot taken without `desymmetrize_channels` therefore
  carries an untied right section. The contract already forbids that snapshot
  (`effect_contract` `snapshot_track_state_payload` and `desymmetrize_channels`; `swap_bank_lanes`
  says the same). The one engine caller, the plan-swap carry, disengages first. **Holds under the
  existing contract.**
- A restore (`commit_lane` 5279) copies the history verbatim and re-ages the ring from the
  payload's cursor to the receiver's, so age `a` lands at age `a`. The plan-swap carry and both
  halves of `swap_bank_lanes` restore engine payloads. **Holds.**
- A skipped block (`advance_clocks` 4858) moves the cursor without writing the ring. The contract
  skips a bank only whole and only with every lane at rest, where history and ring are all `+0.0`
  (`lanes_at_silent_rest`). **Holds.**
- Start-up: preparation zeroes both. **Holds.**

**Dropped claim: "lookahead automation".** The lookahead is not automatable: its descriptor row
is `AutomationRate::None` (268), and `apply_automation` refuses `parameter >= RAMP_COUNT` (4923).
A lane takes a new lookahead only from `seed_lane_defaults`, at a reset to defaults (which
clears both history and ring), or from a restore (which carries a tied pair). Neither path moves
a ring word. No writer is missing.

**Fix.** `read_lane` (5130) checks the twelve taps after the cursor check. If tap `k` is not
bit-for-bit the main-ring word at slot `(main_cursor + B - 1 - k) mod B`, it refuses with
`effect.state.signal`. The comparison is bitwise, so a non-finite pair that agrees still restores
(#2324). The check costs twelve words per channel, is allocation-free and lock-free, and sits
inside the payload codec's existing `REALTIME_POLICY` region. `parse_track` is shared by restore
and `validate_track_state_payload`, so the swap's validation gives the same verdict. The
`read_lane` note now states the tie and lists every writer. The `plant` note in `rest_tests.rs`
now says a restore can no longer install its stale-history case. Render code is unchanged.

**Tests.**
- `tests::a_history_tap_that_disagrees_with_its_main_ring_word_is_refused` (new, `lib.rs`). It
  starts from the effect's own snapshot, taken after 700 loud frames (cursor off zero, ring
  wrapped), and checks that each of the 12 taps is tied. Then, on both channels, it moves one bit
  of each tap, and separately of its tied ring word. Each forgery must be refused with
  `effect.state.signal`, and the peer must be left unchanged.
  - **Red on base** (base `lib.rs` with only this test added):
    `channel 0 tap 0: a moved history tap was not refused`, `left: Ok(())`,
    `right: Err(StatePayloadError { code: "effect.state.signal" })`.
- `swap_tests::every_writer_leaves_each_lane_a_payload_that_restores` (new). It drives two W4 or
  W8 banks of drawn strips through a random walk of every writer, 4 seeds x 120 steps per width:
  - dual blocks with poison and retargets;
  - explicit D7 recoveries, whole and per lane;
  - collapsed runs, ended by the next writer or by `desymmetrize_channels`;
  - rest, then `advance_clocks`;
  - both resets and `reset_lane`;
  - own restore, a cross-bank carry, and swaps within a bank and across banks.

  After every step outside a collapse, every lane of both banks snapshots. Its payload must
  validate, restore into a twin whose cursor is elsewhere, and restore into itself word for word.
  Reach (head): W4 runs every writer 24-53 times, 85 skipped blocks, 22 whole and 33 per-lane
  recoveries, 20 payloads holding a non-finite word, 3636 payloads. W8 runs every writer 23-53
  times, 98 skipped, 13 whole and 21 per-lane, 183 non-finite, 7160 payloads. About 19 s in debug.
- `swap_tests::a_lane_holding_a_non_finite_delay_word_restores_and_swaps_exactly` stays green.

**Mutations** (each applied alone, then reverted):
1. Tie index off by one (`B - k` for `B - 1 - k`). Red: `every_writer_...` and
   `a_lane_holding_a_non_finite_...`, because own payloads are refused.
2. Only taps `0..6` checked. Red: `a_history_tap_...` (`channel 0 tap 6`).
3. Float `!=` in place of bit comparison. Red: `a_lane_holding_a_non_finite_...` and
   `every_writer_...`, because a NaN pair is refused.
4. `clear_lane_runtime` leaves the history. Red: `every_writer_...`.
5. `commit_lane` does not re-age the main ring to the receiver's cursor. Red: `every_writer_...`.
6. `copy_state_from` (the disengage) skips the history. Red: `every_writer_...`.

**No bit moved** (one-time, not committed). The new walk was instrumented with an FNV-1a fold of
every rendered output word and every snapshotted payload byte. It was run with head `lib.rs`,
then with `origin/main`'s `lib.rs` (`3b3301d02`) under the same test files. The digests are equal:
W4 `8abecf241e422f9c` and W8 `4e20d180e60da733` at both. The walk refuses nothing at base or
head, and no other limiter test moved.

**Gates** (head):
- `cargo test --locked -p true-peak-limiter`: green, including `tests/randomized.rs`, which is the
  conformance randomized harness with #2377's T7 continuation, `tests/conformance.rs` and
  `tests/allocation.rs`.
- `cargo test --locked` for `-p effect-compiler` (including `restore_verdict.rs`), `-p rack`,
  `-p conformance` and `-p graph`: green.
- `cargo clippy --locked -p true-peak-limiter --all-targets -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p true-peak-limiter`: clean.
- `bash scripts/check-workspace-policy.sh`: ok.
- `./target/release/realtime_policy`: ok (298 marked regions in 48 files).
