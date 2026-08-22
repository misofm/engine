# 027 Third-party WASM package and effect ABI conformance kit

## Outcome

Specify and test third-party Wasm effects now without executing them in launch render graphs.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; higher named rates are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Bind a capability-free core Wasm ABI to the canonical package/artifact/CID contract, with bounded memory/state/scratch declarations, parameter events, ports, latency/tail, fixtures, signing/trust metadata separation and an offline validator/conformance CLI.

## Required public interfaces/contracts

`ThirdPartyEffectAbiV1` defines fixed f32 pointer/length process calls, prepare/reset/state/metadata exports, maximum resource declarations and no-WASI imports; `validate_third_party_effect_package` first requires a package verified under **Canonical effect package, CID, and artifact selection**, then returns exact ABI diagnostic codes and the already-verified CID.

## Deliverables

Public ABI/spec, WIT/control metadata if useful outside realtime ABI, validator, malformed ABI corpus, bindings to the shared package/CID vectors, conformance producer guide and sample no-op effect.

## Explicit non-goals

Executing untrusted Wasm, permitting it in SIMD racks, WASI/network/filesystem imports, trust-store product, or a web effect marketplace.

## Dependencies by exact issue title

- Canonical effect package, CID, and artifact selection
- Transport-neutral binary control protocol
- DSP research corpus and conformance harness

## Hazards/decisions

CID is deterministic exact-byte identity, not safety/trust: https://specs.ipfs.tech/cid/. Component/WIT may describe control surfaces but the realtime process ABI remains fixed/bounded and no-alloc.

## Acceptance gates with objective measurements

Valid sample package CID matches vector; each forbidden import/oversize/malformed export rejects with code; conformance fixture declares/checks latency/tail/resource bounds; validator has no renderer dependency.

## Target matrix

Native and browser toolchains validate packages; no runtime executor on any launch target.

## Required evidence

ABI text, CID fixtures, validator/fuzz report, malformed corpus checksums, and producer conformance transcript.
