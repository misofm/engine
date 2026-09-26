# DIAGNOSIS-2 adversarial verification: evidence

> Adversarial verification of `DIAGNOSIS-2.md`, run 2026-09-26 on Opus 5.5. The scratch builds it
> names (`v2base`, `v2c1`, `v2c12`, `v2c124`, `diagbench`, `xcore/xcore.c`) lived in a session
> scratchpad and are not in the repository; the numbers and file:line evidence are.

Subject: `docs/handoffs/plumbing-floor-2026-09-26/DIAGNOSIS-2.md` at `d62d9d44` (base `14f2917b`).
Host: EPYC 7313P (Zen 3), cpu 31 pinned, three other agents building (loadavg 2.5-4.8), rustc 1.97.1.
All work in the scratchpad. Nothing was pushed. `main` and the diagnosis worktree were not touched.

## What I built (the production shape, not the test harness)

The diagnosis measured its prototypes inside a `cargo test --release` harness. That harness uses
`panic=unwind`, `graph/test-support` (TLS source-plane counters on every Output input, a probe test
per unit), and per-unit experiment-flag branches on the baseline path. To remove all three, I
wrote `tools/diagbench`, a `[[bin]]` built with the shipped release profile (fat LTO, CGU 1,
`panic=abort`) and without `test-support` unless `--features ts`. Each variant is a
`git archive 14f2917b` copy plus the diagnosis patch's harness-only alignment hunks
(`DIAG2_CLAIM_MOD64`/`DIAG2_OUT_MOD64`, without `diag_played_planes_into`):

