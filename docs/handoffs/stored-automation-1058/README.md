# Findings: render stored session automation in the engine, identically on every platform (#1058)

Issue: *Research: render stored session automation in the engine, identically on every platform*
(#1058), stream K of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-16). Attempt 1.

**Commit.** Every `path:line` in this note and in its drafts was read on `6ee64f484` (the worktree
head). That commit adds only the #1057 note to `main` at `8be19c86e`, so every code anchor is also an
anchor on `main` at `8be19c86e`. `main` has moved since (finding F15). The spec's own anchors were
written on an earlier `main`; each one was read again (section "Spec anchors, checked again").

**Authority.** The owner ruled that the core engine renders a producer's automation from the
session file, identically on every platform (`docs/rulings/engine-footprint-2026-09-28.md:52-53`).
Decision 15 binds every choice here to the no-shortcuts principle
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md:26-28`). A1 and
A3-A11 are design decisions for the implementing issues. A2 is an owner question with a
recommendation. A second owner question (send automation) is in "Owner questions". Points that
touch a decided item are in "Findings for the coordinator"; nothing here reopens them.

**Review.** A fresh adversarial verifier reviewed the first draft and returned FAIL (one blocker, six
majors). Every finding is folded in (section "Verification").

**No benchmark ran.** The one quoted time comes from a recorded run (section "Costs").

## The decisions in one line each

| Answer | Decision |
|---|---|
| A1 | The control plane compiles each automated cell into an immutable segment table in the plan. Render evaluates it in node time (the timeline delayed by the node's arrival), at a fixed 64-sample grid and at exact jump samples, and feeds the existing strip ramps, effect `Point` spans (in pieces) and filter targets. Hosts move the timeline with one session seek. |
| A2 | Owner question. Recommendation: an offset on level rows (fader, trim), OR on mute, automation wins elsewhere. Until the ruling, automation wins on every row. |
| A3 | The table is enough. It gains rules, not shapes: the hold rule, one entry per lane, the pan or matrix form, 64 samples between jumps, the target's unit and domain, shape limits, the filter order, and jump ramps from `control_smoothing`. |
| A4 | Memory per automated cell (one lane of one parameter) is `80 + 32·n` bytes, `n` its segment count, owned by the plan and independent of song length. CPU per block is at most `2⌈q/64⌉ + 3` events per cell times a per-row operation count. |
| A5 | The same session and the same host operations give the same bits on every target. Scalar `f64` evaluation through `crates/math` at events only, the existing unfused kernels and a grid in node time make it so; after a session seek the automation is independent of when the seek landed and of the quantum. |
| A6 | Every route into a submix whose source mute is automated gets a route lane with `follows_mute` as a cell word. At each automated mute event render calls `graph::gated_route_coefficients`, at the strip's sample and over the strip's ramp. |
| A7 | `stored(i)` is the number of distinct automated cells of instance `i` (two for `both` on a `PerLane` parameter); 0 for a target-owning effect (the EQ). At most the instance's `Block` cell count. |
| A8 | No. `AUTOMATION_ENQUEUE` never stages spans. #1306 drops its third term. |
| A9 | The C ABI never serves `AUTOMATION_ENQUEUE`. The refusal is permanent, and the command, its event and its queue leave the protocol registry (slices 21a and 21b, one push; slice 22 reserves the C limit). No ack can precede a drop. |
| A10 | The mask becomes per target row. Slice 10a removes the first row (fader) and slice 20 the last (EQ). An automation edit becomes a `rebuild` that carries every owner and completes `exact`; a static-value edit on an automated lane gives no record. |
| A11 | Thirty-seven slices in twenty-four steps (drafts in `proposed-specs/`). Slices 18a and 18b render stored effect automation; #1306 lands in their batch. |

## What the design starts from

- **No session timeline exists.** The plan has one clock, the render clock, which the host supplies
  and render keeps continuous (`crates/engine/src/realtime/plan.rs:496-501`, `:870-884`); a swap
  hands it to the incoming plan (`crates/engine/src/realtime/plan_exchange.rs:414-418`). Every seek
  names one source (`crates/source/src/lib.rs:51-78`, `:753-785`). Sources have no offset in the
  session (`crates/session/src/model.rs:165-176`; every region is `0..frames`,
  `crates/host-core/src/prepare.rs:1231-1253`). The protocol's transport position is stored and
  never read by render (`crates/host-core/src/control_provider.rs:424-438`). No loop feature exists.
  The browser's only "seek every stem" is a producer-side helper
  (`hosts/host-web/web/stem-store/pcm-pump.js:210-220`).
- **Nothing reads the table.** It is valid and inert (`docs/SESSION_SCHEMA_V1.md:220-225`;
  `crates/session/src/validate.rs:842-853`). The browser counts its segments only to size a window
  (`hosts/host-web/src/lib.rs:6574-6583`).
- **The quantum is the session's.** Any nonzero quantum is valid (`crates/session/src/validate.rs:48-53`).
  The browser asks the `AudioContext` for it (`sdk/src/browser/engine.ts:529-532`) and the worklet
  checks it (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:394`). Anchored seeks are quantum
  multiples (`crates/source/src/lib.rs:772-777`).
- **Strip ramps start at block entry.** Fader and mute, matrix and input records are drained before
  the block (`crates/builtins-compiler/src/lib.rs:1064-1104`, `:1108-1134`, `:460-506`) and
  `first_sample` is ignored (`:745`). The fader ramp is linear in linear gain, accumulated in `f32`,
  with an exact final assignment (`crates/builtins/src/lib.rs:2730-2750`;
  `crates/lane/src/kernels/builtins.rs:180-202`), and a muted lane's fader change restarts its
  ramp toward 0 (`crates/builtins/src/lib.rs:2756-2771`).
- **Effect spans are Points at the block's first sample.** A `Block` parameter accepts only a `Point`
  at `first_sample` (`crates/effect-contract/src/lib.rs:1496-1499`;
  `docs/EFFECT_CONTRACT_V1.md:149-155`), and the effect ramps over its descriptor's
  `smoothing_samples` (`crates/effect-runtime/src/ramp.rs:3-13`).
- **Effects are partition-invariant without spans.** Conformance renders each effect in blocks of 1,
  `q - 1` and `q` frames and fails `process.partition_invariance` if any bit differs
  (`crates/conformance/src/effect.rs:848-900`); it passes no spans (`:801-818`). A block may be
  shorter than the quantum (`crates/effect-contract/src/lib.rs:1472-1477`).
- **Filter targets are designed off render today.** The EQ refuses raw spans
  (`crates/parametric-eq/src/lib.rs:3513-3520`); targets come from the control owner
  (`docs/EFFECT_CONTRACT_V1.md:157-168`). The input HPF and LPF use the same scheme, one pair per
  design (`crates/builtins/src/filter_control.rs:9`, `:53-73`).

## A1. Design

### A1.1 Where curves compile

- Preparation (control plane, both hosts, `host-core` preparation) compiles the committed model's
  `automation` table into one **automation program** per plan. The program holds, per automated
  **cell** (one lane of one parameter: a strip row's `left` or `right`, a shared matrix row, or one
  effect `(parameter_index, channel)`), an immutable segment table and the cell's event state.
- A segment record is 32 bytes: `start` and `end` (`u64` timeline samples), `start_value` and
  `end_value` (`f32`), and the shape. Preparation validates every rule of A3 before it allocates.
- Preparation evaluates curves and designs under the canonical floating-point environment, as render
  does (`crates/lane/src/fpenv.rs:19-27`), so a prepared value has render's bits on every host.
- The program lives in the plan like the graph's other prepared tables. Builtin cells are charged with
  the builtin bank payload (`checked_add_builtin_banks`, `crates/graph/src/lib.rs:654-687`), and effect
  cells through the effect control resource (`effect_control_resource`,
  `crates/graph-compiler/src/estimate.rs:188`); both reach
  the graph's plan rows (`graph_session_plus_plan_bytes`, `graph_incremental_plan_bytes`;
  `crates/capi/include/miso_engine_v1.h:254-255`) and the C ABI's replacement peak, so the caller's
  existing cap `maximum_graph_session_plus_plan_bytes` (`:174`) and the browser boot budget bound it.
  No new ABI row is needed.
- The evaluator is a new leaf crate, `crates/automation` (lib `automation`), that depends on `math`
  only. Builtin processors, the rack's bank stage and the graph runtime call it; the rack's
  dependency policy (`scripts/check-rack-policy.sh:23`) gains it. Render never parses, allocates or
  frees for it.

### A1.2 The timeline clock: time base, seek and loop

- **Definition.** A segment's `start_sample` is a **timeline** sample. Timeline sample `t` is the
  moment source frame `t` enters the graph when the sources play in step. This follows from the
  schema: sources have no offset, so the session's own time is the shared source frame index.
- **The timeline consumer.** Every plan carries one timeline consumer in its source set. It is a
  source consumer with no PCM: a generation, a one-slot command queue and a held anchored seek, with
  exactly the rules of a source, through one shared seek state machine
  (`crates/source/src/lib.rs:1323-1365`). It advances one quantum per block on every block, as a
  source does on underrun (`:1154-1168`), and starts at generation 1, timeline 0, as a source starts
  at frame 0. It keeps the timeline value at the start of each of its last `K` blocks, where
  `K = ⌈A_max/q⌉ + 1` and `A_max` is the plan's largest node arrival (A1.3).
