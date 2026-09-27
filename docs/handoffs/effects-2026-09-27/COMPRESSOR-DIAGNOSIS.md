# Compressor performance diagnosis (2026-09-27, overnight effects directive)

Base: `codex/batch-plumbing-floor-2` at `6ca203f8`. Branch `compressor-diagnosis`. This is
diagnosis only. Every prototype below was measured and then reverted, and the tree carries none of
them. The harness and the prototypes are recorded as two patches:

* `compressor-diagnosis-harness.patch` applies to `6ca203f8`. It holds a `compressor::diag`
  hook, three examples (`diag_kernel`, `diag_probe`, and the shared `diag_support/replicas.rs`),
  a wasm guest (`tools/compressor-diag-guest`) with its Node hosts and analysis scripts, a
  console-workload test harness (`compressor_diag`), and a mode switch in `wasm-console-guest`.
  `tools/compressor-diag-guest/host/README.md` gives the commands.
* `compressor-diagnosis-prototypes.patch` applies on top of it. It holds the kernel prototypes,
  selected per block by `compressor::diag::MODE`.

## Verdict

* **The compressor is 3.2 times its class-A floor today, and most of the gap is how the frame loop
  is scheduled, not the arithmetic.** In process it measures 8.9 cycles per lane-sample (native
  `Simd8`, 64 tracks, compressor-only minus builtins-only). The floor is 2.753. The kernel alone is
  97 % of that. The frame law is one long dependent chain per channel (about 140 cycles from input
  to output), and the loop holds both channels' chains in one 241-instruction body. Zen 3 keeps
  only about two such chains in flight. `Simd4` and `Simd8` therefore cost the same per frame.
* **Four bounded, class-A changes, all prototyped in the real kernel, take the native isolate from
  39.3 to 23.0 us per block (-41 %) and the browser (V8) isolate from about 82 to 59-61 us
  (-26 to -28 %).** Every standing console digest is unchanged at both dispatch widths in each
  prototype mode. The wasm console digests match native. The replicas also agree word for word
  with today's kernel on hostile inputs at `Scalar`, `Simd4` and `Simd8`, natively and in wasm.
  In order, the four changes are:
  1. loop hygiene: chunked iteration, the recursive word in a local, and the detector match
     hoisted out of the frame loop;
  2. an all-wet output arm;
  3. a two-pass settled body that computes the targets first;
  4. the DualMono link arm, inside pass 1.
* **The collapsed (mono) body gets the same rewrite, for -38 % native and -51 % V8 at kernel
  level.** Mono stems are the product's common case. The whole `sixty_four_track_console_mono` row
  drops 9-11 % natively and 17 % under V8.
* **Moving one compressor knob nearly doubles that bank's cost.** One lane's threshold Point per
  block costs +79 % natively and +92 % under V8, and all eight lanes cost +170 %. The standing rows
  never automate, so no record shows this. It is the next diagnosis, not a prototype here.
* **The 81.5 lane-op inventory still matches the general path as written. It is not what the
  standing fixture requires.** Every bank of the fixture is DualMono with `mix = 1`, so the
  arithmetic it needs is 66 lane-ops (2.23 cycles/lane-sample). The inventory's memory note, "the
  detector gather and delay ring", describes the pre-#737 compressor and is stale.
* **The owner has five points to rule on or note, listed in section 7:**
  * three provably inactive clamps (class A by a value-range proof, but they sit in a frozen
    order, two of them in the sealed tier);
  * the floor-recount basis;
  * two class-B candidates, flagged and not chased;
  * a caveat that the browser numbers are Node/V8 12.4, not Chrome;
  * the two-pass body's stack scratch under the copy rule.

## Method

* **Host.** AMD EPYC 7313P (Zen 3), Linux 6.8. rustc 1.97.1 / LLVM 22. Release profile (fat LTO,
  one codegen unit), `+avx2,+fma`. Every timed run held the shared timing lock
  (`flock -w 7200 .../timing.lock`, under 20 minutes per hold) and was pinned with
  `taskset -c 31`. Load average during the runs was 2.1-8.1, mostly 2.5-5, and each figure below
  names its run. `perf` is unavailable (`perf_event_paranoid` 4). Cycles are therefore derived:
  nanoseconds times a core clock calibrated in process from a dependent `vaddss` chain at 3 cycles
  per add, which read 3.69-3.70 GHz on every run.
