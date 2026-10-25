//! *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354a): `warm_lead`
//! and the preparation of its lead (D2, D4), at 48 kHz, quantum 128, on this build's backend
//! (the arrival arithmetic is backend-free, scope R5).
//!
//! Every plan is prepared through the live-controls entry, as the control plane prepares it (a
//! per-node effect carries only with a control lane, #1281). Every gate prepares its predecessor
//! A fresh, then the ordinary successor of the edit (whose
//! inventory `warm_lead` reads, #1354a ruling 1), then asks `warm_lead` and, for a warm answer,
//! prepares the warm successor W. `L` is the true-peak limiter's latency at 48 kHz, read from a
//! prepared plan.
//!
//! Gates 5, 6 and the second case of gate 7 (the sidechain closure and the submix `input`-tap
//! rule) are #1354b's.
//!
//! The second half (#1355) streams the warm successor through the plan exchange: render's
//! readiness check, the adoption and its prime, on both bank widths, and the growth chain at
//! every launch rate.

#![cfg(feature = "test-support")]

#[path = "support/successor.rs"]
mod successor;

use engine::realtime::{PlanAdoption, SwapOutcome};
use graph::{GraphNodeId, StableGraphId, TrackStage};
use graph_compiler::Backend;
use host_core::{
    HostLiveControlRequest, HostPrepareCaps, PrepareDiagnostics, PreparedHost, SuccessorBase,
    WarmConfig, WarmDecision, WarmLead, WarmUnavailable, stall_ring_frames, warm_lead,
};
use session::{
    Console, Effect, EffectIdentity, EffectQuality, LinkMode, Route, RouteDestination, RouteSource,
    SendTap, SessionModel, Sidechain, SidechainDeclaration, StableId, Submix, parse_session_json,
};
use successor::{Block, Feed, Session, caps, exchange, render};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const QUANTUM: u64 = 128;
/// `P` of every gate whose growth is one limiter: `ceil(L / 128) * 128`.
const LEAD: u64 = 512;

fn sid(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

/// The default `WarmConfig` of the gates.
const CONFIG: WarmConfig = WarmConfig {
    p_max: 4096,
    prime_bytes_max: u64::MAX,
};

/// The gates' ring: `stall_ring_frames(48_000, 128) + 4096 + 128`, so no gate depends on
/// `default_source_ring_frames`.
fn ring_caps(frames: u32) -> HostPrepareCaps {
    HostPrepareCaps {
        source_ring_frames: frames,
        ..caps()
    }
}

fn gate_caps() -> HostPrepareCaps {
    ring_caps(stall_ring_frames(48_000, 128) + 4096 + 128)
}

/// The fixture with no strip, no route and no console slot: tracks are added by [`track`].
fn empty() -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    assert_eq!(model.sample_rate_hz, 48_000);
    model.console = Console {
        pre_insert: Vec::new(),
        post_insert: Vec::new(),
    };
    model.tracks.truncate(1);
    model.tracks[0].console.clear();
    model.tracks[0].inserts.effects.clear();
    model.routes.truncate(1);
    model
}

/// A track `id` on the fixture's source, with `inserts`, and no route.
fn track(model: &mut SessionModel, id: &str, inserts: Vec<Effect>) {
    let mut track = model.tracks[0].clone();
    track.id = sid(id);
    track.inserts.effects = inserts;
    model.tracks.push(track);
}

/// Strip `id`'s inserts.
fn inserts<'m>(model: &'m mut SessionModel, id: &str) -> &'m mut Vec<Effect> {
    if let Some(track) = model
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == id)
    {
        return &mut track.inserts.effects;
    }
    let submix = model
        .submixes
        .iter_mut()
        .find(|submix| submix.id.as_str() == id)
        .expect("a strip");
    &mut submix.inserts.effects
}

/// [`track`] with its fader muted on both lanes.
fn muted_track(model: &mut SessionModel, id: &str, inserts: Vec<Effect>) {
    track(model, id, inserts);
    let added = model.tracks.last_mut().expect("the added track");
    added.fader.left_mute = true;
    added.fader.right_mute = true;
}

fn submix(model: &mut SessionModel, id: &str, inserts: Vec<Effect>) {
    let mut submix = Submix::unity(sid(id), &model.console);
    submix.inserts.effects = inserts;
    model.submixes.push(submix);
}

fn from_track(id: &str, tap: SendTap) -> RouteSource {
    RouteSource::Track {
        track_id: sid(id),
        tap,
    }
}

fn from_submix(id: &str, tap: SendTap) -> RouteSource {
    RouteSource::Submix {
        submix_id: sid(id),
        tap,
    }
}

/// A unity route `id` from `source`, into submix `into` or, with `None`, the output.
fn route(model: &mut SessionModel, id: &str, source: RouteSource, into: Option<&str>) {
    let mut route: Route = model.routes[0].clone();
    route.id = sid(id);
    route.source = source;
    route.destination = match into {
        Some(submix) => RouteDestination::SubmixInput {
            submix_id: sid(submix),
        },
        None => RouteDestination::OutputInput {
            output_id: sid("main-out"),
        },
    };
    model.routes.push(route);
}

/// Strip `id` routed from its `post_pan` tap to the output.
fn to_output(model: &mut SessionModel, id: &str) {
    let source = if model.submixes.iter().any(|submix| submix.id.as_str() == id) {
        from_submix(id, SendTap::PostPan)
    } else {
        from_track(id, SendTap::PostPan)
    };
    route(model, &format!("{id}-main"), source, None);
}

/// Strip `id` routed from its `post_fader` tap into submix `into`.
fn to_submix(model: &mut SessionModel, id: &str, into: &str) {
    let source = if model.submixes.iter().any(|submix| submix.id.as_str() == id) {
        from_submix(id, SendTap::PostFader)
    } else {
        from_track(id, SendTap::PostFader)
    };
    route(model, &format!("{id}-{into}"), source, Some(into));
}

/// Remove the template track and route [`empty`] kept.
fn finish(mut model: SessionModel) -> SessionModel {
    model.tracks.remove(0);
    model.routes.remove(0);
    model
}

