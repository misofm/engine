# 018 Multiband compressor

## Outcome

Implement a multiband compressor with documented crossover phase/latency and bounded band count.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement two through eight prepared bands, selectable bounded crossover topology, per-band compressor controls, band solo/mute, detector linking, recombination, lookahead and quality modes.

## Required public interfaces/contracts

`MultibandCompressor` implements `NativeEffect`; `CrossoverSpec` reports integer latency/tail and phase policy; each band’s numeric descriptors are stable and preallocated.

## Deliverables

Effect/note, crossover fixtures, recombination tests, metadata, scalar/4-lane/8-lane kernels, benchmark and listening evidence.

## Explicit non-goals

Unlimited bands, zero-latency claim without proof, hidden phase policy, or adaptive allocation.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- Launch feed-forward peak compressor

## Hazards/decisions

Document crossover equations/coefficient updates and cancellation/recombination numerical limits; link behavior remains explicit.

## Acceptance gates with objective measurements

Flat settings recombine swept sine within ±0.1 dB where the documented crossover is nominal; reported latency and bypass compensation are impulse-exact; each band’s static curve is within 0.1 dB and timing within the greater of one sample or 2%; isolated-band controls stay within the pre-briefed crossover leakage tolerance; eight bands yield 0 render alloc.

## Target matrix

Native scalar/AVX2 and 4-lane portable/Wasm; quality availability declared per target.

## Required evidence

Crossover/recombination plots, latency fixture, stress tests, CPU/RAM table, and listening evidence.
