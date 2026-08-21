# 017 Dynamic EQ

## Outcome

Implement a bankable dynamic EQ that combines disclosed parametric filters with per-band dynamics.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement per-band filter descriptors, detector source/link, threshold/ratio/range, attack/release, direction, sidechain and quality/latency declarations.

## Required public interfaces/contracts

`DynamicEq` implements `NativeEffect`; every band has stable numeric ID and independent state; declared ports/latency include sidechain and detector topology.

## Deliverables

Effect/note, metadata, allocation-free band storage strategy, test fixtures, kernels, benchmark and listening evidence.

## Explicit non-goals

Unbounded runtime band creation, third-party modules, implicit sidechain link, or linear-phase mode.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- Parametric EQ
- Compressor
- Deterministic graph compiler, sends, submixes, sidechains, and PDC

## Hazards/decisions

Use RBJ and dynamics references from issues 012/013, restated in the effect note; parameters must define order of filter/gain updates.

## Acceptance gates with objective measurements

Static mode matches issue 012’s 0.05 dB/0.1% response gates; dynamic gain matches the independent design within 0.1 dB and timing within the greater of one sample or 2%; external sidechain routing/PDC and bypass latency are sample-exact; the maximum prepared band count has 0 process allocations.

## Target matrix

Scalar/4-lane/AVX2 native core; browser/mobile through common core.

## Required evidence

Per-band response/envelope fixtures, capacity test, scalar/SIMD data, benchmark and listening record.
