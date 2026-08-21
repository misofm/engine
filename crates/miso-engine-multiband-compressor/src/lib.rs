//! Fixed two-band Linkwitz-Riley multiband compressor.
//!
//! Scalar and homogeneous banks own four independent conditioned TPT section histories per lane.
#![allow(missing_docs)]

use miso_engine_core::{
    CompressorGainMixKernelError, PreparedCompressorGainMixKernelV1, PreparedTptBankKernelV1,
    TptBankKernelError,
};
use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptorV1, EffectPrepareError, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LatencySamples, LinkMode, LinkModeSet, NativeEffectFactory,
    ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId,
    ParameterMapping, ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole,
    PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan, PreparedBankMetadata,
    PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport,
    ResetKind, SmoothingRule, StatePayloadError, StatePayloadInput, StatePayloadOutput,
    StatePayloadSizes, TailSamples, expected_prepared_metadata, sanitize_sample,
};

const PARAMETER_COUNT: usize = 12;
const RAMP_COUNT: usize = 10;
const STATE_HEADER_WORDS: usize = 43;
const LOW_BAND: usize = 0;
const HIGH_BAND: usize = 1;
const KNEE_DB: f32 = 6.0;

const fn effect_id(value: &'static str) -> miso_engine_effect_contract::EffectId {
    match miso_engine_effect_contract::EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static effect id"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static port id"),
    }
}

const fn parameter_id(value: u32) -> ParameterId {
    match ParameterId::new(value) {
        Some(value) => value,
        None => panic!("nonzero parameter id"),
    }
}

#[allow(clippy::too_many_arguments)]
const fn parameter(
    id: u32,
    display_name: &'static str,
    display_unit: &'static str,
    unit: ParameterUnit,
    minimum: f32,
    maximum: f32,
    default_value: f32,
    mapping: ParameterMapping,
    automation_rate: AutomationRate,
    smoothing: SmoothingRule,
    smoothing_samples: u32,
) -> ParameterDescriptorV1 {
    ParameterDescriptorV1 {
        id: parameter_id(id),
        display_name,
        display_unit,
        unit,
        domain: ParameterDomain::Continuous,
        minimum: Some(minimum),
        maximum: Some(maximum),
        default_value,
        mapping,
        automation_rate,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing,
        smoothing_samples,
        readable: true,
        automatable: !matches!(automation_rate, AutomationRate::None),
        enum_choices: &[],
    }
}

