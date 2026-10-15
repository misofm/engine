# Carry an insert lane that moves between a bank and a per-node instance

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8).
Slice 11b of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

An insert whose lane changes representation at a swap keeps its exact state. Adding the eighth
compressor insert makes a group of seven per-node instances bank, and removing one makes a full bank
fall apart. Both directions continue bit for bit, live state included. The carry moves the state at
the swap block, and the predecessor never renders again.

## Note (root, 2026-10-12, stream B batch-3 verdict)

After #1345 a bank lane's observation queue consumer lives in
`LiveControlEffectBankStage::observe_queues` (`crates/rack/src/lib.rs` ~1162), taken out of its
`EffectControlLane` at bind. A per-node `LiveControlEffect` keeps its consumer in its
`EffectControlLane` (`crates/graph/src/runtime.rs` ~1230, drained by `drain_observations`). A
lane's move between a bank and a per-node instance therefore moves the consumer between those two
homes as well as the cells; #1280's D3 states the carry rule.

## Context

- An insert group banks only when it is full. The planner pads only console groups (`pads`,
  `crates/graph-compiler/src/banks.rs:76`). So at 8 lanes, seven tracks with the same insert render
  per node and eight bank; at 4 lanes, three and four. #888 and #889 (open) would bank partial
  groups. Whichever lands later keeps this slice's gates.
- A per-node insert is `NodeKind::Effect(GraphPreparedEffect)` or `NodeKind::LiveControlEffect`
  (`crates/graph/src/runtime.rs:1136`, `:1143`; struct `LiveControlEffect` `:1184`). Both hold a
  `GraphPreparedEffect` (`crates/graph/src/lib.rs:881`) with `processor: Box<dyn
  PreparedNativeEffect>` (`:884`). A bank lane is a lane of an `EffectBankStage` or a
  `LiveControlEffectBankStage` (`crates/rack/src/lib.rs:750`, `:937`).
