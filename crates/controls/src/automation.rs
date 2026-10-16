//! The row rule of a stored automation (decision 17, C10; #1566 D6): one function that holds every
//! stored automation of a session to the control it names.

use automation::{Segment, Shape, value_at};
use effect_contract::ParameterUnit;
use session::{
    Automation, AutomationSegment, AutomationShape, ControlAddress, Owner, Section, SessionModel,
    StableId, StripRef,
};

use crate::ControlKind;
use crate::builtin_rows::{FADER_DB, HPF_HZ, LPF_HZ};
use crate::descriptor::{ControlDescriptor, Lanes, check};
use crate::effect_rows::{has_rows, session_unit};
use crate::error::ControlError;
use crate::lane::Side;
use crate::registry::resolve;
use session::Lane;

/// Whether `target` names an effect instance (a console slot or an insert) whose identity has no
/// control rows: a `cid` effect, or a native effect the launch registry lacks.
fn effect_has_no_rows(model: &SessionModel, target: &ControlAddress) -> bool {
    let Owner::Strip(strip_id) = &target.owner else {
        return false;
    };
    let identity = match &target.section {
        Section::Console(slot) => model
            .console
            .slots()
            .find(|candidate| candidate.slot == *slot)
            .map(|candidate| &candidate.identity),
        Section::Inserts(effect) => model
            .strips()
            .find(|strip| strip.id == strip_id)
            .and_then(|strip| {
                strip
                    .inserts
                    .effects
                    .iter()
                    .find(|candidate| candidate.id == *effect)
            })
            .map(|candidate| &candidate.identity),
        Section::Own | Section::Builtins => None,
    };
    identity.is_some_and(|identity| !has_rows(identity))
}

/// Whether the row at `target` takes a stored hand-move offset (#1650 D2, root's ruling R17-6):
/// only a strip's builtin `fader_db`. The slices that render the input trim (#1658) and the send
/// gain (#1654) add their rows here.
pub(crate) fn takes_offset(target: &ControlAddress) -> bool {
    matches!(target.owner, Owner::Strip(_))
        && target.section == Section::Builtins
        && target.control == FADER_DB
}

/// The offset rule of one entry whose target is accepted (#1650 D2), per lane, left then right:
/// on a lane that takes no offset (the entry does not drive it, or its row takes none) the
/// offset is exactly `+0.0`, else `control.automation.offset`; on a driven lane of a row that
/// takes one, the offset lies in the row's own domain (`fader_db`: `[-144, 24]` dB, the VCA
/// offset domain), else `control.domain`. `-0.0` is a valid zero only on a lane that takes an
/// offset, so a lane without one has one spelling.
fn offset_faults(
    automation: &Automation,
    descriptor: &ControlDescriptor,
) -> impl Iterator<Item = (ControlError, &'static str)> {
    let target = &automation.target;
    let takes = takes_offset(target);
    [
        (Lane::Left, automation.offset.left_db, ".offset.left_db"),
        (Lane::Right, automation.offset.right_db, ".offset.right_db"),
    ]
    .into_iter()
    .filter_map(move |(lane, value, suffix)| {
        let driven = target.lane == lane || target.lane == Lane::Both;
        let fault = if !(takes && driven) {
            (value.to_bits() != 0.0_f32.to_bits()).then_some(ControlError::AutomationOffset)
        } else {
            check(descriptor, lane, value, None)
                .err()
                .map(|_| ControlError::Domain)
        };
        fault.map(|error| (error, suffix))
    })
}

/// One refusal of a stored automation: a stable code and the session path it belongs to.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlDiagnostic {
    /// The stable machine code (a [`ControlError`] code).
    pub code: &'static str,
    /// `$.automation[id=<id>].target`, `$.automation[id=<id>].segments[<i>]`, or
    /// `$.automation[id=<id>].offset.left_db` / `.right_db`.
    pub path: String,
}

