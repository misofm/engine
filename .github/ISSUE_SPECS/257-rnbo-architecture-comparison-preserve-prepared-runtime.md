# 257 RNBO architecture comparison: preserve V2's prepared runtime, sample-time events and streaming assets

One-line summary: RNBO's export ecosystem is effective product tooling, but its public adapter and
host contracts do not improve V2's core numerical, realtime, state or asset architecture; retain at
most an explicit strict-export-profile lesson for future programmable effects.

**This is a completed research and decision record, not implementation authority.**

**Authority: GitHub issue #257.** This local file mirrors its source-backed decision record.

## Authority, pins and method

- Engine V2: [`90c3b9a598f1244938d9cdcce04c4a4641c6b758`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758).
- RNBO bare-metal example: [`4aef0bf7835c7512da617a6a5306d14937d632bd`](https://github.com/Cycling74/rnbo.example.baremetal/tree/4aef0bf7835c7512da617a6a5306d14937d632bd).
- RNBO Wasm adapter: [`951384b7ae5e6c2b723a9a92e91c3f51077bd8c4`](https://github.com/Cycling74/rnbo.adapter.wasm/tree/951384b7ae5e6c2b723a9a92e91c3f51077bd8c4).
- Official RNBO architecture, export, state, event and dependency documentation was retrieved
  2026-08-29. RNBO's compiler/core are proprietary; this record makes no claim about their internal
  IR, optimizer, SIMD lowering or parity. No legacy Miso source or cross-project benchmark was used.

## Findings

RNBO's official architecture generates C++ from a Max graph, then compiles that output for targets
including Wasm and plugins ([architecture](https://rnbo.cycling74.com/learn/architecture),
[export overview](https://rnbo.cycling74.com/learn/export-targets-overview)). Generated C++ is a
pragmatic deployment seam, not public evidence of a typed backend-neutral semantic contract. V2
should not make C++ source its canonical effect identity.

The bare-metal example disables exceptions, installs a custom allocator, prepares a generated object
and can use fixed lists, but its comments still account for buffer/list/configuration allocations
([main.cpp](https://github.com/Cycling74/rnbo.example.baremetal/blob/4aef0bf7835c7512da617a6a5306d14937d632bd/main.cpp#L9-L255),
[minimal export](https://rnbo.cycling74.com/learn/minimal-export)). This is a host recipe, not a
universal callback audit. V2's immutable prepared rate/quantum/topology and transactional plan
replacement remain stricter.

The public Wasm adapter grows/reallocates channel buffers at preparation, allocates binding/preset
values and enables Emscripten memory growth
([adapter](https://github.com/Cycling74/rnbo.adapter.wasm/blob/951384b7ae5e6c2b723a9a92e91c3f51077bd8c4/RNBO_WebAssembly.cpp#L192-L225),
[CMake](https://github.com/Cycling74/rnbo.adapter.wasm/blob/951384b7ae5e6c2b723a9a92e91c3f51077bd8c4/CMakeLists.txt#L6-L42)).
That does not prove allocation in a correctly used RNBO callback, but it is unsuitable as V2's
fixed-memory browser boundary.

RNBO presets are user state, with official warnings about synchronous capture blocking processing;
events are scheduled in millisecond time
([presets](https://rnbo.cycling74.com/learn/working-with-presets-cpp),
[CoreObject](https://rnbo.cycling74.com/cpp/ref/classes/core_object)). V2's absolute sample-time
events and provenance-bearing effect-state envelopes are the more precise replay and continuation
contracts. RNBO dependencies are host-supplied decoded data, and the official example loads full
files ([dependencies](https://rnbo.cycling74.com/learn/loading-file-dependencies)). This is not proof
RNBO forbids streaming, but it supplies no improvement over V2's bounded SPSC source rings.

No public official RNBO contract was found for FMA contraction, transcendental implementation,
generated SIMD or strict native/Wasm numeric parity. Those remain unknown and cannot be evidence for
relaxing V2's built-in bit determinism.

## Decision

- **Adopt:** nothing at core-architecture level.
- **Investigate only with programmable-effect scope:** present a strict embedded export profile with
  fixed rate/quantum, bounded state/scratch, no dynamic lists/imports/memory growth and exact
  sample-time events. This is product ergonomics around the Onda-derived artifact seam, not a new
  runtime design.
- **Preserve:** fixed prepared plans; shared native/browser session compiler; stable browser memory;
  absolute sample-time events; typed effect-state provenance; duration-independent source streaming;
  and V2's deterministic numeric policy.
- **Reject:** generated C++ as canonical semantic interchange, mandatory cloud compilation, runtime
  memory growth in the realtime core, millisecond engine events, presets as portable DSP
  continuation, and whole decoded stems as the standard asset model.

## Gate for any future strict export profile

Statically reject imports/WASI, `memory.grow`, dynamic-list expansion, rate or quantum mutation,
unbounded queues, and any allocator reachable from `process`. Seal descriptor, artifact, target and
numeric-profile lineage; pass V2's existing render audit and exact sample-time fixtures. This gate is
qualification for V2 artifacts, not a claim about RNBO internals.

## Limitation

Only public adapters, examples and official documentation were inspectable. RNBO's proprietary core
may have stronger private contracts; the honest conclusion is “not established,” not “absent.”
