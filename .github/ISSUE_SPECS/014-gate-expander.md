# 014 Launch hysteretic peak gate/expander

## Outcome

Deliver one useful console-style dual-mono gate/downward expander with explicit hysteresis, hold,
sidechain and detector linking, fixed-latency lookahead, scalar execution, homogeneous W4/W8
banks, scalar tails and one public registry-to-graph vertical.

## Context

Engine V2 is greenfield; never inspect or inherit V1. Render owns a preallocated immutable-shape
plan and performs no allocation/free, locks, feature detection, I/O, logging, syscalls, panic/
unwind, structural mutation or data-dependent unbounded work. L/R audio and state are independent
except for the declared detector link. Launch rates are exactly 44,100, 48,000, 88,200 and 96,000
Hz, with no implicit SRC or compiled track ceiling.

This issue uses the accepted native-effect, rack-bank and graph/PDC contracts and the Issue-013
dynamics conventions. It has exactly **two total attempts**: Terra attempt 1 and one bounded Sol
correction/review. A second failure stops and requires a stateless rebrief; gates cannot weaken.

## Scope

- Add `miso.gate-expander`, contract 1.0, state layout 1 and Normal quality at all launch rates.
- Use feed-forward instantaneous absolute-peak detection and one hard-knee downward-expansion
  curve, capped by an explicit range.
- Use an explicit hysteretic open/hold/close state. Attack smooths toward open; release smooths
  toward attenuation. No hidden program-dependent timing exists.
- Reuse `LinkMode::{DualMono, Maximum, Average}`. Only the detector value links; parameters,
  phase, hold, gain, rings, recovery and payload remain lane-local.
- Expose optional dual-mono `sidechain-in`. Unconnected detection uses main input and is bankable;
  connected sidechain follows the accepted scalar fallback.
- Report fixed 10 ms latency (`N=Fs/100`). Preparation/state-only lookahead selects 0–10 ms
  detector advance inside that delay; enabled and bypass paths retain exact latency.
- Implement scalar plus unconnected W4/W8 banks and one ten-track registry/graph fixture.

## Required public interfaces/contracts

`GateExpanderFactory` implements `NativeEffectFactory`; prepared scalar/bank products implement the
accepted runtime traits. Ports are ordered required `main-in`/`main-out` plus optional
`sidechain-in`, all dual-mono planar. Metadata fixes Normal quality, rate, quantum, bypass, link,
ports, `LatencySamples(N)`, `TailSamples::Finite(0)`, state/resources and automation capacity.

Stable per-lane parameter IDs, in descriptor order, are:

| ID | control | unit | inclusive domain | default | mapping | automation/smoothing |
|---:|---|---|---:|---:|---|---|
| 1 | threshold | dB | -80..0 | -40 | linear | Block Point / Linear 64 |
| 2 | ratio | ratio | 1..20 | 4 | logarithmic | Block Point / Linear 64 |
| 3 | range | dB | 0..96 | 80 | linear | Block Point / Linear 64 |
| 4 | hysteresis | dB | 0..24 | 6 | linear | Block Point / Linear 64 |
| 5 | attack | ms | 0.1..50 | 1 | logarithmic | None / None |
| 6 | hold | ms | 0..1000 | 100 | linear | None / None |
| 7 | release | ms | 5..2000 | 100 | logarithmic | None / None |
| 8 | lookahead | ms | 0..10 | 2 | linear | None / None |

The tracked Sol brief freezes the exact curve, transition order, coefficient/sample rounding,
delay realization, state words, resource rows, sanitation/recovery, reset/restore transaction and
scalar/W4/W8 operation graph.

## Deliverables

- `miso-engine-gate-expander` with descriptor, factory, scalar and bank products;
- the smallest core gain-kernel, effect-registry and graph seams;
- compact independent `f64` curve/transition tests and representative checked fixtures; and
- one ten-track bank-plus-tail-plus-connected-sidechain graph differential.

## Explicit non-goals

RMS detection, soft knee, detector HPF/LPF, key listen, ducking inversion, program-dependent
timing, multiple qualities, changing latency, connected-sidechain banking, generic fixture/audit/
benchmark infrastructure, expanded randomized or million-sample matrices, cross-target object
qualification, performance tuning, benchmark invocation or completed listening. Those evidence
surfaces belong to Issue 047, **Launch gate/expander qualification, realtime audit, and benchmark**.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Production SIMD builtin bank graph retention and reachability qualification
- Deterministic graph compiler, sends, submixes, sidechains, and PDC

Stopped Issue 008 contributes only its preserved generic architecture/effect-bank slice, not PASS
or benchmark evidence.

## Sol implementation brief

**READY FOR TERRA ATTEMPT 1 after local/remote title and body synchronization.** The authoritative
brief is `.github/ISSUE_SPECS/BRIEFS/014-gate-expander.md`. No implementation, benchmark or GitHub
mutation occurs in this checkpoint.

