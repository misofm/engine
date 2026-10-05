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
A changed `delay_samples` restarts its line, and the strip is reported for the duck-swap transition
(D15-9). A re-pointed send never ducks its source strip: it is a route removed plus a route added,
which *Ramp a route that a plan swap adds to or removes from a surviving strip* (#1363) ramps at
route level (decision 15, D15-9). The retarget of a carried send whose live value changed is
*Deliver value-only send edits to the running C ABI plan* (#1225) D8.

## Context

- A strip's `delay_samples` is a `TrackDelayLine` (`crates/graph/src/runtime.rs:1021`). It is driven
  by `NodeKind::TrackDelay` on a track input or `NodeKind::SumDelay` on a submix input
  (`:1106-1132`), and held in `Runtime::track_delays` (`:2513`).
  - It is not latency (`crates/graph-compiler/src/pdc.rs:1-24`).
  - A strip whose delay is 0 on both lanes has no line.
- A live send is `NodeKind::LiveRoute(Box<LiveRoute>)` (`runtime.rs:861-874`): an `IndexedRamp`,
  its position, its mute and its control lane. At bind it is settled on the prepared coefficients.
  Its `drain` (`:896`) applies the records present at entry directly to the ramp.
- The send records the classifier emits arrive with #1225 and *Let C ABI sends follow their
  source strip's mute live* (#1226); decision 15 also makes `follows_mute` live in the browser
  (#1342). Until they land the classifier emits no route record, so every send value change is a
  prepared difference. #1225 D8 adds the carried send's retarget when it makes them live; this slice
  does not depend on it.
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
  structural.
  - If any of them differs, the route is re-pointed (decision 15, D15-9): its
    ramp state does not carry, and its source strip does **not** go into the restart set. #1363
    D1(b) ramps it at route level, as the old route removed plus the new route added.
  - Gain, matrix, mute and `follows_mute` are live exactly when the classifier emits a record for
    their change (#1277 D3). A send whose only differences are live carries.
  - A send with any other value difference does not carry and starts at its prepared value. Its
    source strip does not go into the restart set; #1363 handles it like a re-pointed route. Once
    #1225, #1226 and #1342 have landed, every value of a send into a submix is live, so this case no
    longer arises.
  - **Move mode:** at the swap block, drain the predecessor's route lane (the records present at
    entry), then copy the ramp, its position and its mute.
  - **Copy mode:** the same drain-then-copy. The drain applies records at the boundary the
    predecessor's next block would apply them, so it is bit-neutral (#1322 D2).
- **Moved out.** The former D3 (carry, then retarget for sends) and its gate are #1225 D8 and its
  gate 7. This slice writes no route record and edits no classifier code.

## Deliverables

1. D1-D2 in `crates/graph`, and the inventory rows and the join in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`

## Non-goals

- No change to delay or ramp kernels. No floors (#1285).
- No duck-swap itself (#1324), and no route-level ramp (#1363).
- No send retarget (#1225 D8).

## Objective gates

1. **Gap-free acceptance.** Session A has:
   - one track with `delay_samples` on both lanes (different values);
   - two tracks routed into a submix with `delay_samples`;
   - a live send from a third track into that submix.

   Prepare with live controls. Admit a send-level record with a multi-block ramp between blocks 5
   and 6. Prepare B (A plus a muted track whose ID sorts first) from the base. Every block equals
   the reference fed the same record at the same block.
2. **Prepared changes.** A strip whose `delay_samples` changed starts its line at rest. A send
   re-pointed to another submix starts at its prepared value, and its ramp is not carried.
   `restarted_strips()` is exactly the delay-changed strip; the re-pointed send's source strip is
   not in it.
3. **Copy mode.** Gate 1 with a copy after block 6, followed at once by the adoption. Every block
   equals the move run, and A equals its uncopied twin.
4. **Realtime.** The swap block and the copy call make zero allocations and frees.
5. Commands:
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
  a new destination mixes the old ramp into the wrong bus. A re-pointed send that still ducks its
  source strip puts it in the restart set. Each turns it red.
- Gate 3: a copy-mode ring swap leaves the predecessor with the successor's at-rest line. Its twin
  turns red.

## Dependencies

- *Carry compensation lines across a plan swap* (#1283).
- None on #1225: the send retarget lives there (D8) and #1225 depends on this slice.
