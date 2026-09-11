//! Bounded descriptive timing for the causal launch compressor (issue #737).
//!
//! The no-argument invocation is the one native timing workload. `--preflight` runs the same
//! preparation, input, processing and activity checks with sixteen untimed blocks per arm. The
//! process call is the only operation between the two clock boundaries in a measured block.

use compressor::{COMPRESSOR_PARAMETERS, CompressorFactory};
use conformance::SplitMix64;
use effect_contract::{
    BankProcessReport, BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LinkMode, NativeEffectFactory, ParameterChannel, PortId,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedPorts,
    PreparedSidechainPort, ProcessReport,
};
use lane::Backend;

const SAMPLE_RATE: u32 = 48_000;
const FRAMES: usize = 128;
const BLOCKS: usize = 4_096;
const WARMUP_BLOCKS: usize = 512;
const PREFLIGHT_BLOCKS: usize = 16;
const PREFLIGHT_WARMUP_BLOCKS: usize = 8;
const ROUNDS: usize = 2;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Preflight,
    Measure,
}

#[derive(Clone, Copy, Default)]
struct ReportTotals {
    sanitized_main_samples: u64,
    sanitized_sidechain_samples: u64,
    invalid_spans: u64,
    nonfinite_left_blocks: u64,
    nonfinite_right_blocks: u64,
}

impl ReportTotals {
    fn add(&mut self, report: ProcessReport) {
        self.sanitized_main_samples = self
            .sanitized_main_samples
            .saturating_add(report.sanitized_main_samples);
        self.sanitized_sidechain_samples = self
            .sanitized_sidechain_samples
            .saturating_add(report.sanitized_sidechain_samples);
        self.invalid_spans = self.invalid_spans.saturating_add(report.invalid_spans);
        self.nonfinite_left_blocks = self
            .nonfinite_left_blocks
            .saturating_add(report.nonfinite_left_blocks);
        self.nonfinite_right_blocks = self
            .nonfinite_right_blocks
            .saturating_add(report.nonfinite_right_blocks);
    }

    fn add_bank(&mut self, report: BankProcessReport, lane: usize) {
        self.add(report.reports[lane]);
    }
}

#[derive(Clone, Copy, Default)]
struct LaneActivity {
    input_left: f64,
    input_right: f64,
    output_left: f64,
    output_right: f64,
}

impl LaneActivity {
    fn add_scalar(
        &mut self,
        source_left: &[f32],
        source_right: &[f32],
        left: &[f32],
        right: &[f32],
    ) {
        self.input_left += energy(source_left);
        self.input_right += energy(source_right);
        self.output_left += energy(left);
        self.output_right += energy(right);
    }

    fn add_bank(
        &mut self,
        source_left: &[f32],
        source_right: &[f32],
        left: &[f32],
        right: &[f32],
        lane: usize,
        width: usize,
    ) {
        self.input_left += lane_energy(source_left, lane, width);
        self.input_right += lane_energy(source_right, lane, width);
        self.output_left += lane_energy(left, lane, width);
        self.output_right += lane_energy(right, lane, width);
    }

    fn active_left(self) -> bool {
        self.input_left.is_finite()
            && self.output_left.is_finite()
            && self.input_left > 0.0
            && self.output_left > 0.0
            && self.output_left < self.input_left
    }

    fn active_right(self) -> bool {
        self.input_right.is_finite()
            && self.output_right.is_finite()
            && self.input_right > 0.0
            && self.output_right > 0.0
            && self.output_right < self.input_right
    }
}

#[derive(Clone, Copy, Default)]
struct MeasuredRound {
    elapsed_nanoseconds: u64,
    activity: LaneActivity,
}

struct BankRun {
    rounds: [MeasuredRound; ROUNDS],
    report_totals: [ReportTotals; 8],
    round_activities: [[LaneActivity; 8]; ROUNDS],
    preflight_activities: [LaneActivity; 8],
    lanes: usize,
}

fn initial_values() -> [InitialParameterValue; 14] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: COMPRESSOR_PARAMETERS[index / 2].default_value,
    })
}

fn request<'a>(values: &'a [InitialParameterValue]) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate: SAMPLE_RATE,
        quantum: FRAMES as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::Unconnected {
                id: PortId::new("sidechain-in").expect("static port id"),
                required: false,
            },
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 16,
        },
    }
}

fn noise(samples: usize, seed: u64) -> Vec<f32> {
    let mut generator = SplitMix64::new(seed);
    (0..samples)
        .map(|_| generator.next_bipolar_f32() * 0.5)
        .collect()
}

