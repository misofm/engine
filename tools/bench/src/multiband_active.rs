//! Active causal two-band multiband-compressor workload for issue #748.
//!
//! Preparation, input construction, snapshots and activity accounting are outside the timer. The
//! only timed operation is the prepared scalar or native W8 process call. This subject deliberately
//! keeps its scalar and bank loops explicit: their process and state-snapshot APIs have different
//! shapes, and the benchmark is not a generic workload framework.

use bench_support::alloc as bench_alloc;
use bench_support::digest::Sha256Sink;
use bench_support::metadata::Metadata;
use bench_support::stats::Percentiles;
use bench_support::timing;
use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    BankProcessReport, BankWidth, EffectBankProcessBlock, EffectDescriptor, EffectPrepareError,
    EffectProcessBlock, EffectQuality, InitialParameterValue, LatencySamples, LinkMode,
    ParameterChannel, ParameterId, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedNativeEffect, PreparedNativeEffectBank, PreparedPorts,
    PreparedSidechainPort, ProcessReport, StatePayloadOutput, StatePayloadSizes,
};
use lane::Backend;

use super::gate_active::{Phase, ReportCounts};

const ISSUE: u32 = 748;
const SAMPLE_RATE: u32 = 48_000;
const QUANTUM: usize = 128;
#[cfg(test)]
const PREFLIGHT_BLOCKS: u64 = 128;
const WIDTH_SCALAR: usize = 1;
const WIDTH_BANK: usize = 8;
const CHANNELS: usize = 2;
const STATE_BYTES: usize = 188;
const HIGH_GAIN_LIMIT_DB: f64 = -3.0;
const QUIET_GAIN_FLOOR_DB: f64 = -0.1;
const QUIET_GAIN_CEILING_DB: f64 = 0.0;
const EFFECT_ID: &str = "miso.multiband-compressor";
const STIMULUS_ID: &str = "square_components_v1";
const LOW_COMPONENT_HZ: u32 = 125;
const HIGH_COMPONENT_HZ: u32 = 4_000;
const HIGH_AMPLITUDE: f32 = 1.0 / 4.0;
const QUIET_AMPLITUDE: f32 = 1.0 / 4_096.0;

/// Stable IDs and values frozen by issue #748. ID 2 is retired by the descriptor.
const ACTIVE_PARAMETERS: [(u32, f32); 11] = [
    (1, 1_000.0),
    (3, -30.0),
    (4, 4.0),
    (5, 1.0),
    (6, 5.0),
    (7, 0.0),
    (8, -30.0),
    (9, 4.0),
    (10, 1.0),
    (11, 5.0),
    (12, 0.0),
];

#[derive(Clone, Debug)]
struct Activity {
    lane: usize,
    channel: &'static str,
    band: &'static str,
    high_witnesses: u64,
    quiet_witnesses: u64,
    high_gain_min_db: f64,
    high_gain_max_db: f64,
    quiet_gain_min_db: f64,
    quiet_gain_max_db: f64,
    finite_output_samples: u64,
    nonzero_input_samples: u64,
    nonzero_output_samples: u64,
    input_energy: f64,
    output_energy: f64,
}

impl Activity {
    fn new(lane: usize, channel: &'static str, band: &'static str) -> Self {
        Self {
            lane,
            channel,
            band,
            high_witnesses: 0,
            quiet_witnesses: 0,
            high_gain_min_db: f64::INFINITY,
            high_gain_max_db: f64::NEG_INFINITY,
            quiet_gain_min_db: f64::INFINITY,
            quiet_gain_max_db: f64::NEG_INFINITY,
            finite_output_samples: 0,
            nonzero_input_samples: 0,
            nonzero_output_samples: 0,
            input_energy: 0.0,
            output_energy: 0.0,
        }
    }

