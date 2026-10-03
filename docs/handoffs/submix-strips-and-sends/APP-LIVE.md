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

## Master designation

`liveControls.masterTrackPlusOne` keeps its name and now takes a **strip index plus one** (tracks
first, then submixes), or `0` for none. A mix bus can therefore be the master: with tracks
`[kick, snare, vox]` and submixes `[drums, mix]`, `masterTrackPlusOne: 5` designates `mix`
(`3 + 1`, plus one), and `masterGrDb` reads that bus's gain reduction. A value past every strip is
refused at boot. A track master keeps its old value.

## Policy key

The app still passes live-control options under the older `console:` policy key, against an older
SDK pin. This SDK reads only `liveControls` (`createEngine({ policy: { liveControls: { .. } } })`
in the browser, `createOfflineEngine(doc, { liveControls: { .. } })` headless); `console:` was
retired without an alias (`docs/handoffs/console-strip-2026-09-29/APP-LIVE-CONTROLS.md`), so an
app that bumps to this SDK and keeps `console:` boots with **no** live controls attached, and
`liveControls()` then refuses. Move the key with the bump:

```ts
createEngine({ document, policy: { liveControls: { commandQueueRecords: 64, meterBlocks: 12 } } });
```
