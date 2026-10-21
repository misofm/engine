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
    name: "gain_db",
    display_unit: "dB",
    unit: ParameterUnit::Db,
    domain: ParameterDomain::Continuous,
    minimum: Some(-24.0),
    maximum: Some(24.0),
    default_value: 0.0,
    mapping: ParameterMapping::Linear,
    // `canonical.json` automates this parameter, and preparation accepts only a `Block` target
    // (decision 15 E1, #1335 D1).
    automation_rate: AutomationRate::Block,
    channel_policy: ParameterChannelPolicy::Shared,
    smoothing: SmoothingRule::Linear,
    smoothing_samples: 8,
    readable: true,
    automatable: true,
    enum_choices: &[],
    lattice: effect_contract::ParameterLattice::arithmetic(0.1, 1),
    prepared_reason: None,
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
    tail_and_rest: |_, _| NodeTailBound {
        tail: TailSamples::Finite(7),
        tail_every_peak: TailSamples::Infinite,
        rest: RestBound::Unstated,
        composition: CompositionBound::Unstated,
    },
    observations: &[],
};

struct Factory(&'static EffectDescriptor);
impl NativeEffectFactory for Factory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        self.0
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        Ok(PreparedEffect {
            processor: Box::new(Processor),
            metadata: expected_prepared_metadata(self.0, request)?,
        })
    }
    fn bind_homogeneous_bank(
        &self,
        _: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
        Ok(None)
    }
}
/// The double's processor reads no prepared value, so it holds none (issue #1461).
struct Processor;
impl PreparedNativeEffect for Processor {
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

fn compile_model(
    model: &session::SessionModel,
) -> Result<session::CompiledSession, session::DiagnosticSet> {
    compile_session(
        model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
}

fn compiled() -> session::CompiledSession {
    let model =
        parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json")).unwrap();
    compile_model(&model).unwrap()
}
fn caps() -> EffectCompileCaps {
    EffectCompileCaps {
        maximum_total_state_bytes: 1 << 20,
        maximum_scratch_bytes: 1 << 20,
        maximum_automation_spans_per_block: 32,
    }
}

#[test]
fn preparation_is_complete_sorted_and_preserves_cached_metadata() {
    let registry =
        NativeEffectRegistry::new([Box::new(Factory(&DESCRIPTOR)) as Box<dyn NativeEffectFactory>])
            .unwrap();
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
        NativeEffectRegistry::new([Box::new(Factory(&DESCRIPTOR)) as Box<dyn NativeEffectFactory>])
            .unwrap();
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
fn out_of_domain_session_parameters_reject_transactionally_without_panic() {
    let registry =
        NativeEffectRegistry::new([Box::new(Factory(&DESCRIPTOR)) as Box<dyn NativeEffectFactory>])
            .unwrap();
    let source = include_str!("../../../fixtures/session/v1/canonical.json");
    for value in [f32::from_bits(24.0_f32.to_bits() + 1), 25.0, f32::MAX] {
        let mut model = parse_session_json(source).unwrap();
        model.tracks[0].inserts.effects[0].params[0].value = value;
        let compiled = compile_model(&model).unwrap();
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

/// A retired parameter ID on a launch effect is refused with `effect.parameter.unknown` before
/// native publication: the gate's ID 8, the compressor's ID 8 and the multiband's ID 2. The
/// multiband row first proves its current parameter set prepares.
#[test]
fn retired_parameter_ids_reject_before_native_publication() {
    type Select = fn(&mut session::SessionModel) -> &mut session::Effect;
    let gate: Select = |model| {
        model
            .tracks
            .iter_mut()
            .flat_map(|track| track.inserts.effects.iter_mut())
            .find(|effect| effect.id.as_str() == "gate")
            .expect("fixture gate effect")
    };
    let compressor: Select = |model| &mut model.tracks[0].inserts.effects[0];
    let multiband: Select = |model| {
        // The fixture's automation would now target the multiband's prepared crossover (#1335 D1).
        model.automation.clear();
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
        effect
    };
    let registry = launch_native_effect_registry().expect("launch registry");
    for (source, select, parameter_id, unit, value, path, control) in [
        (
            include_str!("../../../fixtures/session/v1/observation-frame-shape.json"),
            gate,
            8,
            SessionParameterUnit::Milliseconds,
            2.0,
            "gate",
            false,
        ),
        (
            include_str!("../../../fixtures/session/v1/compressor-dynamic-observation.json"),
            compressor,
            8,
            SessionParameterUnit::Milliseconds,
            5.0,
            "comp0",
            false,
        ),
        (
            include_str!("../../../fixtures/session/v1/canonical.json"),
            multiband,
            2,
            SessionParameterUnit::Hz,
            1_000.0,
            "eq",
            true,
        ),
    ] {
        let mut model = parse_session_json(source).expect("session fixture");
        select(&mut model);
        if control {
            let valid = compile_model(&model).expect("the current parameter set compiles");
            let prepared = prepare_native_session_effects(&valid, registry, caps())
                .expect("the current parameter set prepares");
            assert_eq!(prepared.entries.len(), 1, "{path}");
        }
        select(&mut model).params.push(EffectParam {
            parameter_id,
            channel: ParameterChannel::Both,
            unit,
            value,
        });
        let compiled = compile_model(&model).expect("a retired ID keeps the session compilable");
        let diagnostics = match prepare_native_session_effects(&compiled, registry, caps()) {
            Ok(_) => panic!("{path}: a retired parameter must prevent prepared publication"),
            Err(diagnostics) => diagnostics,
        };
        assert_eq!(diagnostics.0.len(), 1, "{path}");
        assert_eq!(diagnostics.0[0].code, "effect.parameter.unknown", "{path}");
        assert!(diagnostics.0[0].path.contains(path), "{path}");
    }
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
        compile_model(&model).expect("the console session compiles")
    };
    for (index, effect_id) in effect_compiler::CONSOLE_ELIGIBLE_EFFECTS.iter().enumerate() {
        let session = compiled(effect_id, index % 2 == 1);
        let prepared = prepare_native_session_effects(&session, registry, generous)
            .unwrap_or_else(|diagnostics| panic!("{effect_id}: {:?}", diagnostics.0));
        assert_eq!(prepared.entries.len(), 1, "{effect_id} prepares one slot");
    }
    for (effect_id, post, section) in [
        ("miso.delay", false, "pre_insert"),
        ("miso.multiband-compressor", true, "post_insert"),
        ("parametric-eq", false, "pre_insert"),
    ] {
        let session = compiled(effect_id, post);
        let diagnostics = match prepare_native_session_effects(&session, registry, generous) {
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
    let session = compile_model(&model).expect("the insert session compiles");
    prepare_native_session_effects(&session, registry, generous)
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
    compile_model(&model).expect("compiled console")
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
        prepare_native_session_effects(&bypassed_console(), registry, caps()).expect("prepared");
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
        prepare_native_session_effects(&bypassed_console(), registry, caps()).expect("prepared");
    let producers = effect_compiler::attach_effect_live_controls(
        &mut prepared,
        &engine::realtime::LiveValuesSuperseded::new(),
        0,
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

/// A one-insert session of the launch effect `effect_id` at its default parameters.
fn insert_session(effect_id: &str) -> session::CompiledSession {
    let mut model = parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
        .expect("canonical session fixture");
    model.automation.clear();
    model.tracks[0].inserts.effects[0].identity = session::EffectIdentity::Native {
        effect_id: session::StableId::parse(effect_id).expect("effect ID"),
    };
    model.tracks[0].inserts.effects[0].params.clear();
    compile_model(&model).expect("the insert session compiles")
}

/// Decision 15, D15-2 and #1345 D4: whatever automation capacity the caller asks for, every
/// launch effect is prepared with a window for every one of its live parameter cells, so a block
/// stages every cell it reads and none is deferred or dropped. The bound depends on the descriptor
/// alone, so a larger request stays and a live lane changes no program key.
///
/// Mutation (PR evidence): pass the caller's capacity through unraised -> every effect with more
/// live cells than one is red.
#[test]
fn every_launch_effect_is_prepared_with_a_window_for_all_its_live_cells() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let small = EffectCompileCaps {
        maximum_automation_spans_per_block: 1,
        ..caps()
    };
    let large = EffectCompileCaps {
        maximum_automation_spans_per_block: 4_096,
        ..caps()
    };
    let mut checked = 0;
    let mut with_many_cells = 0;
    for descriptor in registry.descriptors() {
        let id = descriptor.id.as_str();
        let cells = EffectLaneShape::live_parameter_cells(descriptor);
        assert!(cells > 0, "{id}: every launch effect has a live parameter");
        let session = insert_session(id);
        let prepared = prepare_native_session_effects(&session, registry, small)
            .unwrap_or_else(|diagnostics| panic!("{id}: {:?}", diagnostics.0));
        let capacity = prepared.entries[0].metadata.automation_capacity as usize;
        assert_eq!(
            capacity, cells,
            "{id}: a request of 1 is raised to the live cells"
        );
        let mut attached =
            prepare_native_session_effects(&session, registry, small).expect("prepared again");
        let mut producers = effect_compiler::attach_effect_live_controls(
            &mut attached,
            &engine::realtime::LiveValuesSuperseded::new(),
            0,
        )
        .unwrap_or_else(|diagnostics| panic!("{id}: {:?}", diagnostics.0));
        let lane = attached.entries[0].control.as_deref().expect("a live lane");
        assert_eq!(
            lane.check_window(capacity),
            Ok(()),
            "{id}: a window of the prepared capacity (here the lane's cells) holds the lane's cells"
        );
        // One transaction writes every live parameter cell of the instance (the two channels of a
        // per-lane parameter apart, so no pair folds into fewer spans), and one block, staging into
        // a window of exactly the lane's live cells (which the prepared capacity equals here),
        // applies every one of them.
        let (gate, mut pin) = engine::realtime::RevisionGate::pair(0);
        let stamp = pin.stamp(1).expect("a legal revision");
        let mut written = 0;
        for (index, parameter) in descriptor.parameters.iter().enumerate() {
            if !parameter.automatable || parameter.automation_rate != AutomationRate::Block {
                continue;
            }
            let channels: &[effect_contract::ParameterChannel] = match parameter.channel_policy {
                ParameterChannelPolicy::Shared => &[effect_contract::ParameterChannel::Both],
                ParameterChannelPolicy::PerLane => &[
                    effect_contract::ParameterChannel::Left,
                    effect_contract::ParameterChannel::Right,
                ],
            };
            for (offset, channel) in channels.iter().enumerate() {
                // The EQ's parameters ride its prepared targets: its lane holds no parameter cell.
                if lane.parameter_cells() == 0 {
                    continue;
                }
                let value = parameter.default_value + 0.5 * offset as f32;
                producers[0]
                    .write(
                        stamp,
                        EffectControlRecord::Parameter {
                            parameter_index: index as u32,
                            channel: *channel,
                            value,
                        },
                    )
                    .unwrap_or_else(|refusal| panic!("{id}: parameter {index}: {refusal:?}"));
                written += 1;
            }
        }
        pin.publish(&gate, 1);
        assert_eq!(
            written,
            if lane.parameter_cells() == 0 {
                0
            } else {
                cells
            }
        );
        let lane = attached.entries[0]
            .control
            .as_deref_mut()
            .expect("a live lane");
        let mut window = vec![
            PreparedAutomationSpan {
                channel: effect_contract::ParameterChannel::Both,
                parameter_index: u32::MAX,
                sample: 0,
                value: 0.0,
            };
            capacity
        ];
        let staged = lane.stage(&mut window, 0, gate.snapshot());
        assert_eq!(
            staged.staged, written,
            "{id}: every cell written in one transaction is staged in the one block that reads it"
        );
        let bigger = prepare_native_session_effects(&session, registry, large)
            .unwrap_or_else(|diagnostics| panic!("{id}: {:?}", diagnostics.0));
        assert_eq!(
            bigger.entries[0].metadata.automation_capacity, 4_096,
            "{id}: a larger request is kept"
        );
        checked += 1;
        with_many_cells += usize::from(cells > 1);
    }
    assert_eq!(checked, 8, "the eight launch effects");
    assert!(with_many_cells > 0);
}

/// An instance whose automation capacity is below its lane's live cells is refused at attach, not
/// discovered by a render block. Preparation makes this unreachable; a hand-built entry reaches
/// it.
#[test]
fn an_entry_whose_capacity_is_below_its_live_cells_is_refused_at_attach() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut prepared =
        prepare_native_session_effects(&insert_session("miso.compressor"), registry, caps())
            .expect("prepared");
    prepared.entries[0].metadata.automation_capacity = 1;
    let refusal = effect_compiler::attach_effect_live_controls(
        &mut prepared,
        &engine::realtime::LiveValuesSuperseded::new(),
        0,
    )
    .err()
    .expect("a window of one span cannot hold the compressor's cells");
    assert_eq!(refusal.0[0].code, "effect.control.capacity");
    assert!(prepared.entries[0].control.is_none());
}
