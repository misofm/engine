//! Issue-042 preimplementation-only retained-f32 realization comparison.
//!
//! This module is compiled solely in the independent reference crate's test boundary. Production
//! packages do not depend on this crate, so none of the candidate equations or selection results
//! are render-reachable. It intentionally evaluates candidate state transitions from their stored
//! f32 words rather than from the source f64 design.

use core::cmp::Ordering;
use core::f64::consts::{FRAC_1_SQRT_2, PI};

use crate::{ReferenceParametricEqCoefficients, ReferenceParametricEqKind};

const SAMPLE_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const FREQUENCIES: [f64; 6] = [10.0, 20.0, 100.0, 1_000.0, 10_000.0, 20_000.0];
const QS: [f64; 4] = [0.1, FRAC_1_SQRT_2, 1.0, 18.0];
const GAINS: [f64; 5] = [-24.0, -6.0, 0.0, 6.0, 24.0];
const SLOPES: [f64; 3] = [0.1, 0.5, 1.0];
const RESPONSE_DB_TOLERANCE: f64 = 0.005;
const NULL_MAGNITUDE_LIMIT: f64 = 1e-5;
const PROBE_COUNT: usize = 2_048;
const STATE_PROBE_SAMPLES: usize = 2_048;
const EXPECTED_SUMMARY_HASHES: [u64; 3] = [
    0xca96_986d_381e_3fe4,
    0xd500_4e7d_c41d_bb27,
    0x1bff_fc2d_8628_0ce8,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Family {
    TptStateVariable,
    CoupledForm,
    DeltaOperator,
}

impl Family {
    const ALL: [Self; 3] = [
        Self::TptStateVariable,
        Self::CoupledForm,
        Self::DeltaOperator,
    ];

    const fn name(self) -> &'static str {
        match self {
            Self::TptStateVariable => "tpt_state_variable",
            Self::CoupledForm => "coupled_form",
            Self::DeltaOperator => "delta_operator",
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Row {
    rate: u32,
    kind: ReferenceParametricEqKind,
    frequency: f64,
    gain: f64,
    q: f64,
    slope: f64,
}

#[derive(Clone, Copy)]
struct Tpt {
    c1: f32,
    a2: f32,
    a3: f32,
    k: f32,
    low: f32,
    band: f32,
    high: f32,
    s1: f32,
    s2: f32,
}

#[derive(Clone, Copy)]
struct Coupled {
    p: f32,
    a01: f32,
    a10: f32,
    direct: f32,
    c0: f32,
    c1: f32,
    s0: f32,
    s1: f32,
}

#[derive(Clone, Copy)]
struct Delta {
    n0: f32,
    n1: f32,
    n2: f32,
    d0: f32,
    d1: f32,
    d2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

#[derive(Clone, Copy)]
enum Candidate {
    Tpt(Tpt),
    Coupled(Coupled),
    Delta(Delta),
}

impl Candidate {
    fn design(
        family: Family,
        row: Row,
        reference: ReferenceParametricEqCoefficients,
    ) -> Result<Self, &'static str> {
        match family {
            Family::TptStateVariable => Self::design_tpt(row, reference),
            Family::CoupledForm => Self::design_coupled(reference),
            Family::DeltaOperator => Ok(Self::design_delta(reference)),
        }
    }

    fn design_tpt(
        row: Row,
        reference: ReferenceParametricEqCoefficients,
    ) -> Result<Self, &'static str> {
        let (b0, b1, b2, a1, a2) = reference.values();
        let plus = 1.0 + a1 + a2;
        let minus = 1.0 - a1 + a2;
        if !(plus > 0.0 && minus > 0.0) {
            return Err("tpt_denominator_not_bilinear_representable");
        }
        let g = (plus / minus).sqrt();
        let denominator = 4.0 / minus;
        let k = (denominator - 1.0 - g * g) / g;
        let t = g * (g + k);
        let tpt_denominator = 1.0 + t;
        let c1 = (t / tpt_denominator) as f32;
        let a2_tpt = (g / tpt_denominator) as f32;
        let a3 = (g * g / tpt_denominator) as f32;
        let k = k as f32;
        if ![c1, a2_tpt, a3, k].into_iter().all(f32::is_finite) {
            return Err("tpt_nonfinite_retained_word");
        }
        let base = Tpt {
            c1,
            a2: a2_tpt,
            a3,
            k,
            low: 0.0,
            band: 0.0,
            high: 0.0,
            s1: 0.0,
            s2: 0.0,
        };
        let target = [b0, b1 - a1 * b0, b2 - a1 * (b1 - a1 * b0) - a2 * b0];
        let (low, band, high) = tpt_basis_impulses(base);
        let matrix = [
            [low[0], band[0], high[0]],
            [low[1], band[1], high[1]],
            [low[2], band[2], high[2]],
        ];
        let [low_mix, band_mix, high_mix] =
            solve_3x3(matrix, target).ok_or("tpt_observation_basis_singular")?;
        let candidate = Tpt {
            low: low_mix as f32,
            band: band_mix as f32,
            high: high_mix as f32,
            ..base
        };
        if ![candidate.low, candidate.band, candidate.high]
            .into_iter()
            .all(f32::is_finite)
        {
            return Err("tpt_nonfinite_observation_word");
        }
        let _ = row;
        Ok(Self::Tpt(candidate))
    }

    fn design_coupled(reference: ReferenceParametricEqCoefficients) -> Result<Self, &'static str> {
        let (b0, b1, b2, a1, a2) = reference.values();
        if a2 == 0.0 {
            return Err("coupled_zero_determinant");
        }
        let p = -a1 * 0.5;
        let discriminant = a2 - p * p;
        let (a01, a10) = if discriminant >= 0.0 {
            let q = discriminant.sqrt();
            (-q, q)
        } else {
            let q = (-discriminant).sqrt();
            (q, q)
        };
        if a10 == 0.0 {
            return Err("coupled_repeated_pole_observation_singular");
        }
        let direct = b2 / a2;
        let c0 = b0 - direct;
        let c1 = (b1 + 2.0 * p * direct + p * c0) / a10;
        let candidate = Coupled {
            p: p as f32,
            a01: a01 as f32,
            a10: a10 as f32,
            direct: direct as f32,
            c0: c0 as f32,
            c1: c1 as f32,
            s0: 0.0,
            s1: 0.0,
        };
        if [
            candidate.p,
            candidate.a01,
            candidate.a10,
            candidate.direct,
            candidate.c0,
            candidate.c1,
        ]
        .into_iter()
        .all(f32::is_finite)
        {
            Ok(Self::Coupled(candidate))
        } else {
            Err("coupled_nonfinite_retained_word")
        }
    }

    fn design_delta(reference: ReferenceParametricEqCoefficients) -> Self {
        let (b0, b1, b2, a1, a2) = reference.values();
        Self::Delta(Delta {
            n0: (b0 + b1 + b2) as f32,
            n1: (b1 + 2.0 * b2) as f32,
            n2: b2 as f32,
            d0: (1.0 + a1 + a2) as f32,
            d1: (a1 + 2.0 * a2) as f32,
            d2: a2 as f32,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        })
    }

    fn magnitude(self, frequency: f64, sample_rate: f64) -> Option<f64> {
        match self {
            Self::Tpt(candidate) => tpt_magnitude(candidate, frequency, sample_rate),
            Self::Coupled(candidate) => coupled_magnitude(candidate, frequency, sample_rate),
            Self::Delta(candidate) => delta_magnitude(candidate, frequency, sample_rate),
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        match self {
            Self::Tpt(candidate) => candidate.process(input),
            Self::Coupled(candidate) => candidate.process(input),
            Self::Delta(candidate) => candidate.process(input),
        }
    }

    fn finite_state(self) -> bool {
        match self {
            Self::Tpt(candidate) => [candidate.s1, candidate.s2].into_iter().all(normal_or_zero),
            Self::Coupled(candidate) => {
                [candidate.s0, candidate.s1].into_iter().all(normal_or_zero)
            }
            Self::Delta(candidate) => [candidate.x1, candidate.x2, candidate.y1, candidate.y2]
                .into_iter()
                .all(normal_or_zero),
        }
    }

    fn words(self) -> ([u32; 8], usize) {
        match self {
            Self::Tpt(candidate) => (
                [
                    candidate.c1.to_bits(),
                    candidate.a2.to_bits(),
                    candidate.a3.to_bits(),
                    candidate.k.to_bits(),
                    candidate.low.to_bits(),
                    candidate.band.to_bits(),
                    candidate.high.to_bits(),
                    0,
                ],
                7,
            ),
            Self::Coupled(candidate) => (
                [
                    candidate.p.to_bits(),
                    candidate.a01.to_bits(),
                    candidate.a10.to_bits(),
                    candidate.direct.to_bits(),
                    candidate.c0.to_bits(),
                    candidate.c1.to_bits(),
                    0,
                    0,
                ],
                6,
            ),
            Self::Delta(candidate) => (
                [
                    candidate.n0.to_bits(),
                    candidate.n1.to_bits(),
                    candidate.n2.to_bits(),
                    candidate.d0.to_bits(),
                    candidate.d1.to_bits(),
                    candidate.d2.to_bits(),
                    0,
                    0,
                ],
                6,
            ),
        }
    }
}

impl Tpt {
    fn process(&mut self, input: f32) -> f32 {
        let v3 = input - self.s2;
        let p1 = self.a2 * v3;
        let p2 = self.c1 * self.s1;
        let d1 = p1 - p2;
        let band = self.s1 + d1;
        let p3 = self.a2 * self.s1;
        let p4 = self.a3 * v3;
        let d2 = p3 + p4;
        let low = self.s2 + d2;
        let high = (input - self.k * band) - low;
        self.s1 += d1 + d1;
        self.s2 += d2 + d2;
        self.low * low + self.band * band + self.high * high
    }
}

impl Coupled {
    fn process(&mut self, input: f32) -> f32 {
        let old_s0 = self.s0;
        let old_s1 = self.s1;
        self.s0 = (self.p * old_s0 + self.a01 * old_s1) + input;
        self.s1 = self.a10 * old_s0 + self.p * old_s1;
        self.direct * input + self.c0 * self.s0 + self.c1 * self.s1
    }
}

impl Delta {
    fn process(&mut self, input: f32) -> f32 {
        let dx = self.x1 - input;
        let ddx = (self.x2 - self.x1) - dx;
        let numerator = (self.n0 * input + self.n1 * dx) + self.n2 * ddx;
        let scale = (self.d0 - self.d1) + self.d2;
        let history = (self.d1 - self.d2 - self.d2) * self.y1 + self.d2 * self.y2;
        let output = (numerator - history) / scale;
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;
        output
    }
}

fn tpt_basis_impulses(candidate: Tpt) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let mut candidate = candidate;
    let mut low = [0.0; 3];
    let mut band = [0.0; 3];
    let mut high = [0.0; 3];
    for index in 0..3 {
        let input = if index == 0 { 1.0_f32 } else { 0.0 };
        let v3 = input - candidate.s2;
        let d1 = candidate.a2 * v3 - candidate.c1 * candidate.s1;
        let band_output = candidate.s1 + d1;
        let d2 = candidate.a2 * candidate.s1 + candidate.a3 * v3;
        let low_output = candidate.s2 + d2;
        let high_output = (input - candidate.k * band_output) - low_output;
        candidate.s1 += d1 + d1;
        candidate.s2 += d2 + d2;
        low[index] = f64::from(low_output);
        band[index] = f64::from(band_output);
        high[index] = f64::from(high_output);
    }
    (low, band, high)
}

fn solve_3x3(matrix: [[f64; 3]; 3], target: [f64; 3]) -> Option<[f64; 3]> {
    let determinant = determinant_3x3(matrix);
    if !determinant.is_finite() || determinant.abs() < 1e-30 {
        return None;
    }
    let mut result = [0.0; 3];
    for column in 0..3 {
        let mut replacement = matrix;
        for row in 0..3 {
            replacement[row][column] = target[row];
        }
        result[column] = determinant_3x3(replacement) / determinant;
    }
    result.into_iter().all(f64::is_finite).then_some(result)
}

fn determinant_3x3(matrix: [[f64; 3]; 3]) -> f64 {
    matrix[0][0] * (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1])
        - matrix[0][1] * (matrix[1][0] * matrix[2][2] - matrix[1][2] * matrix[2][0])
        + matrix[0][2] * (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0])
}

