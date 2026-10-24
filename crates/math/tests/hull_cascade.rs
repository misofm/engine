//! Gates of `math::tail::hull` (issue #1372, tranche a): the common quadratic norm over a word
//! hull and the cascade bound built on it
//! (`docs/derivations/1372-parametric-eq-tail-and-rest.md`, part 1).
//!
//! Every hull here is built from TPT SVF designs at seeded random `(g, k)` (the parametric EQ's
//! designer form, `t = g (g + k)`, `c1 = t / (1 + t)`, `a2 = g (1 - c1)`, `a3 = g a2`, each word
//! rounded to `f32` once), never with the identity section. The simulations run in `f64` with
//! envelopes that carry no kernel rounding: they check the norm and the propagation, which is
//! what this tranche adds; the kernel's own rounding terms are the parent module's.

use math::tail::hull::{
    HullNorm, HullSection, common_lyapunov, hull_cascade, hull_cascade_composition,
    hull_cascade_crossing,
};
use math::tail::{EnvelopeSection, FlushLaw, TAIL_FLOOR, TailBoundError};

/// The engine's flush law at 48 kHz (`lane::FLUSH_EPS`, `lane::REST_EPS`,
/// `lane::silence_frames(48_000)`); `math` does not depend on `lane` in its default build.
const LAW: FlushLaw = FlushLaw {
    flush_eps: 1.0e-20,
    rest_eps: 1.0e-14,
    silence_frames: 4_096,
};

const RATES: [f64; 4] = [44_100.0, 48_000.0, 88_200.0, 96_000.0];

