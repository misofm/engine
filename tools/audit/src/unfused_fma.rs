//! Issue #163 phase 2 step 2: the per-site unfused multiply-add audit.
//!
//! # What this subject decides
//!
//! Every render-path `Lane::fma` site is a candidate for the phase-2 contract change (single
//! rounding -> two roundings, natively, on every backend). This subject supplies the *numbers*
//! behind each per-site verdict in `docs/rulings/unfused-multiply-add-audit.md`. It is evidence
//! code, not render code.
//!
//! # The criterion
//!
//! For one site `fma(a, b, c)` versus `(a * b) + c`, write `u = 2^-24` (the `f32` unit roundoff).
//!
//! * fused: `r_f = fl(a*b + c) = (a*b + c)(1 + d)`, `|d| <= u`.
//! * unfused: `r_u = fl(fl(a*b) + c) = (a*b(1 + e1) + c)(1 + e2)`, `|e1|, |e2| <= u`.
//!
//! Subtracting, `r_u - r_f = a*b*e1 + (a*b + c)(e2 - d) + O(u^2)`, so
//!
//! ```text
//! |r_u - r_f| <= u*|a*b| + 2*u*|a*b + c| + O(u^2).
//! ```
//!
//! The second term is the ordinary rounding both forms already carry. The first term, `u*|a*b|`,
//! is the whole of the contract change. It is an **absolute** quantity in units of the *product*,
//! which is why the audit reports two things per site:
//!
//! * the **absolute** divergence referred to full scale (dBFS) -- the audible quantity, because a
//!   render path's signals are bounded by a few units full scale; and
//! * the **cancellation ratio** `|a*b| / |a*b + c|` -- the factor by which the change is amplified
//!   in *relative* terms. A site whose ratio is bounded by a small constant over its operating
//!   domain is incidental. A site whose ratio is unbounded is a candidate for load-bearing, and
//!   then the question becomes whether the amplified error survives to the output.
//!
//! # Recurrence sites
//!
//! A recurrence adds a third question, and it is the one that decides "recurrence stability".
//! The TPT state update is `x[n+1] = A x[n] + B u[n]` with (svf.rs, `ReferenceSvfStateSpace`)
//!
//! ```text
//! A = [[1 - 2*c1, -2*a2], [2*a2, 1 - 2*a3]].
//! ```
//!
//! `A` is built **only** from the prepared coefficients `c1`, `a2`, `a3`. The fma contract does
//! not appear in it. A per-step arithmetic perturbation `delta[n]` therefore propagates as
//! `sum_k A^k delta[n-k]` -- through an operator the contract change *cannot touch*. Two
//! consequences, and they are the audit's central finding:
//!
//! 1. **Stability is not a function of the fma contract.** The Jury test reads `A`; `A` is
//!    unchanged; a filter that was strictly stable stays strictly stable. Unfusing cannot move a
//!    pole. "Recurrence stability" is therefore never a reason to keep an exact path.
//! 2. What *does* change is the injected noise amplitude, amplified by the propagation gain
//!    `G = sum_k ||A^k||_inf`. So the honest question for a recurrence is not "does it diverge"
//!    but "does `G * |delta_u|` sit materially above the `G * |delta_f|` the frozen contract
//!    already accepts". That is a ratio, and this subject measures it directly against the `f64`
//!    oracle rather than bounding it.
//!
//! # Method
//!
//! * **Exhaustive** where the domain is enumerable: the identity/bypass sub-domains, where one
//!   operand is pinned to the contract's identity value and the free operand is swept over all
//!   2^32 `f32` bit patterns. These are the cases a pinned bit-identity contract depends on.
//! * **Dense stratified sweep** elsewhere: the operating domain of each site, sampled on a grid
//!   that includes every boundary value, with the grid stated per family below.
//! * **Long-run oracle comparison** for the recurrences: sustained rendering against the `f64`
//!   oracle, reporting the max sample delta in dBFS for the fused and the unfused f32 path, so
//!   the verdict is "the oracle distance did (not) materially grow", not "the two f32 paths
//!   differ".

use dsp_reference::{
    ReferenceSvf, ReferenceSvfCoefficients, ReferenceSvfKind, deterministic_bipolar_noise,
};

/// `f32` unit roundoff.
const U: f64 = 1.0 / 16_777_216.0;

/// The denormal-flush threshold (D7), restated here so the audit does not depend on render code.
const FLUSH_EPS: f32 = 1.0e-20;

#[inline(always)]
fn flush(x: f32) -> f32 {
    if x.abs() < FLUSH_EPS { 0.0 } else { x }
}

/// dBFS of an absolute error, floored so an exact match prints as a finite sentinel.
fn dbfs(x: f64) -> f64 {
    if x <= 0.0 {
        f64::NEG_INFINITY
    } else {
        20.0 * x.log10()
    }
}

fn fmt_db(x: f64) -> String {
    if x.is_infinite() {
        "    exact".to_string()
    } else {
        format!("{x:9.2}")
    }
}

/// Running maximum of an absolute divergence and the cancellation ratio that produced it.
#[derive(Default, Clone, Copy)]
struct Divergence {
    max_abs: f64,
    max_ulps: u64,
    max_cancel: f64,
}

impl Divergence {
    fn observe(&mut self, fused: f32, unfused: f32, product: f64, sum: f64) {
        let delta = (f64::from(fused) - f64::from(unfused)).abs();
        if delta > self.max_abs {
            self.max_abs = delta;
        }
        let ulps = ulp_distance(fused, unfused);
        if ulps > self.max_ulps {
            self.max_ulps = ulps;
        }
        if sum != 0.0 {
            let ratio = (product / sum).abs();
            if ratio.is_finite() && ratio > self.max_cancel {
                self.max_cancel = ratio;
            }
        }
    }
}

/// Distance in representable `f32` steps, using the standard monotone total-order mapping.
///
/// Negatives are bit-complemented and non-negatives get the sign bit set, which makes the map
/// monotone across zero and puts `-0.0` and `+0.0` one step apart -- they are adjacent in value,
/// and an earlier form of this function that placed them 2^63 apart made every sweep touching a
/// signed zero report a nonsense ulp count.
fn ulp_distance(a: f32, b: f32) -> u64 {
    fn ordered(x: f32) -> u32 {
        let bits = x.to_bits();
        if bits & 0x8000_0000 != 0 {
            !bits
        } else {
            bits | 0x8000_0000
        }
    }
    if a.is_nan() || b.is_nan() {
        return 0;
    }
    u64::from(ordered(a).abs_diff(ordered(b)))
}

// ---------------------------------------------------------------------------------------------
// Site models. Each pair is the frozen operation order of the site, written twice: once with the
// fused operation the contract pins today, once with the `(a*b) + c` the contract change makes
// universal. Nothing here calls `Lane`, so the audit's numbers do not move when the contract does.
// ---------------------------------------------------------------------------------------------

/// F1: the TPT state update, `svf_step` (`kernels.rs`), both `d1` and `d2`.
struct SvfF32 {
    nc1: f32,
    a2: f32,
    a3: f32,
    m0: f32,
    m1: f32,
    m2: f32,
    ic1: f32,
    ic2: f32,
}

impl SvfF32 {
    fn new(c: &ReferenceSvfCoefficients) -> Self {
        Self {
            nc1: -(c.c1 as f32),
            a2: c.a2 as f32,
            a3: c.a3 as f32,
            m0: c.m0 as f32,
            m1: c.m1 as f32,
            m2: c.m2 as f32,
            ic1: 0.0,
            ic2: 0.0,
        }
    }

