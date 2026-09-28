# Resolve Output inputs in groups of eight and tighten the pair kernel

**Ruled** (coordinator, 2026-09-26): change 2 of
`docs/handoffs/plumbing-floor-2026-09-26/DIAGNOSIS-2.md`, with the amendments of its adversarial
verification (`DIAGNOSIS-2-VERIFY.md`). Change 4 (a batched call on the public driver trait) is
dropped and is not part of this issue. Class A.

## Product outcome

The session Output op's fused reduction (`route_reduce`, `crates/graph/src/runtime.rs:714`) walks
its inputs a pair at a time. For every pair it resolves each input's planes through
`OutputSources::input` (two indirect calls per in-place claim, the planes returned through memory as
an `Option`), builds slice arrays, checks lengths with `.any()`, splits the host planes, and tests a
`?` on every chunk. That bookkeeping costs more than the arithmetic's own overhead. Resolve inputs
eight at a time into a stack table, check lengths once per group, and run each pair as a plain zip of
`chunks_exact` iterators. Measured in the shipped release profile on
`sixty_four_track_plumbing_ring`, on top of "Skip inert source-input units at render dispatch": -1,370
to -1,710 cycles per block. Class A: the per-frame summation chain and the unfused `mix` operands are
unchanged.

## Root evidence

