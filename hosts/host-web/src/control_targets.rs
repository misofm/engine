//! Bounded browser codec and workspace for the EQ target-preparation ABI.

#![allow(missing_docs)]

use core::{cell::RefCell, mem::size_of};
use effect_contract::ParameterChannel;
use host_core::{
    EQ_EDIT_CAPACITY, EQ_TARGET_CAPACITY, EQ_VALUE_COUNT, EqTargetEdit, EqTargetPreparer,
    EqTargetPreparerError,
};

pub const EQ_TARGET_REQUEST_HEADER_BYTES: usize = 32;
pub const EQ_TARGET_EDIT_BYTES: usize = 12;
pub const EQ_TARGET_REQUEST_CAPACITY: usize =
    EQ_TARGET_REQUEST_HEADER_BYTES + EQ_VALUE_COUNT * 4 + EQ_EDIT_CAPACITY * EQ_TARGET_EDIT_BYTES;
pub const EQ_TARGET_RESULT_HEADER_BYTES: usize = 32;
pub const EQ_TARGET_TARGET_BYTES: usize = 56;
pub const EQ_TARGET_RESULT_CAPACITY: usize = EQ_TARGET_RESULT_HEADER_BYTES
    + EQ_VALUE_COUNT * 4
    + EQ_TARGET_CAPACITY * EQ_TARGET_TARGET_BYTES;
pub const PREPARED_EFFECT_COMPANION_CAPACITY: usize = size_of::<WebPreparedEffectCompanionHeader>()
    + 2 * crate::MAXIMUM_COMMAND_RECORDS as usize * size_of::<WebPreparedEffectCompanionRecord>();

/// One fixed host-side workspace for the opaque prepared-control companion and addressed EQ
/// configuration copy.  It is allocated once with the host and reused for every between-block
/// submission; no target list or per-owner cache is retained here.
#[repr(C)]
pub struct PreparedControlWorkspace {
    pub companion: [u8; PREPARED_EFFECT_COMPANION_CAPACITY],
    pub config: [u8; size_of::<WebEqTargetConfig>()],
}

impl PreparedControlWorkspace {
    pub fn zeroed() -> Box<Self> {
        Box::new(Self {
            companion: [0; PREPARED_EFFECT_COMPANION_CAPACITY],
            config: [0; size_of::<WebEqTargetConfig>()],
        })
    }
}

