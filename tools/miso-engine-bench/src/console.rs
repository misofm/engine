//! Issue #149 item 3: the standing console qualification benchmark.
//!
//! # Why this subject exists
//!
//! The number the sprint inherited is a ragged nine-track EQ-only session. Nine tracks at the
//! launch eight-lane width is one full bank plus a one-track scalar tail -- the worst shape a bank
//! can be asked to render, and not the shape of a mixing session. This subject measures the shape
//! a session actually has: sixty-four tracks, each carrying the full channel strip (input
//! trim/HPF/LPF, a parametric EQ in SIMD rack 1, a compressor in the dynamic rack, a fader and a
//! pan matrix), rendered through a real [`miso_engine_core::realtime::PreparedRenderPlan`] at
//! 48 kHz and a 128-frame quantum.
//!
//! Sixty-four tracks is eight full banks and no tail, so the per-track cost reported here is the
//! cost of a full bank rather than the cost of a remainder. The nine-track fixture is kept as a
//! workload so the two numbers are produced by one binary, on one host, in one run -- the only way
//! the comparison means anything.
//!
//! # The measurement discipline
//!
//! Frozen before timing and not tuned afterwards (AGENTS.md): fixed fixtures, fixed observation
//! count, one warmup pass and two measured rounds supplied by the runner. Percentiles are the
//! shared nearest-rank implementation, and the timed region is `timing::timed`, which panics if
//! the body hashed anything -- so evidence collection cannot drift inside the clock (#104 F1).
//!
//! ## Paired alternation
//!
//! The hoist arms are **interleaved observation by observation**, not run one after the other. A
//! benchmark that renders all of arm A and then all of arm B measures the difference between two
//! points in time as much as the difference between two arms: the host's clock ramps, another
//! tenant lands on the sibling core, a thermal limit engages. Alternating means every drift a run
//! suffers is shared by both arms, and the per-observation pairing makes the delta a distribution
//! rather than a difference of two summaries.
//!
//! ## What the hoist arms measure
//!
//! Issue #144 item 6 elides the smoothing window a parameter opens when it is retargeted to the
//! value it already holds. The two arms deliver the *same* automation traffic and differ only in
//! whether that traffic is stationary:
//!
//! * `restated` sends every parameter its current value on every block. The hoist settles it.
//! * `moving` alternates each parameter between two nearby values on every block, so a real
//!   window is open at all times and the hoist can never fire.
//!
//! A first attempt used a one-ULP move for the control arm and it did **not** work, which is worth
//! recording: a one-ULP change in a dB gain designs to the *same* `f32` SVF coefficient words, so
//! the EQ's hoist fired on the designed words and the "control" arm was hoisted too. That is a
//! real property of the optimisation -- it catches redundancy at the coefficient level, not only
//! at the parameter level -- but it makes one ULP useless as a control. The moving arm therefore
//! uses the smallest step that actually changes the designed words.
//!
//! The two arms deliver the same number of spans and do the same control-plane work, so the delta
//! between them is the ramping decision and the window it opens. `moving` is a faithful cost
//! stand-in for the pre-hoist behaviour of `restated`, which opened exactly that window with a
//! step of `+0.0` -- the same ramped kernel over the same lanes for the same sixty-four samples.
//! Their *audio* differs, and only `quiet` and `restated` are asserted bit-identical.
//!
//! `quiet` delivers no automation at all and is the standing baseline row.
//!
//! # The decomposition rows (issue #163 item 0c)
//!
//! One number for a whole console strip says how long the block took and nothing about where the
//! time went. The five rows added here answer that by subtraction, and they can only be subtracted
//! because every one of them is the *same* fixture with part of the strip removed in code:
//! `sixty_four_track_eq_only`, `sixty_four_track_compressor_only`, `sixty_four_track_builtins_only`,
//! `sixty_four_track_dispatch_only` and `sixty_four_track_idle`. Same tracks, same parameters, same
//! sources, same binary, same run.
//!
//! They are derived from the checked-in model by [`miso_engine_console_workload`]'s strip
//! edits rather than being five more
//! 900-line TOMLs, for the reason the 128-track stretch fixture already gives: nothing about an
//! emptied rack is a new *shape* to review, and five near-duplicate fixtures would be five files
//! that can drift apart from the one they were copied from. Every derived row says
//! `synthetic_fixture: true` and names what its strip carries, and the validator pins that pairing
//! per workload kind, so a row cannot quietly claim a rack it emptied.
//!
//! # The console-facility arms (issue #163 item 0d)
//!
//! Every session row above renders a plan with no meter stream, no live-console control channel
//! and no observation capacity -- which is not what a running console is. `console_meters` and
//! `console_observation` measure what those facilities cost, under the same paired-alternation
//! protocol as the hoist arms, and through the production entry points rather than a hand-built
//! stand-in: a meter is a `MeterRequest` handed to `prepare_session_builtins`, and observation is
//! `attach_effect_observation_v1` plus an `EffectControlRecordV1::Observe` pushed through the same
//! bounded queue a host pushes it through.
//!
//! `console_observation` measures the issue #143 two-level zero rather than restating it: `absent`
//! has no lane at all, `unarmed` has a lane with nothing armed, `armed` has every declared tap
//! armed. All three carry the control channel, so the deltas are the lane and the arming and never
//! the queue drain.
//!
//! Both records carry a class-A statement asserted in-run: observing a console must not change
//! what the console renders, so every arm of both measurements must produce byte-identical output.
//!
//! # The automation-active row (`console_automation`)
//!
//! Every session row above renders with automation **cleared**: `console_model` empties the
//! fixture's table unconditionally, and both fixture gates assert the standing sessions declare
//! none. The compressor's ramping body is therefore dead code in the whole standing table, and the
//! one arm that does deliver spans -- `console_hoist` -- drives banks of parametric EQs. No
//! compressor in this benchmark had ever seen an automation span, so no change to a compressor's
//! ramping path could move a number in it.
//!
//! `console_automation` closes that gap with the console's real traffic shape: **one Point span
//! per block, on one track**, pushed through the same bounded live-console queue a host pushes
//! through, into a real prepared plan. Its three arms carry the identical control channel and
//! differ only in what rides it -- `quiet` pushes nothing, `restated` restates the value in force
//! (the #144 hoist settles it, so no window opens), `automated` moves it every block. The paired
//! ramp delta is `automated - restated`: the same queue drain, the same span count, and a
//! smoothing window open in one arm and not the other. `quiet == restated` is the class-A
//! statement, asserted in-run.

use miso_engine_bench_support::alloc as bench_alloc;
use miso_engine_bench_support::digest::Sha256Sink;
use miso_engine_bench_support::json::escape as json_escape;
use miso_engine_bench_support::metadata::Metadata;
use miso_engine_bench_support::stats;
use miso_engine_bench_support::timing;
use miso_engine_console_workload::{
    ObservationArm, PlanConfig, QUANTUM, SAMPLE_RATE_HZ, SessionRuntime, WINDOW_BLOCKS, WORKLOADS,
    Workload,
};
use miso_engine_core::realtime::audit;
use miso_engine_effect_compiler::launch_native_effect_registry_v1;
use miso_engine_effect_contract::{
    AutomationSpanKind, BankWidth, EffectBankProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffectBank, PreparedPortsV1,
    PreparedSidechainPort,
};
use miso_engine_lane::Backend;

