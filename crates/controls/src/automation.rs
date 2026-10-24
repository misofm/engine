//! The row rule of a stored automation (decision 17, C10; #1566 D6): one function that holds every
//! stored automation of a session to the control it names.

use automation::{CellDescription, Completion, JumpLength, SeekMode, Segment, Shape, value_at};
use effect_contract::ParameterUnit;
use session::{
    Automation, AutomationSegment, AutomationShape, ControlAddress, Owner, Section, SessionModel,
    StableId, StripRef, Vca,
};

use crate::ControlKind;
use crate::builtin_rows::{FADER_DB, HPF_HZ, LPF_HZ, MUTE};
use crate::descriptor::{ControlDescriptor, Lanes, check};
use crate::effect_rows::{has_rows, session_unit};
use crate::error::ControlError;
use crate::lane::Side;
use crate::registry::resolve;
use crate::vca::{VCA_MUTE, VCA_OFFSET_DB};
use session::Lane;
use std::collections::BTreeMap;

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

/// Whether the row at `target` takes a stored hand-move offset (#1650 D2, root's ruling R17-6): a
/// strip's builtin `fader_db` and a VCA's `offset_db` (row 1, [`vca_offset_row`], per lane, #1655
/// D5 and root's ruling of 2026-10-14: a level control). The slices that render the input trim
/// (#1658) and the send gain (#1654) add their rows here.
pub(crate) fn takes_offset(target: &ControlAddress) -> bool {
    fader_row(target) || vca_offset_row(target)
}

/// The offset rule of one entry whose target is accepted (#1650 D2), per lane, left then right:
/// on a lane that takes no offset (the entry does not drive it, or its row takes none) the
/// offset is exactly `+0.0`, else `control.automation.offset`; on a driven lane of a row that
/// takes one, the offset lies in the row's own domain (`fader_db` and a VCA's `offset_db`:
/// `[-144, 24]` dB, the VCA offset domain), else `control.domain`. `-0.0` is a valid zero only on
/// a lane that takes an offset, so a lane without one has one spelling.
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
///   entry does not drive, or on any lane of a row that takes no offset (only `fader_db` and a
///   VCA's `offset_db` take one, #1655 D5); `control.domain` for an offset outside the row's
///   domain. Each is reported at `.offset.left_db` or `.offset.right_db`, after the entry's
///   segments.
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

/// How the cells of one row's stored automation move in render (#1637): the parameters of every
/// cell compiled from an entry on the row. Each rendering row has one, written here and nowhere
/// else, so a stage never spells a row's grid or ramp rule (#1646 departure 6).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CellRule {
    /// `L`, the grid ramp in samples; the grid period is the larger of 64 and `L`.
    pub grid_ramp: u32,
    /// Where a jump's ramp length comes from.
    pub jump: JumpLength,
    /// Where a ramp started at `τ` completes.
    pub completion: Completion,
    /// A target whose ramp cannot restart while one runs.
    pub no_restart: bool,
    /// How a seek lands on the cell.
    pub seek: SeekMode,
}

impl CellRule {
    /// The cell description of `segments` under this rule, the input of
    /// [`automation::ProgramBuilder`].
    #[must_use]
    pub const fn describe(self, segments: &[Segment]) -> CellDescription<'_> {
        CellDescription {
            segments,
            grid_ramp: self.grid_ramp,
            grid: if self.grid_ramp > 64 {
                self.grid_ramp
            } else {
                64
            },
            jump: self.jump,
            completion: self.completion,
            no_restart: self.no_restart,
        }
    }
}

/// The fader row's cell rule (#1646 D1): grid period and grid ramp 64, completion `End` (the
/// fader ramp assigns its target on its last frame), the jump length of the block
/// (`controls::LiveRamps::for_row(LiveRampRow::Fader)`, resolved by the caller), restartable, and
/// an exact seek landing (a builtin cell).
pub const FADER_CELL_RULE: CellRule = CellRule {
    grid_ramp: 64,
    jump: JumpLength::Block,
    completion: Completion::End,
    no_restart: false,
    seek: SeekMode::Exact,
};