fn tpt_magnitude(candidate: Tpt, frequency: f64, sample_rate: f64) -> Option<f64> {
    let c1 = f64::from(candidate.c1);
    let a2 = f64::from(candidate.a2);
    let a3 = f64::from(candidate.a3);
    let k = f64::from(candidate.k);
    let a00 = 1.0 - 2.0 * c1;
    let a01 = -2.0 * a2;
    let a10 = 2.0 * a2;
    let a11 = 1.0 - 2.0 * a3;
    let b = [2.0 * a2, 2.0 * a3];
    let low_c = [a2, 1.0 - a3];
    let low_d = a3;
    let band_c = [1.0 - c1, -a2];
    let band_d = a2;
    let high_c = [-k * (1.0 - c1) - a2, k * a2 - (1.0 - a3)];
    let high_d = 1.0 - k * a2 - a3;
    let c = [
        f64::from(candidate.low) * low_c[0]
            + f64::from(candidate.band) * band_c[0]
            + f64::from(candidate.high) * high_c[0],
        f64::from(candidate.low) * low_c[1]
            + f64::from(candidate.band) * band_c[1]
            + f64::from(candidate.high) * high_c[1],
    ];
    let direct = f64::from(candidate.low) * low_d
        + f64::from(candidate.band) * band_d
        + f64::from(candidate.high) * high_d;
    state_space_magnitude(
        StateSpace {
            a00,
            a01,
            a10,
            a11,
            b,
            c,
            direct,
        },
        frequency,
        sample_rate,
    )
}

