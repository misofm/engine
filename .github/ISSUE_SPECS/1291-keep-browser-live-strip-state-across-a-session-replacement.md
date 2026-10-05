# Keep browser live strip state across a session replacement

Slice B3 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

A producer who has moved faders, muted, soloed, changed a pan, an input filter, a send or a VCA live,
and then adds a track in the browser, keeps hearing exactly that mix: those strips carry like any
other, solo stays on, and no live record admitted before the replacement is lost. The replacement
document stays authoritative for every value it actually changes. (Live effect edits are *Keep
browser live effect edits across a session replacement*, B3b.)

## Context

- The browser has no committed model (#1057). Live records change lanes, not the booted document,
  and the SDK holds the live controls to the booted document byte for byte (`assertBootedSession`,
  `sdk/src/core/live-controls.ts:266-290`). So a replacement an app builds from its session builder
  carries the **booted** values of everything it did not edit.
- `ReadyOwnership` (`hosts/host-web/src/lib.rs:1509`) mirrors some live values: `solo`
  (`LiveControlSoloState`, `crates/host-core/src/solo.rs:129-136`, which keeps `user_mute`, `solo`
  and `vca_mute` apart), `routes` (`LiveRouteState`), `vcas` (`LiveVcaState`) and the input-filter
  shadows. Prepared mutes come only from the document (`StripMuteSeed`, `lib.rs:6816-6850`).
- Records admitted since the last render (`in_flight`, `has_in_flight_commands`, `lib.rs:1554-1559`)
  sit in the old plan's queues at a replacement; builtin and send carries drain them (slices 7, 8,
  14).
- *Replace the running browser session in the Rust host* (#1290) compares against the booted model
  and does not carry live-touched owners (its D4).

## Decisions frozen for this slice

- **D1. Effective model.** The host keeps the predecessor's effective model: the booted (or last
  replaced) `SessionModel` with every admitted live strip record applied at admission: fader, user
  mute, pan or matrix, input trim, polarity and filters, send gain, mute and matrix, VCA offsets and
  mutes. It is updated on the control path only, and its storage is sized at preparation.
- **D2. Three-way merge.** The successor is prepared from a merged model: for every strip value,
  where the new document's value equals the old document's, use the effective value; otherwise use
  the new document's. `SuccessorBase.committed` is the effective model (umbrella P1.4). B2's D4 rule
  is deleted for strip state.
- **D3. Solo and VCA composition.** Host-core preparation gains an input of per-strip solo and VCA
  mute terms, composed into the prepared fader-mute words exactly as admission composes them. The new
  `ReadyOwnership`'s solo and VCA state is seeded from the old one by strip ID (a removed strip drops
  out), and the same terms go to preparation, so a soloed mix stays soloed from the first block.
- **D4. Shadows.** Input-filter shadows and route state of persisting strips and routes are seeded
  from the merged model.

## Deliverables

1. D1-D4 in `hosts/host-web/src/lib.rs`, with the preparation input in `crates/host-core`.
2. Native tests in `hosts/host-web/src/tests.rs`.

## Authorized paths

- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`
- `crates/host-core/src/prepare.rs`, `solo.rs`, `vca.rs`, `live_route_state.rs` (seeding and the
  preparation input)

## Non-goals

- No effect parameters or bypass (B3b). No Wasm export or JavaScript (B4-B8).

## Objective gates

1. **Live strip edits carry.** Live controls on: boot A, render 4 blocks, admit a fader record with a
   multi-block ramp, a pan record and a high-pass record on one track; render 1 block; replace with
   the **booted** document plus a muted track (what an app's builder writes). Every block equals the
   reference: A plus the muted track, fed the same records at the same blocks.
2. **Pending record not lost.** As gate 1, with the fader record admitted after the last render
   before the replacement: still bit-identical.
3. **Document wins.** A replacement that sets a live-edited fader to a new value (different from the
   old document's) starts that stage at rest at the new value; other strips stay bit-identical.
4. **Solo survives.** With one track soloed live, after the replacement only that track is audible
   from the first block, and releasing the solo live restores the others.
5. Commands:
   - `cargo test --locked -p host-web --features host-web/test-support` and
     `cargo test --locked -p host-core --features host-core/test-support`
   - the artifact build and `scripts/check-web-audioworklet.sh` as in B2
   - the umbrella's inherited gates.

## Test value

- Gate 1: preparing from the document as sent reverts every live-moved value; comparing with the
  booted model restarts every touched owner; either turns it red.
- Gate 2: records dropped with the old plan's queues turn it red.
- Gate 4: prepared mutes taken only from the document un-solo the mix for a block or more; it turns
  red.

## Dependencies

- *Replace the running browser session in the Rust host* (#1290).
