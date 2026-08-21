# 013 Launch feed-forward peak compressor

## Outcome

Deliver one launch-ready dual-mono feed-forward peak compressor with a soft-knee gain computer,
explicit detector linking, optional external sidechain, fixed-latency lookahead window, scalar
processing, homogeneous four/eight-track banks, scalar tails, and a public registry-to-graph
vertical.

## Context

Engine V2 is greenfield and must never inspect or inherit V1/legacy. The realtime plane exclusively
owns a preallocated `PreparedRenderPlan`; render performs zero allocation/free, locks, feature
detection, I/O, logging, syscalls, panic/unwind, structural mutation, or data-dependent unbounded
work. Tracks and effect state are dual-mono. Launch rates are exactly 44,100, 48,000, 88,200, and
96,000 Hz. There is no implicit SRC or compiled track ceiling.

This issue consumes the accepted semantic native-effect runtime, effect compiler, graph/PDC, and
AoSoA bank seams. Issue 008 stopped overall; only its preserved architecture checkpoint and the
accepted production bank seam from Issue 037 are technical inputs. This issue has exactly **two
total attempts**: Terra attempt 1 and, if necessary, one bounded Sol correction/review. A second
failure stops and requires a stateless rebrief; gates may not be weakened.

## Scope

- Add effect `miso.compressor`, contract 1.0, state layout 1, and Normal quality at all four launch
  rates.
- Implement only feed-forward, instantaneous-peak detection and the documented dB soft-knee curve.
- Reuse `LinkMode::{DualMono, Maximum, Average}`. Linking shares only the instantaneous detector
  value; parameters, gain state, delay/history, recovery and state payload remain per lane.
- Expose the optional dual-mono `sidechain-in` port. An unconnected port detects the main input and
  is bank-eligible. A connected port detects the routed input and follows the accepted graph's
  scalar fallback.
- Report a fixed 20 ms integer latency at each launch rate. A preparation-only per-lane lookahead
  control selects 0–20 ms effective detector advance inside that fixed delay; bypass and parallel
  dry always preserve the full declared latency.
- Implement the exact scalar and homogeneous W4/W8 operation graphs, state/reset/recovery behavior,
  fixtures, public graph vertical, realtime audit, target evidence, and one descriptive benchmark
  invocation frozen by the tracked Sol brief.

## Required public interfaces/contracts

`CompressorFactory` implements `NativeEffectFactory`; its scalar and bank products implement the
accepted `PreparedNativeEffect` and `PreparedNativeEffectBank` traits. The descriptor has
`main-in`, `main-out`, and optional `sidechain-in` dual-mono planar ports. Prepared metadata is
immutable; latency, tail, state sizes, scratch/resource declarations, quality, link mode, bypass,
port topology, rate and quantum match the accepted runtime's expected metadata exactly.

Stable per-lane parameter IDs, in descriptor order, are:

| ID | control | unit | domain | default | mapping | automation/smoothing |
|---:|---|---|---:|---:|---|---|
| 1 | threshold | dB | -80..0 | -18 | linear | block Point; linear 64 updates |
| 2 | ratio | ratio | 1..20 | 4 | logarithmic | block Point; linear 64 updates |
| 3 | knee | dB | 0..24 | 6 | linear | block Point; linear 64 updates |
| 4 | attack | ms | 0.1..200 | 10 | logarithmic | block Point; linear 64 updates |
| 5 | release | ms | 5..5000 | 100 | logarithmic | block Point; linear 64 updates |
| 6 | makeup | dB | -24..24 | 0 | linear | block Point; linear 64 updates |
| 7 | mix | linear | 0..1 | 1 | linear | block Point; linear 64 updates |
| 8 | lookahead | ms | 0..20 | 5 | linear | none; preparation/state only |

The authoritative equations, sample ordering, fixed delay realization, exact state words and byte
counts, SIMD/FMA policy, sanitation and restore transaction are frozen in
`.github/ISSUE_SPECS/BRIEFS/013-compressor.md`.

## Deliverables

- one `miso-engine-compressor` package and the smallest direct core/effect-compiler/graph seams;
- descriptor, factory, scalar processor, W4/W8 bank, scalar tail and launch registry integration;
- independent `f64` reference, compact checked fixtures and deterministic differential tests;
- one ten-track registry/session/graph bank-plus-scalar-tail-plus-connected-sidechain fixture;
- one 100,000-render non-timed realtime audit and native/ARM/Wasm target/instruction evidence;
- workload-free benchmark preflight followed, only when authorized, by one invocation containing
  one warmup and two measured rounds; and
- checksummed audition PCM plus a blinded listening preregistration, without fabricated listeners.

## Explicit non-goals

