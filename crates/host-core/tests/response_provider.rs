#![allow(missing_docs)]

use std::sync::Mutex;

use bench_support::alloc as bench_alloc;
use effect_compiler::{EffectCompileCaps, EffectPreparedSession, EffectRack};
use effect_contract::{
    ResponseAnalysisError, ResponseOutput, ResponsePrepareLimits, ResponseQuery,
};
use host_core::{
    PreparedSessionResponseCatalog, RequestedResponseIdentity, ResponseAvailability,
    ResponseFrequencyGrid, ResponseTarget, SessionResponseError, SessionResponseLimits,
    SessionResponseProvider, SessionResponseSelection, describe_session_response_target,
};
use math::{exp, log};
use session::{CompileCaps, EffectParam, ParameterChannel, ParameterUnit, StableId};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json");
const NONPROVIDER_SESSION: &str =
    include_str!("../../../fixtures/session/v1/compressor-dynamic-observation.json");
static ALLOCATION_LOCK: Mutex<()> = Mutex::new(());

fn limits() -> SessionResponseLimits {
    SessionResponseLimits {
        maximum_bindings: 64,
        maximum_points_per_binding: 256,
        maximum_total_grid_points: 4_096,
        maximum_retained_bytes: usize::MAX,
        maximum_single_allocation_bytes: usize::MAX,
    }
}

fn compile_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn effect_caps() -> EffectCompileCaps {
    EffectCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_scratch_bytes: u64::MAX,
        maximum_automation_spans_per_block: u32::MAX,
    }
}

fn prepared_session(rate: u32, all_racks: bool, asymmetric: bool) -> EffectPreparedSession {
    prepared_session_with_bypass(rate, all_racks, asymmetric, false)
}

fn prepared_session_with_bypass(
    rate: u32,
    all_racks: bool,
    asymmetric: bool,
    bypass: bool,
) -> EffectPreparedSession {
    let mut model = session::parse_session_json(SESSION).expect("fixture parses");
    model.sample_rate_hz = rate;
    let track = model
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == "eq0")
        .expect("fixture track");
    let effect = track.simd1.effects.pop().expect("fixture EQ");
    track.dynamic.effects.push(effect);
    track.dynamic.effects[0].bypass = bypass;
    if asymmetric {
        track.builtins.left.hpf_hz = 100.0;
        track.builtins.right.hpf_hz = 300.0;
        track.builtins.left.lpf_hz = 12_000.0;
        track.builtins.right.lpf_hz = 16_000.0;
        let effect = track.dynamic.effects.first_mut().expect("fixture EQ");
        effect.params.retain(|parameter| {
            !(parameter.parameter_id == 4 && parameter.channel == ParameterChannel::Both)
        });
        effect.params.extend([
            EffectParam {
                parameter_id: 4,
                channel: ParameterChannel::Left,
                unit: ParameterUnit::Db,
                value: 6.0,
            },
            EffectParam {
                parameter_id: 4,
                channel: ParameterChannel::Right,
                unit: ParameterUnit::Db,
                value: -6.0,
            },
        ]);
    }
    if all_racks {
        let mut simd1 = track.dynamic.effects[0].clone();
        simd1.id = StableId::parse("eq-simd1").expect("stable simd1 id");
        let mut simd2 = track.dynamic.effects[0].clone();
        simd2.id = StableId::parse("eq-simd2").expect("stable simd2 id");
        track.simd1.effects.push(simd1);
        track.simd2.effects.push(simd2);
    }
    let compiled = session::compile_session(&model, compile_caps()).expect("session compiles");
    let registry = effect_compiler::launch_native_effect_registry().expect("launch registry");
    effect_compiler::prepare_native_session_effects(&compiled, &registry, effect_caps())
        .expect("effects prepare")
}

fn prepared_nonprovider_session() -> EffectPreparedSession {
    let model = session::parse_session_json(NONPROVIDER_SESSION).expect("compressor fixture");
    let compiled = session::compile_session(&model, compile_caps()).expect("session compiles");
    let registry = effect_compiler::launch_native_effect_registry().expect("launch registry");
    effect_compiler::prepare_native_session_effects(&compiled, &registry, effect_caps())
        .expect("effects prepare")
}

