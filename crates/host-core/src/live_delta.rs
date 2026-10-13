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
//! a new arm of the lowering (the input records are #1556's; route records #1557, derived values #1558); it
//! does not add a second classifier.

use crate::control_preparation::{
    EQ_TARGET_CAPACITY, EQ_VALUE_COUNT, EqTargetEdit, EqTargetPreparer, EqTargetPreparerError,
    INPUT_FILTER_TARGET_CAPACITY, InputFilterEdit, InputFilterPreparer,
};
use builtins::{
    BuiltinLaneSelector, PreparedInputFilterTarget, checked_trim_gain, validate_input_filter_pair,
    validate_prepared_input_filter_target,
};
use builtins_compiler::{TrackControlRecord, TrackFaderRecord, TrackInputRecord};
use controls::{ControlClass, Family, Owner, RebuildReason, Section};
use effect_compiler::{
    LiveEffectAddress, effect_automation_diagnostics, launch_native_effect_registry,
    resolve_initial_values,
};
use effect_contract::{
    AutomationRate, EffectControlRecord, NativeEffectFactory, PREPARED_EFFECT_TARGET_WORDS,
    ParameterChannel, PreparedEffectTarget,
};
use session::{
    EditRamps, Effect, EffectIdentity, EffectParam, LoweredRacks, SessionModel, StableId, StripRef,
    canonical_session_json,
};

// The ramp rule and the per-owner record derivation live with preparation, which needs them on
// every build (a successor's carry-then-retarget, #1277 D5); the classifier calls them.
use crate::prepare::{LiveRampRow, LiveRamps};
use crate::prepare::{
    StripEditRamps, fader_records, followed_mute_changes, lane_records, matrix_record,
};

/// The records of one strip whose live values change.
#[derive(Clone, Debug, PartialEq)]
pub struct LiveStripRecords<'a> {
    /// The strip's ID, borrowed from the post-commit model.
    pub strip_id: &'a str,
    /// For the strip's fader/mute queue, in push order: `FaderDb` records first, then `Mute`
    /// records, each left before right; `None` after the last one.
    pub fader: [Option<TrackFaderRecord>; 4],
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
    /// One [`EffectControlRecord::Bypass`] first when the instance's bypass changes (#1266 D1),
    /// then one [`EffectControlRecord::Parameter`] per `(parameter_index, channel)` whose resolved
    /// value changes, in descriptor order, `Left` before `Right`; never empty.
    ///
    /// For an instance with `targets` (the EQ) the `Parameter` records are its owner's edits
    /// (`EffectControlProducer::edit_owner`), never written as they are: the cells get the targets.
    /// Its `Bypass` record, like every other instance's, is written as it is, ahead of the
    /// targets.
    pub records: Vec<EffectControlRecord>,
    /// For an effect whose parameters ride prepared targets (the parametric EQ, #1265): the
    /// targets [`EqTargetPreparer`] designed on the control thread from the instance's pre-commit
    /// values and `records`, in the order its owner publishes them
    /// (`EffectControlProducer::publish_candidate_targets`). `None` for every other effect, whose
    /// `records` are written as they are.
    pub targets: Option<Vec<PreparedEffectTarget>>,
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
}

/// Why a delta needs a plan rebuild.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveRebuild {
    /// Either model declares a VCA (#1053 G3, until #1558).
    Vca,
    /// The models differ in something that is not a control value: the strip, route, slot or
    /// insert set or order, an effect's identity, quality or sidechain, a profile's other fields,
    /// or any other field no control row describes.
    Structure,
    /// A fader or pan/matrix value is outside the domain its render-side setter accepts, or an
    /// effect's pre- or post-commit `params` do not resolve. The rebuild then reports the same
    /// preparation diagnostic it always has.
    Domain,
    /// A control whose row is rebuild-only changes, for the reason its row states (decision 17,
    /// C3). This replaces the former `Prepared` and `PreparedBypass`: an effect parameter the plan
    /// keeps prepared (an EQ band's `enabled` and `kind` among them, decision 14 F2), the bypass
    /// of the delay and the multiband compressor (decision 14 F4), an input value, a route value.
    RebuildOnly(RebuildReason),
    /// A strip's mute changes while a route with `follows_mute` sends from it (#1053 G2, until
    /// #1558).
    FollowedMute,
    /// The stored automation changes and `next` automates an effect parameter that the plan keeps
    /// prepared ([`effect_automation_diagnostics`] is not empty, decision 15 E1, #1335 D4). The
    /// rebuild's preparation refuses it with `effect.automation.rate`; the classifier only
    /// routes, so the refusal has one author and no live commit can ack it first.
    AutomationTarget,
}

