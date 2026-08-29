//! Launch hysteretic peak gate/downward-expander.
//!
//! One generic kernel body owns the whole per-sample graph ([`kernel`]); this module is its
//! control plane — the frozen descriptor, preparation, parameter smoothing, the state payload and
//! the once-per-block boundary policy. Everything scaffolding-shaped comes from
//! `miso-engine-effect-runtime`, everything transcendental from `miso-engine-math` and every
//! vector operation from `miso-engine-lane`; this crate keeps only the gate's own equations.
//!
//! # Shapes
//!
//! A prepared instance is `PreparedGate<L, CONNECTED>`. The scalar instance is `L = f32`, which is
//! literally the `WIDTH = 1` instantiation of the same kernel, so it is the oracle for the four-
//! and eight-lane banks rather than a second implementation of the same arithmetic. `CONNECTED`
//! is `true` only for a scalar instance with a sidechain patched in: a bank requires an
//! unconnected sidechain, so a bank never allocates the detector ring at all (finding F4).
//!
//! # Realtime rules
//!
//! Every allocation happens in [`NativeEffectFactory::prepare`] and
//! [`NativeEffectFactory::bind_homogeneous_bank`] — two `Box<[f32]>` rings per instance, four when
//! a sidechain is connected. `process` and `process_bank` touch nothing else, take no lock, make
//! no syscall and call no platform libm.
#![allow(missing_docs)]

pub mod corpus;
pub mod kernel;

use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptor, EffectPrepareError, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LatencySamples, LinkMode, LinkModeSet, NativeEffectFactory, ObservationCadence,
    ObservationChannels, ObservationCost, ObservationDescriptor, ObservationFold, ObservationKind,
    ObservationSample, ObservationTapId, ParameterChannel, ParameterChannelPolicy,
    ParameterDescriptor, ParameterDomain, ParameterId, ParameterMapping, ParameterUnit,
    PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedBankMetadata, PreparedEffectMetadata, PreparedNativeEffect,
    PreparedNativeEffectBank, PreparedSidechainPort, ProcessReport, ResetKind, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata,
};
use miso_engine_effect_runtime::bank::{check_block, nonfinite_lane_mask};
use miso_engine_effect_runtime::envelope::attack_release_coefficient;
use miso_engine_effect_runtime::params::{
    ParameterSpec, is_negative_zero, normalize_zero, parameter_value_valid,
};
use miso_engine_effect_runtime::ramp::LinearRamp;
use miso_engine_effect_runtime::state_payload as payload;
use miso_engine_lane::{Backend, Lane, Simd4, Simd8};

use kernel::{GateArgs, GateCoef, GateRing, GateState, MAX_WIDTH, RAMP_COUNT, gate_block};

const PARAMETER_COUNT: usize = 8;

/// Effect-owned words at the front of each channel section of the state payload.
const STATE_LANE_HEADER_WORDS: usize = 23;

/// Samples every smoothed parameter takes to reach a new target, from the descriptor.
const RAMP_SAMPLES: u32 = 64;

/// The lane word that encodes an open gate. A closed one is `+0.0`.
const OPEN_WORD: f32 = 1.0;

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
    mapping: ParameterMapping,
    automation_rate: AutomationRate,
    smoothing: SmoothingRule,
    smoothing_samples: u32,
) -> ParameterDescriptor {
    ParameterDescriptor {
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
        lattice: miso_engine_effect_contract::default_parameter_lattice(
            unit,
            ParameterDomain::Continuous,
            mapping,
        ),
    }
}

