# Hold route-lane values in latest-target cells

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A live route's (a send's) gain, mute and matrix target is held in one latest-target cell instead
of a bounded queue, through the shared graph lane on both hosts. Any number of send edits between
two render calls is accepted; render ramps from where the route is to the last committed target at
the next block, and #1312's counter records every replaced target. The C ABI's send edits
(#1225, #1226, #1247) are built on these cells.

## Context

- **Record.** `RouteControlRecord { target: [f32; 4], mute: bool, length: u32 }`, validated at
  construction (`crates/graph/src/lib.rs:929-966`; `RouteControlRecord::new` refuses a ramp above
  `ROUTE_RAMP_LENGTH_MAXIMUM` and a muted route with a non-zero target).
- **Ring and producer.** `attach_route_controls(depth)` (`lib.rs:1697`) creates one bounded SPSC
  per live route (`:1756`); `RouteControlLane` holds the consumer (`:969-975`),
  `GraphRouteControlProducer` the producer, with `free()` and `try_push` (`:986-1019`);
  `route_control_resources` charges the rings (`:1074-1090`).
- **Render.** The route op's `drain` (`crates/graph/src/runtime.rs:895-913`) applies, in order,
  every record available at block entry, each ramping from the coefficients the route is at.
- **Browser admission.** The route band (`queue_available`, `hosts/host-web/src/lib.rs:1783-1786`;
  `push`, `:1821-1827`). `free()` is `#[inline(always)]` for the worklet call-graph gate
  (`lib.rs:1003-1011`).
- **C ABI.** No route lane today (`crates/capi/src/runtime/compile.rs:15-21`); #1225 attaches
  them, written to these cells.

## Decisions frozen for this slice

- **D1. One cell per live route**, on #1312's primitive: six words (four target coefficient bits,
  mute, ramp length), one dirty bit per route. The ring, the `depth` argument of
  `attach_route_controls` and `free()` go; the cells' rows replace the ring rows in
  `route_control_resources`. The route-activity table rows are unchanged.
- **D2. Producer.** `GraphRouteControlProducer::write(record)` is infallible; validation stays in
  `RouteControlRecord::new`, before any write.
- **D3. Render.** At block entry the route op applies its cell if dirty: one retarget from the
  coefficients it is at to the cell's target over its ramp, exactly as one drained record does
  today. A torn read leaves the bit set for the next block (#1312 D1).
- **D4. Browser admission.** The route band leaves the room pass and `in_flight`; a send edit is
  validated (record construction, follow-mute composition) for the whole batch before the first
  write.
- **D5. The acked-batch question: can an ack ever precede a drop? No.** Every check precedes the
  first write, writes cannot fail, a replaced target is in the committed model and counted, and
  the route ramps to the newest.

## Deliverables

1. Route cells in `graph` (D1-D3), its tests and resource rows.
2. Browser admission (D4) and the producer type #1225 writes to.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs` (the route lane, producer, drain and
  resources only): stream A's crate, sequenced by the coordinator.
- `crates/host-core/src/prepare.rs` (the attach call only; stream A's file).
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs` (route band; stream H's files).

## Non-goals

- The C ABI send edits (#1225, #1226, #1247). `follows_mute` in the browser (#1342).

## Hazards

- Two records in one drain used to ramp to the first target and then from wherever that ramp
  stood; now one ramp goes to the last target. Re-pin only fixtures that send two route edits in
  one block, each with that reason.
- The callgraph gate reads function names (`lib.rs:1003-1011`): keep the write path free of a
  function named like an allocator.

## Objective gates

1. **Many edits, one block (new host-web test).** 30 send-gain edits on one route, each
   admitted; one render equals a twin that sent only the last; the counter grows by 29.
2. **Ramp from where the route is (new graph test).** A ramp in flight, then a new target in the
   cell: the next block starts from the in-flight coefficients, bit-identical to today's one-record
   drain.
3. **Mute rule (keep green):** a muted target with a non-zero coefficient is refused by
   `RouteControlRecord::new` before any write; the route's existing mute tests pass.
4. **Superseded tests.** Graph or browser tests that fill a route queue and expect a full queue
   (`RouteQueueFull`, `COMMAND_REASON_BACKPRESSURE` on a send kind) are deleted with
   `RouteQueueFull`; gate 1 replaces them.
5. **Realtime and workspace.** `cargo build --locked --release -p audit -p capi &&
   target/release/audit capi` (all violation counts 0); `bash scripts/check-realtime-policy.sh`;
   `bash scripts/check-graph-policy.sh`; `bash scripts/check-web-audioworklet.sh`;
   `cargo test --locked -p graph --features test-support`; `cargo test --locked -p host-web
   --features test-support`; `cargo test --locked -p host-core --features
   control-provider,test-support`; `cargo fmt --all -- --check`; `cargo clippy --locked
   --workspace --all-targets --all-features -- -D warnings`.

## Test value

- Gate 1: a route lane left a queue, or a miscounted supersession.
- Gate 2: a cell drain that ramps from the target or from rest instead of from the route's
  current coefficients, which clicks.

## Dependencies

- *Hold live values in latest-target cells on both hosts* (#1312).
- Followed by *Deliver value-only send edits to the running C ABI plan* (#1225),
  *Let C ABI sends follow their source strip's mute live* (#1226) and *Deliver value-only VCA edits
  to the running C ABI plan* (#1247).
