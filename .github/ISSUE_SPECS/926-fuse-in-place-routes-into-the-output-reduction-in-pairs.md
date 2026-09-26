# Fuse in-place routes into the Output reduction in pairs

Supersedes #920 (withheld: its `route_run::<L, 8>` kernel hoisted 32 splatted coefficients and
sixteen iterators into one loop, and its wasm instantiation failed the AudioWorklet callgraph
gate's rule 3 with vector 560 against scalar 1,680 arithmetic because its same-body `f32` tail
was unrolled inside the `f32x4` instantiation; it did not move the row). The eligibility half of
#920 stands; the kernel is re-shaped from the measurement in `DRAFTS/PLAN.md`, Part 2, table C,
and its tail is outlined.

## Product outcome

`sixty_four_track_plumbing_only` pays 64 dispatched route units (8,052 cycles: 43 of kernel and
about 83 of dispatch each) and then an Output unit that reloads their 64 buffers in eight groups
of eight (3,409 cycles). Retire every plain route whose sole reader is the Output op and give that
op one fused pass: per **pair** of inputs, load both planes of each, apply its 2x2 in registers
from eight coefficient splats hoisted per pair, accumulate in edge order with the first pair
storing and each later pair reloading the running master, store. Measured as a replica over the
row's working set: 3,450-3,550 cycles for the pair shape against 4,500-5,000 for #920's group of eight and
11,461 for today's route-then-reduce; every group size and both coefficient policies are in the
table and the pair won at both. Class A: the same per-element sequence as `mix2x2_block` then
`accumulate_run` (`lr.fma(r, ll.mul(l))`, `rr.fma(r, rl.mul(l))`, then `value = in0` or
`load(out)`, then `value.add(in_i)` in edge order), asserted bit-identical by the harness for
every group size.

## Root evidence

- `crates/graph/src/runtime.rs:2953-2956`: `NodeKind::Route` runs `mix2x2_block::<FrameLane>` in
  place; `crates/lane/src/kernels.rs:734` fixes the order (unfused `fma` by contract, `vmulps`,
  `vmulps`, `vaddps` in the release binary).
- `runtime.rs:534-600`: the Output op reduces through `reduce_plane_into` -> `reduce_many_into`
  (groups of `REDUCE_GROUP = 8`, `:385`) -> `reduce_group_into::<L, N>` -> `accumulate_group` /
  `accumulate_run` (`:451`, `:480`): the first group stores, later groups reload `target`.
- `program.rs:716-720`: a plain route with a single undelayed sole-read input is lowered in place
  over that buffer, which stays coloured until its last reader, the Output op.
- `scripts/check-web-audioworklet-callgraph.py:352-359` (rule 3): every function matching
  `4wide6f32x[48]` that carries `f32x4` arithmetic must have strictly more vector than scalar
  arithmetic. A spill emits no `f32.mul`/`f32.add`; an unrolled width-4 scalar tail emits three
  per vector op. Reviewer's scratch simd128 builds: pair kernel with a same-body tail 30 vector /
  90 scalar (fails); #920's group of eight 168 / 624; pair kernel with the tail outlined into a
  non-generic `#[inline(never)]` function 30 / 0 (passes).
- #920's branch `origin/codex/920-fold-routes-into-output-reduction`, `runtime.rs:6427`
  `output_route_fold` (bind-time eligibility: Output op, `NodeKind::Identity`, no sidechain, no
  split pair, no delayed input, not a bank member, fan-in >= 2; every producer a `plain_route_gains`
  route with one undelayed input, `readers[R] == [M]`, in place, unobserved; the in-between
  `op_names_buffer` scan; retire through the `retired` set `build_sequential` honours
  (`:4896` on `main`); `route_table: Box<[[f32; 4]]>` in edge order; counter `output_route_folds`;
  seam `test_only_set_output_route_fold_declined`). Its gates 1-5 and red mutations are reused.
- Measured (`DRAFTS/PLAN.md` table C, cycles per block, bit-identical): hoisted G=1 3,763-3,790,
  **G=2 3,451-3,549**, G=4 4,121-4,139, G=8 4,522-4,985 (two builds); broadcast-per-chunk G=1 5,483-5,598,
  G=2 4,478-4,561, G=4 4,795-4,865, G=8 7,766. On AVX2 a broadcast is a load-port op per chunk;
  hoisting more than two tracks' coefficients (8 x G of 16 registers) spills.
