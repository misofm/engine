//! Independent offline `f64` Linkwitz-Riley fourth-order crossover reference.
//!
//! This test-only oracle owns its own coefficient design and section state. It deliberately
//! neither imports production multiband types nor shares their scalar recurrence implementation.

/// Reference construction failed because the rate or frozen crossover domain was invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceMultibandError {
    /// Input was non-finite or outside the fixed two-band launch domain.
    InvalidInput,
}

#[derive(Clone, Copy, Debug)]
struct Coefficients {
    c1: f64,
    a2: f64,
    a3: f64,
    k: f64,
}

#[derive(Clone, Copy, Debug, Default)]
struct Section {
    s1: f64,
    s2: f64,
}

impl Section {
    fn process(&mut self, input: f64, coefficients: Coefficients) -> (f64, f64) {
        let v3 = input - self.s2;
        let d1 = coefficients.a2 * v3 - coefficients.c1 * self.s1;
        let v1 = self.s1 + d1;
        let d2 = coefficients.a2 * self.s1 + coefficients.a3 * v3;
        let v2 = self.s2 + d2;
        self.s1 += d1 + d1;
        self.s2 += d2 + d2;
        (v2, (input - coefficients.k * v1) - v2)
    }
}

/// Independently state-owning two-section low/high Linkwitz-Riley crossover.
#[derive(Clone, Debug)]
pub struct ReferenceLr4Crossover {
    coefficients: Coefficients,
    low_a: Section,
    low_b: Section,
    high_a: Section,
    high_b: Section,
}

impl ReferenceLr4Crossover {
    /// Designs the fixed Butterworth-Q conditioned TPT sections in `f64`.
    pub fn new(sample_rate_hz: f64, crossover_hz: f64) -> Result<Self, ReferenceMultibandError> {
        if !sample_rate_hz.is_finite()
            || !crossover_hz.is_finite()
            || sample_rate_hz <= 0.0
            || !(80.0..=8_000.0).contains(&crossover_hz)
            || crossover_hz >= 0.5 * sample_rate_hz
        {
            return Err(ReferenceMultibandError::InvalidInput);
        }
        let g = (core::f64::consts::PI * crossover_hz / sample_rate_hz).tan();
        let k = core::f64::consts::SQRT_2;
        let t1 = g * (g + k);
        let denominator = 1.0 + t1;
        let coefficients = Coefficients {
            c1: t1 / denominator,
            a2: g / denominator,
            a3: (g * g) / denominator,
            k,
        };
        if ![
            coefficients.c1,
            coefficients.a2,
            coefficients.a3,
            coefficients.k,
        ]
        .into_iter()
        .all(f64::is_finite)
        {
            return Err(ReferenceMultibandError::InvalidInput);
        }
        Ok(Self {
            coefficients,
            low_a: Section::default(),
            low_b: Section::default(),
            high_a: Section::default(),
            high_b: Section::default(),
        })
    }

    /// Returns the independent low/high LR4 sample pair.
    pub fn process_sample(&mut self, input: f64) -> (f64, f64) {
        let (low_a, _) = self.low_a.process(input, self.coefficients);
        let (low, _) = self.low_b.process(low_a, self.coefficients);
        let (_, high_a) = self.high_a.process(input, self.coefficients);
        let (_, high) = self.high_b.process(high_a, self.coefficients);
        (low, high)
    }
}
