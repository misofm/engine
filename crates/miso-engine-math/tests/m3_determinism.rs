//! Gate M3 — the vendored scalar layer computes the same bits on every target.
//!
//! M3 has two halves, because a digest alone cannot prove the property it is supposed to prove.
//!
//! * **Structural.** A source scan of `src/vendored/` for the constructs that make a build
//!   target-dependent: `target_feature`, the `arch` intrinsic modules, and the fused
//!   multiply-add method. This is the half
//!   that fails on the host that *does* have FMA, immediately, without needing a second target.
//! * **Numerical.** SHA-256 digests over a million-point corpus per function, pinned in
//!   `corpus::M3_DIGESTS`. Job 83d replays the identical corpus under wasmtime and compares
//!   against these same pins; until that harness exists, the pins are this crate's regression
//!   guard against an accidental re-introduction of a target-conditional path.
//!
//! Hazard the structural half exists for (master plan §11): libm's sources fuse a multiply and an
//! add under an FMA target-feature cfg in places. Vendoring strips those, but a future re-vendor
//! that forgets would only show up numerically on an FMA-enabled build.
//!
//! The needles are assembled from fragments rather than written out, because
//! `scripts/check-lane-policy.sh` forbids the fusion vocabulary outside `crates/miso-engine-lane`
//! (D3) and a test that searches for a token would otherwise trip it.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use miso_engine_math::corpus::{CASE_COUNT, CASE_NAMES, M3_DIGESTS, POINTS, run_case};
use sha2::{Digest, Sha256};

fn vendored_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/vendored")
}

/// SHA-256 of one corpus case's result words, little-endian.
fn case_digest(case: usize) -> [u8; 32] {
    let mut out = vec![0u64; POINTS];
    run_case(case, &mut out);
    let mut hasher = Sha256::new();
    for word in &out {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// The structural half of M3: no construct in `src/vendored/` can make one target diverge.
///
/// Red mutation: re-add an FMA fast path in `vendored/exp2.rs` -- an FMA target-feature cfg around
/// a call to the fused multiply-add method. Both the cfg and the call are rejected, on any host,
/// without building for a second target.
#[test]
fn m3_no_target_conditional_source() {
    let forbidden: [String; 6] = [
        "target_feature".to_string(),
        format!("core::{}", "arch"),
        format!("std::{}", "arch"),
        format!("mul{}add", '_'),
        "target_arch".to_string(),
        "is_x86_feature".to_string(),
    ];

    let mut hits = Vec::new();
    let mut files = 0usize;
    for entry in fs::read_dir(vendored_dir()).expect("src/vendored must exist") {
        let path = entry.expect("readable directory entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        files += 1;
        let text = fs::read_to_string(&path).expect("vendored file must be readable");
        for (number, line) in text.lines().enumerate() {
            for needle in &forbidden {
                if line.contains(needle.as_str()) {
                    hits.push(format!(
                        "{}:{}: {needle}: {}",
                        path.file_name().expect("named file").to_string_lossy(),
                        number + 1,
                        line.trim()
                    ));
                }
            }
        }
    }

    assert!(
        files >= 30,
        "expected the full vendored file set, found {files} files"
    );
    assert!(
        hits.is_empty(),
        "src/vendored must contain no target-conditional or fused construct:\n{}",
        hits.join("\n")
    );
}

/// The same scan for `unsafe`, which the workspace denies anyway but which the vendoring edits
/// (libm's `force_eval!`, `i!` and `div!` macros) are specifically responsible for removing.
#[test]
fn m3_no_unsafe_or_force_eval_in_vendored_source() {
    let mut hits = Vec::new();
    for entry in fs::read_dir(vendored_dir()).expect("src/vendored must exist") {
        let path = entry.expect("readable directory entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let text = fs::read_to_string(&path).expect("vendored file must be readable");
        for (number, line) in text.lines().enumerate() {
            // The provenance header names `force_eval!` when explaining that it was removed.
            if line.trim_start().starts_with("//") {
                continue;
            }
            for needle in [
                "unsafe",
                "force_eval!",
                "select_implementation!",
                "read_volatile",
            ] {
                if line.contains(needle) {
                    hits.push(format!(
                        "{}:{}: {needle}",
                        path.file_name().expect("named file").to_string_lossy(),
                        number + 1
                    ));
                }
            }
        }
    }
    assert!(
        hits.is_empty(),
        "vendored source must be free of these constructs:\n{}",
        hits.join("\n")
    );
}

/// The numerical half of M3: every corpus case hashes to its pinned digest.
///
/// Set `MISO_MATH_PIN=1` to print the digests instead of asserting them; that is how
/// `corpus::M3_DIGESTS` is generated after a deliberate re-vendor (VENDORED.md, "Re-vendoring").
#[test]
fn m3_corpus_digests_match_pins() {
    let pinning = std::env::var_os("MISO_MATH_PIN").is_some();
    let mut mismatches = Vec::new();

    for case in 0..CASE_COUNT {
        let digest = case_digest(case);
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
        "MISO_MATH_PIN was set: digests printed, nothing asserted. Unset it to run the gate."
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
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let mut out = vec![0u64; POINTS];
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
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let mut out = vec![0u64; POINTS];
        run_case(case, &mut out);

        let distinct: HashSet<u64> = out.iter().copied().collect();
        let fraction = distinct.len() as f64 / POINTS as f64;
        assert!(
            fraction >= 0.50,
            "corpus case {name} produced only {} distinct results in {POINTS} points ({:.1}%); \
             it is not exercising the function",
            distinct.len(),
            fraction * 100.0
        );
    }
}
