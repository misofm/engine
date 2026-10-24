//! The effective values (decision 17, C7): the fader, the mute and the follow gate that render
//! sees, computed from the committed session and the host's monitor overlay by one function.
//!
//! The rules are written in three allocation-free building blocks and nowhere else:
//!
//! - [`any_solo`]: whether some soloable strip is soloed;
//! - [`effective_strip`]: one strip's fader, mute and VCA mute;
//! - [`effective_follow`]: the part of one route's gate that its source strip's mute causes;
//! - [`effective_gate`]: one route's gate, per source lane.
//!
//! [`effective_into`] is a loop over them, [`effective`] builds the inputs of a model and calls it,
//! and a host that updates one strip at a time calls the blocks, over [`LocalInputs`] and
//! [`any_solo_in`], which read one strip and the overlay without building the model's tables.
//! Nobody spells a rule again.

use core::fmt;
use std::cell::OnceCell;

use session::{Route, RouteSource, SessionModel, StableId, VcaSum};

use crate::view::ModelView;

/// The host-local monitor values. They are not part of the session: they never reach a snapshot.
///
/// It holds the soloed strips, as a sorted set of IDs, and the designated master strip (#1683). A
/// soloed ID that names a submix or no strip has no effect: a submix is solo-safe.
///
/// The master is a designation, not a discovery (issue #143 D6): the strip whose gain reduction a
/// host reports as the master's. It names a strip by ID, so a strip added or removed before it
/// keeps it, and at most one strip is the master. It moves no render value.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MonitorOverlay {
    /// Sorted, with no duplicate.
    soloed: Vec<StableId>,
    /// The designated master strip, or `None`.
    master: Option<StableId>,
}

impl MonitorOverlay {
    /// An overlay that solos nothing and designates no master. `const`, so a host may keep one in
    /// a `static`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            soloed: Vec::new(),
            master: None,
        }
    }

    /// An overlay that solos exactly the named strips.
    #[must_use]
    pub fn with_soloed(ids: impl IntoIterator<Item = StableId>) -> Self {
        let mut soloed: Vec<StableId> = ids.into_iter().collect();
        soloed.sort_unstable();
        soloed.dedup();
        Self {
            soloed,
            master: None,
        }
    }

    /// The soloed IDs, sorted.
    #[must_use]
    pub fn soloed(&self) -> &[StableId] {
        &self.soloed
    }

    /// Whether `id` is soloed. Allocation-free.
    #[must_use]
    pub fn is_soloed(&self, id: &StableId) -> bool {
        self.soloed.binary_search(id).is_ok()
    }

    /// Solo or unsolo `id`. Keeps the set sorted and free of duplicates.
    pub fn set_soloed(&mut self, id: &StableId, soloed: bool) {
        match (self.soloed.binary_search(id), soloed) {
            (Err(index), true) => self.soloed.insert(index, id.clone()),
            (Ok(index), false) => {
                self.soloed.remove(index);
            }
            (Ok(_), true) | (Err(_), false) => {}
        }
    }

    /// The designated master strip, or `None` (#1683).
    #[must_use]
    pub fn master(&self) -> Option<&StableId> {
        self.master.as_ref()
    }

    /// Whether `id` is the designated master. Allocation-free.
    #[must_use]
    pub fn is_master(&self, id: &StableId) -> bool {
        self.master.as_ref() == Some(id)
    }

    /// Designate `id` the master (`on`), which ends any other strip's designation, or end its
    /// designation (`!on`); ending the designation of a strip that is not the master changes
    /// nothing.
    pub fn set_master(&mut self, id: &StableId, on: bool) {
        if on {
            self.master = Some(id.clone());
        } else if self.is_master(id) {
            self.master = None;
        }
    }

    /// Whether the two overlays solo the same strips: a change of the master alone moves no
    /// effective value.
    #[must_use]
    pub fn same_solo(&self, other: &Self) -> bool {
        self.soloed == other.soloed
    }

    /// Drop every soloed ID, and the master, that names no strip of `model`: a removed strip keeps
    /// no solo and is no longer the master.
    pub fn retain_strips_of(&mut self, model: &SessionModel) {
        self.soloed
            .retain(|id| crate::registry::strip_index(model, id).is_some());
        if self
            .master
            .as_ref()
            .is_some_and(|id| crate::registry::strip_index(model, id).is_none())
        {
            self.master = None;
        }
    }

    /// The overlay after `writes`, applied in order, on a session whose strips are `next`'s, and
    /// the ramp the change carries (decision 17, C6 step 4, #1559 D7).
    ///
    /// The ramp is that of the last solo write whose strip's solo differs between `self` and the
    /// result, and `None` (the session's `mute_ms`) when no solo write changed anything or the last
    /// such write gave no ramp; a master write has no ramp, since it moves no render value. A strip
    /// `next` no longer has is dropped from the result.
    #[must_use]
    pub fn after(&self, writes: &[MonitorWrite], next: &SessionModel) -> MonitorChange {
        let mut overlay = self.clone();
        for write in writes {
            match write.row {
                MonitorRow::Solo => overlay.set_soloed(&write.strip, write.on),
                MonitorRow::Master => overlay.set_master(&write.strip, write.on),
            }
        }
        overlay.retain_strips_of(next);
        let ramp = writes
            .iter()
            .rev()
            .find(|write| {
                write.row == MonitorRow::Solo
                    && overlay.is_soloed(&write.strip) != self.is_soloed(&write.strip)
            })
            .and_then(|write| write.ramp);
        MonitorChange { overlay, ramp }
    }
}