/// Frozen parameter order and stable numeric IDs for the V1 two-band product.
pub const MULTIBAND_COMPRESSOR_PARAMETERS_V1: [ParameterDescriptorV1; PARAMETER_COUNT] = [
    parameter(
        1,
        "crossover",
        "Hz",
        ParameterUnit::Hz,
        80.0,
        8_000.0,
        1_000.0,
        ParameterMapping::Logarithmic,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
    parameter(
        2,
        "lookahead",
        "ms",
        ParameterUnit::Milliseconds,
        0.0,
        20.0,
        5.0,
        ParameterMapping::Linear,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
    parameter(
        3,
        "low_threshold",
        "dB",
        ParameterUnit::Db,
        -80.0,
        0.0,
        -18.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        4,
        "low_ratio",
        "ratio",
        ParameterUnit::Ratio,
        1.0,
        20.0,
        4.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        5,
        "low_attack",
        "ms",
        ParameterUnit::Milliseconds,
        0.1,
        200.0,
        10.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        6,
        "low_release",
        "ms",
        ParameterUnit::Milliseconds,
        5.0,
        5_000.0,
        100.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        7,
        "low_makeup",
        "dB",
        ParameterUnit::Db,
        -24.0,
        24.0,
        0.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        8,
        "high_threshold",
        "dB",
        ParameterUnit::Db,
        -80.0,
        0.0,
        -18.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        9,
        "high_ratio",
        "ratio",
        ParameterUnit::Ratio,
        1.0,
        20.0,
        4.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        10,
        "high_attack",
        "ms",
        ParameterUnit::Milliseconds,
        0.1,
        200.0,
        10.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        11,
        "high_release",
        "ms",
        ParameterUnit::Milliseconds,
        5.0,
        5_000.0,
        100.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        12,
        "high_makeup",
        "dB",
        ParameterUnit::Db,
        -24.0,
        24.0,
        0.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
];

const PORTS: [PortDescriptorV1; 2] = [
    PortDescriptorV1 {
        id: port_id("main-in"),
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptorV1 {
        id: port_id("main-out"),
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
];

const fn lane_bytes(sample_rate: u32) -> u32 {
    let ring = sample_rate / 50 + 1;
    (STATE_HEADER_WORDS as u32 + 3 * ring) * 4
}

const fn quality(sample_rate: u32) -> miso_engine_effect_contract::QualityDescriptorV1 {
    let bytes = lane_bytes(sample_rate);
    miso_engine_effect_contract::QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples((sample_rate / 50) as u64),
        tail: TailSamples::Infinite,
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: bytes,
            right_bytes: bytes,
        },
        scratch_fixed_bytes: 136,
        scratch_bytes_per_frame: 0,
    }
}

const QUALITIES: [miso_engine_effect_contract::QualityDescriptorV1; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// Immutable descriptor for the launch two-band multiband compressor.
pub const MULTIBAND_COMPRESSOR_DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("miso.multiband-compressor"),
    display_name: "Multiband Compressor",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &MULTIBAND_COMPRESSOR_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
};

/// Factory for the fixed two-band scalar implementation.
#[derive(Clone, Copy, Debug, Default)]
pub struct MultibandCompressorFactory;

#[derive(Clone, Copy, Debug)]
struct Ramp {
    current: f32,
    target: f32,
    remaining: u32,
}

impl Ramp {
    const fn fixed(value: f32) -> Self {
        Self {
            current: value,
            target: value,
            remaining: 0,
        }
    }
    fn advance(&mut self) {
        if self.remaining != 0 {
            self.current += (self.target - self.current) / self.remaining as f32;
            self.remaining -= 1;
            if self.remaining == 0 {
                self.current = self.target;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Coefficients {
    c1: f32,
    a2: f32,
    a3: f32,
    k: f32,
}

impl Coefficients {
    fn design(sample_rate: u32, crossover_hz: f32) -> Option<Self> {
        if sample_rate == 0
            || !parameter_value_valid(&MULTIBAND_COMPRESSOR_PARAMETERS_V1[0], crossover_hz)
        {
            return None;
        }
        let g = (core::f64::consts::PI * f64::from(crossover_hz) / f64::from(sample_rate)).tan();
        let k64 = core::f64::consts::SQRT_2;
        let t1 = g * (g + k64);
        let den = 1.0 + t1;
        let values = [t1 / den, g / den, (g * g) / den, k64].map(|value| value as f32);
        if !values.into_iter().all(normal_or_zero) {
            return None;
        }
        let [c1, a2, a3, k] = values;
        let a00 = 1.0 - 2.0 * f64::from(c1);
        let a01 = -2.0 * f64::from(a2);
        let a10 = 2.0 * f64::from(a2);
        let a11 = 1.0 - 2.0 * f64::from(a3);
        let trace = a00 + a11;
        let determinant = a00 * a11 - a01 * a10;
        let a1 = -trace;
        if determinant.abs() >= 1.0
            || 1.0 + a1 + determinant <= 0.0
            || 1.0 - a1 + determinant <= 0.0
        {
            return None;
        }
        let coefficients = Self { c1, a2, a3, k };
        let low = coefficients.magnitude_db(sample_rate, crossover_hz, false)?;
        let high = coefficients.magnitude_db(sample_rate, crossover_hz, true)?;
        if (low + 3.010_299_956_6).abs() > 0.005 || (high + 3.010_299_956_6).abs() > 0.005 {
            return None;
        }
        Some(coefficients)
    }

    fn magnitude_db(self, sample_rate: u32, frequency: f32, high_pass: bool) -> Option<f64> {
        let (c1, a2, a3, k) = (
            f64::from(self.c1),
            f64::from(self.a2),
            f64::from(self.a3),
            f64::from(self.k),
        );
        let a00 = 1.0 - 2.0 * c1;
        let a01 = -2.0 * a2;
        let a10 = 2.0 * a2;
        let a11 = 1.0 - 2.0 * a3;
        let b0 = 2.0 * a2;
        let b1 = 2.0 * a3;
        let (o0, o1, direct) = if high_pass {
            (-k * (1.0 - c1) - a2, k * a2 - (1.0 - a3), 1.0 - k * a2 - a3)
        } else {
            (a2, 1.0 - a3, a3)
        };
        let phase = core::f64::consts::TAU * f64::from(frequency) / f64::from(sample_rate);
        let (zr, zi) = (phase.cos(), phase.sin());
        let m00r = zr - a00;
        let m11r = zr - a11;
        let determinant_r = m00r * m11r - zi * zi - (-a01) * (-a10);
        let determinant_i = zi * (m00r + m11r);
        let norm = determinant_r * determinant_r + determinant_i * determinant_i;
        if norm == 0.0 || !norm.is_finite() {
            return None;
        }
        let divide = |real: f64, imaginary: f64| {
            (
                (real * determinant_r + imaginary * determinant_i) / norm,
                (imaginary * determinant_r - real * determinant_i) / norm,
            )
        };
        let (s0r, s0i) = divide(m11r * b0 + a01 * b1, zi * b0);
        let (s1r, s1i) = divide(a10 * b0 + m00r * b1, zi * b1);
        let magnitude = (direct + o0 * s0r + o1 * s1r).hypot(o0 * s0i + o1 * s1i);
        (magnitude.is_finite() && magnitude > 0.0).then(|| 20.0 * magnitude.log10())
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct TptSection {
    s1: f32,
    s2: f32,
}

impl TptSection {
    fn process(&mut self, input: f32, coefficients: Coefficients) -> Option<(f32, f32)> {
        self.s1 = flushed(self.s1)?;
        self.s2 = flushed(self.s2)?;
        let v3 = input - self.s2;
        let p1 = coefficients.a2 * v3;
        let p2 = coefficients.c1 * self.s1;
        let d1 = p1 - p2;
        let v1 = self.s1 + d1;
        let p3 = coefficients.a2 * self.s1;
        let p4 = coefficients.a3 * v3;
        let d2 = p3 + p4;
        let v2 = self.s2 + d2;
        let n1 = flushed(self.s1 + (d1 + d1))?;
        let n2 = flushed(self.s2 + (d2 + d2))?;
        let low = flushed(v2)?;
        let high = flushed((input - coefficients.k * v1) - v2)?;
        self.s1 = n1;
        self.s2 = n2;
        Some((low, high))
    }
}

#[derive(Clone, Copy, Debug)]
struct Crossover {
    coefficients: Coefficients,
    sections: [TptSection; 4],
}

impl Crossover {
    fn new(sample_rate: u32, crossover_hz: f32) -> Option<Self> {
        Some(Self {
            coefficients: Coefficients::design(sample_rate, crossover_hz)?,
            sections: [TptSection::default(); 4],
        })
    }
    fn process(&mut self, input: f32) -> Option<(f32, f32)> {
        let (low_a, _) = self.sections[0].process(input, self.coefficients)?;
        let (low, _) = self.sections[1].process(low_a, self.coefficients)?;
        let (_, high_a) = self.sections[2].process(input, self.coefficients)?;
        let (_, high) = self.sections[3].process(high_a, self.coefficients)?;
        Some((low, high))
    }
    fn reset(&mut self) {
        self.sections = [TptSection::default(); 4];
    }
}

#[derive(Clone, Copy)]
struct BandFrame {
    dry: f32,
    low: f32,
    high: f32,
    detector_low: f32,
    detector_high: f32,
}

#[derive(Debug)]
struct Lane {
    cursor: u32,
    crossover_hz: f32,
    lookahead_ms: f32,
    detector_delay: usize,
    gains: [f32; 2],
    ramps: [Ramp; RAMP_COUNT],
    crossover: Crossover,
    dry_ring: Box<[f32]>,
    low_ring: Box<[f32]>,
    high_ring: Box<[f32]>,
}

impl Lane {
    fn new(defaults: &[f32; PARAMETER_COUNT], sample_rate: u32) -> Option<Self> {
        let latency = usize::try_from(sample_rate / 50).ok()?;
        let ring_length = latency.checked_add(1)?;
        let crossover = Crossover::new(sample_rate, defaults[0])?;
        let detector_delay = detector_delay(defaults[1], sample_rate, latency)?;
        Some(Self {
            cursor: 0,
            crossover_hz: defaults[0],
            lookahead_ms: defaults[1],
            detector_delay,
            gains: [0.0; 2],
            ramps: core::array::from_fn(|index| Ramp::fixed(defaults[index + 2])),
            crossover,
            dry_ring: vec![0.0; ring_length].into_boxed_slice(),
            low_ring: vec![0.0; ring_length].into_boxed_slice(),
            high_ring: vec![0.0; ring_length].into_boxed_slice(),
        })
    }
    fn full_reset(&mut self, defaults: &[f32; PARAMETER_COUNT], sample_rate: u32) {
        *self = Self::new(defaults, sample_rate).expect("validated prepared defaults");
    }
    fn discontinuity_reset(&mut self) {
        self.cursor = 0;
        self.gains = [0.0; 2];
        self.crossover.reset();
        self.dry_ring.fill(0.0);
        self.low_ring.fill(0.0);
        self.high_ring.fill(0.0);
        for ramp in &mut self.ramps {
            ramp.current = ramp.target;
            ramp.remaining = 0;
        }
    }
    fn crossover_frame(&mut self, input: f32) -> Option<BandFrame> {
        let (low, high) = self.crossover.process(input)?;
        Some(self.store_band_frame(input, low, high))
    }
    fn store_band_frame(&mut self, input: f32, low: f32, high: f32) -> BandFrame {
        let cursor = self.cursor as usize;
        let length = self.dry_ring.len();
        self.dry_ring[cursor] = input;
        self.low_ring[cursor] = low;
        self.high_ring[cursor] = high;
        let delayed = (cursor + 1) % length;
        let detector = (cursor + length - self.detector_delay) % length;
        self.cursor = ((cursor + 1) % length) as u32;
        BandFrame {
            dry: self.dry_ring[delayed],
            low: self.low_ring[delayed],
            high: self.high_ring[delayed],
            detector_low: self.low_ring[detector],
            detector_high: self.high_ring[detector],
        }
    }
    fn recover(&mut self, recovered: &mut u64) -> f32 {
        let delayed = self.dry_ring[(self.cursor as usize + 1) % self.dry_ring.len()];
        self.crossover.reset();
        self.gains = [0.0; 2];
        *recovered = recovered.saturating_add(1);
        delayed
    }
}

/// Prepared allocation-free scalar multiband compressor.
#[derive(Debug)]
pub struct PreparedMultibandCompressor {
    metadata: PreparedEffectMetadata,
    left_defaults: [f32; PARAMETER_COUNT],
    right_defaults: [f32; PARAMETER_COUNT],
    left: Lane,
    right: Lane,
}

struct PreparedMultibandCompressorBank<const W: usize> {
    metadata: PreparedBankMetadata,
    effect_metadata: PreparedEffectMetadata,
    tpt_kernel: PreparedTptBankKernelV1,
    gain_kernel: PreparedCompressorGainMixKernelV1,
    left_defaults: [[f32; PARAMETER_COUNT]; W],
    right_defaults: [[f32; PARAMETER_COUNT]; W],
    left: [Lane; W],
    right: [Lane; W],
}

impl NativeEffectFactory for MultibandCompressorFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        &MULTIBAND_COMPRESSOR_DESCRIPTOR_V1
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let (left_defaults, right_defaults) = initial_defaults(request.initial_values)?;
        let left = Lane::new(&left_defaults, metadata.sample_rate).ok_or(EffectPrepareError {
            code: "effect.prepare.failed",
        })?;
        let right = Lane::new(&right_defaults, metadata.sample_rate).ok_or(EffectPrepareError {
            code: "effect.prepare.failed",
        })?;
        Ok(Box::new(PreparedMultibandCompressor {
            metadata,
            left_defaults,
            right_defaults,
            left,
            right,
        }))
    }
    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        if !request.has_matching_backend_width()
            || request.requests.len() != request.width.lanes() as usize
        {
            return Err(EffectPrepareError {
                code: "effect.bank.requests",
            });
        }
        match request.width {
            BankWidth::Four => prepare_homogeneous_bank::<4>(self, request),
            BankWidth::Eight => prepare_homogeneous_bank::<8>(self, request),
        }
    }
}

fn prepare_homogeneous_bank<const W: usize>(
    factory: &MultibandCompressorFactory,
    request: PrepareEffectBankRequest<'_>,
) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
    let first = request
        .requests
        .first()
        .copied()
        .ok_or(EffectPrepareError {
            code: "effect.bank.requests",
        })?;
    let metadata = expected_prepared_metadata(factory.descriptor(), first)?;
    let (first_left, first_right) = initial_defaults(first.initial_values)?;
    let mut left_defaults = [first_left; W];
    let mut right_defaults = [first_right; W];
    let mut same_program = true;
    for (track, item) in request.requests.iter().copied().enumerate() {
        let candidate = expected_prepared_metadata(factory.descriptor(), item)?;
        if candidate.program_key() != metadata.program_key() {
            same_program = false;
        }
        let (left, right) = initial_defaults(item.initial_values)?;
        left_defaults[track] = left;
        right_defaults[track] = right;
    }
    if !same_program {
        return Ok(None);
    }
    let tpt_kernel = match PreparedTptBankKernelV1::try_new(request.backend) {
        Ok(kernel) => kernel,
        Err(TptBankKernelError::BackendUnavailable) => return Ok(None),
        Err(_) => {
            return Err(EffectPrepareError {
                code: "effect.bank.backend",
            });
        }
    };
    let gain_kernel = match PreparedCompressorGainMixKernelV1::try_new(request.backend) {
        Ok(kernel) => kernel,
        Err(CompressorGainMixKernelError::BackendUnavailable) => return Ok(None),
        Err(_) => {
            return Err(EffectPrepareError {
                code: "effect.bank.backend",
            });
        }
    };
    let left = core::array::from_fn(|track| {
        Lane::new(&left_defaults[track], metadata.sample_rate).expect("validated bank request")
    });
    let right = core::array::from_fn(|track| {
        Lane::new(&right_defaults[track], metadata.sample_rate).expect("validated bank request")
    });
    Ok(Some(Box::new(PreparedMultibandCompressorBank::<W> {
        metadata: PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
        effect_metadata: metadata,
        tpt_kernel,
        gain_kernel,
        left_defaults,
        right_defaults,
        left,
        right,
    })))
}

fn initial_defaults(
    values: &[InitialParameterValue],
) -> Result<([f32; PARAMETER_COUNT], [f32; PARAMETER_COUNT]), EffectPrepareError> {
    if values.len() != PARAMETER_COUNT * 2 {
        return Err(EffectPrepareError {
            code: "effect.parameter.initial",
        });
    }
    let mut left = [0.0; PARAMETER_COUNT];
    let mut right = [0.0; PARAMETER_COUNT];
    for (index, parameter) in MULTIBAND_COMPRESSOR_PARAMETERS_V1.iter().enumerate() {
        let left_value = values[index * 2];
        let right_value = values[index * 2 + 1];
        if left_value.parameter_index != index as u32
            || right_value.parameter_index != index as u32
            || left_value.channel != ParameterChannel::Left
            || right_value.channel != ParameterChannel::Right
            || !parameter_value_valid(parameter, left_value.value)
            || !parameter_value_valid(parameter, right_value.value)
            || negative_zero(left_value.value)
            || negative_zero(right_value.value)
        {
            return Err(EffectPrepareError {
                code: "effect.parameter.initial",
            });
        }
        left[index] = normalize_zero(left_value.value);
        right[index] = normalize_zero(right_value.value);
    }
    Ok((left, right))
}

fn parameter_value_valid(parameter: &ParameterDescriptorV1, value: f32) -> bool {
    value.is_finite()
        && parameter
            .minimum
            .zip(parameter.maximum)
            .is_some_and(|(minimum, maximum)| value >= minimum && value <= maximum)
}
fn negative_zero(value: f32) -> bool {
    value.to_bits() == (-0.0_f32).to_bits()
}
fn normalize_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}
fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && (value == 0.0 || value.is_normal())
}
fn flushed(value: f32) -> Option<f32> {
    if !value.is_finite() {
        None
    } else if value.is_subnormal() {
        Some(0.0)
    } else {
        Some(value)
    }
}

fn detector_delay(lookahead_ms: f32, sample_rate: u32, latency: usize) -> Option<usize> {
    if !parameter_value_valid(&MULTIBAND_COMPRESSOR_PARAMETERS_V1[1], lookahead_ms)
        || sample_rate == 0
    {
        return None;
    }
    let lookahead = (f64::from(lookahead_ms) * f64::from(sample_rate) / 1_000.0 + 0.5).floor();
    if !lookahead.is_finite() || lookahead < 0.0 || lookahead > usize::MAX as f64 {
        return None;
    }
    Some(latency - (lookahead as usize).min(latency))
}

fn linked_levels(link: LinkMode, left: f32, right: f32) -> (f32, f32) {
    let (left, right) = (left.abs(), right.abs());
    match link {
        LinkMode::DualMono => (left, right),
        LinkMode::Maximum => {
            let value = left.max(right);
            (value, value)
        }
        LinkMode::Average => {
            let value = 0.5 * left + 0.5 * right;
            (value, value)
        }
    }
}

fn gain_target(detector: f32, threshold: f32, ratio: f32) -> Option<f32> {
    let level = (20.0 * detector.max(1.0e-8).log10()).clamp(-160.0, 24.0);
    let half = 0.5 * KNEE_DB;
    let reciprocal_ratio = 1.0 / ratio;
    let output = if level < threshold - half {
        level
    } else if level > threshold + half {
        threshold + (level - threshold) * reciprocal_ratio
    } else {
        let v = level - threshold + half;
        level + (reciprocal_ratio - 1.0) * (v * v) / (2.0 * KNEE_DB)
    };
    flushed((output - level).clamp(-100.0, 0.0))
}

fn band_amplitude(lane: &mut Lane, band: usize, detector: f32, sample_rate: u32) -> Option<f32> {
    let base = if band == LOW_BAND { 0 } else { 5 };
    let target = gain_target(
        detector,
        lane.ramps[base].current,
        lane.ramps[base + 1].current,
    )?;
    let attack_ms = lane.ramps[base + 2].current;
    let release_ms = lane.ramps[base + 3].current;
    let attack = flushed((-1.0 / (0.001 * attack_ms * sample_rate as f32)).exp())?;
    let release = flushed((-1.0 / (0.001 * release_ms * sample_rate as f32)).exp())?;
    let coefficient = if target < lane.gains[band] {
        attack
    } else {
        release
    };
    let updated = flushed(coefficient * lane.gains[band] + (1.0 - coefficient) * target)?;
    lane.gains[band] = updated;
    flushed(10.0_f32.powf((updated + lane.ramps[base + 4].current) * 0.05))
}

fn active_output(
    frame: BandFrame,
    detector_low: f32,
    detector_high: f32,
    lane: &mut Lane,
    metadata: PreparedEffectMetadata,
) -> Option<f32> {
    let low =
        flushed(frame.low * band_amplitude(lane, LOW_BAND, detector_low, metadata.sample_rate)?)?;
    let high = flushed(
        frame.high * band_amplitude(lane, HIGH_BAND, detector_high, metadata.sample_rate)?,
    )?;
    let identity = lane.gains[LOW_BAND].to_bits() == 0
        && lane.gains[HIGH_BAND].to_bits() == 0
        && lane.ramps[4].current.to_bits() == 0
        && lane.ramps[9].current.to_bits() == 0;
    if metadata.bypass || identity {
        Some(frame.dry)
    } else {
        flushed(low + high)
    }
}

fn sanitize(value: f32, report: &mut ProcessReport) -> f32 {
    match sanitize_sample(value) {
        Some(value) => value,
        None => {
            report.sanitized_main_samples = report.sanitized_main_samples.saturating_add(1);
            0.0
        }
    }
}

impl PreparedNativeEffect for PreparedMultibandCompressor {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }
    fn reset(&mut self, kind: ResetKind) {
        match kind {
            ResetKind::FullToDefaults => {
                self.left
                    .full_reset(&self.left_defaults, self.metadata.sample_rate);
                self.right
                    .full_reset(&self.right_defaults, self.metadata.sample_rate);
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.discontinuity_reset();
                self.right.discontinuity_reset();
            }
        }
    }
    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        apply_automation(
            block.automation,
            self.metadata,
            block.first_sample,
            &mut self.left,
            &mut self.right,
            &mut report,
        );
        for index in 0..block.left.len() {
            for ramp in &mut self.left.ramps {
                ramp.advance();
            }
            for ramp in &mut self.right.ramps {
                ramp.advance();
            }
            let left_input = sanitize(block.left[index], &mut report);
            let right_input = sanitize(block.right[index], &mut report);
            let left = self.left.crossover_frame(left_input);
            let right = self.right.crossover_frame(right_input);
            match (left, right) {
                (Some(left), Some(right)) => {
                    let (left_low, right_low) = linked_levels(
                        self.metadata.link_mode,
                        left.detector_low,
                        right.detector_low,
                    );
                    let (left_high, right_high) = linked_levels(
                        self.metadata.link_mode,
                        left.detector_high,
                        right.detector_high,
                    );
                    block.left[index] =
                        active_output(left, left_low, left_high, &mut self.left, self.metadata)
                            .unwrap_or_else(|| {
                                self.left.recover(&mut report.recovered_left_samples)
                            });
                    block.right[index] =
                        active_output(right, right_low, right_high, &mut self.right, self.metadata)
                            .unwrap_or_else(|| {
                                self.right.recover(&mut report.recovered_right_samples)
                            });
                }
                (Some(left), None) => {
                    // A fault resets only its lane. The valid lane continues from its own
                    // detector when the other lane cannot contribute to a linked detector.
                    block.left[index] = active_output(
                        left,
                        left.detector_low.abs(),
                        left.detector_high.abs(),
                        &mut self.left,
                        self.metadata,
                    )
                    .unwrap_or_else(|| self.left.recover(&mut report.recovered_left_samples));
                    block.right[index] = self.right.recover(&mut report.recovered_right_samples);
                }
                (None, Some(right)) => {
                    block.left[index] = self.left.recover(&mut report.recovered_left_samples);
                    block.right[index] = active_output(
                        right,
                        right.detector_low.abs(),
                        right.detector_high.abs(),
                        &mut self.right,
                        self.metadata,
                    )
                    .unwrap_or_else(|| self.right.recover(&mut report.recovered_right_samples));
                }
                (None, None) => {
                    block.left[index] = self.left.recover(&mut report.recovered_left_samples);
                    block.right[index] = self.right.recover(&mut report.recovered_right_samples);
                }
            }
        }
        report
    }
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        validate_state_lengths(
            output.common.len(),
            output.left.len(),
            output.right.len(),
            self.metadata.state_sizes,
        )?;
        write_lane(output.left, &self.left);
        write_lane(output.right, &self.right);
        Ok(())
    }
    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if state_layout_version != 1 {
            return Err(state_error("effect.state.version"));
        }
        validate_state_lengths(
            input.common.len(),
            input.left.len(),
            input.right.len(),
            self.metadata.state_sizes,
        )?;
        let left = read_lane(input.left, self.metadata.sample_rate)?;
        let right = read_lane(input.right, self.metadata.sample_rate)?;
        self.left = left;
        self.right = right;
        Ok(())
    }
}