const OBSERVATIONS: usize = 1_000;
const ISSUE: u32 = 149;

pub(crate) fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    assert_eq!(
        std::env::args_os().count(),
        1,
        "benchmark accepts no arguments"
    );
    let round = round_from_runner();
    // Feature detection ends here, before every timed observation.
    let backend = Backend::current();
    let metadata = &Metadata::gather();

    for workload in WORKLOADS {
        let session = SessionMeasurement::run(workload);
        println!("{}", session.record(workload, round, backend, metadata));
    }
    for workload in [
        Workload::NineTrackRaggedStrip,
        Workload::SixtyFourTrackConsole,
    ] {
        let hoist = HoistMeasurement::run(workload, backend);
        println!("{}", hoist.record(workload, round, backend, metadata));
    }
    let meters = FacilityMeasurement::run(&METER_CONFIGS);
    println!("{}", meters.meters_record(round, backend, metadata));
    let observation = FacilityMeasurement::run(&OBSERVATION_CONFIGS);
    println!(
        "{}",
        observation.observation_record(round, backend, metadata)
    );
    let placement = PlacementMeasurement::run();
    println!("{}", placement.record(round, backend, metadata));
    let automation = AutomationMeasurement::run();
    println!("{}", automation.record(round, backend, metadata));
}

fn round_from_runner() -> u32 {
    match Metadata::gather().var("MISO_ENGINE_BENCH_ROUND").as_deref() {
        Ok("warmup") => 0,
        Ok("1") => 1,
        Ok("2") => 2,
        _ => panic!("the console benchmark must be launched by its fixed runner"),
    }
}

// ---------------------------------------------------------------------------------------------
// The standing session measurement: a real prepared plan, rendered block by block.
// ---------------------------------------------------------------------------------------------

struct SessionMeasurement {
    ns_per_block: Vec<u64>,
    output_sha256: String,
    audit: audit::AuditSnapshot,
    render_errors: u64,
}

impl SessionMeasurement {
    fn run(workload: Workload) -> Self {
        let mut runtime = SessionRuntime::new(workload);
        let mut durations = Vec::with_capacity(OBSERVATIONS);
        let mut output_hash = Sha256Sink::new();
        let mut render_errors = 0_u64;
        // Untimed settling. Only the idle row asks for any, and it asks for a lot: see
        // `Workload::warmup_blocks`.
        for observation in 0..workload.warmup_blocks() {
            let _ = runtime.render(observation as u64);
        }
        audit::warm_up();
        audit::reset();
        for observation in 0..OBSERVATIONS {
            // The timed region is one block of the production render entry and nothing else. The
            // output identity is taken outside it, which `timing::timed` enforces structurally.
            let (elapsed_ns, result) = timing::timed(|| runtime.render(observation as u64));
            if result.is_err() {
                render_errors += 1;
            }
            runtime.hash_output(&mut output_hash);
            durations.push(elapsed_ns);
        }
        Self {
            ns_per_block: durations,
            output_sha256: output_hash.finish_hex(),
            audit: audit::snapshot(),
            render_errors,
        }
    }

    fn record(
        &self,
        workload: Workload,
        round: u32,
        backend: Backend,
        metadata: &Metadata,
    ) -> String {
        let percentiles = Percentiles::from_samples(&self.ns_per_block);
        let tracks = f64::from(workload.tracks());
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_session\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},\"synthetic_fixture\":{synthetic},",
                "\"strip_content\":\"{strip}\",\"strip_layout\":\"{layout}\",",
                "\"input_signal\":\"{signal}\",",
                "\"fixture_id\":\"{fixture}\",\"round\":{round},\"backend\":\"{backend}\",",
                "\"sample_rate_hz\":{rate},\"quantum_frames\":{quantum},\"observations\":{obs},",
                "\"units\":\"us_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"min_us_per_block\":{min},\"p50_us_per_block\":{p50},",
                "\"p95_us_per_block\":{p95},\"p99_us_per_block\":{p99},",
                "\"max_us_per_block\":{max},\"p50_us_per_block_per_track\":{per_track},",
                "\"min_ns_per_block\":{min_ns},\"p50_ns_per_block\":{p50_ns},",
                "\"p95_ns_per_block\":{p95_ns},\"p99_ns_per_block\":{p99_ns},",
                "\"max_ns_per_block\":{max_ns},\"output_sha256\":\"{digest}\",",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"nearest-rank percentiles over per-block nanoseconds; ",
                "one warmup pass and two measured rounds; descriptive only; no threshold\"}}"
            ),
            issue = ISSUE,
            kind = workload.kind(),
            tracks = workload.tracks(),
            synthetic = workload.synthetic(),
            strip = workload.strip_content(),
            layout = workload.strip_layout(),
            signal = workload.input_signal(),
            fixture = json_escape(workload.fixture_id()),
            round = round,
            backend = backend_name(backend),
            rate = SAMPLE_RATE_HZ,
            quantum = QUANTUM,
            obs = OBSERVATIONS,
            min = microseconds(percentiles.min),
            p50 = microseconds(percentiles.p50),
            p95 = microseconds(percentiles.p95),
            p99 = microseconds(percentiles.p99),
            max = microseconds(percentiles.max),
            per_track = format_f64(percentiles.p50 as f64 / 1_000.0 / tracks),
            min_ns = percentiles.min,
            p50_ns = percentiles.p50,
            p95_ns = percentiles.p95,
            p99_ns = percentiles.p99,
            max_ns = percentiles.max,
            digest = self.output_sha256,
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
        )
    }
}

// ---------------------------------------------------------------------------------------------
// The hoist measurement: paired alternation between a stationary and a moving arm.
// ---------------------------------------------------------------------------------------------

/// Which arm of the paired comparison an observation belongs to.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Arm {
    /// No automation traffic at all: the standing baseline.
    Quiet,
    /// Every parameter restated at the value it already holds. The hoist settles it.
    Restated,
    /// Every parameter alternating between two nearby values. A window is always open.
    Moving,
}

const ARMS: [Arm; 3] = [Arm::Quiet, Arm::Restated, Arm::Moving];

/// The dB step the moving arm alternates over.
///
/// Small enough to be ordinary console traffic, large enough that the designed `f32` coefficient
/// words genuinely change -- a one-ULP step does not, which is why it cannot be the control.
const MOVING_STEP_DB: f32 = 0.25;

impl Arm {
    const fn name(self) -> &'static str {
        match self {
            Self::Quiet => "quiet",
            Self::Restated => "restated",
            Self::Moving => "moving",
        }
    }
}

struct HoistMeasurement {
    ns_per_block: [Vec<u64>; 3],
    digests: [String; 3],
}

