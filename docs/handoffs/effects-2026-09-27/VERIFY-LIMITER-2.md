# Adversarial verification: limiter dense-path diagnosis, round 2

Subject: `LIMITER-DIAGNOSIS-2.md`, `limiter-diagnosis-2-prototypes.patch`,
`limiter-diagnosis-2-raw-timings.txt` and drafts `issues/limiter-2-1..3-*.md`, read against round 1
(`LIMITER-DIAGNOSIS.md`, `VERIFY-LIMITER.md`) and the held specs #991 and #992. Base:
`codex/batch-plumbing-floor-2` at `220c5db5`. The limiter crate is identical there and on the
diagnosis base `49f696c7`, so every `file:line` in the drafts still holds. Nothing was pushed and no
issue was touched.

Evidence committed beside this file:

* `verify-limiter-2-raw-timings.txt`: every timed output, in run order, with its load average.
* `verify-limiter-2-harness.patch`: the variant generators, the build, identity and timing scripts
  and the isolated detector benchmark (`detbench`), as new files under
  `verify-limiter-2-harness/`. Its `README.txt` says how to reproduce each number below.
* `verify-limiter-2-prototype.patch`: the recommended merged slice (limiter-2-1 + 2-2 + 2-3,
  stationary dispatch only) as one diff of `crates/true-peak-limiter/src/lib.rs`.
  It is evidence only. It passes the crate's 50 tests in dev and in release, the #990 differential,
  the V8 digests, `check-lane-policy.sh` and `check-realtime-policy.sh`.

## Verdict

* **Holds.**
  * The headline dense costs, within 1-4 % (section 3).
  * The detector's 44-cycle chain, as arithmetic.
  * V8's detector: 200 instructions per channel-frame, 81 of them vector memory moves.
  * The gain loop's gap is bookkeeping, not arithmetic.
  * The codegen instability, and it is larger than the diagnosis says (F1).
  * Change 1's native saving, at a smaller size at `Simd4` (-3 %, not -5 %); under V8 it is neutral.
  * The pairwise tree loses under V8: +24-30 % on this batch through the shipped artifact, not the
    +50 % of round 1's multi-variant guest.
* **Does not hold.**
  * **The detector is not bound by its chains** (F5). A bit-identical software-pipelined detector
    halves the per-iteration chain and gains nothing. A class-B depth-4 tree is slower in isolation
    at `Simd8` and only 4-6 % faster in place. So "frozen Annex-2 summation order" is the wrong named
    reason for the detector's residual, and the owner should not be asked to rule on a reorder.
  * **Draft limiter-2-2 fails its own timing gate on its own evidence** (F2). Alone it makes the
    usual stereo rows 1-2 % slower and the ramping row 5-9 % slower than slice 1.
  * **"Do not restrict the change to the stationary dispatch" is a scaffold artefact** (F3). Built
    clean, the stationary-only form keeps every stationary gain and removes the ramping regression.
  * **Change 3 is no longer "ranked on its prize"** (F4). The no-copy form is safe Rust, branch-free,
    and the largest lever on all three targets. Measured against the pristine binary, the merged
    stationary-only slice is -9 to -10 % (`Simd8`) and -8 % (`Simd4`) on native linked rows, and
    -10.2 % on V8's linked isolate.
* **Class B: none qualifies.** No summation reorder wins on all three targets. Tree shapes lose
    16-30 % under V8, because V8's detector is bound by coefficient reloads and spills, which no
    summation order reduces (F6). The BS.1770 tolerance argument is in section 4.3 for the record.

## 1. Method

* **Host and tools.** AMD EPYC 7313P (Zen 3), cpu 31 (its SMT sibling is cpu 15). rustc 1.97.1,
  fat LTO, `+avx2,+fma`. Node 22.23.2 (V8 12.4). No PMU access.