fn effect(id: &str, native: &str, sidechain: SidechainDeclaration) -> Effect {
    Effect {
        id: sid(id),
        identity: EffectIdentity::Native {
            effect_id: sid(native),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: Vec::new(),
        sidechain,
    }
}

fn limiter() -> Effect {
    effect(
        "limiter",
        "miso.true-peak-limiter",
        SidechainDeclaration::None,
    )
}

/// A compressor keyed from `key`.
fn keyed_compressor(key: RouteSource) -> Effect {
    effect(
        "comp",
        "miso.compressor",
        SidechainDeclaration::Routed(Sidechain {
            source: key,
            port_id: sid("sidechain-in"),
        }),
    )
}

/// A predecessor, an edit, and what preparation and `warm_lead` make of them.
struct Edit {
    a: Session,
    b: Session,
    caps: HostPrepareCaps,
    host_a: PreparedHost,
    backend: Backend,
}

impl Edit {
    fn new(a: SessionModel, b: SessionModel) -> Self {
        Self::with_caps(a, b, gate_caps())
    }

    fn with_caps(a: SessionModel, b: SessionModel, caps: HostPrepareCaps) -> Self {
        Self::on(a, b, caps, backend())
    }

    fn on(a: SessionModel, b: SessionModel, caps: HostPrepareCaps, backend: Backend) -> Self {
        let a = Session::compile(finish(a));
        let b = Session::compile(finish(b));
        let host_a = fresh(&a, &caps, backend);
        Self {
            a,
            b,
            caps,
            host_a,
            backend,
        }
    }

    fn base(&self) -> SuccessorBase<'_> {
        SuccessorBase::new(&self.host_a.inventory, &self.a.model, 2)
    }

    fn prepare(&self, base: SuccessorBase<'_>) -> Result<PreparedHost, PrepareDiagnostics> {
        successor_of(&self.b, &self.caps, base, self.backend)
    }

    fn decide(&self, config: &WarmConfig) -> WarmDecision {
        let ordinary = self
            .prepare(self.base())
            .unwrap_or_else(|failure| panic!("ordinary successor: {failure:?}"));
        warm_lead(
            &self.b.compiled,
            &self.caps,
            self.base(),
            &ordinary.inventory,
            config,
        )
        .unwrap_or_else(|failure| panic!("warm_lead: {failure:?}"))
    }

    fn warm(&self, lead: &WarmLead) -> PreparedHost {
        self.prepare(SuccessorBase {
            warm: Some(lead),
            ..self.base()
        })
        .unwrap_or_else(|failure| panic!("warm successor: {failure:?}"))
    }

    /// Every node of `nodes` arrives in `w` at exactly its arrival in A plus `lead`.
    fn assert_leads(&self, w: &PreparedHost, nodes: &[GraphNodeId], lead: u64) {
        for node in nodes {
            assert_eq!(
                arrival(w, node),
                arrival(&self.host_a, node) + lead,
                "{node:?} arrives at a(n) + {lead}"
            );
        }
    }
}

fn backend() -> Backend {
    Backend::current()
}

/// `session` prepared with no predecessor, through the live-controls entry.
fn fresh(session: &Session, caps: &HostPrepareCaps, backend: Backend) -> PreparedHost {
    host_core::test_only_prepare_host_runtime_with_live_controls_on(
        &session.compiled,
        caps,
        &HostLiveControlRequest::default(),
        backend,
    )
    .unwrap_or_else(|failure| panic!("fresh: {failure:?}"))
    .0
}

/// `session` prepared as the successor `base` describes, through the live-controls entry.
fn successor_of(
    session: &Session,
    caps: &HostPrepareCaps,
    base: SuccessorBase<'_>,
    backend: Backend,
) -> Result<PreparedHost, PrepareDiagnostics> {
    host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
        &session.compiled,
        caps,
        &HostLiveControlRequest::default(),
        base,
        backend,
    )
    .map(|(host, _)| host)
}

fn arrival(host: &PreparedHost, node: &GraphNodeId) -> u64 {
    host.inventory
        .input_floors()
        .iter()
        .find(|(row, _)| row == node)
        .map_or(0, |(_, arrival)| *arrival)
}

fn gid(text: &str) -> StableGraphId {
    StableGraphId::parse(text).expect("graph ID")
}

/// Every stage of strip `id` from `from` on.
fn stages(id: &str, from: TrackStage) -> Vec<GraphNodeId> {
    [
        TrackStage::Input,
        TrackStage::PostInputBuiltins,
        TrackStage::PostSimd1,
        TrackStage::PostDynamic,
        TrackStage::PostSimd2PreFader,
        TrackStage::PostFader,
        TrackStage::PostMatrix,
    ]
    .into_iter()
    .filter(|stage| *stage >= from)
    .map(|stage| GraphNodeId::TrackStage {
        track_id: gid(id),
        stage,
    })
    .collect()
}

fn route_node(id: &str) -> GraphNodeId {
    GraphNodeId::Route { route_id: gid(id) }
}

fn output() -> GraphNodeId {
    GraphNodeId::Output {
        output_id: gid("main-out"),
    }
}

fn warm(decision: WarmDecision) -> WarmLead {
    match decision {
        WarmDecision::Warm(lead) => lead,
        other => panic!("expected a warm answer, got {other:?}"),
    }
}

fn names(ids: &[&str]) -> Vec<String> {
    ids.iter().map(|id| (*id).to_owned()).collect()
}

fn restarted(host: &PreparedHost) -> Vec<&str> {
    host.restarted_strips().iter().map(|id| &**id).collect()
}

/// Two audible tracks into the output with no latency.
fn two_tracks() -> SessionModel {
    let mut model = empty();
    for id in ["t1", "t2"] {
        track(&mut model, id, Vec::new());
        to_output(&mut model, id);
    }
    model
}

/// Gate 1 shape: [`two_tracks`], and the edit adds a limiter insert to track 1.
fn limiter_on_t1() -> Edit {
    let mut b = two_tracks();
    *inserts(&mut b, "t1") = vec![limiter()];
    Edit::new(two_tracks(), b)
}

/// Gate 1: the lead is `Δ` over the carried nodes in whole quanta, and the warm successor's
/// carried nodes arrive at exactly `a(n) + P`.
#[test]
fn a_growth_leads_every_carried_node_by_whole_quanta() {
    // A latent insert on a playing track.
    let edit = limiter_on_t1();
    let latency = edit
        .prepare(edit.base())
        .expect("ordinary")
        .report
        .latency_samples;
    assert_eq!(
        latency.div_ceil(QUANTUM) * QUANTUM,
        LEAD,
        "L rounds up to 512"
    );
    let lead = warm(edit.decide(&CONFIG));
    assert_eq!((lead.lead_samples, &*lead.restart_whole), (LEAD, &[][..]));
    let w = edit.warm(&lead);
    assert_eq!(restarted(&w), ["t1"]);
    assert_eq!(w.lead_samples(), LEAD);
    let mut carried = stages("t2", TrackStage::Input);
    carried.extend([
        route_node("t2-main"),
        route_node("t1-main"),
        output(),
        stages("t1", TrackStage::PostMatrix).remove(0),
    ]);
    edit.assert_leads(&w, &carried, LEAD);

    // An added muted limiter track: the same lead, nothing restarted.
    let mut b = two_tracks();
    muted_track(&mut b, "t3", vec![limiter()]);
    to_output(&mut b, "t3");
    let edit = Edit::new(two_tracks(), b);
    let lead = warm(edit.decide(&CONFIG));
    assert_eq!((lead.lead_samples, &*lead.restart_whole), (LEAD, &[][..]));
    let w = edit.warm(&lead);
    assert!(w.restarted_strips().is_empty());
    let mut carried = stages("t1", TrackStage::Input);
    carried.extend(stages("t2", TrackStage::Input));
    carried.push(output());
    edit.assert_leads(&w, &carried, LEAD);

    // An added muted limiter track into a submix whose arrival grows while the output's does
    // not: X's limiter already holds the output at L.
    let parallel = || {
        let mut model = empty();
        track(&mut model, "x", vec![limiter()]);
        to_output(&mut model, "x");
        submix(&mut model, "s", Vec::new());
        to_output(&mut model, "s");
        track(&mut model, "t", Vec::new());
        to_submix(&mut model, "t", "s");
        model
    };
    let mut b = parallel();
    muted_track(&mut b, "m", vec![limiter()]);
    to_submix(&mut b, "m", "s");
    let edit = Edit::new(parallel(), b);
    let lead = warm(edit.decide(&CONFIG));
    assert_eq!((lead.lead_samples, &*lead.restart_whole), (LEAD, &[][..]));
    let w = edit.warm(&lead);
    assert!(w.restarted_strips().is_empty());
    let mut carried = stages("s", TrackStage::Input);
    carried.extend(stages("x", TrackStage::Input));
    carried.extend([output(), route_node("t-s")]);
    edit.assert_leads(&w, &carried, LEAD);

    // An edit that grows nothing.
    let mut b = two_tracks();
    muted_track(&mut b, "t3", Vec::new());
    to_output(&mut b, "t3");
    assert_eq!(
        Edit::new(two_tracks(), b).decide(&CONFIG),
        WarmDecision::Ordinary
    );
}

