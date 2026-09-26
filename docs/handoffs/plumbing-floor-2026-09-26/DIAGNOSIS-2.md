# Plumbing-floor diagnosis 2 (2026-09-26, after #925-#928)

Rows: `sixty_four_track_plumbing_only` (bound feed, recorded 3.336 / 3.377 us p50) and
`sixty_four_track_plumbing_ring` (driver-fed, recorded 2.915 / 3.216 us), both from
`artifacts/plumbing-floor/`. Base: `main` at `14f2917b`. This is diagnosis only. Every
prototype named below was measured and then reverted; the tree carries none of them.

## Verdict

* **The ring row (the production feed) can drop from ~2.3-2.5 us to ~1.2-1.3 us in process.**
  Three bounded, bit-identical changes were measured as working prototypes. The remaining cost
  after them is almost all the fused kernel's own loop.
* **The bound row (the headline number) can drop from ~3.2 us to ~2.6-2.8 us.** Its input model
  is a host processor that copies 64 KiB per block into the arena. That makes it
  L2-bandwidth-bound at three times the minimum traffic, and no kernel change removes the copy.
* **On this host the floor is about 2,048 cycles (0.55 us), and two separate limits set it.**
  The arithmetic and the L2 fill of the 64 KiB input each take 2,048 cycles, so the row is
  compute-bound and bandwidth-bound in equal measure. PLAN.md's memory term was half the true
  value.
* **Neither row has an engine-side block copy left.** The only per-block memcpy is the bound
  row's harness processor.
* **Two measurement effects make the recorded number move for reasons outside the engine.** The
  heap alignment of the harness's frozen blocks moves the ring row by about ±0.1 us. The bench
  binary also compiles the render path with `graph/test-support`, which is not needed.

## Method

Host: AMD EPYC 7313P (Zen 3), cpu 31 pinned (`taskset -c 31`), SMT sibling 15 idle, loadavg
1.0-2.4, rustc 1.97.1, release profile (fat LTO, one codegen unit), `+avx2,+fma`. `perf` is
unavailable, so every cycle count is derived: nanoseconds from `Instant` multiplied by a core clock
calibrated in process from a dependent `vaddss` chain (3 cycles per add). The calibration gave
3.697-3.703 GHz on every run. The measurements came from four instruments:

* **Phase probes.** These use the existing `graph::test_only_phase_profile` instrument (each probe
  transition costs about 50-90 cycles and is charged to the phase it closes). Each "probes off"
  figure is the p50 of 4,000 back-to-back blocks, repeated three times. A bench-shaped loop
  (`bench_support::timing::timed` plus a SHA-256 of the output between blocks) gave the same p50
  as the tight loop to the nanosecond on both rows: 3,186 ns bound and 2,284 ns ring.
* **Kernel replicas and machine probes.** These cover L1/L2 bandwidth, the Zen 3 FP mix rate,
  alignment and 4K-aliasing sweeps, and a verbatim copy of the engine's `route_reduce`. Every
  replica kernel was checked bit-identical against a scalar mix-then-sum reference.
* **In-engine prototypes.** Test-support-only flags were read once per block. Each flag was
  toggled on the same runtime, so every A/B comparison runs at the same heap addresses. The
  measurement used four runtimes per row (different addresses) and three runs. Before each flag
  was timed, the harness checked that it produced the same 64-block digest as the unmodified
  path on both rows.
* **The patch.** Harness and prototypes together are
  `docs/handoffs/plumbing-floor-2026-09-26/diagnosis-2-prototypes.patch`, which applies to
  `14f2917b`. Run it with `CARGO_INCREMENTAL=0 taskset -c 31 cargo test --release -p
  console-workload --test plumbing_diag2 -- --ignored --nocapture --test-threads 1 <rows|memory|
  fp_throughput|kernels|alignment|aliasing|verbatim|experiments|one_config>`. Set
  `DIAG2_CLAIM_MOD64` / `DIAG2_OUT_MOD64` to pin the frozen blocks' and the output planes'
  address modulo 64.