* **Timing.**
  * Every timed command held the shared lock and was pinned with `taskset -c 31`, each hold under
    five minutes.
  * The host was shared with builds from other agents, at load averages of 8-31, and cpu 15 was
    busy at times. Interference only adds time.
  * Natively: separate binaries, forward-then-reverse over 3-4 passes of 4 rounds of 800 blocks;
    the statistic is the minimum over all rounds.
  * V8: three separate Node processes of 10 rounds each (the diagnosis's `web.mjs time`); the
    statistic is the mean of the per-process median isolates. The per-process minima agree within
    0.5 % and are in the raw file.
  * The runs quoted below were taken at load 2-13 (`quiet.sh` waited for it). Runs at load 17-31
    are kept in the raw file, marked, and not quoted, except where a table says so.
* **Clean single-change builds.** Every arm quoted here was built from the pristine tree plus
  only the change under test, by `variants.py`/`variants2.py`. None was built from the diagnosis's
  scaffolded tree (F1 says why). The pristine binary built twice is byte-identical
  (`2492f03d6301`).
* **Rigs.** The diagnosis's out-of-tree kernel rig (64 tracks, +3 dBFS noise, `HotLinked`,
  `HotDualMono`, `QuietLinked`, `RampLinked`). The shipped `host_web.wasm` under Node on the
  diagnosis's five derived sessions. A new isolated detector benchmark, `detbench`: one channel's
  32-frame chunks, L1-resident, shapes interleaved per round, with a port probe of the detector's
  operand mix.
* **Identity** (every class-A arm):
  * the #990 differential at W8 and W4 over 100-150 seeds and at W1 over 60-100 seeds;
  * the V8 output digests of all five sessions over 300 blocks;
  * the crate's 50 tests: in dev and in release for the merged prototype, and in release for the
    seed alone.

  Every class-A arm read identical everywhere. The two class-B arms differ, as they must.

## 2. Findings, most severe first

### F1 (high). The diagnosis's per-change numbers were measured in a scaffold that moves the kernel by up to 26 %

The diagnosis timed each prototype as a `LIMDIAG2` switch inside one tree carrying all twenty
prototypes as dead code. It checked that the scaffolded base compiled to byte-identical
`process_block` code, and then compared scaffolded arms with each other.

