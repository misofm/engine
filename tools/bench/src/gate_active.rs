//! Active causal gate/expander workload for issue #746.
//!
//! The subject deliberately has one narrow workload shape.  Preparation and the deterministic
//! source fill happen outside the clock; the only operation inside [`bench_support::timing::timed`]
//! is the prepared effect's process call.  The same subject can run an untimed preflight, one
//! untimed warmup, or one of the two measured rounds selected by the issue runner.

use bench_support::alloc as bench_alloc;
use bench_support::metadata::Metadata;
use bench_support::stats::Percentiles;
use bench_support::timing;
use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    BankProcessReport, BankWidth, EffectBankProcessBlock, EffectDescriptor, EffectPrepareError,
    EffectProcessBlock, EffectQuality, InitialParameterValue, LatencySamples, LinkMode,
    ParameterChannel, ParameterId, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedNativeEffect, PreparedNativeEffectBank, PreparedPorts,
    PreparedSidechainPort, ProcessReport,
};
use lane::Backend;

const ISSUE: u32 = 746;
const SAMPLE_RATE: u32 = 48_000;
const QUANTUM: usize = 128;
const WARMUP_BLOCKS: u64 = 8_192;
const MEASURED_BLOCKS: u64 = 32_768;
const PREFLIGHT_BLOCKS: u64 = 128;
const WIDTH_SCALAR: usize = 1;
const WIDTH_BANK: usize = 8;
const CHANNELS: usize = 2;
const HIGH_MAGNITUDE: f32 = 0.5;
const LOW_MAGNITUDE: f32 = 0.000_976_562_5;
const HIGH_RATIO_LIMIT: f64 = 0.9;
const LOW_RATIO_LIMIT: f64 = 0.01;
const GATE_ID: &str = "miso.gate-expander";