/// Every refusal of the stored automation of `model`, judged by the row each target names.
///
/// The codes are the control validator's own ([`check`]) and one more, so a stored automation, a
/// `SET_CONTROL` write and an enqueued automation are refused alike:
///
/// - `control.address.unknown`: no row at the address (an unknown builtin ID, effect parameter or
///   own row);
/// - `control.automation.target`: the row is not [`automatable`](crate::ControlDescriptor::automatable);
/// - `control.inactive`: the row is automatable but the owner's shape does not use it (a matrix row
///   on a pan strip);
/// - `control.lane`: `left` or `right` on a shared row;
/// - `control.domain`: a segment whose unit is not the row's unit, or a value (`start_value` or
///   `end_value`) the row refuses;
/// - `control.automation.shape`: a segment whose shape the row does not take (#1645): a toggle or
///   enumeration row takes only `step`; `exponential` only on a continuous row whose minimum is
///   strictly positive and whose unit is not dB; a `linear` or `exponential` segment never has a
///   value equal to the row's disabled value (a filter's "off" is reached only by a `step`);
/// - `control.automation.order`: on a strip lane where the HPF or the LPF is automated, the two
///   curves can come to an HPF at or above an enabled LPF (#1645 D2; see
///   `input_filter_order`). It is reported once per lane, at the LPF entry's segment when the LPF
///   is automated, else at the HPF entry's;
/// - `control.automation.offset`: a hand-move offset (#1650 D2) that is not `+0.0` on a lane the
///   entry does not drive, or on any lane of a row that takes no offset (only `fader_db` takes
///   one); `control.domain` for an offset outside the fader row's domain. Each is reported at
///   `.offset.left_db` or `.offset.right_db`, after the entry's segments.
///
/// A target is reported once, at `.target`; its segments are judged only when the target is
/// accepted. A segment is reported once, at `.segments[<i>]`: a domain fault first, else a shape
/// fault. The order rule runs only on a lane whose automated filter entries have no other fault. Whether the owner, the console slot
/// and the insert exist is the session validator's rule, not this one's; an address it names but the
/// model lacks is `control.address.unknown` here too. A target in an effect that has no rows (a
/// `cid` effect, a native effect the registry lacks) is not reported here: effect preparation
/// refuses that effect with its own code.
///
/// Control plane only: it allocates its result.
#[must_use]
pub fn automation_diagnostics(model: &SessionModel, sample_rate: u32) -> Vec<ControlDiagnostic> {
    let mut diagnostics = Vec::new();
    // Whether each entry has a fault of its own, so the order rule reads only sound curves.
    let mut faulted = vec![false; model.automation.len()];
    for (index, automation) in model.automation.iter().enumerate() {
        let target = &automation.target;
        let at = |suffix: &str| format!("$.automation[id={}]{suffix}", automation.id.as_str());
        let refuse = |error: ControlError, path: String| ControlDiagnostic {
            code: error.code(),
            path,
        };
        let before = diagnostics.len();
        let descriptor = match resolve(model, target, sample_rate) {
            Ok(resolved) => resolved.descriptor,
            // An effect that carries no rows is refused by effect preparation with its own code
            // (`effect.native.unavailable`, `effect.third_party.unavailable_at_launch`); one fault is
            // reported once.
            Err(_) if effect_has_no_rows(model, target) => {
                faulted[index] = true;
                continue;
            }
            Err(error) => {
                diagnostics.push(refuse(error, at(".target")));
                faulted[index] = true;
                continue;
            }
        };
        let target_fault = if !descriptor.automatable() {
            Some(ControlError::AutomationTarget)
        // The target's own faults, in `check`'s order: an inactive row, then a lane the row lacks.
        } else if !descriptor.active {
            Some(ControlError::Inactive)
        } else if descriptor.lanes == Lanes::Shared && target.lane != Lane::Both {
            Some(ControlError::Lane)
        } else {
            None
        };
        if let Some(error) = target_fault {
            diagnostics.push(refuse(error, at(".target")));
            faulted[index] = true;
            continue;
        }
        for (position, segment) in automation.segments.iter().enumerate() {
            let in_domain = session_unit(descriptor.unit) == segment.unit
                && [segment.start_value, segment.end_value]
                    .into_iter()
                    .all(|value| check(&descriptor, target.lane, value, None).is_ok());
            let fault = if !in_domain {
                Some(ControlError::Domain)
            } else if !shape_fits(&descriptor, segment) {
                Some(ControlError::AutomationShape)
            } else {
                None
            };
            if let Some(error) = fault {
                diagnostics.push(refuse(error, at(&format!(".segments[{position}]"))));
            }
        }
        for (error, suffix) in offset_faults(automation, &descriptor) {
            diagnostics.push(refuse(error, at(suffix)));
        }
        faulted[index] = diagnostics.len() > before;
    }
    for strip in model.strips() {
        for side in [Side::Left, Side::Right] {
            if let Some(diagnostic) = input_filter_order(model, &strip, side, &faulted) {
                // A `both` entry on two lanes whose static values match fails both lanes at one
                // segment: one fault, reported once.
                if !diagnostics.contains(&diagnostic) {
                    diagnostics.push(diagnostic);
                }
            }
        }
    }
    diagnostics
}