/// Classifies the delta from `current` to `next` as live records or a rebuild (#1053 D1, #1554).
///
/// Both inputs must be **normalized** models (`CompiledSession::normalized_model()`): the
/// committed session's and the transaction's prospective one. The structure comparison relies on
/// normalized order.
///
/// The control rows (`controls`, decision 17) say what a value is and how it changes; this
/// function keeps no list of fields. The steps, in order:
/// 1. [`LiveRebuild::Vca`] if either model declares a VCA.
/// 2. [`LiveRebuild::Structure`] if `next`, with `current`'s revision, session ID, render- and
///    output-profile IDs and stored automation, and with every control value of
///    [`controls::mask_values`] copied from `current`, has canonical JSON bytes other than
///    `current`'s. Bytes, never `PartialEq`: the canonical `f32` spelling keeps a zero's sign. A
///    canonical JSON error is structural too.
/// 3. [`LiveRebuild::AutomationTarget`] if `current.automation != next.automation` and
///    [`effect_automation_diagnostics`] refuses one of `next`'s automations (decision 15 E1,
///    #1335 D4). An unchanged automation table is never re-checked: it was prepared. An accepted
///    change stays live with no records, as #1260 D2 made it.
/// 4. With the structure equal, the two models list the same controls in the same order. For each
///    control whose value bits differ ([`controls::changed_controls`]): a row that is
///    rebuild-only is [`LiveRebuild::RebuildOnly`] with the row's reason; a row of family
///    `Setting` gives no record; a live row marks its owner changed for its family.
/// 5. [`LiveRebuild::Domain`] if a changed effect's `params` do not resolve through
///    [`resolve_initial_values`] (the one function preparation uses), whether or not a row's value
///    moved: an unknown parameter or a unit mismatch has no row.
/// 6. Each changed owner is lowered once per family (`lower`): a strip's input trim, polarity and
///    filters (#1556), a strip's fader and mute, a strip's pan or matrix, an effect instance's
///    bypass and parameters. An unchanged owner is
///    never lowered. A lowering's domain refusal is [`LiveRebuild::Domain`]; a strip whose mute
///    changes while a `follows_mute` route sends from it is [`LiveRebuild::FollowedMute`].
///
/// The session ID, the two profile IDs, the stored automation and `control_smoothing` are
/// model-only: no prepared plan reads them (#1260, #1365), so a delta that changes only them is
/// live with no records, once step 3 has accepted the automation's targets. `control_smoothing`'s
/// only reader is [`LiveRamps::for_session`], which the caller evaluates on `next`: the records
/// of the same transaction use the new lengths, and a ramp already in flight keeps its own. A
/// preparation that bakes a ramp length into a plan (a transition decided at preparation) reads
/// the key at that preparation only, so a later edit still governs only later transitions. The
/// first such slice is a successor's carry retarget (#1277 D5): its fader, mute and matrix records
/// are written to the successor's cells at preparation, with the lengths of the successor's own
/// model (and the transaction's per-edit lengths, #1394 D6). Masking
/// `automation` is correct only while no host renders stored automation: the first issue that
/// renders it (#1058) must remove it from the mask, or an automation edit would commit without
/// reaching the running plan.
///
/// A record is emitted only when the value the render plane holds changes (`solo.rs`: never a
/// redundant record): a `FaderDb` per lane whose gain bits change (one `Both` when both change to
/// the same dB bits), a `Mute` per lane whose mute changes (one `Both` when both change to the
/// same value), a matrix record when a lowered coefficient's bits change, and the strip's input
/// records (a `TrimDb` and a `PolarityInvert` by the same rules, then the filter targets
/// [`InputFilterPreparer`] designs for the changed cutoffs, #1556). A fader move on a
/// lane that stays muted still gets its record, so the stage remembers the gain for a later
/// unmute. An effect record is emitted only for a resolved value whose bits change: a `Both`
/// value rewritten as equal per-lane values gives none.
///
/// A record's ramp length is [`LiveRamps::resolve`]'s: the length the transaction's own edit gave
/// the record's `(strip, row)` in `edit_ramps` (an explicit 0 included), else the session's
/// `ramps` for the row (#1394 D5, D6). A pan or matrix record without an edit length takes the
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
    if !current.vcas.is_empty() || !next.vcas.is_empty() {
        return Err(LiveRebuild::Vca);
    }
    // Step 2. The control values are the rows' (decision 17, C6); the model-only fields are the
    // classifier's own.
    let mut masked = next.clone();
    controls::mask_values(&mut masked, current);
    masked.revision = current.revision;
    // #1260 D1: fields no prepared plan reads. Each profile's other fields stay compared.
    masked.session_id = current.session_id.clone();
    masked.render_profile.id = current.render_profile.id.clone();
    masked.output_profile.id = current.output_profile.id.clone();
    // #1260 D2: correct only while no host renders stored automation (#1058). The first issue
    // that renders it must drop this line.
    masked.automation = current.automation.clone();
    match (
        canonical_session_json(current),
        canonical_session_json(&masked),
    ) {
        (Ok(before), Ok(after)) if before.as_bytes() == after.as_bytes() => {}
        _ => return Err(LiveRebuild::Structure),
    }
    drop(masked);

    // Step 3.
    if current.automation != next.automation {
        let registry = launch_native_effect_registry().map_err(|_| LiveRebuild::Structure)?;
        if !effect_automation_diagnostics(next, registry).is_empty() {
            return Err(LiveRebuild::AutomationTarget);
        }
    }

    // Step 4.
    let changes = controls::changed_controls(current, next, next.sample_rate_hz)
        .ok_or(LiveRebuild::Structure)?;
    let mut work: Vec<Work> = Vec::new();
    for change in &changes {
        let descriptor = &change.descriptor;
        match descriptor.class {
            ControlClass::RebuildOnly { reason, .. } => {
                return Err(LiveRebuild::RebuildOnly(reason));
            }
            // `changed_controls` skips a derived row; #1558 lowers it.
            ControlClass::Derived => continue,
            ControlClass::Live => {}
        }
        if descriptor.family == Family::Setting {
            continue;
        }
        let Owner::Strip(strip) = &change.address.owner else {
            // A route or a VCA row is rebuild-only or derived in this slice.
            return Err(LiveRebuild::Structure);
        };
        if work.last().is_none_or(|last| last.strip != *strip) {
            work.push(Work::new(strip.clone()));
        }
        let item = work.last_mut().expect("a work item was just pushed");
        match descriptor.family {
            Family::StripLevel => item.level = true,
            Family::StripMatrix => item.matrix = true,
            Family::Effect => {
                if item.instances.last() != Some(&change.address.section) {
                    item.instances.push(change.address.section.clone());
                }
            }
            Family::StripInput => item.input = true,
            Family::Route => item.unlowered = Some(Family::Route),
            Family::Setting => {}
        }
    }

    // Step 5.
    refuse_unresolvable_params(current, next)?;

    // Step 6.
    let mut delta = LiveDelta::default();
    for item in &work {
        let before = strip_of(current, &item.strip).ok_or(LiveRebuild::Structure)?;
        let after = strip_of(next, &item.strip).ok_or(LiveRebuild::Structure)?;
        let ctx = Context {
            current,
            next,
            ramps,
            edit: StripEditRamps::of(edit_ramps, &item.strip),
        };
        let mut records = LiveStripRecords {
            strip_id: after.id.as_str(),
            fader: [None; 4],
            matrix: None,
            input: Vec::new(),
        };
        let families = item
            .unlowered
            .into_iter()
            .chain(item.input.then_some(Family::StripInput))
            .chain(item.level.then_some(Family::StripLevel))
            .chain(item.matrix.then_some(Family::StripMatrix));
        for family in families {
            let target = Target {
                before,
                after,
                instance: None,
            };
            match lower(family, &ctx, &target)? {
                Lowered::Fader(fader) => records.fader = fader,
                Lowered::Matrix(matrix) => records.matrix = matrix,
                Lowered::Input(input) => records.input = input,
                Lowered::Effect(_) | Lowered::Nothing => {}
            }
        }
        if records.fader[0].is_some() || records.matrix.is_some() || !records.input.is_empty() {
            delta.strips.push(records);
        }
        for section in &item.instances {
            let target = Target {
                before,
                after,
                instance: Some(section),
            };
            if let Lowered::Effect(Some(effect)) = lower(Family::Effect, &ctx, &target)? {
                delta.effects.push(effect);
            }
        }
    }
    Ok(delta)
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
    /// A live row of a family that this slice cannot lower changed (#1557).
    unlowered: Option<Family>,
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
            unlowered: None,
            instances: Vec::new(),
        }
    }
}

