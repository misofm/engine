//! Transactional native-session preparation coverage.

use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use effect_contract::*;
use session::{
    CompileCaps, EffectParam, ParameterChannel, ParameterUnit as SessionParameterUnit,
    compile_session, parse_session_json,
};

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
const QUALITIES: [QualityDescriptor; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
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
        model.tracks[0].inserts.effects[0].params[0].value = 25.0 + seed as f32;
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

#[test]
fn retired_gate_parameter_id_eight_rejects_before_native_publication() {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/observation-frame-shape.json"
    ))
    .expect("actual gate session fixture");
    let gate = model
        .tracks
        .iter_mut()
        .flat_map(|track| track.inserts.effects.iter_mut())
        .find(|effect| effect.id.as_str() == "gate")
        .expect("fixture gate effect");
    gate.params.push(EffectParam {
        parameter_id: 8,
        channel: ParameterChannel::Both,
        unit: SessionParameterUnit::Milliseconds,
        value: 2.0,
    });
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
    .expect("session model remains structurally compilable");
    let registry = launch_native_effect_registry().expect("launch registry");
    let diagnostics = match prepare_native_session_effects(&compiled, &registry, caps()) {
        Ok(_) => panic!("retired gate parameter must prevent prepared publication"),
        Err(diagnostics) => diagnostics,
    };
    assert_eq!(diagnostics.0.len(), 1);
    assert_eq!(diagnostics.0[0].code, "effect.parameter.unknown");
    assert!(diagnostics.0[0].path.contains("gate"));
}

#[test]
fn retired_compressor_parameter_id_eight_rejects_before_native_publication() {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/compressor-dynamic-observation.json"
    ))
    .expect("causal compressor fixture");
    model.tracks[0].inserts.effects[0].params.push(EffectParam {
        parameter_id: 8,
        channel: ParameterChannel::Both,
        unit: SessionParameterUnit::Milliseconds,
        value: 5.0,
    });
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
    .expect("session model remains structurally compilable");
    let registry = launch_native_effect_registry().expect("launch registry");
    let diagnostics = match prepare_native_session_effects(&compiled, &registry, caps()) {
        Ok(_) => panic!("retired parameter must prevent prepared publication"),
        Err(diagnostics) => diagnostics,
    };
    assert_eq!(diagnostics.0.len(), 1);
    assert_eq!(diagnostics.0[0].code, "effect.parameter.unknown");
    assert!(diagnostics.0[0].path.contains("comp0"));
}

