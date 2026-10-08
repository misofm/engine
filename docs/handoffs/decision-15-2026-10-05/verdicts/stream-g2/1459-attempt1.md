PASS

# #1459 attempt 1 verdict: `ffcbba6b7` (parent `7c43b2513`), branch `codex/d15-stream-g2`

Verifier: opus-xhigh, 2026-10-07. Reviewed from an export of `ffcbba6b7` at
`/tmp/claude-1002/v1459/tree`. Built with `CARGO_TARGET_DIR=/tmp/claude-1002/v1459/target`. I did
not touch the worktree (`git status` is clean).

The case does what the spec asks. G5 case 124 `effect/gate_expander/dual_mono/clamped_ramp` renders
a site-5 move in which the clamp acts on every word of every lane and channel. It adds one new pin
and moves no existing pin. The wasm `simd128` leg replays it with 0 mismatches. #1409's site-5
mutant turns only this case red, at all three widths. There is no BLOCKER and no MAJOR. Three
MINORs need a fold-in: two evidence claims are too broad, and the third is a spec test-value claim
for root.

## Answers to the four questions

1. **The D1 deviation is within the spec's intent.** The recorded reason is correct. The scan's
   ratio move ends at `1.0`, and there `curve = (ratio - 1)(level - T)` is zero. Also, an unclamped
   ratio below `1.0` makes the curve positive, and `min(0)` then removes it. So a verbatim replay
   would hide the ratio clamp from the output, and it would also scale the threshold and range
   words by `ratio - 1 <= 3.9e-6`. The 0x21-ulp offset on each lane's own resting word moves every
   word by 1 ulp per frame. Every word first passes its target at step 34. That is the same path
   as the scan's n=64 moves (I measured the step at 0.53 to 0.83 ulp, which rounds to 1 ulp). A
   window of 40..62 equals a production 64-sample ramp seen in mid-flight
   (`remaining < 64`), so the replayed state is reachable. The window also makes each lane and
   channel end on a different frame, which D1 requires. The distinct per-lane targets also exercise
   D4 (each lane clamps toward its own target). The case covers both sides of the clamp: the
   threshold rises and is held by `min(high, .)`, and ratio, range and hysteresis fall and are held
   by `max(low, .)`.
