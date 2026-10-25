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
//! A structural transaction whose successor restarts a strip it keeps (an insert added, removed
//! or reordered, a quality, `delay_samples` or another prepared value; the successor's
//! `restarted_strips()`) ducks that strip the same way and adopts the successor at `S`; the
//! successor arms the strip's fade-in (issue #1324, a duck-swap).
//!
//! The running plan keeps a [`DuckOverlay`] while the successor waits: the strips and routes
//! phase 1 wrote, the removed sources that still feed that plan, and `S`. A later transaction in
//! the same phase 1 writes no second ramp for anything in it and inherits its `S`.

use builtins_compiler::{FaderOffsetsRecord, TrackFaderRecord};
use session::{
    EditRamps, Route, RouteDestination, RouteSource, SendTap, SessionModel, SidechainDeclaration,
    StableId,
};

use crate::duck::DuckOverlay;
use crate::live_delta::classify_live_delta_with_monitor;
use crate::prepare::{
    LiveRamps, LiveRebuild, MonitorBase, SendRecord, SuccessorBase, sidechain_line_samples,
    source_strip, submix_input_is_quiet,
};

impl DuckOverlay {
    /// The overlay after `transition`'s writes join it, with `S` unchanged; the caller sets the
    /// new `S` with [`Self::raise_not_before`] once it has read `p`. Built before the commit, so
    /// the commit only moves it.
    ///
    /// A source the transaction restores leaves the overlay (#1325 D6): its producer moves into
    /// the successor's set with the commit, so a source call for it goes to the newest committed
    /// session again. A restored strip or route stays: the running plan's cells still hold its
    /// duck, so a later transaction of the same phase 1 writes it no second ramp, and the
    /// successor's base keeps composing it.
    #[must_use]
    pub fn joined(&self, transition: &StripTransition) -> Self {
        let mut sources = merged(&self.sources, &transition.sources);
        sources.retain(|id| {
            transition
                .restored_sources
                .binary_search_by(|held| (**held).cmp(id))
                .is_err()
        });
        sources.shrink_to_fit();
        Self {
            strips: merged(&self.strips, &transition.strips),
            routes: merged(&self.routes, &transition.routes),
            sources,
            not_before: self.not_before,
        }
    }
}

