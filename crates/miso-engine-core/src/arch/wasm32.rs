//! Base WebAssembly `simd128` four-lane TPT kernel (no relaxed SIMD).

use core::arch::wasm32::*;

use super::{CompressorGainMixKernelBlock, DeltaKernelBlock, TptKernelBlock};

#[inline(never)]
#[target_feature(enable = "simd128")]
unsafe fn process_compressor_gain_mix_wasm_simd128_inner(block: CompressorGainMixKernelBlock<'_>) {
    // SAFETY: the `simd128` artifact and prepared token prove support; all slices have four lanes.
    unsafe {
        let dry = v128_load(block.samples.as_ptr().cast::<v128>());
        let gain = v128_load(block.gains.as_ptr().cast::<v128>());
        let mix = v128_load(block.mixes.as_ptr().cast::<v128>());
        let p0 = f32x4_mul(dry, gain);
        let p1 = f32x4_sub(p0, dry);
        let p2 = f32x4_mul(mix, p1);
        let p3 = f32x4_add(dry, p2);
        let dry_mask = v128_load(block.dry_mask.as_ptr().cast::<v128>());
        let wet_mask = v128_load(block.wet_mask.as_ptr().cast::<v128>());
        let mixed_or_wet = v128_bitselect(p0, p3, wet_mask);
        let output = v128_bitselect(dry, mixed_or_wet, dry_mask);
        v128_store(block.samples.as_mut_ptr().cast::<v128>(), output);
    }
}

#[inline(never)]
pub(super) fn process_compressor_gain_mix_wasm_simd128(block: CompressorGainMixKernelBlock<'_>) {
    // SAFETY: module cfg and the prepared token prove base `simd128` before this call.
    unsafe { process_compressor_gain_mix_wasm_simd128_inner(block) }
}

#[inline(never)]
#[target_feature(enable = "simd128")]
unsafe fn process_tpt_wasm_simd128_inner(block: TptKernelBlock<'_>) {
    // SAFETY: this module exists only in a `simd128` artifact. Slice validation proves four
    // lanes for every unaligned v128 load/store.
    unsafe {
        let x = v128_load(block.samples.as_ptr().cast::<v128>());
        let c1 = v128_load(block.c1.as_ptr().cast::<v128>());
        let a2 = v128_load(block.a2.as_ptr().cast::<v128>());
        let a3 = v128_load(block.a3.as_ptr().cast::<v128>());
        let k = v128_load(block.k.as_ptr().cast::<v128>());
        let old_s1 = v128_load(block.s1.as_ptr().cast::<v128>());
        let old_s2 = v128_load(block.s2.as_ptr().cast::<v128>());
        let v3 = f32x4_sub(x, old_s2);
        let p1 = f32x4_mul(a2, v3);
        let p2 = f32x4_mul(c1, old_s1);
        let d1 = f32x4_sub(p1, p2);
        let v1 = f32x4_add(old_s1, d1);
        let p3 = f32x4_mul(a2, old_s1);
        let p4 = f32x4_mul(a3, v3);
        let d2 = f32x4_add(p3, p4);
        let v2 = f32x4_add(old_s2, d2);
        let n1 = f32x4_add(old_s1, f32x4_add(d1, d1));
        let n2 = f32x4_add(old_s2, f32x4_add(d2, d2));
        let th = f32x4_sub(x, f32x4_mul(k, v1));
        let high = f32x4_sub(th, v2);
        let mask = v128_load(block.high_pass_mask.as_ptr().cast::<v128>());
        let output = v128_bitselect(high, v2, mask);
        v128_store(block.s1.as_mut_ptr().cast::<v128>(), n1);
        v128_store(block.s2.as_mut_ptr().cast::<v128>(), n2);
        v128_store(block.samples.as_mut_ptr().cast::<v128>(), output);
    }
}

#[inline(never)]
pub(super) fn process_tpt_wasm_simd128(block: TptKernelBlock<'_>) {
    // SAFETY: the module-level cfg and prepared token prove `simd128` before this safe call.
    unsafe { process_tpt_wasm_simd128_inner(block) }
}

#[inline(never)]
#[target_feature(enable = "simd128")]
unsafe fn process_delta_wasm_simd128_inner(block: DeltaKernelBlock<'_>) {
    // SAFETY: this module requires a simd128 artifact and the safe token validates four lanes.
    unsafe {
        let x = v128_load(block.samples.as_ptr().cast::<v128>());
        let a = v128_load(block.a.as_ptr().cast::<v128>());
        let n0 = v128_load(block.n0.as_ptr().cast::<v128>());
        let d0 = v128_load(block.d0.as_ptr().cast::<v128>());
        let n1 = v128_load(block.n1.as_ptr().cast::<v128>());
        let d1 = v128_load(block.d1.as_ptr().cast::<v128>());
        let n2 = v128_load(block.n2.as_ptr().cast::<v128>());
        let d2 = v128_load(block.d2.as_ptr().cast::<v128>());
        let old_x1 = v128_load(block.x1.as_ptr().cast::<v128>());
        let old_x2 = v128_load(block.x2.as_ptr().cast::<v128>());
        let old_y1 = v128_load(block.y1.as_ptr().cast::<v128>());
        let old_y2 = v128_load(block.y2.as_ptr().cast::<v128>());
        let t0 = f32x4_mul(a, x);
        let dx = f32x4_sub(old_x1, t0);
        let t1 = f32x4_mul(a, old_x1);
        let t2 = f32x4_sub(old_x2, t1);
        let t3 = f32x4_mul(a, dx);
        let ddx = f32x4_sub(t2, t3);
        let p0 = f32x4_mul(n0, x);
        let p1 = f32x4_mul(n1, dx);
        let s0 = f32x4_add(p0, p1);
        let p2 = f32x4_mul(n2, ddx);
        let num = f32x4_add(s0, p2);
        let q0 = f32x4_mul(a, d1);
        let scale = f32x4_add(f32x4_sub(d0, q0), d2);
        let q1 = f32x4_mul(a, d2);
        let q2 = f32x4_sub(f32x4_sub(d1, q1), q1);
        let h0 = f32x4_mul(q2, old_y1);
        let h1 = f32x4_mul(d2, old_y2);
        let history = f32x4_add(h0, h1);
        let y = f32x4_div(f32x4_sub(num, history), scale);
        let mask = v128_load(block.identity_mask.as_ptr().cast::<v128>());
        let new_y2 = v128_bitselect(old_x1, old_y1, mask);
        let new_y1 = v128_bitselect(x, y, mask);
        let output = v128_bitselect(x, y, mask);
        v128_store(block.x1.as_mut_ptr().cast::<v128>(), x);
        v128_store(block.x2.as_mut_ptr().cast::<v128>(), old_x1);
        v128_store(block.y1.as_mut_ptr().cast::<v128>(), new_y1);
        v128_store(block.y2.as_mut_ptr().cast::<v128>(), new_y2);
        v128_store(block.samples.as_mut_ptr().cast::<v128>(), output);
    }
}

#[inline(never)]
pub(super) fn process_delta_wasm_simd128(block: DeltaKernelBlock<'_>) {
    // SAFETY: artifact and prepared token prove base simd128 before render execution.
    unsafe { process_delta_wasm_simd128_inner(block) }
}