/// Frozen V1 gate/expander parameters. Descriptor position and stable numeric ID agree.
pub const GATE_EXPANDER_PARAMETERS: [ParameterDescriptor; PARAMETER_COUNT] = [
    parameter(
        1,
        "threshold",
        "dB",
        ParameterUnit::Db,
        -80.0,
        0.0,
        -40.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        2,
        "ratio",
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
        3,
        "range",
        "dB",
        ParameterUnit::Db,
        0.0,
        96.0,
        80.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        4,
        "hysteresis",
        "dB",
        ParameterUnit::Db,
        0.0,
        24.0,
        6.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        5,
        "attack",
        "ms",
        ParameterUnit::Milliseconds,
        0.1,
        50.0,
        1.0,
        ParameterMapping::Logarithmic,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
    parameter(
        6,
        "hold",
        "ms",
        ParameterUnit::Milliseconds,
        0.0,
        1000.0,
        100.0,
        ParameterMapping::Linear,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
    parameter(
        7,
        "release",
        "ms",
        ParameterUnit::Milliseconds,
        5.0,
        2000.0,
        100.0,
        ParameterMapping::Logarithmic,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
    parameter(
        8,
        "lookahead",
        "ms",
        ParameterUnit::Milliseconds,
        0.0,
        10.0,
        2.0,
        ParameterMapping::Linear,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
];

const PORTS: [PortDescriptor; 3] = [
    PortDescriptor {
        id: port_id("main-in"),
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
        id: port_id("main-out"),
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
        id: port_id("sidechain-in"),
        role: PortRole::SidechainInput,
        required: false,
        layout: PortLayout::DualMonoPlanar,
    },
];

/// State layout 2, per channel: 23 effect words, then the two cursor-normalised rings.
///
/// Layout 1 carried a physical ring cursor, a `u32` phase word and three-word ramps, and put no
/// header in the common section at all. Layout 2 drops the cursor (a payload is normalised so that
/// word `j` of a ring is the sample written `N - j` samples ago, which is what lets a track be
/// restored into a bank whose shared cursor is somewhere else), carries the open flag and the hold
/// countdown as the `f32` lane words the kernel actually holds, gives each ramp its precomputed
/// step (D11), and adopts the runtime codec's two-word common header.
const fn quality(sample_rate: u32, latency: u64) -> miso_engine_effect_contract::QualityDescriptor {
    let per_lane = (STATE_LANE_HEADER_WORDS as u32 + 2 * latency as u32) * 4;
    let common = payload::HEADER_WORDS * payload::WORD_BYTES as u32;
    miso_engine_effect_contract::QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(latency),
        tail: TailSamples::Finite(0),
        maximum_state: StatePayloadSizes {
            common_bytes: common,
            left_bytes: per_lane,
            right_bytes: per_lane,
        },
        scratch_fixed_bytes: 64,
        scratch_bytes_per_frame: 0,
    }
}

const QUALITIES: [miso_engine_effect_contract::QualityDescriptor; 4] = [
    quality(44_100, 441),
    quality(48_000, 480),
    quality(88_200, 882),
    quality(96_000, 960),
];

/// State payload layout version.
///
/// Bumped from 1 to 2 by #89: the payload is cursor-normalised, the phase and hold words are the
/// `f32` lane words the kernel holds, each ramp carries its precomputed D11 step, and the common
/// section carries the runtime codec's two-word header. `maximum_state` moves in the same change,
/// which is why the two are one bump and not two.
pub const STATE_LAYOUT_VERSION: u32 = 2;

/// The one declared observation tap: the branching smoother's own gain word.
///
/// `GateState::gain_db` is what `gate_block` writes every sample and reads back on the next one,
/// in decibels and negative for reduction -- a closed gate holds a large negative number and an
/// open one holds `+0.0`. Publishing it is a copy out of state the block wrote anyway.
pub const GATE_EXPANDER_OBSERVATIONS: [ObservationDescriptor; 1] = [ObservationDescriptor {
    id: ObservationTapId(1),
    display_name: "Gain Reduction",
    display_unit: "dB",
    kind: ObservationKind::GainReductionDb,
    unit: ParameterUnit::Db,
    cost: ObservationCost::Resident,
    cadence: ObservationCadence::PerBlock,
    fold: ObservationFold::PeakMagnitude,
    channels: ObservationChannels::PerLane,
    minimum: 0.0,
    maximum: 100.0,
}];

/// Immutable descriptor for the launch gate/expander contract.
pub const GATE_EXPANDER_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.gate-expander"),
    display_name: "Gate / Expander",
    contract_major: 1,
    // Issue #143 P1: declaring the first tap is a `contract_minor` bump and a derived identity
    // re-pin of exactly `32 + len("Gain Reduction") + len("dB")` = 48 bytes.
    // `state_layout_version` does not move: the tap reads state that was already there.
    contract_minor: 1,
    state_layout_version: STATE_LAYOUT_VERSION,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &GATE_EXPANDER_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &GATE_EXPANDER_OBSERVATIONS,
};

/// The parameter domains, in the runtime's vocabulary.
///
/// The descriptor stays the contract; this is the same table in the shape
/// `miso_engine_effect_runtime::params` validates against, so the gate has no domain check of its
/// own. `descriptor_and_specs_agree` (tests/contract.rs) asserts the two cannot drift.
const GATE_SPECS: [ParameterSpec; PARAMETER_COUNT] = [
    ParameterSpec::continuous(-80.0, 0.0, -40.0),
    ParameterSpec::logarithmic(1.0, 20.0, 4.0),
    ParameterSpec::continuous(0.0, 96.0, 80.0),
    ParameterSpec::continuous(0.0, 24.0, 6.0),
    ParameterSpec::logarithmic(0.1, 50.0, 1.0),
    ParameterSpec::continuous(0.0, 1000.0, 100.0),
    ParameterSpec::logarithmic(5.0, 2000.0, 100.0),
    ParameterSpec::continuous(0.0, 10.0, 2.0),
];

/// Factory for the launch gate/expander.
#[derive(Clone, Copy, Debug, Default)]
pub struct GateExpanderFactory;

/// The four unsmoothed times of one lane and channel, kept for the state payload and for the
/// coefficient re-derivation a restore performs.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct LaneTiming {
    lookahead_ms: f32,
    attack_ms: f32,
    hold_ms: f32,
    release_ms: f32,
}

/// One channel's owned delay storage.
struct Ring {
    main: Box<[f32]>,
    detector: Box<[f32]>,
    tap: [u32; MAX_WIDTH],
}

/// A prepared gate at one lane width.
///
/// `CONNECTED` is `true` only when a sidechain is patched in, which is a scalar-only
/// configuration: the detector ring is allocated exactly then, and is a zero-length slice
/// otherwise.
struct PreparedGate<L: Lane, const CONNECTED: bool> {
    metadata: PreparedEffectMetadata,
    bank_width: Option<BankWidth>,
    defaults: [[[f32; PARAMETER_COUNT]; 2]; MAX_WIDTH],
    coef: [GateCoef<L>; 2],
    state: [GateState<L>; 2],
    ring: [Ring; 2],
    timing: [[LaneTiming; 2]; MAX_WIDTH],
    cursor: u32,
    slots: usize,
    slot_mask: u32,
    delay: u32,
    ramp_frames_left: u32,
}

/// Reads one lane out of a lane word.
fn lane_get<L: Lane>(value: L, lane: usize) -> f32 {
    let mut words = [0.0_f32; MAX_WIDTH];
    value.store(&mut words[..L::WIDTH]);
    words[lane]
}

/// Writes one lane of a lane word, leaving the others alone. Control plane only.
fn lane_set<L: Lane>(value: &mut L, lane: usize, sample: f32) {
    let mut words = [0.0_f32; MAX_WIDTH];
    value.store(&mut words[..L::WIDTH]);
    words[lane] = sample;
    *value = L::load(&words[..L::WIDTH]);
}

/// `floor(ms * fs / 1000 + 0.5)`, the frozen sample-count rounding of brief 014.
fn rounded_samples(milliseconds: f32, sample_rate: u32) -> Option<u32> {
    let samples = (f64::from(milliseconds) * f64::from(sample_rate) / 1000.0 + 0.5).floor();
    if !samples.is_finite() || samples < 0.0 || samples > f64::from(u32::MAX) {
        return None;
    }
    Some(samples as u32)
}

/// `true` for a finite value that is either zero or normal.
fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && (value == 0.0 || value.is_normal())
}

/// `true` for a finite value that is a whole number in `[0, limit]`.
fn integral_within(value: f32, limit: f32) -> bool {
    value.is_finite() && value >= 0.0 && value <= limit && value == value.floor()
}

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

/// Maps the runtime codec's error onto the contract's, preserving the code.
fn codec_error(error: payload::StatePayloadError) -> StatePayloadError {
    StatePayloadError { code: error.code }
}

impl<L: Lane, const CONNECTED: bool> PreparedGate<L, CONNECTED> {
    /// The payload layout of one prepared shape.
    fn layout(&self) -> payload::StateLayout {
        payload::StateLayout {
            version: STATE_LAYOUT_VERSION,
            common_words: 0,
            lane_words: STATE_LANE_HEADER_WORDS as u32 + 2 * self.delay,
        }
    }