- **It is carried like a source.** Across a swap (#1273), at a declared discontinuity (#1323), at a
  prime adoption (#1320, #1402: it advances by the prime like a source) and on the source-read clock
  (#1396), its history included. Each of those specs gains one line (amendment table).
- **Session seek.** A host moves the playhead with one function both hosts call, in `host-core`'s
  `SourceControlSet` (#1309 moves only the C ABI wrapper): generation `g`, timeline sample `T`, and an optional anchor `A` on the source-read clock.
  It seeks the timeline and every declared source to `T` (each source clamped to its region) with
  generation `g`, all or nothing: it checks every consumer's command slot before it pushes any. Only
  the control thread pushes and render only pops, so room cannot shrink between the check and the
  push. The C ABI exposes it as a new call with a feature bit (D15-12's growth rule); the browser as
  an export, which the SDK's new `seek` calls before it refills each ring with generation `g`.
  #1323 D1 still asks the host to declare a discontinuity before a seek of every source.
- **Per-source seeks stay**, for a stem a transaction adds (`crates/capi/include/miso_engine_v1.h:96-108`).
  They never move the timeline. The timeline's seek lands in #1316's seek report as a row of its own.
- **Loop.** The engine adds no loop mechanism. A host loops by an anchored session seek at the end of
  each pass, as it would for sources.

### A1.3 Node time

- A node `n` processes, at render sample `r`, the audio the sources read at source-read sample
  `s = r + ΣP - a(n)`, where `ΣP` is the plan's source-read offset (#1396) and `a(n)` is the node's
  input arrival on the compensated graph (PDC's `max`, `crates/graph-compiler/src/pdc.rs:53-104`;
  the floored value #1285 D2 records). Its **node time** is `τ(r) = timeline(s)`, read from the
  timeline consumer's history: the timeline is linear inside each source-read block, so `timeline(s)`
  is the block's start value plus the offset of `s` in it.
- So automation lines up with the audio it acts on at every node, latency included, and a seek
  reaches a node exactly when the seek's audio does. `delay_samples` is not latency and stays out of
  `a(n)` (`crates/graph-compiler/src/pdc.rs:11-22`): automation follows the output's timeline, as PDC
  does.
- Across a warm adoption a carried node keeps its node time: its floor grows by `P` and the
  source-read clock by `P`, so `s` and `τ` are unchanged (D15-8 lemma L4 holds for automation too).

### A1.4 Evaluation law

- **Value of a curve** (A3 gives the hold rule): on a `linear` segment `v = v0 + (v1 - v0)·x`; on an
  `exponential` segment `v = v0·(v1/v0)^x`; on a `step` segment `v = v0`; with
  `x = (t - t0)/(t1 - t0)`. The arithmetic is scalar `f64` through `crates/math`, rounded once to
  `f32`. These are the Web Audio ramp formulas [S1].
- **Grid events.** At every render sample whose node time is a multiple of `G` (`G = 64`, or the
  cell's ramp length when that is larger: 128 for the delay time), a moving cell retargets once, over
  `L` samples, to the curve value at the sample where its lane first outputs the target exactly
  (`τ + L - 1` for the fader, matrix, trim and effect ramps; `τ + L` for the EQ and input-filter
  coefficient ramps, which reach the target at `A + 64`, `docs/EFFECT_CONTRACT_V1.md:165-166`). `L`
  is 64 for strip rows and filter targets and the descriptor's `smoothing_samples` for an effect
  parameter. So the lane equals the curve at each ramp's completion sample and is linear between:
  the piecewise-linear model VST 3 uses for automation points [S2].
- **Why 64.** It is the fixed coefficient ramp of the EQ and the input filters
  (`crates/builtins/src/filter_control.rs:9`; `crates/parametric-eq/src/lib.rs:116-119`) and the
  smoothing length of 55 of the 56 launch `Block` parameters; the delay time declares 128
  (`sdk/assets/miso-engine-v1-parameter-metadata.json`; `crates/compressor/src/design.rs:47`;
  `crates/delay/src/lib.rs:65`). So every ramp ends exactly when the next one starts. A grid in node
  time does not depend on the quantum, on when a seek landed, or on a warm adoption.
- **Jump events.** A discontinuity of the curve (A3 defines it) retargets at its exact sample over
  the row's jump length (A3), to the curve value at that ramp's completion sample.
- **Held events.** An event whose ramp would end before a ramp already in flight on the same stage
  lane (a jump, a mute ramp, a live record's ramp) is held: the lane's value is remembered, and the
  cell's grid resumes at its first grid sample after that ramp completes. A later jump restarts from
  the current value (D11, `docs/EFFECT_CONTRACT_V1.md:136-143`). A target that cannot restart (the
  delay's time crossfade starts only when none runs, `crates/delay/src/lib.rs:504-509`) takes a jump
  that falls inside its crossfade at the crossfade's last sample plus one.
- **Session seek.** Where a seek reaches a node (its node time steps, at any offset of the node's
  block, the first sample included), each builtin lane of that
  node is set exactly to the curve value, with no ramp, and each effect cell stages a `Point` there
  (the effect ramps over its smoothing). The node's audio is discontinuous at that same sample, and a
  seek of every source is a declared discontinuity (D15-8), so no continuity is owed.
- **Change only.** A cell emits an event only when the new target differs, bit for bit, from its
  current target: a redundant retarget moves bits (`crates/host-core/src/solo.rs:58-67`).
- **New lanes.** Preparation sets every automated lane to the curve value at node time `-a(n)`,
  never to the static value. By the hold rule that is always the entry's first `start_value`, since
  every `start_sample` is at least 0. A strip that a swap adds or restarts (D15-9) sets its lanes
  exactly at its first block; it fades in from silence. A carried cell whose curve an edit changed
  takes a jump at the adoption block's first sample (A1.8).

### A1.5 Feeding the strip ramps

- Each strip stage (input, fader and mute, matrix and pan) splits its kernel call at the event offsets
  of its bank's lanes and applies each event between the pieces. The kernels are the existing ones;
  they advance each lane by its own additions, so a split moves no bit for the same events
  (`crates/lane/src/kernels/builtins.rs:180-183`).
- **Fader** (id 5): the event value `v` (dB) becomes `vca_effective_db(v, offsets)`
  (`crates/session/src/vca.rs:19-34`), then the fader's own `db_gain`
  (`crates/builtins/src/lib.rs:5356-5358`). `vca_effective_db` adds the member first, then each
  offset in order, so an automated lane keeps the offsets of every VCA that reaches it, in the order
  `effective_strip_faders` adds them (`crates/session/src/vca.rs:96-120`), in a multi-word cell the
  control plane writes on a VCA move. A flat curve at `v` renders the bits of a plan prepared with
  static `v`, whatever the number of VCAs.
- **Mute** (id 6): the lane's stage mute is `curve || vca_mute || solo_mute`; the last two are
  control-plane terms in the strip's mute cell (`crates/host-core/src/solo.rs:255-263` composes them
  today). A mute event ramps over the session mute length, as a live mute does
  (`crates/builtins/src/lib.rs:2774-2794`).
- **Fader events on a muted lane.** While a lane is muted, or a mute ramp is in flight on it, a fader
  event updates only the remembered gain; it never restarts the ramp toward 0, which
  `set_fader_gain` does today (`crates/builtins/src/lib.rs:2756-2771`). Unmute retargets to the
  remembered gain, and the cell's grid resumes after the unmute ramp (held events).
- **Pan and matrix** (ids 12, 7-10): a pan event value goes through `pan_matrix`
  (`crates/builtins/src/lib.rs:4363-4378`), so pan is equal-power at every completion sample; matrix
  events retarget through `set_target_over` (`:3008-3049`).
- **Trim and polarity** (ids 2, 1): the signed trim coefficient (`crates/builtins/src/lib.rs:1434-1519`).
- **HPF and LPF** (ids 3, 4): at each event, render designs the target with the designer the control
  plane runs (`SvfSection::design`, `crates/builtins/src/lib.rs:722-760`), validates it with the same
  check (`crates/builtins/src/filter_control.rs:76-137`), and applies it through
  `apply_prepared_filter` (`crates/builtins/src/lib.rs:1352-1397`). A1.6 says why the design runs in
  render.
- **Target groups.** Render builds some targets from several cells: a strip's matrix stage (pan
  `left` and `right`, or the four matrix coefficients), one lane's input filter pair (HPF and LPF), and
  one EQ section on one channel. When any cell of a group is automated, render owns the group's
  target: the control plane writes a live edit of the group's other cells as a semantic value (pan
  position, cutoff, band value) into the group's cell, and render designs the target from the curve
  and that value, at each event and at the next block entry after the cell changes (a jump). So a
  live edit of one cell can never overwrite an automated one. A6 does the same for routes.
- **Slots on every plan.** Stored automation drives a stage directly, with no queue, so it needs no
  live lane. Only a group cell or a route lane (A6) is attached for it, on both hosts, whatever the
  live-control options.
- **Mono collapse.** Fader, mute, pan and matrix are seam-side by design: a collapsed track
  duplicates its one plane into them, so their per-channel values never gate the collapse
  (`crates/builtins-compiler/src/lib.rs:391-398`). Their automation, symmetric or not, keeps it. The
  input rows (1-4) and effect parameters are upstream of the seam: an entry that addresses one lane
  of such a `PerLane` row, or two entries with different curves, makes the track asymmetric at
  preparation, so preparation declines its collapse, as it does for unequal delays
  (`docs/SESSION_SCHEMA_V1.md:236-237`); a `both` entry keeps it. (A live one-channel effect record
  retires the collapse at render instead, `crates/effect-contract/src/live.rs:320-328`.)

### A1.6 Feeding effect spans and filter targets

- **Pieces.** An effect node (per node or bank) with an event in the block is processed in pieces that
  end at each event sample. At the start of each piece, each cell with an event there stages one
  `Point` at that piece's first sample, the only shape the contract accepts
  (`crates/effect-contract/src/lib.rs:1496-1499`). No effect changes if every launch effect is
  partition-invariant with spans; conformance checks it only without spans, so slice 17a gates it at
  widths 1, 4 and 8 against the scalar oracle, and depends on #1069, an open bank defect of exactly
  this kind. The delay's crossfade starts only at chunk starts (`crates/delay/src/lib.rs:1276-1285`),
  and a piece start is a chunk start.
- **Live cells.** Non-automated cells are drained at block entry as today, into the first piece. An
  automated cell takes no live record (A10), so no piece holds two spans for one cell.
- **Target-owning effects (the EQ).** At each event, render designs the section's target with the
  EQ's own designer (`design_svf_words_f64`, `crates/parametric-eq/src/lib.rs:747-787`: one `pow`, one
  `tan`, one or two `sqrt` in `f64`), validates it as preparation does, and starts the existing
  64-sample coefficient ramp (`:1441-1462`). Its other band values come from the group cell.
- **Why designs run in render.** The other choices fail a gate. Designs prepared for every grid sample
  grow with the length of the automated motion (one 48-byte target per section, channel and 64
  samples is about 36 KB per second of motion). Designs streamed ahead by the control plane make the
  audio depend on when the host calls the service step (D15-17: the engine owns no thread), so two
  hosts could render different bits. A design is bounded, allocates nothing and uses the same
  deterministic code as the control plane. A3's domain and order rules make a refused design
  unreachable; render still counts one. This amends a contract line (finding F1).

### A1.7 Realtime properties

- **Allocation, locks, syscalls:** none. The program and the timeline history are allocated at
  preparation; render reads the tables and writes only fixed event and ramp state. No queue exists
  between the control plane and the evaluator.
- **Bounded work.** Per cell and block: at most `⌈q/G⌉ + 1` grid events, `⌈q/64⌉ + 1` jumps (A3's
  spacing rule) and one seek: at most `2⌈q/64⌉ + 3` events. A cursor advance per segment crossed (at
  most `q/64 + 2`, by the spacing rule, plus continuous joints the grid passes). A seek repositions a
  cursor by binary search over its own table (`⌈log2(n + 1)⌉` compares). A bank stage runs at most one
  piece per distinct event offset of its lanes. Nothing depends on song length.
- **Scalar evaluation.** Events are sparse control arithmetic, not per-sample work: at most
  `2⌈q/64⌉ + 3` per cell and block, at offsets that differ between a bank's lanes. Evaluating them as
  vectors would need lane math, which diverges on AArch64 release builds (LANE-3,
  `docs/TARGET_MATRIX.md:163-167`). So event values are scalar `f64`; every per-sample operation stays
  in the vector kernels. This is the reason the no-scalar rule asks a brief to state.
- **Silence.** A node with an event in the block takes its normal path, as an in-flight live ramp
  already vetoes a bank's silent skip (`crates/compressor/src/lib.rs:568`;
  `crates/parametric-eq/src/lib.rs:2936-2939`). A flat curve emits no event and blocks no skip. A
  moving curve over silence therefore keeps its chain processing: finding F10. The #1053 D11 hazard
  applies: a future silence skip must still apply events.

### A1.8 Swaps and edits

- An automation edit is a rebuild (A10). The successor's program is compiled from the new table.
- **Carry.** A cell that exists in both plans carries its whole event state (current target bits, the
  end sample of a ramp in flight, a held grid, the last seek boundary) by its stable address, and
  repositions its cursor in the new table at adoption. If the new curve's value at the adoption block
  differs from the carried target, the cell takes a jump at that block's first sample. A cell whose
  entry the edit removes ramps to its static value over its jump length (D15-7: carry, then
  retarget). The stage state itself carries under #1277 and #1279-#1282.
- **The program is not a control kind.** Gaining, losing or changing stored automation, and the span
  window capacity it changes (A7), are neither a prepared value nor a change of live-control kind, so
  the carry slices carry the stage (amendments to #1277 D4, #1279 D1-D2 and #1280 D1).

## A2. Live changes against automation

**Decision: owner question OQ1** (section "Owner questions"). Recommendation: an offset on level rows
(fader dB, input trim dB) and an OR on mute, and automation wins on every other row.

What ships before the ruling, and stays under every option:

- The static value of an automated lane is not rendered while its automation exists. It is the value
  the lane takes when the entry is removed (A3).
- A transaction that changes only that static value gives no record and commits as `model_only`;
  the response reports the path (D15-17). It is a document value no plan reads, the same category
  #1315 examined and kept for the session ID and the stored table. Finding F9 asks root to confirm
  this reading against #1315's definition; the alternative is a typed refusal, which would also
  refuse a document replacement (D15-11 `replaceSession`) that changes such a fallback value.
- The browser's 48-byte record admission has no model to keep a value in, so until #1382 replaces it
  (D15-11) it refuses a live command on an automated lane with a typed reason and admits nothing from
  that batch, as #1315 D4 refuses a prepared-bypass lift (`admit_commands`,
  `hosts/host-web/src/lib.rs:4596`). A VCA command on an automated member writes the lane's offset
  term (A1.5).
- This is option A of OQ1. Options B and C add a stored field and an edit; they never change the
  rules above. So no shipped behaviour is undone by any answer.

Evidence for the options (vendor documentation shows interface behaviour only):

- Logic Pro's Read mode does not let a moved control change an automated parameter; Touch returns to
  the curve on release; Latch keeps the new value; Trim offsets the curve for volume, pan and sends
  [S3].
- Pro Tools' Trim writes relative values for volume and send level only, and Touch returns to the
  curve at the AutoMatch rate [S4].
- Ableton Live overrides automation when a control moves outside recording, until "Re-Enable
  Automation" [S5].
- Cubase's Trim offsets volume and cue sends; Touch returns over the Return Time [S6].
- Web Audio's `value` setter is `setValueAtTime(value, currentTime)`, so a later event takes over
  again [S1]. VST 3 sends GUI changes and automation on one path and leaves recording modes to the
  host [S2].

## A3. Format

**Decision: today's table is enough. It gains rules, not new shapes or fields.**

- **Kept.** One entry per target with ordered, non-overlapping segments; `step`, `linear`,
  `exponential`; `u64` sample times at the session rate; `f32` values with an explicit unit
  (`crates/session/src/model.rs:874-955`; `crates/session/src/validate.rs:890-1030`). Sample-exact
  times and three shapes express breakpoint automation: VST 3 reduces every host curve to linear
  pieces [S2], and the exponential shape is Web Audio's [S1]. No other shape is added.
- **The hold rule** (new, documented and tested): before the first segment the lane holds the first
  `start_value`; inside a segment it follows the segment; after a segment, and in a gap, it holds that
  segment's `end_value`. A `step` segment holds `start_value` on `[start, end)`. This is the rule the
  protocol's transient record used ("holds its end value until replaced",
  `docs/CONTROL_PROTOCOL_REGISTRY.md:45`) and the DAW curve model.
- **Discontinuities.** A discontinuity is a sample where the hold rule's value jumps: a `step`
  segment's start or end where the value changes, or a segment start whose `start_value` differs
  from the value just before it. Two discontinuities of one entry are at least 64 samples apart
  (1.3 ms at 48 kHz), so a block holds a bounded number of jumps (A1.7). Faster gating is not a mix
  move; it belongs in an effect.
- **One entry per lane.** Two entries on one target, or a `both` entry and a `left` or `right` entry
  on one parameter, are refused. Today nothing refuses them (`crates/session/src/validate.rs:890-1030`).
- **Pan or matrix.** A `pan` target (id 12) is valid only on a strip whose `matrix_or_pan` is pan,
  and matrix targets (ids 7-10) only on a matrix strip: one authority per strip
  (`docs/EFFECT_CONTRACT_V1.md:125-127`).
- **Unit and domain.** A segment's unit equals the target's unit, and both values lie in the target's
  domain. The session crate cannot see a descriptor (it depends only on `engine` and `json-syntax`,
  `crates/session/Cargo.toml`), so the check runs where the descriptors are: builtin preparation for
  the strip rows (unit from `builtin_parameter_unit`, `crates/builtins/src/lib.rs:255`; fader and trim
  `[-144, 24]` dB, matrix and pan `[-1, 1]`, `:453-673`), and effect preparation for effect
  parameters (the function #1335 adds).
- **Shapes per row.** `exponential` only on a strictly positive domain whose unit is not `db`
  (frequency, time, ratio). Boolean rows (mute, polarity, an effect's boolean parameter) take only
  `step`, with values `0` or `1`. An input filter's value `0` means "off" and lies outside its enabled
  range (`DisabledOrRateKeyedHertz`, `crates/builtins/src/lib.rs:497-520`), so a `linear` or
  `exponential` segment on it stays inside the enabled range, and "off" is reached only by a step.
- **Filter order.** Where a lane's HPF and LPF are both enabled, `HPF < LPF` holds at every sample
  (the pair rule, `crates/builtins/src/filter_control.rs:43-45`). Preparation checks it on each common
  sub-interval of the two curves: equal shapes need only the order at both ends; mixed shapes need the
  HPF's largest value below the LPF's smallest (each piece is monotone, so the check is sufficient,
  not necessary). Where either side moves, the order keeps a relative margin of `2^-22`, so `f32`
  rounding between checked samples cannot make the two equal. A static value is a constant curve, so
  the same check refuses a live edit that would cross.
- **Resolution.** Timing is sample-exact for jumps; continuous motion renders on the 64-sample grid
  of A1.4. Values are `f32`, as every live value is.
- **`control_smoothing` (#1054).** A jump ramps over the row's key in #1054 D3's table: `fader_ms` for
  fader and trim, `mute_ms` for mute and polarity, `pan_ms` for pan and matrix. HPF, LPF and EQ targets
  use their fixed 64-sample ramp; other effect parameters their descriptor's `smoothing_samples`
  (binding, `docs/EFFECT_CONTRACT_V1.md:133-134`). A session `explicit` 0 makes jumps hard steps,
  which bit-comparison tests use. No segment carries its own ramp length: one documented table is
  owner decision 1 (`docs/rulings/engine-footprint-2026-09-28.md:44-48`).
- **Targets.** Unchanged: the eleven builtin rows (`crates/session/src/validate.rs:823-840`) and
  `Block` effect parameters (#1335). A submix target is already valid; the SDK builder gains
  `{ submixId }` (the #1196 known gap). VCA automation stays out of scope
  (`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md:129`). Send automation is OQ2.

## A4. Cost

**Decision: compiled tables owned by the plan, not streamed.** Memory per automated cell is
`80 + 32·n` bytes (event state 48, program header 32, 32 per segment), where `n` is the cell's authored segment count. Song length adds nothing: a
ten-minute session with one fade has one segment. The committed model already holds the same segments
(decision 2: the core owns the session), and the document cap bounds them: the smallest segment in
JSON is 99 bytes, so the tables are at most `32/99` of the document (about 339 KB at the browser's
1 MiB cap, `hosts/host-web/src/lib.rs:66`). Streaming was rejected: it makes the audio depend on the
host's control cadence (D15-17) and a seek would wait for a refill. CPU per block is a bounded sum of
events times per-row operation counts. Formulas, the operation table and the one measured cost are in
"Costs".

## A5. Bit-identity

**Decision: the same session and the same host operations (boot, PCM, seeks and edits at the same
render samples) give the same bits on the browser and the C ABI, on x86-64, AArch64 and wasm. After
a session seek, stored automation is also independent of when the seek landed and of the quantum.**

- **Same inputs.** The session declares the rate and the quantum (`crates/session/src/model.rs:96-99`),
  and both hosts render at them (A1 start). Event samples are pure functions of the segment table,
  the timeline and `a(n)`, all integers, and the grid is in node time.
- **Sample-time arithmetic.** `u64` timeline and render samples; `t - t0` and `t1 - t0` convert to
  `f64` exactly below 2^53; one `f64` division gives `x`.
- **Transcendentals.** Exponential segments use `math::pow` in `f64`; conversions use `db_gain`,
  `pan_matrix`, the filter designers and `vca_effective_db`. `crates/math` is vendored libm with every
  target-conditional path and intrinsic removed (`crates/math/src/lib.rs:13-16`), and `clippy.toml`
  bans platform transcendentals (`clippy.toml:15-23`). The evaluator calls no lane math, so the
  AArch64 release divergence in `exp2_lane` (LANE-3, #1019; `docs/TARGET_MATRIX.md:163-167`) cannot
  reach it.
- **No fused multiply-add.** `Lane::fma` is two roundings on every backend
  (`crates/lane/src/lib.rs:19-23`), sealed by `scripts/check-unfused-seal.sh`; Rust does not contract
  scalar `f64` code.
- **Floating-point environment.** Every native render entry installs the canonical MXCSR or FPCR
  (`crates/lane/src/fpenv.rs:19-27`), and A1.1 installs it around preparation's evaluations; wasm has
  none. Finding F11 covers control-thread designs that exist today.
- **Lanes and banks.** Ramps and kernels are the existing ones, identical at widths 1, 4 and 8 and
  across x86-64, AArch64 and wasm (`tools/wasm-gate-corpus/src/lib.rs:4-5`). Effect pieces rely on
  partition invariance with spans, which slice 17a gates. Banking moves no bit (decision 12).
- **Events, not samples.** Linear and exponential segments are both evaluated only at events, never
  per sample, so no per-sample transcendental exists to diverge.
- **After a seek.** Builtin lanes are set exactly at the seek's sample, so they are a function of the
  session and the seek target from that sample on. Effect parameters are after their smoothing length
  (64 or 128 samples), and effect state after the pre-seek audio's tail reaches exact rest (D15-4).
- **Gates.** Slice 07 adds the evaluator's event values to the G5 corpus, the single owner of the
  cross-target corpus (widths 1, 4 and 8; x86-64, wasm and AArch64). Slices 23a and 23b add a session with every
  row automated to the browser direct oracle
  (`hosts/host-web/tests/browser-v1/direct-oracle.mjs:571-583`) and an AArch64 equality leg, since no
  session digest is compared on AArch64 today, and a seek-time invariance case.

## A6. Automated mute and follows_mute sends

**Decision: every route into a submix whose source strip's mute is automated gets a route lane on
every plan, with `follows_mute` as a word of its cell. At each automated mute event of the source,
render composes the gate and calls the one route derivation, `graph::gated_route_coefficients`, at
the same sample and over the same ramp as the strip.**

- **The composition is the live path's.** `graph_compiler::route_coefficients` maps the source lanes'
  mutes to `follow_zeroed` and calls `graph::gated_route_coefficients`
  (`crates/graph-compiler/src/ids.rs:341-355`; `crates/graph/src/lib.rs:777-812`), which its own
  documentation names the one derivation for prepared and live routes. It is a `const fn` that
  allocates nothing, so render may call it.
- **What the lane holds.** The route's open transform (`route_values`,
  `crates/graph-compiler/src/ids.rs:309-322`), its own `mute`, its `follows_mute`, and per source lane
  the source's other mute terms: VCA mute, solo mute, and the static mute of a lane whose mute is not
  automated. The control plane writes them through the route's cell (#1347) whenever a route, VCA,
  solo, static mute or `follows_mute` edit changes them, so a live `follows_mute` toggle (D15-6) is a
  cell write.
- **At a mute event** of the source strip at sample `r` over `R` samples (the session mute ramp), and
  at the next block entry after the cell changes:
  `follow_zeroed[lane] = follows_mute && (curve[lane](r) || other_terms[lane])`; the target is
  `gated_route_coefficients(&transform, RouteGate { mute, follow_zeroed })`; the route's ramp starts at
  `r` over `R`. This is #1226 D5's rule (strip and sends ramp together) at a sample offset. Only a
  changed gate retargets (`crates/host-core/src/live_route_state.rs:232-257` yields only changed
  routes).
- **No leak.** A saved session with automated mute and pre-fader sends silences those sends exactly
  when the mute engages. Slice 13b's gate compares the output, after the ramp, with a plan prepared with
  the same mute as a static value, and toggles `follows_mute` both ways on a playing engine.
- **Out of scope by rule.** Every route into the output never follows (`follows_mute` is always false
  there, decision 13 P11).

## A7. Stored span bound, for #1306 D1

**Decision.** For one prepared effect instance `i`:

- `C(i)` is the set of distinct cells the automation table drives on `i`. For each entry whose
  target resolves to `i` (an `inserts` target by insert ID, a `console` target by slot on that
  strip): a `PerLane` parameter adds `{left}`, `{right}` or, for `both`, `{left, right}`; a `Shared`
  parameter adds `{both}`.
- `stored(i) = |C(i)|`, and `stored(i) = 0` when `i`'s effect owns prepared targets (the parametric
  EQ): its stored automation designs targets in render (A1.6), never spans.
- **Why one span per cell.** A1.6 processes the node in pieces at events, and a cell stages at most one
  `Point` per piece. So a window holds at most `|C(i)|` stored spans in any process call.
- **Inputs:** the session's `automation` table and the instance's descriptor (`channel_policy`).
  **Upper bound:** the instance's `Block` cell count (two per `PerLane` and one per `Shared` `Block`
  parameter); 14 for the compressor; 9 for the delay. It never reads a segment, so it does not grow
  with song length.
- It is a summand of #1306 D1's `capacity`. Because an automated cell takes no live record (A10),
  #1306's live term should count only cells the session does not automate; then `live + stored` is the
  instance's `Block` cell count on an instance with a live lane (amendment table).

## A8. AUTOMATION_ENQUEUE and effect windows, for #1306 D1

**Decision: no.** Accepted `AUTOMATION_ENQUEUE` batches never stage spans into effect windows,
because the C ABI never serves the command (A9). #1306 D1 has no third term.

## A9. Serving AUTOMATION_ENQUEUE, for #1315 D3

**Decision: the C ABI does not serve `AUTOMATION_ENQUEUE`. The refusal is permanent, and the command
leaves the protocol registry.**

- **Why.** The owner ruled that sample-timed `AutomationEnqueue` delivery has no product consumer
  (`docs/rulings/engine-footprint-2026-09-28.md:58-59`), and that code nothing uses is removed
  (`:20-21`). Every sample-timed outcome a host wants is a stored automation edit, which renders by
  A1 and reaches every host through one edit API (D15-11). Two paths for one outcome would be the
  shortcut the owner principle forbids.
- **What leaves.** Command `0x0006` and event `0x8002` (`crates/protocol/src/wire.rs:110`, `:124`),
  status codes 15 and 16, the automation counters, queue kind 2, the capability fields (the
  admission quantum, `crates/protocol/src/message_wire.rs:92`, included) and flag bit 5, the
  provider hooks that serve only the command (`parameter_descriptor` for admission and
  `record_canceled_automation`), the 32-byte record codec and the queue
  (`crates/protocol/src/queue.rs:30-200`, `:773-812`). `AUTOMATION_BATCH_RECORDS` also sizes the
  meter and counter pages, so it is renamed and kept. Each
  retired code is refused and never reallocated, the wire-identity rule of decision 12
  (`docs/CONTROL_PROTOCOL_REGISTRY.md:89`); an unassigned message ID decodes as unsupported
  (`crates/protocol/src/wire.rs:142-165`). The C limits field `maximum_automation_spans_per_block`
  (`crates/capi/include/miso_engine_v1.h:166`) loses its last role once #1306 lands and becomes
  reserved, must be zero, layout unchanged.
- **Slices.** Slices 21a (wire identities and dispatch) and 21b (the queue, records and counters)
  land in one push, so `main` never holds a dead queue. They replace #1315 D1-D3, so no feature
  switch is added only to be deleted, and they land no later than #1315's C ABI half. Slice 22
  reserves the C field after #1306.
- **The acked-batch question: can an ack ever precede a drop?** No. The command is refused at decode,
  before the revision check and before any state changes, and no queue remains to hold a batch.
  Stored automation itself has no queue: render generates its events from the plan, and each window
  is sized to the per-piece bound (A7), so `Staged.dropped` stays 0
  (`crates/graph/src/runtime.rs:3522-3525`).

## A10. The classifier mask, for #1260

**Decision: the mask becomes per target row** (`crates/host-core/src/live_delta.rs:181-185`,
`:234-236`). Slice 10a, *Classify fader automation edits as carried rebuilds*, removes the first row
(fader) in the same batch as slices 09a and 09b, which render it. Each later rendering slice removes its own
row, and slice 20 (the EQ) removes the last. A row that does not render yet stays model-only and
inert, as today. For a row that renders:

- **An automation edit is a `rebuild`.** The successor needs a new compiled program: decision 14
  rule 1 (new memory). No prepared value of any strip changes and the program is not a control kind
  (A1.8), so every owner is carried and no strip takes a transition (D15-7, D15-9). The revision
  completes `exact` at adoption (D15-17). A1.8 gives the jumps at adoption.
- **A static-value edit on an automated lane gives no record** (A2): alone it is `model_only`.
  A transaction that also edits automation is a rebuild.
- **A group cell edit** (A1.5) and a live `follows_mute` toggle on a following route (A6) are `live`.
  Before a group's cell exists (slices 14b and 16b add them), a static edit of a group's other cell is
  a carried rebuild: decision 14 rule 2 makes a value live only where the plan has a slot.
- **A `control_smoothing` edit** reaches the program's jump lengths through one plan-level cell
  (slice 12), so on a session with stored automation it is `live` (amendment to #1365).
- A refusable target never commits through a masked row: #1335 D4 routes an automation edit whose
  effect diagnostics are non-empty to the rebuild path, whose preparation refuses it, and slice 02
  adds the builtin rules to the same route. #1335 lands before slice 09a (D15-13 E1). Once slice 20
  removes the last row, every automation edit is a rebuild and slice 20 deletes the route.

## A11. The staged plan

**Decision: thirty-seven slices in twenty-four numbered steps (steps that would not fit half a working
day are split into lettered parts), each a stateless draft in `proposed-specs/`, smallest first
inside each batch. Slices 18a and 18b, *Compile and bind stored effect parameter automation* and
*Render stored effect parameter automation across seeks and on both hosts*, render stored effect
automation; #1306 lands in their batch.** The table, batches and dependencies are in "Staged plan".

## Costs

### Memory per automated cell

| Item | Bytes | Count |
|---|---|---|
| Segment record (`start`, `end`: `u64`; `start_value`, `end_value`: `f32`; shape, padding) | 32 | per authored segment |
| Event state (cursor, current target, ramp end, held grid, seek boundary, lane address, flags) | 48 | per automated cell |
| Cell program header (segment slice, `G`, `L`, jump-length key, completion kind, flags) | 32 | per automated cell |
| Group cell (pan positions, a filter pair's cutoffs, or an EQ band's values) | 32 | per target group with an automated cell |
| VCA offsets cell | `12·v` plus the cell's fixed words (#1312's three slots of one `f32` per VCA) | per automated fader lane, `v` the VCAs that reach it (at most the session's VCA count: 256 in the browser, `maximum_vcas` on the C ABI) |
| Follow state of a route lane: #1347's cell of seven words (transform, route mute, `follows_mute`, two mute terms) in #1312's three slots | 84 plus the cell's fixed words | per route into a submix whose source mute is automated |
| Timeline history | `16·K`, `K = ⌈A_max/q⌉ + 1` | per plan |
| Effect window | #1306's per-span bank term (`8,944 + 360·S` per eight-lane bank, `4,560 + 200·S` per four-lane bank) | `S` includes A7 |

- **Formula.** `M = Σ_cells (80 + 32·n_c) + 32·groups + 12·Σ v + 84·follow_routes + 16·K + window
  bytes`, plus the fixed words of each cell. No
  term reads a duration. `n_c` is bounded by the document: `Σ n_c ≤ document_bytes / 99`.
- **Plans in flight.** Each plan owns its program. Up to three plans coexist during a supersession
  (D15-9), so the peak is three programs; the same caps that refuse a third plan refuse it (#1398).
  Shared programs were rejected: a shared reference count is machinery a later change must maintain,
  for at most about 1 MB in the browser.

### CPU per block

Per block of `q` frames:

- **Events.** At most `2⌈q/64⌉ + 3` per automated cell (A1.7); 7 at `q = 128`. A flat curve emits
  none and costs one compare.
- **Per event** (operation counts, scalar `f64` unless named):

| Row | Evaluate | Convert | Start the ramp |
|---|---|---|---|
| Any `linear` | 2 int-to-float, 1 div, 1 mul, 2 add | | |
| Any `exponential` | as linear, plus 1 div and 1 `math::pow` | | |
| Fader, trim | | 1 `math::pow` (`db_gain`), VCA sum and clamp | 1 sub, 1 div (`f32`) |
| Pan | | 2 `math::cos`, 2 `math::sin` (`pan_matrix`) | 4 sub, 4 div |
| Matrix | | none | 1 sub, 1 div |
| HPF, LPF | | 1 `math::tan` and design arithmetic; check: 2 `math::sqrt` (`crates/builtins/src/filter_control.rs:123-132`) | 6 sub, 6 mul |
| EQ section | | 1 `math::pow`, 1 `math::tan`, 1-2 `sqrt`; check: 2 `sqrt` | 6 sub, 6 mul |
| Effect parameter | | none | 1 span write (the effect divides once) |
| Following route | | `gated_route_coefficients`: 4 mul, 4 select | 4 sub, 4 mul |

- **Pieces.** A bank stage runs one kernel call per distinct event offset of its lanes: at most
  `1 + lanes·(2⌈q/64⌉ + 3)` calls, and at most `q`. Lanes that share a node arrival share grid
  offsets, so a bank of tracks with no latent insert before the stage takes at most `2⌈q/64⌉ + 3`
  extra calls.
- **Ramping paths.** Each moving cell keeps its bank on the ramping path. One recorded run measures
  that path: the `console_mixing_automation` row (`tools/bench/src/console.rs:138-158`, issue #1003;
  the record cites issue 149). In its latest record, eight controls in eight banks (EQ band gain,
  compressor threshold, limiter ceiling) ramp every block over 64 samples, and the paired ramp delta
  is 1,643 ns per block in round 1 and 1,823 ns in round 2: about 205-228 ns per ramping control and
  block (Simd8, 48 kHz, `q = 128`, AMD EPYC 7313P, uncontrolled; commit `244a52a0c`;
  `artifacts/steps/bus-send-base/console-benchmark.accepted.jsonl:31`, `:62`). That run pushed the
  edits outside the clock, so it does not include the EQ's design cost; the design stays in operation
  counts above.
- **Bound.** `CPU(block) ≤ Σ_events (evaluate + convert + start) + Σ_pieces (call) + Σ_moving banks
  (ramping-path delta)`. Each sum is bounded by `q`, the bank widths and the automated cell count;
  none reads the song length.
- **Measurement.** This issue runs no benchmark. Slices 24a and 24b add and record one descriptive row on the real console
  path (frozen workload, one warmup, two measured rounds).

## Staged plan

Each row lists the slice's direct dependencies; a draft's own Dependencies section says the same.

| # | Draft | Title | Depends on | Batch |
|---|---|---|---|---|
| 01 | `proposed-specs/01-validate-automation-lanes-in-the-session.md` | Validate stored automation lanes in the session crate and state the hold rule | — | R1 |
| 02 | `proposed-specs/02-validate-builtin-automation-targets-at-preparation.md` | Validate builtin automation targets against their rows at preparation | 01, #1335 | R1 |
| 03a | `proposed-specs/03a-validate-effect-automation-units-domains-and-shapes.md` | Validate effect automation units, domains and shapes at preparation | 01, 02, #1335 | R3 |
| 03b | `proposed-specs/03b-mirror-the-automation-rules-in-the-sdk-builder.md` | Mirror the stored automation rules in the SDK builder | 01, 02, 03a | R3 |
| 03c | `proposed-specs/03c-author-submix-automation-targets-in-the-sdk.md` | Author submix automation targets in the SDK and enginectl | — | R3 |
| 04 | `proposed-specs/04-give-every-plan-a-timeline-clock.md` | Give every plan a timeline clock that seeks and carries like a source | #1285, #1316, #1318 | R1 |
| 05 | `proposed-specs/05-seek-the-session-from-the-c-abi.md` | Seek the timeline and every source in one C ABI call | 04, #1323 | R1 |
| 06a | `proposed-specs/06a-seek-the-session-from-the-headless-engine.md` | Seek the timeline and every source from the browser module export and the headless SDK | 05 | R1 |
| 06b | `proposed-specs/06b-seek-the-session-from-the-browser-sdk.md` | Seek the timeline and every source from the browser SDK and the PCM feed | 06a | R1 |
| 07 | `proposed-specs/07-compile-stored-automation-into-per-cell-events.md` | Compile stored automation into per-cell events in node time | 01 | R1 |
| 08 | `proposed-specs/08-apply-timed-operations-inside-a-block-on-the-fader-stage.md` | Apply timed operations inside a block on the strip fader stage | — | R1 |
| 09a | `proposed-specs/09a-prepare-stored-fader-automation.md` | Prepare stored fader automation and render it flat | 01, 02, 07, 08, 12, #1054, #1285, #1309, #1312 (with its D1 amendment) | R1 |
| 09b | `proposed-specs/09b-render-moving-stored-fader-automation.md` | Render moving stored fader automation, seeks and latency | 09a (same push), 04, 05, 06a | R1 |
| 10a | `proposed-specs/10a-classify-and-carry-fader-automation-edits.md` | Classify fader automation edits as carried rebuilds | 09b, #1277, #1313, #1314, #1335 | R1 |
| 10b | `proposed-specs/10b-refuse-browser-live-commands-on-automated-lanes.md` | Refuse browser live commands on automated fader lanes | 10a, #1315; one push with 11 | R1 |
| 11 | `proposed-specs/11-compose-vca-offsets-with-stored-fader-automation.md` | Compose VCA offsets with stored fader automation | 09a, 10a, 12, #1312; one push with 10b | R1 |
| 12 | `proposed-specs/12-hold-the-automation-jump-lengths-in-a-cell.md` | Hold the automation jump lengths in a plan cell | #1054, #1312; #1365 for the edit path | R1 |
| 13a | `proposed-specs/13a-render-stored-mute-automation.md` | Render stored mute automation on the strip | 10a, 10b, 12 | R2 |
| 13b | `proposed-specs/13b-let-following-sends-follow-automated-mute.md` | Let following sends follow an automated mute | 13a (same push), 12, #1225, #1226, #1284, #1342, #1347 | R2 |
| 14a | `proposed-specs/14a-render-stored-pan-and-matrix-automation.md` | Render stored pan and matrix automation | 10a, 10b, 12 | R2 |
| 14b | `proposed-specs/14b-edit-the-pan-and-matrix-group-cell.md` | Edit a pan or matrix group cell under stored automation | 14a | R2 |
| 15 | `proposed-specs/15-render-stored-input-trim-and-polarity-automation.md` | Render stored input trim and polarity automation | 10a, 10b, 12, #1261, #1346 | R2 |
| 16a | `proposed-specs/16a-render-stored-input-filter-automation.md` | Render stored input HPF and LPF automation | 15, #1329, #1346, #1407, finding F1 confirmed | R2 |
| 16b | `proposed-specs/16b-edit-the-input-filter-group-cell.md` | Edit an input filter group cell under stored automation | 16a, #1262 | R2 |
| 17a | `proposed-specs/17a-prove-effects-partition-invariant-with-point-spans.md` | Prove every launch effect partition-invariant with Point spans | #1069 | R3 |
| 17b | `proposed-specs/17b-process-an-effect-node-in-pieces-at-automation-events.md` | Process an effect node in pieces at automation events | 04, 07, 17a | R3 |
| 18a | `proposed-specs/18a-compile-and-bind-stored-effect-automation.md` | Compile and bind stored effect parameter automation | 03a, 10a, 17b, #1306 (same batch), #1345 | R3 |
| 18b | `proposed-specs/18b-render-stored-effect-automation-across-seeks-and-hosts.md` | Render stored effect parameter automation across seeks and on both hosts | 18a (same push) | R3 |
| 19 | `proposed-specs/19-classify-and-carry-effect-automation-edits.md` | Classify and carry effect automation edits | 18b, #1279, #1280 | R3 |
| 20 | `proposed-specs/20-render-stored-parametric-eq-automation.md` | Render stored parametric EQ automation | 19, #1337, finding F1 confirmed | R4 |
| 21a | `proposed-specs/21a-retire-automation-enqueue-wire-identities.md` | Retire AUTOMATION_ENQUEUE and AUTOMATION_CANCELED from the protocol wire and dispatch | 01, #1309; no later than #1315's C ABI half | P1 |
| 21b | `proposed-specs/21b-delete-the-protocol-automation-queue.md` | Delete the protocol automation queue, its records and counters | 21a (same push) | P1 |
| 22 | `proposed-specs/22-reserve-the-c-abi-automation-span-limit.md` | Reserve the C ABI's automation span limit | 21b, #1306 | P2 |
| 23a | `proposed-specs/23a-compare-stored-automation-across-browser-and-native.md` | Compare stored automation bits across the browser and native hosts | 13b, 14a, 15, 16a, 20, #1399 | Q |
| 23b | `proposed-specs/23b-compare-stored-automation-on-aarch64-and-across-seeks.md` | Compare stored automation bits on AArch64 and across seek times | 23a, #1019 | Q |
| 24a | `proposed-specs/24a-build-the-stored-automation-console-benchmark-row.md` | Build the stored-automation console benchmark row | 20 | Q |
| 24b | `proposed-specs/24b-record-the-stored-automation-console-benchmark-baseline.md` | Record the stored-automation console benchmark baseline | 24a | Q |

**Batches** (CI-conscious, one push each; a building block merges in the batch of its first user,
so `main` never holds code that nothing calls):

1. **R1, the first rendering:** 01, 02, 04, 05, 06a, 06b, 07, 08, 09a, 09b, 10a, 10b, 11, 12. After
   it, stored fader automation renders on both hosts, both hosts can seek the timeline, and the
   fader row is out of the mask.
2. **R2, the other strip rows:** 13a, 13b, 14a, 14b, 15, 16a, 16b. Each brings the in-block
   operations of its own stage, on slice 08's model.
3. **R3, effects:** 03a, 03b, 03c, 17a, 17b, 18a, 18b, 19 and #1306. This is the batch D15-5 names.
4. **R4:** 20.
5. **P1, the protocol:** 21a and 21b, after R1 (21a moves the hold rule's home) and no later than
   #1315's C ABI half.
6. **P2:** 22, after R3 (#1306).
7. **Q:** 23a, 23b, 24a and 24b, after R4.

**After the owner rules on OQ1:** if the answer is B or C, one more slice adds the stored field and
its edit (B: an override value and flag per automated lane; C: an offset per automated level lane
and a stored mute term). It is not drafted here, because its fields depend on the answer. If the
answer is A, nothing is added. If OQ2 is answered yes, one slice adds route targets after R2.

### Amendments to existing specs

| Spec | Section | Change | Why |
|---|---|---|---|
| #1306 | D1 | `stored` is A7: the number of cells in `C(i)`, 0 for a target-owning effect. Delete the `AUTOMATION_ENQUEUE` term. The live term counts only `Block` cells the session does not automate. | A7, A8; an automated cell takes no live span. |
| #1306 | D3 | "capi keeps S only for `per_block_automation_density`" becomes "capi keeps S until slice 22 reserves the field". | A9. |
| #1306 | Dependencies | "the slice that renders stored effect automation" is slices 18a and 18b. | A11. |
| #1315 | D1-D3, gates 1, 2, 4 | Move to slice 21a, which retires the command instead of adding `ProviderFeatures::automation`. D4 (browser bypass), D5 and D6 stay. | A9; a switch that is never set true is code a later slice deletes. |
| #1315 | D5 | The docs say the refusal is permanent and the command is retired (slice 21a). | A9. |
| #1335 | Product outcome, Dependencies | "the first is filed from #1058's design" names slice 09a; slice 03a extends `effect_automation_diagnostics` with unit, domain and shape rules. | A3, A11. |
| #1053 | D12, D15 | D12: the mask becomes per row, from slice 10a to slice 20. D15's "S bounds only `AUTOMATION_ENQUEUE` density" becomes "S has no role after slice 22". | A9, A10. |
| #1054 | D3 | The table also gives the jump ramp of stored automation per row. | A3. |
| #1054 | D10 | A fader change on a muted lane (or during a mute ramp) updates only the remembered gain, with no retarget, on the live path too (finding F12). | A1.5. |
| #1365 | Product outcome, classification | On a session with stored automation, a `control_smoothing` edit also writes the program's jump-length cell (slice 12), so its path is `live`; without automation it stays `model_only`. | A10. |
| #1312 | D3 | For an automated fader lane, a multi-word cell, its word count set at preparation, holds the offsets of the VCAs that reach it, in order, and the mute cell's value word holds the non-automated mute terms; for an automated matrix group the matrix cell holds the pan positions or the non-automated coefficients. No cell is added. | A1.5. |
| #1345 | D1 | For an EQ section with an automated band value, the section's target cell holds the band's semantic values (the group cell), and render designs the target. | A1.5, A1.6. |
| #1262, #1346 | D-section on the filter target | For a lane whose HPF or LPF is automated, a live edit of the other filter writes the group cell, not a designed pair; the order check of A3 runs first. | A1.5, A3. |
| #1382 | D3 | The shared commit's automated-lane and group rules apply in the Worker; no browser-only path. | A2, A1.5. |
| #1225 | D3 | The route diff yields no record for a route whose source mute is automated; slice 13b's route cell carries it. | A6. |
| #1312 | D1 | A cell can be constructed with a word count set at preparation (for slice 11's VCA offsets cell), under the same publication rule and loom gate. | A1.5. |
| #1226 | D2, D3, D6 | A following route whose source mute is automated is driven in render through `gated_route_coefficients` (slice 13b); the classifier writes its cell (including a `follows_mute` toggle) and yields no follow record for it. | A6. |
| #1342 | D2 | The same, in the browser. | A6. |
| #1347 | D1 | A route lane whose source mute is automated holds the open transform, the route mute, `follows_mute` and the two mute terms instead of four folded coefficients (slice 13b). | A6. |
| #1247 | D2, Dependencies | For a member lane whose fader is automated, the effective-fader diff writes the lane's whole offsets slot through slice 11's writer, not a `FaderDb` record; #1247 depends on slice 11 and runs slice 11's gate 2 on the C ABI. | A1.5. |
| #1284 | D2 | A route lane gained or lost because a source mute became automated (or stopped being) carries its ramp state like any route lane. | A6, A1.8. |
| #1277 | D4 | Gaining, losing or changing stored automation is not a change of control kind: the stage carries. | A1.8. |
| #1279 | D1, D2 | The automation program, the span window capacity it sets and the static value of an automated cell are not prepared values; an `EffectBankStage` with stored automation and no live lane carries like one without. | A1.8, A7. |
| #1280 | D1 | "Live controls in both plans or in neither" ignores stored automation, which is not a live control. | A1.8. |
| #1351 | Context, D-section on counters | `CANCELED_AUTOMATION` leaves with slice 21b; the snapshot serves the remaining counters. | A9 (F13). |
| #1316 | D2-D3 | The seek report has a row for the timeline consumer. | A1.2. |
| #1317 | Header text | The header states the timeline, the session seek and that a per-source seek leaves the timeline. | A1.2. |
| #1320 | Readiness and replay | `prime_ready` and `prime_block_at` cover the timeline consumer (a source with no PCM). | A1.2, D15-8 C4. |
| #1323 | D2-D3 | The discontinuity successor carries the timeline consumer, its history included. | A1.2. |
| #1396 | D1 | The timeline consumer reads the source-read clock. | A1.3. |
| #1293 | D6 | Also export the session seek, anchored, beside the anchored source seek. | A1.2. |
| #1297 | Context | "The engine has no playhead" becomes: the timeline (slice 04) gives the frame at any anchor. | A1.2. |

## Owner questions

### OQ1. What does a moved control do when the producer automated it?

**Background.** A producer can automate a control, for example a vocal fader that rides up in the
chorus. The engine plays that automation on every device. A fan, or the producer in the browser
mixer, can also move the same control by hand. The engine must know what that move does. Your answer
also sets what a fan's personal mix stores for an automated control (#1057, part 3; the other parts
of that question stay open and this answer does not decide them).

**Options.**

- **A. Automation wins.** The move is stored but not heard while the automation exists. Cost: none;
  this is what ships first. A fan cannot change an automated control.
- **B. Override until re-enabled** (as Ableton Live). The move replaces the automation for that
  control until a "resume" edit. Cost: one stored value and one flag per automated control, one edit.
  The producer's ride is lost for that fan, also after the producer updates the session.
- **C. Offset** (as the Trim modes of Logic Pro, Pro Tools and Cubase). On a level control (fader,
  input trim) the move adds a stored offset in dB to the curve; a mute adds to the automated mute.
  Other controls behave as A. Cost: one stored offset per automated level control and one stored
  mute; one addition per event in render.
- **D. Touch** (override while held, back to the curve on release). Cost: a new "touch" edit;
  nothing is stored, so a personal mix cannot keep it. An app can build touch, latch and write
  recording itself by writing automation.

**Recommendation: C.** The fan's change survives the producer's rides and new versions, fits the
recommended personal-mix overlay of #1057, matches the DAW trim modes, and costs one addition per
event. A ships first; C adds one slice and changes nothing already shipped.

### OQ2. Should send levels be automatable?

**Background.** Stored automation can move a strip's fader, mute, pan, input and effects. It cannot
move a send (a route into a submix). DAWs automate send levels; Logic Pro's and Pro Tools' trim modes
include sends. Decision 13 made sends live; nothing makes them automatable.

**Options.**

- **1. No.** Cost: none. A producer who wants a moving send must automate the bus input instead.
- **2. Yes, for sends only:** route `gain_db`, `mute` and `channel_matrix` of routes into a submix
  become targets (a new target kind, by route ID). Routes into the output stay prepared (decision 14
  O9). Cost: an in-place V1 schema and wire addition, one slice, a route lane per automated send.

**Recommendation: 2,** after the strip rows ship (batch R2). It completes the mix vocabulary at a
small cost and keeps the output fold.

## Findings for the coordinator

- **F1. Filter designs in render.** A1.6 and slices 16a and 20 design EQ and input-filter targets on
  the render thread for stored automation. That amends `docs/EFFECT_CONTRACT_V1.md:157-163` ("prepares
  fixed targets off render"), the prepared-target contract ("Render-side code never calls this
  method", `crates/effect-contract/src/prepared_target.rs:6-8`, `:73`) and the EQ's code rule
  (`crates/parametric-eq/src/lib.rs:3518-3520`). D2's
  #808 amendment governs live edits (`docs/rulings/builtins-input-liveness-d2.md:6-14`) and leaves
  stored rendering to #1058 (`:163-179`). Root confirms the amendment; the alternatives fail A4 or A5
  (A1.6).
- **F2. Ruling text that presumes `AUTOMATION_ENQUEUE` is wired.** D15-13 E4 says "until #1058 wires
  it", D15-5 says S "bounds only `AUTOMATION_ENQUEUE` density", and D15-2 keeps "Automation" records
  FIFO (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md:82`, `:139-143`,
  `:407-410`). Under A9 the command is retired, S has no role after slice 22, and E4's item for the
  command moves from #1315 to slice 21a. Root updates the record and sequences slice 21a no later than
  #1315's C ABI half.
- **F3. `AGENTS.md` "Interfaces and transports"** (`AGENTS.md:49`) says the protocol uses absolute
  sample-time parameter events and high-rate automation may use point batches or segments. After A9
  those are the stored automation edits only. Root may reword.
- **F4. Two positions.** `TRANSPORT_SET` stores an endpoint-local position nothing reads
  (`crates/host-core/src/control_provider.rs:424-438`); A1.2 adds the timeline. #1315 examined
  `TRANSPORT_SET` and kept it. Root decides whether its position reports the timeline or is retired.
- **F5. #1306's EQ live term.** #1306 D1 gives the EQ 44 live cells, but the EQ stages no span: it
  counts every span invalid (`crates/parametric-eq/src/lib.rs:3513-3520`) and its live edits ride
  target cells. Its exact live span term is 0, which changes #1306's predicted 60,480-byte saving.
- **F6. Span vocabulary with no producer.** With A1, effects receive only `Point` spans. After slice 21b,
  `AutomationSpanKind::{Step, Linear, Exponential}`, `automation_segment_value` and
  `AutomationRate::Sample` (`crates/effect-contract/src/lib.rs:139`, `:145`, `:1631-1656`;
  `docs/EFFECT_CONTRACT_V1.md:151-155`) have no production producer. Under the removal ruling root may
  file their removal, or record why they stay.
- **F7. Stale numeric documents.** `dsp-research/simd-numerics.md:9`, `:17`, `.cargo/config.toml:2`
  and `docs/REALTIME_DEPENDENCY_POLICY.md:148-150` describe a fused or runtime-dispatched FMA and a
  tolerance, against the unfused, bit-exact contract A5 cites. Not part of #1058.
- **F8. This issue's gates 3 and 4.** By root's instruction the slices are drafts under
  `proposed-specs/`, not files under `.github/ISSUE_SPECS/` with GitHub issues; root files them, and
  gate 3 is met at filing. Gate 4's `origin/main...HEAD` also lists the #1057 files this branch
  carries.
- **F9. Static edits on automated lanes.** A2 commits them as `model_only`, reading the static value
  as a document value no plan reads, as #1315 reads the session ID. #1315's definition (the state
  render reaches differs from the state the command set) can also be read the other way, which D15-13
  E4 would turn into a typed refusal. Root confirms the reading; a refusal would also refuse a
  document replacement that changes such a value.
- **F10. Silence.** A moving curve keeps a silent chain processing, because an in-flight ramp vetoes
  a skip today (A1.7). The owner's skip-on-silence priority asks for the skip to advance ramp and
  cursor state bit-exactly instead. That belongs to the silence work (#1107 and successors), for live
  ramps and stored automation together; root files it.
- **F11. Control-thread designs and the FP environment.** The canonical environment is installed only
  at render entries (`crates/capi/src/ffi.rs:813-817`; `crates/host-core/src/render_session.rs:111`).
  Preparation and live target designs run in the host's environment, so a native host whose control
  thread sets FTZ or DAZ could prepare different coefficient bits from the browser. A1.1 pins it for
  automation; root may file the same pin for every preparation.
- **F12. #1054 D10.** It keeps the ramp when a fader record reaches a muted lane, which restarts the
  ramp toward 0 and gives a `-0.0` artefact. A1.5 needs "remember only" for stored automation, or each
  grid event would restart a mute ramp. Root decides whether the live path takes the same rule
  (amendment table).

- **F13. #1351 serves a counter slice 21b retires.** *Report each configured counter's own value in
  the C ABI counter snapshot* (#1351) serves `CANCELED_AUTOMATION`
  (`.github/ISSUE_SPECS/1351-report-each-configured-counters-own-value-in-the-c-abi-counter-snapshot.md:23-24`,
  `:48`), which slice 21b retires. Root amends #1351 (amendment table).
- **F14. A checked-in fixture breaks the pan or matrix rule.** `fixtures/session/v1/builtins-automation.json`
  automates `matrix_ll` on a pan track; slice 01 moves that entry to a matrix track and updates the
  SDK twin that compares it byte for byte.

- **F15. `main` moved during this attempt.** The note and its drafts are anchored on `6ee64f484`
  (code equal to `main` at `8be19c86e`). `main` is now `68ef86651`; #1335 has landed
  (`0c19119d0`), and anchors in `crates/host-core/src/live_delta.rs` (+13 lines),
  `crates/session/src/validate.rs` (+21) and `crates/graph/src/lib.rs` (+3 in places) have shifted.
  Root re-checks the anchors of each draft when it files it, and treats #1335 as landed.

## Verification

A fresh adversarial verifier (Opus 5.5, extra-high effort) reviewed the first draft of this note and
returned **FAIL**. Every finding is folded in:

- **B1** (the bits after a seek depended on when it landed, for a quantum not a multiple of 64): the
  grid moved from the render clock to node time (A1.3, A1.4).
- **M1** (a false "pure function after a declared discontinuity" claim): replaced by the seek rule
  (builtin lanes set exactly where the seek reaches the node) and A5's precise contract.
- **M2** (live edits of one cell overwrote an automated cell of the same target): target groups
  (A1.5) and the filter order rule (A3).
- **M3** (`follows_mute` toggles were not followed): every route into a submix whose source mute is
  automated gets a lane with `follows_mute` in its cell (A6).
- **M4** (static edits as `model_only` against #1315's definition): argued in A2 and raised as F9.
- **M5** (partition invariance not shown with spans): slice 17a gates it and depends on #1069.
- **M6** (carry not proven): the program is not a control kind, the whole event state carries, and
  #1277, #1279 and #1280 are amended (A1.8).
- **Minors**: silence (F10), one seek function on both hosts (A1.2), the FP environment (A1.1, F11),
  the jump spacing rule (A3), the mute-ramp rule (A1.5, F12), the pan or matrix rule (A3), the split
  and per-row mask (staged plan, A10), F2's wording, the scalar exception (A1.7), the cost table, and
  every anchor it named. One finding was not taken: the filter check does use two `sqrt` (its spectral
  norm, `crates/builtins/src/filter_control.rs:123-132`).

A second fresh verifier (Opus 5.5, extra-high effort) reviewed the revised note, its decision
record and the 37 drafts, and returned **PASS-WITH-FIXES** (no blocker; eight majors on package
consistency, a vacuous gate, two unowned changes, interim code across same-batch slices, a missing
no-bit-moves gate and the moved `main`). Every finding is folded in: the plan table is the union of
the drafts' direct dependencies, the cross-references use split names, the interim code is removed
(09a builds the VCA offsets cell, 12 owns the jump lengths from the start, browser refusals sit in
the rendering halves), and F15 records the moved `main`.

## Spec anchors, checked again

Every anchor of the #1058 spec points at its stated text on `6ee64f484`:

- `docs/SESSION_SCHEMA_V1.md:220-225` (the table renders nothing); `docs/REALTIME_MEMORY.md:13`
  (#1058 owns stored rendering).
- `tools/bench/src/console.rs:138-158` (the `console_mixing_automation` row).
- `crates/session/src/validate.rs:823-840` (`BUILTIN_AUTOMATION_TARGETS`) and `:842-853` (the
  inertness note).
- `crates/host-core/src/live_delta.rs:181-185` and `:234-236` (the mask).
- `crates/capi/src/runtime/compile.rs:130-133` (`per_block_automation_density`).
- `crates/protocol/src/queue.rs:801` (`try_dequeue_automation`; its callers are the control-side
  cancellation, `crates/protocol/src/controller.rs:3464`, protocol tests and
  `tools/audit/src/protocol.rs:470`; no render caller).

Anchors in other documents that moved: decision 14 cites `track_mono_source` at
`crates/builtins-compiler/src/lib.rs:3815-3820`; it is now at `:3967`. Decision 14 and #1315 cite the
`AutomationRate::None` refusal at `crates/protocol/src/controller.rs:3347-3351`; it is now at
`:3366-3370`. The EQ's `apply_target_lane` is at `crates/parametric-eq/src/lib.rs:2860-2894`, not
`:2815-2829` as decision 14 cites.

## Sources

Primary and official sources, paraphrased. Vendor manuals support interface behaviour only.

- [S1] W3C, *Web Audio API 1.1*, Working Draft 22 September 2026, §1.6 "The AudioParam Interface",
  §1.6.1 (`value`), §1.6.2 (`linearRampToValueAtTime`, `exponentialRampToValueAtTime`), §1.6.3
  "Computation of Value". https://www.w3.org/TR/webaudio-1.1/
- [S2] Steinberg, VST 3 SDK, `IParamValueQueue` ("Implicit Points", "Jumps") and `IParameterChanges`;
  "Parameters and Automation", §Automation Playback.
  https://steinbergmedia.github.io/vst3_doc/vstinterfaces/classSteinberg_1_1Vst_1_1IParamValueQueue.html,
  https://steinbergmedia.github.io/vst3_doc/vstinterfaces/classSteinberg_1_1Vst_1_1IParameterChanges.html,
  https://steinbergmedia.github.io/vst3_dev_portal/pages/Technical+Documentation/Parameters+Automation/Index.html
- [S3] Apple, *Logic Pro User Guide for Mac*, "Choose automation modes in Logic Pro for Mac".
  https://support.apple.com/guide/logicpro/choose-automation-modes-lgcpb1a6ab26/mac
- [S4] Avid, *Pro Tools Reference Guide 2024.6*, chapter "Automation Modes" (pp. 1325-1331) and
  "AutoMatch Time" (p. 1331). https://resources.avid.com/SupportFiles/PT/Pro_Tools_Reference_Guide_2024.6.pdf
- [S5] Ableton, *Live 12 Reference Manual*, chapter 26 "Automation", §26.4 "Overriding Automation".
  https://www.ableton.com/en/live-manual/12/automation/
- [S6] Steinberg, *Cubase Pro 15 Help*, Automation: "Automation Modes", "Touch", "Trim".
  https://www.steinberg.help/r/cubase-pro/15.0/en/cubase_nuendo/topics/automation/
- [S7] J. O. Smith III, *Physical Audio Signal Processing*, "Linear Interpolation" and "Delay-Line
  Interpolation" (linear interpolation; zipper noise from unsmoothed changes).
  https://ccrma.stanford.edu/~jos/pasp/Linear_Interpolation.html
- [S8] S. J. Orfanidis, *Introduction to Signal Processing*, §8.1.3 (linear interpolation, Eq. 8.1.48)
  and §8.3.1 (first-order exponential smoother). Author copy:
  https://web.archive.org/web/2023id_/http://www.ece.rutgers.edu/~orfanidi/intro2sp/

[S7] and [S8] support the interpolation law of A1.4: linear interpolation between samples of a slowly
varying control is cheap and keeps the change smooth when the control's bandwidth is small against
the update rate.
