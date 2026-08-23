//! Shared bounded-TLV reader, validation, and output sinks.

use crate::{DecodeError, EncodeError, ProtocolLimits, schema::MessageSpec};

#[cfg(test)]
pub(crate) const WIRE_U8: u8 = 1;
#[cfg(test)]
pub(crate) const WIRE_U32: u8 = 3;
#[cfg(test)]
pub(crate) const WIRE_F32: u8 = 6;
#[cfg(test)]
pub(crate) const WIRE_BOOL: u8 = 8;
#[cfg(test)]
pub(crate) const WIRE_UTF8: u8 = 9;
pub(crate) const WIRE_MESSAGE: u8 = 11;

const TLV_PREFIX_BYTES: usize = 8;

/// One borrowed field produced by the shared allocation-free TLV reader.
#[derive(Clone, Copy)]
pub(crate) struct Field<'a> {
    pub(crate) id: u16,
    pub(crate) wire: u8,
    pub(crate) mandatory: bool,
    pub(crate) value: &'a [u8],
    pub(crate) start: usize,
    pub(crate) end: usize,
}

/// Allocation-free structural reader for one BTLV message level.
pub(crate) struct Reader<'a> {
    bytes: &'a [u8],
    declared_count: u32,
    index: u32,
    cursor: usize,
    previous_id: u16,
}

impl<'a> Reader<'a> {
    pub(crate) const fn new(bytes: &'a [u8], declared_count: u32) -> Self {
        Self {
            bytes,
            declared_count,
            index: 0,
            cursor: 0,
            previous_id: 0,
        }
    }

    pub(crate) fn next_field(&mut self) -> Result<Option<Field<'a>>, DecodeError> {
        if self.index == self.declared_count {
            if self.cursor != self.bytes.len() {
                return Err(DecodeError::InvalidTlv);
            }
            return Ok(None);
        }
        let start = self.cursor;
        let prefix_end = self
            .cursor
            .checked_add(TLV_PREFIX_BYTES)
            .ok_or(DecodeError::LimitExceeded)?;
        let prefix = self
            .bytes
            .get(self.cursor..prefix_end)
            .ok_or(DecodeError::Truncated)?;
        let id = read_u16_at(prefix, 0)?;
        let wire = prefix[2];
        let flags = prefix[3];
        if id == 0
            || !(1..=15).contains(&wire)
            || flags & !1 != 0
            || (self.index != 0 && id < self.previous_id)
        {
            return Err(DecodeError::InvalidTlv);
        }
        let length =
            usize::try_from(read_u32_at(prefix, 4)?).map_err(|_| DecodeError::LimitExceeded)?;
        let value_end = prefix_end
            .checked_add(length)
            .ok_or(DecodeError::LimitExceeded)?;
        let value = self
            .bytes
            .get(prefix_end..value_end)
            .ok_or(DecodeError::Truncated)?;
        let padded_end = value_end
            .checked_add(padding(length))
            .ok_or(DecodeError::LimitExceeded)?;
        if self
            .bytes
            .get(value_end..padded_end)
            .ok_or(DecodeError::Truncated)?
            .iter()
            .any(|byte| *byte != 0)
        {
            return Err(DecodeError::InvalidTlv);
        }
        self.index += 1;
        self.cursor = padded_end;
        self.previous_id = id;
        Ok(Some(Field {
            id,
            wire,
            mandatory: flags & 1 != 0,
            value,
            start,
            end: padded_end,
        }))
    }
}

#[derive(Clone, Copy)]
struct FieldSlot<'a> {
    wire: u8,
    value: &'a [u8],
    count: u32,
    span: &'a [u8],
    start: usize,
}

/// Fixed-slot schema view built in one allocation-free pass.
pub(crate) struct Fields<'a> {
    bytes: &'a [u8],
    count: u32,
    spec: Option<&'static MessageSpec>,
    slots: [Option<FieldSlot<'a>>; 32],
    validation: Option<(ProtocolLimits, u8)>,
}

