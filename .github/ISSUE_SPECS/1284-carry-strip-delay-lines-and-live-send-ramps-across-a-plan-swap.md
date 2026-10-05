# Carry strip delay lines and live send ramps across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8, D15-9).
Slice 14 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

Three kinds of state keep their exact values through a plan swap, in move mode and in copy mode:
- a strip's input alignment delay (`delay_samples`, used for multi-mic alignment);
- a submix's input delay;
- a live send whose level is ramping at the swap.

After this slice every DSP state family of a plan carries in both modes. A structural edit that
changes no node's latency is then bit-continuous for every unchanged path. Meters follow in #1327.
A transaction that also changes a carried send's live value sounds like "live edit, then structural
edit" (D15-7). A changed `delay_samples` restarts its line, and the strip is reported for the
duck-swap transition (D15-9).

## Context

- A strip's `delay_samples` is a `TrackDelayLine` (`crates/graph/src/runtime.rs:1021`). It is driven
  by `NodeKind::TrackDelay` on a track input or `NodeKind::SumDelay` on a submix input
  (`:1106-1132`), and held in `Runtime::track_delays` (`:2513`).
  - It is not latency (`crates/graph-compiler/src/pdc.rs:1-24`).
  - A strip whose delay is 0 on both lanes has no line.
- A live send is `NodeKind::LiveRoute(Box<LiveRoute>)` (`runtime.rs:861-874`): an `IndexedRamp`,
  its position, its mute and its control lane. At bind it is settled on the prepared coefficients.
  Its `drain` (`:896`) applies the records present at entry directly to the ramp.
- The send records the classifier emits arrive with *Deliver value-only send and submix-strip edits
  to the running C ABI plan* (#1225) and *Let C ABI sends follow their source strip's mute live*
  (#1226). Until they land, every send value change is a prepared difference. Decision 15 also makes
  `follows_mute` live in the browser (#1342).
- The scaffold, both modes, the base, the retarget pattern and the restart set come from #1322,
  #1277 (D2-D6) and *Carry compensation lines across a plan swap* (#1283).

## Decisions frozen for this slice

- **D1. Strip delay lines.** The key is the strip ID and the lane. A line carries when the lane's
  `delay_samples` is equal in the base and in the successor's model. The lengths are then equal.
  - Move mode swaps the two rings and the cursor.
  - Copy mode copies them into the successor's preallocated line.
  - A changed `delay_samples` is a prepared value (decision 14; D15-9). Its line starts at rest, and
    the strip goes into the restart set, so #1324's duck-swap removes the click.
- **D2. Live sends.** The key is the route ID. The route's source, tap and destination are
  structural: if any of them differs, the send does not carry, and its source strip goes into the
  restart set. Gain, matrix, mute and `follows_mute` are live exactly when the classifier emits a
  record for their change (#1277 D3). Any other difference in them is prepared and restarts the
  send.
  - **Move mode:** at the swap block, drain the predecessor's route lane (the records present at
    entry), then copy the ramp, its position and its mute.
  - **Copy mode:** the same drain-then-copy. The drain applies records at the boundary the
    predecessor's next block would apply them, so it is bit-neutral (#1322 D2).
- **D3. Carry, then retarget.** For a carried send whose live values differ, the successor entry
  points push the classifier's route records into the successor's route lane before they return.
  In copy mode the records are kept for publication (#1277 D5). This slice extracts the per-route
  record derivation the same way #1277 D5 did for strips. If #1225 has not landed, the classifier
  emits no route record, D3 has nothing to push, and gate 3 runs when #1225 lands.

## Deliverables

1. D1-D3 in `crates/graph`, and the inventory rows, the join and D3 in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`
- `crates/host-core/src/live_delta.rs`: D3's extraction only. This is stream B's file, and root
  orders the merge.

## Non-goals

- No change to delay or ramp kernels. No floors (#1285).
- No duck-swap itself (#1324).

## Objective gates

1. **Gap-free acceptance.** Session A has:
   - one track with `delay_samples` on both lanes (different values);
   - two tracks routed into a submix with `delay_samples`;
   - a live send from a third track into that submix.

   Prepare with live controls. Admit a send-level record with a multi-block ramp between blocks 5
   and 6. Prepare B (A plus a muted track whose ID sorts first) from the base. Every block equals
   the reference fed the same record at the same block.
2. **Prepared changes restart.** A strip whose `delay_samples` changed starts its line at rest. A
   send re-pointed to another submix starts at its prepared value. `restarted_strips()` is exactly
   those two source strips.
3. **Carry, then retarget** (with #1225 on `main`). B also changes a carried send's gain. Run 1 is
   the structural swap with D3's retarget. Run 2 pushes the same record to A just before the swap
   and prepares B from a base that holds it. The two runs are bit-identical.
4. **Copy mode.** Gate 1 with a copy after block 6, followed at once by the adoption. Every block
   equals the move run, and A equals its uncopied twin.
5. **Realtime.** The swap block and the copy call make zero allocations and frees.
6. Commands:
   - `cargo test --locked -p graph -p host-core --features graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: any of three defects turns it red:
  - a strip or submix delay line left at rest drops the delayed strip for its delay length;
  - a send ramp restarted from its target steps the level;
  - a record is lost because the copy ran before the drain.
- Gate 2: carrying a line whose `delay_samples` changed plays the old alignment. A send carried into
  a new destination mixes the old ramp into the wrong bus. Either turns it red.
- Gate 3: a missing retarget keeps the old send level. It turns red.
- Gate 4: a copy-mode ring swap leaves the predecessor with the successor's at-rest line. Its twin
  turns red.

## Dependencies

- *Carry compensation lines across a plan swap* (#1283).
- For gate 3 only: *Deliver value-only send and submix-strip edits to the running C ABI plan*
  (#1225).