const _: () = {
    assert!(size_of::<WebEqTargetRequest>() == 32);
    assert!(size_of::<WebEqTargetEdit>() == 12);
    assert!(size_of::<WebPreparedEffectTarget>() == 56);
    assert!(size_of::<WebEqTargetResult>() == 32);
    assert!(size_of::<WebPreparedEffectCompanionHeader>() == 24);
    assert!(size_of::<WebPreparedEffectCompanionRecord>() == 80);
    assert!(size_of::<WebEqTargetConfig>() == 272);
};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WebEqTargetRequest {
    pub struct_size: u32,
    pub abi_version: u32,
    pub sample_rate_hz: u32,
    pub seed_count: u32,
    pub edit_count: u32,
    pub reserved: [u32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WebEqTargetEdit {
    pub parameter_id: u32,
    pub channel: u32,
    pub value: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WebPreparedEffectTarget {
    pub slot: u32,
    pub channel: u32,
    pub words: [u32; 12],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WebEqTargetResult {
    pub struct_size: u32,
    pub abi_version: u32,
    pub value_count: u32,
    pub target_count: u32,
    pub workspace_retained_bytes: u64,
    pub workspace_largest_allocation_bytes: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WebPreparedEffectCompanionHeader {
    pub struct_size: u32,
    pub abi_version: u32,
    pub host_generation: u64,
    pub target_count: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WebPreparedEffectCompanionRecord {
    pub track_index: u32,
    pub rack: u32,
    pub effect_index: u32,
    pub reserved: u32,
    pub base_revision: u64,
    pub slot: u32,
    pub channel: u32,
    pub words: [u32; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WebEqTargetConfig {
    pub struct_size: u32,
    pub abi_version: u32,
    pub sample_rate_hz: u32,
    pub value_count: u32,
    pub host_generation: u64,
    pub owner_revision: u64,
    pub values: [f32; EQ_VALUE_COUNT],
}
impl Default for WebEqTargetConfig {
    fn default() -> Self {
        Self {
            struct_size: 0,
            abi_version: 0,
            sample_rate_hz: 0,
            value_count: 0,
            host_generation: 0,
            owner_revision: 0,
            values: [0.0; EQ_VALUE_COUNT],
        }
    }
}

pub struct EqTargetWorkspace {
    pub request: [u8; EQ_TARGET_REQUEST_CAPACITY],
    pub result: [u8; EQ_TARGET_RESULT_CAPACITY],
    pub result_bytes: usize,
    pub preparer: EqTargetPreparer,
    pub rejected_edit_index: u32,
    pub rejected_reason: u32,
}

impl EqTargetWorkspace {
    fn new(preparer: EqTargetPreparer) -> Self {
        Self {
            request: [0; EQ_TARGET_REQUEST_CAPACITY],
            result: [0; EQ_TARGET_RESULT_CAPACITY],
            result_bytes: 0,
            preparer,
            rejected_edit_index: u32::MAX,
            rejected_reason: crate::COMMAND_REASON_NONE,
        }
    }
}

thread_local! {
    static WORKSPACE: RefCell<Option<Box<EqTargetWorkspace>>> = const { RefCell::new(None) };
}

fn get_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        bytes.get(offset..offset + 4)?.try_into().ok()?,
    ))
}

fn get_f32(bytes: &[u8], offset: usize) -> Option<f32> {
    Some(f32::from_bits(get_u32(bytes, offset)?))
}

fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn channel_wire(channel: ParameterChannel) -> u32 {
    match channel {
        ParameterChannel::Left => 0,
        ParameterChannel::Right => 1,
        ParameterChannel::Both => 2,
    }
}

pub fn open() -> u32 {
    WORKSPACE.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return crate::RESULT_INTERNAL;
        };
        if slot.is_some() {
            return crate::RESULT_OK;
        }
        let Some(factory) = host_core::parametric_eq_target_preparation_factory() else {
            return crate::RESULT_UNSUPPORTED;
        };
        let Ok(preparer) = EqTargetPreparer::new(factory) else {
            return crate::RESULT_UNSUPPORTED;
        };
        *slot = Some(Box::new(EqTargetWorkspace::new(preparer)));
        crate::RESULT_OK
    })
}

pub fn request_ptr() -> u32 {
    WORKSPACE.with(|slot| {
        slot.try_borrow_mut()
            .ok()
            .and_then(|mut workspace| {
                workspace
                    .as_deref_mut()
                    .map(|w| ptr_u32(w.request.as_mut_ptr()))
            })
            .unwrap_or(0)
    })
}

pub fn request_capacity() -> u32 {
    WORKSPACE.with(|slot| {
        if slot.try_borrow().is_ok_and(|workspace| workspace.is_some()) {
            EQ_TARGET_REQUEST_CAPACITY as u32
        } else {
            0
        }
    })
}

fn request_header(request: &[u8]) -> Result<(u32, u32), u32> {
    if request.len() < EQ_TARGET_REQUEST_HEADER_BYTES {
        return Err(crate::RESULT_INVALID_ARGUMENT);
    }
    let struct_size = get_u32(request, 0).ok_or(crate::RESULT_INVALID_ARGUMENT)?;
    let version = get_u32(request, 4).ok_or(crate::RESULT_INVALID_ARGUMENT)?;
    let rate = get_u32(request, 8).ok_or(crate::RESULT_INVALID_ARGUMENT)?;
    let seeds = get_u32(request, 12).ok_or(crate::RESULT_INVALID_ARGUMENT)?;
    let edits = get_u32(request, 16).ok_or(crate::RESULT_INVALID_ARGUMENT)?;
    if struct_size as usize != EQ_TARGET_REQUEST_HEADER_BYTES
        || version != crate::ABI_VERSION
        || seeds as usize != EQ_VALUE_COUNT
        || edits as usize > EQ_EDIT_CAPACITY
        || request[20..32].iter().any(|byte| *byte != 0)
    {
        return Err(crate::RESULT_INVALID_ARGUMENT);
    }
    let exact =
        EQ_TARGET_REQUEST_HEADER_BYTES + EQ_VALUE_COUNT * 4 + edits as usize * EQ_TARGET_EDIT_BYTES;
    if exact != request.len() {
        return Err(if exact > request.len() {
            crate::RESULT_BUFFER_TOO_SMALL
        } else {
            crate::RESULT_INVALID_ARGUMENT
        });
    }
    Ok((rate, edits))
}

fn preparation_refusal(error: EqTargetPreparerError) -> (u32, u32) {
    use host_core::control_preparation::EditErrorKind;
    let reason = match error {
        EqTargetPreparerError::Edit {
            kind: EditErrorKind::Parameter,
            ..
        } => crate::COMMAND_REASON_UNKNOWN_PARAMETER,
        EqTargetPreparerError::Edit {
            kind: EditErrorKind::NotAutomatable,
            ..
        }
        | EqTargetPreparerError::Unsupported => crate::COMMAND_REASON_UNSUPPORTED_KIND,
        EqTargetPreparerError::Edit {
            kind: EditErrorKind::Domain,
            ..
        } => crate::COMMAND_REASON_DOMAIN,
        _ => crate::COMMAND_REASON_MALFORMED,
    };
    let result = if reason == crate::COMMAND_REASON_UNSUPPORTED_KIND {
        crate::RESULT_UNSUPPORTED
    } else {
        crate::RESULT_INVALID_ARGUMENT
    };
    (result, reason)
}

pub fn prepare(request_bytes: u32) -> u32 {
    WORKSPACE.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return crate::RESULT_INTERNAL;
        };
        let Some(workspace) = slot.as_deref_mut() else {
            return crate::RESULT_WRONG_STATE;
        };
        workspace.rejected_edit_index = u32::MAX;
        workspace.rejected_reason = crate::COMMAND_REASON_MALFORMED;
        let request = &workspace.request;
        if request_bytes as usize > request.len() {
            return crate::RESULT_BUFFER_TOO_SMALL;
        }
        let request = &request[..request_bytes as usize];
        let (rate, edit_count) = match request_header(request) {
            Ok(header) => header,
            Err(result) => return result,
        };
        let mut seeds = [0.0_f32; EQ_VALUE_COUNT];
        for (index, seed) in seeds.iter_mut().enumerate() {
            let Some(value) = get_f32(request, EQ_TARGET_REQUEST_HEADER_BYTES + index * 4) else {
                return crate::RESULT_INVALID_ARGUMENT;
            };
            *seed = value;
        }
        let mut edits = [EqTargetEdit {
            parameter_id: 0,
            channel: ParameterChannel::Both,
            value: 0.0,
        }; EQ_EDIT_CAPACITY];
        for (index, edit) in edits.iter_mut().enumerate().take(edit_count as usize) {
            let offset =
                EQ_TARGET_REQUEST_HEADER_BYTES + EQ_VALUE_COUNT * 4 + index * EQ_TARGET_EDIT_BYTES;
            let (Some(parameter_id), Some(channel), Some(value)) = (
                get_u32(request, offset),
                get_u32(request, offset + 4),
                get_f32(request, offset + 8),
            ) else {
                return crate::RESULT_INVALID_ARGUMENT;
            };
            let Some(channel) = (match channel {
                0 => Some(ParameterChannel::Left),
                1 => Some(ParameterChannel::Right),
                2 => Some(ParameterChannel::Both),
                _ => None,
            }) else {
                workspace.rejected_edit_index = index as u32;
                return crate::RESULT_INVALID_ARGUMENT;
            };
            *edit = EqTargetEdit {
                parameter_id,
                channel,
                value,
            };
        }
        let mut targets = [effect_contract::PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Both,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; EQ_TARGET_CAPACITY];
        let (values, count) = match workspace.preparer.prepare(
            rate,
            &seeds,
            &edits[..edit_count as usize],
            &mut targets,
        ) {
            Ok(result) => result,
            Err(error) => {
                workspace.rejected_edit_index = error.index();
                let (result, reason) = preparation_refusal(error);
                workspace.rejected_reason = reason;
                return result;
            }
        };
        let mut output = [0_u8; EQ_TARGET_RESULT_CAPACITY];
        put_u32(&mut output, 0, EQ_TARGET_RESULT_HEADER_BYTES as u32);
        put_u32(&mut output, 4, crate::ABI_VERSION);
        put_u32(&mut output, 8, EQ_VALUE_COUNT as u32);
        put_u32(&mut output, 12, count as u32);
        let factory_bytes = workspace.preparer.factory_allocation_bytes();
        let workspace_bytes = size_of::<EqTargetWorkspace>() + factory_bytes;
        put_u64(&mut output, 16, workspace_bytes as u64);
        put_u64(
            &mut output,
            24,
            size_of::<EqTargetWorkspace>().max(factory_bytes) as u64,
        );
        for (index, value) in values.iter().copied().enumerate() {
            put_u32(&mut output, 32 + index * 4, value.to_bits());
        }
        for (index, target) in targets[..count].iter().enumerate() {
            let offset = 272 + index * EQ_TARGET_TARGET_BYTES;
            put_u32(&mut output, offset, target.slot);
            put_u32(&mut output, offset + 4, channel_wire(target.channel));
            for (word, value) in target.words.iter().copied().enumerate() {
                put_u32(&mut output, offset + 8 + word * 4, value);
            }
        }
        workspace.result[..output.len()].copy_from_slice(&output);
        workspace.result_bytes = 272 + count * EQ_TARGET_TARGET_BYTES;
        workspace.rejected_reason = crate::COMMAND_REASON_NONE;
        crate::RESULT_OK
    })
}

