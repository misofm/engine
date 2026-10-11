# Replay an effect ramp whose endpoint clamp acts in the wasm corpus

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-06 by the decision-15 root coordinator's ruling (2) on *Keep every effect parameter
ramp inside its endpoints* (#1409, attempt 1 verdict NIT 7). Code anchors verified on
`codex/d15-stream-g` at `c7f7bbdfb`.

## Product outcome

The browser build is proven to clamp an effect's parameter ramp at its target bit for bit as the
native build does. Today the wasm `simd128` leg reaches #1409's clamp only through `lane`'s own
cases (the `ramp_toward` `pmin`/`pmax` lowering); no G5 corpus case renders an effect move in which
the clamp acts. After this slice at least one effect corpus case does, and its digest is replayed
under wasm.

## Context

- **The clamp.** #1409 routes every effect ramp site through
  `lane::kernels::ramp_toward(current, step, target)` (strict `min`/`max`, keeps an in-range word's
  bits, passes NaN). Gate 2 (`tests/ramp_endpoint.rs` per effect) runs native only; `aarch64-debug`
  runs it at NEON width.
- **The G5 corpus.** `tools/wasm-gate-corpus/src/lib.rs` delegates whole rendered effect blocks to
  each effect's `corpus` module (`transient_shaper`, `delay`, `multiband_compressor`, `soft_clip`,
  `parametric_eq`, `gate_expander`, `builtins`), pins their digests in the effect crate (for example
  `crates/gate-expander/src/gate_digests.in`), and replays them under wasm
  (`scripts/run-wasm-gates.sh`); `wasm-gates`' `g5_native_digests_match_pins` is the native owner of
  every pin (#1048). Some cases already have a D11 ramp in flight (gate-expander, parametric EQ),
  but #1409 moved no pin, so no case's ramp is one the clamp changes.
- **The moves.** #1409's scan found, per ramped word, an in-domain move whose unclamped word passed
  its target (for example the gate threshold at frame 33). Such a move is the case to add.

## Decisions frozen for this slice

- **D0. Root decision (2026-10-06).** The decision-15 root coordinator filed this slice as the wasm
  reach of #1409's clamp.
- **D1. One effect, one case.** Add one case to the gate-expander corpus (site 5, the gate
  prologue, a vector clamp site at every lane width): a move from #1409's scan, with lanes ending
  their ramps on different frames, in which the clamp acts on at least one frame per lane.
  Another effect may be chosen only with a reason recorded (for example a site the gate case cannot
  reach).
- **D2. New pin, no moved pin.** The new case adds one digest row; every existing pin stays.

## Deliverables

1. The new case in the chosen effect's `corpus` module, its pin in that crate's `.in` file, the
   case count in `tools/wasm-gate-corpus` updated.
2. The attempt record: the clamp acts in the case (the frames where the unclamped word passes its
   target), and the mutation evidence of gate 2.

## Authorized paths

- `crates/gate-expander/src/corpus.rs`, `crates/gate-expander/src/gate_digests.in` (one new row)
- `tools/wasm-gate-corpus/src/lib.rs` (case count and doc only)
- This spec

## Non-goals

- Cases for the other effects; changing the clamp or any render code; native gate-2 tests.

## Hazards

- Hot files: `crates/gate-expander/src/corpus.rs` and `tools/wasm-gate-corpus/src/lib.rs`
  (other slices add G5 cases to the same list); the later slice rebases.
- A move that is clamped only after the window, or on no lane at the native width, is vacuous:
  gate 1 checks reach.

## Objective gates

1. **Reach.** A check in the case's test (native) shows the unclamped law passes the target on at
   least one frame of the case per lane, so the clamp acts.
2. **Pins.** `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
   passes with the new row, and no existing pin moved; `bash scripts/run-wasm-gates.sh` passes
   (simd128 leg replays the new case).
3. **Red.** The gate prologue clamp reverted to `add` (#1409's site-5 mutant) moves the new digest
   (red natively); recorded.

## Test value

(Corrected by root after the attempt 1 verdict's MINOR-3.) In the wasm `simd128` build, a gate
ramping prologue that lets a threshold, ratio or range word pass its target before the snap (the
clamp dropped, bypassed, or bounded by the wrong endpoint) moves the new case's digest under wasm;
no existing case reaches it, and the native `tests/ramp_endpoint.rs` never runs under wasm. The
case does not pin `ramp_toward`'s operand order, signed zero or NaN: its words are finite and
nonzero (D5 excludes NaN; the gate normalizes zero targets), and swapping the `min`/`max` operands
moves no G5 digest. That coverage is a successor issue's (filed with the batch follow-ups).

## Dependencies

- #1409 (merged with the stream G batch).

## Attempt record

### Attempt 1 (2026-10-07, implementer)

**Case.** `crates/gate-expander/src/corpus.rs` case 6, `dual_mono/clamped_ramp` (G5 case 124),
pinned as the seventh row of `gate_digests.in` from the scalar oracle
(`fb0c11e0…db147186`); `CASE_COUNT` 6 → 7, and `tools/wasm-gate-corpus` derives its count from it
(doc updated only). No existing pin moved: `g5_native_digests_match_pins` printed case 124 only,
equal at scalar, simd4 and simd8, before the pin was written.

**The move (D1).** Every ramped word (threshold, ratio, range, hysteresis) of every lane and
channel starts `0x21` ulps from its resting value, the offset of #1409's scan moves (threshold
`0xc29fffdf → -80.0`, ratio `0x3f800021 → 1.0`, range `0x42bfffdf → 96.0`, hysteresis
`0x41bfffdf → 24.0`), and ramps back to it with the control plane's own step law
(`LinearRamp::set_target`) over a window of `40 + 3·lane + channel` samples (40..62, each lane and
channel ending on a different frame, all inside the 64-frame ramping block). Reason for applying
the scan's offset to each lane's own resting word rather than replaying the four scan moves
verbatim: the scan's ratio move ends at `1.0`, where the expansion curve is zero and the threshold
and range words cannot reach the output; per-lane resting words keep threshold, ratio and range
live in the curve (hysteresis is not: see the batch follow-ups for the measured reach). The input
is quiet noise at an exact power of two 3 to 9 dB under each lane's re-arm level, so
the gate closes after its hold and the curve sets the gain while the clamp acts.

**Gate 1 (reach).** `corpus::tests::the_clamped_case_passes_every_target_under_the_unclamped_law`
replays the unclamped law per lane, channel and word. All 64 words first pass their target at
ramp step 34 (kernel frame 33, counting from 0), and every window is 40..62, so the clamp acts on frames 34..window−1 (6 to 28 frames)
of every word of every lane at every width. Test value: a case edit that leaves some lane's move
unclamped turns it red; the digest cannot, as it would be re-pinned with the case. Mutations:
window `66 + 3·lane + channel` → red ("window 66 ends past the ramping block"); window
`34 + lane + channel` → red ("stays inside … over its 34-sample window"); offset `0x10` → red
(step under half an ulp, "stays inside … over its 40-sample window"); reverted → green.

**Gate 3 (red).** #1409's site-5 mutant (`let stepped = ramp.current.add(ramp.step);` in
`kernel.rs` `channel_step`) → `g5_native_digests_match_pins` red on case 124 only, at scalar, simd4
and simd8 (got `99502499…18ea64bb`); every other case stayed green, confirming no existing case
reaches the clamp. Per lane output under the mutant: 15 of 16 lane-channels move (first moved
frame 35..49); right channel of lane 0 does not move its output (its clamp acts on the ramp words,
but under the combined mutant the bits round away in the one-pole; a ratio-only or a range-only
mutant moves that lane-channel's output on 3 frames each, so it is not a structural blind spot). Reverted → green.

**Gate 2 (pins).** `cargo test --locked --release -p lane -p math -p wasm-gates --features
math/lane` pass. `bash scripts/run-wasm-gates.sh` pass: native and wasm `simd128` legs both
report 144 cases (was 143), 0 mismatches.

**Other gates.** `cargo test --locked --release -p gate-expander` pass; `cargo fmt --all --
--check` pass; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
pass; `scripts/check-workspace-policy.sh` pass; `scripts/check-cross-targets.sh` PASS. Realtime
policy and the worklet chain not run: no render code and no worklet-compiled code changed (the
corpus module is not in the worklet). AArch64 runs only in CI.

### Batch follow-ups (stream G2 part 2, 2026-10-07; verdict `1459-attempt1.md` PASS)

No digest moves: the corpus refactor (`group_input`, extracted from `run_case`) builds the same
inputs, and `g5_native_corpus` (release, `--features math/lane`) passes 7/7.

- **MINOR-1: the reach test now sees the rendered case.** The superseded
  `the_clamped_case_passes_every_target_under_the_unclamped_law` (it read only the move constants)
  is replaced by `corpus::tests::the_clamp_holds_every_word_of_the_clamped_case_at_its_target`. It
  steps the case's own `prepare::<f32>(CLAMPED_CASE, lane, channel)` state through
  `gate_block::<f32, false, true>` one frame at a time, and asserts for every lane, channel and word
  that on some frame the kernel holds the word at its target while the unclamped replay of the
  case's own move is past it and the ramp still counts (the clamp's hold, not the snap), and that
  every ramp has ended at the end of the ramping block. It then renders the rest with
  `gate_block::<f32, false, false>` and requires `run_case::<f32>(CLAMPED_CASE)`'s output bit for
  bit, so the case checked is the case pinned.
  - Test value: an edit that stops the clamp acting in the rendered case turns it red; the digest
    cannot, since it would be re-pinned with the case.
  - Mutations (`cargo test -p gate-expander --lib corpus`, each alone, reverted): M1 window
    `66 + 3·lane + channel`: red ("lane 0 channel 0 word 0: the clamp never held the word at its
    target ..."); M2 window `34 + lane + channel`: red (same); M3 offset `0x10`: red (same); M4
    `let ramping = case == 5;` in `run_case`: red ("lane 0 channel 0 frame 13: run_case does not
    render the stepped clamped case"); M5 `remaining: L::zero()` in the clamped `GateRamp`: red
    (held); #1409's site-5 mutant (`ramp.current.add(ramp.step)`): red (held); `RAMP_FRAMES` 48:
    red ("lane 3 channel 0 word 0: the ramp still counts at the end of the 48-frame ramping
    block"). Reverted: green. M4 and M5 were green on the old test.
- **MINOR-2: output reach as measured** (verifier's per-word mutants at scalar, simd4 and simd8,
  identical across widths): without the clamp on all four words, 15 of 16 lane-channels' output
  moves (first moved frame 35..49); range alone 16 of 16; threshold alone 6; ratio alone 4;
  hysteresis alone 0. Hysteresis cannot reach the output in this case: the expansion curve does not
  read it, only the re-arm comparison does, and that runs only while the gate is open, which ends
  with the hold (frame 23 or earlier) before the clamp acts (from frame 33). The module doc
  (`corpus.rs`) now states this; the record above is corrected.
- **MINOR-3 (root): the Test value is corrected** to what the case proves (a wasm prologue that lets
  a threshold, ratio or range word pass its target), not operand order, `-0.0` or NaN. Root filed
  the successor **#1473** (*Pin ramp_toward's operand order, signed zero and NaN in the wasm
  corpus*, which also takes wasm digest coverage of the multiband's per-segment dispatch, #1455);
  STREAMS row added after #1459.
- NIT-1: `CLAMPED_CASE`, `CLAMPED_OFFSET_ULPS`, `clamped_window` and `clamped_move` are private;
  the public module doc names the case without intra-doc links. This also fixes
  `RUSTDOCFLAGS='-D warnings' cargo doc` at `corpus.rs:64` (a public item's doc linking the private
  `RAMP_FRAMES`); `cargo doc -p gate-expander --no-deps` with `-D warnings` passes.
- NIT-2: the input is 3 to 9 dB under the re-arm level (record and `quiet_peak` doc), and the first
  pass is at ramp step 34, kernel frame 33 counting from 0 (record).
- NIT-3: the lane 0 right-channel gap is a rounding coincidence of the combined mutant; a ratio-only
  or range-only mutant moves that lane-channel's output on 3 frames each (record).
