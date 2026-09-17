# `@misofm/engine`

Engine V1 is a headless Rust mixing and mastering engine. This package provides its TypeScript
session builder, semantic controls, analysis APIs, and packaged WebAssembly runtime for offline
rendering and browser playback. The engine produces planar `Float32Array` PCM; your application
owns source delivery, playback UI, storage, and output encoding.

Sessions are strict, versioned canonical JSON. Tracks have independent left/right processing,
three effect racks, explicit routing and integer-sample delay compensation. The native effect
library includes EQ, compressor, gate/expander, limiter, multiband compressor, saturator/clipper,
transient shaper, and delay. The generated catalog describes their parameters,
units, domains, and observation capabilities.

## Install and choose an entry point

```sh
npm install @misofm/engine@0.4.3
```

The package ships compiled ESM and TypeScript declarations, the `simd128` Wasm engine, its
AudioWorklet host and processor, scratch/analysis Workers, PCM-feed worklet, parameter metadata,
ABI layout, and an artifact manifest. Headless consumers need no Rust toolchain or separate engine
download. Keep the SDK, Wasm, host, worklets, and metadata from the same package release together.

| Public import | Use |
| --- | --- |
| `@misofm/engine` | `session`, `effect`, generated catalog/ABI, parameter helpers, `EngineConsole`, `ConsoleWriter`, and shared types |
| `@misofm/engine/headless` | `createOfflineEngine`, validation, bundled-asset loading, and response previews |
| `@misofm/engine/browser` | `createEngine`, browser preparation, PCM rings/feed, measurements, and response previews |
| `@misofm/engine/assets` | `BUNDLED_ENGINE_ASSETS` URLs and `BUNDLED_ENGINE_FILES` names |

Use these entry points rather than deep imports. The package also installs the `enginectl` CLI.

```ts
import { catalog } from "@misofm/engine";
import { BUNDLED_ENGINE_ASSETS } from "@misofm/engine/assets";

export function packageInfo() {
  return { parameterCount: catalog().length, wasmUrl: BUNDLED_ENGINE_ASSETS.wasm };
}
```

## Render offline

Node 20+ and Bun use the packaged Wasm runtime. Supply a Session V1 document as JSON text, bytes,
or a builder returned by `session(...)`. `validate()` checks a document by booting the actual
engine; `shape()` reports the rate, quantum, sources, tracks, and ring size the engine compiled.

This example renders the first quantum from caller-decoded PCM. Each declared source must contain
at least one quantum; the map supplies that many frames per channel at the session's sample rate.

```ts
import { createOfflineEngine } from "@misofm/engine/headless";

export async function renderFirstQuantum(
  document: string,
  pcmBySource: ReadonlyMap<string, readonly Float32Array[]>,
) {
  const engine = await createOfflineEngine(document);
  try {
    const shape = engine.shape();
    for (const source of shape.sources) {
      const planes = pcmBySource.get(source.id);
      if (!planes) throw new Error(`Missing PCM for ${source.id}`);
      const result = engine.submitSource({
        sourceId: source.id,
        generation: 1n,
        startFrame: 0n,
        planes,
        endOfRegion: source.frames === BigInt(shape.quantumFrames),
      });
      if (!result.ok) throw new Error(`Source refused: ${result.code}`);
    }
    return engine.render(); // Owned left/right output arrays, one quantum.
  } finally {
    engine.dispose();
  }
}
```

For a full render, keep the engine alive, submit bounded source chunks with increasing
`startFrame`, and render each quantum. Handle submission backpressure before advancing your
producer. `seekSource()` changes the source generation; stale chunks cannot satisfy the new seek.
`loadSession()` replaces the headless session on the same instance and invalidates its old
subscriptions. Use `loadBundledEngineAsset()` and the `asset` option to share one compiled module
across engines.

## Play in the browser

`createEngine()` scratch-boots in a Worker, learns the session's shape, verifies the `AudioContext`
rate and render quantum, and boots the AudioWorklet. Package-relative asset URLs are the defaults;
your bundler/server must deliver the shipped Worker, worklet, Wasm, and companion files.

The callback below is your source-delivery integration: attach and prefill decoded PCM before
resuming playback. Call `startMix` from your application's playback action.

```ts
import { createEngine } from "@misofm/engine/browser";
import type { BrowserEngine } from "@misofm/engine/browser";

export async function startMix(
  document: string,
  prepareSources: (engine: BrowserEngine) => Promise<void>,
) {
  const engine = await createEngine({
    document,
    policy: { console: { commandQueueRecords: 64, meterBlocks: 12 } },
  });
  try {
    await prepareSources(engine);
    engine.host.node.connect(engine.context.destination);
    await engine.context.resume();
    return engine; // Call await engine.close() when finished.
  } catch (error) {
    await engine.close();
    throw error;
  }
}
```

For shared-ring delivery, the browser entry exposes `prepareEngineFeed`, `attachEngineFeed`,
`Msb1RingWriter`, and `waitForPcmRunway`. These transport already-decoded PCM; they do not fetch or
decode media. The shared-ring path requires `SharedArrayBuffer` and a cross-origin-isolated page.
The host also exposes bounded source submission. Source underruns produce silence and counters.

