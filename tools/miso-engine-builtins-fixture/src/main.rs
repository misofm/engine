//! Deterministic issue-007 expected-output fixture generator and checker.

use core::fmt::Write as _;
use std::{env, fs, path::Path};

use miso_engine_builtins::{
    BuiltinChain, BuiltinParameterError, BuiltinParameters, BuiltinResetKind, ChannelParameters,
    DualMonoBlock, Matrix2x2, MeterAccumulator, MeterConfig, MeterHandle, MeterTap,
};
use miso_engine_builtins_compiler::{BuiltinCompileCaps, MeterRequest, prepare_session_builtins};
use miso_engine_core::realtime::{PlanarBufferMut, RenderError, RenderIo, RenderTime};
use miso_engine_dsp_reference::{ReferenceFilterKind, rbj_butterworth_magnitude_db};
use miso_engine_effect_compiler::EffectPreparedSession;
use miso_engine_graph::{
    GraphBindingBlock, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, GraphRuntimeProcessor,
    TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use miso_engine_session::{
    CompileCaps, RouteSource, SendTap, StableId, compile_session, parse_session_toml,
};
use sha2::{Digest, Sha256};

const MANIFEST_HEADER: &str = "path\tlength\tsha256\n";
const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const QUANTA: [u32; 5] = [1, 127, 128, 255, 1_024];

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(arguments: Vec<String>) -> Result<(), String> {
    match arguments.as_slice() {
        [mode, root] if mode == "--write" => write_and_verify(Path::new(root)),
        [mode, root] if mode == "--check" => verify(Path::new(root), &generated()),
        _ => {
            Err("usage: miso_engine_builtins_fixture --write|--check SCRATCH_DIRECTORY".to_owned())
        }
    }
}

fn generated() -> Vec<(String, Vec<u8>)> {
    let (graph_pcm, graph_meters) = graph_tap_fixtures();
    let mut files = vec![
        ("cases.toml".to_owned(), cases().into_bytes()),
        ("diagnostics.jsonl".to_owned(), diagnostics().into_bytes()),
        ("metadata.toml".to_owned(), metadata().into_bytes()),
        (
            "meters/window-and-drop.jsonl".to_owned(),
            meters().into_bytes(),
        ),
        (
            "reference/filter-response.csv".to_owned(),
            responses().into_bytes(),
        ),
        (
            "meters/graph-taps.jsonl".to_owned(),
            graph_meters.into_bytes(),
        ),
        ("pcm/graph-taps.f32le".to_owned(), graph_pcm),
        ("resources.jsonl".to_owned(), resources().into_bytes()),
    ];
    for (id, pcm) in pcm_cases() {
        files.push((format!("pcm/{id}.f32le"), pcm));
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files
}

struct FixtureSource;
impl GraphRuntimeProcessor for FixtureSource {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        for (index, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            *left = 0.125 + index as f32 * 0.001;
            *right = -0.25 - index as f32 * 0.002;
        }
        Ok(())
    }
}

struct FixtureIdentity;
impl GraphRuntimeProcessor for FixtureIdentity {
    fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        Ok(())
    }
}

fn graph_tap_fixtures() -> (Vec<u8>, String) {
    let mut model = parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
        .expect("fixture session");
    model.tracks[0].simd1.effects.clear();
    model.tracks[0].dynamic.effects.clear();
    model.tracks[0].simd2.effects.clear();
    model.automation.clear();
    let session = compile_session(
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
    .expect("compile session");
    let config = MeterConfig {
        period_frames: core::num::NonZeroU32::new(session.quantum().0).expect("quantum"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: core::num::NonZeroUsize::new(4).expect("capacity"),
        reset_generation: 0,
    };
    let requests: Vec<_> = [
        MeterTap::Input,
        MeterTap::PostInputBuiltins,
        MeterTap::PostSimd1,
        MeterTap::PostDynamic,
        MeterTap::PostSimd2PreFader,
        MeterTap::PostFader,
        MeterTap::PostMatrix,
    ]
    .into_iter()
    .map(|tap| MeterRequest {
        track_id: "vocal".to_owned(),
        tap,
        config,
    })
    .collect();
    let builtins = prepare_session_builtins(
        &session,
        &requests,
        BuiltinCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_total_meter_items: u64::MAX,
            maximum_total_meter_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        },
    )
    .expect("prepare builtins");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        plan_id: 7,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        builtins,
        caps: miso_engine_graph::GraphCompileCaps {
            maximum_nodes: 10_000,
            maximum_edges: 10_000,
            maximum_schedule_items: 10_000,
            maximum_dependency_levels: 10_000,
            maximum_audio_buffer_samples: 10_000_000,
            maximum_delay_samples_per_edge: 1_000_000,
            maximum_total_delay_samples: 10_000_000,
            maximum_graph_bytes: 10_000_000,
            maximum_plan_bytes: 100_000_000,
            maximum_single_allocation_bytes: 10_000_000,
            maximum_finite_tail_samples: 10_000_000,
        },
    })
    .unwrap_or_else(|_| panic!("compile graph"));
    let envelope = artifact.graph.envelope;
    let bindings = artifact
        .graph
        .required_bindings
        .iter()
        .cloned()
        .map(|node| {
            let processor: Box<dyn GraphRuntimeProcessor> = matches!(
                node,
                GraphNodeId::TrackStage {
                    stage: TrackStage::Input,
                    ..
                }
            )
            .then(|| Box::new(FixtureSource) as Box<dyn GraphRuntimeProcessor>)
            .unwrap_or_else(|| Box::new(FixtureIdentity));
            GraphNodeBinding::new(node, processor)
        })
        .collect();
    let mut plan = artifact
        .graph
        .bind(GraphRuntimeBindings {
            envelope,
            nodes: bindings,
        })
        .unwrap_or_else(|_| panic!("bind graph"));
    let frames = envelope.quantum.0 as usize;
    let mut pcm = vec![0.0_f32; frames * 2];
    plan.render(
        RenderIo {
            input: None,
            output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
        },
        RenderTime { absolute_sample: 0 },
    )
    .expect("render graph");
    let mut records = Vec::new();
    for mut consumer in artifact.meter_consumers {
        let snapshot = consumer
            .consumer
            .try_pop()
            .expect("one window per graph tap");
        records.push(format!(
            "{{\"tap\":\"{:?}\",\"snapshot\":{}}}",
            consumer.tap,
            meter_snapshot_json("graph-taps", snapshot)
        ));
    }
    records.sort();
    (
        pcm.into_iter().flat_map(f32::to_le_bytes).collect(),
        records.join("\n") + "\n",
    )
}

