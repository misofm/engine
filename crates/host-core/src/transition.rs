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
use session::{EditRamps, Route, RouteSource, SessionModel, StableId};

use crate::duck::DuckOverlay;
use crate::live_delta::classify_live_delta_with_monitor;
use crate::prepare::{LiveRamps, LiveRebuild, MonitorBase, SendRecord, SuccessorBase};

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
/// duck-swap, #1324 D4), and that `overlay` does not list; the sources `overlay` lists that `next` keeps are restored
/// ([`StripTransition::restored_sources`]). The records are the classifier's for the running plan's
/// base with `overlay` laid over it, against the same with each strip to duck muted on both
/// channels and each of its routes whose tap precedes the fader muted, all at `next`'s mute ramp:
/// exactly the records a value-only transaction making those mutes would write, so a strip or route
/// already in the overlay gets no second write. `C` reads the running plan's compensation lines
/// (`base.inventory`), keyed by the route's two edges, which add, over every route out of a
/// ducked strip: a restarted strip's route lines must empty before `S` as a removed strip's do.
/// A sidechain line from a ducked strip's tap is not counted, and a carried effect keyed from a
/// restarted strip's tap before the fader still steps at `S`: both are #1324b's.
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
    // strips of both plans only.
    let restarts = |id: &str| restarted.binary_search_by(|held| (**held).cmp(id)).is_ok();
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
}