/// Gate 1, last case: a lead one sample off a multiple of the quantum is refused with the typed
/// error, before anything is prepared.
#[test]
fn a_lead_off_the_quantum_is_refused() {
    let edit = limiter_on_t1();
    let mut lead = warm(edit.decide(&CONFIG));
    lead.lead_samples += 1;
    let refused = edit
        .prepare(SuccessorBase {
            warm: Some(&lead),
            ..edit.base()
        })
        .expect_err("a lead off the quantum");
    assert_eq!(
        refused,
        PrepareDiagnostics::fixed(
            host_core::PrepareRejection::Session,
            "host.successor.warm_lead"
        )
    );
}

/// Gate 2: a post-fader send from latent X into bus S restarts X whole, and S, T and the output
/// lead by 512, with S unchanged and with a limiter on S. X's fader and pre-fader owners do not
/// carry (scope R6).
#[test]
fn a_send_into_a_bus_from_a_latent_track_restarts_the_track() {
    for bus_limiter in [false, true] {
        let a = || {
            let mut model = empty();
            track(&mut model, "x", vec![limiter()]);
            to_output(&mut model, "x");
            submix(
                &mut model,
                "s",
                if bus_limiter {
                    vec![limiter()]
                } else {
                    Vec::new()
                },
            );
            to_output(&mut model, "s");
            track(&mut model, "t", Vec::new());
            to_submix(&mut model, "t", "s");
            model
        };
        let mut b = a();
        to_submix(&mut b, "x", "s");
        let edit = Edit::new(a(), b);
        let lead = warm(edit.decide(&CONFIG));
        assert_eq!(
            (lead.lead_samples, &*lead.restart_whole),
            (LEAD, &names(&["x"])[..]),
            "bus limiter {bus_limiter}"
        );
        let mut w = edit.warm(&lead);
        assert_eq!(restarted(&w), ["x"], "bus limiter {bus_limiter}");
        // Scope R6: the warm program moves none of X's pre-fader owners (its input section and
        // its limiter) and not its fader; everything else the ordinary successor carries, it
        // carries.
        let mut ordinary = edit.prepare(edit.base()).expect("ordinary");
        assert!(ordinary.restarted_strips().is_empty());
        let [sources, inputs, strips, effects] = graph::test_only_carry_counts(&mut ordinary.plan);
        assert_eq!(
            graph::test_only_carry_counts(&mut w.plan),
            [sources, inputs - 1, strips - 1, effects - 1],
            "bus limiter {bus_limiter}"
        );
        let mut carried = stages("s", TrackStage::Input);
        carried.extend(stages("t", TrackStage::Input));
        carried.push(output());
        edit.assert_leads(&w, &carried, LEAD);
    }
}

/// Gate 3: `LeadBound` at the exact thresholds of `ΣP + P` and of a carried ring's headroom
/// above the fixed stall baseline.
#[test]
fn lead_bound_holds_at_the_exact_thresholds() {
    assert_eq!(stall_ring_frames(48_000, 128), 5120);
    assert_eq!(stall_ring_frames(44_100, 128), 4736);
    let stall = stall_ring_frames(48_000, 128);
    // `ΣP = 384`.
    let mut edit = limiter_on_t1();
    host_core::test_only_set_source_read_offset(&mut edit.host_a, 384);
    let bound = |p_max| WarmConfig { p_max, ..CONFIG };
    assert!(matches!(
        edit.decide(&bound(384 + LEAD)),
        WarmDecision::Warm(_)
    ));
    assert_eq!(
        edit.decide(&bound(384 + LEAD - 1)),
        WarmDecision::Unavailable(WarmUnavailable::LeadBound)
    );
    // The ring: `P + q` frames above the stall ring, one quantum less, and none.
    let mut b = two_tracks();
    *inserts(&mut b, "t1") = vec![limiter()];
    for (frames, warm) in [
        (stall + LEAD as u32 + 128, true),
        (stall + LEAD as u32, false),
        (stall, false),
    ] {
        let edit = Edit::with_caps(two_tracks(), b.clone(), ring_caps(frames));
        let decision = edit.decide(&CONFIG);
        assert_eq!(
            matches!(decision, WarmDecision::Warm(_)),
            warm,
            "ring {frames}: {decision:?}"
        );
        if !warm {
            assert_eq!(
                decision,
                WarmDecision::Unavailable(WarmUnavailable::LeadBound)
            );
        }
    }
}

/// Gate 4: the prime is D3 over the warm successor's own claim lines and carried source, and
/// `PrimeBudget` holds at that exact count.
#[test]
fn prime_budget_holds_at_the_exact_count() {
    let edit = limiter_on_t1();
    let lead = warm(edit.decide(&CONFIG));
    let w = edit.warm(&lead);
    // A track's claim line is its `Input` node's floored arrival (its incoming arrival is 0); the
    // tracks share the fixture's one stereo source.
    let lines: u64 = edit
        .b
        .model
        .tracks
        .iter()
        .map(|track| arrival(&w, &stages(track.id.as_str(), TrackStage::Input)[0]))
        .sum();
    let channels: u64 = edit
        .b
        .model
        .sources
        .iter()
        .map(|source| u64::from(source.channels))
        .sum();
    let expected = 4 * (LEAD * channels + 2 * lines);
    assert_eq!(w.prime_bytes(), expected);
    assert_eq!(lead.prime_bytes(), expected);
    let budget = |prime_bytes_max| WarmConfig {
        prime_bytes_max,
        ..CONFIG
    };
    assert!(matches!(
        edit.decide(&budget(expected)),
        WarmDecision::Warm(_)
    ));
    assert_eq!(
        edit.decide(&budget(expected - 1)),
        WarmDecision::Unavailable(WarmUnavailable::PrimeBudget)
    );
}