fn energy(samples: &[f32]) -> f64 {
    samples
        .iter()
        .map(|&sample| f64::from(sample) * f64::from(sample))
        .sum()
}

fn lane_energy(samples: &[f32], lane: usize, width: usize) -> f64 {
    samples
        .iter()
        .skip(lane)
        .step_by(width)
        .map(|&sample| f64::from(sample) * f64::from(sample))
        .sum()
}

fn parse_mode() -> Mode {
    let mut arguments = std::env::args_os();
    let _program = arguments.next();
    match (arguments.next(), arguments.next()) {
        (None, None) => Mode::Measure,
        (Some(argument), None) if argument == "--preflight" => Mode::Preflight,
        _ => {
            eprintln!("usage: lane_sample_timing [--preflight]");
            std::process::exit(2);
        }
    }
}

fn process_report_is_clean(report: ReportTotals) -> bool {
    report.sanitized_main_samples == 0
        && report.sanitized_sidechain_samples == 0
        && report.invalid_spans == 0
        && report.nonfinite_left_blocks == 0
        && report.nonfinite_right_blocks == 0
}

fn all_reports_clean(reports: &[ReportTotals; 8], lanes: usize) -> bool {
    (0..lanes).all(|lane| process_report_is_clean(reports[lane]))
}

fn run_scalar(mode: Mode) -> ([MeasuredRound; ROUNDS], ReportTotals, LaneActivity) {
    let values = initial_values();
    let mut effect = CompressorFactory
        .prepare(request(&values))
        .expect("scalar prepare");

    // The source planes never change. The process planes are copied before every block and are
    // the only buffers handed to the effect.
    let source_left = noise(FRAMES, 0x5EED_0001);
    let source_right = noise(FRAMES, 0x5EED_0002);
    let mut left = vec![0.0_f32; FRAMES];
    let mut right = vec![0.0_f32; FRAMES];
    let mut first_sample = 0_u64;
    let mut report_totals = ReportTotals::default();
    let mut rounds = [MeasuredRound::default(); ROUNDS];
    let mut preflight_activity = LaneActivity::default();

    let warmup = match mode {
        Mode::Preflight => PREFLIGHT_WARMUP_BLOCKS,
        Mode::Measure => WARMUP_BLOCKS,
    };
    let total = match mode {
        Mode::Preflight => PREFLIGHT_BLOCKS,
        Mode::Measure => warmup + (ROUNDS * BLOCKS),
    };
    for block in 0..total {
        left.copy_from_slice(&source_left);
        right.copy_from_slice(&source_right);
        let process_block = EffectProcessBlock::new(
            &mut left,
            &mut right,
            None,
            first_sample,
            &[],
            FRAMES as u32,
        )
        .expect("scalar process block");
        let measured_round = if mode == Mode::Measure && block >= warmup {
            Some((block - warmup) / BLOCKS)
        } else {
            None
        };
        let start = measured_round.map(|_| std::time::Instant::now());
        let report = effect.process(process_block);
        let elapsed = start.map(|clock| clock.elapsed().as_nanos() as u64);
        report_totals.add(report);
        if block >= warmup {
            if let Some(round) = measured_round {
                rounds[round].elapsed_nanoseconds = rounds[round]
                    .elapsed_nanoseconds
                    .saturating_add(elapsed.expect("measured clock"));
                rounds[round]
                    .activity
                    .add_scalar(&source_left, &source_right, &left, &right);
            } else {
                preflight_activity.add_scalar(&source_left, &source_right, &left, &right);
            }
        }
        first_sample = first_sample
            .checked_add(FRAMES as u64)
            .expect("sample time overflow");
    }

    (rounds, report_totals, preflight_activity)
}

