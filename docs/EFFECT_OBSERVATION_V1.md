# Effect observation V1

Issue #143. The general mechanism by which every aspect of an effect's state and impact becomes
observable: a **declared menu**, two **cost classes**, a **leased binding**, and a **conflating
transport**. Gain reduction is the first resident tap; it is not the design.

## The declared menu

An effect declares what may be observed, in the same descriptor that declares its parameters and
never anywhere else. `EffectDescriptor::observations` is a `&'static [ObservationDescriptor]`
and each entry states an effect-local nonzero ascending `id`, a display name and unit, a `kind`, a
transport `unit`, a `cost`, a `cadence`, a `fold`, a `channels` policy, and the declared bounds of
the value a consumer reads.

The effect is the only thing that knows which of its internal values exist, what they mean, and
whether reading one is a copy or a second computation. A host-side table would be a second source
of truth that goes stale the moment a kernel changes.

`validate_descriptor` enforces three rules beyond text and bounds:

* a `Computed` tap may not claim `PerBlock` cadence — that would put an analysis pass on the render
  thread, which is exactly what the cost split exists to prevent;
* a `PerLane` tap requires per-lane state to read (`maximum_state.left_bytes > 0`), because the
  observation is a read of kernel state and an effect with none cannot produce two lanes;
* every declared float is finite, ordered and free of `-0.0`, so an identity comparison never
  depends on a zero's sign.

Addressing mirrors `miso.command.v1` exactly: `(track_index, rack, effect_index, tap_id)`.

## The two cost classes

**`Resident`** means the value already exists in kernel state when the block ends. Publishing it is
`observe_resident(&self, tap_index, &mut out)` — a copy out of state that `process` wrote anyway.
No lane kernel is touched by observation, and the `&self` receiver is the enforcement rather than a
comment: an implementation physically cannot advance a smoother in the read, so "twice-called
returns identical bits" is a property of the signature.

A bank reads **every lane at once**: `observe_resident_bank(&self, tap_index, &mut [out])`. A
cohort of eight costs one vector extraction, not eight scalar reads.

**`Computed`** means an analysis pass that would not otherwise run. V1 declares the class,
validates it, and refuses to bind one: a subscription to a computed tap is answered
`UnsupportedKind`. A bound computed tap would be a lane that never publishes — a meter frozen at
zero with no way for the caller to learn why.

## The two-level zero

**Level 1 — structural.** A session whose console request names no observation capacity
(`observation_taps == 0`) has no observation state in the compiled plan *at all*: no lane, no
accumulator, no conflating cell. Not a disabled one — none. `attach_effect_observation` is the
only thing that creates one and it is never called. `observation_retained_bytes` is `0`, and that
zero is **walked over the built runtime** rather than computed from the request.

**Level 2 — arm/disarm.** Inside a capable plan, subscribe and unsubscribe ride the effect's
existing bounded control channel as `EffectControlRecord::Observe`, applied at the block
boundary and acknowledged with the exact `applied_at_sample`. Slots are preallocated from the
declared menu, so subscribe allocates nothing and disarm frees nothing: `retained_bytes` is
unchanged across both. An unarmed tap's state is never read, transformed or stored; the honest cost
is one predicted branch per driven effect per block.

Because a subscription and a parameter command share one queue and one drain, STATE and IMPACT
correlate on one sample timeline **by construction** rather than by two clocks agreeing.

## The transport

`miso_engine_core::realtime::observe` is a conflating single-writer cell, not a queue. An
observation is a *level*, not an *event*: a meter that missed three windows wants the newest one,
not a replay. A queue would make the render thread's cost depend on whether a control thread
happened to drain it.

The writer is wait-free — nine stores, no full, no backpressure, no return value, because there is
no outcome a render thread could act on. Latest wins, always. Each published field is its own
atomic word behind an odd/even sequence counter; a reader that sees an odd counter, or two
different counters, retries. There is no `UnsafeCell` and no `unsafe` block, which is what lets it
live inside the realtime root whose approved-unsafe list is exactly two files. Under `wasm32`
without the `atomics` feature every operation lowers to a plain load or store.

