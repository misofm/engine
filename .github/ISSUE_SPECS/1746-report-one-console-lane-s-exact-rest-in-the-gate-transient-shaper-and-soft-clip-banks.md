# Report one console lane's exact rest in the gate, transient shaper and soft clip banks

Split from *Skip silent lanes' work in console banks* (#1107), D2 item 2. Second of two predicate slices; #1745 is the first.

## Problem

The gate/expander, transient shaper and soft clip have no silent fast path at all, so a console
slot that holds one of them can never skip a lane. #1107 needs every console-eligible effect to
answer "is this lane at its exact fixed point?".

## Root ruling

As in #1745 (#1107 D3): a state-only per-lane predicate whose meaning is that
one zero block leaves the lane's state bits unchanged and outputs exact `+0.0`. No `RestSamples`
bound is needed.

## Smallest closable slice

- Implement the per-lane rest query that #1745 adds to the bank contract, for
  the gate/expander, the transient shaper and the soft clip.
- Add each effect's bank-wide silent fast path as "every member lane is at rest", so whole silent
  banks skip as they do in the EQ, compressor and limiter. The soft clip's shared `pos` cursor
  still advances in a skipped block.

## Authorized paths

- `crates/gate-expander/`, `crates/transient-shaper/`, `crates/soft-clip/` (source and tests)
- this spec

## Objective gates

1. The same randomized predicate test as the first slice, per effect.
2. A whole silent bank renders bit-identically with the fast path and without it.
3. Render allocates nothing (`bench_support::alloc` thread counters after warm-up).
4. `cargo fmt --all --check`, the workspace clippy line, `tools/realtime-policy`.

## Test value

Gate 1 turns red when a predicate claims rest while an envelope, detector or cursor-dependent
word would still change; gate 2 turns red when the new fast path skips work that changes bits.

## Dependencies

- #1745 (EQ, compressor, limiter), which adds the bank-contract query.
- #1376 stays open and independent.

## Decision record

### D1 The claim is decided on demand (attempt 1, 2026-10-23)

Each effect's bank-wide fast path asks `lanes_at_rest()` at the start of every quiet block, after
the block's automation is applied, and skips the kernel when every lane (padded lanes included) is
set. No cached `silent_fixed_point` flag, so there are no withdrawal sites (target, reset, restore,
automation) to miss. The input scan runs first, so a block with signal never evaluates the
predicate. The fast path sits in the driver each bank shares with its scalar instance (gate
`run_block`, shaper `Shaper::process_block`, soft clip `SoftClip::process`), as in the compressor.
Test builds only: an admissions counter and a switch that forbids the fast path.

### D2 Per effect

- **Gate/expander**: per lane and channel, every ramp settled, then one frame of the kernel's own
  `channel_step::<f32, false>` on a scalar copy with a `+0.0` main and detector must leave
  `gain_db`, `open` and `hold` bit-identical, write `+0.0`, and keep `gain_db` inside the D7 bound
  (`finish_block` would reset it). With settled ramps and constant input a block is that frame map
  applied `frames` times, so a one-frame fixed point is a block fixed point; the scalar lane is the
  vector backends' bit oracle. The fixed point is not a closed form (the gain stalls at a
  bit-stable word, the hysteresis may stay open with `hold` reloaded), hence the evaluation.
  Clock: the bank's `ramp_frames_left`, which the skip advances as `run_block` does. The output
  leg is implied by the others (an equivalent mutant) and is kept so the clause is checked.
  Padded lanes are in the bank-wide check, but leaving them out is equivalent at the bank
  interface: a padded lane's output stays `+0.0`, it never swaps (#1747) and never gains a member
  within a plan, so only its internal words would differ (attempt-2 verifier, mutant G8).
- **Transient shaper**: both followers of both channels `+0.0` and every ramp settled. No clocks.
- **Soft clip**: every row of the lane in `x`, `e` and `dry` (all 64 double-written rows) `+0.0`
  and every ramp settled. Clock: each channel's shared `history.pos`, which the skip advances by
  `frames` modulo 32.

### D3 The wasm kernel roster

