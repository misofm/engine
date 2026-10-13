# List every effect parameter, bypass and link mode as a control

Slice S2 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C2
and C3). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review; no per-attempt verifier or mutation runs. The planner's
S2 is split in two: this slice (the rows) and #1550 (the generated SDK catalog).

## Product outcome

Every native effect parameter, every effect bypass and every insert's link mode is a control row
of the registry, with the same address, descriptor, class and validator as a fader. Every
parameter that is not live states why in its own descriptor row. Every row, builtin and own rows
included, gets a display name and a display unit, so a frontend can draw it.

## Context

- **Effect descriptors.** `ParameterDescriptor` (`crates/effect-contract/src/lib.rs:717-740`) has
  `id`, `display_name`, `display_unit`, `unit`, `domain`, `minimum`, `maximum`, `default_value`,
  `mapping`, `automation_rate`, `channel_policy`, `smoothing`, `smoothing_samples`, `readable`,
  `automatable`, `enum_choices` and `lattice`. `AutomationRate` is `Sample`, `Block`, `None`
  (`:413`). `validate_descriptor` (`:992`) checks every table; `parameter_automation_smoothing_valid`
  (`:469`) requires `None` to pair with `!automatable`; error codes are `DescriptorDiagnosticCode`
  (`:822`). About 27 `ParameterDescriptor {` literals exist (effect crates, their test helpers,
  `crates/effect-contract/src/{lib.rs,step.rs}` and tests, `crates/effect-compiler`,
  `crates/graph/src/lib.rs:7035`, `crates/conformance`).
- **The `None` parameters** (no launch parameter is `Sample`):
  - `miso.gate-expander` attack, hold, release, ids 5-7 (`crates/gate-expander/src/lib.rs:183`,
    `:196`, `:209`);
  - `miso.multiband-compressor` crossover, id 1 (`crates/multiband-compressor/src/lib.rs:188`);
  - `miso.true-peak-limiter` lookahead, id 3 (`crates/true-peak-limiter/src/lib.rs:208`);
  - `miso.parametric-eq` band `enabled` and `kind`, ids 1, 2, 17, 18, 33, 34, 49, 50 (the helper
    at `crates/parametric-eq/src/lib.rs:499`, rate at `:515-519`).
