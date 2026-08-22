//! Rust-side half of the Issue 081 native C/Rust ABI qualification matrix.

use core::mem::{align_of, offset_of, size_of};

use miso_engine_effect_package::{
    EffectDescriptorEnumChoiceRecordV1, EffectDescriptorParameterRecordV1,
    EffectDescriptorPortRecordV1, EffectDescriptorQualityRecordV1, EffectDescriptorSummaryV1,
    EffectDescriptorWireDiagnosticV1, EffectPackageDiagnosticV1, EffectStateDiagnosticV1,
};

fn assert_offsets<T>(actual: &[usize], expected: &[usize]) {
    assert_eq!(actual, expected);
    assert!(!core::any::type_name::<T>().is_empty());
}

#[test]
fn six_descriptor_c_records_have_exact_rust_layouts_and_offsets() {
    assert_eq!(
        (
            size_of::<EffectDescriptorParameterRecordV1>(),
            align_of::<EffectDescriptorParameterRecordV1>()
        ),
        (80, 4)
    );
    assert_offsets::<EffectDescriptorParameterRecordV1>(
        &[
            offset_of!(EffectDescriptorParameterRecordV1, id),
            offset_of!(EffectDescriptorParameterRecordV1, unit),
            offset_of!(EffectDescriptorParameterRecordV1, domain),
            offset_of!(EffectDescriptorParameterRecordV1, mapping),
            offset_of!(EffectDescriptorParameterRecordV1, automation_rate),
            offset_of!(EffectDescriptorParameterRecordV1, channel_policy),
            offset_of!(EffectDescriptorParameterRecordV1, smoothing),
            offset_of!(EffectDescriptorParameterRecordV1, smoothing_samples),
            offset_of!(EffectDescriptorParameterRecordV1, flags),
            offset_of!(EffectDescriptorParameterRecordV1, minimum_bits),
            offset_of!(EffectDescriptorParameterRecordV1, maximum_bits),
            offset_of!(EffectDescriptorParameterRecordV1, default_bits),
            offset_of!(EffectDescriptorParameterRecordV1, enum_start),
            offset_of!(EffectDescriptorParameterRecordV1, enum_count),
            offset_of!(EffectDescriptorParameterRecordV1, display_name_offset),
            offset_of!(EffectDescriptorParameterRecordV1, display_name_length),
            offset_of!(EffectDescriptorParameterRecordV1, display_unit_offset),
            offset_of!(EffectDescriptorParameterRecordV1, display_unit_length),
            offset_of!(EffectDescriptorParameterRecordV1, reserved0),
            offset_of!(EffectDescriptorParameterRecordV1, reserved1),
        ],
        &(0..20).map(|index| index * 4).collect::<Vec<_>>(),
    );

    assert_eq!(
        (
            size_of::<EffectDescriptorPortRecordV1>(),
            align_of::<EffectDescriptorPortRecordV1>()
        ),
        (24, 4)
    );
    assert_offsets::<EffectDescriptorPortRecordV1>(
        &[
            offset_of!(EffectDescriptorPortRecordV1, id_offset),
            offset_of!(EffectDescriptorPortRecordV1, id_length),
            offset_of!(EffectDescriptorPortRecordV1, role),
            offset_of!(EffectDescriptorPortRecordV1, required),
            offset_of!(EffectDescriptorPortRecordV1, layout),
            offset_of!(EffectDescriptorPortRecordV1, reserved),
        ],
        &[0, 4, 8, 12, 16, 20],
    );

    assert_eq!(
        (
            size_of::<EffectDescriptorQualityRecordV1>(),
            align_of::<EffectDescriptorQualityRecordV1>()
        ),
        (64, 8)
    );
    assert_offsets::<EffectDescriptorQualityRecordV1>(
        &[
            offset_of!(EffectDescriptorQualityRecordV1, quality),
            offset_of!(EffectDescriptorQualityRecordV1, sample_rate),
            offset_of!(EffectDescriptorQualityRecordV1, latency_samples),
            offset_of!(EffectDescriptorQualityRecordV1, tail_kind),
            offset_of!(EffectDescriptorQualityRecordV1, reserved0),
            offset_of!(EffectDescriptorQualityRecordV1, tail_samples),
            offset_of!(EffectDescriptorQualityRecordV1, common_state_bytes),
            offset_of!(EffectDescriptorQualityRecordV1, left_state_bytes),
            offset_of!(EffectDescriptorQualityRecordV1, right_state_bytes),
            offset_of!(EffectDescriptorQualityRecordV1, reserved1),
            offset_of!(EffectDescriptorQualityRecordV1, scratch_fixed_bytes),
            offset_of!(EffectDescriptorQualityRecordV1, scratch_bytes_per_frame),
        ],
        &[0, 4, 8, 16, 20, 24, 32, 36, 40, 44, 48, 56],
    );

    assert_eq!(
        (
            size_of::<EffectDescriptorEnumChoiceRecordV1>(),
            align_of::<EffectDescriptorEnumChoiceRecordV1>()
        ),
        (16, 4)
    );
    assert_offsets::<EffectDescriptorEnumChoiceRecordV1>(
        &[
            offset_of!(EffectDescriptorEnumChoiceRecordV1, value_bits),
            offset_of!(EffectDescriptorEnumChoiceRecordV1, label_offset),
            offset_of!(EffectDescriptorEnumChoiceRecordV1, label_length),
            offset_of!(EffectDescriptorEnumChoiceRecordV1, reserved),
        ],
        &[0, 4, 8, 12],
    );

    assert_eq!(
        (
            size_of::<EffectDescriptorSummaryV1>(),
            align_of::<EffectDescriptorSummaryV1>()
        ),
        (64, 4)
    );
    assert_offsets::<EffectDescriptorSummaryV1>(
        &[
            offset_of!(EffectDescriptorSummaryV1, abi_version),
            offset_of!(EffectDescriptorSummaryV1, total_bytes),
            offset_of!(EffectDescriptorSummaryV1, parameter_count),
            offset_of!(EffectDescriptorSummaryV1, port_count),
            offset_of!(EffectDescriptorSummaryV1, quality_count),
            offset_of!(EffectDescriptorSummaryV1, enum_choice_count),
            offset_of!(EffectDescriptorSummaryV1, state_layout_version),
            offset_of!(EffectDescriptorSummaryV1, supported_link_mode_bits),
            offset_of!(EffectDescriptorSummaryV1, identity),
        ],
        &[0, 4, 8, 12, 16, 20, 24, 28, 32],
    );

    assert_eq!(
        (
            size_of::<EffectDescriptorWireDiagnosticV1>(),
            align_of::<EffectDescriptorWireDiagnosticV1>()
        ),
        (16, 4)
    );
    assert_offsets::<EffectDescriptorWireDiagnosticV1>(
        &[
            offset_of!(EffectDescriptorWireDiagnosticV1, code),
            offset_of!(EffectDescriptorWireDiagnosticV1, byte_offset),
            offset_of!(EffectDescriptorWireDiagnosticV1, record_index),
            offset_of!(EffectDescriptorWireDiagnosticV1, required_bytes),
        ],
        &[0, 4, 8, 12],
    );
}

#[test]
fn rust_only_package_and_state_diagnostics_retain_exact_c_layouts() {
    assert_eq!(
        (
            size_of::<EffectPackageDiagnosticV1>(),
            align_of::<EffectPackageDiagnosticV1>()
        ),
        (32, 8)
    );
    assert_eq!(
        (
            size_of::<EffectStateDiagnosticV1>(),
            align_of::<EffectStateDiagnosticV1>()
        ),
        (32, 8)
    );
    assert_eq!(offset_of!(EffectPackageDiagnosticV1, required_bytes), 24);
    assert_eq!(offset_of!(EffectStateDiagnosticV1, required_bytes), 24);
}
