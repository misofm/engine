//! The restore half of the channel-symmetry witness: a restored payload whose two channel
//! sections differ declines the instance until a reset.
//!
//! # Why the check is a byte comparison of the wire sections
//!
//! A state payload carries running state as well as designed words -- rings, cursors, integrators.
//! Two channels whose *parameters* agree but whose restored rings differ are not doing identical
//! work, and only a comparison that sees the whole section catches that. Every payload word is
//! little-endian and every `f32` is stored as its raw `to_bits`, so byte equality of the two
//! sections is exactly bitwise equality of the two channels' words, `-0.0` included. The two
//! sections are always the same length, so the comparison is a `memcmp` of two equal slices, off
//! the render thread, once per restore.
//!
//! Nothing rendered reads the result; it is control-plane state on the restored capability.

use miso_engine_effect_compiler::*;
use miso_engine_effect_contract::*;
use miso_engine_effect_package::*;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

const fn effect_id(value: &'static str) -> EffectId {
    match EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("effect id"),
    }
}
const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("port id"),
    }
}

const LANE_BYTES: usize = 4;

static PARAMETERS: [ParameterDescriptor; 1] = [ParameterDescriptor {
    id: ParameterId(1),
    display_name: "value",
    display_unit: "linear",
    unit: ParameterUnit::Linear,
    domain: ParameterDomain::Continuous,
    minimum: Some(-1.0),
    maximum: Some(1.0),
    default_value: 0.0,
    mapping: ParameterMapping::Linear,
    automation_rate: AutomationRate::Block,
    channel_policy: ParameterChannelPolicy::PerLane,
    smoothing: SmoothingRule::None,
    smoothing_samples: 0,
    readable: true,
    automatable: true,
    enum_choices: &[],
}];
static PORTS: [PortDescriptor; 2] = [
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
];
const fn quality(sample_rate: u32) -> QualityDescriptor {
    QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        tail: TailSamples::Finite(0),
        maximum_state: StatePayloadSizes {
            common_bytes: 1,
            left_bytes: LANE_BYTES as u32,
            right_bytes: LANE_BYTES as u32,
        },
        scratch_fixed_bytes: 2,
        scratch_bytes_per_frame: 0,
    }
}
static QUALITIES: [QualityDescriptor; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];
static DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("test.symmetry-restore"),
    display_name: "Symmetry restore",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &[],
};
static INITIAL: [InitialParameterValue; 2] = [
    InitialParameterValue {
        parameter_index: 0,
        channel: ParameterChannel::Left,
        value: 0.25,
    },
    InitialParameterValue {
        parameter_index: 0,
        channel: ParameterChannel::Right,
        value: 0.25,
    },
];

/// Whether the snapshot this factory's instances produce has equal channel sections.
#[derive(Default)]
struct Controls {
    asymmetric: AtomicBool,
}

struct MockFactory {
    controls: Arc<Controls>,
}

struct MockEffect {
    metadata: PreparedEffectMetadata,
    common: [u8; 1],
    left: [u8; LANE_BYTES],
    right: [u8; LANE_BYTES],
}

struct MockBank {
    metadata: PreparedBankMetadata,
    lanes: usize,
}

fn sections(asymmetric: bool) -> ([u8; LANE_BYTES], [u8; LANE_BYTES]) {
    let left = [0x21, 0x22, 0x23, 0x24];
    // One byte apart. The point of the test is that *one byte* is enough: a restore that leaves
    // the two channels one word apart is a restore a collapse would render wrong.
    let right = if asymmetric {
        [0x21, 0x22, 0x23, 0x25]
    } else {
        left
    };
    (left, right)
}

impl NativeEffectFactory for MockFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &DESCRIPTOR
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(&DESCRIPTOR, request)?;
        let (left, right) = sections(self.controls.asymmetric.load(Ordering::SeqCst));
        Ok(Box::new(MockEffect {
            metadata,
            common: [0x11],
            left,
            right,
        }))
    }
    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        let first = request.requests[0];
        let metadata = expected_prepared_metadata(&DESCRIPTOR, first)?;
        Ok(Some(Box::new(MockBank {
            metadata: PreparedBankMetadata {
                width: request.width,
                program_key: metadata.program_key(),
            },
            lanes: request.width.lanes() as usize,
        })))
    }
}

