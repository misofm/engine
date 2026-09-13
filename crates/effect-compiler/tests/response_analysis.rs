#![allow(missing_docs)]

use effect_compiler::launch_native_effect_registry;
use effect_contract::*;

#[test]
fn launch_registry_discovers_eq_response_without_effect_crate_import() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let factory = registry
        .get_ascii("miso.parametric-eq")
        .expect("parametric EQ is in the launch registry");
    let response = factory.response_analysis().expect("EQ response capability");
    let descriptor = response.analysis_descriptor();
    assert_eq!(descriptor.id, 1);
    assert_eq!(descriptor.axis_unit, ParameterUnit::Hz);
    assert_eq!(descriptor.unit, ParameterUnit::Db);
    assert_eq!(descriptor.cost, ObservationCost::Computed);
    assert_eq!(
        descriptor.mode,
        ResponseAnalysisMode::RequestedConfiguration
    );
    assert_eq!(descriptor.cadence, ResponseQueryCadence::ExplicitQuery);
    assert_eq!(descriptor.sections.len(), 4);
    assert_eq!(
        descriptor
            .sections
            .iter()
            .map(|section| section.id)
            .collect::<Vec<_>>(),
        [1, 2, 3, 4]
    );

    let initial_values: Vec<_> = default_initial_values(factory.descriptor()).collect();
    let request = PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: &initial_values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: u64::MAX,
            maximum_scratch_bytes: u64::MAX,
            maximum_automation_spans_per_block: u32::MAX,
        },
    };
    let prepared = response
        .prepare_response(
            request,
            ResponsePrepareLimits {
                maximum_prepared_bytes: usize::MAX,
            },
        )
        .expect("response preparation");
    let configuration = prepared.configuration();
    assert_eq!(configuration.sample_rate_hz, 48_000);
    assert_eq!(configuration.enabled_left.len(), 4);
    assert_eq!(configuration.enabled_right.len(), 4);
    assert_eq!(configuration.bypass, Some(false));

    let frequencies = [0.0, 1_000.0, 24_000.0];
    let mut left = [f32::NAN; 3];
    let mut right = [f32::NAN; 3];
    let summary = prepared
        .query_into(
            ResponseQuery {
                configuration_id: u64::MAX,
                frequencies_hz: &frequencies,
                maximum_points: frequencies.len(),
            },
            ResponseOutput {
                total_left_db: &mut left,
                total_right_db: &mut right,
                sections_left_db: None,
                sections_right_db: None,
            },
        )
        .expect("response query");
    assert_eq!(summary.configuration_id, u64::MAX);
    assert_eq!(summary.mode, ResponseAnalysisMode::RequestedConfiguration);
    assert_eq!(summary.points, frequencies.len());
    assert!(left.iter().all(|value| value.is_finite()));
    assert!(right.iter().all(|value| value.is_finite()));

    let compressor = registry
        .get_ascii("miso.compressor")
        .expect("compressor is in the launch registry");
    assert!(compressor.response_analysis().is_none());
}
