#![allow(static_mut_refs, clippy::all)]
use lane::{Lane, Simd4};

const N: usize = 1 << 16;
static mut BUF: [f32; N] = [0.0; N];
static mut OUT: [f32; N] = [0.0; N];

#[inline(always)]
fn meter_sample_peak_block<L: Lane>(words: &[f32], frames: usize, peak: L) -> L {
    let mut peak = peak;
    for f in 0..frames {
        let a = L::load(&words[f * L::WIDTH..]).abs();
        let c = L::select(
            L::mask_and(a.ge(L::splat(f32::MIN_POSITIVE)), a.lt(L::splat(f32::INFINITY))),
            a,
            L::zero(),
        );
        peak = L::max(c, peak);
    }
    peak
}

#[inline(always)]
fn sanitize<L: Lane>(x: L) -> L {
    let a = x.abs();
    L::select(
        L::mask_and(a.ge(L::splat(f32::MIN_POSITIVE)), a.lt(L::splat(f32::INFINITY))),
        a,
        L::zero(),
    )
}

#[inline(never)]
fn meter_oracle(x: f32) -> f32 {
    let s = if x.is_finite() && !x.is_subnormal() { x } else { 0.0 };
    s.abs()
}

#[unsafe(no_mangle)]
pub extern "C" fn buf_ptr() -> *mut f32 { unsafe { BUF.as_mut_ptr() } }
#[unsafe(no_mangle)]
pub extern "C" fn out_ptr() -> *mut f32 { unsafe { OUT.as_mut_ptr() } }

/// Per-element sanitize over BUF[..n] into OUT (Simd4), for JS-side oracle checks.
#[unsafe(no_mangle)]
pub extern "C" fn sanitize4(n: usize) {
    unsafe {
        let mut i = 0;
        while i + 4 <= n {
            sanitize::<Simd4>(Simd4::load(&BUF[i..])).store(&mut OUT[i..]);
            i += 4;
        }
    }
}

/// Exhaustive in-module check over all 2^32 patterns: Simd4 sanitize vs scalar oracle.
#[unsafe(no_mangle)]
pub extern "C" fn exhaustive_mismatches() -> u32 {
    let mut mismatches = 0u32;
    let mut base: u64 = 0;
    let mut buf = [0.0f32; 4];
    let mut out = [0.0f32; 4];
    while base <= u32::MAX as u64 {
        for i in 0..4 { buf[i] = f32::from_bits((base + i as u64) as u32); }
        sanitize::<Simd4>(Simd4::load(&buf)).store(&mut out);
        for i in 0..4 {
            if out[i].to_bits() != meter_oracle(buf[i]).to_bits() { mismatches += 1; }
        }
        base += 4;
    }
    mismatches
}

/// The census target: one pass over a 4-lane AoSoA block, both planes.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn peak_pass4(frames: usize) {
    unsafe {
        let l = meter_sample_peak_block::<Simd4>(&BUF[..frames * 4], frames, Simd4::zero());
        let r = meter_sample_peak_block::<Simd4>(&BUF[N / 2..N / 2 + frames * 4], frames, Simd4::zero());
        l.store(&mut OUT[0..]);
        r.store(&mut OUT[4..]);
    }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 { self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17; self.0 }
}

/// Randomized kernel identity in-module: Simd4 kernel (carried) vs scalar serial loop, hostile pool.
#[unsafe(no_mangle)]
pub extern "C" fn kernel_mismatches(seed: u64, rounds: u32) -> u32 {
    let mut pool = vec![f32::NAN, -f32::NAN, f32::from_bits(0x7FC0_0001), f32::from_bits(0xFF80_0001),
        f32::INFINITY, f32::NEG_INFINITY, 0.0, -0.0, f32::from_bits(1), f32::from_bits(0x8000_0001),
        f32::from_bits(0x007F_FFFF), f32::from_bits(0x807F_FFFF), f32::MIN_POSITIVE, -f32::MIN_POSITIVE,
        1.0, -1.0, f32::from_bits(0x3F7F_FFFF), f32::from_bits(0x3F80_0001), f32::MAX, -f32::MAX, 1.0e30];
    for k in -34..=5 { pool.push((2.0f32).powi(k)); pool.push(-(2.0f32).powi(k)); }
    let mut rng = Rng(seed);
    let mut mismatches = 0;
    for _ in 0..rounds {
        let frames = [1usize, 2, 3, 127, 128, 129][(rng.next() % 6) as usize];
        let mut carried = Simd4::zero();
        let mut serial = [0.0f32; 4];
        let mut merged = [0.0f32; 4];
        for _block in 0..64 {
            let words: Vec<f32> = (0..frames * 4).map(|_| {
                let r = rng.next();
                if r % 3 == 0 { f32::from_bits((r >> 16) as u32) } else { pool[((r >> 8) % pool.len() as u64) as usize] }
            }).collect();
            carried = meter_sample_peak_block::<Simd4>(&words, frames, carried);
            let mut partial = [0.0f32; 4];
            meter_sample_peak_block::<Simd4>(&words, frames, Simd4::zero()).store(&mut partial);
            let mut got = [0.0f32; 4];
            carried.store(&mut got);
            for lane in 0..4 {
                for f in 0..frames {
                    let a = meter_oracle(words[f * 4 + lane]);
                    serial[lane] = if a > serial[lane] { a } else { serial[lane] };
                }
                merged[lane] = if partial[lane] > merged[lane] { partial[lane] } else { merged[lane] };
                if got[lane].to_bits() != serial[lane].to_bits() { mismatches += 1; }
                if merged[lane].to_bits() != serial[lane].to_bits() { mismatches += 1; }
            }
        }
    }
    mismatches
}
