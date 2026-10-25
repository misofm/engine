//! Issues #1078, #1079 and #1947: host preparation holds sessions above the former 65,536-track
//! boundary.
//!
//! Per PR (#1078): a plain 65,537-track session goes through every preparation stage before the
//! builtin compile and is refused only by the configured builtin cap.
//!
//! Release-only nightly row (#1079, #1947): the prepare compiles, lays out, binds and attaches
//! live controls to 65,537 tracks, each with its own source, a fader automation and a send into one
//! of 64 submixes; every 8th track carries a sidechained compressor, one VCA holds every track and
//! a second holds `track-9999`. Only the two rendered tracks' sends are open.

use std::time::{Duration, Instant};

use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes, HostPrepareCaps,
    HostShapePolicy, LiveEffectAddress, LiveValuesSuperseded, PrepareRejection, PreparedHost,
    SourceSubmission, compile_host_session, launch_maximum_observation_taps, prepare_host_runtime,
    prepare_host_runtime_with_live_lanes_and_spectrum, strip_position,
};
use session::{
    Automation, AutomationOffset, AutomationSegment, AutomationShape, ChannelMatrix, CompileCaps,
    Console, ControlAddress, DualMonoFader, Effect, EffectIdentity, EffectQuality, LinkMode, Owner,
    ParameterChannel, ParameterUnit, Route, RouteDestination, RouteSource, Section, SendTap,
    SessionModel, Sidechain, SidechainDeclaration, StableId, Submix, Vca, canonical_session_json,
    compile_session, parse_session_json,
};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

/// One past the former 65,536 boundary.
const TRACKS: u32 = 65_537;

/// Submixes the tracks send into, round robin.
const SUBMIXES: u32 = 64;

/// Every track `i` with `i % SIDECHAIN_EVERY == SIDECHAIN_PHASE` carries a compressor keyed by
/// track `i - 1`. The phase puts one on `track-9999`, the last track in ID order (index 65,536).
const SIDECHAIN_EVERY: u32 = 8;
const SIDECHAIN_PHASE: u32 = 7;

/// The two tracks the render feeds: `track-9999` (index 65,536) and its sidechain key.
const RENDERED: [u32; 2] = [9_998, 9_999];

/// The fader row (builtin parameter 5).
const FADER_DB: u32 = 5;

/// Render quantum of the fixture, and blocks rendered: past the fader automation's end (960).
const QUANTUM: usize = 128;
const BLOCKS: usize = 8;

/// The row's wall-clock bound, from the document build to the last rendered block: about 3.7 times
/// the 65,537-track time measured on 2026-10-25 (the #1947 spec's doubling table), so a path
/// quadratic in the track count fails it while a slower runner passes.
const BOUND: Duration = Duration::from_secs(100);

/// Every count and byte cap open, so only a fixed limit can refuse.
fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: u64::MAX,
        maximum_submixes: u64::MAX,
        maximum_vcas: u64::MAX,
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

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("generated stable ID")
}

fn route(id_text: &str, source: RouteSource, destination: RouteDestination) -> Route {
    Route {
        id: id(id_text),
        source,
        destination,
        channel_matrix: ChannelMatrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        },
        gain_db: 0.0,
        mute: false,
        follows_mute: false,
    }
}

fn post_fader(track: u32) -> RouteSource {
    RouteSource::Track {
        track_id: id(&format!("track-{track}")),
        tap: SendTap::PostFader,
    }
}

const fn sidechained(track: u32) -> bool {
    track % SIDECHAIN_EVERY == SIDECHAIN_PHASE
}