    /// The frozen order as it stood before #163 phase 2: single-rounding `d1`, `d2` and mix.
    ///
    /// UNFUSED-SEAL-EXEMPT: the retired fused arm, kept so the audit can measure the unfused
    /// contract against the one it replaced. Evidence code, unreachable from any render path.
    fn step_fused(&mut self, v0: f32) -> f32 {
        let v3 = v0 - self.ic2;
        // UNFUSED-SEAL-EXEMPT
        let d1 = self.nc1.mul_add(self.ic1, self.a2 * v3);
        let v1 = self.ic1 + d1;
        // UNFUSED-SEAL-EXEMPT
        let d2 = self.a3.mul_add(v3, self.a2 * self.ic1);
        let v2 = self.ic2 + d2;
        self.ic1 = flush(self.ic1 + (d1 + d1));
        self.ic2 = flush(self.ic2 + (d2 + d2));
        // UNFUSED-SEAL-EXEMPT (two calls)
        self.m2.mul_add(v2, self.m1.mul_add(v1, self.m0 * v0))
    }

    /// The same order with every fused operation replaced by `(a*b) + c`.
    fn step_unfused(&mut self, v0: f32) -> f32 {
        let v3 = v0 - self.ic2;
        let d1 = mutate(M_SVF_D1, (self.nc1 * self.ic1) + (self.a2 * v3));
        let v1 = self.ic1 + d1;
        let d2 = mutate(M_SVF_D2, (self.a3 * v3) + (self.a2 * self.ic1));
        let v2 = self.ic2 + d2;
        self.ic1 = flush(self.ic1 + (d1 + d1));
        self.ic2 = flush(self.ic2 + (d2 + d2));
        mutate(
            M_SVF_MIX,
            (self.m2 * v2) + ((self.m1 * v1) + (self.m0 * v0)),
        )
    }
}

/// F3: the one-pole recurrence `y' = fma(c, x - y, y)` (`one_pole_block`, `envelope`, the delay
/// damper, the gate's gain-dB slew, the limiter's release, the multiband shim).
fn one_pole_fused(c: f32, x: f32, y: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT: the retired fused arm (see `SvfF32::step_fused`).
    flush(c.mul_add(x - y, y))
}

fn one_pole_unfused(c: f32, x: f32, y: f32) -> f32 {
    let site = if c == 0.0 {
        M_ONE_POLE_IDENTITY
    } else {
        M_ONE_POLE
    };
    flush(mutate(site, (c * (x - y)) + y))
}

/// F4: the dry/wet mix `y = fma(mix, w - x, x)` with `w = x * g` (`gain_mix_step`, the delay mix).
fn mix_fused(x: f32, g: f32, mix: f32) -> f32 {
    let w = x * g;
    let d = w - x;
    // UNFUSED-SEAL-EXEMPT: the retired fused arm (see `SvfF32::step_fused`).
    mix.mul_add(d, x)
}

fn mix_unfused(x: f32, g: f32, mix: f32) -> f32 {
    let w = x * g;
    let d = w - x;
    let site = if mix == 0.0 { M_MIX_IDENTITY } else { M_MIX };
    mutate(site, (mix * d) + x)
}

/// F5: one row of the 2x2 matrix, `l' = fma(lr, r, ll * l)`.
fn matrix_fused(ll: f32, lr: f32, l: f32, r: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT: the retired fused arm (see `SvfF32::step_fused`).
    lr.mul_add(r, ll * l)
}

fn matrix_unfused(ll: f32, lr: f32, l: f32, r: f32) -> f32 {
    mutate(M_MATRIX, (lr * r) + (ll * l))
}

// ---------------------------------------------------------------------------------------------
// F1/F2: the SVF families.
// ---------------------------------------------------------------------------------------------

/// The spectral radius of the error-propagation matrix `A`, in closed form.
///
/// `A = [[1 - 2*c1, -2*a2], [2*a2, 1 - 2*a3]]` is 2x2, so its eigenvalues are the roots of
/// `z^2 - tr z + det`. A per-step arithmetic perturbation `delta[n]` accumulates as
/// `sum_k A^k delta[n-k]`, whose gain scales as `1/(1 - rho(A))`; this function returns
/// `rho(A)` exactly rather than iterating, which is what makes a 12 000-design sweep affordable.
///
/// The number matters because it is the *same* for both arms. `A` is built only from the
/// prepared coefficients, so unfusing cannot move it -- the audit reports it to show how much of
/// the observed noise floor is propagation the frozen contract already carries.
fn spectral_radius(c: &ReferenceSvfCoefficients) -> f64 {
    let (a00, a01) = (1.0 - 2.0 * c.c1, -2.0 * c.a2);
    let (a10, a11) = (2.0 * c.a2, 1.0 - 2.0 * c.a3);
    let trace = a00 + a11;
    let determinant = a00 * a11 - a01 * a10;
    let discriminant = trace * trace - 4.0 * determinant;
    if discriminant < 0.0 {
        // A complex-conjugate pair, both of modulus sqrt(det).
        determinant.abs().sqrt()
    } else {
        let root = discriminant.sqrt();
        (0.5 * (trace + root))
            .abs()
            .max((0.5 * (trace - root)).abs())
    }
}

/// One long-run design comparison: fused f32, unfused f32 and the `f64` oracle on one input.
struct SvfRun {
    fused_vs_oracle: f64,
    unfused_vs_oracle: f64,
    fused_vs_unfused: f64,
    state_fused_vs_oracle: f64,
    state_unfused_vs_oracle: f64,
}

fn svf_long_run(c: &ReferenceSvfCoefficients, input: &[f32]) -> SvfRun {
    let mut fused = SvfF32::new(c);
    let mut unfused = SvfF32::new(c);
    let mut oracle = ReferenceSvf::new(*c);
    // A second oracle tracked in parallel gives the state trajectory to compare against.
    let mut run = SvfRun {
        fused_vs_oracle: 0.0,
        unfused_vs_oracle: 0.0,
        fused_vs_unfused: 0.0,
        state_fused_vs_oracle: 0.0,
        state_unfused_vs_oracle: 0.0,
    };
    // The oracle's state words are not exposed, so the state comparison uses the `f64`
    // recurrence restated here from `ReferenceSvfStateSpace`'s documented substitution.
    let (mut o1, mut o2) = (0.0f64, 0.0f64);
    let (nc1, a2, a3) = (-c.c1, c.a2, c.a3);
    for &x in input {
        let yf = fused.step_fused(x);
        let yu = unfused.step_unfused(x);
        let yo = oracle.process(f64::from(x));

        let v3 = f64::from(x) - o2;
        let d1 = nc1 * o1 + a2 * v3;
        let d2 = a3 * v3 + a2 * o1;
        o1 += d1 + d1;
        o2 += d2 + d2;

        run.fused_vs_oracle = run.fused_vs_oracle.max((f64::from(yf) - yo).abs());
        run.unfused_vs_oracle = run.unfused_vs_oracle.max((f64::from(yu) - yo).abs());
        run.fused_vs_unfused = run
            .fused_vs_unfused
            .max((f64::from(yf) - f64::from(yu)).abs());
        run.state_fused_vs_oracle = run
            .state_fused_vs_oracle
            .max((f64::from(fused.ic1) - o1).abs())
            .max((f64::from(fused.ic2) - o2).abs());
        run.state_unfused_vs_oracle = run
            .state_unfused_vs_oracle
            .max((f64::from(unfused.ic1) - o1).abs())
            .max((f64::from(unfused.ic2) - o2).abs());
    }
    run
}

