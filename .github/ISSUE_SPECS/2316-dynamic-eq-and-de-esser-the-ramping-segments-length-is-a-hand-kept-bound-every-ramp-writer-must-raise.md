# Dynamic EQ and de-esser: the ramping segment's length is a hand-kept bound every ramp writer must raise

Tracker: #2258, item FX-5. Found by the implicit-state-machine audit of 2026-11-06.

## Problem
`DynamicEq` (`crates/dynamic-eq/src/lib.rs`) and `DeEsser` (`crates/de-esser/src/lib.rs`) each kept a
bank-level `ramp_frames_left: u32` that had to stay at least every lane's ramp `remaining` (parameter
ramps and detector-link weights, both channels). `run_block` splits each block at
`min(ramp_frames_left, frames)`: the prefix runs the `RAMPING` segment, which advances the ramps, and
the rest runs the settled segment, which does not. The bound was a derived cache kept by hand: every
writer that starts a ramp had to raise it (`retarget`, `retarget_link`, `restore_lane`), `run_block`
and `advance_clocks` lowered it, and `reset` zeroed it. A writer that forgot to raise it, or raised it
too little, would leave `ramp_frames_left < remaining`. The ramp would then stop in mid-flight on the
settled segment: its word stays off target and the output bits move. Nothing checked the bound
against the words.

## Contract
- Find every writer of `ramp_frames_left` and of every ramp `remaining` word in both crates, and
  record whether an underestimate is reachable today: a retarget, a restore or lane swap, a reset, a
  parameter change mid-ramp, a link glide, the D7 recovery, a collapse and its disengage copy, or a
  whole-bank skip. If one is reachable, a regression test reaches it and is red on main.
- Replace the hand-kept field with a bound derived from the ramp words at block start: the largest
  `remaining` over every lane of the channels the block renders. That is the compressor's
  `max_remaining` shape. It costs one vector max per ramp per channel once per block, with no
  per-sample work, and the silent fast path does not pay for it.
- A regression test, red on main, in which a ramp that no writer announced still arrives.

## Non-goals
GR-7 (the ramp encodings, and these crates rebuilding `LinearRamp` from words). The other effects.
The `gliding` predicate. No rendered bit, state word or payload word may move.

## Gates
- A red-on-main regression test, and a recorded reachability proof.
- A one-time no-bit-moved comparison against base, as PR evidence.
- `cargo test --locked -p dynamic-eq -p de-esser`, clippy `-D warnings`, `cargo fmt --check`, rustdoc
  `-D warnings`, `scripts/check-workspace-policy.sh`, the realtime policy tool.

## Evidence (attempt 1)

Base: `main` @ `5d403acd8`.

**Reachability: no underestimate is reachable today.** The two crates have the same shape, so one
proof covers both. The invariant is `ramp_frames_left >= remaining` for every ramp and link word, on
every lane, of every channel the next block renders. Every writer of a `remaining` word or of the
bound, checked against it:
- *Preparation* (`new`, `with_lane_links`) seats every ramp at rest (`reset_lane_full`, `rest_link`)
  with the bound at `0`.
- *Automation* (`apply_automation` -> `retarget`) writes `RAMP_SAMPLES` (64) or `0` through
  `LinearRamp::set_target`, and raised the bound to 64 for every ramp it touched. A change in
  mid-ramp restarts from `current`, still at most 64.
- *`retarget_link`* writes `samples` (at most `LINK_GLIDE_MAX_SAMPLES`) or `0`, and raised the bound
  to exactly that.
