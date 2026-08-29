# 256 Cmajor architecture comparison: preserve V2 boundaries and retain cache-lineage evidence

One-line summary: Cmajor proves that native and browser performers can share a compiler toolchain,
but its public architecture provides no stronger numerical, state, realtime or packaging contract
than V2; retain only its cache-key inputs as supporting evidence for a future compiled-effect issue.

**This is a completed research and decision record, not implementation authority.**

**Authority: GitHub issue #256.** This local file mirrors its source-backed decision record.

## Authority, pins and method

- Engine V2: [`90c3b9a598f1244938d9cdcce04c4a4641c6b758`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758).
- Cmajor: [`024a208515f15e43271d9b2ea85ee22a2233384b`](https://github.com/cmajor-lang/cmajor/tree/024a208515f15e43271d9b2ea85ee22a2233384b).
- No legacy Miso source was inspected and no cross-project benchmark ran.

The audit followed Cmajor compiler transformations, LLVM/Wasm/C++ performers, numerical settings,
state capture, allocation checking, endpoints and patch resources, then challenged each candidate
lesson against current V2.

## Findings

Cmajor transforms a mutable AST through simplification, intrinsic replacement, canonicalization and
graph flattening before its backends. The reviewed source does not expose an immutable serialized
validated MIR as the public backend/package boundary
([engine base](https://github.com/cmajor-lang/cmajor/blob/024a208515f15e43271d9b2ea85ee22a2233384b/modules/compiler/src/backends/cmaj_EngineBase.h#L135-L236),
[transformations](https://github.com/cmajor-lang/cmajor/blob/024a208515f15e43271d9b2ea85ee22a2233384b/modules/compiler/src/transformations/cmaj_Transformations.cpp#L120-L165)).
Its cache key does correctly include program-code hash, engine version and build settings. That is
supporting evidence that future V2 compiled artifacts must invalidate on semantic input, compiler,
backend, target/features and numeric-profile changes.

Cmajor can generate native LLVM and Wasm performers, including SIMD and non-SIMD browser modules
selected after runtime feature detection
([LLVM performer](https://github.com/cmajor-lang/cmajor/blob/024a208515f15e43271d9b2ea85ee22a2233384b/modules/compiler/src/backends/LLVM/cmaj_LLVMPerformer.cpp#L185-L375),
[Wasm generator](https://github.com/cmajor-lang/cmajor/blob/024a208515f15e43271d9b2ea85ee22a2233384b/modules/compiler/src/backends/WebAssembly/cmaj_JavascriptClassGenerator.h#L33-L180)).
V2's compile-time target selection and one boot attestation deliberately keep runtime state space
smaller ([backend.rs](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/crates/miso-engine-lane/src/backend.rs#L1-L129)).

At higher optimization Cmajor enables fast-math behavior, and native/Wasm transcendental paths are
not one public bit-pinned contract
([build settings](https://github.com/cmajor-lang/cmajor/blob/024a208515f15e43271d9b2ea85ee22a2233384b/include/cmajor/API/cmaj_BuildSettings.h#L32-L45),
[LLVM flags](https://github.com/cmajor-lang/cmajor/blob/024a208515f15e43271d9b2ea85ee22a2233384b/modules/compiler/src/backends/LLVM/cmaj_LLVMGenerator.h#L35-L60)).
No public exact cross-backend numerical fixture was located. That absence is an evidence gap, not
proof of failure, but it cannot justify weakening V2's deterministic math contract.

The generated browser performer's `getState` clones whole Wasm memory
([generator](https://github.com/cmajor-lang/cmajor/blob/024a208515f15e43271d9b2ea85ee22a2233384b/modules/compiler/src/backends/WebAssembly/cmaj_JavascriptClassGenerator.h#L322-L342)).
V2's typed provenance-bearing, capacity-checked, caller-buffer effect snapshots are the safer portable
boundary. Cmajor's optional allocation checker is useful tooling, but V2 already makes the broader
render prohibition an engine invariant
([Cmajor checker](https://github.com/cmajor-lang/cmajor/blob/024a208515f15e43271d9b2ea85ee22a2233384b/modules/playback/src/cmaj_AllocationChecker.cpp#L23-L157),
[V2 audit](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/crates/miso-engine-core/src/realtime/audit.rs#L1-L68)).

## Decision

- **Adopt:** nothing independently.
- **Retain as evidence:** a future compiled-effect cache/lineage key includes the canonical semantic
  artifact, compiler and backend versions, target triple/features and numeric profile. A mutation
  matrix must prove every field causes a miss and identical canonical inputs reproduce a key.
- **Preserve:** V2's strict graph program, deterministic operations, typed state, target-pinned SIMD,
  exact-byte package identity, fixed prepared plan, streamed sources and narrow C ABI.
- **Reject:** a mutable transformed AST or generated C++ as stable semantic interchange, raw linear
  memory as portable state, launch fast-math, runtime dual-SIMD selection in the engine contract, or
  whole-resource patch loading as the stem model.

## Limitation

This audit covers the public Cmajor toolkit, not every commercial embedding. Private parity or
packaging mechanisms may exist. They cannot be credited without evidence, just as absent public
fixtures cannot be treated as proof of a defect.

## Closure

Closed as completed research and superseded by final synthesis #268. No independent Cmajor-derived
work survives.
