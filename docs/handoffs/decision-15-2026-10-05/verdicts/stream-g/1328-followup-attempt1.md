FAIL

# #1328 follow-up, attempt 1: adversarial verdict

- **Reviewed:** `git diff a124b40be 4264bb190` (the checkpoint) and `git diff f6f599d84 f65f80495`
  (`6293440ff`, `8aa7c5ab5`, `5b887edb9`, `37d8583f8`, `f65f80495`) on `codex/d15-stream-g`. The
  #1409/#1451/#1452 commits between them are not in scope. I exported the commits with
  `git archive` to `/tmp/claude-1002/v1328f/` (`tree` = `f65f80495`, `base` = `a124b40be`, and
  scratch copies for mutants and harnesses). I did not build, edit or check out anything in
  `/home/bl/misofm/wt-d15-g`.
- **Host:** AMD EPYC 7313P, rustc 1.97.1, Node v22.23.2. The load average was 7-11 because other
  builds were running. All timings are descriptive only.
- **Why it fails:** one MAJOR. The required `qualification` lint job runs
  `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`, and that step fails on
  this tree with two new private intra-doc links. The fix is trivial, but as committed the branch
  cannot pass its required check. Everything else holds:
  - The bits did not move. My independent differential gives 2,160 of 2,160 scenarios identical,
    base `a124b40be` against head. Ten harness mutants each move scenarios.
  - Every ruled item is implemented as ruled.
  - Every other gate I ran is green.
  - The new tests are red on their named defects.
  - The GitHub body is byte-identical to the spec.
- **For the root's decision:** the residual silent-lane cost of the builtins, about 3 %, does
  **not** meet ruling 2's target ("gone or within noise"). The root must decide; see the section
  "Root decision needed".
- **Evidence:** `/tmp/claude-1002/v1328f/evidence/` (small files only; the build directories are
  deleted).

## BLOCKER

None.

## MAJOR

**M1. The required documentation gate fails (two private intra-doc links).**
`.github/workflows/qualification.yml:460` runs
`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` in the lint job. That job
feeds the `qualification` verdict, which is the only required check on `main`. On `f65f80495` the
step fails:

```
error: public documentation for `InputChainConstants` links to private item `channel_arms`
   --> crates/lane/src/kernels/builtins.rs:542:14
error: public documentation for `silence_skip_block` links to private item `silence_skip_settle`
   --> crates/lane/src/kernels.rs:375:14
error: could not document `lane`
```

- `6293440ff` introduced the `silence_skip_settle` link and `37d8583f8` the `channel_arms` link.
- The same command passes on `a124b40be` and on `f6f599d84`.
- On a copy of `f65f80495` where only those two links are plain code spans, the whole-workspace
  doc build passes, so there is no third error.
- The record's "Gates (this tree)" says every gate is ok, but this gate was not run. The root's
  gate list did not name it either, but the gate is part of the required check.
- Fix: make the two references plain code spans, or point them at public items. Then re-run the
  doc step. (Evidence: `evidence/gates/doc.tail.log`.)

## MINOR

**m1. The m5 restatement of the cost for live audio is still incomplete: a live block that ends on
an exact zero runs the counter's frame loop, and now the scan as well.**

- What the text says now:
  - `docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md:601-605` says
    that on a block of live audio "what remains is one armability test per channel per block and
    one compare of the block's last frame".
  - The spec record (line 1334) says "On live documents the scan cannot cost anything".
  - The `RestThresholds` doc (`crates/lane/src/kernels.rs:44-47`) says the same for live audio.
- Both statements hold only when every lane's last sample is non-zero. A live lane whose block
  ends on an exact `0.0` takes form 3 of `silence_skip_block` (`kernels.rs:370`). That form runs
  the whole-block scan, finds the lane not silent throughout, and then runs the full
  loop-carried counter loop. The doc's gloss "its silence starts inside the block" hides this
  case. The floor doc's "a whole-block zero scan only when some lane ends the block on a zero" is
  accurate, but it does not mention the loop that follows.
- Quantised sources often end blocks on exact zeros: quiet or dithered 16-bit stems, fades and
  near-silent passages.
