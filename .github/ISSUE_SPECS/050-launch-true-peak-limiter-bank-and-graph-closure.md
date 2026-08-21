# 050 Launch true-peak limiter bank and graph closure

## Outcome

Close the stopped Issue 016 launch product by carrying its accepted corrected scalar limiter into
homogeneous W4/W8 gain-apply banks, scalar tails, the native registry/effect compiler and one
accepted graph/PDC vertical. This issue does not redesign or requalify the detector or scalar gain
law.

## Context

Issue **Launch fixed-4x true-peak safety limiter** exhausted its two attempts without overall
PASS. Its final scalar checkpoint is accepted technical input only: the fixed BS.1770-5 Annex-2
four-phase detector, 1 dB guard, fixed latency `T=N+6`, corrected `H=L+6` attenuation hold, state
layout 1 and focused scalar evidence remain frozen. Banks, registry/graph wiring and their product
proof were not implemented and cannot be added as a third Issue-016 attempt.

This stateless successor has exactly **two total attempts**: one Terra implementation/review and,
if needed, one bounded Sol correction/review. A second failure stops. Render remains allocation-
free, lock-free and bounded; tracks and state remain dual-mono; no compiled track ceiling is
introduced. `timed_benchmark_invocations=0` and no benchmark is authorized.

## Scope

- Implement fixed-width homogeneous W4/W8 limiter banks. Each track retains the exact accepted
  scalar detector, histories, delay, gain, hold, automation and recovery state. Only the final
  delayed-sample/gain/identity selection uses the accepted packed gain kernel.
- Preserve scalar tails for every non-multiple track count and validate every bank member before
  an unavailable backend may return a legal `Ok(None)` fallback.
- Register `miso.true-peak-limiter` in the native effect registry/effect compiler without changing
  the descriptor, program key, quality, ports, scalar preparation or state payload.
- Retain a representative ten-track 48-kHz/128-frame graph fixture with exact width counts,
  latency-preserving bypass and exact integer PDC.
- Close only the remaining representative product state/resource/parity and final workspace/policy
  gates.

## Required public interfaces/contracts

The scalar `TruePeakLimiterFactory`, descriptor, parameter IDs/domains/order, rate set, FIR table,
gain law, fixed latency, tail, resets, recovery, state layout and scalar outputs are immutable
Issue-016 input. A bank implements the accepted `PreparedNativeEffectBank` contract at widths four
and eight and has byte-compatible per-track snapshot/restore state with scalar processing.

The limiter has required dual-mono `main-in` and `main-out` ports and **no sidechain**. A request
that invents a sidechain or otherwise changes the prepared topology is malformed; it is not a
legal connected-sidechain scalar fallback. Initial parameter values may differ per track/lane, but
all bank members must have the same immutable Normal-quality program signature.

At launch rates, exact accepted scalar state/resource values are:

| rate (Hz) | latency `T` | lane state bytes | dual-mono state bytes | fixed bytes |
|---:|---:|---:|---:|---:|
| 44,100 | 447 | 3,652 | 7,304 | 24 |
| 48,000 | 486 | 3,964 | 7,928 | 24 |
| 88,200 | 888 | 7,180 | 14,360 | 24 |
| 96,000 | 966 | 7,804 | 15,608 | 24 |

A width-`W` bank declares exactly `W * (dual_mono_state_bytes + 24)` effect-owned state/default
payload bytes. Width-specific member metadata, arrays and graph/AoSoA planes must additionally use
the already accepted checked compiler accounting; none may be hidden from preparation caps. State
and scratch remain independent of render quantum and source duration.

## Deliverables

- W4/W8 homogeneous limiter bank and exact scalar-tail behavior;
- registry/effect-compiler integration and policy allowlist/mutation updates if required;
- representative direct bank state/resource/parity tests and one ten-track graph/PDC/cap fixture;
- final nonbenchmark workspace and policy evidence with an explicit PASS/FAIL verdict.

## Explicit non-goals

Any scalar FIR, interpolation, guard, gain-law, hold, release, latency, parameter, state-layout or
resource redesign; sidechain; another quality/factor; clipping; reusable oversampling; expanded
corpus/oracle or parameter matrices; long sequences; 100,000-render audit; target or object
inspection; benchmark/preflight/timing/optimization; audition or listening. All qualification work
remains Issue 049, **Launch true-peak limiter qualification, realtime audit, and benchmark**.

## Dependencies by exact issue title

- Launch fixed-4x true-peak safety limiter
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Production SIMD builtin bank graph retention and reachability qualification
- Deterministic graph compiler, sends, submixes, sidechains, and PDC

