//! The one timed region, with the evidence-collection ban enforced by construction.
//!
//! AGENTS.md: a benchmark measures the production shape. Audit #104 F1 is what happens when it
//! measures the harness instead. [`timed`] closes that by construction rather than by review: it
//! samples [`crate::digest`]'s update counter on both sides of the clock and panics if the body
//! hashed anything. The timed body is the workload and arithmetic; the digest, the meter drain and
//! the record building happen outside it.

use crate::digest;
use std::time::Instant;

/// Run `operation`, returning its elapsed nanoseconds and its result.
///
/// # Panics
///
/// Panics if `operation` updated a [`crate::digest::Sha256Sink`]. That is finding F1 -- a
/// benchmark timing its own evidence collection -- and it is a harness defect in the caller, never
/// something to relax here.
pub fn timed<R>(operation: impl FnOnce() -> R) -> (u64, R) {
    let mark = digest::updates_mark();
    let started = Instant::now();
    let result = operation();
    let elapsed = started.elapsed();
    assert_eq!(
        digest::updates_since(mark),
        0,
        "a Sha256Sink was updated inside a timed interval (#104 F1)"
    );
    (
        u64::try_from(elapsed.as_nanos()).unwrap_or(u64::MAX),
        result,
    )
}

/// Run `operation` untimed. The counterpart of [`timed`]: it exists so a call site says which side
/// of the clock it is on.
pub fn untimed<R>(operation: impl FnOnce() -> R) -> R {
    operation()
}

#[cfg(test)]
mod tests {
    use super::{timed, untimed};
    use crate::digest::Sha256Sink;

    #[test]
    fn returns_the_operation_result() {
        let (nanoseconds, value) = timed(|| 21 * 2);
        assert_eq!(value, 42);
        let _ = nanoseconds;
    }

    #[test]
    fn a_pure_body_is_accepted() {
        let mut total = 0u64;
        let (_, ()) = timed(|| {
            for index in 0..1024u64 {
                total = total.wrapping_add(index);
            }
        });
        assert_eq!(total, 523_776);
    }

    #[test]
    #[should_panic(expected = "a Sha256Sink was updated inside a timed interval")]
    fn hashing_inside_the_timer_is_a_harness_defect() {
        let mut sink = Sha256Sink::new();
        let (_, ()) = timed(|| sink.update(b"evidence"));
    }

    #[test]
    fn hashing_outside_the_timer_is_how_it_is_done() {
        let mut sink = Sha256Sink::new();
        let (_, product) = timed(|| 7u64 * 6);
        untimed(|| sink.update(product.to_le_bytes()));
        assert_eq!(sink.finish_hex().len(), 64);
    }
}