* **Console isolate.** `tools/console-workload/tests/compressor_diag.rs` (`rows`). It builds
  `sixty_four_track_compressor_only` and `sixty_four_track_builtins_only` once each. It times 5
  interleaved rounds of 2,000 blocks and reports the per-round p50 of each and their difference.
  Prototype modes are switched on the same runtime, so every A/B runs at the same heap addresses.
  It is a test binary, so it unwinds; the kernel harness is an example, which aborts, as the shipped
  profile does.
* **Kernel.** `crates/compressor/examples/diag_kernel.rs` renders one bank-block (128 frames, two
  channels) of the fixture's first-track settings (threshold -6, ratio 1.5, knee 3, attack 2,
  release 40, makeup 0, mix 1, DualMono) on 0.5-amplitude noise. It takes the p50 of 4,096 blocks
  and the median of 3 rounds, net of an empty-clock row. It times three things: the real kernel
  (through `compressor::diag::DiagBank`, at any width), `Instance::render`, and the public
  `bind_homogeneous_bank`/`process_bank`.
* **Replicas.** These are verbatim transcriptions of the frame law, each with one structural
  change. They include cumulative stage truncations for the breakdown. `diag_kernel verify`
  checks every replica that should be equivalent against the real kernel word for word, including
  the recursive words. It covers three parameter sets (fixture, hard compression, and parallel
  compression with makeup), all three link modes, and noise plus hostile inputs (signed zeros,
  subnormals, 1e29..3e30, both infinities, and two NaN payloads). It runs at `Scalar`, `Simd4` and
  `Simd8`, natively and in wasm. It passed for every variant, for every prototype mode natively, and
  for modes 0, 5, 7, 9, 13 and 15 in wasm. The mono prototypes were checked against today's mono
  body the same way, natively.
* **Browser.** `tools/compressor-diag-guest` is built with the production web recipe
  (`RUSTFLAGS=-C target-feature=+simd128`, release). It runs under Node v22.23.2 (V8 12.4) with
  `--no-liftoff`, so TurboFan code only. The Node hosts time `render` calls with
  `process.hrtime`. The console arm is the unmodified `wasm-console-guest` plus one mode export. It
  runs the same fixture from the native source table, and its 64-block digests equal the native
  ones.
* **Disassembly.** x86 is `objdump` of the release example. wasm is `wasm-objdump -d` of the
  guest. V8 is `node --print-wasm-code`. Innermost loops are classified by
  `tools/compressor-diag-guest/host/{x86_loops,wasm_loops,v8_loops}.py`.
* **Machine probe.** `crates/compressor/examples/diag_probe.rs` runs twelve independent `ymm`
  accumulators, one inline-asm instruction each, one or two instruction kinds per stream. It
  measures which pipes the compressor's operations share on Zen 3.

## 1. Where the isolate goes today

### Console isolate

| arm | us per block | cycles per lane-sample | run |
|---|---:|---:|---|
| native `Simd8`, compressor-only minus builtins-only | **39.3-39.7** | **8.88-8.96** | loadavg 3.6-8.1 |
| batch record (console step, same host) | 40.9 | 9.2 | from the brief |
| V8 wasm `simd128` (`Simd4` banks), fast mode | **81.7-82.9** | **18.5-18.7** | loadavg 3.1-5.1 |
| V8 wasm, slow mode (see the V8 caveat below) | 107-110 | 24.2-24.7 | same runs |
| native `Simd4` dispatch: **the compressor does not bank** | 315.6 | 71.3 | loadavg 7.9 |

The last row is a finding in its own right. Under D4, `CompressorFactory::bind_homogeneous_bank`
returns `Ok(None)` when `Backend::current().width()` differs from the requested width
(`crates/compressor/src/lib.rs:800`), and the parametric EQ does the same (`:2400`). So a native
"`Simd4`" console leg renders the compressor as 64 unbanked scalar instances. It is not a
four-lane native comparison for the compressor. The wasm floor rule's reading of the native `Simd4`
leg (`docs/rulings/effect-floor-accounting.md`, "The wasm floor rule") should carry that caveat.

