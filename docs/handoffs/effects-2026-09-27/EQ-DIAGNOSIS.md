# Parametric EQ diagnosis (2026-09-27)

Base: `codex/batch-plumbing-floor-2` at `6ca203f8`. Diagnosis only: every prototype named below
was measured and reverted, and the tree carries none of them. The prototypes and harnesses are
recorded as patches beside this file (see "Reproduction").

Row: `sixty_four_track_eq_only` minus `sixty_four_track_builtins_only` (the "isolate"), the
standing fixture: 64 tracks, one bell per track (general band 0, the physical section 1), both
dedicated cuts and the other three general bands disabled, 48 kHz, 128-frame quantum.

## Verdict

* **Half of the executed EQ arithmetic is an identity section that computes nothing.** The
  stationary cascade rounds its live-section count up to whole depth-2 passes
  (`cascade_sections`, `kept = live.div_ceil(depth) * depth`), so the fixture's one live bell runs
  beside the disabled HPF as "padding". Dropping the padding (a class-A schedule change) measured,
  on clean builds: **native Simd8 isolate 15.0 -> 9.3 us per block (-38 %)**, and **wasm simd128
  (V8) isolate 43.2 -> 20.5 us (-52 %; the whole `eq_only` row -24 %)**. All 15 console
  workloads render the base digests at `Scalar`, `Simd4` and `Simd8` natively and in wasm.
* **The ruling's attribution of the gap (Boundary 3) does not hold on the current code.** There is
  no AoSoA round trip in the isolate (the EQ rides the builtins bank chain: 8 transposes per block
  with or without the EQ), and graph dispatch around the slot is at most 0.15 cycles per
  lane-sample. The gap is the padding section, the dry-mask selects and their per-block
  derivation, the elision gate, the 4.4 boundary scan, and, on this host, the issue rate of the
  recurrence.
