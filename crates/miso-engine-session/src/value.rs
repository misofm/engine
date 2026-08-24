//! Small checked numeric helpers shared by parsing and compilation.

use core::fmt::Write as _;

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

pub(crate) fn parse_i64_token(text: &str, radix: u32) -> Option<i64> {
    i64::from_str_radix(text, radix).ok()
}

/// Append one canonical finite `f32` spelling and report whether exact-`f64` fallback was needed.
pub(crate) fn write_f32(output: &mut String, value: f32) -> bool {
    let start = output.len();
    let _ = write!(output, "{value}");
    let survives_session_parse = output[start..]
        .parse::<f64>()
        .is_ok_and(|parsed| (parsed as f32).to_bits() == value.to_bits());
    let used_f64_fallback = !survives_session_parse;
    if used_f64_fallback {
        output.truncate(start);
        let _ = write!(output, "{}", f64::from(value));
    }
    if !output[start..].contains(['.', 'e', 'E']) {
        output.push_str(".0");
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

    #[test]
    fn ten_million_deterministic_f32_patterns_round_trip() {
        const GENERATED: u64 = 10_000_000;
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
        assert_eq!(finite, 9_960_907);
        assert_eq!(fallbacks, 1);
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
    #[ignore = "authorized one-shot exhaustive release qualification"]
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