impl<'a> Fields<'a> {
    pub(crate) fn nested(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        let header = bytes.get(..8).ok_or(DecodeError::Truncated)?;
        let count = read_u32_at(header, 0)?;
        if read_u32_at(header, 4)? != 0 {
            return Err(DecodeError::NonzeroReserved);
        }
        Ok(Self::raw(&bytes[8..], count))
    }

    pub(crate) const fn raw(bytes: &'a [u8], count: u32) -> Self {
        Self {
            bytes,
            count,
            spec: None,
            slots: [None; 32],
            validation: None,
        }
    }

    pub(crate) const fn bounded(
        bytes: &'a [u8],
        count: u32,
        limits: ProtocolLimits,
        depth: u8,
    ) -> Self {
        Self {
            bytes,
            count,
            spec: None,
            slots: [None; 32],
            validation: Some((limits, depth)),
        }
    }

    pub(crate) const fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub(crate) fn count(&self, id: u16) -> usize {
        self.slot(id).map_or(0, |slot| slot.count as usize)
    }

    pub(crate) fn schema_spec(mut self, spec: &'static MessageSpec) -> Result<Self, DecodeError> {
        debug_assert!(!spec.name.is_empty());
        if spec.fields.len() > self.slots.len() {
            return Err(DecodeError::LimitExceeded);
        }
        let mut reader = Reader::new(self.bytes, self.count);
        while let Some(field) = reader.next_field()? {
            let position = spec
                .fields
                .binary_search_by_key(&field.id, |candidate| candidate.id)
                .ok();
            if let Some((limits, depth)) = self.validation {
                let recurse_nested = position
                    .and_then(|index| spec.fields[index].nested)
                    .is_none();
                validate_value(field.wire, field.value, depth, limits, recurse_nested)?;
            }
            match position.map(|index| spec.fields[index]) {
                Some(field_spec)
                    if field.mandatory != field_spec.mandatory
                        || crate::schema::Wire::from_raw(field.wire) != Some(field_spec.wire)
                        || (field_spec.wire == crate::schema::Wire::Message)
                            != field_spec.nested.is_some() =>
                {
                    return Err(DecodeError::InvalidTlv);
                }
                Some(field_spec) => {
                    let position = position.ok_or(DecodeError::InvalidTlv)?;
                    if let Some(slot) = self.slots[position].as_mut() {
                        if !field_spec.repeated {
                            return Err(DecodeError::InvalidTlv);
                        }
                        slot.count = slot
                            .count
                            .checked_add(1)
                            .ok_or(DecodeError::LimitExceeded)?;
                        slot.span = self
                            .bytes
                            .get(slot.start..field.end)
                            .ok_or(DecodeError::InvalidTlv)?;
                    } else {
                        self.slots[position] = Some(FieldSlot {
                            wire: field.wire,
                            value: field.value,
                            count: 1,
                            span: self
                                .bytes
                                .get(field.start..field.end)
                                .ok_or(DecodeError::InvalidTlv)?,
                            start: field.start,
                        });
                    }
                }
                None if field.mandatory => return Err(DecodeError::UnknownRequiredField),
                _ => {}
            }
        }
        self.spec = Some(spec);
        Ok(self)
    }

    pub(crate) fn tagged_schema_spec(
        self,
        spec: &'static MessageSpec,
        known: &'static MessageSpec,
    ) -> Result<Self, DecodeError> {
        let bytes = self.bytes;
        let count = self.count;
        let message = self.schema_spec(spec)?;
        let mut reader = Reader::new(bytes, count);
        while let Some(field) = reader.next_field()? {
            if known
                .fields
                .iter()
                .any(|candidate| candidate.id == field.id)
                && !spec.fields.iter().any(|candidate| candidate.id == field.id)
            {
                return Err(DecodeError::InvalidTlv);
            }
        }
        Ok(message)
    }

