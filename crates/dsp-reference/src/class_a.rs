//! Class-A identity's NaN rule, the one fold every class-A comparison, digest and differential
//! reads a word through (issue #1065; owner decision 10 in
//! `docs/rulings/engine-footprint-2026-09-29.md`).
//!
//! Class-A identity promises the same bits on every lane width and every target, except for a
//! NaN's sign and payload. Those belong to the CPU: an invalid operation such as `inf - inf` gives
//! `0xFFC0_0000` on x86 and `0x7FC0_0000` on AArch64, the two choose differently when both
//! operands are NaN, and WebAssembly leaves both unspecified, so the browser module inherits its
//! host's rule. The engine does not canonicalize NaNs at render. A test does it instead, here and
//! nowhere else: [`bits`] and [`word`] fold every NaN to [`NAN_WORD`] and leave every other word,
//! signed zeros and subnormals included, exactly as it is.
//!
//! The fold keeps a NaN a NaN. [`NAN_WORD`] is itself a NaN and no finite or infinite word folds
//! to it, so a finite output that turns NaN still moves a digest, still fails a class-A
//! comparison with its finite oracle, and still fails every finiteness assertion. The NaN-safety
//! rules stand beside this one: finite input must not produce NaN, and each effect's documented
//! NaN behaviour still holds.

/// The one word every NaN folds to: the positive quiet NaN with no payload, which is Rust's
/// `f32::NAN`, the AArch64 default NaN and WebAssembly's canonical NaN.
pub const NAN_WORD: u32 = 0x7fc0_0000;

/// `bits` with every NaN folded to [`NAN_WORD`]: an exponent field of all ones and a non-zero
/// mantissa, of either sign.
#[must_use]
pub const fn word(bits: u32) -> u32 {
    if bits & 0x7fff_ffff > 0x7f80_0000 {
        NAN_WORD
    } else {
        bits
    }
}

/// The class-A bits of `value`: its bits, or [`NAN_WORD`] for any NaN.
#[must_use]
pub const fn bits(value: f32) -> u32 {
    word(value.to_bits())
}

/// Whether two words are the same class-A value: equal bits, or both NaN.
#[must_use]
pub const fn same(a: f32, b: f32) -> bool {
    bits(a) == bits(b)
}

/// The little-endian bytes of a stream of `f32` words, a rendered plane or a state payload, each
/// word folded by [`word`]. Hashing these chunks in order equals hashing the whole buffer when it
/// holds no NaN, so a digest pinned over a NaN-free stream does not move.
///
/// A payload's integer fields (version headers, word counts, ramp counters, enable flags) fold as
/// words too. None of them reaches the NaN range, `0x7F80_0001` and above with the sign bit
/// cleared, so the fold is the identity on them.
///
/// # Panics
///
/// Panics if `bytes` is not a whole number of words.
pub fn le_words(bytes: &[u8]) -> impl Iterator<Item = [u8; 4]> + '_ {
    assert!(
        bytes.len().is_multiple_of(4),
        "a stream of f32 words has a length divisible by four, not {}",
        bytes.len()
    );
    bytes.chunks_exact(4).map(|chunk| {
        word(u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])).to_le_bytes()
    })
}

#[cfg(test)]
mod tests {
    use super::{NAN_WORD, bits, le_words, same, word};

    /// The x86 default NaN, the AArch64 one, a payload and both signalling forms fold to one word,
    /// and nothing else moves: both zeros, both infinities, the extreme finite words and the
    /// subnormal and normal boundaries keep their bits.
    #[test]
    fn every_nan_folds_to_one_word_and_nothing_else_moves() {
        for nan in [
            0x7fc0_0000_u32,
            0xffc0_0000,
            0x7fc0_1234,
            0xffa0_0001,
            0x7f80_0001,
            0xff80_0001,
            0x7fff_ffff,
            0xffff_ffff,
        ] {
            assert!(f32::from_bits(nan).is_nan());
            assert_eq!(word(nan), NAN_WORD, "{nan:#010x}");
        }
        for kept in [
            0x0000_0000_u32,
            0x8000_0000,
            0x0000_0001,
            0x8000_0001,
            0x007f_ffff,
            0x0080_0000,
            0x3f80_0000,
            0x7f7f_ffff,
            0xff7f_ffff,
            0x7f80_0000,
            0xff80_0000,
        ] {
            assert!(!f32::from_bits(kept).is_nan());
            assert_eq!(word(kept), kept, "{kept:#010x}");
        }
        assert!(f32::from_bits(NAN_WORD).is_nan());
    }

    /// The fold hides no NaN-ness: a NaN is never the same class-A value as a finite or infinite
    /// word, and two different non-NaN words stay different.
    #[test]
    fn a_nan_is_never_the_same_value_as_a_number() {
        let nan = f32::from_bits(0xffc0_0000);
        for number in [
            0.0_f32,
            -0.0,
            1.0,
            f32::MAX,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ] {
            assert!(!same(nan, number) && !same(number, nan), "{number}");
            assert_ne!(bits(number), NAN_WORD);
        }
        assert!(same(nan, f32::from_bits(0x7fc0_1234)));
        assert!(!same(0.0, -0.0), "signed zeros are two values");
    }

    /// Folding a byte stream chunk by chunk hashes the same bytes as the stream itself when it
    /// holds no NaN, and folds the NaN words when it does.
    #[test]
    fn le_words_is_the_identity_on_a_nan_free_stream() {
        let words = [0x3f80_0000_u32, 0x8000_0000, 7, 1, 0x7f80_0000];
        let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
        assert_eq!(le_words(&bytes).flatten().collect::<Vec<_>>(), bytes);
        let poisoned: Vec<u8> = [1_u32, 0xffc0_0000]
            .iter()
            .flat_map(|word| word.to_le_bytes())
            .collect();
        assert_eq!(
            le_words(&poisoned).collect::<Vec<_>>(),
            [1_u32.to_le_bytes(), NAN_WORD.to_le_bytes()]
        );
    }
}
