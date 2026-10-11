PASS

# #1455 attempt 1 -- adversarial verdict

Commit `db4078c3a` (parent `88c62e7f5`), branch `codex/d15-stream-g2`. Reviewed `git diff 88c62e7f5 db4078c3a`
(`crates/multiband-compressor/src/lib.rs`, `crates/multiband-compressor/tests/product.rs`, the spec) against
the spec, `AGENTS.md`, decision 15, STREAMS (stream G owns the multiband `run_segment` and its
per-segment dispatch for #1455; #1409 and #1451 are on `main` and in the branch), the owner principle,
the owner memories skip-work-on-silence, no-scalar-where-vector-possible and two-percent-allowance, and
root's standing rulings. Everything was built from `git archive` exports under `/tmp/claude-1002/v1455`;
the worktree was not touched.

## Findings

No BLOCKER. No MAJOR.

### MINOR

1. **Three documents outside the authorized paths now say the opposite of the code, and the attempt
   record does not report them to root.**
   - `dsp-research/filters.md:43`: "The multiband compressor runs the counter and the joint flush on
     every frame, in one body: ... a second copy would double its `memset_pattern16` calls".
   - `docs/rulings/effect-floor-accounting.md:148` (the `silence_step` cost row): "the multiband
     compressor runs the counter and the joint flush on every frame".
   - `docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md:618-619` (Root
     decisions after S0, #1328 A9): "(the multiband compressor runs the counter and the joint flush on
     every frame)".

   The spec's authorized paths do not include these files, so the implementer was right not to edit
   them. But the research corpus and the floor-accounting ruling now state a false cost law for the
   multiband. Under the owner principle, leaving them false is not acceptable. Root must authorize
   the doc follow-up: amend this spec's paths, or file a small doc issue. This does not block the
   code.

2. **The spec body's Test value clause is still false.**
   `.github/ISSUE_SPECS/1455-...md:130-133` says "the multiband `DIGESTS` cover live and silent-tail
   frames". They do not: `corpus.rs:120-158` calls `lr4_step` with its own counter and never calls
   `process_block`. The implementer recorded this correctly in the Attempt record ("Spec correction"),
   and I confirmed it. The body text belongs to root and is not amended. It is listed here so that
   root amends it when the spec closes.

### NIT

3. **TurboFan ramping loops carry more stack slots** (descriptive, not gated). In the shipped module:
   - The unarmed ramping loops carry 44/45/44 slots. The base ramping loops carry 41/40/42. So the
     unarmed loops carry +3/+5/+2, but they are 26/22/18 instructions shorter.
   - The armed ramping loops carry 43/44/42 slots (+2/+4/0) at the same length as the base.

   These loops run only in the 63-frame ramp segment after an automation point. The settled
   (live-audio) loops improve from 519/23, 525/24 and 564/23 to 461/14, 469/15 and 502/15. Gate 4 lists
   TurboFan as descriptive. D3 does not gate it. The V8 spill gate holds no multiband row. No browser
   document runs a multiband, so nothing can time this path. This is acceptable under the spec.
   Recorded only so that a later V8 pass knows where the slots are.
4. `crates/multiband-compressor/src/lib.rs:709-711` (`Side::silence`): "Advanced on every frame the
   crossover runs". On the unarmed form the counter now advances once per segment
   (`silence_skip_block`) to the same value. The wording is still defensible, but "kept where a
   frame-by-frame count would leave it" would match the new code.
5. `crates/multiband-compressor/tests/product.rs:1461`: the test doc says "the per-block arming test".
   The dispatch is per segment (`lib.rs:1113`). The words mean the same thing in this test, which runs
   no automation, but "per-segment" would match the code.
6. Deliverable 2 asks for the multiband's iOS count. The Attempt record's Gate 2 only says that there
   is no multiband row. The count is **0**: I measured it directly from the iOS release assembly. No
   eight-lane lines were found either.

## The four questions

**1. Is the per-segment arming decision exactly equivalent to the per-frame form? Yes.**

The proof is `lane::silence_armable_holding`'s, applied per side over both stages:
`Side::arms`, `lib.rs:752-767`, calls `svf_state_held([filter.a, filter.b])`.

- **A lane that cannot arm in the segment.** Its rest threshold is `+0.0` on every frame, and
  `flush_pair(.., +0.0)` equals two `flush` calls.
- **A lane that can arm, at rest in both stages, in a segment of at most N frames.** It sees only zero
  input up to any armed frame. Stage b's input is stage a's low-pass tap, so both stages stay at
  `+0.0`, and the joint term has nothing to zero. A nonzero sample after arming resets the counter,
  which cannot reach N again inside the segment.
- **A segment longer than N, or a held NaN.** These force the armed form.

The skip-block advance cannot diverge from the frame-by-frame count:
- `silence_skip_block` reads the segment's input slice before `run_segment` overwrites it. The armed
  frame loop also reads frame `f` before it stores frame `f`, so both see the same input.
- The primitive is exact against the frame loop, including saturation at 2^24 (lane
  `g4_silence_skip_block_is_the_frame_loop`, `..._saturates_a_long_trailing_count`).
- `ARMABLE = false` never writes the local counter copy, so the write-back is the advanced value.

Bypass: `armable = BYPASS || ..`, so the bypass path never runs the skip, and its unarmed instantiation
is dead code. The five bypass loops are unchanged on x86. In the link modes, `link_levels` acts after
the crossover, and one decision covers both sides. If either side arms, both sides run the base body.

The differential confirms this. My own harness, `zz_v1455_diff.rs`, runs through the public contract
with separate target dirs for base and head:
- **Coverage:**
  - scalar, `Simd4` (two banks) and `Simd8`;
  - all three link modes, bypassed and not;
  - 44.1 and 96 kHz; quanta 1, 128 and 4,096;
  - 40,000 frames of eight per-lane shapes, with left and right shapes offset so that one side can
    arm while the other is live. The shapes include stops on a block boundary and inside a block,
    zero gaps of N-1, N and N+1 between impulses, live audio then 2N+17 zeros then a burst, `±0.0`
    alternation, subnormals, and joint-band impulses of 3e-15 and -2e-16 spaced around N;
  - block-rate automation on rotating parameters at multiples of 2,500 frames and at the live/silent
    edges, which gives ramping segments of 63 frames plus the rest;
  - a snapshot about every 2,048 frames;
  - two restores into fresh instances, at frames about 18,000 and 30,000. Each lane and channel gets
    a joint-band word pattern (both stages, stage a, stage b or none) and a counter of N-1, N-q,
    N-q-1, N-2, the counted value or 0.
- **Variant 2** adds NaN, -NaN, +inf and -inf inputs, a normal minimum and a subnormal, and restored
  counters of 2^24-3 and 2^24.
- **Result:** **108 of 108 configuration hashes are identical in each variant** (every output word,
  report and snapshot byte).
- **Sensitivity.** Each mutant of head moves lines (of 108; the 54 bypassed lines cannot move):
  near-only 33, far-only 54, stage-a-only 21, stage-b-only 18, always-unarmed 54, off-by-one 21,
  no-skip 54, and a skip that reads the wrong channel 54. So the harness reaches the dispatch.

**2. Are the TurboFan slots and the module growth acceptable under the spec's gates? Yes.**

D3's keep criteria pass on the measured targets. I reproduced the implementer's counts exactly.
Counts are base -> head unarmed, with the head armed form in brackets:

| Target | Settled loop (live audio) | Ramping loop |
|---|---|---|
| x86-64-v3 `f32x8` | 457/462/545 -> **426/428/505** (459/464/546) | 620/625/703 -> 581/586/663 (619/624/705) |
| x86-64-v3 `f32x4` | 458/463/531 -> **424/424/494** (453/457/528) | 617/623/689 -> 584/587/650 (618/625/683) |
| `simd128` (shipped module) | 835/844/931 -> **755/764/851** (834/843/930) | 1067/1076/1163 -> 987/996/1083 (1067/1076/1163) |

In the unarmed loops, `vpmaxud` falls from 4 to 0 and `vcmp` from 32 to 24. Spills in the `f32x4`
settled loop fall from 64 to 57.

TurboFan is a descriptive count (see NIT 3). There is no size gate for the module:
`check-web-audioworklet.sh` passes. The multiband roster entry is vector=2555, scalar=20, budget 255.5,
so the scalar ratio is unchanged. The spec's hazard section expected the doubling. The shipped module
grows from 3,097,126 B to 3,139,847 B (+42,721 B, +1.38 %), and both digests match the implementer's.

I did not re-run p50: it was not in my re-run list, and it is descriptive. The codegen agrees with the
implementer's -2 % to -6.8 % in direction and size.

**3. Is the new test's value true? Yes. No existing test catches its four mutants.**

For each mutant I ran two test sets on head:
- the spec's package set (`-p lane -p multiband-compressor -p effect-runtime -p dsp-reference -p conformance`);
- the `test-debug-a` workspace command, which covers every multiband dependent that has behavioural
  tests: effect-compiler, graph-compiler, host-core, host-web, capi, parameter-metadata and
  session-validator.

In each case only `either_channel_and_either_stage_arm_the_crossover` failed. The other 290 tests in
the spec's set and all 1,458 in `test-debug-a` passed.

| Mutant | Failure message |
|---|---|
| near-only | "channel 1, stage 0: an armed block leaves [9.4e-16, -8.7e-16, -1.1e-16, -7.3e-18]" |
| far-only | channel 0, stage 0 |
| stage-a-only | "channel 0, stage 1 ... [0.0, 0.0, 9.4e-16, -8.7e-16]" |
| stage-b-only | channel 0, stage 0 |

The new test passes at base and at head. The wasm-gate corpus cannot catch these mutants, because it
never reaches `process_block` (see MINOR 2).

I also confirmed the implementer's other claims:
- always-unarmed and off-by-one are caught by `the_crossover_joint_flush_arms_after_its_inputs_silence`;
- no-skip is caught by `each_channel_counts_its_own_input` and `the_silence_counter_is_carried_and_validated`.

**4. Is render still allocation-, lock- and syscall-free? Yes.**

The new render work is `Side::arms`, which is mask arithmetic, and `silence_skip_block` /
`silence_skip_settle`, which are slice loads and compares. Neither allocates, locks or makes I/O calls.
`no_alloc_render` (scalar and both bank widths, on live input, so on the unarmed form) passes, and
`check-realtime-policy.sh` passes.

No new state, no queue and no control-only data enters render memory. The acked-batch question does
not apply.

## Test value

- `either_channel_and_either_stage_arm_the_crossover` (`tests/product.rs:1464`): the test turns red
  when the per-segment arming test reads the held state of only one channel or only one crossover
  stage. The crossover then keeps joint-band words that the armed form zeros. No existing test catches
  this, because every existing silence test restores both channels with all four crossover words
  banded. Each of the four mutants is red on this test alone; the test is green at base and at head.

## Gates run (head `db4078c3a` unless marked)

- **Gate 1:** my own base-vs-head differential, 2 x 108 configurations, identical (see question 1).
  The base and head binaries were checked to be different builds.
- **Gate 2:** `bash scripts/check-cross-targets.sh` passes (exit 0). The known-defect rows are
  builtins 5, host-core 4 and soft-clip 1. There is no multiband row, and the direct count of
  multiband iOS `memset_pattern16` calls is 0.
- **Gate 3:**
  - `bash scripts/run-wasm-gates.sh` exit 0 (native, wasm simd128 and the V8 EQ loops; the spill gate
    is ok).
  - The worklet chain passes:
    - `build-web-audioworklet.sh --named-twin`: module `bf0125fe...`, 3,139,847 B. The base build is
      `04b7c5f5...`, 3,097,126 B.
    - `check-web-audioworklet.sh --without-metadata-regeneration`: exit 0.
    - `check-browser-expected-resources.py --artifacts`: exit 0.
    - `check-scalar-oracle-absent.py`: ok.
    - `test-web-audioworklet.sh` with a private TMPDIR: exit 0, nothing left behind.
- **Gate 4:** x86-64-v3 (`--emit asm`), `simd128` (`wasm2wat` of the base and head named twins) and
  TurboFan (the repo's spill-gate listing) counts, as in question 2.
- **Gate 5:**
  - The spec's `cargo test` (291 passed).
  - `test-debug-a` (1,458 passed).
  - `conformance_fixtures --check` (exit 0).
  - `check-lane-policy.sh`, `check-workspace-policy.sh`, `check-realtime-policy.sh`, and the
    `aarch64-known-defects.py --self-test`: all ok.
  - `cargo fmt --all -- --check`: ok.
  - `cargo clippy -p multiband-compressor --all-targets -- -D warnings`: ok. The only crate this slice
    changes is the multiband; I did not run the whole-workspace clippy because disk space was short.
  - `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps -p multiband-compressor -p lane` (and the
    multiband with `--document-private-items`): clean.
  - The workspace `cargo doc` fails only at `crates/gate-expander/src/corpus.rs:64`. That line comes
    from `ffcbba6b7` (#1459) and is present at the base. It is not this slice's.
- **Mutation runs:** the four mutants above against the spec set and against `test-debug-a`, plus
  always-unarmed, off-by-one and no-skip against the spec set.

Not run: native AArch64 (CI only), a p50 re-run, and the whole-workspace clippy.

**Process note for other verifiers (not a finding).** Two `git archive` exports of the same workspace
that share one `CARGO_TARGET_DIR` reuse each other's artifacts silently. Cargo strips the workspace
path from crate metadata, and the exported files keep the commit's mtime. My first base run was
really the head binary, and one later "base" run was really a stale mutant build. Give each export its
own target dir, and check that the binaries differ.

Evidence (logs, harness, analysers, hashes): `/tmp/claude-1002/v1455/evidence/`.
