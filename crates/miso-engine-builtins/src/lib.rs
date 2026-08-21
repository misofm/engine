//! Scalar fixed track processors and transparent meter state for issue 007.
#![allow(missing_docs)]

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

use miso_engine_core::realtime::{Consumer, Producer, QueueGeneration, bounded_spsc};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChannelLinkMode {
    DualMono,
    ExplicitMatrix2x2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinResetKind {
    FullToPrepared,
    DiscontinuityKeepTargets,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2x2 {
    pub ll: f32,
    pub lr: f32,
    pub rl: f32,
    pub rr: f32,
}

impl Matrix2x2 {
    pub const IDENTITY: Self = Self {
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
    };

    pub fn checked(self) -> Result<Self, BuiltinParameterError> {
        if [self.ll, self.lr, self.rl, self.rr]
            .into_iter()
            .all(|v| v.is_finite() && (-1.0..=1.0).contains(&v))
        {
            Ok(Self {
                ll: zero(self.ll),
                lr: zero(self.lr),
                rl: zero(self.rl),
                rr: zero(self.rr),
            })
        } else {
            Err(BuiltinParameterError::MatrixCoefficient)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinParameterError {
    EmptyBlock,
    LaneLength,
    SampleTimeOverflow,
    GainDomain,
    FilterCutoff,
    FilterOrder,
    FilterCoefficients,
    MatrixCoefficient,
    MatrixSmoothing,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BuiltinProcessReport {
    pub sanitized_input: u64,
    pub sanitized_output: u64,
    pub recovered_left_state: u64,
    pub recovered_right_state: u64,
}

impl BuiltinProcessReport {
    fn add(&mut self, other: Self) {
        self.sanitized_input = self.sanitized_input.saturating_add(other.sanitized_input);
        self.sanitized_output = self.sanitized_output.saturating_add(other.sanitized_output);
        self.recovered_left_state = self
            .recovered_left_state
            .saturating_add(other.recovered_left_state);
        self.recovered_right_state = self
            .recovered_right_state
            .saturating_add(other.recovered_right_state);
    }
}

pub struct DualMonoBlock<'a> {
    pub left: &'a mut [f32],
    pub right: &'a mut [f32],
    pub first_sample: u64,
}

impl DualMonoBlock<'_> {
    fn checked_len(&self) -> Result<usize, BuiltinParameterError> {
        if self.left.is_empty() {
            return Err(BuiltinParameterError::EmptyBlock);
        }
        if self.left.len() != self.right.len() {
            return Err(BuiltinParameterError::LaneLength);
        }
        let len = u64::try_from(self.left.len())
            .map_err(|_| BuiltinParameterError::SampleTimeOverflow)?;
        self.first_sample
            .checked_add(len)
            .ok_or(BuiltinParameterError::SampleTimeOverflow)?;
        Ok(self.left.len())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChannelParameters {
    pub polarity_invert: bool,
    pub trim_db: f32,
    pub hpf_hz: f32,
    pub lpf_hz: f32,
    pub fader_db: f32,
    pub muted: bool,
}

impl Default for ChannelParameters {
    fn default() -> Self {
        Self {
            polarity_invert: false,
            trim_db: 0.0,
            hpf_hz: 0.0,
            lpf_hz: 0.0,
            fader_db: 0.0,
            muted: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BuiltinParameters {
    pub left: ChannelParameters,
    pub right: ChannelParameters,
    pub matrix: Matrix2x2,
    pub smoothing_samples: u32,
}

impl Default for BuiltinParameters {
    fn default() -> Self {
        Self {
            left: ChannelParameters::default(),
            right: ChannelParameters::default(),
            matrix: Matrix2x2::IDENTITY,
            smoothing_samples: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinTail {
    FiniteZero,
    Infinite,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinParameterDescriptorV1 {
    pub id: u32,
    pub name: &'static str,
    pub per_lane: bool,
}

pub const BUILTIN_PARAMETER_DESCRIPTORS_V1: [BuiltinParameterDescriptorV1; 10] = [
    BuiltinParameterDescriptorV1 {
        id: 1,
        name: "polarity_invert",
        per_lane: true,
    },
    BuiltinParameterDescriptorV1 {
        id: 2,
        name: "trim_db",
        per_lane: true,
    },
    BuiltinParameterDescriptorV1 {
        id: 3,
        name: "hpf_hz",
        per_lane: true,
    },
    BuiltinParameterDescriptorV1 {
        id: 4,
        name: "lpf_hz",
        per_lane: true,
    },
    BuiltinParameterDescriptorV1 {
        id: 5,
        name: "fader_db",
        per_lane: true,
    },
    BuiltinParameterDescriptorV1 {
        id: 6,
        name: "mute",
        per_lane: true,
    },
    BuiltinParameterDescriptorV1 {
        id: 7,
        name: "matrix_ll",
        per_lane: false,
    },
    BuiltinParameterDescriptorV1 {
        id: 8,
        name: "matrix_lr",
        per_lane: false,
    },
    BuiltinParameterDescriptorV1 {
        id: 9,
        name: "matrix_rl",
        per_lane: false,
    },
    BuiltinParameterDescriptorV1 {
        id: 10,
        name: "matrix_rr",
        per_lane: false,
    },
];

#[derive(Clone, Copy)]
struct TptSvf {
    a1: f32,
    a2: f32,
    a3: f32,
    k: f32,
    s1: f32,
    s2: f32,
    high_pass: bool,
    enabled: bool,
}

impl TptSvf {
    fn identity() -> Self {
        Self {
            a1: 0.0,
            a2: 0.0,
            a3: 0.0,
            k: 0.0,
            s1: 0.0,
            s2: 0.0,
            high_pass: false,
            enabled: false,
        }
    }
    fn design(rate: u32, cutoff: f32, high_pass: bool) -> Result<Self, BuiltinParameterError> {
        if cutoff == 0.0 {
            return Ok(Self::identity());
        }
        if !cutoff.is_finite() || cutoff < 10.0 || f64::from(cutoff) >= f64::from(rate) / 2.0 {
            return Err(BuiltinParameterError::FilterCutoff);
        }
        let g = (core::f64::consts::PI * f64::from(cutoff) / f64::from(rate)).tan();
        let k = core::f64::consts::SQRT_2;
        let denominator = 1.0 + g * (g + k);
        let values =
            [1.0 / denominator, g / denominator, g * g / denominator, k].map(|value| value as f32);
        if !values.into_iter().all(normal_or_zero) {
            return Err(BuiltinParameterError::FilterCoefficients);
        }
        let [a1, a2, a3, k] = values;
        let transition_00 = 2.0_f64 * f64::from(a1) - 1.0;
        let transition_01 = -2.0_f64 * f64::from(a2);
        let transition_10 = 2.0_f64 * f64::from(a2);
        let transition_11 = 1.0 - 2.0_f64 * f64::from(a3);
        let trace = transition_00 + transition_11;
        let determinant = transition_00 * transition_11 - transition_01 * transition_10;
        let denominator_a1 = -trace;
        let denominator_a2 = determinant;
        if denominator_a2.abs() >= 1.0
            || 1.0 + denominator_a1 + denominator_a2 <= 0.0
            || 1.0 - denominator_a1 + denominator_a2 <= 0.0
        {
            return Err(BuiltinParameterError::FilterCoefficients);
        }
        Ok(Self {
            a1,
            a2,
            a3,
            k,
            s1: 0.0,
            s2: 0.0,
            high_pass,
            enabled: true,
        })
    }
    fn reset(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
    }
    fn process(
        &mut self,
        input: f32,
        recovered: &mut u64,
        report: &mut BuiltinProcessReport,
    ) -> f32 {
        if !self.enabled {
            return input;
        }
        if !normal_or_zero(self.s1) || !normal_or_zero(self.s2) {
            self.reset();
            *recovered = recovered.saturating_add(1);
        }
        let v3 = input - self.s2;
        let p1 = self.a1 * self.s1;
        let p2 = self.a2 * v3;
        let v1 = p1 + p2;
        let p3 = self.a2 * self.s1;
        let p4 = self.a3 * v3;
        let t2 = self.s2 + p3;
        let v2 = t2 + p4;
        let n1 = 2.0 * v1 - self.s1;
        let n2 = 2.0 * v2 - self.s2;
        let low = v2;
        let kh = self.k * v1;
        let th = input - kh;
        let high = th - v2;
        if !normal_or_zero(n1) || !normal_or_zero(n2) {
            self.reset();
            *recovered = recovered.saturating_add(1);
            return 0.0;
        }
        self.s1 = n1;
        self.s2 = n2;
        sanitize(
            if self.high_pass { high } else { low },
            &mut report.sanitized_output,
        )
    }
}

#[derive(Clone, Copy)]
struct InputLane {
    polarity: bool,
    trim: f32,
    hpf: TptSvf,
    lpf: TptSvf,
}
#[derive(Clone, Copy)]
struct FaderLane {
    gain: f32,
    muted: bool,
}

pub struct InputBuiltins {
    left: InputLane,
    right: InputLane,
    recovered_left: u64,
    recovered_right: u64,
}
pub struct FaderMuteBuiltins {
    left: FaderLane,
    right: FaderLane,
}
pub struct MatrixBuiltins {
    current: Matrix2x2,
    target: Matrix2x2,
    smoothing_samples: u32,
    remaining_updates: u32,
}

pub struct BuiltinChain {
    input: InputBuiltins,
    fader_mute: FaderMuteBuiltins,
    matrix: MatrixBuiltins,
}

impl BuiltinChain {
    pub fn new(
        sample_rate: u32,
        parameters: BuiltinParameters,
    ) -> Result<Self, BuiltinParameterError> {
        let (input, fader_mute, matrix) = prepare_sections(sample_rate, parameters)?;
        Ok(Self {
            input,
            fader_mute,
            matrix,
        })
    }
    pub fn process_input(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        self.input.process(block)
    }
    pub fn process_fader_mute(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        self.fader_mute.process(block)
    }
    pub fn process_matrix(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        self.matrix.process(block)
    }
    pub fn process_dual_mono(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let DualMonoBlock {
            left,
            right,
            first_sample,
        } = block;
        let mut report = self.input.process(DualMonoBlock {
            left,
            right,
            first_sample,
        });
        report.add(self.fader_mute.process(DualMonoBlock {
            left,
            right,
            first_sample,
        }));
        report.add(self.matrix.process(DualMonoBlock {
            left,
            right,
            first_sample,
        }));
        report
    }
    pub fn set_matrix_target(&mut self, target: Matrix2x2) -> Result<(), BuiltinParameterError> {
        self.matrix.set_target(target)
    }
    pub fn reset(&mut self, kind: BuiltinResetKind) {
        self.input.reset();
        self.matrix.reset();
        if matches!(kind, BuiltinResetKind::FullToPrepared) {
            self.fader_mute.reset();
        }
    }
    pub fn link_mode(&self) -> ChannelLinkMode {
        ChannelLinkMode::ExplicitMatrix2x2
    }
    pub fn tail(&self) -> BuiltinTail {
        self.input.tail()
    }
    pub fn into_sections(self) -> (InputBuiltins, FaderMuteBuiltins, MatrixBuiltins) {
        (self.input, self.fader_mute, self.matrix)
    }
}

fn prepare_sections(
    sample_rate: u32,
    parameters: BuiltinParameters,
) -> Result<(InputBuiltins, FaderMuteBuiltins, MatrixBuiltins), BuiltinParameterError> {
    if sample_rate == 0 {
        return Err(BuiltinParameterError::FilterCutoff);
    }
    parameters.matrix.checked()?;
    for lane in [parameters.left, parameters.right] {
        if !lane.trim_db.is_finite()
            || !(-144.0..=24.0).contains(&lane.trim_db)
            || !lane.fader_db.is_finite()
            || !(-144.0..=24.0).contains(&lane.fader_db)
        {
            return Err(BuiltinParameterError::GainDomain);
        }
        if lane.hpf_hz > 0.0 && lane.lpf_hz > 0.0 && lane.hpf_hz >= lane.lpf_hz {
            return Err(BuiltinParameterError::FilterOrder);
        }
    }
    let lane = |params: ChannelParameters| -> Result<InputLane, BuiltinParameterError> {
        Ok(InputLane {
            polarity: params.polarity_invert,
            trim: db_gain(params.trim_db)?,
            hpf: TptSvf::design(sample_rate, zero(params.hpf_hz), true)?,
            lpf: TptSvf::design(sample_rate, zero(params.lpf_hz), false)?,
        })
    };
    let fader = |params: ChannelParameters| -> Result<FaderLane, BuiltinParameterError> {
        Ok(FaderLane {
            gain: db_gain(params.fader_db)?,
            muted: params.muted,
        })
    };
    Ok((
        InputBuiltins {
            left: lane(parameters.left)?,
            right: lane(parameters.right)?,
            recovered_left: 0,
            recovered_right: 0,
        },
        FaderMuteBuiltins {
            left: fader(parameters.left)?,
            right: fader(parameters.right)?,
        },
        MatrixBuiltins {
            current: parameters.matrix,
            target: parameters.matrix,
            smoothing_samples: parameters.smoothing_samples,
            remaining_updates: 0,
        },
    ))
}

impl InputBuiltins {
    pub fn process(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let mut report = BuiltinProcessReport::default();
        if block.checked_len().is_err() {
            return report;
        }
        for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
            *left =
                process_input_lane(&mut self.left, *left, &mut self.recovered_left, &mut report);
            *right = process_input_lane(
                &mut self.right,
                *right,
                &mut self.recovered_right,
                &mut report,
            );
        }
        report.recovered_left_state = self.recovered_left;
        report.recovered_right_state = self.recovered_right;
        report
    }
    pub fn reset(&mut self) {
        self.left.hpf.reset();
        self.left.lpf.reset();
        self.right.hpf.reset();
        self.right.lpf.reset();
    }
    pub fn tail(&self) -> BuiltinTail {
        if self.left.hpf.enabled
            || self.left.lpf.enabled
            || self.right.hpf.enabled
            || self.right.lpf.enabled
        {
            BuiltinTail::Infinite
        } else {
            BuiltinTail::FiniteZero
        }
    }
}

fn process_input_lane(
    lane: &mut InputLane,
    sample: f32,
    recovered: &mut u64,
    report: &mut BuiltinProcessReport,
) -> f32 {
    let sample = sanitize_input(sample, &mut report.sanitized_input);
    let signed = if lane.polarity { -sample } else { sample };
    let trimmed = sanitize(signed * lane.trim, &mut report.sanitized_output);
    let high = lane.hpf.process(trimmed, recovered, report);
    lane.lpf.process(high, recovered, report)
}

impl FaderMuteBuiltins {
    pub fn process(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let mut report = BuiltinProcessReport::default();
        if block.checked_len().is_err() {
            return report;
        }
        for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
            *left = if self.left.muted {
                0.0
            } else {
                sanitize(*left * self.left.gain, &mut report.sanitized_output)
            };
            *right = if self.right.muted {
                0.0
            } else {
                sanitize(*right * self.right.gain, &mut report.sanitized_output)
            };
        }
        report
    }
    fn reset(&mut self) {}
}

impl MatrixBuiltins {
    pub fn set_target(&mut self, target: Matrix2x2) -> Result<(), BuiltinParameterError> {
        self.target = target.checked()?;
        self.remaining_updates = self.smoothing_samples;
        if self.remaining_updates == 0 {
            self.current = self.target;
        }
        Ok(())
    }
    pub fn process(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let mut report = BuiltinProcessReport::default();
        if block.checked_len().is_err() {
            return report;
        }
        for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
            self.advance();
            let in_left = sanitize_input(*left, &mut report.sanitized_input);
            let in_right = sanitize_input(*right, &mut report.sanitized_input);
            *left = sanitize(
                self.current.ll * in_left + self.current.lr * in_right,
                &mut report.sanitized_output,
            );
            *right = sanitize(
                self.current.rl * in_left + self.current.rr * in_right,
                &mut report.sanitized_output,
            );
        }
        report
    }
    fn advance(&mut self) {
        if self.remaining_updates == 0 {
            return;
        }
        let remaining = self.remaining_updates as f32;
        self.current.ll += (self.target.ll - self.current.ll) / remaining;
        self.current.lr += (self.target.lr - self.current.lr) / remaining;
        self.current.rl += (self.target.rl - self.current.rl) / remaining;
        self.current.rr += (self.target.rr - self.current.rr) / remaining;
        self.remaining_updates -= 1;
        if self.remaining_updates == 0 {
            self.current = self.target;
        }
    }
    pub fn reset(&mut self) {
        self.current = self.target;
        self.remaining_updates = 0;
    }
}

pub fn pan_matrix(left: f32, right: f32) -> Result<Matrix2x2, BuiltinParameterError> {
    if !left.is_finite()
        || !right.is_finite()
        || !(-1.0..=1.0).contains(&left)
        || !(-1.0..=1.0).contains(&right)
    {
        return Err(BuiltinParameterError::MatrixCoefficient);
    }
    let gains = |position: f32| {
        let theta = (f64::from(position) + 1.0) * core::f64::consts::FRAC_PI_4;
        (theta.cos() as f32, theta.sin() as f32)
    };
    let (ll, rl) = gains(left);
    let (lr, rr) = gains(right);
    Matrix2x2 { ll, lr, rl, rr }.checked()
}

pub fn balance_matrix(balance: f32) -> Result<Matrix2x2, BuiltinParameterError> {
    if !balance.is_finite() || !(-1.0..=1.0).contains(&balance) {
        return Err(BuiltinParameterError::MatrixCoefficient);
    }
    let gain = (f64::from(balance.abs()) * core::f64::consts::FRAC_PI_2).cos() as f32;
    if balance >= 0.0 {
        Ok(Matrix2x2 {
            ll: gain,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        })
    } else {
        Ok(Matrix2x2 {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: gain,
        })
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MeterTap {
    Input = 1,
    PostInputBuiltins = 2,
    PostSimd1 = 3,
    PostDynamic = 4,
    PostSimd2PreFader = 5,
    PostFader = 6,
    PostMatrix = 7,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MeterHandle(pub NonZeroU64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterConfig {
    pub period_frames: NonZeroU32,
    pub peak_hold_frames: u32,
    pub peak_decay_db_per_second: f32,
    pub queue_capacity: NonZeroUsize,
    pub reset_generation: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeterConfigError {
    DecayDomain,
    Queue,
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MeterLaneSnapshot {
    pub sample_peak: f32,
    pub rms: f64,
    pub energy: f64,
    pub held_peak: f32,
    pub clipped_samples: u64,
    pub sanitized_samples: u64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterSnapshot {
    pub handle: MeterHandle,
    pub reset_generation: u64,
    pub window_sequence: u64,
    pub start_sample: u64,
    pub end_sample: u64,
    pub frames: u32,
    pub left: MeterLaneSnapshot,
    pub right: MeterLaneSnapshot,
    pub cumulative_clipped_samples: u64,
    pub cumulative_sanitized_samples: u64,
    pub cumulative_discontinuities: u64,
    pub cumulative_dropped_snapshots: u64,
}

struct MeterLane {
    peak: f32,
    energy: f64,
    clipped: u64,
    sanitized: u64,
    held: f32,
    hold_remaining: u32,
}
pub struct MeterAccumulator {
    handle: MeterHandle,
    config: MeterConfig,
    decay: f32,
    start: Option<u64>,
    frames: u32,
    sequence: u64,
    left: MeterLane,
    right: MeterLane,
    cumulative_clipped: u64,
    cumulative_sanitized: u64,
    discontinuities: u64,
    dropped: u64,
    producer: Producer<MeterSnapshot>,
}

pub struct PreparedMeter {
    pub accumulator: MeterAccumulator,
    pub consumer: Consumer<MeterSnapshot>,
}

impl MeterAccumulator {
    pub fn prepare(
        handle: MeterHandle,
        config: MeterConfig,
        sample_rate: u32,
    ) -> Result<PreparedMeter, MeterConfigError> {
        if !config.peak_decay_db_per_second.is_finite()
            || !(0.0..=120.0).contains(&config.peak_decay_db_per_second)
            || sample_rate == 0
        {
            return Err(MeterConfigError::DecayDomain);
        }
        let (producer, consumer) = bounded_spsc(
            config.queue_capacity,
            QueueGeneration(config.reset_generation),
        )
        .map_err(|_| MeterConfigError::Queue)?;
        let decay = 10.0_f64
            .powf(-f64::from(config.peak_decay_db_per_second) / (20.0 * f64::from(sample_rate)))
            as f32;
        Ok(PreparedMeter {
            accumulator: Self {
                handle,
                config,
                decay: if normal_or_zero(decay) { decay } else { 0.0 },
                start: None,
                frames: 0,
                sequence: 0,
                left: meter_lane(),
                right: meter_lane(),
                cumulative_clipped: 0,
                cumulative_sanitized: 0,
                discontinuities: 0,
                dropped: 0,
                producer,
            },
            consumer,
        })
    }
    pub fn observe(&mut self, left: &[f32], right: &[f32], first_sample: u64) {
        if left.len() != right.len() {
            return;
        }
        let len = match u64::try_from(left.len())
            .ok()
            .and_then(|len| first_sample.checked_add(len))
        {
            Some(_) => left.len(),
            None => {
                self.discontinuity(first_sample);
                return;
            }
        };
        if self
            .start
            .is_some_and(|start| first_sample != start.saturating_add(u64::from(self.frames)))
        {
            self.discontinuity(first_sample);
        }
        if self.start.is_none() {
            self.start = Some(first_sample);
        }
        for index in 0..len {
            observe_lane(
                &mut self.left,
                left[index],
                self.config,
                self.decay,
                &mut self.cumulative_clipped,
                &mut self.cumulative_sanitized,
            );
            observe_lane(
                &mut self.right,
                right[index],
                self.config,
                self.decay,
                &mut self.cumulative_clipped,
                &mut self.cumulative_sanitized,
            );
            self.frames = self.frames.saturating_add(1);
            if self.frames == self.config.period_frames.get() {
                self.emit();
            }
        }
    }
    pub fn reset(&mut self, kind: BuiltinResetKind) {
        self.start = None;
        self.frames = 0;
        self.left = meter_lane();
        self.right = meter_lane();
        if matches!(kind, BuiltinResetKind::FullToPrepared) {
            self.sequence = 0;
            self.cumulative_clipped = 0;
            self.cumulative_sanitized = 0;
            self.discontinuities = 0;
            self.dropped = 0;
        }
    }
    fn discontinuity(&mut self, first_sample: u64) {
        self.start = Some(first_sample);
        self.frames = 0;
        self.left = meter_lane();
        self.right = meter_lane();
        self.discontinuities = self.discontinuities.saturating_add(1);
    }
    fn emit(&mut self) {
        let start = self.start.expect("meter start initialized");
        let end = match start.checked_add(u64::from(self.frames)) {
            Some(value) => value,
            None => {
                self.discontinuity(start);
                return;
            }
        };
        let snapshot = MeterSnapshot {
            handle: self.handle,
            reset_generation: self.config.reset_generation,
            window_sequence: self.sequence,
            start_sample: start,
            end_sample: end,
            frames: self.frames,
            left: lane_snapshot(&self.left, self.frames),
            right: lane_snapshot(&self.right, self.frames),
            cumulative_clipped_samples: self.cumulative_clipped,
            cumulative_sanitized_samples: self.cumulative_sanitized,
            cumulative_discontinuities: self.discontinuities,
            cumulative_dropped_snapshots: self.dropped,
        };
        if self.producer.try_push(snapshot).is_err() {
            self.dropped = self.dropped.saturating_add(1);
        }
        self.sequence = self.sequence.saturating_add(1);
        self.start = Some(end);
        self.frames = 0;
        clear_interval(&mut self.left);
        clear_interval(&mut self.right);
    }
}

fn meter_lane() -> MeterLane {
    MeterLane {
        peak: 0.0,
        energy: 0.0,
        clipped: 0,
        sanitized: 0,
        held: 0.0,
        hold_remaining: 0,
    }
}
fn clear_interval(lane: &mut MeterLane) {
    lane.peak = 0.0;
    lane.energy = 0.0;
    lane.clipped = 0;
    lane.sanitized = 0;
}
fn observe_lane(
    lane: &mut MeterLane,
    sample: f32,
    config: MeterConfig,
    decay: f32,
    cumulative_clipped: &mut u64,
    cumulative_sanitized: &mut u64,
) {
    let sanitized = !normal_or_zero(sample);
    let sample = if sanitized { 0.0 } else { sample };
    if sanitized {
        lane.sanitized = lane.sanitized.saturating_add(1);
        *cumulative_sanitized = cumulative_sanitized.saturating_add(1);
    }
    let absolute = sample.abs();
    lane.peak = lane.peak.max(absolute);
    lane.energy += f64::from(sample) * f64::from(sample);
    if absolute >= 1.0 {
        lane.clipped = lane.clipped.saturating_add(1);
        *cumulative_clipped = cumulative_clipped.saturating_add(1);
    }
    if absolute >= lane.held {
        lane.held = absolute;
        lane.hold_remaining = config.peak_hold_frames;
    } else if lane.hold_remaining > 0 {
        lane.hold_remaining -= 1;
    } else if config.peak_decay_db_per_second != 0.0 {
        lane.held = sanitize(lane.held * decay, &mut 0);
    }
}
fn lane_snapshot(lane: &MeterLane, frames: u32) -> MeterLaneSnapshot {
    MeterLaneSnapshot {
        sample_peak: lane.peak,
        rms: (lane.energy / f64::from(frames)).sqrt(),
        energy: lane.energy,
        held_peak: lane.held,
        clipped_samples: lane.clipped,
        sanitized_samples: lane.sanitized,
    }
}

fn db_gain(db: f32) -> Result<f32, BuiltinParameterError> {
    let value = 10.0_f64.powf(f64::from(db) / 20.0) as f32;
    if normal_or_zero(value) {
        Ok(zero(value))
    } else {
        Err(BuiltinParameterError::GainDomain)
    }
}
fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && !value.is_subnormal()
}
fn zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}
fn sanitize_input(value: f32, count: &mut u64) -> f32 {
    if normal_or_zero(value) {
        value
    } else {
        *count = count.saturating_add(1);
        0.0
    }
}
fn sanitize(value: f32, count: &mut u64) -> f32 {
    sanitize_input(value, count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_core::{EXTENDED_COMPATIBILITY_SAMPLE_RATES, LAUNCH_SAMPLE_RATES};
    use miso_engine_dsp_reference::{
        ReferenceBiquad, ReferenceFilterKind, ReferenceTptOutput, ReferenceTptStateSpace,
        rbj_butterworth_magnitude_db,
    };

    // Issue 032: the first tier is launch-gated; the second remains informational compatibility
    // evidence from issue 007 and is not an engine session or host support claim.
    fn launch_and_extended_compatibility_rates() -> impl Iterator<Item = u32> {
        LAUNCH_SAMPLE_RATES
            .into_iter()
            .chain(EXTENDED_COMPATIBILITY_SAMPLE_RATES)
            .map(|rate| rate.0)
    }

    #[test]
    fn polarity_trim_fader_and_matrix_are_exact() {
        let mut chain = BuiltinChain::new(
            48_000,
            BuiltinParameters {
                left: ChannelParameters {
                    polarity_invert: true,
                    trim_db: 6.0206,
                    fader_db: 0.0,
                    ..ChannelParameters::default()
                },
                right: ChannelParameters::default(),
                matrix: Matrix2x2::IDENTITY,
                smoothing_samples: 0,
            },
        )
        .expect("prepare");
        let mut left = [0.5_f32];
        let mut right = [0.0_f32];
        chain.process_dual_mono(DualMonoBlock {
            left: &mut left,
            right: &mut right,
            first_sample: 0,
        });
        assert!((left[0] + 1.0).abs() < 2e-5);
        assert_eq!(right, [0.0]);
    }
    #[test]
    fn matrix_ramp_reaches_target() {
        let mut chain = BuiltinChain::new(
            48_000,
            BuiltinParameters {
                smoothing_samples: 2,
                ..BuiltinParameters::default()
            },
        )
        .expect("prepare");
        chain
            .set_matrix_target(Matrix2x2 {
                ll: 0.0,
                lr: 0.0,
                rl: 0.0,
                rr: 0.0,
            })
            .expect("target");
        let mut left = [1.0, 1.0];
        let mut right = [0.0, 0.0];
        chain.process_matrix(DualMonoBlock {
            left: &mut left,
            right: &mut right,
            first_sample: 0,
        });
        assert_eq!(left, [0.5, 0.0]);
    }
    #[test]
    fn meter_windows_are_exact() {
        let handle = MeterHandle(NonZeroU64::new(1).expect("constant"));
        let config = MeterConfig {
            period_frames: NonZeroU32::new(2).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(2).expect("constant"),
            reset_generation: 7,
        };
        let PreparedMeter {
            mut accumulator,
            mut consumer,
        } = MeterAccumulator::prepare(handle, config, 48_000).expect("meter");
        accumulator.observe(&[1.0, 0.5], &[0.0, -1.0], 3);
        let snap = consumer.try_pop().expect("snapshot");
        assert_eq!(snap.start_sample, 3);
        assert_eq!(snap.end_sample, 5);
        assert_eq!(snap.left.clipped_samples, 1);
        assert_eq!(snap.right.clipped_samples, 1);
    }
    #[test]
    fn launch_and_extended_compatibility_rates_match_the_independent_f64_rbj_oracle() {
        for rate in launch_and_extended_compatibility_rates() {
            let parameters = BuiltinParameters {
                left: ChannelParameters {
                    hpf_hz: 100.0,
                    lpf_hz: 1_000.0,
                    ..ChannelParameters::default()
                },
                ..BuiltinParameters::default()
            };
            let mut chain = BuiltinChain::new(rate, parameters).expect("prepare");
            let mut left = [0.0_f32; 256];
            let mut right = [0.0_f32; 256];
            left[0] = 1.0;
            let mut high = ReferenceBiquad::rbj_butterworth(
                f64::from(rate),
                100.0,
                ReferenceFilterKind::HighPass,
            )
            .expect("reference high pass");
            let mut low = ReferenceBiquad::rbj_butterworth(
                f64::from(rate),
                1_000.0,
                ReferenceFilterKind::LowPass,
            )
            .expect("reference low pass");
            let expected: Vec<_> = (0..left.len())
                .map(|index| low.process(high.process(if index == 0 { 1.0 } else { 0.0 })))
                .collect();
            let _ = chain.process_input(DualMonoBlock {
                left: &mut left,
                right: &mut right,
                first_sample: 0,
            });
            for (actual, reference) in left.iter().zip(expected) {
                assert!(
                    (f64::from(*actual) - reference).abs() <= 2e-5,
                    "rate={rate}, actual={actual}, reference={reference}"
                );
            }
            assert_eq!(right, [0.0; 256]);
        }
    }
    #[test]
    fn ten_thousand_bounded_parameter_and_block_mutations_stay_finite() {
        let mut state = 0x5EED_CAFE_1234_5678_u64;
        for iteration in 0..10_000_u64 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let fraction = |shift| ((state >> shift) as u32) as f32 / u32::MAX as f32;
            let db = |shift| -144.0 + fraction(shift) * 168.0;
            let matrix = Matrix2x2 {
                ll: fraction(0) * 2.0 - 1.0,
                lr: fraction(8) * 2.0 - 1.0,
                rl: fraction(16) * 2.0 - 1.0,
                rr: fraction(24) * 2.0 - 1.0,
            };
            let rate = [44_100, 48_000, 88_200, 96_000][(state as usize) & 3];
            let mut chain = BuiltinChain::new(
                rate,
                BuiltinParameters {
                    left: ChannelParameters {
                        polarity_invert: state & 1 != 0,
                        trim_db: db(0),
                        hpf_hz: 100.0,
                        lpf_hz: 1_000.0,
                        fader_db: db(32),
                        muted: state & 2 != 0,
                    },
                    right: ChannelParameters {
                        polarity_invert: state & 4 != 0,
                        trim_db: db(8),
                        hpf_hz: 0.0,
                        lpf_hz: 0.0,
                        fader_db: db(40),
                        muted: state & 8 != 0,
                    },
                    matrix,
                    smoothing_samples: (state as u32) & 127,
                },
            )
            .expect("generated parameters are in the prepared domain");
            chain
                .set_matrix_target(Matrix2x2::IDENTITY)
                .expect("identity");
            let mut left = [0.25_f32; 127];
            let mut right = [-0.5_f32; 127];
            let _ = chain.process_dual_mono(DualMonoBlock {
                left: &mut left,
                right: &mut right,
                first_sample: iteration.saturating_mul(127),
            });
            assert!(left.iter().chain(&right).all(|sample| sample.is_finite()));
        }
    }
    #[test]
    fn launch_and_extended_compatibility_rate_sweeps_match_f64_magnitude() {
        for rate in launch_and_extended_compatibility_rates() {
            for frequency in [100.0, 1_000.0, f64::from(rate) * 0.2] {
                let frames = 4_096;
                let mut left: Vec<f32> = (0..frames)
                    .map(|index| {
                        (core::f64::consts::TAU * frequency * index as f64 / f64::from(rate)).sin()
                            as f32
                    })
                    .collect();
                let mut right = vec![0.0_f32; frames];
                let parameters = BuiltinParameters {
                    left: ChannelParameters {
                        hpf_hz: 100.0,
                        lpf_hz: 1_000.0,
                        ..ChannelParameters::default()
                    },
                    ..BuiltinParameters::default()
                };
                let mut chain = BuiltinChain::new(rate, parameters).expect("prepare");
                let mut offset = 0;
                for quantum in [1, 127, 128, 255, 1_024].into_iter().cycle() {
                    if offset == frames {
                        break;
                    }
                    let end = (offset + quantum).min(frames);
                    let _ = chain.process_input(DualMonoBlock {
                        left: &mut left[offset..end],
                        right: &mut right[offset..end],
                        first_sample: offset as u64,
                    });
                    offset = end;
                }
                let mut high = ReferenceBiquad::rbj_butterworth(
                    f64::from(rate),
                    100.0,
                    ReferenceFilterKind::HighPass,
                )
                .expect("reference high pass");
                let mut low = ReferenceBiquad::rbj_butterworth(
                    f64::from(rate),
                    1_000.0,
                    ReferenceFilterKind::LowPass,
                )
                .expect("reference low pass");
                let mut actual_energy = 0.0_f64;
                let mut reference_energy = 0.0_f64;
                for index in 0..frames {
                    let input =
                        (core::f64::consts::TAU * frequency * index as f64 / f64::from(rate)).sin();
                    let reference = low.process(high.process(input));
                    if index >= frames / 2 {
                        actual_energy += f64::from(left[index]) * f64::from(left[index]);
                        reference_energy += reference * reference;
                    }
                }
                let actual_db = 10.0 * actual_energy.log10();
                let reference_db = 10.0 * reference_energy.log10();
                if reference_db >= -120.0 {
                    assert!(
                        (actual_db - reference_db).abs() <= 0.05,
                        "rate={rate}, frequency={frequency}, actual={actual_db}, reference={reference_db}"
                    );
                }
            }
        }
    }
    #[test]
    fn cast_tpt_state_space_matches_independent_rbj_transfer_at_compatibility_rates() {
        for rate in launch_and_extended_compatibility_rates() {
            let mut cutoffs = vec![
                10.0,
                20.0,
                100.0,
                1_000.0,
                (20_000.0_f64).min(0.1 * f64::from(rate)),
                0.45 * f64::from(rate),
            ];
            cutoffs.sort_by(f64::total_cmp);
            cutoffs.dedup_by(|left, right| *left == *right);
            for (high_pass, kind, output) in [
                (
                    true,
                    ReferenceFilterKind::HighPass,
                    ReferenceTptOutput::HighPass,
                ),
                (
                    false,
                    ReferenceFilterKind::LowPass,
                    ReferenceTptOutput::LowPass,
                ),
            ] {
                for cutoff in &cutoffs {
                    let filter = TptSvf::design(rate, *cutoff as f32, high_pass).expect("valid");
                    let state = ReferenceTptStateSpace::from_cast_coefficients(
                        filter.a1, filter.a2, filter.a3, filter.k, output,
                    );
                    let mut probes = vec![
                        0.25 * cutoff,
                        *cutoff,
                        4.0 * cutoff,
                        0.2 * f64::from(rate),
                        0.45 * f64::from(rate),
                        0.49 * f64::from(rate),
                    ];
                    probes.retain(|probe| *probe > 0.0 && *probe < 0.5 * f64::from(rate));
                    probes.sort_by(f64::total_cmp);
                    probes.dedup_by(|left, right| *left == *right);
                    for frequency in probes {
                        let reference =
                            rbj_butterworth_magnitude_db(f64::from(rate), *cutoff, kind, frequency)
                                .expect("reference");
                        let actual = state
                            .magnitude_db(f64::from(rate), frequency)
                            .expect("state");
                        if reference >= -120.0 {
                            assert!(
                                (actual - reference).abs() <= 0.005,
                                "rate={rate}, cutoff={cutoff}, frequency={frequency}, actual={actual}, reference={reference}"
                            );
                        }
                    }
                    let cutoff_db = state
                        .magnitude_db(f64::from(rate), *cutoff)
                        .expect("cutoff state");
                    assert!(
                        (cutoff_db + 3.010_299_956_6).abs() <= 0.005,
                        "rate={rate}, cutoff={cutoff}, db={cutoff_db}"
                    );
                }
            }
        }
    }
    #[test]
    fn one_second_impulse_dfts_match_rbj_at_launch_and_extended_compatibility_rates() {
        for rate in launch_and_extended_compatibility_rates() {
            let mut cutoffs = vec![
                10.0,
                20.0,
                100.0,
                1_000.0,
                (20_000.0_f64).min(0.1 * f64::from(rate)),
                0.45 * f64::from(rate),
            ];
            cutoffs.sort_by(f64::total_cmp);
            cutoffs.dedup_by(|left, right| *left == *right);
            for (high_pass, kind) in [
                (true, ReferenceFilterKind::HighPass),
                (false, ReferenceFilterKind::LowPass),
            ] {
                for cutoff in &cutoffs {
                    for quantum in [1, 127, 128, 255, 1_024] {
                        let mut filter = TptSvf::design(rate, *cutoff as f32, high_pass)
                            .expect("valid matrix cutoff");
                        let mut report = BuiltinProcessReport::default();
                        let mut recovered = 0;
                        let mut impulse = vec![0.0_f32; rate as usize];
                        for block_start in (0..impulse.len()).step_by(quantum) {
                            let block_end = (block_start + quantum).min(impulse.len());
                            for (index, sample) in
                                impulse[block_start..block_end].iter_mut().enumerate()
                            {
                                *sample = filter.process(
                                    if block_start + index == 0 { 1.0 } else { 0.0 },
                                    &mut recovered,
                                    &mut report,
                                );
                            }
                        }
                        assert!(impulse.iter().all(|sample| sample.is_finite()));
                        for frequency in coherent_probes(rate, *cutoff) {
                            let reference = rbj_butterworth_magnitude_db(
                                f64::from(rate),
                                *cutoff,
                                kind,
                                frequency,
                            )
                            .expect("reference");
                            let actual =
                                impulse_dft_magnitude_db(&impulse, f64::from(rate), frequency);
                            if reference >= -120.0 {
                                assert!(
                                    (actual - reference).abs() <= 0.05,
                                    "rate={rate}, cutoff={cutoff}, quantum={quantum}, frequency={frequency}, actual={actual}, reference={reference}"
                                );
                            } else {
                                assert!(
                                    actual <= -115.0,
                                    "rate={rate}, cutoff={cutoff}, quantum={quantum}, frequency={frequency}, actual={actual}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn coherent_sustained_sines_cover_launch_and_extended_compatibility_rates() {
        for rate in launch_and_extended_compatibility_rates() {
            let mut cutoffs = vec![
                10.0,
                20.0,
                100.0,
                1_000.0,
                (20_000.0_f64).min(0.1 * f64::from(rate)),
                0.45 * f64::from(rate),
            ];
            cutoffs.sort_by(f64::total_cmp);
            cutoffs.dedup_by(|left, right| *left == *right);
            for (high_pass, kind) in [
                (true, ReferenceFilterKind::HighPass),
                (false, ReferenceFilterKind::LowPass),
            ] {
                for cutoff in &cutoffs {
                    for frequency in coherent_probes(rate, *cutoff) {
                        let mut production = TptSvf::design(rate, *cutoff as f32, high_pass)
                            .expect("valid matrix cutoff");
                        let mut reference =
                            ReferenceBiquad::rbj_butterworth(f64::from(rate), *cutoff, kind)
                                .expect("reference");
                        let measurement =
                            sustained_measurement(&mut production, &mut reference, rate, frequency);
                        if measurement.reference_gain_db >= -90.0 {
                            assert!(
                                (measurement.production_gain_db - measurement.reference_gain_db)
                                    .abs()
                                    <= 0.05,
                                "rate={rate}, cutoff={cutoff}, frequency={frequency}, production={}, reference={}",
                                measurement.production_gain_db,
                                measurement.reference_gain_db
                            );
                            assert!(
                                measurement.residual_db <= -100.0,
                                "rate={rate}, cutoff={cutoff}, frequency={frequency}, residual={}",
                                measurement.residual_db
                            );
                        } else {
                            assert!(
                                measurement.total_output_db <= -88.0,
                                "rate={rate}, cutoff={cutoff}, frequency={frequency}, output={}",
                                measurement.total_output_db
                            );
                        }
                    }
                }
            }
        }
    }

    struct SustainedMeasurement {
        production_gain_db: f64,
        reference_gain_db: f64,
        residual_db: f64,
        total_output_db: f64,
    }

    fn sustained_measurement(
        production: &mut TptSvf,
        reference: &mut ReferenceBiquad,
        rate: u32,
        frequency: f64,
    ) -> SustainedMeasurement {
        let settle = rate as usize / 2;
        let frames = rate as usize / 4;
        let mut report = BuiltinProcessReport::default();
        let mut recovered = 0;
        let mut production_sum = 0.0_f64;
        let mut production_sine = 0.0_f64;
        let mut production_cosine = 0.0_f64;
        let mut reference_sine = 0.0_f64;
        let mut reference_cosine = 0.0_f64;
        let mut input_energy = 0.0_f64;
        let mut output_energy = 0.0_f64;
        let mut measured_outputs = Vec::with_capacity(frames);
        let rate_f64 = f64::from(rate);
        for index in 0..settle + frames {
            let phase = core::f64::consts::TAU * frequency * index as f64 / rate_f64;
            let input = (0.5 * phase.sin()) as f32;
            let output = production.process(input, &mut recovered, &mut report);
            let reference_output = reference.process(f64::from(input));
            if index >= settle {
                let sine = phase.sin();
                let cosine = phase.cos();
                let output = f64::from(output);
                measured_outputs.push(output);
                production_sum += output;
                production_sine += output * sine;
                production_cosine += output * cosine;
                reference_sine += reference_output * sine;
                reference_cosine += reference_output * cosine;
                input_energy += f64::from(input) * f64::from(input);
                output_energy += output * output;
            }
        }
        let frames_f64 = frames as f64;
        let input_rms = (input_energy / frames_f64).sqrt();
        let production_dc = production_sum / frames_f64;
        let production_sine_coefficient = 2.0 * production_sine / frames_f64;
        let production_cosine_coefficient = 2.0 * production_cosine / frames_f64;
        let production_amplitude = production_sine_coefficient.hypot(production_cosine_coefficient);
        let reference_amplitude =
            (2.0 * reference_sine / frames_f64).hypot(2.0 * reference_cosine / frames_f64);
        let mut residual_energy = 0.0_f64;
        for (offset, output) in measured_outputs.into_iter().enumerate() {
            let index = settle + offset;
            let phase = core::f64::consts::TAU * frequency * index as f64 / rate_f64;
            let fitted = production_dc
                + production_sine_coefficient * phase.sin()
                + production_cosine_coefficient * phase.cos();
            residual_energy += (output - fitted).powi(2);
        }
        let residual_rms = (residual_energy / frames_f64).sqrt();
        let output_rms = (output_energy / frames_f64).sqrt();
        SustainedMeasurement {
            production_gain_db: 20.0 * (production_amplitude / 0.5).log10(),
            reference_gain_db: 20.0 * (reference_amplitude / 0.5).log10(),
            residual_db: 20.0 * (residual_rms / input_rms).log10(),
            total_output_db: 20.0 * (output_rms / input_rms).log10(),
        }
    }

    fn coherent_probes(rate: u32, cutoff: f64) -> Vec<f64> {
        let nyquist = 0.5 * f64::from(rate);
        let mut probes = [
            0.25 * cutoff,
            cutoff,
            4.0 * cutoff,
            0.2 * f64::from(rate),
            0.45 * f64::from(rate),
        ]
        .into_iter()
        .map(|probe| probe.clamp(4.0, nyquist - 4.0))
        .map(|probe| (probe / 4.0).round() * 4.0)
        .collect::<Vec<_>>();
        probes.sort_by(f64::total_cmp);
        probes.dedup_by(|left, right| *left == *right);
        probes
    }

    fn impulse_dft_magnitude_db(samples: &[f32], rate: f64, frequency: f64) -> f64 {
        let phase = -core::f64::consts::TAU * frequency / rate;
        let (step_real, step_imaginary) = (phase.cos(), phase.sin());
        let (mut unit_real, mut unit_imaginary) = (1.0_f64, 0.0_f64);
        let (mut real, mut imaginary) = (0.0_f64, 0.0_f64);
        for sample in samples {
            let sample = f64::from(*sample);
            real += sample * unit_real;
            imaginary += sample * unit_imaginary;
            (unit_real, unit_imaginary) = (
                unit_real * step_real - unit_imaginary * step_imaginary,
                unit_real * step_imaginary + unit_imaginary * step_real,
            );
        }
        let magnitude = real.hypot(imaginary);
        if magnitude == 0.0 {
            f64::NEG_INFINITY
        } else {
            20.0 * magnitude.log10()
        }
    }
}
