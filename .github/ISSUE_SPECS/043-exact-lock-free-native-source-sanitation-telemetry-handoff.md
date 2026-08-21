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