* **The 53-op floor counts the implementation, not the algorithm.** It prices the padding section
  (25 ops) and a select on a band that can never be dry (1 op). The class-A floor for the fixture
  is **27 lane-ops (0.912 cycles per lane-sample at the ruling's constants)**. The ruling's
  3.7 ops/cycle is a Zen 5 figure; this host is Zen 3, where the SVF op mix sustains about 2.75.
* **The cascade-depth ruling holds for dual banks natively at both widths**, but in V8 depth 2 only
  ties depth 1 unless the pass is software-pipelined ("skewed"); a skewed depth-2 pass is the
  fastest arrangement measured on all three targets (native -13 %, V8 -18 % per pass).
* **Two real-session costs the fixture cannot show.** (1) Every admitted pass runs the masked
  kernel even when no lane is dry: V8 rebuilds each mask inside the loop (8 x86 instructions per
  select per frame). (2) A dedicated cut switched on and then off freezes a non-zero state in an
  identity section, and the elision gate then refuses the whole bank on every later block
  (measured: 0 of 9 blocks elide afterwards); the bank runs all six sections until a reset.

## Method

* **Host.** AMD EPYC 7313P (Zen 3, 16 cores, SMT), Linux 6.8, rustc 1.97.1 / LLVM 22, release
  profile (fat LTO, one codegen unit), `x86-64-v3` (`+avx2,+fma`). Every timed run was pinned to cpu
  31 (`taskset -c 31`, SMT sibling 15) under the shared `flock` timing lock, one hold per run
  (all holds under 5 minutes). Load average during timed runs: 1.5-8.2, mostly 3-5 (other agents
  building). `perf` is unavailable, so cycles are derived: nanoseconds times a core clock
  calibrated in process from a dependent scalar add chain (3 cycles per add), 3.69-3.70 GHz on
  every run.
* **In-process A/B.** A diagnosis build (`eq-diagnosis-prototypes.patch`) reads one flag word per
  bank call (`parametric_eq::diag::FLAGS`), so one runtime renders with each toggle in turn: 6
  interleaved rounds of 1,500 blocks, median of the per-round p50s, `builtins_only` measured each
  round as the control. Attribution toggles skip a piece (the output may change); prototype toggles
  are class-A candidates and were checked against the 64-block digests of all 15 `WORKLOADS` at
  `Scalar`, `Simd4` and `Simd8` natively and in wasm (0 mismatches).
* **Native Simd4.** An x86 build refuses to bind four-lane EQ banks
  (`bind_homogeneous_bank`, `lanes != Backend::current().width()`, `lib.rs:2400`), so the shipped
  native `Simd4` plan renders the EQ per node at width 1, splits the chain around it (32
  transposes per block against 16) and adds 64 units. The diagnosis build lets the bind accept four
  lanes (bind time only). Every `Simd4` figure below is that four-lane bank, which is what the
  browser runs. The digests are identical either way.
* **Wasm.** The console guest (`tools/wasm-console-guest`) built with `-C target-feature=+simd128`
  and the release profile, run under Node 22.23.2 (V8 12.4.254.21) on the same pinned core,
  `process.hrtime` around each `miso_console_render` call. The in-process A/B build carries a flag
  export; the headline numbers come from four **clean** guest builds (base, skew, no padding,
  both) loaded into one Node process and interleaved, because the flag build outlined `render` out
  of `process_bank` (it fails the `KERNEL_ROSTER` rule) and moved V8's code for it.
* **Kernel replicas.** The lane kernels the EQ calls, on fixture-shaped bell coefficients, timed
  per bank-block (`eq_diag.rs` `kernels`/`skew`/`vertical` natively; `eq-diag-tools/kbench` in V8).
  Each call refreshes both planes from a source first; the refresh is measured alone and
  subtracted.
* **Clean native builds.** `eq_rows.rs` built at base and at each minimal variant, run alternately
  three times.

## 1. Where the isolate goes (current tree)

Cycles per lane-sample (`cyc/ls`; 16,384 lane-samples per block), median of the in-process A/B
runs. At 3.7 GHz, 1 cyc/ls is 4.43 us per 64-track block.

| term | native Simd8 | native Simd4 (4-lane bank) | wasm simd128 (V8) | instrument |
|---|---:|---:|---:|---|
| **isolate** (row p50 difference) | **3.32-3.47** clean (14.7-15.3 us) | **6.4-6.8** (28-30 us) | **9.76** clean (43.2 us) | rows |
| kernel: one depth-2 masked pass `[HPF identity, bell]` | 2.78-2.82 | 5.32-5.34 | 7.24 | `skip_kernel` |
| - of which the identity padding section | 1.09 | 1.84 | 3.05 | replica: unmasked pass minus lone bell |
| - of which the four per-frame dry selects | 0.28 | 0.76 | 0.92 | replica: masked minus unmasked pass |
| - the live bell alone (depth 1, select-free) | 1.50 | 2.96 | 2.93 | replica |
| elision gate `block_admits_elision` (both planes) | 0.22-0.26 | 0.25-0.27 | 0.71 | `skip_gate`; replica 0.21 / 0.24 / 0.60 |
| 4.4 boundary scan `check_block` (both planes) | 0.12-0.18 | 0.21 | 0.57 | `skip_check`; replica 0.115 / 0.21 / 0.34 |
| dry-mask derivation `dry_mask`, per pass per block | 0.14 | 0.02 | 0.12 | `zero_masks` |
| elision state legs (b) and (c) | 0.03 | 0.06 | 0.19-0.21 | `skip_state_legs` |
| positive-zero input test (music) | 0.02 | 0.03 | - | `skip_zerotest` |
| remaining bank-call control (coefficient/state marshalling, list, guard, report) | ~0.2 | ~0.3 | ~0.6 | by difference |
| bank chain around the slot (drain, stage dispatch; no transpose) | 0.02-0.15 | 0-0.15 | 0.20 | `skip_bank` |
| silent path (idle row: two plane scans and the early return) | 0.33 (1.5 us) | 0.28 | - | `skip_bank` on `sixty_four_track_idle` |

The replica figures per bank-block (two channels x 128 frames), masked pass / the same pass
select-free / the lone bell select-free: native W8 5,883 / 5,301 / 3,069 cycles; W4 5,691 / 4,916 /
3,032; V8 7,062 / 6,125 / 3,002.

**What is not in the isolate.** At `Simd8` (and at a four-lane bind) the plan merges the EQ slot
into the builtins chain: `bank_shape` is `[8, 32]` against `[8, 24]`, and transposes per block are
8 in both rows. There is no AoSoA round trip to attribute. `acquire_resident_input`'s copy is not
on this path either.

**Silent path.** On the idle row the EQ's early return costs 0.33 cyc/ls natively (two full
`block_is_positive_zero` scans per bank, 680 cycles). Forcing the kernel instead costs +4.5 cyc/ls,
so the fixed point works. Issue #942 (wider chunk) already covers the scan.

