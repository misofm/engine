//! Classify a committed session delta as a live track fader, mute and pan update, or a rebuild
//! (issue #1255; umbrella #1053 D1, D3 and D9).
//!
//! A host that holds a running plan and commits a transaction has two models: the committed one
//! the plan was prepared with (plus every record pushed into it since, #1053 D9), and the
//! transaction's prospective one. [`classify_live_delta`] reads the two and decides, without ever
//! looking at edit opcodes, whether the difference can ride the plan's live fader and matrix lanes
//! as records, or needs a plan rebuild. For a live delta it returns exactly the records that bring
//! the render plane from the first model's values to the second's, and never a redundant one.
//!
//! This is the one classifier. Later slices widen it in place -- input records (#1261, #1262),
//! effect records (#1264-#1266), submix strips and routes (#1225), followed mutes (#1226) and VCAs
//! (#1247) -- they do not add a second one.

use builtins::{BuiltinLaneSelector, checked_fader_gain};
use builtins_compiler::{TrackControlRecord, TrackFaderRecord, lower_matrix_or_pan};
use session::{DualMonoFader, RouteSource, SessionModel, canonical_session_json};

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

/// A live delta: the records that bring the running plan to the post-commit model.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LiveDelta<'a> {
    /// One entry per strip that has at least one record, in normalized (canonical track ID)
    /// order. Empty for a transaction that rewrites identical values: that is still a live delta.
    pub strips: Vec<LiveStripRecords<'a>>,
}

/// Why a delta needs a plan rebuild.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveRebuild {
    /// Either model declares a VCA (#1053 G3, until #1247).
    Vca,
    /// The track set differs, or a field other than a track's fader or pan/matrix differs.
    Structure,
    /// A fader or pan/matrix value is outside the domain its render-side setter accepts. The
    /// rebuild then reports the same preparation diagnostic it always has.
    Domain,
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
/// 3. [`LiveRebuild::Structure`] if `next`, with `current`'s revision and every track's fader and
///    pan/matrix copied from `current`, has canonical JSON bytes other than `current`'s. Bytes,
///    never `PartialEq`: the canonical `f32` spelling keeps a zero's sign, so a `trim_db` edit
///    from `0.0` to `-0.0` is structural. A canonical JSON error is structural too.
/// 4. Per track, in order: [`LiveRebuild::Domain`] if a fader dB is refused by
///    [`checked_fader_gain`] or a pan/matrix does not lower through [`lower_matrix_or_pan`] (the
///    single authorities the render-side setters use); [`LiveRebuild::FollowedMute`] if a lane's
///    mute changes and a `follows_mute` route in `next` sends from the track; otherwise the
///    track's records.
///
/// A record is emitted only when the value the render plane holds changes (`solo.rs`: never a
/// redundant record): a `FaderDb` per lane whose gain bits change (one `Both` when both change to
/// the same dB bits), a `Mute` per lane whose mute changes (one `Both` when both change to the
/// same value), and a matrix record when a lowered coefficient's bits change. A fader move on a
/// lane that stays muted still gets its record, so the stage remembers the gain for a later
/// unmute.
///
/// No smoothing window is checked: host-core preparation passes `maximum_smoothing_samples:
/// u32::MAX` and the render setters accept any `u32` window. If preparation ever gets a finite
/// smoothing cap, this classifier must refuse a smoothing above it as [`LiveRebuild::Domain`];
/// otherwise a live commit could leave a committed model that its own rebuild refuses.
///
/// # Allocation
///
/// Control thread only. The masked clone and the two canonical JSON strings are allocated and
/// freed on every call, on the control-plane precedent of #369 (the protocol already compiles a
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
    for (track, before) in masked.tracks.iter_mut().zip(&current.tracks) {
        track.fader = before.fader.clone();
        track.matrix_or_pan = before.matrix_or_pan.clone();
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
    }
    Ok(delta)
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