/// The strip `id` of `model` (a track or a submix).
fn strip_of<'m>(model: &'m SessionModel, id: &StableId) -> Option<StripRef<'m>> {
    model.strips().find(|strip| strip.id == id)
}

/// What a lowering reads besides its target.
struct Context<'m> {
    current: &'m SessionModel,
    next: &'m SessionModel,
    ramps: LiveRamps,
    edit: StripEditRamps,
}

/// What a lowering is of: a strip in both models, or one effect instance of it.
struct Target<'s, 'm, 'a> {
    before: StripRef<'m>,
    after: StripRef<'a>,
    /// The effect section, for the [`Family::Effect`] lowering.
    instance: Option<&'s Section>,
}

/// What one lowering produced.
enum Lowered<'a> {
    Fader([Option<TrackFaderRecord>; 4]),
    Matrix(Option<TrackControlRecord>),
    Input(Vec<TrackInputRecord>),
    Effect(Option<LiveEffectRecords<'a>>),
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
/// `Route` is unreachable in this slice (its rows are rebuild-only until #1557 adds its arm) and
/// refuses defensively.
fn lower<'a>(
    family: Family,
    ctx: &Context<'_>,
    target: &Target<'_, '_, 'a>,
) -> Result<Lowered<'a>, LiveRebuild> {
    #[cfg(feature = "test-support")]
    LOWERED_OWNERS.with(|count| count.set(count.get() + 1));
    let (before, after) = (&target.before, &target.after);
    match family {
        Family::StripLevel => {
            let fader = fader_records(before.fader, after.fader, ctx.ramps, ctx.edit)
                .map_err(|_| LiveRebuild::Domain)?;
            if followed_mute_changes(before.fader, after.fader, ctx.next, after.id.as_str()) {
                return Err(LiveRebuild::FollowedMute);
            }
            Ok(Lowered::Fader(fader))
        }
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
            ctx.next.sample_rate_hz,
            ctx.ramps,
            ctx.edit,
        )
        .map(Lowered::Input),
        Family::Setting => Ok(Lowered::Nothing),
        Family::Route => Err(LiveRebuild::Structure),
    }
}

