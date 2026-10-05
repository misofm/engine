# Carry meter and effect observation state across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-14, D15-7).
Code anchors verified on `main` at `6fb211594`.

This slice carries builtin meters and effect observation taps. The spectrum observer is *Carry
spectrum capture state across a plan swap* (#1395), which reuses D1-D4's rules for its own key
and its shared mode word.

## Product outcome

A producer watching a strip's level or an effect's gain reduction sees no reset when a track is
added or another strip is edited. A meter or observation tap is unchanged when its owner's key and
configuration are the same in both plans. Such an owner keeps its open
window, its counters and its sequence through a plan swap. The host's existing reader keeps
receiving, with no generation bump, no counted loss, and no window relabelled. This replaces
#1269's P10 ("restart at the swap"), which decision 15 rejects as a shortcut (D15-14). Only
changed or added owners start fresh.

## Context

- **Builtin meters.**
  - A meter is a graph observer, `MeterObserver(MeterAccumulator)`
    (`crates/builtins-compiler/src/lib.rs:4820`, impl `:4821`).
  - `MeterAccumulator` (`crates/builtins/src/lib.rs:4553`) holds the window state (`start`,
    `frames`, `sequence`, two `MeterLane`s and the cumulative counters) and the
    `Producer<MeterSnapshot>`.
  - The host reads through `MeterConsumer` (`builtins-compiler/src/lib.rs:289`), returned in
    `HostLiveControlHandles::meters` (`crates/host-core/src/prepare.rs:445`) and requested by
    `HostMeterRequest` (`:399`).
  - A snapshot carries `reset_generation` and `window_sequence` (`builtins/src/lib.rs:4505-4518`).
- **Effect observation taps.**
  - `ObservationLane` (`crates/effect-contract/src/live.rs:588`) is held per node by
    `LiveControlEffect::observation` (`crates/graph/src/runtime.rs:1196`) and per bank lane by
    `LiveControlEffectBankStage::observations` (`crates/rack/src/lib.rs:961`).
  - Readers reach the host as `HostLiveControlHandles::effect_observations` (`prepare.rs:452`).
- **Spectrum** is #1395. `SpectrumCaptureObserver` (`crates/host-core/src/spectrum.rs:1441`) is a
  graph observer too, so #1395 registers it in this slice's observer location table (D2).
- **The browser** bumps `meter_generation` and counts a loss when a snapshot's `reset_generation`
  changes (`hosts/host-web/src/lib.rs:3545-3561`), which is what a swap to fresh meters causes.
- **The C ABI** has no strip meter, observation tap or spectrum capture. Its telemetry is the
  output peak each render publishes (`crates/capi/src/runtime/plan.rs:234`, read by
  `collect_render_activity`, `crates/capi/src/runtime/control.rs:522`). That is continuous across a
  swap already, so this slice has nothing to carry there.
- The carry scaffold, the effect carries and the source-producer precedent
  (`SourceControlSet::adopt_persisting`, `crates/host-core/src/source.rs:286`) are on `main` or come
  from #1279-#1284.

## Decisions frozen for this slice

- **D1. Keys and rule (P1).** There are two kinds of owner here:
  - A **meter** carries when its strip ID, tap, metric set and `MeterConfig` are equal in both
    plans.
  - An **observation tap** carries when its effect owner carried (#1279-#1282) and its tap
    descriptor is the same.

  Nothing in them is a live value, so there is no retarget. A non-carried owner starts fresh and
  is not a strip restart: it changes no audio.
- **D2. Move mode.** At the swap block, swap each carried observer's whole state between the plans,
  its producer included. The host's existing reader is therefore still
  connected to the observer that renders on. The successor's fresh observer goes to the retiring
  predecessor.
  The observer location table records each observer's owner kind and key, not its handle, so
  #1395 adds the spectrum kind without changing the section.
- **D3. Control side.** Host-core gives one adoption call per kind, after
  `SourceControlSet::adopt_persisting`. Each call swaps the reader of every carried owner between
  the successor's handles and the predecessor's, and returns the count. The host calls them right
  after the transaction commits, as it does for sources.
- **D4. Copy mode belongs to #1287.** In a warm successor the observers render `[B, S + P)` during
  the catch-up, which overlaps windows the predecessor publishes. How they continue depends on the
  clock *Pre-roll a successor whose latency grows* (#1287) gives the successor at adoption. So #1287
  carries observers through its catch-up under D1's keys:
  - copy the window state at `B`;
  - publish nothing until adoption;
  - at `S`, take the predecessor's producer and publish only the windows whose sequence the
    predecessor did not publish.

  This slice's program sections are move-mode only. #1287 must not adopt a warm successor whose
  observers it does not carry.
- **D5. Both hosts.** Everything is in host-core, graph, rack, builtins and effect-contract.
  - The browser gets it with no browser code of its own once it replaces plans through the shared
    control plane: *Run the browser control plane in a Worker and keep the AudioWorklet
    render-only* (#1332), then *Replace the running browser session in the Rust host* (#1290). #1290
    calls D3's adoption calls, and its gate asserts that `meter_generation` does not advance across
    a replacement that carries every meter.
  - The C ABI has nothing to carry (Context).

## Deliverables

1. D2 in `crates/builtins` (`MeterAccumulator` swap), `crates/builtins-compiler` (`MeterObserver`),
   `crates/effect-contract/src/live.rs` (`ObservationLane`), `crates/rack` and `crates/graph` (the
   observer location table and the program section).
2. D1 and D3 in `crates/host-core/src/prepare.rs`.
3. Tests in `crates/host-core/tests/successor_swap.rs`, through the browser's preparation entry
   points (`prepare_host_runtime_with_selected_meters_between_render_calls`,
   `crates/host-core/src/prepare.rs:1051`, and, for the observation taps,
   `prepare_host_runtime_with_live_controls_successor`, `:926`).

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`, `crates/rack/src/lib.rs`
- `crates/effect-contract/src/live.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`

## Non-goals

- No copy mode (D4). No browser code (#1290). No change to meter or spectrum kernels.
- No spectrum carry (#1395).

## Objective gates

1. **Meters carry, both widths.** Prepare A with meters at every tap of three strips, with a window
   several blocks long, so a window is open at the swap. B adds a muted track whose ID sorts first.
   Read through A's consumers before and after the swap. The snapshot sequence has no gap, keeps
   `reset_generation`, has `cumulative_discontinuities` 0, and every snapshot equals the reference
   (a fresh B with the same meters, fed from frame 0) field by field.
2. **A changed meter starts fresh.** B changes one meter's window length. That meter's successor
   reader starts a new sequence, and every other meter carries as in gate 1.
3. **Observation taps.** With a live-controlled compressor insert and its gain-reduction tap armed,
   the tap's readings continue across the swap, equal to the reference. A compressor that restarts
   (its threshold changed, prepared on a lane without live controls) starts its tap fresh.
4. **Realtime.** The swap block makes zero allocations and frees.
5. Commands:
   - `cargo test --locked -p builtins -p builtins-compiler -p effect-contract -p rack -p graph -p host-core --features builtins-compiler/test-support,rack/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1 turns red on any of four defects:
  - a carry that copies the window but keeps the successor's producer leaves the host's reader on a
    dead queue;
  - a meter keyed by handle index instead of `(strip, tap)` reads the wrong strip after the lane
    shift;
  - a window restarted at the swap shows a counted discontinuity;
  - a lost open window shows a sequence gap.
- Gate 2: a rule that ignores the configuration carries a window of the wrong length. It turns red.
- Gate 3: observation state that does not follow its effect owner restarts the tap on an unchanged
  effect. It turns red.

## Dependencies

- *Carry strip delay lines and live send ramps across a plan swap* (#1284), and through it every
  effect carry that observation taps follow.
