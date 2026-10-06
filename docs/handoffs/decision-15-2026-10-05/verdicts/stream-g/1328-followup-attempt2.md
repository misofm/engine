PASS

# #1328 follow-up, attempt 2: adversarial verdict

- **Reviewed:** `91724e66c` (parent `e2e6d0e7d`) and `0725a8949` on `codex/d15-stream-g`, against
  the root's six rulings for attempt 2, the attempt-1 verdict
  (`submix-verdicts/1328-followup-attempt1.md`) and the spec's "### Follow-up" and "Follow-up
  attempt 2" sections.
- **Method:** I exported the commits with `git archive` to `/tmp/claude-1002/v1328g/`:
  - `tree` = `0725a8949`, used for the gates;
  - `base` = `a124b40be`, the A9 bits;
  - `pre` = `e2e6d0e7d`, used for timing;
  - scratch copies for the harnesses and the mutants.
  I did not build, edit or check out anything in `/home/bl/misofm/wt-d15-g`.
- **Host:** AMD EPYC 7313P, rustc 1.97.1, Node v22.23.2. Other agents' builds ran during the run
  (load average 4 to 42), so all timings are descriptive.
- **Verdict:** PASS. There is no BLOCKER and no MAJOR finding.
  - Every binding ruling is implemented and its evidence holds.
  - Every gate I ran is green, including the doc gate that failed attempt 1.
  - My independent differential against `a124b40be` is identical:
    - 3,600 of 3,600 whole-chain scenarios;
    - 1,605,235 of 1,605,235 direct `silence_skip_block` scenarios.
  - Every bits mutant of the new scan is red on a committed test. The two cost-only mutants
    (cost only, no bit change) move nothing, as recorded.
- **Remaining findings:** one MINOR (m1) and two NITs.
  - m1: the backward scan's trailing count does not saturate at 2^24. It therefore differs from
    the frame loop on blocks of 2^24 + 3 frames or longer, which are legal but absurd.
  - The NITs are stale test docs.
- **Evidence:** `/tmp/claude-1002/v1328g/evidence/` (small files only; the build directories are
  deleted).

## BLOCKER

None.

## MAJOR

None.

## MINOR

**m1. The backward scan is not the frame loop for a trailing zero run longer than 2^24 + 1
frames: `scanned` does not saturate.**

- **Site:** `crates/lane/src/kernels.rs:356`, `:367` and `:382`.
  - `let mut scanned = 1;` infers `i32`.
  - Both leave paths write `L::splat(scanned as f32)`.
- **What the frame loop does:** `silence_block` adds `run + 1` per zero frame, which saturates at
  2^24 (`2^24 + 1` rounds back to `2^24`). So a lane that is non-zero on frame `k` and zero after
  it ends at `min(F - 1 - k, 2^24)`.
- **What the scan does:** it writes `(F - 1 - k) as f32`, which is not clamped.
- **Measured:** `evidence/zz_v2_skip.rs`, extreme part, `evidence/skip2-{base,head}.txt`. Width
  `f32`, input non-zero on frame 0 and zero after it:

  | block length | head | frame loop and base `a124b40be` |
  |---|---|---|
  | 2^24 + 3 frames | `0x4b800001` (16,777,218) | `0x4b800000` (16,777,216) |
  | 2^24 + 4 frames | 16,777,220 | 16,777,216 |

  Blocks of 2^24 + 1 and 2^24 + 2 frames, and every lane that is zero throughout (which takes
  `silence_advance`, with its `min`), still agree. The base has 0 mismatches.
- **Reachable, though absurd:**
  - The session refuses only a zero quantum (`crates/session/src/validate.rs:48`).
  - The builtins' all-identity bodies call `silence_skip_block` on every block, whatever its
    length (`crates/lane/src/kernels/builtins.rs:1393`, `:1459`, `:1650`, `:2093`). A strip with
    neither filter engaged takes that path.
  - The graph runtime already treats blocks longer than 2^24 frames as legal input for this exact
    reason: `BANK_METER_MAX_FRAMES = 1 << 24` makes the meter pass decline them
    (`crates/graph/src/runtime.rs:3663`, `:3787`).
  - The EQ cannot reach the scan with such a block: a block longer than the window always arms
    and runs `silence_block`.