    fn observe_sample(&mut self, input: f32, output: f32) -> Result<(), String> {
        if !input.is_finite() || !output.is_finite() {
            return Err(format!(
                "{} lane {} band {} received a nonfinite sample",
                self.channel, self.lane, self.band
            ));
        }
        let input = f64::from(input);
        let output = f64::from(output);
        self.input_energy += input * input;
        self.output_energy += output * output;
        if input != 0.0 {
            self.nonzero_input_samples += 1;
        }
        self.finite_output_samples += 1;
        if output != 0.0 {
            self.nonzero_output_samples += 1;
        }
        Ok(())
    }

    fn observe_gain(&mut self, high: bool, gain_db: f32) -> Result<(), String> {
        let gain = f64::from(gain_db);
        if !gain.is_finite() || gain > QUIET_GAIN_CEILING_DB {
            return Err(format!(
                "{} lane {} band {} returned invalid gain {gain_db}",
                self.channel, self.lane, self.band
            ));
        }
        if high {
            if gain >= HIGH_GAIN_LIMIT_DB {
                return Err(format!(
                    "{} lane {} band {} did not engage: {gain_db} dB",
                    self.channel, self.lane, self.band
                ));
            }
            self.high_witnesses += 1;
            self.high_gain_min_db = self.high_gain_min_db.min(gain);
            self.high_gain_max_db = self.high_gain_max_db.max(gain);
        } else {
            if gain <= QUIET_GAIN_FLOOR_DB {
                return Err(format!(
                    "{} lane {} band {} did not recover: {gain_db} dB",
                    self.channel, self.lane, self.band
                ));
            }
            self.quiet_witnesses += 1;
            self.quiet_gain_min_db = self.quiet_gain_min_db.min(gain);
            self.quiet_gain_max_db = self.quiet_gain_max_db.max(gain);
        }
        Ok(())
    }

