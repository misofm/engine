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

## Attempt 1 evidence

Implementer: Terra (attempt 1). Branch `codex/915-fused-fold-epilogue`. Implementation checkpoint:
`62c76b08`. Every gate below ran on that code; the evidence commit changes only the two
`MUTATIONS.md` files and this section.

### Design

**Rack.** `crates/rack/src/lib.rs` gains `pub struct ResidentFoldCohort<'a>`, a shared view with
these parts:

- The two resident scratch planes, exactly `frames * W` words each.
- The bank's `BankWidth`, which is `W`.
- `frames`.

Its accessors are `left`, `right`, `width`, `lanes` (= `W`), `lane_ids` (always `0..W`, ascending)
and `frames`. `ResidentFoldCohort::new` is modelled on `FoldCohort::new`. It returns `Overflow` if
`frames * W` overflows, and `Shape` for zero frames or a plane of any other length.
`BankMembers::fold_resident(&mut self, ResidentFoldCohort<'_>) -> bool` defaults to `false`.

`scatter_tiled` makes the offer at one point: after the unchanged unfolded direct-scatter block,
and before any `tile_scatter`. It offers only when `self.fold.len() == W` and every entry is
`true`, so every lane of `0..W` folds, not merely `all_active_folded`. An accepted offer returns at
once, so neither `staging_*` nor any plane is written. A declined offer, or a failed constructor,
falls through to today's staged path byte for byte. These are unchanged:

- the partial-bank `scatter`
- the mixed folded/unfolded lane loop
- `fold_plane`
- `fold_cohort`
- the transposes
- the staging allocation

**Graph.** `ArenaMembers::fold_resident` in `crates/graph/src/runtime.rs` dispatches on the
cohort's width:

- W4 runs `fold_resident_tiles::<4, lane::Simd4>` with `transpose_tile_4`.
- W8 runs `fold_resident_tiles::<8, lane::Simd8>` with `transpose_tile_8`.

The row lane is always exactly `W` wide. On x86 a W4 tile therefore computes 4-wide where the
staged pass computed `FrameLane` = `Simd8`-wide, which the brief anticipates.

Every premise is checked before the first write, and a failed one returns `false`:

- `W <= 8`
- `L::WIDTH == W`
- `cohort.lanes() == W`
- `self.fold.len() == W`. The graph gives a folded chain one `FoldLane` per lane and an unfolded
  chain none, so this is also "every lane folds".
- `1 <= frames <= lease.frames()`
- both planes are exactly `frames * W` words
- `lease.writes(master)`
- no lane after lane 0 carries `store`

Once per cohort it hoists `initial_store = fold[0].store`, every lane's constants as `[[f32; 4]; W]`
and their splats as `[[L; 4]; W]`. It takes the master once, through
`lease.write_stereo(self.master)`.

For each whole `W`-frame tile, the kernel does this:

1. It builds the `W` frame rows of both planes, exactly as `tile_scatter` does.
2. It transposes them with the same `effect_contract` transpose.
3. It calls `fold_words::<L, W>` on the master's `W` words for that tile.

For the ragged tail (`frames % W` frames) it calls `fold_words::<f32, W>` on one master word,
reading lane `k` of frame `f` straight from resident word `f * W + k`. `fold_words` works like this:

1. It routes lane 0.
2. The running value becomes that routed word when `initial_store`, otherwise
   `L::load(master).add(routed_0)`.
3. For each lane `k` in `1..W`, in order, it sets `running = running.add(routed_k)`.
4. It stores `running` once.

`route_word` is `(lr.fma(right, ll.mul(left)), rr.fma(right, rl.mul(left)))`.

**Why each element equals `mix2x2_block` followed by the accumulate.** Both replaced kernels are
frame-independent:

- `mix2x2_block` computes a frame's `l'` and `r'` only from that frame's original `l` and `r` and
  the lane's constants, as `lr.fma(r, ll.mul(l))` and `rr.fma(r, rl.mul(l))`.
- `ordered_accumulate_block` computes a master word only from that frame's contributors. It takes
  `c0` (store) or `out + c0` (no store), then adds `c1`, `c2`, ... in order, running value on the
  left.

So each master word is one fixed expression over its frame's words. `route_word` and `fold_words`
evaluate that expression with the same operations, the same operands in the same operand
positions, and the same association. Evaluating it `W` frames at a time, or one frame at `L = f32`,
instead of `FrameLane` frames over the whole staging plane, cannot change it. Two facts back this:

- The lane contract is IEEE per element on every backend.
- `Lane::fma` is the unfused `(a * b) + c` everywhere, and Rust never contracts it.

The staged path also stores every routed word to staging and reloads it. A store and a load move an
`f32`'s bits unchanged, so keeping the word in a register moves no bit either. The one reordering
is in time, not in dataflow: lane `k`'s route now runs just before its add, where before all routes
ran first. Each operation is a pure function of its operands, so this cannot move a bit.

**What goes, per fully folded bank per block, with no new copy:**

- the transpose's stores into `staging_*` (`2 * W * frames` words)
- the ragged-tail staging stores
- `mix2x2_block`'s load and store pass over the staging block
- `ordered_accumulate_block`'s reload of the staging block
- the per-lane `fold[lane]` loop
- the `FoldCohort` construction and its checks
- `fold_cohort`'s premise checks, replaced by one premise check per cohort

The master traffic is what `ordered_accumulate_block` did: one store per word, plus one load on a
continuation. The only tile-sized array is the transpose's own `W x W` row input, as in
`tile_scatter`. No performance claim is made. The cycle's paired console benchmark runs once, at
the batch boundary.

### Tests added

- rack `tests::a_fully_folded_full_bank_offers_its_resident_block_before_writing_staging`: the rack
  half of gate 1. It runs at both widths over every `frame_shapes` shape: 1, `W - 1`, `W`, `W + 1`,
  `3W + 2` and 128. The words are hostile and pass through `ScaleByLane`. It checks four things:
  1. The offered block is the unarmed scatter's words, bit for bit. The offer carries lanes `0..W`
     in order and exactly `frames` frames.
  2. An accepted offer leaves both staging planes at their NaN fill (`0x7fc0_3915`/`0x7fc0_3916`)
     and writes no plane. It calls neither `fold_plane` nor `fold_cohort`.
  3. A declined offer renders exactly what a provider without `fold_resident` renders, through one
     `fold_cohort` over `0..W`. Staging then holds the staged transpose.
  4. A mixed mask, an unarmed chain and a partial bank are never offered the block.
- rack `tests::resident_fold_cohort_constructor_rejects_every_invalid_shape`.
- graph `runtime::tests::a_resident_fold_is_the_staged_scatter_and_cohort_fold_bit_for_bit` (gate 1):
  - **Shapes.** 32 cases: W4 x 4 and W8 x 8, frames in `{W, 13, 16, 128}`, one cohort and two
    cohorts into one master, `initial_store` both ways (`false` accumulates onto a live hostile
    prior master).
  - **Corpus.** Samples are hostile and finite: `±0.0`, signed subnormals, and signed normals over
    `2^-24 .. 2^25`. Every lane gets its own random 2x2, including signed-zero entries.
  - **Arms.** Both arms run the real `BankChain::run` over the same arena layout. The only
    difference is that the oracle arm declines the offer. The oracle therefore takes the real
    `tile_scatter` and tail copy into staging, then `ArenaMembers::fold_cohort`. The scratch is
    sized for 128 frames, so the staged stride differs from `frames` on every shorter block.
  - **Assertions.** Both master planes are bit-identical across the two arms. The counters show the
    fused arm accepted every offer and the oracle took `fold_cohort` every time. No lane's own
    output buffer is written, and the master moved.
- graph `runtime::tests::a_resident_fold_declines_before_writing_on_a_broken_premise`: a control
  that accepts, then five broken premises, each declining with the master untouched. The five
  are: one fold entry short, an unfolded chain, a later lane storing, a block longer than the
  lease, and a master outside the write set.
- graph
  `runtime::tests::a_resident_fold_stores_its_first_contributor_so_a_negative_zero_master_keeps_its_sign`:
  identity constants and all-`-0.0` resident words at 13 frames, so both tiles and the tail are
  covered. The three cases are store onto `+0.0` (gives `-0.0`), a continuation onto `-0.0` (gives
  `-0.0`), and a continuation onto `+0.0` (gives `+0.0`).

### Gates