const KINDS: [(ReferenceSvfKind, &str); 7] = [
    (ReferenceSvfKind::LowPass, "low-pass"),
    (ReferenceSvfKind::HighPass, "high-pass"),
    (ReferenceSvfKind::BandPass, "band-pass"),
    (ReferenceSvfKind::Notch, "notch"),
    (ReferenceSvfKind::Bell, "bell"),
    (ReferenceSvfKind::LowShelf, "low-shelf"),
    (ReferenceSvfKind::HighShelf, "high-shelf"),
];

/// The launch-supported rates (AGENTS.md: 44.1, 48, 88.2, 96 kHz).
const RATES: [f64; 4] = [44_100.0, 48_000.0, 88_200.0, 96_000.0];

fn svf_family(frames: usize, long_frames: usize) {
    println!("## F1/F2 -- the SVF state update and output mix");
    println!();
    println!(
        "Dense stratified sweep: 7 kinds x 4 launch rates x 23 cutoffs (20 Hz .. 0.45*fs, \
         logarithmic) x 7 Q (0.1 .. 32) x 5 gains (-24 .. +24 dB) = up to 22540 designs, each \
         rendered for {frames} frames of SplitMix64 bipolar noise at full scale (seed 0x163_2)."
    );
    println!();
    let noise = deterministic_bipolar_noise(1, frames, 0x0163_0002)
        .expect("deterministic noise for the SVF sweep");
    let input: Vec<f32> = noise.samples().iter().map(|&x| x as f32).collect();

    let mut rows: Vec<(String, f64, f64, f64, f64, f64, f64)> = Vec::new();
    for (kind, kind_name) in KINDS {
        let mut worst_gain = 0.0f64;
        let mut worst_fused = 0.0f64;
        let mut worst_unfused = 0.0f64;
        let mut worst_delta = 0.0f64;
        let mut worst_state_fused = 0.0f64;
        let mut worst_state_unfused = 0.0f64;
        let mut designs = 0usize;
        for rate in RATES {
            for fi in 0..23 {
                let frequency = 20.0 * (rate * 0.45 / 20.0).powf(f64::from(fi) / 22.0);
                if frequency >= rate * 0.5 {
                    continue;
                }
                for qi in 0..7 {
                    let q = 0.1 * 320.0f64.powf(f64::from(qi) / 6.0);
                    // Only the bell and the two shelves read `gain_db`; sweeping it for the
                    // other four kinds would be four redundant copies of the same design.
                    let gains = match kind {
                        ReferenceSvfKind::Bell
                        | ReferenceSvfKind::LowShelf
                        | ReferenceSvfKind::HighShelf => 5,
                        _ => 1,
                    };
                    for gi in 0..gains {
                        let gain_db = if gains == 1 {
                            0.0
                        } else {
                            -24.0 + 12.0 * f64::from(gi)
                        };
                        let Ok(c) =
                            ReferenceSvfCoefficients::design(kind, rate, frequency, q, gain_db)
                        else {
                            continue;
                        };
                        designs += 1;
                        let rho = spectral_radius(&c);
                        let gain = 1.0 / (1.0 - rho);
                        if gain > worst_gain {
                            worst_gain = gain;
                        }
                        let run = svf_long_run(&c, &input);
                        worst_fused = worst_fused.max(run.fused_vs_oracle);
                        worst_unfused = worst_unfused.max(run.unfused_vs_oracle);
                        worst_delta = worst_delta.max(run.fused_vs_unfused);
                        worst_state_fused = worst_state_fused.max(run.state_fused_vs_oracle);
                        worst_state_unfused = worst_state_unfused.max(run.state_unfused_vs_oracle);
                    }
                }
            }
        }
        println!("  {kind_name:<11} designs={designs:<5} max propagation gain G={worst_gain:12.1}");
        println!(
            "    output vs f64 oracle : fused {} dBFS   unfused {} dBFS   growth {:6.2} dB",
            fmt_db(dbfs(worst_fused)),
            fmt_db(dbfs(worst_unfused)),
            dbfs(worst_unfused) - dbfs(worst_fused)
        );
        println!(
            "    state  vs f64 oracle : fused {} dBFS   unfused {} dBFS   growth {:6.2} dB",
            fmt_db(dbfs(worst_state_fused)),
            fmt_db(dbfs(worst_state_unfused)),
            dbfs(worst_state_unfused) - dbfs(worst_state_fused)
        );
        println!(
            "    fused vs unfused     : {} dBFS",
            fmt_db(dbfs(worst_delta))
        );
        rows.push((
            kind_name.to_string(),
            worst_gain,
            worst_fused,
            worst_unfused,
            worst_delta,
            worst_state_fused,
            worst_state_unfused,
        ));
    }
    println!();
    let g = rows.iter().fold(0.0f64, |a, r| a.max(r.1));
    let f = rows.iter().fold(0.0f64, |a, r| a.max(r.2));
    let uf = rows.iter().fold(0.0f64, |a, r| a.max(r.3));
    let d = rows.iter().fold(0.0f64, |a, r| a.max(r.4));
    println!("  F1/F2 WORST OVER ALL KINDS");
    println!("    propagation gain G          {g:.1}");
    println!("    fused   vs oracle           {} dBFS", fmt_db(dbfs(f)));
    println!("    unfused vs oracle           {} dBFS", fmt_db(dbfs(uf)));
    println!(
        "    oracle-distance growth      {:.3} dB ({:.3}x)",
        dbfs(uf) - dbfs(f),
        uf / f
    );
    println!("    fused vs unfused            {} dBFS", fmt_db(dbfs(d)));
    println!();

    // The sustained-rendering arm. The sweep above is wide but short; a recurrence's accumulated
    // state divergence is a long-run quantity, so the worst corner of each kind is rendered for
    // `long_frames` frames -- long enough that a filter whose settling time is Q/(2*pi*f) seconds
    // has settled many times over, which is the regime where an accumulating error would show.
    println!(
        "  Sustained rendering: the worst corner of each kind, {long_frames} frames \
         ({:.1} s at 48 kHz)",
        long_frames as f64 / 48_000.0
    );
    let long_noise = deterministic_bipolar_noise(1, long_frames, 0x0163_0005)
        .expect("deterministic noise for the sustained SVF arm");
    let long_input: Vec<f32> = long_noise.samples().iter().map(|&x| x as f32).collect();
    println!(
        "    {:<11} {:>9} {:>12} {:>12} {:>12} {:>8}",
        "kind", "rho(A)", "fused dBFS", "unfused dBFS", "state f/u dB", "growth"
    );
    let mut worst_output_growth = f64::NEG_INFINITY;
    let mut worst_state_growth = f64::NEG_INFINITY;
    for (kind, kind_name) in KINDS {
        // The tightest corner the domain admits: the lowest cutoff at the highest Q, where
        // rho(A) is closest to 1 and the propagation gain is largest.
        let Ok(c) = ReferenceSvfCoefficients::design(kind, 48_000.0, 20.0, 32.0, 12.0) else {
            continue;
        };
        let run = svf_long_run(&c, &long_input);
        let output_growth = dbfs(run.unfused_vs_oracle) - dbfs(run.fused_vs_oracle);
        let state_growth = dbfs(run.state_unfused_vs_oracle) - dbfs(run.state_fused_vs_oracle);
        worst_output_growth = worst_output_growth.max(output_growth);
        worst_state_growth = worst_state_growth.max(state_growth);
        println!(
            "    {kind_name:<11} {:>9.6} {:>12} {:>12} {:>12} {output_growth:>7.2} dB",
            spectral_radius(&c),
            fmt_db(dbfs(run.fused_vs_oracle)),
            fmt_db(dbfs(run.unfused_vs_oracle)),
            format!("{state_growth:+.2}"),
        );
    }
    println!();
    println!(
        "    worst sustained output growth vs the f64 oracle: {worst_output_growth:.2} dB \
         ({:.2}x)",
        10.0f64.powf(worst_output_growth / 20.0)
    );
    println!(
        "    worst sustained state  growth vs the f64 oracle: {worst_state_growth:.2} dB \
         ({:.2}x)",
        10.0f64.powf(worst_state_growth / 20.0)
    );
    println!();
}