impl HoistMeasurement {
    /// Runs the three arms **interleaved**, one observation each, round-robin.
    fn run(workload: Workload, backend: Backend) -> Self {
        let lanes = BankWidth::for_backend(backend)
            .expect("a native bank width")
            .lanes() as usize;
        // The arm carries the workload's whole track count, as the banks a session would form:
        // nine tracks is two banks (one full, one holding the ragged remainder), sixty-four is
        // eight full banks. Measuring one bank for both would report the same number twice.
        let banks = (workload.tracks() as usize).div_ceil(lanes);
        let mut arms: Vec<HoistArm> = ARMS
            .iter()
            .map(|arm| HoistArm::new(*arm, backend, lanes, banks))
            .collect();

        // Warm every arm before any of them is timed, so no arm pays first-touch inside the clock.
        for arm in &mut arms {
            for observation in 0..64 {
                arm.fill_input();
                arm.render(observation);
            }
        }

        let mut samples: [Vec<u64>; 3] = [
            Vec::with_capacity(OBSERVATIONS),
            Vec::with_capacity(OBSERVATIONS),
            Vec::with_capacity(OBSERVATIONS),
        ];
        // Paired alternation: arm 0, arm 1, arm 2, arm 0, ... Every drift the run suffers is
        // shared by all three arms, and the deltas are per-observation pairs rather than a
        // difference of two summaries taken minutes apart.
        for observation in 0..OBSERVATIONS as u64 {
            for (index, arm) in arms.iter_mut().enumerate() {
                arm.fill_input();
                let (elapsed_ns, ()) = timing::timed(|| arm.render(observation));
                arm.absorb_output();
                samples[index].push(elapsed_ns);
            }
        }

        let mut arms = arms.into_iter();
        let digests = [
            arms.next().expect("quiet arm").finish_digest(),
            arms.next().expect("restated arm").finish_digest(),
            arms.next().expect("moving arm").finish_digest(),
        ];
        // The measurement is its own class-A gate: the quiet arm and the restated arm must render
        // byte-identical output, because a restated parameter is by construction a no-op. If the
        // hoist ever changed a rendered bit, this run would say so before any number is reported.
        assert_eq!(
            digests[0],
            digests[1],
            "{}: restating a parameter changed rendered output -- the hoist is not bit-identical",
            workload.kind()
        );
        assert_ne!(
            digests[1],
            digests[2],
            "{}: the moving arm must actually move, or it is not a control arm",
            workload.kind()
        );

        Self {
            ns_per_block: samples,
            digests,
        }
    }

    fn record(
        &self,
        workload: Workload,
        round: u32,
        backend: Backend,
        metadata: &Metadata,
    ) -> String {
        let quiet = Percentiles::from_samples(&self.ns_per_block[0]);
        let restated = Percentiles::from_samples(&self.ns_per_block[1]);
        let moving = Percentiles::from_samples(&self.ns_per_block[2]);
        // The paired delta: moving minus restated, per observation, then summarised. This is the
        // ramp arithmetic the hoist elides, measured on pairs taken microseconds apart.
        let paired_delta = paired_median(&self.ns_per_block[2], &self.ns_per_block[1]);

        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_hoist\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},\"round\":{round},",
                "\"backend\":\"{backend}\",\"bank_boundary\":\"effect_bank\",",
                "\"observations\":{obs},\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"{arm0}\",\"{arm1}\",\"{arm2}\"],",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"quiet_p50_ns\":{q50},\"quiet_p99_ns\":{q99},",
                "\"restated_p50_ns\":{r50},\"restated_p95_ns\":{r95},\"restated_p99_ns\":{r99},",
                "\"moving_p50_ns\":{n50},\"moving_p95_ns\":{n95},\"moving_p99_ns\":{n99},",
                "\"paired_delta_median_ns\":{delta},",
                "\"quiet_output_sha256\":\"{qd}\",\"restated_output_sha256\":\"{rd}\",",
                "\"moving_output_sha256\":\"{nd}\",",
                "\"bit_identity\":\"quiet == restated, asserted in-run\",",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"three arms alternated per observation; nearest-rank ",
                "percentiles over per-block nanoseconds; paired delta is moving minus restated ",
                "per observation; descriptive only; no threshold\"}}"
            ),
            issue = ISSUE,
            kind = workload.kind(),
            tracks = workload.tracks(),
            round = round,
            backend = backend_name(backend),
            obs = OBSERVATIONS,
            arm0 = ARMS[0].name(),
            arm1 = ARMS[1].name(),
            arm2 = ARMS[2].name(),
            q50 = quiet.p50,
            q99 = quiet.p99,
            r50 = restated.p50,
            r95 = restated.p95,
            r99 = restated.p99,
            n50 = moving.p50,
            n95 = moving.p95,
            n99 = moving.p99,
            delta = paired_delta,
            qd = self.digests[0],
            rd = self.digests[1],
            nd = self.digests[2],
            metadata = metadata.record_fields(),
        )
    }
}

/// One arm: a bank of parametric EQs and a bank of compressors, driven with one traffic pattern.
struct HoistArm {
    width: BankWidth,
    banks: Vec<Box<dyn PreparedNativeEffectBank>>,
    left: Vec<f32>,
    right: Vec<f32>,
    spans: Vec<PreparedAutomationSpan>,
    offsets: Vec<u32>,
    alternate_spans: Vec<PreparedAutomationSpan>,
    alternate_offsets: Vec<u32>,
    /// One frozen input block, copied in before every observation.
    ///
    /// The bank renders in place, so without this the arm would be filtering its own output a
    /// thousand times over and every arm would decay to the same silence -- which is exactly what
    /// the first version of this subject did, and why its digests compared equal.
    source_left: Vec<f32>,
    source_right: Vec<f32>,
    lanes: usize,
    digest: Sha256Sink,
}

impl HoistArm {
    fn new(arm: Arm, backend: Backend, lanes: usize, banks: usize) -> Self {
        let width = BankWidth::for_backend(backend).expect("a native bank width");
        // The factory comes from the launch registry rather than from a direct dependency on the
        // effect crate: this subject measures what a session would actually instantiate, and the
        // bench crate keeps the dependency boundary it already had.
        let registry = launch_native_effect_registry_v1().expect("launch effect registry");
        let eq = registry
            .get_shared_ascii("miso.parametric-eq")
            .expect("the launch registry carries the parametric EQ");
        let eq_values: Vec<_> = (0..lanes)
            .map(|track| eq_track_values(&*eq, track))
            .collect();
        let eq_requests: Vec<_> = eq_values.iter().map(|values| eq_request(values)).collect();
        let prepared_banks: Vec<Box<dyn PreparedNativeEffectBank>> = (0..banks)
            .map(|_| {
                eq.bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests: &eq_requests,
                })
                .expect("valid eq bank request")
                .expect("the native width must bind an eq bank")
            })
            .collect();

        // Both span sets are built once, before any timing, and selected by block parity inside
        // the render call. The timed region allocates nothing.
        let build = |offset_db: f32| {
            let mut spans = Vec::new();
            let mut offsets = vec![0_u32];
            for track in 0..lanes {
                if arm != Arm::Quiet {
                    let value = eq_band0_gain(track) + offset_db;
                    spans.push(PreparedAutomationSpan {
                        kind: AutomationSpanKind::Point,
                        channel: ParameterChannel::Left,
                        // Index 3 is band 1's gain: the parameter the arms restate or move.
                        parameter_index: 3,
                        start_sample: 0,
                        end_sample: 0,
                        start_value: value,
                        end_value: value,
                    });
                }
                offsets.push(spans.len() as u32);
            }
            (spans, offsets)
        };
        let (spans, offsets) = build(0.0);
        let (alternate_spans, alternate_offsets) = match arm {
            Arm::Moving => build(MOVING_STEP_DB),
            _ => build(0.0),
        };

        Self {
            width,
            banks: prepared_banks,
            left: vec![0.0; QUANTUM * lanes * banks],
            right: vec![0.0; QUANTUM * lanes * banks],
            spans,
            offsets,
            alternate_spans,
            alternate_offsets,
            source_left: (0..QUANTUM * lanes * banks)
                .map(|index| ((index as f32) * 0.017).sin() * 0.4)
                .collect(),
            source_right: (0..QUANTUM * lanes * banks)
                .map(|index| ((index as f32) * 0.017).sin() * -0.4)
                .collect(),
            lanes,
            digest: Sha256Sink::new(),
        }
    }

    /// Restores the frozen input. Outside the clock, like every other evidence step.
    fn fill_input(&mut self) {
        self.left.copy_from_slice(&self.source_left);
        self.right.copy_from_slice(&self.source_right);
    }

    /// Folds this observation's output into the arm's running identity. Outside the clock.
    fn absorb_output(&mut self) {
        for value in self.left.iter().chain(self.right.iter()) {
            self.digest.update(value.to_bits().to_le_bytes());
        }
    }

    fn render(&mut self, observation: u64) {
        let first_sample = observation * QUANTUM as u64;
        // A point span is only admitted when it lands on the block's first sample, so the sample
        // stamp is refreshed in place each block. In place, because this runs inside the timed
        // region and the region must not allocate.
        let (spans, offsets) = if observation.is_multiple_of(2) {
            (&mut self.spans, &self.offsets)
        } else {
            (&mut self.alternate_spans, &self.alternate_offsets)
        };
        for span in spans.iter_mut() {
            span.start_sample = first_sample;
            span.end_sample = first_sample;
        }
        let spans = &*spans;
        let stride = QUANTUM * self.lanes;
        for (index, bank) in self.banks.iter_mut().enumerate() {
            let range = index * stride..(index + 1) * stride;
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut self.left[range.clone()],
                    &mut self.right[range],
                    None,
                    QUANTUM as u32,
                    self.width,
                    first_sample,
                    spans,
                    offsets,
                    128,
                )
                .expect("console hoist block"),
            );
        }
    }

    fn finish_digest(self) -> String {
        self.digest.finish_hex()
    }
}