fn run_bank(mode: Mode, backend: Backend, width: BankWidth) -> Option<BankRun> {
    let lanes = width.lanes() as usize;
    let values = vec![initial_values(); lanes];
    let requests = values.iter().map(|item| request(item)).collect::<Vec<_>>();
    let mut bank = CompressorFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .expect("bank prepare")?;

    let source_left = noise(FRAMES * lanes, 0x5EED_0003);
    let source_right = noise(FRAMES * lanes, 0x5EED_0004);
    let mut left = vec![0.0_f32; FRAMES * lanes];
    let mut right = vec![0.0_f32; FRAMES * lanes];
    let offsets = vec![0_u32; lanes + 1];
    let mut first_sample = 0_u64;
    let mut report_totals = [ReportTotals::default(); 8];
    let mut rounds = [MeasuredRound::default(); ROUNDS];
    let mut round_activities = [[LaneActivity::default(); 8]; ROUNDS];
    let mut preflight_activities = [LaneActivity::default(); 8];

    let warmup = match mode {
        Mode::Preflight => PREFLIGHT_WARMUP_BLOCKS,
        Mode::Measure => WARMUP_BLOCKS,
    };
    let total = match mode {
        Mode::Preflight => PREFLIGHT_BLOCKS,
        Mode::Measure => warmup + (ROUNDS * BLOCKS),
    };
    for block in 0..total {
        left.copy_from_slice(&source_left);
        right.copy_from_slice(&source_right);
        let process_block = EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            FRAMES as u32,
            width,
            first_sample,
            &[],
            &offsets,
            FRAMES as u32,
        )
        .expect("bank process block");
        let measured_round = if mode == Mode::Measure && block >= warmup {
            Some((block - warmup) / BLOCKS)
        } else {
            None
        };
        let start = measured_round.map(|_| std::time::Instant::now());
        let report = bank.process_bank(process_block);
        let elapsed = start.map(|clock| clock.elapsed().as_nanos() as u64);
        for (lane, report_total) in report_totals.iter_mut().enumerate().take(lanes) {
            report_total.add_bank(report, lane);
        }
        if block >= warmup {
            for lane in 0..lanes {
                if let Some(round) = measured_round {
                    round_activities[round][lane].add_bank(
                        &source_left,
                        &source_right,
                        &left,
                        &right,
                        lane,
                        lanes,
                    );
                } else {
                    preflight_activities[lane].add_bank(
                        &source_left,
                        &source_right,
                        &left,
                        &right,
                        lane,
                        lanes,
                    );
                }
            }
            if let Some(round) = measured_round {
                rounds[round].elapsed_nanoseconds = rounds[round]
                    .elapsed_nanoseconds
                    .saturating_add(elapsed.expect("measured clock"));
            }
        }
        first_sample = first_sample
            .checked_add(FRAMES as u64)
            .expect("sample time overflow");
    }

    Some(BankRun {
        rounds,
        report_totals,
        round_activities,
        preflight_activities,
        lanes,
    })
}

