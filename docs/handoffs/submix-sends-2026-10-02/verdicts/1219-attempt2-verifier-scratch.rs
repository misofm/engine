//! Sol verifier scratch for #1219 attempt 2 (crates/lane/tests/zz_verifier.rs in a scratch copy). Not part of the tree under review.
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
}

/// Independent per-coefficient law, from (start, target, length) only.
fn law(start: [f32; 4], target: [f32; 4], length: u32, k: u32) -> [f32; 4] {
    let d: [f32; 4] = core::array::from_fn(|i| target[i] - start[i]);
    if length == 0 || d.iter().any(|x| !x.is_finite()) || k >= length {
        return target;
    }
    if k == 0 {
        return start;
    }
    core::array::from_fn(|i| {
        let s = d[i] / length as f32;
        let p = k as f32 * s;
        p + start[i]
    })
}

const UNIT: f64 = 1.0 / (1u128 << 100) as f64 / (1u64 << 50) as f64; // 2^-150

fn overshoot_units(start: f32, target: f32, length: u32) -> (f64, f64, u32, bool) {
    // returns (max overshoot in 2^-150 units, bound (length-1) units, k at max, nonmonotone below snap)
    let ramp = IndexedRamp::new([start, 0.0, 0.0, 0.0], [target, 0.0, 0.0, 0.0], length);
    let up = target >= start;
    let mut worst = 0.0f64;
    let mut at = 0;
    let mut prev = ramp.coefficients_at(0)[0];
    let mut nonmono = false;
    for k in 1..length {
        let c = ramp.coefficients_at(k)[0];
        let over = if up { c as f64 - target as f64 } else { target as f64 - c as f64 };
        if over > worst {
            worst = over;
            at = k;
        }
        if (up && c < prev) || (!up && c > prev) {
            nonmono = true;
        }
        prev = c;
    }
    (worst / UNIT, (length - 1) as f64, at, nonmono)
}

#[test]
fn v_constructed_subnormal_overshoots() {
    let _c = CanonicalFpEnv::enter();
    // L = 7: start = 2^-124, target = 2^-124 + 2^-147 (odd mantissa), step = 2^-149.
    let start = f32::from_bits((3u32) << 23); // exponent field 3 -> 2^-124
    let target = f32::from_bits(((3u32) << 23) | 1);
    assert_eq!(start as f64, 2f64.powi(-124));
    assert_eq!(target as f64 - start as f64, 2f64.powi(-147));
    let r = IndexedRamp::new([start, 0.0, 0.0, 0.0], [target, 0.0, 0.0, 0.0], 7);
    eprintln!("L=7 step bits {} ({}) is_normal {}", r.step[0].to_bits(), r.step[0] as f64 / UNIT, r.step[0].is_normal());
    let (o, b, k, nm) = overshoot_units(start, target, 7);
    eprintln!("L=7: overshoot {o} units of 2^-150 at k={k}, doc bound (L-1) = {b} units, nonmono<snap {nm}");

    // L = 2^22: target in [2^-105, 2^-104), start = target - 3*2^-128, step = 2^-148.
    let target = f32::from_bits((22u32 << 23) | 100); // 2^-105 * (1 + 100*2^-23)
    let start = f32::from_bits((22u32 << 23) | 97);
    assert_eq!(target as f64 - start as f64, 3.0 * 2f64.powi(-128));
    let length = INDEXED_RAMP_LENGTH_MAXIMUM;
    let r = IndexedRamp::new([start, 0.0, 0.0, 0.0], [target, 0.0, 0.0, 0.0], length);
    eprintln!("L=2^22 step {} units normal {}", r.step[0] as f64 / UNIT, r.step[0].is_normal());
    let (o, b, k, nm) = overshoot_units(start, target, length);
    eprintln!(
        "L=2^22: overshoot {o} units = 2^{:.4}, at k={k}; doc bound (L-1) = {b} units = 2^{:.4}; 2^-128 = {} units; nonmono<snap {nm}",
        (o * UNIT).log2(),
        (b * UNIT).log2(),
        2f64.powi(-128) / UNIT
    );
}