/// Band 0's left gain for `track`, the parameter the hoist arms restate or nudge.
fn eq_band0_gain(track: usize) -> f32 {
    -7.5 + (track % 15) as f32
}

/// A prepare request for one lane of the hoist bank, at the launch rate and quantum.
fn eq_request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
    PrepareEffectRequest {
        sample_rate: SAMPLE_RATE_HZ,
        quantum: QUANTUM as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 16,
            maximum_scratch_bytes: 1 << 16,
            maximum_automation_spans_per_block: 48,
        },
    }
}

/// Band 1 enabled as a peaking filter, with its left gain at this track's held value.
///
/// Band 1 has to be **enabled explicitly**: the descriptor's default leaves it disabled, and a
/// disabled band ignores its gain entirely -- an earlier version of this subject left the defaults
/// alone and both hoist arms rendered identical audio because the parameter they were moving was
/// not in the signal path at all.
fn eq_track_values(factory: &dyn NativeEffectFactory, track: usize) -> Vec<InitialParameterValue> {
    let mut values = Vec::new();
    for (index, parameter) in factory.descriptor().parameters.iter().enumerate() {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            let value = match (index, channel) {
                // Band 1: enabled, peaking, at a per-track frequency.
                (0, _) => 1.0,
                (1, _) => 3.0,
                (2, _) => 400.0 + track as f32 * 37.0,
                (3, ParameterChannel::Left) => eq_band0_gain(track),
                (4, _) => 0.9,
                _ => parameter.default_value,
            };
            values.push(InitialParameterValue {
                parameter_index: index as u32,
                channel,
                value,
            });
        }
    }
    values
}

// ---------------------------------------------------------------------------------------------
// The console-facility measurements (#163 item 0d): meters on/off, observation armed/unarmed.
// ---------------------------------------------------------------------------------------------

/// The workload every facility arm renders. One row, the qualification session.
const FACILITY_WORKLOAD: Workload = Workload::SixtyFourTrackConsole;

/// The meter arms, in emission order.
const METER_CONFIGS: [PlanConfig; 2] = [
    PlanConfig::BASELINE,
    PlanConfig {
        meters: true,
        ..PlanConfig::BASELINE
    },
];

/// The observation arms, in emission order: the two levels of the #143 zero, then armed.
const OBSERVATION_CONFIGS: [PlanConfig; 3] = [
    PlanConfig {
        meters: false,
        control: true,
        observation: ObservationArm::Absent,
    },
    PlanConfig {
        meters: false,
        control: true,
        observation: ObservationArm::Unarmed,
    },
    PlanConfig {
        meters: false,
        control: true,
        observation: ObservationArm::Armed,
    },
];

/// A set of prepared arms of one workload, alternated observation by observation.
///
/// The #104 paired-alternation protocol, the same one the hoist arms use and for the same reason:
/// running all of arm A and then all of arm B measures the difference between two points in time
/// as much as the difference between two arms. Every arm here is a *complete* sixty-four-track
/// console plan built by `SessionRuntime::build` from one `console_model`, so two arms differ in
/// exactly the one `PlanConfig` field that separates them and in nothing else.
struct FacilityMeasurement {
    ns_per_block: Vec<Vec<u64>>,
    digests: Vec<String>,
    audit: audit::AuditSnapshot,
    render_errors: u64,
    meter_frames: u64,
    observation_lanes: usize,
    observation_taps: usize,
    published_windows: Vec<u64>,
}

impl FacilityMeasurement {
    fn run(configs: &[PlanConfig]) -> Self {
        let mut arms: Vec<SessionRuntime> = configs
            .iter()
            .map(|config| SessionRuntime::build(FACILITY_WORKLOAD, *config))
            .collect();
        let mut hashes: Vec<Sha256Sink> = configs.iter().map(|_| Sha256Sink::new()).collect();

        // Warm every arm before any of them is timed, so no arm pays first-touch inside the clock
        // -- and so the armed arm's queued subscriptions are drained by a render that is not
        // measured. Sixty-four blocks is sixteen full observation windows.
        for arm in &mut arms {
            for observation in 0..64 {
                let _ = arm.render(observation);
            }
            arm.drain_meters();
        }

        let mut samples: Vec<Vec<u64>> = configs
            .iter()
            .map(|_| Vec::with_capacity(OBSERVATIONS))
            .collect();
        let mut render_errors = 0_u64;
        let mut meter_frames = 0_u64;
        audit::warm_up();
        audit::reset();
        for observation in 0..OBSERVATIONS as u64 {
            for (index, arm) in arms.iter_mut().enumerate() {
                let (elapsed_ns, result) = timing::timed(|| arm.render(observation));
                if result.is_err() {
                    render_errors += 1;
                }
                samples[index].push(elapsed_ns);
            }
            // Every evidence step is outside the clock and after the whole round-robin, so one
            // arm's bookkeeping never lands between another arm's two timed blocks.
            for (index, arm) in arms.iter_mut().enumerate() {
                arm.hash_output(&mut hashes[index]);
                meter_frames += arm.drain_meters();
            }
        }
        let snapshot = audit::snapshot();

        let observation_lanes = arms
            .iter()
            .map(SessionRuntime::observation_lanes)
            .max()
            .unwrap_or(0);
        let observation_taps = arms
            .iter()
            .map(SessionRuntime::observation_taps)
            .max()
            .unwrap_or(0);
        let published_windows: Vec<u64> =
            arms.iter().map(SessionRuntime::published_windows).collect();
        let digests: Vec<String> = hashes.into_iter().map(Sha256Sink::finish_hex).collect();

        // The class-A statement, asserted in-run rather than only claimed in a record: attaching a
        // meter stream, attaching observation capacity and arming a tap are all *observation*. None
        // of them may change a rendered bit. If one ever did, this run says so before it reports a
        // number.
        for (index, digest) in digests.iter().enumerate() {
            assert_eq!(
                digest, &digests[0],
                "arm {index} rendered different output: an observation facility changed the signal"
            );
        }
        Self {
            ns_per_block: samples,
            digests,
            audit: snapshot,
            render_errors,
            meter_frames,
            observation_lanes,
            observation_taps,
            published_windows,
        }
    }

