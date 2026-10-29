# `@misofm/engine`

Engine V1 is a headless Rust mixing and mastering engine. This package provides its TypeScript
session builder, semantic controls, analysis APIs, and packaged WebAssembly runtime for offline
rendering and browser playback. The engine produces planar `Float32Array` PCM; your application
owns source delivery, playback UI, storage, and output encoding.

Sessions are strict, versioned canonical JSON. Tracks have independent left/right processing, a
session-level console strip, per-track inserts, explicit routing and integer-sample delay
compensation. The native effect
library includes EQ, compressor, gate/expander, limiter, multiband compressor, saturator/clipper,
transient shaper, and delay. The generated catalog describes their parameters,
units, domains, and observation capabilities.

## Install and choose an entry point

```sh
npm install @misofm/engine@0.10.1
```

The package ships compiled ESM and TypeScript declarations, the `simd128` Wasm engine, its
AudioWorklet host and processor, scratch/analysis Workers, PCM-feed worklet, parameter metadata,
ABI layout, and an artifact manifest. Headless consumers need no Rust toolchain or separate engine
download. Keep the SDK, Wasm, host, worklets, and metadata from the same package release together.

| Public import | Use |
| --- | --- |
| `@misofm/engine` | `session`, `effect`, generated catalog/ABI, parameter helpers, `EngineLiveControls`, `LiveControlWriter`, and shared types |
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

## Build a session

A session declares its console once: `preInsert` slots run before each track's inserts and
`postInsert` slots after them, each with a slot ID, an effect, and the quality and link mode every
track runs it at. Every track then carries one `console` entry per slot, in slot order, with only
its own `bypass` and parameters, plus its own `inserts`. A console slot is one of the seven effects
in `CONSOLE_ELIGIBLE_EFFECTS` and takes no sidechain; a keyed or other effect is an insert.

```ts
import { effect, session } from "@misofm/engine";

export function vocalMix() {
  return session({ id: "vocal.mix", sampleRateHz: 48_000 })
    .source("vox", { channels: 2, bitDepth: 24, frames: 480_000, content: `blake3:${"0".repeat(64)}` })
    .console({
      preInsert: [
        { slot: "eq", effectId: "miso.parametric-eq" },
        { slot: "comp", effectId: "miso.compressor" },
      ],
      postInsert: [{ slot: "limiter", effectId: "miso.true-peak-limiter", linkMode: "maximum" }],
    })
    .track("vocal", {
      source: "vox",
      console: [
        { slot: "eq", parameters: { "band-1-enabled": true, "band-1-gain": 3 } },
        { slot: "comp", bypass: true },
        { slot: "limiter" },
      ],
      inserts: [effect("miso.delay", { "delay time": 120 }, { slotId: "echo" })],
    })
    .output("main")
    .route({
      id: "vocal-main",
      source: { kind: "track", trackId: "vocal", tap: "post_pan" },
      destination: { kind: "output_input", outputId: "main" },
    });
}
```

The builder refuses what the engine refuses -- a missing, repeated or misordered console entry, an
unknown slot, an ineligible effect, a sidechain or bypass on a slot, a link mode the effect does
not support (`EFFECT_LINK_MODES`: the EQ, soft-clip, delay and reverb are `dual_mono` only, and the limiter
has no `average`) -- and `MisoUsageError`'s `diagnosticCode` is the engine's own code for it
(`console.entry_order`, `console.slot.ineligible_effect`, `effect.link_mode.unsupported`, ...).
The seven track taps are `input`, `post_input`, `insert_send`, `insert_return`, `pre_fader`,
`post_fader` and `post_pan`.

A session also says how long its live mute, fader and pan edits ramp. Pass `controlSmoothing:
{ muteMs, faderMs, panMs }` to `session(...)`, each a finite number of milliseconds in `[0, 1000]`
(`0` is a hard step); the builder refuses anything else at `session().controlSmoothing`. Omitted,
the document says `{ "kind": "default" }` and the engine uses its default table of 10 ms for mute,
20 ms for fader and 20 ms for pan, rounded to samples at the session's rate. The mute length also
governs solo, send mutes and VCA mutes, and twice it governs polarity invert; the fader length
governs input trim, send gain and VCA offsets; the pan length governs the matrix.

### Placement and mono sources

A track or submix built without `pan` gets the identity matrix (`ll = rr = 1`, `lr = rl = 0`), the
catalog defaults of the `matrix_*` rows. It passes each lane through unchanged, so a stereo source
keeps its image. Give `pan: { left, right }` only when you want the per-lane pan: that law is equal
power on each lane, so `{ left: 0, right: 0 }` puts both lanes in the centre and folds a stereo
source to mono.

A mono source reads its channel 0 on both lanes. When both lanes carry the same settings the
engine may render it as one plane and copy it at the fader/matrix seam; the output is the same
either way. With the identity matrix, a mono source plays at exactly the level of its stereo
dual-mono twin (the same signal on both channels): a mono file is a smaller copy of that twin, not
a different sound.

The recommended placement law (apply it with `matrix(...)`) is a balance, not a -3 dB equal-power
mono pan (root ruling, #2101). At the centre both sides are at 0 dB. At position `p` in `[-1, 1]`,
the far side is `cos(p * pi / 2)` and the near side stays at 0 dB: for `p > 0` the matrix is
`{ ll: cos(p * pi / 2), lr: 0, rl: 0, rr: 1 }`, and for `p < 0` it is `{ ll: 1, lr: 0, rl: 0,
rr: cos(p * pi / 2) }`. With no crossfeed
(`lr = rl = 0`), a coefficient of at most 1 cannot lift the near side above 0 dB, so a
constant-power law with 0 dB at the centre needs crossfeed, which would mix the lanes of a stereo
source and lift a mono source up to +3 dB at the sides. The balance keeps the lanes separate and
never boosts, so a mono source stays at its stereo twin's level at every position.

A strip has `pan` or `matrix`, not both. To give a matrix strip a live `pan(left, right)`, put
`matrixMode("pan")` in the same transaction: `controls.submit(strip.matrixMode("pan"),
strip.pan(left, right))`. A `pan` edit alone on a matrix strip is refused with `control.inactive`.

### Aux reverb return

`miso.reverb` is an insert, not a console slot. Put it on a submix (the aux return) at `mix` 1
(its default: the return carries only the wet signal), and send each track to it from its
`post_fader` tap. The return and the dry tracks then sum at the output.

```ts
import { effect, session } from "@misofm/engine";

export function vocalWithReverb() {
  return session({ id: "vocal.verb", sampleRateHz: 48_000 })
    .source("vox", { channels: 2, bitDepth: 24, frames: 480_000, content: `blake3:${"0".repeat(64)}` })
    .track("vocal", { source: "vox" })
    .submix("aux-b", { inserts: [effect("miso.reverb", { decay: 2400, mix: 1 }, { slotId: "verb" })] })
    .output("main")
    .route({
      id: "vocal-aux-b",
      source: { kind: "track", trackId: "vocal", tap: "post_fader" },
      destination: { kind: "submix_input", submixId: "aux-b" },
      gainDb: -6,
    })
    .route({
      id: "vocal-main",
      source: { kind: "track", trackId: "vocal", tap: "post_pan" },
      destination: { kind: "output_input", outputId: "main" },
    })
    .route({
      id: "aux-b-main",
      source: { kind: "submix", submixId: "aux-b", tap: "post_pan" },
      destination: { kind: "output_input", outputId: "main" },
    });
}
```

The send's `gainDb` sets how much of the track reaches the reverb. The reverb has no latency, so
the return needs no delay compensation.

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

### Edit a headless session

`engine.apply(tx)` sends one structural transaction built with `transaction(base)...build()`.
`engine.replaceSession(document)` sends a full document, and the engine compares it with the
committed session to make one transaction. Each call resolves with `{ revision, path }`:

- `live`: the running plan ramps to the new values from the next block.
- `model_only`: the committed session changed, but render has nothing different to do.
- `rebuild`: a new plan replaces the running plan at the next `render()`. Sources keep playing,
  and unchanged nodes keep their state.
- `unchanged` (`replaceSession` only): the document is the committed session. Nothing is
  committed, and the revision stays the same.

A transaction frame larger than the core's frame limit (`capabilities().maximumFrameBytes`,
128 KiB by default, set at boot with `maximumControlFrameBytes`) is not sent. It rejects with
`MisoControlRefusal`, `status` and `code` `limit_exceeded`, and its message and its diagnostic's
`detail` (`frame_bytes=<n>,maximum_frame_bytes=<m>`) give the frame's size and the limit. Nothing is
committed. Split the edits into smaller transactions, each one built on the session the last one
committed.

