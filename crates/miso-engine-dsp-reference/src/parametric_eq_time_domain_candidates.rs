//! Issue-044 test-only comparison of bounded `f32` state recurrences.
//!
//! This module deliberately stays inside the independent reference crate's test boundary. It
//! retains the Issue-042 endpoint-conditioned transfer words but never imports a production EQ
//! processor. The comparison is evidence for Sol's recurrence freeze, not a runtime design.

use crate::{
    ReferenceParametricEqCoefficients, ReferenceParametricEqKind, ReferenceParametricEqSection,
};

const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const IMPULSE_FRAMES_PER_SECOND: usize = 1;
const MILLION_SAMPLES: usize = 1_000_000;
const DFT_TOLERANCE_DB: f64 = 0.05;
const DIRECT_HISTORY_SCALE: f32 = 16_777_216.0;
const INV_DIRECT_HISTORY_SCALE: f32 = 1.0 / DIRECT_HISTORY_SCALE;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    Bell,
    LowShelf,
    HighShelf,
    LowPass,
    HighPass,
    Notch,
}

impl Kind {
    const ALL: [Self; 6] = [
        Self::Bell,
        Self::LowShelf,
        Self::HighShelf,
        Self::LowPass,
        Self::HighPass,
        Self::Notch,
    ];