/// Gate 4 (#1355 D5's narrowing): the prime counts the words its fill writes. After two growths
/// every track holds a claim line of `2P`; a limiter on track 1 then restarts it, and its line,
/// which it keeps, takes only the `P` words of the prime, while every carried track's line grows
/// and is written whole.
#[test]
fn the_prime_counts_the_words_its_fill_writes() {
    let grown = |growths: usize, limited: bool| {
        let mut model = two_tracks();
        if limited {
            *inserts(&mut model, "t1") = vec![limiter()];
        }
        for growth in 0..growths {
            let id = format!("m{growth}");
            let chain = (0..=growth)
                .map(|stage| {
                    let mut stage_limiter = limiter();
                    stage_limiter.id = sid(&format!("limiter-{stage}"));
                    stage_limiter
                })
                .collect();
            muted_track(&mut model, &id, chain);
            to_output(&mut model, &id);
        }
        Session::compile(finish(model))
    };
    let caps = gate_caps();
    let mut stream = alone(&grown(0, false), &caps, backend(), AHEAD);
    for growth in 1..=2 {
        stream.render(BEFORE);
        assert_eq!(stream.grow(&grown(growth, false), &CONFIG), LEAD);
    }
    stream.render(BEFORE);
    assert_eq!(stream.offset, 2 * LEAD);
    let edit = grown(2, true);
    let (lead, w) = stream.warm(&edit, &CONFIG);
    assert_eq!(lead.lead_samples, LEAD);
    assert_eq!(restarted_ids(&w.inventory), ["t1"]);
    let words: u64 = edit
        .model
        .tracks
        .iter()
        .map(|track| {
            let line = arrival(&w, &stages(track.id.as_str(), TrackStage::Input)[0]);
            if track.id.as_str() == "t1" {
                assert_eq!(line, 2 * LEAD, "track 1 keeps its line");
                2 * LEAD
            } else {
                2 * line
            }
        })
        .sum();
    // One carried stereo source.
    let expected = 4 * (LEAD * 2 + words);
    assert_eq!((lead.prime_bytes(), w.prime_bytes()), (expected, expected));

    // A track that reads another source in the edit takes no prime: gate 1's growth with track
    // 2 re-pointed to an added source counts track 1's source alone. (Its warm preparation is
    // refused today, `host.successor.warm_invariant`: #1354a's `R` misses the re-pointed track.)
    let mut b = two_tracks();
    *inserts(&mut b, "t1") = vec![limiter()];
    let mut other = b.sources[0].clone();
    other.id = sid("other-source");
    b.sources.push(other);
    b.sources.sort_by(|left, right| left.id.cmp(&right.id));
    let t2 = b
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == "t2")
        .expect("track 2");
    t2.source_id = sid("other-source");
    let lead = warm(Edit::new(two_tracks(), b).decide(&CONFIG));
    assert_eq!(lead.prime_bytes(), 4 * (LEAD * 2));
}

/// Gate 7: a limiter on a bus restarts the bus's carried feeders whole, and the bus after its
/// fader, T3 and the output lead by `P`, with T3's claim line exactly `P`.
#[test]
fn a_limiter_on_a_bus_restarts_its_feeders() {
    let a = || {
        let mut model = empty();
        submix(&mut model, "b", Vec::new());
        to_output(&mut model, "b");
        for id in ["t1", "t2"] {
            track(&mut model, id, Vec::new());
            to_submix(&mut model, id, "b");
        }
        track(&mut model, "t3", Vec::new());
        to_output(&mut model, "t3");
        model
    };
    let mut b = a();
    *inserts(&mut b, "b") = vec![limiter()];
    let edit = Edit::new(a(), b);
    let lead = warm(edit.decide(&CONFIG));
    assert_eq!(
        (lead.lead_samples, &*lead.restart_whole),
        (LEAD, &names(&["t1", "t2"])[..])
    );
    let w = edit.warm(&lead);
    assert_eq!(restarted(&w), ["b", "t1", "t2"]);
    let mut carried = stages("t3", TrackStage::Input);
    carried.extend(stages("b", TrackStage::PostMatrix));
    carried.extend([route_node("b-main"), route_node("t3-main"), output()]);
    edit.assert_leads(&w, &carried, LEAD);
    assert_eq!(arrival(&w, &stages("t3", TrackStage::Input)[0]), LEAD);
}

/// Gate 8: a carried key or feeder that holds a restarted chain late is restarted whole: the
/// walk follows sidechain edges.
#[test]
fn a_carried_key_or_feeder_holding_a_restarted_chain_late_restarts() {
    for tap in [SendTap::PreFader, SendTap::Input] {
        let a = || {
            let mut model = empty();
            for id in ["k", "v"] {
                track(&mut model, id, Vec::new());
                to_output(&mut model, id);
            }
            track(
                &mut model,
                "bs",
                vec![keyed_compressor(from_track("k", tap))],
            );
            to_output(&mut model, "bs");
            model
        };
        let mut b = a();
        inserts(&mut b, "bs").push(limiter());
        let edit = Edit::new(a(), b);
        let lead = warm(edit.decide(&CONFIG));
        assert_eq!(
            (lead.lead_samples, &*lead.restart_whole),
            (LEAD, &names(&["k"])[..]),
            "key tap {tap:?}"
        );
        let w = edit.warm(&lead);
        assert_eq!(restarted(&w), ["bs", "k"], "key tap {tap:?}");
        let mut carried = stages("v", TrackStage::Input);
        carried.extend([route_node("v-main"), output()]);
        edit.assert_leads(&w, &carried, LEAD);
    }

    let a = || {
        let mut model = empty();
        submix(&mut model, "b", Vec::new());
        to_output(&mut model, "b");
        submix(
            &mut model,
            "b2",
            vec![keyed_compressor(from_track("t6", SendTap::PreFader))],
        );
        to_submix(&mut model, "b2", "b");
        track(&mut model, "t4", Vec::new());
        to_submix(&mut model, "t4", "b2");
        track(&mut model, "t5", Vec::new());
        to_submix(&mut model, "t5", "b");
        for id in ["t3", "t6"] {
            track(&mut model, id, Vec::new());
            to_output(&mut model, id);
        }
        model
    };
    let mut b = a();
    *inserts(&mut b, "b") = vec![limiter()];
    let edit = Edit::new(a(), b);
    let lead = warm(edit.decide(&CONFIG));
    assert_eq!(
        (lead.lead_samples, &*lead.restart_whole),
        (LEAD, &names(&["b2", "t4", "t5", "t6"])[..])
    );
    let w = edit.warm(&lead);
    assert_eq!(restarted(&w), ["b", "b2", "t4", "t5", "t6"]);
    let mut carried = stages("t3", TrackStage::Input);
    carried.extend(stages("b", TrackStage::PostMatrix));
    carried.extend([route_node("t3-main"), route_node("b-main"), output()]);
    edit.assert_leads(&w, &carried, LEAD);
}

/// Gate 9: a parallel bus on a bus restarts the bus and only the feeder that holds it late.
#[test]
fn a_parallel_bus_restarts_only_the_feeder_holding_it_late() {
    let a = || {
        let mut model = empty();
        submix(&mut model, "bu", Vec::new());
        to_output(&mut model, "bu");
        track(&mut model, "t0", Vec::new());
        to_submix(&mut model, "t0", "bu");
        track(&mut model, "t1", vec![limiter()]);
        to_submix(&mut model, "t1", "bu");
        model
    };
    let mut b = a();
    submix(&mut b, "nb", vec![limiter()]);
    to_output(&mut b, "nb");
    route(
        &mut b,
        "bu-nb",
        from_submix("bu", SendTap::PostPan),
        Some("nb"),
    );
    let edit = Edit::new(a(), b);
    let lead = warm(edit.decide(&CONFIG));
    assert_eq!(
        (lead.lead_samples, &*lead.restart_whole),
        (LEAD, &names(&["bu", "t1"])[..])
    );
    let w = edit.warm(&lead);
    assert_eq!(restarted(&w), ["bu", "t1"]);
    let mut carried = stages("t0", TrackStage::Input);
    carried.extend([route_node("t0-bu"), output()]);
    edit.assert_leads(&w, &carried, LEAD);
}

