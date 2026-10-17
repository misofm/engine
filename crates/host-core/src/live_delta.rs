//! Classify a committed session delta as live records or a rebuild, through the control registry
//! (issues #1255, #1264, #1265, #1266 and #1554; umbrella #1053; decision 17, design point C6).
//!
//! A host that holds a running plan and commits a transaction has two models: the committed one
//! the plan was prepared with (plus every record pushed into it since, #1053 D9), and the
//! transaction's prospective one. [`classify_live_delta`] reads the two and decides, without ever
//! looking at edit opcodes, whether the difference can ride the plan's live lanes as records, or
//! needs a plan rebuild. For a live delta it returns exactly the records that bring the render
//! plane from the first model's values to the second's, and never a redundant one.
//!
//! This is the one classifier, and it keeps no list of fields: `controls` says which model fields
//! are control values and whether each is live or rebuild-only, and this module lowers a live
//! change to records, one lowering per [`controls::Family`]. A later slice widens it in place with
//! a new arm of the lowering (the input records are #1556's, the route records #1557's, the derived
//! values #1558's); it does not add a second classifier.
//!
//! A strip's fader and mute, and a send's gate, are *derived* values (decision 17, C7): the effective
//! values of `controls::effective`, which preparation also reads. The classifier diffs the effective
//! values of the two models (`EffectiveDiff`), never the raw ones, so a VCA move, a membership change,
//! a followed mute and an own edit are one difference, and preparation and the live path cannot
//! compose them two ways.

use std::cell::OnceCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::route_controls::RouteControlRecord;
use builtins::{checked_trim_gain, validate_prepared_input_filter_target};
use builtins_compiler::{
    FaderOffsetsRecord, TrackControlRecord, TrackFaderRecord, TrackInputRecord,
};
use controls::{
    ChangedControl, ControlAddress, ControlClass, EffectiveStrip, Family, LocalInputs, ModelView,
    Owner, Section,
};
use effect_compiler::{LiveEffectAddress, launch_native_effect_registry, resolve_initial_values};
use effect_contract::{EffectControlRecord, PreparedEffectTarget};
use session::{
    EditRamps, Effect, EffectIdentity, EffectParam, LoweredRacks, Route, RouteDestination,
    SessionModel, StableId, StripRef, canonical_session_json,
};

#[cfg(feature = "test-support")]
thread_local! {
    static FAST_CLASSIFICATIONS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

/// How many calls of [`classify_live_edit`] on this thread were answered by the fast path (from the
/// rows the edits name, with no model clone and no canonical JSON) since the last call, and resets
/// the count (#1612). A test shows that a transaction took the path it was meant to.
#[cfg(feature = "test-support")]
pub fn take_fast_classification_count() -> usize {
    FAST_CLASSIFICATIONS.with(|count| count.replace(0))
}

// The ramp rule and the per-owner record derivation live with preparation, which needs them on
// every build (a successor's carry-then-retarget, #1277 D5); the classifier calls them.
use crate::prepare::{
    EffectiveDiff, LevelRecords, LevelRules, OutOfDomain, RouteEditRamps, StripChange,
    StripEditRamps, effect_instance_records, input_records, level_differs, level_records_between,
    matrix_record, route_record_between, route_source_strip, same_params, strip_at,
};
use crate::prepare::{GraphJumpLengths, LiveRampRow, LiveRamps, MonitorBase, session_jump_lengths};

/// The records of one strip whose live values change.
#[derive(Clone, Debug, PartialEq)]
pub struct LiveStripRecords<'a> {
    /// The strip's ID, borrowed from the post-commit model.
    pub strip_id: &'a str,
    /// For the strip's fader/mute queue, in push order: `FaderDb` records first, then `Mute`
    /// records, each left before right; `None` after the last one.
    pub fader: [Option<TrackFaderRecord>; 4],
    /// For the offsets cells of the strip's automated fader lanes (#1649 D4), `[left, right]`:
    /// the lane's new `S` and the length of its jump, when `S` changes on a lane the post-commit
    /// model automates. A host writes each to the lane's cell (`FaderCells::write_offsets`).
    pub offsets: [Option<FaderOffsetsRecord>; 2],
    /// For the strip's matrix/pan queue: the new lowered target, when it changes.
    pub matrix: Option<TrackControlRecord>,
    /// For the strip's input cells (#1556 D2), in write order: the `TrimDb` records, then the
    /// `PolarityInvert` records, then the `PreparedFilter` targets in the order the preparer
    /// returns them, each left before right; empty when no input value changes.
    pub input: Vec<TrackInputRecord>,
}

impl LiveStripRecords<'_> {
    /// Whether every input record satisfies the contract the strip's input cells assume (#1556
    /// D5): a trim inside the domain and a filter target [`validate_prepared_input_filter_target`]
    /// accepts, so that a cell write, which cannot fail, never meets a value it was not built for.
    /// The classifier produces only such records, so a host that finds `false` has an internal
    /// fault and must refuse before its first write.
    #[must_use]
    pub fn input_records_valid(&self) -> bool {
        self.input.iter().all(|record| match record {
            TrackInputRecord::TrimDb { db, .. } => checked_trim_gain(*db).is_ok(),
            TrackInputRecord::PolarityInvert { .. } => true,
            TrackInputRecord::PreparedFilter { target } => {
                validate_prepared_input_filter_target(target).is_ok()
            }
        })
    }

    /// The fader/mute records, in push order.
    pub fn fader_records(&self) -> impl Iterator<Item = TrackFaderRecord> + '_ {
        self.fader.iter().map_while(|record| *record)
    }
}

/// The records of one effect instance whose live bypass or parameter values change (#1264 D2,
/// #1265 D1, #1266 D1).
#[derive(Clone, Debug, PartialEq)]
pub struct LiveEffectRecords<'a> {
    /// The ID of the strip (a track) that owns the instance, borrowed from the post-commit model.
    pub strip_id: &'a str,
    /// The instance's live address in its strip: a console slot by its slot index, an insert by
    /// its index. A producer is found by `(strip_id, address)`.
    pub address: LiveEffectAddress,
    /// One [`EffectControlRecord::Bypass`] first when the instance's bypass changes (#1266 D1); it
    /// carries the crossfade ramp, in samples: the session's `mute_ms` through
    /// [`LiveRampRow::BypassCrossfade`] (#1341 D1, #1622 D1), the one live row with no per-edit
    /// length, so `0` (a step) only when the session's `mute_ms` is 0. A host writes the record
    /// as it is.
    /// then one [`EffectControlRecord::Parameter`] per `(parameter_index, channel)` whose resolved
    /// value changes, in descriptor order, `Left` before `Right`; never empty.
    ///
    /// For an instance with `targets` (the EQ) the `Parameter` records are its owner's edits
    /// (`EffectControlProducer::edit_owner`), never written as they are: the cells get the targets.
    /// Its `Bypass` record, like every other instance's, is written as it is, ahead of the
    /// targets.
    pub records: Vec<EffectControlRecord>,
    /// For an effect whose parameters ride prepared targets (the parametric EQ, #1265): the
    /// targets [`crate::EqTargetPreparer`] designed on the control thread from the instance's pre-commit
    /// values and `records`, in the order its owner publishes them
    /// (`EffectControlProducer::publish_candidate_targets`). `None` for every other effect, whose
    /// `records` are written as they are.
    pub targets: Option<Vec<PreparedEffectTarget>>,
}

/// The record of one route into a submix whose live values change (#1557 D2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LiveRouteRecord<'a> {
    /// The route's ID, borrowed from the post-commit model. A producer is found by it
    /// ([`crate::route_producer_position`]).
    pub route_id: &'a str,
    /// The send's new target, its gate and its ramp length: what
    /// `graph_compiler::route_coefficients` gives for the post-commit values, which is what
    /// preparation folds into the route's constants.
    pub record: RouteControlRecord,
}

