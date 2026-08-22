# 057 Builtin direct and graph realtime audit closure

## Outcome

Close the sealed builtin candidate's launch-critical realtime proof with one direct-chain and one
production-graph million-call audit at the exact frozen 48-kHz/128-frame configuration.

## Context

This issue starts only after **Seal independent builtin corpus corruption and read-only
qualification** passes. It consumes that corpus without regenerating or changing expected bytes.
The prior combined audit/target issue was too broad: target builds, runtime backend selection and
instruction qualification move to **Builtin native, AArch64, and Wasm runtime-selection and
instruction qualification**.

This issue permits exactly one Terra attempt and one bounded Sol correction/review; a second
failure stops. Benchmark, workload and timing invocations are forbidden and remain zero.

## Scope

Finish only the direct builtin and production-graph audit tools, exact lifecycle/output records,
nine forbidden-operation detectors and probes, all-thread marker-delimited syscall validation,
focused tests and the proportional nonbenchmark policy seal. Both audits use 48,000 Hz and quantum
128 and execute exactly 1,000,000 production calls/renders.

## Required public interfaces/contracts

The direct audit calls the public prepared builtin chain. Its fixed early schedule starts a
257-update matrix ramp, retargets at the next block boundary with updates outstanding, injects the
frozen nonfinite input/target and paired filter-recovery cases, exercises both reset modes, and
then renders deterministic steady state. It drains two bounded meter sets off render and compares
PCM, state, sanitation/recovery/reset reports and all seven taps to the sealed corpus.

The graph audit compiles and binds the accepted session through `PreparedRenderPlan` and
`RealtimePlanOwner`; it never substitutes a direct `BuiltinChain`. The seven tap values are
pairwise distinct and the accepted side route has positive nine-sample PDC. The audit is bound to
manifest SHA-256 `bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`,
graph PCM SHA-256 `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`
and graph-meter SHA-256 `958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`.

Plan A renders block 1. Plan B applies exactly once before block 2 and displaces A into a
capacity-one retirement queue. Pending Plan C is then deferred while that queue is full and never
renders. Plan B renders blocks 2 through 1,000,000. The exact render counts are A=1, B=999,999 and
C=0; there is one applied swap and at least one explicit retirement-full deferral. After all armed
render markers close, the retirement owner destroys A and the control owner disposes of
never-applied C; no plan is destroyed on render.

The nine counted categories are allocation/reallocation, deallocation, lock/blocking sync,
feature detection, logging/formatting, file I/O, network I/O, other syscall and panic/unwind. Each
counter and their checked exact sum are zero in both audits. There is exactly one terminating
probe per category. The trace validator examines every traced thread, not only the thread that
writes markers, across every armed render interval; marker writes, publication, meter drains and
retirement occur outside those intervals.

## Deliverables

Deterministic direct and graph audit records; exact A/B/C lifecycle/destruction evidence; sealed
fixture/hash binding; nine probe results; all-thread trace result; focused audit tests and a clean
nonbenchmark policy/static seal.

## Explicit non-goals

Corpus or production DSP changes; rates other than 48 kHz; target builds; runtime backend
selection; object/instruction inspection; deployment adapters; benchmark/preflight/workload/
timing; performance claims; listening; or V1/legacy inspection.

## Dependencies by exact issue title

- Seal independent builtin corpus corruption and read-only qualification
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Production SIMD builtin bank graph retention and reachability qualification

## Acceptance gates with objective measurements

1. The direct record reports exactly 1,000,000 calls at 48,000 Hz/q128, stable backing addresses,
   exact sealed PCM/state/meter/report results and zero in all nine counters and their total.
2. The graph record reports exactly 1,000,000 renders at 48,000 Hz/q128, seven pairwise-distinct
   taps, positive PDC=9, the three frozen hashes, A=1/B=999,999/C=0, one applied swap, at least one
   full-retirement deferral, stable backing storage and no render-thread destruction.
3. A and C are destroyed by their declared off-render owners after the applicable marker; the
   active plan is retired or destroyed only after disarm.
4. Exactly nine terminating detector probes pass for each applicable audit entrypoint. The
   all-thread trace finds zero syscalls in every armed interval and rejects a syscall injected on
   either the render or auxiliary thread.
5. Focused audit/core/graph tests, format, warning-denied package Clippy, relevant realtime/graph
   policies and static no-artifact/no-workload checks pass on one candidate.

## Target matrix

One native Linux audit host at 48,000 Hz/q128. Cross-target and instruction evidence belong only
to the successor.

## Required evidence

Dependency/candidate/corpus identities; both exact million-count records; sealed payload hashes;
A/B/C and destruction-thread rows; nine-category counter/probe rows; all-thread trace hash;
commands/results and strict Terra/Sol verdicts; `workload_invocations=0` and
`timed_benchmark_invocations=0`.

## Terra attempt 1 evidence — FAIL pending bounded Sol correction

