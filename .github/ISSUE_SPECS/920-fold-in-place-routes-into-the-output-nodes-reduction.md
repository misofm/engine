# Fold in-place routes into the Output node's reduction

## Product outcome
`sixty_four_track_plumbing_only` pays sixty-four dispatched route ops, each a `mix2x2_block` store pass over its in-place buffer, then the Output op's eight-input `reduce_group`s reload them. Retire every plain route whose sole reader is the Output op and give that op a fused kernel: per group of up to eight inputs, load both planes of each input, apply its 2x2 in registers, accumulate in edge order with the first contributor stored. Class A: the same per-element sequence as `mix2x2_block` then `reduce_group`; sixty-four fewer dispatches and store passes.

## Root evidence
- `crates/graph/src/program.rs:691-700`: a plain route with a single undelayed sole-read input is lowered in place over that buffer; the buffer stays coloured until its last reader, the Output op.
- `runtime.rs:2332` `execute_op` reduces (`reduce_plane` `:298`, groups of `REDUCE_GROUP` via `reduce_group` `:355`) then runs `NodeKind::Route` as `mix2x2_block::<FrameLane>` in place (`:2401-2404`). Per element, `crates/lane/src/kernels.rs:734` and `:644` fix the order: `lr.fma(r, ll.mul(l))`, `rr.fma(r, rl.mul(l))`, then `value = in0` (group 0) or `value = load(out)`, then `value.add(in_i)` in edge order.
- `route_fold` (`:5237`) already has the helpers: `op_dataflow` (`:5460`, readers and first producers), `input_producers`, `plain_route_gains` (`:5008`), `observed` (`:5033`, covers `program::Tap` aliases), `op_names_buffer`, the `retired` set that `build_sequential` honours (`:4313-4315`, `:4450`), and the master-op rewrite `inputs = vec![arena(op.output.0)]` (`:4362-4364`).
- After "Write the master straight into the host planes", the Output op writes the host planes, so the fused kernel forms its N inputs with `lease.read(plane, input)` (`disjoint.rs:198`, shared) and needs no arena accessor.
- `tools/console-workload/tests/chain_shape.rs:838-872` `the_plumbing_row_binds_no_strip_at_all` pins the row's bank shape `[0, 0]`; the seam `graph::test_only_set_route_fold_declined` (`lib.rs:35`) is the model for a decline switch, and console-workload already enables `graph/test-support` for its tests.
- `docs/rulings/effect-floor-accounting.md:527-529` states the row "pays sixty-four individually dispatched route ops and an unfolded reduction"; this issue supersedes that sentence.

## Smallest closable slice
Authorized paths: `crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs` (seam export and a `GraphExecutor::output_route_folds()` accessor only), `crates/graph/tests/MUTATIONS.md` (new rows), `tools/console-workload/tests/chain_shape.rs` (one new test), `docs/rulings/effect-floor-accounting.md` (that sentence), and this spec.
1. Bind-time `output_route_fold(program, spec, parts)`: take the op `M` whose output is `program.output`; require `NodeKind::Identity`, no sidechain, no split pair, no delayed input, not a bank member, fan-in >= 2. For every input position, the producer `R` (from `input_producers`) must be a `plain_route_gains` route with one undelayed input, `readers[R] == [M]`, `R.output` equal to its own input buffer (in place), and `!observed(.., R, served_by_nothing)`; run `route_fold`'s in-between `op_names_buffer` scan over the units between `R` and `M`. Any failure declines the whole fold. On success, retire every `R` (reuse the `retired` mechanism) and give `M` a `route_table: Box<[[f32; 4]]>` in edge order; counter `output_route_folds`.
2. `execute_op`: when `route_table` is non-empty, replace the two `reduce_plane` calls by `route_reduce::<FrameLane>`: per group of `REDUCE_GROUP` inputs, per plane, per `chunks_exact(L::WIDTH)` position, `value = mix_p(in0)` if group 0 else `L::load(out)`, then `value = value.add(mix_p(in_i))`, store; tail at `L = f32`. `mix_left = lr.fma(r, ll.mul(l))`, `mix_right = rr.fma(r, rl.mul(l))` from `L::load` of both input planes. Write through the Output's host planes.
3. Seam `test_only_set_output_route_fold_declined(bool)`, bind-time only, exported `#[doc(hidden)]` beside `test_only_set_route_fold_declined`.