    fn valid(&self, expected_witnesses: u64, expected_samples: u64) -> bool {
        self.high_witnesses == expected_witnesses
            && self.quiet_witnesses == expected_witnesses
            && self.high_gain_min_db.is_finite()
            && self.high_gain_max_db.is_finite()
            && self.quiet_gain_min_db.is_finite()
            && self.quiet_gain_max_db.is_finite()
            && self.high_gain_min_db <= self.high_gain_max_db
            && self.quiet_gain_min_db <= self.quiet_gain_max_db
            && self.high_gain_min_db <= QUIET_GAIN_CEILING_DB
            && self.high_gain_max_db <= QUIET_GAIN_CEILING_DB
            && self.quiet_gain_min_db <= QUIET_GAIN_CEILING_DB
            && self.quiet_gain_max_db <= QUIET_GAIN_CEILING_DB
            && self.high_gain_max_db < HIGH_GAIN_LIMIT_DB
            && self.quiet_gain_min_db > QUIET_GAIN_FLOOR_DB
            && self.quiet_gain_max_db <= QUIET_GAIN_CEILING_DB
            && self.finite_output_samples == expected_samples
            && self.nonzero_input_samples > 0
            && self.nonzero_input_samples <= expected_samples
            && self.nonzero_output_samples > 0
            && self.nonzero_output_samples <= expected_samples
            && self.input_energy.is_finite()
            && self.input_energy > 0.0
            && self.output_energy.is_finite()
            && self.output_energy > 0.0
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
    input_digest: String,
}

pub(crate) fn main() {
    bench_alloc::assert_installed();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let phase = Phase::parse(&args).unwrap_or_else(|error| {
        eprintln!("multiband-active: {error}");
        std::process::exit(2);
    });
    let metadata = Metadata::gather();
    for width in [WIDTH_SCALAR, WIDTH_BANK] {
        let measurement = run(width, phase).unwrap_or_else(|error| {
            eprintln!("multiband-active {} width {width}: {error}", phase.name());
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
        .get_ascii(EFFECT_ID)
        .ok_or_else(|| format!("launch registry lacks {EFFECT_ID}"))?;
    let values = active_values(factory.descriptor())?;
    let request = prepare_request(factory.descriptor(), &values)?;
    let mut effect = factory
        .prepare(request)
        .map_err(|error| format_prepare_error(error, "scalar"))?;
    let metadata = verify_scalar(effect.as_ref(), factory.descriptor())?;
    let mut left = [0.0_f32; QUANTUM];
    let mut right = [0.0_f32; QUANTUM];
    let mut source_bytes = [0_u8; QUANTUM * CHANNELS * 4];
    let mut digest = Sha256Sink::new();
    let mut activity = (0..CHANNELS)
        .flat_map(|channel| {
            let name = if channel == 0 { "left" } else { "right" };
            [
                Activity::new(0, name, "low"),
                Activity::new(0, name, "high"),
            ]
        })
        .collect::<Vec<_>>();
    let mut reports = vec![ReportCounts::default()];
    let mut elapsed_ns = Vec::with_capacity(phase.blocks() as usize);
    let mut common = [0_u8; 0];
    let mut state_left = [0_u8; STATE_BYTES];
    let mut state_right = [0_u8; STATE_BYTES];
    for block in 0..phase.blocks() {
        fill_input(
            &mut left,
            &mut right,
            block,
            WIDTH_SCALAR,
            &mut source_bytes,
            &mut digest,
        );
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
        if is_snapshot_boundary(block, 0, 0) {
            snapshot_scalar_gains(
                &*effect,
                metadata.state_sizes,
                &mut common,
                &mut state_left,
                &mut state_right,
                &mut activity,
                block % 64 == 31,
                (block + 32) % 64 == 31,
            )?;
        }
        if phase.timed() {
            if elapsed == 0 {
                return Err("timed process call returned zero elapsed nanoseconds".to_owned());
            }
            elapsed_ns.push(elapsed);
        }
    }
    finish_measurement(
        phase,
        WIDTH_SCALAR,
        "scalar",
        elapsed_ns,
        activity,
        reports,
        digest.finish_hex(),
    )
}

fn run_bank(phase: Phase) -> Result<Measurement, String> {
    let registry =
        launch_native_effect_registry().map_err(|error| format!("registry: {error:?}"))?;
    let factory = registry
        .get_ascii(EFFECT_ID)
        .ok_or_else(|| format!("launch registry lacks {EFFECT_ID}"))?;
    let values = active_values(factory.descriptor())?;
    let request = prepare_request(factory.descriptor(), &values)?;
    let requests = [request; WIDTH_BANK];
    let mut bank = factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::Simd8,
            width: BankWidth::Eight,
            requests: &requests,
        })
        .map_err(|error| format_prepare_error(error, "W8 bank"))?
        .ok_or_else(|| "native homogeneous AVX2 W8 bank binding declined".to_owned())?;
    let metadata = verify_bank(bank.as_ref(), factory.descriptor())?;
    let mut left = [0.0_f32; QUANTUM * WIDTH_BANK];
    let mut right = [0.0_f32; QUANTUM * WIDTH_BANK];
    let mut source_bytes = [0_u8; QUANTUM * WIDTH_BANK * CHANNELS * 4];
    let mut digest = Sha256Sink::new();
    let mut activity = (0..WIDTH_BANK)
        .flat_map(|lane| {
            [
                Activity::new(lane, "left", "low"),
                Activity::new(lane, "left", "high"),
                Activity::new(lane, "right", "low"),
                Activity::new(lane, "right", "high"),
            ]
        })
        .collect::<Vec<_>>();
    let mut reports = vec![ReportCounts::default(); WIDTH_BANK];
    let mut elapsed_ns = Vec::with_capacity(phase.blocks() as usize);
    let offsets = [0_u32; WIDTH_BANK + 1];
    let mut common = [0_u8; 0];
    let mut state_left = [0_u8; STATE_BYTES];
    let mut state_right = [0_u8; STATE_BYTES];
    for block in 0..phase.blocks() {
        fill_input(
            &mut left,
            &mut right,
            block,
            WIDTH_BANK,
            &mut source_bytes,
            &mut digest,
        );
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
        for lane in 0..WIDTH_BANK {
            let phase_in_lane = (block + 8 * lane as u64) % 64;
            if phase_in_lane == 31 || phase_in_lane == 63 {
                snapshot_bank_gains(
                    &*bank,
                    lane,
                    metadata.state_sizes,
                    &mut common,
                    &mut state_left,
                    &mut state_right,
                    &mut activity,
                    phase_in_lane == 31,
                    phase_in_lane == 63,
                )?;
            }
        }
        if phase.timed() {
            if elapsed == 0 {
                return Err("timed process call returned zero elapsed nanoseconds".to_owned());
            }
            elapsed_ns.push(elapsed);
        }
    }
    finish_measurement(
        phase,
        WIDTH_BANK,
        "Simd8",
        elapsed_ns,
        activity,
        reports,
        digest.finish_hex(),
    )
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

fn prepare_request<'a>(
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
    if descriptor
        .ports
        .iter()
        .any(|port| port.role == effect_contract::PortRole::SidechainInput)
    {
        return Err("multiband descriptor unexpectedly declares a sidechain input".to_owned());
    }
    if quality.maximum_state.common_bytes != 0
        || quality.maximum_state.left_bytes != STATE_BYTES as u32
        || quality.maximum_state.right_bytes != STATE_BYTES as u32
        || quality.scratch_fixed_bytes != 0
        || quality.scratch_bytes_per_frame != 0
    {
        return Err("descriptor state/scratch sizes do not match frozen workload".to_owned());
    }
    Ok(PrepareEffectRequest {
        sample_rate: SAMPLE_RATE,
        quantum: QUANTUM as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            // validate_prepare_request requires positive admission ceilings even though this
            // effect's actual scratch is zero. The one byte is an admission limit, not usage.
            maximum_total_state_bytes: (STATE_BYTES * CHANNELS) as u64,
            maximum_scratch_bytes: 1,
            maximum_automation_spans_per_block: 16,
        },
    })
}

fn verify_scalar(
    effect: &dyn PreparedNativeEffect,
    descriptor: &'static EffectDescriptor,
) -> Result<effect_contract::PreparedEffectMetadata, String> {
    let metadata = effect.metadata();
    if metadata.descriptor.id != descriptor.id
        || metadata.descriptor.id.as_str() != EFFECT_ID
        || metadata.sample_rate != SAMPLE_RATE
        || metadata.quantum != QUANTUM as u32
        || metadata.quality != EffectQuality::Normal
        || metadata.bypass
        || metadata.link_mode != LinkMode::DualMono
        || metadata.latency != LatencySamples(0)
        || metadata.ports.sidechain != PreparedSidechainPort::None
        || metadata.state_sizes
            != (StatePayloadSizes {
                common_bytes: 0,
                left_bytes: STATE_BYTES as u32,
                right_bytes: STATE_BYTES as u32,
            })
        || metadata.scratch_bytes != 0
        || metadata.automation_capacity != 16
        || metadata.descriptor.state_layout_version != 1
    {
        return Err("prepared scalar metadata does not match frozen descriptor".to_owned());
    }
    Ok(metadata)
}

fn verify_bank(
    bank: &dyn PreparedNativeEffectBank,
    descriptor: &'static EffectDescriptor,
) -> Result<effect_contract::EffectProgramKey, String> {
    let metadata = bank.metadata();
    let key = &metadata.program_key;
    let expected_sizes = StatePayloadSizes {
        common_bytes: 0,
        left_bytes: STATE_BYTES as u32,
        right_bytes: STATE_BYTES as u32,
    };
    if metadata.width != BankWidth::Eight
        || key.effect_id != descriptor.id
        || key.effect_id.as_str() != EFFECT_ID
        || key.sample_rate != SAMPLE_RATE
        || key.quantum != QUANTUM as u32
        || key.quality != EffectQuality::Normal
        || key.bypass
        || key.link_mode != LinkMode::DualMono
        || key.latency != LatencySamples(0)
        || key.ports.sidechain != PreparedSidechainPort::None
        || key.state_sizes != expected_sizes
        || key.scratch_bytes != 0
        || key.automation_capacity != 16
        || key.state_layout_version != 1
    {
        return Err("prepared W8 bank metadata does not match frozen descriptor".to_owned());
    }
    Ok(key.clone())
}

fn fill_input(
    left: &mut [f32],
    right: &mut [f32],
    block: u64,
    width: usize,
    source_bytes: &mut [u8],
    digest: &mut Sha256Sink,
) {
    let mut byte_offset = 0;
    for frame in 0..QUANTUM {
        let sample = block * QUANTUM as u64 + frame as u64;
        for lane in 0..width {
            let left_value = source_sample(sample, lane, 0, block);
            let right_value = source_sample(sample, lane, 1, block);
            let base = frame * width + lane;
            left[base] = left_value;
            right[base] = right_value;
            source_bytes[byte_offset..byte_offset + 4].copy_from_slice(&left_value.to_le_bytes());
            byte_offset += 4;
            source_bytes[byte_offset..byte_offset + 4].copy_from_slice(&right_value.to_le_bytes());
            byte_offset += 4;
        }
    }
    debug_assert_eq!(byte_offset, source_bytes.len());
    digest.update(source_bytes);
}

fn source_sample(sample: u64, lane: usize, channel: usize, block: u64) -> f32 {
    let m = sample + 24 * lane as u64 + 12 * channel as u64;
    let low = if m % 384 < 192 { 1.0_f32 } else { -1.0 };
    let high = if m % 12 < 6 { 1.0_f32 } else { -1.0 };
    let plateau = (block + 8 * lane as u64 + 32 * channel as u64) % 64;
    let amplitude = if plateau < 32 {
        HIGH_AMPLITUDE
    } else {
        QUIET_AMPLITUDE
    };
    amplitude * (low + high)
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
        activity[0].observe_sample(
            source_sample(block * QUANTUM as u64 + frame as u64, 0, 0, block),
            left[frame],
        )?;
        activity[1].observe_sample(
            source_sample(block * QUANTUM as u64 + frame as u64, 0, 0, block),
            left[frame],
        )?;
        activity[2].observe_sample(
            source_sample(block * QUANTUM as u64 + frame as u64, 0, 1, block),
            right[frame],
        )?;
        activity[3].observe_sample(
            source_sample(block * QUANTUM as u64 + frame as u64, 0, 1, block),
            right[frame],
        )?;
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
            let sample = block * QUANTUM as u64 + frame as u64;
            let base = frame * WIDTH_BANK + lane;
            activity[lane * 4].observe_sample(source_sample(sample, lane, 0, block), left[base])?;
            activity[lane * 4 + 1]
                .observe_sample(source_sample(sample, lane, 0, block), left[base])?;
            activity[lane * 4 + 2]
                .observe_sample(source_sample(sample, lane, 1, block), right[base])?;
            activity[lane * 4 + 3]
                .observe_sample(source_sample(sample, lane, 1, block), right[base])?;
        }
    }
    Ok(())
}