/// A live delta: the records that bring the running plan to the post-commit model.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LiveDelta<'a> {
    /// One entry per strip that has at least one record, in normalized (canonical track ID)
    /// order. Empty for a transaction that rewrites identical values: that is still a live delta.
    pub strips: Vec<LiveStripRecords<'a>>,
    /// One entry per effect instance that has at least one parameter record: by track in
    /// normalized order, then in chain order (`pre_insert` slots, inserts, `post_insert` slots).
    pub effects: Vec<LiveEffectRecords<'a>>,
    /// One entry per route into a submix that has a record, in canonical route-ID order (#1557 D2).
    pub routes: Vec<LiveRouteRecord<'a>>,
    /// The list-order indices (`controls::owner_count`'s order) of every owner with at least one
    /// row whose value bits differ between the two models, of any class and family: the owners a
    /// control catalog has to read again (#1600 D1). A derived row is not counted, since it
    /// follows other controls, and it is the catalog's to refresh from the effective values.
    /// A strip whose solo differs between the two monitor overlays is counted too: its solo row is
    /// a host value, not a model row. Ascending, with no duplicate. Empty when the models differ in no control value.
    ///
    /// Only the rows both models list are compared, so a VCA added or removed names no VCA owner
    /// here; a catalog compares its own owner list.
    pub changed_owners: Vec<usize>,
    /// Whether a row of family `Setting` (the session's `control_smoothing` lengths) differs. Such
    /// a row changes the descriptors of every owner, not only its own (`default_ramp_samples`).
    pub settings_changed: bool,
    /// The list-order indices of the owners whose **derived** rows the classification reached: the
    /// strips whose effective fader or mute can differ (their own level, a VCA that reaches them,
    /// their solo), by strip index, and the sends whose gate can differ, as the strip count plus the
    /// route index. Ascending. `Some` only when the classification was made from the rows the edits
    /// name (#1612, #1613), which also means the models list the same owners in the same order; a
    /// control catalog then stages the derived rows of these owners and of `changed_owners` and no
    /// others. `None` when the classification compared the whole models: every derived row is read.
    pub derived_owners: Option<Vec<usize>>,
    /// The new value of the plan's jump-length cell (#1639 D3): `Some` when the session's
    /// `control_smoothing` lengths in samples differ between the two models and the post-commit
    /// model holds at least one stored automation whose jumps ramp over one of them
    /// ([`automation_jump_row`]); `None` otherwise, equal sample counts included. The host writes
    /// it to the newest plan's cell after every fallible check, with the transaction's records.
    pub jump_lengths: Option<GraphJumpLengths>,
}

impl LiveDelta<'_> {
    /// Whether the delta holds no record of any kind (#1313 D2): a transaction that changed only
    /// model-only fields (the session ID, a profile ID, the stored automation of a row that does
    /// not render yet) or rewrote
    /// identical values. The host reports such a commit as `model_only` and a delta with at least
    /// one record as `live`.
    ///
    /// The body names every field, so a field added to [`LiveDelta`] does not compile until it is
    /// classified here.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        let Self {
            strips,
            effects,
            routes,
            // Which owners changed and whether a setting did are the catalog's inputs, not
            // records: a model-only commit moves values and has no record.
            changed_owners: _,
            settings_changed: _,
            derived_owners: _,
            jump_lengths,
        } = self;
        strips.is_empty() && effects.is_empty() && routes.is_empty() && jump_lengths.is_none()
    }
}

/// The live ramp row whose session length the jumps of a stored automation on `target` ramp over
/// (#1639 D3, #1637 D2 as amended): the row of a control whose live motion is a
/// `control_smoothing` ramp (`controls::row_motion`), and an effect's bypass, whose crossfade is
/// [`LiveRampRow::BypassCrossfade`]. A stage resolves the row's length with
/// [`LiveRamps::for_row`]. `None` for every other row: an effect parameter, the input filters and
/// the EQ keep their fixed ramps, and a link-mode row is not automatable.
#[must_use]
pub fn automation_jump_row(target: &ControlAddress) -> Option<LiveRampRow> {
    match &target.section {
        Section::Console(_) | Section::Inserts(_) => {
            (target.control == controls::BYPASS_CONTROL).then_some(LiveRampRow::BypassCrossfade)
        }
        Section::Own | Section::Builtins => {
            match controls::row_motion(target.owner.kind(), target.section.kind(), target.control) {
                Some(controls::Motion::Ramp(row)) => Some(row),
                Some(controls::Motion::Fixed) | None => None,
            }
        }
    }
}

/// D3 of #1639: the new jump lengths when `current` and `next` differ in them, in samples, and
/// `next` holds an automation whose jumps read one of them. The rule reads the session only, not
/// which rows render yet: every plan has the cell, so the write always lands, and a row whose
/// rendering slice is still to come reads the new value from its first jump.
fn jump_length_change(current: &SessionModel, next: &SessionModel) -> Option<GraphJumpLengths> {
    let after = session_jump_lengths(next);
    if session_jump_lengths(current) == after {
        return None;
    }
    next.automation
        .iter()
        .any(|automation| automation_jump_row(&automation.target).is_some())
        .then_some(after)
}

pub use crate::prepare::LiveRebuild;

