# 258 Csound architecture comparison: consider an offline importer, not a render VM

One-line summary: Csound demonstrates the portability of an executable music document, but its live
VM, dynamic opcodes, render-time compile/eval and temp-file assets conflict with V2's bounded prepared
renderer; consider only a transactionally compiled offline importer if product demand appears.

**This is a completed research and decision record, not implementation authority.**

**Authority: GitHub issue #258.** This local file mirrors its source-backed decision record.

## Authority, pins and method

- Engine V2: [`90c3b9a598f1244938d9cdcce04c4a4641c6b758`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758).
- Csound: [`609072449df7cce6b1b8f0ba4efc36f97441b52b`](https://github.com/csound/csound/tree/609072449df7cce6b1b8f0ba4efc36f97441b52b).
- Source was inspected but neither project was built or benchmarked. No legacy Miso source was read.

## Findings

Csound accepts CSD files/strings and can compile or replace instruments while performance is active
([API](https://github.com/csound/csound/blob/609072449df7cce6b1b8f0ba4efc36f97441b52b/include/csound.h#L674-L743)).
Its performance loop senses events every control period
([csound_perf.c](https://github.com/csound/csound/blob/609072449df7cce6b1b8f0ba4efc36f97441b52b/Top/csound_perf.c#L412-L449)),
while compile/eval opcodes can allocate, read files and invoke compilation from executing orchestra
code ([compile_ops.c](https://github.com/csound/csound/blob/609072449df7cce6b1b8f0ba4efc36f97441b52b/OOps/compile_ops.c#L27-L120)).
Line-event input can reallocate buffers and report lost input on overflow
([linevent.c](https://github.com/csound/csound/blob/609072449df7cce6b1b8f0ba4efc36f97441b52b/Engine/linevent.c#L174-L259)).
These are valid live-coding choices, not bounded callback behavior.

CSD can embed binary assets, but the implementation decodes them into temporary filesystem files
([one_file.c](https://github.com/csound/csound/blob/609072449df7cce6b1b8f0ba4efc36f97441b52b/Top/one_file.c#L933-L973)).
Modules/opcodes are dynamically extensible
([csmodule.c](https://github.com/csound/csound/blob/609072449df7cce6b1b8f0ba4efc36f97441b52b/Top/csmodule.c#L25-L69)).
Native precision is a float/double build choice and the public Wasm wrapper allocates across its API
boundary ([CMake](https://github.com/csound/csound/blob/609072449df7cce6b1b8f0ba4efc36f97441b52b/CMakeLists.txt#L205-L232),
[Wasm wrapper](https://github.com/csound/csound/blob/609072449df7cce6b1b8f0ba4efc36f97441b52b/wasm/src/csound_wasm.c#L21-L63)).
No native/Wasm bit-exact gate was found in the targeted source.

V2 instead compiles a strict declarative model transactionally into a structurally immutable plan,
admits absolute-sample commands through bounded queues and streams identified sources through fixed
rings. Csound supplies no renderer, package, state or numeric component that should replace those
boundaries.

## Decision

- **Investigate only on demand:** an offline executable-document importer which compiles a capped
  source format into ordinary canonical V2 TOML plus existing source/effect identities. The imported
  document is authoring input, never runtime authority.
- **Preserve:** V2's typed session, immutable prepared graph, atomic queue admission, deterministic
  event order, CID packages and off-render asset resolution.
- **Reject:** a Csound VM/opcode ABI in render, render-time compilation/evaluation, within-block graph
  replacement, implicit filesystem access, temp-file assets and build-selected numeric semantics.

## Gates for any future importer

1. Parsing/compilation is control-thread-only and produces byte-identical canonical V2 TOML and
   stable IDs across supported hosts.
2. Invalid code, unresolved assets or cap overflow rejects transactionally without changing the
   published plan.
3. Embedded assets are size-capped and digest-verified; no temp files or implicit filesystem calls.
4. Generated events/nodes, zero-time recursion, memory and compile time are explicitly bounded.
5. Snapshot round-trip retains no Csound VM heap and existing realtime probes remain green.

## Limitation

An importer improves authoring interchange only if musicians actually need Csound-compatible input.
It does not by itself improve mixing performance, simplify the V2 runtime or prove bit identity.

