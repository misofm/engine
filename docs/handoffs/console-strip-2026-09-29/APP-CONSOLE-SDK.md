# App handoff: the session console and inserts in the SDK (#1097, S1d)

Batch C3 (S1r, S1a, S1b, S1c, S1d) replaces every track's `simd1`, `dynamic` and `simd2` racks with
a session-level console strip and per-track inserts (owner decision 12,
`docs/rulings/engine-footprint-2026-09-29.md`). The engine, its wire records and the SDK change
together. This is an in-place V1 amendment: `ABI_VERSION` did not move, every retired token and
code is refused (never reinterpreted), and the app updates in lockstep. S1r's
`APP-LIVE-CONTROLS.md` covers the earlier rename of the live console to "live controls"; this note
covers the console strip itself.

## The new session shape

```json
"console": {
  "pre_insert":  [ { "slot": "eq", "identity": { "kind": "native", "effect_id": "miso.parametric-eq" },
                     "quality": "normal", "link_mode": "dual_mono" },
                   { "slot": "compressor", "identity": { "kind": "native", "effect_id": "miso.compressor" },
                     "quality": "normal", "link_mode": "dual_mono" } ],
  "post_insert": []
},
"tracks": [
  { "id": "track-000", ..., "builtins": { ... },
    "console": [ { "slot": "eq",         "bypass": false, "params": [ ... ] },
                 { "slot": "compressor", "bypass": false, "params": [ ... ] } ],
    "inserts": { "effects": [] },
    "fader": { ... }, "pan": { ... } }
]
```

- The root `console` sits between `sources` and `tracks`. Each slot is declared once, with its
  effect, quality and link mode; slot IDs are unique across both sections.
- Every track carries exactly one entry per slot, in slot order (`pre_insert`, then
  `post_insert`), with only its own `bypass` and `params`. A track cannot add, drop or reorder a
  slot.
- `inserts` is the old `dynamic` rack under a new name, between the two console sections.
- A console slot is one of `miso.parametric-eq`, `miso.compressor`, `miso.gate-expander`,
  `miso.soft-clip`, `miso.transient-shaper`, `miso.true-peak-limiter`, with no sidechain. The delay,
  the multiband compressor and any keyed effect are inserts.
- Chain: input section -> `pre_insert` -> inserts -> `post_insert` -> fader/mute -> pan -> routes.

## The app's EQ -> compressor

Today (`src/lib/mixer/engine/session-document.ts`) every track carries the pair in its `dynamic`
rack, with `slotId` `eq` and `compressor`, and a track the user has not selected has both marked
`bypass`. Under the console the pair becomes two `pre_insert` slots, and each track's entry keeps
its bypass (decision 12, "The app"):

```ts
let builder = session({ id, sampleRateHz, revision })
  /* .source(...) for each stem */
  .console({
    preInsert: [
      { slot: "eq", effectId: ENGINE_EQ_EFFECT_ID },
      { slot: "compressor", effectId: ENGINE_COMPRESSOR_EFFECT_ID },
    ],
  });
for (const track of tracks) {
  builder = builder.track(track.trackId, {
    source: { id: track.sourceId, left: 0, right: track.channels > 1 ? 1 : 0 },
    builtins: { left: strip, right: strip },
    console: [
      { slot: "eq", bypass: effectiveBypass(mix, state, track.stem, MIXER_EQ_SLOT),
        parameters: { ...bandSetupParameters(), ...slotParameters(eqValues, MIXER_EQ_SLOT) } },
      { slot: "compressor", bypass: effectiveBypass(mix, state, track.stem, MIXER_COMPRESSOR_SLOT),
        parameters: slotParameters(compressorValues, MIXER_COMPRESSOR_SLOT) },
    ],
    inserts: [],
    fader: { ... },
    pan: { ... },
  });
}
// routes: tap "post_pan" (was "post_matrix")
```

- `.console()` must come before the first `.track()`; each track's entries are checked against it.
- The bypass is per track and per slot, exactly as before; a bypassed track now stays in its
  effect bank (P1) instead of splitting it.
- What it renders: S1a's class-A evidence rendered B0's app-shape row (EQ -> compressor on all 64
  tracks, both bypassed on tracks 2 mod 3) before and after this move with identical PCM. Only the
  compiled plan changed: the pair now binds as full console banks.
