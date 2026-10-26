//! #1682 D1 and D2: a session with no source and no track prepares through the shared facade and
//! renders silence, with no special path for a host adapter.
//!
//! The browser and the C ABI both prepare through `prepare_host_runtime`, so this is the one core
//! gate of the empty boot; `crates/capi/tests/empty_boot.rs` drives the exported entries.

use bench_support::alloc as bench_alloc;
use engine::realtime::{PlanarBufferMut, RenderIo};
use host_core::{HostPrepareCaps, HostShapePolicy, prepare_host_session};
use session::{Route, RouteDestination, RouteSource, SendTap, SessionModel, StableId, Submix, Vca};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const QUANTUM: usize = 128;

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
    }
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

/// The fixture with no source, no track and no route; its console slot stays declared.
fn empty_model() -> SessionModel {
    let mut model = session::parse_session_json(FIXTURE).expect("fixture");
    model.sources.clear();
    model.tracks.clear();
    model.routes.clear();
    model
}

/// The empty model with one submix routed into the output and one VCA that holds the submix.
fn empty_model_with_a_bus() -> SessionModel {
    let mut model = empty_model();
    let submix = Submix::unity(id("bus"), &model.console);
    model.submixes.push(submix);
    model.routes.push(Route {
        id: id("bus-main"),
        source: RouteSource::Submix {
            submix_id: id("bus"),
            tap: SendTap::PostPan,
        },
        destination: RouteDestination::OutputInput {
            output_id: id("main-out"),
        },
        channel_matrix: session::ChannelMatrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        },
        gain_db: 0.0,
        mute: false,
        follows_mute: false,
    });
    model.vcas.push(Vca {
        id: id("master"),
        fader: session::DualMonoFader {
            left_db: -3.0,
            right_db: -3.0,
            left_mute: false,
            right_mute: false,
        },
        members: vec![id("bus")],
    });
    model
}

/// Prepares `model`, checks the report's counts and renders four blocks of `+0.0`, counting the
/// render thread's allocator calls.
fn boots_silent(model: &SessionModel, label: &str) {
    let document = session::canonical_session_json(model).expect("canonical session");
    let (_compiled, mut prepared) =
        prepare_host_session(&document, &caps()).unwrap_or_else(|failure| {
            panic!("{label}: {}", String::from_utf8_lossy(failure.as_bytes()))
        });
    assert_eq!(
        (
            prepared.report.source_count,
            prepared.report.track_count,
            prepared.sources.len()
        ),
        (0, 0, 0),
        "{label}: nothing to feed"
    );
    bench_alloc::assert_installed();
    for block in 0..4 {
        let mut samples = [f32::NAN; QUANTUM * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
        let mark = bench_alloc::current_thread_counters();
        prepared
            .plan
            .render_contiguous(RenderIo { output }, (block * QUANTUM) as u64)
            .expect("render");
        let moved = bench_alloc::current_thread_delta_since(mark);
        assert_eq!(
            (moved.allocations, moved.deallocations),
            (0, 0),
            "{label}: block {block} allocates nothing"
        );
        assert!(
            samples.iter().all(|sample| sample.to_bits() == 0),
            "{label}: block {block} is +0.0 silence"
        );
    }
}

/// Test value: red if the shared preparation (the source set, the graph, the buses or the console
/// slots) refuses or mis-sizes a session whose strip count is zero, or renders anything but
/// silence for it. The source graph set refused an empty source list before this slice, so every
/// host failed to boot an empty mixer.
#[test]
fn an_empty_session_prepares_and_renders_silence() {
    boots_silent(&empty_model(), "console slot only");
    let minimal = include_str!("../../../fixtures/session/v1/canonical-minimal.json");
    let model = session::parse_session_json(minimal).expect("minimal fixture");
    boots_silent(&model, "minimal");
}

/// Test value: red if declaring a submix and a VCA (no track feeds the bus) breaks the empty boot:
/// a bus with no input renders silence, and a VCA over it carries no audio.
#[test]
fn declared_submixes_and_vcas_do_not_make_an_empty_session_noisy() {
    boots_silent(&empty_model_with_a_bus(), "an unfed bus");
}