    fn meters_record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let off = Percentiles::from_samples(&self.ns_per_block[0]);
        let on = Percentiles::from_samples(&self.ns_per_block[1]);
        // The meter arm must actually have metered. A silently empty meter set would report a
        // delta of nothing and look like a wonderful result.
        assert!(
            self.meter_frames > 0,
            "the meters-on arm published no meter frame: it is not measuring meters"
        );
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_meters\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},\"round\":{round},",
                "\"backend\":\"{backend}\",\"observations\":{obs},",
                "\"pairing\":\"alternating_per_observation\",\"arms\":[\"meters_off\",\"meters_on\"],",
                "\"meter_streams\":{streams},\"meter_tap\":\"post_matrix\",",
                "\"meter_window_blocks\":{window},\"meter_frames_drained\":{frames},",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"meters_off_p50_ns\":{o50},\"meters_off_p95_ns\":{o95},\"meters_off_p99_ns\":{o99},",
                "\"meters_on_p50_ns\":{n50},\"meters_on_p95_ns\":{n95},\"meters_on_p99_ns\":{n99},",
                "\"paired_delta_median_ns\":{delta},",
                "\"meters_off_output_sha256\":\"{od}\",\"meters_on_output_sha256\":\"{nd}\",",
                "\"bit_identity\":\"meters_off == meters_on, asserted in-run\",",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"two arms alternated per observation; nearest-rank ",
                "percentiles over per-block nanoseconds; paired delta is meters_on minus ",
                "meters_off per observation; descriptive only; no threshold\"}}"
            ),
            issue = ISSUE,
            kind = FACILITY_WORKLOAD.kind(),
            tracks = FACILITY_WORKLOAD.tracks(),
            round = round,
            backend = backend_name(backend),
            obs = OBSERVATIONS,
            streams = FACILITY_WORKLOAD.tracks(),
            window = WINDOW_BLOCKS,
            frames = self.meter_frames,
            o50 = off.p50,
            o95 = off.p95,
            o99 = off.p99,
            n50 = on.p50,
            n95 = on.p95,
            n99 = on.p99,
            delta = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]),
            od = self.digests[0],
            nd = self.digests[1],
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
        )
    }

    fn observation_record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let absent = Percentiles::from_samples(&self.ns_per_block[0]);
        let unarmed = Percentiles::from_samples(&self.ns_per_block[1]);
        let armed = Percentiles::from_samples(&self.ns_per_block[2]);
        // The two halves of the honesty gate. An unarmed lane that published anything would not be
        // unarmed, and an armed lane that published nothing would be measuring the unarmed cost
        // twice and reporting the difference as noise.
        assert_eq!(
            self.published_windows[1], 0,
            "the unarmed arm published an observation window"
        );
        assert!(
            self.published_windows[2] > 0,
            "the armed arm published no observation window: it is not actually armed"
        );
        assert!(
            self.observation_lanes > 0 && self.observation_taps > 0,
            "no observation lane was prepared: the arms are all the level-1 zero"
        );
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_observation\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},\"round\":{round},",
                "\"backend\":\"{backend}\",\"observations\":{obs},",
                "\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"{arm0}\",\"{arm1}\",\"{arm2}\"],",
                "\"observation_lanes\":{lanes},\"observation_taps\":{taps},",
                "\"observation_window_blocks\":{window},",
                "\"unarmed_windows_published\":{uwin},\"armed_windows_published\":{awin},",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"absent_p50_ns\":{a50},\"absent_p95_ns\":{a95},\"absent_p99_ns\":{a99},",
                "\"unarmed_p50_ns\":{u50},\"unarmed_p95_ns\":{u95},\"unarmed_p99_ns\":{u99},",
                "\"armed_p50_ns\":{r50},\"armed_p95_ns\":{r95},\"armed_p99_ns\":{r99},",
                "\"paired_capacity_delta_median_ns\":{capacity},",
                "\"paired_arm_delta_median_ns\":{arming},",
                "\"absent_output_sha256\":\"{ad}\",\"unarmed_output_sha256\":\"{ud}\",",
                "\"armed_output_sha256\":\"{rd}\",",
                "\"bit_identity\":\"absent == unarmed == armed, asserted in-run\",",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"three arms alternated per observation; nearest-rank ",
                "percentiles over per-block nanoseconds; capacity delta is unarmed minus absent ",
                "and arm delta is armed minus unarmed, per observation; descriptive only; no ",
                "threshold\"}}"
            ),
            issue = ISSUE,
            kind = FACILITY_WORKLOAD.kind(),
            tracks = FACILITY_WORKLOAD.tracks(),
            round = round,
            backend = backend_name(backend),
            obs = OBSERVATIONS,
            arm0 = ObservationArm::Absent.name(),
            arm1 = ObservationArm::Unarmed.name(),
            arm2 = ObservationArm::Armed.name(),
            lanes = self.observation_lanes,
            taps = self.observation_taps,
            window = WINDOW_BLOCKS,
            uwin = self.published_windows[1],
            awin = self.published_windows[2],
            a50 = absent.p50,
            a95 = absent.p95,
            a99 = absent.p99,
            u50 = unarmed.p50,
            u95 = unarmed.p95,
            u99 = unarmed.p99,
            r50 = armed.p50,
            r95 = armed.p95,
            r99 = armed.p99,
            capacity = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]),
            arming = paired_median(&self.ns_per_block[2], &self.ns_per_block[1]),
            ad = self.digests[0],
            ud = self.digests[1],
            rd = self.digests[2],
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
        )
    }
}

// ---------------------------------------------------------------------------------------------
// The placement measurement: the #175 chain-shape row-pair, alternated observation by observation.
// ---------------------------------------------------------------------------------------------

/// The two arms of the chain-shape comparison, in record order.
///
/// `split_chains` is the retired layout: EQ on `simd1` and the compressor in the `dynamic` rack,
/// two one-slot bank chains per cohort. `merged_chain` is the standing fixture with its limiter
/// removed: the same two effects, the same coefficients, the same order, as one two-slot chain on
/// `simd1`.
///
/// They carry *identical arithmetic*. Every difference between their timings is chain shape --
/// how many planar/AoSoA round-trips the plan pays and how the slots are grouped -- and nothing
/// else. That is what makes this a paired measurement rather than two rows a reader subtracts.
const PLACEMENT_ARMS: [Workload; 2] = [
    Workload::SixtyFourTrackConsoleLegacy,
    Workload::SixtyFourTrackEqCompSimd1,
];

