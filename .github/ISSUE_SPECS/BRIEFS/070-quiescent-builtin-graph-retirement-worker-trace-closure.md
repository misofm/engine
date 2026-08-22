# Sol implementation brief — issue 070 quiescent builtin graph retirement-worker trace closure

## Decision and attempt budget

**READY.** Consume stopped Issue-069 checkpoint `5ce93c0` only as technical input. Preserve its
accepted direct trace, fixture root, graph functional million record, lifecycle counts, probes and
shared validator/tests byte-for-byte unless a purely mechanical source-path adjustment is required
by the bounded harness correction. Permit one Terra attempt and one bounded Sol correction; a
second failure stops. Direct, target, workload, benchmark and timing invocations remain zero.

## Exact correction

In the audit graph binary only, replace `std::sync::mpsc` retirement commands with one
capacity-one `miso_engine_core::realtime::bounded_spsc_move` created before worker spawn. Move its
consumer and the existing `PlanRetirer` to the retirement worker. Keep the producer on control.
Use stack-owned `AtomicBool ready`, `AtomicU64 reclaimed_epoch_plus_one` and `AtomicBool stop`
under a scoped worker lifetime so no new shared-ownership mechanism or production surface is
introduced.

The worker publishes `ready.store(true, Release)` only after closure startup, immediately before
entering a loop whose only operations are `try_pop`, acquire atomic loads and
`core::hint::spin_loop`. Control observes readiness with `Acquire` before audit warm/reset and
before the first marker. Do not use `recv`, `park`, `yield_now`, `sleep`, condvars, mutexes,
allocation, formatting, logging or I/O in the armed lifetime.

After the fourth end marker, control pushes exactly one `Reclaim` value. The worker pops it,
requires retired epoch zero, drops A, then stores `reclaimed_epoch_plus_one=1` with `Release`.
Control observes `1` with `Acquire`, sets `stop=true` with `Release`, joins the worker and verifies
its thread identity owns A's sole destruction. Only then drop `RealtimePlanOwner` on control and
verify B and never-applied C are each destroyed there. The render owner destroys nothing. Queue
full, missing retirement, any other epoch or any role/count drift fails immediately outside
markers. The qualification-only busy poll is non-timed and makes no performance claim.

Do not alter the four marker pairs, render schedule, plan topology, meter requests, fixture hashes,
canonical JSON or detector scope. In particular preserve exact graph-record SHA-256
`54103c89b557a72da9c79cd00a636ea64933240a4dcb27c27647fb960b013db4` and A/B/C
`1/999999/0`, `swaps_applied=1`, `swaps_deferred=999998`,
`prior_plan_renders_on_deferred=999998`, PDC 9, seven distinct taps and all nine zero counters.

## Mandatory preflight before the one-shot trace

Run only non-million gates first:

1. focused graph-bin tests proving ready-before-marker ordering, one-command move ownership,
   epoch-zero completion and exact destruction roles;
2. all nine graph probe mutations and the existing shared clean/render/auxiliary trace-validator
   mutations;
3. shell syntax, format, audit-package all-target tests and warning-denied all-target Clippy;
4. applicable realtime and graph policy checks/mutations plus diff/static scans; and
5. hashes sealing the candidate, Cargo lockfile, five Issue-069 fixture files, immutable Issue-064
   inputs, benchmark-input tree, accepted direct evidence and exact graph record.

Static source checks must prove the changed graph audit contains no `mpsc`, `.recv(`, `park`,
`yield_now`, `sleep` or direct-audit launch and that the retirement loop can reach only SPSC,
atomic and spin-loop operations until disarm. The trace wrapper must still take no arguments,
clear only its owned ignored trace root, use `strace -ff -qq -ttt`, validate exactly four marker
intervals, require at least two trace TIDs and require the exact graph JSON schema and hash.

Any red preflight stops before the million-render command. Repair only a bounded harness defect
inside the attempt; do not consume the sole trace to diagnose an unsealed candidate.

## Sole authorized execution

After all preflight gates pass on one clean committed candidate, run exactly:

```sh
scripts/trace-builtins-graph-audit.sh
```

This single command is simultaneously the sole graph audit, strace and one-million-render
invocation. Do not run the graph binary separately and do not retry the command. PASS requires the
unchanged canonical graph record hash above, four paired intervals, at least the marker/render TID
and retirement-worker TID, zero validator violations, and no syscall on any TID strictly inside an
armed interval. Hash the sorted raw trace-file set and canonical validator output.

## Stop conditions and verdict

STOP on a dirty candidate, changed direct/fixture/corpus/benchmark input, graph JSON drift,
missing auxiliary TID, worker syscall overlap, nonzero detector, wrong render/swap/deferral/PDC/tap
or destruction value, a failed one-shot command, or a second implementation failure. Do not repair
production, change validator semantics, rerun direct, trace again, qualify targets or invoke a
benchmark/timed workload.

Record the exact candidate, unchanged input hashes, preflight commands, one graph record/raw
trace/validator hash set, TID and interval counts, lifecycle row, Terra/Sol verdict,
`direct_audit_invocations=0`, `graph_trace_invocations=1`, `workload_invocations=0`,
`timed_benchmark_invocations=0` and `benchmark_invocations=0`. PASS alone unblocks **Builtin
native, AArch64, and Wasm runtime-selection and instruction qualification**.