| dir | contents |
|---|---|
| `v2base/` | unmodified engine |
| `v2c1/` | change 1: bind-time `active_units: Box<[u32]>` in `GraphExecutor` (both layout mirrors updated), `Runtime::unit_inert` = the diagnosis predicate, render loop `for &unit in active_units.iter()` |
| `v2c12/` | + change 2: groups of 8 resolved into a stack `[(&[f32], &[f32]); 8]`, lengths checked once per group, a generic `#[inline(never)] route_group<L>` (the prototype's tight zip-of-`chunks_exact` kernel), and the f32 tail still in `route_tail`, called per pair after each group's vector frames |
| `v2c124/` | + change 4: a provided `played_planes_into` on `GraphPreparedSourceSetDriver`, one dyn call per group. Test counters restored |

Every variant renders the same 64-block digest on both rows (`57535244ba953d82…`).

## 1. Clean A/B, ring row (p50 cycles; median over 4 runtimes of min-of-3 p50; output at 0 mod 64; two runs)

| claims mod 64 | base | change 1 | 1+2 | 1+2+4 |
|---:|---:|---:|---:|---:|
| 0 | 8,199 / 8,187 | 6,447 / 6,415 | 4,960 / 4,850 | 4,776-4,820 |
| 16 | 8,556 / 8,743 | 6,742 / 6,927 | 5,369 / 5,335 | 5,151-5,264 |
| 32 | 8,188 / 8,230 | 6,339 / 6,489 | 4,783 / 4,782 | 4,704-4,742 |
| 48 | 8,708 / 8,820 | 7,078 / 7,112 | 5,520 / 5,409 | 5,411-5,556 |

| change | measured here (production shape) | DIAGNOSIS-2 claim |
|---|---|---|
| 1 skip inert units | **-1,630 to -1,850** | -1,800 to -2,150 |
| 2 groups + tight kernel | **-1,370 to -1,710** | -1,480 to -2,000 |
| 4 batched driver call | **about -75 (range +147 to -147)** | -350 to -450 |
| 1+2+4 endpoint | **4,704-5,556 cycles = 1.27-1.50 us** | 4,374-4,933 = 1.18-1.33 us; "realistic 1.0-1.2" |

The direction is confirmed and the magnitudes are 10-20 % smaller for changes 1 and 2. Change 4
is about a fifth of the claim. The prototype's extra saving is consistent with three things its
baseline paid and the flagged paths did not: the TLS counters, four flag branches per unit, and the
unwind landing pads of a test harness. Raw output: `v2-run-aligned0.txt`, `v2-run-unpinned.txt`,
`v2-run-c124.txt`, `v2-run-c12.txt`.

**`graph/test-support` in the bench (claim d).** Base ring row: `prod` 8,151-8,193 against `ts`
8,232-8,262, so +40-110 cycles (about 1 %). The bound row is within its noise. In a scratch copy
with `tools/bench/Cargo.toml:31` changed to `graph.workspace = true`, `cargo check --release -p
bench --all-targets` passes and `cargo tree -e features -i graph` shows only `default`. Claim (d)
is **confirmed**.

**Heap alignment (claim e).** With the claims pinned, the base ring row moves 8,188→8,820
(peak to peak about 630 cycles, ±0.085 us). Unpinned, one process's 8 runtimes span 8,007-9,862.
That spread is about 3x what mod-64 alignment explains. Claim (e) holds as stated, but aligning to
64 bytes will not remove most of the recorded variance.

## 2. Class A review (read, then tested)

**Change 1.** Read: `Runtime::execute` (runtime.rs:2807-2940) has no side effect for a plain op
beyond `execute_op`. `execute_op` returns at runtime.rs:3361 for `SourceInput`, and `TrackDelay` is
a separate kind handled before that line, so PDC-delayed claims are *not* inert. `observe_unit`
returns on `!identity.observed` (3033). The `#[cfg(test)]` skip-disabled override only affects
tests that dispatch source inputs, and the #900 test binds bound inputs. Activation entries are
emitted once per observer binding (runtime.rs:4692-4704) and resolved through `op.observers`
(3207-3217), so a unit with no observers has no cursor entry. Resident input uses
`before.last()` by index and needs a `Bank` predecessor (2830-2840), so skipping an op cannot
change adjacency. `complete_pending` and invalidation walk every unit anyway. Meters are observer
bindings, and alias taps are folded into `op.observers` at bind.
**Class A: yes.**

Tested on `v2c1`: every console-workload test passes (`chain_shape` `BASE_DIGEST`,
`the_driver_fed_plumbing_row_renders_the_bound_rows_bits`), and so do rt10, rt1 and rt9 alloc.
**One red:** `rt9_resident_entry_has_one_guarded_production_caller_and_control`. It splits the
render source on the literal `for unit in 0..runtime.units.len() {` (runtime.rs:7946) and fails
on any change to the loop header. Change 3 (`render.matches("runtime.execute(").count() == 1`)
and change 5 hit the same gate.

**Change 2.** The prototype and my production version keep the chain exactly. The first pair
stores `m0.add(m1)`. Every later pair computes `load.add(m2k).add(m2k+1)`, with the same left
operand as `add_mixed_chunks`. The odd last input computes `load.add(m)`. `mix_chunk`
(runtime.rs:938) is unchanged, so the unfused `(lr*r)+(ll*l)` operand order holds. The group table
is a batch size looped over, with no track cap and no allocation. The rejected one-table form did
need `inputs.len() <= 128`.

Tested on `v2c124`: `assert_route_reduce_is_the_route_ops_and_the_reduction` (runtime.rs:14143)
already sweeps frames {1,3,7,8,13,16,33,64} × fan-in 2..=19 plus 64 × signed zero against the
route-ops-plus-reduction oracle, and it passes. So does `assert_route_reduce_reads_lent_inputs_as_the_copy`.
The diagnosis's "new fan-in sweep" gate therefore already exists.

**Two reds before the fix:** the source-plane counters. The prototype's group path drops
`test_only_count_source_plane`. That breaks `[0, claims*BLOCKS, 0]`
(console-workload lib.rs:2036, which is the gate the diagnosis names), three graph unit tests
(runtime.rs:15874, 16053), and it would break host-core `source_in_place.rs:157`. The internal
trait also needs a provided default, because test mocks implement `GraphSourcePlanes`. With the
counters restored, only the rt9 source-pin is red.

**Change 4 contract hazard.** Today the set checks `claim_index < claims.len()` before it calls the
driver (lib.rs:1935-1937). The batched form passes the group's raw claims, `NO_SOURCE_CLAIM`
(`u32::MAX`) entries included, to a public-trait driver. A host driver that indexes
`self.x[claim]` would panic on the render thread. With a measured benefit of about 75 cycles, this
is not worth a public API.

**Change 3.** It is equivalent to `execute_op`'s bound arm when there are no inputs and nothing is
staged: the reduction is skipped (runtime.rs:3389) and `output_planes` falls back to
`write_stereo` when there is no host. The error path is unchanged. The rt9 source-pin needs the
same re-pin.

**Change 5.** The shared scratch is consistent with the bound contract, since #218 already hands a
processor stale arena words. The prototype fold (`test_only_bound_fold`) is non-generic and calls
`route_run::<FrameLane,1>`. Its wasm symbol would not match `4wide6f32x[48]`, so rule 3 would
never inspect it. That makes it gate-blind rather than gate-failing.

## 3. Wasm and allocation gates

* **Rule 3** (`--kernel-pattern '4wide6f32x[48]' --kernel-min 11`, check-web-audioworklet.sh:428)
  matches by symbol name. `route_reduce<f32x4>` is visible in the current artifact
  (`wasm927.dis`, `…route_reduce…4wide6f32x4…`). A generic `#[inline(never)] route_group<L>` stays
  visible and keeps the count, provided the f32 tail stays in `route_tail`.
* **The callgraph allocation and trap half cannot see the graph executor.** It follows only direct
  `call`, and the executor sits behind `Box<dyn PreparedPlanExecutor>`
  (engine realtime/plan.rs:557). Allocation-freedom for changes 1-5 therefore rests on the native
  counting-allocator tests. rt10 covers the Output in-place read (changes 1, 2 and 4), and it
  passes on `v2c124`. Changes 3 and 5 need a bound-feed allocation test named in their briefs.
* **Resource accounting.** There is no "runtime metadata reservation" term for an executor table.
  `GraphRuntimeMetadataResourceEstimate` (lib.rs:452-510) charges only layout deltas, and the
  sibling `source_input_buffers` (lib.rs:2239) is uncharged too. Adding the field to
  `GraphExecutor` without adding it to both mirrors (lib.rs:2246, 2257) would shift the
  observation-state and split-owner deltas.

## 4. Floor recount (claim b)

* The prior verifier reproduced 3.960 and 3.984 mul:add ops/cycle and 32.0 B/cycle L2→L1 at
  48-128 KiB. My frozen 68 KiB read took 2,261 cycles (30.8 B/cycle). The arithmetic
  (4,096 mul + 4,064 add = 8,160 → 2,048-2,055 cycles) and the L2 term (2,048) are **right for
  this host**.
* The ruling defines the floor as the arithmetic only; loads, stores and dispatch are named gap
  terms (effect-floor-accounting.md:60-63). "max(arith, L2) + fixed" is therefore a different
  quantity. That it also lands at 0.60 us is a coincidence. `floor.rs` needs no change.
* Worth a sentence in the ruling: the plumbing rows are recorded on this Zen 3 (3.97), while
  `OPS_PER_CYCLE` = 3.695 was probed on Zen 5 (a 7 % scale). PLAN.md's term (b), "one line per
  cycle", is Zen 5 behaviour and should be corrected for this host.

## 5. What is missing: the live producer (the largest term)

The ring row's driver re-lends one frozen 64 KiB block, which stays warm in the render core's L2.
A real `crates/source` ring is written by a decode worker every block. `xcore/xcore.c` measures
this: 64 SPSC rings, depth 4, and a producer thread writing 1 KiB plus a header per source per
block. The render core (cpu 31) spins until each head is published, then times only the read of
all 64 blocks (8 accumulators, bandwidth-bound).

| producer | read of 64 x 1 KiB per block |
|---|---:|
| none (frozen, like the ring row) | 2,261 cycles |
| SMT sibling (cpu 15, shared L2) | 3,152 |
| same CCD (cpu 12/13/14) | 5,894-6,416 (**+3,600-4,150**) |
| other CCD (cpu 0/4) | 20,572-20,979 (**+18,300-18,700**) |

This is before `begin_block`'s real bookkeeping: pop, recycle and the generation check per
source, at crates/source/src/lib.rs:1301-1360 and 1506-1530.

* **On a live feed this term is larger than every change in the plan combined.** The floor
  analysis ("L2 fill 2,048") does not apply there.
* **The "software prefetch: not worth doing" dismissal was measured only on L2-warm input.**
  Change 2's group resolution hands over the next group's plane addresses a group early, so
  prefetching group g+1 is plausible on a live feed. It should be re-opened against a
  live-producer row.
* **Which core the decode worker runs on is a host placement question.** It moves the block by up
  to 18k cycles.

## 6. Class B (owner's reorder ruling)

No summation reordering can beat the class A kernel on this row.

* **Reassociation cannot reduce the operation count.** The row needs 4,096 multiplies and N-1 adds
  per plane-chunk however they are associated.
* **The pair kernel is not chain-bound.** Each pair runs 512 independent chunk chains. Its
  measured limit is issue width, about 25 instructions per pair-chunk at about 4.2-4.5 IPC, plus
  the master re-pass.
* **The class A frame-tiled kernel already removes the re-pass.** It keeps edge order in
  registers.
* **Reordering only helps the tiled T=1/2 shapes, and those still lose.** Those shapes are
  chain-bound (T=1 from L2: 8,721 cycles). T=4 already escapes the chain in class A.
* **The only algebraic changes that cut operations are not reorderings:**
  * FMA fusion. It is forbidden by the unfused ruling, and on Zen 3 FMA shares the two FMUL pipes,
    so the pipe bound stays at 2,048.
  * Distributive factoring of equal-coefficient routes. It changes rounding and is specific to
    this row.

## 7. Files

* Builds: `diagbench-{v2base,v2c1,v2c12,v2c124}-{prod,ts}`, and sources in the matching `v2*/`
  directories. The change-2 kernel is `v2c124/crates/graph/src/runtime.rs` (`route_group`,
  `OUTPUT_GROUP`).
* Tests: `v2tests.log`, `v2tests2.log`.
* Cross-core probe: `xcore/xcore.c`, `v2-xcore.txt`.

## 8. Disposition (coordinator, 2026-09-26)

| DIAGNOSIS-2 item | disposition |
|---|---|
| change 1, skip inert units | briefed as #936, with the rt9 re-pin and the resource-accounting charge |
| change 2, groups of eight and a tighter pair kernel | briefed as #937, counters kept, public driver trait untouched |
| change 3, fast bound dispatch | later, bound feed only; needs a bound-feed allocation test |
| change 4, batched driver call | dropped: about 75 cycles, and a public-trait hazard |
| change 5, fold bound inputs as written | deferred |
| change 6, 64-byte alignment | harness half and claim (d) briefed as #935; engine half deferred until #938 gives a live-feed number |
| finding 1, live producer | briefed as #938; prefetch and worker placement reopen after it |