## 2. The hot loop, disassembled

Per frame iteration of the stationary pass, both channels:

| build | loop | instructions / frame | lane-samples / frame | instructions / lane-sample | inventory (vector ops / lane-sample) | cycles / frame | IPC |
|---|---|---:|---:|---:|---:|---:|---:|
| x86 LLVM, Simd8, today | depth-2 masked | 117 | 16 | 7.31 | 6.25 (50 / 8) | 46 | 2.5 |
| x86 LLVM, Simd8, no padding | depth-1 select-free | 56 | 16 | 3.50 | 3.00 (24 / 8) | 24 | 2.3 |
| wasm bytecode, today | depth-2 masked | 272 (105 `v128`) | 8 | 34 (13.1 `v128`) | 12.5 (50 / 4) | - | - |
| V8 x86 code, today | depth-2 masked | 204 | 8 | 25.5 | 12.5 | 55 | 3.7 |
| V8 x86 code, no padding | depth-1 select-free | 83 | 8 | 10.4 | 6.0 (24 / 4) | 23.5 | 3.5 |

* **Native.** The depth-2 loop (release test binary, `process_bank::<f32x8>`) is 100 arithmetic
  instructions (40 `vaddps`, 4 `vsubps`, 28 `vmulps`, 8 each of `vandps`, `vcmplt_oqps`,
  `vandnps`, 4 `vblendvps`; exactly the inventory), 9 `vmovaps` spill reloads, 2 loads and 2 stores
  of the planes, 1 `vbroadcastss` and 3 of loop control; 33 instructions take a memory operand
  (coefficients from the stack). It is instruction-lean (1.17x the inventory); the gap is issue rate.
* **V8.** The same loop is 204 instructions: 47 `vmovups` spill/reloads (V8 keeps the 24
  coefficient vectors in stack slots and uses register operands only), and TurboFan **rebuilds every
  dry mask inside the loop** from four scalar stack words before a three-instruction `bitselect`:

  ```text
  vbroadcastss xmm7,xmm14 ; vinsertps xmm7,xmm7,[rbp-0x2f0],0x10 ; vinsertps ..,0x20 ;
  vinsertps ..,0x30 ; vcmpps xmm7,xmm7,xmm5,(eq) ; vpandn xmm15,xmm7,xmm4 ; vpand xmm4,xmm6,xmm7 ;
  vpor xmm4,xmm4,xmm15
  ```

  That is 32 instructions per frame for four selects that are all no-ops on the general band. The
  mask is `dry_mask`'s `L::load(&decisions).eq(L::splat(1.0))` (`lib.rs:1150`).
* **x86 masked depth 1.** A masked depth-1 pass stores `select(mask, load(slot), wet)` to the same
  slot, which LLVM folds into `vmaskmovps` (the #944 pattern): 4,519 against 3,069 cycles per bank
  select-free. The shipped dedicated-cut ramp kernel (`svf_block_ramped_with_dry_mask`) has the same
  fold (two `vmaskmovps` loops in `process_bank::<f32x8>` today).

## 3. The machine, and the cascade-depth ruling

Measured on this host with a 12-chain probe (`eq-diag-tools/probe.rs`): `vaddps` 2.00 ops/cycle,
`vmulps` 2.00, mul+add 3.85, compare+blend 3.99, add+and 3.49, add+flush 3.17. The SVF section mix
never exceeded about **2.75 ops/cycle** in any arrangement (cross-bank `S = 4, D = 1` and skewed
`S = 2, D = 2` both 9.0-9.2 cycles per 25-op section-frame at W8). One chain's recurrence
(ic2 -> v3 -> a3*v3 -> d2 -> d2+d2 -> ic2+ -> flush) is **20.4 cycles per frame** (measured `S1 D1`).

Cycles per section-frame (one section, one channel, one frame of a bank), bell sections, lower is
better:

| target | S2 D1 | S2 D2 | S2 D3 | S2 D4 | S2 D6 | S2 D2 skewed | S2 D3 skewed | S1 D1 | S1 D2 | S1 D3 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| native W8 | 12.0 | 10.3 | 11.7 | 13.1 | 15.1 | 9.0 | 8.9 | 20.4 | 12.8 | 11.4 |
| native W4 | 11.8 | 9.6 | 11.3 | 12.5 | 15.1 | 9.4 | 8.7 | 20.4 | 12.8 | 11.4 |
| V8 (wasm W4) | 12.1 | 12.0 | 14.9 | 16.1 | - | 9.8 | 12.2 | 19.8 | 12.9-13.9 | 13.8 |

* Dual banks: depth 2 is best unskewed natively (D3 +13 %, D4 +27 %), and in V8 it only ties
  depth 1. **The `SVF_CASCADE_DEPTH = 2` ruling holds** for x86 and for V8 on x86 (16 `xmm`).
* Skewing (section `k` runs frame `i - k` in iteration `i`, so the sections of one iteration are
  independent) makes depth 2 the best arrangement everywhere. Op-by-op ("vertical") interleaving
  of the chains added nothing over skewing.
* Mono (collapsed banks): depth 3 wins by 11 % natively and loses in V8.
* The browser on arm64 has 32 vector registers, which is the ruling's reopening condition. Not
  measurable here; nothing below depends on it.

## 4. The floor, rechecked

The ruling's inventory (`effect-floor-accounting.md`, "EQ inventory"; `floor.rs:69`,
`console-benchmark-record-lib.jq:69`): 25 per executed section (SVF step 19, output mix 5, select
1), two kept sections, plus the 3-op boundary scan: **53**. Recounted on the current source, the 53
is an exact count of what the stationary pass executes (the disassembly matches it instruction for
instruction). It is not the floor of the algorithm:

| item | lane-ops | required? |
|---|---:|---|
| live bell: `svf_step` 19 + output mix 5 | 24 | yes |
| select on the live bell | 1 | no: a general band is never dry (`dry_mask` returns the empty mask) |
| identity padding section (HPF) with its select | 25 | no: the elision proof already covers dropping it; the padding exists only to keep one depth-2 instantiation |
| boundary scan | 3 | yes (4.4) |
| **class-A floor for the fixture** | **27** | 0.912 cyc/ls at 8 x 3.7 |

For `active` live sections after the padding and the redundant selects are gone, the executed
inventory is `24 * active + 3` on an admitted block, and `25 * 6 + 3 = 153` when elision is refused
(all six sections, masked).

**The machine constant.** 3.7 ops/cycle was probed on Zen 5. On this Zen 3 host the SVF mix peaks
near 2.75, so 27 lane-ops are 1.23 cyc/ls of throughput, and one live section per bank is
latency-bound anyway (20.4 cycles per frame per chain; the two channels give two chains): about
1.28-1.50 cyc/ls. Records taken on this host report a %-of-floor about 1.35x worse than the same
code would score on the host the constant came from.

**The gap, named (exit clause), native Simd8, about 3.5-3.7 cyc/ls in process (3.3-3.5 on a clean
build) against 0.912:**

| part of the gap | cyc/ls | reason |
|---|---:|---|
| identity padding section | 1.09 | schedule artefact of depth-2 rounding (implementation; EQ-1) |
| dry selects in admitted passes | 0.28 | masked kernel used when the selects are provably no-ops (implementation; EQ-2) |
| dry-mask derivation | 0.14 | per-lane bit extraction through the stack, per pass per block (implementation; EQ-2) |
| elision gate | 0.24 | guard on the optimisation, 7 integer vector ops per 8 words (implementation; EQ-5 trims it) |
| boundary scan as a second pass | ~0.1 of 0.15 | required check, run as a separate re-read (implementation; EQ-6) |
| state legs, marshalling, report, chain | ~0.3 | control plane per bank call (implementation, diffuse) |
| live section at 1.50 against 0.91 | 0.59 | recurrence latency on Zen 3 (20 cycles per frame per chain, two chains per bank) and the Zen 5 constant; within one bank no implementation removes it; cross-bank fusion would (not proposed) |

## 5. Ranked changes

Savings are per 64-track block. "Fixture" is `sixty_four_track_eq_only`; the multi-band figures
come from the kernel replicas (per bank-block times 8 native banks or 16 wasm banks).

### EQ-1. Drop the identity padding section (odd live counts run a depth-1 tail)

* **What.** `cascade_sections` and `cascade_sections_mono`: `kept = live`. `interleave` and
  `interleave_mono`: the pairs, then one depth-1 pass over the last live section.
