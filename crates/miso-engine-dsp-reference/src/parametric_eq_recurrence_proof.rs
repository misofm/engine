//! Issue-045 phase-one-only f64 recurrence derivations.
//!
//! This test-boundary module intentionally stops before retained `f32` evaluation. The eventual
//! single ignored full-matrix invocation must extend this derivation; it must not execute this
//! phase separately and combine transcripts later.

use core::f64::consts::FRAC_1_SQRT_2;

use crate::{
    ReferenceParametricEqCoefficients, ReferenceParametricEqKind, ReferenceParametricEqSection,
};

const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const FREQUENCIES: [f64; 6] = [10.0, 20.0, 100.0, 1_000.0, 10_000.0, 20_000.0];
const QS: [f64; 4] = [0.1, FRAC_1_SQRT_2, 1.0, 18.0];
const GAINS: [f64; 5] = [-24.0, -6.0, 0.0, 6.0, 24.0];
const SLOPES: [f64; 3] = [0.1, 0.5, 1.0];
const IMPULSE_SAMPLES: usize = 4_096;
const F64_TOLERANCE: f64 = 1e-12;
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const EQUATION_VERSION: u64 = 0x4930_3435_5048_4153; // I045_PHAS

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CandidateId {
    L1,
    D2,
    B3,
}

impl CandidateId {
    const ALL: [Self; 3] = [Self::L1, Self::D2, Self::B3];

