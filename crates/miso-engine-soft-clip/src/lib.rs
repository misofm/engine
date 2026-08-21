//! Fixed-two-times cubic soft clipper with a private scalar oversampling lane.
#![allow(missing_docs)]

use miso_engine_core::{PreparedSoftClipBankKernelV1, SoftClipBankKernelError};
use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptorV1, EffectPrepareError, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LatencySamples, LinkModeSet, NativeEffectFactory, ParameterChannel,
    ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId, ParameterMapping,
    ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole, PrepareEffectBankRequest,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedBankMetadata, PreparedEffectMetadata,
    PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport, ResetKind, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata, sanitize_sample,
};

const PARAMETER_COUNT: usize = 3;
const STATE_WORDS: usize = 169;
const LANE_STATE_BYTES: u32 = (STATE_WORDS * 4) as u32;
const HISTORY: usize = 63;
const DRY_HISTORY: usize = 32;
const RAMP_SAMPLES: u32 = 64;
const TAPS: [usize; 31] = [
    2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 31, 32, 34, 36, 38, 40, 42, 44, 46, 48,
    50, 52, 54, 56, 58, 60,
];

const fn effect_id(value: &'static str) -> miso_engine_effect_contract::EffectId {
    match miso_engine_effect_contract::EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static effect identifier"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static port identifier"),
    }
}

const fn parameter_id(value: u32) -> ParameterId {
    match ParameterId::new(value) {
        Some(value) => value,
        None => panic!("nonzero static parameter identifier"),
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
        mapping: ParameterMapping::Linear,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: SmoothingRule::Linear,
        smoothing_samples: RAMP_SAMPLES,
        readable: true,
        automatable: true,
        enum_choices: &[],
    }
}

/// Frozen scalar soft-clip parameter rows, in stable numeric-ID order.
pub const SOFT_CLIP_PARAMETERS_V1: [ParameterDescriptorV1; PARAMETER_COUNT] = [
    parameter(1, "drive", "dB", ParameterUnit::Db, -24.0, 36.0, 0.0),
    parameter(2, "output", "dB", ParameterUnit::Db, -24.0, 24.0, 0.0),
    parameter(3, "mix", "linear", ParameterUnit::Linear, 0.0, 1.0, 1.0),
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

const fn quality(rate: u32) -> miso_engine_effect_contract::QualityDescriptorV1 {
    miso_engine_effect_contract::QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate: rate,
        latency: LatencySamples(31),
        tail: TailSamples::Finite(29),
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: LANE_STATE_BYTES,
            right_bytes: LANE_STATE_BYTES,
        },
        scratch_fixed_bytes: 24,
        scratch_bytes_per_frame: 0,
    }
}

const QUALITIES: [miso_engine_effect_contract::QualityDescriptorV1; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// Immutable descriptor for the frozen cubic soft-clip contract.
pub const SOFT_CLIP_DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("miso.soft-clip"),
    display_name: "Cubic Soft Clip",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &SOFT_CLIP_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
};

/// Factory for the fixed-latency scalar launch realization.
#[derive(Clone, Copy, Debug, Default)]
pub struct SoftClipFactory;

/// The frozen symmetric 63-tap f32 halfband table, held in direct index order.
const H: [f32; HISTORY] = [
    0.0,
    0.0,
    4.117_896_6e-5,
    0.0,
    -1.843_658_7e-4,
    0.0,
    4.762_265_3e-4,
    0.0,
    -9.890_399e-4,
    0.0,
    1.823_257_9e-3,
    0.0,
    -3.110_171_5e-3,
    0.0,
    5.017_224_7e-3,
    0.0,
    -7.761_148e-3,
    0.0,
    1.163_983_6e-2,
    0.0,
    -1.710_855_8e-2,
    0.0,
    2.496_969_9e-2,
    0.0,
    -3.690_095e-2,
    0.0,
    5.726_340_8e-2,
    0.0,
    -1.021_490_2e-1,
    0.0,
    3.169_724_3e-1,
    5.0e-1,
    3.169_724_3e-1,
    0.0,
    -1.021_490_2e-1,
    0.0,
    5.726_340_8e-2,
    0.0,
    -3.690_095e-2,
    0.0,
    2.496_969_9e-2,
    0.0,
    -1.710_855_8e-2,
    0.0,
    1.163_983_6e-2,
    0.0,
    -7.761_148e-3,
    0.0,
    5.017_224_7e-3,
    0.0,
    -3.110_171_5e-3,
    0.0,
    1.823_257_9e-3,
    0.0,
    -9.890_399e-4,
    0.0,
    4.762_265_3e-4,
    0.0,
    -1.843_658_7e-4,
    0.0,
    4.117_896_6e-5,
    0.0,
    0.0,
];

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

    fn advance(&mut self) -> Option<()> {
        if self.remaining != 0 {
            if self.remaining == 1 {
                self.current = self.target;
            } else {
                let delta = checked(self.target - self.current)?;
                let step = checked(delta / self.remaining as f32)?;
                self.current = checked(self.current + step)?;
            }
            self.remaining -= 1;
        }
        normal_or_zero(self.current).then_some(())
    }

    fn snap_to_target(&mut self) {
        self.current = self.target;
        self.remaining = 0;
    }
}

#[derive(Clone, Debug)]
struct Lane {
    high_cursor: u32,
    dry_cursor: u32,
    ramps: [Ramp; PARAMETER_COUNT],
    interp: [f32; HISTORY],
    decim: [f32; HISTORY],
    dry: [f32; DRY_HISTORY],
}

impl Lane {
    fn new(defaults: [f32; PARAMETER_COUNT]) -> Option<Self> {
        if !state_parameter_valid(0, defaults[0])
            || !state_parameter_valid(1, defaults[1])
            || !state_parameter_valid(2, defaults[2])
        {
            return None;
        }
        Some(Self {
            high_cursor: 0,
            dry_cursor: 0,
            ramps: defaults.map(Ramp::fixed),
            interp: [0.0; HISTORY],
            decim: [0.0; HISTORY],
            dry: [0.0; DRY_HISTORY],
        })
    }

    fn clear_histories(&mut self) {
        self.high_cursor = 0;
        self.dry_cursor = 0;
        self.interp.fill(0.0);
        self.decim.fill(0.0);
        self.dry.fill(0.0);
    }

    fn full_reset(&mut self, defaults: [f32; PARAMETER_COUNT]) {
        self.clear_histories();
        self.ramps = defaults.map(Ramp::fixed);
    }

    fn discontinuity_reset(&mut self) {
        self.clear_histories();
        self.ramps.iter_mut().for_each(Ramp::snap_to_target);
    }

    fn process(&mut self, input: f32, bypass: bool) -> Result<f32, f32> {
        let recovery_dry = delayed_dry(self);
        for ramp in &mut self.ramps {
            if ramp.advance().is_none() {
                return Err(recovery_dry);
            }
        }
        let dry_index = self.dry_cursor as usize;
        self.dry[dry_index] = input;
        let delayed = self.dry[(dry_index + 1) % DRY_HISTORY];
        self.dry_cursor = ((dry_index + 1) % DRY_HISTORY) as u32;
        let doubled = checked(2.0_f32 * self.ramps[0].current).ok_or(recovery_dry)?;
        let first = checked(doubled * input).ok_or(recovery_dry)?;
        let wet = self.stage(first).map_err(|()| recovery_dry)?;
        let _discarded = self.stage(0.0).map_err(|()| recovery_dry)?;
        let mix = self.ramps[2].current;
        let output = self.ramps[1].current;
        if bypass || (mix.to_bits() == 0.0_f32.to_bits() && output.to_bits() == 1.0_f32.to_bits()) {
            return Ok(delayed);
        }
        let a = checked(1.0_f32 - mix).ok_or(recovery_dry)?;
        let b = checked(a * delayed).ok_or(recovery_dry)?;
        let c = checked(mix * wet).ok_or(recovery_dry)?;
        let e = checked(b + c).ok_or(recovery_dry)?;
        checked(output * e).ok_or(recovery_dry)
    }

