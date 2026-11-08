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
(empty)
