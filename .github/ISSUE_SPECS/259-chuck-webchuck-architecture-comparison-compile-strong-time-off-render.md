# 259 ChucK/WebChucK architecture comparison: compile strong-time semantics off render

One-line summary: ChucK's strong-time model is useful authoring evidence, but its per-sample shred VM,
dynamic UGen graph and browser memory/filesystem behavior violate V2's bounded renderer; any lesson
must compile off-render into canonical absolute-sample events and prepared replacements.

**This is a completed research and decision record, not implementation authority.**

**Authority: GitHub issue #259.** This local file mirrors its source-backed decision record.

## Authority, pins and method

- Engine V2: [`90c3b9a598f1244938d9cdcce04c4a4641c6b758`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758).
- ChucK: [`3eaa05edf0f893c5ab191cc8cb70211d279b58ae`](https://github.com/ccrma/chuck/tree/3eaa05edf0f893c5ab191cc8cb70211d279b58ae).
- WebChucK: [`94c8707d02d7f48c0d3a90e1042202e184b87887`](https://github.com/ccrma/webchuck/tree/94c8707d02d7f48c0d3a90e1042202e184b87887).
- Source was inspected without builds, benchmarks or legacy Miso access.

## Findings

ChucK shares a compiler, VM and synthesis core across native and Wasm targets
([README](https://github.com/ccrma/chuck/blob/3eaa05edf0f893c5ab191cc8cb70211d279b58ae/README.md#L74-L127)),
which establishes source reuse rather than numeric identity. Language/time values are double, audio
samples default to float, and integer width differs under Emscripten
([types](https://github.com/ccrma/chuck/blob/3eaa05edf0f893c5ab191cc8cb70211d279b58ae/src/core/chuck_def.h#L58-L90)).
No bit-exact native/Wasm corpus was found.

At every sample the VM repeatedly runs ready shreds, broadcasts events and processes messages until
no work remains ([VM](https://github.com/ccrma/chuck/blob/3eaa05edf0f893c5ab191cc8cb70211d279b58ae/src/core/chuck_vm.cpp#L560-L651)).
Equal-time scheduling is stable by insertion order
([queue](https://github.com/ccrma/chuck/blob/3eaa05edf0f893c5ab191cc8cb70211d279b58ae/src/core/chuck_vm.cpp#L2691-L2782)),
but asynchronous producer arrival is not a reproducible semantic key. `spork` creates shreds and UGen
edges can allocate/grow dynamically
([spork](https://github.com/ccrma/chuck/blob/3eaa05edf0f893c5ab191cc8cb70211d279b58ae/src/core/chuck_vm.cpp#L1219-L1273),
[UGen graph](https://github.com/ccrma/chuck/blob/3eaa05edf0f893c5ab191cc8cb70211d279b58ae/src/core/chuck_ugen.cpp#L49-L130)).

WebChucK preloads files/plugins and the processor can create filesystem files, compile/replace code
and accommodate Wasm memory growth while copying audio
([WebChucK loader](https://github.com/ccrma/webchuck/blob/94c8707d02d7f48c0d3a90e1042202e184b87887/src/Chuck.ts#L57-L102),
[processor](https://github.com/ccrma/chuck/blob/3eaa05edf0f893c5ab191cc8cb70211d279b58ae/src/host-web/chucknode-postjs.js#L657-L760)).
V2 instead treats memory growth as a sticky host fault and keeps rendering within fixed prepared
storage ([worklet](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js#L754-L866)).

## Decision

- **Investigate conceptually:** a restricted strong-time authoring/control language compiled on a
  worker into V2 absolute-sample events and immutable plan replacements. Same-time order must be
  `(absolute_sample, stable_transaction_sequence)`, never arrival order.
- **Preserve:** fixed graph plans, bounded atomic admission, typed backpressure, deterministic
  ordering, fixed Wasm memory and canonical session/effect snapshots.
- **Reject:** a shred VM in render, zero-time unbounded execution, callback `spork`, dynamic UGen
  rewiring, worklet filesystem/plugin loading, whole-file preload as streaming, or tolerated Wasm
  growth.

## Gates for any future strong-time front end

1. Compilation is off-render and outputs only capped events and prepared replacements.
2. Same-time conflicts have a canonical sequence independent of thread/arrival scheduling.
3. Events per block, zero-time recursion, memory and replacement work are admitted before ack; an ack
   can never precede a later drop.
4. Native/Wasm `to_bits` corpora cover collisions, replacements, chunk partitions and explicit RNG
   seeds.
5. Snapshots contain V2 semantic state, never an opaque VM heap; render performs no compilation,
   allocation, lock, I/O or structural mutation.

## Limitation

Strong-time syntax may improve agent authoring ergonomics, but V2 already has the necessary runtime
sample-time representation. Without validated user demand, a language front end adds complexity with
no core-engine gain.

## Closure

Closed as completed research and superseded by final synthesis #268. No engine VM/language work is
authorized; an external compiler remains demand-gated.
