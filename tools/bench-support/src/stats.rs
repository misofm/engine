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