/// The kitchen-sink session of `tracks` (track numbers): the fixture's first track (with the
/// console's EQ slot) on its own source `src-{i}`, a two-segment fader automation, a post-fader
/// send into `bus-{i % 64}` (muted unless the track is in [`RENDERED`]), a compressor keyed by the
/// previous track's post-fader tap on every 8th track, one VCA over every track and one over
/// `track-9999`, and each submix routed to the output.
fn kitchen_sink_document(tracks: &[u32]) -> String {
    let mut model: SessionModel = parse_session_json(EXAMPLE).expect("fixture parses");
    let source = model.sources[0].clone();
    let template = model.tracks[0].clone();
    model.sources = tracks
        .iter()
        .map(|&index| {
            let mut source = source.clone();
            source.id = id(&format!("src-{index}"));
            source.content = format!("blake3:{index:064x}");
            source.frames = 4_800;
            source
        })
        .collect();
    model.tracks = tracks
        .iter()
        .map(|&index| {
            let mut track = template.clone();
            track.id = id(&format!("track-{index}"));
            track.source_id = id(&format!("src-{index}"));
            if sidechained(index) {
                track.inserts.effects = vec![Effect {
                    id: id("duck"),
                    identity: EffectIdentity::Native {
                        effect_id: id("miso.compressor"),
                    },
                    quality: EffectQuality::Normal,
                    bypass: false,
                    link_mode: LinkMode::Maximum,
                    params: Vec::new(),
                    sidechain: SidechainDeclaration::Routed(Sidechain {
                        source: post_fader(index - 1),
                        port_id: id("sidechain-in"),
                    }),
                }];
            }
            track
        })
        .collect();
    model.submixes = (0..SUBMIXES)
        .map(|bus| Submix::unity(id(&format!("bus-{bus}")), &model.console))
        .collect();
    model.vcas = vec![Vca {
        id: id("vca-all"),
        fader: DualMonoFader {
            left_db: -3.0,
            right_db: -3.0,
            left_mute: false,
            right_mute: false,
        },
        members: model.tracks.iter().map(|track| track.id.clone()).collect(),
    }];
    // ID order, so the member at index 65,536 is `track-9999`, the strip the render reads.
    model.vcas[0].members.sort();
    // A second VCA on `track-9999` alone, so the reach of the strip at index 65,536 (both VCAs)
    // differs from the reach of every other strip.
    if tracks.contains(&9_999) {
        model.vcas.push(Vca {
            id: id("vca-last"),
            fader: DualMonoFader {
                left_db: -1.5,
                right_db: -1.5,
                left_mute: false,
                right_mute: false,
            },
            members: vec![id("track-9999")],
        });
    }
    model.routes = tracks
        .iter()
        .map(|&index| {
            let mut send = route(
                &format!("send-{index}"),
                post_fader(index),
                RouteDestination::SubmixInput {
                    submix_id: id(&format!("bus-{}", index % SUBMIXES)),
                },
            );
            // Only the rendered pair's sends are open: a muted live send writes its
            // route-activity bit `false` every block, so a send index that aliases one of them
            // silences the pair.
            send.mute = !RENDERED.contains(&index);
            send
        })
        .chain((0..SUBMIXES).map(|bus| {
            route(
                &format!("bus-{bus}-main"),
                RouteSource::Submix {
                    submix_id: id(&format!("bus-{bus}")),
                    tap: SendTap::PostPan,
                },
                RouteDestination::OutputInput {
                    output_id: id("main-out"),
                },
            )
        }))
        .collect();
    let segment = |start_sample, end_sample, start_value, end_value| AutomationSegment {
        shape: AutomationShape::Linear,
        start_sample,
        end_sample,
        start_value,
        end_value,
        unit: ParameterUnit::Db,
    };
    model.automation = tracks
        .iter()
        .map(|&index| Automation {
            id: id(&format!("ride-{index}")),
            target: ControlAddress {
                owner: Owner::Strip(id(&format!("track-{index}"))),
                section: Section::Builtins,
                control: FADER_DB,
                lane: ParameterChannel::Both,
            },
            segments: vec![segment(0, 480, 0.0, -6.0), segment(480, 960, -6.0, 0.0)],
            offset: AutomationOffset::default(),
        })
        .collect();
    canonical_session_json(&model).expect("the kitchen-sink model is canonical")
}

