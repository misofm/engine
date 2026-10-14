//! Small checked numeric helpers shared by parsing and compilation.

pub(crate) enum F32Token {
    Value(f32),
    NonFinite,
    NotRepresentable,
}

pub(crate) fn parse_f32_token(text: &str) -> F32Token {
    if text.contains("inf") || text.contains("nan") {
        return F32Token::NonFinite;
    }
    match text.parse::<f32>() {
        Ok(value) if value.is_finite() => F32Token::Value(value),
        Ok(_) | Err(_) => F32Token::NotRepresentable,
    }
}

/// Append one canonical finite `f32` spelling and report whether exact-`f64` fallback was needed.
#[cfg(test)]
pub(crate) fn write_f32(output: &mut String, value: f32) -> bool {
    spell_f32(output, value)
}

/// A text a spelling is written into and can take back: a `String`, or the fixed [`Spelling`]
/// buffer of a counting writer.
pub(crate) trait SpellingSink: core::fmt::Write {
    fn mark(&self) -> usize;
    fn since(&self, start: usize) -> &str;
    fn cut(&mut self, start: usize);
}

impl SpellingSink for String {
    fn mark(&self) -> usize {
        self.len()
    }
    fn since(&self, start: usize) -> &str {
        &self[start..]
    }
    fn cut(&mut self, start: usize) {
        self.truncate(start);
    }
}

/// The longest spelling of a finite `f32`: the shortest round-trip digits of the value as an `f32`
/// or, past that, as an `f64`, in positional notation (about 70 bytes for the smallest subnormal),
/// so 128 bytes hold every one. A test walks the extremes.
const SPELLING_BYTES: usize = 128;

/// A fixed buffer for one `f32` spelling: the length of a number can be counted with no heap.
pub(crate) struct Spelling {
    bytes: [u8; SPELLING_BYTES],
    length: usize,
}

impl Spelling {
    pub(crate) const fn new() -> Self {
        Self {
            bytes: [0; SPELLING_BYTES],
            length: 0,
        }
    }

    /// The bytes written so far.
    pub(crate) fn len(&self) -> usize {
        self.length
    }
}

impl core::fmt::Write for Spelling {
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        let end = self
            .length
            .checked_add(text.len())
            .filter(|&end| end <= SPELLING_BYTES)
            .ok_or(core::fmt::Error)?;
        self.bytes[self.length..end].copy_from_slice(text.as_bytes());
        self.length = end;
        Ok(())
    }
}

impl SpellingSink for Spelling {
    fn mark(&self) -> usize {
        self.length
    }
    fn since(&self, start: usize) -> &str {
        // Only `write_str` fills the buffer, with whole `str`s.
        core::str::from_utf8(&self.bytes[start..self.length]).expect("a spelling is UTF-8")
    }
    fn cut(&mut self, start: usize) {
        self.length = start;
    }
}