/// The stable IDs and values frozen by issue #746.  The descriptor is searched by ID below so the
/// request still uses its compact descriptor position if a future descriptor reorders IDs.
const ACTIVE_PARAMETERS: [(u32, f32); 7] = [
    (1, -20.0),
    (2, 20.0),
    (3, 48.0),
    (4, 6.0),
    (5, 1.0),
    (6, 0.0),
    (7, 5.0),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Phase {
    Preflight,
    Warmup,
    Measured(u8),
}

impl Phase {
    pub(crate) fn parse(args: &[String]) -> Result<Self, String> {
        match args {
            [flag] if flag == "--preflight" => Ok(Self::Preflight),
            [flag, value] if flag == "--phase" && value == "warmup" => Ok(Self::Warmup),
            [flag, value] if flag == "--phase" && value == "1" => Ok(Self::Measured(1)),
            [flag, value] if flag == "--phase" && value == "2" => Ok(Self::Measured(2)),
            [] => Err("one of --preflight or --phase warmup|1|2 is required".to_owned()),
            _ => Err("unknown, duplicate, contradictory, or extra subject argument".to_owned()),
        }
    }

    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Preflight => "preflight",
            Self::Warmup => "warmup",
            Self::Measured(1) => "1",
            Self::Measured(2) => "2",
            Self::Measured(_) => "invalid",
        }
    }

    pub(crate) const fn blocks(self) -> u64 {
        match self {
            Self::Preflight => PREFLIGHT_BLOCKS,
            Self::Warmup => WARMUP_BLOCKS,
            Self::Measured(_) => MEASURED_BLOCKS,
        }
    }

    pub(crate) const fn timed(self) -> bool {
        matches!(self, Self::Measured(_))
    }

    pub(crate) const fn round(self) -> Option<u8> {
        match self {
            Self::Measured(round) => Some(round),
            Self::Preflight | Self::Warmup => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ReportCounts {
    pub(crate) sanitized_main_samples: u64,
    pub(crate) sanitized_sidechain_samples: u64,
    pub(crate) invalid_spans: u64,
    pub(crate) nonfinite_left_blocks: u64,
    pub(crate) nonfinite_right_blocks: u64,
}

impl ReportCounts {
    pub(crate) fn add(&mut self, report: ProcessReport) {
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

    pub(crate) fn is_clear(self) -> bool {
        self.sanitized_main_samples == 0
            && self.sanitized_sidechain_samples == 0
            && self.invalid_spans == 0
            && self.nonfinite_left_blocks == 0
            && self.nonfinite_right_blocks == 0
    }
}

#[derive(Clone, Debug)]
struct Activity {
    lane: usize,
    channel: &'static str,
    high_plateaus: u64,
    low_plateaus: u64,
    high_ratio_witness: f64,
    low_ratio_witness: f64,
    finite_output_samples: u64,
    nonzero_input_samples: u64,
    nonzero_output_samples: u64,
}

impl Activity {
    fn new(lane: usize, channel: &'static str) -> Self {
        Self {
            lane,
            channel,
            high_plateaus: 0,
            low_plateaus: 0,
            high_ratio_witness: f64::INFINITY,
            low_ratio_witness: f64::NEG_INFINITY,
            finite_output_samples: 0,
            nonzero_input_samples: 0,
            nonzero_output_samples: 0,
        }
    }

    fn observe_sample(&mut self, input: f32, output: f32) -> Result<(), String> {
        if !input.is_finite() || input == 0.0 {
            return Err(format!(
                "{} lane {} received invalid input",
                self.channel, self.lane
            ));
        }
        self.nonzero_input_samples += 1;
        if !output.is_finite() {
            return Err(format!(
                "{} lane {} produced nonfinite output",
                self.channel, self.lane
            ));
        }
        self.finite_output_samples += 1;
        if output == 0.0 {
            return Err(format!(
                "{} lane {} produced zero output",
                self.channel, self.lane
            ));
        }
        self.nonzero_output_samples += 1;
        Ok(())
    }

    fn observe_plateau(&mut self, high: bool, input: f32, output: f32) -> Result<(), String> {
        let ratio = f64::from(output.abs()) / f64::from(input.abs());
        if !ratio.is_finite() || ratio < 0.0 {
            return Err(format!(
                "{} lane {} produced invalid activity ratio",
                self.channel, self.lane
            ));
        }
        if high {
            self.high_plateaus += 1;
            self.high_ratio_witness = self.high_ratio_witness.min(ratio);
        } else {
            self.low_plateaus += 1;
            self.low_ratio_witness = self.low_ratio_witness.max(ratio);
        }
        Ok(())
    }

    fn valid(&self) -> bool {
        self.high_plateaus > 0
            && self.low_plateaus > 0
            && self.high_ratio_witness > HIGH_RATIO_LIMIT
            && self.low_ratio_witness < LOW_RATIO_LIMIT
            && self.finite_output_samples == self.nonzero_input_samples
            && self.nonzero_output_samples == self.nonzero_input_samples
    }

    fn high_ratio(&self) -> f64 {
        if self.high_ratio_witness.is_finite() {
            self.high_ratio_witness
        } else {
            0.0
        }
    }

    fn low_ratio(&self) -> f64 {
        if self.low_ratio_witness.is_finite() {
            self.low_ratio_witness
        } else {
            0.0
        }
    }
}

struct Measurement {
    phase: Phase,
    width: usize,
    backend: &'static str,
    blocks: u64,
    elapsed_ns: Vec<u64>,
    activity: Vec<Activity>,
    reports: Vec<ReportCounts>,
}

pub(crate) fn main() {
    bench_alloc::assert_installed();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let phase = Phase::parse(&args).unwrap_or_else(|error| {
        eprintln!("gate-active: {error}");
        std::process::exit(2);
    });
    let metadata = Metadata::gather();
    for width in [WIDTH_SCALAR, WIDTH_BANK] {
        let measurement = run(width, phase).unwrap_or_else(|error| {
            eprintln!("gate-active {} width {width}: {error}", phase.name());
            std::process::exit(1);
        });
        println!("{}", measurement.record(metadata));
    }
}

fn run(width: usize, phase: Phase) -> Result<Measurement, String> {
    if width == WIDTH_BANK && Backend::current() != Backend::Simd8 {
        return Err("native homogeneous AVX2 W8 backend is unavailable".to_owned());
    }
    match width {
        WIDTH_SCALAR => run_scalar(phase),
        WIDTH_BANK => run_bank(phase),
        _ => Err("unsupported frozen width".to_owned()),
    }
}

fn run_scalar(phase: Phase) -> Result<Measurement, String> {
    let registry =
        launch_native_effect_registry().map_err(|error| format!("registry: {error:?}"))?;
    let factory = registry
        .get_ascii(GATE_ID)
        .ok_or_else(|| format!("launch registry lacks {GATE_ID}"))?;
    let values = active_values(factory.descriptor())?;
    let request = request(factory.descriptor(), &values)?;
    let mut effect = factory
        .prepare(request)
        .map_err(|error| format_prepare_error(error, "scalar"))?;
    verify_scalar(effect.as_ref(), factory.descriptor())?;
    let mut left = [0.0_f32; QUANTUM];
    let mut right = [0.0_f32; QUANTUM];
    let mut activity = vec![Activity::new(0, "left"), Activity::new(0, "right")];
    let mut reports = vec![ReportCounts::default()];
    let mut elapsed_ns = Vec::with_capacity(phase.blocks() as usize);
    for block in 0..phase.blocks() {
        fill_input(&mut left, &mut right, block, WIDTH_SCALAR);
        let first_sample = block * QUANTUM as u64;
        let process_block = EffectProcessBlock::new(
            &mut left,
            &mut right,
            None,
            first_sample,
            &[],
            QUANTUM as u32,
        )
        .expect("frozen scalar block shape");
        let (elapsed, report) = if phase.timed() {
            timing::timed(|| effect.process(process_block))
        } else {
            (0, timing::untimed(|| effect.process(process_block)))
        };
        inspect_scalar_block(block, &left, &right, &mut activity, report, &mut reports[0])?;
        if phase.timed() {
            if elapsed == 0 {
                return Err("timed process call returned zero elapsed nanoseconds".to_owned());
            }
            elapsed_ns.push(elapsed);
        }
    }
    finish_measurement(phase, WIDTH_SCALAR, "scalar", elapsed_ns, activity, reports)
}

fn run_bank(phase: Phase) -> Result<Measurement, String> {
    let registry =
        launch_native_effect_registry().map_err(|error| format!("registry: {error:?}"))?;
    let factory = registry
        .get_ascii(GATE_ID)
        .ok_or_else(|| format!("launch registry lacks {GATE_ID}"))?;
    let values = active_values(factory.descriptor())?;
    let request = request(factory.descriptor(), &values)?;
    let requests = [request; WIDTH_BANK];
    let mut bank = factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::Simd8,
            width: BankWidth::Eight,
            requests: &requests,
        })
        .map_err(|error| format_prepare_error(error, "W8 bank"))?
        .ok_or_else(|| "native homogeneous AVX2 W8 bank binding declined".to_owned())?;
    verify_bank(bank.as_ref(), factory.descriptor())?;
    let mut left = [0.0_f32; QUANTUM * WIDTH_BANK];
    let mut right = [0.0_f32; QUANTUM * WIDTH_BANK];
    let mut activity = (0..WIDTH_BANK)
        .flat_map(|lane| [Activity::new(lane, "left"), Activity::new(lane, "right")])
        .collect::<Vec<_>>();
    let mut reports = vec![ReportCounts::default(); WIDTH_BANK];
    let mut elapsed_ns = Vec::with_capacity(phase.blocks() as usize);
    let offsets = [0_u32; WIDTH_BANK + 1];
    for block in 0..phase.blocks() {
        fill_input(&mut left, &mut right, block, WIDTH_BANK);
        let first_sample = block * QUANTUM as u64;
        let process_block = EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            QUANTUM as u32,
            BankWidth::Eight,
            first_sample,
            &[],
            &offsets,
            QUANTUM as u32,
        )
        .expect("frozen W8 bank block shape");
        let (elapsed, report) = if phase.timed() {
            timing::timed(|| bank.process_bank(process_block))
        } else {
            (0, timing::untimed(|| bank.process_bank(process_block)))
        };
        inspect_bank_block(block, &left, &right, &mut activity, report, &mut reports)?;
        if phase.timed() {
            if elapsed == 0 {
                return Err("timed process call returned zero elapsed nanoseconds".to_owned());
            }
            elapsed_ns.push(elapsed);
        }
    }
    finish_measurement(phase, WIDTH_BANK, "Simd8", elapsed_ns, activity, reports)
}

