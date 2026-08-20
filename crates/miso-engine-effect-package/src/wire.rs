use crate::PackageError;
use miso_engine_effect_contract::EffectDescriptorV1;
pub fn canonical_effect_descriptor_v1(
    value: &'static EffectDescriptorV1,
    output: &mut Vec<u8>,
) -> Result<(), PackageError> {
    miso_engine_effect_contract::validate_descriptor_v1(value)
        .map_err(|_| PackageError::Canonical)?;
    output.clear();
    output.extend_from_slice(b"MISOEFD1");
    output.extend_from_slice(&1u16.to_le_bytes());
    output.extend_from_slice(&128u16.to_le_bytes());
    output.extend_from_slice(&[0; 4]);
    output.extend_from_slice(&1u16.to_le_bytes());
    output.extend_from_slice(&0u16.to_le_bytes());
    // Provisional issue-029 encoding remains buildable by treating the sole issue-011 current
    // layout as both bounds. Issue 029 owns and must replace this wire contract.
    output.extend_from_slice(&value.state_layout_version.to_le_bytes());
    output.extend_from_slice(&value.state_layout_version.to_le_bytes());
    output.extend_from_slice(&(value.id.as_str().len() as u32).to_le_bytes());
    output.extend_from_slice(value.id.as_str().as_bytes());
    Ok(())
}
pub fn decode_effect_descriptor_v1(
    bytes: &[u8],
    maximum_bytes: usize,
) -> Result<&[u8], PackageError> {
    if bytes.len() > maximum_bytes
        || bytes.len() < 20
        || &bytes[..8] != b"MISOEFD1"
        || u16::from_le_bytes(bytes[8..10].try_into().unwrap()) != 1
    {
        return Err(PackageError::Canonical);
    }
    Ok(bytes)
}
pub fn verify_canonical_effect_descriptor_v1(
    bytes: &[u8],
    maximum_bytes: usize,
) -> Result<(), PackageError> {
    decode_effect_descriptor_v1(bytes, maximum_bytes).map(|_| ())
}
