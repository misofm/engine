# Fuse the fold epilogue into the scatter transpose

## Product outcome

On a full bank whose every active lane folds (the console shape since #885), the epilogue is two
passes over every transposed tile: `tile_scatter` stores the tile into the chain's `staging_*`
block, then `fold_cohort` re-reads each lane's staging plane for `mix2x2_block` and re-reads it
again in `ordered_accumulate_block`. Route and accumulate each lane's tile the moment the
transpose produces it, straight into the master, and never write `staging_*` on that path. Class
A: same arithmetic in the same per-element order, one pass instead of three, no staging store or
reload.

## Root evidence

- `crates/rack/src/lib.rs:2666` `scatter_tiled`: with a non-empty fold it calls `tile_scatter`
  (`:290`) for both planes into `staging_left`/`staging_right` (`:1619-1621`), copies the ragged
  tail word by word (`:2731-2738`), then, when `all_active_folded`, builds a `FoldCohort`
  (`:1322`) over the staging block and calls `members.fold_cohort` (`:2788`). The direct scatter
  (`tile_scatter_direct_plane`, `:310`) is taken only when the fold is empty.
- `crates/graph/src/runtime.rs:1358` `ArenaMembers::fold_cohort`: `mix2x2_block::<FrameLane>`
  over each lane's whole staging plane (`:1404-1409`), then `ordered_accumulate_block::<FrameLane>`
  over all `count` planes into `lease.write_stereo(self.master)` with `initial_store =
  stores[0]` (`:1418-1428`). `fold_plane` (`:1345`) is the per-lane form for the mixed case.
- The per-element arithmetic the fused pass must reproduce, verbatim from
  `crates/lane/src/kernels.rs:734` `mix2x2_block`: `l' = lr.fma(r, ll.mul(l))`,
  `r' = rr.fma(r, rl.mul(l))` on the frame's original `l`, `r`; then from
  `ordered_accumulate_block` (`:644`): the first contributor is stored when `initial_store`,
  otherwise the running master is loaded, and every contributor is added in lane order with
  `L::add`. Frames are independent, so a `W`-wide row sees exactly the ops a `frames`-wide pass
  sees; `FrameLane` is `Simd8` on x86-64-v3 and `Simd4` on wasm (`runtime.rs:273-287`). On x86
  `FrameLane` is `Simd8` even for a W4 bank, so a W4 tile computes 4-wide where today's staging
  pass computes 8-wide; the bits are identical because the lane contract is unfused everywhere
  (`Lane::fma` is `(a * b) + c` on every backend) and frames are independent. Gate 1's W4 arm
  rests on exactly that.
- Rack cannot call `lane` directly: `scripts/check-rack-policy.sh` pins its dependencies to
  `effect-contract` and `engine`, and `effect-contract` re-exports no kernel. The fused kernel
  therefore lives in `graph` (which depends on `lane` and may use the `Lane` trait, as
  `reduce_many` at `runtime.rs:325` does), fed by the resident AoSoA scratch;
  `effect_contract::transpose_tile_4/8` (`crates/effect-contract/src/lib.rs:301`) are the same
  transposes rack uses.
- Bit-identity gates already exist: `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit`
  (`runtime.rs:7739`), `the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign`
  (`:7988`), `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit`
  (`:9235`), and console-workload `the_folded_master_is_the_reductions_own_bits`
  (`tools/console-workload/tests/chain_shape.rs:903`, oracle through
  `graph::test_only_set_route_fold_declined`).

## Smallest closable slice

Authorized paths: `crates/rack/src/lib.rs` (`BankMembers`, `scatter_tiled`, a new resident
cohort view type), `crates/graph/src/runtime.rs` (`ArenaMembers` and its tests),
`crates/rack/tests/MUTATIONS.md` and `crates/graph/tests/MUTATIONS.md` (new rows only), and this
spec.

1. In rack, add `pub struct ResidentFoldCohort<'a>` carrying the two resident scratch planes
   (`&self.scratch.left[..frames * W]`, `&self.scratch.right[..frames * W]`), `W`, `frames`, and
   the lane order `0..W`. Add to `BankMembers` a method
   `fn fold_resident(&mut self, cohort: ResidentFoldCohort<'_>) -> bool { false }` (default:
   declined). In `scatter_tiled`, offer `fold_resident` only when **every lane of `0..W` is
   folded** (`self.fold[lane]` true for all `W` lanes) -- not merely `all_active_folded`, whose
   loop today still `plane_mut`-copies an unfolded lane (`rack:2764-2769`). Call
   `members.fold_resident(..)` **before** any `tile_scatter`; if it returns `true`, return without
   touching `staging_*`. If it returns `false`, take today's staging path unchanged. The cohort
   is therefore always lanes `0..W` in order, so `stores[0]` is lane 0's flag exactly as today.
   The partial bank path (`scatter`, `:2599`) and the mixed folded/unfolded lane path are not
   changed.
2. In graph, implement `fold_resident` on `ArenaMembers`: hoist every lane's four coefficient
   splats and `store` flags once per cohort; for each `W`-frame tile of both planes, build the
   `[[f32; W]; W]` rows, transpose with `effect_contract::transpose_tile_{4,8}`, and for lanes in
   `0..W` compute the mixed row with the two `Lane` expressions quoted above and store it into
   (first lane with `store`) or add it into (every other lane, `L::add(master, mixed)`) the
   master's `W` words at that tile. For the ragged tail (`frames % W` frames) run the same
   expressions at `L = f32`. Every premise (`W` equals the lane count, `count <= 8`, `frames <=
   lease.frames()`, `lease.writes(master)`, all lanes folded, only lane 0 may carry `store`) is
   checked before the first write, returning `false` on failure so rack falls back.
3. Keep `fold_plane` and `fold_cohort` as they are (they serve the mixed and partial paths).

The master remains `lease.write_stereo(self.master)`; if the host-planes issue below lands first,
write through whatever master accessor it introduced.

## Non-goals

No change to the partial-bank scalar scatter (that is "Tiled gather and scatter for partial
banks"), to `fold_plane`, to the route eligibility predicates, to the transposes, to the mono
collapse, or to the staging block's allocation. No new lane-crate kernel.

## Objective gates

1. New graph test: for random hostile input (signed zeros, subnormals, magnitudes over
   `2^-24..2^25`), random per-lane 2x2s, `W4 x 4` and `W8 x 8` full cohorts, one and two cohorts
   into one master, frames in `{W, 13, 16, 128}` (ragged tails included), `initial_store` both
   ways: the master produced by `fold_resident` is bit-identical to the master produced by the
   existing `tile_scatter` + `fold_cohort` path run on the same words, and the staging block is
   untouched (pre-filled with a NaN pattern, as `full_bank_gather_scatter_round_trip_is_bit_exact`
   at `rack/src/lib.rs:4408` does).
2. `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit`,
   `the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign`,
   `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit`, and
   console-workload `the_folded_master_is_the_reductions_own_bits` and
   `every_standing_workload_folds_one_route_per_track` pass unchanged. Note which of these reach
   the fused path: `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit` calls
   `members.fold_cohort` directly (`runtime.rs:7801-7805`) and never enters `scatter_tiled`, so
   it cannot guard `fold_resident`. The gates that do reach it are
   `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit` (W4x4 at 13
   frames, W8x8 at 13, two W4 cohorts at 16), console-workload's
   `the_folded_master_is_the_reductions_own_bits`, and gate 1.
3. Red mutations recorded in the two `MUTATIONS.md` files: swap the coefficient roles
   (`ll.fma(r, lr.mul(l))`), accumulate lanes in reverse, seed a non-first lane from zero, skip
   the ragged tail, and return `true` from `fold_resident` after writing nothing. (Swapping the
   fma *operand* order, `ll.fma(l, lr.mul(r))`, is not red: `Lane::fma` is unfused everywhere
   and IEEE addition is commutative; do not record it as a catcher.)
4. `cargo test -p rack -p graph` (with and without `--features test-support`),
   `cargo test -p graph-compiler -p console-workload`, `scripts/check-graph-determinism.sh`,
   `scripts/check-graph-policy.sh`, `scripts/check-rack-policy.sh`,
   `scripts/check-realtime-policy.sh`, `scripts/check-lane-policy.sh`.

## Console benchmark rows

Can move: every row that binds full bank cohorts and folds -- `sixty_four_track_console`,
`_eq_only`, `_compressor_only`, `_builtins_only`, `_dispatch_only`, `_idle`, `_gain_pan_only`,
the three mono rows, both `console_meters` arms (since #885) and the hoist/observation/automation
arms. Cannot move: `sixty_four_track_plumbing_only` (binds no chain: nothing to fold) and the
partial ninth cohort of `nine_track_ragged_strip` (scalar staging path). No saving is projected.

## Dependencies

None to implement. Merge before "Write the master straight into the host planes", which edits the
same `ArenaMembers` master write sites; the later of the two rebases.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate above that says "bit-identical" is a hard stop, not a tolerance.
- The owner's copy rule: a block-sized copy on the render path exists only with a written justification that no in-place or direct-write form exists. This issue removes one; do not introduce another (no scratch copy of the tile or the master).
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`); `graph` may use the `Lane` trait and `effect_contract::transpose_tile_*`.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named above before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence. The cycle's paired console benchmark is run once at the batch boundary, not per issue.
- The AudioWorklet artifact pin (`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`) and browser qualification are repinned once at the batch boundary; do not repin here.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (O3, PR #879) and tracker #349.

## What the implementer will hit

- `full_bank_gather_scatter_round_trip_is_bit_exact` (`rack:4408`) pins staging untouched on the
  *unfolded* direct path; it stays green. No test pins that staging is written on the folded path.
- `rack`'s own `BankMembers` test implementors take the default `fold_resident` and keep the
  staging path, so rack's fold tests stay valid; the graph implementor is the only real one.
- `FoldCohort::new` shape checks (`rack:1338-1357`) are the model for the resident view's checks.
- `tools/console-workload/tests/chain_shape.rs` counts folds and transposes, not staging writes;
  the counters must not move.
