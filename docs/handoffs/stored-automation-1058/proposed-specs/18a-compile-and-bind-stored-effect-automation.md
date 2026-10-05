# Compile and bind stored effect parameter automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1 (A1.4, A1.6), A4, A5, A7 and A11, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R3.

## Product outcome

A saved session that automates a `Block` parameter of a span-driven effect (compressor, delay,
gate/expander, multiband compressor, soft clip, transient shaper, true-peak limiter), on an insert
or a console slot of a track or a submix, plays that automation on the C ABI. Preparation compiles
each automated cell into a program, gives the instance a span window, and binds the program to
the effect's stage with the node's arrival `a(n)`. The parameter follows the curve in node time:
it equals the curve at each 64-sample grid ramp's completion (128 for the delay time) and jumps at
the curve's discontinuities on the exact sample. A session with no stored effect automation
renders the bits it renders today. The seek step, the browser host and quantum independence are
draft 18b; the parametric EQ is draft 20. This slice and draft 18b land in batch R3, in one push,
with #1306, which sizes the span windows.

## Context

**The table and its validation.**
- An effect target is `rack` `inserts` or `console`, an `effect_id`, a `parameter_id` and a
  `channel` (`crates/session/src/model.rs:874-955`). The session crate checks that it names a
  declared `(parameter_id, channel)` (`crates/session/src/validate.rs:890-967`).
- #1335 refuses a target that is not `automatable` with `AutomationRate::Block`. Draft 03a adds the
  unit, domain and shape rules. Draft 01 adds one entry per lane and the hold rule.
- Nothing renders the table (`docs/SESSION_SCHEMA_V1.md:220-225`).

**The launch `Block` parameters** (`sdk/assets/miso-engine-v1-parameter-metadata.json`): 56 in
all, and 34 outside the EQ:

| Effect | `Block` parameters | Cells (`PerLane` x2) | Smoothing |
|---|---|---|---|
| compressor | 7 | 14 | linear, 64 |
| delay | 5 | 9 | linear, 64; the delay time 128 |
| gate/expander | 4 | 8 | linear, 64 |
| multiband compressor | 10 | 20 | linear, 64 |
| soft clip | 3 | 6 | linear, 64 |
| transient shaper | 3 | 6 | linear, 64 |
| true-peak limiter | 2 | 4 | linear, 64 |

**The ramp law.** Every span-driven effect retargets a `LinearRamp` over its smoothing length
(`crates/compressor/src/design.rs:47`; `crates/delay/src/lib.rs:64`;
`crates/multiband-compressor/src/lib.rs:105`; `crates/true-peak-limiter/src/lib.rs:89`). The ramp
assigns the exact target on its last sample (`LinearRamp::next_value`,
`crates/effect-runtime/src/ramp.rs:140-155`), so a `Point` at sample `τ` reaches its target at
`τ + L - 1`: completion `End`. The delay time does not ramp; it starts a 128-sample crossfade
only when none runs (`begin_transition`, `crates/delay/src/lib.rs:504-509`; `TRANSITION_SAMPLES`,
`:65`).

**Windows and live lanes.**
- A per-node effect has a span window only with a live channel
  (`crates/graph/src/runtime.rs:1206-1239`); a bank slot likewise (`crates/rack/src/lib.rs:1014-1029`).
- The binder makes `NodeKind::Effect` when an instance has no control lane
  (`crates/graph/src/runtime.rs:4636-4648`) and `EffectBankStage` when no lane of a slot has one
  (`:4800-4848`).
- An `EffectPreparedEntry` carries its lane in `control` (`crates/effect-compiler/src/prepare.rs:28-70`),
  which `into_effects` turns into a `GraphEffectControlBinding`
  (`crates/graph-compiler/src/ids.rs:362-398`; `crates/graph/src/lib.rs:901-906`).
- The graph estimate charges the windows only for a live channel
  (`effect_control_resource`, `crates/graph-compiler/src/estimate.rs:188-345`).
- Host-core prepares the effects at `crates/host-core/src/prepare.rs:1344-1353` and attaches live
  lanes at `:1385-1393`.

**The browser's S.** It derives S from the stored segment count (`hosts/host-web/src/lib.rs:6574-6583`).
#1306 D3 deletes that derivation in this batch.

**Node arrivals.** PDC computes each node's arrival as the `max` over its inputs
(`crates/graph-compiler/src/pdc.rs:53-104`). #1285 D2 records each node's floored value; drafts 09a
and 09b give it to the fader stage.

## Decisions frozen for this slice

