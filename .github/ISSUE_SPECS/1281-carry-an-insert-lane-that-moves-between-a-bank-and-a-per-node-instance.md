# Carry an insert lane that moves between a bank and a per-node instance

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8).
Slice 11b of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

An insert whose lane changes representation at a swap keeps its exact state. Adding the eighth
compressor insert makes a group of seven per-node instances bank, and removing one makes a full bank
fall apart. Both directions continue bit for bit, live state included. The carry moves the state at
the swap block, and the predecessor never renders again.

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
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: a carry that handles only bank-to-bank leaves the new bank's lanes at rest. It turns red.
- Gate 2: the reverse direction is a separate code path (a bank snapshot into a per-node restore).
  A carry that handles only gate 1's direction turns it red.
- Gate 3: control state lost across a representation change, or a retarget routed to the old
  representation's producer, turns it red.

## Dependencies

- *Carry live-controlled effect lanes across a plan swap* (#1280).
