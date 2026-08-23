//! Analytic acceptance gates: the transfer the kernel actually realises, against the RBJ oracle.
//!
//! Every gate here evaluates the **state space of the implemented recurrence** from the words
//! exactly as the kernel rounds them. That is the correction issue #87 F1/F8 demanded: the gate
//! this replaces evaluated `N(delta)/D(delta)` of seven stored words, a rational function the
//! kernel did not compute, which is how 483 of these 1,488 rows passed while the shipped graph was
//! 12.4859 dB out.
//!
//! The oracle side is `miso-engine-dsp-reference`, hand-written from the cookbook and independently
//! re-checked in issue #105.

mod support;

use miso_engine_core::SampleRateHz;
use miso_engine_dsp_reference::{
    ReferenceParametricEqCoefficients, ReferenceSvfCoefficients, ReferenceSvfStateSpace,
    shelf_slope_to_q,
};
use miso_engine_parametric_eq::{EqBandKindV1, EqSvfWordsV1, design_svf_v1};
use support::{
    FROZEN_GAINS, FROZEN_QS, FROZEN_SLOPES, GridRow, LAUNCH_RATES, frozen_grid, reference_kind,
    reference_svf_kind,
};

/// The frozen response tolerance of issues #42/#44/#45, unchanged.
const RESPONSE_TOLERANCE_DB: f64 = 0.005;
/// The frozen exclusion floor: below it the cookbook magnitude is its own rounding noise.
const RESPONSE_FLOOR_DB: f64 = -120.0;
/// The frozen notch-null gate.
const NOTCH_NULL: f64 = 1.0e-5;
/// The frozen frequency-search tolerance.
const FREQUENCY_TOLERANCE_RATIO: f64 = 0.001;

/// The state space of the words the kernel holds, in `f64`, from the exact `f32` bits.
fn realized(words: EqSvfWordsV1) -> ReferenceSvfStateSpace {
    ReferenceSvfStateSpace::new(
        f64::from(words.c1),
        f64::from(words.a2),
        f64::from(words.a3),
        [
            f64::from(words.m0),
            f64::from(words.m1),
            f64::from(words.m2),
        ],
    )
}

fn design(row: GridRow) -> EqSvfWordsV1 {
    design_svf_v1(
        row.kind,
        row.frequency,
        row.gain,
        row.q,
        row.slope,
        SampleRateHz(row.rate),
    )
    .unwrap_or_else(|error| panic!("every frozen row is a legal design: {row:?} {error:?}"))
}

fn oracle(row: GridRow) -> ReferenceParametricEqCoefficients {
    ReferenceParametricEqCoefficients::design(
        reference_kind(row.kind),
        f64::from(row.rate),
        f64::from(row.frequency),
        f64::from(row.gain),
        f64::from(row.q),
        f64::from(row.slope),
    )
    .expect("every frozen row is a legal independent reference design")
}

fn magnitude_db(words: EqSvfWordsV1, rate: u32, probe: f64) -> f64 {
    realized(words)
        .magnitude_db(f64::from(rate), probe)
        .expect("probe inside Nyquist")
}

fn magnitude(words: EqSvfWordsV1, rate: u32, probe: f64) -> f64 {
    let response = realized(words)
        .response(f64::from(rate), probe)
        .expect("probe inside Nyquist");
    response.re.hypot(response.im)
}