fn json_bool(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

fn emit_activity(activity: LaneActivity, report: ReportTotals, lane: usize) {
    print!(
        "{{\"lane\":{lane},\"input_energy\":{{\"left\":{},\"right\":{}}},\"output_energy\":{{\"left\":{},\"right\":{}}},\"active\":{{\"left\":{},\"right\":{}}},\"process_report\":{{\"sanitized_main_samples\":{},\"sanitized_sidechain_samples\":{},\"invalid_spans\":{},\"nonfinite_left_blocks\":{},\"nonfinite_right_blocks\":{}}}}}",
        activity.input_left,
        activity.input_right,
        activity.output_left,
        activity.output_right,
        json_bool(activity.active_left()),
        json_bool(activity.active_right()),
        report.sanitized_main_samples,
        report.sanitized_sidechain_samples,
        report.invalid_spans,
        report.nonfinite_left_blocks,
        report.nonfinite_right_blocks,
    );
}

fn emit_preflight_scalar(activity: LaneActivity, report: ReportTotals) {
    print!(
        "{{\"record_type\":\"preflight\",\"arm\":\"scalar\",\"sample_rate\":{SAMPLE_RATE},\"frames\":{FRAMES},\"preflight_block_count\":{PREFLIGHT_BLOCKS},\"bank_width\":1,\"clock_calls\":0,\"activity\":[",
    );
    emit_activity(activity, report, 0);
    println!("]}}");
}

fn emit_preflight_bank(activities: &[LaneActivity; 8], reports: &[ReportTotals; 8], lanes: usize) {
    print!(
        "{{\"record_type\":\"preflight\",\"arm\":\"bank\",\"sample_rate\":{SAMPLE_RATE},\"frames\":{FRAMES},\"preflight_block_count\":{PREFLIGHT_BLOCKS},\"bank_width\":{lanes},\"clock_calls\":0,\"activity\":[",
    );
    for lane in 0..lanes {
        if lane != 0 {
            print!(",");
        }
        emit_activity(activities[lane], reports[lane], lane);
    }
    println!("]}}");
}

fn emit_unavailable(mode: Mode) {
    let record_type = match mode {
        Mode::Preflight => "preflight",
        Mode::Measure => "measurement",
    };
    println!(
        "{{\"record_type\":\"{record_type}\",\"arm\":\"bank\",\"available\":false,\"sample_rate\":{SAMPLE_RATE},\"frames\":{FRAMES},\"bank_width\":null,\"reason\":\"unsupported_backend\"}}"
    );
}

fn emit_measurement_scalar(round: usize, measured: MeasuredRound, report: ReportTotals) {
    let denominator = (BLOCKS * FRAMES * 2) as u64;
    let nanoseconds_per_block = measured.elapsed_nanoseconds as f64 / BLOCKS as f64;
    let nanoseconds_per_lane_sample = measured.elapsed_nanoseconds as f64 / denominator as f64;
    print!(
        "{{\"record_type\":\"measurement\",\"arm\":\"scalar\",\"round\":{},\"sample_rate\":{SAMPLE_RATE},\"frames\":{FRAMES},\"measured_block_count\":{BLOCKS},\"bank_width\":1,\"elapsed_nanoseconds\":{},\"lane_sample_denominator\":{denominator},\"nanoseconds_per_block\":{},\"nanoseconds_per_lane_sample\":{},\"activity\":[",
        round + 1,
        measured.elapsed_nanoseconds,
        nanoseconds_per_block,
        nanoseconds_per_lane_sample,
    );
    emit_activity(measured.activity, report, 0);
    println!("]}}");
}

fn emit_measurement_bank(
    round: usize,
    measured: MeasuredRound,
    activities: &[LaneActivity; 8],
    reports: &[ReportTotals; 8],
    lanes: usize,
) {
    let denominator = (BLOCKS * FRAMES * 2 * lanes) as u64;
    let nanoseconds_per_block = measured.elapsed_nanoseconds as f64 / BLOCKS as f64;
    let nanoseconds_per_lane_sample = measured.elapsed_nanoseconds as f64 / denominator as f64;
    print!(
        "{{\"record_type\":\"measurement\",\"arm\":\"bank\",\"round\":{},\"sample_rate\":{SAMPLE_RATE},\"frames\":{FRAMES},\"measured_block_count\":{BLOCKS},\"bank_width\":{lanes},\"elapsed_nanoseconds\":{},\"lane_sample_denominator\":{denominator},\"nanoseconds_per_block\":{},\"nanoseconds_per_lane_sample\":{},\"activity\":[",
        round + 1,
        measured.elapsed_nanoseconds,
        nanoseconds_per_block,
        nanoseconds_per_lane_sample,
    );
    for lane in 0..lanes {
        if lane != 0 {
            print!(",");
        }
        emit_activity(activities[lane], reports[lane], lane);
    }
    println!("]}}");
}

fn main() {
    let mode = parse_mode();
    let backend = Backend::current();
    let width = BankWidth::for_backend(backend);

    match mode {
        Mode::Preflight => {
            let (_, scalar_report, scalar_activity) = run_scalar(mode);
            if !process_report_is_clean(scalar_report)
                || !scalar_activity.active_left()
                || !scalar_activity.active_right()
            {
                eprintln!("scalar preflight activity or process report failed");
                std::process::exit(1);
            }
            emit_preflight_scalar(scalar_activity, scalar_report);
            match width.and_then(|bank_width| run_bank(mode, backend, bank_width)) {
                Some(BankRun {
                    preflight_activities: activities,
                    report_totals: report,
                    lanes,
                    ..
                }) => {
                    if !all_reports_clean(&report, lanes)
                        || (0..lanes).any(|lane| {
                            !activities[lane].active_left() || !activities[lane].active_right()
                        })
                    {
                        eprintln!("bank preflight activity or process report failed");
                        std::process::exit(1);
                    }
                    emit_preflight_bank(&activities, &report, lanes);
                }
                None => emit_unavailable(mode),
            }
        }
        Mode::Measure => {
            let (rounds, scalar_report, _) = run_scalar(mode);
            if !process_report_is_clean(scalar_report)
                || rounds
                    .iter()
                    .any(|round| !round.activity.active_left() || !round.activity.active_right())
            {
                eprintln!("scalar measurement activity or process report failed");
                std::process::exit(1);
            }
            for (round, measured) in rounds.into_iter().enumerate() {
                emit_measurement_scalar(round, measured, scalar_report);
            }
            match width.and_then(|bank_width| run_bank(mode, backend, bank_width)) {
                Some(BankRun {
                    rounds,
                    report_totals: report,
                    round_activities,
                    lanes,
                    ..
                }) => {
                    if !all_reports_clean(&report, lanes)
                        || round_activities.iter().any(|activities| {
                            (0..lanes).any(|lane| {
                                !activities[lane].active_left() || !activities[lane].active_right()
                            })
                        })
                    {
                        eprintln!("bank measurement activity or process report failed");
                        std::process::exit(1);
                    }
                    for (round, measured) in rounds.into_iter().enumerate() {
                        emit_measurement_bank(
                            round,
                            measured,
                            &round_activities[round],
                            &report,
                            lanes,
                        );
                    }
                }
                None => emit_unavailable(mode),
            }
        }
    }
}
