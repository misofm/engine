# Write the master straight into the host planes

## Product outcome

Every block ends with the executor copying the Output node's arena buffer into the host's
`PlanarBufferMut` planes, and a single-input Output node first copies its one producer into that
arena buffer. Make the host's planes the Output node's storage for the block: the fold
epilogues, the master reduction and the fan-in-one copy write the host planes directly, honouring
`plane_stride != frames`, and the end-of-block copy disappears. Class A: the words are the same,
one fewer `2·Q` pass per block for every session.

## Root evidence

- `crates/graph/src/lib.rs:2281-2283`: after the unit loop, `runtime.buffer(*output_buffer)` is
  `copy_from_slice`d into `output.plane_mut(0)` and `plane_mut(1)`. `output` is
  `program.output.0 + ARENA_BASE` (`:2136`).
- `crates/graph/src/runtime.rs:298` `reduce_plane`: fan-in one copies the input into `out` unless
  the op is in place; fan-in two or more reduces through `reduce_many` (`:325`) whose groups write
  `out` via `ArenaLease::write_read_many` (`crates/engine/src/realtime/disjoint.rs:387`).
- `runtime.rs:1345` `fold_plane` and `:1358` `fold_cohort` write `lease.write_stereo(self.master)`;
  `RouteFold.master` (`:5083-5095`) is the master op's output buffer, and the master op is the op
  whose reduction the epilogues replaced (on every console fixture, the Output node's op).
- `crates/graph/src/program.rs:207` `is_dedicated`: only `PostInputBuiltins` stages and
  non-dynamic effects own dedicated storage; the Output node does not, so its logical buffer may
  share a physical slot with earlier-retired buffers (the `route_fold` ledger at `runtime.rs:5205`
  records that the console fixtures give it track zero's input slot) and its op may be in place
  over a single producer (`:691-700`).
- `crates/engine/src/realtime/buffer.rs:139` `PlanarBufferMut` holds one `&mut [f32]` with
  `channels`, `frames`, `stride`; `plane_mut` (`:211`) returns `storage[c*stride .. c*stride +
  frames]`. Hosts build it with `stride == frames` (`hosts/host-web/src/lib.rs:4876`,
  `crates/host-core/src/render_session.rs:145` from the caller's `plane_stride`) or the C ABI's
  caller stride (`crates/capi/src/ffi.rs:805`).
- Observers bound to the Output node read `lease.read_stereo(op.output)` in `observe`
  (`runtime.rs:2602`).
- `crates/graph` carries no `unsafe` (`docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code
  ownership"); the design below keeps it that way and does not touch `disjoint.rs`.

## Smallest closable slice

Authorized paths: `crates/engine/src/realtime/buffer.rs` (one accessor), `crates/graph/src/lib.rs`
(`GraphExecutor::render`), `crates/graph/src/program.rs` (`is_dedicated` only),
`crates/graph/src/runtime.rs`, their tests, `crates/graph/tests/MUTATIONS.md` (new rows), and
this spec.

1. `PlanarBufferMut::stereo_planes_mut(&mut self) -> Result<(&mut [f32], &mut [f32]),
   BufferArenaError>`: both planes of a two-channel buffer at once (`split_at_mut(stride)`, each
   trimmed to `frames`), error otherwise. Add a unit test at `stride > frames`.
2. `is_dedicated` returns `true` for `GraphNodeId::Output`. Consequences to accept and state: the
   Output op is never in place and its logical buffer never shares a physical slot, so "the op
   whose output buffer index equals `program.output`" identifies exactly the Output op, and the
   fold epilogues' `master` equals that index whenever the master op is the Output op.
3. `GraphExecutor::render` takes the two host planes once (step 1), checks both lengths equal
   `lease.frames()` (else `RenderError::InvalidEnvelope` before any unit runs), and passes a
   `HostMaster<'_> { left, right }` reborrow into `runtime.execute`, `observe_unit` and
   `observe_active_unit`. In the runtime:
   - `execute_op`: when `op.output == self.output`, `reduce_plane`'s three arms write the host
     plane: fan-in zero fills it, fan-in one copies the single input into it (`lease.read` + host
     `copy_from_slice`), fan-in two or more forms the group's `N` inputs with
     `lease.read(plane, input)` (`disjoint.rs:198`; it takes `&self`, so `[&[f32]; N]` coexist)
     and runs the same `chunks_exact` group loop as `reduce_group` with the host plane as `out`,
     `initial_store` only for group 0. No `disjoint.rs` change: `write_read_many` cannot take a
     host plane as `out` (`disjoint.rs:397`) and a `read_many` sibling would live outside this
     issue's paths. The op's own processing (`Identity`, `Bound`, ...) then runs on the host
     planes. `Runtime` has no `output` field today (`runtime.rs:1513-1543`); add it in
     `build_sequential` from `program.output.0 + ARENA_BASE`, as `lib.rs:2136` computes it.
   - `ArenaMembers`: `master` becomes an enum `{ Arena(u32), Host(&mut [f32], &mut [f32]) }`;
     `fold_plane`, `fold_cohort` (and `fold_resident` if the fold-epilogue issue landed) write
     through it. The runtime picks `Host` when `master == self.output`.
   - `observe` for the Output op reads the host planes instead of `lease.read_stereo`.
   - The end-of-block copy in `lib.rs:2281-2283` is deleted.
4. On an executor-level `Err` (a unit or observer failing inside the unit loop, or the source
   set failing in `begin_block`/`copy_track_input`), fill both host planes with `0.0` before
   returning (failure path only, never on the success path), so a host that ignores the error
   emits silence rather than a partially written master. Envelope rejections made before any
   unit runs (the length check in step 3, and `plan.rs:927`, which already rejects
   `io.output.frames() != frames` before the executor is entered; keep the executor's check as
   belt and braces) leave the planes untouched. State exactly this scope in the render contract
   comment.

## Non-goals

No change to `disjoint.rs`, to the fold eligibility, to the C ABI or web ABI, to the browser
output copy (Web Audio owns those arrays), or to hosts. No change to how source inputs enter the
arena (separate issues).

## Objective gates

1. New graph test: for random input and three plan shapes -- 64 folded routes into the Output
   (console shape), 64 unfolded routes (fold declined through `test_only_set_route_fold_declined`),
   and one submix into the Output (fan-in one) -- the host planes after `render` are bit-identical
   to the pre-change path's output (an oracle that keeps the arena buffer and copies), at
   `stride == frames` and at `stride == frames + 7`, with the words beyond `frames` in each plane
   untouched. Every Output-node observer window is bit-identical between the two.
2. `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit`,
   `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit`, console-workload
   `the_folded_master_is_the_reductions_own_bits`, `every_standing_workload_folds_one_route_per_track`
   and `the_plumbing_row_binds_no_strip_at_all` pass; `bank_route_folds` and `bank_shape` of every
   standing workload are unchanged (assert in the new test against the values the existing
   chain-shape tests pin).
3. New test: a render that fails at a mid-schedule unit leaves both host planes all `+0.0`; a
   render rejected for an output length mismatch leaves them holding their prior words.
4. `cargo test -p engine -p graph -p graph-compiler -p console-workload -p host-core -p capi`,
   `scripts/check-graph-determinism.sh` (its evidence JSON byte-identical to the pre-change run),
   `scripts/check-graph-policy.sh`, `scripts/check-realtime-policy.sh`,
   `scripts/check-host-core-policy.sh`, `scripts/check-capi-abi.sh`.
5. `MUTATIONS.md` rows: keep the end-of-block copy (red on gate 1's observer/plane checks only if
   the test asserts no arena write to the output slot -- it must), write the master to the arena
   and skip the host (red), fan-in-one copy from the wrong plane (red), zero-fill skipped on the
   failure path (red on gate 3).

## Console benchmark rows

Can move: every row, by one `2·Q` copy per block, including `sixty_four_track_plumbing_only`;
this is a small constant and no saving is projected. Cannot show anything else.

## Dependencies

Merge after "Fuse the fold epilogue into the scatter transpose" (both edit the `ArenaMembers`
master write sites; this issue rebases and routes the fused kernel's master through
`HostMaster`). Independent of the source-ring issues.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate above that says "bit-identical" is a hard stop, not a tolerance.
- The owner's copy rule: a block-sized copy on the render path exists only with a written justification that no in-place or direct-write form exists. This issue removes the master-to-host copy and the fan-in-one route-to-arena copy; the browser's wasm-to-`Float32Array` copy stays and its justification is Web Audio's ownership of the output arrays.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). `crates/graph` stays free of `unsafe`. Only `crates/lane` may name `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named above before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. The cycle's paired console benchmark runs once at the batch boundary.
- The AudioWorklet artifact pin and browser qualification are repinned once at the batch boundary; do not repin here.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (O9, PR #879) and tracker #349 (RT-12/RT-13).

## What the implementer will hit

- Dedicating the Output adds one physical buffer to every plan. `crates/graph-compiler/src/lib.rs:3827`
  asserts `program.buffers < executor_buffers`, and `crates/graph/src/lib.rs:5531` asserts
  `program.buffers <= 2` for a two-node graph; check both, and any resource-report or boot-budget
  fixture that pins arena bytes (`scripts/check-web-boot-budget.mjs`, host-core `prepare` tests).
  If a pinned byte count moves by exactly `2·Q·4` bytes, update the pin and say so.
- The `route_fold` ledger comment at `runtime.rs:5205-5208` ("the colouring gives the session
  output the physical slot of track zero's input buffer") describes the pre-dedication colouring;
  correct the comment, and keep the "opening chain excluded from the in-between scan" clause,
  which is still conservative and still pinned.
- `runtime.buffer(..)` / `buffer_mut(..)` (`runtime.rs:1830-1837`) and
  `test_only_capture_failed_buffer` read the arena; a test that reads the master from the arena
  after a render must read the host planes instead.
- Tests that construct `ArenaMembers` directly (`runtime.rs:6005`, `:6922`, `:7627-8121`) use the
  `Arena` variant of `master`.
- Text-pinning tests split the source on the exact spellings `pub(crate) fn execute(`,
  `pub(crate) fn observe_unit(` and `    fn render(` (`runtime.rs:6088-6095`, `:6180-6192`);
  keep those spellings when adding the `HostMaster` parameter (put it after the existing
  parameters, on a new line if `rustfmt` wraps).
- The C ABI header documents no "output untouched on failure" promise (checked); the web worklet
  already zero-fills on failure. State the new failure-path fill in the render contract comment
  and, if `docs/` describes the executor copy, correct it there in the batch's docs sweep (not in
  this issue's paths).

## Attempt 1 evidence

Implementer: Terra (attempt 1). Branch `codex/916-master-into-host-planes`, from `e178a379`
(#915's verified tip). Implementation checkpoint: `608f0379`. A comment-only follow-up is
`10de67c9`. Every gate below ran on that code. The evidence commit changes only
`crates/graph/tests/MUTATIONS.md` and this section.

### Design

**Engine accessor.** `PlanarBufferMut::stereo_planes_mut` (`crates/engine/src/realtime/buffer.rs`)
returns both planes of a two-channel buffer at once. It splits the storage at `stride` with
`split_at_mut_checked`, then trims each half to `frames` with `get_mut`. A buffer with any other
channel count gets `InvalidPlane`. The split and trims cannot fail on a validated layout, and they
return `InvalidBorrow` rather than panic. The words from `frames` to `stride` are in neither borrow,
so no kernel can reach a padding word. That is how `plane_stride != frames` is honoured, by
construction.

**Dedication.** `is_dedicated` returns `true` for `GraphNodeId::Output`. Two consequences, both
relied on:

- The Output op is never in place: the in-place clause requires `!dedicated`.
- Its slot is never returned to the free list.

On a multi-input output nothing changes. Its op already owned a fresh logical buffer, and its
`last_use` is `ops.len()`, which the colouring never processes. The 64-track console fixture's
arena stays at 193 (`banking_a_dynamic_rack_costs_no_arena_buffers`, unchanged). On a single-input
output that was in place, the arena gains the buffer the op now owns, and nothing ever reads or
writes it. The two pins that counted that case moved by exactly that buffer:

- `program::tests::chain_of_seven_stages_lowers_to_six_ops_three_taps_and_two_buffers`: 2 to 3,
  and the Output is no longer in place.
- `tests::aliased_identity_stages_do_not_change_audio`: `<= 2` to `<= 3`.

**The Output op is identified by node, not by buffer index.** This is deviation 1. Dedication does
not give the Output a slot that no earlier buffer used. `take` may hand it a slot retired before
its op. On the standing console workloads that is track zero's input slot: `route_fold`'s ledger
says so, and re-measuring the ledger's own mutation on this tree confirms it. So
`op.output == program.output` also holds for earlier ops, and routing those to the host would be
wrong. Mutation 916-5 applies exactly that rule, and it is red on gate 1 and on five pre-existing
graph tests (for example `fifty_random_dag_sessions_render_deterministic_nonsilent_pcm`, "the
corpus must not be silent").

The identity is therefore decided once, at bind:

- `runtime::output_op(program, spec)` is `lower`'s own `output_node` resolved through `node_op`,
  checked to write `program.output`.
- `preflight_sequential` resolves it and fails the bind with `graph.scheduler.layout` if it cannot.
  That is unreachable for a lowered program, and it is transactional rather than a panic.
- `validate_fold_installation` checks it is a plain unit of its own (last, so fold faults keep their
  codes). It also installs every folded chain's master as `FoldTarget::Output` when the master op
  is that op, and `FoldTarget::Arena(buffer)` otherwise.
- `build_sequential` records `Runtime::output_unit`.

**`HostMaster` threading.** `GraphExecutor::render` takes the planes once, before any observer
boundary, source work or unit:

1. `output.stereo_planes_mut()?`.
2. `HostMaster::new(left, right, lease.frames())`. It is `None` unless both planes are exactly
   `lease.frames()` words, and `None` means `InvalidEnvelope`.
3. Every unit gets the same value:
   - `Runtime::execute(index, first_sample, host.reborrow())` takes the host by value.
   - `observe_unit(.., validity, &host)` and `observe_active_unit(.., validity, &host)` take a
     shared borrow, because observers only read.

The new parameter is last, and the pinned spellings `pub(crate) fn execute(`,
`pub(crate) fn observe_unit(` and `    fn render(` are unchanged. A later parameter (#918, #920)
goes after `host`, so every call site keeps one shape; the `HostMaster` doc says so.
`GraphExecutor.output` is gone (its only reader was the deleted copy), and so is its line in both
layout witnesses. `Runtime` gains `output_unit: Option<usize>`, mirrored in both runtime layout
witnesses so every derived delta is unchanged. `Runtime::buffer` is now test/test-support only: its
only production caller was the copy.

**Write sites that now target the host planes.**

- **The Output op (`execute_op` with `host: Some`).** Every write of the op's own output goes
  through `output_planes` or `output_and_sidechain_planes`. Those are the host planes for the
  Output op and `lease.write_stereo` / `lease.write_read_stereo` for every other op. That covers
  the `TrackDelay`, `Identity` split-pair, `Route`, `Bound`, `Effect` and `ConsoleEffect` arms, the
  bypass shunt included. The Output's real kinds are `Identity` and `Bound` (rt1 binds a
  processor to it). Staging a delayed input still goes through the arena, because it is an input.
- **The Output op's reduction.** `reduce_plane_into` has `reduce_plane`'s three arms:
  - **Fan-in zero** fills the host plane.
  - **Fan-in one** is `target.copy_from_slice(lease.read(plane, input))`. When the single input is
    the op's own slot, the arm leaves the plane alone. That only happens for a folded master's
    neutralised reduction, because the Output is never in place.
  - **Fan-in two or more** is `reduce_many_into` / `reduce_group_into`. They use the same
    `REDUCE_GROUP` chunking and set `initial_store` for group 0 only. Each group's `N` inputs are
    formed with `lease.read` (`&self`, so they coexist). Then `accumulate_group::<L, N>` runs with
    the host plane as `output`, which is the exact loop `reduce_group` runs after
    `write_read_many`. `disjoint.rs` is untouched.
- **Folded masters.** `ArenaMembers.master` is `MasterPlanes::{Arena(u32), Host(HostMaster)}`.
  `fold_plane`, `fold_cohort` and `fold_resident_tiles` (#915's fused kernel) reach the master only
  through `master_planes()` and check it only through `master_writable()`. `execute` maps
  `FoldTarget::Output` to `MasterPlanes::Host(host)`.
- **Observers of the output.** In both dispatchers the Output unit's observers go to
  `observe_output` / `observe_output_one`, which is `observe`'s planar path over the host planes.
  The binding order, dispatch counters and one planar acquisition per block are the same. The
  #885 resident dispatcher and its text pins are untouched.

**A scatter redirect into the Output op is withheld** (deviation 3). `scatter_target` would admit a
redirect whose consumer is the Output op, for example a bank's last slot read only by the Output.
It would scatter into the Output's arena slot and neutralise its reduction, so the host planes would
never receive the lane. `build_sequential` filters those redirects out (the `scatter_target`
predicate and the corpus that drives it are unchanged). Declined, the chain scatters into its own
slot, and the Output op copies that into the host: one copy, as the end-of-block copy made before.
No compiled session reaches this shape. Mutation 916-6 is red on gate 1's `BankIntoOutput`
shape.

**Why the bits cannot move.** Every kernel writes the same words in the same order with the same
arithmetic, into a destination of the same length (`HostMaster::new` guarantees
`lease.frames()`). Only the destination changed: the host planes instead of arena slot `P`. A
copy, a store or a reload moves an `f32`'s bits unchanged, and the one copy that went (`P` into the
host) only ever moved bits. The one-plane reduction reuses `accumulate_group` verbatim.

**Copies removed per block.**

- Multi-input Output (every console row): the reduction or the fold lands in the host planes, so
  there is one `2·Q` copy fewer.
- Folded Output master: one fewer.
- Single-input Output that was not in place (its producer was dedicated or had another reader):
  two copies become one.
- Single-input Output that was in place: one copy before (the end-of-block copy) and one now (the
  Output op's fan-in-one copy into the host).

No performance claim is made.

### The `Err`-path contract

Stated in `GraphExecutor::render`'s doc comment:

- **`Ok`:** each plane's `frames` words are the block's master, and no padding word is written.
- **Envelope rejection: `output` untouched.** A non-stereo `output` gets `Buffer(InvalidPlane)`. A
  plane that is not `lease.frames()` words gets `InvalidEnvelope`. Both come before any observer
  boundary, source work or unit. `render_inner`'s existing `OutputShape` check sits in front and
  is kept; the executor's check is belt and braces.
- **Executor-level failure: both planes `+0.0`.** This covers a source set failing in
  `begin_block` or `copy_track_input`, and a unit, an observer or an active observer failing in the
  unit loop. `host.silence()` runs straight after `invalidate_observers_after_failure` on each of
  those five paths, and only on them.

### Tests added

- engine `realtime::buffer::tests::stereo_planes_are_the_two_strided_planes_and_leave_the_padding_alone`
  (stride `frames + 3`, both planes, padding untouched, agrees with `plane_mut`)
- engine `realtime::buffer::tests::stereo_planes_refuse_every_other_channel_count`
- graph `runtime::tests::the_host_planes_are_the_arena_oracles_master_bit_for_bit_at_every_stride`
  is gate 1.
  - **Oracle.** `render_arena_oracle` binds the same plan with the `#[cfg(test)]` switch
    `test_only_set_host_master_declined`: no Output unit, arena fold masters, and redirects into
    the Output not withheld. That is the pre-issue runtime. It drives the pre-issue render loop,
    then copies the Output's arena slot into the host layout, which is the deleted end-of-block
    copy.
  - **Candidate.** The real `GraphExecutor::render`, through a directly bound executor, so the
    arena stays inspectable.
  - **Shapes.** Nine, each at `stride == frames` and `stride == frames + 7`, over four blocks of
    seeded noise:
    - 64 folded routes (W8, 13 frames; again at 128 frames)
    - the same with the meters bound through the activation catalog
    - 64 unfolded routes
    - the submix bus plan with its three pads (fan-in-one Output, tracks folded into a bus master,
      one redirect)
    - the bus plan without pads, where the Output's slot is track two's input slot, which the
      tracks' bank gathers
    - one route folded into the Output, and the same route unfolded (the fan-in-one copy)
    - a bank straight into the Output (the withheld redirect)
  - **Assertions, per block.** The host buffer is bit-identical to the oracle's, padding included.
    Every padding word is still `0x7fc0_0916`. The Output's arena slot holds exactly what it held
    before the master's first writer ran in the oracle. The slot is pre-filled each block, and the
    oracle snapshots it just before its first master write: the Output unit, a fold into the slot,
    or a redirected scatter into it.
  - **Assertions, per shape.** The route-fold and redirect counts are pinned, and the oracle's
    are compared. `bank_shape` is equal in both arms. Every observer window equals the oracle's,
    and both Output meters' windows equal the planes the host received.
- graph `runtime::tests::a_failed_render_silences_the_host_planes_and_a_rejected_one_leaves_them_alone`
  is gate 3. A good block is rendered, then a failing one into the same storage (stride
  `frames + 7`). There are five failure arms:
  - cohort 5 of 8 fails its bank processor
  - an Output observer fails through `observe_unit`
  - the same through `observe_active_unit`
  - the source set fails in `begin_block`
  - the source set fails in `copy_track_input`

  Each arm must leave both planes all `+0.0` with the padding intact. Then three rejections must
  leave the storage bit-identical: the executor's short planes (`InvalidEnvelope`), a mono view
  (`Buffer(InvalidPlane)`), and the plan's `OutputShape`.
- graph `runtime::tests::a_host_plane_reduction_is_the_arena_reduction_bit_for_bit`:
  `reduce_plane_into` against `reduce_plane`. Hostile words, fan-in 0 to 19 (both group
  boundaries), frames `{1, 7, 8, 13, 16, 33}`. Different words per plane, a repeated input and the
  silence buffer, a stale arena destination. The neutralised single input must leave the host
  plane untouched.

**Fixture seams (runtime.rs tests).**

- `fold_fixture_parts` and `folded_bus_parts(pads, meters)` split out of their binders.
- `FoldFixture` gains `output_meters` and `failing_cohort`.
- New helpers: `direct_bank_parts`, `FailingTilt`, `FailingObserver`, `FailingSource`,
  `bind_executor`, `render_host`.

**Existing tests edited (all in `crates/graph/src`).**

- `rt9_resident_entry_has_one_guarded_production_caller_and_control` pins the render loop's call
  spellings, which the brief changes. Its expected statements and its two observe-call controls
  now name the new argument, and its loop-body end marker is the region end. Every control is
  still red, because the test asserts `!valid` on each.
- `observed_flag_is_derived_at_construction_and_skips_only_unobserved_units` passes a `HostMaster`.
- The two buffer pins above.
- `ArenaMembers` constructions use `MasterPlanes::Arena`.
- Bank units use `FoldTarget::Arena`.
- `execute_op` calls pass `None`.

### Gates

| gate | command | result |
|---|---|---|
| 1 | gate-1 test | pass: 9 shapes × 2 strides × 4 blocks; also `--release` |
| 2 | `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit`, `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit`, console-workload `the_folded_master_is_the_reductions_own_bits`, `every_standing_workload_folds_one_route_per_track`, `the_plumbing_row_binds_no_strip_at_all` | all pass unchanged; the graph two also `--release` |
| 3 | gate-3 test | pass; also `--release` |
| 4 | `cargo test -p graph` / `--features test-support` | lib 98, rt1 1, rt9 1 / lib 98, rt1 1, rt9 8; 0 failed |
| 4 | `cargo test -p engine` | lib 40, `observation_transport` 4, doc 1; 0 failed |
| 4 | `cargo test -p host-core` | lib 73 plus 12 integration suites, 0 failed (2 pre-existing ignores). One invocation with engine and capi: 23 suites, 0 failed. |
| 4 | `cargo test -p capi` | lib 32, `resource_lifecycle` 4; 0 failed |
| 4 | `cargo test -p graph-compiler -p console-workload` | 73 + 1 + 3 + 1 + 8 + 6; `automation` 4, `chain_shape` 22, `placement` 3; 0 failed |
| 4 | `bash scripts/check-graph-determinism.sh` | PASS (100/100). `target/issue6/fresh-process-determinism.json` is byte-identical to the pre-change run (`cmp`); sha256 `e5d45be6...a8face` both times. |
| 4 | `check-graph-policy.sh` / `check-realtime-policy.sh` / `check-host-core-policy.sh` / `check-capi-abi.sh` | PASS / ok (50 regions in 14 files) / ok / ok |
| 5 | mutation sweep | 916-1 to 916-15 and 916-1b in `crates/graph/tests/MUTATIONS.md`, every row RED |
| std | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean; exit 0 |
| extra | `RUSTFLAGS="-C target-feature=+simd128" cargo check --target wasm32-unknown-unknown -p graph -p engine` | compiles; nothing was run under wasm |
| extra | CI's "Workspace debug tests" step, verbatim, with `--no-fail-fast` | 105 suites green. **14 builtins-compiler tests red**; see "Out-of-path finding". |

Gate 2's clause "assert in the new test against the values the chain-shape tests pin" is deviation
5.

### Mutations

The full table is in `crates/graph/tests/MUTATIONS.md`, "Issue #916". The brief's four rows:

- **Keep the end-of-block copy (916-1 / 916-1b).** Red. 916-1b shows the host bits and padding
  stay equal and only the no-arena-write assertion fires, as the brief required.
- **Write the master to the arena and skip the host (916-2, 916-3).** Red.
- **Fan-in-one copy from the wrong plane (916-4).** Red on the kernel test and gate 1.
- **No fill on the failure path (916-7, 916-8, 916-9, 916-13, 916-14).** One row per path, each red
  on gate 3.

Beyond the brief, these are also red:

- 916-5: the brief's buffer-index identification
- 916-6: the withheld redirect
- 916-10: the fill on success
- 916-11: the Output undedicated
- 916-12: no plane-length check
- 916-15: Output observers reading the arena

### Deviations

1. **Identity by node, not `op.output == self.output`.** See Design. The brief's premise that
   dedication makes the Output's slot unshared does not hold (row 916-5).
2. **`Runtime.output` is `output_unit: Option<usize>`**, a unit index, not an arena buffer id.
   `GraphExecutor.output` is removed as dead.
3. **The redirect into the Output op is withheld**, which the brief did not name. It is needed for
   correctness, and no standing workload reaches it.
4. **The `route_fold` ledger sentence is not corrected.** The brief expected dedication to change
   it ("the colouring gives the session output the physical slot of track zero's input buffer").
   It does not: a multi-input Output's colouring is unchanged. I re-measured the ledger's own
   mutation on this tree (the opening chain included in the in-between scan).
   `every_standing_workload_folds_one_route_per_track` goes red (`nine_track_baseline`: 0 of 9
   folded). The sentence stays true, and the opening-chain clause stays load-bearing. The
   graph-compiler-only compile of the console fixture gives the Output a fresh slot. So my own
   new comments say "the standing console workloads", not "every console fixture" (`10de67c9`).
5. **Gate 2's "assert in the new test" for `bank_route_folds`/`bank_shape` of the standing
   workloads is not a new assertion.** A graph test cannot compile a standing workload (graph does
   not depend on the compilers), and `tools/console-workload/tests/chain_shape.rs` is outside the
   authorized paths. The chain-shape tests that pin those values pass unchanged. Gate 1 asserts the
   route-fold and redirect counts and `bank_shape` of its own shapes against the oracle.
6. **Comment edits outside the named items, each made false by this issue:**
   - `program.rs`'s "the executor copies it out afterwards" at the output's `last_use`
   - `scatter_target`'s "not the session output" bullets
   - `buffer_mut`'s "host copy-out"
7. **`MasterPlanes::Host` wraps a `HostMaster`**, not two bare slices. This is equivalent.
8. **`observe_output` is a small sibling of `observe`**, not a new parameter on it. The #885
   dispatcher and its text pins stay untouched.
9. **`preflight_sequential` can refuse with `graph.scheduler.layout`** if the Output op is missing
   or is not a plain unit. That is unreachable, and it replaces what would otherwise be a bind-time
   panic.

### Out-of-path finding (not edited; needs a decision)

Dedicating the Output changes one bind decision the brief did not anticipate: scalar fader/matrix
pair admission. `chains_into` declines a pair whose fader buffer is `program.output`
(`producer.output == program.output`). Before, a single-input Output folded in place onto the last
fader/matrix buffer, and that pair declined. Now the Output owns its buffer, and the pair is
admitted.

The builtins-compiler harness depends on the old decline. Its comment at
`crates/builtins-compiler/src/lib.rs:6126` reads: "Track 0 is already ineligible because its
post-matrix buffer is the graph output". With `--features test-support`, 14 tests are red at
`608f0379` and green at `e178a379`. In the lib (11):

- `actual_graph_mono_collapse_disengages_on_input_command_and_recovers_nonfinite_input`
- `actual_scalar_extra_reader_declines_and_retains_separate_owner_pcm`
- `actual_scalar_graph_queues_fuse_and_fall_back_against_separate_owners`
- `actual_scalar_nonadjacent_failed_render_materializes_post_fader_before_error`
- `actual_scalar_nonadjacent_intervening_observer_error_completes_and_retries`
- `actual_scalar_nonadjacent_physical_output_conflict_declines_before_owner_transfer`
- `actual_scalar_nonadjacent_ramp_retarget_and_failed_retry_stay_at_original_boundaries`
- `actual_scalar_nonadjacent_schedule_selects_the_split_owner`
- `actual_scalar_overlapping_nonadjacent_candidates_select_one_and_keep_the_other_separate`
- `serialized_alias_observer_is_the_decline_boundary`
- `serialized_live_fader_matrix_is_selected_by_the_bound_render_path`

In `tests/allocation_tracker.rs` (3):

- `actual_queued_graph_phases_allocate_and_free_nothing`
- `actual_queued_scalar_graph_allocates_and_frees_nothing`
- `actual_scalar_split_table_and_failed_render_fit_the_resource_gate`

In every one, the first failing assertion is a pairing witness: factory, fused or fallback counts,
the selected split fader's identity, the offer count, or the witness's fader-state words. None is a
PCM or allocation assertion. Where `render_scalar_pair_and_compare`'s PCM equality runs first
(`actual_scalar_graph_queues_fuse_and_fall_back_against_separate_owners`,
`actual_scalar_nonadjacent_schedule_selects_the_split_owner`), it passes. Because each test stops at
its first failure, the later PCM and allocation gates in these 14 were not reached on this tree.

The newly admitted pair looks sound. Its fader/matrix buffer is read only by the Output op, after
the matrix slot, exactly like any other admitted pair's consumer. Two ways forward, which are the
owner's call:

- **(a)** Update the harness in builtins-compiler, outside this issue's paths.
- **(b)** Add a clause that keeps the old decline. It would emulate the old in-place colouring
  ("the Output would have folded onto this buffer"). A cruder form, "the matrix's sole reader is
  the Output op", was tried as a diagnostic, turned 13 tests red, and was reverted.

### Anchor drift and notes

- #915 shifted the runtime anchors:
  - `reduce_plane` is at `:362`.
  - `fold_plane` / `fold_cohort` are at `:1516` / `:1529`.
  - `Runtime` is at `:1871`.
  - `RouteFold` is at `:5700`.
  - the `route_fold` ledger is at `:5822` and `:5952`.
  - the text pins split at about `:6260` and `:6352` (rt9 test).
- The web boot budget (`check-web-boot-budget.mjs`) was not run. It runs inside
  `check-web-audioworklet.sh`, the batch-boundary AudioWorklet gate. It pins parse-transient growth
  (17 × the 1 MiB document plus one page), not arena bytes, so an extra `2·Q·4` arena buffer on a
  single-input output cannot move it.
- **Risks.**
  - The Output op carries one `Option` test per output write site. That adds branches, not
    arithmetic.
  - `lease.read` panics on an out-of-range buffer, where `write_read_many` returned `None`. Every
    input of a lowered op is a reserved buffer.
  - A single-input output costs one never-touched arena buffer.
