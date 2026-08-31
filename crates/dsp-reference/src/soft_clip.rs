//! Independent `f64` oracle for the fixed-two-times cubic soft clipper.

/// Errors returned by the bounded soft-clip oracle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceSoftClipError {
    /// A parameter was outside the frozen scalar domain.
    Parameter,
}

const TAPS: [usize; 31] = [
    2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 31, 32, 34, 36, 38, 40, 42, 44, 46, 48,
    50, 52, 54, 56, 58, 60,
];

/// Independently design the frozen 63-tap Blackman halfband filter in `f64`.
///
/// It uses the ideal pi/2 response and a 63-point Blackman window, scales only off-centre terms
/// to a one-half sum, and fixes the centre to one half.
#[must_use]
pub fn reference_halfband_63() -> [f64; 63] {
    let mut table = [0.0; 63];
    let mut off_centre_sum = 0.0;
    for (index, value) in table.iter_mut().enumerate() {
        let relative = index as i32 - 31;
        if relative == 0 {
            continue;
        }
        let r = relative as f64;
        let ideal = (core::f64::consts::PI * r * 0.5).sin() / (core::f64::consts::PI * r);
        let phase = 2.0 * core::f64::consts::PI * index as f64 / 62.0;
        let window = 0.42 - 0.5 * phase.cos() + 0.08 * (2.0 * phase).cos();
        *value = ideal * window;
        off_centre_sum += *value;
    }
    let scale = 0.5 / off_centre_sum;
    for (index, value) in table.iter_mut().enumerate() {
        if index != 31 {
            *value *= scale;
        }
    }
    table[31] = 0.5;
    table
}

/// Evaluate the independent memoryless cubic used by the naive one-times alias baseline.
#[must_use]
pub fn reference_cubic_soft_clip(value: f64) -> f64 {
    cubic(value)
}

/// Offline, independent `f64` realization of the frozen scalar lane.
#[derive(Clone, Debug)]
pub struct ReferenceSoftClip {
    h: [f64; 63],
    interp: [f64; 63],
    decim: [f64; 63],
    dry: [f64; 32],
    high_cursor: usize,
    dry_cursor: usize,
    drive_gain: f64,
    output_gain: f64,
    mix: f64,
}

impl ReferenceSoftClip {
    /// Create a reference lane with frozen external parameter values.
    pub fn new(drive_db: f64, output_db: f64, mix: f64) -> Result<Self, ReferenceSoftClipError> {
        if !drive_db.is_finite()
            || !output_db.is_finite()
            || !mix.is_finite()
            || !(-24.0..=36.0).contains(&drive_db)
            || !(-24.0..=24.0).contains(&output_db)
            || !(0.0..=1.0).contains(&mix)
        {
            return Err(ReferenceSoftClipError::Parameter);
        }
        Ok(Self {
            h: reference_halfband_63(),
            interp: [0.0; 63],
            decim: [0.0; 63],
            dry: [0.0; 32],
            high_cursor: 0,
            dry_cursor: 0,
            drive_gain: 10.0_f64.powf(drive_db * 0.05),
            output_gain: 10.0_f64.powf(output_db * 0.05),
            mix,
        })
    }

    /// Render one host-rate input sample.
    #[must_use]
    pub fn process(&mut self, input: f64) -> f64 {
        self.dry[self.dry_cursor] = input;
        let delayed = self.dry[(self.dry_cursor + 1) % 32];
        self.dry_cursor = (self.dry_cursor + 1) % 32;
        let wet = self.stage(2.0 * self.drive_gain * input);
        let _discarded = self.stage(0.0);
        self.output_gain * ((1.0 - self.mix) * delayed + self.mix * wet)
    }

    fn stage(&mut self, input: f64) -> f64 {
        self.interp[self.high_cursor] = input;
        let interpolated = self.convolve(&self.interp);
        self.decim[self.high_cursor] = cubic(interpolated);
        let output = self.convolve(&self.decim);
        self.high_cursor = (self.high_cursor + 1) % 63;
        output
    }

    fn convolve(&self, history: &[f64; 63]) -> f64 {
        let mut sum = 0.0;
        for index in TAPS {
            sum += self.h[index] * history[(self.high_cursor + 63 - index) % 63];
        }
        sum
    }
}

fn cubic(value: f64) -> f64 {
    if value <= -1.0 {
        -2.0 / 3.0
    } else if value >= 1.0 {
        2.0 / 3.0
    } else {
        value - value * value * value / 3.0
    }
}
