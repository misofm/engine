# 069 Builtin prepared-chain and graph realtime audit proof closure

## Outcome

Close the launch-critical 48-kHz/q128 builtin realtime proof with exact functional evidence for the
prepared chain, the production graph and the A/B/C plan lifecycle, without changing DSP or the
accepted Issue-064 corpus.

## Context

Issue 057 stopped after two attempts because its count-only direct tool could not observe retained
state, inject recovery or expose true tap boundaries, while its graph and trace validators remained
incomplete. This stateless successor consumes checkpoint `376774f` only as technical input. The
nine detector categories, direct zero-counter million-call result and two nine-probe suites may be
retained after adversarial validation; Issue 057 has no PASS.

This issue permits one Terra implementation attempt and one bounded Sol correction/review. A
second failure stops. `workload_invocations=0` and `timed_benchmark_invocations=0`; benchmark,
preflight, workload and timing commands are forbidden.

## Scope

Add the smallest test-configuration-only, nonproduction builtin qualification seam; author and seal a
separate deterministic audit-evidence fixture root; correct the direct and graph million-call
records; prove exact off-render plan destruction; complete both nine-probe suites and an all-thread
marker trace. Execute exactly 1,000,000 direct calls and 1,000,000 production-graph renders at
48,000 Hz with quantum 128.

## Required public interfaces/contracts

The qualification seam exists only inside `miso-engine-builtins` under `#[cfg(test)]`, where the
unit-test module already has legitimate access to private `BuiltinChain`, `InputBuiltins` and
`TptSvf` state. It creates no Cargo feature, exported item, production/session ABI, dependency edge
or linkable symbol. Its V1 test record has this exact order: left HPF `s1/s2`, left LPF `s1/s2`,
right HPF `s1/s2`, right LPF `s1/s2` as eight `u32` bits; matrix current `ll/lr/rl/rr` and target
`ll/lr/rl/rr` as eight `u32` bits; `remaining_updates: u32`; and lifetime left/right recovery
counts as two `u64`. One internal helper may inject only the two retained state words of one named
lane/filter section. Tests invoke the unchanged production process methods and compare exact state
and `BuiltinProcessReport`; the helper may not fork equations, coefficients, sanitation, reset or
recovery behavior.

The prepared-chain qualification schedule prepares left HPF/LPF at 100/1000 Hz, right HPF/LPF at
200/2000 Hz, identity
matrix and 257 updates. Call 1 targets `[0,1;1,0]`; before call 2 it retargets to
`[0.9,0.1;-0.1,0.9]` with updates outstanding. A nonfinite target is rejected atomically before
call 3, whose first left/right inputs are NaN/+infinity. Before call 4, the seam injects one
nonfinite retained state into left HPF and right LPF; recovery is lane/section local and reported
once. Discontinuity and full resets occur before calls 5 and 6 respectively. All other samples are
the frozen finite `(0.25,-0.5)` pair. Every call begins from that declared input rather than feeding
the prior output back as new input. The internal test owns the injected call-4 state/report row.
The external direct audit mirrors the public target/input/reset schedule with ordinary finite call 4
and zero recovery, then runs calls 7 through 1,000,000 in deterministic steady state.

The seven true tap/two-meter-set proof is a graph-compiler integration test, not a fabricated
`BuiltinChain` tap API. It binds two real `MeterRequest`s at each ordered tap `Input`,
`PostInputBuiltins`, `PostSimd1`, `PostDynamic`, `PostSimd2PreFader`, `PostFader` and
`PostMatrix`. One set is drained successfully and the capacity-one set is rendered full and proves
its exact drop count. The test also proves seven distinct accepted tuples, positive runtime PDC and
continued rendering after drain/full. Meter drains remain outside render.

Audit-only expected bytes live exclusively under
`tools/miso-engine-builtins-audit/fixtures/v1/`: `direct-schedule.pcm.f32le`,
`prepared-chain-state-report.jsonl`, `graph-meter-sets.jsonl`, `direct-result.json` and
`MANIFEST.tsv`.
An independent retained-f32/reference author writes only a unique scratch root; a read-only checker
validates canonical formats, exact state/report/meter semantics and manifest identities before the
tracked bytes are accepted. The root is not added to `fixtures/builtins/v1`, does not change its
manifest or its three frozen hashes, and is not referenced by benchmark inputs. After acceptance,
the audit reads but never authors it. Its manifest and four payload hashes are frozen in evidence
before either million-call command.

The graph audit uses session compilation, `PreparedRenderPlan` and `RealtimePlanOwner`. Plan A
renders call 1; B applies once and renders calls 2 through 1,000,000; C remains pending and never
renders. With A occupying the capacity-one retirement queue, every call 3 through 1,000,000 returns
`DeferredRetirementFull`. Require A=1/B=999999/C=0, `swaps_applied=1`,
`swaps_deferred=999998`, `prior_plan_renders_on_deferred=999998` and
`owner.deferred_count()=999998`. Plan-A block 1 is compared byte-for-byte to the accepted graph PCM
and seven meter records, including seven pairwise-distinct tuples and the exact positive PDC route
of nine samples. The accepted identities remain manifest
`bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`, graph PCM
`508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19` and graph meters
`958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`.

