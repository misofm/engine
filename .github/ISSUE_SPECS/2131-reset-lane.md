# Reset one lane of a bound effect bank (`reset_lane`)

Prerequisite of #2129 S1b (bypass skip with per-lane staleness) and of the banking design's
packing and placement slices. Class A: the method has no caller in this issue, so no rendered bit
moves.

## Why (root, 2026-10-26)

A design review of #2129 found a BLOCKER: under #2129's first D2/D3, a lane is reset at un-bypass
only when its whole bank skipped, so a lane's bits after un-bypass depend on its bank mates
(AGENTS.md: banking couples cost, never bits). Root's amended rule (it will be recorded in #2129):
staleness is a per-lane fact, and every un-bypass from settled bypass resets that lane alone. A
whole-bank `reset` cannot do that when the bank's other lanes are running, so the bank trait needs
a per-lane reset. The owner was told on 2026-10-26 that an effect turned back on "starts from rest".

## Product contract

1. `PreparedNativeEffectBank` gains a **required** method (no default, so a new banking effect
   cannot omit it): `fn reset_lane(&mut self, lane: usize, kind: ResetKind)`.
2. For `lane < width` and both `ResetKind` values: after `reset_lane(i, k)`, lane `i`'s state on
   both channels is exactly the state lane `i` would have after the whole-bank `reset(k)`, and
   every other lane's state is untouched. So every later output word of every other lane is
   bit-identical to a run without the call, and lane `i`'s later words are bit-identical to a run
   where the whole bank was reset at that block and only lane `i` is compared.
3. The bank's shared bookkeeping stays true: rest claims (`lanes_at_rest`), fast-path or elision
   flags (e.g. the EQ's `ramping_elided` and `select_free`), silence counters, and shared cursors
   (the limiter's, which per-lane D7 recovery already leaves in place).
4. Realtime: allocation-free, lock-free and syscall-free, and bounded by one lane's state size. It is
   callable on the render thread at a block boundary.
5. Implemented for every effect that binds a bank today: gate-expander, de-esser, dynamic-eq,
   soft-clip, true-peak-limiter, transient-shaper, compressor, parametric-eq and
   multiband-compressor. The delay never banks. If the reverb (#2114) banks when it lands, the
   PR that lands second adds its implementation.

Implementer's survey (2026-10-26, on #2129's branch). Every bank is AoSoA lane-strided.
- **Helpers already exist:** the gate, de-esser and dynamic-eq (`reset_lane_full` and
  `reset_lane_discontinuity`), and the soft-clip (`reset_lane_discontinuity`; full needs a small
  addition).
- **Easy:** the limiter (`reset_lane_to_defaults`, `clear_lane_runtime`, plus per-lane ramp snaps)
  and the transient-shaper (`recover_lanes`, plus a ramp snap and default reseed).
- **Moderate:** the compressor needs per-lane versions of `full_reset` and `discontinuity_reset`.
  The parametric-eq needs the lane's section ramps, designed targets and the bank's fast-path
  bookkeeping. The multiband needs the crossover, smoothers, silence counters, glide and band
  envelopes.

## Gates (one test per claim, red first on a named mutant)

1. **One conformance test** over every banking effect, in the crate that already hosts the
   cross-effect bank conformance (the implementer names it; do not add a new crate). The setup:
   a full bank of distinct non-trivial signals and parameters per lane, several blocks, then
   `reset_lane(i, k)` for each `k` and for a middle and an edge lane, then more blocks. The test
   asserts claim 2 bit-exactly against the two reference runs. Red first: for each effect, one
   mutant that forgets one piece of lane state, plus one mutant that touches a neighbour lane.
   Record which mutant turns which row red.
2. **Allocation.** `reset_lane` under `bench_support::alloc`'s thread-scoped counter equals 0,
   inside the same conformance test.
3. The realtime-policy tool and clippy are clean. `scripts/check-workspace-policy.sh` passes.

No timing gate: there is no caller yet.

## Paths

`crates/effect-contract/src/lib.rs` (the trait). Each of the nine effect crates' bank modules.
The conformance test file.

## Overlap

- #2129 branch `codex/batch-b13-bypassskip` also edits `crates/effect-contract/src/lib.rs`
  (`warmup_samples`, the bypass clause, `ObservationDescriptor.bypass_observation`). This issue
  lands first, and #2129 S1b merges main.
- #1981 (PR #2130) is in rack and graph only, so it does not overlap.
- The reverb #2114 is a new crate; see contract item 5.
- The module bytes change, so the release-window rule applies.
