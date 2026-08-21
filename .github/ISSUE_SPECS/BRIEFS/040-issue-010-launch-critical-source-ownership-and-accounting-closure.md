# Sol implementation brief — issue 040 launch-critical source ownership and accounting closure

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1 only after the matching remote issue exists and matches the tracked
spec.** Start from Issue-010 checkpoint `5dbe1cb` or a descendant containing it. Issue 010 remains
FAIL; consume only its reviewed source/parser/ring/source-set implementation and named passing
evidence as technical input. This rescope permits one Terra implementation/review attempt and at
most one bounded Sol correction/review. A second failure stops.

Do not run or create a benchmark. `timed_benchmark_invocations=0` is invariant.

## Preserve the accepted technical slice

Keep the current RIFF/RF64 algorithms and conversion rules, move-owned SPSC transfer blocks,
positive-zero underrun/EOF semantics, generation-tagged block-boundary seek, host chunk contract,
one-pull source-set fan-out, graph topology/PDC/reduction order, target policy and Wasm local-ring
implementation. Do not redesign parsers, queues, graph execution or host adapters.

The correction boundary is production ownership, observable frozen metadata and exact retained
accounting. Qualification expansion belongs only to Issue 041.

## Frozen ownership shape

Split native control from retirement ownership. Controllers may submit bounded seek/wake commands
and read bounded non-render events/telemetry, but cannot own or detach the worker join handle. The
uncloneable retirement owner travels with its `PcmSourceConsumer` into the sealed graph source-set
driver. Successful sequential/native-fallback bind moves the driver into `PreparedRenderPlan`.
Rejected bind returns it with the graph and ordinary bindings unchanged.

Off-render plan reclamation sends one stop to each worker, joins it once, and then destroys reader,
decoder, staging, queues and source planes on the retirement owner. A full retirement queue defers
the plan swap. Rendering, source `begin_block`, auxiliary graph workers and controller drop never
stop, join, detach or free a worker. Add explicit lifecycle evidence; do not rely on detached
`JoinHandle` behavior.

Graph must not depend on the source crate. Retain the worker owner behind the existing source-owned
`GraphPreparedSourceSetDriver` object. Any single-source preparation API must preserve the same
uncloneable owner and provide a transactional move into that driver rather than exposing a raw
join handle beside the source set.

## Frozen shape and telemetry

Add one typed immutable shape shared by producer and consumer:

```text
PcmSourceShape {
    channel_count: u32,
    quantum_frames: QuantumFrames,
    frame_capacity: u64,
    transfer_block_count: u64,
}
```

Both endpoint accessors return equal values fixed at preparation. Do not derive frame capacity from
a lossy platform `usize` value.

Expose the decoder's cumulative saturating sanitation count through a bounded producer/controller
telemetry path. Count every native decoded sample replaced with positive zero, including decoded
work later invalidated by a seek; never count host-supplied already-decoded chunks. The mechanism
may use exact preallocated SPSC state or another proven fixed native primitive, but must not add
render synchronization or baseline Wasm atomics. Record exact F32 and F64 sanitation results.

## Frozen exact accounting boundary

Replace any retained queue whose allocation cannot be stated exactly, or account for its complete
engine-owned storage through an existing exact prepared primitive. The report must enumerate:

- data, recycle, seek, native command and event/telemetry queue headers and slots;
- every transfer-block header and PCM allocation;
- decoder read scratch and worker planar staging;
- worker/controller and retirement-owner arrays/records retained by the prepared product;
- source cache planes, source entries, mappings, graph claims, driver storage and owned stable-ID
  payloads; and
- PCM already charged by the session as a separate exactly equal subtotal.

For each category record count, bytes and largest request with checked `u64`/`usize` conversions.
Allocator headers, OS thread stacks, file page cache and RSS remain descriptive exclusions. The
combined session+source+graph total adds overhead once and rechecks compile caps,
`limits.memory_bytes`, item caps and largest allocation before ownership is consumed. Exact cap
accepts; one byte below rejects and returns all inputs. No term may use source duration.

## Representative product gates

1. Plan bind/replacement/retirement and transactional failure ownership, with exact stop/join/drop
   counts and threads.
2. Equal endpoint shapes and exact native F32/F64 versus host-zero sanitation telemetry.
3. Enumerated accounting, checked overflow, exact-cap acceptance, one-below rejection and no PCM
   double charge.
4. Matching preparation at all four launch rates and one extended-source mismatch.
5. One delayed-old-chunk seek boundary and existing one-ring/three-track repeated/crossed fan-out,
   plus missing/extra/duplicate/ordinary-overlap claim rejection with ownership return.
6. Existing render audit, focused/full quality and policy gates, Linux native tests, mobile/Wasm
   compile checks and Wasm no-worker/no-atomic inspection.
7. Explicit absence of Issue-040 timing tools/artifacts and invocation count zero.

## Stop conditions

FAIL for a detached worker, stop/join/drop in render, worker lifetime outside retired plan/source
set, shared ring consumption by graph workers, unreturned ownership, invented or omitted exact
bytes, double-charged PCM, duration-dependent storage, changed parser/DSP/seek semantics, implicit
SRC, new benchmark or a third attempt. Preserve evidence and rescope instead of expanding this
closure into Issue 041 qualification work.

## Delivery rule

A PASS unblocks the native PCM runner and mobile/browser host implementation. It does not satisfy
Issue 041 or final release qualification and makes no performance, device-runtime or exhaustive
format/race claim.