    fn stage(&mut self, input: f32) -> Result<f32, ()> {
        let cursor = self.high_cursor as usize;
        self.interp[cursor] = checked(input).ok_or(())?;
        let interpolated = convolve(&self.interp, cursor).ok_or(())?;
        let shaped = cubic(interpolated).ok_or(())?;
        self.decim[cursor] = shaped;
        let output = convolve(&self.decim, cursor).ok_or(())?;
        self.high_cursor = ((cursor + 1) % HISTORY) as u32;
        Ok(output)
    }

    fn recover(&mut self) {
        self.clear_histories();
        self.ramps.iter_mut().for_each(Ramp::snap_to_target);
    }
}

/// A prepared scalar soft-clip dual-mono instance.
#[derive(Debug)]
pub struct PreparedSoftClip {
    metadata: PreparedEffectMetadata,
    left_defaults: [f32; PARAMETER_COUNT],
    right_defaults: [f32; PARAMETER_COUNT],
    left: Lane,
    right: Lane,
}

/// Width-specialized effect-owned bytes: state plus the six retained f32 defaults per track.
fn bank_effect_bytes(width: usize) -> Option<u64> {
    let state = (LANE_STATE_BYTES as u64).checked_mul(2)?;
    let per_track = state.checked_add(24)?;
    (width as u64).checked_mul(per_track)
}

/// Sample-major state for one audio channel of a homogeneous soft-clip cohort.
#[derive(Clone, Debug)]
struct BankLane<const W: usize> {
    high_cursor: [u32; W],
    dry_cursor: [u32; W],
    ramps: [[Ramp; PARAMETER_COUNT]; W],
    interpolation: [[f32; W]; HISTORY],
    decimation: [[f32; W]; HISTORY],
    dry: [[f32; W]; DRY_HISTORY],
}

impl<const W: usize> BankLane<W> {
    fn new(defaults: &[[f32; PARAMETER_COUNT]; W]) -> Self {
        Self {
            high_cursor: [0; W],
            dry_cursor: [0; W],
            ramps: (*defaults).map(|values| values.map(Ramp::fixed)),
            interpolation: [[0.0; W]; HISTORY],
            decimation: [[0.0; W]; HISTORY],
            dry: [[0.0; W]; DRY_HISTORY],
        }
    }

    fn lane(&self, track: usize) -> Lane {
        Lane {
            high_cursor: self.high_cursor[track],
            dry_cursor: self.dry_cursor[track],
            ramps: self.ramps[track],
            interp: core::array::from_fn(|word| self.interpolation[word][track]),
            decim: core::array::from_fn(|word| self.decimation[word][track]),
            dry: core::array::from_fn(|word| self.dry[word][track]),
        }
    }

    fn set_lane(&mut self, track: usize, lane: Lane) {
        self.high_cursor[track] = lane.high_cursor;
        self.dry_cursor[track] = lane.dry_cursor;
        self.ramps[track] = lane.ramps;
        for word in 0..HISTORY {
            self.interpolation[word][track] = lane.interp[word];
            self.decimation[word][track] = lane.decim[word];
        }
        for word in 0..DRY_HISTORY {
            self.dry[word][track] = lane.dry[word];
        }
    }

    fn reset_full(&mut self, track: usize, defaults: [f32; PARAMETER_COUNT]) {
        self.set_lane(
            track,
            Lane::new(defaults).expect("prepared bank defaults remain state-valid"),
        );
    }

    fn reset_discontinuity(&mut self, track: usize) {
        let mut lane = self.lane(track);
        lane.discontinuity_reset();
        self.set_lane(track, lane);
    }

    fn recover(&mut self, track: usize) {
        let mut lane = self.lane(track);
        lane.recover();
        self.set_lane(track, lane);
    }

    fn delayed_dry(&self, track: usize) -> f32 {
        self.dry[(self.dry_cursor[track] as usize + 1) % DRY_HISTORY][track]
    }
}

/// A fixed-width homogeneous cohort with independent dual-mono tracks and AoSoA FIR histories.
struct PreparedSoftClipBank<const W: usize> {
    metadata: PreparedBankMetadata,
    effect_metadata: PreparedEffectMetadata,
    kernel: PreparedSoftClipBankKernelV1,
    left_defaults: [[f32; PARAMETER_COUNT]; W],
    right_defaults: [[f32; PARAMETER_COUNT]; W],
    left: BankLane<W>,
    right: BankLane<W>,
}

