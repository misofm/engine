# 026 End-to-end release, performance, and listening qualification

## Outcome

Qualify the launch mixer/mastering engine with repeatable functional, realtime, portability, performance and documented listening evidence.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; higher named rates are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Create release sessions covering tracks/racks/sends/submix/PDC/effects/streaming/control; run correctness, memory, underrun, SIMD, host and listening matrices; publish known limits and issue residual optimization work.

## Required public interfaces/contracts

`ReleaseQualificationReportV1` records engine/git/toolchain versions, target/capabilities, fixtures, tolerances, CPU/RAM/underrun measures, defects and listening protocol; pass/fail derives solely from recorded gates.

## Deliverables

Automated release suite, benchmark dashboard artifacts, listening protocol/results, target matrix, release checklist, known-limits document and follow-up issue list.

## Explicit non-goals

Relaxing gates for schedule, declaring third-party Wasm shipped, delivery encoding, or unsupported hardware claims.

## Dependencies by exact issue title

- Stable C ABI and native PCM reference runner
- iOS and Android embedding examples
- WASM SIMD AudioWorklet embedding
- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral
- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime
- Versioned TOML schema and transactional session compiler
- Transport-neutral binary control protocol
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Issue-007 builtin qualification tooling, audits, and benchmark
- Issue-007 builtin filter and matrix human listening qualification
- Production SIMD builtin bank graph retention and reachability qualification
- Issue-008 real audio benchmark workloads and exactly-once qualification
- Native graph scheduler qualification and benchmark
- Exact lock-free native source sanitation telemetry handoff
- Issue-010 source streaming qualification tooling and adversarial evidence
- Native effect runtime contract and conformance
- Launch parametric EQ recurrence derivation and runtime proof
- Launch feed-forward peak compressor
- Launch compressor qualification, realtime audit, and benchmark
- Launch gate reset, restore, and recovery proof
- Launch gate/expander qualification, realtime audit, and benchmark
- De-esser
- Launch fixed-4x true-peak safety limiter
- Launch true-peak limiter qualification, realtime audit, and benchmark
- Dynamic EQ
- Multiband compressor
- Antialiased saturator/clipper
- Transient shaper
- Dual-mono/stereo delay
- Optional binary WebSocket sidecar

## Hazards/decisions

Use ITU/EBU references where meters/loudness are claimed: https://www.itu.int/rec/R-REC-BS.1770-5-202311-I and https://tech.ebu.ch/publications/r128. Listening evidence complements, never replaces, objective gates.

## Acceptance gates with objective measurements

All referenced issue gates pass. On each named pinned realtime baseline, a ten-minute run has zero deadline misses and P99.99 callback time below 70% of the quantum; reports include dry-routing, 32 dual-mono console (builtins + EQ + compressor, four submixes and two sends), and representative mastering scenarios at each launch rate (44.1/48/88.2/96 kHz). Publish maximum sustainable track count for each scenario/rate rather than claiming equal capacity. A 60-minute sparse-stem streaming soak stays within the exact prepared ring/arena ceiling and has 0 render allocations/frees. Median CPU regression above 5% or P99 above 10% fails against the pinned baseline unless a new Sol-approved issue records the correctness tradeoff. Every target has a golden PCM tolerance result; unresolved release-gate defects fail qualification rather than merely becoming links.

## Target matrix

Linux/cloud native, iOS, Android, browser scalar and SIMD where supported; optional sidecar separately qualified.

## Required evidence

Checksummed report artifact; raw benchmark/soak logs; allocator traces; fixture hashes; target results; anonymized blinded-listening records; residual issue links; and CPU, OS, power/governor mode, compiler, target features, runtime/browser, sample rate, quantum, fixture, warm-up, run duration and statistical method for every performance claim.