/// Classifies the delta from `current` to `next` as live records or a rebuild (#1053 D1, #1554).
///
/// Both inputs must be **normalized** models (`CompiledSession::normalized_model()`): the
/// committed session's and the transaction's prospective one. The structure comparison relies on
/// normalized order.
///
/// The control rows (`controls`, decision 17) say what a value is and how it changes; this
/// function keeps no list of fields. The steps, in order:
/// 1. [`LiveRebuild::Structure`] if `next`, with `current`'s revision, session ID, render- and
///    output-profile IDs and stored automation, and with every control value of
///    [`controls::mask_values`] copied from `current`, has canonical JSON bytes other than
///    `current`'s. Bytes, never `PartialEq`: the canonical `f32` spelling keeps a zero's sign. A
///    canonical JSON error is structural too.
/// 2. [`LiveRebuild::AutomationTarget`] if `current.automation != next.automation` or a row's
///    activity changed ([`controls::activity_changed`]: a strip's `matrix_mode` switch flips which
///    of its pan and matrix rows are active, #1631) or a static HPF or LPF moved on a strip that
///    automates either filter ([`controls::filter_statics_changed_under_automation`]: the order
///    rule reads the static side, #1645) and [`controls::automation_diagnostics`] refuses one of
///    `next`'s automations (decision 15 E1, #1335 D4, #1566 D6). An unchanged automation table
///    over unchanged activity and unchanged filter statics is never re-checked: it was prepared.
/// 3. [`LiveRebuild::Automation`] if the entries of the rows that render differ
///    ([`controls::rendered_automation_changed`], #1648 D1): an entry on such a row (today a
///    strip's fader and mute, builtin rows 5 and 6, [`controls::automation_row_renders`], #1651)
///    is added, removed, retargeted or given other segments. A plan compiles its cells from these
///    entries, so only a rebuild can carry the change; the successor also seeds its jump-length
///    cell from its own model (#1639 D4). An entry's hand offset (#1650) is not compared. An
///    accepted change on a row that does not render yet stays live with no records, as #1260 D2
///    made it.
/// 4. With the structure equal, the two models list the same controls in the same order. For each
///    control whose value bits differ ([`controls::changed_controls`]): a row that is
///    rebuild-only is [`LiveRebuild::RebuildOnly`] with the row's reason; a row of family
///    `Setting` gives no record; a live row marks its owner changed for its family.
/// 5. [`LiveRebuild::Domain`] if a changed effect's `params` do not resolve through
///    [`resolve_initial_values`] (the one function preparation uses), whether or not a row's value
///    moved: an unknown parameter or a unit mismatch has no row.
/// 6. Each changed owner is lowered once per family (`lower`): a strip's input trim, polarity and
///    filters (#1556), a strip's fader and mute, a strip's pan or matrix, an effect instance's
///    bypass and parameters, a send's gain, mute and matrix (#1557). An unchanged owner is
///    never lowered. A lowering's domain refusal is [`LiveRebuild::Domain`].
///
/// A strip's fader and mute and a send's gate are derived (decision 17, C6 step 3, #1558 D1): the
/// control list does not say which strip an edit reaches, so when a strip-level row, a VCA (its set,
/// offsets, mutes or members) or a send's row changes, the classifier computes
/// [`controls::effective`] for both models. Each strip whose effective fader or mute changed gets a
/// level record, and each send whose effective gate or own gain or matrix changed gets a route
/// record. The records come from the effective values only, never from a raw fader, mute or VCA
/// field.
///
/// The session ID, the two profile IDs, the stored automation of rows that do not render yet and
/// `control_smoothing` are model-only: no prepared plan reads them (#1260, #1365), so a delta that
/// changes only them is live with no records, once step 2 has accepted the automation's targets
/// and step 3 has found no rendered entry changed. Each rendering slice adds its row to
/// [`controls::automation_row_renders`], which moves that row's edits to step 3. The one exception is
/// the plan's jump-length cell (#1639 D3): when the lengths in samples differ and `next` holds a
/// stored automation whose jumps ramp over one of them ([`automation_jump_row`]), the delta
/// carries [`LiveDelta::jump_lengths`] and is not empty. `control_smoothing`'s
/// record reader is [`LiveRamps::for_session`], which the caller evaluates on `next`: the records
/// of the same transaction use the new lengths, and a ramp already in flight keeps its own. A
/// preparation that bakes a ramp length into a plan (a transition decided at preparation) reads
/// the key at that preparation only, so a later edit still governs only later transitions. The
/// first such slice is a successor's carry retarget (#1277 D5): its fader, mute and matrix records
/// are written to the successor's cells at preparation, with the lengths of the successor's own
/// model (and the transaction's per-edit lengths, #1394 D6). Step 1 masks the whole automation
/// table because steps 2 and 3 judge it, by row and by rendered curve.
///
/// A static fader value of a lane that `next` automates ([`controls::automated_cell`]) gives no
/// `FaderDb` record (#1648 D3): the lane's cell renders the curve, and the static value is its
/// fallback. Since the record comes from the effective value, the rule covers a VCA move that
/// reaches the lane too. The lane's `Mute` records are unchanged: on a lane whose mute `next`
/// automates (#1651 D1, OQ1 = C) the record still carries the effective mute
/// ([`controls::EffectiveStrip::mute`]: the strip's own static mute, any reaching VCA's, or the
/// solo term), which is the lane's `terms`, and render composes the stage mute `curve || terms`.
///
/// A record is emitted only when the value the render plane holds changes (`solo.rs`: never a
/// redundant record): a `FaderDb` per lane whose effective gain bits change (one `Both` when both change to
/// the same dB bits), a `Mute` per lane whose mute changes (one `Both` when both change to the
/// same value), a matrix record when a lowered coefficient's bits change, and the strip's input
/// records (a `TrimDb` and a `PolarityInvert` by the same rules, then the filter targets
/// [`InputFilterPreparer`](crate::InputFilterPreparer) designs for the changed cutoffs, #1556).
/// A send into a submix gets one record when its folded target or its gate changes
/// (`route_records`). A fader move on a
/// lane that stays muted still gets its record, so the stage remembers the gain for a later
/// unmute. An effect record is emitted only for a resolved value whose bits change: a `Both`
/// value rewritten as equal per-lane values gives none.
///
/// A record's ramp length is [`LiveRamps::resolve`]'s: the length the transaction's own edit gave
/// the record's `(strip, row)` in `edit_ramps` (an explicit 0 included), else the session's
/// `ramps` for the row (#1394 D5, D6). A level record of a lane the strip's own value did not move
/// takes the reaching VCA's edit and row, and a send's gate record takes its source strip's `Mute`
/// length when no edit of the send caused it (#1558 D7). A pan or matrix record without an edit length takes the
/// post-commit model's own non-zero `smoothing_samples` before the session's `pan_ms` (#1054 D5).
///
/// No smoothing window is checked: host-core preparation passes `maximum_smoothing_samples:
/// u32::MAX` and the render setters accept any `u32` window. If preparation ever gets a finite
/// smoothing cap, this classifier must refuse a smoothing above it as [`LiveRebuild::Domain`];
/// otherwise a live commit could leave a committed model that its own rebuild refuses.
///
/// # Allocation
///
/// Control thread only. The masked clone, the two canonical JSON strings, the two control lists
/// and their difference, when the automation changes the automation diagnostics, and, for a
/// changed effect instance, the lowered racks, the resolved values and an EQ's target designer
/// are allocated and freed on every call, on the control-plane precedent of #369 (the protocol
/// already compiles a whole session per edit). Nothing it allocates is retained apart from the
/// returned entries. The launch registry is the process's shared one (#1469): the first call of
/// the process that needs it builds it, and every later call reads it.
///
/// # Errors
///
/// The [`LiveRebuild`] reason of the first step that refuses.
pub fn classify_live_delta<'a>(
    current: &SessionModel,
    next: &'a SessionModel,
    ramps: LiveRamps,
    edit_ramps: &EditRamps,
) -> Result<LiveDelta<'a>, LiveRebuild> {
    classify_live_delta_with_monitor(current, next, ramps, edit_ramps, MonitorBase::NONE)
}

/// [`classify_live_delta`] for a host that holds a monitor overlay (decision 17, C8, #1559 D6,
/// D7): the effective values of step 6 are those of `current` under `monitor.committed` and of
/// `next` under `monitor.next`, so a solo's mutes and the closing of the sends that follow them
/// are records like any other. A change of the overlay alone makes the effective values differ
/// (the model is the same), and its records take the ramp of `monitor.ramp` unless a model edit
/// of the same transaction produced them too.
///
/// # Errors
///
/// As [`classify_live_delta`].
pub fn classify_live_delta_with_monitor<'a>(
    current: &SessionModel,
    next: &'a SessionModel,
    ramps: LiveRamps,
    edit_ramps: &EditRamps,
    monitor: MonitorBase<'_>,
) -> Result<LiveDelta<'a>, LiveRebuild> {
    // Step 1. The control values are the rows' (decision 17, C6); the model-only fields are the
    // classifier's own. The VCAs are wholly control values (`mask_values`, #1558 D1).
    let mut masked = next.clone();
    controls::mask_values(&mut masked, current);
    masked.revision = current.revision;
    // #1260 D1: fields no prepared plan reads. Each profile's other fields stay compared.
    masked.session_id = current.session_id.clone();
    masked.render_profile.id = current.render_profile.id.clone();
    masked.output_profile.id = current.output_profile.id.clone();
    // The stored automation is judged by steps 2 and 3, not by bytes here: its targets by their
    // rows (step 2), the entries of the rows that render by their curves (step 3, #1648 D1).
    masked.automation = current.automation.clone();
    match (
        canonical_session_json(current),
        canonical_session_json(&masked),
    ) {
        (Ok(before), Ok(after)) if before.as_bytes() == after.as_bytes() => {}
        _ => return Err(LiveRebuild::Structure),
    }
    drop(masked);

    // Step 2.
    if (current.automation != next.automation
        || controls::activity_changed(current, next)
        || controls::filter_statics_changed_under_automation(current, next))
        && !controls::automation_diagnostics(next, next.sample_rate_hz).is_empty()
    {
        return Err(LiveRebuild::AutomationTarget);
    }

    // Step 3 (#1648 D1). After step 2, so a refusable entry keeps its refusal's route.
    if controls::rendered_automation_changed(current, next) {
        return Err(LiveRebuild::Automation);
    }

    // Step 4. A VCA added or removed has no row in the diff, so the lists decide whether the
    // effective values can differ, and a change of the overlay moves the effective mutes without
    // moving a row.
    let changes = controls::changed_controls(current, next, next.sample_rate_hz)
        .ok_or(LiveRebuild::Structure)?;
    let mut accumulated = Changes {
        derived: current.vcas != next.vcas || !monitor.committed.same_solo(monitor.next),
        ..Changes::default()
    };
    for change in &changes {
        accumulated.note(next, change)?;
    }

    // A strip whose solo or master designation differs between the two overlays changed its
    // monitor row, which is no model row and so has no entry in `changes` (#1600 D1, #1559,
    // #1683). The master moves no effective value, only the row.
    if monitor.committed != monitor.next {
        for (index, strip) in next.strips().enumerate() {
            if monitor.committed.is_soloed(strip.id) != monitor.next.is_soloed(strip.id)
                || monitor.committed.is_master(strip.id) != monitor.next.is_master(strip.id)
            {
                accumulated.note_owner(index);
            }
        }
    }

    // Step 5.
    refuse_unresolvable_params(current, next)?;

    // Step 6. The effective values of both models, once (decision 17, C7).
    let diff = accumulated
        .derived
        .then(|| EffectiveDiff::new(current, next, ramps, edit_ramps, monitor));
    let mut work = accumulated.work.drain(..).peekable();
    let strips = next.strips().enumerate().map(|(index, after)| {
        let item = work
            .next_if(|item| item.strip == *after.id)
            .unwrap_or_else(|| Work::new(after.id.clone()));
        (index, item)
    });
    let mut delta = LiveDelta {
        changed_owners: core::mem::take(&mut accumulated.changed_owners),
        settings_changed: accumulated.settings_changed,
        jump_lengths: jump_length_change(current, next),
        ..LiveDelta::default()
    };
    let diff = diff.as_ref().map(|diff| diff as &dyn LevelDiff);
    lower_strips(
        current,
        ModelView::whole(next),
        ramps,
        edit_ramps,
        diff,
        strips,
        &mut delta,
    )?;
    if let Some(diff) = diff {
        lower_routes(
            current,
            ModelView::whole(next),
            ramps,
            edit_ramps,
            diff,
            &accumulated.routes,
            0..next.routes.len(),
            &mut delta,
        )?;
    }
    Ok(delta)
}

