# `@misofm/engine`

The TypeScript SDK for Engine V1: a Session V1 builder, a boot-v1 host for Node and the browser,
and an agent-facing parameter surface that speaks decimals and ranks rather than floats.

## Installing the package

```sh
npm install @misofm/engine
```

The published package is a self-contained Engine V1 release: compiled ESM, declarations, the
simd128 Wasm engine, the pinned FLAC decoder Wasm and loader, the AudioWorklet modules, parameter
metadata, ABI layout, and a manifest with
the byte length and SHA-256 of every artifact. A Node or Bun headless consumer needs neither a Rust
toolchain nor a separate engine download. Browser consumers receive package-relative artifact URLs,
so the host and Wasm cannot silently come from different releases.

`npm run build` prepares `dist/`; `npm run check:package` additionally packs it, imports every
public entry from a fresh extraction, boots the embedded Wasm, renders one quantum, and proves a
one-byte Wasm mutation is rejected by the manifest digest before compilation.

The four entry points are the only supported import sites:

| specifier | file | for |
| --- | --- | --- |
| `@misofm/engine` | `src/index.ts` | catalog, Session V1 builder, agent surface, `ConsoleWriter` |
| `@misofm/engine/headless` | `src/headless/index.ts` | the Node/Bun offline engine |
| `@misofm/engine/browser` | `src/browser/index.ts` | the Worker scratch boot, policy, host mirror |
| `@misofm/engine/assets` | `src/assets.ts` | URLs and names for the embedded release artifacts |

Import through those barrels; do not deep-import `src/core/*`. Every symbol that previously had to
be reached by a deep path is on a barrel as of #278, and `sdk/test/barrel-surface.ts` fails
compilation if one stops being reachable or starts resolving to a different declaration.

## `enginectl session build`

The package installs `enginectl`, a Node 20+ machine interface for producing one canonical,
engine-accepted Session V1 file. It can consume either a bounded JSON request for fully authored
sessions or a leaf directory of local FLAC stems:

```sh
enginectl session build --request request.json --output session.toml
enginectl session build --request - --output - < request.json
enginectl session build --stems ./song-stems --output song.session.toml
```

`--stems` examines only the directory's directly owned regular `.flac` files and refuses nested
directories, symlinks, non-FLAC entries, empty leaves, and mixed or unsupported sample rates. It
does not recurse: a parent containing several song directories is a collection, and its typed
`stems.collection` refusal reports the sorted leaf names so an agent can issue one call per song.
Each FLAC becomes one source and one unity-routed track feeding the single `main` output. Mono is
duplicated across the track's two lanes; stereo maps channels 0 and 1. Filenames determine stable
IDs only—they never imply gain, pan, effects, categories, or any other audio decision.

The session ID defaults deterministically from the leaf directory name and the quantum defaults to
128. `--session-id` and `--quantum-frames` override those values only in stems mode. Metadata and
canonical PCM identities come from the packaged decoder: identities are SHA-256 over verified,
source-depth, interleaved PCM rather than over FLAC transport bytes. The command compiles one
decoder instance, reuses it across the sorted files, reads each FLAC once, and never retains the
complete decoded PCM.

The request has `schemaVersion: 1`, a required `session` object, and optional `sources`, `tracks`,
`submixes`, `outputs`, `routes`, and `automation` arrays. Sources and tracks are `{ id, spec }`;
rack entries are `{ effectId, parameters?, options? }`. Automation sample positions are canonical
unsigned decimal strings such as `"480"`, never JSON numbers. Unknown structural keys are refused.
The complete input is limited to 4 MiB and must be valid UTF-8 JSON.

`--output -` writes exactly the canonical TOML, including its final LF. A path writes a
same-directory temporary file and publishes it atomically only after the embedded Wasm engine has
accepted the document; stdout then receives one compact JSON receipt with the caller's path, byte
count, and SHA-256. Existing files, directories, and symlinks are preserved by default.
`--overwrite` authorizes replacement of one filesystem destination and is invalid with stdout.

The executable is always non-interactive: it never prompts, pages, opens another program, reads
configuration or credentials, loads plugins, invokes media subprocesses, emits telemetry, or uses
the network. Machine
failures leave stdout empty and write one compact JSON document to stderr. Exit status is `2` for
command/flag usage, `3` for request, stem discovery, FLAC, or builder refusal, `4` for
embedded-engine refusal, `5` for output refusal, and `70` for an unexpected internal or packaged-
asset failure. Success, help, and version use `0`.

### Pinning the embedded engine

A package release is pinned at both ends:

- **Source provenance.** The package build runs `scripts/check-sdk-generated.sh`: it re-derives
  `sdk/assets/*.json` from the Rust and `sdk/src/generated/*.ts` from those assets, and compares
  byte for byte. `PROVENANCE` carries the ABI version, schema IDs, and expected artifact set.
- **Artifact provenance.** The package build copies the artifacts produced by
  `scripts/build-web-audioworklet.sh` and writes `miso-engine-v1-sdk-manifest.json` beside them.
  `loadBundledEngineAsset()` checks the Wasm's byte count and SHA-256 against that manifest before
  compilation, then checks the module's exported ABI word before boot.

Explicit `MisoEngineAsset.load(bytes, sha256)` remains available for deployments whose release
manifest or content store is the artifact authority.

## What this package is not

It is not a second implementation of anything the engine already decides.

There is **no TOML parser** here and there never will be (ruling 5438024085). `validate()` boots the
real engine and throws the result away, so its diagnostics *are* the engine's diagnostics and its
budget checks are the real ones under the real physics gate. A grammar written twice is a grammar
that disagrees with itself eventually, and the disagreement is always discovered in production.

There is **no hand-written ABI table**. Every structure offset, result code, command reason, buffer
kind and export name is read by name out of `src/generated/abi.ts`, which is transcribed from
`miso-engine-v1-abi-layout.json`, which the engine emits from its own Rust `offset_of!`. Issue
#207's review found *five* independent hand-written copies of the boot configuration table in the
old code; one of them wrote a 192-byte struct's offsets into a 64-byte buffer and produced garbage
in silence, because a wrong offset is still a valid address.

There is **no guessing**. The predecessor to this package sniffed a document's sample rate with a
regex over its text, could not see a quoted key, and silently fell back to 48 kHz and 128 frames
when it failed — then fabricated a source ring of 1024 frames, which is not a multiple of a
127-frame quantum, so a 96 kHz session was not merely mis-shaped but unbootable. Boot v1 removed the
need for all of it: hand the engine bytes, and ask it what it compiled.

## The transcription chain

```
Rust structures and frozen constants
  └─ parameter-metadata          offset_of!, registry walk
       └─ sdk/assets/*.json                   checked in
            └─ sdk/src/generated/*.ts         checked in
                 └─ the SDK's public types
```

`scripts/check-sdk-generated.sh` re-derives **both** arrows and compares byte for byte. Checking
only the TypeScript would let a stale asset regenerate stale modules consistently; checking only the
JSON would let a hand-edited constant sit in the modules consumers actually import.

Refresh with `npm run assets && npm run codegen` (needs `cargo`).

## Headless

```ts
import { createOfflineEngine, loadBundledEngineAsset } from "@misofm/engine/headless";

// With no asset option, the package's embedded Wasm is verified and loaded automatically.
const engine = await createOfflineEngine(document);

// For several sessions, compile once and inject the shared asset:
const asset = await loadBundledEngineAsset();
const another = await createOfflineEngine(anotherDocument, { asset });
const shape = engine.shape();   // the ENGINE's answer: rate, quantum, ring, sources, tracks
const console = engine.console();

// Names and values are catalog-derived; the SDK resolves track/parameter IDs.
const vocal = console.edit.track("vocal");
await console.submit(
  vocal.faderDb(-3, { channel: "both" }),
  vocal.effect("simd1", 0, "miso.compressor")
    .parameter("threshold", -24, { channel: "both" }),
);

for (let block = 0; block < blocks; block += 1) {
  for (const source of shape.sources) {
    engine.submitSource({ sourceId: source.id, generation: 1n, startFrame, planes, endOfRegion });
  }
  const { left, right } = engine.render();
}

engine.loadSession(anotherDocument);   // the mix switch: dispose and restage, same instance
engine.dispose();
```

`shape()` is the whole point of boot v1. Nothing in it was parsed out of the document's text, so a
consumer that reads its rate from it cannot be told 48000 by a fallback that never looked.

`BootOptions` is five optional keys over the engine's own 64-byte block. Absent means zero means
*the engine's* default — in particular `maximumMemoryBytes` absent selects the engine's named
`DEFAULT_MAXIMUM_MEMORY_BYTES`, which this SDK documents and never restates.

## Browser

```ts
import { createEngine, scratchBootInWorker } from "@misofm/engine/browser";
```

The order matters, and each step exists because the next is expensive to undo: refuse what web
delivery does not carry → scratch-boot the document in a Worker to learn its shape → construct,
verify and if necessary close-and-retry the `AudioContext` → refuse a quantum mismatch *before*
`addModule` → boot the worklet with the physical shape required as a backstop.

Both boots read the same policy object, so the ring, the memory budget and all four console words
are identical by construction; the two `require_*` words are role-defined (zero in the scratch boot,
physical in the worklet). Source plumbing is bring-your-own: SDK core has no opinions about audio
plumbing — no OPFS, no fetch, no Workers — so the Worker boot and the context constructor are
injected rather than reached for.