/// The mute row's cell rule (#1651 D1, batch 18 amendment): the fader row's rule. A mute curve is
/// `step` only with values 0 and 1 (#1645 D1), so it is constant between its discontinuities and
/// its cell gives no grid event, only jumps and seek landings: completion `End` (a mute ramp
/// assigns its target on its last frame, as the fader ramp does), the jump length of the block
/// (`controls::LiveRamps::for_row(LiveRampRow::Mute)`, resolved by the caller), restartable, and an
/// exact seek landing (a builtin cell).
pub const MUTE_CELL_RULE: CellRule = CellRule {
    grid_ramp: 64,
    jump: JumpLength::Block,
    completion: Completion::End,
    no_restart: false,
    seek: SeekMode::Exact,
};

/// Whether a stored automation of `target` renders (#1646, #1647, #1651, #1655): the fader row of
/// a strip (builtin row 5, [`fader_row`]), its mute row (builtin row 6, [`mute_row`]) and a VCA's
/// rows 1 and 2 (`offset_db`, [`vca_offset_row`], and `mute`, [`vca_mute_row`]). An entry on any
/// other row (a send's rows among them, #1654) is validated and kept in the session but compiles
/// to no cell. Every rendering slice adds its row here, so preparation and the live classifier
/// (#1648) read one list.
///
/// A reader that means "the fader" tests [`fader_row`] instead (#1651 T2): neither the mute row
/// nor a VCA row gives a lane a fader cell or a fader hand-move offset. A VCA ride
/// makes its members' lanes event lanes ([`fader_event_lanes`], [`mute_event_lanes`]); it does not
/// automate a member's own fader.
#[must_use]
pub fn automation_row_renders(target: &ControlAddress) -> bool {
    fader_row(target) || mute_row(target) || vca_row(target)
}

/// Whether `target` is a VCA's `offset_db` row (row 1, #1655): per lane, a level control that
/// takes a hand-move offset (D5).
#[must_use]
pub fn vca_offset_row(target: &ControlAddress) -> bool {
    matches!(target.owner, Owner::Vca(_))
        && target.section == Section::Own
        && target.control == VCA_OFFSET_DB
}

/// Whether `target` is a VCA's `mute` row (row 2, #1655): per lane, a toggle whose curve ORs into
/// each reached member lane's stage mute (D4); a hand VCA mute stays a live static value.
#[must_use]
pub fn vca_mute_row(target: &ControlAddress) -> bool {
    matches!(target.owner, Owner::Vca(_))
        && target.section == Section::Own
        && target.control == VCA_MUTE
}

/// Whether `target` is one of a VCA's rendered rows ([`vca_offset_row`] or [`vca_mute_row`]).
fn vca_row(target: &ControlAddress) -> bool {
    vca_offset_row(target) || vca_mute_row(target)
}

/// The stored automation entries that drive VCA `vca`'s `offset_db`, per lane (`[left, right]`),
/// by the rule of [`fader_automation`] on [`vca_offset_row`] (#1655): the first entry per lane in
/// table order. An empty table returns at once.
#[must_use]
pub fn vca_offset_automation<'a>(
    model: &'a SessionModel,
    vca: &StableId,
) -> [Option<&'a Automation>; 2] {
    owner_row_automation(model, vca, vca_offset_row)
}

/// The stored automation entries that drive VCA `vca`'s `mute`, per lane (`[left, right]`), by the
/// rule of [`vca_offset_automation`] on [`vca_mute_row`] (#1655).
#[must_use]
pub fn vca_mute_automation<'a>(
    model: &'a SessionModel,
    vca: &StableId,
) -> [Option<&'a Automation>; 2] {
    owner_row_automation(model, vca, vca_mute_row)
}

/// The lanes of strip `strip` that are fader event lanes (#1655 D2, batch 19 amendment), per lane
/// (`[left, right]`): the strip's own fader lane is automated ([`automated_fader_lanes`]), or an
/// automated VCA `offset_db` lane reaches it (VCA lane `l` reaches its members' lane `l`). `reach`
/// is the strip's VCA reach in `model` (its entry of [`SessionModel::vca_reach`], or
/// [`SessionModel::vca_reach_of`]): indices into `model.vcas`. The one rule of preparation, the
/// classifier, the strip join and the route attach. An empty table returns at once.
///
/// A reader that means "this strip's own entry" (the `fader_db` row's value and flag, the lane's
/// own hand offset, the mask) reads [`fader_automation`] instead.
#[must_use]
pub fn fader_event_lanes(model: &SessionModel, strip: &StableId, reach: &[usize]) -> [bool; 2] {
    if model.automation.is_empty() {
        return [false; 2];
    }
    event_lanes(
        &model.vcas,
        reach,
        automated_fader_lanes(model, strip),
        |vca| vca_offset_automation(model, vca),
    )
}

