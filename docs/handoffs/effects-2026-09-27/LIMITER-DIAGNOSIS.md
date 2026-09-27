# True-peak limiter performance diagnosis (2026-09-27, overnight effects directive)

Base: `codex/batch-plumbing-floor-2` at `6ca203f8` (the limiter crate is unchanged on that branch
since). Branch `limiter-diagnosis`. Diagnosis only: every prototype below was measured and then
reverted, and the tree carries none of them. The prototypes, the two in-process harnesses and the
wasm guest switch are one patch, `limiter-diagnosis-prototypes.patch` (applies to `6ca203f8`); the
Node driver is `limiter-diagnosis-wasm-console.mjs.txt`, the machine probe is
`limiter-diagnosis-throughput-probe.rs.txt`, and every raw timing output is in
`limiter-diagnosis-raw-timings.txt`. The draft issues are `issues/limiter-1-*.md` to
`issues/limiter-5-*.md`.

## Verdict

* **The limiter costs about 12 cycles per lane-sample natively at `Simd8`, 21 at `Simd4`, and
  26-28 under V8 at `simd128` (the browser), and its cost does not depend on the signal.** It is
  half detector, 40 % gain loop and 5-7 % block bookkeeping. The bank-chain round trip it adds to
  the console is 0.05-0.2. It measures the same on material that never limits as on material that
  limits every block.
* **Both hot loops run at about half their port-bound speed, and the arithmetic is not what limits
  them.** The detector spends about a fifth of its instructions (21-22 of 133) rotating its
  twelve-tap history through the sixteen vector registers. The gain loop carries 223 instructions per frame for 49
  vector operations, so the reorder buffer holds about one frame and the frame's ~55-cycle
  dependency path is exposed. Removing both divides moved nothing.
* **The largest class-A savings are data-dependent, and they are large.** The detector's peak can
  be bounded exactly from the Annex-2 table (every phase is at most 2.0228 times the largest tap).
  When every tap a block's detector would read is below `0.49 x ceiling`, every required gain is
  exactly `1.0`:
  * the detector can be skipped (**-48 % native `Simd8`, -51 to -53 % `Simd4`**, prototyped);
  * once the gain rings have also settled at unity, the whole limiter is a delay line
    (**-88 % / -90 %**, prototyped).

  On the standing console row the fixture sits at that threshold, so the saving there is partial:
  **-31 % of the limiter under V8** (with the linked computer and the cleanups: -40 %). None of
  this helps the worst case (every track limiting).
* **The largest data-independent class-A saving is the linked gain computer.** Under
  `link = maximum` with equal ceiling, release and lookahead on the two channels (the fixture, and
  the usual stereo setting), both channels compute bit-identical gain paths. Computing it once
  saves -9 to -12 % natively and -14 to -19 % under V8.
* **The 129.5 lane-op inventory omits the §4.4 boundary scan (+3) and counts 9 lane-ops that are
  not required.** As implemented today the count is 132.5. The class-A minimum is 125.5, and 116
  if a linked pair's gain path is counted once (an owner ruling). This host is Zen 3, not the
  Zen 5 `floor.rs` was measured on: its mixed rate is 3.88 operations per cycle, not 3.7.
* **The two Boundary-4 residuals are gone from the uniform path.** Round 2 fixed both: R1(b)
  returns the van Herk minimum in a register, and R1(a) keeps the phase in a local that is written
  once per block. Both remain in the ragged per-lane path, where they are inherent. That path costs
  2.6x the uniform one at `Simd8` and 2.0x at `Simd4`.

## Method

* **Host.** AMD EPYC 7313P (Zen 3), 3.70 GHz, Linux 6.8; rustc 1.97.1 / LLVM 22.1.6; release
  profile (fat LTO, one codegen unit), `+avx2,+fma`. Node 22.23.2 (V8 12.4.254, TurboFan) for
  wasm. Every timed run held the shared timing lock
  (`flock -w 7200 .../scratchpad/timing.lock`, each hold under 20 minutes), pinned with
  `taskset -c 31`. The load average is printed in each raw record: 2.3-5.9 on most runs, and
  6.5-10.5 on two console runs (cpu 31 stayed pinned; comparisons are within one runtime,
  interleaved). `perf` is unavailable (`perf_event_paranoid` 4), so cycles are derived:
  nanoseconds times a core clock calibrated in process from a dependent `vaddss` chain (3 cycles
  per add), which read 3.695-3.704 GHz on every run.
