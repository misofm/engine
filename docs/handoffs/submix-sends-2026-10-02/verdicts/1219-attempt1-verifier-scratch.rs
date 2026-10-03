//! Verifier scratch for #1219 attempt 1. Not part of the tree under review.
#![allow(missing_docs, clippy::all)]

use lane::kernels::{INDEXED_RAMP_LENGTH_MAXIMUM, IndexedRamp, mix2x2_block, route_mix_ramp_block};
use lane::{CanonicalFpEnv, Lane};

struct Rng(u64);
impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn coefficient(&mut self) -> f32 {
        // log-uniform magnitude in [2^-20, 16), random sign, sometimes zero
        match self.next_u64() % 10 {
            0 => 0.0,
            1 => -0.0,
            _ => {
                let mag = (2.0f64).powf(self.unit() * 24.0 - 20.0) as f32;
                if self.next_u64() & 1 == 1 { -mag } else { mag }
            }
        }
    }
}

/// Overshoot and monotonicity of coefficients_at over a whole ramp. Returns (overshoots, nonmonotone, max_err_ulp_of_scale).
fn scan(ramp: &IndexedRamp) -> (u64, u64, f64) {
    let mut over = 0;
    let mut nonmono = 0;
    let mut max_err: f64 = 0.0;
    for i in 0..4 {
        let s = ramp.start[i];
        let t = ramp.target[i];
        let up = t >= s;
        let mut prev = ramp.coefficients_at(0)[i];
        let scale = s.abs().max(t.abs()) as f64;
        let ulp = if scale == 0.0 { 1.0 } else { (scale * f64::EPSILON * (1u64 << 29) as f64).max(f64::MIN_POSITIVE) };
        for k in 1..=ramp.length {
            let c = ramp.coefficients_at(k)[i];
            if k < ramp.length {
                if (up && c > t) || (!up && c < t) {
                    over += 1;
                }
                let exact = s as f64 + (t as f64 - s as f64) * (k as f64 / ramp.length as f64);
                let err = ((c as f64) - exact).abs() / ulp;
                if err > max_err {
                    max_err = err;
                }
            }
            if (up && c < prev) || (!up && c > prev) {
                nonmono += 1;
            }
            prev = c;
        }
    }
    (over, nonmono, max_err)
}

#[test]
fn v_subnormal_overshoot() {
    let _c = CanonicalFpEnv::enter();
    let t = f32::from_bits(5);
    let ramp = IndexedRamp::new([0.0, 0.0, 0.0, 0.0], [t, 0.0, 0.0, 0.0], 7);
    let cs: Vec<u32> = (0..=7).map(|k| ramp.coefficients_at(k)[0].to_bits()).collect();
    eprintln!("s=0 t=5*2^-149 L=7 step bits {} c(k) bits {:?}", ramp.step[0].to_bits(), cs);
    let ramp = IndexedRamp::new([1.0e-36, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], INDEXED_RAMP_LENGTH_MAXIMUM);
    let (o, n, e) = scan(&ramp);
    let last = ramp.coefficients_at(INDEXED_RAMP_LENGTH_MAXIMUM - 1)[0];
    eprintln!(
        "s=1e-36 t=0 L=2^22 step {:e} overshoots {o} nonmono {n} maxerr {e} c(L-1) {:e}",
        ramp.step[0], last
    );
    // Normal-range rough frequency of overshoot with tiny differences
    let mut rng = Rng(42);
    let mut total_over = 0;
    for _ in 0..200 {
        let s = (rng.unit() * 1e-33) as f32;
        let ramp = IndexedRamp::new([s, 0.0, 0.0, 0.0], [0.0; 4], 1 + (rng.next_u64() % 4_194_304) as u32);
        let mut o = 0;
        for k in 1..ramp.length {
            if ramp.coefficients_at(k)[0] < 0.0 {
                o += 1;
            }
        }
        total_over += o;
    }
    eprintln!("tiny-start ramps to 0 that go negative before snap: {total_over} frames");
}

