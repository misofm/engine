# Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes

Owner ruling (2026-09-28, `docs/rulings/engine-footprint-2026-09-28.md`): ramp lengths for live control are **not hardcoded**; they are optional session settings, so the producer's session carries them to every fan's device and browser and phone render identical bits.

## Outcome

1. **Schema.** An optional session-level object, for example `"controlSmoothing": { "muteMs": …, "faderMs": …, "panMs": … }`, in the canonical JSON session (`crates/session`, `docs/SESSION_SCHEMA_V1.md`), with strict validation (finite, non-negative, an upper bound), canonical serialization and snapshot round-trip. If omitted, **one documented default table** in the session schema applies; the values in that table are set by the research issue filed alongside this one (placeholders until then, recorded as such).
2. **Conversion.** Milliseconds convert to samples at the session's sample rate with one fixed, documented rounding rule, so every host computes the same sample count.
3. **Engine.** A live fader, mute/solo or pan change that does not carry its own smoothing uses the session's value (or the default). A change that carries an explicit smoothing value keeps it (per-change override). This is core behaviour in the portable crates, used identically by the browser adapter and the C ABI adapter.
4. **Browser SDK.** `sdk/src/core/live-controls.ts` stops defaulting `smoothingSamples` to 0: when the app passes nothing, the engine's session value applies.
5. **Editing.** Changing `controlSmoothing` in a session is an ordinary session transaction.

## Objective gates

- Schema tests: accept, reject (negative, NaN, over the bound), omit, canonical round-trip.
- The same session renders bit-identical output through the browser adapter (V8, shipped artifact) and the C ABI adapter for a scripted sequence of mute, fader and pan changes with no per-change smoothing, at all four launch sample rates.
- A per-change smoothing value still overrides the session value.
- Every existing console digest is unchanged for sessions that omit the field *and* whose commands carry explicit smoothing; any digest that changes because the default is no longer 0 is listed and re-pinned with the reason.
- Render stays allocation-free.

## Defaults from #1055 (root, 2026-09-28)

The research (`docs/handoffs/control-smoothing-defaults/FINDINGS.md`) recommends the default table:
`muteMs` 10 ms (provisional until the owner's listening test; 5 ms if the preregistered rule says
so), `faderMs` 20 ms, `panMs` 20 ms. Rounding: `floor(ms * rate / 1000 + 1/2)` in `f64` from the
`f32` value. Bounds: finite, `0 <= ms <= 1000`, zero legal. The listening packet is in
`docs/handoffs/control-smoothing-defaults/listening/`.
