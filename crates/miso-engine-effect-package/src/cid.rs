use core::{fmt, str::FromStr};
use sha2::{Digest, Sha256};
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EffectCid([u8; 36]);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CidError {
    InvalidText,
    InvalidBinary,
    Mismatch,
}
impl EffectCid {
    pub fn from_package_bytes(bytes: &[u8]) -> Self {
        let digest = Sha256::digest(bytes);
        let mut out = [0u8; 36];
        out[..4].copy_from_slice(&[1, 0x55, 0x12, 0x20]);
        out[4..].copy_from_slice(&digest);
        Self(out)
    }
    pub fn from_binary(bytes: &[u8]) -> Result<Self, CidError> {
        let out = <[u8; 36]>::try_from(bytes).map_err(|_| CidError::InvalidBinary)?;
        if out[..4] != [1, 0x55, 0x12, 0x20] {
            return Err(CidError::InvalidBinary);
        }
        Ok(Self(out))
    }
    pub fn as_binary(&self) -> &[u8; 36] {
        &self.0
    }
    pub fn verify(&self, bytes: &[u8]) -> Result<(), CidError> {
        if *self == Self::from_package_bytes(bytes) {
            Ok(())
        } else {
            Err(CidError::Mismatch)
        }
    }
}
impl fmt::Display for EffectCid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("b")?;
        f.write_str(&base32(&self.0))
    }
}
impl FromStr for EffectCid {
    type Err = CidError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 59
            || !value.starts_with('b')
            || value.as_bytes().iter().any(|b| b.is_ascii_uppercase())
        {
            return Err(CidError::InvalidText);
        }
        let bytes = unbase32(&value.as_bytes()[1..])?;
        let cid = Self::from_binary(&bytes)?;
        if cid.to_string() != value {
            return Err(CidError::InvalidText);
        }
        Ok(cid)
    }
}
fn base32(bytes: &[u8]) -> String {
    const A: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";
    let mut result = String::with_capacity((bytes.len() * 8).div_ceil(5));
    let mut acc = 0u16;
    let mut bits = 0;
    for byte in bytes {
        acc = (acc << 8) | u16::from(*byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            result.push(char::from(A[((acc >> bits) & 31) as usize]));
        }
    }
    if bits != 0 {
        result.push(char::from(A[((acc << (5 - bits)) & 31) as usize]));
    }
    result
}
fn unbase32(text: &[u8]) -> Result<Vec<u8>, CidError> {
    let mut out = Vec::with_capacity(text.len() * 5 / 8);
    let mut acc = 0u16;
    let mut bits = 0;
    for c in text {
        let value = match c {
            b'a'..=b'z' => c - b'a',
            b'2'..=b'7' => c - b'2' + 26,
            _ => return Err(CidError::InvalidText),
        };
        acc = (acc << 5) | u16::from(value);
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
        }
    }
    if bits != 0 && (acc & ((1 << bits) - 1)) != 0 {
        return Err(CidError::InvalidText);
    }
    Ok(out)
}