fn stable(value: &str) -> StableId {
    StableId::parse(value).expect("stable id")
}

fn input_selection(rate: u32) -> SessionResponseSelection {
    SessionResponseSelection {
        target: ResponseTarget::InputFilters {
            track_id: stable("eq0"),
        },
        analysis_id: 1,
        grid: ResponseFrequencyGrid::Linear {
            points: 5,
            minimum_hz: 0.0,
            maximum_hz: rate as f32 * 0.5,
        },
    }
}

fn effect_selection(rack: EffectRack, slot: &str, rate: u32) -> SessionResponseSelection {
    SessionResponseSelection {
        target: ResponseTarget::Effect {
            track_id: stable("eq0"),
            rack,
            effect_slot_id: stable(slot),
        },
        analysis_id: 1,
        grid: ResponseFrequencyGrid::Logarithmic {
            points: 7,
            minimum_hz: 20.0,
            maximum_hz: rate as f32 * 0.5,
        },
    }
}

fn provider(
    effects: &EffectPreparedSession,
    selections: &[SessionResponseSelection],
) -> SessionResponseProvider {
    SessionResponseProvider::new(
        PreparedSessionResponseCatalog::prepare(
            effects,
            RequestedResponseIdentity {
                configuration_id: 9_007_199_254_740_993,
                plan_id: Some(u64::MAX),
            },
            selections,
            limits(),
        )
        .expect("catalog prepares"),
    )
}

#[test]
fn target_resolution_uses_stable_tuples() {
    let effects = prepared_session(48_000, true, false);
    let targets = [
        ResponseTarget::Effect {
            track_id: stable("eq0"),
            rack: EffectRack::Simd1,
            effect_slot_id: stable("eq-simd1"),
        },
        ResponseTarget::Effect {
            track_id: stable("eq0"),
            rack: EffectRack::Dynamic,
            effect_slot_id: stable("eq"),
        },
        ResponseTarget::Effect {
            track_id: stable("eq0"),
            rack: EffectRack::Simd2,
            effect_slot_id: stable("eq-simd2"),
        },
        ResponseTarget::InputFilters {
            track_id: stable("eq0"),
        },
    ];
    for target in &targets {
        assert!(matches!(
            describe_session_response_target(&effects, target),
            Ok(ResponseAvailability::Available { .. })
        ));
    }
    assert_eq!(
        describe_session_response_target(
            &effects,
            &ResponseTarget::InputFilters {
                track_id: stable("missing"),
            }
        ),
        Err(SessionResponseError::UnknownTrack)
    );
    assert_eq!(
        describe_session_response_target(
            &effects,
            &ResponseTarget::Effect {
                track_id: stable("eq0"),
                rack: EffectRack::Simd1,
                effect_slot_id: stable("missing"),
            }
        ),
        Err(SessionResponseError::UnknownEffectSlot)
    );
    let duplicate = vec![
        effect_selection(EffectRack::Dynamic, "eq", 48_000),
        effect_selection(EffectRack::Dynamic, "eq", 48_000),
    ];
    assert_eq!(
        PreparedSessionResponseCatalog::prepare(&effects, identity(), &duplicate, limits()).err(),
        Some(SessionResponseError::DuplicateSelection)
    );
    let unknown_analysis = SessionResponseSelection {
        target: ResponseTarget::InputFilters {
            track_id: stable("eq0"),
        },
        analysis_id: 99,
        grid: ResponseFrequencyGrid::Linear {
            points: 2,
            minimum_hz: 0.0,
            maximum_hz: 24_000.0,
        },
    };
    assert_eq!(
        PreparedSessionResponseCatalog::prepare(
            &effects,
            identity(),
            &[unknown_analysis],
            limits()
        )
        .err(),
        Some(SessionResponseError::UnknownAnalysis)
    );
    assert_eq!(
        describe_session_response_target(
            &effects,
            &ResponseTarget::Effect {
                track_id: stable("eq0"),
                rack: EffectRack::Simd1,
                effect_slot_id: stable("eq"),
            }
        ),
        Err(SessionResponseError::UnknownEffectSlot)
    );

    let unsupported = prepared_nonprovider_session();
    assert_eq!(
        describe_session_response_target(
            &unsupported,
            &ResponseTarget::Effect {
                track_id: stable("comp0"),
                rack: EffectRack::Dynamic,
                effect_slot_id: stable("comp"),
            }
        ),
        Err(SessionResponseError::UnsupportedResponse)
    );

    let mut malformed = prepared_session(48_000, false, false);
    malformed
        .entries
        .iter_mut()
        .find(|entry| entry.rack == EffectRack::Dynamic)
        .expect("dynamic entry")
        .metadata
        .sample_rate = 44_100;
    assert_eq!(
        describe_session_response_target(
            &malformed,
            &ResponseTarget::Effect {
                track_id: stable("eq0"),
                rack: EffectRack::Dynamic,
                effect_slot_id: stable("eq"),
            }
        ),
        Err(SessionResponseError::PreparedEntryMismatch)
    );
}

