# 022 Stable C ABI and native PCM reference runner

## Outcome

Expose a narrow stable native embedding ABI and a reference PCM runner for cloud/headless verification.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement versioned opaque handles for engine/session/plan, explicit error/result buffers, planar `f32` PCM render, control submission outside render, capability query and a CLI/reference library that loads TOML and produces PCM.

## Required public interfaces/contracts

`engine_v2_abi_version`, `engine_create`, `engine_compile_session`, `engine_render_f32_planar`, `engine_submit_command`, and `engine_destroy` use fixed-width types and caller-owned buffers; render has no strings/allocating callbacks.

## Deliverables

C header, Rust exports, ABI tests, reference runner, WAV/RF64 source integration, PCM output fixtures, docs and compatibility policy.

## Explicit non-goals

Network server, delivery codecs, a broad unstable C++ API, host UI, or freeing caller PCM in render.

## Dependencies by exact issue title

- Bootstrap Rust workspace and target matrix
- Versioned TOML schema and transactional session compiler
- Transport-neutral binary control protocol
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Dual-mono builtins and metering
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- JIT PCM streaming and host-supplied source rings
- Native effect runtime contract and conformance

## Hazards/decisions

ABI is narrow while semantic control is broad through issue 005. ABI changes require version negotiation or a new entrypoint.

## Acceptance gates with objective measurements

C and Rust ABI layout smoke test passes; the opaque ABI submits/receives golden issue-005 protocol frames without exposing Rust layout; reference runner writes raw planar `f32` PCM and renders a golden session checksum; one million render calls show 0 engine allocations/frees in render; malformed handles and undersized caller buffers fail safely without process crash.

## Target matrix

Linux/cloud mandatory; macOS/Windows where supported; mobile and browser use adapter-specific bindings.

## Required evidence

Generated header, ABI test log, golden PCM hashes, allocation audit, and runner command transcript.