fn active_values(
    descriptor: &'static EffectDescriptor,
) -> Result<Vec<InitialParameterValue>, String> {
    let mut values = Vec::with_capacity(descriptor.parameters.len() * CHANNELS);
    for (parameter_index, parameter) in descriptor.parameters.iter().enumerate() {
        let value = ACTIVE_PARAMETERS
            .iter()
            .find_map(|(id, value)| (parameter.id == ParameterId(*id)).then_some(*value))
            .ok_or_else(|| {
                format!(
                    "descriptor parameter ID {} is absent from frozen active set",
                    parameter.id.0
                )
            })?;
        values.push(InitialParameterValue {
            parameter_index: parameter_index as u32,
            channel: ParameterChannel::Left,
            value,
        });
        values.push(InitialParameterValue {
            parameter_index: parameter_index as u32,
            channel: ParameterChannel::Right,
            value,
        });
    }
    Ok(values)
}

fn request<'a>(
    descriptor: &'static EffectDescriptor,
    values: &'a [InitialParameterValue],
) -> Result<PrepareEffectRequest<'a>, String> {
    let quality = descriptor
        .qualities
        .iter()
        .find(|quality| {
            quality.quality == EffectQuality::Normal && quality.sample_rate == SAMPLE_RATE
        })
        .ok_or_else(|| "descriptor lacks frozen 48 kHz Normal quality".to_owned())?;
    let sidechain_id = descriptor
        .ports
        .iter()
        .find(|port| port.id.as_str() == "sidechain-in")
        .map(|port| port.id)
        .ok_or_else(|| "descriptor lacks sidechain-in port".to_owned())?;
    let state = quality
        .maximum_state
        .total()
        .ok_or_else(|| "descriptor state size overflows".to_owned())?;
    Ok(PrepareEffectRequest {
        sample_rate: SAMPLE_RATE,
        quantum: QUANTUM as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::Unconnected {
                id: sidechain_id,
                required: false,
            },
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: state,
            maximum_scratch_bytes: quality.scratch_fixed_bytes,
            maximum_automation_spans_per_block: 16,
        },
    })
}