fn identity() -> RequestedResponseIdentity {
    RequestedResponseIdentity {
        configuration_id: 9_007_199_254_740_993,
        plan_id: Some(u64::MAX),
    }
}

#[test]
fn owner_query_parity_at_launch_rates() {
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let effects = prepared_session_with_bypass(rate, false, true, rate == 96_000);
        let selections = [
            input_selection(rate),
            effect_selection(EffectRack::Dynamic, "eq", rate),
        ];
        let provider = provider(&effects, &selections);
        for view in provider.bindings() {
            let frequencies = view.frequencies_hz.to_vec();
            let mut left = vec![f32::NAN; frequencies.len()];
            let mut right = vec![f32::NAN; frequencies.len()];
            let mut sections_left =
                vec![f32::NAN; view.descriptor.sections.len() * frequencies.len()];
            let mut sections_right =
                vec![f32::NAN; view.descriptor.sections.len() * frequencies.len()];
            let summary = provider
                .query_into(
                    view.handle,
                    ResponseOutput {
                        total_left_db: &mut left,
                        total_right_db: &mut right,
                        sections_left_db: Some(&mut sections_left),
                        sections_right_db: Some(&mut sections_right),
                    },
                )
                .expect("provider query");
            assert_eq!(summary.identity, identity());
            assert_eq!(
                summary.summary.configuration_id,
                identity().configuration_id
            );
            assert_eq!(summary.summary.sample_rate_hz, rate);
            assert!(left.iter().all(|value| value.is_finite()));
            assert!(right.iter().all(|value| value.is_finite()));
            assert_eq!(view.configuration.sample_rate_hz, rate);
            match view.target {
                host_core::ResponseTargetView::InputFilters { .. } => {
                    assert!(
                        view.configuration
                            .enabled_left
                            .iter()
                            .any(|enabled| *enabled)
                    );
                    assert!(
                        view.configuration
                            .enabled_right
                            .iter()
                            .any(|enabled| *enabled)
                    );
                    assert_eq!(view.configuration.bypass, None);
                }
                host_core::ResponseTargetView::Effect { .. } => {
                    assert_eq!(view.configuration.bypass, Some(rate == 96_000));
                    if rate == 96_000 {
                        assert!(left.iter().all(|value| *value == 0.0));
                        assert!(right.iter().all(|value| *value == 0.0));
                    }
                }
            }

            let (
                mut direct_left,
                mut direct_right,
                mut direct_sections_left,
                mut direct_sections_right,
            ) = (
                vec![f32::NAN; frequencies.len()],
                vec![f32::NAN; frequencies.len()],
                vec![f32::NAN; sections_left.len()],
                vec![f32::NAN; sections_right.len()],
            );
            let direct = match view.target {
                host_core::ResponseTargetView::InputFilters { .. } => None,
                host_core::ResponseTargetView::Effect {
                    rack,
                    effect_slot_id,
                    ..
                } => Some(
                    effects
                        .entries
                        .iter()
                        .find(|entry| entry.rack == rack && entry.effect_id == effect_slot_id)
                        .expect("direct effect entry")
                        .factory
                        .response_analysis()
                        .expect("direct response provider"),
                ),
            };
            if let Some(factory) = direct {
                let prepared = factory
                    .prepare_response(
                        effects
                            .entries
                            .iter()
                            .find(|entry| {
                                entry.rack
                                    == match view.target {
                                        host_core::ResponseTargetView::Effect { rack, .. } => rack,
                                        host_core::ResponseTargetView::InputFilters { .. } => {
                                            unreachable!()
                                        }
                                    }
                                    && entry.effect_id
                                        == match view.target {
                                            host_core::ResponseTargetView::Effect {
                                                effect_slot_id,
                                                ..
                                            } => effect_slot_id,
                                            host_core::ResponseTargetView::InputFilters {
                                                ..
                                            } => {
                                                unreachable!()
                                            }
                                        }
                            })
                            .expect("direct entry")
                            .bank_preparation
                            .request(),
                        ResponsePrepareLimits {
                            maximum_prepared_bytes: usize::MAX,
                        },
                    )
                    .expect("direct prepare");
                prepared
                    .query_into(
                        ResponseQuery {
                            configuration_id: identity().configuration_id,
                            frequencies_hz: &frequencies,
                            maximum_points: frequencies.len(),
                        },
                        ResponseOutput {
                            total_left_db: &mut direct_left,
                            total_right_db: &mut direct_right,
                            sections_left_db: Some(&mut direct_sections_left),
                            sections_right_db: Some(&mut direct_sections_right),
                        },
                    )
                    .expect("direct query");
            } else {
                let track = effects
                    .session
                    .normalized_model()
                    .tracks
                    .iter()
                    .find(|track| track.id.as_str() == "eq0")
                    .expect("direct track");
                let parameters =
                    builtins_compiler::requested_track_builtin_parameters(track, u32::MAX)
                        .expect("builtin projection");
                let prepared = builtins::prepare_input_filter_response(
                    rate,
                    parameters,
                    ResponsePrepareLimits {
                        maximum_prepared_bytes: usize::MAX,
                    },
                )
                .expect("direct builtin prepare");
                prepared
                    .query_into(
                        ResponseQuery {
                            configuration_id: identity().configuration_id,
                            frequencies_hz: &frequencies,
                            maximum_points: frequencies.len(),
                        },
                        ResponseOutput {
                            total_left_db: &mut direct_left,
                            total_right_db: &mut direct_right,
                            sections_left_db: Some(&mut direct_sections_left),
                            sections_right_db: Some(&mut direct_sections_right),
                        },
                    )
                    .expect("direct query");
            }
            assert_eq!(left, direct_left);
            assert_eq!(right, direct_right);
            assert_eq!(sections_left, direct_sections_left);
            assert_eq!(sections_right, direct_sections_right);

            let mut total_only_left = vec![f32::NAN; frequencies.len()];
            let mut total_only_right = vec![f32::NAN; frequencies.len()];
            provider
                .query_into(
                    view.handle,
                    ResponseOutput {
                        total_left_db: &mut total_only_left,
                        total_right_db: &mut total_only_right,
                        sections_left_db: None,
                        sections_right_db: None,
                    },
                )
                .expect("total-only query");
            assert_eq!(total_only_left, left);
            assert_eq!(total_only_right, right);

            let mut left_sections_only = vec![f32::NAN; sections_left.len()];
            let mut left_only_total = vec![f32::NAN; frequencies.len()];
            let mut left_only_right = vec![f32::NAN; frequencies.len()];
            provider
                .query_into(
                    view.handle,
                    ResponseOutput {
                        total_left_db: &mut left_only_total,
                        total_right_db: &mut left_only_right,
                        sections_left_db: Some(&mut left_sections_only),
                        sections_right_db: None,
                    },
                )
                .expect("left-section-only query");
            assert_eq!(left_only_total, left);
            assert_eq!(left_only_right, right);

            let mut right_sections_only = vec![f32::NAN; sections_right.len()];
            let mut right_only_total = vec![f32::NAN; frequencies.len()];
            let mut right_only_left = vec![f32::NAN; frequencies.len()];
            provider
                .query_into(
                    view.handle,
                    ResponseOutput {
                        total_left_db: &mut right_only_left,
                        total_right_db: &mut right_only_total,
                        sections_left_db: None,
                        sections_right_db: Some(&mut right_sections_only),
                    },
                )
                .expect("right-section-only query");
            assert_eq!(right_only_left, left);
            assert_eq!(right_only_total, right);
        }
    }
}