    const fn name(self) -> &'static str {
        match self {
            Self::L1 => "L1",
            Self::D2 => "D2",
            Self::B3 => "B3",
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

impl Row {
    fn reference(self) -> ReferenceParametricEqCoefficients {
        ReferenceParametricEqCoefficients::design(
            self.kind,
            f64::from(self.rate),
            self.frequency,
            self.gain,
            self.q,
            self.slope,
        )
        .expect("the frozen Issue-042 grid is legal")
    }

    const fn anchor(self) -> f64 {
        if self.frequency <= self.rate as f64 * 0.25 {
            1.0
        } else {
            -1.0
        }
    }
}

fn rows() -> Vec<Row> {
    let mut output = Vec::with_capacity(1_488);
    for rate in RATES {
        for frequency in FREQUENCIES {
            for q in QS {
                for gain in GAINS {
                    output.push(Row {
                        rate,
                        kind: ReferenceParametricEqKind::Bell,
                        frequency,
                        gain,
                        q,
                        slope: 1.0,
                    });
                }
                for kind in [
                    ReferenceParametricEqKind::LowPass,
                    ReferenceParametricEqKind::HighPass,
                    ReferenceParametricEqKind::Notch,
                ] {
                    output.push(Row {
                        rate,
                        kind,
                        frequency,
                        gain: 0.0,
                        q,
                        slope: 1.0,
                    });
                }
            }
            for gain in GAINS {
                for slope in SLOPES {
                    for kind in [
                        ReferenceParametricEqKind::LowShelf,
                        ReferenceParametricEqKind::HighShelf,
                    ] {
                        output.push(Row {
                            rate,
                            kind,
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
    assert_eq!(output.len(), 1_488);
    output
}

#[derive(Clone, Copy)]
struct L1 {
    a: f64,
    n0: f64,
    n1: f64,
    n2: f64,
    k1: f64,
    k2: f64,
    x1: f64,
    x2: f64,
    s0: f64,
    s1: f64,
}

impl L1 {
    fn process(&mut self, x: f64) -> f64 {
        let dx = self.x1 - self.a * x;
        let ddx = (self.x2 - self.a * self.x1) - self.a * dx;
        let e = (self.n0 * x + self.n1 * dx) + self.n2 * ddx;
        let f1 = e - self.k2 * self.s1;
        let y = f1 - self.k1 * self.s0;
        self.s1 = self.k1 * y + self.s0;
        self.s0 = y;
        self.x2 = self.x1;
        self.x1 = x;
        y
    }

    fn direct_form(self) -> [f64; 5] {
        let b1 = self.n1 - 2.0 * self.a * self.n2;
        [
            self.n0 - self.a * b1 - self.n2,
            b1,
            self.n2,
            self.k1 * (1.0 + self.k2),
            self.k2,
        ]
    }
}

#[derive(Clone, Copy)]
struct D2 {
    a: f64,
    n0: f64,
    d0: f64,
    n1: f64,
    d1: f64,
    n2: f64,
    d2: f64,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

impl D2 {
    fn process(&mut self, x: f64) -> f64 {
        let dx = self.x1 - self.a * x;
        let ddx = (self.x2 - self.a * self.x1) - self.a * dx;
        let numerator = (self.n0 * x + self.n1 * dx) + self.n2 * ddx;
        let scale = (self.d0 - self.a * self.d1) + self.d2;
        let q1 = self.a * self.d2;
        let q2 = (self.d1 - q1) - q1;
        let history = q2 * self.y1 + self.d2 * self.y2;
        let y = (numerator - history) / scale;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }

    fn direct_form(self) -> [f64; 5] {
        let scale = (self.d0 - self.a * self.d1) + self.d2;
        [
            (self.n0 - self.a * self.n1 + self.n2) / scale,
            (self.n1 - 2.0 * self.a * self.n2) / scale,
            self.n2 / scale,
            (self.d1 - 2.0 * self.a * self.d2) / scale,
            self.d2 / scale,
        ]
    }
}

#[derive(Clone, Copy)]
struct B3 {
    a00: f64,
    a01: f64,
    a10: f64,
    a11: f64,
    b0: f64,
    b1: f64,
    c0: f64,
    c1: f64,
    d: f64,
    s0: f64,
    s1: f64,
}

impl B3 {
    fn process(&mut self, x: f64) -> f64 {
        let y = (self.d * x + self.c0 * self.s0) + self.c1 * self.s1;
        let n0 = (self.a00 * self.s0 + self.a01 * self.s1) + self.b0 * x;
        let n1 = (self.a10 * self.s0 + self.a11 * self.s1) + self.b1 * x;
        self.s0 = n0;
        self.s1 = n1;
        y
    }

    fn direct_form(self) -> [f64; 5] {
        let a1 = -(self.a00 + self.a11);
        let a2 = self.a00 * self.a11 - self.a01 * self.a10;
        [
            self.d,
            self.d * a1 + self.c0 * self.b0 + self.c1 * self.b1,
            self.d * a2
                + self.c0 * (-self.a11 * self.b0 + self.a01 * self.b1)
                + self.c1 * (self.a10 * self.b0 - self.a00 * self.b1),
            a1,
            a2,
        ]
    }
}

#[derive(Clone, Copy)]
enum Candidate {
    Identity,
    L1(L1),
    D2(D2),
    B3(B3),
}

impl Candidate {
    fn design(
        id: CandidateId,
        row: Row,
        reference: ReferenceParametricEqCoefficients,
    ) -> Result<Self, &'static str> {
        if reference.is_identity() {
            return Ok(Self::Identity);
        }
        let (b0, b1, b2, a1, a2) = reference.values();
        match id {
            CandidateId::L1 => {
                let denominator = 1.0 + a2;
                if denominator == 0.0 {
                    return Err("l1_zero_k1_denominator");
                }
                let a = row.anchor();
                Ok(Self::L1(L1 {
                    a,
                    n0: b0 + a * b1 + b2,
                    n1: b1 + 2.0 * a * b2,
                    n2: b2,
                    k1: a1 / denominator,
                    k2: a2,
                    x1: 0.0,
                    x2: 0.0,
                    s0: 0.0,
                    s1: 0.0,
                }))
            }
            CandidateId::D2 => {
                let a = row.anchor();
                Ok(Self::D2(D2 {
                    a,
                    n0: b0 + a * b1 + b2,
                    d0: 1.0 + a * a1 + a2,
                    n1: b1 + 2.0 * a * b2,
                    d1: a1 + 2.0 * a * a2,
                    n2: b2,
                    d2: a2,
                    x1: 0.0,
                    x2: 0.0,
                    y1: 0.0,
                    y2: 0.0,
                }))
            }
            CandidateId::B3 => balanced_b3(b0, b1, b2, a1, a2).map(Self::B3),
        }
    }

    fn process(&mut self, x: f64) -> f64 {
        match self {
            Self::Identity => x,
            Self::L1(value) => value.process(x),
            Self::D2(value) => value.process(x),
            Self::B3(value) => value.process(x),
        }
    }

    fn direct_form(self) -> [f64; 5] {
        match self {
            Self::Identity => [1.0, 0.0, 0.0, 0.0, 0.0],
            Self::L1(value) => value.direct_form(),
            Self::D2(value) => value.direct_form(),
            Self::B3(value) => value.direct_form(),
        }
    }
}

/// The frozen deterministic balanced realization: solve both Lyapunov equations, lower-Cholesky
/// P, descending symmetric eigensystem, canonical eigenvector signs, then the stated similarity.
fn balanced_b3(b0: f64, b1: f64, b2: f64, a1: f64, a2: f64) -> Result<B3, &'static str> {
    let a = [[-a1, 1.0], [-a2, 0.0]];
    let b = [b1 - a1 * b0, b2 - a2 * b0];
    let p = lyapunov(
        a,
        [[b[0] * b[0], b[0] * b[1]], [b[0] * b[1], b[1] * b[1]]],
        false,
    )?;
    let q = lyapunov(a, [[1.0, 0.0], [0.0, 0.0]], true)?;
    let r00 = p[0][0].sqrt();
    if !(r00.is_finite() && r00 > 0.0) {
        return Err("b3_nonpositive_controllability");
    }
    let r10 = p[0][1] / r00;
    let r11 = (p[1][1] - r10 * r10).sqrt();
    if !(r11.is_finite() && r11 > 0.0) {
        return Err("b3_nonpositive_cholesky");
    }
    let r = [[r00, 0.0], [r10, r11]];
    let gram = multiply(transpose(r), multiply(q, r));
    let (sigma_squared, u) = symmetric_eigen(gram)?;
    let sigma = [sigma_squared[0].sqrt(), sigma_squared[1].sqrt()];
    if !sigma
        .into_iter()
        .all(|value| value.is_finite() && value > 0.0)
    {
        return Err("b3_nonpositive_hankel_singular_value");
    }
    let mut t = multiply(r, u);
    t[0][0] /= sigma[0].sqrt();
    t[0][1] /= sigma[1].sqrt();
    t[1][0] /= sigma[0].sqrt();
    t[1][1] /= sigma[1].sqrt();
    let inverse_t = inverse(t)?;
    let ab = multiply(multiply(inverse_t, a), t);
    let bb = multiply_vector(inverse_t, b);
    let cb = vector_multiply([1.0, 0.0], t);
    let words = [
        ab[0][0], ab[0][1], ab[1][0], ab[1][1], bb[0], bb[1], cb[0], cb[1], b0,
    ];
    if !words.into_iter().all(f64::is_finite) {
        return Err("b3_nonfinite_balanced_word");
    }
    Ok(B3 {
        a00: ab[0][0],
        a01: ab[0][1],
        a10: ab[1][0],
        a11: ab[1][1],
        b0: bb[0],
        b1: bb[1],
        c0: cb[0],
        c1: cb[1],
        d: b0,
        s0: 0.0,
        s1: 0.0,
    })
}

fn lyapunov(
    a: [[f64; 2]; 2],
    rhs: [[f64; 2]; 2],
    transposed: bool,
) -> Result<[[f64; 2]; 2], &'static str> {
    let basis = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let mut system = [[0.0; 3]; 3];
    for (column, vector) in basis.into_iter().enumerate() {
        let x = [[vector[0], vector[1]], [vector[1], vector[2]]];
        let axat = if transposed {
            multiply(multiply(transpose(a), x), a)
        } else {
            multiply(multiply(a, x), transpose(a))
        };
        for row in 0..3 {
            system[row][column] = vector[row] - [axat[0][0], axat[0][1], axat[1][1]][row];
        }
    }
    let [x00, x01, x11] = solve_3x3(system, [rhs[0][0], rhs[0][1], rhs[1][1]])?;
    Ok([[x00, x01], [x01, x11]])
}

fn solve_3x3(mut matrix: [[f64; 3]; 3], mut rhs: [f64; 3]) -> Result<[f64; 3], &'static str> {
    for pivot in 0..3 {
        let mut selected = pivot;
        for row in pivot + 1..3 {
            if matrix[row][pivot].abs() > matrix[selected][pivot].abs() {
                selected = row;
            }
        }
        if !matrix[selected][pivot].is_finite() || matrix[selected][pivot].abs() <= 1e-30 {
            return Err("b3_singular_lyapunov_system");
        }
        if selected != pivot {
            matrix.swap(selected, pivot);
            rhs.swap(selected, pivot);
        }
        let divisor = matrix[pivot][pivot];
        for value in matrix[pivot].iter_mut().skip(pivot) {
            *value /= divisor;
        }
        rhs[pivot] /= divisor;
        let pivot_values = matrix[pivot];
        for row in 0..3 {
            if row == pivot {
                continue;
            }
            let factor = matrix[row][pivot];
            for (column, value) in matrix[row].iter_mut().enumerate().skip(pivot) {
                *value -= factor * pivot_values[column];
            }
            rhs[row] -= factor * rhs[pivot];
        }
    }
    rhs.into_iter()
        .all(f64::is_finite)
        .then_some(rhs)
        .ok_or("b3_nonfinite_lyapunov_solution")
}

fn symmetric_eigen(matrix: [[f64; 2]; 2]) -> Result<([f64; 2], [[f64; 2]; 2]), &'static str> {
    let half_trace = (matrix[0][0] + matrix[1][1]) * 0.5;
    let radius = ((matrix[0][0] - matrix[1][1]) * 0.5).hypot(matrix[0][1]);
    let high = half_trace + radius;
    let low = half_trace - radius;
    if !(high.is_finite() && low.is_finite() && high > 0.0 && low > 0.0) {
        return Err("b3_nonpositive_eigenvalue");
    }
    if (high - low).abs() <= high.abs().max(low.abs()) * 1e-14 {
        return Err("b3_repeated_noncanonical_eigenbasis");
    }
    let mut first = if matrix[0][1].abs() > (high - matrix[0][0]).abs() {
        [matrix[0][1], high - matrix[0][0]]
    } else {
        [high - matrix[1][1], matrix[0][1]]
    };
    let norm = first[0].hypot(first[1]);
    if !(norm.is_finite() && norm > 0.0) {
        return Err("b3_degenerate_eigenvector");
    }
    first[0] /= norm;
    first[1] /= norm;
    if first[0] < 0.0 || (first[0] == 0.0 && first[1] < 0.0) {
        first = [-first[0], -first[1]];
    }
    let mut second = [-first[1], first[0]];
    if second[0] < 0.0 || (second[0] == 0.0 && second[1] < 0.0) {
        second = [-second[0], -second[1]];
    }
    Ok(([high, low], [[first[0], second[0]], [first[1], second[1]]]))
}

fn multiply(left: [[f64; 2]; 2], right: [[f64; 2]; 2]) -> [[f64; 2]; 2] {
    [
        [
            left[0][0] * right[0][0] + left[0][1] * right[1][0],
            left[0][0] * right[0][1] + left[0][1] * right[1][1],
        ],
        [
            left[1][0] * right[0][0] + left[1][1] * right[1][0],
            left[1][0] * right[0][1] + left[1][1] * right[1][1],
        ],
    ]
}

const fn transpose(matrix: [[f64; 2]; 2]) -> [[f64; 2]; 2] {
    [[matrix[0][0], matrix[1][0]], [matrix[0][1], matrix[1][1]]]
}

fn inverse(matrix: [[f64; 2]; 2]) -> Result<[[f64; 2]; 2], &'static str> {
    let determinant = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    if !determinant.is_finite() || determinant.abs() <= 1e-30 {
        return Err("b3_singular_similarity");
    }
    Ok([
        [matrix[1][1] / determinant, -matrix[0][1] / determinant],
        [-matrix[1][0] / determinant, matrix[0][0] / determinant],
    ])
}

fn multiply_vector(matrix: [[f64; 2]; 2], vector: [f64; 2]) -> [f64; 2] {
    [
        matrix[0][0] * vector[0] + matrix[0][1] * vector[1],
        matrix[1][0] * vector[0] + matrix[1][1] * vector[1],
    ]
}

fn vector_multiply(vector: [f64; 2], matrix: [[f64; 2]; 2]) -> [f64; 2] {
    [
        vector[0] * matrix[0][0] + vector[1] * matrix[1][0],
        vector[0] * matrix[0][1] + vector[1] * matrix[1][1],
    ]
}

#[derive(Clone, Copy, Debug)]
struct Summary {
    candidate: CandidateId,
    rows: u32,
    mapping_failures: u32,
    impulse_failures: u32,
    row_rejections: u32,
    worst_mapping_error: f64,
    worst_impulse_error: f64,
    first_rejection: Option<(usize, &'static str)>,
    hash: u64,
}

impl Summary {
    const fn new(candidate: CandidateId) -> Self {
        Self {
            candidate,
            rows: 0,
            mapping_failures: 0,
            impulse_failures: 0,
            row_rejections: 0,
            worst_mapping_error: 0.0,
            worst_impulse_error: 0.0,
            first_rejection: None,
            hash: FNV_OFFSET,
        }
    }

    fn mix(&mut self, word: u64) {
        self.hash ^= word;
        self.hash = self.hash.wrapping_mul(FNV_PRIME);
    }

    fn survives(self) -> bool {
        self.mapping_failures == 0 && self.impulse_failures == 0 && self.row_rejections == 0
    }
}

fn phase_one(candidate_id: CandidateId, rows: &[Row]) -> Summary {
    let mut summary = Summary::new(candidate_id);
    summary.mix(EQUATION_VERSION);
    summary.mix(candidate_id as u64);
    for (row_index, row) in rows.iter().copied().enumerate() {
        summary.rows += 1;
        for word in [
            u64::from(row.rate),
            row.kind as u64,
            row.frequency.to_bits(),
            row.gain.to_bits(),
            row.q.to_bits(),
            row.slope.to_bits(),
        ] {
            summary.mix(word);
        }
        let reference = row.reference();
        let mut candidate = match Candidate::design(candidate_id, row, reference) {
            Ok(value) => value,
            Err(reason) => {
                summary.row_rejections += 1;
                summary.first_rejection.get_or_insert((row_index, reason));
                continue;
            }
        };
        let (b0, b1, b2, a1, a2) = reference.values();
        for (actual, expected) in candidate
            .direct_form()
            .into_iter()
            .zip([b0, b1, b2, a1, a2])
        {
            let error = (actual - expected).abs();
            summary.worst_mapping_error = summary.worst_mapping_error.max(error);
            summary.mix(actual.to_bits());
            if !error.is_finite() || error > F64_TOLERANCE {
                summary.mapping_failures += 1;
            }
        }
        let mut oracle = ReferenceParametricEqSection::new(reference);
        for sample in 0..IMPULSE_SAMPLES {
            let input = if sample == 0 { 1.0 } else { 0.0 };
            let error = (candidate.process(input) - oracle.process(input)).abs();
            summary.worst_impulse_error = summary.worst_impulse_error.max(error);
            summary.mix(error.to_bits());
            if !error.is_finite() || error > F64_TOLERANCE {
                summary.impulse_failures += 1;
            }
        }
    }
    summary
}

#[test]
#[ignore = "Issue-045's one permitted execution must include the later retained-f32 phases"]
fn issue_045_phase_one_f64_reconstruction_and_impulse_gate() {
    let rows = rows();
    let summaries = CandidateId::ALL.map(|candidate| phase_one(candidate, &rows));
    for summary in summaries {
        eprintln!(
            "issue-045 phase=1 candidate={} rows={} map_failures={} impulse_failures={} row_rejections={} worst_map={:.17e} worst_impulse={:.17e} hash={:016x} first_rejection={:?} survives={}",
            summary.candidate.name(),
            summary.rows,
            summary.mapping_failures,
            summary.impulse_failures,
            summary.row_rejections,
            summary.worst_mapping_error,
            summary.worst_impulse_error,
            summary.hash,
            summary.first_rejection,
            summary.survives(),
        );
        assert_eq!(summary.rows, 1_488);
    }
}
