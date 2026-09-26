# Plumbing-floor cycle plan (2026-09-26)

Owner goal, verbatim: *"Let's work on the no-effects plumbing. The goal is still to get that to
theoretical floor."* The row is `sixty_four_track_plumbing_only` (`tools/bench/src/console.rs`;
`tools/console-workload/src/lib.rs:242`). This cycle started from a **measured** breakdown, not a
reading of the code; the last cycle's #920 and #923 are what reading alone produced.

Base: `main` at `97435208` (the copy-removal batch #914-#919 without #920). Every `file:line` in
the drafts was verified on that commit. Baseline for attribution: `05e91981` (`main` before PR #922).

Host: AMD EPYC 7313P (Zen 3), Linux 6.8, rustc 1.97.1, `x86-64-v3` (`+avx2,+fma`), cpu 31 pinned,
loadavg 1.1-1.6 during every run. `perf` is unavailable (`perf_event_paranoid` 4, no sudo), so
every cycle figure here is **derived**: nanoseconds from the vDSO monotonic clock times a core
clock calibrated in-process from a dependent chain of scalar `f32` adds at the Zen 3 `vaddss`
latency of 3 cycles (3.698-3.700 GHz on every run, against the 3.73 GHz nameplate boost). The
instrument and the harness are in this branch:

- `crates/graph/src/lib.rs` `test_only_phase_profile` (behind `cfg(any(test, feature =
  "test-support"))`; a production build carries none of it; a `test-support` build that never
  enables it pays one thread-local read per block) and `Runtime::test_only_unit_phase`.
- `tools/console-workload/tests/plumbing_profile.rs`: `phase_profile` (the real plan, probes on
  and off) and `kernel_replicas` (each phase's kernel standalone over the row's own working set,
  plus the fused one-pass candidates, all asserted bit-identical). Run:
  `CARGO_INCREMENTAL=0 taskset -c 31 cargo test --release -p console-workload --test
  plumbing_profile -- --ignored --nocapture --test-threads 1`.

## Part 1 -- the floor, derived

The row renders 64 tracks x 128 frames x 2 channels = 16,384 lane-samples per block. The frozen
arithmetic per lane-sample is the plumbing inventory of `docs/rulings/effect-floor-accounting.md`
("Plumbing inventory"): the route `mix2x2` (`mul` + unfused `fma` = 3 lane-ops per channel; the
release binary confirms it is `vmulps`, `vmulps`, `vaddps` -- `Lane::fma` is unfused by contract so
wasm and native agree) and one `add` per lane-sample of the 64-input reduction: **4 lane-ops**.

| term | derivation | cycles | us at 3.70 GHz |
|---|---|---:|---:|
| (a) arithmetic | 4 x 16,384 = 65,536 lane-ops / (8 lanes x 3.7 vector ops per cycle, `tools/bench/src/floor.rs:50,59`) | **2,214** | 0.598 |
| (a') Zen 3 pipe cross-check | 8,192 vector ops per block: 4,096 `vmulps` on the two FMA pipes (2,048 cycles) beside 4,096 `vaddps` on the two FADD pipes (2,048 cycles) | 2,048 | 0.554 |
| (b) memory | 64 KiB of source planes read once = 2,048 x 32-byte loads at 2 per cycle; the working set (64 KiB of sources, plus 66 KiB of arena on today's plan) exceeds the 32 KiB L1D, so the sources stream from the 512 KiB L2 at one line per cycle: 1,024 lines. Sequential streams prefetch, so this **overlaps** (a) rather than adding to it. The 1 KiB host write is 32 stores. | 1,024 (overlapped) | 0.28 |
| (c) fixed per block | measured `enter` phase (host-plane shape check, `begin_observation_block`, two flags): 107 cycles; the `PreparedRenderPlan::render_inner` wrapper and `audit::in_render_scope` are outside the probes and bounded by the probes-off/probes-on difference at under 100 ns; in production the C-ABI entry adds the MXCSR pin (`stmxcsr`/`ldmxcsr`), tens of cycles, absent from this row | ~150-300 | 0.04-0.08 |
| **floor** | (a) + (c), (b) overlapped | **~2,400** | **~0.65** |

The ruling's 3.7 ops per cycle was probed on the Zen 5 host, not on this Zen 3; the pipe
cross-check (a') says the Zen 3 figure would be within 8 % of it, so 2,214 stands as the number
(sensitivity: every floor here scales with it).

**The one-pass design that reaches it:** read each track's two planes once, apply its 2x2 in
registers, accumulate into the master in edge order -- the class-A order `mix2x2_block` then
`reduce` today, i.e. `value = mix(in0)` for the first contributor, `value = value + mix(in_i)` after
-- store the master, and emit **no per-track unit and no per-track copy**. Measured as a replica
over the row's own working set (Part 2, table C), the best class-A shape of that pass is
**3,450 cycles (0.93 us) = 64 % of floor**; the residual is the master re-passes a two-track
group costs and the loop overhead, both accounted below.

**Standing:** the row measures 37,505 cycles (10.14 us) on this host: **5.9 % of floor**, 15.9x
the floor, matching the ruling's 6.7 % order. `perf` would have replaced the calibrated clock
with counted cycles and given IPC and L1D/L2 miss counts per phase; nothing below depends on it.

## Part 2 -- the measured breakdown

### A. Phases of one block, current tree (`97435208`)

Probes off, p50 of 4,000 blocks, three repeats: **10,140 / 10,140 / 10,160 ns** (min 9,929-9,949,
p95 10,289-10,309) = 37,505-37,579 cycles = 2.289-2.294 cycles per lane-sample. A previous build
of the same tree with only test-only code differing measured 10,571-10,590 ns: **build-to-build
layout variance on this row is about 4 %**, which matters for reading #923.

Probes on (8 probes per block at 20 ns each; the per-unit kind lookup adds ~0.5 us to the
instrumented total, charged to the phases proportionally), repeat 0; repeats 1 and 2 agree within
0.5 % on every phase:

| phase | units | ns/block | cycles/block | share | per unit (cycles) |
|---|---:|---:|---:|---:|---:|
| enter (shape check, `begin_observation_block`) | -- | 28.8 | 107 | 0.3 % | -- |
| source set + loop entry (no source set on this row) | -- | 53.6 | 198 | 0.5 % | -- |
| bound units: `FrozenGraphSource::process` (two 512-byte `memcpy`) | 64 | 2,170 | 8,025 | 19.9 % | 125 |
| identity-copy units: `PostInputBuiltins` (dedicated) and `PostFader` (reads a dedicated buffer), `reduce_plane`'s copy arm | 128 | 4,470 | 16,534 | **40.9 %** | 129 |
| identity-alias units: `PostMatrix`, in place, dispatch only | 64 | 1,098 | 4,060 | 10.1 % | 63 |
| route units: `mix2x2_block::<Simd8>` in place | 64 | 2,177 | 8,052 | 19.9 % | 126 |
| output unit: `reduce_many_into`, eight groups of eight into the host planes | 1 | 922 | 3,409 | 8.4 % | 3,409 |
| exit | -- | 0 | 0 | 0 % | -- |

Unit schedule (from the probe's run record): `64 x bound, 128 x identity-copy, 64 x
identity-alias, 64 x route, 1 x output` = 321 units. The row's lowered chain per track is
therefore **Input (bound copy) -> PostInputBuiltins (copy) -> PostFader (copy) -> PostMatrix
(no-op) -> Route (in place)**, not "source -> route -> Output". The three builtin stages are
`required_bindings` of the builtins-less compile (`crates/graph-compiler/src/compile.rs:803-816`),
bound through `GraphNodeBinding::identity` by the workload (`tools/console-workload/src/lib.rs:1620`),
and lowered as ops because only `PostSimd1 | PostDynamic | PostSimd2PreFader` are alias candidates
(`crates/graph/src/program.rs:186-194`); `PostInputBuiltins` is dedicated by kind
(`program.rs:226-231`), which forces its copy and its consumer's.

Every plain unit pays about **60-85 cycles of dispatch** on top of its kernel: bound 125 = 66
(the copy, table C) + ~60; identity-copy 129 = 66 + ~63; identity-alias 63 = 0 + 63; route 126 =
43 + ~83; output 3,409 = 2,700 (table C) + ~700. Dispatch is 321 units x ~65 = ~21,000 cycles,
**56 % of the block**; copies of every track's block three times over (bound, then two
identity copies: 192 KiB per block) are ~12,700 cycles, 34 %; the four lane-ops of arithmetic
are inside the remaining 10 %.

### B. Baseline (`05e91981`) and the #923 attribution

Same harness, same core, same session, three repeats each:

| phase | baseline cycles | current cycles | delta |
|---|---:|---:|---:|
| enter | 99 | 107-112 | +10 |
| source set + loop entry | 127 | 198 | +70 |
| bound (64) | 7,711-7,766 | 8,025-8,052 | +300 |
| identity-copy (128) | 16,273-16,291 | 16,495-16,534 | +220 |
| identity-alias (64) | 3,670-3,687 | 4,035-4,060 | +370 |
| route (64) | 8,435-8,485 | 8,011-8,052 | -430 |
| output (1) | 3,588-3,595 | 3,404-3,413 | -185 |
| exit (the end-of-block master copy, #916 removed it) | 180-191 | 0 | -185 |
| **probes off, p50** | **37,969-38,187** (10.27-10.33 us) | **37,505-37,579** (10.14-10.16 us) | **-1.2 %** |

**#923 is not reproduced.** In-process, on the same core, the batch tree renders the row 1.2 %
*faster* than the baseline, and every phase moves by less than the 4 % layout variance measured
above. #916's reduction into the host planes is 185 cycles cheaper than the arena reduction it
replaced and the 185-cycle exit copy is gone; #916's `HostMaster` threading (the
`output_unit == Some(index)` test and `host.reborrow()` per unit, `runtime.rs:2374-2416`) costs
about 3 cycles per unit, +900 over 321 units, which is what the +220/+370/+300 rows are, and
draft A removes 192 of those units. The +12 % in `artifacts/copy-removal-without-920/` is a
single uncontrolled run of a different binary against another single uncontrolled run
(#923 itself states 6 % run-to-run noise). Disposition proposed for #923: re-run its arms once
under the runner's controlled preconditions; if flat, close as *not reproduced* citing this table;
no correction to #916 is warranted by any evidence here. (#920's kernel was a real defect and is
superseded by draft B.)

### C. Kernel replicas (current tree, cycles per block, min of 30 batches x 200, three repeats)

The row's working set exactly (64 frozen stereo blocks, a plane-major 66-buffer arena, two host
planes), with the engine's own `mix2x2_block::<Simd8>` and a verbatim copy of
`accumulate_run::<Simd8, 8>`:

| replica | cycles | cycles/lane-sample |
|---|---:|---:|
| copy phase: 64 x two 128-word `copy_from_slice` | 4,224 | 0.258 |
| route phase: 64 x `mix2x2_block::<Simd8>` in place | 2,712-2,781 | 0.167 |
| reduce phase: eight groups of eight, both planes, `accumulate_run` shape | 2,694-2,700 | 0.165 |
| copy + route + reduce | 9,738 | 0.594 |

The fused one-pass candidates, every one asserted **bit-identical** to route-then-reduce (from the
arena after the copy, and straight from the sources with no copy):

| group G | coefficients | from the arena | from the sources (no copy) |
|---:|---|---:|---:|
| 1 | hoisted (4 splats) | 3,752-3,788 | 3,763-3,790 |
| **2** | **hoisted (8 splats)** | **3,492-3,504** | **3,451-3,453** |
| 4 | hoisted (16 splats) | 4,405-4,416 | 4,121-4,139 |
| 8 (#920's `route_run::<L, 8>` shape) | hoisted (32 splats) | 5,270-5,301 (4,502-4,552 on a second build) | 4,965-4,985 (4,522 on a second build) |
| 1 | broadcast per chunk | 5,594-5,619 | 5,483-5,598 |
| 2 | broadcast per chunk | 4,612-4,615 | 4,478-4,561 |
| 4 | broadcast per chunk | 5,045-5,160 | 4,795-4,865 |
| 8 | broadcast per chunk | 7,930-7,951 | 7,766 |

Reading: on AVX2 a coefficient broadcast is a load-port op per chunk, so broadcasting loses to
hoisting at every G; hoisting more than two tracks' 2x2s (8 x G registers of 16) spills, so G = 8
is the worst hoisted shape on both builds measured and G = 2 the best on both (a rebuild after a
clippy fix moved G = 8 by 10 % and G = 2 by under 3 %: the spill shape is the layout-sensitive one). G = 2 hoisted beats today's route + output
(8,052 + 3,409 = 11,461) by 8,000 cycles and stands at 64 % of floor. Reading from the sources
instead of the arena saves nothing in the kernel (both are L2 streams); what it saves is the copy
that put the words in the arena (4,224 cycles plus 64 dispatches).

### D. Static instruction counts (release test binary, `objdump -d`)

| function | instructions | calls | note |
|---|---:|---:|---|
| `GraphExecutor::render` (with `Runtime::execute` and `observe_unit` inlined) | 1,602 | 53 | the unit loop is 46 instructions + one `call execute_op` per unit |
| `graph::runtime::execute_op` | 1,246 | 46 | 16 `panic_bounds_check`, 5 `slice_index_fail`; `reduce_plane` (x2), `reduce_plane_into` (x2) and `output_planes` are **out of line** |
| `execute_op` mix2x2 loop body | 12 per 8 frames | -- | 2 loads, 4 `vmulps`, 2 `vaddps`, 2 stores, 3 loop: the Route path's 16 iterations are ~43 cycles |
| `graph::runtime::reduce_plane_into` | 2,567 | 23 | all eight `N` specialisations inlined; the per-group `match group.len()` is an 86-113-instruction jump-table loop |
| `reduce_plane_into` N=8 group loop body | 12 per 8 frames | -- | 1 load, 7 `vaddps` with memory operands, 1 store, 3 loop: load-port bound at 4 cycles per chunk, 256 chunks |
| `graph::runtime::reduce_plane` | 3,560 | 4 | the arena twin, called twice per op that has inputs |
| `FrozenGraphSource::process` | 35 | 4 | two 512-byte `memcpy` calls |
| `fused_route_reduce::<2, true>` (harness) | 138 | 4 | 16 vector ops, 8 `vbroadcastss` outside the chunk loop |
| `fused_route_reduce::<8, true>` (harness, #920's shape) | 369 | 4 | 64 vector ops, 32 `vbroadcastss`: the spill shape |

## Part 3 -- the plan

Three class-A drafts, in the order the measurement ranks them; each states the phases it removes,
never a projected number.

| # | draft | removes | rows it can move | needs |
|---|---|---|---|---|
| A | `lower-identity-bound-track-stages-as-aliases.md` | the 128 identity-copy units and the 64 identity-alias units: 20,600 cycles, 55 % of the block; the row's chain becomes Input -> Route (in place over the Input's buffer) -> Output | `sixty_four_track_plumbing_only` only | **owner ruling** (a lowering shape) |
| B | `fuse-in-place-routes-into-the-output-reduction-in-pairs.md` | the 64 route units and the output unit's grouped reduction (11,460 cycles) for one fused pass measured at 3,450 | `sixty_four_track_plumbing_only` only | after A (convenient, not forced); supersedes #920 |
| C | `read-plain-strip-sources-in-place-from-the-played-transfer-block.md` | the source copy of every ring-fed plain-strip claim (the bench's equivalent: 4,224 cycles of copy plus 64 dispatches) | none today: the row binds `FrozenGraphSource` processors | after B; **owner question** on feeding the row through a source driver |

After A + B on the bench row as it is fed today: 64 bound units (~8,000) + the fused pass
(~3,450) + fixed (~300) = ~11,800 cycles, 3.2 us, 19 % of floor. After C on a driver-fed row: the
fused pass + fixed, ~3,750 cycles, 1.0 us, 59 % of floor. The rest of the gap to 2,400 is inside
the kernel (the 32 master re-passes of G = 2 and its loop overhead) and is the next measurement,
not a brief.

### Considered and not briefed

- **Per-unit dispatch (~65 cycles per plain unit).** After A the row has 129 units, after C on a
  driver-fed row it has one. The static counts name where the 65 cycles go (a 46-instruction
  loop, `execute_op`'s prologue over a 1,246-instruction body, two out-of-line `reduce_plane`
  calls for every op with inputs, `output_planes` out of line, the indirect `process` call), but
  a dispatch trim would optimise a path the three drafts empty; re-measure after them.
- **The `bound` units on the bench row.** Their copy *is* the processor contract
  (`GraphRuntimeProcessor::process` is write-only into the block it is handed), so "in place" has
  no meaning for a bound processor. The production feed is the source set, and C is its in-place
  form; whether the row should be driver-fed is the owner question under C.
- **A trivial bank for unbanked strips.** Declined last cycle for adding two transposes per track;
  table C confirms the frame-major fused pass is already at 64 % of floor without any transpose.
- **Fixed per-block cost.** 107 cycles measured; nothing in it is removable (shape check,
  observation boundary, two flags).
- **#920's kernel.** Superseded by B: G = 8 hoisted is the measured worst hoisted shape (table C)
  and the callgraph gate's rejection of its wasm instantiation is the same spill seen from the
  other side.

### What needs an owner ruling

1. **Draft A's lowering shape.** A builtins-less plan's `PostInputBuiltins`, `PostFader` and
   `PostMatrix` stages emit no op at all (they become aliases like the three rack boundaries) and
   are no longer `required_bindings` of `GraphCompiler::compile`. The alternative that needs no
   ruling -- keep the three ops but stop dedicating an identity-bound `PostInputBuiltins`, so all
   three run in place -- removes the two copies (about 8,400 cycles) and keeps the 192 dispatches
   (about 12,200). The question is whether a host may still bind its own processor to a builtin
   stage of a builtins-less plan; A's option 2 keeps that door open at the price of bind-time
   lowering. Both options are in the draft.
2. **Draft C's benchmark fidelity.** `sixty_four_track_plumbing_only` binds bound processors, so
   neither #918 nor C can ever show on it. Feeding the row through a `GraphSourceDriver` that
   offers played planes would make the row measure the production path and lets C move it; it
   changes what the row is. If the owner wants it, it is a one-line tooling brief on
   `tools/console-workload/src/lib.rs` (a `FrozenSourceDriver` beside `FrozenGraphSource`) with
   the digest pinned equal across the two feeds.

### Benchmark plan

One paired `run-console-benchmark.sh` invocation per merged draft is not needed: A, B and C are
one batch on the plumbing row. Register a `--plumbing-floor` / `--plumbing-floor-baseline` arm
pair beside `--copy-removal` and run once at the batch boundary, controlled if the host allows;
the in-process harness above is the exploration tool until then and must never be quoted as the
row's number. Every `output_sha256` must equal the baseline arm's (the row's digest today:
`38ebb48908b62b9770ac7df1a8f6f2427bfd15eb849429219f8705a3926084c3` on all three copy-removal arms).

### What `perf` would have added

Counted cycles instead of a calibrated clock (the 3-cycle `vaddss` latency is an assumption the
3.70 GHz result makes plausible, not a measurement); IPC per phase, which would say directly
whether the 65-cycle dispatch is front-end or dependency bound; L1D/L2 miss counts to confirm the
sources stream from L2 and the master stays in L1; and `perf record` attribution *inside*
`execute_op`, which the phase probes cannot split. None of the three drafts depends on any of it.
