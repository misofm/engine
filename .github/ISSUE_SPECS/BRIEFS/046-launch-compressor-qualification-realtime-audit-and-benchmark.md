# Sol implementation brief — issue 046 launch compressor qualification, realtime audit, and benchmark

## Decision and attempt budget

Qualify the exact accepted Issue-013 product; do not repair or redesign production. One Terra
qualification attempt and at most one bounded Sol correction to test/tool/evidence code are
available. A production defect immediately FAILs and moves to a new stateless product issue. A
second failed qualification attempt stops.

No timed compressor workload is authorized at briefing. Keep
`timed_benchmark_invocations=0` until all nonbenchmark gates are green and root explicitly
authorizes the single frozen command.

## Frozen candidate and boundaries

Issue 013 owns all production semantics. This issue may add only the compressor reference, corpus,
fixture/checker, functional audit, target/instruction checks, benchmark runner/preflight/validator,
audition assets and evidence needed below. Reuse existing conformance, graph and realtime audit
seams; do not create parallel general frameworks.

The oracle must be an independently written test/tool-only `f64` implementation of the Issue-013
equations, ordering, one-pole envelope and delay. It cannot call production helpers or read
production state/coefficient values, and production cannot depend on it. A deterministic source-
boundary mutation check must reject either direction of leakage.

## Checked corpus and objective matrices

Create exactly one sorted `fixtures/compressor/v1` manifest with safe relative paths, exact byte
lengths and lowercase SHA-256. Cover static curves, step and sine-burst envelopes, latency/
lookahead/bypass/mix impulses, main versus external sidechain, asymmetric
DualMono/Maximum/Average, automation/reset/restore continuation, sanitation/recovery and short
audition PCM. The checker rejects missing, changed, unlisted, unsafe-path and coverage-invalid
files. Plots are derived evidence, not another corpus.

Freeze these comparisons:

- detector sweep -160..24 dB at thresholds `[-80,-18,0]`, ratios `[1,2,4,20]` and knees
  `[0,6,24]`: maximum curve/gain-reduction error `<=0.01 dB`;
- every launch rate and attack/release minimum/default/maximum: trace error `<=0.005 dB`, with the
  `1-exp(-1)` crossing within the greater of one sample or 2%;
- lookahead `[0,5,20]` ms: enabled, bypass and mix-zero impulses land at declared latency `N`,
  detector action advances by exact rounded `L`, and all links plus connected sidechain match;
- block partitions `1/63/64/127/128`, consecutive blocks and restart Points: exact 64-update,
  partition-invariant continuation and byte-identical snapshot/restore;
- signed-zero identity, malformed spans/payloads, both resets, finite/subnormal sanitation,
  bounded lane-local recovery and L/R/track isolation; and
- same-target finite-normal scalar/base-W4/W8 output/state/report equality, with cross-target bound
  `abs(error) <= 1e-6 + 2e-5*abs(reference)` and the same bound for zero-FMA AVX2.

Run exactly 10,000 legal configurations from seed `0x000000000013c0de`, spanning every launch
rate, port, link and parameter edge. Freeze the generator before candidate evaluation and record
its transcript hash and all maxima.

Run exactly twelve one-million-sample rows: four launch rates by three link modes. Each row drives
unconnected and connected scalar instances and an available unconnected bank using bounded finite-
normal asymmetric audio and extreme block Points. Valid rows remain finite and report zero
recovery. Invalid audio/state probes are separate expected sanitation/rejection/recovery evidence.

## Cohort, graph and determinism proof

Use the accepted ten-track 48-kHz/128-frame public fixture: nine homogeneous unconnected tracks and
one connected-sidechain scalar track. W8 retains one bank, one unconnected tail and one connected
scalar; W4 retains two banks, one unconnected tail and one connected scalar; scalar dispatch
retains ten scalar bindings.

Add counts `1,2,3,4,5,7,8,9,17`. Assert exact stable IDs/order, membership, program keys, absent-
slot identity, no padding, scalar remainders, resource reports and transactional one-byte-below cap
ownership return. Across fresh equivalent preparations, freeze canonical graph, schedule,
topological/reduction/observer ordering, route/sidechain/main PDC, PCM, carried state and report
hashes. A 65,537-track case is control-plane no-ceiling evidence, never an audio workload.