impl<const W: usize> PreparedNativeEffectBank for PreparedMultibandCompressorBank<W> {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }

    fn reset(&mut self, kind: ResetKind) {
        for track in 0..W {
            match kind {
                ResetKind::FullToDefaults => {
                    self.left[track]
                        .full_reset(&self.left_defaults[track], self.effect_metadata.sample_rate);
                    self.right[track].full_reset(
                        &self.right_defaults[track],
                        self.effect_metadata.sample_rate,
                    );
                }
                ResetKind::DiscontinuityKeepParameters => {
                    self.left[track].discontinuity_reset();
                    self.right[track].discontinuity_reset();
                }
            }
        }
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.metadata.width);
        let expected_samples = (block.frames as usize).checked_mul(W);
        let offsets_valid = block.automation_offsets.len() == W + 1
            && block.automation_offsets.first() == Some(&0)
            && block.automation_offsets.last().copied() == Some(block.automation.len() as u32)
            && block
                .automation_offsets
                .windows(2)
                .all(|pair| pair[0] <= pair[1]);
        if block.width != self.metadata.width
            || W != self.metadata.width.lanes() as usize
            || block.frames == 0
            || block.frames > self.effect_metadata.quantum
            || expected_samples != Some(block.left.len())
            || expected_samples != Some(block.right.len())
            || block.sidechain.is_some()
            || !offsets_valid
            || block
                .first_sample
                .checked_add(u64::from(block.frames))
                .is_none()
        {
            return report;
        }
        for track in 0..W {
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            apply_automation(
                &block.automation[start..end],
                self.effect_metadata,
                block.first_sample,
                &mut self.left[track],
                &mut self.right[track],
                &mut report.reports[track],
            );
        }
        for frame in 0..block.frames as usize {
            process_bank_frame(
                &mut self.left,
                &mut self.right,
                self.tpt_kernel,
                self.gain_kernel,
                self.effect_metadata,
                &mut report,
                &mut block.left[frame * W..(frame + 1) * W],
                &mut block.right[frame * W..(frame + 1) * W],
            );
        }
        report
    }

    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, W)?;
        validate_state_lengths(
            output.common.len(),
            output.left.len(),
            output.right.len(),
            self.effect_metadata.state_sizes,
        )?;
        write_lane(output.left, &self.left[track]);
        write_lane(output.right, &self.right[track]);
        Ok(())
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, W)?;
        if state_layout_version != 1 {
            return Err(state_error("effect.state.version"));
        }
        validate_state_lengths(
            input.common.len(),
            input.left.len(),
            input.right.len(),
            self.effect_metadata.state_sizes,
        )?;
        let left = read_lane(input.left, self.effect_metadata.sample_rate)?;
        let right = read_lane(input.right, self.effect_metadata.sample_rate)?;
        self.left[track] = left;
        self.right[track] = right;
        Ok(())
    }
}