/// Which monitor row a [`MonitorWrite`] sets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MonitorRow {
    /// The strip's `solo` (#1559).
    Solo,
    /// The strip's `master` designation (#1683).
    Master,
}

/// One validated write to a monitor row (solo or master): the strip, the row, the new state and
/// the explicit ramp length of the write, in samples. See [`check_monitor`](crate::check_monitor).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MonitorWrite {
    /// The strip whose row the write sets.
    pub strip: StableId,
    /// The row it sets.
    pub row: MonitorRow,
    /// Whether the row is on after the write: the strip soloed, or designated the master.
    pub on: bool,
    /// The write's own ramp length in samples; `None` is the session's `mute_ms`. A master row
    /// takes none.
    pub ramp: Option<u32>,
}

/// What [`MonitorOverlay::after`] returns.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MonitorChange {
    /// The overlay after the writes.
    pub overlay: MonitorOverlay,
    /// The ramp length that the records the change produces take, in samples; `None` is the
    /// session's `mute_ms`.
    pub ramp: Option<u32>,
}

/// One strip's effective values, per lane (`[left, right]`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EffectiveStrip {
    /// The fader in dB: [`session::vca_effective_db`] over the reaching VCAs' static offsets
    /// (an automated VCA lane's static offset is its fallback, #1655 D5: a lane it reaches is a
    /// fader event lane, which takes no `FaderDb` record).
    pub fader_db: [f32; 2],
    /// The mute: the strip's own, any reaching VCA's, or the solo term.
    pub mute: [bool; 2],
    /// Whether any reaching VCA mutes the lane.
    pub vca_mute: [bool; 2],
    /// `S`, the lane's offset sum (#1646 D1, D6; #1650 D6; #1655 D2): the `f64` sum of the lane's
    /// own hand-move offset ([`Self::fader_offset`], when an entry drives the lane) first, then the
    /// static offsets of the reaching VCAs whose `offset_db` lane no rendered entry drives, then
    /// the hand-move offsets of the reaching VCAs whose lane one does, each group in ascending
    /// VCA-ID order ([`session::vca_offsets_sum`]'s order), and `+0.0` when there is none. A
    /// stored fader automation value `v` renders as `session::vca_compose_db(v, S)`, and each
    /// reaching VCA curve adds its value after `S` (D2). With no VCA automation the middle group
    /// is every reaching VCA's static offset and the last is empty, so `S` keeps its bits. The
    /// static path composes the member with the VCA offsets alone (`fader_db`): a lane with no
    /// driving entry has no hand offset, so the two agree there.
    pub offsets_sum: [f64; 2],
    /// The hand-move offset of the fader-row stored automation entry ([`crate::fader_row`]) that
    /// drives each lane (#1650 D3), or `None` for a lane no fader-row entry drives.
    pub fader_offset: [Option<f32>; 2],
}

