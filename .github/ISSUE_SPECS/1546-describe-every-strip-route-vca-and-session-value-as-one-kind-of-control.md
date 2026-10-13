# Describe every strip, route, VCA and session value as one kind of control

Slice S1 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors
verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal
tests, a batch review; no per-attempt verifier or mutation runs. The coordinator's frozen decisions
for this slice (`s1-decisions.md`) are restated here as D1-D9; this spec adds no decision that
contradicts them.

## Product outcome

One new crate, `controls`, lists every value of a session as a control row: a stable address, one
descriptor, a class with its reason, and the committed value. One validator checks a write against
the row, and one setter writes the model. Nothing outside tests calls the crate yet; later slices
route the hosts through it.

## Context

- **Builtin rows today.** `BuiltinParameterDescriptor` (`crates/builtins/src/lib.rs:239-255`) has
  `id`, `name`, `scope` (`PerLane` or `MatrixShared`), `mapping`, `domain`, `default`,
  `update_rate`, `smoothing`, `reset`, `disabled_value` and `lattice`. The table
  `BUILTIN_PARAMETER_DESCRIPTORS` (`:462-680`) has 12 rows: 1 `polarity_invert`, 2 `trim_db`,
  3 `hpf_hz`, 4 `lpf_hz`, 5 `fader_db`, 6 `mute`, 7-10 `matrix_ll`/`lr`/`rl`/`rr`,
  11 `delay_samples`, 12 `pan` (per-lane, `:663-679`). Only metadata generation and session
  validation read it; admission and the classifier do not.
- **Domain authority.** `BuiltinParameterDomain::contains(value, sample_rate)`
  (`crates/builtins/src/lib.rs:307-323`) is the builtin check: `BooleanExact` compares bits, so
  `-0.0` is refused; `DisabledOrRateKeyedHertz` accepts the disabled `0.0` or a cutoff up to
  `builtin_filter_cutoff_maximum_hz(rate)` (`:330-338`).
- **Ramp rows.** `LiveRamps` (`crates/host-core/src/prepare.rs:2411-2421`), `LiveRampRow`
  (`:2431-2463`) and `LiveRamps::resolve` (`:2508-2513`) live in host-core. host-core re-exports
  both (`crates/host-core/src/lib.rs:137-139`). Other users: `crates/host-core/src/live_delta.rs:35`
  (imports `crate::prepare::LiveRamps`), host-core tests, `crates/capi/src/runtime/live_tests.rs`,
  `crates/control-plane/src/control.rs` and `hosts/host-web/src/lib.rs`, all through the
  re-export.
- **The model.** `SessionModel` (`crates/session/src/model.rs:89-128`); `strips()` gives tracks,
  then submixes (`:490-495`); a strip's fader is `DualMonoFader` (`:713-722`, per-lane dB and
  mute); its pan or matrix is `MatrixOrPan` (`:726-755`, pan per lane, matrix shared, each with its
  own `smoothing_samples`); a route (`:875-895`) has `gain_db`, `mute`, `channel_matrix`
  (`ChannelMatrix`, `:931-940`) and `follows_mute`; a VCA (`:835-842`) has a `DualMonoFader` and
  members; `control_smoothing` (`ControlSmoothing`, `:140-148`, maximum
  `CONTROL_SMOOTHING_MS_MAXIMUM`, `:132`) holds `fader_ms`, `mute_ms`, `pan_ms`.
- **IDs.** Tracks, submixes, outputs and VCAs share one entity namespace
  (`crates/session/src/validate.rs:137-139`). `RackName` codes are inserts 2, builtins 4, console 5
  (`crates/session/src/model.rs:999-1019`). `ParameterChannel` is left, right, both (`:664-673`).
  `ParameterId::new(0)` is `None` (`crates/effect-contract/src/lib.rs:125-129`), so control `0` is
  free for an effect's bypass.
- **Derived values today.** `SessionModel::effective_strip_faders` (`crates/session/src/vca.rs:100`)
  composes the VCA-effective fader and mute. S1b replaces it.