fn checked_track(track_index: u32, width: usize) -> Result<usize, StatePayloadError> {
    let track = usize::try_from(track_index).map_err(|_| state_error("effect.state.track"))?;
    if track >= width {
        return Err(state_error("effect.state.track"));
    }
    Ok(track)
}

fn process_tpt_bank_section<const W: usize>(
    lanes: &mut [Lane; W],
    kernel: PreparedTptBankKernelV1,
    section: usize,
    high_pass: bool,
    samples: &mut [f32; W],
    failed: &mut [bool; W],
) -> bool {
    let mut c1 = [0.0; W];
    let mut a2 = [0.0; W];
    let mut a3 = [0.0; W];
    let mut k = [0.0; W];
    let mut s1 = [0.0; W];
    let mut s2 = [0.0; W];
    let mask = [if high_pass { u32::MAX } else { 0 }; W];
    for track in 0..W {
        let coefficients = lanes[track].crossover.coefficients;
        c1[track] = coefficients.c1;
        a2[track] = coefficients.a2;
        a3[track] = coefficients.a3;
        k[track] = coefficients.k;
        if failed[track] {
            samples[track] = 0.0;
            continue;
        }
        match (
            flushed(lanes[track].crossover.sections[section].s1),
            flushed(lanes[track].crossover.sections[section].s2),
        ) {
            (Some(left), Some(right)) => {
                s1[track] = left;
                s2[track] = right;
            }
            _ => failed[track] = true,
        }
    }
    if kernel
        .process_tpt(samples, &c1, &a2, &a3, &k, &mut s1, &mut s2, &mask)
        .is_err()
    {
        return false;
    }
    for track in 0..W {
        if failed[track] {
            continue;
        }
        match (
            flushed(samples[track]),
            flushed(s1[track]),
            flushed(s2[track]),
        ) {
            (Some(sample), Some(next_s1), Some(next_s2)) => {
                samples[track] = sample;
                lanes[track].crossover.sections[section].s1 = next_s1;
                lanes[track].crossover.sections[section].s2 = next_s2;
            }
            _ => failed[track] = true,
        }
    }
    true
}