/// The lanes of strip `strip` that are mute event lanes (#1655 D4, batch 19 amendment), per lane
/// (`[left, right]`): the strip's own mute lane is automated ([`mute_automation`]), or an
/// automated VCA `mute` lane reaches it. `reach` is as for [`fader_event_lanes`]. A following send
/// of such a lane takes the seven-word route cell.
#[must_use]
pub fn mute_event_lanes(model: &SessionModel, strip: &StableId, reach: &[usize]) -> [bool; 2] {
    if model.automation.is_empty() {
        return [false; 2];
    }
    let own = mute_automation(model, strip).map(|entry| entry.is_some());
    event_lanes(&model.vcas, reach, own, |vca| {
        vca_mute_automation(model, vca)
    })
}

/// `own`, ORed per lane with whether some VCA of `reach` (indices into `vcas`) has an entry
/// `entries` gives on that lane.
fn event_lanes<'a>(
    vcas: &[Vca],
    reach: &[usize],
    own: [bool; 2],
    entries: impl Fn(&StableId) -> [Option<&'a Automation>; 2],
) -> [bool; 2] {
    let mut lanes = own;
    if lanes == [true; 2] {
        return lanes;
    }
    for vca in reach.iter().filter_map(|&index| vcas.get(index)) {
        let driven = entries(&vca.id);
        for lane in 0..2 {
            lanes[lane] |= driven[lane].is_some();
        }
    }
    lanes
}

/// Whether `target` is a strip's builtin `mute` row (row 6): the rendered row whose entry gives a
/// lane its mute cell ([`mute_automation`], #1651).
#[must_use]
pub fn mute_row(target: &ControlAddress) -> bool {
    matches!(target.owner, Owner::Strip(_))
        && target.section == Section::Builtins
        && target.control == MUTE
}

/// Whether `target` is a strip's builtin `fader_db` row (row 5): the one rendered row whose entry
/// gives a lane its fader cell ([`fader_automation`]) and its hand-move offset (#1650). The
/// readers of those test this predicate, not [`automation_row_renders`], so an entry on another
/// rendered row reaches neither of them (#1651 T2).
#[must_use]
pub fn fader_row(target: &ControlAddress) -> bool {
    matches!(target.owner, Owner::Strip(_))
        && target.section == Section::Builtins
        && target.control == FADER_DB
}

/// The stored automation entries that drive strip `strip`'s fader, per lane (`[left, right]`): a
/// `both` entry drives both lanes, a `left` or `right` entry its own. The session validator allows
/// at most one entry per lane (#1626), so each lane has at most one.
#[must_use]
pub fn fader_automation<'a>(
    model: &'a SessionModel,
    strip: &StableId,
) -> [Option<&'a Automation>; 2] {
    owner_row_automation(model, strip, fader_row)
}

/// The stored automation entries that drive strip `strip`'s mute, per lane (`[left, right]`), by
/// the rule of [`fader_automation`] on the mute row ([`mute_row`], #1651).
#[must_use]
pub fn mute_automation<'a>(
    model: &'a SessionModel,
    strip: &StableId,
) -> [Option<&'a Automation>; 2] {
    owner_row_automation(model, strip, mute_row)
}

/// The entries on the row `row` accepts whose owner (a strip or a VCA; `row` fixes the kind) has
/// the ID `owner`, per lane; the first entry per lane in table order.
fn owner_row_automation<'a>(
    model: &'a SessionModel,
    owner: &StableId,
    row: fn(&ControlAddress) -> bool,
) -> [Option<&'a Automation>; 2] {
    let mut lanes = [None, None];
    for automation in &model.automation {
        let target = &automation.target;
        let id = match &target.owner {
            Owner::Strip(id) | Owner::Vca(id) => id,
            Owner::Route(_) | Owner::Session => continue,
        };
        if !row(target) || id != owner {
            continue;
        }
        for (index, lane) in [Lane::Left, Lane::Right].into_iter().enumerate() {
            if drives(target.lane, lane) && lanes[index].is_none() {
                lanes[index] = Some(automation);
            }
        }
    }
    lanes
}