#[test]
fn v_random_subnormal_overshoot_ratio() {
    let _c = CanonicalFpEnv::enter();
    let mut rng = Rng(0xfeed_f00d);
    let mut worst_ratio = 0.0f64;
    let mut worst_abs = 0.0f64;
    let mut worst_case = (0.0f32, 0.0f32, 0u32);
    let mut over_bound = 0u64;
    let mut over_2m128 = 0u64;
    let mut ramps = 0u64;
    for _ in 0..4000 {
        // lengths, mostly small for speed, some large
        let length = match rng.next_u64() % 4 {
            0 => 2 + (rng.next_u64() % 64) as u32,
            1 => 2 + (rng.next_u64() % 5000) as u32,
            2 => 2 + (rng.next_u64() % 100_000) as u32,
            _ => INDEXED_RAMP_LENGTH_MAXIMUM - (rng.next_u64() % 8) as u32,
        };
        // target exponent field 0..=30 (subnormal up to ~2^-97), random mantissa
        let e = (rng.next_u64() % 31) as u32;
        let target = f32::from_bits((e << 23) | (rng.next_u64() as u32 & 0x7f_ffff));
        // difference in grid units of target's spacing, below length * 2^-126
        let spacing = if e == 0 { 1u64 } else { 1u64 << (e - 1) }; // in units of 2^-149
        let max_d_units = (length as f64 * 2f64.powi(-126) / 2f64.powi(-149)) as u64; // in 2^-149
        let max_j = (max_d_units / spacing).max(1);
        let j = 1 + rng.next_u64() % max_j.min(1 << 20);
        let start_bits = target.to_bits() as i64 - j as i64;
        if start_bits < 0 {
            continue;
        }
        let start = f32::from_bits(start_bits as u32);
        let (s, t) = if rng.next_u64() & 1 == 0 { (start, target) } else { (target, start) };
        let ramp = IndexedRamp::new([s, 0.0, 0.0, 0.0], [t, 0.0, 0.0, 0.0], length);
        if ramp.step[0] == 0.0 || ramp.step[0].is_normal() {
            continue;
        }
        if length > 200_000 && ramps % 8 != 0 {
            ramps += 1;
            continue;
        }
        ramps += 1;
        let (o, b, _, _) = overshoot_units(s, t, length);
        if o > b {
            over_bound += 1;
        }
        if o * UNIT >= 2f64.powi(-128) {
            over_2m128 += 1;
        }
        if b > 0.0 && o / b > worst_ratio {
            worst_ratio = o / b;
            worst_case = (s, t, length);
        }
        if o > worst_abs {
            worst_abs = o;
        }
    }
    eprintln!(
        "random subnormal-step ramps {ramps}: overshoot > (L-1)*2^-150 in {over_bound}; >= 2^-128 in {over_2m128}; worst ratio {worst_ratio:.4} at {:?} (bits {:#x} {:#x}); worst abs {} units = 2^{:.3}",
        worst_case,
        worst_case.0.to_bits(),
        worst_case.1.to_bits(),
        worst_abs,
        (worst_abs * UNIT).log2()
    );
}

fn full_ramp<L: Lane>(start: [f32; 4], target: [f32; 4], length: u32, frames: usize) -> u64 {
    let ramp = IndexedRamp::new(start, target, length);
    let mut rng = Rng(0x77 ^ frames as u64 ^ (length as u64) << 20);
    let mut position = 0u32;
    let mut mismatches = 0u64;
    let mut left = vec![0.0f32; frames];
    let mut right = vec![0.0f32; frames];
    let mut wl = vec![0.0f32; frames];
    let mut wr = vec![0.0f32; frames];
    let mut settled = 0;
    loop {
        for f in 0..frames {
            left[f] = (rng.unit() * 2.0 - 1.0) as f32;
            right[f] = (rng.unit() * 2.0 - 1.0) as f32;
        }
        wl.copy_from_slice(&left);
        wr.copy_from_slice(&right);
        route_mix_ramp_block::<L>(&mut left, &mut right, &ramp, position);
        for f in 0..frames {
            let k = position + f as u32 + 1;
            let [ll, lr, rl, rr] = law(start, target, length, k);
            let api = ramp.coefficients_at(k);
            if api.map(f32::to_bits) != [ll, lr, rl, rr].map(f32::to_bits) {
                mismatches += 1;
            }
            let (l, r) = (wl[f], wr[f]);
            let (pl, pr) = (ll * l, rl * l);
            let a = lr * r + pl;
            let b = rr * r + pr;
            if a.to_bits() != left[f].to_bits() || b.to_bits() != right[f].to_bits() {
                mismatches += 1;
            }
        }
        position = ramp.length.min(position + frames as u32);
        if position == ramp.length {
            settled += 1;
            if settled == 2 {
                break;
            }
        }
    }
    mismatches
}

