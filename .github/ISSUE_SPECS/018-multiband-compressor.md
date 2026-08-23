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
Their sum is all-pass, not zero-phase or sample-identical dry, so only the prepare-time `bypass`
path selects the delayed-dry ring; the enabled path always outputs the band sum (`low + high`),
including at unity gain. A runtime dry/sum switch is a discontinuity (audit #94 F1). Crossover
coefficients are prepared, never automated. Research authority is `[REISS-COMP]`, `[ORFANIDIS-ISP]`,
`[SMITH-SASP]` and `[VST3-LATENCY]` plus the accepted builtin TPT evidence.

## Acceptance gates with objective measurements

1. Descriptor/preparation/resource mutations reject transactionally; exact and one-byte-below
   state/scratch caps pass at every launch rate.
2. Independent `f64` LP4/HP4 and raw-sum tests at 80/1000/8000 Hz meet 0.05 dB all-pass magnitude
   and 0.02 dB crossing tolerances.
3. Representative ratio-identity, low-only, high-only and both-band fixtures meet 0.01 dB curve,
   0.005 dB envelope and greater-of-one-sample-or-2% time-constant gates.
4. Lookahead 0/5/20 ms enabled/bypass impulses land at `Fs/50`; link modes distinguish correctly;
   whole-effect bypass returns delayed-dry bits exactly; the enabled unity-gain path returns the
   delayed LR4 sum within the crossover gate and has no step at unity-gain transitions.
5. Exact 64-update automation, both resets, transactional continuation restore, signed-zero
   bypass, sanitation, lane-local recovery and L/R/track isolation pass.
6. Scalar, Wasm/NEON W4 and base-AVX2 W8 TPT/gain paths are bit-exact for finite-normal inputs.
   Existing AVX2+FMA retains exactly the accepted three TPT contractions and frozen
   `abs(error) <= 1e-6 + 2e-5*abs(scalar)` tolerance; its compressor gain kernel remains
   noncontracting. The ten-track graph retains width-correct banks/tails, exact PDC and
   one-byte-below ownership return.
7. Focused format/check/tests/Clippy and relevant policies pass; static scans prove the realtime,
   backend/FMA and no-track-cap contracts.

## Target matrix

Product closure executes native scalar and the available host bank backend. W4/W8 source and
selection contracts are mandatory. Expanded x86/AArch64/Wasm instruction evidence is Issue 051.

## Required evidence

Candidate identity; exact descriptor/state/resource rows; independent response/dynamics maxima;
latency/link/state/recovery/bank/graph results; focused commands/policies; attempt count; Terra and
final Sol verdicts; successor link; and `timed_benchmark_invocations=0`.

## Terra attempt 1 scalar checkpoint — incomplete

- Candidate base: `b6b2a23`. Added `miso-engine-multiband-compressor` with a Normal-only
  `miso.multiband-compressor` descriptor, 12 ordered PerLane parameters, fixed `Fs/50` latency,
  Infinite tail, required main-in/main-out-only topology, exact four-rate lane-state rows and
  136-byte fixed scratch declaration. The scalar factory returns `Ok(None)` for homogeneous-bank
  binding until the dedicated W4/W8 checkpoint; no registry, compiler or graph seam changed.
- Scalar lanes retain independent dry/low/high rings, four conditioned Butterworth-Q TPT section
  states, ten 64-update dynamics ramps, two gain-reduction states, common 0/5/20-ms lookahead,
  dual-mono/Maximum/Average detector linking, bypass identity warming, both resets, atomic
  snapshot/restore and lane-local sanitation/recovery. Finite TPT subnormals flush to zero rather
  than entering recovery, preserving the fixed latency ring progression.
- Added a test-only, independently derived `f64` LR4 crossover in
  `miso-engine-dsp-reference`. Representative scalar tests cover every launch-rate resource row
  and one-byte-below preparation rejection, independent crossover/recombination at 80/1000/8000
  Hz, isolated low/high compression, fixed bypass latency, point automation and transactional
  restore.
- PASS: `cargo fmt --check --package miso-engine-multiband-compressor --package
  miso-engine-dsp-reference`; locked multiband check; locked multiband library tests (4 passed);
  and locked all-target warning-denied multiband Clippy. No bank, registry, graph, Issue-051
  qualification/audit/target/benchmark/listening command ran; `timed_benchmark_invocations=0`.

## Terra attempt 1 bank checkpoint — STOPPED

- No Issue-018 bank source, core token, test, registry, graph, benchmark or other product edit
  was made in this checkpoint; `timed_benchmark_invocations=0`.
- The existing `PreparedTptBankKernelV1` cannot satisfy the frozen exact-parity rule for an
  AVX2+FMA selection: `crates/miso-engine-core/src/arch/x86.rs`,
  `process_tpt_x86_avx2_fma_inner`, uses `_mm256_fmsub_ps` for `d1`, `_mm256_fmadd_ps` for `d2`,
  and `_mm256_fnmadd_ps` for the high observation.
- The authoritative brief requires AVX2+FMA to alias the noncontracting scalar/base TPT graph
  with zero TPT contractions. Binding that token would change separately rounded TPT results and
  invalidate required scalar/W8 exact PCM/state/report parity; binding a different backend would
  violate the requested prepared-backend contract. Stop rather than weakening either rule. No
  test, Clippy, policy, audit, target/object, timing or benchmark command ran after this finding.

## Sol attempt 2 bank checkpoint — incomplete

- Corrected the brief-only blocker by inheriting the accepted Issue-008 TPT contract: base
  backends are bit-exact to scalar, while AVX2+FMA retains its existing exact three contraction
  sites and frozen tolerance. No backend is forked, disabled or silently substituted.
- Added the bounded W4/W8 bank implementation using only `PreparedTptBankKernelV1` and
  `PreparedCompressorGainMixKernelV1`, with complete request validation before fallback, per-track
  lane state, scalar-compatible filter flush/recovery, automation, reset and track snapshot/restore.
  Registry/effect-compiler/graph integration remains intentionally unmodified at this checkpoint.
- PASS: `cargo fmt --all -- --check`; locked core plus multiband tests (27 core, 4 multiband and one
  compile-fail doctest passed); warning-denied all-target core plus multiband Clippy. The existing
  four scalar product tests remain green; dedicated bank parity/state/recovery and graph closure
  evidence is still required before an overall verdict.
- No Issue-051 corpus/audit/target/object/benchmark/listening work ran;
  `timed_benchmark_invocations=0`. **Overall Issue 018 remains incomplete, not PASS.**

### Sol attempt 2 executed bank evidence checkpoint

- Added test-only native W8 evidence with eight distinct per-track states. Base AVX2 is bit-exact
  to scalar for PCM, complete state payloads and reports across flat recombination, isolated
  low/high and both-band compression, lookahead 0/5/20 ms, all link modes, one canonical
  automation Point, signed-zero identity and main-input sanitation. Executed AVX2+FMA remains
  within the frozen accepted sample bound.
- Valid track restore and both reset kinds preserve scalar/bank state parity; malformed filter-state
  restore rejects atomically. Directly injected nonfinite W8 filter state recovers only the matching
  left lane with output, state and report parity; other lanes/tracks remain unchanged.
- Exact retained envelopes are 23,544 bytes/track, 94,176 bytes/W4 and 188,352 bytes/W8. Wrong
  width/count rejects, every malformed request rejects before an unavailable-backend fallback, and
  a legal W4 backend returns `None` only when either accepted prepared kernel is unavailable.
- PASS: `cargo fmt --all -- --check`; locked core plus multiband tests (27 core, 7 multiband and one
  compile-fail doctest); warning-denied all-target core plus multiband Clippy. No production source
  changed in this evidence checkpoint. Registry/graph closure is still outstanding, so overall
  Issue 018 remains incomplete. `timed_benchmark_invocations=0`.

### Sol attempt 2 final registry/graph closure — PASS

- Added `miso.multiband-compressor` to the immutable launch registry and exact effect-compiler
  dependency policy. Baseline policy and mutations for arbitrary addition, removal and substitution
  of every approved launch effect pass.
- The accepted ten-track, 48 kHz/Q128 graph has no sidechain declaration or prepared sidechain
  port. It retains the host-width-correct full W4/W8 banks plus ordered scalar tails, with stable
  member IDs and exact post-bank scratch/runtime-buffer/metadata accounting. The scalar-only
  delegate has zero bank resources and the exact bank-resource delta appears in both incremental
  and session-plus-plan estimates.
- Bank and scalar-delegate graph schedules, route timing, inserted PDC and canonical bytes agree.
  Executed PCM stays within the inherited accumulated ten-track AVX2+FMA bound through the fixed
  960-sample latency, an active burst and a later release-state probe. Bypass retains identical
  bank membership, schedule, 960-sample route arrival, zero compensation and canonical bytes.
  A one-byte-below post-bank plan cap rejects before publication and returns all ten prepared
  effects and the complete compiled session.
- PASS: focused locked effect-compiler/graph-compiler all-target/all-feature tests (4 compiler,
  19 graph library, 2 graph binary tests and the 65,537-track scale row); focused warning-denied
  Clippy; shell syntax; effect-runtime baseline/mutations; `cargo fmt --all -- --check`; locked
  workspace all-target/all-feature check and tests; warning-denied workspace all-target/all-feature
  Clippy; warning-denied workspace rustdoc; workspace, realtime, graph and rack baseline/mutation
  policies; and `git diff --check`.
- **Final Issue 018 verdict: PASS.** The bounded fixed two-band launch product is complete. Issue
  051 exclusively retains expanded topology/qualification/audit/target/object/benchmark/listening
  work. No audit main, target matrix, object inspection, benchmark, timing or listening command
  ran; `timed_benchmark_invocations=0`.

## Audit #94 wave 0 — F1

- Deleted the scalar and W4/W8 enabled unity-gain dry/identity selections. Only immutable
  prepare-time bypass now selects delayed dry; enabled processing always emits the flushed LR4
  `low + high` sum. Dry rings, state layout and recovery are unchanged.
- Added `unity_gain_transition_has_no_step_at_crossover` (E0), whose pre-fix run failed at sample
  5120 with consecutive delta `0.8296956` against `0.0656`, and
  `unity_gain_output_is_the_delayed_lr4_sum` (E0b), whose independent four-section `f64` oracle
  run failed pre-fix with dry/LR4 error `0.8660186` at 1 kHz/48 kHz against `2e-5`.
- Amended `bypass_latency_automation_and_restore_are_transactional` (E0c) to preserve delayed
  bypass `-0.0` bits at sample 961. Canonicalizing sanitizer `-0.0` to `+0.0` made the gate fail
  with actual bits `0` against expected bits `2147483648`; the mutation was reverted.
- PASS: `cargo fmt --all -- --check`; `cargo clippy --locked -p
  miso-engine-multiband-compressor --all-targets -- -D warnings`; `cargo test --locked -p
  miso-engine-multiband-compressor`; `cargo test --locked -p miso-engine-graph-compiler
  launch_multiband_compressor_fixture_closes_bank_graph_and_transactional_caps`;
  `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p
  miso-engine-multiband-compressor`; `bash scripts/check-realtime-policy.sh`; `bash
  scripts/check-effect-runtime-policy.sh`; `bash scripts/check-workspace-policy.sh`; and `git diff
  --check`. The policy sources have no executable bit in this checkout, so they were invoked via
  `bash` without changing their modes. The identity-branch and signed-zero mutations were each
  executed once and reverted. No fixture was re-pinned, and no benchmark, timing, target matrix or
  listening command ran; `timed_benchmark_invocations=0`.