- Measured (descriptive; `evidence/zz_timing.rs`, `evidence/timing-transition.txt`; builtins dual
  chain at `Simd8`, 128-frame blocks, `taskset -c 30`, 300,000 blocks, warmup plus two rounds,
  loaded host). Rounds 1 / 2, against all-live float noise:

  | case | base `a124b40be` | head `f65f80495` |
  |---|---|---|
  | live, lane 7's last frame `0.0` | 1.172 / 1.161 (1,972 / 1,963 ns) | 1.196 / 1.151 (2,054 / 2,029 ns) |
  | int16 noise at ±2 LSB (62 of 64 blocks end on a zero) | 1.199 / 1.130 | 1.134 / 1.112 |

  So such blocks pay +13 % to +20 %. That was already true at A9. The head adds about 3 % to 7 %
  absolute in the "ends on zero" case, because the scan now runs before the loop.
- The design that root prescribed (scan, then the loop only on a transition) gives this
  behaviour, so the defect is not in the implementation. The documents should state it, and the
  root may want the cheaper exact transition form: a backward trailing-zero scan that stops early
  once every lane that ends silent has met a non-zero frame. That replaces the forward
  loop-carried loop and costs a few frames for a live lane that ends on a zero.

**m2. No test holds the long-block boundary of `silence_armable_holding`.**

- The site is `crates/lane/src/lib.rs:332`, `let long = frames.gt(armed_after);`.
- Mutant MLoff, `frames.gt(armed_after + 1)`, treats a block of exactly `window + 1` frames as
  short. It is **green on every committed test**: lane, parametric-eq and builtins, with
  test-support, 350 of 350 pass.
- It still moves bits. In my differential it moves 3 of 2,160 scenarios (builtins at
  `frames = window + 1`): a lane at rest takes a sample on frame 0 and then `window` zeros, so it
  arms on the last frame, where the unarmed form skips the flush.
- The exactness proof needs `frames <= armed_after` exactly. The existing long-block tests are far
  from that boundary: 32 frames against a window of 8, and 8,192 against 4,096.
- In production it needs a quantum of `window + 1` (3,765 / 4,097 / 7,528 / 8,193 frames), which
  is unlikely but legal.
- Add a `frames == window + 1` case to `input_chain_arming.rs` and to the EQ's long-block test.

## NIT

- **n1. A doc comment is misplaced.** In `tools/wasm-gate-corpus/src/lib.rs:1656-1666` (m4,
  `4264bb190`), `svf_armed_values` was inserted between `meter_peak_values`' doc comment and its
  `fn`. The sample-peak doc now heads `svf_armed_values` as one merged comment, and
  `meter_peak_values` (line 1721) has no doc.
- **n2. Two numbers disagree.** Decision 15 says the EQ's silent-lane scan is "about 1 % of an
  eight-lane EQ block". The record says the EQ penalty is gone (0.996 / 1.009).
- **n3. The record misstates the MG mutant's test value.** The MG row and the test-value sentence
  for G5 `svf_block/armed/impulse_tails` say it catches "a joint test that does not take the
  larger magnitude". Natively, that mutant is also red on the existing `g4_pair_law_holds_at_every_width`
  and `g4_pair_law_cases_at_every_width` (and on G6). The G5 case's own value is the wasm leg:
  an `i32x4.max_u` or armed-compare lowering that differs from native. The sentence should say so.
- **n4. The record omits one red test.** ML (the long-block term dropped) is also red on the
  existing `builtins/tests/stage.rs::scalar_stage_is_bit_identical_to_reference_recurrence`, and
  the record does not list it.
- **n5. New doc lines exceed 100 columns.** `crates/lane/src/kernels/builtins.rs:542, 820, 935,
  1075, 1330` and `crates/lane/src/kernels.rs:1092` (153 columns). rustfmt does not wrap
  comments.
- **n6. The dual armed pair's ceiling is 11, not the ruling's "12".** I accept 11. The ruling's
  binding principle is "a ceiling equal to today's measured counts", and the gate measures 11 on
  this tree (`ok dual armed depth-2 pair ... (reported, ceiling 11) ... 11 carried slots`).

## Root decision needed: the residual silent-lane cost of the builtins

Ruling 2 set the target as "gone or within noise". It is **not met** for the builtins.

- The implementer's harness measured 1.030 / 1.035 (one silent lane against all live, eight-lane
  bank) against 1.001 / 1.001 for a timing-only build with no scan.
- My own harness agrees: lane 7 silent gives 1.041 / 0.980 at head (warmup 1.030), against
  1.305 / 1.292 at base. Round 2 at 0.980 shows the host noise.
- That is about 55-60 ns per eight-lane block: two `Lane` operations per frame per channel (`eq`,
  `mask_and`, plus the load) over 2 × 128 frames.