/// Whether an entry on lane `entry` drives lane `lane` (`Left` or `Right`): a `both` entry drives
/// both lanes, a `left` or `right` entry its own.
fn drives(entry: Lane, lane: Lane) -> bool {
    entry == lane || entry == Lane::Both
}

/// The rendered rows whose driving entries [`AutomationIndex`] holds, one per owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RenderedRow {
    /// A strip's `fader_db` ([`fader_row`]).
    Fader,
    /// A strip's `mute` ([`mute_row`]).
    Mute,
    /// A VCA's `offset_db` ([`vca_offset_row`]).
    VcaOffset,
    /// A VCA's `mute` ([`vca_mute_row`]).
    VcaMute,
}

/// The driving entries of every owner's rendered rows, from one pass over a model's automation
/// table (#1710): per owner and row, the first entry per lane in table order, the rule of
/// [`fader_automation`], [`mute_automation`], [`vca_offset_automation`] and
/// [`vca_mute_automation`], which each scan the whole table for one owner.
///
/// A caller that reads many owners of one model (a control list, a catalog build or a staged
/// commit) builds the index once and reads it through [`crate::ModelView::indexed`], so the walk
/// costs `O(entries)` once and `O(log entries)` per owner, not `O(entries)` per owner. It is a
/// value of one build, freed with it: `O(entries)` in size, and empty (no allocation) for a model
/// with no rendered entry.
#[derive(Clone)]
pub struct AutomationIndex<'a> {
    /// The model the index was built from: a view checks it reads the same model, and the event
    /// lanes read a VCA reach's IDs from its `vcas`.
    model: &'a SessionModel,
    lanes: BTreeMap<(RenderedRow, &'a StableId), [Option<&'a Automation>; 2]>,
    /// Whether some entry is on a VCA `mute` row ([`vca_mute_row`]).
    any_vca_mute: bool,
}