**Read-reset without a second writer.** The reader owns `consumed_sequence` and is its only writer.
It does not gate publication; it makes what was overwritten *countable*
(`ObservationReaderV1::missed_windows`), which turns "the meter froze" from an invisible failure
into a counted one.

**Windows tile.** A window closes at exactly `window_blocks` blocks and the next opens where it
closed — `[first_sample, end_sample)`, half-open, no gap and no overlap. That is a property of the
lane, not of whatever the caller passes, and it is what lets a consumer correlate a window against
a command's `applied_at_sample` rather than against a wall clock.

> **Deviation from the original design, argued.** D4 proposed opening a fresh window "at
> `window_blocks` *or* on observing a newer `consumed_sequence`". That would make window length
> depend on control-plane timing: in the browser, where the reader runs on the render thread between
> blocks, it collapses every window to one block and defeats `console_meter_blocks`; and it
> decouples the gain-reduction window from the peak-meter window that shares the same
> `miso.meter.v1` frame. Fixed-length tiling is the stronger property. `consumed_sequence` is kept
> and is what makes the gap observable.

## Units, folds, and where a logarithm is allowed

The tap declares what crosses the transport (`unit`) and what a consumer reads (`display_unit`,
`minimum`, `maximum`). They differ in exactly one place, and the difference is the whole point of
declaring the transport unit separately.

* A compressor, gate/expander and multiband compressor hold gain reduction as **negative decibels**
  and publish exactly that.
* A true-peak limiter holds it as the **linear** recursive word `d`, where `gain = 1 - d`. It
  declares `unit: Linear` and publishes `d`. Converting it to decibels needs a logarithm, which a
  render thread may not take, so the host converts once per **closed window** on the control plane:
  `-20 log10(1 - d)`, clamped into the tap's declared range.

`ObservationFoldV1::PeakMagnitude` is `max(|x|)` over the window. That is what turns an effect's
own negative-for-reduction convention into the non-negative magnitude a meter reads, and it is why
an app's `Math.max(0, x ?? 0)` is a **no-op** rather than a silent zeroing.

## Absent-effect semantics

`MisoMeterFrameV1.trackGrDb` is positional and `trackCount` long, and every entry is finite.
`0` deliberately conflates "not reducing" with "no observed effect on this track", because the
array is read without null checks. The distinction lives in the `miso.observe.v1` acknowledgement's
subscription map — `{ trackIndex, rack, effectIndex, tapId, frameSlot, windowBlocks }` — which is
the only place that can express it.

Several armed taps on one track fold max-magnitude into the one slot, on the control plane. Each
tap keeps its own cell.

`masterGrDb` is `null`, never `0`, when no track was designated or the designated track published
no window: `0` would be indistinguishable from "the master is not reducing".

V1 has no structural master bus — submixes and outputs carry no effect racks — so the master
reading is a **designation** (`console_master_track_plus_one`), not a discovery. The successor is
effect racks on submixes.

## Plan replacement drops subscriptions

Subscriptions are **per plan**. A structural session edit produces a replacement plan whose lanes
exist and are unarmed; the readers the old plan handed out simply stop advancing. An app
re-subscribes against the new plan and receives a fresh map whose sequences restart at `1`. Nothing
carries over, and nothing has to be torn down.

## Lease and arming are different switches

**Arming is the switch that stops the work. The lease is the switch that stops the traffic.**
Releasing the meter lease stops the frames; it does not unsubscribe anything, and subscriptions
survive it. Retaking the lease restarts the frame sequence and the reported window, so a consumer
never folds a window from before the release into one after it.

## What ships in V1

`miso.compressor`, `miso.gate-expander`, `miso.multiband-compressor` and `miso.true-peak-limiter`
each declare exactly one tap: id `1`, "Gain Reduction", `Resident`, `PerBlock`, `PeakMagnitude`,
`PerLane`, `0 .. 100`. The multiband's is a single aggregate — the **deeper** of its two bands,
because the bands are applied to disjoint parts of the spectrum and adding them would report a
reduction the signal never received. Per-band taps are an additive follow-up that needs no wire,
contract or transport change to arrive.

No computed tap ships. No other effect declares a tap, and every one of their descriptors is
byte-for-byte unmoved.