/// `document` prepared as `capi`'s `CapiPreparer::prepare` prepares it: an observation slot for
/// every declared tap, the control plane's lanes with the effect lanes on, no spectrum, no
/// successor.
fn prepare(document: &str) -> (PreparedHost, HostLiveControlHandles) {
    let compiled = compile_host_session(document, &caps())
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    let live = HostLiveControlRequest {
        observation_taps: launch_maximum_observation_taps(),
        ..HostLiveControlRequest::default()
    };
    let lanes: HostLiveLanes = controls::live_lanes(compiled.normalized_model()).into();
    let lanes = HostLiveLanes {
        effects: true,
        ..lanes
    };
    let (prepared, handles, spectrum) = prepare_host_runtime_with_live_lanes_and_spectrum(
        &compiled,
        &caps(),
        &live,
        lanes,
        None,
        None,
        &LiveValuesSuperseded::new(),
    )
    .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    assert!(spectrum.is_none());
    (prepared, handles)
}

/// Deterministic noise in `[-0.5, 0.5)` at `frame` of stream `seed`.
fn signal(seed: u64, frame: usize) -> f32 {
    let mut state = (frame as u64 ^ (seed << 32)).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    state ^= state >> 33;
    state = state.wrapping_mul(0xff51_afd7_ed55_8ccd);
    state ^= state >> 29;
    (((state >> 40) as f32) / 16_777_216.0) - 0.5
}

/// The output bits of [`BLOCKS`] blocks with only `src-9998` (the key) and `src-9999` fed; every
/// other source underruns to silence.
fn render(prepared: &mut PreparedHost) -> Vec<u32> {
    let mut bits = Vec::new();
    for block in 0..BLOCKS {
        let base = block * QUANTUM;
        for (seed, source) in [(1, "src-9998"), (2, "src-9999")] {
            let plane: Vec<f32> = (0..QUANTUM)
                .map(|frame| signal(seed, base + frame))
                .collect();
            prepared
                .sources
                .submit(
                    source.as_bytes(),
                    SourceSubmission {
                        generation: 1,
                        start_frame: base as u64,
                        sample_rate_hz: prepared.report.sample_rate_hz,
                        planes: &[&plane, &plane],
                        frames: QUANTUM as u32,
                        end_of_region: false,
                    },
                )
                .expect("source block");
        }
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
        prepared
            .plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: base as u64,
                },
            )
            .expect("render");
        bits.extend(samples.iter().map(|value| value.to_bits()));
    }
    bits
}

