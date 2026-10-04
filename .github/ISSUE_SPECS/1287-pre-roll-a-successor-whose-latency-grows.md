# Pre-roll a successor whose latency grows

Slice 17 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

**Blocked on owner question Q2** of the umbrella (pre-roll, a latency reserve, or an accepted gap)
and on the numbers of *Record the swap block's cost on the 64-track console* (#1286). Do not start
it before the ruling. If the owner chooses the reserve or the gap, this issue is rewritten or closed
as not planned.

## Product outcome

Adding a latent effect (a limiter) that raises some node's latency while audio plays (the output's,
or a submix's) neither gaps nor jumps the rest of the mix. Every unchanged path continues bit for
bit; the mix only becomes later by whole quanta, which nobody hears as a discontinuity. The price is
one swap callback that renders `k + 1` blocks instead of one: `k = 4` for a true-peak limiter
(`Fs/100 + 6` samples) at 48 kHz and a 128-frame quantum.

## Context

- With the floors of *Keep every node's latency from dropping during playback* (#1285), no node's
  input timing drops. When a node's natural timing exceeds its floor by `Δ`, every unchanged path
  into it needs `Δ` more samples of history than its line holds, and those samples were played and
  discarded. Slice 15's gate 5 pins the resulting gap.
- Exact continuation needs the affected paths to process `Δ` more source samples than the host clock
  has reached, before the successor's first audible block. The rings normally hold more than that
  ahead (`default_source_ring_frames`, about 100 ms plus two quanta,
  `crates/host-core/src/prepare.rs:60-79`).
- `RealtimePlanOwner::enter_block` (`crates/engine/src/realtime/plan_exchange.rs:358-404`) runs the
  carry hook of *Hand the outgoing plan to its successor at the swap block* before the successor's
  first block. Lines carry head-aligned with a front pad `P` (*Carry compensation lines across a plan
  swap*, D3).
- Anchored seeks are on the host clock (*Hold an anchored source seek until its render sample*, D2).

## Open design, to settle after the ruling (planner's proposal and proof obligation)

- **Pre-roll.** After the carry, in the same `enter_block`, the owner renders `k` blocks of the
  successor into a scratch output the successor preallocates and discards them, then renders the
  host's block. The host clock does not move during the pre-roll.
- **Proof obligation.** Derive the floors and the per-line front pad `P` that make every unchanged
  edge bit-continuous when **any** surviving node's timing grows, whether the output's or a
  submix's. The proposal to test: `P = k × quantum` with `k = ceil(max growth / quantum)`, every
  surviving node floored at its predecessor value plus `P`, and every carried line padded by `P`.
  Show it on an output-growth case and a submix-internal growth case before implementing.
- **Input clock.** After a pre-roll the graph reads sources `P` samples ahead of the host clock. The
  plan keeps and reports that offset (C ABI: a reserved word of the plan resource report; browser:
  its status). An anchored seek stays on the host clock and the engine adds the offset when it
  compares; a host aligning a new stem adds the offset to the frame it computes.
- **Bound.** A successor whose `k` exceeds a fixed bound (proposed: 16) is not pre-rolled; it takes
  slice 15's transition, and a counter records it.

## Deliverables (after the ruling)

1. The design above, with its proof, in the PR description first.
2. Implementation in `crates/engine`, `crates/graph`, `crates/graph-compiler`, `crates/source`
   (anchor offset), `crates/host-core`, and the hosts' reports.
3. Documentation in the header, `docs/C_ABI_V1_QUALIFICATION.md` and the browser status docs.
4. Slice 15's gate-5 test replaced by gate 1 below.

## Authorized paths

- `crates/engine/src/realtime/plan.rs`, `plan_exchange.rs`
- `crates/graph/src/`, `crates/graph-compiler/src/`, `crates/source/src/lib.rs`
- `crates/host-core/src/`, `crates/host-core/tests/successor_swap.rs`
- `crates/capi/src/`, `crates/capi/include/miso_engine_v1.h`, `docs/C_ABI_V1_QUALIFICATION.md`
- `hosts/host-web/src/lib.rs` (status field only)

## Non-goals

- No spreading of the pre-roll over several callbacks (umbrella Deferred). No latency reserve.

## Objective gates (proposed)

1. **Gap-free acceptance, output growth.** B adds a muted track with a true-peak limiter insert and
   silent input, raising the output timing by `Δ`. With the swap after block 6, block `j` of the
   swapped run equals block `j + k` of a fresh B compiled with the derived floors and fed the same PCM
   from frame 0, for every `j`, at all four launch rates and both bank widths.
2. **Gap-free acceptance, submix growth.** The same with the limiter added inside a submix whose
   timing grows while the output's does not.
3. **Bound.** A swap with `k` above the bound takes slice 15's transition and increments the counter.
4. **Anchor.** Slice 5's gates pass after a pre-roll, with the offset applied.
5. **Realtime.** The swap callback, pre-roll included, makes zero allocations and frees.
6. Commands: the engine, graph, graph-compiler, host-core and capi test runs,
   `./target/release/audit capi`, and the umbrella's inherited gates.

## Test value

- Gate 1: a pre-roll that pads lines at the tail instead of the front, or advances the host clock,
  turns it red.
- Gate 2: a pre-roll sized only from the output's growth leaves the submix's paths gapped; it turns
  red.
- Gate 4: an anchored seek compared against the internal clock starts a stem `k` blocks early; it
  turns red.

## Dependencies

- *Keep every node's latency from dropping during playback* (#1285).
- *Record the swap block's cost on the 64-track console* (#1286).
- Owner question Q2 of *Swap a rebuilt plan without an audio gap* (#1269).