- This is the ruled design's own cost, not an implementation slip.
- The EQ's cost is within noise.

Options for the root:
- (a) Accept about 3 % per bank-block that has a silent or padding lane. The builtins are 69 of
  the strip's 307 lane-ops, so this is about 0.7 % of a strip.
- (b) Authorize a bitwise-OR accumulation `Lane` op. One folded load-or per frame, then one
  compare, halves the cost.
- (c) Authorize fusing the zero-tracking into the unarmed body, whose SVF recurrence is
  latency-bound, so a short independent chain may cost nothing. That changes the floors.

Options (b) and (c) are cross-cutting lane or floor changes that were not ruled, so the
implementer was right not to make them unilaterally. The record lists the residual as an open
item.

## Requirements checked (rulings file, binding)

| item | verdict |
|---|---|
| 1. App-shape cost accepted and recorded with numbers and cause (218 vs 196 instructions; 397 vs 186 memset calls), #1451 referenced | done; #1451 is open on GitHub |
| 2. Cheaper exact counter (scan, then the loop only on transitions), in the shared `silence_skip_block` (builtins and EQ) | built and exact (differentials below); the residual is reported; the target is not met for the builtins (root decision) |
| 3. Spill gate: armed rows reported with ceilings at today's counts; the four unarmed rows held at zero; a self-test for the ceiling | done; measured dual armed tail 2, dual armed pair 11, mono armed 0/0/0; held unarmed rows 0; self-test mutants F6/F7/F8/MF re-run: red |
| 4. `track_delay` re-pin, individually with its reason | verified independently (below) |
| m1 (#1427 order and its disproof) | recorded; #1427 is CLOSED (`not_planned`) on GitHub |
| m5 | decision 15, `kernels.rs` and `filters.md` corrected; `1.53e-9` / `1.68e-9` recorded; still incomplete for live blocks that end on a zero (m1 above) |
| m7 | `3.744e-10` and `3.104e-7` reproduced by re-running the attempt-5 verifier's `bounds.py` (96 kHz, four 10 Hz shelves) |
| Floors | EQ 27 (24·active + 3; 153 for six or refused; 177 armed) and strip 307 = 69 + 27 + 81.5 + 129.5, 10.372 cycles: the arithmetic checks; `bench floor` and `test-console-benchmark.sh` are green |
| Stale `svf_step_armable` links | none left |
| GitHub #1328 body | `gh issue view 1328 --json body \| jq -j .body` is byte-identical to the spec at `f65f80495` (sha256 `c764c703…f56`). Note: the branch is not pushed (batch mode), so the evidence commit is not upstream yet. |

## Bit identity (independent)

- **Differential.** `evidence/zz_verifier_diff.rs` is a different harness from the implementer's.
  It runs random per-lane programs (signed zeros, noise at 1 down to 2e-39, impulses at segment
  starts and ends, NaN/inf/1e30 in silence, sprinkled exact zeros, slow tones) with segment
  lengths around the window (`window ± 1`, `2·window + 3`). Coverage:
  - Builtins: every entry point (plain, trim ramp, filter ramp, mixed elision with three plans),
    dual and mono, at `f32`, `Simd4` and `Simd8`.
  - Builtins conditions: windows of 3-300 frames, block lengths of 1-128, random trims, random
    state words (band pairs, `±0`, `FLUSH_EPS`, `REST_EPS`, NaN) and random counters up to `2^24`.
  - The EQ scalar effect at the four launch rates, quanta 64 / 128 / 1,000 / 8,192, blocks of
    1 to `2q + 3`, with mid-run restores of band pairs and valid counters.
  - The EQ native bank, dual and collapsed, with padding prefixes, restores and target ramps.
- **Result.** Base `a124b40be` against head: **2,160 of 2,160 digests identical**
  (`vdiff-base.txt` / `vdiff-head.txt`).
- **Sensitivity.** Each mutant was applied in a scratch copy and the harness re-run
  (`hm-summary.txt`):

  | mutant | scenarios moved |
  |---|---|
  | builtins never arm | 1,419 |
  | EQ never arms | 505 |
  | settled form without its select | 1,349 |
  | no long-block term | 52 |
  | long-block off by one | 3 |
  | holding AND long | 1,812 |
  | no saturation | 397 |
  | no scan remainder | 1,030 |
  | holding without `ic2` | 61 |
  | EQ arms from the left channel only | 145 |

- **Other identity gates (green).**
  - `g5_native_digests_match_pins` (release).
  - `conformance_fixtures --check`.
  - `check-builtins-fixtures.sh` (50 files).
  - `graph_fixture --check`.
  - `check-graph-determinism.sh` (100 of 100).
  - `audit unfused-fma conformance` (`svf_block` silence 0 unfused mismatches).
  - The browser `expected.json` digests (`check-browser-expected-resources.py --artifacts`).
  - No fixture file changed in the diff.
- **`track_delay`.** I dumped the zero-delay canonical text at head with
  `GraphCompiler::evidence` (`evidence/canon-head.txt`). It hashes to the new pin
  `bf2dfd6c…7b10d8b3`. Line 685 (`estimate`) is the only row that carries `17712` and
  `159847 159847`. Reversing those three tokens to `8496` / `150631 150631` hashes to
  `bb25028730eb702d827953f28534f549eadf6b09c04e84bccdd2a80ffbfc40bf`, the A9 pin. The
  difference 9,216 = 9 × 128 × 8. The re-pin is individual and its reason is correct.

## Deviations the implementer reported

- **Builtins silent-lane residual of about 3 %:** this is not a finding against the
  implementation. It needs a root decision (above).
- **Spill-gate ceiling 11, not 12:** acceptable (n6).
- **Outlined `silence_skip_settle` and the lowered iOS ratchet:**
  - The counts reproduce on this host exactly: builtins 40, parametric-eq 47, multiband 566
    (`check-cross-targets.sh` PASS).
  - The judge passes a count below its ceiling, and lowering the row to the measured count is
    the correct ratchet direction: it makes the next rise visible. The rustc version is pinned,
    so CI measures the same count.
  - The outlining is a plain generic `#[inline(never)]` function. It does no allocation, takes no
    lock and makes no syscall. `check-realtime-policy.sh` and `test-realtime-policy.sh` are
    green.
  - It moves no bit (the differentials above).
- **908 / 908:** consistent with my 2,160 / 2,160.
- **Derivation of the m5, m7, floor and `track_delay` numbers:** verified as above.

## Mutation runs (re-done)

Each mutant ran in a scratch copy. The command was
`cargo test -p lane -p parametric-eq -p builtins` with test-support (350 tests), unless stated
otherwise. Log: `evidence/mut-run.log`.

| mutant | red tests |
|---|---|
| F1 settled form drops the select | 12, including `g4_silence_skip_block_is_the_frame_loop` |
| F2 no `2^24` saturation | **only** `g4_silence_skip_block_is_the_frame_loop` |
| F3 no transition check | 9, including g4 |
| F4 scan drops its remainder | **only** g4 |
| F5 scan drops two chains | **only** g4 |
| MK2 (clamped: frame `min(i, frames-1)`'s threshold) | **only** `g2_skewed_cascade_arms_each_section_on_its_own_frame`. My unclamped MK panics out of bounds instead, which is not a bits test. |
| MI (all dual entries) and MIb (`input_chain_block` alone) | **only** `the_right_channel_arms_its_joint_flush_alone`, `any_state_word_in_the_band_arms_the_block` |
| MW `svf_state_held` ignores `ic2` | **only** `any_state_word_in_the_band_arms_the_block` |
| ML long-block term dropped | `a_block_longer_than_the_window_arms_from_rest`, the EQ `a_block_longer_than_the_window_renders_as_its_quantum_sized_parts`, and the existing `scalar_stage_is_bit_identical_to_reference_recurrence` |
| MLoff long-block off by one | **none** (m2) |
| ME EQ arms from the left only | **only** `the_joint_flush_arms_for_whichever_channel_and_section_holds_the_band` |
| MS EQ holding reads section 0 only | 6, including the new test, `the_collapsed_body_arms_its_joint_flush` and existing exact-rest tests |
| MM EQ collapsed body never arms | **only** `the_collapsed_body_arms_its_joint_flush` |
| MD descriptor declares no scratch | **only** `the_rest_planes_are_the_scratch_the_descriptor_declares` |
| MNever (builtins never arm) | 6 |
| MG (`a1 < rest`), release, `--no-fail-fast` | `g5_native_digests_match_pins`, G6 canonical-under-FTZ, and the existing `g4_pair_law_*` (n3) |

The spill self-test mutants were applied to a copy of the script: F6 ceiling ignored (99), F7
ceiling rows as info, F8 off by one, and MF rows plus anchors form-blind all go red. MF's halves
alone stay green, as recorded. F9 (ceiling 10 on the real module) follows from the measured 11.

## Test value (one sentence each)

- `g4_silence_skip_block_is_the_frame_loop`: a settled skip form that does not saturate at `2^24`,
  loses the scan's remainder frames or drops scan chains. No other lane, EQ or builtins test catches
  these (F2, F4 and F5 are red only here).
- `g2_skewed_cascade_arms_each_section_on_its_own_frame`: a skewed cascade whose section `k` reads
  frame `i`'s rest threshold instead of frame `i - k`'s (MK2 is red only here).
- `input_chain_arming::the_right_channel_arms_its_joint_flush_alone` and
  `any_state_word_in_the_band_arms_the_block`: an input-chain body that decides armability from
  channel 0 alone, or a holding mask that skips `ic2` (MI and MW are red only here).
  `a_block_longer_than_the_window_arms_from_rest`: the long-block term dropped. This is shared with
  an existing builtins stage test, but it is the only lane-level test that holds it on every entry
  point.
- EQ `the_joint_flush_arms_for_whichever_channel_and_section_holds_the_band`: an EQ that arms from
  the left channel alone (ME is red only here).
- EQ `a_block_longer_than_the_window_renders_as_its_quantum_sized_parts`: the EQ's long-block term
  dropped (the only EQ test red on ML).
- `mono_collapse::the_collapsed_body_arms_its_joint_flush`: a collapsed EQ body that never takes
  the armed form (MM is red only here).
- `the_rest_planes_are_the_scratch_the_descriptor_declares`: a descriptor that under-declares the
  rest planes (MD is red only here).
- G5 `svf_block/armed/impulse_tails`: a wasm lowering of `i32x4.max_u` or of the armed compare that
  differs from native. Natively the law defect is already caught by `g4_pair_law_*` (n3).
- Spill self-test, the two-forms cases and the three ceiling cases: rows or anchors that ignore the
  form, and a ceiling that is ignored, not failed closed, or compared off by one.

## Gates run (head export, x86-64; AArch64 is CI-only and was not emulated)

- **Green:**
  - gate-4 set (14 packages, spec features): 879 passed, 0 failed.
  - `audit`, `graph-compiler`, `builtins-compiler`, `graph`, `host-core`, `host-web`, `capi`,
    `bench`: 900 passed, 0 failed.
  - `cargo test --release -p lane -p math -p wasm-gates --features math/lane`.
  - `conformance_fixtures --check`, `check-builtins-fixtures.sh` (50 files),
    `audit unfused-fma conformance`, `graph_fixture --check`, `check-graph-determinism.sh`.
  - Policy scripts: `check-lane-policy`, `check-dsp-research`, `check-workspace-policy`,
    `check-realtime-policy`, `test-lane-policy`, `test-realtime-policy`, `check-unfused-seal`
    (and `--self-test`), `check-builtins-policy`, `check-graph-policy`,
    `check-effect-runtime-policy`, `check-rack-policy`, `check-session-policy`,
    `check-host-core-policy`, `check-bench-policy`, `check-env-vocabulary`,
    `check-console-benchmark-fixture`, `check-realtime-audit-leak`,
    `check-artifact-evidence-leak`, `check-builtins-listening`, `check-test-support-ci.py`,
    `check-ci-path-routing.py`, `check-script-reachability.py` (in a scratch git repo).
  - `cargo fmt --all -- --check`.
  - CI clippy (`--workspace --all-targets --all-features -D warnings`).
  - `cargo test -p bench floor`, `test-console-benchmark.sh`.
  - Spill and known-defects `--self-test`.
  - `run-wasm-gates.sh` (spill rows as above), `check-cross-targets.sh`.
  - Worklet chain: `build-web-audioworklet.sh --named-twin`,
    `check-web-audioworklet.sh --without-metadata-regeneration`,
    `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`,
    `strip-wasm-names.py check`, `test-web-audioworklet.sh`.
- **Red:** `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` (M1).
- **Acked-batch question:** not applicable; the change adds no queue.
- **Realtime:** the render paths gain only stack arrays, compares and one outlined plain function.
  No allocation, lock, syscall or I/O.
- **Naming:** `RestThresholds`, `ArmedRest`, `UnarmedRest`, `svf_state_held`,
  `silence_armable_holding`, `silence_skip_settle`, `channel_arms`,
  `REST_PLANE_BYTES_PER_FRAME`: no version suffix and no prefix.
