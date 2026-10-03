# App handoff: live submix strips (#1214, batch K2)

Batch K2 (#1206 to #1214) lets live controls and meters reach every submix strip, not only the
tracks (owner decision 13, `docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`; design record
`docs/handoffs/submix-sends-2026-10-02/`). The session side is in `APP-SDK.md` in this folder.
This is an in-place V1 amendment: `ABI_VERSION` did not move, no record field moved, and a session
without submixes behaves exactly as before.

## Live submix strips

`edit.submix(id)` addresses a bus by its ID, beside `edit.track(id)`:

```ts
const controls = await engine.liveControls(); // headless: engine.liveControls()
await controls.submit(
  controls.edit.submix("drums").faderDb(-6),
  controls.edit.submix("verb").mute(true),
  controls.edit.submix("drums").insert("glue", "miso.compressor").parameter("threshold", -18),
);
```

- `submix(id)` returns `SubmixEdits`: every `TrackEdits` method -- `pan`, `matrix`, `faderDb`,
  `mute`, `trimDb`, `polarityInvert`, `hpfHz`, `lpfHz`, `inputFilters`, `effect`, `console`,
  `insert` -- with the same options, smoothing and refusals, **except `solo`**. Both classes share
  one implementation (`StripEdits`, exported as their common base).
- A bus has no solo. A bus is solo-safe: soloing a track keeps the buses it feeds audible. A raw
  command record (`LiveControlWriter`, or the worklet host's `command()`) that solos a submix index
  is refused whole with reason `notSoloable` (12) and result `invalidArgument`.
- An unknown ID throws `MisoUsageError` naming the known submixes. A track ID is not a submix ID
  and a submix ID is not a track ID: `edit.submix("kick")` and `edit.track("drums")` both throw.
- `console(slot, effectId)` and `insert(id, effectId)` resolve on a bus exactly as on a track, for
  an engine booted from an SDK builder (or after `withSession(builder)`): the console slot by its
  index in slot order, the insert in the bus's own chain.
- `SessionMap.submixes` (from `OfflineEngine.shape()`, and the browser host's
  `miso.sessionmap.v1` reply) lists the submix IDs in canonical (sorted) order, after
  `SessionMap.tracks`.
- On the wire a record's index word is a **strip index**: the tracks are `0..T`, and submix `j`
  (`SessionMap.submixes[j]`) is `T + j`, where `T = SessionMap.tracks.length`. The SDK computes it;
  an app that writes raw records must add `T` itself.

### Per-submix meters

- SDK: `MeterUpdate.submixes` is a `ReadonlyMap<submixId, TrackMeter>` (`peakLeft`, `peakRight`,
  `gainReductionDb`) in canonical order beside `MeterUpdate.tracks`. It is empty for a session
  without submixes.
- Worklet host: the `miso.meter.v1` frame carries `submixCount`, `submixPeaks`
  (`[bus0 L, bus0 R, ..]`, `2 * submixCount` long) and `submixGrDb` (`submixCount` long), in
  `SessionMap.submixes` order.
- An effect observation on a bus (`edit.submix(id).insert(..).observe(..)`) names the submix ID.
  `observationMap()` lists a bus's bindings with `trackId` set to the submix ID, and both
  `readObservations()` and the managed `subscribeObservations()` accept that selection: the
  managed owner arms and disarms through `edit.strip(id)`, which resolves a track or a submix ID.

## Live sends (#1223, batch K3)

A send -- a route into a submix -- is live in an engine booted with live controls
(`liveControls: { commandQueueRecords: .. }`). Address it by its route ID:

```ts
await controls.submit(
  controls.edit.route("kick-verb").gainDb(-12, { smoothingSamples: 480 }),
  controls.edit.route("snare-verb").mute(true),
  controls.edit.route("vox-delay").matrix({ ll: 1, lr: 0.25, rl: 0, rr: 0.8 }),
);
```

- `route(id)` returns `RouteEdits`: `gainDb(db, options?)`, `mute(on, options?)` (`true` silences
  the send) and `matrix({ ll, lr, rl, rr }, options?)`. `options` takes `smoothingSamples` only: a
  send has no lane, so an edit moves both of its lanes together. The engine holds the gain and the
  matrix to the session's route domain and refuses a value outside it with `domain`.
- `SessionMap.routes` (from `OfflineEngine.sessionMap()`, and the browser host's
  `miso.sessionmap.v1` reply) lists the live routes in canonical (sorted) route-ID order, as the
  engine enumerates them. The SDK takes a send's index from that list and never from the session.
- Only routes into submixes are live. A route into the output, and every route's `follows_mute`,
  stay structural: they change only through the session. `edit.route(id)` throws
  `MisoUsageError` (`diagnosticCode` `unknownRoute`) for any ID not in `SessionMap.routes`; for an
  engine booted from an SDK builder (or after `withSession(builder)`) the message says when the ID
  is an output route, and otherwise it lists the live routes.
- On the wire a send record is kind `routeGainDb` (13), `routeMute` (14) or `routeMatrix` (15).
  Its index word is the **live-route index**, the send's position in `SessionMap.routes`; `rack`
  and `channel` are `255`. An app that writes raw records takes the index from that list; an index
  past it is refused whole with reason `unknownRoute` (13).

## Master designation

`liveControls.masterTrackPlusOne` keeps its name and now takes a **strip index plus one** (tracks
first, then submixes), or `0` for none. A mix bus can therefore be the master: with tracks
`[kick, snare, vox]` and submixes `[drums, mix]`, `masterTrackPlusOne: 5` designates `mix`
(`3 + 1`, plus one), and `masterGrDb` reads that bus's gain reduction. A value past every strip is
refused at boot. A track master keeps its old value.

- A designation other than `0` is refused at boot unless observation taps are reserved
  (`liveControls.observationTaps > 0`), for a bus exactly as for a track.
- `masterGrDb` is `null` until the designated strip's effect has published an observation, so
  arm the master bus's limiter or compressor tap before reading it.

## Policy key

The app still passes live-control options under the older `console:` policy key, against an older
SDK pin. This SDK reads only `liveControls` (`createEngine({ policy: { liveControls: { .. } } })`
in the browser, `createOfflineEngine(doc, { liveControls: { .. } })` headless); `console:` was
retired without an alias (`docs/handoffs/console-strip-2026-09-29/APP-LIVE-CONTROLS.md`), so an
app that bumps to this SDK and keeps `console:` boots with **no** live controls attached, and
`liveControls()` then refuses. In TypeScript, an object literal that still carries `console:`
fails to compile first (an excess-property error on the policy type), which is the symptom the app
will see. Move the key with the bump:

```ts
createEngine({ document, policy: { liveControls: { commandQueueRecords: 64, meterBlocks: 12 } } });
```