impl<'a> AutomationIndex<'a> {
    /// The index of `model`'s automation table.
    #[must_use]
    pub fn of(model: &'a SessionModel) -> Self {
        let mut lanes: BTreeMap<_, [Option<&'a Automation>; 2]> = BTreeMap::new();
        let mut any_vca_mute = false;
        for automation in &model.automation {
            let target = &automation.target;
            let key = match &target.owner {
                Owner::Strip(id) if fader_row(target) => (RenderedRow::Fader, id),
                Owner::Strip(id) if mute_row(target) => (RenderedRow::Mute, id),
                Owner::Vca(id) if vca_offset_row(target) => (RenderedRow::VcaOffset, id),
                Owner::Vca(id) if vca_mute_row(target) => {
                    any_vca_mute = true;
                    (RenderedRow::VcaMute, id)
                }
                _ => continue,
            };
            let driven = lanes.entry(key).or_default();
            for (index, lane) in [Lane::Left, Lane::Right].into_iter().enumerate() {
                if drives(target.lane, lane) && driven[index].is_none() {
                    driven[index] = Some(automation);
                }
            }
        }
        Self {
            model,
            lanes,
            any_vca_mute,
        }
    }

    /// Whether the index was built from `model` (the same model, not only an equal table).
    pub(crate) fn is_of(&self, model: &SessionModel) -> bool {
        core::ptr::eq(self.model, model)
    }

    fn get(&self, row: RenderedRow, owner: &StableId) -> [Option<&'a Automation>; 2] {
        let lanes: &BTreeMap<(RenderedRow, &StableId), [Option<&'a Automation>; 2]> = &self.lanes;
        lanes.get(&(row, owner)).copied().unwrap_or_default()
    }

    /// [`fader_automation`] of strip `strip`.
    #[must_use]
    pub fn fader(&self, strip: &StableId) -> [Option<&'a Automation>; 2] {
        self.get(RenderedRow::Fader, strip)
    }

    /// [`mute_automation`] of strip `strip`.
    #[must_use]
    pub fn mute(&self, strip: &StableId) -> [Option<&'a Automation>; 2] {
        self.get(RenderedRow::Mute, strip)
    }

    /// [`vca_offset_automation`] of VCA `vca`.
    #[must_use]
    pub fn vca_offset(&self, vca: &StableId) -> [Option<&'a Automation>; 2] {
        self.get(RenderedRow::VcaOffset, vca)
    }

    /// [`vca_mute_automation`] of VCA `vca`.
    #[must_use]
    pub fn vca_mute(&self, vca: &StableId) -> [Option<&'a Automation>; 2] {
        self.get(RenderedRow::VcaMute, vca)
    }

    /// [`automated_fader_lanes`] of strip `strip` (#1844): the lanes [`Self::fader`] drives.
    #[must_use]
    pub fn automated_fader_lanes(&self, strip: &StableId) -> [bool; 2] {
        self.fader(strip).map(|entry| entry.is_some())
    }

    /// [`fader_event_lanes`] of strip `strip`, `reach` its VCA reach in the indexed model.
    #[must_use]
    pub fn fader_event_lanes(&self, strip: &StableId, reach: &[usize]) -> [bool; 2] {
        event_lanes(
            &self.model.vcas,
            reach,
            self.automated_fader_lanes(strip),
            |vca| self.vca_offset(vca),
        )
    }

    /// [`mute_event_lanes`] of strip `strip`, `reach` its VCA reach in the indexed model.
    #[must_use]
    pub fn mute_event_lanes(&self, strip: &StableId, reach: &[usize]) -> [bool; 2] {
        let own = self.mute(strip).map(|entry| entry.is_some());
        event_lanes(&self.model.vcas, reach, own, |vca| self.vca_mute(vca))
    }

    /// Whether some entry of the indexed table is on a VCA `mute` row ([`vca_mute_row`]).
    #[must_use]
    pub const fn any_vca_mute(&self) -> bool {
        self.any_vca_mute
    }
}

impl core::fmt::Debug for AutomationIndex<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AutomationIndex")
            .field("driven_rows", &self.lanes.len())
            .finish_non_exhaustive()
    }
}

/// Whether `model` holds a stored automation that renders ([`automation_row_renders`]) and drives
/// the cell at `address` (#1648 D3): an entry on the same owner, section and row whose lane drives
/// the address's lane. A `left` or `right` address names one cell; a `both` address names two,
/// and is automated when either lane is. This is the enumeration preparation compiles cells from
/// ([`fader_automation`] for the fader row, [`mute_automation`] for the mute row): a cell exists
/// exactly where this holds.
///
/// A static value of an automated cell is the lane's fallback, not what the stage renders, so a
/// static edit of it gives no live record (#1648 D3).
#[must_use]
pub fn automated_cell(model: &SessionModel, address: &ControlAddress) -> bool {
    model.automation.iter().any(|automation| {
        let target = &automation.target;
        automation_row_renders(target)
            && target.owner == address.owner
            && target.section == address.section
            && target.control == address.control
            && match address.lane {
                Lane::Both => true,
                lane => drives(target.lane, lane),
            }
    })
}

/// Whether `model` automates strip `strip`'s fader, per lane (`[left, right]`): [`automated_cell`]
/// at the strip's builtin `fader_db` row (#1648 D3). The live classifier and a successor's strip
/// join read it to give no `FaderDb` record to an automated lane.
#[must_use]
pub fn automated_fader_lanes(model: &SessionModel, strip: &StableId) -> [bool; 2] {
    if model.automation.is_empty() {
        return [false; 2];
    }
    let mut address = ControlAddress {
        owner: Owner::Strip(strip.clone()),
        section: Section::Builtins,
        control: FADER_DB,
        lane: Lane::Left,
    };
    let left = automated_cell(model, &address);
    address.lane = Lane::Right;
    [left, automated_cell(model, &address)]
}

