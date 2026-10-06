//! One versioned little-endian word codec for every effect's state payload.
//!
//! The audit found this codec copied into seven effect crates, byte for byte in the arithmetic and
//! divergent everywhere else: four different `.expect()` strings, two spellings of the same length
//! check, one crate whose `read_lane` re-derives its own length and one that does not, and a
//! diagnostic vocabulary that had grown crate-private codes. This module is the single copy.
//!
//! # Layout
//!
//! A payload is three byte sections — one common and one per channel — each a whole number of
//! 4-byte **words**, little-endian, with `f32` stored as its raw `to_bits` pattern so a snapshot
//! round-trips exactly, signed zeros and all.
//!
//! ```text
//! common: [0] version  [1] data word count  [2 ..] the effect's common words
//! left:   [0 ..] the effect's left-channel words
//! right:  [0 ..] the effect's right-channel words   (same length as left)
//! ```
//!
//! The two header words are the change this module makes to what the effects do today: a payload
//! carried its version **out of band**, as the `state_layout_version` argument of
//! `restore_state_payload`, and its length out of band as the descriptor's `maximum_state`. That
//! works only while the caller is the same build as the writer. Stamping the version and the word
//! count into the bytes makes a payload self-describing, so a restore can reject a stale or
//! truncated payload on its own evidence. The out-of-band version argument stays — it is the
//! contract's — and [`read_header`] checks the two against each other.
//!
//! # Diagnostics
//!
//! Only the two codes every effect already agrees on are defined here: [`STATE_LENGTH_CODE`] and
//! [`STATE_VERSION_CODE`]. The crate-private codes the audit found — `effect.state.ring`,
//! `effect.state.history`, `effect.state.payload`, `effect.state.envelope`, `effect.state.phase`
//! — describe an effect's own value validation, which stays with the effect.
//!
//! # Ramps
//!
//! [`write_ramp`], [`read_ramp`] and [`ramp_path_inside`] carry a [`LinearRamp`] as four words,
//! its step included, so a restored mid-flight ramp continues bit for bit (#1278).
//!
//! # Endianness
//!
//! Little-endian, unconditionally, on every target. A state payload crosses hosts and
//! architectures; making it depend on the writer's byte order would make a session non-portable.

use crate::ramp::LinearRamp;

/// The payload's length did not match what its layout requires.
pub const STATE_LENGTH_CODE: &str = "effect.state.length";

/// The payload's version did not match the layout being restored into.
pub const STATE_VERSION_CODE: &str = "effect.state.version";

/// Words reserved at the front of the common section: the version and the data word count.
pub const HEADER_WORDS: u32 = 2;

/// Size of one word in bytes.
pub const WORD_BYTES: usize = 4;

/// A state payload was rejected.
///
/// Carries a stable diagnostic code rather than a variant, matching the contract's
/// `StatePayloadError`, so an effect can return it unchanged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatePayloadError {
    /// Stable diagnostic code: [`STATE_LENGTH_CODE`] or [`STATE_VERSION_CODE`].
    pub code: &'static str,
}

impl StatePayloadError {
    /// The length error.
    #[must_use]
    pub const fn length() -> Self {
        Self {
            code: STATE_LENGTH_CODE,
        }
    }

    /// The version error.
    #[must_use]
    pub const fn version() -> Self {
        Self {
            code: STATE_VERSION_CODE,
        }
    }
}

/// The shape of one effect's state payload.
///
/// `common_words` and `lane_words` count the effect's **data** words; the two header words are
/// added by [`expected_sizes`] and are not the effect's to write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateLayout {
    /// Layout version, stamped into word 0 of the common section.
    pub version: u32,
    /// Effect-owned words in the common section — the state both channels share.
    pub common_words: u32,
    /// Effect-owned words in each channel section. Left and right are always the same length.
    pub lane_words: u32,
}

impl StateLayout {
    /// Total number of effect-owned data words, `common_words + 2 * lane_words`.
    ///
    /// This is the value stamped into word 1 of the common section.
    #[must_use]
    pub const fn data_words(&self) -> u32 {
        self.common_words + 2 * self.lane_words
    }
}