**The same edit measures differently in and out of the scaffold.** The seed edit (`P_SEED`), built
inside the scaffolded tree (with this verification's prototypes also present as dead code), against
the clean pristine binary, minimum over three passes (`raw-k1`):

| row | seed in the scaffold | seed built clean (`raw-k7`) |
|---|---:|---:|
| `Simd8` `HotLinked` | **+5 %** | -4 % |
| `Simd8` `RampLinked` | **+24 %** | -3 % |
| `Simd4` `HotLinked` | **+7 %** | -4 % |
| `Simd4` `RampLinked` | **+19 %** | -3 % |

**Failure scenario.** F3 below is the concrete case. The drafts forbid the stationary-only scope
on the strength of the scaffolded `P_STATONLY` measurement. An implementer who obeys ships limiter-2-2
in both dispatches, with a 5-9 % ramping regression that the clean stationary-only build does not
have. More generally, every per-change Δ in the diagnosis is provisional until reproduced clean.
This verification reproduced the ones that matter (section 3).

**Fix** (in every draft's gate 7): the change arm is built from the slice's own commit and the base
arm from its parent, with no diagnostic switch, prototype or dead code in either.

### F2 (high). Draft limiter-2-2 alone fails its own gate 7, by its own evidence

Gate 7 of limiter-2-2 requires `HotLinked`, `HotDualMono` and `QuietLinked` to be at most the
slice-1 tree, and `RampLinked` at most +0 %. Clean builds, against clean slice 1 (seed), minimum
over rounds (`raw-k5`, `raw-k7`, load 9-12):

| row | `Simd8` | `Simd4` |
|---|---:|---:|
| `HotDualMono` | -3 % | -4 to -5 % |
| `HotLinked` | **+1 %** | **+1 to +2 %** |
| `QuietLinked` | **+1 to +2 %** | **+1 to +2 %** |
| `RampLinked` | **+8 to +9 %** | **+5 to +6 %** |

The diagnosis's own increment table predicts this: "linked +0.5 to +1.6 %". Under V8 the slice alone
is -3 % against slice 1.

**Failure scenario.** The implementer builds limiter-2-2 as briefed and fails gate 7 on the usual
stereo row, `HotLinked`. The attempt budget is then spent on a slice whose standalone value is the
dual body only (`dual_mono`, or asymmetric stereo).

**Fix.** Limiter-2-2 is a precondition, not a product outcome, for the linked pair. Deliver it
together with limiter-2-3, in the stationary dispatch only (F3), with one timing gate on the
combined build. The drafts' amendments do this.

### F3 (high). Restricting 2 + 3 to the stationary dispatch removes the ramping regression

Built clean with the new walk only under `DISPATCH_STATIONARY`, and today's fused loop token for
token under `DISPATCH_RAMPING` (`statonly`), the combined slice keeps every stationary gain.
`RampLinked` returns to the slice-1 level. Minimum over rounds, cycles per lane-sample (`raw-k7`,
load 11-12). A second run, `raw-k8` at load 8-13, agrees to within 1-3 points:

| row | pristine | slice 1 (seed) | 1 + 2 | 1 + 2 + 3, all dispatches | **1 + 2 + 3, stationary only** |
|---|---:|---:|---:|---:|---:|
| `Simd8` dual | 9.89 | 9.65 | 9.32 | 9.49 | **9.10** |
| `Simd8` linked | 8.65 | 8.30 | 8.37 | 8.01 | **7.87** |
| `Simd8` quiet | 8.63 | 8.18 | 8.35 | 7.98 | **8.03** |
| `Simd8` ramping | 8.98 | 8.71 | 9.50 | 9.13 | **8.79** |
| `Simd4` dual | 18.21 | 17.63 | 16.87 | 16.92 | **16.98** |
| `Simd4` linked | 15.38 | 14.79 | 15.05 | 14.17 | **14.20** |
| `Simd4` quiet | 15.37 | 14.86 | 15.03 | 14.18 | **14.25** |
| `Simd4` ramping | 16.04 | 15.59 | 16.44 | 15.89 | **15.64** |

Across the two runs, against slice 1, the stationary-only combination is -2 to -7 % on every
stationary row and -2 to +1 % on `RampLinked`, whose source is unchanged. Against pristine it is
faster on every row in both runs.

Under V8 (three processes, load 4-6, `raw-w6`; the 1 + 2 column is `raw-w3`, load 8-12), change
against pristine:

| isolate | slice 1 | 1 + 2 | 1 + 2 + 3, all dispatches | **1 + 2 + 3, stationary only** |
|---|---:|---:|---:|---:|
| `lim - bi` | -0.9 % | -4.0 % | -9.4 % | **-10.2 %** |
| `limdm - bi` | -0.8 % | -3.5 % | -3.1 % | **-3.2 %** |
| `console - nolim` | -0.3 % | -4.5 % | -8.8 % | **-9.8 %** |

The diagnosis's `P_STATONLY` (+20-25 % ramping, and the stationary gain lost) was measured in the
scaffold (F1). Its "What the implementer will hit" item, "do not restrict the change to the
stationary dispatch", must be deleted.

### F4 (high, favourable). Change 3's no-copy form is safe Rust, branch-free, and the largest lever on all three targets

`linked_steady_streams` in `verify-limiter-2-prototype.patch` implements limiter-2-3's contract as
written:

* two `split_at_mut` per ring, at `max(e, c)` and at `max(x, c)`;
* eight zipped `chunks_exact(_mut)` iterators;
* a prologue that starts frame 0, a body that finishes frame `j - 1` (reading the old word of
  cursor slot `j`) before starting frame `j`, and an epilogue.

It has no `unsafe` and no copy. `check-lane-policy.sh` and `check-realtime-policy.sh` pass.

* **Codegen.** The stationary body loop is **23 instructions at `Simd8` and at `Simd4`**, with one
  backward branch and no panic edge. The ramping instantiation, in the all-dispatch build, is 34
  and 36. The checked steady loop it replaces is 64 instructions (`Simd8`) with 9 branches.
* **Under V8** the stream loop is 85 instructions per two frames, with 3 branches; LLVM unrolled
  the wasm loop by two. The checked loop is 103 instructions per frame, with 10 branches.
* **Identity.** The #990 differential (W8, W4, W1) and the V8 digests read unchanged.
  * The generator draws lookaheads up to 10 ms (`Wb = R`), so both the stream path and the
    `steady > R - Wb` fallback are reachable.
  * They were not counted here. The merged slice's gate 2 counts them.
* **Measured.**
  * Native, against the pristine binary, all dispatches: linked -7 % (`Simd8`) and -7 to -8 %
    (`Simd4`); quiet -7 / -8 %. Stationary-only: linked -9 to -10 % and -8 %. The unchecked
    diagnostic prices all bounds checks at -5 to -9 %.
  * V8, against pristine: -9.4 % (`lim - bi`), -3.1 % (`limdm - bi`) and -8.8 %
    (`console - nolim`) in all dispatches; -10.2 %, -3.2 % and -9.8 % stationary-only. The
    unchecked diagnostic reads -8.2 %, -5.4 % and -8.1 %.

  The safe no-copy form takes more than the bounds-check prize on linked blocks, because it also
  removes the per-frame address arithmetic.
* **A further safe stream, measured and not recommended (`p2s`).**
  * Pass 2's delay line is read and then written at the same slot, frame by frame, and the segment
    walk guarantees `main_cursor + run ≤ B`. So
    `main_ring[main_base·W .. (main_base + run)·W].chunks_exact_mut(W)`, zipped with the frames,
    gives a check-free pass 2 in safe Rust.
  * It is bit-identical and passes the crate's suite.
  * On top of the stationary-only 1 + 2 + 3 it measured -1 to +1 % natively, and -0.1 %, +0.7 %
    and +0.2 % under V8. Pass 2's checks do not cost time.

### F5 (high). The detector's stated bound is refuted, and so is the class-B named reason

The diagnosis says the detector is bound by its four frozen 12-add chains (about 44 cycles), with
the out-of-order window holding about 1.3 frames, and that "the remaining gap is attributable to the
frozen summation order, which is class B". The chain arithmetic is right: `vmulps` (3), then twelve
dependent `vaddps` (36), then `abs` and a four-deep `max`, is about 44 cycles. The mechanism is not.

Three shapes test it directly:

* **A class-A software pipeline** (`skewr`): iteration `f` finishes frame `f`'s taps 6-11 and starts
  frame `f + 1`'s taps 0-5.
  * It is bit-identical: every accumulator takes the same products in the same order.
  * It halves the dependent chain each iteration carries, from 12 adds to 6.
  * Its codegen is as clean as the base's: 133 instructions per channel-frame, 4-5 spills.
* **A class-B depth-4 pairwise tree** (round 1's `V_TREE`, rebuilt clean): 12 adds become 4
  levels.
* **A class-B two-chain split** (`tree2`): two 6-tap chains merged by one add.

If the chains bound the detector, all three would approach the port bound. They do not:

| shape | chain per iteration | in place, `Simd8` linked | in place, `Simd4` linked | isolated `Simd8` / `Simd4` (cycles per channel-frame) | V8 `lim - bi` |
|---|---|---:|---:|---:|---:|
| pristine | 12 adds | 8.67 | 15.36 | 44.1-46.5 / 36.3 | 22.11 |
| seed (change 1) | 11 | -3 % | -3 % | 45.1-47.4 / 34.9 | -0.7 % |
| `skewr` (class A) | 6 | -1 % | -1 % | - | +1.1 % |
| `skewr` + seed | 5-6 | -2 % | -3 % | 49.0-50.0 / 35.7 | +1.1 % |
| tap-buffer `skew` + seed | 5-6, plus a load | +22 % | +22 % | - | - |
| pair + seed (class A, F7) | 11, two frames | -3 % | -2 % | **35.0-36.9 / 31.3** | -2.9 % |
| tree (class B) | 4 levels | -5 % | -4 % | 47.1-49.3 / 34.5 | **+25.5 %** |
| tree2 (class B) | 6 + 1 | -4 % | -2 % | - | **+17.8 %** |
| port probe (same op mix, 3-add chains) | - | - | - | 29.0 / 28.4 | - |

Sources:

* **In place:** `raw-k9` (load 4-6), the minimum over three passes, as a change against pristine.
* **Isolated:** `raw-det4` and `raw-det5`, the range over processes. Two placement outliers (F10)
  are excluded.
* **V8:** `raw-w5` (load 2-4), the mean of three processes. `skewr` alone is from `raw-w1`
  (load 12-14).

So:

* **Chain depth is not what binds the detector.**
  * Halving the chain (class A) or cutting it to 4 levels (class B) leaves the isolated `Simd8`
    detector at 47-50 cycles, against 44-46 for pristine and 29 for a probe of the same operand mix.
  * At `Simd4` the tree is 5 % faster in isolation, the same as the seed.
  * In place, the tree takes 4-5 % off the whole limiter. A latency bound would give about 15-20 %.
* **The only shape much faster in isolation is the one that loads each coefficient once for two
  frames** (pair: -21 % at `Simd8`, -14 % at `Simd4`). In place, LLVM folds each coefficient into
  both frames' `vmulps` as a memory operand, 96 per pair. The sharing is lost there and so is most of
  the gain. Forcing a per-iteration broadcast spilled the accumulators (+4-8 % against pristine).
* **The named reason should be:** "not identified; chain depth ruled out (a class-A half-depth
  pipeline and a class-B depth-4 tree are no faster in isolation); coefficient-load sharing is the
  one lever measured faster in isolation, and LLVM does not keep it in place". That is a class-A
  implementation gap by `docs/rulings/effect-floor-accounting.md`'s own test, not a class-B gap.

**Failure scenario.** The loop's exit report records "frozen Annex-2 summation order" as the
residual's reason, and flags a class-B reorder for owner ruling. The owner then rules on a change
that measures +16 to +30 % in the product runtime and -2 to -5 % natively. Future rounds skip the
detector as "algorithmic" while an implementation gap of about 20 % (isolated) stays unexplored.

### F6 (medium). No summation reorder wins on all three targets

The pairwise tree was re-measured on this batch through the shipped `host_web.wasm`, with the
variant compiled as a constant. It reads **+25.5 %** (`lim - bi`), +24.2 % (`limdm - bi`) and
+24.4 % (`console - nolim`) at load 2-4, and +27-30 % at load 12-14. Round 1 read +50 %, through a
guest carrying every variant behind a runtime switch.

The two-6-chain split, the shape the question suggests, reads +16-18 % at load 2-4 and +21-24 % at
load 12-14. Natively, both trees are 1-2 points better than the seed alone (F5 table).

The mechanism is V8's register allocation, not the tree as such:

| detector under V8 | instructions per channel-frame | vector memory moves |
|---|---:|---:|
| pristine | 200 | 81 |
| tree | 267 | 151 |
| pair + seed | 180.5 | 69 |

V8 does not fold loads, so every coefficient is a separate load. Its detector is bound by those 48
reloads and by spills. A reorder of the adds cannot reduce the 48 coefficient-tap products, and
every tree shape raises register pressure. So no class-B reorder can beat the class-A form under
V8, and none is in scope.

### F7 (medium). A class-A browser lever the diagnosis did not build: two frames per iteration

`P_DET2` is declared in the diagnosis's patch but never implemented. Round 2's two-frame arm
(`P_DETTM2`) read its taps from a buffer. The register-tap pair (`pair` + seed: 8 accumulators
interleaved tap-major, 13 taps in registers) is bit-identical, as the differential and the V8
digests show. Its results:

* **V8:** -2.9 % (`lim - bi`), -2.7 % (`limdm - bi`) and -3.5 % (`console - nolim`) against
  pristine at load 2-4. That is 1.6-2.3 % beyond the seed. Under V8 one coefficient load feeds both
  frames, which gives 69 memory moves per channel-frame against 81.
* **Native:** +0 to +3 % against the seed alone, at both widths, in two clean runs. LLVM folds each
  coefficient into both multiplies.
* **Stacked on the merged gain-loop slice:**
  * V8: -12.2 %, -6.0 % and -11.5 % against pristine, against -10.2 %, -3.2 % and -9.8 % without it;
  * native: 4-10 % slower than without it (`raw-k8`).

It fails a three-target no-regression gate, so it is not briefed. A wasm-only shape
(`#[cfg(target_family = "wasm")]`, the same arithmetic in a different loop) would be class A and
would pass. Whether a target-specific loop shape is acceptable is an owner question (section 7).

### F8 (medium). The drafts' gates miss things

1. **No clean-build rule** (F1). Added to all three.
2. **V8 has no ramping row.** The five sessions are stationary. The drafts' ramping clauses are
   native-only, and the diagnosis never says so.
   * Under the stationary-only scope the ramping body's source is unchanged, which bounds the risk.
   * A V8 ramping row needs parameter events through `miso_engine_web_v1_command_submit`. That is a
     harness follow-up, not part of these slices.
3. **Limiter-2-2's M3 is not a mutation that goes red.**
   * "Advance the release ramp in pass 1" is equivalent if the per-frame values are carried to
     pass 2.
   * Under the stationary-only scope the release ramp is resting, so M3 cannot go red at all.
   * Replaced: pass 2 reads the previous frame's target (red). Also recorded: dropping the dispatch
     guard is equivalent (green), which shows the guard is performance-only.
4. **Limiter-2-3's M3** ("drop the `steady <= R - Wb` leg") **panics** in safe Rust. The `E` or
   `X` view runs past its split point. It does not render wrong words. Record it as "red (panic)".
5. **Limiter-2-1's E1 does not see the seed.** E1 passes unchanged against the seedless form.
   E1b's signed-zero rows are the discriminating rows. Its inputs gain a NaN made inside a chain
   (`+inf` + `-inf`).
6. **Gate statistics are under-specified.** Minimum over which rounds, how many processes, what
   load. Made exact in each draft.

### F9 (low). Numbers that do not reproduce as stated

* **Change 1:**
  * `Simd4` reads -3 % on every row, not -5 %.
  * `RampLinked` reads -3 % to +0.4 % across four runs, not -3 %. A strict "≤ base" on that row is
    a coin flip (limiter-2-1 A3).
  * V8 is neutral (-0.2 to -1.9 %), within process noise.
* **Change 1 + 2:** reproduces within 1-3 points: dual -6/-8 %, linked -3/-2 %, ramping +6/+3 %
  (`Simd8`/`Simd4`, against pristine).
* **The pairwise tree under V8:** +24-30 %, not +50 %.
* **The pristine V8 isolates** read 22.2 / 24.3 / 22.5 here against the diagnosis's 23.2 / 25.4 /
  23.5, about 4 % lower on the same bytes.
* **Draft limiter-2-1 cites `tests::reference_block` at `:6534`.** That line is
  `reference_block_uniform`; `reference_block` is at `:6499`.
* **Confirmed exactly:**
  * the roster vector counts (910 → 878 with the seed, 926 with 1 + 2);
  * the x86 detector loops (133 → 129);
  * the gain-loop counts (linked steady pass 1 64, dual 111);
  * the V8 detector census (200 / 105 / 81).

### F10 (low). Per-process placement moves an isolated loop by up to 2x

* The same `detbench` binary measured the seed detector at 85 cycles per channel-frame in one
  process and at 45-47 in six others (`raw-det3` to `raw-det5`). The tap-buffer pipeline read 72 in
  one process and 49-50 in five. Distributions within each process were tight.
* The kernel rig showed whole processes 30-58 % slow: `raw-k1` pristine `HotLinked` and
  `RampLinked`, and `raw-k2` pair at `Simd4`.

The drafts already use three passes and the minimum, and that is what makes their gates robust to
this. Keep it: a gate on one process is not evidence.

## 3. The headline numbers

Pristine, cycles per lane-sample. Natively, the minimum over rounds of four clean runs (`raw-k5`,
`raw-k7`, `raw-k8`, `raw-k9`, load 4-13). Under V8, the mean of three processes in each of three
clean runs (`raw-w3`, `raw-w5`, `raw-w6`, load 2-12):

| | diagnosis | this verification |
|---|---:|---:|
| `Simd8` linked / dual | 8.69 / 10.02 | 8.65-8.67 / 9.89-10.05 |
| `Simd4` linked / dual | 15.55 / 18.43 | 15.27-15.38 / 18.18-18.25 |
| V8 `lim - bi` / `limdm - bi` / `console - nolim` | 23.17 / 25.43 / 23.5 | 22.11-22.16 / 24.23-24.34 / 22.14-22.45 |

The truncation breakdown was not re-derived. The truncated builds are in the harness (`nogain`,
`nodet`) and were not needed for any conclusion here.

## 4. The detector

### 4.1 Is 44 cycles the chain latency?

As arithmetic, yes: about 44 cycles from the tap to the peak, with frames independent. As the
binding constraint, no (F5). The model the diagnosis gives, a window of about 1.3 frames over
105-120 FP results per frame, predicts the tap-buffer shape's +21-27 % (it adds a load to each
chain) and V8's 55 cycles (about 165 FP-writing ops per frame). But it also predicts that the
half-depth pipeline and the depth-4 tree approach the port bound, and they do not. The isolated
numbers put the pristine detector at 1.5x a 29-cycle probe of the same operand mix. No shape but
coefficient sharing moves it.

### 4.2 Is a class-A detector shape exhausted?

* **Natively, in place: yes, to within about 3 %.** The shapes tried here are the pipeline, the
  pair, the pair with forced broadcasts and the tap-buffer pipeline. None beats the seed alone by
  more than noise. Against pristine, the pair with forced broadcasts loses 4-8 % and the tap-buffer
  pipeline 18-24 %. The isolated -21 % of the pair says a gap exists.
  Reaching it needs LLVM to keep coefficient loads shared in place. That was not achieved and is
  recorded as not investigated further.
* **Under V8: no.** The pair is about -2 % beyond the seed (F7). Hoisting all 48 coefficient
  broadcasts is impossible in V8's roughly 14 allocatable `xmm` registers. The pair is the hoist
  that fits: one load per two multiplies.

### 4.3 The BS.1770 tolerance argument for a summation reorder

No reorder qualifies (F6), so this is recorded for completeness.

* **Arithmetic bound.**
  * Let `u = 2^-24`. Any evaluation order of a 12-term `f32` sum of the same rounded products
    differs from the exact sum by at most `γ_11 · Σ|h_k x_k|`, with `γ_11 = 11u / (1 - 11u)`, about
    `6.6e-7`.
  * The largest phase L1 norm of the stored table is 2.0228 (round 1's recount).
  * So two orders differ by at most `1.33e-6 · max|x|` per phase output.
* **Against the standards.**
  * **EBU Tech 3341 §2.6.** It sets the true-peak meter's tolerance at **+0.2 / -0.4 dB** on its
    true-peak test signals 15-23. That total includes the 4x oversampler's under-read, which ITU-R
    BS.1770 describes in Appendix 1 to Annex 2.
  * **The reorder's error, when the estimate is within 20 dB of the window's largest tap:** at most
    `1.2e-4` dB.
  * **On the Tech 3341 signals themselves** (sines at 0.5-1.41 of full scale, estimate ≈ `max|x|`):
    about `1.2e-5` dB. That is more than three orders of magnitude inside the tolerance.
* **Against the crate's own gate.** E2 (`bs1770_annex2_conformance_is_unchanged`) asserts
  `|phase - f64 oracle| ≤ 2e-6` for noise in `[-1, 1]`. The bound above, plus product rounding
  (`u · 2.0228`, about `1.2e-7`), is at most `1.45e-6`. So E2 passes under any order.
* **Against the output.** The gain path quantises the window minimum to a `2^-14` grid, and the grid
  masks most such differences. A 1-ulp estimate change reaches the output only when `l / p` crosses
  a grid line, and then moves the gain by one box term, `2^-14 / Wb`. The V8 digests show it: the
  tree changed `limdm-hot` and left `lim-hot` unchanged.
* **What is still required.** `effect-floor-accounting.md` asks a class-B change for a derived
  tolerance and a listening qualification before a benchmark. The bound above is the first. The
  second would be moot at `1e-5` dB, but the ruling asks for it.

## 5. The drafts

| draft | saving real on all three targets? | class-A argument | amendments |
|---|---|---|---|
| limiter-2-1, seed | Native yes on the stationary rows (-2 to -5 %). `RampLinked` read -3 to +0.4 %. V8 is neutral (-0.2 to -1.9 %), within its ≤ +2 % gate. | Sound. The signed-zero induction holds, NaN-payload freedom holds (`abs`, then an ordered compare in the gain computer), and the whole suite passes. | Clean-build rule; restated values; exact gate statistic; E1b NaN-in-chain rows; a line fix |
| limiter-2-2, cut and pass split | **No, alone** (F2). Linked +1-2 %, ramping +5-9 % against slice 1. | Sound. The `+inf` preset, steady frames that cannot complete, and commuting passes are all verified by identity on every target. | Merge with 2-3; stationary dispatch only; M3 replaced; one combined gate |
| limiter-2-3, streams | Yes, merged (F4). | Sound. Stream facts 1-5 are what makes the safe split compile and hold; checked against the fallback cases. | Merged into 2-2; the safe form exists; M3 is a panic; the prize table replaced by measurements |

**Change 2's ramping regression** is not acceptable as drafted. It is avoidable by scoping to the
stationary dispatch (F3), which leaves the ramping body's source untouched. What remains on
`RampLinked` is codegen movement of an untouched path: -2 to +1 % in two clean runs. The amended
gate allows +2 % on that row only, and only while the ramping arm is token-identical to today's.

**Change 3's no-copy form is expressible in safe Rust** (F4). It needs neither `unsafe` nor a
lane-crate array view.

**Each draft is small and self-contained after amendment.**

* Limiter-2-1: one function, one test file.
* Merged 2 + 3: about 300 lines of new functions plus one loop body, from a working prototype.

Each now has:

* mutations that go red, with the equivalent ones recorded green;
* dev and release gates;
* a both-NaN comparison wherever an output or peak word may be NaN;
* a three-target no-regression timing gate through the shipped wasm artifact, with a clean-build
  rule.

## 6. Ranked fix order

1. **Limiter-2-1 (seed), as amended.** Native -2 to -5 % on every row; neutral in the browser.
2. **Limiter-2-2 + 2-3 merged, stationary dispatch only.** Against pristine:
   * native stationary rows -7 to -11 % (`Simd8`) and -7 to -8 % (`Simd4`), `RampLinked` -1 to
     -3 %;
   * V8 -10.2 %, -3.2 % and -9.8 %.

   This is the browser lever.
3. **(Owner option) a wasm-only paired-frame detector.** About -2 % more under V8. It regresses
   natively, so it goes in only as a target-specific shape (section 7).
4. **Not recommended:**
   * any class-B reorder (F6);
   * the pair detector on native targets;
   * forced coefficient broadcasts;
   * the tap-buffer pipeline;
   * #992 contract 1 (`l / max(p, l)`), which the diagnosis measured as a net loss;
   * #991, which should be re-timed only on top of 1 and 2.

## 7. For the owner

1. **Reject the diagnosis's owner item 1.** "Frozen Annex-2 summation order" is not the detector
   residual's reason (F5). Record: "not identified; chain depth ruled out by a class-A half-depth
   pipeline and a class-B depth-4 tree; coefficient-load sharing is the one lever faster in
   isolation (-21 %), and LLVM does not keep it in place". No class-B ruling is needed. No reorder
   wins in the browser (F6).
2. **Ramping.** No regression allowance is needed if the merged slice is stationary-only. The gate
   keeps a +2 % codegen allowance on the untouched ramping row. Rule whether that allowance is
   acceptable. The alternative is ≤ +0 %, which a clean build met in some runs and missed by 0.9 %
   in others, with no source change on that path.
3. **Target-specific loop shapes.** The paired detector is bit-identical, -2 % under V8 and +0 to
   +3 % natively. Rule whether a `cfg(target_family = "wasm")` loop shape of the same arithmetic is
   acceptable. It would be the first such split in this crate.
4. **Scaffolded measurements.** Future diagnoses should quote per-change numbers from clean builds,
   or state that they come from a scaffold. The scaffold moved this kernel by up to 26 % (F1).
5. **V8 ramping is unmeasured** by any gate. A harness follow-up would add a parameter-event row.
6. **Still open from round 1:** which host the floors refer to, and whether a linked pair's gain
   path counts once.

## 8. Reproduction

Apply `verify-limiter-2-harness.patch`. Its `README.txt` says how to lay out the scratch
directory: a worktree of `220c5db5` with the diagnosis's `limiter-diag-2-harness/` copied in, and
`lib.rs.orig`. Then:

```text
./clean.sh native NAME SPEC ; ./clean.sh web NAME SPEC     # SPEC e.g. seed, vh+bcf+seed+statonly
./diff.sh 8 0 150 96 base NAME ; node web.mjs digest web/c-base.wasm web/NAME.wasm
LD2_SHAPES=HotLinked,HotDualMono,QuietLinked,RampLinked ./quiet.sh 13 2400 ./abba.sh kbench 4 800 4 base NAME...
./quiet.sh 13 2400 ./webtime.sh 3 c-base NAME...           # then python3 websumm.py RAW c-base web
./test.sh release SPEC ; ./test.sh dev SPEC ; ./policy.sh NAME
cargo build --release (detbench) ; ./detrep.sh 3 40
```
