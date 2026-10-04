//! Classify a committed session delta as a live track fader, mute, pan and effect-parameter
//! update, or a rebuild (issues #1255 and #1264; umbrella #1053 D1, D3, D9 and D14).
//!
//! A host that holds a running plan and commits a transaction has two models: the committed one
//! the plan was prepared with (plus every record pushed into it since, #1053 D9), and the
//! transaction's prospective one. [`classify_live_delta`] reads the two and decides, without ever
//! looking at edit opcodes, whether the difference can ride the plan's live fader, matrix and
//! effect lanes as records, or needs a plan rebuild. For a live delta it returns exactly the
//! records that bring the render plane from the first model's values to the second's, and never a
//! redundant one.
//!
//! This is the one classifier. Later slices widen it in place -- input records (#1261, #1262),
//! EQ and bypass records (#1265, #1266), submix strips and routes (#1225), followed mutes (#1226)
//! and VCAs (#1247) -- they do not add a second one.

use builtins::{BuiltinLaneSelector, checked_fader_gain};
use builtins_compiler::{TrackControlRecord, TrackFaderRecord, lower_matrix_or_pan};
use effect_compiler::{LiveEffectAddress, launch_native_effect_registry, resolve_initial_values};
use effect_contract::{AutomationRate, EffectControlRecord, NativeEffectRegistry};
use session::{
    DualMonoFader, Effect, EffectIdentity, EffectParam, RouteSource, SessionModel, Track,
    canonical_session_json,
};

/// The ramp lengths, in samples, that live fader and mute records carry (#1053 D3).
///
/// Pan and matrix records do not take theirs from here: they carry the post-commit model's own
/// `smoothing_samples`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiveRamps {
    /// The ramp of a `FaderDb` record.
    pub fader_samples: u32,
    /// The ramp of a `Mute` record.
    pub mute_samples: u32,
}

impl LiveRamps {
    /// The ramps a session's live fader and mute records carry.
    ///
    /// #1053 D3: zero (a step) until #1054 derives it from the session's `controlSmoothing`. A
    /// step is bit-exact to the value a rebuild of the committed model bakes.
    #[must_use]
    pub fn for_session(model: &SessionModel) -> Self {
        let _ = model;
        Self {
            fader_samples: 0,
            mute_samples: 0,
        }
    }
}

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
}

impl LiveStripRecords<'_> {
    /// The fader/mute records, in push order.
    pub fn fader_records(&self) -> impl Iterator<Item = TrackFaderRecord> + '_ {
        self.fader.iter().map_while(|record| *record)
    }
}

/// The records of one effect instance whose live parameter values change (#1264 D2).
#[derive(Clone, Debug, PartialEq)]
pub struct LiveEffectRecords<'a> {
    /// The ID of the strip (a track) that owns the instance, borrowed from the post-commit model.
    pub strip_id: &'a str,
    /// The instance's live address in its strip: a console slot by its slot index, an insert by
    /// its index. A producer is found by `(strip_id, address)`.
    pub address: LiveEffectAddress,
    /// One [`EffectControlRecord::Parameter`] per `(parameter_index, channel)` whose resolved
    /// value changes, in descriptor order, `Left` before `Right`; never empty.
    pub records: Vec<EffectControlRecord>,
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
    /// Either model declares a VCA (#1053 G3, until #1247).
    Vca,
    /// The track set differs, or a field other than a track's fader, pan/matrix or effect
    /// `params` differs.
    Structure,
    /// A fader or pan/matrix value is outside the domain its render-side setter accepts, or an
    /// effect's pre- or post-commit `params` do not resolve. The rebuild then reports the same
    /// preparation diagnostic it always has.
    Domain,
    /// An effect parameter value changes that the plan keeps prepared: one that is not
    /// automatable or whose `automation_rate` is not `Block`, or any parameter of an effect whose
    /// parameters ride prepared targets (the parametric EQ, #1053 G4, until #1265).
    Prepared,
    /// A track's mute changes while a route with `follows_mute` sends from it (#1053 G2, until
    /// #1226).
    FollowedMute,
}