fn coupled_magnitude(candidate: Coupled, frequency: f64, sample_rate: f64) -> Option<f64> {
    state_space_magnitude(
        StateSpace {
            a00: f64::from(candidate.p),
            a01: f64::from(candidate.a01),
            a10: f64::from(candidate.a10),
            a11: f64::from(candidate.p),
            b: [1.0, 0.0],
            c: [f64::from(candidate.c0), f64::from(candidate.c1)],
            direct: f64::from(candidate.direct),
        },
        frequency,
        sample_rate,
    )
}

#[derive(Clone, Copy)]
struct StateSpace {
    a00: f64,
    a01: f64,
    a10: f64,
    a11: f64,
    b: [f64; 2],
    c: [f64; 2],
    direct: f64,
}

fn state_space_magnitude(state: StateSpace, frequency: f64, sample_rate: f64) -> Option<f64> {
    let phase = 2.0 * PI * frequency / sample_rate;
    let zr = phase.cos();
    let zi = phase.sin();
    let m00r = zr - state.a00;
    let m11r = zr - state.a11;
    let m01 = -state.a01;
    let m10 = -state.a10;
    let determinant_r = m00r * m11r - zi * zi - m01 * m10;
    let determinant_i = zi * (m00r + m11r);
    let norm = determinant_r * determinant_r + determinant_i * determinant_i;
    if norm == 0.0 || !norm.is_finite() {
        return None;
    }
    let x0r = ((m11r * state.b[0] - m01 * state.b[1]) * determinant_r
        + zi * state.b[0] * determinant_i)
        / norm;
    let x0i = (zi * state.b[0] * determinant_r
        - (m11r * state.b[0] - m01 * state.b[1]) * determinant_i)
        / norm;
    let x1r = ((-m10 * state.b[0] + m00r * state.b[1]) * determinant_r
        + zi * state.b[1] * determinant_i)
        / norm;
    let x1i = (zi * state.b[1] * determinant_r
        - (-m10 * state.b[0] + m00r * state.b[1]) * determinant_i)
        / norm;
    let response_r = state.direct + state.c[0] * x0r + state.c[1] * x1r;
    let response_i = state.c[0] * x0i + state.c[1] * x1i;
    response_r
        .hypot(response_i)
        .is_finite()
        .then_some(response_r.hypot(response_i))
}