/// One route's effective values.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EffectiveRoute {
    /// Per source lane (index = source lane): the route's mute, or its follow gate on a muted lane
    /// of its source strip. A gated lane zeroes that source lane's matrix column.
    pub gate: [bool; 2],
    /// Per source lane: the part of the gate that following causes, `follows_mute` on a muted lane
    /// of the source strip. The graph keeps it apart from the route's own mute (`graph::RouteGate`).
    pub follow: [bool; 2],
}

/// Every effective value of a session. Strips are in [`SessionModel::strips`] order and routes in
/// `model.routes` order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Effective {
    /// One entry per strip.
    pub strips: Box<[EffectiveStrip]>,
    /// One entry per route.
    pub routes: Box<[EffectiveRoute]>,
}

impl Effective {
    /// A zeroed value of the given shape. This and [`effective`] allocate; nothing else here does.
    #[must_use]
    pub fn with_shape(strips: usize, routes: usize) -> Self {
        Self {
            strips: vec![EffectiveStrip::default(); strips].into_boxed_slice(),
            routes: vec![EffectiveRoute::default(); routes].into_boxed_slice(),
        }
    }
}

/// [`effective_into`] was given an [`Effective`] of another shape than its inputs. Nothing was
/// written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectiveShapeError {
    /// The strip and route counts of the inputs.
    pub expected: (usize, usize),
    /// The strip and route counts of the `Effective`.
    pub actual: (usize, usize),
}

impl fmt::Display for EffectiveShapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "effective shape mismatch: inputs have {} strips and {} routes, the value has {} and {}",
            self.expected.0, self.expected.1, self.actual.0, self.actual.1
        )
    }
}

impl std::error::Error for EffectiveShapeError {}

/// What [`EffectiveInputs::route`] reads of one route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectiveRouteInput {
    /// The route's own mute.
    pub mute: bool,
    /// Whether the route follows its source strip's mute.
    pub follows_mute: bool,
    /// The strip index of the route's source. An index at or past `strip_count()` names no strip
    /// (an unvalidated model); the route then follows nothing.
    pub source_strip: usize,
}

/// The committed values the effective functions read, without allocating. Strips are indexed in
/// [`SessionModel::strips`] order.
pub trait EffectiveInputs {
    /// The number of strips.
    fn strip_count(&self) -> usize;
    /// A strip's own fader (`[left, right]` dB) and mute.
    fn own_fader(&self, strip: usize) -> ([f32; 2], [bool; 2]);
    /// Whether the strip is soloed.
    fn soloed(&self, strip: usize) -> bool;
    /// Whether the strip is solo-safe (every submix is).
    fn solo_safe(&self, strip: usize) -> bool;
    /// The VCAs that reach the strip (directly or through nesting), each once, as VCA indices in
    /// ascending VCA-ID order.
    fn reach(&self, strip: usize) -> &[usize];
    /// A VCA's offset (`[left, right]` dB) and mute.
    fn vca(&self, vca: usize) -> ([f32; 2], [bool; 2]);
    /// The hand-move offset (dB) of the rendered stored automation entry that drives each of the
    /// strip's fader lanes, or `None` for a lane no rendered entry drives (#1650 D3, D6).
    fn fader_offset(&self, strip: usize) -> [Option<f32>; 2];
    /// The hand-move offset (dB) of the rendered stored automation entry that drives each of a
    /// VCA's `offset_db` lanes, or `None` for a lane no rendered entry drives (#1655 D2, D5).
    fn vca_offset(&self, vca: usize) -> [Option<f32>; 2];
    /// The number of routes.
    fn route_count(&self) -> usize;
    /// One route's inputs.
    fn route(&self, route: usize) -> EffectiveRouteInput;
}

/// Whether some soloable strip is soloed. A soloed solo-safe strip does not count.
#[must_use]
pub fn any_solo(inputs: &impl EffectiveInputs) -> bool {
    (0..inputs.strip_count()).any(|strip| inputs.soloed(strip) && !inputs.solo_safe(strip))
}

/// The solo term of one strip's mute: some soloable strip is soloed (`any_solo`, from
/// [`any_solo`]) and this strip is neither soloed nor solo-safe. The only spelling of the rule;
/// [`effective_strip`] and a host that must tell a solo's mute from another cause call it.
#[must_use]
pub fn solo_term(inputs: &impl EffectiveInputs, strip: usize, any_solo: bool) -> bool {
    any_solo && !inputs.solo_safe(strip) && !inputs.soloed(strip)
}