- *`restore_lane`* (the plan-swap carry and the #1747 lane swap's second half). `parse_lane` holds
  every parameter ramp's `remaining` to `[0, RAMP_SAMPLES]` and every link's to
  `[0, LINK_GLIDE_MAX_SAMPLES]`. The restore raised the bound to 64 when any parameter ramp moves, and
  to each adopted link word's own `remaining`. A payload under another mode leaves the links at rest.
- *`exchange_lane_defaults`* (the swap's first half) only calls `rest_link`, which writes `0`.
- *`reset`* rests every ramp of every lane (a full reset re-seeds them; a discontinuity snaps them),
  links included, before it zeroed the bound: exact. *`reset_lane`* only lowers words and keeps the
  bound.
- *The D7 recovery* (`finish_block` -> `reset_lane_full`) rests one lane's parameter ramps and leaves
  a glide in flight, which the bound still covers.
- *`run_block`* lowered the bound by `smoothed = min(bound, frames)`, and the `RAMPING` segment steps
  every ramp of every lane of the rendered channels once per frame for exactly `smoothed` frames
  (`advance_word_ramps` over `s.ramps`, and over `s.links` whenever `gliding`, which is true whenever
  any link word moves). So each `remaining` falls by `min(remaining, smoothed)`, and the invariant
  holds after the block. The silent fast path runs only when every lane rests, where every
  `remaining` is `0`.
- *`advance_clocks`* lowered the bound by `frames`. Its one caller, `rack`'s whole-bank skip, passes
  every lane and only skips a bank whose every lane is at rest, so every `remaining` is `0`.
- *A collapsed block* steps the left channel only. The frozen right channel is never read before the
  disengage copy (`desymmetrize_channels`) overwrites it with the left. `rest_lanes` reads the left
  alone while `collapsed`, and the swap and carry paths disengage first.
- Neither crate implements `copy_lane_from`, `swap_lanes_direct` or `apply_prepared_target_lane`, so
  the contract's defaults refuse them.

A scratch harness confirmed this empirically on base. Instrumented, it asserted
`exact <= ramp_frames_left` at every block of the no-bit-moved run below (about 54,000 blocks), and
the assertion never fired.

**Model chosen.** The hand-kept field is gone from both crates, along with every write to it. The
bound is now derived, in `ramp_frames::<MONO>()`:
- It is called once per block, after the silent fast path declines.
- It takes the lane-wise `Lane::max` of every `remaining` word, over `ramps` and `links`, on the
  channels the block renders: the left alone on a collapsed block, as `gliding::<MONO>` reads.
- It then takes one horizontal max over `L::WIDTH` lanes. A restored `-0.0` counts as `0`, which
  agrees with the kernel's `remaining > 0`.
- Cost per block: dynamic EQ 70 vector maxes per channel, de-esser 17. Nothing is computed per
  sample, and the silent path computes nothing.

It is the compressor's `max_remaining` shape (`crates/compressor/src/kernel.rs`). Two consequences:
- A writer that starts a ramp no longer has to announce it to anything.
- The prefix is now exact rather than an upper bound. Where the old bound was loose, the frames
  between the exact end and the old end ran the `RAMPING` body on settled ramps. That body is an
  exact no-op there (`advance_word_ramps` selects a settled ramp's fields back, and the glide blend
  at the rest weights selects its endpoint). The evidence is the no-bit-moved run below, which
  exercised exactly those blocks.

**Test.** `ramp_tests::a_ramp_no_writer_announced_still_arrives`, one in each crate, on a `Simd4`
bank built directly. It makes two plants, each on a fresh bank, so each is that bank's only
ramp. Each plant is band 0's threshold ramp (the de-esser's threshold ramp), pointed 10 dB down
over 64 frames and written straight into the lane words with no announcement: first on lane 2's
right channel, then on lane 1's left channel. One 64-frame block must land each plant exactly on
its target, with `step` and `remaining` at `0`.
- On base `5d403acd8` (the attempt-1 one-plant test, added; it compiles unchanged there), both
  crates are red:
  ```
  assertion `left == right` failed: the right channel's threshold ramp froze in flight
    left: [3248488448, 3253731328, 3189768192, 1115684864]
   right: [3253731328, 3253731328, 0, 0]
  ```
  `remaining` stays at `64.0` (`0x42800000`) and `current` stays at its start. That is the frozen
  ramp a writer that forgets to raise the bound produces.
- At head: passes in both crates.
- Test value: the test turns red if the ramping prefix is sized by anything but the words in force:
  - a cached bound that some writer did not raise (the old shape);
  - a derivation that reads one channel alone on a dual block, either one. No existing test
    catches either (mutations M1 and M1b).

  An attempt-1 draft also planted a link glide on lane 1. It was dropped, because the existing
  `glide_tests` already catch a derivation that skips the link weights or every lane but the first
  (M2, M3), so that plant added no catch of its own.

**Mutations (head).** Each mutant was applied to both crates. Each run is that crate's whole test
suite (lib and every integration target), with the scratch harness skipped.
- M1, `ramp_frames` reads the left channel alone on a dual block: red only on
  `a_ramp_no_writer_announced_still_arrives`, in both crates. Rerun on the two-plant test after the
  fold: red only on it.
- M1b (the review's finding), `ramp_frames` reads the right channel alone on a dual block
  (`&self.state[channels - 1..channels]`): green on both crates' suites with the attempt-1
  one-plant test. On the two-plant test: red only on `a_ramp_no_writer_announced_still_arrives`,
  in both crates.
- M2, `ramp_frames` skips the link weights: red on `a_ramp_no_writer_announced_still_arrives`
  (the attempt-1 two-plant draft), plus `glide_tests::a_silent_bank_never_skips_a_glide` and
  `glide_tests::a_glide_tracks_the_f64_blend_into_the_target_law`, in both crates.
- M3, `ramp_frames` reads lane 0 alone: red on the draft test, plus
  `glide_tests::a_glide_moves_no_bank_mates_bit_and_allocates_nothing` (both crates) and
  `every_bank_lane_renders_its_scalar_instance` (de-esser).
- M4, no ramping prefix at all: red on the new test and 3 to 4 existing ones per crate
  (`a_mid_ramp_nudge_restarts_*`, `a_silent_bank_never_skips_a_glide`,
  `a_glide_tracks_the_f64_blend_into_the_target_law`, and, in the dynamic EQ,
  `a_silent_bank_reaches_exact_rest_and_a_skip_equals_a_render`).

**No bit moved.** The scratch harness (`fx5_nbm.rs`) was never committed. It ran as a
`#[cfg(test)]` module in each crate, so that it could bind `Simd4` and `Simd8` banks through
`bind_bank::<false>`.
- *Cases.* 24 seeds for each of: the scalar instance, a W4 bank and a W8 bank. 500 steps per seed,
  in each crate.
- *Inputs.* Random member prefixes, initial values, link modes and bypass. Blocks of 1 to 128 frames,
  each one of: noise, `+0.0`, `1e-30`, `-0.0`/`+0.0` alternation, or an occasional NaN or `inf`
  (which drives the D7 recovery).
- *Control.* Random automation (a mid-ramp change, left, right or both channels). Symmetric and
  asymmetric blocks. Both resets, and both lane resets. `retarget_link` over 0 to 40,000 samples.
  Snapshots, and later restores into any member lane. In-bank lane swaps (defaults exchanged
  through a buffer, then the payloads crossed). Collapsed runs of 1 to 8 `process_bank_mono` blocks
  with their disengage copy. Whole-bank skips (`advance_clocks`) whenever the bank reported rest.
- *Hashed after every step* (FNV-1a): every output bit, every report, every lane's full state
  payload (both channels' ramp and link words included), the rest mask and the channel agreement.

Results:
- Base `5d403acd8` and head: all 144 digests identical (72 per crate). Totals: de-esser
  `68bbc8b3f9fead14`, dynamic EQ `4024bee2a8d4b3eb`. Debug profile.
- Event counts per crate (base and head identical):
  - de-esser: 22,183 dual and 4,910 collapsed blocks, 61,250 spans, 2,096 link retargets,
    1,229 restores, 702 swaps, 165 skips, 274 D7 recoveries;
  - dynamic EQ: 22,355 dual and 4,616 collapsed blocks, 57,205 spans, 2,169 link retargets,
    1,309 restores, 735 swaps, 164 skips, 303 D7 recoveries.
- Instrumented base: the exact prefix differs from the old bound's prefix in 1,007 rendered de-esser
  blocks and 1,222 rendered dynamic EQ blocks. Those are the blocks the change re-partitions, and
  their bits did not move.

**Gates (head).**
- `cargo test --locked -p dynamic-eq -p de-esser`: all pass (each lib 8, every integration target).
- `cargo test --locked -p effect-compiler -p rack`: all pass.
- `cargo clippy --locked -p dynamic-eq -p de-esser --all-targets -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p dynamic-eq -p de-esser`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `./target/release/realtime_policy`: `realtime policy: ok (294 marked regions in 46 files)`. The
  render change adds no call, allocation, lock or syscall: `ramp_frames` is a fixed-length loop of
  `Lane::max` over inline arrays plus one store to a stack array.
- `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown
  -p dynamic-eq -p de-esser`: clean.

**Interaction with GR-7.** None. `ramp_frames` reads only the `remaining` lane words, which GR-7's
model would keep as lane words, because the SoA stays branch-free. If GR-7 gives `WordRamp` a
`Settled|Ramping` view, `ramp_frames` is its natural "frames until every ramp settles" accessor.

**Verdict (attempt 1): PASS, with one fold.**
- The adversarial review confirmed the main claims:
  - the derived bound is exact, so it is never above the old bound;
  - the frames the old bound ran past the exact end were exact no-ops, which is why the
    re-partitioned blocks moved no bit.
- It also reproduced the harness, judged the per-block cost acceptable and found that the branch
  merges cleanly.
- The fold: the attempt-1 test planted on the right channel only, so a `ramp_frames` that read the
  right channel alone on a dual block survived both suites while freezing left-only ramps. The test
  now also plants a left-channel ramp on lane 1, on its own bank; both M1 and M1b are red (above).
- The coordinator tracks the `-0.0` `remaining` loose restore, below, in #2258.

**Follow-ups (not in this issue).**
- Both crates' `parse_lane` accepts a `remaining` word of `-0.0`. `integral_within` admits it, for a
  parameter ramp and for a link weight, when `current == target`. The kernel never steps such a
  word (`remaining > 0` is false), but `positive_zero_lanes` never reports it at rest. So a restored
  lane carrying one is never silent-skipped. The engine's own snapshots never write one
  (`u32 as f32`), so this is the FX-7 shape: a loose restore. Reject it.
