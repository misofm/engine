# Carry strip delay lines and live send ramps across a plan swap

Slice 14 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

A strip's input alignment delay (`delay_samples`, multi-mic alignment), a submix's input delay and a
live send whose level is ramping at the swap all keep their exact state through a plan swap. After
this slice every state family of a plan carries, so a structural edit that changes no node's latency
is bit-continuous for every unchanged path.

## Context

- A strip's `delay_samples` is a `TrackDelayLine` (`crates/graph/src/runtime.rs:1021`) driven by
  `NodeKind::TrackDelay` (track input) or `NodeKind::SumDelay` (submix input) (`:1106-1132`), held in
  `Runtime::track_delays` (`:2513`). It is not latency (`crates/graph-compiler/src/pdc.rs:1-24`), and
  a strip whose delay is 0 on both lanes has no line.
- A live send is `NodeKind::LiveRoute(Box<LiveRoute>)` (`runtime.rs:861-938`): an `IndexedRamp` and
  its position, settled at bind on the prepared coefficients. Its drain applies the records present
  at entry directly to the ramp.
- The compensation-line carry and its head-aligned copy come from *Carry compensation lines across a
  plan swap* (#1283).

## Decisions frozen for this slice

- **D1. Strip delay lines.** Key: strip ID and lane. Carry when the lane's `delay_samples` is equal in
  the committed model before the transaction and in the successor's model: equal lengths, so the two
  rings and the cursor move. A changed delay is a rebuild-only value (decision 14) and starts at rest.
- **D2. Live sends.** Key: route ID. Carry when the route's source, tap, destination, gain, matrix,
  mute and `follows_mute` are bit-equal in the two models. At the swap block, drain the predecessor's
  route lane (records present at entry), then copy the ramp and its position.

## Deliverables

1. D1-D2 in `crates/graph` and the inventory rows in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`

## Non-goals

- No change to delay or ramp kernels; no floors (slice 15).

## Objective gates

1. **Gap-free acceptance.** Session A: one track with `delay_samples` on both lanes (different
   values), two tracks routed into a submix with `delay_samples`, and a live send from a third track
   into that submix. Prepared with live controls, a send-level record with a multi-block ramp is
   admitted between blocks 5 and 6; B (A plus a muted track whose ID sorts first) is prepared from the
   committed model. Every block equals the reference fed the same record at the same block.
2. **Rule.** A send whose gain the transaction changed starts at its new prepared value; a strip whose
   `delay_samples` changed starts its line at rest.
3. **Realtime.** The swap block makes zero allocations and frees.
4. Commands:
   - `cargo test --locked -p graph -p host-core --features graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - the umbrella's inherited gates.

## Test value

- Gate 1: a strip or submix delay line left at rest drops the delayed strip for its delay length; a
  send ramp restarted from its target steps the level; a record lost because the copy ran before the
  drain; each turns it red.
- Gate 2: carrying a send whose gain the transaction changed keeps the old level; it turns red.

## Dependencies

- *Carry compensation lines across a plan swap* (#1283).
