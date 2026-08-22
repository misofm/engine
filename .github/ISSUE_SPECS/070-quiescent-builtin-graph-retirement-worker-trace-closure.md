# 070 Quiescent builtin graph retirement-worker trace closure

## Outcome

Close only the Issue-069 graph all-TID trace failure by making the audit-only retirement worker
ready and syscall-quiescent before the first render marker, while preserving every accepted
functional, fixture, lifecycle and direct-trace result from checkpoint `5ce93c0`.

## Context

**Builtin prepared-chain and graph realtime audit proof closure** stopped after its final Sol
attempt. Its direct one-million-call trace passed, its graph binary produced the exact accepted
one-million-render functional/lifecycle record, and its shared validator correctly rejected
retirement-worker startup and blocking `mpsc::recv` syscalls inside the first armed interval. This
stateless successor consumes `5ce93c0` only as technical input; Issue 069 has no overall PASS.

This issue permits one Terra implementation attempt and one bounded Sol correction/review. A
second failure stops. It authorizes no benchmark, timing, target, direct-audit or general workload
invocation.

## Scope

Change only the builtin graph audit harness and its directly owned focused/static trace proof.
Prestart the retirement worker and prove readiness before any marker. Replace blocking command
receive during armed intervals with an already accepted, preallocated, nonblocking move-SPSC plus
acquire/release atomic handoff. After all markers close, reclaim A on the retirement worker, stop
and join it, then destroy B and C on the control owner exactly as already accepted.

## Frozen harness contract

Create one capacity-one `bounded_spsc_move` before spawning the worker. Its sole value is a
`Reclaim` command; the producer remains on the control thread and the consumer and existing
`PlanRetirer` move to the worker. Stack-owned atomics have these single meanings:

- `ready: AtomicBool` becomes true with `Release` only after the worker has completed startup and
  its next operation is the nonblocking poll loop;
- `reclaimed_epoch_plus_one: AtomicU64` becomes `1` with `Release` only after epoch-zero plan A
  has been reclaimed and dropped on that worker; zero means incomplete; and
- `stop: AtomicBool` becomes true with `Release` only after reclamation is observed by the control
  thread.

The control thread waits with acquire loads before the first marker and after the last marker. The
worker loop uses only `Consumer::try_pop`, atomic loads/stores and `core::hint::spin_loop`; it must
not call `mpsc`, `recv`, `park`, `yield_now`, `sleep`, a mutex, allocation, formatting, logging or
I/O while markers may be armed. A full command queue is an assertion failure outside markers, not
a retry loop. Thread creation, startup, reclaim/drop, stop and join all occur outside armed
intervals. No production engine, DSP, graph, public API or fixture byte changes are permitted.

Preserve unchanged:

- the Issue-069 direct audit record SHA-256
  `3581ebf058151a0a0014ff08adcdd7fcd6fe6ad51a5baf41538272d4bba6ce8e`, raw trace-set hash
  `09f820aecf3490c3189595478d0a53deb14c8288e21de46a04bd0bda693a4c04` and validator-output hash
  `6fe4fa42f86b4a3e35c611acfc99b568001827bda23b202af01a931920f7e3de` without rerunning it;
- all five Issue-069 audit fixture payloads and manifest, the immutable Issue-064 corpus and all
  benchmark-input bytes;
- the graph canonical functional record SHA-256
  `54103c89b557a72da9c79cd00a636ea64933240a4dcb27c27647fb960b013db4`;
- A/B/C renders `1/999999/0`, one applied swap, 999,998 deferrals and prior-B renders, PDC 9,
  seven distinct real taps, stable storage, all nine detector counters zero, and destruction roles
  A=retirement owner, B/C=control owner, render owner=zero; and
- the shared all-TID validator, its clean/render-thread/auxiliary-thread mutation tests, four graph
  marker intervals, probes and deterministic JSON schema.

## Deliverables

Audit-only quiescent retirement-worker handoff; focused readiness/ownership/static proof; one
successful graph all-TID trace and strict evidence record.

## Explicit non-goals

Direct-audit rerun; fixture authoring or corpus changes; production engine/DSP/API changes;
changing graph counts, taps, PDC, detectors, markers or validator semantics; target/instruction
qualification; benchmark/preflight/workload/timing; optimization; listening; or V1 inspection.

## Dependencies by exact issue title

- Real-time memory, buffers, queues, and plan lifetime

Stopped Issue-069 checkpoint `5ce93c0` supplies accepted technical input, not a PASS dependency.

## Acceptance gates with objective measurements

1. Before any million-render command, focused tests prove readiness is published before markers,
   the capacity-one move-SPSC transfers exactly one reclaim command, epoch-zero reclamation is
   observed before stop, and A/B/C retain the accepted destruction owners. Static scans reject
   `mpsc`, blocking receive, park/yield/sleep and any direct-audit invocation from the changed
   graph path.
2. Before the one-shot trace, graph focused tests, all nine graph probes, the shared synthetic
   trace clean/render/auxiliary mutations, shell syntax, format, warning-denied audit-package
   all-target Clippy, applicable realtime/graph policies and diff/static no-artifact/no-workload
   checks pass. Candidate, fixture, direct evidence and graph functional-record identities are
   sealed.
3. On that unchanged clean candidate, run `scripts/trace-builtins-graph-audit.sh` exactly once.
   It is the sole authorized graph audit/strace/million-render invocation and may not be retried.
   The canonical graph record must retain SHA-256
   `54103c89b557a72da9c79cd00a636ea64933240a4dcb27c27647fb960b013db4`; the validator reports four
   paired intervals, at least two traced TIDs and zero violations, with no retirement-worker
   syscall timestamp inside any interval.
4. Record the clean candidate, exact command, graph record/raw trace/validator hashes, worker-TID
   presence, lifecycle roles and strict Terra/Sol verdict. Any failed precondition or one-shot trace
   is final FAIL/STOP; gates may not be weakened or rerun.

## Target matrix

One native Linux qualification host at exactly 48,000 Hz/q128. Issue 068 retains all
native/AArch64/Wasm target and instruction qualification.

## Required evidence

Checkpoint `5ce93c0`; clean candidate identity; unchanged direct/fixture/corpus/benchmark-input and
graph-record hashes; focused readiness/static outcomes; exact one-shot graph JSON; raw all-TID and
validator-output hashes; at least two trace TIDs; four intervals; zero violations; exact lifecycle
counts/roles; strict Terra/Sol verdicts; `direct_audit_invocations=0`;
`graph_trace_invocations=1`; `workload_invocations=0`; `timed_benchmark_invocations=0`; and
`benchmark_invocations=0`.
