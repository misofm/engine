# App handoff: submix strips and bus taps in the SDK (#1205)

Batch K1 (#1199 to #1205) gives every submix a full strip and lets a route or a routed sidechain
read any of a submix's seven taps (owner decision 13,
`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`; design record
`docs/handoffs/submix-sends-2026-10-02/`). The engine, its wire records and the SDK change together. This is an in-place V1 amendment: `ABI_VERSION` did not move, and the retired
`submix_output` source is refused, never reinterpreted. Live controls and meters for submixes come
in batch K2 (`APP-LIVE.md` in this folder, when they land).

At app `7effbd6` no app code authors a submix or a `submix_output` source: the session document
writes `submixes: []`. Nothing in the app has to change for K1 unless it loads saved documents
that carry a bus, or starts authoring buses.

## The new session shape

```json
"submixes": [
  { "id": "drums",
    "builtins": { "left": { ... }, "right": { ... } },
    "console": [ { "slot": "eq", "bypass": false, "params": [ ... ] },
                 { "slot": "compressor", "bypass": true, "params": [] } ],
    "inserts": { "effects": [ ... ] },
    "fader": { ... },
    "matrix": { "ll": 1.0, "lr": 0.0, "rl": 0.0, "rr": 1.0, "smoothing_samples": 0 } }
],
"routes": [
  { "id": "drums-main",
    "source": { "kind": "submix", "submix_id": "drums", "tap": "post_fader" },
    "destination": { "kind": "output_input", "output_id": "main-out" }, ... }
]
```

- A submix is a track's strip without the source fields: `id`, `builtins`, `console`, `inserts`,
  `fader`, then `pan` or `matrix`, every key required, each with the track's grammar and codes.
- Every submix carries every console slot, as every track does: one entry per slot, in slot order.
- A route or sidechain source is `{ kind: "track", track_id, tap }` or
  `{ kind: "submix", submix_id, tap }`, at any of the seven taps (`input`, `post_input`,
  `insert_send`, `insert_return`, `pre_fader`, `post_fader`, `post_pan`). A pre-fader tap is not
  gated by the fader mute.
- A submix runs its strip on the sum of the routes that target it, summed in route-ID order.

## SDK names, old to new

| Old | New |
|---|---|
| `SessionBuilder.submix(id)`: a bare `{ id }` | `submix(id, spec?: SubmixSpec)`. With no spec it is the **transparent strip**: identity input section, every console entry `bypass: true` with no params (one per declared slot, in slot order), no inserts, a 0 dB unmuted fader and the identity matrix with no smoothing. With a spec, each field follows the track's rules and defaults, and its console entries are checked against `.console()` exactly as a track's are, so `.submix(id, spec)` must follow `.console()` |
| (none) | `SubmixSpec` = `TrackSpec` without `source` (`builtins`, `console`, `inserts`, `fader`, `pan`) |
| `RouteSource` `{ kind: "submix_output", submixId }` | `{ kind: "submix", submixId, tap }`; `tap: "post_pan"` is the old submix output. The old variant is gone from the type and refused at runtime (`MisoUsageError`, `diagnosticCode` `schema.invalid_enum`, naming the replacement), as a route source and as a sidechain source |
| `SendTap` documented as the seven *track* taps | the same seven, on every strip, track or submix |
| `SessionModel.submixes[]` `{ id }` | `{ id, builtins, console, inserts, fader, pan \| matrix }` in that canonical order |
| `.console()` refused only after the first track | also after the first submix declared with a spec (a spec-less `.submix(id)` may precede it) |
| `enginectl session build` request `submixes: ["bus"]` | a bare string is still accepted (the transparent strip); an object `{ id, builtins?, console?, inserts?, fader?, pan? }` is a submix strip. Route and sidechain sources take `{ kind: "submix", submixId, tap }`; `submix_output` is refused with `request.shape` and the diagnostic `schema.invalid_enum` |

## What a bare bus sounds like now

`submix(id)` keeps the sound of a bare bus. Its only changes are the ones every strip makes: the
identity input section turns a `-0.0` sample into `+0.0` (and sanitizes a non-finite one), and the
bus pays the latency of every latent console slot even with its entries bypassed (decision 12,
L4), which plugin-delay compensation then aligns. A session whose console has no latent slot
renders a track through a transparent bus bit-identically to the same track routed straight to the
output (`console-evals.mjs`, #1205 gate 3).

## The bus-compressor hazard

A console slot's `link_mode` is session-level, and every bus carries every console slot. The app's
console compressor (`compressor`, declared `dual_mono` for the tracks) therefore runs **unlinked**
on any stereo bus, and moves the image under asymmetric material. On a bus:

- bypass the console compressor's entry (`{ slot: "compressor", bypass: true }`);
- for bus glue, put a linked compressor in the bus's own inserts:
  `inserts: [effect("miso.compressor", { ... }, { linkMode: "maximum" })]` (or `"average"`).

## Latency and bus depth

Every strip pays every latent console slot, bypassed or not, and each level of bus nesting adds it
again. With a console `miso.true-peak-limiter`, that is 486 samples per level at 48 kHz. Keep latent
effects off the console when buses nest.

## A return

A return is a submix with an effect insert. A reverb or delay send routes a track's `post_fader`
(or `pre_fader`) tap to the return, whose inserts hold the effect, and the return goes to the output
at `post_pan`:

```ts
builder
  .submix("verb", {
    console: [{ slot: "eq", bypass: true }, { slot: "compressor", bypass: true }],
    inserts: [effect("miso.delay", { "delay time": 80 }, { slotId: "space" })],
  })
  .route({ id: "vox-verb", source: { kind: "track", trackId: "vox", tap: "post_fader" },
           destination: { kind: "submix_input", submixId: "verb" }, gainDb: -6 })
  .route({ id: "verb-main", source: { kind: "submix", submixId: "verb", tap: "post_pan" },
           destination: { kind: "output_input", outputId: "main-out" } });
```

Routes form an acyclic graph; a cycle through buses is refused when the graph compiles at boot.

## Saved documents

A document saved before K1 is refused twice: a bare `{ "id": .. }` submix (`schema.missing_field`)
and a `submix_output` source (`schema.invalid_enum`). `migrate-submix-strips.py` beside this note
rewrites both mechanically: every bare submix gets the transparent strip (one bypassed entry per
declared console slot), and every `submix_output` source, on a route or a routed sidechain, becomes
`{ "kind": "submix", "submix_id": .., "tap": "post_pan" }`, which is the same point. Key order,
layout and number spellings are preserved elsewhere, so a canonical document stays canonical.

```sh
python3 docs/handoffs/submix-strips-and-sends/migrate-submix-strips.py --check saved.json
python3 docs/handoffs/submix-strips-and-sends/migrate-submix-strips.py saved.json
```

Then validate the result with `session_validator validate --canonical`. Run on the pre-K1
`author-session` worked session, the script's output is byte-identical to the migrated checked-in
document (#1205 gate 7).