/// The one spelling algorithm, over any [`SpellingSink`]. The direct `f32` digits when they parse
/// back to the same `f32` through `f64` (the session parser's path), the exact `f64` digits when
/// not, and a `.0` when the digits hold no point or exponent.
pub(crate) fn spell_f32<S: SpellingSink>(output: &mut S, value: f32) -> bool {
    let start = output.mark();
    let _ = write!(output, "{value}");
    let survives_session_parse = output
        .since(start)
        .parse::<f64>()
        .is_ok_and(|parsed| (parsed as f32).to_bits() == value.to_bits());
    let used_f64_fallback = !survives_session_parse;
    if used_f64_fallback {
        output.cut(start);
        let _ = write!(output, "{}", f64::from(value));
    }
    if !output.since(start).contains(['.', 'e', 'E']) {
        let _ = output.write_str(".0");
    }
    used_f64_fallback
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_round_trip(bits: u32) -> (String, bool) {
        let value = f32::from_bits(bits);
        assert!(value.is_finite());
        let mut spelling = String::with_capacity(64);
        let fallback = write_f32(&mut spelling, value);
        assert_eq!(
            spelling
                .parse::<f32>()
                .expect("direct f32 spelling")
                .to_bits(),
            bits,
            "direct f32 parse mismatch for {bits:#010x}: {spelling}"
        );
        assert_eq!(
            (spelling.parse::<f64>().expect("session f64 spelling") as f32).to_bits(),
            bits,
            "f64-then-f32 parse mismatch for {bits:#010x}: {spelling}"
        );
        (spelling, fallback)
    }

    #[test]
    fn known_double_rounding_values_and_signed_zero_round_trip() {
        let directed = [
            0x15ae_43fd,
            0x95ae_43fd,
            0x8000_0000,
            0x0000_0001,
            0x007f_ffff,
            0x7f7f_ffff,
            0x3f80_0000,
        ];
        let results: Vec<_> = directed.into_iter().map(assert_round_trip).collect();
        assert_eq!(results[0].0, "0.00000000000000000000000007038530691851209");
        assert_eq!(results[1].0, "-0.00000000000000000000000007038530691851209");
        assert!(results[0].1);
        assert!(results[1].1);
        assert!(results[2..].iter().all(|(_, fallback)| !fallback));
        assert_eq!(results[2].0, "-0.0");
        assert_eq!(results[6].0, "1.0");
    }

    /// Issue #1045: one million patterns per PR, down from ten million. The ten-million sample's
    /// only fallback, `0x15ae_43fd` (its pattern 3,644,705), is a directed case above, and
    /// [`exhaustive_f32_round_trip`] runs every pattern nightly, in release.
    #[test]
    fn one_million_deterministic_f32_patterns_round_trip() {
        const GENERATED: u64 = 1_000_000;
        let mut state = 0x004d_4953_4f31_3037_u64;
        let mut spelling = String::with_capacity(64);
        let mut finite = 0_u64;
        let mut fallbacks = 0_u64;
        let mut maximum_length = 0_usize;
        for _ in 0..GENERATED {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            let bits = state.wrapping_mul(0x2545_f491_4f6c_dd1d) as u32;
            let value = f32::from_bits(bits);
            if !value.is_finite() {
                continue;
            }
            finite += 1;
            spelling.clear();
            fallbacks += u64::from(write_f32(&mut spelling, value));
            maximum_length = maximum_length.max(spelling.len());
            assert!(spelling.contains('.'), "missing decimal point: {spelling}");
            assert!(
                !spelling.contains(['e', 'E']),
                "exponent spelling: {spelling}"
            );
            assert_eq!(
                spelling.parse::<f32>().expect("direct f32 parse").to_bits(),
                bits,
                "direct f32 parse mismatch for {bits:#010x}: {spelling}"
            );
            assert_eq!(
                (spelling.parse::<f64>().expect("session f64 parse") as f32).to_bits(),
                bits,
                "f64-then-f32 parse mismatch for {bits:#010x}: {spelling}"
            );
        }
        assert_eq!(finite, 996_076);
        assert_eq!(fallbacks, 0);
        assert_eq!(maximum_length, 48);
    }

    #[derive(Debug)]
    struct SweepResult {
        first_mismatch: Option<(u32, String, Option<u32>, Option<u32>)>,
        fallback_count: u64,
        maximum_length: usize,
    }

    fn sweep_range(start: u64, end: u64) -> SweepResult {
        let mut spelling = String::with_capacity(64);
        let mut fallback_count = 0_u64;
        let mut maximum_length = 0_usize;
        for raw in start..end {
            let bits = raw as u32;
            let value = f32::from_bits(bits);
            if !value.is_finite() {
                continue;
            }
            spelling.clear();
            fallback_count += u64::from(write_f32(&mut spelling, value));
            maximum_length = maximum_length.max(spelling.len());
            let direct = spelling.parse::<f32>().ok().map(f32::to_bits);
            let session = spelling
                .parse::<f64>()
                .ok()
                .map(|parsed| (parsed as f32).to_bits());
            if direct != Some(bits) || session != Some(bits) {
                return SweepResult {
                    first_mismatch: Some((bits, spelling.clone(), direct, session)),
                    fallback_count,
                    maximum_length,
                };
            }
        }
        SweepResult {
            first_mismatch: None,
            fallback_count,
            maximum_length,
        }
    }

    #[test]
    #[ignore = "exhaustive 2^32 sweep; runs nightly in release"]
    fn exhaustive_f32_round_trip() {
        const PATTERNS: u64 = 1_u64 << 32;
        let workers = std::thread::available_parallelism()
            .expect("available parallelism")
            .get() as u64;
        let partition = PATTERNS.div_ceil(workers);
        let results = std::thread::scope(|scope| {
            let mut handles = Vec::with_capacity(workers as usize);
            for worker in 0..workers {
                let start = worker * partition;
                let end = (start + partition).min(PATTERNS);
                if start < end {
                    handles.push(scope.spawn(move || sweep_range(start, end)));
                }
            }
            handles
                .into_iter()
                .map(|handle| handle.join().expect("exhaustive worker"))
                .collect::<Vec<_>>()
        });
        let first_mismatch = results
            .iter()
            .find_map(|result| result.first_mismatch.as_ref());
        assert_eq!(first_mismatch, None, "first finite mismatch");
        let fallback_count = results
            .iter()
            .map(|result| result.fallback_count)
            .sum::<u64>();
        let maximum_length = results
            .iter()
            .map(|result| result.maximum_length)
            .max()
            .unwrap_or(0);
        assert_eq!(fallback_count, 2, "exact f64 fallback population");
        assert_eq!(maximum_length, 48, "maximum canonical f32 spelling");
        assert!(maximum_length <= 50);
    }
}