#[test]
fn retired_multiband_parameter_id_two_rejects_before_native_publication() {
    let mut model = parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
        .expect("canonical session fixture");
    {
        let effect = &mut model.tracks[0].inserts.effects[0];
        effect.identity = session::EffectIdentity::Native {
            effect_id: session::StableId::parse("miso.multiband-compressor")
                .expect("multiband effect ID"),
        };
        effect.params = vec![EffectParam {
            parameter_id: 1,
            channel: ParameterChannel::Both,
            unit: SessionParameterUnit::Hz,
            value: 1_000.0,
        }];
    }
    let valid_session = compile_session(
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
    .expect("multiband session remains structurally compilable");
    let registry = launch_native_effect_registry().expect("launch registry");
    let prepared = prepare_native_session_effects(&valid_session, &registry, caps())
        .expect("current multiband parameter set prepares");
    assert_eq!(prepared.entries.len(), 1);

    model.tracks[0].inserts.effects[0].params.push(EffectParam {
        parameter_id: 2,
        channel: ParameterChannel::Both,
        unit: SessionParameterUnit::Hz,
        value: 1_000.0,
    });
    let rejected_session = compile_session(
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
    .expect("retired ID does not change session structure");
    let diagnostics = match prepare_native_session_effects(&rejected_session, &registry, caps()) {
        Ok(_) => panic!("retired multiband parameter must prevent prepared publication"),
        Err(diagnostics) => diagnostics,
    };
    assert_eq!(diagnostics.0.len(), 1);
    assert_eq!(diagnostics.0[0].code, "effect.parameter.unknown");
    assert!(diagnostics.0[0].path.contains("eq"));
}

/// Decision 12's eligibility list, enforced where native identities resolve. Every listed effect
/// prepares as a console slot on the launch registry; the delay, the multiband compressor (until
/// #1069) and an identity the registry does not carry are each refused with
/// `console.slot.ineligible_effect` at the slot's identity. A check that looked at the registry
/// instead of the list would admit the delay; a check keyed on the rack rather than the console
/// would refuse the same effect as an insert, which the last assertion rules out.
#[test]
fn console_slots_accept_exactly_the_eligible_native_effects() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let generous = EffectCompileCaps {
        maximum_total_state_bytes: 1 << 30,
        maximum_scratch_bytes: 1 << 30,
        maximum_automation_spans_per_block: 32,
    };
    let compiled = |effect_id: &str, post: bool| {
        let mut model =
            parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
                .expect("canonical session fixture");
        model.tracks[0].inserts.effects.clear();
        model.automation.clear();
        let slot = session::ConsoleSlot {
            slot: session::StableId::parse("desk").expect("slot"),
            identity: session::EffectIdentity::Native {
                effect_id: session::StableId::parse(effect_id).expect("effect ID"),
            },
            quality: session::EffectQuality::Normal,
            link_mode: session::LinkMode::DualMono,
        };
        if post {
            model.console.post_insert.push(slot);
        } else {
            model.console.pre_insert.push(slot);
        }
        model.tracks[0].console.push(session::ConsoleEntry {
            slot: session::StableId::parse("desk").expect("slot"),
            bypass: false,
            params: Vec::new(),
        });
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
        .expect("the console session compiles")
    };
    for (index, effect_id) in effect_compiler::CONSOLE_ELIGIBLE_EFFECTS.iter().enumerate() {
        let session = compiled(effect_id, index % 2 == 1);
        let prepared = prepare_native_session_effects(&session, &registry, generous)
            .unwrap_or_else(|diagnostics| panic!("{effect_id}: {:?}", diagnostics.0));
        assert_eq!(prepared.entries.len(), 1, "{effect_id} prepares one slot");
    }
    for (effect_id, post, section) in [
        ("miso.delay", false, "pre_insert"),
        ("miso.multiband-compressor", true, "post_insert"),
        ("parametric-eq", false, "pre_insert"),
    ] {
        let session = compiled(effect_id, post);
        let diagnostics = match prepare_native_session_effects(&session, &registry, generous) {
            Ok(_) => panic!("{effect_id} must not prepare as a console slot"),
            Err(diagnostics) => diagnostics,
        };
        assert!(
            diagnostics.0.iter().any(|diagnostic| {
                diagnostic.code == "console.slot.ineligible_effect"
                    && diagnostic.path == format!("$.console.{section}[slot=desk].identity")
            }),
            "{effect_id}: {:?}",
            diagnostics.0
        );
    }

    // The same delay as an insert prepares: eligibility is a console rule, not an effect ban.
    let mut model = parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
        .expect("canonical session fixture");
    model.automation.clear();
    model.tracks[0].inserts.effects[0].identity = session::EffectIdentity::Native {
        effect_id: session::StableId::parse("miso.delay").expect("delay ID"),
    };
    model.tracks[0].inserts.effects[0].params.clear();
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
    .expect("the insert session compiles");
    prepare_native_session_effects(&session, &registry, generous)
        .unwrap_or_else(|diagnostics| panic!("a delay insert prepares: {:?}", diagnostics.0));
}

/// The 64-track console with track 0's EQ and limiter bypassed, and a delay inserted on tracks 0
/// (bypassed) and 1 (enabled).
fn bypassed_console() -> session::CompiledSession {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/console-sixty-four-track-intended.json"
    ))
    .expect("console fixture");
    let comp = model
        .console
        .pre_insert
        .iter()
        .find(|slot| slot.slot.as_str() == "comp")
        .expect("the compressor slot")
        .clone();
    for (index, bypass) in [(0, true), (1, false)] {
        let delay = session::Effect {
            id: session::StableId::parse("delay").expect("id"),
            identity: session::EffectIdentity::Native {
                effect_id: session::StableId::parse("miso.delay").expect("id"),
            },
            quality: comp.quality,
            bypass,
            link_mode: comp.link_mode,
            params: Vec::new(),
            sidechain: session::SidechainDeclaration::None,
        };
        let mut multiband = delay.clone();
        multiband.id = session::StableId::parse("multiband").expect("id");
        multiband.identity = session::EffectIdentity::Native {
            effect_id: session::StableId::parse("miso.multiband-compressor").expect("id"),
        };
        let track = &mut model.tracks[index];
        track.inserts.effects.push(delay);
        track.inserts.effects.push(multiband);
    }
    for entry in &mut model.tracks[0].console {
        if matches!(entry.slot.as_str(), "eq" | "limiter") {
            entry.bypass = true;
        }
    }
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
    .expect("compiled console")
}

fn entry<'a>(
    prepared: &'a effect_compiler::EffectPreparedSession,
    track: &str,
    effect: &str,
) -> &'a effect_compiler::EffectPreparedEntry {
    prepared
        .entries
        .iter()
        .find(|entry| entry.track_id == track && entry.effect_id == effect)
        .unwrap_or_else(|| panic!("{track} {effect} prepared"))
}

