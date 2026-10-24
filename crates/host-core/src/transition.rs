//! Remove a strip in two phases: ramp it out on the running plan, then adopt the successor at a
//! scheduled sample (issue #1325, decision 15 D15-9).
//!
//! A structural transaction that removes a track or a submix while audio plays does not cut the
//! strip at the swap block. The control plane first writes a ramped mute of every removed strip,
//! and of every route out of it whose tap precedes its fader, into the running plan's cells
//! (phase 1); it then publishes the successor to adopt no earlier than
//! `S = ceil_q(p + q + N + C)` (phase 2, [`StripTransition::not_before`]). [`plan_strip_transition`]
//! is the pure step that decides what phase 1 writes. It allocates and runs only on the control
//! thread, before the commit; the records it returns are written after the commit and cannot
//! fail.
//!
//! The running plan keeps a [`DuckOverlay`] while the successor waits: the strips and routes
//! phase 1 wrote, the removed sources that still feed that plan, and `S`. A later transaction in
//! the same phase 1 writes no second ramp for anything in it and inherits its `S`.

use builtins_compiler::{FaderOffsetsRecord, TrackFaderRecord};
use session::{EditRamps, Route, RouteSource, SessionModel, StableId};

use crate::live_delta::classify_live_delta_with_monitor;
use crate::prepare::{LiveRamps, LiveRebuild, MonitorBase, SendRecord, SuccessorBase};

/// What phase 1 wrote to a running plan while its successor waits (#1325 D6): kept with that
/// plan's epoch and dropped when the epoch stops running.
///
/// Every list is sorted and holds no duplicate. An empty overlay is "not in phase 1".
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DuckOverlay {
    strips: Vec<Box<str>>,
    routes: Vec<Box<str>>,
    sources: Vec<Box<str>>,
    not_before: u64,
}

impl DuckOverlay {
    /// The empty overlay: nothing ducks.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            strips: Vec::new(),
            routes: Vec::new(),
            sources: Vec::new(),
            not_before: 0,
        }
    }

    /// Whether the overlay holds nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.strips.is_empty() && self.routes.is_empty() && self.sources.is_empty()
    }

    /// The strips whose mute phase 1 wrote, sorted.
    #[must_use]
    pub fn strips(&self) -> &[Box<str>] {
        &self.strips
    }

    /// The routes whose mute or gate phase 1 wrote, sorted.
    #[must_use]
    pub fn routes(&self) -> &[Box<str>] {
        &self.routes
    }

    /// The sources the successor removes that still feed the running plan (#1325 D4), sorted.
    #[must_use]
    pub fn sources(&self) -> &[Box<str>] {
        &self.sources
    }

    /// `S`, the earliest block start at which a successor may be adopted, or `None` for an empty
    /// overlay.
    #[must_use]
    pub fn not_before(&self) -> Option<u64> {
        (!self.is_empty()).then_some(self.not_before)
    }

    /// Whether `id` names a strip in the overlay.
    #[must_use]
    pub fn has_strip(&self, id: &str) -> bool {
        self.strips
            .binary_search_by(|held| (**held).cmp(id))
            .is_ok()
    }

    /// Whether `id` names a route in the overlay.
    #[must_use]
    pub fn has_route(&self, id: &str) -> bool {
        self.routes
            .binary_search_by(|held| (**held).cmp(id))
            .is_ok()
    }

    /// Whether `id` names a source in the overlay: one binary search, no allocation, so a source
    /// call may route by it on any thread the facade runs on.
    #[must_use]
    pub fn has_source(&self, id: &[u8]) -> bool {
        self.sources
            .binary_search_by(|held| held.as_bytes().cmp(id))
            .is_ok()
    }

    /// Lay the overlay over `model`: each overlay strip's two channels muted and each overlay
    /// route muted, which is what the running plan's cells hold once the ramps settle. A name
    /// the model does not hold is skipped.
    pub fn apply_to(&self, model: &mut SessionModel) {
        for track in &mut model.tracks {
            if self.has_strip(track.id.as_str()) {
                track.fader.left_mute = true;
                track.fader.right_mute = true;
            }
        }
        for submix in &mut model.submixes {
            if self.has_strip(submix.id.as_str()) {
                submix.fader.left_mute = true;
                submix.fader.right_mute = true;
            }
        }
        for route in &mut model.routes {
            if self.has_route(route.id.as_str()) {
                route.mute = true;
            }
        }
    }

    /// The overlay after `transition`'s writes join it, with `S` unchanged; the caller sets the
    /// new `S` with [`Self::raise_not_before`] once it has read `p`. Built before the commit, so
    /// the commit only moves it.
    #[must_use]
    pub fn joined(&self, transition: &StripTransition) -> Self {
        Self {
            strips: merged(&self.strips, &transition.strips),
            routes: merged(&self.routes, &transition.routes),
            sources: merged(&self.sources, &transition.sources),
            not_before: self.not_before,
        }
    }

    /// Raise `S` to `not_before` if it is later (#1325 D6: the overlay's `S` is the maximum of its
    /// own and the one the new writes give).
    pub fn raise_not_before(&mut self, not_before: u64) {
        self.not_before = self.not_before.max(not_before);
    }
}