/// Whether `address` is a strip's static HPF or LPF and `model` stores automation on either filter
/// of that strip: a write to it can break the input filter order rule with the automation table
/// unchanged (#1645 fold), so a classifier judges it with [`automation_diagnostics`].
#[must_use]
pub fn filter_under_automation(model: &SessionModel, address: &ControlAddress) -> bool {
    address.section == Section::Builtins
        && (address.control == HPF_HZ || address.control == LPF_HZ)
        && matches!(&address.owner, Owner::Strip(strip) if has_filter_automation(model, strip))
}

/// Whether `model` stores automation on the HPF or the LPF of strip `strip` (either lane).
fn has_filter_automation(model: &SessionModel, strip: &StableId) -> bool {
    model.automation.iter().any(|automation| {
        let target = &automation.target;
        matches!(&target.owner, Owner::Strip(owner) if owner == strip)
            && target.section == Section::Builtins
            && (target.control == HPF_HZ || target.control == LPF_HZ)
    })
}

/// Whether a static HPF or LPF value differs between `current` and `next` on a strip that `next`
/// automates either filter of (#1645 fold). The order rule reads a non-automated side's static
/// value, so such an edit can break it with the automation table unchanged: a classifier re-runs
/// [`automation_diagnostics`] on `next` when this holds. Strips are compared in order; a model
/// whose strips differ is a structural change, which the caller refuses first.
#[must_use]
pub fn filter_statics_changed_under_automation(
    current: &SessionModel,
    next: &SessionModel,
) -> bool {
    current.strips().zip(next.strips()).any(|(before, after)| {
        let moved = [
            (before.builtins.left.hpf_hz, after.builtins.left.hpf_hz),
            (before.builtins.left.lpf_hz, after.builtins.left.lpf_hz),
            (before.builtins.right.hpf_hz, after.builtins.right.hpf_hz),
            (before.builtins.right.lpf_hz, after.builtins.right.lpf_hz),
        ]
        .into_iter()
        .any(|(old, new)| old.to_bits() != new.to_bits());
        moved && has_filter_automation(next, after.id)
    })
}

/// Whether the row takes the segment's shape (#1645 D1, the shape rule; the values are already in
/// the row's domain).
fn shape_fits(descriptor: &ControlDescriptor, segment: &AutomationSegment) -> bool {
    let touches_disabled = descriptor
        .disabled_value
        .is_some_and(|disabled| segment.start_value == disabled || segment.end_value == disabled);
    match segment.shape {
        AutomationShape::Step => true,
        AutomationShape::Linear => descriptor.kind == ControlKind::Continuous && !touches_disabled,
        AutomationShape::Exponential => {
            descriptor.kind == ControlKind::Continuous
                && !touches_disabled
                && descriptor.minimum > 0.0
                && descriptor.unit != ParameterUnit::Db
        }
    }
}

/// The relative margin of the order rule: `2^-22`, four times the `f32` rounding bound, so that
/// after render rounds each ramped value once to `f32` no rendered pair ties or crosses.
const ORDER_MARGIN: f64 = 1.0 + 1.0 / 4_194_304.0;

