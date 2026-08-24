//! Issue-045 four-phase recurrence derivation and retained-f32 comparison.
//!
//! This test-boundary module executes only through one ignored full-matrix test. It persists each
//! completed phase to a create-new transcript; phases cannot be run separately and recombined.

use core::cmp::Ordering;
use core::f64::consts::{FRAC_1_SQRT_2, PI};
use core::mem::size_of;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

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
const EQUATION_VERSION: u64 = 0x4930_3435_534f_4c32; // I045_SOL2
const TRANSCRIPT_ENV: &str = "MISO_ENGINE_TRANSCRIPT_045";

struct Transcript {
    path: PathBuf,
    file: File,
    expected: String,
}

impl Transcript {
    fn create() -> Self {
        let path = std::env::var_os(TRANSCRIPT_ENV)
            .map(PathBuf::from)
            .expect("the final Issue-045 run requires MISO_ENGINE_TRANSCRIPT_045");
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .expect("the Issue-045 transcript path must not already exist");
        Self {
            path,
            file,
            expected: String::new(),
        }
    }

    fn record(&mut self, line: String) {
        eprintln!("{line}");
        writeln!(self.file, "{line}").expect("write Issue-045 transcript record");
        self.file
            .sync_all()
            .expect("persist Issue-045 transcript record");
        self.expected.push_str(&line);
        self.expected.push('\n');
    }

