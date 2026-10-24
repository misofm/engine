//! Shared fixtures for the dynamic EQ's integration tests.
#![allow(dead_code, unreachable_pub)]

use dynamic_eq::{DYNAMIC_EQ_DESCRIPTOR, DYNAMIC_EQ_PARAMETERS, DynamicEqFactory};
use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PortId, PrepareEffectBankRequest,
    PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan, PreparedEffect,
    PreparedEffectBank, PreparedPorts, PreparedSidechainPort, ProcessReport, StatePayloadOutput,
};
use lane::Backend;

pub const RATE: u32 = 48_000;
pub const PARAMETERS: usize = 48;
pub type Values = [InitialParameterValue; PARAMETERS * 2];

/// This build's own bank width and its lane count.
pub const NATIVE_LANES: usize = <lane::Native as lane::Lane>::WIDTH;
pub fn native_width() -> BankWidth {
    BankWidth::for_backend(Backend::current()).expect("a vector build")
}

/// The descriptor defaults in the interleaved left/right order `prepare` requires.
pub fn defaults() -> Values {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: DYNAMIC_EQ_PARAMETERS[index / 2].default_value,
    })
}

/// Sets one parameter on both channels.
pub fn set(values: &mut Values, parameter: usize, value: f32) {
    values[parameter * 2].value = value;
    values[parameter * 2 + 1].value = value;
}

pub fn request(values: &[InitialParameterValue], link_mode: LinkMode) -> PrepareEffectRequest<'_> {
    let quality = DYNAMIC_EQ_DESCRIPTOR
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == RATE)
        .expect("launch rate");
    PrepareEffectRequest {
        sample_rate: RATE,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::Unconnected {
                id: PortId::new("sidechain-in").expect("port"),
                required: false,
            },
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 32,
        },
        tail_bound: conformance::tail_bound_of(
            Box::new(DynamicEqFactory),
            RATE,
            EffectQuality::Normal,
        ),
    }
}

pub fn prepare(values: &Values, link_mode: LinkMode) -> PreparedEffect {
    DynamicEqFactory
        .prepare(request(values, link_mode))
        .expect("prepared dynamic EQ")
}

/// A bank at this build's width, one parameter set and link mode per lane.
pub fn prepare_bank(values: &[Values], links: &[LinkMode]) -> PreparedEffectBank {
    let requests: Vec<_> = values
        .iter()
        .zip(links)
        .map(|(set, &link)| request(set, link))
        .collect();
    DynamicEqFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::current(),
            width: native_width(),
            requests: &requests,
            active_mask: native_width().full_mask(),
        })
        .expect("bank binding")
        .expect("a bank at the native width")
}

/// One block through a scalar instance.
pub fn process(
    effect: &mut PreparedEffect,
    left: &mut [f32],
    right: &mut [f32],
    first_sample: u64,
    spans: &[PreparedAutomationSpan],
) -> ProcessReport {
    effect.processor.process(
        EffectProcessBlock::new(left, right, None, first_sample, spans, 128).expect("block"),
    )
}

/// One block through a bank, dual or collapsed.
pub fn process_bank(
    bank: &mut PreparedEffectBank,
    left: &mut [f32],
    right: &mut [f32],
    first_sample: u64,
    spans: &[PreparedAutomationSpan],
    offsets: &[u32],
    mono: bool,
) -> effect_contract::BankProcessReport {
    let frames = (left.len() / NATIVE_LANES) as u32;
    let block = EffectBankProcessBlock::new(
        left,
        right,
        None,
        frames,
        native_width(),
        first_sample,
        spans,
        offsets,
        128,
    )
    .expect("bank block");
    if mono {
        bank.processor.process_bank_mono(block)
    } else {
        bank.processor.process_bank(block)
    }
}

/// One track's state payload from a bank.
pub fn snapshot_bank(bank: &PreparedEffectBank, track: u32) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let sizes = bank.metadata.program_key.state_sizes;
    let mut sections = (
        vec![0; sizes.common_bytes as usize],
        vec![0; sizes.left_bytes as usize],
        vec![0; sizes.right_bytes as usize],
    );
    bank.processor
        .snapshot_track_state_payload(
            track,
            StatePayloadOutput::new(&mut sections.0, &mut sections.1, &mut sections.2, sizes)
                .expect("sizes"),
        )
        .expect("snapshot");
    sections
}

/// A deterministic bipolar noise signal.
pub fn noise(seed: u64, frames: usize, amplitude: f32) -> Vec<f32> {
    let mut state = seed | 1;
    (0..frames)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            ((state >> 40) as f32 / 8_388_608.0 - 1.0) * amplitude
        })
        .collect()
}

/// `amplitude sin(2 pi f n / fs)`.
pub fn tone(frequency: f64, n: usize, amplitude: f64) -> f32 {
    (amplitude * math::sin(core::f64::consts::TAU * frequency * n as f64 / f64::from(RATE))) as f32
}

/// One span at `sample` on both channels.
pub fn spans_both(parameter: u32, value: f32, sample: u64) -> [PreparedAutomationSpan; 2] {
    [ParameterChannel::Left, ParameterChannel::Right].map(|channel| PreparedAutomationSpan {
        channel,
        parameter_index: parameter,
        sample,
        value,
    })
}

/// Fields of one band, in descriptor order.
pub const ENABLED: usize = 0;
pub const FREQUENCY: usize = 1;
pub const Q: usize = 2;
pub const GAIN: usize = 3;
pub const THRESHOLD: usize = 4;
pub const RATIO: usize = 5;
pub const RANGE: usize = 6;
pub const ATTACK: usize = 7;
pub const RELEASE: usize = 8;
pub const DIRECTION: usize = 9;
pub const DETECTOR: usize = 10;
pub const KNEE: usize = 11;

/// The descriptor index of `field` of `band`.
pub const fn index(band: usize, field: usize) -> usize {
    band * 12 + field
}

/// Sets `field` of `band` on both channels.
pub fn set_band(values: &mut Values, band: usize, field: usize, value: f32) {
    set(values, index(band, field), value);
}

/// The defaults with only `band` enabled.
pub fn only(band: usize) -> Values {
    let mut values = defaults();
    for other in (0..4).filter(|&other| other != band) {
        set_band(&mut values, other, ENABLED, 0.0);
    }
    values
}

/// A scalar instance with the sidechain connected.
pub fn prepare_connected(values: &Values) -> PreparedEffect {
    let mut request = request(values, LinkMode::DualMono);
    request.ports = PreparedPorts {
        sidechain: PreparedSidechainPort::Connected {
            id: PortId::new("sidechain-in").expect("port"),
            required: false,
        },
    };
    DynamicEqFactory.prepare(request).expect("connected")
}
