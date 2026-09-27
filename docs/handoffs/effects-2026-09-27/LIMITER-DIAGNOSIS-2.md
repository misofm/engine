# True-peak limiter performance diagnosis, round 2: the loop structure of the dense path

Base: `codex/batch-plumbing-floor-2` at `49f696c7` (contains #990, the linked-pair gain path). Branch
`limiter-diagnosis-2`. Diagnosis only: every prototype was measured and then reverted, and the tree
carries none of them. The prototypes, the scaffolding that selects them, and the out-of-tree harness
with its scripts are one patch, `limiter-diagnosis-2-prototypes.patch` (applies to `49f696c7`). Every
raw timing output is in `limiter-diagnosis-2-raw-timings.txt`. The draft issues are
`issues/limiter-2-1-*.md` to `issues/limiter-2-3-*.md`. Round 1 is `LIMITER-DIAGNOSIS.md`; its
verification is `VERIFY-LIMITER.md`.

## Verdict

* **Dense material (every track limiting), after #990, cycles per lane-sample.** The limiter alone
  costs 8.7 (linked pair) and 10.0 (`dual_mono`) natively at `Simd8`, 15.5 and 18.4 at `Simd4`, and
  23.2 and 25.4 under V8 in the shipped `host_web.wasm`. The detector is half to two thirds of it,
  the gain loop a quarter to 43 %, and fixed per-block work about 10 %.
* **The detector cannot be brought near its port bound in class A.** Each of its four phases is a
  frozen chain of twelve dependent adds. A channel-frame therefore has a critical path of about 44
  cycles and issues about 105 FP operations. The out-of-order window holds about 1.3 frames of that,
  which gives the measured 37-46 cycles per channel-frame against a port bound of 26-28. Every
  class-A shape that trades the register rotation for memory taps, or phases for passes, lengthens
  the chains or thins their interleaving: they lost 16-51 % natively. One change shortens the chain
  itself: dropping the `+0.0` seed makes 12 adds 11. It gains 3-5 % natively on every row.
* **The gain loop's gap is bookkeeping, not arithmetic.** An unchecked diagnostic build shows the
  Rust bounds checks cost 3.5-10 % natively and 6-9 % under V8, the largest V8 lever found. Removing
  the nested van Herk backward pass is worth 5-9 % natively (±2 % V8). The recursion is not
  binding: split into its own pass it runs at 15 cycles per frame against its 13-cycle chain.
* **The kernel's codegen is unstable, and that instability is as large as any class-A gain.** Edits
  that leave a path's source untouched move that path by 8-25 % (section 4). This is the effect
  that made #991 and #992 regress. No slice can be accepted on instruction counts. Each needs the
  three-target timing gate the drafts specify.
* **Ranked changes** (section 5): (1) seedless Annex-2 accumulators; (2) van Herk completion as a
  segment boundary with the release recursion as its own pass; (3) bounds-check-free streams for
  the steady frames (prize measured, safe form unproven). Everything else tried is in the "not
  worth doing" table.

| change | native `Simd8` | native `Simd4` | V8 (shipped artifact, mean of 3 processes) |
|---|---:|---:|---:|
| 1 seedless accumulators | -4 to -5 % (every row) | -3 to -5 % (every row) | linked -3.1 %, dual +1.1 %, console -1.7 % |
| 1 + 2 segment boundary and recursion pass | dual -8 %, linked -3 %, ramping +8 % | dual -6 %, linked -5 %, ramping +3 % | linked -2.6 %, dual -1.9 %, console -3.4 % |
| 3 prize (unchecked diagnostic, not shippable) | -3.5 to -9 % | -4.5 to -10 % | linked -8.7 %, dual -6.0 %, console -8.9 % |

## 1. Method

* **Host and tools.** AMD EPYC 7313P (Zen 3, 16 cores and 32 threads, 3.70 GHz; cpu 31's SMT
  sibling is cpu 15). rustc 1.97.1, release profile (fat LTO, one codegen unit), `+avx2,+fma`.
  Node 22.23.2 (V8 12.4, TurboFan). There is no PMU access (`perf_event_paranoid` 4).
  * Cycles are nanoseconds times a core clock calibrated in process from a dependent add chain.
    It read 3.66-3.70 GHz on every pass but three:
    * 3.54 and 3.60 GHz, under interference;
    * one pass of `st-1` at 1.86 GHz, whose cycles are therefore halved. That pass is excluded;
      the `st-1` figures quoted here are medians.
  * Port bounds and model estimates come from `llvm-mca -mcpu=znver3` on loop bodies extracted
    from the release binaries.
  * The V8 code is from `node --print-wasm-code-function-index`.
* **Timing discipline.**
  * Every timed run held the shared timing lock (each hold under 5 minutes) and was pinned with
    `taskset -c 31`.
  * The host was shared: load averages were 6-29, and other agents' unpinned processes were seen
    on cpu 31. Interference only adds time. So each A/B runs the arms as separate binaries, in
    forward-then-reverse order over three passes, and reports the **minimum** over all rounds
    (medians are also in the raw file).
  * The minima reproduce across load levels. For example, base `Simd8` linked read 8.63-8.75 on
    every run from load 5.9 to 20.
  * Runs where everything moved together by 40 % (load above 20) are kept in the raw file, marked,
    and were used only for ratios within the run.
* **Native kernel rig** (`limiter-diag-2-harness`, an out-of-tree crate on the public bank API):
  * 64 tracks as 8 `Simd8` or 16 `Simd4` banks, with the fixture parameters (ceiling
    `-0.5 - 0.03125 k` dB, release `60 + 1.25 k` ms, lookahead 5 ms), 48 kHz, 128 frames.
  * 16 staged blocks of non-repeating seeded noise at +3 dBFS. Every track limits (deepest
    reduction 0.70).
  * Shapes:
    * `HotLinked` (link `maximum`, the #990 linked body);
    * `HotDualMono` (the dual body);
    * `QuietLinked` (the fixture tone at 0.3, never limits);
    * `RampLinked` (both channels retargeted every block, the ramping dispatch).
  * Each variant is a separate binary: `LIMDIAG2=<bits>` at build time selects it through a
    `const` that the compiler folds.
  * The scaffolded base compiles to byte-identical `process_block` code at both widths and in the
    wasm artifact. That held until the layout experiment of section 4; after it, the reference arm
    is a binary built from the pristine tree.
* **In-place console rows.** `sixty_four_track_console` minus `sixty_four_track_eq_comp_simd1` (the
  directive's isolate), at `Simd8` and `Simd4`, with the +20 dBFS noise source and the fixture tone.
* **wasm.**
  * The artifact is the shipped `host_web.wasm`, built with `build-web-audioworklet.sh`'s cargo line
    (`+simd128`, stripped, remapped; only the pin check is skipped).
  * It is timed through `miso_engine_web_v1_render` under Node, on 64-track sessions derived from
    `console-sixty-four-track-intended.json`:
    * `lim` is the limiter alone;
    * `bi` is no effects;
    * `limdm` is the limiter under `dual_mono`;
    * `console` and `nolim` are the full chain with and without the limiter.
  * The source is fresh JS noise every block (+12 dBFS, so every limiter reduces after the -6 dB
    trim, or +20 dBFS for the console rows).
  * Isolates: `lim - bi` (linked), `limdm - bi` (dual), `console - nolim`.
  * All arms run in one process, rotated per round, 10 rounds of 800 blocks. The final comparison
    is the mean of three separate processes, because V8's code placement differs per compile. A
    single process's isolate moves ±2-4 % between processes for the same module.
* **Identity** (every class-A prototype):
  * The #990 verifier's randomized differential, reused verbatim, at W8, W4 and W1 over 300-1,000
    seeds. It covers launch rates, quanta 1-256, ragged cohorts, restores including hostile ramps,
    collapse runs, and NaN and inf inputs. It digests every output word, report, state payload and
    observation.
  * The crate's test suite in release, including #990's reference-kernel randomized oracle and D90.
  * The native console digests (`Simd8`, `Simd4`, hot and tone).
  * V8 digests through the shipped artifact (five sessions, 300 blocks).
  * The AudioWorklet roster and callgraph gates.
  * All class-A arms read identical everywhere. One arm was not class A, and the differential
    caught it at W8 (section 6).

## 2. Breakdown

Truncated builds: skip the detector (peaks black-boxed), skip the gain loop (peaks kept live), or
skip both. Native truncations are not additive, because each truncated build's codegen differs. The
two derivations are both shown as a range.

| cycles per lane-sample, dense | native `Simd8` linked / dual | native `Simd4` linked / dual | V8 linked / dual |
|---|---:|---:|---:|
| whole limiter | **8.69 / 10.02** | **15.55 / 18.43** | **23.17 / 25.43** |
| no detector (gain loop + fixed) | 3.96 / 5.15 | 5.88 / 8.68 | 9.03 / 12.19 |
| no gain loop (detector + fixed) | 6.63 / 6.58 | 12.31 / 12.21 | 15.85 / 15.70 |
| fixed per-block work (both skipped) | 0.90 / 0.84 | 1.11 / 1.04 | 2.17 / 2.02 |
| **detector** | 4.7-5.7 / 4.9-5.7 | 9.7-11.2 / 9.8-11.2 | 13.7-14.1 / 13.2-13.7 |
| **gain loop** | 2.1-3.1 / 3.4-4.3 | 3.2-4.8 / 6.2-7.6 | 6.9-7.3 / 9.7-10.2 |
| detector, cycles per channel-frame | 38-46 | 39-45 | 55-56 |
| gain loop, cycles per frame (both channels) | 33-49 / 55-69 | 26-38 / 50-61 | 55-58 / 78-82 |
| console isolate, in place (hot / tone) | 9.40 / 9.54 | 15.83 / 15.88 | 23.5 (`console - nolim`) |

Since round 1 (11.5-12.0 at `Simd8`), #990 took the linked pair to 8.7. The dual body is unchanged
at 10.0.

### Emitted code, per iteration of the stationary uniform body

| loop | x86 `Simd8` | x86 `Simd4` | V8 (x86 from TurboFan) | wasm opcodes |
|---|---|---|---|---|
| detector, per channel-frame | 133: 106 FP ops (48 `vmulps` with a stack coefficient operand, 48 `vaddps`, 5 `vandps`, 4 `vmaxps`), 15 register moves, 3 tap spills and 3 reloads | 133-136, same shape | 200: 105 FP ops, 81 vector memory moves (48 coefficient reloads, 26 history-rotation moves through spill slots) | 256 |
| linked gain loop, per frame | 119: 25 vector ops (2 `vdivps`), 11 bounds checks, about 24 reloads of spilled ring bases and lengths, `d` spilled to `[rsp+0xca0]` and reloaded every frame, plus the nested backward pass | 117 | 191 (3 `v128.bitselect`, 3 instructions each) | 266 |
| dual gain loop, per frame | 196: 47 vector ops (4 `vdivps`), both `d` and the right box sum spilled | 199 | 248 | 405 |

`llvm-mca` (znver3) estimates, as model / port bound in cycles per iteration:

| loop | `Simd8` model / bound | `Simd4` model / bound |
|---|---:|---:|
| detector | 40.6 / 28.5 | 30.6 / 29.5 |
| linked loop | 31.4 / 21.2 | 32.5 / 20.8 |
| dual loop | 50.1 / 34.5 | 52.8 / 35.0 |

The model is a guide only. It predicted the phase-major detector at 36 cycles per frame, and it
measured 62.

### Dependency paths

* **Detector.** Per channel-frame, four independent chains run
  `vmulps` (3) → twelve dependent `vaddps` (36) → `abs` → a four-deep `max`. That is about 44
  cycles. Frames do not depend on each other.
* **Gain loop, per frame.**
  * **Carried.** The release recursion
    `sub → mul → add → max → and → cmp → andn` is 13 cycles in registers. In the fused loop it runs
    through a stack spill, which adds store forwarding. The box sum (`add → sub`) is 6 cycles and
    the van Herk prefix (`min`) is 1.
  * **Feed-forward, about 60 cycles.** The linked peak `max` → the divide (about 11) → the
    `blend`s and the ring store and loads → two `min`s → quantise (9) → the box sum (6) → the divide
    (about 11) → `1 - s` → the recursion (13) → the gain `sub` → `mul` → `blend` → store.

### Why both loops run at half their port bound

* **The detector is bound by its chains, not its ports.**
  * The port bound is about 26-28 cycles per channel-frame: 48 `vmulps` on the two FMUL pipes, the
    `max` and logic.
  * To reach it, about 1.6 frames of 44-cycle chains must be in flight: about 170 FP operations
    renamed and waiting. What we see is consistent with Zen 3 holding about 1.3 frames (its FP
    register file and scheduler, behind in-order retirement).
  * We cannot confirm the mechanism without counters. The measurements say the same thing:
    * every shape that lengthened each frame's path lost, even with fewer instructions (taps
      loaded from memory add the load latency: +16-25 %);
    * every shape that thinned the chains' interleaving lost (phase-major: +34-51 %);
    * doubling the chains in program order did not help (two frames per iteration: +20-25 %);
    * the only shape that shortened the chain gained (seedless: 12 adds become 11, -3-5 %).
  * The rotation moves cost dispatch slots, not the bound.
  * The remaining gap is attributable to the frozen summation order, which is class B
    (section 8).
* **The gain loop is bound by bookkeeping and exposed latency.**
  * Of 119 instructions per linked frame, 25 are vector arithmetic.
  * The rest:
    * bounds checks (about 11 compare-and-branch pairs);
    * reloads of ring bases and lengths the register allocator spilled;
    * the phase and completion branches;
    * a 241-iteration backward pass nested inside the frame loop, which pins registers for the
      whole loop;
    * `d` living on the stack.
  * Measured prizes:
    * bounds checks: -3.5 to -10 % native, -6 to -9 % V8 (unchecked diagnostic);
    * the nested backward pass: -5 to -9 % native, ±2 % V8 (skipped as a diagnostic, wrong
      output);
    * a register-resident recursion: the split pass 2 at 15.4-15.8 cycles per frame against the
      13-cycle chain.
  * The divider is not binding. It carries 2 (linked) or 4 (dual) `vdivps` per frame at 3.5 cycles
    each.
* **Memory is not binding.** Eight tracks (one `Simd8` bank, a working set in L2) against 64 moved
  the whole limiter by 2-3 %.
* **Under V8,** the detector's 200 instructions are 105 arithmetic plus 81 vector memory moves.
  V8 cannot hold 48 coefficient splats in about 14 allocatable `xmm` registers and does not fold
  loads into `vmulps`, so every coefficient is a separate load. It also spills the twelve rotating
  taps and moves them stack to stack. The gain loop is dominated by `i32` address and bounds
  arithmetic: the V8 pass-1 loop of the split prototype is 105 instructions, 68 of them GPR.

## 3. What was prototyped

All class A unless marked. Bits refer to `LIMDIAG2` in the patch.

| prototype (bit) | what it does |
|---|---|
| `P_SEED` (4) | each accumulator starts at its first product (#992 proof 3) |
| `P_DETPM` (8) | the detector as four phase-major passes over a tap buffer, twelve coefficients in registers |
| `P_DETTM` / `P_DETTM2` (4096 / 8192) | tap-major over a tap buffer, one or two frames per iteration, no register rotation |
| `P_DETPP` (262144) | two phase-pair passes: phases 0 and 3, then 1 and 2, from one set of twelve coefficient registers (`h[k][p] == h[11-k][3-p]` bit for bit, asserted at compile time) |
| `P_DETLR` (33554432) | both channels' detectors in one loop, one coefficient load feeding two `mul`s |
| `P_GSPLIT` (16) | the gain frame loop split into pass 1 (steps 1-5 and `1 - s` into a stack scratch) and pass 2 (the release recursion and step 7) |
| `P_VHSEG` (65536, with `P_GSPLIT`) | segments also cut at van Herk completion; pass 1 runs branch-free steady frames with no nested loop |
| `P_VHSEGF` (131072) | the same cut in the fused loop, without the pass split |
| `P_BCF` (8388608) | the linked steady pass 1 with every stream an exact-length `chunks_exact` view, read streams copied at segment entry |
| `P_MAXDIV` (268435456) | `l / max(p, l)` in steady frames. Not class A as built: see section 6 |
| `P_COLD` (16777216), `P_OUTDET` (134217728) | ramping and per-lane bodies out of line; the detector out of line |
| chunk 64 / 128 (1024 / 2048) | a larger detector chunk |
| diagnostics only | `X_NODET`, `X_NOGAIN`, `X_NOG1`, `X_NOG2` (truncations), `X_UNCHECKED` (`get_unchecked` rings), `X_NOBACK` (skip the backward pass), `P_PADS` (peak-array layout) |

## 4. Codegen instability

`LimiterCore<L>::process_block` is one function of about 48 KB at each width. It holds the detector
and gain loops of every dispatch, the per-lane body and the backward passes. Edits that leave a path's
source text unchanged move that path's time, natively:

* **The two peak arrays turned into slices of one stack array** (no semantic change;
  `P_PADS` with zero padding): +8-13 % on every row.
  * Padding them by 1, 2 or 3 KiB moved nothing further, whether or not their addresses aliased
    the coefficient spill slots modulo 4 KiB. So 4 KiB aliasing is not the mechanism.
* **The pass split restricted to the stationary dispatch** (`P_STATONLY`):
  * the ramping dispatch, whose source is then pristine, ran +20-25 %;
  * the stationary rows lost their gain.
* **The ramping and per-lane bodies moved out of line** (`P_COLD`): +7-10 % on stationary rows
  whose code did not change. `process_block` was then inlined into `process_bank`.
* **The detector moved out of line** (`P_OUTDET`): +11-15 % at `Simd8`, -1 to -3 % at `Simd4`.
* **Adding `P_MAXDIV`** (two fewer vector ops per steady frame) to the split: +5 % to +20 % against
  the same split without it.
* **Adding the linked-only `P_BCF` pass**, dead code for dual blocks: the dual rows went from -7 %
  to -1 %.
* **Two builds of the same truncation mode** differed by 1 cycle per lane-sample.

V8 recompiles each module and varies ±2-4 % per process for the same bytes.

This is the mechanism behind #991 and #992 reading +6-9 % slower with fewer instructions. It has two
consequences:

* a class-A slice worth less than about 5 % cannot be told from noise by one build;
* acceptance must be the three-target timing gate on the final whole-function code, with
  separate binaries, not instruction counts.

## 5. Ranked changes

Δ is the change of the minimum over rounds against the pristine binary in the same run. The rows
are `HotLinked`, `HotDualMono`, `QuietLinked` and `RampLinked`. V8 numbers are the mean of three
separate Node processes, as `lim - bi` (linked), `limdm - bi` (dual) and `console - nolim`. Every
change is allocation-free, adds no `unsafe` and does not change the state layout.

### 1. Seedless Annex-2 accumulators (`issues/limiter-2-1-seedless-annex2-accumulators.md`)

`annex2_phases` (`src/lib.rs:1145`) starts each accumulator at its first product instead of
`+0.0 + product`.

* **Why it wins.** The chain shortens from 12 dependent adds to 11, and the detector's bound is its
  chains. It saves 4 `vaddps` per channel-frame: x86 133 to 129, wasm 48 to 44 `f32x4.add`, and the
  roster's vector count falls from 910 to 878.
* **Native, final run.**
  * `Simd8`: 8.69 → 8.28 linked, 10.02 → 9.59 dual, quiet -5 %, ramping -3 %.
  * `Simd4`: 15.55 → 14.74, 18.43 → 17.59, quiet -5 %, ramping -3 %.
  * It was faster on every row in each of the seven runs that included it, by 1-6 %.
* **Console isolate:** `Simd8` -3 % (hot), -2 % (tone); `Simd4` -4 %, -1 %.
* **V8:** linked -3.1 %, dual +1.1 %, console -1.7 %. The earlier single runs read -4.5, +0.3, +1.0
  and -3.7 %: within V8's per-process spread.
* **Class A.** #992's proof 3: the seed can change only the sign of a zero, and the phases feed only
  `abs`.
  * A NaN phase makes the peak NaN, whatever its payload, and a NaN peak makes `r` exactly 1.0.
    NaN payloads therefore never reach state or output.
  * LLVM already commutes these `fadd`s: the base listing shows `vaddps ymm0,ymm9,ymm0`.
* **wasm risk:** none. The roster row stays one function and rule 3's count is unchanged.
* **Worst case:** helps, since it applies to every block.
* **Relation to #992:** this is #992's contract 2 alone. #992 as a whole measured +8-9 % natively in
  the verification. Its contract 1 (`P_MAXDIV`, measured here in steady frames) was a net loss.

### 2. van Herk completion as a segment boundary, and the release recursion as its own pass (`issues/limiter-2-2-van-herk-segments-and-recursion-pass.md`)

In the uniform dual and linked bodies, cut each wrap-free segment also at the frame where the van
Herk block completes.

* **Steady frames.** Every frame before a segment's last is steady: no `position == 0` branch, no
  completion branch, no nested backward pass. `prefix` is preset to `+inf` when a segment starts at
  phase 0, and D8's `min(+inf, x)` is `x` for every `x`, NaN included.
* **Pass 1** runs steps 1-5 and `1 - s` for the steady frames into a stack scratch. The segment's
  last frame goes through today's function.
* **Pass 2** runs the recursion and step 7, with `d` in a register.
* **Instructions per frame (`Simd8`):** linked 119 → 63 + 35, dual 196 → 111 + 45. The V8 linked
  pass 1 is 105 instructions, 68 of them GPR.
* **Measured with change 1, against pristine.**

  | row | `Simd8` | `Simd4` |
  |---|---:|---:|
  | dual | -8 % | -6 % |
  | linked | -3 % | -5 % |
  | quiet | -2 % | -5 % |
  | ramping | **+8 %** | **+3 %** |

  * Console isolate: `Simd8` -5 % / -4 %, `Simd4` -3 % / -4 %.
  * V8: linked -2.6 %, dual -1.9 %, console -3.4 %.
* **Increment over change 1 alone:**
  * native: dual -2 to -3 %, linked +0.5 to +1.6 %;
  * V8: dual -3.0 %, linked +0.5 %, console -1.8 %.
* **Why it ranks second.** Its own increment is small and falls mostly on dual blocks
  (`dual_mono`, or asymmetric stereo). It is the structural precondition for change 3, whose streams
  are uniform only inside completion-free segments.
* **The ramping regression is the risk.** Restricting the change to the stationary dispatch made the
  stationary rows lose their gain too (section 4). Owner ruling in section 8.
* **Class A.**
  * Steady frames cannot complete: with `run <= window - phase` on both channels, frame `s`
    completes only if `s == run - 1`.
  * The passes exchange nothing within a segment except `targets`. Each ramp is advanced once per
    frame in frame order: the limit ramp in pass 1, the release ramp in pass 2.
  * Identity held on the #990 oracle suite, the differential at W8, W4 and W1, the console digests
    and the V8 digests.
* **wasm risk:** the roster vector count rises 910 → 926 (the split duplicates code). Rule 3 and the
  callgraph are unchanged.

### 3. Bounds-check-free steady streams (`issues/limiter-2-3-bounds-check-free-steady-streams.md`)

* **The prize.** Rust bounds checks and their address arithmetic are 3.5-10 % natively and 6-9 % under
  V8 (`X_UNCHECKED`, a diagnostic that is not shippable). That is the largest V8 lever found.
* **Why the steady frames allow it.** Inside a completion-free, wrap-free segment of `steady` frames
  with `window < ring` and `steady <= ring - window`, the rings are uniform streams:
  * the window's newest slot and the box term leaving the sum are never written before they are
    read;
  * the oldest slot (`cursor + 1 + s`) is read at frame `s` and overwritten at frame `s + 1`.
* **The measured prototype lost.** `P_BCF` copies the three read streams out at segment entry and
  zips `chunks_exact` views. The loop fell to 24 instructions (`Simd8`, no branch, `llvm-mca` 7.1
  cycles), yet the whole kernel measured +4-6 % natively. That held with `memcpy` copies and with
  lane-sized ones.
* **The brief specifies a no-copy form:**
  * split borrows for the newest and expiring streams;
  * a skewed walk that reads each cursor slot's old value before overwriting it;
  * a stop rule if the three-target timing gate fails.

  This change is ranked on its prize, not on a measured implementation.

### Not worth doing (measured)

| candidate | native `Simd8` | native `Simd4` | V8 | why |
|---|---:|---:|---:|---|
| phase-major detector (`P_DETPM`) | +31-49 % | +27-50 % | +9 % | one 12-add chain per pass-frame: too few chains in flight |
| tap-buffer detector, one frame (`P_DETTM`, with the seed) | +21-27 % | +15-19 % | -1.8 % linked, +2.3 % dual (3-process mean; three earlier single runs read -5 %) | loading taps adds about 8 cycles to every chain; V8 still reloads 48 coefficients (191 instructions against 195) |
| tap buffer, two frames per iteration (`P_DETTM2`) | +22-25 % | +20-25 % | +5 to +14 % | as above; V8 spills |
| phase-pair passes (`P_DETPP`) | +34-44 % | +35-51 % | -3 % | two chains per pass-frame |
| two-channel detector (`P_DETLR`) | not measured | not measured | **+35 %** | V8 spills 8 accumulators: 201 instructions per channel-frame, 148 of them memory moves |
| segment cut in the fused loop (`P_VHSEGF`, with or without the seed) | +3-7 % | +3-9 % | -1 to +9 % | the fused steady loop compiled worse |
| segment cut without the pass split (with the seed) | +7-26 % | +6-20 % | not measured | ditto |
| read streams copied, bounds-check-free (`P_BCF`) | linked +3-5 %, ramping +25 % | linked +4-6 %, ramping +24 % | not measured | see change 3 |
| pass split alone (`P_GSPLIT`) | -5 to +1 % | -1 to -5 % | ±1 % | superseded by change 2 |
| detector chunk 64 or 128 | ±2 % | -1 to -2 % | -1 % | noise |
| ramping and per-lane bodies out of line (`P_COLD`) | +9 % | +7-10 % | not measured | section 4; also inlines `process_block` away, which the wasm roster names |
| detector out of line (`P_OUTDET`) | +11-15 % | -1 to -3 % | not measured | not robust |
| `l / max(p, l)` in steady frames (`P_MAXDIV`) | +5-16 % against change 2 | +5-20 % against change 2 | not measured | codegen; and not class A outside the stationary dispatch (section 6) |
| class-B pairwise tree (round 1) | -8 to -25 % | | **+50 %** | loses in the product runtime |

Not re-measured here: #991 (lean block state). The fixed per-block term it targets is 0.84-1.11
natively and 2.0-2.2 under V8 (about 10 %). The verification measured #991 alone at +6-9 % native and
-0 to -2 % V8. Section 4 explains that sign. It should be re-timed only on top of whichever of
changes 1-3 lands, under the same gate.

## 6. Correctness notes

* **The differential discriminates.** `P_MAXDIV` was built into the steady frames of both
  dispatches, and it diverged at W8 (`4fa1822f…` against `636a7926…`). The scenario generator
  restores a ramp whose `step` is `f32::MAX`, which walks the limit to `+inf`. There,
  `select(p > l, l / p, 1)` gives 1.0 and `l / max(p, l)` gives NaN.
  * This confirms the verification's #992 finding: the select-free gain computer is class A only in
    the stationary dispatch.
  * No other prototype read differently anywhere.
* **Crate suite.** `cargo test --release -p true-peak-limiter` passed 50 of 50 with the change-2
  build and with the tap-buffer build. That includes #990's reference-kernel oracle, D90, the
  linked scenario pins, `mono_collapse` and `allocation`.
* **The render stays allocation-free.** Every scratch the prototypes add is a fixed stack array of
  at most `(11 + 32) x 8` words, and `allocation.rs` passes.
* **Dev-profile stack.** The split adds three 1 KiB arrays. The verification measured the dev frames
  at 245-274 KB against a 2 MB test-thread stack.

## 7. Reproduction

```text
git apply docs/handoffs/effects-2026-09-27/limiter-diagnosis-2-prototypes.patch
cd docs/handoffs/effects-2026-09-27/limiter-diag-2-harness   # set D= in scripts/*.sh to a scratch directory
scripts/build.sh pristine -                   # LIMDIAG2 unset; then e.g. build.sh seed 4
scripts/build.sh vhseg-seed 65556             # 65536 | 16 | 4
flock -w 7200 <lock> scripts/final-native.sh > raw.txt; python3 scripts/summ.py raw.txt pristine
scripts/build-web.sh base -; scripts/build-web.sh seed 4
python3 scripts/sessions.py <repo>/fixtures/session/v1/console-sixty-four-track-intended.json $D/web
node scripts/web.mjs digest web/base.wasm web/seed.wasm
flock -w 7200 <lock> scripts/final-web.sh
bin/<arm> diff 8 0 300 96                     # the #990 differential; compare the combined digest
scripts/dis.sh <arm>; python3 scripts/loops.py dis/<arm>-w8.s 20; scripts/mca.sh <arm>-w8.s <addr> <name>
scripts/v8all.sh <arm>; python3 scripts/v8loops.py dis/v8-<arm>.txt 100
```

The bit values are in section 3, and composites add them.

## 8. For the owner

1. **The detector's gap is algorithmic.** Natively it runs at 34-46 cycles per channel-frame
   against a port bound of 26-28; under V8 at about 55. That gap belongs to the frozen tap order
   (four 12-deep add chains per frame). Class A cannot close it: seven shapes were tried, and only
   the one-add shortening helped. Under `docs/rulings/effect-floor-accounting.md` this is a class-B
   gap, to be flagged and not chased. The pairwise tree, the obvious reorder, measured +50 % under V8
   in round 1. Recommendation: record "frozen Annex-2 summation order" as the named reason for the
   detector's residual, and do not brief a reorder.
2. **Ramping blocks against stationary blocks.** Change 2 measured +3 % to +8 % on blocks that
   carry a de-zipper ramp, and gains on stationary blocks. Ramps run for at most 64 samples after an
   automation event, so in a mix they are a small share of blocks. Rule whether the change-2 gate
   may accept a bounded ramping regression (say, at most +5 %). The alternative is holding ramping
   at no regression, which the prototype could not do (section 4).
3. **Accept the instability explicitly.** Limiter slices should be accepted only by the three-target
   timing gate on the final build: separate binaries, minimum over rounds natively, the mean of three
   Node processes under V8. Instruction counts do not predict the result. The same likely applies to
   #991 and the rest of #992.
4. **The browser lever is bounds checks** (-6-9 % under V8). None of the detector shapes moves V8
   beyond its ±2-4 % process noise. If change 3 fails its gate, the next step is a lane-crate
   question, not a limiter one: a way to address a ring as `[[f32; W]]` so that one compare per slot
   is left for LLVM to hoist. That needs its own issue.
5. **Round 1's open items stand:** which host the floors refer to (this Zen 3 host or the Zen 5 host
   of #184), and whether a linked pair's gain path counts once.