/// One strip's effective values. `any_solo` is [`any_solo`]'s result for the same inputs.
///
/// Per lane: `fader_db = vca_effective_db(own, the reaching VCAs' offsets in ascending VCA-ID
/// order)`; `offsets_sum` = the `f64` sum of the lane's hand-move offset (when a rendered entry
/// drives the lane), then the static offsets of the reaching VCAs no rendered entry drives on the
/// lane, then the hand-move offsets of those one does, each group in ascending VCA-ID order, or
/// `+0.0` with none (#1650 D6, #1655 D2); `vca_mute` =
/// any reaching VCA mutes; `mute = own || vca_mute || solo_term` with
/// `solo_term = any_solo && !solo_safe && !soloed`.
#[must_use]
pub fn effective_strip(
    inputs: &impl EffectiveInputs,
    strip: usize,
    any_solo: bool,
) -> EffectiveStrip {
    let (own_db, own_mute) = inputs.own_fader(strip);
    let reach = inputs.reach(strip);
    let solo_term = solo_term(inputs, strip, any_solo);
    let mut out = EffectiveStrip {
        fader_offset: inputs.fader_offset(strip),
        ..EffectiveStrip::default()
    };
    // One read of each reaching VCA feeds both lanes; the sum is `vca_effective_db`'s: the
    // offsets first, then the member. `S` is the lane's hand offset first, then the same offsets
    // in the same order (#1650 D6).
    let mut sums = [VcaSum::new(); 2];
    let mut offsets = [VcaSum::new(); 2];
    for (sum, hand) in offsets.iter_mut().zip(out.fader_offset) {
        if let Some(hand) = hand {
            sum.add(hand);
        }
    }
    // The static offsets of the VCA lanes no entry drives, then (a second pass, only when some
    // reaching VCA lane is automated) the hand offsets of those one drives (#1655 D2).
    let mut automated = false;
    for &vca in reach {
        let (offset_db, muted) = inputs.vca(vca);
        let hand = inputs.vca_offset(vca);
        for lane in 0..2 {
            sums[lane].add(offset_db[lane]);
            if hand[lane].is_none() {
                offsets[lane].add(offset_db[lane]);
            } else {
                automated = true;
            }
            out.vca_mute[lane] |= muted[lane];
        }
    }
    if automated {
        for &vca in reach {
            for (sum, hand) in offsets.iter_mut().zip(inputs.vca_offset(vca)) {
                if let Some(hand) = hand {
                    sum.add(hand);
                }
            }
        }
    }
    for lane in 0..2 {
        out.fader_db[lane] = sums[lane].compose(own_db[lane]);
        out.offsets_sum[lane] = offsets[lane].sum().unwrap_or(0.0);
        out.mute[lane] = own_mute[lane] || out.vca_mute[lane] || solo_term;
    }
    out
}

/// The part of one route's gate that following causes, per source lane:
/// `follows_mute && source.mute[lane]`. `source` is the route's source strip, from
/// [`effective_strip`].
#[must_use]
pub fn effective_follow(
    inputs: &impl EffectiveInputs,
    route: usize,
    source: &EffectiveStrip,
) -> [bool; 2] {
    let route = inputs.route(route);
    [0, 1].map(|lane| route.follows_mute && source.mute[lane])
}

/// One route's gate per source lane: `mute || (follows_mute && source.mute[lane])`, the route's
/// own mute or [`effective_follow`]. `source` is the route's source strip, from
/// [`effective_strip`].
#[must_use]
pub fn effective_gate(
    inputs: &impl EffectiveInputs,
    route: usize,
    source: &EffectiveStrip,
) -> [bool; 2] {
    let mute = inputs.route(route).mute;
    effective_follow(inputs, route, source).map(|follow| mute || follow)
}

/// Compute every effective value into `out`. Never allocates.
///
/// # Errors
///
/// [`EffectiveShapeError`] if `out` does not have one entry per strip and per route of `inputs`;
/// nothing is written then.
pub fn effective_into(
    inputs: &impl EffectiveInputs,
    out: &mut Effective,
) -> Result<(), EffectiveShapeError> {
    let expected = (inputs.strip_count(), inputs.route_count());
    let actual = (out.strips.len(), out.routes.len());
    if expected != actual {
        return Err(EffectiveShapeError { expected, actual });
    }
    fill(inputs, out);
    Ok(())
}