A refusal rejects, and nothing is committed. `engine.revision` is the committed revision.
`engine.applied(revision)` resolves with the watermark `{ revision, sample, outcome }` when that
revision is audible. `sample` is the first sample of the block where the revision took effect. The
engine reads the watermark after each `render()`, so `applied` stays pending until you call
`render()`. `engine.watermark()` reads the watermark immediately.

```ts
import { transaction } from "@misofm/engine/headless";
import type { OfflineEngine, SessionBuilder, TransactionBuilder } from "@misofm/engine/headless";

type TrackSpec = Parameters<TransactionBuilder["upsertTrack"]>[1];

// `upsertTrack` writes the whole track record. Pass the track's full spec from `base` and change
// only its fader, so that the edit stays a live fader move.
export async function lowerTrack(
  engine: OfflineEngine,
  base: SessionBuilder,
  trackId: string,
  spec: TrackSpec,
) {
  const edit = transaction(base)
    .upsertTrack(trackId, { ...spec, fader: { ...spec.fader, leftDb: -6, rightDb: -6 } })
    .build();
  const { revision, path } = await engine.apply(edit);
  const audible = engine.applied(revision);
  engine.render(); // Submit the source PCM for this block first, as for every block.
  return { path, firstSample: (await audible).sample };
}
```

`replaceSession()` is an edit. `loadSession()` is a reboot: every source starts again from its first
frame, `revision` goes back to `0`, and each pending `applied` rejects.

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
    policy: { liveControls: { meterBlocks: 12 } },
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

The engine runs in one of two modes, which `engine.instanceMode` (and `engine.host.instanceMode`)
reports. On a cross-origin-isolated page (`Cross-Origin-Opener-Policy: same-origin` and
`Cross-Origin-Embedder-Policy: require-corp`) the default is `"worker"`: a dedicated control
Worker boots, edits and disposes the session and drains the PCM feed, on one shared
`WebAssembly.Memory` with the AudioWorklet, which only renders. On a page without isolation the
mode is `"single"`: one instance, whose control work runs in the worklet's message handler. Pass
`instanceMode: "single"` to `createEngine` to choose single mode on an isolated page; `"worker"`
on a page without isolation is refused. Worker mode is qualified on desktop Chromium and WebKit;
iOS Safari is not qualified.

For shared-ring delivery, the browser entry exposes `prepareEngineFeed`, `attachEngineFeed`,
`Msb1RingWriter`, and `waitForPcmRunway`. These transport already-decoded PCM; they do not fetch or
decode media. The shared-ring path requires `SharedArrayBuffer` and a cross-origin-isolated page.
The host also exposes bounded source submission. Source underruns produce silence and counters.

Pass `engine` to `attachEngineFeed` to feed the sources that an edit adds or removes while audio
plays:

- **Add a stem.** After `engine.apply(tx)` (or `replaceSession`) resolves, attach a feed for the new
  source with `start: { frameAt, prime }`. `feed.ready()` reads the engine's next render sample
  once and sets the anchor `A` to that sample plus the ring's capacity in frames, rounded up to a
  quantum. It calls `frameAt(A)` once (`frameAt` is your mapping from anchor to source frame;
  `engine.position()` gives the timeline frame now), publishes the anchored seek in the ring,
  awaits `prime({ generation, frame, anchorSample })`, and then sends the seek to the engine. Write the stem's PCM from `frame` at
  `generation` in `prime`, and do not seek the ring there. The drain reads none of that ring's
  chunks until the engine applied the seek. If the audio reaches `A` before that, the engine
  starts the stem at the frame plus the lateness and reports it in the source's seek report; the
  stem stays in time. The margin in `A` is one ring of audio, so write at most one ring's worth
  in `prime` (the ring holds no more), and keep writing after `ready()` resolves. A refused seek
  rejects `ready()` with a `MisoEngineError` and closes the feed. Do not publish an anchored seek
  with `Msb1RingWriter.seek` yourself: without the SDK's message to the engine, that ring stalls.
  To start in time with a playing stem instead of your own mapping, give `start: { alignTo:
  sourceId, prime }` (or `alignTo: "$timeline"`): the SDK reads that source's seek report before
  and after the anchor is computed and uses `sourceFrame + (A - firstSample)`. To add
  several stems together, start one with `frameAt` and align the others to it: if the reference's
  own anchored start is published but not rendered yet, the SDK uses that start's frame at `A`. In
  one feed the SDK starts a source after the source it aligns to, whatever the list order; a
  self-alignment or an `alignTo` cycle throws at attach. Across feeds the SDK does not wait: a
  reference whose feed has not published its start yet refuses (`feed.align.unstarted`), so align
  after that feed's `ready()` resolved. If the engine later refuses the reference's own seek, the
  aligned stem stays at the frame the reference's `frameAt` mapping gave. A reference that has not
  rendered yet, whose report stays busy, that seeked or held frames between the two reads, or
  whose ring publishes another seek the engine has not rendered rejects `ready()` with a
  `MisoUsageError` whose `diagnosticCode` names the cause (`feed.align.*`), publishes nothing and
  closes the feed; use `frameAt` then.
