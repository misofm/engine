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

## Attempt 1 evidence

Implementer: Terra (attempt 1). Branch `codex/920-fold-routes-into-output-reduction`, from
`135df65a` (#916's verified tip). The commits:

- `cc89a300`: the implementation, the kernel test, the gate 1/2 test, the console gate and the
  floor-ruling sentence.
- `b3650988`: two more gate-2 shapes, `SharedInput` and `LateReader`, which isolate the in-place
  and sole-reader clauses.
- `42daf985` and `6148da30`: signed-zero data on the delayed and submix shapes, so that the
  hazard each of those declines defends reaches the master's bits.
- `bda90ca1`: doc comments only.
- The evidence commit: this section and `crates/graph/tests/MUTATIONS.md` rows 920-1 to 920-14.

Every gate and mutation below ran on `bda90ca1`.

### Design

**Bind-time eligibility: `output_route_fold(program, spec, parts, run_units, master_op)`.**

`preflight_sequential` resolves the Output op by node (`output_op`, #916). Then:

1. The seam `HOST_MASTER_DECLINED` may drop that op, in `#[cfg(test)]` builds only. The fold
   is attempted only if the op survives, because the fused kernel writes the host planes.
2. The fold is computed, and the new seam may drop it (it exists only with `test-support`).
3. The result travels in `SequentialPlan.output_fold`.

Any failed clause declines the **whole** fold: the table has one entry per Output input, or none.

- **The master:**
  - **The plan binds no bank** (`parts.membership().is_empty()`).
  - **The node is `GraphNodeId::Output`, and `node_kind`'s cascade gives it `NodeKind::Identity`**:
    no source, no bank membership, no bound processor, no effect, no route.
  - **No sidechain, no delayed input, and fan-in two or more.**
  - A split pair never binds the Output node, because only a `PostFader`/`PostMatrix` pair does.
    The `Runtime` constructor's `debug_assert` also checks that the built Output op is
    `Identity`, has no split pair, no sidechain and no staging, and has one input per table entry.
- **Every input position `i`, whose producer is `R` (`input_producers`):**
  - `R` exists.
  - `R` is a plain route (`plain_route_gains`, so its entry is `folded_route`, the constants
    `node_kind` would have built).
  - `R` has exactly one input, undelayed, and no sidechain.
  - `R.in_place`, and `R.output` is its input's buffer, which is also the Output's input `i`.
  - `readers[R] == [master_op]`, with sidechain reads counted.
  - `!observed(program, spec, parts, R, served_by_nothing)`: nothing observes the route node, or
    an elided stage whose tap aliases `R`'s buffer after `R`.
  - `R` is a plain unit of its own, in a run before the Output's.
  - `op_names_buffer` finds no mention of `R`'s buffer in any op of any run strictly between `R`
    and the Output.
  - `R` is not already in the set: no route feeds two positions.

**Every decline.** Each was measured with a probe on the gate test, which was then removed:

| decline | shape that reaches it | clause that fires, and only it |
|---|---|---|
| banked plan | every standing workload but the plumbing row; every graph fold fixture | no-bank |
| Output not an identity (bound processor) | 23 bankless graph-compiler binds | master cascade (`has_binding`) |
| observed route | `ObservedRouteAlias`: an elided `PostSimd2PreFader` after track 0's route, metered | `observed` |
| delayed Output input | `DelayedEdge`: track 1's route-to-Output edge delayed three samples | master delayed input |
| non-route contributor | `SubmixContributor`: a submix fed by an extra `Input`; `RouteIntoSubmix`: track 63's route into a submix into the Output | plain route |
| route not in place | `SharedInput`: track 0's `Input` also feeds a second route to the Output | `in_place` and output-is-input |
| second reader of a route | `LateReader`: track 0's route also feeds a metered `PostFader` scheduled after the Output | `readers` |
| fan-in below two | #916's single-route shapes | master fan-in |

**Retire path.** The folded routes reuse `route_fold`'s `retired` mechanism.

- `validate_fold_installation` treats an op in either retired set the same way. A retired op
  must be a plain run of its own, and that run gets no unit and no `op_slot`. The observation
  activation catalog and `output_unit` both read that mapping.
- `build_sequential` builds the union of both folds' retired sets. A run whose ops are all
  retired emits no unit, and the two scalar-pairing passes and `response_owner_bindings` skip
  retired ops.
- The Output op keeps its inputs. An in-place route's buffer is its input's buffer, so the
  Output op already names it. Only what the buffer holds changes: the route's input, not its
  output.
- `validate_fold_installation` refuses the bind with `graph.route_fold.master` if an Output
  fold arrives with a chain fold, or with no Output op. The no-bank clause makes both
  impossible. Row 920-12 shows the mutual-exclusion check is what would stop a banked plan
  binding with two folds over the same routes.

**The kernel: `route_reduce::<FrameLane>`**, with `route_group`, `route_accumulate`,
`route_run`, `mix_chunk` and `add_mixed_chunks`. They sit inside a `REALTIME_POLICY` region with
no allocation, no `unsafe` and no `wide`. For each group of `REDUCE_GROUP` inputs:

- The kernel forms the `N` left and `N` right input slices with `lease.read` (shared borrows).
- It takes both output planes in one pass, `chunks_exact(L::WIDTH)`, with the tail at `L = f32`.
- In group 0, `value = mix(in0)`. In every later group, `value = L::load(host plane)`.
- It then applies `value = value.add(mix(in_i))` in edge order, and stores.
- `mix` is `(lr.fma(r, ll.mul(l)), rr.fma(r, rl.mul(l)))`, from `L::load` of both planes of the
  input.

`execute_op` takes the kernel's branch when `host` is `Some` and `routes` is not empty. Its
`None` arm and its empty-table arm are unchanged. A refused shape returns `false`, and
`execute_op` turns that into `Err(RenderError::InvalidEnvelope)`, so the executor's #916 failure
path silences the host planes rather than play a partial sum. An admitted fold cannot reach it.

**Table storage.** `Runtime.output_routes: Box<[[f32; 4]]>` is empty unless the fold was
admitted.

- The `Runtime` constructor takes it as its last parameter.
- `Runtime::execute` hands it to `execute_op`'s new last parameter (`routes: &[[f32; 4]]`) for
  the Output unit only, and `&[]` everywhere else.
- `Runtime::execute`'s signature is unchanged, so the rt9 text pins still split on the same
  spellings.
- The field is mirrored in `RuntimeWithoutSplitPairTable` and
  `RuntimeWithoutObservationActivation`, so both layout deltas are unchanged
  (`runtime_metadata_charge_covers_mixed_ops_once_and_refuses_overflow` and
  `no_reported_runtime_byte_moved` pass).

**Seam.** `test_only_set_output_route_fold_declined(bool)` lives in `runtime.rs` under
`any(test, feature = "test-support")`. It is read once per bind in `preflight_sequential`, and
it is exported `#[doc(hidden)]` from `lib.rs` beside `test_only_set_route_fold_declined`.

**Accessor.** `GraphExecutor::output_route_folds()` returns `Runtime::output_route_folds()`,
which is `output_routes.len()`. Both are `#[cfg(test)]`: `GraphExecutor` is private, and nothing
outside the crate's tests can call it.

### The per-element order argument

Fix a frame `f` and the left plane. Before this issue:

1. Each route op `R_i` ran `mix2x2_block::<FrameLane>` in place. It stored
   `m_i = lr_i.fma(r_i[f], ll_i.mul(l_i[f]))`, computed from the frame's original `l_i[f]` and
   `r_i[f]`.
2. The Output op's `reduce_many_into` loaded the `m_i` back in edge order. Group 0 stored
   `((m_0 + m_1) + ...) + m_7`. Each later group loaded the running sum from the host plane,
   added its `m_i` left to right, and stored.

The kernel computes each `m_i` with the same operation, the same operands and the same operand
order, from the same unmodified `l_i[f]` and `r_i[f]`, because the retired route never wrote
the buffer. It feeds `m_i` to the same `add` in the same position of the same chain, with the
same group boundaries, and stores and reloads at the same points.

- A store and a load move an `f32`'s bits unchanged. So the only change, taking `m_i` from a
  register instead of from memory, moves no bit.
- `Lane::fma` is two roundings on every backend. So the value does not depend on whether `f`
  falls in the vector body or the `f32` tail, at any `L::WIDTH`, just as in `mix2x2_block`.
- The right plane is the same argument with `rr` and `rl`.
- The two planes share only their loads, so computing them in one pass changes neither plane's
  sequence.

### Which standing workloads take the Output fold

A temporary probe was run over `every_standing_workload_folds_one_route_per_track` and then
removed.

- `sixty_four_track_plumbing_only` folds **64** routes: it binds no bank.
- The other fifteen rows fold **0**. They all bind banks, so they decline on the no-bank clause:
  `nine_track_baseline`, `nine_track_ragged_strip`, `sixty_four_track_console`,
  `one_twenty_eight_track_stretch`, `_eq_only`, `_compressor_only`, `_builtins_only`,
  `_dispatch_only`, `_idle`, `_console_legacy`, `_eq_comp_simd1`, `_gain_pan_only`,
  `_console_mono`, `_console_mono_dual` and `_console_half_mono`.
- Their chain folds, bank shapes and digests are unchanged, and
  `every_standing_workload_folds_one_route_per_track` passes unchanged.
- Outside the new tests and the plumbing row, the same probe found two existing tests that bind
  a plan taking the fold, both in host-core and both passing unchanged:
  - `builtin_batch_endpoint::tests::endpoint_selects_existing_pair_factories_without_observer_barriers`
    (fan-in 3).
  - `builtin_batch_endpoint::tests::forced_scalar_and_native_bank_match_with_state_and_post_fader_witnesses`
    (fan-in 9). Its forced-scalar arm is bankless and now takes the Output fold. Its native-bank
    arm takes the chain fold. The test's equality of the two arms still holds.
- No graph, graph-compiler, builtins-compiler or capi test binds a plan that takes it.

### Tests

- graph `runtime::tests::a_route_reduction_is_the_route_ops_and_the_reduction_bit_for_bit` (the
  kernel test):
  - **Candidate:** `route_reduce::<L>` at `L` = `f32`, `Simd4` and `Simd8`.
  - **Oracle:** the production route op (a copy, then `mix2x2_block::<FrameLane>` in place),
    then `reduce_plane_into`.
  - **Cases:** hostile words and a hostile 2x2 per input; frames `{1, 3, 7, 8, 13, 16, 33, 64}`;
    fan-in 2 to 19 and 64; a signed-zero case whose master must stay `-0.0`.
  - **Refusals:** a mismatched table and fan-in one are refused, with no write.
- graph `runtime::tests::an_output_route_fold_is_the_route_ops_and_the_reduction_bit_for_bit`
  (gates 1 and 2):
  - **Shapes:** every `RoutedShape`, at frames `{1, 3, 7, 13, 16, 64, 128}`. Fan-in is 64, and
    also 2 and 9 for the admitted shapes.
  - **Arms:** each case binds its plan as bound and seam-declined, and renders 8 blocks each
    through `GraphExecutor::render` at stride `frames + 3`.
  - **Per block:** the host storage is bit-identical between the arms, padding included, and no
    padding word moves.
  - **Per case:**
    - `output_route_folds()` is the fan-in for an admitted shape, 0 for a declining one, and 0
      on the oracle.
    - The folded arm has exactly one unit per folded route fewer.
    - Every observer saw every block, and every window equals the oracle's.
    - The master carries audio, or on `NegativeZero` stays `-0.0`.
  - **Admitted shapes:** `Plain`, `NegativeZero`, `Metered(false)` and `Metered(true)`. The
    `Metered` shapes put a meter on every route's producer and two on the Output; `true` binds
    them through the activation catalog.
  - **Declining shapes:** as in the table above.
- console-workload `chain_shape::the_plumbing_rows_output_fold_is_the_route_ops_own_bits` (gate 3):
  - The plumbing row as bound is compared with the row seam-declined.
  - The unit census (`unit_eligibility().len()`) differs by exactly 64.
  - Both arms have `bank_shape() == [0, 0]` and `bank_route_folds() == 0`.
  - Their 64-block digests are equal.

### Gates

| gate | command | result |
|---|---|---|
| 1, 2 | the gate test | pass, debug and `--release` |
| kernel | the kernel test | pass, debug and `--release` |
| 3 | `cargo test -p console-workload` | `automation` 4, `chain_shape` 23 (22 + the new one), `placement` 3; 0 failed. `the_plumbing_row_binds_no_strip_at_all`, `every_standing_workload_folds_one_route_per_track` and `the_folded_master_is_the_reductions_own_bits` pass unchanged. |
| 4 | mutation sweep | 920-1 to 920-5 RED in the kernel test and gate 1; see below |
| 5 | `cargo test -p graph` | lib 101, rt1 1, rt9 1; 0 failed |
| 5 | `cargo test -p graph --features test-support` | lib 101, rt1 1, rt9 8; 0 failed |
| 5 | `cargo test -p graph-compiler` | 73 + 1 + 3 + 1 + 8 + 6; 0 failed |
| 5 | `cargo test -p builtins-compiler --features test-support` | lib 58, `allocation_tracker` 9, `builtin_automation_targets` 3, `input_drain` 6, `scale` 1, `track_delay_domain` 2; 0 failed |
| 5 | `bash scripts/check-graph-determinism.sh` | PASS (100/100). The evidence JSON's sha256 is `e5d45be6...a8face`, #916's value. |
| 5 | `check-graph-policy.sh` / `check-realtime-policy.sh` / `check-lane-policy.sh` | PASS / ok (50 marked regions in 14 files) / ok |
| std | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean; exit 0 |
| extra | `cargo test -p host-core --all-features`; `cargo test -p capi` | lib 86 plus every integration suite, 2 pre-existing ignores; lib 32, `resource_lifecycle` 4; 0 failed |
| extra | `RUSTFLAGS="-C target-feature=+simd128" cargo check --target wasm32-unknown-unknown -p graph` | compiles; nothing was run under wasm |

### Mutations

The full table is in `crates/graph/tests/MUTATIONS.md`, "Issue #920". These are the brief's five:

| # | mutation | kernel test | gate 1 | console gate |
|---|---|---|---|---|
| 920-1 | reverse the accumulation order | RED (fan-in 6, one ulp) | RED | RED |
| 920-2 | zero-seed instead of taking `mix(in0)` as the value | RED (`-0.0` case) | RED (`NegativeZero`) | GREEN: the console data never sums to an all-`-0.0` frame |
| 920-3 | apply the 2x2 to the running sum instead of the input | RED | RED | GREEN: every console route is the identity 2x2 at 0 dB |
| 920-4 | swap coefficient roles (`ll.fma(r, lr.mul(l))`) | RED | RED | RED |
| 920-5 | group 0's `initial_store` in every group | RED (fan-in 9) | RED | RED |

Beyond the brief, each clause of `output_route_fold` and each half of the retire path has its
own row, and each is RED on the host planes or an observer window with the fold-count
assertions removed:

- the observed clause (920-6)
- the in-place clause (920-7)
- the sole-reader clause (920-8)
- the delayed-input clause (920-9)
- the plain-route clause (920-10)
- routes not retired (920-13)
- table not installed (920-14)

The no-bank clause (920-12) is RED at bind on four banked tests. The in-between scan (920-11)
is GREEN, because no test can make it fire; its doc comment says why.

### Deviations

1. **The route table lives on `Runtime`, not on the Output's `RuntimeOp`.** The fold admits
   only the Output op, and the runtime already names that op's unit (`output_unit`). A field on
   `RuntimeOp` would add 16 bytes to every op of every plan, and `size_of::<RuntimeOp>()` is a
   reported byte (`runtime_op_containing_bytes`). As a single `Runtime` field it is mirrored in
   both layout witnesses, so every derived delta is unchanged.
2. **Both planes in one pass per group, where the brief says "per plane".** Each mix reads both
   input planes, so a per-plane pass would load every input word twice. The per-element
   sequence is the same, as the order argument shows.
3. **The master clause "not a bank member" is widened to "the plan binds no bank".** The brief
   makes banked plans a non-goal and lets only the plumbing row move. Two things would break
   without the wider clause:
   - Where the chain fold declines (the half-mono row, and every seam-declined oracle), the
     Output fold would fire and move rows the brief says cannot move.
   - Where the chain fold is admitted, both folds claim the same routes (920-12).
4. **Gate 2's "a `PostMatrix` observer whose tap aliases a route" cannot be built as written.**
   Only `PostSimd1`, `PostDynamic` and `PostSimd2PreFader` are alias candidates, and bind admits
   no observer on a route node. The declining shape is the constructible equivalent: an elided
   `PostSimd2PreFader` after the route, metered. Its tap aliases the route's buffer after the
   route. An observer on a route's *producer* (a post-matrix meter in a session) fires before
   the route and keeps the fold; `Metered` is that control.
5. **Two gate-2 shapes beyond the brief's four** (`SharedInput`, `LateReader`), and signed-zero
   data on three declining shapes. That data is what makes the delayed-edge and non-route
   clauses behaviourally red:
   - **Delayed edge.** A fused delayed route would mix the compensation line's `+0.0` warm-up
     words to `-0.0`.
   - **Non-route contributor.** An identity 2x2 turns a pass-through's `-0.0` into `+0.0`.
6. **`execute_op` gains a last parameter**, so it now carries
   `#[expect(clippy::too_many_arguments)]`. `Runtime::execute` is unchanged.
7. **The in-between scan also scans the other retired routes**, which `route_fold`'s scan
   excludes. Each names only its own, simultaneously live buffer, so this only adds conservatism.
8. **The console gate counts units through `unit_eligibility().len()`.** The engine's plan trait
   carries no `output_route_folds`, and `crates/engine` is outside this issue's paths.
9. **The kernel's refusal becomes a render error** where `reduce_many` returns silently. It is
   unreachable, and the error path silences the host planes.

### Anchor drift

All anchors were taken on `bda90ca1`:

| cited | now |
|---|---|
| `execute_op` `:2332` | `:3021` |
| `reduce_plane` `:298` | `:371` |
| `reduce_group` `:355` | `:428` |
| `route_fold` `:5237` | `:6202` |
| `op_dataflow` `:5460` | `:6569` |
| `plain_route_gains` `:5008` | `:5973` |
| `observed` `:5033` | `:5998` |
| `input_producers` | `:6066` |
| `op_names_buffer` | `:5768` |
| `build_sequential` | `:4996` |
| `preflight_sequential` | `:4718` |
| text pins at `6088-6095` and `6180-6192` | the rt9 pin at `:7283` and `resident_meter_entry_has_one_final_output_dispatch_and_admission_control`, both untouched |
| `the_plumbing_row_binds_no_strip_at_all` at `chain_shape.rs:838` | `:834` |
| the new console test | `:986` |
| `crates/lane/src/kernels.rs:734` | unchanged |
| `docs/rulings/effect-floor-accounting.md:527-529` | edited in place; it now reads "Until issue #920 ... since #920 the route ops are retired into the Output op's reduction" |

### Risks and notes

- **The route table is a bind-time heap allocation** of 16 bytes per Output input (1 KiB on the
  plumbing row). No resource-estimate term charges it, as none charges the chain fold's
  `FoldLane` tables or the bank scratch index boxes. No estimate or allocation gate moved.
- **The containing `GraphExecutor` grows by 16 bytes.** The estimate's
  `runtime_owner_allocation_bytes` is derived from `size_of`, and no test pins its value.
- **The Output unit takes one more branch** (`routes.is_empty()`) per block.
- **No performance claim is made.**
- **Not run:** the full CI "Workspace debug tests" step, the AudioWorklet artifact and browser
  qualification (both batch-boundary work), and `-p bench`. The bench graph tests are
  compile-level and bind no render.