fn delta_magnitude(candidate: Delta, frequency: f64, sample_rate: f64) -> Option<f64> {
    let phase = 2.0 * PI * frequency / sample_rate;
    let w_r = phase.cos() - 1.0;
    let w_i = -phase.sin();
    let w2_r = w_r * w_r - w_i * w_i;
    let w2_i = 2.0 * w_r * w_i;
    let numerator_r =
        f64::from(candidate.n0) + f64::from(candidate.n1) * w_r + f64::from(candidate.n2) * w2_r;
    let numerator_i = f64::from(candidate.n1) * w_i + f64::from(candidate.n2) * w2_i;
    let denominator_r =
        f64::from(candidate.d0) + f64::from(candidate.d1) * w_r + f64::from(candidate.d2) * w2_r;
    let denominator_i = f64::from(candidate.d1) * w_i + f64::from(candidate.d2) * w2_i;
    let denominator = denominator_r.hypot(denominator_i);
    let magnitude = numerator_r.hypot(numerator_i) / denominator;
    magnitude.is_finite().then_some(magnitude)
}

fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && (value == 0.0 || value.is_normal())
}

#[derive(Clone, Copy, Debug)]
struct Failure {
    category: &'static str,
    row: Row,
    probe: f64,
    observed: f64,
    expected: f64,
}

