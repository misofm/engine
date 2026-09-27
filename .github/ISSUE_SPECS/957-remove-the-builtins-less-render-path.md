# Remove the builtins-less render path

Scoping study: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` (owner-directed removal, 2026-09-27; option (b) per the owner's second ruling below).

## Amendments (adversarial verification, 2026-09-27; override conflicting text)

Safety confirmed empirically: among with-builtins plans only host-core's two forced-`Scalar` tests
reach the fold (fan-in 9 and 3); no `Simd4` or `Simd8` plan reaches the fold, #927's read or #925's
alias; with the fold disabled `host-core --all-features` stays green. Evidence: `docs/handoffs/builtins-less-removal-2026-09-27/VERIFY.md`.
1. **ObservedAlias port.** #918's `SourceShape` (`runtime.rs:13863-13886`) has no unbanked alias;
   a `PostSimd1` after `PostInputBuiltins` aliases the bank's buffer and the shape degenerates to
   `Plain`. Add a `SourceShape` field that puts a `PostSimd1` alias directly between the claimed
   Input and its first consumer (as the `routed` track does), and port ObservedAlias onto it.
2. **Record the ported gates' digests on `64b155d0`** (before #936, with #918's fixture), not on
   this issue's base, which already has #936 and would make "moves no bit" circular.
3. **Port or explicitly retire #927's `DeadClaim`**, the only test of clause (b)'s zero-reader arm
   with a recoloured slot, which stays live.
4. **Gate 9:** `graph_fixture --check` already fails on the batch head (#947 and #963); use
   `graph_fixture --manifest` equality before and after instead.
5. **Gate 7:** padding the executor by 32 bytes moved no native pin or fixture byte, so expect no
   native re-pin; run `scripts/check-browser-expected-resources.py` for the wasm side and re-pin
   only what moves, with the reason.

## Rulings (coordinator, 2026-09-27)

The owner directed the complete removal of the builtins-less path ("I don't think we should be
benchmarking something that never gets used in the real world"). Option (b) is taken, by the owner's
second ruling ("We should remove everything related to a builtins-less compile because a
builtins-less compile is never needed in production"): the builtins-less compile is removed
entirely, including any test-only entry point, builtins become mandatory in the graph compiler,
and every test that compiled without builtins is ported to `compile_with_builtins`. The optimisation batch lands unchanged and this work deletes #937's
code afterwards. The wasm console arm is re-indexed. `sixty_four_track_gain_pan_only` is the
pure-audio-path target; its fused twin follows. #938 is re-based onto the gain/pan session. The
phase-profile harness is retargeted to `gain_pan_only`. Ported tools compile at
`Backend::current()`.

## Product outcome

No host compiles a plan without builtins: the browser boot, the C ABI and the native host all reach
`GraphCompiler::compile_with_builtins` (`crates/host-core/src/prepare.rs:1195`) at the build's
vector width. Yet `crates/graph` carries about 900 lines of render and bind code that only a plan
with **no bank at all** can reach, and on a vector backend every with-builtins track is a bank
member:

- the fused Output route reduction (#926), its kernels (#926/#937) and its in-place source reads
  (#927, `played_planes_group`);
- the alias lowering of unlisted builtin stages (#925).

The shipped AudioWorklet module carries two of those kernels, and they can never run there. Two
benchmark rows measure this path, one of them in the wasm arm. Delete the code, retire the rows,
and point the benchmark at the real no-effects session.

## Root evidence

- Why the fold is bankless-only: `output_route_fold` declines any plan with a bank member
  (`crates/graph/src/runtime.rs:7367`; the membership includes builtin banks, `:4439-4461`).
- Why every with-builtins track banks: on a vector backend every builtin stage is a bank member,
  and short banks are padded (`crates/builtins-compiler/src/lib.rs:1290-1294, 1342`;
  `crates/rack-compiler/src/lib.rs:243-245`).
- Why the in-place Output read needs a builtins-less plan: it needs an Input whose only reader is a
  retired route (`runtime.rs:6280-6284`). With builtins, the reader is `PostInputBuiltins`.
- Why the #925 arm needs a builtins-less plan: it needs a builtin stage outside
  `required_bindings` (`crates/graph/src/program.rs:662`). `compile_with_builtins` lists all
  three stages (`crates/graph-compiler/src/compile.rs:816-832`).
- Shipped builds are `Simd8` natively and `Simd4` in the browser, which ships `simd128` only
  (`crates/lane/src/backend.rs:31-70`; `scripts/build-web-audioworklet.sh:26-30`).
  `Backend::Scalar` is reached only by tests and an unshipped CI compile.

## Smallest closable slice (this issue): delete the Output route fold family

Authorized paths:
- `crates/graph/src/runtime.rs` and `crates/graph/src/lib.rs`;
- `crates/graph/tests/rt10_source_in_place_alloc.rs` and `crates/graph/tests/MUTATIONS.md`;
- this spec.

1. **The fold.** Delete `output_route_fold` and `OutputRouteFold` (`runtime.rs:7287-7439`).
2. **The kernels.** Delete `OutputSources`, `OUTPUT_GROUP`, `route_reduce`, `route_group`,
   `route_pair_vectors`, `route_lone_vectors`, `route_tail`, `route_run`, `mix_chunk` and
   `add_mixed_chunks` (`:622-1087`).
3. **The runtime state.** Delete:
   - the `output_routes` and `output_sources` fields and their two layout mirrors;
   - the constructor parameter and its assert;
   - the `execute` routing;
   - `execute_op`'s `routes` and `sources` parameters and its third host arm, leaving two arms
     (`:3676-3684`);
   - the Output-fold arms of `validate_fold_installation` and `build_sequential`;
   - the seam and the `output_route_folds` accessor.
4. **`source_plane_table`.** Delete clauses (b′) and (e), the `output_producers` parameter and
   `SourcePlanes.output`. Rewrite the doc: a claim is bound in place only by a bank gather, or when nothing reads it
   (clause (b), `runtime.rs:6159`).
5. **`lib.rs`.** Delete `GraphSourcePlanes::played_planes_group` and its realtime region
   (`:1863-1899`), the seam export (`:36`) and the accessor (`:2572`).
6. **Port #936's four graph gates** (`runtime.rs:16433`, `:16452`, `:16484`, `:16515`) onto #918's
   banked source-fed fixture, keeping all four shapes:
   - `Plain`: an inert claim.
   - `ObservedInput`.
   - An observed elided alias, now at a rack boundary (`PostSimd1`) instead of `PostFader`.
   - `TrackDelayed`.
   Record the pre-change digests on the base commit, as `INERT_PRE_CHANGE` does.
7. **Delete the other dedicated tests** (listed in section 3.1 of the study) and the bankless arm
   of `rt10`. Mark the #926, #927 and #937 rows in `MUTATIONS.md` retired.

## Non-goals

- No change to the bank-chain route fold (#218/#915/#945), to #918's in-place bank gathers, to
  #936's skip, to #916's host planes, or to any with-builtins lowering.
- #925's alias arm is #958. The compile entry is #959.

## Objective gates

1. **Digests.** Every console workload's 64-block digest and unit census is unchanged. After the
   companion issue, every row has builtins.
2. **Host-core.** `cargo test -p host-core --all-features` is green, including the two forced-scalar
   tests. They took the fold. Now they run the route ops and the reduction, which #926 proved
   bit-identical.
3. **#936.** Its four ported gates are green on the banked fixture, with digests equal to base.
4. **#918.** `rt10`'s #918 arm is green, with zero allocations over 1,000 blocks.
5. **Nothing left behind.**
   `rg -n 'output_route|route_reduce|route_group|route_tail|OutputSources|played_planes_group|output_sources' crates/`
   finds nothing outside history text.
6. **AudioWorklet artifact.** The build script's cargo line and `scripts/check-web-audioworklet.sh`
   exit 0. The kernel census is the previous one minus one (at least 11), and the render closure is
   unchanged. The pin is repinned at the batch boundary.
7. **Resources.** The `capi` `resource_lifecycle` test and the host-core resource tests pass.
   `size_of::<GraphExecutor>()` shrinks by 32 bytes. Totals charge only layout deltas, which do
   not move, but `runtime_owner_allocation_bytes` does, and so may a `largest_allocation_bytes`
   it dominates (`crates/graph/src/lib.rs:484-536`). Re-pin only what moves, and state why.
8. **Red mutations** in `MUTATIONS.md`. Each must turn a ported #936 gate red:
   - dispatch every `SourceInput` unit;
   - skip an observed Input unit;
   - skip a `TrackDelay` unit.
9. fmt; clippy with `-D warnings`; `cargo test -p graph` with and without `test-support`;
   `-p graph-compiler`, `-p console-workload`, `-p host-core --all-features`, `-p capi`,
   `-p source --all-features`; `scripts/check-graph-determinism.sh` and #947's fixture test (the slice must not move
   `fixtures/graph/v1/*`),
   `check-graph-policy.sh`, `check-realtime-policy.sh`, `check-lane-policy.sh`.

## Console benchmark rows

No row may move a bit. After the companion issue no row reaches the deleted code, so no row may
move a unit either.

## Dependencies

- #956 lands first.
- Land after the plumbing-floor-2 batch (#936 and #937 merged), so the four #936 gates exist to be
  ported.

## Standing rules for the implementer

- Work only from this body. Class A: every "unchanged" gate is a hard stop.
- The render path stays allocation-, lock- and syscall-free, and `crates/graph` stays free of
  `unsafe`.
- Commit on `codex/<issue>-<slug>` from synchronized `main`. Do not run timed benchmarks.

## Attempt 1 evidence

Implementer: Claude Opus 5.5 (attempt 1), 2026-09-27, branch `codex/957-remove-builtins-less-render-path`
on `d86dd62d` (#956's attempt-1 tip). Code commit `bf3bacab`; MUTATIONS commit `10b53a56`. Host AMD
EPYC 7313P (Zen 3), 32 threads, rustc 1.97.1, `CARGO_INCREMENTAL=0`, the worktree's own `target/`
(deleted afterwards), `CARGO_PROFILE_DEV_DEBUG=0` to save disk (debug info only; no codegen change).
The timed runner was not run. Scratch files named below live in this session's scratchpad,
`scratchpad/957/`.

### What was deleted

| file | production lines (removed / added) | test lines (removed / added) |
|---|---:|---:|
| `crates/graph/src/runtime.rs` | 875 / 58 | 2,122 / 337 |
| `crates/graph/src/lib.rs` | 58 / 10 | – |
| `crates/graph/tests/rt10_source_in_place_alloc.rs` | – | 114 / 27 |

- `runtime.rs`, the kernels (466 lines, one block): `OutputSources` and `resolve_group`,
  `OUTPUT_GROUP`, `route_reduce`, `route_group`, `route_pair_vectors`, `route_lone_vectors`,
  `route_tail`, `route_run`, `mix_chunk`, `add_mixed_chunks`.
- `runtime.rs`, the fold (154 lines): `OutputRouteFold` and `output_route_fold`.
- `runtime.rs`, the plumbing: `Runtime.output_routes` and `Runtime.output_sources` and their two
  layout mirrors (`RuntimeWithoutSplitPairTable`, `RuntimeWithoutObservationActivation`); the
  constructor's `output_routes` parameter and its route-table `debug_assert`; `execute`'s route and
  source selection; `execute_op`'s `routes` and `sources` parameters and its third host arm (two
  arms remain); `SequentialPlan.output_fold`, its computation in `preflight_sequential` and its arm
  in `validate_fold_installation`; `build_sequential`'s `output_producers` and retired-route
  extension; the `OUTPUT_ROUTE_FOLD_DECLINED` seam and `test_only_set_output_route_fold_declined`;
  `Runtime::output_route_folds`.
- `source_plane_table`: clauses (b') and (e), the `output_producers` parameter, and `SourcePlanes`
  (with its `output` field gone it wrapped one table, so the function now returns that table). The
  doc is rewritten: a claim is bound in place only for a bank gather or when nothing reads it
  (clause (b)). The exposure (e)'s doc recorded for (b) -- a hand-built plan whose input slot a
  value dying before the input op overwrites -- is kept in the doc, since (e) no longer names it.
- `lib.rs`: `GraphSourcePlanes::played_planes_group` and its `REALTIME_POLICY` region (36 lines),
  the seam's re-export, and `GraphExecutor::output_route_folds`. Stale #927 comments at the copy
  loop and the lent-claim list are corrected.
- Two `#[expect(clippy::too_many_arguments)]` went with their parameters: `execute_op` and
  `source_plane_table` now take seven, below the lint's threshold, and an unfulfilled expectation
  would fail `-D warnings`.

### The ported #936 gates and their base digests

#918's `SourceShape` gains two fields (test-only):

- `aliased: Option<usize>` (amendment 1): track `K`'s `Input` feeds a `PostSimd1` rack boundary,
  which feeds `K`'s first consumer (its `PostInputBuiltins` bank stage here; its route when `K` is
  also `routed`). One undelayed input makes the boundary an elided alias of the input's buffer, so
  its meter binds to the input op. `K` stays banked, so the alias meter is the only reason `K`'s
  claim keeps the copy and `K`'s input unit stays dispatched, which is the role `PostFader` played in
  #936's ring shape. The meter uses the observed input's handle, so `ObservedInput` and
  `ObservedAlias` lower alike and digest alike, as in #936.
- `dead: bool` (amendment 3): one more claim, `track99`'s `Input`, with no reader, scheduled last
  among the inputs.

The four gates run on `W4 x 6` (`SourceShape::banked`: a four-lane and a two-lane cohort, every
bank stage metered), `K = 5` (the partial cohort's last lane), frames `{10, 13, 16, 128}`, sixteen
blocks of `PLAYED_SCRIPT`, host stride `frames + 3`, every claim's slot poisoned before each block:

| gate | test | shape | census / table | digest (`64b155d0` = base = after) |
|---|---|---|---|---|
| 1 | `an_unobserved_source_input_is_not_dispatched_and_moves_no_bit` | `Plain` | 15 units; table = the 9 non-input units | `0x3adf_c3ee_a1b4_75ab` |
| 2 | `an_observed_source_input_stays_dispatched_and_meters_the_base_values` | `ObservedInput` | 15; table = 10 (`K`'s input kept) | `0x51c6_c3cf_4cdc_5509` |
| 2 | (same test) | `ObservedAlias` (`PostSimd1` on `K`) | 15; table = 10 | `0x51c6_c3cf_4cdc_5509` |
| 3 | `a_delayed_claim_stays_dispatched_and_renders_the_base_bits` | `TrackDelayed` | 15; table = 10 (`K`'s `TrackDelay`) | `0xd1eb_c3d4_2404_cb33` |
| 5 | `the_metadata_charge_covers_the_banked_source_plans_executor_tables` | `Plain`, in place and declined | tables `[9 × 4, 0]` and `[9 × 4, 6 × 16]` fit the charge | – |
| #927 `DeadClaim` | `a_claim_nothing_reads_is_bound_in_place_and_its_recoloured_slot_moves_no_bit` | `dead`, redirects bound and declined | 7 claims in place; bits = copy arm | `0xe889_005e_2b7c_170c` (both arms) |

- **Recording (amendment 2).** A scratch recorder (`scratchpad/957/record-on-64b155d0.patch`, the
  same fixture edits by exact-text replacement plus `zz_record_957`, which binds, poisons and
  renders exactly as the ported helpers do but uses no #936 API) ran in a detached worktree at
  `64b155d0`, whose loop dispatched every unit. It printed the digests above. The same recorder on
  this issue's base (`d86dd62d`, before the deletion) printed the same six values, and the ported
  gates pass after the deletion, so the deletion moves no bit.
- **Deviation: frames.** #936 used `{1, 7, 16, 128}`. #918's `PLAYED_SCRIPT` has a nine-frame short
  block, which panics below nine frames, so the port uses `{10, 13, 16, 128}` (the short block is
  short at every quantum; three quanta have a ragged four-lane tail).
- **Gate 5** is renamed from `the_metadata_charge_covers_the_ring_plans_executor_tables`, and it now
  also asserts that the table is nine units whether the claims are copied or not.
- **DeadClaim (amendment 3), ported.** The test checks, on the lowered program, that the dead
  input's slot is recoloured: a later op writes it and a later op reads it (the first bank member's
  dedicated output and its route). Bound, the scatter redirect then leaves that slot for the
  route's buffer; declined, the bank scatters into it and the route reads it from the arena. The
  mode table is all seven claims in place, the counts are `[0, 6 × 7, 6 × 1]` against the copy
  arm's `[7 × 8, 0, 0]`, and the bits equal the copy arm's. Limitation: #927's shape recoloured the
  dead slot to an Output input, which read by position; on the banked fixture the next op after
  the inputs is a bank member, so no bank *gathers* the recoloured slot, and the port's red
  mutation (957-4) is mode-level. A bank gather of a dead slot would need a scalar stage between the
  inputs and the first bank, which the fixture cannot express without a new stage kind.

### Deleted tests, and why

All in `crates/graph`. Graph lib tests go from 114 to 108; rt10 from 2 to 1.

| test (and fixture) | why it goes |
|---|---|
| `a_route_reduction_is_the_route_ops_and_the_reduction_bit_for_bit`, `assert_route_reduce_is_the_route_ops_and_the_reduction` | #926/#937's kernel oracle; `route_reduce` is deleted |
| `a_route_tail_refuses_a_run_as_long_as_the_widest_lane` | `route_tail` is deleted |
| `an_output_route_fold_is_the_route_ops_and_the_reduction_bit_for_bit`, with `HostileInput`, `RoutedShape`, `routed_output_parts`, `bind_routed` | the Output route fold is deleted |
| `a_plain_strip_source_is_read_in_place_by_the_fused_output_with_the_copy_bits` | #927 gates 1 and 3: clause (b') is deleted |
| `a_claim_with_another_reader_keeps_the_copy_and_the_copy_bits` | #927 gate 2: its shapes test (b') and (e). `DeadClaim` is ported; `TrackDelayed`, `ObservedInput` and `ObservedAlias` stay covered by #918's gate 1 (shape 4) and the ported #936 gates |
| the ring fixture: `RingBlock`, `RING_SCRIPT`, `ring_plays`, `RingSource`, `RING_TRACKS`, `RING_SPECIAL`, `RingShape`, `ring_output_parts`, `RING_FRAMES`, `RING_PRE_CHANGE`, `ring_fnv`, `assert_ring_shape_at_every_quantum`, `RING_SLOT_POISON`, `RingRun`, `ring_slots`, `render_ring_shape`, `ring_hazard_is_built`, `assert_ring_shape` | bankless-only; its four #936 gates are ported first |
| #936's ring gates and helpers (`INERT_PRE_CHANGE` over `RingShape`, `InertRun`, `render_ring_blocks`, `assert_inert_shape`) | replaced by the ported gates of the same names (gate 5 renamed) |
| `a_route_reduction_reads_each_lent_input_as_the_copys_words`, `assert_route_reduce_reads_lent_inputs_as_the_copy`, `LentPlanes` | #927's kernel test; the kernel is deleted |
| `a_source_sets_group_call_is_its_per_claim_call` | #937's test of `played_planes_group`, deleted |
| rt10 `an_in_place_output_read_renders_the_copy_bits_and_allocates_nothing` and `prepared_plan`'s `bankless` arm | #927's allocation gate on the deleted read |

### Gates

1. **Digests and unit census: unchanged.** A scratch probe (`scratchpad/957/zz_census_probe.rs`,
   copied into `tools/console-workload/tests/` only while it ran, never committed) renders every
   native session row (`native_session_rows()`: the 15 `WORKLOADS`, the ring row and the metered
   row) for 64 blocks at `Backend::current()` (Simd8) and at `Simd4`, and prints per row the SHA-256
   of the output, units, banked units, symmetry census, bank shape, transposes, folds, redirects,
   the per-block dispatched-unit counts and the source-plane counts. `diff` of the 34 lines on the
   base (`d86dd62d`) and after `bf3bacab`: identical. The base lines also equal #956's recorded
   census. E.g. `sixty_four_track_gain_pan_ring` Simd8: `01e465a7…2dfdb4`, 73 units, 9 dispatched
   per block, planes `[0, 4096, 0]`; Simd4: the same digest, 81 units, 17 dispatched.
   Command: `cargo test --locked -p console-workload --test zz_census_probe -- --nocapture`.
2. **host-core:** `cargo test --locked -p host-core --all-features`: exit 0; lib 86 (including
   `builtin_batch_endpoint::tests::forced_scalar_and_native_bank_match_with_state_and_post_fader_witnesses`
   and `…::endpoint_selects_existing_pair_factories_without_observer_barriers`, the two forced-Scalar
   tests that took the fold and now run the route ops and the reduction) and every integration
   binary green (`builtin_batch_endpoint` 20, `prepare` 15, `source_in_place` 1, `track_delay` 8/2/7,
   and the rest).
3. **#936:** the four ported gates are green with digests equal to `64b155d0`'s (table above).
4. **#918:** `cargo test --locked -p graph --features test-support`: rt10's
   `an_in_place_source_gather_renders_the_copy_bits_and_allocates_nothing` passes (zero
   allocations and frees over 1,000 blocks, lent bits equal the copied plan's).
5. **Nothing left behind.** `rg -n 'output_route|route_reduce|route_group|route_tail|OutputSources|played_planes_group|output_sources' crates/`
   finds only history rows of `crates/graph/tests/MUTATIONS.md` (#926, #927 and #937 sections, now
   marked retired). `tools/`, `hosts/` and `scripts/` have no match either.
6. **AudioWorklet artifact.** Built with the build script's cargo line
   (`RUSTFLAGS="-C target-feature=+simd128 -C strip=debuginfo <remaps>" cargo build --locked --release --target wasm32-unknown-unknown -p host-web`,
   into `target/wasm-simd`), before and after:
   - kernel census (`--kernel-shape --kernel-pattern '4wide6f32x[48]' --kernel-min 11`): 15 → 14;
     the one kernel gone is `graph::runtime::route_group<f32x4>` (vector 38, scalar 0). `f32x4`
     arithmetic 11,751 → 11,713 (−38). The roster lines are identical. `route_tail` is gone too
     (it was scalar and not a counted kernel).
   - render closure (`--callgraph miso_engine_web_v1_render`): `closure=8 traps=5`, the same trap
     owner (`PreparedRenderPlan::render_inner`) and entry (`slice_index_fail`), before and after.
   - module 3,310,208 → 3,292,914 bytes. Digest before `1bc33051…f674`, after `190fb681…42e3`. The
     pin (`8934cdd9…0078`) matches neither: it is already stale on the base, and it is repinned at
     the batch boundary, not here.
   - `bash scripts/check-web-audioworklet.sh <dir>` over the seven-file set (the after module, the
     four web files, and `parameter-metadata --write`): exit 0.
7. **Resources.** `size_of::<GraphExecutor>()` 616 → 584 (−32) and `size_of::<Runtime>()` 488 → 456
   on x86_64 (scratch probe); both layout mirrors shrink with it, so the owner deltas the
   accounting charges are unchanged (split 16, observation 256).
   - `cargo test --locked -p capi`: `resource_lifecycle` 4 of 4 and lib 32 green; host-core's
     resource tests green (gate 2). No native pin moved, as amendment 5 predicted.
   - `scripts/check-browser-expected-resources.py`: exit 1 **on the base and after, with the same
     message**: `graphIncrementalPlanBytes 32294 -> 32402, graphMetadataBytes 4023 -> 4131,
     graphSessionPlusPlanBytes 32294 -> 32402`. A dump of the oracle's rows and the native witness
     (`scratchpad/957/dump_resources.py`, the script's own `build_module`, `print_oracle` and
     `native_report`) is byte-identical before and after, all 24 rows on each side. This issue moves
     no browser row, so nothing is re-pinned; the +108 staleness predates it (see Deviations).
8. **Red mutations** (`crates/graph/tests/MUTATIONS.md`, issue #957; each applied alone to
   `bf3bacab`, `cargo test --locked -p graph --lib --no-fail-fast`, restored with `git checkout`):
   - dispatch every `SourceInput` unit (957-1): RED on gates 1, 2, 3 and 5 at their tables;
     bits-only, the three digests stay (class A) and only gate 5's count is red. The loop form
     (957-1b, 936-4 re-run) is red on gates 1-3 at the per-block dispatch count.
   - skip an observed `Input` unit (957-2): RED on gate 2; bits-only RED on its pinned digest (the
     `Plain` digest appears, because the input meter never publishes). 957-2c isolates the
     `PostSimd1` alias arm: bits-only RED on `ObservedAlias`'s digest.
   - skip a `TrackDelay` unit (957-3): RED on gate 3; bits-only RED on its digest (the undelayed
     `Plain` digest).
   - the dead claim (957-4, clause (b) refuses a zero-reader claim): RED at its mode table;
     bits-only GREEN, disclosed.
9. **Build, lint, tests, scripts** (`scratchpad/957/gates.sh` on `bf3bacab`'s tree, one log per
   step, all exit 0; after `10b53a56`, which changes one assertion message in a test, `cargo fmt
   --all --check`, `cargo clippy -p graph --all-targets --all-features -- -D warnings` and both graph
   test runs were repeated, green):
   - `cargo fmt --all --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `cargo test --locked -p graph` (lib 108, rt1 1, rt10 1, rt9 1) and `--features test-support`
     (lib 108, rt1 1, rt10 1, rt9 8)
   - `-p graph-compiler`, `-p console-workload` (lib 8, automation 4, chain_shape 23, placement 3),
     `-p host-core --all-features`, `-p capi`, `-p source --all-features` (72)
   - `scripts/check-graph-determinism.sh` (PASS 100/100), `check-graph-policy.sh` (PASS),
     `check-realtime-policy.sh` (ok, 55 marked regions in 16 files: `played_planes_group`'s region
     is gone), `check-lane-policy.sh` (ok)
   - `graph_fixture --manifest` (amendment 4): byte-identical before and after (seven files, e.g.
     `v1/direct-route.resources.json` 350 bytes `24bb7a5a…`), so no `fixtures/graph/v1` byte moves.

### Deviations and notes for the verifier

1. **Pre-existing, not re-pinned.**
   - `hosts/host-web/tests/browser-v1/expected.json` is stale on the base by +108 bytes in three
     graph rows (gate 7), and this issue moves none of them. It is outside this issue's authorised
     paths, and nothing here would justify the byte accounting, so it is left for whoever owns the
     move (the batch boundary, like the artifact pin).
   - The AudioWorklet artifact pin already mismatches the base's module (gate 6). Not repinned, per
     the brief.
2. **How gate 6's script ran.** `build-web-audioworklet.sh` refuses on the pin mismatch before it
   copies anything, so the seven-file set was assembled by hand from the build script's own cargo
   line and copies, with `parameter-metadata --write` run as a debug build (its output is data, not
   code), and `check-web-audioworklet.sh <dir>` was run on it.
3. **`SourcePlanes` is deleted**, not trimmed to one field, and `source_plane_table` returns the
   table (slice item 4 names only its `output` field).
4. **Amendment 1's reading.** The alias sits between `K`'s input and `K`'s *bank* stage, so `K`
   stays banked and the alias meter alone keeps its copy and its dispatch. With `routed` the same
   field would put the alias before the route, as VERIFY.md §7 describes; the gate does not use that
   combination.
5. **Amendment 3: ported, with a limit** (the ported-gates section above, and row 957-4).
6. **A hazard kept in prose, not fixed.** Deleting clause (e) removes the only clause that named
   the colouring exposure of an input scheduled after a value that takes its slot. That exposure
   stays for (b) on hand-built plans (no compiled plan builds it: with builtins every input has a
   reader). The rewritten `source_plane_table` doc now records it, since fixing (b) is outside this
   issue (#918's gathers are a non-goal).
7. **MUTATIONS.md add-on** (coordinator, from #956's verification): the #936 and #937 records' console
   test names are mapped to `the_driver_fed_gain_pan_row_dispatches_every_unit_but_its_inputs` and
   `the_driver_fed_gain_pan_row_renders_the_bound_rows_bits` (old names kept in parentheses as
   history), and the two deleted `chain_shape` plumbing tests are marked retired where the #925,
   #926, #936 and #937 records cite them. The #925 record is otherwise untouched (#958 owns it).
8. **Disk.** `CARGO_PROFILE_DEV_DEBUG=0` for every debug build, to fit the host's free space; it
   changes debug info only. The worktree's `target/` and the scratch worktree at `64b155d0` are
   deleted.