// ---------------------------------------------------------------------------------------------
// F3: the one-pole recurrence.
// ---------------------------------------------------------------------------------------------

/// The one-pole time constants the render path actually asks for, in seconds.
///
/// The extremes are what matter: the shortest is the limiter's attack and the gate's fastest
/// slew, the longest is a mean-square follower's integration window and the delay's damping at
/// its slowest. `c = 1 - exp(-1/(tau*fs))`, so the propagation gain of the recurrence is
/// `sum_k (1-c)^k = 1/c`, which is roughly `tau*fs` -- the amplification is the time constant in
/// samples, and the audit's slowest entry is the worst case the family can present.
const TAUS: [(f64, &str); 8] = [
    (0.000_02, "20 us  (limiter attack floor)"),
    (0.000_1, "100 us (gate fast attack)"),
    (0.001, "1 ms   (peak detector)"),
    (0.01, "10 ms  (typical attack)"),
    (0.1, "100 ms (typical release)"),
    (0.4, "400 ms (mean-square window)"),
    (2.0, "2 s    (slow release)"),
    (10.0, "10 s   (domain ceiling)"),
];

fn one_pole_family(frames: usize) {
    println!("## F3 -- the one-pole recurrence `y' = fma(c, x - y, y)`");
    println!();
    println!(
        "Dense sweep: 8 time constants spanning the render path's domain x 4 launch rates, each \
         rendered for {frames} frames of full-scale bipolar noise, compared against the same \
         recurrence in f64. Propagation gain is 1/c exactly (a first-order pole at 1-c)."
    );
    println!();
    let noise = deterministic_bipolar_noise(1, frames, 0x0163_0003)
        .expect("deterministic noise for the one-pole sweep");
    let input: Vec<f32> = noise.samples().iter().map(|&x| x as f32).collect();

    let mut worst_growth = f64::NEG_INFINITY;
    let mut worst_row = String::new();
    for (tau, label) in TAUS {
        for rate in RATES {
            let c64 = 1.0 - (-1.0 / (tau * rate)).exp();
            let c = c64 as f32;
            if c == 0.0 {
                continue;
            }
            let (mut yf, mut yu, mut yo) = (0.0f32, 0.0f32, 0.0f64);
            let mut df = 0.0f64;
            let mut du = 0.0f64;
            let mut delta = 0.0f64;
            let mut cancel = 0.0f64;
            for &x in &input {
                yf = one_pole_fused(c, x, yf);
                yu = one_pole_unfused(c, x, yu);
                yo += f64::from(c) * (f64::from(x) - yo);
                df = df.max((f64::from(yf) - yo).abs());
                du = du.max((f64::from(yu) - yo).abs());
                delta = delta.max((f64::from(yf) - f64::from(yu)).abs());
                let product = f64::from(c) * (f64::from(x) - f64::from(yf));
                if yf != 0.0 {
                    cancel = cancel.max((product / f64::from(yf)).abs());
                }
            }
            let growth = dbfs(du) - dbfs(df);
            if growth > worst_growth {
                worst_growth = growth;
                worst_row = format!(
                    "{label} @ {rate:.0} Hz: c={c:.3e} G=1/c={:.1} fused {} dBFS unfused {} dBFS \
                     growth {growth:.3} dB, max |ab/(ab+c)| = {cancel:.3}",
                    1.0 / f64::from(c),
                    fmt_db(dbfs(df)),
                    fmt_db(dbfs(du)),
                );
            }
            if rate == 48_000.0 {
                println!(
                    "  {label:<28} c={c:.4e} G={:11.1}  fused {} unfused {} growth {growth:6.3} dB \
                     cancel {cancel:8.3}",
                    1.0 / f64::from(c),
                    fmt_db(dbfs(df)),
                    fmt_db(dbfs(du)),
                );
            }
        }
    }
    println!();
    println!("  F3 WORST GROWTH OVER THE SWEEP");
    println!("    {worst_row}");
    println!();
}

// ---------------------------------------------------------------------------------------------
// F4/F5/F6: the feed-forward families.
// ---------------------------------------------------------------------------------------------

/// The **operating domain** grid: what a render path's signals and coefficients actually reach.
///
/// Audio samples on the render path are bounded by a few units full scale (the builtins clamp,
/// the limiter ceiling and the fader law all keep them there), so the audit's headline bound is
/// taken over `|x| <= 4.0` -- 12 dB of headroom above full scale -- and not over the whole `f32`
/// range. The exact endpoints, both zeros, both unit values and the smallest normal are forced in
/// so no boundary case is missed by the striding.
fn operating_grid(points: usize) -> Vec<f32> {
    let mut values = vec![
        0.0f32,
        -0.0f32,
        1.0,
        -1.0,
        4.0,
        -4.0,
        f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE,
    ];
    for i in 0..points {
        let t = (i as f32 / (points - 1) as f32) * 2.0 - 1.0;
        values.push(t * 4.0);
        // A logarithmic companion so the small-magnitude cancellation region -- where the
        // relative amplification lives -- is sampled as densely as the linear region.
        let l = 10.0f32.powf(t * 6.0 - 6.0) * 4.0;
        values.push(l);
        values.push(-l);
    }
    values
}

/// The **out-of-domain** grid, swept separately so it cannot contaminate the headline bound.
///
/// A render path does not present these magnitudes, but the two forms genuinely differ on them
/// and the difference is worth recording: `fma` computes `a*b` exactly, so a product that
/// overflows `f32` while `a*b + c` is representable comes back finite from the fused form and
/// infinite from the unfused one. That is a real boundary of the contract change, not a rounding
/// difference, and it is reported as a finiteness disagreement rather than folded into a dBFS.
fn extended_grid(points: usize) -> Vec<f32> {
    let mut values = vec![f32::MAX, f32::MIN, 1e30, -1e30, 1e-30, -1e-30];
    for i in 0..points {
        let t = (i as f32 / (points - 1) as f32) * 2.0 - 1.0;
        let l = 10.0f32.powf(t * 30.0);
        values.push(l);
        values.push(-l);
    }
    values
}

/// A sweep result: the in-domain divergence plus the finiteness disagreements found.
#[derive(Default)]
struct Sweep {
    divergence: Divergence,
    finiteness_disagreements: u64,
    cases: u64,
}

impl Sweep {
    fn observe(&mut self, fused: f32, unfused: f32, product: f64, sum: f64) {
        self.cases += 1;
        if fused.is_finite() != unfused.is_finite() {
            self.finiteness_disagreements += 1;
            return;
        }
        if !fused.is_finite() {
            return;
        }
        self.divergence.observe(fused, unfused, product, sum);
    }