fn merged(left: &[Box<str>], right: &[Box<str>]) -> Vec<Box<str>> {
    let mut out: Vec<Box<str>> = left.iter().chain(right).cloned().collect();
    out.sort_unstable();
    out.dedup();
    // At its length, so the overlay holds no more than `DuckOverlay::retained_ceiling` charges.
    out.shrink_to_fit();
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
    restored_sources: Vec<Box<str>>,
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

    /// The sources of the running plan's [`DuckOverlay`] that the transaction keeps (#1325 D6):
    /// a restore. Each leaves the overlay ([`DuckOverlay::joined`]); one declared as the running
    /// plan declares it carries that plan's ring and producer. Sorted.
    #[must_use]
    pub fn restored_sources(&self) -> &[Box<str>] {
        &self.restored_sources
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
    out.shrink_to_fit();
    out
}

/// What phase 1 of a structural transaction to `next` writes on the plan `base` describes (#1325
/// D1, D2, D7).
///
/// `base.committed` is the running plan's committed model and `base.monitor.committed` the monitor
/// overlay its cells hold; `overlay` is the running plan's [`DuckOverlay`], empty outside phase 1.
/// The strips to duck are every strip of `base.committed` that `next` does not hold or that the
/// prepared successor restarts (`restarted`, its [`crate::PreparedHost::restarted_strips`]: a
/// duck-swap, #1324 D4; none at a declared discontinuity, `base.discontinuity`), and that
/// `overlay` does not list; the sources `overlay` lists that `next` keeps are restored
/// ([`StripTransition::restored_sources`]). The records are the classifier's for the running plan's
/// base with `overlay` laid over it, against the same with each strip to duck muted on both
/// channels and each of its routes whose tap precedes the fader muted, all at `next`'s mute ramp:
/// exactly the records a value-only transaction making those mutes would write, so a strip or route
/// already in the overlay gets no second write. `C` reads the running plan's compensation lines
/// (`base.inventory`), keyed by the route's two edges, which add, over every route out of a
/// ducked strip: a restarted strip's route lines must empty before `S` as a removed strip's do.
/// `C` also counts, into each carried strip's effects, every sidechain line from a ducked strip's
/// `post_fader` or `post_pan` tap, and a ducked submix's `input`-tap sidechain line plus the
/// longest route line into the submix when D1's exemption keeps the consumer carried (#1324 D4):
/// a line the successor shortens then drops only exact `+0.0` at `S`. A carried effect keyed from
/// a restarted strip's other taps is no case: its strip is restarted too (D1's closure).
/// A warm successor whose transaction ducks strips is published `Primed` no earlier than this
/// `S` (#1397 D1), so the same holds for it.
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
    restarted: &[Box<str>],
) -> Result<StripTransition, StripTransitionError> {
    let committed = base.committed;
    let ramps = LiveRamps::for_session(next);
    let ramp = ramps.mute_samples;
    let in_next = |id: &str| next.strips().any(|strip| strip.id.as_str() == id);
    // #1324 D4: a strip the successor restarts is ducked like a removed one. `restarted` names
    // strips of both plans only. A successor at a declared discontinuity restarts every owner by
    // design and need not continue the running plan's output (#1323 D2): its restarts duck
    // nothing.
    let restarts = |id: &str| {
        !base.discontinuity && restarted.binary_search_by(|held| (**held).cmp(id)).is_ok()
    };
    let strips = sorted_ids(
        committed
            .strips()
            .map(|strip| strip.id.as_str())
            .filter(|id| (!in_next(id) || restarts(id)) && !overlay.has_strip(id)),
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
    let restored_sources = sorted_ids(
        overlay
            .sources()
            .iter()
            .map(|id| &**id)
            .filter(|id| next.sources.iter().any(|kept| kept.id.as_str() == *id)),
    );
    let mut transition = StripTransition {
        strips,
        routes: Vec::new(),
        route_records: Vec::new(),
        sources,
        restored_sources,
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
    let route_line = |route: &Route| {
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
    };
    transition.compensation = committed
        .routes
        .iter()
        .filter(|route| ducked(route_source_id(route)))
        .map(route_line)
        .max()
        .unwrap_or(0);
    // #1324 D4: and the longest sidechain path into a carried node that the successor can
    // shorten, so it holds only exact `+0.0` at `S`. A strip is carried when `next` keeps it and
    // neither the successor nor the overlay restarts it. Each sidechain line from a ducked strip's
    // `post_fader` or `post_pan` tap counts; so does, for a ducked submix whose `input` tap keys a
    // carried node while every line into its `Input` stage comes from a ducked or an added strip
    // (D1's exemption, [`submix_input_is_quiet`]), that sidechain line plus the longest route line
    // into the stage. Any other keyed tap of a ducked strip has restarted its consumer (D1).
    let not_carried = |id: &str| {
        !in_next(id)
            || restarted.binary_search_by(|held| (**held).cmp(id)).is_ok()
            || overlay.has_strip(id)
    };
    for strip in committed.strips() {
        let consumer = strip.id.as_str();
        if not_carried(consumer) {
            continue;
        }
        for effect in &strip.inserts.effects {
            let SidechainDeclaration::Routed(sidechain) = &effect.sidechain else {
                continue;
            };
            let (source, tap, submix) = source_strip(&sidechain.source);
            let now = |id: &str| {
                transition
                    .strips
                    .binary_search_by(|held| (**held).cmp(id))
                    .is_ok()
            };
            if !now(source) {
                continue;
            }
            let line = sidechain_line_samples(
                lines,
                consumer,
                effect.id.as_str(),
                sidechain.port_id.as_str(),
            );
            let length = match tap {
                SendTap::PostFader | SendTap::PostPan => line,
                SendTap::Input
                    if submix && submix_input_is_quiet(committed, next, source, &not_carried) =>
                {
                    let into = committed
                        .routes
                        .iter()
                        .filter(|route| {
                            matches!(&route.destination, RouteDestination::SubmixInput { submix_id }
                                if submix_id.as_str() == source)
                        })
                        .map(route_line)
                        .max()
                        .unwrap_or(0);
                    line.saturating_add(into)
                }
                _ => 0,
            };
            transition.compensation = transition.compensation.max(length);
        }
    }

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

/// The transition fallback's duck set (#1397 D2 step 4): every strip of both `committed` (the
/// predecessor's model, whose plan's inventory is `predecessor`) and `next` whose content timing
/// moves in `ordinary`, the inventory of `next` compiled as an ordinary successor (#1285 floors, no
/// lead), sorted by ID. Called on that compile, before any carry join reads it (round-9 M1).
///
/// A strip is in the set when a node of it that both plans hold arrives later in `ordinary` than
/// in `predecessor` (each plan's [`crate::PlanStateInventory::input_floors`], the arrivals it
/// renders), or when an edge of it that both plans hold (a route out of it, or an edge of its own
/// chain, `TrackMain`) has a compensation line whose length changes: a line that changes length
/// restarts with a gap. A node or an edge only one plan holds moves no timing of the other: an added send
/// or a removed route does not duck its source. When the output's arrival grows, every strip of
/// both plans that reaches the output in `next` is in the set. An added strip never is: it is
/// #1288's to arm.
///
/// **Limit (#1397a):** a sidechain line is not read, and #1324 D1's sidechain closure is not run
/// over the set: a carried effect keyed from a tap before the fader of a strip in the set is not
/// added, and a sidechain line that changes length does not add its strip. Both are #1397b's.
///
/// Control thread only; it allocates the set.
#[must_use]
pub fn grown_strips(
    predecessor: &crate::PlanStateInventory,
    committed: &SessionModel,
    next: &SessionModel,
    ordinary: &crate::PlanStateInventory,
) -> Vec<String> {
    use graph::{GraphEdgeId, GraphNodeId};
    use std::collections::{BTreeMap, BTreeSet};
    let routes: BTreeMap<&str, &Route> = committed
        .routes
        .iter()
        .chain(&next.routes)
        .map(|route| (route.id.as_str(), route))
        .collect();
    let owner = |node: &GraphNodeId| -> Option<String> {
        match node {
            GraphNodeId::Submix { submix_id } => Some(submix_id.as_str().to_owned()),
            _ => crate::prepare::strip_owner(&routes, node).map(|(strip, _)| strip.to_owned()),
        }
    };
    let in_both = |id: &str| {
        crate::prepare::committed_has_strip(committed, id)
            && crate::prepare::committed_has_strip(next, id)
    };
    let held = crate::prepare::session_graph_nodes(committed);
    let arrival = |floors: &[(GraphNodeId, u64)], node: &GraphNodeId| {
        floors
            .binary_search_by(|(other, _)| other.cmp(node))
            .map_or(0, |index| floors[index].1)
    };
    let mut grown: BTreeSet<String> = BTreeSet::new();
    let mut output_grew = false;
    for (node, samples) in ordinary.input_floors() {
        let output = matches!(node, GraphNodeId::Output { .. });
        if (output || held.contains(node)) && *samples > arrival(predecessor.input_floors(), node) {
            if output {
                output_grew = true;
            } else if let Some(strip) = owner(node) {
                grown.insert(strip);
            }
        }
    }
    // An edge both plans hold: a route of both models with the same ends, or an edge of a
    // strip's own chain (`TrackMain`), whose source must be a node the predecessor holds (below)
    // and whose strip must be in both plans. A `TrackMain` line into a node only the successor
    // holds (an added insert) can only add that same strip, the one the edit changed, which its
    // own restart already ducks: harmless.
    let edge_in_both = |edge: &GraphEdgeId| match edge {
        GraphEdgeId::RouteSource { route_id } | GraphEdgeId::RouteDestination { route_id } => {
            let find = |model: &'_ SessionModel| {
                model
                    .routes
                    .iter()
                    .find(|route| route.id.as_str() == route_id.as_str())
                    .map(|route| (route.source.clone(), route.destination.clone()))
            };
            find(committed).is_some_and(|before| find(next) == Some(before))
        }
        GraphEdgeId::TrackMain { .. } => true,
        GraphEdgeId::EffectSidechain { .. } => false,
    };
    let lines =
        |inventory: &crate::PlanStateInventory| -> BTreeMap<GraphEdgeId, (u64, GraphNodeId)> {
            inventory
                .delay_lines()
                .iter()
                .filter(|line| edge_in_both(&line.edge))
                .map(|line| (line.edge.clone(), (line.samples, line.source.clone())))
                .collect()
        };
    let (before, after) = (lines(predecessor), lines(ordinary));
    for (edge, (_, source)) in before.iter().chain(&after) {
        if before.get(edge).map(|line| line.0) != after.get(edge).map(|line| line.0)
            && held.contains(source)
            && let Some(strip) = owner(source)
        {
            grown.insert(strip);
        }
    }
    if output_grew {
        grown.extend(reaching_output(next));
    }
    grown.into_iter().filter(|id| in_both(id)).collect()
}

/// The strips of `model` that reach its output through their routes, directly or through
/// submixes.
fn reaching_output(model: &SessionModel) -> Vec<String> {
    use std::collections::BTreeSet;
    let mut reaching: BTreeSet<&str> = BTreeSet::new();
    loop {
        let before = reaching.len();
        for route in &model.routes {
            let source = match &route.source {
                RouteSource::Track { track_id, .. } => track_id.as_str(),
                RouteSource::Submix { submix_id, .. } => submix_id.as_str(),
            };
            let reaches = match &route.destination {
                session::RouteDestination::OutputInput { .. } => true,
                session::RouteDestination::SubmixInput { submix_id } => {
                    reaching.contains(submix_id.as_str())
                }
            };
            if reaches {
                reaching.insert(source);
            }
        }
        if reaching.len() == before {
            return reaching.into_iter().map(str::to_owned).collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DuckOverlay, StripTransition, plan_strip_transition};
    use crate::prepare::{HostPrepareCaps, HostShapePolicy, SuccessorBase, compile_host_model};
    use builtins_compiler::TrackFaderRecord;
    use session::{RouteDestination, RouteSource, SendTap, SessionModel, StableId};

    const FIXTURE: &str =
        include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

    fn id(text: &str) -> StableId {
        StableId::parse(text).expect("stable ID")
    }

    fn caps() -> HostPrepareCaps {
        HostPrepareCaps {
            shape: HostShapePolicy::AnyLaunchRate,
            source_ring_frames: 4_096,
            maximum_automation_spans_per_block: 128,
            maximum_tracks: 100,
            maximum_submixes: 100,
            maximum_vcas: 100,
            maximum_vca_reach_pairs: u64::MAX,
            maximum_sources: 100,
            maximum_routes: 100,
            maximum_effects: 100,
            maximum_graph_session_plus_plan_bytes: 1_000_000_000,
            maximum_source_total_bytes: 100_000_000,
            maximum_source_overhead_bytes: 100_000_000,
            maximum_effect_state_bytes: 1_000_000_000,
            maximum_effect_scratch_bytes: 1_000_000_000,
            maximum_builtin_retained_bytes: 1_000_000_000,
            maximum_named_allocation_bytes: 1_000_000_000,
            maximum_meter_streams: 64,
            maximum_meter_items: 1 << 16,
            maximum_meter_bytes: 1 << 24,
        }
    }

    fn limiter(name: &str) -> session::Effect {
        session::Effect {
            id: id(name),
            identity: session::EffectIdentity::Native {
                effect_id: id("miso.true-peak-limiter"),
            },
            quality: session::EffectQuality::Normal,
            bypass: false,
            link_mode: session::LinkMode::Maximum,
            params: Vec::new(),
            sidechain: session::SidechainDeclaration::None,
        }
    }

    fn compile(model: &SessionModel) -> session::CompiledSession {
        compile_host_model(
            model,
            caps().compile_caps(model.sources.len()).expect("caps"),
        )
        .expect("compiles")
    }

    /// Tracks `a` (a true-peak limiter insert), `b` and `c` into the output; `b` and `c` each
    /// have a `pre_fader` send into submix `r`; each track reads its own source.
    fn model() -> SessionModel {
        let fixture = session::parse_session_json(FIXTURE).expect("fixture");
        let mut model = fixture.clone();
        model.console.pre_insert.clear();
        model.console.post_insert.clear();
        model.vcas.clear();
        model.automation.clear();
        model.sources.clear();
        model.tracks.clear();
        model.routes.clear();
        let out = fixture.routes[0].destination.clone();
        for name in ["a", "b", "c"] {
            let mut source = fixture.sources[0].clone();
            source.id = id(&format!("src-{name}"));
            model.sources.push(source);
            let mut track = fixture.tracks[0].clone();
            track.id = id(name);
            track.source_id = id(&format!("src-{name}"));
            track.console.clear();
            track.inserts.effects.clear();
            model.tracks.push(track);
            let mut main = fixture.routes[0].clone();
            main.id = id(&format!("{name}-main"));
            main.source = RouteSource::Track {
                track_id: id(name),
                tap: SendTap::PostPan,
            };
            main.destination = out.clone();
            model.routes.push(main.clone());
            if name != "a" {
                let mut send = main;
                send.id = id(&format!("{name}-send"));
                send.source = RouteSource::Track {
                    track_id: id(name),
                    tap: SendTap::PreFader,
                };
                send.destination = RouteDestination::SubmixInput { submix_id: id("r") };
                model.routes.push(send);
            }
        }
        model.tracks[0].inserts.effects.push(limiter("lim"));
        model.submixes = vec![session::Submix::unity(id("r"), &model.console)];
        let mut r_main = fixture.routes[0].clone();
        r_main.id = id("r-main");
        r_main.source = RouteSource::Submix {
            submix_id: id("r"),
            tap: SendTap::PostPan,
        };
        r_main.destination = out;
        model.routes.push(r_main);
        model
    }

    /// #1325 D1, D2, D3: the duck set of a removal of `b` and `c` while `c` is already in the
    /// running plan's overlay is `b` alone, with its two mutes at `N` and its `pre_fader` send's
    /// record and no other, its source listed, and `C` the compensation delay the limiter on `a`
    /// gives `b`'s route into the output: the plan's output latency.
    #[test]
    fn a_transition_ducks_the_removed_strips_outside_the_overlay() {
        let committed = model();
        let compiled = compile_host_model(
            &committed,
            caps().compile_caps(committed.sources.len()).expect("caps"),
        )
        .expect("compiles");
        let prepared = crate::prepare::prepare_host_runtime(&compiled, &caps()).expect("prepares");
        let latency = prepared.report.latency_samples;
        assert!(latency > 0, "the limiter delays b's route into the output");
        let committed = compiled.normalized_model();

        // `c` was ramped out by an earlier transaction of this phase 1.
        let mut overlay = DuckOverlay::new();
        let earlier = {
            let mut next = committed.clone();
            next.tracks.retain(|track| track.id.as_str() != "c");
            next.routes
                .retain(|route| !route.id.as_str().starts_with("c-"));
            next.sources.retain(|source| source.id.as_str() != "src-c");
            plan_strip_transition(
                &SuccessorBase::new(&prepared.inventory, committed, 2),
                &overlay,
                &next,
                &[],
            )
            .expect("c's transition")
        };
        overlay = overlay.joined(&earlier);
        overlay.raise_not_before(4_096);

        let mut next = committed.clone();
        next.tracks.retain(|track| track.id.as_str() == "a");
        next.routes
            .retain(|route| matches!(route.id.as_str(), "a-main" | "r-main"));
        next.sources.retain(|source| source.id.as_str() == "src-a");
        let transition = plan_strip_transition(
            &SuccessorBase::new(&prepared.inventory, committed, 3),
            &overlay,
            &next,
            &[],
        )
        .expect("b's transition");

        let n = crate::prepare::LiveRamps::for_session(&next).mute_samples;
        assert_eq!(transition.ramp(), n);
        assert_eq!(transition.strips(), [Box::<str>::from("b")]);
        assert_eq!(transition.sources(), [Box::<str>::from("src-b")]);
        assert_eq!(transition.routes(), [Box::<str>::from("b-send")]);
        assert_eq!(transition.compensation(), latency);
        let [strip] = transition.strip_records() else {
            panic!("one strip record")
        };
        assert_eq!(&*strip.strip_id, "b");
        let mutes: Vec<_> = strip.fader_records().collect();
        assert_eq!(
            mutes,
            [TrackFaderRecord::Mute {
                lanes: builtins::BuiltinLaneSelector::Both,
                muted: true,
                smoothing_samples: n,
            }],
            "both of b's channels mute over N"
        );
        let joined = overlay.joined(&transition);
        // The overlay stays inside the row the control plane charges for it at the plan's model.
        let ids = |ids: &mut dyn Iterator<Item = &StableId>| {
            ids.fold((0, 0), |(count, bytes), id| {
                (count + 1, bytes + id.as_str().len())
            })
        };
        let (strips, strip_bytes) = ids(&mut committed.strips().map(|strip| strip.id));
        let (routes, route_bytes) = ids(&mut committed.routes.iter().map(|route| &route.id));
        let (sources, source_bytes) = ids(&mut committed.sources.iter().map(|source| &source.id));
        let ceiling = DuckOverlay::retained_ceiling(
            [strips, routes, sources],
            [strip_bytes, route_bytes, source_bytes],
        )
        .expect("the ceiling");
        assert!(
            joined.retained_bytes() <= ceiling,
            "the overlay is within its charge"
        );
        // At the ceiling: an overlay that names every strip, route and source of the plan, built
        // by `joined` as a rebuild builds it, retains exactly the charge.
        let names = |ids: &mut dyn Iterator<Item = &StableId>| -> Vec<Box<str>> {
            ids.map(|id| Box::from(id.as_str())).collect()
        };
        let full = StripTransition {
            strips: names(&mut committed.strips().map(|strip| strip.id)),
            routes: names(&mut committed.routes.iter().map(|route| &route.id)),
            route_records: Vec::new(),
            sources: names(&mut committed.sources.iter().map(|source| &source.id)),
            restored_sources: Vec::new(),
            strip_records: Vec::new(),
            ramp: n,
            compensation: 0,
        };
        assert_eq!(
            DuckOverlay::new().joined(&full).retained_bytes(),
            ceiling,
            "a full overlay retains exactly its charge"
        );
        assert_eq!(
            joined.not_before(),
            Some(4_096),
            "S is kept until the caller raises it"
        );
        assert!(joined.has_strip("b") && joined.has_strip("c"));

        // `C` adds a route's two lines. `a` carries two limiters and `b` one, so the output
        // arrives at `2L` and `b-main`'s route node at `L`. Re-pointing `b-main` to `b`'s input
        // tap (arrival 0) in a successor keeps the route node floored at `L` (#1285): its
        // source edge gets a line of `L` and its destination edge one of `L`, `2L` in all.
        let mut two = model();
        two.tracks[0].inserts.effects.push(limiter("lim-2"));
        two.tracks[1].inserts.effects.push(limiter("lim-b"));
        let first = compile(&two);
        let p0 = crate::prepare::prepare_host_runtime(&first, &caps()).expect("prepares");
        let mut repointed = first.normalized_model().clone();
        for route in &mut repointed.routes {
            if route.id.as_str() == "b-main" {
                route.source = RouteSource::Track {
                    track_id: id("b"),
                    tap: SendTap::Input,
                };
            }
        }
        let second = compile(&repointed);
        let p1 = crate::prepare::prepare_host_runtime_successor(
            &second,
            &caps(),
            SuccessorBase::new(&p0.inventory, first.normalized_model(), 2),
        )
        .expect("the successor prepares");
        let b_lines = p1
            .inventory
            .delay_lines()
            .iter()
            .filter(|line| match &line.edge {
                graph::GraphEdgeId::RouteSource { route_id }
                | graph::GraphEdgeId::RouteDestination { route_id } => {
                    route_id.as_str() == "b-main"
                }
                _ => false,
            })
            .count();
        assert_eq!(b_lines, 2, "b-main has a line on each of its edges");
        let committed = second.normalized_model();
        let mut next = committed.clone();
        next.tracks.retain(|track| track.id.as_str() != "b");
        next.routes
            .retain(|route| !route.id.as_str().starts_with("b-"));
        next.sources.retain(|source| source.id.as_str() != "src-b");
        let transition = plan_strip_transition(
            &SuccessorBase::new(&p1.inventory, committed, 3),
            &DuckOverlay::new(),
            &next,
            &[],
        )
        .expect("b's transition");
        assert_eq!(
            transition.compensation(),
            p1.report.latency_samples,
            "C is the sum of b-main's two lines"
        );
    }

    /// `grown_strips` over `first -> second`: the predecessor prepared fresh, `second` prepared
    /// as its ordinary successor.
    fn grown(first: &SessionModel, second: &SessionModel) -> Vec<String> {
        let (first, second) = (compile(first), compile(second));
        let p0 = crate::prepare::prepare_host_runtime(&first, &caps()).expect("prepares");
        let p1 = crate::prepare::prepare_host_runtime_successor(
            &second,
            &caps(),
            SuccessorBase::new(&p0.inventory, first.normalized_model(), 2),
        )
        .expect("the successor prepares");
        super::grown_strips(
            &p0.inventory,
            first.normalized_model(),
            second.normalized_model(),
            &p1.inventory,
        )
    }

    fn compressor(name: &str) -> session::Effect {
        session::Effect {
            identity: session::EffectIdentity::Native {
                effect_id: id("miso.compressor"),
            },
            ..limiter(name)
        }
    }

    /// #1397 D2 step 4, `grown_strips`'s rules, on [`model`] (output latency 486, `a`'s limiter):
    /// - a line alone: a limiter on `c` raises `r`'s input, so `b`'s `pre_fader` send line into
    ///   `r` grows while `b`'s nodes and the output's arrival stay: `[b, c, r]`;
    /// - the output alone: with `b` feeding only `r` and a routeless track `d`, a second limiter
    ///   on `a` raises the output's arrival, which neither `b`'s nodes nor its line show: every
    ///   strip that reaches the output, `[a, b, c, r]`, and not `d`;
    /// - an added send, a removed route, and a node only the successor holds, move nothing: `[]`.
    ///
    /// Test value: without the line rule `b` drops from the first set; without the output rule
    /// `b` drops from the second; an output rule over every strip adds `d`; a line or node read
    /// from one plan alone ducks `b` (the added send, the removed route) or `a` (the node).
    #[test]
    fn grown_strips_reads_lines_nodes_and_the_output() {
        let base = model();
        let with = |edit: &dyn Fn(&mut SessionModel)| {
            let mut next = base.clone();
            edit(&mut next);
            next
        };
        let track = |model: &mut SessionModel, name: &str| -> usize {
            model
                .tracks
                .iter()
                .position(|track| track.id.as_str() == name)
                .expect("a track")
        };
        let limited_c = with(&|next| {
            let c = track(next, "c");
            next.tracks[c].inserts.effects.push(limiter("lim-c"));
        });
        assert_eq!(grown(&base, &limited_c), ["b", "c", "r"], "a line alone");

        let only_send = with(&|next| {
            next.routes.retain(|route| route.id.as_str() != "b-main");
            let mut d = next.tracks[0].clone();
            d.id = id("d");
            d.inserts.effects.clear();
            next.tracks.push(d);
            next.tracks
                .sort_by(|left, right| left.id.as_str().cmp(right.id.as_str()));
        });
        let mut louder = only_send.clone();
        let a = track(&mut louder, "a");
        louder.tracks[a].inserts.effects.push(limiter("lim-2"));
        assert_eq!(
            grown(&only_send, &louder),
            ["a", "b", "c", "r"],
            "the output alone"
        );

        let added_send = with(&|next| {
            let mut send = next
                .routes
                .iter()
                .find(|route| route.id.as_str() == "b-main")
                .expect("b-main")
                .clone();
            send.id = id("b-out");
            send.source = RouteSource::Track {
                track_id: id("b"),
                tap: SendTap::PreFader,
            };
            next.routes.push(send);
            next.routes
                .sort_by(|left, right| left.id.as_str().cmp(right.id.as_str()));
        });
        assert_eq!(
            grown(&base, &added_send),
            Vec::<String>::new(),
            "an added send"
        );
        let removed_route = with(&|next| {
            next.routes.retain(|route| route.id.as_str() != "b-main");
        });
        assert_eq!(
            grown(&base, &removed_route),
            Vec::<String>::new(),
            "a removed route"
        );
        let added_node = with(&|next| {
            let a = track(next, "a");
            next.tracks[a].inserts.effects.push(compressor("comp-a"));
        });
        assert_eq!(
            grown(&base, &added_node),
            Vec::<String>::new(),
            "an added node"
        );
    }

    /// #1324 D4: `C` counts a sidechain line from a ducked strip's `post_fader` tap into a carried
    /// node. `b` has no route and keys `c`'s compressor, which sits behind a limiter, so the key's
    /// line is the limiter's latency `L`; a successor that restarts `b` (here: given an insert)
    /// may shorten it, so `C` is `L`. Red if `C` counts route lines only (0 here: `b` has none).
    #[test]
    fn c_counts_a_post_fader_sidechain_line_into_a_carried_node() {
        let mut committed = model();
        committed
            .routes
            .retain(|route| !route.id.as_str().starts_with("b-"));
        let mut key = limiter("key");
        key.identity = session::EffectIdentity::Native {
            effect_id: id("miso.compressor"),
        };
        key.link_mode = session::LinkMode::DualMono;
        key.sidechain = session::SidechainDeclaration::Routed(session::Sidechain {
            source: RouteSource::Track {
                track_id: id("b"),
                tap: SendTap::PostFader,
            },
            port_id: id("sidechain-in"),
        });
        committed.tracks[2].inserts.effects = vec![limiter("c-lim"), key];
        let compiled = compile(&committed);
        let prepared = crate::prepare::prepare_host_runtime(&compiled, &caps()).expect("prepares");
        let latency = prepared.report.latency_samples;
        assert!(latency > 0);
        let committed = compiled.normalized_model();
        let mut next = committed.clone();
        next.tracks[1].inserts.effects.push(limiter("b-lim"));
        let transition = plan_strip_transition(
            &SuccessorBase::new(&prepared.inventory, committed, 2),
            &DuckOverlay::new(),
            &next,
            &[Box::from("b")],
        )
        .expect("b's duck-swap");
        assert_eq!(transition.strips(), [Box::<str>::from("b")]);
        assert_eq!(transition.compensation(), latency, "C is the key's line");
    }
}
