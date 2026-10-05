# Fade in a strip that a swap adds during playback

Stream D of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9).
Slice 18 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

Owner question Q3 of #1269 (fade in added strips, and over what length) is answered by decision 15
D15-9: yes, from the strip's first played block, over the session mute ramp. No blocker remains.

Split (R10): this slice is the fade for one swap. Keeping an arm that has not fired yet across a
later swap is the successor *Keep an added strip's pending fade-in across a later plan swap*
(#1392). The two merge to `main` in the same batch.

## Product outcome

A stem that a structural edit adds while audio plays enters with a fade instead of a step. A stem
rarely starts at a zero crossing, so without a fade its first block is a click. Strips the edit did
not add are not touched: their output stays bit-identical to the reference. The same armed fade is
the "successor fades in" half of the duck-swap (#1324). It starts no earlier than the block at
which render adopts the plan, so it composes with a warm successor (stream C, #1287) that rendered
the strip off the render thread before adoption.

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
- Adoption hook: at a swap, render calls the incoming plan's `carry_from`
  (`crates/engine/src/realtime/plan_exchange.rs:421`), which runs the executor's
  `adopt_predecessor` (`crates/engine/src/realtime/plan.rs:311`, `:917`; the graph executor's at
  `crates/graph/src/lib.rs:3139`). A synchronous host reaches the same hook through
  `PreparedRenderPlan::adopt_predecessor_plan` (`plan.rs:899`). It runs once per adoption, on the
  render thread, before the plan's first adopted block. An off-thread catch-up slice (#1321) and a
  copy-mode snapshot (#1354) never call it.

## Decisions frozen for this slice

- **D1. Which strips.** Exactly the strips (tracks and submixes) whose ID is absent from
  `base.committed`. Never at a boot (no base). Per channel: a channel the model mutes stays muted
  and is not armed. The arm entry point takes the strip set as an argument, so the duck-swap
  (#1324 D3) and a restore over a duck (#1325 D6) arm their strips through the same code.
- **D2. Arm (builtins).** `FaderLane` gains `armed: bool`. An armed channel is prepared muted
  (settled `+0.0`) and remembers its fader gain. `FaderRampStage` keeps `armed: [[bool; MAX]; 2]`
  and gains `fire_fade_in(lane, ramp_samples)`: for each armed channel of the lane, clear the flag
  and run the existing `set_mute(.., false, ramp_samples)`. Any `set_mute` on a channel clears its
  flag first (a user mute wins; a later unmute is an ordinary unmute). A fader record while armed
  only updates the remembered gain. `fire_fade_in` on an unarmed lane does nothing.
- **D3. Fire (graph).** The prepared graph plan holds a fixed table of fade arms, one per armed
  strip: `{ claim: Option<u32>, owner, lane, delay_samples: u32, ramp_samples: u32, state }`, with
  `state` one of `Waiting`, `FireAt(u64)`, `Done`, plus a count of entries not `Done`, plus one
  `adopted: bool`, false at preparation. In `GraphExecutor::render`, after `begin_block` and only
  while that count is nonzero:
  - `Waiting` becomes `FireAt(block_start + delay_samples)` when the claim's `played_planes` is
    `Some`, or at once when `claim` is `None` (a submix) or the driver does not provide played
    planes. This runs on every block the plan renders, off-thread catch-up blocks included, so
    the first played block is measured in the plan's own render time;
  - `FireAt(s)` with `s <= block_start` **and `adopted`** calls the owner's `fire_fade_in` and
    becomes `Done`.

  The graph executor's `adopt_predecessor` sets `adopted = true` on entry, before any of its early
  returns (`crates/graph/src/lib.rs:3139-3142` returns early without a carry program). So the fade
  starts at the first block at or after `max(S, first played block + D)`, where `S` is the adopted
  block. For a warm successor whose catch-up already played the source through its latent nodes,
  that is `S` itself; no special case and no `D = 0` override exists. The frames before the fire
  stay muted (zeros, never a step). New trait methods with
  a no-op default: `GraphPreparedBuiltinBankProcessor::fire_fade_in(&mut self, lane, ramp)` and
  `GraphRuntimeProcessor::fire_fade_in(&mut self, ramp)`, implemented by the three owners above.
- **D4. Delay `D` (host-core).** For a track: the sum of the latencies of the strip's nodes from its
  input to its fader in the successor (a latent insert at rest emits zeros for its latency). For a
  submix: that sum plus the largest `compensation_delay` of a route into it whose line starts at
  rest. Computed on the control thread from the compiled successor.
- **D5. Length.** `ramp_samples` = `LiveRamps::for_session(successor model).mute_samples`, the
  session mute ramp. After the ramp the stage is settled and the output equals the reference bit
  for bit (the ramp's exact end assignment).
- **D6. One swap.** An arm lives in the plan that armed it. A superseding candidate prepared before
  that plan is adopted re-arms the strip by D1 (its base is the plan render runs, where the strip is
  absent, #1310 D2). An arm still waiting when the adopted plan is itself succeeded is carried by
  *Keep an added strip's pending fade-in across a later plan swap* (#1392; it needs the fader lane
  export of #1277). This slice and #1392 merge to `main` in the same batch (Hazards).
- **D7. Cost.** No allocation; the table is sized at preparation. With no arms, render pays one
  integer test per block. Unarmed strips are untouched.

## Deliverables

- **Part A (render mechanism).** D2 in `crates/builtins`; the owners' `fire_fade_in` and armed
  construction in `crates/builtins-compiler`; D3 in `crates/graph`.
- **Part B (arming).** D1, D4 and D5 in host-core successor preparation.
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
- The added strip's routes whose tap precedes its fader (`input`, `post_input`, `insert_send`,
  `insert_return`, `pre_fader`; `SendTap`, `crates/session/src/model.rs:856-871`) bypass the fader
  this slice arms. *Ramp a route that a plan swap adds to or removes from a surviving strip*
  (#1363) arms each of them with this strip's table entry (its D1 (c)); it needs this slice's
  table first.
- An arm carried across a later swap (D6): #1392.
- No declick of an ordinary seek.
- No crossfade between two plans' outputs (#1269 P6). A true crossfade with ghost strips is
  deferred by D15-9; it reopens on a measured, audible dip in a listening test.

## Hazards

- On its own, this slice would let a second structural swap adopted after this plan and before an
  arm fires (an added source anchored far ahead) start the strip unarmed in the newer plan, so it
  would enter at full gain. #1392 carries the arm and its second-swap gate turns that case red.
  Therefore this slice and #1392 merge to `main` in the same batch: root does not push this slice
  at a batch boundary without #1392.

## Objective gates

1. **Kernel (Part A).** Builtins unit tests at both bank widths and in the per-node form: an armed
   lane outputs exact `+0.0`; after `fire_fade_in(lane, N)` its output is bit-identical to an
   unarmed muted lane given `set_mute(false, N)` at the same block; a `set_mute(true)` before the
   fire leaves the lane muted after it; a fader record while armed is the gain the fade reaches.
2. **Fire time (Part A).** A graph test with the played-planes fake (`PlayedSource`,
   `crates/graph/src/runtime.rs:13288`) at quantum 128: (a) a plan adopted (through
   `adopt_predecessor_plan`) before block 0, whose claim first plays at block 5 with `D = 200`,
   fires at the block that starts at sample 896, not at 640 or 768. (b) Adoption composes: the same
   plan rendered blocks 0-7 unadopted (the claim plays from block 2, `D = 200`), then adopted
   before block 8, fires at the block that starts at sample 1024 (block 8), not at 640.
3. **Only added strips fade.** In `successor_swap.rs`, at `Backend::Simd8` and `Backend::Simd4`: a
   transaction adds a track whose source is fed exact zeros; every block of the swapped run equals a
   run with no add.
4. **Fade shape.** With every other strip muted: (a) an added track on a playing source; (b) an
   added source started by `seek_at` at `A`, three blocks after the swap; (c) case (b) with a
   true-peak limiter insert (`D > 0`). Each output equals a fresh plan of the successor session, fed
   the same source frames at the same blocks, with the strip muted and given a live unmute of `N`
   samples at the D3 fire block: bit-identical for every block.
5. **Browser form.** Gate 4(a) with both runs prepared between render calls
   (`FaderMatrixBankProcessor`).
6. **Realtime.** Extend `the_swap_block_allocates_and_frees_nothing` (`successor_swap.rs:476`) over
   the fade blocks of gate 4(c): zero allocations and frees.
7. Commands:
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
- Gate 2(b): a fire on a block rendered before adoption (the fade spent off-thread in a warm
  successor's catch-up, so the strip enters at full gain at `S`) turns it red.
- Gate 4(b): a fade that starts at the swap block for an anchored stem (finished before the stem
  plays, so the click remains) turns it red.
- Gate 5: arming only the split banks leaves the browser's fused form unfaded; it turns red.

## Dependencies

- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054). It merges first and supplies the non-zero session mute ramp `N` the fade runs over.
