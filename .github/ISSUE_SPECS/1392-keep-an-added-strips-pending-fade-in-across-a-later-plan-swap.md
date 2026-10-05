# Keep an added strip's pending fade-in across a later plan swap

Stream D of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-9).
Split from *Fade in a strip that a swap adds during playback* (#1288): its former D6 (carry) and
gate 5 (arm survives a second swap). Code anchors verified on `main` at `6fb211594`.

## Product outcome

A stem that a structural edit adds still fades in when a second structural edit is swapped in
before the fade has started, for example while the added source waits for its anchored start
(`miso_engine_v1_source_seek_at` at a later render sample). Today, after #1288, an arm lives only
in the plan that armed it, so the newer plan starts that strip unarmed and the stem enters at full
gain: a click. After this slice a waiting fade travels with the strip from plan to plan until it
fires, and a fade that already fired never runs a second time. The same holds for a route's
waiting fade-in (an added route, or a send from a tap before an armed fader, #1363). This slice
and #1288 merge to `main` in the same batch (#1288 Hazards).

## Context

- The arm itself is #1288's (a dependency):
  - `FaderLane` gains `armed: bool` and an armed channel is prepared muted and remembers its fader
    gain; `FaderRampStage` keeps `armed: [[bool; MAX]; 2]` and gains `fire_fade_in` (#1288 D2).
    On `main`: `FaderRampStage` at `crates/builtins/src/lib.rs:2653`, `FaderLane` at `:798`,
    `set_mute` at `:2774`, the per-node form `FaderMuteRampBuiltins` at `:4163`.
  - The prepared graph plan holds a fixed fade table, one entry per armed strip, with `state` one
    of `Waiting`, `FireAt(u64)`, `Done` (#1288 D3). The graph executor's `adopt_predecessor`, which
    this slice extends, is on `main` at `crates/graph/src/lib.rs:3139`.
  - Arming happens inside successor preparation through one entry point that takes a strip set
    (#1288 D1); the delay `D` is the strip's pre-fader latency, its chain's compensation lines included
    (#1288 D4).
  - #1288 D6 leaves an arm still waiting when its plan is succeeded to this issue.
- Route arms: *Ramp a route that a plan swap adds to or removes from a surviving strip* (#1363)
  gives an added route (its D3) and a send from a tap before an armed fader (its D1 (c)) an entry
  in the same fade table, keyed by the route and, for case (c), with its strip's claim. Its arm is
  route-op state (#1363 D2), and its ramp state carries as a live send ramp (*Carry strip delay
  lines and live send ramps across a plan swap*, #1284). A fading route (a removed one) is retired
  by #1363 D5, not by this issue.
- `FireAt(s)` holds an absolute render sample: `GraphExecutor::render` receives
  `time.absolute_sample` (`crates/graph/src/lib.rs:3228-3256`), the host's render clock, which runs
  on across a swap. A carried `FireAt(s)` therefore means the same instant in the successor.
- Fader lane carry: *Carry fader, mute and pan ramps across a plan swap* (#1277) gives each fader
  stage a fixed-size, plain-data lane state with `export_lane` and `import_lane` (its D7), carries
  a stage only when no prepared value differs (its D4), and reports the strips it could not carry
  in `PreparedHost::restarted_strips()` (its D6).
- The adoption hook runs once per adoption on the render thread, before the plan's first adopted
  block: `carry_from` (`crates/engine/src/realtime/plan_exchange.rs:421`, `plan.rs:917`), or
  `PreparedRenderPlan::adopt_predecessor_plan` (`crates/engine/src/realtime/plan.rs:899`) for a
  synchronous host.
- Successor preparation: `SuccessorBase { inventory, committed }`
  (`crates/host-core/src/prepare.rs:641-647`); `PlanStateInventory` (`:558-564`) holds one row
  list per state family.

## Decisions frozen for this slice

- **D1. Inventory.** `PlanStateInventory` gains two sorted row lists: the strip IDs and the route
  IDs whose fade entry the plan's table holds (fading routes excluded). It is built at
  preparation; it says "this plan armed the strip", not "the fade is still waiting", which only
  render knows.
- **D2. Re-arm on the control thread.** Successor preparation arms, through #1288's arm entry
  point, every strip that is in the base's inventory rows (D1), is present in the successor's
  model, and is not in `restarted_strips()`. A fresh entry starts `Waiting` with the successor's
  `D` and ramp. Channels the successor's model mutes are not armed (#1288 D1). Likewise it arms,
  through #1363's route entry, every route in the base's route rows that the successor keeps with
  the same source strip, tap and destination, whose source strip is not in `restarted_strips()`,
  and that #1363 D1 gives no entry of its own in this swap; a case (c) route keeps its strip's
  claim, and is re-armed only with its strip.
- **D3. The render state wins at adoption.** At adoption, after #1277's lane import and #1284's
  route ramp import, the graph executor's `adopt_predecessor` copies, for each entry armed by D2,
  the predecessor's entry state for the same strip ID or route ID: `Waiting` stays `Waiting`,
  `FireAt(s)` becomes `FireAt(s)`, and `Done` becomes `Done` (and decrements the count of entries not `Done`). The lane state #1277 imports
  carries the armed bits and the remembered fader gain, so a lane that already fired imports
  `armed = false`; a route op imports its armed flag with its ramp the same way (D6). A fired
  strip or route therefore never fades twice, and a waiting one keeps waiting with its
  predecessor's progress.
- **D4. Delay stays valid.** A carried entry keeps the predecessor's `FireAt(s)`, which used the
  predecessor's `D`. That `D` is still right: a strip outside `restarted_strips()` carried every
  owner before its fader (*Duck-swap a strip whose state cannot continue across a plan swap*,
  #1324 D2, restarts the whole pre-fader chain otherwise), so its pre-fader latency is unchanged.
  A route entry's `FireAt(s)` likewise stays valid: its source strip carried, and its compensation
  line carries with its pending audio at the same head (*Carry compensation lines across a plan
  swap*, #1283 D4).
- **D5. Restarted strips are not carried.** A strip in `restarted_strips()` takes no carried
  entry. Its lane starts at rest, and #1324 decides its own arm and `D`.
- **D6. Lane state.** #1277's fader lane state (D7 there) gains the `armed` bits per channel. The
  remembered gain is the lane's fader gain, already in that state. `import_lane` restores the bits,
  in the split bank, the fused `FaderMatrixBankProcessor` and the per-node form. The route ramp
  state #1284 carries for a route op gains the op's armed flag (#1363 D2), restored on import, in
  `crates/graph/src/runtime.rs`.
- **D7. A warm successor takes the same carry.** A warm successor (*Grow latency during playback
  by adopting a primed warm successor*, #1287) is adopted in move mode at `S`, so D3 runs in its
  adoption block as at any swap. It renders no block before that (D15-8 (round-5 amendment)), so no
  carried entry can fire before adoption.
- **D8. Realtime.** No allocation: the D3 copy walks two tables sized at preparation, matched by a
  strip-index map built at preparation. With no carried entry, adoption pays one length test.
- **D9. Acked-batch question.** No queue or cell changes here; a structural edit's fallible steps
  are preparation's, all before publication.

## Deliverables

1. D1, D2 and D5 in host-core successor preparation (`crates/host-core/src/prepare.rs`).
2. D3 and D8 in `crates/graph/src/lib.rs` (the fade table carry in `adopt_predecessor`).
3. D6 in `crates/builtins/src/lib.rs` and `crates/builtins-compiler/src/lib.rs`.
4. The tests below. A sentence in `docs/C_ABI_V1_QUALIFICATION.md` beside #1288's fade text.

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`
- `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- The first fade, its fire rule and its delay (#1288).
- Edited strips and their duck-swap arm (#1324); removed strips (#1325).
- The first route arm, its delay and its fire (#1363); fading routes (#1363 D5).
- No crossfade between plans (deferred by D15-9).

## Objective gates

In `crates/host-core/tests/successor_swap.rs`, quantum 128, `N = 2000`, at `Backend::Simd8`,
`Backend::Simd4` and prepared between render calls (the fused form), with every other strip muted.

1. **Arm survives a second swap (#1288's former gate 5).** #1288 gate 4(b) (an added source started
   by `seek_at` at `A`, three blocks after the swap), with a second structural transaction (it adds
   a muted track) swapped in before `A`. Every block equals #1288 gate 4(b)'s reference:
   bit-identical. The same with #1288 gate 4(c)'s true-peak limiter insert, the second swap landing
   after the source first plays and before `FireAt(s)` (the carried `FireAt` case).
2. **A fired fade does not repeat.** #1288 gate 4(a), with a second structural transaction swapped
   in after the fade has finished. Every block equals a run without the second transaction. A
   second case swaps mid-ramp: also equal (the ramp carries by #1277).
3. **Restarted strip is not carried.** The added strip's second transaction also adds an insert to
   it: the successor holds no carried entry for it, and `restarted_strips()` contains it.
4. **Route arm survives a second swap.** #1363 gate 1 (an added route A -> B with a compensation
   delay of 300 samples), with a second structural transaction (it adds a muted track) prepared
   between render calls and adopted at the block after `S`, before `S + 300`. Every block equals
   #1363 gate 1's reference: bit-identical. The same for #1363 gate 6 (an added track's
   `pre_fader` send) with T's source started by `seek_at` three blocks after the swap (#1288 gate
   4(b)'s shape) and the second swap adopted before that start.
5. **Realtime.** Extend `the_swap_block_allocates_and_frees_nothing` (`successor_swap.rs:476`) over
   gate 1's and gate 4's second swap blocks: zero allocations and frees.
6. Commands:
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

- Gate 1: a successor that does not re-arm from the inventory lets the stem pop in at full gain;
  one that re-arms but restarts a carried `FireAt` as `Waiting` fires `D` late. Either turns it red.
- Gate 2: a re-arm whose lane import does not clear `armed` (or whose entry is not marked `Done`)
  mutes a playing strip and fades it in again: a dip. It turns red.
- Gate 3: a carried entry on a restarted chain (the predecessor's `D` against a new latency) turns
  it red.
- Gate 4: a successor that re-arms strips but not routes lets the added route, or the added
  strip's pre-fader send, enter at full level at the second swap; one that carries the entry but
  not the route op's armed flag fires it twice. Either turns it red.
- Gate 5: a table carry that allocates on the render thread turns it red.

## Dependencies

- *Fade in a strip that a swap adds during playback* (#1288).
- *Carry fader, mute and pan ramps across a plan swap* (#1277) (lane export and import,
  `restarted_strips()`).
- *Ramp a route that a plan swap adds to or removes from a surviving strip* (#1363) (route
  entries).
- *Carry strip delay lines and live send ramps across a plan swap* (#1284) (route ramp import).
- *Carry compensation lines across a plan swap* (#1283) (route entry delay, D4).