### Kernel and its surroundings, one bank-block (128 frames x 2 channels)

| term | native `Simd8` | native `Simd4` | V8 `Simd4` | how |
|---|---:|---:|---:|---|
| kernel `process_block` (settled body) | 4.73-4.95 us = **68.4-71.5 c/ch-frame** | 4.55-4.67 us = 65.7-67.4 | 4.65-4.71 us = 67.1-68.1 | `real_kernel` |
| silent-path admission (quiet check, one early-exiting zero scan per plane) | ~0.02-0.04 us | ~0.02 | 0.13 with the scan | `real_render` - `real_kernel` - `real_finish` |
| boundary scan (`finish_channel`, both planes) | 0.05 us (0.7 c/ch-frame) | 0.06 | (in the row above) | `real_finish` |
| contract (`process_bank` guard, 8 empty automation drains, report) | 0.04-0.09 us | n/a (not bindable) | 0.18-0.25 (noisy) | `real_bank` - `real_render` |
| rack slot (stage dispatch, block construction) and plan effects | ~0 (within noise) | n/a | ~0.2 | console isolate / banks - `real_bank` |
| a whole silent block (earned fixed point) | 0.07 us | 0.04 | n/a | `real_render_silent` |

There is no extra transpose to charge. The compressor rides the builtins chain, and both rows run
8 chains per block (`bank_shape` [8, 32] against [8, 24], 8 transposes each). **The kernel is 97 %
of the native isolate and about 90 % of the V8 one.**

### Stage breakdown of the kernel (cumulative truncations of the frame law)

Each replica truncates the frame law after a stage and stores that stage's value, so each column
is the cost of the law up to and including the stage. The increments are the marginal cost of
appending a stage to the same frame. They are latency-shaped, not throughput-shaped, which is the
point of section 3. Units are cycles per channel-frame.

| through stage | native `Simd8` | native `Simd4` | V8 `Simd4` |
|---|---:|---:|---:|
| load and store only | 1.0 | 1.1 | 1.6 |
| + link (both channels, select form) | 1.6 | 1.7 | 3.6 |
| + level: floor, `fast_level_db`, clamp | 16.1 | 15.6 | 21.7 |
| + curve: `gain_delta_db`, clamp (the target) | 28.1 | 26.2 | 34.3 |
| + ballistic: select, `rms_follow`, `flush` (the recurrence) | 34.9 | 32.4 | 41.8 |
| + gain conversion: `fast_gain_from_db` | 57.8 | 56.1 | 57.5 |
| + mix and the four identity selects (the full law) | 68.0 | 69.7 | 69.2 |

The gain conversion adds 23 cycles for 19 lane-ops per channel-frame natively. It is the stage that
hangs off the recurrence, so its chain cannot start until the frame's ballistic finishes.

### Collapsed (mono) body and the ramping prefix

* **Collapsed body** (`process_block_mono`). One bank-block of one plane takes 2.455 us natively
  at `Simd8` (71.0 cycles per mono frame, 4 % more per channel than the dual body) and 3.478 us
  under V8 (100.5 cycles per mono frame, 50 % more per channel than dual). With one chain in
  flight instead of two, the latency problem of section 3 is worse.
* **Ramping prefix** (`frames_loop::<L, true>`). One lane's threshold receives a Point every block,
  which is a knob drag, so a 64-sample ramp is always in flight. The bank then costs 8.79 us
  natively against 4.91 settled (+79 %) and 9.59 us under V8 against 5.00 (+92 %). With all
  8 lanes ramping, the bank costs 13.2-13.4 us natively (+170 %), and 12.75 us under V8 (+155 %).
  Ramping attack instead costs 11.0 us (all lanes), and ramping makeup 12.4-12.7 us. No
  standing row delivers compressor automation, so no sealed record shows this.

## 2. The hot loop, disassembled

Idle (settled) body, one frame, which covers both channels. Per channel-frame figures are half.