/// The classification of a transaction at a host that holds a running plan (#1612): the live delta
/// of `current` to `next`, or the reason it needs a rebuild. The one entry both hosts' control
/// plane calls.
///
/// `rows` is the transaction's list of the rows its edits wrote, when every edit
/// is a `SetControl` (`PreparedSessionTransaction::set_control_rows`), else `None`. With rows, the
/// delta is read from the rows the edits name: each row's old and new value, the strips, sends and
/// VCAs that row reaches, and the lowering of [`classify_live_delta_with_monitor`], with no model
/// clone and no canonical JSON; its cost grows with the edits, not with the session. It returns the
/// very delta the full classifier returns for the same models, and whenever it cannot judge, it
/// runs the full classifier and returns that, error included:
///
/// - `rows` is `None` (a structural edit, or any other edit);
/// - a named row moves more than itself (`controls::changed_rows`: the session's rows, a strip's
///   `matrix_mode`), does not resolve in both models, or is not live;
/// - a named row is a strip's HPF or LPF and the strip automates either filter: the order rule
///   reads the static value (#1645), and the full classifier's step 2 judges it;
/// - the effective values of the strips and sends the rows reach cannot be built locally;
/// - a step of the lowering refuses: the full classifier then decides which refusal wins.
///
/// So an edit is never acked as live by the fast path and then dropped: the fast path answers `Ok`
/// only with the delta that the shared lowering built.
///
/// Both models must be normalized (`CompiledSession::normalized_model()`), as for
/// [`classify_live_delta_with_monitor`].
///
/// # Allocation
///
/// Control thread only. The fast path allocates the changed rows, the entries of the strips and
/// sends they reach and the returned delta, none of which grows with the number of strips; a
/// transaction that moves a mute scans the routes once (comparing sources) to find the sends that
/// follow it.
///
/// # Errors
///
/// As [`classify_live_delta_with_monitor`].
pub fn classify_live_edit<'a>(
    current: &SessionModel,
    next: &'a SessionModel,
    ramps: LiveRamps,
    edit_ramps: &EditRamps,
    monitor: MonitorBase<'_>,
    rows: Option<&[ControlAddress]>,
) -> Result<LiveDelta<'a>, LiveRebuild> {
    if let Some(rows) = rows
        && let Some(delta) = classify_live_staged(
            current,
            ModelView::whole(next),
            ramps,
            edit_ramps,
            monitor,
            rows,
        )
    {
        return Ok(delta);
    }
    classify_live_delta_with_monitor(current, next, ramps, edit_ramps, monitor)
}

/// The fast path of [`classify_live_edit`] over a [`ModelView`] (#1613): the classification of a
/// transaction of `SetControl`s whose model `next` is the committed model `current` with the owners
/// the transaction edited, read as one model, so that no prospective model is built. `None` when it
/// cannot answer, and the caller then has the full classifier decide on a whole model, which it
/// first builds ([`ModelView::to_model`], or the full preparation of the transaction).
///
/// `rows` are the rows the edits wrote ([`protocol`'s `set_control_rows`]); the view must hold every
/// owner they name. For a whole view it is [`classify_live_edit`]'s fast path, with the cost the
/// rows have and the answers the full classifier has.
///
/// The `Ok` delta carries [`LiveDelta::derived_owners`].
///
/// # Allocation
///
/// As [`classify_live_edit`].
pub fn classify_live_staged<'a>(
    current: &SessionModel,
    next: ModelView<'a>,
    ramps: LiveRamps,
    edit_ramps: &EditRamps,
    monitor: MonitorBase<'_>,
    rows: &[ControlAddress],
) -> Option<LiveDelta<'a>> {
    let delta = classify_rows(current, next, ramps, edit_ramps, monitor, rows)?;
    #[cfg(feature = "test-support")]
    FAST_CLASSIFICATIONS.with(|count| count.set(count.get() + 1));
    Some(delta)
}

/// The fast path of [`classify_live_edit`]: `None` when it cannot answer.
fn classify_rows<'a>(
    current: &SessionModel,
    view: ModelView<'a>,
    ramps: LiveRamps,
    edit_ramps: &EditRamps,
    monitor: MonitorBase<'_>,
    rows: &[ControlAddress],
) -> Option<LiveDelta<'a>> {
    let next = view.structure();
    let rate = next.sample_rate_hz;
    // The edits wrote their rows and nothing else, so the two models' strips, routes and effects
    // are the same ones; a count that differs is not a transaction of this kind.
    if current.tracks.len() != next.tracks.len()
        || current.submixes.len() != next.submixes.len()
        || current.routes.len() != next.routes.len()
    {
        return None;
    }

    // A static filter under filter automation can break the order rule (#1645): step 2 of the full
    // classifier judges it. The edits leave the automation table as committed.
    if rows
        .iter()
        .any(|address| controls::filter_under_automation(current, address))
    {
        return None;
    }

    // The changed rows, in the control list's order, each row once.
    let mut ordered = Vec::with_capacity(rows.len());
    for address in rows {
        ordered.push((RowOrder::of(next, address)?, address));
    }
    ordered.sort_by_key(|left| left.0);
    ordered.dedup_by(|later, earlier| later.0 == earlier.0);
    let mut changes: Vec<ChangedControl> = Vec::new();
    for (_, address) in ordered {
        changes.extend(controls::changed_rows(
            current,
            view.owners(),
            address,
            rate,
        )?);
    }

    let mut accumulated = Changes {
        derived: !monitor.committed.same_solo(monitor.next),
        ..Changes::default()
    };
    for change in &changes {
        // A refusal (a rebuild-only row) is the full classifier's to report.
        accumulated.note(next, change).ok()?;
    }

    // What the edits reach: the strips whose level a row or a VCA moves, the strips whose solo
    // differs (every strip when whether any strip is soloed differs), and the sends whose row
    // changed.
    let mut seeds = LocalSeeds {
        strips: Vec::new(),
        routes: Vec::new(),
        all_strips: controls::any_solo_in(current, monitor.committed)
            != controls::any_solo_in(next, monitor.next),
    };
    for change in &changes {
        match (&change.address.owner, change.descriptor.family) {
            (Owner::Strip(id), Family::StripLevel) => {
                seeds.strips.push(controls::strip_index(next, id)?);
            }
            (Owner::Vca(id), _) => {
                let vca = next.vcas.iter().position(|vca| vca.id == *id)?;
                seeds.strips.extend(next.strips_reached_by(vca));
            }
            (Owner::Route(id), Family::Route) => seeds.routes.push(route_index(next, id)?),
            _ => {}
        }
    }
    let (committed, now) = (monitor.committed.soloed(), monitor.next.soloed());
    let mut moved: Vec<&StableId> = Vec::new();
    for id in committed.iter().filter(|id| now.binary_search(id).is_err()) {
        moved.push(id);
    }
    for id in now.iter().filter(|id| committed.binary_search(id).is_err()) {
        moved.push(id);
    }
    for id in moved {
        // An ID that names no strip is no solo.
        if let Some(index) = controls::strip_index(next, id) {
            seeds.strips.push(index);
            accumulated.note_owner(index);
        }
    }
    // A master designation that moved changed the master row of the strip it left and of the
    // strip it names (#1683); it moves no level, so it seeds nothing.
    if monitor.committed.master() != monitor.next.master() {
        for id in [monitor.committed.master(), monitor.next.master()]
            .into_iter()
            .flatten()
        {
            if let Some(index) = controls::strip_index(next, id) {
                accumulated.note_owner(index);
            }
        }
    }

    // Step 5 of the full classifier, for the strips whose rows changed.
    for item in &accumulated.work {
        let index = controls::strip_index(next, &item.strip)?;
        let (before, after) = (strip_at(current, index), view.strip(index));
        if before.id != after.id || refuse_unresolvable_strip(next, &before, &after).is_err() {
            return None;
        }
    }

    let diff = if accumulated.derived {
        Some(LocalDiff::new(
            LevelRules {
                current,
                next: view,
                ramps,
                edit_ramps,
                solo_ramp: monitor.ramp,
            },
            monitor,
            seeds,
        )?)
    } else {
        None
    };
    let diff = diff.as_ref();

    // Each strip with a changed row, and each strip whose level moved, in strip order.
    let mut items: BTreeMap<usize, Work> = BTreeMap::new();
    for item in accumulated.work.drain(..) {
        items.insert(controls::strip_index(next, &item.strip)?, item);
    }
    if let Some(diff) = diff {
        for index in diff.level_changed_strips() {
            items
                .entry(index)
                .or_insert_with(|| Work::new(view.strip(index).id.clone()));
        }
    }
    // No jump-length write here (#1639 D3): the session's `control_smoothing` rows are never judged
    // on this path (`controls::changed_rows` refuses them), so the lengths are the committed ones.
    let mut delta = LiveDelta {
        changed_owners: core::mem::take(&mut accumulated.changed_owners),
        settings_changed: accumulated.settings_changed,
        derived_owners: Some(diff.map_or_else(Vec::new, |diff| {
            diff.derived_owners(next.tracks.len() + next.submixes.len())
        })),
        ..LiveDelta::default()
    };
    let level: Option<&dyn LevelDiff> = diff.map(|diff| diff as &dyn LevelDiff);
    lower_strips(
        current,
        view,
        ramps,
        edit_ramps,
        level,
        items.into_iter(),
        &mut delta,
    )
    .ok()?;
    if let (Some(diff), Some(level)) = (diff, level) {
        lower_routes(
            current,
            view,
            ramps,
            edit_ramps,
            level,
            &accumulated.routes,
            diff.route_indices(),
            &mut delta,
        )
        .ok()?;
    }
    Some(delta)
}