- **D1. Cells.** Host-core preparation compiles one cell per `(instance, parameter_index,
  channel)` the table drives (A7's `C(i)`): a `PerLane` parameter's `left` or `right` entry gives
  one cell, its `both` entry two (`Left`, `Right`, one curve); a `Shared` parameter gives one `Both`
  cell. An `inserts` target resolves by insert ID and a `console` target by slot ID on that strip,
  as #1335 D2 resolves them. A target-owning effect (one with `target_preparation`, the EQ) gets
  no span cell here (draft 20). A cell's stable address is `(strip ID, rack, effect ID,
  parameter_index, channel)`.
- **D2. The program.** Each cell takes draft 07's builder with:
  - grid period `G = max(64, L)` and grid ramp `L`, the descriptor's `smoothing_samples`: 64, and
    128 for the delay time;
  - jump ramp `J = L` (`docs/EFFECT_CONTRACT_V1.md:133-134` makes the descriptor's length binding);
  - completion `End`, because every span-driven ramp assigns its target on update `L` (Context).
    A parameter whose smoothing rule is `None` (none at HEAD; the gate's hold after #1336) has
    `L = 0` and completion at `τ`;
  - the "no restart" flag on the delay time, so a jump that falls inside a running crossfade moves
    to the crossfade's last sample plus one (A1.4).

  Each program's node is the effect node; the binder gives it that node's floored arrival `a(n)`
  (#1285 D2), as drafts 09a and 09b do for the fader node.
- **D3. The prepared value.** An automated cell's prepared value is the curve at node time
  `-a(n)` (A1.4). Every segment starts at timeline 0 or later, and the hold rule holds the first
  `start_value` before the first segment; at timeline 0 every shape starts at `start_value`. So
  that value is the entry's first `start_value` for every `a(n) >= 0`, and effect preparation
  writes it into the instance's initial values before it prepares the instance. It needs no `a(n)`,
  so preparation keeps one pass. A static `params` value of an automated cell is not rendered (A2).
- **D4. Windows** (#1306 D1, with the README's A7 term).
  - `capacity = live + stored`, where `stored(i) = |C(i)|` and `live` counts the `Block` cells
    the session does not automate (amendment to #1306 D1). #1306 owns the function; this slice
    passes it `C(i)`.
  - An instance with stored automation and no live lane gets a channel-less lane
    (`EffectControlLane::without_channel` with its `initial_bypass`) and a window, so draft 17b's
    stage renders it.
  - `Staged.dropped` stays 0: a piece stages at most one `Point` per cell (draft 17b D5).
- **D5. Plumbing.** `EffectPreparedEntry` gains `automation: Option<Box<EffectAutomationProgram>>`.
  `into_effects` passes it as a `GraphEffectAutomationBinding` beside the control binding. The graph
  binder hands it, with `a(n)`, to the stage draft 17b builds. Every plan that has no stored effect
  automation carries `None` everywhere and binds today's node kinds.
- **D6. Fresh cells.** A cell in a new plan, or in an instance a successor adds or restarts, starts
  with its prepared value as its current target. If the curve at its first block's node time
  differs, its first event is a jump at that block's first sample.
- **D7. Mono collapse.** This draft adds one preparation-time `DESIGNED` term, on the model of draft
  15 D4: an effect whose automation addresses one lane of a `PerLane` parameter, or both lanes with
  segment tables that are not bit-identical, has its prepared witness's `DESIGNED` term cleared
  (`ChannelSymmetryWitness::DESIGNED`, `crates/effect-contract/src/symmetry.rs:162`).
  `SessionPoolClasses` conjoins each prepared native effect's `DESIGNED` term into the track's class
  (`crates/builtins-compiler/src/lib.rs:4064-4067`), so the track declines its collapse. A `both`
  entry keeps it. Draft 09a adds nothing to `track_mono_source` (`:3967`), which stays the source
  rule. Draft 17b D7 is the render-side form of this rule.
- **D8. Memory.** Effect programs are charged through `effect_control_resource`
  (`crates/graph-compiler/src/estimate.rs:188`): each program's bytes (`80 + 32·n` per cell, A4) and
  each window go to the graph plan rows there, and the exact-charge oracles stay exact.

## Deliverables

1. D1-D3 and D6-D7 in `crates/host-core/src/prepare.rs`.
2. D4-D5: the entry field and window in `crates/effect-compiler/src/prepare.rs`, the binding in
   `crates/graph-compiler/src/ids.rs` and `crates/graph/src/lib.rs`, the binder in
   `crates/graph/src/runtime.rs`.
3. D8 in `crates/graph-compiler/src/estimate.rs`.
4. The tests below.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/` (new `effect_automation.rs`).
- `crates/effect-compiler/src/prepare.rs` (the entry field and the window of automated instances).
- `crates/graph-compiler/src/{ids.rs,estimate.rs}`, `crates/graph/src/lib.rs` (the binding type),
  `crates/graph/src/runtime.rs` (the binder only).
