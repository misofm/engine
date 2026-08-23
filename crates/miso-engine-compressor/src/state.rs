//! State payload: the frozen V1 word layout, on the effect runtime's little-endian word codec.
//!
//! The layout is a **contract fixture** and does not change here (master plan §8.2): per lane,
//! `24 + 2B` little-endian words in the order
//!
//! ```text
//!   0        cursor                    u32, < B
//!   1        lookahead_ms              f32, in the parameter domain, not -0.0
//!   2        gain_reduction_db         f32, normal or zero, in [-100, 0]
//!   3 + 3i   ramp i current            f32, in parameter i's domain      (i = 0..6)
//!   4 + 3i   ramp i target             f32, in parameter i's domain
//!   5 + 3i   ramp i remaining          u32, <= 64
//!  24 .. 24+B    main ring             f32, normal or zero
//!  24+B .. 24+2B detector ring         f32, normal or zero
//! ```
//!
//! 83c's `state_payload::{snapshot, restore}` stamp a two-word version/length header into the
//! **common** section. The compressor's `common_bytes` is `0` in every `QualityDescriptorV1` row,
//! and #88's plan freezes the payload layout and the descriptor rows, so adopting the header would
//! change a contract fixture this job is explicitly forbidden from touching. What is adopted is
//! the part that is pure arithmetic: the word codec (`write_u32`/`write_f32`/`read_u32`/
//! `read_f32`), and exact-length validation — `!=`, never `<`, so a payload with trailing bytes is
//! rejected rather than silently truncated. Wiring the versioned header (and the
//! `state_layout_version` bump and `maximum_state` change it implies) belongs to #95, which owns
//! the program key and the contract cleanup.
//!
//! # Transactional restore
//!
//! Every section is validated before anything is written, and the commit is infallible, so a
//! payload that is bad in its last word leaves the effect exactly as it was — including the case
//! where the left channel is valid and the right is not. The pre-audit code achieved that by
//! parsing into a freshly allocated `Lane` and moving it; this version validates in a read-only
//! pass and then commits in place, so a restore allocates nothing at all.

use miso_engine_effect_contract::{StatePayloadError, StatePayloadOutput, StatePayloadSizes};
use miso_engine_effect_runtime::params::{is_negative_zero, normalize_zero, parameter_value_valid};
use miso_engine_effect_runtime::ramp::LinearRamp;
use miso_engine_effect_runtime::state_payload::{read_f32, read_u32, write_f32, write_u32};
use miso_engine_lane::Lane;

use crate::design::{MAX_WIDTH, PARAMETER_SPECS, RAMP_COUNT, SMOOTHING_SAMPLES};
use crate::kernel::Channel;

/// Fixed scalar words before the two ring arrays. Not a header: nothing is stamped into it.
pub const STATE_HEADER_WORDS: usize = 24;

/// The payload words one lane occupies.
pub(crate) const fn lane_words(ring_length: usize) -> usize {
    STATE_HEADER_WORDS + 2 * ring_length
}

/// Builds a diagnostic. The codes are the ones the pre-audit crate used and are part of the
/// contract's failure vocabulary.
const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

/// The three sections must have exactly the lengths the prepared metadata declares.
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

/// `value` is finite and either zero or a normal.
///
/// Written as magnitude comparisons rather than `is_finite() && (v == 0.0 || v.is_normal())`
/// because this crate carries no per-value finiteness predicate any more (D7) and a reader should
/// not have to decide whether one on the control plane is the same thing as one on the render
/// path. `abs() <= f32::MAX` is false for infinities and for NaN; `abs() >= f32::MIN_POSITIVE`
/// excludes the subnormals.
fn normal_or_zero(value: f32) -> bool {
    let magnitude = value.abs();
    value == 0.0 || (f32::MIN_POSITIVE..=f32::MAX).contains(&magnitude)
}

/// A parameter word is valid if it is in that parameter's domain and is not `-0.0`.
///
/// `-0.0` is rejected on the way *in* from a payload — unlike a control message, where the runtime
/// rule is to normalise it — because the payload is this effect's own output and a `-0.0` in it
/// means the bytes did not come from a snapshot of a healthy instance.
pub(crate) fn parameter_state_valid(index: usize, value: f32) -> bool {
    !is_negative_zero(value) && parameter_value_valid(&PARAMETER_SPECS[index], value)
}