/// The input records that bring a strip's input section from `before`'s values to `after`'s
/// (#1556 D2-D4), in write order: trims, then polarities, then filter targets, each left before
/// right. A record is emitted only when the value the render plane holds changes.
///
/// - `TrimDb`: one per lane whose [`checked_trim_gain`] bits change, or one `Both` when both lanes
///   change to the same `trim_db` bits. The comparison is of the gain's bits, so `0.0` to `-0.0`
///   (the same gain) gives no record.
/// - `PolarityInvert`: one per lane whose value changes, or one `Both` when both change to the
///   same value.
/// - `PreparedFilter`: the targets [`InputFilterPreparer`] designs, off the render thread, from the
///   pre-commit normalized pairs and the changed values (see [`filter_edits`]).
///
/// # Errors
///
/// [`LiveRebuild::Domain`] when a trim is outside the domain, or a pair or a designed target is
/// refused.
fn input_records(
    before: &session::DualMonoBuiltins,
    after: &session::DualMonoBuiltins,
    sample_rate_hz: u32,
    ramps: LiveRamps,
    edit: StripEditRamps,
) -> Result<Vec<TrackInputRecord>, LiveRebuild> {
    let lanes_before = [&before.left, &before.right];
    let lanes_after = [&after.left, &after.right];
    let mut records = Vec::with_capacity(4 + INPUT_FILTER_TARGET_CAPACITY);

    let gain_bits = |db: f32| {
        checked_trim_gain(db)
            .map(f32::to_bits)
            .map_err(|_| LiveRebuild::Domain)
    };
    let mut trim_changed = [false; 2];
    for lane in 0..2 {
        trim_changed[lane] =
            gain_bits(lanes_before[lane].trim_db)? != gain_bits(lanes_after[lane].trim_db)?;
    }
    for (lanes, lane) in lane_records(
        trim_changed,
        lanes_after[0].trim_db.to_bits() == lanes_after[1].trim_db.to_bits(),
    ) {
        records.push(TrackInputRecord::TrimDb {
            lanes,
            db: lanes_after[lane].trim_db,
            smoothing_samples: ramps.resolve(LiveRampRow::InputTrim, edit.input),
        });
    }

    let polarity_changed =
        [0, 1].map(|lane| lanes_before[lane].polarity_invert != lanes_after[lane].polarity_invert);
    for (lanes, lane) in lane_records(
        polarity_changed,
        lanes_after[0].polarity_invert == lanes_after[1].polarity_invert,
    ) {
        records.push(TrackInputRecord::PolarityInvert {
            lanes,
            inverted: lanes_after[lane].polarity_invert,
            smoothing_samples: ramps.resolve(LiveRampRow::PolarityInvert, edit.input),
        });
    }

    // The filters: compare the values each lane's pair normalizes to, so `-0.0` to `0.0` is no
    // change (#1556 D4).
    let pair = |lane: &session::ChannelBuiltins| {
        validate_input_filter_pair(sample_rate_hz, lane.hpf_hz, lane.lpf_hz)
            .map_err(|_| LiveRebuild::Domain)
    };
    let pairs_before = [pair(lanes_before[0])?, pair(lanes_before[1])?];
    let pairs_after = [pair(lanes_after[0])?, pair(lanes_after[1])?];
    let mut changed = [(false, false); 2];
    for lane in 0..2 {
        changed[lane] = (
            pairs_before[lane].hpf_hz.to_bits() != pairs_after[lane].hpf_hz.to_bits(),
            pairs_before[lane].lpf_hz.to_bits() != pairs_after[lane].lpf_hz.to_bits(),
        );
    }
    let edits = filter_edits(changed, pairs_after);
    if !edits.is_empty() {
        let seeds = [
            pairs_before[0].hpf_hz,
            pairs_before[0].lpf_hz,
            pairs_before[1].hpf_hz,
            pairs_before[1].lpf_hz,
        ];
        let mut out = [PreparedInputFilterTarget {
            lanes: BuiltinLaneSelector::Both,
            section: 0,
            pair: [0.0; 2],
            coefficients: [0.0; 6],
        }; INPUT_FILTER_TARGET_CAPACITY];
        let (_, count) = InputFilterPreparer
            .prepare(sample_rate_hz, &seeds, &edits, &mut out)
            .map_err(|_| LiveRebuild::Domain)?;
        records.extend(
            out[..count]
                .iter()
                .map(|&target| TrackInputRecord::PreparedFilter { target }),
        );
    }
    Ok(records)
}