* **Saving (measured, clean builds).** Fixture: native Simd8 -5.8 us (isolate 15.0 -> 9.3 us);
  wasm -22.4 us (43.2 -> 20.5 us); native Simd4 four-lane -11.6 us (toggle). Mono replicas:
  -21 % (native W8) and -29 % (V8) of the collapsed kernel. Every dual console row that carries
  the EQ should move by about the same absolute amount (not timed here).
* **Class A.** The dropped section is an identity section at `+0.0` state that the existing proof
  already lets the cascade drop; the padding was only there to keep one kernel instantiation.
* **Wasm.** The variant build passes `KERNEL_ROSTER` (`parametric-eq f32x4 dual` still one
  arithmetic-carrying `process_bank`, vector 312, scalar 0; kernel count unchanged at 14).
* **Gate.** New public-API scenario test pinned on base (odd live counts 1/3/5, cuts live on some
  lanes); the in-crate elided-versus-full test over all 64 live masks; all console digests. Five
  in-crate tests pin the rounded count and change to `live`: `the_two_channels_are_judged_together`
  (`lib.rs:3988-3993`), `the_shipped_shape_actually_elides` (`:4006-4014`),
  `a_section_live_on_one_lane_is_not_elided` (`:4054`), `a_negative_zero_input_refuses_elision`
  (`:4094`) and `a_non_finite_or_oversized_input_refuses_elision` (`:4131`). Floor constants
  53 -> 27.

### EQ-2. Run every pass of an admitted plan select-free

* **What.** When `cascade_sections` returned a shortened (admitted) list, `interleave` and
  `interleave_mono` call `svf_cascade_interleaved` for every pass and never build a dry mask. The
  full six-section plan (refused, or all live) keeps the masked kernel.
