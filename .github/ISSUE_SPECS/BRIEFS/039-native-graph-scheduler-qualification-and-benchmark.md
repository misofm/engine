# Sol implementation brief — issue 039 native graph scheduler qualification and benchmark

## Decision, input and attempt budget

**READY FOR TERRA ATTEMPT 1 only after the matching GitHub issue exists and matches the tracked
spec.** Start from upstream scheduler checkpoint `3236b9c` or a descendant containing it. Issue 009
stopped without overall PASS; consume its real parallel scheduler and recorded evidence as bounded
technical input only. This issue permits one Terra qualification/review attempt and at most one
bounded Sol correction/review. A second failure stops. Do not inspect V1/legacy.

Timed benchmark invocation count is **0**. Do not execute the scheduler benchmark binary or
`scripts/run-scheduler-benchmark.sh` until every ordered nonbenchmark gate passes on one clean
committed candidate and root Sol authorizes that exact command.

## Preserve the production checkpoint

Do not redesign `NativeSchedulerV1`, `PreparedNativeGraphPlanV1`, graph/PDC/reduction semantics,
retained-bank ownership, SPSC transport, sequential binding, target selection or retirement. Do
not introduce shared executor state, a mutex, unsafe alias, retry or a second job path. Test-only
injection may intercept startup/completion protocol events but must compile away from production
and must carry the same move-owned parcels through the real queues.

A bounded harness/fixture/validator defect may be corrected once. Any production-semantic defect,
lost/duplicated ownership, render-time forbidden operation or need for architecture redesign is a
FAIL and a new stateless correction issue, not scope for this qualification.

## Frozen representative fixture

Build one reusable fixture through current production compiler and sealed builtins APIs. At 48 kHz
and quantum 128 it contains 12 dual-mono tracks with asymmetric nonidentity L/R values, one real
retained builtin bank, compatible scalar tails, continuous state, an explicit sidechain, explicit
send into a submix with multiple stable contributions, nonzero exact integer PDC, final output and
at least two stable observers at different boundaries. Parameterize sample rate across exactly
44,100/48,000/88,200/96,000 and native render lanes across exactly 1/2/4; do not fork fixture logic.

For every rate render at least two consecutive blocks, then compare a declared continuation block
from equivalent fresh sequential/two/four-lane plans. Compare PCM by `to_bits`, exact PDC position,
qualification/error counters and complete observer `(sample_time,node_id,handle,boundary,value_bits)`
transcript. Freeze fixture/configuration/transcript hashes.

## Frozen deterministic and protocol matrices

- Run exactly 32 completion-acceptance perturbations from `0x000000000009d37a`. Record the generated
  order transcript/hash. Perturb only completion acceptance; stable partition recovery, reductions
  and observers remain canonical.
- Run exactly 100 fresh scheduler preparations and record one wave/unit/partition transcript hash.
  Generated track counts are exactly `1,3,4,5,12,17`; include retained-bank/no-padding, scalar-tail,
  narrow-wave, exact-byte/cap and overflow cases.
- Inject ready-handshake failure before publication and prove graph/bindings/config return
  transactionally. Inject command queue full, stale generation and duplicate completion through
  the real ownership protocol. Recover all issued parcels before returning, select worker errors in
  stable partition order and assert each token/job/state is executed/dropped exactly once. Do not
  model panic as supported recovery.

## Audit and syscall boundary

Reuse the production fixture and existing 10,000-callback 48-kHz/q128 four-lane audit. Retain one
block-boundary plan replacement, exact command/completion counts, fixed addresses, coordinator and
per-worker zero forbidden counters, and retirement-thread stop/join/destruction after disarm.

Add a reproducible Linux per-thread syscall trace. The audit emits unambiguous prepared/armed/
disarmed/retired phase markers outside the render scope. Trace the process with all threads
followed; attribute coordinator and worker TIDs and machine-validate that the armed interval has no
unexpected syscall for any participating render thread. Startup handshake, trace plumbing and
retirement syscalls are outside that interval and must not be misreported as render work. Preserve
raw trace and validator output hashes. Do not use logging or trace markers inside armed render.

## Repository and target proof

Run focused qualification tests first. Then run format, locked full workspace check/test,
warning-denied workspace Clippy and rustdoc, and workspace/realtime/graph/rack/builtin/scheduler
policies plus relevant mutation suites. Compile the graph/native scheduler path for the repository's
supported macOS target. If qualification-only changes do not touch production target reachability,
the recorded Issue-009 Android/iOS/Wasm object evidence may be cited with its candidate hash; if
they do, rerun the affected target/object gates. Never claim macOS or mobile runtime execution from
a compile result.

## Clean seal and exactly-once benchmark

Use the existing scheduler benchmark workload, validators and no-argument runner; do not add a
second timing framework. Preflight on one clean commit must launch zero audio workloads and prove:
real retained-bank graph/native-bind reachability; exact field and aggregate mutations; three modes
times rounds 1/2; stable output identity; output persistence; pipeline/shell failure propagation;
interruption disposition; overwrite and argument refusal; and no retry/resume loop. Seal the commit,
binary, source/fixture, runner and validator SHA-256 values.

After Sol verifies every nonbenchmark artifact, root alone may authorize exactly:

```sh
bash scripts/run-scheduler-benchmark.sh
```

The runner owns one untimed warmup and measured rounds 1 and 2, with sequential, two-lane and
four-lane records in each measured round. It emits exactly six accepted records. A workload or
runner failure consumes authorization: preserve raw/stderr/disposition bytes and stop. Never invoke
the binary directly, rerun, tune, add warmups/rounds or set a speedup threshold. Report rough ratios
as descriptive host-specific observations only.

## Ordered review gates

1. q128 all-rate 1/2/4-lane PCM/continuation/PDC/counter/observer differential.
2. Exactly 32 seeded completion perturbations.
3. Exactly 100 preparations and exact generated count/cap/resource matrix.
4. Handshake/queue/stale/duplicate/error/drop move-ownership matrix.
5. Existing 10,000-callback audit plus validated all-thread syscall trace.
6. Full warning-denied rustdoc/workspace/policy gates and macOS native compile.
7. Clean committed zero-launch preflight and sealed identities.
8. Sol authorization, then one runner invocation with one warmup/two rounds/six records.

Review immediately fails for a synthetic scheduler/graph fixture, incomplete exact counts, mutable
sharing, changed arithmetic/observer order, unreturned or twice-run parcel, forbidden render
operation, missing thread in the syscall trace, benchmark before authorization, retry/tuning or
work beyond the two-attempt budget. Do not weaken or rename a frozen gate to pass.

## Delivery and dependency rule

Issue 039 gates only consumers that require the qualified native parallel graph path and end-to-end
release qualification. It does not block sequential streaming, effects, browser/mobile adapters,
control-plane work or other features that do not claim native scheduler qualification. A PASS is
not a launch qualification or a performance-speedup claim.