#[test]
fn frequency_grids_validate_before_publication() {
    let effects = prepared_session(48_000, false, false);
    let invalid = [
        ResponseFrequencyGrid::Linear {
            points: 0,
            minimum_hz: 0.0,
            maximum_hz: 1_000.0,
        },
        ResponseFrequencyGrid::Linear {
            points: 1,
            minimum_hz: 0.0,
            maximum_hz: 1_000.0,
        },
        ResponseFrequencyGrid::Linear {
            points: 2,
            minimum_hz: f32::NAN,
            maximum_hz: 1_000.0,
        },
        ResponseFrequencyGrid::Linear {
            points: 2,
            minimum_hz: 1_000.0,
            maximum_hz: 1_000.0,
        },
        ResponseFrequencyGrid::Linear {
            points: 2,
            minimum_hz: -1.0,
            maximum_hz: 1_000.0,
        },
        ResponseFrequencyGrid::Linear {
            points: 2,
            minimum_hz: 0.0,
            maximum_hz: 24_001.0,
        },
        ResponseFrequencyGrid::Logarithmic {
            points: 2,
            minimum_hz: 0.0,
            maximum_hz: 1_000.0,
        },
        ResponseFrequencyGrid::Linear {
            points: 3,
            minimum_hz: 1_000.0,
            maximum_hz: f32::from_bits(1_000.0_f32.to_bits() + 1),
        },
    ];
    for grid in invalid {
        let selection = SessionResponseSelection {
            target: ResponseTarget::InputFilters {
                track_id: stable("eq0"),
            },
            analysis_id: 1,
            grid,
        };
        assert!(matches!(
            PreparedSessionResponseCatalog::prepare(&effects, identity(), &[selection], limits()),
            Err(SessionResponseError::InvalidFrequencyGrid | SessionResponseError::Capacity)
        ));
    }
    let overflow = SessionResponseSelection {
        target: ResponseTarget::InputFilters {
            track_id: stable("eq0"),
        },
        analysis_id: 1,
        grid: ResponseFrequencyGrid::Linear {
            points: usize::MAX,
            minimum_hz: 0.0,
            maximum_hz: 24_000.0,
        },
    };
    let huge = SessionResponseLimits {
        maximum_bindings: 1,
        maximum_points_per_binding: usize::MAX,
        maximum_total_grid_points: usize::MAX,
        maximum_retained_bytes: usize::MAX,
        maximum_single_allocation_bytes: usize::MAX,
    };
    assert_eq!(
        PreparedSessionResponseCatalog::prepare(&effects, identity(), &[overflow], huge).err(),
        Some(SessionResponseError::Capacity)
    );
    let valid = input_selection(48_000);
    let catalog = PreparedSessionResponseCatalog::prepare(&effects, identity(), &[valid], limits())
        .expect("valid grid");
    let provider = SessionResponseProvider::new(catalog);
    let view = provider.bindings().next().expect("binding");
    assert_eq!(view.frequencies_hz.first(), Some(&0.0));
    assert_eq!(view.frequencies_hz.last(), Some(&24_000.0));
    assert!(view.frequencies_hz.windows(2).all(|pair| pair[1] > pair[0]));
    let logarithmic_two = SessionResponseSelection {
        target: ResponseTarget::InputFilters {
            track_id: stable("eq0"),
        },
        analysis_id: 1,
        grid: ResponseFrequencyGrid::Logarithmic {
            points: 2,
            minimum_hz: 20.0,
            maximum_hz: 24_000.0,
        },
    };
    let logarithmic =
        PreparedSessionResponseCatalog::prepare(&effects, identity(), &[logarithmic_two], limits())
            .expect("two-point logarithmic grid");
    let logarithmic_provider = SessionResponseProvider::new(logarithmic);
    let logarithmic_view = logarithmic_provider
        .bindings()
        .next()
        .expect("logarithmic binding");
    assert_eq!(logarithmic_view.frequencies_hz, &[20.0, 24_000.0]);

    let logarithmic_reference = SessionResponseSelection {
        target: ResponseTarget::InputFilters {
            track_id: stable("eq0"),
        },
        analysis_id: 1,
        grid: ResponseFrequencyGrid::Logarithmic {
            points: 7,
            minimum_hz: 20.0,
            maximum_hz: 20_000.0,
        },
    };
    let reference_catalog = PreparedSessionResponseCatalog::prepare(
        &effects,
        identity(),
        &[logarithmic_reference],
        limits(),
    )
    .expect("reference logarithmic grid");
    let reference_provider = SessionResponseProvider::new(reference_catalog);
    let reference_view = reference_provider
        .bindings()
        .next()
        .expect("reference logarithmic binding");
    let minimum = 20.0_f64;
    let maximum = 20_000.0_f64;
    let log_minimum = log(minimum);
    let log_span = log(maximum) - log_minimum;
    let expected = (0..7)
        .map(|index| exp(log_minimum + log_span * (index as f64 / 6.0)) as f32)
        .collect::<Vec<_>>();
    assert_eq!(reference_view.frequencies_hz, expected.as_slice());
}

