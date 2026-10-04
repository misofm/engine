# Carry compensation lines across a plan swap

Slice 13 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

In a session with plug-in delay compensation, adding a track or a zero-latency effect no longer
empties the compensation lines: every compensated path keeps its delayed audio through the swap, at
the output and inside submixes alike. A path whose own compensation changes gets a documented, local
transition instead of a silent line.

## Context

- PDC delays each edge into a node by that node's latest incoming arrival minus the edge's own
  arrival (`crates/graph-compiler/src/pdc.rs:60-89`), at every node with several inputs: the output,
  every submix input and every effect with a sidechain. Each delay is an `InsertedDelay { node,
  edge_id, samples }` (`crates/graph/src/lib.rs:343`).
- At runtime a line is two rings and one cursor (`CompensationDelay`,
  `crates/graph/src/runtime.rs:940-1019`), in `Runtime::delays` (`:2509`), reached through
  `StagedInput::line` (`:1295-1302`).
- Edge IDs are stable: `GraphEdgeId::{TrackMain, RouteSource, RouteDestination, EffectSidechain}`
  (`graph/src/lib.rs:305-310`), keyed by node, route or effect.
- The scaffold and the effect carries come from *Carry per-node effect instances across a plan swap*
  (#1282).

## Decisions frozen for this slice

- **D1. Key and rule.** A line carries when its `GraphEdgeId` exists in both plans and the edge's
  source and destination nodes are the same in both. A re-pointed route (same route ID, new source or
  destination) is not carried: its line starts at rest, a documented transition on that path.
- **D2. Equal length** moves: swap the two rings and the cursor.
- **D3. Different length** is copied **head-aligned**: the successor first emits `P` samples of
  `+0.0` (`P = 0` here; a later pre-roll may set `P > 0`), then the predecessor's pending samples in
  the order the predecessor would have emitted them, truncated to fit, or followed by `+0.0` to fill.
  So a path whose compensation grew by `δ` has a gap of `δ` samples after its pending audio, and one
  whose compensation shrank by `δ` skips `δ` samples. With no surviving node's arrival changing, an
  unchanged edge always has equal length (umbrella P7); slice 15 keeps arrivals from dropping.
- **D4. One routine.** One allocation-free copy routine serves both directions and both channels.

## Deliverables

1. D1-D4 in `crates/graph` (location table, carry routine) and the inventory rows in `crates/host-core`.
2. Unit tests of D3's copy in `crates/graph`: equal, longer and shorter successor lengths; a
   predecessor cursor at every position modulo the block; a wrapping source cursor.
3. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`

## Non-goals

- No change to how PDC computes delays; no floors (slice 15); no pre-roll (slice 17).
- No strip or submix input delay lines and no live send ramps (slice 14).

## Objective gates

1. **Gap-free acceptance at the output.** Session A: three tracks, the first with a true-peak limiter
   insert (latency `Fs/100 + 6`), so the other two edges into the output carry lines. Session B adds a
   muted track with no latent effect; no node's arrival changes. The swapped run (swap after block 6)
   and a fresh B fed from frame 0 are bit-identical for every block, at 44.1, 48, 88.2 and 96 kHz.
   With this slice's section disabled, the compensated tracks drop out for the line's length.
2. **Inside a submix.** Session A also routes two tracks into a submix, one of them through a limiter
   insert, so the other's edge into the submix carries a line; B adds a muted zero-latency track to
   that submix. Bit-identical for every block.
3. **Head alignment** (deliverable 2) passes.
4. **Realtime.** The swap block makes zero allocations and frees.
5. Commands:
   - `cargo test --locked -p graph -p graph-compiler -p host-core --features graph/test-support,host-core/test-support`
   - `bash scripts/check-graph-determinism.sh`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - the umbrella's inherited gates.

## Test value

- Gate 1: a line keyed by delay-vector index (which shifts when a track is added) instead of edge ID,
  or a line left at rest, turns it red.
- Gate 2: a carry that only handles edges into the output leaves submix lines at rest; it turns red.
- Gate 3: a copy that starts from the predecessor's write position instead of its read position
  turns it red.

## Dependencies

- *Carry per-node effect instances across a plan swap* (#1282).