    /// Builds one prepared shape. `defaults[track][channel]` holds the validated initial values;
    /// only `[..WIDTH]` is read.
    fn new(
        metadata: PreparedEffectMetadata,
        bank_width: Option<BankWidth>,
        defaults: [[[f32; PARAMETER_COUNT]; 2]; MAX_WIDTH],
    ) -> Option<Self> {
        let width = L::WIDTH;
        let delay = u32::try_from(metadata.latency.0).ok()?;
        let slots = (delay as usize).checked_add(1)?.next_power_of_two();
        let slot_mask = u32::try_from(slots - 1).ok()?;
        let connected = matches!(
            metadata.ports.sidechain,
            PreparedSidechainPort::Connected { .. }
        );
        if connected != CONNECTED {
            return None;
        }
        let detector = |slots: usize| -> Box<[f32]> {
            if CONNECTED {
                vec![0.0; slots * width].into_boxed_slice()
            } else {
                Vec::new().into_boxed_slice()
            }
        };
        let mut gate = Self {
            metadata,
            bank_width,
            defaults,
            coef: [GateCoef {
                attack: L::zero(),
                release: L::zero(),
                hold_samples: L::zero(),
                bypass: L::splat(if metadata.bypass { 1.0 } else { 0.0 }),
                link_max: L::splat(f32::from(u8::from(metadata.link_mode == LinkMode::Maximum))),
                link_avg: L::splat(f32::from(u8::from(metadata.link_mode == LinkMode::Average))),
            }; 2],
            state: [GateState::default(); 2],
            ring: [
                Ring {
                    main: vec![0.0; slots * width].into_boxed_slice(),
                    detector: detector(slots),
                    tap: [0; MAX_WIDTH],
                },
                Ring {
                    main: vec![0.0; slots * width].into_boxed_slice(),
                    detector: detector(slots),
                    tap: [0; MAX_WIDTH],
                },
            ],
            timing: [[LaneTiming::default(); 2]; MAX_WIDTH],
            cursor: 0,
            slots,
            slot_mask,
            delay,
            ramp_frames_left: 0,
        };
        for channel in 0..2 {
            for lane in 0..width {
                gate.seed_lane(channel, lane)?;
            }
        }
        Some(gate)
    }

    /// Derives one lane's timing, coefficients and resting ramps from its prepared defaults.
    fn seed_lane(&mut self, channel: usize, lane: usize) -> Option<()> {
        let values = self.defaults[lane][channel];
        self.timing[lane][channel] = LaneTiming {
            lookahead_ms: values[7],
            attack_ms: values[4],
            hold_ms: values[5],
            release_ms: values[6],
        };
        self.rederive_lane(channel, lane)?;
        let hold = lane_get(self.coef[channel].hold_samples, lane);
        lane_set(&mut self.state[channel].gain_db, lane, 0.0);
        lane_set(&mut self.state[channel].hysteresis.open, lane, OPEN_WORD);
        lane_set(&mut self.state[channel].hysteresis.hold, lane, hold);
        for (index, ramp) in self.state[channel].ramps.iter_mut().enumerate() {
            lane_set(&mut ramp.current, lane, values[index]);
            lane_set(&mut ramp.target, lane, values[index]);
            lane_set(&mut ramp.step, lane, 0.0);
            lane_set(&mut ramp.remaining, lane, 0.0);
        }
        Some(())
    }

    /// Recomputes one lane's tap and one-pole coefficients from its timing.
    fn rederive_lane(&mut self, channel: usize, lane: usize) -> Option<()> {
        let timing = self.timing[lane][channel];
        let sample_rate = self.metadata.sample_rate;
        let lookahead = rounded_samples(timing.lookahead_ms, sample_rate)?.min(self.delay);
        let hold = rounded_samples(timing.hold_ms, sample_rate)?;
        self.ring[channel].tap[lane] = self.delay - lookahead;
        lane_set(&mut self.coef[channel].hold_samples, lane, hold as f32);
        lane_set(
            &mut self.coef[channel].attack,
            lane,
            attack_release_coefficient(timing.attack_ms, sample_rate),
        );
        lane_set(
            &mut self.coef[channel].release,
            lane,
            attack_release_coefficient(timing.release_ms, sample_rate),
        );
        Some(())
    }

    /// Clears one lane of one channel back to its prepared defaults: rings, gain, phase, hold and
    /// resting ramps. This is `ResetKind::FullToDefaults` for one lane, and it is also the D7
    /// recovery action for a lane whose block failed the boundary check.
    fn reset_lane_full(&mut self, channel: usize, lane: usize) {
        self.clear_lane_rings(channel, lane);
        let _ = self.seed_lane(channel, lane);
    }

    /// Clears one lane's column of one channel's rings.
    fn clear_lane_rings(&mut self, channel: usize, lane: usize) {
        let width = L::WIDTH;
        let slots = self.slots;
        let ring = &mut self.ring[channel];
        for slot in 0..slots {
            ring.main[slot * width + lane] = 0.0;
            if CONNECTED {
                ring.detector[slot * width + lane] = 0.0;
            }
        }
    }

    /// `ResetKind::DiscontinuityKeepParameters` for one lane: history goes, the smoothed values
    /// snap to their targets and the unsmoothed times are kept.
    fn reset_lane_discontinuity(&mut self, channel: usize, lane: usize) {
        self.clear_lane_rings(channel, lane);
        let hold = lane_get(self.coef[channel].hold_samples, lane);
        lane_set(&mut self.state[channel].gain_db, lane, 0.0);
        lane_set(&mut self.state[channel].hysteresis.open, lane, OPEN_WORD);
        lane_set(&mut self.state[channel].hysteresis.hold, lane, hold);
        for ramp in &mut self.state[channel].ramps {
            let target = lane_get(ramp.target, lane);
            lane_set(&mut ramp.current, lane, target);
            lane_set(&mut ramp.step, lane, 0.0);
            lane_set(&mut ramp.remaining, lane, 0.0);
        }
    }

    fn reset(&mut self, kind: ResetKind) {
        for channel in 0..2 {
            for lane in 0..L::WIDTH {
                match kind {
                    ResetKind::FullToDefaults => self.reset_lane_full(channel, lane),
                    ResetKind::DiscontinuityKeepParameters => {
                        self.reset_lane_discontinuity(channel, lane);
                    }
                }
            }
        }
        self.cursor = 0;
        self.ramp_frames_left = 0;
    }

