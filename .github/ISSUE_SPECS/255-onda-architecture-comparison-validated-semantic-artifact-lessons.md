# 255 Onda architecture comparison: validated semantic-artifact lessons for future compiled effects

One-line summary: Source-level comparison of Onda and Engine V2 finds one bounded future lesson—a
validated backend-neutral compiled-effect artifact with an explicit numeric profile and logical state
map—while preserving V2's current realtime, determinism, SIMD, packaging, graph and streaming
architecture.

**This is a completed research and decision record, not implementation authority.** It authorizes no
runtime compiler, third-party executor, graph rewrite or new effect language. Any product change needs
its own smallest-closable Sol-approved issue after programmable effects become real product scope.

**Authority: GitHub issue #255.** This local file mirrors its source-backed decision record.

## Authority, pins and method

- Engine V2: [`90c3b9a598f1244938d9cdcce04c4a4641c6b758`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758).
- Onda: [`60958ab177a3cf37407fe28c54778c4776b45fc8`](https://github.com/onda-lang/onda/tree/60958ab177a3cf37407fe28c54778c4776b45fc8).
- No V1 or legacy Miso source was inspected. No cross-project benchmark ran. Performance statements
  are code-shape observations, not whole-engine rankings.

The audit traced Onda's source-to-MIR-to-native/Wasm path, validator, numerical operations, state
metadata and project images, then checked each apparent lesson against current V2 source and launch
constraints.

## Executive verdict

Onda has the clearest public example in this comparison of one immutable semantic program feeding
native and browser backends. Its MIR carries compile configuration, interface, functions, state,
fixed-width operations, bounds behavior and process entry, and it is validated before lowering
([MIR overview](https://github.com/onda-lang/onda/blob/60958ab177a3cf37407fe28c54778c4776b45fc8/crates/onda_mir/src/lib.rs#L1-L63),
[IR](https://github.com/onda-lang/onda/blob/60958ab177a3cf37407fe28c54778c4776b45fc8/crates/onda_mir/src/ir.rs#L39-L129),
[validator](https://github.com/onda-lang/onda/blob/60958ab177a3cf37407fe28c54778c4776b45fc8/crates/onda_mir/src/validate.rs#L30-L167)).
Native LLVM and browser Binaryen paths consume that semantic program rather than independently
recovering source semantics ([LLVM](https://github.com/onda-lang/onda/blob/60958ab177a3cf37407fe28c54778c4776b45fc8/crates/onda_codegen_llvm/src/lib.rs#L753-L797),
[browser backend](https://github.com/onda-lang/onda/blob/60958ab177a3cf37407fe28c54778c4776b45fc8/packages/onda_binaryen_web/src/index.js#L186-L293)).

V2 already has a deterministic graph execution program, but it is a session-routing/runtime program,
not portable programmable-DSP semantics ([program.rs](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/crates/miso-engine-graph/src/program.rs#L1-L124)).
Its effect package can contain Source, CoreWasm and TargetNative artifacts, while a CID proves exact
package bytes rather than semantic lineage or cross-target equivalence
([package.rs](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/crates/miso-engine-effect-package/src/package.rs#L17-L167),
[cid.rs](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/crates/miso-engine-effect-package/src/cid.rs#L8-L99)).
That missing lineage is relevant only to future generated effects; it is not a present native-effect
runtime defect.

## Decision

### Adopt later, behind a scope trigger

If programmable effects enter product scope, specify a validated semantic artifact from which Core
Wasm and qualified native artifacts derive. Bind its digest to the descriptor, compiler/backend
identity, target features, numeric profile and a backend-independent logical state map. Onda's
snapshot/scratch/control-mirror separation is useful evidence for the last item
([state classes](https://github.com/onda-lang/onda/blob/60958ab177a3cf37407fe28c54778c4776b45fc8/crates/onda_mir/src/ir.rs#L411-L436),
[logical snapshot metadata](https://github.com/onda-lang/onda/blob/60958ab177a3cf37407fe28c54778c4776b45fc8/crates/onda_codegen_llvm/src/mir_metadata.rs#L330-L363)).

The smallest future slice is schema, validator and an offline one-effect native/CoreWasm conformance
prototype. It must not add a product JIT, callback Wasm runtime, whole-session compiler or native
promotion path.

### Preserve

- V2's unfused multiply-plus-add and deterministic transcendental policy. Onda deliberately treats
  transcendental parity approximately in part of its backend suite; that is not V2's bit contract
  ([Onda parity script](https://github.com/onda-lang/onda/blob/60958ab177a3cf37407fe28c54778c4776b45fc8/packages/onda_binaryen_web/scripts/verify-backend-parity.mjs#L95-L208),
  [V2 deterministic math](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/crates/miso-engine-math/src/lib.rs#L1-L27)).
- V2's prepared-plan ownership and render audit
  ([plan.rs](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/crates/miso-engine-core/src/realtime/plan.rs#L168-L182),
  [audit.rs](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/crates/miso-engine-core/src/realtime/audit.rs#L1-L68)).
- Existing target-pinned scalar/Simd4/Simd8 banking, typed effect-state envelopes, exact-byte package
  CIDs, canonical sessions and duration-independent source streaming.

### Reject

Do not adopt Onda wholesale, compile every V2 session as generated code, make exact software FMA the
default, expose physical native/Wasm memory as portable state, weaken transcendentals to tolerance,
or load whole stems into project buffers. Onda's project buffers are `Vec`-backed whole assets
([buffer.rs](https://github.com/onda-lang/onda/blob/60958ab177a3cf37407fe28c54778c4776b45fc8/crates/onda_project/src/buffer.rs#L11-L24));
V2's bounded rings keep retained memory independent of stem duration.

## Gates for any future semantic-artifact issue

1. Reject unknown versions/opcodes/types/imports, recursion, indirect calls, unbounded loops,
   unchecked memory and resources above explicit caps.
2. Cache identity changes with semantic IR, compiler/backend, target/features or numeric profile;
   removing any field is a red mutation.
3. The strict profile forbids contraction, reuses V2 deterministic math, defines NaN behavior, and
   proves native/CoreWasm plus scalar/SIMD parity over a frozen corpus.
4. Persistent state is packed canonically in logical little-endian order; scratch and control mirrors
   are excluded; complete restore validation precedes publication.
5. Core Wasm has no WASI/imports, `memory.grow`, reachable allocator or unbounded table/memory.
6. Native output remains qualification material until a separate promotion issue proves trust,
   licensing, conformance, sound quality and performance.
7. The prepared processor passes the existing allocation/free/lock/I/O/log/syscall audit. Opaque
   generated instances never enter V2 homogeneous across-track bank kernels.

## Limitations

Onda supplies strong source architecture and internal tests, not independent production deployment
evidence or a comparable V2 performance workload. A general IR adds validator, compiler, evolution
and security surface. Without an actual programmable-effect requirement, the simpler and more
correct decision is to build nothing.