/// The semantic edits that carry a strip's changed filter values (#1556 D4): per lane, `3` or `4`
/// for a one-sided change and the atomic pair `0` when both of the lane's values change (so a
/// pair that two one-sided edits would refuse, an HPF raised past the old LPF while the LPF
/// rises, is one valid edit). The same edit on both lanes is one `Both` edit, which is how the
/// preparer later merges equal targets.
fn filter_edits(
    changed: [(bool, bool); 2],
    after: [builtins::InputFilterPair; 2],
) -> Vec<InputFilterEdit> {
    let lane_edit = |lane: usize| {
        let pair = after[lane];
        match changed[lane] {
            (false, false) => None,
            (true, false) => Some((3, pair.hpf_hz, 0.0)),
            (false, true) => Some((4, pair.lpf_hz, 0.0)),
            (true, true) => Some((0, pair.hpf_hz, pair.lpf_hz)),
        }
    };
    let (left, right) = (lane_edit(0), lane_edit(1));
    let edit = |(parameter_id, value0, value1): (u32, f32, f32), channel| InputFilterEdit {
        parameter_id,
        channel,
        value0,
        value1,
    };
    match (left, right) {
        (Some(left), Some(right))
            if left.0 == right.0
                && left.1.to_bits() == right.1.to_bits()
                && left.2.to_bits() == right.2.to_bits() =>
        {
            vec![edit(left, ParameterChannel::Both)]
        }
        _ => left
            .map(|left| edit(left, ParameterChannel::Left))
            .into_iter()
            .chain(right.map(|right| edit(right, ParameterChannel::Right)))
            .collect(),
    }
}