fn verify_scalar(
    effect: &dyn PreparedNativeEffect,
    descriptor: &'static EffectDescriptor,
) -> Result<(), String> {
    let metadata = effect.metadata();
    if metadata.descriptor.id != descriptor.id
        || metadata.sample_rate != SAMPLE_RATE
        || metadata.quantum != QUANTUM as u32
        || metadata.quality != EffectQuality::Normal
        || metadata.bypass
        || metadata.link_mode != LinkMode::DualMono
        || metadata.latency != LatencySamples(0)
    {
        return Err("prepared scalar metadata does not match frozen descriptor".to_owned());
    }
    Ok(())
}

fn verify_bank(
    bank: &dyn PreparedNativeEffectBank,
    descriptor: &'static EffectDescriptor,
) -> Result<(), String> {
    let metadata = bank.metadata();
    let key = &metadata.program_key;
    if metadata.width != BankWidth::Eight
        || key.effect_id != descriptor.id
        || key.sample_rate != SAMPLE_RATE
        || key.quantum != QUANTUM as u32
        || key.quality != EffectQuality::Normal
        || key.bypass
        || key.link_mode != LinkMode::DualMono
        || key.latency != LatencySamples(0)
    {
        return Err("prepared W8 bank metadata does not match frozen descriptor".to_owned());
    }
    Ok(())
}

fn fill_input(left: &mut [f32], right: &mut [f32], block: u64, width: usize) {
    for frame in 0..QUANTUM {
        let absolute_sample = block * QUANTUM as u64 + frame as u64;
        for lane in 0..width {
            let base = frame * width + lane;
            let high_left = (block + 8 * lane as u64) % 64 < 32;
            let high_right = (block + 8 * lane as u64 + 32) % 64 < 32;
            let left_sign = if (absolute_sample + lane as u64).is_multiple_of(2) {
                1.0
            } else {
                -1.0
            };
            let right_sign = if (absolute_sample + lane as u64 + 1).is_multiple_of(2) {
                1.0
            } else {
                -1.0
            };
            left[base] = left_sign
                * if high_left {
                    HIGH_MAGNITUDE
                } else {
                    LOW_MAGNITUDE
                };
            right[base] = right_sign
                * if high_right {
                    HIGH_MAGNITUDE
                } else {
                    LOW_MAGNITUDE
                };
        }
    }
}