| build | instructions per frame | vector ALU | broadcast loads | vector moves | scalar | branches |
|---|---:|---:|---:|---:|---:|---:|
| x86 `Simd8`, today | **241** | 152 (18 with a folded load) | 51 | 17 | 21 | in scalar |
| x86 `Simd4`, today | 237 | 152 | 45 | 19 | 21 | in scalar |
| x86 `Simd8`, mono body today (one channel) | 127 | 79 | 26 | 8 | 14 | in scalar |
| x86 `Simd8`, prototype mode 15 (pass 1 + pass 2) | **169** (99 + 70) | 129 | 17 | 17 | 6 | in scalar |
| wasm bytecode `Simd4`, today (LLVM output) | 512 ops | 159 SIMD arithmetic, 30 `v128.const`, 10 `v128` load/store | | | 35 scalar ALU, 252 `local.get/set` | 26 control |
| V8 TurboFan x64 for that loop | **321** | 186 | (constants: see below) | 63 | 56 | 16 |
| V8 TurboFan x64, prototype mode 15 | **217** (123 + 94) | 138 | | 54 | 21 | 4 |

**Against the inventory.** The x86 `Simd8` loop executes 76 vector ALU operations per
channel-frame. The inventory's in-loop count is 78.5 (the 81.5 total minus the 3-op boundary scan,
which runs outside the loop). LLVM already removes about 3.5 lane-ops of the inventory:

* the loop-invariant `neg` of the half knee;
* the `bypassed | dry_mix_zero` mask-or;
* the duplicate `input * gain` (`wet` and `gain_mix_step`'s `w` are one product);
* one of the curve's two `* (1/R - 1)` multiplies. It emits
  `select(over, d, v*v*itk) * (1/R - 1)`, which is bit-exact because both arms share the factor.

It adds one `vpcmpgtd`, because `select(m, 0, x)` lowers to a sign-splat and `vpandn`. The
historical accounting's gather is gone: scalar is 10.5 of 120.5 instructions per channel-frame
(8.7 %) against 141 of 315 (45 %) before #737.

**What the 44.5 non-arithmetic instructions per channel-frame are (x86 `Simd8`, today):**

* 25.5 `vbroadcastss`/`vbroadcastsd`. These re-splat the fast-dB polynomial coefficients and the
  kernel's clamp constants every frame, because sixteen `ymm` registers cannot hold them beside
  two channels' coefficient words.
* 8.5 vector moves: the input load, the output store, spill reloads, and **the recursive word's
  store and reload through the channel every frame**:
  `vmovaps 0x660(%rbx),%ymm3 ... vmovaps %ymm10,0x660(%rbx)`. That puts a store-to-load forward
  inside the recurrence.
* 10.5 scalar:
  * four slice bounds checks per frame (`cmp (%rsp),%rdi; ja; cmp $0x7,%r9; jbe; ...`);
  * the per-frame `match detector` (`test %rcx,%rcx; je ...; cmp $0x1,%ecx; je ...`);
  * five loop counters;
  * a spilled output-pointer reload.

**V8 specifics.** V8's code for today's settled loop is 321 instructions per frame. 72 of them
are scalar or branches:

* 56 scalar instructions: loop-index bookkeeping spilled to and reloaded from the frame, and
  wasm-memory address arithmetic;
* 16 branches: the four slice bounds checks as `cmp`/`jc`/`jna` pairs, the `match detector`, and
  the loop's stack-guard check;
* the recursive words, which live in linear memory and are loaded and stored every frame.

The chunked loop of mode 1 has 21 scalar instructions and 2 branches. Removing those is why the
loop hygiene is worth 12 % in the browser and nothing natively.

Two more V8 costs appear in shapes other than today's settled loop:

* **Constant materialisation.** V8 builds every `v128.const` splat inside the loop as
  `movq r64, imm64; vmovq; vpunpcklqdq`. It does this in the ramping loop (22 constants per
  frame, 66 instructions) and in a verbatim replica of the settled loop, but not in today's
  settled loop, which keeps them in stack slots.
* **`v128.bitselect`** lowers to three instructions (`vpand`/`vpandn`/`vpor`), 13 per frame. The
  wet and DualMono arms remove most of them.

## 3. Why it is slow: the frame law is latency-bound, not throughput-bound

* **Evidence 1: `Simd4` and `Simd8` cost the same per frame** (65.7 against 68.4 cycles per
  channel-frame). Zen 3 executes 256-bit float operations at full rate, so width is free and only
  the instruction stream matters. The stream is throughput-light, though. At 68 cycles per
  channel-frame the loop retires 1.1 vector ALU operations per cycle, against the 3.99 that a
  mixed `vmulps`/`vaddps` stream retires on this core.
* **Evidence 2: reordering alone saves a fifth.** The two-pass body executes exactly today's
  operations on exactly today's values; only the order across frames changes. It runs at 53.9
  against 69.1 cycles per channel-frame natively (prototype mode 9 against mode 1).
* **Evidence 3: the chain arithmetic.** Per channel, the law is a chain:
  * the detector (3 ops);
  * `frexp` (5);
  * a 10-step Horner (5 dependent mul/add pairs at 3 cycles each);
  * the curve (about 10);
  * the recurrence (about 12 cycles);
  * a second 8-step Horner in `fast_exp2`;
  * the mix.

  From load to store this is about 140 cycles. The loop issues each channel's chain as a
  contiguous run: LLVM keeps left-then-right source order, as the disassembly shows. Zen 3's
  floating-point scheduler fills with the waiting operations of about one frame. So two chains
  progress at a time, and the pipes idle.
* **The pipe floor on this host (probe).** Zen 3 has two pipe pairs:
  * **FP01** runs `vmulps`, `vblendvps` and `vpslld` at 2 per cycle;
  * **FP23** runs `vaddps`/`vsubps`, `vmaxps`/`vminps`, `vcmpps` and `vroundps` at 2 per cycle.

  Bitwise logic goes to any pipe at 4 per cycle. A mixed mul+add stream retires 3.99 per cycle,
  but add+max retires 1.998 and mul+blend 1.998, because each pair shares a pipe. The general path
  has 37 FP23 operations per channel-frame, so its pipe bound is 18.5 cycles per channel-frame, or
  2.31 cycles/lane-sample at `Simd8`. The settled all-wet DualMono arm has 33 FP23 operations
  (16.5 cycles, 2.06 cycles/lane-sample). The ruling's 3.7 ops/cycle (Zen 5) is within 10 % of
  this host's effective rate for this op mix.

## 4. Floor check: is 81.5 still the right inventory?

Recounted against `crates/compressor/src/kernel.rs` at `6ca203f8`: `link_frame` `:316`,
`curve_target` `:348`, `ballistic` `:359`, `gain_mix` `:372`, and `frames_loop` `:452`.

* **As a count of the general path as written, yes: 81.5.** Each row of the ruling's table
  matches the current source:
  * link 4.5;
  * detector floor 1;
  * `fast_level_db` 20;
  * clamp 2;
  * `gain_delta_db` 11;
  * clamp 2;
  * ballistic 8;
  * `fast_gain_from_db` 19;
  * gain, mix and identities 11;
  * boundary scan 3.

  The causal rewrite did not change the arithmetic of the law. It removed the delay ring and the
  per-lane gather, which the floor never counted.
* **Stale.** Its memory note, "4 vector accesses per channel-frame (main load, main store,
  detector store, delayed load), plus the per-lane detector gather", describes the pre-#737
  compressor. Today it is one load and one store per channel-frame, plus the boundary scan's
  re-read.
* **Over-counts, even for the general path (about 3.5 lane-ops, all bit-exact, and LLVM already
  removes them):**
  * the loop-invariant `neg` (1);
  * the duplicate `input * gain` (1);
  * the loop-invariant `bypassed | dry_mix_zero` (1);
  * the second `* (1/R - 1)` (1, removed by factoring out of the select).

  A class-A floor that counts required arithmetic would be about 78 for the general path.