/// The invariant error a warm preparation returns.
fn invariant() -> PrepareDiagnostics {
    PrepareDiagnostics::fixed(
        host_core::PrepareRejection::Graph,
        "host.successor.warm_invariant",
    )
}

/// D4's C1 re-check: a lead too short for the edit (gate 1's restart set, `P = 384` instead of
/// 512) leaves the output late, and the preparation returns the invariant error.
#[test]
fn a_lead_that_misses_c1_is_the_invariant_error() {
    let edit = limiter_on_t1();
    let mut lead = warm(edit.decide(&CONFIG));
    lead.lead_samples = 3 * QUANTUM;
    let refused = edit
        .prepare(SuccessorBase {
            warm: Some(&lead),
            ..edit.base()
        })
        .expect_err("a lead that misses C1");
    assert_eq!(refused, invariant());
}

/// D4's join re-check (#1354a ruling 2): gate 1's lead on an edit that also restarts track 2
/// (a latency-free insert changes its chain) keeps C1 but the join restarts a strip outside the
/// lead's `R`, and the preparation returns the invariant error.
#[test]
fn a_join_outside_the_leads_restart_set_is_the_invariant_error() {
    let lead = warm(limiter_on_t1().decide(&CONFIG));
    let mut b = two_tracks();
    *inserts(&mut b, "t1") = vec![limiter()];
    *inserts(&mut b, "t2") = vec![effect(
        "comp",
        "miso.compressor",
        SidechainDeclaration::None,
    )];
    let other = Edit::new(two_tracks(), b);
    let refused = other
        .prepare(SuccessorBase {
            warm: Some(&lead),
            ..other.base()
        })
        .expect_err("a join outside R");
    assert_eq!(refused, invariant());
}

// -------------------------------------------------------------------------------------------------
// #1355: render's readiness check and the adoption of a warm successor with its prime. Every gate
// streams a predecessor A, publishes the warm successor W `Primed { lead_blocks: P / q }` through
// the plan exchange (D3) after setting the carried ring's `prime_required` flag, and renders on.
// "A continued" is A rendered on with no swap and the same host commands.
// -------------------------------------------------------------------------------------------------

/// The fixture's one source, which every track reads.
const SOURCE: &str = "fixture-source";
/// Blocks of PCM a stream can read: well inside the fixture source's region.
const PCM_BLOCKS: usize = 300;

/// The host's feed of the one source: generation `generation` from source frame `base` at the
/// source-read sample `anchor` on, kept `ahead` frames past the frame the next block reads.
struct Feeder {
    planes: [Vec<f32>; 2],
    generation: u64,
    base: u64,
    anchor: u64,
    next: u64,
    ahead: u64,
    rate: u32,
}

impl Feeder {
    fn new(rate: u32, ahead_blocks: u64) -> Self {
        let Feed { planes, .. } = Feed::with_blocks(SOURCE, 7, PCM_BLOCKS);
        Self {
            planes,
            generation: 1,
            base: 0,
            anchor: 0,
            next: 0,
            ahead: ahead_blocks * QUANTUM,
            rate,
        }
    }

    /// Submit whole blocks until the source holds `ahead` frames past the block that reads from
    /// the source-read sample `read`.
    fn top_up(&mut self, sources: &mut host_core::SourceControlSet, read: u64) {
        let until = self.base + (read + self.ahead).saturating_sub(self.anchor);
        while self.next < until {
            let range = self.next as usize..(self.next + QUANTUM) as usize;
            sources
                .submit(
                    SOURCE.as_bytes(),
                    host_core::SourceSubmission {
                        generation: self.generation,
                        start_frame: self.next,
                        sample_rate_hz: self.rate,
                        planes: &[&self.planes[0][range.clone()], &self.planes[1][range]],
                        frames: QUANTUM as u32,
                        end_of_region: false,
                    },
                )
                .unwrap_or_else(|error| panic!("submit at {}: {error:?}", self.next));
            self.next += QUANTUM;
        }
    }

    /// Seek the source to generation `generation` from `frame`, entering at the source-read
    /// sample `anchor`, and feed that generation from then on.
    fn seek_at(
        &mut self,
        sources: &mut host_core::SourceControlSet,
        generation: u64,
        frame: u64,
        anchor: u64,
    ) {
        sources
            .seek_at(SOURCE.as_bytes(), generation, frame, anchor)
            .unwrap_or_else(|error| panic!("seek_at: {error:?}"));
        self.generation = generation;
        self.base = frame;
        self.anchor = anchor;
        self.next = frame;
    }
}

/// One stream of plans through the plan exchange, as a host drives it.
struct Stream {
    publisher: engine::realtime::PlanPublisher,
    owner: engine::realtime::RealtimePlanOwner,
    retirer: engine::realtime::PlanRetirer,
    sources: host_core::SourceControlSet,
    /// The newest published plan's inventory and committed model: the next edit's base.
    inventory: host_core::PlanStateInventory,
    committed: SessionModel,
    /// The running plan's source-read offset, and the pending candidate's.
    offset: u64,
    pending: Option<u64>,
    revision: u64,
    feeder: Feeder,
    blocks: Vec<Block>,
    caps: HostPrepareCaps,
    backend: Backend,
}

impl Stream {
    fn start(
        host: PreparedHost,
        committed: &SessionModel,
        caps: &HostPrepareCaps,
        backend: Backend,
        ahead_blocks: u64,
    ) -> Self {
        let rate = host.report.sample_rate_hz;
        let PreparedHost {
            plan,
            sources,
            inventory,
            ..
        } = host;
        let offset = inventory.source_read_offset();
        let (publisher, owner, retirer) = exchange(plan);
        Self {
            publisher,
            owner,
            retirer,
            sources,
            inventory,
            committed: committed.clone(),
            offset,
            pending: None,
            revision: 1,
            feeder: Feeder::new(rate, ahead_blocks),
            blocks: Vec::new(),
            caps: *caps,
            backend,
        }
    }

    /// The source-read sample the next block reads from on the running plan.
    fn read(&self) -> u64 {
        self.owner.next_absolute_sample() + self.offset
    }

    /// Render `count` blocks, topping the feed up before each, and reclaim the plans they
    /// displaced off the render thread.
    fn render(&mut self, count: usize) {
        for _ in 0..count {
            let read = self.read();
            self.feeder.top_up(&mut self.sources, read);
            let block = render(&mut self.owner);
            if block.swap == SwapOutcome::Applied {
                self.offset = self.pending.take().expect("a pending candidate");
            }
            self.blocks.push(block);
            while self.retirer.try_reclaim().is_ok() {}
        }
    }