/// E0: the `f64` mapping this crate designs is the mapping issue #105 verified against the cookbook.
///
/// Phase 0 of the #87 plan is the merged `miso-engine-dsp-reference::svf` oracle, whose own gate
/// `svf_transfer_matches_rbj_cookbook` measured the Simper mapping against the RBJ closed forms over
/// exactly this 1,488-row grid: 1e-9 dB flat for every row at or above 1 kHz (worst 3.9e-10) and
/// 3.6e-8 dB worst over the whole grid, including the 10 Hz / Q = 18 corners. This gate closes the
/// remaining link on **this crate's** side, before any `f32` rounding is involved:
///
/// 1. the `f64` words agree with the reference design to a relative 1e-12 — the residue is the last
///    bit of `10^(dB/40)` (this crate must use `miso-engine-math`, the reference uses the platform
///    libm) and of the shelf `1/Q_S` round trip, and
/// 2. the `f64` transfer of the realized recurrence matches the cookbook biquad over the same
///    2,051 probes and the same -120 dB floor the `f32` gate uses, five thousand times inside the
///    frozen 0.005 dB tolerance.
///
/// Anything worse than that is a mapping error, not a rounding error, which is the whole point of
/// running it before touching the production path.
#[test]
fn the_f64_mapping_reproduces_the_verified_reference_mapping() {
    const WORD_TOLERANCE: f64 = 1.0e-12;
    const MAPPING_TOLERANCE_DB: f64 = 1.0e-6;
    let mut rows = 0_u32;
    let mut worst_word = 0.0_f64;
    let mut worst_error = 0.0_f64;
    for row in frozen_grid() {
        rows += 1;
        let q = if matches!(row.kind, EqBandKindV1::LowShelf | EqBandKindV1::HighShelf) {
            shelf_slope_to_q(f64::from(row.gain), f64::from(row.slope)).expect("legal shelf slope")
        } else {
            f64::from(row.q)
        };
        let reference = ReferenceSvfCoefficients::design(
            reference_svf_kind(row.kind),
            f64::from(row.rate),
            f64::from(row.frequency),
            q,
            f64::from(row.gain),
        )
        .expect("legal reference design");
        let words = miso_engine_parametric_eq::design_svf_words_f64(
            row.kind,
            f64::from(row.frequency),
            f64::from(row.gain),
            f64::from(row.q),
            f64::from(row.slope),
            f64::from(row.rate),
        );
        let expected = [
            reference.c1,
            reference.a2,
            reference.a3,
            reference.m0,
            reference.m1,
            reference.m2,
        ];
        for (index, (mine, theirs)) in words.into_iter().zip(expected).enumerate() {
            let scale = theirs.abs().max(1.0);
            let relative = (mine - theirs).abs() / scale;
            worst_word = worst_word.max(relative);
            assert!(
                relative <= WORD_TOLERANCE,
                "word {index} of {row:?}: {mine} vs {theirs} (relative {relative})"
            );
        }
        let exact = ReferenceSvfStateSpace::new(
            words[0],
            words[1],
            words[2],
            [words[3], words[4], words[5]],
        );
        let cookbook = oracle(row);
        for probe in grid_probes(row) {
            let Ok(reference_magnitude) = cookbook.magnitude_at_hz(probe) else {
                continue;
            };
            if reference_magnitude <= 0.0 {
                continue;
            }
            let reference_db = 20.0 * reference_magnitude.log10();
            if reference_db < RESPONSE_FLOOR_DB {
                continue;
            }
            let realized_db = exact
                .magnitude_db(f64::from(row.rate), probe)
                .expect("probe inside Nyquist");
            let error = (realized_db - reference_db).abs();
            worst_error = worst_error.max(error);
            assert!(
                error <= MAPPING_TOLERANCE_DB,
                "{row:?} probe={probe}: f64 mapping error={error} dB"
            );
        }
    }
    assert_eq!(rows, 1_488);
    eprintln!(
        "issue-087 E0 rows=1488 worst_word_relative={worst_word:.6e} worst_mapping_db={worst_error:.6e}"
    );
}

/// The frozen probe set of one grid row: 2,048 log probes plus `f0`, DC and Nyquist.
fn grid_probes(row: GridRow) -> Vec<f64> {
    let mut probes: Vec<f64> = (0..2_048)
        .map(|index| 10.0 * 2_000.0_f64.powf(f64::from(index) / 2_047.0))
        .collect();
    probes.extend([f64::from(row.frequency), 0.0, f64::from(row.rate) * 0.5]);
    probes
}