* **Why it is exact.** Under admission the input carries no `-0.0` and is finite. A dry lane is
  a dedicated cut at the exact identity words; its wet output is
  `(+0*v2) + ((+0*v1) + x)`, which is `x` bit for bit for every finite `x != -0.0` and any finite
  state, and its state update does not read the mask. So the select always returns what the wet
  path already holds. The crate's own suite (mixed cuts, signed-zero refusals, disabled-cut
  oracles, mono collapse, bank partition invariance) passes with this change in dev and release;
  only the five kept-count pins move (they are EQ-1's).
* **Saving.** Every admitted pass today runs the masked kernel, even with two live general bands
  and all-clear masks. Per two-section pass: native W8 5,878 -> 5,301 cycles (-10 %), W4 -13 %,
  V8 7,062 -> 6,125 (-13 %), which includes V8's in-loop mask rebuilds. Two live bands per track:
  native about -0.6 us, wasm about -4.0 us. On today's fixture (before EQ-1): native -0.28 cyc/ls in
  the kernel plus the 0.14 of derivation. After EQ-1 the fixture's one pass is already select-free,
  so it gains nothing there.
* **Gate.** A test-only counter of masked passes; a scenario with a dedicated cut live on some lanes
  and frozen state on the dry lanes, pinned on base; M1 (masked everywhere) red on the counter.

### EQ-3. Software-pipeline (skew) the depth-2 cascade

* **What.** A lane kernel beside `svf_cascade_interleaved` in which section `k` of each stream runs
  frame `i - k` in iteration `i` (prologue, branch-free body, epilogue); the EQ's depth-2 passes
  call it.
* **Saving.** Per two-section pass: native W8 -13 % unmasked (5,301 -> 4,616) and -18 % masked;
  W4 -3 % unmasked, -11 % masked; V8 -18 % unmasked (6,124 -> 5,021), -16 % masked. Clean wasm
  build on today's padded fixture: -6.3 us (isolate -14.5 %); native clean -1.3 us. After EQ-1 the
  fixture has no depth-2 pass; two live bands: native about -0.7 us, wasm about -4.8 us; six live
  bands: native about -4.4 us, wasm about -14 us.
* **Class A.** Each chain runs `svf_step` and the output mix on the same inputs in the same
  order; only the interleaving of independent chains changes. Checked bit-identical against the
  shipped kernel over 40 hostile blocks (signed zeros, subnormals) at S1/S2, D2/D3, masked and not.
* **Risk.** V8's register allocation: the flag build (six kernel variants in one outlined function)
  measured +13 us, the clean build -6.3 us. The gate must be a clean wasm build.

### EQ-4. Keep an identity section with a frozen state elidable (the disable cliff)

* **What.** Elision leg (b) (`section_state_is_positive_zero`, `lib.rs:975`) requires an elided
  section's state to be exactly `+0.0`. An identity section's recurrence leaves any finite state
  word whose magnitude is at least `FLUSH_EPS` exactly where it is (`c1 = a2 = a3 = 0` make every
  increment a signed zero), and its output under admission is `x`. So the leg can accept "every
  state word is `+0.0`, or finite with magnitude at least `FLUSH_EPS`".
* **Why it matters.** Switching a dedicated cut on and then off (a prepared target, #807) ramps it
  to the identity words and freezes whatever state it had. Today the whole bank then refuses
  elision on every later block: measured 0 of 9 blocks elide after the disable, against 9 of 9
  with the relaxed leg, and the relaxed arm renders the full cascade's bits and integrators. An
  affected bank runs six sections instead of one: native W8 about +14,600 cycles (+3.9 us) per 8
  tracks, V8 about +18,200 cycles (+4.9 us) per 4 tracks, on every block until a reset. A restored
  payload with a non-zero state in a disabled section has the same effect.
* **Class A**, with an amendment to the elision proof. `eq-disable-cliff-test.patch` has the test.

### EQ-5. The elision gate's min/max form

* **What.** `block_admits_elision` as `nearest = min(nearest, bits ^ 0x8000_0000)` and
  `largest = max(largest, bits & 0x7fff_ffff)`, admitted iff `nearest != 0 && largest <= CEILING`.
  The same predicate (checked on every ordered pair of 16 edge patterns).
* **Saving.** Replica: native W8 431 -> 309 cycles per bank (-28 %), V8 617 -> 402 (-35 %).
  In process: wasm -1.0 us, native Simd8 -0.1 to -0.25 us, Simd4 -0.1 us.
* **Class A.** Same verdict on every input.

### EQ-6. Fold the 4.4 boundary scan into the cascade's final store

* **What.** The last pass of an admitted stationary block accumulates `abs(y) < BLOCK_LIMIT` per
  stream as it stores, and `render` uses that verdict instead of re-reading both planes. A
  zero-section plan, a refused plan and the ramped path keep `check_block`.
* **Saving.** Replica (depth-1 pass): V8 -294 cycles per bank (about -1.3 us per block); native W8
  -125 (about -0.3 us). The depth-1 kernel is latency-bound, so the three extra ops ride free.
* **Class A.** The same conjunction over the same words; the verdict, zeroing and reset are
  unchanged. It needs a verdict-returning lane kernel variant, so it touches `crates/lane`.

### EQ-7. Keep the elision state legs off the block

* **What.** Legs (b) and (c) read state that only a restore, a reset or a ramp end can change
  (the kernel never writes `-0.0`, and an identity section's state does not move). Cache the two
  answers per section at those sites, beside `identity`.
* **Saving.** wasm -0.85 us, native -0.17 (Simd8) / -0.33 (Simd4) us. Class A. Lands after EQ-4,
  which changes leg (b).

### EQ-8. Bind partial EQ cohorts as banks (#888), with one amendment

A per-node EQ costs about 6,750-8,400 cycles per track-block natively (all-scalar row: 116.7 us for
64 tracks), against 3,069 per eight-lane bank after EQ-1 and 5,883 today, so banking a partial
cohort wins even for one member. **Amendment:** an absent-slot lane must be *dry-selected*, not
given identity coefficients. Identity coefficients rewrite `-0.0` to `+0.0` and turn an infinity
into NaN (`0 * inf`), so they are not bit-identical to rendering that track without the EQ. With
EQ-2 the admitted path needs no mask; the refused path must keep one, and a masked depth-1 pass
folds into `vmaskmovps` at W8.

## 6. Not worth doing

* **Deeper cascades (D3, D4, D6) for dual banks.** Slower on every target (table in section 3).
  D3 for mono wins 11 % natively and loses 7 % in V8.
* **Op-by-op interleaving of the chains.** No gain over skewing (native W8 4,929 against 4,616).
* **Cross-bank fusion (`S = 4`).** A lone section at W8 runs 23 % faster with four streams, but it
  needs two banks in one kernel call across the effect contract's per-bank boundary. An
  architecture issue, not a bounded change.
* **Kind-specialised kernels** (a bell has `m2 = 0`, `m0 = 1`: 3 of 24 ops). Exact only under the
  `-0.0` gate, one instantiation per kind, and the depth-1 kernel is latency-bound.
* **`(ic + d) + d` for the state update, fused multiply-add, relaxed-simd.** Class B; the first
  also rounds twice where today rounds once (`d + d` is exact), for no latency gain. Not proposed.
* **Removing the elision gate by a `+0.0` canonicalisation.** Possible and class A in principle
  (an elided general band is `x + 0.0` on finite input), but the add must sit at every elided
  general-band position between live sections. Design work, not a bounded change; EQ-5 takes a third
  of the gate for a few lines.
* **Chain overhead, the positive-zero test, coefficient marshalling.** Each at most 0.15 cyc/ls.
* **Clearing a disabled section's state instead of EQ-4.** Moves bits for `-0.0` input.

## 7. For the owner

1. **Floor accounting.** Adopt 27 lane-ops for the fixture once EQ-1 (and EQ-2) land, and restate
   the inventory as `24 * active + 3` for admitted blocks. The 3.7 ops/cycle constant is Zen 5's;
   records taken on this Zen 3 host look about 1.35x worse against it. Rule whether floors are
   per host.
2. **Cascade depth for the browser on arm64.** The ruling's reopening condition (more than 16
   registers) is the browser on Apple Silicon. Measure there before changing the depth; keep 2 now.
3. **Native Simd4 records.** The shipped native `Simd4` plan renders the EQ (and the compressor,
   transient shaper and gate, which have the same bind check) per node, so the native `Simd4` legs
   are not a four-lane proxy for the browser. Letting a native build bind four-lane banks moved no
   digest. Rule whether the native `Simd4` leg should bind them.
4. **No class-B change is proposed.** EQ-1 to EQ-7 are all class A.

## Reproduction

Patches (each applies to `6ca203f8` alone):

* `eq-diagnosis-prototypes.patch`: the flag word and toggles in `parametric-eq` (feature `diag`),
  `tools/console-workload/tests/eq_diag.rs` (`facts`, `rows`, `ab`, `digests`, `kernels`, `skew`,
  `vertical`), `eq_rows.rs` (`clean_rows`, `gate_forms`, `fused_check`), the guest's
  `miso_console_set_eq_diag` export, and `docs/handoffs/effects-2026-09-27/eq-diag-tools/`
  (the wasm replica crate `kbench`, the Node drivers, the probe, the loop summarisers and the
  build/run scripts; their scratch paths need editing before reuse). This patch leaves
  `parametric-eq`'s in-crate tests uncompiled (they call the unflagged signatures).
* `eq-variant-no-padding.patch`, `eq-variant-no-padding-select-free.patch` (EQ-1 + EQ-2),
  `eq-variant-skewed-depth-2.patch` (EQ-3): the minimal variants that were timed as clean builds.
* `eq-disable-cliff-test.patch`: EQ-4's measurement (`cargo test -p parametric-eq --lib
  diag_disable_cliff -- --nocapture`).

Commands (build outside the lock, time inside it):

```text
CARGO_INCREMENTAL=0 cargo test --release -p console-workload --test eq_diag --no-run
flock -w 7200 LOCK taskset -c 31 target/release/deps/eq_diag-<hash> --ignored --nocapture \
    --test-threads 1 ab          # EQ_DIAG_FLAGS=0,1,2,... EQ_DIAG_BACKENDS=8,4 EQ_DIAG_ROUNDS=6
RUSTFLAGS="-C target-feature=+simd128" cargo build --release --target wasm32-unknown-unknown \
    -p wasm-console-guest
flock -w 7200 LOCK taskset -c 31 node wasm_variants.mjs guest_base.wasm guest_nopad.wasm ...
node --no-liftoff --print-wasm-code print_code.mjs guest_base.wasm   # V8 code of the hot loop
```

Flag values: `skip_bank` 1, `skip_kernel` 2, `skip_gate` 4, `skip_check` 8, `skip_zerotest` 16,
`zero_masks` 32, `skip_state_legs` 64, `gate_minmax` 128, `no_pad` 256, `unmasked` 512,
`pad_general` 1024, `skew` 2048; the four-lane native bind is `1 << 16` at build time.