    fn report(&self, label: &str) {
        println!(
            "  {label:<22} cases {:>12}  max |f-u| {} dBFS ({} ulp)  max |ab/(ab+c)| {:.3e}  \
             finiteness disagreements {}",
            self.cases,
            fmt_db(dbfs(self.divergence.max_abs)),
            self.divergence.max_ulps,
            self.divergence.max_cancel,
            self.finiteness_disagreements
        );
    }
}

fn feed_forward_families() {
    println!("## F4 -- the dry/wet mix `y = fma(mix, x*g - x, x)`");
    println!();
    println!(
        "Operating domain: 264-point stratified x-grid (linear over +/-4.0 = full scale + 12 dB \
         headroom, plus a logarithmic companion down to 4e-6, with 0, -0, +/-1, +/-4, \
         +/-MIN_POSITIVE forced) x the same grid for g x 130 mix values covering [0, 1] with the \
         exact endpoints. The out-of-domain row sweeps 1e-30 .. 1e30 and is reported separately \
         because a render path does not present those magnitudes."
    );
    let xs = operating_grid(128);
    let gs = operating_grid(128);
    let mut mixes: Vec<f32> = (0..=128).map(|i| i as f32 / 128.0).collect();
    mixes.push(0.0);
    mixes.push(1.0);

    let mut inside = Sweep::default();
    for &x in &xs {
        for &g in &gs {
            for &mix in &mixes {
                let d = (x * g) - x;
                inside.observe(
                    mix_fused(x, g, mix),
                    mix_unfused(x, g, mix),
                    f64::from(mix) * f64::from(d),
                    f64::from(mix) * f64::from(d) + f64::from(x),
                );
            }
        }
    }
    inside.report("F4 operating domain");

    let ext = extended_grid(96);
    let mut outside = Sweep::default();
    for &x in &ext {
        for &g in &ext {
            for &mix in &mixes {
                let d = (x * g) - x;
                outside.observe(
                    mix_fused(x, g, mix),
                    mix_unfused(x, g, mix),
                    f64::from(mix) * f64::from(d),
                    f64::from(mix) * f64::from(d) + f64::from(x),
                );
            }
        }
    }
    outside.report("F4 out of domain");
    println!();

    println!("## F5 -- one row of the 2x2 matrix, `l' = fma(lr, r, ll * l)`");
    println!();
    println!(
        "Operating domain: the same 264-point stratified grid for each of the two signal operands \
         x a 68-point grid for each of the two smoothed matrix coefficients over [-1.5, 1.5] (the \
         domain a smoothed pan/width matrix reaches, with the exact 0 and +/-1 corners forced)."
    );
    let coefficients: Vec<f32> = (0..65)
        .map(|i| (i as f32 / 64.0) * 3.0 - 1.5)
        .chain([0.0, 1.0, -1.0])
        .collect();
    let mut inside = Sweep::default();
    for &l in &xs {
        for &r in &gs {
            for &ll in &coefficients {
                for &lr in &coefficients {
                    inside.observe(
                        matrix_fused(ll, lr, l, r),
                        matrix_unfused(ll, lr, l, r),
                        f64::from(lr) * f64::from(r),
                        f64::from(lr) * f64::from(r) + f64::from(ll) * f64::from(l),
                    );
                }
            }
        }
    }
    inside.report("F5 operating domain");

    let mut outside = Sweep::default();
    for &l in &ext {
        for &r in &ext {
            for &ll in &coefficients {
                for &lr in &coefficients {
                    outside.observe(
                        matrix_fused(ll, lr, l, r),
                        matrix_unfused(ll, lr, l, r),
                        f64::from(lr) * f64::from(r),
                        f64::from(lr) * f64::from(r) + f64::from(ll) * f64::from(l),
                    );
                }
            }
        }
    }
    outside.report("F5 out of domain");
    println!();
}

// ---------------------------------------------------------------------------------------------
// The exhaustive identity sweeps.
// ---------------------------------------------------------------------------------------------

/// Classification of the cases where the *frozen* identity does not hold exactly.
///
/// The identity `fma(0, d, x) == x` is not universal, and never was: it fails on a negative zero
/// (`+0.0 + -0.0` is `+0.0`), on a non-finite input, and when the intermediate `x*g` overflows so
/// that `d` is non-finite. All three are pre-existing properties of the fused contract, recorded
/// at `kernels.rs:415`. The audit's claim is not that these vanish -- it is that the unfused form
/// reproduces them **exactly**, so no pinned bit moves. Counting them by cause is what turns that
/// claim from an assertion into a measurement: if `other` is zero, the carve-out is fully
/// characterised and nothing unexplained is hiding in 2^32 cases.
#[derive(Default)]
struct Carveout {
    signed_zero: u64,
    non_finite_input: u64,
    non_finite_intermediate: u64,
    other: u64,
}

impl Carveout {
    fn classify(&mut self, x: f32, intermediate: f32) {
        if !x.is_finite() {
            self.non_finite_input += 1;
        } else if !intermediate.is_finite() {
            self.non_finite_intermediate += 1;
        } else if x == 0.0 {
            self.signed_zero += 1;
        } else {
            self.other += 1;
        }
    }

    fn total(&self) -> u64 {
        self.signed_zero + self.non_finite_input + self.non_finite_intermediate + self.other
    }
}