#[test]
fn replacement_invalidates_previous_bindings() {
    let effects = prepared_session(48_000, false, false);
    let mut first = [input_selection(48_000)];
    let second = [effect_selection(EffectRack::Dynamic, "eq", 48_000)];
    let mut provider = provider(&effects, &first);
    first[0].analysis_id = 77;
    first[0].grid = ResponseFrequencyGrid::Linear {
        points: 2,
        minimum_hz: 100.0,
        maximum_hz: 200.0,
    };
    let old = {
        let retained = provider.bindings().next().expect("old binding");
        assert_eq!(retained.analysis_id, 1);
        assert_eq!(
            retained.frequencies_hz,
            &[0.0, 6_000.0, 12_000.0, 18_000.0, 24_000.0]
        );
        retained.handle
    };
    let candidate =
        PreparedSessionResponseCatalog::prepare(&effects, identity(), &second, limits())
            .expect("new catalog");
    provider.replace_catalog(candidate).expect("replacement");
    let mut left = [f32::from_bits(0x7fc0_1234); 7];
    let mut right = left;
    let before = left;
    assert_eq!(
        provider
            .query_into(
                old,
                ResponseOutput {
                    total_left_db: &mut left,
                    total_right_db: &mut right,
                    sections_left_db: None,
                    sections_right_db: None,
                },
            )
            .unwrap_err(),
        SessionResponseError::StaleHandle
    );
    assert!(
        left.iter()
            .zip(before)
            .all(|(actual, expected)| actual.to_bits() == expected.to_bits())
    );
    let new = provider.bindings().next().expect("new binding");
    assert_eq!(
        new.handle,
        provider.bindings().next().expect("same binding").handle
    );
    assert_eq!(
        new.target,
        host_core::ResponseTargetView::Effect {
            track_id: "eq0",
            rack: EffectRack::Dynamic,
            effect_slot_id: "eq",
        }
    );
    assert_eq!(new.configuration.sample_rate_hz, 48_000);

    let changed_effects = prepared_session_with_bypass(48_000, false, true, true);
    let changed_candidate =
        PreparedSessionResponseCatalog::prepare(&changed_effects, identity(), &second, limits())
            .expect("changed configuration catalog");
    provider
        .replace_catalog(changed_candidate)
        .expect("changed configuration replacement");
    let changed = provider.bindings().next().expect("changed binding");
    assert_eq!(changed.configuration.bypass, Some(true));
}