    /// Validates one block's automation spans for one lane and points its ramps at the accepted
    /// values. The division that D11 permits happens exactly here, in
    /// `LinearRamp::set_target`.
    fn apply_automation(
        &mut self,
        spans: &[PreparedAutomationSpan],
        first_sample: u64,
        lane: usize,
        report: &mut ProcessReport,
    ) {
        let mut pending = [[None; RAMP_COUNT]; 2];
        let mut last_order = None;
        for (span_index, span) in spans.iter().enumerate() {
            let channel = match span.channel {
                ParameterChannel::Left => 0,
                ParameterChannel::Right => 1,
                ParameterChannel::Both => {
                    report.invalid_spans = report.invalid_spans.saturating_add(1);
                    continue;
                }
            };
            let parameter_index = span.parameter_index as usize;
            let Some(order) = span
                .parameter_index
                .checked_mul(2)
                .and_then(|value| value.checked_add(channel as u32))
            else {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            };
            let valid = span_index < self.metadata.automation_capacity as usize
                && parameter_index < RAMP_COUNT
                && span.kind == AutomationSpanKind::Point
                && span.start_sample == first_sample
                && span.end_sample == first_sample
                && span.start_value.to_bits() == span.end_value.to_bits()
                && parameter_value_valid(&GATE_SPECS[parameter_index], span.start_value)
                && last_order.is_none_or(|previous| order > previous)
                && pending[channel][parameter_index].is_none();
            if !valid {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            }
            last_order = Some(order);
            pending[channel][parameter_index] = Some(normalize_zero(span.start_value));
        }
        for (channel, targets) in pending.iter().enumerate() {
            for (index, target) in targets.iter().enumerate() {
                let Some(value) = *target else { continue };
                let slot = &mut self.state[channel].ramps[index];
                let mut ramp = LinearRamp {
                    current: lane_get(slot.current, lane),
                    target: lane_get(slot.target, lane),
                    step: lane_get(slot.step, lane),
                    remaining: lane_get(slot.remaining, lane) as u32,
                };
                ramp.set_target(value, RAMP_SAMPLES);
                lane_set(&mut slot.current, lane, ramp.current);
                lane_set(&mut slot.target, lane, ramp.target);
                lane_set(&mut slot.step, lane, ramp.step);
                lane_set(&mut slot.remaining, lane, ramp.remaining as f32);
                self.ramp_frames_left = RAMP_SAMPLES;
            }
        }
    }

    /// Runs one block through the kernel and applies the D7 boundary policy.
    ///
    /// The block is split at the frame the last ramp finishes, so the smoothing prologue runs as a
    /// const-generic variant rather than as a per-sample branch. Splitting anywhere else, or not
    /// at all, would produce the same samples — the `RAMPING = false` variant omits updates that
    /// are exact no-ops once every `remaining` is zero — which is what makes the render
    /// partition-invariant.
    fn run_block(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        sidechain: Option<(&[f32], &[f32])>,
        frames: usize,
        reports: &mut [ProcessReport; MAX_WIDTH],
    ) {
        let width = L::WIDTH;
        let smoothed = (self.ramp_frames_left as usize).min(frames);
        if smoothed > 0 {
            let split = smoothed * width;
            let side = sidechain.map(|(l, r)| (&l[..split], &r[..split]));
            self.run_segment::<true>(&mut left[..split], &mut right[..split], side, smoothed);
        }
        if smoothed < frames {
            let split = smoothed * width;
            let side = sidechain.map(|(l, r)| (&l[split..], &r[split..]));
            self.run_segment::<false>(
                &mut left[split..],
                &mut right[split..],
                side,
                frames - smoothed,
            );
        }
        self.ramp_frames_left -= smoothed as u32;
        self.finish_block(left, right, frames, reports);
    }

    fn run_segment<const RAMPING: bool>(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        sidechain: Option<(&[f32], &[f32])>,
        frames: usize,
    ) {
        let (state_left, state_right) = self.state.split_at_mut(1);
        let (ring_left, ring_right) = self.ring.split_at_mut(1);
        let Ring {
            main: main_left,
            detector: detector_left,
            tap: tap_left,
        } = &mut ring_left[0];
        let Ring {
            main: main_right,
            detector: detector_right,
            tap: tap_right,
        } = &mut ring_right[0];
        gate_block::<L, CONNECTED, RAMPING>(GateArgs {
            left,
            right,
            sidechain,
            frames,
            coef: (&self.coef[0], &self.coef[1]),
            state: (&mut state_left[0], &mut state_right[0]),
            rings: (
                GateRing {
                    main: main_left,
                    detector: detector_left,
                    tap: tap_left,
                },
                GateRing {
                    main: main_right,
                    detector: detector_right,
                    tap: tap_right,
                },
            ),
            cursor: &mut self.cursor,
            slot_mask: self.slot_mask,
            delay: self.delay,
        });
    }

    /// The D7 boundary check, once per block per channel, and the lane-local recovery.
    ///
    /// The scan and the lane attribution are `miso_engine_effect_runtime::bank`'s §4.4 forms. The
    /// gain words are scanned as a one-frame block of their own, because a NaN `G` produces a
    /// perfectly finite output: `exp2_lane` clamps its argument with the D8 `max`/`min`, which
    /// swallow NaN, so the output block alone would never show it.
    ///
    /// Recovery is *lane-local and channel-local*, which is where this departs from
    /// `bank::finish_block`: that zeroes both channels and resets the whole bank, and issue #48
    /// froze the gate's recovery as one track's one channel, with the other lanes bit-unchanged.
    fn finish_block(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
        reports: &mut [ProcessReport; MAX_WIDTH],
    ) {
        let width = L::WIDTH;
        let mut gains = [0.0_f32; MAX_WIDTH];
        for (channel, block) in [left, right].into_iter().enumerate() {
            self.state[channel].gain_db.store(&mut gains[..width]);
            if check_block::<L>(block) && check_block::<L>(&gains[..width]) {
                continue;
            }
            let failed =
                nonfinite_lane_mask::<L>(block) | nonfinite_lane_mask::<L>(&gains[..width]);
            for lane in 0..width {
                if failed & (1 << lane) == 0 {
                    continue;
                }
                for frame in 0..frames {
                    block[frame * width + lane] = 0.0;
                }
                self.reset_lane_full(channel, lane);
                let report = &mut reports[lane];
                let counter = if channel == 0 {
                    &mut report.nonfinite_left_blocks
                } else {
                    &mut report.nonfinite_right_blocks
                };
                *counter = counter.saturating_add(frames as u64);
            }
        }
    }

