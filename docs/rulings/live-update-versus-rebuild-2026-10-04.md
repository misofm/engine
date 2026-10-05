# Live update versus plan rebuild: owner ruling of 2026-10-04

Follows decision 13 (`submix-strips-sends-and-vca-2026-10-02.md`), recorded from the owner's
approval on 2026-10-04, quoted below. Each point names whose authority it carries, on decision 13's
model.

## Decision 14: when a change is a live update and when it is a plan rebuild

**Context.** The engine changes a running mix in one of two ways. This ruling names them:

- **Live update.** A value reaches the running plan through a bounded live-control queue and is
  applied at a block boundary, ramped where the value has a ramp. No plan is compiled and the audio
  thread allocates nothing. On `main` these are the browser's 48-byte command records, kinds 1-6
  and 9-17 (`hosts/host-web/src/lib.rs:827-939`; kinds 7 and 8 arm observation taps and change no
  audio).
- **Plan rebuild.** The control plane compiles and validates a replacement `PreparedRenderPlan`
  and transfers ownership at a block boundary. The displaced plan goes to the bounded retirement
  queue (`AGENTS.md`, "Approved audio architecture"). On `main` the C ABI rebuilds the plan for
  every committed `SESSION_TRANSACTION_APPLY` (`crates/capi/src/runtime/control.rs:718` onward).
  The browser cannot rebuild in place; it boots a new engine instead (see "Each host on `main`"
  below).