## Exact non-timed realtime audit

Use the real production registry, graph compiler, prepared graph, compressor bank and scalar
fallback at 48 kHz/128 frames. Before arming, assert backend/width/members/bank/scalar metadata,
prepared capacities and stable backing addresses. Render exactly 100,000 blocks.

While armed, allocation, deallocation, lock, feature detection, logging, file/network I/O, syscall,
panic/unwind and structural-mutation counters are exactly zero. Retained address-free counters
prove both the real bank kernel and scalar fallback executed. Disarm before reading counters,
retiring or destroying the plan/effects. This functional audit records no clock or timing value.

## Target and instruction gates

On one unchanged candidate, prove:

- native scalar baseline and runtime-gated x86 AVX2 and AVX2+FMA;
- Android/iOS AArch64 NEON W4 compile and named packed operations;
- wasm32 scalar and base `+simd128` W4 compile and named SIMD/bitselect operations;
- scalar nonlinear lane work plus packed W4/W8 gain/mix, with no hidden scalar bank delegate;
- AVX2+FMA has zero compressor contractions and the frozen noncontracting operation count; and
- Wasm correctness has no relaxed-SIMD dependency.

Cross-compilation is compile/instruction evidence, not hardware/device/browser execution. Do not
forge unavailable runtime capability results.

## Ordered nonbenchmark gates

1. Freeze and validate the independent reference, corpus manifest and boundary mutations.
2. Run all compact and expanded comparison matrices, then the exact 10,000 and twelve-million-row
   gates once on the candidate.
3. Run scalar/W4/W8 state/report/isolation and cohort/graph/determinism gates.
4. Run the exact 100,000-render functional audit without timing.
5. Run target compile and named instruction/object gates.
6. Run formatting, focused/full locked tests, warning-denied Clippy/rustdoc and relevant workspace,
   realtime, effect-runtime, rack, graph, research and compressor policies/mutations.
7. Run workload-free benchmark argument/schema/persistence/shell-failure/overwrite preflight and
   assert `workload_launches=0`.

Any failure leaves the benchmark unauthorized.

## Descriptive benchmark protocol

After every ordered nonbenchmark gate passes, root may explicitly authorize exactly one invocation:

```text
bash scripts/run-compressor-benchmark.sh
```

The command accepts no arguments and refuses overwrite. It performs one untimed warmup and exactly
two measured rounds with no retry, tuning, optimization loop or timing threshold. Each round has
1,000 observations for exactly three 48-kHz/128-frame workloads: one unconnected DualMono scalar
track; one full host-selected unconnected Maximum-link bank; and the ten-track production graph
bank plus connected-sidechain scalar fallback.

Exactly six JSONL records report nearest-rank min/p50/p95/p99/p99.9/max ns/frame/track, cycles when
available, backend/width, semantic fixture/build hashes, allocation/free counters, CPU, OS,
governor/power, Rust/LLVM, target features, optimization/LTO/codegen and explicit missing metadata.
Preserve the first raw output if promotion or validation fails and do not rerun. Runner repair/
promotion moves to a tooling issue; performance observations move to weekly optimization.

## Audition and listening handoff

After objective sealing, generate checksum-stable level-matched audition PCM for slow/fast attack
and release, hard/soft knee, parallel mix, asymmetric links and external sidechain. Record the
matching method and an answer-key-separated blinded preregistration using `listening/TEMPLATE.md`.
Do not fabricate listeners, preferences, confidence or sound-quality claims. Completed human
listening is nonblocking and may occur only as a separately recorded follow-up.

## Stop conditions and evidence

FAIL for a production semantic change, production-derived oracle, mutable corpus/seed/count,
shortened rows, tolerance/domain change, hidden f64 production state, connected-sidechain banking,
runtime instrumentation, relaxed SIMD, any compressor FMA contraction, timed audit, benchmark
before authorization, retry/overwrite, synthetic listening or a third attempt.

Record candidate/source hashes; manifest/reference/transcript hashes; all maxima and first failures;
cohort/graph/audit counters and identities; target/instruction outputs; policy results; preflight
`workload_launches=0`; current `timed_benchmark_invocations`; and, only after authorization, raw/
accepted benchmark hashes and six-record count; audition/preregistration hashes; and explicit Terra
and final Sol PASS/FAIL verdicts.
