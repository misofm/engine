# Put a limiter bank back on its uniform body after a partial recovery

Follow-up to *Pad true-peak limiter banks* (#1091; Sol verdict L1, commit `592741a7`). This is a
performance issue: it moves no rendered bit.

## Problem

Since #1091 a true-peak limiter bank recovers from a failing block (D7) one lane at a time. The
recovered lane's van Herk window restarts at phase 0, while its bank-mates keep their phase. The
bank then fails its uniform-body test (`lanes_uniform` in `crates/true-peak-limiter/src/lib.rs`,
phase leg; taken in `limiter_block` and the collapsed body)
and runs the per-lane body until a whole reset: `reset()`, every member failing in the same
block, or a re-preparation. In steady playback that is indefinitely.

- **Cost.** The per-lane body costs 2.6x the uniform one at `Simd8` and 2.0x at `Simd4`
  (`docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS.md:122`).
- **Reachability.** Low. The limiter's gain is at most 1, so legal input cannot reach its D7. Only
  a non-finite state or a corrupted restore can.
- **The bits are already right.** Sol's #1091 probe ran 1,128 scenarios with 18,154 slow-body
  blocks. Every member matched its per-node twin and the untripped bank.

## Smallest closable slice

After a partial recovery, give each recovered lane the window phase of a surviving lane that has
the same window. The bank then passes its uniform-body test again from the next block.

This is bit-neutral: every required gain lies in `(0, 1]`, so the window minimum is exact whatever
the block phase. Sol's probe ran this one-loop change and found 0 mismatches against the twins.

Authorized paths: `crates/true-peak-limiter/src/lib.rs` and its tests, and this spec.

## Objective gates

1. After a partial recovery with at least one survivor, the bank runs the uniform body from the
   next block. A test observes the body selection, and it goes red without the realignment.
2. Class A: every member's output after the recovery is bit-identical to its per-node twin and to
   the untripped bank. Cover W = 4 and 8, both link modes, dual and collapsed, and NaN and `+inf`
   trips.
3. When every member fails in one block, the behaviour is unchanged from #1091.
4. The console digests are unchanged, render allocates nothing, and the limiter audit and the
   realtime policy pass.

## Non-goals

- No change to D7 detection or reporting.
- No benchmark: the cost ratio above is the evidence.

## Decision and evidence record (attempt 1, 2026-10-24)

- Survivor rule (root, from the scope note): per channel independently, each failed lane takes the
  phase of the lowest lane outside `failed` with the same window; padded lanes count as survivors;
  no survivor keeps phase 0. Per channel because the collapsed body advances only the left channel
  (#990). Shape: `ChannelState::align_phase_to_survivor`, called by `LimiterCore::reset_failed_lanes`
  after the per-lane reset loop.
- Bit-neutral: a cleared lane's `required_ring` and `prefix` are all `1.0`, which is the state of
  every phase, and required gains lie in `(0, 1]`. Gate 2 is the existing `PaddedRun::render` twin
  comparison (W4/W8, both links, dual and collapsed); `plant_nonfinite` now plants NaN and `+inf`
  by member parity.
- Gate 1: a route assertion (`DualUniform` / `MonoUniform`) on every "after" block of
  `a_failed_lane_is_recovered_and_reported_alone_at`, and after a padded lane's recovery. Red on the
  reverted fix (align calls commented out): `left: DualPerLane, right: DualUniform`. No new test.