    fn slot(&self, id: u16) -> Option<FieldSlot<'a>> {
        let spec = self.spec?;
        let position = spec
            .fields
            .binary_search_by_key(&id, |candidate| candidate.id)
            .ok()?;
        self.slots[position]
    }

    pub(crate) fn tag(&self, id: u16, wire: u8) -> Result<&'a [u8], DecodeError> {
        let mut reader = Reader::new(self.bytes, self.count);
        let field = reader.next_field()?.ok_or(DecodeError::InvalidTlv)?;
        if field.id != id || field.wire != wire {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(field.value)
    }

    pub(crate) fn one(&self, id: u16, wire: u8) -> Result<&'a [u8], DecodeError> {
        let slot = self.slot(id).ok_or(DecodeError::InvalidTlv)?;
        if slot.count != 1 || slot.wire != wire {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(slot.value)
    }

    pub(crate) fn optional_one(&self, id: u16, wire: u8) -> Result<Option<&'a [u8]>, DecodeError> {
        let Some(slot) = self.slot(id) else {
            return Ok(None);
        };
        if slot.count != 1 || slot.wire != wire {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(Some(slot.value))
    }

    pub(crate) fn values(
        &self,
        id: u16,
        wire: u8,
    ) -> Result<impl Iterator<Item = &'a [u8]> + '_, DecodeError> {
        let slot = self.slot(id);
        if slot.is_some_and(|slot| slot.wire != wire) {
            return Err(DecodeError::InvalidTlv);
        }
        let (span, count) = slot.map_or((&[][..], 0), |slot| (slot.span, slot.count));
        let mut reader = Reader::new(span, count);
        Ok(core::iter::from_fn(move || {
            loop {
                match reader.next_field() {
                    Ok(Some(field)) if field.id == id => return Some(field.value),
                    Ok(Some(_)) => {}
                    Ok(None) | Err(_) => return None,
                }
            }
        }))
    }
}

/// Validate one complete BTLV tree with the shared reader.
pub(crate) fn validate_message(
    bytes: &[u8],
    count: u32,
    depth: u8,
    limits: ProtocolLimits,
    spec: Option<&'static MessageSpec>,
    mut top_field: impl FnMut(u16) -> Result<(), DecodeError>,
) -> Result<(), DecodeError> {
    if count > limits.max_tlv_count || bytes.len() > limits.max_frame_bytes {
        return Err(DecodeError::LimitExceeded);
    }
    let mut reader = Reader::new(bytes, count);
    while let Some(field) = reader.next_field()? {
        validate_value(field.wire, field.value, depth, limits, true)?;
        if let Some(spec) = spec {
            match spec
                .fields
                .binary_search_by_key(&field.id, |candidate| candidate.id)
                .ok()
                .map(|position| spec.fields[position])
            {
                Some(field_spec)
                    if field_spec.wire.raw() != field.wire
                        || field_spec.mandatory != field.mandatory =>
                {
                    return Err(DecodeError::InvalidTlv);
                }
                None if field.mandatory => return Err(DecodeError::UnknownRequiredField),
                _ => {}
            }
        }
        top_field(field.id)?;
    }
    Ok(())
}

pub(crate) fn validate_value(
    wire: u8,
    value: &[u8],
    depth: u8,
    limits: ProtocolLimits,
    recurse_nested: bool,
) -> Result<(), DecodeError> {
    let exact = match wire {
        1 | 8 => Some(1),
        2 => Some(2),
        3 | 6 => Some(4),
        4 | 5 | 7 => Some(8),
        _ => None,
    };
    if let Some(exact) = exact {
        if value.len() != exact {
            return Err(DecodeError::InvalidValueLength);
        }
        if wire == 8 && !matches!(value[0], 0 | 1) {
            return Err(DecodeError::InvalidValueLength);
        }
    }
    match wire {
        9 => {
            if value.len() > limits.max_string_bytes {
                return Err(DecodeError::LimitExceeded);
            }
            if core::str::from_utf8(value).is_err() {
                return Err(DecodeError::InvalidUtf8);
            }
        }
        WIRE_MESSAGE => {
            if depth >= limits.max_nesting {
                return Err(DecodeError::LimitExceeded);
            }
            let header = value.get(..8).ok_or(DecodeError::Truncated)?;
            let count = read_u32_at(header, 0)?;
            if read_u32_at(header, 4)? != 0 {
                return Err(DecodeError::NonzeroReserved);
            }
            if count > limits.max_tlv_count {
                return Err(DecodeError::LimitExceeded);
            }
            if recurse_nested {
                validate_message(&value[8..], count, depth + 1, limits, None, |_| Ok(()))?;
            }
        }
        12 if !value.len().is_multiple_of(2) => return Err(DecodeError::InvalidValueLength),
        13 | 15 if !value.len().is_multiple_of(4) => {
            return Err(DecodeError::InvalidValueLength);
        }
        14 if !value.len().is_multiple_of(8) => return Err(DecodeError::InvalidValueLength),
        _ => {}
    }
    Ok(())
}

