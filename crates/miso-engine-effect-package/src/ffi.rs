//! Raw C-pointer boundary for descriptor-wire inspection only.

#![allow(unsafe_code)]

use crate::{
    EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE, EffectDescriptorWireDiagnosticCodeV1,
    EffectDescriptorWireDiagnosticV1, verify_effect_descriptor_wire_v1,
};
use core::slice;

pub const EFFECT_DESCRIPTOR_INSPECTION_ABI_VERSION_V1: u32 = 0x0001_0000;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct EffectDescriptorParameterRecordV1 {
    pub id: u32,
    pub unit: u32,
    pub domain: u32,
    pub mapping: u32,
    pub automation_rate: u32,
    pub channel_policy: u32,
    pub smoothing: u32,
    pub smoothing_samples: u32,
    pub flags: u32,
    pub minimum_bits: u32,
    pub maximum_bits: u32,
    pub default_bits: u32,
    pub enum_start: u32,
    pub enum_count: u32,
    pub display_name_offset: u32,
    pub display_name_length: u32,
    pub display_unit_offset: u32,
    pub display_unit_length: u32,
    pub reserved0: u32,
    pub reserved1: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct EffectDescriptorPortRecordV1 {
    pub id_offset: u32,
    pub id_length: u32,
    pub role: u32,
    pub required: u32,
    pub layout: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct EffectDescriptorQualityRecordV1 {
    pub quality: u32,
    pub sample_rate: u32,
    pub latency_samples: u64,
    pub tail_kind: u32,
    pub reserved0: u32,
    pub tail_samples: u64,
    pub common_state_bytes: u32,
    pub left_state_bytes: u32,
    pub right_state_bytes: u32,
    pub reserved1: u32,
    pub scratch_fixed_bytes: u64,
    pub scratch_bytes_per_frame: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct EffectDescriptorEnumChoiceRecordV1 {
    pub value_bits: u32,
    pub label_offset: u32,
    pub label_length: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct EffectDescriptorSummaryV1 {
    pub abi_version: u32,
    pub total_bytes: u32,
    pub parameter_count: u32,
    pub port_count: u32,
    pub quality_count: u32,
    pub enum_choice_count: u32,
    pub state_layout_version: u32,
    pub supported_link_mode_bits: u32,
    pub identity: [u8; 32],
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("verified field"),
    )
}

fn read_u64(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(
        bytes[offset..offset + 8]
            .try_into()
            .expect("verified field"),
    )
}

unsafe fn write_diagnostic(
    output: *mut EffectDescriptorWireDiagnosticV1,
    value: EffectDescriptorWireDiagnosticV1,
) {
    // SAFETY: The caller promises writable storage for the mandatory diagnostic record.
    unsafe { output.write(value) };
}

unsafe fn zero_required_counts(
    parameters: *mut u32,
    ports: *mut u32,
    qualities: *mut u32,
    choices: *mut u32,
) {
    for output in [parameters, ports, qualities, choices] {
        if !output.is_null() {
            // SAFETY: Every nonnull required-count pointer denotes writable storage for one u32.
            unsafe { output.write(0) };
        }
    }
}

fn null_diagnostic() -> EffectDescriptorWireDiagnosticV1 {
    EffectDescriptorWireDiagnosticV1::new(
        EffectDescriptorWireDiagnosticCodeV1::Null,
        EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE,
        EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE,
    )
}

fn parameter_record(bytes: &[u8], offset: usize) -> EffectDescriptorParameterRecordV1 {
    EffectDescriptorParameterRecordV1 {
        id: read_u32(bytes, offset),
        unit: read_u32(bytes, offset + 4),
        domain: read_u32(bytes, offset + 8),
        mapping: read_u32(bytes, offset + 12),
        automation_rate: read_u32(bytes, offset + 16),
        channel_policy: read_u32(bytes, offset + 20),
        smoothing: read_u32(bytes, offset + 24),
        smoothing_samples: read_u32(bytes, offset + 28),
        flags: read_u32(bytes, offset + 32),
        minimum_bits: read_u32(bytes, offset + 36),
        maximum_bits: read_u32(bytes, offset + 40),
        default_bits: read_u32(bytes, offset + 44),
        enum_start: read_u32(bytes, offset + 48),
        enum_count: read_u32(bytes, offset + 52),
        display_name_offset: read_u32(bytes, offset + 56),
        display_name_length: read_u32(bytes, offset + 60),
        display_unit_offset: read_u32(bytes, offset + 64),
        display_unit_length: read_u32(bytes, offset + 68),
        reserved0: read_u32(bytes, offset + 72),
        reserved1: read_u32(bytes, offset + 76),
    }
}

fn port_record(bytes: &[u8], offset: usize) -> EffectDescriptorPortRecordV1 {
    EffectDescriptorPortRecordV1 {
        id_offset: read_u32(bytes, offset),
        id_length: read_u32(bytes, offset + 4),
        role: read_u32(bytes, offset + 8),
        required: read_u32(bytes, offset + 12),
        layout: read_u32(bytes, offset + 16),
        reserved: read_u32(bytes, offset + 20),
    }
}

fn quality_record(bytes: &[u8], offset: usize) -> EffectDescriptorQualityRecordV1 {
    EffectDescriptorQualityRecordV1 {
        quality: read_u32(bytes, offset),
        sample_rate: read_u32(bytes, offset + 4),
        latency_samples: read_u64(bytes, offset + 8),
        tail_kind: read_u32(bytes, offset + 16),
        reserved0: read_u32(bytes, offset + 20),
        tail_samples: read_u64(bytes, offset + 24),
        common_state_bytes: read_u32(bytes, offset + 32),
        left_state_bytes: read_u32(bytes, offset + 36),
        right_state_bytes: read_u32(bytes, offset + 40),
        reserved1: read_u32(bytes, offset + 44),
        scratch_fixed_bytes: read_u64(bytes, offset + 48),
        scratch_bytes_per_frame: read_u64(bytes, offset + 56),
    }
}

fn enum_choice_record(bytes: &[u8], offset: usize) -> EffectDescriptorEnumChoiceRecordV1 {
    EffectDescriptorEnumChoiceRecordV1 {
        value_bits: read_u32(bytes, offset),
        label_offset: read_u32(bytes, offset + 4),
        label_length: read_u32(bytes, offset + 8),
        reserved: read_u32(bytes, offset + 12),
    }
}

/// Inspect one complete canonical descriptor wire value into caller-owned fixed records.
///
/// # Safety
///
/// Every nonnull pointer must denote readable or writable storage for its declared length or
/// capacity for this call. All input and output regions must be mutually nonoverlapping. No pointer
/// is retained. Output record pointers may be null exactly when their capacity is zero; all other
/// output pointers are mandatory.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn miso_engine_effect_descriptor_v1_inspect(
    wire: *const u8,
    wire_len: usize,
    maximum_wire_bytes: u32,
    summary: *mut EffectDescriptorSummaryV1,
    parameters: *mut EffectDescriptorParameterRecordV1,
    parameter_capacity: u32,
    ports: *mut EffectDescriptorPortRecordV1,
    port_capacity: u32,
    qualities: *mut EffectDescriptorQualityRecordV1,
    quality_capacity: u32,
    enum_choices: *mut EffectDescriptorEnumChoiceRecordV1,
    enum_choice_capacity: u32,
    required_parameters: *mut u32,
    required_ports: *mut u32,
    required_qualities: *mut u32,
    required_enum_choices: *mut u32,
    diagnostic: *mut EffectDescriptorWireDiagnosticV1,
) -> u32 {
    if diagnostic.is_null() {
        return EffectDescriptorWireDiagnosticCodeV1::Null as u32;
    }
    let mandatory_null = summary.is_null()
        || required_parameters.is_null()
        || required_ports.is_null()
        || required_qualities.is_null()
        || required_enum_choices.is_null()
        || (wire.is_null() && wire_len != 0)
        || (parameters.is_null() && parameter_capacity != 0)
        || (ports.is_null() && port_capacity != 0)
        || (qualities.is_null() && quality_capacity != 0)
        || (enum_choices.is_null() && enum_choice_capacity != 0);
    if mandatory_null {
        // SAFETY: `diagnostic` is nonnull; any nonnull required-count pointers follow their ABI
        // writable-storage contracts even when another mandatory argument is null.
        unsafe {
            zero_required_counts(
                required_parameters,
                required_ports,
                required_qualities,
                required_enum_choices,
            );
            write_diagnostic(diagnostic, null_diagnostic());
        }
        return EffectDescriptorWireDiagnosticCodeV1::Null as u32;
    }
    let wire_bytes = if wire_len == 0 {
        &[]
    } else {
        // SAFETY: Nonzero input was checked nonnull and the caller promises readable storage for
        // exactly `wire_len` bytes for this call.
        unsafe { slice::from_raw_parts(wire, wire_len) }
    };
    let verified = match verify_effect_descriptor_wire_v1(wire_bytes, maximum_wire_bytes) {
        Ok(value) => value,
        Err(error) => {
            // SAFETY: All required pointers and the diagnostic are mandatory and nonnull here.
            unsafe {
                zero_required_counts(
                    required_parameters,
                    required_ports,
                    required_qualities,
                    required_enum_choices,
                );
                write_diagnostic(diagnostic, error);
            }
            return error.code as u32;
        }
    };
    let required_parameter_count = verified.parameter_count();
    let required_port_count = verified.port_count();
    let required_quality_count = verified.quality_count();
    let required_choice_count = verified.enum_choice_count();
    if parameter_capacity < required_parameter_count
        || port_capacity < required_port_count
        || quality_capacity < required_quality_count
        || enum_choice_capacity < required_choice_count
    {
        let required_bytes = required_parameter_count
            .checked_mul(80)
            .and_then(|value| value.checked_add(required_port_count.checked_mul(24)?))
            .and_then(|value| value.checked_add(required_quality_count.checked_mul(64)?))
            .and_then(|value| value.checked_add(required_choice_count.checked_mul(16)?));
        let Some(required_bytes) = required_bytes else {
            let error = EffectDescriptorWireDiagnosticV1::new(
                EffectDescriptorWireDiagnosticCodeV1::Overflow,
                EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE,
                EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE,
            );
            // SAFETY: Mandatory outputs were checked nonnull.
            unsafe {
                zero_required_counts(
                    required_parameters,
                    required_ports,
                    required_qualities,
                    required_enum_choices,
                );
                write_diagnostic(diagnostic, error);
            }
            return error.code as u32;
        };
        let error = EffectDescriptorWireDiagnosticV1::buffer_too_small(required_bytes);
        // SAFETY: Mandatory count and diagnostic outputs were checked nonnull; no summary or
        // record array has been written.
        unsafe {
            required_parameters.write(required_parameter_count);
            required_ports.write(required_port_count);
            required_qualities.write(required_quality_count);
            required_enum_choices.write(required_choice_count);
            write_diagnostic(diagnostic, error);
        }
        return error.code as u32;
    }
    // Issue 078 forbids validating the same descriptor twice: the identity comes from the value
    // the single verification pass above already proved canonical.
    let identity = *verified.identity().as_bytes();
    let summary_value = EffectDescriptorSummaryV1 {
        abi_version: EFFECT_DESCRIPTOR_INSPECTION_ABI_VERSION_V1,
        total_bytes: wire_bytes.len() as u32,
        parameter_count: required_parameter_count,
        port_count: required_port_count,
        quality_count: required_quality_count,
        enum_choice_count: required_choice_count,
        state_layout_version: verified.state_layout_version(),
        supported_link_mode_bits: verified.supported_link_mode_bits(),
        identity,
    };
    let parameter_offset = read_u32(wire_bytes, 52) as usize;
    let port_offset = read_u32(wire_bytes, 60) as usize;
    let quality_offset = read_u32(wire_bytes, 68) as usize;
    let choice_offset = read_u32(wire_bytes, 76) as usize;
    // SAFETY: Every capacity was checked against the complete required count. The caller promises
    // writable mutually nonoverlapping storage; every projected record is built field-by-field and
    // no Rust wire layout is reinterpreted.
    unsafe {
        for index in 0..required_parameter_count as usize {
            parameters
                .add(index)
                .write(parameter_record(wire_bytes, parameter_offset + index * 80));
        }
        for index in 0..required_port_count as usize {
            ports
                .add(index)
                .write(port_record(wire_bytes, port_offset + index * 24));
        }
        for index in 0..required_quality_count as usize {
            qualities
                .add(index)
                .write(quality_record(wire_bytes, quality_offset + index * 64));
        }
        for index in 0..required_choice_count as usize {
            enum_choices
                .add(index)
                .write(enum_choice_record(wire_bytes, choice_offset + index * 16));
        }
        summary.write(summary_value);
        required_parameters.write(required_parameter_count);
        required_ports.write(required_port_count);
        required_qualities.write(required_quality_count);
        required_enum_choices.write(required_choice_count);
        write_diagnostic(
            diagnostic,
            EffectDescriptorWireDiagnosticV1::new(
                EffectDescriptorWireDiagnosticCodeV1::Ok,
                EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE,
                EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE,
            ),
        );
    }
    EffectDescriptorWireDiagnosticCodeV1::Ok as u32
}

const _: () = {
    assert!(size_of::<EffectDescriptorParameterRecordV1>() == 80);
    assert!(align_of::<EffectDescriptorParameterRecordV1>() == 4);
    assert!(size_of::<EffectDescriptorPortRecordV1>() == 24);
    assert!(align_of::<EffectDescriptorPortRecordV1>() == 4);
    assert!(size_of::<EffectDescriptorQualityRecordV1>() == 64);
    assert!(align_of::<EffectDescriptorQualityRecordV1>() == 8);
    assert!(size_of::<EffectDescriptorEnumChoiceRecordV1>() == 16);
    assert!(align_of::<EffectDescriptorEnumChoiceRecordV1>() == 4);
    assert!(size_of::<EffectDescriptorSummaryV1>() == 64);
    assert!(align_of::<EffectDescriptorSummaryV1>() == 4);
    assert!(size_of::<EffectDescriptorWireDiagnosticV1>() == 16);
    assert!(align_of::<EffectDescriptorWireDiagnosticV1>() == 4);
};

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};
    use core::ptr;

    #[test]
    fn c_records_have_the_frozen_sizes_alignments_and_offsets() {
        assert_eq!(
            (
                size_of::<EffectDescriptorParameterRecordV1>(),
                align_of::<EffectDescriptorParameterRecordV1>()
            ),
            (80, 4)
        );
        assert_eq!(
            (
                size_of::<EffectDescriptorPortRecordV1>(),
                align_of::<EffectDescriptorPortRecordV1>()
            ),
            (24, 4)
        );
        assert_eq!(
            (
                size_of::<EffectDescriptorQualityRecordV1>(),
                align_of::<EffectDescriptorQualityRecordV1>()
            ),
            (64, 8)
        );
        assert_eq!(
            (
                size_of::<EffectDescriptorEnumChoiceRecordV1>(),
                align_of::<EffectDescriptorEnumChoiceRecordV1>()
            ),
            (16, 4)
        );
        assert_eq!(
            (
                size_of::<EffectDescriptorSummaryV1>(),
                align_of::<EffectDescriptorSummaryV1>()
            ),
            (64, 4)
        );
        assert_eq!(
            (
                size_of::<EffectDescriptorWireDiagnosticV1>(),
                align_of::<EffectDescriptorWireDiagnosticV1>()
            ),
            (16, 4)
        );
        assert_eq!(
            offset_of!(EffectDescriptorParameterRecordV1, default_bits),
            44
        );
        assert_eq!(offset_of!(EffectDescriptorParameterRecordV1, reserved1), 76);
        assert_eq!(offset_of!(EffectDescriptorPortRecordV1, reserved), 20);
        assert_eq!(
            offset_of!(EffectDescriptorQualityRecordV1, tail_samples),
            24
        );
        assert_eq!(
            offset_of!(EffectDescriptorQualityRecordV1, scratch_bytes_per_frame),
            56
        );
        assert_eq!(offset_of!(EffectDescriptorEnumChoiceRecordV1, reserved), 12);
        assert_eq!(offset_of!(EffectDescriptorSummaryV1, identity), 32);
        assert_eq!(
            offset_of!(EffectDescriptorWireDiagnosticV1, required_bytes),
            12
        );
    }

    #[test]
    fn null_diagnostic_returns_null_without_dereferencing_other_arguments() {
        // SAFETY: This intentionally exercises the documented null-diagnostic early return; all
        // other pointers are null and must not be dereferenced in this case.
        let code = unsafe {
            miso_engine_effect_descriptor_v1_inspect(
                ptr::null(),
                1,
                1,
                ptr::null_mut(),
                ptr::null_mut(),
                1,
                ptr::null_mut(),
                1,
                ptr::null_mut(),
                1,
                ptr::null_mut(),
                1,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        assert_eq!(code, EffectDescriptorWireDiagnosticCodeV1::Null as u32);
    }

    #[test]
    fn mandatory_null_publishes_only_zero_counts_and_null_diagnostic() {
        let mut required_parameters = u32::MAX;
        let mut required_ports = u32::MAX;
        let mut required_qualities = u32::MAX;
        let mut required_choices = u32::MAX;
        let mut diagnostic =
            EffectDescriptorWireDiagnosticV1::new(EffectDescriptorWireDiagnosticCodeV1::Ok, 7, 9);
        // SAFETY: All nonnull outputs point to one writable value. Null record pointers have zero
        // capacity; the deliberately null mandatory summary is the behavior under test.
        let code = unsafe {
            miso_engine_effect_descriptor_v1_inspect(
                ptr::null(),
                0,
                1,
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                0,
                &mut required_parameters,
                &mut required_ports,
                &mut required_qualities,
                &mut required_choices,
                &mut diagnostic,
            )
        };
        assert_eq!(code, EffectDescriptorWireDiagnosticCodeV1::Null as u32);
        assert_eq!(
            [
                required_parameters,
                required_ports,
                required_qualities,
                required_choices,
            ],
            [0; 4]
        );
        assert_eq!(diagnostic, null_diagnostic());
    }
}