fn metadata() -> String {
    concat!(
        "fixture_schema = 1\n",
        "producer = \"miso-engine-builtins-fixture\"\n",
        "production_pcm = \"miso-engine-builtins scalar f32, planar L then R\"\n",
        "independent_response_oracle = \"miso-engine-dsp-reference::rbj_butterworth_magnitude_db\"\n",
        "oracle_dependency_rule = \"miso-engine-dsp-reference has no production-builtin dependency\"\n",
        "launch_rates_hz = [44100, 48000, 88200, 96000]\n",
        "quanta_frames = [1, 127, 128, 255, 1024]\n"
    )
    .to_owned()
}

fn cases() -> String {
    let mut entries = Vec::new();
    for rate in RATES {
        for quantum in QUANTA {
            for (section, _) in response_sections() {
                for (cutoff_index, cutoff) in response_cutoffs(rate).into_iter().enumerate() {
                    for (probe_index, probe) in probes(rate, cutoff).into_iter().enumerate() {
                        entries.push((
                            format!("response-{section}-{rate}-{quantum}-{cutoff_index}-{probe_index}"),
                            format!(
                                "category = \"filter_response\"\nrate_hz = {rate}\nquantum_frames = {quantum}\nsection = \"{section}\"\ncutoff_hz = {cutoff:.17}\nprobe_hz = {probe:.17}\noracle = \"rbj_f64_and_cast_state\"\n"
                            ),
                        ));
                    }
                }
            }
        }
    }
    for (id, category, detail) in [
        (
            "identity-signed-zero",
            "pcm",
            "planar_l_then_r; signed_zero",
        ),
        ("polarity-gain", "pcm", "per_lane_polarity_trim_fader"),
        ("mute", "pcm", "per_lane_mute"),
        ("filters-asymmetric", "pcm", "left_hpf_right_lpf"),
        ("matrix-corners", "pcm", "all_16_binary_2x2_matrices"),
        (
            "matrix-ramp",
            "pcm",
            "updates=0,1,2,127,128,u32_max_bounded_prefix",
        ),
        ("matrix-retarget", "pcm", "mid_ramp_target_replacement"),
        ("reset", "pcm", "discontinuity_and_full_reset"),
        ("lr-isolation", "pcm", "independent_filter_state"),
        ("partition", "pcm", "all_declared_block_splits"),
        ("graph-taps", "graph", "seven_taps_and_output_pcm"),
        ("meter-partial", "meter", "incomplete_window"),
        ("meter-multiple", "meter", "multiple_windows"),
        ("meter-wrap", "meter", "ring_wrap_with_interleaved_drain"),
        ("meter-full-drop", "meter", "full_queue_and_loss_counter"),
        ("meter-drain", "meter", "consumer_drain"),
        ("meter-discontinuity", "meter", "noncontiguous_sample_time"),
        ("meter-reset", "meter", "both_reset_modes"),
        ("meter-overflow", "meter", "sample_time_overflow"),
        ("meter-sanitization", "meter", "nan_and_infinity"),
        ("diagnostics", "diagnostic", "exact_code_and_path_tuples"),
        (
            "resource-grid",
            "resource",
            "tracks=1,4,65537; meters=0,1,7",
        ),
    ] {
        entries.push((id.to_owned(), format!("category = \"{category}\"\nrate_hz = 48000\nquantum_frames = 128\ndetail = \"{detail}\"\n")));
    }
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    let mut output = String::from("fixture_schema = 1\n\n");
    for (id, body) in entries {
        writeln!(output, "[[case]]\nid = \"{id}\"\n{body}").expect("string");
    }
    output
}