After every armed marker closes, the retirement owner reclaims and destroys A. The disarmed
control owner destroys active B and never-applied C. Deterministic role records prove exactly one
destruction for each plan, zero render-thread destruction and no destructor before its allowed
marker; raw nondeterministic thread IDs or addresses are evidence inputs, not canonical result
fields.

The detector record contains exactly allocation/reallocation, deallocation, lock/blocking sync,
feature detection, logging/formatting, file I/O, network I/O, other syscall and panic/unwind. Each
counter and their checked sum is zero, and each audit binary has one terminating probe per
category. Trace capture uses timestamped per-TID files. The validator pairs the direct audit's six
single-call early intervals plus one steady interval and the graph audit's A, B-apply, first-defer
and remaining-B intervals, then rejects every non-marker syscall whose timestamp overlaps any
armed interval on any traced TID. Hermetic mutations inject one render-thread and one auxiliary-
thread syscall and both must be rejected. Raw-trace and validator-output hashes are preserved.

## Deliverables

Feature-gated qualification seam and tests; separate sealed audit fixture root and read-only
checker; deterministic direct/graph records; exact lifecycle/destruction rows; nine detector probes
per entrypoint; all-thread trace validator/mutations; proportional nonbenchmark seal.

## Explicit non-goals

Production DSP, coefficients or runtime APIs enabled in normal builds; Issue-064 corpus or
benchmark-input changes; rates other than 48 kHz; target/runtime-selection/instruction work;
general snapshot interchange; graph redesign; benchmark/preflight/workload/timing; performance
claims; listening; or V1 inspection.

## Dependencies by exact issue title

- Seal independent builtin corpus corruption and read-only qualification
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Production SIMD builtin bank graph retention and reachability qualification

## Acceptance gates with objective measurements

1. Non-test builds contain no qualification seam or symbol. Internal focused tests prove the exact
   17-word-plus-two-counter state layout, atomic nonfinite-target rejection, both reset modes and
   lane/filter recovery isolation; graph-compiler integration proves seven real ordered taps and
   both seven-meter-set outcomes.
2. The separate audit fixture author/checker proves independent provenance, canonical bytes,
   read-only operation and exact manifest/payload hashes without changing any Issue-064 or
   benchmark-input byte.
3. The direct record proves exactly 1,000,000 calls, the frozen schedule, exact fixture PCM/state/
   report/meter/result matches, stable storage and zero in every detector category and total.
4. The graph record proves exactly 1,000,000 renders, accepted graph bytes/hashes, seven distinct
   taps, PDC=9, A=1/B=999999/C=0, exactly 999,998 deferrals, one applied swap, stable storage and
   exact off-render owner destruction.
5. Both nine-probe suites pass. Both real all-thread traces contain no forbidden syscall in an
   armed interval; synthetic render/auxiliary injections are rejected; raw and validator hashes
   are recorded.
6. Focused audit/builtin/core/graph tests, format, warning-denied package Clippy, relevant realtime/
   graph policies, shell syntax/mutations and static no-artifact/no-workload checks pass on one
   clean candidate.

## Target matrix

One native Linux qualification host at exactly 48,000 Hz/q128. Cross-target and instruction
qualification belongs to the downstream issue.

## Required evidence

Candidate and `376774f` technical-input identities; default/feature surface proof; separate audit
fixture manifest and four payload hashes; unchanged three Issue-064 hashes; both exact million-call
records; state/report/meter/result rows; A/B/C and destruction-owner rows; nine counter/probe rows;
all-thread raw/validator hashes; commands/results; strict Terra/Sol verdicts;
`workload_invocations=0`; and `timed_benchmark_invocations=0`.

## Terra attempt 1, tranche 1 evidence — FAIL

Candidate technical input: `431cdc3`. The internal `#[cfg(test)]` builtin proof is implemented in
`miso-engine-builtins`: its private V1 record is exactly eight filter-state words, eight matrix
words, `remaining_updates`, and two lifetime recovery counters. Its six-call script passes atomic
nonfinite-target rejection, input sanitation, lane/filter-local injected recovery, and both reset
modes without adding a Cargo feature, exported item, or normal-build symbol.

The required graph half is structurally blocked by the current production preparation contract.
The attempted accepted-Issue-067 topology bound fourteen real requests (two capacity-one meter
sets at each of the seven taps), but `prepare_session_builtins` rejected every second request with
`builtin.meter.duplicate` at `$.meters[track_id=vocal,tap=<Tap>]`. The existing request identity
therefore permits only one meter consumer per `(track_id, tap)`, so it cannot prove the frozen
success/full two-set outcome without a separately scoped change to preparation semantics. The
unbinding graph-test scaffold was removed; no production duplicate-meter behavior was changed and
no substitute observer or fabricated tap proof was used.

Focused builtin command: `cargo test --locked -p miso-engine-builtins --lib
issue069_prepared_chain_snapshot_reset_and_recovery_script_is_exact` — PASS (1 passed, 0 failed).
Graph work stopped at the exact duplicate-meter diagnostic; no graph replacement test, audit,
fixture author, lifecycle/trace, workload, timing, or benchmark command ran.

`workload_invocations=0`; `timed_benchmark_invocations=0`; `benchmark_invocations=0`.
