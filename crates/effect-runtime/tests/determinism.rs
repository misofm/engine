//! Gate D1's corpus can fail: no case hashes a NaN payload (`ramp_toward` carries NaN only as the
//! canonical token) and every case has a wide output spread.
//!
//! The corpus's cross-target claim -- the effect runtime's lane functions compute the same bits at
//! `f32`, `Simd4` and `Simd8`, natively and under wasmtime -- has one owner, gate G5 (issue #1048):
//! `g5_native_digests_match_pins` in `tools/wasm-gates/tests/g5_native_corpus.rs` compares every
//! case at every width against `corpus::D1_DIGESTS` in the shipping profile, and the wasm guests of
//! `scripts/run-wasm-gates.sh` compare the same cases against the same pins.
//! `tests/lane_identity.rs` compares the widths to each other in this crate's own debug run.
//!
//! A digest is not an oracle. `tests/dynamics.rs` is what says the curve is Giannoulis, Massberg
//! and Reiss equation 4, and `tests/envelope.rs` is what says the followers round once.

use effect_runtime::corpus::{CASE_NAMES, NAN_TOKEN, POINTS, run_case};

/// No case carries a NaN payload into a digest, so the pins survive wasm's NaN canonicalisation
/// (D5): every case but `ramp_toward` is NaN-free, and `ramp_toward` emits NaN only as
/// [`NAN_TOKEN`], and does emit it, because its NaN `next` passing through is what it pins.
///
/// Claim: a corpus change that makes a case emit NaN, or makes `ramp_toward` hash a NaN payload
/// instead of the token, turns this red; G5 would otherwise compare a payload wasm canonicalises.
#[test]
fn the_corpus_is_nan_free() {
    let mut out = vec![0u32; POINTS];
    for (case, name) in CASE_NAMES.iter().enumerate() {
        run_case::<f32>(case, &mut out);
        let mut tokens = 0usize;
        for (point, word) in out.iter().enumerate() {
            if *name == "ramp_toward" && *word == NAN_TOKEN {
                tokens += 1;
                continue;
            }
            assert!(
                !f32::from_bits(*word).is_nan(),
                "{name} point {point} is a NaN payload"
            );
        }
        if *name == "ramp_toward" {
            assert!(tokens > 0, "{name} never reaches its NaN arm");
        }
    }
}

/// The corpus actually exercises its functions: a case whose outputs are nearly all the same value
/// would pass a digest check while proving nothing.
///
/// Claim: a corpus change that collapses a case to a few output words turns this red.
#[test]
fn every_case_has_a_wide_output_spread() {
    let mut out = vec![0u32; POINTS];
    for (case, name) in CASE_NAMES.iter().enumerate() {
        run_case::<f32>(case, &mut out);
        let mut distinct = std::collections::HashSet::new();
        for word in &out {
            distinct.insert(*word);
        }
        // The floors are per-case because the cases are not equally spread by nature. The two
        // hysteresis cases are genuinely low-cardinality — one is a flag, the other a small
        // countdown — and the gain-computer cases return an exact `+0.0` for every level below the
        // knee, which is most of a `[-160, 24]` dB sweep. Everything else must cover a real range.
        let floor = if name.starts_with("hysteresis") {
            2
        } else if name.starts_with("gain_delta") {
            POINTS / 16
        } else {
            POINTS / 4
        };
        assert!(
            distinct.len() >= floor,
            "{name}: only {} distinct outputs out of {POINTS}",
            distinct.len()
        );
    }
}