The roster needs exactly one `f32x4`-arithmetic function per effect. The shaper and soft clip
predicates are bit tests and integer compares. The gate's scalar evaluation is a non-generic
`#[inline(never)]` `kernel::channel_at_rest` (its name has no `f32x4`, and its scalar ops stay out
of `process_bank`'s scalar budget), called from an `#[inline(never)]` `silent_block`. It is scalar
by necessity: a vector form would be a second `gate_expander` `f32x4` symbol (the trait method is
kept alive by the vtable). It runs once per quiet block, `2 * W` calls. The soft clip's
`Channel::process` is pinned `#[inline(never)]` as a precaution; it was already out of line on
main.

## Evidence (attempt 1, 2026-10-23)

- Gate 1 (`rest_tests::a_lane_reported_at_rest_stays_at_rest_through_silence` in each crate, W4
  and W8 on the AVX2 host, `conformance::rest_probe`). Reach, claims / earned / quiet refusals:
  gate W4 6409/4833/12424, W8 16008/11554/21847; shaper W4 21202/4844/50709, W8
  57294/14353/86488 (463 and 1413 of the earned claims after a follower held a non-zero word);
  soft clip W4 7099/6999/2778, W8 14913/14537/5192. The gate plants a NaN gain, the shaper plants
  envelope words, and the soft clip plants a restored `X` row, so each leg is reachable.
- Gate 2 (gate `the_silent_fast_path_renders_the_kernel_bits`, shaper
  `the_silent_fast_path_renders_the_kernels_bits`, soft clip
  `the_silent_fast_path_renders_what_the_kernel_renders`): staggered rests, full and padded, W4
  and W8, fast path allowed against forbidden; every output word, every lane word, the reports and
  the clocks equal after every block; admitted on some silent blocks but not all.
- Gate 3: zero allocations on the fast path: soft clip and shaper `tests/allocation.rs` gain a
  silent stretch; the gate has no allocation test, so `rest_tests::the_silent_fast_path_allocates_nothing`
  counts 256 admitted blocks at `(0, 0)`.
- Mutants, each red: every predicate leg (gate ramps, gain, open, hold, D7; shaper four envelope
  words and ramps; soft clip `x`, `e`, `dry`, ramps, right channel), `== all` to `!= 0` (all
  three), and the skipped clock advance (gate, soft clip). The gate output leg survives (equivalent).
- Gate 4: `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings`, `cargo run --release -p realtime-policy` (171 regions in 34
  files): clean. `cargo test` on the three crates plus `rack` and `effect-compiler`: 212 passed, 0
  failed, 3 ignored.
- Wasm: `build-web-audioworklet.sh` plus `check-web-audioworklet.sh` pass; the roster rows are
  unchanged from main (gate vector 174 scalar 8, shaper 522/0, soft clip 47/0); the V8 spill
  gate, `check-browser-expected-resources.py --artifacts` and `check-scalar-oracle-absent.py` pass.
- One-time base comparison (PR evidence, not committed): a scratch test through each public
  factory (scalar, and W8 banks with 8, 7, 5 and 1 members; 44.1/48/96 kHz; every link mode;
  about 32 s of tone, noise, long `+0.0` silences with `-0.0` blocks, short blocks and automation;
  every output word, every report and every member's payload hashed) gave 105 of 105 equal
  digests on main e9c1dc40c and on this branch (gate 45, shaper 45, soft clip 15). The branch took
  the fast path in 83 of the 84 bank cases. W4 does not bind publicly on this AVX2 host; the
  crate-internal gate 2 tests cover it.

## Evidence (attempt 2, 2026-10-23)

- Verifier (attempt 1): FAIL on test reach in the gate/expander only; predicates, fast paths and
  roster rows sound. Four gate mutants survived: `coef[channel]` -> `coef[0]` in `rest_lanes`,
  `remaining != 0.0` -> `> 1.0` in `channel_at_rest`, and the right-plane and sidechain legs of
  `silent_block` -> `true`. Tests only were changed (no production code).
- Gate 1 (gate): the two channels get different attack, hold and release times, and the probe's
  block is 21 frames, so a 64-sample ramp reaches `remaining == 1` at a block boundary. Reach W4
  1686/978/17286, W8 5189/4353/32636.
- Gate 2 (gate): after rest, one block carries noise on one lane's right plane only (the fast path
  must refuse it), and a connected scalar `PreparedGate<f32, true>` arm renders `+0.0` main under
  a live sidechain after rest, against the forbidden arm. The reload-block comment is now an
  assert that the block was admitted.
- Each of the four mutants is red (rerun by the coordinator for `coef[0]` and the right plane).
- Gate 1 (shaper): schedule `blocks: 800, longest_silence: 300`, the woken-claims count removed
  (gate 2 covers rest from woken followers). Reach W4 5641/1631/13322, W8 13904/3496/23947; its
  five leg mutants stay red; runtime 35 s to about 2 s (debug).
- `cargo test --locked -p gate-expander -p transient-shaper -p soft-clip`: 114 passed, 0 failed,
  3 ignored. `cargo fmt --all --check`, clippy on the three crates and `realtime-policy` (171
  regions in 34 files): clean.
- Verifier (attempt 2): PASS. Root applied its two small fixes: the right channel's hold is
  `hold_ms * 2.0` (with `hold_ms + 1.0` no lane was claimed in the seed state, so gate 1 lost the
  open leg; the verifier showed the open-leg mutant red again at W4 and W8 with this change), and
  gate 2's doc names the connected scalar arm. Stated limit: the `coef[channel]` mutant is red in
  gate 1 at W8 only; the code is generic over the lane width, and x86 runs W8.