fn is_snapshot_boundary(block: u64, lane: usize, channel: usize) -> bool {
    let phase = (block + 8 * lane as u64 + 32 * channel as u64) % 64;
    phase == 31 || phase == 63
}

#[allow(clippy::too_many_arguments)]
fn snapshot_scalar_gains(
    effect: &dyn PreparedNativeEffect,
    sizes: StatePayloadSizes,
    common: &mut [u8],
    state_left: &mut [u8],
    state_right: &mut [u8],
    activity: &mut [Activity],
    left_high: bool,
    right_high: bool,
) -> Result<(), String> {
    let output = StatePayloadOutput::new(common, state_left, state_right, sizes)
        .map_err(|error| format!("scalar state output: {}", error.code))?;
    effect
        .snapshot_state_payload(output)
        .map_err(|error| format!("scalar state snapshot: {}", error.code))?;
    let low = read_gain(state_left, 1)?;
    let high_gain = read_gain(state_left, 2)?;
    activity[0].observe_gain(left_high, low)?;
    activity[1].observe_gain(left_high, high_gain)?;
    let low = read_gain(state_right, 1)?;
    let high_gain = read_gain(state_right, 2)?;
    activity[2].observe_gain(right_high, low)?;
    activity[3].observe_gain(right_high, high_gain)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn snapshot_bank_gains(
    bank: &dyn PreparedNativeEffectBank,
    lane: usize,
    sizes: StatePayloadSizes,
    common: &mut [u8],
    state_left: &mut [u8],
    state_right: &mut [u8],
    activity: &mut [Activity],
    left_high: bool,
    right_high: bool,
) -> Result<(), String> {
    let output = StatePayloadOutput::new(common, state_left, state_right, sizes)
        .map_err(|error| format!("W8 lane {lane} state output: {}", error.code))?;
    bank.snapshot_track_state_payload(lane as u32, output)
        .map_err(|error| format!("W8 lane {lane} state snapshot: {}", error.code))?;
    let low = read_gain(state_left, 1)?;
    let high_gain = read_gain(state_left, 2)?;
    activity[lane * 4].observe_gain(left_high, low)?;
    activity[lane * 4 + 1].observe_gain(left_high, high_gain)?;
    let low = read_gain(state_right, 1)?;
    let high_gain = read_gain(state_right, 2)?;
    activity[lane * 4 + 2].observe_gain(right_high, low)?;
    activity[lane * 4 + 3].observe_gain(right_high, high_gain)?;
    Ok(())
}

fn read_gain(bytes: &[u8], word: usize) -> Result<f32, String> {
    let offset = word
        .checked_mul(4)
        .ok_or_else(|| "state gain offset overflow".to_owned())?;
    let end = offset + 4;
    let raw = bytes
        .get(offset..end)
        .ok_or_else(|| "state payload is shorter than frozen gain words".to_owned())?;
    Ok(f32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]))
}

