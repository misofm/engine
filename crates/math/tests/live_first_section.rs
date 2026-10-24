//! Brute-force gate of `math::tail::live_first_section` (issue #1373, tranche a): a row on the
//! first section's state of a live cascade, here the multiband crossover's all-pass tap
//! `ap1 = fl(fl(-2k v1) + x)` (`docs/derivations/1373-multiband-compressor-tail-and-rest.md`).
//!
//! The test builds the first section's live envelope over a crossover domain of 80 Hz to 8 kHz
//! at 48 kHz with a 64-frame iterated linear ramp, renders the `f32` kernel (the stored-form TPT
//! SVF with the per-word and joint flush, the silence counter and the all-pass tap, in the
//! engine's operation order) under worst-sign inputs, and holds every stated value against it.
//! The envelope covers the test's own histories (one glide from a settled design), and the run
//! checks that premise on every word it loads.

use math::tail::{
    EnvelopeSection, FirstSectionRow, FlushLaw, LiveCascade, PoleDomain, SvfWords, TAIL_FLOOR,
    live_first_section, live_first_section_composition, output_rounding, state_rounding,
    v_dual_norm, v_norm, word_box_norm,
};

const RATE: f64 = 48_000.0;
const LOW_HZ: f64 = 80.0;
const HIGH_HZ: f64 = 8_000.0;
const RAMP: u32 = 64;
/// `2^-24`.
const U: f64 = 1.0 / 16_777_216.0;
/// The engine's flush law at 48 kHz (`lane::FLUSH_EPS`, `lane::REST_EPS`,
/// `lane::silence_frames(48_000)`); `math` does not depend on `lane` in its default build.
const FLUSH_EPS: f32 = 1.0e-20;
const REST_EPS: f32 = 1.0e-14;
const SILENCE: u32 = 4_096;
/// Frames of worst-sign input before `N`: the 80 Hz impulse response falls below `1e-120` of
/// its peak by then, far below anything an `f32` sum can carry.
const LOUD: usize = 40_000;

/// `g = tan(pi f / fs)`.
fn prewarp(hz: f64) -> f64 {
    math::tan(core::f64::consts::PI * hz / RATE)
}

/// The crossover's `f32` design words `(c1, a2, a3)` (`multiband_compressor::design_lr4_words`).
fn design(hz: f64) -> [f32; 3] {
    let g = prewarp(hz);
    let t = g * (g + core::f64::consts::SQRT_2);
    let c1 = t / (1.0 + t);
    let a1 = 1.0 - c1;
    [c1 as f32, (g * a1) as f32, (g * (g * a1)) as f32]
}

/// The exact design's words in `f64`.
fn exact(g: f64) -> [f64; 3] {
    let t = g * (g + core::f64::consts::SQRT_2);
    let c1 = t / (1.0 + t);
    [c1, g * (1.0 - c1), g * g * (1.0 - c1)]
}

/// `-2k` as the kernel's `f32` word.
fn tap_word() -> f32 {
    (-2.0 * core::f64::consts::SQRT_2) as f32
}

/// The test's word allowances: the half-ulp box `h` of an `f32` design, and the ramp box: a
/// 64-step iterated ramp `w = fl(w + s)`, `s = fl(t - c) / 64`, from one `f32` design to another
/// lies within `64 h + 2 u |t - c|` of their chord, which lies within `h` of the exact designs'.
fn boxes() -> ([f64; 3], [f64; 3], [f64; 3]) {
    let half_ulp = [U / 2.0, U / 4.0, U / 2.0];
    let range = exact(prewarp(HIGH_HZ));
    let chord: [f64; 3] = core::array::from_fn(|i| 64.0 * half_ulp[i] + 2.0 * U * range[i]);
    let ramp: [f64; 3] = core::array::from_fn(|i| chord[i] + half_ulp[i]);
    (half_ulp, chord, ramp)
}