- Static (table D): the harness's `fused_route_reduce::<2, true>` is 138 instructions with 8
  `vbroadcastss` outside its chunk loop; `::<8, true>` is 369 with 32.

## Smallest closable slice

Authorized paths: `crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs` (seam export and
`GraphExecutor::output_route_folds()` only), `crates/graph/tests/MUTATIONS.md`,
`tools/console-workload/tests/chain_shape.rs` (one new test),
`docs/rulings/effect-floor-accounting.md` (the sentence at `:527-529`), and this spec.

1. Take #920's bind-time `output_route_fold`, its retire path, `route_table` and the seam
   unchanged (cherry-pick from the branch; re-anchor on `main`).
2. Replace its kernel: `route_reduce::<L>` walks `route_table` in `chunks(2)`; for each pair
   `route_run::<L, 2>` (tail `route_run::<L, 1>` on an odd fan-in) hoists the pair's eight
   coefficients as `L::splat`, walks the pair's four planes and the two master planes by
   `chunks_exact(L::WIDTH)` in one loop, and per chunk computes `mix` of input 0
   (`value = mix(in0)` on the first pair, `L::load(out).add(mix(in0))` after), adds `mix(in1)`,
   stores. No `REDUCE_GROUP` chunking, no per-pair `match` on group length beyond the 2/1
   tail. The tail frames are handled by a separate non-generic `#[inline(never)]` function at
   `f32` (the same body via `route_run::<f32, G>`), never inlined into the `L`-generic kernel:
   the AudioWorklet gate's rule 3 counts scalar `f32` arithmetic inside any `wide::f32x4`
   instantiation, and a same-body tail unrolled three times is 3x the vector count (measured in
   `DRAFTS/PLAN.md`; #920's 560/1,680 was this, not a spill).
3. Write through the Output's `HostMaster` planes (`runtime.rs:324`), read the arena through
   `lease.read` (shared).

## Non-goals

As #920: masters other than the Output op, mixed masters, banked plans (the chain fold stays and
`every_standing_workload_folds_one_route_per_track` must not move), observers on the route or its
aliases, delayed edges. No change to the eligibility rules of #920. No group size other than 2 is
implemented (the measurement is the reason; a later host may re-measure).

## Objective gates

1-5. #920's gates verbatim (64-input bit-identity against the seam-declined render with hostile
   input over frames `{1, 3, 7, 13, 16, 64, 128}` and `output_route_folds() == 64`; the four
   declining shapes; console-workload digests folded vs declined over 64 blocks and the three
   chain-shape tests unchanged; the five red mutations plus one new: *hoist the whole table's
   coefficients before the pair loop* recorded as a `MUTATIONS.md` row (amended by Sol's
   attempt-1 verdict: the callgraph gate cannot see a hoist, since rule 3 counts arithmetic
   families and a hoist moves splats; the row is red on `check-realtime-policy.sh` and on the
   allocation audit instead, and no automated gate sees a kernel that merely spills; only the
   benchmark does); `cargo test -p graph` both ways, `-p graph-compiler`, `-p console-workload`,
   the four policy scripts).
6. **The AudioWorklet callgraph gate in the issue, not at the batch boundary:**
   `scripts/build-web-audioworklet.sh` then `scripts/check-web-audioworklet.sh` (rule 3 in
   `scripts/check-web-audioworklet-callgraph.py:352-359`) must accept the wasm instantiation of
   the fused kernel. This is the gate #920 failed; passing it is a closing condition of this
   issue. Local pre-check before the full gate: `wasm-objdump -d` on the built artifact, count
   `f32x4.{mul,add,sub,div}` against `f32.{mul,add,sub,div}` in every function whose name matches
   `4wide6f32x4` and contains `route`; vector must exceed scalar in each, and the outlined tail
   must appear as its own non-generic function with no `f32x4` arithmetic.
