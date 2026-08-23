//! Shared bounded-TLV output sinks.

use crate::{EncodeError, ProtocolLimits};

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
