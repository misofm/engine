# 018 Launch two-band LR4 multiband compressor

## Outcome

Deliver one bounded launch multiband processor: a fixed two-band, phase-declared LR4 crossover
feeding two independent feed-forward peak compressors, with explicit detector linking, fixed
lookahead latency, scalar processing, homogeneous W4/W8 banks, scalar tails and one public
registry-to-graph vertical.

## Context

Engine V2 is greenfield and must never inspect or inherit V1/legacy. The realtime plane owns a
preallocated immutable-shape `PreparedRenderPlan`; render performs no allocation/free, locks,
feature detection, I/O, logging, syscalls, panic/unwind, structural mutation or data-dependent
unbounded work. L/R audio, filter histories, dynamics state and parameters are independent except
for the selected detector-link mode. Launch rates are exactly 44,100, 48,000, 88,200 and 96,000 Hz.

This issue consumes the accepted native-effect, rack-bank and graph/PDC contracts, Issue 013's
compressor conventions, and the conditioned incremental-TPT primitive accepted by the builtin
line. It does not depend on the unresolved parametric-EQ recurrence. It permits exactly **two total
attempts**: Terra attempt 1 and one bounded Sol correction/review. A second failure stops and
requires a stateless rebrief; no gate may weaken.

## Scope

- Add `miso.multiband-compressor`, contract 1.0, state layout 1 and Normal quality at all launch
  rates.
- Split each lane through one fixed fourth-order Linkwitz-Riley crossover: two cascaded conditioned
  TPT Butterworth low-pass sections and two independent cascaded high-pass sections. Low plus high
  is the LR4 all-pass sum: flat magnitude, declared nonlinear phase and zero crossover latency.
- Use exactly two feed-forward instantaneous-peak compressors with Issue 013's gain curve and time
  constants, fixed 6 dB knees, fully wet band processing and no auto gain.
- Reuse `LinkMode::{DualMono, Maximum, Average}` per corresponding band. Linking shares only the
  instantaneous detector magnitude; all other state remains lane- and band-local.
- Provide one per-lane preparation/state-only crossover control from 80 to 8,000 Hz and one
  preparation/state-only 0–20 ms lookahead shared by both bands. Report fixed `Fs/50` latency;
  enabled, identity and bypass paths retain it exactly.
- Expose required dual-mono `main-in`/`main-out` only. V1 has no external sidechain.
- Implement scalar plus W4/W8 homogeneous banks, scalar tails and a ten-track registry/graph
  fixture.

## Required public interfaces/contracts

`MultibandCompressorFactory` implements `NativeEffectFactory`; prepared scalar/bank products use
the accepted runtime traits. Metadata fixes Normal quality, rate, quantum, bypass, link, ports,
`LatencySamples(Fs/50)`, `TailSamples::Infinite`, exact state/resources and automation capacity.

Stable readable `PerLane` parameter IDs, in descriptor order, are:

| ID | control | unit | inclusive domain | default | mapping | automation/smoothing |
|---:|---|---|---:|---:|---|---|
| 1 | crossover | Hz | 80..8000 | 1000 | logarithmic | None / None |
| 2 | lookahead | ms | 0..20 | 5 | linear | None / None |
| 3 | low threshold | dB | -80..0 | -18 | linear | Block Point / Linear 64 |
| 4 | low ratio | ratio | 1..20 | 4 | logarithmic | Block Point / Linear 64 |
| 5 | low attack | ms | 0.1..200 | 10 | logarithmic | Block Point / Linear 64 |
| 6 | low release | ms | 5..5000 | 100 | logarithmic | Block Point / Linear 64 |
| 7 | low makeup | dB | -24..24 | 0 | linear | Block Point / Linear 64 |
| 8 | high threshold | dB | -80..0 | -18 | linear | Block Point / Linear 64 |
| 9 | high ratio | ratio | 1..20 | 4 | logarithmic | Block Point / Linear 64 |
| 10 | high attack | ms | 0.1..200 | 10 | logarithmic | Block Point / Linear 64 |
| 11 | high release | ms | 5..5000 | 100 | logarithmic | Block Point / Linear 64 |
| 12 | high makeup | dB | -24..24 | 0 | linear | Block Point / Linear 64 |

