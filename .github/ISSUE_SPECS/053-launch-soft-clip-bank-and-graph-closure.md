# 053 Launch soft-clip bank and graph closure

## Outcome

Close the stopped Issue 019 launch product by carrying its accepted fixed-2x cubic scalar checkpoint
into homogeneous W4/W8 banks, scalar tails, the launch registry/effect compiler, and one accepted
graph/PDC/resource vertical. Preserve the scalar algorithm and state without redesign.

## Context

Issue **Launch fixed-2x cubic soft-clip saturator** exhausted its Terra and Sol attempts without an
overall PASS. Checkpoint `e674d5e` is accepted technical input only: the Normal-only descriptor,
fixed 2x FIR/cubic scalar path, exact coefficient bits, 64-update automation, layout-1 state,
31-sample latency, finite tail 29, reset/recovery behavior, and focused scalar evidence remain
frozen. Its missing bank/registry/graph product gates cannot be added as a third Issue-019 attempt.

This stateless successor has exactly **two total attempts**: one Terra implementation/review and,
if needed, one bounded Sol correction/review. A second failure stops. Render remains allocation-
free, lock-free and bounded. `timed_benchmark_invocations=0`; no benchmark is authorized.

## Scope

- Implement homogeneous W4/W8 `miso.soft-clip` banks with independent dual-mono/per-track state,
  exact scalar operation order per vector lane, and scalar tails for every track count.
- Execute the one already-frozen representative alias row needed to support the product's
  “antialiased” claim; do not add a corpus or parameter matrix.
- Register the effect in the launch native registry/effect compiler and update only its exact
  dependency-policy allowlist and mutations.
- Retain one ten-track 48-kHz/128-frame graph fixture proving width-correct banks and tails,
  latency/tail/bypass/PDC, scalar-delegate PCM/state, and transactional post-bank capacity.
- Close the focused and clean nonbenchmark workspace/policy gates.

## Required public interfaces/contracts

The Issue-019 scalar descriptor, parameter IDs/domains/order, table and arithmetic, automation,
ports, latency/tail, snapshot bytes, reset/recovery rules and scalar output are immutable. The
effect is Normal-only, DualMono, has required dual-mono `main-in`/`main-out`, and has no sidechain.

Each bank track owns the exact scalar state: 676 bytes per lane, 1,352 bytes dual-mono, plus 24
bytes of retained reset defaults. Therefore exact effect-owned retained bytes are:

| width | retained effect bytes |
|---:|---:|
| W4 | 5,504 |
| W8 | 11,008 |

W4 uses the accepted Wasm-simd128/NEON width and W8 uses AVX2. Base and AVX2+FMA selections execute
the same frozen multiply-then-add graph with zero contractions. Every member is validated before a
legal unavailable backend may return `Ok(None)`; malformed requests never become fallback. No
padding track, shared lane state, compiled track ceiling, hidden allocation, or changed state
payload is permitted.

Bypass and mix-zero/unity-output preserve the 31-sample delayed dry bits while warming complete wet
state. Enabled metadata remains `LatencySamples(31)` and `TailSamples::Finite(29)`, with final
causal support at base sample 60. Width metadata, AoSoA histories, member buffers and graph-owned
scratch are included through the accepted checked post-bank graph accounting.

## Deliverables

- W4/W8 homogeneous bank plus scalar-tail behavior and representative direct parity/resource tests;
- the single frozen alias-claim result;
- registry/effect-compiler and exact policy integration;
- one ten-track graph/PDC/cap fixture; and
- final focused/workspace/policy evidence with an explicit PASS/FAIL verdict.

## Explicit non-goals

Scalar DSP/table/domain/state redesign; another nonlinear mode, factor or quality; general
oversampling/SRC; expanded transfer/alias/rate/drive/mix corpus; 10,000 or million-sample rows;
100,000-render audit; cross-target or instruction inspection; benchmark/preflight/timing;
optimization; audition or listening. Those broad qualification surfaces remain Issue 052,
**Launch saturator/clipper qualification, realtime audit, and benchmark**.

## Dependencies by exact issue title

- Launch fixed-2x cubic soft-clip saturator
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Production SIMD builtin bank graph retention and reachability qualification
- Deterministic graph compiler, sends, submixes, sidechains, and PDC

The stopped Issue 019 contributes only its explicitly accepted scalar checkpoint, not an overall
PASS.

## Sol implementation brief

**READY FOR TERRA ATTEMPT 1 after local/remote Issue 053 synchronization.** The tracked
authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/053-launch-soft-clip-bank-and-graph-closure.md`. This checkpoint
authorizes no benchmark.

## Hazards/decisions

The bank is a fixed-width packing of the accepted effect-local 2x realization, not a reusable
oversampling framework. Vector lanes remain independent and use the scalar tap/polynomial order;
backend FMA capability must not change results. The representative alias row is retained because
without it the product cannot honestly claim measured antialiasing; every broader qualification
row is deferred.

## Acceptance gates with objective measurements

1. The accepted scalar descriptor, coefficient bits, output/state behavior and resource values are
   unchanged. The frozen 16,384-sample bin-3001, +18-dB-drive row records the 2x and naive-1x ratios
   and improves total nonfundamental/fundamental energy by at least 2.0 dB exactly as Issue 019
   specified.
2. Bank binding rejects wrong width/backend/count, malformed members, program/quality/port changes,
   invented sidechain and cap failure before unavailable fallback. Exact W4/W8 retained bytes pass;
   one byte below rejects transactionally without consuming ownership.
3. Available native bank PCM, complete per-track state and reports match scalar peers bit-exactly
   across consecutive blocks with representative per-track automation, bypass/identity warming,
   active restore, both resets, signed zero, sanitation and one injected lane-local recovery.
   Scalar tails remain ordered and isolated.
4. The ten-track fixture retains one W8 bank plus two scalar tails on W8, two W4 banks plus two
   tails on W4, or ten scalar instances otherwise. It proves stable membership, exact latency 31,
   tail 29/support 60, bypass/PDC/canonical stability, scalar-delegate PCM/state, corrected post-bank
   resource estimates and one-byte-below ownership return.
5. Focused soft-clip/core/effect-compiler/registry/graph tests, formatting, warning-denied Clippy,
   one locked workspace check/test/Clippy/rustdoc seal and applicable workspace/realtime/effect-
   runtime/rack/graph policies pass. No Issue-052, audit, target/object, benchmark, timing or
   listening command runs; `timed_benchmark_invocations=0`.

## Target matrix

Execute scalar and the available native W8 backend on the candidate host; compile and test the W4
source contract through focused checks. Cross-target and named-instruction qualification belongs
only to Issue 052.

## Required evidence

Accepted scalar checkpoint and candidate identities; exact W4/W8 resource rows; request mutation
and unavailable-fallback results; scalar/bank PCM/state/report and recovery/isolation rows; frozen
alias ratios; ten-track bank/tail/latency/tail/PDC/cap report; focused/final/policy outputs; attempt
count; strict Terra/final Sol verdict; and `timed_benchmark_invocations=0`.