/// The first section's live envelope over the domain, and the all-pass tap's row.
fn envelope() -> (LiveCascade, FirstSectionRow) {
    let sqrt2 = core::f64::consts::SQRT_2;
    let (g_min, g_max) = (prewarp(LOW_HZ), prewarp(HIGH_HZ));
    let (half_ulp, _, ramp_box) = boxes();
    // Every word magnitude is largest at the top design (`g < 1` on the whole domain).
    let largest = exact(g_max);
    // The pole radius `sqrt(1 + g^4) / (1 + sqrt(2) g + g^2)` falls on `g < 1`: largest at 80 Hz.
    let radius = math::sqrt(1.0 + g_min.powi(4)) / (1.0 + sqrt2 * g_min + g_min * g_min);
    let beta = 2.0 * g_max / math::sqrt(1.0 + g_max * (g_max + sqrt2));
    let rounding = |extra: [f64; 3], q: f64| {
        state_rounding(core::array::from_fn(|i| largest[i] + extra[i]), q, 2.0)
    };
    let q_ramp = radius + word_box_norm(ramp_box);
    let q_settled = radius + word_box_norm(half_ulp);
    let (mu_ramp, mu_input) = rounding(ramp_box, q_ramp);
    let (mu_settled, _) = rounding(half_ulp, q_settled);
    let mut column = 0.0_f64;
    for signs in 0..4_u32 {
        let sign = |bit: u32| if signs & bit == 0 { 1.0 } else { -1.0 };
        column = column.max(v_norm([
            2.0 * ramp_box[1] * sign(1),
            2.0 * ramp_box[2] * sign(2),
        ]));
    }
    let real_pole = |g: f64| (1.0 - g * g) / (1.0 + sqrt2 * g + g * g);
    let live = LiveCascade {
        envelope: EnvelopeSection {
            rho_ramp: q_ramp + mu_ramp,
            rho_settled: q_settled + mu_settled,
            input: beta + column + mu_input,
            // The second section's quantities: the bound of a first-section row reads none.
            output_state: 2.0,
            output_input: 1.0,
        },
        poles: PoleDomain {
            low: real_pole(g_max) - 1.0e-12,
            high: real_pole(g_min) + 1.0e-12,
            ramp_box,
            design_box: half_ulp,
            state_rounding: mu_ramp,
            settled_state_rounding: mu_settled,
            input_rounding: mu_input,
        },
        first_mix_row: 2.0,
        first_output_rounding: 0.0,
        second_mix_row: 2.0,
        second_output_rounding: 0.0,
    };
    // The tap's mix `(m0, m1, m2) = (1, fl(-2k), 0)`: its output rounding at the largest words
    // plus at the corner where `c1` and `a3` are `-box` (the `|1 - c1|`, `|1 - a3|` terms), and
    // `|d| = |1 + m1 a2| <= 1 + |m1| box` (`m1 < 0`, `a2 >= -box`).
    let m1 = f64::from(tap_word());
    let top = SvfWords {
        c1: largest[0] + ramp_box[0],
        a2: largest[1] + ramp_box[1],
        a3: largest[2] + ramp_box[2],
        m0: 1.0,
        m1,
        m2: 0.0,
    };
    let corner = SvfWords {
        c1: -ramp_box[0],
        a3: -ramp_box[2],
        ..top
    };
    let (omega_top, omega_input) = output_rounding(&top);
    let row = FirstSectionRow {
        mix_row: v_dual_norm([m1, 0.0]) * (1.0 + 1.0 / 1_073_741_824.0),
        output_rounding: omega_top + output_rounding(&corner).0,
        feedthrough: 1.0 + m1.abs() * ramp_box[1] + omega_input,
    };
    (live, row)
}

fn law() -> FlushLaw {
    FlushLaw {
        flush_eps: f64::from(FLUSH_EPS),
        rest_eps: f64::from(REST_EPS),
        silence_frames: u64::from(SILENCE),
    }
}

/// The kernel's first section, its silence counter and its crossover word ramp.
struct Kernel {
    state: [f32; 2],
    run: u32,
    words: [f32; 3],
    /// The ramp: target, step, frames left (`effect_runtime::ramp::LinearRamp`).
    target: [f32; 3],
    step: [f32; 3],
    remaining: u32,
    /// The glide's start and how many ramp words it has produced, for the premise check.
    from: [f32; 3],
    produced: u32,
}

impl Kernel {
    fn new(words: [f32; 3]) -> Self {
        Self {
            state: [0.0; 2],
            run: 0,
            words,
            target: words,
            step: [0.0; 3],
            remaining: 0,
            from: words,
            produced: 0,
        }
    }

    /// `LinearRamp::set_target`: `step = (target - current) / 64`.
    fn glide(&mut self, target: [f32; 3]) {
        self.from = self.words;
        self.produced = 0;
        self.target = target;
        self.step = core::array::from_fn(|i| (target[i] - self.words[i]) / RAMP as f32);
        self.remaining = RAMP;
    }

    /// `LinearRamp::next_value` per word: `current + step` held between `current` and the target
    /// (#1409), the target assigned on the last sample.
    fn advance(&mut self) {
        if self.remaining > 0 {
            self.produced += 1;
        }
        match self.remaining {
            0 => {}
            1 => {
                self.words = self.target;
                self.remaining = 0;
            }
            _ => {
                for i in 0..3 {
                    let (current, target) = (self.words[i], self.target[i]);
                    let next = current + self.step[i];
                    self.words[i] = next.clamp(current.min(target), current.max(target));
                }
                self.remaining -= 1;
            }
        }
    }