- The live address of the pair changes only in its rack: `eq` is console slot 0 and `compressor`
  console slot 1 (their indices in the old `dynamic` rack were also 0 and 1). So
  `ENGINE_DYNAMIC_RACK = 1` (`key-map.ts`) becomes the console code `3` for these two effects;
  better, read it from `ABI_LAYOUT.constants.racks` (`console`), as the SDK does.
- An authoritative document the app imports (`authoritative-session.ts`) reads `console` and
  `inserts` instead of `simd1`/`dynamic`/`simd2`. Its `insertedSlots` projection's `rack:
  "dynamic"` becomes `rack: "console"` for the pair, and `emptyRack(track, ...)` checks become "the
  track has no inserts" plus "the console declares exactly the pair".
- A stored automation target on the pair is `rack: "console"`, `effect_id: "eq"` or
  `"compressor"` (was `rack: "dynamic"`).

## SDK names, old to new

| Old | New |
|---|---|
| `Rack` = `"simd1" \| "dynamic" \| "simd2"` | `Rack` = `"console" \| "inserts"` |
| `AutomationRack` = `Rack \| "builtins"` (with the three retired tokens) | `"console" \| "inserts" \| "builtins"` |
| `TrackSpec.simd1`, `.dynamic`, `.simd2` (`EffectDecl[]`) | `TrackSpec.console` (`ConsoleEntrySpec[]`, one per slot in slot order) and `TrackSpec.inserts` (`EffectDecl[]`); the old keys refuse with `schema.unknown_field` |
| (none) | `SessionBuilder.console({ preInsert, postInsert })`, `ConsoleSpec`, `ConsoleSlotSpec` (`slot`, `effectId`, `quality?`, `linkMode?`), `ConsoleEntrySpec` (`slot`, `bypass?`, `parameters?`, `channel?`), `ConsoleEffectId`, `CONSOLE_ELIGIBLE_EFFECTS` |
| Default effect IDs `simd1-1`, `dynamic-1`, `simd2-1`, ... | Default insert IDs `insert-1`, `insert-2`, ...; console slots always name their `slot` |
| `SessionModel` (no `console`); track model `simd1`/`dynamic`/`simd2` | `SessionModel.console` (`pre_insert`, `post_insert`); track model `console` and `inserts` |
| `SendTap`: `input`, `post_input_builtins`, `post_simd1`, `post_dynamic`, `post_simd2_pre_fader`, `post_fader`, `post_matrix` | `input`, `post_input`, `insert_send`, `insert_return`, `pre_fader`, `post_fader`, `post_pan` (codes 1-7 unchanged) |
| `AutomationTarget { rack: "simd1", slotId }` | `{ rack: "console", slotId: <slot> }` or `{ rack: "inserts", slotId: <insert id> }` |
| `MisoUsageError` (message only) | `MisoUsageError.diagnosticCode`: the engine's code when the builder refuses what the engine would (`console.entry_missing`, `console.entry_order`, `id.duplicate`, `reference.missing_entity`, `schema.unknown_field`, `schema.invalid_enum`, `schema.wrong_type`, `id.invalid`, `console.slot.ineligible_effect`, `effect.quality.unsupported`, `effect.link_mode.unsupported`) |
| `LiveControlRack` = `"simd1" \| "dynamic" \| "simd2"` | `"console" \| "inserts"` |
| `TrackEdits.effect("simd1" \| "dynamic" \| "simd2", index, effectId)` | `TrackEdits.effect("console" \| "inserts", index, effectId)`: a console slot by its index in `pre_insert`-then-`post_insert` order, an insert by its chain index |
| (none) | `TrackEdits.console(slot, effectId)` (by slot ID) and `TrackEdits.insert(idOrIndex, effectId)` |
| (none) | `EngineLiveControls.withSession(builder)`; the constructor's optional fourth argument and `LiveControlEdits`' optional second argument take the session, and the constructor's optional fifth argument (`createBrowserLiveControls`' fourth) the booted document. IDs resolve automatically when the engine booted from a `session(...)` builder (`createOfflineEngine(builder)`, `createEngine({ document: builder })`); for document text, call `withSession(builder)`. `withSession()` requires the builder's `toJson()` to equal the booted document byte for byte, and throws a `MisoUsageError` for any other session (a reordered console, a renamed slot, another insert chain) or when the live controls were constructed without the booted document |
| (none) | `EffectEdits.bypass(false)` throws a `MisoUsageError` ("keeps its prepared bypass") for a session-bypassed delay or multiband whenever the SDK has the session; `PREPARED_BYPASS_EFFECTS` lists the two. The `EffectEdits` constructor's optional fifth argument (`AuthoredInstance`) carries what the session authored |
| (none) | `EFFECT_LINK_MODES`: the link modes each effect supports. The builder refuses any other on a slot or an insert with `effect.link_mode.unsupported`: the EQ, soft-clip and delay are `dual_mono` only, and the limiter has no `average` |
| `LaneEdit.rack` `0` simd1, `1` dynamic, `2` simd2 | `1` inserts, `3` console, `255` not applicable |
| `ObservationRack` = `"simd1" \| "dynamic" \| "simd2"` | `"console" \| "inserts"` (selections, bindings and read results) |
| `TrackResponseMember.rack` `"input" \| "simd1" \| "dynamic" \| "simd2"`, `rackValue` 0-3 | `"input" \| "console" \| "inserts"`, `rackValue` `0` input, `4` console, `2` inserts |
| `ResponseTargetCorrelation.rack` `"simd1" \| "dynamic" \| "simd2"` | `"console" \| "inserts"` |
| `SpectrumTarget.kind` `"trackPostInputBuiltins"`, `"trackPostMatrix"` | `"trackPostInput"`, `"trackPostPan"` (codes 1 and 2 unchanged) |
| `enginectl session build` request track `spec.simd1`/`dynamic`/`simd2` | root `console: { preInsert, postInsert }` and track `spec.console`/`spec.inserts`; the old keys are refused by name, and a builder refusal's engine code is in the stderr document's `diagnostics` |

## Raw worklet host codes

For code that addresses the worklet host's records by number: the app's `ENGINE_DYNAMIC_RACK`
(`key-map.ts`) is compared with observation bindings in `observations.ts` and `intent.ts` and
written into `EngineTrackEffect.rack` in `session-document.ts`. At app `0757a84` two more places
carry the old numbering:

- `src/lib/mixer/engine/index.ts` compares effects and targets with `ENGINE_DYNAMIC_RACK` five
  times (`:311`, `:867`, `:876`, `:985`, `:1062`). For the EQ -> compressor pair each becomes the
  console code `3`.
- `authoritative-session.ts`'s `projectedEffects` (`:1120-1124`) walks a hard-coded
  `simd1 0 / dynamic 1 / simd2 2` table over `track[name].effects`. It becomes two walks: the
  session's console slots (`console.pre_insert` then `console.post_insert`) at code `3`, whose
  effect ID is the slot's `identity.effect_id` and whose index is the slot's position in that
  order, and the track's `inserts.effects` at code `1`, indexed by chain position. Its
  `${track.id}/${name}/${effectIndex}` address keys move with it.

The layout JSON's `racks` and `liveResponseRacks` tables are the machine-readable source (S1c).
The host-web qualification fixture `console-session.json` is now `live-control-session.json`; the
app does not reference it.

| Record | Old | New |
|---|---|---|
| `MisoCommand.rack` (48-byte record), observation selections, map bindings and rows | `0` simd1, `1` dynamic, `2` simd2 | `1` inserts, `3` console; `255` still not applicable. `0` and `2` are refused: the shipped host's `command()`, `observe()` and observation reads reject them locally with a `miso.error.v1` invalid argument before the port, and a raw-export caller gets `unknownRack` / `RESULT_INVALID_ARGUMENT` |
| `effectIndex` with an effect rack | index within the rack | console: index in `pre_insert`-then-`post_insert` order; inserts: index in the track's chain |
| Live-response owner `rack` | `0` input filters, `1` simd1, `2` dynamic, `3` simd2 | `0` input filters, `2` inserts, `4` console (either section); `1` and `3` refused |
| Spectrum targets | `trackPostInputBuiltins` (1), `trackPostMatrix` (2), `output` (3) | `trackPostInput` (1), `trackPostPan` (2), `output` (3) |

Live bypass: an EQ, compressor, gate, soft-clip, transient shaper or limiter bypass is a per-lane
shunt, so a live toggle is exact and keeps the track in its bank. A session-bypassed delay or
multiband keeps its prepared bypass: a live un-bypass of either is admitted and changes nothing
(S1c). Through the SDK, with the session known, that lift throws a `MisoUsageError` before it is
sent; a raw record, or an SDK edit addressed by index with no session, is still acknowledged and
changes nothing.