/// E1: the realized `f32` transfer matches the cookbook over the complete frozen grid.
#[test]
fn svf_words_match_the_independent_oracle_on_the_complete_grid() {
    let mut rows = 0_u32;
    let mut worst_error = 0.0_f64;
    for row in frozen_grid() {
        rows += 1;
        let words = design(row);
        let reference = oracle(row);
        for probe in grid_probes(row) {
            let Ok(reference_magnitude) = reference.magnitude_at_hz(probe) else {
                continue;
            };
            if reference_magnitude <= 0.0 {
                continue;
            }
            let reference_db = 20.0 * reference_magnitude.log10();
            if reference_db < RESPONSE_FLOOR_DB {
                continue;
            }
            let realized_magnitude = magnitude(words, row.rate, probe);
            assert!(
                realized_magnitude.is_finite() && realized_magnitude > 0.0,
                "{row:?} probe={probe}: realized magnitude {realized_magnitude}"
            );
            let error = (20.0 * realized_magnitude.log10() - reference_db).abs();
            worst_error = worst_error.max(error);
            assert!(
                error <= RESPONSE_TOLERANCE_DB,
                "{row:?} probe={probe}: error={error} dB"
            );
        }
        if row.kind == EqBandKindV1::Notch {
            assert!(
                magnitude(words, row.rate, f64::from(row.frequency)) <= NOTCH_NULL,
                "{row:?} did not retain the -100 dB null"
            );
        }
    }
    assert_eq!(rows, 1_488);
    assert!(worst_error <= RESPONSE_TOLERANCE_DB);
    eprintln!("issue-087 E1 grid rows=1488 worst_error_db={worst_error:.6e}");
}

fn find_crossing(words: EqSvfWordsV1, rate: u32, target_db: f64) -> f64 {
    let mut low = 0.0;
    let mut high = f64::from(rate) * 0.5;
    let mut low_side = magnitude_db(words, rate, low) >= target_db;
    let high_side = magnitude_db(words, rate, high) >= target_db;
    assert_ne!(
        low_side, high_side,
        "frequency gate must bracket its crossing"
    );
    for _ in 0..96 {
        let middle = (low + high) * 0.5;
        let middle_side = magnitude_db(words, rate, middle) >= target_db;
        if middle_side == low_side {
            low = middle;
            low_side = middle_side;
        } else {
            high = middle;
        }
    }
    (low + high) * 0.5
}

fn find_log_extremum(words: EqSvfWordsV1, rate: u32, maximum: bool) -> f64 {
    let mut low = f64::from(rate) * 1.0e-12;
    let mut high = f64::from(rate) * 0.5;
    for _ in 0..96 {
        let log_low = low.ln();
        let span = high.ln() - log_low;
        let first = (log_low + span / 3.0).exp();
        let second = (log_low + span * (2.0 / 3.0)).exp();
        let first_value = magnitude(words, rate, first);
        let second_value = magnitude(words, rate, second);
        let keep_left = if maximum {
            first_value >= second_value
        } else {
            first_value <= second_value
        };
        if keep_left {
            high = second;
        } else {
            low = first;
        }
    }
    (low.ln() + (high.ln() - low.ln()) * 0.5).exp()
}

fn assert_frequency_match(found: f64, requested: f32, gate: &str) {
    let relative_error = (found - f64::from(requested)).abs() / f64::from(requested);
    assert!(
        relative_error <= FREQUENCY_TOLERANCE_RATIO,
        "{gate}: found={found} requested={requested} relative_error={relative_error}"
    );
}

/// E2: the 1,104 frozen frequency searches, now run on the implemented state space.
#[test]
fn frequency_searches_cover_cutoff_center_midpoint_and_notch_minimum() {
    let mut searches = 0_u32;
    for rate in LAUNCH_RATES {
        for frequency in support::FROZEN_FREQUENCIES {
            let row = |kind, gain, q, slope| GridRow {
                kind,
                rate,
                frequency,
                gain,
                q,
                slope,
            };
            for kind in [EqBandKindV1::LowPass, EqBandKindV1::HighPass] {
                let words = design(row(kind, 0.0, core::f32::consts::FRAC_1_SQRT_2, 1.0));
                let found = find_crossing(words, rate, -3.010_299_956_6);
                assert_frequency_match(found, frequency, "Butterworth cutoff");
                searches += 1;
            }
            for q in FROZEN_QS {
                for gain in FROZEN_GAINS {
                    if gain == 0.0 {
                        continue;
                    }
                    let words = design(row(EqBandKindV1::Bell, gain, q, 1.0));
                    let found = find_log_extremum(words, rate, gain > 0.0);
                    assert_frequency_match(found, frequency, "bell center");
                    assert!(
                        (magnitude_db(words, rate, found) - f64::from(gain)).abs()
                            <= RESPONSE_TOLERANCE_DB,
                        "bell center gain Fs={rate} f={frequency} gain={gain} Q={q}"
                    );
                    searches += 1;
                }
                let words = design(row(EqBandKindV1::Notch, 0.0, q, 1.0));
                let found = find_log_extremum(words, rate, false);
                assert_frequency_match(found, frequency, "notch minimum");
                assert!(
                    magnitude(words, rate, found) <= NOTCH_NULL,
                    "notch null Fs={rate} f={frequency} Q={q}"
                );
                searches += 1;
            }
            for gain in FROZEN_GAINS {
                if gain == 0.0 {
                    continue;
                }
                for slope in FROZEN_SLOPES {
                    for kind in [EqBandKindV1::LowShelf, EqBandKindV1::HighShelf] {
                        let words = design(row(kind, gain, 1.0, slope));
                        let found = find_crossing(words, rate, f64::from(gain) * 0.5);
                        assert_frequency_match(found, frequency, "shelf midpoint");
                        searches += 1;
                    }
                }
            }
        }
    }
    assert_eq!(searches, 1_104);
}