    fn base(&self) -> SuccessorBase<'_> {
        SuccessorBase::new(&self.inventory, &self.committed, self.revision + 1)
    }

    /// Publish `host`, prepared from [`Self::base`], with `adoption`; for a `Primed` candidate,
    /// first set the carried ring's `prime_required` flag (D3).
    fn publish(
        &mut self,
        mut host: PreparedHost,
        committed: &SessionModel,
        adoption: PlanAdoption,
    ) {
        assert!(self.pending.is_none(), "one candidate at a time");
        host.sources.adopt_persisting(&mut self.sources);
        if let PlanAdoption::Primed { .. } = adoption {
            host.sources
                .test_only_prime_flag(SOURCE.as_bytes())
                .expect("the carried ring")
                .set(true);
        }
        self.revision += 1;
        let PreparedHost {
            mut plan,
            sources,
            inventory,
            ..
        } = host;
        plan.take_revision_gate()
            .expect("the successor's gate")
            .publish(self.revision);
        self.pending = Some(inventory.source_read_offset());
        self.publisher
            .reserve_replacement(plan, adoption)
            .unwrap_or_else(|_| panic!("reserve the successor"))
            .commit(self.revision);
        self.sources = sources;
        self.inventory = inventory;
        self.committed = committed.clone();
    }

    /// The warm lead of `b` from the newest plan, and its warm successor.
    fn warm(&self, b: &Session, config: &WarmConfig) -> (WarmLead, PreparedHost) {
        let ordinary = successor_of(b, &self.caps, self.base(), self.backend)
            .unwrap_or_else(|failure| panic!("ordinary successor: {failure:?}"));
        let lead = warm(
            warm_lead(
                &b.compiled,
                &self.caps,
                self.base(),
                &ordinary.inventory,
                config,
            )
            .unwrap_or_else(|failure| panic!("warm_lead: {failure:?}")),
        );
        let host = successor_of(
            b,
            &self.caps,
            SuccessorBase {
                warm: Some(&lead),
                ..self.base()
            },
            self.backend,
        )
        .unwrap_or_else(|failure| panic!("warm successor: {failure:?}"));
        (lead, host)
    }

    /// Grow to `b` warm: publish its warm successor `Primed` and return `P`.
    fn grow(&mut self, b: &Session, config: &WarmConfig) -> u64 {
        let (lead, host) = self.warm(b, config);
        let lead_blocks = u32::try_from(lead.lead_samples / QUANTUM).expect("blocks");
        self.publish(
            host,
            &b.model,
            PlanAdoption::Primed {
                not_before: 0,
                lead_blocks,
            },
        );
        lead.lead_samples
    }

    /// Rebuild to `b` with an ordinary successor, adopted at the next block.
    fn rebuild(&mut self, b: &Session) {
        let host = successor_of(b, &self.caps, self.base(), self.backend)
            .unwrap_or_else(|failure| panic!("ordinary successor: {failure:?}"));
        self.publish(host, &b.model, PlanAdoption::Next);
    }

    /// A render call at the wrong sample: render adopts the due candidate at the block boundary,
    /// then refuses the call, so the adopted plan renders nothing (#1652 MINOR-2's path).
    fn refused_call(&mut self) {
        let read = self.read();
        self.feeder.top_up(&mut self.sources, read);
        let mut output = [f32::NAN; QUANTUM as usize * 2];
        let wrong = self.owner.next_absolute_sample() + 1;
        engine::realtime::audit::reset();
        let mark = bench_support::alloc::current_thread_counters();
        let refused = self.owner.render_contiguous(
            engine::realtime::RenderIo {
                output: engine::realtime::PlanarBufferMut::try_new(
                    &mut output,
                    2,
                    QUANTUM as usize,
                    QUANTUM as usize,
                )
                .expect("output"),
            },
            wrong,
        );
        let delta = bench_support::alloc::current_thread_delta_since(mark);
        let audit = engine::realtime::audit::snapshot();
        assert_eq!(
            (
                delta.allocations,
                delta.reallocations,
                delta.deallocations,
                audit.allocations,
                audit.deallocations
            ),
            (0, 0, 0, 0, 0),
            "the call that adopts the candidate and refuses allocates or frees nothing"
        );
        assert!(
            matches!(
                refused,
                Err(engine::realtime::RenderError::TimeDiscontinuity { .. })
            ),
            "the call is refused after the boundary: {refused:?}"
        );
        assert!(
            self.retirer.try_reclaim().is_ok(),
            "the candidate was adopted"
        );
        self.offset = self.pending.take().expect("a pending candidate");
    }

    /// The block the newest swap applied at.
    fn swap_blocks(&self) -> Vec<usize> {
        self.blocks
            .iter()
            .enumerate()
            .filter(|(_, block)| block.swap == SwapOutcome::Applied)
            .map(|(index, _)| index)
            .collect()
    }

    fn bits(&self) -> Vec<&[u32]> {
        self.blocks.iter().map(|block| &block.bits[..]).collect()
    }
}

/// A stream of `session` alone, fed `ahead_blocks` ahead.
fn alone(session: &Session, caps: &HostPrepareCaps, backend: Backend, ahead: u64) -> Stream {
    Stream::start(
        fresh(session, caps, backend),
        &session.model,
        caps,
        backend,
        ahead,
    )
}

/// Blocks rendered before an edit.
const BEFORE: usize = 6;
/// Blocks rendered after one.
const AFTER: usize = 10;
/// How far ahead a ready host feeds: `P / q + 1` blocks for `P = 512`, and one more.
const AHEAD: u64 = LEAD / QUANTUM + 2;

/// The first block whose bits differ between two streams, if any.
fn first_divergence(left: &Stream, right: &Stream) -> Option<usize> {
    let (left, right) = (left.bits(), right.bits());
    assert_eq!(left.len(), right.len());
    left.iter()
        .zip(&right)
        .position(|(left, right)| left != right)
}

/// Gate 1: a warm successor that adds a muted limiter track (and, second shape, feeds it into a
/// submix whose arrival grows while the output's does not) is adopted at the first ready block
/// and renders A continued, bit for bit; block `j` of it is block `j + k` of a fresh W compiled
/// with the same floors and fed from frame 0; and the watermark reports `EXACT` at the adoption
/// block. Both bank widths.
#[test]
fn a_primed_warm_successor_continues_its_predecessor_bit_for_bit() {
    let parallel = || {
        let mut model = empty();
        track(&mut model, "x", vec![limiter()]);
        to_output(&mut model, "x");
        submix(&mut model, "s", Vec::new());
        to_output(&mut model, "s");
        track(&mut model, "t", Vec::new());
        to_submix(&mut model, "t", "s");
        model
    };
    let shapes = [
        {
            let mut b = two_tracks();
            muted_track(&mut b, "t3", vec![limiter()]);
            to_output(&mut b, "t3");
            (two_tracks(), b)
        },
        {
            let mut b = parallel();
            muted_track(&mut b, "m", vec![limiter()]);
            to_submix(&mut b, "m", "s");
            (parallel(), b)
        },
    ];
    for &backend in Backend::VECTOR {
        for (shape, (a, b)) in shapes.iter().enumerate() {
            let (a, b) = (
                Session::compile(finish(a.clone())),
                Session::compile(finish(b.clone())),
            );
            let caps = gate_caps();
            let mut swapped = alone(&a, &caps, backend, AHEAD);
            swapped.render(BEFORE);
            let lead = swapped.grow(&b, &CONFIG);
            assert_eq!(lead, LEAD, "shape {shape}");
            let k = (lead / QUANTUM) as usize;
            swapped.render(AFTER);
            assert_eq!(
                swapped.swap_blocks(),
                [BEFORE],
                "shape {shape}: adopted at once"
            );
            let watermark = swapped
                .publisher
                .watermark_reader()
                .read()
                .expect("watermark");
            assert_eq!(
                (
                    watermark.revision,
                    watermark.first_sample,
                    watermark.outcome_flags
                ),
                (
                    swapped.revision,
                    BEFORE as u64 * QUANTUM,
                    engine::realtime::OUTCOME_EXACT
                ),
                "shape {shape}"
            );
            let mut continued = alone(&a, &caps, backend, AHEAD);
            continued.render(BEFORE + AFTER);
            assert_eq!(
                first_divergence(&swapped, &continued),
                None,
                "shape {shape}, {backend:?}: A continued"
            );
            // The fresh W: the warm successor's floors, offset 0, fed from frame 0.
            let floors = swapped.inventory.test_only_floors_only();
            let reference =
                successor_of(&b, &caps, SuccessorBase::new(&floors, &b.model, 2), backend)
                    .expect("the fresh W");
            assert_eq!(reference.inventory.source_read_offset(), 0);
            let mut reference = Stream::start(reference, &b.model, &caps, backend, AHEAD);
            reference.render(BEFORE + AFTER + k);
            for j in 0..AFTER {
                assert_eq!(
                    swapped.blocks[BEFORE + j].bits,
                    reference.blocks[BEFORE + j + k].bits,
                    "shape {shape}, {backend:?}: block {j} after the swap"
                );
            }
        }
    }
}

