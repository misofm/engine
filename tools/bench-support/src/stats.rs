//! The one nearest-rank percentile.
//!
//! Audit #104 F4 found eight copies with three edge behaviours: `rack-bench` computed
//! `rank - 1` unguarded, `builtins-bench` used `saturating_sub(1)`, `effect-contract-bench`
//! clamped the rank with `.max(1)`, and `conformance`/`session`/`scheduler` additionally clamped
//! against `len - 1`. For a non-empty sample and `1 <= numerator <= denominator` they all agree,
//! which is why this one replaces all eight without moving a single published number.

/// Nearest rank (Hyndman and Fan type 1): `rank = ceil(len * numerator / denominator)`, clamped
/// into `1..=len`, and the value at `rank - 1` of the ascending `sorted` slice.
///
/// # Panics
///
/// Panics if `sorted` is empty or `denominator` is zero. Every historical copy indexed a slice
/// directly and would have panicked on an empty sample too.
#[must_use]
pub fn nearest_rank<T: Copy>(sorted: &[T], numerator: usize, denominator: usize) -> T {
    assert!(!sorted.is_empty(), "nearest_rank on an empty sample");
    assert!(denominator != 0, "nearest_rank with a zero denominator");
    let rank = sorted
        .len()
        .saturating_mul(numerator)
        .div_ceil(denominator)
        .clamp(1, sorted.len());
    sorted[rank - 1]
}

/// Nearest rank at a per-mille position, the form eight of the nine call sites used.
#[must_use]
pub fn per_mille<T: Copy>(sorted: &[T], per_mille: usize) -> T {
    nearest_rank(sorted, per_mille, 1_000)
}

/// Nearest-rank min/p50/p95/p99/max over one leg's per-block nanoseconds.
///
/// Audit #104 F4: the native console benchmark and the #163 phase 2 wasm console arm each defined
/// this same five-field summary over the same percentiles. One definition, so the two legs' ratio
/// is computed the same way on both sides of the comparison.
pub struct Percentiles {
    /// The minimum observation.
    pub min: u64,
    /// The 50th percentile (median).
    pub p50: u64,
    /// The 95th percentile.
    pub p95: u64,
    /// The 99th percentile.
    pub p99: u64,
    /// The maximum observation.
    pub max: u64,
}

impl Percentiles {
    /// Compute min/p50/p95/p99/max over `samples`.
    ///
    /// # Panics
    ///
    /// Panics if `samples` is empty.
    #[must_use]
    pub fn from_samples(samples: &[u64]) -> Self {
        let mut sorted = samples.to_vec();
        sorted.sort_unstable();
        assert!(!sorted.is_empty(), "measured observations");
        let rank =
            |numerator: usize, denominator: usize| nearest_rank(&sorted, numerator, denominator);
        Self {
            min: sorted[0],
            p50: rank(50, 100),
            p95: rank(95, 100),
            p99: rank(99, 100),
            max: *sorted.last().expect("nonempty"),
        }
    }
}

/// `nanoseconds` rendered as fixed-point microseconds, [`format_f64`]'s three-decimal form.
#[must_use]
pub fn microseconds(nanoseconds: u64) -> String {
    format_f64(nanoseconds as f64 / 1_000.0)
}

/// A stable three-decimal rendering of `value`, the one record-field float format.
#[must_use]
pub fn format_f64(value: f64) -> String {
    format!("{value:.3}")
}

#[cfg(test)]
mod tests {
    use super::{nearest_rank, per_mille};

    #[test]
    fn frozen_indices_over_one_thousand_observations() {
        let sorted: Vec<u64> = (0..1000).collect();
        assert_eq!(per_mille(&sorted, 500), 499);
        assert_eq!(per_mille(&sorted, 950), 949);
        assert_eq!(per_mille(&sorted, 990), 989);
        assert_eq!(per_mille(&sorted, 999), 998);
        assert_eq!(per_mille(&sorted, 1000), 999);
    }

    #[test]
    fn a_single_observation_is_every_percentile() {
        assert_eq!(per_mille(&[7u128], 1), 7);
        assert_eq!(per_mille(&[7u128], 500), 7);
        assert_eq!(per_mille(&[7u128], 1000), 7);
    }

    #[test]
    fn small_samples_round_up_to_the_next_rank() {
        let sorted = [1u64, 2, 3];
        assert_eq!(per_mille(&sorted, 1), 1);
        assert_eq!(per_mille(&sorted, 334), 2);
        assert_eq!(per_mille(&sorted, 500), 2);
        assert_eq!(per_mille(&sorted, 667), 3);
    }

    #[test]
    fn the_numerator_denominator_form_matches_the_per_mille_form() {
        let sorted: Vec<u64> = (0..256).collect();
        for position in [1usize, 500, 950, 990, 999, 1000] {
            assert_eq!(
                nearest_rank(&sorted, position, 1_000),
                per_mille(&sorted, position)
            );
        }
    }

    #[test]
    fn is_monotone_in_the_position() {
        let sorted: Vec<u64> = (0..97).collect();
        let mut previous = per_mille(&sorted, 1);
        for position in 2..=1_000 {
            let value = per_mille(&sorted, position);
            assert!(value >= previous, "percentile went backwards at {position}");
            previous = value;
        }
    }

    #[test]
    #[should_panic(expected = "nearest_rank on an empty sample")]
    fn rejects_an_empty_sample() {
        let _ = per_mille::<u64>(&[], 500);
    }
}