impl NativeEffectFactory for SoftClipFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        &SOFT_CLIP_DESCRIPTOR_V1
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let (left_defaults, right_defaults) = initial_defaults(request.initial_values)?;
        let left = Lane::new(left_defaults).ok_or(EffectPrepareError {
            code: "effect.parameter.initial",
        })?;
        let right = Lane::new(right_defaults).ok_or(EffectPrepareError {
            code: "effect.parameter.initial",
        })?;
        Ok(Box::new(PreparedSoftClip {
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
    factory: &SoftClipFactory,
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
    for (track, member) in request.requests.iter().copied().enumerate() {
        let candidate = expected_prepared_metadata(factory.descriptor(), member)?;
        if candidate.program_key() != metadata.program_key() {
            same_program = false;
        }
        let (left, right) = initial_defaults(member.initial_values)?;
        left_defaults[track] = left;
        right_defaults[track] = right;
    }
    if !same_program {
        return Ok(None);
    }
    if bank_effect_bytes(W) != Some(W as u64 * 1_376) {
        return Err(EffectPrepareError {
            code: "effect.resource.limit",
        });
    }
    let kernel = match PreparedSoftClipBankKernelV1::try_new(request.backend) {
        Ok(kernel) => kernel,
        Err(SoftClipBankKernelError::BackendUnavailable) => return Ok(None),
        Err(_) => {
            return Err(EffectPrepareError {
                code: "effect.bank.backend",
            });
        }
    };
    Ok(Some(Box::new(PreparedSoftClipBank::<W> {
        metadata: PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
        effect_metadata: metadata,
        kernel,
        left_defaults,
        right_defaults,
        left: BankLane::new(&left_defaults),
        right: BankLane::new(&right_defaults),
    })))
}

impl PreparedNativeEffect for PreparedSoftClip {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }

    fn reset(&mut self, kind: ResetKind) {
        match kind {
            ResetKind::FullToDefaults => {
                self.left.full_reset(self.left_defaults);
                self.right.full_reset(self.right_defaults);
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
            &mut self.left.ramps,
            &mut self.right.ramps,
            &mut report,
        );
        for index in 0..block.frames() {
            let left_input = sanitize(block.left[index], &mut report.sanitized_main_samples);
            let right_input = sanitize(block.right[index], &mut report.sanitized_main_samples);
            block.left[index] = match self.left.process(left_input, self.metadata.bypass) {
                Ok(value) => value,
                Err(delayed) => {
                    self.left.recover();
                    report.recovered_left_samples = report.recovered_left_samples.saturating_add(1);
                    delayed
                }
            };
            block.right[index] = match self.right.process(right_input, self.metadata.bypass) {
                Ok(value) => value,
                Err(delayed) => {
                    self.right.recover();
                    report.recovered_right_samples =
                        report.recovered_right_samples.saturating_add(1);
                    delayed
                }
            };
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
        let left = read_lane(input.left)?;
        let right = read_lane(input.right)?;
        self.left = left;
        self.right = right;
        Ok(())
    }
}

impl<const W: usize> PreparedNativeEffectBank for PreparedSoftClipBank<W> {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }

    fn reset(&mut self, kind: ResetKind) {
        for track in 0..W {
            match kind {
                ResetKind::FullToDefaults => {
                    self.left.reset_full(track, self.left_defaults[track]);
                    self.right.reset_full(track, self.right_defaults[track]);
                }
                ResetKind::DiscontinuityKeepParameters => {
                    self.left.reset_discontinuity(track);
                    self.right.reset_discontinuity(track);
                }
            }
        }
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.metadata.width);
        if !bank_block_matches(&block, self.metadata.width, self.effect_metadata.quantum)
            || W != self.metadata.width.lanes() as usize
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
                &mut self.left.ramps[track],
                &mut self.right.ramps[track],
                &mut report.reports[track],
            );
        }
        for frame in 0..block.frames as usize {
            let start = frame * W;
            let mut left_input = [0.0_f32; W];
            let mut right_input = [0.0_f32; W];
            for track in 0..W {
                left_input[track] = sanitize(
                    block.left[start + track],
                    &mut report.reports[track].sanitized_main_samples,
                );
                right_input[track] = sanitize(
                    block.right[start + track],
                    &mut report.reports[track].sanitized_main_samples,
                );
            }
            let (left_output, left_recovered) = process_bank_channel(
                &mut self.left,
                left_input,
                self.effect_metadata.bypass,
                self.kernel,
            );
            let (right_output, right_recovered) = process_bank_channel(
                &mut self.right,
                right_input,
                self.effect_metadata.bypass,
                self.kernel,
            );
            for track in 0..W {
                block.left[start + track] = left_output[track];
                block.right[start + track] = right_output[track];
                if left_recovered[track] {
                    report.reports[track].recovered_left_samples = report.reports[track]
                        .recovered_left_samples
                        .saturating_add(1);
                }
                if right_recovered[track] {
                    report.reports[track].recovered_right_samples = report.reports[track]
                        .recovered_right_samples
                        .saturating_add(1);
                }
            }
        }
        report
    }

    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = bank_track_index::<W>(track_index)?;
        validate_state_lengths(
            output.common.len(),
            output.left.len(),
            output.right.len(),
            self.effect_metadata.state_sizes,
        )?;
        write_lane(output.left, &self.left.lane(track));
        write_lane(output.right, &self.right.lane(track));
        Ok(())
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = bank_track_index::<W>(track_index)?;
        if state_layout_version != 1 {
            return Err(state_error("effect.state.version"));
        }
        validate_state_lengths(
            input.common.len(),
            input.left.len(),
            input.right.len(),
            self.effect_metadata.state_sizes,
        )?;
        let left = read_lane(input.left)?;
        let right = read_lane(input.right)?;
        self.left.set_lane(track, left);
        self.right.set_lane(track, right);
        Ok(())
    }
}

fn bank_track_index<const W: usize>(track_index: u32) -> Result<usize, StatePayloadError> {
    let track = usize::try_from(track_index).map_err(|_| state_error("effect.bank.track"))?;
    if track >= W {
        return Err(state_error("effect.bank.track"));
    }
    Ok(track)
}

fn bank_block_matches(block: &EffectBankProcessBlock<'_>, width: BankWidth, quantum: u32) -> bool {
    let lanes = width.lanes() as usize;
    let Some(length) = (block.frames as usize).checked_mul(lanes) else {
        return false;
    };
    block.width == width
        && block.frames != 0
        && block.frames <= quantum
        && block.left.len() == length
        && block.right.len() == length
        && block.sidechain.is_none()
        && block
            .first_sample
            .checked_add(block.frames as u64)
            .is_some()
        && block.automation_offsets.len() == lanes + 1
        && block.automation_offsets.first() == Some(&0)
        && block.automation_offsets.last().copied() == Some(block.automation.len() as u32)
        && !block
            .automation_offsets
            .windows(2)
            .any(|pair| pair[0] > pair[1])
}

fn process_bank_channel<const W: usize>(
    lane: &mut BankLane<W>,
    input: [f32; W],
    bypass: bool,
    kernel: PreparedSoftClipBankKernelV1,
) -> ([f32; W], [bool; W]) {
    let mut delayed = [0.0_f32; W];
    let mut first_phase = [0.0_f32; W];
    let mut failed = [false; W];
    for track in 0..W {
        delayed[track] = lane.delayed_dry(track);
        for ramp in &mut lane.ramps[track] {
            if ramp.advance().is_none() {
                failed[track] = true;
                break;
            }
        }
        if failed[track] {
            continue;
        }
        let dry_index = lane.dry_cursor[track] as usize;
        lane.dry[dry_index][track] = input[track];
        delayed[track] = lane.dry[(dry_index + 1) % DRY_HISTORY][track];
        lane.dry_cursor[track] = ((dry_index + 1) % DRY_HISTORY) as u32;
        let Some(doubled) = checked(2.0_f32 * lane.ramps[track][0].current) else {
            failed[track] = true;
            continue;
        };
        let Some(value) = checked(doubled * input[track]) else {
            failed[track] = true;
            continue;
        };
        first_phase[track] = value;
    }
    let phase_one = {
        let BankLane {
            high_cursor,
            interpolation,
            decimation,
            ..
        } = lane;
        kernel.process_phase(
            &mut first_phase,
            &H,
            high_cursor,
            interpolation.as_flattened_mut(),
            decimation.as_flattened_mut(),
        )
    };
    let mut zero_phase = [0.0_f32; W];
    let phase_two = {
        let BankLane {
            high_cursor,
            interpolation,
            decimation,
            ..
        } = lane;
        kernel.process_phase(
            &mut zero_phase,
            &H,
            high_cursor,
            interpolation.as_flattened_mut(),
            decimation.as_flattened_mut(),
        )
    };
    match phase_one {
        Ok(failed_lanes) => {
            for (track, failed) in failed.iter_mut().enumerate() {
                *failed |= failed_lanes & (1_u32 << track) != 0;
            }
        }
        Err(_) => failed.fill(true),
    }
    match phase_two {
        Ok(failed_lanes) => {
            for (track, failed) in failed.iter_mut().enumerate() {
                *failed |= failed_lanes & (1_u32 << track) != 0;
            }
        }
        Err(_) => failed.fill(true),
    }
    let mut output = [0.0_f32; W];
    for track in 0..W {
        if !failed[track] {
            let mix = lane.ramps[track][2].current;
            let gain = lane.ramps[track][1].current;
            let wet = checked(first_phase[track]);
            let rendered = wet.and_then(|wet| {
                if bypass
                    || (mix.to_bits() == 0.0_f32.to_bits() && gain.to_bits() == 1.0_f32.to_bits())
                {
                    Some(delayed[track])
                } else {
                    let a = checked(1.0_f32 - mix)?;
                    let b = checked(a * delayed[track])?;
                    let c = checked(mix * wet)?;
                    let e = checked(b + c)?;
                    checked(gain * e)
                }
            });
            if let Some(value) = rendered {
                output[track] = value;
                continue;
            }
            failed[track] = true;
        }
        output[track] = delayed[track];
        lane.recover(track);
    }
    (output, failed)
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
    for (index, value) in values.iter().enumerate() {
        let parameter = index / 2;
        let channel = if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        };
        if value.parameter_index != parameter as u32
            || value.channel != channel
            || !parameter_value_valid(parameter, value.value)
            || negative_zero(value.value)
        {
            return Err(EffectPrepareError {
                code: "effect.parameter.initial",
            });
        }
        let converted = convert_parameter(parameter, value.value).ok_or(EffectPrepareError {
            code: "effect.parameter.initial",
        })?;
        if index % 2 == 0 {
            left[parameter] = converted;
        } else {
            right[parameter] = converted;
        }
    }
    Ok((left, right))
}