Attempt 1 added the nine-category realtime detector surface and corresponding direct/graph
terminating probes. The focused build plus both nine-probe scripts passed. The exact direct
million-call command was run once through a yielded PTY:

```text
cargo run --locked -p miso-engine-builtins-audit --bin miso_engine_builtins_audit
```

It completed `blocks=1000000` at `sample_rate_hz=48000` and `quantum_frames=128`, with all nine
serialized counters (`allocations`, `deallocations`, `locks`, `feature_detection`, `logs`,
`file_io`, `network_io`, `syscalls`, `panic_unwinds`) and `total_violations` equal to zero.

The single graph million-render command was also launched through a yielded PTY:

```text
cargo run --locked -p miso-engine-builtins-audit --bin miso_engine_builtins_audit_graph
```

It stopped at the first lifecycle assertion in
`tools/miso-engine-builtins-audit/src/graph_main.rs:284`: after Plan C is pending behind the full
retirement queue, subsequent Plan-B renders correctly report `DeferredRetirementFull`; the new
range helper incorrectly required `SwapOutcome::None`. No sealed corpus PCM, meter, or PDC
mismatch was observed before that stop. The graph million-run, all-thread trace, final policy
seal, and Clippy gates remain unrun.

This is a strict attempt-1 **FAIL**, awaiting only the single bounded Sol correction. No corpus or
production DSP bytes changed; `workload_invocations=0`; `timed_benchmark_invocations=0`; no
benchmark, preflight, workload, timing, target, or instruction command was invoked.

## Sol attempt 2 final evidence — FAIL / STOP

Sol reviewed clean checkpoint `376774f78c11a1fa6f67785cb9c20d9d7cdeb3dd` against the tracked
brief at `682b9e0`. The repeated retirement-full result is valid and exact: after A occupies the
capacity-one retirement queue and C is pending, render calls 3 through 1,000,000 must each return
`DeferredRetirementFull` while B continues rendering. A legal record therefore requires
A=1/B=999999/C=0, `swaps_applied=1`, `swaps_deferred=999998`,
`prior_plan_renders_on_deferred=999998` and `owner.deferred_count()=999998`. The preserved tool
instead requires `SwapOutcome::None` for calls 4 onward and serializes the three deferral values as
one.

That count correction is bounded, but the frozen direct acceptance gate cannot be completed
inside this issue's permitted audit-tool/corpus boundary:

- `BuiltinChain` exposes preparation, the three process sections, target update, reset and
  consumption into sections, but no state snapshot/restore or state-injection interface. The
  required exact retained-state comparison and paired nonfinite filter-state recovery injection
  therefore cannot be exercised through the declared public prepared-chain path.
- The sealed corpus contains functional PCM and meter payloads but no direct-audit state/report
  payload for the required 257-update schedule. Creating that expected payload would change the
  immutable Issue-064 input.
- The direct audit has one queue-full meter set and feeds the same final post-chain buffers to all
  seven accumulators. It does not expose or compare seven boundary taps, a successful-drain meter
  set, either reset result, the retarget schedule, sanitation, recovery or sealed PCM/state/report
  values.
- Both trace scripts remain nonqualifying: the direct script supplies unsupported `--blocks`; the
  graph script validates the obsolete two-swap/4+999996 lifecycle; and both inspect only the
  marker-writing trace file rather than every traced TID. Neither proves auxiliary-thread syscall
  rejection or preserves raw/validator hashes.
- The graph tool checks PDC metadata only. Its per-plan meter handles and reset generations differ
  from the sealed graph-meter payload, it does not compare graph PCM or seven snapshot bytes, and
  its destruction assertions do not prove the exact retirement/control/render owner roles.

Fixing the direct blocker requires a separately briefed test/qualification interface and sealed
expected-state/report evidence, or a truthful narrowing of the direct gate; either change is
outside this final correction and the immutable-input rule. Sol therefore made no partial code or
script correction and did not rerun either million-call entrypoint or either trace. The Terra
nine-probe results remain accepted technical input, but the incomplete direct, graph, lifecycle,
fixture and all-thread trace gates prevent an overall PASS.

Final verdict: **FAIL / STOP** after the allowed Terra attempt plus one bounded Sol correction
review. `workload_invocations=0`; `timed_benchmark_invocations=0`; no benchmark, preflight,
workload, timing, target or instruction command was invoked in either attempt. No production DSP,
corpus, expected bytes or audit source changed in the Sol attempt.

## Post-stop rescope

Issue 057 is **STOPPED / RESCOPED** with no overall PASS. Checkpoint `376774f` remains technical
input only: its nine detector categories, direct zero-counter million-call record and terminating
probe suites may be reused, but none substitutes for the missing functional, lifecycle or
all-thread evidence. The smallest successor is Issue 069, **Builtin prepared-chain and graph
realtime audit proof closure**. It alone owns the nonproduction qualification seam, separate
audit-only evidence fixtures, corrected direct/graph records and all-thread trace. Target and
instruction qualification remains downstream and no benchmark or timed workload is authorized.