    /// One frame: the words in force, the silence counter, `svf_step_when(true, ..)` and the
    /// all-pass tap, in the engine's order (`lane::kernels::svf_step_when`, `lane::flush_pair`,
    /// `multiband_compressor::lr4_step`). `fma` is two roundings (`lane::Lane::fma`).
    fn frame(&mut self, x: f32) -> f32 {
        self.advance();
        self.run = if x == 0.0 { self.run + 1 } else { 0 };
        let rest = if self.run < SILENCE { 0.0 } else { REST_EPS };
        let [c1, a2, a3] = self.words;
        let [ic1, ic2] = self.state;
        let v3 = x - ic2;
        let d1 = -c1 * ic1 + a2 * v3;
        let v1 = ic1 + d1;
        let d2 = a3 * v3 + a2 * ic1;
        let (n1, n2) = (ic1 + (d1 + d1), ic2 + (d2 + d2));
        let joint = n1.abs().max(n2.abs()) < rest;
        let flush = |n: f32| {
            if n.abs() < FLUSH_EPS || joint { 0.0 } else { n }
        };
        self.state = [flush(n1), flush(n2)];
        tap_word() * v1 + x
    }
}

/// The `f64` impulse response of the all-pass tap at fixed words, `h[0] = d`, `h[j] = c A^(j-1) b`.
fn impulse(words: [f32; 3], frames: usize) -> Vec<f64> {
    let w = SvfWords::from_f32([words[0], words[1], words[2], 1.0, tap_word(), 0.0]);
    let (a, b, c) = (w.a(), w.b(), w.c());
    let mut response = vec![w.d()];
    let mut s = b;
    for _ in 1..frames {
        response.push(c[0] * s[0] + c[1] * s[1]);
        s = [
            a[0][0] * s[0] + a[0][1] * s[1],
            a[1][0] * s[0] + a[1][1] * s[1],
        ];
    }
    response
}

/// What one run of the kernel showed after `N`.
struct Run {
    /// The tap's largest magnitude over the whole run.
    peak: f64,
    /// The tap after `N`, per frame.
    after: Vec<f32>,
    /// The first frame (from `N`) from which the state entering each frame is `+0.0` in both
    /// words and the tap is `0` (`None` when the run never rests).
    rested: Option<usize>,
}

/// A run: settled at `start`, a glide toward `target` beginning `lead` frames before `N` (when
/// `target` is given), the worst-sign input `P sign(h[T + 1 + i])` at `N - 1 - i` for the 80 Hz
/// response `h`, then `after` frames of zeros. Every ramp word is checked against the test's box.
fn run(
    start: [f32; 3],
    target: Option<[f32; 3]>,
    lead: usize,
    signs: &[f64],
    peak: f32,
    after: usize,
) -> Run {
    let (_, chord, _) = boxes();
    let mut kernel = Kernel::new(start);
    let mut largest = 0.0_f64;
    // The `j`-th word of a glide (`j` = `produced`, the target from `j = 64` on) against the
    // chord point `(1 - j/64) from + (j/64) target`; without a glide `from` is the target.
    let check_word = |kernel: &Kernel| {
        let alpha = f64::from(kernel.produced) / f64::from(RAMP);
        for i in 0..3 {
            let line =
                (1.0 - alpha) * f64::from(kernel.from[i]) + alpha * f64::from(kernel.target[i]);
            let off = (f64::from(kernel.words[i]) - line).abs();
            assert!(off <= chord[i], "word {i} is {off:e} off its chord");
        }
    };
    for n in 0..LOUD {
        if target.is_some() && n + lead == LOUD {
            kernel.glide(target.expect("a target"));
        }
        let sign = if signs[LOUD - 1 - n] < 0.0 { -1.0 } else { 1.0 };
        let y = kernel.frame(sign * peak);
        check_word(&kernel);
        largest = largest.max(f64::from(y.abs()));
    }
    let mut tail = Vec::with_capacity(after);
    let mut rested = None;
    for n in 0..after {
        let at_rest = kernel.state.iter().all(|word| word.to_bits() == 0);
        let y = kernel.frame(0.0);
        check_word(&kernel);
        largest = largest.max(f64::from(y.abs()));
        tail.push(y);
        if at_rest && y == 0.0 {
            rested.get_or_insert(n);
        } else {
            rested = None;
        }
    }
    Run {
        peak: largest,
        after: tail,
        rested,
    }
}

