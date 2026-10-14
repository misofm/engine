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

use session::{Route, RouteSource, SessionModel, StableId, vca_effective_db};

use crate::view::ModelView;

/// The host-local monitor values. They are not part of the session: they never reach a snapshot.
///
/// In this slice it holds only the soloed strips, as a sorted set of IDs. A soloed ID that names a
/// submix or no strip has no effect: a submix is solo-safe.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MonitorOverlay {
    /// Sorted, with no duplicate.
    soloed: Vec<StableId>,
}

impl MonitorOverlay {
    /// An overlay that solos nothing. `const`, so a host may keep one in a `static`.
    #[must_use]
    pub const fn new() -> Self {
        Self { soloed: Vec::new() }
    }

    /// An overlay that solos exactly the named strips.
    #[must_use]
    pub fn with_soloed(ids: impl IntoIterator<Item = StableId>) -> Self {
        let mut soloed: Vec<StableId> = ids.into_iter().collect();
        soloed.sort_unstable();
        soloed.dedup();
        Self { soloed }
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

    /// Drop every soloed ID that names no strip of `model`: a removed strip keeps no solo.
    pub fn retain_strips_of(&mut self, model: &SessionModel) {
        self.soloed
            .retain(|id| crate::registry::strip_index(model, id).is_some());
    }

    /// The overlay after `writes`, applied in order, on a session whose strips are `next`'s, and
    /// the ramp the change carries (decision 17, C6 step 4, #1559 D7).
    ///
    /// The ramp is that of the last write whose strip's solo differs between `self` and the
    /// result, and `None` (the session's `mute_ms`) when no write changed anything or the last
    /// such write gave no ramp. A strip `next` no longer has is dropped from the result.
    #[must_use]
    pub fn after(&self, writes: &[MonitorWrite], next: &SessionModel) -> MonitorChange {
        let mut overlay = self.clone();
        for write in writes {
            overlay.set_soloed(&write.strip, write.soloed);
        }
        overlay.retain_strips_of(next);
        let ramp = writes
            .iter()
            .rev()
            .find(|write| overlay.is_soloed(&write.strip) != self.is_soloed(&write.strip))
            .and_then(|write| write.ramp);
        MonitorChange { overlay, ramp }
    }
}

/// One validated write to a monitor row (solo): the strip, the new state and the explicit ramp
/// length of the write, in samples. See [`check_monitor`](crate::check_monitor).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MonitorWrite {
    /// The strip whose solo the write sets.
    pub strip: StableId,
    /// Whether the strip is soloed after the write.
    pub soloed: bool,
    /// The write's own ramp length in samples; `None` is the session's `mute_ms`.
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
    /// The fader in dB: [`vca_effective_db`] over the reaching VCAs' offsets.
    pub fader_db: [f32; 2],
    /// The mute: the strip's own, any reaching VCA's, or the solo term.
    pub mute: [bool; 2],
    /// Whether any reaching VCA mutes the lane.
    pub vca_mute: [bool; 2],
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
/// order)`; `vca_mute` = any reaching VCA mutes; `mute = own || vca_mute || solo_term` with
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
    let mut out = EffectiveStrip::default();
    for lane in 0..2 {
        out.fader_db[lane] = vca_effective_db(
            own_db[lane],
            reach.iter().map(|&vca| inputs.vca(vca).0[lane]),
        );
        out.vca_mute[lane] = reach.iter().any(|&vca| inputs.vca(vca).1[lane]);
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
                let id = source_id(route);
                model
                    .strips()
                    .position(|strip| strip.id == id)
                    .unwrap_or(strip_count)
            })
            .collect();
        Self {
            model,
            overlay,
            reach: model.vca_reach(),
            sources,
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

/// [`EffectiveInputs`] for **one strip** of a model, and any of its routes (TBD9 D7): the inputs
/// of [`effective_strip`] for that strip and of [`effective_follow`] and [`effective_gate`] for
/// any route, with no table built for the other strips. The strip's VCA reach is
/// [`SessionModel::vca_reach_of`]; a route's source strip is found by binary search over the
/// sorted strip IDs (tracks, then submixes), so the model must be a compiled session's normalized
/// model.
///
/// The model is read through a [`ModelView`] (TBD10), so the strip, the VCAs and the routes are the
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
        let id = strip_id_at(view.structure(), strip)?;
        Some(Self {
            view,
            overlay,
            strip,
            reach: view.structure().vca_reach_of(id),
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
/// values when asked (TBD10). Both come from the one set of rules above.
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

/// The effective values of **the strips and routes asked for**, with no table of the others (TBD10):
/// [`effective_strip`], [`effective_follow`] and [`effective_gate`] over [`LocalInputs`], with the
/// solo term from [`any_solo_in`]. The values are those [`effective`] gives for the same model and
/// overlay.
#[derive(Clone, Copy, Debug)]
pub struct LocalEffective<'a> {
    view: ModelView<'a>,
    overlay: &'a MonitorOverlay,
    any_solo: bool,
}

impl<'a> LocalEffective<'a> {
    /// The effective values of the model `view` stands for under `overlay`.
    #[must_use]
    pub fn new(view: ModelView<'a>, overlay: &'a MonitorOverlay) -> Self {
        Self {
            view,
            overlay,
            any_solo: any_solo_in(view.structure(), overlay),
        }
    }

    /// Strip number `index`'s effective values.
    ///
    /// # Panics
    ///
    /// If the model has no such strip.
    #[must_use]
    pub fn strip(&self, index: usize) -> EffectiveStrip {
        let inputs =
            LocalInputs::of(self.view, self.overlay, index).expect("the model has the strip");
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
        let inputs = LocalInputs::of(self.view, self.overlay, source).expect("the source exists");
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
