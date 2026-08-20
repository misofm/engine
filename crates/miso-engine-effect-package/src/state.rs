use crate::PackageError;
use miso_engine_effect_contract::{EffectId, LatencySamples, TailSamples};
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatePayloadValidationError {
    Truncated,
    InvalidDirectory,
    LengthMismatch,
}

pub fn verify_lane_payload_v1(payload: &[u8]) -> Result<(), StatePayloadValidationError> {
    if payload.len() < 32 {
        return Err(StatePayloadValidationError::Truncated);
    }
    if u32::from_le_bytes(payload[0..4].try_into().expect("four bytes")) != 1
        || payload[4..8] != [0; 4]
        || payload[20..32] != [0; 12]
    {
        return Err(StatePayloadValidationError::InvalidDirectory);
    }
    let common = u32::from_le_bytes(payload[8..12].try_into().expect("four bytes")) as usize;
    let left = u32::from_le_bytes(payload[12..16].try_into().expect("four bytes")) as usize;
    let right = u32::from_le_bytes(payload[16..20].try_into().expect("four bytes")) as usize;
    let expected = 32usize
        .checked_add(common)
        .and_then(|value| value.checked_add(left))
        .and_then(|value| value.checked_add(right));
    if left != right || expected != Some(payload.len()) {
        return Err(StatePayloadValidationError::LengthMismatch);
    }
    Ok(())
}

pub fn encode_lane_payload_v1(
    common: &[u8],
    left: &[u8],
    right: &[u8],
    output: &mut Vec<u8>,
) -> Result<(), StatePayloadValidationError> {
    if left.len() != right.len()
        || [common.len(), left.len(), right.len()]
            .iter()
            .any(|length| u32::try_from(*length).is_err())
    {
        return Err(StatePayloadValidationError::LengthMismatch);
    }
    output.clear();
    output.extend_from_slice(&1u32.to_le_bytes());
    output.extend_from_slice(&[0; 4]);
    for length in [common.len(), left.len(), right.len()] {
        output.extend_from_slice(&(length as u32).to_le_bytes());
    }
    output.extend_from_slice(&[0; 12]);
    output.extend_from_slice(common);
    output.extend_from_slice(left);
    output.extend_from_slice(right);
    Ok(())
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectStateV1 {
    pub contract_minor: u16,
    pub state_schema_version: u32,
    pub sample_rate: u32,
    pub quantum: u32,
    pub quality: u32,
    pub link_mode: u32,
    pub bypass: bool,
    pub latency: LatencySamples,
    pub tail: TailSamples,
    pub effect_id: EffectId,
    pub payload: Vec<u8>,
}
pub fn encode_effect_state_v1(
    state: &EffectStateV1,
    output: &mut Vec<u8>,
) -> Result<(), PackageError> {
    verify_lane_payload_v1(&state.payload).map_err(|_| PackageError::State)?;
    output.clear();
    output.extend_from_slice(b"MISOEFST");
    output.extend_from_slice(&1u16.to_le_bytes());
    output.extend_from_slice(&144u16.to_le_bytes());
    output.extend_from_slice(&0u32.to_le_bytes());
    output.extend_from_slice(&1u16.to_le_bytes());
    output.extend_from_slice(&state.contract_minor.to_le_bytes());
    output.extend_from_slice(&state.state_schema_version.to_le_bytes());
    output.extend_from_slice(&state.sample_rate.to_le_bytes());
    output.extend_from_slice(&state.quantum.to_le_bytes());
    output.extend_from_slice(&state.quality.to_le_bytes());
    output.extend_from_slice(&state.link_mode.to_le_bytes());
    output.extend_from_slice(&(state.bypass as u32).to_le_bytes());
    output.extend_from_slice(&0u32.to_le_bytes());
    output.extend_from_slice(&state.latency.0.to_le_bytes());
    match state.tail {
        TailSamples::Finite(v) => {
            output.extend_from_slice(&1u32.to_le_bytes());
            output.extend_from_slice(&0u32.to_le_bytes());
            output.extend_from_slice(&v.to_le_bytes())
        }
        TailSamples::Infinite => {
            output.extend_from_slice(&2u32.to_le_bytes());
            output.extend_from_slice(&0u32.to_le_bytes());
            output.extend_from_slice(&0u64.to_le_bytes())
        }
    }
    output.extend_from_slice(&(state.payload.len() as u64).to_le_bytes());
    output.extend_from_slice(&Sha256::digest(state.effect_id.as_str().as_bytes()));
    output.extend_from_slice(&Sha256::digest(&state.payload));
    output.extend_from_slice(&state.payload);
    Ok(())
}
pub fn verify_effect_state_v1(bytes: &[u8]) -> Result<EffectStateV1, PackageError> {
    if bytes.len() < 144
        || &bytes[..8] != b"MISOEFST"
        || u16::from_le_bytes(bytes[8..10].try_into().unwrap()) != 1
        || u16::from_le_bytes(bytes[10..12].try_into().unwrap()) != 144
        || bytes[12..16] != [0; 4]
        || u16::from_le_bytes(bytes[16..18].try_into().unwrap()) != 1
        || bytes[44..48] != [0; 4]
        || bytes[60..64] != [0; 4]
    {
        return Err(PackageError::State);
    }
    let len = u64::from_le_bytes(bytes[72..80].try_into().unwrap()) as usize;
    if bytes.len() != 144usize.checked_add(len).ok_or(PackageError::State)? {
        return Err(PackageError::State);
    }
    let payload = bytes[144..].to_vec();
    if Sha256::digest(&payload).as_slice() != &bytes[112..144]
        || verify_lane_payload_v1(&payload).is_err()
    {
        return Err(PackageError::State);
    }
    let tail = match u32::from_le_bytes(bytes[56..60].try_into().unwrap()) {
        1 => TailSamples::Finite(u64::from_le_bytes(bytes[64..72].try_into().unwrap())),
        2 if bytes[64..72] == [0; 8] => TailSamples::Infinite,
        _ => return Err(PackageError::State),
    };
    Ok(EffectStateV1 {
        contract_minor: u16::from_le_bytes(bytes[18..20].try_into().unwrap()),
        state_schema_version: u32::from_le_bytes(bytes[20..24].try_into().unwrap()),
        sample_rate: u32::from_le_bytes(bytes[24..28].try_into().unwrap()),
        quantum: u32::from_le_bytes(bytes[28..32].try_into().unwrap()),
        quality: u32::from_le_bytes(bytes[32..36].try_into().unwrap()),
        link_mode: u32::from_le_bytes(bytes[36..40].try_into().unwrap()),
        bypass: u32::from_le_bytes(bytes[40..44].try_into().unwrap()) == 1,
        latency: LatencySamples(u64::from_le_bytes(bytes[48..56].try_into().unwrap())),
        tail,
        effect_id: EffectId::parse("unresolved").unwrap(),
        payload,
    })
}
