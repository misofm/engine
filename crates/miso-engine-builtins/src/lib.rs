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
    pub const IDENTITY: Self = Self { ll: 1.0, lr: 0.0, rl: 0.0, rr: 1.0 };

    pub fn checked(self) -> Result<Self, BuiltinParameterError> {
        if [self.ll, self.lr, self.rl, self.rr]
            .into_iter()
            .all(|v| v.is_finite() && (-1.0..=1.0).contains(&v))
        {
            Ok(Self { ll: zero(self.ll), lr: zero(self.lr), rl: zero(self.rl), rr: zero(self.rr) })
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
        self.recovered_left_state = self.recovered_left_state.saturating_add(other.recovered_left_state);
        self.recovered_right_state = self.recovered_right_state.saturating_add(other.recovered_right_state);
    }
}

pub struct DualMonoBlock<'a> {
    pub left: &'a mut [f32],
    pub right: &'a mut [f32],
    pub first_sample: u64,
}

impl DualMonoBlock<'_> {
    fn checked_len(&self) -> Result<usize, BuiltinParameterError> {
        if self.left.is_empty() { return Err(BuiltinParameterError::EmptyBlock); }
        if self.left.len() != self.right.len() { return Err(BuiltinParameterError::LaneLength); }
        let len = u64::try_from(self.left.len()).map_err(|_| BuiltinParameterError::SampleTimeOverflow)?;
        self.first_sample.checked_add(len).ok_or(BuiltinParameterError::SampleTimeOverflow)?;
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
        Self { polarity_invert: false, trim_db: 0.0, hpf_hz: 0.0, lpf_hz: 0.0, fader_db: 0.0, muted: false }
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
    fn default() -> Self { Self { left: ChannelParameters::default(), right: ChannelParameters::default(), matrix: Matrix2x2::IDENTITY, smoothing_samples: 0 } }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinTail { FiniteZero, Infinite }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinParameterDescriptorV1 {
    pub id: u32,
    pub name: &'static str,
    pub per_lane: bool,
}

pub const BUILTIN_PARAMETER_DESCRIPTORS_V1: [BuiltinParameterDescriptorV1; 10] = [
    BuiltinParameterDescriptorV1 { id: 1, name: "polarity_invert", per_lane: true },
    BuiltinParameterDescriptorV1 { id: 2, name: "trim_db", per_lane: true },
    BuiltinParameterDescriptorV1 { id: 3, name: "hpf_hz", per_lane: true },
    BuiltinParameterDescriptorV1 { id: 4, name: "lpf_hz", per_lane: true },
    BuiltinParameterDescriptorV1 { id: 5, name: "fader_db", per_lane: true },
    BuiltinParameterDescriptorV1 { id: 6, name: "mute", per_lane: true },
    BuiltinParameterDescriptorV1 { id: 7, name: "matrix_ll", per_lane: false },
    BuiltinParameterDescriptorV1 { id: 8, name: "matrix_lr", per_lane: false },
    BuiltinParameterDescriptorV1 { id: 9, name: "matrix_rl", per_lane: false },
    BuiltinParameterDescriptorV1 { id: 10, name: "matrix_rr", per_lane: false },
];

#[derive(Clone, Copy)]
struct Biquad { b0: f32, b1: f32, b2: f32, a1: f32, a2: f32, z1: f32, z2: f32, enabled: bool }

impl Biquad {
    fn identity() -> Self { Self { b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0, z1: 0.0, z2: 0.0, enabled: false } }
    fn design(rate: u32, cutoff: f32, high_pass: bool) -> Result<Self, BuiltinParameterError> {
        if cutoff == 0.0 { return Ok(Self::identity()); }
        if !cutoff.is_finite() || cutoff <= 0.0 || f64::from(cutoff) >= f64::from(rate) / 2.0 { return Err(BuiltinParameterError::FilterCutoff); }
        let w0 = 2.0_f64 * core::f64::consts::PI * f64::from(cutoff) / f64::from(rate);
        let cosine = w0.cos();
        let alpha = w0.sin() / (2.0 * core::f64::consts::FRAC_1_SQRT_2);
        let (b0, b1, b2) = if high_pass { ((1.0 + cosine) / 2.0, -(1.0 + cosine), (1.0 + cosine) / 2.0) } else { ((1.0 - cosine) / 2.0, 1.0 - cosine, (1.0 - cosine) / 2.0) };
        let a0 = 1.0 + alpha;
        let values = [b0 / a0, b1 / a0, b2 / a0, -2.0 * cosine / a0, (1.0 - alpha) / a0];
        let values: [f32; 5] = values.map(|value| value as f32);
        if !values.into_iter().all(normal_or_zero) { return Err(BuiltinParameterError::FilterCoefficients); }
        let [b0, b1, b2, a1, a2] = values;
        if a2.abs() >= 1.0 || 1.0 + a1 + a2 <= 0.0 || 1.0 - a1 + a2 <= 0.0 { return Err(BuiltinParameterError::FilterCoefficients); }
        Ok(Self { b0, b1, b2, a1, a2, z1: 0.0, z2: 0.0, enabled: true })
    }
    fn reset(&mut self) { self.z1 = 0.0; self.z2 = 0.0; }
    fn process(&mut self, input: f32, recovered: &mut u64, report: &mut BuiltinProcessReport) -> f32 {
        if !self.enabled { return input; }
        if !normal_or_zero(self.z1) || !normal_or_zero(self.z2) { self.reset(); *recovered = recovered.saturating_add(1); }
        let y = self.b0 * input + self.z1;
        let z1 = self.b1 * input - self.a1 * y + self.z2;
        let z2 = self.b2 * input - self.a2 * y;
        self.z1 = sanitize(z1, &mut report.sanitized_output);
        self.z2 = sanitize(z2, &mut report.sanitized_output);
        sanitize(y, &mut report.sanitized_output)
    }
}

#[derive(Clone, Copy)]
struct InputLane { polarity: bool, trim: f32, hpf: Biquad, lpf: Biquad }
#[derive(Clone, Copy)]
struct FaderLane { gain: f32, muted: bool }

pub struct InputBuiltins { left: InputLane, right: InputLane, recovered_left: u64, recovered_right: u64 }
pub struct FaderMuteBuiltins { left: FaderLane, right: FaderLane }
pub struct MatrixBuiltins { current: Matrix2x2, target: Matrix2x2, smoothing_samples: u32, remaining_updates: u32 }

pub struct BuiltinChain { input: InputBuiltins, fader_mute: FaderMuteBuiltins, matrix: MatrixBuiltins }

impl BuiltinChain {
    pub fn new(sample_rate: u32, parameters: BuiltinParameters) -> Result<Self, BuiltinParameterError> {
        let (input, fader_mute, matrix) = prepare_sections(sample_rate, parameters)?;
        Ok(Self { input, fader_mute, matrix })
    }
    pub fn process_input(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport { self.input.process(block) }
    pub fn process_fader_mute(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport { self.fader_mute.process(block) }
    pub fn process_matrix(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport { self.matrix.process(block) }
    pub fn process_dual_mono(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let DualMonoBlock { left, right, first_sample } = block;
        let mut report = self.input.process(DualMonoBlock { left, right, first_sample });
        report.add(self.fader_mute.process(DualMonoBlock { left, right, first_sample }));
        report.add(self.matrix.process(DualMonoBlock { left, right, first_sample }));
        report
    }
    pub fn set_matrix_target(&mut self, target: Matrix2x2) -> Result<(), BuiltinParameterError> { self.matrix.set_target(target) }
    pub fn reset(&mut self, kind: BuiltinResetKind) { self.input.reset(); self.matrix.reset(); if matches!(kind, BuiltinResetKind::FullToPrepared) { self.fader_mute.reset(); } }
    pub fn link_mode(&self) -> ChannelLinkMode { ChannelLinkMode::ExplicitMatrix2x2 }
    pub fn tail(&self) -> BuiltinTail { self.input.tail() }
    pub fn into_sections(self) -> (InputBuiltins, FaderMuteBuiltins, MatrixBuiltins) { (self.input, self.fader_mute, self.matrix) }
}

fn prepare_sections(sample_rate: u32, parameters: BuiltinParameters) -> Result<(InputBuiltins, FaderMuteBuiltins, MatrixBuiltins), BuiltinParameterError> {
    if sample_rate == 0 { return Err(BuiltinParameterError::FilterCutoff); }
    parameters.matrix.checked()?;
    for lane in [parameters.left, parameters.right] {
        if !lane.trim_db.is_finite() || !(-144.0..=24.0).contains(&lane.trim_db) || !lane.fader_db.is_finite() || !(-144.0..=24.0).contains(&lane.fader_db) { return Err(BuiltinParameterError::GainDomain); }
        if lane.hpf_hz > 0.0 && lane.lpf_hz > 0.0 && lane.hpf_hz >= lane.lpf_hz { return Err(BuiltinParameterError::FilterOrder); }
    }
    let lane = |params: ChannelParameters| -> Result<InputLane, BuiltinParameterError> { Ok(InputLane { polarity: params.polarity_invert, trim: db_gain(params.trim_db)?, hpf: Biquad::design(sample_rate, zero(params.hpf_hz), true)?, lpf: Biquad::design(sample_rate, zero(params.lpf_hz), false)? }) };
    let fader = |params: ChannelParameters| -> Result<FaderLane, BuiltinParameterError> { Ok(FaderLane { gain: db_gain(params.fader_db)?, muted: params.muted }) };
    Ok((InputBuiltins { left: lane(parameters.left)?, right: lane(parameters.right)?, recovered_left: 0, recovered_right: 0 }, FaderMuteBuiltins { left: fader(parameters.left)?, right: fader(parameters.right)? }, MatrixBuiltins { current: parameters.matrix, target: parameters.matrix, smoothing_samples: parameters.smoothing_samples, remaining_updates: 0 }))
}

impl InputBuiltins {
    pub fn process(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let mut report = BuiltinProcessReport::default();
        if block.checked_len().is_err() { return report; }
        for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
            *left = process_input_lane(&mut self.left, *left, &mut self.recovered_left, &mut report);
            *right = process_input_lane(&mut self.right, *right, &mut self.recovered_right, &mut report);
        }
        report.recovered_left_state = self.recovered_left;
        report.recovered_right_state = self.recovered_right;
        report
    }
    pub fn reset(&mut self) { self.left.hpf.reset(); self.left.lpf.reset(); self.right.hpf.reset(); self.right.lpf.reset(); }
    pub fn tail(&self) -> BuiltinTail { if self.left.hpf.enabled || self.left.lpf.enabled || self.right.hpf.enabled || self.right.lpf.enabled { BuiltinTail::Infinite } else { BuiltinTail::FiniteZero } }
}

fn process_input_lane(lane: &mut InputLane, sample: f32, recovered: &mut u64, report: &mut BuiltinProcessReport) -> f32 {
    let sample = sanitize_input(sample, &mut report.sanitized_input);
    let signed = if lane.polarity { -sample } else { sample };
    let trimmed = sanitize(signed * lane.trim, &mut report.sanitized_output);
    let high = lane.hpf.process(trimmed, recovered, report);
    lane.lpf.process(high, recovered, report)
}

impl FaderMuteBuiltins {
    pub fn process(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let mut report = BuiltinProcessReport::default();
        if block.checked_len().is_err() { return report; }
        for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
            *left = if self.left.muted { 0.0 } else { sanitize(*left * self.left.gain, &mut report.sanitized_output) };
            *right = if self.right.muted { 0.0 } else { sanitize(*right * self.right.gain, &mut report.sanitized_output) };
        }
        report
    }
    fn reset(&mut self) {}
}

impl MatrixBuiltins {
    pub fn set_target(&mut self, target: Matrix2x2) -> Result<(), BuiltinParameterError> {
        self.target = target.checked()?;
        self.remaining_updates = self.smoothing_samples;
        if self.remaining_updates == 0 { self.current = self.target; }
        Ok(())
    }
    pub fn process(&mut self, block: DualMonoBlock<'_>) -> BuiltinProcessReport {
        let mut report = BuiltinProcessReport::default();
        if block.checked_len().is_err() { return report; }
        for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
            self.advance();
            let in_left = sanitize_input(*left, &mut report.sanitized_input);
            let in_right = sanitize_input(*right, &mut report.sanitized_input);
            *left = sanitize(self.current.ll * in_left + self.current.lr * in_right, &mut report.sanitized_output);
            *right = sanitize(self.current.rl * in_left + self.current.rr * in_right, &mut report.sanitized_output);
        }
        report
    }
    fn advance(&mut self) {
        if self.remaining_updates == 0 { return; }
        let remaining = self.remaining_updates as f32;
        self.current.ll += (self.target.ll - self.current.ll) / remaining;
        self.current.lr += (self.target.lr - self.current.lr) / remaining;
        self.current.rl += (self.target.rl - self.current.rl) / remaining;
        self.current.rr += (self.target.rr - self.current.rr) / remaining;
        self.remaining_updates -= 1;
        if self.remaining_updates == 0 { self.current = self.target; }
    }
    pub fn reset(&mut self) { self.current = self.target; self.remaining_updates = 0; }
}

pub fn pan_matrix(left: f32, right: f32) -> Result<Matrix2x2, BuiltinParameterError> {
    if !left.is_finite() || !right.is_finite() || !(-1.0..=1.0).contains(&left) || !(-1.0..=1.0).contains(&right) { return Err(BuiltinParameterError::MatrixCoefficient); }
    let gains = |position: f32| { let theta = (f64::from(position) + 1.0) * core::f64::consts::FRAC_PI_4; (theta.cos() as f32, theta.sin() as f32) };
    let (ll, rl) = gains(left); let (lr, rr) = gains(right); Matrix2x2 { ll, lr, rl, rr }.checked()
}

pub fn balance_matrix(balance: f32) -> Result<Matrix2x2, BuiltinParameterError> {
    if !balance.is_finite() || !(-1.0..=1.0).contains(&balance) { return Err(BuiltinParameterError::MatrixCoefficient); }
    let gain = (f64::from(balance.abs()) * core::f64::consts::FRAC_PI_2).cos() as f32;
    if balance >= 0.0 { Ok(Matrix2x2 { ll: gain, lr: 0.0, rl: 0.0, rr: 1.0 }) } else { Ok(Matrix2x2 { ll: 1.0, lr: 0.0, rl: 0.0, rr: gain }) }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MeterTap { Input = 1, PostInputBuiltins = 2, PostSimd1 = 3, PostDynamic = 4, PostSimd2PreFader = 5, PostFader = 6, PostMatrix = 7 }
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MeterHandle(pub NonZeroU64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterConfig { pub period_frames: NonZeroU32, pub peak_hold_frames: u32, pub peak_decay_db_per_second: f32, pub queue_capacity: NonZeroUsize, pub reset_generation: u64 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeterConfigError { DecayDomain, Queue }
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MeterLaneSnapshot { pub sample_peak: f32, pub rms: f64, pub energy: f64, pub held_peak: f32, pub clipped_samples: u64, pub sanitized_samples: u64 }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterSnapshot { pub handle: MeterHandle, pub reset_generation: u64, pub window_sequence: u64, pub start_sample: u64, pub end_sample: u64, pub frames: u32, pub left: MeterLaneSnapshot, pub right: MeterLaneSnapshot, pub cumulative_clipped_samples: u64, pub cumulative_sanitized_samples: u64, pub cumulative_discontinuities: u64, pub cumulative_dropped_snapshots: u64 }

struct MeterLane { peak: f32, energy: f64, clipped: u64, sanitized: u64, held: f32, hold_remaining: u32 }
pub struct MeterAccumulator { handle: MeterHandle, config: MeterConfig, decay: f32, start: Option<u64>, frames: u32, sequence: u64, left: MeterLane, right: MeterLane, cumulative_clipped: u64, cumulative_sanitized: u64, discontinuities: u64, dropped: u64, producer: Producer<MeterSnapshot> }

pub struct PreparedMeter { pub accumulator: MeterAccumulator, pub consumer: Consumer<MeterSnapshot> }

impl MeterAccumulator {
    pub fn prepare(handle: MeterHandle, config: MeterConfig, sample_rate: u32) -> Result<PreparedMeter, MeterConfigError> {
        if !config.peak_decay_db_per_second.is_finite() || !(0.0..=120.0).contains(&config.peak_decay_db_per_second) || sample_rate == 0 { return Err(MeterConfigError::DecayDomain); }
        let (producer, consumer) = bounded_spsc(config.queue_capacity, QueueGeneration(config.reset_generation)).map_err(|_| MeterConfigError::Queue)?;
        let decay = 10.0_f64.powf(-f64::from(config.peak_decay_db_per_second) / (20.0 * f64::from(sample_rate))) as f32;
        Ok(PreparedMeter { accumulator: Self { handle, config, decay: if normal_or_zero(decay) { decay } else { 0.0 }, start: None, frames: 0, sequence: 0, left: meter_lane(), right: meter_lane(), cumulative_clipped: 0, cumulative_sanitized: 0, discontinuities: 0, dropped: 0, producer }, consumer })
    }
    pub fn observe(&mut self, left: &[f32], right: &[f32], first_sample: u64) {
        if left.len() != right.len() { return; }
        let len = match u64::try_from(left.len()).ok().and_then(|len| first_sample.checked_add(len)) { Some(_) => left.len(), None => { self.discontinuity(first_sample); return; } };
        if self.start.is_some_and(|start| first_sample != start.saturating_add(u64::from(self.frames))) { self.discontinuity(first_sample); }
        if self.start.is_none() { self.start = Some(first_sample); }
        for index in 0..len {
            observe_lane(&mut self.left, left[index], self.config, self.decay, &mut self.cumulative_clipped, &mut self.cumulative_sanitized);
            observe_lane(&mut self.right, right[index], self.config, self.decay, &mut self.cumulative_clipped, &mut self.cumulative_sanitized);
            self.frames = self.frames.saturating_add(1);
            if self.frames == self.config.period_frames.get() { self.emit(); }
        }
    }
    pub fn reset(&mut self, kind: BuiltinResetKind) { self.start = None; self.frames = 0; self.left = meter_lane(); self.right = meter_lane(); if matches!(kind, BuiltinResetKind::FullToPrepared) { self.sequence = 0; self.cumulative_clipped = 0; self.cumulative_sanitized = 0; self.discontinuities = 0; self.dropped = 0; } }
    fn discontinuity(&mut self, first_sample: u64) { self.start = Some(first_sample); self.frames = 0; self.left = meter_lane(); self.right = meter_lane(); self.discontinuities = self.discontinuities.saturating_add(1); }
    fn emit(&mut self) {
        let start = self.start.expect("meter start initialized");
        let end = match start.checked_add(u64::from(self.frames)) { Some(value) => value, None => { self.discontinuity(start); return; } };
        let snapshot = MeterSnapshot { handle: self.handle, reset_generation: self.config.reset_generation, window_sequence: self.sequence, start_sample: start, end_sample: end, frames: self.frames, left: lane_snapshot(&self.left, self.frames), right: lane_snapshot(&self.right, self.frames), cumulative_clipped_samples: self.cumulative_clipped, cumulative_sanitized_samples: self.cumulative_sanitized, cumulative_discontinuities: self.discontinuities, cumulative_dropped_snapshots: self.dropped };
        if self.producer.try_push(snapshot).is_err() { self.dropped = self.dropped.saturating_add(1); }
        self.sequence = self.sequence.saturating_add(1); self.start = Some(end); self.frames = 0; clear_interval(&mut self.left); clear_interval(&mut self.right);
    }
}

fn meter_lane() -> MeterLane { MeterLane { peak: 0.0, energy: 0.0, clipped: 0, sanitized: 0, held: 0.0, hold_remaining: 0 } }
fn clear_interval(lane: &mut MeterLane) { lane.peak = 0.0; lane.energy = 0.0; lane.clipped = 0; lane.sanitized = 0; }
fn observe_lane(lane: &mut MeterLane, sample: f32, config: MeterConfig, decay: f32, cumulative_clipped: &mut u64, cumulative_sanitized: &mut u64) {
    let sanitized = !normal_or_zero(sample); let sample = if sanitized { 0.0 } else { sample }; if sanitized { lane.sanitized = lane.sanitized.saturating_add(1); *cumulative_sanitized = cumulative_sanitized.saturating_add(1); }
    let absolute = sample.abs(); lane.peak = lane.peak.max(absolute); lane.energy += f64::from(sample) * f64::from(sample); if absolute >= 1.0 { lane.clipped = lane.clipped.saturating_add(1); *cumulative_clipped = cumulative_clipped.saturating_add(1); }
    if absolute >= lane.held { lane.held = absolute; lane.hold_remaining = config.peak_hold_frames; } else if lane.hold_remaining > 0 { lane.hold_remaining -= 1; } else if config.peak_decay_db_per_second != 0.0 { lane.held = sanitize(lane.held * decay, &mut 0); }
}
fn lane_snapshot(lane: &MeterLane, frames: u32) -> MeterLaneSnapshot { MeterLaneSnapshot { sample_peak: lane.peak, rms: (lane.energy / f64::from(frames)).sqrt(), energy: lane.energy, held_peak: lane.held, clipped_samples: lane.clipped, sanitized_samples: lane.sanitized } }

fn db_gain(db: f32) -> Result<f32, BuiltinParameterError> { let value = 10.0_f64.powf(f64::from(db) / 20.0) as f32; if normal_or_zero(value) { Ok(zero(value)) } else { Err(BuiltinParameterError::GainDomain) } }
fn normal_or_zero(value: f32) -> bool { value.is_finite() && !value.is_subnormal() }
fn zero(value: f32) -> f32 { if value == 0.0 { 0.0 } else { value } }
fn sanitize_input(value: f32, count: &mut u64) -> f32 { if normal_or_zero(value) { value } else { *count = count.saturating_add(1); 0.0 } }
fn sanitize(value: f32, count: &mut u64) -> f32 { sanitize_input(value, count) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn polarity_trim_fader_and_matrix_are_exact() { let mut chain = BuiltinChain::new(48_000, BuiltinParameters { left: ChannelParameters { polarity_invert: true, trim_db: 6.0206, fader_db: 0.0, ..ChannelParameters::default() }, right: ChannelParameters::default(), matrix: Matrix2x2::IDENTITY, smoothing_samples: 0 }).expect("prepare"); let mut left = [0.5_f32]; let mut right = [0.0_f32]; chain.process_dual_mono(DualMonoBlock { left: &mut left, right: &mut right, first_sample: 0 }); assert!((left[0] + 1.0).abs() < 2e-5); assert_eq!(right, [0.0]); }
    #[test] fn matrix_ramp_reaches_target() { let mut chain = BuiltinChain::new(48_000, BuiltinParameters { smoothing_samples: 2, ..BuiltinParameters::default() }).expect("prepare"); chain.set_matrix_target(Matrix2x2 { ll: 0.0, lr: 0.0, rl: 0.0, rr: 0.0 }).expect("target"); let mut left = [1.0, 1.0]; let mut right = [0.0, 0.0]; chain.process_matrix(DualMonoBlock { left: &mut left, right: &mut right, first_sample: 0 }); assert_eq!(left, [0.5, 0.0]); }
    #[test] fn meter_windows_are_exact() { let handle = MeterHandle(NonZeroU64::new(1).expect("constant")); let config = MeterConfig { period_frames: NonZeroU32::new(2).expect("constant"), peak_hold_frames: 0, peak_decay_db_per_second: 0.0, queue_capacity: NonZeroUsize::new(2).expect("constant"), reset_generation: 7 }; let PreparedMeter { mut accumulator, mut consumer } = MeterAccumulator::prepare(handle, config, 48_000).expect("meter"); accumulator.observe(&[1.0, 0.5], &[0.0, -1.0], 3); let snap = consumer.try_pop().expect("snapshot"); assert_eq!(snap.start_sample, 3); assert_eq!(snap.end_sample, 5); assert_eq!(snap.left.clipped_samples, 1); assert_eq!(snap.right.clipped_samples, 1); }
}