pub(crate) fn read_u8(value: &[u8]) -> Result<u8, DecodeError> {
    value
        .first()
        .copied()
        .filter(|_| value.len() == 1)
        .ok_or(DecodeError::InvalidValueLength)
}

pub(crate) fn read_u16(value: &[u8]) -> Result<u16, DecodeError> {
    let value: [u8; 2] = value
        .try_into()
        .map_err(|_| DecodeError::InvalidValueLength)?;
    Ok(u16::from_le_bytes(value))
}

pub(crate) fn read_u32(value: &[u8]) -> Result<u32, DecodeError> {
    let value: [u8; 4] = value
        .try_into()
        .map_err(|_| DecodeError::InvalidValueLength)?;
    Ok(u32::from_le_bytes(value))
}

pub(crate) fn read_u64(value: &[u8]) -> Result<u64, DecodeError> {
    let value: [u8; 8] = value
        .try_into()
        .map_err(|_| DecodeError::InvalidValueLength)?;
    Ok(u64::from_le_bytes(value))
}

pub(crate) fn read_f32(value: &[u8]) -> Result<f32, DecodeError> {
    let value: [u8; 4] = value
        .try_into()
        .map_err(|_| DecodeError::InvalidValueLength)?;
    Ok(f32::from_le_bytes(value))
}

pub(crate) fn read_u16_at(bytes: &[u8], offset: usize) -> Result<u16, DecodeError> {
    read_u16(
        bytes
            .get(offset..offset + 2)
            .ok_or(DecodeError::Truncated)?,
    )
}

pub(crate) fn read_u32_at(bytes: &[u8], offset: usize) -> Result<u32, DecodeError> {
    read_u32(
        bytes
            .get(offset..offset + 4)
            .ok_or(DecodeError::Truncated)?,
    )
}

pub(crate) fn read_u64_at(bytes: &[u8], offset: usize) -> Result<u64, DecodeError> {
    read_u64(
        bytes
            .get(offset..offset + 8)
            .ok_or(DecodeError::Truncated)?,
    )
}

pub(crate) const fn padding(value_len: usize) -> usize {
    (8 - (value_len & 7)) & 7
}

/// A bounded output target used by one sizing pass and one caller-buffer write pass.
pub(crate) trait Sink {
    fn limits(&self) -> ProtocolLimits;
    fn written(&self) -> usize;
    fn raw(&mut self, bytes: &[u8]) -> Result<(), EncodeError>;

    fn message_header(&mut self, count: u32) -> Result<(), EncodeError> {
        if count > self.limits().max_tlv_count {
            return Err(EncodeError::LimitExceeded);
        }
        self.raw(&count.to_le_bytes())?;
        self.raw(&0_u32.to_le_bytes())
    }

    fn field(&mut self, id: u16, wire: u8, value: &[u8]) -> Result<(), EncodeError> {
        let length = u32::try_from(value.len()).map_err(|_| EncodeError::LimitExceeded)?;
        let mut prefix = [0_u8; TLV_PREFIX_BYTES];
        prefix[..2].copy_from_slice(&id.to_le_bytes());
        prefix[2] = wire;
        prefix[3] = 1;
        prefix[4..].copy_from_slice(&length.to_le_bytes());
        self.raw(&prefix)?;
        self.raw(value)?;
        self.raw(&[0_u8; 7][..padding(value.len())])
    }

    fn nested(
        &mut self,
        id: u16,
        body: &mut dyn FnMut(&mut dyn Sink) -> Result<(), EncodeError>,
    ) -> Result<(), EncodeError>;
}

pub(crate) struct CountSink {
    length: usize,
    limits: ProtocolLimits,
}

impl CountSink {
    pub(crate) const fn new(limits: ProtocolLimits) -> Self {
        Self { length: 0, limits }
    }