/// The position of the route `id` in `model.routes` (canonical route-ID order).
fn route_index(model: &SessionModel, id: &StableId) -> Option<usize> {
    model
        .routes
        .binary_search_by(|route| route.id.cmp(id))
        .ok()
        .or_else(|| model.routes.iter().position(|route| route.id == *id))
}

/// Where a row sits in the control list, as far as the classifier's grouping reads it: the owner's
/// index, then the owner's section in chain order (own rows, builtins, the pre-insert console
/// slots, the inserts, the post-insert slots), then the row.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct RowOrder {
    owner: usize,
    phase: u8,
    index: usize,
    control: u32,
}

impl RowOrder {
    fn of(next: &SessionModel, address: &ControlAddress) -> Option<Self> {
        let owner = controls::owner_index(next, &address.owner)?;
        let (phase, index) = match &address.section {
            Section::Own => (0, 0),
            Section::Builtins => (1, 0),
            Section::Console(slot) => {
                match next.console.pre_insert.iter().position(|s| s.slot == *slot) {
                    Some(index) => (2, index),
                    None => (
                        4,
                        next.console
                            .post_insert
                            .iter()
                            .position(|s| s.slot == *slot)?,
                    ),
                }
            }
            Section::Inserts(effect) => {
                let Owner::Strip(strip) = &address.owner else {
                    return None;
                };
                let strip = strip_at(next, controls::strip_index(next, strip)?);
                (
                    3,
                    strip
                        .inserts
                        .effects
                        .iter()
                        .position(|candidate| candidate.id == *effect)?,
                )
            }
        };
        Some(Self {
            owner,
            phase,
            index,
            control: address.control,
        })
    }
}

/// What the changed controls say, gathered by one body for both classifiers (step 4).
#[derive(Default)]
struct Changes {
    /// The changed controls of each strip, in the control list's order.
    work: Vec<Work>,
    /// The sends into a submix whose own rows changed.
    routes: BTreeSet<StableId>,
    /// Whether the effective values can differ: a strip-level row, a VCA input or a send's row
    /// changed (#1558 D1).
    derived: bool,
    changed_owners: Vec<usize>,
    settings_changed: bool,
}

impl Changes {
    /// Marks the owner at list position `position` changed.
    fn note_owner(&mut self, position: usize) {
        if let Err(at) = self.changed_owners.binary_search(&position) {
            self.changed_owners.insert(at, position);
        }
    }

    /// Takes one changed control into account.
    ///
    /// # Errors
    ///
    /// [`LiveRebuild::RebuildOnly`] for a rebuild-only row, [`LiveRebuild::Structure`] for a
    /// changed control no lowering carries.
    fn note(&mut self, next: &SessionModel, change: &ChangedControl) -> Result<(), LiveRebuild> {
        let descriptor = &change.descriptor;
        // The changes come in list order, but the VCA rows follow the session's.
        if let Some(position) = controls::owner_index(next, &change.address.owner) {
            self.note_owner(position);
        }
        self.settings_changed |= descriptor.family == Family::Setting;
        match descriptor.class {
            ControlClass::RebuildOnly { reason, .. } => {
                return Err(LiveRebuild::RebuildOnly(reason));
            }
            // `changed_controls` skips a derived row: the effective values follow its inputs.
            ControlClass::Derived => return Ok(()),
            ControlClass::Live => {}
        }
        if descriptor.family == Family::Setting {
            return Ok(());
        }
        let strip = match &change.address.owner {
            Owner::Strip(strip) => strip,
            // A live route row is a send into a submix (#1557); an output route's rows are
            // rebuild-only above.
            Owner::Route(route) if descriptor.family == Family::Route => {
                self.routes.insert(route.clone());
                self.derived = true;
                return Ok(());
            }
            // A VCA row reaches render through its members' effective values.
            Owner::Vca(_) if descriptor.family == Family::StripLevel => {
                self.derived = true;
                return Ok(());
            }
            _ => return Err(LiveRebuild::Structure),
        };
        if self.work.last().is_none_or(|last| last.strip != *strip) {
            self.work.push(Work::new(strip.clone()));
        }
        let item = self.work.last_mut().expect("a work item was just pushed");
        match descriptor.family {
            // The records come from the effective values, which this row's change moves.
            Family::StripLevel => self.derived = true,
            Family::StripMatrix => item.matrix = true,
            Family::Effect => {
                if item.instances.last() != Some(&change.address.section) {
                    item.instances.push(change.address.section.clone());
                }
            }
            Family::StripInput => item.input = true,
            // A route's row has a route owner, handled above.
            Family::Route => return Err(LiveRebuild::Structure),
            Family::Setting => {}
        }
        Ok(())
    }
}

