# Realtime memory and plan lifetime

Issue 003 establishes ownership and bounded transport, not a graph, decoder, host callback, or DSP
architecture. All allocations and validation occur before a plan reaches the render owner.

## Prepared memory

`BufferArena` owns one planar `Box<[f32]>` plus fixed offset/shape tables. Construction checks every
channel/frame multiplication and accumulated offset. Borrowed `PlanarBufferRef` and
`PlanarBufferMut` contain slices and scalar shape metadata only. Issue 003 promises contiguous
planar `f32`; SIMD alignment and AoSoA layout belong to issue 008.

Live parameter delivery is owned by the per-effect `EffectControlLane` (#140), fed by the `EffectControlProducer`s prepared with the plan; the protocol crate's accepted-automation queue (#102) admits, retains and cancels sample-timed batches but has no render-side consumer yet (see `CONTROL_PROTOCOL_SEMANTICS.md`, "Delivery status"). The plan itself carries no parameter store. (#84 phase C deleted the unused issue-003 slot/event
store; `PlanEpoch` now lives with the plan exchange, whose publication epochs it names.)

`PreparedRenderPlan` privately separates immutable `PreparedProgram`/`RenderEnvelope` from mutable
arena and render-counter state. It is `Send`, deliberately not `Sync`, not
cloneable, and renders only through exclusive `&mut self`. The issue-003 reference renderer checks
the complete fixed I/O shape and writes silence; graph execution replaces that inner implementation
in issue 006 without changing the lifetime contract.

## SPSC cursor protocol

The native SPSC has one non-cloneable producer and consumer sharing a fixed `Arc<Ring<T>>`. The Arc
is cloned only at construction and released only when an endpoint is destroyed. Push/pop never
touch the reference count.

The ring allocates `capacity + 1` slots so equal cursors mean empty and advancing the producer onto
the consumer means full. Only the producer writes its local slot; only the consumer reads its local
slot.

1. Producer acquire-loads the consumer cursor before reusing a slot.
2. Producer initializes the slot and release-stores its advanced cursor.
3. Consumer acquire-loads the producer cursor before reading the slot.
4. Consumer moves the value and release-stores its advanced cursor.

Two refinements from #84 phase B (finding F6), neither of which changes an `Ordering`:

* Each cursor is `#[repr(align(64))]`-padded, after the read-mostly header (slot pointer, slot
  count, capacity, generation). A `Release` store by one endpoint therefore cannot invalidate the
  line the other endpoint reads on its fast path. The ring header is consequently `Arc`'s two
  counts plus three cache lines -- 256 bytes at align 64 on a 64-bit target -- which
  `ring_header_is_arc_counts_plus_three_cache_lines` pins from `core::alloc::Layout`.
* Each endpoint caches the peer cursor it last observed and reloads the shared line only when the
  queue *looks* full (producer) or empty (consumer). This is sound because a peer cursor only
  advances and never passes its partner: the true consumer cursor lies on the arc
  `[cached_consumer, local]`, and `next = local + 1` is on that arc only when
  `cached_consumer == next`, so "not full by the cached value" implies "not full by the true
  value". The consumer's case is the mirror image. Publication is unaffected -- every slot in
  `[local, cached_producer)` was released before the `Acquire` load that produced
  `cached_producer`.

Cursor wrap is a compare (`if next == slot_count { 0 }`), not `%`: `slot_count` is `capacity + 1`
and so never a power of two, which made the remainder an integer division on the hottest line of
the queue.

There is no CAS, spin, retry, lock, wait, or atomic counter. Owner-local successes/full/empty
counters use saturating `u64`. Queue generation is immutable; reset or seek creates a new lifetime
or adds a generation to the payload.

The only production unsafe code is slot initialization/read/drop inside `realtime/spsc.rs`. The
final Arc owner drops any still-initialized slots only after both endpoints have ceased access.
Endpoint drop order is safe and covered by a move-only drop test.

Browser launch uses `LocalRing`: the same bounded semantics with `MaybeUninit<T>` slots (for
`T: Copy`) and plain cursors. #84 phase B replaced the `Option<T>` slots: the discriminant
duplicated occupancy the head/tail cursors already carry, and `Option::take().expect(..)` put a
panic path inside a `REALTIME_POLICY` region, which `scripts/check-realtime-policy.sh` now
rejects. It is host-mediated on one render agent and does not claim SharedArrayBuffer or Wasm-thread
support.

## Plan publication

`plan_exchange` creates two bounded SPSC directions:

- control publisher to render owner: fully prepared plans with monotonically assigned epochs;
- render owner to control retirer: displaced plans awaiting destruction or reuse.

The initial plan fixes sample rate, quantum, and external channel envelope. Publication rejects and
returns an incompatible plan. A full queue also returns ownership and does not consume an epoch.

At each `RealtimePlanOwner::render` entry, the owner polls at most one candidate. It reserves a
retirement slot before moving the active owner. When retirement is full, the candidate stays in the
single pending slot and the unchanged active plan renders. When a reservation succeeds, the whole
candidate becomes active, the whole displaced plan is published to retirement, and only then does
the block render. Publication is never observed mid-block.

Actual reclamation is ownership transfer, not hazard-pointer or epoch garbage collection. Epochs
identify plan revisions. `PlanRetirer::try_reclaim` returns ownership to the
control/retirement caller; only that caller destroys or reuses a displaced plan. Engine teardown
must likewise move the render owner off the host callback before dropping it.

## Failure policy and evidence

Capacity/shape/time failures are typed and bounded. No queue silently loses an item. Full, empty,
event overflow, and deferred-swap counters saturate. Standard allocator OOM policy applies only to
off-render preparation.

The checked evidence consists of:

- source-policy and policy-mutation scripts;
- allocator/deallocator/lock/log/I/O/network/syscall mutation probes;
- one-million-item native FIFO stress plus Loom release/acquire state-space modeling;
- capacity-one/non-power-of-two wraparound and ownership-return tests;
- concurrent complete-plan publication with thread-ID destruction evidence;
- one-million-block allocation and syscall audit with accepted and deferred swaps;
- Wasm object inspection proving the browser-local fallback has no atomic opcode.

The two-round timing harness is descriptive only. It records the issue-001 environment metadata and
has no timing threshold, retry, or optimization loop.