/// Writes one lane's state into a channel section.
pub(crate) fn write_channel<L: Lane>(bytes: &mut [u8], channel: &Channel<L>, lane: usize) {
    let width = L::WIDTH;
    let ring_length = channel.ring_length as usize;
    let mut reduction = [0.0_f32; MAX_WIDTH];
    channel.gain_reduction_db.store(&mut reduction);

    write_u32(bytes, 0, channel.cursor);
    write_f32(bytes, 1, channel.lookahead_ms[lane]);
    write_f32(bytes, 2, reduction[lane]);
    for (index, parameter) in channel.ramps.iter().enumerate() {
        let word = 3 + index * 3;
        write_f32(bytes, word, parameter[lane].current);
        write_f32(bytes, word + 1, parameter[lane].target);
        write_u32(bytes, word + 2, parameter[lane].remaining);
    }
    for index in 0..ring_length {
        write_f32(
            bytes,
            STATE_HEADER_WORDS + index,
            channel.main[index * width + lane],
        );
        write_f32(
            bytes,
            STATE_HEADER_WORDS + ring_length + index,
            channel.detector[index * width + lane],
        );
    }
}

/// Validates one channel section without writing anything.
pub(crate) fn validate_channel(bytes: &[u8], ring_length: usize) -> Result<(), StatePayloadError> {
    let expected = lane_words(ring_length)
        .checked_mul(4)
        .ok_or(state_error("effect.state.length"))?;
    if bytes.len() != expected {
        return Err(state_error("effect.state.length"));
    }
    if read_u32(bytes, 0) as usize >= ring_length {
        return Err(state_error("effect.state.cursor"));
    }
    if !parameter_state_valid(7, read_f32(bytes, 1)) {
        return Err(state_error("effect.state.parameter"));
    }
    let reduction = read_f32(bytes, 2);
    if !(normal_or_zero(reduction) && (-100.0..=0.0).contains(&reduction)) {
        return Err(state_error("effect.state.gain"));
    }
    for index in 0..RAMP_COUNT {
        let word = 3 + index * 3;
        if !parameter_state_valid(index, read_f32(bytes, word))
            || !parameter_state_valid(index, read_f32(bytes, word + 1))
            || read_u32(bytes, word + 2) > SMOOTHING_SAMPLES
        {
            return Err(state_error("effect.state.parameter"));
        }
    }
    for index in 0..2 * ring_length {
        if !normal_or_zero(read_f32(bytes, STATE_HEADER_WORDS + index)) {
            return Err(state_error("effect.state.ring"));
        }
    }
    Ok(())
}

/// Commits a section that [`validate_channel`] has already accepted.
///
/// The ramp `step` is not serialised — the layout is frozen and adding a word would change a
/// contract fixture — so it is re-derived as `(target - current) / remaining`. That makes a
/// **mid-ramp** restore a class-B change: the pre-audit law recomputed the same quotient every
/// sample, and D11's law computes it once at the event, so a ramp restored with `remaining = 37`
/// continues on the quotient of the remaining distance rather than on the original 1/64 step. It
/// still lands exactly on the target on the same sample (gate E7). A restore at rest
/// (`remaining == 0`, which is every sample outside a 64-sample window after an event) is
/// bit-exact.
pub(crate) fn commit_channel<L: Lane>(
    bytes: &[u8],
    channel: &mut Channel<L>,
    lane: usize,
    sample_rate: u32,
) {
    let width = L::WIDTH;
    let ring_length = channel.ring_length as usize;

    channel.cursor = read_u32(bytes, 0);
    let lookahead_ms = read_f32(bytes, 1);
    channel.lookahead_ms[lane] = lookahead_ms;
    channel.delay[lane] = crate::design::detector_delay(lookahead_ms, sample_rate, ring_length);

    let mut reduction = [0.0_f32; MAX_WIDTH];
    channel.gain_reduction_db.store(&mut reduction);
    reduction[lane] = normalize_zero(read_f32(bytes, 2));
    channel.gain_reduction_db = L::load(&reduction);

    for (index, parameter) in channel.ramps.iter_mut().enumerate() {
        let word = 3 + index * 3;
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
    for index in 0..ring_length {
        channel.main[index * width + lane] = read_f32(bytes, STATE_HEADER_WORDS + index);
        channel.detector[index * width + lane] =
            read_f32(bytes, STATE_HEADER_WORDS + ring_length + index);
    }
    channel.redesign(lane, sample_rate);
}

/// Writes both channel sections of one lane and leaves the (empty) common section alone.
pub(crate) fn snapshot_lane<L: Lane>(
    output: &mut StatePayloadOutput<'_>,
    left: &Channel<L>,
    right: &Channel<L>,
    lane: usize,
) {
    write_channel(output.left, left, lane);
    write_channel(output.right, right, lane);
}