/// Gate 2: a growth, an ordinary rebuild that adds another muted track, and a second growth
/// (offset `2P`): the output equals A continued throughout. Both bank widths.
#[test]
fn growth_rebuild_growth_continues_its_predecessor() {
    let steps = |count: usize| {
        let mut model = two_tracks();
        for (index, effects) in [
            vec![limiter()],
            Vec::new(),
            vec![limiter(), {
                let mut second = limiter();
                second.id = sid("limiter-2");
                second
            }],
        ]
        .into_iter()
        .take(count)
        .enumerate()
        {
            let id = format!("m{index}");
            muted_track(&mut model, &id, effects);
            to_output(&mut model, &id);
        }
        Session::compile(finish(model))
    };
    for &backend in Backend::VECTOR {
        let caps = gate_caps();
        let a = steps(0);
        let mut swapped = alone(&a, &caps, backend, AHEAD);
        swapped.render(BEFORE);
        assert_eq!(swapped.grow(&steps(1), &CONFIG), LEAD);
        swapped.render(AFTER);
        swapped.rebuild(&steps(2));
        swapped.render(AFTER);
        assert_eq!(swapped.offset, LEAD, "the rebuild inherits the offset");
        assert_eq!(swapped.grow(&steps(3), &CONFIG), LEAD);
        swapped.render(AFTER);
        assert_eq!(swapped.offset, 2 * LEAD);
        assert_eq!(
            swapped.swap_blocks(),
            [BEFORE, BEFORE + AFTER, BEFORE + 2 * AFTER],
            "{backend:?}"
        );
        let mut continued = alone(&a, &caps, backend, AHEAD);
        continued.render(BEFORE + 3 * AFTER);
        assert_eq!(first_divergence(&swapped, &continued), None, "{backend:?}");
    }
}

/// Gate 3: track 1 (muted) keys track 2's compressor from its `input` tap; a limiter inserted on
/// track 1 restarts it and leaves track 2 carried, whose sidechain line from track 1 grows by
/// `P`. The output equals A continued. Second shape: the same edit after a first growth (a muted
/// limiter track), so track 1 holds a claim line of `P` it keeps: the sidechain line takes the
/// head of its stream (that claim line's pending words) and track 1's line its tail (the prime).
/// Both bank widths.
#[test]
fn an_input_tap_sidechain_from_a_restarted_track_continues() {
    let a = |grown: bool| {
        let mut model = empty();
        muted_track(&mut model, "t1", Vec::new());
        to_output(&mut model, "t1");
        track(
            &mut model,
            "t2",
            vec![keyed_compressor(from_track("t1", SendTap::Input))],
        );
        to_output(&mut model, "t2");
        if grown {
            muted_track(&mut model, "t3", vec![limiter()]);
            to_output(&mut model, "t3");
        }
        model
    };
    let edit = |grown: bool| {
        let mut b = a(grown);
        *inserts(&mut b, "t1") = vec![limiter()];
        Session::compile(finish(b))
    };
    let start = Session::compile(finish(a(false)));
    for &backend in Backend::VECTOR {
        for grown in [false, true] {
            let caps = gate_caps();
            let mut swapped = alone(&start, &caps, backend, AHEAD);
            swapped.render(BEFORE);
            if grown {
                assert_eq!(
                    swapped.grow(&Session::compile(finish(a(true))), &CONFIG),
                    LEAD
                );
                swapped.render(AFTER);
            }
            assert_eq!(swapped.grow(&edit(grown), &CONFIG), LEAD);
            assert_eq!(restarted_ids(&swapped.inventory), ["t1"]);
            swapped.render(AFTER);
            assert_eq!(swapped.swap_blocks().len(), 1 + usize::from(grown));
            let mut continued = alone(&start, &caps, backend, AHEAD);
            continued.render(swapped.blocks.len());
            assert_eq!(
                first_divergence(&swapped, &continued),
                None,
                "{backend:?}, after a growth: {grown}"
            );
        }
    }
}

/// A warm successor displaced before its first block (#1355 MAJOR-1): render adopts W in a call
/// it then refuses, so W renders nothing, and the successor C, prepared from W's inventory,
/// displaces it. C's carry replays W's pending prime at C's adoption block before it takes W's
/// lines and sources, so the output equals A continued. C is an ordinary rebuild, adopted at
/// once, or a second growth: then readiness walks W's pending prime and C's (from the ring head,
/// `k_W + k_C + 1` blocks), so a feed of only `P_C + q` past the render clock holds C while W
/// renders its first block, and C is adopted once fed. No block, the refused call included,
/// allocates or frees on the render thread. Both bank widths.
#[test]
fn a_warm_successor_displaced_before_its_first_block_keeps_its_prime() {
    let a = Session::compile(finish(two_tracks()));
    let mut b = two_tracks();
    muted_track(&mut b, "t3", vec![limiter()]);
    to_output(&mut b, "t3");
    let mut rebuilt = b.clone();
    muted_track(&mut rebuilt, "t4", Vec::new());
    to_output(&mut rebuilt, "t4");
    let mut grown = b.clone();
    muted_track(
        &mut grown,
        "t4",
        vec![limiter(), {
            let mut second = limiter();
            second.id = sid("limiter-2");
            second
        }],
    );
    to_output(&mut grown, "t4");
    let b = Session::compile(finish(b));
    let (rebuilt, grown) = (
        Session::compile(finish(rebuilt)),
        Session::compile(finish(grown)),
    );
    for &backend in Backend::VECTOR {
        for warm_c in [false, true] {
            let caps = gate_caps();
            let mut swapped = alone(&a, &caps, backend, AHEAD);
            swapped.render(BEFORE);
            assert_eq!(swapped.grow(&b, &CONFIG), LEAD);
            swapped.refused_call();
            assert_eq!(swapped.offset, LEAD);
            let adopted = if warm_c {
                assert_eq!(swapped.grow(&grown, &CONFIG), LEAD);
                // `P_C + q` past the render clock: one block past W's read.
                swapped.feeder.ahead = LEAD + QUANTUM - swapped.offset;
                swapped.render(1);
                assert!(
                    swapped.swap_blocks().is_empty(),
                    "{backend:?}: C held while W's prime is pending"
                );
                swapped.feeder.ahead = AHEAD * QUANTUM;
                swapped.render(AFTER - 1);
                BEFORE + 1
            } else {
                swapped.rebuild(&rebuilt);
                swapped.render(AFTER);
                BEFORE
            };
            assert_eq!(swapped.swap_blocks(), [adopted], "{backend:?}, {warm_c}");
            for (index, block) in swapped.blocks.iter().enumerate() {
                assert_eq!(
                    (block.allocator, block.audit),
                    ((0, 0, 0), (0, 0)),
                    "{backend:?}, {warm_c}: block {index}"
                );
            }
            let mut continued = alone(&a, &caps, backend, AHEAD);
            continued.render(BEFORE + AFTER);
            assert_eq!(
                first_divergence(&swapped, &continued),
                None,
                "{backend:?}, warm C: {warm_c}"
            );
        }
    }
}

