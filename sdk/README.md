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
npm install @misofm/engine@0.4.3
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
its own `bypass` and parameters, plus its own `inserts`. A console slot is one of the six effects
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
not support (`EFFECT_LINK_MODES`: the EQ, soft-clip and delay are `dual_mono` only, and the limiter
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

For shared-ring delivery, the browser entry exposes `prepareEngineFeed`, `attachEngineFeed`,
`Msb1RingWriter`, and `waitForPcmRunway`. These transport already-decoded PCM; they do not fetch or
decode media. The shared-ring path requires `SharedArrayBuffer` and a cross-origin-isolated page.
The host also exposes bounded source submission. Source underruns produce silence and counters.

Pass `engine` to `attachEngineFeed` to feed the sources that an edit adds or removes while audio
plays:

- **Add a stem.** After `engine.apply(tx)` (or `replaceSession`) resolves, attach a feed for the new
  source with `start: { frameAt, prime }`. `feed.ready()` reads the engine's next render sample
  once and sets the anchor `A` to that sample plus the ring's capacity in frames, rounded up to a
  quantum. It calls `frameAt(A)` once (the engine has no playhead: the source frame at `A` is your
  mapping), publishes the anchored seek in the ring, awaits `prime({ generation, frame,
  anchorSample })`, and then sends the seek to the engine. Write the stem's PCM from `frame` at
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
refuses a ring with another header offset.

Browser execution requires WebAssembly SIMD, Workers, and AudioWorklet in a secure context; there
is no shipped scalar fallback. Supported session rates are exactly **44.1, 48, 88.2, and 96 kHz**.
The device/context must accept the session rate and quantum (normally 128 frames); mismatches are
refused. Browser source delivery supports declared 16- and 24-bit sources at launch, decoded to
`f32` PCM. Declared `32f` sources remain outside browser launch delivery; raw-document consumers
must enforce that restriction at their ingest boundary.

The npm hosts both execute Wasm. Native and mobile applications must integrate the separate
Rust/C-ABI host layer and platform audio callbacks; this package supplies no native binaries or
mobile audio-device adapter.

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

On a page without cross-origin isolation, the worklet's message handler prepares a `rebuild` on
the audio thread. The audio thread is blocked while it prepares, and
`(await engine.host.status()).blockingRebuilds` counts each blocking rebuild.

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
never an error. The root entry's `parameter()` helper provides exact decimal/lattice edits for
agent-facing controls.

Observation subscriptions (`console(...)` or `insert(...)` then `.observe(...)`) are the one edit that
is still a command record: it names its tap by stable key (the strip's ID, the console slot's or
insert's ID, the tap's ID), which the core resolves on the newest committed plan and refuses with a
typed reason when it has no such tap. They resolve with a `CommandReport` and need
`liveControls.observationTaps` at boot, passed to `createOfflineEngine` directly or under
`policy` to `createEngine`. The cap must be at least the largest tap count of any effect in the
session (4 for `miso.dynamic-eq`, one tap per band): a lower positive cap refuses the boot with
`effect.observation.taps`, even with no subscription. Value edits are `SET_CONTROL` transactions and need no boot word.

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
nothing changes. A named row stays bound to the effect it was resolved for: after a rebuild, call
`engine.liveControls()` again so that names resolve against the new list. A numeric address names
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
| Resident effect values | `observationMap()`, `readObservations()`, or `subscribeObservations()`. Select stable track/rack/effect-slot/tap IDs. Ready values retain the effect owner's own window and native unit. A managed subscription follows a rebuild (the core keeps the tap armed on the effect that carries on). When a commit removes the effect, the core ends the tap with `OBSERVATION_ENDED`: pass `onEnd` to `subscribeObservations()` to be told which selections ended and which remain, once for each commit; `handle.ended` lists them, and a subscription left with no selection is closed. |
| Live EQ/filter response | `queryTrackResponse()` or `subscribeTrackResponse()`. `capturedSample` marks capture of applied target state at a block boundary. The curve is a stationary EQ/input-filter subtotal; fader, pan, routing, and unsupported processors are excluded. |
| Spectrum | Browser `querySpectrum()`; headless `armSpectrum()`, explicit renders, then `readSpectrum()`. Each result covers 2,048 captured PCM samples, `[capturedSample, endSample)`, with dBFS bins and a graph-wide source-underrun flag. |
| Continuous spectrum | `subscribeSpectrum()` retains one managed stream. Metadata carries the capture span, hop, epochs, drops, and smoothing history. The default 100 ms power smoothing includes earlier windows; set `smoothingMs: 0` for unsmoothed analysis. |
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
render has drained the first. An identical second subscribe, and a close of a binding that other
handles still share, send no record and need no render. In a headless engine the refused
operation rejects at once: render, then retry. In a browser engine it waits for a render and sends
again, up to `requestDeadlineMs` and at most 8 resends (one render drains the tap, so more means
another writer keeps refilling it), and then ends in typed `backpressure`. Prepare `liveControls.observationTaps` for resident effect taps: at least the largest tap
count of any effect in the session (4 for `miso.dynamic-eq`): a lower positive cap refuses the
boot with `effect.observation.taps`, even with no subscription.

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
`miso-engine-v1-sdk-manifest.json` records byte counts and SHA-256 digests for the six engine
artifacts, including Wasm, host, processor, host declaration, metadata, and ABI layout.
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