pub fn result_ptr() -> u32 {
    WORKSPACE.with(|slot| {
        slot.try_borrow()
            .ok()
            .and_then(|w| w.as_ref().map(|w| ptr_u32(w.result.as_ptr())))
            .unwrap_or(0)
    })
}
pub fn result_bytes() -> u32 {
    WORKSPACE.with(|slot| {
        slot.try_borrow()
            .ok()
            .and_then(|w| w.as_ref().and_then(|w| u32::try_from(w.result_bytes).ok()))
            .unwrap_or(0)
    })
}
pub fn rejected_edit_index() -> u32 {
    WORKSPACE.with(|slot| {
        slot.try_borrow()
            .ok()
            .and_then(|w| w.as_ref().map(|w| w.rejected_edit_index))
            .unwrap_or(u32::MAX)
    })
}
pub fn rejected_reason() -> u32 {
    WORKSPACE.with(|slot| {
        slot.try_borrow()
            .ok()
            .and_then(|w| w.as_ref().map(|w| w.rejected_reason))
            .unwrap_or(crate::COMMAND_REASON_WRONG_STATE)
    })
}
pub fn result_capacity() -> u32 {
    WORKSPACE.with(|slot| {
        if slot.try_borrow().is_ok_and(|workspace| workspace.is_some()) {
            EQ_TARGET_RESULT_CAPACITY as u32
        } else {
            0
        }
    })
}
pub fn close() -> u32 {
    WORKSPACE.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return crate::RESULT_INTERNAL;
        };
        if slot.take().is_some() {
            crate::RESULT_OK
        } else {
            crate::RESULT_WRONG_STATE
        }
    })
}