/// Step 5: a changed effect's `params` must resolve, as preparation resolves them. An unknown
/// parameter ID, a unit mismatch or a duplicate lane has no control row to move, yet a rebuild
/// refuses it, so a live commit must not ack it.
fn refuse_unresolvable_params(
    current: &SessionModel,
    next: &SessionModel,
) -> Result<(), LiveRebuild> {
    for (before, after) in current.strips().zip(next.strips()) {
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
/// (#1264 D2, #1266 D1). The structure comparison has already proved that the two strips'
/// instances agree in everything but their `params` and `bypass`, and the rows have proved that
/// each changed value is live.
fn effect_records<'a>(
    ctx: &Context<'_>,
    target: &Target<'_, '_, 'a>,
    section: &Section,
) -> Result<Option<LiveEffectRecords<'a>>, LiveRebuild> {
    let racks_before = ctx.current.lower_strip(&target.before);
    let racks_after = ctx.next.lower_strip(&target.after);
    let (address, rack, index) = locate(&racks_after, section).ok_or(LiveRebuild::Structure)?;
    let (address_before, rack_before, index_before) =
        locate(&racks_before, section).ok_or(LiveRebuild::Structure)?;
    if (address, rack, index) != (address_before, rack_before, index_before) {
        return Err(LiveRebuild::Structure);
    }
    let effect_before = &racks_before.in_chain_order()[rack][index];
    let effect_after = &racks_after.in_chain_order()[rack][index];
    // This lowering carries a bypass and parameters. Any other changed effect row (today the link
    // mode, whose row is rebuild-only) has no record here, so a live classification of it would be
    // acked and then dropped: it is a rebuild until its own lowering exists.
    if effect_before.link_mode != effect_after.link_mode {
        return Err(LiveRebuild::Structure);
    }
    let bypass = (effect_before.bypass != effect_after.bypass).then_some(effect_after.bypass);
    let (mut records, targets) = if same_params(&effect_before.params, &effect_after.params) {
        (Vec::new(), None)
    } else {
        parameter_records(effect_before, effect_after, ctx.next.sample_rate_hz)?
    };
    if let Some(bypass) = bypass {
        records.insert(0, EffectControlRecord::Bypass(bypass));
    }
    Ok((!records.is_empty()).then(|| LiveEffectRecords {
        strip_id: target.after.id.as_str(),
        address,
        records,
        targets,
    }))
}

/// The records of one instance, and the targets of a target-capable one.
type InstanceRecords = (Vec<EffectControlRecord>, Option<Vec<PreparedEffectTarget>>);

