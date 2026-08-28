//! Rust-side half of the Issue 081 native C/Rust ABI qualification matrix.

use core::mem::{align_of, offset_of, size_of};

use miso_engine_effect_package::{
    EffectDescriptorEnumChoiceRecord, EffectDescriptorParameterRecord, EffectDescriptorPortRecord,
    EffectDescriptorQualityRecord, EffectDescriptorSummary, EffectDescriptorWireDiagnostic,
    EffectPackageDiagnostic, EffectStateDiagnostic,
};

fn assert_offsets<T>(actual: &[usize], expected: &[usize]) {
    assert_eq!(actual, expected);
    assert!(!core::any::type_name::<T>().is_empty());
}

#[test]
fn six_descriptor_c_records_have_exact_rust_layouts_and_offsets() {
    assert_eq!(
        (
            size_of::<EffectDescriptorParameterRecord>(),
            align_of::<EffectDescriptorParameterRecord>()
        ),
        (80, 4)
    );
    assert_offsets::<EffectDescriptorParameterRecord>(
        &[
            offset_of!(EffectDescriptorParameterRecord, id),
            offset_of!(EffectDescriptorParameterRecord, unit),
            offset_of!(EffectDescriptorParameterRecord, domain),
            offset_of!(EffectDescriptorParameterRecord, mapping),
            offset_of!(EffectDescriptorParameterRecord, automation_rate),
            offset_of!(EffectDescriptorParameterRecord, channel_policy),
            offset_of!(EffectDescriptorParameterRecord, smoothing),
            offset_of!(EffectDescriptorParameterRecord, smoothing_samples),
            offset_of!(EffectDescriptorParameterRecord, flags),
            offset_of!(EffectDescriptorParameterRecord, minimum_bits),
            offset_of!(EffectDescriptorParameterRecord, maximum_bits),
            offset_of!(EffectDescriptorParameterRecord, default_bits),
            offset_of!(EffectDescriptorParameterRecord, enum_start),
            offset_of!(EffectDescriptorParameterRecord, enum_count),
            offset_of!(EffectDescriptorParameterRecord, display_name_offset),
            offset_of!(EffectDescriptorParameterRecord, display_name_length),
            offset_of!(EffectDescriptorParameterRecord, display_unit_offset),
            offset_of!(EffectDescriptorParameterRecord, display_unit_length),
            offset_of!(EffectDescriptorParameterRecord, reserved0),
            offset_of!(EffectDescriptorParameterRecord, reserved1),
        ],
        &(0..20).map(|index| index * 4).collect::<Vec<_>>(),
    );

    assert_eq!(
        (
            size_of::<EffectDescriptorPortRecord>(),
            align_of::<EffectDescriptorPortRecord>()
        ),
        (24, 4)
    );
    assert_offsets::<EffectDescriptorPortRecord>(
        &[
            offset_of!(EffectDescriptorPortRecord, id_offset),
            offset_of!(EffectDescriptorPortRecord, id_length),
            offset_of!(EffectDescriptorPortRecord, role),
            offset_of!(EffectDescriptorPortRecord, required),
            offset_of!(EffectDescriptorPortRecord, layout),
            offset_of!(EffectDescriptorPortRecord, reserved),
        ],
        &[0, 4, 8, 12, 16, 20],
    );

    assert_eq!(
        (
            size_of::<EffectDescriptorQualityRecord>(),
            align_of::<EffectDescriptorQualityRecord>()
        ),
        (64, 8)
    );
    assert_offsets::<EffectDescriptorQualityRecord>(
        &[
            offset_of!(EffectDescriptorQualityRecord, quality),
            offset_of!(EffectDescriptorQualityRecord, sample_rate),
            offset_of!(EffectDescriptorQualityRecord, latency_samples),
            offset_of!(EffectDescriptorQualityRecord, tail_kind),
            offset_of!(EffectDescriptorQualityRecord, reserved0),
            offset_of!(EffectDescriptorQualityRecord, tail_samples),
            offset_of!(EffectDescriptorQualityRecord, common_state_bytes),
            offset_of!(EffectDescriptorQualityRecord, left_state_bytes),
            offset_of!(EffectDescriptorQualityRecord, right_state_bytes),
            offset_of!(EffectDescriptorQualityRecord, reserved1),
            offset_of!(EffectDescriptorQualityRecord, scratch_fixed_bytes),
            offset_of!(EffectDescriptorQualityRecord, scratch_bytes_per_frame),
        ],
        &[0, 4, 8, 16, 20, 24, 32, 36, 40, 44, 48, 56],
    );

    assert_eq!(
        (
            size_of::<EffectDescriptorEnumChoiceRecord>(),
            align_of::<EffectDescriptorEnumChoiceRecord>()
        ),
        (16, 4)
    );
    assert_offsets::<EffectDescriptorEnumChoiceRecord>(
        &[
            offset_of!(EffectDescriptorEnumChoiceRecord, value_bits),
            offset_of!(EffectDescriptorEnumChoiceRecord, label_offset),
            offset_of!(EffectDescriptorEnumChoiceRecord, label_length),
            offset_of!(EffectDescriptorEnumChoiceRecord, reserved),
        ],
        &[0, 4, 8, 12],
    );

    assert_eq!(
        (
            size_of::<EffectDescriptorSummary>(),
            align_of::<EffectDescriptorSummary>()
        ),
        (64, 4)
    );
    assert_offsets::<EffectDescriptorSummary>(
        &[
            offset_of!(EffectDescriptorSummary, abi_version),
            offset_of!(EffectDescriptorSummary, total_bytes),
            offset_of!(EffectDescriptorSummary, parameter_count),
            offset_of!(EffectDescriptorSummary, port_count),
            offset_of!(EffectDescriptorSummary, quality_count),
            offset_of!(EffectDescriptorSummary, enum_choice_count),
            offset_of!(EffectDescriptorSummary, state_layout_version),
            offset_of!(EffectDescriptorSummary, supported_link_mode_bits),
            offset_of!(EffectDescriptorSummary, identity),
        ],
        &[0, 4, 8, 12, 16, 20, 24, 28, 32],
    );

    assert_eq!(
        (
            size_of::<EffectDescriptorWireDiagnostic>(),
            align_of::<EffectDescriptorWireDiagnostic>()
        ),
        (16, 4)
    );
    assert_offsets::<EffectDescriptorWireDiagnostic>(
        &[
            offset_of!(EffectDescriptorWireDiagnostic, code),
            offset_of!(EffectDescriptorWireDiagnostic, byte_offset),
            offset_of!(EffectDescriptorWireDiagnostic, record_index),
            offset_of!(EffectDescriptorWireDiagnostic, required_bytes),
        ],
        &[0, 4, 8, 12],
    );
}

#[test]
fn rust_only_package_and_state_diagnostics_retain_exact_c_layouts() {
    assert_eq!(
        (
            size_of::<EffectPackageDiagnostic>(),
            align_of::<EffectPackageDiagnostic>()
        ),
        (32, 8)
    );
    assert_eq!(
        (
            size_of::<EffectStateDiagnostic>(),
            align_of::<EffectStateDiagnostic>()
        ),
        (32, 8)
    );
    assert_eq!(offset_of!(EffectPackageDiagnostic, required_bytes), 24);
    assert_eq!(offset_of!(EffectStateDiagnostic, required_bytes), 24);
}
