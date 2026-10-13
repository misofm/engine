# One control abstraction, one core: owner decision of 2026-10-13

Decision 17. Follows decision 14 (`live-update-versus-rebuild-2026-10-04.md`) and decision 15
(`live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). Recorded against `main` at
`df585e503`. Authority labels follow decision 13's model.

## Decision 17: every value is one kind of control, owned by one core

**Context.** Each live value has its own code today: 15 to 30 files per value (the controller
audit of 2026-10-13, section 2). The browser has its own 17 command kinds and its own admission
(`hosts/host-web/src/lib.rs:837-949`); the C ABI has one generic entry
(`miso_engine_v1_submit_command`, `crates/capi/src/ffi.rs:685`) but a classifier that makes only
fader, pan and effect values live (`crates/host-core/src/live_delta.rs:215-300`). So the same value
behaves differently on the two hosts: send gain, send mute and matrix, VCA offset and mute, trim,
polarity, HPF, LPF and submix strips are live in the browser and rebuild on the C ABI; solo exists
only in the browser; a prepared effect parameter is refused in the browser and rebuilds on the
C ABI; the protocol's parameter list (`PARAMETER_METADATA_GET`, served by
`crates/host-core/src/control_provider.rs:306`) lists effect parameters only, and only on the
C ABI. Ardour, CLAP, VST3, Web Audio and LMMS all use one descriptor and one write path for every
value (the DAW research of 2026-10-13).

**The owner's words** (2026-10-13, verbatim):

> "I'm pretty certain that's better than building each knob separately which can result in drift.
> Remember that this audio engine is designed to be manipulated by agents so we can't have ABIs
> that can slightly drift. For stuff like controls, there should be a single abstraction that
> offers agents a standardized interface to control stuff with. Imagine if a console had different
> types of knobs for EQs and compressors, it would be a pretty bad user experience. While we don't
> exactly have to care about the physical or frontend shape of a control necessarily, we should
> take care to build a reliable/flexible/sensible abstraction that provides a way to manipulate a
> value. It should be self-describing so that a frontend can be built around it later that makes
> sense."

> "Functionality should NEVER be different across platforms. iPhone and Android should just be a
> thin client over the same internal code as other platforms. Thick core, thin clients please."

> "Build it out so that it's extremely straightforward and logical to add new features that
> require new controls in the future."

The owner approved building it now (2026-10-13).

**Authority.**

- **Owner decision:** one control abstraction for every value; self-describing; no drift between
  ABIs; thick core and thin clients, with no behaviour that differs between platforms; adding a
  control must be simple; build now.
- **Root's application, subject to owner review:** the design below (C1-C13). A fresh review
  approved it with the edits that this text includes.
- **No code changes.** This record changes no behaviour. The slices listed at the end deliver it.

### The design

**C1. Thick core, thin hosts.** One core owns every control rule: the control list, the
descriptors, validation, classification (live or rebuild), ramp resolution and derived values.
It is the new crate `controls` (`crates/controls`, lib `controls`), called by `host-core` and
`control-plane`, which both hosts call (D15-11, #1309). Lowering (turning a changed value into a
render record) lives in `host-core`, keyed by `Family`; `controls` never depends on
`builtins-compiler`. Preparation receives `controls::effective` output as an argument. A host
adapter keeps only transport: FFI pointer checks, frame staging, exports, the Worker and worklet
scripts, and render-half delivery of meters and observations. **A behaviour that exists on one
host only is a defect.** The SDK is a thin client: it encodes and decodes the core's bytes and
generates its types from the core's tables.

The descriptor vocabulary that effect and builtin rows must name (`RebuildReason`, `LiveRampRow`,
`Family`, `Taper`, `ControlKind`) lives in `effect_contract::control`, because `builtins` and the
effect crates sit below `controls`; `controls` re-exports it. `controls` depends on `session`,
`builtins`, `effect-contract` and `effect-compiler` (for `launch_native_effect_registry`), and on
nothing above them.

**C2. The control address.** Every control has one stable address, built from stable IDs, never
from indices:

| Field | Values |
|---|---|
| `owner` | `session` / `strip(id)` (a track or a submix; one entity namespace, `crates/session/src/validate.rs:137`) / `route(id)` / `vca(id)` |
| `section` | `own` / `builtins` / `console(slot_id)` / `inserts(effect_id)` (the existing `RackName` codes 4, 5, 2; `own` appends code 6) |
| `control` | `u32`: the builtin, route, VCA or session row ID, or the effect's parameter ID; `0` in an effect section is that effect's bypass (effect parameter IDs start at 1, `ParameterId::new(0)` is `None`, `crates/effect-contract/src/lib.rs:127`) |
| `lane` | `left` / `right` / `both` (the existing `ParameterChannel` codes) |

**The lane rule.** A row is either per-lane (its value is held per dual-mono lane: fader, trim,
an effect parameter, a VCA offset and mute) or shared (one value for the owner: a matrix
coefficient, `matrix_mode`, a route gain, bypass). The list shows a per-lane row as two rows, `left` and `right`,
and never as `both`; it shows a shared row as one row, `both`. On write, `both` on a per-lane row
is an alias that sets both lanes in one edit; `left` or `right` on a shared row is refused
(`control.lane`). For effect parameters whose session entry is stored as `both`, a write to one
lane splits the entry: the canonical model then holds an explicit `left` and an explicit `right`
entry (the written lane takes the new value, the other keeps the old value), and the `both` entry
is gone. A later write of `both` with equal lane values does not merge them back; the canonical
form keeps what the edits made, so a snapshot round-trips exactly.

The stored automation target `{entity_id, rack, effect_id, parameter_id, channel}`
(`crates/session/src/model.rs:980`) is already this shape for strips; C10 makes it the address.

**C3. The descriptor.** One type, `ControlDescriptor`, for every control:

- identity: address; stable machine `name` (`fader_db`, `threshold_db`); `display_name`,
  `display_unit`;
- value: `unit`; `kind` (continuous, toggle, enumeration); `minimum`, `maximum` (rate-resolved at
  the session rate), optional `disabled_value` (HPF and LPF `0` = off); `default`; `taper`
  (linear, logarithmic, exponential, decibel, stepped); `step` (lattice step and precision, and the
  named step ladder, as `effect_contract::ParameterLattice`); enumeration choices;
- motion: `ramp_row` (the `LiveRampRow`) or `fixed` (an effect's own coefficient ramp; the bypass
  crossfade, #1341);
- class: `live` / `rebuild_only(reason, reopen)` / `derived` (read-only, computed by C7);
- reason codes, all required for `rebuild_only`: decision 14 rule 1 (`new_memory`, `new_graph`,
  `processing_order`, `latency`), rule 3 (`glitch`, `optimisation`), D15-13 E4 (`correctness`),
  and `ruled_live_not_built(issue)` for a value that a ruling already makes live and whose issue has
  not landed (D15-13 E2: #1336, #1337, #1338, #1367); each such issue's last step flips its row to
  `live`;
- `scope`: `model` (in the session document) or `monitor` (solo: host-local, never saved, never
  advances the revision; #1057 item 8);
- `automatable`: **computed, not declared**: `class == live && scope == model && family != setting`
  (decision 14 rule 4 with D15's "automatable iff live");
- `family`: the render family of C6;
- `active`: false for a row that the owner's present shape does not use (the matrix rows of a strip
  in pan mode, and the reverse). The row set of a session changes only with its structure.

Rows come from one table per section, and nowhere else: the builtin rows are
`BUILTIN_PARAMETER_DESCRIPTORS` (`crates/builtins/src/lib.rs:462`) with the new fields added to
that table (not a second table); route, VCA, session and strip-own rows are new tables in
`controls`; effect rows are the effect descriptors (`crates/effect-contract/src/lib.rs:717`) with
one new field, `prepared_reason`, required when `automation_rate` is `None`. A row that `controls`
defines carries its model accessors (`get` and `set` function pointers), so the row is the only
place that binds a control to its model field; builtin rows bind through one `match` arm per
builtin ID in `controls`. A class may depend on the owner's shape, by a rule in the row: a route
into a submix is `live`; a route from `post_fader` or `post_pan` into the output is
`rebuild_only(optimisation)` (decision 14, routes table; #1391 changes the pre-fader taps).

**C4. One query: the control list.** Two new message kinds, `CONTROL_LIST_GET` and
`CONTROL_STATE_GET`, take the next free codes (`0x000c`, `0x000d`). `PARAMETER_METADATA_GET`
(`0x0004`) and `PARAMETER_STATE_GET` (`0x0005`) are removed: their codes are refused, never
reallocated. Contract identity stays V1; the Rust names are unversioned. Wire form, in the TLV
encoding of `docs/CONTROL_BTLV_V1.md`:

- **`structure_epoch`.** A `U64` that the core advances on every commit that changes the row set
  or the row order (any structural edit, and nothing else). Handles are valid only within one
  `structure_epoch`.
- **`CONTROL_LIST_GET` request:** `structure_epoch U64` (`0` for the first page), `after_handle
  U32`, `limit U16`, optional `owner` filter (`owner_kind U8`, `owner_id UTF8`). A non-zero
  `structure_epoch` that is not the current one is refused with `control.list.stale`; the caller
  starts again.
- **Response page:** `revision U64`, `structure_epoch U64`, `last_handle U32`, `eof BOOL`, then one
  `MESSAGE` per entry. An entry is a header row or a control row.
  - **Header row**, one per owner and one per section, before that owner's or section's control
    rows: `handle U32`, `row_kind U8` (`owner` or `section`), `address MESSAGE` (the control field
    absent), `kind U8` (track, submix, route, VCA, session; or builtins, console, inserts, own),
    `name UTF8` (the entity's or slot's display name where the session has one, else its ID),
    `effect UTF8 optional` (the effect identity of a console or insert section), and for a route
    owner `source UTF8`, `destination UTF8`, `tap U8`.
  - **Control row:** `handle U32`, `address MESSAGE` (`owner_kind U8`, `owner_id UTF8`,
    `section U8`, `slot_id UTF8`, `control U32`, `lane U8`), `name UTF8`, `display_name UTF8`,
    `display_unit UTF8`, `unit U8`, `kind U8`, `minimum F32`, `maximum F32`,
    `disabled_value F32 optional`, `default F32`, `taper U8`, `step F32`, `precision U8`,
    `ladder F32 array`, `choices MESSAGE array {value F32, label UTF8}`, `ramp_row U8 optional`
    (absent: fixed), `default_ramp_samples U32` (resolved for this session), `class U8`,
    `reason U8 optional`, `reopen UTF8 optional`, `scope U8`, `automatable BOOL`, `family U8`,
    `active BOOL`, `value F32` (committed value; for a derived row, the effective value; for solo,
    the overlay).
- **`CONTROL_STATE_GET`:** the same request; the page carries `revision`, `structure_epoch`,
  `last_handle`, `eof` and one `{handle U32, value F32}` pair per control row. It is the cheap
  poll for a client that already holds the list.
- **Order:** strips in `model.strips()` order, then routes, then VCAs, then session rows; within a
  strip: own, builtins, pre-insert console slots, inserts, post-insert console slots, each in
  declared order. Handles count from 1 in that order.
- **Hosts:** the C ABI sends the request through `miso_engine_v1_submit_command`; no new export.
  Browser and headless SDK: `engine.controls(filter?)` sends the same frame through the Worker's
  command message to the same crate function and decodes the same bytes.

**C5. One write path.** One transaction op, `SET_CONTROL` (new `SessionEdit::SetControl`, opcode
`0x0800`): `address MESSAGE`, `value F32`, `ramp_samples U32 optional` (absent: the session
default, resolved by `LiveRamps::resolve`, #1394 D5; `0` is a step). It addresses by stable
address, never by handle. It travels in `SESSION_TRANSACTION_APPLY` on both hosts (D15-11), with
the same atomicity, expected revision and D15-17 completion as every edit. A timed value is
automation (C10), not a field of this op: time-stamped records stay FIFO (D15-2). Edits of one
transaction apply in order to the running model. **One validator**, `controls::check` (the
descriptor-level check, pure and allocation-free) inside `controls::set` (check, then write the
model), refuses, before any write, with:

- `control.address.unknown`: no such owner, section, slot or row;
- `control.read_only`: a derived row;
- `control.inactive`: a row the owner's shape does not use (switch `matrix_mode` first, in the same
  transaction);
- `control.lane`: a `left` or `right` lane on a shared row;
- `control.domain`: not finite, outside the range, a toggle not exactly 0 or 1, not an
  enumeration choice, a rate-keyed frequency above its rate bound;
- `control.ramp.fixed`: an explicit `ramp_samples` on a row whose ramp is fixed (an ack must not
  hide an ignored field).

Values are not snapped to the step lattice (as today for session values). A value equal to the
committed value is accepted and changes nothing (#1315's rule: its commanded state holds).
The per-kind value ops are retired in slice S9: `SetTrackFader` (`0x020f`),
`SetTrackMatrixOrPan` (`0x0210`), the value use of `SetTrackBuiltins` (`0x0203`),
`SetEffectBypass` (`0x020a`), `UpsertEffectParam` and `RemoveEffectParam` (`0x020d`, `0x020e`),
the value use of `SetTrackConsole` (`0x0211`), `SetEffectLinkMode` (`0x020b`),
`SetRouteGainDb`, `SetRouteMute`, `SetRouteChannelMatrix`, `SetRouteFollowsMute`
(`0x0504`-`0x0507`), `SetVcaFader` (`0x0702`) and `SetControlSmoothing` (`0x0008`). Their codes are
refused and never reallocated. Structural ops stay: add, remove, reorder, identity, quality,
sidechain, source mapping, route topology, VCA membership, console slot declaration.

**C6. One classifier, a fixed set of render families.** The classifier (in `controls`) compares
the committed and the prospective model through the control list, not through ops:

1. Mask every control value (from the registry) and every input of the effective function (C7:
   VCA membership, `follows_mute`); if the rest of the canonical model differs, the transaction is
   a plan rebuild. This replaces the hand-written mask of `live_delta.rs:233-263`.
2. For each control whose value changed: `rebuild_only` makes the transaction a rebuild; a
   `setting` row (ramp lengths, a strip's pan window) produces no record; a `live` row goes to its
   family's one lowering function in `host-core`.
3. Recompute C7 before and after; every changed effective value goes to its family.
4. Each record takes `LiveRamps::resolve(row.ramp_row, edit ramp)`; a record that a model edit and
   an overlay change both produce takes the model edit's ramp (#1562, formerly #1382 D3).

The families are fixed and typed: **strip level** (fader, mute: `FaderCells`), **strip matrix**
(pan, matrix, `matrix_mode`), **strip input** (trim, polarity, HPF and LPF through prepared filter
targets designed off render), **route** (gain, mute, matrix, folded with the follow gate into the
route coefficients, `RouteControlProducer`), **effect** (parameters, bypass, EQ targets), and
**setting** (no render record). Render cells stay typed per family, banked and dual-mono; render
code, D15-2's latest-target cells, the `live_values_superseded` counter, the acked-batch rule (every
fallible check before the first cell write) and the watermark are unchanged.

**Live lanes on every plan.** The core attaches the live lanes that a session's `live` rows need
on every plan, on every host; no host chooses them. This supersedes the C ABI's lane set
(`C_ABI_LIVE_LANES`), the browser's boot word, #1523 and #1524. The slice that makes the switch
(S4b) records, once, the retained bytes the always-attached lanes add on a representative session
and their per-block render cost; this is evidence, not a gate.

**C7. One effective-value function.** `controls::effective(model, overlay) -> Effective` is pure.
Per strip and lane: `fader_db = clamp(own + sum of reaching VCA offsets)` and
`mute = own || any reaching VCA mute || solo term` (the solo term: some strip is soloed, this strip
is not, and it is not solo-safe). Per route and source lane: `gate = mute || (follows_mute && source lane's effective mute)`, with
its gain and matrix. It is one generic function over a read-only input view;
`controls::effective_into(&inputs, &mut Effective)` writes into caller-owned storage and never
allocates, so a host may call it where allocation is refused. Preparation receives its output as
an argument (it replaces `SessionModel::effective_strip_faders`, `crates/session/src/vca.rs:100`)
and the classifier diffs it (C6 step 3), so a VCA edit, a membership edit, a follow change and a
solo change all compose through the same code on both hosts. Its outputs are listed as `derived`
rows (`effective_fader_db`, `effective_mute` per strip lane; `effective_gate` per route and source lane).

**C8. Solo.** Solo is a strip-own control with `scope: monitor`, held in the core's overlay
(`LiveControlSoloState` moves behind `controls`) and written by `SET_CONTROL` on both hosts. A
monitor write is not a session mutation (the same class as observe). The reply carries
`path: monitor` and no revision. It applies by D15-2's convergence block, or at adoption when a
replacement plan is pending. A transaction that mixes it with model edits is all or nothing
(#1562 D4, formerly #1382 D3 and D4). The C ABI gains solo.

**C9. Observation is not a control.** Observe subscriptions and the meter lease are not values and
stay monitor operations (#1057 section 3.3). C13 still moves them into the core.

**C10. Automation uses the same address and descriptor.** A stored automation target is a control
address (adding route and VCA owners to the grammar); validation accepts only `automatable` rows.
An `AUTOMATION_ENQUEUE` batch carries a header table of addresses; its records index that table.
No handle crosses a revision. Each record is validated against its address's row. Rendering
automation is not in this decision (#1058's successors, and #1315 until then).

**C11. Unchanged.** Zero allocation, lock and syscall in render; transactional commit; D15-2
supersession semantics and its counter; the watermark and D15-17; decision 14's classification
of every existing value (the registry encodes it, it does not change it); dual-mono strips.

**C12. The pan and matrix mode.** `matrix_mode` (`pan` 0, `matrix` 1) is a live enumeration row,
so the switch is live (#1053 A2 D1). Switching to `matrix` seeds the matrix with the pan's exact
2x2 coefficients, so nothing moves; switching to `pan` sets each lane's pan to its default unless
the same transaction sets it. The strip's pan window (`smoothing_samples`) and the session's
`control_smoothing` keys are `setting` rows.

**C13. No browser-only feature.** The same rule as C1 for features that are not values: spectrum
capture, the track response, the response query, the meter lease and observation subscriptions
move into the core with one implementation, and each gets its C ABI form, so a mobile app has every
feature the browser has. Each feature has three parts: its logic moves into the core, it gets
its C ABI form (a protocol message through `miso_engine_v1_submit_command`, no new C export), and
the browser moves onto the same frames. The phase after S13 delivers them (slices S14 to S31). Eight
smaller one-host differences found by the same audit (plan latency and tail, the agent lattice,
source submit diagnostics, render-failure recovery, session admission caps, CPU telemetry, the SDK
builder's checks, canonical-PCM decode and content identity) are recorded for root to file as
their own issues under this rule.

### How to add a control

1. **An effect parameter.** Add a `ParameterDescriptor` row to the effect and its DSP. If its
   `automation_rate` is `None`, give `prepared_reason`. Regenerate the SDK
   (`cargo run -p parameter-metadata -- --write`). Nothing else: list, validator, classifier,
   ramps, both hosts, SDK types and automation follow from the row.
2. **A value of a strip, route, VCA or the session.** Three mechanical edits:
   1. the model field and its grammar: the field (`crates/session/src/model.rs`); its key and
      canonical walk event (`crates/session/src/visit.rs`); its JSON key and parse line
      (`crates/session/src/parse.rs`); its wire field spec, encode and decode lines
      (`crates/protocol/src/schema.rs`, `crates/protocol/src/session_wire.rs`) and the schema-hash
      re-pin; its setter in the SDK session builder (`sdk/src/core/session.ts`); and the key added
      to every existing document, because the V1 grammar has no optional key. Slice S13a adds a
      test that holds the session and wire field IDs equal; slice S13b generates the SDK key tables
      from the engine's walk and holds the JSON schema document's keys equal to it, so a missing
      property schema turns a test red. A single
      key table for the whole grammar was assessed and does not fit one slice (the value records
      are not flat; the wire codec is in another crate); it is not planned;
   2. one row in that section's table in `controls` (or in `BUILTIN_PARAMETER_DESCRIPTORS` with
      its accessor arm): ID, name, unit, domain, default, taper, step, ramp row, class with its
      reason, family, and the model accessor;
   3. the DSP, only if render must use the value, in its family's existing cell. Then regenerate
      the SDK. No protocol, host, classifier, ramp or SDK code.
3. **A value that is not live yet.** As 1 or 2, with `rebuild_only` and its reason and reopening
   condition. Making it live later changes the class in the row, after the render work.
4. **A derived value.** Add it to `controls::effective` and add a read-only row.
5. **A new render family** (a new kind of render cell). This needs its own issue under this
   decision: one cell type, one lowering function in `host-core` and one family code. It should be
   rare.

A test in `controls` checks the tables: every row has a unique address, every `rebuild_only` row
has a reason, every effect `None` row has `prepared_reason`, and no `automatable` row is outside
`live`.

### What this supersedes

- The per-kind value ops of C5, retired by slice S9.
- `PARAMETER_METADATA_GET` and `PARAMETER_STATE_GET`, by C4 (slice S3).
- The browser's command kinds 1-6 and 9-17 and its admission (`admit_commands_staged`), deleted by
  the browser slices (S10, S11), with the SDK's 48-byte record writer. Slice S7c first makes that
  admission call the core's validator, ramp resolution and effective function, so browser
  behaviour matches the core before the browser joins it.
- #1382 (now #1562) is amended in place: D2 (lowering 48-byte records in the Worker) goes, replaced by thin SDK
  builders of `SET_CONTROL`, as #1057's design note section 3.2 option A already chose; D3 (the
  solo overlay in the shared commit) moves to slice S7b. #1382 (now #1562) remains the slice that moves browser
  admission into the Worker (S10).
- The hand-written mask and the per-kind refusals of `classify_live_delta`, by C6, and its
  `LiveRebuild::Vca` early exit (VCAs are live through C7).
- Per-host live-lane requests (the C ABI's lane set, the browser's boot word) and #1523 and #1524,
  by C6's "live lanes on every plan".
- The open issues listed as superseded in the slice specs, closed as superseded when the slice
  that covers them merges.

### Relation to earlier rulings

- **Decision 14.** Unchanged rules; C3 records each row's class and reason in code, so the
  classification tables become the registry's test fixture. C1 completes rule C1 (one edit API).
- **Decision 15.** D15-1 (every live value ramps), D15-2, D15-11 (one control plane) and D15-17 are
  kept; C1 extends D15-11 from "one crate" to "one crate owns every control rule".
- **#1057 design note.** Section 3.2 option A (thin builders) and item 8 (solo is host-local) are
  kept; P1 (lane-granular setters) is met by `SET_CONTROL`, which is lane-granular by address.

### Slices

The slices are filed as issues; `docs/handoffs/decision-15-2026-10-05/STREAMS.md`, section
"Controls program", gives their order and file ownership.

| Issue | Label | Title |
|---|---|---|
| #1546 | S1 | Describe every strip, route, VCA and session value as one kind of control |
| #1547 | S1b | Compute every effective fader, mute and send gate with one function |
| #1548 | S7c | Make the browser check, ramp and compose live edits through the core |
| #1549 | S2 | List every effect parameter, bypass and link mode as a control |
| #1550 | S2b | Generate the SDK's control types from the registry |
| #1551 | S3 | Add the control-list and control-state messages to the protocol |
| #1552 | S3b | Serve the control list on the C ABI from the core |
| #1553 | S4a | Set any control with one transaction edit |
| #1554 | S4b | Classify every value edit through the control registry |
| #1555 | S4c | Let the core choose the live lanes of every C ABI plan |
| #1556 | S5 | Make trim, polarity and the input filters live on the C ABI |
| #1557 | S6 | Make send gain, mute and matrix live on the C ABI |
| #1558 | S7a | Make VCA moves, VCA membership and send follows live on the C ABI |
| #1559 | S7b | Solo a strip on the C ABI through the core's monitor overlay |
| #1560 | S8 | Prove that both hosts give the same result for the same control edits |
| #1561 | S9 | Set every value through one protocol op |
| #1562 | S10a | Admit browser live edits in the Worker through the committed model |
| #1563 | S11a | Send every SDK value edit as a SET_CONTROL transaction |
| #1564 | S11b | List and set any control from the SDK |
| #1565 | S10b | Delete the browser's own live-command path |
| #1566 | S12a | Store automation targets as control addresses |
| #1567 | S12b | Address enqueued automation by control address |
| #1568 | S13a | Keep the session JSON keys and the session wire field IDs equal |
| #1569 | S13b | Generate the SDK's session key tables from the engine's walk |
| #1570 | S14 | The browser serves every protocol command and event through the core |
| #1571 | S14b | Read capabilities, transport, telemetry, counters, diagnostics and events from the SDK |
| #1572 | S15 | Compute the response preview with one core function |
| #1573 | S16 | Get an effect or input-filter response preview on the C ABI |
| #1574 | S17 | Preview a response in the browser with the same protocol frame as the C ABI |
| #1575 | S18 | Preview the response of a session effect or input filters by its address |
| #1576 | S19 | Compose the track response and its members in the core |
| #1577 | S20a | Capture a strip's response owners after a render block in the core |
| #1578 | S20b | Get a track's response on the C ABI |
| #1579 | S21a | Read the browser track response through the same frames as the C ABI |
| #1580 | S21b | Delete the browser's own track-response capture |
| #1581 | S22 | Meter every strip and the output on every plan with one core selection |
| #1582 | S23 | Fold meter windows in one core delivery that the browser calls |
| #1583 | S24a | Configure meters by stable target and receive windowed output peaks on the C ABI |
| #1584 | S25 | Subscribe to an effect's observation tap with a transaction edit |
| #1585 | S26a | Publish each observation tap's arm state from render and return arm tags |
| #1586 | S26b | Read observation windows with one protocol command on every host |
| #1587 | S24b | Report strip peaks and gain reduction to C ABI apps from the core's meter fold |
| #1588 | S24c | Serve the same meter batches in the browser from its worklet fold |
| #1589 | S27a | Subscribe and read observations through the core's frames in the browser and the SDK |
| #1590 | S27b | Keep only handles and callbacks in the SDK and delete the browser's own observation exports |
| #1591 | S28 | Capture, select and analyze one spectrum window through one core service |
| #1592 | S29 | Start, select, read and stop the spectrum stream through one core state machine |
| #1593 | S30 | Keep the session's spectrum taps on every plan it prepares |
| #1594 | S30b | Change the spectrum taps at run time with SPECTRUM_CONFIGURE |
| #1595 | S31 | Read spectrum windows and streams on the C ABI |
| #1596 | S31b | Read spectrum in the browser and the SDK through the same frames as the C ABI |
| #1597 | S31c | Delete the browser's own spectrum exports, records and analysis Worker |