/// Step 6 for strips: the records of each `(index, changed controls)` item, in the order given
/// (strip order), into `delta`. `index` counts [`SessionModel::strips`] in both models.
fn lower_strips<'a>(
    current: &SessionModel,
    next: ModelView<'a>,
    ramps: LiveRamps,
    edit_ramps: &EditRamps,
    diff: Option<&dyn LevelDiff>,
    items: impl Iterator<Item = (usize, Work)>,
    delta: &mut LiveDelta<'a>,
) -> Result<(), LiveRebuild> {
    for (index, mut item) in items {
        let after = next.strip(index);
        item.level = diff.is_some_and(|diff| diff.level_changed(index));
        if !(item.level || item.matrix || item.input || !item.instances.is_empty()) {
            continue;
        }
        let before = strip_of(current, &item.strip).ok_or(LiveRebuild::Structure)?;
        let ctx = Context {
            current,
            next,
            ramps,
            edit: StripEditRamps::of(edit_ramps, &item.strip),
            diff,
        };
        let mut records = LiveStripRecords {
            strip_id: after.id.as_str(),
            fader: [None; 4],
            offsets: [None; 2],
            matrix: None,
            input: Vec::new(),
        };
        let families = item
            .input
            .then_some(Family::StripInput)
            .into_iter()
            .chain(item.level.then_some(Family::StripLevel))
            .chain(item.matrix.then_some(Family::StripMatrix));
        for family in families {
            let subject = Subject::Strip(Target {
                before,
                after,
                index,
                instance: None,
            });
            match lower(family, &ctx, &subject)? {
                Lowered::Fader(level) => {
                    records.fader = level.fader;
                    records.offsets = level.offsets;
                }
                Lowered::Matrix(matrix) => records.matrix = matrix,
                Lowered::Input(input) => records.input = input,
                Lowered::Effect(_) | Lowered::Route(_) | Lowered::Nothing => {}
            }
        }
        if records.fader[0].is_some()
            || records.offsets.iter().any(Option::is_some)
            || records.matrix.is_some()
            || !records.input.is_empty()
        {
            delta.strips.push(records);
        }
        for section in &item.instances {
            let subject = Subject::Strip(Target {
                before,
                after,
                index,
                instance: Some(section),
            });
            if let Lowered::Effect(Some(effect)) = lower(Family::Effect, &ctx, &subject)? {
                delta.effects.push(effect);
            }
        }
    }
    Ok(())
}

/// Step 6 for sends: the record of each route in `indices` (ascending positions in
/// `model.routes`) into a submix whose own rows changed (`routes`) or whose effective gate changed.
#[expect(
    clippy::too_many_arguments,
    reason = "the two models and the lowering's inputs"
)]
fn lower_routes<'a>(
    current: &SessionModel,
    next: ModelView<'a>,
    ramps: LiveRamps,
    edit_ramps: &EditRamps,
    diff: &dyn LevelDiff,
    routes: &BTreeSet<StableId>,
    indices: impl Iterator<Item = usize>,
    delta: &mut LiveDelta<'a>,
) -> Result<(), LiveRebuild> {
    let ctx = Context {
        current,
        next,
        ramps,
        edit: StripEditRamps::default(),
        diff: Some(diff),
    };
    for index in indices {
        if index >= next.structure().routes.len() {
            return Err(LiveRebuild::Structure);
        }
        let after = next.route(index);
        let before = current
            .routes
            .get(index)
            .filter(|before| before.id == after.id)
            .ok_or(LiveRebuild::Structure)?;
        if !matches!(after.destination, RouteDestination::SubmixInput { .. }) {
            continue;
        }
        let gates = diff.gates(index).ok_or(LiveRebuild::Structure)?;
        if !(routes.contains(&after.id) || gates[0] != gates[1]) {
            continue;
        }
        let edit = RouteEditRamps::of(edit_ramps, &after.id);
        let gate_length = if gates[0] == gates[1] {
            0
        } else {
            diff.gate_length(before, after, edit.mute, gates)
                .ok_or(LiveRebuild::Structure)?
        };
        let subject = Subject::Route(RouteTarget {
            before,
            after,
            gates,
            gate_length,
            edit,
        });
        if let Lowered::Route(Some(record)) = lower(Family::Route, &ctx, &subject)? {
            delta.routes.push(record);
        }
    }
    Ok(())
}

/// What the lowering asks of the effective values of the two models: the dense form of every strip
/// and send ([`EffectiveDiff`]) or the local form of the strips and sends an edit reaches
/// ([`LocalDiff`]). Both answer from the same rules.
trait LevelDiff {
    /// Whether strip `strip`'s effective fader or mute differs, as bits, between the models.
    fn level_changed(&self, strip: usize) -> bool;
    /// The level records of strip `strip`, with its offsets-cell writes (#1649 D4).
    fn level_records(&self, strip: usize) -> Result<LevelRecords, OutOfDomain>;
    /// The effective gate of route `route` per source lane, in `[current, next]`, or `None` when
    /// the form does not hold it.
    fn gates(&self, route: usize) -> Option<[[bool; 2]; 2]>;
    /// The ramp length of a route record whose gate changed (#1558 D7), or `None` when the form
    /// does not hold the route's source strip.
    fn gate_length(
        &self,
        before: &Route,
        after: &Route,
        own_edit: Option<u32>,
        gates: [[bool; 2]; 2],
    ) -> Option<u32>;
}

impl LevelDiff for EffectiveDiff<'_> {
    fn level_changed(&self, strip: usize) -> bool {
        EffectiveDiff::level_changed(self, strip, strip)
    }

    fn level_records(&self, strip: usize) -> Result<LevelRecords, OutOfDomain> {
        EffectiveDiff::strip_level_records(self, strip, strip)
    }

    fn gates(&self, route: usize) -> Option<[[bool; 2]; 2]> {
        Some(EffectiveDiff::gates(self, route))
    }

    fn gate_length(
        &self,
        before: &Route,
        after: &Route,
        own_edit: Option<u32>,
        gates: [[bool; 2]; 2],
    ) -> Option<u32> {
        let source = self.source_strips(before, after)?;
        Some(EffectiveDiff::gate_length(
            self, before, after, own_edit, source, gates,
        ))
    }
}

/// What an edit reaches, for [`LocalDiff`].
struct LocalSeeds {
    /// Strips whose effective level can differ.
    strips: Vec<usize>,
    /// Sends whose own row changed.
    routes: Vec<usize>,
    /// Every strip's mute can differ: whether some strip is soloed differs between the overlays.
    all_strips: bool,
}

/// One strip's effective values in both models, with what the ramp rules read.
struct LocalStrip {
    index: usize,
    before: EffectiveStrip,
    after: EffectiveStrip,
    reach_before: Vec<usize>,
    reach: Vec<usize>,
    solo_before: bool,
    solo_after: bool,
}

/// The effective values of **the strips and sends an edit reaches**, in both models (#1612 D6): the
/// rules of [`EffectiveDiff`], computed for the strips whose own level, VCA, solo or following send
/// moved and for nothing else. A strip or send it does not hold has unchanged effective values.
struct LocalDiff<'a> {
    rules: LevelRules<'a>,
    /// Ascending by strip index.
    strips: Vec<LocalStrip>,
    /// Ascending by route index: the gate per source lane in `[current, next]`.
    routes: Vec<(usize, [[bool; 2]; 2])>,
}