/// SplitMix64.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform in `[0, 1)`.
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1_u64 << 53) as f64
    }

    /// Log-uniform in `[low, high]`.
    fn log_uniform(&mut self, low: f64, high: f64) -> f64 {
        math::exp(math::log(low) + self.unit() * (math::log(high) - math::log(low)))
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// The `f32` design words `(c1, a2, a3)` of `(g, k)`, widened.
fn design(g: f64, k: f64) -> [f64; 3] {
    let t = g * (g + k);
    let c1 = t / (1.0 + t);
    let a2 = g * (1.0 - c1);
    let a3 = g * a2;
    [c1, a2, a3].map(|word| f64::from(word as f32))
}

/// `A(w)`.
fn step_matrix(w: [f64; 3]) -> [[f64; 2]; 2] {
    [
        [1.0 - 2.0 * w[0], -2.0 * w[1]],
        [2.0 * w[1], 1.0 - 2.0 * w[2]],
    ]
}

/// `||L x||_2`, directly.
fn q_norm(norm: &HullNorm, x: [f64; 2]) -> f64 {
    let first = x[0] + norm.l12() * x[1];
    let second = norm.l22() * x[1];
    math::sqrt(first * first + second * second)
}

/// `||A(w)||_Q` evaluated directly in `f64`: `L A L^-1` formed entry by entry, then the
/// largest singular value from the eigenvalues of `M^T M`. Independent of the module's closed
/// form.
fn direct_operator_norm(norm: &HullNorm, w: [f64; 3]) -> f64 {
    let a = step_matrix(w);
    let l = [[1.0, norm.l12()], [0.0, norm.l22()]];
    let inverse = [[1.0, -norm.l12() / norm.l22()], [0.0, 1.0 / norm.l22()]];
    let product = |x: [[f64; 2]; 2], y: [[f64; 2]; 2]| {
        let mut out = [[0.0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                out[i][j] = x[i][0] * y[0][j] + x[i][1] * y[1][j];
            }
        }
        out
    };
    let m = product(product(l, a), inverse);
    let p = m[0][0] * m[0][0] + m[1][0] * m[1][0];
    let r = m[0][1] * m[0][1] + m[1][1] * m[1][1];
    let s = m[0][0] * m[0][1] + m[1][0] * m[1][1];
    let largest = 0.5 * (p + r) + math::sqrt(0.25 * (p - r) * (p - r) + s * s);
    math::sqrt(largest)
}

/// A random hull: `count` designs in a random `(g, k)` rectangle at `rate`.
fn random_hull(rng: &mut Rng, rate: f64, count: usize, fast: bool) -> Vec<[f64; 3]> {
    let (g_low, g_high) = if fast {
        (0.02, 0.6)
    } else {
        (
            math::tan(core::f64::consts::PI * 10.0 / rate),
            math::tan(core::f64::consts::PI * 20_000.0 / rate),
        )
    };
    let (k_low, k_high) = (1.0 / (18.0 * 3.981), 10.0);
    let span = |rng: &mut Rng, low: f64, high: f64| {
        let a = rng.log_uniform(low, high);
        let b = rng.log_uniform(low, high);
        (a.min(b), a.max(b))
    };
    let (g0, g1) = span(rng, g_low, g_high);
    let (k0, k1) = span(rng, k_low, k_high);
    (0..count)
        .map(|_| design(rng.log_uniform(g0, g1), rng.log_uniform(k0, k1)))
        .collect()
}

/// A random point of the hull: a convex combination of up to three vertices.
fn hull_point(rng: &mut Rng, vertices: &[[f64; 3]]) -> [f64; 3] {
    let picks = 1 + rng.below(3);
    let weights: Vec<f64> = (0..picks).map(|_| rng.unit() + 1e-3).collect();
    let total: f64 = weights.iter().sum();
    let mut w = [0.0; 3];
    for weight in &weights {
        let vertex = vertices[rng.below(vertices.len())];
        for i in 0..3 {
            w[i] += weight / total * vertex[i];
        }
    }
    w
}

/// Gate a1 (the norm): for seeded random hulls with random word margins, the certified
/// contraction is at least the directly evaluated `||A(w)||_Q` at every vertex, at random hull
/// points moved within the margin, and along random switching trajectories; the conversion
/// constants bound the norm both ways.
#[test]
fn contraction_bounds_every_hull_word() {
    let mut rng = Rng(0x1372_a001);
    for hull in 0..48 {
        let rate = RATES[hull % 4];
        let count = 4 + rng.below(24);
        let vertices = random_hull(&mut rng, rate, count, false);
        let scale = vertices.iter().map(|w| w[1]).fold(0.0_f64, f64::max);
        let margin = [rng.unit(), rng.unit(), rng.unit()].map(|value| value * 1e-9 * scale);
        let certificate = match common_lyapunov(&vertices, margin) {
            Ok(certificate) => certificate,
            Err(error) => panic!("hull {hull}: no certificate ({error:?})"),
        };
        let (norm, q) = (certificate.norm, certificate.contraction);
        assert!(q < 1.0, "hull {hull}: q = {q}");
        let offset = |rng: &mut Rng, w: [f64; 3]| {
            core::array::from_fn::<f64, 3, _>(|i| w[i] + (2.0 * rng.unit() - 1.0) * margin[i])
        };
        for vertex in &vertices {
            let value = direct_operator_norm(&norm, *vertex);
            assert!(value <= q, "hull {hull}: vertex norm {value} above {q}");
        }
        for _ in 0..256 {
            let point = hull_point(&mut rng, &vertices);
            let w = offset(&mut rng, point);
            let value = direct_operator_norm(&norm, w);
            assert!(value <= q, "hull {hull}: hull point norm {value} above {q}");
        }
        for _ in 0..64 {
            let x = [2.0 * rng.unit() - 1.0, 2.0 * rng.unit() - 1.0];
            let plain = math::sqrt(x[0] * x[0] + x[1] * x[1]);
            let value = q_norm(&norm, x);
            assert!(value <= norm.norm(x), "hull {hull}: norm bound");
            assert!(
                value <= norm.from_words() * plain,
                "hull {hull}: from_words"
            );
            assert!(plain <= norm.to_words() * value, "hull {hull}: to_words");
        }
        let mut s = [1.0, -0.5];
        let start = q_norm(&norm, s);
        let mut envelope = start;
        for frame in 0..4_096 {
            let point = hull_point(&mut rng, &vertices);
            let w = offset(&mut rng, point);
            let a = step_matrix(w);
            s = [
                a[0][0] * s[0] + a[0][1] * s[1],
                a[1][0] * s[0] + a[1][1] * s[1],
            ];
            envelope *= q;
            let value = q_norm(&norm, s);
            assert!(
                value <= envelope * (1.0 + 1e-12),
                "hull {hull}: frame {frame}: {value} above {envelope}"
            );
        }
    }
}

/// One section's `f64` envelope over a hull with a fixed mix `(m0, m1, m2)`: `b`, `c` and `d` are
/// affine in the words, so their norms are largest at a vertex.
fn envelope(norm: &HullNorm, q: f64, vertices: &[[f64; 3]], mix: [f64; 3]) -> EnvelopeSection {
    let [m0, m1, m2] = mix;
    let mut input = 0.0_f64;
    let mut output_state = 0.0_f64;
    let mut output_input = 0.0_f64;
    for [c1, a2, a3] in vertices.iter().copied() {
        input = input.max(norm.norm([2.0 * a2, 2.0 * a3]));
        let c = [m1 * (1.0 - c1) + m2 * a2, -m1 * a2 + m2 * (1.0 - a3)];
        output_state = output_state.max(norm.dual_norm(c));
        output_input = output_input.max((m0 + m1 * a2 + m2 * a3).abs() * (1.0 + 1e-15));
    }
    EnvelopeSection {
        rho_ramp: q,
        rho_settled: q,
        input,
        output_state,
        output_input,
    }
}

/// A random mix: a bell (`m0 = 1`, `m1 = k (A^2 - 1)`), a low-pass or a high-pass row.
fn random_mix(rng: &mut Rng) -> [f64; 3] {
    match rng.below(3) {
        0 => [1.0, 0.5 * (rng.log_uniform(0.0631, 15.85) - 1.0), 0.0],
        1 => [0.0, 0.0, 1.0],
        _ => [1.0, -1.0, -1.0],
    }
}

/// One step of a section in `f64`: `(state, input) -> (state', output)`.
fn section_step(w: [f64; 3], mix: [f64; 3], s: &mut [f64; 2], x: f64) -> f64 {
    let [c1, a2, a3] = w;
    let v3 = x - s[1];
    let d1 = -c1 * s[0] + a2 * v3;
    let d2 = a3 * v3 + a2 * s[0];
    let (v1, v2) = (s[0] + d1, s[1] + d2);
    *s = [s[0] + 2.0 * d1, s[1] + 2.0 * d2];
    mix[0] * x + mix[1] * v1 + mix[2] * v2
}

/// `S(j) = sum_{m >= j} |h[m]|` of a fixed cascade's impulse response, as a function of `j`,
/// summed until the remaining state is negligible.
fn suffix_sums(words: &[[f64; 3]], mixes: &[[f64; 3]], frames: usize) -> Vec<f64> {
    let mut states = vec![[0.0; 2]; words.len()];
    let mut h = Vec::with_capacity(frames);
    for frame in 0..frames {
        let mut x = if frame == 0 { 1.0 } else { 0.0 };
        for (i, state) in states.iter_mut().enumerate() {
            x = section_step(words[i], mixes[i], state, x);
        }
        h.push(x.abs());
    }
    let mut suffix = vec![0.0; frames + 1];
    for j in (0..frames).rev() {
        suffix[j] = suffix[j + 1] + h[j];
    }
    suffix
}

/// A random cascade of one to three hull sections at `rate`, with its norm, envelopes, vertices
/// and mixes. A third of the sections are one-vertex hulls, where the bound is tightest.
#[allow(clippy::type_complexity)]
fn random_cascade(
    rng: &mut Rng,
    rate: f64,
) -> (
    HullNorm,
    Vec<HullSection>,
    Vec<Vec<[f64; 3]>>,
    Vec<[f64; 3]>,
) {
    let count = 1 + rng.below(3);
    let hulls: Vec<Vec<[f64; 3]>> = (0..count)
        .map(|_| {
            let size = if rng.below(3) == 0 {
                1
            } else {
                2 + rng.below(6)
            };
            random_hull(rng, rate, size, true)
        })
        .collect();
    let every: Vec<[f64; 3]> = hulls.iter().flatten().copied().collect();
    let certificate = common_lyapunov(&every, [0.0; 3]).expect("a certificate");
    let (norm, q) = (certificate.norm, certificate.contraction);
    let mixes: Vec<[f64; 3]> = (0..count).map(|_| random_mix(rng)).collect();
    let sections = hulls
        .iter()
        .zip(&mixes)
        .map(|(vertices, mix)| HullSection {
            envelope: envelope(&norm, q, vertices, *mix),
            identity_reachable: false,
        })
        .collect();
    (norm, sections, hulls, mixes)
}

/// Gate a1 (the cascade): for seeded random cascades of hull sections, every fixed member of the
/// hulls (an admitted history) has its worst-sign output `S(m + 1)` at `N + m` below the
/// bound's level from the bound's crossings on, for every decade checked; random switching
/// histories with worst-sign-like inputs stay below too; and a section whose words can reach the
/// identity is refused.
#[test]
fn cascade_bound_covers_every_hull_member() {
    let mut rng = Rng(0x1372_a002);
    for cascade in 0..32 {
        let rate = RATES[cascade % 4];
        let (norm, sections, hulls, mixes) = random_cascade(&mut rng, rate);
        let crossings: Vec<u64> = (0..4)
            .map(|decades| {
                hull_cascade_crossing(&sections, &norm, 1.0, &LAW, 0, decades).expect("a crossing")
            })
            .collect();
        let frames = (crossings[3] as usize + 2).max(16) * 4;
        for _ in 0..4 {
            let words: Vec<[f64; 3]> = hulls.iter().map(|h| hull_point(&mut rng, h)).collect();
            let suffix = suffix_sums(&words, &mixes, frames);
            for (decades, crossing) in crossings.iter().enumerate() {
                let limit = TAIL_FLOOR / 2.0 * math::pow(10.0, -(decades as f64));
                let index = (*crossing as usize + 1).min(frames);
                assert!(
                    suffix[index] < limit,
                    "cascade {cascade}: decade {decades}: S({index}) = {} not below {limit}",
                    suffix[index]
                );
            }
        }
        // Switching: words drawn afresh every frame, inputs of random sign at full scale.
        let mut states = vec![[0.0; 2]; sections.len()];
        let loud = frames;
        for frame in 0..loud + frames {
            let mut x = if frame < loud {
                if rng.below(2) == 0 { 1.0 } else { -1.0 }
            } else {
                0.0
            };
            for (i, state) in states.iter_mut().enumerate() {
                x = section_step(hull_point(&mut rng, &hulls[i]), mixes[i], state, x);
            }
            if frame >= loud + crossings[0] as usize {
                assert!(
                    x.abs() < TAIL_FLOOR / 2.0,
                    "cascade {cascade}: switching output"
                );
            }
        }
        let mut refused = sections.clone();
        refused[0].identity_reachable = true;
        assert_eq!(
            hull_cascade(&refused, &norm, 1.0, &LAW, 64, [15.85, 1e29]),
            Err(TailBoundError::IdentityReachable)
        );
    }
}

/// The parametric EQ's slowest design: a bell at 10 Hz, Q 18, +24 dB at 48 kHz.
fn extreme() -> ([f64; 3], [f64; 3]) {
    let amplitude = math::pow(10.0, 24.0 / 40.0);
    let g = math::tan(core::f64::consts::PI * 10.0 / 48_000.0);
    let k = 1.0 / (18.0 * amplitude);
    let mix = [1.0, k * (amplitude * amplitude - 1.0), 0.0];
    (design(g, k), mix.map(|word| f64::from(word as f32)))
}

/// Gate a2 (tightness): a one-vertex hull at the EQ's extreme. Its certified `T` is sound against
/// the brute-force `T_b(eps / 2)` of the design's own impulse response and within the factor 2 of
/// it.
#[test]
fn one_vertex_tail_is_tight() {
    let (words, mix) = extreme();
    let certificate = common_lyapunov(&[words], [0.0; 3]).expect("a certificate");
    let norm = certificate.norm;
    let sections = [HullSection {
        envelope: envelope(&norm, certificate.contraction, &[words], mix),
        identity_reachable: false,
    }];
    let bound = hull_cascade(&sections, &norm, 1.0, &LAW, 64, [15.85, 1e29]).expect("a bound");
    let frames = 2 * bound.tail as usize + 1_000;
    let suffix = suffix_sums(&[words], &[mix], frames);
    let brute = suffix
        .iter()
        .position(|value| *value < TAIL_FLOOR / 2.0)
        .expect("a crossing") as u64;
    let ratio = bound.tail as f64 / brute as f64;
    eprintln!(
        "one-vertex extreme at 48 kHz: T = {}, T_b = {brute}, ratio {ratio:.4}, q = {}",
        bound.tail, certificate.contraction
    );
    assert!(
        brute <= bound.tail + 1,
        "T = {} below T_b = {brute}",
        bound.tail
    );
    assert!(
        ratio <= 2.0,
        "T = {} above 2 T_b = {}",
        bound.tail,
        2 * brute
    );
}

/// The composition's decay certificate covers the bound's own crossings: for every decade `d`
/// checked, `T + crossing(d)` and `T + d D` are at or after the crossing searched directly.
#[test]
fn composition_covers_crossings() {
    let mut rng = Rng(0x1372_a004);
    for cascade in 0..16 {
        let rate = RATES[cascade % 4];
        let (norm, sections, _, _) = random_cascade(&mut rng, rate);
        let ramp = [0, 64][cascade % 2];
        let composition = hull_cascade_composition(&sections, &norm, 1.0, &LAW, ramp)
            .expect("a bound")
            .expect("certified values");
        let tail = hull_cascade_crossing(&sections, &norm, 1.0, &LAW, ramp, 0).expect("T");
        for decades in 1..=6 {
            let direct =
                hull_cascade_crossing(&sections, &norm, 1.0, &LAW, ramp, decades).expect("T(k)");
            let certified = tail + composition.crossing(decades);
            assert!(
                certified >= direct,
                "cascade {cascade}: decade {decades}: {certified} before {direct}"
            );
            assert!(tail + decades * composition.decay >= direct);
        }
    }
}