- **Decision 14's classes.** `docs/rulings/live-update-versus-rebuild-2026-10-04.md:93-177`: strip
  builtins (all live except `delay_samples`, rule 1 new memory), routes (a send into a submix live;
  a route into the output prepared, rule 3 optimisation; `follows_mute` live by rule 2, not built),
  VCAs (live, not built on the C ABI).

## Decisions frozen for this slice

- **D1. Crates and layering.**
  - New module `effect_contract::control` (file `crates/effect-contract/src/control.rs`, declared
    `pub mod control;` in `crates/effect-contract/src/lib.rs`) holds `RebuildReason`,
    `ControlClass`, `LiveRampRow` (moved from `crates/host-core/src/prepare.rs` with its variants
    and docs unchanged), `Family`, `Taper` and `ControlKind`.
  - New crate `crates/controls` (package `controls`, lib `controls`). It depends on `session`,
    `builtins` and `effect-contract` only (S2 adds `effect-compiler`). It never depends on
    `builtins-compiler`, `host-core`, `protocol` or a crate above them. It re-exports the
    `effect_contract::control` vocabulary.
  - `LiveRamps` (with `for_session`, `for_row`, `resolve`) moves into `controls`. `host-core`
    depends on `controls` and re-exports `LiveRamps` and `LiveRampRow` from their old paths
    (`host_core::LiveRamps`, `host_core::LiveRampRow`), so no call site outside host-core changes.
- **D2. Vocabulary.**
  - `RebuildReason`: `NewMemory`, `NewGraph`, `ProcessingOrder`, `Latency`, `Glitch`,
    `Optimisation`, `Correctness`, `RuledLiveNotBuilt { issue: u32 }`.
  - `ControlClass`: `Live`, `RebuildOnly { reason: RebuildReason, reopen: &'static str }`,
    `Derived`.
  - `Family`: `StripLevel`, `StripMatrix`, `StripInput`, `Route`, `Effect`, `Setting`.
  - `Taper`: `Linear`, `Logarithmic`, `Exponential`, `Decibel`, `Stepped`.
  - `ControlKind`: `Continuous`, `Toggle`, `Enumeration`.
  - In `controls`: `Motion` (`Ramp(LiveRampRow)` or `Fixed`), `ControlScope` (`Model`,
    `Monitor`) and `LaneShape` (`PerLane`, `Shared`).
