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

## Terra attempt 1 bank evidence checkpoint

- Candidate base: `8a8acd2`; test-only changes added no production, core, registry, graph, PDC,
  audit, target/object, benchmark, timing or listening behavior.
- `bank_binding_validates_before_fallback_and_retains_exact_width_bytes` covers all four rate
  width-resource rows, legal unavailable W4 fallback, malformed-member-before-fallback, immutable
  program mismatch, and invented-sidechain rejection.
- `executed_w8_matches_scalar_through_state_automation_and_lane_recovery` executed native W8 and
  compared it with eight scalar peers for DualMono and Maximum: guarded impulse and hold/release,
  per-track 0/5/10 ms lookahead, ceiling Point automation, sanitation, active transactional
  restore, both resets, and an injected left-lane `NaN` gain recovery with track/L/R report and
  payload isolation.
- `cargo fmt --check --package miso-engine-true-peak-limiter`, locked limiter library tests
  (7 passed), and locked all-target warning-denied limiter Clippy: PASS.
- This is a representative bank proof checkpoint, not final Issue-050 PASS: graph/registry/PDC,
  workspace/policy seals and Issue-049 qualification remain unrun. The bank signed-zero path has
  no separate W8-only assertion in this frozen tranche. `timed_benchmark_invocations=0`.

- Follow-up signed-zero checkpoint on `5d610f6`: executed W8 bypass/identity processing preserved
  track 0 left `-0`/right `+0` and track 1 left `+0`/right `-0` exactly at fixed sample 486, with
  scalar-peer PCM/report parity. Limiter fmt, locked tests (8 passed), and all-target `-D warnings`
  Clippy passed; `timed_benchmark_invocations=0`.

## Terra attempt 1 final candidate — PASS pending Sol review

- Candidate base: `f293b41`. The accepted Issue-016 scalar descriptor, 48-tap detector, guarded
  gain/hold/release law, fixed `T=N+6`, snapshot layout, reset/recovery and scalar output were not
  changed. The four launch-rate state/default rows and W4/W8 exact retained-byte tests remain the
  direct bank evidence recorded above.
- `miso.true-peak-limiter` is now an approved direct dependency of the injected launch registry
  and is registered beside EQ, compressor and gate/expander. The effect-runtime dependency
  allowlist and its missing/substituted-dependency mutation coverage were updated accordingly.
- The one accepted ten-track 48-kHz/128-frame graph fixture uses homogeneous Normal,
  no-sidechain limiter programs with legal per-track/lane parameter differences. It verifies the
  selected native W8 shape (one full eight-member bank plus `eq8`/`eq9` scalar tails; W4/scalar
  expectations remain width-conditional), stable ascending membership, ten independent scalar
  control instances, exact PCM over the one-shot guarded-impulse/fixed-latency/release sequence,
  and unchanged schedule, PDC, inserted-delay and canonical graph bytes.
- Every enabled and bypassed fixture instance declares and routes exactly `T=486`; all route
  arrivals are 486 with zero compensation. Bypass preserves the same prepared-bank count,
  schedule, PDC and canonical bytes. A plan cap one byte below the checked final estimate rejects
  with `graph.resource.limit` and returns all ten prepared effect inputs, without publishing a
  graph.
- PASS: focused formatting; locked core (27), limiter (8), effect-compiler (4), and graph
  compiler (18) tests; all-target warning-denied focused Clippy; workspace/realtime/effect-runtime/
  rack/graph policy and mutation scripts; locked workspace check, test, all-target Clippy and
  warning-denied rustdoc. No Issue-049 corpus/audit/target/object/timing/listening command ran;
  `timed_benchmark_invocations=0`.

## Final Sol correction attempt 2 — PASS

- Reviewed clean candidate `b512d2b` against this issue, its authoritative brief and the accepted
  Issue-016 scalar checkpoint. Bank member validation precedes unavailable-backend fallback;
  estimator/hold/release operation order, scalar/W8 PCM and complete state/report parity,
  transactional restore, both resets, bypass/signed-zero warming, injected lane recovery,
  registry membership, ten-track cohort/tails and exact `T=486` PDC are non-vacuous and unchanged.
- Found one product blocker in the candidate evidence: graph caps and the reported plan estimate
  were evaluated before effect-bank binding and therefore omitted the bank's four retained AoSoA
  planes, additional simultaneous member output buffers and bank/member metadata. The claimed
  one-byte-below post-bank cap was checking only the lower scalar graph estimate.
- The bounded correction binds the selected banks transactionally before final resource
  publication, derives every count/size with checked arithmetic and folds exact bank scratch,
  member-buffer and metadata values into audio-sample, graph-byte, plan-byte, session-byte and
  largest-single-allocation checks. Overflow or any corrected cap failure drops the unpublished
  bank and returns the complete prepared scalar effect session. Target-neutral semantic graph
  bytes remain unchanged.
- On the accepted 48-kHz/q128 W8 fixture the report now records one retained bank, exactly 16,384
  AoSoA scratch bytes and 8,192 additional runtime member-buffer bytes, plus the checked target-
  layout bank/member metadata. The strengthened test independently derives those values, proves
  the exact delta from the scalar report, then rejects `incremental_plan_bytes-1` with
  `graph.resource.limit` while returning all ten prepared limiter inputs.
- Focused formatting/check, graph (11), graph-compiler (18 plus fixture/doctests), source (29), and
  warning-denied graph/compiler/source Clippy: PASS. The first workspace check exposed only two
  aggregate-literal compile omissions in audit tools; adding the four new fields as zeros was a
  mechanical compatibility repair and changed no audit behavior.
- Final locked workspace all-target/all-feature check and tests: PASS. Warning-denied workspace
  all-target/all-feature Clippy and rustdoc: PASS. Workspace, realtime, effect-runtime and rack
  baseline/mutation policies plus graph policy: PASS. `git diff --check`: PASS.
- No functional audit main, Issue-049 corpus, target/object inspection, benchmark/preflight,
  timed workload or listening command ran. `timed_benchmark_invocations=0`.

**Final Sol verdict: PASS.** Issue 050 closes the bank/registry/graph launch-product vertical over
the preserved Issue-016 scalar contract. Issue 049 remains the sole owner of deferred
qualification, realtime audit, target/instruction, benchmark and listening work.
