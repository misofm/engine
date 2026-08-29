# `@misofm/engine`

The TypeScript SDK for Engine V2: a Session V1 builder, a boot-v2 host for Node and the browser,
and an agent-facing parameter surface that speaks decimals and ranks rather than floats.

## What this package is not

It is not a second implementation of anything the engine already decides.

There is **no TOML parser** here and there never will be (ruling 5438024085). `validate()` boots the
real engine and throws the result away, so its diagnostics *are* the engine's diagnostics and its
budget checks are the real ones under the real physics gate. A grammar written twice is a grammar
that disagrees with itself eventually, and the disagreement is always discovered in production.

There is **no hand-written ABI table**. Every structure offset, result code, command reason, buffer
kind and export name is read by name out of `src/generated/abi.ts`, which is transcribed from
`miso-engine-v2-abi-layout.json`, which the engine emits from its own Rust `offset_of!`. Issue
#207's review found *five* independent hand-written copies of the boot configuration table in the
old code; one of them wrote a 192-byte struct's offsets into a 64-byte buffer and produced garbage
in silence, because a wrong offset is still a valid address.

There is **no guessing**. The predecessor to this package sniffed a document's sample rate with a
regex over its text, could not see a quoted key, and silently fell back to 48 kHz and 128 frames
when it failed — then fabricated a source ring of 1024 frames, which is not a multiple of a
127-frame quantum, so a 96 kHz session was not merely mis-shaped but unbootable. Boot v2 removed the
need for all of it: hand the engine bytes, and ask it what it compiled.

## The transcription chain

```
Rust structures and frozen constants
  └─ miso-engine-parameter-metadata          offset_of!, registry walk
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
import { MisoEngineAsset, createOfflineEngine } from "@misofm/engine/headless";

// One compile per SDK lifetime. An asset serves any number of engines.
const asset = await MisoEngineAsset.load(wasmBytes, releaseManifest.sha256);

const engine = await createOfflineEngine(document, { asset });
const shape = engine.shape();   // the ENGINE's answer: rate, quantum, ring, sources, tracks

for (let block = 0; block < blocks; block += 1) {
  for (const source of shape.sources) {
    engine.submitSource({ sourceId: source.id, generation: 1n, startFrame, planes, endOfRegion });
  }
  const { left, right } = engine.render();
}

engine.loadSession(anotherDocument);   // the mix switch: dispose and restage, same instance
engine.dispose();
```

`shape()` is the whole point of boot v2. Nothing in it was parsed out of the document's text, so a
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
`miso_engine_parameter_metadata_lattice_oracle` over the entire shipped catalog.

`decimalToFloat32` is the single site where a lattice value stops being a decimal, mirroring
`miso_engine_effect_contract::decimal_to_f32`. Auditing that boundary is one grep.

## The writer

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

## Tests

The eval suites run under Node's native type stripping, so `sdk/src/**/*.ts` is imported directly
with no build step and no `node_modules`:

```sh
bash scripts/check-sdk-headless.sh              # builds the artifact, then runs every eval
bash scripts/check-sdk-generated.sh             # the transcription chain, both arrows
bash scripts/check-sdk-types.sh                 # needs `npm ci` in sdk/ once
```

`sdk/test/host-mirror.ts` runs no assertions: it is checked by `tsc` and its job is to fail
*compilation* if the shipped `.d.ts` and the SDK's adapter ever disagree.