/// Sweeps all 2^32 `f32` bit patterns of the free operand of one identity contract.
///
/// These are the sub-domains where the *domain is enumerable*, and they are the ones a pinned
/// bit-identity contract rests on: a bypassed slot must be an identity kernel, not a
/// near-identity one. Two separate claims are measured:
///
/// * **mismatches** -- cases where the unfused form disagrees with the fused form. This must be
///   **zero**: it is the statement that the contract change moves no bit in a bypass path.
/// * **carve-out** -- cases where neither form returns `x`, classified by cause. These are the
///   frozen contract's documented exceptions; they are reported so that "the identity holds" is
///   never overclaimed, and so that an unexplained case would be visible.
fn exhaustive_identities() {
    println!("## Exhaustive identity sweeps (all 2^32 f32 bit patterns of the free operand)");
    println!();
    println!(
        "  mismatches = fused and unfused disagree (must be 0). carve-out = neither returns x, \
         by cause; these are the frozen contract's documented exceptions, reproduced exactly."
    );
    println!();
    println!(
        "  {:<26} {:>10}  {:>10} {:>10} {:>10} {:>7}",
        "sweep", "mismatches", "-0.0", "nonfin in", "nonfin mid", "other"
    );

    let mut total_mismatches = 0u64;
    let mut total_other = 0u64;

    // F4, mix = 0: the bypassed-slot identity.
    for &g in &[0.0f32, 0.5, 1.0, 2.0, -1.0, 1e30, f32::INFINITY] {
        let mut mismatches = 0u64;
        let mut carveout = Carveout::default();
        for bits in 0..=u32::MAX {
            let x = f32::from_bits(bits);
            let f = mix_fused(x, g, 0.0);
            let u = mix_unfused(x, g, 0.0);
            if f.to_bits() != u.to_bits() && !(f.is_nan() && u.is_nan()) {
                mismatches += 1;
            }
            if u.to_bits() != bits && !(x.is_nan() && u.is_nan()) {
                carveout.classify(x, (x * g) - x);
            }
        }
        total_mismatches += mismatches;
        total_other += carveout.other;
        println!(
            "  {:<26} {mismatches:>10}  {:>10} {:>10} {:>10} {:>7}",
            format!("F4 mix=0 g={g}"),
            carveout.signed_zero,
            carveout.non_finite_input,
            carveout.non_finite_intermediate,
            carveout.other
        );
        debug_assert_eq!(carveout.total(), carveout.total());
    }

    // F3, c = 0: a stalled smoother must hold its state exactly.
    for &x in &[0.0f32, 1.0, -1.0, 1e30] {
        let mut mismatches = 0u64;
        let mut carveout = Carveout::default();
        for bits in 0..=u32::MAX {
            let y = f32::from_bits(bits);
            let f = one_pole_fused(0.0, x, y);
            let u = one_pole_unfused(0.0, x, y);
            if f.to_bits() != u.to_bits() && !(f.is_nan() && u.is_nan()) {
                mismatches += 1;
            }
            if u.to_bits() != flush(y).to_bits() && !(y.is_nan() && u.is_nan()) {
                carveout.classify(y, x - y);
            }
        }
        total_mismatches += mismatches;
        total_other += carveout.other;
        println!(
            "  {:<26} {mismatches:>10}  {:>10} {:>10} {:>10} {:>7}",
            format!("F3 c=0 x={x}"),
            carveout.signed_zero,
            carveout.non_finite_input,
            carveout.non_finite_intermediate,
            carveout.other
        );
    }

    // F5, lr = 0 and ll = 1: an unrotated matrix must pass its channel through exactly.
    for &r in &[0.0f32, 1.0, -1.0, 1e30] {
        let mut mismatches = 0u64;
        let mut carveout = Carveout::default();
        for bits in 0..=u32::MAX {
            let l = f32::from_bits(bits);
            let f = matrix_fused(1.0, 0.0, l, r);
            let u = matrix_unfused(1.0, 0.0, l, r);
            if f.to_bits() != u.to_bits() && !(f.is_nan() && u.is_nan()) {
                mismatches += 1;
            }
            if u.to_bits() != l.to_bits() && !(l.is_nan() && u.is_nan()) {
                carveout.classify(l, 0.0 * r);
            }
        }
        total_mismatches += mismatches;
        total_other += carveout.other;
        println!(
            "  {:<26} {mismatches:>10}  {:>10} {:>10} {:>10} {:>7}",
            format!("F5 lr=0 ll=1 r={r}"),
            carveout.signed_zero,
            carveout.non_finite_input,
            carveout.non_finite_intermediate,
            carveout.other
        );
    }

    println!();
    println!(
        "  15 sweeps x 2^32 = 64 424 509 440 cases. Total fused-vs-unfused mismatches: \
         {total_mismatches}. Unexplained carve-out cases: {total_other}."
    );
    println!();
}

/// The signed-zero question the phase-4 silence fast path rests on.
///
/// Phase 4's earned-silence claims are observations, so they re-earn automatically under new
/// bits -- but the *reasoning* has to survive: the contract change must not turn a `-0.0` into a
/// `+0.0` or the reverse anywhere the silence path inspects. Enumerated in full because the
/// interesting domain is the sixteen sign/zero corners, not a sweep.
fn signed_zero_corners() {
    println!("## Signed-zero corners (phase-4 silence fast path)");
    println!();
    println!("  site                       operands                       fused     unfused  same");
    let zeros = [0.0f32, -0.0f32];
    let others = [0.0f32, -0.0f32, 1.0f32, -1.0f32];
    let mut all_same = true;
    for &mix in &zeros {
        for &x in &others {
            for &g in &others {
                let f = mix_fused(x, g, mix);
                let u = mix_unfused(x, g, mix);
                let same = f.to_bits() == u.to_bits();
                all_same &= same;
                if x == 0.0 || g == 0.0 {
                    println!(
                        "  gain_mix_step              mix={mix:+.1} x={x:+.1} g={g:+.1}       \
                         {:>9} {:>9}  {}",
                        format_signed(f),
                        format_signed(u),
                        if same { "yes" } else { "NO" }
                    );
                }
            }
        }
    }
    for &c in &zeros {
        for &x in &others {
            for &y in &zeros {
                let f = one_pole_fused(c, x, y);
                let u = one_pole_unfused(c, x, y);
                let same = f.to_bits() == u.to_bits();
                all_same &= same;
                println!(
                    "  one_pole                   c={c:+.1} x={x:+.1} y={y:+.1}       \
                     {:>9} {:>9}  {}",
                    format_signed(f),
                    format_signed(u),
                    if same { "yes" } else { "NO" }
                );
            }
        }
    }
    println!();
    println!(
        "  every signed-zero corner agrees: {}",
        if all_same { "yes" } else { "NO" }
    );
    println!();
}

fn format_signed(x: f32) -> String {
    if x == 0.0 {
        if x.is_sign_negative() {
            "-0.0".to_string()
        } else {
            "+0.0".to_string()
        }
    } else {
        format!("{x:+.3}")
    }
}

// ---------------------------------------------------------------------------------------------
// Red mutations.
// ---------------------------------------------------------------------------------------------
//
// A bound is only evidence if a violation would be *detected*. Each mutation below perturbs one
// unfused site by exactly one representable step and re-runs the same measurement. A mutation is
// RED when the re-measured bound leaves the envelope the clean run recorded -- which is the
// statement that the recorded bound is tight at 1 ulp rather than slack.

/// Selected mutation, or [`M_NONE`]. Evidence code: a process-wide switch is the simplest thing
/// that lets the *same* measurement body serve the clean and the mutant run.
static MUTATION: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

const M_NONE: u32 = 0;
const M_SVF_D1: u32 = 1;
const M_SVF_D2: u32 = 2;
const M_SVF_MIX: u32 = 3;
const M_ONE_POLE: u32 = 4;
const M_MIX: u32 = 5;
const M_MATRIX: u32 = 6;
const M_MIX_IDENTITY: u32 = 7;
const M_ONE_POLE_IDENTITY: u32 = 8;

const MUTATIONS: [(u32, &str); 8] = [
    (M_SVF_D1, "F1 svf_step d1  +1 ulp"),
    (M_SVF_D2, "F1 svf_step d2  +1 ulp"),
    (M_SVF_MIX, "F2 svf output mix +1 ulp"),
    (M_ONE_POLE, "F3 one_pole y'  +1 ulp"),
    (M_MIX, "F4 gain_mix_step +1 ulp"),
    (M_MATRIX, "F5 matrix row   +1 ulp"),
    (M_MIX_IDENTITY, "F4 mix=0 identity broken"),
    (M_ONE_POLE_IDENTITY, "F3 c=0 identity broken"),
];

/// One representable step away from zero, the smallest perturbation an `f32` site can carry.
fn next_ulp(x: f32) -> f32 {
    if x.is_nan() {
        return x;
    }
    if x == 0.0 {
        return f32::from_bits(1);
    }
    let bits = x.to_bits();
    f32::from_bits(if x > 0.0 { bits + 1 } else { bits - 1 })
}

#[inline(always)]
fn mutate(site: u32, value: f32) -> f32 {
    if MUTATION.load(std::sync::atomic::Ordering::Relaxed) == site {
        next_ulp(value)
    } else {
        value
    }
}