/// Whether the stored automation of the rows that render ([`automation_row_renders`]) differs
/// between `current` and `next` (#1648 D1): the rendered entries of each model, in table order,
/// compared by `id`, `target` and `segments`, every `f32` by bits (the canonical spelling keeps a
/// zero's sign, so this is the canonical-bytes comparison of those fields). An entry's hand offset
/// (#1650) is not compared: an offset-only change is a live write of its own (#1650 D5). An entry
/// whose target leaves or joins a rendered row is a difference.
///
/// A difference needs a plan rebuild: a plan's cells are compiled from these entries at
/// preparation, so no live record can carry a changed curve.
///
/// With equal entries, it is also a difference when the `(member strip, lane)` set that an
/// automated VCA lane reaches differs between the two models (#1655 D5, root's ruling of
/// 2026-10-14): a membership or nesting edit of a VCA that some rendered entry drives. Each newly
/// reached pair needs new event state, so only a rebuild can carry it (decision 14 rule 1). An
/// automated VCA added or removed is already a difference of the entries. A VCA lane `l` reaches
/// its members' lane `l`, and the entries (so the automated lanes) are equal here, so the rule
/// compares, per automated VCA (by ID), the strip IDs it reaches ([`SessionModel::vca_reach`]). A reach change that involves only VCAs no rendered entry drives
/// stays D15-6's live update.
#[must_use]
pub fn rendered_automation_changed(current: &SessionModel, next: &SessionModel) -> bool {
    fn rendered(model: &SessionModel) -> impl Iterator<Item = &Automation> {
        model
            .automation
            .iter()
            .filter(|automation| automation_row_renders(&automation.target))
    }
    let (mut before, mut after) = (rendered(current), rendered(next));
    loop {
        match (before.next(), after.next()) {
            (None, None) => break,
            (Some(before), Some(after)) if same_curve(before, after) => {}
            _ => return true,
        }
    }
    vca_reach_changed(current, next)
}

/// Whether some VCA that a rendered entry of `next` drives ([`vca_row`]) is reached by another set
/// of strips in `current` than in `next` (the reach rule of [`rendered_automation_changed`]). No
/// reach table is built when no rendered entry is on a VCA row.
fn vca_reach_changed(current: &SessionModel, next: &SessionModel) -> bool {
    let mut automated: Vec<&StableId> = next
        .automation
        .iter()
        .filter(|automation| vca_row(&automation.target))
        .filter_map(|automation| match &automation.target.owner {
            Owner::Vca(id) => Some(id),
            _ => None,
        })
        .collect();
    if automated.is_empty() {
        return false;
    }
    automated.sort_unstable();
    automated.dedup();
    let (before, after) = (reached_strips(current), reached_strips(next));
    automated
        .into_iter()
        .any(|vca| before.get(vca) != after.get(vca))
}

/// Per VCA ID of `model`, the IDs of the strips that reach it, in strip order: the inverse of
/// [`SessionModel::vca_reach`], which reads any model (a strip list in any order). A VCA no strip
/// reaches has no key.
fn reached_strips(model: &SessionModel) -> BTreeMap<&StableId, Vec<&StableId>> {
    let mut reached: BTreeMap<&StableId, Vec<&StableId>> = BTreeMap::new();
    for (strip, reach) in model.strips().zip(model.vca_reach()) {
        for vca in reach.into_iter().filter_map(|index| model.vcas.get(index)) {
            reached.entry(&vca.id).or_default().push(strip.id);
        }
    }
    reached
}

/// Whether two entries have the same `id`, `target` and `segments`, `f32` values by bits.
fn same_curve(before: &Automation, after: &Automation) -> bool {
    before.id == after.id && before.target == after.target && same_segments(before, after)
}

/// Whether two entries have the same `segments`, `f32` values by bits: the same table, whatever
/// their `id`, `target` or offset.
fn same_segments(before: &Automation, after: &Automation) -> bool {
    before.segments.len() == after.segments.len()
        && before
            .segments
            .iter()
            .zip(&after.segments)
            .all(|(before, after)| {
                before.shape == after.shape
                    && before.start_sample == after.start_sample
                    && before.end_sample == after.end_sample
                    && before.start_value.to_bits() == after.start_value.to_bits()
                    && before.end_value.to_bits() == after.end_value.to_bits()
                    && before.unit == after.unit
            })
}

