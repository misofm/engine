# 043 Exact lock-free native source sanitation telemetry handoff

## Outcome

Close the one remaining launch source-product defect by replacing opaque shared sanitation/stop
ownership with exactly accounted safe move/SPSC handoff, without changing the accepted source
parser, ring, graph fan-out or plan-retired worker-lifetime behavior.

## Context

Issue **Issue-010 launch-critical source ownership and accounting closure** stopped after two total
attempts. Its green ownership, shape, representative correctness and non-`Arc` accounting checkpoint
is accepted only as technical input; it has no overall PASS. Its native sanitation/stop state is
held in `std::sync::Arc`, whose opaque control allocation cannot be included in an exact retained
layout. A proposed intrusive replacement was rejected before compilation because it required new
custom unsafe shared ownership.

This issue has exactly **two total attempts**: one Terra implementation/review attempt and, if
needed, one bounded Sol correction/review. A second failure stops. Benchmark and timing commands are
forbidden; `timed_benchmark_invocations=0` is invariant.

## Scope

- Remove the native sanitation/stop `Arc` and its reported pseudo-exact payload category.
- Give the source-set retirement token the producer of one dedicated, exactly accounted stop SPSC;
  the worker owns its consumer, polls it outside render, and is still joined exactly once off render.
- Keep the cumulative saturating sanitation watermark worker-local and stamp it on every moved
  transfer block, including blocks later discarded as stale. Carry an unsubmitted/stale watermark
  forward to the next submitted block.
- Make controller-only snapshots explicit bounded request/event exchanges through an exactly
  two-item event queue (ready plus one snapshot-or-terminal slot). Producer, controller,
  consumer and after-disarm source-set reports are monotonic; equality is required only at frozen
  synchronization points (ready/snapshot, observed block, and terminal event), not during races.
- Recompute exact retained categories, totals, largest allocation and exact/one-below caps for the
  stop queue, worker command/event queues, changed transfer metadata and inline controller state.

## Required contracts

Use only the existing safe move-owned bounded SPSC primitive and ordinary owned fields. Add no
`Arc`, `Rc`, custom refcount, raw pointer, new `unsafe`, lock, render allocation or baseline Wasm
atomic. The stop SPSC has one producer (retirement owner), one consumer (worker) and one item; the
controller command producer cannot stop or detach the worker. Stop-send and join remain off render.

Each native transfer block carries the cumulative number of decoded non-finite, subnormal or
non-representable samples replaced by positive zero through that block. Consumer/source-set state
takes the saturating maximum while consuming or discarding blocks. An explicit controller snapshot
request yields the worker-local value through the bounded event queue; worker termination publishes
one final value. Host-decoded chunks always report zero native sanitation.

## Deliverables

- safe SPSC stop ownership and block-stamped/controller-event sanitation handoff;
- exact updated source/session/graph retained accounting and cap checks;
- adversarial controller-first, source-set-first, stale-block, terminal and exact-cap tests; and
- focused/full policy and target evidence, subject to independently recorded external blockers.

## Explicit non-goals

Parser/corpus expansion, randomized races, real-worker 100,000-render audit, allocation-layout/RSS
qualification, new graph/host APIs, SRC, benchmarks, performance tuning, `Arc` layout assumptions,
or custom unsafe/shared-ownership machinery. Issue 041 retains qualification-only work.

## Dependencies by exact issue title

- Issue-010 launch-critical source ownership and accounting closure
- Real-time memory, buffers, queues, and plan lifetime
- JIT PCM streaming and host-supplied source rings

The stopped Issue-040 dependency means only its explicitly accepted green checkpoint, not PASS.

## Sol implementation brief (2026-08-21)

**READY FOR TERRA ATTEMPT 1 after the matching remote issue exists and matches this body.** The
tracked brief is
`.github/ISSUE_SPECS/BRIEFS/043-exact-lock-free-native-source-sanitation-telemetry-handoff.md`.

## Acceptance gates

1. No `Arc`/custom unsafe/refcount remains on the native telemetry/stop path; render performs no
   telemetry atomic, stop send, allocation, lock or join, and Wasm gains no atomic requirement.
2. Controller-first and source-set-first destruction retain exact one-stop/one-join behavior;
   preparation/bind/cap rejection returns every owner without a detached worker.
3. F32/F64 sanitation, stale/discarded work, explicit controller snapshot, after-disarm source-set
   telemetry, terminal telemetry and host-zero cases match exact frozen counts.
4. Every changed retained category has count/bytes/largest/alignment where applicable; exact total
   cap accepts, one byte below rejects, PCM is not double charged and duration is absent.
5. Focused locked source tests, format and warning-denied source Clippy pass before the existing
   workspace/policy/target gates. An unrelated failing workspace gate is reported, never weakened.
6. `timed_benchmark_invocations=0` and no benchmark/timing artifact exists.

## Target matrix

