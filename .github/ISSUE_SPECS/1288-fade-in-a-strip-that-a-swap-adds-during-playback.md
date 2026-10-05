# Fade in a strip that a swap adds during playback

Stream D of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9).
Slice 18 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

Owner question Q3 of #1269 (fade in added strips, and over what length) is answered by decision 15
D15-9: yes, from the strip's first played block, over the session mute ramp. No blocker remains.

## Product outcome

A stem that a structural edit adds while audio plays enters with a fade instead of a step. A stem
rarely starts at a zero crossing, so without a fade its first block is a click. Strips the edit did
not add are not touched: their output stays bit-identical to the reference. The same armed fade is
the "successor fades in" half of the duck-swap (#1324).

## Context

- An added strip's nodes start at rest (#1269 P1). Its source is either a persisting source, which
  plays from the swap block, or an added source the host starts with
  `miso_engine_v1_source_seek_at` at a quantum-aligned render sample `A`
  (`crates/capi/include/miso_engine_v1.h:96-108`, #1275, closed); before `A` it underruns.
- Fader and mute state: `FaderRampStage` (`crates/builtins/src/lib.rs:2653`), `[channel][lane]`
  arrays of gain, mute and `remaining`; `FaderLane { gain, muted }` (`:798`). The stage's
  `set_mute` (`:2774`) retargets to 0 or back to the fader gain over a window, and a settled mute is
  exact `+0.0` (doc at `:4150-4162`). Per-node form: `FaderMuteRampBuiltins` (`:4163`).
- Owners in `crates/builtins-compiler/src/lib.rs`: `FaderBankProcessor` (`:715`), the browser's
  fused `FaderMatrixBankProcessor` (`:831`), `LiveControlFaderProcessor` (`:4377`). Graph traits:
  `GraphPreparedBuiltinBankProcessor` (`crates/graph/src/lib.rs:1285`), `GraphRuntimeProcessor`
  (`:2447`).
- Render entry: `GraphExecutor::render` (`crates/graph/src/lib.rs:3228`) calls the source set's
  `begin_block` (`:3256`). `GraphPreparedSourceSetDriver::played_planes` (`:2200`) is `Some` exactly
  when a claim's source played a block (`crates/source/src/lib.rs:1793`; `None` on underrun).
  `provides_played_planes` (`crates/graph/src/lib.rs:2189`) is true for the production driver
  (`crates/source/src/lib.rs:1780`).
- A successor is prepared with `SuccessorBase { inventory, committed }`
  (`crates/host-core/src/prepare.rs:641-647`). `committed` is the predecessor's normalized model, so
  "strip ID absent from it" is decidable on the control thread. `PlanStateInventory` (`:558`).
- Ramp lengths: `LiveRamps::for_session` (`crates/host-core/src/live_delta.rs:52`) returns 0 until
  #1054 derives `mute_samples` from the session's `controlSmoothing`.
- Route compensation: `RouteTiming { compensation_delay, .. }` (`crates/graph/src/lib.rs:337`),
  listed in `route_timings` (`:854`).

## Decisions frozen for this slice

- **D1. Which strips.** Exactly the strips (tracks and submixes) whose ID is absent from
  `base.committed`. Never at a boot (no base). Never a strip that existed before. Per channel: a
  channel the model mutes stays muted and is not armed.
- **D2. Arm (builtins).** `FaderLane` gains `armed: bool`. An armed channel is prepared muted
  (settled `+0.0`) and remembers its fader gain. `FaderRampStage` keeps `armed: [[bool; MAX]; 2]`
  and gains `fire_fade_in(lane, ramp_samples)`: for each armed channel of the lane, clear the flag
  and run the existing `set_mute(.., false, ramp_samples)`. Any `set_mute` on a channel clears its
  flag first (a user mute wins; a later unmute is an ordinary unmute). A fader record while armed
  only updates the remembered gain. `fire_fade_in` on an unarmed lane does nothing.
- **D3. Fire (graph).** The prepared graph plan holds a fixed table of fade arms, one per armed
  strip: `{ claim: Option<u32>, owner, lane, delay_samples: u32, ramp_samples: u32, state }`, with
  `state` one of `Waiting`, `FireAt(u64)`, `Done`, plus a count of entries not `Done`. In
  `GraphExecutor::render`, after `begin_block` and only while that count is nonzero:
  - `Waiting` becomes `FireAt(block_start + delay_samples)` when the claim's `played_planes` is
    `Some`, or at once when `claim` is `None` (a submix) or the driver does not provide played
    planes;
  - `FireAt(s)` with `s <= block_start` calls the owner's `fire_fade_in` and becomes `Done`.

  So the fade starts at the first block boundary at or after the moment the cut reaches the fader;
  the frames of a partial block before it stay muted (zeros, never a step). New trait methods with
  a no-op default: `GraphPreparedBuiltinBankProcessor::fire_fade_in(&mut self, lane, ramp)` and
  `GraphRuntimeProcessor::fire_fade_in(&mut self, ramp)`, implemented by the three owners above.
- **D4. Delay `D` (host-core).** For a track: the sum of the latencies of the strip's nodes from its
  input to its fader in the successor (a latent insert at rest emits zeros for its latency). For a
  submix: that sum plus the largest `compensation_delay` of a route into it whose line starts at
  rest. Computed on the control thread from the compiled successor.
- **D5. Length.** `ramp_samples` = `LiveRamps::for_session(successor model).mute_samples`, the
  session mute ramp. After the ramp the stage is settled and the output equals the reference bit
  for bit (the ramp's exact end assignment).
- **D6. Carry.** The armed bits join the fader lane state that *Carry fader, mute and pan ramps
  across a plan swap* (#1277) exports and imports, and the inventory records which strips a plan
  armed. A successor arms again (D1 flags set, a fresh `Waiting` entry) every strip its base's
  inventory marks armed. The carry then overwrites the lane: a lane that already fired imports
  `armed = false`, so its new entry is a no-op; a lane still waiting keeps waiting.
- **D7. Cost.** No allocation; the table is sized at preparation. With no arms, render pays one
  integer test per block. Unarmed strips are untouched.

## Deliverables

- **Part A (render mechanism).** D2 in `crates/builtins`; the owners' `fire_fade_in` and armed
  construction in `crates/builtins-compiler`; D3 in `crates/graph`.
- **Part B (arming).** D1 and D4-D6 in host-core successor preparation, with the inventory row.
- The tests below. The fade documented in `crates/capi/include/miso_engine_v1.h` (a comment beside
  the issue-1275 paragraph) and in `docs/C_ABI_V1_QUALIFICATION.md`.

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`
- `crates/capi/include/miso_engine_v1.h` (comments only), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Edited and removed strips: *Duck-swap a strip whose state cannot continue across a plan swap*
  (#1324) and *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325).
- No declick of an ordinary seek.
- No crossfade between two plans' outputs (#1269 P6). A true crossfade with ghost strips is
  deferred by D15-9; it reopens on a measured, audible dip in a listening test.

## Objective gates

1. **Kernel (Part A).** Builtins unit tests at both bank widths and in the per-node form: an armed
   lane outputs exact `+0.0`; after `fire_fade_in(lane, N)` its output is bit-identical to an
   unarmed muted lane given `set_mute(false, N)` at the same block; a `set_mute(true)` before the
   fire leaves the lane muted after it; a fader record while armed is the gain the fade reaches.
2. **Fire time (Part A).** A graph test with the played-planes fake (`PlayedSource`,
   `crates/graph/src/runtime.rs:13288`) at quantum 128: a claim that first plays at block 5 with
   `D = 200` fires at the block that starts at sample 896, not at 640 or 768.
3. **Only added strips fade.** In `successor_swap.rs`, at `Backend::Simd8` and `Backend::Simd4`: a
   transaction adds a track whose source is fed exact zeros; every block of the swapped run equals a
   run with no add.
4. **Fade shape.** With every other strip muted: (a) an added track on a playing source; (b) an
   added source started by `seek_at` at `A`, three blocks after the swap; (c) case (b) with a
   true-peak limiter insert (`D > 0`). Each output equals a fresh plan of the successor session, fed
   the same source frames at the same blocks, with the strip muted and given a live unmute of `N`
   samples at the D3 fire block: bit-identical for every block.
5. **Arm survives a second swap.** Case 4(b) with a second structural transaction swapped in before
   `A`: the same output.
6. **Browser form.** Gate 4(a) with both runs prepared between render calls
   (`FaderMatrixBankProcessor`).
7. **Realtime.** Extend `the_swap_block_allocates_and_frees_nothing` (`successor_swap.rs:476`) over
   the fade blocks of gate 4(c): zero allocations and frees.
8. Commands:
   - `cargo test --locked --all-targets -p builtins --features math/lane,builtins/test-support,lane/test-support`
   - `cargo test --locked -p builtins-compiler -p graph -p host-core -p capi --features builtins-compiler/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-audit.sh target/release/audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts && bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/check-builtins-policy.sh`, `bash scripts/check-graph-policy.sh`,
     `bash scripts/check-host-core-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-cross-targets.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: a user mute that does not disarm the lane (the fire unmutes a strip the user muted) turns
  it red.
- Gate 2: a fire at the played block that ignores `D` (the ramp runs while a latent insert still
  emits its at-rest zeros, so the cut stays a step) turns it red.
- Gate 3: a fade applied to every strip at the swap (a global dip) turns it red.
- Gate 4(b): a fade that starts at the swap block for an anchored stem (finished before the stem
  plays, so the click remains) turns it red.
- Gate 5: a successor that does not re-arm from the inventory lets the stem pop in at full gain; it
  turns red.
- Gate 6: arming only the split banks leaves the browser's fused form unfaded; it turns red.

## Dependencies

- *Carry fader, mute and pan ramps across a plan swap* (#1277).
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054). Until it lands the session mute ramp is 0 and the fade is a step.
