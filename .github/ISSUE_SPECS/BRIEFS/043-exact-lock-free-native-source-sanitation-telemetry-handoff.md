# Sol implementation brief — issue 043 exact lock-free native source sanitation telemetry handoff

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1 after remote issue synchronization.** Consume Issue-040's accepted
plan-owned worker lifetime, transactional return, shapes, representative product tests and all
non-`Arc` accounting as technical input only. This issue permits one Terra attempt and at most one
bounded Sol correction. A second failure stops. No benchmark or timing command is authorized;
`timed_benchmark_invocations=0`.

## Frozen implementation boundary

Delete the shared sanitation/stop `Arc`; do not replace it with `Arc`, `Rc`, a custom refcount, raw
pointer, leaked allocation, new unsafe or lock. Reuse only the accepted safe bounded move-SPSC:

- a dedicated capacity-one stop queue moves from the source-set retirement owner to the worker;
- the worker-local cumulative sanitation watermark is copied into each moved transfer-block header;
- stale/discarded blocks update the consumer watermark before recycling, and an unsubmitted pending
  watermark carries into the next submitted block;
- controller snapshots are synchronous explicit command/event exchanges through an exact two-item
  event queue (ready plus snapshot-or-terminal), with one terminal event carrying the final
  watermark; and
- host providers have an inline zero native-sanitation value.

Controller, producer, consumer and source-set values are monotonic. Require equality only after an
explicit snapshot response, after the stamped block is observed/discarded, and after the terminal
event. Do not promise an instantaneous cross-thread getter.

## Ownership and realtime rules

The retirement token alone sends stop once and joins once. Dropping a controller neither stops nor
detaches the worker. Source-set/retired-plan destruction remains off render and orders stop/join
before ring/decoder/staging destruction. Rendering only moves existing ring blocks and updates
owner-local integers; it never touches a new atomic, stop queue producer, event wait or allocation.

## Exact accounting

Remove the pseudo-exact `Arc` category. Enumerate the stop queue, command/event queues, changed
transfer-block metadata and inline controller/retirement records with checked count, bytes,
alignment and largest request. Recompute per-source and combined totals before starting/publishing;
exact accepts and one byte below rejects. Preserve the separate equal PCM charge and exclude
allocator headers/OS state explicitly. No formula uses source duration.

## Ordered gates

1. Controller-first, source-set-first, rejected preparation/bind/cap and retired-plan replacement
   prove returned ownership and exactly one off-render stop/join.
2. Exact F32/F64, stale pending/block, snapshot, after-disarm, terminal and host-zero sanitation.
3. Allocation table plus exact/one-below/overflow/no-double-charge tests.
4. Focused locked source tests, format and warning-denied source Clippy, then existing workspace,
   policy and changed target checks; preserve unrelated failures exactly.
5. Static absence of `Arc`/new unsafe/refcount on this path, render synchronization, Wasm native
   worker/atomic reachability, and benchmark/timing artifacts.

## Stop conditions

FAIL for opaque/invented bytes, custom shared ownership, controller-owned worker lifetime, lost
terminal watermark, stale work omitted from the counter, render atomic/wait/send/free, duration
storage, weakened cap, product expansion into Issue 041, any timing call or a third attempt.

A PASS replaces Issue 040 as the launch source-product dependency for Issues 022–024 and permits
Issue 041 qualification. It does not itself satisfy Issue 041 or release qualification.