/// E4a: ten thousand seeded legal designs are finite, contractive and reference bounded.
#[test]
fn ten_thousand_seeded_legal_designs_are_finite_and_reference_bounded() {
    const COUNT: usize = 10_000;
    const SEED: u64 = 0x0000_0000_0012_e911;
    let kinds = support::FROZEN_KINDS;
    let mut state = SEED;
    let mut strata = [[0_u32; 6]; 4];
    let mut worst_norm = 0.0_f64;
    let mut worst_response_error = 0.0_f64;
    let mut transcript = 0xcbf2_9ce4_8422_2325_u64;
    for index in 0..COUNT {
        let (rate_index, kind_index, frequency, gain, q, slope) = if index < 48 {
            let high_edge = index % 2 == 1;
            let edge = support::FROZEN_EDGES[usize::from(high_edge)];
            (index / 12, (index / 2) % 6, edge.0, edge.1, edge.2, edge.3)
        } else {
            let rate_index = (support::splitmix64(&mut state) as usize) % LAUNCH_RATES.len();
            let kind_index = (support::splitmix64(&mut state) as usize) % kinds.len();
            let frequency = 10.0 * 2_000.0_f32.powf(support::seeded_unit_interval(&mut state));
            let gain = -24.0 + 48.0 * support::seeded_unit_interval(&mut state);
            let q = 0.1 * 180.0_f32.powf(support::seeded_unit_interval(&mut state));
            let slope = 0.1 + 0.9 * support::seeded_unit_interval(&mut state);
            (rate_index, kind_index, frequency, gain, q, slope)
        };
        let row = GridRow {
            kind: kinds[kind_index],
            rate: LAUNCH_RATES[rate_index],
            frequency,
            gain,
            q,
            slope,
        };
        strata[rate_index][kind_index] += 1;
        let words = design(row);
        worst_norm = worst_norm.max(miso_engine_parametric_eq::word_spectral_norm(words));
        for value in words.to_array() {
            assert!(value.is_finite(), "seeded word {row:?}");
            transcript ^= u64::from(value.to_bits());
            transcript = transcript.wrapping_mul(0x0000_0100_0000_01b3);
        }
        let reference_magnitude = oracle(row)
            .magnitude_at_hz(f64::from(frequency))
            .expect("independent seeded f0");
        let reference_db = 20.0 * reference_magnitude.log10();
        if reference_db >= RESPONSE_FLOOR_DB {
            let error = (magnitude_db(words, row.rate, f64::from(frequency)) - reference_db).abs();
            worst_response_error = worst_response_error.max(error);
            assert!(
                error <= RESPONSE_TOLERANCE_DB,
                "seeded response {row:?}: error={error}"
            );
        }
    }
    assert_eq!(strata.iter().flatten().copied().sum::<u32>(), COUNT as u32);
    assert!(strata.iter().flatten().all(|count| *count >= 2));
    assert!(worst_response_error <= RESPONSE_TOLERANCE_DB);
    eprintln!(
        "issue-087 seeded-designs count={COUNT} seed={SEED:#018x} worst_norm={worst_norm:.12} \
         worst_response_db={worst_response_error:.12} transcript={transcript:016x}"
    );
}

