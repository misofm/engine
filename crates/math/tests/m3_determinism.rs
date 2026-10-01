//! Gate M3 — the vendored scalar layer computes the same bits on every target.
//!
//! M3 has two halves, because a digest alone cannot prove the property it is supposed to prove.
//!
//! * **Structural.** A source scan of `src/vendored/` for the constructs that make a build
//!   target-dependent: `target_feature`, the `arch` intrinsic modules, and the fused
//!   multiply-add method, plus `unsafe` and libm's `force_eval!`. This is the half that fails on
//!   the host that *does* have FMA, immediately, without needing a second target. A source scan is
//!   repository policy, not a test, so it lives in `scripts/check-lane-policy.sh` (issue #1047).
//! * **Numerical.** SHA-256 digests over a million-point corpus per function, pinned in
//!   `corpus::M3_DIGESTS`, in this file. Job 83d replays the identical corpus under wasmtime and
//!   compares against these same pins; until that harness exists, the pins are this crate's
//!   regression guard against an accidental re-introduction of a target-conditional path.
//!
//! Hazard the structural half exists for (master plan §11): libm's sources fuse a multiply and an
//! add under an FMA target-feature cfg in places. Vendoring strips those, but a future re-vendor
//! that forgets would only show up numerically on an FMA-enabled build.

use math::corpus::{CASE_COUNT, CASE_NAMES, M3_DIGESTS, POINTS, run_case};
use sha2::{Digest, Sha256};

/// SHA-256 of one corpus case's result words, little-endian.
fn case_digest(out: &[u64]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for word in out {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

fn hex(bytes: &[u8; 32]) -> String {
    engine::hex_lower(bytes)
}

/// The numerical half of M3: every corpus case hashes to its pinned digest.
///
/// Set `MISO_ENGINE_MATH_PIN=1` to print the digests instead of asserting them; that is how
/// `corpus::M3_DIGESTS` is generated after a deliberate re-vendor (VENDORED.md, "Re-vendoring").
#[test]
fn m3_corpus_digests_match_pins() {
    let pinning = std::env::var_os("MISO_ENGINE_MATH_PIN").is_some();
    let mut mismatches = Vec::new();
    let mut out = vec![0u64; POINTS];

    for case in 0..CASE_COUNT {
        run_case(case, &mut out);
        let digest = case_digest(&out);
        if pinning {
            println!("    // {}", CASE_NAMES[case]);
            println!("    {:?},", digest);
            continue;
        }
        if digest != M3_DIGESTS[case] {
            mismatches.push(format!(
                "{}: got {} want {}",
                CASE_NAMES[case],
                hex(&digest),
                hex(&M3_DIGESTS[case])
            ));
        }
    }

    assert!(
        !pinning,
        "MISO_ENGINE_MATH_PIN was set: digests printed, nothing asserted. Unset it to run the gate."
    );
    assert!(
        mismatches.is_empty(),
        "M3 corpus digests differ from the pins. This is a cross-target determinism failure, not \
         something to re-pin, unless libm was deliberately re-vendored:\n{}",
        mismatches.join("\n")
    );
}

/// The corpus must not produce NaN: master plan D5 excludes NaN payloads from the determinism
/// claim because wasm canonicalises them, so a NaN in the corpus would make the wasm replay of
/// these digests fail for a reason that is not a real divergence.
#[test]
fn m3_corpus_is_nan_free() {
    let mut out = vec![0u64; POINTS];
    for (case, name) in CASE_NAMES.iter().enumerate() {
        run_case(case, &mut out);
        let f32_case = matches!(case, 13..=23 | 25 | 27 | 30 | 31);
        for (index, &word) in out.iter().enumerate() {
            let is_nan = if f32_case {
                f32::from_bits(word as u32).is_nan()
            } else {
                f64::from_bits(word).is_nan()
            };
            assert!(!is_nan, "corpus case {name} produced NaN at point {index}");
        }
    }
}

/// The corpus must actually exercise each function, not just its saturation branches.
///
/// This test exists because the first version of the corpus did not: it drew raw `f64` bit
/// patterns, whose exponents are uniform, so almost every `exp2` input was past the overflow or
/// underflow threshold and a one-ulp change to a polynomial coefficient left every digest
/// unchanged. Counting *distinct* result words is the metric that catches that: a corpus stuck in
/// its saturation branches produces a handful of values however many points it has.
#[test]
fn m3_corpus_exercises_each_domain() {
    let mut out = vec![0u64; POINTS];
    for (case, name) in CASE_NAMES.iter().enumerate() {
        run_case(case, &mut out);

        out.sort_unstable();
        let distinct = 1 + out.windows(2).filter(|pair| pair[0] != pair[1]).count();
        let fraction = distinct as f64 / POINTS as f64;
        assert!(
            fraction >= 0.50,
            "corpus case {name} produced only {} distinct results in {POINTS} points ({:.1}%); \
             it is not exercising the function",
            distinct,
            fraction * 100.0
        );
    }
}