#[test]
fn retained_resources_enforce_exact_caps() {
    let effects = prepared_session(48_000, false, false);
    let selection = [input_selection(48_000)];
    let generous =
        PreparedSessionResponseCatalog::prepare(&effects, identity(), &selection, limits())
            .expect("generous catalog");
    let resources = generous.resources();
    assert!(resources.total_retained_bytes > 0);
    assert!(resources.largest_allocation_bytes > 0);
    let exact = SessionResponseLimits {
        maximum_bindings: resources.bindings,
        maximum_points_per_binding: 5,
        maximum_total_grid_points: resources.grid_points,
        maximum_retained_bytes: resources.total_retained_bytes,
        maximum_single_allocation_bytes: resources.largest_allocation_bytes,
    };
    assert!(
        PreparedSessionResponseCatalog::prepare(&effects, identity(), &selection, exact).is_ok()
    );
    let mut below_total = exact;
    below_total.maximum_retained_bytes -= 1;
    assert!(
        PreparedSessionResponseCatalog::prepare(&effects, identity(), &selection, below_total)
            .is_err()
    );
    let mut below_single = exact;
    below_single.maximum_single_allocation_bytes -= 1;
    assert!(
        PreparedSessionResponseCatalog::prepare(&effects, identity(), &selection, below_single)
            .is_err()
    );
    let empty = PreparedSessionResponseCatalog::prepare(
        &effects,
        identity(),
        &[],
        SessionResponseLimits {
            maximum_bindings: 0,
            maximum_points_per_binding: 0,
            maximum_total_grid_points: 0,
            maximum_retained_bytes: 0,
            maximum_single_allocation_bytes: 0,
        },
    )
    .expect("empty catalog under zero limits");
    assert_eq!(empty.resources().total_retained_bytes, 0);
}