impl<'a> LocalDiff<'a> {
    /// `None` when a strip or send of the seeds is not in both models.
    fn new(rules: LevelRules<'a>, monitor: MonitorBase<'_>, seeds: LocalSeeds) -> Option<Self> {
        let (current, view) = (rules.current, rules.next);
        let next = view.structure();
        let strip_count = next.tracks.len() + next.submixes.len();
        let any_before = controls::any_solo_in(current, monitor.committed);
        let any_after = controls::any_solo_in(next, monitor.next);

        let mut route_indices = seeds.routes;
        route_indices.sort_unstable();
        route_indices.dedup();
        let mut indices = if seeds.all_strips {
            (0..strip_count).collect()
        } else {
            seeds.strips
        };
        // A send's source strip is held with the send.
        for &route in &route_indices {
            indices.push(route_source_strip(next, next.routes.get(route)?)?);
        }
        indices.sort_unstable();
        indices.dedup();

        let mut strips = Vec::with_capacity(indices.len());
        // The VCA tree of each model, built once for all the strips the edit reaches.
        let (nesting_before, nesting_after) = (OnceCell::new(), OnceCell::new());
        for index in indices {
            let before = LocalInputs::of_nested(
                ModelView::whole(current),
                monitor.committed,
                index,
                &nesting_before,
            )?;
            let after = LocalInputs::of_nested(view, monitor.next, index, &nesting_after)?;
            strips.push(LocalStrip {
                index,
                before: controls::effective_strip(&before, index, any_before),
                after: controls::effective_strip(&after, index, any_after),
                solo_before: controls::solo_term(&before, index, any_before),
                solo_after: controls::solo_term(&after, index, any_after),
                reach_before: before.into_reach(),
                reach: after.into_reach(),
            });
        }

        // The sends that follow a strip whose mute moved: one scan comparing sources, and only when
        // a mute moved.
        if strips
            .iter()
            .any(|strip| strip.before.mute != strip.after.mute)
        {
            for (index, route) in next.routes.iter().enumerate() {
                if !route.follows_mute {
                    continue;
                }
                let Some(source) = route_source_strip(next, route) else {
                    continue;
                };
                if let Ok(at) = strips.binary_search_by_key(&source, |strip| strip.index)
                    && strips[at].before.mute != strips[at].after.mute
                {
                    route_indices.push(index);
                }
            }
            route_indices.sort_unstable();
            route_indices.dedup();
        }

        let mut routes = Vec::with_capacity(route_indices.len());
        for index in route_indices {
            let (before, after) = (current.routes.get(index)?, view.route(index));
            let source = route_source_strip(next, after)?;
            if before.id != after.id || route_source_strip(current, before)? != source {
                return None;
            }
            let at = strips
                .binary_search_by_key(&source, |strip| strip.index)
                .ok()?;
            let gate = |model: ModelView<'_>, overlay, effective: &EffectiveStrip| {
                let inputs = LocalInputs::of(model, overlay, source)?;
                Some(controls::effective_gate(&inputs, index, effective))
            };
            routes.push((
                index,
                [
                    gate(
                        ModelView::whole(current),
                        monitor.committed,
                        &strips[at].before,
                    )?,
                    gate(view, monitor.next, &strips[at].after)?,
                ],
            ));
        }
        Some(Self {
            rules,
            strips,
            routes,
        })
    }

    fn strip(&self, index: usize) -> Option<&LocalStrip> {
        self.strips
            .binary_search_by_key(&index, |strip| strip.index)
            .ok()
            .map(|at| &self.strips[at])
    }

    fn change<'s>(&'s self, strip: &'s LocalStrip) -> StripChange<'s> {
        let after = self.rules.next.strip(strip.index);
        StripChange {
            own_before: strip_at(self.rules.current, strip.index).fader,
            own_after: after.fader,
            offset_before: strip.before.fader_offset,
            offset_after: strip.after.fader_offset,
            id: after.id,
            reach_before: &strip.reach_before,
            reach: &strip.reach,
            solo_before: strip.solo_before,
            solo_after: strip.solo_after,
        }
    }

    /// The strips whose effective fader or mute differs between the models, ascending.
    fn level_changed_strips(&self) -> Vec<usize> {
        self.strips
            .iter()
            .filter(|strip| level_differs(&strip.before, &strip.after))
            .map(|strip| strip.index)
            .collect()
    }

    /// The sends the diff holds, ascending.
    fn route_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.routes.iter().map(|(index, _)| *index)
    }

    /// The owners whose derived rows the diff reaches, in list order: the strips it holds, then
    /// its sends as `strip_count + route index`.
    fn derived_owners(&self, strip_count: usize) -> Vec<usize> {
        self.strips
            .iter()
            .map(|strip| strip.index)
            .chain(self.route_indices().map(|route| strip_count + route))
            .collect()
    }
}

impl LevelDiff for LocalDiff<'_> {
    fn level_changed(&self, strip: usize) -> bool {
        self.strip(strip)
            .is_some_and(|strip| level_differs(&strip.before, &strip.after))
    }

    fn level_records(&self, strip: usize) -> Result<LevelRecords, OutOfDomain> {
        let strip = self.strip(strip).ok_or(OutOfDomain)?;
        let change = self.change(strip);
        level_records_between(
            &strip.before,
            &strip.after,
            self.rules.lengths(&change),
            || controls::automated_fader_lanes(self.rules.next.structure(), change.id),
        )
    }

    fn gates(&self, route: usize) -> Option<[[bool; 2]; 2]> {
        self.routes
            .binary_search_by_key(&route, |(index, _)| *index)
            .ok()
            .map(|at| self.routes[at].1)
    }

    fn gate_length(
        &self,
        before: &Route,
        after: &Route,
        own_edit: Option<u32>,
        gates: [[bool; 2]; 2],
    ) -> Option<u32> {
        let source = self.strip(route_source_strip(self.rules.next.structure(), after)?)?;
        Some(
            self.rules
                .gate_length(before, after, own_edit, &self.change(source), gates),
        )
    }
}

/// The changed controls of one strip, from the control list's order.
struct Work {
    strip: StableId,
    /// A fader or mute row changed.
    level: bool,
    /// A pan, matrix or matrix-mode row changed.
    matrix: bool,
    /// A trim, polarity, HPF or LPF row changed (#1556).
    input: bool,
    /// The effect sections with a changed row, in chain order.
    instances: Vec<Section>,
}

impl Work {
    fn new(strip: StableId) -> Self {
        Self {
            strip,
            level: false,
            matrix: false,
            input: false,
            instances: Vec::new(),
        }
    }
}

/// The strip `id` of `model` (a track or a submix).
fn strip_of<'m>(model: &'m SessionModel, id: &StableId) -> Option<StripRef<'m>> {
    controls::strip_index(model, id).map(|index| strip_at(model, index))
}

/// What a lowering reads besides its target.
struct Context<'m> {
    current: &'m SessionModel,
    next: ModelView<'m>,
    ramps: LiveRamps,
    edit: StripEditRamps,
    /// The effective values of both models; present whenever a derived value can differ.
    diff: Option<&'m dyn LevelDiff>,
}

/// What a lowering is of: a strip (or one of its effect instances), or a route.
enum Subject<'s, 'm, 'a> {
    Strip(Target<'s, 'm, 'a>),
    Route(RouteTarget<'m, 'a>),
}

/// A route into a submix in both models.
struct RouteTarget<'m, 'a> {
    before: &'m Route,
    after: &'a Route,
    /// The route's effective gate per source lane, in `[before, after]` (`controls::effective`).
    gates: [[bool; 2]; 2],
    /// The ramp length of a record whose gate changed (#1558 D7); unused when it did not.
    gate_length: u32,
    edit: RouteEditRamps,
}

/// What a strip lowering is of: a strip in both models, or one effect instance of it.
struct Target<'s, 'm, 'a> {
    before: StripRef<'m>,
    after: StripRef<'a>,
    /// The strip's index in [`SessionModel::strips`], the same in both models.
    index: usize,
    /// The effect section, for the [`Family::Effect`] lowering.
    instance: Option<&'s Section>,
}

/// What one lowering produced.
enum Lowered<'a> {
    Fader(LevelRecords),
    Matrix(Option<TrackControlRecord>),
    Input(Vec<TrackInputRecord>),
    Effect(Option<LiveEffectRecords<'a>>),
    Route(Option<LiveRouteRecord<'a>>),
    Nothing,
}

#[cfg(feature = "test-support")]
thread_local! {
    static LOWERED_OWNERS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

/// How many owners [`classify_live_delta`] has lowered on this thread since the last call, and
/// resets the count (#1554 D7). One call of the lowering per owner and family counts one, so a
/// test can show that one edit lowers one owner and an unchanged owner is never lowered.
#[cfg(feature = "test-support")]
pub fn take_lowered_owner_count() -> usize {
    LOWERED_OWNERS.with(|count| count.replace(0))
}

/// Lowers one changed owner of one family to its render records: the one lowering of each family
/// (decision 17, C6). A family whose rows are live decides here what a change of them costs the
/// plan; the classifier above decides only that it is live.
///
/// A strip family given a route subject, or the reverse, is a classifier fault and refuses as a
/// structure change.
fn lower<'a>(
    family: Family,
    ctx: &Context<'_>,
    subject: &Subject<'_, '_, 'a>,
) -> Result<Lowered<'a>, LiveRebuild> {
    #[cfg(feature = "test-support")]
    LOWERED_OWNERS.with(|count| count.set(count.get() + 1));
    let target = match (family, subject) {
        (Family::Route, Subject::Route(route)) => {
            return route_records(ctx, route).map(Lowered::Route);
        }
        (Family::Setting, _) => return Ok(Lowered::Nothing),
        (Family::Route, Subject::Strip(_)) | (_, Subject::Route(_)) => {
            return Err(LiveRebuild::Structure);
        }
        (_, Subject::Strip(target)) => target,
    };
    let (before, after) = (&target.before, &target.after);
    match family {
        // The strip's effective values, never its raw ones (#1558 D1).
        Family::StripLevel => ctx
            .diff
            .ok_or(LiveRebuild::Structure)?
            .level_records(target.index)
            .map(Lowered::Fader)
            .map_err(|_| LiveRebuild::Domain),
        Family::StripMatrix => matrix_record(
            before.matrix_or_pan,
            after.matrix_or_pan,
            ctx.ramps,
            ctx.edit,
        )
        .map(Lowered::Matrix)
        .map_err(|_| LiveRebuild::Domain),
        Family::Effect => {
            let section = target.instance.ok_or(LiveRebuild::Structure)?;
            effect_records(ctx, target, section).map(Lowered::Effect)
        }
        Family::StripInput => input_records(
            before.builtins,
            after.builtins,
            ctx.next.structure().sample_rate_hz,
            ctx.ramps,
            ctx.edit,
        )
        .map(Lowered::Input)
        .map_err(|_| LiveRebuild::Domain),
        Family::Setting | Family::Route => Ok(Lowered::Nothing),
    }
}

