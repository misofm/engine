use crate::{
    EFFECT_PACKAGE_V1_UNAVAILABLE_INDEX, EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
    EffectPackageDiagnosticCodeV1, EffectPackageDiagnosticV1, EffectPackageLimitsV1,
    VerifiedEffectPackageV1, verify_effect_package_v1,
};
use core::{fmt, str::FromStr};
use sha2::{Digest, Sha256};

const CID_BYTES: usize = 36;
const CID_TEXT_BYTES: usize = 59;
const CID_PREFIX: [u8; 4] = [0x01, 0x55, 0x12, 0x20];
const BASE32: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EffectCid([u8; CID_BYTES]);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CidError {
    InvalidText,
    InvalidBinary,
    BufferTooSmall,
    Mismatch,
}

impl EffectCid {
    /// Raw-byte CID primitive retained for standard SHA-256/CID vectors.
    pub fn from_raw_bytes(bytes: &[u8]) -> Self {
        Self::from_digest(Sha256::digest(bytes).into())
    }

    fn from_digest(digest: [u8; 32]) -> Self {
        let mut binary = [0; CID_BYTES];
        binary[..4].copy_from_slice(&CID_PREFIX);
        binary[4..].copy_from_slice(&digest);
        Self(binary)
    }

    pub fn from_binary(bytes: &[u8]) -> Result<Self, CidError> {
        let binary = <[u8; CID_BYTES]>::try_from(bytes).map_err(|_| CidError::InvalidBinary)?;
        if binary[..4] != CID_PREFIX {
            return Err(CidError::InvalidBinary);
        }
        Ok(Self(binary))
    }

    pub const fn as_binary(&self) -> &[u8; CID_BYTES] {
        &self.0
    }

    pub fn write_text(&self, output: &mut [u8]) -> Result<usize, CidError> {
        if output.len() < CID_TEXT_BYTES {
            return Err(CidError::BufferTooSmall);
        }
        let mut encoded = [0; CID_TEXT_BYTES];
        encode_text(&self.0, &mut encoded);
        output[..CID_TEXT_BYTES].copy_from_slice(&encoded);
        Ok(CID_TEXT_BYTES)
    }

    /// Raw-byte comparison retained beside the official primitive vector.
    pub fn verify_raw_bytes(&self, bytes: &[u8]) -> Result<(), CidError> {
        if *self == Self::from_raw_bytes(bytes) {
            Ok(())
        } else {
            Err(CidError::Mismatch)
        }
    }

    pub fn verify_package(
        &self,
        bytes: &[u8],
        limits: EffectPackageLimitsV1,
    ) -> Result<(), EffectPackageDiagnosticV1> {
        verify_effect_package_cid_v1(bytes, limits, self).map(|_| ())
    }
}

pub fn effect_package_cid_v1(
    bytes: &[u8],
    limits: EffectPackageLimitsV1,
) -> Result<EffectCid, EffectPackageDiagnosticV1> {
    let verified = verify_effect_package_v1(bytes, limits)?;
    Ok(EffectCid::from_raw_bytes(verified.as_bytes()))
}

pub fn verify_effect_package_cid_v1<'a>(
    bytes: &'a [u8],
    limits: EffectPackageLimitsV1,
    expected: &EffectCid,
) -> Result<VerifiedEffectPackageV1<'a>, EffectPackageDiagnosticV1> {
    let verified = verify_effect_package_v1(bytes, limits)?;
    if EffectCid::from_raw_bytes(verified.as_bytes()) != *expected {
        return Err(EffectPackageDiagnosticV1::new(
            EffectPackageDiagnosticCodeV1::Cid,
            0,
            EFFECT_PACKAGE_V1_UNAVAILABLE_INDEX,
            EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
        ));
    }
    Ok(verified)
}

impl fmt::Display for EffectCid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = [0; CID_TEXT_BYTES];
        encode_text(&self.0, &mut text);
        formatter.write_str(core::str::from_utf8(&text).expect("CID encoder emits ASCII"))
    }
}

impl FromStr for EffectCid {
    type Err = CidError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let text = value.as_bytes();
        if text.len() != CID_TEXT_BYTES || text[0] != b'b' {
            return Err(CidError::InvalidText);
        }
        let mut binary = [0; CID_BYTES];
        decode_base32(&text[1..], &mut binary)?;
        let cid = Self::from_binary(&binary).map_err(|_| CidError::InvalidText)?;
        let mut canonical = [0; CID_TEXT_BYTES];
        encode_text(&binary, &mut canonical);
        if canonical != text {
            return Err(CidError::InvalidText);
        }
        Ok(cid)
    }
}