* **Not the standing fixture's requirement.** All 64 tracks of the standing fixture are DualMono
  with `mix = 1` (`fixtures/session/v1/console-sixty-four-track-intended.json`; makeup 0-3 dB,
  threshold -31.5 to -6 dB). For such a bank:
  * the link needs only `abs` (1 lane-op, not 4.5);
  * the output law needs only `input * gain` (1, not 11).

  The required arithmetic is 1 + 1 + 20 + 2 + 9 + 2 + 8 + 19 + 1 + 3 = **66 lane-ops, or
  2.230 cycles/lane-sample**. This has the same shape as the builtins' prepared-identity elision
  (the ruling's appendix): once a prepared configuration makes arithmetic unnecessary *and the
  implementation elides it*, the floor of that row moves. Recommendation: recount once changes
  2-4 have landed, not before. Keep 81.5 as the inventory of the general (linked or parallel) path.
  This is a floor change, so it needs the owner ruling below.
* **Arithmetic in the frozen order that the parameter domains make inactive (class A, but not the
  implementer's call):**
  * `max(-100)` / `min(0)` on the reduction. With level clamped to 24 dB, threshold at least
    -80 and ratio at most 20, the delta is at least -98.8 dB. `-0.0` needs a knee arm with
    `v*v*itk` rounding to zero, and `v` is at least `ulp(W/2)`.
  * `fast_exp2`'s argument clamp `[-126, 127]`. The argument is
    `(smoothed + makeup) * log2(10)/20`, in [-20.6, 4].
  * `fast_log2`'s `max(MIN_POSITIVE)` after the kernel's own `max(1e-8)`.

  That is up to 5 lane-ops. A replica without the reduction clamp alone measured -1.3 cycles per
  channel-frame natively and -3.3 under V8.

## 5. The gap, named (the exit clause)

Today, at native `Simd8`, the isolate is 8.88-8.96 against 2.753: 30.8 % of the floor, a 3.2x
gap. Each part has a named mechanism:

1. **Latency-bound scheduling of one long frame chain.** Together with term 3, this is the
   remaining 4.0 cycles/lane-sample once terms 2 and 4 are taken out. Evidence is in section 3.
   The two-pass reorder alone recovers 1.9 cycles/lane-sample at console level (mode 1 to mode 9:
   9.08 to 7.18). The rest is the post-change residual named after this list.
2. **Arithmetic the prepared configuration does not need, about 1.9 cycles/lane-sample:** the mix
   and identity law on all-wet banks, and the link selects on DualMono banks. Measured as the
   wet and DualMono arms: modes 1 to 5 and 13 to 15.
3. **Non-arithmetic instructions, 44.5 of 120.5 per channel-frame:**
   * constant re-splats (register file);
   * the recursive word's memory round trip;
   * bounds checks;
   * the per-frame detector match.

   These cost little on their own natively (hygiene alone: +2 %, within noise). They are what
   fills the scheduler in term 1. Under V8 they are 12 % of the isolate: the loop's scalar
   bookkeeping, bounds checks and linear-memory state.
4. **Plan overhead, about 0.25 cycles/lane-sample:** admission, the boundary scan, the contract and
   the rack slot.

After all four changes (mode 15: 5.19 cycles/lane-sample at console level), the gap is 1.9x
against 2.753, or 2.3x against the recounted 2.230. Its named reasons:

* **Serial Horner chains** (10 and 8 dependent steps) and the recurrence-gated `exp2` chain in
  pass 2. Interleaving more frames in program order (`Pair<Pair<L>>` replicas, 4 to 8 chains)
  measured at most 5 % better natively and was unstable under V8: the same replica moved ±30 %
  between two builds, because it spills sixteen registers. That is the register file's limit.
  AVX-512's 32 registers would reopen it.
* **17 broadcasts, 17 moves and 6 scalar instructions per frame.** The same register-file limit.
* **Class B, flagged and not chased:** Estrin or pairwise polynomial evaluation would shorten the
  chains, but changes rounding.

## 6. Ranked changes (at most eight)

Each measurement is an in-process A/B on one runtime, under the lock. "Console" is the
compressor-only minus builtins-only isolate for 64 tracks. Native is `Simd8`. V8 is `Simd4` banks
in the fast mode. Kernel figures are cycles per channel-frame. The order is the order they should
land in: the V8 gain of change 3 depends on change 2 being in first.

| # | change | console native | console V8 | kernel native `Simd8` / `Simd4` | kernel V8 | class |
|---|---|---:|---:|---:|---:|---|
| 1 | Settled-body loop hygiene (`kernel.rs::process_block`, new `settled_main`): chunked iteration, recursive words in locals, `Detector::Main` matched once per block; Silent/Sidechain keep `frames_loop` | 39.3 -> 40.2 (+2 %, noise) | **82.9 -> 72.6 (-12 %)** | 68.4 -> 69.1 / 65.8 -> 65.6 | 67.9 -> 60.3 (-11 %) | A |
| 2 | All-wet output arm: `input * gain` when no lane is bypassed and every lane has `mix == 1` | **40.2 -> 34.1 (-15 %)** | **72.6 -> 64.2 (-12 %)** | 69.1 -> 59.6 / 65.6 -> 58.0 | 60.3 -> 52.4 | A (NaN words stay NaN; blocks with NaN are zeroed by `finish_channel` either way) |
| 3 | Two-pass settled body: targets of 32 frames into a 2x32-vector stack scratch, then recurrence and output | **34.1 -> 25.3 (-26 %)** | 64.2 -> 61.6 (-4 %) | 59.6 -> 42.6 / 58.0 -> 41.9 | 52.4 -> 50.3 | A |
| 4 | DualMono link arm (`abs` only), **in pass 1 only** | 25.3 -> 23.0 (-9 %) | 61.6 -> 59.3..61.5 (-0..-4 %) | 42.6 -> 38.3 / 41.9 -> 38.4 | 50.3 -> 48.4 | A |
| 5 | Collapsed body (`process_block_mono`): changes 1-4 together | console_mono row 71.8 -> 64.0-65.8 (-9..-11 %) | console_mono row 177.7 -> 148.2 (-17 %) | mono kernel 2.455 -> 1.513 us (-38 %); `Simd4` 2.434 -> 1.432 (-41 %) | 3.478 -> 1.690 us (-51 %) | A |
| 6 | Ramping prefix: make its per-frame cost proportional to the ramping lanes (measure, then a first slice) | knob drag on one lane: +79 % bank cost today | +92 % | not prototyped | not prototyped | A (slice) |
| 7 | Floor recount after 2-4: the settled DualMono all-wet inventory (66), plus the stale memory note (`docs/rulings/effect-floor-accounting.md`, `tools/bench/src/floor.rs`, `scripts/console-benchmark-record-lib.jq`) | accounting | accounting | | | ruling |
| 8 | Remove the provably inactive clamps (reduction clamp; `fast_exp2` argument clamp; `fast_log2`'s minimum after the 1e-8 floor) | about -2 % (replica, reduction clamp only) | about -5 % (replica) | 57.0 -> 55.7 (replica) | 61.9 -> 58.6 (replica) | A by proof; **owner ruling** |

All together (modes 0 -> 15), the native console isolate goes from 39.3 to 23.0 us (-41 %; 8.88 to
5.19 cycles/lane-sample), and the V8 isolate from 82.9 to 59.3-61.5 us (-26 to -28 %). The scalar
unbanked path, which is what every sidechain-connected compressor and the native `Simd4`
dispatch run, goes from 315.6 to 88.4 us for 64 tracks (-72 %) with the same rewrite. That
number is from the native `Simd4` dispatch console row under modes 0 -> 15, at loadavg 8.

For every change:

* **Risk to wasm, rule 3 of `check-web-audioworklet-callgraph.py`.** The roster requires exactly
  one arithmetic-carrying function matching `compressor6kernel13process_block.*4wide6f32x4`, and
  one matching `...18process_block_mono...`. The new bodies must be `#[inline(always)]` into
  `process_block`/`process_block_mono`. The prototypes used `#[inline(never)]` for disassembly,
  and that would move the hot arithmetic out of the rostered function. No scalar `f32` arithmetic
  is added to the `Simd4` instantiation. The `simd128` lowering needs no new operation: `abs`,
  `mul`, and the existing law.
* **Allocation-free render.** No heap. Change 3 adds a stack array of `2 x 32` lane vectors
  (2 KiB at `Simd8`, 1 KiB at `Simd4`). It is an intermediate value, not a copy of audio: the
  one-pass body keeps the same values in registers or spill slots.
* **Code size.** Changes 2 and 4 are block-level arms, which means up to four settled-body
  instantiations per width (DualMono or linked, times wet or general), plus the generic
  `frames_loop`. `scripts/check-web-boot-budget.mjs` and the artifact pin must be checked at the
  batch boundary.

### Not worth doing, and why

* **Fusing the boundary scan into the kernel.** It is 0.05 us per bank-block (1 %).
  `docs/rulings/d7-check-block-fusion.md` already measured it null.
* **The silent-path admission.** 0.02-0.07 us per bank-block, and it earns a whole-block skip on
  silence.
* **Caching `max_remaining`** (four 56-word scans per block). Under 0.5 %.
* **Skipping the per-lane automation drain when a block carries no spans.** The contract overhead
  is 0.04-0.09 us natively and 0.1-0.25 us under V8, within noise. At most 1-3 %.
* **Hoisting the fast-dB constants into per-block values (`K` replicas).** In the settled loop,
  V8 already keeps them in stack slots (today and after change 1). In a replica of today's shape
  it saved 8 cycles per channel-frame under V8. On top of interleaving it regressed V8 by 15-60 %,
  and natively it moves less than 3 %. The ramping loop, which does re-materialise them, belongs
  to change 6.
* **The DualMono arm outside the two-pass body.** It regresses V8 by 18 to 40 % (modes 3 and 7;
  the V8 loop spills the recurrence differently), while saving 7-11 % natively.
* **Deeper op-level interleaving** (two or four frames per iteration as `Pair`/`Pair<Pair>`).
  Natively it is within 5 % of the two-pass body. Under V8 it is unstable: the same replica
  measured 33.5 and 49.6 cycles per channel-frame in two builds.
* **Other chunk sizes for the two-pass body.** 16, 32, 64 and 128 frames are equal within 3 %.
* **Cross-bank interleaving.** It needs to cross the contract's `dyn` boundary, and was ruled null
  for the EQ at 1.10x (`docs/rulings/cross-bank-interleave.md`).
* **wasm `Simd8`.** Ruled null (`docs/rulings/wasm-simd8-null.md`). The compressor-only W8/W4
  ratio was 1.03.
* **Class B: Estrin or pairwise polynomial evaluation, or a lower-degree fast-dB refit.** They
  would shorten the dependency chains, but they change rounding or change the sound within a
  tolerance. They are flagged for the owner, with no projection.

## 7. For the owner

1. **Inactive clamps (change 8).** Removing a clamp that a value-range proof shows can never act
   is bit-identical. But the clamps are part of a frozen operation order, and two of them sit in
   the sealed fast-dB tier. Is a pinned range proof (a test that sweeps the parameter domains'
   corners) enough to remove them?
2. **Floor basis (change 7).** Should the standing row's floor count the arithmetic that the
   settled arm for its prepared configuration executes (66 lane-ops, 2.230), following the
   prepared-identity elision precedent, rather than the general-path inventory (81.5)?
3. **Class B, flagged only.** Polynomial evaluation order (Estrin or pairwise) and any fast-dB
   refit change rounding or the sound, so they need DSP evidence and a ruling. No projection is
   quoted for them.
4. **Browser evidence.** Every wasm figure here is Node v22.23.2 (V8 12.4, TurboFan only), not
   Chrome. V8 also showed a bimodal base: the current kernel's compressor-only row ran at 133 or at
   159 us per block in alternating 2,000-block rounds of one instance, with no mode switch
   (loadavg about 3.1). The prototypes with the two-pass body or the wet arm were stable. A Chrome
   confirmation run should precede any browser claim, and the bimodality is itself worth an issue
   if Chrome shows it.
5. **Copy rule.** Change 3's stack scratch holds intermediate targets, not audio. It is recorded
   here in case the owner wants that distinction ruled.

## Draft issues

`docs/handoffs/effects-2026-09-27/issues/`:

* `compressor-1-settled-loop-hygiene.md`
* `compressor-2-all-wet-output-arm.md`
* `compressor-3-two-pass-settled-body.md`
* `compressor-4-dual-mono-link-arm-in-pass-one.md`
* `compressor-5-collapsed-body-settled-rewrite.md`
* `compressor-6-ramping-prefix-cost.md`