#[test]
fn v_full_ramps_near_maximum_every_width() {
    let _c = CanonicalFpEnv::enter();
    let cases: [([f32; 4], [f32; 4]); 2] = [
        (
            [0.891_250_9, -0.3, 1.0e-3, 3.981_071_7],
            [-0.0, 0.707_106_77, -1.584_893_2, 1.0e-4],
        ),
        ([1.0, -1.0, 1.0e-6, 0.0], [0.999_999_9, 1.0, -7.0, 1.0e-30]),
    ];
    for length in [
        INDEXED_RAMP_LENGTH_MAXIMUM,
        INDEXED_RAMP_LENGTH_MAXIMUM - 1,
        INDEXED_RAMP_LENGTH_MAXIMUM - 3,
        4_000_037,
        48_000,
        4801,
        5,
    ] {
        for (ci, (s, t)) in cases.iter().enumerate() {
            for frames in [128usize, 125, 3] {
                lane::each_lane!(|L| {
                    let m = full_ramp::<L>(*s, *t, length, frames);
                    if m != 0 || length >= INDEXED_RAMP_LENGTH_MAXIMUM - 3 {
                        eprintln!("width {} length {length} case {ci} frames {frames}: mismatches {m}", L::WIDTH);
                    }
                    assert_eq!(m, 0);
                });
            }
        }
    }
}