- Per-node and bank payloads share one layout: `snapshot_state_payload` and `restore_state_payload`
  (`crates/effect-contract/src/lib.rs:1930`, `:1975`) and the bank pair (`:2070`, `:2079`). Both
  are render-safe and exact mid-ramp (#1278, on `main`). Both snapshots take `&self`.
- #1069 (*Multiband compressor: a ramp's cut moves a lane's bits in a bank*, open): a multiband lane
  with a ramp in flight is not bit-identical between a bank and a per-node instance until #1069
  lands. The gates use the compressor and the limiter.
- The effect rule, the retarget, the carried cells and the shunt copy come from *Carry console
  effect lanes across a plan swap* (#1279, D1) and *Carry live-controlled effect lanes across a
  plan swap* (#1280, D2-D5).

## Decisions frozen for this slice

- **D1. Rule.** As #1279 D1, and #1280 D1 for live-controlled owners. Live differences retarget as
  #1280 D4. Prepared differences restart the owner and add its strip to the restart set (#1277 D6).
- **D2. Move mode.** The state goes through the payload scratch in one of two directions:
  - per-node `snapshot_state_payload` → bank `restore_track_state_payload`;
  - bank `snapshot_track_state_payload` → per-node `restore_state_payload`.

  The control state follows #1280 D2's `carry_from`. The shunt words move between the per-node
  `BypassShunt` and the lane's words of the bank line, relative to the bank's shared cursor.
- **D3. A collapsed bank is desymmetrized first.** Before a bank snapshot, a collapsed predecessor
  bank is desymmetrized, once per bank (`desymmetrize_channels`,
  `crates/effect-contract/src/lib.rs:2173-2182`). This is bit-neutral: it writes into the right
  channel the state a dual run would hold, and collapse never changes bits. So the bank snapshot is
  exactly the payload a dual run would write, and the per-node restore continues it bit for bit
  (#1279 D4 states the same rule for bank-to-bank lanes).

## Deliverables

1. D2-D3 in `crates/graph` and `crates/rack`, and the join in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/rack/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`

## Non-goals

- No per-node to per-node move (#1282).
- No multiband gate before #1069.

## Objective gates

1. **A group becomes a bank, both widths.** At `Backend::Simd8`, seven tracks carry a compressor
   insert (per node), and B adds an eighth, muted, so the group banks. Every block is bit-identical
   to the reference. At `Backend::Simd4`, the same with three and four tracks. Repeat with the
   limiter insert.
2. **A bank falls apart.** The reverse edit (remove one track from a full group) is bit-identical.
3. **Live state.** Gate 1 with live controls and a compressor value unread in A's cell at the swap
   is bit-identical to the reference fed the same value. A second case also changes the threshold of
   a lane that changes representation (a live value): it is bit-identical to "live edit, then
   structural edit" (#1280 gate 5's shape).
4. **Realtime.** Each swap block makes zero allocations and frees.
5. Commands:
   - `cargo test --locked -p rack -p graph -p host-core --features rack/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `cargo run --locked --release -q -p realtime-policy`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: a carry that handles only bank-to-bank leaves the new bank's lanes at rest. It turns red.
- Gate 2: the reverse direction is a separate code path (a bank snapshot into a per-node restore).
  A carry that handles only gate 1's direction turns it red.
- Gate 3: control state lost across a representation change, or a retarget routed to the old
  representation's producer, turns it red.

## Dependencies

- *Carry live-controlled effect lanes across a plan swap* (#1280).

## Amendment (controls coordinator, batch 10, 2026-10-14)

Checked against `12d4a4f9d`.

- **Anchors moved.** `NodeKind::LiveControlEffect` is `crates/graph/src/runtime.rs:1161`, struct
  `LiveControlEffect` `:1228`; `GraphPreparedEffect` is `crates/graph/src/lib.rs:909`; the per-node
  payload pair is `crates/effect-contract/src/lib.rs:2325` and `:2370`, the bank pair `:2468` and
  `:2483`; `desymmetrize_channels` is the trait method at `:2588`; `pads` is still
  `crates/graph-compiler/src/banks.rs:76`. `LiveControlEffectBankStage` is
  `crates/rack/src/lib.rs:1154`. #888, #889 and #1069 are still open, so the Context and gate
  choices stand.
- **Every production lane is live-controlled** (see #1280's batch 10 amendment): a bank lane is a
  `LiveControlEffectBankStage` lane and a per-node insert is a `LiveControlEffect`. The non-live
  forms are reached only by test entries. The gates use the live entries.
- **D3.** Use `BankChain::disengage_for_carry`, once per predecessor chain, as
  `Runtime::carry_effect_lanes` does (`runtime.rs:4726`), not a direct `desymmetrize_channels`
  call: the chain owns the collapse state.
- **The observation consumers do not move.** Each plan keeps its own queue consumers (a bank's in
  `observe_queues`, a per-node lane's in its `EffectControlLane`). The control plane writes only the
  newest plan's queue, and #1280b applies the predecessor's pending records to its
  `ObservationLane` at the carry; #1327b then moves the tap state. So the Note's "moves the
  consumer" reads "applies the pending records, then moves the tap state".
- **Live retarget.** D1's "live differences retarget as #1280 D4" is #1280c (the per-instance
  function). Gate 3's second case needs it.
- **Shunt copy.** The per-node `BypassShunt` holds `dry_*`, `line_*` and one `cursor`; a bank's line
  is interleaved by lane. The copy helper of #1280a (D5) serves both directions; if it does not,
  this slice adds a `BypassShunt` lane-word copy in `crates/effect-contract/src/live.rs` (added to
  the authorized paths for that helper only).
- **Dependencies.** #1280a and #1280c (gate 3's second case). Not #1280b.

## Decision record: #1281 (batch 8)

**Shipped.** An insert instance that changes representation at a swap (a per-node live-controlled
effect to a lane of a `LiveControlEffectBankStage`, and back) carries its exact state, its control
lane (bypass, crossfade, symmetry terms, unread cells), its shunt words and its pending Observe
records. D1 is the one per-instance function of #1280c: a live difference retargets the moved
instance, a prepared one restarts the strip.

- `rack`: `EffectInstance` / `EffectInstanceMut` (processor, control lane, dry shunt of a per-node
  instance), `EffectLaneKind::of_instance`, and on `LiveControlEffectBankStage`
  `can_carry_instance`, `carry_lane_from_instance` (per node to bank: the instance's
  `snapshot_state_payload` into the bank's `restore_track_state_payload`, `carry_from`, the shunt
  line as lane `lane` of `lanes`, then `designed` and the `RESTORED` bit as for a lane carry) and
  `carry_lane_into_instance` (bank to per node, `&self`: the bank's
  `snapshot_track_state_payload` into `restore_state_payload`, `carry_from`, the shunt words as
  lane 0 of 1). D5 needed no new helper: `BypassShunt::carry_line_lane_from` serves a per-node
  shunt as a bank of one lane.
- `graph`: `GraphEffectSite { Lane(GraphLaneLocation), Node(unit) }` replaces the location of
  `GraphEffectMove` and `GraphEffectLane`; `bank_effect_lanes` also lists each
  `NodeKind::LiveControlEffect` (layout `None`); `Runtime::{has_effect_site, can_carry_effect_lanes,
  carry_effect_lanes}` handle the four site pairs (node to node is refused at install,
  `EffectMoveBetweenNodes`, and is #1282). D3: a bank predecessor is disengaged once per chain by
  `BankChain::disengage_for_carry` before any snapshot, as the lane carry does; a per-node
  predecessor has no collapse. `install_effect_carry` takes the joined scratch size
  (`scratch_bytes`), since a per-node instance has no layout in a bound plan.
- `host-core`: `EffectLaneInventoryRow.location` is a `GraphEffectSite`. A per-node row takes its
  layout from the prepared entries (`InstanceLayout`, `EffectLaneLayout::of(program_key())`), so it
  compares equal to a bank row exactly when the payloads match. `resolve_effect_carry` still
  restarts a strip when both sides of an owner are per-node (#1282).

**Departures.**

- Pending Observe records of a per-node predecessor are applied at the carry too (the amendment
  says the consumers do not move; applying them is #1280b's rule, so both directions follow it).
  A per-node instance has no clock, and a field on `LiveControlEffect` would break the estimate's
  field sum (`graph-compiler/src/estimate.rs`), so `GraphExecutor` keeps `next_block_sample` (the
  first sample after its last rendered block) and passes it to the carry.
- A per-node successor that refuses a restore has no stage to count it, so `GraphExecutor` keeps
  `node_carry_refused`, added to `carry_refusal_count`.
- Only the live forms move across representations: a per-node `NodeKind::Effect` (no control lane)
  has no row and a plain `EffectBankStage` pairs with no node. Both are reached only by test
  entries (amendment, batch 10). A strip with such an insert restarts, as before.
- `LiveControlEffectBankStage::carry_lane_into_instance` takes `&self`; the refusal is counted by
  the caller.

**Gates and tests** (`crates/host-core/tests/successor_swap.rs`; the four-lane cases are `ignore`d
on this AVX2 host, #1017):

| Gate | Test |
|---|---|
| 1 (per node to bank, compressor and limiter) | `a_group_of_inserts_becomes_a_bank_at_{eight,four}_lanes` |
| 2 (bank to per node) | `a_bank_of_inserts_falls_apart_at_{eight,four}_lanes` |
| 3 (unread value; live difference on moved lanes, both directions) | `live_values_cross_the_form_change_at_{eight,four}_lanes` |
| 4 (zero allocations and frees on the swap block) | `assert_clean_swap` in every case above |
| Observe records of a moved lane, both directions | `pending_observe_records_cross_the_form_change_at_eight_lanes` (the #1280b test, parameterised by session pair) |

Each form test first checks that A is per node and B banked (or the reverse) through
`bank_effect_lanes`, and that a swap whose strips restart renders other bits. Results:
`cargo test --locked --all-features` for `host-core` (422), `effect-contract` (87), `rack` (67),
`graph` (103), `graph-compiler` (178) and `capi` (153), and `host-web --features
host-web/test-support` (127), all green; clippy `-D warnings` and fmt clean;
`realtime-policy` ok.
