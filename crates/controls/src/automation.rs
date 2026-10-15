//! The row rule of a stored automation (decision 17, C10; #1566 D6): one function that holds every
//! stored automation of a session to the control it names.

use automation::{Segment, Shape, value_at};
use effect_contract::ParameterUnit;
use session::{
    Automation, AutomationSegment, AutomationShape, ControlAddress, Owner, Section, SessionModel,
    StableId, StripRef,
};

use crate::ControlKind;
use crate::builtin_rows::{HPF_HZ, LPF_HZ};
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

/// One refusal of a stored automation: a stable code and the session path it belongs to.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlDiagnostic {
    /// The stable machine code (a [`ControlError`] code).
    pub code: &'static str,
    /// `$.automation[id=<id>].target`, or `$.automation[id=<id>].segments[<i>]`.
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
///   is automated, else at the HPF entry's.
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

/// One side of a lane's filter pair: its automation entry (with its segments in the evaluator's
/// type) or its static value.
enum Curve<'a> {
    Static(f32),
    Automated {
        entry: &'a Automation,
        segments: Vec<Segment>,
    },
}

/// What one side does on a sub-interval `[a, b)` of the cut time axis.
#[derive(Clone, Copy)]
enum Piece {
    /// A hold, a `step` or a static value.
    Constant(f32),
    /// A `linear` or `exponential` piece, with its values at `a` and (the left limit) at `b`.
    Ramp { shape: Shape, at_a: f32, at_b: f32 },
}

impl Piece {
    fn ends(self) -> (f64, f64) {
        match self {
            Self::Constant(value) => (f64::from(value), f64::from(value)),
            Self::Ramp { at_a, at_b, .. } => (f64::from(at_a), f64::from(at_b)),
        }
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

    /// The piece on `[a, b)`, where no cut of this curve lies strictly inside; `b` is `None` on the
    /// last sub-interval, a hold.
    fn piece(&self, a: u64, b: Option<u64>) -> Piece {
        let segments = match self {
            Self::Static(value) => return Piece::Constant(*value),
            Self::Automated { segments, .. } => segments,
        };
        let at_a = value_at(segments, node_time(a));
        let segment = segments.get(Self::defining(segments, a));
        match (segment, b) {
            (Some(segment), Some(b))
                if segment.start <= a && a < segment.end && segment.shape != Shape::Step =>
            {
                Piece::Ramp {
                    shape: segment.shape,
                    at_a,
                    // The segment alone: at its end the value is its own `end_value`, never the
                    // start of a segment that begins there.
                    at_b: value_at(core::slice::from_ref(segment), node_time(b)),
                }
            }
            _ => Piece::Constant(at_a),
        }
    }
}

/// Whether the HPF piece stays below the LPF piece on the sub-interval (#1645 D2).
fn ordered(hpf: Piece, lpf: Piece) -> bool {
    match (hpf, lpf) {
        // "Off" on either side leaves the sub-interval unchecked.
        (Piece::Constant(off), _) | (_, Piece::Constant(off)) if off == 0.0 => true,
        // Render does not round a constant: the static rule.
        (Piece::Constant(h), Piece::Constant(l)) => h < l,
        (Piece::Ramp { shape: hs, .. }, Piece::Ramp { shape: ls, .. }) if hs == ls => {
            let (h_a, h_b) = hpf.ends();
            let (l_a, l_b) = lpf.ends();
            l_a > h_a * ORDER_MARGIN && l_b > h_b * ORDER_MARGIN
        }
        _ => {
            let (h_a, h_b) = hpf.ends();
            let (l_a, l_b) = lpf.ends();
            l_a.min(l_b) > h_a.max(h_b) * ORDER_MARGIN
        }
    }
}

/// The input filter order rule on one lane of one strip, at all times (#1645 D2).
///
/// Each side's curve is its automation entry under the hold rule, or the constant of its static
/// value when it is not automated; a lane with neither side automated is the static rule's. The
/// time axis is cut at every `start_sample` and `end_sample` of either curve; on each
/// sub-interval each side is one monotone piece, so its extremes lie at the ends:
///
/// - both constant: `H < L`, in `f32`;
/// - both `linear` or both `exponential`: `L > H·(1 + 2^-22)` at both ends (a linear difference
///   or a monotone ratio keeps the order inside);
/// - otherwise: `min(L) > max(H)·(1 + 2^-22)`, sufficient and not necessary, by design.
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
            target.owner == Owner::Strip(strip.id.clone())
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
    for (position, &a) in cuts.iter().enumerate() {
        let b = cuts.get(position + 1).copied();
        if ordered(hpf.piece(a, b), lpf.piece(a, b)) {
            continue;
        }
        // The LPF entry when the LPF is automated, else the HPF entry.
        let reported = if matches!(lpf, Curve::Automated { .. }) {
            &lpf
        } else {
            &hpf
        };
        let Curve::Automated { entry, segments } = reported else {
            unreachable!("one side is automated")
        };
        return Some(ControlDiagnostic {
            code: ControlError::AutomationOrder.code(),
            path: format!(
                "$.automation[id={}].segments[{}]",
                entry.id.as_str(),
                Curve::defining(segments, a)
            ),
        });
    }
    None
}