#[derive(Clone, Copy, Debug)]
struct CandidateSummary {
    family: Family,
    rows: u32,
    design_failures: u32,
    response_failures: u32,
    null_failures: u32,
    center_failures: u32,
    state_failures: u32,
    worst_db_error: f64,
    worst_stability_margin: f64,
    max_state: f64,
    first_failure: Option<Failure>,
    hash: u64,
}

impl CandidateSummary {
    const fn new(family: Family) -> Self {
        Self {
            family,
            rows: 0,
            design_failures: 0,
            response_failures: 0,
            null_failures: 0,
            center_failures: 0,
            state_failures: 0,
            worst_db_error: 0.0,
            worst_stability_margin: f64::INFINITY,
            max_state: 0.0,
            first_failure: None,
            hash: 14_695_981_039_346_656_037,
        }
    }

    fn fail(&mut self, category: &'static str, row: Row, probe: f64, observed: f64, expected: f64) {
        if self.first_failure.is_none() {
            self.first_failure = Some(Failure {
                category,
                row,
                probe,
                observed,
                expected,
            });
        }
    }

    fn add_word(&mut self, word: u64) {
        self.hash ^= word;
        self.hash = self.hash.wrapping_mul(1_099_511_628_211);
    }

    fn selectable(self) -> bool {
        self.design_failures == 0
            && self.response_failures == 0
            && self.null_failures == 0
            && self.center_failures == 0
            && self.state_failures == 0
    }
}