fn apply_automation(
    spans: &[PreparedAutomationSpan],
    metadata: PreparedEffectMetadata,
    first_sample: u64,
    left: &mut [Ramp; PARAMETER_COUNT],
    right: &mut [Ramp; PARAMETER_COUNT],
    report: &mut ProcessReport,
) {
    let mut pending = [[None; PARAMETER_COUNT]; 2];
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
        let Some(order) = span
            .parameter_index
            .checked_mul(2)
            .and_then(|value| value.checked_add(lane as u32))
        else {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        };
        let valid = span_index < metadata.automation_capacity as usize
            && parameter < PARAMETER_COUNT
            && span.kind == AutomationSpanKind::Point
            && span.start_sample == first_sample
            && span.end_sample == first_sample
            && span.start_value.to_bits() == span.end_value.to_bits()
            && parameter_value_valid(parameter, span.start_value)
            && prior.is_none_or(|previous| order > previous)
            && pending[lane][parameter].is_none();
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        let Some(value) = convert_parameter(parameter, normalize_zero(span.start_value)) else {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        };
        prior = Some(order);
        pending[lane][parameter] = Some(value);
    }
    for (parameter, (left_target, right_target)) in
        pending[0].into_iter().zip(pending[1]).enumerate()
    {
        if let Some(target) = left_target {
            left[parameter].target = target;
            left[parameter].remaining = RAMP_SAMPLES;
        }
        if let Some(target) = right_target {
            right[parameter].target = target;
            right[parameter].remaining = RAMP_SAMPLES;
        }
    }
}

fn convolve(history: &[f32; HISTORY], cursor: usize) -> Option<f32> {
    let mut accumulator = 0.0_f32;
    for tap in TAPS {
        let sample = history[(cursor + HISTORY - tap) % HISTORY];
        let product = checked(H[tap] * sample)?;
        accumulator = checked(accumulator + product)?;
    }
    Some(accumulator)
}

fn cubic(value: f32) -> Option<f32> {
    if value <= -1.0 {
        Some(-2.0_f32 / 3.0_f32)
    } else if value >= 1.0 {
        Some(2.0_f32 / 3.0_f32)
    } else {
        let p0 = checked(value * value)?;
        let p1 = checked(p0 * value)?;
        let p2 = checked(p1 / 3.0_f32)?;
        checked(value - p2)
    }
}

fn delayed_dry(lane: &Lane) -> f32 {
    lane.dry[(lane.dry_cursor as usize + 1) % DRY_HISTORY]
}

fn sanitize(value: f32, counter: &mut u64) -> f32 {
    match sanitize_sample(value) {
        Some(value) => value,
        None => {
            *counter = counter.saturating_add(1);
            0.0
        }
    }
}

fn checked(value: f32) -> Option<f32> {
    if !value.is_finite() {
        None
    } else if value.is_subnormal() {
        Some(0.0)
    } else {
        Some(value)
    }
}

fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && (value == 0.0 || value.is_normal())
}

fn parameter_value_valid(index: usize, value: f32) -> bool {
    value.is_finite()
        && SOFT_CLIP_PARAMETERS_V1
            .get(index)
            .and_then(|parameter| parameter.minimum.zip(parameter.maximum))
            .is_some_and(|(minimum, maximum)| value >= minimum && value <= maximum)
}

fn converted_domain(index: usize, value: f32) -> bool {
    match index {
        0 => (db_gain(-24.0)..=db_gain(36.0)).contains(&value),
        1 => (db_gain(-24.0)..=db_gain(24.0)).contains(&value),
        2 => (0.0..=1.0).contains(&value),
        _ => false,
    }
}

fn state_parameter_valid(index: usize, value: f32) -> bool {
    !negative_zero(value) && normal_or_zero(value) && converted_domain(index, value)
}

fn convert_parameter(index: usize, value: f32) -> Option<f32> {
    if !parameter_value_valid(index, value) {
        return None;
    }
    let value = normalize_zero(value);
    match index {
        0 | 1 => checked(db_gain(value)),
        2 => Some(value),
        _ => None,
    }
}

fn db_gain(value: f32) -> f32 {
    10.0_f32.powf(value * 0.05_f32)
}

fn negative_zero(value: f32) -> bool {
    value.to_bits() == (-0.0_f32).to_bits()
}

fn normalize_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
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
        return Err(state_error("effect.state.length"));
    }
    Ok(())
}

fn write_lane(bytes: &mut [u8], lane: &Lane) {
    write_u32(bytes, 0, lane.high_cursor);
    write_u32(bytes, 1, lane.dry_cursor);
    for (index, ramp) in lane.ramps.iter().enumerate() {
        let word = 2 + index * 3;
        write_f32(bytes, word, ramp.current);
        write_f32(bytes, word + 1, ramp.target);
        write_u32(bytes, word + 2, ramp.remaining);
    }
    for (index, value) in lane.interp.iter().enumerate() {
        write_f32(bytes, 11 + index, *value);
    }
    for (index, value) in lane.decim.iter().enumerate() {
        write_f32(bytes, 74 + index, *value);
    }
    for (index, value) in lane.dry.iter().enumerate() {
        write_f32(bytes, 137 + index, *value);
    }
}

fn read_lane(bytes: &[u8]) -> Result<Lane, StatePayloadError> {
    if bytes.len() != LANE_STATE_BYTES as usize {
        return Err(state_error("effect.state.length"));
    }
    let high_cursor = read_u32(bytes, 0);
    let dry_cursor = read_u32(bytes, 1);
    if high_cursor as usize >= HISTORY || dry_cursor as usize >= DRY_HISTORY {
        return Err(state_error("effect.state.cursor"));
    }
    let mut ramps = [Ramp::fixed(0.0); PARAMETER_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        let word = 2 + index * 3;
        let current = read_f32(bytes, word);
        let target = read_f32(bytes, word + 1);
        let remaining = read_u32(bytes, word + 2);
        if !state_parameter_valid(index, current)
            || !state_parameter_valid(index, target)
            || remaining > RAMP_SAMPLES
        {
            return Err(state_error("effect.state.parameter"));
        }
        *ramp = Ramp {
            current,
            target,
            remaining,
        };
    }
    let mut interp = [0.0; HISTORY];
    let mut decim = [0.0; HISTORY];
    let mut dry = [0.0; DRY_HISTORY];
    for (index, value) in interp.iter_mut().enumerate() {
        *value = read_f32(bytes, 11 + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.history"));
        }
    }
    for (index, value) in decim.iter_mut().enumerate() {
        *value = read_f32(bytes, 74 + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.history"));
        }
    }
    for (index, value) in dry.iter_mut().enumerate() {
        *value = read_f32(bytes, 137 + index);
        if !normal_or_zero(*value) {
            return Err(state_error("effect.state.history"));
        }
    }
    Ok(Lane {
        high_cursor,
        dry_cursor,
        ramps,
        interp,
        decim,
        dry,
    })
}