/// Byte lengths of the three sections of a payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatePayloadSizes {
    /// `4 * (HEADER_WORDS + common_words)`.
    pub common: usize,
    /// `4 * lane_words`.
    pub left: usize,
    /// `4 * lane_words`, always equal to `left`.
    pub right: usize,
}

impl StatePayloadSizes {
    /// Sum of the three sections.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.common + self.left + self.right
    }
}

/// The three byte sections of a payload being read.
#[derive(Clone, Copy, Debug)]
pub struct StatePayloadInput<'a> {
    /// Common section, including the two header words.
    pub common: &'a [u8],
    /// Left channel section.
    pub left: &'a [u8],
    /// Right channel section.
    pub right: &'a [u8],
}

/// The three byte sections of a payload being written.
#[derive(Debug)]
pub struct StatePayloadOutput<'a> {
    /// Common section, including room for the two header words.
    pub common: &'a mut [u8],
    /// Left channel section.
    pub left: &'a mut [u8],
    /// Right channel section.
    pub right: &'a mut [u8],
}

/// The effect's data words, for a snapshot.
#[derive(Clone, Copy, Debug)]
pub struct StateWords<'a> {
    /// Common words, `common_words` of them.
    pub common: &'a [u32],
    /// Left channel words, `lane_words` of them.
    pub left: &'a [u32],
    /// Right channel words, `lane_words` of them.
    pub right: &'a [u32],
}

/// The effect's data words, for a restore.
#[derive(Debug)]
pub struct StateWordsMut<'a> {
    /// Common words, `common_words` of them.
    pub common: &'a mut [u32],
    /// Left channel words, `lane_words` of them.
    pub left: &'a mut [u32],
    /// Right channel words, `lane_words` of them.
    pub right: &'a mut [u32],
}

/// The byte lengths a payload of `layout` must have.
#[must_use]
pub const fn expected_sizes(layout: &StateLayout) -> StatePayloadSizes {
    let lane = layout.lane_words as usize * WORD_BYTES;
    StatePayloadSizes {
        common: (HEADER_WORDS as usize + layout.common_words as usize) * WORD_BYTES,
        left: lane,
        right: lane,
    }
}

// REALTIME_POLICY_BEGIN: #1278 D3, the word and ramp codec the effects' payload calls run in the
// plan-swap block.
/// Writes `value` to word `word` of `bytes`, little-endian.
///
/// # Panics
///
/// Panics if `bytes` is shorter than `(word + 1) * 4`. Section lengths are validated once by
/// [`validate_lengths`] before any word is written, so this is a debugging aid rather than a
/// recoverable path — and it is why the six copies this replaces could each panic on a payload
/// their own `read_lane` had not re-measured.
pub fn write_u32(bytes: &mut [u8], word: usize, value: u32) {
    let offset = word * WORD_BYTES;
    bytes[offset..offset + WORD_BYTES].copy_from_slice(&value.to_le_bytes());
}

/// Writes the raw bits of `value` to word `word` of `bytes`.
///
/// Stored as `to_bits`, not as a rounded decimal: a snapshot restores the exact `f32`, including
/// `-0.0` and including a value that no parameter domain would accept, because a snapshot records
/// what the state *was*.
///
/// # Panics
///
/// As [`write_u32`].
pub fn write_f32(bytes: &mut [u8], word: usize, value: f32) {
    write_u32(bytes, word, value.to_bits());
}

/// Reads word `word` of `bytes`, little-endian.
///
/// # Panics
///
/// Panics if `bytes` is shorter than `(word + 1) * 4`.
#[must_use]
pub fn read_u32(bytes: &[u8], word: usize) -> u32 {
    let offset = word * WORD_BYTES;
    let mut buffer = [0u8; WORD_BYTES];
    buffer.copy_from_slice(&bytes[offset..offset + WORD_BYTES]);
    u32::from_le_bytes(buffer)
}

