# Fuse in-place routes into the Output reduction in pairs

Supersedes #920 (withheld: its `route_run::<L, 8>` kernel hoisted 32 splatted coefficients and
sixteen iterators into one loop, failed the AudioWorklet callgraph gate with vector 560 against
scalar 1,680 arithmetic, and did not move the row). The eligibility half of #920 stands; the
kernel is re-shaped from the measurement in `DRAFTS/PLAN.md`, Part 2, table C.

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
- `program.rs:691-700`: a plain route with a single undelayed sole-read input is lowered in place
  over that buffer, which stays coloured until its last reader, the Output op.
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
   stores; tail frames at `L = f32` by the same body. No `REDUCE_GROUP` chunking, no
   per-pair `match` on group length beyond the 2/1 tail.
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
   coefficients before the pair loop* must fail a new `MUTATIONS.md` row by the callgraph gate
   below; `cargo test -p graph` both ways, `-p graph-compiler`, `-p console-workload`, the four
   policy scripts).
6. **The AudioWorklet callgraph gate in the issue, not at the batch boundary:**
   `scripts/build-web-audioworklet.sh` then `scripts/check-web-audioworklet.sh` (roster in
   `scripts/check-web-audioworklet-callgraph.py`) must accept the wasm instantiation of the fused
   kernel. This is the gate #920 failed; passing it is a closing condition of this issue.
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
