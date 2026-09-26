# Read plain-strip sources in place from the played transfer block

Drafted from `DRAFTS/PLAN.md`, Part 2. Depends on drafts A and B; the row it can move is the one
draft D adds (`sixty_four_track_plumbing_ring`); `sixty_four_track_plumbing_only` is not refed
(coordinator ruling).

## Product outcome

A ring-fed session copies every source claim's played planes into the claim's arena buffer at
the top of each block (`copy_track_input`), and #918 let a **bank's gather** read the played
planes instead. After drafts A and B, an unbanked track's whole chain is that copy followed by the
fused Output kernel reading the buffer once. Extend #918's in-place table to a claim whose
buffer's only reader is the fused Output kernel, and let that kernel read the played planes for
such inputs. Class A: the words the kernel reads are the words the copy would have written
(#918's contract: "the words are the ones `copy_track_input` would have written for the claim").
The bench's equivalent of the removed pass is 4,224 cycles of copy plus 64 dispatches per block
(table C); the production pass is `copy_track_input` per claim.

## Root evidence

- `crates/graph/src/lib.rs:2200-2231`: `lent_claims` are offered when the driver
  `provides_played_planes()` (`:1653`), `build_sequential` decides which are bound in place, and
  the copy loop (`:2321-2331`) is filtered by `runtime.source_in_place(claim, buffer)`
  (`runtime.rs:2289`).
- `runtime.rs:5324` `source_plane_table`: builds `source_plane_of_buffer` (`:1989`; sentinel
  `NO_SOURCE_CLAIM`, `:1451`) for **bank gathers only**; `SourceGather` (`:1456`) and
  `ArenaMembers::plane` (`:1526`) read `sources.played_planes(claim)` (`GraphSourcePlanes`,
  `lib.rs:1674`) for a lane whose `source_lanes` bit is set.
- `runtime.rs:2374-2416` `execute` passes `sources` to the Bank arm only; the Op arm (the
  Output op after draft B) never sees them.
- `tools/console-workload/src/lib.rs:1513-1575` `FrozenGraphSource` is a `GraphRuntimeProcessor`
  bound to each `Input` node; `process` copies its frozen block into the arena. The row binds no
  source set, so no claim exists to be lent.
- #918's gates and `test_only_set_source_in_place_declined` (`lib.rs:2211`) are the model.

## Smallest closable slice

Authorized paths: `crates/graph/src/runtime.rs` (`source_plane_table`, the Output op's fused
kernel input selection, `execute`'s Op arm), `crates/graph/src/lib.rs` (none expected), their tests,
`crates/graph/tests/MUTATIONS.md`, and this spec.

1. `source_plane_table`: also admit a lent claim whose buffer is an input of the fused Output op
   (draft B's `route_table` inputs) and is read by nothing else (`op_dataflow` readers), setting
   `source_plane_of_buffer[buffer] = claim`.
2. `execute`: pass `sources` to the Op arm when the unit is `output_unit`; the fused kernel forms
   each input's planes as `sources.played_planes(claim)` when
   `source_plane_of_buffer[buffer] != NO_SOURCE_CLAIM`, else `lease.read`. **Underrun rule:** on
   `played_planes(claim) == None` the kernel reads the arena's silence buffer
   (`ARENA_SILENCE_BUFFER`), exactly as `ArenaMembers::plane` (`runtime.rs:1526`) does for a bank
   lane, which is the `+0.0` block `copy_track_input` would have written.
3. `source_in_place` answers true for such a claim, so the copy loop skips it.

## Non-goals

No change to #918's bank path, to the ring (`crates/source`), to the bench's `FrozenGraphSource`,
or to a claim with any other reader (a send, an observer tap, a delayed edge): those keep the copy.

## Objective gates

1. Graph test with a driver that offers played planes: a 64-claim plan with in-place routes into
   the Output, hostile input, frames `{1, 7, 16, 128}`: host planes bit-identical to the same plan
   with `test_only_set_source_in_place_declined(true)` over 8 blocks;
   `test_only_source_plane_counts()` (`runtime.rs:4567`, `[u64; 3]`) shows 64 in-place reads and
   0 copies per block with the table on and the reverse with it off; `output_route_folds() == 64`
   both ways.
2. Declining shapes: a claim with a send tap reader, a claim under a delayed edge, a claim read by
   a submix: copied (count 1 each), output identical.
3. Underrun: a driver that answers `None` for one claim on block 3 renders that block bit-identical
   to the declined arm (whose `copy_track_input` wrote `+0.0`), with the silence buffer read and no
   copy counted.
4. Red mutations: read `lease.read` for an admitted claim after skipping its copy (stale words:
   gate 1 fails on block 2); admit a claim with two readers (gate 2); read the claim's stale arena
   buffer instead of the silence buffer on `None` (gate 3).
5. `cargo test -p graph` both ways, `-p source`, `-p host-core`; `scripts/check-graph-determinism.sh`,
   `check-graph-policy.sh`, `check-realtime-policy.sh`.

## Console benchmark rows

`sixty_four_track_plumbing_ring` only (draft D's row: the same session fed through a
`FrozenSourceDriver` that lends played planes; its `source set` phase is the copy loop this issue
removes). `sixty_four_track_plumbing_only` binds `FrozenGraphSource` processors, not a source set,
so neither #918 nor this issue can show on it, and by ruling it is not refed. "In place" has no
meaning for a bound processor: `GraphRuntimeProcessor::process` is write-only into the block it
is handed, so the frozen processor's copy *is* its contract.

## Dependencies

After "Lower identity-bound track stages as aliases" and "Fuse in-place routes into the Output
reduction in pairs" (forced: the kernel that reads the played planes is B's). After #917/#918
(merged). "Add a driver-fed plumbing row to the console benchmark" must be merged for the
batch-boundary measurement to show it.

## Standing rules for the implementer

- Work only from this body. Read the cited functions and #918's spec first; do not survey the
  workspace.
- Class A means the change moves no rendered bit. Every gate above that says "bit-identical" is a
  hard stop.
- The owner's copy rule: this issue removes a block-sized copy per claim; do not introduce one.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh`
  is mandatory). `crates/graph` stays free of `unsafe`.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features
  -- -D warnings`, and the focused tests named above before every checkpoint. Commit on a
  `codex/<issue>-<slug>` branch from synchronized `main`.
- Do not quote a projected saving.
- The AudioWorklet artifact pin and browser qualification are repinned once at the batch boundary.
- Source of these findings: `.github/ISSUE_SPECS/DRAFTS/PLAN.md`,
  `.github/ISSUE_SPECS/918-gather-banked-source-inputs-from-the-played-transfer-block.md`.

## What the implementer will hit

- The played block is released at the next `begin_block` (#917); the Output op runs last in the
  block, inside the borrow `render` forms at `lib.rs:2337-2338`, so the lifetime argument #918
  made for the bank gather holds unchanged.
- `source_plane_table` sets a per-lane bit in `UnitIdentity::source_lanes` (`runtime.rs:1888`)
  for banks; the Output op has up to 64 inputs, so its selection is by
  `source_plane_of_buffer` lookup per input, not by a bit field.
- The seam `test_only_set_source_in_place_declined` already covers both admission sites once
  `lent_claims` is empty; gate 1's declined arm needs no second seam.