/// How far apart in time two stored filter values can be when render compares them, in node-time
/// samples: the order rule holds the HPF at each sample below the LPF at every sample this close on
/// either side (#1645 review fold, root's ruling (a)). #1637 gives an automated builtin filter cell
/// two reaches of 64 samples each, and neither is an exported constant: a grid ramp started at `τ`
/// aims at the curve's value 64 samples ahead (`automation::events` `target_at`), so render can
/// lead the stored curve by 64, and a discontinuity ramps over 64 samples, so render can lag it by
/// 64. The rendered value at `τ` therefore lies among the stored values of `[τ - 64, τ + 64]` on
/// each filter, and a rendered pair can read stored values up to 128 samples apart. A change to
/// either reach changes this constant with it.
const ORDER_REACH: u64 = 128;

/// One side of a lane's filter pair: its automation entry (with its segments in the evaluator's
/// type) or its static value.
enum Curve<'a> {
    Static(f32),
    Automated {
        entry: &'a Automation,
        segments: Vec<Segment>,
    },
}

/// What one side does on a closed sub-interval `[a, b]` of the cut time axis (at `b` the left
/// limit).
#[derive(Clone, Copy)]
enum Piece {
    /// A hold, a `step` or a static value.
    Constant(f32),
    /// A `linear` or `exponential` piece: the one segment that defines the sub-interval.
    Ramp(Segment),
}

impl Piece {
    /// The value at `t`, a sample of the piece's closed sub-interval: the segment alone, so at its
    /// end the value is its own `end_value`, never the start of a segment that begins there.
    fn at(self, t: u64) -> f64 {
        match self {
            Self::Constant(value) => f64::from(value),
            Self::Ramp(segment) => {
                f64::from(value_at(core::slice::from_ref(&segment), node_time(t)))
            }
        }
    }

    /// Whether the piece is affine in time (a constant or a `linear` ramp).
    fn linear(self) -> bool {
        !matches!(self, Self::Ramp(segment) if segment.shape != Shape::Linear)
    }

    /// Whether the piece's logarithm is affine in time (a constant or an `exponential` ramp).
    fn exponential(self) -> bool {
        !matches!(self, Self::Ramp(segment) if segment.shape != Shape::Exponential)
    }
}

fn stored_segment(segment: &AutomationSegment) -> Segment {
    Segment {
        start: segment.start_sample,
        end: segment.end_sample,
        start_value: segment.start_value,
        end_value: segment.end_value,
        shape: match segment.shape {
            AutomationShape::Step => Shape::Step,
            AutomationShape::Linear => Shape::Linear,
            AutomationShape::Exponential => Shape::Exponential,
        },
    }
}

fn node_time(sample: u64) -> i64 {
    i64::try_from(sample).unwrap_or(i64::MAX)
}

impl<'a> Curve<'a> {
    /// The side's automation entry, or its static value when it is not automated.
    fn new(entry: Option<&'a Automation>, value: f32) -> Self {
        match entry {
            Some(entry) => Self::Automated {
                entry,
                segments: entry.segments.iter().map(stored_segment).collect(),
            },
            None => Self::Static(value),
        }
    }

    /// The index of the segment that defines the curve's value at `a`: the first segment for the
    /// leading hold, the held segment in a gap or after the last one.
    fn defining(segments: &[Segment], a: u64) -> usize {
        segments
            .partition_point(|segment| segment.start <= a)
            .saturating_sub(1)
    }

    /// The piece on `[a, b]`, where no cut of this curve lies strictly inside; `b` is `None` on the
    /// last sub-interval, a hold.
    fn piece(&self, a: u64, b: Option<u64>) -> Piece {
        let segments = match self {
            Self::Static(value) => return Piece::Constant(*value),
            Self::Automated { segments, .. } => segments,
        };
        match (segments.get(Self::defining(segments, a)), b) {
            (Some(segment), Some(_))
                if segment.start <= a && a < segment.end && segment.shape != Shape::Step =>
            {
                Piece::Ramp(*segment)
            }
            _ => Piece::Constant(value_at(segments, node_time(a))),
        }
    }
}