2. **The clamp acts on every lane and channel the spec requires. The gap in lane 0's right channel
   is acceptable and is not a reach hole.** Word level: all 64 words (8 lanes x 2 channels x 4
   words) first pass their target at step 34, and the clamp changes 6 to 28 steps per word before
   the snap. Output level: I measured it with a per-word copy of the site-5 mutant, at scalar,
   simd4 and simd8, and the three widths agree bit for bit (`probe-report.keep.txt`):

   | mutant | lane-channels whose output moves |
   |---|---|
   | all four words (= #1409 site 5) | 15/16 (first moved frame 35..49) |
   | range only | 16/16 |
   | threshold only | 6/16 |
   | ratio only | 4/16 |
   | hysteresis only | 0/16 |

   Lane 0's right channel moves under the ratio-only and range-only mutants (3 frames each). Only
   the combined mutant cancels to identical bits. So this is a rounding coincidence, not a
   structural hole. At the wasm width (Simd4), every vector position on each channel has at least
   one lane-channel whose output moves. The hysteresis gap is structural: the curve does not read
   hysteresis, only the re-arm comparison does, and that comparison runs only while the gate is
   open. In this case the gate closes when its hold ends (frame 23 or earlier), and the clamp acts
   from frame 33. The spec asks for reach at word level per lane, and the case meets it. The output
   claims are MINOR-2.
3. **The digest row is allowed and has its reason.** `gate_digests.in` is the gate's slice of the
   G5 cross-target corpus. `wasm-gates`' `g5_native_digests_match_pins` is its single owner
   (#1048), and `g5_delegated_cases_use_the_owning_crates_pins` makes sure that the crate's table is
   the one that pins it. This is the case that AGENTS.md permits. The row is new, not a re-pin, and
   the diff appends it only. Its source (the scalar oracle, equal at scalar, simd4 and simd8 before
   the pin was written) is in the commit message and the attempt record. The file has no
   per-row comments, by convention, and `CASE_NAMES` gives the row's name.
4. **The reach test asserts the property, not bytes.** It replays the unclamped law from the
   case's own `clamped_move`, and it asserts that each word leaves `[min, max]` before the snap and
   that each window fits in `RAMP_FRAMES`. It pins nothing. Its weak point is that it checks only
   the move constants, not the way they reach the rendered case (MINOR-1).

## Findings

### MINOR-1: the reach test does not see the rendered case, and the attempt record's test-value sentence is too broad

`crates/gate-expander/src/corpus.rs:290-324`; spec `:112-113`.

The test reads `clamped_move`, but nothing that `prepare` or `run_case` does with that result. I
ran two wiring edits that stop the clamp from acting anywhere in the rendered case. The test stayed
green on both:

- M4: `let ramping = case == 5;` (`:249`). The case then renders without the ramping prologue.
- M5: `remaining: L::zero()` in the clamped `GateRamp` (`:208`). No word moves.

The test's doc scopes its claim with a parenthesis: a window too long, a window too short, or a
window past the block. That claim is true, and I reproduced all three reds. But the doc's first
paragraph says "so #1409's clamp acts in the rendered case". The attempt record's sentence "a case
edit that leaves some lane's move unclamped turns it red" is general, and M4 and M5 make it false.
The spec's Hazards section names this file as hot (other slices add G5 cases), so a wiring edit such
as a shifted `CLAMPED_CASE` or a refactored `run_case` is plausible.

Fix in the fold-in, preferred option: drive the case's own `prepare::<f32>(CLAMPED_CASE, lane, channel)` state through
`gate_block::<f32, false, true>`, one frame at a time. For each lane, channel and word, assert that
on some frame the kernel holds the word at its target while the unclamped replay is past it. That
covers M1 to M5. Other option: narrow the test doc and the record sentence to the edits the test
actually catches.

### MINOR-2: the evidence says more about output reach than was measured

Spec `:105-106`; `crates/gate-expander/src/corpus.rs:23-28`.

The record says "per-lane resting words keep every word live in the curve". This is not true for
hysteresis. The curve does not read it, and its clamp changes no digest bit on any of the 16
lane-channels. Threshold and ratio reach the output on only 6 and 4 of the 16 lane-channels. The
module doc's "the clamped prologue (`ramp_toward`) holds the word at the target instead, and the
digest records that" is also too broad. It is true for the move as a whole (15 of 16 lane-channels)
and for range (16 of 16), but not for every word. Fix: put the measured per-word reach (the table
above) in the record, and say why hysteresis is unreachable here. The spec does not need to change.

### MINOR-3 (for root, spec text): the spec's test-value examples are beyond the reach of this case

Spec `:77-81`.

The spec's Test value says that a wasm difference in pmin/pmax operand order, `-0.0` or NaN would
turn the new case red. Every word in this case is finite and nonzero. D5 excludes NaN from the
corpus. The gate normalizes zero targets (`normalize_zero` in `apply_automation`). For such
operands `min` and `max` are symmetric. Measured: I changed `ramp_toward` to
`L::min(L::max(next, low), high)` (`crates/lane/src/kernels/builtins.rs:226`), and no G5 digest
moved, case 124 included (`operand-order-mutant.log`). The case's real value is narrower (see
"Test value" below). The implementer cannot edit that section, so root should correct the
sentence. If wasm reach of signed zero in a real render path is wanted, root can file a successor.

### NIT-1: new corpus items are public without a need

`crates/gate-expander/src/corpus.rs:54, :61, :69, :76`.

`CLAMPED_CASE`, `CLAMPED_OFFSET_ULPS`, `clamped_window` and `clamped_move` are `pub`, but only this
module and its tests use them. They add to the crate's public API for no reason. Make them private.

### NIT-2: the record's input level and frame numbers are loose

Spec `:106`, `:110-111`.

- The input is "at an exact power of two 3 dB under each lane's re-arm level". `quiet_peak` is the
  power of two at or below `-64 + 2.5*bias` dB, so the input is 3 to 9 dB below the re-arm level.
  The doc at `corpus.rs:134-136` says this correctly.
- "First pass their target at frame 34" counts ramp steps from 1. The kernel's 0-based frame is 33,
  and the spec's Context uses 0-based frames ("the gate threshold at frame 33").

### NIT-3: the explanation of the lane 0 right-channel gap could be more exact

Spec `:121-123`.

The record says "the bits round away in the one-pole". That is true only for the combined mutant.
A single-word mutant of ratio or range moves that lane-channel's output on 3 frames each. Add one
sentence so that a reader does not take it for a structural blind spot.

## Test value (new case and new test)

- **Case 124 (`dual_mono/clamped_ramp`, its pin row).** In the wasm `simd128` build, a gate
  ramping prologue that lets a threshold, ratio or range word pass its target before the snap (the
  clamp dropped, bypassed or bounded by the wrong endpoint) moves case 124's digest. Natively,
  `crates/gate-expander/tests/ramp_endpoint.rs` already catches the site-5 revert (red, which I
  re-ran). But that file never runs under wasm, and the site-5 revert moves no other G5 case (I
  re-ran this; red on case 124 only, at scalar, simd4 and simd8, got `99502499...18ea64bb`). The
  case does not catch an operand-order, `-0.0` or NaN difference (MINOR-3), and it does not catch a
  hysteresis-only defect (MINOR-2).
- **`corpus::tests::the_clamped_case_passes_every_target_under_the_unclamped_law`.** If someone
  changes the case's offset, window or ramping-block length so that some lane's word no longer
  passes its target before the snap, this test turns red. The digest pin cannot catch that,
  because the pin would be re-pinned with the case. I reproduced the three reds: window `66+3l+c`
  ("ends past the ramping block"), window `34+l+c` and offset `0x10` ("stays inside ... so the
  clamp never acts"). After the revert it is green. The test does not catch wiring edits (MINOR-1,
  M4 and M5 stay green).

## Gates I ran (from the export)

| Gate | Command | Result |
|---|---|---|
| 1 (reach) | `cargo test --locked --release -p gate-expander` (lib unit test included) | pass. M1, M2 and M3 red with the recorded messages; M4 and M5 green (MINOR-1) |
| 2 (pins) | `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` | pass, exit 0. `g5_native_digests_match_pins` and the G6 full-corpus FTZ arms are green |
| 2 (wasm) | `bash scripts/run-wasm-gates.sh` (native + wasm simd128 + V8 spill) | pass. Native: 144 cases, 364 comparisons, 0 mismatches. Wasm: 144 cases, 254 comparisons (252 + 2), 0 mismatches |
| 3 (red) | site 5 changed to `ramp.current.add(ramp.step)`, then `cargo test --release -p wasm-gates --test g5_native_corpus` | red on case 124 only, at scalar, simd4 and simd8; reverted afterwards |
| extra | `cargo fmt --all -- --check`; `scripts/check-workspace-policy.sh`; `cargo clippy --locked -p gate-expander -p wasm-gate-corpus --all-targets --all-features -- -D warnings` | pass |
| extra | shipped AudioWorklet module built at the parent and at `ffcbba6b7` | identical: shipped `04b7c5f5...6e34aa` (3097126 B), named twin `e3816ff2...d6ef50`. The corpus module does not reach the browser artifact, which confirms that skipping the worklet chain was correct |

Not re-run: `scripts/check-cross-targets.sh` (the implementer reported a PASS). The change adds
only an unreferenced test-corpus module that uses `floor` and the existing `effect_runtime`
imports, the wasm32 guest compiled it, and clippy passes on the touched crates. The realtime policy
check does not apply, because no render code changed. AArch64 runs only in CI.

Scope: the diff touches only the authorized paths (`corpus.rs`, `gate_digests.in` with one appended
row, a doc-only change in `tools/wasm-gate-corpus/src/lib.rs`, and the spec's Attempt record, which
it only appends to). STREAMS: #1459 is stream G slice 16, and it depends only on #1409, which has
landed. The change has no queue, so the acked-batch question does not apply. No name has a version
suffix.

Evidence kept in `/tmp/claude-1002/v1459/`: `gate2-native.log`, `wasm-gates.log`,
`wasm-gates.jsonl`, `gate3-mutant-g5.log`, `operand-order-mutant.log`, `probe-report.keep.txt` (the
per-word and per-lane-channel reach), `probe_1459.rs` (the probe test, which I did not commit),
`parent-worklet.log`.