fn ptr_u32<T>(pointer: *const T) -> u32 {
    u32::try_from(pointer as usize).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_header_rejects_truncation_and_malformed_counts() {
        let mut backing = [0_u8; EQ_TARGET_REQUEST_CAPACITY + 1];
        // Deliberately unaligned bytes: decoding must not cast to a repr(C) struct.
        let bytes = &mut backing[1..];
        put_u32(bytes, 0, 32);
        put_u32(bytes, 4, crate::ABI_VERSION);
        put_u32(bytes, 8, 48_000);
        put_u32(bytes, 12, 60);
        for length in 0..272 {
            assert!(request_header(&bytes[..length]).is_err(), "length {length}");
        }
        assert_eq!(request_header(&bytes[..272]), Ok((48_000, 0)));
        assert!(request_header(&bytes[..273]).is_err());
        put_u32(bytes, 16, 256);
        assert_eq!(request_header(bytes), Ok((48_000, 256)));
        for (offset, value) in [(0, 31), (4, 0), (12, 59), (16, 257), (20, 1)] {
            let original = get_u32(bytes, offset).unwrap();
            put_u32(bytes, offset, value);
            assert!(request_header(bytes).is_err(), "offset {offset}");
            put_u32(bytes, offset, original);
        }
        assert_eq!(PREPARED_EFFECT_COMPANION_CAPACITY, 40_984);
    }

    #[test]
    fn production_capability_is_still_unsupported_and_layouts_are_fixed() {
        assert_eq!(size_of::<WebEqTargetRequest>(), 32);
        assert_eq!(size_of::<WebEqTargetEdit>(), 12);
        assert_eq!(size_of::<WebPreparedEffectTarget>(), 56);
        assert_eq!(size_of::<WebEqTargetResult>(), 32);
        assert_eq!(size_of::<WebPreparedEffectCompanionHeader>(), 24);
        assert_eq!(size_of::<WebPreparedEffectCompanionRecord>(), 80);
        assert_eq!(size_of::<WebEqTargetConfig>(), 272);
        assert_eq!(open(), crate::RESULT_UNSUPPORTED);
        assert_eq!(request_ptr(), 0);
        assert_eq!(prepare(0), crate::RESULT_WRONG_STATE);
        assert_eq!(rejected_edit_index(), u32::MAX);
        assert_eq!(rejected_reason(), crate::COMMAND_REASON_WRONG_STATE);
        assert_eq!(close(), crate::RESULT_WRONG_STATE);
    }
}