| gate | command | result |
|---|---|---|
| 1 | graph `a_resident_fold_is_the_staged_scatter_and_cohort_fold_bit_for_bit` + rack `a_fully_folded_full_bank_offers_its_resident_block_before_writing_staging` | pass (debug); pass with `--release` |
| 2 | `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit`, `the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign`, `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit`, console-workload `the_folded_master_is_the_reductions_own_bits` and `every_standing_workload_folds_one_route_per_track` | all pass, unchanged; the graph three and the console two also pass with `--release` |
| 3 | mutation sweep | 915-1 to 915-7 in `crates/graph/tests/MUTATIONS.md`, 915-R1 to 915-R4 in `crates/rack/tests/MUTATIONS.md` |
| 4 | `cargo test -p graph -p rack` | graph lib 95, `rt1_direct_bank_alloc` 1, `rt9_resident_bank_input_alloc` 1; rack lib 40, `console_bank` 10, `mono_reengage` 4; 0 failed |
| 4 | `cargo test -p graph -p rack --features graph/test-support` | graph lib 95, rt1 1, rt9 8; rack 40 / 10 / 4; 0 failed |
| 4 | `cargo test -p graph-compiler` | 73 + 1 + 3 + 1 + 8 + 6 passed, 0 failed |
| 4 | `cargo test -p console-workload` | `automation` 4, `chain_shape` 22, `placement` 3; 0 failed |
| 4 | `bash scripts/check-graph-determinism.sh` | `graph fresh-process determinism: PASS (100/100)` |
| 4 | `bash scripts/check-graph-policy.sh` / `check-rack-policy.sh` / `check-realtime-policy.sh` / `check-lane-policy.sh` | `PASS` / `PASS` / `ok (50 marked regions in 14 files)` / `ok` |
| std | `cargo fmt --all --check` | clean |
| std | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| extra | `RUSTFLAGS="-C target-feature=+simd128" cargo check --target wasm32-unknown-unknown -p graph -p rack` | compiles; nothing was run under wasm |

**Which gate-2 tests reach the fused path.**

- `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit` reaches it. Rows
  915-1/2/3/4/5 turn it red; 915-3 turns it red only on the two-cohort `Four/8` shape.
- console-workload `the_folded_master_is_the_reductions_own_bits` reaches it. Rows 915-1/2/3/5
  turn it red. It has no ragged tail, because every console quantum is 128.
- `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit` does not reach it. It calls
  `fold_cohort` directly, as the brief says.
- `the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign` does not reach it. It
  calls `fold_plane`. The fused counterpart is the new `-0.0` test.

**Realtime.** The folded arm of `crates/graph/tests/rt1_direct_bank_alloc.rs`
`direct_bank_graph_render_is_allocation_free_and_bit_exact` now takes the fused path. That is a full
W4 bank at 11 frames, so it has a tail; row 915-5 turns it red. It still measures 0 allocations and
0 deallocations over 16 blocks.

### Deviations

1. Gate 1's "staging block untouched" clause is asserted in rack, not in the graph test.
   `staging_*` is private to `rack`, and graph cannot observe it without a test-support export
   outside this slice. The graph side also receives only shared borrows of the resident planes, so
   `fold_resident` cannot reach staging by construction. The graph test's doc comment points to the
   rack test.
2. "Seed a non-first lane from zero" is recorded as the continuation seed (915-3, red on every
   gate). The variant that seeds the first contributor from zero is recorded separately (915-3b).
   That one is red only on the `-0.0` test, and the row says so.
3. Two doc sentences in `crates/rack/src/lib.rs` were amended because this change made them false.
   The file is authorized, but both sit outside the three named items: `BankChain::scatter`'s
   ("the tiled path already lands there") and `BankChain::arm_fold`'s ("it lands in this chain's
   staging block"). No code outside the named items changed.
4. `ResidentFoldCohort` carries `BankWidth` rather than a bare `usize` `W`, so it cannot describe a
   width the tiled path does not have. `lanes()` is `W`, and `lane_ids()` returns the `0..W` order
   the brief asks it to carry.

### Anchor drift and notes

- **Anchors.** Every anchor in the brief matched base `12b621f2`. The one imprecision is
  `effect-contract/src/lib.rs:301`, which is `transpose_tile_8`; `transpose_tile_4` is at
  `:289`.
- **A test that no longer mirrors production.**
  `runtime::tests::all_active_folded_bank_chain_dispatches_the_real_graph_cohort` wraps
  `ArenaMembers` in a probe that does not override `fold_resident`. It therefore still pins
  `fold_cohort` through the staged path, and it passes unchanged. Production all-folded full banks
  now take the fused path instead.
- **Codegen.** The hoisted W8 splats (32 vectors) may spill on AVX2. That is a codegen question for
  the batch benchmark, not a bit question.
