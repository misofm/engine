# Software-pipeline the EQ's depth-2 cascade passes


EQ optimisation, slice 3 (research 2026-09-27, base `6ca203f8`). Evidence:
`docs/handoffs/effects-2026-09-27/EQ-DIAGNOSIS.md`, sections 3 and 5 (EQ-3). The timed prototype is
`docs/handoffs/effects-2026-09-27/eq-variant-skewed-depth-2.patch` (a local kernel in
`parametric-eq`); this issue puts the kernel in `crates/lane`, where the SVF lives.

## Product outcome

`svf_cascade_interleaved_impl` (`crates/lane/src/kernels.rs:259`) runs, per frame, section 0 then
section 1 of each stream, and section 1 consumes section 0's output of the same frame. Each
frame's work is therefore a dependency chain two sections long, roughly twice the 20-cycle
recurrence on the bench host (Zen 3), and the out-of-order window does not overlap enough of it
across frames: a depth-2 pass runs 10.3 cycles per section-frame at `Simd8` against about 9.0 for
the best arrangement measured.

Skew the pass: in iteration `i`, section `k` of every stream processes frame `i - k`, taking its
input from section `k - 1`'s output of the previous iteration (held in a register). The sections of
one iteration are then independent. Kernel replicas, per two-section bank-block:

| target | today, select-free | skewed, select-free | today, masked | skewed, masked |
|---|---:|---:|---:|---:|
| native `Simd8` | 5,301 | 4,616 (-13 %) | 5,883 | 4,828 (-18 %) |
| native `Simd4` | 4,953 | 4,815 (-3 %) | 5,691 | 5,059 (-11 %) |
| wasm `simd128`, V8 | 6,124 | 5,021 (-18 %) | 7,062 | 5,963 (-16 %) |

A clean browser build of the prototype moved today's padded fixture's EQ isolate from 43.2 to 36.9
us per 64-track block, and a clean native build by -1.3 us. After EQ-1 the standing fixtures have no
depth-2 pass; the saving lands on sessions with two or more live sections per bank.

## Lessons carried from #944

Dev and release; NaN words compared as "both NaN, or equal bits"; scenario test pinned on base
first; mutations recorded red in `crates/lane/tests/MUTATIONS.md`.

## Invariants

- **Class A.** Each `(stream, section)` chain runs `svf_step` and the output mix
  `m2.fma(v2, m1.fma(v1, m0.mul(x)))` on exactly the inputs, in exactly the order, it does today;
  with a dry mask, the same `L::select(mask, x, wet)` at the same section boundary. Only the
  interleaving of independent chains changes. State words leave the kernel identical.
- A frame is read before any section writes it back (the last section writes frame `i - (D - 1)`
  in iteration `i`, after section 0 has read frame `i`).
- `frames < D` falls back to today's kernel. No allocation, no `unsafe`, `#[inline(always)]`.
- `svf_cascade_interleaved` and `svf_cascade_interleaved_with_dry_masks` are not edited: they stay
  the oracle.

## Interface contract

1. `crates/lane/src/kernels.rs`, beside `svf_cascade_interleaved_with_dry_masks`:

   ```rust
   /// [`svf_cascade_interleaved`] scheduled as a software pipeline: in iteration `i`, section `k`
   /// of every stream runs frame `i - k`. Same chains, same order per chain, same bits.
   #[inline(always)]
   pub fn svf_cascade_skewed<L: Lane, const S: usize, const D: usize>(
       io: [&mut [f32]; S], frames: usize,
       c: &[[SvfCoef<L>; D]; S], s: &mut [[SvfState<L>; D]; S],
   )
   /// The same with [`svf_cascade_interleaved_with_dry_masks`]'s per-section dry selection.
   #[inline(always)]
   pub fn svf_cascade_skewed_with_dry_masks<L: Lane, const S: usize, const D: usize>(
       io: [&mut [f32]; S], frames: usize,
       c: &[[SvfCoef<L>; D]; S], s: &mut [[SvfState<L>; D]; S],
       dry_masks: &[[L::Mask; D]; S],
   )
   ```

   Body: a prologue (iterations `0..D-1`), a branch-free steady state (iterations `D-1..frames`,
   sections visited from `D-1` down to `0` so each consumes the previous iteration's carry before it
   is overwritten), an epilogue (iterations `frames..frames+D-1`). The prototype's body is in the
   patch named above.
