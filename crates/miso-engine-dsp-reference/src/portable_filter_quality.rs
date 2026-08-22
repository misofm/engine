//! Issue-031 retained-f64 builtin-filter comparison.
//!
//! This module is deliberately test-boundary-only. It contains the one frozen candidate and an
//! algebraically independent RBJ/direct-form oracle; it never imports a production builtin.

use core::f64::consts::{PI, SQRT_2, TAU};
use core::mem::size_of;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use crate::{ReferenceRetainedTptF32, ReferenceTptOutput, ReferenceTptRetainedAction};

const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const MAXIMUM_BITS: [u32; 4] = [0x46ac_42f7, 0x46bb_7ede, 0x472c_42f7, 0x473b_7ede];
const PARTITIONS: [usize; 5] = [1, 127, 128, 255, 1_024];
const SEQUENCE_SAMPLES: usize = 65_536;
const NOISE_SEED: u64 = 0x0000_0000_0000_0310;
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const EQUATION_VERSION: u64 = 0x4930_3331_5f46_3634; // I031_F64
const TRANSCRIPT_ENV: &str = "MISO_ISSUE_031_TRANSCRIPT";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FilterKind {
    LowPass,
    HighPass,
}

impl FilterKind {
    const ALL: [Self; 2] = [Self::LowPass, Self::HighPass];

    const fn tpt_output(self) -> ReferenceTptOutput {
        match self {
            Self::LowPass => ReferenceTptOutput::LowPass,
            Self::HighPass => ReferenceTptOutput::HighPass,
        }
    }