/// [`effective_into`] for an `out` already known to have the inputs' shape.
fn fill(inputs: &impl EffectiveInputs, out: &mut Effective) {
    let any_solo = any_solo(inputs);
    for (strip, slot) in out.strips.iter_mut().enumerate() {
        *slot = effective_strip(inputs, strip, any_solo);
    }
    for (route, slot) in out.routes.iter_mut().enumerate() {
        let source = inputs.route(route).source_strip;
        let source = out.strips.get(source).copied().unwrap_or_default();
        *slot = EffectiveRoute {
            gate: effective_gate(inputs, route, &source),
            follow: effective_follow(inputs, route, &source),
        };
    }
}

/// Every effective value of `model` under `overlay`. One code path with [`effective_into`].
#[must_use]
pub fn effective(model: &SessionModel, overlay: &MonitorOverlay) -> Effective {
    let inputs = ModelInputs::new(model, overlay);
    let mut out = Effective::with_shape(inputs.strip_count(), inputs.route_count());
    fill(&inputs, &mut out);
    out
}

/// [`EffectiveInputs`] over a session model and a monitor overlay.
///
/// Built once (it allocates its VCA reach and route-source tables); reads then allocate nothing.
#[derive(Clone, Debug)]
pub struct ModelInputs<'a> {
    model: &'a SessionModel,
    overlay: &'a MonitorOverlay,
    /// [`SessionModel::vca_reach`], per strip.
    reach: Vec<Vec<usize>>,
    /// The strip index of each route's source, or the strip count when it names none.
    sources: Vec<usize>,
    /// The hand-move offsets of each strip's automated fader lanes (#1650), indexed by strip;
    /// empty when the model stores no rendered fader automation.
    fader_offsets: Vec<[Option<f32>; 2]>,
    /// The hand-move offsets of each VCA's automated `offset_db` lanes (#1655), indexed by VCA;
    /// empty when the model stores no rendered VCA offset automation.
    vca_offsets: Vec<[Option<f32>; 2]>,
}

impl<'a> ModelInputs<'a> {
    /// The inputs of `model` under `overlay`.
    #[must_use]
    pub fn new(model: &'a SessionModel, overlay: &'a MonitorOverlay) -> Self {
        let strip_count = model.tracks.len() + model.submixes.len();
        let sources = model
            .routes
            .iter()
            .map(|route| {
                crate::registry::strip_index(model, source_id(route)).unwrap_or(strip_count)
            })
            .collect();
        Self {
            model,
            overlay,
            reach: model.vca_reach(),
            sources,
            fader_offsets: model_fader_offsets(model),
            vca_offsets: model_vca_offsets(model),
        }
    }

    fn strip_id(&self, strip: usize) -> Option<&StableId> {
        let tracks = self.model.tracks.len();
        if strip < tracks {
            Some(&self.model.tracks[strip].id)
        } else {
            self.model.submixes.get(strip - tracks).map(|s| &s.id)
        }
    }
}

/// The [`EffectiveInputs::fader_offset`] of every strip of `model`, by strip index: one pass over
/// the automation table ([`crate::fader_automation`]'s rule: the first fader-row entry,
/// [`crate::fader_row`], that drives a lane), or an empty table when no entry is on a fader row.
/// An entry on another rendered row (the mute row, #1651) gives no lane an offset.
pub(crate) fn model_fader_offsets(model: &SessionModel) -> Vec<[Option<f32>; 2]> {
    let mut offsets = Vec::new();
    for entry in &model.automation {
        let target = &entry.target;
        let session::Owner::Strip(id) = &target.owner else {
            continue;
        };
        if !crate::fader_row(target) {
            continue;
        }
        let Some(index) = crate::registry::strip_index(model, id) else {
            continue;
        };
        if offsets.is_empty() {
            offsets = vec![[None; 2]; model.tracks.len() + model.submixes.len()];
        }
        keep_first_offsets(&mut offsets[index], entry);
    }
    offsets
}