#[test]
fn v_random_normal_range_sweep() {
    let _c = CanonicalFpEnv::enter();
    let mut rng = Rng(0xdead_beef);
    let mut worst = 0.0f64;
    let mut over_total = 0;
    let mut nonmono_total = 0;
    let lengths_fixed = [
        INDEXED_RAMP_LENGTH_MAXIMUM,
        INDEXED_RAMP_LENGTH_MAXIMUM - 1,
        INDEXED_RAMP_LENGTH_MAXIMUM - 3,
        3_000_001,
    ];
    for case in 0..48 {
        let start: [f32; 4] = core::array::from_fn(|_| rng.coefficient());
        let target: [f32; 4] = core::array::from_fn(|_| rng.coefficient());
        let length = if case < 16 {
            lengths_fixed[case % 4]
        } else {
            1 + (rng.next_u64() % 100_000) as u32
        };
        let ramp = IndexedRamp::new(start, target, length);
        let (o, n, e) = scan(&ramp);
        over_total += o;
        nonmono_total += n;
        if e > worst {
            worst = e;
        }
    }
    eprintln!("normal-range sweep: overshoot {over_total} nonmono {nonmono_total} worst err {worst:.3} ulp(scale)");
    assert_eq!(over_total, 0);
    assert_eq!(nonmono_total, 0);
}

fn full_ramp_width<L: Lane>(ramp: &IndexedRamp, frames: usize) -> Vec<u32> {
    // Inputs l = 1, r = 0 on even blocks, l = 0, r = 1 on odd: outputs then carry the coefficients
    // Instead, compare against a per-frame coefficients_at oracle with random planes.
    let mut rng = Rng(7 ^ frames as u64);
    let mut position = 0u32;
    let mut mismatches = Vec::new();
    let mut left = vec![0.0f32; frames];
    let mut right = vec![0.0f32; frames];
    let mut wl = vec![0.0f32; frames];
    let mut wr = vec![0.0f32; frames];
    loop {
        for f in 0..frames {
            left[f] = (rng.unit() * 2.0 - 1.0) as f32;
            right[f] = (rng.unit() * 2.0 - 1.0) as f32;
        }
        wl.copy_from_slice(&left);
        wr.copy_from_slice(&right);
        route_mix_ramp_block::<L>(&mut left, &mut right, ramp, position);
        for f in 0..frames {
            let [ll, lr, rl, rr] = ramp.coefficients_at(position + f as u32 + 1);
            let (l, r) = (wl[f], wr[f]);
            let a = lr * r + ll * l;
            let b = rr * r + rl * l;
            if a.to_bits() != left[f].to_bits() || b.to_bits() != right[f].to_bits() {
                mismatches.push(position + f as u32 + 1);
            }
        }
        if position == ramp.length {
            break;
        }
        position = ramp.length.min(position + frames as u32);
    }
    mismatches
}

#[test]
fn v_full_maximum_ramp_every_width() {
    let _c = CanonicalFpEnv::enter();
    let ramp = IndexedRamp::new(
        [0.891_250_9, -0.3, 1.0e-3, 3.981_071_7],
        [-0.0, 0.707_106_77, -1.584_893_2, 1.0e-4],
        INDEXED_RAMP_LENGTH_MAXIMUM - 1,
    );
    for frames in [128usize, 125, 3] {
        lane::each_lane!(|L| {
            let m = full_ramp_width::<L>(&ramp, frames);
            eprintln!("width {} frames {frames}: mismatches {}", L::WIDTH, m.len());
            assert!(m.is_empty(), "{:?}", &m[..m.len().min(8)]);
        });
    }
}

#[test]
fn v_settled_is_static_mix_and_position_past_length() {
    let _c = CanonicalFpEnv::enter();
    let ramp = IndexedRamp::new([0.5, 0.25, -0.125, 1.0], [-0.0, 0.3, 0.7, -2.0], 4800);
    for position in [4800u32, 4801, 1 << 24, u32::MAX] {
        for frames in [128usize, 125, 7, 1, 0] {
            lane::each_lane!(|L| {
                let mut l: Vec<f32> = (0..frames).map(|f| f as f32 * 0.01 - 0.3).collect();
                let mut r: Vec<f32> = (0..frames).map(|f| 0.2 - f as f32 * 0.003).collect();
                let (mut wl, mut wr) = (l.clone(), r.clone());
                route_mix_ramp_block::<L>(&mut l, &mut r, &ramp, position);
                mix2x2_block::<L>(&mut wl, &mut wr, ramp.target);
                assert_eq!(l.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), wl.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
                assert_eq!(r.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), wr.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
            });
        }
    }
}

