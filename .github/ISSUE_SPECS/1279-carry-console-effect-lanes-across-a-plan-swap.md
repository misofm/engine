# Carry console effect lanes across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8).
Slice 10 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

A console slot keeps its exact state through a plan swap when the transaction changed none of its
prepared values. This holds on every strip, also when the strip's lane moves to another bank, and
in move mode and copy mode. The state covers EQ filter memories, compressor, gate and
transient-shaper envelopes, soft-clip filters and limiter look-ahead lines. Adding a track to a
playing session with a console EQ, compressor and limiter is bit-continuous for every other track.
Mono-source chains keep collapsing after the swap.

## Context

- Console groups always bank, padded (`pads`, `crates/graph-compiler/src/banks.rs:76`). The
  eligible effects are `CONSOLE_ELIGIBLE_EFFECTS` (`crates/effect-compiler/src/prepare.rs:223-230`):
  EQ, compressor, gate/expander, soft-clip, transient shaper and true-peak limiter.
- A console slot without live controls is an `EffectBankStage` (`crates/rack/src/lib.rs:750`, impl
  `:800`) over a `Box<dyn PreparedNativeEffectBank>`. With live controls it is #1280's
  `LiveControlEffectBankStage`.
- The lane payload calls are render-safe and exact mid-ramp (#1278, on `main`):
  `snapshot_track_state_payload(&self, …)` and `restore_track_state_payload`
  (`crates/effect-contract/src/lib.rs:2070`, `:2079`). A collapsed bank must be desymmetrized before
  a snapshot (`desymmetrize_channels`, `:2182`). Two docs still say that no engine path snapshots a
  bound bank (`:2066`, `:2178`).
- `EffectBankStage` caches each lane's designed-word symmetry at bind (`designed`, `:755-771`), on
  the premise that restore never runs on a bound bank. Every witness term a stage caches for a lane
  must be refreshed after a restore, the RESTORED term included (`ChannelSymmetryWitness`,
  `crates/effect-contract/src/symmetry.rs:155`).
- The base, the live/prepared split, the restart set and the copy-mode rules come from *Carry
  fader, mute and pan ramps across a plan swap* (#1277, D2-D6) and *Carry plan state by copy as
  well as by move* (#1322).

## Decisions frozen for this slice

- **D1. Key and rule** (every effect owner; #1280-#1282 reuse it). The owner key is `(strip ID,
  rack, effect ID)` (`EffectNodeId`); a console slot's effect ID is its slot ID. The owner's
  **prepared values** are:
  - its identity, quality and link mode (a console slot's come from the session slot);
  - every value the classifier refuses as `Prepared`, `PreparedBypass` or `Domain`;
  - for an owner with no live control lane, every parameter value and its bypass.

  The owner carries when no prepared value differs between the predecessor's base (#1277 D2) and
  the successor's model, and the inventory's prepared layout matches: prepared bypass, latency,
  `state_sizes` and `state_layout_version`. Otherwise it starts at rest, and its strip goes into
  the restart set (#1277 D6). Padding lanes never carry.

  A strip whose effect chain changed also goes into the restart set, even when its remaining owners
  carry (D15-9). A changed chain means an insert added, removed or reordered, or a session console
  slot added or removed, which changes every strip's chain. The chain's processing changes, so a
  carried neighbour does not make it continuous.
- **D2. Live differences.** An `EffectBankStage` has no live lane, so every difference on it is
  prepared (D1), and this slice emits no retarget. #1280 retargets live-controlled lanes.
- **D3. Move mode.** At the swap block, for each carried lane:
  1. desymmetrize a collapsed predecessor bank, once per bank;
  2. snapshot the predecessor lane into one scratch buffer;
  3. restore it into the successor lane.

  The successor preallocates the scratch at the largest carried `state_sizes.total()`.
- **D4. Copy mode.** The same three steps. Desymmetrizing moves none of the predecessor's bits
  (#1322 D2), and the snapshot takes `&self`. The bytes go into `carry_program_copy_bytes`.
- **D5. Witness refresh.** After a lane is restored, recompute every term the stage caches for it
  (the `designed` flag and any RESTORED term), and update the cache's rustdoc: restore now also runs
  on a bound bank, in the swap block or copy block only. Chains take #1276's AND rule for their
  agreement flag. Update the two "no engine path snapshots a bound bank" docs.
- **D6. Refusal.** A restore that refuses leaves the lane as prepared (at rest) and increments a
  carry-refusal counter readable after render. It never panics on the render thread. #1278 makes a
  refusal of an own payload a defect, so gate 1 asserts the counter is 0.

## Deliverables

1. D1-D6 in `crates/rack` and `crates/graph` (both modes), and the inventory rows and the join in
   `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/rack/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`
- `crates/effect-contract/src/lib.rs` (docs only)

## Non-goals

- No live-controlled bank lanes (#1280), and no bank to per-node flips (#1281).
- No whole-bank move (#1269, Deferred; it reopens only if #1286 shows that lane copies dominate).
- No transition for a restarted strip (#1324).

## Objective gates

1. **Gap-free acceptance, every console effect, both widths.** At `Backend::Simd8` and
   `Backend::Simd4`, session A is `parametric-eq-nine-track.json` with a console of all six
   `CONSOLE_ELIGIBLE_EFFECTS` in `pre_insert` and `post_insert`. Each effect does work on the test
   signal: EQ gain, the compressor and the limiter reducing gain, the gate opening and closing.
   Session B adds a muted track whose ID sorts first, so every lane shifts. The swapped run (swap
   after block 6) and a fresh B fed from frame 0 are bit-identical in every block. The refusal
   counter is 0. With this slice's section of the program disabled, block 7 differs.
2. **Collapse kept.** On a mono-source session with the same console, the successor's chains
   collapse in the blocks after the swap (`bank_collapse_counters` grows), and the output stays
   bit-identical.
3. **Prepared change restarts.** B changes one strip's compressor threshold. On this owner it is
   prepared: an `EffectBankStage` has no live lane. That slot does not carry, its other slots do,
   and `restarted_strips()` is exactly that strip. A second case adds a compressor insert to one
   strip: its console slots carry, and the strip is in the restart set.
4. **Copy mode.** Gate 1 and gate 2 with a copy after block 6, followed at once by the adoption.
   Every block equals the move run, and A, rendered on after the copy, equals its uncopied twin
   (collapsed case included).
5. **Realtime.** The swap block of gate 1, and the copy call of gate 4, each make zero allocations
   and frees with every console effect carried (`bench_support::alloc` thread counters and the
   engine render audit).
6. Commands:
   - `cargo test --locked -p rack -p graph -p host-core --features rack/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `cargo test --locked -p console-workload`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`

## Test value

- Gate 1: a lane restored to its old lane index instead of its strip's new lane, or an effect whose
  restore misses a derived word, turns it red.
- Gate 2: a stale `designed` cache, or an agreement flag cleared for good, either collapses on
  asymmetric words (bits move) or never collapses again. One of gates 1-2 turns red.
- Gate 3: a rule that carries an owner whose prepared value changed keeps the old threshold. A
  restart set that misses effect owners fails the exact list. Either turns it red.
- Gate 4: a copy-mode desymmetrize that is not bit-neutral fails the twin, and a copy into the
  wrong successor lane fails the move comparison.
- Gate 5: a payload call that still allocates on any console effect turns it red.

## Dependencies

- *Carry fader, mute and pan ramps across a plan swap* (#1277).
- *Carry plan state by copy as well as by move* (#1322).
- #1278 is on `main`.
