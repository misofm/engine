//! Gate 7.9 (native leg): the corpus digests are width independent and match their pins.
//!
//! The pins in `src/gate_digests.in` come from the scalar `Lane` instantiation, which is the
//! oracle for every other width (master plan #83 §1.7 and §8). `tools/wasm-gates`
//! replays exactly these cases under wasmtime and compares against exactly these pins.

use gate_expander::corpus::{CASE_COUNT, CASE_NAMES, GATE_DIGESTS, POINTS, run_case};
use lane::{Lane, Simd4, Simd8};
use sha2::{Digest, Sha256};

fn digest<L: Lane>(case: usize) -> [u8; 32] {
    let mut words = vec![0_u32; POINTS];
    run_case::<L>(case, &mut words);
    let mut hasher = Sha256::new();
    for word in &words {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

/// Set `MISO_ENGINE_REPIN_GATE_EXPANDER_CORPUS=1` to print the scalar pins in `gate_digests.in`
/// form.
///
/// Per master plan §8.3 the pins come from the `f32` instantiation and from nowhere else; the
/// vector widths and the wasm legs *confirm* them. Re-pin mode suppresses only the comparison
/// against the pin: `Simd4` and `Simd8` are still required to agree with the scalar oracle, so a
/// width disagreement cannot be laundered into a fresh pin.
#[test]
fn every_case_agrees_at_every_width_and_matches_its_pin() {
    let repinning = std::env::var_os("MISO_ENGINE_REPIN_GATE_EXPANDER_CORPUS").is_some();
    let mut repin = String::from("[\n");
    for case in 0..CASE_COUNT {
        let scalar = digest::<f32>(case);
        if !repinning {
            assert_eq!(
                scalar, GATE_DIGESTS[case],
                "{}: the scalar oracle moved away from its pin",
                CASE_NAMES[case]
            );
        }
        assert_eq!(
            digest::<Simd4>(case),
            scalar,
            "{}: Simd4 disagrees with the scalar oracle",
            CASE_NAMES[case]
        );
        assert_eq!(
            digest::<Simd8>(case),
            scalar,
            "{}: Simd8 disagrees with the scalar oracle",
            CASE_NAMES[case]
        );
        repin.push_str("    [\n");
        for row in scalar.chunks(8) {
            let bytes: Vec<String> = row.iter().map(|byte| format!("0x{byte:02x},")).collect();
            repin.push_str(&format!("        {} \n", bytes.join(" ")));
        }
        repin.push_str("    ],\n");
    }
    repin.push_str("]\n");
    if repinning {
        println!("{repin}");
        panic!("re-pin mode: copy the block above into src/gate_digests.in");
    }
}

#[test]
fn no_case_is_vacuous() {
    // A corpus of zeros, or of NaN, would agree at every width for the wrong reason. D5 also
    // excludes NaN payloads outright, because wasm canonicalises them.
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let mut words = vec![0_u32; POINTS];
        run_case::<f32>(case, &mut words);
        let values: Vec<f32> = words.iter().map(|word| f32::from_bits(*word)).collect();
        assert!(
            values.iter().all(|value| value.is_finite()),
            "{name}: a non-finite value reached the digest"
        );
        let distinct = values.iter().filter(|value| **value != 0.0).count();
        assert!(
            distinct > POINTS / 4,
            "{name}: only {distinct} of {POINTS} results are non-zero"
        );
    }
}