/// The #175 chain-shape row-pair.
///
/// Structurally a sibling of [`FacilityMeasurement`], and deliberately so: the two arms are
/// alternated observation by observation, for the reason the module header gives. This pair
/// especially needs it. The hypothesis under test predicted a *saving*, and the measured answer is
/// that there is none -- and "no difference" is precisely the claim a run-A-then-run-B benchmark
/// cannot make honestly, because the drift between two points in time is indistinguishable from
/// the effect at that scale. Alternating makes the delta a paired statistic over simultaneous
/// conditions, so a reported zero is a measured zero.
struct PlacementMeasurement {
    ns_per_block: [Vec<u64>; 2],
    digests: [String; 2],
    /// Transposes attributable to the timed region, per arm, per block.
    transposes_per_block: [u64; 2],
    audit: audit::AuditSnapshot,
    render_errors: u64,
}

impl PlacementMeasurement {
    fn run() -> Self {
        let mut arms: Vec<SessionRuntime> = PLACEMENT_ARMS
            .iter()
            .map(|workload| SessionRuntime::new(*workload))
            .collect();
        let mut hashes: Vec<Sha256Sink> =
            PLACEMENT_ARMS.iter().map(|_| Sha256Sink::new()).collect();
        for arm in &mut arms {
            for observation in 0..64 {
                let _ = arm.render(observation);
            }
        }
        // Counted across the timed region only, so the warmup's round-trips are not attributed to
        // it. Read before and after, outside the clock.
        let before: Vec<u64> = arms.iter().map(SessionRuntime::bank_transposes).collect();

        let mut samples: Vec<Vec<u64>> = PLACEMENT_ARMS
            .iter()
            .map(|_| Vec::with_capacity(OBSERVATIONS))
            .collect();
        let mut render_errors = 0_u64;
        audit::warm_up();
        audit::reset();
        for observation in 0..OBSERVATIONS as u64 {
            for (index, arm) in arms.iter_mut().enumerate() {
                let (elapsed_ns, result) = timing::timed(|| arm.render(observation));
                if result.is_err() {
                    render_errors += 1;
                }
                samples[index].push(elapsed_ns);
            }
            for (index, arm) in arms.iter_mut().enumerate() {
                arm.hash_output(&mut hashes[index]);
            }
        }
        let snapshot = audit::snapshot();
        let digests: Vec<String> = hashes.into_iter().map(Sha256Sink::finish_hex).collect();

        // The class-A statement, asserted in-run. Rack placement regroups lanes; it never changes
        // per-lane arithmetic (AGENTS.md, #166). If the two arms ever disagree, this run says so
        // before it reports a delta -- because a timing difference between two arms that compute
        // different things is not a chain-shape measurement at all.
        assert_eq!(
            digests[0], digests[1],
            "the two placements rendered different output: a placement change moved a rendered bit"
        );

        let transposes: Vec<u64> = arms
            .iter()
            .zip(before.iter())
            .map(|(arm, start)| (arm.bank_transposes() - start) / OBSERVATIONS as u64)
            .collect();

        Self {
            ns_per_block: [samples[0].clone(), samples[1].clone()],
            digests: [digests[0].clone(), digests[1].clone()],
            transposes_per_block: [transposes[0], transposes[1]],
            audit: snapshot,
            render_errors,
        }
    }

    fn record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let split = Percentiles::from_samples(&self.ns_per_block[0]);
        let merged = Percentiles::from_samples(&self.ns_per_block[1]);
        let delta = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]);
        let tracks = f64::from(PLACEMENT_ARMS[0].tracks());
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_placement\",",
                "\"workload_kind\":\"sixty_four_track_placement\",\"tracks\":{tracks},",
                "\"round\":{round},\"backend\":\"{backend}\",",
                "\"observations\":{obs},\"units\":\"ns_per_block\",",
                "\"percentile_method\":\"nearest_rank\",",
                "\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"split_chains\",\"merged_chain\"],",
                "\"split_chains_layout\":\"{split_layout}\",",
                "\"merged_chain_layout\":\"{merged_layout}\",",
                "\"split_chains_p50_ns\":{split_p50},\"split_chains_p95_ns\":{split_p95},",
                "\"split_chains_p99_ns\":{split_p99},",
                "\"merged_chain_p50_ns\":{merged_p50},\"merged_chain_p95_ns\":{merged_p95},",
                "\"merged_chain_p99_ns\":{merged_p99},",
                "\"paired_delta_median_ns\":{delta},",
                "\"paired_delta_median_ns_per_track\":{delta_per_track},",
                "\"split_chains_transposes_per_block\":{split_transposes},",
                "\"merged_chain_transposes_per_block\":{merged_transposes},",
                "\"split_chains_output_sha256\":\"{split_digest}\",",
                "\"merged_chain_output_sha256\":\"{merged_digest}\",",
                "\"bit_identity\":\"split_chains == merged_chain, asserted in-run\",",
                "\"render_errors\":{errors},",
                "\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"{method}\"}}"
            ),
            issue = ISSUE,
            tracks = PLACEMENT_ARMS[0].tracks(),
            round = round,
            backend = backend_name(backend),
            obs = OBSERVATIONS,
            split_layout = PLACEMENT_ARMS[0].strip_layout(),
            merged_layout = PLACEMENT_ARMS[1].strip_layout(),
            split_p50 = split.p50,
            split_p95 = split.p95,
            split_p99 = split.p99,
            merged_p50 = merged.p50,
            merged_p95 = merged.p95,
            merged_p99 = merged.p99,
            delta = delta,
            delta_per_track = format_f64(delta as f64 / tracks),
            split_transposes = self.transposes_per_block[0],
            merged_transposes = self.transposes_per_block[1],
            split_digest = self.digests[0],
            merged_digest = self.digests[1],
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
            method = PLACEMENT_STATISTICAL_METHOD,
        )
    }
}

/// Pinned verbatim in the validator, for the reason every method sentence in this stream is.
const PLACEMENT_STATISTICAL_METHOD: &str = "two arms alternated per observation; nearest-rank \
percentiles over per-block nanoseconds; paired delta is merged_chain minus split_chains per \
observation; descriptive only; no threshold";

// ---------------------------------------------------------------------------------------------
// The compressor-automation measurement: the ramping body no standing row can see.
// ---------------------------------------------------------------------------------------------

/// The workload every automation arm renders: the compressor decomposition row.
///
/// Deliberately the decomposition row and not the whole strip. The subject here is the
/// compressor's *ramping body*, and a row that also carries an EQ and a limiter buries a
/// microsecond of it under two other effects' steady-state cost.
const AUTOMATION_WORKLOAD: Workload = Workload::SixtyFourTrackCompressorOnly;

/// The plan every automation arm is prepared with: the live-console control channel and nothing
/// else.
///
/// All three arms carry the channel, so the deltas between them are the *traffic* and never the
/// queue's presence -- the same discipline `console_observation` uses for its three arms.
const AUTOMATION_CONFIG: PlanConfig = PlanConfig {
    meters: false,
    control: true,
    observation: ObservationArm::Absent,
};

/// The effect the arms address, by contract id rather than by session slot name.
const AUTOMATION_EFFECT_ID: &str = "miso.compressor";

/// Descriptor index of the compressor's `threshold`. An index into the parameter table, never the
/// wire `parameter_id`.
const AUTOMATION_PARAMETER_INDEX: u32 = 0;