fn write_u32(bytes: &mut [u8], word: usize, value: u32) {
    let offset = word * 4;
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_f32(bytes: &mut [u8], word: usize, value: f32) {
    write_u32(bytes, word, value.to_bits());
}

fn read_u32(bytes: &[u8], word: usize) -> u32 {
    let offset = word * 4;
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("state length was checked"),
    )
}

fn read_f32(bytes: &[u8], word: usize) -> f32 {
    f32::from_bits(read_u32(bytes, word))
}

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_core::KernelBackendV1;
    use miso_engine_dsp_reference::{
        ReferenceSoftClip, reference_cubic_soft_clip, reference_halfband_63,
    };
    use miso_engine_effect_contract::{
        BankWidth, EffectBankProcessBlock, EffectProcessBlock, LinkMode, PrepareEffectBankRequest,
        PrepareEffectLimits, PreparedNativeEffect, PreparedNativeEffectBank, PreparedPortsV1,
        StatePayloadInput, StatePayloadOutput, validate_descriptor_v1,
    };

    fn initial_values() -> [InitialParameterValue; PARAMETER_COUNT * 2] {
        core::array::from_fn(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: SOFT_CLIP_PARAMETERS_V1[index / 2].default_value,
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
                maximum_total_state_bytes: 1_352,
                maximum_scratch_bytes: 24,
                maximum_automation_spans_per_block: 16,
            },
        }
    }

    fn prepare(values: &[InitialParameterValue]) -> Box<dyn PreparedNativeEffect> {
        SoftClipFactory.prepare(request(values)).expect("prepare")
    }

    fn process(
        effect: &mut dyn PreparedNativeEffect,
        left: &mut [f32],
        right: &mut [f32],
        first: u64,
        automation: &[PreparedAutomationSpan],
    ) -> ProcessReport {
        effect.process(
            EffectProcessBlock::new(left, right, None, first, automation, 128).expect("block"),
        )
    }

    fn snapshot(effect: &dyn PreparedNativeEffect) -> (Vec<u8>, Vec<u8>) {
        let sizes = effect.metadata().state_sizes;
        let mut left = vec![0; sizes.left_bytes as usize];
        let mut right = vec![0; sizes.right_bytes as usize];
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("sizes"),
            )
            .expect("snapshot");
        (left, right)
    }

    fn snapshot_bank(effect: &dyn PreparedNativeEffectBank, track: u32) -> (Vec<u8>, Vec<u8>) {
        let sizes = effect.metadata().program_key.state_sizes;
        let mut left = vec![0; sizes.left_bytes as usize];
        let mut right = vec![0; sizes.right_bytes as usize];
        effect
            .snapshot_track_state_payload(
                track,
                StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("sizes"),
            )
            .expect("snapshot");
        (left, right)
    }

    fn bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|value| value.to_bits()).collect()
    }

    fn rectangular_nonfundamental_ratio_db(samples: &[f64], fundamental_bin: usize) -> f64 {
        assert!(fundamental_bin != 0 && fundamental_bin < samples.len() / 2);
        let length = samples.len() as f64;
        let mut time_energy = 0.0_f64;
        let mut dc = 0.0_f64;
        let mut fundamental_re = 0.0_f64;
        let mut fundamental_im = 0.0_f64;
        for (index, sample) in samples.iter().copied().enumerate() {
            let phase = -core::f64::consts::TAU * fundamental_bin as f64 * index as f64 / length;
            time_energy += sample * sample;
            dc += sample;
            fundamental_re += sample * phase.cos();
            fundamental_im += sample * phase.sin();
        }
        let total_dft_energy = length * time_energy;
        let dc_energy = dc * dc;
        let fundamental_energy =
            2.0 * (fundamental_re * fundamental_re + fundamental_im * fundamental_im);
        let nonfundamental_energy = (total_dft_energy - dc_energy - fundamental_energy).max(0.0);
        assert!(fundamental_energy.is_finite() && fundamental_energy > 0.0);
        assert!(nonfundamental_energy.is_finite() && nonfundamental_energy > 0.0);
        10.0 * (nonfundamental_energy / fundamental_energy).log10()
    }

    fn bank_request<'a>(
        width: BankWidth,
        backend: KernelBackendV1,
        requests: &'a [PrepareEffectRequest<'a>],
    ) -> PrepareEffectBankRequest<'a> {
        PrepareEffectBankRequest {
            backend,
            width,
            requests,
        }
    }

    #[test]
    fn descriptor_resources_and_independent_fir_design_are_frozen() {
        validate_descriptor_v1(&SOFT_CLIP_DESCRIPTOR_V1).expect("descriptor");
        assert_eq!(
            SOFT_CLIP_DESCRIPTOR_V1.supported_link_modes,
            LinkModeSet::DUAL_MONO
        );
        assert_eq!(SOFT_CLIP_DESCRIPTOR_V1.parameters.len(), 3);
        for quality in QUALITIES {
            assert_eq!(quality.latency, LatencySamples(31));
            assert_eq!(quality.tail, TailSamples::Finite(29));
            assert_eq!(quality.maximum_state.left_bytes, 676);
            assert_eq!(quality.maximum_state.right_bytes, 676);
            assert_eq!(quality.scratch_fixed_bytes, 24);
        }
        let reference = reference_halfband_63();
        for (index, (actual, expected)) in H.into_iter().zip(reference).enumerate() {
            let expected = if TAPS.contains(&index) {
                expected as f32
            } else {
                0.0
            };
            assert_eq!(actual.to_bits(), expected.to_bits(), "tap {index}");
        }
        let values = initial_values();
        let mut too_small = request(&values);
        too_small.limits.maximum_total_state_bytes = 1_351;
        assert!(matches!(
            SoftClipFactory.prepare(too_small),
            Err(EffectPrepareError {
                code: "effect.resource.limit"
            })
        ));
    }

    #[test]
    fn scalar_matches_independent_oracle_after_warmup() {
        let mut values = initial_values();
        values[0].value = 18.0;
        values[1].value = 18.0;
        values[2].value = 0.0;
        values[3].value = 0.0;
        values[4].value = 1.0;
        values[5].value = 1.0;
        let mut effect = prepare(&values);
        let mut oracle = ReferenceSoftClip::new(18.0, 0.0, 1.0).expect("oracle");
        let mut input = (0..128)
            .map(|index| (index as f32 * 0.073).sin() * 0.8)
            .collect::<Vec<_>>();
        let expected = input
            .iter()
            .map(|value| oracle.process(*value as f64) as f32)
            .collect::<Vec<_>>();
        let mut right = input.clone();
        process(effect.as_mut(), &mut input, &mut right, 0, &[]);
        for (actual, expected) in input.into_iter().zip(expected).skip(64) {
            assert!(
                (actual - expected).abs() <= 3.0e-6,
                "actual={actual:?}, expected={expected:?}"
            );
        }
    }

    #[test]
    fn frozen_alias_claim_improves_over_independent_naive_cubic() {
        const LENGTH: usize = 16_384;
        const FUNDAMENTAL_BIN: usize = 3_001;
        const WARM_PERIODS: usize = 3;
        const BLOCK: usize = 128;

        let mut values = initial_values();
        values[0].value = 18.0;
        values[1].value = 18.0;
        let mut effect = prepare(&values);
        let mut fixed_2x = Vec::with_capacity(LENGTH);
        for block_start in (0..((WARM_PERIODS + 1) * LENGTH)).step_by(BLOCK) {
            let mut left = [0.0_f32; BLOCK];
            for (offset, sample) in left.iter_mut().enumerate() {
                let index = (block_start + offset) % LENGTH;
                let phase =
                    core::f64::consts::TAU * FUNDAMENTAL_BIN as f64 * index as f64 / LENGTH as f64;
                *sample = phase.sin() as f32;
            }
            let mut right = left;
            let report = process(
                effect.as_mut(),
                &mut left,
                &mut right,
                block_start as u64,
                &[],
            );
            assert_eq!(report, ProcessReport::default());
            assert_eq!(left.map(f32::to_bits), right.map(f32::to_bits));
            if block_start >= WARM_PERIODS * LENGTH {
                fixed_2x.extend(left.into_iter().map(f64::from));
            }
        }
        assert_eq!(fixed_2x.len(), LENGTH);

        let drive = 10.0_f64.powf(18.0 * 0.05);
        let naive_1x = (0..LENGTH)
            .map(|index| {
                let phase =
                    core::f64::consts::TAU * FUNDAMENTAL_BIN as f64 * index as f64 / LENGTH as f64;
                reference_cubic_soft_clip(drive * phase.sin())
            })
            .collect::<Vec<_>>();
        let fixed_2x_ratio_db = rectangular_nonfundamental_ratio_db(&fixed_2x, FUNDAMENTAL_BIN);
        let naive_1x_ratio_db = rectangular_nonfundamental_ratio_db(&naive_1x, FUNDAMENTAL_BIN);
        let improvement_db = naive_1x_ratio_db - fixed_2x_ratio_db;
        println!(
            "issue_053_alias fixed_2x_nonfundamental_ratio_db={fixed_2x_ratio_db:.12} \
             naive_1x_nonfundamental_ratio_db={naive_1x_ratio_db:.12} \
             improvement_db={improvement_db:.12}"
        );
        assert!(fixed_2x_ratio_db.is_finite());
        assert!(naive_1x_ratio_db.is_finite());
        assert!(
            improvement_db >= 2.0,
            "fixed-2x improvement {improvement_db:.12} dB is below 2.0 dB"
        );
    }

    #[test]
    fn wet_impulse_has_exact_group_delay_and_final_causal_support() {
        let values = initial_values();
        let mut effect = prepare(&values);
        let mut left = vec![0.0; 128];
        let mut right = vec![0.0; 128];
        left[0] = 0.001;
        right[0] = -0.001;
        process(effect.as_mut(), &mut left, &mut right, 0, &[]);
        let left_peak = left
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| left.abs().total_cmp(&right.abs()))
            .map(|(index, _)| index)
            .expect("nonempty impulse");
        let right_peak = right
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| left.abs().total_cmp(&right.abs()))
            .map(|(index, _)| index)
            .expect("nonempty impulse");
        assert_eq!(left_peak, 31);
        assert_eq!(right_peak, 31);
        assert_ne!(left[60].to_bits(), 0.0_f32.to_bits());
        assert_ne!(right[60].to_bits(), 0.0_f32.to_bits());
        assert!(left[61..].iter().all(|sample| *sample == 0.0));
        assert!(right[61..].iter().all(|sample| *sample == 0.0));
    }

    #[test]
    fn automation_active_restore_and_both_resets_are_word_exact_and_lane_local() {
        let values = initial_values();
        let mut effect = prepare(&values);
        let spans = [
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Left,
                parameter_index: 0,
                start_sample: 0,
                end_sample: 0,
                start_value: 12.0,
                end_value: 12.0,
            },
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Left,
                parameter_index: 2,
                start_sample: 0,
                end_sample: 0,
                start_value: -0.0,
                end_value: -0.0,
            },
        ];
        let mut left = [0.2];
        let mut right = [0.1];
        let report = process(effect.as_mut(), &mut left, &mut right, 0, &spans);
        assert_eq!(report.invalid_spans, 0);
        let active = snapshot(effect.as_ref());
        let drive_target = db_gain(12.0);
        let first_drive = 1.0_f32 + (drive_target - 1.0_f32) / 64.0_f32;
        let first_mix = 1.0_f32 + (0.0_f32 - 1.0_f32) / 64.0_f32;
        assert_eq!(read_f32(&active.0, 2).to_bits(), first_drive.to_bits());
        assert_eq!(read_f32(&active.0, 3).to_bits(), drive_target.to_bits());
        assert_eq!(read_u32(&active.0, 4), 63);
        assert_eq!(read_f32(&active.0, 8).to_bits(), first_mix.to_bits());
        assert_eq!(read_f32(&active.0, 9).to_bits(), 0.0_f32.to_bits());
        assert_eq!(read_u32(&active.0, 10), 63);
        for word in [2, 3, 5, 6, 8, 9] {
            assert_eq!(read_f32(&active.1, word).to_bits(), 1.0_f32.to_bits());
        }
        for word in [4, 7, 10] {
            assert_eq!(read_u32(&active.1, word), 0);
        }

        let mut continuation_left = [0.3; 16];
        let mut continuation_right = [-0.2; 16];
        let mut expected_left = continuation_left;
        let mut expected_right = continuation_right;
        process(
            effect.as_mut(),
            &mut expected_left,
            &mut expected_right,
            1,
            &[],
        );
        effect
            .restore_state_payload(
                1,
                StatePayloadInput::new(&[], &active.0, &active.1, effect.metadata().state_sizes)
                    .expect("active state"),
            )
            .expect("active restore");
        process(
            effect.as_mut(),
            &mut continuation_left,
            &mut continuation_right,
            1,
            &[],
        );
        assert_eq!(continuation_left, expected_left);
        assert_eq!(continuation_right, expected_right);

        effect
            .restore_state_payload(
                1,
                StatePayloadInput::new(&[], &active.0, &active.1, effect.metadata().state_sizes)
                    .expect("active state"),
            )
            .expect("active restore");
        let mut left = [0.2; 62];
        let mut right = [0.1; 62];
        process(effect.as_mut(), &mut left, &mut right, 1, &[]);
        let update_63 = snapshot(effect.as_ref());
        assert_eq!(read_u32(&update_63.0, 4), 1);
        assert_eq!(read_u32(&update_63.0, 10), 1);
        assert_ne!(read_f32(&update_63.0, 2).to_bits(), drive_target.to_bits());
        assert_ne!(read_f32(&update_63.0, 8).to_bits(), 0.0_f32.to_bits());
        let mut left = [0.2];
        let mut right = [0.1];
        process(effect.as_mut(), &mut left, &mut right, 63, &[]);
        let update_64 = snapshot(effect.as_ref());
        assert_eq!(read_f32(&update_64.0, 2).to_bits(), drive_target.to_bits());
        assert_eq!(read_u32(&update_64.0, 4), 0);
        assert_eq!(read_f32(&update_64.0, 8).to_bits(), 0.0_f32.to_bits());
        assert_eq!(read_u32(&update_64.0, 10), 0);

        effect.reset(ResetKind::DiscontinuityKeepParameters);
        let discontinuity = snapshot(effect.as_ref());
        for lane in [&discontinuity.0, &discontinuity.1] {
            assert_eq!(read_u32(lane, 0), 0);
            assert_eq!(read_u32(lane, 1), 0);
            assert!((11..STATE_WORDS).all(|word| read_u32(lane, word) == 0));
            for word in [4, 7, 10] {
                assert_eq!(read_u32(lane, word), 0);
            }
            for current in [2, 5, 8] {
                assert_eq!(read_u32(lane, current), read_u32(lane, current + 1));
            }
        }
        assert_eq!(
            read_f32(&discontinuity.0, 2).to_bits(),
            drive_target.to_bits()
        );
        assert_eq!(read_f32(&discontinuity.0, 8).to_bits(), 0.0_f32.to_bits());

        effect.reset(ResetKind::FullToDefaults);
        let full = snapshot(effect.as_ref());
        let defaults = snapshot(prepare(&values).as_ref());
        assert_eq!(full, defaults);
    }

    #[test]
    fn delayed_identity_sanitation_and_recovery_are_lane_local() {
        let mut values = initial_values();
        values[4].value = 0.0;
        values[5].value = 0.0;
        let mut effect = prepare(&values);
        let mut left = vec![-0.0; 64];
        let mut right = vec![0.0; 64];
        left[31] = 0.25;
        right[31] = -0.5;
        process(effect.as_mut(), &mut left, &mut right, 0, &[]);
        assert_eq!(left[62].to_bits(), 0.25_f32.to_bits());
        assert_eq!(right[62].to_bits(), (-0.5_f32).to_bits());
        assert_eq!(left[31].to_bits(), (-0.0_f32).to_bits());

        let mut invalid_left = vec![f32::NAN; 1];
        let mut invalid_right = vec![0.1; 1];
        let report = process(
            effect.as_mut(),
            &mut invalid_left,
            &mut invalid_right,
            160,
            &[],
        );
        assert_eq!(report.sanitized_main_samples, 1);
        let mut concrete = PreparedSoftClip {
            metadata: effect.metadata(),
            left_defaults: [1.0, 1.0, 1.0],
            right_defaults: [1.0, 1.0, 1.0],
            left: Lane::new([1.0, 1.0, 1.0]).expect("lane"),
            right: Lane::new([1.0, 1.0, 1.0]).expect("lane"),
        };
        concrete.left.interp[61] = f32::NAN;
        concrete.left.dry[1] = -0.25;
        let mut bad_left = [0.0];
        let mut good_right = [0.0];
        let report = concrete.process(
            EffectProcessBlock::new(&mut bad_left, &mut good_right, None, 0, &[], 128)
                .expect("block"),
        );
        assert_eq!(report.recovered_left_samples, 1);
        assert_eq!(report.recovered_right_samples, 0);
        assert_eq!(bad_left[0].to_bits(), (-0.25_f32).to_bits());
        concrete.reset(ResetKind::DiscontinuityKeepParameters);
        assert_eq!(concrete.left.high_cursor, 0);
        concrete.reset(ResetKind::FullToDefaults);
        assert_eq!(concrete.left.ramps[0].current, 1.0);
    }

    #[test]
    fn bank_binding_validates_before_unavailable_fallback_and_counts_exact_bytes() {
        assert_eq!(bank_effect_bytes(4), Some(5_504));
        assert_eq!(bank_effect_bytes(8), Some(11_008));
        let values: [[InitialParameterValue; PARAMETER_COUNT * 2]; 8] =
            core::array::from_fn(|_| initial_values());
        let requests = values.each_ref().map(|values| request(values));

        assert!(matches!(
            SoftClipFactory.bind_homogeneous_bank(bank_request(
                BankWidth::Four,
                KernelBackendV1::X86Avx2,
                &requests,
            )),
            Err(EffectPrepareError {
                code: "effect.bank.requests"
            })
        ));

        let mut malformed = requests;
        malformed[3].limits.maximum_total_state_bytes = 1_351;
        assert!(matches!(
            SoftClipFactory.bind_homogeneous_bank(bank_request(
                BankWidth::Four,
                KernelBackendV1::WasmSimd128,
                &malformed[..4],
            )),
            Err(EffectPrepareError {
                code: "effect.resource.limit"
            })
        ));

        assert!(matches!(
            SoftClipFactory.bind_homogeneous_bank(bank_request(
                BankWidth::Four,
                KernelBackendV1::WasmSimd128,
                &requests[..4],
            )),
            Ok(None)
        ));

        let mut incompatible = requests;
        incompatible[2].sample_rate = 44_100;
        assert!(matches!(
            SoftClipFactory.bind_homogeneous_bank(bank_request(
                BankWidth::Four,
                KernelBackendV1::WasmSimd128,
                &incompatible[..4],
            )),
            Ok(None)
        ));
    }

    #[test]
    fn available_w8_bank_matches_scalar_state_reports_and_lane_isolation() {
        let Ok(kernel) = PreparedSoftClipBankKernelV1::try_new(KernelBackendV1::X86Avx2) else {
            return;
        };
        let values: [[InitialParameterValue; PARAMETER_COUNT * 2]; 8] =
            core::array::from_fn(|track| {
                let mut values = initial_values();
                values[0].value = -12.0 + track as f32 * 4.0;
                values[1].value = 18.0 - track as f32 * 3.0;
                values[2].value = -6.0 + track as f32 * 2.0;
                values[3].value = 6.0 - track as f32;
                values[4].value = if track == 6 { 0.0 } else { 1.0 };
                values[5].value = if track == 5 { 0.0 } else { 1.0 };
                values
            });
        let requests = values.each_ref().map(|values| request(values));
        let mut bank = SoftClipFactory
            .bind_homogeneous_bank(bank_request(
                BankWidth::Eight,
                KernelBackendV1::X86Avx2,
                &requests,
            ))
            .expect("binding")
            .expect("available AVX2 bank");
        let mut scalars = values
            .iter()
            .map(|values| prepare(values))
            .collect::<Vec<_>>();
        let first_automation = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 0,
            end_sample: 0,
            start_value: 12.0,
            end_value: 12.0,
        };
        let second_automation = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Right,
            parameter_index: 2,
            start_sample: 0,
            end_sample: 0,
            start_value: 0.25,
            end_value: 0.25,
        };
        let automation = [first_automation, second_automation];
        let offsets = [0, 1, 1, 1, 2, 2, 2, 2, 2];
        let mut bank_left = vec![0.0_f32; 8 * 64];
        let mut bank_right = vec![0.0_f32; 8 * 64];
        for frame in 0..64 {
            for track in 0..8 {
                let index = frame * 8 + track;
                bank_left[index] = ((frame * 13 + track * 7) as f32 * 0.03125).sin() * 0.6;
                bank_right[index] = -((frame * 11 + track * 3) as f32 * 0.027).cos() * 0.45;
            }
        }
        bank_left[10 * 8 + 5] = f32::NAN;
        let mut expected_left = bank_left.clone();
        let mut expected_right = bank_right.clone();
        let mut expected_reports = [ProcessReport::default(); 8];
        for track in 0..8 {
            let mut left = (0..64)
                .map(|frame| expected_left[frame * 8 + track])
                .collect::<Vec<_>>();
            let mut right = (0..64)
                .map(|frame| expected_right[frame * 8 + track])
                .collect::<Vec<_>>();
            let spans = if track == 0 {
                &automation[..1]
            } else if track == 3 {
                &automation[1..]
            } else {
                &[]
            };
            expected_reports[track] =
                process(scalars[track].as_mut(), &mut left, &mut right, 0, spans);
            for frame in 0..64 {
                expected_left[frame * 8 + track] = left[frame];
                expected_right[frame * 8 + track] = right[frame];
            }
        }
        let report = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bank_left,
                &mut bank_right,
                None,
                64,
                BankWidth::Eight,
                0,
                &automation,
                &offsets,
                128,
            )
            .expect("bank block"),
        );
        assert_eq!(bits(&bank_left), bits(&expected_left));
        assert_eq!(bits(&bank_right), bits(&expected_right));
        assert_eq!(report.reports, expected_reports);
        for (track, scalar) in scalars.iter().enumerate() {
            assert_eq!(
                snapshot_bank(bank.as_ref(), track as u32),
                snapshot(scalar.as_ref())
            );
        }

        let saved_bank = (0..8)
            .map(|track| snapshot_bank(bank.as_ref(), track))
            .collect::<Vec<_>>();
        let saved_scalar = scalars
            .iter()
            .map(|effect| snapshot(effect.as_ref()))
            .collect::<Vec<_>>();
        let mut bank_next_left = (0..(8 * 16))
            .map(|index| (index as f32 * 0.037).sin() * 0.3)
            .collect::<Vec<_>>();
        let mut bank_next_right = (0..(8 * 16))
            .map(|index| -(index as f32 * 0.019).cos() * 0.2)
            .collect::<Vec<_>>();
        let mut expected_next_left = bank_next_left.clone();
        let mut expected_next_right = bank_next_right.clone();
        for track in 0..8 {
            let mut left = (0..16)
                .map(|frame| expected_next_left[frame * 8 + track])
                .collect::<Vec<_>>();
            let mut right = (0..16)
                .map(|frame| expected_next_right[frame * 8 + track])
                .collect::<Vec<_>>();
            process(scalars[track].as_mut(), &mut left, &mut right, 64, &[]);
            for frame in 0..16 {
                expected_next_left[frame * 8 + track] = left[frame];
                expected_next_right[frame * 8 + track] = right[frame];
            }
        }
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bank_next_left,
                &mut bank_next_right,
                None,
                16,
                BankWidth::Eight,
                64,
                &[],
                &[0; 9],
                128,
            )
            .expect("next block"),
        );
        assert_eq!(bits(&bank_next_left), bits(&expected_next_left));
        assert_eq!(bits(&bank_next_right), bits(&expected_next_right));

        for track in 0..8 {
            bank.restore_track_state_payload(
                track,
                1,
                StatePayloadInput::new(
                    &[],
                    &saved_bank[track as usize].0,
                    &saved_bank[track as usize].1,
                    bank.metadata().program_key.state_sizes,
                )
                .expect("bank state"),
            )
            .expect("bank restore");
            let state_sizes = scalars[track as usize].metadata().state_sizes;
            scalars[track as usize]
                .restore_state_payload(
                    1,
                    StatePayloadInput::new(
                        &[],
                        &saved_scalar[track as usize].0,
                        &saved_scalar[track as usize].1,
                        state_sizes,
                    )
                    .expect("scalar state"),
                )
                .expect("scalar restore");
        }
        let default_values = initial_values();
        let effect_metadata =
            expected_prepared_metadata(&SOFT_CLIP_DESCRIPTOR_V1, request(&default_values))
                .expect("default metadata");
        let defaults = [[1.0; PARAMETER_COUNT]; 8];
        let mut recovered_bank = PreparedSoftClipBank::<8> {
            metadata: PreparedBankMetadata {
                width: BankWidth::Eight,
                program_key: effect_metadata.program_key(),
            },
            effect_metadata,
            kernel,
            left_defaults: defaults,
            right_defaults: defaults,
            left: BankLane::new(&defaults),
            right: BankLane::new(&defaults),
        };
        recovered_bank.left.interpolation[61][0] = f32::NAN;
        recovered_bank.left.dry[1][0] = -0.25;
        let mut scalar_recovery = Lane::new([1.0; PARAMETER_COUNT]).expect("scalar lane");
        scalar_recovery.interp[61] = f32::NAN;
        scalar_recovery.dry[1] = -0.25;
        let scalar_output = scalar_recovery
            .process(0.0, false)
            .expect_err("scalar computed fault");
        scalar_recovery.recover();
        let mut unaffected = Lane::new([1.0; PARAMETER_COUNT]).expect("unaffected lane");
        assert_eq!(unaffected.process(0.0, false), Ok(0.0));

        let mut recovery_left = [0.0; 8];
        let mut recovery_right = [0.0; 8];
        let recovery_report = recovered_bank.process_bank(
            EffectBankProcessBlock::new(
                &mut recovery_left,
                &mut recovery_right,
                None,
                1,
                BankWidth::Eight,
                0,
                &[],
                &[0; 9],
                128,
            )
            .expect("recovery block"),
        );
        assert_eq!(recovery_left[0].to_bits(), scalar_output.to_bits());
        assert_eq!(recovery_report.reports[0].recovered_left_samples, 1);
        assert_eq!(recovery_report.reports[0].recovered_right_samples, 0);
        assert!(
            recovery_report.reports[1..]
                .iter()
                .all(|report| report.recovered_left_samples == 0
                    && report.recovered_right_samples == 0)
        );
        let mut scalar_state = [0; LANE_STATE_BYTES as usize];
        let mut recovered_state = [0; LANE_STATE_BYTES as usize];
        let mut unaffected_state = [0; LANE_STATE_BYTES as usize];
        write_lane(&mut scalar_state, &scalar_recovery);
        write_lane(&mut recovered_state, &recovered_bank.left.lane(0));
        write_lane(&mut unaffected_state, &unaffected);
        assert_eq!(recovered_state, scalar_state);
        for track in 0..8 {
            let mut actual = [0; LANE_STATE_BYTES as usize];
            write_lane(&mut actual, &recovered_bank.right.lane(track));
            assert_eq!(actual, unaffected_state);
        }
        for track in 1..8 {
            let mut actual = [0; LANE_STATE_BYTES as usize];
            write_lane(&mut actual, &recovered_bank.left.lane(track));
            assert_eq!(actual, unaffected_state);
        }

        bank.reset(ResetKind::DiscontinuityKeepParameters);
        for scalar in &mut scalars {
            scalar.reset(ResetKind::DiscontinuityKeepParameters);
        }
        for (track, scalar) in scalars.iter().enumerate() {
            assert_eq!(
                snapshot_bank(bank.as_ref(), track as u32),
                snapshot(scalar.as_ref())
            );
        }
        bank.reset(ResetKind::FullToDefaults);
        for scalar in &mut scalars {
            scalar.reset(ResetKind::FullToDefaults);
        }
        for (track, scalar) in scalars.iter().enumerate() {
            assert_eq!(
                snapshot_bank(bank.as_ref(), track as u32),
                snapshot(scalar.as_ref())
            );
        }
    }
}
