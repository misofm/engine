//! Gate D1 — the effect runtime's lane functions compute the same bits on every target.
//!
//! SHA-256 digests over a fixed corpus per function, pinned in `corpus::D1_DIGESTS`. Job 83d
//! replays the identical corpus under wasmtime, at the wasm scalar and `Simd4` backends, and
//! compares against these same pins; until that harness exists the pins are this crate's guard
//! against an accidental change to a frozen operation order.
//!
//! A digest is not an oracle. `tests/dynamics.rs` is what says the curve is Giannoulis, Massberg
//! and Reiss equation 4, and `tests/envelope.rs` is what says the followers round once. This file
//! only says the answer does not move.

use miso_engine_effect_runtime::corpus::{CASE_NAMES, D1_DIGESTS, POINTS, run_case};
use miso_engine_lane::{Lane, Simd4, Simd8};
use sha2::{Digest, Sha256};

fn case_digest<L: Lane>(case: usize) -> [u8; 32] {
    let mut out = vec![0u32; POINTS];
    run_case::<L>(case, &mut out);
    let mut hasher = Sha256::new();
    for word in &out {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Every case matches its pin, at all three widths.
#[test]
fn the_corpus_matches_its_pins() {
    let mut mismatches = Vec::new();
    for (case, name) in CASE_NAMES.iter().enumerate() {
        for (width, digest) in [
            ("W=1", case_digest::<f32>(case)),
            ("W=4", case_digest::<Simd4>(case)),
            ("W=8", case_digest::<Simd8>(case)),
        ] {
            if digest != D1_DIGESTS[case] {
                mismatches.push(format!(
                    "{name} at {width}: {} (pinned {})",
                    hex(&digest),
                    hex(&D1_DIGESTS[case])
                ));
            }
        }
    }
    assert!(
        mismatches.is_empty(),
        "D1 digest mismatch:\n{}\n\nIf an operation order changed deliberately, re-pin \
         corpus::D1_DIGESTS in the same commit and record the reason.",
        mismatches.join("\n")
    );
}

/// The corpus is NaN-free, so the pins survive wasm's NaN canonicalisation (D5).
#[test]
fn the_corpus_is_nan_free() {
    let mut out = vec![0u32; POINTS];
    for (case, name) in CASE_NAMES.iter().enumerate() {
        run_case::<f32>(case, &mut out);
        for (point, word) in out.iter().enumerate() {
            assert!(
                !f32::from_bits(*word).is_nan(),
                "{name} point {point} is NaN"
            );
        }
    }
}

/// The corpus actually exercises its functions: a case whose outputs are nearly all the same value
/// would pass a digest check while proving nothing.
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

/// Prints the digests in the form `corpus.rs` pins them. Run with `--ignored` after a deliberate
/// change to a frozen operation order.
#[test]
#[ignore = "re-pinning helper"]
fn print_digests() {
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let digest = case_digest::<f32>(case);
        let bytes: Vec<String> = digest.iter().map(|b| format!("0x{b:02x}")).collect();
        println!("    // {name}");
        println!("    [{}],", bytes.join(", "));
    }
}