#[cfg(not(debug_assertions))]
#[test]
fn v_unequal_planes_release_processes_common_prefix() {
    let _c = CanonicalFpEnv::enter();
    let ramp = IndexedRamp::new([0.5, 0.25, -0.125, 1.0], [-0.0, 0.3, 0.7, -2.0], 37);
    for (nl, nr) in [(130usize, 125usize), (125, 130), (7, 3), (3, 7), (0, 5), (5, 0)] {
        for position in [0u32, 30, 37] {
            lane::each_lane!(|L| {
                let l0: Vec<f32> = (0..nl).map(|f| f as f32 * 0.01 - 0.3).collect();
                let r0: Vec<f32> = (0..nr).map(|f| 0.2 - f as f32 * 0.003).collect();
                let (mut l, mut r) = (l0.clone(), r0.clone());
                route_mix_ramp_block::<L>(&mut l, &mut r, &ramp, position);
                let n = nl.min(nr);
                let (mut wl, mut wr) = (l0[..n].to_vec(), r0[..n].to_vec());
                route_mix_ramp_block::<f32>(&mut wl, &mut wr, &ramp, position);
                assert_eq!(l[..n].iter().map(|x| x.to_bits()).collect::<Vec<_>>(), wl.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
                assert_eq!(r[..n].iter().map(|x| x.to_bits()).collect::<Vec<_>>(), wr.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
                assert_eq!(&l[n..], &l0[n..], "left remainder untouched");
                assert_eq!(&r[n..], &r0[n..], "right remainder untouched");
            });
        }
    }
    let _ = mix2x2_block::<f32>;
}

// ---- mutation driver (python) ----
// import subprocess, sys, os
// B = "/tmp/claude-1002/v1219b"
// K = f"{B}/mut/crates/lane/src/kernels.rs"
// S = f"{B}/src/scripts/check-web-audioworklet-callgraph.py"
// orig = open(K).read()
// CUT = "    let count = core::cmp::min(left.len(), right.len());\n    let (left, right) = (&mut left[..count], &mut right[..count]);\n"
// TAILCUT = "    let count = core::cmp::min(left.len(), right.len());\n    mix2x2_block::<f32>(&mut left[..count], &mut right[..count], c);\n"
// M = {
//  "M12": [(CUT, "    let count = left.len();\n    let right = &mut right[..count];\n")],
//  "M13": [(TAILCUT, "    mix2x2_block::<f32>(left, right, c);\n")],
//  "S1-count-max": [(CUT, CUT.replace("core::cmp::min(left.len()", "core::cmp::max(left.len()"))],
//  "S2-tail-cut-right-to-left-len": [(TAILCUT, "    mix2x2_block::<f32>(left, &mut right[..left.len()], c);\n")],
//  "S3-tail-drops-last-frame": [(TAILCUT, TAILCUT.replace("let count = core::cmp::min(left.len(), right.len());", "let count = core::cmp::min(left.len(), right.len()).saturating_sub(1);"))],
//  "S4-planes-swapped-at-cut": [(CUT, CUT.replace("(&mut left[..count], &mut right[..count])", "(&mut right[..count], &mut left[..count])"))],
// }
// which = sys.argv[1:] or list(M)
// env = dict(os.environ)
// for name in which:
//     src = orig
//     for a, b in M[name]:
//         assert src.count(a) == 1, (name, src.count(a))
//         src = src.replace(a, b)
//     open(K, "w").write(src)
//     try:
//         e = dict(env, CARGO_TARGET_DIR=f"{B}/target-probe-mut", RUSTFLAGS="-C target-feature=+simd128")
//         p = subprocess.run(["cargo", "build", "--release", "--target", "wasm32-unknown-unknown"], cwd=f"{B}/probe-mut", env=e, capture_output=True, text=True)
//         if p.returncode != 0:
//             print("=====", name, "PROBE BUILD FAILED"); print(p.stderr[-2000:]); continue
//         dis = subprocess.run(["wasm-objdump", "-d", f"{B}/target-probe-mut/wasm32-unknown-unknown/release/probe.wasm"], capture_output=True, text=True).stdout
//         cg = subprocess.run(["python3", "-B", S, "--callgraph", "probe_ramp"], input=dis, capture_output=True, text=True)
//         sh = subprocess.run(["python3", "-B", f"{B}/shape.py", S, "4wide6f32x4"], input=dis, capture_output=True, text=True)
//         e2 = dict(env, CARGO_TARGET_DIR=f"{B}/target-mut")
//         t = subprocess.run(["cargo", "test", "--locked", "-p", "lane", "--test", "route_ramp"], cwd=f"{B}/mut", env=e2, capture_output=True, text=True)
//         out = t.stdout + t.stderr
//         tests = [l for l in out.splitlines() if l.startswith("test ") and ("FAILED" in l or " ok" in l)]
//         pan = [l for l in out.splitlines() if l.strip().startswith(("width", "case")) or "panicked" in l][:3]
//         print("=====", name)
//         print("  callgraph exit", cg.returncode, "|", (cg.stdout + cg.stderr).strip().replace("\n", " || ")[:600])
//         print("  shape:", sh.stdout.strip().splitlines()[0] if sh.stdout.strip() else sh.stderr[-300:])
//         print("  native route_ramp exit", t.returncode)
//         for l in tests: print("    ", l)
//         for l in pan: print("    >", l[:300])
//         sys.stdout.flush()
//     finally:
//         open(K, "w").write(orig)

// ---- shape.py (vector:scalar per 4wide6f32x4 function, via the gate's own kernel_arithmetic) ----
// import sys, importlib.util
// spec = importlib.util.spec_from_file_location("cg", sys.argv[1])
// cg = importlib.util.module_from_spec(spec); spec.loader.exec_module(cg)
// text = sys.stdin.read()
// fns = cg.parse(text)
// for f, v, s in cg.kernel_arithmetic(fns, sys.argv[2]):
//     print(f"vector={v} scalar={s} {f.name[:140]}")
// # also all functions containing route_mix, with scalar count
// for f in fns.values():
//     if 'route_mix' in f.name:
//         v = sum(1 for o in f.opcodes if cg.KERNEL_VECTOR.match(o)) if hasattr(cg,'KERNEL_VECTOR') else '?'
//         s = sum(1 for o in f.opcodes if cg.KERNEL_SCALAR.match(o))
//         print(f"  fn vector={v} scalar={s} {f.name[:140]}")

// ---- probe crate src/lib.rs (wasm32 +simd128, workspace release profile, own [workspace]) ----
// use lane::kernels::{IndexedRamp, route_mix_ramp_block};
// 
// 
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn probe_ramp(l: *mut f32, r: *mut f32, n: usize, ramp: *const IndexedRamp, position: u32) {
//     let left = unsafe { core::slice::from_raw_parts_mut(l, n) };
//     let right = unsafe { core::slice::from_raw_parts_mut(r, n) };
//     route_mix_ramp_block::<lane::Simd4>(left, right, unsafe { &*ramp }, position);
// }
// 
// /// Unequal plane lengths, to check no trap survives when lengths are not provably equal.
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn unequal_probe(l: *mut f32, nl: usize, r: *mut f32, nr: usize, ramp: *const IndexedRamp, position: u32) {
//     let left = unsafe { core::slice::from_raw_parts_mut(l, nl) };
//     let right = unsafe { core::slice::from_raw_parts_mut(r, nr) };
//     route_mix_ramp_block::<lane::Simd4>(left, right, unsafe { &*ramp }, position);
// }