#[test]
fn query_and_discovery_do_not_allocate() {
    let _guard = ALLOCATION_LOCK.lock().expect("allocation lock");
    bench_alloc::set_mode(bench_alloc::Mode::Count);
    bench_alloc::assert_installed();
    let effects = prepared_session(48_000, false, false);
    let selection = [input_selection(48_000)];
    let mut provider = provider(&effects, &selection);
    let view = provider.bindings().next().expect("binding");
    let discovery_mark = bench_alloc::current_thread_counters();
    let borrowed = provider.bindings().next().expect("borrowed binding");
    std::hint::black_box((
        borrowed.target,
        borrowed.frequencies_hz,
        borrowed.configuration,
    ));
    let discovery_delta = bench_alloc::current_thread_delta_since(discovery_mark);
    assert_eq!(discovery_delta.allocations, 0);
    assert_eq!(discovery_delta.deallocations, 0);
    assert_eq!(discovery_delta.reallocations, 0);

    let mut left = [f32::NAN; 5];
    let mut right = [f32::NAN; 5];
    let query_mark = bench_alloc::current_thread_counters();
    provider
        .query_into(
            view.handle,
            ResponseOutput {
                total_left_db: &mut left,
                total_right_db: &mut right,
                sections_left_db: None,
                sections_right_db: None,
            },
        )
        .expect("query");
    let query_delta = bench_alloc::current_thread_delta_since(query_mark);
    assert_eq!(query_delta.allocations, 0);
    assert_eq!(query_delta.deallocations, 0);
    assert_eq!(query_delta.reallocations, 0);

    let mut bad_left = [f32::from_bits(0x7fc0_1111); 4];
    let mut bad_right = bad_left;
    let before = bad_left;
    let refusal_mark = bench_alloc::current_thread_counters();
    assert!(matches!(
        provider.query_into(
            view.handle,
            ResponseOutput {
                total_left_db: &mut bad_left,
                total_right_db: &mut bad_right,
                sections_left_db: None,
                sections_right_db: None,
            },
        ),
        Err(SessionResponseError::Owner(
            ResponseAnalysisError::OutputShape
        ))
    ));
    let refusal_delta = bench_alloc::current_thread_delta_since(refusal_mark);
    assert_eq!(refusal_delta.allocations, 0);
    assert_eq!(refusal_delta.deallocations, 0);
    assert_eq!(refusal_delta.reallocations, 0);
    assert!(
        bad_left
            .iter()
            .zip(before)
            .all(|(actual, expected)| actual.to_bits() == expected.to_bits())
    );

    let stale = view.handle;
    let replacement = PreparedSessionResponseCatalog::prepare(
        &effects,
        identity(),
        &[effect_selection(EffectRack::Dynamic, "eq", 48_000)],
        limits(),
    )
    .expect("replacement catalog");
    provider.replace_catalog(replacement).expect("replacement");
    let stale_mark = bench_alloc::current_thread_counters();
    assert!(matches!(
        provider.query_into(
            stale,
            ResponseOutput {
                total_left_db: &mut bad_left,
                total_right_db: &mut bad_right,
                sections_left_db: None,
                sections_right_db: None,
            },
        ),
        Err(SessionResponseError::StaleHandle)
    ));
    let stale_delta = bench_alloc::current_thread_delta_since(stale_mark);
    assert_eq!(stale_delta.allocations, 0);
    assert_eq!(stale_delta.deallocations, 0);
    assert_eq!(stale_delta.reallocations, 0);
}