fn encode_text(binary: &[u8; CID_BYTES], output: &mut [u8; CID_TEXT_BYTES]) {
    output[0] = b'b';
    let mut accumulator = 0u16;
    let mut bits = 0u8;
    let mut cursor = 1usize;
    for byte in binary {
        accumulator = (accumulator << 8) | u16::from(*byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            output[cursor] = BASE32[((accumulator >> bits) & 31) as usize];
            cursor += 1;
        }
    }
    if bits != 0 {
        output[cursor] = BASE32[((accumulator << (5 - bits)) & 31) as usize];
        cursor += 1;
    }
    debug_assert_eq!(cursor, CID_TEXT_BYTES);
}

fn decode_base32(text: &[u8], output: &mut [u8; CID_BYTES]) -> Result<(), CidError> {
    if text.len() != CID_TEXT_BYTES - 1 {
        return Err(CidError::InvalidText);
    }
    let mut accumulator = 0u16;
    let mut bits = 0u8;
    let mut cursor = 0usize;
    for byte in text {
        let value = match byte {
            b'a'..=b'z' => byte - b'a',
            b'2'..=b'7' => byte - b'2' + 26,
            _ => return Err(CidError::InvalidText),
        };
        accumulator = (accumulator << 5) | u16::from(value);
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            if cursor >= output.len() {
                return Err(CidError::InvalidText);
            }
            output[cursor] = (accumulator >> bits) as u8;
            cursor += 1;
        }
    }
    if cursor != output.len() || bits == 0 || accumulator & ((1u16 << bits) - 1) != 0 {
        return Err(CidError::InvalidText);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        EffectArtifactAuthoringV1, EffectArtifactKindV1, EffectPackageAuthoringV1,
        effect_package_v1_required_size, encode_effect_package_v1,
    };

    fn descriptor() -> Vec<u8> {
        let compact: Vec<_> =
            include_str!("../../../fixtures/effect-descriptor/v1/comprehensive-a.wire.hex")
                .bytes()
                .filter(|byte| !byte.is_ascii_whitespace())
                .collect();
        compact
            .chunks_exact(2)
            .map(|pair| {
                let digit = |byte: u8| match byte {
                    b'0'..=b'9' => byte - b'0',
                    b'a'..=b'f' => byte - b'a' + 10,
                    _ => panic!("fixture is lowercase hexadecimal"),
                };
                digit(pair[0]) << 4 | digit(pair[1])
            })
            .collect()
    }

    fn package() -> Vec<u8> {
        let descriptor = descriptor();
        let artifacts = [EffectArtifactAuthoringV1 {
            kind: EffectArtifactKindV1::Source,
            path: "src/lib.rs",
            target: "",
            features: "",
            content: b"source",
        }];
        let authoring = EffectPackageAuthoringV1 {
            descriptor: &descriptor,
            artifacts: &artifacts,
        };
        let required = effect_package_v1_required_size(&authoring, EffectPackageLimitsV1::default())
            .unwrap() as usize;
        let mut bytes = vec![0; required];
        encode_effect_package_v1(&authoring, EffectPackageLimitsV1::default(), &mut bytes).unwrap();
        bytes
    }

    #[test]
    fn official_hello_vector_and_fixed_layout_are_exact() {
        let cid = EffectCid::from_raw_bytes(b"hello");
        assert_eq!(core::mem::size_of::<EffectCid>(), CID_BYTES);
        assert_eq!(&cid.as_binary()[..4], &CID_PREFIX);
        assert_eq!(
            cid.to_string(),
            "bafkreibm6jg3ux5qumhcn2b3flc3tyu6dmlb4xa7u5bf44yegnrjhc4yeq"
        );
    }

    #[test]
    fn writer_is_atomic_and_preserves_trailing_canary() {
        let cid = EffectCid::from_raw_bytes(b"hello");
        let mut short = [0xa5; CID_TEXT_BYTES - 1];
        assert_eq!(cid.write_text(&mut short), Err(CidError::BufferTooSmall));
        assert_eq!(short, [0xa5; CID_TEXT_BYTES - 1]);
        let mut output = [0xa5; CID_TEXT_BYTES + 3];
        assert_eq!(cid.write_text(&mut output), Ok(CID_TEXT_BYTES));
        assert_eq!(&output[..CID_TEXT_BYTES], cid.to_string().as_bytes());
        assert_eq!(&output[CID_TEXT_BYTES..], &[0xa5; 3]);
    }

    #[test]
    fn binary_codec_rejects_length_prefix_codec_hash_and_digest_length() {
        let cid = EffectCid::from_raw_bytes(b"hello");
        assert_eq!(EffectCid::from_binary(cid.as_binary()), Ok(cid));
        for length in 0..CID_BYTES {
            assert_eq!(
                EffectCid::from_binary(&cid.as_binary()[..length]),
                Err(CidError::InvalidBinary),
                "length {length}"
            );
        }
        let mut trailing = cid.as_binary().to_vec();
        trailing.push(0);
        assert_eq!(
            EffectCid::from_binary(&trailing),
            Err(CidError::InvalidBinary)
        );
        for offset in 0..4 {
            for replacement in u8::MIN..=u8::MAX {
                if replacement != cid.as_binary()[offset] {
                    let mut binary = *cid.as_binary();
                    binary[offset] = replacement;
                    assert_eq!(
                        EffectCid::from_binary(&binary),
                        Err(CidError::InvalidBinary),
                        "offset {offset}, replacement {replacement}"
                    );
                }
            }
        }
    }

    #[test]
    fn text_codec_rejects_every_alphabet_prefix_length_and_pad_bit_class() {
        let cid = EffectCid::from_raw_bytes(b"hello");
        let canonical = cid.to_string();
        assert_eq!(canonical.parse::<EffectCid>(), Ok(cid));
        for length in 0..CID_TEXT_BYTES {
            assert_eq!(
                canonical[..length].parse::<EffectCid>(),
                Err(CidError::InvalidText),
                "length {length}"
            );
        }
        assert_eq!(
            format!("{canonical}a").parse::<EffectCid>(),
            Err(CidError::InvalidText)
        );
        for replacement in *b"Ba108=+/" {
            let mut text = canonical.as_bytes().to_owned();
            text[0] = replacement;
            assert_eq!(
                core::str::from_utf8(&text).unwrap().parse::<EffectCid>(),
                Err(CidError::InvalidText)
            );
        }
        for position in 1..CID_TEXT_BYTES {
            for replacement in *b"A0189=+/_" {
                let mut text = canonical.as_bytes().to_owned();
                text[position] = replacement;
                assert_eq!(
                    core::str::from_utf8(&text).unwrap().parse::<EffectCid>(),
                    Err(CidError::InvalidText),
                    "position {position}, replacement {replacement}"
                );
            }
        }
        let last_value = BASE32
            .iter()
            .position(|byte| *byte == canonical.as_bytes()[58])
            .unwrap();
        assert_eq!(last_value & 3, 0);
        for pad_bits in 1..=3 {
            let mut nonzero_pad = canonical.as_bytes().to_owned();
            nonzero_pad[58] = BASE32[(last_value & !3) | pad_bits];
            assert_eq!(
                core::str::from_utf8(&nonzero_pad)
                    .unwrap()
                    .parse::<EffectCid>(),
                Err(CidError::InvalidText),
                "pad bits {pad_bits}"
            );
        }
    }

    #[test]
    fn package_creation_and_expected_verification_are_strict() {
        let bytes = package();
        let cid = effect_package_cid_v1(&bytes, EffectPackageLimitsV1::default()).unwrap();
        assert_eq!(cid, EffectCid::from_raw_bytes(&bytes));
        let verified =
            verify_effect_package_cid_v1(&bytes, EffectPackageLimitsV1::default(), &cid).unwrap();
        assert_eq!(verified.as_bytes().as_ptr(), bytes.as_ptr());
        cid.verify_package(&bytes, EffectPackageLimitsV1::default())
            .unwrap();
        let wrong = EffectCid::from_raw_bytes(b"wrong");
        let error = verify_effect_package_cid_v1(&bytes, EffectPackageLimitsV1::default(), &wrong)
            .unwrap_err();
        assert_eq!(error.code, EffectPackageDiagnosticCodeV1::Cid);
        assert_eq!(error.artifact_index, u32::MAX);
        assert_eq!(error.byte_offset, u64::MAX);
        let mut invalid = bytes.clone();
        invalid[0] ^= 1;
        assert_eq!(
            effect_package_cid_v1(&invalid, EffectPackageLimitsV1::default())
                .unwrap_err()
                .code,
            EffectPackageDiagnosticCodeV1::Header
        );
    }
}