- **Their reasons.** Decision 14 records only the limiter's (rule 3, glitch, with a reopening
  condition; `docs/rulings/live-update-versus-rebuild-2026-10-04.md:150`); its F2 lists the others
  with no reason (`:281-289`). Decision 15 E2 makes them live: #1336 (gate), #1337 (EQ), #1338
  (crossover), and link mode: #1367 (multiband), #1236 with #1368-#1371 (console link mode). E4
  gives the delay and multiband compressor a live bypass shunt (#1339, #1340) and makes lifting
  their bypass a rebuild until then
  (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md:425-440`).
- **Bypass and link mode are not parameters.** Bypass is `Effect.bypass`
  (`crates/session/src/model.rs:597-598`); `lowers_session_bypass`
  (`crates/effect-compiler/src/prepare.rs:314`) is false for `miso.delay` and
  `miso.multiband-compressor`. Link mode is `Effect.link_mode` for an insert and
  `ConsoleSlot.link_mode` for a console slot (session-level, structural); the supported set is
  `EffectDescriptor.supported_link_modes` (`crates/effect-contract/src/lib.rs:796-821`). Decision
  14 classes link mode as prepared bank metadata, rule 3 optimisation (`:133` of the ruling).
  `SetEffectLinkMode` (`0x020b`) is one of the value ops #1561 retires, so the link mode needs a row.
- **Parameter storage.** `Effect.params` / `ConsoleEntry.params` hold `EffectParam { parameter_id,
  channel, unit, value }` (`crates/session/src/model.rs:650-660`), canonical by parameter ID then
  channel. `effect_compiler::resolve_initial_values` (`crates/effect-compiler/src/prepare.rs:1970`)
  resolves them per lane (an absent entry is the default). `launch_native_effect_registry`
  (`:215`) returns the process's shared registry.
- **Display strings.** Effect rows have them; `BuiltinParameterDescriptor`
  (`crates/builtins/src/lib.rs:239-255`) and #1546's own rows do not.

## Decisions frozen for this slice

- **D1. The reason field.** `ParameterDescriptor` gains `prepared_reason: Option<PreparedReason>`,
  where `effect_contract::control::PreparedReason { reason: RebuildReason, reopen: &'static str }`.
  `validate_descriptor` refuses a descriptor whose parameter has `automation_rate` `None` and no
  reason, or a reason with any other rate, with a new code `DescriptorDiagnosticCode::PreparedReason`
  (appended). The values:
  - limiter lookahead: `Glitch`, reopen "a window transition that keeps the proof within the plan's
    memory, with a test that no sample exceeds the ceiling across a change";
  - gate attack, hold, release: `RuledLiveNotBuilt { issue: 1336 }`;
  - EQ band `enabled` and `kind`: `RuledLiveNotBuilt { issue: 1337 }`;
  - multiband crossover: `RuledLiveNotBuilt { issue: 1338 }`.
  For a `RuledLiveNotBuilt` row, `reopen` names the issue that flips it.
- **D2. Effect rows.** For each strip, each console entry (section `Console(slot_id)`) and each
  insert (section `Inserts(effect_id)`) of a native effect in the launch registry, `controls::list`
  yields, in this order: the bypass row (control `0`), then one row per descriptor parameter in
  descriptor order (`PerLane`: `Left` then `Right`; `Shared`: `Both`), then the link-mode row
  (D4) on an insert. A `cid` effect or an unknown identity yields no rows. `controls` gains a
  dependency on `effect-compiler`.
- **D3. Effect row descriptors.** A parameter row: unit, minimum, maximum, default, choices and
  lattice from its descriptor; `kind` from `domain` (Continuous, Toggle for Boolean, Enumeration);
  `taper` from `mapping` (`Decibel` when the unit is dB and the mapping linear); family `Effect`;
  motion `Fixed` (the effect's own coefficient ramp); class `Live` for `Block`, else
  `RebuildOnly(prepared_reason)`. The bypass row: `Toggle`, shared, family `Effect`, motion `Fixed`
  (the bypass crossfade, #1341), class `Live`, except `miso.delay`
  (`RebuildOnly(RuledLiveNotBuilt { issue: 1339 })`) and `miso.multiband-compressor`
  (`RebuildOnly(RuledLiveNotBuilt { issue: 1340 })`). `check` uses
  `effect_contract::parameter_value_valid` semantics for the domain (one authority), so the row and
  the browser's effect admission agree.
- **D4. The link-mode row.** Control `LINK_MODE_CONTROL = 0x8000_0000` in an insert section, only
  for an effect whose `supported_link_modes` has more than one mode. `Enumeration`, shared, choices
  the supported modes with their `effect_contract::LinkMode` codes (`dual_mono` 1, `maximum` 2,
  `average` 3), family `Effect`, motion `Fixed`, class
  `RebuildOnly(RuledLiveNotBuilt { issue: 1367 })` for `miso.multiband-compressor` and
  `RebuildOnly(Optimisation)` for the others, reopen "decision 15 E2, #1368-#1371". The registry
  test refuses a descriptor parameter ID at or above `0x8000_0000`. A console slot's link mode is a
  slot declaration (structural) and has no row; a strip's override arrives with #1369.
- **D5. Writing an effect row.** `set` on a parameter row writes the strip's `params` and keeps
  them canonical (ID, then channel); each entry's `unit` is the one session validation requires for
  that parameter. Lanes (ruling C2):
  - a `Left` or `Right` write on a per-lane parameter stored as one `Both` entry splits it: an
    explicit `Left` and an explicit `Right` entry (the written lane takes the new value, the other
    keeps the old one), and the `Both` entry is removed;
  - a `Both` write on a per-lane parameter updates whichever entries exist (`Both`, or `Left` and
    `Right`), and adds a `Both` entry when there is none;
  - a write equal to the resolved committed value changes nothing (no split, no new entry);
  - a later `Both` write with equal lane values does not merge entries back.
  `get` returns the lane's resolved value (`resolve_initial_values`). A bypass write sets `bypass`;
  a link-mode write sets `link_mode`.
- **D6. Display strings.** `BuiltinParameterDescriptor` gains `display_name` and `display_unit`;
  #1546's own-row tables gain the same two columns; `ControlDescriptor` carries both for every row.
  Values: title-case names ("Fader", "Input Trim", "High-Pass", "Send Gain", "VCA Offset",
  "Fader Ramp" ...) and units ("dB", "Hz", "ms", "samples", "" for unitless). The registry test
  refuses an empty display name.
- **D7. No behaviour change.** No host calls the new rows yet. The metadata generator output is
  unchanged in this slice (#1550 emits the rows).

## Deliverables

1. D1: the field, the validation code, the four effects' reasons, and the field on every
   `ParameterDescriptor` literal.
2. D2-D5 in `crates/controls`.
3. D6 on the builtin table and the own-row tables.
4. The tests below.

## Authorized paths

- `crates/effect-contract/src/{lib.rs,control.rs,step.rs}` and `crates/effect-contract/tests/**`.
- The descriptor tables and test helpers of `crates/{parametric-eq,compressor,gate-expander,
  multiband-compressor,true-peak-limiter,soft-clip,transient-shaper,delay}` (the new field only;
  the four effects of D1 also set their reasons).
- `crates/effect-compiler/src/prepare.rs` and `crates/effect-compiler/tests/native_session.rs`,
  `crates/graph/src/lib.rs`, `crates/conformance/src/effect.rs`,
  `crates/conformance/tests/conformance_corpus.rs`: the new field in existing literals only.
- `crates/builtins/src/lib.rs` (the two display fields on the 12 rows).
- `crates/controls/**`, `crates/controls/Cargo.toml`, `Cargo.lock`.

## Non-goals

- No SDK, JSON or generated-file change (#1550). No wire form (#1551).
- No class flip: every row keeps decision 14's and decision 15's present class.
- No console link-mode row and no change to `SetConsole`.

## Objective gates

Commands: `cargo test --locked -p controls`, `cargo test --locked -p effect-contract`,
`cargo test --locked -p effect-compiler`, the eight effect crates' `cargo test --locked -p <crate>`,
`cargo test --locked -p conformance`, `bash scripts/check-sdk-generated.sh` (the metadata is unchanged, D7).

1. **Effect rows.** On a session with one console slot and one insert of each launch effect (two
   tracks and a submix), `list` yields for every instance one bypass row, one row per parameter and
   lane, and a link-mode row exactly on the inserts whose effect supports more than one mode, each
   address once and in D2's order. Each `None` parameter's row is `RebuildOnly` with D1's reason;
   the delay's and the multiband compressor's bypass rows carry #1339 and #1340.
2. **Reason required.** A test descriptor with a `None` parameter and no `prepared_reason` is
   refused by `validate_descriptor` with `PreparedReason`; one with a reason on a `Block`
   parameter too.
3. **Lane split.** Starting from one `Both` entry of a per-lane parameter: a `Left` write gives
   explicit `Left` (new) and `Right` (old) entries and no `Both`; a following `Both` write of equal
   values keeps two entries; a write equal to the committed value changes nothing; the resulting
   model round-trips through `canonical_session_json` and parse with equal bytes.
4. **One domain.** For every launch effect parameter, `check` accepts and refuses exactly where
   `parameter_value_valid` does at the minimum, the maximum, their next `f32` values and NaN.
5. **Table check** (#1546 D8) extended: every row has a display name.

## Test value

- Gate 1 turns red if an effect parameter, bypass or link mode is not a control, is listed twice,
  or carries a class that contradicts decision 14 or 15.
- Gate 2 turns red if a prepared parameter can be declared without a recorded reason.
- Gate 3 turns red if a one-lane write changes the other lane, loses the old value, or leaves a
  model that does not round-trip.
- Gate 4 turns red if the registry and the effect contract disagree on a parameter's domain.
- Gate 5 turns red if a row reaches a frontend with no name to show.

## Dependencies

- #1546 (the registry). #1547 is not needed.

## Supersedes and shrinks

- *Make the gate-expander's attack, hold and release live* (#1336), *Make a parametric EQ band's
  enabled and kind live* (#1337), *Make the multiband compressor's crossover live* (#1338), *Make
  the multiband compressor's link mode live* (#1367), *Give the delay a live bypass shunt* (#1339)
  and *Give the multiband compressor a live bypass shunt* (#1340): each is amended in place with one
  last step, flip its row to `Live` (see `amendments-a.md`).
