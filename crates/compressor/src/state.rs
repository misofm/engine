//! Exact causal compressor state payload codec.
//!
//! Each channel is 37 little-endian words (148 bytes) (#1278 D2a):
//!
//! ```text
//! [0]        gain reduction, dB
//! [1 .. 29]  the seven smoothed parameters' ramps: current, target, step, remaining
//! [29 .. 37] the attack and release coefficient ramps: current, target, step, remaining
//! ```
//!
//! A payload carries every word a continuation reads, the ramps' steps included, so a lane
//! restored mid-ramp continues bit for bit like the lane it was taken from. The 22-word layout before it carried each
//! ramp's current, target and remaining, re-derived the step, and reconstructed the coefficient
//! ramps from the parameter ramps, so a mid-ramp restore landed on the same endpoints by another
//! path. The coefficient words are the design of the parameters' current values, except attack
//! and release, which are their coefficient ramps' current values.
//!
//! Restore validates both channels in place, reading the payload bytes, then commits by reading
//! them again: no heap value is created or dropped (#1278 D1), and the four payload calls are
//! render-safe.

use effect_contract::{StatePayloadError, StatePayloadOutput, StatePayloadSizes};
use effect_runtime::params::{is_negative_zero, normalize_zero, parameter_value_valid};
use effect_runtime::state_payload::{
    RAMP_WORDS, ramp_path_inside, read_f32, read_ramp, write_f32, write_ramp,
};
use lane::Lane;

use crate::design::{
    ALL_PARAMETERS, COEF_ATTACK, COEF_RELEASE, MAX_WIDTH, PARAMETER_SPECS, RAMP_COUNT,
    SMOOTHING_SAMPLES, design_lane,
};
use crate::kernel::Channel;

/// Words in one channel section.
pub const STATE_HEADER_WORDS: usize = 1 + RAMP_WORDS * (RAMP_COUNT + RATE_RAMPS);

/// The payload layout version.
///
/// #1278 D2a grew the layout from 22 to 37 words (every ramp's step and the coefficient ramps)
/// without a bump: AGENTS.md gives a contract version its sole prelaunch identity, V1, and
/// `effect-compiler`'s `launch_native_state_layouts_are_v1` holds every launch-native layout
/// there. Nothing persists a payload (R6b), so no reader of the 22-word layout exists; the
/// section lengths, which `maximum_state` declares, refuse one.
pub(crate) const STATE_LAYOUT_VERSION: u32 = 1;

/// The attack and release coefficient ramps.
const RATE_RAMPS: usize = 2;

/// First word of the parameter ramps.
const PARAMETER_RAMP_WORD: usize = 1;

/// First word of the coefficient ramps.
const RATE_RAMP_WORD: usize = PARAMETER_RAMP_WORD + RAMP_WORDS * RAMP_COUNT;

/// The parameter mask a restore designs from the current values: every smoothed parameter but
/// attack and release, whose words are their coefficient ramps' current values.
const DESIGNED_ON_RESTORE: u8 = ALL_PARAMETERS & !((1 << 3) | (1 << 4));

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

// REALTIME_POLICY_BEGIN: #1278 D3, the payload codec runs in the plan-swap block.
pub(crate) fn write_channel<L: Lane>(bytes: &mut [u8], channel: &Channel<L>, lane: usize) {
    let mut reduction = [0.0_f32; MAX_WIDTH];
    channel.gain_reduction_db.store(&mut reduction);
    write_f32(bytes, 0, reduction[lane]);
    for (index, parameter) in channel.ramps.iter().enumerate() {
        write_ramp(
            bytes,
            PARAMETER_RAMP_WORD + index * RAMP_WORDS,
            parameter[lane],
        );
    }
    for (index, ramp) in channel.rate_ramps.iter().enumerate() {
        write_ramp(bytes, RATE_RAMP_WORD + index * RAMP_WORDS, ramp[lane]);
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
    for (index, spec) in PARAMETER_SPECS.iter().enumerate() {
        let ramp = read_ramp(bytes, PARAMETER_RAMP_WORD + index * RAMP_WORDS);
        // The target is a designed value and lies in the domain exactly. A `current`, moving or
        // settled, is held to the strict domain too: the clamped law keeps every word the
        // effect's own ramps produce between its start and its target, both in the domain
        // (issue #1409 D2), so a word outside it is one the engine never holds (issue #1411 D1).
        if !parameter_state_valid(index, ramp.current)
            || !parameter_state_valid(index, ramp.target)
            || !ramp_path_inside(ramp, (spec.minimum, spec.maximum), SMOOTHING_SAMPLES)
        {
            return Err(state_error("effect.state.parameter"));
        }
    }
    // `design::rate_coefficient` designs every coefficient of a legal time in `(0, 1]`: it gives
    // exactly `0.0` only for an infinite time, which the domains exclude, and its smallest legal
    // coefficient (5 s at 96 kHz) is about 2.1e-6. A target is a designed value, held there
    // exactly; a settled `0.0` would freeze the smoother until the next retarget, and the smoother
    // `y += c (x - y)` diverges for every `c < 0` (and for `c > 2`). A `current`, moving or
    // settled, lies between two designed values under the clamped law (issue #1409 D2), so it is
    // held to the same designed range, with no rounding budget (issue #1411 D1).
    for index in 0..RATE_RAMPS {
        let ramp = read_ramp(bytes, RATE_RAMP_WORD + index * RAMP_WORDS);
        let designed = |value: f32| value > 0.0 && value <= 1.0;
        if !designed(ramp.target)
            || !designed(ramp.current)
            || !ramp_path_inside(ramp, (0.0, 1.0), SMOOTHING_SAMPLES)
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
        parameter[lane] = read_ramp(bytes, PARAMETER_RAMP_WORD + index * RAMP_WORDS);
    }
    for (index, ramp) in channel.rate_ramps.iter_mut().enumerate() {
        ramp[lane] = read_ramp(bytes, RATE_RAMP_WORD + index * RAMP_WORDS);
    }
    let values = channel.current_values(lane);
    design_lane(
        &values,
        sample_rate,
        DESIGNED_ON_RESTORE,
        &mut channel.words,
        lane,
    );
    channel.words[COEF_ATTACK][lane] = channel.rate_ramps[0][lane].current;
    channel.words[COEF_RELEASE][lane] = channel.rate_ramps[1][lane].current;
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
// REALTIME_POLICY_END