/// The parameter's descriptor name, recorded so a reader does not have to resolve the index.
const AUTOMATION_PARAMETER_NAME: &str = "threshold";

/// The threshold, in dB, every arm is settled at before the clock starts.
///
/// Inside the descriptor's `[-80, 0]` domain and far enough below the source tone (0.6 peak, less
/// a -6 dB input trim) that the static curve is genuinely engaged. A threshold the signal never
/// reaches would make the arms differ in a coefficient nothing reads, and the in-run inequality
/// gate below would catch it -- but as a failure, after a run.
const AUTOMATION_BASE_DB: f32 = -24.0;

/// The dB step the automated arm alternates over, either side of the base.
///
/// It alternates `base + step`, `base - step`, `base + step`, ... rather than restating the base
/// on every other block, so that **every** timed block opens a real smoothing window. Alternating
/// against the base itself would let the #144 stationary hoist settle half the blocks and the row
/// would report half the ramping cost it claims to.
const AUTOMATION_STEP_DB: f32 = 0.5;

/// Which traffic pattern an automation arm delivers.
#[derive(Clone, Copy, PartialEq, Eq)]
enum AutomationArm {
    /// The channel is attached and nothing is ever pushed. The standing baseline.
    Quiet,
    /// The threshold restated at the value it already holds, every block. The #144 hoist settles
    /// it, so no window ever opens and this arm must render `quiet`'s bits exactly.
    Restated,
    /// The threshold moved every block, so a `Linear 64` window is open on every block of the run.
    Automated,
}

const AUTOMATION_ARMS: [AutomationArm; 3] = [
    AutomationArm::Quiet,
    AutomationArm::Restated,
    AutomationArm::Automated,
];

impl AutomationArm {
    const fn name(self) -> &'static str {
        match self {
            Self::Quiet => "quiet",
            Self::Restated => "restated",
            Self::Automated => "automated",
        }
    }

    /// The value this arm pushes before block `observation`, or `None` if it pushes nothing.
    fn value(self, observation: u64) -> Option<f32> {
        match self {
            Self::Quiet => None,
            Self::Restated => Some(AUTOMATION_BASE_DB),
            Self::Automated => Some(if observation.is_multiple_of(2) {
                AUTOMATION_BASE_DB + AUTOMATION_STEP_DB
            } else {
                AUTOMATION_BASE_DB - AUTOMATION_STEP_DB
            }),
        }
    }
}

/// The automation-active row: one Point span per block, on one track, through a real plan.
///
/// # Why this measurement had to be added
///
/// Every existing row of this stream renders with automation cleared -- `console_model` clears the
/// fixture's table unconditionally and both fixture gates assert the standing sessions declare
/// none -- so the compressor's ramping body is dead code in all of them. The one arm that does
/// deliver automation, `console_hoist`, drives banks of *parametric EQs*; no compressor in this
/// benchmark has ever seen a span. A change to the compressor's ramping path is therefore
/// invisible to the whole standing table, which is exactly the gap this row closes.
///
/// # What one Point span per block on one track actually exercises
///
/// Sixty-four tracks is eight eight-lane banks. Automating one track's left channel puts a
/// `Linear 64` window in flight on one lane of one bank, and the compressor cuts its block into a
/// ramping prefix and an idle remainder from the longest ramp anywhere in *either* channel of that
/// bank. So one automated lane drags fifteen unautomated channel-lanes of that bank through the
/// ramping body for sixty-four of the block's hundred and twenty-eight frames, while the other
/// seven banks stay entirely on the idle body. That asymmetry is the realistic console case -- a
/// console rides one parameter on one track, not sixty-four at once -- and it is the case whose
/// cost no other row in this file reports.
struct AutomationMeasurement {
    ns_per_block: [Vec<u64>; 3],
    digests: [String; 3],
    /// Pushes the bounded control queue accepted, per arm. A refused push is a measured lie.
    accepted: [u64; 3],
    track_id: String,
    effect_id: String,
    audit: audit::AuditSnapshot,
    render_errors: u64,
}

impl AutomationMeasurement {
    fn run() -> Self {
        let mut arms: Vec<SessionRuntime> = AUTOMATION_ARMS
            .iter()
            .map(|_| SessionRuntime::build(AUTOMATION_WORKLOAD, AUTOMATION_CONFIG))
            .collect();
        let channel = arms[0]
            .first_track_control_channel(AUTOMATION_EFFECT_ID)
            .expect("the compressor decomposition row prepares a compressor control channel");
        let (track_id, effect_id) = {
            let (track, effect) = arms[0].control_identity(channel);
            (track.to_owned(), effect.to_owned())
        };

        // Untimed pre-roll. Every arm -- `quiet` included -- is settled at the same base threshold
        // before the clock starts, which is what makes `quiet` and `restated` comparable at all:
        // `restated` can only be bit-identical to a baseline that holds the value it restates.
        // Sixty-four blocks is far more than the sixty-four *samples* the window takes.
        for arm in &mut arms {
            assert!(
                arm.push_parameter(
                    channel,
                    AUTOMATION_PARAMETER_INDEX,
                    ParameterChannel::Left,
                    AUTOMATION_BASE_DB,
                ),
                "the bounded control queue refused the pre-roll push"
            );
            for observation in 0..64 {
                let _ = arm.render(observation);
            }
        }

        let mut hashes: Vec<Sha256Sink> =
            AUTOMATION_ARMS.iter().map(|_| Sha256Sink::new()).collect();
        let mut samples: Vec<Vec<u64>> = AUTOMATION_ARMS
            .iter()
            .map(|_| Vec::with_capacity(OBSERVATIONS))
            .collect();
        let mut accepted = [0_u64; 3];
        let mut render_errors = 0_u64;
        audit::warm_up();
        audit::reset();
        for observation in 0..OBSERVATIONS as u64 {
            for (index, arm) in arms.iter_mut().enumerate() {
                // The push is control-plane work and stays outside the clock, exactly where a host
                // does it. What the timed region pays for is the render-side drain, the span it
                // stages and the ramping body that span opens -- which is the cost under test.
                if let Some(value) = AUTOMATION_ARMS[index].value(observation)
                    && arm.push_parameter(
                        channel,
                        AUTOMATION_PARAMETER_INDEX,
                        ParameterChannel::Left,
                        value,
                    )
                {
                    accepted[index] += 1;
                }
                let (elapsed_ns, result) = timing::timed(|| arm.render(observation));
                if result.is_err() {
                    render_errors += 1;
                }
                samples[index].push(elapsed_ns);
            }
            for (index, arm) in arms.iter_mut().enumerate() {
                arm.hash_output(&mut hashes[index]);
            }
        }
        let snapshot = audit::snapshot();
        let digests: Vec<String> = hashes.into_iter().map(Sha256Sink::finish_hex).collect();

        // The class-A statement, asserted in-run. Restating a parameter at the value it already
        // holds is by construction a no-op, and the #144 hoist is what makes it free; if it ever
        // moved a rendered bit, this run says so before it reports a number. This is the first
        // time that statement is made about the *compressor*, through a real prepared plan.
        assert_eq!(
            digests[0], digests[1],
            "restating the compressor's threshold changed rendered output: the hoist is not \
             bit-identical"
        );
        // And the honesty half: an arm that renders the restated arm's bits is not automating.
        assert_ne!(
            digests[1], digests[2],
            "the automated arm rendered the restated arm's output: no window ever opened"
        );
        // A silently refused push would report the cost of automation that never arrived.
        assert_eq!(accepted[0], 0, "the quiet arm pushed a record");
        for index in [1, 2] {
            assert_eq!(
                accepted[index],
                OBSERVATIONS as u64,
                "the {} arm's bounded control queue refused a push",
                AUTOMATION_ARMS[index].name()
            );
        }

        Self {
            ns_per_block: [samples[0].clone(), samples[1].clone(), samples[2].clone()],
            digests: [digests[0].clone(), digests[1].clone(), digests[2].clone()],
            accepted,
            track_id,
            effect_id,
            audit: snapshot,
            render_errors,
        }
    }