    fn finish(mut self) {
        self.record("issue-045 complete=true".to_owned());
        drop(self.file);
        let actual = fs::read_to_string(&self.path).expect("read persisted Issue-045 transcript");
        assert_eq!(actual, self.expected, "persisted transcript changed");
    }
}

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

    const fn coefficient_words(self) -> usize {
        match self {
            Self::L1 => L1_COEFFICIENT_WORDS,
            Self::D2 => D2_COEFFICIENT_WORDS,
            Self::B3 => B3_COEFFICIENT_WORDS,
        }
    }

    const fn state_words(self) -> usize {
        match self {
            Self::L1 => L1_STATE_WORDS,
            Self::D2 => D2_STATE_WORDS,
            Self::B3 => B3_STATE_WORDS,
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
#[ignore = "Issue-045's one permitted execution is the complete four-phase matrix"]
fn issue_045_complete_recurrence_comparison_requires_sol_freeze() {
    complete_comparison();
}

#[repr(C)]
struct LaneWords<const WIDTH: usize, const WORDS: usize> {
    /// Field-major, direct `field[lane]` storage: no lane communication or hidden scalar state.
    fields: [[f32; WIDTH]; WORDS],
}

const L1_COEFFICIENT_WORDS: usize = 6;
const L1_STATE_WORDS: usize = 4;
const D2_COEFFICIENT_WORDS: usize = 7;
const D2_STATE_WORDS: usize = 8;
const B3_COEFFICIENT_WORDS: usize = 9;
const B3_STATE_WORDS: usize = 2;

#[derive(Clone, Copy, Default)]
struct StepEvents {
    canonicalizations: u64,
    recoveries: u64,
    first_canonicalized_bits: Option<u32>,
    first_recovery_bits: Option<u32>,
}

impl StepEvents {
    fn canonicalized(&mut self, value: f32) {
        self.canonicalizations += 1;
        self.first_canonicalized_bits.get_or_insert(value.to_bits());
    }

    fn recovered(&mut self, value: f32) {
        self.recoveries += 1;
        self.first_recovery_bits.get_or_insert(value.to_bits());
    }
}

/// Every committed retained value is a normal `f32` or canonical positive zero.
fn canonical_boundary(value: f32, events: &mut StepEvents) -> Result<f32, ()> {
    if !value.is_finite() {
        events.recovered(value);
        return Err(());
    }
    if value.to_bits() == 0 {
        return Ok(value);
    }
    if value == 0.0 || !value.is_normal() {
        events.canonicalized(value);
        return Ok(0.0);
    }
    Ok(value)
}

fn normal_or_positive_zero(value: f32) -> bool {
    value.is_normal() || value.to_bits() == 0
}

#[derive(Clone, Copy)]
struct RetainedL1 {
    a: f32,
    n0: f32,
    n1: f32,
    n2: f32,
    k1: f32,
    k2: f32,
    x1: f32,
    x2: f32,
    s0: f32,
    s1: f32,
}

#[derive(Clone, Copy)]
struct RetainedD2 {
    a: f32,
    n0: f32,
    d0: f32,
    n1: f32,
    d1: f32,
    n2: f32,
    d2: f32,
    x1: Expansion,
    x2: Expansion,
    y1: Expansion,
    y2: Expansion,
}

#[derive(Clone, Copy)]
struct RetainedB3 {
    a00: f32,
    a01: f32,
    a10: f32,
    a11: f32,
    b0: f32,
    b1: f32,
    c0: f32,
    c1: f32,
    d: f32,
    s0: f32,
    s1: f32,
}

#[derive(Clone, Copy)]
enum RetainedCandidate {
    Identity,
    L1(RetainedL1),
    D2(RetainedD2),
    B3(RetainedB3),
}

impl RetainedCandidate {
    fn design(
        id: CandidateId,
        row: Row,
        reference: ReferenceParametricEqCoefficients,
    ) -> Result<Self, &'static str> {
        let source = Candidate::design(id, row, reference)?;
        match source {
            Candidate::Identity => Ok(Self::Identity),
            Candidate::L1(value) => {
                let words = [value.a, value.n0, value.n1, value.n2, value.k1, value.k2];
                if !words.into_iter().all(|word| (word as f32).is_finite()) {
                    return Err("l1_nonfinite_retained_word");
                }
                Ok(Self::L1(RetainedL1 {
                    a: value.a as f32,
                    n0: value.n0 as f32,
                    n1: value.n1 as f32,
                    n2: value.n2 as f32,
                    k1: value.k1 as f32,
                    k2: value.k2 as f32,
                    x1: 0.0,
                    x2: 0.0,
                    s0: 0.0,
                    s1: 0.0,
                }))
            }
            Candidate::D2(value) => {
                let words = [
                    value.a, value.n0, value.d0, value.n1, value.d1, value.n2, value.d2,
                ];
                if !words.into_iter().all(|word| (word as f32).is_finite()) {
                    return Err("d2_nonfinite_retained_word");
                }
                Ok(Self::D2(RetainedD2 {
                    a: value.a as f32,
                    n0: value.n0 as f32,
                    d0: value.d0 as f32,
                    n1: value.n1 as f32,
                    d1: value.d1 as f32,
                    n2: value.n2 as f32,
                    d2: value.d2 as f32,
                    x1: Expansion::zero(),
                    x2: Expansion::zero(),
                    y1: Expansion::zero(),
                    y2: Expansion::zero(),
                }))
            }
            Candidate::B3(value) => {
                let words = [
                    value.a00, value.a01, value.a10, value.a11, value.b0, value.b1, value.c0,
                    value.c1, value.d,
                ];
                if !words.into_iter().all(|word| (word as f32).is_finite()) {
                    return Err("b3_nonfinite_retained_word");
                }
                Ok(Self::B3(RetainedB3 {
                    a00: value.a00 as f32,
                    a01: value.a01 as f32,
                    a10: value.a10 as f32,
                    a11: value.a11 as f32,
                    b0: value.b0 as f32,
                    b1: value.b1 as f32,
                    c0: value.c0 as f32,
                    c1: value.c1 as f32,
                    d: value.d as f32,
                    s0: 0.0,
                    s1: 0.0,
                }))
            }
        }
    }

    fn words(self) -> ([u32; 9], usize) {
        match self {
            Self::Identity => ([1.0_f32.to_bits(), 0, 0, 0, 0, 0, 0, 0, 0], 0),
            Self::L1(value) => (
                [
                    value.a.to_bits(),
                    value.n0.to_bits(),
                    value.n1.to_bits(),
                    value.n2.to_bits(),
                    value.k1.to_bits(),
                    value.k2.to_bits(),
                    0,
                    0,
                    0,
                ],
                L1_COEFFICIENT_WORDS,
            ),
            Self::D2(value) => (
                [
                    value.a.to_bits(),
                    value.n0.to_bits(),
                    value.d0.to_bits(),
                    value.n1.to_bits(),
                    value.d1.to_bits(),
                    value.n2.to_bits(),
                    value.d2.to_bits(),
                    0,
                    0,
                ],
                D2_COEFFICIENT_WORDS,
            ),
            Self::B3(value) => (
                [
                    value.a00.to_bits(),
                    value.a01.to_bits(),
                    value.a10.to_bits(),
                    value.a11.to_bits(),
                    value.b0.to_bits(),
                    value.b1.to_bits(),
                    value.c0.to_bits(),
                    value.c1.to_bits(),
                    value.d.to_bits(),
                ],
                B3_COEFFICIENT_WORDS,
            ),
        }
    }

    fn direct_form(self) -> [f64; 5] {
        match self {
            Self::Identity => [1.0, 0.0, 0.0, 0.0, 0.0],
            Self::L1(value) => {
                let a = f64::from(value.a);
                let b2 = f64::from(value.n2);
                let b1 = f64::from(value.n1) - 2.0 * a * b2;
                [
                    f64::from(value.n0) - a * b1 - b2,
                    b1,
                    b2,
                    f64::from(value.k1) * (1.0 + f64::from(value.k2)),
                    f64::from(value.k2),
                ]
            }
            Self::D2(value) => {
                let a = f64::from(value.a);
                let d2 = f64::from(value.d2);
                let scale_word = (value.d0 - value.a * value.d1) + value.d2;
                let q1 = value.a * value.d2;
                let q2 = (value.d1 - q1) - q1;
                let scale = f64::from(scale_word);
                [
                    (f64::from(value.n0) - a * f64::from(value.n1) + f64::from(value.n2)) / scale,
                    (f64::from(value.n1) - 2.0 * a * f64::from(value.n2)) / scale,
                    f64::from(value.n2) / scale,
                    f64::from(q2) / scale,
                    d2 / scale,
                ]
            }
            Self::B3(value) => RetainedB3::direct_form(value),
        }
    }

    fn step(&mut self, input: f32) -> (f32, StepEvents) {
        match self {
            Self::Identity => (input, StepEvents::default()),
            Self::L1(value) => value.step(input),
            Self::D2(value) => value.step(input),
            Self::B3(value) => value.step(input),
        }
    }

    fn state(self) -> ([f32; D2_STATE_WORDS], usize) {
        match self {
            Self::Identity => ([0.0; D2_STATE_WORDS], 0),
            Self::L1(value) => (
                [value.x1, value.x2, value.s0, value.s1, 0.0, 0.0, 0.0, 0.0],
                L1_STATE_WORDS,
            ),
            Self::D2(value) => (
                [
                    value.x1.hi,
                    value.x1.lo,
                    value.x2.hi,
                    value.x2.lo,
                    value.y1.hi,
                    value.y1.lo,
                    value.y2.hi,
                    value.y2.lo,
                ],
                D2_STATE_WORDS,
            ),
            Self::B3(value) => (
                [value.s0, value.s1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                B3_STATE_WORDS,
            ),
        }
    }

    fn strict_stability_margin(self) -> Option<f64> {
        let [_, _, _, a1, a2] = self.direct_form();
        let margin = (1.0 - a2.abs()).min(1.0 + a1 + a2).min(1.0 - a1 + a2);
        margin.is_finite().then_some(margin)
    }
}

impl RetainedL1 {
    fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.s0 = 0.0;
        self.s1 = 0.0;
    }

    fn step(&mut self, x: f32) -> (f32, StepEvents) {
        let mut events = StepEvents::default();
        let dx = self.x1 - self.a * x;
        let ddx = (self.x2 - self.a * self.x1) - self.a * dx;
        let e = (self.n0 * x + self.n1 * dx) + self.n2 * ddx;
        let f1 = e - self.k2 * self.s1;
        let y = f1 - self.k1 * self.s0;
        let values = [x, self.x1, y, self.k1 * y + self.s0];
        let mut next = [0.0; L1_STATE_WORDS];
        for (destination, value) in next.iter_mut().zip(values) {
            let Ok(value) = canonical_boundary(value, &mut events) else {
                self.reset();
                return (0.0, events);
            };
            *destination = value;
        }
        self.x1 = next[0];
        self.x2 = next[1];
        self.s0 = next[2];
        self.s1 = next[3];
        (next[2], events)
    }
}

impl RetainedB3 {
    fn direct_form(self) -> [f64; 5] {
        let a1 = -(f64::from(self.a00) + f64::from(self.a11));
        let a2 =
            f64::from(self.a00) * f64::from(self.a11) - f64::from(self.a01) * f64::from(self.a10);
        [
            f64::from(self.d),
            f64::from(self.d) * a1
                + f64::from(self.c0) * f64::from(self.b0)
                + f64::from(self.c1) * f64::from(self.b1),
            f64::from(self.d) * a2
                + f64::from(self.c0)
                    * (-f64::from(self.a11) * f64::from(self.b0)
                        + f64::from(self.a01) * f64::from(self.b1))
                + f64::from(self.c1)
                    * (f64::from(self.a10) * f64::from(self.b0)
                        - f64::from(self.a00) * f64::from(self.b1)),
            a1,
            a2,
        ]
    }

    fn reset(&mut self) {
        self.s0 = 0.0;
        self.s1 = 0.0;
    }

    fn step(&mut self, x: f32) -> (f32, StepEvents) {
        let mut events = StepEvents::default();
        let y = (self.d * x + self.c0 * self.s0) + self.c1 * self.s1;
        let s0 = (self.a00 * self.s0 + self.a01 * self.s1) + self.b0 * x;
        let s1 = (self.a10 * self.s0 + self.a11 * self.s1) + self.b1 * x;
        let Ok(y) = canonical_boundary(y, &mut events) else {
            self.reset();
            return (0.0, events);
        };
        let Ok(s0) = canonical_boundary(s0, &mut events) else {
            self.reset();
            return (0.0, events);
        };
        let Ok(s1) = canonical_boundary(s1, &mut events) else {
            self.reset();
            return (0.0, events);
        };
        self.s0 = s0;
        self.s1 = s1;
        (y, events)
    }
}

/// Canonical two-word `f32` expansion. Every primitive is separate-multiply/add only.
#[derive(Clone, Copy)]
struct Expansion {
    hi: f32,
    lo: f32,
}

impl Expansion {
    const fn zero() -> Self {
        Self { hi: 0.0, lo: 0.0 }
    }

    const fn word(value: f32) -> Self {
        Self { hi: value, lo: 0.0 }
    }

    fn add(self, other: Self) -> Result<Self, f32> {
        let (high_sum, high_error) = two_sum(self.hi, other.hi)?;
        let (low_sum, low_error) = two_sum(self.lo, other.lo)?;
        let (middle, middle_error) = two_sum(high_error, low_sum)?;
        let (high, high_tail) = two_sum(high_sum, middle)?;
        let (low, low_tail) = two_sum(low_error, middle_error)?;
        let (tail, tail_error) = two_sum(high_tail, low)?;
        Self::renormalize(high, (tail + tail_error) + low_tail)
    }

    fn sub(self, other: Self) -> Result<Self, f32> {
        self.add(Self {
            hi: -other.hi,
            lo: -other.lo,
        })
    }

    fn times_word(self, word: f32) -> Result<Self, f32> {
        let (product, error) = two_prod(self.hi, word)?;
        Self::renormalize(product, error + self.lo * word)
    }

    fn divided_by_word(self, word: f32) -> Result<Self, f32> {
        if !word.is_finite() || word == 0.0 {
            return Err(word);
        }
        let q1 = self.hi / word;
        let (product_hi, product_lo) = two_prod(q1, word)?;
        let q2 = (((self.hi - product_hi) - product_lo) + self.lo) / word;
        Self::renormalize(q1, q2)
    }

    fn renormalize(hi: f32, lo: f32) -> Result<Self, f32> {
        let (sum, error) = two_sum(hi, lo)?;
        let (hi, lo) = quick_two_sum(sum, error)?;
        if !hi.is_finite() || !lo.is_finite() || (hi == 0.0 && lo != 0.0) {
            return Err(if !hi.is_finite() { hi } else { lo });
        }
        if hi != 0.0 && lo.abs() > ulp(hi) * 0.5 {
            return Err(lo);
        }
        Ok(Self { hi, lo })
    }

    fn canonicalize(&mut self, events: &mut StepEvents) -> Result<(), ()> {
        self.hi = canonical_boundary(self.hi, events)?;
        self.lo = canonical_boundary(self.lo, events)?;
        if self.hi == 0.0 {
            self.lo = 0.0;
        }
        if self.hi != 0.0 && self.lo.abs() > ulp(self.hi) * 0.5 {
            events.recovered(self.lo);
            return Err(());
        }
        Ok(())
    }
}

fn two_sum(left: f32, right: f32) -> Result<(f32, f32), f32> {
    let sum = left + right;
    let virtual_right = sum - left;
    let error = (left - (sum - virtual_right)) + (right - virtual_right);
    (sum.is_finite() && error.is_finite())
        .then_some((sum, error))
        .ok_or(if !sum.is_finite() { sum } else { error })
}

fn quick_two_sum(left: f32, right: f32) -> Result<(f32, f32), f32> {
    let sum = left + right;
    let error = right - (sum - left);
    (sum.is_finite() && error.is_finite())
        .then_some((sum, error))
        .ok_or(if !sum.is_finite() { sum } else { error })
}

fn two_prod(left: f32, right: f32) -> Result<(f32, f32), f32> {
    const SPLIT: f32 = 4_097.0;
    const MAX_SPLIT_OPERAND: f32 = f32::MAX / SPLIT;
    if !left.is_finite()
        || !right.is_finite()
        || left.abs() > MAX_SPLIT_OPERAND
        || right.abs() > MAX_SPLIT_OPERAND
    {
        return Err(if !left.is_finite() || left.abs() > MAX_SPLIT_OPERAND {
            left
        } else {
            right
        });
    }
    let product = left * right;
    let left_split = SPLIT * left;
    let left_high = left_split - (left_split - left);
    let left_low = left - left_high;
    let right_split = SPLIT * right;
    let right_high = right_split - (right_split - right);
    let right_low = right - right_high;
    let error =
        ((left_high * right_high - product) + left_high * right_low + left_low * right_high)
            + left_low * right_low;
    (product.is_finite() && error.is_finite())
        .then_some((product, error))
        .ok_or(if !product.is_finite() { product } else { error })
}

fn ulp(value: f32) -> f32 {
    if value == 0.0 {
        f32::from_bits(1)
    } else {
        let bits = value.abs().to_bits();
        (f32::from_bits(bits.saturating_add(1)) - f32::from_bits(bits)).abs()
    }
}

impl RetainedD2 {
    fn reset(&mut self) {
        self.x1 = Expansion::zero();
        self.x2 = Expansion::zero();
        self.y1 = Expansion::zero();
        self.y2 = Expansion::zero();
    }

    fn step(&mut self, input: f32) -> (f32, StepEvents) {
        let mut events = StepEvents::default();
        let input = Expansion::word(input);
        let result = (|| -> Result<Expansion, f32> {
            let dx = self.x1.sub(input.times_word(self.a)?)?;
            let ddx = self
                .x2
                .sub(self.x1.times_word(self.a)?)?
                .sub(dx.times_word(self.a)?)?;
            let numerator = input
                .times_word(self.n0)?
                .add(dx.times_word(self.n1)?)?
                .add(ddx.times_word(self.n2)?)?;
            // These coefficient-only derived words retain the exact Issue-042 noncontracting f32
            // graph. Audio/state arithmetic remains the frozen double-single graph.
            let scale = (self.d0 - self.a * self.d1) + self.d2;
            if !scale.is_finite() || scale == 0.0 {
                return Err(scale);
            }
            let q1 = self.a * self.d2;
            let q2 = (self.d1 - q1) - q1;
            let history = self.y1.times_word(q2)?.add(self.y2.times_word(self.d2)?)?;
            numerator.sub(history)?.divided_by_word(scale)
        })();
        let mut output = match result {
            Ok(value) => value,
            Err(value) => {
                events.recovered(value);
                self.reset();
                return (0.0, events);
            }
        };
        let mut x1 = input;
        let mut x2 = self.x1;
        let mut y1 = output;
        let mut y2 = self.y1;
        for value in [&mut x1, &mut x2, &mut y1, &mut y2] {
            if value.canonicalize(&mut events).is_err() {
                self.reset();
                return (0.0, events);
            }
        }
        output = y1;
        let output_word = match canonical_boundary(output.hi + output.lo, &mut events) {
            Ok(value) => value,
            Err(()) => {
                self.reset();
                return (0.0, events);
            }
        };
        self.x1 = x1;
        self.x2 = x2;
        self.y1 = y1;
        self.y2 = y2;
        (output_word, events)
    }
}

#[derive(Clone, Copy, Debug)]
struct RetainedSummary {
    candidate: CandidateId,
    analytic_rows: u32,
    analytic_failures: u32,
    searches: u32,
    search_failures: u32,
    impulse_cases: u32,
    impulse_failures: u32,
    million_cases: u32,
    recoveries: u64,
    canonicalizations: u64,
    invalid_values: u64,
    minimum_stability_margin: f64,
    worst_analytic_db: f64,
    worst_dft_db: f64,
    max_output: f64,
    max_state: f64,
    minimum_nonzero: f64,
    first_failure: Option<(&'static str, usize, usize, u32)>,
    analytic_hash: u64,
    impulse_hash: u64,
    million_hash: u64,
}

impl RetainedSummary {
    const fn new(candidate: CandidateId) -> Self {
        Self {
            candidate,
            analytic_rows: 0,
            analytic_failures: 0,
            searches: 0,
            search_failures: 0,
            impulse_cases: 0,
            impulse_failures: 0,
            million_cases: 0,
            recoveries: 0,
            canonicalizations: 0,
            invalid_values: 0,
            minimum_stability_margin: f64::INFINITY,
            worst_analytic_db: 0.0,
            worst_dft_db: 0.0,
            max_output: 0.0,
            max_state: 0.0,
            minimum_nonzero: f64::INFINITY,
            first_failure: None,
            analytic_hash: FNV_OFFSET,
            impulse_hash: FNV_OFFSET,
            million_hash: FNV_OFFSET,
        }
    }

    fn fail(&mut self, category: &'static str, row: usize, sample: usize, bits: u32) {
        self.first_failure
            .get_or_insert((category, row, sample, bits));
    }

    fn observe(
        &mut self,
        candidate: RetainedCandidate,
        output: f32,
        events: StepEvents,
        row: usize,
        sample: usize,
        phase_hash: &mut u64,
    ) {
        mix_hash(phase_hash, u64::from(output.to_bits()));
        self.recoveries += events.recoveries;
        self.canonicalizations += events.canonicalizations;
        if events.recoveries != 0 {
            self.fail(
                "recovery",
                row,
                sample,
                events.first_recovery_bits.unwrap_or(output.to_bits()),
            );
        }
        self.observe_value(output, false, row, sample);
        let (state, count) = candidate.state();
        for value in state.into_iter().take(count) {
            mix_hash(phase_hash, u64::from(value.to_bits()));
            self.observe_value(value, true, row, sample);
        }
    }

    fn observe_value(&mut self, value: f32, state: bool, row: usize, sample: usize) {
        if !normal_or_positive_zero(value) {
            self.invalid_values += 1;
            self.fail("invalid_committed_value", row, sample, value.to_bits());
            return;
        }
        let magnitude = f64::from(value).abs();
        if state {
            self.max_state = self.max_state.max(magnitude);
        } else {
            self.max_output = self.max_output.max(magnitude);
        }
        if magnitude != 0.0 {
            self.minimum_nonzero = self.minimum_nonzero.min(magnitude);
        }
    }

    fn passes(self) -> bool {
        self.analytic_rows == 1_488
            && self.analytic_failures == 0
            && self.searches == 1_104
            && self.search_failures == 0
            && self.impulse_cases == 48
            && self.impulse_failures == 0
            && self.million_cases == 48
            && self.recoveries == 0
            && self.invalid_values == 0
    }
}

fn mix_hash(hash: &mut u64, word: u64) {
    *hash ^= word;
    *hash = hash.wrapping_mul(FNV_PRIME);
}

fn f32_direct_magnitude(coefficients: [f64; 5], frequency: f64, rate: f64) -> f64 {
    let [b0, b1, b2, a1, a2] = coefficients;
    let omega = 2.0 * PI * frequency / rate;
    let numerator_re = b0 + b1 * omega.cos() + b2 * (2.0 * omega).cos();
    let numerator_im = -b1 * omega.sin() - b2 * (2.0 * omega).sin();
    let denominator_re = 1.0 + a1 * omega.cos() + a2 * (2.0 * omega).cos();
    let denominator_im = -a1 * omega.sin() - a2 * (2.0 * omega).sin();
    numerator_re.hypot(numerator_im) / denominator_re.hypot(denominator_im)
}

fn db(value: f64) -> f64 {
    if value == 0.0 {
        f64::NEG_INFINITY
    } else {
        20.0 * value.log10()
    }
}

fn probes(row: Row) -> Vec<f64> {
    let mut output = Vec::with_capacity(2_051);
    for index in 0..2_048 {
        output.push(10.0 * 2_000.0_f64.powf(index as f64 / 2_047.0));
    }
    output.push(row.frequency);
    output.push(0.0);
    output.push(f64::from(row.rate) * 0.5);
    output
}

fn retained_analytic(candidate_id: CandidateId, all_rows: &[Row]) -> RetainedSummary {
    let mut summary = RetainedSummary::new(candidate_id);
    mix_hash(&mut summary.analytic_hash, EQUATION_VERSION);
    mix_hash(&mut summary.analytic_hash, candidate_id as u64);
    for (row_index, row) in all_rows.iter().copied().enumerate() {
        summary.analytic_rows += 1;
        let reference = row.reference();
        let candidate = match RetainedCandidate::design(candidate_id, row, reference) {
            Ok(value) => value,
            Err(_) => {
                summary.analytic_failures += 1;
                summary.fail("retained_design", row_index, 0, 0);
                continue;
            }
        };
        let (words, count) = candidate.words();
        for word in words.into_iter().take(count) {
            mix_hash(&mut summary.analytic_hash, u64::from(word));
        }
        let Some(margin) = candidate.strict_stability_margin() else {
            summary.analytic_failures += 1;
            summary.fail("retained_nonfinite_transition", row_index, 0, 0);
            continue;
        };
        summary.minimum_stability_margin = summary.minimum_stability_margin.min(margin);
        if margin.partial_cmp(&0.0) != Some(Ordering::Greater) {
            summary.analytic_failures += 1;
            summary.fail("retained_unstable_transition", row_index, 0, 0);
        }
        let coefficients = candidate.direct_form();
        if !coefficients.into_iter().all(f64::is_finite) {
            summary.analytic_failures += 1;
            summary.fail("retained_nonfinite_words", row_index, 0, 0);
            continue;
        }
        for probe in probes(row) {
            let expected = reference.magnitude_at_hz(probe).expect("frozen probe");
            let actual = f32_direct_magnitude(coefficients, probe, f64::from(row.rate));
            let expected_db = db(expected);
            let actual_db = db(actual);
            mix_hash(&mut summary.analytic_hash, actual.to_bits());
            if !actual.is_finite()
                || (expected_db >= -120.0 && (actual_db - expected_db).abs() > 0.005)
            {
                summary.analytic_failures += 1;
                summary.fail("analytic_response", row_index, 0, (actual as f32).to_bits());
            }
            if expected_db >= -120.0 {
                summary.worst_analytic_db = summary
                    .worst_analytic_db
                    .max((actual_db - expected_db).abs());
            }
        }
        run_characteristic_search(&mut summary, row_index, row, coefficients);
    }
    summary
}

fn run_characteristic_search(
    summary: &mut RetainedSummary,
    row_index: usize,
    row: Row,
    coefficients: [f64; 5],
) {
    let result = match row.kind {
        ReferenceParametricEqKind::LowPass | ReferenceParametricEqKind::HighPass
            if (row.q - FRAC_1_SQRT_2).abs() < f64::EPSILON =>
        {
            Some((
                find_crossing(coefficients, row.rate, -3.010_299_956_6),
                None,
            ))
        }
        ReferenceParametricEqKind::Bell if row.gain != 0.0 => {
            let found = find_log_extremum(coefficients, row.rate, row.gain > 0.0);
            Some((found, Some((row.gain, false))))
        }
        ReferenceParametricEqKind::LowShelf | ReferenceParametricEqKind::HighShelf
            if row.gain != 0.0 =>
        {
            Some((find_crossing(coefficients, row.rate, row.gain * 0.5), None))
        }
        ReferenceParametricEqKind::Notch => {
            let found = find_log_extremum(coefficients, row.rate, false);
            Some((found, Some((0.0, true))))
        }
        _ => None,
    };
    let Some((found, magnitude_gate)) = result else {
        return;
    };
    summary.searches += 1;
    let magnitude = f32_direct_magnitude(coefficients, found, row.rate as f64);
    mix_hash(&mut summary.analytic_hash, found.to_bits());
    mix_hash(&mut summary.analytic_hash, magnitude.to_bits());
    let relative = ((found - row.frequency) / row.frequency).abs();
    let magnitude_ok = magnitude_gate.is_none_or(|(target_db, notch)| {
        if notch {
            magnitude <= 1e-5
        } else {
            (db(magnitude) - target_db).abs() <= 0.005
        }
    });
    if !relative.is_finite() || relative > 0.001 || !magnitude_ok {
        summary.search_failures += 1;
        summary.fail(
            "characteristic_search",
            row_index,
            0,
            (found as f32).to_bits(),
        );
    }
}

fn find_crossing(coefficients: [f64; 5], rate: u32, target_db: f64) -> f64 {
    let mut low = 0.0;
    let mut high = f64::from(rate) * 0.5;
    let mut low_side = db(f32_direct_magnitude(coefficients, low, f64::from(rate))) >= target_db;
    let high_side = db(f32_direct_magnitude(coefficients, high, f64::from(rate))) >= target_db;
    if low_side == high_side {
        return f64::NAN;
    }
    for _ in 0..96 {
        let middle = (low + high) * 0.5;
        let middle_side =
            db(f32_direct_magnitude(coefficients, middle, f64::from(rate))) >= target_db;
        if middle_side == low_side {
            low = middle;
            low_side = middle_side;
        } else {
            high = middle;
        }
    }
    (low + high) * 0.5
}

fn find_log_extremum(coefficients: [f64; 5], rate: u32, maximum: bool) -> f64 {
    let mut low = f64::from(rate) * 1.0e-12;
    let mut high = f64::from(rate) * 0.5;
    for _ in 0..96 {
        let log_low = low.ln();
        let span = high.ln() - log_low;
        let first = (log_low + span / 3.0).exp();
        let second = (log_low + span * (2.0 / 3.0)).exp();
        let first_value = f32_direct_magnitude(coefficients, first, f64::from(rate));
        let second_value = f32_direct_magnitude(coefficients, second, f64::from(rate));
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

#[derive(Clone, Copy)]
enum Edge {
    Low,
    High,
}

impl Edge {
    const ALL: [Self; 2] = [Self::Low, Self::High];

    const fn values(self) -> (f64, f64, f64, f64) {
        match self {
            Self::Low => (10.0, -24.0, 0.1, 0.1),
            Self::High => (20_000.0, 24.0, 18.0, 1.0),
        }
    }
}

fn edge_rows() -> Vec<Row> {
    let mut output = Vec::with_capacity(48);
    for rate in RATES {
        for kind in [
            ReferenceParametricEqKind::Bell,
            ReferenceParametricEqKind::LowShelf,
            ReferenceParametricEqKind::HighShelf,
            ReferenceParametricEqKind::LowPass,
            ReferenceParametricEqKind::HighPass,
            ReferenceParametricEqKind::Notch,
        ] {
            for edge in Edge::ALL {
                let (frequency, gain, q, slope) = edge.values();
                output.push(Row {
                    rate,
                    kind,
                    frequency,
                    gain,
                    q,
                    slope,
                });
            }
        }
    }
    assert_eq!(output.len(), 48);
    output
}

fn impulse_dft_step(
    sample: f64,
    unit_re: &mut f64,
    unit_im: &mut f64,
    step_re: f64,
    step_im: f64,
    re: &mut f64,
    im: &mut f64,
) {
    *re += sample * *unit_re;
    *im += sample * *unit_im;
    (*unit_re, *unit_im) = (
        *unit_re * step_re - *unit_im * step_im,
        *unit_re * step_im + *unit_im * step_re,
    );
}

fn impulse_phase(summary: &mut RetainedSummary) {
    let mut impulse_hash = summary.impulse_hash;
    for (row_index, row) in edge_rows().into_iter().enumerate() {
        let reference = row.reference();
        let mut candidate = match RetainedCandidate::design(summary.candidate, row, reference) {
            Ok(value) => value,
            Err(_) => {
                summary.impulse_failures += 1;
                summary.fail("impulse_design", row_index, 0, 0);
                continue;
            }
        };
        let mut oracle = ReferenceParametricEqSection::new(reference);
        let phase = -core::f64::consts::TAU * row.frequency / f64::from(row.rate);
        let (step_re, step_im) = (phase.cos(), phase.sin());
        let (mut actual_re, mut actual_im) = (0.0, 0.0);
        let (mut expected_re, mut expected_im) = (0.0, 0.0);
        let (mut actual_unit_re, mut actual_unit_im) = (1.0, 0.0);
        let (mut expected_unit_re, mut expected_unit_im) = (1.0, 0.0);
        for sample in 0..row.rate as usize {
            let input = if sample == 0 { 1.0 } else { 0.0 };
            let (actual, events) = candidate.step(input as f32);
            summary.observe(
                candidate,
                actual,
                events,
                row_index,
                sample,
                &mut impulse_hash,
            );
            impulse_dft_step(
                f64::from(actual),
                &mut actual_unit_re,
                &mut actual_unit_im,
                step_re,
                step_im,
                &mut actual_re,
                &mut actual_im,
            );
            impulse_dft_step(
                oracle.process(input),
                &mut expected_unit_re,
                &mut expected_unit_im,
                step_re,
                step_im,
                &mut expected_re,
                &mut expected_im,
            );
        }
        let actual_db = db(actual_re.hypot(actual_im));
        let expected_db = db(expected_re.hypot(expected_im));
        mix_hash(&mut impulse_hash, actual_db.to_bits());
        mix_hash(&mut impulse_hash, expected_db.to_bits());
        if expected_db >= -120.0 {
            let error = (actual_db - expected_db).abs();
            summary.worst_dft_db = summary.worst_dft_db.max(error);
            if !error.is_finite() || error > 0.05 {
                summary.impulse_failures += 1;
                summary.fail(
                    "finite_window_dft",
                    row_index,
                    0,
                    (actual_db as f32).to_bits(),
                );
            }
        }
        summary.impulse_cases += 1;
    }
    summary.impulse_hash = impulse_hash;
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut value = *state;
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn deterministic_noise(state: &mut u64) -> f32 {
    let word = splitmix64(state);
    let sign = if word & 1 == 0 { -1.0 } else { 1.0 };
    let magnitude = 0.01 + ((word >> 40) as u32 as f32 / ((1_u32 << 24) - 1) as f32) * 0.98;
    sign * magnitude
}

fn million_phase(summary: &mut RetainedSummary) {
    let mut million_hash = summary.million_hash;
    for (row_index, row) in edge_rows().into_iter().enumerate() {
        let reference = row.reference();
        let mut candidate = match RetainedCandidate::design(summary.candidate, row, reference) {
            Ok(value) => value,
            Err(_) => {
                summary.fail("million_design", row_index, 0, 0);
                continue;
            }
        };
        let mut state = 0x0000_0000_0012_e911_u64
            ^ u64::from(row.rate)
            ^ (u64::from(row.kind as u8) << 32)
            ^ (edge_discriminator(row) << 40);
        for sample in 0..1_000_000 {
            let input = if sample == 0 {
                0.99
            } else {
                deterministic_noise(&mut state)
            };
            let (output, events) = candidate.step(input);
            summary.observe(
                candidate,
                output,
                events,
                row_index,
                sample,
                &mut million_hash,
            );
        }
        summary.million_cases += 1;
    }
    summary.million_hash = million_hash;
}

fn edge_discriminator(row: Row) -> u64 {
    if row.frequency == 10.0 { 0 } else { 1 }
}

fn configuration_hash(all_rows: &[Row]) -> u64 {
    let mut hash = FNV_OFFSET;
    for word in [
        EQUATION_VERSION,
        IMPULSE_SAMPLES as u64,
        F64_TOLERANCE.to_bits(),
        2_048,
        1_104,
        48,
        1_000_000,
        0x0000_0000_0012_e911,
        0.005_f64.to_bits(),
        0.05_f64.to_bits(),
        0.001_f64.to_bits(),
    ] {
        mix_hash(&mut hash, word);
    }
    for candidate in CandidateId::ALL {
        mix_hash(&mut hash, candidate as u64);
        mix_hash(&mut hash, candidate.coefficient_words() as u64);
        mix_hash(&mut hash, candidate.state_words() as u64);
    }
    for row in all_rows.iter().copied() {
        for word in [
            u64::from(row.rate),
            row.kind as u64,
            row.frequency.to_bits(),
            row.gain.to_bits(),
            row.q.to_bits(),
            row.slope.to_bits(),
        ] {
            mix_hash(&mut hash, word);
        }
        for probe in probes(row) {
            mix_hash(&mut hash, probe.to_bits());
        }
    }
    for row in edge_rows() {
        for word in [
            u64::from(row.rate),
            row.kind as u64,
            row.frequency.to_bits(),
            row.gain.to_bits(),
            row.q.to_bits(),
            row.slope.to_bits(),
        ] {
            mix_hash(&mut hash, word);
        }
    }
    hash
}

fn retained_record(stage: u8, summary: &RetainedSummary) -> String {
    format!(
        "issue-045 phase={stage} candidate={} analytic_rows={} analytic_failures={} searches={} search_failures={} impulse_cases={} impulse_failures={} million_cases={} recoveries={} canonicalizations={} invalid={} margin={:.17e} worst_analytic_db={:.17e} worst_dft_db={:.17e} max_output={:.17e} max_state={:.17e} min_nonzero={:.17e} analytic_hash={:016x} impulse_hash={:016x} million_hash={:016x} first_failure={:?} pass={}",
        summary.candidate.name(),
        summary.analytic_rows,
        summary.analytic_failures,
        summary.searches,
        summary.search_failures,
        summary.impulse_cases,
        summary.impulse_failures,
        summary.million_cases,
        summary.recoveries,
        summary.canonicalizations,
        summary.invalid_values,
        summary.minimum_stability_margin,
        summary.worst_analytic_db,
        summary.worst_dft_db,
        summary.max_output,
        summary.max_state,
        summary.minimum_nonzero,
        summary.analytic_hash,
        summary.impulse_hash,
        summary.million_hash,
        summary.first_failure,
        summary.passes(),
    )
}

fn select_candidate(summaries: &[RetainedSummary]) -> Option<CandidateId> {
    summaries
        .iter()
        .filter(|summary| summary.passes())
        .min_by(|left, right| {
            right
                .minimum_stability_margin
                .total_cmp(&left.minimum_stability_margin)
                .then_with(|| left.worst_dft_db.total_cmp(&right.worst_dft_db))
                .then_with(|| {
                    left.candidate
                        .state_words()
                        .cmp(&right.candidate.state_words())
                })
                .then_with(|| (left.candidate as u8).cmp(&(right.candidate as u8)))
        })
        .map(|summary| summary.candidate)
}

fn complete_comparison() {
    let all_rows = rows();
    let mut transcript = Transcript::create();
    transcript.record(format!(
        "issue-045 begin attempt=2 equation_version={EQUATION_VERSION:016x} rows={} configuration_hash={:016x}",
        all_rows.len(),
        configuration_hash(&all_rows),
    ));
    for candidate in CandidateId::ALL {
        transcript.record(format!(
            "issue-045 layout candidate={} coefficient_words={} state_words={} w4_bytes={} w8_bytes={}",
            candidate.name(),
            candidate.coefficient_words(),
            candidate.state_words(),
            (candidate.coefficient_words() + candidate.state_words()) * 4 * size_of::<f32>(),
            (candidate.coefficient_words() + candidate.state_words()) * 8 * size_of::<f32>(),
        ));
    }
    let phase_one_summaries = CandidateId::ALL.map(|candidate| phase_one(candidate, &all_rows));
    let mut retained = Vec::new();
    for summary in phase_one_summaries {
        transcript.record(format!(
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
        ));
        assert_eq!(summary.rows, 1_488);
        if summary.survives() {
            let retained_summary = retained_analytic(summary.candidate, &all_rows);
            transcript.record(retained_record(2, &retained_summary));
            retained.push(retained_summary);
        }
    }
    if retained.is_empty() {
        transcript.record(
            "issue-045 selection passing_candidates=0 selected=none reason=no_phase_one_survivor"
                .to_owned(),
        );
        transcript.finish();
        return;
    }
    for summary in &mut retained {
        impulse_phase(summary);
        transcript.record(retained_record(3, summary));
        million_phase(summary);
        transcript.record(retained_record(4, summary));
    }
    let passing = retained.iter().filter(|summary| summary.passes()).count();
    let selected = select_candidate(&retained);
    transcript.record(format!(
        "issue-045 selection passing_candidates={passing} selected={}",
        selected.map_or("none", CandidateId::name),
    ));
    transcript.finish();
}

#[test]
fn issue_045_retained_layout_and_dekker_primitives_are_static() {
    assert_eq!(
        size_of::<LaneWords<4, { L1_COEFFICIENT_WORDS + L1_STATE_WORDS }>>(),
        160
    );
    assert_eq!(
        size_of::<LaneWords<8, { L1_COEFFICIENT_WORDS + L1_STATE_WORDS }>>(),
        320
    );
    assert_eq!(
        size_of::<LaneWords<4, { D2_COEFFICIENT_WORDS + D2_STATE_WORDS }>>(),
        240
    );
    assert_eq!(
        size_of::<LaneWords<8, { D2_COEFFICIENT_WORDS + D2_STATE_WORDS }>>(),
        480
    );
    assert_eq!(
        size_of::<LaneWords<4, { B3_COEFFICIENT_WORDS + B3_STATE_WORDS }>>(),
        176
    );
    assert_eq!(
        size_of::<LaneWords<8, { B3_COEFFICIENT_WORDS + B3_STATE_WORDS }>>(),
        352
    );
    assert_eq!(
        two_sum(1.0, f32::EPSILON).expect("finite").0,
        1.0 + f32::EPSILON
    );
    assert_eq!(
        quick_two_sum(1.0, f32::EPSILON).expect("finite").0,
        1.0 + f32::EPSILON
    );
    let (product, error) = two_prod(4_097.0, 0.25).expect("finite split");
    assert_eq!(product + error, 1_024.25);
    assert!(two_prod(f32::MAX, 1.0).is_err());
}