/// The [`EffectiveInputs::vca_offset`] of every VCA of `model`, by VCA index: one pass over the
/// automation table ([`crate::vca_offset_automation`]'s rule: the first VCA `offset_db` entry,
/// [`crate::vca_offset_row`], that drives a lane, #1710), or an empty table when no entry is on a
/// VCA `offset_db` row.
fn model_vca_offsets(model: &SessionModel) -> Vec<[Option<f32>; 2]> {
    let mut offsets = Vec::new();
    for entry in &model.automation {
        let target = &entry.target;
        let session::Owner::Vca(id) = &target.owner else {
            continue;
        };
        if !crate::vca_offset_row(target) {
            continue;
        }
        let Some(index) = crate::view::position(&model.vcas, id, |vca| &vca.id) else {
            continue;
        };
        if offsets.is_empty() {
            offsets = vec![[None; 2]; model.vcas.len()];
        }
        keep_first_offsets(&mut offsets[index], entry);
    }
    offsets
}

/// Give each lane of `lanes` that `entry` drives and that has no offset yet `entry`'s hand-move
/// offset: the first entry per lane in table order.
fn keep_first_offsets(lanes: &mut [Option<f32>; 2], entry: &session::Automation) {
    let lane = entry.target.lane;
    for (index, (drives, offset)) in [
        (
            matches!(lane, session::Lane::Left | session::Lane::Both),
            entry.offset.left_db,
        ),
        (
            matches!(lane, session::Lane::Right | session::Lane::Both),
            entry.offset.right_db,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        if drives && lanes[index].is_none() {
            lanes[index] = Some(offset);
        }
    }
}

fn source_id(route: &Route) -> &StableId {
    match &route.source {
        RouteSource::Track { track_id, .. } => track_id,
        RouteSource::Submix { submix_id, .. } => submix_id,
    }
}

impl EffectiveInputs for ModelInputs<'_> {
    fn strip_count(&self) -> usize {
        self.model.tracks.len() + self.model.submixes.len()
    }

    fn own_fader(&self, strip: usize) -> ([f32; 2], [bool; 2]) {
        let tracks = self.model.tracks.len();
        let fader = if strip < tracks {
            &self.model.tracks[strip].fader
        } else {
            &self.model.submixes[strip - tracks].fader
        };
        (
            [fader.left_db, fader.right_db],
            [fader.left_mute, fader.right_mute],
        )
    }

    fn soloed(&self, strip: usize) -> bool {
        self.strip_id(strip)
            .is_some_and(|id| self.overlay.is_soloed(id))
    }

    fn solo_safe(&self, strip: usize) -> bool {
        strip >= self.model.tracks.len()
    }

    fn reach(&self, strip: usize) -> &[usize] {
        &self.reach[strip]
    }

    fn vca(&self, vca: usize) -> ([f32; 2], [bool; 2]) {
        let fader = &self.model.vcas[vca].fader;
        (
            [fader.left_db, fader.right_db],
            [fader.left_mute, fader.right_mute],
        )
    }

    fn fader_offset(&self, strip: usize) -> [Option<f32>; 2] {
        self.fader_offsets.get(strip).copied().unwrap_or([None; 2])
    }

    fn vca_offset(&self, vca: usize) -> [Option<f32>; 2] {
        self.vca_offsets.get(vca).copied().unwrap_or([None; 2])
    }

    fn route_count(&self) -> usize {
        self.model.routes.len()
    }

    fn route(&self, route: usize) -> EffectiveRouteInput {
        let model = &self.model.routes[route];
        EffectiveRouteInput {
            mute: model.mute,
            follows_mute: model.follows_mute,
            source_strip: self.sources[route],
        }
    }
}

/// Whether some soloable strip of `model` is soloed under `overlay`: [`any_solo`] over
/// [`ModelInputs`], read from the overlay's sorted IDs instead of from every strip. A soloed ID that
/// names a submix, or no strip, does not count. Allocation-free; the cost is the number of soloed
/// strips times a binary search over the tracks, which a compiled session's normalized model lists
/// in ID order.
#[must_use]
pub fn any_solo_in(model: &SessionModel, overlay: &MonitorOverlay) -> bool {
    overlay.soloed().iter().any(|id| {
        model
            .tracks
            .binary_search_by(|track| track.id.cmp(id))
            .is_ok()
    })
}

