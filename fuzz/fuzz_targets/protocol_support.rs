use miso_engine_protocol::{DecodeError, DecodeScratch, ProtocolCodec, ProtocolLimits};

pub const MAX_FRAME_BYTES: usize = 4096;

pub fn codec() -> ProtocolCodec {
    ProtocolCodec::new(ProtocolLimits {
        max_frame_bytes: MAX_FRAME_BYTES,
        max_tlv_count: 64,
        max_string_bytes: 512,
        max_nesting: 4,
    })
}

/// Corpus files use an ASCII `hex:` prefix so valid BTLV seeds remain reviewable in source.
/// Arbitrary libFuzzer bytes remain raw input if the prefix is absent or malformed.
pub fn corpus_or_raw<'a>(input: &'a [u8], output: &'a mut [u8; MAX_FRAME_BYTES]) -> &'a [u8] {
    let Some(hex) = input.strip_prefix(b"hex:") else {
        return input;
    };
    if hex.len() % 2 != 0 || hex.len() / 2 > output.len() {
        return input;
    }
    for (index, pair) in hex.chunks_exact(2).enumerate() {
        let Some(high) = nibble(pair[0]) else {
            return input;
        };
        let Some(low) = nibble(pair[1]) else {
            return input;
        };
        output[index] = (high << 4) | low;
    }
    &output[..hex.len() / 2]
}

#[allow(dead_code)]
pub fn command_class(input: &[u8]) -> Result<(), DecodeError> {
    let codec = codec();
    let mut fields = [0_u16; 64];
    codec
        .decode_typed_command(input, &mut DecodeScratch::new(&mut fields))
        .map(|_| ())
}

#[allow(dead_code)]
pub fn response_class(input: &[u8]) -> Result<(), DecodeError> {
    let codec = codec(); let mut fields = [0_u16; 64];
    codec.decode_typed_response(input, &mut DecodeScratch::new(&mut fields)).map(|_| ())
}

#[allow(dead_code)]
pub fn event_class(input: &[u8]) -> Result<(), DecodeError> {
    let codec = codec(); let mut fields = [0_u16; 64];
    codec.decode_typed_event(input, &mut DecodeScratch::new(&mut fields)).map(|_| ())
}

#[allow(dead_code)] // Each libFuzzer binary selects exactly one of these decoder classes.
pub fn transaction_class(input: &[u8]) -> Result<(), DecodeError> {
    let codec = codec();
    let mut fields = [0_u16; 64];
    codec
        .decode_session_transaction(input, &mut DecodeScratch::new(&mut fields))
        .map(|_| ())
}

pub fn assert_stable(classify: impl Fn(&[u8]) -> Result<(), DecodeError>, input: &[u8]) {
    let first = classify(input);
    let second = classify(input);
    assert_eq!(first, second, "same bytes must produce one stable decode class");
    if input.len() > MAX_FRAME_BYTES {
        assert_eq!(first, Err(DecodeError::LimitExceeded));
    }
}

const fn nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}
