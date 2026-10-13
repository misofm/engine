//! The effective values (decision 17, C7): the fader, the mute and the follow gate that render
//! sees, computed from the committed session and the host's monitor overlay by one function.
//!
//! The rules are written in three allocation-free building blocks and nowhere else:
//!
//! - [`any_solo`]: whether some soloable strip is soloed;
//! - [`effective_strip`]: one strip's fader, mute and VCA mute;
//! - [`effective_gate`]: one route's gate, per source lane.
//!
//! [`effective_into`] is a loop over them, [`effective`] builds the inputs of a model and calls it,
//! and a host that updates one strip at a time calls the blocks. Nobody spells a rule again.

use core::fmt;

use session::{Route, RouteSource, SessionModel, StableId, vca_effective_db};

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
    let solo_term = any_solo && !inputs.solo_safe(strip) && !inputs.soloed(strip);
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

/// One route's gate per source lane: `mute || (follows_mute && source.mute[lane])`. `source` is
/// the route's source strip, from [`effective_strip`].
#[must_use]
pub fn effective_gate(
    inputs: &impl EffectiveInputs,
    route: usize,
    source: &EffectiveStrip,
) -> [bool; 2] {
    let route = inputs.route(route);
    [0, 1].map(|lane| route.mute || (route.follows_mute && source.mute[lane]))
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