- **D3. Builtin rows.** `BuiltinParameterDescriptor` gains `ramp: Option<LiveRampRow>` (`None` is
  fixed), `class: ControlClass`, `family: Family`, `taper: Taper` and `kind: ControlKind`. All 12
  rows are filled in the existing table; there is no second builtin table.

  | id | name | ramp | class | family | taper | kind |
  |---|---|---|---|---|---|---|
  | 1 | `polarity_invert` | `PolarityInvert` | `RebuildOnly(RuledLiveNotBuilt { issue: STRIP_INPUT_SLICE })` | `StripInput` | `Stepped` | `Toggle` |
  | 2 | `trim_db` | `InputTrim` | as row 1 | `StripInput` | `Decibel` | `Continuous` |
  | 3 | `hpf_hz` | `None` (the fixed 64-update coefficient ramp) | as row 1 | `StripInput` | `Logarithmic` | `Continuous` |
  | 4 | `lpf_hz` | `None` | as row 1 | `StripInput` | `Logarithmic` | `Continuous` |
  | 5 | `fader_db` | `Fader` | `Live` | `StripLevel` | `Decibel` | `Continuous` |
  | 6 | `mute` | `Mute` | `Live` | `StripLevel` | `Stepped` | `Toggle` |
  | 7-10 | `matrix_*` | `Matrix` | `Live` | `StripMatrix` | `Linear` | `Continuous` |
  | 11 | `delay_samples` | `None` | `RebuildOnly(NewMemory)` | `StripInput` | `Linear` | `Continuous` |
  | 12 | `pan` | `Pan` | `Live` | `StripMatrix` | `Linear` | `Continuous` |

  `STRIP_INPUT_SLICE` is a named `pub const u32` in `controls` (or `builtins`), value `0` with a
  `// numbered by root` comment until root numbers #1556. Every `RebuildOnly` row has a non-empty
  `reopen` text (for `delay_samples`: decision 14's reopening condition, "a re-timing that does
  not glitch"; for the four input rows: "#1556 flips this row to live"). Builtin rows bind to the
  model through one `match` arm per builtin ID in `controls`.
- **D4. Address.** `ControlAddress { owner: Owner, section: Section, control: u32, lane: Lane }`.
  `Owner::{Session, Strip(StableId), Route(StableId), Vca(StableId)}`.
  `Section::{Own, Builtins, Console(StableId), Inserts(StableId)}`, with the codes own 6,
  builtins 4, console 5, inserts 2 (the `RackName` codes plus own). `Lane` is
  `session::ParameterChannel`. In an effect section, control `0` is the bypass. Lane rule (ruling
  C2): `list` shows a per-lane row as two rows, `Left` and `Right`, never `Both`; it shows a shared
  row as one row, `Both`. On write, `Both` on a per-lane row sets both lanes in one edit; `Left` or
  `Right` on a shared row is `control.lane`.
- **D5. Own rows (stable IDs, never reused).**
  - **Strip own:** 1 `matrix_mode` (`Enumeration`, choices pan `0` and matrix `1`, shared, `Live`,
    `StripMatrix`, ramp `Pan`); 2 `solo` (`Toggle`, shared, scope `Monitor`, `Live`, `StripLevel`,
    ramp `Solo`; listed on track strips only: a submix is solo-safe, so an address of `solo` on a
    submix is `control.address.unknown`); 3 `pan_window_samples` (the active `MatrixOrPan` variant's `smoothing_samples`,
    shared, `Live`, `Setting`, `Fixed`, unit samples, integral, domain `[0, 16_777_216]`);
    64 `effective_fader_db` (per-lane, `Derived`, dB, `[-144, 24]`); 65 `effective_mute`
    (per-lane, `Derived`, `Toggle`).
  - **Route own:** 1 `gain_db` (`[ROUTE_GAIN_DB_MINIMUM, ROUTE_GAIN_DB_MAXIMUM]`, ramp
    `SendGain`), 2 `mute` (ramp `SendMute`), 3 `matrix_ll`, 4 `matrix_lr`, 5 `matrix_rl`,
    6 `matrix_rr` (each `[-ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM, +..]`, ramp `SendMatrix`),
    7 `follows_mute` (ramp `SendFollowsMute`), 64 `effective_gate` (`Derived`, `Toggle`). Rows 1-7
    are shared; `effective_gate` is per-lane, one row per source lane (`Left`, `Right`), because a
    follow zeroes each muted source lane's matrix column. Family `Route`. The class depends on the route's shape:
    - a route into a submix: rows 1-6 are `RebuildOnly(RuledLiveNotBuilt { issue: ROUTE_SLICE })`
      (#1557 flips them) and row 7 is `RebuildOnly(RuledLiveNotBuilt { issue: EFFECTIVE_SLICE })`
      (#1558 flips it);
    - a route into the output: rows 1-6 are `RebuildOnly(Optimisation)` with decision 14's
      reopening condition (O9 with O4, or a real-path measurement that losing the fold costs
      little); row 7 is `active: false` there, because validation requires `false` on an output
      route, so a write is `control.inactive`.
  - **VCA own:** 1 `offset_db` (`[-144, 24]` dB, ramp `VcaOffset`), 2 `mute` (ramp `VcaMute`).
    Per-lane (listed `Left` and `Right`, as the model's `DualMonoFader`, `LiveVcaState` and the
    browser's per-lane VCA commands hold them). `RebuildOnly(RuledLiveNotBuilt { issue: EFFECTIVE_SLICE })` until #1558. Family
    `StripLevel`.
  - **Session own:** 1 `fader_ms`, 2 `mute_ms`, 3 `pan_ms` (`[0, CONTROL_SMOOTHING_MS_MAXIMUM]`,
    unit milliseconds). Shared. `Live`, `Setting`, `Fixed`. A write of any of them stores an
    explicit `ControlSmoothing` that copies the other two keys from the current lengths (the
    default table when the model says `default`).
  - `ROUTE_SLICE` and `EFFECTIVE_SLICE` are named constants like `STRIP_INPUT_SLICE` (D3).
  - Rows that `controls` defines carry their model accessors (`get` and `set` function pointers).
- **D6. The descriptor.** `ControlDescriptor` (rate-resolved) holds `name`, `unit`
  (`effect_contract::ParameterUnit`), `kind`, `minimum`, `maximum`, `disabled_value`, `default`,
  `taper`, `lattice` (`effect_contract::ParameterLattice`), `choices`
  (`&'static [effect_contract::EnumChoice]`), `motion`, `class`, `scope`, `family` and `lanes`
  (`LaneShape`). `automatable()` is computed, never stored:
  `class == Live && scope == Model && family != Setting`. A row's `active` flag is not a
  descriptor field; it is on the listed row, because it depends on the owner's shape: the pan row
  is active only in pan mode, the four matrix rows only in matrix mode, and `follows_mute` only on
  a route into a submix. `hpf_hz` and `lpf_hz` resolve `maximum` to
  `builtin_filter_cutoff_maximum_hz(rate)`. Display strings are not in this slice (S2 adds them).
- **D7. API.**
  - `controls::list(&SessionModel, sample_rate: u32) -> impl Iterator<Item = ControlRow>`, where
    `ControlRow { address, descriptor, active: bool, value: f32 }`. Order (ruling C4): strips in
    `model.strips()` order, then routes, then VCAs, then session rows. Within a strip: own rows,
    builtin rows (table order, `Left` before `Right`), pre-insert console slots, inserts,
    post-insert console slots. Effect sections yield no rows in S1 (S2 adds them). Header rows are
    a wire concern (S3).
  - `controls::get(&SessionModel, &ControlAddress, sample_rate) -> Result<f32, ControlError>`.
  - `controls::check(&ControlDescriptor, lane: Lane, value: f32, ramp: Option<u32>) ->
    Result<(), ControlError>`: pure and allocation-free. It refuses `control.lane` (a `Left` or
    `Right` lane on a shared row), `control.domain` (not finite; outside `[minimum, maximum]`
    unless equal to `disabled_value`; a toggle whose bits are not exactly `0.0` or `1.0`, so `-0.0`
    is refused as `BooleanExact` refuses it; an enumeration value that is not a choice; a fraction
    on an integral row) and `control.ramp.fixed` (`Some` ramp on a `Fixed` row). It does not see
    the model, so it does not check `read_only`, `inactive` or the address.
  - `controls::set(&mut SessionModel, &ControlAddress, value: f32, ramp: Option<u32>,
    sample_rate) -> Result<(), ControlError>`: look up the row (`control.address.unknown`),
    refuse a `Derived` row (`control.read_only`) and an inactive row (`control.inactive`), call
    `check`, then write the model. Nothing is written before every check passes. A value equal to
    the committed value is accepted and changes nothing. Values are not snapped to the lattice.
  - **Monitor rows.** A write to a `Monitor` row (solo) through `set` is refused with a typed
    error whose code names it, `control.monitor`, or `set` takes the overlay as
    well. The implementer picks the smaller design and records it in this spec's attempt record.
  - **`matrix_mode` (ruling C12).** Setting `matrix` from pan mode writes `MatrixOrPan::Matrix`
    with the exact coefficients of `builtins::pan_matrix(left, right)` and keeps
    `smoothing_samples`. Setting `pan` from matrix mode writes `MatrixOrPan::Pan` with each lane's
    pan at the `pan` row's default (`0.0`) and keeps `smoothing_samples`. A later edit of the same
    transaction can then set the pan. Setting the current mode changes nothing.
  - `ControlError` has one variant per code: `control.address.unknown`, `control.read_only`,
    `control.inactive`, `control.lane`, `control.domain`, `control.ramp.fixed` and `control.monitor`
    (if the implementer picks the refusal). Each has a stable `code() -> &'static str`.
  - The derived rows' `value` comes from a private helper equal to today's
    `SessionModel::effective_strip_faders` for fader and mute, and to
    `mute || (follows_mute && source lane mute)` for each source lane's gate. S1b replaces it.
- **D8. Table check.** A test in `controls` walks every row of a fixture: every address is unique;
  every `RebuildOnly` row has a reason and a non-empty `reopen`; `automatable()` is true only on
  `Live` + `Model` + non-`Setting` rows.
- **D9. No behaviour change.** No host calls `controls` in this slice. The SDK metadata generator
  output does not change (the new builtin fields are not emitted until S2).

## Deliverables

1. `crates/effect-contract/src/control.rs` (D1, D2).
2. `crates/controls` with the address, descriptor, own-row tables, builtin binding arms, `list`,
   `get`, `check`, `set`, `ControlError` and the moved `LiveRamps` (D1-D7).
3. The five new fields on the 12 builtin rows (D3).
4. host-core: `LiveRamps` and `LiveRampRow` removed from `prepare.rs` and re-exported (D1).
5. The tests of the gates below.

## Authorized paths

- `crates/effect-contract/src/control.rs` (new); `crates/effect-contract/src/lib.rs` (the `mod`
  line and a re-export only).
- `crates/controls/**` (new).
- `Cargo.toml` (one workspace member), `Cargo.lock`.
- `crates/builtins/src/lib.rs`: `BuiltinParameterDescriptor` and the 12 rows of
  `BUILTIN_PARAMETER_DESCRIPTORS` only.
- `crates/host-core/Cargo.toml`, `crates/host-core/src/lib.rs` (the re-export line),
  `crates/host-core/src/prepare.rs` (the `LiveRamps`/`LiveRampRow` block and its imports),
  `crates/host-core/src/live_delta.rs` (the one import line).

## Non-goals

- No host, protocol, classifier or SDK change. No wire form (S3) and no `SET_CONTROL` op (S4a).
- No effect rows and no display strings (S2).
- No effective-value function (S1b). The derived rows use the private helper of D7.
- No change to `BuiltinParameterDescriptor`'s existing fields or to session validation.

## Objective gates

Commands: `cargo test --locked -p controls`, `cargo test --locked -p builtins -p effect-contract`,
`cargo test --locked -p host-core --all-features`, `bash scripts/check-sdk-generated.sh` (runs the metadata generator's `--check`).

1. **List.** On one fixture with two tracks, one submix, a send into the submix, a `pre_fader`
   and a `post_fader` route into the output, and one VCA, `list` yields every expected address
   exactly once, in C4's order, and each row's class and reason are as D3 and D5 say (for
   example: the post-fader output route's `gain_db` is `RebuildOnly(Optimisation)`,
   `delay_samples` is `RebuildOnly(NewMemory)`, the send's `gain_db` is
   `RebuildOnly(RuledLiveNotBuilt { .. })`, `fader_db` is `Live`). The pan rows are active and the
   matrix rows inactive on a pan-mode strip, and the reverse on a matrix-mode strip. `solo` is listed on
   each track and not on the submix; the VCA rows and `effective_gate` are listed `Left` and `Right`.
2. **Set and get.** `set` then `get` round-trips every writable row of the fixture. Each
   `ControlError` code is produced at its boundary: each range end accepted and its next `f32`
   refused; toggle `0.5` and `-0.0`; a shared row written with `Left`; a derived row; an inactive
   matrix row; an explicit ramp on a `Fixed` row; an unknown strip; an unknown builtin ID; `solo` on the submix
   (`control.address.unknown`). A
   refused `set` leaves the model equal to its clone taken before the call. `matrix_mode` from pan
   to matrix gives the exact `pan_matrix` coefficients.
3. **Table check** (D8).
4. **Nothing else moved.** The builtins, effect-contract and host-core suites pass unchanged, and
   `scripts/check-sdk-generated.sh` passes with no regenerated file.

## Test value

- Gate 1 turns red if a row is missing, listed twice, out of order, or carries a class that
  contradicts decision 14.
- Gate 2 turns red if the validator accepts an out-of-domain or wrong-lane write, or if a refused
  write leaves a partial model change.
- Gate 3 turns red if a later row is added without a reason or is marked automatable when it is
  not live.
- Gate 4 turns red if moving `LiveRamps` changed a ramp length or a metadata byte.

## Dependencies

- None. This is the first slice of decision 17.

## Supersedes and shrinks

- *Route the builtins fader domain checks through checked_fader_gain* (#1303): amended (see
  `amendments-a.md`). Its preparation scope stays; its non-goal that keeps host-web's literal copy
  is overridden by #1548.
