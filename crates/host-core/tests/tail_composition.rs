//! #1379 Amendment 1's real-kernel gates for the graph's tail composition through gain: C1 (gain
//! after a tail), C2 (gain before a tail), C6 (below the floor, every launch rate) and C11 (b) (a
//! live gain lane at its domain maximum). C5's host half (the ring keeps the output going for its
//! length after the input stops) is `track_delay.rs`'s shift-exactness eval P2-1, which this file
//! does not repeat.
//!
//! Each renders through the real host-core runtime. The values these check are recomputed by
//! enumeration in `graph-compiler`'s `tail_composition.rs` (C3). Release scale: ignored in debug.
//! Evidence lines are printed with `--nocapture`.

use builtins::BuiltinLaneSelector;
use builtins_compiler::{TestOnlyWriteGate, TrackFaderRecord};
use effect_contract::TailSamples;
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use graph_compiler::FlushFloor;
use host_core::{
    HostLiveControlRequest, HostLiveLanes, HostPrepareCaps, HostShapePolicy, LiveValuesSuperseded,
    PreparedHost, SourceSubmission, compile_host_session, prepare_host_runtime,
    prepare_host_runtime_with_live_lanes,
};
use math::tail::SvfWords;
use session::{
    ChannelMatrix, MatrixOrPan, Route, RouteDestination, RouteSource, SendTap, SessionModel,
    StableId, Submix, canonical_session_json, parse_session_json,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.json");
const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
/// `10^(-144/20)`.
const EPS: f64 = math::tail::TAIL_FLOOR;
/// The worst-sign input's length (C1, C2, C11).
const WORST_SIGN_FRAMES: usize = 1_000_000;

fn sid(value: &str) -> StableId {
    StableId::parse(value).expect("stable id")
}

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
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

const ONES: ChannelMatrix = ChannelMatrix {
    ll: 1.0,
    lr: 1.0,
    rl: 1.0,
    rr: 1.0,
};
const IDENTITY: ChannelMatrix = ChannelMatrix {
    ll: 1.0,
    lr: 0.0,
    rl: 0.0,
    rr: 1.0,
};

fn matrix(m: ChannelMatrix) -> MatrixOrPan {
    MatrixOrPan::Matrix {
        ll: m.ll,
        lr: m.lr,
        rl: m.rl,
        rr: m.rr,
        smoothing_samples: 0,
    }
}

fn route(
    id: &str,
    source: RouteSource,
    destination: RouteDestination,
    gain_db: f32,
    m: ChannelMatrix,
) -> Route {
    Route {
        id: sid(id),
        source,
        destination,
        channel_matrix: m,
        gain_db,
        mute: false,
        follows_mute: false,
    }
}

fn into_output() -> RouteDestination {
    RouteDestination::OutputInput {
        output_id: sid("main-out"),
    }
}

/// The fixture's one track at `rate`, no inserts, no automation and no routes, its input section
/// at `(hpf, lpf)` with trim `trim_db`, its fader at `fader_db` and its matrix `m`.
fn track(
    rate: u32,
    hpf: f32,
    lpf: f32,
    trim_db: f32,
    fader_db: f32,
    m: ChannelMatrix,
) -> SessionModel {
    let mut model = parse_session_json(SESSION).expect("fixture parses");
    model.sample_rate_hz = rate;
    model.sources[0].frames = 50_000_000;
    model.automation.clear();
    model.routes.clear();
    let track = &mut model.tracks[0];
    track.inserts.effects.clear();
    for lane in [&mut track.builtins.left, &mut track.builtins.right] {
        lane.hpf_hz = hpf;
        lane.lpf_hz = lpf;
        lane.trim_db = trim_db;
    }
    track.fader.left_db = fader_db;
    track.fader.right_db = fader_db;
    track.matrix_or_pan = matrix(m);
    model
}

fn worst_pair(rate: u32) -> (f32, f32) {
    builtins::input_section_worst_case_pair(rate).expect("a launch rate")
}

/// C1's session: the worst-case pair at 0 dB trim, then every gain at its maximum: fader +24 dB,
/// matrix ones, one route at +24 dB with ones to the output. `fader_db` is C11's knob.
fn c1(fader_db: f32) -> SessionModel {
    let (hpf, lpf) = worst_pair(48_000);
    let mut model = track(48_000, hpf, lpf, 0.0, fader_db, ONES);
    model.routes.push(route(
        "to-main",
        RouteSource::Track {
            track_id: sid("vocal"),
            tap: SendTap::PostPan,
        },
        into_output(),
        24.0,
        ONES,
    ));
    model
}

/// C2's session: the track with both filters off at trim +24 dB, every gain at its maximum, into
/// a submix whose input section is the worst-case pair at 0 dB trim, every gain at its maximum.
fn c2() -> SessionModel {
    let (hpf, lpf) = worst_pair(48_000);
    let mut model = track(48_000, 0.0, 0.0, 24.0, 24.0, ONES);
    let mut bus = Submix::unity(sid("bus"), &model.console);
    for lane in [&mut bus.builtins.left, &mut bus.builtins.right] {
        lane.hpf_hz = hpf;
        lane.lpf_hz = lpf;
    }
    bus.fader.left_db = 24.0;
    bus.fader.right_db = 24.0;
    bus.matrix_or_pan = matrix(ONES);
    model.submixes.push(bus);
    model.routes.push(route(
        "to-bus",
        RouteSource::Track {
            track_id: sid("vocal"),
            tap: SendTap::PostPan,
        },
        RouteDestination::SubmixInput {
            submix_id: sid("bus"),
        },
        24.0,
        ONES,
    ));
    model.routes.push(route(
        "bus-main",
        RouteSource::Submix {
            submix_id: sid("bus"),
            tap: SendTap::PostPan,
        },
        into_output(),
        24.0,
        ONES,
    ));
    model
}

/// C6's first design (Amendment 1 H5): the 1 kHz input LPF, HPF off, trim 0 dB, every downstream
/// gain at 0 dB (fader, identity matrix, one 0 dB identity route).
fn c6(rate: u32) -> SessionModel {
    let mut model = track(rate, 0.0, 1_000.0, 0.0, 0.0, IDENTITY);
    model.routes.push(route(
        "to-main",
        RouteSource::Track {
            track_id: sid("vocal"),
            tap: SendTap::PostPan,
        },
        into_output(),
        0.0,
        IDENTITY,
    ));
    model
}

fn prepare(model: &SessionModel) -> PreparedHost {
    let document = canonical_session_json(model).expect("canonical");
    let compiled = compile_host_session(&document, &caps()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    prepare_host_runtime(&compiled, &caps()).unwrap_or_else(|failure| {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    })
}

/// The graph's three values for `model` under the gain selection `liveness` (no input lane).
fn graph_values(
    model: &SessionModel,
    liveness: graph_compiler::GainLiveness,
) -> (u64, u64, FlushFloor) {
    let document = canonical_session_json(model).expect("canonical");
    let compiled = compile_host_session(&document, &caps()).expect("compiles");
    let builtins = builtins_compiler::prepare_session_builtins(
        &compiled,
        &[],
        builtins_compiler::BuiltinCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_total_retained_payload_bytes: u64::MAX,
            maximum_total_meter_items: u64::MAX,
            maximum_total_meter_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        },
    )
    .expect("builtins");
    let effects = effect_compiler::prepare_native_session_effects(
        &compiled,
        effect_compiler::launch_native_effect_registry().expect("registry"),
        effect_compiler::EffectCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_scratch_bytes: u64::MAX,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("effects");
    let artifact = graph_compiler::GraphCompiler::compile_with_builtins(
        graph_compiler::GraphBuiltinsCompileRequest {
            plan_id: 1,
            effects,
            builtins,
            caps: graph::GraphCompileCaps {
                maximum_nodes: u64::MAX,
                maximum_edges: u64::MAX,
                maximum_schedule_items: u64::MAX,
                maximum_dependency_levels: u64::MAX,
                maximum_audio_buffer_samples: u64::MAX,
                maximum_delay_samples_per_edge: u64::MAX,
                maximum_total_delay_samples: u64::MAX,
                maximum_graph_bytes: u64::MAX,
                maximum_plan_bytes: u64::MAX,
                maximum_single_allocation_bytes: u64::MAX,
                maximum_finite_tail_samples: u64::MAX,
            },
            dispatch: graph_compiler::Backend::current(),
            input_floors: Default::default(),
            gain_liveness: liveness,
        },
    )
    .unwrap_or_else(|_| panic!("graph compile"));
    let report = artifact.report();
    (
        finite(report.output_tail),
        finite(report.output_tail_every_peak),
        report.output_flush_floor,
    )
}

fn finite(tail: TailSamples) -> u64 {
    match tail {
        TailSamples::Finite(samples) => samples,
        TailSamples::Infinite => panic!("a finite tail"),
    }
}

/// Renders `input` (both source channels identical), then zeros, for `frames` frames in all, and
/// returns the two output planes. `before_block` runs before each block's render.
fn render(
    host: &mut PreparedHost,
    input: &[f32],
    frames: usize,
    mut before_block: impl FnMut(&mut PreparedHost, usize),
) -> [Vec<f32>; 2] {
    let quantum = host.report.quantum_frames as usize;
    let rate = host.report.sample_rate_hz;
    let blocks = frames.div_ceil(quantum);
    let mut out = [
        Vec::with_capacity(blocks * quantum),
        Vec::with_capacity(blocks * quantum),
    ];
    let mut plane = vec![0.0_f32; quantum];
    let mut samples = vec![0.0_f32; quantum * 2];
    for block in 0..blocks {
        let start = block * quantum;
        for (index, sample) in plane.iter_mut().enumerate() {
            *sample = input.get(start + index).copied().unwrap_or(0.0);
        }
        host.sources
            .submit(
                b"voice",
                SourceSubmission {
                    generation: 1,
                    start_frame: start as u64,
                    sample_rate_hz: rate,
                    planes: &[&plane, &plane],
                    frames: quantum as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        before_block(host, block);
        samples.fill(0.0);
        let output = PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("planes");
        host.plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: start as u64,
                },
            )
            .expect("render");
        out[0].extend_from_slice(&samples[..quantum]);
        out[1].extend_from_slice(&samples[quantum..]);
    }
    out
}

/// The designed `f32` words of one section, as `f64`.
fn section(rate: u32, cutoff: f32, high_pass: bool) -> SvfWords {
    let [c1, a2, a3, _k, m0, m1, m2] =
        builtins::test_support::section_words(rate, cutoff, high_pass)
            .expect("design")
            .map(f32::from_bits);
    SvfWords::from_f32([c1, a2, a3, m0, m1, m2])
}

/// The designed cascade's exact-arithmetic impulse response, in `f64`.
fn impulse_response(sections: &[SvfWords], frames: usize) -> Vec<f64> {
    let mut state = vec![[0.0_f64; 2]; sections.len()];
    let mut out = vec![0.0_f64; frames];
    for (frame, value) in out.iter_mut().enumerate() {
        let mut x = if frame == 0 { 1.0 } else { 0.0 };
        for (words, state) in sections.iter().zip(state.iter_mut()) {
            let v3 = x - state[1];
            let d1 = -words.c1 * state[0] + words.a2 * v3;
            let d2 = words.a3 * v3 + words.a2 * state[0];
            let v1 = state[0] + d1;
            let v2 = state[1] + d2;
            state[0] += 2.0 * d1;
            state[1] += 2.0 * d2;
            x = words.m0 * x + words.m1 * v1 + words.m2 * v2;
        }
        *value = x;
    }
    out
}

/// `x[N - 1 - i] = P sign(h[tau + 1 + i])` for `i < frames`, `N = frames`: the input whose output
/// at `N + tau` is the largest the cascade can give there.
fn worst_sign(response: &[f64], tau: usize, frames: usize, peak: f32) -> Vec<f32> {
    let mut input = vec![0.0_f32; frames];
    for i in 0..frames {
        let h = response[tau + 1 + i];
        input[frames - 1 - i] = if h < 0.0 { -peak } else { peak };
    }
    input
}

/// The largest output magnitude from frame `from` on, on either plane.
fn peak_from(planes: &[Vec<f32>; 2], from: usize) -> f64 {
    planes
        .iter()
        .flat_map(|plane| plane[from.min(plane.len())..].iter())
        .map(|sample| f64::from(sample.abs()))
        .fold(0.0, f64::max)
}

/// One worst-sign render at peak 1 aimed at `N + output_tail`, through `host` prepared without a
/// control queue; asserts the host reports the graph's every-peak value and every output sample
/// from `N + output_tail` on is below `eps`. `sections` is the cascade the input is signed for.
fn worst_sign_holds(what: &str, model: &SessionModel, sections: &[SvfWords]) {
    let (tail, every_peak, floor) = graph_values(model, graph_compiler::GainLiveness::FIXED);
    let mut host = prepare(model);
    assert_eq!(
        host.report.output_tail_every_peak,
        TailSamples::Finite(every_peak),
        "{what}: the host reports the graph's every-peak value"
    );
    assert!(
        matches!(floor, FlushFloor::Millibels(mb) if mb <= 0),
        "{what}: P = 1 is at or above the floor {floor:?}"
    );
    let response = impulse_response(sections, tail as usize + 2 + WORST_SIGN_FRAMES);
    let input = worst_sign(&response, tail as usize, WORST_SIGN_FRAMES, 1.0);
    let planes = render(
        &mut host,
        &input,
        WORST_SIGN_FRAMES + every_peak as usize + 1,
        |_, _| {},
    );
    let peak = peak_from(&planes, WORST_SIGN_FRAMES + tail as usize);
    let margin = 20.0 * math::log10(peak / EPS);
    eprintln!(
        "{what}: output_tail {tail}, output_tail_every_peak {every_peak}, floor {floor:?}, \
         peak from N + output_tail {peak:e} = {margin:+.1} dB re P eps"
    );
    assert!(peak < EPS, "{what}: {margin:+.1} dB over P eps");
}

/// C1. Red for a composition that ignores the gain between nodes (the reported tail is then the
/// input section's alone, and the worst-case output beats it by about 40 dB).
#[test]
#[cfg_attr(debug_assertions, ignore = "release scale")]
fn gain_after_a_tail_is_composed() {
    let (hpf, lpf) = worst_pair(48_000);
    let sections = [section(48_000, hpf, true), section(48_000, lpf, false)];
    worst_sign_holds("C1", &c1(24.0), &sections);
}

/// C2. Red for a rule that counts only the gain after the tail node.
#[test]
#[cfg_attr(debug_assertions, ignore = "release scale")]
fn gain_before_a_tail_is_composed() {
    let (hpf, lpf) = worst_pair(48_000);
    let sections = [section(48_000, hpf, true), section(48_000, lpf, false)];
    worst_sign_holds("C2", &c2(), &sections);
}

/// C6 (ruling (d)), every launch rate. Below the floor every output sample from
/// `N + output_tail_every_peak` on is exactly `+-0.0` for DC, worst-sign, random and alternating
/// inputs of peak `P*_graph / 2`, and for DC or the worst-sign input the output is still non-zero
/// at or after `N + output_tail` (so the rest branch is what the gate exercises). Red for an
/// every-peak value that drops the rest branch.
#[test]
#[cfg_attr(debug_assertions, ignore = "release scale")]
fn below_the_floor_the_output_is_exact_zero_from_the_every_peak_tail() {
    const FRAMES: usize = 100_000;
    for rate in LAUNCH_RATES {
        let model = c6(rate);
        let (tail, every_peak, floor) = graph_values(&model, graph_compiler::GainLiveness::FIXED);
        let FlushFloor::Millibels(mb) = floor else {
            panic!("C6 at {rate} Hz: a finite floor, got {floor:?}");
        };
        let peak = (math::exp(f64::from(mb) * core::f64::consts::LN_10 / 2000.0) / 2.0) as f32;
        let response =
            impulse_response(&[section(rate, 1_000.0, false)], tail as usize + 2 + FRAMES);
        let mut draw = dsp_reference::randomized::Draw::new(0x1379);
        let patterns: [(&str, Vec<f32>); 4] = [
            ("dc", vec![peak; FRAMES]),
            (
                "worst-sign",
                worst_sign(&response, tail as usize, FRAMES, peak),
            ),
            (
                "random",
                (0..FRAMES).map(|_| draw.noise(1.0) * peak).collect(),
            ),
            (
                "alternating",
                (0..FRAMES)
                    .map(|i| if i % 2 == 0 { peak } else { -peak })
                    .collect(),
            ),
        ];
        let mut lasts = Vec::new();
        for (name, input) in patterns {
            let mut host = prepare(&model);
            assert_eq!(
                host.report.output_tail_every_peak,
                TailSamples::Finite(every_peak)
            );
            let planes = render(
                &mut host,
                &input,
                FRAMES + every_peak as usize + 4_096,
                |_, _| {},
            );
            let last = planes
                .iter()
                .filter_map(|plane| plane.iter().rposition(|sample| *sample != 0.0))
                .max();
            for plane in &planes {
                assert!(
                    plane[FRAMES + every_peak as usize..]
                        .iter()
                        .all(|sample| *sample == 0.0),
                    "C6 at {rate} Hz, {name}: exact zero from N + output_tail_every_peak"
                );
            }
            if matches!(name, "dc" | "worst-sign") {
                lasts.push(last.map(|last| last as i64 - FRAMES as i64));
            }
            eprintln!(
                "C6 at {rate} Hz, {name}: P = {peak:e}, output_tail {tail}, \
                 output_tail_every_peak {every_peak}, floor {mb} mB, last non-zero output at N{:+}",
                last.map_or(i64::MIN, |last| last as i64 - FRAMES as i64)
            );
        }
        assert!(
            lasts.iter().flatten().any(|&last| last >= tail as i64),
            "C6 at {rate} Hz: the design meets (ii) (the decay value alone is short below the floor)"
        );
    }
}

/// C11 (b): the fader is prepared at -24 dB, moved live to +24 dB, and its ramp completes before
/// the worst-sign input starts; every output sample from `N + output_tail` on stays below `P eps`.
/// Red for a rule that states a live lane's prepared value.
#[test]
#[cfg_attr(debug_assertions, ignore = "release scale")]
fn a_live_fader_moved_to_its_maximum_is_covered() {
    let (hpf, lpf) = worst_pair(48_000);
    let sections = [section(48_000, hpf, true), section(48_000, lpf, false)];
    let model = c1(-24.0);
    let document = canonical_session_json(&model).expect("canonical");
    let compiled = compile_host_session(&document, &caps()).expect("compiles");
    let (mut host, mut handles) = prepare_host_runtime_with_live_lanes(
        &compiled,
        &caps(),
        &HostLiveControlRequest::default(),
        HostLiveLanes::FADER_AND_MATRIX,
        &LiveValuesSuperseded::default(),
    )
    .unwrap_or_else(|failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())));
    // The decay value the live plan states: the fader and matrix at their domain maxima.
    let (tail, every_peak, _) = graph_values(
        &model,
        graph_compiler::GainLiveness {
            strip_fader_matrix: true,
            routes: false,
        },
    );
    assert_eq!(
        host.report.output_tail_every_peak,
        TailSamples::Finite(every_peak)
    );
    let mut gate = host
        .plan
        .take_revision_gate()
        .expect("the plan's gate handle");
    gate.write_published(|stamp| {
        handles.strip_controls[0].fader.write(
            stamp,
            TrackFaderRecord::FaderDb {
                lanes: BuiltinLaneSelector::Both,
                db: 24.0,
                smoothing_samples: 0,
            },
        );
    });
    // Lead-in silence so the move settles before the input, then the worst-sign input aimed at
    // `N + output_tail`.
    let lead = 4_096;
    let response = impulse_response(&sections, tail as usize + 2 + WORST_SIGN_FRAMES);
    let mut input = vec![0.0_f32; lead];
    input.extend(worst_sign(&response, tail as usize, WORST_SIGN_FRAMES, 1.0));
    let n = lead + WORST_SIGN_FRAMES;
    let planes = render(&mut host, &input, n + tail as usize + 1, |_, _| {});
    let peak = peak_from(&planes, n + tail as usize);
    eprintln!(
        "C11 (b): output_tail {tail}, peak from N + output_tail {peak:e} = {:+.1} dB re P eps",
        20.0 * math::log10(peak / EPS)
    );
    assert!(peak < EPS);
}