fn compare(family: Family) -> CandidateSummary {
    let mut summary = CandidateSummary::new(family);
    for_each_row(|row| {
        summary.rows += 1;
        summary.add_word(u64::from(row.rate));
        summary.add_word(row.frequency.to_bits());
        summary.add_word(row.gain.to_bits());
        summary.add_word(row.q.to_bits());
        summary.add_word(row.slope.to_bits());
        let reference = ReferenceParametricEqCoefficients::design(
            row.kind,
            f64::from(row.rate),
            row.frequency,
            row.gain,
            row.q,
            row.slope,
        )
        .expect("frozen grid is legal for f64 reference");
        let Ok(candidate) = Candidate::design(family, row, reference) else {
            summary.design_failures += 1;
            summary.fail("design", row, row.frequency, f64::NAN, 0.0);
            return;
        };
        let (words, len) = candidate.words();
        for word in &words[..len] {
            summary.add_word(u64::from(*word));
        }
        let stability = stability_margin(candidate);
        summary.worst_stability_margin = summary.worst_stability_margin.min(stability);
        if stability.partial_cmp(&0.0) != Some(Ordering::Greater) {
            summary.design_failures += 1;
            summary.fail("stability", row, row.frequency, stability, 0.0);
        }
        let probes = comparison_probes(row.rate, row.frequency);
        for probe in probes {
            let reference_magnitude = reference
                .magnitude_at_hz(probe)
                .expect("frozen probe remains in reference domain");
            let candidate_magnitude = candidate.magnitude(probe, f64::from(row.rate));
            let Some(candidate_magnitude) = candidate_magnitude else {
                summary.response_failures += 1;
                summary.fail(
                    "analytic_nonfinite",
                    row,
                    probe,
                    f64::NAN,
                    reference_magnitude,
                );
                continue;
            };
            if reference_magnitude > 0.0 {
                let reference_db = 20.0 * reference_magnitude.log10();
                if reference_db >= -120.0 && candidate_magnitude > 0.0 {
                    let candidate_db = 20.0 * candidate_magnitude.log10();
                    let error = (candidate_db - reference_db).abs();
                    summary.worst_db_error = summary.worst_db_error.max(error);
                    if error > RESPONSE_DB_TOLERANCE {
                        summary.response_failures += 1;
                        summary.fail("analytic_db", row, probe, candidate_db, reference_db);
                    }
                } else if reference_db >= -120.0 {
                    summary.response_failures += 1;
                    summary.fail(
                        "analytic_zero",
                        row,
                        probe,
                        candidate_magnitude,
                        reference_magnitude,
                    );
                }
            }
        }
        apply_family_frequency_gates(&mut summary, row, candidate);
        let (state_ok, state_max) = scalar_state_probe(candidate, summary.rows);
        summary.max_state = summary.max_state.max(state_max);
        if !state_ok {
            summary.state_failures += 1;
            summary.fail("state_probe", row, row.frequency, state_max, 0.0);
        }
    });
    summary
}

fn comparison_probes(rate: u32, frequency: f64) -> Vec<f64> {
    let mut probes = Vec::with_capacity(PROBE_COUNT + 3);
    let ratio = 2_000.0_f64;
    for index in 0..PROBE_COUNT {
        probes.push(10.0 * ratio.powf(index as f64 / (PROBE_COUNT - 1) as f64));
    }
    probes.push(frequency);
    probes.push(0.0);
    probes.push(f64::from(rate) * 0.5);
    probes
}

fn apply_family_frequency_gates(summary: &mut CandidateSummary, row: Row, candidate: Candidate) {
    let magnitude = candidate
        .magnitude(row.frequency, f64::from(row.rate))
        .unwrap_or(f64::NAN);
    match row.kind {
        ReferenceParametricEqKind::Notch => {
            if magnitude > NULL_MAGNITUDE_LIMIT || !magnitude.is_finite() {
                summary.null_failures += 1;
                summary.fail(
                    "notch_null",
                    row,
                    row.frequency,
                    magnitude,
                    NULL_MAGNITUDE_LIMIT,
                );
            }
        }
        ReferenceParametricEqKind::LowPass | ReferenceParametricEqKind::HighPass
            if (row.q - FRAC_1_SQRT_2).abs() < f64::EPSILON =>
        {
            let db = 20.0 * magnitude.log10();
            if (db + 3.010_299_956_6).abs() > RESPONSE_DB_TOLERANCE {
                summary.center_failures += 1;
                summary.fail(
                    "butterworth_cutoff",
                    row,
                    row.frequency,
                    db,
                    -3.010_299_956_6,
                );
            }
        }
        ReferenceParametricEqKind::Bell if row.gain != 0.0 => {
            let db = 20.0 * magnitude.log10();
            if (db - row.gain).abs() > RESPONSE_DB_TOLERANCE {
                summary.center_failures += 1;
                summary.fail("bell_center", row, row.frequency, db, row.gain);
            }
        }
        ReferenceParametricEqKind::LowShelf | ReferenceParametricEqKind::HighShelf
            if row.gain != 0.0 =>
        {
            let db = 20.0 * magnitude.log10();
            if (db - row.gain * 0.5).abs() > RESPONSE_DB_TOLERANCE {
                summary.center_failures += 1;
                summary.fail("shelf_midpoint", row, row.frequency, db, row.gain * 0.5);
            }
        }
        _ => {}
    }
}