2. `crates/parametric-eq/src/lib.rs`: `interleave` and `interleave_mono` call the skewed kernels for
   their depth-2 passes (masked or select-free per EQ-2). Depth-1 passes are unchanged.

## Smallest closable slice

Authorized paths: `crates/lane/src/kernels.rs` (the two kernels and their docs);
`crates/lane/tests/g2_kernel_identity.rs` (gate 1); `crates/lane/tests/MUTATIONS.md`;
`crates/parametric-eq/src/lib.rs` (`interleave`, `interleave_mono`); `crates/parametric-eq/tests/bank.rs`
(gate 2); this spec.

## Non-goals

A different depth (`SVF_CASCADE_DEPTH` stays 2), cross-bank fusion, op-by-op interleaving (measured:
no gain over skewing), any change to the ramped kernels or to the multiband crossover's `svf_step`
use.

## Objective gates

1. **Kernel identity (lane).** `g2_skewed_cascade_equals_the_interleaved_cascade`: for `L` in `f32`,
   `Simd4`, `Simd8`, `S` in `{1, 2}`, `D` in `{1, 2, 3}`, masked and select-free, frame counts
   `{1, 2, 3, 128, 1024}`, over `ALL_SIGNALS` plus a hostile family (both signed zeros, subnormals,
   a NaN payload, an infinity), 40 consecutive blocks with carried state: output words and final
   state words equal to `svf_cascade_interleaved[_with_dry_masks]` ("both NaN, or equal bits"), guard
   words untouched, under `CanonicalFpEnv::enter()`. Dev and release.
2. **Scenario (EQ, pinned on base).** `two_and_four_live_sections_render_the_base_bits` in
   `tests/bank.rs`: 2, 4 and 6 live sections, scalar and `native_bank()`, 32 hostile blocks,
   SHA-256 of outputs and snapshots pinned on base. `cargo test -p parametric-eq` green.
3. **Rows.** `chain_shape` green and every `WORKLOADS` digest unchanged (as EQ-1 gate 3).
4. **Mutations**, each alone, red:
   - M1: visit the sections `0..D` ascending in the steady state (the carry is overwritten before
     use): gate 1.
   - M2: store section `D - 1`'s output to frame `i` instead of `i - (D - 1)`: gate 1.
   - M3: drop the epilogue: gate 1 (the last `D - 1` frames).
5. **Browser artifact.** As EQ-1 gate 5 (rule 1 must still find one arithmetic-carrying EQ
   `process_bank`), **plus a clean-build wasm timing**: build the console guest at base and with the
   change, run both in one Node process on `sixty_four_track_eq_only` with the EQ-1 fixture and a
   two-band variant, and attach the numbers. A build that also carries other kernel variants is not
   evidence: the diagnosis's flag build outlined `render` and measured the prototype +13 us slower,
   while the clean build measured it 6.3 us faster.
6. Toolchain and policy scripts as EQ-1 gate 7; `bash scripts/check-lane-policy.sh`.

## Console benchmark rows

After EQ-1, no standing row has a depth-2 pass: expect no move. A multi-band row is a separate
tooling issue.

## Dependencies

EQ-1 and EQ-2 first (they decide which passes are depth 2 and which are select-free).

## Standing rules for the implementer

As EQ-1. Only `crates/lane` may name `wide`; the new kernels use the `Lane` trait alone.

## What the implementer will hit

- **V8 is the risk.** Sixteen `xmm` registers hold 2 x 2 sections' state, the carries and some
  coefficients; the rest are reloaded. The clean prototype still won 16-18 % there, but check the
  emitted loop (`node --no-liftoff --print-wasm-code`) for new spills.
- **`Simd4` natively gains little select-free (-3 %)**; that is not a reason to specialise by width.
- **The multiband crossover** calls `svf_step` directly and is not part of this change.


## Attempt 1 evidence