/// With every cap open, the live-attached preparation accepts the kitchen-sink session and binds
/// every per-track structure in full: one source, meter, send and automated fader per track (and
/// a strip, meter and console EQ per submix), and one sidechained compressor, with its
/// observation, per 8th track. The strip at index 65,536 (the last track: tracks are in ID order,
/// so it is `track-9999`) is addressable, and so are its console EQ's and its compressor's
/// producers. That strip renders the bits it renders in a session of only itself and its key
/// track: its own source, its fader automation, its VCA offset and its sidechain key reach it.
#[test]
#[ignore = "release-mode scale gate; runs nightly"]
fn prepares_a_65_537_track_kitchen_sink_session_live_attached() {
    let tracks = TRACKS as usize;
    let submixes = SUBMIXES as usize;
    let sidechains = tracks / SIDECHAIN_EVERY as usize;
    let started = Instant::now();
    let numbers: Vec<u32> = (0..TRACKS).collect();
    let (mut prepared, mut handles) = prepare(&kitchen_sink_document(&numbers));

    assert_eq!(handles.track_count, tracks);
    assert_eq!(handles.strip_controls.len(), tracks + submixes);
    assert_eq!(prepared.sources.len(), tracks);
    assert_eq!(prepared.inventory.sources().len(), tracks);
    assert_eq!(handles.meters.len(), tracks + submixes);
    assert_eq!(prepared.inventory.meter_count(), tracks + submixes);
    assert_eq!(
        handles.route_controls.len(),
        tracks,
        "one live send per track"
    );
    assert_eq!(prepared.inventory.live_route_count(), tracks);
    assert_eq!(
        handles
            .strip_controls
            .iter()
            .filter(|strip| strip.retained_heap_bytes() != 0)
            .count(),
        tracks,
        "every track's fader carries its automation's offsets cells"
    );
    assert_eq!(
        handles.effect_controls.len(),
        tracks + submixes + sidechains
    );
    assert_eq!(handles.effect_observations.len(), sidechains);

    let last = tracks - 1;
    assert_eq!(
        strip_position(
            handles.strips.len(),
            handles.track_count,
            |index| &*handles.strips[index],
            "track-9999",
        ),
        Some(last),
        "the last track in ID order is at index 65,536"
    );
    assert_eq!(&*handles.strip_controls[last].track_id, "track-9999");
    assert!(
        handles
            .effect_control_mut("track-9999", LiveEffectAddress::console(0))
            .is_some(),
        "the console EQ's producer on the strip at index 65,536 resolves"
    );
    assert!(
        handles
            .effect_control_mut("track-9999", LiveEffectAddress::insert(0))
            .is_some(),
        "the compressor's producer on the strip at index 65,536 resolves"
    );

    let (mut pair, _) = prepare(&kitchen_sink_document(&RENDERED));
    let expected = render(&mut pair);
    assert!(
        expected.iter().any(|&bits| f32::from_bits(bits) != 0.0),
        "the pair renders"
    );
    assert!(
        render(&mut prepared) == expected,
        "the strip at index 65,536 renders as it does beside its key alone"
    );
    let elapsed = started.elapsed();
    assert!(
        elapsed < BOUND,
        "the kitchen-sink build, prepare and render took {elapsed:?}, over {BOUND:?}: a per-track \
         path has gone superlinear"
    );
}

/// Issue #1078: a plain 65,537-track session (the fixture's first track, no console slot, no
/// insert, no automation, one output route; all tracks share one source) compiles and goes through
/// host-core's count and shape checks and effective-value composition, then the builtin compiler's
/// per-strip domain preflight and resource plan, where the configured builtin cap
/// (`maximum_builtin_retained_bytes = 1`) refuses it before the builtin and graph compile. The
/// plain entry builds no meter or control requests; the source mapping is built but consumed only
/// after the cap. Debug x86 with CI's unified `control-provider,test-support` features: about
/// 4.5-6.6 s (2026-10-25, load about 10-11); AArch64 estimate 9-13 s. It runs per PR.
#[test]
fn prepares_65_537_tracks_until_the_configured_builtin_cap() {
    let started = Instant::now();
    let mut model: SessionModel = parse_session_json(EXAMPLE).expect("fixture parses");
    model.console = Console {
        pre_insert: Vec::new(),
        post_insert: Vec::new(),
    };
    let mut template = model.tracks[0].clone();
    template.console.clear();
    template.inserts.effects.clear();
    model.automation.clear();
    model.submixes.clear();
    model.vcas.clear();
    model.tracks = (0..TRACKS)
        .map(|index| {
            let mut track = template.clone();
            track.id = id(&format!("track-{index}"));
            track
        })
        .collect();
    model.routes = vec![route(
        "track-0-main",
        post_fader(0),
        RouteDestination::OutputInput {
            output_id: id("main-out"),
        },
    )];
    let compiled = compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("the 65,537-track session compiles");
    let failure = prepare_host_runtime(
        &compiled,
        &HostPrepareCaps {
            maximum_builtin_retained_bytes: 1,
            ..caps()
        },
    )
    .map(|_| ())
    .expect_err("the configured builtin cap refuses");
    eprintln!(
        "#1078 65,537-track prepare refused after {:?}",
        started.elapsed()
    );
    assert_eq!(
        failure.as_bytes(),
        b"builtin.resource.limit\t$.builtin_compile_caps\n",
        "{}",
        String::from_utf8_lossy(failure.as_bytes())
    );
    assert_eq!(failure.kind(), PrepareRejection::Builtin);
}
