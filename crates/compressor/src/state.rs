//! Exact causal compressor state payload codec.
//!
//! Each channel is 22 little-endian words (88 bytes): word 0 is gain reduction, followed by
//! current/target/remaining for each of the seven smoothed parameters.  There is no cursor,
//! lookahead value, audio ring, detector ring, or staging storage in the current payload.

use effect_contract::{StatePayloadError, StatePayloadOutput, StatePayloadSizes};
use effect_runtime::params::{is_negative_zero, normalize_zero, parameter_value_valid};
use effect_runtime::ramp::LinearRamp;
use effect_runtime::state_payload::{read_f32, read_u32, write_f32, write_u32};
use lane::Lane;

use crate::design::{MAX_WIDTH, PARAMETER_SPECS, RAMP_COUNT, SMOOTHING_SAMPLES};
use crate::kernel::Channel;

/// Fixed scalar words in one channel section.
pub const STATE_HEADER_WORDS: usize = 22;

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

pub(crate) fn validate_lengths(
    common_bytes: usize,
    left_bytes: usize,
    right_bytes: usize,
    sizes: StatePayloadSizes,
) -> Result<(), StatePayloadError> {
    if common_bytes != sizes.common_bytes as usize
        || left_bytes != sizes.left_bytes as usize
        || right_bytes != sizes.right_bytes as usize
    {
        return Err(state_error("effect.state.length"));
    }
    Ok(())
}

fn normal_or_zero(value: f32) -> bool {
    let magnitude = value.abs();
    value == 0.0 || (f32::MIN_POSITIVE..=f32::MAX).contains(&magnitude)
}

fn parameter_state_valid(index: usize, value: f32) -> bool {
    !is_negative_zero(value) && parameter_value_valid(&PARAMETER_SPECS[index], value)
}

pub(crate) fn write_channel<L: Lane>(bytes: &mut [u8], channel: &Channel<L>, lane: usize) {
    let mut reduction = [0.0_f32; MAX_WIDTH];
    channel.gain_reduction_db.store(&mut reduction);
    write_f32(bytes, 0, reduction[lane]);
    for (index, parameter) in channel.ramps.iter().enumerate() {
        let word = 1 + index * 3;
        write_f32(bytes, word, parameter[lane].current);
        write_f32(bytes, word + 1, parameter[lane].target);
        write_u32(bytes, word + 2, parameter[lane].remaining);
    }
}

pub(crate) fn validate_channel(bytes: &[u8]) -> Result<(), StatePayloadError> {
    let expected = STATE_HEADER_WORDS
        .checked_mul(4)
        .ok_or(state_error("effect.state.length"))?;
    if bytes.len() != expected {
        return Err(state_error("effect.state.length"));
    }
    let reduction = read_f32(bytes, 0);
    if !(normal_or_zero(reduction) && (-100.0..=0.0).contains(&reduction)) {
        return Err(state_error("effect.state.gain"));
    }
    for index in 0..RAMP_COUNT {
        let word = 1 + index * 3;
        if !parameter_state_valid(index, read_f32(bytes, word))
            || !parameter_state_valid(index, read_f32(bytes, word + 1))
            || read_u32(bytes, word + 2) > SMOOTHING_SAMPLES
        {
            return Err(state_error("effect.state.parameter"));
        }
    }
    Ok(())
}

/// Commits an already validated channel section.  Validation of both channels happens before
/// either commit, so a malformed right section leaves the complete destination unchanged.
pub(crate) fn commit_channel<L: Lane>(
    bytes: &[u8],
    channel: &mut Channel<L>,
    lane: usize,
    sample_rate: u32,
) {
    let mut reduction = [0.0_f32; MAX_WIDTH];
    channel.gain_reduction_db.store(&mut reduction);
    reduction[lane] = normalize_zero(read_f32(bytes, 0));
    channel.gain_reduction_db = L::load(&reduction);

    for (index, parameter) in channel.ramps.iter_mut().enumerate() {
        let word = 1 + index * 3;
        let current = read_f32(bytes, word);
        let target = read_f32(bytes, word + 1);
        let remaining = read_u32(bytes, word + 2);
        let step = if remaining == 0 {
            0.0
        } else {
            (target - current) / remaining as f32
        };
        parameter[lane] = LinearRamp {
            current,
            target,
            step,
            remaining,
        };
    }
    channel.redesign(lane, sample_rate);
}

pub(crate) fn snapshot_lane<L: Lane>(
    output: &mut StatePayloadOutput<'_>,
    left: &Channel<L>,
    right: &Channel<L>,
    lane: usize,
) {
    write_channel(output.left, left, lane);
    write_channel(output.right, right, lane);
}