fn merged(left: &[Box<str>], right: &[Box<str>]) -> Vec<Box<str>> {
    let mut out: Vec<Box<str>> = left.iter().chain(right).cloned().collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// The fader/mute records phase 1 writes to one strip's lane of the running plan.
#[derive(Clone, Debug, PartialEq)]
pub struct DuckStripRecords {
    /// The strip's ID.
    pub strip_id: Box<str>,
    /// The records, in push order (`LiveStripRecords::fader`); `None` after the last.
    pub fader: [Option<TrackFaderRecord>; 4],
    /// The offsets-cell records of a lane the running plan renders by events
    /// (`LiveStripRecords::offsets`).
    pub offsets: [Option<FaderOffsetsRecord>; 2],
}

impl DuckStripRecords {
    /// The fader/mute records, in push order.
    pub fn fader_records(&self) -> impl Iterator<Item = TrackFaderRecord> + '_ {
        self.fader.iter().map_while(|record| *record)
    }
}

/// The record phase 1 writes to one route's lane of the running plan.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DuckRouteRecord<'a> {
    /// The route's ID.
    pub route_id: &'a str,
    /// The send's new target and gate, as a value-only transaction that mutes the route would
    /// write it.
    pub record: SendRecord,
}

/// What phase 1 of a structural transaction writes and how long the successor waits (#1325 D7).
///
/// The strips and routes are those it ducks now, never one already in the running plan's
/// [`DuckOverlay`]. It holds no arm set: arming is a preparation step (#1288).
#[derive(Clone, Debug, PartialEq)]
pub struct StripTransition {
    strips: Vec<Box<str>>,
    routes: Vec<Box<str>>,
    route_records: Vec<SendRecord>,
    sources: Vec<Box<str>>,
    strip_records: Vec<DuckStripRecords>,
    ramp: u32,
    compensation: u64,
}

impl StripTransition {
    /// Whether the transaction ducks a strip now: then the successor waits for `S`.
    #[must_use]
    pub fn ducks(&self) -> bool {
        !self.strips.is_empty()
    }

    /// The strips the transaction ducks now (D1), sorted.
    #[must_use]
    pub fn strips(&self) -> &[Box<str>] {
        &self.strips
    }

    /// The routes whose lane phase 1 writes (D2), sorted.
    #[must_use]
    pub fn routes(&self) -> &[Box<str>] {
        &self.routes
    }

    /// The sources the transaction removes that still feed the running plan (D4), sorted.
    #[must_use]
    pub fn sources(&self) -> &[Box<str>] {
        &self.sources
    }

    /// The strip records phase 1 writes, in normalized strip order.
    #[must_use]
    pub fn strip_records(&self) -> &[DuckStripRecords] {
        &self.strip_records
    }

    /// The route records phase 1 writes, in canonical route-ID order.
    pub fn route_records(&self) -> impl Iterator<Item = DuckRouteRecord<'_>> + '_ {
        self.routes
            .iter()
            .zip(&self.route_records)
            .map(|(route_id, &record)| DuckRouteRecord { route_id, record })
    }

    /// `N`, the ramp of every mute phase 1 writes: the transaction model's `mute_ms`.
    #[must_use]
    pub const fn ramp(&self) -> u32 {
        self.ramp
    }

    /// `C`, the longest compensation delay, in the running plan, of a route out of a strip the
    /// transaction ducks now (0 if none).
    #[must_use]
    pub const fn compensation(&self) -> u64 {
        self.compensation
    }

    /// `S = ceil_q(p + q + N + C)` (D3): `p` is the render clock read after the phase-1 writes
    /// and `quantum` is `q`. Render drains the writes no later than the block that starts at
    /// `p + q`, every ramp ends by `p + q + N`, and every route line out of a ducked strip has
    /// played its last nonzero frame by `p + q + N + C`. Saturates at `u64::MAX`, a block start
    /// render never reaches.
    #[must_use]
    pub fn not_before(&self, p: u64, quantum: u32) -> u64 {
        let quantum = u64::from(quantum.max(1));
        let end = p
            .saturating_add(quantum)
            .saturating_add(u64::from(self.ramp))
            .saturating_add(self.compensation);
        end.div_ceil(quantum).saturating_mul(quantum)
    }
}

/// Why [`plan_strip_transition`] refused: an invariant fault, since a strip mute and a route mute
/// are live rows on every plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StripTransitionError {
    /// The classifier did not answer the mutes as live.
    Rebuild(LiveRebuild),
    /// The mutes produced a record other than a strip fader/mute record or a route record.
    Records,
}

fn route_source_id(route: &Route) -> &StableId {
    match &route.source {
        RouteSource::Track { track_id, .. } => track_id,
        RouteSource::Submix { submix_id, .. } => submix_id,
    }
}