/// [`EffectiveInputs`] for **one strip** of a model, and any of its routes (#1612 D7): the inputs
/// of [`effective_strip`] for that strip and of [`effective_follow`] and [`effective_gate`] for
/// any route, with no table built for the other strips. The strip's VCA reach is
/// [`SessionModel::vca_reach_of`]; a route's source strip is found by binary search over the
/// sorted strip IDs (tracks, then submixes), so the model must be a compiled session's normalized
/// model.
///
/// The model is read through a [`ModelView`] (#1613), so the strip, the VCAs and the routes are the
/// ones the view holds edited.
///
/// [`EffectiveInputs::reach`] answers for the strip given to [`Self::new`] only; a call for another
/// strip is a caller fault (a debug assertion).
#[derive(Clone, Debug)]
pub struct LocalInputs<'a> {
    view: ModelView<'a>,
    overlay: &'a MonitorOverlay,
    strip: usize,
    reach: Vec<usize>,
}

impl<'a> LocalInputs<'a> {
    /// The inputs for strip number `strip` of `model` under `overlay`; `None` if the model has no
    /// such strip.
    #[must_use]
    pub fn new(model: &'a SessionModel, overlay: &'a MonitorOverlay, strip: usize) -> Option<Self> {
        Self::of(ModelView::whole(model), overlay, strip)
    }

    /// [`Self::new`] over a [`ModelView`].
    #[must_use]
    pub fn of(view: ModelView<'a>, overlay: &'a MonitorOverlay, strip: usize) -> Option<Self> {
        Self::of_nested(view, overlay, strip, &OnceCell::new())
    }

    /// [`Self::of`] for a caller that builds the inputs of many strips of one model: `nesting`
    /// carries the VCA tree between them ([`SessionModel::vca_reach_of_in`]).
    #[must_use]
    pub fn of_nested(
        view: ModelView<'a>,
        overlay: &'a MonitorOverlay,
        strip: usize,
        nesting: &OnceCell<session::VcaNesting>,
    ) -> Option<Self> {
        let id = strip_id_at(view.structure(), strip)?;
        Some(Self {
            view,
            overlay,
            strip,
            reach: view.structure().vca_reach_of_in(nesting, id),
        })
    }

    /// The VCAs that reach the strip: [`SessionModel::vca_reach_of`], handed over so a caller that
    /// keeps the strip's values keeps its reach without a second table.
    #[must_use]
    pub fn into_reach(self) -> Vec<usize> {
        self.reach
    }
}

/// The ID of strip number `strip` of `model` (a track, then a submix).
fn strip_id_at(model: &SessionModel, strip: usize) -> Option<&StableId> {
    match model.tracks.get(strip) {
        Some(track) => Some(&track.id),
        None => model
            .submixes
            .get(strip - model.tracks.len())
            .map(|submix| &submix.id),
    }
}

impl EffectiveInputs for LocalInputs<'_> {
    fn strip_count(&self) -> usize {
        let model = self.view.structure();
        model.tracks.len() + model.submixes.len()
    }

    fn own_fader(&self, strip: usize) -> ([f32; 2], [bool; 2]) {
        let fader = self.view.strip(strip).fader;
        (
            [fader.left_db, fader.right_db],
            [fader.left_mute, fader.right_mute],
        )
    }

    fn soloed(&self, strip: usize) -> bool {
        strip_id_at(self.view.structure(), strip).is_some_and(|id| self.overlay.is_soloed(id))
    }

    fn solo_safe(&self, strip: usize) -> bool {
        strip >= self.view.structure().tracks.len()
    }

    fn reach(&self, strip: usize) -> &[usize] {
        debug_assert_eq!(
            strip, self.strip,
            "LocalInputs reads the reach of one strip"
        );
        &self.reach
    }

    fn vca(&self, vca: usize) -> ([f32; 2], [bool; 2]) {
        let fader = &self.view.vca(vca).fader;
        (
            [fader.left_db, fader.right_db],
            [fader.left_mute, fader.right_mute],
        )
    }

    fn fader_offset(&self, strip: usize) -> [Option<f32>; 2] {
        strip_id_at(self.view.structure(), strip).map_or([None; 2], |id| {
            crate::builtin_rows::fader_offsets(self.view.fader_automation(id))
        })
    }

    fn vca_offset(&self, vca: usize) -> [Option<f32>; 2] {
        let id = &self.view.structure().vcas[vca].id;
        crate::builtin_rows::fader_offsets(self.view.vca_automation(id))
    }

    fn route_count(&self) -> usize {
        self.view.structure().routes.len()
    }

    fn route(&self, route: usize) -> EffectiveRouteInput {
        let model = self.view.structure();
        let edited = self.view.route(route);
        let id = source_id(&model.routes[route]);
        let tracks = model.tracks.len();
        let source_strip = model
            .tracks
            .binary_search_by(|track| track.id.cmp(id))
            .ok()
            .or_else(|| {
                model
                    .submixes
                    .binary_search_by(|submix| submix.id.cmp(id))
                    .ok()
                    .map(|at| tracks + at)
            })
            .unwrap_or(self.strip_count());
        EffectiveRouteInput {
            mute: edited.mute,
            follows_mute: edited.follows_mute,
            source_strip,
        }
    }
}