fn inspect_scalar_block(
    block: u64,
    left: &[f32],
    right: &[f32],
    activity: &mut [Activity],
    report: ProcessReport,
    counts: &mut ReportCounts,
) -> Result<(), String> {
    counts.add(report);
    if !counts.is_clear() {
        return Err(format!(
            "scalar process report contains a fault: {counts:?}"
        ));
    }
    for frame in 0..QUANTUM {
        activity[0].observe_sample(left_input(block, frame, 0, 0), left[frame])?;
        activity[1].observe_sample(left_input(block, frame, 0, 1), right[frame])?;
    }
    let input_left = left_input(block, QUANTUM - 1, 0, 0);
    let input_right = left_input(block, QUANTUM - 1, 0, 1);
    if block % 64 == 31 {
        activity[0].observe_plateau(true, input_left, left[QUANTUM - 1])?;
        activity[1].observe_plateau(false, input_right, right[QUANTUM - 1])?;
    } else if block % 64 == 63 {
        activity[0].observe_plateau(false, input_left, left[QUANTUM - 1])?;
        activity[1].observe_plateau(true, input_right, right[QUANTUM - 1])?;
    }
    Ok(())
}

fn inspect_bank_block(
    block: u64,
    left: &[f32],
    right: &[f32],
    activity: &mut [Activity],
    report: BankProcessReport,
    counts: &mut [ReportCounts],
) -> Result<(), String> {
    if report.width != BankWidth::Eight {
        return Err("bank returned a report for the wrong width".to_owned());
    }
    for lane in 0..WIDTH_BANK {
        counts[lane].add(report.reports[lane]);
        if !counts[lane].is_clear() {
            return Err(format!(
                "W8 lane {lane} process report contains a fault: {:?}",
                counts[lane]
            ));
        }
        for frame in 0..QUANTUM {
            let base = frame * WIDTH_BANK + lane;
            activity[lane * 2].observe_sample(left_input(block, frame, lane, 0), left[base])?;
            activity[lane * 2 + 1]
                .observe_sample(left_input(block, frame, lane, 1), right[base])?;
        }
        let left_sample_input = left_input(block, QUANTUM - 1, lane, 0);
        let right_sample_input = left_input(block, QUANTUM - 1, lane, 1);
        let phase = (block + 8 * lane as u64) % 64;
        let left_high = phase < 32;
        let right_high = (phase + 32) % 64 < 32;
        if (phase + 1).is_multiple_of(32) {
            activity[lane * 2].observe_plateau(
                left_high,
                left_sample_input,
                left[(QUANTUM - 1) * WIDTH_BANK + lane],
            )?;
            activity[lane * 2 + 1].observe_plateau(
                right_high,
                right_sample_input,
                right[(QUANTUM - 1) * WIDTH_BANK + lane],
            )?;
        }
    }
    Ok(())
}

fn left_input(block: u64, frame: usize, lane: usize, channel: usize) -> f32 {
    let phase = (block + 8 * lane as u64 + 32 * channel as u64) % 64;
    let magnitude = if phase < 32 {
        HIGH_MAGNITUDE
    } else {
        LOW_MAGNITUDE
    };
    let sign = if (block * QUANTUM as u64 + frame as u64 + lane as u64 + channel as u64)
        .is_multiple_of(2)
    {
        1.0
    } else {
        -1.0
    };
    sign * magnitude
}

fn finish_measurement(
    phase: Phase,
    width: usize,
    backend: &'static str,
    elapsed_ns: Vec<u64>,
    activity: Vec<Activity>,
    reports: Vec<ReportCounts>,
) -> Result<Measurement, String> {
    if !activity.iter().all(Activity::valid) {
        return Err(format!(
            "activity did not witness both causal plateaus: {activity:?}"
        ));
    }
    let blocks = phase.blocks();
    if phase.timed() && elapsed_ns.len() as u64 != blocks {
        return Err("timed call count does not equal frozen measured blocks".to_owned());
    }
    Ok(Measurement {
        phase,
        width,
        backend,
        blocks,
        elapsed_ns,
        activity,
        reports,
    })
}