- **Effect:**
  - No audio bit moves. Both counter values are at or above any window, and the next zero block's
    `silence_advance` clamps the word back to `2^24`.
  - But the counter word is non-canonical. The EQ's own decoder refuses such a word ("a count past
    saturation", `crates/parametric-eq/src/lib.rs:2744`). The builtins carry it word for word
    across a plan swap.
  - The function's doc says the counter is left "exactly where `silence_block` would leave it",
    and the trailing count is "an exact integer" (`kernels.rs:309-312`, `:320`). The ruling says
    "exact". Both statements are true only up to 2^24 + 2 frames.
  - In debug builds, `scanned` would also overflow `i32` at 2^31 frames.
- **Fix:** clamp at the conversion with `L::splat(scanned.min(1 << 24) as f32)`, `scanned: usize`.
  This is one scalar `min` on the leave path only. Hold the clamp with an `f32`-width
  `silence_skip_block` case at 2^24 + 3 frames against the frame loop (64 MB, quick in release).
  Alternatively, state the bound in the doc. The no-shortcuts rule prefers the clamp.
- **Why only MINOR:** no audio bit changes, the EQ is unaffected, and production quanta are 128 to
  8,192 frames.

## NIT

- **n1. The g4 test docs still describe attempt 1's three forms.**
  - `crates/lane/tests/g4_flush.rs:550` says "every lane mix reaches each of
    `silence_skip_block`'s three forms".
  - The doc of `g4_silence_skip_block_is_the_frame_loop` (`:641-645`) says "in each of its forms:
    ... live on its last frame (counter saturation included), and a block where some lane's
    silence starts inside it".
  - The function now has two forms: live, and the backward scan. The input categories still reach
    the scan (my mutants confirm), but the doc names forms that no longer exist.
- **n2. The doc's description of the frame loop omits the saturation (part of m1).**
  `kernels.rs:309-311` says the loop leaves "the count of the block's trailing zero frames" on a
  lane that is non-zero earlier. The loop saturates that count at 2^24 too, not only the
  zero-throughout lane's count.

## Rulings checked (binding)

| ruling | verdict |
|---|---|
| 1. M1: `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` passes | **done.** rc 0, no warning, 38 crates documented (`evidence/gates/doc.log`). `cc2b4e0c1` removed the `silence_skip_settle` link; D3 removed `channel_arms`' public-doc link; the new doc names the outlined function as a code span. |
| 2. Backward trailing-zero scan, exact, bit identity against `a124b40be`, scan mutants, measured numbers, restatements | **done.** Exact for every block of up to 2^24 + 2 frames (m1 above). Bit identity: see "Bit identity". Mutants: see "Mutation runs". Numbers: see "Timing". Decision 15 (`live-updates-...-2026-10-05.md:613-626`), the `RestThresholds` doc (`kernels.rs:43-49`), the floor doc rows and #1328's record now state the backward scan and its cost. The "within noise" claim for a block that ends on an exact zero reproduces independently. |
| 3. m2: a test holds the long-block boundary | **done.** MLoff (`lib.rs:316`, `frames.gt(armed_after + 1)`) is red on exactly `a_block_longer_than_the_window_arms_from_rest` and `a_block_longer_than_the_window_renders_as_its_quantum_sized_parts`, and green on every other test. It moves 3 of my 3,600 scenarios. |
| 4. NITs n1-n5 | **done.** n1: the sample-peak doc is back on `meter_peak_values`. n2: decision 15 gives the EQ as within noise. n3 and n4: the MG and ML rows and the G5 sentence are corrected. n5: no line added by the follow-up exceeds 100 columns; every remaining long line in those files predates the follow-up or comes from #1409 (by blame). |
| 5a. About 3 % for an 8-lane builtin bank with one silent lane, recorded with numbers | **done** (record, deviation (a); decision 15). My run read 1.041 / 1.055 at head against 1.053 / 1.075 at `e2e6d0e7d`'s forward scan. That is the same cost; the backward scan neither adds nor removes cost here. |
| 5b. Spill ceiling for the dual armed pair 11, then 12 after #1451, with its reason | **done.** `run-wasm-gates.sh` reports `dual armed depth-2 pair ... (reported, ceiling 12): 219 instructions ... 12 carried slots`. The reason is #1451's (splatted `FLUSH_EPS`, armed loop only). |
| 6. Attribution: 218 vs 196 from the loop structure; #1451 fixed iOS; #1454 owns the browser gap | **done.** The record and decision 15 agree with #1451's record (indexed `vmovdqu` 4 / 26 / 26, a bounds branch; 2,122 -> 16 calls). Decision 15's "about 2 % to 5 %" matches #1454's "+2.0 % to +4.9 % p50". |
| GitHub #1328 body equals the spec at `0725a8949` | **byte-identical** (`gh issue view 1328 --json body \| jq -j .body`; both sha256 `33afa39a...814eb`, 130,295 bytes). The issue is OPEN; the branch is not pushed (batch mode). |