/// Issue #1373 tranche a: the first-section row bound holds on the `f32` kernel's all-pass tap,
/// settled at the slowest design and with a glide 8 kHz -> 80 Hz in flight at `N`, for
/// `P = 10^(24/20)` and `1e29`: `|ap1| < P eps` from `N + T`; `|ap1| < P (eps / 2) 10^-k + sigma_t`
/// from `N + T + k D`; the state and the tap exactly zero from `N + R(P)`; `|ap1| <= G_p P +
/// sigma_p` over the whole run; and for a peak below `P*`, exactly zero from
/// `N + T_rest`. A late input from a zero state, glide in flight, stays within `G_t`.
#[test]
fn the_first_section_row_bound_holds_on_the_kernel_all_pass_tap() {
    let (live, row) = envelope();
    let law = law();
    let (low, high) = (design(LOW_HZ), design(HIGH_HZ));
    for words in [low, high] {
        let re = math::tail::pole_real(&SvfWords::from_f32([
            words[0], words[1], words[2], 0.0, 0.0, 0.0,
        ]));
        assert!(
            live.poles.low < re && re < live.poles.high,
            "{words:?} in the domain"
        );
    }
    let peaks = [10.0_f64.powf(24.0 / 20.0), 1.0e29];
    let bound = live_first_section(&live, &row, 1.0, &law, u64::from(RAMP), peaks)
        .expect("the tap is bounded");
    let composition = live_first_section_composition(&live, &row, 1.0, &law, u64::from(RAMP))
        .expect("the tap is bounded")
        .expect("the composition is certified");
    let tail = bound.tail as usize;
    let decay = composition.decay as usize;
    let response = impulse(low, tail + 1 + LOUD);
    let signs = &response[tail + 1..];
    let horizon = (bound.rest_any as usize).max(tail + 3 * decay) + 64;
    for (peak, rest) in peaks.into_iter().zip([bound.rest_peak, bound.rest_any]) {
        let level = peak as f32;
        for (name, target) in [("settled", None), ("glide", Some(low))] {
            let start = if target.is_some() { high } else { low };
            let result = run(start, target, 32, signs, level, horizon);
            let p = f64::from(level);
            let last_loud = result
                .after
                .iter()
                .rposition(|y| f64::from(y.abs()) >= p * TAIL_FLOOR);
            assert!(
                last_loud.is_none_or(|frame| frame < tail),
                "{name}, P = {p:e}: |ap1| >= P eps at N + {last_loud:?}, T = {tail}"
            );
            for k in 1..=3_usize {
                let line =
                    p * TAIL_FLOOR / 2.0 * 10.0_f64.powi(-(k as i32)) + composition.tail_stall;
                let from = tail + composition.crossing(k as u64) as usize;
                assert!(from <= tail + k * decay, "the crossing lies within T + k D");
                let late = result.after[from..]
                    .iter()
                    .position(|y| f64::from(y.abs()) >= line);
                assert!(
                    late.is_none(),
                    "{name}, P = {p:e}, k = {k}: at N + {from} + {late:?}"
                );
            }
            let rested = result.rested.expect("the run rests");
            assert!(
                rested as u64 <= rest,
                "{name}, P = {p:e}: at rest from N + {rested}, R = {rest}"
            );
            let ceiling = composition.peak_gain * p + composition.peak_stall;
            assert!(
                result.peak <= ceiling,
                "{name}, P = {p:e}: peak {:e} above G_p P + sigma_p = {ceiling:e}",
                result.peak
            );
        }
    }

    // Below the flush floor the claim is exact rest from `N + T_rest`, and from `N + R(P)` at the
    // peak itself. Its state starts below `REST_EPS`, so only the per-word flush or the joint
    // flush after `N_SILENCE` can rest it.
    let quiet = 1.0e-15_f32;
    assert!(f64::from(quiet) < bound.flush_floor);
    let quiet_rest = live_first_section(&live, &row, 1.0, &law, u64::from(RAMP), [1.0e-15; 2])
        .expect("the tap is bounded")
        .rest_peak;
    for (name, target) in [("settled", None), ("glide", Some(low))] {
        let start = if target.is_some() { high } else { low };
        let result = run(start, target, 32, signs, quiet, horizon);
        let rested = result.rested.expect("the run rests") as u64;
        assert!(
            rested <= quiet_rest && rested <= bound.tail_every_peak,
            "{name}, P = {quiet:e}: at rest from N + {rested}, R = {quiet_rest}, T_rest = {}",
            bound.tail_every_peak
        );
    }

    // `G_t`: a zero state at `M`, the glide 8 kHz -> 80 Hz in flight, then a unit worst-sign
    // input for the tap's response at 80 Hz, aimed at `M + 4096`.
    let aim = 4_096;
    let mut kernel = Kernel::new(high);
    kernel.glide(low);
    for _ in 0..32 {
        kernel.frame(0.0);
    }
    let late = impulse(low, aim + 1);
    let mut largest = 0.0_f64;
    for n in 0..2 * aim {
        let x = if n <= aim {
            if late[aim - n] < 0.0 { -1.0 } else { 1.0 }
        } else {
            0.0
        };
        largest = largest.max(f64::from(kernel.frame(x).abs()));
    }
    let ceiling = composition.tail_gain + composition.peak_stall;
    assert!(
        largest <= ceiling,
        "late peak {largest:e} above G_t + sigma_p = {ceiling:e}"
    );
}