#[test]
fn v_nonfinite_inputs() {
    let _c = CanonicalFpEnv::enter();
    for (s, t) in [
        (f32::NAN, 1.0f32),
        (1.0, f32::NAN),
        (f32::INFINITY, f32::INFINITY),
        (0.0, f32::INFINITY),
        (f32::MAX, -f32::MAX),
        (f32::MAX, 0.0),
        (-f32::MAX, f32::MAX * 0.5),
    ] {
        let ramp = IndexedRamp::new([s, 0.0, 0.0, 0.0], [t, 0.0, 0.0, 0.0], 4800);
        let finite_steps = ramp.step.iter().all(|x| x.is_finite());
        let mut nan_coeff = false;
        for k in 0..=4800 {
            let c = ramp.coefficients_at(k)[0];
            if c.is_nan() && !t.is_nan() && !(k == 0 && s.is_nan()) {
                nan_coeff = true;
            }
            if !c.is_finite() && s.is_finite() && t.is_finite() {
                nan_coeff = true;
            }
        }
        eprintln!("s={s:e} t={t:e}: length {} step finite {finite_steps} bad coeff {nan_coeff}", ramp.length);
        assert!(finite_steps);
        assert!(!nan_coeff);
    }
}

#[test]
fn v_endpoint_search_near_maximum() {
    let _c = CanonicalFpEnv::enter();
    let mut rng = Rng(0x1234_5678);
    let mut over = 0u64;
    let mut equal_before_snap = 0u64;
    let mut n = 0u64;
    for length in [INDEXED_RAMP_LENGTH_MAXIMUM, INDEXED_RAMP_LENGTH_MAXIMUM - 1, INDEXED_RAMP_LENGTH_MAXIMUM - 5, 3 * (1 << 20) + 1, 4_000_037] {
        for _ in 0..400_000 {
            // normal-range coefficients with |difference| >= 2^-100 so the step stays normal
            let s = f32::from_bits((rng.next_u64() as u32 & 0x807f_ffff) | ((100 + (rng.next_u64() % 31) as u32) << 23));
            let t = f32::from_bits((rng.next_u64() as u32 & 0x807f_ffff) | ((100 + (rng.next_u64() % 31) as u32) << 23));
            let ramp = IndexedRamp::new([s, t, -s, -t], [t, s, -t, -s], length);
            if ramp.length == 0 { continue; }
            if ramp.step.iter().any(|x| *x != 0.0 && !x.is_normal()) { continue; }
            n += 1;
            let c = ramp.coefficients_at(length - 1);
            for i in 0..4 {
                let (st, tg) = (ramp.start[i], ramp.target[i]);
                if (tg >= st && c[i] > tg) || (tg < st && c[i] < tg) { over += 1; }
                if c[i] == tg && st != tg { equal_before_snap += 1; }
            }
        }
    }
    eprintln!("endpoint search: {n} ramps x4, overshoot {over}, c(L-1)==target {equal_before_snap}");
    assert_eq!(over, 0);
}