    fn record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let quiet = Percentiles::from_samples(&self.ns_per_block[0]);
        let restated = Percentiles::from_samples(&self.ns_per_block[1]);
        let automated = Percentiles::from_samples(&self.ns_per_block[2]);
        // The row's headline: the ramping surcharge, taken against an arm that delivers the *same*
        // control traffic and differs only in whether that traffic opens a window.
        let ramp = paired_median(&self.ns_per_block[2], &self.ns_per_block[1]);
        let control = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]);
        let tracks = f64::from(AUTOMATION_WORKLOAD.tracks());
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_automation\",",
                "\"workload_kind\":\"sixty_four_track_compressor_automation\",\"tracks\":{tracks},",
                "\"synthetic_fixture\":{synthetic},\"strip_content\":\"{strip}\",",
                "\"strip_layout\":\"{layout}\",\"input_signal\":\"{signal}\",",
                "\"fixture_id\":\"{fixture}\",\"round\":{round},\"backend\":\"{backend}\",",
                "\"sample_rate_hz\":{rate},\"quantum_frames\":{quantum},",
                "\"observations\":{obs},\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"{arm0}\",\"{arm1}\",\"{arm2}\"],",
                "\"automated_track_id\":\"{track}\",\"automated_effect_id\":\"{effect}\",",
                "\"automated_effect\":\"{contract}\",\"automated_parameter\":\"{parameter}\",",
                "\"automated_parameter_index\":{parameter_index},",
                "\"automated_channel\":\"left\",\"automation_spans_per_block\":1,",
                "\"smoothing_samples\":{smoothing},",
                "\"restated_pushes_accepted\":{restated_pushes},",
                "\"automated_pushes_accepted\":{automated_pushes},",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"quiet_p50_ns\":{q50},\"quiet_p95_ns\":{q95},\"quiet_p99_ns\":{q99},",
                "\"restated_p50_ns\":{r50},\"restated_p95_ns\":{r95},\"restated_p99_ns\":{r99},",
                "\"automated_p50_ns\":{a50},\"automated_p95_ns\":{a95},\"automated_p99_ns\":{a99},",
                "\"paired_ramp_delta_median_ns\":{ramp},",
                "\"paired_ramp_delta_median_ns_per_track\":{ramp_per_track},",
                "\"paired_control_delta_median_ns\":{control},",
                "\"quiet_output_sha256\":\"{qd}\",\"restated_output_sha256\":\"{rd}\",",
                "\"automated_output_sha256\":\"{ad}\",",
                "\"bit_identity\":\"quiet == restated, asserted in-run\",",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"{method}\"}}"
            ),
            issue = ISSUE,
            tracks = AUTOMATION_WORKLOAD.tracks(),
            synthetic = AUTOMATION_WORKLOAD.synthetic(),
            strip = AUTOMATION_WORKLOAD.strip_content(),
            layout = AUTOMATION_WORKLOAD.strip_layout(),
            signal = AUTOMATION_WORKLOAD.input_signal(),
            fixture = json_escape(AUTOMATION_WORKLOAD.fixture_id()),
            round = round,
            backend = backend_name(backend),
            rate = SAMPLE_RATE_HZ,
            quantum = QUANTUM,
            obs = OBSERVATIONS,
            arm0 = AUTOMATION_ARMS[0].name(),
            arm1 = AUTOMATION_ARMS[1].name(),
            arm2 = AUTOMATION_ARMS[2].name(),
            track = json_escape(&self.track_id),
            effect = json_escape(&self.effect_id),
            contract = AUTOMATION_EFFECT_ID,
            parameter = AUTOMATION_PARAMETER_NAME,
            parameter_index = AUTOMATION_PARAMETER_INDEX,
            smoothing = AUTOMATION_SMOOTHING_SAMPLES,
            restated_pushes = self.accepted[1],
            automated_pushes = self.accepted[2],
            q50 = quiet.p50,
            q95 = quiet.p95,
            q99 = quiet.p99,
            r50 = restated.p50,
            r95 = restated.p95,
            r99 = restated.p99,
            a50 = automated.p50,
            a95 = automated.p95,
            a99 = automated.p99,
            ramp = ramp,
            ramp_per_track = format_f64(ramp as f64 / tracks),
            control = control,
            qd = self.digests[0],
            rd = self.digests[1],
            ad = self.digests[2],
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
            method = AUTOMATION_STATISTICAL_METHOD,
        )
    }
}

/// The compressor descriptor's `smoothing_samples` for every automatable parameter.
///
/// Recorded so the row states the window length its surcharge is the cost of, rather than leaving
/// a reader to look it up in the effect crate.
const AUTOMATION_SMOOTHING_SAMPLES: u32 = 64;

/// Pinned verbatim in the validator, for the reason every method sentence in this stream is.
const AUTOMATION_STATISTICAL_METHOD: &str = "three arms alternated per observation; nearest-rank \
percentiles over per-block nanoseconds; ramp delta is automated minus restated and control delta \
is restated minus quiet, per observation; descriptive only; no threshold";

/// The median of the per-observation differences `left[i] - right[i]`.
///
/// Pairs taken microseconds apart, then summarised -- never a difference of two summaries taken
/// minutes apart, which is the whole point of alternating the arms.
fn paired_median(left: &[u64], right: &[u64]) -> i64 {
    let mut paired: Vec<i64> = left
        .iter()
        .zip(right)
        .map(|(left, right)| *left as i64 - *right as i64)
        .collect();
    paired.sort_unstable();
    paired[paired.len() / 2]
}

// ---------------------------------------------------------------------------------------------
// Shared plumbing.
// ---------------------------------------------------------------------------------------------

struct Percentiles {
    min: u64,
    p50: u64,
    p95: u64,
    p99: u64,
    max: u64,
}

impl Percentiles {
    fn from_samples(samples: &[u64]) -> Self {
        let mut sorted = samples.to_vec();
        sorted.sort_unstable();
        assert!(!sorted.is_empty(), "measured observations");
        let rank = |numerator: usize, denominator: usize| {
            stats::nearest_rank(&sorted, numerator, denominator)
        };
        Self {
            min: sorted[0],
            p50: rank(50, 100),
            p95: rank(95, 100),
            p99: rank(99, 100),
            max: *sorted.last().expect("nonempty"),
        }
    }
}

fn microseconds(nanoseconds: u64) -> String {
    format_f64(nanoseconds as f64 / 1_000.0)
}

fn format_f64(value: f64) -> String {
    format!("{value:.3}")
}

fn backend_name(backend: Backend) -> &'static str {
    match backend {
        Backend::Scalar => "Scalar",
        Backend::Simd4 => "Simd4",
        Backend::Simd8 => "Simd8",
    }
}
