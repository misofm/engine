//! The native half of gate G5, and the assertions that make the wasm half mean something.
//!
//! The wasm leg needs a built `wasm32-unknown-unknown` artifact and therefore lives in
//! `scripts/run-wasm-gates.sh` and the `wasm-gates` CI job. What runs here is everything that can
//! be checked in-process: that the pins still describe the corpus at every width, that the corpus
//! carries no NaN into a digest (master plan D5 excludes NaN payloads because wasm canonicalises
//! them), that no two cases are the same computation, and that the `lane_fma` case actually
//! separates a fused evaluation from an unfused one.
//!
//! Without the last of those, a green wasm run would only prove that both targets computed
//! *something* the same way.

use wasm_gate_corpus as corpus;
use wasm_gates::{hex, native_report};

/// The native leg: every case, at every width, equals its pin.
///
/// Red mutation: change one byte of `src/lane_digests.in`, or reorder `KERNELS` — both make this
/// fail immediately and name the case.
#[test]
fn g5_native_digests_match_pins() {
    let report = native_report();
    assert_eq!(
        report.cases,
        corpus::CASE_COUNT,
        "every case must be compared"
    );
    assert!(
        report.comparisons >= corpus::LANE_CASE_COUNT * corpus::WIDTHS,
        "every lane case must be compared at every width, got {} comparisons",
        report.comparisons
    );
    assert!(
        report.mismatches.is_empty(),
        "native corpus digests differ from the pins:\n{}",
        report
            .mismatches
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// D5: nothing NaN or infinite is ever digested, so the wasm comparison is not comparing
/// canonicalised NaN payloads to native ones.
///
/// Red mutation: add `f32::NAN` to one signal's fill; this fails and names the case.
#[test]
fn g5_lane_corpus_is_finite() {
    for case in 0..corpus::CASE_COUNT {
        if !corpus::has_lane_values(case) {
            continue;
        }
        for width in 0..corpus::WIDTHS {
            let values = corpus::lane_case_values(case, width);
            assert_eq!(
                values.len(),
                corpus::LANES * corpus::FRAMES,
                "case {case} must produce one value per lane frame"
            );
            let offending = values.iter().position(|value| !value.is_finite());
            assert!(
                offending.is_none(),
                "case {} ({}) at {} produced a non-finite value at index {}",
                case,
                corpus::case_name(case),
                corpus::width_name(width),
                offending.unwrap_or_default()
            );
        }
    }
}

/// The one pair of cases that must be identical: `svf_block_ramped` with `ramp_frames = 0` is
/// specified to equal `svf_block` bit for bit, so the corpus checks that rather than excusing it.
const IDLE_PREFIX: &str = "svf_block_ramped/idle/";

/// Every case is a different computation, except the one pair that is defined to be the same.
///
/// A corpus with two accidentally identical cases silently halves the coverage its case count
/// suggests.
#[test]
fn g5_case_digests_are_distinct() {
    let mut seen: Vec<(String, [u8; 32])> = Vec::with_capacity(corpus::CASE_COUNT);
    for case in 0..corpus::CASE_COUNT {
        let name = corpus::case_name(case);
        if name.starts_with(IDLE_PREFIX) {
            continue;
        }
        let digest = corpus::expected_digest(case);
        if let Some((other, _)) = seen.iter().find(|(_, other)| *other == digest) {
            panic!(
                "cases '{name}' and '{other}' have the same pinned digest {}: they are the same \
                 computation twice",
                hex(&digest)
            );
        }
        seen.push((name, digest));
    }
}

/// `svf_block_ramped` with an idle ramp is `svf_block`, on every signal and at every width.
///
/// Red mutation: make the ramped kernel add `step` before the first frame; every signal fails.
#[test]
fn g5_idle_ramped_svf_equals_the_plain_svf() {
    for case in 0..corpus::CASE_COUNT {
        let name = corpus::case_name(case);
        let Some(signal) = name.strip_prefix(IDLE_PREFIX) else {
            continue;
        };
        let plain = (0..corpus::CASE_COUNT)
            .find(|other| corpus::case_name(*other) == format!("svf_block/low/{signal}"))
            .expect("every idle-ramp case has a plain low-pass counterpart");
        assert_eq!(
            hex(&corpus::expected_digest(case)),
            hex(&corpus::expected_digest(plain)),
            "an idle coefficient ramp changed the output for signal '{signal}'"
        );
    }
}

/// The `lane_fma` case proves `Lane::fma` is unfused, and proves the case is not vacuous.
///
/// Two assertions, and both are needed:
///
/// 1. `Lane::fma` agrees with an explicit multiply followed by an add, at every width. That is the
///    contract issue #163 phase 2 installed, checked on the corpus the wasm legs replay.
/// 2. A *genuinely fused* evaluation of the same operands disagrees. Without this the first
///    assertion would pass vacuously the moment the operands stopped separating the two forms,
///    and the wasm leg would go on being green while proving nothing.
///
/// This is the standing form of the G5 red mutation "build the guest with `+relaxed-simd` and use
/// a relaxed multiply-add": a wasm build that started fusing would break assertion 1 here and in
/// the guest, and the digest it produced would be the one assertion 2 computes.
///
/// The fused reference is built here rather than in the corpus crate because that crate is
/// compiled into the wasm guest, where no fused instruction exists. Native `x86-64-v3` pins
/// `+fma`, so `f32::mul_add` below is `vfmadd` -- the single-rounding IEEE operation, which is
/// exactly what this assertion needs to contrast against.
#[test]
fn g5_fma_case_is_unfused_and_the_case_is_not_vacuous() {
    let pinned = corpus::expected_digest(corpus::fma_case());
    for width in 0..corpus::WIDTHS {
        let unfused = corpus::unfused_fma_digest(width);
        assert_eq!(
            hex(&pinned),
            hex(&unfused),
            "Lane::fma must equal an explicit multiply and add at {} (#163 phase 2)",
            corpus::width_name(width)
        );
    }

    let mut fused_lanes = [[0.0_f32; corpus::FRAMES]; corpus::LANES];
    for (lane, out) in fused_lanes.iter_mut().enumerate() {
        let operands = corpus::fma_operands(lane);
        for frame in 0..corpus::FRAMES {
            // UNFUSED-SEAL-EXEMPT: the fused reference gate G5 contrasts the contract against.
            out[frame] = operands[0][frame].mul_add(operands[1][frame], operands[2][frame]);
        }
    }
    let fused = corpus::digest_of_lanes(&fused_lanes);
    assert_ne!(
        hex(&fused),
        hex(&pinned),
        "the lane_fma corpus no longer separates a fused evaluation from an unfused one: \
         the case proves nothing"
    );
}

/// The delegated parts of the corpus are not second pins: they are compared against the digests
/// gates M3, D1 and issue #91 wrote in `math`, `effect-runtime` and
/// `soft-clip`, so the wasm run replays those gates rather than a copy of them that
/// could drift away from the originals.
#[test]
fn g5_delegated_cases_use_the_owning_crates_pins() {
    assert_eq!(
        corpus::MATH_CASE_COUNT,
        math::corpus::CASE_COUNT,
        "the math half must cover every M3 case"
    );
    assert_eq!(
        corpus::RUNTIME_CASE_COUNT,
        effect_runtime::corpus::CASE_COUNT,
        "the effect-runtime half must cover every D1 case"
    );
    for case in 0..corpus::MATH_CASE_COUNT {
        assert_eq!(
            corpus::expected_digest(corpus::LANE_CASE_COUNT + case),
            math::corpus::M3_DIGESTS[case],
            "math case {case} must be pinned by math, not by this crate"
        );
    }
    for case in 0..corpus::RUNTIME_CASE_COUNT {
        assert_eq!(
            corpus::expected_digest(corpus::LANE_CASE_COUNT + corpus::MATH_CASE_COUNT + case),
            effect_runtime::corpus::D1_DIGESTS[case],
            "runtime case {case} must be pinned by effect-runtime, not by this crate"
        );
    }
    assert_eq!(
        corpus::SOFT_CLIP_CASE_COUNT,
        soft_clip::corpus::CASE_COUNT,
        "the soft-clip block must cover every case that crate pins"
    );
    // Each family's base is everything before it, read off the counts rather than written as a
    // literal: that is what keeps these assertions honest when the next effect crate appends its
    // own family, and what makes "appended, never inserted" checkable rather than a convention.
    let soft_clip_base = corpus::LANE_CASE_COUNT
        + corpus::MATH_CASE_COUNT
        + corpus::RUNTIME_CASE_COUNT
        + corpus::TRANSIENT_SHAPER_CASE_COUNT
        + corpus::DELAY_CASE_COUNT
        + corpus::MULTIBAND_CASE_COUNT;
    for case in 0..corpus::SOFT_CLIP_CASE_COUNT {
        assert_eq!(
            corpus::expected_digest(soft_clip_base + case),
            soft_clip::corpus::SOFT_CLIP_DIGESTS[case],
            "soft-clip case {case} must be pinned by soft-clip, not by this crate"
        );
        assert!(
            corpus::is_width_dependent(soft_clip_base + case),
            "a soft-clip case is lane generic and must be digested at every width"
        );
        assert!(
            corpus::case_name(soft_clip_base + case).starts_with("effect/soft_clip/"),
            "soft-clip cases keep their owning crate's names"
        );
    }
    assert_eq!(
        corpus::PARAMETRIC_EQ_CASE_COUNT,
        parametric_eq::corpus::CASE_COUNT,
        "the parametric-EQ block must cover every case that crate pins"
    );
    let parametric_eq_base = soft_clip_base + corpus::SOFT_CLIP_CASE_COUNT;
    for case in 0..corpus::PARAMETRIC_EQ_CASE_COUNT {
        assert_eq!(
            corpus::expected_digest(parametric_eq_base + case),
            parametric_eq::corpus::E9_DIGESTS[case],
            "parametric-eq case {case} must be pinned by parametric-eq, not by this crate"
        );
        assert!(
            corpus::is_width_dependent(parametric_eq_base + case),
            "a parametric-EQ case is lane generic and must be digested at every width"
        );
        assert!(
            corpus::case_name(parametric_eq_base + case).starts_with("effect/parametric_eq/"),
            "parametric-eq cases keep their owning crate's names"
        );
    }
    assert_eq!(
        corpus::GATE_EXPANDER_CASE_COUNT,
        gate_expander::corpus::CASE_COUNT,
        "the gate/expander block must cover every case that crate pins"
    );
    let gate_expander_base = parametric_eq_base + corpus::PARAMETRIC_EQ_CASE_COUNT;
    for case in 0..corpus::GATE_EXPANDER_CASE_COUNT {
        assert_eq!(
            corpus::expected_digest(gate_expander_base + case),
            gate_expander::corpus::GATE_DIGESTS[case],
            "gate-expander case {case} must be pinned by gate-expander, not by this crate"
        );
        assert!(
            corpus::is_width_dependent(gate_expander_base + case),
            "a gate/expander case is lane generic and must be digested at every width"
        );
        assert!(
            corpus::case_name(gate_expander_base + case).starts_with("effect/gate_expander/"),
            "gate-expander cases keep their owning crate's names"
        );
    }
    assert_eq!(
        corpus::BUILTINS_CASE_COUNT,
        builtins::corpus::CASE_COUNT,
        "the builtins block must cover every case that crate pins"
    );
    let builtins_base = gate_expander_base + corpus::GATE_EXPANDER_CASE_COUNT;
    for case in 0..corpus::BUILTINS_CASE_COUNT {
        assert_eq!(
            corpus::expected_digest(builtins_base + case),
            builtins::corpus::BUILTINS_DIGESTS[case],
            "builtins case {case} must be pinned by builtins, not by this crate"
        );
        assert!(
            corpus::is_width_dependent(builtins_base + case),
            "a builtins case is lane generic and must be digested at every width"
        );
        assert!(
            corpus::case_name(builtins_base + case).starts_with("builtins/"),
            "builtins cases keep their owning crate's names, and it is not an effect crate"
        );
    }
    assert_eq!(
        corpus::LIMITER_CASE_COUNT,
        true_peak_limiter::corpus::CASE_COUNT,
        "the true-peak limiter block must cover every case that crate pins"
    );
    let limiter_base = builtins_base + corpus::BUILTINS_CASE_COUNT;
    for case in 0..corpus::LIMITER_CASE_COUNT {
        assert_eq!(
            corpus::expected_digest(limiter_base + case),
            true_peak_limiter::corpus::D90_DIGESTS[case],
            "limiter case {case} must be pinned by true-peak-limiter, not by this crate"
        );
        assert!(
            corpus::is_width_dependent(limiter_base + case),
            "a limiter case is lane generic and must be digested at every width"
        );
        assert!(
            corpus::case_name(limiter_base + case).starts_with("effect/true_peak_limiter/"),
            "limiter cases keep their owning crate's names"
        );
    }
    assert_eq!(
        corpus::COMPRESSOR_CASE_COUNT,
        compressor::corpus::CASE_COUNT,
        "the compressor block must cover every case that crate pins"
    );
    let compressor_base = limiter_base + corpus::LIMITER_CASE_COUNT;
    assert_eq!(
        compressor_base + corpus::COMPRESSOR_CASE_COUNT,
        corpus::CASE_COUNT,
        "the compressor must be the last family in the pin order"
    );
    for case in 0..corpus::COMPRESSOR_CASE_COUNT {
        assert_eq!(
            corpus::expected_digest(compressor_base + case),
            compressor::corpus::C1_DIGESTS[case],
            "compressor case {case} must be pinned by compressor, not by this crate"
        );
        assert!(
            corpus::is_width_dependent(compressor_base + case),
            "a compressor case is lane generic and must be digested at every width"
        );
        assert!(
            corpus::case_name(compressor_base + case).starts_with("effect/compressor/"),
            "compressor cases keep their owning crate's names"
        );
    }
}

/// The one lane case whose correct output is nothing at all.
///
/// A subnormal signal driven into a one-pole recurrence is flushed to `+0.0` by D7 on every
/// target, so the case's value is precisely that it produces zeros. Every *other* case must
/// produce something, or its digest agrees across targets for the empty reason.
const ALL_ZERO_CASES: [&str; 1] = ["one_pole_block/subnormal"];

/// No case is vacuously `+0.0` except the enumerated flush case.
///
/// This is the assertion that caught `ramp_block/impulse`: a ramp starting at zero multiplied the
/// impulse's only non-zero sample by zero, so the case digested 8,192 zeros and would have agreed
/// across every target and every width while proving nothing about `ramp_block`.
#[test]
fn g5_no_case_is_vacuously_zero() {
    for case in 0..corpus::CASE_COUNT {
        if !corpus::has_lane_values(case) {
            continue;
        }
        let name = corpus::case_name(case);
        for width in 0..corpus::WIDTHS {
            let values = corpus::lane_case_values(case, width);
            let all_zero = values.iter().all(|value| value.to_bits() == 0);
            if ALL_ZERO_CASES.contains(&name.as_str()) {
                assert!(
                    all_zero,
                    "case '{name}' at {} is listed as an all-zero flush case but produced a \
                     non-zero value",
                    corpus::width_name(width)
                );
            } else {
                assert!(
                    !all_zero,
                    "case '{name}' at {} produced only +0.0: it would agree across targets for \
                     the empty reason",
                    corpus::width_name(width)
                );
            }
        }
    }
}
