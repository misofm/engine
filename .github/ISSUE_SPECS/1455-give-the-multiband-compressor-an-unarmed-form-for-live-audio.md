# Give the multiband compressor an unarmed form for live audio

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Filed 2026-10-06 by root order (Amendment 2 of *Let the builtins splat their chain constants
without iOS memset calls*, #1451). Code anchors verified on `codex/d15-stream-g` at `44585c80f`
(#1451 attempt 1's record). Ordered after #1451.

## Product outcome

On live audio the multiband compressor stops paying for exact rest. Today its crossover runs the
silence counter and the SVF joint flush on every frame, also on blocks where no lane can arm. The
builtin input chain and the parametric EQ already run those only on a block that can arm, and an
unarmed form (per-word flush, no counter in the loop) on every other block. After this slice the
multiband compressor has the same two forms with no rendered bit moved, kept only if codegen and a
descriptive p50 show it is better.

## Context

- **Why it has one body (#1328, record "Two forms per builtin chain body").** At #1328 A9 a second
  form doubled the crossover's splatted vector constants, and on `aarch64-apple-ios` every splat
  was a `memset_pattern16` libc call (#1018): `multiband-compressor`'s count would have risen from
  566 to 1128 against its ceiling. So the multiband kept one body with the counter and the joint
  flush on every frame. It was not in the browser documents measured then.
- **The ratchet cause is gone (#1451 D2).** `wide`'s `splat` reached LLVM as an array-repeat store
  loop that loop-idiom turned into Darwin's `memset_pattern16`; #1451 builds splats from an array
  literal in `lane::Lane::splat`/`zero`. `multiband-compressor`'s iOS count fell from 566 to 0 and
  its row in `scripts/lib/aarch64-known-defects.py` was deleted; a crate with calls and no row now
  fails `check-cross-targets.sh`. A second form no longer argues against the ratchet.
- **Code anchors.**
  - `crates/multiband-compressor/src/lib.rs`: `lr4_step` (`:509-514`), one frame of the LR4 split
    with `svf_step(.., rest, ..)` on both stages; `run_segment` (`:906`) computes
    `rest_near`/`rest_far` with `lane::silence_step` on every frame (`:945-946`); its caller `process_block` (`:1006`) sets
    `armed_after = L::splat(lane::silence_frames(sample_rate) as f32)` (`:1019-1020`) and runs
    `run_segment` per ramp segment (`:1022-1059`); the counter's state record (`:1347`) and its
    restore (`:1464`).
  - The pattern to follow: `lane::kernels::builtins::input_chain_block` (`:597-615`) tests
    `channel_arms` (`:516-518`, `silence_armable_holding` over the counter, the block length,
    `armed_after` and whether the sections hold state) once per block; if no lane can arm it
    advances each counter once with `lane::kernels::silence_skip_block` (`crates/lane/src/kernels.rs:327`)
    and runs the body with `svf_step_when(false, ..)` (`crates/lane/src/kernels.rs:1028`), the
    per-word flush. `flush_pair(n1, n2, +0.0)` is `(flush(n1), flush(n2))`, so the two forms are the
    same bits.
  - Pins: multiband `DIGESTS` (`crates/multiband-compressor/src/corpus.rs:252`,
    `src/corpus_digests.in`); `crates/multiband-compressor/src/corpus.rs:151-153` (the corpus's
    own counter use).
- **No browser document reaches the multiband.** The four browser documents and the native console
  rows run no multiband compressor, so p50 here is a scratch native harness and the worklet
  module's codegen (the multiband `f32x4` body is in the shipped module).

## Decisions frozen for this slice

- **D0. Root decision (2026-10-06).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), ruled in #1451 Amendment 2 that the
  multiband's unarmed form is unblocked by #1451 D2 and gets its own issue, this one, gated
  bit-identical with codegen and p50 evidence.
- **D1. Same law, two forms.** Per segment, test once whether some lane's counter can arm (the
  builtins' `channel_arms` rule, over both crossover stages' state of each side). If none can,
  advance each side's counter once with `silence_skip_block` and run the crossover with the
  per-word flush (`svf_step_when(false, ..)`) and no counter in the frame loop; otherwise run
  today's body. Ramping and settled segments, every link mode and the bypass path keep their
  behaviour. No change to `N_SILENCE`, the counter semantics, the state record or its restore.
- **D2. Class A.** No rendered bit moves on any target.
- **D3. Keep-or-revert, decided before timing.** The unarmed form lands when gates 1-3 pass, the
  live-audio frame loop's instruction count on x86-64-v3 and `simd128` is lower than the base (or
  equal with a lower p50), and the descriptive p50 on live audio is not higher than the base and on
  a silent tail at most +2 % of it. Otherwise it is reverted, and the attempt record gives the
  counts, the p50 and the reason.

## Deliverables

1. The two-form crossover (D1), with a doc stating the forms and that they are the same bits.
2. The PR evidence: the base-versus-head differential (gate 1), the codegen counts and the p50
   table (gate 4), the iOS count (gate 2).

## Authorized paths

- `crates/multiband-compressor/src/lib.rs` (`run_segment`, its caller's per-segment dispatch, and
  `lr4_step` only if D1 needs a flag parameter)
- `crates/lane/src/kernels/builtins.rs` (`channel_arms` only, to make it `pub` for reuse, if
  needed) or a shared helper in `crates/lane/src/kernels.rs`
- `crates/multiband-compressor/tests/` only where a test names a changed item
- this spec

## Non-goals

- Any change to the #1328 law, the counter's payload, the multiband's tail or rest bounds (#1373),
  its live crossover (#1338) or bypass shunt (#1340).
- Native AArch64 timing (CI-only, no funded runner).
- A browser document with a multiband: not added here (`benchmark-real-paths-only`).
- Rejected alternatives: a run-time flag in the loop body in place of two instantiations (LLVM
  unswitched it at #1328: same code); keeping the single body without measurement (D0 orders the
  measurement).

## Hazards

- `crates/multiband-compressor/src/lib.rs` is a hot file: #1409 and #1338 edit `run_segment`'s ramp
  (STREAMS.md hot-file row), and #1340 and #1367 edit the crate; rebase onto whichever lands first
  and keep to the dispatch.
- A second instantiation of `run_segment` doubles its code size; the shipped worklet module's size
  is checked by `check-web-audioworklet.sh`.
- `check-lane-policy.sh` refuses some spellings inside `crates/lane`; use the crate's accepted forms.
- Another implementer may be working in the same worktree; commit exact paths only.

## Objective gates

1. **No rendered bit moves.** A base-versus-head differential through the multiband compressor at
   scalar, `Simd4` and `Simd8`, every link mode, bypassed and not, ramping and settled segments,
   live input, input that stops inside a block and at a block boundary, a silent tail through
   arming and exact rest, a state capture and restore mid-tail, at 44.1 and 96 kHz with quanta 1,
   128 and 4,096: every output and every state word identical. PR evidence, not a committed test.
   The multiband `DIGESTS`, `conformance_fixtures --check` and the multiband crate's tests are
   unchanged.
2. **iOS count stays zero.** `bash scripts/check-cross-targets.sh` passes with no
   `multiband-compressor` row (a crate with calls and no row fails).
3. **V8 and worklet.** `bash scripts/run-wasm-gates.sh` exits 0, and the worklet chain passes.
4. **Codegen and p50 (descriptive, D3).** The x86-64-v3 and `simd128` instruction counts of the
   multiband's live-audio frame loop, base and head, and the TurboFan count of the same loop in the
   shipped module. p50: a scratch native harness (not committed), the multiband bank at scalar,
   `Simd4` and `Simd8`, 128-frame blocks, all lanes live and one lane silent, one warmup and two
   measured rounds per build, pinned to one CPU, one invocation, not retried.
5. **Workspace gates.**
   - `cargo test --locked --all-targets -p lane -p multiband-compressor -p effect-runtime -p dsp-reference -p conformance --features math/lane,lane/test-support`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml`
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - `bash scripts/check-lane-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `python3 -B scripts/lib/aarch64-known-defects.py --self-test`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

The multiband `DIGESTS` (native and wasm) do not cover the per-segment dispatch: `corpus.rs`
drives `lr4_step` with its own counter and never calls `process_block` (corrected by root after
attempt 1's verdict). The dispatch is defended by the existing silence tests (always unarmed, off
by one, no counter advance) and by the new product test
`either_channel_and_either_stage_arm_the_crossover`, which turns red when the per-segment arming
test reads one channel only or one crossover stage only. No wasm digest covers the dispatch; part 2
of stream G2's follow-ups adds wasm coverage to a successor issue.

## Dependencies

#1451 (and #1328, which it follows).

## Attempt record

### Attempt 1 (2026-10-07)

Base `88c62e7f5` (stream G2 after #1458/#1459). Changed: `crates/multiband-compressor/src/lib.rs`
and `crates/multiband-compressor/tests/product.rs` (one new test). No `crates/lane` change: the
arming test is `Side::arms`, the builtins' `channel_arms` rule written from the two public lane
primitives it wraps (`lane::silence_armable_holding` over `kernels::svf_state_held([a, b])`).

**The change (D1).** `process_block` tests once per segment, for a non-bypassed instance, whether
either side `arms` over the segment's length. If not, it advances each side's counter once with
`silence_skip_block` over the segment's input (before the frame loop overwrites it) and runs
`run_segment::<.., ARMABLE = false>`: no `silence_step` in the frame loop and both crossover
stages on the per-word flush (`lr4_step_when(false, ..)`, a private sibling of `lr4_step`, which
now delegates with `true`; `lr4_step`'s signature and its corpus and test callers are
unchanged). Otherwise today's body (`ARMABLE = true`). Bypass takes `ARMABLE = true` only, so it
gains no second body. `ARMABLE` is a fourth const generic, not a run-time flag (the spec's
rejected alternative). The docs on `run_segment`, `lr4_step_when` and `Side::arms` state the two
forms and why they are the same bits. `N_SILENCE`, the counter, the state record and its restore
are unchanged.

**Gate 1, no bit moved.** Scratch differential (`w1455/w1455_diff.rs`, not committed) through the
public contract: scalar instances, `Simd4` (two banks) and `Simd8`, every link mode, bypassed and
not, 44.1 and 96 kHz, quanta 1, 128 and 4,096; 40,000 frames of eight per-lane input shapes (stop
on a block boundary, stop mid-block, live with short zero runs, silent throughout, live then an
armed silence then a burst, `±0.0` alternation, subnormal input, impulses); block-rate automation
in the live part, at the live/silent edge and in the silent tail (ramping and settled segments);
a snapshot every 2,048 frames; and at frame 18,000 a capture and restore into a fresh instance
with joint-band crossover words (both stages, one stage, none) and counters at `N_SILENCE - 1`,
`N_SILENCE - q`, `N_SILENCE - q - 1`, `N_SILENCE - 2` or as counted, per lane and side. Every
output word, report and snapshot hashed per configuration: **36 of 36 lines identical** base vs
head. Sensitivity (each mutation of head, then reverted): always unarmed moves 18 lines, arming
test over `frames - 1` 6, near side only 9, far side only 6, stage `a` only 3, stage `b` only 9,
no `silence_skip_block` 18. The multiband `DIGESTS`, `conformance_fixtures --check` and the
crate's tests pass unchanged.

**Spec correction.** The multiband `DIGESTS` do not reach this dispatch: `corpus.rs` calls
`lr4_step` with its own counter, never `process_block`. Of the seven mutations above, existing
tests catch three (always unarmed, off by one and no `silence_skip_block`:
`the_crossover_joint_flush_arms_after_its_inputs_silence`, `each_channel_counts_its_own_input`,
`the_silence_counter_is_carried_and_validated`); the four that read one side or one stage only
were green on every existing test. Hence the one test below (the spec's Test value clause).

**Test value.** New `either_channel_and_either_stage_arm_the_crossover` (`tests/product.rs`):
*red when the per-block arming test reads only one channel, or only one crossover stage, for its
held state, which no existing test catches because each restores both channels with both stages
banded.* Mutation runs (head mutated, test run, reverted): near side only -> red (channel 1,
stage 0 keeps its words); far side only -> red (channel 0, stage 0); stage `a` only -> red
(channel 0, stage 1); stage `b` only -> red (channel 0, stage 0); base `88c62e7f5` (one form) and
head -> green.

**Gate 2.** `check-cross-targets.sh` passes; no `multiband-compressor` row (rows left: builtins 5,
host-core 4, soft-clip 1).

**Gate 3.** `run-wasm-gates.sh` exits 0 (EQ rows unchanged). Worklet chain passes:
`build-web-audioworklet.sh --named-twin` (shipped module `bf0125fe...`, 3,139,847 B; base
`04b7c5f5...`, 3,097,126 B: **+42,721 B**, the second `run_segment` body), `check-web-audioworklet.sh
--without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
`test-web-audioworklet.sh` (empty TMPDIR after).

**Gate 4, codegen** (innermost frame loop of `process_bank`, live audio = settled, non-bypassed,
DualMono / Average / Maximum; base -> head unarmed, head armed in brackets):

| target | settled (live audio) | ramping |
|---|---|---|
| x86-64-v3 `f32x8` (`--emit asm`, instruction lines) | 457 / 462 / 545 -> **426 / 428 / 505** (459 / 464 / 546) | 620 / 625 / 703 -> 581 / 586 / 663 (619 / 624 / 705) |
| x86-64-v3 `f32x4` | 458 / 463 / 531 -> **424 / 424 / 494** (453 / 457 / 528) | 617 / 623 / 689 -> 584 / 587 / 650 (618 / 625 / 683) |
| `simd128` (`wasm2wat`, instructions in the loop) | 835 / 844 / 931 -> **755 / 764 / 851** (834 / 843 / 930) | 1067 / 1076 / 1163 -> 987 / 996 / 1083 (1067 / 1076 / 1163) |
| TurboFan, shipped module (instructions / carried slots, all 5 blocks) | 519/23, 525/24, 564/23 -> **461/14, 469/15, 502/15** (504/18, 509/18, 542/17) | 690/41, 694/40, 737/42 -> 664/44, 672/45, 719/44 (690/43, 694/44, 737/42) |

Named increases: TurboFan's unarmed ramping loops carry 2 to 5 more slots than the base's ramping
loop (they are 18 to 26 instructions shorter); the armed ramping loops carry 0 to 4 more. No
multiband row is held by the V8 spill gate. Whole function: `process_bank<f32x8>` 5,924 -> 9,995
x86 instructions, `<f32x4>` 7,426 -> 11,455; `simd128` `process_bank<f32x4>` 20,515 -> 27,411;
TurboFan 14,023 -> 18,852. The scalar `process` (x86) has branching frame loops (the scalar
branching smoother) that a loop-range count does not separate cleanly; its settled DualMono loop
reads 515 -> 441, not used for D3.

**Gate 4, p50** (scratch `w1455/w1455_bench.rs`, not committed; 48 kHz, 128-frame blocks,
DualMono, defaults, per-block `Instant`; live and one-lane-silent: 50 warm + 2,000 timed blocks x
2 instances; silent tail: 20 live then 400 timed silent blocks x 10 instances, through arming and
rest). One invocation, `taskset -c 31`, load 2.1, order warmup base, warmup head, r1 base, r1 head,
r2 head, r2 base; not retried. ns per block, r1 / r2:

| lanes | live base | live head | one silent base | one silent head | silent tail base | silent tail head |
|---|---|---|---|---|---|---|
| scalar | 6,182 / 6,181 | **5,771 / 5,761** (-6.6 %, -6.8 %) | n/a | n/a | 6,182 / 6,172 | **5,871 / 5,841** (-5.0 %, -5.4 %) |
| `Simd4` | 9,518 / 9,337 | **9,118 / 9,137** (-4.2 %, -2.1 %) | 9,528 / 9,328 | 9,177 / 9,178 (-3.7 %, -1.6 %) | 9,538 / 9,328 | 9,178 / 9,178 (-3.8 %, -1.6 %) |
| `Simd8` | 10,320 / 10,310 | **9,779 / 9,789** (-5.2 %, -5.1 %) | 10,319 / 10,310 | 9,849 / 9,839 (-4.6 %) | 10,319 / 10,320 | 9,849 / 9,839 (-4.6 %, -4.7 %) |

**D3 verdict: keep.** Gates 1-3 pass; the live-audio loop is shorter on x86-64-v3 (-31..-40) and
`simd128` (-80); p50 is lower on live audio and on the silent tail at every width.

**Gate 5.** Pass: the spec's `cargo test` (291 passed), `test-debug-a` (1,458 passed),
`conformance_fixtures --check`, lane, workspace and realtime policies, the AArch64 known-defect
self-test, clippy `-D warnings`, fmt. **`cargo doc -D warnings` fails** outside this slice,
in `crates/gate-expander/src/corpus.rs:64` (`clamped_window` links the private `RAMP_FRAMES`, from
#1459's `ffcbba6b7`); not this slice's file; the workspace without `gate-expander` documents
cleanly. AArch64 legs: CI only, not run.

### Batch follow-ups (stream G2 part 1, after the attempt-1 PASS)

From `/home/bl/misofm/submix-verdicts/1455-attempt1.md`.

- **MINOR 1 (root authorized).** Three documents said the multiband runs the counter and the joint
  flush on every frame. Each now states the shipped behaviour: armability is tested once per
  segment; on a segment where neither side can arm, the counter advances once per segment and
  the frame loop has no silence step. `dsp-research/filters.md` (the adopted law; its "a second
  copy would double its `memset_pattern16` calls" clause is replaced by the measured iOS count
  of 0), `docs/rulings/effect-floor-accounting.md` (the `silence_step` row) and decision 15's
  "Root decisions after S0" (#1328 A9).
- **MINOR 2 (root), the Test value corrected.** The spec body's Test value clause says the
  multiband `DIGESTS` cover live and silent-tail frames. They do not cover the dispatch:
  `corpus.rs` drives `lr4_step` with its own counter and never calls `process_block`. The new
  product test `either_channel_and_either_stage_arm_the_crossover` does (attempt 1's record).
  **Wasm digest coverage of the dispatch: none.** The wasm multiband digests (G5,
  `tools/wasm-gate-corpus` `digest_multiband` -> `multiband_compressor::corpus::run_case`) are the
  same `lr4_step` corpus. No browser document runs a multiband; the SDK's `console-evals.mjs`
  renders an unbypassed multiband only to compare it with its bypassed render (no digest), and a
  bypassed multiband never reaches the dispatch (`armable = BYPASS || ...`). Part 2 adds wasm
  coverage to a successor issue.
- **NIT 4.** `Side::silence`'s doc no longer says "Advanced on every frame": it is kept where a
  frame-by-frame count would leave it, frame by frame on a segment where a side can arm and once
  per segment on any other.
- **NIT 5.** The new test's doc says "per-segment arming test" and "segment", matching the
  dispatch (`process_block`, per segment). Doc text only; the test's code and its mutation
  evidence (attempt 1, reproduced by the verifier) are unchanged.
- **NIT 6.** The multiband's iOS `memset_pattern16` count (deliverable 2) is **0**, measured by
  the verifier from the iOS release assembly; there is no multiband row in
  `scripts/check-cross-targets.sh`, which passes on this tree with its rows unchanged.
- **Gates.** All pass on the follow-up tree (x86_64): fmt; workspace clippy `-D warnings` with and
  without `--all-features`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps
  --exclude gate-expander` (gate-expander's own doc failure is part 2's);
  `check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-effect-runtime-policy.sh`;
  test-debug-a 1,462 passed, 0 failed; test-debug-b 890 passed, 0 failed; `conformance_fixtures
  --check`; release `lane`/`math`/`wasm-gates` (G5 `g5_native_digests_match_pins`) 123 passed;
  `run-wasm-gates.sh` (native 144 cases, wasm simd128 144 cases, 0 mismatches; V8 spill ok);
  `check-cross-targets.sh` PASS (known-defect rows unchanged: builtins 5, host-core 4, soft-clip 1);
  the worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`,
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`,
  `test-web-audioworklet.sh` with a private TMPDIR left empty, the V8 spill gate on the named twin):
  all pass. AArch64: CI only.