Browser execution requires WebAssembly SIMD, Workers, and AudioWorklet in a secure context; there
is no shipped scalar fallback. Supported session rates are exactly **44.1, 48, 88.2, and 96 kHz**.
The device/context must accept the session rate and quantum (normally 128 frames); mismatches are
refused. Browser source delivery supports declared 16- and 24-bit sources at launch, decoded to
`f32` PCM. Declared `32f` sources remain outside browser launch delivery; raw-document consumers
must enforce that restriction at their ingest boundary.

The npm hosts both execute Wasm. Native and mobile applications must integrate the separate
Rust/C-ABI host layer and platform audio callbacks; this package supplies no native binaries or
mobile audio-device adapter.

## Change a mix and correlate the acknowledgement

Prepare a positive `console.commandQueueRecords` capacity at boot: pass `console` directly to
`createOfflineEngine`, or under `policy` to `createEngine`. Audio-only boot remains valid without
it. `engine.console()` is synchronous headlessly and asynchronous in the browser; `await` works
for either. With an existing `vocal` track and an EQ in its first SIMD rack slot:

```ts
import type { EngineConsole } from "@misofm/engine";

export async function adjustVocal(controls: EngineConsole) {
  const vocal = controls.edit.track("vocal");
  const report = await controls.submit(
    vocal.faderDb(-3, { channel: "both", smoothingSamples: 64 }),
    vocal.effect("simd1", 0, "miso.parametric-eq")
      .parameter({ key: "band-1-gain", value: 6, channel: "both" }),
  );
  if (!report.ok) throw new Error(`${report.code}: ${report.reasonName}`);
  return report.appliedAtSample;
}
```

One `submit()` is one atomic admission: all edits are accepted or none are. A successful report's
`appliedAtSample` is the absolute engine sample at which the edits take effect, at the start of
the next rendered block. It is not a wall-clock time or proof that playback has already reached
that block. Smoothing may continue after that boundary. Refusals carry typed result/reason names;
queue saturation must be handled, and acknowledged batches are not silently dropped.

Parameter keys and units come from the catalog. `inputFilters({ hpfHz, lpfHz })` changes both
builtin cutoffs atomically; zero disables a filter. `ConsoleWriter` adds bounded batch submission
and latest-value coalescing for gesture loops. The root entry's `parameter()` helper provides exact
decimal/lattice edits for agent-facing controls.

## Observe the rendered mix

Choose observation capacity and spectrum targets when preparing the engine. Analysis and delivery
are bounded; notifications can arrive later than the samples they describe.

Set the prepared continuous-spectrum hop with `BootOptions.spectrumHopFrames` when calling
`createOfflineEngine`, or with `BrowserBootPolicy.spectrumHopFrames` under `policy` for
`createEngine`. The allowed values are `256`, `512`, `1024`, and `2048` frames. This is a
preparation-time structural setting passed as a validated boot number: it is outside canonical
session JSON and is not a Wasm environment variable. Omit it to retain the rate/quantum-derived
default (`48 kHz` with a `128`-frame quantum derives `2048`). Headless and browser preparation
have the same semantics. For an explicit browser hop, the asset must expose
`miso_engine_web_v1_spectrum_hop_capability` returning exactly `1` and the additive
`miso_engine_web_v1_boot_with_spectrum_hop`; an old or incompatible asset is refused without
falling back to the default path.

Every spectrum result uses a fixed `2,048`-frame analysis window. The prepared hop controls the
window start spacing, so hops below `2048` overlap successive windows. Runtime `cadenceMs` controls
subscription delivery and `smoothingMs` controls power smoothing; neither changes the prepared
native hop. Halving the hop approximately doubles analysis and publication frequency and their CPU work.
Native capture storage and the SDK's bounded buffers and retained result storage keep memory bounded
instead of growing with stem duration.

| Observation | API and meaning of its time |
| --- | --- |
| Track/master peaks | Browser `subscribeMeters()`; headless `meters(true)` then `pollMeters()`. Linear peak magnitudes cover `[firstSample, endSample)`. This span timestamps peaks only, not an exact gain-reduction join. |
| Resident effect values | `observationMap()`, `readObservations()`, or `subscribeObservations()`. Select stable track/rack/effect-slot/tap IDs. Ready values retain the effect owner's own window and native unit. |
| Live EQ/filter response | `queryTrackResponse()` or `subscribeTrackResponse()`. `capturedSample` marks capture of applied target state at a block boundary. The curve is a stationary EQ/input-filter subtotal; fader, pan, routing, and unsupported processors are excluded. |
| Spectrum | Browser `querySpectrum()`; headless `armSpectrum()`, explicit renders, then `readSpectrum()`. Each result covers 2,048 captured PCM samples, `[capturedSample, endSample)`, with dBFS bins and a graph-wide source-underrun flag. |
| Continuous spectrum | `subscribeSpectrum()` retains one managed stream. Metadata carries the capture span, hop, epochs, drops, and smoothing history. The default 100 ms power smoothing includes earlier windows; set `smoothingMs: 0` for unsmoothed analysis. |
| Render telemetry | Browser `subscribeTelemetry()` reports CPU/deadline measurements. It has no audio sample span. |

