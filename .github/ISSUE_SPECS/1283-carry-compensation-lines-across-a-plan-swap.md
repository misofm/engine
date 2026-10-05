# Carry compensation lines across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8, D15-9).
Slice 13 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

In a session with plug-in delay compensation, adding a track or a zero-latency effect no longer
empties the compensation lines. Every compensated path keeps its delayed audio through the swap, at
the output and inside submixes alike. A route that the transaction
re-points does not carry its line under its route ID, and its source strip is never ducked for it:
a re-pointed route is a route removed plus a route added, which *Ramp a route that a plan swap adds
to or removes from a surviving strip* (#1363) ramps at route level (D15-9).

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
- The program scaffold is today's move-mode `GraphCarryProgram` (`crates/graph/src/lib.rs:2757`),
  run at the swap block; the predecessor then retires. The restart set and the per-node carries
  come from *Carry fader, mute and pan ramps across a plan swap* (#1277, D6) and *Carry per-node
  effect instances across a plan swap* (#1282).

## Decisions frozen for this slice

- **D1. Key and rule.** A line carries when its `GraphEdgeId` exists in both plans and the edge's
  source and destination nodes are the same in both. A line has no live value, so there is no
  retarget.
  - A re-pointed route keeps its route ID but gets a new source or destination. Its line does not
    carry: the new route's line starts at rest. Its source strip does **not** go into the restart
    set. #1363 D1(b) and D4 keep the old route as a fading route under its own graph ID and fade
    the new route in.
  - The lookup is by successor edge ID, so *Ramp a route that a plan swap adds to or removes from a
    surviving strip* (#1363 D4) can extend it with an alias: a fading route's edge carries from the
    predecessor's edge of the original route ID, and a predecessor line named by an alias carries
    only into the alias (the successor's edge under the original ID then starts at rest, even with
    unchanged endpoints). This slice's lookup maps each successor edge to the predecessor edge
    it reads, which is the same ID here; #1363 D4 supplies the alias entries.
  - An effect owner that restarts (#1279 D1) does not stop the lines around it from carrying.
- **D2. Equal length, move mode.** Swap the two rings and the cursor.
- **D3. Only edge lines.** This slice carries only lines keyed by a `GraphEdgeId`. A source-claim
  line (a raw-source line on a source-reading node, *Grow latency during playback by adopting a
  primed warm successor*, #1287 D1) is not an edge. *Carry source-claim lines across a plan swap
  and fill a grown line for a prime* (#1402) carries it, keyed by claiming node and source, and
  fills it by #1287's L2 and L3.
- **D4. Different lengths: a head-aligned copy.** The successor emits the predecessor's pending
  samples first, in the order the predecessor would have emitted them, truncated to fit or followed
  by `+0.0`.

  So a path whose compensation grew by `δ` has a gap of `δ` samples after its pending audio, and one
  whose compensation shrank by `δ` skips `δ` samples. With no surviving node's arrival changing, an
  unchanged edge always has equal length. *Keep every node's latency from dropping during
  playback* (#1285) keeps arrivals from dropping. In a warm adoption (#1287) every edge between
  carried nodes keeps its length (its L1), so growth never reaches this gap.
- **D5. One routine.** One allocation-free copy routine serves D4 in both directions and both
  channels.

## Deliverables

1. D1-D5 in `crates/graph` (the location table and the carry routine), and the inventory rows and
   the join in `crates/host-core`.
2. Unit tests of D4's copy in `crates/graph`: equal, longer and shorter successor lengths; a
   predecessor cursor at every position modulo the block; a wrapping source cursor.
3. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`

## Non-goals

- No change to how PDC computes delays. No floors (#1285), no source-claim lines (#1287 D1) and no
  claim-line carry (#1402).
- No strip or submix input delay lines, and no live send ramps (#1284).
- No route-level ramp for a re-pointed route, and no fading-route line alias (#1363 D4).

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
4. **A re-pointed route does not duck its strip.** B re-points one route to another submix. Its
   line under the route ID starts at rest, every other line carries, and `restarted_strips()` is
   empty.
5. **Realtime.** The swap block makes zero allocations and frees.
6. Commands:
   - `cargo test --locked -p graph -p graph-compiler -p host-core --features graph/test-support,host-core/test-support`
   - `bash scripts/check-graph-determinism.sh`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: a line keyed by delay-vector index (which shifts when a track is added) instead of edge
  ID, or a line left at rest, turns it red.
- Gate 2: a carry that handles only edges into the output leaves submix lines at rest. It turns red.
- Gate 3: a copy that starts from the predecessor's write position instead of its read position
  turns it red.
- Gate 4: a rule keyed only by route ID carries a re-pointed route's line into the wrong bus. A
  rule that still restarts the source strip for a re-pointed route puts it in the restart set.
  Either turns it red.

## Dependencies

- *Carry per-node effect instances across a plan swap* (#1282).