impl PreparedNativeEffect for MockEffect {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }
    fn reset(&mut self, _: ResetKind) {}
    fn process(&mut self, _: EffectProcessBlock<'_>) -> ProcessReport {
        ProcessReport::default()
    }
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        output.common.copy_from_slice(&self.common);
        output.left.copy_from_slice(&self.left);
        output.right.copy_from_slice(&self.right);
        Ok(())
    }
    fn restore_state_payload(
        &mut self,
        _version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.common.copy_from_slice(input.common);
        self.left.copy_from_slice(input.left);
        self.right.copy_from_slice(input.right);
        Ok(())
    }
    /// The designed-word half is deliberately *always* true here, so a decline can only have come
    /// from the restore comparison.
    fn channel_symmetry(&self) -> bool {
        true
    }
}

impl PreparedNativeEffectBank for MockBank {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }
    fn reset(&mut self, _: ResetKind) {}
    fn process_bank(&mut self, _: EffectBankProcessBlock<'_>) -> BankProcessReport {
        BankProcessReport::empty(self.metadata.width)
    }
    fn snapshot_track_state_payload(
        &self,
        _track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let (left, right) = sections(false);
        output.common.copy_from_slice(&[0x11]);
        output.left.copy_from_slice(&left);
        output.right.copy_from_slice(&right);
        Ok(())
    }
    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        _version: u32,
        _input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if track_index as usize >= self.lanes {
            return Err(StatePayloadError { code: "test.lane" });
        }
        Ok(())
    }
    fn lane_channel_symmetry(&self, _lane: usize) -> bool {
        true
    }
}

fn preparation(sample_rate: u32) -> EffectBankPreparation {
    EffectBankPreparation {
        sample_rate,
        quantum: 32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: INITIAL.into(),
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 16,
            maximum_scratch_bytes: 8,
            maximum_automation_spans_per_block: 7,
        },
    }
}

fn wire() -> Vec<u8> {
    let required = effect_descriptor_wire_required_size(&DESCRIPTOR, 1 << 20).unwrap();
    let mut output = vec![0; required as usize];
    encode_effect_descriptor_wire(&DESCRIPTOR, 1 << 20, &mut output).unwrap();
    output
}

fn bound<'a>(controls: &Arc<Controls>, wire: &'a [u8]) -> WireBoundNativeEffectFactory<'a> {
    let factory: Arc<dyn NativeEffectFactory> = Arc::new(MockFactory {
        controls: Arc::clone(controls),
    });
    bind_native_effect_factory_state(factory, wire, 1 << 20).unwrap()
}

fn admission(preparation: &EffectBankPreparation) -> EffectStateRestoreAdmission {
    EffectStateRestoreAdmission {
        sample_rate: preparation.sample_rate,
        quantum: preparation.quantum,
        maximum_total_state_bytes: preparation.limits.maximum_total_state_bytes,
        maximum_scratch_bytes: preparation.limits.maximum_scratch_bytes,
        maximum_automation_spans_per_block: preparation.limits.maximum_automation_spans_per_block,
    }
}

/// Snapshots one envelope whose channel sections agree, or differ by exactly one byte.
fn envelope(asymmetric: bool) -> Vec<u8> {
    let controls = Arc::new(Controls::default());
    controls.asymmetric.store(asymmetric, Ordering::SeqCst);
    let descriptor_wire = wire();
    let capability = bound(&controls, &descriptor_wire);
    let preparation = preparation(48_000);
    let processor = capability.factory().prepare(preparation.request()).unwrap();
    let requirements = scalar_effect_state_requirements(
        &capability,
        &preparation,
        EffectStateLimits::default(),
    )
    .unwrap();
    let mut scratch = vec![0; requirements.payload_snapshot_scratch_bytes as usize];
    let mut output = vec![0; requirements.envelope_bytes as usize];
    snapshot_scalar_effect_state(
        &capability,
        &preparation,
        processor.as_ref(),
        EffectStateLimits::default(),
        &mut scratch,
        &mut output,
    )
    .unwrap();
    output
}