For spectrum, pass either `spectrum` for one boundary or `spectrumCollection` for several
budgeted boundaries at engine creation. Supported targets are `trackPostInputBuiltins`,
`trackPostMatrix`, and `output`. A collection permits atomic managed selection among prepared
entries, with one active spectrum producer. It does not enable simultaneous independent streams.

Managed observation, response, and spectrum handles expose `readLatest()`, `update()`, and
`close()`. Browser delivery progresses automatically; headless consumers call `pump()` between
explicit renders. Pumping never renders audio. Identical subscriptions share work; refused updates
preserve the prior configuration. Prepare `console.observationTaps` for resident effect taps.

Use the recorded sample spans to compare observations with `appliedAtSample`. A window crossing
the command boundary can contain both states. A later window may still contain filter settling
or smoothing history. Response subscriptions suppress unchanged state and retain its original
capture timestamp; a new polling callback does not imply a new capture. These APIs do not promise
sample-identical display timing, a shared publication clock across hosts, or an instantaneous
match between a target curve and measured PCM.

Browser meters additionally expose generation, validity, and loss fields. Inspect them before
joining windows; gaps can mean omitted publications. Gain reduction is non-negative dB; track
zero can also mean unobserved, while master `null` means unavailable. For hypothetical or paused
edits, `createResponsePreview()` evaluates an explicit configuration without a live session or
live-session timestamp.

The release also carries an additive **protected-observation native/Wasm ABI** for explicitly
prepared hosts, with bounded observation work, admission, and receipts. “Protected” describes
resource/ownership guarantees, not DRM or encrypted PCM. Ordinary SDK creation does not activate
that profile. Combined ordinary/protected preparation and protected EQ coexistence are not part
of this release; see the [coexistence policy](https://github.com/misofm/engine/issues/835).

## Ownership and artifact integrity

The realtime engine owns a preallocated render plan and executes on one render thread. Render
performs no allocation/free, locks, file/network I/O, logging, or syscalls. Compilation, structural
preparation, source decoding, and analysis stay off render. The headless JavaScript wrapper copies
output into owned arrays; the engine's realtime guarantee does not make arbitrary SDK calls or
application callbacks allocation-free. See the
[realtime policy](https://github.com/misofm/engine/blob/main/docs/REALTIME_DEPENDENCY_POLICY.md).

Use bounded source delivery rather than loading complete stems solely for rendering. Content
resolution, transport-byte verification, decoding, and seek coordination belong to the caller.
There is no implicit sample-rate conversion. The control protocol carries commands, never PCM.

`BUNDLED_ENGINE_ASSETS` locates the installed release's assets. Its
`miso-engine-v1-sdk-manifest.json` records byte counts and SHA-256 digests for the seven engine
artifacts, including Wasm, host, processor, metadata, ABI layout, and prepared-control helper.
`loadBundledEngineAsset()` verifies the Wasm length and digest before compilation; boot checks
the exported ABI version. Browser package-relative URLs bind deployment locations but do not
constitute runtime digest verification. Custom delivery must preserve the matching artifact set;
`MisoEngineAsset.load(bytes, expectedSha256)` is available for explicit byte verification.

Metadata and TypeScript ABI declarations are generated from Rust. Release provenance binds the
published archive to its source/workflow; the
[0.4.3 release record](https://github.com/misofm/engine/issues/865) contains the accepted artifact
and archive identities. The manifest is an integrity reference, not an independent trust root.

## Build a session from the command line

```sh
enginectl session build --request request.json --output session.json
enginectl session build --request - --output - < request.json
```

Requests use `schemaVersion: 1`, a required `session` object, and optional `sources`, `tracks`,
`submixes`, `outputs`, `routes`, and `automation` arrays. The CLI validates with the packaged engine before publishing
canonical Session V1 JSON. File output preserves existing destinations unless `--overwrite` is
specified; stdout output contains only the document. It does not download or decode stems.
See the [CLI request shape](https://github.com/misofm/engine/blob/main/sdk/src/cli/session-request.ts) and
[Session V1 schema](https://github.com/misofm/engine/blob/main/docs/SESSION_SCHEMA_V1.md).

## Boundaries and contributing

This package does not provide a DAW/timeline UI, delivery codecs, an unbounded media cache,
implicit feedback routing, third-party Wasm effect execution, or a general remote audio transport.
Native effect availability does not imply a third-party plugin loader.

From a repository checkout, install locked SDK dependencies and check the generated/public surface:

```sh
cd sdk
npm ci
npm run check:generated
npm run check:assets
bash ../scripts/check-sdk-types.sh
```

`npm run check:assets` requires Rust. `npm run build` prepares the packaged artifacts;
`npm run check:package` additionally checks a fresh packed consumer, public imports, rendering,
and digest rejection. See the [engine guide](https://github.com/misofm/engine/blob/main/AGENTS.md)
for realtime, DSP, and contribution requirements. Licensed under Apache-2.0.
