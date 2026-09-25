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