The in-process p50s sit below the recorded ones: 3.19 vs 3.34 us bound, 2.28-2.49 vs 2.92-3.22 us
ring. The runtime-to-runtime spread inside a single process is itself 8,100-10,100 cycles on the
ring row (the heap placement section of question 5 explains it), and the recorded run was
uncontrolled (loadavg 1.6-3.3). Every figure below is an in-process figure. None of them is the
row's number.

## 1. Where the time goes, per block (current tree)

Shares are of the probes-on phase sum. The probes add about 900 cycles in all (probes-on mean
9,391 against probes-off 8,455 on the ring row, 12,690 against 11,789 on the bound row), mostly
inside the unit-loop phases. Read the phase figures as ±10 %, and treat the probes-off A/B deltas
in section 6 as the authority on savings.

### Ring row: 8,451 cycles p50 (2,284 ns); probes-on phase sum 9,236

| term | cycles | share | how measured |
|---|---:|---:|---|
| fixed: render wrapper, envelope check, observation boundary, set `begin_block` (dyn), empty copy loop, `observation_validity` (dyn), audit scope | ~200-300 | 3 % | `enter` + `source` probes (102 + 198, each containing one ~37-cycle `Instant`) |
| **64 inert `SourceInput` units** (dispatch only: `execute` then `execute_op`, which returns at `runtime.rs:3361`, then `observe_unit`'s flag) | **2,090-2,103** (32.7 per unit) | **23 %** | phase probe; A/B: skipping them saves 1,800-2,150 |
| Output unit, of which: played-plane resolution (`OutputSources::input`, 64 times: two indirect calls each, `Option<(&[f32],&[f32])>` returned through memory and moved with a 256-bit load, which defeats store-to-load forwarding) | 1,170-1,340 (18-21 per input) | 13-15 % | A/B: 8 extra resolutions per block, `(delta)/512` |
| Output unit, of which: per-pair setup and per-chunk exhaustion tests (`route_pair`/`route_run`: slice arrays, `.any()` length checks, `split_at_mut`, a `?` per chunk, the first-pair iterator shape) | ~1,300-1,400 | 14-15 % | A/B (flags 768 vs 896): the same arithmetic as a zip of `chunks_exact` per pair, -1,300 to -1,400 |
| Output unit, of which: input misalignment (the frozen blocks at 16/48 mod 64 turn half the 32-byte loads into cache-line splits) | 0-450 | 0-5 % | controlled alignment sweep (`one_config`) |
| Output unit, of which: the pair loop itself (25 instructions per pair-chunk, about 6 cycles each, 512 pair-chunks) | ~3,100-3,700 | 34-40 % | replica on the same layout: 3,086-3,317 at 0/32 mod 64, 3,574-3,822 misaligned |
| **Output unit total** | 6,834-6,842 | 74 % | phase probe |

### Bound row: 11,789 cycles p50 (3,186 ns); probes-on phase sum 12,600

| term | cycles | share | how measured |
|---|---:|---:|---|
| fixed | ~200-250 | 2 % | `enter` + `source` probes |
| 64 bound units: `FrozenGraphSource::process`'s two 512-byte memcpys (L2-bound: 64 KiB read plus 64 KiB read-for-ownership into the arena) | ~4,200 (66 per unit) | 33 % | copy replica 4,224; machine probe: a 64 KiB copy costs 4,159 cycles |
| 64 bound units: dispatch (`execute`, the 9-argument out-of-line `execute_op`, `write_stereo`, indirect `process`, two libc calls, `observe_unit`) | ~3,300 (51 per unit) | 26 % | phase 7,456-7,556 minus the copy; A/B fast path saves 13-20 per unit |
| Output unit: resolution (`lease.read_stereo` path, 3.7 per input) | ~240 | 2 % | A/B, as for the ring row |
| Output unit: kernel over 64 KiB of arena the bound units just wrote (dirty, mostly back in L2) | ~4,600 | 37 % | phase 4,810-4,871 |

### Memory traffic per block (the minimum is 64 KiB in, 1 KiB out)

* **Ring row.** 64 KiB of L2-to-L1 fill (the 64 played planes), which is the minimum. The 1 KiB
  master stays in L1. The pair kernel rewrites that master 32 times per block (992 reloads and
  1,024 stores of 32 bytes, all L1 hits), which costs store-port time but no L2 traffic.
* **Bound row.** About 192 KiB of L2-to-L1 fill: the frozen 64 KiB, 64 KiB of read-for-ownership
  on the arena, and up to 64 KiB of arena re-read by the Output unit. Up to 64 KiB of dirty arena
  lines are also written back to L2. At the measured 32 B/cycle that fill is at least 6,144
  cycles, over half the row. The bound row is bandwidth-bound by its input model.
* **Where the input sits between blocks.** It sits in L2 on both rows, never in L1. The 64 KiB
  sequential stream through the 32 KiB 8-way L1 evicts every line before the next block reuses
  it. The machine probe read 48-384 KiB at 31.8-32.0 B/cycle, and 8-32 KiB at 59-62 B/cycle.

### Per-block bookkeeping

Per block this path runs only a few counters. It has no meters (`PlanConfig::BASELINE`), no
retirement-queue poll (the bench drives `PreparedRenderPlan` directly, not
`RealtimePlanOwner`), and no generation tags (the frozen driver has none). The bookkeeping it
does run is `rendered_blocks`, `next_absolute_sample`, `response_snapshot_valid`, the observation
cursor and its flag, and the audit scope (see question 5). Together these are the ~200-300-cycle
fixed term.

The production source set's `begin_block` does per-source SPSC pops and generation checks
(`crates/source/src/lib.rs` near 1820). This row does not measure that work, so the ring row
understates a real ring-fed session's feed cost by an amount nobody has measured yet.

## 2. The floor, recounted

A 64-track sum into one stereo bus at 8 lanes needs 64 x 128 x 2 = 16,384 lane-samples per block:

| operation | count per block | Zen 3 port bound |
|---|---:|---:|
| input loads (64 KiB) | 2,048 x 32 B | 1,024 cycles at 2 loads per cycle |
| `vmulps` (2 per channel of `mix2x2`) | 4,096 | **2,048** on the two FMUL pipes |
| `vaddps`: 2,048 mix + 2,016 accumulate (the first contributor stores) | 4,064 | **2,032** on the two FADD pipes |
| stores (the 1 KiB master) | 32 minimum (today's pair kernel does 1,024) | 32 (1,024) |
| L2-to-L1 fill of the 64 KiB input, measured 32.0 B/cycle | 1,024 lines | **2,048** |

* **Arithmetic.** 8,160 vector ops. The Zen 3 rate for a 1:1 mul:add stream measured **3.97**
  ops/cycle (12 independent chains; `vaddps` alone gives 2.00), so the arithmetic takes
  2,048-2,055 cycles, or 0.555 us. The ruling's 3.7 ops/cycle was probed on Zen 5, and 2,214
  cycles (0.598 us) stays within 8 %, so `floor.rs` needs no change.
* **Memory.** PLAN.md's term (b) is wrong on this host. It assumed one 64-byte line per cycle
  (1,024 cycles) and said the stream "overlaps". This L2 delivers half a line per cycle, so the
  64 KiB fill takes 2,048 cycles, the same as the arithmetic. The row is compute-bound and
  L2-bandwidth-bound in equal measure. That means perfect overlap is the best case, not
  something to expect.
* **Floor.** max(2,048, 2,048) + fixed ~150-250 = **~2,200-2,300 cycles (0.60-0.62 us)**. The
  0.60 us figure is therefore right, though PLAN.md reached it by the wrong route.
* **Best measured kernel shapes on this host.** Pairs from L2 run at 3,086-3,317 cycles
  (inputs at 0 or 32 mod 64) or up to 3,822 (misaligned). Pairs from L1 run at 3,090. The
  frame-tiled T = 4 kernel reaches 2,662-2,722 from L1 and 2,913-3,038 from L2, but only with
  64-byte-aligned inputs.
* **The ceiling on the kernel.** Both loops sustain about 4.5 instructions per cycle. So the pair
  kernel runs at about 6 cycles per pair-chunk rather than the 4 its FP pipes allow. That, not
  memory, is why no measured kernel gets under about 2,700 cycles. The L1-vs-L2 gap is small
  (0-400 cycles), which shows the L2 stream is already mostly hidden.

## 3. Copies still executed per block

| where | bytes per block | needed? |
|---|---:|---|
| ring row: played planes to the Output reduction | 0 (read in place, #927; pinned `[0, 64 x blocks, 0]`) | none left |
| both rows: Output reduction to the host planes | written in place (#916); no arena-to-host copy | required (it is the output) |
| bound row: `FrozenGraphSource::process`, two `copy_from_slice` of 512 B per track | 64 KiB | **not an engine copy**: the harness processor's own write under the `GraphRuntimeProcessor` contract (a bound processor fills the block it is handed). It is the reason the arena is read-for-ownership from L2. Change 5 below keeps the write but makes it land in L1. |
| pair kernel: the master reloaded and stored once per pair | 31 re-passes of 1 KiB, in L1 | not a copy. A register-tiled kernel removes them but did not win on this host (see "not worth doing") |
| production native host (`host-core` `render_planar`) | writes the caller's planes directly | none. The Web Audio output copy is the known required one and is not on this row |

**Verdict: no block-sized engine copy remains on either row.**

## 4. Does the schedule cost anything per unit? One fused unit? Order?

* **Per-unit cost.** Yes. An inert unit costs 32.7 cycles and a bound unit about 51 cycles of
  dispatch, measured. The cost is in the render loop, then `Runtime::execute` (a
  `split_at_mut`, two identity-row loads, the resident-input test, `output_unit == Some(index)`,
  the `HostMaster` reborrow, the `OutputSources` construction), then the out-of-line
  `execute_op`. That call takes 9 arguments, some on the stack, into a 1,248-instruction body
  that saves 6 registers and opens a 760-byte frame. Last comes the `observed` test in
  `observe_unit`. The bound path adds `write_stereo`'s checks and the indirect `process`.
* **One fused unit.** The routes are already fused into the single Output unit (#926). What is
  left of "65 units" is 64 input units, and on the ring row they do nothing. Dispatching only the
  units that do work (change 1) is the "one fused unit" for that row, and it measured -1,800 to
  -2,150 cycles.
* **Frame-tiled alternative.** A different fused shape keeps the running sum for T chunks in
  registers across all 64 inputs. It is class A but did not beat pairs in the engine (below).
* **Summation order today, which every change keeps.** Per frame and per plane:
  `v = mix(in_0); v = v + mix(in_1); ...`, strictly left to right in the Output op's edge order
  (edges sorted by `GraphEdgeId`). `mix` is `lr.fma(r, ll.mul(l))` = `(lr*r) + (ll*l)` for left
  and `(rr*r) + (rl*l)` for right, with every product and sum rounded separately (`Lane::fma` is
  unfused). The pair kernel preserves this by storing the running sum and reloading it, which
  does not change any bits.

## 5. What the harness adds that is not engine cost

1. **`tools/bench/Cargo.toml:31` builds `graph` with `features = ["test-support"]`.** Nothing
   under `tools/bench/src` uses a test-only API. `cargo check --release -p bench` passes without
   the feature, and `cargo tree` then shows it off. With it on, the benchmarked render path
   compiles in several hooks: a `Probe::start` TLS read per block, a probe test per unit, a TLS
   counter increment (`inc fs:[..]`) per in-place Output input (64 per block on the ring row),
   and the resident-input TLS test (short-circuited on these rows). Estimated at 100 cycles or
   less (not isolated), but it means the binary is not the production binary. **Drop the
   feature.**
2. **Heap-alignment lottery.** `FrozenSourceDriver`'s claims (`console-workload/src/lib.rs:1811`,
   one 64 KiB `Box`) and `SessionRuntime`'s output `Vec` (line 1097) land on any 16-byte boundary.
   The controlled sweep measured:
   * ring row with the claims at 0/32 mod 64: 8,377-8,562 cycles;
   * ring row with the claims at 16/48 mod 64: 8,747-8,972 cycles;
   * output at 16 mod 64: another +100-250 cycles.
   Different runtimes in one process ranged 8,100-10,100 cycles on the ring row, and the two
   recorded rounds differ by 10 %. **Allocate the frozen blocks and the output planes 64-byte
   aligned** so the record measures the engine, not the allocator. Change 6 fixes the product
   side.
3. **Audit scope.** `realtime-audit` `in_render_scope` does a TLS get and set of an 80-byte state
   twice per block, plus `thread::panicking`. That is tens of cycles, and it is intended.
4. **Timer.** Roughly half of one `Instant::now` (~37 cycles) falls inside each timed interval.
5. **Not an inflation: the SHA-256 digest.** It runs outside the clock, and the bench-shaped loop
   gives the same p50 as the tight loop.

## 6. Ranked changes (class A; each measured as an in-engine prototype)

The deltas are p50 cycles on the same runtime with the flag toggled: four runtimes, two runs,
plus one run with 64-byte-aligned inputs and output. Every flag passed the 64-block digest check
against the unmodified path on both rows.

### 1. Dispatch only units that do work (skip inert source-input units)

* **What changes.** `GraphExecutor::render` (`crates/graph/src/lib.rs:2518`) iterates a bind-time
  `Box<[u32]>` of non-inert units instead of `0..units.len()`. A unit is inert when all of these
  hold:
  * it is a plain `RuntimeUnit::Op` whose kind is `NodeKind::SourceInput`;
  * `op.observers` is empty;
  * `identity[unit].observed` is false;
  * it is not the Output op.

  Units stay in `units`, so the census and `unit_eligibility` are unchanged.
* **Saving.** Ring row **-1,800 to -2,150 cycles (-0.49 to -0.58 us)**, bound row 0. Beyond this
  row, every session fed by a source set has one such unit per claimed input (claims gathered by
  banks included), so every ring-fed 64-track session saves about 2,000 cycles per block.
* **Class A.** An inert unit computes nothing: `execute_op` returns before any memory access,
  and `observe_unit` returns on `observed == false`. Selective observation (`observe_active_unit`)
  walks entries by unit index with a cursor, and a unit without observers has no entries, so
  skipping it leaves the cursor in step.
* **Rule 3 / allocation.** No arithmetic. The list is built at bind and charged to the runtime
  metadata reservation.
* **Gates.**
  * A test-only dispatched-unit counter reads 1 per block on the ring row and 65 on the bound row.
  * `the_driver_fed_plumbing_row_renders_the_bound_rows_bits` and `BASE_DIGEST` in
    `chain_shape.rs` still pass.
  * A new test shows an input-stage observer keeps its unit dispatched and fires.
  * `crates/graph/tests/rt10_source_in_place_alloc.rs` passes.

### 2. Tighten the fused Output kernel and resolve its inputs a group at a time

* **What changes.** In `route_reduce` / `route_pair` / `route_run` (`runtime.rs:714-944`):
  * resolve up to `REDUCE_GROUP` = 8 inputs at a time into a stack `[(&[f32], &[f32]); 8]`, a
    batch size and not a track cap. Arena inputs come from `read_stereo`; in-place claims come
    from one set-level call per group, a new `GraphSourcePlanes` method that loops over the
    driver's `played_planes`;
  * check every length once per group;
  * run each pair as one zip of `chunks_exact` iterators, with no per-chunk `?` and a simple
    first-pair store loop;
  * keep the odd last input at `G = 1` and keep the tail in the outlined `route_tail`.
* **Saving.** Ring row **-1,480 to -2,000 cycles (-0.40 to -0.54 us)** on top of change 1 (one
  outlier run excluded): the Output unit fell from 6,970 to about 5,280. Bound row 0 to -150 at
  p50 (Output phase -270).
* **Class A.** Unchanged per-frame chain: the first pair stores `m0 + m1`, and every later pair
  computes `(load + m_2k) + m_(2k+1)`. The operands of `mix_chunk` are unchanged. Store and
  reload move no bits.
* **Rule 3.** Real risk. The kernel is generic and is instantiated at `f32x4` on wasm. All `f32`
  tail arithmetic must stay in the non-generic `#[inline(never)]` `route_tail`, exactly as #926
  did. The prototype carries no tail and has only vector arithmetic.
* **Allocation.** None.
* **Gates.**
  * `the_plumbing_rows_output_fold_is_the_route_ops_own_bits`, which compares against the
    declined oracle.
  * A fan-in sweep (2..=20, odd and even) at a quantum that is not a multiple of 8, compared with
    the declined oracle.
  * `reduction_is_left_to_right_bit_identical_to_scalar_reference`.
  * `scripts/check-web-audioworklet-callgraph.py` rule 3 on the wasm artifact.
  * The `plumbing_profile` Output phase.
* **Rejected shape.** One stack table for all inputs measured 1,500 cycles *worse* (4 KiB of
  initialisation and forwarding stalls) and would need a compiled maximum. Groups of 8 are the
  shape to use.

### 3. Fast dispatch for plain bound units

* **What changes.** A bind-time per-unit dispatch kind. For a plain `NodeKind::Bound` op with no
  inputs, no staging, no sidechain and no split pair that is not the Output, the loop calls
  `processor.process(lease.write_stereo(op.output))` directly. It skips `Runtime::execute` and
  `execute_op`, and it skips `observe_unit` when `observed` is false.
* **Saving.** Bound row **-810 to -1,300 cycles (-0.22 to -0.35 us)** (bound phase 7,908 to
  6,626), 13-20 cycles per unit. Ring row 0. The same idea applies to other plain kinds, but
  only this one was measured.
* **Class A.** No arithmetic. The error path is unchanged: an `Err` from `process` still reaches
  the render loop's failure branch.
* **Rule 3 / allocation.** None. The table is built at bind.
* **Gates.** Digests; the failure-path tests (a processor error silences the host planes and
  completes pending splits); a dispatched-unit counter.

### 4. Batch played-plane resolution in the driver

* **What changes.** Add a provided method to `GraphPreparedSourceSetDriver`
  (`lib.rs:1787-1829`): `played_planes_many(claims, out)`. Its default implementation loops over
  `played_planes`. `crates/source` (`lib.rs:1887`) overrides it with a static loop over
  `played_plane`. Change 2's group resolution calls it once per group: one indirect call per 8
  claims instead of two per claim, and no 32-byte `Option` returned through memory per claim.
* **Saving.** Ring row **-350 to -450 cycles (-0.1 us)** on top of change 2.
* **Class A.** The same words: on `None`, the set serves the arena silence buffer, as today.
* **Rule 3.** None.
* **Allocation.** None, but the method must follow the trait's realtime rule.
* **Gates.** A source-crate test that `played_planes_many` equals `played_planes` mapped over the
  claims, including underrun and end of region; `rt10_source_in_place_alloc`; the ring-row
  digest.

### 5. Fold each bound input into the master as soon as it is written

* **What changes.** The Output op's reduction becomes a per-input epilogue that runs right after
  each bound input. Every such input writes the same L1-hot scratch buffer: the colouring ends
  its liveness at its fold. This is the plain-op analogue of `route_fold`/`FoldTarget::Output`.
  It is eligible when:
  * the bound inputs are scheduled in edge order;
  * each is read only by its retired route;
  * none is observed.
* **Saving.** Bound row: the prototype (`G = 1` folds, which include their own direct dispatch)
  measured 9,156-11,605 cycles against 11,379-12,493. That is **-700 to -2,630**. Beyond change 3
  alone it ranged from +550 to -1,450 (median about -850), and -890 to -1,450 on every runtime
  with aligned buffers. It removes the 64 KiB read-for-ownership and the arena re-read from
  L2. Folding in pairs of inputs would halve the prototype's master re-passes. Ring row 0.
* **Class A.** The same chain, `m0` then `+ m_i`, in edge order. It needs an association-order
  proof like `route_fold`'s, and a declined oracle.
* **Rule 3.** Same risk as change 2: outline the tail.
* **Allocation.** None at render. It is structural: colouring, schedule, eligibility.
* **Gates.** A `the_folded_master_is_the_reductions_own_bits`-style declined oracle; a red
  mutation that reverses the fold order; digests; a chain-shape census update.
* **Risk.** The highest of the six, and only a host-processor-fed input benefits. Brief it last.

### 6. Allocate the graph arena and source transfer blocks 64-byte aligned

* **What changes.**
  * `DisjointArena` (`crates/engine/src/realtime/disjoint.rs:75,610`): over-allocate by 15 cells
    and start the base at a 64-byte boundary. Glibc hands any allocation of 128 KiB or more
    (mmap) a pointer at 16 mod 64, so large arenas are misaligned every time.
  * `TransferBlock::try_new` (`crates/source/src/lib.rs:499`): the same, with a start offset.
  * The harness allocates the frozen blocks and the output planes the same way (question 5,
    item 2).
* **Saving.** Ring row **up to -450 cycles** when unlucky (claims at 16/48 mod 64), about 0 when
  lucky. Mostly it removes run-to-run variance. It is also the precondition for the frame-tiled
  kernel ever paying off.
* **Class A.** Placement only.
* **Rule 3.** None.
* **Allocation.** Allocation time only, plus 64 B of charge per allocation in resource
  accounting.
* **Gates.** Alignment assertions; every digest unchanged; the resource-report tests.

## 7. Not worth doing (measured)

* **Frame-tiled register-accumulator kernel** (T = 1, 2 or 4; pre-splatted table or per-tile
  broadcast). In L1 it is up to 13 % faster than pairs (2,662 vs 3,090 cycles). From L2 it wins
  only with 64-byte-aligned inputs (2,913-3,038 vs 3,160), and misaligned it loses badly
  (4,058-4,380). In the engine it was 600-2,200 cycles *worse* than the pair kernel reading the
  same pre-resolved table (flags 1|4 and 1|8 against 1|2, eight runtimes over two runs). It
  matched pairs only on the one runtime whose frozen blocks happened to sit 64-byte aligned. Its
  128 concurrent streams and its alignment sensitivity outweigh the master re-passes it removes.
  Re-measure only after change 6: at most about 250 cycles are at stake.
* **Larger hoisted groups (G = 4 or 8) and per-chunk coefficient broadcast.** Both spill or use
  load ports (PLAN table C).
* **Software prefetch or non-temporal stores.** Pairs from L2 and from L1 differ by only 0-400
  cycles, so the hardware prefetcher already covers the four-stream pattern. The output is 1 KiB
  and stays in L1.
* **Trimming the fixed per-block cost** (envelope check, observation boundary, set
  `begin_block`, audit scope). It is about 200-300 cycles, 3 % of the ring row.
* **4K-aliasing workarounds.** An output-offset sweep over 0-4000 bytes moved the pair kernel by
  under 7 %.
* **Unrolling the pair loop.** Its loop overhead is 3 of 25 instructions.
* **Refeeding `plumbing_only` through the source set, or a "lend planes" API for bound
  processors.** Either would make the bound row equal the ring row, but it changes what the row
  measures. PLAN ruling 2 keeps its feed. It is an owner question, not a change.

## 8. Realistic endpoint

**Ring row.** Changes 1+2+4 measured 4,374-4,933 cycles (1.18-1.33 us) against 8,456-9,223, and
4,447-4,632 with aligned inputs. Of what is left, about 3,100-3,700 is the pair loop and about
600-1,000 is group resolution and checks, part of which a production version can trim.
Realistically that is **~1.0-1.2 us, 50-60 % of the 0.60 us floor**.

The floor is not reachable on this Zen 3 for two measured reasons:

* the FP pipes and the L2 fill port are both saturated at 2,048 cycles, so perfect overlap is
  the only way to hit the floor;
* the kernels sustain about 4.5 instructions per cycle, which puts even the best measured kernel
  at about 2,700 cycles from L1 and about 2,900 from L2.

**Bound row.** Changes 3+2 measured 10,192-11,194 cycles, and 5 brings it to about 9,200-10,500:
**~2.5-2.8 us**. Its floor includes the harness processor's 64 KiB write per block and 64
indirect calls, so it cannot approach 0.60 us without changing its feed.

**In the ruling's terms (`percent_of_floor`, against 2,214 cycles).** Today the recorded runs
stand at about 18 % (bound, 3.336 us) and 20-22 % (ring, 2.915 / 3.216 us). After these changes
the bound row would be about 22-24 % and the ring row about 50-55 %.