/// Whether the HPF piece on `[a, b]` stays below the LPF piece on `[c, d]` at every pair of
/// samples `(t, s)`, `t` in the first and `s` in the second, at most [`ORDER_REACH`] apart (#1645
/// D2, widened by the review fold's ruling (a)). The pairs form a convex polygon in `(t, s)`.
fn ordered(hpf: Piece, (a, b): (u64, u64), lpf: Piece, (c, d): (u64, u64)) -> bool {
    // No sample of one is within reach of the other.
    if c > b.saturating_add(ORDER_REACH) || a > d.saturating_add(ORDER_REACH) {
        return true;
    }
    match (hpf, lpf) {
        // "Off" on either side leaves the pair unchecked.
        (Piece::Constant(off), _) | (_, Piece::Constant(off)) if off == 0.0 => true,
        // Render does not round a constant: the static rule.
        (Piece::Constant(h), Piece::Constant(l)) => h < l,
        // `L(s) - H(t)·m` is affine in `(t, s)` (both pieces linear), or `ln L(s) - ln H(t)` is
        // (both exponential), so its least value over the polygon is at a vertex. Every vertex
        // is where two of its edges meet (`t` at `a` or `b`, `s` at `c` or `d`, `s - t` at `±R`);
        // the candidates that leave the polygon are skipped, and any extra one inside it is only
        // one more sample the rule must hold at.
        _ if (hpf.linear() && lpf.linear()) || (hpf.exponential() && lpf.exponential()) => {
            let r = ORDER_REACH;
            [
                (a, c),
                (a, d),
                (b, c),
                (b, d),
                (a, a.saturating_sub(r)),
                (a, a.saturating_add(r)),
                (b, b.saturating_sub(r)),
                (b, b.saturating_add(r)),
                (c.saturating_sub(r), c),
                (c.saturating_add(r), c),
                (d.saturating_sub(r), d),
                (d.saturating_add(r), d),
            ]
            .into_iter()
            .filter(|&(t, s)| (a..=b).contains(&t) && (c..=d).contains(&s) && t.abs_diff(s) <= r)
            .all(|(t, s)| lpf.at(s) > hpf.at(t) * ORDER_MARGIN)
        }
        // Mixed pieces: the least LPF value over the samples within reach of the HPF piece above
        // the greatest HPF value over the samples within reach of the LPF piece. Sufficient and
        // not necessary, by design; each piece is monotone, so its extremes are at its ends.
        _ => {
            let (t_from, t_to) = (
                a.max(c.saturating_sub(ORDER_REACH)),
                b.min(d.saturating_add(ORDER_REACH)),
            );
            let (s_from, s_to) = (
                c.max(a.saturating_sub(ORDER_REACH)),
                d.min(b.saturating_add(ORDER_REACH)),
            );
            lpf.at(s_from).min(lpf.at(s_to)) > hpf.at(t_from).max(hpf.at(t_to)) * ORDER_MARGIN
        }
    }
}