    /// Writes one lane's channel section of the state payload.
    fn write_lane(&self, channel: usize, lane: usize, bytes: &mut [u8]) {
        let state = &self.state[channel];
        payload::write_f32(bytes, 0, lane_get(state.gain_db, lane));
        payload::write_f32(bytes, 1, lane_get(state.hysteresis.open, lane));
        payload::write_f32(bytes, 2, lane_get(state.hysteresis.hold, lane));
        let timing = self.timing[lane][channel];
        payload::write_f32(bytes, 3, timing.lookahead_ms);
        payload::write_f32(bytes, 4, timing.attack_ms);
        payload::write_f32(bytes, 5, timing.hold_ms);
        payload::write_f32(bytes, 6, timing.release_ms);
        for (index, ramp) in state.ramps.iter().enumerate() {
            let word = STATE_RAMP_WORD + index * 4;
            payload::write_f32(bytes, word, lane_get(ramp.current, lane));
            payload::write_f32(bytes, word + 1, lane_get(ramp.target, lane));
            payload::write_f32(bytes, word + 2, lane_get(ramp.step, lane));
            payload::write_f32(bytes, word + 3, lane_get(ramp.remaining, lane));
        }
        let width = L::WIDTH;
        let ring = &self.ring[channel];
        let live = self.cursor.wrapping_sub(self.delay);
        for word in 0..self.delay as usize {
            let slot = ((live.wrapping_add(word as u32) & self.slot_mask) as usize) * width + lane;
            payload::write_f32(bytes, STATE_LANE_HEADER_WORDS + word, ring.main[slot]);
            let detector = if CONNECTED { ring.detector[slot] } else { 0.0 };
            payload::write_f32(
                bytes,
                STATE_LANE_HEADER_WORDS + self.delay as usize + word,
                detector,
            );
        }
    }

    fn snapshot_lane(
        &self,
        lane: usize,
        output: &mut StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let layout = self.layout();
        payload::validate_lengths(
            &layout,
            (output.common.len(), output.left.len(), output.right.len()),
        )
        .map_err(codec_error)?;
        payload::write_header(&layout, output.common);
        self.write_lane(0, lane, output.left);
        self.write_lane(1, lane, output.right);
        Ok(())
    }

    /// Parses and validates one channel section without touching any state.
    ///
    /// Every value is checked here so that a restore is all-or-none: two calls of this, both
    /// accepted, before the first word of either channel is written.
    fn parse_lane(&self, bytes: &[u8]) -> Result<LaneRestore, StatePayloadError> {
        let gain_db = payload::read_f32(bytes, 0);
        let open = payload::read_f32(bytes, 1);
        let hold = payload::read_f32(bytes, 2);
        let timing = LaneTiming {
            lookahead_ms: payload::read_f32(bytes, 3),
            attack_ms: payload::read_f32(bytes, 4),
            hold_ms: payload::read_f32(bytes, 5),
            release_ms: payload::read_f32(bytes, 6),
        };
        for (index, value) in [
            (7_usize, timing.lookahead_ms),
            (4, timing.attack_ms),
            (5, timing.hold_ms),
            (6, timing.release_ms),
        ] {
            if is_negative_zero(value) || !parameter_value_valid(&GATE_SPECS[index], value) {
                return Err(state_error("effect.state.parameter"));
            }
        }
        if !normal_or_zero(gain_db) || !(-96.0..=0.0).contains(&gain_db) {
            return Err(state_error("effect.state.gain"));
        }
        if open.to_bits() != 0 && open.to_bits() != OPEN_WORD.to_bits() {
            return Err(state_error("effect.state.phase"));
        }
        let hold_samples = rounded_samples(timing.hold_ms, self.metadata.sample_rate)
            .ok_or(state_error("effect.state.parameter"))?;
        if !integral_within(hold, hold_samples as f32) {
            return Err(state_error("effect.state.hold"));
        }
        let mut ramps = [[0.0_f32; 4]; RAMP_COUNT];
        for (index, ramp) in ramps.iter_mut().enumerate() {
            let word = STATE_RAMP_WORD + index * 4;
            let current = payload::read_f32(bytes, word);
            let target = payload::read_f32(bytes, word + 1);
            let step = payload::read_f32(bytes, word + 2);
            let remaining = payload::read_f32(bytes, word + 3);
            let resting = remaining == 0.0;
            if is_negative_zero(current)
                || is_negative_zero(target)
                || !parameter_value_valid(&GATE_SPECS[index], current)
                || !parameter_value_valid(&GATE_SPECS[index], target)
                || !integral_within(remaining, RAMP_SAMPLES as f32)
                || !normal_or_zero(step)
                || (resting && step.to_bits() != 0)
            {
                return Err(state_error("effect.state.parameter"));
            }
            *ramp = [current, target, step, remaining];
        }
        for word in 0..2 * self.delay as usize {
            let value = payload::read_f32(bytes, STATE_LANE_HEADER_WORDS + word);
            let detector = word >= self.delay as usize;
            if !normal_or_zero(value) || (detector && !CONNECTED && value.to_bits() != 0) {
                return Err(state_error("effect.state.ring"));
            }
        }
        Ok(LaneRestore {
            gain_db: normalize_zero(gain_db),
            open,
            hold,
            timing,
            ramps,
        })
    }

    /// Commits one validated channel section into one lane.
    fn commit_lane(
        &mut self,
        channel: usize,
        lane: usize,
        parsed: &LaneRestore,
        bytes: &[u8],
    ) -> Result<(), StatePayloadError> {
        self.timing[lane][channel] = parsed.timing;
        self.rederive_lane(channel, lane)
            .ok_or(state_error("effect.state.parameter"))?;
        lane_set(&mut self.state[channel].gain_db, lane, parsed.gain_db);
        lane_set(&mut self.state[channel].hysteresis.open, lane, parsed.open);
        lane_set(&mut self.state[channel].hysteresis.hold, lane, parsed.hold);
        for (index, ramp) in self.state[channel].ramps.iter_mut().enumerate() {
            lane_set(&mut ramp.current, lane, parsed.ramps[index][0]);
            lane_set(&mut ramp.target, lane, parsed.ramps[index][1]);
            lane_set(&mut ramp.step, lane, parsed.ramps[index][2]);
            lane_set(&mut ramp.remaining, lane, parsed.ramps[index][3]);
        }
        if parsed.ramps.iter().any(|ramp| ramp[3] != 0.0) {
            self.ramp_frames_left = self.ramp_frames_left.max(RAMP_SAMPLES);
        }
        let width = L::WIDTH;
        let live = self.cursor.wrapping_sub(self.delay);
        let delay = self.delay as usize;
        let mask = self.slot_mask;
        let ring = &mut self.ring[channel];
        for word in 0..delay {
            let slot = ((live.wrapping_add(word as u32) & mask) as usize) * width + lane;
            ring.main[slot] = payload::read_f32(bytes, STATE_LANE_HEADER_WORDS + word);
            if CONNECTED {
                ring.detector[slot] =
                    payload::read_f32(bytes, STATE_LANE_HEADER_WORDS + delay + word);
            }
        }
        Ok(())
    }