The stopped Issue 016 and stopped Issue 008 contribute only their explicitly preserved scalar and
generic bank/API checkpoints. Neither stopped issue is treated as overall PASS.

## Sol implementation brief

**READY FOR TERRA ATTEMPT 1 after local/remote Issue 050 synchronization.** The tracked
authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/050-launch-true-peak-limiter-bank-and-graph-closure.md`. This checkpoint
authorizes no benchmark.

## Hazards/decisions

The detector and all nonlinear/state work remain scalar per track and lane. Packing only delayed
audio, gain and the identity mask into the already accepted W4/W8 multiply/select kernel avoids a
new SIMD FIR or oversampling framework and preserves scalar operation order. Bypass continues to
warm all scalar state while selecting delayed dry bits, so reported latency and graph PDC never
change.

## Acceptance gates with objective measurements

1. The scalar descriptor, FIR/gain/hold equations, output bits, state layout, rate rows and exact
   resource values remain unchanged. All four rate preparations and exact caps pass; one-byte-below
   state/default or graph capacity rejects transactionally.
2. Bank binding rejects wrong width/backend, malformed members, changed program/quality/ports and
   every invented sidechain before considering backend availability. A genuinely unavailable
   legal backend returns `Ok(None)` without consuming ownership or publishing partial state.
3. Same-target scalar and available W4/W8 processing are bit-identical for finite-normal inputs:
   PCM, complete per-track state and sanitation/recovery reports match across consecutive blocks.
   Reset, active snapshot/restore, bypass/identity warming, signed-zero identity and injected
   lane-local recovery preserve track and L/R isolation.
4. Width-specific retained state/default bytes equal `W * (dual_mono_state_bytes + 24)` at all four
   rates and match scalar state payloads track by track. Resource reports include every bank/member
   and AoSoA allocation with no padding track or compiled track ceiling.
5. The ten-track 48-kHz/128-frame fixture retains one W8 bank plus two scalar tails on W8, two W4
   banks plus two scalar tails on W4, and ten scalar instances otherwise. It proves stable
   membership/order, exact `T=486` enabled/bypass latency and PDC, consecutive-block scalar parity,
   unchanged graph/schedule bytes and full ownership return on cap failure.
6. Focused limiter/core/effect-compiler/registry/graph tests, formatting, warning-denied Clippy and
   one locked workspace check/test/Clippy/rustdoc seal pass with applicable workspace/realtime/
   effect-runtime/rack/graph policies and mutations. No Issue-049, audit, target/object,
   benchmark, timing or listening command runs; `timed_benchmark_invocations=0`.

## Target matrix

Execute scalar and the available native W8 backend on the candidate host; compile and test the W4
source contract through focused checks. Cross-target and named-instruction qualification belongs
only to Issue 049.

## Required evidence

Candidate/source identity; accepted Issue-016 scalar checkpoint identity; exact rate/state/resource
table; bank validation and unavailable-fallback results; scalar/W4/W8 PCM/state/report parity;
reset/restore/recovery/isolation rows; ten-track bank/tail/latency/PDC/resource/cap report; focused
and final workspace/policy outputs; attempt count; explicit Terra/final Sol PASS/FAIL; and
`timed_benchmark_invocations=0`.

## Terra attempt 1 bank implementation checkpoint — incomplete

- Base candidate: `34e4b7c`; the accepted Issue-016 scalar descriptor, FIR, guarded gain/hold
  law, latency, state layout, reset/recovery and scalar path were not changed.
- Added fixed-array `PreparedTruePeakLimiterBank<W>` binding and rendering for W4/W8. Every member
  is metadata/default-validated before the prepared gate-gain token is acquired; changed immutable
  program signatures reject, and a valid unavailable backend returns `Ok(None)`.
- Detector, linking, ramps, required-gain hold/release, delay, reports and state remain scalar and
  independent per track/lane. Only delayed sample/gain/identity selection uses the accepted packed
  `PreparedGateGainKernelV1`; no core API or kernel changed.
- Focused `cargo fmt --check --package miso-engine-core --package
  miso-engine-true-peak-limiter`, locked core+limiter library tests (27 core and 5 limiter), and
  locked all-target warning-denied Clippy for both packages: PASS.
- The direct W4/W8 scalar-parity, snapshot/restore, recovery/isolation and resource tests were
  intentionally not added after the scope-freeze instruction, and registry/graph work did not
  start. This is not a full Issue-050 PASS verdict. `timed_benchmark_invocations=0`.
