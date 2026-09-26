# Plumbing-floor cycle plan (2026-09-26)

Owner goal, verbatim: *"Let's work on the no-effects plumbing. The goal is still to get that to
theoretical floor."* The row is `sixty_four_track_plumbing_only` (`tools/bench/src/console.rs`;
`tools/console-workload/src/lib.rs:242`). This cycle started from a **measured** breakdown, not a
reading of the code; the last cycle's #920 and #923 are what reading alone produced.

Base: `main` at `52971510` (PR #924, which adds the `--copy-removal*` arms and the
`artifacts/copy-removal-*` records cited below on top of `97435208`, the copy-removal batch
#914-#919 without #920; #924's code diff is 26 lines in `scripts/run-console-benchmark.sh`, so no
anchor moved). Every `file:line` in the drafts was verified on that tree. Baseline for
attribution: `05e91981` (`main` before PR #922).

Host: AMD EPYC 7313P (Zen 3), Linux 6.8, rustc 1.97.1, `x86-64-v3` (`+avx2,+fma`), cpu 31 pinned,
loadavg 1.1-1.6 during every run (3.7 GHz nameplate boost). `perf` is unavailable (`perf_event_paranoid` 4, no sudo), so
every cycle figure here is **derived**: nanoseconds from the vDSO monotonic clock times a core
clock calibrated in-process from a dependent chain of scalar `f32` adds at the Zen 3 `vaddss`
latency of 3 cycles (3.698-3.700 GHz on every run, against the 3.7 GHz nameplate boost). The
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
| **floor plus fixed** | (a) + (c), (b) overlapped | **~2,400** | **~0.65** |

The **floor** every percent-of-floor below is measured against is the ruling's arithmetic term
(a), **2,214 cycles**, as `floor.rs` states it; the ~2,400 figure is the floor plus the fixed cost
no design removes.

The ruling's 3.7 ops per cycle was probed on the Zen 5 host, not on this Zen 3; the pipe
cross-check (a') says the Zen 3 figure would be within 8 % of it, so 2,214 stands as the number
(sensitivity: every floor here scales with it).

**The one-pass design that reaches it:** read each track's two planes once, apply its 2x2 in
registers, accumulate into the master in edge order -- the class-A order `mix2x2_block` then
`reduce` today, i.e. `value = mix(in0)` for the first contributor, `value = value + mix(in_i)` after
-- store the master, and emit **no per-track unit and no per-track copy**. Measured as a replica
over the row's own working set (Part 2, table C), the best class-A shape of that pass is
**3,450 cycles (0.93 us) = 64 % of the 2,214 floor**; the residual is the master re-passes a
two-track group costs and the loop overhead, both accounted below.

**Standing:** the row measures 37,505 cycles (10.14 us) on this host: **5.9 % of the 2,214
floor**, 16.9x the floor, matching the ruling's 6.7 % order. `perf` would have replaced the calibrated clock
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

The probed phases sum to about 740 ns above the probes-off p50 (the probes and the per-unit kind
lookup), about 8.5 cycles per unit, so the per-unit figures above overstate dispatch by that much.
Net, every plain unit pays about **50-75 cycles of dispatch** on top of its kernel: bound 125 -
8.5 = 66 (the copy, table C) + ~50; identity-copy 129 - 8.5 = 66 + ~55; identity-alias 63 - 8.5
= 0 + ~55; route 126 - 8.5 = 43 + ~75; output 3,409 = 2,700 (table C) + ~700. Dispatch is 321
units x ~57 = **~18,300 cycles, 49 % of the block**; copies of every track's block three times
over (bound, then two identity copies: 192 KiB per block) are ~12,700 cycles, 34 %; the four
lane-ops of arithmetic are inside the remaining 17 % with the kernels' own loop overhead.

### B. Baseline (`05e91981`) and the #923 attribution

Same harness, same core, same session, three repeats each:

The baseline port of the instrument is the patch
`docs/handoffs/plumbing-floor-2026-09-26/baseline-05e91981-instrument.patch` (applies to
`05e91981` with `git apply`; the harness file is the branch's, unchanged).

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
| **probes off, p50** | **37,969-38,187** (10.27-10.33 us) | **37,505-37,579** (10.14-10.16 us) | flat within noise |

The reviewer's independent re-run of the same patch agrees: baseline probes off 37,782-38,377
cycles against current 37,592-37,737; identity-alias phase +400 (the `HostMaster` threading);
output plus exit -250 to -300; net **flat within noise**.

**#923 is not reproduced.** In-process, on the same core, the batch tree and the baseline render
the row within the 4 % build-to-build layout variance measured above, and every phase moves by
less than that. #916's reduction into the host planes is 185 cycles cheaper than the arena
reduction it replaced and the 185-cycle exit copy is gone; #916's `HostMaster` threading (the
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
(8,052 + 3,409 = 11,461) by 8,000 cycles and stands at 64 % of the 2,214 floor. Reading from the sources
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

Three class-A drafts in the order the measurement ranks them, and one tooling draft; each states
the phases it removes, never a projected number. Coordinator rulings (final): A takes option 1,
keyed on `required_bindings` at compile time; `sixty_four_track_plumbing_only` is **not** refed,
and a new row `sixty_four_track_plumbing_ring` (D) is the row C can move.

| # | draft | removes | rows it can move | order |
|---|---|---|---|---|
| A | `lower-identity-bound-track-stages-as-aliases.md` | the 128 identity-copy units and the 64 identity-alias units: 20,600 cycles, 55 % of the block; the row's chain becomes Input -> Route (in place over the Input's buffer) -> Output | `sixty_four_track_plumbing_only` (and D's row) | first |
| B | `fuse-in-place-routes-into-the-output-reduction-in-pairs.md` | the 64 route units and the output unit's grouped reduction (11,460 cycles) for one fused pass measured at 3,450 | `sixty_four_track_plumbing_only` (and D's row) | after A: not forced (its eligibility holds on today's shape, as #920 showed), but the cheaper rebase; supersedes #920 |
| C | `read-plain-strip-sources-in-place-from-the-played-transfer-block.md` | the source copy of every ring-fed plain-strip claim (`copy_track_input` per claim; the bench-row equivalent is 4,224 cycles of copy plus 64 dispatches) | `sixty_four_track_plumbing_ring` only | after B (**forced**: B's fused kernel is C's reader) |
| D | `add-a-driver-fed-plumbing-row-to-the-console-benchmark.md` | nothing: tooling. Adds `sixty_four_track_plumbing_ring`, the same session fed through a played-planes driver, digest pinned equal to the bound-feed row | adds a row | independent; run with A |

**Concurrency groups.** A, B and D may run in parallel worktrees. Files: A touches
`crates/graph-compiler/src/compile.rs`, `crates/graph/src/program.rs` (the alias predicate and
`lower`'s signature), `crates/graph/src/lib.rs` (`lower_from_current_fields`), `program/tests.rs`,
plus the three doc/comment corrections it lists. B touches `crates/graph/src/runtime.rs` (kernel,
bind-time fold) and `crates/graph/src/lib.rs` (seam export, accessor) and the floor ruling's
sentence. Their one shared file is `crates/graph/src/lib.rs`, on disjoint lines (`:1195` against the
`test_only_*` re-export block and the executor accessor); neither touches `program.rs`'s lowering
passes in the other's region (B does not edit `program.rs` at all). D touches `tools/bench/src/
console.rs`, `tools/console-workload/src/lib.rs` (driver and builder), `tools/bench/src/floor.rs`
and the validator scripts: no overlap with A or B. C follows B on `runtime.rs` and is
sequential with it.

After A + B on the bound-feed row: 64 bound units (~8,000) + the fused pass (~3,450) + fixed
(~300) = ~11,800 cycles, 3.2 us, 19 % of the 2,214 floor. After C on the driver-fed row D adds:
the fused pass + fixed, ~3,750 cycles, 1.0 us, 59 % of floor. The rest of the gap to the floor
is inside the kernel (the 32 master re-passes of G = 2 and its loop overhead) and is the next
measurement, not a brief.

### Considered and not briefed

- **Per-unit dispatch (~57 cycles net per plain unit).** After A the row has 129 units, after C on
  the driver-fed row it has one. The static counts name where the cycles go (a 46-instruction
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
- **#920's kernel.** Superseded by B on two separate findings. Native: G = 8 hoisted is the
  measured worst hoisted shape (table C). Wasm: the callgraph gate that rejected it is **rule 3**
  (`scripts/check-web-audioworklet-callgraph.py:352-359`: every function matching `4wide6f32x[48]`
  that carries `f32x4` arithmetic must have vector > scalar), and the cause is not a spill (a
  spill emits no `f32.mul`/`f32.add`): LLVM fully unrolls the width-4 `f32` tail written "by the
  same body" inside the vector instantiation, three scalar ops per vector op. Reviewer's scratch
  simd128 builds: the pair kernel with a same-body tail 30 vector / 90 scalar (fails); #920's
  group of eight 168 / 624 (its 560 / 1,680 on the artifact was this); the pair kernel with the
  tail outlined into a non-generic `#[inline(never)]` function 30 / 0 (passes). B's kernel
  outlines the tail.

### Rulings taken (coordinator, final)

1. **Draft A's lowering shape: option 1**, keyed on `required_bindings` at compile time, no new
   struct field. A builtins-less plan's `PostInputBuiltins`, `PostFader` and `PostMatrix` stages
   are dropped from `required_bindings` by `compile` and lowered as aliases; a plan that lists a
   stage keeps its op. The reasons are recorded in the draft.
2. **Draft C's benchmark fidelity: do not refeed `sixty_four_track_plumbing_only`.** `floor.rs`
   asserts it is the floor of the whole table and the paired arms compare it across commits. Draft
   D adds `sixty_four_track_plumbing_ring` beside it, the same session fed through a
   `FrozenSourceDriver` that lends played planes (the shape of the rt10 `Played` driver,
   `crates/graph/tests/rt10_source_in_place_alloc.rs:38-110`: `claim_count`, `begin_block`,
   `copy_track_input`, `provides_played_planes`, `played_planes`, about 70 lines) bound through
   `GraphPreparedSourceSet::new` (`crates/graph/src/lib.rs:1843`) and `into_bound_with_source_set`
   (call site `crates/host-core/src/prepare.rs:1405`); it is the only row C can move.

### Benchmark plan

One paired `run-console-benchmark.sh` invocation per merged draft is not needed: A, B, C and D
are one batch on the two plumbing rows. Register a `--plumbing-floor` / `--plumbing-floor-baseline`
arm pair beside `--copy-removal` (`scripts/run-console-benchmark.sh`, on `main` since #924) and
run once at the batch boundary, controlled if the host allows;
the in-process harness above is the exploration tool until then and must never be quoted as the
row's number. Every `output_sha256` must equal the baseline arm's (the row's digest today:
`38ebb48908b62b9770ac7df1a8f6f2427bfd15eb849429219f8705a3926084c3` on all three copy-removal arms).

### What `perf` would have added

Counted cycles instead of a calibrated clock (the 3-cycle `vaddss` latency is an assumption the
3.70 GHz result makes plausible, not a measurement); IPC per phase, which would say directly
whether the 65-cycle dispatch is front-end or dependency bound; L1D/L2 miss counts to confirm the
sources stream from L2 and the master stays in L1; and `perf record` attribution *inside*
`execute_op`, which the phase probes cannot split. None of the three drafts depends on any of it.