fn restarted_ids(inventory: &host_core::PlanStateInventory) -> Vec<&str> {
    inventory
        .restarted_strips()
        .iter()
        .map(|id| &**id)
        .collect()
}

/// Gate 4: readiness. With the frames past each consumer withheld (the ring holds `P` frames
/// past the block, not `P + q`) W is not adopted and the output equals A continued; once the
/// frames arrive, W is adopted at the next block. A `seek_at` anchored inside the prime window
/// holds the adoption until the block after the seek applied, and the output equals A continued
/// with the same seek. The seek case runs after a first growth, so the running plan's offset `O`
/// is `P` and the window `[S + O, S + O + P + q)` is read at `S + O`, not at `S`.
#[test]
fn a_warm_successor_waits_for_every_carried_ring_and_a_held_seek() {
    let a = Session::compile(finish(two_tracks()));
    let mut b = two_tracks();
    muted_track(&mut b, "t3", vec![limiter()]);
    to_output(&mut b, "t3");
    let mut c = b.clone();
    muted_track(
        &mut c,
        "t4",
        vec![limiter(), {
            let mut second = limiter();
            second.id = sid("limiter-2");
            second
        }],
    );
    to_output(&mut c, "t4");
    let (b, c) = (Session::compile(finish(b)), Session::compile(finish(c)));
    let caps = gate_caps();
    let backend = backend();
    let k = LEAD / QUANTUM;
    // Withheld, then released.
    let mut swapped = alone(&a, &caps, backend, k);
    swapped.render(BEFORE);
    swapped.grow(&b, &CONFIG);
    swapped.render(4);
    assert!(
        swapped.swap_blocks().is_empty(),
        "not adopted while withheld"
    );
    swapped.feeder.ahead = (k + 1) * QUANTUM;
    swapped.render(AFTER);
    assert_eq!(swapped.swap_blocks(), [BEFORE + 4], "adopted once ready");
    let mut continued = alone(&a, &caps, backend, k);
    continued.render(BEFORE + 4);
    continued.feeder.ahead = (k + 1) * QUANTUM;
    continued.render(AFTER);
    assert_eq!(first_divergence(&swapped, &continued), None, "withheld");

    // A seek anchored two blocks past the running plan's read, inside the window
    // `[S + O, S + O + P + q)`. A source-read sample `X` is heard at render sample `X` on either
    // plan (`O` ahead, then `O` late in the claim lines), so A continued takes the same anchor.
    // Each feed is first topped up to `through` with the old generation: A continued's (`O`
    // behind) to the anchor, so it has no gap, and the swapped stream's past the window's end,
    // so the frames readiness walks are all there and only the held anchor holds the adoption
    // (the seek discards the frames past the anchor).
    let seek = |stream: &mut Stream, anchor: u64, through: u64| {
        let ahead = stream.feeder.ahead;
        stream
            .feeder
            .top_up(&mut stream.sources, through.saturating_sub(ahead));
        stream
            .feeder
            .seek_at(&mut stream.sources, 2, 40 * QUANTUM, anchor);
    };
    let mut swapped = alone(&a, &caps, backend, AHEAD);
    swapped.render(BEFORE);
    swapped.grow(&b, &CONFIG);
    swapped.render(AFTER);
    assert_eq!(swapped.offset, LEAD);
    let anchor = swapped.read() + 2 * QUANTUM;
    seek(&mut swapped, anchor, anchor + 2 * LEAD);
    swapped.grow(&c, &CONFIG);
    swapped.render(AFTER);
    assert_eq!(
        swapped.swap_blocks(),
        [BEFORE, BEFORE + AFTER + 3],
        "adopted at the block after the seek applied"
    );
    let mut continued = alone(&a, &caps, backend, AHEAD);
    continued.render(BEFORE + AFTER);
    seek(&mut continued, anchor, anchor);
    continued.render(AFTER);
    assert_eq!(first_divergence(&swapped, &continued), None, "held seek");
}

/// Gate 6: #1286 D1's growth chain at every launch rate. Growth `j` adds to track 1 a limiter and
/// `c(fs)` soft clippers; after the fourth, `ΣP` is `p_max_samples(fs, 128)`. Every pending block
/// (readiness checks while the frames are withheld) and every adoption block, the fourth's
/// included, allocates and frees nothing on the render thread.
#[test]
fn the_growth_chain_primes_without_allocating() {
    for (rate, clippers) in [(44_100, 2), (48_000, 0), (88_200, 0), (96_000, 1)] {
        let model = |growths: usize| {
            let mut model = two_tracks();
            model.sample_rate_hz = rate;
            let chain = inserts(&mut model, "t1");
            for growth in 0..growths {
                let mut stage = limiter();
                stage.id = sid(&format!("limiter-{growth}"));
                chain.push(stage);
                for clipper in 0..clippers {
                    let mut clip = effect(
                        &format!("clip-{growth}-{clipper}"),
                        "miso.soft-clip",
                        SidechainDeclaration::None,
                    );
                    clip.link_mode = LinkMode::DualMono;
                    chain.push(clip);
                }
            }
            Session::compile(finish(model))
        };
        let p_max = u64::from(host_core::p_max_samples(rate, 128));
        let caps = ring_caps(stall_ring_frames(rate, 128) + p_max as u32 + 128);
        let config = WarmConfig { p_max, ..CONFIG };
        let mut stream = alone(&model(0), &caps, backend(), 0);
        stream.feeder.rate = rate;
        let mut leads = Vec::new();
        for growth in 1..=4 {
            stream.feeder.ahead = QUANTUM;
            stream.render(BEFORE);
            let lead = stream.grow(&model(growth), &config);
            leads.push(lead);
            stream.render(2);
            stream.feeder.ahead = lead + QUANTUM;
            stream.render(AFTER);
        }
        assert_eq!(
            leads.iter().sum::<u64>(),
            p_max,
            "{rate} Hz: ΣP is p_max_samples"
        );
        assert_eq!(stream.offset, p_max);
        let swaps = stream.swap_blocks();
        assert_eq!(swaps.len(), 4, "{rate} Hz: four adoptions");
        for (index, block) in stream.blocks.iter().enumerate() {
            assert_eq!(
                (block.allocator, block.audit),
                ((0, 0, 0), (0, 0)),
                "{rate} Hz: block {index} (adoptions at {swaps:?})"
            );
        }
    }
}
