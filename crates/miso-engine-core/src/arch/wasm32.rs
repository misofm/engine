//! Base WebAssembly `simd128` four-lane TPT kernel (no relaxed SIMD).

use core::arch::wasm32::*;

use super::TptKernelBlock;

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
