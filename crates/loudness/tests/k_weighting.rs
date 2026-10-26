//! The K-weighting design: the BS.1770-5 48 kHz table, and the prewarped bilinear transform of
//! the analog prototype at every launch rate.

#![allow(clippy::disallowed_methods)]
// Test file: compares against the platform's own f64 transcendentals deliberately (D6 exemption).

use loudness::{
    HIGHPASS_GAIN_RATE, HIGHPASS_HZ, HIGHPASS_Q, KWeighting, SHELF_BAND_EXPONENT, SHELF_GAIN_DB,
    SHELF_HZ, SHELF_Q, SUPPORTED_RATES,
};

/// BS.1770-5 Annex 1, Tables 1 and 2, `[b0, b1, b2, a1, a2]`.
const TABLE_STAGE1: [f64; 5] = [
    1.535_124_859_586_97,
    -2.691_696_189_406_38,
    1.198_392_810_852_85,
    -1.690_659_293_182_41,
    0.732_480_774_215_85,
];
const TABLE_STAGE2: [f64; 5] = [1.0, -2.0, 1.0, -1.990_047_454_833_98, 0.990_072_250_366_21];

#[test]
fn the_48_khz_design_is_the_standard_table() {
    let design = KWeighting::new(48_000).expect("48 kHz");
    for (stage, got, table) in [
        (1, design.stage1(), TABLE_STAGE1),
        (2, design.stage2(), TABLE_STAGE2),
    ] {
        for (index, (got, want)) in got.iter().zip(table).enumerate() {
            assert!(
                (got - want).abs() <= 1e-6,
                "stage {stage} coefficient {index}: {got} against the table's {want}"
            );
        }
    }
    assert_eq!(
        KWeighting::new(192_000),
        Err(loudness::LoudnessError::UnsupportedRate(192_000))
    );
}

#[derive(Clone, Copy)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
    fn scale(self, k: f64) -> Self {
        Self {
            re: self.re * k,
            im: self.im * k,
        }
    }
    fn div(self, other: Self) -> Self {
        let d = other.re * other.re + other.im * other.im;
        Self {
            re: (self.re * other.re + self.im * other.im) / d,
            im: (self.im * other.re - self.re * other.im) / d,
        }
    }
    fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }
}

/// `H(e^{jw})` of `[b0, b1, b2, a1, a2]`.
fn digital(coefficients: [f64; 5], w: f64) -> Complex {
    let [b0, b1, b2, a1, a2] = coefficients;
    let z1 = Complex {
        re: w.cos(),
        im: -w.sin(),
    };
    let z2 = z1.mul(z1);
    let one = Complex { re: 1.0, im: 0.0 };
    let numerator = one.scale(b0).add(z1.scale(b1)).add(z2.scale(b2));
    let denominator = one.add(z1.scale(a1)).add(z2.scale(a2));
    numerator.div(denominator)
}

/// `(n2 s^2 + n1 s + n0) / (s^2 + s/q + 1)` at `s = j omega`.
fn analog(n2: f64, n1: f64, n0: f64, q: f64, omega: f64) -> Complex {
    let numerator = Complex {
        re: n0 - n2 * omega * omega,
        im: n1 * omega,
    };
    let denominator = Complex {
        re: 1.0 - omega * omega,
        im: omega / q,
    };
    numerator.div(denominator)
}

#[test]
fn every_rate_is_the_prewarped_bilinear_transform_of_the_prototype() {
    let vh = 10.0_f64.powf(SHELF_GAIN_DB / 20.0);
    let vb = vh.powf(SHELF_BAND_EXPONENT);
    let k48 = (core::f64::consts::PI * HIGHPASS_HZ / f64::from(HIGHPASS_GAIN_RATE)).tan();
    let g48 = 1.0 + k48 / HIGHPASS_Q + k48 * k48;
    for rate in SUPPORTED_RATES {
        let fs = f64::from(rate);
        let design = KWeighting::new(rate).expect("launch rate");
        let k1 = (core::f64::consts::PI * SHELF_HZ / fs).tan();
        let k2 = (core::f64::consts::PI * HIGHPASS_HZ / fs).tan();
        let mut worst = 0.0_f64;
        let mut hz = 10.0;
        while hz <= 0.45 * fs {
            let w = core::f64::consts::TAU * hz / fs;
            let got = digital(design.stage1(), w).mul(digital(design.stage2(), w));
            let prewarped = (w / 2.0).tan();
            let want = analog(vh, vb / SHELF_Q, 1.0, SHELF_Q, prewarped / k1).mul(analog(
                g48,
                0.0,
                0.0,
                HIGHPASS_Q,
                prewarped / k2,
            ));
            let error = Complex {
                re: got.re - want.re,
                im: got.im - want.im,
            }
            .abs()
                / want.abs();
            worst = worst.max(error);
            hz *= 1.01;
        }
        assert!(
            worst <= 1e-9,
            "{rate} Hz: worst relative response error {worst:e}"
        );
    }
}