fn route_tap_precedes_fader(route: &Route) -> bool {
    match &route.source {
        RouteSource::Track { tap, .. } | RouteSource::Submix { tap, .. } => tap.precedes_fader(),
    }
}

fn sorted_ids<'a>(ids: impl Iterator<Item = &'a str>) -> Vec<Box<str>> {
    let mut out: Vec<Box<str>> = ids.map(Box::from).collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// What phase 1 of a structural transaction to `next` writes on the plan `base` describes (#1325
/// D1, D2, D7).
///
/// `base.committed` is the running plan's committed model and `base.monitor.committed` the
/// monitor overlay its cells hold; `overlay` is the running plan's [`DuckOverlay`], empty outside
/// phase 1. The strips to duck are every strip of `base.committed` that `next` does not hold and
/// `overlay` does not list. The records are the classifier's for the running plan's base with
/// `overlay` laid over it, against the same with each strip to duck muted on both channels and
/// each of its routes whose tap precedes the fader muted, all at `next`'s mute ramp: exactly the
/// records a value-only transaction making those mutes would write, so a strip or route
/// already in the overlay gets no second write. `C` reads the running plan's compensation lines
/// (`base.inventory`), keyed by the route's two edges, which add.
///
/// Control thread only: it clones the base model twice and allocates the lists.
///
/// # Errors
///
/// [`StripTransitionError`], an invariant fault, with nothing written.
pub fn plan_strip_transition(
    base: &SuccessorBase<'_>,
    overlay: &DuckOverlay,
    next: &SessionModel,
) -> Result<StripTransition, StripTransitionError> {
    let committed = base.committed;
    let ramps = LiveRamps::for_session(next);
    let ramp = ramps.mute_samples;
    let in_next = |id: &str| next.strips().any(|strip| strip.id.as_str() == id);
    let strips = sorted_ids(
        committed
            .strips()
            .map(|strip| strip.id.as_str())
            .filter(|id| !in_next(id) && !overlay.has_strip(id)),
    );
    let sources = sorted_ids(
        committed
            .sources
            .iter()
            .map(|source| source.id.as_str())
            .filter(|id| {
                !next.sources.iter().any(|kept| kept.id.as_str() == *id)
                    && !overlay.has_source(id.as_bytes())
            }),
    );
    let mut transition = StripTransition {
        strips,
        routes: Vec::new(),
        route_records: Vec::new(),
        sources,
        strip_records: Vec::new(),
        ramp,
        compensation: 0,
    };
    if transition.strips.is_empty() {
        return Ok(transition);
    }
    let ducked = |id: &StableId| {
        transition
            .strips
            .binary_search_by(|held| (**held).cmp(id.as_str()))
            .is_ok()
    };

    // C: the longest line of a route out of a ducked strip. A route's lines sit on its two edges.
    let lines = base.inventory.delay_lines();
    transition.compensation = committed
        .routes
        .iter()
        .filter(|route| ducked(route_source_id(route)))
        .map(|route| {
            lines
                .iter()
                .filter(|line| match &line.edge {
                    graph::GraphEdgeId::RouteSource { route_id }
                    | graph::GraphEdgeId::RouteDestination { route_id } => {
                        route_id.as_str() == route.id.as_str()
                    }
                    _ => false,
                })
                .map(|line| line.samples)
                .fold(0_u64, u64::saturating_add)
        })
        .max()
        .unwrap_or(0);

    // The records: the base with the overlay, against the same with the new mutes.
    let mut current = committed.clone();
    overlay.apply_to(&mut current);
    let mut target = current.clone();
    for track in &mut target.tracks {
        if ducked(&track.id) {
            track.fader.left_mute = true;
            track.fader.right_mute = true;
        }
    }
    for submix in &mut target.submixes {
        if ducked(&submix.id) {
            submix.fader.left_mute = true;
            submix.fader.right_mute = true;
        }
    }
    for route in &mut target.routes {
        if ducked(route_source_id(route)) && route_tap_precedes_fader(route) {
            route.mute = true;
        }
    }
    let cells = base.monitor.committed;
    let delta = classify_live_delta_with_monitor(
        &current,
        &target,
        ramps,
        &EditRamps::new(),
        MonitorBase {
            committed: cells,
            next: cells,
            ramp: None,
        },
    )
    .map_err(StripTransitionError::Rebuild)?;
    if !delta.effects.is_empty() || delta.jump_lengths.is_some() {
        return Err(StripTransitionError::Records);
    }
    for strip in &delta.strips {
        if strip.matrix.is_some() || !strip.input.is_empty() {
            return Err(StripTransitionError::Records);
        }
        transition.strip_records.push(DuckStripRecords {
            strip_id: Box::from(strip.strip_id),
            fader: strip.fader,
            offsets: strip.offsets,
        });
    }
    for route in &delta.routes {
        transition.routes.push(Box::from(route.route_id));
        transition.route_records.push(route.record);
    }
    Ok(transition)
}
