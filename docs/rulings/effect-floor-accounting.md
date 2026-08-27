# Per-effect class-A floors: the op inventories, the machine, and what the gap is made of

**Candidate.** Owner directive, 2026-08-26 (#184): *"the DSP must reach the theoretical bottom — so
the bottom becomes a recorded number per effect, and convergence is measured against it."* The
instrument asked for is a published op inventory per kernel — required arithmetic per lane-sample
from the frozen spec, fast-dB polynomial ops included because they *are* the spec — divided by
measured machine throughput, derived in a ruling doc in the #163 style, and carried into the
standing records as `cycles_per_lane_sample` and `percent_of_floor` columns.

**Status.** Adopted for all four kernels of the standing strip, and extended twice since: by the
appendix, which split the two rack-free rows' floors once a prepared-identity section stopped being
executed, and by the strip round's job 4, which added the *plumbing* inventory below them — the
route and the master reduction, which is what a lane-sample costs when no builtin is prepared at
all, and which is the floor of the whole table. Two of the directive's own premises did not survive
the derivation and are ruled against below, with the evidence:

* the standing figure *"compressor ~13.2 derived cycles/lane-sample"* does not divide by lane
  width. The compressor's inventory is 94 operations per lane-sample and 94 ÷ 13.2 is 7.1, which is
  a **core dispatch rate**, not the rate a 256-bit machine retires lane-work at. Divided by the
  eight lanes a `Simd8` bank renders per instruction, the floor is **3.18** cycles/lane-sample and
  the compressor stands at **19.0 %** of it, not 88 % (boundary 1);
* *"the identity-section content is workload-dependent — floors are stated per enabled-section
  count"* was not true of this tree when this ruling was written. A disabled builtins filter is
  designed as the arithmetic identity rather than branched around, the section count is fixed at
  two per channel by the prepared type, and the enabled count moved no instruction at all
  (boundary 5). **Amended by the appendix:** that last clause was a fact about the implementation,
  not a statement of the spec, and the strip round changed it. The directive's instinct was right
  and its reasoning was wrong — the two rack-free rows now have different floors, but because a
  prepared identity section is *elided*, not because the executed section count varies with the
  workload.

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
differs from the exported figure by more than 3 %. On the run this document quotes
(`artifacts/issue184/console-benchmark.core-clock.csv`), the three launches measured
5 455 548 845, 5 446 054 703 and 5 443 122 651 Hz: a spread of 0.23 %.

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
| input sanitise and trim: `abs`, `lt`, `mask_not`, `1.0 & bad`, `add` (the counter), `andnot`, `mul` | 7 |
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

Input time alignment (`builtins.*.delay_samples`, issue #210 phase 2) is **0** as well, and it is a
named gap term rather than a floor row: *input delay — 1 load + 1 store per lane-sample when
engaged, floor 0*. It falls under the standing rule that loads and stores are outside the floor
(see "Counting rules" and the gap-term list above). The kernel is `pdc_delay_block`, the same
two-segment ring swap PDC uses: it performs no arithmetic at all — it exchanges block words with
ring words — so there is no lane-op to count, and the residual it contributes is a memory-traffic
term of exactly the shape the transpose and dispatch terms already are. The overwhelming majority
of tracks declare `delay_samples = 0` and are not lowered to a delay node at all, so the term is
zero for them by construction rather than by rounding. `BUILTINS_LANE_OPS = 69.0`
(`tools/miso-engine-bench/src/floor.rs`) is therefore unchanged and this feature is **not** a
recount trigger.

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
per **executed** section — `24 x sections` — and at the time of writing `sections = 2`
unconditionally, so the two rack-free benchmark rows shared one floor. In the standing fixture every
one of the 128 channel-lanes declares a non-zero HPF (30-70 Hz) and LPF (17 250-19 000 Hz), so
enabled and executed coincide there.

This was also the evidence for the near-equality the decomposition rows relied on:
`sixty_four_track_builtins_only` and `sixty_four_track_dispatch_only` measured 22.833 and 21.962 µs,
0.87 µs (3.8 %) apart, because the same instructions ran over the same lanes with different
constants.

> **Amended — see "Appendix: the prepared-identity elision".** The ruling above stands as a ruling
> about *enabled counts*: the render path still has no `enabled` flag and still no per-lane branch.
> What changed is that a section whose prepared words are the **exact identity in every lane** is no
> longer executed at all: it is a `v |-> v + 0.0` map, a run of them is one `add(+0.0)`, and
> `input_chain_block_elided` emits that. `sections` is therefore the count of sections that are not
> the prepared identity, decided once per bank at construction. The standing fixture is unaffected —
> every one of its sections is real — but `sixty_four_track_dispatch_only`, whose whole content is
> the identity, drops from 69 lane-ops to 22 and the two rack-free rows stop sharing a floor.

### Identity inventory

The `dispatch_only` row, per lane-sample, once both sections are elided:

| stage | lane-ops |
|---|---:|
| input sanitise and trim: `abs`, `lt`, `mask_not`, `1.0 & bad`, `add` (the counter), `andnot`, `mul` | 7 |
| the run of two identity sections, collapsed: one `add` | 1 |
| output boundary scan: `abs`, `lt`, `mask_not`, `mask_or` | 4 |
| fader: `mul`, `andnot` (mute) | 2 |
| pan matrix: two `mul`, an `add`, a `select`, per channel | 4 |
| route `mix2x2`: `mul` + unfused `fma` per channel | 3 |
| output node's 64-input reduction, amortised per track | 1 |
| **total** | **22** |

> **The counter's spelling, not its cost.** The sanitise row used to read `select` where it now
> reads `1.0 & bad`. `sanitize_gain_block` and the three copies of its prologue accumulate the
> counter as `count + (1.0 & bad)` rather than `count + select(bad, 1.0, 0.0)`, which is the same
> bits on a canonical mask — `select(m, a, +0.0)` *is* `m & a` when `m` is all-ones or all-zeros —
> and the same **one** mask-and-value operation. The inventory is unchanged at 7 and no floor row
> moves; only the instruction the row names does. `crates/miso-engine-lane/tests/sanitise_counter.rs`
> holds both halves: the equivalence, and an independent scalar recount of what the D7 boundary
> should have counted.

> **The strip round's job 2 moves no row of this table, and that is the point.** Banking the fader
> and the pan matrix (issue #212) makes them slots of the cohort's chain instead of 128 individually
> dispatched per-track ops. A floor is an inventory of **lane-ops** -- the arithmetic one lane of one
> sample must pass through -- and banking changes none of it: the fader is the same `mul` and
> `andnot`, dispatched from the settled arm of the ramped stage rather than from the prepared-only
> one, and the pan matrix is the same `matrix2x2_block` it always was. What banking removes is
> dispatch, buffers and planar round-trips, none of which this table counts. So the derived floors
> below are unchanged, and the round's result shows up as measured rows moving *toward* them rather
> than as the floors moving. A round that claimed a floor reduction here would be claiming to have
> removed arithmetic it did not touch.

Sanitisation, the boundary scan, the fader and the pan matrix keep their full cost, and that is the
whole reason this is 22 and not something smaller. The D7 policy requires the input clear and the
output scan of *every* block regardless of what the chain between them does; a 0 dB fader is still a
multiply and a mask clear (`gain_mute_block` has no identity arm, deliberately — the `andnot` is
what makes a muted `-1.0` exactly `+0.0`); and a settled identity pan matrix evaluates both arms of
its per-lane select unconditionally (`matrix2x2_block`). Only the input sections have a
prepared-identity rewrite.

That last claim was, until the strip round's job 4, an argument rather than a measurement. It has a
row now: `sixty_four_track_gain_pan_only` makes the *same* strip edit as `dispatch_only` but for one
field — it keeps the fixture's declared per-channel fader trims and pan positions where
`dispatch_only` asks for 0 dB and hard identity. The two rows therefore execute the same
instructions over the same lanes with different constants, they are costed at **one** inventory in
`floor.rs` and in the jq restatement, and that shared inventory *is* the claim. A material gap
between the two measured rows would mean `gain_mute_block` or `matrix2x2_block` had acquired a
data-dependent path, and the shared basis string is what makes that show up as a contradiction
between two rows rather than as an unexplained microsecond.

---

## Plumbing inventory, and the overhead floor

`sixty_four_track_dispatch_only` has been read as "the overhead" since it was added, and it is not.
An identity strip still executes 22 lane-ops of arithmetic the frozen spec requires of every block.
The row *below* it — `sixty_four_track_plumbing_only`, added by the strip round's job 4 — is the one
that prepares nothing: `prepare_session_builtins` is never called for it, so the graph is built
through `GraphCompiler::compile`, every `TrackStage` lowers to an elided alias, and no bank chain is
bound at all (`[chains, slots] == [0, 0]`, and therefore no planar/AoSoA round-trip and no route
fold — the chain-shape gate pins all three). Per lane-sample:

| stage | lane-ops |
|---|---:|
| route `mix2x2`: `mul` + unfused `fma` per channel | 3 |
| output node's 64-input reduction, amortised per track | 1 |
| **total** | **4** |

Both lines are already lines of the builtins inventory and of the identity inventory, so the two
*inventories* subtract exactly:

    sixty_four_track_gain_pan_only − sixty_four_track_plumbing_only  =  22 − 4  =  18

and those 18 are precisely the sanitise (7), the collapsed run of identity sections (1), the output
boundary scan (4), the fader (2) and the pan matrix (4).

### Why these two rows are **not** a control pair

The obvious next step is to make `plumbing_only` the control row of `gain_pan_only` and publish that
18 as an isolate. It was tried, and the floor table itself refused it — which is the most useful
thing this row has done so far.

`gain_pan_only` binds eight bank chains, so issue #218's route fold fires on every one of its
sixty-four lanes: its route and its share of the master reduction are an epilogue on a tile the
chain has already transposed, and they cost almost nothing. `plumbing_only` binds **no chain at
all**, so there is no epilogue to fold into: it pays sixty-four individually dispatched route ops
and an unfolded reduction over sixty-four separate planar buffers. The two rows execute the same
*arithmetic* plumbing and completely different *plans* for it.

Subtracting the second from the first therefore removes the fold's saving as well as the plumbing's
four lane-ops, and the result lands **below** the 18-lane-op floor it is supposed to be measured
against: on `artifacts/strip4/` the difference is 0.327 cycles/lane-sample against a floor of 0.608,
an `isolated_percent_of_floor` of 186 % — the table stating that the quantity is not the one its name
claims. The subtraction is retired rather than tolerated: `floor_control_row`
is `none` on both rows, `floor.rs` and the jq restatement agree, and
`the_overhead_inventories_differ_by_the_scaffolding_and_neither_row_claims_an_isolate` is what stops
a later edit from quietly reinstating it.

What survives is more interesting than the isolate would have been. `plumbing_only`'s own
`percent_of_floor` is the *unfolded* plumbing measured against the four lane-ops plumbing requires,
and `artifacts/strip4/` measures it at **6.7 %** — the worst standing in this table by a factor of
nearly five. The next two are the identity rows at 31.9 % and 32.4 %, and the idle row, which
boundary 5 singles out as the strongest statement in the stream that a row's cost is dispatch rather
than arithmetic, is 35.4 % on that same host. Six microseconds a block of 64 dispatched route ops
and an unfolded 64-buffer reduction, against four lane-ops of required arithmetic: that gap is a
direct measurement of what job 3's fold removed from every banked row.
The pair that *would* subtract cleanly is one where both sides bank and both sides fold:
`sixty_four_track_builtins_only` against `gain_pan_only` differ by 69 − 22 = 47 lane-ops, which is
the two 24-op SVF sections less the single `add(+0.0)` the elided run composes to, over two rows
that realise the same twenty-four bank slots and the same eight round-trips. That control is not
declared here — it would move an existing row's `floor_control_row`, which is not this job's to do —
and it is written down so the next person reaching for an overhead isolate reaches for that one
rather than for the plumbing row. The plumbing row's job is to be the floor, not the control.

**The post-fold recheck, and why the number did not move.** Job 3 folded the route application and
the master accumulation into the cohort chain's own epilogue, which is where the route and the
reduction now live on every banked row. The inventory above is stated on the *post*-fold tree and is
unchanged from the pre-fold one, for the same reason job 2's banking moved no row of the builtins
table: `ArenaMembers::fold_plane` runs the same `mix2x2_block` over the same bind-folded 2×2 on a
slice of the same length, and then the same `sum_into_block` — with the first contributor storing
instead of accumulating, which is what keeps sixty-four contributors at sixty-three adds rather than
sixty-four. The route is deliberately *not* merged into the matrix slot above it (two 2×2s
multiplied out is a different rounding), so no operation was eliminated and none was added. What the
fold removed is 64 whole passes over buffers the chain had just scattered, 63 reduction passes and
64 dead fan-in-zero fills — dispatch, buffers and stores, none of which this table counts.

The plumbing row is the *floor of the whole table*, and `floor.rs` asserts that: no row in this
stream may be costed below it, because a row that renders sixty-four tracks into one master pays a
route matrix and its share of the reduction whatever else it does or does not prepare.

### The mono rows

`sixty_four_track_console_mono`, `sixty_four_track_console_mono_dual` and
`sixty_four_track_console_half_mono` render `fixtures/session/v1/console-sixty-four-track-mono.toml`,
which is the standing fixture with its source mapping and its upstream per-channel parameters
symmetrised. They carry the whole intended strip and are costed at the whole intended strip's
inventory — 352 lane-ops — because their fixture differs from the standing one in per-channel
*values* only, and a floor is an inventory of operations, not of operands.

**One question is deliberately left open**, and it is left open here rather than answered quietly in
the table. The mono collapse landed with mono-collapse M2, and `sixty_four_track_console_mono` now
takes it on every cohort of every block: a collapsed track computes one plane where the spec
describes two. So the row's measured cost has fallen against an inventory that has not, and its
%-of-floor now reads above what any stereo row can reach. Whether a collapsed row's floor *should*
halve is a ruling this document still does not make: the honest candidates are "the spec requires the arithmetic
of both channels and the collapse is an implementation that exploits their equality, so the floor
stands at 352 and the row's %-of-floor rises above what a stereo row can reach", and "a lane-sample
whose value is determined by another lane-sample is not independent arithmetic, so the upstream half
of the inventory halves". Both are defensible and they give different numbers for the same row. The
rows exist now so that the question is asked against measurements; the pinned equality in
`floor.rs`'s `the_mono_rows_carry_the_standing_strips_floor` is what makes answering it a deliberate
edit rather than a table drift. This joins the #193 max/min re-pricing as open floor-accounting debt;
today's pricing is unchanged by either.

---

## The derived floors, and the standing %-of-floor table

Composed by `tools/miso-engine-bench/src/floor.rs` from the inventories above, restated
independently by `scripts/console-benchmark-record-lib.jq`, and carried in every
`console_session` record of a run whose runner could measure the core clock.

| kernel | lane-ops | derived floor, cycles/lane-sample |
|---|---:|---:|
| route and master reduction (the plumbing floor) | 4 | 0.135 |
| builtins chain and routing | 69 | 2.331 |
| builtins chain, identity sections (`dispatch_only`, `gain_pan_only`) | 22 | 0.743 |
| parametric EQ, two kept sections | 51 | 1.723 |
| compressor | 94 | 3.176 |
| true-peak limiter, uniform cohort | 138 | 4.662 |
| the whole intended strip | 352 | 11.892 |

### The standing table

`artifacts/issue184/`, commit `a1ef5f1`, controlled, AMD Ryzen 7 9700X pinned to cpu 15, exported
core clock **5 455 548 845 Hz**. p50, minimum of the two measured rounds, as the round READMEs
report it.

| row | p50 µs/block | measured cycles/lane-sample | floor | % of floor | isolate | isolated % of floor |
|---|---:|---:|---:|---:|---:|---:|
| console — the intended strip | 123.685 | 41.185 | 11.892 | 28.9 % | **13.918** *(limiter)* | **33.5 %** |
| console, synthetic, 128 tracks | 246.258 | 41.000 | 11.892 | 29.0 % | — | — |
| eq+compressor on simd1 | 81.816 | 27.243 | 7.230 | 26.5 % | 19.593 *(eq+comp)* | 25.0 % |
| console legacy | 86.054 | 28.654 | 7.230 | 25.2 % | 21.051 *(eq+comp, split chains)* | 23.3 % |
| compressor only | 72.959 | 24.294 | 5.507 | 22.7 % | **16.691** *(compressor)* | **19.0 %** |
| eq only | 37.942 | 12.634 | 4.054 | 32.1 % | **4.984** *(eq)* | **34.6 %** |
| idle (silence) | 38.974 | 12.978 | 2.331 | **18.0 %** | — | — |
| builtins only | 22.833 | 7.603 | 2.331 | 30.7 % | — | — |
| dispatch only (identity) | 21.962 | 7.313 | 2.331 † | 31.9 % † | — | — |
| nine-track ragged strip | 24.978 | 59.144 | 21.141 | 35.7 % | — | — |
| nine-track eq fixture | 6.092 | 14.425 | *not derived* | — | — | — |

**The four per-effect standings, which is what the directive asked for:**

| effect | measured | derived floor | % of floor | isolated against |
|---|---:|---:|---:|---|
| parametric EQ | 4.984 | 1.723 | **34.6 %** | `sixty_four_track_builtins_only` |
| true-peak limiter | 13.918 | 4.662 | **33.5 %** | `sixty_four_track_eq_comp_simd1` |
| compressor | 16.691 | 3.176 | **19.0 %** | `sixty_four_track_builtins_only` |
| builtins and routing (row, not isolated) | 7.603 | 2.331 | **30.7 %** | — |

† Superseded by the prepared-identity elision. Both the measurement and the floor in that row are
the #184 pair and are left as measured: the floor column was 69 lane-ops and the row's kernels
executed 69 lane-ops' worth of work. Under the elision the same row's derived floor is
`22 / (8 x 3.7) = 0.743` cycles/lane-sample, and its measurement is a different measurement. The two
are not mixed here; the strip round's sealed records carry the new pair.

The compressor's 16.691 is 3.059 nanoseconds per lane-sample, which is 15.0 cycles at 4.92 GHz —
the directive's measured figure, reproduced — and agrees to three digits with the 3.043
ns/lane-sample recorded for the W8 bank in `.github/ISSUE_SPECS/013-compressor.md` on this same
machine.

**How to read the two percentages.** A row's `percent_of_floor` compares the row's *whole* cost —
graph dispatch, transposes and all — against the arithmetic its strip requires, so it is a lower
bound. `isolated_percent_of_floor` is the per-effect number: the row's cost minus the named control
row's, against the difference of their floors. `sixty_four_track_compressor_only` minus
`sixty_four_track_builtins_only` is the compressor kernel and its bank chain and nothing else,
because the two rows are the same fixture with one rack emptied.

---

## Boundary 1 — the floor divides by lane width, and the standing 13.2 did not

The directive's figure is *"compressor ~15.0 measured vs ~13.2 derived cycles/lane-sample (≈88 % of
floor)"*. The measured half reproduces: this document measures the compressor isolate at 3.059
nanoseconds per lane-sample, which is 15.0 cycles at 4.92 GHz and 16.691 at the 5.456 GHz this host
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

At the measured isolate of 16.691 cycles/lane-sample against a 3.176 floor, the gap is 5.255x. It
decomposes without a residual:

```text
instructions per frame iteration (idle body, objdump)     630
the floor's instruction count (94 lane-ops x 2 channels)  188
                                             ratio       3.351x

measured cycles per frame iteration (16.691 x 16)         267.1
                        implied IPC (630 / 267.1)         2.359
                  the probe's mixed-stream rate             3.7
                                             ratio        1.568x

                            3.351 x 1.568  =             5.255x     (observed 5.255x)
```

Both halves are named:

**The instruction count.** Of 315 instructions per channel-frame, 94 are the floor. The other 221
are: 24 from `bitselect` lowering to three bitwise operations instead of `vblendvps`; 29
`vbroadcastss` re-splatting loop-invariant constants that sixteen `ymm` registers cannot all hold;
27 vector data movement; and **141 scalar instructions**, which is more than the arithmetic. The
scalar half is the per-lane detector gather — `D` is a per-lane parameter deliberately kept out of
the program key, so eight scalar loads, eight scalar stores and their bounds checks build a vector
that is then loaded back, twice per frame.

**The issue rate.** 2.359 against 3.7. The gather's scalar stores followed immediately by a vector
load of the same buffer is a store-to-load forwarding stall by construction; and the lookahead
rings are 481 and 486 slots deep, so a bank's ring working set is far larger than L1 and every ring
access is an L2 hit.

**None of this is arithmetic, and none of it is class B.** Every term above is an implementation
gap in the sense the loop's exit clause uses: a different way of writing the same computation would
remove it without moving a rendered bit. That is the finding, and it is the opposite of the
finding "the compressor is at 88 % of floor and there is nothing left".

---

## Boundary 3 — the EQ's gap is the same shape, and its kernel is not the subject

The EQ isolate measures 4.984 cycles/lane-sample against a 1.723 floor: 34.6 %, the best of the
three rack effects. The kernel itself is at its register-file ceiling — `SVF_CASCADE_DEPTH = 2` is
the tuned constant `docs/rulings/cross-bank-interleave.md` ruled on, and that ruling's reopening
condition is a machine with more than sixteen vector registers, which this is not.

What the EQ's 2.89x gap contains is therefore not the section body. It is the same list as the
compressor's minus the gather: the interleave pass's load and store per two sections, the block
elision gate's five integer comparisons per lane-sample, the AoSoA round trip the fused
`simd1:eq+compressor` chain shares, and the graph dispatch the row cannot subtract. **Naming that
as control plane is a claim this document can now put a number on**, which it could not before: the
kernel's floor is 1.723 of the 4.904, so 65 % of the EQ row's isolated cost is outside the section
arithmetic.

---

## Boundary 4 — the limiter's floor moved because round 1 changed its shape, not its arithmetic

The limiter isolate measures 13.918 cycles/lane-sample against a 4.662 floor: 33.5 %.

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

The row measures 12.978 cycles/lane-sample, 18.0 % of that floor and the worst standing in the
table. That is not a defect in the fixed points: it is the statement that **what the idle row costs
is almost entirely the AoSoA transposes and the graph dispatch that the fixed points cannot skip.**
The rack performs one planar/AoSoA round trip per realised chain per block whether the chain
computes anything or not — 24 chains per block on this fixture, of which the three rack chains
compute nothing at all on this row.

**Amendment.** This boundary's floor is unchanged by the prepared-identity elision, and the reason
is worth stating because it is the same reason the boundary was drawn: the idle row's strip is the
*standing console strip*, whose 128 channel-lanes every one declare a real HPF and a real LPF. Its
builtins chain elides nothing, so its floor stays 69 lane-ops and its measurement does not move.
The row that moves is `dispatch_only`, whose builtins are the identity by construction.

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

**The additivity was checked, not asserted.** Every `console-benchmark.accepted.jsonl` under
`artifacts/` was validated against the validator as it stood at `98b5706` and against this one:
`compressor-round1` and `compressor-round1-baseline` pass both, and the ten older directories fail
both — for reasons that predate this change and belong to the record shapes the validator has moved
past since, not to these columns. No sealed record's verdict moved.

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

## Appendix — the prepared-identity elision

Added by the strip/overhead round. This appendix states the derivation the elision rests on, the
gate that decides it, and the two things an adversarial reader should check first.

### The map

A builtin section is disabled by *designing* it, not by branching around it: `SvfSection::design`
returns `SvfSection::IDENTITY` for a 0 Hz cutoff — `m0 = +1.0`, `m1 = m2 = c1 = a2 = a3 = +0.0`.
Take such a section whose two integrators are also `+0.0`, and run one frame of
`input_chain_block`'s body over any input `v0` the chain can produce.

`svf_step`'s frozen order, with `nc1 = neg(c1) = neg(+0.0) = -0.0`:

1. `v3 = v0 - ic2 = v0 - (+0.0) = v0`
2. `d1 = fma(nc1, ic1, a2 * v3) = fma(-0.0, +0.0, +0.0 * v3) = (-0.0) + (±0.0)`
3. `v1 = ic1 + d1 = (+0.0) + d1`
4. `d2 = fma(a3, v3, a2 * ic1) = fma(+0.0, v3, +0.0 * (+0.0)) = (±0.0) + (+0.0) = +0.0`
5. `v2 = ic2 + d2 = (+0.0) + (+0.0) = +0.0`
6. `ic1' = flush(ic1 + (d1 + d1)) = flush((+0.0) + (d1 + d1))`
7. `ic2' = flush(ic2 + (+0.0 + +0.0)) = +0.0`

**Correction to the first draft of this derivation, which claimed every intermediate is `+0.0`.**
Step 2 is `-0.0` whenever `v3` is negative or `-0.0`, because `a2 * v3` is then `-0.0` and
`(-0.0) + (-0.0) = -0.0`. The conclusion survives it, and it survives it for a reason that must be
written down rather than assumed: `+0` **absorbs** `-0` under round-to-nearest. So step 3 is
`(+0.0) + (-0.0) = +0.0` and step 6 is `flush((+0.0) + (-0.0)) = +0.0`. Both integrators are `+0.0`
after the frame, so the section is a fixed point of its own state and the argument runs for every
frame of every block, forever.

With `v1 = v2 = +0.0`, the output mix — `m2.fma(v2, m1.fma(v1, m0.mul(v0)))`, in that order — is
`(+0.0) + ((+0.0) + (1.0 * v0))`, which is the map

> `v |-> v + 0.0`

exactly: it sends `-0.0` to `+0.0` and fixes every other value. It is idempotent, so a run of `N`
consecutive identity sections is **one** `add(+0.0)`, and `input_chain_block_elided` emits exactly
one, **at the run's position in the chain**. The position matters: an identity high-pass followed by
a real low-pass must feed the low-pass `v + 0.0` and not `v`, or a `-0.0` reaches the recurrence that
should never have seen one.

Nonfinites are excluded upstream and are not part of this argument: `sanitize_gain_block` clears any
`|x| >= 1e30` (NaN included, by the one ordered compare) to exactly `+0.0` before the trim, and the
trim domain is the bounded −144..+24 dB of the version-1 contract. The environment premise is the
one the whole tree runs under: `CanonicalFpEnv` pins round-to-nearest with FTZ and DAZ clear at
every native entry, and wasm is spec-IEEE with no relaxed operations anywhere (`check-lane-policy`).

### The gate, and why it is on bit patterns

`input_chain_plan` elides a section only when **six coefficient words and two state words** are
**bit-pattern-equal** to the identity in **every lane of the bank**, padding lanes included. Three
separate claims, each load-bearing:

**Bitwise, not `==`.** Float equality calls `-0.0` equal to `+0.0`, and `-0.0` in a mix word is a
real divergence, not a pedantic one. With `m1 = m2 = -0.0` in both sections — a set of words a `==`
gate accepts as the identity — the chain emits `-0.0` for a `-0.0` input where the elided form emits
`+0.0`: `-0.0 * v1` is `-0.0` for a non-negative `v1`, and `-0.0` added to the `-0.0` the direct term
carries stays `-0.0`. The case is in the tree, measured, as
`negative_zero_mix_words_are_not_the_identity`.

*A correction to the finding that prompted this paragraph.* The divergence was originally attributed
to `-0.0` **state** words. That attribution is wrong, and the probe that produced this appendix says
so: with exact identity coefficients and both integrators in `{+0.0, -0.0}`, the section **is**
`v |-> v + 0.0` — `v1` can never be `-0.0`, because `ic1 = -0.0` forces `d1 = +0.0` (the product
`(-0.0) * (-0.0)` is `+0.0`) and `(-0.0) + (+0.0) = +0.0`, while `ic1 = +0.0` gives
`(+0.0) + (±0.0) = +0.0`; and the one path to `v2 = -0.0` needs `ic1 = -0.0`, which has just been
shown to wash `v1`. So a `-0.0` state word is genuinely inert. The state words stay in the bitwise
test anyway, for two reasons that are about the rule and not about this pattern: it is one rule
rather than two, and a `==` state test would need its own standing proof that no `==`-equal pattern
diverges — which is exactly the proof that just failed for the coefficients.

**The state words at all.** A genuinely non-zero negative integrator *is* a divergence: seeded at
`-1.0` in both sections, the identity chain emits `-0.0` for a `-0.0` input where the elided form
emits `+0.0`. `identity_coefficients_over_non_zero_state_are_not_elidable` forces the elision and
watches the bits move, so the check is not defensive.

**Every lane or none.** The kernel body is a vector body with no per-lane branch, so one real lane
keeps the section for the whole bank. A padding lane is not an exception in either direction: it
carries `SvfSection::IDENTITY` and `+0.0` state, so it qualifies, and it is tested on the same
footing as a member.

### Why "prepare time" is not an optimisation of "per call"

The plan is decided at bank construction and can never go stale in the unsafe direction:

* the six coefficient words are written once, at preparation. `hpf_hz`, `lpf_hz`, `trim_db` and
  `polarity_invert` all declare `BuiltinParameterUpdateRate::PreparedOnly` in
  `BUILTIN_PARAMETER_DESCRIPTORS_V1`, so no live surface can move them;
* an elided section's integrators are never written by the render path — the section that would
  have written them is gone. The render path's one write to the retained state anywhere is the
  boundary-check recovery, `state.andnot(bad)`, and that write is **monotone toward the identity**:
  it either leaves a word alone or replaces it with the `+0.0` the identity pattern wants. It can
  therefore never invalidate a `true`, and it is left bit-for-bit as it was rather than made to
  re-decide anything — a section it happens to make newly *elidable* simply stays unelided until the
  next reset, which is a missed saving and not a moved bit;
* the post-preparation writes that are not monotone are `InputStage::set_lane_state_words` — the
  fault-injection seam `tests/stage.rs` T5 and T6 use — and `InputStage::reset`, and both
  **re-decide the plan**. So the rule in the file is one line: every write to `state` outside the
  render path re-decides `plan`, and the render path's only write cannot need to.

The sanitise/trim/boundary-scan law, the sanitisation counter and the output-recovery path are
untouched by the elision and run bit-identically on every path.

### What an adversarial reader should check first

1. **That the elided path still counts and still scans.** The saving is the recurrence, not the D7
   policy. `identity_chain_block` keeps the per-frame sanitisation compare, the counter accumulation
   and the output boundary `mask_or`, in the same order, and returns the same `InputChainReport`.
2. **That nothing elides that should not.** The gate is `input_chain_plan`; the sixteen
   enabled/disabled section patterns at three widths are
   `elision_is_bit_identical_at_every_width_and_section_pattern`, and the two forced-elision arms are
   the evidence that the gate is buying something.
3. **That the gate sweeps section *shape*, not just section presence.** This one caught a real hole
   in the first draft of the gate, and it is the reason the sweep is 16 patterns x 16 shapes rather
   than 16 patterns. A real **low-pass** washes the sign of a `-0.0` on its own — its mix is
   `m0 = m1 = 0`, so the direct term that carries the sign is multiplied away — while a real
   **high-pass** does not, because `m0 = 1`. The console's chain is high-pass then low-pass, so a
   gate that ties each section's shape to its index only ever puts a *low-pass* after an elided
   section, and a misplaced `add(+0.0)` is invisible: dropping the run's add before a following real
   section was a **surviving mutant** until the shape axis was added, and dies at
   `pattern=0010, shapes=0010` — an elided section 0 followed by a real high-pass — once it is.

---

## Links

* **Directive:** issue #184, owner 2026-08-26; sequenced after the round-1 merges (#181, #182,
  #186-#188) and before #183's W4/W8 capture.
* **Floor table (authority):** `tools/miso-engine-bench/src/floor.rs`.
* **Validator (independent restatement):** `scripts/console-benchmark-record-lib.jq`,
  `scripts/console-benchmark-validator.jq`.
* **Mutation coverage:** `scripts/test-console-benchmark.sh`.
* **The elision and its gates:** `crates/miso-engine-lane/src/kernels/builtins.rs`
  (`input_chain_plan`, `input_chain_block_elided`),
  `crates/miso-engine-lane/tests/input_chain_elision.rs`,
  `crates/miso-engine-builtins/tests/stage.rs` T11.
* **Runner and its counter:** `scripts/run-console-benchmark.sh`,
  `scripts/check-bench-preconditions.sh`.
* **Inventoried sources:** `crates/miso-engine-compressor/src/kernel.rs`,
  `crates/miso-engine-parametric-eq/src/lib.rs`,
  `crates/miso-engine-true-peak-limiter/src/lib.rs`, `crates/miso-engine-builtins/src/`,
  `crates/miso-engine-lane/src/kernels.rs`, `crates/miso-engine-math/src/fast_db.rs`.
* **Standing qualification authority:** `artifacts/issue175/` for what the strip renders;
  this document accounts for what it costs. The measurement it quotes is `artifacts/issue184/`.
* **Precedent for the unit:** `docs/rulings/fast-db-tier-boundaries.md`, and its rule that a
  per-sample kernel component may not be sized by an isolated throughput loop — which is why the
  standings here are taken from the console rows and not from a microbenchmark.
* **Precedent for the ceremony:** `docs/rulings/unfused-multiply-add-audit.md`, itself modelled on
  `docs/rulings/fast-db-tier-boundaries.md`.