// ---- mutation driver (python), applied to crates/lane/src/kernels.rs of a scratch copy ----
// import subprocess, sys, shutil, json
// K = "/tmp/claude-1002/v1219/mut/crates/lane/src/kernels.rs"
// orig = open(K).read()
// M = {
//  "V1-snap-vector-rounds-up": [(
//   "    let ramping = core::cmp::min(count, ramping as usize);\n    let vectored = ramping - ramping % L::WIDTH;",
//   "    let ramping = core::cmp::min(count, ramping as usize);\n    let ramping = core::cmp::min(count - count % L::WIDTH, ramping.div_ceil(L::WIDTH) * L::WIDTH).max(ramping);\n    let vectored = ramping - ramping % L::WIDTH;")],
//  "V2-fused-mul_add-in-body": [(
//   "        let ll = index.fma(step[0], start[0]);\n        let lr = index.fma(step[1], start[1]);\n        let rl = index.fma(step[2], start[2]);\n        let rr = index.fma(step[3], start[3]);",
//   "        let fused = |s: f32, c: f32| { let mut a = [0.0f32; 16]; index.store(&mut a[..L::WIDTH]); for x in &mut a[..L::WIDTH] { *x = x.mul_add(s, c); } L::load(&a[..L::WIDTH]) };\n        let ll = fused(ramp.step[0], ramp.start[0]);\n        let lr = fused(ramp.step[1], ramp.start[1]);\n        let rl = fused(ramp.step[2], ramp.start[2]);\n        let rr = fused(ramp.step[3], ramp.start[3]);")],
//  "V3-recursive-accumulation": [(
//   "    for (left, right) in left_vectors\n        .chunks_exact_mut(L::WIDTH)\n        .zip(right_vectors.chunks_exact_mut(L::WIDTH))\n    {\n        let ll = index.fma(step[0], start[0]);\n        let lr = index.fma(step[1], start[1]);\n        let rl = index.fma(step[2], start[2]);\n        let rr = index.fma(step[3], start[3]);",
//   "    let mut coef = [index.fma(step[0], start[0]), index.fma(step[1], start[1]), index.fma(step[2], start[2]), index.fma(step[3], start[3])];\n    let stride = step.map(|s| s.mul(advance));\n    for (left, right) in left_vectors\n        .chunks_exact_mut(L::WIDTH)\n        .zip(right_vectors.chunks_exact_mut(L::WIDTH))\n    {\n        let [ll, lr, rl, rr] = coef;\n        coef = [coef[0].add(stride[0]), coef[1].add(stride[1]), coef[2].add(stride[2]), coef[3].add(stride[3])];")],
//  "V4-iota-lane3-off-by-one": [(
//   "    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,",
//   "    1.0, 2.0, 3.0, 5.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,")],
//  "V5-advance-off-by-one-at-vector-boundary": [(
//   "    let advance = L::splat(L::WIDTH as f32);",
//   "    let advance = L::splat(if L::WIDTH > 1 { (L::WIDTH + 1) as f32 } else { 1.0 });")],
//  "V6-snap-one-frame-early": [(
//   "    let ramping = ramp.length.saturating_sub(position).saturating_sub(1);",
//   "    let ramping = ramp.length.saturating_sub(position).saturating_sub(2);")],
//  "V7-nonfinite-branch-is_nan-only": [(
//   "        if !difference.iter().all(|difference| difference.is_finite()) {",
//   "        if difference.iter().any(|difference| difference.is_nan()) {")],
//  "V8-lerp-law": [(
//   "        let ll = index.fma(step[0], start[0]);\n        let lr = index.fma(step[1], start[1]);\n        let rl = index.fma(step[2], start[2]);\n        let rr = index.fma(step[3], start[3]);",
//   "        let frac = index.div(L::splat(ramp.length as f32));\n        let tgt = ramp.target.map(L::splat);\n        let ll = frac.fma(tgt[0].sub(start[0]), start[0]);\n        let lr = frac.fma(tgt[1].sub(start[1]), start[1]);\n        let rl = frac.fma(tgt[2].sub(start[2]), start[2]);\n        let rr = frac.fma(tgt[3].sub(start[3]), start[3]);")],
//  "V9-ramp-all-scalar (perf only)": [(
//   "    let vectored = ramping - ramping % L::WIDTH;",
//   "    let vectored = 0 * (ramping - ramping % L::WIDTH);")],
//  "V10-settled-all-scalar (perf only)": [(
//   "    let settled_vectored = settled - settled % L::WIDTH;",
//   "    let settled_vectored = 0 * (settled - settled % L::WIDTH);")],
//  "V11-tail-uses-vector-start-k (k frozen in tail)": [(
//   "        let index = k as f32;\n        let ll = <f32 as Lane>::fma(index, step_ll, start_ll);",
//   "        let index = first as f32 + 0.0 * k as f32;\n        let ll = <f32 as Lane>::fma(index, step_ll, start_ll);")],
// }
// which = sys.argv[1:] or list(M)
// res = {}
// for name in which:
//     src = orig
//     for a, b in M[name]:
//         assert src.count(a) == 1, (name, a[:60], src.count(a))
//         src = src.replace(a, b)
//     open(K, "w").write(src)
//     try:
//         p = subprocess.run(["cargo", "test", "--locked", "-p", "lane", "--test", "route_ramp"], cwd="/tmp/claude-1002/v1219/mut",
//             env={**__import__('os').environ, "CARGO_TARGET_DIR": "/tmp/claude-1002/v1219/target-mut"}, capture_output=True, text=True, timeout=1800)
//         out = p.stdout + p.stderr
//         fails = [l for l in out.splitlines() if l.startswith("test ") and ("FAILED" in l or " ok" in l)]
//         panics = [l for l in out.splitlines() if "panicked" in l or l.strip().startswith("width") or "against" in l][:6]
//         res[name] = (p.returncode, fails, panics, out[-1500:] if p.returncode not in (0, 101) else "")
//     finally:
//         open(K, "w").write(orig)
//     print("=====", name, "exit", res[name][0]); [print("  ", l) for l in res[name][1]]; [print("   >", l[:300]) for l in res[name][2]]
//     if res[name][3]: print(res[name][3])
//     sys.stdout.flush()
// ---- wasm/aarch64/x86 probe crate src/lib.rs ----
// use lane::kernels::{IndexedRamp, route_mix_ramp_block};
// 
// 
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn probe_ramp(l: *mut f32, r: *mut f32, n: usize, ramp: *const IndexedRamp, position: u32) {
//     let left = unsafe { core::slice::from_raw_parts_mut(l, n) };
//     let right = unsafe { core::slice::from_raw_parts_mut(r, n) };
//     route_mix_ramp_block::<lane::Simd4>(left, right, unsafe { &*ramp }, position);
// }