    fn restore_lane(
        &mut self,
        lane: usize,
        state_layout_version: u32,
        input: &StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if state_layout_version != STATE_LAYOUT_VERSION {
            return Err(state_error("effect.state.version"));
        }
        let layout = self.layout();
        payload::validate_lengths(
            &layout,
            (input.common.len(), input.left.len(), input.right.len()),
        )
        .map_err(codec_error)?;
        payload::read_header(&layout, input.common).map_err(codec_error)?;
        let left = self.parse_lane(input.left)?;
        let right = self.parse_lane(input.right)?;
        self.commit_lane(0, lane, &left, input.left)?;
        self.commit_lane(1, lane, &right, input.right)
    }
}

/// Word index of the first ramp quadruple in a channel section.
const STATE_RAMP_WORD: usize = 7;

/// One validated channel section, held on the stack until both channels have been accepted.
struct LaneRestore {
    gain_db: f32,
    open: f32,
    hold: f32,
    timing: LaneTiming,
    ramps: [[f32; 4]; RAMP_COUNT],
}

impl<const CONNECTED: bool> PreparedNativeEffect for PreparedGate<f32, CONNECTED> {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }

    fn reset(&mut self, kind: ResetKind) {
        PreparedGate::reset(self, kind);
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut reports = [ProcessReport::default(); MAX_WIDTH];
        self.apply_automation(block.automation, block.first_sample, 0, &mut reports[0]);
        let frames = block.frames();
        let sidechain = if CONNECTED { block.sidechain } else { None };
        self.run_block(block.left, block.right, sidechain, frames, &mut reports);
        reports[0]
    }

    /// Issue #143 D2: the branching smoother's own gain word, read for lane 0.
    fn observe_resident(&self, tap_index: u32, out: &mut ObservationSample) -> bool {
        if tap_index != 0 {
            return false;
        }
        out.left = lane_get(self.state[0].gain_db, 0);
        out.right = lane_get(self.state[1].gain_db, 0);
        true
    }

    fn snapshot_state_payload(
        &self,
        mut output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.snapshot_lane(0, &mut output)
    }

    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.restore_lane(0, state_layout_version, &input)
    }
}

/// Rejects a track index the bank does not have.
fn checked_track(track_index: u32, width: usize) -> Result<usize, StatePayloadError> {
    let track = usize::try_from(track_index).map_err(|_| state_error("effect.state.track"))?;
    if track >= width {
        return Err(state_error("effect.state.track"));
    }
    Ok(track)
}

macro_rules! bank_impl {
    ($lane:ty) => {
        impl PreparedNativeEffectBank for PreparedGate<$lane, false> {
            fn observe_resident_bank(&self, tap_index: u32, out: &mut [ObservationSample]) -> bool {
                let lanes = <$lane as Lane>::WIDTH;
                if tap_index != 0 || out.len() != lanes {
                    return false;
                }
                let mut left = [0.0_f32; MAX_WIDTH];
                let mut right = [0.0_f32; MAX_WIDTH];
                self.state[0].gain_db.store(&mut left[..lanes]);
                self.state[1].gain_db.store(&mut right[..lanes]);
                for (lane, sample) in out.iter_mut().enumerate() {
                    sample.left = left[lane];
                    sample.right = right[lane];
                }
                true
            }

            fn metadata(&self) -> PreparedBankMetadata {
                PreparedBankMetadata {
                    width: self.bank_width.expect("a bank is prepared with a width"),
                    program_key: self.metadata.program_key(),
                }
            }

            fn reset(&mut self, kind: ResetKind) {
                PreparedGate::reset(self, kind);
            }

            fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
                let width = self.bank_width.expect("a bank is prepared with a width");
                let mut report = BankProcessReport::empty(width);
                // The block constructor validated every shape (finding F11); what it cannot know
                // is which bank the block is handed to, so the width and the absent sidechain --
                // a bank is only bound for an unconnected sidechain port -- stay real guards.
                if block.width != width || block.sidechain.is_some() {
                    return report;
                }
                debug_assert_eq!(block.left.len(), block.frames as usize * <$lane>::WIDTH);
                debug_assert!(block.frames <= self.metadata.quantum);
                let mut reports = [ProcessReport::default(); MAX_WIDTH];
                for track in 0..<$lane>::WIDTH {
                    let start = block.automation_offsets[track] as usize;
                    let end = block.automation_offsets[track + 1] as usize;
                    self.apply_automation(
                        &block.automation[start..end],
                        block.first_sample,
                        track,
                        &mut reports[track],
                    );
                }
                let frames = block.frames as usize;
                self.run_block(block.left, block.right, None, frames, &mut reports);
                report.reports[..<$lane>::WIDTH].copy_from_slice(&reports[..<$lane>::WIDTH]);
                report
            }

            fn snapshot_track_state_payload(
                &self,
                track_index: u32,
                mut output: StatePayloadOutput<'_>,
            ) -> Result<(), StatePayloadError> {
                let track = checked_track(track_index, <$lane>::WIDTH)?;
                self.snapshot_lane(track, &mut output)
            }

            fn restore_track_state_payload(
                &mut self,
                track_index: u32,
                state_layout_version: u32,
                input: StatePayloadInput<'_>,
            ) -> Result<(), StatePayloadError> {
                let track = checked_track(track_index, <$lane>::WIDTH)?;
                self.restore_lane(track, state_layout_version, &input)
            }
        }
    };
}

bank_impl!(Simd4);
bank_impl!(Simd8);