Linux/cloud exercises the native worker. Android/iOS compile the host/source ownership surface.
Browser Wasm retains only the host ring and no native worker or baseline atomic.

## Required evidence

Candidate hash; public API diff; ownership/event transcript; exact sanitation counts; retained
allocation table and exact/one-below results; focused/full/policy/target results; explicit external
blockers; `timed_benchmark_invocations=0`; and Terra plus final Sol PASS/FAIL verdicts.

## Terra attempt 1 focused checkpoint (2026-08-21)

**Focused source gates: PASS; awaiting Sol review/full evidence.** Native telemetry no longer uses
`Arc`, `Rc`, custom shared ownership, raw pointers, new `unsafe`, or a render-side atomic. The
source-set retirement token owns a capacity-one stop SPSC producer; the worker owns its consumer
and emits a terminal watermark before the off-render join returns. Native transfer blocks now carry
the worker-local cumulative sanitation watermark, and consumer/source-set telemetry takes the
saturating maximum even while discarding stale blocks. Controller ready/snapshot/terminal events
are an exactly two-item SPSC exchange, and the public worker-token compile-fail proof remains.

- Updated accounting removes the pseudo-exact shared-telemetry category and adds exact stop-queue
  bytes/items; the worker event queue is exactly two items. Transfer-block metadata is included by
  the existing ring metadata layout. Existing combined exact/one-byte-short transaction test
  remains green.
- Focused tests cover controller-first and source-set-first retirement, ready/snapshot/final
  watermarks, stale stamped block discard, saturation, deterministic seek, host-zero telemetry,
  wrapped host submission, and exact-cap rejection.
- PASS: `cargo fmt --check`; `cargo test -p miso-engine-source --locked` (27 unit tests plus one
  compile-fail doctest); `cargo clippy -p miso-engine-source --all-targets --locked -- -D warnings`;
  static source-path absence check for `Arc`, `Rc`, `unsafe`, and prior shared telemetry types.
- Not run at this focused checkpoint: workspace/policy/target gates. No benchmark or timing
  command was run.

`timed_benchmark_invocations=0`.

## Sol correction attempt 2 final verdict (2026-08-21)

**Status: PASS for the Issue 043 product boundary; one unrelated full-workspace test failure is
preserved below.** Sol reviewed candidate `e265ffb` and applied one bounded source-only correction.
No `Arc`, `Rc`, raw pointer, custom refcount, new `unsafe`, lock, render atomic, allocation, stop
send or join was introduced. The capacity-one stop SPSC remains owned by the source-set retirement
token and consumed by the worker; the exactly two-item event SPSC still holds ready plus one
snapshot-or-terminal event. Graph source-entry drop takes and drops the retirement worker before
its consumer/planes, so stop, terminal publication and join complete off render before retained
source storage is destroyed.

The correction fixes two acceptance defects found adversarially:

- `NativeWaveDecoder::decode_quantum_into_planar` reports a cumulative saturating sanitation
  count. The worker had added consecutive cumulative reports and therefore overcounted a two-block
  sequence (`2`, then `3`) as `5`. It now takes the monotonic maximum, and a two-quantum F32 test
  proves consumer, controller snapshot and terminal values are exactly `3`. Existing F64,
  stale-generation discard, saturation, seek, disarm and host-zero tests remain green.
- The source report had treated each queue's header-plus-slot total as one allocation when
  computing the largest request. Command, event and stop queues now report their exact total,
  actual largest allocation and maximum alignment separately from the audited SPSC retained
  payload. The combined largest-request cap uses the actual maximum. The fixed capacities remain
  command=`caps.control_queue_items`, event=`2`, stop=`1`; exact total and exact largest caps accept,
  while either cap one byte below rejects before publication.

Focused and changed-boundary evidence is PASS: `cargo test -p miso-engine-source --locked` (29
unit tests plus one compile-fail doctest), source all-target warning-denied Clippy, workspace check,
format, warning-denied Clippy and rustdoc, workspace/realtime policies and their mutation tests,
graph policy, Android ARM64 and iOS ARM64 source+graph checks, Wasm scalar and `simd128`
source+graph checks, and Wasm object inspection with no atomic opcode (5 objects). Static review
found none of `Arc`, `Rc`, `unsafe`, atomics or locks on the changed native source path.

The single requested post-`b68abf5` `cargo test --workspace --locked` run passed Issue 042 and all
Issue 043 source tests reached in focused evidence, but failed an unchanged scheduler-fixture test:
`q128_preparation_matrix_is_exact_for_100_runs_and_generated_track_counts` reported transcript
`3615656561314613090` instead of frozen `14752737557138656094` at
`tools/miso-engine-scheduler-fixture/src/lib.rs:858`. That crate and test are outside Issue 043 and
were not modified or retried. This independently recorded external failure is not converted into
an Issue 043 failure and is not weakened. No Issue 041 qualification work was run.

No benchmark or timing command was run; `timed_benchmark_invocations=0`.