/// The record that brings a send from `before`'s live values to `after`'s (#1557 D2, D3), when the
/// target the render plane holds changes: [`route_record_between`] of the lowering's inputs, the
/// one function of a send (#1284 D3), which a successor's preparation asks too.
///
/// # Errors
///
/// [`LiveRebuild::Domain`] when either value set is outside the route domain, or a ramp is longer
/// than the cell's maximum.
fn route_records<'a>(
    ctx: &Context<'_>,
    target: &RouteTarget<'_, 'a>,
) -> Result<Option<LiveRouteRecord<'a>>, LiveRebuild> {
    Ok(route_record_between(
        ctx.ramps,
        target.before,
        target.after,
        target.gates,
        target.gate_length,
        target.edit,
    )?
    .map(|record| LiveRouteRecord {
        route_id: target.after.id.as_str(),
        record,
    }))
}

/// Step 5: a changed effect's `params` must resolve, as preparation resolves them. An unknown
/// parameter ID, a unit mismatch or a duplicate lane has no control row to move, yet a rebuild
/// refuses it, so a live commit must not ack it.
fn refuse_unresolvable_params(
    current: &SessionModel,
    next: &SessionModel,
) -> Result<(), LiveRebuild> {
    for (before, after) in current.strips().zip(next.strips()) {
        refuse_unresolvable_strip(next, &before, &after)?;
    }
    Ok(())
}

/// [`refuse_unresolvable_params`] for one strip, as it is in `current` (`before`) and in `next`
/// (`after`).
fn refuse_unresolvable_strip(
    next: &SessionModel,
    before: &StripRef<'_>,
    after: &StripRef<'_>,
) -> Result<(), LiveRebuild> {
    for (prior, entry) in before.console.iter().zip(after.console) {
        if !same_params(&prior.params, &entry.params) {
            let slot = next
                .console
                .slots()
                .find(|slot| slot.slot == entry.slot)
                .ok_or(LiveRebuild::Structure)?;
            resolves(&slot.identity, &entry.params)?;
        }
    }
    for (prior, effect) in before.inserts.effects.iter().zip(&after.inserts.effects) {
        if !same_params(&prior.params, &effect.params) {
            resolves(&effect.identity, &effect.params)?;
        }
    }
    Ok(())
}

/// Whether `params` resolve against the descriptor of `identity`.
fn resolves(identity: &EffectIdentity, params: &[EffectParam]) -> Result<(), LiveRebuild> {
    // A third-party or unknown identity never prepared; the rebuild reports it.
    let EffectIdentity::Native { effect_id } = identity else {
        return Err(LiveRebuild::Structure);
    };
    let registry = launch_native_effect_registry().map_err(|_| LiveRebuild::Structure)?;
    let factory = registry
        .get_shared_ascii(effect_id.as_str())
        .ok_or(LiveRebuild::Structure)?;
    resolve_initial_values(factory.descriptor(), params)
        .map(|_| ())
        .map_err(|_| LiveRebuild::Domain)
}

/// The index of the instance `section` in a strip's three lowered racks, as its live address and
/// its `(rack, position)` in chain order.
fn locate(
    racks: &LoweredRacks<'_>,
    section: &Section,
) -> Option<(LiveEffectAddress, usize, usize)> {
    let [pre, inserts, _] = racks.in_chain_order();
    match section {
        Section::Console(slot) => {
            let position = |rack: &[Effect]| rack.iter().position(|effect| effect.id == *slot);
            position(pre)
                .map(|index| (LiveEffectAddress::console(index as u32), 0, index))
                .or_else(|| {
                    position(&racks.post_insert).map(|index| {
                        (
                            LiveEffectAddress::console((pre.len() + index) as u32),
                            2,
                            index,
                        )
                    })
                })
        }
        Section::Inserts(effect) => inserts
            .iter()
            .position(|candidate| candidate.id == *effect)
            .map(|index| (LiveEffectAddress::insert(index as u32), 1, index)),
        Section::Own | Section::Builtins => None,
    }
}

/// The bypass and parameter records of one effect instance whose `bypass` or `params` differ
/// (#1264 D2, #1266 D1): [`effect_instance_records`] for the instance's lowered pair, with its
/// live address. The rows have proved that each changed value is live; the function is the same
/// one a successor's join asks (#1280 D1, D4), so the live path and a carry cannot disagree.
fn effect_records<'a>(
    ctx: &Context<'_>,
    target: &Target<'_, '_, 'a>,
    section: &Section,
) -> Result<Option<LiveEffectRecords<'a>>, LiveRebuild> {
    let racks_before = ctx.current.lower_strip(&target.before);
    let racks_after = ctx.next.structure().lower_strip(&target.after);
    let (address, rack, index) = locate(&racks_after, section).ok_or(LiveRebuild::Structure)?;
    let (address_before, rack_before, index_before) =
        locate(&racks_before, section).ok_or(LiveRebuild::Structure)?;
    if (address, rack, index) != (address_before, rack_before, index_before) {
        return Err(LiveRebuild::Structure);
    }
    let effect_before = &racks_before.in_chain_order()[rack][index];
    let effect_after = &racks_after.in_chain_order()[rack][index];
    Ok(effect_instance_records(
        effect_before,
        effect_after,
        ctx.next.structure().sample_rate_hz,
        ctx.ramps,
    )?
    .map(|instance| LiveEffectRecords {
        strip_id: target.after.id.as_str(),
        address,
        records: instance.records,
        targets: instance.targets,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use session::{LinkMode, parse_session_json};

    const FIXTURE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

    /// The lowering of the fixture's insert `eq` between the fixture and `edit` applied to it.
    fn insert_records(edit: impl FnOnce(&mut Effect)) -> Result<Option<usize>, LiveRebuild> {
        let current = parse_session_json(FIXTURE).expect("fixture parses");
        let mut next = current.clone();
        let eq = StableId::parse("eq").expect("stable id");
        edit(
            next.tracks[0]
                .inserts
                .effects
                .iter_mut()
                .find(|effect| effect.id == eq)
                .expect("the fixture's insert"),
        );
        let (current, next) = (&current, &next);
        let ctx = Context {
            current,
            next: ModelView::whole(next),
            ramps: LiveRamps::for_session(next),
            edit: StripEditRamps::default(),
            diff: None,
        };
        let target = Target {
            before: strip_of(current, &next.tracks[0].id).expect("strip"),
            after: strip_of(next, &next.tracks[0].id).expect("strip"),
            index: 0,
            instance: None,
        };
        effect_records(&ctx, &target, &Section::Inserts(eq))
            .map(|records| records.map(|records| records.records.len()))
    }

    /// Batch 2 review fold item 2. Red if a changed effect row that is neither the bypass nor a
    /// parameter (the link mode) lowers to no record: a live classification of it would be acked
    /// and dropped, and the next row made live would repeat the bug. A bypass flip stays one
    /// record, so the guard is not a blanket refusal.
    #[test]
    fn a_changed_effect_row_without_a_record_is_a_rebuild() {
        assert!(matches!(
            insert_records(|effect| effect.link_mode = LinkMode::Maximum),
            Err(LiveRebuild::Structure)
        ));
        assert_eq!(
            insert_records(|effect| effect.bypass = !effect.bypass),
            Ok(Some(1))
        );
    }
}