Until now each feature decided for itself which of its values were live: D2 for the strip input
section (`builtins-input-liveness-d2.md` and its #808 amendment), each effect descriptor through its
`automation_rate`, and decision 13 for sends, output routes and VCAs. No one rule covered them all.

**The owner's words.** The assistant (root) proposed this guideline, quoted verbatim:

> 1. A change that needs new memory, a new graph, a new processing order or a new latency uses a
>    plan rebuild.
> 2. A value change is live when all of these are true: the plan already has a slot for the value;
>    the engine can make the change smooth without new memory; the extra render cost is small.
> 3. A value stays prepared (rebuild-only) when a live change always causes a glitch, or when a
>    constant value enables an important optimisation. In each case the ruling must give the
>    reason.
> 4. A value is automatable only if it is live.

The owner's reply on 2026-10-04, verbatim: "Yes, write the ruling and file it."

In the same conversation the owner also agreed to the four points below. They are root's summary
of that conversation; the owner's own words for them are not quoted, and the reasons spelled out in
C2 are root's.

- **C1. One edit API.** Callers see one edit API on every host. The engine decides, inside, whether
  an edit is a live update or a plan rebuild.
- **C2. Values stay live after swaps become seamless.** A value change stays a live update even
  once plan swaps are seamless, for four reasons:
  - a ramp is smooth, where a swap is a cut at a block boundary;
  - latency: a live update applies at the next block, while a rebuild first compiles a plan;
  - a continuous gesture, such as a fader drag, makes many changes a second;
  - every swap puts a plan into the bounded retirement queue, and a full queue defers the swap.
- **C3. Why structure needs a rebuild.** A structural change needs a plan rebuild because the audio
  thread must not allocate.
- **C4. Names.** The two mechanisms are called "live update" and "plan rebuild", not "fast path".

**Authority.**

- **Owner decision:**
  - the four-point rule, approved as quoted above;
  - C1-C4, agreed by the owner (root's summary).
- **Root's application, subject to owner review:**
  - the classification below;
  - the reason it gives for each value that stays prepared, except where it cites the reason
    from an earlier ruling;
  - the follow-ups.

  Root checked each row against `main` at `54b0a1bf8`. A fresh adversarial verifier then checked
  the record (see "Verification" below).
- **No code changes.** Where `main` breaks the rule, this record names a follow-up. Behaviour on
  `main` stays as it is until a filed issue changes it.

Ruling:

- **The rule** (owner decision): points 1-4 as quoted, with C1-C4.
- **Reading the rule** (root's reading, subject to owner review):
  - **A slot** is storage the prepared plan already holds for the value: a lane, a queue, a mirror
    or a ramp.
  - **New memory** is any allocation the audio thread would need. This includes the browser's
    command admission, which runs on the audio thread, in the AudioWorklet's port handler
    (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:266`, `:1554`).
  - **"Small" render cost** is not quantified by the owner. A row that was live before this ruling
    inherits the cost acceptance of the ruling that made it live (D2, the #808 amendment, decision
    12 or decision 13). A row this ruling newly classifies live (F3) states its cost.
- **Rule 4 is "only if", not "if and only if".** A live value need not be an automation target.
  Solo, sends and VCAs are live and have no automation target. D2's "automatable iff live" for the
  strip input parameters still holds: the automatable builtin rows are exactly its live rows
  (`BUILTIN_AUTOMATION_TARGETS`, `crates/session/src/validate.rs:823-838`). Stored automation
  renders nothing yet on any host (#1058), so rule 4 constrains the grammar and the contract today.
- **A value that stays prepared states its reason** (rule 3): a glitch, an optimisation, or rule 1
  (new memory, graph, order or latency), and the evidence that would reopen it. A value with no
  such reason that meets rule 2 is a follow-up (below), not an exception.

### Classification of current values and structural actions

**Columns.**

- **Class:** "live" means a live update. "Rebuild" means a plan rebuild under rule 1. "Prepared"
  means rebuild-only under rule 3.
- **Browser** and **C ABI:** what each host does on `main`.
- On the browser, "new engine" means that the change takes a new engine boot (see "Each host on
  `main`").
- On the C ABI, "rebuild" means that every committed transaction replaces the plan, as it does for
  every edit on `main`.
- The C ABI column records `main` at `54b0a1bf8`. Since then #1053's slices made several of its
  rows live; see "Factual amendment: the C ABI after PR #1298 and PR #1299" below, which supersedes
  that column where they differ.

**Strip builtins.** These apply to every strip, tracks and submixes alike (decision 13 O3 (b) and
Q5).

| Value | Class | Reason | Browser | C ABI |
|---|---|---|---|---|
| `fader_db` (id 5) | live | Lane and linear-N ramp exist (`BlockTarget`, `crates/builtins/src/lib.rs:533-548`). | live, kind 3 (composed with VCA offsets) | rebuild; live after #1053 (tracks), #1225 (submixes), #1247 (sessions with a VCA) |
| `mute` (id 6) | live | A retarget of the same gain to zero over the fader ramp (`lib.rs:526-532`). | live, kind 4 (composed with solo and VCA mute) | as `fader_db`; the mute of a strip that a following send reads stays a rebuild until #1226 (#1053 A2 D1) |
| `pan` (id 12), `matrix_*` (ids 7-10) | live | Live 2x2 lane and ramp. | live, kinds 1 and 2 | as `fader_db` |
| `trim_db` (id 2), `polarity_invert` (id 1) | live | D2: polarity is the sign of the trim coefficient, on the same ramp. | live, kinds 10 and 11 | rebuild; no filed issue (follow-up F5) |
| `hpf_hz` (id 3), `lpf_hz` (id 4) | live | #808 amendment: prepared targets designed off render, then a fixed 64-update coefficient ramp. | live, kind 12 | rebuild; no filed issue (F5) |
| `delay_samples` (id 11) | rebuild, and prepared | **Rule 1, new memory:** the line is sized to the declared delay, `sum(delay_samples) * 4` bytes (`crates/graph-compiler/src/estimate.rs:38-41`), and a strip whose delay is 0 has no line at all (`crates/graph-compiler/src/compile.rs:495-512`). **Rule 3, glitch** (D2 phase 2; `crates/builtins/src/lib.rs:627-633`): changing a delay length mid-render re-times the ring. | new engine | rebuild |
| solo | live | Not a session value. It composes strip mutes and moves no render state (`COMMAND_SOLO`, `lib.rs:854-869`). | live, kind 9, tracks only; a submix is solo-safe | no solo; the app composes mutes in one transaction (#1053, out of scope), a rebuild today and live after #1053 |

**Reopening `delay_samples`** (the D2 condition stands): a re-timing that does not glitch. One way
is a crossfaded dual read, as `miso.delay` does for its time parameter over a two-second ring it
sizes at preparation (`crates/delay/src/lib.rs:425-438`). It would read from a line sized to the
48,000-sample maximum, and that memory would have to be accepted.

**Effects.** These rows apply to console slots and inserts.

| Value or action | Class | Reason | Browser | C ABI |
|---|---|---|---|---|
| A parameter whose descriptor `automation_rate` is `Block` | live | Effect control lane, with the descriptor's linear ramp: 64 updates, or 128 for the delay's time. EQ parameters ride prepared targets. | live, kind 5 | rebuild; no filed issue (F5) |
| A parameter whose `automation_rate` is `None` | prepared | Per row of the next table. | refused, `UNSUPPORTED_KIND` (`lib.rs:4513-4518`) | rebuild |
| Bypass | live | Per-lane `BypassShunt`; latency is always paid (decision 12). | live, kind 6, except one case: lifting a session bypass on the delay or the multiband compressor is admitted and renders nothing; that bypass stays prepared for recorded correctness reasons, which rule 3 does not name (F4) | rebuild; no filed issue (F5) |
| Console slot identity, quality, link mode (`SetConsole`) | rebuild; link mode also prepared | **Identity** and **quality** need new effect state, and quality has its own latency row. **Link mode** is prepared bank metadata (part of `EffectProgramKey`, `crates/effect-contract/src/lib.rs:1179`) with no live slot, and a constant mode selects the compressor's dual-mono settled path (`crates/compressor/src/kernel.rs:624-640`; rule 3, optimisation). #1236 L1 makes link mode per-lane state (see "Notes for open briefs"). | new engine | rebuild |
| Insert identity, quality, link mode or sidechain; add, remove or reorder an insert | rebuild | New effect state, a new processing order or a new graph edge; latency and PDC can change. | new engine | rebuild |

**Launch effects, by descriptor.** The published metadata
(`sdk/assets/miso-engine-v1-parameter-metadata.json`, generated from the descriptors) agrees with
every row. The browser's comments at `hosts/host-web/src/lib.rs:1041-1043` and `:4513-4514` say
that no launch effect declares `AutomationRate::None`; they are stale, since four effects do.

| Effect | Live (`Block`) | Prepared (`None`), with its reason |
|---|---|---|
| `miso.compressor` | all 7 | none |
| `miso.delay` | all 5 (the time parameter ramps over 128 updates, with a crossfade) | none |
| `miso.soft-clip` | all 3 | none |
| `miso.transient-shaper` | all 3 | none |
| `miso.gate-expander` | threshold, ratio, range, hysteresis (ids 1-4) | attack, hold, release (ids 5-7). **No reason is recorded** (see the deleted spec, `git show 210f4d3e8^:.github/ISSUE_SPECS/014-gate-expander.md`). The compressor's attack and release are live. Follow-up F2. |
| `miso.multiband-compressor` | ids 3-12 | crossover (id 1). **No reason is recorded:** the deleted spec 018 says the crossover coefficients "are prepared, never automated" and does not say why. Follow-up F2. |
| `miso.parametric-eq` | each band's frequency, gain, Q and shelf slope; HPF and LPF enabled, frequency and Q (all through prepared targets) | each band's `enabled` and `kind` (ids 1, 2, 17, 18, 33, 34, 49, 50). **No reason is recorded**, and `apply_target_lane` refuses a target whose `enabled` or `kind` differs from the prepared value (`crates/parametric-eq/src/lib.rs:2815-2829`). HPF and LPF `enabled` are live, on the same prepared-target path. Follow-up F2. |
| `miso.true-peak-limiter` | ceiling, release | lookahead (id 3). **Rule 3, glitch** (root's reading; the deleted spec 016 says only "preparation/state only"). The lookahead sets the window `Wb` of the box ramp and of the sliding minimum (`crates/true-peak-limiter/src/lib.rs:11-24`). The running box sum, the van Herk phase, and the proof that `g[n] <= r[n-N]`, which keeps the output under the ceiling, all hold for one window. The plan's rings cannot rebuild that state for another window: the van Herk pass overwrites the raw required gains with suffix minima of the old window (`lib.rs:470-471`, `:1610-1620`), so a change that keeps the proof needs more history (rule 1) or a reset. A constant window is also what lets a bank take the uniform body (`lanes_uniform`, `:1507-1510`; rule 3, optimisation). Latency is fixed at `N + 6` (`:11`, `:242`), so a lookahead change moves no latency. **Reopens on:** a window transition that keeps the proof within the plan's memory, with a test that no sample exceeds the ceiling across a change. |

**Routes.**

| Value or action | Class | Reason | Browser | C ABI |
|---|---|---|---|---|
| A send into a bus (route into a submix): `gain_db`, `mute`, `channel_matrix` | live | Per-route lane and the indexed ramp (decision 13 P5; `attach_route_controls`, `crates/graph/src/lib.rs:1635-1645`). The extra cost is that a live route never folds (DESIGN R9). That cost has not been measured on a session that folds: #1229's fixture had zero static folds. | live, kinds 13-15 | rebuild; live after #1225 |
| `follows_mute` (route into a submix) | rebuild today; **rule 2 classifies it live** | Decision 13's DESIGN 5.7 classifies a `follows_mute` change as structural, a planner decision with no recorded reason (`docs/handoffs/submix-sends-2026-10-02/DESIGN.md:620-626`). The mirror entry exists and is "Fixed for the plan" (`crates/host-core/src/live_route_state.rs:46`). A toggle would only change the route's gated coefficients, which the existing route ramp carries. Follow-up F3. | new engine | rebuild |
| A route into the output: `gain_db`, `mute`, `channel_matrix` | prepared | **Rule 3, optimisation** (decision 13, DESIGN 5.7, R7 and O9). Routes into the output keep the single-master route fold (#218, #916), and a live route never folds (VERIFY-2 M3). Strip faders do the live job. `follows_mute` is always `false` here (P11). **Reopens on:** O9, an owner product request, together with O4, live route lanes in the epilogue fold; or a real-path measurement showing that losing the fold costs little. #1229 does not answer this, because its fixture folds nothing. | new engine | rebuild |
| Route source, destination or tap; adding or removing a route | rebuild | A new graph edge. Tap and destination set PDC and the topological order. | new engine | rebuild |

**VCA groups.**

| Value or action | Class | Reason | Browser | C ABI |
|---|---|---|---|---|
| VCA fader offset and mute | live | A VCA has no audio path. Admission composes it into each member's existing fader and mute lanes (`COMMAND_VCA_FADER_DB`, `COMMAND_VCA_MUTE`, `lib.rs:917-939`). | live, kinds 16 and 17 | applied at preparation (#1242); every edit of a session with a VCA is a rebuild until #1247 |
| Membership; adding or removing a VCA | rebuild | **Browser: rule 1, new memory.** The browser sizes its admission scratch (`boxed_command_staging` and `command_staging_count`, `2 * vca_reached_strips` records, `hosts/host-web/src/lib.rs:7265-7313`) and its VCA reach state at boot, within the bounds of decision 13 V-Q3, and admission runs on the audio thread. **C ABI:** admission runs on the control thread and render holds no VCA state, so rule 1's memory reason does not apply there. #1247 D1 keeps membership structural, a planner decision whose reason is reach drift, not rule 1; under this rule it is a candidate for review (F9). | new engine | rebuild |

**Structure and shape.**

| Value or action | Class | Reason | Browser | C ABI |
|---|---|---|---|---|
| Adding or removing a track, submix or output | rebuild | New strip state, banks and graph nodes. | new engine | rebuild |
| Model-only edits: session ID, render and output profile IDs (`SetSessionId`, `SetRenderProfile`, `SetOutputProfile`), and the automation edits `0600`-`0603` while nothing renders stored automation (#1058) | neither (root's reading) | They change no render state, so rule 1 asks no rebuild and there is no value to update live. Under C1 such an edit should commit without a plan rebuild. Once #1058 renders stored automation, the automation edits need their own classification. | not editable in place (new engine) | rebuild today (`crates/capi/src/runtime/tests.rs:1259-1270` drives `SetSessionId` as a structural transaction); follow-up F8 |
| Source mapping (`source_id`, left and right source channel); source content | rebuild, and prepared | **Rule 1:** the mapping binds a source ring to the strip's `Input` node at preparation (`crates/host-core/src/prepare.rs:873-892`). **Rule 3, optimisation:** the mono collapse is decided from the mapping at preparation (`track_mono_source`, `crates/builtins-compiler/src/lib.rs:3815-3820`). | new engine | rebuild |
| Sample rate | rebuild | Every coefficient is designed at the rate, and delay and PDC lines are counted in samples. There is no implicit SRC. The browser's `AudioContext` rate is fixed at construction. | new engine and a new `AudioContext` | rebuild at most; the host's device must match |
| Quantum | rebuild | Every arena buffer is sized by the quantum. | new engine (the render quantum is fixed for the `AudioContext`) | rebuild at most |

### Each host on `main`

- **Browser.**
  - **Live updates:** kinds 1-6 and 9-17 above. They exist only in a plan prepared with live
    controls, which are opt-in at boot; a fan's control-free playback keeps every value prepared
    (decision 13, DESIGN 3.1).
  - **Rebuilds:** there is no in-place plan rebuild. A structural change boots a new engine: SDK
    `createEngine`, or the headless `loadSession`, which reboots the same instance
    (`sdk/src/headless/engine.ts:332`). Effect state and source delivery start again.
- **C ABI.**
  - **Live updates:** none on `main`. The C ABI requests no live controls, and `live_builtin_delta`
    does not exist yet.
  - **Rebuilds:** every committed transaction is a plan rebuild.
    - Source rings reset at the replacement boundary
      (`StructuralSourceStatePolicy::ResetAtReplacementBoundary`,
      `crates/capi/src/runtime/control.rs:46-53`).
    - The replacement's effect state starts from preparation (#1053, context).
    - Decision 13's DESIGN R7 records a silent block.
  - **Issues that bring live updates:**
    - #1053: fader, mute and pan or matrix on tracks;
    - #1225: submix strips and sends;
    - #1226: sends that follow their source's mute;
    - #1247: VCA edits and member faders in sessions with a VCA.
  - **Not filed:** effect parameters and bypass (#1053's "L2") and EQ targets with input trim and
    polarity (#1053's "L3"). Follow-up F5.
  - **`AUTOMATION_ENQUEUE`** is admitted and acked for `Block`-rate parameters but reaches no PCM on
    any host (`docs/CONTROL_PROTOCOL_SEMANTICS.md:15`; decision 13's DESIGN R8). It refuses
    `AutomationRate::None` (`crates/protocol/src/controller.rs:3347-3351`), which agrees with
    rule 4. Refusing it as unavailable is audit row IO-5 of the open #349 tracker.
- **Known gap: plan swaps are not seamless on either host.** Under C2, value changes stay live
  even after swaps become seamless, so the gap matters only for structural edits.

### Factual amendment: the C ABI after PR #1298 and PR #1299

Recorded on 2026-10-04 after #1053's live updates (PR #1298) and phase 1 of *Swap a rebuilt plan
without an audio gap* (#1269, PR #1299) merged (`d2fe0555a`). It corrects what the record says
`main` does on the C ABI. It is not a new owner decision: no rule, class or follow-up's authority
changes. The C ABI bullets above, and the C ABI column of the tables, describe `main` at
`54b0a1bf8`; where they differ, this section holds (the contract text is the header,
`crates/capi/include/miso_engine_v1.h`, "Live edits" and "Sources across a structural
transaction").

- **Live updates on the C ABI.** A committed transaction whose changes are all live is applied to
  the running plan, with no replacement:
  - a track's fader level, mute, and pan or matrix values (#1253-#1258);
  - a changed value of a track effect parameter whose `automation_rate` is `Block`, on a console
    slot or an insert (#1263, #1264), the parametric EQ's live parameters included, through
    prepared targets (#1265);
  - a track effect's bypass (#1266), except the delay's and the multiband compressor's, which
    rebuild (F4's prepared bypass).
- **Model-only edits commit without a rebuild** (#1260): a transaction that changes only the
  session ID, a profile's ID or the stored automation table replaces no plan. F8 is closed.
- **Still rebuilds on the C ABI:**
  - input `trim_db` and `polarity_invert` (#1261) and input `hpf_hz` and `lpf_hz` (#1262), both
    waiting for the owner's answer to #1053's Q4 (an infinite tail for live input filters);
  - every submix-strip value (#1225, #1267), any edit of a session that declares a VCA (#1247),
    and the mute of a track that a `follows_mute` send reads (#1226);
  - every structural row above.
- **A structural rebuild no longer resets source rings** (#1273). `StructuralSourceStatePolicy`
  is deleted. The replacement keeps every source the transaction left unchanged playing, with its
  ring, generation and read position; the host neither seeks nor refills it. A source the
  transaction added or changed starts in a fresh ring at generation 1, frame 0; the host places it
  in time with `miso_engine_v1_source_seek_at` (#1274, #1275). The swap block is no longer silent:
  the predecessor hands over at the swap block (#1270), and the strip input sections carry their
  state across it (#1276). The replacement's fader ramps and effect state still start from
  preparation; #1277-#1284 carry them.
- **F5** is filed: effect parameters and bypass as #1263-#1266 (closed), the input trim and
  polarity as #1261 and the input HPF and LPF as #1262 (open, waiting for Q4).

### Follow-ups: where `main` breaks the rule

**Answered by decision 15** (`live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`,
root decisions of 2026-10-05 under the owner's explicit delegation). #1053's Q3 (report the edit
path) is answered by D15-3 and D15-17, and its Q4 (an infinite tail for live input filters) by
D15-4: a bounded tail, never `Infinite`. The findings below are answered there: F1 by D15-13 E1, F2
by E2, F3 by D15-6, F4 by E4 (a correctness reason may keep a value prepared, recorded with its
reopening condition; the delay and multiband compressor gain live bypass shunts), F6 by D15-11, F7
by D15-1 and E5, and F9 by D15-6 (VCA membership is live on the C ABI). The classification row "routes into the
output stay prepared" is narrowed by D15-9: a route whose tap precedes its strip's fader gets a
live lane on every plan (#1391). The text below is kept as
the record of 2026-10-04.

Each follow-up is root's finding, subject to owner review. None is filed by this record, and no
code changed.

- **F1, rule 4: automation can target a prepared-only effect parameter.**
  - **The gap.** `validate_automation` (`crates/session/src/validate.rs:890` onward) checks an
    `inserts` or `console` target only for a declared `(parameter_id, channel)`. It does not check
    the descriptor's `automation_rate`.
  - **Probe.** A probe added an automation target to
    `fixtures/session/v1/observation-frame-shape.json`, naming its EQ's `band-1-enabled`
    (`AutomationRate::None`). It passed all five stages of
    `cargo run -p session-validator -- validate`. A `builtins` target on `delay_samples` is refused
    at stage 2 with `parameter ID is not an automatable builtin parameter`.
  - **Impact.** Nothing renders stored automation yet (#1058), so no audio is wrong today.
  - **The SDK too.** The SDK builder's `resolveAutomationTarget`
    (`sdk/src/core/session.ts:1405-1477`) refuses a builtin row that is not `blockTarget`, but never
    checks an effect row's `automatable` or `automationRate`.
  - **Fix.** A crate that sees both the session and the descriptors must refuse the target; the
    `session` crate may not depend on `effect-contract` or `effect-compiler`
    (`scripts/check-effect-runtime-policy.sh:14`). The SDK's `resolveAutomationTarget` must refuse
    an effect row with `automatable: false` too. This needs to land before #1058 renders effect
    automation.
- **F2, rule 3: prepared effect parameters with no recorded reason.**
  - **The parameters:**
    - the gate-expander's attack, hold and release;
    - the multiband compressor's crossover;
    - the parametric EQ's band `enabled` and `kind`.
  - **What the fix must do.** For each one, either record a glitch or optimisation reason, with
    evidence, or make it live. Two paths already exist: the compressor's live attack and release,
    and the EQ's prepared-target path, which already carries HPF and LPF `enabled`.
  - **Stale comments.** The same follow-up corrects the browser comments named above.
- **F3, rule 2: `follows_mute` should be live.** It meets all three conditions of rule 2:
  - the slot is the live-route mirror entry;
  - the existing route ramp makes the change smooth, with no new memory;
  - the extra render cost is nil, since the route is already a live route.

  Decision 13's DESIGN 5.7 made it structural without a reason. Make it live (browser; C ABI after
  #1226), or record a reason.
- **F4, C1 and the acked-batch question: a session bypass on the delay or the multiband compressor
  cannot be lifted live.**
  - **The gap.** Such a bypass stays a prepared bypass (`NEVER_BANKED_EFFECTS`,
    `PREPARED_BYPASS_EFFECTS`, `crates/effect-compiler/src/prepare.rs:244-268`). A live command
    that lifts it is admitted and renders nothing different (`COMMAND_EFFECT_BYPASS`,
    `hosts/host-web/src/lib.rs:836-844`). The edit is acked but never heard: it is neither a live
    update nor a rebuild.
  - **Why it is prepared.** The recorded reasons are correctness reasons: for the delay, exactness
    of its D7 block check (`prepare.rs:237-240`); for the multiband compressor, decision 12's rule
    that banking never couples lanes' bits (`prepare.rs:247-259`). Rule 3 names only a glitch or an
    optimisation, so whether a correctness reason may keep a value prepared is an owner question.
  - **The fix.** Make the edit a rebuild, or give these two effects a live shunt. For the
    multiband compressor, the slice after #1069 that gives it per-lane D7 recovery removes it from
    `PREPARED_BYPASS_EFFECTS` (`prepare.rs:256-259`). The delay has no owner.
- **F5, C1 and rule 2 on the C ABI: unfiled live updates.**
  - **The values:** effect parameters, effect bypass, `trim_db`, `polarity_invert`, `hpf_hz` and
    `lpf_hz`. They are live updates by this rule.
  - **The gap.** On the C ABI no implementation issue owns them. #1053 names its unfiled successors
    L2 (effect parameters and bypass) and L3 (EQ prepared targets, input trim and polarity), and L3
    does not name the input HPF and LPF. #1020, the open research issue, asked the question.
  - **The fix.** File them, the input HPF and LPF included.
- **F6, C1: the browser has no single edit API.** A browser caller picks a live command or a new
  engine itself; the engine does not choose. Decision 2 of `engine-footprint-2026-09-28.md` makes
  the core engine own the current, edited session on every platform, and #1057 researches it. No
  implementation issue owns the browser's edit path yet.
- **F7, rule 2's "smooth": unmeasured.**
  - **The concern.** The bypass shunt switches a lane between dry and wet for a whole block, with
    no crossfade (`crates/effect-contract/src/live.rs:820-842`). The wet path keeps running, so the
    effect's state is continuous. The output still steps by `wet - dry` at the switch.
  - **What decides it.** A measurement of that step settles whether a crossfade is needed. #1053
    A2 D3 measured the analogous step for a mute. Until that measurement, the bypass step is a
    possible gap, not a finding.
  - **Ramp lengths.** The ramp of every live builtin, send and VCA command is chosen by the caller
    (decision 13's DESIGN P6), and the SDK defaults it to 0, a step
    (`sdk/src/core/live-controls.ts:298-300`, `:416`). #1053 A2 D3 measured a step on a mute as a
    click. So whether those live updates are smooth depends on the caller today. Decision 1 of
    `engine-footprint-2026-09-28.md` makes ramp lengths session settings with researched defaults,
    owned by #1054 and #1055.
- **F8, C1: model-only edits rebuild the plan on the C ABI.** The edits in the "Model-only edits"
  row change no render state, yet each is a full plan rebuild today, with the source-ring reset
  and the silent block. #1053's classifier is the natural place to commit them without a rebuild.
  **Closed by #1260** (see the factual amendment): they now commit with no replacement, and a
  structural rebuild no longer resets source rings or renders a silent block (#1270, #1273).
- **F9, rule 1: VCA membership on the C ABI.** #1247 D1 keeps membership structural for reach
  drift, a planner decision. Under this rule's reading of new memory, membership on the C ABI needs
  none, so #1247's brief should give a rule 3 reason or make it live.

### Notes for open briefs

- **#1236, a per-strip console link mode.** Its frozen D2 says that a per-strip link-mode change
  "is structural (a recompile), never a live control", and gives no reason. Once its L1 makes link
  mode per-lane state, that change needs no new memory. Under rule 3, the brief must then give a
  reason or make the change live. A candidate reason is an optimisation: the cheaper all-`dual_mono`
  path (`crates/compressor/src/kernel.rs:624`).
- **#1053 A2 D1** (a switch between `Pan` and `Matrix` is live, and so is a change that only
  alters smoothing) agrees with this rule.
- **#1247 D1** (VCA membership is structural) needs a reason under this rule on the C ABI (F9).

### Relation to earlier rulings

- **`AGENTS.md`.** Rule 1 restates "Every structural control-plane mutation produces and validates
  a replacement plan", and C3 gives the reason for it. `AGENTS.md` gains one sentence that points
  here.
- **D2.** This ruling classifies the strip input rows exactly as D2 and its #808 amendment do. D2's
  reasons and reopening conditions for `delay_samples` and builtin automation stand. HPF and LPF
  were reopened and made live by the #808 amendment. Root's reading: rule 2 now states the
  liveness test that D2 wrote as "live iff its declick story is the existing linear-gain law or
  cheaper", which #808 had already relaxed.
- **`engine-footprint-2026-09-28.md`.** Decision 1 (ramp lengths are session settings with
  researched defaults, #1054 and #1055) bears on rule 2's "smooth" (F7). Decision 2 (the core
  engine owns the edited session on every platform, #1057) is the ground for C1 (F6).
- **Decision 12.**
  - A console slot's declaration (identity, quality, link mode) is session-level and needs a
    rebuild.
  - Per-lane bypass through the shunt is live.
  - A bypassed slot keeps its latency, so a bypass toggle changes no latency.
- **Decision 13.**
  - Sends are live.
  - Routes into the output stay prepared, for the fold reason it records (O9).
  - VCA offsets and mutes are live in the browser and composed at preparation everywhere.
  - Decision 13's DESIGN 5.7 classifies a `follows_mute` change as structural, a planner
    decision with no recorded reason; F3 asks for it to be revisited under rules 2 and 3.

### Verification

Root checked every row against `main` at `54b0a1bf8`. A fresh Opus 5.5 adversarial verifier checked
four things:

- that the owner's authority is stated accurately;
- that every row is correct against the code;
- that nothing contradicts `AGENTS.md`, D2, decision 12 or decision 13;
- that the follow-ups are correct.

**Verdict: PASS-WITH-FIXES**, with no BLOCKER and no MAJOR. It reproduced F1. Its fourteen MINOR
and nine NIT findings are folded in:

- the limiter lookahead reason (the rings cannot rebuild the window state);
- the policy F1 cites, and the SDK half of F1;
- the model-only edits row and F8;
- the correctness reason behind F4, as an owner question;
- the D2 relation for HPF and LPF;
- decision 13's DESIGN 5.7 on `follows_mute`;
- decisions 1 and 2 of 2026-09-28 (F6, F7);
- #1196's Q5 text (the local spec; its GitHub body is synchronized after the push);
- the console link-mode reason;
- the C ABI side of VCA membership (F9);
- the authority marker in `AGENTS.md`;
- the cost method for "small";
- `AUTOMATION_ENQUEUE`;
- F5's scope;
- the anchors and wording.

None was declined. It could not check the owner's conversation, the size of the bypass step, or the
limiter argument by test.
