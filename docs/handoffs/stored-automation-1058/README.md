# Findings: render stored session automation in the engine, identically on every platform (#1058)

Issue: *Research: render stored session automation in the engine, identically on every platform*
(#1058), stream K of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-16). Attempt 4.

**Commit.** Every `path:line` in this note and in its drafts was read on `6ee64f484`. That commit
adds only the #1057 note to `main` at `8be19c86e`, so every code anchor is also an anchor on `main`
at `8be19c86e`. Attempt 2 runs on `45c5a1819`, which changes only documents after `6ee64f484`
(`git diff --name-only 6ee64f484 45c5a1819` lists only `docs/handoffs/` and `.github/ISSUE_SPECS/`),
so every code anchor, the new ones of attempt 2 included, reads the same on both. Attempt 3 runs on
`c63f5f37d`, which also changes only documents after `6ee64f484` (`git diff --name-only 6ee64f484
c63f5f37d` lists only `docs/handoffs/` and `.github/ISSUE_SPECS/`), so the same holds for it and for
the anchors attempt 3 adds. Attempt 4 runs on `423b9d4a1`; `git diff --stat 6ee64f484 423b9d4a1 --
crates hosts sdk tools scripts` is empty, so the same holds for the anchors attempt 4 adds. `main`
has moved since (finding F15). The spec's own anchors were written on an earlier `main`; each one was read
again (section "Spec anchors, checked again").

**Authority.** The owner ruled that the core engine renders a producer's automation from the
session file, identically on every platform (`docs/rulings/engine-footprint-2026-09-28.md:52-53`).
Decision 15 binds every choice here to the no-shortcuts principle
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md:26-28`). A1 and
A3-A11 are design decisions for the implementing issues. A2 is an owner question with a
recommendation. A second owner question (send automation) is in "Owner questions". Points that
touch a decided item are in "Findings for the coordinator". F1 asks root to decide before slices are
filed (filter designs in render; slices 16a, 16b and 20 wait for it). F16 (the VCA offsets cell's
layout) is decided by root: it sums the VCA offsets first, a class B change to #1242's order that
root rules under the standing summation-order ruling, so every VCA edit stays live on a session
with stored fader automation (slices 09a and 11). No other point reopens a decided item.

**Review.** In attempt 1 two fresh internal verifiers reviewed the drafts (FAIL, then
PASS-WITH-FIXES), and the attempt-1 adversarial verdict returned FAIL (one major, seven minors,
seven nits). Every finding of all three is folded in (section "Verification").

**No benchmark ran.** The one quoted time comes from a recorded run (section "Costs").

## The decisions in one line each

| Answer | Decision |
|---|---|
| A1 | The control plane compiles each automated cell into an immutable segment table in the plan. Render evaluates it in node time (the timeline delayed by the node's arrival), at a fixed 64-sample grid and at exact jump samples, and feeds the existing strip ramps, effect `Point` spans (in pieces) and filter targets. Hosts move the timeline with one session seek. |
| A2 | Owner question. Recommendation: an offset on level rows (fader, trim), OR on mute, automation wins elsewhere. Until the ruling, automation wins on every row by one permanent rule in the shared commit, the same on both hosts: a live edit of an automated cell commits as its fallback value, `model_only`. No host refuses it, and no host has its own admission path. |
| A3 | The table is enough. It gains rules, not shapes: the hold rule, one entry per lane, the pan or matrix form, 64 samples between jumps, the target's unit and domain, shape limits, the filter order, and jump ramps from `control_smoothing`. |
| A4 | Memory per automated cell (one lane of one parameter) is `80 + 32·n` bytes, `n` its segment count, owned by the plan and independent of song length. CPU per block is at most `2⌈q/64⌉ + 3` events per cell times a per-row operation count. |
| A5 | The same session and the same host operations give the same bits on every target. Scalar `f64` evaluation through `crates/math` at events only, the existing unfused kernels and a grid in node time make it so; after a session seek the automation is independent of when the seek landed and of the quantum. |
| A6 | Every route into a submix whose source mute is automated gets a route lane with `follows_mute` as a cell word. At each automated mute event render calls `graph::gated_route_coefficients`, at the render sample where the send's tap carries the curve's timeline sample (A1.3), over the strip's ramp. |
| A7 | `stored(i)` is the number of distinct automated cells of instance `i` (two for `both` on a `PerLane` parameter); 0 for a target-owning effect (the EQ). At most the instance's `Block` cell count. |
| A8 | No. `AUTOMATION_ENQUEUE` never stages spans. #1306 drops its third term. |
| A9 | The C ABI never serves `AUTOMATION_ENQUEUE`. The refusal is permanent, and the command, its event and its queue leave the protocol registry (slices 21a-21c in batch P1, which needs only slice 01; slice 26 retires the span vocabulary no producer uses; slice 22 reserves the C limit). No ack can precede a drop. |
| A10 | The mask becomes per target row. Slice 10 removes the first row (fader) and slice 20 the last (EQ). An automation edit becomes a `rebuild` that carries every owner and completes `exact`; a static-value edit on an automated lane gives no record, on both hosts; a live edit of a group's other cell is a `live` group-cell write from the slice that renders the group. |
| A11 | Forty-one slices in twenty-six steps (drafts in `proposed-specs/`), in seven batches. Every slice that renders stored automation lands after #1382, so its browser half acts on the shared commit. Slices 18a and 18b render stored effect automation; #1306 lands in their batch. |

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
  prime adoption (#1320: it advances by the prime like a source) and on the source-read clock
  (#1396), its history included. Those specs land first and are not amended: slice 04b adds the
  timeline to each mechanism they build, and depends on them.
- **Session seek.** A host moves the playhead with one function both hosts call, in `host-core`'s
  `SourceControlSet` (#1309 moves only the C ABI wrapper): generation `g`, timeline sample `T`, and an optional anchor `A` on the source-read clock.
  It seeks the timeline and every declared source to `T` (each source clamped to its region) with
  generation `g`, all or nothing: it checks every consumer's command slot before it pushes any. Only
  the control thread pushes and render only pops, so room cannot shrink between the check and the
  push. The C ABI exposes it as a new call with a feature bit (D15-12's growth rule); the browser as
  an export, which the SDK's new `seek` calls before it refills each ring with generation `g`.
  #1323 D1 still asks the host to declare a discontinuity before a seek of every source.
- **Per-source seeks stay**, for a stem a transaction adds (`crates/capi/include/miso_engine_v1.h:96-108`).
  They never move the timeline. The timeline's seek lands in #1316's seek report as a row of its own
  (slice 04b).
- **Loop.** The engine adds no loop mechanism. A host loops by an anchored session seek at the end of
  each pass, as it would for sources. An anchor is a quantum multiple
  (`crates/source/src/lib.rs:772-777`), so a loop seam lands only on a block boundary: loop points
  are quantum-granular. A host that wants a seam inside a block renders the pass end itself; the
  engine does not split a block for a seek.
- **The protocol's transport position.** `TRANSPORT_SET` stores a position nothing reads
  (`crates/host-core/src/control_provider.rs:424-438`). A second, settable playhead beside the
  timeline would contradict it and cannot move it, since a seek needs the host to refill every
  ring. Slice 25 retires that field; the transport keeps its state (finding F4).

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
  coefficient ramps, which reach the target at `A + 64`, `docs/EFFECT_CONTRACT_V1.md:165-166`).
  A ramp of length `L = 0` is a step: the lane outputs the target at `τ` itself, so its completion
  sample is `τ`, never `τ - 1`. That is the case of an effect parameter whose smoothing rule is
  `None` (the gate's hold after #1336; draft 18a D2 uses `τ` for it) and of a jump under an
  `explicit` 0 of `control_smoothing` (A3). `L`
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
- **Fader** (id 5): the event value `v` (dB) is composed with the lane's VCA offsets, then goes
  through the fader's own `db_gain` (`crates/builtins/src/lib.rs:5356-5358`). Today
  `vca_effective_db` (`crates/session/src/vca.rs:19-34`) adds the member first, then each offset in
  ascending VCA-ID order (`:96-120`), so a precomputed sum of the offsets can give other bits, and
  each cell layout that keeps that order either narrows #1247's live set or is sized by a host cap.
  Root decided the layout (finding F16): the composition sums the offsets first, on the static
  path too: `clamp(f64(v) + S, -144, 24)` rounded once to
  `f32`, `S` the `f64` sum of the reaching offsets in ascending VCA-ID order. Each automated fader
  lane then has a cell of three words (`S` and a ramp word) that the control plane writes on any VCA
  edit; a lane no VCA reaches holds `S = +0.0`, which keeps the gain bits (`db_gain(±0) = 1`). A
  flat curve at `v` renders the bits of a plan prepared with static `v`, whatever the number of
  VCAs, and every VCA edit, adding or removing a VCA included, is a live cell write (slices 09a and
  11).
- **Mute** (id 6): the lane's stage mute is `curve || vca_mute || solo_mute`; the last two are
  control-plane terms in the strip's mute cell (`crates/host-core/src/solo.rs:255-263` composes them
  today; after #1382 the shared commit composes the browser's solo overlay, #1382 D3). A mute event ramps over the session mute length, as a live mute does
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
  `apply_prepared_filter` (`crates/builtins/src/lib.rs:1352-1426`). A1.6 says why the design runs in
  render.
- **Target groups.** Render builds some targets from several cells: a strip's matrix stage (pan
  `left` and `right`, or the four matrix coefficients), one lane's input filter pair (HPF and LPF), and
  one EQ section on one channel. When any cell of a group is automated, render owns the group's
  target: the shared commit writes a live edit of the group's other cells as a semantic value (pan
  position, cutoff, band value) into the group's cell, on both hosts, and render designs the target from the curve
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
  spacing rule) and one seek: at most `2⌈q/64⌉ + 3` events. A cursor advances one segment at a time
  (one compare and one increment); every segment is at least one sample long, so a cursor advances
  at most `q` times per cell and block, whatever the joints the grid passes. A seek repositions a
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
  the carry slices carry the stage. Slice 10 extends #1277 D4's carry predicate for it, and slice 19
  extends #1279 D1-D2 and #1280 D1; those specs land first and are not amended.

## A2. Live changes against automation

**Decision: owner question OQ1** (section "Owner questions"). Recommendation: an offset on level rows
(fader dB, input trim dB) and an OR on mute, and automation wins on every other row.

What ships before the ruling, and stays under every option. It is one permanent rule, written once
in the shared commit of `crates/control-plane` (#1309), so both hosts apply it by construction:

- The static value of an automated lane is not rendered while its automation exists. It is the value
  the lane takes when the entry is removed (A3).
- **A live edit of an automated cell commits as that fallback value.** A transaction that changes
  only that static value gives no record and commits as `model_only`; the response reports the path
  (D15-17, #1313). It is a document value no plan reads while the curve exists, the same category
  #1315 examined and kept for the session ID and the stored table: the edit's commanded state is
  the committed document, which render reaches when the entry goes (finding F9).
- **No host has its own path.** After *Admit browser live edits in the Worker through the committed
  model* (#1382) the browser has no admission of its own: every live edit reaches the Worker's
  apply and is committed by the same classifier as the C ABI's (#1382 D5). This holds whether #1382
  receives index-addressed command records and lowers them (#1382 D2 as written) or receives
  transaction edits by stable ID (the #1057 note, its finding F4); the drafts' browser gates name
  "a browser live edit through the Worker's apply", so they hold under either. So a browser fader,
  mute, pan, trim, filter or effect edit on an automated cell replies `model_only`, as the same
  C ABI transaction does. The first slice that renders stored automation (09a) depends on
  #1382, so no host ever renders a curve while a host-only path could write a live record onto it.
  No slice adds a browser refusal or a browser-only reason.
- **Why not a typed refusal.** The alternative rule is one typed refusal in the shared commit, for
  both hosts. It is rejected: a refusal would also refuse a document replacement (D15-11
  `replaceSession`) or a personal mix (#1057 part 3) that carries a changed fallback value; option A
  of OQ1 says a move is stored but not heard, which a refusal cannot do; and options B and C build
  on a stored value, so a refusal is the one rule that a B or C answer would have to undo. The
  `model_only` reply already tells the caller that nothing was heard.
- **Terms that compose.** A VCA ride on an automated member writes the lane's offsets cell (A1.5,
  slice 11). A VCA mute or a solo on an automated mute lane writes the mute cell's `terms` word
  (slice 13a). A live edit of a group's other cell writes the group cell (A1.5). None of them is a
  live record on an automated cell.
- This is option A of OQ1. **No answer undoes anything shipped:**
  - Under every option the existing static-value edit keeps its meaning: it sets the fallback value,
    gives no record and commits `model_only`. Options B, C and D add their behaviour only through a
    new edit (B's override, C's offset, D's touch). So draft 10's gate 5 and its equivalents in
    13a, 14b, 15, 16b and 19 stay true under every answer, and nothing shipped changes meaning.
  - Options B and C add a stored field per automated cell and an edit that sets it. Render reads the
    field at events, as it reads a group cell. The rules above stay true: the fallback value is
    still stored and not heard, a static edit still gives no record, and an automated cell still
    takes no live record (A10).
  - Option D (touch) is the only one that puts a host's live value on an automated cell while the
    control is held. Built as this note requires, it is a word of the automated cell (a touch value
    and a held flag, in a latest-target cell) that render reads at events and at block entry, as it
    reads a group cell: while the flag is set, the cell's events use the touch value; on release
    the cell jumps back to the curve. It adds one cell per automated cell and one host overlay edit,
    and changes nothing shipped: an automated cell still takes no live record (A10), a piece still
    holds at most one span per cell (A7, slice 17b D5), and #1306's live term still counts only
    cells the session does not automate. If D were built instead as a live record on an automated
    cell, it would undo all three; this note rules that form out.

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
  `pan_matrix`, the filter designers and `vca_compose_db` (draft 09a D6). `crates/math` is vendored libm with every
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
the render sample where the send's tap carries the curve's timeline sample, over the strip's
ramp.**

- **The composition is the live path's.** `graph_compiler::route_coefficients` maps the source lanes'
  mutes to `follow_zeroed` and calls `graph::gated_route_coefficients`
  (`crates/graph-compiler/src/ids.rs:341-355`; `crates/graph/src/lib.rs:777-812`), which its own
  documentation names the one derivation for prepared and live routes. It is a `const fn` that
  allocates nothing, so render may call it.
- **What the lane holds.** The route's open transform (`route_values`,
  `crates/graph-compiler/src/ids.rs:309-322`), its own `mute`, its `follows_mute`, and per source lane
  the source's other mute terms: VCA mute, solo mute, and the static mute of a lane whose mute is not
  automated. The shared commit writes them through the route's cell (#1347) whenever a route, VCA,
  solo, static mute or `follows_mute` edit changes them, on both hosts, so a live `follows_mute`
  toggle (D15-6) is a cell write (slice 13c).
- **When the send's gate changes.** The route op runs before the send's compensation delay
  (`crates/graph/src/runtime.rs:916-920`), so at render sample `r` it carries the tap's audio of
  timeline sample `τ_tap(r) = timeline(r + ΣP - a(tap))`, where `a(tap)` is the arrival of the
  tap's signal on the compensated graph (A1.3). The route op evaluates the source's mute curve in
  that node time, with its own cursor over the same immutable table. At its event at `r` over `R`
  samples (the session mute ramp), and at the next block entry after the cell changes:
  `follow_zeroed[lane] = follows_mute && (curve[lane](τ_tap(r)) || other_terms[lane])`; the target is
  `gated_route_coefficients(&transform, RouteGate { mute, follow_zeroed })`; the route's ramp starts at
  `r` over `R`. Only a changed gate retargets (`crates/host-core/src/live_route_state.rs:232-257`
  yields only changed routes).
- **Why the tap's arrival.** A1.3 lines automation up with the audio it acts on at every node. For a
  post-fader or post-pan tap, `a(tap) = a(fader)`, so the send and the strip change on the same
  render sample, which is #1226 D5's rule (strip and sends ramp together). For an input, post-input
  or insert-send tap behind inserts of latency `L`, the send changes `L` render samples before the
  strip, and after the compensation delay both act on the same timeline sample at the submix input.
  Timing the send at the fader's arrival would gate the send `L` timeline samples late and pass `L`
  samples of the audio the strip mutes: the leak this answer exists to prevent. #1226 D5's render-time
  rule stays the rule for a live mute, which has no timeline sample.
- **No leak.** A saved session with automated mute and pre-fader sends silences those sends exactly
  when the mute engages. Slice 13b's gates compare the output, after the ramp, with a plan prepared
  with the same mute as a static value, and check the send's and the strip's ramps on the same
  timeline sample with a latent insert; slice 13c's gate toggles `follows_mute` both ways on a
  playing engine.
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
- **Slices.** Slices 21a (the command), 21b (the event and the cancellation path) and 21c (the
  queue, records and counters) land in batch P1, one push, so `main` never holds a dead queue. They
  replace #1315 D1-D3, so no feature switch is added only to be deleted, and they land no later than
  #1315's C ABI half. P1 needs only slice 01 (the hold rule's new home, before 21c deletes the old
  one) and #1309, not the rendering batches, so the acked-with-no-effect command leaves as early as
  #1315 can. Slice 22 reserves the C field after #1306.
- **The vocabulary that served it.** With the command gone, nothing produces a `Step`, `Linear` or
  `Exponential` span or a sample-rate parameter: live records and stored automation stage only
  `Point` spans (A1.6). Under the same removal ruling, slice 26 retires those span kinds,
  `automation_segment_value` and `AutomationRate::Sample` (finding F6).
- **The acked-batch question: can an ack ever precede a drop?** No. The command is refused at decode,
  before the revision check and before any state changes, and no queue remains to hold a batch.
  Stored automation itself has no queue: render generates its events from the plan, and each window
  is sized to the per-piece bound (A7), so `Staged.dropped` stays 0
  (`crates/graph/src/runtime.rs:3522-3525`).

## A10. The classifier mask, for #1260

**Decision: the mask becomes per target row** (`crates/host-core/src/live_delta.rs:181-185`,
`:234-236`). Slice 10, *Classify fader automation edits as carried rebuilds*, removes the first row
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
  The slice that renders a group also adds its group-cell write (14b, 16b, 20), so no interim rule
  classifies such an edit as a rebuild.
- **One rule on both hosts.** The classifier is the shared one, which the browser calls in its
  Worker after #1382; no host has another admission path for an automated cell (A2).
- **A `control_smoothing` edit** reaches the program's jump lengths through one plan-level cell
  (slice 12), so on a session with stored automation it is `live` (slice 12 extends the classifier
  path #1365 adds).
- A refusable target never commits through a masked row: #1335 D4 routes an automation edit whose
  effect diagnostics are non-empty to the rebuild path, whose preparation refuses it, and slice 02
  adds the builtin rules to the same route. #1335 lands before slice 09a (D15-13 E1). Once slice 20
  removes the last row, every automation edit is a rebuild and slice 20 deletes the route.

## A11. The staged plan

**Decision: forty-one slices in twenty-six numbered steps (steps that would not fit half a working
day are split into lettered parts), each a stateless draft in `proposed-specs/`, smallest first
inside each batch. Every slice that renders stored automation lands after #1382 (through slice
09a), so no slice adds browser-only code that #1382 deletes. Slices 18a and 18b, *Compile and bind stored effect parameter automation* and
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
| VCA offsets cell (layout 4 of F16, root's decision) | 36 plus the cell's fixed words (#1312's three slots of three words: `S` as one `f64` and one ramp word) | per automated fader lane, whatever the session's VCA count |
| Follow state of a route lane: #1347's cell of seven words (transform, route mute, `follows_mute`, two mute terms) in #1312's three slots | 84 plus the cell's fixed words | per route into a submix whose source mute is automated |
| Timeline history | `16·K`, `K = ⌈A_max/q⌉ + 1` | per plan |
| Effect window | #1306's per-span bank term (`8,944 + 360·S` per eight-lane bank, `4,560 + 200·S` per four-lane bank) | `S` includes A7 |

- **Formula.** `M = Σ_cells (80 + 32·n_c) + 32·groups + 36·F + 84·follow_routes + 16·K + window
  bytes`, plus the fixed words of each cell, where `F` is the number of automated fader lanes
  (F16's layout 4); a lane with no VCA is charged too, since its cell exists for a later VCA edit.
  No term reads a duration. `n_c` is bounded by the document: `Σ n_c ≤ document_bytes / 99`.
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
- **Cursor advances.** One compare and one increment per segment boundary crossed, at most `q` per
  cell and block (every segment is at least one sample long, A1.7); a seek adds one binary search,
  `⌈log2(n + 1)⌉` compares.
- **Bound.** `CPU(block) ≤ Σ_events (evaluate + convert + start) + Σ_cells (cursor advances) +
  Σ_pieces (call) + Σ_moving banks (ramping-path delta)`. Each sum is bounded by `q`, the bank widths
  and the automated cell count, and the seek term by `log2` of a cell's own segment count; none
  reads the song length.
- **Measurement.** This issue runs no benchmark. Slices 24a and 24b add and record one descriptive row on the real console
  path (frozen workload, one warmup, two measured rounds).

## Staged plan

Each row lists the slice's direct dependencies; a draft's own Dependencies section says the same.

| # | Draft | Title | Depends on | Batch |
|---|---|---|---|---|
| 01 | `proposed-specs/01-validate-automation-lanes-in-the-session.md` | Validate stored automation lanes in the session crate and state the hold rule | — | P1 |
| 02 | `proposed-specs/02-validate-builtin-automation-targets-at-preparation.md` | Validate builtin automation targets against their rows at preparation | 01, 07, #1335 | R1 |
| 03a | `proposed-specs/03a-validate-effect-automation-units-domains-and-shapes.md` | Validate effect automation units, domains and shapes at preparation | 01, 02, #1335 | R3 |
| 03b | `proposed-specs/03b-mirror-the-automation-rules-in-the-sdk-builder.md` | Mirror the stored automation rules in the SDK builder | 01, 02, 03a | R3 |
| 03c | `proposed-specs/03c-author-submix-automation-targets-in-the-sdk.md` | Author submix automation targets in the SDK and enginectl | — | R3 |
| 04a | `proposed-specs/04a-build-the-timeline-consumer-and-producer.md` | Build the timeline consumer and producer in the source crate | ordering: #1316, #1318, #1320 | R1 |
| 04b | `proposed-specs/04b-give-every-plan-a-timeline-that-carries-like-a-source.md` | Give every plan a timeline that carries like a source | 04a, #1285, #1316, #1320, #1323, #1396 | R1 |
| 05 | `proposed-specs/05-seek-the-session-from-the-c-abi.md` | Seek the timeline and every source in one C ABI call | 04b, #1309, #1317, #1319, #1323 | R1 |
| 06a | `proposed-specs/06a-seek-the-session-from-the-headless-engine.md` | Seek the timeline and every source from the browser module export and the headless SDK | 05, #1293 | R1 |
| 06b | `proposed-specs/06b-seek-the-session-from-the-browser-sdk.md` | Seek the timeline and every source from the browser SDK and the PCM feed | 06a, #1332, #1387, #1294 | R1 |
| 07 | `proposed-specs/07-compile-stored-automation-into-per-cell-events.md` | Compile stored automation into per-cell events in node time | 01 | R1 |
| 08 | `proposed-specs/08-apply-timed-operations-inside-a-block-on-the-fader-stage.md` | Apply timed operations inside a block on the strip fader stage | — | R1 |
| 09a | `proposed-specs/09a-prepare-stored-fader-automation.md` | Prepare stored fader automation and render it flat | 01, 02, 07, 08, 12, #1054, #1285, #1309, #1312, #1382, one push with 09b, 10, 11 | R1 |
| 09b | `proposed-specs/09b-render-moving-stored-fader-automation.md` | Render moving stored fader automation, seeks and latency | 09a, 04b, 05, 06a; one push with 09a, 10, 11 | R1 |
| 10 | `proposed-specs/10-classify-and-carry-fader-automation-edits.md` | Classify fader automation edits as carried rebuilds | 09b, #1277, #1313, #1314, #1335; one push with 09a, 09b, 11 | R1 |
| 11 | `proposed-specs/11-compose-vca-offsets-with-stored-fader-automation.md` | Compose VCA offsets with stored fader automation | 09a, 10, 12, one push with 09a, 09b, 10 | R1 |
| 12 | `proposed-specs/12-hold-the-automation-jump-lengths-in-a-cell.md` | Hold the automation jump lengths in a plan cell | #1054, #1312, #1365, #1382 | R1 |
| 13a | `proposed-specs/13a-render-stored-mute-automation.md` | Render stored mute automation on the strip | 10, 12; one push with 13b and 13c | R2 |
| 13b | `proposed-specs/13b-follow-an-automated-mute-on-following-sends-in-render.md` | Follow an automated mute on following sends in render | 13a, 12, #1284, #1347 | R2 |
| 13c | `proposed-specs/13c-write-a-following-sends-route-cell-from-the-shared-commit.md` | Write a following send's route cell from the shared commit | 13b, #1342 | R2 |
| 14a | `proposed-specs/14a-retarget-the-matrix-stage-inside-a-block.md` | Retarget the matrix stage inside a block | 08 | R2 |
| 14b | `proposed-specs/14b-render-stored-pan-and-matrix-automation.md` | Render stored pan and matrix automation | 14a, 10, 12 | R2 |
| 15 | `proposed-specs/15-render-stored-input-trim-and-polarity-automation.md` | Render stored input trim and polarity automation | 10, 12, #1261, #1346 | R2 |
| 16a | `proposed-specs/16a-design-an-input-filter-section-in-render.md` | Design an input filter section in render | 15, #1407, finding F1 confirmed | R2 |
| 16b | `proposed-specs/16b-render-stored-input-filter-automation.md` | Render stored input HPF and LPF automation | 16a, #1262, #1329, #1346, finding F1 confirmed | R2 |
| 17a | `proposed-specs/17a-prove-effects-partition-invariant-with-point-spans.md` | Prove every launch effect partition-invariant with Point spans | #1069 | R3 |
| 17b | `proposed-specs/17b-process-an-effect-node-in-pieces-at-automation-events.md` | Process an effect node in pieces at automation events | 04b, 07, 17a, #1345 | R3 |
| 18a | `proposed-specs/18a-compile-and-bind-stored-effect-automation.md` | Compile and bind stored effect parameter automation | 03a, 10, 17b, #1306, #1345; one push with #1306, 18b, 19 | R3 |
| 18b | `proposed-specs/18b-render-stored-effect-automation-across-seeks-and-hosts.md` | Render stored effect parameter automation across seeks and on both hosts | 18a; one push with #1306, 18a, 19 | R3 |
| 19 | `proposed-specs/19-classify-and-carry-effect-automation-edits.md` | Classify and carry effect automation edits | 18b, #1279, #1280; one push with #1306, 18a, 18b | R3 |
| 20 | `proposed-specs/20-render-stored-parametric-eq-automation.md` | Render stored parametric EQ automation | 19, #1337, finding F1 confirmed | R4 |
| 21a | `proposed-specs/21a-retire-the-automation-enqueue-command.md` | Retire the AUTOMATION_ENQUEUE command from the protocol wire and dispatch | 01, #1309; no later than #1315's C ABI half | P1 |
| 21b | `proposed-specs/21b-retire-the-automation-canceled-event.md` | Retire the AUTOMATION_CANCELED event and the cancellation path | 21a | P1 |
| 21c | `proposed-specs/21c-delete-the-protocol-automation-queue.md` | Delete the protocol automation queue, its records and counters | 21b | P1 |
| 22 | `proposed-specs/22-reserve-the-c-abi-automation-span-limit.md` | Reserve the C ABI's automation span limit | 21c, #1306 | P2 |
| 23a | `proposed-specs/23a-compare-stored-automation-across-browser-and-native.md` | Compare stored automation bits across the browser and native hosts | 13c, 14b, 15, 16b, 20, #1399 | Q |
| 23b | `proposed-specs/23b-compare-stored-automation-on-aarch64-and-across-seeks.md` | Compare stored automation bits on AArch64 and across seek times | 23a, #1019 | Q |
| 24a | `proposed-specs/24a-build-the-stored-automation-console-benchmark-row.md` | Build the stored-automation console benchmark row | 20 | Q |
| 24b | `proposed-specs/24b-record-the-stored-automation-console-benchmark-baseline.md` | Record the stored-automation console benchmark baseline | 24a | Q |
| 25 | `proposed-specs/25-retire-the-protocol-transport-position.md` | Retire the protocol's stored transport position | 21b | P1 |
| 26 | `proposed-specs/26-retire-the-span-kinds-and-sample-rate-no-producer-uses.md` | Retire the automation span kinds and the sample rate that no producer uses | 21c | P1 |

**Batches** (CI-conscious, one push each; a building block merges in the batch of its first user,
so `main` never holds code that nothing calls):

1. **P1, the protocol:** 01, 21a, 21b, 21c, 25, 26. It needs only #1309 and lands no later than
   #1315's C ABI half, so the acked-with-no-effect command leaves as early as #1315 can. Draft 01 is
   first in it: the hold rule's new home exists before 21c deletes the old one.
2. **R1, the first rendering:** 02, 04a, 04b, 05, 06a, 06b, 07, 08, 09a, 09b, 10, 11, 12, after P1
   and after #1382. After it, stored fader automation renders on both hosts, both hosts can seek the
   timeline, the fader row is out of the mask, and a live edit of an automated fader meets one rule
   on both hosts.
3. **R2, the other strip rows:** 13a, 13b, 13c, 14a, 14b, 15, 16a, 16b. Each brings the in-block
   operations of its own stage, on slice 08's model.
4. **R3, effects:** 03a, 03b, 03c, 17a, 17b, 18a, 18b, 19 and #1306. This is the batch D15-5 names.
5. **R4:** 20.
6. **P2:** 22, after R3 (#1306).
7. **Q:** 23a, 23b, 24a and 24b, after R4.

**No placeholder and no cycle.**

- **No slice edits code that an earlier slice or spec deletes or moves.** This is the result of
  two checks, and it is only as complete as they are. Attempt 3 checked it mechanically: for
  every draft, its transitive dependency closure (draft and spec Dependencies,
  plus the batch order: #1309 before P1, #1382 before R1), and every deletion or move in each spec
  of that closure, against the draft's Context, Decisions, Deliverables, paths and gates. The
  closure reaches 70 specs. Each definite finding is fixed in the draft:
  - 06b follows #1387's routing (worker and single mode) and drain realms, and depends on #1387,
    #1332 and #1294.
  - 06a refuses with #1293 D1's `RESULT_REFUSED_LIFECYCLE` on an instance with no control half,
    calls `SessionState`'s wrapper (#1381 D2), and tests a seek while a browser candidate waits
    (#1290 is in its closure).
  - 05 puts its wrapper in `crates/control-plane` (#1309 lands before P1).
  - 14b and 11 take their ramp words through `LiveRamps::resolve` (#1394 D5-D6, #1247 D5).
  - 02 calls draft 07's evaluator instead of an interim copy, and depends on 07.
  - 10 moves the superseded `model_only_edits` cases to an EQ row, which no slice unmasks before
    20, and 20 deletes them.
  - 20 names the gates that pin `AutomationTarget` (02 gate 7, 10 gate 1, #1335 gate 4).
  - #1306's text that keeps a role for S gets amendment rows (slices 21a and 21c delete it in P1).
  - 10 D4, 12 D5, 25, 21a-21c, 22 and 13c no longer name a file, line or record that their
    closure moves.
  Context anchors that describe today's code (the capi files #1309 moves, the record rings #1312
  and #1346 replace) stay as anchors of `6ee64f484`; F15 asks root to re-check them at filing.
  The attempt-3 verdict found two conflicts that check missed, and attempt 4 fixed both and
  re-checked each against the text it touches:
  - 05 D6 now builds on #1319 D4's `audit capi` seek (in 05's closure through 04b and #1320, and
    now a direct dependency): #1319's anchored generation-2 source seek and its `RESULT_OK`
    assertions stay; 05's session seek runs at generation 3 after call 4's render has applied
    #1319's seek, so no slot holds a pending seek. Checked against #1319 D4 and against 05 D1, D3
    and D4 (generation rule, results, the unanchored entry point).
  - 20 D5 keeps the copy line of the masked comparison (`live_delta.rs:236`) that 10 D1 keeps, and
    adds the EQ rows to 10 D1's `automation_row_renders`, so 10 D1's step still returns
    `LiveRebuild::Automation` for every automation edit. Checked against 10 D1 and against 20's own
    D5 and gates.
  Attempt 4 did not re-run the mechanical check over all 41 drafts; it checked these two pairs and
  the superseded gates below.
- **Earlier gates that a later slice turns red.** Each later slice names the gate and rewrites or
  deletes it in its own change, inside its own paths (AGENTS.md: a change that supersedes a test
  deletes it in the same PR):
  - 13a D2 rewrites draft 10 gate 1's mute (row 6) case to `Err(LiveRebuild::Automation)`;
  - 20 D5 rewrites draft 02 gate 7, draft 10 gate 1, both halves of #1335 gate 4, draft 19 gate 1's
    EQ case and the band-gain half of #1335 gate 3 (now a rebuild with one pending candidate);
  - 22 D5a deletes #1306 gate 2's `resource_lifecycle.rs` test at S = 128 and S = 4,096, which a
    reserved word makes `INVALID_ARGUMENT`;
  - 09a D6 rewrites #1242's pinned order test (`crates/session/tests/vca_composition.rs:79-82`).
  Attempt 4 found these from the attempt-3 verdict and from 09a's own change; it did not search
  every later slice for others.
- **Other deletions.** #1382 deletes the browser's own admission (#1382 D5), and no slice edits `admit_commands` or adds
  a browser reason. Slices 21a-21c delete the protocol queue; no slice reads it. A building block
  (04a, 14a, 16a) lands in the batch of its first user (07 lands with 02, its first user).
- **No interim rule on `main`.** The group-cell write lands with the slice that renders the group
  (14b, 16b, 20). One same-batch change of a rule exists: alone, draft 10 D3 makes a VCA ride on an
  automated member an empty delta (`model_only`), and draft 11 makes it a live offsets-cell write.
  The two are in one must-land-together group, so `main` never holds draft 10's interim result.
- **Must-land-together groups.** Each group below must reach `main` in one push: any proper subset
  on `main` leaves a state that is wrong for a user, so root files each group knowingly and never
  splits it across pushes. Each group is inside one batch, and each batch is one push.
  - **09a, 09b, 10, 11** (R1). Without 09b, a moving curve holds its first value. Without 10, a live
    fader edit on an automated lane retargets it until the next event, so the edit fights the curve
    (09b Hazards). Without 11, 10 D3 drops the `FaderDb` that a VCA ride gives an automated member,
    so an acknowledged ride is not heard.
  - **13a, 13b, 13c** (R2). Without 13b and 13c, an automated mute renders while a send that follows
    it keeps passing audio: the leak A6 exists to prevent.
  - **#1306, 18a, 18b, 19** (R3). Without 18b, effect automation renders but a seek and the browser
    do not follow it. Without 19, a live effect record on an automated cell still reaches the lane,
    while amended #1306 sizes the window without that cell's live term (A7), and 17b D5 and 18a D4
    assume that no such record exists. A piece can then hold one span more than the window holds,
    and `Staged.dropped > 0` drops an acknowledged edit. Amended #1306 alone has the same defect,
    because before 19 nothing stops that record.
  - **21a, 21b, 21c** (P1). Without all three, `main` holds a queue, event or command that nothing
    serves (a dead queue).
  - No other subset is unsafe: a building block (04a, 14a, 16a) alone is code nothing calls, which
    the batch rule above already keeps off `main`, and every other slice is a complete outcome on
    its own once its dependencies have landed.
- **Acyclic.** Every spec a slice depends on lands before it, and no amendment makes a spec
  depend on a slice. Each amendment below comes from the research of this note (A7, A8, A9, A10,
  F12, F13), not from a slice. Two amended specs are dependencies of slices, and both land before
  them: #1054 (09a, 12, 14b, 15, 24a), whose D10 row names no slice, and #1306 (18a, 18b, 22), which
  lands in the same push as 18a and whose amended Dependencies name 18a and 18b only as same-push
  partners, not as dependencies. Specs that land first and whose code a slice extends are listed
  as not amended.

**After the owner rules on OQ1:** if the answer is B or C, one more slice adds the stored field and
its edit (B: an override value and flag per automated lane; C: an offset per automated level lane
and a stored mute term). If the answer is D, one slice adds the touch cell and its overlay edit, as
A2 requires. None of them is drafted here, because its fields depend on the answer. If the answer is
A, nothing is added. If OQ2 is answered yes, one slice adds route targets after R2.

### Amendments to existing specs

Each row comes from this note's research. No row makes a spec depend on a slice: #1306's rows
name slices 18a and 18b only as its same-push partners, #1054's row names no slice, and the other
rows change specs that no slice depends on.

| Spec | Section | Change | Why |
|---|---|---|---|
| #1306 | D1 | `stored` is A7: the number of cells in `C(i)`, 0 for a target-owning effect. Delete the `AUTOMATION_ENQUEUE` term. The live term counts only `Block` cells the session does not automate. | A7, A8; an automated cell takes no live span. |
| #1306 | D3 | "capi keeps S only for `per_block_automation_density`" becomes "capi keeps S until slice 22 reserves the field". | A9. |
| #1306 | Introduction, "Not ready" paragraph, Context | "The caller's S then bounds only `AUTOMATION_ENQUEUE` density" becomes "The caller's S has no role after slice 22". The "Not ready" paragraph says #1058 recorded A7 and A8: `stored` is A7, and there is no `AUTOMATION_ENQUEUE` term (A8). "S is also the protocol's `per_block_automation_density` … That role stays" becomes "slice 21c (batch P1, before this spec) deletes `per_block_automation_density` and its read; S keeps no role". | A8, A9; slices 21a and 21c remove capability field 19 and the density field before R3. |
| #1306 | Dependencies | "the slice that renders stored effect automation" is slices 18a and 18b, which land in the same push with draft 19 (must-land-together groups); #1306 does not depend on them. | A11. |
| #1315 | D1-D3, gates 1, 2, 4 | Move to slices 21a-21c, which retire the command instead of adding `ProviderFeatures::automation`. D4 (browser bypass), D5 and D6 stay. | A9; a switch that is never set true is code a later slice deletes. |
| #1315 | D5 | The docs say the refusal is permanent and the command is retired (slice 21a). | A9. |
| #1053 | D12, D15 | D12: the mask becomes per row, from slice 10 to slice 20. D15's "S bounds only `AUTOMATION_ENQUEUE` density" becomes "S has no role after slice 22". | A9, A10; #1053 is an umbrella root keeps current. |
| #1054 | D10 | While a lane is muted (settled, or ramping toward 0), a live fader change only remembers the gain; during an unmute ramp it retargets as today. This replaces "keep the ramp": it never cuts a mute ramp and removes the `-0.0` artefact. | Finding F12, decided. |
| #1351 | Context, D-section on counters | `CANCELED_AUTOMATION` is never served: slice 21c retires it. The snapshot serves the remaining counters. | A9, finding F13. |

**Not amended.** These specs land before the slice that needs a change, so the slice owns the change
in their code, and depends on them: #1054 D3 (slice 12 D7 documents the jump ramps), #1365 (slice
12 D3), #1312 D1 and D3 (slices 09a, 13a, 14b), #1345 D1 (slice 20 D4), #1262 and #1346 (slice
16b), #1382 (no change: the slices add their rules to the shared commit it calls), #1225, #1226,
#1342 and #1347 (slices 13b and 13c), #1247 (slice 11), #1284 (slice 13b), #1277 (slice 10), #1279
and #1280 (slice 19), #1316, #1320, #1323 and #1396 (slice 04b), #1317 (slice 05), #1293 (slice
06a). #1297 needs no change: its `alignTo` start reads a stem that plays in step with the timeline,
so it gives the timeline's frame. #1335 has landed (finding F15); slice 02 extends its route.

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
- **D. Touch** (override while held, back to the curve on release). Cost: a new "touch" edit and
  one small cell per automated control; nothing is stored, so a personal mix cannot keep it. An app
  can build touch, latch and write recording itself by writing automation.

**Recommendation: C.** The fan's change survives the producer's rides and new versions, fits the
recommended personal-mix overlay of #1057, matches the DAW trim modes, and costs one addition per
event. A ships first; B, C or D each add one slice and change nothing already shipped (A2).

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

- **F1. Filter designs in render.** A1.6 and slices 16a, 16b and 20 design EQ and input-filter targets on
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
  those are the stored automation edits `0x0600`-`0x0603`, which carry absolute sample times and
  step, linear and exponential segments, so the text still holds. No change is needed; a rewording
  that names the stored edits is optional.
- **F4. Two positions, decided.** `TRANSPORT_SET` stores an endpoint-local position nothing reads
  (`crates/host-core/src/control_provider.rs:424-438`); A1.2 adds the timeline. The note decides to
  retire the stored position (slice 25, batch P1): a second settable playhead would contradict the
  timeline, and it cannot move it, because a seek needs the host to refill every ring. Reporting
  the timeline through the transport snapshot was rejected: the control thread would need a
  render-published playhead that the seek report (#1316, slice 04b D6) already gives the host. The
  transport state stays. #1315 kept `TRANSPORT_SET` for its readback, which slice 25 keeps for the
  state; this follows the owner's removal ruling (`docs/rulings/engine-footprint-2026-09-28.md:20-21`),
  so root only files the slice.
- **F5. #1306's EQ live term.** #1306 D1 gives the EQ 44 live cells, but the EQ stages no span: it
  counts every span invalid (`crates/parametric-eq/src/lib.rs:3513-3520`) and its live edits ride
  target cells. Its exact live span term is 0, which changes #1306's predicted 60,480-byte saving.
- **F6. Span vocabulary with no producer, decided.** With A1, effects receive only `Point` spans.
  After slice 21c, `AutomationSpanKind::{Step, Linear, Exponential}`, `automation_segment_value` and
  `AutomationRate::Sample` (`crates/effect-contract/src/lib.rs:139`, `:145`, `:1632-1656`;
  `docs/EFFECT_CONTRACT_V1.md:151-155`) have no production producer; the contract doc keeps them for
  "a later protocol capability", which A9 retires. The owner's removal ruling
  (`docs/rulings/engine-footprint-2026-09-28.md:20-21`) settles it: slice 26 (batch P1) retires them
  and the protocol's sample rate value, with their raw values refused and never reallocated.
- **F7. Stale numeric documents.** `dsp-research/simd-numerics.md:9`, `:17`, `.cargo/config.toml:2`
  and `docs/REALTIME_DEPENDENCY_POLICY.md:148-150` describe a fused or runtime-dispatched FMA and a
  tolerance, against the unfused, bit-exact contract A5 cites. Not part of #1058.
- **F8. This issue's gates 3 and 4.** By root's instruction the slices are drafts under
  `proposed-specs/`, not files under `.github/ISSUE_SPECS/` with GitHub issues; root files them, and
  gate 3 is met at filing. Gate 4's `origin/main...HEAD` also lists the #1057 files this branch
  carries.
- **F9. Static edits on automated lanes, decided.** A2 commits them as `model_only` by one rule in
  the shared commit for both hosts. #1315's definition is "the state render reaches differs from
  the state the command set" (`.github/ISSUE_SPECS/1315-refuse-commands-that-would-be-acknowledged-with-no-effect.md:13-15`).
  The edit sets the committed document's fallback value, render reaches that state when the entry
  goes, and the response names the path, so it is the model-only category #1315 examined and kept
  (`:44-47`). A typed refusal was rejected (A2: it would refuse document replacements and personal
  mixes, it cannot store option A's value, and a B or C answer would undo it). No browser-only
  refusal exists. Root needs only to keep #1315's reading as it is.
- **F10. Silence.** A moving curve keeps a silent chain processing, because an in-flight ramp vetoes
  a skip today (A1.7). The owner's skip-on-silence priority asks for the skip to advance ramp and
  cursor state bit-exactly instead. That belongs to the silence work (#1107 and successors), for live
  ramps and stored automation together. The owner ranks silence work high, so root files it
  promptly, before batch R2, whose rows add moving ramps to more stages.
- **F11. Control-thread designs and the FP environment.** The canonical environment is installed only
  at render entries (`crates/capi/src/ffi.rs:813-817`; `crates/host-core/src/render_session.rs:111`).
  Preparation and live target designs run in the host's environment, so a native host whose control
  thread sets FTZ or DAZ could prepare different coefficient bits from the browser. A1.1 pins it for
  automation; root may file the same pin for every preparation.
- **F12. #1054 D10, decided.** It keeps the ramp when a fader record reaches a muted lane, which
  restarts the ramp toward 0 and gives a `-0.0` artefact. A1.5 needs "remember only" for stored
  automation, or each grid event would restart a mute ramp. The note decides that the live path
  takes the same rule while the lane is muted (settled, or ramping toward 0): it never cuts a mute
  ramp, it removes the artefact, and one rule serves both paths. During an unmute ramp a live change
  still retargets, so the lane ends on the new value. #1054 lands before slice 08 and needs nothing
  from #1058, so the change is an amendment to #1054 D10 (amendment table); draft 08 D2 reads it.

- **F13. #1351 serves a counter slice 21c retires.** *Report each configured counter's own value in
  the C ABI counter snapshot* (#1351) serves `CANCELED_AUTOMATION`
  (`.github/ISSUE_SPECS/1351-report-each-configured-counters-own-value-in-the-c-abi-counter-snapshot.md:23-24`,
  `:48`), which slice 21c retires. Root amends #1351 (amendment table).
- **F14. A checked-in fixture breaks the pan or matrix rule.** `fixtures/session/v1/builtins-automation.json`
  automates `matrix_ll` on a pan track; slice 01 moves that entry to a matrix track and updates the
  SDK twin that compares it byte for byte.

- **F15. `main` moved during this attempt.** The note and its drafts are anchored on `6ee64f484`
  (code equal to `main` at `8be19c86e`). `main` is now `68ef86651`; #1335 has landed
  (`0c19119d0`), and anchors in `crates/host-core/src/live_delta.rs` (+13 lines),
  `crates/session/src/validate.rs` (+21) and `crates/graph/src/lib.rs` (+3 in places) have shifted.
  Root re-checks the anchors of each draft when it files it, and treats #1335 as landed. Attempt 2
  re-read its new anchors on `45c5a1819`, whose code equals `6ee64f484`'s.

- **F16. The VCA offsets cell's layout. Decided by root: layout 4, the offsets sum first.**
  - **The problem.** D15-6 makes VCA membership live on the C ABI because "a membership change
    needs no new render memory" (#1247 header,
    `.github/ISSUE_SPECS/1247-deliver-value-only-vca-edits-to-the-running-c-abi-plan.md:8-10`), and
    #1247's product outcome lists, as live: ride or mute a VCA, add or remove a VCA, change a VCA's
    members (`:16-21`). A lane with stored fader automation composes its fader value with its VCA
    offsets in render at each event, so render keeps the offsets, or their sum, in a cell that the
    control plane writes on a VCA edit. With #1242's order (the member's own value first, then each
    reaching offset in ascending VCA-ID order, `crates/session/src/vca.rs:19-34`, `:96-120`), a
    precomputed sum of the offsets can give other bits (`crates/session/tests/vca_composition.rs:81`),
    so the cell would have to hold each offset, and every such layout either narrows #1247's live
    set or is sized by a host cap.
  - **The decision (root's ruling).** The composition becomes `clamp(f64(member) + S, -144, 24)`,
    rounded once to `f32`, where `S` is the `f64` sum of the reaching offsets in ascending VCA-ID
    order, starting from the first offset. With an empty reach the member is returned unchanged, as
    today. The static path (`vca_effective_db`, so `effective_strip_faders`, #1247's classifier row,
    host-core's live VCA state and builtins lowering, which all call it) uses the same rule, so a
    cell that holds `S` gives the static bits by construction. Each automated fader lane, in every
    session, has a cell of three words (`S` as one `f64`, one ramp word): 36 bytes in #1312's three
    slots, whatever the VCA count, plus the cell's fixed words. A lane no VCA reaches holds
    `S = +0.0`: in `f64`, `v + (+0.0) == v` except that `-0.0` becomes `+0.0`, the clamp changes
    nothing for a curve value in the fader domain (draft 02), and `db_gain(±0) = 1` exactly, so the
    gain has the static bits. Every VCA edit, adding and removing a VCA included, is one live cell
    write; nothing rebuilds and no cap sizes the cell. Render adds one `f64` per event.
  - **Authority.** Root ruled it under the owner's standing summation-order ruling: a class B
    change of summation order is acceptable when it measurably helps. Here it keeps D15-6's whole
    live set at a fixed 36 bytes per lane. No file in `docs/rulings/` records that standing ruling
    (a search of `docs/rulings/` for "summation order" finds nothing), and the decision-15 record
    has no row for F16. The nearest repository text is decision 15's class B delegation for
    D15-4(a) only (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md:119-121`,
    `:609-610`). The ruling as the owner gave it names a measured speed-up as the ground; this
    change is for liveness and fixed memory. Root must record the standing ruling, and this
    reading of it, in the decision-15 record when it files drafts 09a and 11, for owner review.
  - **The amendment to #1242's order.** It is a class B change to shipped, pinned bits:
    - the order is documented at `docs/SESSION_SCHEMA_V1.md:80` ("its own value first, then the
      reaching VCAs' offsets in ascending VCA ID"), implemented at `crates/session/src/vca.rs:19-34`
      and pinned at `crates/session/tests/vca_composition.rs:79-82` (line 81,
      `vca_effective_db(24.0, [-24.0, 1e-30]) == 1e-30`, gives `0.0` in the new order);
    - decision 13 (a) does not fix the order: the offset "adds to each member's own fader"
      (`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md:114-115`), and the module doc already
      spells the rule `clamp(own_db + sum(offsets), -144, 24)` (`crates/session/src/vca.rs:5`);
    - **slice 09a makes the change** (draft 09a D6): the two functions in `vca.rs`, the sentence at
      `SESSION_SCHEMA_V1.md:80`, and the pinned test rewritten in the same change (draft 09a
      gate 6). Three more test oracles compute the member-first order today:
      `reference_db` (`crates/host-core/tests/vca.rs:268-279`), the `lane` closure of
      `a_browser_vca_renders_as_its_effective_faders_under_solo_and_mute`
      (`hosts/host-web/src/tests.rs:13358-13371`) and `vca_reference_effective` (`:13697-13715`).
      They stay green, because they compare rendered PCM and the new order moves a dB value only in
      degenerate cases, but each documents a retired rule. Draft 09a D6 rewrites the three in the
      same change, so no test computes or pins the old order after it lands;
    - **which bits move.** Each term is in `[-144, 24]` (`crates/session/src/validate.rs:584` for a
      VCA offset; a member's fader likewise). With `k` terms, every partial sum is exact in `f64`
      when each nonzero term is at least `k·2^-20` dB in size (about `2e-6` dB for two terms,
      `2.5e-4` dB for 257), so both orders give the same `f64` sum and the same bits. Bits move only
      when the terms are many orders of magnitude apart, such as the pinned `1e-30` case. Today only
      static sessions render, so only their bits can move; stored automation renders nothing today.
  - **Rejected alternatives** (each keeps #1242's order; each figure is three #1312 slots of 4
    bytes per word, the ramp word included):
    - *Layout 1, one word per reaching VCA* (`12·v + 12` bytes): rejected, because any change of a
      lane's reach (a membership edit, a VCA added or removed) needs new memory and so rebuilds,
      which narrows D15-6's live set.
    - *Layout 2, one word per session VCA, `+0.0` where it does not reach* (`12·V + 12` bytes):
      rejected, because adding or removing a VCA changes every cell's word count and so rebuilds on
      a session with stored fader automation, which narrows D15-6's live set.
    - *Layout 3, a count word and room for the host's VCA cap* (`12·V_cap + 24` bytes on every
      automated lane): rejected, because a host cap, not the session, sizes render memory. On the C
      ABI `V_cap` is `maximum_vcas`, and `maximum_vcas == 0` means `maximum_tracks`
      (`crates/capi/src/runtime/compile.rs:517-541`; header `crates/capi/include/miso_engine_v1.h:191-194`;
      the capi test limits pass 0, `crates/capi/src/ffi.rs:1338`), so on a host that keeps the
      default the figure is `12·maximum_tracks + 24` with no bound of its own; preparation refuses a
      session over the cap (`crates/host-core/src/prepare.rs:1162-1170`). In the browser host-core's
      cap is `u64::MAX` (`hosts/host-web/src/lib.rs:6594`); the bound is the compiled
      `MAXIMUM_BROWSER_VCAS = 256` (`:89`), which boot enforces (`browser_vca_shape`,
      `:6398-6400`), so the figure is `12·256 + 24 = 3,096` bytes per automated lane in every
      session.
  - **The drafts.** Slices 09a and 11 are written for layout 4 and wait for no ruling. Root records
    the decision, with its authority, in the decision-15 record; it narrows nothing in #1247's live
    set.

- **F17. GitHub #1306 is closed, but its spec is open.** The body of commit `6b8bc7c96` contains
  "Fix #1306/#1058 …", which GitHub reads as a closing keyword. The attempt-2 verdict reports that
  GitHub closed #1306 at 2026-10-05T13:28Z for that reason; this attempt did not query GitHub.
  Its spec `.github/ISSUE_SPECS/1306-size-each-effects-automation-span-window-from-the-producers-its-plan-has.md`
  is still open, and this plan lands it in batch R3 (with the amendments above). Root reopens
  GitHub #1306.

- **F18. Two dependency specs are not on this branch.** Draft 16a depends on *Retarget a live input
  filter only through its designs and their mixtures* (#1407), and drafts 08, 14a and 15 cite *Keep
  every trim, fader and matrix ramp inside its endpoints* (#1408; draft 08 for its site 1, the fader
  and mute ramp kernel `gain_mute_ramp_block`). Their specs exist only on
  `codex/d15-stream-g` (`5cfc1fb6d`), not on `c63f5f37d`. Attempt 3 read #1407 there: its D5
  keeps `InputStage::apply_prepared_filter` as the one retarget entry and changes only its rules,
  so 16a's call through it stays valid. Root merges stream G's specs before it files 16a.

- **F19. The iOS memset rule. The worklet trap owners do not apply.**
  - **The rule.** The required `cross-target` job (`.github/workflows/qualification.yml:889-915`)
    runs `bash scripts/check-cross-targets.sh`. It counts `bl _memset_pattern16` in the iOS
    release assembly of each product crate, which is every workspace crate in `capi`'s normal
    dependency closure (`scripts/lib/product-crates.sh`). It fails when a count rises above the
    crate's ceiling, and on any call in a crate that has no row
    (`scripts/lib/aarch64-known-defects.py:62-73`, `:131-160`). `builtins-compiler`, `rack`,
    `source` and `effect-compiler` are in the closure today with no row; `control-plane` (#1309)
    and `automation` (draft 07) join it with no row. A new call site of an inlined four-lane kernel,
    for example a split piece path beside a fused path, can add calls.
  - **What every draft does.** Each draft that changes product code in a crate of that closure
    runs `bash scripts/check-cross-targets.sh` as a gate and adds no `memset_pattern16` call. The
    fix for a new call is in the slice's own code, never a ceiling: a raised ceiling adds a libc
    call in render. No draft authorizes an `IOS_MEMSET_CEILINGS` row (draft 23b may add only a
    #1019 expected test failure to `scripts/lib/aarch64-known-defects.py`). The drafts cite
    this finding as "the iOS memset rule".
  - **The trap owners do not apply.** `scripts/check-web-audioworklet-callgraph.py` follows only
    direct `call N` edges (`CALL`, `:96`). Render reaches every new piece of code through
    `call_indirect`: the plan executor (`crates/engine/src/realtime/plan.rs:550`,
    `Option<Box<dyn PreparedPlanExecutor>>`), every bank stage (`crates/rack/src/lib.rs:1470`,
    `Box<dyn BankStage>`) and every effect (`crates/graph/src/lib.rs:884`,
    `Box<dyn PreparedNativeEffect>`). No draft adds Rust on the direct path from
    `miso_engine_web_v1_render`, and draft 06a's export is a control export with its own list
    entry. So no slice can add a trap owner to the closure that the gate walks. Draft 20's hazard
    on trap owners is conservative and stays.

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
  #1277, #1279 and #1280 are extended (A1.8; attempt 2 moved the change from amendments into
  slices 10 and 19).
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
the rendering halves), and F15 records the moved `main`. Attempt 2 removed those browser refusals
(M1 below).

**Attempt 2.** The attempt-1 adversarial verdict (`/home/bl/misofm/submix-verdicts/1058-attempt1.md`)
returned **FAIL**: one major, seven minors, seven nits. Each is folded in:

- **M1** (browser refusals that #1382 deletes, and R2 slices that edit code #1382 deletes): slice
  09a depends on #1382, so every rendering slice acts on the shared commit on both hosts. Draft 10b
  and `COMMAND_REASON_AUTOMATED` are deleted; the browser refusals of 11, 13a, 14a, 15, 16a, 19 and
  20 are removed; 13b is rewritten onto the shared commit and split (13b render, 13c commit). The
  permanent rule is `model_only` in the shared commit, decided against a typed refusal (A2, F9).
  Amendments that made an earlier-landing spec refer to a later slice are removed; the slice owns
  the change (amendment table, "Not amended"). That also removed a real cycle: slice 04 depended on
  #1316, which an amendment asked to report the timeline.
- **m1** (P1 tied to R1): P1 depends on slice 01 alone, which moves into P1.
- **m2** (send ramp timing): the send's gate changes at the tap's arrival (A6, slice 13b D2), the
  timing A1.3 requires; the fader's arrival would leak `L` samples behind a latent insert.
- **m3** ("no answer undoes anything" covered only B and C): A2 states what D changes and requires
  it to be a cell word read at events, so A10, slice 17b D5 and the #1306 live term stay true.
- **m4** (sizes and interim rules): 04 split into 04a and 04b, 13b into 13b and 13c, 21a into 21a and
  21b (the queue is 21c). 14 and 16 are re-split by layer (14a and 16a the in-block operations,
  14b and 16b the product with its group-cell write), so no same-batch slice classifies an edit as a
  rebuild for a later slice to replace. Must-land-together groups fell from five to four.
- **m5** (F4, F6 handed to root): both decided; slices 25 and 26.
- **m6** (inconsistencies): 18a D8 reads `80 + 32·n`; F12 is decided and the #1054 D10 row states
  the decision, which draft 08 D2 reads; the #1225 row is gone and 13c D1 writes the cell instead of
  #1225's record, consistently.
- **m7** (citations): [S6] cites the deep pages, [S7] adds the Delay-Line Interpolation page, [S8]
  uses the real section titles; each was fetched again (section "Sources").
- **Nits:** loop points are quantum-granular (A1.2); the cursor bound is `q` per cell and block and
  joins the CPU bound (A1.7, "Costs"); 17b lists #1345; the spec's attempt record names both
  internal reviews; the three short anchor ranges are fixed (`crates/builtins/src/lib.rs:1352-1426`,
  gate-expander `:534-595`, multiband-compressor `:1228-1282`); F3 says no change is needed; F10
  asks root to file promptly.

**Attempt 3.** The attempt-2 adversarial verdict (`/home/bl/misofm/submix-verdicts/1058-attempt2.md`)
returned **FAIL**: one major, three minors, five nits. Each is folded in:

- **MA1** (06b written for the worklet routing that #1387 removes): 06b is rewritten for #1387. The
  session seek message goes to the control half (the Worker in `worker` mode, the worklet's control
  handler in `single` mode); the handler that calls the export records the accepted generation, and
  the MSB1 drain reads it in its own realm (#1387's Worker drain module, or the worklet prelude).
  Gates 1 and 2 cover both modes; #1387, #1332 and #1294 are dependencies. A fresh sub-agent then
  checked "No slice edits code that an earlier slice deletes" mechanically for every draft against
  its whole dependency closure; every definite finding is fixed ("No placeholder and no cycle").
- **m1** (F16): the middle layout, one word per session VCA with `+0.0` where the VCA does not
  reach, is bit-exact (draft 09a D1) and keeps every ride and membership change live. It does not
  keep #1247's whole live set: adding or removing a VCA changes the word count, so on a session with
  stored fader automation it rebuilds. So F16 is a root decision with three layouts and their costs,
  recommending the middle one; slices 09a and 11 are written for it and wait for the ruling; the
  Authority line is corrected.
- **m2**: the must-land-together groups are 09a-09b-10-11, 13a-13b-13c, #1306-18a-18b-19 and
  21a-21b-21c, each with its reason, in the README and in each draft.
- **m3**: draft 25 adds `tools/audit/src/protocol.rs:206` and `crates/capi/src/runtime/tests.rs:2557`;
  draft 26 adds the four test fixtures and the policy pin. Both list every user a `git grep` finds.
- **Nits**: browser gates say "a browser live edit through the Worker's apply", which holds whether
  #1382 lowers records or takes stable-ID edits (the #1057 note's F4); A2 states that the static
  edit keeps its meaning under every option; A1.4 states the completion sample `τ` for `L = 0`; the
  "Acyclic" wording is corrected; the hot-file hazards of 19 and 20 name `hosts/host-web/src/tests.rs`;
  F17 asks root to reopen GitHub #1306.

**Attempt 4.** The attempt-3 adversarial verdict (`/home/bl/misofm/submix-verdicts/1058-attempt3.md`)
returned **FAIL**: one major, three minors, nine nits. Each is folded in:

- **MA1** (F16 incomplete, false premise): the sentence "every exact layout makes some VCA edit
  need new memory" is gone. F16 now holds layout 4 (the offsets sum first, `S` in one `f64`, 36
  bytes per automated lane whatever the VCA count, every VCA edit a live write), which root adopted
  under the standing summation-order ruling (F16 records the authority and that the memory's text
  names a measured speed-up). Layouts 1-3 are rejected alternatives with one line each. Layout 3's
  sizing is corrected on both hosts (`maximum_vcas == 0` means `maximum_tracks` on the C ABI; the
  browser bound is the compiled `MAXIMUM_BROWSER_VCAS`, enforced at boot; 3,096 bytes). Drafts 09a
  and 11 are written for layout 4 and wait for no ruling; 09a D6 makes the order change and
  rewrites the pinned test in the same change. The memory formula charges `36·F`, which is right
  for a lane with no VCA, since its cell exists.
- **m1**: draft 25 authorizes the `TransportState` reliable payload and constructor in `queue.rs`
  and widens its grep; draft 02 authorizes the builtins policy script's dependency list. A fresh
  sub-agent (Opus 5.5, extra-high effort; read-only, no build) then checked every other draft's
  gates against the policy scripts and its authorized paths, and each finding is fixed in the
  draft: 04a (the host-core diagnostic arm and its table, since `SourceSeekError` is exhaustive),
  04b (host-core's test re-export of the timeline reader, the browser ceilings), 05 and 12 (capi's
  `graph` dev-dependency with `test-support`; 05's symbol-list anchor), 06a (gate 2 no longer
  claims a timeline read the SDK cannot make), 07 (`g6_full_corpus_ftz.rs`'s row count), 09a
  (host-core's `automation` edge), 09b (the source driver's block reader), 11, 13a, 13b, 14b, 15
  and 16b (the allocation gates move to `crates/host-core/tests/`, since
  `scripts/check-bench-policy.sh:257-280` bans `bench-support` in any `hosts/` manifest), 13b
  (graph's `automation` edge and the graph policy pins), 16a and 16b (a `test-support` reader of
  the filter design count), 17b (`Cargo.lock`), 18a (effect-compiler's `automation` edge and its
  policy pin), 20 (the EQ target buffer is sized at bind from `maximum_targets()`, since
  `MAXIMUM_TARGETS` is private and rack may not depend on the EQ), 21a and 21c (capi tests that
  name the deleted protocol types), and 24a (gate 1 moves to the bench test; `bench` gains `capi`
  and `session-validator`, and `tools/bench/src/console.rs` joins the approved unsafe owners, with
  the dependency-union and unsafe-owner pins and their self-tests authorized). Not checked without a
  build: new iOS `memset_pattern16` call sites (`scripts/lib/aarch64-known-defects.py`) and new
  trap owners in the worklet call graph.
- **m2**: draft 05 D6 builds on #1319 D4 (a generation-3 session seek after #1319's seek has
  applied); draft 20 D5 keeps 10 D1's copy line and adds the EQ rows to `automation_row_renders`.
  The "No placeholder and no cycle" claim now states what each attempt checked.
- **m3**: 13a, 20 and 22 name and rewrite or delete the earlier gates they turn red (list in "No
  placeholder and no cycle").
- **Nits**: row 05 lists #1309 and #1319; 21b's stray fragment is gone; 06b's producer takes a
  `number` frame and the SDK converts; 05 D5 places its paragraph in #1317 D1's sequence; 07 D3's
  `value_at` takes a segment table; 26 removes the retired rate from
  `scripts/check-parameter-metadata-v1.py`; 22's Context matches the #1306 rows; draft 08 cites
  #1408 and F18 says so; the memory formula is fixed (MA1).

### Follow-ups after PASS

The attempt-4 verdict returned **PASS** with four minors and five nits. Each is folded in:

- **m1**: the realtime gates of drafts 11, 13a, 13b, 14b, 15 and 16b move to
  `crates/control-plane/tests/`, the crate that holds the shared commit. A host-core binary cannot
  reach the commit without a dev-dependency cycle. Each draft authorizes control-plane's
  `bench-support` dev-dependency, which `scripts/check-bench-policy.sh:257-280` allows in a
  `crates/` manifest.
- **m2**: draft 09a D6 rewrites the three member-first oracles in the same change; F16 names them.
- **m3**: finding F19 states the iOS memset rule and why the trap-owner risk does not apply. Every
  draft that changes product code in `capi`'s closure runs `bash scripts/check-cross-targets.sh`.
- **m4**: draft 24a's only new unsafe owner is `tools/bench/src/console_capi.rs`, which holds only
  the C ABI calls, and its gates run the six policy scripts whose pins it edits.
- **Nits**: 09a gate 1 asks for different gain bits, with an example; 11 gate 2 uses three VCAs;
  05 D6 says why `replacements == 1` stays true; A5 and 09a D2 name `vca_compose_db`; F16's
  authority cites no private file and says that no repository ruling records the standing
  summation-order ruling.

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
  https://www.steinberg.help/r/cubase-pro/15.0/en/cubase_nuendo/topics/automation/automation_automation_modes_c.html,
  https://www.steinberg.help/r/cubase-pro/15.0/en/cubase_nuendo/topics/automation/automation_touch_c.html,
  https://www.steinberg.help/r/cubase-pro/15.0/en/cubase_nuendo/topics/automation/automation_trim_c.html
- [S7] J. O. Smith III, *Physical Audio Signal Processing*, "Linear Interpolation" and "Delay-Line
  Interpolation" (linear interpolation; zipper noise from unsmoothed changes).
  https://ccrma.stanford.edu/~jos/pasp/Linear_Interpolation.html,
  https://ccrma.stanford.edu/~jos/pasp/Delay_Line_Interpolation.html
- [S8] S. J. Orfanidis, *Introduction to Signal Processing*, §8.1.3 "Wavetable Generators" (linear
  interpolation between table entries, Eq. 8.1.48) and §8.3.1 "Noise Reduction Filters" (the
  first-order exponential smoother, Example 8.3.1). Author copy:
  https://eceweb1.rutgers.edu/~orfanidi/intro2sp/

[S7] and [S8] support the interpolation law of A1.4: linear interpolation between samples of a slowly
varying control is cheap and keeps the change smooth when the control's bandwidth is small against
the update rate.