fn process_crossover_bank<const W: usize>(
    lanes: &mut [Lane; W],
    kernel: PreparedTptBankKernelV1,
    input: [f32; W],
) -> Option<([f32; W], [f32; W], [bool; W])> {
    let mut failed = [false; W];
    let mut low = input;
    let mut high = input;
    if !process_tpt_bank_section(lanes, kernel, 0, false, &mut low, &mut failed)
        || !process_tpt_bank_section(lanes, kernel, 1, false, &mut low, &mut failed)
        || !process_tpt_bank_section(lanes, kernel, 2, true, &mut high, &mut failed)
        || !process_tpt_bank_section(lanes, kernel, 3, true, &mut high, &mut failed)
    {
        return None;
    }
    Some((low, high, failed))
}

#[allow(clippy::too_many_arguments)]
fn finish_bank_side<const W: usize>(
    lanes: &mut [Lane; W],
    frames: [Option<BandFrame>; W],
    detector_low: [f32; W],
    detector_high: [f32; W],
    kernel: PreparedCompressorGainMixKernelV1,
    metadata: PreparedEffectMetadata,
    reports: &mut [ProcessReport; 8],
    left_lane: bool,
    output: &mut [f32],
) {
    let mut low = [0.0; W];
    let mut high = [0.0; W];
    let mut low_gain = [1.0; W];
    let mut high_gain = [1.0; W];
    let mixes = [1.0; W];
    let dry_mask = [0_u32; W];
    let wet_mask = [u32::MAX; W];
    let mut dry = [0.0; W];
    let mut identity = [false; W];
    let mut recovered = [false; W];
    for track in 0..W {
        let counter = if left_lane {
            &mut reports[track].recovered_left_samples
        } else {
            &mut reports[track].recovered_right_samples
        };
        let Some(frame) = frames[track] else {
            output[track] = lanes[track].recover(counter);
            recovered[track] = true;
            continue;
        };
        dry[track] = frame.dry;
        low[track] = frame.low;
        high[track] = frame.high;
        match (
            band_amplitude(
                &mut lanes[track],
                LOW_BAND,
                detector_low[track],
                metadata.sample_rate,
            ),
            band_amplitude(
                &mut lanes[track],
                HIGH_BAND,
                detector_high[track],
                metadata.sample_rate,
            ),
        ) {
            (Some(low_value), Some(high_value)) => {
                low_gain[track] = low_value;
                high_gain[track] = high_value;
                identity[track] = lanes[track].gains[LOW_BAND].to_bits() == 0
                    && lanes[track].gains[HIGH_BAND].to_bits() == 0
                    && lanes[track].ramps[4].current.to_bits() == 0
                    && lanes[track].ramps[9].current.to_bits() == 0;
            }
            _ => {
                output[track] = lanes[track].recover(counter);
                recovered[track] = true;
            }
        }
    }
    let kernels_ok = kernel
        .process_gain_mix(&mut low, &low_gain, &mixes, &dry_mask, &wet_mask)
        .is_ok()
        && kernel
            .process_gain_mix(&mut high, &high_gain, &mixes, &dry_mask, &wet_mask)
            .is_ok();
    for track in 0..W {
        if recovered[track] {
            continue;
        }
        let counter = if left_lane {
            &mut reports[track].recovered_left_samples
        } else {
            &mut reports[track].recovered_right_samples
        };
        if !kernels_ok {
            output[track] = lanes[track].recover(counter);
        } else if metadata.bypass || identity[track] {
            output[track] = dry[track];
        } else if let Some(value) = flushed(low[track] + high[track]) {
            output[track] = value;
        } else {
            output[track] = lanes[track].recover(counter);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn process_bank_frame<const W: usize>(
    left_lanes: &mut [Lane; W],
    right_lanes: &mut [Lane; W],
    tpt_kernel: PreparedTptBankKernelV1,
    gain_kernel: PreparedCompressorGainMixKernelV1,
    metadata: PreparedEffectMetadata,
    report: &mut BankProcessReport,
    left_samples: &mut [f32],
    right_samples: &mut [f32],
) {
    let mut left_input = [0.0; W];
    let mut right_input = [0.0; W];
    for track in 0..W {
        for ramp in &mut left_lanes[track].ramps {
            ramp.advance();
        }
        for ramp in &mut right_lanes[track].ramps {
            ramp.advance();
        }
        left_input[track] = sanitize(left_samples[track], &mut report.reports[track]);
        right_input[track] = sanitize(right_samples[track], &mut report.reports[track]);
    }
    let Some((left_low, left_high, left_failed)) =
        process_crossover_bank(left_lanes, tpt_kernel, left_input)
    else {
        return;
    };
    let Some((right_low, right_high, right_failed)) =
        process_crossover_bank(right_lanes, tpt_kernel, right_input)
    else {
        return;
    };
    let mut left_frames = [None; W];
    let mut right_frames = [None; W];
    for track in 0..W {
        if left_failed[track] {
            left_lanes[track].crossover.reset();
            left_lanes[track].gains = [0.0; 2];
        } else {
            left_frames[track] = Some(left_lanes[track].store_band_frame(
                left_input[track],
                left_low[track],
                left_high[track],
            ));
        }
        if right_failed[track] {
            right_lanes[track].crossover.reset();
            right_lanes[track].gains = [0.0; 2];
        } else {
            right_frames[track] = Some(right_lanes[track].store_band_frame(
                right_input[track],
                right_low[track],
                right_high[track],
            ));
        }
    }
    let mut left_detector_low = [0.0; W];
    let mut right_detector_low = [0.0; W];
    let mut left_detector_high = [0.0; W];
    let mut right_detector_high = [0.0; W];
    for track in 0..W {
        match (left_frames[track], right_frames[track]) {
            (Some(left), Some(right)) => {
                (left_detector_low[track], right_detector_low[track]) =
                    linked_levels(metadata.link_mode, left.detector_low, right.detector_low);
                (left_detector_high[track], right_detector_high[track]) =
                    linked_levels(metadata.link_mode, left.detector_high, right.detector_high);
            }
            (Some(left), None) => {
                left_detector_low[track] = left.detector_low.abs();
                left_detector_high[track] = left.detector_high.abs();
            }
            (None, Some(right)) => {
                right_detector_low[track] = right.detector_low.abs();
                right_detector_high[track] = right.detector_high.abs();
            }
            (None, None) => {}
        }
    }
    finish_bank_side(
        left_lanes,
        left_frames,
        left_detector_low,
        left_detector_high,
        gain_kernel,
        metadata,
        &mut report.reports,
        true,
        left_samples,
    );
    finish_bank_side(
        right_lanes,
        right_frames,
        right_detector_low,
        right_detector_high,
        gain_kernel,
        metadata,
        &mut report.reports,
        false,
        right_samples,
    );
}

fn apply_automation(
    spans: &[PreparedAutomationSpan],
    metadata: PreparedEffectMetadata,
    first_sample: u64,
    left: &mut Lane,
    right: &mut Lane,
    report: &mut ProcessReport,
) {
    let mut pending = [[None; RAMP_COUNT]; 2];
    let mut prior = None;
    for (span_index, span) in spans.iter().enumerate() {
        let lane = match span.channel {
            ParameterChannel::Left => 0,
            ParameterChannel::Right => 1,
            ParameterChannel::Both => {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            }
        };
        let parameter = span.parameter_index as usize;
        let ramp = parameter.checked_sub(2);
        let order = span
            .parameter_index
            .checked_mul(2)
            .and_then(|value| value.checked_add(lane as u32));
        let valid = span_index < metadata.automation_capacity as usize
            && ramp.is_some_and(|value| value < RAMP_COUNT)
            && span.kind == AutomationSpanKind::Point
            && span.start_sample == first_sample
            && span.end_sample == first_sample
            && span.start_value.to_bits() == span.end_value.to_bits()
            && parameter < PARAMETER_COUNT
            && parameter_value_valid(
                &MULTIBAND_COMPRESSOR_PARAMETERS_V1[parameter],
                span.start_value,
            )
            && order.is_some_and(|value| prior.is_none_or(|previous| value > previous))
            && ramp.is_some_and(|value| pending[lane][value].is_none());
        let Some(ramp) = ramp.filter(|value| *value < RAMP_COUNT) else {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        };
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        prior = order;
        pending[lane][ramp] = Some(normalize_zero(span.start_value));
    }
    for (index, (left_pending, right_pending)) in
        pending[0].iter().zip(pending[1].iter()).enumerate()
    {
        if let Some(value) = *left_pending {
            left.ramps[index].target = value;
            left.ramps[index].remaining = 64;
        }
        if let Some(value) = *right_pending {
            right.ramps[index].target = value;
            right.ramps[index].remaining = 64;
        }
    }
}

fn validate_state_lengths(
    common: usize,
    left: usize,
    right: usize,
    sizes: StatePayloadSizes,
) -> Result<(), StatePayloadError> {
    if common != sizes.common_bytes as usize
        || left != sizes.left_bytes as usize
        || right != sizes.right_bytes as usize
    {
        Err(state_error("effect.state.length"))
    } else {
        Ok(())
    }
}
fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}
fn write_u32(bytes: &mut [u8], word: usize, value: u32) {
    bytes[word * 4..word * 4 + 4].copy_from_slice(&value.to_le_bytes());
}
fn write_f32(bytes: &mut [u8], word: usize, value: f32) {
    write_u32(bytes, word, value.to_bits());
}
fn read_u32(bytes: &[u8], word: usize) -> u32 {
    u32::from_le_bytes(
        bytes[word * 4..word * 4 + 4]
            .try_into()
            .expect("validated state bytes"),
    )
}
fn read_f32(bytes: &[u8], word: usize) -> f32 {
    f32::from_bits(read_u32(bytes, word))
}

fn write_lane(bytes: &mut [u8], lane: &Lane) {
    write_u32(bytes, 0, lane.cursor);
    write_f32(bytes, 1, lane.crossover_hz);
    write_f32(bytes, 2, lane.lookahead_ms);
    write_f32(bytes, 3, lane.gains[LOW_BAND]);
    write_f32(bytes, 4, lane.gains[HIGH_BAND]);
    for (index, ramp) in lane.ramps.iter().enumerate() {
        let word = 5 + index * 3;
        write_f32(bytes, word, ramp.current);
        write_f32(bytes, word + 1, ramp.target);
        write_u32(bytes, word + 2, ramp.remaining);
    }
    for (index, section) in lane.crossover.sections.iter().enumerate() {
        let word = 35 + index * 2;
        write_f32(bytes, word, section.s1);
        write_f32(bytes, word + 1, section.s2);
    }
    let length = lane.dry_ring.len();
    for (index, value) in lane.dry_ring.iter().enumerate() {
        write_f32(bytes, STATE_HEADER_WORDS + index, *value);
    }
    for (index, value) in lane.low_ring.iter().enumerate() {
        write_f32(bytes, STATE_HEADER_WORDS + length + index, *value);
    }
    for (index, value) in lane.high_ring.iter().enumerate() {
        write_f32(bytes, STATE_HEADER_WORDS + 2 * length + index, *value);
    }
}

fn read_lane(bytes: &[u8], sample_rate: u32) -> Result<Lane, StatePayloadError> {
    let latency =
        usize::try_from(sample_rate / 50).map_err(|_| state_error("effect.state.length"))?;
    let length = latency
        .checked_add(1)
        .ok_or(state_error("effect.state.length"))?;
    if bytes.len() != (STATE_HEADER_WORDS + 3 * length) * 4 {
        return Err(state_error("effect.state.length"));
    }
    let cursor = read_u32(bytes, 0);
    if cursor as usize >= length {
        return Err(state_error("effect.state.cursor"));
    }
    let crossover_hz = read_f32(bytes, 1);
    let lookahead_ms = read_f32(bytes, 2);
    if !parameter_state_valid(0, crossover_hz) || !parameter_state_valid(1, lookahead_ms) {
        return Err(state_error("effect.state.parameter"));
    }
    let gains = [read_f32(bytes, 3), read_f32(bytes, 4)];
    if gains.into_iter().any(|value| {
        !normal_or_zero(value) || negative_zero(value) || !(-100.0..=0.0).contains(&value)
    }) {
        return Err(state_error("effect.state.gain"));
    }
    let mut ramps = [Ramp::fixed(0.0); RAMP_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        let word = 5 + index * 3;
        let current = read_f32(bytes, word);
        let target = read_f32(bytes, word + 1);
        let remaining = read_u32(bytes, word + 2);
        if !parameter_state_valid(index + 2, current)
            || !parameter_state_valid(index + 2, target)
            || remaining > 64
        {
            return Err(state_error("effect.state.parameter"));
        }
        *ramp = Ramp {
            current,
            target,
            remaining,
        };
    }
    let mut defaults = [0.0; PARAMETER_COUNT];
    defaults[0] = crossover_hz;
    defaults[1] = lookahead_ms;
    for (index, ramp) in ramps.iter().enumerate() {
        defaults[index + 2] = ramp.current;
    }
    let mut lane =
        Lane::new(&defaults, sample_rate).ok_or(state_error("effect.state.coefficient"))?;
    lane.cursor = cursor;
    lane.gains = gains;
    lane.ramps = ramps;
    for (index, section) in lane.crossover.sections.iter_mut().enumerate() {
        let word = 35 + index * 2;
        section.s1 = read_f32(bytes, word);
        section.s2 = read_f32(bytes, word + 1);
        if !normal_or_zero(section.s1) || !normal_or_zero(section.s2) {
            return Err(state_error("effect.state.filter"));
        }
    }
    for (index, value) in lane.dry_ring.iter_mut().enumerate() {
        *value = read_f32(bytes, STATE_HEADER_WORDS + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.ring"));
        }
    }
    for (index, value) in lane.low_ring.iter_mut().enumerate() {
        *value = read_f32(bytes, STATE_HEADER_WORDS + length + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.ring"));
        }
    }
    for (index, value) in lane.high_ring.iter_mut().enumerate() {
        *value = read_f32(bytes, STATE_HEADER_WORDS + 2 * length + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.ring"));
        }
    }
    Ok(lane)
}

fn parameter_state_valid(index: usize, value: f32) -> bool {
    !negative_zero(value)
        && parameter_value_valid(&MULTIBAND_COMPRESSOR_PARAMETERS_V1[index], value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_dsp_reference::ReferenceLr4Crossover;
    use miso_engine_effect_contract::{
        EffectProcessBlock, PrepareEffectLimits, PreparedPortsV1, StatePayloadInput,
        StatePayloadOutput, validate_descriptor_v1,
    };

    fn values() -> [InitialParameterValue; PARAMETER_COUNT * 2] {
        core::array::from_fn(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: MULTIBAND_COMPRESSOR_PARAMETERS_V1[index / 2].default_value,
        })
    }
    fn request<'a>(values: &'a [InitialParameterValue]) -> PrepareEffectRequest<'a> {
        PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: 128,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPortsV1 {
                sidechain: miso_engine_effect_contract::PreparedSidechainPort::None,
            },
            initial_values: values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: u64::MAX,
                maximum_scratch_bytes: u64::MAX,
                maximum_automation_spans_per_block: 32,
            },
        }
    }
    fn rms(values: &[f32]) -> f64 {
        (values
            .iter()
            .map(|value| f64::from(*value) * f64::from(*value))
            .sum::<f64>()
            / values.len() as f64)
            .sqrt()
    }

    #[test]
    fn descriptor_preparation_and_exact_four_rate_resources_are_frozen() {
        validate_descriptor_v1(&MULTIBAND_COMPRESSOR_DESCRIPTOR_V1).expect("descriptor");
        assert_eq!(MULTIBAND_COMPRESSOR_PARAMETERS_V1.len(), 12);
        for (rate, bytes) in [
            (44_100, 10_768),
            (48_000, 11_704),
            (88_200, 21_352),
            (96_000, 23_224),
        ] {
            let initial = values();
            let mut prepared = request(&initial);
            prepared.sample_rate = rate;
            let effect = MultibandCompressorFactory
                .prepare(prepared)
                .expect("prepare");
            assert_eq!(
                effect.metadata().latency,
                LatencySamples((rate / 50) as u64)
            );
            assert_eq!(effect.metadata().state_sizes.left_bytes, bytes);
            assert_eq!(effect.metadata().state_sizes.right_bytes, bytes);
            assert_eq!(effect.metadata().scratch_bytes, 136);
            let mut below = request(&initial);
            below.sample_rate = rate;
            below.limits.maximum_total_state_bytes = u64::from(bytes * 2 - 1);
            assert_eq!(
                MultibandCompressorFactory.prepare(below).err(),
                Some(EffectPrepareError {
                    code: "effect.resource.limit"
                })
            );
        }
    }

    #[test]
    fn conditioned_lr4_matches_independent_reference_and_recombines_flat() {
        for rate in [44_100_u32, 48_000, 88_200, 96_000] {
            for cutoff in [80.0_f32, 1_000.0, 8_000.0] {
                let mut production = Crossover::new(rate, cutoff).expect("production crossover");
                let mut reference = ReferenceLr4Crossover::new(f64::from(rate), f64::from(cutoff))
                    .expect("reference crossover");
                let mut input = Vec::new();
                let mut low = Vec::new();
                let mut high = Vec::new();
                let mut sum = Vec::new();
                for index in 0..8_192 {
                    let sample =
                        (core::f32::consts::TAU * cutoff * index as f32 / rate as f32).sin();
                    let (actual_low, actual_high) =
                        production.process(sample).expect("finite crossover");
                    let (expected_low, expected_high) = reference.process_sample(f64::from(sample));
                    assert!((actual_low - expected_low as f32).abs() < 2.0e-5);
                    assert!((actual_high - expected_high as f32).abs() < 2.0e-5);
                    if index >= 4_096 {
                        input.push(sample);
                        low.push(actual_low);
                        high.push(actual_high);
                        sum.push(actual_low + actual_high);
                    }
                }
                let crossing = 20.0 * (rms(&low) / rms(&input)).log10();
                assert!(
                    (crossing + 6.020_599_913).abs() <= 0.02,
                    "rate={rate} cutoff={cutoff} crossing={crossing}"
                );
                let all_pass = 20.0 * (rms(&sum) / rms(&input)).log10();
                assert!(
                    all_pass.abs() <= 0.05,
                    "rate={rate} cutoff={cutoff} sum={all_pass}"
                );
            }
        }
    }

    #[test]
    fn isolated_low_and_high_band_compression_reduce_only_the_selected_band() {
        for (frequency, active_base) in [(120.0_f32, 0_usize), (4_000.0_f32, 5_usize)] {
            let mut active_values = values();
            let mut identity_values = values();
            for lane in 0..2 {
                active_values[(active_base + 2) * 2 + lane].value = -45.0;
                active_values[(active_base + 3) * 2 + lane].value = 20.0;
                active_values[(active_base + 4) * 2 + lane].value = 0.1;
                active_values[(active_base + 5) * 2 + lane].value = 5.0;
                identity_values[(active_base + 3) * 2 + lane].value = 1.0;
            }
            let mut active = MultibandCompressorFactory
                .prepare(request(&active_values))
                .expect("active");
            let mut identity = MultibandCompressorFactory
                .prepare(request(&identity_values))
                .expect("identity");
            let mut active_pcm = (0..3_072)
                .map(|index| {
                    0.8 * (core::f32::consts::TAU * frequency * index as f32 / 48_000.0).sin()
                })
                .collect::<Vec<_>>();
            let mut identity_pcm = active_pcm.clone();
            let mut right_active = active_pcm.clone();
            let mut right_identity = identity_pcm.clone();
            for block in 0..24 {
                let start = block * 128;
                active.process(
                    EffectProcessBlock::new(
                        &mut active_pcm[start..start + 128],
                        &mut right_active[start..start + 128],
                        None,
                        start as u64,
                        &[],
                        128,
                    )
                    .expect("active block"),
                );
                identity.process(
                    EffectProcessBlock::new(
                        &mut identity_pcm[start..start + 128],
                        &mut right_identity[start..start + 128],
                        None,
                        start as u64,
                        &[],
                        128,
                    )
                    .expect("identity block"),
                );
            }
            assert!(
                rms(&active_pcm[1_600..]) < rms(&identity_pcm[1_600..]) * 0.9,
                "frequency={frequency}"
            );
        }
    }

    #[test]
    fn bypass_latency_automation_and_restore_are_transactional() {
        let initial = values();
        let mut request = request(&initial);
        request.bypass = true;
        let mut effect = MultibandCompressorFactory.prepare(request).expect("bypass");
        let sizes = effect.metadata().state_sizes;
        let mut left = vec![0.0; 128];
        let mut right = vec![0.0; 128];
        left[0] = -0.5;
        right[0] = 0.25;
        let span = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 2,
            start_sample: 0,
            end_sample: 0,
            start_value: -80.0,
            end_value: -80.0,
        };
        let mut output = Vec::new();
        for block in 0..8 {
            let spans = if block == 0 {
                core::slice::from_ref(&span)
            } else {
                &[]
            };
            effect.process(
                EffectProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    (block * 128) as u64,
                    spans,
                    128,
                )
                .expect("block"),
            );
            output.extend_from_slice(&left);
            left.fill(0.0);
            right.fill(0.0);
        }
        assert!(output[..960].iter().all(|sample| *sample == 0.0));
        assert_eq!(output[960].to_bits(), (-0.5_f32).to_bits());
        let mut saved_left = vec![0_u8; sizes.left_bytes as usize];
        let mut saved_right = vec![0_u8; sizes.right_bytes as usize];
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut saved_left, &mut saved_right, sizes)
                    .expect("output"),
            )
            .expect("snapshot");
        let mut malformed = saved_right.clone();
        malformed[..4].fill(u8::MAX);
        assert!(
            effect
                .restore_state_payload(
                    1,
                    StatePayloadInput::new(&[], &saved_left, &malformed, sizes).expect("input")
                )
                .is_err()
        );
        let mut after_left = vec![0_u8; sizes.left_bytes as usize];
        let mut after_right = vec![0_u8; sizes.right_bytes as usize];
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut after_left, &mut after_right, sizes)
                    .expect("output"),
            )
            .expect("snapshot");
        assert_eq!(after_left, saved_left);
        assert_eq!(after_right, saved_right);
    }
}