/// Classifies the delta from `current` to `next` as live records or a rebuild (#1053 D1).
///
/// Both inputs must be **normalized** models (`CompiledSession::normalized_model()`): the
/// committed session's and the transaction's prospective one. The track pairing and the canonical
/// JSON comparison rely on normalized order.
///
/// The steps, in order:
/// 1. [`LiveRebuild::Vca`] if either model declares a VCA.
/// 2. [`LiveRebuild::Structure`] if the track IDs differ, by count or pairwise in order.
/// 3. [`LiveRebuild::Structure`] if `next`, with `current`'s revision, session ID, render- and
///    output-profile IDs, stored automation, and every track's fader, pan/matrix, console-entry
///    `params` and insert `params` copied from `current`, has canonical JSON bytes other than
///    `current`'s. An effect's identity, quality, link mode, bypass and sidechain, the insert
///    order and the console slot set stay compared (#1266 lifts bypass). Bytes,
///    never `PartialEq`: the canonical `f32` spelling keeps a zero's sign, so a `trim_db` edit
///    from `0.0` to `-0.0` is structural. A canonical JSON error is structural too.
/// 4. Per track, in order: [`LiveRebuild::Domain`] if a fader dB is refused by
///    [`checked_fader_gain`] or a pan/matrix does not lower through [`lower_matrix_or_pan`] (the
///    single authorities the render-side setters use); [`LiveRebuild::FollowedMute`] if a lane's
///    mute changes and a `follows_mute` route in `next` sends from the track; otherwise the
///    track's records. Then, for each of the track's effect instances in chain order whose
///    `params` differ: [`LiveRebuild::Domain`] if either model's `params` do not resolve through
///    [`resolve_initial_values`] (the one function preparation uses); [`LiveRebuild::Prepared`]
///    if a resolved value whose bits change belongs to a parameter that is not automatable, whose
///    `automation_rate` is not `Block`, or of an effect with a prepared-target capability (the
///    EQ, whose producer refuses a bare parameter record); otherwise one
///    [`EffectControlRecord::Parameter`] per changed `(parameter_index, channel)`, carrying the
///    resolved post-commit value. A removed `params` entry resolves to the default, so its change
///    is a record too.
///
/// The session ID, the two profile IDs and the stored automation are model-only: no prepared plan
/// reads them (#1260), so a delta that changes only them is live with no records. Masking
/// `automation` is correct only while no host renders stored automation: the first issue that
/// renders it (#1058) must remove it from the mask, or an automation edit would commit without
/// reaching the running plan.
///
/// A record is emitted only when the value the render plane holds changes (`solo.rs`: never a
/// redundant record): a `FaderDb` per lane whose gain bits change (one `Both` when both change to
/// the same dB bits), a `Mute` per lane whose mute changes (one `Both` when both change to the
/// same value), and a matrix record when a lowered coefficient's bits change. A fader move on a
/// lane that stays muted still gets its record, so the stage remembers the gain for a later
/// unmute. An effect record is emitted only for a resolved value whose bits change: a `Both`
/// value rewritten as equal per-lane values gives none.
///
/// No smoothing window is checked: host-core preparation passes `maximum_smoothing_samples:
/// u32::MAX` and the render setters accept any `u32` window. If preparation ever gets a finite
/// smoothing cap, this classifier must refuse a smoothing above it as [`LiveRebuild::Domain`];
/// otherwise a live commit could leave a committed model that its own rebuild refuses.
///
/// # Allocation
///
/// Control thread only. The masked clone, the two canonical JSON strings and, when an effect's
/// `params` differ, the lowered racks, the launch registry and the resolved values are allocated
/// and freed on every call, on the control-plane precedent of #369 (the protocol already compiles a
/// whole session per edit). Nothing it allocates is retained apart from the returned entries.
///
/// # Errors
///
/// The [`LiveRebuild`] reason of the first step that refuses.
pub fn classify_live_delta<'a>(
    current: &SessionModel,
    next: &'a SessionModel,
    ramps: LiveRamps,
) -> Result<LiveDelta<'a>, LiveRebuild> {
    if !current.vcas.is_empty() || !next.vcas.is_empty() {
        return Err(LiveRebuild::Vca);
    }
    if current.tracks.len() != next.tracks.len()
        || current
            .tracks
            .iter()
            .zip(&next.tracks)
            .any(|(before, after)| before.id != after.id)
    {
        return Err(LiveRebuild::Structure);
    }
    let mut masked = next.clone();
    masked.revision = current.revision;
    // #1260 D1: fields no prepared plan reads. Each profile's other fields stay compared.
    masked.session_id = current.session_id.clone();
    masked.render_profile.id = current.render_profile.id.clone();
    masked.output_profile.id = current.output_profile.id.clone();
    // #1260 D2: correct only while no host renders stored automation (#1058). The first issue
    // that renders it must drop this line.
    masked.automation = current.automation.clone();
    for (track, before) in masked.tracks.iter_mut().zip(&current.tracks) {
        track.fader = before.fader.clone();
        track.matrix_or_pan = before.matrix_or_pan.clone();
        // #1264 D2: only the values. Paired by position; a changed slot set or insert order
        // still differs below, by its IDs.
        for (entry, prior) in track.console.iter_mut().zip(&before.console) {
            entry.params.clone_from(&prior.params);
        }
        for (effect, prior) in track
            .inserts
            .effects
            .iter_mut()
            .zip(&before.inserts.effects)
        {
            effect.params.clone_from(&prior.params);
        }
    }
    match (
        canonical_session_json(current),
        canonical_session_json(&masked),
    ) {
        (Ok(before), Ok(after)) if before.as_bytes() == after.as_bytes() => {}
        _ => return Err(LiveRebuild::Structure),
    }
    drop(masked);

    let mut delta = LiveDelta::default();
    let mut registry = None;
    for (before, after) in current.tracks.iter().zip(&next.tracks) {
        let gains_before = fader_gains(&before.fader)?;
        let gains_after = fader_gains(&after.fader)?;
        let (matrix_before, _) =
            lower_matrix_or_pan(&before.matrix_or_pan).map_err(|_| LiveRebuild::Domain)?;
        let (matrix_after, smoothing_after) =
            lower_matrix_or_pan(&after.matrix_or_pan).map_err(|_| LiveRebuild::Domain)?;

        let mutes_before = [before.fader.left_mute, before.fader.right_mute];
        let mutes_after = [after.fader.left_mute, after.fader.right_mute];
        if mutes_before != mutes_after && follows_mute_from(next, after.id.as_str()) {
            return Err(LiveRebuild::FollowedMute);
        }

        let mut fader = [None; 4];
        let mut used = 0;
        let mut push = |record| {
            fader[used] = Some(record);
            used += 1;
        };
        let db_after = [after.fader.left_db, after.fader.right_db];
        let gain_changed = [0, 1].map(|lane| gains_before[lane] != gains_after[lane]);
        for (lanes, lane) in
            lane_records(gain_changed, db_after[0].to_bits() == db_after[1].to_bits())
        {
            push(TrackFaderRecord::FaderDb {
                lanes,
                db: db_after[lane],
                smoothing_samples: ramps.fader_samples,
            });
        }
        let mute_changed = [0, 1].map(|lane| mutes_before[lane] != mutes_after[lane]);
        for (lanes, lane) in lane_records(mute_changed, mutes_after[0] == mutes_after[1]) {
            push(TrackFaderRecord::Mute {
                lanes,
                muted: mutes_after[lane],
                smoothing_samples: ramps.mute_samples,
            });
        }

        let matrix = (matrix_bits(matrix_before) != matrix_bits(matrix_after)).then_some(
            TrackControlRecord {
                matrix: matrix_after,
                smoothing_samples: smoothing_after,
            },
        );
        if used > 0 || matrix.is_some() {
            delta.strips.push(LiveStripRecords {
                strip_id: after.id.as_str(),
                fader,
                matrix,
            });
        }
        effect_records(
            (current, before),
            (next, after),
            &mut registry,
            &mut delta.effects,
        )?;
    }
    Ok(delta)
}