fn response_sections() -> [(&'static str, ReferenceFilterKind); 3] {
    [
        ("high_pass", ReferenceFilterKind::HighPass),
        ("low_pass", ReferenceFilterKind::LowPass),
        ("cascade", ReferenceFilterKind::LowPass),
    ]
}

fn response_cutoffs(rate: u32) -> [f64; 6] {
    [
        10.0,
        20.0,
        100.0,
        1000.0,
        20_000.0_f64.min(0.1 * f64::from(rate)),
        0.45 * f64::from(rate),
    ]
}

fn responses() -> String {
    let mut output = String::from(
        "case,rate_hz,section,cutoff_hz,probe_hz,quantum_frames,rbj_magnitude_db,cast_state_magnitude_db,impulse_dft_magnitude_db,sustained_fundamental_db,sustained_residual_db,sustained_total_db,tail_energy,recovery_count\n",
    );
    for rate in RATES {
        for (section, kind) in response_sections() {
            for (cutoff_index, cutoff) in response_cutoffs(rate).into_iter().enumerate() {
                for (probe_index, probe) in probes(rate, cutoff).into_iter().enumerate() {
                    let rbj = if section == "cascade" {
                        let (hpf, lpf) = cascade_cutoffs(rate, cutoff);
                        rbj_butterworth_magnitude_db(
                            f64::from(rate),
                            hpf,
                            ReferenceFilterKind::HighPass,
                            probe,
                        )
                        .expect("HPF oracle")
                            + rbj_butterworth_magnitude_db(
                                f64::from(rate),
                                lpf,
                                ReferenceFilterKind::LowPass,
                                probe,
                            )
                            .expect("LPF oracle")
                    } else {
                        rbj_butterworth_magnitude_db(f64::from(rate), cutoff, kind, probe)
                            .expect("oracle")
                    };
                    for quantum in QUANTA {
                        let measurement = measure_response(rate, section, cutoff, probe, quantum);
                        writeln!(output, "response-{section}-{rate}-{quantum}-{cutoff_index}-{probe_index},{rate},{section},{cutoff:.17},{probe:.17},{quantum},{rbj:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{}", measurement.0, measurement.1, measurement.2, measurement.3, measurement.4, measurement.5, measurement.6).expect("string");
                    }
                }
            }
        }
    }
    output
}

fn measure_response(
    rate: u32,
    section: &str,
    cutoff: f64,
    probe: f64,
    quantum: u32,
) -> (f64, f64, f64, f64, f64, f64, u64) {
    let mut parameters = BuiltinParameters::default();
    match section {
        "high_pass" => {
            parameters.left.hpf_hz = cutoff as f32;
            parameters.right.hpf_hz = cutoff as f32;
        }
        "low_pass" => {
            parameters.left.lpf_hz = cutoff as f32;
            parameters.right.lpf_hz = cutoff as f32;
        }
        "cascade" => {
            let (hpf, lpf) = cascade_cutoffs(rate, cutoff);
            parameters.left.hpf_hz = hpf as f32;
            parameters.left.lpf_hz = lpf as f32;
            parameters.right.hpf_hz = hpf as f32;
            parameters.right.lpf_hz = lpf as f32;
        }
        _ => panic!("unknown response section"),
    }
    let mut chain = BuiltinChain::new(rate, parameters).expect("fixture filter");
    let mut impulse = Vec::with_capacity(rate as usize);
    let mut left = vec![0.0_f32; quantum as usize];
    let mut right = vec![0.0_f32; quantum as usize];
    let mut recoveries = 0_u64;
    for start in (0..rate as usize).step_by(quantum as usize) {
        let frames = (rate as usize - start).min(quantum as usize);
        left[..frames].fill(0.0);
        right[..frames].fill(0.0);
        if start == 0 {
            left[0] = 1.0;
            right[0] = 1.0;
        }
        let report = chain
            .process_dual_mono(
                DualMonoBlock::new(&mut left[..frames], &mut right[..frames], start as u64)
                    .expect("block"),
            )
            .expect("render");
        recoveries += report.recovered_left_state + report.recovered_right_state;
        impulse.extend_from_slice(&left[..frames]);
    }
    let impulse_db = dft_magnitude_db(&impulse, f64::from(rate), probe);
    let tail_energy = impulse[impulse.len().saturating_sub(4096)..]
        .iter()
        .map(|value| f64::from(*value).powi(2))
        .sum();
    let (fundamental, residual, total) = sustained_metrics(rate, quantum, parameters, probe);
    (
        cast_state_response_db(rate, section, cutoff, probe),
        impulse_db,
        fundamental,
        residual,
        total,
        tail_energy,
        recoveries,
    )
}

fn cascade_cutoffs(rate: u32, cutoff: f64) -> (f64, f64) {
    let hpf = (0.5 * cutoff).max(10.0);
    let lpf = (2.0 * cutoff).min(0.475 * f64::from(rate));
    debug_assert!(hpf < lpf);
    (hpf, lpf)
}

fn cast_state_response_db(rate: u32, section: &str, cutoff: f64, probe: f64) -> f64 {
    match section {
        "high_pass" => cast_state_magnitude_db(rate, cutoff as f32, true, probe),
        "low_pass" => cast_state_magnitude_db(rate, cutoff as f32, false, probe),
        "cascade" => {
            let (hpf, lpf) = cascade_cutoffs(rate, cutoff);
            cast_state_magnitude_db(rate, hpf as f32, true, probe)
                + cast_state_magnitude_db(rate, lpf as f32, false, probe)
        }
        _ => panic!("unknown response section"),
    }
}

fn dft_magnitude_db(samples: &[f32], rate: f64, frequency: f64) -> f64 {
    let phase = -core::f64::consts::TAU * frequency / rate;
    let (step_re, step_im) = (phase.cos(), phase.sin());
    let (mut unit_re, mut unit_im) = (1.0_f64, 0.0_f64);
    let (mut real, mut imaginary) = (0.0_f64, 0.0_f64);
    for sample in samples {
        let value = f64::from(*sample);
        real += value * unit_re;
        imaginary += value * unit_im;
        (unit_re, unit_im) = (
            unit_re * step_re - unit_im * step_im,
            unit_re * step_im + unit_im * step_re,
        );
    }
    20.0 * real.hypot(imaginary).log10()
}

#[allow(clippy::needless_range_loop)]
fn sustained_metrics(
    rate: u32,
    quantum: u32,
    parameters: BuiltinParameters,
    frequency: f64,
) -> (f64, f64, f64) {
    let mut chain = BuiltinChain::new(rate, parameters).expect("fixture filter");
    let settle = rate as usize / 2;
    let frames = rate as usize / 4;
    let mut samples = Vec::with_capacity(frames);
    let mut input_energy = 0.0;
    let mut output_energy = 0.0;
    let mut left = vec![0.0_f32; quantum as usize];
    let mut right = vec![0.0_f32; quantum as usize];
    for start in (0..settle + frames).step_by(quantum as usize) {
        let count = (settle + frames - start).min(quantum as usize);
        for (index, (left, right)) in left[..count]
            .iter_mut()
            .zip(right[..count].iter_mut())
            .enumerate()
        {
            let input = (0.5
                * (core::f64::consts::TAU * frequency * (start + index) as f64 / f64::from(rate))
                    .sin()) as f32;
            *left = input;
            *right = input;
            if start + index >= settle {
                input_energy += f64::from(input).powi(2);
            }
        }
        chain
            .process_dual_mono(
                DualMonoBlock::new(&mut left[..count], &mut right[..count], start as u64)
                    .expect("block"),
            )
            .expect("render");
        for index in 0..count {
            if start + index >= settle {
                samples.push(f64::from(left[index]));
                output_energy += f64::from(left[index]).powi(2);
            }
        }
    }
    let count = frames as f64;
    let (mut sin_sum, mut cos_sum, mut dc_sum) = (0.0, 0.0, 0.0);
    for (index, sample) in samples.iter().enumerate() {
        let phase = core::f64::consts::TAU * frequency * (settle + index) as f64 / f64::from(rate);
        sin_sum += sample * phase.sin();
        cos_sum += sample * phase.cos();
        dc_sum += sample;
    }
    let dc = dc_sum / count;
    let sine = 2.0 * sin_sum / count;
    let cosine = 2.0 * cos_sum / count;
    let residual: f64 = samples
        .iter()
        .enumerate()
        .map(|(index, sample)| {
            let phase =
                core::f64::consts::TAU * frequency * (settle + index) as f64 / f64::from(rate);
            (sample - (dc + sine * phase.sin() + cosine * phase.cos())).powi(2)
        })
        .sum();
    let input_rms = (input_energy / count).sqrt();
    (
        20.0 * (sine.hypot(cosine) / 0.5).log10(),
        20.0 * ((residual / count).sqrt() / input_rms).log10(),
        20.0 * ((output_energy / count).sqrt() / input_rms).log10(),
    )
}

fn cast_state_magnitude_db(rate: u32, cutoff: f32, high_pass: bool, frequency: f64) -> f64 {
    let g = (core::f64::consts::PI * f64::from(cutoff) / f64::from(rate)).tan();
    let k = core::f64::consts::SQRT_2 as f32;
    let denominator = 1.0 + g * (g + core::f64::consts::SQRT_2);
    let (c1, a2, a3) = (
        (g * (g + core::f64::consts::SQRT_2) / denominator) as f32,
        (g / denominator) as f32,
        (g * g / denominator) as f32,
    );
    let (c1, a2, a3, k) = (f64::from(c1), f64::from(a2), f64::from(a3), f64::from(k));
    let (a00, a01, a10, a11) = (1.0 - 2.0 * c1, -2.0 * a2, 2.0 * a2, 1.0 - 2.0 * a3);
    let (b0, b1) = (2.0 * a2, 2.0 * a3);
    let (c0, c_1, direct) = if high_pass {
        (-k * (1.0 - c1) - a2, k * a2 - (1.0 - a3), 1.0 - k * a2 - a3)
    } else {
        (a2, 1.0 - a3, a3)
    };
    let phase = core::f64::consts::TAU * frequency / f64::from(rate);
    let (zr, zi) = (phase.cos(), phase.sin());
    let (m00, m01, m10, m11) = (zr - a00, -a01, -a10, zr - a11);
    let (det_re, det_im) = (m00 * m11 - zi * zi - m01 * m10, zi * (m00 + m11));
    let norm = det_re * det_re + det_im * det_im;
    let divide = |real: f64, imaginary: f64| {
        (
            (real * det_re + imaginary * det_im) / norm,
            (imaginary * det_re - real * det_im) / norm,
        )
    };
    let (s0r, s0i) = divide(m11 * b0 - m01 * b1, zi * b0);
    let (s1r, s1i) = divide(-m10 * b0 + m00 * b1, zi * b1);
    20.0 * (direct + c0 * s0r + c_1 * s1r)
        .hypot(c0 * s0i + c_1 * s1i)
        .log10()
}

fn probes(rate: u32, cutoff: f64) -> Vec<f64> {
    let nyquist = 0.5 * f64::from(rate);
    let mut values: Vec<_> = [
        0.25 * cutoff,
        cutoff,
        4.0 * cutoff,
        0.2 * f64::from(rate),
        0.45 * f64::from(rate),
    ]
    .into_iter()
    .map(|value| (value.clamp(4.0, nyquist - 4.0) / 4.0).round() * 4.0)
    .collect();
    values.sort_by(f64::total_cmp);
    values.dedup();
    values
}

fn diagnostics() -> String {
    // Do not hand-maintain diagnostic declarations: each tuple below is obtained from
    // the public production API that emits it.  The path is the stable request/session
    // path supplied to that API, so this fixture catches both code and path drift.
    let mut rows = Vec::new();
    let direct = |case: &str, parameters: BuiltinParameters, path: &str| {
        let Err(error) = BuiltinChain::new(48_000, parameters) else {
            panic!("invalid fixture input accepted");
        };
        let code = match error {
            BuiltinParameterError::FilterCutoff
            | BuiltinParameterError::FilterOrder
            | BuiltinParameterError::FilterCoefficients => "builtin.filter.cutoff",
            BuiltinParameterError::GainDomain => "builtin.gain.domain",
            BuiltinParameterError::MatrixCoefficient | BuiltinParameterError::MatrixSmoothing => {
                "builtin.matrix.coefficient"
            }
            BuiltinParameterError::LaneLength | BuiltinParameterError::EmptyBlock => {
                "builtin.block.length"
            }
            BuiltinParameterError::SampleTimeOverflow => "builtin.block.sample_time_overflow",
        };
        format!("{{\"case\":\"{case}\",\"code\":\"{code}\",\"path\":\"{path}\"}}")
    };
    rows.push(direct(
        "filter-cutoff",
        BuiltinParameters {
            left: ChannelParameters {
                hpf_hz: -1.0,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        },
        "$.tracks[id=vocal].builtins.left.hpf_hz",
    ));
    rows.push(direct(
        "filter-order",
        BuiltinParameters {
            left: ChannelParameters {
                hpf_hz: 1_000.0,
                lpf_hz: 1_000.0,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        },
        "$.tracks[id=vocal].builtins.left",
    ));
    rows.push(direct(
        "gain-domain",
        BuiltinParameters {
            left: ChannelParameters {
                fader_db: 24.5,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        },
        "$.tracks[id=vocal].fader.left_db",
    ));
    rows.push(direct(
        "matrix-coefficient",
        BuiltinParameters {
            matrix: Matrix2x2 {
                ll: 1.1,
                ..Matrix2x2::IDENTITY
            },
            ..BuiltinParameters::default()
        },
        "$.tracks[id=vocal].matrix_or_pan.ll",
    ));
    let mut left = [0.0_f32];
    let mut right = [0.0_f32, 0.0];
    let Err(block) = DualMonoBlock::new(&mut left, &mut right, 0) else {
        panic!("mismatched lanes accepted");
    };
    rows.push(format!(
        "{{\"case\":\"block-length\",\"code\":\"builtin.block.length\",\"path\":\"$.render.block\",\"error\":\"{block:?}\"}}"
    ));
    let mut left = [0.0_f32];
    let mut right = [0.0_f32];
    let Err(block) = DualMonoBlock::new(&mut left, &mut right, u64::MAX) else {
        panic!("overflow accepted");
    };
    rows.push(format!(
        "{{\"case\":\"block-overflow\",\"code\":\"builtin.block.sample_time_overflow\",\"path\":\"$.render.first_sample\",\"error\":\"{block:?}\"}}"
    ));
    let session = fixture_session();
    let config = meter_config(2, 1, 3);
    let duplicate = [
        MeterRequest {
            track_id: "vocal".to_owned(),
            tap: MeterTap::Input,
            config,
        },
        MeterRequest {
            track_id: "vocal".to_owned(),
            tap: MeterTap::Input,
            config,
        },
        MeterRequest {
            track_id: "missing".to_owned(),
            tap: MeterTap::PostFader,
            config,
        },
    ];
    let Err(error) = prepare_session_builtins(&session, &duplicate, unlimited_builtin_caps())
    else {
        panic!("duplicate and unknown request accepted");
    };
    for diagnostic in error.0 {
        rows.push(format!(
            "{{\"case\":\"meter-request\",\"code\":\"{}\",\"path\":\"{}\"}}",
            diagnostic.code, diagnostic.path
        ));
    }
    let mut caps = unlimited_builtin_caps();
    caps.maximum_total_state_bytes = 1;
    let Err(error) = prepare_session_builtins(&session, &[], caps) else {
        panic!("resource cap accepted");
    };
    for diagnostic in error.0 {
        rows.push(format!(
            "{{\"case\":\"resource-cap\",\"code\":\"{}\",\"path\":\"{}\"}}",
            diagnostic.code, diagnostic.path
        ));
    }
    let prepared = prepare_session_builtins(&session, &[], unlimited_builtin_caps())
        .expect("valid prepared artifact");
    let mismatched = fixture_session_tracks(4);
    for diagnostic in prepared.validate_for_session(&mismatched).0 {
        rows.push(format!(
            "{{\"case\":\"seal-session-mismatch\",\"code\":\"{}\",\"path\":\"{}\"}}",
            diagnostic.code, diagnostic.path
        ));
    }
    rows.sort();
    rows.dedup();
    rows.join("\n") + "\n"
}

fn meter_config(period: u32, capacity: usize, reset_generation: u64) -> MeterConfig {
    MeterConfig {
        period_frames: core::num::NonZeroU32::new(period).expect("period"),
        peak_hold_frames: 1,
        peak_decay_db_per_second: 12.0,
        queue_capacity: core::num::NonZeroUsize::new(capacity).expect("capacity"),
        reset_generation,
    }
}

fn meters() -> String {
    let mut rows = Vec::new();
    let handle = MeterHandle(core::num::NonZeroU64::new(1).expect("handle"));

    // A partial interval intentionally produces no snapshot; retain the observed state as
    // an expected record so it cannot be mistaken for missing fixture coverage.
    let mut partial =
        MeterAccumulator::prepare(handle, meter_config(4, 4, 3), 48_000).expect("partial meter");
    partial
        .accumulator
        .observe(&[1.0, 0.5], &[0.0, -1.0], 3)
        .expect("partial");
    rows.push("{\"case\":\"partial\",\"snapshot\":null,\"observed_frames\":2,\"period_frames\":4,\"start\":\"0000000000000003\"}".to_owned());

    let mut multiple =
        MeterAccumulator::prepare(handle, meter_config(2, 8, 3), 48_000).expect("multiple meter");
    multiple
        .accumulator
        .observe(&[1.0, 0.5, 0.25, 0.0], &[0.0, -1.0, 0.5, -0.25], 3)
        .expect("multiple");
    while let Ok(snapshot) = multiple.consumer.try_pop() {
        rows.push(meter_snapshot_json("multiple", snapshot));
    }

    // Interleaved draining forces the ring to wrap without a drop.
    let mut wrap =
        MeterAccumulator::prepare(handle, meter_config(2, 2, 3), 48_000).expect("wrap meter");
    wrap.accumulator
        .observe(&[1.0, 0.5, 0.25, 0.0], &[0.0, -1.0, 0.5, -0.25], 3)
        .expect("wrap first");
    rows.push(meter_snapshot_json(
        "wrap",
        wrap.consumer.try_pop().expect("first wrap snapshot"),
    ));
    wrap.accumulator
        .observe(&[0.75, 0.25, 0.5, 0.0], &[-0.5, 0.0, 0.25, -0.25], 7)
        .expect("wrap second");
    while let Ok(snapshot) = wrap.consumer.try_pop() {
        rows.push(meter_snapshot_json("wrap", snapshot));
    }

    let mut full =
        MeterAccumulator::prepare(handle, meter_config(2, 1, 3), 48_000).expect("full meter");
    full.accumulator
        .observe(&[1.0, 0.0, 0.5, 0.0], &[0.0, 1.0, 0.0, 0.5], 3)
        .expect("full");
    rows.push(meter_snapshot_json(
        "full",
        full.consumer.try_pop().expect("full first"),
    ));
    full.accumulator
        .observe(&[0.25, 0.0], &[0.0, -0.25], 7)
        .expect("after drop");
    while let Ok(snapshot) = full.consumer.try_pop() {
        rows.push(meter_snapshot_json("drop", snapshot));
    }

    let mut discontinuity = MeterAccumulator::prepare(handle, meter_config(2, 4, 3), 48_000)
        .expect("discontinuity meter");
    discontinuity
        .accumulator
        .observe(&[1.0, 0.0], &[0.0, 1.0], 3)
        .expect("first");
    discontinuity
        .accumulator
        .observe(&[0.5, 0.0], &[0.0, 0.5], 9)
        .expect("discontinuous");
    while let Ok(snapshot) = discontinuity.consumer.try_pop() {
        rows.push(meter_snapshot_json("discontinuity", snapshot));
    }

    let mut reset =
        MeterAccumulator::prepare(handle, meter_config(2, 4, 3), 48_000).expect("reset meter");
    reset
        .accumulator
        .observe(&[1.0, 0.0], &[0.0, 1.0], 3)
        .expect("reset first");
    rows.push(meter_snapshot_json(
        "drain",
        reset.consumer.try_pop().expect("drain snapshot"),
    ));
    reset
        .accumulator
        .reset(BuiltinResetKind::DiscontinuityKeepTargets);
    reset
        .accumulator
        .observe(&[0.5, 0.0], &[0.0, 0.5], 9)
        .expect("keep reset");
    rows.push(meter_snapshot_json(
        "reset-discontinuity",
        reset.consumer.try_pop().expect("keep reset snapshot"),
    ));
    reset.accumulator.reset(BuiltinResetKind::FullToPrepared);
    reset
        .accumulator
        .observe(&[0.25, 0.0], &[0.0, 0.25], 11)
        .expect("full reset");
    rows.push(meter_snapshot_json(
        "reset-full",
        reset.consumer.try_pop().expect("full reset snapshot"),
    ));

    let mut sanitized = MeterAccumulator::prepare(handle, meter_config(2, 4, 3), 48_000)
        .expect("sanitization meter");
    sanitized
        .accumulator
        .observe(&[f32::NAN, 0.5], &[f32::INFINITY, -0.5], 3)
        .expect("sanitize");
    rows.push(meter_snapshot_json(
        "sanitization",
        sanitized.consumer.try_pop().expect("sanitized snapshot"),
    ));

    let mut overflow =
        MeterAccumulator::prepare(handle, meter_config(2, 4, 3), 48_000).expect("overflow meter");
    let Err(error) = overflow.accumulator.observe(&[0.0], &[0.0], u64::MAX) else {
        panic!("meter overflow accepted");
    };
    rows.push(format!(
        "{{\"case\":\"overflow\",\"error\":\"{error:?}\",\"path\":\"$.meter.first_sample\"}}"
    ));
    rows.sort();
    rows.join("\n") + "\n"
}

fn meter_snapshot_json(case: &str, snapshot: miso_engine_builtins::MeterSnapshot) -> String {
    format!(
        "{{\"case\":\"{case}\",\"handle\":\"{:016x}\",\"reset_generation\":\"{:016x}\",\"sequence\":\"{:016x}\",\"start\":\"{:016x}\",\"end\":\"{:016x}\",\"frames\":{},\"left_peak\":\"{:08x}\",\"right_peak\":\"{:08x}\",\"left_held_peak\":\"{:08x}\",\"right_held_peak\":\"{:08x}\",\"left_energy\":\"{:016x}\",\"right_energy\":\"{:016x}\",\"left_rms\":\"{:016x}\",\"right_rms\":\"{:016x}\",\"clipped\":\"{:016x}\",\"sanitized\":\"{:016x}\",\"dropped\":\"{:016x}\",\"discontinuities\":\"{:016x}\"}}",
        snapshot.handle.0.get(),
        snapshot.reset_generation,
        snapshot.window_sequence,
        snapshot.start_sample,
        snapshot.end_sample,
        snapshot.frames,
        snapshot.left.sample_peak.to_bits(),
        snapshot.right.sample_peak.to_bits(),
        snapshot.left.held_peak.to_bits(),
        snapshot.right.held_peak.to_bits(),
        snapshot.left.energy.to_bits(),
        snapshot.right.energy.to_bits(),
        snapshot.left.rms.to_bits(),
        snapshot.right.rms.to_bits(),
        snapshot.cumulative_clipped_samples,
        snapshot.cumulative_sanitized_samples,
        snapshot.cumulative_dropped_snapshots,
        snapshot.cumulative_discontinuities,
    )
}

fn resources() -> String {
    let mut output = String::new();
    for tracks in [1_usize, 4, 65_537] {
        for meters in [0_usize, 1, 7] {
            let capacity = if meters == 7 { 4 } else { 1 };
            let session = fixture_session_tracks(tracks);
            let config = MeterConfig {
                period_frames: core::num::NonZeroU32::new(session.quantum().0).expect("quantum"),
                peak_hold_frames: 0,
                peak_decay_db_per_second: 0.0,
                queue_capacity: core::num::NonZeroUsize::new(capacity).expect("capacity"),
                reset_generation: 0,
            };
            let requests: Vec<_> = [
                MeterTap::Input,
                MeterTap::PostInputBuiltins,
                MeterTap::PostSimd1,
                MeterTap::PostDynamic,
                MeterTap::PostSimd2PreFader,
                MeterTap::PostFader,
                MeterTap::PostMatrix,
            ][..meters]
                .iter()
                .copied()
                .map(|tap| MeterRequest {
                    track_id: if tracks == 1 { "vocal" } else { "track-0" }.to_owned(),
                    tap,
                    config,
                })
                .collect();
            let report = prepare_session_builtins(&session, &requests, unlimited_builtin_caps())
                .expect("resource fixture")
                .resource_report();
            writeln!(output, "{{\"tracks\":{tracks},\"meters\":{meters},\"queue_capacity\":{capacity},\"meter_items\":{},\"engine_owned_processor_payload_bytes\":{},\"engine_owned_meter_payload_bytes\":{},\"engine_owned_retained_payload_bytes\":{},\"maximum_single_allocation_bytes\":{},\"retained_allocation_count\":{}}}", report.meter_items, report.engine_owned_processor_payload_bytes, report.engine_owned_meter_payload_bytes, report.engine_owned_retained_payload_bytes, report.maximum_single_allocation_bytes, report.retained_allocation_count).expect("string");
        }
    }
    output
}

fn fixture_session() -> miso_engine_session::CompiledSession {
    let mut model = parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
        .expect("fixture session");
    model.tracks[0].simd1.effects.clear();
    model.tracks[0].dynamic.effects.clear();
    model.tracks[0].simd2.effects.clear();
    model.automation.clear();
    compile_session(
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
    .expect("compile session")
}

fn fixture_session_tracks(count: usize) -> miso_engine_session::CompiledSession {
    if count == 1 {
        return fixture_session();
    }
    let mut model = parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
        .expect("fixture session");
    let mut template = model.tracks[0].clone();
    template.simd1.effects.clear();
    template.dynamic.effects.clear();
    template.simd2.effects.clear();
    model.tracks.clear();
    model.tracks.reserve(count);
    for index in 0..count {
        let mut track = template.clone();
        track.id = StableId::parse(&format!("track-{index}")).expect("stable track ID");
        model.tracks.push(track);
    }
    model.routes[0].source = RouteSource::Track {
        track_id: StableId::parse("track-0").expect("track ID"),
        tap: SendTap::PostMatrix,
    };
    model.automation.clear();
    compile_session(
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
    .expect("compile session")
}

fn unlimited_builtin_caps() -> BuiltinCompileCaps {
    BuiltinCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_total_meter_items: u64::MAX,
        maximum_total_meter_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    }
}

fn pcm_cases() -> Vec<(String, Vec<u8>)> {
    let filters = BuiltinParameters {
        left: ChannelParameters {
            hpf_hz: 100.0,
            ..ChannelParameters::default()
        },
        right: ChannelParameters {
            lpf_hz: 1000.0,
            ..ChannelParameters::default()
        },
        ..BuiltinParameters::default()
    };
    let gain = BuiltinParameters {
        left: ChannelParameters {
            polarity_invert: true,
            trim_db: -6.0,
            fader_db: -3.0,
            ..ChannelParameters::default()
        },
        right: ChannelParameters {
            trim_db: 3.0,
            ..ChannelParameters::default()
        },
        ..BuiltinParameters::default()
    };
    let matrix = BuiltinParameters {
        matrix: Matrix2x2 {
            ll: 0.0,
            lr: 1.0,
            rl: 1.0,
            rr: 0.0,
        },
        ..BuiltinParameters::default()
    };
    let ramp = BuiltinParameters {
        smoothing_samples: 8,
        ..matrix
    };
    let mut fixtures = vec![
        (
            "identity-signed-zero".to_owned(),
            render_pcm(BuiltinParameters::default()),
        ),
        ("filters-asymmetric".to_owned(), render_pcm(filters)),
        ("polarity-gain".to_owned(), render_pcm(gain)),
        ("matrix-corner".to_owned(), render_pcm(matrix)),
        ("matrix-ramp".to_owned(), render_pcm(ramp)),
        (
            "mute".to_owned(),
            render_pcm(BuiltinParameters {
                left: ChannelParameters {
                    muted: true,
                    ..ChannelParameters::default()
                },
                right: ChannelParameters {
                    muted: true,
                    ..ChannelParameters::default()
                },
                ..BuiltinParameters::default()
            }),
        ),
        ("matrix-retarget".to_owned(), render_matrix_retarget()),
        ("reset".to_owned(), render_reset()),
        ("lr-isolation".to_owned(), render_lr_isolation()),
        ("partition".to_owned(), render_partition()),
    ];
    for bits in 0_u8..16 {
        let matrix = Matrix2x2 {
            ll: f32::from(bits & 1),
            lr: f32::from((bits >> 1) & 1),
            rl: f32::from((bits >> 2) & 1),
            rr: f32::from((bits >> 3) & 1),
        };
        fixtures.push((
            format!("matrix-corner-{bits:02}"),
            render_pcm(BuiltinParameters {
                matrix,
                ..BuiltinParameters::default()
            }),
        ));
    }
    for updates in [0_u32, 1, 2, 127, 128, u32::MAX] {
        fixtures.push((
            format!("matrix-ramp-{updates}"),
            render_matrix_ramp(updates),
        ));
    }
    fixtures.sort_by(|left, right| left.0.cmp(&right.0));
    fixtures
}

fn render_pcm(parameters: BuiltinParameters) -> Vec<u8> {
    let mut chain = BuiltinChain::new(48_000, parameters).expect("fixture parameters");
    let mut left = [0.0_f32, -0.0, 0.25, -0.5, 1.0, -1.0, 0.125, -0.25];
    let mut right = [-0.0_f32, 0.0, -0.125, 0.5, -1.0, 1.0, -0.25, 0.25];
    chain
        .process_dual_mono(DualMonoBlock::new(&mut left, &mut right, 0).expect("fixture block"))
        .expect("fixture render");
    left.into_iter()
        .chain(right)
        .flat_map(f32::to_le_bytes)
        .collect()
}

fn pack_pcm(left: &[f32], right: &[f32]) -> Vec<u8> {
    left.iter()
        .chain(right)
        .copied()
        .flat_map(f32::to_le_bytes)
        .collect()
}

fn render_matrix_ramp(updates: u32) -> Vec<u8> {
    let mut chain = BuiltinChain::new(
        48_000,
        BuiltinParameters {
            smoothing_samples: updates,
            ..BuiltinParameters::default()
        },
    )
    .expect("ramp fixture");
    chain
        .set_matrix_target(Matrix2x2 {
            ll: 0.0,
            lr: 1.0,
            rl: 1.0,
            rr: 0.0,
        })
        .expect("ramp target");
    let mut left = [1.0_f32; 128];
    let mut right = [-0.5_f32; 128];
    chain
        .process_matrix(DualMonoBlock::new(&mut left, &mut right, 0).expect("ramp block"))
        .expect("ramp render");
    pack_pcm(&left, &right)
}

fn render_matrix_retarget() -> Vec<u8> {
    let mut chain = BuiltinChain::new(
        48_000,
        BuiltinParameters {
            smoothing_samples: 8,
            ..BuiltinParameters::default()
        },
    )
    .expect("retarget fixture");
    chain
        .set_matrix_target(Matrix2x2 {
            ll: 0.0,
            lr: 1.0,
            rl: 1.0,
            rr: 0.0,
        })
        .expect("first target");
    let mut left = vec![1.0_f32; 4];
    let mut right = vec![-0.5_f32; 4];
    chain
        .process_matrix(DualMonoBlock::new(&mut left, &mut right, 0).expect("first block"))
        .expect("first render");
    chain
        .set_matrix_target(Matrix2x2::IDENTITY)
        .expect("second target");
    let mut tail_left = [1.0_f32; 8];
    let mut tail_right = [-0.5_f32; 8];
    chain
        .process_matrix(
            DualMonoBlock::new(&mut tail_left, &mut tail_right, 4).expect("second block"),
        )
        .expect("second render");
    left.extend(tail_left);
    right.extend(tail_right);
    pack_pcm(&left, &right)
}

fn render_reset() -> Vec<u8> {
    let parameters = BuiltinParameters {
        left: ChannelParameters {
            hpf_hz: 100.0,
            ..ChannelParameters::default()
        },
        ..BuiltinParameters::default()
    };
    let mut chain = BuiltinChain::new(48_000, parameters).expect("reset fixture");
    let mut left = vec![1.0_f32, 0.0, 0.0, 0.0];
    let mut right = vec![0.0_f32; 4];
    chain
        .process_dual_mono(DualMonoBlock::new(&mut left, &mut right, 0).expect("pre-reset"))
        .expect("pre-reset render");
    chain.reset(BuiltinResetKind::DiscontinuityKeepTargets);
    let mut post_left = [1.0_f32, 0.0, 0.0, 0.0];
    let mut post_right = [0.0_f32; 4];
    chain
        .process_dual_mono(
            DualMonoBlock::new(&mut post_left, &mut post_right, 4).expect("post-reset"),
        )
        .expect("post-reset render");
    left.extend(post_left);
    right.extend(post_right);
    pack_pcm(&left, &right)
}

fn render_lr_isolation() -> Vec<u8> {
    let mut chain = BuiltinChain::new(
        48_000,
        BuiltinParameters {
            left: ChannelParameters {
                hpf_hz: 100.0,
                ..ChannelParameters::default()
            },
            right: ChannelParameters {
                lpf_hz: 1_000.0,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        },
    )
    .expect("isolation fixture");
    let mut left = [1.0_f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut right = [0.0_f32, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    chain
        .process_dual_mono(DualMonoBlock::new(&mut left, &mut right, 0).expect("isolation block"))
        .expect("isolation render");
    pack_pcm(&left, &right)
}

fn render_partition() -> Vec<u8> {
    let parameters = BuiltinParameters {
        left: ChannelParameters {
            hpf_hz: 100.0,
            ..ChannelParameters::default()
        },
        right: ChannelParameters {
            lpf_hz: 1_000.0,
            ..ChannelParameters::default()
        },
        ..BuiltinParameters::default()
    };
    let mut chain = BuiltinChain::new(48_000, parameters).expect("partition fixture");
    let mut left = [1.0_f32, 0.0, -0.5, 0.25, 0.0, 0.0, 0.75, -0.25];
    let mut right = [-0.5_f32, 0.0, 1.0, -0.25, 0.0, 0.5, 0.0, 0.25];
    for (offset, width) in [(0, 1), (1, 2), (3, 1), (4, 4)] {
        chain
            .process_dual_mono(
                DualMonoBlock::new(
                    &mut left[offset..offset + width],
                    &mut right[offset..offset + width],
                    offset as u64,
                )
                .expect("partition block"),
            )
            .expect("partition render");
    }
    pack_pcm(&left, &right)
}

fn manifest(files: &[(String, Vec<u8>)]) -> String {
    let mut output = String::from(MANIFEST_HEADER);
    for (path, bytes) in files {
        writeln!(output, "{path}\t{}\t{}", bytes.len(), sha256(bytes)).expect("string");
    }
    output
}

fn write_and_verify(root: &Path) -> Result<(), String> {
    let files = generated();
    for (path, bytes) in &files {
        let destination = root.join(path);
        let parent = destination
            .parent()
            .ok_or_else(|| "fixture path has no parent".to_owned())?;
        fs::create_dir_all(parent).map_err(|error| format!("create fixture directory: {error}"))?;
        fs::write(destination, bytes).map_err(|error| format!("write fixture: {error}"))?;
    }
    fs::write(root.join("MANIFEST.tsv"), manifest(&files))
        .map_err(|error| format!("write manifest: {error}"))?;
    verify(root, &files)
}

fn verify(root: &Path, expected: &[(String, Vec<u8>)]) -> Result<(), String> {
    if fs::read(root.join("MANIFEST.tsv")).map_err(|error| format!("read manifest: {error}"))?
        != manifest(expected).as_bytes()
    {
        return Err("builtins fixture manifest mismatch".to_owned());
    }
    for (path, bytes) in expected {
        if fs::read(root.join(path)).map_err(|error| format!("read {path}: {error}"))? != *bytes {
            return Err(format!("builtins fixture content mismatch: {path}"));
        }
    }
    let mut actual = list_files(root)?;
    actual.sort();
    let expected_paths: Vec<_> = expected.iter().map(|(path, _)| path.clone()).collect();
    if actual != expected_paths {
        return Err("builtins fixture missing or unlisted file".to_owned());
    }
    Ok(())
}

fn list_files(root: &Path) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    fn visit(root: &Path, path: &Path, files: &mut Vec<String>) -> Result<(), String> {
        for entry in
            fs::read_dir(path).map_err(|error| format!("read fixture directory: {error}"))?
        {
            let entry = entry.map_err(|error| format!("read fixture entry: {error}"))?;
            if entry
                .file_type()
                .map_err(|error| format!("read fixture type: {error}"))?
                .is_dir()
            {
                visit(root, &entry.path(), files)?;
            } else {
                let relative = entry
                    .path()
                    .strip_prefix(root)
                    .map_err(|_| "fixture relative path".to_owned())?
                    .to_str()
                    .ok_or_else(|| "fixture path is not UTF-8".to_owned())?
                    .replace('\\', "/");
                if relative != "MANIFEST.tsv" {
                    files.push(relative);
                }
            }
        }
        Ok(())
    }
    visit(root, root, &mut files)?;
    Ok(files)
}

fn sha256(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(output, "{byte:02x}").expect("string");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generated_corpus_is_complete_and_deterministic() {
        let root = env::temp_dir().join(format!(
            "miso-engine-builtins-fixture-{}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale fixture root");
        }
        let files = generated();
        for (path, bytes) in &files {
            let destination = root.join(path);
            fs::create_dir_all(destination.parent().expect("parent")).expect("directory");
            fs::write(destination, bytes).expect("fixture");
        }
        fs::write(root.join("MANIFEST.tsv"), manifest(&files)).expect("manifest");
        verify(&root, &files).expect("valid fixture corpus");
        for (path, bytes) in &files {
            let mut corrupt = bytes.clone();
            corrupt.push(0);
            fs::write(root.join(path), corrupt).expect("corrupt");
            assert!(
                verify(&root, &files).is_err(),
                "accepted corruption: {path}"
            );
            fs::write(root.join(path), bytes).expect("restore");
        }
        fs::write(root.join("unlisted"), []).expect("unlisted");
        assert!(verify(&root, &files).is_err());
        fs::remove_dir_all(root).expect("cleanup");
    }
}
