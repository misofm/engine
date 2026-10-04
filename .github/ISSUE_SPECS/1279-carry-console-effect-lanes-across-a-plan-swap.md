# Carry console effect lanes across a plan swap

Slice 10 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

Every console slot whose effect and values the transaction did not change keeps its exact state
through a plan swap, on every strip, when the strip's lane moves to another bank: EQ filter
memories, compressor, gate and transient-shaper envelopes, soft-clip filters, limiter look-ahead
lines. Adding a track to a playing session with a console EQ, compressor and limiter is
bit-continuous for every other track. Mono-source chains keep collapsing after the swap.

## Context

- Console groups always bank, padded (`crates/graph-compiler/src/banks.rs:50-77`); the eligible
  effects are `CONSOLE_ELIGIBLE_EFFECTS` (`crates/effect-compiler/src/prepare.rs:223-230`: EQ,
  compressor, gate/expander, soft-clip, transient shaper, true-peak limiter). A console slot without
  live controls is an `EffectBankStage` (`crates/rack/src/lib.rs:743`, impl `:793`) over a `Box<dyn
  PreparedNativeEffectBank>`.
- *Make every banked effect's state restore allocation-free* (#1278) makes the lane payload calls
  render-safe (`snapshot_track_state_payload` and `restore_track_state_payload`,
  `crates/effect-contract/src/lib.rs:2046-2056`). A collapsed bank must be desymmetrized before a
  snapshot (`desymmetrize_channels`, `:2154`; doc `:2033-2045`).
- `EffectBankStage` caches each lane's designed-word symmetry at bind (`designed`, `:747-764`) on the
  premise that restore never runs on a bound bank. Every witness term a stage caches for a lane must
  be refreshed after a restore, the RESTORED term included (`ChannelSymmetryWitness`,
  `crates/effect-contract/src/symmetry.rs`).
- The builtin carry, the rack slot accessor, `as_any_mut` and the chain-flag rule come from *Carry
  fader, mute and pan ramps across a plan swap* (#1277) and its predecessor.

## Decisions frozen for this slice

- **D1. Key and rule** (for every effect owner; slices 11-12 reuse it). Owner key `(strip ID, rack,
  effect ID)` (`EffectNodeId`; a console slot's effect ID is its slot ID). It carries when the
  effect's identity, quality and link mode (a console slot's come from the session slot), its
  `bypass` and every parameter value (a console slot's come from the strip's entry) are bit-equal in
  the committed model before the transaction and in the successor's model, and the inventory's
  prepared layout matches (prepared bypass, latency, `state_sizes`, `state_layout_version`). Padding
  lanes never carry.
- **D2. Mechanism.** At the swap block, per carried lane: desymmetrize a collapsed predecessor bank
  once; snapshot the predecessor lane into one scratch buffer; restore it into the successor lane.
  The successor preallocates the scratch at the largest carried `state_sizes.total()`.
- **D3. Witness refresh.** After restoring a lane, recompute every term the stage caches for it (the
  `designed` flag and any RESTORED term), and update the cache's rustdoc: restore now also runs on a
  bound bank, at the swap block only. Chains take slice 7's AND rule for their agreement flag.
- **D4. Refusal.** A restore that refuses leaves the lane as prepared (at rest) and increments a
  carry-refusal counter readable after render. It never panics on the render thread.

## Deliverables

1. D1-D4 in `crates/rack` and `crates/graph`, and the inventory rows and join in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/rack/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`
- `crates/effect-contract/src/lib.rs` (docs only)

## Non-goals

- No live-controlled bank lanes (slice 11), no bank ↔ per-node flips (slice 11b).
- No whole-bank move (umbrella Deferred).

## Objective gates

1. **Gap-free acceptance, every console effect, both widths.** At `Backend::Simd8` and
   `Backend::Simd4`: session A is `parametric-eq-nine-track.json` with a console of all six
   `CONSOLE_ELIGIBLE_EFFECTS` in `pre_insert` and `post_insert`, configured so each is doing work on
   the test signal (EQ gain, compressor and limiter reducing gain, gate opening and closing). Session
   B adds a muted track whose ID sorts first, so every lane shifts. The swapped run (swap after block
   6) and a fresh B fed from frame 0 are bit-identical for every block. With this slice's section of
   the program disabled, block 7 differs.
2. **Collapse kept.** On a mono-source session with the same console, the successor's chains collapse
   in the blocks after the swap (`bank_collapse_counters` grows), and the output stays bit-identical.
3. **Rule.** A strip whose compressor threshold the transaction changed is not carried for that slot;
   its other slots are.
4. **Realtime.** The swap block of gate 1 makes zero allocations and frees, with every console effect
   carried (`bench_support::alloc` thread counters and the engine render audit).
5. Commands:
   - `cargo test --locked -p rack -p graph -p host-core --features graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `cargo test --locked -p console-workload`
   - the umbrella's inherited gates.

## Test value

- Gate 1: a lane restored to its old lane index instead of its strip's new lane, or an effect whose
  restore misses a derived word, turns it red.
- Gate 2: a stale `designed` cache, or an agreement flag cleared for good, either collapses on
  asymmetric words (bits move) or never collapses again; one of gates 1-2 turns red.
- Gate 4: a payload call that still allocates on any console effect turns it red.

## Dependencies

- *Carry fader, mute and pan ramps across a plan swap* (#1277).
- *Make every banked effect's state restore allocation-free* (#1278).
