//! Issue #2232: `warm_lead` decides on the graph projection, not a full compile.
//!
//! Each pass of `warm_lead` reads the successor's arrivals, edges, schedule and delays at the
//! lead's floors. Before #2232 it read them from a full compile, which prepared every native effect
//! of the session: on the console's bus add each pass held the whole effect state a second time,
//! covered by no admission (217 MB at 256 tracks natively; a trap at the 512 MiB Wasm cap). The
//! graph projection holds the same rows and prepares no processor.
//!
//! Issue #2234: the transition the same bus add plans after preparation (`plan_strip_transition`)
//! holds no more than `strip_transition_ceiling`, the row both replacement admissions charge.

use bench_support::alloc as audited_allocator;
use host_core::{
    CompiledSession, DuckOverlay, HostLiveControlRequest, HostPrepareCaps, HostShapePolicy,
    PreparedHost, SuccessorBase, WarmConfig, compile_host_model, plan_strip_transition,
    prepare_host_runtime_with_live_controls, prepare_host_runtime_with_live_controls_successor,
    project_effects, stall_ring_frames, strip_transition_ceiling, warm_lead,
};
use session::{
    ConsoleEntry, ConsoleLinkMode, ConsoleSlot, Effect, EffectIdentity, EffectQuality, LinkMode,
    Route, RouteDestination, RouteSource, SendTap, SessionModel, SidechainDeclaration, StableId,
    Submix, parse_session_json,
};

/// The gate's track count: a quarter of browser `console-limits`' 256 (#2191).
const TRACKS: usize = 64;

fn sid(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

fn effect(id: &str, native: &str) -> Effect {
    Effect {
        id: sid(id),
        identity: EffectIdentity::Native {
            effect_id: sid(native),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    }
}

fn slot(id: &str, native: &str, link_mode: LinkMode) -> ConsoleSlot {
    ConsoleSlot {
        slot: sid(id),
        identity: EffectIdentity::Native {
            effect_id: sid(native),
        },
        quality: EffectQuality::Normal,
        link_mode,
    }
}

/// The browser `console-limits` session's shape (#2191) on the nine-track fixture's source:
/// `tracks` tracks with an EQ and a compressor in the console's pre-insert section, a limiter in
/// its post-insert section, a gate, a compressor and a delay inserted, and one route each to the
/// output.
fn console(tracks: usize) -> SessionModel {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.json"
    ))
    .expect("fixture parses");
    model.console.pre_insert = vec![
        slot("eq", "miso.parametric-eq", LinkMode::DualMono),
        slot("comp", "miso.compressor", LinkMode::DualMono),
    ];
    model.console.post_insert = vec![slot("limiter", "miso.true-peak-limiter", LinkMode::Maximum)];
    let mut track = model.tracks[0].clone();
    track.console = ["eq", "comp", "limiter"]
        .into_iter()
        .map(|id| ConsoleEntry {
            slot: sid(id),
            bypass: false,
            link_mode: ConsoleLinkMode::Slot,
            params: Vec::new(),
        })
        .collect();
    track.inserts.effects = vec![
        effect("gate", "miso.gate-expander"),
        effect("squeeze", "miso.compressor"),
        effect("slap", "miso.delay"),
    ];
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    model.automation.clear();
    for index in 0..tracks {
        let track_id = sid(&format!("ch{index:03}"));
        model.tracks.push(session::Track {
            id: track_id.clone(),
            ..track.clone()
        });
        model.routes.push(Route {
            id: sid(&format!("ch{index:03}-main")),
            source: RouteSource::Track {
                track_id,
                tap: SendTap::PostPan,
            },
            ..route.clone()
        });
    }
    model
}

/// The console's atomic bus add on [`console`]: a submix, its route to the output and one muted
/// post-fader route from every track into it.
fn bus_add(mut model: SessionModel) -> SessionModel {
    let output = model.routes[0].clone();
    model
        .submixes
        .push(Submix::unity(sid("bus"), &model.console));
    model.routes.push(Route {
        id: sid("bus-main"),
        source: RouteSource::Submix {
            submix_id: sid("bus"),
            tap: SendTap::PostPan,
        },
        ..output.clone()
    });
    let tracks: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
    for track_id in tracks {
        model.routes.push(Route {
            id: sid(&format!("{}-bus", track_id.as_str())),
            source: RouteSource::Track {
                track_id,
                tap: SendTap::PostFader,
            },
            destination: RouteDestination::SubmixInput {
                submix_id: sid("bus"),
            },
            mute: true,
            ..output.clone()
        });
    }
    model
}

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: stall_ring_frames(48_000, 128) + 4_096 + 128,
        maximum_tracks: u64::MAX,
        maximum_submixes: u64::MAX,
        maximum_vcas: 64,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: u64::MAX,
        maximum_routes: u64::MAX,
        maximum_effects: u64::MAX,
        maximum_graph_session_plus_plan_bytes: u64::MAX,
        maximum_source_total_bytes: u64::MAX,
        maximum_source_overhead_bytes: u64::MAX,
        maximum_effect_state_bytes: u64::MAX,
        maximum_effect_scratch_bytes: u64::MAX,
        maximum_builtin_retained_bytes: u64::MAX,
        maximum_named_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_meter_items: u64::MAX,
        maximum_meter_bytes: u64::MAX,
    }
}