## Scrutiny of the scan (`silence_skip_settle`, `kernels.rs:347-389`)

- **Proof by reading.**
  - `open` holds the lanes that were zero on the last frame and are still zero on every frame
    scanned. `scanned` counts the zero frames after the frame being tested.
  - Groups are `rchunks_exact(4W)` over frames `0..F-1`, so the last frame is left out. Inside a
    group, frames resolve latest first. The remainder is the block's first `(F-1) mod 4` frames,
    walked by `rchunks_exact(W)`, so it is also backwards.
  - A lane that meets a non-zero frame (NaN and subnormals count as non-zero; `-0.0 == 0.0`, as in
    `silence_step`) takes `scanned`, which is the frame loop's trailing count. A lane still open
    after frame 0 takes `silence_advance`. Every other lane takes `+0.0`.
  - The early exit is taken only when `open` is empty. After that, no lane can leave, `trailing`
    is final, and the final select returns it. So B9 / S9 (no early exit) is equivalent by
    construction, and so is S18 (group test over every lane, not only the open ones).
  - Valid counters are integers in `[0, 2^24]`; the EQ's decoder enforces this. On them,
    `silence_advance` equals the saturating loop.
- **Boundaries tested exhaustively** (`evidence/zz_v2_skip.rs`):
  - Every per-lane last-non-zero position, including "zero throughout", for every lane
    combination:
    - `f32` for blocks of 1-200 frames;
    - `Simd4` for 1-22 frames (1,431,243 scenarios; every group and remainder shape, every mix of
      trailing counts inside one group);
    - `Simd8` for 1-24 frames, every "one lane differs" case plus 4,000 random cases per length.
  - Blocks of 31-33, 63-65, 127-131, 1,000, 4,096, 4,097 and 8,192 frames.
  - Signed zeros, NaN, ±inf and subnormals before the last non-zero frame.
  - Counters at 0, 1, 2, 5, 127, 4,095, 4,096, 16,776,000, 16,777,100, 16,777,214, 16,777,215 and
    16,777,216.
- **Results:**
  - Base and head both have 0 mismatches against an independent scalar frame loop in the harness,
    with identical per-section digests.
  - The extreme part is the only difference (m1).
- **Realtime:**
  - The loop is bounded by `frames / 4 + 3` iterations. It uses stack masks, `array::from_fn` and
    no allocation, lock, syscall or call.
  - The `Simd8` instance disassembles to the claimed group: four `vcmpeqps` with folded loads,
    three `vandps`, one `vtestps` and the branch. The leave path is `vcvtsi2ss`, `vbroadcastss`,
    `vandnps` and `vblendvps` per frame, with no `call` (`evidence/settle-simd8.asm`).
  - `check-realtime-policy.sh` and `test-realtime-policy.sh` are green.
- **One shape across targets:** the function is one generic `#[inline(never)]` function over
  `L: Lane` with no `cfg`. The iOS `memset_pattern16` counts are unchanged: builtins 5, host-core
  4, soft-clip 1, true-peak-limiter 6 (`check-cross-targets.sh` PASS).

## Bit identity (independent)

