# Carry compensation lines across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8).
Slice 13 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

In a session with plug-in delay compensation, adding a track or a zero-latency effect no longer
empties the compensation lines. Every compensated path keeps its delayed audio through the swap, at
the output and inside submixes alike, in move mode and in copy mode. A route that the transaction
re-points does not carry its line. Its source strip is reported for the transition of *Duck-swap a
strip whose state cannot continue across a plan swap* (#1324).

## Context

- PDC delays each edge into a node by that node's latest incoming arrival minus the edge's own
  arrival (`timings`, `crates/graph-compiler/src/pdc.rs:38`; `max` at `:61-65`, the per-edge delay
  at `:66-88`). It does this at every node with several inputs: the output, every submix input,
  and every effect with a sidechain. Each delay is an `InsertedDelay { node, edge_id, samples }`
  (`crates/graph/src/lib.rs:344`).
- At runtime a line is two rings and one cursor (`CompensationDelay`,
  `crates/graph/src/runtime.rs:940-944`), held in `Runtime::delays` (`:2509`) and reached through
  `StagedInput::line` (`:1295-1302`).
- Edge IDs are stable: `GraphEdgeId::{TrackMain, RouteSource, RouteDestination, EffectSidechain}`
  (`graph/src/lib.rs:306-311`), keyed by node, route or effect.
- The program scaffold, both modes, the restart set and the per-node carries come from *Carry plan
  state by copy as well as by move* (#1322), *Carry fader, mute and pan ramps across a plan swap*
  (#1277, D6) and *Carry per-node effect instances across a plan swap* (#1282).

## Decisions frozen for this slice

- **D1. Key and rule.** A line carries when its `GraphEdgeId` exists in both plans and the edge's
  source and destination nodes are the same in both. A line has no live value, so there is no
  retarget.
  - A re-pointed route keeps its route ID but gets a new source or destination. Its line does not
    carry: it starts at rest, and the route's source strip goes into the restart set.
  - An effect owner that restarts (#1279 D1) does not stop the lines around it from carrying.
- **D2. Equal length, move mode.** Swap the two rings and the cursor.
- **D3. Equal length, copy mode.** Copy both rings and the cursor into the successor's
  preallocated line (`copy_from_slice`). The predecessor's line is only read. The bytes go into
  `carry_program_copy_bytes`.
- **D4. Different lengths, both modes: a head-aligned copy.**
  1. The successor first emits `P` samples of `+0.0`. `P` is a parameter of the carry program; it is
     0 unless *Pre-roll a successor whose latency grows* (#1287) sets it.
  2. Then it emits the predecessor's pending samples, in the order the predecessor would have
     emitted them, truncated to fit or followed by `+0.0`.

  So a path whose compensation grew by `δ` has a gap of `δ` samples after its pending audio, and one
  whose compensation shrank by `δ` skips `δ` samples. With no surviving node's arrival changing, an
  unchanged edge always has equal length. *Keep every node's latency from dropping during
  playback* (#1285) keeps arrivals from dropping, and #1287 makes growth exact.
- **D5. One routine.** One allocation-free copy routine serves D4 in both directions, both
  channels and both modes.

## Deliverables

1. D1-D5 in `crates/graph` (the location table and the carry routine), and the inventory rows and
   the join in `crates/host-core`.
2. Unit tests of D4's copy in `crates/graph`: equal, longer and shorter successor lengths; a
   predecessor cursor at every position modulo the block; a wrapping source cursor; `P` of 0 and of
   one quantum.
3. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`

## Non-goals

- No change to how PDC computes delays. No floors (#1285) and no catch-up (#1287).
- No strip or submix input delay lines, and no live send ramps (#1284).

## Objective gates

1. **Gap-free acceptance at the output.** Session A has three tracks. The first has a true-peak
   limiter insert (latency `Fs/100 + 6`), so the other two edges into the output carry lines.
   Session B adds a muted track with no latent effect, so no node's arrival changes. The swapped run
   (swap after block 6) and a fresh B fed from frame 0 are bit-identical in every block, at 44.1,
   48, 88.2 and 96 kHz. With this slice's section disabled, the compensated tracks drop out for the
   line's length.
2. **Inside a submix.** Session A also routes two tracks into a submix, one of them through a
   limiter insert, so the other's edge into the submix carries a line. B adds a muted zero-latency
   track to that submix. Bit-identical in every block.
3. **Head alignment** (deliverable 2) passes.
4. **A re-pointed route restarts.** B re-points one route to another submix. Its line starts at
   rest, every other line carries, and `restarted_strips()` is exactly the route's source strip.
5. **Copy mode.** Gates 1 and 2 with a copy after block 6, followed at once by the adoption. Every
   block equals the move run, and A, rendered on after the copy, equals its uncopied twin.
6. **Realtime.** The swap block and the copy call make zero allocations and frees.
7. Commands:
   - `cargo test --locked -p graph -p graph-compiler -p host-core --features graph/test-support,host-core/test-support`
   - `bash scripts/check-graph-determinism.sh`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: a line keyed by delay-vector index (which shifts when a track is added) instead of edge
  ID, or a line left at rest, turns it red.
- Gate 2: a carry that handles only edges into the output leaves submix lines at rest. It turns red.
- Gate 3: a copy that starts from the predecessor's write position instead of its read position,
  or that ignores `P`, turns it red.
- Gate 4: a rule keyed only by route ID carries a re-pointed route's line into the wrong bus. It
  turns red.
- Gate 5: a copy-mode swap of the rings leaves the predecessor with the successor's at-rest line.
  Its twin turns red.

## Dependencies

- *Carry per-node effect instances across a plan swap* (#1282).
