//! E4 — the frozen corpus renders the same bits at every width, and the digests are pinned.
//!
//! `tools/miso-engine-wasm-gates` replays exactly this corpus inside a WebAssembly module, with
//! and without `simd128`, against these same pins. That is the cross-target half of master plan
//! #83 D5 for this crate; this file is the native half and the pin's home.
//!
//! A digest is not an oracle. `tests/oracle.rs` is what says the compressor is a compressor;
//! `tests/static_curve.rs` is what says the curve is Giannoulis, Massberg and Reiss equation 4.
//! This file only says the answer does not move — and, through the width sweep, that it does not
//! depend on the backend.

use miso_engine_compressor::corpus::{C1_DIGESTS, CASE_COUNT, CASE_NAMES, POINTS, run_case};
use miso_engine_lane::{Lane, Simd4, Simd8};
use sha2::{Digest, Sha256};

fn case_digest<L: Lane>(case: usize) -> [u8; 32] {
    let mut out = vec![0_u32; POINTS];
    run_case::<L>(case, &mut out);
    let mut hasher = Sha256::new();
    for word in &out {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Every case matches its pin, at `W = 1`, 4 and 8.
///
/// Red mutations (MUTATIONS.md rows 7, 11, 19): the recursive word kept in a local instead of
/// written back to the channel; the ballistic coefficient designed through an `f32`
/// `0.001 * ms * fs` product instead of the `f64` one; the corpus rendered in one block instead of
/// the frozen partition. Each moves the digest at every width.
/// Set `MISO_ENGINE_REPIN_COMPRESSOR_CORPUS=1` to print the scalar pins in `C1_DIGESTS` form.
///
/// Per master plan §8.3 the pins come from the `f32` instantiation and from nowhere else; the
/// vector widths and the wasm legs *confirm* them. Re-pin mode suppresses only the comparison
/// against the pin: `Simd4` and `Simd8` are still required to agree with the scalar oracle, so a
/// width disagreement cannot be laundered into a fresh pin.
#[test]
fn the_corpus_matches_its_pins_at_every_width() {
    let repinning = std::env::var_os("MISO_ENGINE_REPIN_COMPRESSOR_CORPUS").is_some();
    let mut mismatches = Vec::new();
    let mut repin = String::new();
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let scalar = case_digest::<f32>(case);
        for (width, digest) in [
            ("W=4", case_digest::<Simd4>(case)),
            ("W=8", case_digest::<Simd8>(case)),
        ] {
            if digest != scalar {
                mismatches.push(format!(
                    "{name} at {width}: {} (scalar oracle {})",
                    hex(&digest),
                    hex(&scalar)
                ));
            }
        }
        if !repinning && scalar != C1_DIGESTS[case] {
            mismatches.push(format!(
                "{name} at W=1: {} (pinned {})",
                hex(&scalar),
                hex(&C1_DIGESTS[case])
            ));
        }
        let bytes: Vec<String> = scalar.iter().map(|byte| format!("0x{byte:02x}")).collect();
        repin.push_str(&format!("    // {name}\n"));
        repin.push_str(&format!("    [{}],\n", bytes.join(", ")));
    }
    assert!(
        mismatches.is_empty(),
        "E4 digest mismatch:\n{}\n\nA mismatch is never repaired by re-pinning from the run that \
         failed. If an operation order changed deliberately, re-pin from the L = f32 oracle in the \
         same commit and state the deviation (master plan section 8).",
        mismatches.join("\n")
    );
    if repinning {
        println!("{repin}");
        panic!("re-pin mode: copy the block above into C1_DIGESTS in src/corpus.rs");
    }
}

/// The corpus is NaN-free and Inf-free, so the pins survive wasm's NaN canonicalisation (D5).
#[test]
fn the_corpus_is_finite() {
    let mut out = vec![0_u32; POINTS];
    for (case, name) in CASE_NAMES.iter().enumerate() {
        run_case::<f32>(case, &mut out);
        for (index, word) in out.iter().enumerate() {
            let value = f32::from_bits(*word);
            assert!(value.abs() <= f32::MAX, "{name}: word {index} is {value}");
        }
    }
}

/// No case is vacuous: each one produces many distinct non-zero values, and the cases differ from
/// one another.
///
/// Without this a corpus that rendered silence — because, say, the ring never filled — would agree
/// with itself on every target and prove nothing.
#[test]
fn no_case_is_vacuous() {
    let mut digests = Vec::new();
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let mut out = vec![0_u32; POINTS];
        run_case::<f32>(case, &mut out);
        let nonzero = out
            .iter()
            .filter(|word| f32::from_bits(**word) != 0.0)
            .count();
        assert!(
            nonzero > POINTS / 2,
            "{name}: only {nonzero} of {POINTS} words are non-zero"
        );
        let mut distinct: Vec<u32> = out.clone();
        distinct.sort_unstable();
        distinct.dedup();
        assert!(
            distinct.len() > POINTS / 2,
            "{name}: only {} distinct words",
            distinct.len()
        );
        digests.push(C1_DIGESTS[case]);
    }
    let mut unique = digests.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), CASE_COUNT, "two cases share a digest");
}