- **Remove a stem.** When a committed edit removes a source of the feed, keep feeding its ring. The
  SDK detaches it with `feed.detachSource(sourceId)` after `engine.applied(revision)` resolves
  (the strip's ramp-out has then played), and releases its writer.
- `feed.detachSource(sourceId)` resolves after the worklet's drain stopped reading that ring; the
  other rings keep draining. If the worklet withdrew no ring, it rejects with `detachRefused` and
  closes the feed.

The MSB1 ring keeps version 1 with an anchor word: the control block is 144 bytes (the anchor
`i64` at 128, `-1` for a plain seek), the ID is at 144 and the slot headers at 272. The drain
refuses a ring with another header offset. The seek epoch (word 11) is a parity seqlock over the
seek's generation, frame, anchor and the generation tag: `Msb1RingWriter.seek` makes it odd,
writes them, and makes it even again, so one seek adds 2. A reader that sees an odd epoch, or an
epoch that changed while it read, reads the seek again later. A writer of its own must publish
the same way.

A plain ring seek (`Msb1RingWriter.seek(generation, frame)`) while audio plays does not cut to
silence in `single` mode. In `worker` mode it can still lose one quantum (128 frames) at the
block where the seek lands, about once in 50 seeks (#2185 stays open for it). The drain frees the
ring's chunks of the old generation at once, so the writer can write the new one, and the old
position plays on from the engine's ring until the new generation has 50 ms of audio published
(at most a ring, or up to a chunk that ends its region). Then the seek applies, a single cut. If
that runway is not published within 50 ms of rendered audio (in `worker` mode, at most half the
engine ring), the seek applies anyway and the source plays silence, counted as underruns, until
its PCM arrives. Publish the new position in one burst, as the pump does. The engine reports the
block where the seek applied in the source's seek report. A suspended seek with
`feed.prepareSeek()` applies at once, as before.

The transport seek, `engine.seek(frame, { producers })`, sends a session seek first: render then
moves every source to the new generation and drops its old chunks. Each producer's ring seek at
that generation applies at once and does not wait for a runway, because no old audio is left to
play. The transport seek can therefore still go silent until the producers publish (#2205).

Browser execution requires WebAssembly SIMD, Workers, and AudioWorklet in a secure context; there
is no shipped scalar fallback. Supported session rates are exactly **44.1, 48, 88.2, and 96 kHz**.
The device/context must accept the session rate and quantum (normally 128 frames); mismatches are
refused. Browser source delivery supports declared 16- and 24-bit sources at launch, decoded to
`f32` PCM. Declared `32f` sources remain outside browser launch delivery; raw-document consumers
must enforce that restriction at their ingest boundary.

The npm hosts both execute Wasm. Native and mobile applications must integrate the separate
Rust/C-ABI host layer and platform audio callbacks; this package supplies no native binaries or
mobile audio-device adapter.

### Boot with a PCM feed, read the playhead, play and stop

Set `pcmFeed: true` (or `pcmFeed: { moduleUrl }`) in `createEngine` to use shared-ring delivery.
The feed's worklet prelude must load before the engine worklet, because it wraps
`registerProcessor`. With `pcmFeed`, `createEngine` loads the prelude after it verifies the
context and before it creates the host, so you do not wrap `createHost`. `pcmFeed` is off by
default. If the prelude does not load, `createEngine` rejects with `PcmFeedError` (`moduleLoad`)
and closes the context.

On a cross-origin-isolated page, attach the session's sources while the context is suspended and
await `ready()` before prefilling or playing:

```ts
import { attachEngineFeed, createEngine } from "@misofm/engine/browser";

const engine = await createEngine({ document, pcmFeed: true });
const feed = attachEngineFeed({
  engine,
  context: engine.context,
  sources: [{ sourceId: "stem", channels: 2 }], // A source declared in document.
  quantumFrames: engine.shape.quantumFrames,
});
await feed.ready(); // Default attachment deadline: 2 seconds; audio stays suspended.
```

Attachment does not require audio to render. Keep the context suspended until the rings are
prefilled and playback is requested; `ready()` needs no resume/suspend or compensating seek.
Close the feed with `feed.close()` when it is no longer needed.

The engine has one timeline playhead. `engine.position()` resolves with `{ frame, generation,
atSample }`: `frame` is the timeline frame that the next rendered block starts at, and `atSample`
is the absolute render sample of that block. The SDK reads the timeline's seek report and the
host's next render sample, and calculates `frame = sourceFrame + (atSample - firstSample)`. After
`engine.seek(f)`, `position()` gives `f` until the first block renders, and then `f` plus the
frames rendered since. The frame does not move while the context is suspended. You do not need
the meters to calculate a position.

`engine.planLatency()` resolves with `{ samples, tailSamples }` on both engines: the output latency
and tail of the plan that renders now, in samples, as the C ABI's plan resource report gives them
(`tailSamples` is `null` for an unbounded tail). Use it to align meters, the playhead and visuals
with the audio. After a rebuild, it changes at the block that adopts the new plan.

`engine.play()` resumes the context and then sets the transport to `playing`. `engine.stop()`
suspends the context and then sets the transport to `stopped`. Each call starts the context step
at once, and sets the transport after that step settles, in call order. Each resolves with the
new transport state. `engine.setTransport()` alone does not start or stop the audio: the
transport state is a label that the core keeps (#1635). In the browser, the context is what
renders. A browser holds `resume()` until a user gesture: a later `play()` or `stop()` does not
wait for it, and the earlier `play()` rejects with `MisoUsageError`. A call that a newer call
supersedes before its context step settles rejects with `MisoUsageError`. If the core refuses the
transport, `play()` rejects and, if it is the newest call and the context was suspended before
it, suspends the context again. If the core refuses the transport for `stop()`, `stop()` rejects
and the context stays suspended, but the transport label still says `playing`.

The session has no master strip. The master is an ordinary submix that you designate after boot
with `controls.edit.submix(id).master(true)` (#1683). Its meter row carries gain reduction only
(#1690 D3).

```ts
import { createEngine } from "@misofm/engine/browser";
import type { BrowserEngine, SessionSeekProducer } from "@misofm/engine/browser";

export async function openFedMix(
  document: string,
  attachSources: (engine: BrowserEngine) => Promise<SessionSeekProducer>,
) {
  const engine = await createEngine({ document, pcmFeed: true });
  engine.host.node.connect(engine.context.destination);
  // Attach the feed with attachEngineFeed({ engine, ... }) and prefill its rings here.
  const producer = await attachSources(engine);
  await engine.seek(0n, { producers: [producer] });
  return engine;
}

// Call play() from a user gesture, so that the browser lets the context resume.
export const play = (engine: BrowserEngine) => engine.play();
export const stop = (engine: BrowserEngine) => engine.stop();
export const playhead = async (engine: BrowserEngine) => (await engine.position()).frame;
```

### Edit a playing session

The browser engine edits a playing session with the same calls as the headless engine.
`engine.apply(tx)` sends one structural transaction built with `transaction(base)...build()`.
`engine.replaceSession(document)` is a convenience: it sends a full document, and the engine
compares it with the committed session to make one transaction. Each call resolves with
`{ revision, path }` when the control plane commits:

- `live`: the running plan ramps to the new values from the next block.
- `model_only`: the committed session changed, but render has nothing different to do.
- `rebuild`: a new plan replaces the running plan at a later block boundary.
- `unchanged` (`replaceSession` only): the document is the committed session. Nothing is
  committed, and the revision stays the same.

A transaction frame larger than the core's frame limit (`capabilities().maximumFrameBytes`,
128 KiB by default, set at boot with `maximumControlFrameBytes`) is not sent. It rejects with
`MisoControlRefusal`, `status` and `code` `limit_exceeded`, and its message and its diagnostic's
`detail` (`frame_bytes=<n>,maximum_frame_bytes=<m>`) give the frame's size and the limit. Nothing is
committed. Split the edits into smaller transactions, each one built on the session the last one
committed, for example one bus and its routes at a time. A refusal that the browser host makes
before the frame reaches the core rejects with `MisoEngineError`, never with the host's raw
record.

A refusal rejects, and nothing is committed. The SDK does not send a refused edit again: if
another commit came first, the core refuses the edit with `revision_conflict`, and you build the
edit again on the new session. `engine.revision` is the newest committed revision that the engine
saw.

A committed revision is pending until the audio has it. `engine.applied(revision)` resolves with
the watermark `{ revision, sample, outcome }` that the worklet posts when the revision is in
effect. `sample` is the first render sample of that block. `outcome.exact` is set when the
revisions completed as committed, `outcome.transitionFallback` when a new plan could not start
warm and the swap used the transition, and `outcome.superseded` when a newer commit replaced a
plan before render adopted it. `applied` stays pending while the context does not render (for
example, while it is suspended), and it rejects at `engine.close()`. `engine.watermark()` returns
the newest watermark immediately.

Across a `rebuild`, every source keeps playing and every unchanged node keeps its state. A node
that the edit adds or changes starts from rest; `engine.counters(["stripsRestarted"])` counts each
strip that a swap started at rest. After a committed `replaceSession`, `engine.liveControls()`
binds to the new document.

Live controls follow a rebuild that adds, removes or reorders strips (issue #2189). Before a
committed `apply` or `replaceSession` on the `rebuild` path resolves, the engine reads the core's
strip order again, and every `EngineLiveControls` from this engine, also one the app already holds,
checks strips and names effects against that order: `edit.strip("<new bus>")` is accepted, and
`edit.strip("<removed bus>")` throws `MisoUsageError`. An edit staged before the rebuild keeps the
address it was built for, and the core refuses it if that strip or effect is gone. A console slot
or insert addressed by index (`effect()`, `insert(<index>)`) resolves against the session the live
controls were made from only while the core's list still shows that strip as the session declares
it: the same tracks and submixes, and the same console slots and inserts on that strip, in the
same order. A rebuild that changes the inserts of one strip leaves index addressing on the other
strips as it was. On the changed strip, and on every strip once the strips moved, an index throws
`MisoUsageError` and IDs resolve from the control list alone, so an index never addresses an insert
that the rebuild moved. A strip change that another page commits on the same host is seen
when the page next reads the strip order. If that read fails after a commit, the call still
resolves (the edit is committed), and the next read catches up.

On a page without cross-origin isolation (`"single"` mode), the worklet's message handler
prepares a `rebuild` on the audio thread. The audio thread is blocked while it prepares, and
`(await engine.host.status()).blockingRebuilds` counts each blocking rebuild. In `"worker"` mode
the control Worker prepares it, the audio thread keeps rendering, and `blockingRebuilds` stays 0.

```ts
import { transaction } from "@misofm/engine/browser";
import type { BrowserEngine, TransactionBuilder } from "@misofm/engine/browser";
import type { SessionBuilder } from "@misofm/engine";

type TrackSpec = Parameters<TransactionBuilder["upsertTrack"]>[1];

// Pass the track's full spec from `base` and change only its fader, so that the edit stays live.
export async function lowerTrack(
  engine: BrowserEngine,
  base: SessionBuilder,
  trackId: string,
  spec: TrackSpec,
) {
  const edit = transaction(base)
    .upsertTrack(trackId, { ...spec, fader: { ...spec.fader, leftDb: -6, rightDb: -6 } })
    .build();
  const { revision, path } = await engine.apply(edit);
  const { sample } = await engine.applied(revision);
  return { path, firstSample: sample };
}
```

## Load local WAV stems

The browser entry reads a stem from a local file without a decoder of your own. `readWavHeader`
reads a 16- or 24-bit integer PCM WAV's header (RIFF, or RF64/BW64 with the data size from
`ds64`; plain or `WAVE_FORMAT_EXTENSIBLE` fmt) with small reads, and returns where its samples are. For such a WAV the `data` chunk is exactly the stem's
canonical PCM (`docs/STEM_IDENTITY_V1.md`), so one slice of the file is both the input of
`blake3Stream`, which gives the session's `content` identity, and what `CanonicalPcmPump` writes
into the source's `Msb1RingWriter` ring as planar `f32`.

```ts
import { CanonicalPcmPump, Msb1RingWriter, blake3Stream, readWavHeader } from "@misofm/engine/browser";

export async function localStem(file: File) {
  const wav = await readWavHeader(file); // sampleRateHz, channels, bitDepth, frames, dataOffset, canonicalBytes
  const pcm = file.slice(wav.dataOffset, wav.dataOffset + wav.canonicalBytes);
  const identity = `blake3:${(await blake3Stream(pcm.stream())).hex}` as const;
  // Declare the source with { channels, bitDepth, frames, content: identity } and the session at
  // wav.sampleRateHz. When the engine is up, pump into the ring of that source's feed:
  const pump = (ring: SharedArrayBuffer, sourceId: string) => new CanonicalPcmPump({
    lease: { read: async () => pcm },
    sources: [{ sourceId, identity, channels: wav.channels, bitDepth: wav.bitDepth, frames: wav.frames, ring: new Msb1RingWriter(ring) }],
  });
  return { wav, identity, pump };
}
```

Call `pump.pumpUntilFull()` before `context.resume()` and again as the ring drains, and pass the
pump to `engine.seek(frame, { producers: [pump] })`: it repositions every ring it writes after
the engine accepted the seek. The pump holds at most two windows per source, the resident one and
the next one read ahead (`windowFrames`, 4096 by default), never the whole stem; it reads the
sources' windows concurrently, at most eight at a time. `readWavHeader` refuses with a `WavFormatError`, whose
`code` names the cause: a float WAV (`wav.float_unsupported`: export 16- or 24-bit PCM), more than
two channels (`wav.channels`), and a rate other than 44.1, 48, 88.2 or 96 kHz (`wav.sample_rate`).
A `data` size larger than the file is read to the file's end. The pump and the hash are the
engine's own code, copied into the package unchanged; `IncrementalBlake3` is the same hash for
bytes you already hold.

The hash runs in a WebAssembly SIMD module, `dist/assets/miso-engine-v1-blake3.simd128.wasm` (about 1.5 GB/s per core in Chromium, against about 65 MB/s for the JavaScript fallback).
`blake3Stream` loads it itself; call `await prepareBlake3()` once before `new IncrementalBlake3()`
to hash in it, and pass `prepareBlake3({ url })` if your bundler moves the file. It resolves
`"simd128"`, or `"javascript"` when the module cannot load (no WebAssembly SIMD, a failed fetch);
it never rejects, and the digests are the same either way. To hash many stems at once, run one
`blake3Stream` per Worker: each Worker loads its own copy of the module.

## Change a mix and read the core's answer

A value edit (a fader, a mute, a pan or matrix, a trim, a filter, a send, a VCA, an effect
parameter or a bypass) is a `SET_CONTROL` transaction sent to the core, on the browser and on the
headless engine alike. The builders keep one engine-wide transaction queue: one transaction is in
flight at a time, each carries the committed revision the last reply reported, and edits staged
meanwhile coalesce by address into the next transaction. `engine.liveControls()` is synchronous
headlessly and asynchronous in the browser; `await` works for either. With the `vocal` track and
`eq` console slot built above:

```ts
import type { EngineLiveControls } from "@misofm/engine";

export async function adjustVocal(controls: EngineLiveControls) {
  const vocal = controls.edit.track("vocal");
  const report = await controls.submit(
    vocal.faderDb(-3, { channel: "both", smoothingSamples: 64 }),
    vocal.console("eq", "miso.parametric-eq")
      .parameter({ key: "band-1-gain", value: 6, channel: "both" }),
    vocal.insert("echo", "miso.delay").bypass(true),
  );
  return report.revision;
}
```

`submit()` resolves with the core's report of the transaction that carried the edits,
`{ ok: true, revision, path, appliedOperations }`, where `path` is how the core applied it: `live`
(on the running plan), `model_only` (the committed value changed and nothing was audible),
`rebuild` or `monitor` (a solo). The transaction is atomic: the core applies every edit or none. A
refusal rejects with `MisoControlRefusal`, which carries the core's report
(`{ ok: false, revision, code, editIndex? }`) as `report`; its `code` is the core's own
(`control.domain`, `control.address.unknown`, `control.inactive`, `control.lane`,
`control.ramp.fixed`, ...), the same on the browser and on the headless engine. A refused
transaction is not retried and changes nothing. If several callers' edits were coalesced into one
transaction, the queue rejects only the submit that owns the edit the core named, indexes
`editIndex` into that submit's own edits, and sends the others again. Correlate a report with the
mix through `revision`; the core converges every committed value no later than the first block that
begins after the submit returns.

The SDK checks no value: a fader of 30 dB, a pan of 2, a `NaN` trim, an unknown strip, send or VCA
ID, or a lane a control does not allow are built as given and refused by the core. It refuses only
what cannot be encoded (a value that is not a number, a ramp that is not an integer in
`[0, 4294967295]`, a lane other than `left`, `right` or `both`, an ID that is not a string), with a
`MisoUsageError` before any byte is sent.

An edit that gives no `smoothingSamples` sends no ramp, and the core ramps over the session's
length for that control (its `controlSmoothing`, or the default table), so a drag never steps by
omission. An explicit `smoothingSamples` is the edit's `ramp_samples`, and `smoothingSamples: 0` is
a hard step. A control whose ramp is fixed (an input filter, a bypass) refuses an explicit one with
`control.ramp.fixed`. `pan(left, right)` writes the strip's two pan lanes and `matrix(...)` its
four coefficients; a strip in the other shape is refused with `control.inactive`. Routes also take
`followsMute(enabled)`.

`controls.edit.submix(id)` addresses a submix strip (a bus) the same way, with every edit a track
offers except `solo` (a bus is solo-safe), and `controls.edit.strip(id)` resolves either kind by
ID. `console(slot, effectId)` addresses a session console slot by its slot ID and `insert(id or
index, effectId)` one of the strip's inserts. A value edit names the instance by its stable ID, so
`console()` and `insert(id)` need no session; an index (`insert(index)`, or
`effect("console" | "inserts", index, effectId)`) resolves to its ID against the session the engine
booted when it was booted from a `session(...)` builder, and for a document booted from text, call
`controls.withSession(builder)` first, because the SDK never parses a document. `withSession()`
holds the builder to the booted document byte for byte: its `toJson()` must be exactly the booted
text, so any other session (a reordered console, a renamed slot, another insert chain) throws a
`MisoUsageError` rather than resolve IDs to addresses that name other instances. Boot the
builder's `toJson()`, or the engine's canonical text of it.

The core reads every committed transaction's events from a lane with two slots, and refuses the
next commit with backpressure until they are read. The headless engine therefore reads them after
every control frame; `engine.onControlEvent(listener)` receives each as `{ lane, frame }`, and an
event with no listener is dropped and counted in `engine.discardedControlEvents`. A frame sent
through `engine.controlFrame(frame)` draws its request ID from `engine.controlTransactions().requestIds`,
the one allocator that serves the endpoint.

Parameter keys and units come from the catalog. `inputFilters({ hpfHz, lpfHz })` sets both
builtin cutoffs in one transaction; zero disables a filter. `controls.writer()` returns a
`LiveControlWriter` that stages edits by address, coalesces latest values and flushes bounded
batches through the queue, for gesture loops; the core's `backpressure` status is flow control and
never an error. In the browser, a live-control frame the host refuses before the core rejects with
`MisoEngineError` that carries the host's result, never with the host's raw `miso.error.v1` record
(issue #2193). The writer retries only `backpressure` (more than 64 frames in flight), which drains.
A host that failed answers with its sticky record, which can also carry result 6; the SDK asks the
host's `status()`, which a failed host refuses at once, and raises that failure as `wrongState`.
It does not wait for a reply: a refusal stays `backpressure` and returns at once from a host that
has not failed, also from one that stopped answering.
Any other host result (`invalidArgument` for a frame over the limit, `wrongState` or `internal` for
a host that failed) never succeeds on resend: the writer drops the batch and throws. The root entry's `parameter()` helper provides exact decimal/lattice edits for
agent-facing controls.

Observation subscriptions (`engine.subscribeObservations(...)`) send the core's `Observe` edit
through the same control queue, naming each tap by stable key (the strip's ID, the console slot's or
insert's ID, the tap's ID), which the core resolves on the newest committed plan and refuses with a
typed `MisoEngineError` when it has no such tap. A subscription's `armed()` resolves with the sample
at which render applied every arm, from the core's `OBSERVATION_READ` rows. They need no boot
option: every plan binds every declared tap of every effect, on both hosts (issue #1590), and the
retired `liveControls.observationTaps` is refused with `MisoUsageError`. Value edits are
`SET_CONTROL` transactions and need no boot word either.

### List every control, read their values, and set any of them

The control list is the core's, not a table the SDK holds: it names every control of the running
session (strips, routes, VCAs, console slots, inserts and the session's own rows), in the core's
order, with each control's descriptor and committed value.

```ts
const list = await engine.controls();          // headless: Promise; browser: Promise
const state = await engine.controlState(list); // { revision, structureEpoch, values: Map<handle, number> }
const fader = list.controls.find((row) => row.name === "fader_db"
  && row.address.owner.kind === "strip" && row.address.lane === "left");

const controls = await engine.liveControls();
await controls.submit(controls.edit.set(fader.address, -6, { rampSamples: 64 }));
```

`engine.controls(filter?)` resolves with `{ revision, structureEpoch, headers, controls }`. Each
`controls` entry (`ControlListEntry`) carries its `address` (stable owner, section, the numeric row
`control` and the lane), the descriptor (`name`, `displayName`, `unit`, `kind`, `minimum`,
`maximum`, `default`, `taper`, `step`, `choices`, `class`, `reason`, `reopen`, `scope`,
`automatable`, `family`, `active`, ...) and the committed `value`. `headers` holds the owner and
section header rows in list order; the owners come in strip order (tracks, then submixes), then
routes, VCAs and the session. `filter.owner` restricts the list to one owner and `filter.pageLimit`
(1 to 256) sets the rows requested per page. A `class` of `rebuildOnly` or `derived` tells you
before you write that the core will not apply the edit live.

The list is read in pages. Every page names the `structureEpoch` its handles belong to; if the
structure changes between two pages (a `matrix_mode` switch changes which rows a strip uses), the
core refuses the next page `control.list.stale` and the read starts over from the first page. After
three restarts it rejects with `MisoEngineError` code `control.list.stale`. `engine.controlState(list)`
reads the values of that list's handles and rejects `control.list.stale` the same way when the
structure moved on; list again. `revision` is the revision of the first page. The list and state
reads draw their request IDs from the same allocator as value edits.

`edit.set(address, value, { rampSamples? })` stages one edit for any row, in the same transaction
as the typed builders (`faderDb`, `gainDb`, `parameter`, ...) and returns it for `submit()` or a
writer. `address` is either a generated `ControlAddress` (owner, section, the row's machine name,
lane) or the `address` of a list entry, which names the row by its numeric `control`. A row of an
insert or console slot depends on the effect it runs, and the SDK reads that from the control list's
section headers, not from the session it booted: an insert or console slot that is in the list can
be named by its machine name. A named row takes its effect from the list when the edit is staged,
a typed effect builder takes the effect it is typed for, and the SDK sends that effect in field 7
(`expected-effect`) of the `SET_CONTROL` address. If that insert or slot runs another effect when the core applies the edit
(`insert("comp", "miso.compressor")` after a rebuild made `comp` a soft clip), the core refuses
the transaction with `control.effect.changed`, which `submit()` rejects as `MisoControlRefusal`, and
nothing changes. A named row stays bound to the effect it was resolved for when the edit was staged.
In the browser, the live controls resolve the next names against the list a committed rebuild left
(see "Edit a playing session"); headless, call `engine.liveControls()` again after a rebuild so that
names resolve against the new list. A numeric address names
no effect unless it carries `expectedEffect`, and without it the edit applies to whatever runs
there. A section the list does not have cannot be named; use the list entry's numeric address.
`set` checks only that the edit can be encoded. The value's domain, the lane, the owner and whether the
row can be set are the core's: an edit of a derived row rejects `control.read_only`, and one of a
rebuild-only row is committed on the rebuild path, as on the C ABI: the core prepares a replacement
plan and the engine adopts it at a block boundary.

The typed builders are generated from the same control tables (`src/generated/control-builders.ts`),
one method per settable row: a strip's own rows and builtins (`faderDb`, `mute`, `trimDb`, `pan`,
`matrix`, `matrixMode`, ...) on `edit.track(id)` and `edit.submix(id)`, a send's on `edit.route(id)`,
a VCA's on `edit.vca(id)`, the session's own on `edit.session()`, and an effect instance's
`parameter`, `bypass` and `linkMode` on `insert(id, effectId)` or `console(slot, effectId)`. Each is
sugar over `set`: it stages the edit `set` stages for the row's address and value. A row the core
gains reaches the builders when you regenerate. `pan`, `matrix` and `inputFilters` write several
rows in one transaction.

## Observe the rendered mix

Choose observation capacity and spectrum targets when preparing the engine. Analysis and delivery
are bounded; notifications can arrive later than the samples they describe.

Set the boot's spectrum taps with `spectrum` (one tap) or `spectrumCollection` (several, one
selected at a time) when calling `createOfflineEngine` or `createEngine`. Set their continuous-spectrum
hop with `BootOptions.spectrumHopFrames` for `createOfflineEngine`, or with
`BrowserBootPolicy.spectrumHopFrames` under `policy` for `createEngine`. The allowed values are `256`,
`512`, `1024`, and `2048` frames; omit it to keep the rate/quantum-derived default (`48 kHz` with a
`128`-frame quantum derives `2048`). Both engines send the taps and the hop to the core in one
`SPECTRUM_CONFIGURE` before the engine is returned, and refuse an engine whose `CAPABILITIES_GET` lacks
capability flag bit 16 (`sdk.asset.capability`).

Change the taps or the hop of a running session with `configureSpectrum(query | collection, {
hopFrames })` on either engine; an empty collection with `maximumCaptureBytes: 0` removes them. A
change of taps prepares a replacement plan: the reply reports `rebuild` and the epochs, and the new
taps are in effect once a spectrum reply reports `adoptedEpoch === configuredEpoch`. A stream on an
entry that a collection keeps runs on; a change between the single form and a collection ends it. The
core checks every tap, channel mask and budget, and a refusal rejects with `MisoEngineError` whose
`diagnosticCode` is the core's code (`spectrum.target.unknown`, `spectrum.budget`, `spectrum.busy`,
...).

Every spectrum result uses a fixed `2,048`-frame analysis window. The configured hop controls the
window start spacing, so hops below `2048` overlap successive windows. Runtime `cadenceMs` controls
subscription delivery and `smoothingMs` controls power smoothing; neither changes the configured
hop. Halving the hop approximately doubles analysis and publication frequency and their CPU work.
Native capture storage and the SDK's bounded buffers and retained result storage keep memory bounded
instead of growing with stem duration.

| Observation | API and meaning of its time |
| --- | --- |
| Track/master peaks | Browser `subscribeMeters()`; headless `meters(true)` then `pollMeters()`. Linear peak magnitudes cover `[firstSample, endSample)`. This span timestamps peaks only, not an exact gain-reduction join. A headless frame names its own strips (`trackIds`, `submixIds`) by the control list of the `structureEpoch` it carries, so it stays right across a rebuild, including one that keeps the strip count. |
| Resident effect values | `observationMap()`, `readObservations()`, or `subscribeObservations()`. Select stable track/rack/effect-slot/tap IDs. The map is the core's control list (each console slot's and insert's effect) with the generated catalog's taps; a read is one `OBSERVATION_READ`. Ready values retain the effect owner's own window and native unit. A managed subscription follows a rebuild (the core keeps the tap armed on the effect that carries on). When a commit removes the effect, the core ends the tap with `OBSERVATION_ENDED`: pass `onEnd` to `subscribeObservations()` to be told which selections ended and which remain, once for each commit; `handle.ended` lists them, and a subscription left with no selection is closed. |
| Live EQ/filter response | `queryTrackResponse()` or `subscribeTrackResponse()`. `capturedSample` marks capture of applied target state at a block boundary. The curve is a stationary EQ/input-filter subtotal; fader, pan, routing, and unsupported processors are excluded. |
| Spectrum | Browser `querySpectrum()`; headless `armSpectrum()`, explicit renders, then `readSpectrum()`. Each result covers 2,048 captured PCM samples, `[capturedSample, endSample)`, with dBFS bins and a graph-wide source-underrun flag. |
| Continuous spectrum | `subscribeSpectrum()` retains one managed stream. Metadata carries the capture span, hop, epochs, drops, and smoothing history. The default 100 ms power smoothing includes earlier windows; set `smoothingMs: 0` for unsmoothed analysis. |
| Master loudness | `subscribeLoudness()` starts the session's one output loudness meter and returns a handle with `readLatest()`, `pump()`, `reset()` and `close()`. A `LoudnessReading` gives BS.1770-5 momentary, short-term and integrated LUFS, the EBU Tech 3342 range in LU, their maxima, and the Annex-2 true peak in dBTP of the rendered master pair; `observedSample` ends the last 10 ms hop it covers. |
| Render telemetry | Browser `subscribeTelemetry()` reports CPU/deadline measurements. It has no audio sample span. |

For spectrum, pass either `spectrum` for one boundary or `spectrumCollection` for several
budgeted boundaries at engine creation. Supported targets are `trackPostInput`,
`trackPostPan`, and `output`. A collection permits atomic managed selection among prepared
entries, with one active spectrum producer. It does not enable simultaneous independent streams.

An `armSpectrum()` that `readSpectrum()` has not returned and `cancelSpectrum()` has not ended
belongs to the caller, and a managed `subscribeSpectrum()` that would select another entry is
refused with typed `backpressure` until it does. A headless sequence `subscribeSpectrum(A)`, close,
`armSpectrum()`, `subscribeSpectrum(B)` therefore throws `backpressure`: read or cancel the armed
window first.

Managed observation, response, and spectrum handles expose `readLatest()`, `update()`, and
`close()`. Browser delivery progresses automatically; headless consumers call `pump()` between
explicit renders. Pumping never renders audio. Identical subscriptions share work; refused updates
preserve the prior configuration. Two managed operations (subscribe, update, close) that each send a record for the same
effect tap need a render between them: the second is refused with typed `backpressure` until a
render has drained the first. Each handle sends its own `Observe` edits (one arm per selection, one
disarm per selection at close) and the core shares one arm between the subscribers of a tap and
window (issue #1590): an identical second subscribe, and a close that leaves another subscriber on
the tap, queue no record and need no render. Another window on a tap a subscriber holds is refused
with `MisoUsageError` (`observation.window_conflict`). In a headless engine the refused
operation rejects at once: render, then retry. In a browser engine it waits for a render and sends
again, up to `requestDeadlineMs` and at most 8 resends (one render drains the tap, so more means
another writer keeps refilling it), and then ends in typed `backpressure`. Resident effect taps
need no boot option: every plan binds every declared tap (issue #1590).

`subscribeLoudness()` measures the rendered master pair. Momentary loudness (M) is the last 400 ms,
short-term (S) the last 3 s, integrated (I) the gated mean since the last start or reset, and the
loudness range (LRA) the spread of the short-term values since then; true peak (TP) is the 4x
oversampled peak per channel. The engine updates the meter every 10 ms hop; the handle reads it about
every 100 ms (the browser on a timer, the headless engine at each `pump()` between your renders).
A level is absent until its window holds enough audio, and silence reads -120. `reset()` restarts
I, LRA, the maxima and TP from the next hop boundary (`integratingSince`); M and S run on.
`hopsDropped` counts the hops the engine lost over the session's whole life: subtract two readings
to see a loss between them. The meter is one per session: a second `subscribeLoudness()` while a
handle is open is refused with `MisoEngineError` (`diagnosticCode` `loudness.active`). After
`close()` the handle keeps its last reading and refuses `pump()` and `reset()`. A timed read that
fails goes to the request's `onError`; the readings stop advancing until a read lands.

Use the recorded sample spans to compare observations with the first block after a transaction. A window crossing
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

In the browser, every frame the engine posts to its host (a spectrum, observation, loudness or
track-response frame, a protocol call, a live-control submit or a capability probe) and its
`status()` (the PCM feed's too), `sessionMap()` and `dispose()` calls go through one path (issue
#2201). When the host refuses a call before the core sees it, or has failed, the call rejects with
`MisoEngineError` that carries the host's result, never with the host's raw `miso.error.v1`
record. A boot that the default host refuses rejects `createEngine()` with `MisoEngineError`,
phase `boot`, with the host's result and the raw record as `cause`. A failed host's sticky
result 6 is `wrongState`, as for the live controls. Each call keeps its retry rule, which reads
the error's code: an observation refused for `backpressure` waits for a render and is sent again,
and every other refusal is thrown.

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
`miso-engine-v1-sdk-manifest.json` records byte counts and SHA-256 digests for the eight engine
artifacts: the engine Wasm, host, processor, control Worker, host declaration, metadata, ABI
layout, and the BLAKE3 Wasm.
`loadBundledEngineAsset()` verifies the Wasm length and digest before compilation; boot checks
the exported ABI version. Browser package-relative URLs bind deployment locations but do not
constitute runtime digest verification. Custom delivery must preserve the matching artifact set;
`MisoEngineAsset.load(bytes, expectedSha256)` is available for explicit byte verification.

Metadata and TypeScript ABI declarations are generated from Rust. Release provenance binds the
published archive to its source/workflow; the
[0.10.1 release record](https://github.com/misofm/engine/issues/2203) contains the accepted artifact
and archive identities. The manifest is an integrity reference, not an independent trust root.

## Build a session from the command line

```sh
enginectl session build --request request.json --output session.json
enginectl session build --request - --output - < request.json
```

Requests use `schemaVersion: 1`, a required `session` object, an optional `console` object
(`preInsert` and `postInsert` slot arrays), and optional `sources`, `tracks`, `submixes`,
`vcas`, `outputs`, `routes`, and `automation` arrays; each track spec carries its `console` entries
and `inserts`. A `submixes` entry is a bare ID, which is a transparent strip, or an object
`{ id, builtins?, console?, inserts?, fader?, pan? }`, a submix strip with a track's strip keys.
A `vcas` entry is `{ id, fader?, members }`: a VCA group whose members are tracks, submixes and
other VCAs. The CLI declares the VCAs after every track and submix, and each nested VCA before a
VCA that lists it, so the request may list them in any order; a membership cycle, a self-member
included, is refused with the engine's code (`vca.cycle`).
A route or sidechain source is `{ kind: "track", trackId, tap }` or
`{ kind: "submix", submixId, tap }`, at any of the seven taps; the retired `submix_output` is
refused. The CLI validates with the packaged engine before publishing
canonical Session V1 JSON. File output preserves existing destinations unless `--overwrite` is
specified; stdout output contains only the document. It does not download or decode stems.
See the [CLI request shape](https://github.com/misofm/engine/blob/main/sdk/src/cli/session-request.ts) and
[Session V1 schema](https://github.com/misofm/engine/blob/main/docs/SESSION_SCHEMA_V1.md).

## Upgrading from 0.10.0

0.10.1 removes no export. One refusal changes its code (see the first item).

- Changed: the engine projects a session's effect state against the memory budget before it
  allocates, on boot, replace and rebuild (issue #2194). A session whose effects cannot fit is
  refused before memory grows, with `host.effect.resource.limit`,
  `host.budget.retained_projection` or `control.resource.replacement_peak`. A `replaceSession`
  whose document cannot be parsed beside the live session rejects with `MisoEngineError`
  `refusedBudget` (diagnostic `host.budget.parse_projection`, whose detail names
  `live_retained_bytes`). In 0.10.0 it rejected with `bufferTooSmall`.
- Changed: in the browser, the live controls an app holds follow a rebuild that adds, removes or
  reorders strips (issue #2189; see "Edit a playing session"). Index addressing of console slots
  and inserts stays valid on each strip whose racks the rebuild did not change, and throws
  `MisoUsageError` on a strip whose racks changed.
- Changed: in the browser, a live-control frame that the host refuses before the core rejects with
  `MisoEngineError`, never with the host's raw `miso.error.v1` record (issue #2193). A host that
  failed is `wrongState` (terminal: the writer drops the batch and throws), not `backpressure`.

## Upgrading from 0.9.0

The browser host's two compiled byte limits are boot options now (issue #2191). A session that
grows no longer meets a fixed cap: the memory budget is the limit.

- Removed: the export `MAXIMUM_DOCUMENT_BYTES` (1 MiB). There is no fixed document maximum: a
  document of `n` bytes needs `n * 20` bytes of the memory budget to parse
  (`ABI_LAYOUT.constants.parseTransientMultiplier`), so the default 512 MiB budget admits about
  25 MiB. Code that read the export for a size check reads its own `maximumDocumentBytes`, or
  lets the engine refuse.
- Added: the boot options `maximumControlFrameBytes` and `maximumDocumentBytes` (`bigint`, in
  `BootOptions`, `OfflineEngineOptions` and the browser `policy`, and on the shipped host's
  `MisoWebBootOptions`, where they replace `reserved1` and `reserved3`). Absent or `0n`, the
  control frame is 128 KiB (`constants.defaultControlFrameBytes`; 16 KiB to 16 MiB is accepted,
  anything else is refused with `web.options.control_frame_bytes`) and the document limit is the
  budget's. A `maximumDocumentBytes` is a tighter cap, for documents from other people.
- Changed: the default control frame is 128 KiB, up from 16 KiB, so one transaction holds a bus
  add over about 385 tracks. The frame is charged to the budget, about 0.6 MB of retained memory at
  the default. `CAPABILITIES_GET` (and the transaction queue's `maximumFrameBytes`) report it.
- Added: `host.controlFrameBytes`, the frame the engine resolved at boot; `host.control()`
  refuses a longer frame.
- Changed: a document refused before staging is a typed error that names its bytes and the
  limit: `web.document.maximum_bytes` (over `maximumDocumentBytes`, `refusedDocument`) or
  `host.budget.parse_projection` (over the budget, `refusedBudget`, which was `refusedDocument`
  for a document over 1 MiB). An oversized control frame is refused with
  `web.control.frame_bytes`, naming both.
- Changed: a transaction frame larger than `capabilities().maximumFrameBytes` is not sent
  (issue #2190). It rejects with `MisoControlRefusal`, `status` and `code` `limit_exceeded`, and
  its diagnostic's `detail` is `frame_bytes=<n>,maximum_frame_bytes=<m>`; nothing is committed.
  In 0.9.0 the browser rejected it with the host's raw `miso.error.v1` record, and the headless
  engine with `MisoEngineError` `bufferTooSmall`. A refusal that the browser host makes before a
  frame reaches the core is now a `MisoEngineError`, never the raw record.

## Upgrading from 0.8.1

0.9.0 adds worker mode. On a cross-origin-isolated page the engine now runs in worker mode by
default; a page without cross-origin isolation keeps single mode and behaves as in 0.8.1, apart
from the bounded dispose and the shared memory below. Nothing is removed.

- Added: worker mode. On a cross-origin-isolated page (COOP `same-origin` and COEP
  `require-corp`), a dedicated control Worker parses, compiles, prepares, edits and disposes the
  session on one shared `WebAssembly.Memory`, and the AudioWorklet only renders. A structural
  edit no longer blocks the audio thread: `blockingRebuilds` and `singleModeControlAllocations`
  stay 0. The PCM feed's rings drain in the control Worker. Qualified on desktop Chromium and
  WebKit; iOS Safari is not qualified.
- Added: the control Worker artifact `miso-engine-v1-control-worker.js`, resolved beside the
  worklet module. The artifact set is now eight files; serve it with the others, same-origin.
- Added: `engine.instanceMode` and `host.instanceMode` (`"worker"` or `"single"`), and the option
  `instanceMode` on `createEngine` and `createMisoAudioWorkletHost`. Absent, the mode is
  `"worker"` exactly when the page is cross-origin isolated. `"single"` keeps single mode on an
  isolated page; `"worker"` on a page without isolation is refused. The `createHost` request
  carries `instanceMode` when the option is given; a custom `createHost` must forward it.
- Added: the status counter `meterFramesSkipped` (the meter frames the worklet skipped in worker
  mode because a plan was adopted within one control-Worker round trip of its commit; always 0 in
  single mode). `staleViewQuanta` is now always 0.
- Added: `BrowserBootError` operation `worker-failed` (the host's `MisoWorkerFailed` record): the
  control Worker failed before its boot reply. The page never falls back to single mode.
- Added: `PcmFeedError` operation `attachRefused`. In worker mode the worklet drains no ring, so
  `attachEngineFeed` needs the feed's `engine`; a feed attached without it on a worker-mode
  engine closes with `attachRefused`. Pass `engine` (as the examples above do).
- Changed: in worker mode the default per-source ring (`sourceRingFrames` 0) is 65 536 frames,
  about 1.37 s at 48 kHz, so the Worker's timer-driven drain rides out a render burst. Single
  mode keeps the engine's default; an explicit `sourceRingFrames` overrides both.
- Changed: a dispose is bounded and always ends in teardown, in both modes. With no dispose
  reply within 5 s the page disconnects the node (and in worker mode terminates the Worker) and
  `host.dispose()` rejects with a `MisoTeardown` record (`tag: "miso.teardown.v1"`, reason
  `context-closed` or `dispose-timeout`; in worker mode `release-timeout` when the worklet did not
  release its render half within 2 s). `engine.close()` rejects with that record and still
  closes the context. In worker mode every later `dispose()` returns the first one's promise and
  a failed dispose is not retried; in single mode a dispose that ended in teardown is not retried
  either.
- Changed: in worker mode each control reply (`engine.apply`, live-control writes and the other
  command replies) waits for one worklet message turn, up to about one quantum, so that no meter
  frame rendered before the reply reaches the page after it. Status reads do not wait.
- Changed: the module imports one shared `WebAssembly.Memory` (`env.memory`, at most 512 MiB,
  sized by the ABI layout's `memory` pages) and re-exports it as `memory`. Code that instantiates
  the shipped module itself must pass `{ env: { memory } }` with
  `new WebAssembly.Memory({ initial, maximum, shared: true })`; empty imports fail. A boot budget
  (`maximumMemoryBytes`) above 512 MiB is refused with `web.boot.memory_maximum`.

## Upgrading from 0.8.0

0.8.1 adds one optional field and removes nothing, but one custom-host case now fails where
0.8.0 worked (see "Changed: a custom `createHost`").

- Fixed: `createEngine({ spectrum })` and `createEngine({ spectrumCollection })` no longer count
  a blocking rebuild at boot. The taps are part of the first plan, so `blockingRebuilds` is 0
  until the session changes. A `configureSpectrum` call while the engine runs still counts one.
- Added: the `createHost` request carries an optional `bootCommand` (a `Uint8Array`) when the
  boot has spectrum taps.
- Changed: a custom `createHost` must pass `bootCommand` to `createMisoAudioWorkletHost`. A host
  that worked with spectrum taps on 0.8.0 but builds its request from named fields now makes
  `createEngine` fail with a `boot` error whose diagnostic is `sdk.boot.command`, instead of
  returning an engine without its taps. Forward the whole request (or add `bootCommand`).
- Changed: a boot spectrum configuration that the engine refuses (for example an unknown target)
  now fails the boot itself. The error's `code` is the boot refusal (`refusedDocument` or
  `refusedBudget`), and its diagnostic is the engine's spectrum code (for example
  `spectrum.target.unknown`). Before 0.8.1 the error's `code` came from the spectrum refusal
  (for example `notFound`). On the headless engine the diagnostic is in `diagnostics`; the
  browser host reports only the result code.

## Upgrading from 0.7.0

0.8.0 changes no SDK API: it adds nothing and removes nothing. The changes are in the engine
module.

- Changed: a bypassed `miso.delay` or `miso.reverb` now does no wet-path work, as a bypassed
  console slot already did in 0.7.0. Un-bypass still starts from rest (dry for the warm-up, then
  fade in; the warm-up of these two effects is zero). No rendered sample moves.
- Changed: the plan places console and insert banks with fewer, fuller banks (pattern packing,
  the contracted executor order, ASAP/ALAP window placement and the leftover insert pass). No
  rendered sample moves.
- Fixed: a table-only plan replacement (no session change) that lands while a stored automation
  ride is in progress now renders the same samples as uninterrupted playback, off the fader grid
  and when the ride coincides with a VCA step. Before 0.8.0 such a replacement could change the
  first samples after it.
- Docs: the PCM feed section above now shows how to attach a feed on a suspended
  `AudioContext`. `attachEngineFeed` and `ready()` work there; you do not need resume, suspend
  and a seek. The behaviour is the same as in 0.7.0.

## Upgrading from 0.6.0

0.7.0 adds to the 0.6.0 API and removes nothing. One behaviour changes: an un-bypass.

- New: `subscribeLoudness()` on both engines. It reads the master's BS.1770-5 momentary,
  short-term and integrated loudness, the loudness range and the true peak (see "Master
  loudness" above). It refuses an engine whose `CAPABILITIES_GET` lacks capability flag bit 17
  (`sdk.asset.capability`).
- New: the engine reverb `miso.reverb`, a 32-line feedback delay network. It is an insert, not a
  console slot. Use it as an aux return: put it on a submix at `mix` 1, send each track to it
  from its `post_fader` tap, and route the submix to the output (see "Aux reverb return" above).
- Changed: an effect that you un-bypass now starts from rest. It holds the dry signal for its
  warm-up (its declared latency), then fades in. Before 0.7.0 a bypassed effect kept running and
  faded back in with the state it had. A bypassed effect now does no wet-path work, so a console
  with bypassed slots uses less CPU.
- Changed: console banks pack their active strips into fewer banks, and a silent strip passes
  its silence downstream without processing. Neither change moves a rendered sample.

## Upgrading from 0.5.0

0.6.0 adds to the 0.5.0 API and removes nothing.

- New: `prepareBlake3()` (`./browser`) loads the WebAssembly SIMD BLAKE3 module before
  `new IncrementalBlake3()`; it resolves `"simd128"` or `"javascript"` and never rejects.
- New: `engine.planLatency()` resolves `{ samples, tailSamples }` on both engines.
- Changed: `blake3Stream` and `IncrementalBlake3` hash in WebAssembly SIMD when the module loads;
  the JavaScript fallback allocates nothing per chunk. The digests do not change.
- Changed: `CanonicalPcmPump` reads windows concurrently (at most eight in flight) and reads one
  window ahead per source.
- Fixed: a seek during a window read no longer commits the pre-seek window at the post-seek frame.
- Unchanged: a window read shorter than its declared range is refused, as before, with
  `RangeError("PCM window is truncated")`, and the pump calls `onError` once.
- The delivery closure has a seventh file, `miso-engine-v1-blake3.simd128.wasm` (in `dist/assets`).
  Custom delivery must serve it, or pass `prepareBlake3({ url })`; without it the hash falls back
  to JavaScript.

## Upgrading from 0.4.3

0.5.0 breaks the 0.4.3 API. Each line is one change; the sections above give the details.

### Removed exports

- Root (`./headless` too for `EngineConsole` and `CommandReport`): `EngineConsole`, `ConsoleWriter`, `CommandReport`, `CommandKindName`,
  `CommandReasonName`, `commandReasonName`, `AbiCommandField`, `encodeLaneEdits`, `LaneEdit`,
  `WriterOptions`, `SemanticWriterOptions`, `ConsoleRack`, `ConsoleChannel`, `ConsoleSubmit`,
  `ConsoleBeforeSubmit`, `ConsoleEdits` and `AutomationRack` are removed.
- `./browser`: `createBrowserConsole` is removed; use `createBrowserLiveControls`.
- Every entry: the observation-record helpers `decodeObservationRows`, `enrichObservationMap`,
  `resolveObservationAddressesWithTracks`, `observationRackValue`, `observationChannelsValue`,
  `ObservationAddress`, `RawObservationRow` and `RawObservationBinding` are removed.
- `Catalog` loses `commandRecordBytes`, `maximumCommandRecords`, `commandKinds` and
  `commandReasons`; `AbiLayout` loses `commandRecord`.

### Live values: console -> liveControls

- `engine.console()` becomes `engine.liveControls()` (`EngineLiveControls`); `ConsoleWriter`
  becomes `LiveControlWriter` (`controls.writer`).
- A report is `ControlReport` (`{ ok, revision, path, appliedOperations }`), not
  `{ appliedAtSample }`; a refusal rejects with `MisoControlRefusal`.
- An effect is addressed by its slot ID: `track.console(slot, effectId)` or
  `track.insert(slot, effectId)`, not `track.effect("simd1", index, effectId)`.
- `submitCommands()` is removed. Send value edits as `SET_CONTROL` transactions
  (`liveControls().submit`, or headless `controlFrame` and `controlTransactions()`), and structural
  edits with `engine.apply(transaction(base)...build())` or `engine.replaceSession(document)`.
- `EffectEdits.observe` is removed; use `subscribeObservations()`.

### Observation: records -> handles and callbacks

- A `subscribeObservations()` handle and its receipt report `armed`, not `appliedAtSample`.
- Pass `onEnd` to learn which selections a commit ended (`handle.ended`); a subscription with no
  selection left closes.
- The subscription bounds lose `maximumCommandRecords`, and bounds and limits lose
  `maximumBindings` and `maximumWindowBlocks`.

### Session types

- `TrackSpec` `simd1`, `dynamic` and `simd2` become `console` entries (one for each session slot)
  and `inserts`; the session declares its slots with `console({ preInsert, postInsert })`.
- `Rack` and `ObservationRack` are `"console" | "inserts"`, not `"simd1" | "dynamic" | "simd2"`.
- A console slot declares its `linkMode` (`dual_mono`, `maximum` or `average`); a strip entry can
  override it, and `slot` (the default) takes the slot's mode.
- Send taps: `post_input_builtins` -> `post_input`, `post_simd1` -> `insert_send`,
  `post_dynamic` -> `insert_return`, `post_simd2_pre_fader` -> `pre_fader`,
  `post_matrix` -> `post_pan`.
- The route source `{ kind: "submix_output", submixId }` becomes `{ kind: "submix", submixId, tap }`.
- `AutomationTarget` is a control address (`owner`, `section`, `control`, `lane`), not
  `{ trackId, rack, slotId, parameter, channel }`.
- Spectrum targets: `trackPostInputBuiltins` -> `trackPostInput`, `trackPostMatrix` -> `trackPostPan`.

### Boot options

- `console: { commandQueueRecords, meterBlocks, observationTaps, masterTrackPlusOne }` becomes
  `liveControls: { meterBlocks }` (in `BootOptions`, `OfflineEngineOptions` and the browser
  `policy`).
- The command-queue, master-track and observation-taps boot words are retired: `reserved1`,
  `reserved2` and `reserved3`, always zero. Every plan binds every declared observation tap.
  (Since 0.10.0 the offsets of `reserved1` and `reserved3` hold `maximumControlFrameBytes` and
  `maximumDocumentBytes`.)
- `liveControls.observationTaps` is refused with `MisoUsageError`, never ignored.

### Behaviour and license

- A track or submix built without `pan` now gets the identity matrix, not pan 0 on each lane, so a
  stereo source keeps its image. A live `pan(...)` on such a strip needs `matrixMode("pan")` in the
  same transaction.
- The license changes from Apache-2.0 to FSL-1.1-ALv2.

### New APIs

- Live controls and transactions: `engine.liveControls()`, `engine.controls()`,
  `engine.controlState()`, `transaction()`, `engine.apply()`, `engine.replaceSession()`,
  `engine.watermark()`, `engine.applied()`, and the generated `CONTROLS` catalog.
- Session: submix strips, `vcas`, `controlSmoothing`, route `mute` and `followsMute`, automation
  `offset`.
- Browser playback: `createEngine({ pcmFeed })`, `engine.position()`, `engine.play()`,
  `engine.stop()` and `engine.seek()`.
- Local WAV stems (`./browser`): `readWavHeader`, `CanonicalPcmPump` and `blake3Stream`.
- Protocol reads: `capabilities()`, `committedDocument()`, `transport()`, `setTransport()`,
  `counters()`, `diagnostics()`, `configureTelemetry()` and `onEvent()`.

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
for realtime, DSP, and contribution requirements. Licensed under FSL-1.1-ALv2.