/// Validates one prepare request's initial values against the frozen domains.
fn initial_defaults(
    values: &[InitialParameterValue],
) -> Result<[[f32; PARAMETER_COUNT]; 2], EffectPrepareError> {
    if values.len() != PARAMETER_COUNT * 2 {
        return Err(EffectPrepareError {
            code: "effect.parameter.initial",
        });
    }
    let mut defaults = [[0.0; PARAMETER_COUNT]; 2];
    for (index, spec) in GATE_SPECS.iter().enumerate() {
        let left = values[index * 2];
        let right = values[index * 2 + 1];
        if left.parameter_index != index as u32
            || right.parameter_index != index as u32
            || left.channel != ParameterChannel::Left
            || right.channel != ParameterChannel::Right
            || !parameter_value_valid(spec, left.value)
            || !parameter_value_valid(spec, right.value)
            || is_negative_zero(left.value)
            || is_negative_zero(right.value)
        {
            return Err(EffectPrepareError {
                code: "effect.parameter.initial",
            });
        }
        defaults[0][index] = normalize_zero(left.value);
        defaults[1][index] = normalize_zero(right.value);
    }
    Ok(defaults)
}

impl NativeEffectFactory for GateExpanderFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &GATE_EXPANDER_DESCRIPTOR
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let values = initial_defaults(request.initial_values)?;
        let mut defaults = [[[0.0; PARAMETER_COUNT]; 2]; MAX_WIDTH];
        defaults[0] = values;
        let connected = matches!(
            metadata.ports.sidechain,
            PreparedSidechainPort::Connected { .. }
        );
        let invalid = EffectPrepareError {
            code: "effect.parameter.initial",
        };
        if connected {
            Ok(Box::new(
                PreparedGate::<f32, true>::new(metadata, None, defaults).ok_or(invalid)?,
            ))
        } else {
            Ok(Box::new(
                PreparedGate::<f32, false>::new(metadata, None, defaults).ok_or(invalid)?,
            ))
        }
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
        let first = request.requests[0];
        let metadata = expected_prepared_metadata(self.descriptor(), first)?;
        let mut defaults = [[[0.0; PARAMETER_COUNT]; 2]; MAX_WIDTH];
        let mut same_program = true;
        for (track, item) in request.requests.iter().copied().enumerate() {
            let candidate = expected_prepared_metadata(self.descriptor(), item)?;
            if candidate.program_key() != metadata.program_key() {
                same_program = false;
            }
            defaults[track] = initial_defaults(item.initial_values)?;
        }
        // Every request is validated before the shape is decided, so a malformed member is an
        // error and an unbankable-but-valid cohort is a legal `Ok(None)` fallback.
        if !same_program
            || !matches!(
                metadata.ports.sidechain,
                PreparedSidechainPort::Unconnected {
                    id,
                    required: false,
                } if id == port_id("sidechain-in")
            )
        {
            return Ok(None);
        }
        // There is no runtime SIMD dispatch (master plan #83 D4, revision 4): `wide` picks its
        // instruction set at compile time and the workspace pins `x86-64-v3`, so the available
        // width is a compile-time constant and an unavailable backend is a scalar fallback.
        let available = match request.width {
            BankWidth::Four => Backend::current() == Backend::Simd4,
            BankWidth::Eight => Backend::current() == Backend::Simd8,
        };
        if !available {
            return Ok(None);
        }
        let invalid = EffectPrepareError {
            code: "effect.parameter.initial",
        };
        let width = Some(request.width);
        Ok(Some(match request.width {
            BankWidth::Four => Box::new(
                PreparedGate::<Simd4, false>::new(metadata, width, defaults).ok_or(invalid)?,
            ) as Box<dyn PreparedNativeEffectBank>,
            BankWidth::Eight => Box::new(
                PreparedGate::<Simd8, false>::new(metadata, width, defaults).ok_or(invalid)?,
            ) as Box<dyn PreparedNativeEffectBank>,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_effect_contract::{PrepareEffectLimits, PreparedPorts, validate_descriptor};
    use miso_engine_effect_runtime::params::ParameterKind;

    impl<L: Lane, const CONNECTED: bool> PreparedGate<L, CONNECTED> {
        /// Test-only fault injection: makes one lane's smoothed gain a NaN.
        ///
        /// Issue #48 forbids a public injection API, so this exists only under `cfg(test)` and is
        /// the only way to reach the case where the *state* is non-finite while the output block
        /// is not — which is exactly the case `exp2_lane`'s NaN-swallowing clamp creates.
        fn inject_nonfinite_gain(&mut self, lane: usize, channel: usize) {
            lane_set(&mut self.state[channel].gain_db, lane, f32::NAN);
        }
    }

    fn values(threshold: f32) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
        let defaults = [threshold, 20.0, 48.0, 6.0, 1.0, 0.0, 5.0, 10.0];
        core::array::from_fn(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: defaults[index / 2],
        })
    }

    fn metadata(values: &[InitialParameterValue]) -> PreparedEffectMetadata {
        let quality = GATE_EXPANDER_DESCRIPTOR
            .qualities
            .iter()
            .find(|quality| quality.sample_rate == 48_000)
            .expect("launch rate");
        expected_prepared_metadata(
            &GATE_EXPANDER_DESCRIPTOR,
            PrepareEffectRequest {
                sample_rate: 48_000,
                quantum: 128,
                quality: EffectQuality::Normal,
                bypass: false,
                link_mode: LinkMode::DualMono,
                ports: PreparedPorts {
                    sidechain: PreparedSidechainPort::Unconnected {
                        id: port_id("sidechain-in"),
                        required: false,
                    },
                },
                initial_values: values,
                limits: PrepareEffectLimits {
                    maximum_total_state_bytes: quality.maximum_state.total().expect("total"),
                    maximum_scratch_bytes: 64,
                    maximum_automation_spans_per_block: 16,
                },
            },
        )
        .expect("prepared metadata")
    }

    fn noise(seed: u64, frames: usize) -> Vec<f32> {
        let mut state = seed | 1;
        (0..frames)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let word = (state >> 32) as u32;
                (f32::from((word >> 16) as u16) * (2.0 / 65_536.0) - 1.0) * 0.3
            })
            .collect()
    }

    #[test]
    fn the_runtime_parameter_specs_agree_with_the_frozen_descriptor() {
        validate_descriptor(&GATE_EXPANDER_DESCRIPTOR).expect("descriptor");
        for (index, descriptor) in GATE_EXPANDER_PARAMETERS.iter().enumerate() {
            let spec = GATE_SPECS[index];
            assert_eq!(spec.kind, ParameterKind::Continuous, "parameter {index}");
            assert_eq!(Some(spec.minimum), descriptor.minimum, "min {index}");
            assert_eq!(Some(spec.maximum), descriptor.maximum, "max {index}");
            assert_eq!(spec.default, descriptor.default_value, "default {index}");
            let expected = match descriptor.mapping {
                ParameterMapping::Linear => {
                    miso_engine_effect_runtime::params::ParameterMapping::Linear
                }
                ParameterMapping::Logarithmic => {
                    miso_engine_effect_runtime::params::ParameterMapping::Logarithmic
                }
                other => panic!("parameter {index} has an unexpected mapping {other:?}"),
            };
            assert_eq!(spec.mapping, expected, "mapping {index}");
        }
        for (index, descriptor) in GATE_EXPANDER_PARAMETERS.iter().enumerate() {
            assert_eq!(
                descriptor.smoothing_samples,
                if index < RAMP_COUNT { RAMP_SAMPLES } else { 0 },
                "smoothing {index}"
            );
        }
    }

    #[test]
    fn injected_nonfinite_gain_has_scalar_w8_parity() {
        const FRAMES: usize = 128;
        const WARM: usize = 5;
        if Backend::current() != Backend::Simd8 {
            return;
        }
        let track_values: [_; MAX_WIDTH] =
            core::array::from_fn(|track| values(-20.0 - track as f32));
        let mut defaults = [[[0.0; PARAMETER_COUNT]; 2]; MAX_WIDTH];
        for (track, set) in track_values.iter().enumerate() {
            defaults[track] = initial_defaults(set).expect("initial values");
        }
        let shared = metadata(&track_values[0]);
        let mut bank = PreparedGate::<Simd8, false>::new(shared, Some(BankWidth::Eight), defaults)
            .expect("bank");
        let mut control =
            PreparedGate::<Simd8, false>::new(shared, Some(BankWidth::Eight), defaults)
                .expect("control bank");
        let mut peers: Vec<PreparedGate<f32, false>> = (0..MAX_WIDTH)
            .map(|track| {
                let mut lane_defaults = [[[0.0; PARAMETER_COUNT]; 2]; MAX_WIDTH];
                lane_defaults[0] = defaults[track];
                PreparedGate::<f32, false>::new(shared, None, lane_defaults).expect("peer")
            })
            .collect();

        let sources: Vec<Vec<f32>> = (0..MAX_WIDTH)
            .map(|track| noise(17 + track as u64, FRAMES * (WARM + 1)))
            .collect();
        let mut bank_reports = [ProcessReport::default(); MAX_WIDTH];
        let mut peer_reports = [ProcessReport::default(); MAX_WIDTH];
        let mut bank_left = vec![0.0_f32; FRAMES * MAX_WIDTH];
        let mut bank_right = vec![0.0_f32; FRAMES * MAX_WIDTH];
        let mut control_left = bank_left.clone();
        let mut control_right = bank_right.clone();
        let mut peer_left = vec![vec![0.0_f32; FRAMES]; MAX_WIDTH];
        let mut peer_right = vec![vec![0.0_f32; FRAMES]; MAX_WIDTH];

        for block in 0..=WARM {
            if block == WARM {
                bank.inject_nonfinite_gain(3, 0);
                peers[3].inject_nonfinite_gain(0, 0);
            }
            for track in 0..MAX_WIDTH {
                for frame in 0..FRAMES {
                    let sample = sources[track][block * FRAMES + frame];
                    bank_left[frame * MAX_WIDTH + track] = sample;
                    bank_right[frame * MAX_WIDTH + track] = sample;
                    control_left[frame * MAX_WIDTH + track] = sample;
                    control_right[frame * MAX_WIDTH + track] = sample;
                    peer_left[track][frame] = sample;
                    peer_right[track][frame] = sample;
                }
            }
            bank_reports = [ProcessReport::default(); MAX_WIDTH];
            peer_reports = [ProcessReport::default(); MAX_WIDTH];
            bank.run_block(
                &mut bank_left,
                &mut bank_right,
                None,
                FRAMES,
                &mut bank_reports,
            );
            control.run_block(
                &mut control_left,
                &mut control_right,
                None,
                FRAMES,
                &mut [ProcessReport::default(); MAX_WIDTH],
            );
            for track in 0..MAX_WIDTH {
                let mut single = [ProcessReport::default(); MAX_WIDTH];
                peers[track].run_block(
                    &mut peer_left[track],
                    &mut peer_right[track],
                    None,
                    FRAMES,
                    &mut single,
                );
                peer_reports[track] = single[0];
            }
        }

        // The NaN never reaches the output: `exp2_lane`'s D8 clamp swallows it, so `A` is finite
        // and `z * A` is finite. Only the block-end scan of the gain words sees it.
        for track in 0..MAX_WIDTH {
            for frame in 0..FRAMES {
                assert_eq!(
                    bank_left[frame * MAX_WIDTH + track].to_bits(),
                    peer_left[track][frame].to_bits(),
                    "left track {track} frame {frame}"
                );
                assert_eq!(
                    bank_right[frame * MAX_WIDTH + track].to_bits(),
                    peer_right[track][frame].to_bits(),
                    "right track {track} frame {frame}"
                );
            }
            assert_eq!(bank_reports[track], peer_reports[track], "report {track}");
            if track == 3 {
                assert_eq!(bank_reports[track].nonfinite_left_blocks, FRAMES as u64);
                assert_eq!(bank_reports[track].nonfinite_right_blocks, 0);
                for frame in 0..FRAMES {
                    assert_eq!(bank_left[frame * MAX_WIDTH + track].to_bits(), 0);
                }
                assert_eq!(lane_get(bank.state[0].gain_db, 3).to_bits(), 0, "G is +0");
                assert_eq!(lane_get(bank.state[0].hysteresis.open, 3), OPEN_WORD);
                assert_eq!(
                    lane_get(bank.state[0].hysteresis.hold, 3),
                    lane_get(bank.coef[0].hold_samples, 3)
                );
            } else {
                assert_eq!(bank_reports[track], ProcessReport::default());
                for frame in 0..FRAMES {
                    assert_eq!(
                        bank_left[frame * MAX_WIDTH + track].to_bits(),
                        control_left[frame * MAX_WIDTH + track].to_bits(),
                        "track {track} is untouched by track 3's recovery"
                    );
                }
            }
            assert_eq!(
                bank_right[3].to_bits(),
                control_right[3].to_bits(),
                "the right channel of track 3 is untouched"
            );
        }
    }
}
