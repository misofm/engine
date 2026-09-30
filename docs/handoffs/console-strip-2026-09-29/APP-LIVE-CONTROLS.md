# App handoff: the live console is now "live controls" (#1095, S1r)

The engine renamed its live-control attachment -- the channel through which a host changes a
prepared plan's fader, mute, pan, solo, input trim/polarity/filters, effect parameters and bypass
while it renders, and reads its meters and observations -- from "console" to "live controls"
(owner decision 12, "Naming", `docs/rulings/engine-footprint-2026-09-29.md`). "Console" now means
only the session-level console strip (S1a onward).

This is an in-place V1 amendment. No behaviour, wire record, offset, rendered bit or `_v1`
identity changed, and `ABI_VERSION` did not move. The app and the SDK update in lockstep: every
old spelling below is retired, with no alias, and is never exported again for any meaning.

## SDK (`@misofm/engine`, `@misofm/engine/headless`, `@misofm/engine/browser`)

The package's import paths are unchanged.

| Old | New |
|---|---|
| `EngineConsole` (class) | `EngineLiveControls` |
| `ConsoleEdits` (class) | `LiveControlEdits` |
| `ConsoleRack` (type) | `LiveControlRack` |
| `ConsoleChannel` (type) | `LiveControlChannel` |
| `ConsoleSubmit` (type) | `LiveControlSubmit` |
| `ConsoleBeforeSubmit` (type) | `LiveControlBeforeSubmit` |
| `ConsoleWriter` (class) | `LiveControlWriter` |
| `createBrowserConsole(host, beforeSubmit?)` | `createBrowserLiveControls(host, beforeSubmit?)` |
| `BrowserEngine.console()` (browser `createEngine`) | `BrowserEngine.liveControls()` |
| `OfflineEngine.console()` (headless `createOfflineEngine`) | `OfflineEngine.liveControls()` |
| `ObservationSubscriptionTransport.console()` | `ObservationSubscriptionTransport.liveControls()` |
| `ObservationSubscriptionOwner.beforeConsoleSubmit(...)` | `ObservationSubscriptionOwner.beforeLiveControlSubmit(...)` |
| `BootOptions.console` (`createOfflineEngine` options) | `BootOptions.liveControls` |
| `BrowserBootPolicy.console` (`createEngine({ policy })`) | `BrowserBootPolicy.liveControls` |
| `POLICY_WORDS` entries `consoleCommandQueueRecords`, `consoleMeterBlocks`, `consoleObservationTaps`, `consoleMasterTrackPlusOne` | `liveControlCommandQueueRecords`, `liveControlMeterBlocks`, `liveControlObservationTaps`, `liveControlMasterTrackPlusOne` |

The fields inside the options object are unchanged: `{ commandQueueRecords, meterBlocks,
observationTaps, masterTrackPlusOne }`. So

```ts
createEngine({ document, policy: { console: { commandQueueRecords: 64, meterBlocks: 12 } } });
const controls = await engine.console();
```

becomes

```ts
createEngine({ document, policy: { liveControls: { commandQueueRecords: 64, meterBlocks: 12 } } });
const controls = await engine.liveControls();
```

Unchanged: `TrackEdits`, `EffectEdits`, `LaneOptions`, `SmoothingOptions`, `InputFilterValues`,
`InputFilterOptions`, `MatrixValues`, the `LiveEffectParameter*` types, `LaneEdit`,
`CommandReport`, and every method on the edit builders. The rack tokens (`"simd1"`, `"dynamic"`,
`"simd2"`) are unchanged here; S1c (#1096) changes the addressing.

The refusal messages changed with the names: "this engine booted with no live controls attached;
set policy.liveControls.commandQueueRecords" (browser) and "... set
liveControls.commandQueueRecords" (headless).

## AudioWorklet host (`miso-engine-v1-audio-worklet-host.js`, `.d.ts`)

For an app that drives the shipped worklet host directly rather than through the SDK.

| Old | New | Offset, type |
|---|---|---|
| boot word `consoleCommandQueueRecords` | `liveControlCommandQueueRecords` | 32, `u64` |
| boot word `consoleMeterBlocks` | `liveControlMeterBlocks` | 40, `u64` |
| boot word `consoleObservationTaps` | `liveControlObservationTaps` | 48, `u64` |
| boot word `consoleMasterTrackPlusOne` | `liveControlMasterTrackPlusOne` | 56, `u64` |

These are the `MisoWebBootOptions` fields passed as `createMisoAudioWorkletHost({ options })` and
the `bootOptions` rows of `miso-engine-v1-abi-layout.json`. Offsets, types and meanings are
unchanged.

## Wasm exports

| Old | New |
|---|---|
| `miso_engine_web_v1_console_track_count` | `miso_engine_web_v1_live_control_track_count` |
| `miso_engine_web_v1_console_track_id` | `miso_engine_web_v1_live_control_track_id` |

Signatures are unchanged. The shipped module's bytes change because the export names and a few
diagnostic strings changed; the release pin moves at release (`docs/RELEASE.md`).

## Diagnostic codes

| Old | New |
|---|---|
| `web.options.console` | `web.options.live_controls` |
| `web.console.config` | `web.live_controls.config` |
| `web.console.input_filter` | `web.live_controls.input_filter` |
| `web.console.effects` | `web.live_controls.effects` |
| `web.console.observation` | `web.live_controls.observation` |
| `web.internal.console` | `web.internal.live_controls` |
| `host.observation.console` | `host.observation.live_controls` |