Feedback topology, RMS or other detector averaging, hold, detector HPF/LPF, program-dependent or
multi-stage release, auto makeup, more quality modes, dynamically changing reported latency,
connected-sidechain banking, multiband/dynamic-EQ behavior, device/browser runtime qualification,
completed human listening, generic fixture/benchmark infrastructure, optimization, or any session,
wire, graph-topology, PDC, routing, effect-contract or package/CID redesign. These are follow-ups,
not partial gates for this bounded launch product.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Production SIMD builtin bank graph retention and reachability qualification
- Deterministic graph compiler, sends, submixes, sidechains, and PDC

The stopped Issue-008 dependency means its explicitly preserved architecture slice only; it does
not declare Issue 008 passed or import its failed benchmark claims.

## Sol implementation brief

**READY FOR TERRA ATTEMPT 1.** The tracked authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/013-compressor.md`. It freezes the exact two-attempt budget, algorithm,
parameters, fixed-latency lookahead, state/resources, operation graphs, fixtures, audit, targets,
benchmark protocol and stop rules.

Before implementation begins, root must synchronize GitHub Issue #13 to this exact title/body and
verify the remote issue remains open. This docs checkpoint performs no GitHub mutation.

## Hazards/decisions

The detector/gain-computer split, dB curve, and one-pole timing follow `[REISS-COMP]`; bounded delay
state follows `[SMITH-SASP]`; immutable reported latency and bypass preservation follow
`[VST3-LATENCY]`. Production does not import the independent reference. The nonlinear gain
computer remains a bounded scalar lane step inside a homogeneous bank; a narrow prepared core
gain/mix microkernel supplies explicit W4/W8 arithmetic without inventing SIMD transcendental
approximations. AVX2+FMA has zero contractions for this V1 operation graph.

## Acceptance gates with objective measurements

1. Descriptor, preparation, session, sidechain, link, quality, port, parameter, resource and
   metadata mutations reject transactionally with stable diagnostics at every launch rate.
2. Static curves agree with independent `f64` within 0.01 dB. Attack/release traces agree within
   0.005 dB and their declared time constants within the greater of one sample or 2%.
3. Fixed latency and enabled/bypass/parallel-dry impulses land at the exact declared sample;
   detector advance, external sidechain and all three link modes match independent fixtures.
4. Exact 64-update/restart automation, malformed-span handling, both resets, all-or-none state
   continuation, signed-zero identity, sanitation, bounded recovery, and L/R/track isolation pass.
5. Ten thousand seeded legal configurations and twelve frozen million-sample valid sequences stay
   finite; valid sequences report no recovery. Invalid audio/state probes recover only the affected
   lane with saturating telemetry.
6. Same-target scalar/W4/W8 non-FMA output and carried state are bit-identical for finite-normal
   no-sanitation inputs; cross-target comparison passes the brief's fixed tolerance. Named object
   inspection proves scalar, W4, W8 and zero-FMA operation graphs.
7. The public ten-track graph retains exact host-selected full bank(s), the connected-sidechain
   scalar fallback and unconnected scalar tail without graph/PDC/schedule/observer changes.
   Exactly 100,000 prepared 128-frame renders report zero forbidden operations while armed;
   destruction is off-render.
8. Focused and locked full workspace tests, warning-denied Clippy/rustdoc, policies/mutations, native
   baseline, x86 AVX2/AVX2+FMA, Android/iOS AArch64, and Wasm scalar/simd128 gates pass.
9. Benchmark preflight launches zero workloads. Only after all nonbenchmark gates and explicit root
   authorization, the frozen command runs exactly once with one untimed warmup and two measured
   rounds, no timing threshold, tuning, retry, or overwrite.

## Target matrix

Native scalar; runtime-gated x86 AVX2 and AVX2+FMA (zero compressor contractions); AArch64 NEON W4;
wasm32 scalar and base `simd128` W4 with no relaxed-SIMD dependency. Cross-target results are
compile/instruction claims, not device/browser runtime claims.

## Required evidence

Candidate/source hashes; descriptor/equation/state/resource tables; reference and fixture hashes;
static/envelope/latency/link/sidechain, seeded, million-sample, automation/reset/restore/recovery and
isolation maxima; scalar/bank/target instruction reports; exact graph/audit counters and PCM hash;
benchmark preflight launch count and, only after the sole authorized invocation, six-record JSONL
hash/count; audition/preregistration hashes; and explicit Terra plus final Sol PASS/FAIL verdicts.

## Terra attempt 1 checkpoint evidence — scalar and homogeneous-bank foundation

The scalar foundation and its approved gain/mix-token prerequisite are checkpointed. The current
homogeneous-bank slice prepares only exact-width, same-program, unconnected-sidechain requests and
returns the existing legal scalar fallback for unavailable architecture or connected sidechain.
Prepared bank state retains width-specific independent `Lane` storage and uses the core prepared
gain/mix token for the frozen dry/wet/mixed selection graph. Dry identity takes precedence over
wet identity, preserving the scalar `G == 0`/positive-zero-makeup rule without overlapping masks.

Focused evidence run by Terra (no benchmark invocation):

- `cargo check --locked -p miso-engine-compressor` — PASS.
- `cargo test --locked -p miso-engine-compressor --lib` — PASS, 7 tests.
- `cargo clippy --locked -p miso-engine-compressor --all-targets -- -D warnings` — PASS.
- The native available W8 test compares eight unconnected scalar instances with one AVX2 bank for
  eight 128-frame blocks, asserting bit-exact planar PCM and byte-exact per-track state payload
  for track 3. It also exercises independent per-track input/state evolution.

This is not a final Issue-013 verdict. W4 runtime evidence, cross-target instruction inspection,
connected-sidechain scalar fallback in a production graph, corpus/seeded/million-sample evidence,
fixtures, realtime audit, target matrix and benchmark remain unrun and unclaimed.

## Terra attempt 1 checkpoint evidence — registry and graph vertical

The caller-injected V1 launch registry now contains `miso.parametric-eq` and
`miso.compressor`; no render-reachable global catalog was introduced. The focused graph test builds
an accepted ten-track compressor session from the existing accepted nine-track session shape: nine
unconnected compressor tracks form an exact-width bank plus scalar tail on bank-capable dispatch,
and one routed `sidechain-in` compressor is retained as a scalar fallback. It renders sixteen
consecutive 48-kHz/128-frame blocks through both the retained-bank graph and a test-only
scalar-delegate registry, requiring bit-exact PCM. The test also verifies fixed 960-sample effect
metadata, unchanged schedule/route-PDC records relative to scalar dispatch, and unchanged metadata
and PDC records with all compressor instances bypassed.

Focused evidence run by Terra (no benchmark invocation):

- `cargo fmt --check --package miso-engine-effect-compiler --package miso-engine-graph-compiler` — PASS.
- `cargo test --locked -p miso-engine-effect-compiler --test native_session` — PASS, 4 tests.
- `cargo test --locked -p miso-engine-graph-compiler --lib` — PASS, 16 tests.
- `cargo clippy --locked -p miso-engine-effect-compiler -p miso-engine-graph-compiler --all-targets -- -D warnings` — PASS.

This remains a bounded graph integration checkpoint only. It does not claim a checked corpus,
full target/runtime qualification, the realtime audit, seeded or million-sample matrices, or any
benchmark/listening result.

## Sol correction attempt 2 — launch-product checkpoint

**PASS for the bounded product correction; not an overall qualification verdict.** Detector-delay
tap derivation now occurs only at prepare, restore, and full reset. Restore accepts every finite,
in-domain parameter value accepted by preparation/automation while retaining transactional
two-lane replacement. Every bank member is validated before a legal unavailable-backend or
connected-sidechain/heterogeneous-program scalar fallback can return. The bank retains independent
per-track `Lane` state while preserving sample-major AoSoA audio and the packed W4/W8 gain/mix
kernel.

At each launch rate, the checked descriptor identity is `lane_bytes = 4 * (24 + 2 * (N + 1))`;
prepared retained payload plus defaults is exactly `W * (2 * lane_bytes + 64)` bytes for W4/W8.
The independent oracle renders 2,048 samples past the 960-sample 48-kHz latency and proves active
gain reduction. The frozen ten-track graph now asserts width-correct W8 `1 bank + 2 scalar` and W4
`2 banks + 2 scalar` shapes.

Focused candidate evidence (no benchmark or timed command):

- `cargo test --locked -p miso-engine-compressor --lib` — PASS, 10 tests.
- focused graph fixture, core compressor kernel, and registry integration tests — PASS, 1/3/1.
- `cargo fmt --all -- --check` — PASS.
- warning-denied focused four-package all-target Clippy — PASS.

Expanded corpus, 10,000/randomized and million-sample matrices, realtime audit, target/object
qualification, benchmark, and completed listening were not run or claimed; transfer them to the
planned stateless Issue 046 qualification successor. `timed_benchmark_invocations = 0`.

## Final Sol product-closure evidence

**PASS.** Candidate `bd33226` passed formatting; locked workspace all-target/all-feature check;
locked workspace all-feature tests; warning-denied workspace all-target/all-feature Clippy;
warning-denied workspace rustdoc; workspace/realtime policies and mutations; graph policy; rack
policy and mutations; clean diff; and static no-artifact scans. No functional audit main, target
matrix, object inspection, benchmark/preflight, or timing command ran.

The sole closure blocker was a stale effect-runtime policy allowlist. The bounded correction makes
the exact effect-compiler dependency set compressor, core, effect-contract, parametric EQ, and
session. Its mutation suite first proves the unmodified fixture passes, then rejects an arbitrary
extra dependency and the removal or substitution of either approved native effect. Final focused
evidence:

- shell syntax, effect-runtime baseline and mutation policy, and `git diff --check` — PASS;
- `cargo check --locked -p miso-engine-effect-compiler` — PASS; and
- `cargo test --locked -p miso-engine-effect-compiler --test native_session` — PASS, 4 tests.

This closes the launch-product contract only. The explicitly deferred qualification matrix remains
owned by planned stateless Issue 046. `timed_benchmark_invocations = 0`.