/// Reads word `word` of `bytes` as the raw bits of an `f32`.
///
/// # Panics
///
/// As [`read_u32`].
#[must_use]
pub fn read_f32(bytes: &[u8], word: usize) -> f32 {
    f32::from_bits(read_u32(bytes, word))
}

/// Words a [`LinearRamp`] occupies in a payload: `current`, `target`, `step`, `remaining`.
///
/// A payload that carries a ramp carries all four, the step included (#1278 D2a): a restore that
/// re-derived the step as `(target - current) / remaining` would resume a mid-flight ramp on
/// another path than the one the continued ramp walks, because the iterated `current` is rounded
/// at every sample.
pub const RAMP_WORDS: usize = 4;

/// Writes `ramp` to words `word .. word + RAMP_WORDS` of `bytes`.
///
/// # Panics
///
/// As [`write_u32`].
pub fn write_ramp(bytes: &mut [u8], word: usize, ramp: LinearRamp) {
    write_f32(bytes, word, ramp.current);
    write_f32(bytes, word + 1, ramp.target);
    write_f32(bytes, word + 2, ramp.step);
    write_u32(bytes, word + 3, ramp.remaining);
}

/// Reads the ramp [`write_ramp`] wrote at `word`, unvalidated.
///
/// # Panics
///
/// As [`read_u32`].
#[must_use]
pub fn read_ramp(bytes: &[u8], word: usize) -> LinearRamp {
    LinearRamp {
        current: read_f32(bytes, word),
        target: read_f32(bytes, word + 1),
        step: read_f32(bytes, word + 2),
        remaining: read_u32(bytes, word + 3),
    }
}

/// Whether every value a restored ramp can take lies in the closed interval `[low, high]`: its
/// `current` and `target`, and each value its remaining steps visit before the snap to the target,
/// iterated exactly as [`LinearRamp::next_value`] iterates them.
///
/// A ramp also needs `remaining <= max_remaining` and a finite step, and a settled ramp
/// (`remaining == 0`) must carry the `+0.0` step that [`LinearRamp::fixed`], [`LinearRamp::snap`],
/// [`LinearRamp::set_target`] and the last [`LinearRamp::next_value`] all write.
///
/// `[low, high]` is the word's strict domain or designed range, with no rounding budget: under the
/// clamped law no engine ramp word leaves its endpoints (issue #1409 D2), so a word outside the
/// range is one the engine never holds (issue #1411 D1). The check reads nothing but its argument
/// and allocates nothing, so a restore may run it on the render thread.
///
/// # Why the endpoints decide the whole path (issue #1409 D2)
///
/// [`LinearRamp::next_value`] advances the word with `lane::kernels::ramp_toward`, which returns a
/// value inside `[min(current, target), max(current, target)]` for any finite step, and that
/// interval lies inside the previous update's. By induction every value the remaining steps visit,
/// and the snap's `target`, lie between the restored `current` and `target`, so the path is inside
/// `[low, high]` exactly when its two endpoints are. No walk is needed, and none is made: the check
/// is four comparisons whatever `remaining` is.
#[must_use]
pub fn ramp_path_inside(ramp: LinearRamp, (low, high): (f32, f32), max_remaining: u32) -> bool {
    let inside = |value: f32| (low..=high).contains(&value);
    if ramp.remaining > max_remaining
        || !ramp.step.is_finite()
        || !inside(ramp.current)
        || !inside(ramp.target)
    {
        return false;
    }
    ramp.remaining != 0 || ramp.step.to_bits() == 0
}
// REALTIME_POLICY_END

/// Checks the three sections against the layout.
///
/// Lengths must match **exactly**: a payload that is longer than its layout is as wrong as one
/// that is shorter, because the surplus is either a different layout's data or uninitialised
/// memory.
///
/// # Errors
///
/// [`STATE_LENGTH_CODE`] if any section has the wrong length.
pub fn validate_lengths(
    layout: &StateLayout,
    sections: (usize, usize, usize),
) -> Result<(), StatePayloadError> {
    let sizes = expected_sizes(layout);
    if sections.0 != sizes.common || sections.1 != sizes.left || sections.2 != sizes.right {
        return Err(StatePayloadError::length());
    }
    Ok(())
}

