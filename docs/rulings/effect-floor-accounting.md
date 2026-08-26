# Per-effect class-A floors: the op inventories, the machine, and what the gap is made of

**Candidate.** Owner directive, 2026-08-26 (#184): *"the DSP must reach the theoretical bottom — so
the bottom becomes a recorded number per effect, and convergence is measured against it."* The
instrument asked for is a published op inventory per kernel — required arithmetic per lane-sample
from the frozen spec, fast-dB polynomial ops included because they *are* the spec — divided by
measured machine throughput, derived in a ruling doc in the #163 style, and carried into the
standing records as `cycles_per_lane_sample` and `percent_of_floor` columns.

**Status.** Adopted for all four kernels of the standing strip. Two of the directive's own premises
did not survive the derivation and are ruled against below, with the evidence:

* the standing figure *"compressor ~13.2 derived cycles/lane-sample"* does not divide by lane
  width. The compressor's inventory is 94 operations per lane-sample and 94 ÷ 13.2 is 7.1, which is
  a **core dispatch rate**, not the rate a 256-bit machine retires lane-work at. Divided by the
  eight lanes a `Simd8` bank renders per instruction, the floor is **3.18** cycles/lane-sample and
  the compressor stands at **19.2 %** of it, not 88 % (boundary 1);
* *"the identity-section content is workload-dependent — floors are stated per enabled-section
  count"* is not true of this tree. A disabled builtins filter is designed as the arithmetic
  identity rather than branched around, the section count is fixed at two per channel by the
  prepared type, and the enabled count moves no instruction at all (boundary 5).

Both are recorded here rather than quietly worked around, because a floor that agrees with an
expectation it cannot reproduce is not a measurement.

---

## The unit, and the criterion

A **lane-sample** is one channel of one track for one sample: the unit
`docs/rulings/fast-db-tier-boundaries.md` already counts in ("the 16,384 lane-samples in a
sixty-four-track block" is 64 tracks x 128 frames x 2 channels). A **lane-op** is one primitive
arithmetic operation applied to one lane-sample. One AVX2 instruction on a `Simd8` bank performs
eight lane-ops.

```text
                        required lane-ops per lane-sample
floor(cycles/lane-sample) = ---------------------------------------
                        lane width  x  vector ops retired per cycle
```

Both denominators are measured on the pinned benchmark core, not quoted from a vendor table. Both
numerators are counted from the source that implements the frozen operation order, not estimated.

Three things are deliberately **outside** the floor, and each becomes a named gap term rather than
a fudge factor: loads and stores, the AoSoA transpose the rack performs around every bank, and the
graph's per-node dispatch. The floor is the arithmetic the spec requires. Everything else is what
the residual gap is made of, and naming it is the point of the exercise.

---

## The machine

Host: AMD Ryzen 7 9700X (Zen 5, family 26 model 68), 8 cores / 16 threads, `x86-64-v3` with the
workspace's pinned `+avx2,+fma`. The benchmark runner pins to the highest online CPU.

Measured with a throughput probe — twelve independent 256-bit accumulators so that every dependency
chain is longer than any operation's latency, 96 vector operations per loop iteration against two
instructions of loop overhead, counted by `perf stat -e cycles,instructions` on the pinned core
under the CPU-exclusive protocol. Twenty million iterations per variant, two runs, agreeing to
three decimal places:

| stream | ops/cycle | IPC | measured core clock |
|---|---:|---:|---:|
| `vaddps` alone | 1.992 | 2.038 | 5.53 GHz |
| `vmulps` alone | 1.992 | 2.038 | 5.50 GHz |
| `vmulps` then `vaddps`, alternating | **3.695** | 3.738 | 5.47 GHz |
| `vcmpps` then `vblendvps` | **3.763** | 3.807 | 5.49 GHz |
| the kernels' mix (3 mul : 2 add : 1 compare-select per six) | 2.455 | 2.503 | 5.48 GHz |

**A homogeneous stream retires two operations per cycle; a mixed one retires 3.7.** That is the
Zen 5 floating-point unit's shape showing through: adds and multiplies do not contend for the same
pipes, so a stream of one kind is pipe-limited at two and a stream of both is not. Every kernel
below is mixed, so:

```text
OPS_PER_CYCLE = 3.7      (tools/miso-engine-bench/src/floor.rs)
BANK_WIDTH    = 8        (Simd8, 256-bit)
lane-ops per cycle = 29.6
```

**Sensitivity, stated up front because it is the first thing to probe.** Every floor in this
document is inversely proportional to that 3.7. At the homogeneous 1.99 the floors are 1.86x
larger and the compressor stands at 36 % rather than 19 %; at a hypothetical 4.0 they are 7.5 %
smaller. No kernel here is homogeneous enough for the 1.99 figure and none can exceed 4, so the
range of honest floors is bounded, but the *number* is a measurement of this host and nothing else.

A bitwise-only variant (`vxorps`) is in the probe and is **not** in the table: LLVM folds a
repeated bitwise operation against a loop-invariant operand into nothing, and the variant measured
975 ops/cycle, which is a compiler result rather than a machine one. It is left in the probe, and
left out of the table, rather than deleted — a variant that cannot be measured this way is worth
recording as unmeasurable.

### The clock, and why the records carry it

Wall time is not cycles. The runner measures the pinned core's clock the same way this table does —
`perf stat -e cycles,task-clock` over the untimed warmup launch, `cycles / task-clock` — exports it
to the measured rounds, and counts those rounds too, refusing the run if either round's own ratio
differs from the exported figure by more than 3 %. On the run this document quotes, the three
launches measured 5 422 689 287, 5 428 034 794 and 5 427 243 146 Hz: a spread of 0.10 %.

The clock is therefore a hardware counter reading taken under exactly the preconditions the
records already claim (single-core affinity, load-average ceiling, SMT sibling quiet, binary-mtime
cooldown), not a nameplate frequency and not `/proc/cpuinfo`.

---

## Counting rules

Applied identically to all four inventories. Each is the `Lane` trait's own definition
(`crates/miso-engine-lane/src/lib.rs`), not an assumption:

| construct | lane-ops | why |
|---|---:|---|
| `add`, `sub`, `mul`, `div`, `sqrt`, `floor`, `abs`, `neg` | 1 | one instruction each |
| a comparison (`lt`/`le`/`gt`/`ge`/`eq`) | 1 | `vcmpps` |
| a mask operation (`mask_and`/`mask_or`/`mask_not`/`andnot`) | 1 | `vandps`/`vorps`/`vandnps` |
| `Lane::select(m, a, b)` | **1** | `bitselect`; masks here come only from comparisons, so `vblendvps` is a legal lowering |
| `Lane::max` / `Lane::min` | **2** | the D8 default `select(a > b, a, b)`: one compare, one select |
| `Lane::fma(a, b, c)` | **2** | `(a * b) + c`, deliberately unfused on every backend (#163 phase 2) |
| `flush(x)` | **3** | `x.andnot(x.abs().lt(EPS))` |
| a load or a store | 0 in the floor | counted separately; see "the unit, and the criterion" |

**`select` at 1 is a floor, not an observation.** On the emitted code it is three — LLVM builds
`(a & m) | (b & andnot m)` rather than `vblendvps`. That difference is a real cost and it is
quantified as gap term 1 for the compressor below, which is where it belongs: the floor states what
the spec requires, and the lowering the compiler chose is a gap.

---

## Compressor inventory

Source: `crates/miso-engine-compressor/src/kernel.rs`, the frozen nine-step operation order in the
`frames_loop` doc comment, at the idle body (the standing fixture declares no automation, so the
ramping body is not entered). Per lane-sample:

| step | expression | lane-ops |
|---|---|---:|
| 1-2 link, halved between the two channels | `abs` x2, `max`, two `mul` and an `add`, three `select` | 5 |
| 4 detector floor | `detected.max(level_floor)` | 2 |
| 4 `fast_level_db` | `max` 2, `frexp` 5, `t = m - 1` 1, six-term Horner 10, `e + t*q` 2, `* DB_PER_LOG2` 1 | 21 |
| 4 domain clamp | `.max(-160).min(24)` | 4 |
| 5 `gain_delta_db` | `sub`, `gt`, `neg`, `le`, four `mul`, `add`, two `select` | 11 |
| 5 reduction clamp | `.max(-100).min(0)` | 4 |
| 6 branching one-pole | `lt` + `select` 2, `rms_follow` (`sub` + unfused `fma`) 3, `flush` 3 | 8 |
| 7 `fast_gain_from_db` | `+ makeup` 1, `* LOG2_PER_DB` 1, clamp 4, `floor` 1, `sub` 1, five-term Horner 8, `1 + f*p` 2, `exp2_int` 6, final `mul` 1 | 25 |
| 8 gain, mix and the identities | `wet` 1, `gain_mix_step` 4, `eq` 1, `mask_and` 1, two `mask_or` 2, two `select` 2 | 11 |
| 4.4 boundary scan (`bank::finish_channel`) | `abs`, `lt`, `mask_and` | 3 |
| **total** | | **94** |

Memory, excluded from the floor and named here: 4 vector accesses per channel-frame (main load,
main store, detector store, delayed load), plus the per-lane detector gather — `D` is a per-lane
parameter, so the tap is a scalar load and a scalar store **per lane** followed by one vector load.

**The fast-dB polynomials are 46 of the 94**, just under half the kernel, exactly as the directive
says they should be counted: `fast_level_db` and `fast_gain_from_db` are the compressor's frozen
level law, sealed by gate F1, and a floor that excluded them would be a floor for a different
compressor.

### Corroboration from the emitted code

`objdump` of `miso_engine_compressor::kernel::process_block::<f32x8>` in the runner's own release
build finds two loops: 858 instructions (the ramping body, with `advance_ramps` inlined) and **630
instructions** for the idle body, which is one frame of both channels — 16 lane-samples. Its
`vmulps` count is 44, or 22 per channel-frame, against the 22 multiplies the inventory above
predicts; `vaddps` is 35, against 16.5 predicted plus the `exp2_int` and `frexp` biases. The
inventory and the compiler agree about what the kernel computes.

They disagree about how much it costs to compute it, and that disagreement is the whole gap:

| per channel-frame | instructions | against the 94-op floor |
|---|---:|---:|
| vector ALU | 118 | +24 — `bitselect` as `vandps`/`vandnps`/`vorps`, not `vblendvps` |
| `vbroadcastss` | 29 | +29 — constants re-splatted inside the loop; sixteen `ymm` registers is not enough to hold them |
| vector data movement (`vmovups`/`vmovss`/`vinsertps`/`vmovaps`) | 27 | +27 |
| scalar (address arithmetic, bounds checks, the detector gather's `cmovb`/`lea`/`cmp`) | 141 | +141 |
| **total** | **315** | **3.35x the floor's instruction count** |

---

## EQ inventory

Source: `crates/miso-engine-parametric-eq/src/lib.rs` and the shared
`crates/miso-engine-lane/src/kernels.rs` SVF. The standing fixture declares band 1 only on all 64
tracks; the other three of `EQ_SECTION_COUNT_V1 = 4` take the descriptor default `enabled = 0.0`
and design to `EqSvfWordsV1::IDENTITY`. Round 1's elision keeps `live.div_ceil(depth) * depth` = 2
sections at `SVF_CASCADE_DEPTH = 2`, which is the crate's own pinned case for this fixture. Per
lane-sample:

| item | lane-ops |
|---|---:|
| `svf_step`: `sub` 1, two unfused `fma` with their multiplies 6, two `add` 2, two state lines with `flush` 10 | 19 |
| output mix `m2.fma(v2, m1.fma(v1, m0.mul(x)))` | 5 |
| **one section** | **24** |
| two kept sections | 48 |
| 4.4 boundary scan | 3 |
| **total** | **51** |

The block-data elision gate (`block_admits_elision`) adds five integer comparisons per lane-sample
and is not counted as arithmetic: it is a guard on the optimisation, contends for the integer
pipes rather than the FP ones, and would disappear with the optimisation.

**The section count here is workload-dependent in a way the builtins' is not**, and the floor is
stated per kept section: `24 x kept + 3`, where `kept = ceil(live / 2) * 2` and `live` is the
number of enabled bands. At the standing fixture's one live band, `kept = 2` and the floor is 51;
un-elided at four sections it would be 99.

---

## Limiter inventory, re-derived post-round-1

Source: `crates/miso-engine-true-peak-limiter/src/lib.rs` at `98b5706`, which is the round-1 shape
(`git diff fce5da2 HEAD` over the crate is empty). The standing fixture takes the **uniform-cohort
vectorized path**: all 64 tracks declare `lookahead = 5.0 ms`, so every lane's `LaneShape` is
`window = 241`, every lane's phase advances in lockstep, and `lanes_uniform` admits the whole bank.
The ramps are `LinearRamp::fixed`, so the #144 stationary hoist removes the 14 lane-ops of ramp
advance. Per lane-sample:

| stage | lane-ops |
|---|---:|
| A. true-peak detector: 12-tap x 4-phase BS.1770 Annex-2 estimator, one `add(mul(..))` per tap | 96 |
| A. `abs` of the aligned tap, four `peak.max(phase.abs())` | 13 |
| B. stereo link, halved between the channels | 2 |
| C. coefficient ramps, hoisted to zero on this fixture | 0 |
| D. gain computer `select(peak > limit, limit/peak, 1)` — including one `div` | 3 |
| E. van Herk / Gil-Werman sliding minimum, three `min` amortised | 6 |
| F. quantise to the `BOX_GRID = 2^14` grid | 3 |
| G. box running sum, plus the second `div` by the window length | 3 |
| H. release ballistic: `sub`, unfused `fma`, `max`, `flush` | 9 |
| I. gain application and the bypass select | 3 |
| **total** | **138** |

**The detector is 109 of 138 — 79 % of the kernel.** The van Herk window that round 1 vectorized is
six. That is the load-bearing correction the re-derivation was asked for: round 1's win was not an
arithmetic win at all. It removed the ragged path's scalar issue — 180 lane-ops, 46 loads and 37
stores per lane-sample become 138, 10 and 8 — and left the detector, which is where the arithmetic
is, untouched.

Two `div` per lane-sample are the second cost centre and are counted as one op each, which
flatters the floor: `vdivps` is neither single-cycle nor well pipelined. The divide at stage G is
by a per-lane **constant** (`hot.window`, set once at block entry), so a splatted reciprocal would
remove it — at the cost of moving a rendered bit, which makes it class B and not this document's
to take.

---

## Builtins inventory, and the ruling on "per enabled-section count"

Source: `crates/miso-engine-builtins/src/`, `crates/miso-engine-lane/src/kernels/builtins.rs`, and
the graph's routing in `crates/miso-engine-graph/src/runtime.rs`. Per lane-sample:

| stage | lane-ops |
|---|---:|
| input sanitise and trim: `abs`, `lt`, `mask_not`, `select`, `add` (the counter), `andnot`, `mul` | 7 |
| HPF section (one 2nd-order TPT SVF, Butterworth `k = sqrt(2)`) | 24 |
| LPF section | 24 |
| output boundary scan: `abs`, `lt`, `mask_not`, `mask_or` | 4 |
| fader: `mul`, `andnot` (mute) | 2 |
| pan matrix: two `mul`, an `add`, a `select`, per channel | 4 |
| route `mix2x2`: `mul` + unfused `fma` per channel | 3 |
| output node's 64-input reduction, amortised per track | 1 |
| **total** | **69** |

Polarity inversion is **0**: it is folded into the trim coefficient at prepare time
(`trim_signed: if params.polarity_invert { -trim } else { trim }`).

**The ruling.** The directive asks for a floor "per enabled-section count" because the identity
section's content was believed to be workload-dependent. It is not:

* `SvfSection::design` returns `Self::IDENTITY` for a 0 Hz cutoff — `m0 = 1`, `m1 = m2 = 0`,
  `c1 = a2 = a3 = k = 0` — rather than a bypass. Its own doc says so: *"a disabled section is the
  arithmetic identity `(1, 0, 0)` with zero coefficients rather than a branch"*;
* `SvfCoef`, the type the kernel sees, carries no `enabled` field at all, so the flag cannot reach
  the render path. The frame body is an unconditional `for section in 0..2`;
* `enabled` is read in exactly one place, `InputBuiltins::tail()`, which is control plane.

So the section count is fixed at **two per channel** by the prepared type `[[SvfCoef<L>; 2]; 2]`,
and the *enabled* count changes only the values in the coefficient registers. The floor is stated
per **executed** section — `24 x sections`, with `sections = 2` unconditionally — and the two
rack-free benchmark rows share one floor for that reason. In the standing fixture every one of the
128 channel-lanes declares a non-zero HPF (30-70 Hz) and LPF (17 250-19 000 Hz), so enabled and
executed happen to coincide; the point is that they would coincide anyway.

This is also the evidence for the near-equality the decomposition rows already rely on:
`sixty_four_track_builtins_only` and `sixty_four_track_dispatch_only` measure 22.854 and 22.593 µs,
0.26 µs apart, because the same instructions run over the same lanes with different constants.

---

## The derived floors, and the standing %-of-floor table

Composed by `tools/miso-engine-bench/src/floor.rs` from the four inventories above, restated
independently by `scripts/console-benchmark-record-lib.jq`, and carried in every
`console_session` record of a run whose runner could measure the core clock.

| kernel | lane-ops | derived floor, cycles/lane-sample |
|---|---:|---:|
| builtins chain and routing | 69 | 2.331 |
| parametric EQ, two kept sections | 51 | 1.723 |
| compressor | 94 | 3.176 |
| true-peak limiter, uniform cohort | 138 | 4.662 |
| the whole intended strip | 352 | 11.892 |

<!-- STANDINGS -->

**How to read the two percentages.** A row's `percent_of_floor` compares the row's *whole* cost —
graph dispatch, transposes and all — against the arithmetic its strip requires, so it is a lower
bound. `isolated_percent_of_floor` is the per-effect number: the row's cost minus the named control
row's, against the difference of their floors. `sixty_four_track_compressor_only` minus
`sixty_four_track_builtins_only` is the compressor kernel and its bank chain and nothing else,
because the two rows are the same fixture with one rack emptied.

---

## Boundary 1 — the floor divides by lane width, and the standing 13.2 did not

The directive's figure is *"compressor ~15.0 measured vs ~13.2 derived cycles/lane-sample (≈88 % of
floor)"*. The measured half reproduces: this document measures the compressor isolate at 3.05
nanoseconds per lane-sample, which is 15.0 cycles at 4.92 GHz and 16.5 at the 5.42 GHz this host
actually clocked at, and which agrees to three digits with the 3.043 ns/lane-sample recorded for
the W8 bank in `.github/ISSUE_SPECS/013-compressor.md` on this same machine.

The derived half does not. 94 lane-ops ÷ 13.2 cycles is 7.1 operations per cycle. No 256-bit
machine retires 7.1 *vector* operations per cycle — this host retires 3.7 — and 7.1 is instead
within noise of a core's *dispatch* width. The 13.2 figure is the inventory divided by the rate a
core issues instructions at, with the eight lanes each instruction carries left out.

**The rule this establishes.** A floor for a banked kernel is `lane-ops / (width x ops-per-cycle)`.
Leaving the width out states the floor of the scalar instantiation and attributes it to the vector
one, which flatters the standing by the width: 8x here, and it is 8x that turns 19 % into a number
near 88 %.

The correction is not cosmetic. At 88 % of floor the compressor is finished and the loop should
exit; at 19 % there is a factor of five on the table, and the next section says where it is.

---

## Boundary 2 — the compressor's gap factors exactly, and it is not arithmetic

At the measured isolate of 16.540 cycles/lane-sample against a 3.176 floor, the gap is 5.21x. It
decomposes without a residual:

```text
instructions per frame iteration (idle body, objdump)     630
the floor's instruction count (94 lane-ops x 2 channels)  188
                                             ratio       3.35x

measured cycles per frame iteration (16.540 x 16)         264.6
                        implied IPC (630 / 264.6)          2.38
                  the probe's mixed-stream rate             3.7
                                             ratio        1.55x

                                     3.35 x 1.55  =       5.19x     (observed 5.21x)
```

Both halves are named:

**The instruction count.** Of 315 instructions per channel-frame, 94 are the floor. The other 221
are: 24 from `bitselect` lowering to three bitwise operations instead of `vblendvps`; 29
`vbroadcastss` re-splatting loop-invariant constants that sixteen `ymm` registers cannot all hold;
27 vector data movement; and **141 scalar instructions**, which is more than the arithmetic. The
scalar half is the per-lane detector gather — `D` is a per-lane parameter deliberately kept out of
the program key, so eight scalar loads, eight scalar stores and their bounds checks build a vector
that is then loaded back, twice per frame.

**The issue rate.** 2.38 against 3.7. The gather's scalar stores followed immediately by a vector
load of the same buffer is a store-to-load forwarding stall by construction; and the lookahead
rings are 481 and 486 slots deep, so a bank's ring working set is far larger than L1 and every ring
access is an L2 hit.

**None of this is arithmetic, and none of it is class B.** Every term above is an implementation
gap in the sense the loop's exit clause uses: a different way of writing the same computation would
remove it without moving a rendered bit. That is the finding, and it is the opposite of the
finding "the compressor is at 88 % of floor and there is nothing left".

---

## Boundary 3 — the EQ's gap is the same shape, and its kernel is not the subject

The EQ isolate measures 4.904 cycles/lane-sample against a 1.723 floor: 35.1 %, the best of the
three rack effects. The kernel itself is at its register-file ceiling — `SVF_CASCADE_DEPTH = 2` is
the tuned constant `docs/rulings/cross-bank-interleave.md` ruled on, and that ruling's reopening
condition is a machine with more than sixteen vector registers, which this is not.

What the EQ's 2.8x gap contains is therefore not the section body. It is the same list as the
compressor's minus the gather: the interleave pass's load and store per two sections, the block
elision gate's five integer comparisons per lane-sample, the AoSoA round trip the fused
`simd1:eq+compressor` chain shares, and the graph dispatch the row cannot subtract. **Naming that
as control plane is a claim this document can now put a number on**, which it could not before: the
kernel's floor is 1.723 of the 4.904, so 65 % of the EQ row's isolated cost is outside the section
arithmetic.

---

## Boundary 4 — the limiter's floor moved because round 1 changed its shape, not its arithmetic

The limiter isolate measures 13.865 cycles/lane-sample against a 4.662 floor: 33.6 %.

The pre-round-1 inventory cannot be quoted against it and is not. In the ragged path the same
computation costs 180 lane-ops, 46 loads and 37 stores per lane-sample, against 138, 10 and 8 in
the uniform-cohort path — round 1 removed 78 % of the memory operations and 23 % of the ops, and
the ops it removed were the scalar issue of a vector computation, not any arithmetic the spec
requires. **A floor derived against the pre-round-1 shape would have been a floor for a kernel
that no longer exists**, which is why the directive asked for the re-derivation.

Two residuals in the current shape are worth recording because they are cheap and class A:

* the uniform path still round-trips its van Herk result through a `[f32; MAXIMUM_WIDTH]` scratch —
  a store at the end of the forward pass and a load at the top of the frame — for a value already
  in a register;
* `state.phase.fill(..)` writes eight `u32` words per sample under `UNIFORM`, where every lane is
  provably writing the same value. That is sixteen scalar stores per frame across the two channels,
  more scalar stores than the vectorized window does vector stores.

Neither is proposed here. They are named so the next round's exit report can say whether it took
them.

---

## Boundary 5 — the idle row's floor is the builtins alone, and that is why it is 17.8 %

`sixty_four_track_idle` renders the full console strip with every source writing exact zeros. All
three rack effects hold a silence fixed point — `silent_fixed_point` in the EQ, the compressor and
the limiter alike — and on this row all three are earned, so all three kernels are skipped on every
timed block. Its class-A arithmetic floor is therefore the builtins chain, 2.331, and not the
strip's 11.892.

The row measures 13.115 cycles/lane-sample, 17.8 % of that floor and the worst standing in the
table. That is not a defect in the fixed points: it is the statement that **what the idle row costs
is almost entirely the AoSoA transposes and the graph dispatch that the fixed points cannot skip.**
The rack performs one planar/AoSoA round trip per realised chain per block whether the chain
computes anything or not — 24 chains per block on this fixture, of which the three rack chains
compute nothing at all on this row.

---

## The wasm floor rule

There is no cycle counter under wasm. The wasm legs therefore stay wall-only, and their floor is
stated as the native floor times a measured factor with two terms:

```text
wasm floor = native Simd8 floor  x  (native lane width / wasm lane width)  x  residual
           = native Simd8 floor  x  (8 / 4)                               x  residual
```

The width term is 2 and is structural: the guest is built `+simd128`, so a bank is four lanes, and
four lanes carry half the lane-ops per instruction. The residual is measured, per row, and is
**not** a constant. From `artifacts/compressor-round1/wasm-console-benchmark.accepted.jsonl`
(wasmtime 47.0.3, guest `+simd128`, round 1, p50 µs/block):

| row | native `Simd8` | wasm `simd128` | composite factor | residual (factor / 2) |
|---|---:|---:|---:|---:|
| compressor only | 74.001 | 128.885 | 1.742 | **0.871** |
| eq+compressor on simd1 | 95.251 | 172.478 | 1.811 | **0.905** |
| idle | 47.200 | 87.927 | 1.863 | **0.931** |
| dispatch only | 24.176 | 47.069 | 1.947 | **0.973** |
| builtins only | 24.246 | 47.330 | 1.952 | **0.976** |
| eq only | 45.346 | 91.795 | 2.024 | **1.012** |
| console — the intended strip | 135.417 | 297.636 | 2.198 | **1.099** |

**Every residual is inside 0.87-1.10, and three of the seven are below 1.** A residual below one
says wasmtime's Cranelift output is closer to its own four-lane floor than the native build is to
its eight-lane one — which is the same reading as the native `Simd4` leg in the same records, where
`sixty_four_track_console` costs 434.225 µs against wasm's 297.636 at the same lane count.

**The boundary.** A single residual must not be quoted across rows: 0.871 and 1.099 differ by 26 %,
and the rows they belong to differ in which kernel dominates. The rule is per row, from the run's
own paired legs, and a wasm floor quoted without naming the row and the record it took its residual
from is not a floor. Wasm numbers are wasmtime numbers, not browser numbers, on the standing terms:
Cranelift compiles ahead of time and does not tier, and every wasm record says
`browser_field_measurement: false`.

---

## The record columns

`console_session` records gain eleven additive columns. The group is all-or-nothing: a run whose
host had no usable performance counter exports no clock, every record omits the whole group, and
the record validates on exactly the shape every sealed record under `artifacts/` already has.
Nothing under `artifacts/` is re-derived or re-validated by this change.

| column | what it is |
|---|---|
| `lane_samples_per_block` | `tracks x quantum_frames x 2` |
| `core_clock_hz` | the measured pinned-core clock |
| `core_clock_source` | how it was measured |
| `cycles_per_block_p50` | `p50_ns_per_block x core_clock_hz / 1e9` |
| `cycles_per_lane_sample` | that, per lane-sample — the row's whole cost |
| `floor_cycles_per_lane_sample` | this document's derived floor for the row's strip |
| `percent_of_floor` | floor / measured, as a percentage. A lower bound: the row carries control plane |
| `floor_basis` | the inventories composing the floor, and this document's path |
| `floor_control_row` | the workload subtracted to isolate this row's subject, or `none` |
| `isolated_cycles_per_lane_sample` | this row's cost minus the control row's |
| `isolated_percent_of_floor` | the per-effect standing: the floors' difference over the costs' difference |

Every derived column is recomputed by `scripts/console-benchmark-record-lib.jq` from the columns it
was derived from, and the two isolate columns — which are subtractions between two records — by
`scripts/console-benchmark-validator.jq` across the whole run. `scripts/test-console-benchmark.sh`
mutates each of them and asserts the rejection. A column that is merely present proves nothing;
these are the ones that are checked against their own inputs.

`nine_track_baseline` is the one session row with null floors. It is rendered from
`fixtures/session/v1/parametric-eq-nine-track.toml`, which was never inventoried, and its
`floor_basis` says `not_derived` rather than borrowing the console fixture's numbers.

---

## The loop exit clause

The effect-optimisation loop's two-stalled-rounds circuit breaker stands unchanged. What this
document adds is the content of the exit report, and it is a requirement rather than a suggestion:

**On exit, the loop's report must state, for every effect on the standing strip:**

1. the effect's measured `isolated_cycles_per_lane_sample` and its derived floor, both from a
   record produced by `scripts/run-console-benchmark.sh`, cited by artifact directory and round;
2. the **residual floor gap** — the difference, and the percentage — and
3. a **named reason** for that gap. "Not investigated" is a permissible reason and "diminishing
   returns" is not: the reason names a mechanism, in the form boundaries 2 to 5 above use, or it
   names the fact that no mechanism was identified.

**Class-B gaps are flagged, never chased.** A gap attributable to the algorithm rather than the
implementation — a different detection algorithm, a reciprocal in place of a divide, a fused
multiply-add, a change to a window law — moves rendered bits by construction and is therefore
outside a class-A round's authority. The exit report **flags it for owner ruling and stops**. It
does not prototype it, does not measure it, and does not quote a projected win for it: a projection
for a change nobody has ruled on is a lobbying document, and the standing rule is that a class-B
change needs a derived tolerance and a listening qualification before it needs a benchmark.

The class-B candidates this document identified, flagged and not chased:

| effect | candidate | why it is class B |
|---|---|---|
| limiter | splat the reciprocal of `hot.window` once per block instead of dividing per sample | changes the rounding of every gain word |
| limiter | a cheaper true-peak detector than the 12-tap x 4-phase Annex-2 estimator (109 of 138 lane-ops) | a different detection algorithm |
| compressor | make the detector delay `D` part of the program key so the gather becomes a vector load | changes what a bank may contain, and so what a session renders |

The third is the loop's most valuable open question and the least obviously class B: the gather is
141 of 315 instructions per channel-frame, and the only reason it is scalar is that lookahead is
deliberately per lane. Whether uniform-cohort admission — the mechanism the limiter's round 1
already uses for exactly this reason — could apply to the compressor's detector tap is an owner
question about the effect contract, not an optimisation.

---

## What would justify reopening

* **A re-measured machine throughput.** Every floor here is `lane-ops / (8 x 3.7)`. A different
  measurement of the 3.7 — a better probe, a different mix, a microcode or toolchain change —
  rescales every floor and every percentage in this document by the same factor. Re-run the probe
  before disputing a percentage.
* **A change to a frozen operation order.** The inventories are counts of the frozen spec. A
  re-tuned fast-dB polynomial degree, a changed knee law, a different detector length, a change to
  `SVF_CASCADE_DEPTH` or to the elision's `kept` rule each move a numerator and require the
  affected inventory to be recounted here and the constant in
  `tools/miso-engine-bench/src/floor.rs` and `scripts/console-benchmark-record-lib.jq` to move with
  it.
* **A backend with more vector registers.** 29 `vbroadcastss` per channel-frame in the compressor
  is a register-pressure artefact of `x86-64-v3`'s sixteen `ymm`. AVX-512's thirty-two would change
  that term and only that term.
* **A compiler that forms `vblendvps` from `bitselect`.** Gap term 1 for every kernel with a select
  in it disappears; the floor does not move, because the floor already assumed it.
* **A wasm cycle counter.** The wasm floor rule exists because there is none. If one appears, the
  wasm legs stop being wall-only and the residual factors become measurements of one thing rather
  than of a ratio of two.

---

## Appendix — the throughput probe

Deliberately not in the tree: it is a host measurement, it needs `unsafe` intrinsics the workspace
denies outside audited boundaries, and it is run once per host rather than on every sweep. It is
reproduced in full so the 3.7 can be re-measured by anyone who disputes it.

```rust
// rustc -O -C target-feature=+avx2,+fma -C opt-level=3 throughput.rs -o throughput
// perf stat -e cycles,instructions -- taskset -c <cpu> ./throughput <variant> 20000000
#![allow(unsafe_code)]
use std::arch::x86_64::*;

const UNROLL: usize = 12;   // more independent chains than any operation's latency
const REPEAT: usize = 8;    // 12 x 8 = 96 vector operations per loop iteration

#[target_feature(enable = "avx2,fma")]
unsafe fn run(variant: &str, iters: u64) -> f32 {
    let one = unsafe { _mm256_set1_ps(1.000_000_1) };
    let two = unsafe { _mm256_set1_ps(0.999_999_9) };
    let mut a: [__m256; UNROLL] = core::array::from_fn(|i| unsafe {
        _mm256_set1_ps(1.0 + i as f32 * 0.01)
    });
    for _ in 0..iters {
        for _ in 0..REPEAT {
            for slot in a.iter_mut() {
                *slot = match variant {
                    "add"    => unsafe { _mm256_add_ps(*slot, one) },
                    "mul"    => unsafe { _mm256_mul_ps(*slot, two) },
                    // two operations per accumulator: 192 per loop iteration
                    "muladd" => unsafe { _mm256_add_ps(_mm256_mul_ps(*slot, two), one) },
                    "select" => unsafe {
                        let mask = _mm256_cmp_ps(*slot, one, _CMP_GT_OQ);
                        _mm256_blendv_ps(*slot, two, mask)
                    },
                    _ => unreachable!(),
                };
            }
        }
    }
    let mut out = [0.0_f32; 8];
    let mut total = unsafe { _mm256_setzero_ps() };
    for slot in a { total = unsafe { _mm256_add_ps(total, slot) }; }
    unsafe { _mm256_storeu_ps(out.as_mut_ptr(), total) };
    out.iter().sum()
}
```

`ops/cycle` is `iterations x operations-per-iteration / cycles`, and it agrees with `perf stat`'s
own IPC to within the two instructions of loop overhead, which is the check that the loop body is
what it is claimed to be.

---

## Links

* **Directive:** issue #184, owner 2026-08-26; sequenced after the round-1 merges (#181, #182,
  #186-#188) and before #183's W4/W8 capture.
* **Floor table (authority):** `tools/miso-engine-bench/src/floor.rs`.
* **Validator (independent restatement):** `scripts/console-benchmark-record-lib.jq`,
  `scripts/console-benchmark-validator.jq`.
* **Mutation coverage:** `scripts/test-console-benchmark.sh`.
* **Runner and its counter:** `scripts/run-console-benchmark.sh`,
  `scripts/check-bench-preconditions.sh`.
* **Inventoried sources:** `crates/miso-engine-compressor/src/kernel.rs`,
  `crates/miso-engine-parametric-eq/src/lib.rs`,
  `crates/miso-engine-true-peak-limiter/src/lib.rs`, `crates/miso-engine-builtins/src/`,
  `crates/miso-engine-lane/src/kernels.rs`, `crates/miso-engine-math/src/fast_db.rs`.
* **Standing qualification authority:** `artifacts/issue175/` for what the strip renders;
  this document accounts for what it costs.
* **Precedent for the unit:** `docs/rulings/fast-db-tier-boundaries.md`, and its rule that a
  per-sample kernel component may not be sized by an isolated throughput loop — which is why the
  standings here are taken from the console rows and not from a microbenchmark.
* **Precedent for the ceremony:** `docs/rulings/unfused-multiply-add-audit.md`, itself modelled on
  `docs/rulings/fast-db-tier-boundaries.md`.