/// A source of the effective values [`crate::owner_rows_in`] and [`crate::derived_rows_in`] read: the
/// dense [`Effective`] of a whole model, or [`LocalEffective`], which computes one strip's or route's
/// values when asked (#1613). Both come from the one set of rules above.
pub trait EffectiveSource {
    /// Strip number `index`'s effective values.
    fn strip(&self, index: usize) -> EffectiveStrip;
    /// Route number `index`'s effective gate, per source lane.
    fn route_gate(&self, index: usize) -> [bool; 2];
}

impl EffectiveSource for Effective {
    fn strip(&self, index: usize) -> EffectiveStrip {
        self.strips[index]
    }

    fn route_gate(&self, index: usize) -> [bool; 2] {
        self.routes[index].gate
    }
}

/// The effective values of **the strips and routes asked for**, with no table of the others (#1613):
/// [`effective_strip`], [`effective_follow`] and [`effective_gate`] over [`LocalInputs`], with the
/// solo term from [`any_solo_in`]. The values are those [`effective`] gives for the same model and
/// overlay.
///
/// The VCA tree of the model is built once, the first time a strip is held by a VCA, and serves
/// every strip asked for after it.
#[derive(Clone, Debug)]
pub struct LocalEffective<'a> {
    view: ModelView<'a>,
    overlay: &'a MonitorOverlay,
    any_solo: bool,
    nesting: OnceCell<session::VcaNesting>,
}

impl<'a> LocalEffective<'a> {
    /// The effective values of the model `view` stands for under `overlay`.
    #[must_use]
    pub fn new(view: ModelView<'a>, overlay: &'a MonitorOverlay) -> Self {
        Self {
            view,
            overlay,
            any_solo: any_solo_in(view.structure(), overlay),
            nesting: OnceCell::new(),
        }
    }

    /// Strip number `index`'s effective values.
    ///
    /// # Panics
    ///
    /// If the model has no such strip.
    #[must_use]
    pub fn strip(&self, index: usize) -> EffectiveStrip {
        let inputs = LocalInputs::of_nested(self.view, self.overlay, index, &self.nesting)
            .expect("the model has the strip");
        effective_strip(&inputs, index, self.any_solo)
    }

    /// Route number `index`'s effective gate and follow.
    ///
    /// # Panics
    ///
    /// If the model has no such route.
    #[must_use]
    pub fn route(&self, index: usize) -> EffectiveRoute {
        let source = {
            let probe = self.view.structure();
            // The source strip of the route, and an unvalidated model's missing one.
            let id = source_id(&probe.routes[index]);
            crate::registry::strip_index(probe, id)
        };
        let Some(source) = source else {
            return EffectiveRoute::default_gate(self.view.route(index));
        };
        let strip = self.strip(source);
        let inputs = LocalInputs::of_nested(self.view, self.overlay, source, &self.nesting)
            .expect("the source exists");
        EffectiveRoute {
            gate: effective_gate(&inputs, index, &strip),
            follow: effective_follow(&inputs, index, &strip),
        }
    }
}

impl EffectiveRoute {
    /// The gate of a route whose source names no strip: its own mute, with nothing to follow. The
    /// dense [`effective`] gives a missing source the default strip (unmuted), so the same.
    fn default_gate(route: &Route) -> Self {
        Self {
            gate: [route.mute; 2],
            follow: [false; 2],
        }
    }
}

impl EffectiveSource for LocalEffective<'_> {
    fn strip(&self, index: usize) -> EffectiveStrip {
        LocalEffective::strip(self, index)
    }

    fn route_gate(&self, index: usize) -> [bool; 2] {
        self.route(index).gate
    }
}