/// The parameter records that bring one instance from `before`'s resolved values to `after`'s,
/// and, for a target-capable instance (the EQ), the targets that carry them (#1265 D1).
fn parameter_records(
    before: &Effect,
    after: &Effect,
    sample_rate_hz: u32,
) -> Result<InstanceRecords, LiveRebuild> {
    // A third-party or unknown identity never prepared; the rebuild reports it.
    let EffectIdentity::Native { effect_id } = &after.identity else {
        return Err(LiveRebuild::Structure);
    };
    let registry = launch_native_effect_registry().map_err(|_| LiveRebuild::Structure)?;
    let factory = registry
        .get_shared_ascii(effect_id.as_str())
        .ok_or(LiveRebuild::Structure)?;
    let descriptor = factory.descriptor();
    let values_before =
        resolve_initial_values(descriptor, &before.params).map_err(|_| LiveRebuild::Domain)?;
    let values_after =
        resolve_initial_values(descriptor, &after.params).map_err(|_| LiveRebuild::Domain)?;
    let mut records = Vec::new();
    for (value_before, value_after) in values_before.iter().zip(&values_after) {
        if value_before.value.to_bits() == value_after.value.to_bits() {
            continue;
        }
        let parameter = &descriptor.parameters[value_after.parameter_index as usize];
        // A row is live only when the plan keeps a block-rate cell for it; the control rows say
        // so, and this refuses a descriptor whose row would claim more.
        if parameter.automation_rate != AutomationRate::Block {
            return Err(LiveRebuild::RebuildOnly(RebuildReason::Correctness));
        }
        records.push(EffectControlRecord::Parameter {
            parameter_index: value_after.parameter_index,
            channel: value_after.channel,
            value: value_after.value,
        });
    }
    // The EQ's parameters ride its owner's prepared targets: its producer refuses a bare
    // parameter record (`EffectControlProducer::preflight`), so the records become the owner's
    // edits and the targets are designed here, on the control thread (#1265 D1).
    if records.is_empty() || factory.target_preparation().is_none() {
        return Ok((records, None));
    }
    let seeds: Vec<f32> = values_before.iter().map(|value| value.value).collect();
    let targets = design_targets(factory, sample_rate_hz, &seeds, &records)?;
    Ok((records, Some(targets)))
}

/// Designs a target-capable instance's targets with [`EqTargetPreparer`], from its pre-commit
/// `seeds` (canonical row order) and its changed `records` (#1265 D1).
fn design_targets(
    factory: std::sync::Arc<dyn NativeEffectFactory>,
    sample_rate_hz: u32,
    seeds: &[f32],
    records: &[EffectControlRecord],
) -> Result<Vec<PreparedEffectTarget>, LiveRebuild> {
    let descriptor = factory.descriptor();
    // A capability other than the 60-row EQ's has no control-thread designer: rebuild.
    let preparer = EqTargetPreparer::new(factory)
        .map_err(|_| LiveRebuild::RebuildOnly(RebuildReason::Correctness))?;
    if seeds.len() != EQ_VALUE_COUNT {
        return Err(LiveRebuild::RebuildOnly(RebuildReason::Correctness));
    }
    let edits: Vec<EqTargetEdit> = records
        .iter()
        .map(|record| match *record {
            EffectControlRecord::Parameter {
                parameter_index,
                channel,
                value,
            } => Ok(EqTargetEdit {
                parameter_id: descriptor.parameters[parameter_index as usize].id.0,
                channel,
                value,
            }),
            _ => Err(LiveRebuild::RebuildOnly(RebuildReason::Correctness)),
        })
        .collect::<Result<_, _>>()?;
    let mut out = [PreparedEffectTarget {
        slot: 0,
        channel: ParameterChannel::Both,
        words: [0; PREPARED_EFFECT_TARGET_WORDS],
    }; EQ_TARGET_CAPACITY];
    let (_, count) = preparer
        .prepare(sample_rate_hz, seeds, &edits, &mut out)
        .map_err(|error| match error {
            EqTargetPreparerError::Unsupported => {
                LiveRebuild::RebuildOnly(RebuildReason::Correctness)
            }
            _ => LiveRebuild::Domain,
        })?;
    Ok(out[..count].to_vec())
}

/// Whether two `params` lists are the same entries, value bits included.
fn same_params(before: &[EffectParam], after: &[EffectParam]) -> bool {
    before.len() == after.len()
        && before.iter().zip(after).all(|(left, right)| {
            left.parameter_id == right.parameter_id
                && left.channel == right.channel
                && left.unit == right.unit
                && left.value.to_bits() == right.value.to_bits()
        })
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
            next,
            ramps: LiveRamps::for_session(next),
            edit: StripEditRamps::default(),
        };
        let target = Target {
            before: strip_of(current, &next.tracks[0].id).expect("strip"),
            after: strip_of(next, &next.tracks[0].id).expect("strip"),
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
