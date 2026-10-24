# Count the frames a caller's own source removal discards

Successor of #1310 (batch 15a fold 3, root ruling of 2026-10-16), decision 15 D15-2 and decision 17
C1. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Problem

A source that the caller's own edit removes discards the audio still queued for it: a direct P0 -> B
removal, or a superseding B that removes a source A carried unchanged from P0. Root ruled that this
is not an unintended drop, because the caller asked for the removal, and the C header and
`docs/CONTROL_PROTOCOL_SEMANTICS.md` state the contract. But no counter reports how many accepted
frames were discarded, so a host cannot see it, and D15-2 wants every such loss counted.

## Decisions

- **D1. One core counter.** The core counts, per session, the accepted source frames that a removal
  discards (frames written to the removed source's ring and not yet read by render), at the swap
  that retires the ring, off the render thread. It is a monotone `u64`, never reset.
- **D2. Both hosts read it the same way.** It is a counter in the existing counter snapshot (a new
  counter ID from the free range, never a retired one), so the C ABI `COUNTERS_GET` and the browser
  read the same value through the same core function. No host-specific code.
- **D3. Render stays clean.** Counting happens where the retired ring is reclaimed (control side);
  render adds no work, no allocation, no lock.

## Gates

1. A session feeds N frames to source X, then an edit removes X before render reads them: the
   counter rises by exactly the unread frame count, on both hosts (parity row), at two rates.
2. A removal of a source with an empty ring adds 0.
3. A superseding B that removes a source A carried unchanged from P0 counts the same as the direct
   removal.
4. Render-thread allocations stay 0 across the removal swap.

## Authorized paths

`crates/host-core/src/source.rs`, `crates/control-plane/src/`, the counter vocabulary and its
mirrors, `crates/capi/include/miso_engine_v1.h`, `hosts/host-web/src/`, tests next to them,
`docs/CONTROL_PROTOCOL_SEMANTICS.md`, this spec.

## Dependencies

#1310 (on main with batch 15a).

## Root decisions (2026-10-24, from the scope note)

- **R1.** Counter `SOURCE_FRAMES_DISCARDED = 21`. It counts a source the caller's transaction
  removed **or redeclared**: both discard the old ring with its plan, by the same mechanism.
- **R2.** Stale-generation blocks still queued count (accepted, never read).
- **R3.** Not counted: session destroy, a withdrawn candidate's own rings (a fed fresh ring is
  refused, so 0 by construction), frames render played.
- **R4.** The unit is frames (`TransferBlock::frames`; a short end block counts its real frames).
- **Paths.** The scope note's list replaces this spec's. Gate 1's "two rates" is one rate: frames
  are rate-independent. Gate 4 has no new test: render gains no code.

## Attempt record

- Attempt 1. At a swap the successor moves every carried consumer out of the predecessor, so every
  occupied source entry a retired plan still holds is a removed or redeclared source.
  `SessionState::synchronize_plan_epochs` adds `retired_plan.unread_source_frames()` before the
  drop (control thread); `PcmSourceConsumer::unread_frames` sums the pending block and the queued
  blocks (`peek`, no pop). `refresh_provider_counters` copies the count; host-core serves 8
  counters. One path past the scope note: `crates/control-plane/src/compile.rs`, the
  `SessionState` constructor's new field (one line; the struct cannot compile without it).
- Render gains no call: the AudioWorklet gate passes with the pinned render indirect sites (8).
- Mutants (each red, then restored): pending block left out (unit test 6 != 10); every block
  counted as a whole quantum (12 != 10); the add in `synchronize_plan_epochs` removed (host-web
  0 != 256, C ABI 0 != 128).