7. The harness `tools/console-workload/tests/plumbing_profile.rs::phase_profile` shows the row's
   `route` phase at zero units and the unit schedule ending in `1 x output` (descriptive, recorded
   in the spec's evidence, not a timing claim).

## Console benchmark rows

Can move: `sixty_four_track_plumbing_only` only (removes the route phase and replaces the
output unit's grouped reduction with the fused pass). `output_route_folds` must read 0 on every
other row.

## Dependencies

After "Write the master straight into the host planes" (#916, merged: the kernel writes host
planes). Convenient after "Lower identity-bound track stages as aliases" (fewer units between
each route and the Output for the in-between scan); not forced, since the route is in place over
its producer's buffer either way.

## Standing rules for the implementer

- Work only from this body. Read the cited functions and #920's branch first; do not survey the
  workspace.
- Class A means the change moves no rendered bit. Every gate above that says "bit-identical" is a
  hard stop, not a tolerance.
- The owner's copy rule: no copy or scratch block anywhere in the fused kernel.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh`
  is mandatory). `crates/graph` stays free of `unsafe` and never names `wide` or intrinsics; it may
  use the `Lane` trait, as `accumulate_run` does.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features
  -- -D warnings`, and the focused tests named above before every checkpoint. Commit on a
  `codex/<issue>-<slug>` branch from synchronized `main`.
- Do not quote a projected saving. The cycle's paired console benchmark runs once at the batch
  boundary.
- The AudioWorklet artifact pin and browser qualification are repinned once at the batch boundary;
  gate 6 runs the callgraph check on a local build without repinning.
- Source of these findings: `.github/ISSUE_SPECS/DRAFTS/PLAN.md`,
  `.github/ISSUE_SPECS/920-fold-in-place-routes-into-the-output-nodes-reduction.md` and its
  verdict, `docs/rulings/effect-floor-accounting.md` ("Plumbing inventory").

## What the implementer will hit

- #920's branch is at `4dd0fb0e` on `origin/codex/920-fold-routes-into-output-reduction`, based
  before the batch merge; `output_route_fold` and its tests need re-anchoring on `main` (the
  `retired` set is at `runtime.rs:4896`, the master-op rewrite at `:5173`).
- The text-pinning tests at `runtime.rs:7081` and `:7173` split on exact function spellings.
- The mix must read both input planes before either accumulator is updated (the in-place route
  read both before writing; here nothing is written to the inputs, so the only hazard is the
  master's own reload, which is per chunk and precedes its store).
- The last pair on an odd fan-in is `route_run::<L, 1>`; keep it the same body with `G = 1`
  rather than a special case, so the tail is covered by gate 1's fan-in shapes.
- `chunks_exact` on six slices in one loop is what LLVM must keep in registers: 8 coefficient
  splats + 2 accumulators + 2 loads + 2 temporaries = 14 of 16 `ymm`. Do not add a third input.
- The tail must be a **separate, non-generic, `#[inline(never)]`** function. Writing it as the
  vector body "at `L = f32`" inside the generic kernel (the pattern every D9 kernel uses natively)
  is exactly what rule 3 rejects on wasm: LLVM unrolls the width-4 scalar tail inside the `f32x4`
  instantiation and the scalar count triples the vector count. Native code is indifferent; the
  wasm artifact is not.

## Attempt 1 evidence

Implementer: Terra (attempt 1). Branch `codex/926-fuse-routes-in-pairs`, from `3c93469d` (the
cycle's briefs commit on synchronized `main`). Local commits only; nothing pushed, no PR, the
AudioWorklet pin untouched.

- `da4a3f44`: #920's code and tests re-applied verbatim (`git show b04e044b -R` over `runtime.rs`,
  `lib.rs` and `chain_shape.rs`: the revert of #920's batch merge, reversed; it applied cleanly,
  `runtime.rs`'s hunks 27 lines down, past the plumbing-profile instrument). A checkpoint so that
  the next commit is a reviewable diff against #920; its kernel still fails rule 3 (below).
- `67649092`: the kernel replaced; the reused code relabelled to #926; the tail-bound unit test;
  the console gate extended to every other standing row.
- `737d6bf1`: comments only (the lane policy forbids the text `wide::` outside `crates/lane`), and
  the floor ruling's sentence.
- The evidence commit: this section and `crates/graph/tests/MUTATIONS.md` rows 926-1 to 926-18.

### Design

**Reused from #920 unchanged** (`da4a3f44`): the bind-time `output_route_fold` (every clause,
including "the plan binds no bank" and the in-between `op_names_buffer` scan), its retire path
through the `retired` set `build_sequential` and `validate_fold_installation` honour, the
`graph.route_fold.master` mutual-exclusion refusal, `Runtime.output_routes: Box<[[f32; 4]]>` in
edge order (mirrored in both layout witnesses), `execute_op`'s `routes` parameter and its
`Some(host) if routes.is_empty()` split, the `Runtime` constructor's route-table `debug_assert`,
the seam `test_only_set_output_route_fold_declined` (exported `#[doc(hidden)]` from `lib.rs`), the
`#[cfg(test)]` accessor `GraphExecutor::output_route_folds()`, the kernel test's oracle, gate 1/2's
`RoutedShape` fixture and test, and the console gate. #920's anchors re-derived on this tree: the
`retired` union is `runtime.rs:5297`, `output_route_fold` `:6850`, the Output branch
`execute_op` `:3289`.

**Replaced: the kernel** (`runtime.rs:599-871`, inside the existing `REALTIME_POLICY` region):

- `route_reduce::<L>` (`#[inline(never)]`, `:648`): refuses before any write a table that does not
  match the inputs, a fan-in below two, or host planes that are not `lease.frames()` words; then
  walks `inputs`/`routes` in `chunks(2)` with a two-arm `match`: a pair is `route_pair::<L, 2>`, an
  odd fan-in's lone last input `route_pair::<L, 1>`. No `REDUCE_GROUP` chunking.
- `route_pair::<L, G>` (`#[inline(always)]`, `:701`): forms the `G` left and `G` right input slices
  with `lease.read` (shared), checks every plane is `frames` words (never fails after
  `route_reduce`'s check; it is what lets the slicing compile without bounds checks), runs the
  vector frames with `route_run::<L, G>`, then the tail frames, when there are any, with
  `route_tail`.
- `route_tail` (`:755`): **non-generic, `#[inline(never)]`**, taking the pair's tail slices and
  table; dispatches to `route_run::<f32, 2>` or `route_run::<f32, 1>`, the same body. It refuses a
  run as long as the widest lane (`<lane::Simd8 as Lane>::WIDTH`, eight): a tail is always
  shorter, and that bound is what stops LLVM vectorising it (measured below).
- `route_run::<L, G>` (`:800`, #920's body, now only ever at `G` = 1 or 2): `table.map(|route|
  route.map(L::splat))` once, before the loop (eight splats for a pair); the pair's four input
  planes and both host planes walked by `chunks_exact(L::WIDTH)` in one loop; the store form
  (`value = mix(in0)`, then `+ mix(in1)`) and the accumulate form (`value = load(out) +
  mix(in0)`, then `+ mix(in1)`) are two loops, as in `accumulate_run`. `mix_chunk` and
  `add_mixed_chunks` are #920's: `(lr.fma(r, ll.mul(l)), rr.fma(r, rl.mul(l)))`.

`route_reduce` is itself never inlined so that its four-lane instantiation stays one named symbol
that rule 3 inspects on every build (#920's was `#[inline]` and happened to stay out of line); the
call is once per block.

### The per-element order argument

Frame `f`, left plane. Before: each route op `R_i` stored `m_i = lr_i.fma(r_i[f], ll_i.mul(l_i[f]))`
(`mix2x2_block::<FrameLane>`, vector body or `f32` tail), then `reduce_many_into` loaded the `m_i`
back and formed `((m_0 + m_1) + m_2) + ... + m_63`, storing after each group of eight and reloading
at the next. Now each `m_i` is computed by the same operation on the same operands in the same
order from the same unmodified `l_i[f]`, `r_i[f]` (the retired route never wrote the buffer), and
fed to the same `add` at the same position of the same chain; the chain is stored after each pair
and reloaded at the next. A store and a load move an `f32`'s bits unchanged, so where the chain is
stored and reloaded moves no bit: groups of two here and groups of eight there are the same chain
(the kernel test compares exactly those two at every fan-in from two to nineteen and sixty-four).
The first contributor is the value, never added to a `+0.0` seed, so `-0.0` survives. `Lane::fma`
is two roundings on every backend, so the width, and whether `f` is a vector or a tail frame,
cannot move a bit. Within a pair the vector frames run before the tail frames; frames are
independent, so every frame still sees the pairs in edge order. The right plane is the same with
`rr`, `rl`. Nothing is written to the inputs, and the host plane is reloaded per chunk before it is
stored, so there is no aliasing hazard.

### Gate 6: the AudioWorklet artifact

Built with `scripts/build-web-audioworklet.sh`'s own cargo invocation, `RUSTFLAGS` and remaps into
a persistent target dir (the script's own build lives in a `mktemp` dir it deletes, and under
`MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1` it prints the digest and exits before copying anything).
The replica's digest equals the one the script printed under `REPIN=1` on both trees it was
compared on: the base, `c3ab811a8f0ba59afe687ba1364f062048511bfba5d255e7c03ffbc0061ef840`, and
`737d6bf1`, `018b7605a1bd8a4d3cb227dd6497d7bbe7e87b1cd84e3d058386bd5a8b748f23`. The committed pin
is `adbda37c...5c09`, so the base tree itself already differs from it, as the batch-boundary
repin expects; the pin file was not written. `wasm-objdump -d` (1.0.34), `f32x4.{mul,add,sub,div}` against
`f32.{mul,add,sub,div}`, counted with the gate's own parser and regexes:

| tree | function | vector | scalar | rule 3 |
|---|---|---:|---:|---|
| `3c93469d` (base) | none matches `4wide6f32x4` and contains `route` | -- | -- | pass (kernels 14, `f32x4_arith` 11,639) |
| `da4a3f44` (#920 re-applied) | `graph::runtime::route_reduce::<f32x4>` | 560 | 1,680 | **FAIL**, #920's failure reproduced |
| pair kernel, no tail bound (scratch) | `route_reduce::<f32x4>` | 44 | 0 | pass |
| | `graph::runtime::route_tail` (non-generic) | 24 | 44 | not in rule 3's set, but not the scalar-only tail the brief asks for |
| `67649092` / `737d6bf1` | `graph::runtime::route_reduce::<f32x4>` | **44** | **0** | **pass** (kernels 15, `f32x4_arith` 11,683 = 11,639 + 44) |
| | `graph::runtime::route_tail` (non-generic) | **0** | 308 | not in rule 3's set; no vector arithmetic |

`route_reduce::<f32x4>` is the only function matching `4wide6f32x4` whose name contains `route`.
Of the other `graph` functions named for routes, only the bind-time `plain_route_gains` carries
arithmetic: one `f32x4.mul`, the route gain folded into the four coefficients. It is not generic
over a lane, so rule 3 does not apply; it has a symbol of its own now that the Output fold is a
second caller (on the base tree it has none).

`route_reduce::<f32x4>`'s 44 are its four loops: the pair's accumulate form 16 (8 `mul`, 8 `add`),
the pair's store form 14, the lone input's accumulate form 8 and store form 6. Its 12 splats are
`i8x16.shuffle` lane broadcasts of the table rows, all ahead of the loops. `route_tail`'s 308 is
44 unrolled seven times (the bound makes seven the maximum trip count). Then:

- `bash scripts/check-web-audioworklet.sh <dir>` over the seven-file set assembled exactly as the
  build script assembles it after its pin check (the replica wasm, the four JS/`.d.ts` files, and
  `parameter-metadata --write`): **exit 0**, every roster row ok, `kernels=15`. The check script
  does not read the pin, so the full gate ran; only the pin comparison of the build script did
  not.
- `python3 -B scripts/check-web-audioworklet-callgraph.py --self-test`: passed.

**What the gate cannot see.** `miso_engine_web_v1_render`'s direct-call closure is eight functions
and stops at `PreparedRenderPlan::render_inner`'s `call_indirect` into the executor; no `graph`
function is in it, so the gate's allocation and trap halves do not reach the render path at all
(on the base tree, `GraphExecutor::render`'s own direct closure reaches `__rust_dealloc` through
`RealtimeObservationActivation::apply_candidate`; pre-existing and outside this issue, and whether
that path is live on render was not examined: reported for a successor). Only rule 3 sees this kernel. Mutation 926-6 is the consequence (below).

On native (`x86-64-v3`, release), `route_reduce::<f32x8>` hoists eight `vbroadcastss` per pair
(four for the lone input) outside the chunk loop; the pair's accumulate loop is 27 instructions per
eight frames: four input loads, eight `vmulps`, eight `vaddps` (two of them taking the host
planes as memory operands), two stores, and five of loop control and the iterator exit check. No
spill: `ymm0`-`ymm13`, no stack access in the loop.

### Tests

- graph `runtime::tests::a_route_reduction_is_the_route_ops_and_the_reduction_bit_for_bit` (kernel;
  #920's, doc updated): `route_reduce::<L>` at `f32`, `Simd4`, `Simd8` against the production route
  op then `reduce_plane_into`; frames `{1, 3, 7, 8, 13, 16, 33, 64}` (tails at both widths, so
  `route_tail` runs for both pair shapes); fan-in 2-19 and 64 (odd fan-ins exercise `G = 1`);
  hostile words and 2x2s; a signed-zero case; the pre-write refusals.
- graph `runtime::tests::a_route_tail_refuses_a_run_as_long_as_the_widest_lane` (new): eight frames
  refused before any write for both pair shapes, seven frames run.
- graph `runtime::tests::an_output_route_fold_is_the_route_ops_and_the_reduction_bit_for_bit` (gates
  1 and 2; #920's): every `RoutedShape`, frames `{1, 3, 7, 13, 16, 64, 128}`, fan-in 64 (32 pairs)
  and for the admitted shapes 2 (one pair) and 9 (four pairs and a lone input); 8 blocks through
  `GraphExecutor::render` into host planes at stride `frames + 3`, bit-identical with padding to
  the seam-declined oracle; `output_route_folds()` is 64 (the fan-in) when admitted, 0 when
  declined and on the oracle; one unit fewer per folded route; every observer window the
  oracle's.
- console-workload `chain_shape::the_plumbing_rows_output_fold_is_the_route_ops_own_bits` (gate 3;
  #920's, extended): the plumbing row as bound against the row seam-declined: unit census smaller
  by exactly 64, bank shape `[0, 0]` and zero chain folds on both, equal 64-block digests; then,
  new, every other standing workload's full unit census (`Vec<PlanUnitEligibility>`) is unchanged
  by the decline, which is `output_route_folds` reading zero there (the engine's plan trait has no
  such count; `crates/engine` is outside the paths).

### Gates

On `737d6bf1` (the evidence commit changes only Markdown):

| gate | command | result |
|---|---|---|
| 1, 2 | the gate test | pass (debug) |
| kernel | the kernel test, the tail test | pass (debug) |
| 3 | `cargo test --locked -p console-workload` | automation 4, `chain_shape` 23, placement 3, `plumbing_profile` 2 ignored; 0 failed. `the_plumbing_row_binds_no_strip_at_all`, `every_standing_workload_folds_one_route_per_track` and `the_folded_master_is_the_reductions_own_bits` pass and their text is unchanged (the branch's one `chain_shape.rs` hunk is the new test). |
| 4 | mutation sweep | below and `MUTATIONS.md` |
| 5 | `cargo test --locked -p graph` | lib 104, rt10 1, rt1 1, rt9 1; 0 failed |
| 5 | `cargo test --locked -p graph --features test-support` | lib 104, rt10 1, rt1 1, rt9 8; 0 failed |
| 5 | `cargo test --locked -p graph-compiler` | 73 + 1 + 3 + 1 + 8 + 6; 0 failed |
| 5 | `bash scripts/check-graph-determinism.sh` | PASS (100/100); evidence JSON sha256 `e5d45be6...a8face`, #916's and #920's value |
| 5 | `check-graph-policy.sh` / `check-realtime-policy.sh` / `check-lane-policy.sh` | PASS / ok (53 marked regions in 15 files) / ok |
| 6 | the artifact gate and the objdump pre-check | pass; table above |
| 7 | `plumbing_profile::phase_profile` | route phase at zero units, schedule ends `1 x output`; table below |
| extra | `cargo test --locked -p host-core --all-features` | lib 86 and every integration suite (`builtin_batch_endpoint` 20, whose two bankless tests #920 found taking the fold, fan-in 3 and 9); 2 pre-existing ignores; 0 failed |
| extra | `cargo check -p graph --target wasm32-unknown-unknown`, with and without `+simd128` | compiles (`FrameLane` = `f32` and `Simd4`) |
| std | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean, exit 0 (159 crates checked; the two pre-existing `clippy.toml` "does not refer to a reachable function" configuration warnings are not lints) |

### Mutations

`crates/graph/tests/MUTATIONS.md`, "Issue #926", has all eighteen rows with their failures. The
brief's six:

| # | mutation | kernel test | gate 1 | console gate | artifact gate |
|---|---|---|---|---|---|
| 926-1 | reverse the accumulation order (within each pair) | RED (fan-in 6, one ulp) | RED | RED | -- |
| 926-2 | seed the first pair from `+0.0` | RED (`-0.0` case) | RED (`NegativeZero`) | GREEN: no all-`-0.0` frame on the row | -- |
| 926-3 | apply the 2x2 to the running sum | RED | RED | GREEN: every console route is the identity 2x2 | -- |
| 926-4 | swap the coefficient roles | RED | RED | RED | -- |
| 926-5 | store in every pair | RED (fan-in 3) | RED | RED | -- |
| 926-6 | hoist the whole table's coefficients before the pair loop | GREEN | GREEN | **RED** (SIGABRT: the render-scope allocation audit) | **GREEN** (exit 0; 44 / 0) |

**926-6 is not red by the callgraph gate, and the gate as written cannot make it red.** The
brief expected it to be. A whole
table's splats need storage sized by the fan-in, so the natural hoist is a `Vec` on the render
path. Rule 3 counts arithmetic families, and a hoist moves splats, not arithmetic: the kernel
still reads 44 vector and 0 scalar. The gate's allocation half walks `miso_engine_web_v1_render`'s
direct calls, which stop at the executor's `call_indirect`, so it never sees the `Vec`. What does
refuse the row: `scripts/check-realtime-policy.sh` (`.collect(` in a marked region) and the console
gate, whose `bench-support` allocator aborts on an allocation inside the render scope. On wasm a
hoisted-coefficient spill is also invisible to rule 3 (a spill emits no `f32` arithmetic), which is
the brief's own root evidence; the register-pressure half of the reason for pairs is a native
measurement (table C), not something gate 6 can check.

The pair kernel's own rows: **926-7** (tail inlined, #920's shape) is RED on the artifact gate at
rule 3 (`route_reduce` 44 vector / 176 scalar) and GREEN on every native test; **926-8** (tail
bound dropped) is RED on the tail test and on the objdump pre-check (`route_tail` 24 vector) and
GREEN on the artifact gate (a non-generic name is outside rule 3's set); **926-18** (odd fan-in's
lone input skipped) is RED on the kernel test (fan-in 3) and gate 1 (fan-in 9). #920's structural
rows re-run as 926-9 to 926-17: each RED on a count and, with the counts deleted, on the host planes
or an observer window (926-15 at bind, on five graph tests and the console gate's other-rows sweep);
926-14, the in-between scan, is GREEN as it was for #920 (no test can make it fire; its doc says
why).

### Gate 7: the profile harness (descriptive, not a timing claim)

`CARGO_INCREMENTAL=0 taskset -c 31 cargo test --release -p console-workload --test
plumbing_profile phase_profile -- --ignored --nocapture --test-threads 1`, once on the base tree
and once on `67649092`, same host (AMD EPYC 7313P, cpu 31, two other implementers building on
other cores), three repeats each. Calibrated clock 3.693 / 3.695 GHz.

Unit schedule: base `64 x bound, 128 x identity-copy, 64 x identity-alias, 64 x route, 1 x output`;
#926 `64 x bound, 128 x identity-copy, 64 x identity-alias, 1 x output`. The route phase is at zero
units.

| phase (cycles/block, probes on, repeats 0 / 1 / 2) | base `3c93469d` | #926 `67649092` |
|---|---|---|
| enter | 103 / 103 / 104 | 103 / 103 / 103 |
| source set | 169 / 169 / 171 | 166 / 170 / 166 |
| bound units (64) | 8,355 / 8,359 / 8,585 | 8,257 / 8,290 / 8,259 |
| route units | 8,162 / 8,182 / 8,357 (64 units) | -- (0 units) |
| output unit (1) | 3,529 / 3,524 / 3,572 | 4,453 / 4,439 / 4,459 |
| identity-copy units (128) | 16,797 / 16,791 / 17,105 | 16,854 / 16,835 / 16,786 |
| identity-alias units (64) | 3,814 / 3,819 / 3,950 | 3,928 / 3,925 / 3,935 |
| **probes off, p50** | 10,390 / 10,420 / 10,420 ns (38,367-38,478 cycles) | 8,446 / 8,436 / 8,446 ns (31,168-31,205 cycles) |

Build-to-build layout variance on this row is about 4 % (the plan's table A). The output unit now
carries the routes' arithmetic; the paired console benchmark at the batch boundary is the row's
number, not this.

### Deviations

1. **`route_reduce` is `#[inline(never)]`**, not `#[inline]` as #920 wrote it: it keeps the fused
   kernel's four-lane instantiation a named symbol rule 3 inspects, whatever LLVM's inliner does
   later. One call per block.
2. **`route_tail` refuses a run as long as the widest lane.** Not in the brief; without it LLVM
   vectorised the outlined tail's two accumulate loops (24 `f32x4` operations), and the brief
   asks for a tail with no vector arithmetic. The refusal is unreachable (a tail is shorter than
   `L::WIDTH`, at most eight) and pinned by its own test; if it were reached, `route_reduce`
   returns `false` and `execute_op` silences the host planes, as for every other refusal.
3. **The tail runs per pair** (vector frames, then that pair's tail frames), not once after all
   pairs: the tail needs the pair's input slices, and forming them there would repeat the
   `lease.read` calls in a second function. Frames are independent, so this is the same chain.
4. **926-6 is not red by the callgraph gate** (above); it is red by the realtime policy and the
   render allocation audit, and the row records exactly that.
5. **The console gate also pins zero folds on every other standing row**, by census, inside the one
   new test the brief allows.
6. **#920's structural mutations were re-run**, as 926-9 to 926-17, rather than cited: the brief
   reuses them, and they now run against this tree.

### Anchor drift

The brief's `runtime.rs` anchors past `:2330` are 27 lines early (the plumbing-profile instrument,
`f49fdbde`): the `Route` arm is at `:2980` (cited `:2953-2956`), the `retired` set at `:4923`
(`:4896`), the master-op rewrite at `:5200` (`:5173`), the text-pinning tests' `include_str!`
at `:7150` and `:7251` (`:7081`, `:7173`). `HostMaster` `:324`, `REDUCE_GROUP` `:385`,
`accumulate_group` `:451`, `accumulate_run` `:480`, `reduce_plane_into` `:534`, `program.rs:716`,
`kernels.rs:734` and the callgraph script's `:352-359` are as cited. The brief's
`DRAFTS/PLAN.md` is `docs/handoffs/plumbing-floor-2026-09-26/PLAN.md` on this tree.

### Risks and notes

- No performance claim; the table above is descriptive.
- `route_tail` is 1,987 wasm instructions (the seven-way unroll), cold on a 128-frame quantum.
- The kernel's per-chunk iterator exit checks survive on both targets (three `br_if` per chunk on
  wasm, one `cmp`/`je` on native); removing them is loop-overhead work for a later measurement.
- The route table is a bind-time allocation of 16 bytes per Output input, charged nowhere, as
  #920 noted.
- Not run: browser qualification and the pin (batch boundary), `-p bench`, `-p audit`, the
  paired console benchmark.

## Sol attempt 1 verdict: PASS

Adversarial review (Fable 5.1, high effort) against `da4a3f44`, `67649092`, `737d6bf1` and
`0a54330c` on base `3c93469d`. No blocking findings. Bit identity read against the base kernels:
`mix_chunk` is `mix2x2_block`'s two expressions operand for operand; the first pair stores
`mix(in0)` then adds `mix(in1)`, later pairs reload the running sum and add in edge order, the
lone last input is the same body at `G = 1`, and a planar `f32` store and reload is exact on every
target, so pair boundaries and the base's group-of-eight boundaries are the same rounding chain;
`Lane::fma` is unfused, so width cannot move a bit. The reviewer rebuilt the simd128 artifact
(digest `018b7605...8f23`, equal to the implementer's) and counted with the gate's own parser:
`route_reduce<f32x4>` 44 vector / 0 scalar, `route_tail` a separate non-generic symbol at 0 / 308,
kernel shape 15 kernels, the render closure 8 functions, `check-web-audioworklet.sh` exit 0.
The `route_tail` length bound is unreachable (the tail is `frames % L::WIDTH`) and its refusal
matches `reduce_many_into`'s existing refuse-and-silence. Eligibility holds on the shape #925
produces (`Input -> Route -> Output`): the fold asks nothing of the route's producer beyond
`input_producers`. Five mutations re-applied and reverted, red as recorded; the tail-inlined
mutation reproduces #920's failure mode at 44 / 176. Reviewer-run gates, all green: `cargo test -p
graph` (both configurations), `-p console-workload`, `-p graph-compiler`, `-p host-core
--all-features`, fmt, graph clippy, determinism (100/100), graph, realtime and lane policy.

Recorded for the batch merge: #925's console test `the_plumbing_row_is_input_route_output_and_
renders_the_base_bits` pins 129 units and `symmetry_counters` `[65, 129]`; with the 64 routes
retired those become 65 and `[65, 65]` (or whatever the merged tree reports; the digest assertion
is what matters and passes on the scratch merge). Recorded nits, no change: no automated gate can
see a kernel that spills (only the benchmark can); the `route_tail` bound is a compiler-steering
trick no gate pins (bits cannot move either way). Pre-existing, for a successor issue: the wasm
callgraph reaches `__rust_dealloc` from `GraphExecutor::render` through
`RealtimeObservationActivation::apply_candidate`'s drop of a `pending` snapshot that the
ownership protocol keeps `None` on every render, so the path is dynamically dead but statically
live. Not re-verified: the native assembly claims, the 560/1,680 reproduction on `da4a3f44`,
mutations 926-2/3/5/6 and 926-9..17, workspace-wide clippy, `-p bench`, `-p audit`.