fn compile(model: &SessionModel, caps: &HostPrepareCaps) -> CompiledSession {
    let compile_caps = caps
        .compile_caps(model.sources.len())
        .unwrap_or_else(|_| panic!("compile caps"));
    compile_host_model(model, compile_caps).unwrap_or_else(|failure| panic!("{failure:?}"))
}

/// Test value: red when a pass of `warm_lead` prepares the session's effects again (before #2232
/// each pass held every processor of the bus add, so its peak exceeded the whole projected effect
/// state); the `warm_successor` gates check only its answers, never what a pass allocates.
#[test]
fn warm_lead_on_the_console_bus_add_prepares_no_effect() {
    audited_allocator::assert_installed();
    let caps = caps();
    let a = compile(&console(TRACKS), &caps);
    let committed = a.normalized_model().clone();
    let b = compile(&bus_add(committed.clone()), &caps);
    let live = HostLiveControlRequest::default();
    let host_a: PreparedHost = prepare_host_runtime_with_live_controls(&a, &caps, &live)
        .unwrap_or_else(|failure| panic!("predecessor: {failure:?}"))
        .0;
    let base = SuccessorBase::new(&host_a.inventory, &committed, 2);
    let ordinary = prepare_host_runtime_with_live_controls_successor(&b, &caps, &live, base)
        .unwrap_or_else(|failure| panic!("ordinary successor: {failure:?}"))
        .0;
    let projected = project_effects(&b)
        .unwrap_or_else(|failure| panic!("projection: {failure:?}"))
        .state_bytes;
    let config = WarmConfig {
        p_max: 4_096,
        prime_bytes_max: u64::MAX,
    };
    audited_allocator::current_thread_peak_reset();
    let decision = warm_lead(&b, &caps, base, &ordinary.inventory, &config)
        .unwrap_or_else(|failure| panic!("warm_lead: {failure:?}"));
    let peak = audited_allocator::current_thread_peak().peak_bytes;
    eprintln!(
        "#2232 tracks={TRACKS} warm_lead peak={peak} projected_effect_state={projected} decision={decision:?}"
    );
    // The edit grows the output's arrival, so `warm_lead` reaches the lead's pass.
    assert!(
        !matches!(decision, host_core::WarmDecision::Ordinary),
        "{decision:?}"
    );
    assert!(
        peak < projected,
        "warm_lead peaked at {peak} bytes, over the session's {projected} projected effect-state bytes"
    );
}

/// Test value: red when `strip_transition_ceiling` leaves out a term the transition allocates: the
/// classifier's canonical documents, which hold most of it at 64 and 256 tracks, or the row diff's
/// owner buffers, which hold most of it at one track. Every strip restarts, so the step ducks them
/// all and reaches the classifier.
#[test]
fn the_transition_of_the_console_bus_add_stays_under_its_ceiling() {
    audited_allocator::assert_installed();
    let caps = caps();
    for tracks in [1, TRACKS, 4 * TRACKS] {
        let a = compile(&console(tracks), &caps);
        let committed = a.normalized_model();
        let b = compile(&bus_add(committed.clone()), &caps);
        let host_a: PreparedHost =
            prepare_host_runtime_with_live_controls(&a, &caps, &HostLiveControlRequest::default())
                .unwrap_or_else(|failure| panic!("predecessor: {failure:?}"))
                .0;
        let base = SuccessorBase::new(&host_a.inventory, committed, 2);
        let mut restarted: Vec<Box<str>> = committed
            .strips()
            .map(|strip| Box::from(strip.id.as_str()))
            .collect();
        restarted.sort_unstable();
        let ceiling = strip_transition_ceiling(&a);
        audited_allocator::current_thread_peak_reset();
        let transition =
            plan_strip_transition(&base, &DuckOverlay::new(), b.normalized_model(), &restarted)
                .unwrap_or_else(|failure| panic!("transition: {failure:?}"));
        let peak = audited_allocator::current_thread_peak().peak_bytes;
        assert_eq!(transition.strips().len(), tracks, "every strip ducks");
        eprintln!("#2234 tracks={tracks} transition peak={peak} ceiling={ceiling}");
        assert!(
            peak <= ceiling,
            "the transition peaked at {peak} bytes, over its {ceiling}-byte ceiling"
        );
    }
}