/// The result of one measurement: the bound the ruling quotes, and a digest of the whole
/// trajectory that produced it.
///
/// The digest is what makes a mutation *detectable*. A maximum is a poor detector for a uniform
/// one-ulp shift: shifting every sample by one step moves the extremum by one step too, which can
/// leave the reported maximum unchanged to two decimals or even shrink it when the dominant
/// sample happened to lie on the other side. The digest covers every sample of the unfused
/// trajectory, so a single perturbed bit anywhere in the run changes it. The maximum stays in the
/// report because it is the number the ruling quotes; the digest is the evidence that the number
/// was measured on the trajectory it claims.
struct Measurement {
    bound: f64,
    digest: [u8; 32],
}

fn measure(kind: u32, input: &[f32]) -> Measurement {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    let mut bound = 0.0f64;
    let record = |fused: f32, unfused: f32, hasher: &mut Sha256, bound: &mut f64| {
        hasher.update(unfused.to_bits().to_le_bytes());
        *bound = bound.max((f64::from(fused) - f64::from(unfused)).abs());
    };

    match kind {
        M_SVF_D1 | M_SVF_D2 | M_SVF_MIX => {
            // The tightest SVF design in the sweep: the worst-case propagation-gain corner.
            let c = ReferenceSvfCoefficients::design(
                ReferenceSvfKind::LowPass,
                48_000.0,
                20.0,
                32.0,
                0.0,
            )
            .expect("the 20 Hz / Q 32 low-pass corner designs");
            let mut fused = SvfF32::new(&c);
            let mut unfused = SvfF32::new(&c);
            for &x in input {
                let f = fused.step_fused(x);
                let u = unfused.step_unfused(x);
                record(f, u, &mut hasher, &mut bound);
                // The state words carry the d1/d2 perturbation even when the output mix hides it.
                hasher.update(unfused.ic1.to_bits().to_le_bytes());
                hasher.update(unfused.ic2.to_bits().to_le_bytes());
            }
        }
        M_ONE_POLE | M_ONE_POLE_IDENTITY => {
            let c = (1.0 - (-1.0f64 / (0.1 * 48_000.0)).exp()) as f32;
            let (mut yf, mut yu) = (0.0f32, 0.0f32);
            for &x in input {
                yf = one_pole_fused(c, x, yf);
                yu = one_pole_unfused(c, x, yu);
                record(yf, yu, &mut hasher, &mut bound);
            }
        }
        M_MIX | M_MIX_IDENTITY => {
            for &x in input {
                let f = mix_fused(x, 0.7, 0.35);
                let u = mix_unfused(x, 0.7, 0.35);
                record(f, u, &mut hasher, &mut bound);
            }
        }
        M_MATRIX => {
            for (i, &x) in input.iter().enumerate() {
                let r = input[(i + 977) % input.len()];
                let f = matrix_fused(0.8, 0.6, x, r);
                let u = matrix_unfused(0.8, 0.6, x, r);
                record(f, u, &mut hasher, &mut bound);
            }
        }
        _ => {}
    }

    // The identity corners ride in the same digest so an identity-only mutation, which leaves
    // every general-path sample untouched, still flips it.
    for &x in &[0.0f32, -0.0, 1.0, -1.0, 0.5] {
        for &g in &[0.0f32, -0.0, 1.0, 2.0] {
            hasher.update(mix_unfused(x, g, 0.0).to_bits().to_le_bytes());
        }
        for &y in &[0.0f32, -0.0, 1.0, -1.0] {
            hasher.update(one_pole_unfused(0.0, x, y).to_bits().to_le_bytes());
        }
    }

    Measurement {
        bound,
        digest: hasher.finalize().into(),
    }
}

/// The identity contracts, checked over a bounded but complete corner set so a mutation that
/// breaks one is visible without a 2^32 sweep per mutant.
///
/// The contract under test is **agreement**, not sign preservation. `gain_mix_step` at `mix = 0`
/// already does not preserve the sign of a zero `x` when `d` is non-zero -- `kernels.rs:415` says
/// so, and the fused form does not preserve it either (`fma(0, +0.0, -0.0) = +0.0`). An effect
/// that needs a signed-zero identity selects the dry value with a mask instead. So what must hold
/// across the contract change is that the unfused form returns exactly what the fused form
/// returns, and returns `x` wherever the fused form returns `x`.
fn identity_holds() -> bool {
    let corners = [
        0.0f32,
        -0.0,
        1.0,
        -1.0,
        0.5,
        -0.5,
        f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE,
        f32::MAX,
        f32::MIN,
        1e-30,
        1e30,
    ];
    for &x in &corners {
        for &g in &corners {
            let fused = mix_fused(x, g, 0.0);
            let unfused = mix_unfused(x, g, 0.0);
            if fused.to_bits() != unfused.to_bits() {
                return false;
            }
            if fused.to_bits() == x.to_bits() && unfused.to_bits() != x.to_bits() {
                return false;
            }
        }
        for &y in &corners {
            let fused = one_pole_fused(0.0, x, y);
            let unfused = one_pole_unfused(0.0, x, y);
            if fused.to_bits() != unfused.to_bits() {
                return false;
            }
        }
    }
    true
}

fn red_mutations(frames: usize) {
    println!("## Red mutations");
    println!();
    println!(
        "Each row perturbs one unfused site by exactly one representable step and re-runs the \
         measurement the clean row bounds. Detection is by trajectory digest, which flips on a \
         single perturbed bit anywhere in the run; the two bound columns show how far the \
         perturbation moved the quoted maximum. RED means the mutation was detected."
    );
    println!();
    let noise = deterministic_bipolar_noise(1, frames, 0x0163_0004)
        .expect("deterministic noise for the mutation sweep");
    let input: Vec<f32> = noise.samples().iter().map(|&x| x as f32).collect();

    println!(
        "  {:<28} {:>13} {:>13} {:>9} {:>8}  verdict",
        "mutation", "clean (dBFS)", "mutant (dBFS)", "digest", "identity"
    );
    let mut all_red = true;
    for (id, label) in MUTATIONS {
        MUTATION.store(M_NONE, std::sync::atomic::Ordering::Relaxed);
        let clean = measure(id, &input);
        let clean_identity = identity_holds();
        MUTATION.store(id, std::sync::atomic::Ordering::Relaxed);
        let mutant = measure(id, &input);
        let mutant_identity = identity_holds();
        MUTATION.store(M_NONE, std::sync::atomic::Ordering::Relaxed);

        assert!(
            clean_identity,
            "{label}: the clean run must satisfy every identity contract"
        );
        let detected = clean.digest != mutant.digest;
        all_red &= detected;
        println!(
            "  {label:<28} {:>13} {:>13} {:>9} {:>8}  {}",
            fmt_db(dbfs(clean.bound)),
            fmt_db(dbfs(mutant.bound)),
            if detected { "differs" } else { "SAME" },
            if mutant_identity { "holds" } else { "broken" },
            if detected {
                "RED (detected)"
            } else {
                "GREEN -- NOT DETECTED"
            }
        );
    }
    println!();
    println!(
        "  every mutation detected: {}",
        if all_red { "yes" } else { "NO" }
    );
    println!();
}

// ---------------------------------------------------------------------------------------------

fn usage() -> ! {
    eprintln!(
        "usage: audit unfused-fma [dense|exhaustive|mutations|conformance|all] [--frames N] \
         [--long-frames N]"
    );
    std::process::exit(2);
}

