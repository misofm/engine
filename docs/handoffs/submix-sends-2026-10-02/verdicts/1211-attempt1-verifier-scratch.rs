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
use builtins_compiler::{TrackControlRecord, TrackFaderRecord, TrackInputRecord};
use builtins::Matrix2x2;
use session::MatrixOrPan;
use host_core::prepare_host_runtime_between_render_calls;
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
    }
}

/// The 3-track, 1-submix session, with `bus`'s fader at `bus_fader_db`.
fn session_with(edit: &dyn Fn(&mut Submix)) -> String {
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
    edit(&mut bus);
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


fn prepare_mode(document: &str, serialized: bool) -> (PreparedHost, HostLiveControlHandles) {
    let compiled = compile_host_session(document, &caps()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let request = HostLiveControlRequest {
        control_queue_depth: Some(NonZeroUsize::new(8).expect("depth")),
        ..HostLiveControlRequest::default()
    };
    let r = if serialized { prepare_host_runtime_between_render_calls(&compiled, &caps(), &request) } else { prepare_host_runtime_with_live_controls(&compiled, &caps(), &request) };
    r.unwrap_or_else(|failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())))
}

fn drive(edit: &dyn Fn(&mut Submix), push: &dyn Fn(&mut HostLiveControlHandles), serialized: bool) {
    let mut live = prepare_mode(&session_with(&|_| {}), serialized);
    let mut twin = prepare_mode(&session_with(edit), serialized);
    let mut draw = Draw::new(0x1211);
    let mut plane = || -> Vec<f32> {
        (0..QUANTUM * BLOCKS).map(|_| { let s = draw.noise(0.5); if s == 0.0 { 0.25 } else { s } }).collect()
    };
    let feeds: Vec<[Vec<f32>; 2]> = TRACKS.iter().map(|_| [plane(), plane()]).collect();
    for block in 0..BLOCKS {
        if block == COMMANDED { push(&mut live.1); }
        let commanded = bits(&render(&mut live.0, &feeds, block));
        let reference = bits(&render(&mut twin.0, &feeds, block));
        if block < COMMANDED { assert_ne!(commanded, reference, "block {block} vacuous"); }
        else { assert_eq!(commanded, reference, "block {block}"); }
    }
}

#[test]
fn zz_bus_input_trim_live() {
    for serialized in [false, true] {
        drive(&|bus| { bus.builtins.left.trim_db = -6.0; bus.builtins.right.trim_db = -6.0; },
              &|h| h.strip_controls[3].input.try_push(TrackInputRecord::TrimDb { lanes: BuiltinLaneSelector::Both, db: -6.0, smoothing_samples: 0 }).expect("room"), serialized);
    }
}

#[test]
fn zz_bus_polarity_live() {
    for serialized in [false, true] {
        drive(&|bus| { bus.builtins.left.polarity_invert = true; },
              &|h| h.strip_controls[3].input.try_push(TrackInputRecord::PolarityInvert { lanes: BuiltinLaneSelector::Left, inverted: true, smoothing_samples: 0 }).expect("room"), serialized);
    }
}

#[test]
fn zz_bus_matrix_live() {
    for serialized in [false, true] {
        drive(&|bus| { bus.matrix_or_pan = MatrixOrPan::Matrix { ll: 0.5, lr: 0.25, rl: 0.0, rr: 0.75, smoothing_samples: 0 }; },
              &|h| h.strip_controls[3].producer.try_push(TrackControlRecord { matrix: Matrix2x2 { ll: 0.5, lr: 0.25, rl: 0.0, rr: 0.75 }, smoothing_samples: 0 }).expect("room"), serialized);
    }
}

#[test]
fn zz_bus_mute_live() {
    for serialized in [false, true] {
        drive(&|bus| { bus.fader.right_mute = true; },
              &|h| h.strip_controls[3].fader.try_push(TrackFaderRecord::Mute { lanes: BuiltinLaneSelector::Right, muted: true, smoothing_samples: 0 }).expect("room"), serialized);
    }
}

#[test]
fn zz_bus_fader_serialized() {
    drive(&|bus| { bus.fader.left_db = -6.0; bus.fader.right_db = -6.0; },
          &|h| h.strip_controls[3].fader.try_push(TrackFaderRecord::FaderDb { lanes: BuiltinLaneSelector::Both, db: -6.0, smoothing_samples: 0 }).expect("room"), true);
}