- `runtime.rs:610` `OutputSources`; `:631` `OutputSources::input` (one table read and one branch per
  input, then the source set's `played_planes`).
- `runtime.rs:714` `route_reduce`, `:783` `route_pair<L, G>`, `:886` `route_run<L, G>`, `:841`
  `route_tail` (non-generic, `#[inline(never)]`), `:938` `mix_chunk`.
- `crates/graph/src/lib.rs:1834` `pub(crate) trait GraphSourcePlanes`; `:1926` its impl on
  `GraphPreparedSourceSet` range-checks the claim (`claim_index >= self.claims.len()` returns
  `None`) and holds each plane to the quantum before any reader sees it.
- Summation order today, per frame and per plane: the first pair stores `m0 + m1`; each later pair
  computes `(load + m_2k) + m_(2k+1)`; an odd last input computes `load + m`. `mix` is
  `lr.fma(r, ll.mul(l))` and `rr.fma(r, rl.mul(l))` with `Lane::fma` unfused (multiply, round, add,
  round). Inputs are in the Output op's edge order.
- The existing oracle `assert_route_reduce_is_the_route_ops_and_the_reduction` (`runtime.rs:14143`)
  sweeps frames {1, 3, 7, 8, 13, 16, 33, 64} x fan-in 2..=19 plus 64 with signed zeros against the
  route-ops-plus-reduction oracle; `assert_route_reduce_reads_lent_inputs_as_the_copy` (`:15892`)
  covers in-place inputs. Both passed on the verification's production-shape build of this change.
- A single stack table for all inputs was measured 1,500 cycles *worse* and would need a compiled
  maximum; do not build it.

## Smallest closable slice

Authorized paths: `crates/graph/src/runtime.rs` (`OutputSources`, `route_reduce`, `route_pair`,
`route_run`, a new `route_group`, and their tests), `crates/graph/src/lib.rs` (the internal
`GraphSourcePlanes` trait only), `crates/graph/tests/MUTATIONS.md`, and this spec.

1. **Group resolution.** `route_reduce` walks the inputs in groups of `OUTPUT_GROUP = 8` (a batch
   size, not a track cap: any fan-in works). For each group it resolves every input's planes once into
   a stack `[(&[f32], &[f32]); OUTPUT_GROUP]` and checks every plane's length against the lease's
   frames once. In-place claims keep going through the set's range check. If you add a group method
   to the internal `GraphSourcePlanes` trait, give it a provided default body that loops over
   `played_planes` (test mocks implement the trait), and keep the per-claim range check. Do **not**
   add or change any method of the public `GraphPreparedSourceSetDriver`.
2. **The kernel.** A generic `#[inline(never)] fn route_group<L: Lane>` runs the group's pairs over
   the vector frames as a zip of `chunks_exact(L::WIDTH)` iterators, with no per-chunk `?` and a plain
   store loop for the block's first pair. The odd last input stays a `G = 1` step. The `f32` tail
   frames stay in the non-generic `route_tail`, called per pair after the group's vector frames.
3. **Counters.** Keep every `test_only_count_source_plane` call with its current meaning. The
   verification's prototype dropped them and broke four gates.

## Non-goals

No change to the summation order, to `mix_chunk`, to which inputs are read in place (#927), to
dispatch (the inert-unit issue), or to the public driver trait. No frame-tiled register-accumulator
kernel (measured slower in the engine) and no software prefetch (reopened only after a live-producer
row exists).

## Objective gates

1. **Oracle.** `assert_route_reduce_is_the_route_ops_and_the_reduction` and
   `assert_route_reduce_reads_lent_inputs_as_the_copy` pass unchanged at both lane widths. Add fan-in
   257 (odd, above any power of two the group size divides) to the first test's sweep to prove there
   is no track cap.
2. **Counters.** `[0, claims * BLOCKS, 0]` at `tools/console-workload/src/lib.rs:2036`, the graph
   source-plane counter tests near `runtime.rs:15874` and `:16053`, and host-core
   `tests/source_in_place.rs` pass unchanged.
3. **Digests.** Every console workload's 64-block digest is unchanged.
4. **Wasm rule 3.** On the batch's rebuilt AudioWorklet artifact,
   `scripts/check-web-audioworklet-callgraph.py` rule 3 passes: every `4wide6f32x4` function has more
   vector than scalar arithmetic, and `route_group<f32x4>` is visible under that pattern.
5. **Allocation and policy.** `crates/graph/tests/rt10_source_in_place_alloc.rs`,
   `scripts/check-realtime-policy.sh`, `check-graph-determinism.sh`, `check-graph-policy.sh`.
6. **Red mutations** recorded in `crates/graph/tests/MUTATIONS.md`: reassociate a later pair as
   `load + (m_2k + m_(2k+1))` (gate 1 fails); drop the counter call (gate 2 fails); resolve a group
   of 8 but reduce only its first 7 (gate 1 fails at fan-in 8).
7. `cargo test -p graph` with and without `test-support`, `-p console-workload`, `-p host-core`;
   fmt, clippy with `-D warnings`, and `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
   --no-deps`.

## Console benchmark rows

Can move: `sixty_four_track_plumbing_ring` (most) and `sixty_four_track_plumbing_only` (Output phase
only, about -270 cycles). No digest may move.

## Dependencies

None in code. It composes with "Skip inert source-input units at render dispatch"; implement after it
so the benchmark attributes each saving separately.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit. Every "unchanged" gate is a hard stop.
- Render paths stay allocation-free, lock-free and syscall-free. `crates/graph` stays free of
  `unsafe`. Public docs must not link to private items.
- Commit on `codex/<issue>-<slug>` from synchronized `main`. Do not quote a projected saving.
- The AudioWorklet artifact pin and browser qualification are repinned once at the batch boundary.

## What the implementer will hit

- A stack array of slice pairs needs an initial value; `[(&[][..], &[][..]); 8]` costs nothing, but a
  4 KiB table does (the rejected shape). Keep the table at eight entries.
- `route_pair` is `#[inline(always)]` today; the new `route_group` must be `#[inline(never)]` so the
  wasm symbol stays visible to rule 3, and the scalar tail must stay out of it or rule 3 fails exactly
  as #920's attempt 1 did.

## Attempt 1 evidence

Implementer: attempt 1, 2026-09-27. Code commit `40c62101` on `codex/937-grouped-output-kernel`, on
top of the #936, #945 and #944 branches (branch tip `04ed64c7`, the "base" everywhere below). Host
AMD EPYC 7313P (Zen 3), rustc 1.97.1, `.cargo/config.toml` pin `+avx2,+fma`, `CARGO_INCREMENTAL=0`,
the worktree's own target directory plus one scratch target directory for the wasm builds. The
digest harness was an untracked scratch test file, never committed. No timed runner and no timing
were run; no saving is quoted.

### Design

Authorized paths only: `crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs` (the internal
`GraphSourcePlanes` trait), `crates/graph/tests/MUTATIONS.md`, this spec.

- **`OUTPUT_GROUP = 8`** (runtime.rs). `route_reduce` keeps its up-front shape checks, then walks
  `inputs.chunks(8)` with `routes.chunks(8)`. Per group it builds a fresh stack table
  `[(&[], &[]); 8]`, fills it with `OutputSources::resolve_group`, and calls `route_group::<L>`
  with `store = (group == 0)`. The table is rebuilt per group, not hoisted, so a slot that were ever
  left unfilled fails the length check instead of reading the previous group's planes. The group is
  even, so pairs never straddle groups and only the last group can end in a lone input. Any fan-in
  works (fan-in 257 is in the oracle).
- **`OutputSources::resolve_group`** replaces `OutputSources::input`. Without a claim table (or
  without a source set) every slot is `lease.read_stereo(buffer)`, as before. With one, the
  `NO_SOURCE_CLAIM` slots are arena reads, and the in-place slots come from one call per group,
  `set.played_planes_group(claims, silence, planes)`.
- **`GraphSourcePlanes::played_planes_group`** (lib.rs), a provided method, not overridden. Its body
  skips `NO_SOURCE_CLAIM` and sets `slot = self.played_planes(claim)` or `silence` on `None`. In the
  set's vtable that `self.played_planes` is a static call into `GraphPreparedSourceSet::played_planes`,
  so the claim-index range check and the quantum filter still run per claim. The two
  `test_only_count_source_plane` calls moved here from `OutputSources::input` and keep their meaning:
  slot 1 once per claim served a played block, slot 2 once per claim served silence. The bank
  gather's two calls (`ArenaMembers::plane`) are untouched. `test_only_count_source_plane` became
  `pub(crate)` so lib.rs can call it. The provided body sits in a `REALTIME_POLICY` region (55
  regions, from 54). The public `GraphPreparedSourceSetDriver` is unchanged: the driver is still
  asked once per claim.
- **`route_group<L>`**, generic and `#[inline(never)]`. It refuses a bad shape before the group's
  first write: a table that does not match the planes, an empty group, a storing group of fewer than
  two, a right plane of another length, or any input plane that is not `left.len()` words (checked
  once for the group). It splits the host planes at `vectored` with `split_at_mut_checked`, takes
  `planes.as_chunks::<2>()`, and runs every pair's vector frames (`route_pair_vectors`, with
  `store && index == 0`), then the lone input's (`route_lone_vectors`). Then, only if there is a
  tail, it calls `route_tail` per pair and for the lone input, in edge order. Each input's tail is
  `chunks_exact(L::WIDTH).remainder()`, which has no bounds check.
- **`route_pair_vectors` / `route_lone_vectors`** (`#[inline(always)]`) are plain zips of
  `chunks_exact(L::WIDTH)` iterators, with no `?` and no per-chunk test. Store form:
  `mix(in0).add(mix(in1)).store`. Accumulate form: `load.add(mix(in0)).add(mix(in1)).store`. Lone
  input: `load.add(mix(in)).store`. `mix_chunk` is unchanged. This is the same chain and the same
  unfused operands as `route_run`/`add_mixed_chunks`.
- **`route_tail` is unchanged**, so the tail still runs `route_run::<f32, G>`. `route_run` is now
  only the tail's body, and its doc says so. `route_pair` is deleted.

**Class A.** Per frame and plane, the first pair stores `m0 + m1`, each later pair computes
`(load + m_2k) + m_(2k+1)`, and an odd last input computes `load + m`. Groups end on pair
boundaries, so grouping adds no store or reload. Frames are independent, so running every pair's
vector frames before any pair's tail frames changes no frame's chain. Nothing in `crates/graph`
uses `unsafe`.

### Gate 1 -- oracle: PASS

`a_route_reduction_is_the_route_ops_and_the_reduction_bit_for_bit` (`f32`, `Simd4`, `Simd8`) now
sweeps fan-in `(2..=19).chain([64, 257])`. 257 is 32 whole groups plus a lone input in a group of
its own. `a_route_reduction_reads_each_lent_input_as_the_copys_words` is unchanged and passes at
all three widths. The new `a_source_sets_group_call_is_its_per_claim_call` builds a real
`GraphPreparedSourceSet` whose driver underruns claim 1, lends a short plane for claim 2 and a plane
for out-of-range claim 3. It checks, by slice address and length, that each slot is the set's
per-claim answer (silence on `None`), that the `NO_SOURCE_CLAIM` slots keep the caller's planes,
and that the counters read `[0, 2, 3]`.

### Gate 2 -- counters: PASS (unchanged)

The console-workload lib (5/5) passes, including `the_driver_fed_plumbing_row_renders_the_bound_rows_bits`
(`[0, claims * BLOCKS, 0]`). The graph counter tests pass: #927's gates 1 and 2 and #936's inert
gates. host-core `tests/source_in_place.rs` passes (1/1).

### Gate 3 -- digests: PASS (unchanged)

Every console workload's 64-block digest was taken in the release profile with a scratch harness:
`WORKLOADS` plus `DRIVER_FED_WORKLOADS`, `PlanConfig::BASELINE`, `Sha256Sink` over `hash_output`
per block. That is the `digests` test of `docs/handoffs/gain-pan-2026-09-26/gain-pan-diagnosis-harnesses.patch`
with the ring row added. It was run on the base, then on the final tree. The two 17-line outputs are
identical (`diff` is empty):

| workload | base `04ed64c7` | after `40c62101` |
|---|---|---|
| nine_track_baseline | `e7c6ef01770ab7da...` | same |
| nine_track_ragged_strip | `17613a3ab693d3f0...` | same |
| sixty_four_track_console | `fe5bed9becdbc101...` | same |
| one_twenty_eight_track_stretch | `cba2c94f81544caa...` | same |
| sixty_four_track_eq_only | `9b2c56a1da62ebda...` | same |
| sixty_four_track_compressor_only | `95c9375429fbca34...` | same |
| sixty_four_track_builtins_only | `b63eccd09c19eb7a...` | same |
| sixty_four_track_dispatch_only | `15688888612d161e...` | same |
| sixty_four_track_idle | `de2f256064a0af79...` | same |
| sixty_four_track_console_legacy | `f68febb7a10e242b...` | same |
| sixty_four_track_eq_comp_simd1 | `f68febb7a10e242b...` | same |
| sixty_four_track_plumbing_only | `57535244ba953d82...` | same |
| sixty_four_track_gain_pan_only | `01e465a797036fb4...` | same |
| sixty_four_track_console_mono | `fc96d91f6a397e91...` | same |
| sixty_four_track_console_mono_dual | `fc96d91f6a397e91...` | same |
| sixty_four_track_console_half_mono | `4a656cdf63882999...` | same |
| sixty_four_track_plumbing_ring | `57535244ba953d82...` | same |

These are the #945 values. `cargo test -p console-workload` also passes: lib 5, automation 4,
chain_shape 25, placement 3. That includes `BASE_DIGEST` and
`the_plumbing_rows_output_fold_is_the_route_ops_own_bits`.

### Gate 4 -- wasm rule 3: PASS

Both artifacts were built with the build script's own line into one scratch target directory:
`CARGO_TARGET_DIR=<scratch> RUSTFLAGS="-C target-feature=+simd128 -C strip=debuginfo
--remap-path-prefix=$CARGO_HOME=/cargo --remap-path-prefix=<src>=/repo" cargo build --locked
--release --target wasm32-unknown-unknown -p host-web`. The base was built from a `git archive` of
`04ed64c7` and the after from the worktree. Each was checked with `wasm-objdump -d host_web.wasm |
python3 -B scripts/check-web-audioworklet-callgraph.py ...`. Artifacts: base `025b6b20e81ef4cf...`,
after `0b6b06328b56ab23...`. The committed pin `8934cdd9...` was not touched; it is repinned at the
batch boundary.

- `--kernel-shape --kernel-pattern '4wide6f32x[48]' --kernel-min 11`: exit 0 on both. Base
  `f32x4_arith=11725 kernels=15`, after `f32x4_arith=11719 kernels=15`. Every roster row is
  identical.
- Per function, vector / scalar `f32` arithmetic:
  - base `route_reduce<f32x4>`: 44 / 0.
  - after `route_group<f32x4>`: **38 / 0**. It is visible under the pattern and is the arithmetic
    kernel the gate counts.
  - after `route_reduce<f32x4>`: 0 / 0. It now only resolves and calls, so the gate skips it as a
    non-arithmetic function.
  - `route_tail`: 0 vector on both (308 scalar in the base, 132 after). Both are fully unrolled,
    and the `f32.store` count falls from 56 to 24. That is 7 tail frames against 3, times 2 planes,
    times 4 forms. It is consistent with LTO now seeing that the wasm tail is at most three frames:
    `vectored` is computed inside `route_group`, where it used to arrive as a parameter.
  - The 6 vector operations fewer (44 to 38, and 11725 to 11719 in the module) are the lone input's
    store form: 4 `mul`s and 2 `add`s that `route_run::<L, 1>` compiled and could never run. A lone
    input is never the first input.
- `--callgraph miso_engine_web_v1_render`: `closure=8 traps=5` on both, with the same trap owner
  (`PreparedRenderPlan::render_inner`) and the same entry (`slice_index_fail`). Unchanged.
- Disclosure: in `route_group<f32x4>` one `Zip::new` stays out of line for the lone-input loop, so
  that loop's two chunk-size checks (`slice_index_fail`) are hoisted before the loop, not run per
  chunk. The pair loops carry no call and no check. The two remaining calls are to `route_tail`.
- Native, for reference: the release `route_group<f32x8>` calls only `route_tail`, with no panic
  path. Its accumulate loop per chunk is 4 input loads, 8 `vmulps`, 4 plus 2 `vaddps` and 2 stores.
  `route_reduce<f32x8>` makes one indirect call per group (`played_planes_group`); its
  `panic_bounds_check` sites are `ArenaLease::read`'s release read-ID guard, inlined from
  `read_stereo`. `played_planes_group` makes one indirect call per claim, to the driver. The
  per-claim indirect call into the set is gone.

### Gate 5 -- allocation and policy: PASS

- `crates/graph/tests/rt10_source_in_place_alloc.rs`: 2/2 (with and without `test-support`). rt1
  1/1 and rt9 8/8 also pass.
- `bash scripts/check-realtime-policy.sh`: `realtime policy: ok (55 marked regions in 16 files)`.
- `bash scripts/check-graph-policy.sh`: `graph policy: PASS`.
- `bash scripts/check-graph-determinism.sh`: `graph fresh-process determinism: PASS (100/100)`.

### Gate 6 -- red mutations: recorded

The rows are in `crates/graph/tests/MUTATIONS.md`, "Issue #937".

- 937-1, reassociate a later pair: gate 1 RED at `width 1, 1 frames, fan-in 7`, plus the fold,
  #927 and #936 bit gates and both console digests.
- 937-2, drop the played-read counter call: gate 2 RED on the console counter gate
  (`[0, 0, 0]` against `[0, 4096, 0]`), #927's gates 1 and 2, the lent test and the group test.
- 937-2b, the silent-read call: RED on the lent test, the group test and #927's underrun blocks.
- 937-3, reduce 7 of a group of 8: gate 1 RED at `fan-in 8`, plus the bit gates and console
  digests above.
- 937-4 (added), `route_tail` inlined: rule 3 RED, `route_group ... vector=38 scalar=122`.

host-core `source_in_place` stays green under 937-2. It pins bank gathers, which count at the
other site. It is a stay-green test, not that row's witness.

### Gate 7 -- suites: PASS

- `cargo fmt --all --check`: clean.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: exit 0.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: exit 0.
- `cargo test --locked -p graph`: lib 114, rt10 2, rt1 1, rt9 1.
- `cargo test --locked -p graph --features test-support`: lib 114, rt10 2, rt1 1, rt9 8.
- `-p console-workload`: 5 + 4 + 25 + 3.
- `-p host-core`: all binaries pass, `source_in_place` 1.
- `-p capi`: 32 + 4.

### Deviations

1. `OutputSources::input` is gone; `resolve_group` replaces it. The two Output counter calls now
   live in the trait's provided `played_planes_group` (lib.rs, the authorized trait). Their meaning
   is unchanged, and 937-2 and 937-2b prove it.
2. The new test, `a_source_sets_group_call_is_its_per_claim_call`, is in runtime.rs, not lib.rs, to
   stay inside the authorized paths.
3. Row 937-4 was added beyond the brief's three mutations, to show that rule 3 does inspect
   `route_group<f32x4>`.

## Sol attempt 1 verdict: PASS

Reviewer: Sol, attempt 1, on `dc7d9155` (code `40c62101`, base `04ed64c7`). Nothing was pushed,
and no timed benchmark was run. Every mutation, digest and wasm build ran on `git archive` copies
in a scratch target directory, which has since been deleted. The worktree was never edited, apart
from this section.

The change is class A, keeps the source-plane contract and its counters, has no track cap, and
passes every gate. The findings below are informational. None of them blocks the issue.

### Findings, most severe first

1. **Informational: host-core `source_in_place` does not witness the Output counters, before or
   after this change.** Its three fixtures bind every claim to a bank gather. Its pin
   `[0, claims * 9, claims * 3]` therefore counts gathers only (`ArenaMembers::plane`,
   `runtime.rs:2005-2011`), and 937-2 leaves it green, as disclosed. The claim in
   `DIAGNOSIS-2-VERIFY.md:99` that it "would break" does not hold for a drop at the Output site.
   The move is still witnessed. Under 937-2 I reproduced four red graph tests (the group test, the
   lent test, and #927's gates 1 and 2) and the console counter gate (`[0, 0, 0]` against
   `[0, 4096, 0]`).
2. **Informational: the group size's evenness is not a correctness condition.** Two equivalent
   mutants stay green: `OUTPUT_GROUP = 7` (graph 114 of 114 and console-workload green) and
   `OUTPUT_GROUP = 3` (graph 114 of 114). With either, a would-be pair is split across two groups
   and a lone input sits mid-block. The chain is still exact, because a group boundary is only a
   store and a reload. Two statements hold only while the group is even: "the lone input is only
   ever the block's last" in the `route_group` doc (`runtime.rs:770-775`), and "never the block's
   first input" (`route_lone_vectors`, `:910`). A group of 1 would fail loudly at `:794`, not
   silently. These rows can be listed as disclosed equivalents at the batch boundary; nothing is
   required.
3. **Informational: wasm trap placement moved, but no trap kind is new.** `route_reduce<f32x4>`
   now inlines `resolve_group`, so it carries 9 `panic_bounds_check` sites. They are the arena's
   release read-ID guard (`engine/src/realtime/disjoint.rs:176`), which in the base sat inside the
   out-of-line `OutputSources::input`. The render callgraph is unchanged on both artifacts:
   `closure=8 traps=5`, with the same owner and the same entry.
4. **Informational: a fixed cost per group, for the weekly pass only.** `resolve_group`
   (`runtime.rs:655-661`) makes one silence read (two guarded reads) and one `dyn` call per group,
   even when the group has no lent claim. That is at most `ceil(fan-in / 8)` per block.

### What I re-ran and checked

- **Class A (question 1).**
  - *Chain:* by reading the code and the release x86 disassembly of `route_group<f32x8>`. The
    store loop is `m0 + m1`, the accumulate loop `(load + m0) + m1`, and the lone loop
    `load + m`. Each `mix` is `vmulps` and `vmulps` then `vaddps`, with no `vfmadd`. `mix_chunk`
    is untouched. The only calls are the two to `route_tail`, and there is no panic path.
  - *Store flag:* `initial_store` is `index == 0` for the group and `store && index == 0` for
    the pair (`:762`, `:817`, `:840`). It is set exactly once per block.
  - *Pair shape:* no pair can straddle a group of 8. Finding 2 shows that one split across a
    group of 7 or 3 still gives the same bits.
  - *Oracle:* green at `f32`, `Simd4` and `Simd8`. It covers fan-in 2..=19, 64 and 257, frames
    {1, 3, 7, 13, 33}, which are ragged at both widths, and signed zeros. The lent test is green
    too; its fan-ins 9 and 64 put lent, silent and arena inputs across group offsets.
- **Source-plane contract (question 2).**
  - *Provided body:* `played_planes_group` (`lib.rs:1874`) skips `NO_SOURCE_CLAIM`, which
    matches the old arena read. It calls the implementor's own `played_planes` for every other
    claim.
  - *Wasm:* the production `GraphPreparedSourceSet` instance shows, per claim, `== -1` then skip,
    `claim >= claims.len()` then silence, then the driver `call_indirect`, then `Some` and both
    plane lengths `== quantum`, before any slot is written.
  - *Public trait:* `GraphPreparedSourceSetDriver` is byte-identical.
  - *Counter hook:* `test_only_count_source_plane` is `pub(crate)` and under
    `cfg(any(test, feature = "test-support"))`. The production wasm has no `test_only` symbol.
- **Mutations (all reproduced).**
  - *The implementer's rows:*
    - 937-1: 7 of 114 red, first `width 1, 1 frames, fan-in 7` (`1118879272` against
      `1118879273`), and console `9b7337c3...` against `57535244...`.
    - 937-2: as in finding 1.
    - 937-2b: 4 of 114 red.
    - 937-3: 7 of 114 red, first `fan-in 8` (`1254604654` against `1254604426`), plus both
      console digests.
    - 937-4 (rule 3): `route_group ... vector=38 scalar=122`.
  - *My own rows, all red:*
    - Drop the group's claim offset: 6 red, including the lent test at fan-in 9.
    - Store in every group: 8 red, including the oracle at fan-in 9.
    - Lone tail stores: 3 red (`width 4, 1 frames, fan-in 3`).
    - Lone vectors before the pairs: 3 red.
    - Every pair's tail stores: 7 red.
    - No `NO_SOURCE_CLAIM` skip: 5 red, including the group test.
    - Set range check removed: 2 red.
    - Quantum filter weakened to `||`: 2 red (the group test and 918's).
- **Digests (gate 3).** I built base and head, each forced to rebuild, and checked the head binary
  carries `route_group::<f32x8>`. All 17 console workloads render the same 64-block digests at
  three dispatch widths: current (`Simd8`), `Simd4` and `Scalar`, 51 of 51. They equal the table
  above and the #945 values.
- **Wasm rule 3 (gate 4).**
  - *Build:* I rebuilt with the build script's cargo line. Artifacts: base `025b6b20...` and
    head `0b6b0632...`, both equal to the implementer's.
  - *Rule 3:* exit 0, `kernels=15`, and the roster lines are identical to the base.
  - *`route_group<f32x4>`:* 38 vector and 0 scalar. The base `route_reduce<f32x4>` was 44 and
    0; the 6 fewer are the lone input's never-run store form. `route_tail` has 0 vector ops (132
    scalar, 24 `f32.store`).
  - *Out-of-line `Zip::new`:* it is called once, before the lone-input loop, and its two
    chunk-size checks are hoisted with it. The loop body is loads, 4 `mul`s, 4 `add`s and 2
    stores, with no call or check. So its cost is one call per block, only when the fan-in is odd,
    and nothing per chunk.
- **Allocation and policy (gate 5).**
  - `rt10` passes, 2 of 2, with and without `test-support`.
  - `check-realtime-policy.sh`: 55 regions. The base has 54.
  - `check-graph-policy.sh`, `check-graph-determinism.sh` (100 of 100) and
    `check-workspace-policy.sh` pass.
- **Suites (gate 7).**
  - `cargo fmt --check` and workspace clippy `--all-targets --all-features -D warnings` are clean.
    So is graph clippy on a fresh scratch build.
  - `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` passes.
  - `cargo test -p graph`, with and without `test-support`: 114, rt10 2, rt1 1, rt9 1 or 8.
  - `-p console-workload`: 5 + 4 + 25 + 3.
  - `-p host-core`: every binary, including `source_in_place` 1.
  - `-p capi`: 32 + 4.
- **Scope.** Only the four authorized paths changed. Making `test_only_count_source_plane`
  `pub(crate)`, and the doc-only edits to `route_tail`, are what the lib.rs counter placement
  needs; I accept them.