fn format_prepare_error(error: EffectPrepareError, shape: &str) -> String {
    format!("{shape} preparation failed with {}", error.code)
}

impl Measurement {
    fn record(self, metadata: &Metadata) -> String {
        let process_elapsed = if self.elapsed_ns.is_empty() {
            "null".to_owned()
        } else {
            let percentiles = Percentiles::from_samples(&self.elapsed_ns);
            let sum = self
                .elapsed_ns
                .iter()
                .copied()
                .fold(0_u64, u64::saturating_add);
            format!(
                "{{\"observations\":{},\"sum_ns\":{},\"min_ns\":{},\"p50_ns\":{},\"p95_ns\":{},\"p99_ns\":{},\"p999_ns\":{},\"max_ns\":{}}}",
                self.elapsed_ns.len(),
                sum,
                percentiles.min,
                percentiles.p50,
                percentiles.p95,
                percentiles.p99,
                percentiles.p999,
                percentiles.max
            )
        };
        let activity = self
            .activity
            .into_iter()
            .map(|item| {
                format!(
                    "{{\"lane\":{},\"channel\":\"{}\",\"high_plateaus\":{},\"low_plateaus\":{},\"high_ratio_witness\":{:.9},\"low_ratio_witness\":{:.9},\"finite_output_samples\":{},\"nonzero_input_samples\":{},\"nonzero_output_samples\":{}}}",
                    item.lane,
                    item.channel,
                    item.high_plateaus,
                    item.low_plateaus,
                    item.high_ratio(),
                    item.low_ratio(),
                    item.finite_output_samples,
                    item.nonzero_input_samples,
                    item.nonzero_output_samples,
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let reports = self
            .reports
            .iter()
            .map(|report| {
                format!(
                    "{{\"sanitized_main_samples\":{},\"sanitized_sidechain_samples\":{},\"invalid_spans\":{},\"nonfinite_left_blocks\":{},\"nonfinite_right_blocks\":{}}}",
                    report.sanitized_main_samples,
                    report.sanitized_sidechain_samples,
                    report.invalid_spans,
                    report.nonfinite_left_blocks,
                    report.nonfinite_right_blocks,
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let round = self
            .phase
            .round()
            .map_or_else(|| "null".to_owned(), |round| round.to_string());
        // The shared metadata projection is intentionally a field fragment and therefore ends
        // with a comma. It is the final fragment in this record, so remove only that delimiter
        // before closing the object rather than changing the shared helper used by other subjects.
        let metadata = metadata.record_fields();
        let metadata = metadata.strip_suffix(',').unwrap_or(&metadata);
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"gate_active\",",
                "\"phase\":\"{phase}\",\"round\":{round},\"width\":{width},",
                "\"backend\":\"{backend}\",\"sample_rate_hz\":{rate},\"frames\":{frames},",
                "\"channels\":{channels},\"blocks\":{blocks},\"lane_samples\":{lane_samples},",
                "\"timed_call_count\":{timed},\"process_elapsed_ns\":{elapsed},",
                "\"activity\":[{activity}],\"actual_report_counts\":[{reports}],",
                "\"descriptive_only\":true,\"statistical_method\":\"nearest-rank percentiles over actual process-call nanoseconds; input and activity checks outside timer\",",
                "{metadata}}}"
            ),
            issue = ISSUE,
            phase = self.phase.name(),
            round = round,
            width = self.width,
            backend = self.backend,
            rate = SAMPLE_RATE,
            frames = QUANTUM,
            channels = CHANNELS,
            blocks = self.blocks,
            lane_samples = self.blocks * QUANTUM as u64 * CHANNELS as u64 * self.width as u64,
            timed = self.elapsed_ns.len(),
            elapsed = process_elapsed,
            activity = activity,
            reports = reports,
            metadata = metadata,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Activity, HIGH_RATIO_LIMIT, LOW_RATIO_LIMIT, PREFLIGHT_BLOCKS, Phase, WIDTH_BANK,
        WIDTH_SCALAR, fill_input, left_input, run,
    };
    use lane::Backend;

    #[test]
    fn phase_parser_requires_one_frozen_mode() {
        assert_eq!(
            Phase::parse(&["--preflight".to_owned()]),
            Ok(Phase::Preflight)
        );
        assert_eq!(
            Phase::parse(&["--phase".to_owned(), "warmup".to_owned()]),
            Ok(Phase::Warmup)
        );
        assert_eq!(
            Phase::parse(&["--phase".to_owned(), "1".to_owned()]),
            Ok(Phase::Measured(1))
        );
        assert!(Phase::parse(&[]).is_err());
        assert!(
            Phase::parse(&[
                "--preflight".to_owned(),
                "--phase".to_owned(),
                "1".to_owned()
            ])
            .is_err()
        );
        assert!(Phase::parse(&["--phase".to_owned(), "3".to_owned()]).is_err());
    }

    #[test]
    fn stimulus_has_exact_frozen_magnitudes_and_opposite_channels() {
        let mut left = [0.0_f32; 128];
        let mut right = [0.0_f32; 128];
        fill_input(&mut left, &mut right, 0, WIDTH_SCALAR);
        assert_eq!(left[0].abs(), 0.5);
        assert_eq!(right[0].abs(), 1.0 / 1024.0);
        assert_eq!(left_input(0, 0, 0, 0).abs(), 0.5);
        assert_eq!(left_input(0, 0, 0, 1).abs(), 1.0 / 1024.0);
    }

    #[test]
    fn activity_validator_rejects_missing_open_or_close() {
        let mut activity = Activity::new(0, "left");
        activity.high_plateaus = 1;
        activity.high_ratio_witness = HIGH_RATIO_LIMIT + 0.01;
        assert!(!activity.valid());
        activity.low_plateaus = 1;
        activity.low_ratio_witness = LOW_RATIO_LIMIT + 0.01;
        assert!(!activity.valid());
        activity.low_ratio_witness = LOW_RATIO_LIMIT - 0.001;
        activity.finite_output_samples = 1;
        activity.nonzero_input_samples = 1;
        activity.nonzero_output_samples = 1;
        assert!(activity.valid());
    }

    #[test]
    fn record_splice_does_not_leave_trailing_metadata_comma() {
        let activity = vec![
            Activity {
                lane: 0,
                channel: "left",
                high_plateaus: 1,
                low_plateaus: 1,
                high_ratio_witness: HIGH_RATIO_LIMIT + 0.01,
                low_ratio_witness: LOW_RATIO_LIMIT - 0.001,
                finite_output_samples: 1,
                nonzero_input_samples: 1,
                nonzero_output_samples: 1,
            },
            Activity {
                lane: 0,
                channel: "right",
                high_plateaus: 1,
                low_plateaus: 1,
                high_ratio_witness: HIGH_RATIO_LIMIT + 0.01,
                low_ratio_witness: LOW_RATIO_LIMIT - 0.001,
                finite_output_samples: 1,
                nonzero_input_samples: 1,
                nonzero_output_samples: 1,
            },
        ];
        let record = super::Measurement {
            phase: Phase::Preflight,
            width: WIDTH_SCALAR,
            backend: "scalar",
            blocks: PREFLIGHT_BLOCKS,
            elapsed_ns: Vec::new(),
            activity,
            reports: vec![super::ReportCounts::default()],
        }
        .record(bench_support::metadata::Metadata::gather());
        assert!(!record.ends_with(",}"));
        assert!(record.ends_with('}'));
    }

    #[test]
    fn untimed_preflight_runs_the_actual_shapes_without_timing_calls() {
        if Backend::current() != Backend::Simd8 {
            return;
        }
        let scalar = run(WIDTH_SCALAR, Phase::Preflight).expect("scalar preflight");
        let bank = run(WIDTH_BANK, Phase::Preflight).expect("W8 preflight");
        assert_eq!(scalar.blocks, PREFLIGHT_BLOCKS);
        assert_eq!(bank.blocks, PREFLIGHT_BLOCKS);
        assert!(scalar.elapsed_ns.is_empty());
        assert!(bank.elapsed_ns.is_empty());
        assert!(scalar.activity.iter().all(Activity::valid));
        assert!(bank.activity.iter().all(Activity::valid));
    }
}
