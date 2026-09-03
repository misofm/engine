//! Transactional native-session preparation coverage.

use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use effect_contract::*;
use session::{CompileCaps, compile_session, parse_session_json};

const EFFECT_ID: EffectId = match EffectId::new("parametric-eq") {
    Ok(v) => v,
    Err(_) => panic!("id"),
};
const MAIN_IN: PortId = match PortId::new("main-in") {
    Ok(v) => v,
    Err(_) => panic!("id"),
};
const MAIN_OUT: PortId = match PortId::new("main-out") {
    Ok(v) => v,
    Err(_) => panic!("id"),
};
const PARAMETERS: [ParameterDescriptor; 1] = [ParameterDescriptor {
    id: ParameterId(1),
    display_name: "Gain",
    display_unit: "dB",
    unit: ParameterUnit::Db,
    domain: ParameterDomain::Continuous,
    minimum: Some(-24.0),
    maximum: Some(24.0),
    default_value: 0.0,
    mapping: ParameterMapping::Linear,
    automation_rate: AutomationRate::Sample,
    channel_policy: ParameterChannelPolicy::Shared,
    smoothing: SmoothingRule::Linear,
    smoothing_samples: 8,
    readable: true,
    automatable: true,
    enum_choices: &[],
    lattice: effect_contract::ParameterLattice::arithmetic(0.1, 1),
}];
const PORTS: [PortDescriptor; 2] = [
    PortDescriptor {
        id: MAIN_IN,
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
        id: MAIN_OUT,
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
];
const fn quality(sample_rate: u32) -> QualityDescriptor {
    QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(7),
        tail: TailSamples::Finite(7),
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: 0,
            right_bytes: 0,
        },
        scratch_fixed_bytes: 16,
        scratch_bytes_per_frame: 1,
    }
}
const QUALITIES: [QualityDescriptor; 8] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
    quality(176_400),
    quality(192_000),
    quality(352_800),
    quality(384_000),
];
static DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: EFFECT_ID,
    display_name: "Test EQ",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &[],
};

struct Factory;
impl NativeEffectFactory for Factory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &DESCRIPTOR
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        Ok(Box::new(Processor {
            metadata: expected_prepared_metadata(&DESCRIPTOR, request)?,
        }))
    }
    fn bind_homogeneous_bank(
        &self,
        _: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        Ok(None)
    }
}
struct Processor {
    metadata: PreparedEffectMetadata,
}
impl PreparedNativeEffect for Processor {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }
    fn reset(&mut self, _: ResetKind) {}
    fn process(&mut self, _: EffectProcessBlock<'_>) -> ProcessReport {
        ProcessReport::default()
    }
    fn snapshot_state_payload(&self, _: StatePayloadOutput<'_>) -> Result<(), StatePayloadError> {
        Ok(())
    }
    fn restore_state_payload(
        &mut self,
        version: u32,
        _: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if version == 1 {
            Ok(())
        } else {
            Err(StatePayloadError {
                code: "effect.state.version",
            })
        }
    }
}

fn compiled() -> session::CompiledSession {
    let model =
        parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json")).unwrap();
    compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .unwrap()
}
fn caps() -> EffectCompileCaps {
    EffectCompileCaps {
        maximum_total_state_bytes: 1 << 20,
        maximum_scratch_bytes: 1 << 20,
        maximum_automation_spans_per_block: 32,
    }
}

#[test]
fn launch_registry_prepares_the_accepted_nine_track_parametric_eq_fixture() {
    let model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.json"
    ))
    .expect("accepted fixture");
    assert_eq!(model.tracks.len(), 9);
    let session = compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("compiled fixture");
    let registry = launch_native_effect_registry().expect("launch registry");
    assert_eq!(registry.len(), 8);
    assert!(registry.get_ascii("miso.parametric-eq").is_some());
    assert!(registry.get_ascii("miso.compressor").is_some());
    assert!(registry.get_ascii("miso.gate-expander").is_some());
    assert!(registry.get_ascii("miso.multiband-compressor").is_some());
    assert!(registry.get_ascii("miso.true-peak-limiter").is_some());
    assert!(registry.get_ascii("miso.soft-clip").is_some());
    assert!(registry.get_ascii("miso.transient-shaper").is_some());
    assert!(registry.get_ascii("miso.delay").is_some());
    let prepared = prepare_native_session_effects(&session, &registry, caps()).expect("prepared");
    assert_eq!(prepared.entries.len(), 9);
}

#[test]
fn preparation_is_complete_sorted_and_preserves_cached_metadata() {
    let registry =
        NativeEffectRegistry::new([Box::new(Factory) as Box<dyn NativeEffectFactory>]).unwrap();
    let prepared = prepare_native_session_effects(&compiled(), &registry, caps()).unwrap();
    assert_eq!(prepared.entries.len(), 1);
    assert_eq!(prepared.entries[0].metadata.latency, LatencySamples(7));
    assert_eq!(prepared.entries[0].metadata.scratch_bytes, 144);
    assert_eq!(
        prepared.session.canonical_json(),
        compiled().canonical_json()
    );
}

#[test]
fn unavailable_factory_and_resource_caps_return_no_partial_session() {
    let empty = NativeEffectRegistry::default();
    let diagnostics = prepare_native_session_effects(&compiled(), &empty, caps())
        .err()
        .unwrap();
    assert_eq!(diagnostics.0[0].code, "effect.native.unavailable");
    let registry =
        NativeEffectRegistry::new([Box::new(Factory) as Box<dyn NativeEffectFactory>]).unwrap();
    let diagnostics = prepare_native_session_effects(
        &compiled(),
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: 1,
            maximum_scratch_bytes: 1,
            maximum_automation_spans_per_block: 1,
        },
    )
    .err()
    .unwrap();
    assert_eq!(diagnostics.0[0].code, "effect.resource.limit");
}

#[test]
fn ten_thousand_session_parameter_mutations_reject_transactionally_without_panic() {
    let registry =
        NativeEffectRegistry::new([Box::new(Factory) as Box<dyn NativeEffectFactory>]).unwrap();
    let source = include_str!("../../../fixtures/session/v1/canonical.json");
    for seed in 0..10_000_u32 {
        let mut model = parse_session_json(source).unwrap();
        model.tracks[0].dynamic.effects[0].params[0].value = 25.0 + seed as f32;
        let compiled = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .unwrap();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            prepare_native_session_effects(&compiled, &registry, caps())
        }));
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().err().unwrap().0[0].code,
            "effect.parameter.domain"
        );
    }
}