    #[cfg(test)]
    pub(crate) const fn with_length_for_test(length: usize, limits: ProtocolLimits) -> Self {
        Self { length, limits }
    }
}

impl Sink for CountSink {
    fn limits(&self) -> ProtocolLimits {
        self.limits
    }

    fn written(&self) -> usize {
        self.length
    }

    fn raw(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.length = self
            .length
            .checked_add(bytes.len())
            .ok_or(EncodeError::LimitExceeded)?;
        Ok(())
    }

    fn nested(
        &mut self,
        _id: u16,
        body: &mut dyn FnMut(&mut dyn Sink) -> Result<(), EncodeError>,
    ) -> Result<(), EncodeError> {
        self.raw(&[0_u8; TLV_PREFIX_BYTES])?;
        let body_start = self.length;
        body(self)?;
        let body_len = self.length - body_start;
        u32::try_from(body_len).map_err(|_| EncodeError::LimitExceeded)?;
        self.raw(&[0_u8; 7][..padding(body_len)])
    }
}

pub(crate) struct SliceSink<'a> {
    output: &'a mut [u8],
    length: usize,
    limits: ProtocolLimits,
}

impl<'a> SliceSink<'a> {
    pub(crate) const fn new(output: &'a mut [u8], limits: ProtocolLimits) -> Self {
        Self {
            output,
            length: 0,
            limits,
        }
    }
}

impl Sink for SliceSink<'_> {
    fn limits(&self) -> ProtocolLimits {
        self.limits
    }

    fn written(&self) -> usize {
        self.length
    }

    fn raw(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        let end = self
            .length
            .checked_add(bytes.len())
            .ok_or(EncodeError::LimitExceeded)?;
        let target = self
            .output
            .get_mut(self.length..end)
            .ok_or(EncodeError::OutputTooSmall {
                required: usize::MAX,
            })?;
        target.copy_from_slice(bytes);
        self.length = end;
        Ok(())
    }

    fn nested(
        &mut self,
        id: u16,
        body: &mut dyn FnMut(&mut dyn Sink) -> Result<(), EncodeError>,
    ) -> Result<(), EncodeError> {
        let field_start = self.length;
        let mut prefix = [0_u8; TLV_PREFIX_BYTES];
        prefix[..2].copy_from_slice(&id.to_le_bytes());
        prefix[2] = WIRE_MESSAGE;
        prefix[3] = 1;
        self.raw(&prefix)?;
        let body_start = self.length;
        body(self)?;
        let body_len = self.length - body_start;
        let value_len = u32::try_from(body_len).map_err(|_| EncodeError::LimitExceeded)?;
        self.output[field_start + 4..field_start + 8].copy_from_slice(&value_len.to_le_bytes());
        self.raw(&[0_u8; 7][..padding(body_len)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_body_runs_once_per_sink_and_patches_length() {
        let limits = ProtocolLimits::default();
        let mut sizing_calls = 0;
        let mut sizing = CountSink::new(limits);
        sizing
            .nested(7, &mut |sink| {
                sizing_calls += 1;
                sink.message_header(1)?;
                sink.field(2, WIRE_U32, &9_u32.to_le_bytes())
            })
            .expect("size nested field");
        assert_eq!(sizing_calls, 1);

        let mut output = vec![0xa5; sizing.written()];
        let mut writing_calls = 0;
        let mut writer = SliceSink::new(&mut output, limits);
        writer
            .nested(7, &mut |sink| {
                writing_calls += 1;
                sink.message_header(1)?;
                sink.field(2, WIRE_U32, &9_u32.to_le_bytes())
            })
            .expect("write nested field");
        assert_eq!(writing_calls, 1);
        assert_eq!(writer.written(), sizing.written());
        assert_eq!(&output[..2], &7_u16.to_le_bytes());
        assert_eq!(output[2], WIRE_MESSAGE);
        assert_eq!(output[3], 1);
        assert_eq!(
            u32::from_le_bytes(output[4..8].try_into().expect("length")),
            24
        );
        assert!(output[28..].iter().all(|byte| *byte == 0));
    }
}