/// Stamps the version and the data word count into the front of the common section.
///
/// # Panics
///
/// Panics if `common` is shorter than `HEADER_WORDS` words. Call [`validate_lengths`] first.
pub fn write_header(layout: &StateLayout, common: &mut [u8]) {
    write_u32(common, 0, layout.version);
    write_u32(common, 1, layout.data_words());
}

/// Checks the header of the common section against the layout.
///
/// # Errors
///
/// * [`STATE_VERSION_CODE`] if word 0 is not `layout.version`.
/// * [`STATE_LENGTH_CODE`] if the common section is too short to hold a header, or if word 1 is
///   not `layout.data_words()`.
///
/// The version is checked before the word count so that a payload from an older layout reports the
/// version it actually is, rather than the length mismatch that version implies.
pub fn read_header(layout: &StateLayout, common: &[u8]) -> Result<(), StatePayloadError> {
    if common.len() < HEADER_WORDS as usize * WORD_BYTES {
        return Err(StatePayloadError::length());
    }
    if read_u32(common, 0) != layout.version {
        return Err(StatePayloadError::version());
    }
    if read_u32(common, 1) != layout.data_words() {
        return Err(StatePayloadError::length());
    }
    Ok(())
}

/// Writes a whole payload: header, then the effect's words, in order.
///
/// # Errors
///
/// [`STATE_LENGTH_CODE`] if any output section or any word slice has the wrong length.
pub fn snapshot(
    layout: &StateLayout,
    words: &StateWords<'_>,
    out: &mut StatePayloadOutput<'_>,
) -> Result<(), StatePayloadError> {
    validate_lengths(layout, (out.common.len(), out.left.len(), out.right.len()))?;
    validate_word_counts(
        layout,
        words.common.len(),
        words.left.len(),
        words.right.len(),
    )?;
    write_header(layout, out.common);
    for (index, value) in words.common.iter().enumerate() {
        write_u32(out.common, HEADER_WORDS as usize + index, *value);
    }
    for (index, value) in words.left.iter().enumerate() {
        write_u32(out.left, index, *value);
    }
    for (index, value) in words.right.iter().enumerate() {
        write_u32(out.right, index, *value);
    }
    Ok(())
}

/// Reads a whole payload back into the effect's words.
///
/// Validation order is fixed and is the order the audit found the effects disagreeing about:
/// **lengths, then header (version, then word count), then the words**. Nothing is written into
/// `words` until every check has passed, so a rejected restore cannot leave an effect half
/// updated.
///
/// # Errors
///
/// [`STATE_LENGTH_CODE`] or [`STATE_VERSION_CODE`], per [`validate_lengths`] and [`read_header`].
pub fn restore(
    layout: &StateLayout,
    input: &StatePayloadInput<'_>,
    words: &mut StateWordsMut<'_>,
) -> Result<(), StatePayloadError> {
    validate_lengths(
        layout,
        (input.common.len(), input.left.len(), input.right.len()),
    )?;
    validate_word_counts(
        layout,
        words.common.len(),
        words.left.len(),
        words.right.len(),
    )?;
    read_header(layout, input.common)?;
    for (index, value) in words.common.iter_mut().enumerate() {
        *value = read_u32(input.common, HEADER_WORDS as usize + index);
    }
    for (index, value) in words.left.iter_mut().enumerate() {
        *value = read_u32(input.left, index);
    }
    for (index, value) in words.right.iter_mut().enumerate() {
        *value = read_u32(input.right, index);
    }
    Ok(())
}

/// The word slices an effect hands the codec must match its own layout.
fn validate_word_counts(
    layout: &StateLayout,
    common: usize,
    left: usize,
    right: usize,
) -> Result<(), StatePayloadError> {
    if common != layout.common_words as usize
        || left != layout.lane_words as usize
        || right != layout.lane_words as usize
    {
        return Err(StatePayloadError::length());
    }
    Ok(())
}
