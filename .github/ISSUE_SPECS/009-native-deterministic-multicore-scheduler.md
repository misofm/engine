# 009 Native deterministic multicore scheduler

## Outcome

Add native multicore render only where dependency waves can run without waiting or nondeterministic summation.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Compile preallocated dependency waves from the graph, assign deterministic work partitions, define fixed reduction order and fallback to single-thread for insufficient budget/capability.

## Required public interfaces/contracts

`RenderWave` contains immutable job ranges/dependencies; `NativeScheduler::render_wave` has prestarted workers and bounded completion protocol; scheduler exposes deadline/late counters and a single-thread fallback.

## Deliverables

Wave schedule format, worker lifecycle, affinity/workgroup policy where supported, deterministic reduction implementation, tests and benchmark report.

## Explicit non-goals

Browser parallel rendering, spawning/joining threads from the callback, work stealing in realtime, or changing floating reduction order run-to-run.

## Dependencies by exact issue title

- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels

## Hazards/decisions

Initial launch must remain correct single-thread. Apple auxiliary RT threads require appropriate audio workgroup integration: https://developer.apple.com/documentation/audiotoolbox/adding-audio-unit-auxiliary-real-time-threads-to-audio-workgroups.

## Acceptance gates with objective measurements

One hundred randomized worker schedules are bit-identical to the declared single-thread reduction order; no render-thread allocation, OS mutex, thread spawn/join or blocking call is detected; on a pinned sufficiently parallel graph the two-worker run is at least 1.5x and the four-worker run at least 2.5x faster than one worker, or profiling demonstrates a false architectural assumption and triggers rescope rather than waiver; P99.99 callback time stays below 70% of the quantum for ten minutes with zero deadline misses on the issue’s pinned canonical graph; automatic fallback passes the same fixtures.

## Target matrix

Native only: Linux/cloud, macOS/iOS where host permits, Android where validated; browser explicitly sequential.

## Required evidence

Trace of wave order, CPU/deadline benchmark JSON, single-vs-multicore fixtures, and fallback/fault-injection results.