    const fn reference(self) -> ReferenceParametricEqKind {
        match self {
            Self::Bell => ReferenceParametricEqKind::Bell,
            Self::LowShelf => ReferenceParametricEqKind::LowShelf,
            Self::HighShelf => ReferenceParametricEqKind::HighShelf,
            Self::LowPass => ReferenceParametricEqKind::LowPass,
            Self::HighPass => ReferenceParametricEqKind::HighPass,
            Self::Notch => ReferenceParametricEqKind::Notch,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Edge {
    Low,
    High,
}

impl Edge {
    const ALL: [Self; 2] = [Self::Low, Self::High];

    const fn values(self) -> (f32, f32, f32, f32) {
        match self {
            Self::Low => (10.0, -24.0, 0.1, 0.1),
            Self::High => (20_000.0, 24.0, 18.0, 1.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Row {
    rate: u32,
    kind: Kind,
    edge: Edge,
}

impl Row {
    const fn values(self) -> (f32, f32, f32, f32) {
        self.edge.values()
    }

    fn reference(self) -> ReferenceParametricEqCoefficients {
        let (frequency, gain, q, slope) = self.values();
        ReferenceParametricEqCoefficients::design(
            self.kind.reference(),
            f64::from(self.rate),
            f64::from(frequency),
            f64::from(gain),
            f64::from(q),
            f64::from(slope),
        )
        .expect("the frozen Issue-044 row is legal in the independent oracle")
    }
}

fn rows() -> impl Iterator<Item = Row> {
    RATES.into_iter().flat_map(|rate| {
        Kind::ALL.into_iter().flat_map(move |kind| {
            Edge::ALL
                .into_iter()
                .map(move |edge| Row { rate, kind, edge })
        })
    })
}

/// The selected Issue-042 transfer words, rounded once to `f32` in retained-field order.
#[derive(Clone, Copy)]
struct Transfer {
    a: f32,
    n0: f32,
    d0: f32,
    n1: f32,
    d1: f32,
    n2: f32,
    d2: f32,
}

impl Transfer {
    fn design(row: Row) -> Self {
        let reference = row.reference();
        let (b0, b1, b2, a1, a2) = reference.values();
        let (frequency, _, _, _) = row.values();
        let a = if frequency <= row.rate as f32 * 0.25 {
            1.0_f32
        } else {
            -1.0_f32
        };
        let af = f64::from(a);
        let transfer = Self {
            a,
            n0: (b0 + af * b1 + b2) as f32,
            d0: (1.0 + af * a1 + a2) as f32,
            n1: (b1 + 2.0 * af * b2) as f32,
            d1: (a1 + 2.0 * af * a2) as f32,
            n2: b2 as f32,
            d2: a2 as f32,
        };
        assert!(
            [
                transfer.a,
                transfer.n0,
                transfer.d0,
                transfer.n1,
                transfer.d1,
                transfer.n2,
                transfer.d2,
            ]
            .into_iter()
            .all(f32::is_finite)
        );
        assert_ne!(transfer.scale(), 0.0);
        transfer
    }

    fn scale(self) -> f32 {
        let q0 = self.a * self.d1;
        (self.d0 - q0) + self.d2
    }

    fn direct_form(self) -> (f32, f32, f32, f32, f32) {
        let scale = self.scale();
        let inverse_scale = 1.0 / scale;
        let b0 = ((self.n0 - self.a * self.n1) + self.n2) * inverse_scale;
        let b1 = (self.n1 - (2.0 * self.a) * self.n2) * inverse_scale;
        let b2 = self.n2 * inverse_scale;
        let a1 = (self.d1 - (2.0 * self.a) * self.d2) * inverse_scale;
        let a2 = self.d2 * inverse_scale;
        (b0, b1, b2, a1, a2)
    }

    fn words(self) -> [u32; 7] {
        [
            self.a.to_bits(),
            self.n0.to_bits(),
            self.d0.to_bits(),
            self.n1.to_bits(),
            self.d1.to_bits(),
            self.n2.to_bits(),
            self.d2.to_bits(),
        ]
    }
}

#[derive(Clone, Copy, Default)]
struct StepEvents {
    underflow_events: u32,
    recovery: bool,
    first_bad_bits: Option<u32>,
}

impl StepEvents {
    fn underflow(&mut self, value: f32) {
        self.underflow_events += 1;
        self.first_bad_bits.get_or_insert(value.to_bits());
    }

    fn recover(&mut self, value: f32) {
        self.recovery = true;
        self.first_bad_bits.get_or_insert(value.to_bits());
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum BoundaryPolicy {
    Recover,
    FlushFiniteSubnormal,
}

/// Valid runtime values are normal floats or the single positive-zero bit pattern.
fn normal_or_positive_zero(value: f32) -> bool {
    value.is_normal() || value.to_bits() == 0
}

/// Applies a candidate's explicit state-update boundary policy.
///
/// Only finite nonzero subnormals are permitted to become `+0`, and only in the designated
/// policy candidate. Negative zero, infinity, and NaN are state failures rather than decay.
fn boundary(value: f32, policy: BoundaryPolicy, events: &mut StepEvents) -> Option<f32> {
    if normal_or_positive_zero(value) {
        return Some(value);
    }
    if value.is_finite() && value != 0.0 {
        events.underflow(value);
        if policy == BoundaryPolicy::FlushFiniteSubnormal {
            return Some(0.0);
        }
    }
    events.recover(value);
    None
}

#[derive(Clone, Copy)]
struct ScaledDirectHistory {
    transfer: Transfer,
    scaled_x1: f32,
    scaled_x2: f32,
    scaled_y1: f32,
    scaled_y2: f32,
}

impl ScaledDirectHistory {
    fn new(transfer: Transfer) -> Self {
        Self {
            transfer,
            scaled_x1: 0.0,
            scaled_x2: 0.0,
            scaled_y1: 0.0,
            scaled_y2: 0.0,
        }
    }

    fn reset(&mut self) {
        self.scaled_x1 = 0.0;
        self.scaled_x2 = 0.0;
        self.scaled_y1 = 0.0;
        self.scaled_y2 = 0.0;
    }

    fn step(&mut self, input: f32) -> (f32, StepEvents) {
        let mut events = StepEvents::default();
        let x1 = self.scaled_x1 * INV_DIRECT_HISTORY_SCALE;
        let x2 = self.scaled_x2 * INV_DIRECT_HISTORY_SCALE;
        let y1 = self.scaled_y1 * INV_DIRECT_HISTORY_SCALE;
        let y2 = self.scaled_y2 * INV_DIRECT_HISTORY_SCALE;
        let t0 = self.transfer.a * input;
        let dx = x1 - t0;
        let t1 = self.transfer.a * x1;
        let t2 = x2 - t1;
        let t3 = self.transfer.a * dx;
        let ddx = t2 - t3;
        let p0 = self.transfer.n0 * input;
        let p1 = self.transfer.n1 * dx;
        let s0 = p0 + p1;
        let p2 = self.transfer.n2 * ddx;
        let numerator = s0 + p2;
        let q1 = self.transfer.a * self.transfer.d2;
        let q2 = (self.transfer.d1 - q1) - q1;
        let history = q2 * y1 + self.transfer.d2 * y2;
        let output = (numerator - history) / self.transfer.scale();
        let Some(output) = boundary(output, BoundaryPolicy::Recover, &mut events) else {
            self.reset();
            return (0.0, events);
        };
        let next = [
            input * DIRECT_HISTORY_SCALE,
            x1 * DIRECT_HISTORY_SCALE,
            output * DIRECT_HISTORY_SCALE,
            y1 * DIRECT_HISTORY_SCALE,
        ];
        let mut cleaned = [0.0; 4];
        for (destination, value) in cleaned.iter_mut().zip(next) {
            let Some(value) = boundary(value, BoundaryPolicy::Recover, &mut events) else {
                self.reset();
                return (0.0, events);
            };
            *destination = value;
        }
        self.scaled_x2 = cleaned[1];
        self.scaled_x1 = cleaned[0];
        self.scaled_y2 = cleaned[3];
        self.scaled_y1 = cleaned[2];
        (output, events)
    }

    fn state(self) -> ([f32; 4], usize) {
        (
            [
                self.scaled_x1,
                self.scaled_x2,
                self.scaled_y1,
                self.scaled_y2,
            ],
            4,
        )
    }
}

#[derive(Clone, Copy)]
struct TransposedInternal {
    transfer: Transfer,
    s1: f32,
    s2: f32,
}

impl TransposedInternal {
    fn new(transfer: Transfer) -> Self {
        Self {
            transfer,
            s1: 0.0,
            s2: 0.0,
        }
    }

    fn reset(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
    }

    fn step(&mut self, input: f32) -> (f32, StepEvents) {
        let mut events = StepEvents::default();
        let (b0, b1, b2, a1, a2) = self.transfer.direct_form();
        let output = b0 * input + self.s1;
        let Some(output) = boundary(output, BoundaryPolicy::Recover, &mut events) else {
            self.reset();
            return (0.0, events);
        };
        let next_s1 = (b1 * input - a1 * output) + self.s2;
        let next_s2 = b2 * input - a2 * output;
        let Some(next_s1) = boundary(next_s1, BoundaryPolicy::Recover, &mut events) else {
            self.reset();
            return (0.0, events);
        };
        let Some(next_s2) = boundary(next_s2, BoundaryPolicy::Recover, &mut events) else {
            self.reset();
            return (0.0, events);
        };
        self.s1 = next_s1;
        self.s2 = next_s2;
        (output, events)
    }

    fn state(self) -> ([f32; 4], usize) {
        ([self.s1, self.s2, 0.0, 0.0], 2)
    }
}

#[derive(Clone, Copy)]
struct FlushDirectHistory {
    transfer: Transfer,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl FlushDirectHistory {
    fn new(transfer: Transfer) -> Self {
        Self {
            transfer,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }

    fn step(&mut self, input: f32) -> (f32, StepEvents) {
        let mut events = StepEvents::default();
        let t0 = self.transfer.a * input;
        let dx = self.x1 - t0;
        let t1 = self.transfer.a * self.x1;
        let t2 = self.x2 - t1;
        let t3 = self.transfer.a * dx;
        let ddx = t2 - t3;
        let p0 = self.transfer.n0 * input;
        let p1 = self.transfer.n1 * dx;
        let s0 = p0 + p1;
        let p2 = self.transfer.n2 * ddx;
        let numerator = s0 + p2;
        let q1 = self.transfer.a * self.transfer.d2;
        let q2 = (self.transfer.d1 - q1) - q1;
        let history = q2 * self.y1 + self.transfer.d2 * self.y2;
        let output = (numerator - history) / self.transfer.scale();
        let Some(output) = boundary(output, BoundaryPolicy::FlushFiniteSubnormal, &mut events)
        else {
            self.reset();
            return (0.0, events);
        };
        let next = [input, self.x1, output, self.y1];
        let mut cleaned = [0.0; 4];
        for (destination, value) in cleaned.iter_mut().zip(next) {
            let Some(value) = boundary(value, BoundaryPolicy::FlushFiniteSubnormal, &mut events)
            else {
                self.reset();
                return (0.0, events);
            };
            *destination = value;
        }
        self.x2 = cleaned[1];
        self.x1 = cleaned[0];
        self.y2 = cleaned[3];
        self.y1 = cleaned[2];
        (output, events)
    }

    fn state(self) -> ([f32; 4], usize) {
        ([self.x1, self.x2, self.y1, self.y2], 4)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CandidateKind {
    ScaledDirectHistory,
    TransposedInternal,
    FlushDirectHistory,
}

impl CandidateKind {
    const ALL: [Self; 3] = [
        Self::ScaledDirectHistory,
        Self::TransposedInternal,
        Self::FlushDirectHistory,
    ];

    const fn state_words(self) -> usize {
        match self {
            Self::ScaledDirectHistory | Self::FlushDirectHistory => 4,
            Self::TransposedInternal => 2,
        }
    }
}

enum Candidate {
    Scaled(ScaledDirectHistory),
    Transposed(TransposedInternal),
    Flush(FlushDirectHistory),
}

impl Candidate {
    fn new(kind: CandidateKind, transfer: Transfer) -> Self {
        match kind {
            CandidateKind::ScaledDirectHistory => Self::Scaled(ScaledDirectHistory::new(transfer)),
            CandidateKind::TransposedInternal => {
                Self::Transposed(TransposedInternal::new(transfer))
            }
            CandidateKind::FlushDirectHistory => Self::Flush(FlushDirectHistory::new(transfer)),
        }
    }

    fn step(&mut self, input: f32) -> (f32, StepEvents) {
        match self {
            Self::Scaled(candidate) => candidate.step(input),
            Self::Transposed(candidate) => candidate.step(input),
            Self::Flush(candidate) => candidate.step(input),
        }
    }

    fn state(&self) -> ([f32; 4], usize) {
        match self {
            Self::Scaled(candidate) => candidate.state(),
            Self::Transposed(candidate) => candidate.state(),
            Self::Flush(candidate) => candidate.state(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Sequence {
    Impulse,
    MillionSample,
}

#[derive(Clone, Copy, Debug)]
struct EventLocation {
    sequence: Sequence,
    row: Row,
    sample: usize,
    bits: u32,
}

impl EventLocation {
    fn evidence(self) -> (Sequence, Row, usize, u32) {
        (self.sequence, self.row, self.sample, self.bits)
    }
}

struct Summary {
    candidate: CandidateKind,
    impulse_cases: u32,
    million_cases: u32,
    underflow_events: u64,
    recovery_events: u64,
    invalid_values: u64,
    worst_output: f64,
    worst_state: f64,
    minimum_nonzero: f64,
    worst_dft_error: f64,
    dft_failures: u32,
    first_underflow: Option<EventLocation>,
    first_invalid: Option<EventLocation>,
    hash: u64,
}

impl Summary {
    fn new(candidate: CandidateKind) -> Self {
        Self {
            candidate,
            impulse_cases: 0,
            million_cases: 0,
            underflow_events: 0,
            recovery_events: 0,
            invalid_values: 0,
            worst_output: 0.0,
            worst_state: 0.0,
            minimum_nonzero: f64::INFINITY,
            worst_dft_error: 0.0,
            dft_failures: 0,
            first_underflow: None,
            first_invalid: None,
            hash: 0xcbf2_9ce4_8422_2325,
        }
    }

    fn mix(&mut self, word: u64) {
        self.hash ^= word;
        self.hash = self.hash.wrapping_mul(0x0000_0100_0000_01b3);
    }

    fn observe_value(&mut self, value: f32, state: bool) {
        let magnitude = f64::from(value).abs();
        if state {
            self.worst_state = self.worst_state.max(magnitude);
        } else {
            self.worst_output = self.worst_output.max(magnitude);
        }
        if magnitude != 0.0 {
            self.minimum_nonzero = self.minimum_nonzero.min(magnitude);
        }
    }

    fn observe_step(
        &mut self,
        candidate: &Candidate,
        output: f32,
        events: StepEvents,
        sequence: Sequence,
        row: Row,
        sample: usize,
    ) {
        self.mix(u64::from(output.to_bits()));
        self.observe_value(output, false);
        if !normal_or_positive_zero(output) {
            self.invalid_values += 1;
            self.first_invalid.get_or_insert(EventLocation {
                sequence,
                row,
                sample,
                bits: output.to_bits(),
            });
        }
        self.underflow_events += u64::from(events.underflow_events);
        if events.underflow_events != 0 {
            self.first_underflow.get_or_insert(EventLocation {
                sequence,
                row,
                sample,
                bits: events.first_bad_bits.unwrap_or_default(),
            });
        }
        if events.recovery {
            self.recovery_events += 1;
            self.first_invalid.get_or_insert(EventLocation {
                sequence,
                row,
                sample,
                bits: events.first_bad_bits.unwrap_or_default(),
            });
        }
        let (state, words) = candidate.state();
        for value in state.into_iter().take(words) {
            self.mix(u64::from(value.to_bits()));
            self.observe_value(value, true);
            if !normal_or_positive_zero(value) {
                self.invalid_values += 1;
                self.first_invalid.get_or_insert(EventLocation {
                    sequence,
                    row,
                    sample,
                    bits: value.to_bits(),
                });
            }
        }
    }

    fn selectable(&self) -> bool {
        self.impulse_cases == 48
            && self.million_cases == 48
            && self.recovery_events == 0
            && self.invalid_values == 0
            && self.dft_failures == 0
            && self.worst_dft_error <= DFT_TOLERANCE_DB
    }
}

fn impulse_dft_db(samples: &[f32], rate: u32, frequency: f64) -> f64 {
    let phase = -core::f64::consts::TAU * frequency / f64::from(rate);
    let (step_re, step_im) = (phase.cos(), phase.sin());
    let (mut unit_re, mut unit_im) = (1.0_f64, 0.0_f64);
    let (mut re, mut im) = (0.0_f64, 0.0_f64);
    for sample in samples {
        let sample = f64::from(*sample);
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

/// Independent `f64` one-second impulse DFT with the same finite window and probe as a candidate.
///
/// Comparing a truncated candidate impulse with the infinite-duration analytic transfer would
/// charge the candidate for the reference tail omitted by the frozen one-second window.
fn reference_impulse_dft_db(row: Row, frequency: f64) -> f64 {
    let mut reference = ReferenceParametricEqSection::new(row.reference());
    let phase = -core::f64::consts::TAU * frequency / f64::from(row.rate);
    let (step_re, step_im) = (phase.cos(), phase.sin());
    let (mut unit_re, mut unit_im) = (1.0_f64, 0.0_f64);
    let (mut re, mut im) = (0.0_f64, 0.0_f64);
    let frames = row.rate as usize * IMPULSE_FRAMES_PER_SECOND;
    for sample in 0..frames {
        let output = reference.process(if sample == 0 { 1.0 } else { 0.0 });
        re += output * unit_re;
        im += output * unit_im;
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

fn run_impulse_case(candidate_kind: CandidateKind, row: Row, summary: &mut Summary) {
    let transfer = Transfer::design(row);
    for word in transfer.words() {
        summary.mix(u64::from(word));
    }
    let mut candidate = Candidate::new(candidate_kind, transfer);
    let frames = row.rate as usize * IMPULSE_FRAMES_PER_SECOND;
    let mut impulse = Vec::with_capacity(frames);
    for sample in 0..frames {
        let input = if sample == 0 { 1.0 } else { 0.0 };
        let (output, events) = candidate.step(input);
        summary.observe_step(&candidate, output, events, Sequence::Impulse, row, sample);
        impulse.push(output);
    }
    let (frequency, _, _, _) = row.values();
    let expected = reference_impulse_dft_db(row, f64::from(frequency));
    let actual = impulse_dft_db(&impulse, row.rate, f64::from(frequency));
    summary.mix(actual.to_bits());
    summary.mix(expected.to_bits());
    if expected >= -120.0 {
        let error = (actual - expected).abs();
        summary.worst_dft_error = summary.worst_dft_error.max(error);
        if !error.is_finite() || error > DFT_TOLERANCE_DB {
            summary.dft_failures += 1;
        }
    }
    summary.impulse_cases += 1;
}

fn run_million_sample_case(candidate_kind: CandidateKind, row: Row, summary: &mut Summary) {
    let transfer = Transfer::design(row);
    for word in transfer.words() {
        summary.mix(u64::from(word));
    }
    let mut candidate = Candidate::new(candidate_kind, transfer);
    let mut state = 0x0000_0000_0012_e911_u64
        ^ u64::from(row.rate)
        ^ (u64::from(row.kind as u8) << 32)
        ^ (u64::from(row.edge as u8) << 40);
    for sample in 0..MILLION_SAMPLES {
        let input = if sample == 0 {
            0.99
        } else {
            deterministic_noise(&mut state)
        };
        let (output, events) = candidate.step(input);
        summary.observe_step(
            &candidate,
            output,
            events,
            Sequence::MillionSample,
            row,
            sample,
        );
    }
    summary.million_cases += 1;
}

fn compare_candidate(kind: CandidateKind) -> Summary {
    let mut summary = Summary::new(kind);
    for row in rows() {
        run_impulse_case(kind, row, &mut summary);
    }
    for row in rows() {
        run_million_sample_case(kind, row, &mut summary);
    }
    summary
}

#[test]
#[ignore = "Issue #44 exhausted both attempts without a selectable time-domain recurrence"]
fn issue_044_complete_time_domain_recurrence_comparison_requires_sol_freeze() {
    let summaries: Vec<_> = CandidateKind::ALL
        .into_iter()
        .map(compare_candidate)
        .collect();
    for summary in &summaries {
        eprintln!(
            "issue-044 candidate={:?} retained_words=7 state_words={} impulse_cases={} million_cases={} underflows={} recoveries={} invalid={} max_output={:.12e} max_state={:.12e} min_nonzero={:.12e} worst_dft_db={:.12} dft_failures={} hash={:016x} first_underflow={:?} first_invalid={:?}",
            summary.candidate,
            summary.candidate.state_words(),
            summary.impulse_cases,
            summary.million_cases,
            summary.underflow_events,
            summary.recovery_events,
            summary.invalid_values,
            summary.worst_output,
            summary.worst_state,
            summary.minimum_nonzero,
            summary.worst_dft_error,
            summary.dft_failures,
            summary.hash,
            summary.first_underflow.map(EventLocation::evidence),
            summary.first_invalid.map(EventLocation::evidence),
        );
    }
    let selectable: Vec<_> = summaries
        .iter()
        .filter(|summary| summary.selectable())
        .collect();
    assert_eq!(
        selectable.len(),
        1,
        "the frozen preimplementation comparison requires exactly one selectable recurrence"
    );
    assert_eq!(
        selectable[0].candidate,
        CandidateKind::FlushDirectHistory,
        "the candidate result is evidence for Sol's later freeze, not a production selection"
    );
}