- `crates/rack/src/lib.rs` (only if draft 17b's constructor needs the binding's shape).
- `crates/builtins-compiler/src/lib.rs` (D7's `DESIGNED` term only).
- `crates/capi/src/runtime/` tests, `crates/capi/tests/resource_lifecycle.rs` (D8's oracle).

## Non-goals

- The piece loop (draft 17b). The seek step, the browser host, quantum independence and the docs
  (draft 18b). The EQ (draft 20).
- The classifier, the one rule for a live edit of an automated cell on both hosts, and carry across a swap
  (draft 19). Until draft 19, an effect automation edit stays model-only.
- The window sizing function itself (#1306).
- `AUTOMATION_ENQUEUE` (drafts 21a, 21c, 22). Sample-rate parameters (none at launch, #1335 D1).

## Hazards

- **Same batch.** This slice renders effect automation; draft 19 makes an edit of it reach a
  running plan, and draft 18b completes the seek and the browser. All merge in batch R3, so `main`
  never renders automation that an edit or a seek cannot reach.
- **#1306's live term.** If #1306 lands counting every `Block` cell as live, an instance with a
  live lane gets `live + stored` above its cell count: correct, but larger. Root applies the
  amendment row before #1306 lands.
- **Cohorts.** `automation_capacity` is in `EffectProgramKey`; #1306 D2's cohort maximum keeps an
  automated and a plain instance of one effect in one bank. Without it a cohort splits: cost, not
  bits, but the budgets move.
- **D3 rests on `τ(0) <= 0`.** A1.4 puts the first block's node time at `-a(n)`. If #1396's
  source-read offset is ever defined so that the first block's node time is positive, D3's value
  is wrong and preparation must evaluate the curve after PDC instead. Gates 1 and 2 catch it.
- **Hot files.** `crates/host-core/src/prepare.rs` and `crates/graph/src/runtime.rs` are shared
  with streams A and B. Root sequences the merge.

## Objective gates

1. **Flat equals static.** For each of the seven effects, a session with a flat automation at `v`
   on one `Block` parameter renders, through the C ABI, the bits of the same session with `params`
   value `v` and no automation, over 64 blocks (new test in `crates/capi/src/runtime/tests.rs`).
2. **Grid and completion** (new `crates/host-core/tests/effect_automation.rs`). A linear segment on
   a compressor threshold, a soft-clip drive and the delay time renders the bits of the same
   effect instance driven by hand, in 1-frame blocks, with `Point`s at the node-time grid samples
   whose values are the README formula (`x = (t - t0)/(t1 - t0)` in `f64`, rounded once),
   evaluated at `τ + L - 1`. With an upstream latent insert (the limiter), every `Point` moves by
   its latency exactly.
3. **Jumps.** A `step` segment at a sample that is not a grid sample stages its `Point` on that
   sample. On the delay time, a jump inside a running crossfade stages at the crossfade's last
   sample plus one.
4. **Banks equal scalar.** Gate 2's session with four tracks on one compressor cohort, two
   automated with different curves, renders each track at Simd4 and Simd8 as its scalar instance
   renders it (graph test with `--features test-support`).
5. **Windows and memory.** `Staged.dropped` and `shadowed_live_spans` are 0 in gates 1-4. An
   instance with stored automation and no live lane binds with a window of `stored(i)` spans.
   `cargo test --locked -p capi --test resource_lifecycle` passes with the program and windows
   charged exactly.
6. **Allocation.** 1,000 blocks of gate 2's session in a capi test make 0 allocations and 0 frees
   (`bench_support::alloc::current_thread_delta_since` after one warm block), and
   `cargo build --locked --release -p audit -p capi && target/release/audit capi` reports every
   violation count 0.
7. **No rendered bit moved (PR evidence).** `target/release/audit capi` shows the same `pcm_digest`
   at base and head; the browser legs of `qualification.yml`'s `browser` job pass with unchanged
   digests.
8. **Workspace.**
   - `cargo test --locked -p host-core --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p graph --features test-support`,
     `cargo test --locked -p effect-compiler --features test-support`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1 turns red if the prepared value is the static `params` value, or if a flat curve emits an
  event.
- Gate 2 turns red if the grid sits on the render clock instead of node time, if `a(n)` is
  missed, or if the event value is taken at `τ` or `τ + L` instead of `τ + L - 1`.
- Gate 3 turns red if a jump snaps to the grid, or if the delay's crossfade rule is ignored and
  the jump's value is lost.
- Gate 4 turns red if a lane's events reach a neighbour lane in a bank.
- Gate 5 turns red if an automated instance without a live lane gets no window, or if the charge
  misses the program or the window.
- Gate 6 turns red if compiling or binding leaves a render-time allocation.

## Dependencies

Batch R3. Direct dependencies:

- Draft 03a *Validate effect automation units, domains and shapes at preparation*.
- Draft 10 *Classify fader automation edits as carried rebuilds* (the per-row mask this batch
  extends).
- Draft 17b *Process an effect node in pieces at automation events* (same batch).
- *Size each effect's automation span window from the producers its plan has* (#1306), same batch.
- *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345).

Draft 07, drafts 09a and 09b (node time and `a(n)`), *Keep every node's latency from dropping
during playback* (#1285) and *Refuse automation on effect parameters that are not block-rate*
(#1335) arrive through drafts 03a, 10 and 17b. Draft 18b *Render stored effect parameter
automation across seeks and on both hosts* depends on this draft and lands in the same push.
