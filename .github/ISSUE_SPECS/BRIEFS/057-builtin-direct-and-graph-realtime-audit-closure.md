# Sol implementation brief — issue 057 builtin direct and graph realtime audit closure

## Decision and budget

**READY after exact-title PASS of Seal independent builtin corpus corruption and read-only
qualification.** Use one Terra implementation attempt and one bounded Sol correction/review; a
second failure stops. This is a 48-kHz/q128 realtime-audit issue only. Workload, benchmark and
timing invocation counts remain zero.

## Immutable inputs

Consume the sealed Issue-064 corpus without authoring or expected-value changes. Pin:

- manifest `bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`;
- graph PCM `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`;
- graph meters `958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`;
- exactly seven pairwise-distinct taps and the accepted nine-sample positive-PDC route.

Stop rather than changing production DSP, expected bytes, rates, quantum, graph semantics or the
sealed corpus.

## Direct audit

At 48,000 Hz/q128, execute exactly 1,000,000 public prepared-builtin calls. Preserve the frozen
257-update matrix ramp, next-boundary retarget while active, nonfinite input/target cases, paired
filter recovery, both resets and deterministic steady state. Exercise successful drain and
queue-full/drop meter sets across all seven taps. Compare exact PCM, state, sanitation/recovery/
reset reports and meter counters with the sealed fixtures; count/address-only evidence is
insufficient.

## Graph and lifecycle audit

Use production session/effect/builtin compilation, graph binding, `PreparedRenderPlan` and
`RealtimePlanOwner`. Bind only genuine external source/output and declared fixture processors;
never invoke `BuiltinChain` directly from this audit.

Freeze this exact schedule inside the collectively armed render intervals:

1. render Plan A for block 1;
2. publish/apply Plan B at the block-2 boundary, retiring A into a capacity-one queue;
3. publish Plan C, observe `DeferredRetirementFull`, and never apply or render C;
4. render B for blocks 2 through 1,000,000; and
5. after disarm, reclaim/destroy A on the retirement owner and dispose of C on the control owner.

Require `renders=1000000`, A=1, B=999999, C=0, `swaps_applied=1`, at least one explicit
retirement-full deferral, stable buffer/state addresses, exact fixture PCM/meters/PDC and no
render-thread destruction. The currently preserved tool's two-swap `4/999996` lifecycle is not
acceptable evidence.

## Detectors and all-thread trace

Expose and serialize exactly nine categories: allocation/reallocation, deallocation, lock,
feature detection, log/format, file I/O, network I/O, other syscall and panic/unwind. All counters
and their checked sum are zero. Add one terminating mutation probe for each category; a normal
probe return fails.

Use explicit prepared/armed/disarmed markers outside the realtime calls. The trace validator must
scan all traced TIDs across every armed interval, pair markers deterministically and reject a
syscall injected on either the render thread or an auxiliary thread. Publication, drains,
retirement and marker I/O remain outside armed intervals. Preserve raw trace and deterministic
validator-output hashes as evidence.

## Ordered gates and stop rules

First make direct fixture comparisons and all nine probes executable. Then correct the graph to
the exact A/B/C schedule and fixture-bound seven-tap/PDC result. Run focused audit/core/graph tests,
both probe suites, both all-thread trace validators, format, warning-denied package Clippy and the
relevant realtime/graph policy and static no-artifact/no-workload scans.

Stop on any sealed numerical mismatch, inability to observe A=1/B=999999/C=0, render-thread
destruction, detector escape, all-thread syscall, or second failed attempt. Do not add target,
instruction, benchmark or listening work. PASS unblocks **Builtin native, AArch64, and Wasm
runtime-selection and instruction qualification** only.
