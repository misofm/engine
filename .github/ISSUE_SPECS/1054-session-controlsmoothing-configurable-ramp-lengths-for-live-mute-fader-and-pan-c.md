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

## Coordination with #1053 (root, 2026-10-04)

#1053 is now an umbrella of slices that make C ABI edits live updates.

- **The C ABI seam.** The C ABI's live fader and mute records take their lengths from
  `host_core::LiveRamps::for_session(model)` (#1053 D3, added by #1255). It returns 0, a step,
  until this issue replaces its body with the session's `controlSmoothing`, through the same
  rounding rule. That one function is the C ABI half of outcome 3.
- **The dependency.** The C ABI half of this issue's parity gate needs the live path. It depends on
  *Apply value-only track fader, mute and pan transactions to the running C ABI plan* (#1257).
- **Pan and matrix on the C ABI.** The model's `smoothing_samples` is always present, so it always
  "carries" a value. Decide whether a model value of 0 means "use the session's `panMs`".
- **Input trim and polarity.** #1261 adds `LiveRamps::input_samples` (0 until a setting exists).
  This issue's table has no trim key; say whether trim and polarity follow `faderMs` or get a key
  of their own.
- **`controlSmoothing` edits on the C ABI.** A change to `controlSmoothing` alone changes no prepared
  plan state; it only changes the length of later live records. Add it to #1053's model-only mask
  (D12, *Commit model-only C ABI transactions without a plan rebuild*, #1260), so such an edit
  commits without a rebuild.
- **A fader move on a lane that stays muted.** Once `faderMs` is non-zero, such a record retargets
  0 to 0 over the ramp, so the lane runs the ramp kernel for that window and turns a negative input
  into `-0.0` where a rebuild gives `+0.0` (the same mechanism as `crates/host-core/src/solo.rs:58-70`);
  after the window the bits match again. Keep the ramp anyway: a step there would cut short a mute
  ramp that is still in flight, which is an audible click. Say so in the PR, and compare after the
  ramp in any bit-identity gate.