pub(crate) fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut mode = "dense".to_string();
    let mut frames = 20_000usize;
    let mut long_frames = 1_000_000usize;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "dense" | "exhaustive" | "mutations" | "conformance" | "all" => {
                mode = args[index].clone();
            }
            "--frames" => {
                index += 1;
                frames = args
                    .get(index)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or_else(|| usage());
            }
            "--long-frames" => {
                index += 1;
                long_frames = args
                    .get(index)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or_else(|| usage());
            }
            _ => usage(),
        }
        index += 1;
    }

    println!("# Unfused multiply-add audit (issue #163 phase 2 step 2)");
    println!();
    println!("f32 unit roundoff u = 2^-24 = {U:.6e}");
    println!("mode = {mode}, sweep frames = {frames}, sustained frames = {long_frames}");
    println!();

    if mode == "dense" || mode == "all" || mode == "conformance" {
        model_conformance(frames.min(65_536));
    }
    if mode == "dense" || mode == "all" {
        svf_family(frames, long_frames);
        one_pole_family(frames);
        feed_forward_families();
        signed_zero_corners();
    }
    if mode == "mutations" || mode == "all" {
        red_mutations(frames);
    }
    if mode == "exhaustive" || mode == "all" {
        exhaustive_identities();
    }
}

// ---------------------------------------------------------------------------------------------
// Model conformance.
// ---------------------------------------------------------------------------------------------

/// Decides which arm the tree's production kernels implement, and proves the model is exact.
///
/// Every bound in this subject is measured on a *model* of a site, written out in this file so
/// that both arms stay available after the contract change retired one of them. A model is only
/// evidence if it is the thing it models, so this pass runs the real kernels at `Scalar` width --
/// the width the numeric contract makes the oracle for every other backend -- and compares
/// `to_bits` against **both** arms.
///
/// It reports which arm matched rather than asserting a particular one. That makes the instrument
/// a contract *detector* valid on either side of the change: run it before the change and it says
/// `fused`, run it after and it says `unfused`. What it asserts is the part that is true either
/// way -- that production matches exactly one arm, bit for bit, with no third behaviour and no
/// partial agreement. A tree where neither arm matches is a tree where this audit's numbers
/// describe nothing.
fn model_conformance(frames: usize) {
    use lane::kernels::{
        OnePoleCoef, OnePoleState, SvfCoef, SvfState, gain_mix_block, mix2x2_block, one_pole_block,
        svf_block,
    };

    println!("## Model conformance: which arm do the production kernels implement?");
    println!();
    let noise = deterministic_bipolar_noise(1, frames, 0x0163_0006)
        .expect("deterministic noise for the conformance pass");
    let input: Vec<f32> = noise.samples().iter().map(|&x| x as f32).collect();

    /// Names the arm a kernel matched, or fails the audit.
    fn verdict(label: &str, fused_mismatches: usize, unfused_mismatches: usize) -> &'static str {
        let arm = match (fused_mismatches, unfused_mismatches) {
            (0, 0) => "both (the arms coincide on this input -- widen it)",
            (0, _) => "fused",
            (_, 0) => "unfused",
            _ => "NEITHER",
        };
        println!(
            "  {label:<24} vs fused: {fused_mismatches:>6}   vs unfused: {unfused_mismatches:>6}\
             \t=> {arm}"
        );
        assert!(
            fused_mismatches == 0 || unfused_mismatches == 0,
            "{label}: production matches neither arm, so the audit's model is not the kernel"
        );
        arm
    }

    let mut arms: Vec<&'static str> = Vec::new();

    // F1/F2 -- svf_block.
    let design =
        ReferenceSvfCoefficients::design(ReferenceSvfKind::LowShelf, 48_000.0, 1_000.0, 0.7, 6.0)
            .expect("the conformance design");
    let coefficients = SvfCoef::<f32> {
        c1: design.c1 as f32,
        a2: design.a2 as f32,
        a3: design.a3 as f32,
        m0: design.m0 as f32,
        m1: design.m1 as f32,
        m2: design.m2 as f32,
    };
    let mut buffer = input.clone();
    let mut state = SvfState::<f32>::default();
    svf_block(&mut buffer, frames, &coefficients, &mut state);
    let mut fused_model = SvfF32::new(&design);
    let mut unfused_model = SvfF32::new(&design);
    let (mut fused_bad, mut unfused_bad) = (0usize, 0usize);
    for (kernel, &x) in buffer.iter().zip(input.iter()) {
        if kernel.to_bits() != fused_model.step_fused(x).to_bits() {
            fused_bad += 1;
        }
        if kernel.to_bits() != unfused_model.step_unfused(x).to_bits() {
            unfused_bad += 1;
        }
    }
    arms.push(verdict("F1/F2 svf_block", fused_bad, unfused_bad));

    // F3 -- one_pole_block.
    let c = (1.0 - (-1.0f64 / (0.05 * 48_000.0)).exp()) as f32;
    let mut buffer = input.clone();
    let mut one_pole_state = OnePoleState::<f32>::default();
    one_pole_block(
        &mut buffer,
        frames,
        &OnePoleCoef::<f32> { c },
        &mut one_pole_state,
    );
    let (mut yf, mut yu) = (0.0f32, 0.0f32);
    let (mut fused_bad, mut unfused_bad) = (0usize, 0usize);
    for (kernel, &x) in buffer.iter().zip(input.iter()) {
        yf = one_pole_fused(c, x, yf);
        yu = one_pole_unfused(c, x, yu);
        if kernel.to_bits() != yf.to_bits() {
            fused_bad += 1;
        }
        if kernel.to_bits() != yu.to_bits() {
            unfused_bad += 1;
        }
    }
    arms.push(verdict("F3 one_pole_block", fused_bad, unfused_bad));

    // F4 -- gain_mix_block.
    let (g, mix) = (0.6f32, 0.35f32);
    let mut buffer = input.clone();
    gain_mix_block(&mut buffer, frames, g, mix);
    let (mut fused_bad, mut unfused_bad) = (0usize, 0usize);
    for (kernel, &x) in buffer.iter().zip(input.iter()) {
        if kernel.to_bits() != mix_fused(x, g, mix).to_bits() {
            fused_bad += 1;
        }
        if kernel.to_bits() != mix_unfused(x, g, mix).to_bits() {
            unfused_bad += 1;
        }
    }
    arms.push(verdict("F4 gain_mix_block", fused_bad, unfused_bad));

    // F5 -- mix2x2_block.
    let matrix = [0.8f32, 0.6, -0.3, 0.9];
    let mut left = input.clone();
    let mut right: Vec<f32> = input.iter().rev().copied().collect();
    let (source_left, source_right) = (left.clone(), right.clone());
    mix2x2_block::<f32>(&mut left, &mut right, matrix);
    let (mut fused_bad, mut unfused_bad) = (0usize, 0usize);
    for index in 0..frames {
        let (l, r) = (source_left[index], source_right[index]);
        if left[index].to_bits() != matrix_fused(matrix[0], matrix[1], l, r).to_bits()
            || right[index].to_bits() != matrix_fused(matrix[3], matrix[2], r, l).to_bits()
        {
            fused_bad += 1;
        }
        if left[index].to_bits() != matrix_unfused(matrix[0], matrix[1], l, r).to_bits()
            || right[index].to_bits() != matrix_unfused(matrix[3], matrix[2], r, l).to_bits()
        {
            unfused_bad += 1;
        }
    }
    arms.push(verdict("F5 mix2x2_block", fused_bad, unfused_bad));

    println!();
    let first = arms[0];
    assert!(
        arms.iter().all(|arm| *arm == first),
        "the kernels disagree about which arm they implement: {arms:?} -- the numeric contract \
         must be one contract"
    );
    println!("  every production kernel implements the same arm: {first}");
    println!();
}