`await engine.console()` binds the same semantic console shown above to the shipped browser host.
It resolves the browser session map once, then submits the same whole-batch edits over MessagePort.
All eleven live command kinds are available without numeric rack, channel, parameter, or tap IDs;
the browser and headless acknowledgements carry the same generated result/reason names and exact
`appliedAtSample`.

Call `await engine.close()` when the browser session is finished. It disposes the worklet host
before closing its `AudioContext`, is safe to call repeatedly, and still closes the context if the
host's MessagePort has already failed.

## Agents

```ts
import { catalog, parameter } from "@misofm/engine";

const gain = parameter("miso.parametric-eq", "band-1-gain");
gain.set("-6.3");        // canonical decimal in, canonical decimal out
gain.step("md", +2);     // two medium ladder steps: exactly `ladder.md * 2` ranks
gain.value;              // "-5.3", never "-5.300000190734863"
gain.index;              // the rank -- what a persisted edit carries on the wire
```

Every value is a decimal string and every edit is an integer rank. A value that went through an
`f32` and came back would read `0.30000001192092896` where the agent asked for `0.3`: nothing errors,
the audio is imperceptibly different, and the agent's next comparison against its own request fails
for a reason it cannot see.

Membership is decided in exact decimal arithmetic on the **text**, so every spelling of one number —
`0.3`, `0.30`, `3e-1`, `+0.300` — is the same point, and an off-lattice value comes back with the two
points that *bracket* it rather than with a bare refusal. `step` clamps at the endpoints because a
gesture past the top of a dial lands on the top; `setSteps` refuses an out-of-range rank because an
index is an address rather than a gesture.

The SDK generates lattice points rather than shipping them — a one-cent lattice from 20 Hz to 20 kHz
has about twelve thousand — and is held to the engine's own resolver point for point by
`parameter_metadata_lattice_oracle` over the entire shipped catalog.

`decimalToFloat32` is the single site where a lattice value stops being a decimal, mirroring
`effect_contract::decimal_to_f32`. Auditing that boundary is one grep.

## The writer

```ts
import { ConsoleWriter } from "@misofm/engine";
```

`ConsoleWriter` batches live-console edits against the engine's bounded per-track queue. Its
contract is two sentences:

- **A flow-control refusal is never an error and never terminal.** Nothing throws on backpressure,
  and the writer is as usable after a refusal as before it.
- **Re-staging is latest-wins coalesced.** Pending edits are keyed by what they address, so a
  refused batch is never replayed as a queue's worth of stale intermediates. After the drain, what
  lands is where the hand actually is.

A refusal that is *not* flow control throws instead: backpressure succeeds on retry once the render
thread drains, an unknown address never will, and retrying it silently would be an infinite loop
wearing the costume of resilience.

`submit` may answer synchronously or with a `Promise` — in-process the engine answers immediately,
but a browser host reaches it over a worklet port, where the answer is a promise by construction —
so `flush()` and `drain()` are async. Flushes serialize: a call entered while a prior submit is
still outstanding waits for it rather than picking its batch out of a map the earlier flush has not
yet applied to. The contract is otherwise identical on both paths, which the evals hold by running
one episode through a sync and an async submit and comparing the transcripts element for element.

`ConsoleWriter` is the expert-level, coalescing seam over already addressed wire edits. New code
should normally start with `engine.console()`: its `edit.track(id)` builder provides typed strip,
effect, and observation operations, locally checks generated domains, and makes one
`console.submit(...edits)` one atomic engine transaction. `ConsoleWriter` remains useful for a
high-rate gesture loop whose pending values need latest-wins coalescing.

## Tests

The eval suites run under Node's native type stripping, so `sdk/src/**/*.ts` is imported directly
with no build step and no `node_modules`:

```sh
bash scripts/check-sdk-headless.sh              # builds the artifact, then runs every eval
bash scripts/check-sdk-generated.sh             # the transcription chain, both arrows
bash scripts/check-sdk-types.sh                 # needs `npm ci` in sdk/ once
```

Two files under `sdk/test/` run no assertions: they are checked by `tsc`, and their job is to fail
*compilation*. `host-mirror.ts` fails when the shipped `.d.ts` and the SDK's adapter disagree.
`barrel-surface.ts` fails when a symbol stops being barrel-reachable, or when a barrel starts
re-exporting a different declaration than the module it names -- including the one collision the
root barrel has to resolve by hand, where `generated/catalog.ts` and `core/lattice.ts` both spell
`StepDeclaration` and `StepSizeName` and mean two different types.