/// Red mutation: replace the `payload_sections_agree(left, right)` call in
/// `restore_scalar_effect_state` with `true` -> the asymmetric case fails.
/// Replace it with `false` -> the symmetric case fails. The pair pins both directions.
#[test]
fn a_scalar_restore_declines_exactly_when_the_channel_sections_differ() {
    for asymmetric in [false, true] {
        let bytes = envelope(asymmetric);
        let controls = Arc::new(Controls::default());
        let descriptor_wire = wire();
        let capability = bound(&controls, &descriptor_wire);
        let preparation = preparation(48_000);
        let restored = restore_scalar_effect_state(
            capability,
            &bytes,
            EffectStateLimits::default(),
            admission(&preparation),
            &mut [InitialParameterValue {
                parameter_index: 0,
                channel: ParameterChannel::Both,
                value: 0.0,
            }; 8],
        )
        .expect("the restore itself succeeds either way");

        let witness = restored.channel_symmetry();
        assert!(
            witness.holds(ChannelSymmetryWitness::DESIGNED),
            "the designed term is unconditionally true for this mock, so it cannot mask the restore"
        );
        assert_eq!(
            witness.holds(ChannelSymmetryWitness::RESTORED),
            !asymmetric,
            "asymmetric = {asymmetric}"
        );
        assert_eq!(witness.eligible(), !asymmetric);
        if asymmetric {
            assert_eq!(
                witness.declined(),
                ChannelSymmetryWitness::RESTORED,
                "an unequal restore declines on RESTORED and nothing else"
            );
        }
    }
}

/// A restore that *succeeds* and whose sections agree must leave the instance eligible: the
/// negative test above is only meaningful if the positive one is not vacuous. That assertion is
/// inside the loop above; this one pins the second half of the rule -- the declining verdict is
/// per instance, and one instance's asymmetric restore says nothing about another's.
#[test]
fn one_declining_restore_does_not_contaminate_a_sibling() {
    let asymmetric = envelope(true);
    let symmetric = envelope(false);
    let descriptor_wire = wire();
    let preparation = preparation(48_000);

    let controls = Arc::new(Controls::default());
    let first = restore_scalar_effect_state(
        bound(&controls, &descriptor_wire),
        &asymmetric,
        EffectStateLimits::default(),
        admission(&preparation),
        &mut [InitialParameterValue {
            parameter_index: 0,
            channel: ParameterChannel::Both,
            value: 0.0,
        }; 8],
    )
    .unwrap();
    let second = restore_scalar_effect_state(
        bound(&controls, &descriptor_wire),
        &symmetric,
        EffectStateLimits::default(),
        admission(&preparation),
        &mut [InitialParameterValue {
            parameter_index: 0,
            channel: ParameterChannel::Both,
            value: 0.0,
        }; 8],
    )
    .unwrap();
    assert!(!first.channel_symmetry().eligible());
    assert!(second.channel_symmetry().eligible());
}

/// The bank path, lane by lane. Red mutation: write the verdict to `restored[0]` instead of
/// `restored[track_index]` -> the "exactly the restored lane declined" assertion fails.
#[test]
fn a_bank_restore_declines_exactly_the_lane_whose_sections_differ() {
    let descriptor_wire = wire();
    let preparation = preparation(48_000);
    let controls = Arc::new(Controls::default());
    let capability = bound(&controls, &descriptor_wire);
    let Some(width) = BankWidth::for_backend(miso_engine_lane::Backend::current()) else {
        // A backend with no bank width has no bank path to test.
        return;
    };
    let lanes = width.lanes() as usize;
    let replays: Box<[EffectBankPreparation]> = (0..lanes).map(|_| preparation.clone()).collect();
    let mut bank = prepare_unpublished_effect_bank_state(
        capability,
        miso_engine_lane::Backend::current(),
        width,
        replays,
        admission(&preparation),
    )
    .expect("bind");

    for lane in 0..lanes {
        assert!(
            bank.lane_channel_symmetry(lane).eligible(),
            "a freshly bound bank has restored nothing, so no lane has been contradicted"
        );
    }

    // Lane 1 gets the asymmetric envelope; every other lane gets the symmetric one.
    let asymmetric = envelope(true);
    let symmetric = envelope(false);
    for lane in 0..lanes {
        let bytes = if lane == 1 { &asymmetric } else { &symmetric };
        bank = restore_unpublished_effect_bank_track_state(
            bank,
            lane as u32,
            bytes,
            EffectStateLimits::default(),
            admission(&preparation),
        )
        .expect("restore");
    }

    for lane in 0..lanes {
        assert_eq!(
            bank.lane_channel_symmetry(lane).eligible(),
            lane != 1,
            "lane {lane}"
        );
    }
    assert_eq!(
        bank.lane_channel_symmetry(lanes).declined(),
        ChannelSymmetryWitness::from_terms(0).declined(),
        "a lane index the width does not have declines on every term"
    );
}