fn finish_measurement(
    phase: Phase,
    width: usize,
    backend: &'static str,
    elapsed_ns: Vec<u64>,
    activity: Vec<Activity>,
    reports: Vec<ReportCounts>,
    input_digest: String,
) -> Result<Measurement, String> {
    let expected_samples = phase.blocks() * QUANTUM as u64;
    let expected_witnesses = phase.blocks() / 64;
    if !activity
        .iter()
        .all(|item| item.valid(expected_witnesses, expected_samples))
    {
        return Err(format!(
            "activity did not witness both bands and plateaus: {activity:?}"
        ));
    }
    if phase.timed() && elapsed_ns.len() as u64 != phase.blocks() {
        return Err("timed call count does not equal frozen measured blocks".to_owned());
    }
    if reports.iter().any(|report| !report.is_clear()) {
        return Err("actual process report counts are not clear".to_owned());
    }
    Ok(Measurement {
        phase,
        width,
        backend,
        blocks: phase.blocks(),
        elapsed_ns,
        activity,
        reports,
        input_digest,
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
                    "{{\"lane\":{},\"channel\":\"{}\",\"band\":\"{}\",\"high_witnesses\":{},\"quiet_witnesses\":{},\"high_gain_min_db\":{:.9},\"high_gain_max_db\":{:.9},\"quiet_gain_min_db\":{:.9},\"quiet_gain_max_db\":{:.9},\"finite_output_samples\":{},\"nonzero_input_samples\":{},\"nonzero_output_samples\":{},\"input_energy\":{:.9},\"output_energy\":{:.9}}}",
                    item.lane,
                    item.channel,
                    item.band,
                    item.high_witnesses,
                    item.quiet_witnesses,
                    item.high_gain_min_db,
                    item.high_gain_max_db,
                    item.quiet_gain_min_db,
                    item.quiet_gain_max_db,
                    item.finite_output_samples,
                    item.nonzero_input_samples,
                    item.nonzero_output_samples,
                    item.input_energy,
                    item.output_energy,
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
        let metadata = metadata.record_fields();
        let metadata = metadata.strip_suffix(',').unwrap_or(&metadata);
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"multiband_active\",",
                "\"effect_id\":\"{effect_id}\",\"phase\":\"{phase}\",\"round\":{round},",
                "\"width\":{width},\"backend\":\"{backend}\",\"sample_rate_hz\":{rate},",
                "\"frames\":{frames},\"channels\":{channels},\"blocks\":{blocks},",
                "\"lane_samples\":{lane_samples},\"timed_call_count\":{timed},",
                "\"process_elapsed_ns\":{elapsed},\"input_digest\":\"{digest}\",",
                "\"stimulus\":\"{stimulus}\",\"low_component_hz\":{low_hz},",
                "\"high_component_hz\":{high_hz},\"high_amplitude\":{high_amp:.12},",
                "\"quiet_amplitude\":{quiet_amp:.12},\"crossover_hz\":1000,",
                "\"threshold_db\":-30,\"ratio\":4,\"attack_ms\":1,\"release_ms\":5,",
                "\"makeup_db\":0,\"state_layout_version\":1,\"state_common_bytes\":0,",
                "\"state_channel_bytes\":188,\"scratch_admission_bytes\":1,\"scratch_bytes\":0,",
                "\"snapshot_gain_words\":\"little-endian f32 word 1 low, word 2 high\",",
                "\"input_digest_serialization\":\"block-major, frame-major, track-major, channel-left-then-right, f32 little-endian before process\",",
                "\"activity\":[{activity}],\"actual_report_counts\":[{reports}],",
                "\"descriptive_only\":true,\"statistical_method\":\"nearest-rank percentiles over actual process-call nanoseconds; input, snapshots and activity checks outside timer\",",
                "{metadata}}}"
            ),
            issue = ISSUE,
            effect_id = EFFECT_ID,
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
            digest = self.input_digest,
            stimulus = STIMULUS_ID,
            low_hz = LOW_COMPONENT_HZ,
            high_hz = HIGH_COMPONENT_HZ,
            high_amp = HIGH_AMPLITUDE,
            quiet_amp = QUIET_AMPLITUDE,
            activity = activity,
            reports = reports,
            metadata = metadata,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Activity, HIGH_AMPLITUDE, HIGH_COMPONENT_HZ, HIGH_GAIN_LIMIT_DB, LOW_COMPONENT_HZ,
        PREFLIGHT_BLOCKS, QUANTUM, QUIET_AMPLITUDE, QUIET_GAIN_CEILING_DB, QUIET_GAIN_FLOOR_DB,
        STATE_BYTES, STIMULUS_ID, WIDTH_BANK, WIDTH_SCALAR, active_values, fill_input,
        is_snapshot_boundary, run, source_sample,
    };
    use bench_support::digest::Sha256Sink;
    use effect_compiler::launch_native_effect_registry;
    use lane::Backend;

    #[test]
    fn frozen_source_has_expected_components_and_zero_cancellation() {
        assert_eq!(HIGH_COMPONENT_HZ, 4_000);
        assert_eq!(LOW_COMPONENT_HZ, 125);
        assert_eq!(HIGH_AMPLITUDE, 0.25);
        assert_eq!(QUIET_AMPLITUDE, 1.0 / 4096.0);
        assert_eq!(source_sample(0, 0, 0, 0), 0.5);
        assert_eq!(source_sample(6, 0, 0, 0), 0.0);
        assert_eq!(STIMULUS_ID, "square_components_v1");
    }

    #[test]
    fn activity_rejects_incomplete_or_out_of_bound_gain_witnesses() {
        let mut item = Activity::new(0, "left", "low");
        item.high_witnesses = 2;
        item.quiet_witnesses = 2;
        item.high_gain_min_db = HIGH_GAIN_LIMIT_DB - 1.0;
        item.high_gain_max_db = HIGH_GAIN_LIMIT_DB - 0.1;
        item.quiet_gain_min_db = QUIET_GAIN_FLOOR_DB + 0.01;
        item.quiet_gain_max_db = QUIET_GAIN_CEILING_DB;
        item.finite_output_samples = 4;
        item.nonzero_input_samples = 1;
        item.nonzero_output_samples = 1;
        item.input_energy = 1.0;
        item.output_energy = 1.0;
        assert!(item.valid(2, 4));
        item.quiet_gain_min_db = QUIET_GAIN_FLOOR_DB;
        assert!(!item.valid(2, 4));
        item.quiet_gain_min_db = QUIET_GAIN_FLOOR_DB + 0.01;
        item.high_gain_min_db = -4.0;
        item.high_gain_max_db = -5.0;
        assert!(!item.valid(2, 4));
        item.high_gain_min_db = HIGH_GAIN_LIMIT_DB - 1.0;
        item.high_gain_max_db = HIGH_GAIN_LIMIT_DB - 0.1;
        item.quiet_gain_min_db = -0.05;
        item.quiet_gain_max_db = -0.08;
        assert!(!item.valid(2, 4));
        item.quiet_gain_max_db = QUIET_GAIN_CEILING_DB;
        item.high_gain_min_db = f64::NAN;
        assert!(!item.valid(2, 4));
        item.high_gain_min_db = HIGH_GAIN_LIMIT_DB - 1.0;
        item.nonzero_input_samples = 5;
        assert!(!item.valid(2, 4));
        item.nonzero_input_samples = 1;
        item.nonzero_output_samples = 5;
        assert!(!item.valid(2, 4));
    }

    #[test]
    fn input_digest_uses_one_update_per_block_and_frozen_layout() {
        let mut left = [0.0_f32; QUANTUM];
        let mut right = [0.0_f32; QUANTUM];
        let mut bytes = [0_u8; QUANTUM * 2 * 4];
        let mut digest = Sha256Sink::new();
        let mark = bench_support::digest::updates_mark();
        fill_input(
            &mut left,
            &mut right,
            0,
            WIDTH_SCALAR,
            &mut bytes,
            &mut digest,
        );
        assert_eq!(bench_support::digest::updates_since(mark), 1);
        assert_eq!(bytes.len(), QUANTUM * 2 * 4);
        assert_eq!(left[0], 0.5);
        assert_eq!(right[0], 2.0 * QUIET_AMPLITUDE);
    }

    #[test]
    fn snapshot_boundaries_cover_each_phase_endpoint() {
        assert!(is_snapshot_boundary(31, 0, 0));
        assert!(is_snapshot_boundary(63, 0, 0));
        assert!(is_snapshot_boundary(31, 0, 1));
        assert!(is_snapshot_boundary(63, 0, 1));
        assert_eq!(STATE_BYTES, 188);
    }

    #[test]
    fn descriptor_has_all_frozen_active_values() {
        let registry = launch_native_effect_registry().expect("registry");
        let factory = registry
            .get_ascii(super::EFFECT_ID)
            .expect("multiband factory");
        assert_eq!(
            active_values(factory.descriptor()).expect("values").len(),
            22
        );
    }

    #[test]
    fn untimed_preflight_runs_actual_scalar_and_bank_shapes() {
        if Backend::current() != Backend::Simd8 {
            return;
        }
        let scalar = run(WIDTH_SCALAR, super::Phase::Preflight).expect("scalar preflight");
        let bank = run(WIDTH_BANK, super::Phase::Preflight).expect("W8 preflight");
        assert_eq!(scalar.blocks, PREFLIGHT_BLOCKS);
        assert_eq!(bank.blocks, PREFLIGHT_BLOCKS);
        assert!(scalar.elapsed_ns.is_empty());
        assert!(bank.elapsed_ns.is_empty());
    }
}