Implementer: attempt 1, 2026-09-27, branch `codex/977-eq-elision-and-passes`, on #977 (`92deafd9`).
The verification comment (pure scheduling, bit-identical; add the two-band wasm run and the mono
skew to the evidence) is applied. Host: AMD EPYC 7313P (Zen 3), `rustc 1.97.1`, `x86-64-v3`, every
build `CARGO_INCREMENTAL=0`.

### The change

- `crates/lane/src/kernels.rs`: `svf_cascade_skewed` and `svf_cascade_skewed_with_dry_masks`, one
  body (`svf_cascade_skewed_impl`, the `SvfOutput` policy of the interleaved kernels). Prologue
  (iterations `0..D-1`, section `k <= i`), branch-free steady state (iterations `D-1..frames`,
  sections `D-1` down to `0`, each reading the previous iteration's carry before it is overwritten),
  epilogue (iterations `frames..frames+D-1`, section `k >= i + 1 - frames`). Section 0 loads frame
  `i`, section `D - 1` stores frame `i - (D - 1)`; `frames < D` calls `svf_cascade_interleaved_impl`
  with the same policy. `Lane` trait only, no `unsafe`, no allocation, `#[inline(always)]`.
  `svf_cascade_interleaved[_with_dry_masks]` are untouched.
- `crates/parametric-eq/src/lib.rs`: `interleave` and `interleave_mono` run every depth-2 pass
  through the skewed kernels (select-free when admitted, masked otherwise, per #977); the depth-1
  tail is unchanged.
- **Outside the brief's paths (deviation 1):** `#[inline(always)]` on
  `PreparedParametricEq::render`, `render_mono` and `process_bank_inner`. Without them the
  `simd128` build outlined the dual `render` (then `process_bank_inner::<false>`), leaving
  `process_bank` with no arithmetic: `KERNEL_ROSTER` rule 1 failed ("parametric-eq f32x4 dual: 0
  arithmetic-carrying kernels match"). Each has one caller per width, so the pins duplicate nothing;
  the start-ramp precedent (`#[inline(never)]`, pinned "rather than left to a heuristic") is the
  same reasoning.

### Gate 1: `g2_skewed_cascade_equals_the_interleaved_cascade` (`crates/lane/tests/g2_kernel_identity.rs`)

`f32`, `Simd4`, `Simd8` x `S` in {1, 2} x `D` in {1, 2, 3} x masked and select-free x frames
{1, 2, 3, 128, 1024} x the four G2 signals and a hostile family (`+0.0`, `-0.0`, subnormals of either
sign, normals across `2^-30..2^30`, a `0x7fc01234` NaN payload at block 30 and `-inf` at block 35 on
single lanes), 40 carried blocks each; per-(stream, section) dry masks from a fixed pattern. After
every block every output word and every integrator word equals the interleaved kernel's ("both
NaN, or equal bits"), and 8 guard words either side of every block are untouched, under
`CanonicalFpEnv::enter()`. Green in dev and release (`cargo test -p lane`: 69 passed, 2 ignored).

### Gate 2: `two_and_four_live_sections_render_the_base_bits` (`crates/parametric-eq/tests/bank.rs`)

Six shapes, 8 tracks, 32 hostile blocks, frames 128 with 37-frame blocks and blocks of 1, 2, 3 and
37 frames at `8k + 1` (the fallback, the shortest pipelines): two general bands; the two cuts, each
live on some lanes and dry on others; four general bands; both cuts on some lanes plus two bands;
all six with the cuts on some lanes; all six everywhere (masked on every block). `-0.0` and
non-finite blocks as in #977's scenario. Scalar, bank and bank-mono legs. Pinned on the unmodified
base (`92deafd9`), identical in dev and release: scalar
`9fdeb65d468cd6c5b5aed91c853d0dcb3fb4781dc723ecba223cf67926214dda`, bank
`aad039b4e61453d750e40869a0c8b7aa59df2a6299eecfc6430faab0beebcc6a`, bank-mono
`602d2f39e13d8431c2db10bb94caebeeb3f64d767261ec6c773c648079106ea4`; the change reproduces all
three. `cargo test -p parametric-eq`: 107 passed, 3 ignored, dev and release, with and without
`test-support` (#976's and #977's pins included).

### Gate 3: rows

`chain_shape` (release): 23 passed. Scratch digest harness as in #980's record: 90 native lines
(15 rows x three backends, one and two EQ bands) identical to the base; 30 wasm guest digests
identical to the base and to the native ones. The two-band rows run depth-2 passes (dual and
collapsed), so they exercise the skewed kernels natively and in the browser build.

### Gate 4: mutations

Recorded in `crates/lane/tests/MUTATIONS.md` ("Issue #978"), release: M1 (ascending sections), M2
(store to frame `i`; the epilogue store runs off the block), M2b (the same clamped, so it shows on
the bit comparison) and M3 (no epilogue), each red on gate 1 at its first case (scalar, `S = 1`,
`D = 2`, two frames).

### Gate 5: browser artifact, V8, and timing

Artifact (build script's cargo line, not repinned) `ba9438b8…`: `--callgraph
miso_engine_web_v1_render` closure=8 traps=5 (one owner); `--kernel-shape` ok, kernels=14, rule 3
ok; `parametric-eq f32x4 dual` one kernel, vector 384 -> 672, scalar 0; `collapsed` 192 -> 336,
scalar 0; every other roster row unchanged; `meter_poll` and `command_submit --allocation-only` ok.
The growth is the prologue, the epilogue and the short-block fallback of the two skewed
instantiations per body.

V8 (`--no-liftoff --print-wasm-code`, two-band `eq_only`): the admitted dual depth-2 loop goes from
161 instructions (`vmovups` 41, 62 memory operands) to 185 (`vmovups` 54, 78 memory operands): V8
does spill more in the skewed loop. The masked loop goes from 208 to 216 (`vinsertps` 12 -> 6). The
mono admitted loop goes from 72 to 78 instructions (`vmovups` 9 -> 10).

**Kernel replicas** (one binary each, both kernels, fixture-shaped bell coefficients, 128 frames,
the per-call input refresh measured alone and subtracted, best of 7 x 20,000 calls, two rounds that
agree within 1 %, `taskset -c 31` under the lock, 3.7 GHz): cycles per bank-block, depth 2:

| target | dual select-free | dual masked | mono select-free | mono masked |
|---|---:|---:|---:|---:|
| native `Simd8` | 5,527 -> 4,981 (-9.9 %) | 8,212 -> 5,557 (-32.3 %) | 3,266 -> 3,244 (-0.7 %) | 3,673 -> 3,302 (-10.1 %) |
| native `Simd4` | 5,233 -> 4,861 (-7.1 %) | 6,271 -> 5,186 (-17.3 %) | 3,266 -> 3,242 (-0.7 %) | 3,454 -> 3,304 (-4.3 %) |
| V8 `simd128` | 5,801 -> 4,960 (-14.5 %) | 7,378 -> 5,669 (-23.2 %) | 3,193 -> 3,226 (+1.0 %) | 3,433 -> 3,230 (-5.9 %) |

**The mono skew:** select-free, it is neutral (-0.7 % native, +1.0 % in V8); masked (refused blocks,
six live sections) it gains 4-10 %. The contract's skewed mono body is kept.

**Clean-build console rows** (the two-band variant is a scratch-only knob that enables general band
2 on every EQ; the mono rows are the mono fixture with every rack but the EQ emptied, also
scratch-only; neither is committed). Clean scratch builds of #977 and of this change, each verified
by its artifact before timing (EQ `process_bank` arithmetic 384 and 672); two holds, the arms in
both orders, load average 5-6. Isolates, median of the per-round p50s, us:

| row | native #977 | native #978 | wasm #977 | wasm #978 |
|---|---:|---:|---:|---:|
| `eq_only` (one band, no depth-2 pass) | 9.40 / 9.33 | 9.33 / 9.31 | 19.48 / 20.23 | 20.28 / 19.29 |
| two-band `eq_only` (one dual depth-2 pass) | 14.74 / 14.65 | 13.39 / 13.65 | 33.55 / 33.66 | 30.70 / 29.98 |
| EQ-only mono, one band | 7.17 / 7.19 | 7.11 / 7.46 | 15.66 / 15.33 | 15.69 / 15.61 |
| EQ-only mono, two bands (one mono depth-2 pass) | 8.78 / 8.69 | 8.60 / 8.75 | 19.59 / 19.20 | 19.46 / 20.01 |

An earlier pair of holds on the full console mono row agrees (two-band `eq_only`: native 14.07 ->
12.82 and 14.12 -> 12.91, wasm 33.39 -> 31.31 and 33.45 -> 30.68). So the dual two-band row gains
about 1.0-1.35 us natively and 2.9-3.7 us in V8 per 64-track block, and the mono two-band row does
not move beyond noise either way, as the replicas predict (the mono select-free skew is neutral).
The one-band rows do not move. A third pair of holds is discarded: its "#977" arm had been built
from a `git archive` of the commit into the shared scratch target, whose files carried the
commit's older mtimes, so cargo (workspace-relative metadata hashes) reused the #978 rlibs; the
arm's artifact showed 672 EQ ops, not 384. Every arm quoted here was checked that way. No projected
saving is claimed.

### Gate 6: toolchain

fmt, clippy (`-D warnings`), doc (`-D warnings`), `-p lane` dev and release (69), `-p
effect-runtime` (86), `-p console-workload` (39), `-p builtins-compiler --features test-support`
(79), `-p wasm-gates` (9), `-p bench floor` (9), `check-lane-policy.sh` (ok), realtime policy (57
regions), EQ render contract, console benchmark validators: green.

### Deviations and notes for the verifier

1. The three `#[inline(always)]` pins above, outside `interleave`/`interleave_mono`; required by
   gate 5 (rule 1).
2. V8 spills more in the skewed loop (+13 `vmovups`), and the replica still shows -14.5 %.
3. The mono skew is neutral where the admitted plan runs it (select-free) and gains 4-10 % on
   refused or all-live blocks (masked); the skewed mono body is kept as the contract says.

## Sol attempt 1 verdict: PASS

Verifier: Sol, 2026-09-27, on `100469fa` and the stacked tip `04e439db`. Host, toolchain and
artifact checks are as in #977's verdict. The tip's AudioWorklet artifact, `0db9b2f5…`, equals a
build of the pristine worktree.

**The change is purely a schedule.**

- **Indexing.** In the prologue section `k` runs only for `k <= i`; in the epilogue only for
  `k >= i + 1 - frames`. Section `D - 1` writes frame `i - (D - 1)` before section 0 reads frame `i`
  in the same iteration.
- **Short blocks.** `frames < D` falls back to the interleaved body.
- **Oracle untouched.** The `kernels.rs` diff is additive only, so
  `svf_cascade_interleaved[_with_dry_masks]` still stand as the oracle.
- **Tests.** Gate 1 is green in dev and release. M1 and M3 re-run red (`g2_kernel_identity.rs:1040`).
- **Differential.** #977's verdict describes it. Against `1d8c4851`: 0 differing runs at this commit
  (35,000) and at the tip. A harness mutation that visits the steady-state sections in ascending
  order moves all 21,000 runs.
- **Rows.** 90 native and 30 wasm digests are identical, and `chain_shape` is green.

**The inline pins (outside the brief's paths) are accepted.** Three variants each fail `KERNEL_ROSTER`
rule 1 with "parametric-eq f32x4 dual: 0 arithmetic-carrying kernels":

- removing all three pins;
- keeping only `process_bank_inner`'s pin, which outlines `render`;
- keeping only `render`'s and `render_mono`'s pins, which outlines `process_bank_inner::<false>`.

Each pinned function has one caller per width, and the artifact grows by 9.5 KB. `render_mono`'s pin
is not needed today, since the collapsed row stays a single kernel without it; it is harmless.

**Roster.** Dual 312 to 672 and collapsed 156 to 336, scalar 0, 14 kernels. The callgraph,
`meter_poll` and `command_submit` checks are identical to base.

**Performance: the A/B reproduces.** Clean builds, each verified by its artifact or binary. Console
workload (native binary and wasm guest), 64-track EQ isolate against `1d8c4851`:

| row | native | wasm (V8) |
|---|---|---|
| two-band | 14.89 to 13.66 us, and 14.90 to 13.43 us | 44.72 to 32.84 us |
| mono two-band | | 23.20 to 19.30 us |
| standing `eq_only` | | flat, 21.62 to 20.94 us |

**Isolated to this commit (#977 to #978):**

- **Shipped artifact** (`host_web.wasm` through its render export, 64 tracks): the standing one band
  is flat (24.1 to 23.7-24.0 us); two bands gain 34.0 to 30.7-30.9 us (-10 %).
- **V8 per shape** (four-lane bank, `--no-liftoff`): every depth-2 shape improves. Dual passes gain
  10-20 %; mono gains 1.5-14 %. The mono select-free pair gains 2.4 %, not the +1 % of the replica.
  The one-band shapes are flat.
- **Native `Simd8`**: dual depth-2 shapes gain 4-10 %, mono is neutral (-0.5 to +0.5 %).

No shipped shape gets slower. On the browser's four-lane banks the net is positive for every session
with two or more live sections and neutral for one.

Findings:

1. **LOW (process) `crates/parametric-eq/src/lib.rs:2373, 2453, 3139`.** The three pins are outside
   the authorized paths. They are needed for gate 5, as shown above. Record the path amendment on
   the issue.
2. **INFO.** Native `Simd4` one-band dual is 17-33 % slower at this commit in my harness, and noisy.
   This is not a product shape: x86 binds eight-lane EQ banks.
3. **Stacked on #977, which fails attempt 1.** After #977 is revised, re-run gate 2, the row digests
   and the shipped-artifact check.

## Attempt 2 re-verification (rebased onto #977 attempt 2)

2026-09-27. #977 failed Sol's attempt 1 on the shipped artifact's one-band performance and was
revised: its depth-one tail is #976's code again. This commit was cherry-picked onto it
(`100469fa` -> `9e7a342e`). Two doc-comment hunks conflicted (`interleave_mono`'s summary line and
the head of `interleave`'s "Select-free pairs" section); both were resolved by keeping both
statements, and the section names `svf_cascade_skewed` as the pairs' kernel. No code line of this issue
changed. Every gate was re-run on the rebased commit:

| gate | result |
|---|---|
| 1 lane `g2_skewed_cascade_equals_the_interleaved_cascade` | green, dev and release (`-p lane`: 69 passed, 2 ignored) |
| 2 `two_and_four_live_sections_render_the_base_bits` | reproduces its three base pins (`9fdeb65d…`, `aad039b4…`, `602d2f39…`); `-p parametric-eq` 107 passed, 3 ignored, dev and release, ±`test-support` |
| 3 rows | `chain_shape` 23; 90 native lines and 30 wasm guest digests identical to base |
| 4 mutations | M1, M2, M2b and M3 red with the same first messages as recorded in `crates/lane/tests/MUTATIONS.md` |
| 5 artifact `0770f6fe…` | render closure=8 traps=5; kernels=14; EQ dual 672, collapsed 336, scalar 0; other checks identical to base |
| 6 toolchain | as in #977's attempt-2 record, all green |
| 7 shipped-artifact V8 and timing (#977's new gate) | V8: the dual select-free depth-one tail is 83 instructions with no carried stack slot; the skewed pairs (181 and 219) have none. Timing: one-band isolate 20.49 us against base 20.69 (range 19.81-21.28 over 6 runs), two bands 79.98 us against base 92.94 (isolate 30.02 against 43.14), builtins within 0.3 %: PASS |

Sol's finding 1 (the three `#[inline(always)]` pins lie outside the brief's paths) stands as
recorded: they are required by `KERNEL_ROSTER` rule 1. Against attempt 1's numbers, the one-band
row no longer carries #977's regression (Sol measured 24.1 -> 23.7-24.0 us through this commit's
parent; now 20.07 -> 20.49, within noise of base). The two-band gain over #977 is unchanged:
33.50 -> 30.02 us.
