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

#![cfg(feature = "test-support")]

#[path = "support/successor.rs"]
mod successor;

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
use successor::{Session, caps};

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
}

impl Edit {
    fn new(a: SessionModel, b: SessionModel) -> Self {
        Self::with_caps(a, b, gate_caps())
    }

    fn with_caps(a: SessionModel, b: SessionModel, caps: HostPrepareCaps) -> Self {
        let a = Session::compile(finish(a));
        let b = Session::compile(finish(b));
        let host_a = host_core::test_only_prepare_host_runtime_with_live_controls_on(
            &a.compiled,
            &caps,
            &HostLiveControlRequest::default(),
            backend(),
        )
        .unwrap_or_else(|failure| panic!("A: {failure:?}"))
        .0;
        Self { a, b, caps, host_a }
    }

    fn base(&self) -> SuccessorBase<'_> {
        SuccessorBase::new(&self.host_a.inventory, &self.a.model, 2)
    }

    fn prepare(&self, base: SuccessorBase<'_>) -> Result<PreparedHost, PrepareDiagnostics> {
        host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
            &self.b.compiled,
            &self.caps,
            &HostLiveControlRequest::default(),
            base,
            backend(),
        )
        .map(|(host, _)| host)
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
        // carries. Of the compensation lines, X's are all new, and the one line A and the warm
        // successor share, `s-main`'s (without the bus limiter), carries: the whole-strip restart
        // filter (#1324a) drops no line of a strip it does not restart.
        let mut ordinary = edit.prepare(edit.base()).expect("ordinary");
        assert!(ordinary.restarted_strips().is_empty());
        let [sources, inputs, strips, effects, _] =
            graph::test_only_carry_counts(&mut ordinary.plan);
        assert_eq!(
            graph::test_only_carry_counts(&mut w.plan),
            [
                sources,
                inputs - 1,
                strips - 1,
                effects - 1,
                usize::from(!bus_limiter)
            ],
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