The tracked brief freezes coefficient design, recurrence/order, linked detection, gain equations,
delay taps, state/resources, reset/restore/recovery, scalar/W4/W8 graphs, identity and FMA policy.

## Deliverables

- `miso-engine-multiband-compressor` descriptor, factory, scalar and bank products;
- only necessary registry, effect-compiler, rack and graph seams;
- compact independent crossover/recombination and two-band dynamics tests; and
- a ten-track width-correct bank-plus-scalar-tail graph/PDC and transactional-cap fixture.

## Explicit non-goals

Three through eight bands; configurable topology/order; compensation branches for three-or-more
bands; linear-phase, FIR or multirate frameworks; other qualities; band solo/mute; external
sidechain; RMS detection; variable latency; broad corpus/random/long matrices; realtime audit;
target/object qualification; benchmark; tuning; or completed listening. Those surfaces belong to
Issue 051, **Launch multiband compressor qualification, realtime audit, and benchmark**.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Production SIMD builtin bank graph retention and reachability qualification
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Launch feed-forward peak compressor

Stopped Issue 008 contributes only its preserved generic bank architecture. The conditioned TPT
recurrence is accepted technical input; this issue does not claim stopped Issue 007 passed and does
not consume Issue 045.

## Sol implementation brief

**READY FOR TERRA ATTEMPT 1 after local/remote synchronization.** The authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/018-multiband-compressor.md`. No implementation, benchmark or GitHub
mutation occurs here. `timed_benchmark_invocations=0`.

## Hazards/decisions

LP and HP branches require independent histories; sharing their first section changes the transfer.
Their sum is all-pass, not zero-phase or sample-identical dry, so identity/bypass selects a separate
delayed-dry ring while crossover tests inspect the unmasked band sum. Crossover coefficients are
prepared, never automated. Research authority is `[REISS-COMP]`, `[ORFANIDIS-ISP]`,
`[SMITH-SASP]` and `[VST3-LATENCY]` plus the accepted builtin TPT evidence.

## Acceptance gates with objective measurements

1. Descriptor/preparation/resource mutations reject transactionally; exact and one-byte-below
   state/scratch caps pass at every launch rate.
2. Independent `f64` LP4/HP4 and raw-sum tests at 80/1000/8000 Hz meet 0.05 dB all-pass magnitude
   and 0.02 dB crossing tolerances.
3. Representative ratio-identity, low-only, high-only and both-band fixtures meet 0.01 dB curve,
   0.005 dB envelope and greater-of-one-sample-or-2% time-constant gates.
4. Lookahead 0/5/20 ms enabled/bypass impulses land at `Fs/50`; link modes distinguish correctly;
   whole-effect identity/bypass returns delayed-dry bits exactly.
5. Exact 64-update automation, both resets, transactional continuation restore, signed-zero
   identity, sanitation, lane-local recovery and L/R/track isolation pass.
6. Scalar/W4/W8 TPT/gain paths meet the frozen parity contract. The ten-track graph retains
   width-correct banks/tails, exact PDC and one-byte-below ownership return.
7. Focused format/check/tests/Clippy and relevant policies pass; static scans prove the realtime,
   backend/FMA and no-track-cap contracts.

## Target matrix

Product closure executes native scalar and the available host bank backend. W4/W8 source and
selection contracts are mandatory. Expanded x86/AArch64/Wasm instruction evidence is Issue 051.

## Required evidence

Candidate identity; exact descriptor/state/resource rows; independent response/dynamics maxima;
latency/link/state/recovery/bank/graph results; focused commands/policies; attempt count; Terra and
final Sol verdicts; successor link; and `timed_benchmark_invocations=0`.