/// The lanes of strip `strip`'s fader (`[left, right]`) that both models automate with the same
/// table (#1648 D2): the entry that drives the lane in `current` and the one that drives it in
/// `next` ([`fader_automation`]) have the same segments, `f32` values by bits. A successor's strip
/// join reads it: such a lane's cell continues its carried event state verbatim, and every other
/// carried cell is repositioned in its own table at adoption. #1655 D6: a fader event lane
/// continues when its own table and each reaching automated VCA `offset_db` table (in ascending
/// VCA ID) are the same set, bit for bit (a lane with no own entry in either model included).
#[must_use]
pub fn continued_fader_lanes(
    current: &SessionModel,
    next: &SessionModel,
    strip: &StableId,
) -> [bool; 2] {
    continued_lanes(
        current,
        next,
        strip,
        fader_automation,
        vca_offset_automation,
    )
}

/// The lanes of strip `strip`'s mute (`[left, right]`) that both models automate with the same
/// table, by the rule of [`continued_fader_lanes`] on the mute row ([`mute_automation`], #1651
/// T4): such a lane's mute cell continues its carried event state verbatim, and every other
/// carried mute cell is repositioned at adoption.
#[must_use]
pub fn continued_mute_lanes(
    current: &SessionModel,
    next: &SessionModel,
    strip: &StableId,
) -> [bool; 2] {
    continued_lanes(current, next, strip, mute_automation, vca_mute_automation)
}

/// The curves of one event lane in one model: its own entry, and each reaching VCA's entry on the
/// matching VCA row (`vca_entries`) with the VCA's ID, in ascending VCA ID (#1655 D6).
type LaneCurves<'a> = (Option<&'a Automation>, Vec<(&'a str, &'a Automation)>);

/// The lanes of strip `strip` whose curves both models render with the same tables (#1648 D2,
/// #1655 D6): the own entry (`entries`) is absent in both or has the same segments, and the
/// reaching VCAs with an entry on the lane (`vca_entries`, by the rule of [`fader_event_lanes`])
/// are the same VCAs, in ascending VCA ID, each with the same segments, and the lane has a curve.
/// With no VCA row automated in either model this is the own-entry rule alone, as before #1655.
fn continued_lanes<'a>(
    current: &'a SessionModel,
    next: &'a SessionModel,
    strip: &StableId,
    entries: fn(&'a SessionModel, &StableId) -> [Option<&'a Automation>; 2],
    vca_entries: fn(&'a SessionModel, &StableId) -> [Option<&'a Automation>; 2],
) -> [bool; 2] {
    if current.automation.is_empty() || next.automation.is_empty() {
        return [false; 2];
    }
    let curves = |model: &'a SessionModel| -> [LaneCurves<'a>; 2] {
        let own = entries(model, strip);
        let mut vcas = [Vec::new(), Vec::new()];
        if model.automation.iter().any(|entry| vca_row(&entry.target)) {
            for vca in model
                .vca_reach_of(strip)
                .into_iter()
                .filter_map(|index| model.vcas.get(index))
            {
                let driven = vca_entries(model, &vca.id);
                for (lane, curves) in vcas.iter_mut().enumerate() {
                    if let Some(entry) = driven[lane] {
                        curves.push((vca.id.as_str(), entry));
                    }
                }
            }
        }
        let [left, right] = vcas;
        [(own[0], left), (own[1], right)]
    };
    let (before, after) = (curves(current), curves(next));
    [0, 1].map(|lane| {
        let ((own_before, vcas_before), (own_after, vcas_after)) = (&before[lane], &after[lane]);
        let own = match (own_before, own_after) {
            (Some(before), Some(after)) => same_segments(before, after),
            (None, None) => true,
            _ => false,
        };
        own && (own_before.is_some() || !vcas_before.is_empty())
            && vcas_before.len() == vcas_after.len()
            && vcas_before
                .iter()
                .zip(vcas_after)
                .all(|((id_before, before), (id_after, after))| {
                    id_before == id_after && same_segments(before, after)
                })
    })
}

/// An entry's segments in the evaluator's type, in table order: the one conversion of a stored
/// segment, which preparation and the order rule read.
#[must_use]
pub fn stored_segments(entry: &Automation) -> Vec<Segment> {
    entry.segments.iter().map(stored_segment).collect()
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
                segments: stored_segments(entry),
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