    const fn name(self) -> &'static str {
        match self {
            Self::LowPass => "low_pass",
            Self::HighPass => "high_pass",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Configuration {
    rate: u32,
    kind: FilterKind,
    cutoff: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ProbeRow {
    configuration: Configuration,
    frequency: f64,
}

fn configurations() -> Vec<Configuration> {
    let mut result = Vec::with_capacity(64);
    for (rate_index, rate) in RATES.into_iter().enumerate() {
        let maximum = f32::from_bits(MAXIMUM_BITS[rate_index]);
        let cutoffs = [
            10.0,
            20.0,
            100.0,
            1_000.0,
            20_000.0_f32.min(rate as f32 * 0.1),
            rate as f32 * 0.45,
            f32::from_bits(maximum.to_bits() - 1),
            maximum,
        ];
        for kind in FilterKind::ALL {
            for cutoff in cutoffs {
                result.push(Configuration { rate, kind, cutoff });
            }
        }
    }
    result
}

fn probes(configuration: Configuration) -> Vec<f64> {
    let rate = f64::from(configuration.rate);
    let cutoff = f64::from(configuration.cutoff);
    let mut result = [0.25 * cutoff, cutoff, 4.0 * cutoff, 0.2 * rate, 0.45 * rate]
        .into_iter()
        .map(|frequency| frequency.clamp(4.0, rate * 0.5 - 4.0))
        .map(|frequency| (frequency / 4.0).round() * 4.0)
        .collect::<Vec<_>>();
    result.sort_by(f64::total_cmp);
    result.dedup_by(|left, right| left.to_bits() == right.to_bits());
    result
}

fn probe_rows(configurations: &[Configuration]) -> Vec<ProbeRow> {
    configurations
        .iter()
        .copied()
        .flat_map(|configuration| {
            probes(configuration)
                .into_iter()
                .map(move |frequency| ProbeRow {
                    configuration,
                    frequency,
                })
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Report {
    sanitized_inputs: u64,
    sanitized_outputs: u64,
    state_canonicalizations: u64,
    invalid_recoveries: u64,
}

impl Report {
    fn mix_into(self, hash: &mut u64) {
        for word in [
            self.sanitized_inputs,
            self.sanitized_outputs,
            self.state_canonicalizations,
            self.invalid_recoveries,
        ] {
            mix_hash(hash, word);
        }
    }

    fn accumulate(&mut self, other: Self) {
        self.sanitized_inputs = self.sanitized_inputs.saturating_add(other.sanitized_inputs);
        self.sanitized_outputs = self
            .sanitized_outputs
            .saturating_add(other.sanitized_outputs);
        self.state_canonicalizations = self
            .state_canonicalizations
            .saturating_add(other.state_canonicalizations);
        self.invalid_recoveries = self
            .invalid_recoveries
            .saturating_add(other.invalid_recoveries);
    }
}

fn sanitize_input(input: f32, report: &mut Report) -> f32 {
    if input.is_finite() && !input.is_subnormal() {
        input
    } else {
        report.sanitized_inputs = report.sanitized_inputs.saturating_add(1);
        0.0
    }
}

#[derive(Clone, Copy, Debug)]
struct RetainedF64IncrementalV1 {
    c1: f64,
    a2: f64,
    a3: f64,
    k: f64,
    s1: f64,
    s2: f64,
    kind: FilterKind,
    report: Report,
}

impl RetainedF64IncrementalV1 {
    fn design(configuration: Configuration) -> Self {
        let rate = f64::from(configuration.rate);
        let cutoff = f64::from(configuration.cutoff);
        let g = (PI * cutoff / rate).tan();
        let k = SQRT_2;
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let c1 = 1.0 - a1;
        assert!([c1, a2, a3, k].into_iter().all(f64::is_finite));
        Self {
            c1,
            a2,
            a3,
            k,
            s1: 0.0,
            s2: 0.0,
            kind: configuration.kind,
            report: Report::default(),
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let input = f64::from(sanitize_input(input, &mut self.report));
        if !self.s1.is_finite() || !self.s2.is_finite() {
            self.s1 = 0.0;
            self.s2 = 0.0;
            self.report.invalid_recoveries = self.report.invalid_recoveries.saturating_add(1);
        } else {
            self.s1 = canonical_state(self.s1, &mut self.report);
            self.s2 = canonical_state(self.s2, &mut self.report);
        }

        let v3 = input - self.s2;
        let p1 = self.a2 * v3;
        let p2 = self.c1 * self.s1;
        let d1 = p1 - p2;
        let v1 = self.s1 + d1;
        let p3 = self.a2 * self.s1;
        let p4 = self.a3 * v3;
        let d2 = p3 + p4;
        let v2 = self.s2 + d2;
        let q1 = d1 + d1;
        let n1 = self.s1 + q1;
        let q2 = d2 + d2;
        let n2 = self.s2 + q2;
        let low = v2;
        let kh = self.k * v1;
        let th = input - kh;
        let high = th - v2;

        if !n1.is_finite() || !n2.is_finite() {
            self.s1 = 0.0;
            self.s2 = 0.0;
            self.report.invalid_recoveries = self.report.invalid_recoveries.saturating_add(1);
            return 0.0;
        }
        self.s1 = canonical_state(n1, &mut self.report);
        self.s2 = canonical_state(n2, &mut self.report);
        let output = match self.kind {
            FilterKind::LowPass => low,
            FilterKind::HighPass => high,
        } as f32;
        canonical_output(output, &mut self.report)
    }

    fn reset(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
    }

    fn state_bits(self) -> [u64; 2] {
        [self.s1.to_bits(), self.s2.to_bits()]
    }

    fn transfer(self) -> Transfer {
        Transfer::from_tpt(self.c1, self.a2, self.a3, self.k, self.kind)
    }
}

fn canonical_state(value: f64, report: &mut Report) -> f64 {
    if value.abs() < f64::from(f32::MIN_POSITIVE) {
        if value.to_bits() != 0 {
            report.state_canonicalizations = report.state_canonicalizations.saturating_add(1);
        }
        0.0
    } else {
        value
    }
}

fn canonical_output(value: f32, report: &mut Report) -> f32 {
    if !value.is_finite() || value.is_subnormal() {
        report.sanitized_outputs = report.sanitized_outputs.saturating_add(1);
        0.0
    } else if value.to_bits() == (-0.0_f32).to_bits() {
        0.0
    } else {
        value
    }
}

#[derive(Clone, Copy, Debug)]
struct Baseline {
    filter: ReferenceRetainedTptF32,
    report: Report,
}

impl Baseline {
    fn design(configuration: Configuration) -> Self {
        let filter = ReferenceRetainedTptF32::conditioned_butterworth(
            configuration.rate,
            configuration.cutoff,
            configuration.kind.tpt_output(),
        )
        .expect("the frozen Issue-036 configuration must prepare");
        Self {
            filter,
            report: Report::default(),
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let input = sanitize_input(input, &mut self.report);
        let step = self.filter.process(input);
        match step.action {
            ReferenceTptRetainedAction::FiniteNormal => {}
            ReferenceTptRetainedAction::SubnormalCanonicalization => {
                self.report.state_canonicalizations =
                    self.report.state_canonicalizations.saturating_add(1);
            }
            ReferenceTptRetainedAction::InvalidRecovery => {
                self.report.invalid_recoveries = self
                    .report
                    .invalid_recoveries
                    .saturating_add(step.recovery_delta);
            }
        }
        if step.output_sanitized {
            self.report.sanitized_outputs = self.report.sanitized_outputs.saturating_add(1);
        }
        f32::from_bits(step.output_bits)
    }

    fn state_bits(self) -> [u64; 2] {
        self.filter.state_bits().map(u64::from)
    }
}

/// Algebraically independent RBJ design and transposed-direct-form-II realization.
#[derive(Clone, Copy, Debug)]
struct DirectFormOracle {
    transfer: Transfer,
    z1: f64,
    z2: f64,
}

impl DirectFormOracle {
    fn design(configuration: Configuration) -> Self {
        let rate = f64::from(configuration.rate);
        let cutoff = f64::from(configuration.cutoff);
        let omega = TAU * cutoff / rate;
        let cosine = omega.cos();
        let alpha = omega.sin() / (2.0 * core::f64::consts::FRAC_1_SQRT_2);
        let (b0, b1, b2) = match configuration.kind {
            FilterKind::LowPass => {
                let b0 = (1.0 - cosine) * 0.5;
                (b0, 2.0 * b0, b0)
            }
            FilterKind::HighPass => {
                let b0 = (1.0 + cosine) * 0.5;
                (b0, -2.0 * b0, b0)
            }
        };
        let a0 = 1.0 + alpha;
        let transfer = Transfer {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: -2.0 * cosine / a0,
            a2: (1.0 - alpha) / a0,
        };
        assert!(transfer.words().into_iter().all(f64::is_finite));
        Self {
            transfer,
            z1: 0.0,
            z2: 0.0,
        }
    }

    fn process(&mut self, input: f32) -> f64 {
        let input = f64::from(input);
        let output = self.transfer.b0 * input + self.z1;
        self.z1 = self.transfer.b1 * input - self.transfer.a1 * output + self.z2;
        self.z2 = self.transfer.b2 * input - self.transfer.a2 * output;
        output
    }

    fn state_bits(self) -> [u64; 2] {
        [self.z1.to_bits(), self.z2.to_bits()]
    }
}

#[derive(Clone, Copy, Debug)]
struct Transfer {
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
}

impl Transfer {
    fn from_tpt(c1: f64, a2: f64, a3: f64, k: f64, kind: FilterKind) -> Self {
        let a00 = 1.0 - 2.0 * c1;
        let a01 = -2.0 * a2;
        let a10 = 2.0 * a2;
        let a11 = 1.0 - 2.0 * a3;
        let state_b0 = 2.0 * a2;
        let state_b1 = 2.0 * a3;
        let (c0, c1_output, d) = match kind {
            FilterKind::LowPass => (a2, 1.0 - a3, a3),
            FilterKind::HighPass => (-k * (1.0 - c1) - a2, k * a2 - (1.0 - a3), 1.0 - k * a2 - a3),
        };
        let trace = a00 + a11;
        let determinant = a00 * a11 - a01 * a10;
        let cb = c0 * state_b0 + c1_output * state_b1;
        let constant =
            c0 * (-a11 * state_b0 + a01 * state_b1) + c1_output * (a10 * state_b0 - a00 * state_b1);
        Self {
            b0: d,
            b1: cb - d * trace,
            b2: d * determinant + constant,
            a1: -trace,
            a2: determinant,
        }
    }

    const fn words(self) -> [f64; 5] {
        [self.b0, self.b1, self.b2, self.a1, self.a2]
    }

    fn magnitude_db(self, rate: u32, frequency: f64) -> f64 {
        let phase = TAU * frequency / f64::from(rate);
        let (cosine, sine) = (phase.cos(), phase.sin());
        let (cosine2, sine2) = ((2.0 * phase).cos(), (2.0 * phase).sin());
        let numerator = (self.b0 + self.b1 * cosine + self.b2 * cosine2)
            .hypot(-self.b1 * sine - self.b2 * sine2);
        let denominator =
            (1.0 + self.a1 * cosine + self.a2 * cosine2).hypot(self.a1 * sine + self.a2 * sine2);
        if numerator == 0.0 {
            f64::NEG_INFINITY
        } else {
            20.0 * (numerator / denominator).log10()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RealizationKind {
    Baseline,
    Candidate,
    Oracle,
}

#[derive(Debug)]
struct Rendered {
    samples: Vec<f64>,
    state: [u64; 2],
    report: Report,
    hash: u64,
}

fn render(
    kind: RealizationKind,
    configuration: Configuration,
    input: &[f32],
    partition: usize,
) -> Rendered {
    assert!(partition > 0);
    let mut samples = Vec::with_capacity(input.len());
    let (state, report) = match kind {
        RealizationKind::Baseline => {
            let mut filter = Baseline::design(configuration);
            for chunk in input.chunks(partition) {
                samples.extend(
                    chunk
                        .iter()
                        .map(|sample| f64::from(filter.process(*sample))),
                );
            }
            (filter.state_bits(), filter.report)
        }
        RealizationKind::Candidate => {
            let mut filter = RetainedF64IncrementalV1::design(configuration);
            for chunk in input.chunks(partition) {
                samples.extend(
                    chunk
                        .iter()
                        .map(|sample| f64::from(filter.process(*sample))),
                );
            }
            (filter.state_bits(), filter.report)
        }
        RealizationKind::Oracle => {
            let mut filter = DirectFormOracle::design(configuration);
            for chunk in input.chunks(partition) {
                samples.extend(chunk.iter().map(|sample| filter.process(*sample)));
            }
            (filter.state_bits(), Report::default())
        }
    };
    let mut hash = FNV_OFFSET;
    mix_hash(&mut hash, kind as u64);
    for sample in &samples {
        mix_hash(&mut hash, sample.to_bits());
    }
    for word in state {
        mix_hash(&mut hash, word);
    }
    report.mix_into(&mut hash);
    Rendered {
        samples,
        state,
        report,
        hash,
    }
}

fn assert_partition_identity(
    kind: RealizationKind,
    configuration: Configuration,
    input: &[f32],
) -> Rendered {
    let expected = render(kind, configuration, input, PARTITIONS[0]);
    for partition in PARTITIONS.into_iter().skip(1) {
        let actual = render(kind, configuration, input, partition);
        assert_eq!(actual.hash, expected.hash, "partition={partition}");
        assert_eq!(actual.state, expected.state, "partition={partition}");
        assert_eq!(actual.report, expected.report, "partition={partition}");
        assert_eq!(actual.samples, expected.samples, "partition={partition}");
    }
    expected
}

fn legal_render_is_valid(kind: RealizationKind, rendered: &Rendered) -> bool {
    let state_is_finite = match kind {
        RealizationKind::Baseline => rendered
            .state
            .into_iter()
            .all(|word| f32::from_bits(word as u32).is_finite()),
        RealizationKind::Candidate | RealizationKind::Oracle => rendered
            .state
            .into_iter()
            .all(|word| f64::from_bits(word).is_finite()),
    };
    rendered.samples.iter().all(|sample| sample.is_finite())
        && state_is_finite
        && rendered.report.sanitized_inputs == 0
        && rendered.report.sanitized_outputs == 0
        && rendered.report.invalid_recoveries == 0
}

fn impulse(rate: u32) -> Vec<f32> {
    let mut input = vec![0.0; rate as usize];
    input[0] = 1.0;
    input
}

fn coherent_sine(rate: u32, frequency: f64) -> Vec<f32> {
    let frames = (rate as usize / 2) + (rate as usize / 4);
    (0..frames)
        .map(|index| (0.5 * (TAU * frequency * index as f64 / f64::from(rate)).sin()) as f32)
        .collect()
}

fn fixed_sequence(kind: SequenceKind) -> Vec<f32> {
    match kind {
        SequenceKind::PositiveDc => vec![0.5; SEQUENCE_SAMPLES],
        SequenceKind::NegativeDc => vec![-0.5; SEQUENCE_SAMPLES],
        SequenceKind::Noise => {
            let mut state = NOISE_SEED;
            (0..SEQUENCE_SAMPLES)
                .map(|_| splitmix_bipolar(&mut state) * 0.5)
                .collect()
        }
    }
}

fn splitmix_bipolar(state: &mut u64) -> f32 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut word = *state;
    word = (word ^ (word >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    word = (word ^ (word >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    word ^= word >> 31;
    (((word >> 40) as f64 * (1.0 / 16_777_216.0)) * 2.0 - 1.0) as f32
}

fn dft_db(samples: &[f64], rate: u32, frequency: f64) -> f64 {
    let phase = -TAU * frequency / f64::from(rate);
    let (step_re, step_im) = (phase.cos(), phase.sin());
    let (mut unit_re, mut unit_im) = (1.0_f64, 0.0_f64);
    let (mut re, mut im) = (0.0_f64, 0.0_f64);
    for sample in samples {
        re += sample * unit_re;
        im += sample * unit_im;
        (unit_re, unit_im) = (
            unit_re * step_re - unit_im * step_im,
            unit_re * step_im + unit_im * step_re,
        );
    }
    let magnitude = re.hypot(im);
    if magnitude == 0.0 {
        f64::NEG_INFINITY
    } else {
        20.0 * magnitude.log10()
    }
}

fn rms(samples: &[f64]) -> f64 {
    (samples.iter().map(|value| value * value).sum::<f64>() / samples.len() as f64).sqrt()
}

fn residual_db(actual: &[f64], oracle: &[f64]) -> f64 {
    assert_eq!(actual.len(), oracle.len());
    let residual = actual
        .iter()
        .zip(oracle)
        .map(|(actual, oracle)| (actual - oracle).powi(2))
        .sum::<f64>();
    let reference = oracle.iter().map(|value| value * value).sum::<f64>();
    if residual == 0.0 {
        f64::NEG_INFINITY
    } else if reference == 0.0 {
        f64::INFINITY
    } else {
        10.0 * (residual / reference).log10()
    }
}

fn gain_db(output: &[f64], input: &[f32]) -> f64 {
    let input = input
        .iter()
        .map(|value| f64::from(*value))
        .collect::<Vec<_>>();
    20.0 * (rms(output) / rms(&input)).log10()
}

#[derive(Clone, Copy, Debug)]
enum SequenceKind {
    PositiveDc,
    NegativeDc,
    Noise,
}

impl SequenceKind {
    const ALL: [Self; 3] = [Self::PositiveDc, Self::NegativeDc, Self::Noise];
}

#[derive(Clone, Debug)]
struct Failure {
    phase: &'static str,
    rate: u32,
    kind: FilterKind,
    cutoff_bits: u32,
    frequency_bits: u64,
}

#[derive(Debug)]
struct Summary {
    analytic_rows: usize,
    impulse_configurations: usize,
    sustained_rows: usize,
    sequence_rows: usize,
    semantic_rows: usize,
    transfer_failures: usize,
    analytic_failures: usize,
    impulse_failures: usize,
    regression_failures: usize,
    semantic_failures: usize,
    semantic_recoveries: u64,
    limited_rows: usize,
    limited_rate_mask: u8,
    limited_kind_mask: u8,
    worst_transfer_error: f64,
    worst_analytic_error_db: f64,
    worst_impulse_error_db: f64,
    worst_transfer_row: Option<Failure>,
    worst_analytic_row: Option<Failure>,
    worst_impulse_row: Option<Failure>,
    baseline_worst_row: Option<Failure>,
    candidate_worst_row: Option<Failure>,
    baseline_worst_residual_db: f64,
    candidate_on_baseline_worst_db: f64,
    candidate_worst_residual_db: f64,
    global_improvement_db: f64,
    minimum_limited_improvement_db: f64,
    baseline_report: Report,
    candidate_report: Report,
    hash: u64,
    first_failure: Option<Failure>,
}

impl Summary {
    fn new() -> Self {
        Self {
            analytic_rows: 0,
            impulse_configurations: 0,
            sustained_rows: 0,
            sequence_rows: 0,
            semantic_rows: 0,
            transfer_failures: 0,
            analytic_failures: 0,
            impulse_failures: 0,
            regression_failures: 0,
            semantic_failures: 0,
            semantic_recoveries: 0,
            limited_rows: 0,
            limited_rate_mask: 0,
            limited_kind_mask: 0,
            worst_transfer_error: 0.0,
            worst_analytic_error_db: 0.0,
            worst_impulse_error_db: 0.0,
            worst_transfer_row: None,
            worst_analytic_row: None,
            worst_impulse_row: None,
            baseline_worst_row: None,
            candidate_worst_row: None,
            baseline_worst_residual_db: f64::NEG_INFINITY,
            candidate_on_baseline_worst_db: f64::NEG_INFINITY,
            candidate_worst_residual_db: f64::NEG_INFINITY,
            global_improvement_db: f64::NEG_INFINITY,
            minimum_limited_improvement_db: f64::INFINITY,
            baseline_report: Report::default(),
            candidate_report: Report::default(),
            hash: FNV_OFFSET,
            first_failure: None,
        }
    }

    fn fail(&mut self, phase: &'static str, row: ProbeRow) {
        self.first_failure.get_or_insert_with(|| point(phase, row));
    }

    fn observe_worst(
        current: &mut f64,
        current_row: &mut Option<Failure>,
        value: f64,
        phase: &'static str,
        row: ProbeRow,
    ) {
        if value > *current {
            *current = value;
            *current_row = Some(point(phase, row));
        }
    }

    fn observe_precision_row(
        &mut self,
        row: ProbeRow,
        baseline_residual: f64,
        candidate_residual: f64,
        baseline_gain_error: f64,
        candidate_gain_error: f64,
    ) {
        if baseline_residual > self.baseline_worst_residual_db {
            self.baseline_worst_residual_db = baseline_residual;
            self.candidate_on_baseline_worst_db = candidate_residual;
            self.baseline_worst_row = Some(point("baseline_residual", row));
        }
        if candidate_residual > self.candidate_worst_residual_db {
            self.candidate_worst_residual_db = candidate_residual;
            self.candidate_worst_row = Some(point("candidate_residual", row));
        }
        for word in [
            baseline_residual.to_bits(),
            candidate_residual.to_bits(),
            baseline_gain_error.to_bits(),
            candidate_gain_error.to_bits(),
        ] {
            mix_hash(&mut self.hash, word);
        }
        if candidate_residual > baseline_residual + 0.25
            || candidate_gain_error > baseline_gain_error + 0.0001
        {
            self.regression_failures += 1;
            self.fail("regression", row);
        }
        if baseline_residual > -120.0 {
            self.limited_rows += 1;
            self.limited_rate_mask |= 1 << rate_index(row.configuration.rate);
            self.limited_kind_mask |= 1 << row.configuration.kind as u8;
            let improvement = baseline_residual - candidate_residual;
            self.minimum_limited_improvement_db =
                self.minimum_limited_improvement_db.min(improvement);
            if improvement < 6.0 {
                self.regression_failures += 1;
                self.fail("limited_improvement", row);
            }
        }
    }

    fn finish_materiality(&mut self, representative: Configuration) {
        self.global_improvement_db =
            self.baseline_worst_residual_db - self.candidate_on_baseline_worst_db;
        let row = ProbeRow {
            configuration: representative,
            frequency: f64::from(representative.cutoff),
        };
        if self.limited_rows < 8
            || self.limited_rate_mask != 0b1111
            || self.limited_kind_mask != 0b11
        {
            self.regression_failures += 1;
            self.fail("materiality_coverage", row);
        }
        if self.global_improvement_db < 12.0 || self.candidate_worst_residual_db > -126.0 {
            self.regression_failures += 1;
            self.fail("global_improvement", row);
        }
    }

    fn passes(&self) -> bool {
        self.analytic_rows == 296
            && self.impulse_configurations == 64
            && self.sustained_rows == 296
            && self.sequence_rows == 192
            && self.semantic_rows == 8
            && self.transfer_failures == 0
            && self.analytic_failures == 0
            && self.impulse_failures == 0
            && self.regression_failures == 0
            && self.semantic_failures == 0
            && self.semantic_recoveries == 8
            && self.baseline_report.invalid_recoveries == 0
            && self.candidate_report.invalid_recoveries == 0
    }
}

fn point(phase: &'static str, row: ProbeRow) -> Failure {
    Failure {
        phase,
        rate: row.configuration.rate,
        kind: row.configuration.kind,
        cutoff_bits: row.configuration.cutoff.to_bits(),
        frequency_bits: row.frequency.to_bits(),
    }
}

fn rate_index(rate: u32) -> u8 {
    RATES
        .iter()
        .position(|candidate| *candidate == rate)
        .expect("rate belongs to the frozen grid") as u8
}

fn run_analytic(rows: &[ProbeRow], summary: &mut Summary) {
    let mut last_configuration = None;
    for row in rows.iter().copied() {
        let candidate = RetainedF64IncrementalV1::design(row.configuration);
        let candidate_transfer = candidate.transfer();
        let oracle_transfer = DirectFormOracle::design(row.configuration).transfer;
        if last_configuration != Some(row.configuration) {
            for (actual, expected) in candidate_transfer
                .words()
                .into_iter()
                .zip(oracle_transfer.words())
            {
                let error = (actual - expected).abs();
                Summary::observe_worst(
                    &mut summary.worst_transfer_error,
                    &mut summary.worst_transfer_row,
                    error,
                    "transfer",
                    row,
                );
                if !error.is_finite() || error > 1e-12 {
                    summary.transfer_failures += 1;
                    summary.fail("transfer", row);
                }
                mix_hash(&mut summary.hash, error.to_bits());
            }
            let cutoff = f64::from(row.configuration.cutoff);
            let expected = oracle_transfer.magnitude_db(row.configuration.rate, cutoff);
            let actual = candidate_transfer.magnitude_db(row.configuration.rate, cutoff);
            let error = (actual - expected).abs();
            let cutoff_row = ProbeRow {
                configuration: row.configuration,
                frequency: cutoff,
            };
            Summary::observe_worst(
                &mut summary.worst_analytic_error_db,
                &mut summary.worst_analytic_row,
                error,
                "cutoff",
                cutoff_row,
            );
            if !error.is_finite() || error > 1e-9 {
                summary.analytic_failures += 1;
                summary.fail("cutoff", cutoff_row);
            }
            mix_hash(&mut summary.hash, error.to_bits());
            last_configuration = Some(row.configuration);
        }
        let expected = oracle_transfer.magnitude_db(row.configuration.rate, row.frequency);
        let actual = candidate_transfer.magnitude_db(row.configuration.rate, row.frequency);
        if expected >= -120.0 {
            let error = (actual - expected).abs();
            Summary::observe_worst(
                &mut summary.worst_analytic_error_db,
                &mut summary.worst_analytic_row,
                error,
                "analytic",
                row,
            );
            if !error.is_finite() || error > 1e-9 {
                summary.analytic_failures += 1;
                summary.fail("analytic", row);
            }
            mix_hash(&mut summary.hash, error.to_bits());
        }
        summary.analytic_rows += 1;
    }
}

fn run_impulses(configurations: &[Configuration], summary: &mut Summary) {
    for configuration in configurations.iter().copied() {
        let input = impulse(configuration.rate);
        let baseline = assert_partition_identity(RealizationKind::Baseline, configuration, &input);
        let candidate =
            assert_partition_identity(RealizationKind::Candidate, configuration, &input);
        let oracle = assert_partition_identity(RealizationKind::Oracle, configuration, &input);
        let representative = ProbeRow {
            configuration,
            frequency: f64::from(configuration.cutoff),
        };
        for (kind, rendered) in [
            (RealizationKind::Baseline, &baseline),
            (RealizationKind::Candidate, &candidate),
            (RealizationKind::Oracle, &oracle),
        ] {
            if !legal_render_is_valid(kind, rendered) {
                summary.impulse_failures += 1;
                summary.fail("impulse_legal_render", representative);
            }
        }
        mix_input(&mut summary.hash, &input);
        for frequency in probes(configuration) {
            let expected = dft_db(&oracle.samples, configuration.rate, frequency);
            if expected >= -120.0 {
                let baseline_error =
                    (dft_db(&baseline.samples, configuration.rate, frequency) - expected).abs();
                let candidate_error =
                    (dft_db(&candidate.samples, configuration.rate, frequency) - expected).abs();
                let row = ProbeRow {
                    configuration,
                    frequency,
                };
                Summary::observe_worst(
                    &mut summary.worst_impulse_error_db,
                    &mut summary.worst_impulse_row,
                    candidate_error,
                    "candidate_impulse",
                    row,
                );
                if !candidate_error.is_finite() || candidate_error > 0.005 {
                    summary.impulse_failures += 1;
                    summary.fail("candidate_impulse", row);
                }
                mix_hash(&mut summary.hash, baseline_error.to_bits());
                mix_hash(&mut summary.hash, candidate_error.to_bits());
            }
        }
        for hash in [baseline.hash, candidate.hash, oracle.hash] {
            mix_hash(&mut summary.hash, hash);
        }
        summary.baseline_report.accumulate(baseline.report);
        summary.candidate_report.accumulate(candidate.report);
        summary.impulse_configurations += 1;
    }
}

fn observe_time_domain(
    row: ProbeRow,
    input: &[f32],
    measured_offset: usize,
    summary: &mut Summary,
) {
    let baseline = assert_partition_identity(RealizationKind::Baseline, row.configuration, input);
    let candidate = assert_partition_identity(RealizationKind::Candidate, row.configuration, input);
    let oracle = assert_partition_identity(RealizationKind::Oracle, row.configuration, input);
    for (kind, rendered) in [
        (RealizationKind::Baseline, &baseline),
        (RealizationKind::Candidate, &candidate),
        (RealizationKind::Oracle, &oracle),
    ] {
        if !legal_render_is_valid(kind, rendered) {
            summary.regression_failures += 1;
            summary.fail("legal_sequence", row);
        }
    }
    mix_input(&mut summary.hash, input);
    let baseline_samples = &baseline.samples[measured_offset..];
    let candidate_samples = &candidate.samples[measured_offset..];
    let oracle_samples = &oracle.samples[measured_offset..];
    let input_samples = &input[measured_offset..];
    let oracle_gain = gain_db(oracle_samples, input_samples);
    summary.observe_precision_row(
        row,
        residual_db(baseline_samples, oracle_samples),
        residual_db(candidate_samples, oracle_samples),
        (gain_db(baseline_samples, input_samples) - oracle_gain).abs(),
        (gain_db(candidate_samples, input_samples) - oracle_gain).abs(),
    );
    for hash in [baseline.hash, candidate.hash, oracle.hash] {
        mix_hash(&mut summary.hash, hash);
    }
    summary.baseline_report.accumulate(baseline.report);
    summary.candidate_report.accumulate(candidate.report);
}

fn run_sustained(rows: &[ProbeRow], summary: &mut Summary) {
    for row in rows.iter().copied() {
        let input = coherent_sine(row.configuration.rate, row.frequency);
        observe_time_domain(row, &input, row.configuration.rate as usize / 2, summary);
        summary.sustained_rows += 1;
    }
}

fn run_sequences(configurations: &[Configuration], summary: &mut Summary) {
    for configuration in configurations.iter().copied() {
        for sequence in SequenceKind::ALL {
            let input = fixed_sequence(sequence);
            observe_time_domain(
                ProbeRow {
                    configuration,
                    frequency: f64::from(configuration.cutoff),
                },
                &input,
                0,
                summary,
            );
            summary.sequence_rows += 1;
        }
    }
    summary.finish_materiality(configurations[0]);
}

fn run_semantics(summary: &mut Summary) {
    let inputs = [
        0.25,
        -0.0,
        f32::from_bits(1),
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        0.0,
        -0.25,
    ];
    let control = [0.25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.25];
    for rate in RATES {
        for kind in FilterKind::ALL {
            let configuration = Configuration {
                rate,
                kind,
                cutoff: 1_000.0,
            };
            let baseline =
                assert_partition_identity(RealizationKind::Baseline, configuration, &inputs);
            let candidate =
                assert_partition_identity(RealizationKind::Candidate, configuration, &inputs);
            let baseline_control =
                assert_partition_identity(RealizationKind::Baseline, configuration, &control);
            let candidate_control =
                assert_partition_identity(RealizationKind::Candidate, configuration, &control);
            if baseline.report.sanitized_inputs != 4
                || candidate.report.sanitized_inputs != 4
                || baseline.report.sanitized_outputs != 0
                || candidate.report.sanitized_outputs != 0
                || baseline.report.invalid_recoveries != 0
                || candidate.report.invalid_recoveries != 0
                || baseline.state != baseline_control.state
                || candidate.state != candidate_control.state
            {
                summary.semantic_failures += 1;
                summary.fail(
                    "semantics",
                    ProbeRow {
                        configuration,
                        frequency: 0.0,
                    },
                );
            }
            if baseline.samples != baseline_control.samples
                || candidate.samples != candidate_control.samples
            {
                summary.semantic_failures += 1;
                summary.fail(
                    "lane_isolation",
                    ProbeRow {
                        configuration,
                        frequency: 0.0,
                    },
                );
            }
            let mut recovered = RetainedF64IncrementalV1::design(configuration);
            recovered.s1 = f64::NAN;
            let recovered_output = recovered.process(0.25);
            let mut recovery_control = RetainedF64IncrementalV1::design(configuration);
            let recovery_control_output = recovery_control.process(0.25);
            let mut reset = RetainedF64IncrementalV1::design(configuration);
            let _ = reset.process(0.25);
            reset.reset();
            let reset_state = reset.state_bits();
            let reset_output = reset.process(-0.25);
            let mut reset_control = RetainedF64IncrementalV1::design(configuration);
            let reset_control_output = reset_control.process(-0.25);
            if recovered_output.to_bits() != recovery_control_output.to_bits()
                || recovered.state_bits() != recovery_control.state_bits()
                || recovered.report.invalid_recoveries != 1
                || recovered.report.sanitized_inputs != 0
                || recovered.report.sanitized_outputs != 0
                || reset_state != [0, 0]
                || reset_output.to_bits() != reset_control_output.to_bits()
                || reset.state_bits() != reset_control.state_bits()
            {
                summary.semantic_failures += 1;
                summary.fail(
                    "recovery_reset",
                    ProbeRow {
                        configuration,
                        frequency: 0.0,
                    },
                );
            }
            mix_input(&mut summary.hash, &inputs);
            mix_input(&mut summary.hash, &control);
            for hash in [
                baseline.hash,
                candidate.hash,
                baseline_control.hash,
                candidate_control.hash,
            ] {
                mix_hash(&mut summary.hash, hash);
            }
            for word in [
                u64::from(recovered_output.to_bits()),
                recovered.state_bits()[0],
                recovered.state_bits()[1],
                u64::from(reset_output.to_bits()),
                reset.state_bits()[0],
                reset.state_bits()[1],
            ] {
                mix_hash(&mut summary.hash, word);
            }
            recovered.report.mix_into(&mut summary.hash);
            reset.report.mix_into(&mut summary.hash);
            summary.semantic_recoveries = summary
                .semantic_recoveries
                .saturating_add(recovered.report.invalid_recoveries);
            summary.baseline_report.accumulate(baseline.report);
            summary.candidate_report.accumulate(candidate.report);
            summary.semantic_rows += 1;
        }
    }
}

fn grid_hash(configurations: &[Configuration], rows: &[ProbeRow]) -> u64 {
    let mut hash = FNV_OFFSET;
    for word in [
        EQUATION_VERSION,
        NOISE_SEED,
        SEQUENCE_SAMPLES as u64,
        size_of::<RetainedF64IncrementalV1>() as u64,
        size_of::<ReferenceRetainedTptF32>() as u64,
    ] {
        mix_hash(&mut hash, word);
    }
    for partition in PARTITIONS {
        mix_hash(&mut hash, partition as u64);
    }
    for configuration in configurations {
        mix_hash(&mut hash, u64::from(configuration.rate));
        mix_hash(&mut hash, configuration.kind as u64);
        mix_hash(&mut hash, u64::from(configuration.cutoff.to_bits()));
    }
    for row in rows {
        mix_hash(&mut hash, row.frequency.to_bits());
    }
    hash
}

fn mix_hash(hash: &mut u64, word: u64) {
    for byte in word.to_le_bytes() {
        *hash ^= u64::from(byte);
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}

fn mix_input(hash: &mut u64, input: &[f32]) {
    for sample in input {
        mix_hash(hash, u64::from(sample.to_bits()));
    }
}

struct Transcript {
    path: PathBuf,
    file: File,
    expected: String,
}

impl Transcript {
    fn create() -> Self {
        let path = std::env::var_os(TRANSCRIPT_ENV)
            .map(PathBuf::from)
            .expect("the Issue-031 matrix requires MISO_ISSUE_031_TRANSCRIPT");
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .expect("the Issue-031 transcript path must not exist");
        Self {
            path,
            file,
            expected: String::new(),
        }
    }

    fn record(&mut self, line: String) {
        eprintln!("{line}");
        writeln!(self.file, "{line}").expect("write Issue-031 transcript");
        self.file.sync_all().expect("persist Issue-031 transcript");
        self.expected.push_str(&line);
        self.expected.push('\n');
    }

    fn finish(mut self) {
        self.record("issue-031 complete=true".to_owned());
        drop(self.file);
        let actual = fs::read_to_string(&self.path).expect("read Issue-031 transcript");
        assert_eq!(actual, self.expected, "persisted transcript changed");
    }
}

fn summary_record(summary: &Summary) -> String {
    let first_failure = summary.first_failure.as_ref().map(|failure| {
        format!(
            "{}:{}:{}:{:08x}:{:016x}",
            failure.phase,
            failure.rate,
            failure.kind.name(),
            failure.cutoff_bits,
            failure.frequency_bits,
        )
    });
    format!(
        "issue-031 result analytic_rows={} impulse_configurations={} sustained_rows={} sequence_rows={} semantic_rows={} transfer_failures={} analytic_failures={} impulse_failures={} regression_failures={} semantic_failures={} semantic_recoveries={} limited_rows={} limited_rate_mask={:x} limited_kind_mask={:x} worst_transfer={:.17e} worst_transfer_row={:?} worst_analytic_db={:.17e} worst_analytic_row={:?} worst_impulse_db={:.17e} worst_impulse_row={:?} baseline_worst_residual_db={:.17e} baseline_worst_row={:?} candidate_on_baseline_worst_db={:.17e} candidate_worst_residual_db={:.17e} candidate_worst_row={:?} global_improvement_db={:.17e} minimum_limited_improvement_db={:.17e} baseline_sanitized_inputs={} baseline_sanitized_outputs={} baseline_state_canonicalizations={} baseline_invalid_recoveries={} candidate_sanitized_inputs={} candidate_sanitized_outputs={} candidate_state_canonicalizations={} candidate_invalid_recoveries={} hash={:016x} first_failure={first_failure:?} pass={}",
        summary.analytic_rows,
        summary.impulse_configurations,
        summary.sustained_rows,
        summary.sequence_rows,
        summary.semantic_rows,
        summary.transfer_failures,
        summary.analytic_failures,
        summary.impulse_failures,
        summary.regression_failures,
        summary.semantic_failures,
        summary.semantic_recoveries,
        summary.limited_rows,
        summary.limited_rate_mask,
        summary.limited_kind_mask,
        summary.worst_transfer_error,
        summary.worst_transfer_row,
        summary.worst_analytic_error_db,
        summary.worst_analytic_row,
        summary.worst_impulse_error_db,
        summary.worst_impulse_row,
        summary.baseline_worst_residual_db,
        summary.baseline_worst_row,
        summary.candidate_on_baseline_worst_db,
        summary.candidate_worst_residual_db,
        summary.candidate_worst_row,
        summary.global_improvement_db,
        summary.minimum_limited_improvement_db,
        summary.baseline_report.sanitized_inputs,
        summary.baseline_report.sanitized_outputs,
        summary.baseline_report.state_canonicalizations,
        summary.baseline_report.invalid_recoveries,
        summary.candidate_report.sanitized_inputs,
        summary.candidate_report.sanitized_outputs,
        summary.candidate_report.state_canonicalizations,
        summary.candidate_report.invalid_recoveries,
        summary.hash,
        summary.passes(),
    )
}

fn complete_comparison() {
    let configurations = configurations();
    let rows = probe_rows(&configurations);
    assert_eq!(configurations.len(), 64);
    assert_eq!(rows.len(), 296);
    let mut transcript = Transcript::create();
    transcript.record(format!(
        "issue-031 begin attempt=2 matrix_invocations=1 timed_benchmark_invocations=0 equation_version={EQUATION_VERSION:016x} configurations={} probes={} grid_hash={:016x} seed={NOISE_SEED:016x}",
        configurations.len(),
        rows.len(),
        grid_hash(&configurations, &rows),
    ));
    transcript.record(format!(
        "issue-031 layout candidate_payload_bytes=48 candidate_state_bytes=16 baseline_payload_bytes=24 baseline_state_bytes=8 w4_projection=f64x2x2 w8_projection=f64x4x2 vector_operation_ceiling=2 scratch_bytes=0 latency_samples=0 tail_samples=0 rust_candidate_bytes={}",
        size_of::<RetainedF64IncrementalV1>(),
    ));
    let mut summary = Summary::new();
    run_analytic(&rows, &mut summary);
    transcript.record(format!(
        "issue-031 phase=analytic rows={} transfer_failures={} response_failures={} worst_transfer={:.17e} worst_response_db={:.17e} hash={:016x}",
        summary.analytic_rows,
        summary.transfer_failures,
        summary.analytic_failures,
        summary.worst_transfer_error,
        summary.worst_analytic_error_db,
        summary.hash,
    ));
    run_impulses(&configurations, &mut summary);
    transcript.record(format!(
        "issue-031 phase=impulse configurations={} failures={} worst_dft_db={:.17e} hash={:016x}",
        summary.impulse_configurations,
        summary.impulse_failures,
        summary.worst_impulse_error_db,
        summary.hash,
    ));
    run_sustained(&rows, &mut summary);
    transcript.record(format!(
        "issue-031 phase=sustained rows={} hash={:016x}",
        summary.sustained_rows, summary.hash,
    ));
    run_sequences(&configurations, &mut summary);
    transcript.record(format!(
        "issue-031 phase=sequences rows={} limited_rows={} regression_failures={} hash={:016x}",
        summary.sequence_rows, summary.limited_rows, summary.regression_failures, summary.hash,
    ));
    run_semantics(&mut summary);
    transcript.record(format!(
        "issue-031 phase=semantics rows={} failures={} hash={:016x}",
        summary.semantic_rows, summary.semantic_failures, summary.hash,
    ));
    transcript.record(summary_record(&summary));
    transcript.record(format!(
        "issue-031 decision={}",
        if summary.passes() {
            "SELECTED_FOR_SEPARATE_IMPLEMENTATION"
        } else {
            "NO_ADOPTION"
        }
    ));
    transcript.finish();
}

#[test]
fn issue031_grid_is_exact_and_deterministic() {
    let configurations = configurations();
    let rows = probe_rows(&configurations);
    assert_eq!(configurations.len(), 64);
    assert_eq!(rows.len(), 296);
    for (rate_index, rate) in RATES.into_iter().enumerate() {
        let maximum = f32::from_bits(MAXIMUM_BITS[rate_index]);
        let matching = configurations
            .iter()
            .filter(|configuration| configuration.rate == rate)
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 16);
        assert_eq!(matching[6].cutoff.to_bits(), maximum.to_bits() - 1);
        assert_eq!(matching[7].cutoff.to_bits(), maximum.to_bits());
        assert_eq!(matching[14].cutoff.to_bits(), maximum.to_bits() - 1);
        assert_eq!(matching[15].cutoff.to_bits(), maximum.to_bits());
    }
    assert_eq!(grid_hash(&configurations, &rows), 0xb2a2_d521_a519_e55a);
}

#[test]
fn issue031_candidate_transfer_matches_independent_rbj_and_static_cost() {
    assert_eq!(6 * size_of::<f64>(), 48);
    assert_eq!(2 * size_of::<f64>(), 16);
    assert_eq!(6 * size_of::<f32>(), 24);
    assert_eq!(2 * size_of::<f32>(), 8);
    for configuration in configurations() {
        let candidate = RetainedF64IncrementalV1::design(configuration).transfer();
        let oracle = DirectFormOracle::design(configuration).transfer;
        for (actual, expected) in candidate.words().into_iter().zip(oracle.words()) {
            assert!((actual - expected).abs() <= 1e-12);
        }
    }
}

#[test]
fn issue031_candidate_sanitizes_and_recovers_lane_locally() {
    let configuration = Configuration {
        rate: 48_000,
        kind: FilterKind::LowPass,
        cutoff: 1_000.0,
    };
    let mut affected = RetainedF64IncrementalV1::design(configuration);
    let mut unaffected = RetainedF64IncrementalV1::design(configuration);
    let mut control = RetainedF64IncrementalV1::design(configuration);
    affected.s1 = f64::NAN;
    assert_eq!(affected.process(f32::NAN).to_bits(), 0.0_f32.to_bits());
    let input = 0.25;
    assert_eq!(
        unaffected.process(input).to_bits(),
        control.process(input).to_bits()
    );
    assert_eq!(unaffected.state_bits(), control.state_bits());
    assert_eq!(affected.report.sanitized_inputs, 1);
    assert_eq!(affected.report.invalid_recoveries, 1);
    affected.s1 = f64::from(f32::MIN_POSITIVE) * 0.5;
    affected.s2 = -0.0;
    let _ = affected.process(0.0);
    assert!(affected.report.state_canonicalizations >= 2);
    assert!(
        affected
            .state_bits()
            .into_iter()
            .all(|word| word != (-0.0_f64).to_bits())
    );
    affected.reset();
    assert_eq!(affected.state_bits(), [0, 0]);
    let mut negative_zero = RetainedF64IncrementalV1::design(Configuration {
        kind: FilterKind::HighPass,
        ..configuration
    });
    assert_eq!(negative_zero.process(-0.0).to_bits(), 0.0_f32.to_bits());
}

#[test]
fn issue031_partition_driver_preserves_bits_state_and_reports() {
    let configuration = Configuration {
        rate: 44_100,
        kind: FilterKind::HighPass,
        cutoff: 100.0,
    };
    let mut state = NOISE_SEED;
    let input = (0..2_049)
        .map(|_| splitmix_bipolar(&mut state) * 0.5)
        .collect::<Vec<_>>();
    for kind in [
        RealizationKind::Baseline,
        RealizationKind::Candidate,
        RealizationKind::Oracle,
    ] {
        let rendered = assert_partition_identity(kind, configuration, &input);
        assert_eq!(rendered.samples.len(), input.len());
    }
}

#[test]
fn issue031_transcript_schema_has_stable_required_fields() {
    let mut summary = Summary::new();
    summary.analytic_rows = 296;
    summary.impulse_configurations = 64;
    summary.sustained_rows = 296;
    summary.sequence_rows = 192;
    summary.semantic_rows = 8;
    summary.semantic_recoveries = 8;
    let record = summary_record(&summary);
    for field in [
        "analytic_rows=296",
        "impulse_configurations=64",
        "sustained_rows=296",
        "sequence_rows=192",
        "semantic_rows=8",
        "semantic_recoveries=8",
        "limited_rate_mask=0",
        "limited_kind_mask=0",
        "hash=",
        "first_failure=",
        "pass=",
    ] {
        assert!(record.contains(field), "missing transcript field {field}");
    }
}

#[test]
#[ignore = "Issue-031 permits one complete non-timed comparison after Sol pre-run authorization"]
fn issue031_complete_retained_f64_comparison_requires_sol_authorization() {
    complete_comparison();
}
