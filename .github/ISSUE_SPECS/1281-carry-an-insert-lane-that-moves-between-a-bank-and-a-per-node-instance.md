# Carry an insert lane that moves between a bank and a per-node instance

Slice 11b of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

An insert whose lane changes representation at a swap keeps its exact state: adding the eighth
compressor insert makes a group of seven per-node instances bank, and removing one makes a full bank
fall apart. Both directions continue bit for bit, live state included.

## Context

- Insert groups bank only when full (`crates/graph-compiler/src/banks.rs:50-77`): at 8 lanes, seven
  tracks with the same insert render per node and eight bank; at 4 lanes, three and four. #888 and
  #889 would bank partial groups; whichever lands later keeps this slice's gates.
- A per-node insert is `NodeKind::Effect(GraphPreparedEffect)` or `NodeKind::LiveControlEffect`
  (`crates/graph/src/runtime.rs:1135-1143`, struct `:1184`) with `processor: Box<dyn
  PreparedNativeEffect>`; a bank lane is a lane of an `EffectBankStage` or
  `LiveControlEffectBankStage` (`crates/rack/src/lib.rs:743`, `:930`).
- Per-node and bank payloads share one layout (`snapshot_state_payload`/`restore_state_payload`,
  `crates/effect-contract/src/lib.rs:1913`, `:1955`; the bank pair at `:2046-2056`), render-safe and
  exact mid-ramp after *Make every banked effect's state restore allocation-free* (slice 9).
- #1069 (*Multiband compressor: a ramp's cut moves a lane's bits in a bank*): a multiband lane with a
  ramp in flight is not bit-identical between a bank and a per-node instance until it lands.
- The inherited queue and the shunt copy come from *Carry live-controlled effect lanes across a plan
  swap* (#1280).

## Decisions frozen for this slice

- **D1. Rule.** As slice 10's D1.
- **D2. Mechanism.** Through the payload scratch: per-node `snapshot_state_payload` → bank
  `restore_track_state_payload`, or bank `snapshot_track_state_payload` → per-node
  `restore_state_payload`. The control state follows slice 11's `inherit_from`, and the shunt words
  move between the per-node `BypassShunt` and the lane's words of the bank line.

## Deliverables

1. D2 in `crates/graph` and `crates/rack`, and the join in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/rack/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`

## Non-goals

- No per-node to per-node move (slice 12).

## Objective gates

1. **Group becomes a bank, both widths.** At `Backend::Simd8`, seven tracks carry a compressor insert
   (per node); B adds an eighth, muted, so the group banks: every block bit-identical to the
   reference. At `Backend::Simd4`, the same with three and four tracks. Repeat with the limiter
   insert.
2. **Bank falls apart.** The reverse edit (remove one track from a full group): bit-identical.
3. **Live state.** Gate 1 with live controls and a compressor record pending at the swap:
   bit-identical to the reference fed the same record.
4. **Realtime.** Each swap block makes zero allocations and frees.
5. Commands:
   - `cargo test --locked -p rack -p graph -p host-core --features graph/test-support,host-core/test-support`
   - the umbrella's inherited gates.

## Test value

- Gate 1: a carry that handles only bank-to-bank leaves the new bank's lanes at rest; it turns red.
- Gate 2: the reverse direction is a separate code path (bank snapshot into a per-node restore); a
  carry that handles only gate 1's direction turns it red.
- Gate 3: control state lost across a representation change turns it red.

## Dependencies

- *Carry live-controlled effect lanes across a plan swap* (#1280).