/// Appends the parameter records of one track's effect instances whose `params` differ (#1264
/// D2). The step-3 mask has already proved that the two tracks' instances agree in everything but
/// their `params`.
fn effect_records<'a>(
    (current, before): (&SessionModel, &Track),
    (next, after): (&'a SessionModel, &'a Track),
    registry: &mut Option<NativeEffectRegistry>,
    output: &mut Vec<LiveEffectRecords<'a>>,
) -> Result<(), LiveRebuild> {
    let racks_before = current.lower_track(before);
    let racks_after = next.lower_track(after);
    let [pre_before, inserts_before, post_before] = racks_before.in_chain_order();
    let [pre_after, inserts_after, post_after] = racks_after.in_chain_order();
    let pre_insert = pre_after.len();
    let instances = pre_before
        .iter()
        .zip(pre_after)
        .enumerate()
        .map(|(index, pair)| (LiveEffectAddress::console(index as u32), pair))
        .chain(
            inserts_before
                .iter()
                .zip(inserts_after)
                .enumerate()
                .map(|(index, pair)| (LiveEffectAddress::insert(index as u32), pair)),
        )
        .chain(
            post_before
                .iter()
                .zip(post_after)
                .enumerate()
                .map(|(index, pair)| {
                    (
                        LiveEffectAddress::console((pre_insert + index) as u32),
                        pair,
                    )
                }),
        );
    for (address, (effect_before, effect_after)) in instances {
        if same_params(&effect_before.params, &effect_after.params) {
            continue;
        }
        let records = parameter_records(effect_before, effect_after, registry)?;
        if !records.is_empty() {
            output.push(LiveEffectRecords {
                strip_id: after.id.as_str(),
                address,
                records,
            });
        }
    }
    Ok(())
}