fn stability_margin(candidate: Candidate) -> f64 {
    let (a1, a2) = match candidate {
        Candidate::Tpt(value) => {
            let a00 = 1.0 - 2.0 * f64::from(value.c1);
            let a01 = -2.0 * f64::from(value.a2);
            let a10 = 2.0 * f64::from(value.a2);
            let a11 = 1.0 - 2.0 * f64::from(value.a3);
            (-(a00 + a11), a00 * a11 - a01 * a10)
        }
        Candidate::Coupled(value) => {
            let p = f64::from(value.p);
            (
                -2.0 * p,
                p * p - f64::from(value.a01) * f64::from(value.a10),
            )
        }
        Candidate::Delta(value) => {
            let a1 = f64::from(value.d1) - 2.0 * f64::from(value.d2);
            (a1, f64::from(value.d2))
        }
    };
    (1.0 - a2.abs()).min(1.0 + a1 + a2).min(1.0 - a1 + a2)
}

fn scalar_state_probe(mut candidate: Candidate, seed: u32) -> (bool, f64) {
    let mut state = u64::from(seed) ^ 0x4d59_5df4_d0f3_3173;
    let mut maximum = 0.0_f64;
    for index in 0..STATE_PROBE_SAMPLES {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        let noise = (((state >> 40) as i32) as f32 / 8_388_608.0) * 0.99;
        let input = if index == 0 { 0.5 } else { noise };
        let output = candidate.process(input);
        maximum = maximum.max(f64::from(output.abs()));
        if !normal_or_zero(output) || !candidate.finite_state() {
            return (false, maximum);
        }
    }
    (true, maximum)
}

fn for_each_row(mut visit: impl FnMut(Row)) {
    for rate in SAMPLE_RATES {
        for frequency in FREQUENCIES {
            for q in QS {
                for gain in GAINS {
                    visit(Row {
                        rate,
                        kind: ReferenceParametricEqKind::Bell,
                        frequency,
                        gain,
                        q,
                        slope: 1.0,
                    });
                }
                visit(Row {
                    rate,
                    kind: ReferenceParametricEqKind::LowPass,
                    frequency,
                    gain: 0.0,
                    q,
                    slope: 1.0,
                });
                visit(Row {
                    rate,
                    kind: ReferenceParametricEqKind::HighPass,
                    frequency,
                    gain: 0.0,
                    q,
                    slope: 1.0,
                });
                visit(Row {
                    rate,
                    kind: ReferenceParametricEqKind::Notch,
                    frequency,
                    gain: 0.0,
                    q,
                    slope: 1.0,
                });
            }
            for gain in GAINS {
                for slope in SLOPES {
                    visit(Row {
                        rate,
                        kind: ReferenceParametricEqKind::LowShelf,
                        frequency,
                        gain,
                        q: 1.0,
                        slope,
                    });
                    visit(Row {
                        rate,
                        kind: ReferenceParametricEqKind::HighShelf,
                        frequency,
                        gain,
                        q: 1.0,
                        slope,
                    });
                }
            }
        }
    }
}

#[test]
fn issue_042_complete_retained_f32_candidate_comparison_requires_sol_freeze() {
    let summaries = Family::ALL.map(compare);
    for (index, summary) in summaries.into_iter().enumerate() {
        let first = summary
            .first_failure
            .expect("every candidate has a frozen-gate failure");
        println!(
            "issue-042 candidate={} rows={} design={} response={} null={} center={} state={} worst_db={:.12} margin={:.12e} max_state={:.12e} hash={:016x} first_category={} first_rate={} first_kind={:?} first_f0={} first_gain={} first_q={} first_s={} first_probe={} first_observed={:.15e} first_expected={:.15e}",
            summary.family.name(),
            summary.rows,
            summary.design_failures,
            summary.response_failures,
            summary.null_failures,
            summary.center_failures,
            summary.state_failures,
            summary.worst_db_error,
            summary.worst_stability_margin,
            summary.max_state,
            summary.hash,
            first.category,
            first.row.rate,
            first.row.kind,
            first.row.frequency,
            first.row.gain,
            first.row.q,
            first.row.slope,
            first.probe,
            first.observed,
            first.expected,
        );
        assert_eq!(summary.rows, 1_488);
        assert_eq!(summary.hash, EXPECTED_SUMMARY_HASHES[index]);
        assert!(!summary.selectable());
    }
}