## Hazards/decisions

The detector/gain-computer separation and time constants follow `[REISS-COMP]`; bounded delay state
follows `[SMITH-SASP]`; fixed latency/bypass follows `[VST3-LATENCY]`. Hysteresis changes only the
open/close decision, never the declared static expansion curve. A short pre-latency fixture,
production-derived oracle, shared linked state or concealed detector filter is invalid evidence.

## Acceptance gates with objective measurements

1. Descriptor/prepare/resource mutations reject transactionally at every launch rate; exact state
   rows and one-byte-below state/scratch caps pass.
2. Representative independent `f64` curve points cover ratio identity, threshold, hysteresis
   boundary and range clamp within `0.01 dB`. One chatter trace proves exact open/hold/close order;
   attack/release `1-exp(-1)` crossings are within the greater of one sample or 2%.
3. Lookahead `0/2/10` ms enabled/bypass impulses land at exact `N`; all three link modes and a
   connected-sidechain/main-detector distinction pass.
4. Exact 64-update automation, both resets, transactional restore/continuation, signed-zero
   identity, sanitation/recovery and L/R/track isolation pass representative adversarial tests.
5. Available scalar/W8 and core W4/W8 gain paths satisfy the frozen equality/tolerance contract.
   The ten-track graph retains width-correct full banks, one unconnected tail and one connected
   scalar without changing schedule, PDC or graph shape; cap failure returns ownership.
6. Focused format/check/tests and warning-denied Clippy pass. Static policy scans prove no render
   allocation/lock/I/O/log/syscall, unsafe caller, hidden target detection or compiled track cap.

## Target matrix

Product closure executes native scalar and the available host bank backend. W4/W8 source contracts
are mandatory. Complete x86/AArch64/Wasm compile and named instruction evidence is deferred to
Issue 047 and is not falsely claimed here.

## Required evidence

Candidate hash; descriptor/state/resource table; representative curve/timing/lookahead/link/
sidechain/automation/state/recovery maxima; scalar/bank/graph results; focused command outputs;
Terra and final Sol verdicts; exact successor link; and `timed_benchmark_invocations=0`.

## Evidence record

### Terra attempt 1 — scalar foundation checkpoint (2026-08-21)

- Added the standalone `miso-engine-gate-expander` package plus test-only independent f64 static
  curve oracle. Production has no oracle dependency.
- Frozen descriptor and Normal resource rows are covered at 44.1/48/88.2/96 kHz: `N=Fs/100`,
  `B=N+1`, `20+2B` words per lane, 64 scratch bytes, finite-zero tail, and one-byte-below
  state/scratch preparation rejection.
- Scalar tests cover the hard ratio/range curve, hysteresis transition ordering, preparation-only
  0/5/10 ms lookahead derivation, fixed 10 ms impulse delay, connected-sidechain distinction,
  exact 64-update automation state, all-or-none restore, and input sanitation. W4/W8 banks,
  registry/graph work, broad qualification, targets, audits and benchmarks were not started.
- Focused commands passed: `cargo fmt --package miso-engine-gate-expander --package
  miso-engine-dsp-reference`; `cargo check -p miso-engine-gate-expander`; `cargo test -p
  miso-engine-gate-expander` (5 passed); and `cargo clippy -p miso-engine-gate-expander
  --all-targets -- -D warnings`. `timed_benchmark_invocations=0`.

### Terra attempt 1 — homogeneous-bank checkpoint (2026-08-21)

- Added the prepared core `GateGainKernelV1` surface: exact `sample*gain` followed by an identity
  mask selection, width validation, and a separately selected AVX2+FMA alias with zero FMA
  contractions. It has scalar shape/mask and available-host AVX2/base-FMA equality coverage.
- `GateExpanderFactory` now prepares exact W4/W8 unconnected homogeneous cohorts with
  width-specialized lane arrays, independent per-track L/R delay/detector/phase/ramp state and
  per-track snapshot/restore/reset. It validates all requests before a legal `Ok(None)` fallback
  for connected sidechains, heterogeneous programs or unavailable backends.
- Focused commands passed: `cargo fmt --check --package miso-engine-core --package
  miso-engine-gate-expander`; `cargo test --locked -p miso-engine-core -p
  miso-engine-gate-expander` (27 core and 5 gate tests passed); and `cargo clippy --locked -p
  miso-engine-core -p miso-engine-gate-expander --all-targets -- -D warnings`.
- Direct gate-bank PCM/state/report parity and isolation tests were not added after the explicit
  coverage freeze at this checkpoint; this evidence therefore does not claim those representative
  product gates. Registry/graph, qualification/audit/target and benchmark work remain untouched.
  `timed_benchmark_invocations=0`.