## Non-goals
Masters other than the Output op (arena `out` needs an arena accessor), mixed masters (a non-route contributor declines the whole fold; an identity 2x2 is not class A: `0*r` flips `-0.0`), banked plans (the chain fold stays and `every_standing_workload_folds_one_route_per_track` must not move), observers on the route or its aliases, delayed edges.

## Objective gates
1. New graph test: a 64-input Output with in-place routes, random per-route 2x2s, hostile input (signed zeros, subnormals, magnitudes `2^-24..2^25`), frames in `{1, 3, 7, 13, 16, 64, 128}`: host planes bit-identical to the seam-declined render of the same plan over 8 blocks; `output_route_folds() == 64`.
2. Same test, declining shapes each give counter 0 and identical output: a `PostMatrix` observer whose tap aliases a route; one delayed edge; one submix (Identity) contributor; a route feeding a submix that feeds the Output.
3. console-workload: `sixty_four_track_plumbing_only` folded vs seam-declined digests equal over 64 blocks; `the_plumbing_row_binds_no_strip_at_all`, `every_standing_workload_folds_one_route_per_track` and `the_folded_master_is_the_reductions_own_bits` unchanged.
4. Red mutations in `MUTATIONS.md`: reverse the accumulation order; zero-seed instead of taking `in0` as `value`; apply the 2x2 to the running sum instead of the input; swap coefficient roles (`ll.fma(r, lr.mul(l))`); apply group 0's `initial_store` to every group.
5. `cargo test -p graph` (with and without `test-support`), `-p graph-compiler`, `-p console-workload`; `scripts/check-graph-determinism.sh`, `check-graph-policy.sh`, `check-realtime-policy.sh`, `check-lane-policy.sh`.

## Console benchmark rows
Can move: `sixty_four_track_plumbing_only` only. No saving is projected; the row is measured once at the batch boundary.

## Dependencies
After "Write the master straight into the host planes" (forced: the kernel writes host planes and reads the arena through shared borrows only). Independent of every other issue in the cycle.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate above that says "bit-identical" is a hard stop, not a tolerance.
- The owner's copy rule: a block-sized copy on the render path exists only with a written justification that no in-place or direct-write form exists. This issue removes sixty-four store passes; do not introduce a copy or a scratch block anywhere in the fused kernel.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). `crates/graph` stays free of `unsafe` and never names `wide` or intrinsics; only `crates/lane` may (`scripts/check-lane-policy.sh`). `graph` may use the `Lane` trait, as `reduce_group` does.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named above before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. The cycle's paired console benchmark runs once at the batch boundary, not per issue.
- The AudioWorklet artifact pin (`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`) and browser qualification are repinned once at the batch boundary; do not repin here.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (§1.3, O9, PR #879), `docs/rulings/effect-floor-accounting.md` ("Plumbing inventory") and tracker #349.

## What the implementer will hit
- The retire path assumes an `Op` unit per route; `build_sequential:4313-4315` already skips a unit whose ops are all retired.
- `foldable_lane`'s `program.output` clauses (`:5141`) are about a colouring collision that dedication removes; do not copy them.
- The text-pinning tests at `runtime.rs:6088-6095` and `6180-6192` split on exact function spellings.
- Colouring safety: `R`'s in-place buffer is owned from `R`'s position to `M` (its last reader), so retiring `R` leaves the source's words in place; the in-between scan is the belt to that brace, as in `route_fold`.
