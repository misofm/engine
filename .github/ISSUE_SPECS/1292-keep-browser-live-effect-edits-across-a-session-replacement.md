# Keep browser live effect edits across a session replacement

Slice B3b of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

A producer who has tweaked an EQ, a compressor or an effect bypass live, and then adds a track in the
browser, keeps hearing those settings and the effects' state: live-edited effects carry, pending
effect records render, and the replacement document wins only where it changes a value.

## Context

- Browser live effect edits are parameter records, EQ prepared targets (the prepared companion path,
  `submit_prepared_commands`, `hosts/host-web/src/lib.rs:3019`) and bypass records, admitted on the
  audio thread with exact free-slot counts (`in_flight`, `:1554-1559`). The host keeps no mirror of
  live effect values today.
- *Keep browser live strip state across a session replacement* (#1291) adds the effective model and
  the three-way merge for strip values.
- *Carry live-controlled effect lanes across a plan swap* (slice 11) carries effect lanes with an
  inherited queue and requires a host, while a successor is pending, to admit to a successor lane
  only the room the predecessor lane leaves (its D3). In the browser the replacement is synchronous,
  and every render drains every queue, so the inherited records are exactly those admitted since the
  last render; records admitted after the replacement and before the next render land in the
  successor's own queue, and both reach the successor's first window.

## Decisions frozen for this slice

- **D1. Effective effect values.** The effective model (B3 D1) also records every admitted effect
  parameter value, EQ target and bypass, in tables sized at preparation, updated on the control path.
- **D2. Merge.** B3's three-way merge covers effect values too.
- **D3. Window.** At a replacement, each carried lane's `in_flight` count moves to its successor
  lane, so until the successor's first render, admission to that lane accepts only the room the
  inherited records leave and refuses the rest with typed backpressure, as it does today for a full
  queue.

## Deliverables

1. D1-D3 in `hosts/host-web/src/lib.rs`.
2. Native tests in `hosts/host-web/src/tests.rs`.

## Authorized paths

- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`

## Non-goals

- No Wasm export or JavaScript (B4-B8).

## Objective gates

1. **Live effect edits carry.** Live controls on: an EQ gain prepared target, a compressor threshold
   record and a live bypass of an insert compressor, then a replacement with the booted document plus
   a muted track: every block equals the reference fed the same records at the same blocks.
2. **Pending record.** The compressor record admitted after the last render before the replacement:
   still bit-identical.
3. **Window.** Admit records to a lane up to capacity minus 2, replace, then admit 3 more to the same
   lane before rendering: 2 are accepted, the third is refused with backpressure, and the successor's
   first block drops nothing and raises no `target_error`.
4. Commands:
   - `cargo test --locked -p host-web --features host-web/test-support`
   - the artifact build and `scripts/check-web-audioworklet.sh` as in B2
   - the umbrella's inherited gates.

## Test value

- Gate 1: an effective model without effect values restarts every live-tweaked effect; it turns red.
- Gate 3: `in_flight` counts reset at the replacement let admission overfill the inherited window,
  dropping an acked span or failing the block; it turns red.

## Dependencies

- *Keep browser live strip state across a session replacement* (#1291).
- *Carry live-controlled effect lanes across a plan swap* (#1280).