/// The parameter records that bring one instance from `before`'s resolved values to `after`'s.
fn parameter_records(
    before: &Effect,
    after: &Effect,
    registry: &mut Option<NativeEffectRegistry>,
) -> Result<Vec<EffectControlRecord>, LiveRebuild> {
    // A third-party or unknown identity never prepared; the rebuild reports it.
    let EffectIdentity::Native { effect_id } = &after.identity else {
        return Err(LiveRebuild::Structure);
    };
    let registry = match registry {
        Some(registry) => registry,
        None => {
            registry.insert(launch_native_effect_registry().map_err(|_| LiveRebuild::Structure)?)
        }
    };
    let factory = registry
        .get_shared_ascii(effect_id.as_str())
        .ok_or(LiveRebuild::Structure)?;
    let descriptor = factory.descriptor();
    let values_before =
        resolve_initial_values(descriptor, &before.params).map_err(|_| LiveRebuild::Domain)?;
    let values_after =
        resolve_initial_values(descriptor, &after.params).map_err(|_| LiveRebuild::Domain)?;
    // The EQ's parameters ride its owner's prepared targets: its producer refuses a bare
    // parameter record (`EffectControlProducer::preflight`), so every change is prepared until
    // #1265 (#1053 G4).
    let target_capable = factory.target_preparation().is_some();
    let mut records = Vec::new();
    for (value_before, value_after) in values_before.iter().zip(&values_after) {
        if value_before.value.to_bits() == value_after.value.to_bits() {
            continue;
        }
        let parameter = &descriptor.parameters[value_after.parameter_index as usize];
        if target_capable
            || !parameter.automatable
            || parameter.automation_rate != AutomationRate::Block
        {
            return Err(LiveRebuild::Prepared);
        }
        records.push(EffectControlRecord::Parameter {
            parameter_index: value_after.parameter_index,
            channel: value_after.channel,
            value: value_after.value,
        });
    }
    Ok(records)
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

/// The gain bits each lane's fader holds, through the one fader-domain authority.
fn fader_gains(fader: &DualMonoFader) -> Result<[u32; 2], LiveRebuild> {
    let gain = |db| {
        checked_fader_gain(db)
            .map(f32::to_bits)
            .map_err(|_| LiveRebuild::Domain)
    };
    Ok([gain(fader.left_db)?, gain(fader.right_db)?])
}

/// The lane records for one per-lane change mask: one `Both` when both lanes change and
/// `same_value`, else one record per changed lane, left first. Each entry names the lane whose
/// post-commit value the record carries.
fn lane_records(
    changed: [bool; 2],
    same_value: bool,
) -> impl Iterator<Item = (BuiltinLaneSelector, usize)> {
    let records: [Option<(BuiltinLaneSelector, usize)>; 2] = match changed {
        [true, true] if same_value => [Some((BuiltinLaneSelector::Both, 0)), None],
        [left, right] => [
            left.then_some((BuiltinLaneSelector::Left, 0)),
            right.then_some((BuiltinLaneSelector::Right, 1)),
        ],
    };
    records.into_iter().flatten()
}

/// Whether `next` has a `follows_mute` route whose source is the track `track_id` (#1053 G2).
fn follows_mute_from(next: &SessionModel, track_id: &str) -> bool {
    next.routes.iter().any(|route| {
        route.follows_mute
            && matches!(&route.source, RouteSource::Track { track_id: source, .. }
                if source.as_str() == track_id)
    })
}

fn matrix_bits(matrix: builtins::Matrix2x2) -> [u32; 4] {
    [matrix.ll, matrix.lr, matrix.rl, matrix.rr].map(f32::to_bits)
}
