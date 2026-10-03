//! Issue #1211 gate 2: a submix's builtin control producers drive the submix.
//!
//! Three transparent tracks route into one submix `bus` (no console slots, no inserts, an identity
//! input section), and `bus` routes to the output. One host is prepared with live controls and
//! told, between blocks, to set `bus`'s fader to -6 dB with no smoothing; its twin is prepared from
//! the same session with `bus`'s fader already at -6 dB. From the commanded block on, the two
//! render the same bits. The bus strip holds no state that the fader could have perturbed earlier,
//! so the comparison is exact.

use core::num::NonZeroUsize;

use builtins::BuiltinLaneSelector;
use builtins_compiler::TrackFaderRecord;
use dsp_reference::randomized::Draw;
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostPrepareCaps, HostShapePolicy, PreparedHost,
    SourceSubmission, compile_host_session, prepare_host_runtime_with_live_controls,
};
use session::{
    ChannelMatrix, Route, RouteDestination, RouteSource, SendTap, StableId, Submix,
    canonical_session_json, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const QUANTUM: usize = 128;
const BLOCKS: usize = 6;
/// The block before which the live arm's bus fader record is pushed.
const COMMANDED: usize = 2;
const TRACKS: [&str; 3] = ["t0", "t1", "t2"];

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
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

fn sid(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

fn route(id: &str, source: RouteSource, destination: RouteDestination) -> Route {
    Route {
        id: sid(id),
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

/// The 3-track, 1-submix session, with `bus`'s fader at `bus_fader_db`.
fn session(bus_fader_db: f32) -> String {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.quantum_frames = QUANTUM as u32;
    let mut source = model.sources.pop().expect("fixture source");
    source.frames = (QUANTUM * (BLOCKS + 8)) as u64;
    let mut track = model.tracks.swap_remove(0);
    model.tracks.clear();
    model.routes.clear();
    model.sources.clear();
    model.automation.clear();
    let unity = Submix::unity(sid("unused"), &model.console);
    track.builtins = unity.builtins;
    track.console = unity.console;
    track.inserts = unity.inserts;
    track.fader = unity.fader;
    track.matrix_or_pan = unity.matrix_or_pan;
    for id in TRACKS {
        let mut source = source.clone();
        source.id = sid(id);
        model.sources.push(source);
        let mut track = track.clone();
        track.id = sid(id);
        track.source_id = sid(id);
        model.tracks.push(track);
        model.routes.push(route(
            &format!("{id}-bus"),
            RouteSource::Track {
                track_id: sid(id),
                tap: SendTap::PostPan,
            },
            RouteDestination::SubmixInput {
                submix_id: sid("bus"),
            },
        ));
    }
    let mut bus = Submix::unity(sid("bus"), &model.console);
    bus.fader.left_db = bus_fader_db;
    bus.fader.right_db = bus_fader_db;
    model.submixes.push(bus);
    model.routes.push(route(
        "bus-main",
        RouteSource::Submix {
            submix_id: sid("bus"),
            tap: SendTap::PostPan,
        },
        RouteDestination::OutputInput {
            output_id: sid("main-out"),
        },
    ));
    canonical_session_json(&model).expect("the bus session canonicalizes")
}

fn prepare(document: &str) -> (PreparedHost, HostLiveControlHandles) {
    let compiled = compile_host_session(document, &caps()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let request = HostLiveControlRequest {
        control_queue_depth: Some(NonZeroUsize::new(8).expect("depth")),
        ..HostLiveControlRequest::default()
    };
    prepare_host_runtime_with_live_controls(&compiled, &caps(), &request).unwrap_or_else(
        |failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())),
    )
}

/// Submits block `block` of every track's feed and renders it; returns the planar output.
fn render(host: &mut PreparedHost, feeds: &[[Vec<f32>; 2]], block: usize) -> Vec<f32> {
    let rate = host.report.sample_rate_hz;
    let range = block * QUANTUM..(block + 1) * QUANTUM;
    for (id, planes) in TRACKS.iter().zip(feeds) {
        host.sources
            .submit(
                id.as_bytes(),
                SourceSubmission {
                    generation: 1,
                    start_frame: range.start as u64,
                    sample_rate_hz: rate,
                    planes: &[&planes[0][range.clone()], &planes[1][range.clone()]],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
    }
    let mut samples = vec![0.0_f32; QUANTUM * 2];
    let output = PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("planes");
    host.plan
        .render(
            RenderIo { output },
            RenderTime {
                absolute_sample: range.start as u64,
            },
        )
        .expect("render");
    samples
}

fn bits(samples: &[f32]) -> Vec<u32> {
    samples.iter().map(|sample| sample.to_bits()).collect()
}

/// Red if a submix gets no control request, if its producer is not parallel to its strip, or if
/// its fader record lands on a track's lane instead of the bus's.
#[test]
fn a_bus_fader_record_renders_as_the_session_with_that_fader() {
    let mut live = prepare(&session(0.0));
    let mut twin = prepare(&session(-6.0));

    let controls: Vec<&str> = live
        .1
        .strip_controls
        .iter()
        .map(|control| control.track_id.as_ref())
        .collect();
    assert_eq!(live.1.track_count, 3);
    assert_eq!(controls, ["t0", "t1", "t2", "bus"]);
    assert_eq!(live.1.strip_controls.len(), live.1.strips.len());

    let mut draw = Draw::new(0x1211);
    let mut plane = || -> Vec<f32> {
        (0..QUANTUM * BLOCKS)
            .map(|_| {
                let sample = draw.noise(0.5);
                if sample == 0.0 { 0.25 } else { sample }
            })
            .collect()
    };
    let feeds: Vec<[Vec<f32>; 2]> = TRACKS.iter().map(|_| [plane(), plane()]).collect();

    for block in 0..BLOCKS {
        if block == COMMANDED {
            live.1.strip_controls[3]
                .fader
                .try_push(TrackFaderRecord::FaderDb {
                    lanes: BuiltinLaneSelector::Both,
                    db: -6.0,
                    smoothing_samples: 0,
                })
                .expect("bounded queue room");
        }
        let commanded = bits(&render(&mut live.0, &feeds, block));
        let reference = bits(&render(&mut twin.0, &feeds, block));
        if block < COMMANDED {
            // Non-vacuous: before the record, the bus's fader is audibly the difference.
            assert_ne!(
                commanded, reference,
                "block {block}: the bus fader changes nothing"
            );
        } else {
            assert_eq!(commanded, reference, "block {block}");
        }
    }
}