* **Kernel harness** (`crates/true-peak-limiter/tests/limiter_diag.rs` in the patch). It uses the
  public bank API: 64 tracks as 8 `Simd8` or 16 `Simd4` banks, with the fixture's per-track
  parameters (ceiling `-0.5 - 0.03125 k` dB, release `60 + 1.25 k` ms, lookahead 5 ms, link
  `maximum`, 48 kHz, 128 frames). The signals are:
  * the fixture tone at the level the limiter sees (0.3 peak; it never limits);
  * seeded noise at +3 dBFS (limiting every block);
  * silence;
  * two quiet/loud mixes.

  One `Instant` pair per block covers all banks. The p50 of 1,500 blocks is taken per round, four
  rounds, with the order reversed on alternate rounds. Prototypes are selected per block by
  `DIAG_VARIANT` (a hidden atomic read once per block), so an A/B runs on one set of banks at one
  set of heap addresses.
* **Console harness** (`tools/console-workload/tests/limiter_diag_console.rs`).
  `sixty_four_track_console` minus `sixty_four_track_eq_comp_simd1` (the directive's isolate),
  plus `console_legacy`, on the real compiled plan at `Simd8` and `Simd4`. It has the same round
  structure.
* **wasm.** `wasm-console-guest` is built `+simd128` with one added export,
  `miso_diag_set_limiter_variant`. The same rows are timed under Node with `process.hrtime` around
  `miso_console_render`, one block per sample, 600 blocks and 6 rounds per arm. The source tone is
  staged from JS. This is V8, not Chrome.
* **Identity.** Every class-A prototype was checked against the base path:
  * output words and full per-track state snapshots, from fresh banks, over 400-900 blocks;
  * console, limiting, quiet/loud-burst and silent signals;
  * `Simd4` and `Simd8`, link `maximum` and `dual_mono`;
  * block lengths 1-128 (including 1, 5, 11, 12, 13, 31-33 and 127);
  * all five E12 corpus digests (`corpus::D90_DIGESTS`) at `Simd4` and `Simd8`, with the corpus
    routed through the variant switch;
  * every native console digest over 200 blocks, and the wasm console digest.

  A deliberately class-B variant (a reciprocal instead of the box divide) reads DIFFERENT on the
  limiting signal, so the harness discriminates. All class-A variants read SAME everywhere.
* **Caveat on digests.** The standing console fixture barely limits, so its digest cannot see a
  detector change. The class-B pairwise-tree detector left every native console digest unchanged
  (it moved the wasm one, whose JS-staged tone differs slightly from the native tone). The D90 corpus, whose noise
  cases limit, is the discriminating gate for detector work.

## 1. Where the time goes

Cycles per lane-sample (a lane-sample is one channel of one track for one sample). The component
rows come from truncated kernels in the kernel harness (state only, state plus detector, state
plus gain loop; see `X_*` in the patch); they add up to the whole within 2 %.

| term | native `Simd8` | native `Simd4` | wasm `simd128` (V8) |
|---|---:|---:|---:|
| **console isolate** (console minus eq_comp_simd1, in process) | **12.2-12.4** | **20.1-21.0** | **25.2-28.3** |
| kernel harness, whole limiter, 64 tracks | 11.5-12.0 | 20.5-21.2 | - |
| per-call fixed cost: dispatch, the per-lane automation walk, the silence admission | 0.11-0.12 | about 0.15 (estimated) | 0.16 |
| block state in and out: `HotChannel::load`/`store` (out of line, 5.5 KB of code), scalar ramp gather and scatter, peak-scratch zeroing | about 0.50 | about 0.55 (estimated) | about 1.3 |
| §4.4 boundary scan (`finish_block`) | 0.10-0.15 | about 0.15 (estimated; the three `Simd4` bookkeeping rows total 0.83-0.96) | 0.45 |
| **Annex-2 detector** (48 mul, 48 add, 5 abs, 4 max per channel-frame) | **6.0-6.5** | **11.7-12.2** | **14.1** |
| of which the loop alone, isolated with L1-resident input | 4.4-4.5 (35.8 cycles per channel-frame) | 9.8-10.1 (39.3-40.3) | - |
| **gain loop**: gain computer, van Herk, quantise, box, release, gain, delay line, backward pass | **4.4-5.0** | **8.0-8.2** | **11.3** |
| of which the two divides (both removed, as a diagnostic) | +0.01 / -0.01: not binding | - | - |
| bank chain for the limiter's chain (console with the limiter as a no-op, minus eq_comp) | 0.05-0.20 | 0.05-0.62 | 0.03-0.43 |
| working set: 1 bank against 8 (16 at `Simd4`; 741 KB of rings at `Simd8`, 512 KB L2) | +0.5 (11.13 to 11.67) | +0.5 (20.41 to 20.90) | - |
| console cache interference (console isolate minus kernel harness) | about +0.6 | about 0 | - |

Other paths, per lane-sample:

| path | native `Simd8` | native `Simd4` |
|---|---:|---:|
| silent block (the #182 S2 skip) | 0.38-0.39 (783-798 cycles per bank-block) | 0.42-0.44 |
| ragged cohort (mixed lookaheads; the per-lane body) | 30.3 (2.6x uniform) | 42.4 (2.0x) |
| `dual_mono` link | 11.50 (the same as `maximum`) | 20.82 |
| limiting signal (+3 dBFS noise) | 11.50 (the same as the fixture tone) | 20.89 |

Notes on the terms:

* **The detector is not port-bound.** The isolated loop runs at 35.8 cycles per channel-frame at
  `Simd8` and 40 at `Simd4`. Zen 3's full-width FP units make the two the same port cost, about
  26 cycles. The loop carries 21-22 register-to-register moves per frame, the history shift,
  because twelve taps and four accumulators fill all sixteen `ymm` registers. It also carries 7-15
  memory operations beyond the 48 coefficient operands (spills, reloads, the input and the peak).
  Counting those moves as FP-pipe work predicts 32 cycles. In place, the detector costs about 14
  cycles per channel-frame more than in isolation (50 against 36). The instruction count is the
  same, a heap peak scratch changes nothing, and the detector-only truncation shows the penalty
  without the gain loop. The mechanism was not identified.
* **The gain loop is latency-bound through the reorder buffer.** One frame has a dependency path of
  about 55 cycles (divide, quantise, box, divide, release recursion, gain, store) and 223
  instructions (section 2). The 256-entry ROB therefore holds about one frame, and the loop runs
  at about 80 cycles per frame. The divider is 3.5 cycles per `vdivps ymm` on this host, four per
  frame, and it is hidden. Splitting the loop per channel is slower (+0.15-0.20), because each
  channel's loop is then latency-bound alone. Loop fission (required gains first) is flat.
* **Memory is small.** The rings (per channel `B = 486`, `R = 481`, `R = 481` slots, 46 KB per
  channel per bank at `Simd8`) overflow L2 at 64 tracks. The streams prefetch, and the whole
  working-set effect is 4 %.
* **wasm.** V8 keeps the 48 coefficient splats as wasm locals and reloads them: 79 `vmovups` per
  channel-frame. The frame loop is about 400 x86 instructions for 73 FP operations. The detector
  is 53 % of the wasm limiter and the gain loop 43 %.

## 2. Emitted code

Hot loops of `LimiterCore::<L>::process_block`, uniform cohort, stationary dispatch. x86 from the
release test binary built from the unmodified tree (`objdump -d`); wasm from the release guest
(`wasm-objdump -d`, function 1772) and V8's TurboFan output (`node --print-wasm-code`).

| loop | instructions | arithmetic (floor ops) | the rest |
|---|---|---|---|
| x86 `Simd8`, detector, per channel-frame (8 lane-samples) | 133 | 105: `vmulps` 48 (all with a memory operand), `vaddps` 48 (4 of them `+0.0 + x`), `vandps` 5, `vmaxps` 4 | 21 `vmovaps` register rotations and spills, 2 `vmovups`, 1 `vbroadcastss`, 1 `vxorps`, 3 loop |
| x86 `Simd4`, detector (isolated build) | 177 | 105 | 42-47 `vbroadcastss` (the coefficients rematerialised from constants), 25-35 `vmovaps` |
| x86 `Simd8`, gain frame loop, per frame (both channels, 16 lane-samples) | 223 | 49 vector, including 4 `vdivps`, 6 `vblendvps`, 4 `vminps`, 2 `vroundps` | ~40 bounds-check `lea`/`cmp`/`jae`; ~20 reloads of spilled loop invariants (ring bases, offsets); 9 `vbroadcastss` constants; both channels' reduction words and the right box sum stored to and reloaded from the stack every frame |
| x86 `Simd8`, ragged per-lane frame loop | 647 | 49 | the scalar van Herk and box-expiry gather per lane |
| wasm, detector, per channel-frame | 258 wasm / ~199 x86 under V8 | 105 | V8: 79 `vmovups` (coefficient and tap reloads) |
| wasm, gain frame loop, per frame | 484 wasm / ~459 x86 under V8 (with the backward-pass inner loops) | 73 FP | 126 `i32` address and bounds operations, 21 `br_if`; each `v128.bitselect` is three x86 instructions |

**Instructions per lane-sample against the floor's operation count.** The floor is 129.5 vector
operations per channel-frame, which is 16.2 per lane-sample at `Simd8`. The emitted code is about
33 per lane-sample at `Simd8`: detector 16.6, gain loop 13.9, backward pass about 0.7, block
bookkeeping about 1.8. That is 2.0x the floor, retired at an IPC of about 2.7 against the probe's
3.9. Under V8 at four lanes the code is about 105 x86 instructions per lane-sample against 32.4,
which is 3.2x, at an IPC of about 3.9. The scalar remainder is the bookkeeping named above. The
arithmetic stays fully vectorised on every target.

## 3. The floor, rechecked

The machine probe (`limiter-diagnosis-throughput-probe.rs.txt`: the ruling's probe, timed with
the calibrated clock, with `max` and `div` added), pinned to cpu 31 on this Zen 3 host:

| stream | vector ops/cycle |
|---|---:|
| `vaddps` | 2.00 |
| `vmulps` | 2.00 |
| `vmulps` then `vaddps` | **3.88** |
| `vcmpps` then `vblendvps` | 4.00 |
| `vmulps` then `vmaxps` | 4.00 |
| `vdivps` | 0.287 (3.5 cycles each) |

`floor.rs` uses 3.7, from the Zen 5 host of #184. Every floor on this host is 4.6 % lower.

**The inventory, recounted on the current code** (per lane-sample):

| stage | ruling | as implemented | required (class-A minimum) | why |
|---|---:|---:|---:|---|
| A detector FIR, 12 taps x 4 phases | 96 | 96 | 92 | Each accumulator starts at `+0.0` and adds its first product. That add changes only the sign of a zero, which the following `abs` erases (proof in `issues/limiter-5`). |
| A `abs(h[6])` and four `max(abs)` | 9 | 9 | 9 | |
| B stereo link | 1.5 | 1.5 | 0.5 | The two selects are on the prepared `link_max` boolean; only the `max` is required. |
| C ramps (stationary hoist) | 0 | 0 | 0 | |
| D gain computer | 3 | 3 | 2 | `select(p > l, l / p, 1)` is `l / max(p, l)` bit for bit (proof in `issues/limiter-5`). |
| E van Herk, amortised | 3 | 3 | 3 | Exactly `3 - 2/Wb`. |
| F quantise | 3 | 3 | 3 | |
| G box sum and its divide | 3 | 3 | 3 | |
| H release and flush | 8 | 8 | 8 | |
| I gain and output | 3 | 3 | 2 | The bypass select is on a prepared boolean. |
| J §4.4 boundary scan: `abs`, `lt`, `mask_and` | **missing** | 3 | 3 | `finish_block` scans both planes every block. The EQ and compressor inventories count this row; the limiter's omits it. |
| **total** | **129.5** | **132.5** | **125.5** | |
| linked pair (link `maximum`, equal designed words): D-H once per pair | | | **116** | D-H is 19 lane-ops per channel, halved: an owner ruling (section 7). |

Floors on this host (`Simd8`, 3.88 ops/cycle): 132.5 gives 4.27; 125.5 gives 4.04; 116 gives
3.74. `floor.rs` today gives 129.5 / (8 x 3.7) = 4.375.

**Two divides per lane-sample** are counted as one op each. On this host they occupy the divider
for 0.875 cycles per lane-sample, below the ALU floor, and measurement shows they are not binding.

**The gap, named** (console isolate 12.2-12.4 against 4.04-4.375; the terms overlap under
out-of-order execution, so they over-add by about 1):

1. The detector implementation is +2.8 over its 3.38 arithmetic floor. It is made of the
   history-rotation moves and spills (the isolated loop at 4.47 against 3.38) and an in-place
   penalty of +1.8 whose mechanism was not identified (section 1).
2. The gain loop is +4.4 over about 0.8. That is 174 bookkeeping instructions per frame and ROB
   latency-binding.
3. Block bookkeeping is +0.75: state in and out 0.5, boundary scan 0.15 (floor-counted, but a
   separate pass), per-call 0.12.
4. Memory and cache interference are about +1.1 (working set +0.5, sharing the core with EQ and
   compressor +0.6).
5. The bank chain is +0.05-0.2.

None of these is class B.

## 4. Ranked changes

Δ is cycles per lane-sample against base in the same run (negative is a saving). Ranges span
repeated runs and two prototype builds. The console columns are the directive's isolate on the
real plan. "Worst case" says whether the change also helps when every track limits every block.
Every change listed is allocation-free and adds no `unsafe`.

1. **Quiet-block detector screen** (`issues/limiter-1-quiet-block-detector-screen.md`). Class A:
   exact bound, data-dependent, block level.
   * What it does: when every tap the block's detectors read is below `0.49 x min(limit_L,
     limit_R)` on every lane (stationary uniform bodies only), skip both detector passes, set
     `r = 1.0`, and advance the history from the block's last 12 inputs.
   * Kernel harness, quiet material (engaged every block): **`Simd8` -5.7 (-48 %), `Simd4` -10.5
     to -11.0 (-51 to -53 %)**.
   * Console fixture (at the threshold): `Simd8` +0.8 (it never engaged), native `Simd4` -2.6,
     **V8 -4.3 to -4.9 (-17 %)**.
   * Never engaged: +0.55 in the prototype. That is codegen perturbation of the unscreened body,
     not the scan; the brief requires a separate screened body.
   * Worst case: no.
   * wasm risk: none beyond a second body inside `process_block`, which the roster regex already
     covers.
2. **Unity rest fast path** (`issues/limiter-2-unity-rest-fast-path.md`, after 1). Class A:
   induction plus exact bound.
   * What it does: after `2R + 2Wb` consecutive screened frames the required-gain and box rings
     have settled at exactly `1.0`. A scan of those words earns a claim, as the #182 S2 silent
     claim is earned, and while it holds a screened block reduces to the delay line plus the
     release recursion.
   * Prototype (the `d == +0.0` arm): quiet material **`Simd8` 11.65 to 1.38 (-88 %), `Simd4`
     20.8 to 1.98 (-90 %)**.
   * Console fixture under V8: **-8.4 (-31 %)**; with changes 1, 3, 4 and 5: -10.7 (-40 %).
   * Never engaged: +0.11 to +0.23.
   * The prototype required `d == +0.0`, and a release takes about 45 time constants to flush to
     `+0.0` (2.7 s at 60 ms). The brief therefore specifies the release-only body, which is valid
     while `d` decays.
   * Worst case: no.
3. **Linked gain computer** (`issues/limiter-3-linked-gain-computer.md`). Class A, induction.
   * What it does: under `link = maximum`, while the channels' designed words and gain state agree
     bit for bit, compute the gain path once, mirror its ring writes into the right channel, and
     apply one gain to both delay lines.
   * Kernel harness: **`Simd8` -1.2 to -1.5 (-10 to -12 %), `Simd4` -2.2 to -2.5**.
   * Console: `Simd8` -1.1 to -1.2, `Simd4` -1.8 to -2.2.
   * **V8 -3.7 to -5.5 (-14 to -19 %)**.
   * Worst case: **yes**.
4. **Lean block entry and exit** (`issues/limiter-4-lean-block-state.md`). Class A.
   * What it does: in the stationary body, load only the two ramps' `current` and one window
     splat, skip the ramp scatter, and inline the load.
   * State term: -0.28 (`Simd8`), -0.26 (`Simd4`), -0.41 (V8).
   * Whole kernel: -0.2 to -0.5 (`Simd8`), -0.2 to -0.36 (`Simd4`), -0.07 to -0.47 (V8).
   * Worst case: yes.
5. **Select-free gain computer and prepared-boolean arms**
   (`issues/limiter-5-select-free-gain-computer.md`). Class A.
   * What it does: `r = l / max(p, l)`, the link and bypass selects become block-level arms, and
     each Annex-2 accumulator starts at its first product.
   * Floor: -7 lane-ops.
   * Measured together with the ramp-scatter skip: `Simd8` -0.1 to -0.17, native `Simd4` -0.6 to
     -0.8, V8 -0.14 to -0.23.
   * Worst case: yes.
6. **Gain-loop bookkeeping.** Class A, not briefed. Hoist the ring bounds checks and loop
   invariants out of the frame loop: pre-sliced streams per wrap-free segment, recursive words in
   locals. This is the limiter's analogue of `compressor-1`. The cheapest form, segment-entry
   assertions, measured -0.19 (`Simd8`), -0.54 (`Simd4`) and about 0 (V8). A full rewrite is
   unmeasured. The required-ring `start` stream overlaps the cursor stream by one slot, which is
   why plain `chunks_exact_mut` streams do not apply.
7. **Paired detector frames.** Class A, not recommended as briefed. Two frames interleaved per
   iteration give eight independent chains: -0.4 to -0.6 in place at `Simd8`, about 0 at `Simd4`,
   -0.5 under V8. The same loop isolated is 20 % *slower* (43.7 against 35.8 cycles per
   channel-frame). LLVM's generic x86 scheduler reorders the chains to save registers, so the win
   depends on the surrounding code. A brief would need a codegen gate.
8. **Silent-block trim.** Class A, small. Advance the rest phase once per uniform cohort instead
   of per lane (measured -0.03 of the 0.39 silent block), and skip the per-lane automation walk
   when a block carries no spans (unmeasured; part of the 0.12 per-call cost).

**Measured and not worth doing:**

| candidate | measured | why not |
|---|---|---|
| detector with taps read from memory (whole-block pass) | +0.25 to +5.3 `Simd8`, +1.0 to +10.7 `Simd4` (two builds) | LLVM emits each phase's 12-add chain back to back, which serialises the loop |
| detector as a pairwise tree (class B, a summation reorder the owner allows) | native -0.9 to -2.9; **V8 +14.3** | Slower in the product runtime. Not pursued. |
| exact f64 divide for the box average (`(s as f64 * (1/Wb)) as f32`, proven equal) | +0.28 to +0.54 | the divider is not the bottleneck |
| boundary scan fused into the frame loop | +0.32 / -0.24 | adds register pressure to the frame loop for a 0.15 pass |
| per-channel gain loops | +0.15 to +0.20 | each loop becomes latency-bound alone |
| loop fission (required gains first) | -0.25 `Simd8`, +0.48 `Simd4`; -0.6 only together with 3 | not robust |
| heap peak scratch instead of the stack array | 0 / +0.12 | not the in-place detector penalty |
| reciprocal of `Wb` instead of the divide (class B) | 0 with both divides removed | nothing to win, and it moves bits |

## 5. wasm risk, per change

All five briefs keep the arithmetic in `f32x4` inside `LimiterCore<f32x4>::process_block`. That is
roster row "true-peak-limiter f32x4 dual" (`scripts/check-web-audioworklet-callgraph.py:195`) with
its 0.10 scalar ceiling, and rule 3's kernel count does not drop.

* 1 and 2 add a second and a third body to the same function. The body a quiet block takes is
  smaller than today's, and the scan is `f32x4.abs`/`lt`/`and`.
* 3 adds a frame body with fewer vector operations.
* 4 moves scalar loads, not arithmetic.
* 5 removes `v128.bitselect`s, which are three instructions each under V8.

The mono (collapsed) roster row is untouched by all five.

## 6. Reproduction

```text
git apply docs/handoffs/effects-2026-09-27/limiter-diagnosis-prototypes.patch
CARGO_INCREMENTAL=0 cargo test --release -p true-peak-limiter --test limiter_diag --no-run
flock -w 7200 <lock> taskset -c 31 <limiter_diag binary> --ignored --nocapture --test-threads 1 \
    rows_w8 | rows_w4 | rows2_w8 | rows3 | rows4 | final_rows | screen_rows | unity_rows | \
    signals_and_shapes | detector_micro
<limiter_diag binary> --ignored --nocapture identities identities_ragged_frames \
    corpus_digests screen_identity unity_identity
CARGO_INCREMENTAL=0 cargo test --release -p console-workload --test limiter_diag_console --no-run
flock ... taskset -c 31 <limiter_diag_console binary> --ignored --nocapture console_rows
RUSTFLAGS="-C target-feature=+simd128" cargo build --release --target wasm32-unknown-unknown \
    -p wasm-console-guest
flock ... taskset -c 31 node limiter-diagnosis-wasm-console.mjs.txt <guest.wasm> 3.7e9 \
    breakdown | final | screen | unity
```

`final_rows` was run on a build whose `variants!` list held only its seven variants. The patch
carries the full list, and the final-row numbers above are from the reduced build. The full list
perturbs codegen by up to ±0.3 cycles per lane-sample, and V8 more.

## 7. For the owner

1. **Floor recount.** The limiter inventory should read 132.5 as implemented, because the
   boundary scan is missing today. The class-A minimum is 125.5, and brief 5 reaches it.
   `OPS_PER_CYCLE` is a Zen 5 figure; this Zen 3 host measures 3.88. Which host is the floors'
   reference?
2. **A linked pair's gain path counted once** (125.5 to 116 lane-ops) is the same question the
   ruling leaves open for mono collapse: does a value determined by another lane-sample count as
   required arithmetic?
3. **Data-dependent fast paths (briefs 1 and 2).** They are class A and follow the #182 S2
   silent-path precedent, but they engage on non-silent audio. They cut average CPU (-48 to -90 %
   on quiet material), not the worst case. The standing console fixture sits at the screen
   threshold, so the console row would move differently at `Simd8` and `Simd4`/wasm. A
   quiet-material row (or the silence design's sparse rows, `docs/handoffs/silence-2026-09-27/`)
   would show the product effect honestly.
4. **Class B beyond a summation reorder, flagged and not chased.** The Annex-2 table is
   pair-symmetric (`h[k][p] = h[11-k][3-p]`). Sum-and-difference factoring, and
   `max(|a+b|, |a-b|) = |a| + |b|` for each phase pair, would cut the detector's 105 lane-ops to
   about 64. It changes rounding by the distributive law, so it needs BS.1770 conformance evidence
   and a ruling. No timing is projected.
5. **The pairwise-tree detector** is class B inside the standing reorder ruling. It measured
   faster natively but 50 % slower under V8. Recommend not pursuing it.
6. **Outside the limiter:** native `Simd4` `eq_comp_simd1` is 462 us per block against 176 us in
   wasm at the same width, so native `Simd4` is 2.6x slower than V8. That is the EQ/compressor
   `Simd4` path, not the limiter; see `COMPRESSOR-DIAGNOSIS.md` on the scalar and `Simd4` rows.
7. **Browser numbers are Node/V8 12.4, not Chrome.**