/// Issue #1087: a session bypass lowers to an enabled effect plus a bypassed channel-less lane,
/// for every effect that can bank, so a bypassed instance shares its enabled neighbours' program
/// key. The delay never banks and keeps its prepared bypass, and since #1100 so does the
/// multiband, whose bank recovery is still whole-bank.
///
/// Red mutations: prepare with `bypass: effect.bypass` again -> the bypassed EQ's program key
/// differs from the enabled one's; drop the channel-less lane -> nothing carries the bypass to the
/// rack's shunt; lower the delay or the multiband too -> its prepared bypass is gone.
#[test]
fn a_session_bypass_lowers_to_an_enabled_effect_and_a_bypassed_lane() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let prepared =
        prepare_native_session_effects(&bypassed_console(), &registry, caps()).expect("prepared");
    for effect in ["eq", "limiter"] {
        let bypassed = entry(&prepared, "ch00", effect);
        let enabled = entry(&prepared, "ch01", effect);
        assert!(bypassed.initial_bypass && !enabled.initial_bypass);
        assert!(!bypassed.metadata.bypass && !bypassed.bank_preparation.bypass);
        assert_eq!(
            bypassed.metadata.program_key(),
            enabled.metadata.program_key(),
            "{effect}: a bypassed and an enabled instance share one program"
        );
        let lane = bypassed
            .control
            .as_deref()
            .expect("the bypass rides a lane");
        assert!(!lane.has_channel() && lane.bypassed());
        assert!(
            enabled.control.is_none(),
            "an enabled instance holds no lane"
        );
    }
    let delay = entry(&prepared, "ch00", "delay");
    assert!(delay.initial_bypass && delay.metadata.bypass && delay.bank_preparation.bypass);
    assert!(
        delay.control.is_none(),
        "the delay keeps its prepared bypass"
    );
    assert!(!entry(&prepared, "ch01", "delay").metadata.bypass);
    assert_eq!(effect_compiler::NEVER_BANKED_EFFECTS, ["miso.delay"]);
    let multiband = entry(&prepared, "ch00", "multiband");
    assert!(
        multiband.initial_bypass && multiband.metadata.bypass && multiband.bank_preparation.bypass,
        "the multiband keeps its prepared bypass (#1100)"
    );
    assert!(
        multiband.control.is_none(),
        "no lane carries a multiband bypass"
    );
    assert!(!entry(&prepared, "ch01", "multiband").metadata.bypass);
    assert_eq!(
        effect_compiler::PREPARED_BYPASS_EFFECTS,
        ["miso.multiband-compressor"]
    );
    let lowered: Vec<&str> = registry
        .descriptors()
        .map(|descriptor| descriptor.id.as_str())
        .filter(|id| effect_compiler::lowers_session_bypass(id))
        .collect();
    assert_eq!(
        lowered,
        [
            "miso.compressor",
            "miso.gate-expander",
            "miso.parametric-eq",
            "miso.soft-clip",
            "miso.transient-shaper",
            "miso.true-peak-limiter",
        ],
        "every launch effect but the delay and the multiband lowers its session bypass"
    );
}

/// Issue #1087: a live-control lane starts from the session bypass, not the prepared one.
///
/// Red mutation: seed the live lane from `bank_preparation.bypass` again -> the bypassed EQ's
/// live lane starts un-bypassed and the track renders wet from its first block.
#[test]
fn a_live_control_lane_starts_from_the_session_bypass() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut prepared =
        prepare_native_session_effects(&bypassed_console(), &registry, caps()).expect("prepared");
    let producers = effect_compiler::attach_effect_live_controls(
        &mut prepared,
        core::num::NonZeroUsize::new(8).expect("depth"),
    )
    .expect("live controls attached");
    assert_eq!(producers.len(), prepared.entries.len());
    for (track, effect, bypassed) in [
        ("ch00", "eq", true),
        ("ch00", "comp", false),
        ("ch00", "limiter", true),
        ("ch00", "delay", true),
        ("ch00", "multiband", true),
        ("ch01", "eq", false),
        ("ch01", "delay", false),
        ("ch01", "multiband", false),
    ] {
        let lane = entry(&prepared, track, effect)
            .control
            .as_deref()
            .expect("every instance has a live lane");
        assert!(lane.has_channel(), "{track} {effect}: a live channel");
        assert_eq!(
            lane.bypassed(),
            bypassed,
            "{track} {effect}: starts bypassed"
        );
    }
}