/// The input filter order rule on one lane of one strip, at all times (#1645 D2).
///
/// Each side's curve is its automation entry under the hold rule, or the constant of its static
/// value when it is not automated; a lane with neither side automated is the static rule's. The
/// time axis is cut at every `start_sample` and `end_sample` of either curve; on each closed
/// sub-interval each side is one monotone piece.
///
/// Render moves a stored value by up to [`ORDER_REACH`] samples (a grid ramp aims ahead, a jump
/// ramps), so the rule holds the HPF at every sample `t` below the LPF at every sample `s` with
/// `|t - s| <= ORDER_REACH` (#1645 review fold, root's ruling (a)): stored automation is never
/// accepted now and crossed in render. Each HPF piece is judged against every LPF piece whose
/// sub-interval comes within reach of its own, over the pairs of samples within reach:
///
/// - both constant: `H < L`, in `f32`;
/// - both affine in time (a constant or `linear`), or both with an affine logarithm (a constant
///   or `exponential`): `L > H·(1 + 2^-22)` at every vertex of the polygon of pairs, which bounds
///   it everywhere inside;
/// - otherwise: `min(L) > max(H)·(1 + 2^-22)` over the samples of each piece within reach of the
///   other, sufficient and not necessary, by design.
///
/// A constant "off" (`0`) on either side leaves its pairs unchecked.
///
/// It reads only `model`, so a classifier can run it on a candidate model. Entries with their own
/// fault (`faulted`) are not read: their curves may touch "off" inside a ramp.
fn input_filter_order(
    model: &SessionModel,
    strip: &StripRef<'_>,
    side: Side,
    faulted: &[bool],
) -> Option<ControlDiagnostic> {
    let lane = side.pick(&Lane::Left, &Lane::Right);
    let entry = |control: u32| {
        model.automation.iter().enumerate().find(|(_, automation)| {
            let target = &automation.target;
            matches!(&target.owner, Owner::Strip(owner) if owner == strip.id)
                && target.section == Section::Builtins
                && target.control == control
                && (target.lane == *lane || target.lane == Lane::Both)
        })
    };
    let (hpf_entry, lpf_entry) = (entry(HPF_HZ), entry(LPF_HZ));
    if hpf_entry.is_none() && lpf_entry.is_none() {
        return None;
    }
    if [hpf_entry, lpf_entry]
        .into_iter()
        .flatten()
        .any(|(index, _)| faulted.get(index).copied().unwrap_or(true))
    {
        return None;
    }
    let channel = side.pick(&strip.builtins.left, &strip.builtins.right);
    let hpf = Curve::new(hpf_entry.map(|(_, entry)| entry), channel.hpf_hz);
    let lpf = Curve::new(lpf_entry.map(|(_, entry)| entry), channel.lpf_hz);
    let mut cuts = vec![0_u64];
    for found in [&hpf, &lpf] {
        if let Curve::Automated { segments, .. } = found {
            cuts.extend(
                segments
                    .iter()
                    .flat_map(|segment| [segment.start, segment.end]),
            );
        }
    }
    cuts.sort_unstable();
    cuts.dedup();
    // Sub-interval `k` is `[cuts[k], cuts[k + 1]]`; the last one runs on, a hold on both sides.
    let span = |k: usize| {
        let end = cuts.get(k + 1).copied();
        (cuts[k], end, end.unwrap_or(u64::MAX))
    };
    let pieces = |curve: &Curve<'_>| -> Vec<Piece> {
        (0..cuts.len())
            .map(|k| {
                let (a, b, _) = span(k);
                curve.piece(a, b)
            })
            .collect()
    };
    let (hpf_pieces, lpf_pieces) = (pieces(&hpf), pieces(&lpf));
    for (i, &h) in hpf_pieces.iter().enumerate() {
        let (a, _, b) = span(i);
        // The LPF sub-intervals within reach: from the first that ends at or after
        // `a - ORDER_REACH` to the last that starts at or before `b + ORDER_REACH`.
        let first = cuts
            .partition_point(|&cut| cut.saturating_add(ORDER_REACH) < a)
            .saturating_sub(1);
        let last = cuts.partition_point(|&cut| cut <= b.saturating_add(ORDER_REACH));
        for (j, &l) in lpf_pieces.iter().enumerate().take(last).skip(first) {
            let (c, _, d) = span(j);
            if ordered(h, (a, b), l, (c, d)) {
                continue;
            }
            // The LPF entry when the LPF is automated (at the LPF's sub-interval), else the HPF
            // entry (at the HPF's).
            let (reported, at) = if matches!(lpf, Curve::Automated { .. }) {
                (&lpf, c)
            } else {
                (&hpf, a)
            };
            let Curve::Automated { entry, segments } = reported else {
                unreachable!("one side is automated")
            };
            return Some(ControlDiagnostic {
                code: ControlError::AutomationOrder.code(),
                path: format!(
                    "$.automation[id={}].segments[{}]",
                    entry.id.as_str(),
                    Curve::defining(segments, at)
                ),
            });
        }
    }
    None
}