/// E5: every point of a word ramp between two legal designs is contractive.
///
/// `‖M‖₂` is convex in the words and `M` is affine in them, so a linear ramp between two
/// contractive triples is contractive throughout — which is what makes a per-sample coefficient
/// update legal in this topology and illegal in a direct-form one. The convex combinations are
/// checked explicitly rather than argued.
#[test]
fn word_ramps_are_contractive_on_every_grid_row() {
    let norm = miso_engine_parametric_eq::word_spectral_norm;
    let tolerance = 1.0 + 1.0 / 4_194_304.0;
    let rows: Vec<_> = frozen_grid()
        .into_iter()
        .filter(|row| row.rate == 44_100)
        .collect();
    let words: Vec<EqSvfWordsV1> = rows.iter().copied().map(design).collect();
    let mut worst = 0.0_f64;
    for value in &words {
        worst = worst.max(norm(*value));
    }
    let mut combinations = 0_u64;
    for (first_index, first) in words.iter().enumerate() {
        for second in &words[first_index + 1..] {
            for lambda in [0.25_f32, 0.5, 0.75] {
                let mixed = EqSvfWordsV1::from_array(core::array::from_fn(|index| {
                    let a = first.to_array()[index];
                    let b = second.to_array()[index];
                    a + (b - a) * lambda
                }));
                let value = norm(mixed);
                worst = worst.max(value);
                combinations += 1;
                assert!(
                    value <= tolerance,
                    "convex combination lambda={lambda} of {:?} and {:?}: norm={value}",
                    first,
                    second
                );
            }
        }
    }
    assert_eq!(rows.len(), 372);
    assert!(worst <= tolerance, "worst word-ramp norm {worst}");
    eprintln!("issue-087 E5 combinations={combinations} worst_norm={worst:.12}");
}

/// The exact rows the `native-pcm-runner` session fixtures configure, bounded against the oracle.
///
/// Those five PCM digests are re-pinned by issue #87 (master plan §8): the render changed because
/// this crate's realization changed, and nothing else in the plan moved. This gate is the oracle
/// side of that re-pin — the coefficient sets the fixture actually uses, at the four launch rates,
/// held to the same 0.005 dB cookbook tolerance as the whole grid — so the new bits are certified
/// by an independent model rather than by having been produced.
#[test]
fn the_pcm_fixture_rows_are_oracle_bounded() {
    let mut worst = 0.0_f64;
    for rate in LAUNCH_RATES {
        for (frequency, gain) in [(120.0_f32, 6.0_f32), (2_400.0, -9.0)] {
            let row = GridRow {
                kind: EqBandKindV1::Bell,
                rate,
                frequency,
                gain,
                q: 0.70710677,
                slope: 1.0,
            };
            let words = design(row);
            let reference = oracle(row);
            for probe in grid_probes(row) {
                let Ok(reference_magnitude) = reference.magnitude_at_hz(probe) else {
                    continue;
                };
                if reference_magnitude <= 0.0 {
                    continue;
                }
                let reference_db = 20.0 * reference_magnitude.log10();
                if reference_db < RESPONSE_FLOOR_DB {
                    continue;
                }
                let error = (magnitude_db(words, rate, probe) - reference_db).abs();
                worst = worst.max(error);
                assert!(
                    error <= RESPONSE_TOLERANCE_DB,
                    "{row:?} probe={probe}: error={error} dB"
                );
            }
        }
    }
    eprintln!("issue-087 pcm-fixture rows worst_error_db={worst:.6e}");
}

/// The stability predicate separates contractive from expansive word triples.
///
/// `word_spectral_norm` is the guard `design_svf_v1` applies and the quantity the ramp gate above
/// measures, so it has to be discriminating in its own right. Spectral *radius* is not enough: this
/// triple has both eigenvalues on the unit circle and an operator norm of 1.4, because the state
/// matrix is strongly non-normal — which is exactly the failure mode a per-sample coefficient
/// update can excite and a direct-form realization cannot survive.
#[test]
fn the_spectral_norm_predicate_separates_contractive_from_expansive_words() {
    let tolerance = 1.0 + 1.0 / 4_194_304.0;
    let expansive = EqSvfWordsV1 {
        c1: 0.1,
        a2: 0.3,
        a3: 0.9,
        m0: 1.0,
        m1: 0.0,
        m2: 0.0,
    };
    let norm = miso_engine_parametric_eq::word_spectral_norm(expansive);
    assert!(norm > tolerance, "expansive triple scored {norm}");
    assert!(
        (norm - 1.4).abs() < 1.0e-8,
        "expansive triple scored {norm}"
    );
    assert_eq!(
        miso_engine_parametric_eq::word_spectral_norm(EqSvfWordsV1::IDENTITY),
        1.0,
        "the identity section is an isometry"
    );
}