- **Whole chain:**
  - Harness: the attempt-1 verifier's `zz_verifier_diff.rs`. I adapted it to D3 myself (the
    `InputChainConstants` import and field removed) and ran it at **scale 5** (seeds beyond the
    implementer's scale 3).
  - Coverage: every builtin entry point, dual and mono, at `f32`, `Simd4` and `Simd8`; the EQ
    scalar effect with restores at four rates; the EQ bank, dual and collapsed.
  - Result: base `a124b40be` against head, **3,600 of 3,600 digests identical**
    (`evidence/vdiff5-{base,head}.txt`).
- **Direct:** **1,605,235 of 1,605,235** `silence_skip_block` scenarios identical in result and
  digest. Only the 2^24 + 3 and 2^24 + 4 extreme blocks differ (m1).
- **Other identity gates (green):**
  - `g5_native_digests_match_pins` (release, 120 passed);
  - `conformance_fixtures --check`;
  - `check-builtins-fixtures.sh`;
  - `audit unfused-fma conformance`;
  - `graph_fixture --check`;
  - `check-graph-determinism.sh`;
  - `check-browser-expected-resources.py --artifacts`.

## Mutation runs (re-done; `evidence/mutate.py`, `evidence/mut/`)

- **Command:** each mutant was applied to a scratch copy of head, then
  `cargo test --release --no-fail-fast -p lane -p parametric-eq -p builtins` ran with all three
  test-support features (350 tests).
- **Measures:** the tests that turn red; the whole-chain scenarios that move (of 3,600 against
  base); and the direct harness's mismatches against its scalar loop.
- **Baseline:** 350 passed, 0 moved, 0 mismatches.

| mutant | moved | direct mismatches | red tests |
|---|---|---|---|
| MLoff `frames.gt(armed_after + 1)` | 3 | 0 | **only** the two extended long-block tests |
| S1 remainder scanned forwards | 1,590 | 218,718 | 5, incl. `g4_silence_skip_block_is_the_frame_loop` |
| S2 group leave count one short | 2,445 | 1,578,234 | 5, incl. g4 |
| S4 group test "any zero" (`mask_or`) | 2,388 | 1,497,443 | 8, incl. g4 |
| S5 groups taken from the block's start | 2,325 | 1,577,562 | 9, incl. g4 |
| S6 zero-throughout lane advanced `frames - 1` | 3,152 | 266,787 | 13, incl. g4 |
| S16 group resolve stops at the first leaving frame | 1,338 | 1,437,967 | 3, incl. g4 |
| S19 remainder count incremented before the select | 2,121 | 408,037 | 5, incl. g4 |
| S13 `trailing` starts from the old counter | 1,991 | 317,031 | 8, incl. g4 |
| S9 no early exit (cost only) | 0 | 0 | none (equivalent, as recorded for B9) |
| S18 group test over every lane (cost only) | 0 | 0 | none (equivalent) |

These agree with the implementer's B1-B8 (each red on g4) and B9 (equivalent).

## Test value (one sentence each)

- `lane/tests/input_chain_arming.rs::a_block_longer_than_the_window_arms_from_rest` (extended with
  `BOUNDARY_WINDOW`, a 32-frame block against a window of 31): a long-block term off by one
  (`frames > armed_after + 1`) treats a block of exactly `window + 1` frames as short and leaves
  the tail. MLoff was green on every committed test at attempt 1. It is now red here, on every
  builtin entry point, and in the EQ test below only.
- `parametric-eq/tests/exact_rest.rs::a_block_longer_than_the_window_renders_as_its_quantum_sized_parts`
  (extended with a 4,097-frame block against 128-frame parts): the same off-by-one through the
  EQ's own `Channel::arms` at a launch window (4,096 at 48 kHz), through the effect contract. The
  `band_word == 0` check on the parts keeps it from passing vacuously.
- **No new scan test, correctly:** every bits mutant of the backward scan (B1-B8 and my S1-S19) is
  red on the existing `g4_silence_skip_block_is_the_frame_loop`. That test does not reach m1's
  2^24 + 3 boundary.

## Timing (descriptive; `evidence/timing-a2.txt`, `evidence/zz_timing2.rs`)

- **Setup:**
  - Builtin input chain at `Simd8`, 30 Hz high-pass into 18 kHz low-pass, 128-frame blocks,
    300,000 blocks per point.
  - One invocation per build, one warmup and two measured rounds, `taskset -c 30`, load 4.2-4.9.
  - The attempt-1 verifier's harness, adapted to D3, with two cases added: a lane whose last 40
    frames are zero, and every lane ending on one zero.
- **Results:** ratio to all-live float noise, rounds 1 / 2.

| case | `e2e6d0e7d` (forward scan, then loop) | head `0725a8949` |
|---|---|---|
| int16 noise at ±2 LSB | 1.166 / 1.171 | 1.019 / 1.009 |
| lane 7 ends on one zero | 1.211 / 1.209 | 1.003 / 1.004 |
| lane 7's last 40 frames zero | 1.209 / 1.256 | 1.015 / 1.023 |
| every lane ends on one zero | 1.209 / 1.205 | 1.013 / 1.045 |
| lane 7 silent | 1.053 / 1.075 | 1.041 / 1.055 |

- All live: 1,668 / 1,668 ns per block before, 1,701 / 1,709 after (host noise).
- A block that ends on an exact zero drops from +17 % to +26 % to within about 1 % to 5 %. This
  agrees with the record (1.005-1.008).
- The silent-lane cost is unchanged by the backward scan. It reads about 4 % to 5 % in this run,
  against the record's "about 3 %"; the run-to-run spread at that level is as large as the
  difference.

## Gates run (head export, x86-64; AArch64 is CI-only and was not emulated)

All the gates below are green (`evidence/gates/summary*.txt`).

- **Documentation:** `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
- **Tests:**
  - gate-4 / test-debug-b set (14 packages, spec features): **879 passed, 0 failed**.
  - `audit`, `graph-compiler`, `builtins-compiler`, `graph`, `host-core`, `host-web`, `capi` and
    `bench` with test-support: **937 passed, 0 failed**.
  - `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`:
    **120 passed** (G5 and G6 green).
  - `filter_liveness` (release).
- **Fixtures and identity:** `conformance_fixtures --check`, `check-builtins-fixtures.sh`,
  `audit unfused-fma conformance`, `graph_fixture --check`, `check-graph-determinism.sh`.
- **Lint:**
  - `cargo fmt --all -- --check`.
  - `cargo clippy --locked --workspace --all-targets --all-features -D warnings`, and the same
    without `--all-features`.
- **Benchmarks:** `cargo test -p bench floor`, `test-console-benchmark.sh`.
- **Self-tests:** spill and known-defects `--self-test`.
- **Policy scripts:** `check-`/`test-lane-policy`, `check-dsp-research`,
  `check-`/`test-workspace-policy`, `check-`/`test-realtime-policy`, `check-unfused-seal` (and
  `--self-test`), `check-`/`test-builtins-policy`, `check-graph-policy`,
  `check-effect-runtime-policy`, `check-rack-policy`, `check-session-policy`,
  `check-host-core-policy`, `check-bench-policy`, `check-env-vocabulary`,
  `check-console-benchmark-fixture`, `check-realtime-audit-leak`,
  `check-artifact-evidence-leak`, `check-builtins-listening`,
  `check-parametric-eq-render-contract`, `check-test-support-ci.py`.
- **Wasm and cross-target:**
  - `run-wasm-gates.sh`, with the spill rows as in ruling 5b and the held unarmed rows clean.
  - `check-cross-targets.sh` PASS.
- **Worklet chain:** `build-web-audioworklet.sh --named-twin`,
  `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`,
  `strip-wasm-names.py check`, `check-web-audioworklet-v8-spill.py`, `test-web-audioworklet.sh`.
- **Not run:** `check-script-reachability.py` and `check-ci-path-routing.py`. The two commits
  touch no script or workflow.
- **Acked-batch question:** not applicable; the change adds no queue.
- **Naming:** `silence_skip_settle` and the new test constants (`BOUNDARY_WINDOW`, `WINDOW_48K`)
  carry no version suffix and no prefix.