#[test]
fn zz_soloed_track_heard_through_solo_safe_bus() {
    use host_core::{LiveControlSoloState, StripMuteSeed};
    let mut live = prepare_mode(&session_with(&|_| {}), false);
    // Twin: t1 and t2 muted at preparation, bus untouched.
    let twin_doc = {
        let mut model = parse_session_json(&session_with(&|_| {})).unwrap();
        for t in model.tracks.iter_mut() { if t.id.as_str() != "t0" { t.fader.left_mute = true; t.fader.right_mute = true; } }
        canonical_session_json(&model).unwrap()
    };
    let mut twin = prepare_mode(&twin_doc, false);
    let seeds: Vec<StripMuteSeed> = (0..4).map(|s| StripMuteSeed { mutes: [false; 2], solo_safe: s >= live.1.track_count }).collect();
    let mut solo = LiveControlSoloState::try_new(&seeds).unwrap();
    let mut draw = Draw::new(0x1211);
    let mut plane = || -> Vec<f32> { (0..QUANTUM * BLOCKS).map(|_| { let s = draw.noise(0.5); if s == 0.0 { 0.25 } else { s } }).collect() };
    let feeds: Vec<[Vec<f32>; 2]> = TRACKS.iter().map(|_| [plane(), plane()]).collect();
    for block in 0..BLOCKS {
        if block == COMMANDED {
            assert!(solo.set_solo(0, true));
            for strip in 0..4 {
                for (lanes, muted) in solo.track_delta(strip).into_iter().flatten() {
                    live.1.strip_controls[strip].fader.try_push(TrackFaderRecord::Mute { lanes, muted, smoothing_samples: 0 }).unwrap();
                    solo.record_emitted(strip, lanes, muted);
                }
            }
            solo.commit();
        }
        let a = bits(&render(&mut live.0, &feeds, block));
        let b = bits(&render(&mut twin.0, &feeds, block));
        if block < COMMANDED { assert_ne!(a, b); } else {
            assert!(a.iter().any(|w| *w != 0 && *w != 0x8000_0000), "block {block}: silent output under solo");
            assert_eq!(a, b, "block {block}");
        }
    }
}

#[test]
fn zz_bus_records_drain_without_allocating() {
    use bench_support::alloc as bench_alloc;
    use engine::realtime::audit;
    for serialized in [false, true] {
        let mut live = prepare_mode(&session_with(&|_| {}), serialized);
        let mut draw = Draw::new(0x1211);
        let mut plane = || -> Vec<f32> { (0..QUANTUM * BLOCKS).map(|_| draw.noise(0.5)).collect() };
        let feeds: Vec<[Vec<f32>; 2]> = TRACKS.iter().map(|_| [plane(), plane()]).collect();
        audit::warm_up();
        bench_alloc::assert_installed();
        for block in 0..BLOCKS {
            let h = &mut live.1;
            let g = if block % 2 == 0 { -6.0 } else { -3.0 };
            h.strip_controls[3].fader.try_push(TrackFaderRecord::FaderDb { lanes: BuiltinLaneSelector::Both, db: g, smoothing_samples: 64 }).unwrap();
            h.strip_controls[3].fader.try_push(TrackFaderRecord::Mute { lanes: BuiltinLaneSelector::Left, muted: block % 2 == 1, smoothing_samples: 17 }).unwrap();
            h.strip_controls[3].input.try_push(TrackInputRecord::TrimDb { lanes: BuiltinLaneSelector::Right, db: g, smoothing_samples: 33 }).unwrap();
            h.strip_controls[3].input.try_push(TrackInputRecord::PolarityInvert { lanes: BuiltinLaneSelector::Left, inverted: block % 2 == 0, smoothing_samples: 5 }).unwrap();
            h.strip_controls[3].producer.try_push(TrackControlRecord { matrix: Matrix2x2 { ll: 0.5, lr: 0.25, rl: 0.1, rr: 0.75 }, smoothing_samples: 48 }).unwrap();
            let rate = live.0.report.sample_rate_hz;
            let range = block * QUANTUM..(block + 1) * QUANTUM;
            for (id, planes) in TRACKS.iter().zip(&feeds) {
                live.0.sources.submit(id.as_bytes(), SourceSubmission { generation: 1, start_frame: range.start as u64, sample_rate_hz: rate, planes: &[&planes[0][range.clone()], &planes[1][range.clone()]], frames: QUANTUM as u32, end_of_region: false }).unwrap();
            }
            let mut samples = vec![0.0_f32; QUANTUM * 2];
            let output = PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).unwrap();
            audit::reset();
            let mark = bench_alloc::current_thread_counters();
            let (r, snap) = audit::in_render_scope(|| { let r = live.0.plan.render(RenderIo { output }, RenderTime { absolute_sample: range.start as u64 }); (r, audit::snapshot()) });
            let d = bench_alloc::current_thread_delta_since(mark);
            assert!(r.is_ok());
            if block > 0 {
                assert_eq!((snap.allocations, snap.deallocations), (0, 0), "serialized={serialized} block {block} audit");
                assert_eq!((d.allocations, d.reallocations, d.deallocations), (0, 0, 0), "serialized={serialized} block {block}");
            }
        }
    }
}
